use crate::{follow_chain, make_slice, map_enum, native, Label, OwnedLabel};
use std::{borrow::Cow, ffi::CStr, num::NonZeroU32, slice};
use wgc::{id, pipeline::ShaderModuleSource};

map_enum!(
    map_load_op,
    WGPULoadOp,
    wgc::command::LoadOp,
    "Unknown load op",
    Clear,
    Load
);
map_enum!(
    map_store_op,
    WGPUStoreOp,
    wgc::command::StoreOp,
    "Unknown store op",
    Discard,
    Store
);
map_enum!(
    map_address_mode,
    WGPUAddressMode,
    wgt::AddressMode,
    "Unknown address mode",
    ClampToEdge,
    Repeat,
    MirrorRepeat
);
map_enum!(
    map_filter_mode,
    WGPUFilterMode,
    wgt::FilterMode,
    "Unknown filter mode",
    Nearest,
    Linear
);
map_enum!(
    map_mipmap_filter_mode,
    WGPUMipmapFilterMode,
    wgt::FilterMode,
    "Unknown mipmap filter mode",
    Nearest,
    Linear
);
map_enum!(
    map_compare_function,
    WGPUCompareFunction,
    wgt::CompareFunction,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always
);
map_enum!(
    map_texture_aspect,
    WGPUTextureAspect,
    wgt::TextureAspect,
    "Unknown texture aspect",
    All,
    StencilOnly,
    DepthOnly
);
map_enum!(
    map_present_mode,
    WGPUPresentMode,
    wgt::PresentMode,
    "Unknown present mode",
    Immediate,
    Mailbox,
    Fifo
);
map_enum!(
    map_primitive_topology,
    WGPUPrimitiveTopology,
    wgt::PrimitiveTopology,
    "Unknown primitive topology",
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip
);
map_enum!(
    map_index_format,
    WGPUIndexFormat,
    wgt::IndexFormat,
    Uint16,
    Uint32
);
map_enum!(
    map_blend_factor,
    WGPUBlendFactor,
    wgt::BlendFactor,
    "Unknown blend factor",
    Zero: Zero,
    One: One,
    Src: Src,
    OneMinusSrc: OneMinusSrc,
    SrcAlpha: SrcAlpha,
    OneMinusSrcAlpha: OneMinusSrcAlpha,
    Dst: Dst,
    OneMinusDst: OneMinusDst,
    DstAlpha: DstAlpha,
    OneMinusDstAlpha: OneMinusDstAlpha,
    SrcAlphaSaturated: SrcAlphaSaturated,
    Constant: Constant,
    OneMinusConstant: OneMinusConstant
);
map_enum!(
    map_blend_operation,
    WGPUBlendOperation,
    wgt::BlendOperation,
    "Unknown blend operation",
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max
);
map_enum!(
    map_stencil_operation,
    WGPUStencilOperation,
    wgt::StencilOperation,
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap
);
map_enum!(
    map_vertex_format,
    WGPUVertexFormat,
    wgt::VertexFormat,
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,
    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,
    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,
    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4
);

pub fn map_extent3d(native: &native::WGPUExtent3D) -> wgt::Extent3d {
    wgt::Extent3d {
        width: native.width,
        height: native.height,
        depth_or_array_layers: native.depthOrArrayLayers,
    }
}

pub fn map_origin3d(native: &native::WGPUOrigin3D) -> wgt::Origin3d {
    wgt::Origin3d {
        x: native.x,
        y: native.y,
        z: native.z,
    }
}

pub fn map_adapter_options<'a>(
    options: &native::WGPURequestAdapterOptions,
    extras: Option<&native::WGPUAdapterExtras>,
) -> (Option<id::SurfaceId>, native::WGPUBackendType) {
    if let Some(extras) = extras {
        (options.compatibleSurface, extras.backend)
    } else {
        (options.compatibleSurface, native::WGPUBackendType_Null)
    }
}

