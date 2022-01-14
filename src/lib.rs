use log;
use std::{
    borrow::Cow, collections::HashMap, ffi::CString, marker::PhantomData, sync::Arc, sync::Mutex,
};
use wgc::id;

pub mod command;
pub mod conv;
pub mod device;
pub mod logging;

pub mod native {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

type Global = wgc::hub::Global<wgc::hub::IdentityManagerFactory>;

lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Global> = Arc::new(Global::new("wgpu", wgc::hub::IdentityManagerFactory, wgt::Backends::PRIMARY));
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
        self.0.map(|s| Cow::Owned(s))
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

#[cfg(target_os = "windows")]
pub type EnumConstant = i32;

#[cfg(not(target_os = "windows"))]
pub type EnumConstant = u32;

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
        pub fn $name(value: crate::EnumConstant) -> Result<$rs_type, crate::EnumConstant> {
            match value {
                $(paste::paste!(native::[<$c_name _ $variant>]) => Ok(<$rs_type>::$variant)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($variant:ident),+) => {
        pub fn $name(value: crate::EnumConstant) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($variant),+);

            map_fn(value).expect($err_msg)
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $($native_variant:ident:$variant2:ident),+) => {
        pub fn $name(value: crate::EnumConstant) -> Result<$rs_type, crate::EnumConstant> {
            match value {
                $(paste::paste!(native::[<$c_name _ $native_variant>]) => Ok(<$rs_type>::$variant2)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($native_variant:ident:$variant2:ident),+) => {
        pub fn $name(value: crate::EnumConstant) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($native_variant:$variant2),+);

            map_fn(value).expect($err_msg)
        }
    };
}

// see https://github.com/rust-windowing/raw-window-handle/issues/49
struct PseudoRwh(raw_window_handle::RawWindowHandle);
unsafe impl raw_window_handle::HasRawWindowHandle for PseudoRwh {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceCreateSurface(
    _: native::WGPUInstance,
    descriptor: *const native::WGPUSurfaceDescriptor,
) -> id::SurfaceId {
    follow_chain!(
        map_surface(descriptor.as_ref().unwrap(),
            WGPUSType_SurfaceDescriptorFromWindowsHWND => native::WGPUSurfaceDescriptorFromWindowsHWND,
            WGPUSType_SurfaceDescriptorFromXlib => native::WGPUSurfaceDescriptorFromXlib,
            WGPUSType_SurfaceDescriptorFromWaylandSurface => native::WGPUSurfaceDescriptorFromWaylandSurface,
            WGPUSType_SurfaceDescriptorFromMetalLayer => native::WGPUSurfaceDescriptorFromMetalLayer)
    )
}

pub fn wgpu_create_surface(raw_handle: raw_window_handle::RawWindowHandle) -> id::SurfaceId {
    GLOBAL.instance_create_surface(&PseudoRwh(raw_handle), PhantomData)
}

unsafe fn map_surface(
    _: &native::WGPUSurfaceDescriptor,
    _win: Option<&native::WGPUSurfaceDescriptorFromWindowsHWND>,
    _x11: Option<&native::WGPUSurfaceDescriptorFromXlib>,
    _wl: Option<&native::WGPUSurfaceDescriptorFromWaylandSurface>,
    _metal: Option<&native::WGPUSurfaceDescriptorFromMetalLayer>,
) -> id::SurfaceId {
    #[cfg(windows)]
    if let Some(win) = _win {
        let mut handle = raw_window_handle::Win32Handle::empty();
        handle.hwnd = win.hwnd;
        handle.hinstance = win.hinstance;
        return wgpu_create_surface(raw_window_handle::RawWindowHandle::Win32(handle));
    }

    #[cfg(all(
        unix,
        not(target_os = "android"),
        not(target_os = "ios"),
        not(target_os = "macos")
    ))]
    {
        if let Some(x11) = _x11 {
            let mut handle = raw_window_handle::XlibHandle::empty();
            handle.window = x11.window as _;
            handle.display = x11.display as *mut _;

            return wgpu_create_surface(raw_window_handle::RawWindowHandle::Xlib(handle));
        }

        if let Some(wl) = _wl {
            let mut handle = raw_window_handle::WaylandHandle::empty();
            handle.display = wl.display;
            handle.surface = wl.surface;

            return wgpu_create_surface(raw_window_handle::RawWindowHandle::Wayland(handle));
        }
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if let Some(metal) = _metal {
        return GLOBAL.instance_create_surface_metal(metal.layer, PhantomData);
    }

    panic!("Error: Unsupported Surface");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceGetPreferredFormat(
    surface: id::SurfaceId,
    adapter: id::AdapterId,
) -> native::WGPUTextureFormat {
    let preferred_format = match wgc::gfx_select!(adapter => GLOBAL.surface_get_preferred_format(surface, adapter))
    {
        Ok(format) => conv::to_native_texture_format(format),
        Err(err) => panic!("Could not get preferred swap chain format: {}", err),
    };
    return preferred_format;
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
    device: id::DeviceId,
    callback: native::WGPUErrorCallback,
    userdata: *mut std::os::raw::c_void,
) {
    CALLBACKS
        .lock()
        .unwrap()
        .uncaptured_errors
        .insert(device, UncapturedErrorCallback { callback, userdata });
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceSetDeviceLostCallback(
    device: id::DeviceId,
    callback: native::WGPUDeviceLostCallback,
    userdata: *mut std::os::raw::c_void,
) {
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
        Some(device_error) => match device_error {
            wgc::device::DeviceError::Lost => native::WGPUErrorType_DeviceLost,
            _ => native::WGPUErrorType_Unknown,
        },
        None => native::WGPUErrorType_Unknown,
    };

    handle_device_error_raw(device, typ, &format!("{:?}", error));
}
