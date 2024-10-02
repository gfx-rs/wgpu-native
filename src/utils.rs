use std::{borrow::Cow, ffi::CStr};

use crate::native;

// A dummy wrapper that is `Send` + `Sync` to store userdata pointer
// to be usable across Rust callbacks.
pub(crate) struct Userdata(*mut std::ffi::c_void, *mut std::ffi::c_void);
impl Userdata {
    pub(crate) const NULL: Userdata = Userdata::new(std::ptr::null_mut(), std::ptr::null_mut());

    #[inline]
    pub(crate) const fn new(
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) -> Userdata {
        Userdata(userdata1, userdata2)
    }

    #[inline]
    pub(crate) fn get_1(&self) -> *mut std::ffi::c_void {
        self.0
    }

    #[inline]
    pub(crate) fn get_2(&self) -> *mut std::ffi::c_void {
        self.1
    }
}

#[macro_export]
macro_rules! new_userdata {
    ($var:expr) => {
        crate::utils::Userdata::new($var.userdata1, $var.userdata2)
    };
}

unsafe impl Send for Userdata {}
unsafe impl Sync for Userdata {}

// Safer wrapper around `slice::from_raw_parts` to handle
// invalid `ptr` when `len` is zero.
#[inline]
pub(crate) fn make_slice<'a, T: 'a>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[inline]
pub fn get_base_device_limits_from_adapter_limits(adapter_limits: &wgt::Limits) -> wgt::Limits {
    let default_limits = wgt::Limits::default();
    let dim_1d = std::cmp::min(
        adapter_limits.max_texture_dimension_1d,
        default_limits.max_texture_dimension_1d,
    );
    let dim_2d = std::cmp::min(
        adapter_limits.max_texture_dimension_2d,
        default_limits.max_texture_dimension_2d,
    );
    let dim_3d = std::cmp::min(
        adapter_limits.max_texture_dimension_3d,
        default_limits.max_texture_dimension_3d,
    );

    let default_limits_with_resolution = wgt::Limits {
        max_texture_dimension_1d: dim_1d,
        max_texture_dimension_2d: dim_2d,
        max_texture_dimension_3d: dim_3d,
        ..default_limits
    };
    if wgt::Limits::check_limits(&default_limits_with_resolution, adapter_limits) {
        return default_limits_with_resolution;
    }

    let downlevel_defaults_limits_with_resolution = wgt::Limits {
        max_texture_dimension_1d: dim_1d,
        max_texture_dimension_2d: dim_2d,
        max_texture_dimension_3d: dim_3d,
        ..wgt::Limits::downlevel_defaults()
    };
    if wgt::Limits::check_limits(&downlevel_defaults_limits_with_resolution, adapter_limits) {
        return downlevel_defaults_limits_with_resolution;
    }

    wgt::Limits {
        max_texture_dimension_1d: dim_1d,
        max_texture_dimension_2d: dim_2d,
        max_texture_dimension_3d: dim_3d,
        ..wgt::Limits::downlevel_webgl2_defaults()
    }
}

