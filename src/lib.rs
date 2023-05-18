use crate::conv::map_instance_descriptor;
use std::{collections::HashMap, ffi::CString, sync::Arc, sync::Mutex};
use wgc::id;

pub mod command;
pub mod conv;
pub mod device;
pub mod logging;
pub mod unimplemented;
pub mod utils;

pub type Context = wgc::hub::Global<wgc::hub::IdentityManagerFactory>;

pub mod native {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    use crate::Context;
    use std::sync::Arc;
    use wgc::{
        command::{ComputePass, RenderBundleEncoder, RenderPass},
        id::{
            AdapterId, BindGroupId, BindGroupLayoutId, BufferId, CommandBufferId, CommandEncoderId,
            ComputePipelineId, DeviceId, PipelineLayoutId, QuerySetId, QueueId, RenderBundleId,
            RenderPipelineId, SamplerId, ShaderModuleId, StagingBufferId, SurfaceId, TextureId,
            TextureViewId,
        },
    };

    pub struct WGPUContextHandle<I: wgc::id::TypedId> {
        pub context: Arc<Context>,
        pub id: I,
    }

    pub type WGPUDeviceImpl = WGPUContextHandle<DeviceId>;
    pub type WGPUQueueImpl = WGPUContextHandle<QueueId>;
    pub type WGPUPipelineLayoutImpl = WGPUContextHandle<PipelineLayoutId>;
    pub type WGPUShaderModuleImpl = WGPUContextHandle<ShaderModuleId>;
    pub type WGPUBindGroupLayoutImpl = WGPUContextHandle<BindGroupLayoutId>;
    pub type WGPUBindGroupImpl = WGPUContextHandle<BindGroupId>;
    pub type WGPUCommandBufferImpl = WGPUContextHandle<CommandBufferId>;
    pub type WGPUCommandEncoderImpl = WGPUContextHandle<CommandEncoderId>;
    pub type WGPURenderBundleImpl = WGPUContextHandle<RenderBundleId>;
    pub type WGPURenderPipelineImpl = WGPUContextHandle<RenderPipelineId>;
    pub type WGPUComputePipelineImpl = WGPUContextHandle<ComputePipelineId>;
    pub type WGPUQuerySetImpl = WGPUContextHandle<QuerySetId>;
    pub type WGPUBufferImpl = WGPUContextHandle<BufferId>;
    pub type WGPUStagingBufferImpl = WGPUContextHandle<StagingBufferId>;
    pub type WGPUTextureImpl = WGPUContextHandle<TextureId>;
    pub type WGPUTextureViewImpl = WGPUContextHandle<TextureViewId>;
    pub type WGPUSamplerImpl = WGPUContextHandle<SamplerId>;
    pub type WGPUSurfaceImpl = WGPUContextHandle<SurfaceId>;

    pub struct WGPUInstanceImpl {
        pub context: Arc<Context>,
    }

    pub struct WGPUAdapterImpl {
        pub context: Arc<Context>,
        pub id: AdapterId,
        pub name: std::ffi::CString,
        pub vendor_name: std::ffi::CString,
        pub architecture_name: std::ffi::CString,
        pub driver_desc: std::ffi::CString,
    }

    pub struct WGPUSwapChainImpl {
        pub context: Arc<Context>,
        pub surface_id: SurfaceId,
        pub device_id: DeviceId,
    }

    pub struct WGPURenderPassEncoderImpl {
        pub context: Arc<Context>,
        pub encoder: RenderPass,
    }

    pub struct WGPUComputePassEncoderImpl {
        pub context: Arc<Context>,
        pub encoder: ComputePass,
    }

