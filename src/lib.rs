use conv::{
    map_adapter_type, map_backend_type, map_bind_group_entry, map_bind_group_layout_entry,
    map_device_descriptor, map_instance_backend_flags, map_instance_descriptor,
    map_pipeline_layout_descriptor, map_primitive_state, map_query_set_descriptor,
    map_query_set_index, map_shader_module, map_surface, map_surface_configuration,
    CreateSurfaceParams,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    error,
    ffi::{CStr, CString},
    fmt::Display,
    mem,
    num::NonZeroU64,
    sync::{atomic, Arc},
    thread,
};
use utils::{
    get_base_device_limits_from_adapter_limits, make_slice, ptr_into_label, ptr_into_path,
};
use wgc::{
    command::{bundle_ffi, DynComputePass, DynRenderPass},
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

pub type Context = wgc::global::Global;

pub struct WGPUAdapterImpl {
    context: Arc<Context>,
    id: id::AdapterId,
}
impl Drop for WGPUAdapterImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.adapter_drop(self.id));
        }
    }
}

pub struct WGPUBindGroupImpl {
    context: Arc<Context>,
    id: id::BindGroupId,
}
impl Drop for WGPUBindGroupImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.bind_group_drop(self.id));
        }
    }
}

pub struct WGPUBindGroupLayoutImpl {
    context: Arc<Context>,
    id: id::BindGroupLayoutId,
}
impl Drop for WGPUBindGroupLayoutImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.bind_group_layout_drop(self.id));
        }
    }
}

struct BufferData {
    usage: native::WGPUBufferUsageFlags,
    size: u64,
}
pub struct WGPUBufferImpl {
    context: Arc<Context>,
    id: id::BufferId,
    error_sink: ErrorSink,
    data: BufferData,
}
impl Drop for WGPUBufferImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.buffer_drop(self.id, false));
        }
    }
}

pub struct WGPUCommandBufferImpl {
    context: Arc<Context>,
    id: id::CommandBufferId,
    open: atomic::AtomicBool,
}
impl Drop for WGPUCommandBufferImpl {
    fn drop(&mut self) {
        if self.open.load(atomic::Ordering::SeqCst) && !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.command_buffer_drop(self.id));
        }
    }
}

pub struct WGPUCommandEncoderImpl {
    context: Arc<Context>,
    id: id::CommandEncoderId,
    error_sink: ErrorSink,
    open: atomic::AtomicBool,
}
impl Drop for WGPUCommandEncoderImpl {
    fn drop(&mut self) {
        if self.open.load(atomic::Ordering::SeqCst) && !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.command_encoder_drop(self.id));
        }
    }
}

pub struct WGPUComputePassEncoderImpl {
    context: Arc<Context>,
    encoder: *mut dyn DynComputePass,
    error_sink: ErrorSink,
}
impl Drop for WGPUComputePassEncoderImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            drop(unsafe { Box::from_raw(self.encoder) });
        }
    }
}
// ComputePassEncoder is thread-unsafe
unsafe impl Send for WGPUComputePassEncoderImpl {}
unsafe impl Sync for WGPUComputePassEncoderImpl {}

pub struct WGPUComputePipelineImpl {
    context: Arc<Context>,
    id: id::ComputePipelineId,
    error_sink: ErrorSink,
}
impl Drop for WGPUComputePipelineImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.compute_pipeline_drop(self.id));
        }
    }
}

struct QueueId {
    context: Arc<Context>,
    id: id::QueueId,
}
impl Drop for QueueId {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.queue_drop(self.id));
        }
    }
}

pub struct WGPUDeviceImpl {
    context: Arc<Context>,
    id: id::DeviceId,
    queue: Arc<QueueId>,
    error_sink: ErrorSink,
}
impl Drop for WGPUDeviceImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;

            match gfx_select!(self.id => context.device_poll(self.id, wgt::Maintain::Wait)) {
                Ok(_) => (),
                Err(err) => handle_error_fatal(err, "WGPUDeviceImpl::drop"),
            }

            gfx_select!(self.id => context.device_drop(self.id));
        }
    }
}

pub struct WGPUInstanceImpl {
    context: Arc<Context>,
}

pub struct WGPUPipelineLayoutImpl {
    context: Arc<Context>,
    id: id::PipelineLayoutId,
}
impl Drop for WGPUPipelineLayoutImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.pipeline_layout_drop(self.id));
        }
    }
}

struct QuerySetData {
    query_type: native::WGPUQueryType,
    query_count: u32,
}
pub struct WGPUQuerySetImpl {
    context: Arc<Context>,
    id: id::QuerySetId,
    data: QuerySetData,
}
impl Drop for WGPUQuerySetImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.query_set_drop(self.id));
        }
    }
}

pub struct WGPUQueueImpl {
    queue: Arc<QueueId>,
    error_sink: ErrorSink,
}

pub struct WGPURenderBundleImpl {
    context: Arc<Context>,
    id: id::RenderBundleId,
}
impl Drop for WGPURenderBundleImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.render_bundle_drop(self.id));
        }
    }
}

pub struct WGPURenderBundleEncoderImpl {
    context: Arc<Context>,
    encoder: *mut Option<*mut wgc::command::RenderBundleEncoder>,
}
impl Drop for WGPURenderBundleEncoderImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let encoder = unsafe { Box::from_raw(self.encoder) };
            if let Some(encoder) = *encoder {
                drop(unsafe { Box::from_raw(encoder) });
            }
        }
    }
}
// RenderBundleEncoder is thread-unsafe
unsafe impl Send for WGPURenderBundleEncoderImpl {}
unsafe impl Sync for WGPURenderBundleEncoderImpl {}

pub struct WGPURenderPassEncoderImpl {
    context: Arc<Context>,
    encoder: *mut dyn DynRenderPass,
    error_sink: ErrorSink,
}
impl Drop for WGPURenderPassEncoderImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            drop(unsafe { Box::from_raw(self.encoder) });
        }
    }
}
// RenderPassEncodee is thread-unsafe
unsafe impl Send for WGPURenderPassEncoderImpl {}
unsafe impl Sync for WGPURenderPassEncoderImpl {}

pub struct WGPURenderPipelineImpl {
    context: Arc<Context>,
    id: id::RenderPipelineId,
    error_sink: ErrorSink,
}
impl Drop for WGPURenderPipelineImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.render_pipeline_drop(self.id));
        }
    }
}

pub struct WGPUSamplerImpl {
    context: Arc<Context>,
    id: id::SamplerId,
}
impl Drop for WGPUSamplerImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            gfx_select!(self.id => context.sampler_drop(self.id));
        }
    }
}

pub struct WGPUShaderModuleImpl {
    context: Arc<Context>,
    id: Option<id::ShaderModuleId>,
}
impl Drop for WGPUShaderModuleImpl {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            if !thread::panicking() {
                let context = &self.context;
                gfx_select!(id => context.shader_module_drop(id));
            }
        }
    }
}

struct SurfaceData {
    device_id: id::DeviceId,
    error_sink: ErrorSink,
    texture_data: TextureData,
}

pub struct WGPUSurfaceImpl {
    context: Arc<Context>,
    id: id::SurfaceId,
    data: Mutex<Option<SurfaceData>>,
    // Shared bool between Texture & Surface to track surface_present calls
    has_surface_presented: Arc<atomic::AtomicBool>,
}
impl Drop for WGPUSurfaceImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.context.surface_drop(self.id);
        }
    }
}

#[derive(Copy, Clone)]
struct TextureData {
    usage: native::WGPUTextureUsageFlags,
    dimension: native::WGPUTextureDimension,
    size: native::WGPUExtent3D,
    format: native::WGPUTextureFormat,
    mip_level_count: u32,
    sample_count: u32,
}

pub struct WGPUTextureImpl {
    context: Arc<Context>,
    id: id::TextureId,
    error_sink: ErrorSink,
    data: TextureData,
    surface_id: Option<id::SurfaceId>,
    // Shared bool between Texture & Surface to track surface_present calls
    has_surface_presented: Arc<atomic::AtomicBool>,
}
impl Drop for WGPUTextureImpl {
    fn drop(&mut self) {
        if thread::panicking() {
            return;
        }
        match self.surface_id {
            Some(surface_id) => {
                if !self.has_surface_presented.load(atomic::Ordering::SeqCst) {
                    let context = &self.context;
                    match gfx_select!(self.id => context.surface_texture_discard(surface_id)) {
                        Ok(_) => (),
                        Err(cause) => handle_error_fatal(cause, "wgpuTextureRelease"),
                    }
                }
            }
            None => {
                let context = &self.context;
                gfx_select!(self.id => context.texture_drop(self.id, false));
            }
        }
    }
}

