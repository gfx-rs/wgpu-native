use crate::conv::{map_adapter_options, map_device_descriptor, map_shader_module};
use crate::{check_error, conv, follow_chain, make_slice, native, OwnedLabel, GLOBAL};
use std::{
    borrow::Cow,
    convert::TryInto,
    marker::PhantomData,
    num::{NonZeroU32, NonZeroU64, NonZeroU8},
    path::Path,
};
use wgc::{gfx_select, id};

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceRequestAdapter(
    _: native::WGPUInstance,
    options: &native::WGPURequestAdapterOptions,
    callback: native::WGPURequestAdapterCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (compatible_surface, given_backend) = follow_chain!(
        map_adapter_options(options,
        WGPUSType_AdapterExtras => native::WGPUAdapterExtras)
    );
    let backend_bits = match given_backend {
        native::WGPUBackendType_Null => wgt::BackendBit::PRIMARY,
        native::WGPUBackendType_Vulkan => wgt::BackendBit::VULKAN,
        native::WGPUBackendType_Metal => wgt::BackendBit::METAL,
        native::WGPUBackendType_D3D12 => wgt::BackendBit::DX12,
        native::WGPUBackendType_D3D11 => wgt::BackendBit::DX11,
        native::WGPUBackendType_OpenGL => wgt::BackendBit::GL,
        _ => panic!("Invalid backend {}", given_backend),
    };
    let id = GLOBAL
        .request_adapter(
            &wgt::RequestAdapterOptions {
                power_preference: wgt::PowerPreference::default(),
                compatible_surface,
            },
            wgc::instance::AdapterInputs::Mask(backend_bits, |_| PhantomData),
        )
        .expect("Unable to request adapter");
    (callback.unwrap())(id, userdata);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: id::AdapterId,
    descriptor: &native::WGPUDeviceDescriptor,
    callback: native::WGPURequestDeviceCallback,
    userdata: *mut ::std::os::raw::c_void,
) {
    let (desc, trace_str) = follow_chain!(
        map_device_descriptor(descriptor,
        WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
    );
    let trace_path = trace_str.as_ref().map(|path| Path::new(path));
    let device_id = check_error(
        gfx_select!(adapter => GLOBAL.adapter_request_device(adapter, &desc, trace_path, PhantomData)),
    );
    (callback.unwrap())(device_id, userdata);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateShaderModule(
    device: id::DeviceId,
    descriptor: &native::WGPUShaderModuleDescriptor,
) -> id::ShaderModuleId {
    let label = OwnedLabel::new(descriptor.label);
    let source = follow_chain!(
        map_shader_module(descriptor,
        WGPUSType_ShaderModuleSPIRVDescriptor => native::WGPUShaderModuleSPIRVDescriptor,
        WGPUSType_ShaderModuleWGSLDescriptor => native::WGPUShaderModuleWGSLDescriptor)
    );

    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: label.as_cow(),
        flags: wgt::ShaderFlags::VALIDATION,
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_shader_module(device, &desc, source, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateBuffer(
    device: id::DeviceId,
    desc: &native::WGPUBufferDescriptor,
) -> id::BufferId {
    let usage = wgt::BufferUsage::from_bits(desc.usage).expect("Buffer Usage Invalid.");
    let label = OwnedLabel::new(desc.label);
    check_error(gfx_select!(device => GLOBAL.device_create_buffer(
        device,
        &wgt::BufferDescriptor {
            label: label.as_cow(),
            size: desc.size,
            usage,
            mapped_at_creation: desc.mappedAtCreation,
        },
        PhantomData
    )))
}

#[no_mangle]
pub extern "C" fn wgpuBufferDestroy(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_destroy(buffer_id)).expect("Unable to destroy buffer");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: id::DeviceId,
    descriptor: &native::WGPUBindGroupLayoutDescriptor,
) -> id::BindGroupLayoutId {
    let mut entries = Vec::new();

    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let is_buffer = entry.buffer.type_ != native::WGPUBufferBindingType_Undefined;
        let is_sampler = entry.sampler.type_ != native::WGPUSamplerBindingType_Undefined;
        let is_texture = entry.texture.sampleType != native::WGPUTextureSampleType_Undefined;
        let is_storage_texture =
            entry.storageTexture.access != native::WGPUStorageTextureAccess_Undefined;

        let ty = if is_texture {
            wgt::BindingType::Texture {
                sample_type: match entry.texture.sampleType {
                    native::WGPUTextureSampleType_Float => {
                        wgt::TextureSampleType::Float { filterable: true }
                    }
                    native::WGPUTextureSampleType_UnfilterableFloat => {
                        wgt::TextureSampleType::Float { filterable: false }
                    }
                    native::WGPUTextureSampleType_Depth => wgt::TextureSampleType::Depth,
                    native::WGPUTextureSampleType_Sint => wgt::TextureSampleType::Sint,
                    native::WGPUTextureSampleType_Uint => wgt::TextureSampleType::Uint,
                    x => panic!("Unknown texture SampleType: {}", x),
                },
                view_dimension: match entry.texture.viewDimension {
                    native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                    native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                    native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                    native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                    native::WGPUTextureViewDimension_CubeArray => {
                        wgt::TextureViewDimension::CubeArray
                    }
                    native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                    x => panic!("Unknown texture ViewDimension: {}", x),
                },
                multisampled: entry.texture.multisampled,
            }
        } else if is_sampler {
            match entry.sampler.type_ {
                native::WGPUSamplerBindingType_Filtering => wgt::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                native::WGPUSamplerBindingType_NonFiltering => wgt::BindingType::Sampler {
                    filtering: false,
                    comparison: false,
                },
                native::WGPUSamplerBindingType_Comparison => wgt::BindingType::Sampler {
                    filtering: false,
                    comparison: true,
                },
                x => panic!("Unknown Sampler Type: {}", x),
            }
        } else if is_storage_texture {
            wgt::BindingType::StorageTexture {
                access: match entry.storageTexture.access {
                    native::WGPUStorageTextureAccess_ReadOnly => {
                        wgt::StorageTextureAccess::ReadOnly
                    }
                    native::WGPUStorageTextureAccess_WriteOnly => {
                        wgt::StorageTextureAccess::WriteOnly
                    }
                    x => panic!("Unknown StorageTextureAccess: {}", x),
                },
                format: conv::map_texture_format(entry.storageTexture.format)
                    .expect("StorageTexture format missing"),
                view_dimension: match entry.storageTexture.viewDimension {
                    native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                    native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                    native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                    native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                    native::WGPUTextureViewDimension_CubeArray => {
                        wgt::TextureViewDimension::CubeArray
                    }
                    native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                    x => panic!("Unknown texture ViewDimension: {}", x),
                },
            }
        } else if is_buffer {
            wgt::BindingType::Buffer {
                ty: match entry.buffer.type_ {
                    native::WGPUBufferBindingType_Uniform => wgt::BufferBindingType::Uniform,
                    native::WGPUBufferBindingType_Storage => {
                        wgt::BufferBindingType::Storage { read_only: false }
                    }
                    native::WGPUBufferBindingType_ReadOnlyStorage => {
                        wgt::BufferBindingType::Storage { read_only: true }
                    }
                    x => panic!("Unknown Buffer Type: {}", x),
                },
                has_dynamic_offset: entry.buffer.hasDynamicOffset,
                min_binding_size: NonZeroU64::new(entry.buffer.minBindingSize),
            }
        } else {
            panic!("No entry type specified.");
        };

        entries.push(wgt::BindGroupLayoutEntry {
            ty,
            binding: entry.binding,
            visibility: wgt::ShaderStage::from_bits(entry.visibility).unwrap(),
            count: None, // TODO - What is this?
        });
    }
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_bind_group_layout(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: id::DeviceId,
    descriptor: &native::WGPUBindGroupDescriptor,
) -> id::BindGroupId {
    let mut entries = Vec::new();
    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let wgc_entry = if entry.buffer.is_some() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Buffer(
                    wgc::binding_model::BufferBinding {
                        buffer_id: entry.buffer.unwrap(),
                        offset: entry.offset,
                        size: NonZeroU64::new(entry.size),
                    },
                ),
            }
        } else if entry.sampler.is_some() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Sampler(entry.sampler.unwrap()),
            }
        } else if entry.textureView.is_some() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::TextureView(
                    entry.textureView.unwrap(),
                ),
            }
        } else {
            panic!("BindGroup entry does not have buffer nor sampler nor textureView.")
        };
        entries.push(wgc_entry);
    }

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: descriptor.layout,
        entries: Cow::Borrowed(&entries),
    };
    check_error(gfx_select!(device => GLOBAL.device_create_bind_group(device, &desc, PhantomData)))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: id::DeviceId,
    descriptor: &native::WGPUPipelineLayoutDescriptor,
) -> id::PipelineLayoutId {
    let desc = wgc::binding_model::PipelineLayoutDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        bind_group_layouts: Cow::Borrowed(make_slice(
            descriptor.bindGroupLayouts,
            descriptor.bindGroupLayoutCount as usize,
        )),
        push_constant_ranges: Cow::Borrowed(&[]),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_pipeline_layout(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
    device: id::DeviceId,
    descriptor: &native::WGPUComputePipelineDescriptor,
) -> id::ComputePipelineId {
    let stage = wgc::pipeline::ProgrammableStageDescriptor {
        module: descriptor.computeStage.module,
        entry_point: OwnedLabel::new(descriptor.computeStage.entryPoint)
            .into_cow()
            .expect("Entry point not provided"),
    };
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: Some(descriptor.layout),
        stage,
    };

    let (id, error) = gfx_select!(device => GLOBAL.device_create_compute_pipeline(device, &desc, PhantomData, None));

    check_error((id, error))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: id::DeviceId,
    descriptor: &native::WGPUCommandEncoderDescriptor,
) -> id::CommandEncoderId {
    let desc = wgt::CommandEncoderDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_command_encoder(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetQueue(device: id::DeviceId) -> id::QueueId {
    device
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: id::QueueId,
    command_count: u32,
    command_buffers: *const id::CommandBufferId,
) {
    let command_buffer_ids = make_slice(command_buffers, command_count as usize);
    gfx_select!(queue => GLOBAL.queue_submit(queue, command_buffer_ids))
        .expect("Unable to submit queue")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteBuffer(
    queue: id::QueueId,
    buffer: id::BufferId,
    buffer_offset: u64,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
) {
    let slice = make_slice(data, data_size);
    gfx_select!(queue => GLOBAL.queue_write_buffer(queue, buffer, buffer_offset, slice))
        .expect("Unable to write buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteTexture(
    queue: id::QueueId,
    destination: &native::WGPUImageCopyTexture,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
    data_layout: &native::WGPUTextureDataLayout,
    write_size: &native::WGPUExtent3D,
) {
    let slice = make_slice(data, data_size);
    gfx_select!(queue => GLOBAL.queue_write_texture(
        queue,
        &conv::map_image_copy_texture(&destination),
        slice,
        &conv::map_texture_data_layout(&data_layout),
        &conv::map_extent3d(&write_size)
    ))
    .expect("Unable to write texture")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferMapAsync(
    buffer: id::BufferId,
    mode: native::WGPUMapModeFlags,
    offset: usize,
    size: usize,
    callback: native::WGPUBufferMapCallback,
    user_data: *mut u8,
) {
    let operation = wgc::resource::BufferMapOperation {
        host: match mode as crate::EnumConstant {
            native::WGPUMapMode_Write => wgc::device::HostMap::Write,
            native::WGPUMapMode_Read => wgc::device::HostMap::Read,
            x => panic!("Unknown map mode: {}", x),
        },
        // TODO: Change wgpu-core to follow new API
        callback: std::mem::transmute(callback.expect("Callback cannot be null")),
        user_data,
    };

    gfx_select!(buffer => GLOBAL.buffer_map_async(buffer, offset as u64 .. (offset + size) as u64, operation))
        .expect("Unable to map buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePoll(device: id::DeviceId, force_wait: bool) {
    gfx_select!(device => GLOBAL.device_poll(device, force_wait)).expect("Unable to poll device")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: id::BufferId,
    offset: usize,
    size: usize,
) -> *mut u8 {
    gfx_select!(buffer => GLOBAL.buffer_get_mapped_range(buffer, offset as u64, Some(size as u64)))
        .expect("Unable to get mapped range")
        .0
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: id::DeviceId,
    descriptor: &native::WGPURenderPipelineDescriptor,
) -> id::RenderPipelineId {
    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: Some(descriptor.layout),
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor.vertex.module,
                entry_point: OwnedLabel::new(descriptor.vertex.entryPoint)
                    .into_cow()
                    .expect("Entry point not provided"),
            },
            buffers: Cow::Owned(
                make_slice(
                    descriptor.vertex.buffers,
                    descriptor.vertex.bufferCount as usize,
                )
                .iter()
                .map(|buffer| wgc::pipeline::VertexBufferLayout {
                    array_stride: buffer.arrayStride,
                    step_mode: match buffer.stepMode {
                        native::WGPUInputStepMode_Vertex => wgt::InputStepMode::Vertex,
                        native::WGPUInputStepMode_Instance => wgt::InputStepMode::Instance,
                        x => panic!("Unknown step mode {}", x),
                    },
                    attributes: Cow::Owned(
                        make_slice(buffer.attributes, buffer.attributeCount as usize)
                            .iter()
                            .map(|attribute| wgt::VertexAttribute {
                                format: conv::map_vertex_format(attribute.format)
                                    .expect("Vertex Format must be defined"),
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
                _ => panic!("Front face not provided"),
            },
            cull_mode: match descriptor.primitive.cullMode {
                native::WGPUCullMode_Front => Some(wgt::Face::Front),
                native::WGPUCullMode_Back => Some(wgt::Face::Back),
                _ => None,
            },
            clamp_depth: false, // todo: fill this via extras
            polygon_mode: wgt::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: descriptor
            .depthStencil
            .as_ref()
            .map(|desc| wgt::DepthStencilState {
                format: conv::map_texture_format(desc.format)
                    .expect("Texture format must be defined in DepthStencilState"),
                depth_write_enabled: desc.depthWriteEnabled,
                depth_compare: conv::map_compare_function(desc.depthCompare).unwrap(),
                stencil: wgt::StencilState {
                    front: conv::map_stencil_face_state(desc.stencilFront),
                    back: conv::map_stencil_face_state(desc.stencilBack),
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
            alpha_to_coverage_enabled: descriptor.multisample.alphaToCoverageEnabled,
        },
        fragment: descriptor
            .fragment
            .as_ref()
            .map(|fragment| wgc::pipeline::FragmentState {
                stage: wgc::pipeline::ProgrammableStageDescriptor {
                    module: fragment.module,
                    entry_point: OwnedLabel::new(fragment.entryPoint)
                        .into_cow()
                        .expect("Entry point not provided"),
                },
                targets: Cow::Owned(
                    make_slice(fragment.targets, fragment.targetCount as usize)
                        .iter()
                        .map(|color_target| wgt::ColorTargetState {
                            format: conv::map_texture_format(color_target.format)
                                .expect("Texture format must be defined"),
                            blend: color_target.blend.as_ref().map(|blend| wgt::BlendState {
                                color: wgt::BlendComponent {
                                    src_factor: conv::map_blend_factor(blend.color.srcFactor),
                                    dst_factor: conv::map_blend_factor(blend.color.dstFactor),
                                    operation: conv::map_blend_operation(blend.color.operation),
                                },
                                alpha: wgt::BlendComponent {
                                    src_factor: conv::map_blend_factor(blend.alpha.srcFactor),
                                    dst_factor: conv::map_blend_factor(blend.alpha.dstFactor),
                                    operation: conv::map_blend_operation(blend.alpha.operation),
                                },
                            }),
                            write_mask: wgt::ColorWrite::from_bits(color_target.writeMask).unwrap(),
                        })
                        .collect(),
                ),
            }),
    };
    let (id, error) = gfx_select!(device => GLOBAL.device_create_render_pipeline(device, &desc, PhantomData, None));
    if let Some(err) = error {
        panic!("{:?}", err);
    }
    id
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateSwapChain(
    device: id::DeviceId,
    surface: id::SurfaceId,
    desc: &native::WGPUSwapChainDescriptor,
) -> id::SwapChainId {
    let desc = wgt::SwapChainDescriptor {
        usage: wgt::TextureUsage::from_bits(desc.usage).unwrap(),
        format: conv::map_texture_format(desc.format).expect("Texture format not defined"),
        width: desc.width,
        height: desc.height,
        present_mode: conv::map_present_mode(desc.presentMode),
    };
    let (id, error) =
        gfx_select!(device => GLOBAL.device_create_swap_chain(device, surface, &desc));
    if let Some(error) = error {
        panic!("Failed to create swapchain: {}", error);
    }

    id
}

#[no_mangle]
pub extern "C" fn wgpuSwapChainGetCurrentTextureView(
    swap_chain: id::SwapChainId,
) -> Option<id::TextureViewId> {
    gfx_select!(swap_chain => GLOBAL.swap_chain_get_current_texture_view(swap_chain, PhantomData))
        .expect("Unable to get swap chain texture view")
        .view_id
}

#[no_mangle]
pub extern "C" fn wgpuSwapChainPresent(swap_chain: id::SwapChainId) {
    //TODO: Header does not return swap chain status?
    gfx_select!(swap_chain => GLOBAL.swap_chain_present(swap_chain))
        .expect("Unable to present swap chain");
}

#[no_mangle]
pub extern "C" fn wgpuTextureCreateView(
    texture: id::TextureId,
    descriptor: &native::WGPUTextureViewDescriptor,
) -> id::TextureViewId {
    let desc = wgc::resource::TextureViewDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        format: conv::map_texture_format(descriptor.format),
        dimension: conv::map_texture_view_dimension(descriptor.dimension),
        aspect: conv::map_texture_aspect(descriptor.aspect),
        base_mip_level: descriptor.baseMipLevel,
        mip_level_count: NonZeroU32::new(descriptor.mipLevelCount),
        base_array_layer: descriptor.baseArrayLayer,
        array_layer_count: NonZeroU32::new(descriptor.arrayLayerCount),
    };

    check_error(gfx_select!(texture => GLOBAL.texture_create_view(texture, &desc, PhantomData)))
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateTexture(
    device: id::DeviceId,
    descriptor: &native::WGPUTextureDescriptor,
) -> id::TextureId {
    let desc = wgt::TextureDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        size: conv::map_extent3d(&descriptor.size),
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: conv::map_texture_dimension(descriptor.dimension),
        format: conv::map_texture_format(descriptor.format)
            .expect("Texture format must be provided"),
        usage: wgt::TextureUsage::from_bits(descriptor.usage).expect("Invalid texture usage"),
    };

    check_error(gfx_select!(device => GLOBAL.device_create_texture(device, &desc, PhantomData)))
}

#[no_mangle]
pub extern "C" fn wgpuTextureDestroy(texture_id: id::TextureId) {
    gfx_select!(texture_id => GLOBAL.texture_destroy(texture_id))
        .expect("Failed to destroy texture");
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateSampler(
    device: id::DeviceId,
    descriptor: &native::WGPUSamplerDescriptor,
) -> id::SamplerId {
    let desc = wgc::resource::SamplerDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        address_modes: [
            conv::map_address_mode(descriptor.addressModeU),
            conv::map_address_mode(descriptor.addressModeV),
            conv::map_address_mode(descriptor.addressModeW),
        ],
        mag_filter: conv::map_filter_mode(descriptor.magFilter),
        min_filter: conv::map_filter_mode(descriptor.minFilter),
        mipmap_filter: conv::map_filter_mode(descriptor.mipmapFilter),
        lod_min_clamp: descriptor.lodMinClamp,
        lod_max_clamp: descriptor.lodMaxClamp,
        compare: conv::map_compare_function(descriptor.compare).ok(),
        anisotropy_clamp: descriptor
            .maxAnisotropy
            .try_into()
            .ok()
            .and_then(|clamp| NonZeroU8::new(clamp)),
        border_color: None,
    };
    check_error(gfx_select!(device => GLOBAL.device_create_sampler(device, &desc, PhantomData)))
}

#[no_mangle]
pub extern "C" fn wgpuBufferUnmap(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub extern "C" fn wgpuSurface(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}
