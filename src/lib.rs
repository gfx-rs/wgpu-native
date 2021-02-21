use std::borrow::Cow;
use std::sync::Arc;
use std::slice;

mod command;
mod device;
mod logging;

pub use self::command::*;
pub use self::device::*;
pub use self::logging::*;

type Global = wgc::hub::Global<wgc::hub::IdentityManagerFactory>;

pub type Label = *const libc::c_char;

struct OwnedLabel(Option<String>);
impl OwnedLabel {
    fn new(label: Label) -> Self {
        OwnedLabel(if label.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(label) }
                    .to_string_lossy()
                    .to_string(),
            )
        })
    }
    fn as_cow(&self) -> Option<Cow<str>> {
        self.0.as_ref().map(|s| Cow::Borrowed(s.as_str()))
    }
    fn into_cow<'a>(self) -> Option<Cow<'a, str>> {
        self.0.map(|s| Cow::Owned(s))
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Global> = Arc::new(Global::new("wgpu", wgc::hub::IdentityManagerFactory, wgt::BackendBit::PRIMARY));
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
        let mut chain_opt: Option<&$crate::ChainedStruct> = $base.next_in_chain;
        while let Some(next_in_chain) = chain_opt {
            match next_in_chain.s_type {
                $(
                    $crate::SType::$stype => {
                        let next_in_chain_ptr = next_in_chain as *const $crate::ChainedStruct;
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
            chain_opt = next_in_chain.next;
        }
        $func($base, $($stype),*)
    }}};
}

#[repr(u32)]
pub enum SType {
    Invalid = 0x00_00_00_00,
    SurfaceDescriptorFromMetalLayer = 0x00_00_00_01,
    SurfaceDescriptorFromWindowsHWND = 0x00_00_00_02,
    SurfaceDescriptorFromXlib = 0x00_00_00_03,
    SurfaceDescriptorFromHTMLCanvasId = 0x00_00_00_04,
    ShaderModuleSPIRVDescriptor = 0x00_00_00_05,
    ShaderModuleWGSLDescriptor = 0x00_00_00_06,
    /// Placeholder value until real value can be determined
    AnisotropicFiltering = 0x10_00_00_00,
    BorderClampColor = 0x20_00_00_00,
    Force32 = 0x7F_FF_FF_FF,
}

#[repr(C)]
pub struct ChainedStruct<'c> {
    next: Option<&'c ChainedStruct<'c>>,
    s_type: SType,
}

#[track_caller]
pub fn check_error<I, E: std::fmt::Debug>(input: (I, Option<E>)) -> I {
    if let Some(error) = input.1 {
        panic!("{:?}", error);
    }

    input.0
}

pub(crate) unsafe fn make_slice<'a, T: 'a>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        slice::from_raw_parts(pointer, count)
    }
}

#[repr(u32)]
pub enum IndexFormat {
    Undefined = 0,
    Uint16 = 1,
    Uint32 = 2,
}

impl IndexFormat {
    fn to_wgpu(&self) -> Option<wgt::IndexFormat> {
        match self {
            IndexFormat::Undefined => None,
            IndexFormat::Uint16 => Some(wgt::IndexFormat::Uint16),
            IndexFormat::Uint32 => Some(wgt::IndexFormat::Uint32),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_get_version() -> std::os::raw::c_uint {
    let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    (major << 16) + (minor << 8) + patch
}
