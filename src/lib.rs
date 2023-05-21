use conv::{
    map_adapter_options, map_device_descriptor, map_instance_descriptor,
    map_pipeline_layout_descriptor, map_shader_module, map_surface, map_swapchain_descriptor,
    write_limits_struct, CreateSurfaceParams,
};
use std::{
    borrow::Cow,
    error,
    ffi::{CStr, CString},
    fmt::Display,
    num::NonZeroU64,
    path::Path,
    sync::Arc,
    sync::Mutex,
};
use thiserror::Error;
use utils::{make_slice, OwnedLabel};
use wgc::{
    command::{self, bundle_ffi, compute_ffi, render_ffi},
    gfx_select, id, resource, Label,
};

pub mod conv;
pub mod logging;
pub mod unimplemented;
pub mod utils;

pub mod native {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const LABEL: &str = "label";

pub type Context = wgc::hub::Global<wgc::hub::IdentityManagerFactory>;

pub struct WGPUContextHandle<I: id::TypedId> {
    pub context: Arc<Context>,
    pub id: I,
}

pub type WGPUPipelineLayoutImpl = WGPUContextHandle<id::PipelineLayoutId>;
pub type WGPUShaderModuleImpl = WGPUContextHandle<id::ShaderModuleId>;
pub type WGPUBindGroupLayoutImpl = WGPUContextHandle<id::BindGroupLayoutId>;
pub type WGPUBindGroupImpl = WGPUContextHandle<id::BindGroupId>;
pub type WGPUCommandBufferImpl = WGPUContextHandle<id::CommandBufferId>;
pub type WGPURenderBundleImpl = WGPUContextHandle<id::RenderBundleId>;
pub type WGPURenderPipelineImpl = WGPUContextHandle<id::RenderPipelineId>;
pub type WGPUComputePipelineImpl = WGPUContextHandle<id::ComputePipelineId>;
pub type WGPUQuerySetImpl = WGPUContextHandle<id::QuerySetId>;
pub type WGPUStagingBufferImpl = WGPUContextHandle<id::StagingBufferId>;
pub type WGPUTextureViewImpl = WGPUContextHandle<id::TextureViewId>;
pub type WGPUSamplerImpl = WGPUContextHandle<id::SamplerId>;
pub type WGPUSurfaceImpl = WGPUContextHandle<id::SurfaceId>;

pub struct WGPUInstanceImpl {
    pub context: Arc<Context>,
}

pub struct WGPUAdapterImpl {
    pub context: Arc<Context>,
    pub id: id::AdapterId,
    pub name: std::ffi::CString,
    pub vendor_name: std::ffi::CString,
    pub architecture_name: std::ffi::CString,
    pub driver_desc: std::ffi::CString,
}
pub struct WGPUDeviceImpl {
    context: Arc<Context>,
    id: id::DeviceId,
    error_sink: ErrorSink,
}

pub struct WGPUQueueImpl {
    context: Arc<Context>,
    id: id::QueueId,
    error_sink: ErrorSink,
}

pub struct WGPUBufferImpl {
    context: Arc<Context>,
    id: id::BufferId,
    error_sink: ErrorSink,
}

pub struct WGPUCommandEncoderImpl {
    context: Arc<Context>,
    id: id::CommandEncoderId,
    error_sink: ErrorSink,
}

pub struct WGPUSwapChainImpl {
    context: Arc<Context>,
    surface_id: id::SurfaceId,
    device_id: id::DeviceId,
    error_sink: ErrorSink,
}

pub struct WGPUTextureImpl {
    context: Arc<Context>,
    id: id::TextureId,
    error_sink: ErrorSink,
}

pub struct WGPURenderPassEncoderImpl {
    context: Arc<Context>,
    encoder: command::RenderPass,
    error_sink: ErrorSink,
}

pub struct WGPUComputePassEncoderImpl {
    context: Arc<Context>,
    encoder: command::ComputePass,
    error_sink: ErrorSink,
}

pub struct WGPURenderBundleEncoderImpl {
    context: Arc<Context>,
    encoder: command::RenderBundleEncoder,
}

struct DeviceCallback<T> {
    callback: T,
    userdata: *mut std::os::raw::c_void,
}
unsafe impl<T> Send for DeviceCallback<T> {}

type UncapturedErrorCallback = DeviceCallback<native::WGPUErrorCallback>;
type DeviceLostCallback = DeviceCallback<native::WGPUDeviceLostCallback>;

unsafe extern "C" fn default_uncaptured_error_handler(
    _typ: native::WGPUErrorType,
    message: *const ::std::os::raw::c_char,
    _userdata: *mut ::std::os::raw::c_void,
) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::error!("Handling wgpu uncaptured errors as fatal by default");
    panic!("wgpu uncaptured error:\n{message}\n");
}
const DEFAULT_UNCAPTURED_ERROR_HANDLER: UncapturedErrorCallback = UncapturedErrorCallback {
    callback: Some(default_uncaptured_error_handler),
    userdata: std::ptr::null_mut(),
};

unsafe extern "C" fn default_device_lost_handler(
    _reason: native::WGPUDeviceLostReason,
    message: *const ::std::os::raw::c_char,
    _userdata: *mut ::std::os::raw::c_void,
) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::error!("Handling wgpu device lost errors as fatal by default");
    panic!("wgpu device lost error:\n{message}\n");
}
const DEFAULT_DEVICE_LOST_HANDLER: DeviceLostCallback = DeviceLostCallback {
    callback: Some(default_device_lost_handler),
    userdata: std::ptr::null_mut(),
};

