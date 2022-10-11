use crate::conv::{
    map_adapter_options, map_device_descriptor, map_pipeline_layout_descriptor, map_shader_module,
};
use crate::{conv, follow_chain, handle_device_error, make_slice, native, OwnedLabel, GLOBAL};
use lazy_static::lazy_static;
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryInto,
    ffi::CString,
    num::{NonZeroU32, NonZeroU64, NonZeroU8},
    path::Path,
    sync::Mutex,
};
use thiserror::Error;
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
    let power_preference = match options.powerPreference {
        native::WGPUPowerPreference_LowPower => wgt::PowerPreference::LowPower,
        native::WGPUPowerPreference_HighPerformance => wgt::PowerPreference::HighPerformance,
        _ => wgt::PowerPreference::default(),
    };
    let backend_bits = match given_backend {
        native::WGPUBackendType_Null => wgt::Backends::all(),
        native::WGPUBackendType_Vulkan => wgt::Backends::VULKAN,
        native::WGPUBackendType_Metal => wgt::Backends::METAL,
        native::WGPUBackendType_D3D12 => wgt::Backends::DX12,
        native::WGPUBackendType_D3D11 => wgt::Backends::DX11,
        native::WGPUBackendType_OpenGL => wgt::Backends::GL,
        _ => panic!("Invalid backend {}", given_backend),
    };
    let adapter_id = GLOBAL
        .request_adapter(
            &wgt::RequestAdapterOptions {
                power_preference,
                compatible_surface,
                force_fallback_adapter: options.forceFallbackAdapter,
            },
            wgc::instance::AdapterInputs::Mask(backend_bits, |_| ()),
        )
        .expect("Unable to request adapter");
    let status = native::WGPURequestAdapterStatus_Success; // todo: cleanly communicate a non-success
    let message_ptr = std::ptr::null();
    (callback.unwrap())(status, Some(adapter_id), message_ptr, userdata);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: native::WGPUAdapter,
    descriptor: &native::WGPUDeviceDescriptor,
    callback: native::WGPURequestDeviceCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let adapter = adapter.expect("invalid adapter");

    let (desc, trace_str) = follow_chain!(
        map_device_descriptor(descriptor,
        WGPUSType_DeviceExtras => native::WGPUDeviceExtras)
    );
    let trace_path = trace_str.as_ref().map(Path::new);

    let (id, error) =
        gfx_select!(adapter => GLOBAL.adapter_request_device(adapter, &desc, trace_path, ()));

    let status = match error {
        Some(_error) => native::WGPURequestDeviceStatus_Error,
        None => native::WGPURequestDeviceStatus_Success,
    };

    let message_ptr = std::ptr::null();
    (callback.unwrap())(status, Some(id), message_ptr, userdata);
}