pub struct WGPUTextureViewImpl {
    context: Arc<Context>,
    id: id::TextureViewId,
}
impl Drop for WGPUTextureViewImpl {
    fn drop(&mut self) {
        if !thread::panicking() {
            let context = &self.context;
            let _ = gfx_select!(self.id => context.texture_view_drop(self.id, false));
        }
    }
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
    log::warn!("Handling wgpu uncaptured errors as fatal by default");
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
    log::warn!("Handling wgpu device lost errors as fatal by default");
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
    fn new(device_lost_handler: DeviceLostCallback) -> ErrorSinkRaw {
        ErrorSinkRaw {
            scopes: Vec::new(),
            uncaptured_handler: DEFAULT_UNCAPTURED_ERROR_HANDLER,
            device_lost_handler,
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
                        );
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

fn format_error(err: &(impl error::Error + 'static)) -> String {
    let mut output = String::new();
    let mut level = 1;

    fn print_tree(output: &mut String, level: &mut usize, e: &(dyn error::Error + 'static)) {
        let mut print = |e: &(dyn error::Error + 'static)| {
            use std::fmt::Write;
            writeln!(output, "{}{}", " ".repeat(*level * 2), e).unwrap();

            if let Some(e) = e.source() {
                *level += 1;
                print_tree(output, level, e);
                *level -= 1;
            }
        };
        if let Some(multi) = e.downcast_ref::<wgc::error::MultiError>() {
            for e in multi.errors() {
                print(e);
            }
        } else {
            print(e);
        }
    }

    print_tree(&mut output, &mut level, err);

    format!("Validation Error\n\nCaused by:\n{}", output)
}

fn handle_error_fatal(
    cause: impl error::Error + Send + Sync + 'static,
    operation: &'static str,
) -> ! {
    panic!("Error in {operation}: {f}", f = format_error(&cause));
}

fn handle_error(
    sink_mutex: &Mutex<ErrorSinkRaw>,
    source: impl error::Error + Send + Sync + 'static,
    label: Label<'_>,
    fn_ident: &'static str,
) {
    let error = wgc::error::ContextError {
        fn_ident,
        source: Box::new(source),
        label: label.unwrap_or_default().to_string(),
    };
    let mut sink = sink_mutex.lock();
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
        description: format_error(&error),
        source: Box::new(error),
    });
}

// webgpu.h functions

#[no_mangle]
pub unsafe extern "C" fn wgpuCreateInstance(
    descriptor: Option<&native::WGPUInstanceDescriptor>,
) -> native::WGPUInstance {
    let instance_desc = match descriptor {
        Some(descriptor) => follow_chain!(map_instance_descriptor(
            (descriptor),
            WGPUSType_InstanceExtras => native::WGPUInstanceExtras
        )),
        None => wgt::InstanceDescriptor::default(),
    };

    Arc::into_raw(Arc::new(WGPUInstanceImpl {
        context: Arc::new(Context::new("wgpu", instance_desc)),
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
        Err(err) => handle_error_fatal(err, "wgpuAdapterEnumerateFeatures"),
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
) -> native::WGPUBool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(adapter_id => context.adapter_limits(adapter_id));
    match result {
        Ok(wgt_limits) => conv::write_limits_struct(wgt_limits, limits),
        Err(err) => handle_error_fatal(err, "wgpuAdapterGetLimits"),
    }

    true as native::WGPUBool // indicates that we can fill WGPUChainedStructOut
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetInfo(
    adapter: native::WGPUAdapter,
    info: Option<&mut native::WGPUAdapterInfo>,
) {
    let adapter = adapter.as_ref().expect("invalid adapter");
    let info = info.expect("invalid return pointer \"info\"");
    let context = adapter.context.as_ref();
    let adapter_id = adapter.id;

    let result = gfx_select!(adapter_id => context.adapter_get_info(adapter_id));
    let result = match result {
        Ok(info) => info,
        Err(err) => handle_error_fatal(err, "wgpuAdapterGetInfo"),
    };

    info.vendor = CString::new(result.driver).unwrap().into_raw();
    info.architecture = CString::default().into_raw(); // TODO(webgpu.h)
    info.device = CString::new(result.name).unwrap().into_raw();
    info.description = CString::new(result.driver_info).unwrap().into_raw();
    info.backendType = map_backend_type(result.backend);
    info.adapterType = map_adapter_type(result.device_type);
    info.vendorID = result.vendor;
    info.deviceID = result.device;
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterHasFeature(
    adapter: native::WGPUAdapter,
    feature: native::WGPUFeatureName,
) -> native::WGPUBool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let adapter_features = match gfx_select!(adapter_id => context.adapter_features(adapter_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(err, "wgpuAdapterHasFeature"),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false as native::WGPUBool,
    };

    adapter_features.contains(feature) as native::WGPUBool
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterInfoFreeMembers(adapter_info: native::WGPUAdapterInfo) {
    drop(CString::from_raw(
        adapter_info.vendor as *mut std::ffi::c_char,
    ));
    drop(CString::from_raw(
        adapter_info.architecture as *mut std::ffi::c_char,
    ));
    drop(CString::from_raw(
        adapter_info.device as *mut std::ffi::c_char,
    ));
    drop(CString::from_raw(
        adapter_info.description as *mut std::ffi::c_char,
    ));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: native::WGPUAdapter,
    descriptor: Option<&native::WGPUDeviceDescriptor>,
    callback: native::WGPUAdapterRequestDeviceCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let callback = callback.expect("invalid callback");

    let adapter_limits = match gfx_select!(adapter_id => context.adapter_limits(adapter_id)) {
        Ok(adapter_limits) => adapter_limits,
        Err(cause) => {
            let msg = CString::new(format_error(&cause)).unwrap();
            callback(
                native::WGPURequestDeviceStatus_Error,
                std::ptr::null(),
                msg.as_ptr(),
                userdata,
            );
            return;
        }
    };
    let base_limits = get_base_device_limits_from_adapter_limits(&adapter_limits);

    let (desc, trace_str, device_lost_handler, error_callback) = match descriptor {
        Some(descriptor) => {
            let (desc, trace_str, error_callback) = follow_chain!(
                map_device_descriptor((descriptor, base_limits),
                WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
            );
            let device_lost_handler = DeviceLostCallback {
                callback: descriptor.deviceLostCallback,
                userdata: descriptor.deviceLostUserdata,
            };
            (desc, trace_str, device_lost_handler, error_callback)
        }
        None => (
            wgt::DeviceDescriptor {
                required_limits: base_limits,
                ..Default::default()
            },
            std::ptr::null(),
            DEFAULT_DEVICE_LOST_HANDLER,
            None,
        ),
    };

    let (device_id, queue_id, err) = gfx_select!(adapter_id =>
        context.adapter_request_device(
            adapter_id,
            &desc,
            ptr_into_path(trace_str),
            None,
            None
        )
    );
    match err {
        None => {
            let message = CString::default();
            let mut error_sink = ErrorSinkRaw::new(device_lost_handler);
            if let Some(error_callback) = error_callback {
                error_sink.uncaptured_handler = error_callback;
            }

            callback(
                native::WGPURequestDeviceStatus_Success,
                Arc::into_raw(Arc::new(WGPUDeviceImpl {
                    context: context.clone(),
                    id: device_id,
                    queue: Arc::new(QueueId {
                        context: context.clone(),
                        id: queue_id,
                    }),
                    error_sink: Arc::new(Mutex::new(error_sink)),
                })),
                message.as_ptr(),
                userdata,
            );
        }
        Some(err) => {
            let message = CString::new(format_error(&err)).unwrap();
            callback(
                native::WGPURequestDeviceStatus_Error,
                std::ptr::null_mut(),
                message.as_ptr(),
                userdata,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterReference(adapter: native::WGPUAdapter) {
    assert!(!adapter.is_null(), "invalid adapter");
    Arc::increment_strong_count(adapter);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRelease(adapter: native::WGPUAdapter) {
    assert!(!adapter.is_null(), "invalid adapter");
    Arc::decrement_strong_count(adapter);
}

// BindGroup methods

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupReference(bind_group: native::WGPUBindGroup) {
    assert!(!bind_group.is_null(), "invalid bind group");
    Arc::increment_strong_count(bind_group);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupRelease(bind_group: native::WGPUBindGroup) {
    assert!(!bind_group.is_null(), "invalid bind group");
    Arc::decrement_strong_count(bind_group);
}

// BindGroupLayout methods

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupLayoutReference(
    bind_group_layout: native::WGPUBindGroupLayout,
) {
    assert!(!bind_group_layout.is_null(), "invalid bind group layout");
    Arc::increment_strong_count(bind_group_layout);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupLayoutRelease(
    bind_group_layout: native::WGPUBindGroupLayout,
) {
    assert!(!bind_group_layout.is_null(), "invalid bind group layout");
    Arc::decrement_strong_count(bind_group_layout);
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
) -> *const u8 {
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
        Err(err) => handle_error_fatal(err, "wgpuBufferGetConstMappedRange"),
    };

    buf.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut u8 {
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
        Err(err) => handle_error_fatal(err, "wgpuBufferGetMappedRange"),
    };

    buf.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetSize(buffer: native::WGPUBuffer) -> u64 {
    let buffer = buffer.as_ref().expect("invalid buffer");
    buffer.data.size
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetUsage(
    buffer: native::WGPUBuffer,
) -> native::WGPUBufferUsageFlags {
    let buffer = buffer.as_ref().expect("invalid buffer");
    buffer.data.usage
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferMapAsync(
    buffer: native::WGPUBuffer,
    mode: native::WGPUMapModeFlags,
    offset: usize,
    size: usize,
    callback: native::WGPUBufferMapAsyncCallback,
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
        callback: Some(wgc::resource::BufferMapCallback::from_rust(Box::new(
            move |result: resource::BufferAccessResult| {
                let status = match result {
                    Ok(()) => native::WGPUBufferMapAsyncStatus_Success,
                    Err(resource::BufferAccessError::Device(_)) => {
                        native::WGPUBufferMapAsyncStatus_DeviceLost
                    }
                    Err(resource::BufferAccessError::MapAlreadyPending) => {
                        native::WGPUBufferMapAsyncStatus_MappingAlreadyPending
                    }
                    Err(resource::BufferAccessError::InvalidBufferId(_))
                    | Err(resource::BufferAccessError::DestroyedResource(_)) => {
                        native::WGPUBufferMapAsyncStatus_DestroyedBeforeCallback
                    }
                    Err(_) => native::WGPUBufferMapAsyncStatus_ValidationError,
                    // TODO: WGPUBufferMapAsyncStatus_OffsetOutOfRange
                    // TODO: WGPUBufferMapAsyncStatus_SizeOutOfRange
                };

                callback(status, userdata.as_ptr());
            },
        ))),
    };

    if let Err(cause) = gfx_select!(buffer_id => context.buffer_map_async(
        buffer_id,
        offset as wgt::BufferAddress,
        Some(size as wgt::BufferAddress),
        operation,
    )) {
        handle_error(error_sink, cause, None, "wgpuBufferMapAsync");
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferUnmap(buffer: native::WGPUBuffer) {
    let (buffer_id, context, error_sink) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context, &buffer.error_sink)
    };

    if let Err(cause) = gfx_select!(buffer_id => context.buffer_unmap(buffer_id)) {
        handle_error(error_sink, cause, None, "wgpuBufferUnmap");
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferReference(buffer: native::WGPUBuffer) {
    assert!(!buffer.is_null(), "invalid buffer");
    Arc::increment_strong_count(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuBufferRelease(buffer: native::WGPUBuffer) {
    assert!(!buffer.is_null(), "invalid buffer");
    Arc::decrement_strong_count(buffer);
}

// CommandBuffer methods

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandBufferReference(command_buffer: native::WGPUCommandBuffer) {
    assert!(!command_buffer.is_null(), "invalid command buffer");
    Arc::increment_strong_count(command_buffer);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuCommandBufferRelease(command_buffer: native::WGPUCommandBuffer) {
    assert!(!command_buffer.is_null(), "invalid command buffer");
    Arc::decrement_strong_count(command_buffer);
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

    let timestamp_writes = descriptor.and_then(|descriptor| {
        descriptor.timestampWrites.as_ref().map(|timestamp_write| {
            wgc::command::PassTimestampWrites {
                query_set: timestamp_write
                    .querySet
                    .as_ref()
                    .expect("invalid query set in timestamp writes")
                    .id,
                beginning_of_pass_write_index: map_query_set_index(
                    timestamp_write.beginningOfPassWriteIndex,
                ),
                end_of_pass_write_index: map_query_set_index(timestamp_write.endOfPassWriteIndex),
            }
        })
    });

    let desc = match descriptor {
        Some(descriptor) => wgc::command::ComputePassDescriptor {
            label: ptr_into_label(descriptor.label),
            timestamp_writes: timestamp_writes.as_ref(),
        },
        None => wgc::command::ComputePassDescriptor::default(),
    };

    let (pass, err) = gfx_select!(command_encoder_id => context.command_encoder_create_compute_pass_dyn(command_encoder_id, &desc));
    if let Some(cause) = err {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuCommandEncoderBeginComputePass",
        );
    }
    Arc::into_raw(Arc::new(WGPUComputePassEncoderImpl {
        context: context.clone(),
        encoder: Box::into_raw(pass),
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
                load_op: conv::map_load_op(desc.depthLoadOp).unwrap_or(wgc::command::LoadOp::Load),
                store_op: conv::map_store_op(desc.depthStoreOp)
                    .unwrap_or(wgc::command::StoreOp::Store),
                clear_value: desc.depthClearValue,
                read_only: desc.depthReadOnly != 0,
            },
            stencil: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.stencilLoadOp)
                    .unwrap_or(wgc::command::LoadOp::Load),
                store_op: conv::map_store_op(desc.stencilStoreOp)
                    .unwrap_or(wgc::command::StoreOp::Store),
                clear_value: desc.stencilClearValue,
                read_only: desc.stencilReadOnly != 0,
            },
        }
    });

    let timestamp_writes = descriptor.timestampWrites.as_ref().map(|timestamp_write| {
        wgc::command::PassTimestampWrites {
            query_set: timestamp_write
                .querySet
                .as_ref()
                .expect("invalid query set in timestamp writes")
                .id,
            beginning_of_pass_write_index: map_query_set_index(
                timestamp_write.beginningOfPassWriteIndex,
            ),
            end_of_pass_write_index: map_query_set_index(timestamp_write.endOfPassWriteIndex),
        }
    });

    let desc = wgc::command::RenderPassDescriptor {
        label: ptr_into_label(descriptor.label),
        color_attachments: Cow::Owned(
            make_slice(descriptor.colorAttachments, descriptor.colorAttachmentCount)
                .iter()
                .map(|color_attachment| {
                    if color_attachment.depthSlice != native::WGPU_DEPTH_SLICE_UNDEFINED {
                        log::warn!("Depth slice on color attachments is not implemented");
                    }

                    color_attachment.view.as_ref().map(|view| {
                        wgc::command::RenderPassColorAttachment {
                            view: view.id,
                            resolve_target: color_attachment.resolveTarget.as_ref().map(|v| v.id),
                            channel: wgc::command::PassChannel {
                                load_op: conv::map_load_op(color_attachment.loadOp)
                                    .expect("invalid load op for render pass color attachment"),
                                store_op: conv::map_store_op(color_attachment.storeOp)
                                    .expect("invalid store op for render pass color attachment"),
                                clear_value: conv::map_color(&color_attachment.clearValue),
                                read_only: false,
                            },
                        }
                    })
                })
                .collect(),
        ),
        depth_stencil_attachment: depth_stencil_attachment.as_ref(),
        timestamp_writes: timestamp_writes.as_ref(),
        occlusion_query_set: descriptor.occlusionQuerySet.as_ref().map(|v| v.id),
    };

    let (pass, err) = gfx_select!(command_encoder_id => context.command_encoder_create_render_pass_dyn(command_encoder_id, &desc));
    if let Some(cause) = err {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuCommandEncoderBeginRenderPass",
        );
    }
    Arc::into_raw(Arc::new(WGPURenderPassEncoderImpl {
        context: context.clone(),
        encoder: Box::into_raw(pass),
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
            _ => Some(size),
        }
    )) {
        handle_error(error_sink, cause, None, "wgpuCommandEncoderClearBuffer");
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
            error_sink,
            cause,
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
            error_sink,
            cause,
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
            error_sink,
            cause,
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
            error_sink,
            cause,
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
    let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
    let (command_encoder_id, context, error_sink) = (
        command_encoder.id,
        &command_encoder.context,
        &command_encoder.error_sink,
    );
    command_encoder.open.store(false, atomic::Ordering::SeqCst);

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandBufferDescriptor {
            label: ptr_into_label(descriptor.label),
        },
        None => wgt::CommandBufferDescriptor::default(),
    };

    let (command_buffer_id, error) = gfx_select!(command_encoder_id => context.command_encoder_finish(command_encoder_id, &desc));
    if let Some(cause) = error {
        handle_error(error_sink, cause, None, "wgpuCommandEncoderFinish");
    }

    Arc::into_raw(Arc::new(WGPUCommandBufferImpl {
        context: context.clone(),
        id: command_buffer_id,
        open: atomic::AtomicBool::new(true),
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
            error_sink,
            cause,
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
        handle_error(error_sink, cause, None, "wgpuCommandEncoderPopDebugGroup");
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
        handle_error(error_sink, cause, None, "wgpuCommandEncoderPushDebugGroup");
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
        handle_error(error_sink, cause, None, "wgpuCommandEncoderResolveQuerySet");
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
        handle_error(error_sink, cause, None, "wgpuCommandEncoderWriteTimestamp");
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderReference(command_encoder: native::WGPUCommandEncoder) {
    assert!(!command_encoder.is_null(), "invalid command encoder");
    Arc::increment_strong_count(command_encoder);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderRelease(command_encoder: native::WGPUCommandEncoder) {
    assert!(!command_encoder.is_null(), "invalid command encoder");
    Arc::decrement_strong_count(command_encoder);
}

// ComputePassEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroups(
    pass: native::WGPUComputePassEncoder,
    workgroup_count_x: u32,
    workgroup_count_y: u32,
    workgroup_count_z: u32,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.dispatch_workgroups(
        &pass.context,
        workgroup_count_x,
        workgroup_count_y,
        workgroup_count_z,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderDispatchWorkgroups",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroupsIndirect(
    pass: native::WGPUComputePassEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;

    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.dispatch_workgroups_indirect(&pass.context, indirect_buffer_id, indirect_offset) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderDispatchWorkgroupsIndirect",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass: native::WGPUComputePassEncoder) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.end(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(&pass.error_sink, cause, None, "wgpuComputePassEncoderEnd"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderInsertDebugMarker(
    pass: native::WGPUComputePassEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.insert_debug_marker(
        &pass.context,
        CStr::from_ptr(marker_label).to_str().unwrap(),
        0,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderInsertDebugMarker",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPopDebugGroup(pass: native::WGPUComputePassEncoder) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.pop_debug_group(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderPopDebugGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    pass: native::WGPUComputePassEncoder,
    group_label: *const std::ffi::c_char,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.push_debug_group(
        &pass.context,
        CStr::from_ptr(group_label).to_str().unwrap(),
        0,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderPushDebugGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: native::WGPUComputePassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: usize,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    //TODO: as per webgpu.h bindgroup is nullable
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_bind_group(
        &pass.context,
        group_index,
        bind_group_id,
        make_slice(dynamic_offsets, dynamic_offset_count),
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderSetBindGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: native::WGPUComputePassEncoder,
    compute_pipeline: native::WGPUComputePipeline,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let compute_pipeline_id = compute_pipeline
        .as_ref()
        .expect("invalid compute pipeline")
        .id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_pipeline(&pass.context, compute_pipeline_id) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderSetPipeline",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderReference(
    compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    assert!(
        !compute_pass_encoder.is_null(),
        "invalid command pass encoder"
    );
    Arc::increment_strong_count(compute_pass_encoder);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderRelease(
    compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    assert!(
        !compute_pass_encoder.is_null(),
        "invalid command pass encoder"
    );
    Arc::decrement_strong_count(compute_pass_encoder);
}

// ComputePipeline methods

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineGetBindGroupLayout(
    pipeline: native::WGPUComputePipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (pipeline_id, context, error_sink) = {
        let pipeline = pipeline.as_ref().expect("invalid pipeline");
        (pipeline.id, &pipeline.context, &pipeline.error_sink)
    };

    let (bind_group_layout_id, error) = gfx_select!(pipeline_id => context.compute_pipeline_get_bind_group_layout(pipeline_id, group_index, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            None,
            "wgpuComputePipelineGetBindGroupLayout",
        );
    }

    Arc::into_raw(Arc::new(WGPUBindGroupLayoutImpl {
        context: context.clone(),
        id: bind_group_layout_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineReference(
    compute_pipeline: native::WGPUComputePipeline,
) {
    assert!(!compute_pipeline.is_null(), "invalid command pipeline");
    Arc::increment_strong_count(compute_pipeline);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineRelease(compute_pipeline: native::WGPUComputePipeline) {
    assert!(!compute_pipeline.is_null(), "invalid command pipeline");
    Arc::decrement_strong_count(compute_pipeline);
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

    let entries = make_slice(descriptor.entries, descriptor.entryCount)
        .iter()
        .map(|entry| {
            follow_chain!(map_bind_group_entry((entry),
                WGPUSType_BindGroupEntryExtras => native::WGPUBindGroupEntryExtras)
            )
        })
        .collect::<Vec<_>>();

    let desc = wgc::binding_model::BindGroupDescriptor {
        label: ptr_into_label(descriptor.label),
        layout: bind_group_layout_id,
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_id, error) =
        gfx_select!(device_id => context.device_create_bind_group(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, desc.label, "wgpuDeviceCreateBindGroup");
    }

    Arc::into_raw(Arc::new(WGPUBindGroupImpl {
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

    let entries = make_slice(descriptor.entries, descriptor.entryCount)
        .iter()
        .map(|entry| {
            follow_chain!(map_bind_group_layout_entry((entry),
                WGPUSType_BindGroupLayoutEntryExtras => native::WGPUBindGroupLayoutEntryExtras)
            )
        })
        .collect::<Vec<_>>();

    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: ptr_into_label(descriptor.label),
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_layout_id, error) =
        gfx_select!(device_id => context.device_create_bind_group_layout(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreateBindGroupLayout",
        );
    }

    Arc::into_raw(Arc::new(WGPUBindGroupLayoutImpl {
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

    let desc = wgt::BufferDescriptor {
        label: ptr_into_label(descriptor.label),
        size: descriptor.size,
        usage: wgt::BufferUsages::from_bits(descriptor.usage).expect("invalid buffer usage"),
        mapped_at_creation: descriptor.mappedAtCreation != 0,
    };

    let (buffer_id, error) =
        gfx_select!(device_id => context.device_create_buffer(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, desc.label, "wgpuDeviceCreateBuffer");
    }

    Arc::into_raw(Arc::new(WGPUBufferImpl {
        context: context.clone(),
        id: buffer_id,
        error_sink: error_sink.clone(),
        data: BufferData {
            usage: descriptor.usage,
            size: descriptor.size,
        },
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
            label: ptr_into_label(descriptor.label),
        },
        None => wgt::CommandEncoderDescriptor::default(),
    };
    let (command_encoder_id, error) =
        gfx_select!(device_id => context.device_create_command_encoder(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreateCommandEncoder",
        );
    }

    Arc::into_raw(Arc::new(WGPUCommandEncoderImpl {
        context: context.clone(),
        id: command_encoder_id,
        error_sink: error_sink.clone(),
        open: atomic::AtomicBool::new(true),
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

    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: ptr_into_label(descriptor.label),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        stage: wgc::pipeline::ProgrammableStageDescriptor {
            module: descriptor
                .compute
                .module
                .as_ref()
                .expect("invalid fragment shader module for render pipeline descriptor")
                .id
                .expect("invalid fragment shader module for render pipeline descriptor"),
            entry_point: ptr_into_label(descriptor.compute.entryPoint),
            constants: Cow::Owned(
                make_slice(
                    descriptor.compute.constants,
                    descriptor.compute.constantCount,
                )
                .iter()
                .map(|entry| {
                    (
                        CStr::from_ptr(entry.key).to_str().unwrap().to_string(),
                        entry.value,
                    )
                })
                .collect(),
            ),
            // TODO(wgpu.h)
            zero_initialize_workgroup_memory: false,
            // TODO(wgpu.h)
            vertex_pulling_transform: false,
        },
        // TODO(wgpu.h)
        cache: None,
    };

    let (compute_pipeline_id, error) = gfx_select!(device_id => context.device_create_compute_pipeline(
        device_id,
        &desc,
        None,
        None
    ));
    if let Some(cause) = error {
        if let wgc::pipeline::CreateComputePipelineError::Internal(ref error) = cause {
            log::warn!(
                "Shader translation error for stage {:?}: {}",
                wgt::ShaderStages::COMPUTE,
                error
            );
            log::warn!("Please report it to https://github.com/gfx-rs/wgpu");
        }
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreateComputePipeline",
        );
    }

    Arc::into_raw(Arc::new(WGPUComputePipelineImpl {
        context: context.clone(),
        id: compute_pipeline_id,
        error_sink: error_sink.clone(),
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
            (descriptor),
            WGPUSType_PipelineLayoutExtras => native::WGPUPipelineLayoutExtras)
    );
    let (pipeline_layout_id, error) =
        gfx_select!(device_id => context.device_create_pipeline_layout(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreatePipelineLayout",
        );
    }

    Arc::into_raw(Arc::new(WGPUPipelineLayoutImpl {
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
    let descriptor = descriptor.expect("invalid query set descriptor");

    let desc = follow_chain!(
        map_query_set_descriptor(
            (descriptor),
            WGPUSType_QuerySetDescriptorExtras => native::WGPUQuerySetDescriptorExtras)
    );

    let (query_set_id, error) =
        gfx_select!(device_id => context.device_create_query_set(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, desc.label, "wgpuDeviceCreateQuerySet");
    }

    Arc::into_raw(Arc::new(WGPUQuerySetImpl {
        context: context.clone(),
        id: query_set_id,
        data: QuerySetData {
            query_type: descriptor.type_,
            query_count: descriptor.count,
        },
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
        label: ptr_into_label(descriptor.label),
        color_formats: make_slice(descriptor.colorFormats, descriptor.colorFormatCount)
            .iter()
            .map(|format| conv::map_texture_format(*format))
            .collect(),
        depth_stencil: conv::map_texture_format(descriptor.depthStencilFormat).map(|format| {
            wgt::RenderBundleDepthStencil {
                format,
                depth_read_only: descriptor.depthReadOnly != 0,
                stencil_read_only: descriptor.stencilReadOnly != 0,
            }
        }),
        sample_count: descriptor.sampleCount,
        multiview: None,
    };

    match wgc::command::RenderBundleEncoder::new(&desc, device_id, None) {
        Ok(encoder) => Arc::into_raw(Arc::new(WGPURenderBundleEncoderImpl {
            context: context.clone(),
            encoder: Box::into_raw(Box::new(Some(Box::into_raw(Box::new(encoder))))),
        })),
        Err(cause) => {
            handle_error_fatal(cause, "wgpuDeviceCreateRenderBundleEncoder");
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
        label: ptr_into_label(descriptor.label),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor
                    .vertex
                    .module
                    .as_ref()
                    .expect("invalid vertex shader module for vertex state")
                    .id
                    .expect("invalid vertex shader module for vertex state"),
                entry_point: ptr_into_label(descriptor.vertex.entryPoint),
                constants: Cow::Owned(
                    make_slice(descriptor.vertex.constants, descriptor.vertex.constantCount)
                        .iter()
                        .map(|entry| {
                            (
                                CStr::from_ptr(entry.key).to_str().unwrap().to_string(),
                                entry.value,
                            )
                        })
                        .collect(),
                ),
                // TODO(wgpu.h)
                zero_initialize_workgroup_memory: false,
                // TODO(wgpu.h)
                vertex_pulling_transform: false,
            },
            buffers: Cow::Owned(
                make_slice(descriptor.vertex.buffers, descriptor.vertex.bufferCount)
                    .iter()
                    .map(|buffer| wgc::pipeline::VertexBufferLayout {
                        array_stride: buffer.arrayStride,
                        step_mode: match buffer.stepMode {
                            native::WGPUVertexStepMode_Vertex => wgt::VertexStepMode::Vertex,
                            native::WGPUVertexStepMode_Instance => wgt::VertexStepMode::Instance,
                            _ => panic!("invalid vertex step mode for vertex buffer layout"),
                        },
                        attributes: Cow::Owned(
                            make_slice(buffer.attributes, buffer.attributeCount)
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
            unclipped_depth: follow_chain!(
                map_primitive_state(
                    (&descriptor.primitive),
                    WGPUSType_PrimitiveDepthClipControl => native::WGPUPrimitiveDepthClipControl
                )
            ),
            polygon_mode: wgt::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: descriptor
            .depthStencil
            .as_ref()
            .map(|desc| wgt::DepthStencilState {
                format: conv::map_texture_format(desc.format)
                    .expect("invalid texture format for depth stencil state"),
                depth_write_enabled: desc.depthWriteEnabled != 0,
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
            alpha_to_coverage_enabled: descriptor.multisample.alphaToCoverageEnabled != 0,
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
                        .id
                        .expect("invalid fragment shader module for render pipeline descriptor"),
                    entry_point: ptr_into_label(fragment.entryPoint),
                    constants: Cow::Owned(
                        make_slice(fragment.constants, fragment.constantCount)
                            .iter()
                            .map(|entry| {
                                (
                                    CStr::from_ptr(entry.key).to_str().unwrap().to_string(),
                                    entry.value,
                                )
                            })
                            .collect(),
                    ),
                    // TODO(wgpu.h)
                    zero_initialize_workgroup_memory: false,
                    // TODO(wgpu.h)
                    vertex_pulling_transform: false,
                },
                targets: Cow::Owned(
                    make_slice(fragment.targets, fragment.targetCount)
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
        // TODO(wgpu.h)
        multiview: None,
        // TODO(wgpu.h)
        cache: None,
    };

    let (render_pipeline_id, error) = gfx_select!(device_id => context.device_create_render_pipeline(device_id, &desc, None, None));
    if let Some(cause) = error {
        if let wgc::pipeline::CreateRenderPipelineError::Internal { stage, ref error } = cause {
            log::error!("Shader translation error for stage {:?}: {}", stage, error);
            log::error!("Please report it to https://github.com/gfx-rs/wgpu");
        }
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreateRenderPipeline",
        );
    }

    Arc::into_raw(Arc::new(WGPURenderPipelineImpl {
        context: context.clone(),
        id: render_pipeline_id,
        error_sink: error_sink.clone(),
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
            label: ptr_into_label(descriptor.label),
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
            // TODO(wgpu.h)
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
        gfx_select!(device_id => context.device_create_sampler(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, desc.label, "wgpuDeviceCreateSampler");
    }

    Arc::into_raw(Arc::new(WGPUSamplerImpl {
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

    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: ptr_into_label(descriptor.label),
        shader_bound_checks: wgt::ShaderBoundChecks::default(),
    };

    let source = match follow_chain!(
        map_shader_module((descriptor),
        WGPUSType_ShaderModuleSPIRVDescriptor => native::WGPUShaderModuleSPIRVDescriptor,
        WGPUSType_ShaderModuleWGSLDescriptor => native::WGPUShaderModuleWGSLDescriptor,
        WGPUSType_ShaderModuleGLSLDescriptor => native::WGPUShaderModuleGLSLDescriptor)
    ) {
        Ok(source) => source,
        Err(cause) => {
            handle_error(
                error_sink,
                cause,
                desc.label,
                "wgpuDeviceCreateShaderModule",
            );

            return Arc::into_raw(Arc::new(WGPUShaderModuleImpl {
                context: context.clone(),
                id: None,
            }));
        }
    };

    let (shader_module_id, error) = gfx_select!(device_id => context.device_create_shader_module(device_id, &desc, source, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            desc.label,
            "wgpuDeviceCreateShaderModule",
        );
    }

    Arc::into_raw(Arc::new(WGPUShaderModuleImpl {
        context: context.clone(),
        id: Some(shader_module_id),
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
        label: ptr_into_label(descriptor.label),
        size: conv::map_extent3d(&descriptor.size),
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: conv::map_texture_dimension(descriptor.dimension),
        format: conv::map_texture_format(descriptor.format)
            .expect("invalid texture format for texture descriptor"),
        usage: wgt::TextureUsages::from_bits(descriptor.usage)
            .expect("invalid texture usage for texture descriptor"),
        view_formats: make_slice(descriptor.viewFormats, descriptor.viewFormatCount)
            .iter()
            .map(|v| {
                conv::map_texture_format(*v).expect("invalid view format for texture descriptor")
            })
            .collect(),
    };

    let (texture_id, error) =
        gfx_select!(device_id => context.device_create_texture(device_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, desc.label, "wgpuDeviceCreateTexture");
    }

    Arc::into_raw(Arc::new(WGPUTextureImpl {
        context: context.clone(),
        id: texture_id,
        error_sink: error_sink.clone(),
        surface_id: None,
        has_surface_presented: Arc::default(),
        data: TextureData {
            usage: descriptor.usage,
            dimension: descriptor.dimension,
            size: descriptor.size,
            format: descriptor.format,
            mip_level_count: descriptor.mipLevelCount,
            sample_count: descriptor.sampleCount,
        },
    }))
}

#[no_mangle]
pub extern "C" fn wgpuDeviceDestroy(_device: native::WGPUDevice) {
    //TODO: needs to be implemented in wgpu-core
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
        Err(err) => handle_error_fatal(err, "wgpuDeviceEnumerateFeatures"),
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
) -> native::WGPUBool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(device_id => context.device_limits(device_id));
    match result {
        Ok(wgt_limits) => conv::write_limits_struct(wgt_limits, limits),
        Err(err) => handle_error_fatal(err, "wgpuDeviceGetLimits"),
    }

    true as native::WGPUBool // indicates that we can fill WGPUChainedStructOut
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetQueue(device: native::WGPUDevice) -> native::WGPUQueue {
    let (queue, error_sink) = {
        let device = device.as_ref().expect("invalid device");
        (&device.queue, &device.error_sink)
    };

    Arc::into_raw(Arc::new(WGPUQueueImpl {
        queue: queue.clone(),
        error_sink: error_sink.clone(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceHasFeature(
    device: native::WGPUDevice,
    feature: native::WGPUFeatureName,
) -> native::WGPUBool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let device_features = match gfx_select!(device_id => context.device_features(device_id)) {
        Ok(features) => features,
        Err(err) => handle_error_fatal(err, "wgpuDeviceHasFeature"),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false as native::WGPUBool,
    };

    device_features.contains(feature) as native::WGPUBool
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePopErrorScope(
    device: native::WGPUDevice,
    callback: native::WGPUErrorCallback,
    userdata: *mut ::std::os::raw::c_void,
) {
    let device = device.as_ref().expect("invalid device");
    let callback = callback.expect("invalid callback");
    let mut error_sink = device.error_sink.lock();
    let scope = error_sink.scopes.pop().unwrap();

    match scope.error {
        Some(error) => {
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
            };
        }
        None => {
            let msg = CString::default();
            unsafe {
                callback(native::WGPUErrorType_NoError, msg.as_ptr(), userdata);
            };
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePushErrorScope(
    device: native::WGPUDevice,
    filter: native::WGPUErrorFilter,
) {
    let device = device.as_ref().expect("invalid device");
    let mut error_sink = device.error_sink.lock();
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
pub unsafe extern "C" fn wgpuDeviceReference(device: native::WGPUDevice) {
    assert!(!device.is_null(), "invalid device");
    Arc::increment_strong_count(device);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceRelease(device: native::WGPUDevice) {
    assert!(!device.is_null(), "invalid device");
    Arc::decrement_strong_count(device);
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
        map_surface((descriptor),
            WGPUSType_SurfaceDescriptorFromWindowsHWND => native::WGPUSurfaceDescriptorFromWindowsHWND,
            WGPUSType_SurfaceDescriptorFromXcbWindow => native::WGPUSurfaceDescriptorFromXcbWindow,
            WGPUSType_SurfaceDescriptorFromXlibWindow => native::WGPUSurfaceDescriptorFromXlibWindow,
            WGPUSType_SurfaceDescriptorFromWaylandSurface => native::WGPUSurfaceDescriptorFromWaylandSurface,
            WGPUSType_SurfaceDescriptorFromMetalLayer => native::WGPUSurfaceDescriptorFromMetalLayer,
            WGPUSType_SurfaceDescriptorFromAndroidNativeWindow => native::WGPUSurfaceDescriptorFromAndroidNativeWindow)
    );

    let surface_id = match create_surface_params {
        CreateSurfaceParams::Raw((rdh, rwh)) => {
            match context.instance_create_surface(rdh, rwh, None) {
                Ok(surface_id) => surface_id,
                Err(cause) => handle_error_fatal(cause, "wgpuInstanceCreateSurface"),
            }
        }
        #[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "metal"))]
        CreateSurfaceParams::Metal(layer) => {
            match context.instance_create_surface_metal(layer, None) {
                Ok(surface_id) => surface_id,
                Err(cause) => handle_error_fatal(cause, "wgpuInstanceCreateSurface"),
            }
        }
    };

    Arc::into_raw(Arc::new(WGPUSurfaceImpl {
        context: context.clone(),
        id: surface_id,
        data: Mutex::default(),
        has_surface_presented: Arc::default(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceProcessEvents(instance: native::WGPUInstance) {
    let instance = instance.as_ref().expect("invalid instance");
    let context = &instance.context;

    match context.poll_all_devices(false) {
        Ok(_queue_empty) => (),
        Err(cause) => {
            handle_error_fatal(cause, "wgpuInstanceProcessEvents");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceRequestAdapter(
    instance: native::WGPUInstance,
    options: Option<&native::WGPURequestAdapterOptions>,
    callback: native::WGPUInstanceRequestAdapterCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let instance = instance.as_ref().expect("invalid instance");
    let context = &instance.context;
    let callback = callback.expect("invalid callback");

    let (desc, inputs) = match options {
        Some(options) => (
            wgt::RequestAdapterOptions {
                power_preference: match options.powerPreference {
                    native::WGPUPowerPreference_LowPower => wgt::PowerPreference::LowPower,
                    native::WGPUPowerPreference_HighPerformance => {
                        wgt::PowerPreference::HighPerformance
                    }
                    _ => wgt::PowerPreference::default(),
                },
                force_fallback_adapter: options.forceFallbackAdapter != 0,
                compatible_surface: options.compatibleSurface.as_ref().map(|surface| surface.id),
            },
            wgc::instance::AdapterInputs::Mask(
                match options.backendType {
                    native::WGPUBackendType_Undefined => wgt::Backends::all(),
                    native::WGPUBackendType_Null => wgt::Backends::empty(),
                    native::WGPUBackendType_WebGPU => wgt::Backends::BROWSER_WEBGPU,
                    native::WGPUBackendType_D3D12 => wgt::Backends::DX12,
                    native::WGPUBackendType_Metal => wgt::Backends::METAL,
                    native::WGPUBackendType_Vulkan => wgt::Backends::VULKAN,
                    native::WGPUBackendType_OpenGL => wgt::Backends::GL,
                    native::WGPUBackendType_OpenGLES => wgt::Backends::GL,
                    native::WGPUBackendType_D3D11 => {
                        callback(
                            native::WGPURequestAdapterStatus_Error,
                            std::ptr::null_mut(),
                            "unsupported backend type: d3d11".as_ptr() as _,
                            userdata,
                        );
                        return;
                    }
                    backend_type => panic!("invalid backend type: 0x{backend_type:08X}"),
                },
                |_| None,
            ),
        ),
        None => (
            wgt::RequestAdapterOptions::default(),
            wgc::instance::AdapterInputs::Mask(wgt::Backends::all(), |_| None),
        ),
    };

    match context.request_adapter(&desc, inputs) {
        Ok(adapter_id) => {
            let message = CString::default();
            callback(
                native::WGPURequestAdapterStatus_Success,
                Arc::into_raw(Arc::new(WGPUAdapterImpl {
                    context: context.clone(),
                    id: adapter_id,
                })),
                message.as_ptr(),
                userdata,
            );
        }
        Err(err) => {
            let message = CString::new(format_error(&err)).unwrap();
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

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceEnumerateAdapters(
    instance: native::WGPUInstance,
    options: Option<&native::WGPUInstanceEnumerateAdapterOptions>,
    adapters: *mut native::WGPUAdapter,
) -> usize {
    let instance = instance.as_ref().expect("invalid instance");
    let context = &instance.context;

    let inputs = match options {
        Some(options) => wgc::instance::AdapterInputs::Mask(
            map_instance_backend_flags(options.backends as native::WGPUInstanceBackend),
            |_| None,
        ),
        None => wgc::instance::AdapterInputs::Mask(wgt::Backends::all(), |_| None),
    };

    let result = context.enumerate_adapters(inputs);
    let count = result.len();

    if !adapters.is_null() {
        let temp = std::slice::from_raw_parts_mut(adapters, count);

        result.iter().enumerate().for_each(|(i, id)| {
            // It's users responsibility to drop the adapters they
            // don't need.

            temp[i] = Arc::into_raw(Arc::new(WGPUAdapterImpl {
                context: context.clone(),
                id: *id,
            }));
        });
    } else {
        // Drop all the adapters when only counting length.

        result
            .iter()
            .for_each(|id| gfx_select!(id => context.adapter_drop(*id)));
    }

    count
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceReference(instance: native::WGPUInstance) {
    assert!(!instance.is_null(), "invalid instance");
    Arc::increment_strong_count(instance);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceRelease(instance: native::WGPUInstance) {
    assert!(!instance.is_null(), "invalid instance");
    Arc::decrement_strong_count(instance);
}

// PipelineLayout methods

#[no_mangle]
pub unsafe extern "C" fn wgpuPipelineLayoutReference(pipeline_layout: native::WGPUPipelineLayout) {
    assert!(!pipeline_layout.is_null(), "invalid pipeline layout");
    Arc::increment_strong_count(pipeline_layout);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuPipelineLayoutRelease(pipeline_layout: native::WGPUPipelineLayout) {
    assert!(!pipeline_layout.is_null(), "invalid pipeline layout");
    Arc::decrement_strong_count(pipeline_layout);
}

// QuerySet methods

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetDestroy(_query_set: native::WGPUQuerySet) {
    //TODO: needs to be implemented in wgpu-core
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetGetCount(query_set: native::WGPUQuerySet) -> u32 {
    let query_set = query_set.as_ref().expect("invalid query set");
    query_set.data.query_count
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetGetType(
    query_set: native::WGPUQuerySet,
) -> native::WGPUQueryType {
    let query_set = query_set.as_ref().expect("invalid query set");
    query_set.data.query_type
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetReference(query_set: native::WGPUQuerySet) {
    assert!(!query_set.is_null(), "invalid query set");
    Arc::increment_strong_count(query_set);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetRelease(query_set: native::WGPUQuerySet) {
    assert!(!query_set.is_null(), "invalid query set");
    Arc::decrement_strong_count(query_set);
}

// Queue methods

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueOnSubmittedWorkDone(
    queue: native::WGPUQueue,
    callback: native::WGPUQueueOnSubmittedWorkDoneCallback,
    userdata: *mut ::std::os::raw::c_void,
) {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.queue.id, &queue.queue.context)
    };
    let callback = callback.expect("invalid callback");
    let userdata = utils::Userdata::new(userdata);

    let closure = wgc::device::queue::SubmittedWorkDoneClosure::from_rust(Box::new(move || {
        callback(native::WGPUQueueWorkDoneStatus_Success, userdata.as_ptr());
    }));

    if let Err(cause) =
        gfx_select!(queue_id => context.queue_on_submitted_work_done(queue_id, closure))
    {
        handle_error_fatal(cause, "wgpuQueueOnSubmittedWorkDone");
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: native::WGPUQueue,
    command_count: usize,
    commands: *const native::WGPUCommandBuffer,
) {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.queue.id, &queue.queue.context)
    };

    let command_buffers = make_slice(commands, command_count)
        .iter()
        .map(|command_buffer| {
            let command_buffer = command_buffer.as_ref().expect("invalid command buffer");
            command_buffer.open.store(false, atomic::Ordering::SeqCst);
            command_buffer.id
        })
        .collect::<SmallVec<[_; 4]>>();

    if let Err(cause) = gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers)) {
        handle_error_fatal(cause, "wgpuQueueSubmit");
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
        (queue.queue.id, &queue.queue.context, &queue.error_sink)
    };
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    if let Err(cause) = gfx_select!(queue_id => context.queue_write_buffer(
        queue_id,
        buffer_id,
        buffer_offset,
        make_slice(data, data_size)
    )) {
        handle_error(error_sink, cause, None, "wgpuQueueWriteBuffer");
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
        (queue.queue.id, &queue.queue.context, &queue.error_sink)
    };

    if let Err(cause) = gfx_select!(queue_id => context.queue_write_texture(
        queue_id,
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        make_slice(data, data_size),
        &conv::map_texture_data_layout(data_layout.expect("invalid data layout")),
        &conv::map_extent3d(write_size.expect("invalid write size"))
    )) {
        handle_error(error_sink, cause, None, "wgpuQueueWriteTexture");
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueReference(queue: native::WGPUQueue) {
    assert!(!queue.is_null(), "invalid queue");
    Arc::increment_strong_count(queue);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuQueueRelease(queue: native::WGPUQueue) {
    assert!(!queue.is_null(), "invalid queue");
    Arc::decrement_strong_count(queue);
}

// RenderBundle methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleReference(render_bundle: native::WGPURenderBundle) {
    assert!(!render_bundle.is_null(), "invalid render bundle");
    Arc::increment_strong_count(render_bundle);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleRelease(render_bundle: native::WGPURenderBundle) {
    assert!(!render_bundle.is_null(), "invalid render bundle");
    Arc::decrement_strong_count(render_bundle);
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_draw(
        encoder,
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_draw_indexed(
        encoder,
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
        encoder,
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_draw_indirect(encoder, indirect_buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderFinish(
    bundle: native::WGPURenderBundleEncoder,
    descriptor: Option<&native::WGPURenderBundleDescriptor>,
) -> native::WGPURenderBundle {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let context = &bundle.context;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.take().expect("invalid render bundle");
    let encoder = Box::from_raw(encoder);

    let desc = match descriptor {
        Some(descriptor) => wgt::RenderBundleDescriptor {
            label: ptr_into_label(descriptor.label),
        },
        None => wgt::RenderBundleDescriptor::default(),
    };

    let (render_bundle_id, error) = gfx_select!(encoder.parent() => context.render_bundle_encoder_finish(*encoder, &desc, None));
    if let Some(cause) = error {
        handle_error_fatal(cause, "wgpuRenderBundleEncoderFinish");
    }

    Arc::into_raw(Arc::new(WGPURenderBundleImpl {
        context: context.clone(),
        id: render_bundle_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderInsertDebugMarker(
    bundle: native::WGPURenderBundleEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_insert_debug_marker(encoder, marker_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPopDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
) {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_pop_debug_group(encoder);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPushDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_label: *const std::ffi::c_char,
) {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_push_debug_group(encoder, group_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetBindGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: usize,
    dynamic_offsets: *const u32,
) {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    // TODO: as per webgpu.h bindgroup is nullable
    let bind_group_id = group.as_ref().expect("invalid bind group").id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_set_bind_group(
        encoder,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count,
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_set_index_buffer(
        encoder,
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
    let bundle = bundle.as_ref().expect("invalid render bundle");
    let pipeline_id = pipeline.as_ref().expect("invalid render pipeline").id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_set_pipeline(encoder, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetVertexBuffer(
    bundle: native::WGPURenderBundleEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let bundle = bundle.as_ref().expect("invalid render bundle");
    // TODO: as per webgpu.h buffer is nullable
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = bundle.encoder.as_mut().expect("invalid render bundle");
    let encoder = encoder.expect("invalid render bundle");
    let encoder = encoder.as_mut().unwrap();

    bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
        encoder,
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
pub unsafe extern "C" fn wgpuRenderBundleEncoderReference(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    assert!(
        !render_bundle_encoder.is_null(),
        "invalid render bundle encoder"
    );
    Arc::increment_strong_count(render_bundle_encoder);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderRelease(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    assert!(
        !render_bundle_encoder.is_null(),
        "invalid render bundle encoder"
    );
    Arc::decrement_strong_count(render_bundle_encoder);
}

// RenderPassEncoder methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderBeginOcclusionQuery(
    pass: native::WGPURenderPassEncoder,
    query_index: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.begin_occlusion_query(&pass.context, query_index) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderBeginOcclusionQuery",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: native::WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.draw(
        &pass.context,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(&pass.error_sink, cause, None, "wgpuRenderPassEncoderDraw"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexed(
    pass: native::WGPURenderPassEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.draw_indexed(
        &pass.context,
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderDrawIndexed",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.draw_indexed_indirect(&pass.context, indirect_buffer_id, indirect_offset) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderDrawIndexedIndirect",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.draw_indirect(&pass.context, indirect_buffer_id, indirect_offset) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderDrawIndexedIndirect",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass: native::WGPURenderPassEncoder) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.end(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(&pass.error_sink, cause, None, "wgpuRenderPassEncoderEnd"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEndOcclusionQuery(
    pass: native::WGPURenderPassEncoder,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.end_occlusion_query(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderEndOcclusionQuery",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderExecuteBundles(
    pass: native::WGPURenderPassEncoder,
    bundle_count: usize,
    bundles: *const native::WGPURenderBundle,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let bundle_ids = make_slice(bundles, bundle_count)
        .iter()
        .map(|v| v.as_ref().expect("invalid render bundle").id)
        .collect::<SmallVec<[_; 4]>>();
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.execute_bundles(&pass.context, &bundle_ids) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderExecuteBundles",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    pass: native::WGPURenderPassEncoder,
    marker_label: *const std::ffi::c_char,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.insert_debug_marker(
        &pass.context,
        CStr::from_ptr(marker_label).to_str().unwrap(),
        0,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderInsertDebugMarker",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPopDebugGroup(pass: native::WGPURenderPassEncoder) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.pop_debug_group(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderPopDebugGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    pass: native::WGPURenderPassEncoder,
    group_label: *const std::ffi::c_char,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.push_debug_group(
        &pass.context,
        CStr::from_ptr(group_label).to_str().unwrap(),
        0,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderPushDebugGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: native::WGPURenderPassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: usize,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    // TODO: as per webgpu.h bindgroup is nullable
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_bind_group(
        &pass.context,
        group_index,
        bind_group_id,
        make_slice(dynamic_offsets, dynamic_offset_count),
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetBindGroup",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: native::WGPURenderPassEncoder,
    color: Option<&native::WGPUColor>,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_blend_constant(
        &pass.context,
        conv::map_color(color.expect("invalid color")),
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetBlendConstant",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    index_format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_index_buffer(
        &pass.context,
        buffer_id,
        conv::map_index_format(index_format).expect("invalid index format"),
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetIndexBuffer",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: native::WGPURenderPassEncoder,
    render_pipeline: native::WGPURenderPipeline,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let render_pipeline_id = render_pipeline
        .as_ref()
        .expect("invalid render pipeline")
        .id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_pipeline(&pass.context, render_pipeline_id) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetPipeline",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    pass: native::WGPURenderPassEncoder,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_scissor_rect(&pass.context, x, y, width, height) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetPipeline",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    pass: native::WGPURenderPassEncoder,
    reference: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_stencil_reference(&pass.context, reference) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetStencilReference",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    pass: native::WGPURenderPassEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    // TODO: as per webgpu.h buffer is nullable
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_vertex_buffer(
        &pass.context,
        slot,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetVertexBuffer",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetViewport(
    pass: native::WGPURenderPassEncoder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_viewport(&pass.context, x, y, width, height, min_depth, max_depth) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetViewport",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderReference(
    render_pass_encoder: native::WGPURenderPassEncoder,
) {
    assert!(
        !render_pass_encoder.is_null(),
        "invalid render pass encoder"
    );
    Arc::increment_strong_count(render_pass_encoder);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderRelease(
    render_pass_encoder: native::WGPURenderPassEncoder,
) {
    assert!(
        !render_pass_encoder.is_null(),
        "invalid render pass encoder"
    );
    Arc::decrement_strong_count(render_pass_encoder);
}

// RenderPipeline methods

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    render_pipeline: native::WGPURenderPipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (render_pipeline_id, context, error_sink) = {
        let render_pipeline = render_pipeline.as_ref().expect("invalid render pipeline");
        (
            render_pipeline.id,
            &render_pipeline.context,
            &render_pipeline.error_sink,
        )
    };
    let (bind_group_layout_id, error) = gfx_select!(render_pipeline_id => context.render_pipeline_get_bind_group_layout(render_pipeline_id, group_index, None));
    if let Some(cause) = error {
        handle_error(
            error_sink,
            cause,
            None,
            "wgpuRenderPipelineGetBindGroupLayout",
        );
    }

    Arc::into_raw(Arc::new(WGPUBindGroupLayoutImpl {
        context: context.clone(),
        id: bind_group_layout_id,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineReference(render_pipeline: native::WGPURenderPipeline) {
    assert!(!render_pipeline.is_null(), "invalid render pipeline");
    Arc::increment_strong_count(render_pipeline);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineRelease(render_pipeline: native::WGPURenderPipeline) {
    assert!(!render_pipeline.is_null(), "invalid render pipeline");
    Arc::decrement_strong_count(render_pipeline);
}

// Sampler methods

#[no_mangle]
pub unsafe extern "C" fn wgpuSamplerReference(sampler: native::WGPUSampler) {
    assert!(!sampler.is_null(), "invalid sampler");
    Arc::increment_strong_count(sampler);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuSamplerRelease(sampler: native::WGPUSampler) {
    assert!(!sampler.is_null(), "invalid sampler");
    Arc::decrement_strong_count(sampler);
}

// ShaderModule methods

#[no_mangle]
pub unsafe extern "C" fn wgpuShaderModuleReference(shader_module: native::WGPUShaderModule) {
    assert!(!shader_module.is_null(), "invalid shader module");
    Arc::increment_strong_count(shader_module);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuShaderModuleRelease(shader_module: native::WGPUShaderModule) {
    assert!(!shader_module.is_null(), "invalid shader module");
    Arc::decrement_strong_count(shader_module);
}

// Surface methods

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceConfigure(
    surface: native::WGPUSurface,
    config: Option<&native::WGPUSurfaceConfiguration>,
) {
    let surface = surface.as_ref().expect("invalid surface");
    let config = config.expect("invalid config");
    let device = config
        .device
        .as_ref()
        .expect("invalid device for surface configuration");
    let context = &device.context;

    let surface_config = follow_chain!(map_surface_configuration(
        (config),
        WGPUSType_SurfaceConfigurationExtras => native::WGPUSurfaceConfigurationExtras
    ));

    match wgc::gfx_select!(device.id => context.surface_configure(surface.id, device.id, &surface_config))
    {
        Some(cause) => handle_error_fatal(cause, "wgpuSurfaceConfigure"),
        None => {
            let mut surface_data_guard = surface.data.lock();
            *surface_data_guard = Some(SurfaceData {
                device_id: device.id,
                error_sink: device.error_sink.clone(),
                texture_data: TextureData {
                    usage: config.usage,
                    dimension: native::WGPUTextureDimension_2D,
                    format: config.format,
                    mip_level_count: 1,
                    size: native::WGPUExtent3D {
                        width: config.width,
                        height: config.height,
                        depthOrArrayLayers: 1,
                    },
                    sample_count: 1,
                },
            });
            surface
                .has_surface_presented
                .store(false, atomic::Ordering::SeqCst);
        }
    };
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
        Err(cause) => handle_error_fatal(cause, "wgpuSurfaceGetCapabilities"),
    };

    capabilities.usages =
        conv::to_native_texture_usage_flags(caps.usages) as native::WGPUTextureUsageFlags;

    let formats = caps
        .formats
        .iter()
        // some texture formats are not in webgpu.h and
        // conv::to_native_texture_format returns None for them.
        // so, filter them out.
        .filter_map(|f| conv::to_native_texture_format(*f))
        .collect::<Vec<_>>();

    if !formats.is_empty() {
        let mut array = formats.into_boxed_slice();
        capabilities.formats = array.as_mut_ptr();
        capabilities.formatCount = array.len();
        mem::forget(array);
    } else {
        capabilities.formats = std::ptr::null_mut();
        capabilities.formatCount = 0;
    }

    let present_modes = caps
        .present_modes
        .iter()
        .filter_map(|f| conv::to_native_present_mode(*f))
        .collect::<Vec<_>>();

    if !present_modes.is_empty() {
        let mut array = present_modes.into_boxed_slice();
        capabilities.presentModes = array.as_mut_ptr();
        capabilities.presentModeCount = array.len();
        mem::forget(array);
    } else {
        capabilities.presentModes = std::ptr::null_mut();
        capabilities.presentModeCount = 0;
    }

    let alpha_modes = caps
        .alpha_modes
        .iter()
        .map(|f| conv::to_native_composite_alpha_mode(*f))
        .collect::<Vec<_>>();

    if !alpha_modes.is_empty() {
        let mut array = alpha_modes.into_boxed_slice();
        capabilities.alphaModes = array.as_mut_ptr();
        capabilities.alphaModeCount = array.len();
        mem::forget(array);
    } else {
        capabilities.alphaModes = std::ptr::null_mut();
        capabilities.alphaModeCount = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetCurrentTexture(
    surface: native::WGPUSurface,
    surface_texture: Option<&mut native::WGPUSurfaceTexture>,
) {
    let surface = surface.as_ref().expect("invalid surface");
    let context = &surface.context;
    let surface_texture = surface_texture.expect("invalid return pointer \"surface_texture\"");

    let surface_data_guard = surface.data.lock();
    let surface_data = match surface_data_guard.as_ref() {
        Some(surface_data) => surface_data,
        None => handle_error_fatal(
            wgc::present::SurfaceError::NotConfigured,
            "wgpuSurfaceGetCurrentTexture",
        ),
    };

    match wgc::gfx_select!(surface_data.device_id => context.surface_get_current_texture(surface.id, None))
    {
        Ok(wgc::present::SurfaceOutput { status, texture_id }) => {
            surface
                .has_surface_presented
                .store(false, atomic::Ordering::SeqCst);
            surface_texture.status = match status {
                wgt::SurfaceStatus::Good => native::WGPUSurfaceGetCurrentTextureStatus_Success,
                wgt::SurfaceStatus::Suboptimal => {
                    native::WGPUSurfaceGetCurrentTextureStatus_Success
                }
                wgt::SurfaceStatus::Timeout => native::WGPUSurfaceGetCurrentTextureStatus_Timeout,
                wgt::SurfaceStatus::Outdated => native::WGPUSurfaceGetCurrentTextureStatus_Outdated,
                wgt::SurfaceStatus::Lost => native::WGPUSurfaceGetCurrentTextureStatus_Lost,
            };
            surface_texture.suboptimal = match status {
                wgt::SurfaceStatus::Suboptimal => true as native::WGPUBool,
                _ => false as native::WGPUBool,
            };
            surface_texture.texture = match texture_id {
                Some(texture_id) => Arc::into_raw(Arc::new(WGPUTextureImpl {
                    context: context.clone(),
                    id: texture_id,
                    error_sink: surface_data.error_sink.clone(),
                    data: surface_data.texture_data,
                    surface_id: Some(surface.id),
                    has_surface_presented: surface.has_surface_presented.clone(),
                })),
                None => std::ptr::null_mut(),
            };
        }
        Err(cause) => handle_error_fatal(cause, "wgpuSurfaceGetCurrentTexture"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfacePresent(surface: native::WGPUSurface) {
    let surface = surface.as_ref().expect("invalid surface");
    let context = &surface.context;
    let surface_data_guard = surface.data.lock();
    let surface_data = match surface_data_guard.as_ref() {
        Some(surface_data) => surface_data,
        None => handle_error_fatal(
            wgc::present::SurfaceError::NotConfigured,
            "wgpuSurfacePresent",
        ),
    };

    match wgc::gfx_select!(surface_data.device_id => context.surface_present(surface.id)) {
        Ok(_status) => surface
            .has_surface_presented
            .store(true, atomic::Ordering::SeqCst),
        Err(cause) => handle_error_fatal(cause, "wgpuSurfacePresent"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceUnconfigure(surface: native::WGPUSurface) {
    let surface = surface.as_ref().expect("invalid surface");
    let mut surface_data_guard = surface.data.lock();
    let _ = surface_data_guard.take(); // drop SurfaceData
    surface
        .has_surface_presented
        .store(false, atomic::Ordering::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceReference(surface: native::WGPUSurface) {
    assert!(!surface.is_null(), "invalid surface");
    Arc::increment_strong_count(surface);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceRelease(surface: native::WGPUSurface) {
    assert!(!surface.is_null(), "invalid surface");
    Arc::decrement_strong_count(surface);
}

// SurfaceCapabilities methods

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceCapabilitiesFreeMembers(
    capabilities: native::WGPUSurfaceCapabilities,
) {
    if !capabilities.formats.is_null() && capabilities.formatCount > 0 {
        drop(Vec::from_raw_parts(
            capabilities.formats as *mut native::WGPUTextureFormat,
            capabilities.formatCount,
            capabilities.formatCount,
        ));
    }
    if !capabilities.presentModes.is_null() && capabilities.presentModeCount > 0 {
        drop(Vec::from_raw_parts(
            capabilities.presentModes as *mut native::WGPUPresentMode,
            capabilities.presentModeCount,
            capabilities.presentModeCount,
        ));
    }
    if !capabilities.alphaModes.is_null() && capabilities.alphaModeCount > 0 {
        drop(Vec::from_raw_parts(
            capabilities.alphaModes as *mut native::WGPUCompositeAlphaMode,
            capabilities.alphaModeCount,
            capabilities.alphaModeCount,
        ));
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
            label: ptr_into_label(descriptor.label),
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
        gfx_select!(texture_id => context.texture_create_view(texture_id, &desc, None));
    if let Some(cause) = error {
        handle_error(error_sink, cause, None, "wgpuTextureCreateView");
    }

    Arc::into_raw(Arc::new(WGPUTextureViewImpl {
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

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetDepthOrArrayLayers(texture: native::WGPUTexture) -> u32 {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.size.depthOrArrayLayers
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetDimension(
    texture: native::WGPUTexture,
) -> native::WGPUTextureDimension {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.dimension
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetFormat(
    texture: native::WGPUTexture,
) -> native::WGPUTextureFormat {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.format
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetHeight(texture: native::WGPUTexture) -> u32 {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.size.height
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetMipLevelCount(texture: native::WGPUTexture) -> u32 {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.mip_level_count
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetSampleCount(texture: native::WGPUTexture) -> u32 {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.sample_count
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetUsage(
    texture: native::WGPUTexture,
) -> native::WGPUTextureUsageFlags {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.usage
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetWidth(texture: native::WGPUTexture) -> u32 {
    let texture = texture.as_ref().expect("invalid texture");
    texture.data.size.width
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureReference(texture: native::WGPUTexture) {
    assert!(!texture.is_null(), "invalid texture");
    Arc::increment_strong_count(texture);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuTextureRelease(texture: native::WGPUTexture) {
    assert!(!texture.is_null(), "invalid texture");
    Arc::decrement_strong_count(texture);
}

// TextureView methods

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureViewReference(texture_view: native::WGPUTextureView) {
    assert!(!texture_view.is_null(), "invalid texture");
    Arc::increment_strong_count(texture_view);
}
#[no_mangle]
pub unsafe extern "C" fn wgpuTextureViewRelease(texture_view: native::WGPUTextureView) {
    assert!(!texture_view.is_null(), "invalid texture");
    Arc::decrement_strong_count(texture_view);
}

// wgpu.h functions

#[no_mangle]
pub unsafe extern "C" fn wgpuGenerateReport(
    instance: native::WGPUInstance,
    native_report: Option<&mut native::WGPUGlobalReport>,
) {
    let context = &instance.as_ref().expect("invalid instance").context;
    let native_report = native_report.expect("invalid return pointer \"native_report\"");
    conv::write_global_report(native_report, &context.generate_report());
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmitForIndex(
    queue: native::WGPUQueue,
    command_count: usize,
    commands: *const native::WGPUCommandBuffer,
) -> native::WGPUSubmissionIndex {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.queue.id, &queue.queue.context)
    };

    let command_buffers = make_slice(commands, command_count)
        .iter()
        .map(|command_buffer| {
            let command_buffer = command_buffer.as_ref().expect("invalid command buffer");
            command_buffer.open.store(false, atomic::Ordering::SeqCst);
            command_buffer.id
        })
        .collect::<SmallVec<[_; 4]>>();

    match gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers)) {
        Ok(submission_index) => submission_index.index,
        Err(cause) => handle_error_fatal(cause, "wgpuQueueSubmitForIndex"),
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
                        .queue
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
            handle_error_fatal(cause, "wgpuDevicePoll");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPushConstants(
    pass: native::WGPURenderPassEncoder,
    stages: native::WGPUShaderStageFlags,
    offset: u32,
    size_bytes: u32,
    data: *const u8,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.set_push_constants(
        &pass.context,
        wgt::ShaderStages::from_bits(stages).expect("invalid shader stage"),
        offset,
        make_slice(data, size_bytes as usize),
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderSetPushConstants",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.multi_draw_indirect(&pass.context, buffer_id, offset, count) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderMultiDrawIndirect",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.multi_draw_indexed_indirect(&pass.context, buffer_id, offset, count) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderMultiDrawIndexedIndirect",
        ),
    }
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
    let pass = pass.as_ref().expect("invalid render pass");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.multi_draw_indexed_indirect_count(
        &pass.context,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderMultiDrawIndirectCount",
        ),
    }
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
    let pass = pass.as_ref().expect("invalid render pass");
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.multi_draw_indexed_indirect_count(
        &pass.context,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    ) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderMultiDrawIndexedIndirectCount",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let query_set_id = query_set.as_ref().expect("invalid query set").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.begin_pipeline_statistics_query(&pass.context, query_set_id, query_index) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderBeginPipelineStatisticsQuery",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
) {
    let pass = pass.as_ref().expect("invalid compute pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.end_pipeline_statistics_query(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuComputePassEncoderEndPipelineStatisticsQuery",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let query_set_id = query_set.as_ref().expect("invalid query set").id;
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.begin_pipeline_statistics_query(&pass.context, query_set_id, query_index) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderBeginPipelineStatisticsQuery",
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
) {
    let pass = pass.as_ref().expect("invalid render pass");
    let encoder = pass.encoder.as_mut().unwrap();

    match encoder.end_pipeline_statistics_query(&pass.context) {
        Ok(()) => (),
        Err(cause) => handle_error(
            &pass.error_sink,
            cause,
            None,
            "wgpuRenderPassEncoderEndPipelineStatisticsQuery",
        ),
    }
}
