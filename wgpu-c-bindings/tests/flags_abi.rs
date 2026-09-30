use std::mem::{align_of, offset_of, size_of};
use wgpu_c_bindings as bindings;
use wgpu_native::native as runtime;

#[repr(C)]
struct ExpectedDownlevelCapabilities {
    flags: u64,
    shader_model: u32,
}

#[repr(C)]
struct ExpectedTextureFormatCapabilities {
    allowed_usages: u64,
    flags: u64,
}

#[test]
fn public_flag_types_and_layouts_use_wgpu_flags() {
    // Test against an independent 64-bit contract, not just two bindings
    // generated from the same potentially incorrect header.
    const HIGH_BIT: u64 = 1 << 63;
    macro_rules! check_abi {
        ($api:ident) => {
            let _: $api::WGPUNativeTextureFormatFeatureFlags = HIGH_BIT;
            let _: $api::WGPUWgslLanguageFeatures = HIGH_BIT;
            let _: $api::WGPUDownlevelFlags = HIGH_BIT;
            assert_eq!(
                size_of::<$api::WGPUDownlevelCapabilities>(),
                size_of::<ExpectedDownlevelCapabilities>()
            );
            assert_eq!(
                align_of::<$api::WGPUDownlevelCapabilities>(),
                align_of::<ExpectedDownlevelCapabilities>()
            );
            assert_eq!(
                offset_of!($api::WGPUDownlevelCapabilities, shaderModel),
                offset_of!(ExpectedDownlevelCapabilities, shader_model)
            );
            assert_eq!(
                size_of::<$api::WGPUNativeTextureFormatCapabilities>(),
                size_of::<ExpectedTextureFormatCapabilities>()
            );
            assert_eq!(
                align_of::<$api::WGPUNativeTextureFormatCapabilities>(),
                align_of::<ExpectedTextureFormatCapabilities>()
            );
            assert_eq!(
                offset_of!($api::WGPUNativeTextureFormatCapabilities, flags),
                offset_of!(ExpectedTextureFormatCapabilities, flags)
            );
        };
    }
    check_abi!(bindings);
    check_abi!(runtime);
}

#[test]
fn wgsl_query_returns_full_width_flags_through_the_c_binding() {
    let query: unsafe extern "C" fn() -> u64 = bindings::wgpuGetWgslLanguageFeatures;
    let runtime_query: extern "C" fn() -> u64 = wgpu_native::wgpuGetWgslLanguageFeatures;
    assert_eq!(unsafe { query() }, runtime_query());
}