pub fn map_device_descriptor<'a>(
    des: &native::WGPUDeviceDescriptor,
    extras: Option<&native::WGPUDeviceExtras>,
) -> (wgt::DeviceDescriptor<Label<'a>>, Option<String>) {
    let required_limits = unsafe { *des.requiredLimits };
    let mut features = wgt::Features::empty();
    let limits = unsafe {
        follow_chain!(
            map_required_limits(required_limits,
            WGPUSType_RequiredLimitsExtras => native::WGPURequiredLimitsExtras)
        )
    };
    if let Some(extras) = extras {
        // Handle native features speficied in extras
        if (extras.nativeFeatures
            & native::WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES)
            > 0
        {
            features |= wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
        }
        if (extras.nativeFeatures & native::WGPUNativeFeature_PUSH_CONSTANTS) > 0 {
            features |= wgt::Features::PUSH_CONSTANTS
        }
        if (extras.nativeFeatures
            & native::WGPUNativeFeature_VERTEX_WRITABLE_STORAGE)
            > 0
        {
            features |= wgt::Features::VERTEX_WRITABLE_STORAGE;
        }
        return (
            wgt::DeviceDescriptor {
                label: OwnedLabel::new(extras.label).into_cow(),
                features,
                limits,
            },
            OwnedLabel::new(extras.tracePath).into_inner(),
        );
    } else {
        return (
            wgt::DeviceDescriptor {
                label: None,
                features,
                limits: wgt::Limits::default(),
            },
            None,
        );
    }
}

pub fn map_pipeline_layout_descriptor<'a>(
    des: &native::WGPUPipelineLayoutDescriptor,
    extras: Option<&native::WGPUPipelineLayoutExtras>,
) -> wgc::binding_model::PipelineLayoutDescriptor<'a> {
    let mut push_constant_ranges: Vec<wgt::PushConstantRange> = Vec::new();

    if let Some(extras) = extras {
        let raw_push_constant_ranges = unsafe {
            slice::from_raw_parts(
                extras.pushConstantRanges,
                extras.pushConstantRangeCount as usize,
            )
        };
        for range in raw_push_constant_ranges {
            push_constant_ranges.push(wgt::PushConstantRange {
                stages: wgt::ShaderStages::from_bits(range.stages).expect("Invalid shader stage"),
                range: range.start..range.end,
            });
        }
    }

    return wgc::binding_model::PipelineLayoutDescriptor {
        label: OwnedLabel::new(des.label).into_cow(),
        bind_group_layouts: unsafe {
            Cow::Borrowed(make_slice(
                des.bindGroupLayouts,
                des.bindGroupLayoutCount as usize,
            ))
        },
        push_constant_ranges: Cow::from(push_constant_ranges),
    };
}