lazy_static! {
    static ref ADAPTER_NAMES: Mutex<HashMap<id::AdapterId, CString>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterGetProperties(
    adapter: native::WGPUAdapter,
    properties: &mut native::WGPUAdapterProperties,
) {
    let adapter = adapter.expect("invalid adapter");

    let maybe_props = gfx_select!(adapter => GLOBAL.adapter_get_info(adapter));
    if let Ok(props) = maybe_props {
        properties.name = ADAPTER_NAMES
            .lock()
            .unwrap()
            .entry(adapter)
            .or_insert_with(|| CString::new((&props.name) as &str).unwrap())
            .as_ptr();
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
    let adapter = adapter.expect("invalid adapter");

    let result = gfx_select!(adapter => GLOBAL.adapter_limits(adapter));
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
    let adapter = adapter.expect("invalid adapter");

    let adapter_features = match gfx_select!(adapter => GLOBAL.adapter_features(adapter)) {
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
    let adapter = adapter.expect("invalid adapter");

    let adapter_features = match gfx_select!(adapter => GLOBAL.adapter_features(adapter)) {
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
    let device = device.expect("invalid device");

    let device_features = match gfx_select!(device => GLOBAL.device_features(device)) {
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
    let device = device.expect("invalid device");

    let device_features = match gfx_select!(device => GLOBAL.device_features(device)) {
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
    let device = device.expect("invalid device");

    let result = gfx_select!(device => GLOBAL.device_limits(device));
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
            (*extras).maxBufferSize = wgt_limits.max_buffer_size;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateShaderModule(
    device: native::WGPUDevice,
    descriptor: &native::WGPUShaderModuleDescriptor,
) -> native::WGPUShaderModule {
    let device = device.expect("invalid device");

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
        gfx_select!(device => GLOBAL.device_create_shader_module(device, &desc, source, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateBuffer(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBufferDescriptor,
) -> native::WGPUBuffer {
    let device = device.expect("invalid device");

    let usage = wgt::BufferUsages::from_bits(descriptor.usage).expect("Buffer Usage Invalid.");
    let label = OwnedLabel::new(descriptor.label);
    let (id, error) = gfx_select!(device => GLOBAL.device_create_buffer(
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
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuBufferDestroy(buffer_id: native::WGPUBuffer) {
    let buffer_id = buffer_id.expect("invalid buffer");

    gfx_select!(buffer_id => GLOBAL.buffer_destroy(buffer_id)).expect("Unable to destroy buffer");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBindGroupLayoutDescriptor,
) -> native::WGPUBindGroupLayout {
    let device = device.expect("invalid device");

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
                min_binding_size: NonZeroU64::new(entry.buffer.minBindingSize),
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
        gfx_select!(device => GLOBAL.device_create_bind_group_layout(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: native::WGPUDevice,
    descriptor: &native::WGPUBindGroupDescriptor,
) -> native::WGPUBindGroup {
    let device = device.expect("invalid device");

    let mut entries = Vec::new();
    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let wgc_entry = if let Some(buffer) = entry.buffer {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Buffer(
                    wgc::binding_model::BufferBinding {
                        buffer_id: buffer,
                        offset: entry.offset,
                        size: NonZeroU64::new(entry.size),
                    },
                ),
            }
        } else if let Some(sampler) = entry.sampler {
            wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::Sampler(sampler),
            }
        } else if let Some(texture_view) = entry.textureView {
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
            .expect("invalid bind group layout for bind group descriptor"),
        entries: Cow::Borrowed(&entries),
    };
    let (id, error) = gfx_select!(device => GLOBAL.device_create_bind_group(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: native::WGPUDevice,
    descriptor: &native::WGPUPipelineLayoutDescriptor,
) -> native::WGPUPipelineLayout {
    let device = device.expect("invalid device");

    let desc = follow_chain!(
        map_pipeline_layout_descriptor(
            descriptor,
            WGPUSType_PipelineLayoutExtras => native::WGPUPipelineLayoutExtras)
    );
    let (id, error) =
        gfx_select!(device => GLOBAL.device_create_pipeline_layout(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
    device: native::WGPUDevice,
    descriptor: &native::WGPUComputePipelineDescriptor,
) -> native::WGPUComputePipeline {
    let device = device.expect("invalid device");

    let stage = wgc::pipeline::ProgrammableStageDescriptor {
        module: descriptor
            .compute
            .module
            .expect("invalid shader module for compute pipeline descriptor"),
        entry_point: OwnedLabel::new(descriptor.compute.entryPoint)
            .into_cow()
            .expect("Entry point not provided"),
    };
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout,
        stage,
    };

    let implicit_pipeline_ids = match desc.layout {
        Some(_) => None,
        None => Some(wgc::device::ImplicitPipelineIds {
            root_id: (),
            group_ids: &[(); wgc::MAX_BIND_GROUPS],
        }),
    };

    let (id, error) = gfx_select!(device => GLOBAL.device_create_compute_pipeline(device, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUCommandEncoderDescriptor>,
) -> native::WGPUCommandEncoder {
    let device = device.expect("invalid device");

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandEncoderDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandEncoderDescriptor::default(),
    };
    let (id, error) =
        gfx_select!(device => GLOBAL.device_create_command_encoder(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceGetQueue(device: native::WGPUDevice) -> native::WGPUQueue {
    Some(device.expect("invalid device"))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) {
    let queue = queue.expect("invalid queue");

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        command_buffers.push(command_buffer.expect("invalid command buffer"))
    }

    gfx_select!(queue => GLOBAL.queue_submit(queue, &command_buffers))
        .expect("Unable to submit queue");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuQueueSubmitForIndex(
    queue: native::WGPUQueue,
    command_count: u32,
    commands: *const native::WGPUCommandBuffer,
) -> native::WGPUSubmissionIndex {
    let queue = queue.expect("invalid queue");

    let mut command_buffers = Vec::new();
    for command_buffer in make_slice(commands, command_count as usize) {
        command_buffers.push(command_buffer.expect("invalid command buffer"))
    }

    let submission_index = gfx_select!(queue => GLOBAL.queue_submit(queue, &command_buffers))
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
    let queue = queue.expect("invalid queue");
    let buffer = buffer.expect("invalid buffer");

    let slice = make_slice(data, data_size);
    gfx_select!(queue => GLOBAL.queue_write_buffer(queue, buffer, buffer_offset, slice))
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
    let queue = queue.expect("invalid queue");

    let slice = make_slice(data, data_size);
    gfx_select!(queue => GLOBAL.queue_write_texture(
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
    let buffer = buffer.expect("invalid buffer");

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

    gfx_select!(buffer => GLOBAL.buffer_map_async(buffer, offset as u64 .. (offset + size) as u64, operation))
        .expect("Unable to map buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDevicePoll(
    device: native::WGPUDevice,
    wait: bool,
    wrapped_submission_index: Option<&native::WGPUWrappedSubmissionIndex>,
) -> bool {
    let device = device.expect("invalid device");

    let maintain = match wait {
        true => match wrapped_submission_index {
            Some(index) => {
                wgt::Maintain::WaitForSubmissionIndex(wgc::device::queue::WrappedSubmissionIndex {
                    queue_id: index
                        .queue
                        .expect("invalid queue for wrapped submission index"),
                    index: index.submissionIndex,
                })
            }
            None => wgt::Maintain::Wait,
        },
        false => wgt::Maintain::Poll,
    };

    gfx_select!(device => GLOBAL.device_poll(device, maintain)).expect("Unable to poll device")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuBufferGetMappedRange(
    buffer: native::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut u8 {
    let buffer = buffer.expect("invalid buffer");

    gfx_select!(buffer => GLOBAL.buffer_get_mapped_range(buffer, offset as u64, Some(size as u64)))
        .expect("Unable to get mapped range")
        .0
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: native::WGPUDevice,
    descriptor: &native::WGPURenderPipelineDescriptor,
) -> native::WGPURenderPipeline {
    let device = device.expect("invalid device");

    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout,
        vertex: wgc::pipeline::VertexState {
            stage: wgc::pipeline::ProgrammableStageDescriptor {
                module: descriptor
                    .vertex
                    .module
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

    let (id, error) = gfx_select!(device => GLOBAL.device_create_render_pipeline(device, &desc, (), implicit_pipeline_ids));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    pipeline: native::WGPURenderPipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let pipeline = pipeline.expect("invalid render pipeline");

    let (id, error) = gfx_select!(pipeline => GLOBAL.render_pipeline_get_bind_group_layout(pipeline, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the render pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get render pipeline bind group layout: {:?}",
            error
        );
        None
    } else {
        Some(id)
    }
}

lazy_static! {
    static ref SURFACE_TO_DEVICE: Mutex<HashMap<id::SurfaceId, id::DeviceId>> =
        Mutex::new(HashMap::new());
}

fn get_device_from_surface(surface: id::SurfaceId) -> id::DeviceId {
    *SURFACE_TO_DEVICE.lock().unwrap().get(&surface).unwrap()
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateSwapChain(
    device: native::WGPUDevice,
    surface: native::WGPUSurface,
    descriptor: &native::WGPUSwapChainDescriptor,
) -> native::WGPUSurface {
    let device = device.expect("invalid device");
    let surface = surface.expect("invalid surface");

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
    let error = gfx_select!(device => GLOBAL.surface_configure(surface, device, &config));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        SURFACE_TO_DEVICE.lock().unwrap().insert(surface, device);
        Some(surface) // swap chain_id == surface_id
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
pub extern "C" fn wgpuSwapChainGetCurrentTextureView(
    swap_chain: native::WGPUSwapChain,
) -> native::WGPUTextureView {
    let surface_id = swap_chain.expect("invalid swapchain");
    let device = get_device_from_surface(surface_id);
    match gfx_select!(device => GLOBAL.surface_get_current_texture(surface_id, ())) {
        Err(error) => {
            handle_device_error(device, &error);
            None
        }
        Ok(result) => match result.status {
            wgt::SurfaceStatus::Good | wgt::SurfaceStatus::Suboptimal => {
                let texture = result.texture_id.unwrap();
                let desc = wgc::resource::TextureViewDescriptor::default();
                Some(gfx_select!(texture => GLOBAL.texture_create_view(texture, &desc, ())).0)
            }
            wgt::SurfaceStatus::Timeout => {
                handle_device_error(device, &SurfaceError::Timeout);
                None
            }
            wgt::SurfaceStatus::Outdated => {
                handle_device_error(device, &SurfaceError::Outdated);
                None
            }
            wgt::SurfaceStatus::Lost => {
                handle_device_error(device, &SurfaceError::Lost);
                None
            }
        },
    }
}

#[no_mangle]
pub extern "C" fn wgpuSwapChainPresent(swap_chain: native::WGPUSwapChain) {
    let surface_id = swap_chain.expect("invalid swapchain");
    let device_id = get_device_from_surface(surface_id);
    gfx_select!(device_id => GLOBAL.surface_present(surface_id))
        .expect("Unable to present swap chain");
}

#[no_mangle]
pub extern "C" fn wgpuTextureCreateView(
    texture: native::WGPUTexture,
    descriptor: Option<&native::WGPUTextureViewDescriptor>,
) -> native::WGPUTextureView {
    let texture = texture.expect("invalid texture");

    let desc = match descriptor {
        Some(descriptor) => wgc::resource::TextureViewDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
            format: conv::map_texture_format(descriptor.format),
            dimension: conv::map_texture_view_dimension(descriptor.dimension),
            range: wgt::ImageSubresourceRange {
                aspect: conv::map_texture_aspect(descriptor.aspect),
                base_mip_level: descriptor.baseMipLevel,
                mip_level_count: NonZeroU32::new(descriptor.mipLevelCount),
                base_array_layer: descriptor.baseArrayLayer,
                array_layer_count: NonZeroU32::new(descriptor.arrayLayerCount),
            },
        },
        None => wgc::resource::TextureViewDescriptor::default(),
    };

    let (id, error) = gfx_select!(texture => GLOBAL.texture_create_view(texture, &desc, ()));

    if let Some(error) = error {
        // TODO: report via handle_device_error()
        log::error!("Failed to create texture view for texture: {:?}", error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateTexture(
    device: native::WGPUDevice,
    descriptor: &native::WGPUTextureDescriptor,
) -> native::WGPUTexture {
    let device = device.expect("invalid device");

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

    let (id, error) = gfx_select!(device => GLOBAL.device_create_texture(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuTextureDestroy(texture_id: native::WGPUTexture) {
    let texture_id = texture_id.expect("invalid texture");
    gfx_select!(texture_id => GLOBAL.texture_destroy(texture_id))
        .expect("Failed to destroy texture");
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateSampler(
    device: native::WGPUDevice,
    descriptor: Option<&native::WGPUSamplerDescriptor>,
) -> native::WGPUSampler {
    let device = device.expect("invalid device");

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

    let (id, error) = gfx_select!(device => GLOBAL.device_create_sampler(device, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateRenderBundleEncoder(
    device: native::WGPUDevice,
    descriptor: &native::WGPURenderBundleEncoderDescriptor,
) -> native::WGPURenderBundleEncoder {
    let device = device.expect("invalid device");

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
        Ok(encoder) => Box::into_raw(Box::new(encoder)),
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
pub extern "C" fn wgpuComputePipelineGetBindGroupLayout(
    pipeline: native::WGPUComputePipeline,
    group_index: u32,
) -> native::WGPUBindGroupLayout {
    let pipeline = pipeline.expect("invalid compute pipeline");

    let (id, error) = gfx_select!(pipeline => GLOBAL.compute_pipeline_get_bind_group_layout(pipeline, group_index, ()));
    if let Some(error) = error {
        // TODO figure out what device the compute pipeline belongs to and call
        // handle_device_error()
        log::error!(
            "Failed to get compute pipeline bind group layout: {:?}",
            error
        );
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub extern "C" fn wgpuBufferUnmap(buffer_id: native::WGPUBuffer) {
    let buffer_id = buffer_id.expect("invalid buffer");
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub extern "C" fn wgpuSurface(buffer_id: native::WGPUBuffer) {
    let buffer_id = buffer_id.expect("invalid buffer");
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub extern "C" fn wgpuBufferDrop(buffer_id: native::WGPUBuffer) {
    let buffer_id = buffer_id.expect("invalid buffer");
    gfx_select!(buffer_id => GLOBAL.buffer_drop(buffer_id, false))
}

#[no_mangle]
pub extern "C" fn wgpuTextureDrop(texture_id: native::WGPUTexture) {
    let texture_id = texture_id.expect("invalid texture");
    gfx_select!(texture_id => GLOBAL.texture_drop(texture_id, false))
}

#[no_mangle]
pub extern "C" fn wgpuTextureViewDrop(texture_view_id: native::WGPUTextureView) {
    let texture_view_id = texture_view_id.expect("invalid texture view");
    gfx_select!(texture_view_id => GLOBAL.texture_view_drop(texture_view_id, false))
        .expect("Unable to drop texture view")
}

#[no_mangle]
pub extern "C" fn wgpuSamplerDrop(sampler_id: native::WGPUSampler) {
    let sampler_id = sampler_id.expect("invalid sampler");
    gfx_select!(sampler_id => GLOBAL.sampler_drop(sampler_id))
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupLayoutDrop(bind_group_layout_id: native::WGPUBindGroupLayout) {
    let bind_group_layout_id = bind_group_layout_id.expect("invalid bind group layout");
    gfx_select!(bind_group_layout_id => GLOBAL.bind_group_layout_drop(bind_group_layout_id))
}

#[no_mangle]
pub extern "C" fn wgpuPipelineLayoutDrop(pipeline_layout_id: native::WGPUPipelineLayout) {
    let pipeline_layout_id = pipeline_layout_id.expect("invalid pipeline layout");
    gfx_select!(pipeline_layout_id => GLOBAL.pipeline_layout_drop(pipeline_layout_id))
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupDrop(bind_group_id: native::WGPUBindGroup) {
    let bind_group_id = bind_group_id.expect("invalid bind group");
    gfx_select!(bind_group_id => GLOBAL.bind_group_drop(bind_group_id))
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleDrop(shader_module_id: native::WGPUShaderModule) {
    let shader_module_id = shader_module_id.expect("invalid shader module");
    gfx_select!(shader_module_id => GLOBAL.shader_module_drop(shader_module_id))
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderDrop(command_encoder_id: native::WGPUCommandEncoder) {
    let command_encoder_id = command_encoder_id.expect("invalid command encoder");
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_drop(command_encoder_id))
}

#[no_mangle]
pub extern "C" fn wgpuCommandBufferDrop(command_buffer_id: native::WGPUCommandBuffer) {
    let command_buffer_id = command_buffer_id.expect("invalid command buffer");
    gfx_select!(command_buffer_id => GLOBAL.command_buffer_drop(command_buffer_id))
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleDrop(render_bundle_id: native::WGPURenderBundle) {
    let render_bundle_id = render_bundle_id.expect("invalid render bundle");
    gfx_select!(render_bundle_id => GLOBAL.render_bundle_drop(render_bundle_id))
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetDrop(query_set_id: native::WGPUQuerySet) {
    let query_set_id = query_set_id.expect("invalid query set");
    gfx_select!(query_set_id => GLOBAL.query_set_drop(query_set_id))
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineDrop(render_pipeline_id: native::WGPURenderPipeline) {
    let render_pipeline_id = render_pipeline_id.expect("invalid render pipeline");
    gfx_select!(render_pipeline_id => GLOBAL.render_pipeline_drop(render_pipeline_id))
}

#[no_mangle]
pub extern "C" fn wgpuComputePipelineDrop(compute_pipeline_id: native::WGPUComputePipeline) {
    let compute_pipeline_id = compute_pipeline_id.expect("invalid compute pipeline");
    gfx_select!(compute_pipeline_id => GLOBAL.compute_pipeline_drop(compute_pipeline_id))
}

#[no_mangle]
pub extern "C" fn wgpuDeviceDrop(device_id: native::WGPUDevice) {
    let device_id = device_id.expect("invalid device");
    gfx_select!(device_id => GLOBAL.device_drop(device_id))
}

#[no_mangle]
pub extern "C" fn wgpuAdapterDrop(adapter: native::WGPUAdapter) {
    let adapter = adapter.expect("invalid surface");
    gfx_select!(adapter => GLOBAL.adapter_drop(adapter))
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceDrop(surface: native::WGPUSurface) {
    let surface = surface.expect("invalid surface");
    GLOBAL.surface_drop(surface)
}