pub fn texture_format_has_depth(format: wgt::TextureFormat) -> bool {
    return format == wgt::TextureFormat::Depth16Unorm
        || format == wgt::TextureFormat::Depth24Plus
        || format == wgt::TextureFormat::Depth24PlusStencil8
        || format == wgt::TextureFormat::Depth32Float
        || format == wgt::TextureFormat::Depth32FloatStencil8;
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
/// Given two or more extension structs of the same `SType` in the same chain, this macro will favor the latter most. There should
/// not be more than one extension struct with the same `SType` in a chain anyway, so this behavior should be unproblematic.

#[macro_export]
macro_rules! follow_chain {
    ($func:ident(($base:expr) $(, $stype:ident => $ty:ty)*)) => {{
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
    ($func:ident(($base1:expr, $base2:expr) $(, $stype:ident => $ty:ty)*)) => {{
        #[allow(non_snake_case)] // We use the type name as an easily usable temporary name
        {
            $(
                let mut $stype: Option<&$ty> = None;
            )*
            let mut chain_opt: Option<&$crate::native::WGPUChainedStruct> = $base1.nextInChain.as_ref();
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
            $func($base1, $base2, $($stype),*)
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
/// pub fn map_index_format(value: native::WGPUIndexFormat) -> Result<wgt::IndexFormat, native::WGPUIndexFormat> {
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
        #[inline]
        pub fn $name(value: native::$c_name) -> Result<$rs_type, native::$c_name> {
            match value {
                $(paste::paste!(native::[<$c_name _ $variant>]) => Ok(<$rs_type>::$variant)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($variant:ident),+) => {
        #[inline]
        pub fn $name(value: native::$c_name) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($variant),+);

            map_fn(value).expect($err_msg)
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $($native_variant:ident:$variant2:ident),+) => {
        #[inline]
        pub fn $name(value: native::$c_name) -> Result<$rs_type, native::$c_name> {
            match value {
                $(paste::paste!(native::[<$c_name _ $native_variant>]) => Ok(<$rs_type>::$variant2)),+,
                x => Err(x),
            }
        }
    };
    ($name:ident, $c_name:ident, $rs_type:ty, $err_msg:literal, $($native_variant:ident:$variant2:ident),+) => {
        #[inline]
        pub fn $name(value: native::$c_name) -> $rs_type {
            map_enum!(map_fn, $c_name, $rs_type, $($native_variant:$variant2),+);

            map_fn(value).expect($err_msg)
        }
    };
}

pub unsafe fn string_view_into_str<'a>(string_view: native::WGPUStringView) -> Option<&'a str> {
    if string_view.data.is_null() {
        match string_view.length {
            crate::conv::WGPU_STRLEN => None,
            0 => Some(""),
            _ => panic!("Null address to WGPUStringView!"),
        }
    } else {
        unsafe {
            let bytes = match string_view.length {
                crate::conv::WGPU_STRLEN => CStr::from_ptr(string_view.data).to_bytes(),
                _ => make_slice(string_view.data as *const u8, string_view.length),
            };

            Some(std::str::from_utf8_unchecked(bytes))
        }
    }
}

pub unsafe fn string_view_into_label<'a>(string_view: native::WGPUStringView) -> wgc::Label<'a> {
    string_view_into_str(string_view).map(Cow::Borrowed)
}

pub const fn str_into_string_view(str: &str) -> native::WGPUStringView {
    native::WGPUStringView {
        data: str.as_ptr() as *const std::os::raw::c_char,
        length: str.len(),
    }
}

/// Create a string view that "owns" its memory, so it can be later dropped with [drop_string_view].
pub fn str_into_owned_string_view(str: &str) -> native::WGPUStringView {
    let boxed = String::from(str).into_boxed_str();

    let result = native::WGPUStringView {
        data: boxed.as_ptr() as *const std::os::raw::c_char,
        length: boxed.len(),
    };

    std::mem::forget(boxed);

    result
}

/// Drop a string view created by [str_into_owned_string_view].
pub unsafe fn drop_string_view(view: native::WGPUStringView) {
    if view.data.is_null() {
        return;
    }

    drop(Box::from_raw(std::slice::from_raw_parts_mut(
        view.data as *mut u8,
        view.length,
    )))
}

#[test]
pub fn test_string_view_into_str() {
    let str = "Hello, world!";
    let string_view = str_into_string_view(str);
    let str_2 = unsafe { string_view_into_str(string_view) }.unwrap();

    assert_eq!(str, str_2)
}

#[test]
pub fn test_get_base_device_limits_from_adapter_limits() {
    fn expected_limits_with_default_resolution(
        adapter_limits: wgt::Limits,
        expected: wgt::Limits,
    ) -> wgt::Limits {
        let default_limits = wgt::Limits::default();
        let dim_1d = std::cmp::min(
            adapter_limits.max_texture_dimension_1d,
            default_limits.max_texture_dimension_1d,
        );
        let dim_2d = std::cmp::min(
            adapter_limits.max_texture_dimension_2d,
            default_limits.max_texture_dimension_2d,
        );
        let dim_3d = std::cmp::min(
            adapter_limits.max_texture_dimension_3d,
            default_limits.max_texture_dimension_3d,
        );
        wgt::Limits {
            max_texture_dimension_1d: dim_1d,
            max_texture_dimension_2d: dim_2d,
            max_texture_dimension_3d: dim_3d,
            ..expected
        }
    }

    // max_uniform_buffer_binding_size
    //  default: 64 << 10
    //  downlevel_defaults: 16 << 10
    //  downlevel_webgl2_defaults: 16 << 10
    {
        let adapter_limits = wgt::Limits {
            max_uniform_buffer_binding_size: (16 << 10) - 1,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(
                adapter_limits,
                wgt::Limits::downlevel_webgl2_defaults()
            ),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_uniform_buffer_binding_size: 16 << 10,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(
                adapter_limits,
                wgt::Limits::downlevel_defaults()
            ),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_uniform_buffer_binding_size: (16 << 10) + 1,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(
                adapter_limits,
                wgt::Limits::downlevel_defaults()
            ),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_uniform_buffer_binding_size: 64 << 10,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(adapter_limits, wgt::Limits::default()),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_uniform_buffer_binding_size: (64 << 10) + 1,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(adapter_limits, wgt::Limits::default()),
        );
    }

    // max_compute_workgroups_per_dimension
    //  default: 65535
    //  downlevel_defaults: 65535
    //  downlevel_webgl2_defaults: 0
    {
        let adapter_limits = wgt::Limits {
            max_compute_workgroups_per_dimension: 0,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(
                adapter_limits,
                wgt::Limits::downlevel_webgl2_defaults()
            ),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_compute_workgroups_per_dimension: 65535 - 1,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(
                adapter_limits,
                wgt::Limits::downlevel_webgl2_defaults()
            ),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_compute_workgroups_per_dimension: 65535,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(adapter_limits, wgt::Limits::default()),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_compute_workgroups_per_dimension: 65535 + 1,
            ..Default::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            expected_limits_with_default_resolution(adapter_limits, wgt::Limits::default()),
        );
    }

    // Texture dimensions are clamped to default limits.
    {
        let adapter_limits = wgt::Limits {
            max_texture_dimension_1d: 16 << 10,
            max_texture_dimension_2d: 16 << 10,
            max_texture_dimension_3d: 16 << 10,
            ..wgt::Limits::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            wgt::Limits::default(),
        );
    }
    {
        let adapter_limits = wgt::Limits {
            max_texture_dimension_1d: 2 << 10,
            max_texture_dimension_2d: 2 << 10,
            max_texture_dimension_3d: 2 << 10,
            ..wgt::Limits::downlevel_defaults()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            wgt::Limits {
                max_texture_dimension_1d: 2 << 10,
                max_texture_dimension_2d: 2 << 10,
                max_texture_dimension_3d: 2 << 10,
                ..wgt::Limits::downlevel_defaults()
            },
        );
    }
    {
        // Ensure that texture resolution limits of an adapter lower than
        // the default does not lead to the selection of downlevel limits
        // as the base limits.
        let adapter_limits = wgt::Limits {
            max_texture_dimension_1d: 16 << 10,
            max_texture_dimension_2d: 16 << 10,
            max_texture_dimension_3d: 2 << 10,
            ..wgt::Limits::default()
        };
        assert_eq!(
            get_base_device_limits_from_adapter_limits(&adapter_limits),
            wgt::Limits {
                max_texture_dimension_1d: 8 << 10,
                max_texture_dimension_2d: 8 << 10,
                max_texture_dimension_3d: 2 << 10,
                ..wgt::Limits::default()
            },
        );
    }
}