pub fn map_required_limits(
    required_limits: native::WGPURequiredLimits,
    extras: Option<&native::WGPURequiredLimitsExtras>,
) -> wgt::Limits {
    let limits = required_limits.limits;
    let mut wgt_limits = wgt::Limits::default();
    if limits.maxTextureDimension1D != 0 {
        wgt_limits.max_texture_dimension_1d = limits.maxTextureDimension1D;
    }
    if limits.maxTextureDimension2D != 0 {
        wgt_limits.max_texture_dimension_2d = limits.maxTextureDimension2D;
    }
    if limits.maxTextureDimension3D != 0 {
        wgt_limits.max_texture_dimension_3d = limits.maxTextureDimension3D;
    }
    if limits.maxTextureArrayLayers != 0 {
        wgt_limits.max_texture_array_layers = limits.maxTextureArrayLayers;
    }
    if limits.maxBindGroups != 0 {
        wgt_limits.max_bind_groups = limits.maxBindGroups;
    }
    if limits.maxDynamicUniformBuffersPerPipelineLayout != 0 {
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout =
            limits.maxDynamicUniformBuffersPerPipelineLayout;
    }
    if limits.maxDynamicStorageBuffersPerPipelineLayout != 0 {
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout =
            limits.maxDynamicStorageBuffersPerPipelineLayout;
    }
    if limits.maxSampledTexturesPerShaderStage != 0 {
        wgt_limits.max_sampled_textures_per_shader_stage = limits.maxSampledTexturesPerShaderStage;
    }
    if limits.maxSamplersPerShaderStage != 0 {
        wgt_limits.max_samplers_per_shader_stage = limits.maxSamplersPerShaderStage;
    }
    if limits.maxStorageBuffersPerShaderStage != 0 {
        wgt_limits.max_storage_buffers_per_shader_stage = limits.maxStorageBuffersPerShaderStage;
    }
    if limits.maxStorageTexturesPerShaderStage != 0 {
        wgt_limits.max_storage_textures_per_shader_stage = limits.maxStorageTexturesPerShaderStage;
    }
    if limits.maxUniformBuffersPerShaderStage != 0 {
        wgt_limits.max_uniform_buffers_per_shader_stage = limits.maxUniformBuffersPerShaderStage;
    }
    if limits.maxUniformBufferBindingSize != 0 {
        wgt_limits.max_uniform_buffer_binding_size = limits.maxUniformBufferBindingSize as u32;
    }
    if limits.maxStorageBufferBindingSize != 0 {
        wgt_limits.max_storage_buffer_binding_size = limits.maxStorageBufferBindingSize as u32;
    }
    /* not yet available in wgpu-core
    if limits.minUniformBufferOffsetAlignment != 0 {
        wgt_limits.yyyy = limits.minUniformBufferOffsetAlignment;
    }
    if limits.minStorageBufferOffsetAlignment != 0 {
        wgt_limits.yyyy = limits.minStorageBufferOffsetAlignment;
    }
    */
    if limits.maxVertexBuffers != 0 {
        wgt_limits.max_vertex_buffers = limits.maxVertexBuffers;
    }
    if limits.maxVertexAttributes != 0 {
        wgt_limits.max_vertex_attributes = limits.maxVertexAttributes;
    }
    /* not yet available in wgpu-core
    if limits.maxVertexBufferArrayStride != 0 {
        wgt_limits.yyyy = limits.maxVertexBufferArrayStride;
    }
    if limits.maxInterStageShaderComponents != 0 {
        wgt_limits.yyyy = limits.maxInterStageShaderComponents;
    }
    if limits.maxComputeWorkgroupStorageSize != 0 {
        wgt_limits.yyyy = limits.maxComputeWorkgroupStorageSize;
    }
    if limits.maxComputeInvocationsPerWorkgroup != 0 {
        wgt_limits.yyyy = limits.maxComputeInvocationsPerWorkgroup;
    }
    if limits.maxComputeWorkgroupSizeX != 0 {
        wgt_limits.yyyy = limits.maxComputeWorkgroupSizeX;
    }
    if limits.maxComputeWorkgroupSizeY != 0 {
        wgt_limits.yyyy = limits.maxComputeWorkgroupSizeY;
    }
    if limits.maxComputeWorkgroupSizeZ != 0 {
        wgt_limits.yyyy = limits.maxComputeWorkgroupSizeZ;
    }
    if limits.maxComputeWorkgroupsPerDimension != 0 {
        wgt_limits.yyyy = limits.maxComputeWorkgroupsPerDimension;
    }
    */
    if let Some(extras) = extras {
        if extras.maxPushConstantSize != 0 {
            wgt_limits.max_push_constant_size = extras.maxPushConstantSize;
        }
    }
    return wgt_limits;
}

pub fn map_shader_module<'a>(
    _: &native::WGPUShaderModuleDescriptor,
    spirv: Option<&native::WGPUShaderModuleSPIRVDescriptor>,
    wgsl: Option<&native::WGPUShaderModuleWGSLDescriptor>,
) -> ShaderModuleSource<'a> {
    if let Some(wgsl) = wgsl {
        let c_str: &CStr = unsafe { CStr::from_ptr(wgsl.code) };
        let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
        ShaderModuleSource::Wgsl(Cow::Borrowed(str_slice))
    } else if let Some(spirv) = spirv {
        let slice = unsafe { make_slice(spirv.code, spirv.codeSize as usize) };
        // Parse the given shader code and store its representation.
        let options = naga::front::spv::Options {
            adjust_coordinate_space: false, // we require NDC_Y_UP feature
            strict_capabilities: true,
            block_ctx_dump_prefix: None,
        };
        let parser = naga::front::spv::Parser::new(slice.iter().cloned(), &options);
        let module = parser.parse().unwrap();
        ShaderModuleSource::Naga(module)
    } else {
        panic!("Shader not provided.");
    }
}

pub fn map_image_copy_texture(
    native: &native::WGPUImageCopyTexture,
) -> wgc::command::ImageCopyTexture {
    wgt::ImageCopyTexture {
        texture: native.texture,
        mip_level: native.mipLevel,
        origin: map_origin3d(&native.origin),
        aspect: map_texture_aspect(native.aspect),
    }
}

pub fn map_image_copy_buffer(
    native: &native::WGPUImageCopyBuffer,
) -> wgc::command::ImageCopyBuffer {
    wgt::ImageCopyBuffer {
        buffer: native.buffer,
        layout: map_texture_data_layout(&native.layout),
    }
}