    pub struct WGPURenderBundleEncoderImpl {
        pub context: Arc<Context>,
        pub encoder: RenderBundleEncoder,
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCreateInstance(
    descriptor: Option<&native::WGPUInstanceDescriptor>,
) -> native::WGPUInstance {
    let descriptor = descriptor.expect("invalid descriptor");

    let instance_desc = follow_chain!(map_instance_descriptor(descriptor,
        WGPUSType_InstanceExtras => native::WGPUInstanceExtras
    ));

    Box::into_raw(Box::new(native::WGPUInstanceImpl {
        context: Arc::new(Context::new(
            "wgpu",
            wgc::hub::IdentityManagerFactory,
            instance_desc,
        )),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceDrop(instance: native::WGPUInstance) {
    assert!(!instance.is_null(), "invalid instance");
    drop(Box::from_raw(instance));
}

enum CreateSurfaceParams {
    Raw(
        (
            raw_window_handle::RawDisplayHandle,
            raw_window_handle::RawWindowHandle,
        ),
    ),
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    Metal(*mut std::ffi::c_void),
}

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

    Box::into_raw(Box::new(native::WGPUSurfaceImpl {
        context: context.clone(),
        id: surface_id,
    }))
}

unsafe fn map_surface(
    _: &native::WGPUSurfaceDescriptor,
    _win: Option<&native::WGPUSurfaceDescriptorFromWindowsHWND>,
    _xcb: Option<&native::WGPUSurfaceDescriptorFromXcbWindow>,
    _xlib: Option<&native::WGPUSurfaceDescriptorFromXlibWindow>,
    _wl: Option<&native::WGPUSurfaceDescriptorFromWaylandSurface>,
    _metal: Option<&native::WGPUSurfaceDescriptorFromMetalLayer>,
    _android: Option<&native::WGPUSurfaceDescriptorFromAndroidNativeWindow>,
) -> CreateSurfaceParams {
    #[cfg(windows)]
    if let Some(win) = _win {
        let display_handle = raw_window_handle::WindowsDisplayHandle::empty();
        let mut window_handle = raw_window_handle::Win32WindowHandle::empty();
        window_handle.hwnd = win.hwnd;
        window_handle.hinstance = win.hinstance;

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Windows(display_handle),
            raw_window_handle::RawWindowHandle::Win32(window_handle),
        ));
    }

    #[cfg(all(
        unix,
        not(target_os = "android"),
        not(target_os = "ios"),
        not(target_os = "macos")
    ))]
    {
        if let Some(xcb) = _xcb {
            let mut display_handle = raw_window_handle::XcbDisplayHandle::empty();
            display_handle.connection = xcb.connection;
            let mut window_handle = raw_window_handle::XcbWindowHandle::empty();
            window_handle.window = xcb.window;

            return CreateSurfaceParams::Raw((
                raw_window_handle::RawDisplayHandle::Xcb(display_handle),
                raw_window_handle::RawWindowHandle::Xcb(window_handle),
            ));
        }

        if let Some(xlib) = _xlib {
            let mut display_handle = raw_window_handle::XlibDisplayHandle::empty();
            display_handle.display = xlib.display;
            let mut window_handle = raw_window_handle::XlibWindowHandle::empty();
            window_handle.window = xlib.window as _;

            return CreateSurfaceParams::Raw((
                raw_window_handle::RawDisplayHandle::Xlib(display_handle),
                raw_window_handle::RawWindowHandle::Xlib(window_handle),
            ));
        }

        if let Some(wl) = _wl {
            let mut display_handle = raw_window_handle::WaylandDisplayHandle::empty();
            display_handle.display = wl.display;
            let mut window_handle = raw_window_handle::WaylandWindowHandle::empty();
            window_handle.surface = wl.surface;

            return CreateSurfaceParams::Raw((
                raw_window_handle::RawDisplayHandle::Wayland(display_handle),
                raw_window_handle::RawWindowHandle::Wayland(window_handle),
            ));
        }
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if let Some(metal) = _metal {
        return CreateSurfaceParams::Metal(metal.layer);
    }

    #[cfg(target_os = "android")]
    if let Some(android) = _android {
        let display_handle = raw_window_handle::AndroidDisplayHandle::empty();
        let mut window_handle = raw_window_handle::AndroidNdkWindowHandle::empty();
        window_handle.a_native_window = android.window;

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Android(display_handle),
            raw_window_handle::RawWindowHandle::AndroidNdk(window_handle),
        ));
    }

    panic!("Error: Unsupported Surface");
}

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

    let preferred_format = match wgc::gfx_select!(adapter_id => context.surface_get_capabilities(surface_id, adapter_id))
    {
        Ok(caps) => conv::to_native_texture_format(
            *caps
                .formats
                .first() // first format in the vector is preferred
                .expect("Could not get preferred swap chain format"),
        )
        .expect("Could not get preferred swap chain format"),
        Err(err) => panic!("Could not get preferred swap chain format: {err:?}"),
    };

    preferred_format
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

    let caps =
        wgc::gfx_select!(adapter_id => context.surface_get_capabilities(surface_id, adapter_id))
            .expect("failed to get surface capabilities");

    let formats = caps
        .formats
        .iter()
        // some texture formats are not in webgpu.h and
        // conv::to_native_texture_format returns None for them.
        // so, filter them out.
        .filter_map(|f| conv::to_native_texture_format(*f))
        .collect::<Vec<native::WGPUTextureFormat>>();

    capabilities.formatCount = formats.len();

    if !capabilities.formats.is_null() {
        std::ptr::copy_nonoverlapping(formats.as_ptr(), capabilities.formats, formats.len());
    }

    let present_modes = caps
        .present_modes
        .iter()
        .filter_map(|f| conv::to_native_present_mode(*f))
        .collect::<Vec<native::WGPUPresentMode>>();

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
        .collect::<Vec<native::WGPUCompositeAlphaMode>>();

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
pub unsafe extern "C" fn wgpuGenerateReport(
    instance: native::WGPUInstance,
    native_report: Option<&mut native::WGPUGlobalReport>,
) {
    let context = &instance.as_ref().expect("invalid instance").context;
    let native_report = native_report.expect("invalid return pointer \"native_report\"");
    conv::write_global_report(native_report, context.generate_report());
}

