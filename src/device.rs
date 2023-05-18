use crate::conv::{
    map_adapter_options, map_device_descriptor, map_pipeline_layout_descriptor, map_shader_module,
    map_swapchain_descriptor,
};
use crate::utils::{make_slice, OwnedLabel};
use crate::{conv, follow_chain, handle_device_error, native};
use std::{borrow::Cow, ffi::CString, num::NonZeroU64, path::Path};
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
    let callback = callback.expect("invalid callback");

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
                        _ => panic!("Invalid backend {given_backend}"),
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
        Ok(adapter_id) => {
            callback(
                native::WGPURequestAdapterStatus_Success,
                Box::into_raw(Box::new(native::WGPUAdapterImpl {
                    context: context.clone(),
                    id: adapter_id,
                    name: CString::default(),
                    vendor_name: CString::default(),
                    architecture_name: CString::default(),
                    driver_desc: CString::default(),
                })),
                std::ptr::null(),
                userdata,
            );
        }
        Err(err) => {
            let message = CString::new(format!("{err:?}")).unwrap();

            callback(
                match err {
                    wgc::instance::RequestAdapterError::NotFound => {
                        native::WGPURequestAdapterStatus_Unavailable
                    }
                    wgc::instance::RequestAdapterError::InvalidSurface(_) => {
                        native::WGPURequestAdapterStatus_Error
                    }
                    _ => native::WGPURequestAdapterStatus_Unknown,
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
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let callback = callback.expect("invalid callback");

    let (desc, trace_str) = match descriptor {
        Some(descriptor) => follow_chain!(
            map_device_descriptor(descriptor,
            WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
        ),
        None => (wgt::DeviceDescriptor::default(), None),
    };

    let trace_path = trace_str.as_ref().map(Path::new);

    let (device_id, err) = gfx_select!(adapter_id => context.adapter_request_device(adapter_id, &desc, trace_path, ()));
    match err {
        None => {
            callback(
                native::WGPURequestDeviceStatus_Success,
                Box::into_raw(Box::new(native::WGPUDeviceImpl {
                    context: context.clone(),
                    id: device_id,
                })),
                std::ptr::null(),
                userdata,
            );
        }
        Some(err) => {
            let message = CString::new(format!("{err:?}")).unwrap();

            callback(
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
    properties: Option<&mut native::WGPUAdapterProperties>,
) {
    let adapter = adapter.as_mut().expect("invalid adapter");
    let properties = properties.expect("invalid return pointer \"properties\"");
    let context = &adapter.context;
    let id = adapter.id;

    let maybe_props = gfx_select!(id => context.adapter_get_info(id));
    if let Ok(props) = maybe_props {
        adapter.name = CString::new((&props.name) as &str).unwrap();
        let driver_desc = format!("{} {}", props.driver, props.driver_info);
        adapter.driver_desc = CString::new(driver_desc.trim()).unwrap();

        properties.vendorID = props.vendor as u32;
        properties.vendorName = adapter.vendor_name.as_ptr();
        properties.architecture = adapter.architecture_name.as_ptr();
        properties.deviceID = props.device as u32;
        properties.name = adapter.name.as_ptr();
        properties.driverDescription = adapter.driver_desc.as_ptr();
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
    limits: Option<&mut native::WGPUSupportedLimits>,
) -> bool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(adapter_id => context.adapter_limits(adapter_id));
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
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let adapter_features = match gfx_select!(adapter_id => context.adapter_features(adapter_id)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuAdapterEnumerateFeatures() on an invalid adapter."),
    };

    let temp = conv::features_to_native(adapter_features);

    if !features.is_null() {
        std::ptr::copy_nonoverlapping(temp.as_ptr(), features, temp.len());
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterHasFeature(
    adapter: native::WGPUAdapter,
    feature: native::WGPUFeatureName,
) -> bool {
    let (adapter_id, context) = {
        let adapter = adapter.as_ref().expect("invalid adapter");
        (adapter.id, &adapter.context)
    };
    let adapter_features = match gfx_select!(adapter_id => context.adapter_features(adapter_id)) {
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
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let device_features = match gfx_select!(device_id => context.device_features(device_id)) {
        Ok(features) => features,
        _ => panic!("Calling wgpuDeviceEnumerateFeatures() on an invalid device."),
    };

    let temp = conv::features_to_native(device_features);

    if !features.is_null() {
        std::ptr::copy_nonoverlapping(temp.as_ptr(), features, temp.len());
    }

    temp.len()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceHasFeature(
    device: native::WGPUDevice,
    feature: native::WGPUFeatureName,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let device_features = match gfx_select!(device_id => context.device_features(device_id)) {
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
    limits: Option<&mut native::WGPUSupportedLimits>,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let limits = limits.expect("invalid return pointer \"limits\"");

    let result = gfx_select!(device_id => context.device_limits(device_id));
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
    limits.maxBufferSize = wgt_limits.max_buffer_size;
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
    descriptor: Option<&native::WGPUShaderModuleDescriptor>,
) -> native::WGPUShaderModule {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

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
    let (shader_module_id, error) =
        gfx_select!(device_id => context.device_create_shader_module(device_id, &desc, source, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUShaderModuleImpl {
            context: context.clone(),
            id: shader_module_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBuffer(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBufferDescriptor>,
) -> native::WGPUBuffer {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgt::BufferDescriptor {
        label: label.as_cow(),
        size: descriptor.size,
        usage: wgt::BufferUsages::from_bits(descriptor.usage).expect("invalid buffer usage"),
        mapped_at_creation: descriptor.mappedAtCreation,
    };

    let (buffer_id, error) =
        gfx_select!(device_id => context.device_create_buffer(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUBufferImpl {
            context: context.clone(),
            id: buffer_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDestroy(buffer: native::WGPUBuffer) {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };
    gfx_select!(buffer_id => context.buffer_destroy(buffer_id)).expect("Unable to destroy buffer");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBindGroupLayoutDescriptor>,
) -> native::WGPUBindGroupLayout {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

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
                    x => panic!("Unknown texture SampleType: {x}"),
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
                    x => panic!("Unknown texture ViewDimension: {x}"),
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
                x => panic!("Unknown Sampler Type: {x}"),
            }
        } else if is_storage_texture {
            wgt::BindingType::StorageTexture {
                access: match entry.storageTexture.access {
                    native::WGPUStorageTextureAccess_WriteOnly => {
                        wgt::StorageTextureAccess::WriteOnly
                    }
                    x => panic!("Unknown StorageTextureAccess: {x}"),
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
                    x => panic!("Unknown texture ViewDimension: {x}"),
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
                    x => panic!("Unknown Buffer Type: {x}"),
                },
                has_dynamic_offset: entry.buffer.hasDynamicOffset,
                min_binding_size: {
                    assert_ne!(
                        entry.buffer.minBindingSize,
                        conv::WGPU_WHOLE_SIZE,
                        "invalid minBindingSize, use 0 instead"
                    );

                    NonZeroU64::new(entry.buffer.minBindingSize)
                },
            }
        } else {
            panic!("No entry type specified.");
        };

        entries.push(wgt::BindGroupLayoutEntry {
            ty,
            binding: entry.binding,
            visibility: wgt::ShaderStages::from_bits(entry.visibility)
                .expect("invalid visibility for bind group layout entry"),
            count: None, // TODO - What is this?
        });
    }
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_layout_id, error) =
        gfx_select!(device_id => context.device_create_bind_group_layout(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUBindGroupLayoutImpl {
            context: context.clone(),
            id: bind_group_layout_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUBindGroupDescriptor>,
) -> native::WGPUBindGroup {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");
    let bind_group_layout_id = descriptor
        .layout
        .as_ref()
        .expect("invalid bind group layout for bind group descriptor")
        .id;

    let mut entries = Vec::new();
    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let wgc_entry = if let Some(buffer) = entry.buffer.as_ref() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Buffer(
                    wgc::binding_model::BufferBinding {
                        buffer_id: buffer.id,
                        offset: entry.offset,
                        size: match entry.size {
                            0 => panic!("invalid size"),
                            conv::WGPU_WHOLE_SIZE => None,
                            _ => Some(NonZeroU64::new_unchecked(entry.size)),
                        },
                    },
                ),
            }
        } else if let Some(sampler) = entry.sampler.as_ref() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Sampler(sampler.id),
            }
        } else if let Some(texture_view) = entry.textureView.as_ref() {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::TextureView(texture_view.id),
            }
        } else {
            panic!("BindGroup entry does not have buffer nor sampler nor textureView.")
        };
        entries.push(wgc_entry);
    }

    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: bind_group_layout_id,
        entries: Cow::Borrowed(&entries),
    };
    let (bind_group_id, error) =
        gfx_select!(device_id => context.device_create_bind_group(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUBindGroupImpl {
            context: context.clone(),
            id: bind_group_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUPipelineLayoutDescriptor>,
) -> native::WGPUPipelineLayout {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = follow_chain!(
        map_pipeline_layout_descriptor(
            descriptor,
            WGPUSType_PipelineLayoutExtras => native::WGPUPipelineLayoutExtras)
    );
    let (pipeline_layout_id, error) =
        gfx_select!(device_id => context.device_create_pipeline_layout(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUPipelineLayoutImpl {
            context: context.clone(),
            id: pipeline_layout_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUComputePipelineDescriptor>,
) -> native::WGPUComputePipeline {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let stage = wgc::pipeline::ProgrammableStageDescriptor {
        module: descriptor
            .compute
            .module
            .as_ref()
            .expect("invalid shader module for compute pipeline descriptor")
            .id,
        entry_point: OwnedLabel::new(descriptor.compute.entryPoint)
            .into_cow()
            .expect("invalid entry point for compute pipeline descriptor"),
    };
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        stage,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (compute_pipeline_id, error) = gfx_select!(device_id => context.device_create_compute_pipeline(device_id, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUComputePipelineImpl {
            context: context.clone(),
            id: compute_pipeline_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUCommandEncoderDescriptor>,
) -> native::WGPUCommandEncoder {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let desc = match descriptor {
        Some(descriptor) => wgt::CommandEncoderDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandEncoderDescriptor::default(),
    };
    let (commnad_buffer_id, error) =
        gfx_select!(device_id => context.device_create_command_encoder(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUCommandBufferImpl {
            context: context.clone(),
            id: commnad_buffer_id,
        }))
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
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        let ptr = *command_buffer;
        assert!(!ptr.is_null(), "invalid command buffer");

        // NOTE: Automaticaly drop the command buffer
        let buffer_id = Box::from_raw(ptr).id;
        command_buffers.push(buffer_id)
    }

    gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers))
        .expect("Unable to submit queue");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmitForIndex(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) -> native::WGPUSubmissionIndex {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        let ptr = *command_buffer;
        assert!(!ptr.is_null(), "invalid command buffer");

        // NOTE: Automaticaly drop the command buffer
        let buffer_id = Box::from_raw(ptr).id;
        command_buffers.push(buffer_id)
    }

    let submission_index =
        gfx_select!(queue_id => context.queue_submit(queue_id, &command_buffers))
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
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    let slice = make_slice(data, data_size);
    gfx_select!(queue_id => context.queue_write_buffer(queue_id, buffer_id, buffer_offset, slice))
        .expect("Unable to write buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueWriteTexture(
    queue: native::WGPUQueue,
    destination: Option<&native::WGPUImageCopyTexture>,
    data: *const u8, // TODO: Check - this might not follow the header
    data_size: usize,
    data_layout: Option<&native::WGPUTextureDataLayout>,
    write_size: Option<&native::WGPUExtent3D>,
) {
    let (queue_id, context) = {
        let queue = queue.as_ref().expect("invalid queue");
        (queue.id, &queue.context)
    };

    let slice = make_slice(data, data_size);
    gfx_select!(queue_id => context.queue_write_texture(
        queue_id,
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        slice,
        &conv::map_texture_data_layout(data_layout.expect("invalid data layout")),
        &conv::map_extent3d(write_size.expect("invalid write size"))
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
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };

    let operation = wgc::resource::BufferMapOperation {
        host: match mode as native::WGPUMapMode {
            native::WGPUMapMode_Write => wgc::device::HostMap::Write,
            native::WGPUMapMode_Read => wgc::device::HostMap::Read,
            native::WGPUMapMode_None => panic!("Buffer map mode None is not supported."),
            x => panic!("Unknown map mode: {x}"),
        },
        callback: wgc::resource::BufferMapCallback::from_c(wgc::resource::BufferMapCallbackC {
            callback: std::mem::transmute(callback.expect("Callback cannot be null")),
            user_data,
        }),
    };

    gfx_select!(buffer_id => context.buffer_map_async(buffer_id, offset as u64 .. (offset + size) as u64, operation))
        .expect("Unable to map buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePoll(
    device: native::WGPUDevice,
    wait: bool,
    wrapped_submission_index: Option<&native::WGPUWrappedSubmissionIndex>,
) -> bool {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };

    let maintain = match wait {
        true => match wrapped_submission_index {
            Some(index) => {
                wgt::Maintain::WaitForSubmissionIndex(wgc::device::queue::WrappedSubmissionIndex {
                    queue_id: index
                        .queue
                        .as_ref()
                        .expect("invalid queue for wrapped submission index")
                        .id,
                    index: index.submissionIndex,
                })
            }
            None => wgt::Maintain::Wait,
        },
        false => wgt::Maintain::Poll,
    };

    gfx_select!(device_id => context.device_poll(device_id, maintain))
        .expect("Unable to poll device")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut ::std::os::raw::c_void {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };

    let (buf, _) = gfx_select!(buffer_id => context.buffer_get_mapped_range(
        buffer_id,
        offset as u64,
        match size {
            conv::WGPU_WHOLE_MAP_SIZE => None,
            _ => Some(size as u64),
        }
    ))
    .expect("Unable to get mapped range");

    buf as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetConstMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *const ::std::os::raw::c_void {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };

    let (buf, _) = gfx_select!(buffer_id => context.buffer_get_mapped_range(
        buffer_id,
        offset as u64,
        match size {
            conv::WGPU_WHOLE_MAP_SIZE => None,
            _ => Some(size as u64),
        }
    ))
    .expect("Unable to get mapped range");

    buf as *const ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPURenderPipelineDescriptor>,
) -> native::WGPURenderPipeline {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout.as_ref().map(|v| v.id),
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor
                    .vertex
                    .module
                    .as_ref()
                    .expect("invalid vertex shader module for render pipeline descriptor")
                    .id,
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
                        x => panic!("Unknown step mode {x}"),
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
                    .expect("invalid texture format for depth stencil state"),
                depth_write_enabled: desc.depthWriteEnabled,
                depth_compare: conv::map_compare_function(desc.depthCompare)
                    .expect("invalid depth compare function for depth stencil state"),
                stencil: wgt::StencilState {
                    front: conv::map_stencil_face_state(desc.stencilFront, "front"),
                    back: conv::map_stencil_face_state(desc.stencilBack, "back"),
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
                        .as_ref()
                        .expect("invalid fragment shader module for render pipeline descriptor")
                        .id,
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

    let (render_pipeline_id, error) = gfx_select!(device_id => context.device_create_render_pipeline(device_id, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPURenderPipelineImpl {
            context: context.clone(),
            id: render_pipeline_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    render_pipeline: native::WGPURenderPipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let (render_pipeline_id, context) = {
        let render_pipeline = render_pipeline.as_ref().expect("invalid render pipeline");
        (render_pipeline.id, &render_pipeline.context)
    };

    let (bind_group_layout_id, error) = gfx_select!(render_pipeline_id => context.render_pipeline_get_bind_group_layout(render_pipeline_id, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the render pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get render pipeline bind group layout: {:?}",
            error
        );
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUBindGroupLayoutImpl {
            context: context.clone(),
            id: bind_group_layout_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSwapChain(
    device: native::WGPUDevice,
    surface: native::WGPUSurface,
    descriptor: Option<&native::WGPUSwapChainDescriptor>,
) -> native::WGPUSwapChain {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let surface_id = surface.as_ref().expect("invalid surface").id;

    let config = follow_chain!(
        map_swapchain_descriptor(
            descriptor.expect("invalid descriptor"),
            WGPUSType_SwapChainDescriptorExtras => native::WGPUSwapChainDescriptorExtras)
    );

    let error = gfx_select!(device_id => context.surface_configure(surface_id, device_id, &config));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUSwapChainImpl {
            context: context.clone(),
            surface_id,
            device_id,
        }))
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
    let (surface_id, device_id, context) = {
        let swap_chain = swap_chain.as_ref().expect("invalid swap chain");
        (
            swap_chain.surface_id,
            swap_chain.device_id,
            &swap_chain.context,
        )
    };

    match gfx_select!(device_id => context.surface_get_current_texture(surface_id, ())) {
        Err(error) => {
            handle_device_error(device_id, &error);
            std::ptr::null_mut()
        }
        Ok(result) => match result.status {
            wgt::SurfaceStatus::Good | wgt::SurfaceStatus::Suboptimal => {
                let texture = result.texture_id.unwrap();
                let desc = wgc::resource::TextureViewDescriptor::default();
                let (texture_view_id, _) =
                    gfx_select!(texture => context.texture_create_view(texture, &desc, ()));

                Box::into_raw(Box::new(native::WGPUTextureViewImpl {
                    context: context.clone(),
                    id: texture_view_id,
                }))
            }
            wgt::SurfaceStatus::Timeout => {
                handle_device_error(device_id, &SurfaceError::Timeout);
                std::ptr::null_mut()
            }
            wgt::SurfaceStatus::Outdated => {
                handle_device_error(device_id, &SurfaceError::Outdated);
                std::ptr::null_mut()
            }
            wgt::SurfaceStatus::Lost => {
                handle_device_error(device_id, &SurfaceError::Lost);
                std::ptr::null_mut()
            }
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainPresent(swap_chain: native::WGPUSwapChain) {
    let (surface_id, device_id, context) = {
        let swap_chain = swap_chain.as_ref().expect("invalid swap chain");
        (
            swap_chain.surface_id,
            swap_chain.device_id,
            &swap_chain.context,
        )
    };
    gfx_select!(device_id => context.surface_present(surface_id))
        .expect("Unable to present swap chain");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureCreateView(
    texture: native::WGPUTexture,
    descriptor: Option<&native::WGPUTextureViewDescriptor>,
) -> native::WGPUTextureView {
    let (texture_id, context) = {
        let texture = texture.as_ref().expect("invalid texture");
        (texture.id, &texture.context)
    };

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
                    _ => Some(descriptor.mipLevelCount),
                },
                base_array_layer: descriptor.baseArrayLayer,
                array_layer_count: match descriptor.arrayLayerCount {
                    0 => panic!("invalid arrayLayerCount"),
                    native::WGPU_ARRAY_LAYER_COUNT_UNDEFINED => None,
                    _ => Some(descriptor.arrayLayerCount),
                },
            },
        },
        None => wgc::resource::TextureViewDescriptor::default(),
    };

    let (texture_view_id, error) =
        gfx_select!(texture_id => context.texture_create_view(texture_id, &desc, ()));

    if let Some(error) = error {
        // TODO: report via handle_device_error()
        log::error!("Failed to create texture view for texture: {:?}", error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUTextureViewImpl {
            context: context.clone(),
            id: texture_view_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateTexture(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUTextureDescriptor>,
) -> native::WGPUTexture {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let desc = wgt::TextureDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        size: conv::map_extent3d(&descriptor.size),
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: conv::map_texture_dimension(descriptor.dimension),
        format: conv::map_texture_format(descriptor.format)
            .expect("invalid texture format for texture descriptor"),
        usage: wgt::TextureUsages::from_bits(descriptor.usage)
            .expect("invalid texture usage for texture descriptor"),
        view_formats: make_slice(descriptor.viewFormats, descriptor.viewFormatCount as usize)
            .iter()
            .map(|v| {
                conv::map_texture_format(*v).expect("invalid view format for texture descriptor")
            })
            .collect(),
    };

    let (texture_id, error) =
        gfx_select!(device_id => context.device_create_texture(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUTextureImpl {
            context: context.clone(),
            id: texture_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDestroy(texture: native::WGPUTexture) {
    let (texture_id, context) = {
        let texture = texture.as_ref().expect("invalid texture");
        (texture.id, &texture.context)
    };
    gfx_select!(texture_id => context.texture_destroy(texture_id))
        .expect("Failed to destroy texture");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSampler(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUSamplerDescriptor>,
) -> native::WGPUSampler {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };

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
            anisotropy_clamp: descriptor.maxAnisotropy,
            border_color: None,
        },
        // wgpu-core doesn't have Default implementation for SamplerDescriptor,
        // use defaults from spec.
        // ref: https://gpuweb.github.io/gpuweb/#GPUSamplerDescriptor
        None => wgc::resource::SamplerDescriptor {
            label: None,
            address_modes: [
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
            ],
            mag_filter: wgt::FilterMode::Nearest,
            min_filter: wgt::FilterMode::Nearest,
            mipmap_filter: wgt::FilterMode::Nearest,
            lod_min_clamp: 0f32,
            lod_max_clamp: 32f32,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        },
    };

    let (sampler_id, error) =
        gfx_select!(device_id => context.device_create_sampler(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUSamplerImpl {
            context: context.clone(),
            id: sampler_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderBundleEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPURenderBundleEncoderDescriptor>,
) -> native::WGPURenderBundleEncoder {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

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

    match wgc::command::RenderBundleEncoder::new(&desc, device_id, None) {
        Ok(encoder) => Box::into_raw(Box::new(native::WGPURenderBundleEncoderImpl {
            context: context.clone(),
            encoder,
        })),
        Err(error) => {
            handle_device_error(device_id, &error);
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
    let (pipeline_id, context) = {
        let pipeline = pipeline.as_ref().expect("invalid pipeline");
        (pipeline.id, &pipeline.context)
    };

    let (bind_group_layout_id, error) = gfx_select!(pipeline_id => context.compute_pipeline_get_bind_group_layout(pipeline_id, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the compute pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get compute pipeline bind group layout: {:?}",
            error
        );
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUBindGroupLayoutImpl {
            context: context.clone(),
            id: bind_group_layout_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateQuerySet(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUQuerySetDescriptor>,
) -> native::WGPUQuerySet {
    let (device_id, context) = {
        let device = device.as_ref().expect("invalid device");
        (device.id, &device.context)
    };

    let desc = conv::map_query_set_descriptor(descriptor.expect("invalid query set descriptor"));

    let (query_set_id, error) =
        gfx_select!(device_id => context.device_create_query_set(device_id, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUQuerySetImpl {
            context: context.clone(),
            id: query_set_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferUnmap(buffer: native::WGPUBuffer) {
    let (buffer_id, context) = {
        let buffer = buffer.as_ref().expect("invalid buffer");
        (buffer.id, &buffer.context)
    };
    gfx_select!(buffer_id => context.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferDrop(buffer: native::WGPUBuffer) {
    assert!(!buffer.is_null(), "invalid buffer");
    let buffer = Box::from_raw(buffer);
    let context = &buffer.context;

    gfx_select!(buffer.id => context.buffer_drop(buffer.id, false));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureDrop(texture: native::WGPUTexture) {
    assert!(!texture.is_null(), "invalid texture");
    let texture = Box::from_raw(texture);
    let context = &texture.context;

    gfx_select!(texture.id => context.texture_drop(texture.id, false));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureViewDrop(texture_view: native::WGPUTextureView) {
    assert!(!texture_view.is_null(), "invalid texture view");
    let texture_view = Box::from_raw(texture_view);
    let context = &texture_view.context;

    gfx_select!(texture_view.id => context.texture_view_drop(texture_view.id, false))
        .expect("Unable to drop texture view");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSamplerDrop(sampler: native::WGPUSampler) {
    assert!(!sampler.is_null(), "invalid sampler");
    let sampler = Box::from_raw(sampler);
    let context = &sampler.context;

    gfx_select!(sampler.id => context.sampler_drop(sampler.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupLayoutDrop(bind_group_layout: native::WGPUBindGroupLayout) {
    assert!(!bind_group_layout.is_null(), "invalid bind group layout");
    let bind_group_layout = Box::from_raw(bind_group_layout);
    let context = &bind_group_layout.context;

    gfx_select!(bind_group_layout.id => context.bind_group_layout_drop(bind_group_layout.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuPipelineLayoutDrop(pipeline_layout: native::WGPUPipelineLayout) {
    assert!(!pipeline_layout.is_null(), "invalid pipeline layout");
    let pipeline_layout = Box::from_raw(pipeline_layout);
    let context = &pipeline_layout.context;

    gfx_select!(pipeline_layout.id => context.pipeline_layout_drop(pipeline_layout.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBindGroupDrop(bind_group: native::WGPUBindGroup) {
    assert!(!bind_group.is_null(), "invalid bind group");
    let bind_group = Box::from_raw(bind_group);
    let context = &bind_group.context;

    gfx_select!(bind_group.id => context.bind_group_drop(bind_group.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuShaderModuleDrop(shader_module: native::WGPUShaderModule) {
    assert!(!shader_module.is_null(), "invalid shader module");
    let shader_module = Box::from_raw(shader_module);
    let context = &shader_module.context;

    gfx_select!(shader_module.id => context.shader_module_drop(shader_module.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderDrop(command_encoder: native::WGPUCommandEncoder) {
    assert!(!command_encoder.is_null(), "invalid command encoder");
    let command_encoder = Box::from_raw(command_encoder);
    let context = &command_encoder.context;

    gfx_select!(command_encoder.id => context.command_encoder_drop(command_encoder.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrop(
    render_pass_encoder: native::WGPURenderPassEncoder,
) {
    assert!(
        !render_pass_encoder.is_null(),
        "invalid render pass encoder"
    );
    drop(Box::from_raw(render_pass_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDrop(
    compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    assert!(
        !compute_pass_encoder.is_null(),
        "invalid compute pass encoder"
    );
    drop(Box::from_raw(compute_pass_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrop(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    assert!(
        !render_bundle_encoder.is_null(),
        "invalid render bundle encoder"
    );
    drop(Box::from_raw(render_bundle_encoder));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandBufferDrop(command_buffer: native::WGPUCommandBuffer) {
    assert!(!command_buffer.is_null(), "invalid command buffer");
    let command_buffer = Box::from_raw(command_buffer);
    let context = &command_buffer.context;

    gfx_select!(command_buffer.id => context.command_buffer_drop(command_buffer.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleDrop(render_bundle: native::WGPURenderBundle) {
    assert!(!render_bundle.is_null(), "invalid render bundle");
    let render_bundle = Box::from_raw(render_bundle);
    let context = &render_bundle.context;

    gfx_select!(render_bundle.id => context.render_bundle_drop(render_bundle.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQuerySetDrop(query_set: native::WGPUQuerySet) {
    assert!(!query_set.is_null(), "invalid query set");
    let query_set = Box::from_raw(query_set);
    let context = &query_set.context;

    gfx_select!(query_set.id => context.query_set_drop(query_set.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPipelineDrop(render_pipeline: native::WGPURenderPipeline) {
    assert!(!render_pipeline.is_null(), "invalid render pipeline");
    let render_pipeline = Box::from_raw(render_pipeline);
    let context = &render_pipeline.context;

    gfx_select!(render_pipeline.id => context.render_pipeline_drop(render_pipeline.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePipelineDrop(compute_pipeline: native::WGPUComputePipeline) {
    assert!(!compute_pipeline.is_null(), "invalid compute pipeline");
    let compute_pipeline = Box::from_raw(compute_pipeline);
    let context = &compute_pipeline.context;

    gfx_select!(compute_pipeline.id => context.compute_pipeline_drop(compute_pipeline.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceDrop(device: native::WGPUDevice) {
    assert!(!device.is_null(), "invalid device");
    let device = Box::from_raw(device);
    let context = &device.context;

    gfx_select!(device.id => context.device_drop(device.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterDrop(adapter: native::WGPUAdapter) {
    assert!(!adapter.is_null(), "invalid adapter");
    let adapter = Box::from_raw(adapter);
    let context = &adapter.context;

    gfx_select!(adapter.id => context.adapter_drop(adapter.id));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSurfaceDrop(surface: native::WGPUSurface) {
    assert!(!surface.is_null(), "invalid surface");
    let surface = Box::from_raw(surface);
    let context = &surface.context;

    context.surface_drop(surface.id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSwapChainDrop(swap_chain: native::WGPUSwapChain) {
    assert!(!swap_chain.is_null(), "invalid swap chain");
    drop(Box::from_raw(swap_chain));
}