#[derive(Debug)]
pub enum Error {
    DeviceLost {
        source: Box<dyn error::Error + Send + 'static>,
    },
    OutOfMemory {
        source: Box<dyn error::Error + Send + 'static>,
    },
    Validation {
        source: Box<dyn error::Error + Send + 'static>,
        description: String,
    },
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::DeviceLost { source } => Some(source.as_ref()),
            Error::OutOfMemory { source } => Some(source.as_ref()),
            Error::Validation { source, .. } => Some(source.as_ref()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DeviceLost { .. } => f.write_str("Device lost"),
            Error::OutOfMemory { .. } => f.write_str("Out of Memory"),
            Error::Validation { description, .. } => f.write_str(description),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum ErrorFilter {
    /// Catch only out-of-memory errors.
    OutOfMemory,
    /// Catch only validation errors.
    Validation,
}

type ErrorSink = Arc<Mutex<ErrorSinkRaw>>;

struct ErrorScope {
    error: Option<crate::Error>,
    filter: crate::ErrorFilter,
}

struct ErrorSinkRaw {
    scopes: Vec<ErrorScope>,
    uncaptured_handler: UncapturedErrorCallback,
    device_lost_handler: DeviceLostCallback,
}

impl ErrorSinkRaw {
    fn new() -> ErrorSinkRaw {
        ErrorSinkRaw {
            scopes: Vec::new(),
            uncaptured_handler: DEFAULT_UNCAPTURED_ERROR_HANDLER,
            device_lost_handler: DEFAULT_DEVICE_LOST_HANDLER,
        }
    }

    fn handle_error(&mut self, err: crate::Error) {
        let (typ, filter) = match err {
            crate::Error::DeviceLost { .. } => {
                // handle device lost error early
                if let Some(callback) = self.device_lost_handler.callback {
                    let userdata = self.device_lost_handler.userdata;
                    let msg = CString::new(err.to_string()).unwrap();
                    unsafe {
                        callback(
                            native::WGPUDeviceLostReason_Destroyed,
                            msg.as_ptr(),
                            userdata,
                        )
                    };
                }
                return;
            }
            crate::Error::OutOfMemory { .. } => (
                native::WGPUErrorType_OutOfMemory,
                crate::ErrorFilter::OutOfMemory,
            ),
            crate::Error::Validation { .. } => (
                native::WGPUErrorType_Validation,
                crate::ErrorFilter::Validation,
            ),
        };

        match self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.filter == filter)
        {
            Some(scope) => {
                if scope.error.is_none() {
                    scope.error = Some(err);
                }
            }
            None => {
                if let Some(callback) = self.uncaptured_handler.callback {
                    let userdata = self.uncaptured_handler.userdata;
                    let msg = CString::new(err.to_string()).unwrap();
                    unsafe { callback(typ, msg.as_ptr(), userdata) };
                }
            }
        }
    }
}

fn format_error(context: &Context, err: &(impl error::Error + 'static)) -> String {
    let mut err_descs = vec![];

    let mut err_str = String::new();
    wgc::error::format_pretty_any(&mut err_str, context, err);
    err_descs.push(err_str);

    let mut source_opt = err.source();
    while let Some(source) = source_opt {
        let mut source_str = String::new();
        wgc::error::format_pretty_any(&mut source_str, context, source);
        err_descs.push(source_str);
        source_opt = source.source();
    }

    format!("Validation Error\n\nCaused by:\n{}", err_descs.join(""))
}

fn handle_error_fatal(
    context: &Context,
    cause: impl error::Error + Send + Sync + 'static,
    operation: &'static str,
) -> ! {
    panic!(
        "Error in {operation}: {f}",
        f = format_error(context, &cause)
    );
}

fn handle_error(
    context: &Context,
    sink_mutex: &Mutex<ErrorSinkRaw>,
    cause: impl error::Error + Send + Sync + 'static,
    label_key: &'static str,
    label: Label,
    string: &'static str,
) {
    let error = wgc::error::ContextError {
        string,
        cause: Box::new(cause),
        label: label.unwrap_or_default().to_string(),
        label_key,
    };
    let mut sink = sink_mutex.lock().unwrap();
    let mut source_opt: Option<&(dyn error::Error + 'static)> = Some(&error);
    while let Some(source) = source_opt {
        match source.downcast_ref::<wgc::device::DeviceError>() {
            Some(wgc::device::DeviceError::Lost) => {
                return sink.handle_error(crate::Error::DeviceLost {
                    source: Box::new(error),
                });
            }
            Some(wgc::device::DeviceError::OutOfMemory) => {
                return sink.handle_error(crate::Error::OutOfMemory {
                    source: Box::new(error),
                });
            }
            _ => (),
        }
        source_opt = source.source();
    }

    // Otherwise, it is a validation error
    sink.handle_error(crate::Error::Validation {
        description: format_error(context, &error),
        source: Box::new(error),
    });
}

// webgpu.h functions

#[no_mangle]
pub unsafe extern "C" fn wgpuCreateInstance(
    descriptor: Option<&native::WGPUInstanceDescriptor>,
) -> native::WGPUInstance {
    let descriptor = descriptor.expect("invalid descriptor");

    let instance_desc = follow_chain!(map_instance_descriptor(descriptor,
        WGPUSType_InstanceExtras => native::WGPUInstanceExtras
    ));

    Box::into_raw(Box::new(WGPUInstanceImpl {
        context: Arc::new(Context::new(
            "wgpu",
            wgc::hub::IdentityManagerFactory,
            instance_desc,
        )),
    }))
}

// Adapter methods

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterEnumerateFeatures(
    adapter: native::WGPUAdapter,
    features: *mut native::WGPUFeatureName,
) -> usize {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let adapter_features = match gfx_select!(adapter_id => context.adapter_features(adapter_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(context, err, "wgpuAdapterEnumerateFeatures"),
    };

    let temp = conv::features_to_native(adapter_features);

    if !features.is_null() {
        std::ptr::copy_nonoverlapping(temp.as_ptr(), features, temp.len());
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetLimits(
    adapter: native::WGPUAdapter,
    limits: Option<&mut native::WGPUSupportedLimits>,
) -> bool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(adapter_id => context.adapter_limits(adapter_id));
    match result {
        Ok(wgt_limits) => conv::write_limits_struct(wgt_limits, limits),
        Err(err) => handle_error_fatal(context, err, "wgpuAdapterGetLimits"),
    }

    true // indicates that we can fill WGPUChainedStructOut
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetProperties(
    adapter: native::WGPUAdapter,
    properties: Option<&mut native::WGPUAdapterProperties>,
) {
    let adapter = adapter.as_mut().expect("invalid adapter");
    let properties = properties.expect("invalid return pointer \"properties\"");
    let context = &adapter.context;
    let id = adapter.id;

    let maybe_props = gfx_select!(id => context.adapter_get_info(id));
    match maybe_props {
        Ok(props) => {
            adapter.name = CString::new((&props.name) as &str).unwrap();
            let driver_desc = format!("{} {}", props.driver, props.driver_info);
            adapter.driver_desc = CString::new(driver_desc.trim()).unwrap();

            properties.vendorID = props.vendor as u32;
            properties.vendorName = adapter.vendor_name.as_ptr();
            properties.architecture = adapter.architecture_name.as_ptr();
            properties.deviceID = props.device as u32;
            properties.name = adapter.name.as_ptr();
            properties.driverDescription = adapter.driver_desc.as_ptr();
            properties.adapterType = match props.device_type {
                wgt::DeviceType::Other => native::WGPUAdapterType_Unknown,
                wgt::DeviceType::IntegratedGpu => native::WGPUAdapterType_IntegratedGPU,
                wgt::DeviceType::DiscreteGpu => native::WGPUAdapterType_DiscreteGPU,
                wgt::DeviceType::VirtualGpu => native::WGPUAdapterType_CPU, // close enough?
                wgt::DeviceType::Cpu => native::WGPUAdapterType_CPU,
            };
            properties.backendType = match props.backend {
                wgt::Backend::Empty => native::WGPUBackendType_Null,
                wgt::Backend::Vulkan => native::WGPUBackendType_Vulkan,
                wgt::Backend::Metal => native::WGPUBackendType_Metal,
                wgt::Backend::Dx12 => native::WGPUBackendType_D3D12,
                wgt::Backend::Dx11 => native::WGPUBackendType_D3D11,
                wgt::Backend::Gl => native::WGPUBackendType_OpenGL,
                wgt::Backend::BrowserWebGpu => native::WGPUBackendType_OpenGLES, // close enough?
            };
        }
        Err(err) => handle_error_fatal(context, err, "wgpuAdapterGetProperties"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterHasFeature(
    adapter: native::WGPUAdapter,
    feature: native::WGPUFeatureName,
) -> bool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let adapter_features = match gfx_select!(adapter_id => context.adapter_features(adapter_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(context, err, "wgpuAdapterHasFeature"),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false,
    };

    adapter_features.contains(feature)
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: native::WGPUAdapter,
    descriptor: Option<&native::WGPUDeviceDescriptor>,
    callback: native::WGPURequestDeviceCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let callback = callback.expect("invalid callback");

    let (desc, trace_str) = match descriptor {
        Some(descriptor) => follow_chain!(
            map_device_descriptor(descriptor,
            WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
        ),
        None => (wgt::DeviceDescriptor::default(), None),
    };

    let trace_path = trace_str.as_ref().map(Path::new);

    let (device_id, err) = gfx_select!(adapter_id => context.adapter_request_device(adapter_id, &desc, trace_path, ()));
    match err {
        None => {
            callback(
                native::WGPURequestDeviceStatus_Success,
                Box::into_raw(Box::new(WGPUDeviceImpl {
                    context: context.clone(),
                    id: device_id,
                    error_sink: Arc::new(Mutex::new(ErrorSinkRaw::new())),
                })),
                std::ptr::null(),
                userdata,
            );
        }
        Some(err) => {
            let message = CString::new(format_error(context, &err)).unwrap();

            callback(
                native::WGPURequestDeviceStatus_Error,
                std::ptr::null_mut(),
                message.as_ptr(),
                userdata,
            );
        }
    }
}

// Buffer methods

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDestroy(buffer: native::WGPUBuffer) {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };
    // Per spec, no error to report. Even calling destroy multiple times is valid.
    let _ = gfx_select!(buffer_id => context.buffer_destroy(buffer_id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetConstMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *const ::std::os::raw::c_void {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };

    let buf = match gfx_select!(buffer_id => context.buffer_get_mapped_range(
        buffer_id,
        offset as wgt::BufferAddress,
        match size {
            conv::WGPU_WHOLE_MAP_SIZE => None,
            _ => Some(size as u64),
        }
    )) {
        Ok((ptr, _)) => ptr,
        Err(err) => handle_error_fatal(context, err, "wgpuBufferGetConstMappedRange"),
    };

    buf as *const ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut ::std::os::raw::c_void {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };

    let buf = match gfx_select!(buffer_id => context.buffer_get_mapped_range(
        buffer_id,
        offset as wgt::BufferAddress,
        match size {
            conv::WGPU_WHOLE_MAP_SIZE => None,
            _ => Some(size as u64),
        }
    )) {
        Ok((ptr, _)) => ptr,
        Err(err) => handle_error_fatal(context, err, "wgpuBufferGetMappedRange"),
    };

    buf as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferMapAsync(
    buffer: native::WGPUBuffer,
    mode: native::WGPUMapModeFlags,
    offset: usize,
    size: usize,
    callback: native::WGPUBufferMapCallback,
    userdata: *mut std::ffi::c_void,
) {
    let (buffer_id, context, error_sink) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context, &buffer.error_sink)
    };
    let callback = callback.expect("invalid callback");
    let userdata = utils::Userdata::new(userdata);

    let operation = wgc::resource::BufferMapOperation {
        host: match mode as native::WGPUMapMode {
            native::WGPUMapMode_Write => wgc::device::HostMap::Write,
            native::WGPUMapMode_Read => wgc::device::HostMap::Read,
            _ => panic!("invalid map mode"),
        },
        callback: wgc::resource::BufferMapCallback::from_rust(Box::new(
            move |result: resource::BufferAccessResult| {
                let status = match result {
                    Ok(()) => native::WGPUBufferMapAsyncStatus_Success,
                    Err(resource::BufferAccessError::Device(_)) => {
                        native::WGPUBufferMapAsyncStatus_DeviceLost
                    }
                    Err(resource::BufferAccessError::Invalid)
                    | Err(resource::BufferAccessError::Destroyed) => {
                        native::WGPUBufferMapAsyncStatus_DestroyedBeforeCallback
                    }
                    Err(_) => native::WGPUBufferMapAsyncStatus_Error,
                };

                callback(status, userdata.as_ptr());
            },
        )),
    };

    match gfx_select!(buffer_id => context.buffer_map_async(buffer_id, offset as u64 .. (offset + size) as u64, operation))
    {
        Ok(()) => (),
        Err(err) => handle_error(context, error_sink, err, "", None, "wgpuBufferMapAsync"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferUnmap(buffer: native::WGPUBuffer) {
    let (buffer_id, context, error_sink) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context, &buffer.error_sink)
    };
    match gfx_select!(buffer_id => context.buffer_unmap(buffer_id)) {
        Ok(()) => (),
        Err(err) => handle_error(context, error_sink, err, "", None, "wgpuBufferUnmap"),
    }
}

// CommandEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginComputePass(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUComputePassDescriptor>,
) -> native::WGPUComputePassEncoder {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    let desc = match descriptor {
        Some(descriptor) => wgc::command::ComputePassDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgc::command::ComputePassDescriptor::default(),
    };

    Box::into_raw(Box::new(WGPUComputePassEncoderImpl {
        context: context.clone(),
        encoder: wgc::command::ComputePass::new(command_encoder_id, &desc),
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPURenderPassDescriptor>,
) -> native::WGPURenderPassEncoder {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::RenderPassDepthStencilAttachment {
            view: desc
                .view
                .as_ref()
                .expect("invalid texture view for depth stencil attachment")
                .id,
            depth: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.depthLoadOp),
                store_op: conv::map_store_op(desc.depthStoreOp),
                clear_value: desc.depthClearValue,
                read_only: desc.depthReadOnly,
            },
            stencil: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.stencilLoadOp),
                store_op: conv::map_store_op(desc.stencilStoreOp),
                clear_value: desc.stencilClearValue,
                read_only: desc.stencilReadOnly,
            },
        }
    });
    let desc = wgc::command::RenderPassDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        color_attachments: Cow::Owned(
            make_slice(
                descriptor.colorAttachments,
                descriptor.colorAttachmentCount as usize,
            )
            .iter()
            .map(|color_attachment| {
                color_attachment
                    .view
                    .as_ref()
                    .map(|view| wgc::command::RenderPassColorAttachment {
                        view: view.id,
                        resolve_target: color_attachment.resolveTarget.as_ref().map(|v| v.id),
                        channel: wgc::command::PassChannel {
                            load_op: conv::map_load_op(color_attachment.loadOp),
                            store_op: conv::map_store_op(color_attachment.storeOp),
                            clear_value: conv::map_color(&color_attachment.clearValue),
                            read_only: false,
                        },
                    })
            })
            .collect(),
        ),
        depth_stencil_attachment: depth_stencil_attachment.as_ref(),
    };

    Box::into_raw(Box::new(WGPURenderPassEncoderImpl {
        context: context.clone(),
        encoder: wgc::command::RenderPass::new(command_encoder_id, &desc),
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderClearBuffer(
    command_encoder: native::WGPUCommandEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_clear_buffer(
        command_encoder_id,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        }
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderClearBuffer",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToBuffer(
    command_encoder: native::WGPUCommandEncoder,
    source: native::WGPUBuffer,
    source_offset: u64,
    destination: native::WGPUBuffer,
    destination_offset: u64,
    size: u64,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };
    let source_buffer_id = source.as_ref().expect("invalid source").id;
    let destination_buffer_id = destination.as_ref().expect("invalid destination").id;

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_copy_buffer_to_buffer(
        command_encoder_id,
        source_buffer_id,
        source_offset,
        destination_buffer_id,
        destination_offset,
        size
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderCopyBufferToBuffer",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyBuffer>,
    destination: Option<&native::WGPUImageCopyTexture>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_copy_buffer_to_texture(
        command_encoder_id,
        &conv::map_image_copy_buffer(source.expect("invalid source")),
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderCopyBufferToTexture",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyTexture>,
    destination: Option<&native::WGPUImageCopyBuffer>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_copy_texture_to_buffer(
        command_encoder_id,
        &conv::map_image_copy_texture(source.expect("invalid source")),
        &conv::map_image_copy_buffer(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderCopyTextureToBuffer",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToTexture(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyTexture>,
    destination: Option<&native::WGPUImageCopyTexture>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_copy_texture_to_texture(
        command_encoder_id,
        &conv::map_image_copy_texture(source.expect("invalid source")),
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderCopyTextureToTexture",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUCommandBufferDescriptor>,
) -> native::WGPUCommandBuffer {
    assert!(!command_encoder.is_null(), "invalid command encoder");

    // NOTE: Automatically drop the encoder
    let command_encoder = Box::from_raw(command_encoder);
    let context = &command_encoder.context;
    let command_encoder_id = command_encoder.id;
    let error_sink = &command_encoder.error_sink;

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandBufferDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandBufferDescriptor::default(),
    };

    let (command_buffer_id, error) = gfx_select!(command_encoder_id => context.command_encoder_finish(command_encoder_id, &desc));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderFinish",
        );
    }
    Box::into_raw(Box::new(WGPUCommandBufferImpl {
        context: context.clone(),
        id: command_buffer_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderInsertDebugMarker(
    command_encoder: native::WGPUCommandEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_insert_debug_marker(command_encoder_id, CStr::from_ptr(marker_label).to_str().unwrap()))
    {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderInsertDebugMarker",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPopDebugGroup(
    command_encoder: native::WGPUCommandEncoder,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_pop_debug_group(command_encoder_id))
    {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderPopDebugGroup",
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPushDebugGroup(
    command_encoder: native::WGPUCommandEncoder,
    group_label: *const std::ffi::c_char,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_push_debug_group(command_encoder_id, CStr::from_ptr(group_label).to_str().unwrap()))
    {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderPushDebugGroup",
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderResolveQuerySet(
    command_encoder: native::WGPUCommandEncoder,
    query_set: native::WGPUQuerySet,
    first_query: u32,
    query_count: u32,
    destination: native::WGPUBuffer,
    destination_offset: u64,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };
    let query_set_id = query_set.as_ref().expect("invalid query set").id;
    let destination_buffer_id = destination.as_ref().expect("invalid destination").id;

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_resolve_query_set(
        command_encoder_id,
        query_set_id,
        first_query,
        query_count,
        destination_buffer_id,
        destination_offset
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderResolveQuerySet",
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderWriteTimestamp(
    command_encoder: native::WGPUCommandEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let (command_encoder_id, context, error_sink) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (
            command_encoder.id,
            &command_encoder.context,
            &command_encoder.error_sink,
        )
    };
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_write_timestamp(
        command_encoder_id,
        query_set_id,
        query_index
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuCommandEncoderWriteTimestamp",
        )
    }
}

// ComputePassEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    compute_ffi::wgpu_compute_pass_begin_pipeline_statistics_query(pass, query_set_id, query_index);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroups(
    pass: native::WGPUComputePassEncoder,
    workgroup_count_x: u32,
    workgroup_count_y: u32,
    workgroup_count_z: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;

    compute_ffi::wgpu_compute_pass_dispatch_workgroups(
        pass,
        workgroup_count_x,
        workgroup_count_y,
        workgroup_count_z,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroupsIndirect(
    pass: native::WGPUComputePassEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_mut()
        .expect("invalid indirect buffer")
        .id;

    compute_ffi::wgpu_compute_pass_dispatch_workgroups_indirect(
        pass,
        indirect_buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass: native::WGPUComputePassEncoder) {
    assert!(!pass.is_null(), "invalid compute pass");

    // NOTE: Automatically drops the compute pass
    let pass = Box::from_raw(pass);
    let context = &pass.context;
    let error_sink = &pass.error_sink;
    let command_encoder_id = pass.encoder.parent_id();

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_run_compute_pass(command_encoder_id, &pass.encoder))
    {
        let name = wgc::gfx_select!(command_encoder_id => context.command_buffer_label(command_encoder_id));
        handle_error(
            context,
            error_sink,
            cause,
            "encoder",
            Some(Cow::Borrowed(&name)),
            "wgpuComputePassEncoderEnd",
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_end_pipeline_statistics_query(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderInsertDebugMarker(
    pass: native::WGPUComputePassEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPopDebugGroup(pass: native::WGPUComputePassEncoder) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    pass: native::WGPUComputePassEncoder,
    group_label: *const std::ffi::c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: native::WGPUComputePassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;

    compute_ffi::wgpu_compute_pass_set_bind_group(
        pass,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: native::WGPUComputePassEncoder,
    compute_pipeline: native::WGPUComputePipeline,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let compute_pipeline_id = compute_pipeline
        .as_ref()
        .expect("invalid compute pipeline")
        .id;

    compute_ffi::wgpu_compute_pass_set_pipeline(pass, compute_pipeline_id);
}

// ComputePipeline methods

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineGetBindGroupLayout(
    pipeline: native::WGPUComputePipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (pipeline_id, context) = {
        let pipeline = pipeline.as_ref().expect("invalid pipeline");
        (pipeline.id, &pipeline.context)
    };

    let (bind_group_layout_id, error) = gfx_select!(pipeline_id => context.compute_pipeline_get_bind_group_layout(pipeline_id, group_index, ()));
    if let Some(cause) = error {
        panic!(
            "Error in wgpuComputePipelineGetBindGroupLayout: Error reflecting bind group {group_index}: {f}",
            f = format_error(context, &cause)
        );
    }

    Box::into_raw(Box::new(WGPUBindGroupLayoutImpl {
        context: context.clone(),
        id: bind_group_layout_id,
    }))
}

// Device methods

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBindGroupDescriptor>,
) -> native::WGPUBindGroup {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");
    let bind_group_layout_id = descriptor
        .layout
        .as_ref()
        .expect("invalid bind group layout for bind group descriptor")
        .id;

    let entries = make_slice(descriptor.entries, descriptor.entryCount as usize)
        .iter()
        .map(|entry| {
            if let Some(buffer) = entry.buffer.as_ref() {
                wgc::binding_model::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgc::binding_model::BindingResource::Buffer(
                        wgc::binding_model::BufferBinding {
                            buffer_id: buffer.id,
                            offset: entry.offset,
                            size: match entry.size {
                                0 => panic!("invalid size"),
                                conv::WGPU_WHOLE_SIZE => None,
                                _ => Some(NonZeroU64::new_unchecked(entry.size)),
                            },
                        },
                    ),
                }
            } else if let Some(sampler) = entry.sampler.as_ref() {
                wgc::binding_model::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgc::binding_model::BindingResource::Sampler(sampler.id),
                }
            } else if let Some(texture_view) = entry.textureView.as_ref() {
                wgc::binding_model::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgc::binding_model::BindingResource::TextureView(texture_view.id),
                }
            } else {
                panic!("invalid bind group entry for bind group descriptor");
            }
        })
        .collect::<Vec<_>>();

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: bind_group_layout_id,
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_id, error) =
        gfx_select!(device_id => context.device_create_bind_group(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateBindGroup",
        );
    }

    Box::into_raw(Box::new(WGPUBindGroupImpl {
        context: context.clone(),
        id: bind_group_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBindGroupLayoutDescriptor>,
) -> native::WGPUBindGroupLayout {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let entries = make_slice(descriptor.entries, descriptor.entryCount as usize)
        .iter()
        .map(|entry| {
            let is_buffer = entry.buffer.type_ != native::WGPUBufferBindingType_Undefined;
            let is_sampler = entry.sampler.type_ != native::WGPUSamplerBindingType_Undefined;
            let is_texture = entry.texture.sampleType != native::WGPUTextureSampleType_Undefined;
            let is_storage_texture =
                entry.storageTexture.access != native::WGPUStorageTextureAccess_Undefined;

            let ty = if is_texture {
                wgt::BindingType::Texture {
                    sample_type: match entry.texture.sampleType {
                        native::WGPUTextureSampleType_Float => {
                            wgt::TextureSampleType::Float { filterable: true }
                        }
                        native::WGPUTextureSampleType_UnfilterableFloat => {
                            wgt::TextureSampleType::Float { filterable: false }
                        }
                        native::WGPUTextureSampleType_Depth => wgt::TextureSampleType::Depth,
                        native::WGPUTextureSampleType_Sint => wgt::TextureSampleType::Sint,
                        native::WGPUTextureSampleType_Uint => wgt::TextureSampleType::Uint,
                        _ => panic!("invalid sample type for texture binding layout"),
                    },
                    view_dimension: match entry.texture.viewDimension {
                        native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                        native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                        native::WGPUTextureViewDimension_2DArray => {
                            wgt::TextureViewDimension::D2Array
                        }
                        native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                        native::WGPUTextureViewDimension_CubeArray => {
                            wgt::TextureViewDimension::CubeArray
                        }
                        native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                        _ => panic!("invalid texture view dimension for texture binding layout"),
                    },
                    multisampled: entry.texture.multisampled,
                }
            } else if is_sampler {
                match entry.sampler.type_ {
                    native::WGPUSamplerBindingType_Filtering => {
                        wgt::BindingType::Sampler(wgt::SamplerBindingType::Filtering)
                    }
                    native::WGPUSamplerBindingType_NonFiltering => {
                        wgt::BindingType::Sampler(wgt::SamplerBindingType::NonFiltering)
                    }
                    native::WGPUSamplerBindingType_Comparison => {
                        wgt::BindingType::Sampler(wgt::SamplerBindingType::Comparison)
                    }
                    _ => panic!("invalid sampler binding type for sampler binding layout"),
                }
            } else if is_storage_texture {
                wgt::BindingType::StorageTexture {
                    access: match entry.storageTexture.access {
                        native::WGPUStorageTextureAccess_WriteOnly => {
                            wgt::StorageTextureAccess::WriteOnly
                        }
                        _ => {
                            panic!(
                                "invalid storage texture access for storage texture binding layout"
                            )
                        }
                    },
                    format: conv::map_texture_format(entry.storageTexture.format)
                        .expect("invalid texture format for storage texture binding layout"),
                    view_dimension: match entry.storageTexture.viewDimension {
                        native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                        native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                        native::WGPUTextureViewDimension_2DArray => {
                            wgt::TextureViewDimension::D2Array
                        }
                        native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                        native::WGPUTextureViewDimension_CubeArray => {
                            wgt::TextureViewDimension::CubeArray
                        }
                        native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                        _ => {
                            panic!(
                                "invalid texture view dimension for storage texture binding layout"
                            )
                        }
                    },
                }
            } else if is_buffer {
                wgt::BindingType::Buffer {
                    ty: match entry.buffer.type_ {
                        native::WGPUBufferBindingType_Uniform => wgt::BufferBindingType::Uniform,
                        native::WGPUBufferBindingType_Storage => {
                            wgt::BufferBindingType::Storage { read_only: false }
                        }
                        native::WGPUBufferBindingType_ReadOnlyStorage => {
                            wgt::BufferBindingType::Storage { read_only: true }
                        }
                        _ => panic!("invalid buffer binding type for buffer binding layout"),
                    },
                    has_dynamic_offset: entry.buffer.hasDynamicOffset,
                    min_binding_size: {
                        assert_ne!(
                            entry.buffer.minBindingSize,
                            conv::WGPU_WHOLE_SIZE,
                            "invalid min binding size for buffer binding layout, use 0 instead"
                        );

                        NonZeroU64::new(entry.buffer.minBindingSize)
                    },
                }
            } else {
                panic!("invalid bind group layout entry for bind group layout descriptor");
            };

            wgt::BindGroupLayoutEntry {
                ty,
                binding: entry.binding,
                visibility: wgt::ShaderStages::from_bits(entry.visibility)
                    .expect("invalid visibility for bind group layout entry"),
                count: None, // TODO - What is this?
            }
        })
        .collect::<Vec<_>>();

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_layout_id, error) =
        gfx_select!(device_id => context.device_create_bind_group_layout(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateBindGroupLayout",
        );
    }
    Box::into_raw(Box::new(WGPUBindGroupLayoutImpl {
        context: context.clone(),
        id: bind_group_layout_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBuffer(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBufferDescriptor>,
) -> native::WGPUBuffer {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgt::BufferDescriptor {
        label: label.as_cow(),
        size: descriptor.size,
        usage: wgt::BufferUsages::from_bits(descriptor.usage).expect("invalid buffer usage"),
        mapped_at_creation: descriptor.mappedAtCreation,
    };

    let (buffer_id, error) =
        gfx_select!(device_id => context.device_create_buffer(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateBuffer",
        );
    }

    Box::into_raw(Box::new(WGPUBufferImpl {
        context: context.clone(),
        id: buffer_id,
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUCommandEncoderDescriptor>,
) -> native::WGPUCommandEncoder {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let desc = match descriptor {
        Some(descriptor) => wgt::CommandEncoderDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandEncoderDescriptor::default(),
    };
    let (command_encoder_id, error) =
        gfx_select!(device_id => context.device_create_command_encoder(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateCommandEncoder",
        );
    }

    Box::into_raw(Box::new(WGPUCommandEncoderImpl {
        context: context.clone(),
        id: command_encoder_id,
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUComputePipelineDescriptor>,
) -> native::WGPUComputePipeline {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let stage = wgc::pipeline::ProgrammableStageDescriptor {
        module: descriptor
            .compute
            .module
            .as_ref()
            .expect("invalid shader module for compute pipeline descriptor")
            .id,
        entry_point: OwnedLabel::new(descriptor.compute.entryPoint)
            .into_cow()
            .expect("invalid entry point for compute pipeline descriptor"),
    };
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        stage,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (compute_pipeline_id, error) = gfx_select!(device_id => context.device_create_compute_pipeline(
        device_id,
        &desc,
        (),
        implicit_pipeline_ids
    ));
    if let Some(cause) = error {
        if let wgc::pipeline::CreateComputePipelineError::Internal(ref error) = cause {
            log::warn!(
                "Shader translation error for stage {:?}: {}",
                wgt::ShaderStages::COMPUTE,
                error
            );
            log::warn!("Please report it to https://github.com/gfx-rs/naga");
        }
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateComputePipeline",
        );
    }
    Box::into_raw(Box::new(WGPUComputePipelineImpl {
        context: context.clone(),
        id: compute_pipeline_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUPipelineLayoutDescriptor>,
) -> native::WGPUPipelineLayout {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = follow_chain!(
        map_pipeline_layout_descriptor(
            descriptor,
            WGPUSType_PipelineLayoutExtras => native::WGPUPipelineLayoutExtras)
    );
    let (pipeline_layout_id, error) =
        gfx_select!(device_id => context.device_create_pipeline_layout(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreatePipelineLayout",
        );
    }
    Box::into_raw(Box::new(WGPUPipelineLayoutImpl {
        context: context.clone(),
        id: pipeline_layout_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateQuerySet(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUQuerySetDescriptor>,
) -> native::WGPUQuerySet {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };

    let desc = conv::map_query_set_descriptor(descriptor.expect("invalid query set descriptor"));

    let (query_set_id, error) =
        gfx_select!(device_id => context.device_create_query_set(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreatePipelineLayout",
        );
    }
    Box::into_raw(Box::new(WGPUQuerySetImpl {
        context: context.clone(),
        id: query_set_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderBundleEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPURenderBundleEncoderDescriptor>,
) -> native::WGPURenderBundleEncoder {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = wgc::command::RenderBundleEncoderDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        color_formats: unsafe {
            make_slice(
                descriptor.colorFormats,
                descriptor.colorFormatsCount as usize,
            )
        }
        .iter()
        .map(|format| conv::map_texture_format(*format))
        .collect(),
        depth_stencil: conv::map_texture_format(descriptor.depthStencilFormat).map(|format| {
            wgt::RenderBundleDepthStencil {
                format,
                depth_read_only: descriptor.depthReadOnly,
                stencil_read_only: descriptor.stencilReadOnly,
            }
        }),
        sample_count: descriptor.sampleCount,
        multiview: None,
    };

    match wgc::command::RenderBundleEncoder::new(&desc, device_id, None) {
        Ok(encoder) => Box::into_raw(Box::new(WGPURenderBundleEncoderImpl {
            context: context.clone(),
            encoder,
        })),
        Err(cause) => {
            handle_error_fatal(context, cause, "wgpuDeviceCreateRenderBundleEncoder");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPURenderPipelineDescriptor>,
) -> native::WGPURenderPipeline {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor
                    .vertex
                    .module
                    .as_ref()
                    .expect("invalid vertex shader module for vertex state")
                    .id,
                entry_point: OwnedLabel::new(descriptor.vertex.entryPoint)
                    .into_cow()
                    .expect("invalid entry point for vertex state"),
            },
            buffers: Cow::Owned(
                make_slice(
                    descriptor.vertex.buffers,
                    descriptor.vertex.bufferCount as usize,
                )
                .iter()
                .map(|buffer| wgc::pipeline::VertexBufferLayout {
                    array_stride: buffer.arrayStride,
                    step_mode: match buffer.stepMode {
                        native::WGPUVertexStepMode_Vertex => wgt::VertexStepMode::Vertex,
                        native::WGPUVertexStepMode_Instance => wgt::VertexStepMode::Instance,
                        _ => panic!("invalid vertex step mode for vertex buffer layout"),
                    },
                    attributes: Cow::Owned(
                        make_slice(buffer.attributes, buffer.attributeCount as usize)
                            .iter()
                            .map(|attribute| wgt::VertexAttribute {
                                format: conv::map_vertex_format(attribute.format)
                                    .expect("invalid vertex format for vertex attribute"),
                                offset: attribute.offset,
                                shader_location: attribute.shaderLocation,
                            })
                            .collect(),
                    ),
                })
                .collect(),
            ),
        },
        primitive: wgt::PrimitiveState {
            topology: conv::map_primitive_topology(descriptor.primitive.topology),
            strip_index_format: conv::map_index_format(descriptor.primitive.stripIndexFormat).ok(),
            front_face: match descriptor.primitive.frontFace {
                native::WGPUFrontFace_CCW => wgt::FrontFace::Ccw,
                native::WGPUFrontFace_CW => wgt::FrontFace::Cw,
                _ => panic!("invalid front face for primitive state"),
            },
            cull_mode: match descriptor.primitive.cullMode {
                native::WGPUCullMode_None => None,
                native::WGPUCullMode_Front => Some(wgt::Face::Front),
                native::WGPUCullMode_Back => Some(wgt::Face::Back),
                _ => panic!("invalid cull mode for primitive state"),
            },
            unclipped_depth: false, // todo: fill this via extras
            polygon_mode: wgt::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: descriptor
            .depthStencil
            .as_ref()
            .map(|desc| wgt::DepthStencilState {
                format: conv::map_texture_format(desc.format)
                    .expect("invalid texture format for depth stencil state"),
                depth_write_enabled: desc.depthWriteEnabled,
                depth_compare: conv::map_compare_function(desc.depthCompare)
                    .expect("invalid depth compare function for depth stencil state"),
                stencil: wgt::StencilState {
                    front: conv::map_stencil_face_state(desc.stencilFront, "front"),
                    back: conv::map_stencil_face_state(desc.stencilBack, "back"),
                    read_mask: desc.stencilReadMask,
                    write_mask: desc.stencilWriteMask,
                },
                bias: wgt::DepthBiasState {
                    constant: desc.depthBias,
                    slope_scale: desc.depthBiasSlopeScale,
                    clamp: desc.depthBiasClamp,
                },
            }),
        multisample: wgt::MultisampleState {
            count: descriptor.multisample.count,
            mask: descriptor.multisample.mask as u64,
            alpha_to_coverage_enabled: descriptor.multisample.alphaToCoverageEnabled,
        },
        fragment: descriptor
            .fragment
            .as_ref()
            .map(|fragment| wgc::pipeline::FragmentState {
                stage: wgc::pipeline::ProgrammableStageDescriptor {
                    module: fragment
                        .module
                        .as_ref()
                        .expect("invalid fragment shader module for render pipeline descriptor")
                        .id,
                    entry_point: OwnedLabel::new(fragment.entryPoint)
                        .into_cow()
                        .expect("invalid entry point for fragment state"),
                },
                targets: Cow::Owned(
                    make_slice(fragment.targets, fragment.targetCount as usize)
                        .iter()
                        .map(|color_target| {
                            conv::map_texture_format(color_target.format).map(|format| {
                                wgt::ColorTargetState {
                                    format,
                                    blend: color_target.blend.as_ref().map(|blend| {
                                        wgt::BlendState {
                                            color: conv::map_blend_component(blend.color),
                                            alpha: conv::map_blend_component(blend.alpha),
                                        }
                                    }),
                                    write_mask: wgt::ColorWrites::from_bits(color_target.writeMask)
                                        .unwrap(),
                                }
                            })
                        })
                        .collect(),
                ),
            }),
        multiview: None,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (render_pipeline_id, error) = gfx_select!(device_id => context.device_create_render_pipeline(device_id, &desc, (), implicit_pipeline_ids));
    if let Some(cause) = error {
        if let wgc::pipeline::CreateRenderPipelineError::Internal { stage, ref error } = cause {
            log::error!("Shader translation error for stage {:?}: {}", stage, error);
            log::error!("Please report it to https://github.com/gfx-rs/naga");
        }
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateRenderPipeline",
        );
    }
    Box::into_raw(Box::new(WGPURenderPipelineImpl {
        context: context.clone(),
        id: render_pipeline_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSampler(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUSamplerDescriptor>,
) -> native::WGPUSampler {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };

    let desc = match descriptor {
        Some(descriptor) => wgc::resource::SamplerDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
            address_modes: [
                conv::map_address_mode(descriptor.addressModeU),
                conv::map_address_mode(descriptor.addressModeV),
                conv::map_address_mode(descriptor.addressModeW),
            ],
            mag_filter: conv::map_filter_mode(descriptor.magFilter),
            min_filter: conv::map_filter_mode(descriptor.minFilter),
            mipmap_filter: conv::map_mipmap_filter_mode(descriptor.mipmapFilter),
            lod_min_clamp: descriptor.lodMinClamp,
            lod_max_clamp: descriptor.lodMaxClamp,
            compare: conv::map_compare_function(descriptor.compare).ok(),
            anisotropy_clamp: descriptor.maxAnisotropy,
            border_color: None,
        },
        // wgpu-core doesn't have Default implementation for SamplerDescriptor,
        // use defaults from spec.
        // ref: https://gpuweb.github.io/gpuweb/#GPUSamplerDescriptor
        None => wgc::resource::SamplerDescriptor {
            label: None,
            address_modes: [
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
            ],
            mag_filter: wgt::FilterMode::Nearest,
            min_filter: wgt::FilterMode::Nearest,
            mipmap_filter: wgt::FilterMode::Nearest,
            lod_min_clamp: 0f32,
            lod_max_clamp: 32f32,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        },
    };

    let (sampler_id, error) =
        gfx_select!(device_id => context.device_create_sampler(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateSampler",
        );
    }
    Box::into_raw(Box::new(WGPUSamplerImpl {
        context: context.clone(),
        id: sampler_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateShaderModule(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUShaderModuleDescriptor>,
) -> native::WGPUShaderModule {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let label = OwnedLabel::new(descriptor.label);
    let source = follow_chain!(
        map_shader_module(descriptor,
        WGPUSType_ShaderModuleSPIRVDescriptor => native::WGPUShaderModuleSPIRVDescriptor,
        WGPUSType_ShaderModuleWGSLDescriptor => native::WGPUShaderModuleWGSLDescriptor,
        WGPUSType_ShaderModuleGLSLDescriptor => native::WGPUShaderModuleGLSLDescriptor)
    );

    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: label.as_cow(),
        shader_bound_checks: wgt::ShaderBoundChecks::default(),
    };
    let (shader_module_id, error) =
        gfx_select!(device_id => context.device_create_shader_module(device_id, &desc, source, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateSampler",
        );
    }
    Box::into_raw(Box::new(WGPUShaderModuleImpl {
        context: context.clone(),
        id: shader_module_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSwapChain(
    device: native::WGPUDevice,
    surface: native::WGPUSurface,
    descriptor: Option<&native::WGPUSwapChainDescriptor>,
) -> native::WGPUSwapChain {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let surface_id = surface.as_ref().expect("invalid surface").id;

    let config = follow_chain!(
        map_swapchain_descriptor(
            descriptor.expect("invalid descriptor"),
            WGPUSType_SwapChainDescriptorExtras => native::WGPUSwapChainDescriptorExtras)
    );

    let error = gfx_select!(device_id => context.surface_configure(surface_id, device_id, &config));
    if let Some(cause) = error {
        handle_error_fatal(context, cause, "wgpuDeviceCreateSwapChain");
    }
    Box::into_raw(Box::new(WGPUSwapChainImpl {
        context: context.clone(),
        surface_id,
        device_id,
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateTexture(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUTextureDescriptor>,
) -> native::WGPUTexture {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = wgt::TextureDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        size: conv::map_extent3d(&descriptor.size),
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: conv::map_texture_dimension(descriptor.dimension),
        format: conv::map_texture_format(descriptor.format)
            .expect("invalid texture format for texture descriptor"),
        usage: wgt::TextureUsages::from_bits(descriptor.usage)
            .expect("invalid texture usage for texture descriptor"),
        view_formats: make_slice(descriptor.viewFormats, descriptor.viewFormatCount as usize)
            .iter()
            .map(|v| {
                conv::map_texture_format(*v).expect("invalid view format for texture descriptor")
            })
            .collect(),
    };

    let (texture_id, error) =
        gfx_select!(device_id => context.device_create_texture(device_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            LABEL,
            desc.label,
            "wgpuDeviceCreateTexture",
        );
    }
    Box::into_raw(Box::new(WGPUTextureImpl {
        context: context.clone(),
        id: texture_id,
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub extern "C" fn wgpuDeviceDestroy(_device: native::WGPUDevice) {
    // Empty implementation, maybe call drop?
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceEnumerateFeatures(
    device: native::WGPUDevice,
    features: *mut native::WGPUFeatureName,
) -> usize {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let device_features = match gfx_select!(device_id => context.device_features(device_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(context, err, "wgpuDeviceEnumerateFeatures"),
    };

    let temp = conv::features_to_native(device_features);

    if !features.is_null() {
        std::ptr::copy_nonoverlapping(temp.as_ptr(), features, temp.len());
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetLimits(
    device: native::WGPUDevice,
    limits: Option<&mut native::WGPUSupportedLimits>,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(device_id => context.device_limits(device_id));
    match result {
        Ok(wgt_limits) => write_limits_struct(wgt_limits, limits),
        Err(err) => handle_error_fatal(context, err, "wgpuDeviceGetLimits"),
    }

    true // indicates that we can fill WGPUChainedStructOut
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetQueue(device: native::WGPUDevice) -> native::WGPUQueue {
    let (device_id, context, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context, &device.error_sink)
    };
    Box::into_raw(Box::new(WGPUQueueImpl {
        context: context.clone(),
        id: device_id,
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceHasFeature(
    device: native::WGPUDevice,
    feature: native::WGPUFeatureName,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let device_features = match gfx_select!(device_id => context.device_features(device_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(context, err, "wgpuDeviceHasFeature"),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false,
    };

    device_features.contains(feature)
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePopErrorScope(
    device: native::WGPUDevice,
    callback: native::WGPUErrorCallback,
    userdata: *mut ::std::os::raw::c_void,
) -> bool {
    let device = device.as_ref().expect("invalid device");
    let callback = callback.expect("invalid callback");
    let mut error_sink = device.error_sink.lock().unwrap();
    let scope = error_sink.scopes.pop().unwrap();

    if let Some(error) = scope.error {
        let typ = match error {
            crate::Error::OutOfMemory { .. } => native::WGPUErrorType_OutOfMemory,
            crate::Error::Validation { .. } => native::WGPUErrorType_Validation,
            // We handle device lost error early in ErrorSinkRaw::handle_error
            // so we should never get device lost error here.
            crate::Error::DeviceLost { .. } => unreachable!(),
        };

        let msg = CString::new(error.to_string()).unwrap();
        unsafe {
            callback(typ, msg.as_ptr(), userdata);
        }
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePushErrorScope(
    device: native::WGPUDevice,
    filter: native::WGPUErrorFilter,
) {
    let device = device.as_ref().expect("invalid device");
    let mut error_sink = device.error_sink.lock().unwrap();
    error_sink.scopes.push(ErrorScope {
        error: None,
        filter: match filter {
            native::WGPUErrorFilter_Validation => ErrorFilter::Validation,
            native::WGPUErrorFilter_OutOfMemory => ErrorFilter::OutOfMemory,
            _ => panic!("invalid error filter"),
        },
    });
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetDeviceLostCallback(
    device: native::WGPUDevice,
    callback: native::WGPUDeviceLostCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let device = device.as_ref().expect("invalid device");
    let mut error_sink = device.error_sink.lock().unwrap();
    error_sink.device_lost_handler = DeviceLostCallback { callback, userdata };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetUncapturedErrorCallback(
    device: native::WGPUDevice,
    callback: native::WGPUErrorCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let device = device.as_ref().expect("invalid device");
    let mut error_sink = device.error_sink.lock().unwrap();
    error_sink.uncaptured_handler = UncapturedErrorCallback { callback, userdata };
}

// Instance methods

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceCreateSurface(
    instance: native::WGPUInstance,
    descriptor: Option<&native::WGPUSurfaceDescriptor>,
) -> native::WGPUSurface {
    let context = &instance.as_ref().expect("invalid instance").context;
    let descriptor = descriptor.expect("invalid descriptor");

    let create_surface_params = follow_chain!(
        map_surface(descriptor,
            WGPUSType_SurfaceDescriptorFromWindowsHWND => native::WGPUSurfaceDescriptorFromWindowsHWND,
            WGPUSType_SurfaceDescriptorFromXcbWindow => native::WGPUSurfaceDescriptorFromXcbWindow,
            WGPUSType_SurfaceDescriptorFromXlibWindow => native::WGPUSurfaceDescriptorFromXlibWindow,
            WGPUSType_SurfaceDescriptorFromWaylandSurface => native::WGPUSurfaceDescriptorFromWaylandSurface,
            WGPUSType_SurfaceDescriptorFromMetalLayer => native::WGPUSurfaceDescriptorFromMetalLayer,
            WGPUSType_SurfaceDescriptorFromAndroidNativeWindow => native::WGPUSurfaceDescriptorFromAndroidNativeWindow)
    );

    let surface_id = match create_surface_params {
        CreateSurfaceParams::Raw((rdh, rwh)) => context.instance_create_surface(rdh, rwh, ()),
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        CreateSurfaceParams::Metal(layer) => context.instance_create_surface_metal(layer, ()),
    };

    Box::into_raw(Box::new(WGPUSurfaceImpl {
        context: context.clone(),
        id: surface_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceRequestAdapter(
    instance: native::WGPUInstance,
    options: Option<&native::WGPURequestAdapterOptions>,
    callback: native::WGPURequestAdapterCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let instance = instance.as_ref().expect("invalid instance");
    let context = &instance.context;
    let callback = callback.expect("invalid callback");

    let (desc, inputs) = match options {
        Some(options) => {
            let (compatible_surface, given_backend) = follow_chain!(
                map_adapter_options(options,
                WGPUSType_AdapterExtras => native::WGPUAdapterExtras)
            );

            (
                wgt::RequestAdapterOptions {
                    power_preference: match options.powerPreference {
                        native::WGPUPowerPreference_LowPower => wgt::PowerPreference::LowPower,
                        native::WGPUPowerPreference_HighPerformance => {
                            wgt::PowerPreference::HighPerformance
                        }
                        _ => wgt::PowerPreference::default(),
                    },
                    force_fallback_adapter: options.forceFallbackAdapter,
                    compatible_surface: compatible_surface.as_ref().map(|surface| surface.id),
                },
                wgc::instance::AdapterInputs::Mask(
                    match given_backend {
                        native::WGPUBackendType_Null => wgt::Backends::all(),
                        native::WGPUBackendType_Vulkan => wgt::Backends::VULKAN,
                        native::WGPUBackendType_Metal => wgt::Backends::METAL,
                        native::WGPUBackendType_D3D12 => wgt::Backends::DX12,
                        native::WGPUBackendType_D3D11 => wgt::Backends::DX11,
                        native::WGPUBackendType_OpenGL => wgt::Backends::GL,
                        _ => panic!("invalid backend type for adapter extras"),
                    },
                    |_| (),
                ),
            )
        }
        None => (
            wgt::RequestAdapterOptions::default(),
            wgc::instance::AdapterInputs::Mask(wgt::Backends::all(), |_| ()),
        ),
    };

    match context.request_adapter(&desc, inputs) {
        Ok(adapter_id) => {
            callback(
                native::WGPURequestAdapterStatus_Success,
                Box::into_raw(Box::new(WGPUAdapterImpl {
                    context: context.clone(),
                    id: adapter_id,
                    name: CString::default(),
                    vendor_name: CString::default(),
                    architecture_name: CString::default(),
                    driver_desc: CString::default(),
                })),
                std::ptr::null(),
                userdata,
            );
        }
        Err(err) => {
            let message = CString::new(format_error(context, &err)).unwrap();

            callback(
                match err {
                    wgc::instance::RequestAdapterError::NotFound => {
                        native::WGPURequestAdapterStatus_Unavailable
                    }
                    wgc::instance::RequestAdapterError::InvalidSurface(_) => {
                        native::WGPURequestAdapterStatus_Error
                    }
                    _ => native::WGPURequestAdapterStatus_Unknown,
                },
                std::ptr::null_mut(),
                message.as_ptr(),
                userdata,
            );
        }
    };
}

// Queue methods

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };

    let command_buffers = make_slice(commands, command_count as usize)
        .iter()
        .map(|command_buffer| {
            let command_buffer = *command_buffer;
            assert!(!command_buffer.is_null(), "invalid command buffer");

            // NOTE: Automaticaly drop the command buffer
            Box::from_raw(command_buffer).id
        })
        .collect::<Vec<_>>();

    if let Err(cause) = gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers)) {
        handle_error_fatal(context, cause, "wgpuQueueSubmit");
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteBuffer(
    queue: native::WGPUQueue,
    buffer: native::WGPUBuffer,
    buffer_offset: u64,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
) {
    let (queue_id, context, error_sink) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context, &queue.error_sink)
    };
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    let slice = make_slice(data, data_size);
    if let Err(cause) = gfx_select!(queue_id => context.queue_write_buffer(queue_id, buffer_id, buffer_offset, slice))
    {
        handle_error(context, error_sink, cause, "", None, "wgpuQueueWriteBuffer");
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteTexture(
    queue: native::WGPUQueue,
    destination: Option<&native::WGPUImageCopyTexture>,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
    data_layout: Option<&native::WGPUTextureDataLayout>,
    write_size: Option<&native::WGPUExtent3D>,
) {
    let (queue_id, context, error_sink) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context, &queue.error_sink)
    };

    let slice = make_slice(data, data_size);
    if let Err(cause) = gfx_select!(queue_id => context.queue_write_texture(
        queue_id,
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        slice,
        &conv::map_texture_data_layout(data_layout.expect("invalid data layout")),
        &conv::map_extent3d(write_size.expect("invalid write size"))
    )) {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuQueueWriteTexture",
        );
    }
}

// RenderBundleEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDraw(
    bundle: native::WGPURenderBundleEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;

    bundle_ffi::wgpu_render_bundle_draw(
        bundle,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexed(
    bundle: native::WGPURenderBundleEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;

    bundle_ffi::wgpu_render_bundle_draw_indexed(
        bundle,
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexedIndirect(
    bundle: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;

    bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
        bundle,
        indirect_buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndirect(
    bundle: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;

    bundle_ffi::wgpu_render_bundle_draw_indirect(bundle, indirect_buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderFinish(
    bundle: native::WGPURenderBundleEncoder,
    descriptor: Option<&native::WGPURenderBundleDescriptor>,
) -> native::WGPURenderBundle {
    assert!(!bundle.is_null(), "invalid render bundle");

    // NOTE: Automatically drops the bundle encoder
    let bundle = Box::from_raw(bundle);
    let context = &bundle.context;
    let bundle_encoder = bundle.encoder;
    let device_id = bundle_encoder.parent();

    let desc = match descriptor {
        Some(descriptor) => wgt::RenderBundleDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::RenderBundleDescriptor::default(),
    };

    let (render_bundle_id, error) =
        gfx_select!(device_id => context.render_bundle_encoder_finish(bundle_encoder, &desc, ()));
    if let Some(cause) = error {
        handle_error_fatal(context, cause, "wgpuRenderBundleEncoderFinish");
    }
    Box::into_raw(Box::new(WGPURenderBundleImpl {
        context: context.clone(),
        id: render_bundle_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderInsertDebugMarker(
    bundle: native::WGPURenderBundleEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_insert_debug_marker(bundle, marker_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPopDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_pop_debug_group(bundle);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPushDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_label: *const std::ffi::c_char,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_push_debug_group(bundle, group_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetBindGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let bind_group_id = group.as_ref().expect("invalid bind group").id;
    bundle_ffi::wgpu_render_bundle_set_bind_group(
        bundle,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetIndexBuffer(
    bundle: native::WGPURenderBundleEncoder,
    buffer: native::WGPUBuffer,
    format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    bundle_ffi::wgpu_render_bundle_set_index_buffer(
        bundle,
        buffer_id,
        conv::map_index_format(format).expect("invalid index format"),
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetPipeline(
    bundle: native::WGPURenderBundleEncoder,
    pipeline: native::WGPURenderPipeline,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let pipeline_id = pipeline.as_ref().expect("invalid render pipeline").id;

    bundle_ffi::wgpu_render_bundle_set_pipeline(bundle, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetVertexBuffer(
    bundle: native::WGPURenderBundleEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
        bundle,
        slot,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

// RenderPassEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    render_ffi::wgpu_render_pass_begin_pipeline_statistics_query(pass, query_set_id, query_index);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: native::WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    render_ffi::wgpu_render_pass_draw(
        pass,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexed(
    pass: native::WGPURenderPassEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: u32,
    first_instance: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    render_ffi::wgpu_render_pass_draw_indexed(
        pass,
        index_count,
        instance_count,
        first_index,
        base_vertex as i32,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_draw_indexed_indirect(pass, buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_draw_indirect(pass, buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass: native::WGPURenderPassEncoder) {
    assert!(!pass.is_null(), "invalid render pass");

    // NOTE: Automatically drops the render pass
    let pass = Box::from_raw(pass);
    let context = &pass.context;
    let error_sink = &pass.error_sink;
    let command_encoder_id = pass.encoder.parent_id();

    if let Err(cause) = gfx_select!(command_encoder_id => context.command_encoder_run_render_pass(command_encoder_id, &pass.encoder))
    {
        let name =
            gfx_select!(command_encoder_id => context.command_buffer_label(command_encoder_id));
        handle_error(
            context,
            error_sink,
            cause,
            "encoder",
            Some(Cow::Borrowed(&name)),
            "wgpuRenderPassEncoderEnd",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    render_ffi::wgpu_render_pass_end_pipeline_statistics_query(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderExecuteBundles(
    pass: native::WGPURenderPassEncoder,
    bundle_count: u32,
    bundles: *const native::WGPURenderBundle,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    let bundle_ids = make_slice(bundles, bundle_count as usize)
        .iter()
        .map(|v| v.as_ref().expect("invalid render bundle").id)
        .collect::<Vec<_>>();

    render_ffi::wgpu_render_pass_execute_bundles(pass, bundle_ids.as_ptr(), bundle_ids.len());
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    pass: native::WGPURenderPassEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPopDebugGroup(pass: native::WGPURenderPassEncoder) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    pass: native::WGPURenderPassEncoder,
    group_label: *const std::ffi::c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: native::WGPURenderPassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;

    render_ffi::wgpu_render_pass_set_bind_group(
        pass,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: native::WGPURenderPassEncoder,
    color: Option<&native::WGPUColor>,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_blend_constant(
        pass,
        &conv::map_color(color.expect("invalid color")),
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    index_format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    pass.set_index_buffer(
        buffer_id,
        conv::map_index_format(index_format).expect("Index format cannot be undefined"),
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: native::WGPURenderPassEncoder,
    render_pipeline: native::WGPURenderPipeline,
) {
    let render_pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let render_pipeline_id = render_pipeline
        .as_ref()
        .expect("invalid render pipeline")
        .id;

    render_ffi::wgpu_render_pass_set_pipeline(render_pass, render_pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    pass: native::WGPURenderPassEncoder,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_scissor_rect(pass, x, y, w, h);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    pass: native::WGPURenderPassEncoder,
    reference: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_stencil_reference(pass, reference);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    pass: native::WGPURenderPassEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_set_vertex_buffer(
        pass,
        slot,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetViewport(
    pass: native::WGPURenderPassEncoder,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    depth_min: f32,
    depth_max: f32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_viewport(pass, x, y, w, h, depth_min, depth_max);
}

// RenderPipeline methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    render_pipeline: native::WGPURenderPipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (render_pipeline_id, context) = {
        let render_pipeline = render_pipeline.as_ref().expect("invalid render pipeline");
        (render_pipeline.id, &render_pipeline.context)
    };

    let (bind_group_layout_id, error) = gfx_select!(render_pipeline_id => context.render_pipeline_get_bind_group_layout(render_pipeline_id, group_index, ()));
    if let Some(cause) = error {
        panic!(
            "Error in wgpuRenderPipelineGetBindGroupLayout: Error reflecting bind group {group_index}: {f}",
            f = format_error(context, &cause)
        );
    }
    Box::into_raw(Box::new(WGPUBindGroupLayoutImpl {
        context: context.clone(),
        id: bind_group_layout_id,
    }))
}

// Surface methods

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetPreferredFormat(
    surface: native::WGPUSurface,
    adapter: native::WGPUAdapter,
) -> native::WGPUTextureFormat {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let surface_id = surface.as_ref().expect("invalid surface").id;

    let caps = match wgc::gfx_select!(adapter_id => context.surface_get_capabilities(surface_id, adapter_id))
    {
        Ok(caps) => caps,
        Err(wgc::instance::GetSurfaceSupportError::Unsupported) => {
            wgt::SurfaceCapabilities::default()
        }
        Err(err) => handle_error_fatal(context, err, "wgpuSurfaceGetPreferredFormat"),
    };

    match caps
        .formats
        .first()
        .map(|f| conv::to_native_texture_format(*f))
        .flatten()
    {
        Some(format) => format,
        None => panic!(
            "Error in wgpuSurfaceGetPreferredFormat: unsupported format.\n\
            Please report it to https://github.com/gfx-rs/wgpu-native"
        ),
    }
}

#[derive(Debug, Error)]
pub enum SurfaceError {
    #[error("Surface timed out")]
    Timeout,
    #[error("Surface is outdated")]
    Outdated,
    #[error("Surface was lost")]
    Lost,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainGetCurrentTextureView(
    swap_chain: native::WGPUSwapChain,
) -> native::WGPUTextureView {
    let (surface_id, device_id, context, error_sink) = {
        let swap_chain = swap_chain.as_ref().expect("invalid swap chain");
        (
            swap_chain.surface_id,
            swap_chain.device_id,
            &swap_chain.context,
            &swap_chain.error_sink,
        )
    };

    match gfx_select!(device_id => context.surface_get_current_texture(surface_id, ())) {
        Ok(result) => match result.status {
            wgt::SurfaceStatus::Good | wgt::SurfaceStatus::Suboptimal => {
                let texture_id = result.texture_id.unwrap();
                let (texture_view_id, error) = gfx_select!(texture_id => context.texture_create_view(
                    texture_id,
                    &wgc::resource::TextureViewDescriptor::default(),
                    ()
                ));
                gfx_select!(texture_id => context.texture_drop(texture_id, false));
                if let Some(cause) = error {
                    handle_error(
                        context,
                        error_sink,
                        cause,
                        "",
                        None,
                        "wgpuSwapChainGetCurrentTextureView",
                    );
                }

                Box::into_raw(Box::new(WGPUTextureViewImpl {
                    context: context.clone(),
                    id: texture_view_id,
                }))
            }
            _ => {
                handle_error(
                    context,
                    error_sink,
                    match result.status {
                        wgt::SurfaceStatus::Timeout => &SurfaceError::Timeout,
                        wgt::SurfaceStatus::Outdated => &SurfaceError::Outdated,
                        wgt::SurfaceStatus::Lost => &SurfaceError::Lost,
                        _ => unreachable!(),
                    },
                    "",
                    None,
                    "wgpuSwapChainGetCurrentTextureView",
                );
                std::ptr::null_mut()
            }
        },
        Err(cause) => {
            handle_error_fatal(context, cause, "wgpuSwapChainGetCurrentTextureView");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainPresent(swap_chain: native::WGPUSwapChain) {
    let (surface_id, device_id, context) = {
        let swap_chain = swap_chain.as_ref().expect("invalid swap chain");
        (
            swap_chain.surface_id,
            swap_chain.device_id,
            &swap_chain.context,
        )
    };

    if let Err(cause) = gfx_select!(device_id => context.surface_present(surface_id)) {
        handle_error_fatal(context, cause, "wgpuSwapChainPresent");
    }
}

// Texture methods

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureCreateView(
    texture: native::WGPUTexture,
    descriptor: Option<&native::WGPUTextureViewDescriptor>,
) -> native::WGPUTextureView {
    let (texture_id, context, error_sink) = {
        let texture = texture.as_ref().expect("invalid texture");
        (texture.id, &texture.context, &texture.error_sink)
    };

    let desc = match descriptor {
        Some(descriptor) => wgc::resource::TextureViewDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
            format: conv::map_texture_format(descriptor.format),
            dimension: conv::map_texture_view_dimension(descriptor.dimension),
            range: wgt::ImageSubresourceRange {
                aspect: conv::map_texture_aspect(descriptor.aspect),
                base_mip_level: descriptor.baseMipLevel,
                mip_level_count: match descriptor.mipLevelCount {
                    0 => panic!("invalid mipLevelCount"),
                    native::WGPU_MIP_LEVEL_COUNT_UNDEFINED => None,
                    _ => Some(descriptor.mipLevelCount),
                },
                base_array_layer: descriptor.baseArrayLayer,
                array_layer_count: match descriptor.arrayLayerCount {
                    0 => panic!("invalid arrayLayerCount"),
                    native::WGPU_ARRAY_LAYER_COUNT_UNDEFINED => None,
                    _ => Some(descriptor.arrayLayerCount),
                },
            },
        },
        None => wgc::resource::TextureViewDescriptor::default(),
    };

    let (texture_view_id, error) =
        gfx_select!(texture_id => context.texture_create_view(texture_id, &desc, ()));
    if let Some(cause) = error {
        handle_error(
            context,
            error_sink,
            cause,
            "",
            None,
            "wgpuTextureCreateView",
        );
    }
    Box::into_raw(Box::new(WGPUTextureViewImpl {
        context: context.clone(),
        id: texture_view_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDestroy(texture: native::WGPUTexture) {
    let (texture_id, context) = {
        let texture = texture.as_ref().expect("invalid texture");
        (texture.id, &texture.context)
    };

    // Per spec, no error to report. Even calling destroy multiple times is valid.
    let _ = gfx_select!(texture_id => context.texture_destroy(texture_id));
}

// wgpu.h functions

#[no_mangle]
pub unsafe extern "C" fn wgpuGenerateReport(
    instance: native::WGPUInstance,
    native_report: Option<&mut native::WGPUGlobalReport>,
) {
    let context = &instance.as_ref().expect("invalid instance").context;
    let native_report = native_report.expect("invalid return pointer \"native_report\"");
    conv::write_global_report(native_report, context.generate_report());
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmitForIndex(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) -> native::WGPUSubmissionIndex {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };

    let command_buffers = make_slice(commands, command_count as usize)
    .iter()
    .map(|command_buffer| {
        let command_buffer = *command_buffer;
        assert!(!command_buffer.is_null(), "invalid command buffer");

        // NOTE: Automaticaly drop the command buffer
        Box::from_raw(command_buffer).id
    })
    .collect::<Vec<_>>();

    match gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers)) {
        Ok(submission_index) => submission_index.index,
        Err(cause) => handle_error_fatal(context, cause, "wgpuQueueSubmitForIndex"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePoll(
    device: native::WGPUDevice,
    wait: bool,
    wrapped_submission_index: Option<&native::WGPUWrappedSubmissionIndex>,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };

    let maintain = match wait {
        true => match wrapped_submission_index {
            Some(index) => {
                wgt::Maintain::WaitForSubmissionIndex(wgc::device::queue::WrappedSubmissionIndex {
                    queue_id: index
                        .queue
                        .as_ref()
                        .expect("invalid queue for wrapped submission index")
                        .id,
                    index: index.submissionIndex,
                })
            }
            None => wgt::Maintain::Wait,
        },
        false => wgt::Maintain::Poll,
    };

    match gfx_select!(device_id => context.device_poll(device_id, maintain)) {
        Ok(queue_empty) => queue_empty,
        Err(cause) => {
            handle_error_fatal(context, cause, "wgpuDevicePoll");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetCapabilities(
    surface: native::WGPUSurface,
    adapter: native::WGPUAdapter,
    capabilities: Option<&mut native::WGPUSurfaceCapabilities>,
) {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let surface_id = surface.as_ref().expect("invalid surface").id;
    let capabilities = capabilities.expect("invalid return pointer \"capabilities\"");

    let caps = match wgc::gfx_select!(adapter_id => context.surface_get_capabilities(surface_id, adapter_id))
    {
        Ok(caps) => caps,
        Err(wgc::instance::GetSurfaceSupportError::Unsupported) => {
            wgt::SurfaceCapabilities::default()
        }
        Err(cause) => handle_error_fatal(context, cause, "wgpuSurfaceGetCapabilities"),
    };

    let formats = caps
        .formats
        .iter()
        // some texture formats are not in webgpu.h and
        // conv::to_native_texture_format returns None for them.
        // so, filter them out.
        .filter_map(|f| conv::to_native_texture_format(*f))
        .collect::<Vec<_>>();

    capabilities.formatCount = formats.len();

    if !capabilities.formats.is_null() {
        std::ptr::copy_nonoverlapping(formats.as_ptr(), capabilities.formats, formats.len());
    }

    let present_modes = caps
        .present_modes
        .iter()
        .filter_map(|f| conv::to_native_present_mode(*f))
        .collect::<Vec<_>>();

    capabilities.presentModeCount = present_modes.len();

    if !capabilities.presentModes.is_null() {
        std::ptr::copy_nonoverlapping(
            present_modes.as_ptr(),
            capabilities.presentModes,
            present_modes.len(),
        );
    }

    let alpha_modes = caps
        .alpha_modes
        .iter()
        .map(|f| conv::to_native_composite_alpha_mode(*f))
        .collect::<Vec<_>>();

    capabilities.alphaModeCount = alpha_modes.len();

    if !capabilities.alphaModes.is_null() {
        std::ptr::copy_nonoverlapping(
            alpha_modes.as_ptr(),
            capabilities.alphaModes,
            alpha_modes.len(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPushConstants(
    pass: native::WGPURenderPassEncoder,
    stages: native::WGPUShaderStageFlags,
    offset: u32,
    size_bytes: u32,
    size: *const u8,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_push_constants(
        pass,
        wgt::ShaderStages::from_bits(stages).expect("invalid shader stage"),
        offset,
        size_bytes,
        size,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indirect(pass, buffer_id, offset, count);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect(pass, buffer_id, offset, count);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirectCount(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count_buffer: native::WGPUBuffer,
    count_buffer_offset: u64,
    max_count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indirect_count(
        pass,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count_buffer: native::WGPUBuffer,
    count_buffer_offset: u64,
    max_count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect_count(
        pass,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterDrop(adapter: native::WGPUAdapter) {
    assert!(!adapter.is_null(), "invalid adapter");
    let adapter = Box::from_raw(adapter);
    let context = &adapter.context;

    gfx_select!(adapter.id => context.adapter_drop(adapter.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupDrop(bind_group: native::WGPUBindGroup) {
    assert!(!bind_group.is_null(), "invalid bind group");
    let bind_group = Box::from_raw(bind_group);
    let context = &bind_group.context;

    gfx_select!(bind_group.id => context.bind_group_drop(bind_group.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupLayoutDrop(bind_group_layout: native::WGPUBindGroupLayout) {
    assert!(!bind_group_layout.is_null(), "invalid bind group layout");
    let bind_group_layout = Box::from_raw(bind_group_layout);
    let context = &bind_group_layout.context;

    gfx_select!(bind_group_layout.id => context.bind_group_layout_drop(bind_group_layout.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDrop(buffer: native::WGPUBuffer) {
    assert!(!buffer.is_null(), "invalid buffer");
    let buffer = Box::from_raw(buffer);
    let context = &buffer.context;

    gfx_select!(buffer.id => context.buffer_drop(buffer.id, false));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandBufferDrop(command_buffer: native::WGPUCommandBuffer) {
    assert!(!command_buffer.is_null(), "invalid command buffer");
    let command_buffer = Box::from_raw(command_buffer);
    let context = &command_buffer.context;

    gfx_select!(command_buffer.id => context.command_buffer_drop(command_buffer.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderDrop(command_encoder: native::WGPUCommandEncoder) {
    assert!(!command_encoder.is_null(), "invalid command encoder");
    let command_encoder = Box::from_raw(command_encoder);
    let context = &command_encoder.context;

    gfx_select!(command_encoder.id => context.command_encoder_drop(command_encoder.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDrop(
    compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    assert!(
        !compute_pass_encoder.is_null(),
        "invalid compute pass encoder"
    );
    drop(Box::from_raw(compute_pass_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineDrop(compute_pipeline: native::WGPUComputePipeline) {
    assert!(!compute_pipeline.is_null(), "invalid compute pipeline");
    let compute_pipeline = Box::from_raw(compute_pipeline);
    let context = &compute_pipeline.context;

    gfx_select!(compute_pipeline.id => context.compute_pipeline_drop(compute_pipeline.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceDrop(device: native::WGPUDevice) {
    assert!(!device.is_null(), "invalid device");
    let device = Box::from_raw(device);
    let context = &device.context;

    gfx_select!(device.id => context.device_drop(device.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceDrop(instance: native::WGPUInstance) {
    assert!(!instance.is_null(), "invalid instance");
    drop(Box::from_raw(instance));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuPipelineLayoutDrop(pipeline_layout: native::WGPUPipelineLayout) {
    assert!(!pipeline_layout.is_null(), "invalid pipeline layout");
    let pipeline_layout = Box::from_raw(pipeline_layout);
    let context = &pipeline_layout.context;

    gfx_select!(pipeline_layout.id => context.pipeline_layout_drop(pipeline_layout.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetDrop(query_set: native::WGPUQuerySet) {
    assert!(!query_set.is_null(), "invalid query set");
    let query_set = Box::from_raw(query_set);
    let context = &query_set.context;

    gfx_select!(query_set.id => context.query_set_drop(query_set.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueDrop(queue: native::WGPUQueue) {
    assert!(!queue.is_null(), "invalid queue");
    drop(Box::from_raw(queue));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleDrop(render_bundle: native::WGPURenderBundle) {
    assert!(!render_bundle.is_null(), "invalid render bundle");
    let render_bundle = Box::from_raw(render_bundle);
    let context = &render_bundle.context;

    gfx_select!(render_bundle.id => context.render_bundle_drop(render_bundle.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrop(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    assert!(
        !render_bundle_encoder.is_null(),
        "invalid render bundle encoder"
    );
    drop(Box::from_raw(render_bundle_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrop(
    render_pass_encoder: native::WGPURenderPassEncoder,
) {
    assert!(
        !render_pass_encoder.is_null(),
        "invalid render pass encoder"
    );
    drop(Box::from_raw(render_pass_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineDrop(render_pipeline: native::WGPURenderPipeline) {
    assert!(!render_pipeline.is_null(), "invalid render pipeline");
    let render_pipeline = Box::from_raw(render_pipeline);
    let context = &render_pipeline.context;

    gfx_select!(render_pipeline.id => context.render_pipeline_drop(render_pipeline.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSamplerDrop(sampler: native::WGPUSampler) {
    assert!(!sampler.is_null(), "invalid sampler");
    let sampler = Box::from_raw(sampler);
    let context = &sampler.context;

    gfx_select!(sampler.id => context.sampler_drop(sampler.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuShaderModuleDrop(shader_module: native::WGPUShaderModule) {
    assert!(!shader_module.is_null(), "invalid shader module");
    let shader_module = Box::from_raw(shader_module);
    let context = &shader_module.context;

    gfx_select!(shader_module.id => context.shader_module_drop(shader_module.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceDrop(surface: native::WGPUSurface) {
    assert!(!surface.is_null(), "invalid surface");
    let surface = Box::from_raw(surface);
    let context = &surface.context;

    context.surface_drop(surface.id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainDrop(swap_chain: native::WGPUSwapChain) {
    assert!(!swap_chain.is_null(), "invalid swap chain");
    drop(Box::from_raw(swap_chain));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDrop(texture: native::WGPUTexture) {
    assert!(!texture.is_null(), "invalid texture");
    let texture = Box::from_raw(texture);
    let context = &texture.context;

    gfx_select!(texture.id => context.texture_drop(texture.id, false));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureViewDrop(texture_view: native::WGPUTextureView) {
    assert!(!texture_view.is_null(), "invalid texture view");
    let texture_view = Box::from_raw(texture_view);
    let context = &texture_view.context;

    let _ = gfx_select!(texture_view.id => context.texture_view_drop(texture_view.id, false));
}