pub fn map_texture_data_layout(native: &native::WGPUTextureDataLayout) -> wgt::ImageDataLayout {
    wgt::ImageDataLayout {
        offset: native.offset,
        bytes_per_row: NonZeroU32::new(native.bytesPerRow),
        rows_per_image: NonZeroU32::new(native.rowsPerImage),
    }
}

pub fn map_color(native: &native::WGPUColor) -> wgt::Color {
    wgt::Color {
        r: native.r,
        g: native.g,
        b: native.b,
        a: native.a,
    }
}

pub fn map_texture_view_dimension(
    value: native::WGPUTextureViewDimension,
) -> Option<wgt::TextureViewDimension> {
    match value {
        native::WGPUTextureViewDimension_1D => Some(wgt::TextureViewDimension::D1),
        native::WGPUTextureViewDimension_2D => Some(wgt::TextureViewDimension::D2),
        native::WGPUTextureViewDimension_2DArray => Some(wgt::TextureViewDimension::D2Array),
        native::WGPUTextureViewDimension_Cube => Some(wgt::TextureViewDimension::Cube),
        native::WGPUTextureViewDimension_CubeArray => Some(wgt::TextureViewDimension::CubeArray),
        native::WGPUTextureViewDimension_3D => Some(wgt::TextureViewDimension::D3),
        _ => None,
    }
}

pub fn map_texture_dimension(value: native::WGPUTextureDimension) -> wgt::TextureDimension {
    match value {
        native::WGPUTextureDimension_1D => wgt::TextureDimension::D1,
        native::WGPUTextureDimension_2D => wgt::TextureDimension::D2,
        native::WGPUTextureDimension_3D => wgt::TextureDimension::D3,
        x => panic!("Unknown texture dimension: {}", x),
    }
}

pub fn map_texture_format(value: native::WGPUTextureFormat) -> Option<wgt::TextureFormat> {
    // TODO: Add support for BC formats
    match value {
        native::WGPUTextureFormat_R8Unorm => Some(wgt::TextureFormat::R8Unorm),
        native::WGPUTextureFormat_R8Snorm => Some(wgt::TextureFormat::R8Snorm),
        native::WGPUTextureFormat_R8Uint => Some(wgt::TextureFormat::R8Uint),
        native::WGPUTextureFormat_R8Sint => Some(wgt::TextureFormat::R8Sint),
        native::WGPUTextureFormat_R16Uint => Some(wgt::TextureFormat::R16Uint),
        native::WGPUTextureFormat_R16Sint => Some(wgt::TextureFormat::R16Sint),
        native::WGPUTextureFormat_R16Float => Some(wgt::TextureFormat::R16Float),
        native::WGPUTextureFormat_RG8Unorm => Some(wgt::TextureFormat::Rg8Unorm),
        native::WGPUTextureFormat_RG8Snorm => Some(wgt::TextureFormat::Rg8Snorm),
        native::WGPUTextureFormat_RG8Uint => Some(wgt::TextureFormat::Rg8Uint),
        native::WGPUTextureFormat_RG8Sint => Some(wgt::TextureFormat::Rg8Sint),
        native::WGPUTextureFormat_R32Float => Some(wgt::TextureFormat::R32Float),
        native::WGPUTextureFormat_R32Uint => Some(wgt::TextureFormat::R32Uint),
        native::WGPUTextureFormat_R32Sint => Some(wgt::TextureFormat::R32Sint),
        native::WGPUTextureFormat_RG16Uint => Some(wgt::TextureFormat::Rg16Uint),
        native::WGPUTextureFormat_RG16Sint => Some(wgt::TextureFormat::Rg16Sint),
        native::WGPUTextureFormat_RG16Float => Some(wgt::TextureFormat::Rg16Float),
        native::WGPUTextureFormat_RGBA8Unorm => Some(wgt::TextureFormat::Rgba8Unorm),
        native::WGPUTextureFormat_RGBA8UnormSrgb => Some(wgt::TextureFormat::Rgba8UnormSrgb),
        native::WGPUTextureFormat_RGBA8Snorm => Some(wgt::TextureFormat::Rgba8Snorm),
        native::WGPUTextureFormat_RGBA8Uint => Some(wgt::TextureFormat::Rgba8Uint),
        native::WGPUTextureFormat_RGBA8Sint => Some(wgt::TextureFormat::Rgba8Sint),
        native::WGPUTextureFormat_BGRA8Unorm => Some(wgt::TextureFormat::Bgra8Unorm),
        native::WGPUTextureFormat_BGRA8UnormSrgb => Some(wgt::TextureFormat::Bgra8UnormSrgb),
        native::WGPUTextureFormat_RGB10A2Unorm => Some(wgt::TextureFormat::Rgb10a2Unorm),
        native::WGPUTextureFormat_RG32Float => Some(wgt::TextureFormat::Rg32Float),
        native::WGPUTextureFormat_RG32Uint => Some(wgt::TextureFormat::Rg32Uint),
        native::WGPUTextureFormat_RG32Sint => Some(wgt::TextureFormat::Rg32Sint),
        native::WGPUTextureFormat_RGBA16Uint => Some(wgt::TextureFormat::Rgba16Uint),
        native::WGPUTextureFormat_RGBA16Sint => Some(wgt::TextureFormat::Rgba16Sint),
        native::WGPUTextureFormat_RGBA16Float => Some(wgt::TextureFormat::Rgba16Float),
        native::WGPUTextureFormat_RGBA32Float => Some(wgt::TextureFormat::Rgba32Float),
        native::WGPUTextureFormat_RGBA32Uint => Some(wgt::TextureFormat::Rgba32Uint),
        native::WGPUTextureFormat_RGBA32Sint => Some(wgt::TextureFormat::Rgba32Sint),
        native::WGPUTextureFormat_Depth32Float => Some(wgt::TextureFormat::Depth32Float),
        native::WGPUTextureFormat_Depth24Plus => Some(wgt::TextureFormat::Depth24Plus),
        native::WGPUTextureFormat_Depth24PlusStencil8 => {
            Some(wgt::TextureFormat::Depth24PlusStencil8)
        }
        _ => None,
    }
}

