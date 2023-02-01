use crate::conv::map_instance_descriptor;
use native::{Handle, IntoHandle, IntoHandleWithContext, UnwrapId};
use std::{borrow::Cow, collections::HashMap, ffi::CString, sync::Arc, sync::Mutex};
use wgc::id;

pub mod command;
pub mod conv;
pub mod device;
pub mod logging;

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

    type WGPUDeviceImpl = WGPUContextHandle<DeviceId>;
    type WGPUQueueImpl = WGPUContextHandle<QueueId>;
    type WGPUPipelineLayoutImpl = WGPUContextHandle<PipelineLayoutId>;
    type WGPUShaderModuleImpl = WGPUContextHandle<ShaderModuleId>;
    type WGPUBindGroupLayoutImpl = WGPUContextHandle<BindGroupLayoutId>;
    type WGPUBindGroupImpl = WGPUContextHandle<BindGroupId>;
    type WGPUCommandBufferImpl = WGPUContextHandle<CommandBufferId>;
    type WGPUCommandEncoderImpl = WGPUContextHandle<CommandEncoderId>;
    type WGPURenderBundleImpl = WGPUContextHandle<RenderBundleId>;
    type WGPURenderPipelineImpl = WGPUContextHandle<RenderPipelineId>;
    type WGPUComputePipelineImpl = WGPUContextHandle<ComputePipelineId>;
    type WGPUQuerySetImpl = WGPUContextHandle<QuerySetId>;
    type WGPUBufferImpl = WGPUContextHandle<BufferId>;
    type WGPUStagingBufferImpl = WGPUContextHandle<StagingBufferId>;
    type WGPUTextureImpl = WGPUContextHandle<TextureId>;
    type WGPUTextureViewImpl = WGPUContextHandle<TextureViewId>;
    type WGPUSamplerImpl = WGPUContextHandle<SamplerId>;
    type WGPUSurfaceImpl = WGPUContextHandle<SurfaceId>;

    pub struct WGPUInstanceImpl {
        pub context: Arc<Context>,
    }

    pub struct WGPUAdapterImpl {
        pub context: Arc<Context>,
        pub id: AdapterId,
        pub name: std::ffi::CString,
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

    /// Convenience trait for converting handle structs into boxed pointer
    ///
    /// this is equivalent to calling `Box::into_raw(Box::new(Struct{...}))`
    pub trait IntoHandle {
        fn into_handle(self) -> *mut Self;
    }

    /// Convenience trait implementing drop for handles
    ///
    /// this is equivalent to calling `drop(Box::from_raw(ptr))`
    pub trait Handle {
        unsafe fn drop(self);
    }

    /// Convenience trait for converting handle pointers to Ids
    pub trait UnwrapId<I: wgc::id::TypedId> {
        unsafe fn unwrap_handle(&self) -> (I, &Arc<Context>);
        fn as_option(&self) -> Option<I>;
    }

    /// Convenience trait for converting ids into handle pointers
    pub trait IntoHandleWithContext<H> {
        fn into_handle_with_context(self, context: &Arc<Context>) -> *mut H;
    }

    /// This macro implements the IntoHandle & Handle for the struct and it's *mut type
    ///
    /// * `Struct{}.into_handle()` will return a boxed pointer to the struct.
    /// * the box can be dropped by calling `ptr.drop()`
    macro_rules! implement_handle {
        ($impl_type:ty) => {
            impl $crate::native::IntoHandle for $impl_type {
                fn into_handle(self) -> *mut Self {
                    Box::into_raw(Box::new(self))
                }
            }
            impl $crate::native::Handle for *mut $impl_type {
                unsafe fn drop(self) {
                    drop(Box::from_raw(self))
                }
            }
        };
    }

    /// This macro implements the UnwrapId for the *mut type
    ///
    /// * `ptr.unwrap_handle()` will return an `(Id, &Arc<Context>)` or panic on invalid pointers.
    /// * `ptr.as_option()` will return `Option<Id>` based on if the pointer is null or not
    macro_rules! implement_unwrap_handle {
        ($impl_type:ty, $id_type:ty) => {
            impl $crate::native::UnwrapId<$id_type> for *mut $impl_type {
                unsafe fn unwrap_handle(&self) -> ($id_type, &Arc<Context>) {
                    unsafe {
                        let v = self.as_ref().expect(stringify!(invalid $id_type));
                        (v.id, &v.context)
                    }
                }
                fn as_option(&self) -> Option<$id_type> {
                    unsafe { self.as_ref().map(|v| v.id)}
                }
            }
        };
    }

    /// implements id.into_handle_with_context(&context)
    ///
    /// For example, when creating a new WGPUDevice:
    ///
    /// ```ignore
    /// device_id.into_handle_with_context(&context)
    /// ```
    ///
    /// is equivalent to:
    ///
    /// ```ignore
    ///   native::WGPUDeviceImpl{
    ///       context: context.clone(),
    ///       id: id,
    ///   }.into_handle()
    /// ```
    macro_rules! implement_into_handle_with_context {
        ($impl_type:path, $id_type:ty) => {
            impl $crate::native::IntoHandleWithContext<$impl_type> for $id_type {
                fn into_handle_with_context(
                    self,
                    context: &$crate::Arc<$crate::Context>,
                ) -> *mut $impl_type {
                    $impl_type {
                        context: context.clone(),
                        id: self,
                    }
                    .into_handle()
                }
            }
        };
    }

    implement_handle!(WGPUInstanceImpl);
    implement_handle!(WGPUAdapterImpl);
    implement_unwrap_handle!(WGPUAdapterImpl, AdapterId);
    implement_handle!(WGPURenderPassEncoderImpl);
    implement_handle!(WGPUComputePassEncoderImpl);
    implement_handle!(WGPURenderBundleEncoderImpl);
    implement_handle!(WGPUSwapChainImpl);

    pub unsafe fn unwrap_swap_chain_handle<'a>(
        handle: *mut WGPUSwapChainImpl,
    ) -> (SurfaceId, DeviceId, &'a Arc<Context>) {
        unsafe {
            let v = handle.as_ref().expect("invalid swap chain");
            (v.surface_id, v.device_id, &v.context)
        }
    }

    /// Shorthand for handles that are just Id & Context
    macro_rules! implement_id_handle {
        ($impl_type:ty, $id_type:ty) => {
            implement_handle!($impl_type);
            implement_unwrap_handle!($impl_type, $id_type);
            implement_into_handle_with_context!($impl_type, $id_type);
        };
    }

    implement_id_handle!(WGPUDeviceImpl, DeviceId);
    implement_id_handle!(WGPUPipelineLayoutImpl, PipelineLayoutId);
    implement_id_handle!(WGPUShaderModuleImpl, ShaderModuleId);
    implement_id_handle!(WGPUBindGroupLayoutImpl, BindGroupLayoutId);
    implement_id_handle!(WGPUBindGroupImpl, BindGroupId);
    implement_id_handle!(WGPUCommandBufferImpl, CommandBufferId);
    implement_id_handle!(WGPURenderBundleImpl, RenderBundleId);
    implement_id_handle!(WGPURenderPipelineImpl, RenderPipelineId);
    implement_id_handle!(WGPUComputePipelineImpl, ComputePipelineId);
    implement_id_handle!(WGPUQuerySetImpl, QuerySetId);
    implement_id_handle!(WGPUBufferImpl, BufferId);
    implement_id_handle!(WGPUStagingBufferImpl, StagingBufferId);
    implement_id_handle!(WGPUTextureImpl, TextureId);
    implement_id_handle!(WGPUTextureViewImpl, TextureViewId);
    implement_id_handle!(WGPUSamplerImpl, SamplerId);
    implement_id_handle!(WGPUSurfaceImpl, SurfaceId);

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub type Label<'a> = Option<Cow<'a, str>>;

