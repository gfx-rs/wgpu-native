use crate::native::{self, UnwrapId};
use crate::{follow_chain, make_slice, map_enum, Label, OwnedLabel};
use std::{borrow::Cow, ffi::CStr, num::NonZeroU32, slice};

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

map_enum!(
    map_shader_stage,
    WGPUShaderStage,
    naga::ShaderStage,
    Vertex,
    Fragment,
    Compute
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

pub fn map_instance_backend_flags(flags: i32) -> wgt::Backends {
    let mut result: wgt::Backends = wgt::Backends::empty();
    if (flags & native::WGPUInstanceBackend_BrowserWebGPU) != 0 {
        result |= wgt::Backends::BROWSER_WEBGPU;
    }
    if (flags & native::WGPUInstanceBackend_Vulkan) != 0 {
        result |= wgt::Backends::VULKAN;
    }
    if (flags & native::WGPUInstanceBackend_GL) != 0 {
        result |= wgt::Backends::GL;
    }
    if (flags & native::WGPUInstanceBackend_Metal) != 0 {
        result |= wgt::Backends::METAL;
    }
    if (flags & native::WGPUInstanceBackend_DX12) != 0 {
        result |= wgt::Backends::DX12;
    }
    if (flags & native::WGPUInstanceBackend_DX11) != 0 {
        result |= wgt::Backends::DX11;
    }
    result
}

pub fn map_instance_descriptor(
    _base: &native::WGPUInstanceDescriptor,
    extras: Option<&native::WGPUInstanceExtras>,
) -> (wgt::Backends) {
    if let Some(extras) = extras {
        (map_instance_backend_flags(extras.backends as i32))
    } else {
        (wgt::Backends::PRIMARY)
    }
}

pub fn map_adapter_options(
    options: &native::WGPURequestAdapterOptions,
    extras: Option<&native::WGPUAdapterExtras>,
) -> (native::WGPUSurface, native::WGPUBackendType) {
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
    let limits = unsafe { des.requiredLimits.as_ref() }.map_or(
        wgt::Limits::downlevel_defaults(),
        |required_limits| unsafe {
            follow_chain!(
                map_required_limits(required_limits,
                WGPUSType_RequiredLimitsExtras => native::WGPURequiredLimitsExtras)
            )
        },
    );

    (
        wgt::DeviceDescriptor {
            label: OwnedLabel::new(des.label).into_cow(),
            features: map_features(unsafe {
                make_slice(des.requiredFeatures, des.requiredFeaturesCount as usize)
            }),
            limits,
        },
        extras.and_then(|extras| OwnedLabel::new(extras.tracePath).into_inner()),
    )
}

pub unsafe fn map_pipeline_layout_descriptor<'a>(
    des: &native::WGPUPipelineLayoutDescriptor,
    extras: Option<&native::WGPUPipelineLayoutExtras>,
) -> wgc::binding_model::PipelineLayoutDescriptor<'a> {
    let bind_group_layouts =
        unsafe { make_slice(des.bindGroupLayouts, des.bindGroupLayoutCount as usize) }
            .iter()
            .map(|layout| {
                layout
                    .as_option()
                    .expect("invalid bind group layout for pipeline layout descriptor")
            })
            .collect::<Vec<wgc::id::BindGroupLayoutId>>();

    let push_constant_ranges = extras.map_or(Vec::new(), |extras| {
        unsafe {
            make_slice(
                extras.pushConstantRanges,
                extras.pushConstantRangeCount as usize,
            )
        }
        .iter()
        .map(|range| wgt::PushConstantRange {
            stages: wgt::ShaderStages::from_bits(range.stages)
                .expect("invalid shader stage for push constant range"),
            range: range.start..range.end,
        })
        .collect()
    });

    return wgc::binding_model::PipelineLayoutDescriptor {
        label: OwnedLabel::new(des.label).into_cow(),
        bind_group_layouts: Cow::from(bind_group_layouts),
        push_constant_ranges: Cow::from(push_constant_ranges),
    };
}

