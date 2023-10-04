use std::{
    borrow::Cow,
    ffi::CStr,
    path::{Path, PathBuf},
};

// A dummy wrapper that is `Send` + `Sync` to store userdata pointer
// to be usable across Rust callbacks.
pub(crate) struct Userdata(*mut std::ffi::c_void);
impl Userdata {
    #[inline]
    pub(crate) const fn new(userdata: *mut std::ffi::c_void) -> Userdata {
        Userdata(userdata)
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.0
    }
}
unsafe impl Send for Userdata {}
unsafe impl Sync for Userdata {}

#[inline]
pub(crate) fn ptr_into_label<'a>(ptr: *const std::ffi::c_char) -> wgc::Label<'a> {
    unsafe { ptr.as_ref() }.and_then(|ptr| {
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .ok()
            .map(Cow::Borrowed)
    })
}
#[inline]
pub(crate) fn ptr_into_path<'a>(ptr: *const std::ffi::c_char) -> Option<&'a std::path::Path> {
    unsafe { ptr.as_ref() }
        .and_then(|v| unsafe { CStr::from_ptr(v) }.to_str().ok())
        .map(Path::new)
}
#[inline]
pub(crate) fn ptr_into_pathbuf(ptr: *const std::ffi::c_char) -> Option<std::path::PathBuf> {
    unsafe { ptr.as_ref() }
        .and_then(|v| unsafe { CStr::from_ptr(v) }.to_str().ok())
        .map(PathBuf::from)
}

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

// Check if downlevel limits should be preffered over the default limits.
pub fn should_use_downlevel_limits(adapter_limits: &wgt::Limits) -> bool {
    !wgt::Limits::default().check_limits(adapter_limits)
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

#[test]
pub fn test_should_use_downlevel_limits() {
    {
        let adapter_limits = wgt::Limits {
            max_bind_groups: 2, // default are 4
            ..Default::default()
        };
        assert_eq!(should_use_downlevel_limits(&adapter_limits), true);
    }
    {
        let adapter_limits = wgt::Limits {
            max_bind_groups: 4, // default are 4
            ..Default::default()
        };
        assert_eq!(should_use_downlevel_limits(&adapter_limits), false);
    }
    {
        let adapter_limits = wgt::Limits {
            max_bind_groups: 8, // default are 4
            ..Default::default()
        };
        assert_eq!(should_use_downlevel_limits(&adapter_limits), false);
    }
}