struct OwnedLabel(Option<String>);
impl OwnedLabel {
    fn new(ptr: *const std::os::raw::c_char) -> Self {
        Self(if ptr.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(ptr) }
                    .to_string_lossy()
                    .to_string(),
            )
        })
    }
    fn into_inner(self) -> Option<String> {
        self.0
    }
    fn as_cow(&self) -> Option<Cow<str>> {
        self.0.as_ref().map(|s| Cow::Borrowed(s.as_str()))
    }
    fn into_cow<'a>(self) -> Option<Cow<'a, str>> {
        self.0.map(Cow::Owned)
    }
}

pub unsafe fn make_slice<'a, T: 'a>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(pointer, count)
    }
}

/// Follow a chain of next pointers and automatically resolve them to the underlying structs.
///
/// # Syntax:
///
/// Given:
///
/// `fn map_thing_descriptor(base: &ThingDescriptor, ext1: Option<&ThingDescriptorExtension1>) -> wgt::ThingDescriptor`
///
/// Use the syntax:
///
/// `follow_chain!(map_thing_descriptor(base_c_descriptor, ThingDescriptorExtension1STypeValue => ThingDescriptorExtension1))`
///
/// # Safety
///
/// This macro does not use any internal unsafe blocks. The caller (or most likely the function) needs
/// to be unsafe. The following constraints must be upheld for this to be valid:
///
/// - All pointers in the chain of next pointers must point to either null or a valid extension object
/// - All structures used as extension objects must be `#[repr(C)]`.
/// - All structures used as extension objects must have `pub next_in_chain: Option<&ChainedStruct>` and `pub s_type: SType`
///   as the first and second members respectively.
///
/// The result of these rules, and the fact that wgpu-native functions using it do not validate all these assumptions,
/// using this macro is an indication that the function itself must be made unsafe.
///
/// # Notes
///
/// Given two or more extension structs of the same SType in the same chain, this macro will favor the latter most. There should
/// not be more than one extension struct with the same SType in a chain anyway, so this behavior should be unproblematic.