pub fn to_native_texture_format(rs_type: wgt::TextureFormat) -> native::WGPUTextureFormat {
    match rs_type {
        wgt::TextureFormat::R8Unorm => native::WGPUTextureFormat_R8Unorm,
        wgt::TextureFormat::R8Snorm => native::WGPUTextureFormat_R8Snorm,
        wgt::TextureFormat::R8Uint => native::WGPUTextureFormat_R8Uint,
        wgt::TextureFormat::R8Sint => native::WGPUTextureFormat_R8Sint,
        wgt::TextureFormat::R16Uint => native::WGPUTextureFormat_R16Uint,
        wgt::TextureFormat::R16Sint => native::WGPUTextureFormat_R16Sint,
        wgt::TextureFormat::R16Float => native::WGPUTextureFormat_R16Float,
        wgt::TextureFormat::Rg8Unorm => native::WGPUTextureFormat_RG8Unorm,
        wgt::TextureFormat::Rg8Snorm => native::WGPUTextureFormat_RG8Snorm,
        wgt::TextureFormat::Rg8Uint => native::WGPUTextureFormat_RG8Uint,
        wgt::TextureFormat::Rg8Sint => native::WGPUTextureFormat_RG8Sint,
        wgt::TextureFormat::R32Float => native::WGPUTextureFormat_R32Float,
        wgt::TextureFormat::R32Uint => native::WGPUTextureFormat_R32Uint,
        wgt::TextureFormat::R32Sint => native::WGPUTextureFormat_R32Sint,
        wgt::TextureFormat::Rg16Uint => native::WGPUTextureFormat_RG16Uint,
        wgt::TextureFormat::Rg16Sint => native::WGPUTextureFormat_RG16Sint,
        wgt::TextureFormat::Rg16Float => native::WGPUTextureFormat_RG16Float,
        wgt::TextureFormat::Rgba8Unorm => native::WGPUTextureFormat_RGBA8Unorm,
        wgt::TextureFormat::Rgba8UnormSrgb => native::WGPUTextureFormat_RGBA8UnormSrgb,
        wgt::TextureFormat::Rgba8Snorm => native::WGPUTextureFormat_RGBA8Snorm,
        wgt::TextureFormat::Rgba8Uint => native::WGPUTextureFormat_RGBA8Uint,
        wgt::TextureFormat::Rgba8Sint => native::WGPUTextureFormat_RGBA8Sint,
        wgt::TextureFormat::Bgra8Unorm => native::WGPUTextureFormat_BGRA8Unorm,
        wgt::TextureFormat::Bgra8UnormSrgb => native::WGPUTextureFormat_BGRA8UnormSrgb,
        wgt::TextureFormat::Rgb10a2Unorm => native::WGPUTextureFormat_RGB10A2Unorm,
        wgt::TextureFormat::Rg32Float => native::WGPUTextureFormat_RG32Float,
        wgt::TextureFormat::Rg32Uint => native::WGPUTextureFormat_RG32Uint,
        wgt::TextureFormat::Rg32Sint => native::WGPUTextureFormat_RG32Sint,
        wgt::TextureFormat::Rgba16Uint => native::WGPUTextureFormat_RGBA16Uint,
        wgt::TextureFormat::Rgba16Sint => native::WGPUTextureFormat_RGBA16Sint,
        wgt::TextureFormat::Rgba16Float => native::WGPUTextureFormat_RGBA16Float,
        wgt::TextureFormat::Rgba32Float => native::WGPUTextureFormat_RGBA32Float,
        wgt::TextureFormat::Rgba32Uint => native::WGPUTextureFormat_RGBA32Uint,
        wgt::TextureFormat::Rgba32Sint => native::WGPUTextureFormat_RGBA32Sint,
        wgt::TextureFormat::Depth32Float => native::WGPUTextureFormat_Depth32Float,
        wgt::TextureFormat::Depth24Plus => native::WGPUTextureFormat_Depth24Plus,
        wgt::TextureFormat::Depth24PlusStencil8 => native::WGPUTextureFormat_Depth24PlusStencil8,
        _ => unimplemented!(),
    }
}