pub fn map_required_limits(
    required_limits: &native::WGPURequiredLimits,
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
    if limits.minUniformBufferOffsetAlignment != 0 {
        wgt_limits.min_uniform_buffer_offset_alignment = limits.minUniformBufferOffsetAlignment;
    }
    if limits.minStorageBufferOffsetAlignment != 0 {
        wgt_limits.min_storage_buffer_offset_alignment = limits.minStorageBufferOffsetAlignment;
    }
    if limits.maxVertexBuffers != 0 {
        wgt_limits.max_vertex_buffers = limits.maxVertexBuffers;
    }
    if limits.maxVertexAttributes != 0 {
        wgt_limits.max_vertex_attributes = limits.maxVertexAttributes;
    }
    if limits.maxVertexBufferArrayStride != 0 {
        wgt_limits.max_vertex_buffer_array_stride = limits.maxVertexBufferArrayStride;
    }
    if limits.maxInterStageShaderComponents != 0 {
        wgt_limits.max_inter_stage_shader_components = limits.maxInterStageShaderComponents;
    }
    if limits.maxComputeWorkgroupStorageSize != 0 {
        wgt_limits.max_compute_workgroup_storage_size = limits.maxComputeWorkgroupStorageSize;
    }
    if limits.maxComputeInvocationsPerWorkgroup != 0 {
        wgt_limits.max_compute_invocations_per_workgroup = limits.maxComputeInvocationsPerWorkgroup;
    }
    if limits.maxComputeWorkgroupSizeX != 0 {
        wgt_limits.max_compute_workgroup_size_x = limits.maxComputeWorkgroupSizeX;
    }
    if limits.maxComputeWorkgroupSizeY != 0 {
        wgt_limits.max_compute_workgroup_size_y = limits.maxComputeWorkgroupSizeY;
    }
    if limits.maxComputeWorkgroupSizeZ != 0 {
        wgt_limits.max_compute_workgroup_size_z = limits.maxComputeWorkgroupSizeZ;
    }
    if limits.maxComputeWorkgroupsPerDimension != 0 {
        wgt_limits.max_compute_workgroups_per_dimension = limits.maxComputeWorkgroupsPerDimension;
    }
    if let Some(extras) = extras {
        if extras.maxPushConstantSize != 0 {
            wgt_limits.max_push_constant_size = extras.maxPushConstantSize;
        }
        if extras.maxBufferSize != 0 {
            wgt_limits.max_buffer_size = extras.maxBufferSize;
        }
    }
    wgt_limits
}

pub fn map_shader_module<'a>(
    _: &native::WGPUShaderModuleDescriptor,
    spirv: Option<&native::WGPUShaderModuleSPIRVDescriptor>,
    wgsl: Option<&native::WGPUShaderModuleWGSLDescriptor>,
    glsl: Option<&native::WGPUShaderModuleGLSLDescriptor>,
) -> wgc::pipeline::ShaderModuleSource<'a> {
    if let Some(wgsl) = wgsl {
        let c_str: &CStr = unsafe { CStr::from_ptr(wgsl.code) };
        let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
        wgc::pipeline::ShaderModuleSource::Wgsl(Cow::Borrowed(str_slice))
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
        wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))
    } else if let Some(glsl) = glsl {
        let c_str: &CStr = unsafe { CStr::from_ptr(glsl.code) };
        let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
        let mut options = naga::front::glsl::Options::from(
            map_shader_stage(glsl.stage).expect("Unknown shader stage"),
        );

        let raw_defines = unsafe { slice::from_raw_parts(glsl.defines, glsl.defineCount as usize) };
        for define in raw_defines {
            let name_c_str: &CStr = unsafe { CStr::from_ptr(define.name) };
            let name_str_slice: &str = name_c_str.to_str().expect("not a valid utf-8 string");

            let value_c_str: &CStr = unsafe { CStr::from_ptr(define.value) };
            let value_str_slice: &str = value_c_str.to_str().expect("not a valid utf-8 string");

            options
                .defines
                .insert(String::from(name_str_slice), String::from(value_str_slice));
        }

        let mut parser = naga::front::glsl::Parser::default();
        let module = parser.parse(&options, str_slice).unwrap();
        wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))
    } else {
        panic!("Shader not provided.");
    }
}

