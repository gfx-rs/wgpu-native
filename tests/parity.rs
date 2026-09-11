use wgpu_native::{conv, native};

#[test]
fn acceleration_structure_buffer_usages_match_public_wgpu_flags() {
    for (native_usage, expected) in [
        (
            native::WGPUBufferUsage_BlasInput,
            wgt::BufferUsages::BLAS_INPUT,
        ),
        (
            native::WGPUBufferUsage_TlasInput,
            wgt::BufferUsages::TLAS_INPUT,
        ),
    ] {
        assert_eq!(
            conv::from_u64_bits::<wgt::BufferUsages>(native_usage),
            Some(expected)
        );
    }
}

#[test]
fn external_texture_layout_maps_without_an_ordinary_binding() {
    let mut entry: native::WGPUBindGroupLayoutEntry = unsafe { std::mem::zeroed() };
    entry.binding = 7;
    entry.visibility = native::WGPUShaderStage_Fragment;
    let external = native::WGPUExternalTextureBindingLayout {
        chain: native::WGPUChainedStruct {
            next: std::ptr::null_mut(),
            sType: native::WGPUSType_ExternalTextureBindingLayout,
        },
    };
    let mapped = conv::map_bind_group_layout_entry(&entry, None, None, Some(&external));
    assert_eq!(mapped.binding, 7);
    assert_eq!(mapped.visibility, wgt::ShaderStages::FRAGMENT);
    assert_eq!(mapped.ty, wgt::BindingType::ExternalTexture);
    assert_eq!(mapped.count, None);
}

#[test]
fn atomic_texture_symbols_are_available_to_c_callers() {
    assert_eq!(
        conv::map_storage_texture_access(native::WGPUStorageTextureAccess_Atomic as _),
        Some(wgt::StorageTextureAccess::Atomic),
    );
    let format = native::WGPUNativeTextureFormat_R64Uint as native::WGPUTextureFormat;
    assert_eq!(
        conv::map_texture_format(format),
        Some(wgt::TextureFormat::R64Uint)
    );
    assert_eq!(
        conv::to_native_texture_format(wgt::TextureFormat::R64Uint),
        Some(format)
    );
}

#[test]
fn unavailable_presentation_timestamp_is_a_sentinel() {
    assert_eq!(
        conv::map_presentation_timestamp(wgt::PresentationTimestamp::INVALID_TIMESTAMP).nanoseconds,
        u64::MAX,
    );
}

#[test]
fn presentation_timestamp_overflow_does_not_wrap() {
    assert_eq!(
        conv::map_presentation_timestamp(wgt::PresentationTimestamp(u64::MAX as u128 + 1))
            .nanoseconds,
        u64::MAX,
    );
}

#[test]
fn valid_presentation_timestamps_preserve_nanoseconds() {
    for timestamp in [0, 1, 16_666_667, u64::MAX - 1] {
        assert_eq!(
            conv::map_presentation_timestamp(wgt::PresentationTimestamp(timestamp as u128))
                .nanoseconds,
            timestamp,
        );
    }
}
