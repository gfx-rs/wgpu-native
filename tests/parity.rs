use wgpu_native::{conv, native};

#[test]
fn acceleration_structure_buffer_usages_use_a_separate_namespace() {
    let mut descriptor: native::WGPUBufferDescriptor = unsafe { std::mem::zeroed() };
    descriptor.usage = native::WGPUBufferUsage_CopyDst;
    for (native_usage, expected) in [
        (
            native::WGPUWgpuBufferUsage_BlasInput,
            wgt::BufferUsages::BLAS_INPUT,
        ),
        (
            native::WGPUWgpuBufferUsage_TlasInput,
            wgt::BufferUsages::TLAS_INPUT,
        ),
    ] {
        let mut extras: native::WGPUWgpuBufferUsageExtras = unsafe { std::mem::zeroed() };
        extras.usage = native_usage;
        assert_eq!(
            conv::map_buffer_usage(&descriptor, Some(&extras)),
            Some(wgt::BufferUsages::COPY_DST | expected)
        );
    }
    assert_eq!(
        conv::map_buffer_usage(&descriptor, None),
        Some(wgt::BufferUsages::COPY_DST)
    );
    for unknown in [0x400, 0x800, 1 << 32, 1 << 63] {
        descriptor.usage = native::WGPUBufferUsage_CopyDst | unknown;
        assert_eq!(conv::map_buffer_usage(&descriptor, None), None);
    }
    descriptor.usage = native::WGPUBufferUsage_CopyDst;
    let mut extras: native::WGPUWgpuBufferUsageExtras = unsafe { std::mem::zeroed() };
    for unknown in [4, 0x400, 0x800, 1 << 32, 1 << 63] {
        extras.usage = native::WGPUWgpuBufferUsage_BlasInput | unknown;
        assert_eq!(conv::map_buffer_usage(&descriptor, Some(&extras)), None);
    }
}

#[test]
fn all_standard_buffer_usages_remain_independent_of_native_usages() {
    for (flag, expected) in [
        (native::WGPUBufferUsage_MapRead, wgt::BufferUsages::MAP_READ),
        (
            native::WGPUBufferUsage_MapWrite,
            wgt::BufferUsages::MAP_WRITE,
        ),
        (native::WGPUBufferUsage_CopySrc, wgt::BufferUsages::COPY_SRC),
        (native::WGPUBufferUsage_CopyDst, wgt::BufferUsages::COPY_DST),
        (native::WGPUBufferUsage_Index, wgt::BufferUsages::INDEX),
        (native::WGPUBufferUsage_Vertex, wgt::BufferUsages::VERTEX),
        (native::WGPUBufferUsage_Uniform, wgt::BufferUsages::UNIFORM),
        (native::WGPUBufferUsage_Storage, wgt::BufferUsages::STORAGE),
        (
            native::WGPUBufferUsage_Indirect,
            wgt::BufferUsages::INDIRECT,
        ),
        (
            native::WGPUBufferUsage_QueryResolve,
            wgt::BufferUsages::QUERY_RESOLVE,
        ),
    ] {
        let mut descriptor: native::WGPUBufferDescriptor = unsafe { std::mem::zeroed() };
        descriptor.usage = flag;
        assert_eq!(conv::map_buffer_usage(&descriptor, None), Some(expected));
    }
}

#[test]
fn native_usage_extensions_are_found_after_other_chain_entries() {
    use conv::{map_buffer_usage, map_texture_descriptor_usage};
    let mut buffer: native::WGPUBufferDescriptor = unsafe { std::mem::zeroed() };
    buffer.usage = native::WGPUBufferUsage_CopyDst;
    let mut buffer_extras: native::WGPUWgpuBufferUsageExtras = unsafe { std::mem::zeroed() };
    buffer_extras.chain.sType = native::WGPUSType_WgpuBufferUsageExtras;
    buffer_extras.usage =
        native::WGPUWgpuBufferUsage_BlasInput | native::WGPUWgpuBufferUsage_TlasInput;
    let mut texture: native::WGPUTextureDescriptor = unsafe { std::mem::zeroed() };
    texture.usage = native::WGPUTextureUsage_StorageBinding;
    let mut texture_extras: native::WGPUWgpuTextureUsageExtras = unsafe { std::mem::zeroed() };
    texture_extras.chain.sType = native::WGPUSType_WgpuTextureUsageExtras;
    texture_extras.usage = native::WGPUWgpuTextureUsage_StorageAtomic;
    for prefix_count in [0, 1, 2] {
        for (head, is_buffer) in [
            (&mut buffer_extras.chain, true),
            (&mut texture_extras.chain, false),
        ] {
            let mut head = std::ptr::from_mut(head);
            let mut prefix = vec![
                native::WGPUChainedStruct {
                    next: std::ptr::null_mut(),
                    sType: 0x7fff_ff01
                };
                prefix_count
            ];
            for link in &mut prefix {
                link.next = head;
                head = std::ptr::from_mut(link);
            }
            if is_buffer {
                buffer.nextInChain = head;
                let mapped = unsafe {
                    wgpu_native::follow_chain!(map_buffer_usage((&buffer),
                    WGPUSType_WgpuBufferUsageExtras => native::WGPUWgpuBufferUsageExtras))
                };
                assert_eq!(
                    mapped,
                    Some(
                        wgt::BufferUsages::COPY_DST
                            | wgt::BufferUsages::BLAS_INPUT
                            | wgt::BufferUsages::TLAS_INPUT
                    )
                );
            } else {
                texture.nextInChain = head;
                let mapped = unsafe {
                    wgpu_native::follow_chain!(map_texture_descriptor_usage((&texture),
                    WGPUSType_WgpuTextureUsageExtras => native::WGPUWgpuTextureUsageExtras))
                };
                assert_eq!(
                    mapped,
                    Some(wgt::TextureUsages::STORAGE_BINDING | wgt::TextureUsages::STORAGE_ATOMIC)
                );
            }
        }
    }
}