pub fn map_stencil_face_state(value: native::WGPUStencilFaceState) -> wgt::StencilFaceState {
    wgt::StencilFaceState {
        compare: map_compare_function(value.compare).unwrap(),
        fail_op: map_stencil_operation(value.failOp).unwrap(),
        depth_fail_op: map_stencil_operation(value.depthFailOp).unwrap(),
        pass_op: map_stencil_operation(value.passOp).unwrap(),
    }
}

pub fn map_storage_report(report: wgc::hub::StorageReport) -> native::WGPUStorageReport {
    native::WGPUStorageReport {
        numOccupied: report.num_occupied,
        numVacant: report.num_error,
        numError: report.num_error,
        elementSize: report.element_size,
    }
}

pub fn map_hub_report(report: wgc::hub::HubReport) -> native::WGPUHubReport {
    native::WGPUHubReport {
        adapters: map_storage_report(report.adapters),
        devices: map_storage_report(report.devices),
        pipelineLayouts: map_storage_report(report.pipeline_layouts),
        shaderModules: map_storage_report(report.shader_modules),
        bindGroupLayouts: map_storage_report(report.bind_group_layouts),
        bindGroups: map_storage_report(report.bind_groups),
        commandBuffers: map_storage_report(report.command_buffers),
        renderBundles: map_storage_report(report.render_bundles),
        renderPipelines: map_storage_report(report.render_pipelines),
        computePipelines: map_storage_report(report.compute_pipelines),
        querySets: map_storage_report(report.query_sets),
        buffers: map_storage_report(report.buffers),
        textures: map_storage_report(report.textures),
        textureViews: map_storage_report(report.texture_views),
        samplers: map_storage_report(report.samplers),
    }
}

#[inline]
pub unsafe fn write_global_report(
    native_report: &mut native::WGPUGlobalReport,
    report: wgc::hub::GlobalReport,
) {
    native_report.surfaces = map_storage_report(report.surfaces);

    #[cfg(vulkan)]
    if let Some(vulkan) = report.vulkan {
        native_report.vulkan = map_hub_report(vulkan);
        native_report.backendType = native::WGPUBackendType_Vulkan;
    }

    #[cfg(metal)]
    if let Some(metal) = report.metal {
        native_report.metal = map_hub_report(metal);
        native_report.backendType = native::WGPUBackendType_Metal;
    }

    #[cfg(dx12)]
    if let Some(dx12) = report.dx12 {
        native_report.dx12 = map_hub_report(dx12);
        native_report.backendType = native::WGPUBackendType_D3D12;
    }

    #[cfg(dx11)]
    if let Some(dx11) = report.dx11 {
        native_report.dx11 = map_hub_report(dx11);
        native_report.backendType = native::WGPUBackendType_D3D11;
    }

    #[cfg(gl)]
    if let Some(gl) = report.gl {
        native_report.gl = map_hub_report(gl);
        native_report.backendType = native::WGPUBackendType_OpenGL;
    }
}