pub fn map_image_copy_texture(
    native: &native::WGPUImageCopyTexture,
) -> wgc::command::ImageCopyTexture {
    wgt::ImageCopyTexture {
        texture: native
            .texture
            .as_option()
            .expect("invalid texture for image copy texture"),
        mip_level: native.mipLevel,
        origin: map_origin3d(&native.origin),
        aspect: map_texture_aspect(native.aspect),
    }
}

pub fn map_image_copy_buffer(
    native: &native::WGPUImageCopyBuffer,
) -> wgc::command::ImageCopyBuffer {
    wgt::ImageCopyBuffer {
        buffer: native
            .buffer
            .as_option()
            .expect("invalid buffer for image copy buffer"),
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

pub fn map_blend_component(native: native::WGPUBlendComponent) -> wgt::BlendComponent {
    wgt::BlendComponent {
        src_factor: map_blend_factor(native.srcFactor),
        dst_factor: map_blend_factor(native.dstFactor),
        operation: map_blend_operation(native.operation),
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

#[rustfmt::skip]
pub fn map_texture_format(value: native::WGPUTextureFormat) -> Option<wgt::TextureFormat> {
    use wgt::{AstcBlock, AstcChannel};

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
        native::WGPUTextureFormat_RG11B10Ufloat => Some(wgt::TextureFormat::Rg11b10Float), // close enough?
        native::WGPUTextureFormat_RGB9E5Ufloat => Some(wgt::TextureFormat::Rgb9e5Ufloat),
        native::WGPUTextureFormat_RG32Float => Some(wgt::TextureFormat::Rg32Float),
        native::WGPUTextureFormat_RG32Uint => Some(wgt::TextureFormat::Rg32Uint),
        native::WGPUTextureFormat_RG32Sint => Some(wgt::TextureFormat::Rg32Sint),
        native::WGPUTextureFormat_RGBA16Uint => Some(wgt::TextureFormat::Rgba16Uint),
        native::WGPUTextureFormat_RGBA16Sint => Some(wgt::TextureFormat::Rgba16Sint),
        native::WGPUTextureFormat_RGBA16Float => Some(wgt::TextureFormat::Rgba16Float),
        native::WGPUTextureFormat_RGBA32Float => Some(wgt::TextureFormat::Rgba32Float),
        native::WGPUTextureFormat_RGBA32Uint => Some(wgt::TextureFormat::Rgba32Uint),
        native::WGPUTextureFormat_RGBA32Sint => Some(wgt::TextureFormat::Rgba32Sint),
        native::WGPUTextureFormat_Stencil8 => None, // unimplmented in wgpu-core
        native::WGPUTextureFormat_Depth16Unorm => Some(wgt::TextureFormat::Depth16Unorm),
        native::WGPUTextureFormat_Depth24Plus => Some(wgt::TextureFormat::Depth24Plus),
        native::WGPUTextureFormat_Depth24PlusStencil8 => Some(wgt::TextureFormat::Depth24PlusStencil8),
        native::WGPUTextureFormat_Depth24UnormStencil8 =>  None, // unimplmented in wgpu-core
        native::WGPUTextureFormat_Depth32Float => Some(wgt::TextureFormat::Depth32Float),
        native::WGPUTextureFormat_Depth32FloatStencil8 => Some(wgt::TextureFormat::Depth32FloatStencil8),
        native::WGPUTextureFormat_BC1RGBAUnorm => Some(wgt::TextureFormat::Bc1RgbaUnorm),
        native::WGPUTextureFormat_BC1RGBAUnormSrgb => Some(wgt::TextureFormat::Bc1RgbaUnormSrgb),
        native::WGPUTextureFormat_BC2RGBAUnorm => Some(wgt::TextureFormat::Bc2RgbaUnorm),
        native::WGPUTextureFormat_BC2RGBAUnormSrgb => Some(wgt::TextureFormat::Bc2RgbaUnormSrgb),
        native::WGPUTextureFormat_BC3RGBAUnorm => Some(wgt::TextureFormat::Bc3RgbaUnorm),
        native::WGPUTextureFormat_BC3RGBAUnormSrgb => Some(wgt::TextureFormat::Bc3RgbaUnormSrgb),
        native::WGPUTextureFormat_BC4RUnorm => Some(wgt::TextureFormat::Bc4RUnorm),
        native::WGPUTextureFormat_BC4RSnorm => Some(wgt::TextureFormat::Bc4RSnorm),
        native::WGPUTextureFormat_BC5RGUnorm => Some(wgt::TextureFormat::Bc5RgUnorm),
        native::WGPUTextureFormat_BC5RGSnorm => Some(wgt::TextureFormat::Bc5RgSnorm),
        native::WGPUTextureFormat_BC6HRGBUfloat => Some(wgt::TextureFormat::Bc6hRgbUfloat),
        native::WGPUTextureFormat_BC6HRGBFloat => Some(wgt::TextureFormat::Bc6hRgbSfloat),
        native::WGPUTextureFormat_BC7RGBAUnorm => Some(wgt::TextureFormat::Bc7RgbaUnorm),
        native::WGPUTextureFormat_BC7RGBAUnormSrgb => Some(wgt::TextureFormat::Bc7RgbaUnormSrgb),
        native::WGPUTextureFormat_ETC2RGB8Unorm => Some(wgt::TextureFormat::Etc2Rgb8Unorm),
        native::WGPUTextureFormat_ETC2RGB8UnormSrgb => Some(wgt::TextureFormat::Etc2Rgb8UnormSrgb),
        native::WGPUTextureFormat_ETC2RGB8A1Unorm => Some(wgt::TextureFormat::Etc2Rgb8A1Unorm),
        native::WGPUTextureFormat_ETC2RGB8A1UnormSrgb => Some(wgt::TextureFormat::Etc2Rgb8A1UnormSrgb),
        native::WGPUTextureFormat_ETC2RGBA8Unorm => Some(wgt::TextureFormat::Etc2Rgba8Unorm),
        native::WGPUTextureFormat_ETC2RGBA8UnormSrgb => Some(wgt::TextureFormat::Etc2Rgba8UnormSrgb),
        native::WGPUTextureFormat_EACR11Unorm => Some(wgt::TextureFormat::EacR11Unorm),
        native::WGPUTextureFormat_EACR11Snorm => Some(wgt::TextureFormat::EacR11Snorm),
        native::WGPUTextureFormat_EACRG11Unorm => Some(wgt::TextureFormat::EacRg11Unorm),
        native::WGPUTextureFormat_EACRG11Snorm => Some(wgt::TextureFormat::EacRg11Snorm),
        native::WGPUTextureFormat_ASTC4x4Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC4x4UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC5x4Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC5x4UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC5x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC5x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC6x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC6x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC6x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC6x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x8Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x8UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x8Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x8UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x10Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x10UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC12x10Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC12x10UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC12x12Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC12x12UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::UnormSrgb }),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn to_native_texture_format(rs_type: wgt::TextureFormat) -> native::WGPUTextureFormat {
    use wgt::{AstcBlock, AstcChannel};

    // TODO: unimplemented in wgpu-core
    // native::WGPUTextureFormat_Stencil8,
    // native::WGPUTextureFormat_Depth16Unorm

    match rs_type {
        // unimplemented in webgpu.h
        wgt::TextureFormat::R16Unorm => todo!(),
        wgt::TextureFormat::R16Snorm => todo!(),
        wgt::TextureFormat::Rg16Unorm => todo!(),
        wgt::TextureFormat::Rg16Snorm => todo!(),
        wgt::TextureFormat::Rgba16Unorm => todo!(),
        wgt::TextureFormat::Rgba16Snorm => todo!(),
        wgt::TextureFormat::Astc { block: _, channel: AstcChannel::Hdr } => todo!(),

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
        wgt::TextureFormat::Rg11b10Float => native::WGPUTextureFormat_RG11B10Ufloat, // close enough?
        wgt::TextureFormat::Rgb9e5Ufloat => native::WGPUTextureFormat_RGB9E5Ufloat,
        wgt::TextureFormat::Rg32Float => native::WGPUTextureFormat_RG32Float,
        wgt::TextureFormat::Rg32Uint => native::WGPUTextureFormat_RG32Uint,
        wgt::TextureFormat::Rg32Sint => native::WGPUTextureFormat_RG32Sint,
        wgt::TextureFormat::Rgba16Uint => native::WGPUTextureFormat_RGBA16Uint,
        wgt::TextureFormat::Rgba16Sint => native::WGPUTextureFormat_RGBA16Sint,
        wgt::TextureFormat::Rgba16Float => native::WGPUTextureFormat_RGBA16Float,
        wgt::TextureFormat::Rgba32Float => native::WGPUTextureFormat_RGBA32Float,
        wgt::TextureFormat::Rgba32Uint => native::WGPUTextureFormat_RGBA32Uint,
        wgt::TextureFormat::Rgba32Sint => native::WGPUTextureFormat_RGBA32Sint,
        wgt::TextureFormat::Depth16Unorm => native::WGPUTextureFormat_Depth16Unorm,
        wgt::TextureFormat::Depth24Plus => native::WGPUTextureFormat_Depth24Plus,
        wgt::TextureFormat::Depth24PlusStencil8 => native::WGPUTextureFormat_Depth24PlusStencil8,
        wgt::TextureFormat::Depth32Float => native::WGPUTextureFormat_Depth32Float,
        wgt::TextureFormat::Depth32FloatStencil8 => native::WGPUTextureFormat_Depth32FloatStencil8,
        wgt::TextureFormat::Bc1RgbaUnorm => native::WGPUTextureFormat_BC1RGBAUnorm,
        wgt::TextureFormat::Bc1RgbaUnormSrgb => native::WGPUTextureFormat_BC1RGBAUnormSrgb,
        wgt::TextureFormat::Bc2RgbaUnorm => native::WGPUTextureFormat_BC2RGBAUnorm,
        wgt::TextureFormat::Bc2RgbaUnormSrgb => native::WGPUTextureFormat_BC2RGBAUnormSrgb,
        wgt::TextureFormat::Bc3RgbaUnorm => native::WGPUTextureFormat_BC3RGBAUnorm,
        wgt::TextureFormat::Bc3RgbaUnormSrgb => native::WGPUTextureFormat_BC3RGBAUnormSrgb,
        wgt::TextureFormat::Bc4RUnorm => native::WGPUTextureFormat_BC4RUnorm,
        wgt::TextureFormat::Bc4RSnorm => native::WGPUTextureFormat_BC4RSnorm,
        wgt::TextureFormat::Bc5RgUnorm => native::WGPUTextureFormat_BC5RGUnorm,
        wgt::TextureFormat::Bc5RgSnorm => native::WGPUTextureFormat_BC5RGSnorm,
        wgt::TextureFormat::Bc6hRgbUfloat => native::WGPUTextureFormat_BC6HRGBUfloat,
        wgt::TextureFormat::Bc6hRgbSfloat => native::WGPUTextureFormat_BC6HRGBFloat,
        wgt::TextureFormat::Bc7RgbaUnorm => native::WGPUTextureFormat_BC7RGBAUnorm,
        wgt::TextureFormat::Bc7RgbaUnormSrgb => native::WGPUTextureFormat_BC7RGBAUnormSrgb,
        wgt::TextureFormat::Etc2Rgb8Unorm => native::WGPUTextureFormat_ETC2RGB8Unorm,
        wgt::TextureFormat::Etc2Rgb8UnormSrgb => native::WGPUTextureFormat_ETC2RGB8UnormSrgb,
        wgt::TextureFormat::Etc2Rgb8A1Unorm => native::WGPUTextureFormat_ETC2RGB8A1Unorm,
        wgt::TextureFormat::Etc2Rgb8A1UnormSrgb => native::WGPUTextureFormat_ETC2RGB8A1UnormSrgb,
        wgt::TextureFormat::Etc2Rgba8Unorm => native::WGPUTextureFormat_ETC2RGBA8Unorm,
        wgt::TextureFormat::Etc2Rgba8UnormSrgb => native::WGPUTextureFormat_ETC2RGBA8UnormSrgb,
        wgt::TextureFormat::EacR11Unorm => native::WGPUTextureFormat_EACR11Unorm,
        wgt::TextureFormat::EacR11Snorm => native::WGPUTextureFormat_EACR11Snorm,
        wgt::TextureFormat::EacRg11Unorm => native::WGPUTextureFormat_EACRG11Unorm,
        wgt::TextureFormat::EacRg11Snorm => native::WGPUTextureFormat_EACRG11Snorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC4x4Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC4x4UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC5x4Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC5x4UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC5x5Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC5x5UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC6x5Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC6x5UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC6x6Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC6x6UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC8x5Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC8x5UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC8x6Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC8x6UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC8x8Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC8x8UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC10x5Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC10x5UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC10x6Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC10x6UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC10x8Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC10x8UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC10x10Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC10x10UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC12x10Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC12x10UnormSrgb,
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::Unorm } => native::WGPUTextureFormat_ASTC12x12Unorm,
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::UnormSrgb } => native::WGPUTextureFormat_ASTC12x12UnormSrgb,
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
pub fn write_global_report(
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

pub fn map_features(features: &[native::WGPUFeatureName]) -> wgt::Features {
    let mut temp = wgt::Features::empty();

    features.iter().for_each(|f| {
        if let Some(feature) = map_feature(*f) {
            temp.insert(feature);
        };
    });

    temp
}

pub fn features_to_native(features: wgt::Features) -> Vec<native::WGPUFeatureName> {
    let mut temp = Vec::new();

    if features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
        temp.push(native::WGPUFeatureName_DepthClipControl);
    }
    if features.contains(wgt::Features::DEPTH32FLOAT_STENCIL8) {
        temp.push(native::WGPUFeatureName_Depth32FloatStencil8);
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY) {
        temp.push(native::WGPUFeatureName_TimestampQuery);
    }
    if features.contains(wgt::Features::PIPELINE_STATISTICS_QUERY) {
        temp.push(native::WGPUFeatureName_PipelineStatisticsQuery);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_BC) {
        temp.push(native::WGPUFeatureName_TextureCompressionBC);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ETC2) {
        temp.push(native::WGPUFeatureName_TextureCompressionETC2);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC_LDR) {
        temp.push(native::WGPUFeatureName_TextureCompressionASTC);
    }
    if features.contains(wgt::Features::INDIRECT_FIRST_INSTANCE) {
        temp.push(native::WGPUFeatureName_IndirectFirstInstance);
    }

    // native only features
    if features.contains(wgt::Features::PUSH_CONSTANTS) {
        temp.push(native::WGPUNativeFeature_PUSH_CONSTANTS);
    }
    if features.contains(wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        temp.push(native::WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT) {
        temp.push(native::WGPUNativeFeature_MULTI_DRAW_INDIRECT);
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT_COUNT) {
        temp.push(native::WGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT);
    }
    if features.contains(wgt::Features::VERTEX_WRITABLE_STORAGE) {
        temp.push(native::WGPUNativeFeature_VERTEX_WRITABLE_STORAGE);
    }

    temp
}

#[rustfmt::skip]
pub fn map_feature(feature: native::WGPUFeatureName) -> Option<wgt::Features> {
    use wgt::Features;

    match feature {
        native::WGPUFeatureName_DepthClipControl => Some(Features::DEPTH_CLIP_CONTROL),
        native::WGPUFeatureName_Depth32FloatStencil8 => Some(Features::DEPTH32FLOAT_STENCIL8),
        native::WGPUFeatureName_TimestampQuery => Some(Features::TIMESTAMP_QUERY),
        native::WGPUFeatureName_PipelineStatisticsQuery => Some(Features::PIPELINE_STATISTICS_QUERY),
        native::WGPUFeatureName_TextureCompressionBC => Some(Features::TEXTURE_COMPRESSION_BC),
        native::WGPUFeatureName_TextureCompressionETC2 => Some(Features::TEXTURE_COMPRESSION_ETC2),
        native::WGPUFeatureName_TextureCompressionASTC => Some(Features::TEXTURE_COMPRESSION_ASTC_LDR),
        native::WGPUFeatureName_IndirectFirstInstance => Some(Features::INDIRECT_FIRST_INSTANCE),

        // native only features
        native::WGPUNativeFeature_PUSH_CONSTANTS => Some(Features::PUSH_CONSTANTS),
        native::WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES => Some(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
        native::WGPUNativeFeature_MULTI_DRAW_INDIRECT => Some(Features::MULTI_DRAW_INDIRECT),
        native::WGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT => Some(Features::MULTI_DRAW_INDIRECT_COUNT),
        native::WGPUNativeFeature_VERTEX_WRITABLE_STORAGE => Some(Features::VERTEX_WRITABLE_STORAGE),

        _ => None,
    }
}