#[test]
fn atomic_texture_usages_are_separate_and_views_inherit_when_unspecified() {
    let mut extras: native::WGPUWgpuTextureUsageExtras = unsafe { std::mem::zeroed() };
    extras.usage = native::WGPUWgpuTextureUsage_StorageAtomic;
    let expected = wgt::TextureUsages::TEXTURE_BINDING | wgt::TextureUsages::STORAGE_ATOMIC;
    assert_eq!(
        conv::map_texture_usage(native::WGPUTextureUsage_TextureBinding, Some(&extras)),
        Some(expected)
    );
    assert_eq!(
        conv::to_native_texture_usage_flags(expected),
        native::WGPUTextureUsage_TextureBinding
    );
    assert_eq!(
        conv::to_native_wgpu_texture_usage(expected),
        native::WGPUWgpuTextureUsage_StorageAtomic
    );
    for unknown in [1 << 16, 1 << 32, 1 << 63] {
        assert_eq!(conv::map_texture_usage(unknown, None), None);
        extras.usage = unknown | native::WGPUWgpuTextureUsage_StorageAtomic;
        assert_eq!(conv::map_texture_usage(0, Some(&extras)), None);
    }
    let mut view: native::WGPUTextureViewDescriptor = unsafe { std::mem::zeroed() };
    assert_eq!(conv::map_texture_view_usage(&view, None), None);
    extras.usage = 0;
    assert_eq!(conv::map_texture_view_usage(&view, Some(&extras)), None);
    extras.usage = native::WGPUWgpuTextureUsage_StorageAtomic;
    assert_eq!(
        conv::map_texture_view_usage(&view, Some(&extras)),
        Some(wgt::TextureUsages::STORAGE_ATOMIC)
    );
    view.usage = native::WGPUTextureUsage_TextureBinding;
    assert_eq!(
        conv::map_texture_view_usage(&view, Some(&extras)),
        Some(expected)
    );
    assert_eq!(
        conv::map_texture_view_usage(&view, None),
        Some(wgt::TextureUsages::TEXTURE_BINDING)
    );
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

#[test]
fn adapter_info_free_members_visits_later_extensions() {
    for prefix_count in [0, 1, 2] {
        let mut extras: native::WGPUAdapterInfoExtras = unsafe { std::mem::zeroed() };
        extras.chain.sType = native::WGPUSType_AdapterInfoExtras;
        extras.devicePciBusId = wgpu_native::utils::str_into_owned_string_view("0000:01:00.0");
        let mut head = std::ptr::from_mut(&mut extras.chain);
        let mut prefix = vec![
            native::WGPUChainedStruct {
                next: std::ptr::null_mut(),
                sType: 0x7fff_ff01,
            };
            prefix_count
        ];
        for link in &mut prefix {
            link.next = head;
            head = std::ptr::from_mut(link);
        }
        let mut info: native::WGPUAdapterInfo = unsafe { std::mem::zeroed() };
        info.nextInChain = head;
        unsafe { wgpu_native::wgpuAdapterInfoFreeMembers(info) };
        let cleared = extras.devicePciBusId.data.is_null() && extras.devicePciBusId.length == 0;
        if !cleared {
            unsafe { wgpu_native::utils::drop_string_view(extras.devicePciBusId) };
        }
        assert!(cleared, "adapter info extras after {prefix_count} earlier extensions were not freed and cleared");
    }
}
