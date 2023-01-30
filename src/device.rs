use crate::conv::{
    map_adapter_options, map_device_descriptor, map_pipeline_layout_descriptor, map_shader_module,
};
use crate::native::{
    unwrap_swap_chain_handle, Handle, IntoHandle, IntoHandleWithContext, UnwrapId,
};
use crate::{conv, follow_chain, handle_device_error, make_slice, native, OwnedLabel};
use std::{
    borrow::Cow,
    convert::TryInto,
    ffi::CString,
    num::{NonZeroU32, NonZeroU64, NonZeroU8},
    path::Path,
};
use thiserror::Error;
use wgc::gfx_select;

#[no_mangle]
pub unsafe extern "C" fn wgpuInstanceRequestAdapter(
    instance: native::WGPUInstance,
    options: Option<&native::WGPURequestAdapterOptions>,
    callback: native::WGPURequestAdapterCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let instance = instance.as_ref().expect("invalid instance");
    let context = &instance.context;

    let (desc, inputs) = match options {
        Some(options) => {
            let (compatible_surface, given_backend) = follow_chain!(
                map_adapter_options(options,
                WGPUSType_AdapterExtras => native::WGPUAdapterExtras)
            );

            (
                wgt::RequestAdapterOptions {
                    power_preference: match options.powerPreference {
                        native::WGPUPowerPreference_LowPower => wgt::PowerPreference::LowPower,
                        native::WGPUPowerPreference_HighPerformance => {
                            wgt::PowerPreference::HighPerformance
                        }
                        _ => wgt::PowerPreference::default(),
                    },
                    force_fallback_adapter: options.forceFallbackAdapter,
                    compatible_surface: compatible_surface.as_ref().map(|surface| surface.id),
                },
                wgc::instance::AdapterInputs::Mask(
                    match given_backend {
                        native::WGPUBackendType_Null => wgt::Backends::all(),
                        native::WGPUBackendType_Vulkan => wgt::Backends::VULKAN,
                        native::WGPUBackendType_Metal => wgt::Backends::METAL,
                        native::WGPUBackendType_D3D12 => wgt::Backends::DX12,
                        native::WGPUBackendType_D3D11 => wgt::Backends::DX11,
                        native::WGPUBackendType_OpenGL => wgt::Backends::GL,
                        _ => panic!("Invalid backend {}", given_backend),
                    },
                    |_| (),
                ),
            )
        }
        None => (
            wgt::RequestAdapterOptions::default(),
            wgc::instance::AdapterInputs::Mask(wgt::Backends::all(), |_| ()),
        ),
    };

    match context.request_adapter(&desc, inputs) {
        Ok(adapter) => {
            (callback.unwrap())(
                native::WGPURequestAdapterStatus_Success,
                native::WGPUAdapterImpl {
                    context: context.clone(),
                    id: adapter,
                    name: CString::default(),
                }
                .into_handle(),
                std::ptr::null(),
                userdata,
            );
        }
        Err(err) => {
            let message = CString::new(format!("{:?}", err)).unwrap();

            (callback.unwrap())(
                match err {
                    wgc::instance::RequestAdapterError::NotFound => {
                        native::WGPURequestAdapterStatus_Unavailable
                    }
                    wgc::instance::RequestAdapterError::InvalidSurface(_) => {
                        native::WGPURequestAdapterStatus_Error
                    }
                },
                std::ptr::null_mut(),
                message.as_ptr(),
                userdata,
            );
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: native::WGPUAdapter,
    descriptor: Option<&native::WGPUDeviceDescriptor>,
    callback: native::WGPURequestDeviceCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let (adapter, context) = adapter.unwrap_handle();

    let (desc, trace_str) = match descriptor {
        Some(descriptor) => follow_chain!(
            map_device_descriptor(descriptor,
            WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
        ),
        None => (wgt::DeviceDescriptor::default(), None),
    };

    let trace_path = trace_str.as_ref().map(Path::new);

    let (device, err) =
        gfx_select!(adapter => context.adapter_request_device(adapter, &desc, trace_path, ()));
    match err {
        None => {
            (callback.unwrap())(
                native::WGPURequestDeviceStatus_Success,
                device.into_handle_with_context(context),
                std::ptr::null(),
                userdata,
            );
        }
        Some(err) => {
            let message = CString::new(format!("{:?}", err)).unwrap();

            (callback.unwrap())(
                native::WGPURequestDeviceStatus_Error,
                std::ptr::null_mut(),
                message.as_ptr(),
                userdata,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetProperties(
    adapter: native::WGPUAdapter,
    properties: &mut native::WGPUAdapterProperties,
) {
    let adapter = adapter.as_mut().expect("invalid adapter");
    let context = &adapter.context;
    let id = adapter.id;

    let maybe_props = gfx_select!(id => context.adapter_get_info(id));
    if let Ok(props) = maybe_props {
        adapter.name = CString::new((&props.name) as &str).unwrap();

        properties.name = adapter.name.as_ptr();
        properties.vendorID = props.vendor as u32;
        properties.deviceID = props.device as u32;
        properties.adapterType = match props.device_type {
            wgt::DeviceType::Other => native::WGPUAdapterType_Unknown,
            wgt::DeviceType::IntegratedGpu => native::WGPUAdapterType_IntegratedGPU,
            wgt::DeviceType::DiscreteGpu => native::WGPUAdapterType_DiscreteGPU,
            wgt::DeviceType::VirtualGpu => native::WGPUAdapterType_CPU, // close enough?
            wgt::DeviceType::Cpu => native::WGPUAdapterType_CPU,
        };
        properties.backendType = match props.backend {
            wgt::Backend::Empty => native::WGPUBackendType_Null,
            wgt::Backend::Vulkan => native::WGPUBackendType_Vulkan,
            wgt::Backend::Metal => native::WGPUBackendType_Metal,
            wgt::Backend::Dx12 => native::WGPUBackendType_D3D12,
            wgt::Backend::Dx11 => native::WGPUBackendType_D3D11,
            wgt::Backend::Gl => native::WGPUBackendType_OpenGL,
            wgt::Backend::BrowserWebGpu => native::WGPUBackendType_OpenGLES, // close enough?
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetLimits(
    adapter: native::WGPUAdapter,
    limits: &mut native::WGPUSupportedLimits,
) -> bool {
    let (adapter, context) = adapter.unwrap_handle();

    let result = gfx_select!(adapter => context.adapter_limits(adapter));
    match result {
        Ok(wgt_limits) => write_limits_struct(wgt_limits, limits),
        _ => panic!("Calling wgpuAdapterGetLimits() on an invalid adapter."),
    }

    true // indicates that we can fill WGPUChainedStructOut
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterEnumerateFeatures(
    adapter: native::WGPUAdapter,
    features: *mut native::WGPUFeatureName,
) -> usize {
    let (adapter, context) = adapter.unwrap_handle();

    let adapter_features = match gfx_select!(adapter => context.adapter_features(adapter)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuAdapterEnumerateFeatures() on an invalid adapter."),
    };

    let temp = conv::features_to_native(adapter_features);

    if !features.is_null() {
        let out_slice = std::slice::from_raw_parts_mut(features, temp.len());
        out_slice.copy_from_slice(&temp);
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterHasFeature(
    adapter: native::WGPUAdapter,
    feature: native::WGPUFeatureName,
) -> bool {
    let (adapter, context) = adapter.unwrap_handle();

    let adapter_features = match gfx_select!(adapter => context.adapter_features(adapter)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuAdapterHasFeature() on an invalid adapter."),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false,
    };

    adapter_features.contains(feature)
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceEnumerateFeatures(
    device: native::WGPUDevice,
    features: *mut native::WGPUFeatureName,
) -> usize {
    let (device, context) = device.unwrap_handle();

    let device_features = match gfx_select!(device => context.device_features(device)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuDeviceEnumerateFeatures() on an invalid device."),
    };

    let temp = conv::features_to_native(device_features);

    if !features.is_null() {
        let out_slice = std::slice::from_raw_parts_mut(features, temp.len());
        out_slice.copy_from_slice(&temp);
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceHasFeature(
    device: native::WGPUDevice,
    feature: native::WGPUFeatureName,
) -> bool {
    let (device, context) = device.unwrap_handle();

    let device_features = match gfx_select!(device => context.device_features(device)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuDeviceHasFeature() on an invalid device."),
    };

    let feature = match conv::map_feature(feature) {
        Some(feature) => feature,
        None => return false,
    };

    device_features.contains(feature)
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetLimits(
    device: native::WGPUDevice,
    limits: &mut native::WGPUSupportedLimits,
) -> bool {
    let (device, context) = device.unwrap_handle();

    let result = gfx_select!(device => context.device_limits(device));
    match result {
        Ok(wgt_limits) => write_limits_struct(wgt_limits, limits),
        _ => panic!("Calling wgpuDeviceGetLimits() on an invalid device."),
    }

    true // indicates that we can fill WGPUChainedStructOut
}

fn write_limits_struct(
    wgt_limits: wgt::Limits,
    supported_limits: &mut native::WGPUSupportedLimits,
) {
    let mut limits = supported_limits.limits; // This makes a copy - we copy back at the end
    limits.maxTextureDimension1D = wgt_limits.max_texture_dimension_1d;
    limits.maxTextureDimension2D = wgt_limits.max_texture_dimension_2d;
    limits.maxTextureDimension3D = wgt_limits.max_texture_dimension_3d;
    limits.maxTextureArrayLayers = wgt_limits.max_texture_array_layers;
    limits.maxBindGroups = wgt_limits.max_bind_groups;
    limits.maxDynamicUniformBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout;
    limits.maxDynamicStorageBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout;
    limits.maxSampledTexturesPerShaderStage = wgt_limits.max_sampled_textures_per_shader_stage;
    limits.maxSamplersPerShaderStage = wgt_limits.max_samplers_per_shader_stage;
    limits.maxStorageBuffersPerShaderStage = wgt_limits.max_storage_buffers_per_shader_stage;
    limits.maxStorageTexturesPerShaderStage = wgt_limits.max_storage_textures_per_shader_stage;
    limits.maxUniformBuffersPerShaderStage = wgt_limits.max_uniform_buffers_per_shader_stage;
    limits.maxUniformBufferBindingSize = wgt_limits.max_uniform_buffer_binding_size as u64;
    limits.maxStorageBufferBindingSize = wgt_limits.max_storage_buffer_binding_size as u64;
    limits.minUniformBufferOffsetAlignment = wgt_limits.min_uniform_buffer_offset_alignment;
    limits.minStorageBufferOffsetAlignment = wgt_limits.min_storage_buffer_offset_alignment;
    limits.maxVertexBuffers = wgt_limits.max_vertex_buffers;
    limits.maxBufferSize = wgt_limits.max_buffer_size as u64;
    limits.maxVertexAttributes = wgt_limits.max_vertex_attributes;
    limits.maxVertexBufferArrayStride = wgt_limits.max_vertex_buffer_array_stride;
    limits.maxInterStageShaderComponents = wgt_limits.max_inter_stage_shader_components;
    limits.maxComputeWorkgroupStorageSize = wgt_limits.max_compute_workgroup_storage_size;
    limits.maxComputeInvocationsPerWorkgroup = wgt_limits.max_compute_invocations_per_workgroup;
    limits.maxComputeWorkgroupSizeX = wgt_limits.max_compute_workgroup_size_x;
    limits.maxComputeWorkgroupSizeY = wgt_limits.max_compute_workgroup_size_y;
    limits.maxComputeWorkgroupSizeZ = wgt_limits.max_compute_workgroup_size_z;
    limits.maxComputeWorkgroupsPerDimension = wgt_limits.max_compute_workgroups_per_dimension;
    supported_limits.limits = limits;

    if !supported_limits.nextInChain.is_null() {
        unsafe {
            let mut extras = std::mem::transmute::<
                *mut native::WGPUChainedStructOut,
                *mut native::WGPUSupportedLimitsExtras,
            >(supported_limits.nextInChain);

            (*extras).chain.next = std::ptr::null_mut();
            (*extras).chain.sType = native::WGPUSType_SupportedLimitsExtras;

            (*extras).maxPushConstantSize = wgt_limits.max_push_constant_size;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateShaderModule(
    device: native::WGPUDevice,
    descriptor: &native::WGPUShaderModuleDescriptor,
) -> native::WGPUShaderModule {
    let (device, context) = device.unwrap_handle();

    let label = OwnedLabel::new(descriptor.label);
    let source = follow_chain!(
        map_shader_module(descriptor,
        WGPUSType_ShaderModuleSPIRVDescriptor => native::WGPUShaderModuleSPIRVDescriptor,
        WGPUSType_ShaderModuleWGSLDescriptor => native::WGPUShaderModuleWGSLDescriptor,
        WGPUSType_ShaderModuleGLSLDescriptor => native::WGPUShaderModuleGLSLDescriptor)
    );

    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: label.as_cow(),
        shader_bound_checks: wgt::ShaderBoundChecks::default(),
    };
    let (id, error) =
        gfx_select!(device => context.device_create_shader_module(device, &desc, source, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBuffer(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBufferDescriptor,
) -> native::WGPUBuffer {
    let (device, context) = device.unwrap_handle();

    let usage = wgt::BufferUsages::from_bits(descriptor.usage).expect("Buffer Usage Invalid.");
    let label = OwnedLabel::new(descriptor.label);
    let (id, error) = gfx_select!(device => context.device_create_buffer(
        device,
        &wgt::BufferDescriptor {
            label: label.as_cow(),
            size: descriptor.size,
            usage,
            mapped_at_creation: descriptor.mappedAtCreation,
        },
        ()
    ));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDestroy(buffer: native::WGPUBuffer) {
    let (id, context) = buffer.unwrap_handle();
    gfx_select!(id => context.buffer_destroy(id)).expect("Unable to destroy buffer");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBindGroupLayoutDescriptor,
) -> native::WGPUBindGroupLayout {
    let (device, context) = device.unwrap_handle();

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
                native::WGPUSamplerBindingType_Filtering => {
                    wgt::BindingType::Sampler(wgt::SamplerBindingType::Filtering)
                }
                native::WGPUSamplerBindingType_NonFiltering => {
                    wgt::BindingType::Sampler(wgt::SamplerBindingType::NonFiltering)
                }
                native::WGPUSamplerBindingType_Comparison => {
                    wgt::BindingType::Sampler(wgt::SamplerBindingType::Comparison)
                }
                x => panic!("Unknown Sampler Type: {}", x),
            }
        } else if is_storage_texture {
            wgt::BindingType::StorageTexture {
                access: match entry.storageTexture.access {
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
                min_binding_size: match entry.buffer.minBindingSize {
                    0 => panic!("invalid minBindingSize"),
                    conv::WGPU_WHOLE_SIZE => None,
                    _ => Some(NonZeroU64::new_unchecked(entry.buffer.minBindingSize)),
                },
            }
        } else {
            panic!("No entry type specified.");
        };

        entries.push(wgt::BindGroupLayoutEntry {
            ty,
            binding: entry.binding,
            visibility: wgt::ShaderStages::from_bits(entry.visibility).unwrap(),
            count: None, // TODO - What is this?
        });
    }
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    let (id, error) =
        gfx_select!(device => context.device_create_bind_group_layout(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBindGroupDescriptor,
) -> native::WGPUBindGroup {
    let (device, context) = device.unwrap_handle();

    let mut entries = Vec::new();
    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let wgc_entry = if let Some(buffer) = entry.buffer.as_option() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Buffer(
                    wgc::binding_model::BufferBinding {
                        buffer_id: buffer,
                        offset: entry.offset,
                        size: match entry.size {
                            0 => panic!("invalid size"),
                            conv::WGPU_WHOLE_SIZE => None,
                            _ => Some(NonZeroU64::new_unchecked(entry.size)),
                        },
                    },
                ),
            }
        } else if let Some(sampler) = entry.sampler.as_option() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Sampler(sampler),
            }
        } else if let Some(texture_view) = entry.textureView.as_option() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::TextureView(texture_view),
            }
        } else {
            panic!("BindGroup entry does not have buffer nor sampler nor textureView.")
        };
        entries.push(wgc_entry);
    }

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: descriptor
            .layout
            .as_option()
            .expect("invalid bind group layout for bind group descriptor"),
        entries: Cow::Borrowed(&entries),
    };
    let (id, error) = gfx_select!(device => context.device_create_bind_group(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: native::WGPUDevice,
    descriptor: &native::WGPUPipelineLayoutDescriptor,
) -> native::WGPUPipelineLayout {
    let (device, context) = device.unwrap_handle();

    let desc = follow_chain!(
        map_pipeline_layout_descriptor(
            descriptor,
            WGPUSType_PipelineLayoutExtras => native::WGPUPipelineLayoutExtras)
    );
    let (id, error) =
        gfx_select!(device => context.device_create_pipeline_layout(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
    device: native::WGPUDevice,
    descriptor: &native::WGPUComputePipelineDescriptor,
) -> native::WGPUComputePipeline {
    let (device, context) = device.unwrap_handle();

    let stage = wgc::pipeline::ProgrammableStageDescriptor {
        module: descriptor
            .compute
            .module
            .as_option()
            .expect("invalid shader module for compute pipeline descriptor"),
        entry_point: OwnedLabel::new(descriptor.compute.entryPoint)
            .into_cow()
            .expect("Entry point not provided"),
    };
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_option(),
        stage,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (id, error) = gfx_select!(device => context.device_create_compute_pipeline(device, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUCommandEncoderDescriptor>,
) -> native::WGPUCommandEncoder {
    let (device, context) = device.unwrap_handle();

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandEncoderDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandEncoderDescriptor::default(),
    };
    let (id, error) =
        gfx_select!(device => context.device_create_command_encoder(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetQueue(device: native::WGPUDevice) -> native::WGPUQueue {
    device.as_ref().expect("invalid device");
    device
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) {
    let (queue, context) = queue.unwrap_handle();

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        let ptr = (*command_buffer) as native::WGPUCommandBuffer;
        // NOTE: Automaticaly drop the command buffer
        if ptr.is_null() {
            panic!("invalid command buffer");
        }
        let buffer_id = Box::from_raw(ptr).id;
        command_buffers.push(buffer_id)
    }

    gfx_select!(queue => context.queue_submit(queue, &command_buffers))
        .expect("Unable to submit queue");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmitForIndex(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) -> native::WGPUSubmissionIndex {
    let (queue, context) = queue.unwrap_handle();

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        let ptr = (*command_buffer) as native::WGPUCommandBuffer;
        let (id, _) = ptr.unwrap_handle();
        command_buffers.push(id)
    }

    let submission_index = gfx_select!(queue => context.queue_submit(queue, &command_buffers))
        .expect("Unable to submit queue");
    submission_index.index
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteBuffer(
    queue: native::WGPUQueue,
    buffer: native::WGPUBuffer,
    buffer_offset: u64,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
) {
    let (queue, _) = queue.unwrap_handle();
    let (buffer, context) = buffer.unwrap_handle();

    let slice = make_slice(data, data_size);
    gfx_select!(queue => context.queue_write_buffer(queue, buffer, buffer_offset, slice))
        .expect("Unable to write buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteTexture(
    queue: native::WGPUQueue,
    destination: &native::WGPUImageCopyTexture,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
    data_layout: &native::WGPUTextureDataLayout,
    write_size: &native::WGPUExtent3D,
) {
    let (queue, context) = queue.unwrap_handle();

    let slice = make_slice(data, data_size);
    gfx_select!(queue => context.queue_write_texture(
        queue,
        &conv::map_image_copy_texture(destination),
        slice,
        &conv::map_texture_data_layout(data_layout),
        &conv::map_extent3d(write_size)
    ))
    .expect("Unable to write texture")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferMapAsync(
    buffer: native::WGPUBuffer,
    mode: native::WGPUMapModeFlags,
    offset: usize,
    size: usize,
    callback: native::WGPUBufferMapCallback,
    user_data: *mut u8,
) {
    let (buffer, context) = buffer.unwrap_handle();

    let operation = wgc::resource::BufferMapOperation {
        host: match mode as native::WGPUMapMode {
            native::WGPUMapMode_Write => wgc::device::HostMap::Write,
            native::WGPUMapMode_Read => wgc::device::HostMap::Read,
            native::WGPUMapMode_None => panic!("Buffer map mode None is not supported."),
            x => panic!("Unknown map mode: {}", x),
        },
        callback: wgc::resource::BufferMapCallback::from_c(wgc::resource::BufferMapCallbackC {
            callback: std::mem::transmute(callback.expect("Callback cannot be null")),
            user_data,
        }),
    };

    gfx_select!(buffer => context.buffer_map_async(buffer, offset as u64 .. (offset + size) as u64, operation))
        .expect("Unable to map buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePoll(
    device: native::WGPUDevice,
    wait: bool,
    wrapped_submission_index: Option<&native::WGPUWrappedSubmissionIndex>,
) -> bool {
    let (device, context) = device.unwrap_handle();

    let maintain = match wait {
        true => match wrapped_submission_index {
            Some(index) => {
                wgt::Maintain::WaitForSubmissionIndex(wgc::device::queue::WrappedSubmissionIndex {
                    queue_id: index.queue.unwrap_handle().0,
                    index: index.submissionIndex,
                })
            }
            None => wgt::Maintain::Wait,
        },
        false => wgt::Maintain::Poll,
    };

    gfx_select!(device => context.device_poll(device, maintain)).expect("Unable to poll device")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut u8 {
    let (buffer, context) = buffer.unwrap_handle();

    gfx_select!(buffer => context.buffer_get_mapped_range(
        buffer,
        offset as u64,
        match size {
            conv::WGPU_WHOLE_MAP_SIZE => None,
            _ => Some(size as u64),
        }
    ))
    .expect("Unable to get mapped range")
    .0
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: native::WGPUDevice,
    descriptor: &native::WGPURenderPipelineDescriptor,
) -> native::WGPURenderPipeline {
    let (device, context) = device.unwrap_handle();

    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_option(),
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor
                    .vertex
                    .module
                    .as_option()
                    .expect("invalid vertex shader module for render pipeline descriptor"),
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
                        native::WGPUVertexStepMode_Vertex => wgt::VertexStepMode::Vertex,
                        native::WGPUVertexStepMode_Instance => wgt::VertexStepMode::Instance,
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
            unclipped_depth: false, // todo: fill this via extras
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
                    module: fragment
                        .module
                        .as_option()
                        .expect("invalid fragment shader module for render pipeline descriptor"),
                    entry_point: OwnedLabel::new(fragment.entryPoint)
                        .into_cow()
                        .expect("Entry point not provided"),
                },
                targets: Cow::Owned(
                    make_slice(fragment.targets, fragment.targetCount as usize)
                        .iter()
                        .map(|color_target| {
                            conv::map_texture_format(color_target.format).map(|format| {
                                wgt::ColorTargetState {
                                    format,
                                    blend: color_target.blend.as_ref().map(|blend| {
                                        wgt::BlendState {
                                            color: conv::map_blend_component(blend.color),
                                            alpha: conv::map_blend_component(blend.alpha),
                                        }
                                    }),
                                    write_mask: wgt::ColorWrites::from_bits(color_target.writeMask)
                                        .unwrap(),
                                }
                            })
                        })
                        .collect(),
                ),
            }),
        multiview: None,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (id, error) = gfx_select!(device => context.device_create_render_pipeline(device, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    pipeline: native::WGPURenderPipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (pipeline, context) = pipeline.unwrap_handle();

    let (id, error) = gfx_select!(pipeline => context.render_pipeline_get_bind_group_layout(pipeline, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the render pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get render pipeline bind group layout: {:?}",
            error
        );
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSwapChain(
    device: native::WGPUDevice,
    surface: native::WGPUSurface,
    descriptor: &native::WGPUSwapChainDescriptor,
) -> native::WGPUSwapChain {
    let (device, _) = device.unwrap_handle();
    let (surface, context) = surface.unwrap_handle();

    // The swap chain API of wgpu-core (and WebGPU) has been merged into the surface API,
    // so this gets a bit weird until the webgpu.h changes accordingly.
    let config = wgt::SurfaceConfiguration {
        usage: wgt::TextureUsages::from_bits(descriptor.usage).unwrap(),
        format: conv::map_texture_format(descriptor.format).expect("Texture format not defined"),
        width: descriptor.width,
        height: descriptor.height,
        present_mode: conv::map_present_mode(descriptor.presentMode),
        alpha_mode: wgt::CompositeAlphaMode::Auto,
    };
    let error = gfx_select!(device => context.surface_configure(surface, device, &config));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        native::WGPUSwapChainImpl {
            context: context.clone(),
            surface_id: surface,
            device_id: device,
        }
        .into_handle()
    }
}

#[derive(Debug, Error)]
pub enum SurfaceError {
    #[error("Surface timed out")]
    Timeout,
    #[error("Surface is outdated")]
    Outdated,
    #[error("Surface was lost")]
    Lost,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainGetCurrentTextureView(
    swap_chain: native::WGPUSwapChain,
) -> native::WGPUTextureView {
    let (surface, device, context) = unwrap_swap_chain_handle(swap_chain);

    match gfx_select!(device => context.surface_get_current_texture(surface, ())) {
        Err(error) => {
            handle_device_error(device, &error);
            std::ptr::null_mut()
        }
        Ok(result) => match result.status {
            wgt::SurfaceStatus::Good | wgt::SurfaceStatus::Suboptimal => {
                let texture = result.texture_id.unwrap();
                let desc = wgc::resource::TextureViewDescriptor::default();
                gfx_select!(texture => context.texture_create_view(texture, &desc, ()))
                    .0
                    .into_handle_with_context(context)
            }
            wgt::SurfaceStatus::Timeout => {
                handle_device_error(device, &SurfaceError::Timeout);
                std::ptr::null_mut()
            }
            wgt::SurfaceStatus::Outdated => {
                handle_device_error(device, &SurfaceError::Outdated);
                std::ptr::null_mut()
            }
            wgt::SurfaceStatus::Lost => {
                handle_device_error(device, &SurfaceError::Lost);
                std::ptr::null_mut()
            }
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainPresent(swap_chain: native::WGPUSwapChain) {
    let (surface, device, context) = unwrap_swap_chain_handle(swap_chain);
    gfx_select!(device => context.surface_present(surface)).expect("Unable to present swap chain");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureCreateView(
    texture: native::WGPUTexture,
    descriptor: Option<&native::WGPUTextureViewDescriptor>,
) -> native::WGPUTextureView {
    let (texture, context) = texture.unwrap_handle();

    let desc = match descriptor {
        Some(descriptor) => wgc::resource::TextureViewDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
            format: conv::map_texture_format(descriptor.format),
            dimension: conv::map_texture_view_dimension(descriptor.dimension),
            range: wgt::ImageSubresourceRange {
                aspect: conv::map_texture_aspect(descriptor.aspect),
                base_mip_level: descriptor.baseMipLevel,
                mip_level_count: match descriptor.mipLevelCount {
                    0 => panic!("invalid mipLevelCount"),
                    native::WGPU_MIP_LEVEL_COUNT_UNDEFINED => None,
                    _ => Some(NonZeroU32::new_unchecked(descriptor.mipLevelCount)),
                },
                base_array_layer: descriptor.baseArrayLayer,
                array_layer_count: match descriptor.arrayLayerCount {
                    0 => panic!("invalid arrayLayerCount"),
                    native::WGPU_ARRAY_LAYER_COUNT_UNDEFINED => None,
                    _ => Some(NonZeroU32::new_unchecked(descriptor.arrayLayerCount)),
                },
            },
        },
        None => wgc::resource::TextureViewDescriptor::default(),
    };

    let (id, error) = gfx_select!(texture => context.texture_create_view(texture, &desc, ()));

    if let Some(error) = error {
        // TODO: report via handle_device_error()
        log::error!("Failed to create texture view for texture: {:?}", error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateTexture(
    device: native::WGPUDevice,
    descriptor: &native::WGPUTextureDescriptor,
) -> native::WGPUTexture {
    let (device, context) = device.unwrap_handle();

    let desc = wgt::TextureDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        size: conv::map_extent3d(&descriptor.size),
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: conv::map_texture_dimension(descriptor.dimension),
        format: conv::map_texture_format(descriptor.format)
            .expect("Texture format must be provided"),
        usage: wgt::TextureUsages::from_bits(descriptor.usage).expect("Invalid texture usage"),
    };

    let (id, error) = gfx_select!(device => context.device_create_texture(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDestroy(texture: native::WGPUTexture) {
    let (id, context) = texture.unwrap_handle();
    gfx_select!(id => context.texture_destroy(id)).expect("Failed to destroy texture");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSampler(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUSamplerDescriptor>,
) -> native::WGPUSampler {
    let (device, context) = device.unwrap_handle();

    let desc = match descriptor {
        Some(descriptor) => wgc::resource::SamplerDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
            address_modes: [
                conv::map_address_mode(descriptor.addressModeU),
                conv::map_address_mode(descriptor.addressModeV),
                conv::map_address_mode(descriptor.addressModeW),
            ],
            mag_filter: conv::map_filter_mode(descriptor.magFilter),
            min_filter: conv::map_filter_mode(descriptor.minFilter),
            mipmap_filter: conv::map_mipmap_filter_mode(descriptor.mipmapFilter),
            lod_min_clamp: descriptor.lodMinClamp,
            lod_max_clamp: descriptor.lodMaxClamp,
            compare: conv::map_compare_function(descriptor.compare).ok(),
            anisotropy_clamp: descriptor
                .maxAnisotropy
                .try_into()
                .ok()
                .and_then(NonZeroU8::new),
            border_color: None,
        },
        None => wgc::resource::SamplerDescriptor::default(),
    };

    let (id, error) = gfx_select!(device => context.device_create_sampler(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderBundleEncoder(
    device: native::WGPUDevice,
    descriptor: &native::WGPURenderBundleEncoderDescriptor,
) -> native::WGPURenderBundleEncoder {
    let (device, context) = device.unwrap_handle();

    let desc = wgc::command::RenderBundleEncoderDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        color_formats: unsafe {
            make_slice(
                descriptor.colorFormats,
                descriptor.colorFormatsCount as usize,
            )
        }
        .iter()
        .map(|format| conv::map_texture_format(*format))
        .collect(),
        depth_stencil: conv::map_texture_format(descriptor.depthStencilFormat).map(|format| {
            wgt::RenderBundleDepthStencil {
                format,
                depth_read_only: descriptor.depthReadOnly,
                stencil_read_only: descriptor.stencilReadOnly,
            }
        }),
        sample_count: descriptor.sampleCount,
        multiview: None,
    };

    match wgc::command::RenderBundleEncoder::new(&desc, device, None) {
        Ok(encoder) => native::WGPURenderBundleEncoderImpl {
            context: context.clone(),
            encoder,
        }
        .into_handle(),
        Err(error) => {
            handle_device_error(device, &error);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceDestroy(_device: native::WGPUDevice) {
    // Empty implementation, maybe call drop?
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineGetBindGroupLayout(
    pipeline: native::WGPUComputePipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (pipeline, context) = pipeline.unwrap_handle();

    let (id, error) = gfx_select!(pipeline => context.compute_pipeline_get_bind_group_layout(pipeline, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the compute pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get compute pipeline bind group layout: {:?}",
            error
        );
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferUnmap(buffer: native::WGPUBuffer) {
    let (id, context) = buffer.unwrap_handle();
    gfx_select!(id => context.buffer_unmap(id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDrop(buffer: native::WGPUBuffer) {
    let (id, context) = buffer.unwrap_handle();
    gfx_select!(id => context.buffer_drop(id, false));
    buffer.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDrop(texture: native::WGPUTexture) {
    let (id, context) = texture.unwrap_handle();
    gfx_select!(id => context.texture_drop(id, false));
    texture.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureViewDrop(texture_view: native::WGPUTextureView) {
    let (id, context) = texture_view.unwrap_handle();
    gfx_select!(id => context.texture_view_drop(id, false)).expect("Unable to drop texture view");
    texture_view.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSamplerDrop(sampler: native::WGPUSampler) {
    let (id, context) = sampler.unwrap_handle();
    gfx_select!(id => context.sampler_drop(id));
    sampler.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupLayoutDrop(bind_group_layout: native::WGPUBindGroupLayout) {
    let (id, context) = bind_group_layout.unwrap_handle();
    gfx_select!(id => context.bind_group_layout_drop(id));
    bind_group_layout.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuPipelineLayoutDrop(pipeline_layout: native::WGPUPipelineLayout) {
    let (id, context) = pipeline_layout.unwrap_handle();
    gfx_select!(id => context.pipeline_layout_drop(id));
    pipeline_layout.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupDrop(bind_group: native::WGPUBindGroup) {
    let (id, context) = bind_group.unwrap_handle();
    gfx_select!(id => context.bind_group_drop(id));
    bind_group.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuShaderModuleDrop(shader_module: native::WGPUShaderModule) {
    let (id, context) = shader_module.unwrap_handle();
    gfx_select!(id => context.shader_module_drop(id));
    shader_module.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderDrop(command_encoder: native::WGPUCommandEncoder) {
    let (id, context) = {
        let command_encoder_impl = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder_impl.id, &command_encoder_impl.context)
    };
    gfx_select!(id => context.command_encoder_drop(id));
    command_encoder.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrop(
    render_pass_encoder: native::WGPURenderPassEncoder,
) {
    render_pass_encoder.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDrop(
    compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    compute_pass_encoder.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrop(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    render_bundle_encoder.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandBufferDrop(command_buffer: native::WGPUCommandBuffer) {
    let (id, context) = command_buffer.unwrap_handle();
    gfx_select!(id => context.command_buffer_drop(id));
    command_buffer.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleDrop(render_bundle: native::WGPURenderBundle) {
    let (id, context) = render_bundle.unwrap_handle();
    gfx_select!(id => context.render_bundle_drop(id));
    render_bundle.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetDrop(query_set: native::WGPUQuerySet) {
    let (id, context) = query_set.unwrap_handle();
    gfx_select!(id => context.query_set_drop(id));
    query_set.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineDrop(render_pipeline: native::WGPURenderPipeline) {
    let (id, context) = render_pipeline.unwrap_handle();
    gfx_select!(id => context.render_pipeline_drop(id));
    render_pipeline.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineDrop(compute_pipeline: native::WGPUComputePipeline) {
    let (id, context) = compute_pipeline.unwrap_handle();
    gfx_select!(id => context.compute_pipeline_drop(id));
    compute_pipeline.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceDrop(device: native::WGPUDevice) {
    let (id, context) = device.unwrap_handle();
    gfx_select!(id => context.device_drop(id));
    device.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterDrop(adapter: native::WGPUAdapter) {
    let (id, context) = adapter.unwrap_handle();
    gfx_select!(id => context.adapter_drop(id));
    adapter.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceDrop(surface: native::WGPUSurface) {
    let (id, context) = surface.unwrap_handle();
    context.surface_drop(id);
    surface.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainDrop(swap_chain: native::WGPUSwapChain) {
    swap_chain.drop()
}