#[macro_export]
macro_rules! follow_chain {
    ($func:ident($base:expr $(, $stype:ident => $ty:ty)*)) => {{
    #[allow(non_snake_case)] // We use the type name as an easily usable temporary name
    {
        $(
            let mut $stype: Option<&$ty> = None;
        )*
        let mut chain_opt: Option<&$crate::native::WGPUChainedStruct> = $base.nextInChain.as_ref();
        while let Some(next_in_chain) = chain_opt {
            match next_in_chain.sType {
                $(
                    $crate::native::$stype => {
                        let next_in_chain_ptr = next_in_chain as *const $crate::native::WGPUChainedStruct;
                        assert_eq!(
                            0,
                            next_in_chain_ptr.align_offset(::std::mem::align_of::<$ty>()),
                            concat!("Chain structure pointer is not aligned correctly to dereference as ", stringify!($ty), ". Correct alignment: {}"),
                            ::std::mem::align_of::<$ty>()
                        );
                        let type_ptr: *const $ty = next_in_chain_ptr as _;
                        $stype = Some(&*type_ptr);
                    }
                )*
                _ => {}
            }
            chain_opt = next_in_chain.next.as_ref();
        }
        $func($base, $($stype),*)
    }}};
}

/// Creates a function which maps native constants to wgpu enums.
/// If an error message is provided, the function will panic if the
/// input does not match any known variants. Otherwise a Result<T, i32> is returned
///
/// # Syntax
///
/// For enums that have undefined variants:
/// ```ignore
/// map_enum!(function_name, header_prefix, rust_type, Variant1, Variant2...)
/// ```
///
/// For enums where all variants are defined:
/// ```ignore
/// map_enum!(function_name, header_prefix, rust_type, err_msg, Variant1, Variant2...)
/// ```
///
/// # Example
///
/// For the following enum:
/// ```c
/// typedef enum WGPUIndexFormat {
///     WGPUIndexFormat_Undefined = 0x00000000,
///     WGPUIndexFormat_Uint16 = 0x00000001,
///     WGPUIndexFormat_Uint32 = 0x00000002,
///     WGPUIndexFormat_Force32 = 0x7FFFFFFF
/// } WGPUIndexFormat;
/// ```
/// Then you can use the following macro:
/// ```ignore
/// map_enum!(map_index_format, WGPUIndexFormat, wgt::IndexFormat, Uint16, Uint32);
/// ```
/// Which expands into:
/// ```ignore
/// pub fn map_index_format(value: i32) -> Result<wgt::IndexFormat, i32> {
///      match value {
///          native::WGPUIndexFormat_Uint16 => Ok(wgt::IndexFormat::Uint16),
///          native::WGPUIndexFormat_Uint32 => Ok(wgt::IndexFormat::Uint32),
///          x => Err(x),
///      }
/// }
/// ```
///
#[macro_export]
macro_rules! map_enum {
    ($name:ident, $c_name:ident, $rs_type:ty, $($variant:ident),+) => {
        pub fn $name(value: native::$c_name) -> Result<$rs_type, native::$c_name> {
            match value {
                $(paste::paste!(native::[<$c_name _ $variant>]) => Ok(<$rs_type>::$variant)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($variant:ident),+) => {
        pub fn $name(value: native::$c_name) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($variant),+);

            map_fn(value).expect($err_msg)
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $($native_variant:ident:$variant2:ident),+) => {
        pub fn $name(value: native::$c_name) -> Result<$rs_type, native::$c_name> {
            match value {
                $(paste::paste!(native::[<$c_name _ $native_variant>]) => Ok(<$rs_type>::$variant2)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($native_variant:ident:$variant2:ident),+) => {
        pub fn $name(value: native::$c_name) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($native_variant:$variant2),+);

            map_fn(value).expect($err_msg)
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCreateInstance(
    descriptor: Option<&native::WGPUInstanceDescriptor>,
) -> native::WGPUInstance {
    let descriptor = descriptor.expect("invalid descriptor");

    let instance_desc = follow_chain!(map_instance_descriptor(descriptor,
        WGPUSType_InstanceExtras => native::WGPUInstanceExtras
    ));

    native::WGPUInstanceImpl {
        context: Arc::new(Context::new(
            "wgpu",
            wgc::hub::IdentityManagerFactory,
            instance_desc,
        )),
    }
    .into_handle()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceDrop(instance: native::WGPUInstance) {
    instance.drop();
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

    let context = &instance.as_ref().expect("invalid instance").context;
    let surface_id = match create_surface_params {
        CreateSurfaceParams::Raw((rdh, rwh)) => context.instance_create_surface(rdh, rwh, ()),
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        CreateSurfaceParams::Metal(layer) => context.instance_create_surface_metal(layer, ()),
    };

    surface_id.into_handle_with_context(context)
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
    let (adapter, context) = adapter.unwrap_handle();
    let (surface, _) = unsafe {
        let v = surface.as_ref().expect("invalid surface");
        (v.id, &v.context)
    };

    let preferred_format = match wgc::gfx_select!(adapter => context.surface_get_capabilities(surface, adapter))
    {
        Ok(caps) => conv::to_native_texture_format(
            *caps
                .formats
                .first() // first format in the vector is preferred
                .expect("Could not get preferred swap chain format"),
        )
        .expect("Could not get preferred swap chain format"),
        Err(err) => panic!("Could not get preferred swap chain format: {}", err),
    };

    preferred_format
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetSupportedFormats(
    surface: native::WGPUSurface,
    adapter: native::WGPUAdapter,
    count: Option<&mut usize>,
) -> *const native::WGPUTextureFormat {
    let (surface, context) = unsafe {
        let v = surface.as_ref().expect("invalid surface");
        (v.id, &v.context)
    };

    let (adapter, _) = adapter.unwrap_handle();
    assert!(count.is_some(), "count must be non-null");

    let mut native_formats = match wgc::gfx_select!(adapter => context.surface_get_capabilities(surface, adapter))
    {
        Ok(caps) => caps
            .formats
            .iter()
            // some texture formats are not in webgpu.h and
            // conv::to_native_texture_format return None for them.
            // so, filter them out.
            .filter_map(|f| conv::to_native_texture_format(*f))
            .collect::<Vec<native::WGPUTextureFormat>>(),
        Err(err) => panic!("Could not get supported swap chain formats: {}", err),
    };
    native_formats.shrink_to_fit();

    if let Some(count) = count {
        *count = native_formats.len();
    }
    let ptr = native_formats.as_ptr();
    std::mem::forget(native_formats);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetSupportedPresentModes(
    surface: native::WGPUSurface,
    adapter: native::WGPUAdapter,
    count: Option<&mut usize>,
) -> *const native::WGPUPresentMode {
    let (surface, _) = surface.unwrap_handle();
    let (adapter, context) = adapter.unwrap_handle();
    assert!(count.is_some(), "count must be non-null");

    let mut modes = match wgc::gfx_select!(adapter => context.surface_get_capabilities(surface, adapter))
    {
        Ok(caps) => caps
            .present_modes
            .iter()
            .filter_map(|f| match *f {
                wgt::PresentMode::Fifo => Some(native::WGPUPresentMode_Fifo),
                wgt::PresentMode::Immediate => Some(native::WGPUPresentMode_Immediate),
                wgt::PresentMode::Mailbox => Some(native::WGPUPresentMode_Mailbox),

                wgt::PresentMode::AutoVsync
                | wgt::PresentMode::AutoNoVsync
                | wgt::PresentMode::FifoRelaxed => None, // needs to be supported in webgpu.h
            })
            .collect::<Vec<native::WGPUPresentMode>>(),
        Err(err) => panic!("Could not get supported present modes: {}", err),
    };
    modes.shrink_to_fit();

    if let Some(count) = count {
        *count = modes.len();
    }
    let ptr = modes.as_ptr();
    std::mem::forget(modes);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn wgpuGenerateReport(
    instance: native::WGPUInstance,
    native_report: Option<&mut native::WGPUGlobalReport>,
) {
    let context = &instance.as_ref().expect("invalid instance").context;
    conv::write_global_report(
        native_report.expect("invalid return pointer"),
        context.generate_report(),
    );
}

struct DeviceCallback<T> {
    callback: T,
    userdata: *mut std::os::raw::c_void,
}

type UncapturedErrorCallback = DeviceCallback<native::WGPUErrorCallback>;
type DeviceLostCallback = DeviceCallback<native::WGPUDeviceLostCallback>;

unsafe impl<T> Send for DeviceCallback<T> {}

struct Callbacks {
    uncaptured_errors: HashMap<id::DeviceId, UncapturedErrorCallback>,
    device_lost: HashMap<id::DeviceId, DeviceLostCallback>,
}

lazy_static::lazy_static! {
    static ref CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks{
        uncaptured_errors: HashMap::new(),
        device_lost: HashMap::new(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetUncapturedErrorCallback(
    device: native::WGPUDevice,
    callback: native::WGPUErrorCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (device, _) = device.unwrap_handle();

    CALLBACKS
        .lock()
        .unwrap()
        .uncaptured_errors
        .insert(device, UncapturedErrorCallback { callback, userdata });
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetDeviceLostCallback(
    device: native::WGPUDevice,
    callback: native::WGPUDeviceLostCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (device, _) = device.unwrap_handle();

    CALLBACKS
        .lock()
        .unwrap()
        .device_lost
        .insert(device, DeviceLostCallback { callback, userdata });
}

pub fn handle_device_error_raw(device: id::DeviceId, typ: native::WGPUErrorType, msg: &str) {
    log::debug!("Device error ({}): {}", typ, msg);
    let msg_c = CString::new(msg).unwrap();
    unsafe {
        match typ {
            native::WGPUErrorType_DeviceLost => {
                let cbs = CALLBACKS.lock().unwrap();
                let cb = cbs.device_lost.get(&device);
                if let Some(cb) = cb {
                    (*cb).callback.unwrap()(
                        native::WGPUDeviceLostReason_Destroyed,
                        msg_c.as_ptr(),
                        (*cb).userdata,
                    );
                }
            }
            _ => {
                let cbs = CALLBACKS.lock().unwrap();
                let cb = cbs.uncaptured_errors.get(&device);
                if let Some(cb) = cb {
                    (*cb).callback.unwrap()(typ, msg_c.as_ptr(), (*cb).userdata);
                }
            }
        }
    }
}

pub fn handle_device_error<E: std::any::Any + std::error::Error>(device: id::DeviceId, error: &E) {
    let error_any = error as &dyn std::any::Any;

    let typ = match error_any.downcast_ref::<wgc::device::DeviceError>() {
        Some(wgc::device::DeviceError::Lost) => native::WGPUErrorType_DeviceLost,
        _ => native::WGPUErrorType_Unknown,
    };

    handle_device_error_raw(device, typ, &format!("{:?}", error));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuFree(ptr: *mut u8, size: usize, align: usize) {
    std::alloc::dealloc(
        ptr,
        core::alloc::Layout::from_size_align(size, align).unwrap(),
    );
}