struct DeviceCallback<T> {
    callback: T,
    userdata: *mut std::os::raw::c_void,
}
unsafe impl<T> Send for DeviceCallback<T> {}

type UncapturedErrorCallback = DeviceCallback<native::WGPUErrorCallback>;
type DeviceLostCallback = DeviceCallback<native::WGPUDeviceLostCallback>;

struct Callbacks {
    uncaptured_error_cbs: Option<HashMap<id::DeviceId, UncapturedErrorCallback>>,
    device_lost_cbs: Option<HashMap<id::DeviceId, DeviceLostCallback>>,
}

static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    uncaptured_error_cbs: None,
    device_lost_cbs: None,
});

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetUncapturedErrorCallback(
    device: native::WGPUDevice,
    callback: native::WGPUErrorCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let device_id = device.as_ref().expect("invalid device").id;

    let mut callbacks = CALLBACKS.lock().unwrap();
    if callbacks.uncaptured_error_cbs.is_none() {
        callbacks.uncaptured_error_cbs = Some(HashMap::new());
    }

    callbacks
        .uncaptured_error_cbs
        .as_mut()
        .and_then(|v| v.insert(device_id, UncapturedErrorCallback { callback, userdata }));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetDeviceLostCallback(
    device: native::WGPUDevice,
    callback: native::WGPUDeviceLostCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let device_id = device.as_ref().expect("invalid device").id;

    let mut callbacks = CALLBACKS.lock().unwrap();
    if callbacks.device_lost_cbs.is_none() {
        callbacks.device_lost_cbs = Some(HashMap::new());
    }

    callbacks
        .device_lost_cbs
        .as_mut()
        .and_then(|v| v.insert(device_id, DeviceLostCallback { callback, userdata }));
}

#[inline]
pub fn handle_device_error<E: std::any::Any + std::error::Error>(
    device_id: id::DeviceId,
    error: &E,
) {
    let error_any = error as &dyn std::any::Any;

    match error_any.downcast_ref::<wgc::device::DeviceError>() {
        Some(wgc::device::DeviceError::Lost) => {
            let callbacks = CALLBACKS.lock().unwrap();

            if let Some(device_lost_cbs) = &callbacks.device_lost_cbs {
                if let Some(device_lost_cb) = device_lost_cbs.get(&device_id) {
                    if let Some(callback) = device_lost_cb.callback {
                        let msg_c = CString::new(error.to_string()).unwrap();
                        unsafe {
                            callback(
                                native::WGPUDeviceLostReason_Destroyed,
                                msg_c.as_ptr(),
                                device_lost_cb.userdata,
                            );
                        }
                    }
                }
            }
        }
        _ => {
            let callbacks = CALLBACKS.lock().unwrap();

            if let Some(uncaptured_error_cbs) = &callbacks.uncaptured_error_cbs {
                if let Some(uncaptured_error_cb) = uncaptured_error_cbs.get(&device_id) {
                    if let Some(callback) = uncaptured_error_cb.callback {
                        let msg_c = CString::new(error.to_string()).unwrap();
                        unsafe {
                            callback(
                                native::WGPUErrorType_Unknown,
                                msg_c.as_ptr(),
                                uncaptured_error_cb.userdata,
                            );
                        }
                    }
                }
            }
        }
    };
}
