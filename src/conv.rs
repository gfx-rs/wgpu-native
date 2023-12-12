use crate::native;
use crate::utils::{make_slice, ptr_into_label, ptr_into_pathbuf};
use crate::{follow_chain, map_enum};
use std::num::{NonZeroIsize, NonZeroU32, NonZeroU64};
use std::ptr::NonNull;
use std::{borrow::Cow, ffi::CStr};

map_enum!(map_load_op, WGPULoadOp, wgc::command::LoadOp, Clear, Load);
map_enum!(
    map_store_op,
    WGPUStoreOp,
    wgc::command::StoreOp,
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
    Fifo,
    FifoRelaxed
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

#[cfg(feature = "glsl")]
map_enum!(
    map_shader_stage,
    WGPUShaderStage,
    naga::ShaderStage,
    Vertex,
    Fragment,
    Compute
);

map_enum!(
    map_composite_alpha_mode,
    WGPUCompositeAlphaMode,
    wgt::CompositeAlphaMode,
    Auto: Auto,
    Opaque: Opaque,
    Premultiplied: PreMultiplied,
    Unpremultiplied: PostMultiplied,
    Inherit: Inherit
);

map_enum!(
    map_gles3_minor_version,
    WGPUGles3MinorVersion,
    wgt::Gles3MinorVersion,
    "Unknown gles3 minor version",
    Automatic,
    Version0,
    Version1,
    Version2
);

map_enum!(
    map_storage_texture_access,
    WGPUStorageTextureAccess,
    wgt::StorageTextureAccess,
    WriteOnly,
    ReadOnly,
    ReadWrite
);

pub const WGPU_WHOLE_SIZE: ::std::os::raw::c_ulonglong = native::WGPU_WHOLE_SIZE as _;
pub const WGPU_LIMIT_U64_UNDEFINED: ::std::os::raw::c_ulonglong =
    native::WGPU_LIMIT_U64_UNDEFINED as _;
// it's SIZE_MAX in headers but it's not available in some compilers
pub const WGPU_WHOLE_MAP_SIZE: usize = usize::MAX;

#[inline]
pub fn map_extent3d(native: &native::WGPUExtent3D) -> wgt::Extent3d {
    wgt::Extent3d {
        width: native.width,
        height: native.height,
        depth_or_array_layers: native.depthOrArrayLayers,
    }
}

#[inline]
pub fn map_origin3d(native: &native::WGPUOrigin3D) -> wgt::Origin3d {
    wgt::Origin3d {
        x: native.x,
        y: native.y,
        z: native.z,
    }
}

#[inline]
pub fn map_instance_backend_flags(flags: native::WGPUInstanceBackend) -> wgt::Backends {
    let mut result = wgt::Backends::empty();
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
    result
}

#[inline]
pub fn map_instance_flags(flags: native::WGPUInstanceFlag) -> wgt::InstanceFlags {
    let mut result = wgt::InstanceFlags::empty();
    if (flags & native::WGPUInstanceFlag_Debug) != 0 {
        result.insert(wgt::InstanceFlags::DEBUG);
    }
    if (flags & native::WGPUInstanceFlag_Validation) != 0 {
        result.insert(wgt::InstanceFlags::VALIDATION);
    }
    if (flags & native::WGPUInstanceFlag_DiscardHalLabels) != 0 {
        result.insert(wgt::InstanceFlags::DISCARD_HAL_LABELS);
    }
    result
}

#[inline]
pub fn map_instance_descriptor(
    _base: &native::WGPUInstanceDescriptor,
    extras: Option<&native::WGPUInstanceExtras>,
) -> wgt::InstanceDescriptor {
    if let Some(extras) = extras {
        let dx12_shader_compiler = match extras.dx12ShaderCompiler {
            native::WGPUDx12Compiler_Fxc => wgt::Dx12Compiler::Fxc,
            native::WGPUDx12Compiler_Dxc => wgt::Dx12Compiler::Dxc {
                dxil_path: ptr_into_pathbuf(extras.dxilPath),
                dxc_path: ptr_into_pathbuf(extras.dxcPath),
            },
            _ => wgt::Dx12Compiler::default(),
        };

        wgt::InstanceDescriptor {
            backends: match extras.backends as native::WGPUInstanceBackend {
                native::WGPUInstanceBackend_All => wgt::Backends::all(),
                backends => map_instance_backend_flags(backends),
            },
            dx12_shader_compiler,
            gles_minor_version: map_gles3_minor_version(extras.gles3MinorVersion),
            flags: match extras.flags as native::WGPUInstanceFlag {
                native::WGPUInstanceFlag_Default => wgt::InstanceFlags::default(),
                flags => map_instance_flags(flags),
            },
        }
    } else {
        wgt::InstanceDescriptor::default()
    }
}

#[inline]
pub fn map_device_descriptor<'a>(
    des: &native::WGPUDeviceDescriptor,
    base_limits: wgt::Limits,
    extras: Option<&native::WGPUDeviceExtras>,
) -> (
    wgt::DeviceDescriptor<wgc::Label<'a>>,
    *const std::ffi::c_char,
) {
    (
        wgt::DeviceDescriptor {
            label: ptr_into_label(des.label),
            required_features: map_features(make_slice(
                des.requiredFeatures,
                des.requiredFeatureCount,
            )),
            required_limits: match unsafe { des.requiredLimits.as_ref() } {
                Some(required_limits) => unsafe {
                    follow_chain!(
                        map_required_limits((required_limits, base_limits),
                        WGPUSType_RequiredLimitsExtras => native::WGPURequiredLimitsExtras)
                    )
                },
                None => base_limits,
            },
        },
        match extras {
            Some(extras) => extras.tracePath,
            None => std::ptr::null(),
        },
    )
}

#[inline]
pub unsafe fn map_pipeline_layout_descriptor<'a>(
    des: &native::WGPUPipelineLayoutDescriptor,
    extras: Option<&native::WGPUPipelineLayoutExtras>,
) -> wgc::binding_model::PipelineLayoutDescriptor<'a> {
    let bind_group_layouts = make_slice(des.bindGroupLayouts, des.bindGroupLayoutCount)
        .iter()
        .map(|layout| {
            layout
                .as_ref()
                .expect("invalid bind group layout for pipeline layout descriptor")
                .id
        })
        .collect::<Vec<_>>();

    let push_constant_ranges = extras.map_or(Vec::new(), |extras| {
        make_slice(
            extras.pushConstantRanges,
            extras.pushConstantRangeCount as usize,
        )
        .iter()
        .map(|range| wgt::PushConstantRange {
            stages: wgt::ShaderStages::from_bits(range.stages)
                .expect("invalid shader stage for push constant range"),
            range: range.start..range.end,
        })
        .collect()
    });

    return wgc::binding_model::PipelineLayoutDescriptor {
        label: ptr_into_label(des.label),
        bind_group_layouts: Cow::from(bind_group_layouts),
        push_constant_ranges: Cow::from(push_constant_ranges),
    };
}

#[inline]
pub fn write_limits_struct(
    wgt_limits: wgt::Limits,
    supported_limits: &mut native::WGPUSupportedLimits,
) {
    let mut limits = supported_limits.limits;
    limits.maxTextureDimension1D = wgt_limits.max_texture_dimension_1d;
    limits.maxTextureDimension2D = wgt_limits.max_texture_dimension_2d;
    limits.maxTextureDimension3D = wgt_limits.max_texture_dimension_3d;
    limits.maxTextureArrayLayers = wgt_limits.max_texture_array_layers;
    limits.maxBindGroups = wgt_limits.max_bind_groups;
    // TODO: not yet in wgt
    // limits.maxBindGroupsPlusVertexBuffers = wgt_limits.max_bind_groups_plus_vertex_buffers;
    limits.maxBindingsPerBindGroup = wgt_limits.max_bindings_per_bind_group;
    limits.maxDynamicUniformBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout;
    limits.maxDynamicStorageBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout;
    limits.maxSampledTexturesPerShaderStage = wgt_limits.max_sampled_textures_per_shader_stage;
    limits.maxSamplersPerShaderStage = wgt_limits.max_samplers_per_shader_stage;
    limits.maxStorageBuffersPerShaderStage = wgt_limits.max_storage_buffers_per_shader_stage;
    limits.maxStorageTexturesPerShaderStage = wgt_limits.max_storage_textures_per_shader_stage;
    limits.maxUniformBuffersPerShaderStage = wgt_limits.max_uniform_buffers_per_shader_stage;
    limits.maxUniformBufferBindingSize = wgt_limits.max_uniform_buffer_binding_size as _;
    limits.maxStorageBufferBindingSize = wgt_limits.max_storage_buffer_binding_size as _;
    limits.maxVertexBuffers = wgt_limits.max_vertex_buffers;
    limits.maxBufferSize = wgt_limits.max_buffer_size;
    limits.maxVertexAttributes = wgt_limits.max_vertex_attributes;
    limits.maxVertexBufferArrayStride = wgt_limits.max_vertex_buffer_array_stride;
    limits.minUniformBufferOffsetAlignment = wgt_limits.min_uniform_buffer_offset_alignment;
    limits.minStorageBufferOffsetAlignment = wgt_limits.min_storage_buffer_offset_alignment;
    limits.maxInterStageShaderComponents = wgt_limits.max_inter_stage_shader_components;
    // TODO: not yet in wgt
    // limits.maxInterStageShaderVariables = wgt_limits.max_inter_stage_shader_variables;
    // TODO: not yet in wgt
    // limits.maxColorAttachments = wgt_limits.max_color_attachments;
    // TODO: not yet in wgt
    // limits.maxColorAttachmentBytesPerSample = wgt_limits.max_color_attachment_bytes_per_sample;
    limits.maxComputeWorkgroupStorageSize = wgt_limits.max_compute_workgroup_storage_size;
    limits.maxComputeInvocationsPerWorkgroup = wgt_limits.max_compute_invocations_per_workgroup;
    limits.maxComputeWorkgroupSizeX = wgt_limits.max_compute_workgroup_size_x;
    limits.maxComputeWorkgroupSizeY = wgt_limits.max_compute_workgroup_size_y;
    limits.maxComputeWorkgroupSizeZ = wgt_limits.max_compute_workgroup_size_z;
    limits.maxComputeWorkgroupsPerDimension = wgt_limits.max_compute_workgroups_per_dimension;
    supported_limits.limits = limits;

    if let Some(native::WGPUChainedStructOut {
        sType: native::WGPUSType_SupportedLimitsExtras,
        ..
    }) = unsafe { supported_limits.nextInChain.as_ref() }
    {
        unsafe {
            let extras = std::mem::transmute::<
                *mut native::WGPUChainedStructOut,
                *mut native::WGPUSupportedLimitsExtras,
            >(supported_limits.nextInChain);
            (*extras).limits = native::WGPUNativeLimits {
                maxPushConstantSize: wgt_limits.max_push_constant_size,
                maxNonSamplerBindings: wgt_limits.max_non_sampler_bindings,
            };
        }
    };
}

#[inline]
pub fn map_required_limits(
    required_limits: &native::WGPURequiredLimits,
    base_limits: wgt::Limits,
    extras: Option<&native::WGPURequiredLimitsExtras>,
) -> wgt::Limits {
    let limits = required_limits.limits;
    let mut wgt_limits = base_limits;
    if limits.maxTextureDimension1D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_1d = limits.maxTextureDimension1D;
    }
    if limits.maxTextureDimension2D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_2d = limits.maxTextureDimension2D;
    }
    if limits.maxTextureDimension3D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_3d = limits.maxTextureDimension3D;
    }
    if limits.maxTextureArrayLayers != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_array_layers = limits.maxTextureArrayLayers;
    }
    if limits.maxBindGroups != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_bind_groups = limits.maxBindGroups;
    }
    // TODO: not yet in wgt
    // if limits.maxBindGroupsPlusVertexBuffers != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_bind_groups_plus_vertex_buffers = limits.maxBindGroupsPlusVertexBuffers;
    // }
    if limits.maxBindingsPerBindGroup != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_bindings_per_bind_group = limits.maxBindingsPerBindGroup;
    }
    if limits.maxDynamicUniformBuffersPerPipelineLayout != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout =
            limits.maxDynamicUniformBuffersPerPipelineLayout;
    }
    if limits.maxDynamicStorageBuffersPerPipelineLayout != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout =
            limits.maxDynamicStorageBuffersPerPipelineLayout;
    }
    if limits.maxSampledTexturesPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_sampled_textures_per_shader_stage = limits.maxSampledTexturesPerShaderStage;
    }
    if limits.maxSamplersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_samplers_per_shader_stage = limits.maxSamplersPerShaderStage;
    }
    if limits.maxStorageBuffersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_storage_buffers_per_shader_stage = limits.maxStorageBuffersPerShaderStage;
    }
    if limits.maxStorageTexturesPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_storage_textures_per_shader_stage = limits.maxStorageTexturesPerShaderStage;
    }
    if limits.maxUniformBuffersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_uniform_buffers_per_shader_stage = limits.maxUniformBuffersPerShaderStage;
    }
    if limits.maxUniformBufferBindingSize != native::WGPU_LIMIT_U64_UNDEFINED as u64 {
        wgt_limits.max_uniform_buffer_binding_size = limits.maxUniformBufferBindingSize as u32;
    }
    if limits.maxStorageBufferBindingSize != native::WGPU_LIMIT_U64_UNDEFINED as u64 {
        wgt_limits.max_storage_buffer_binding_size = limits.maxStorageBufferBindingSize as u32;
    }
    if limits.minUniformBufferOffsetAlignment != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.min_uniform_buffer_offset_alignment = limits.minUniformBufferOffsetAlignment;
    }
    if limits.minStorageBufferOffsetAlignment != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.min_storage_buffer_offset_alignment = limits.minStorageBufferOffsetAlignment;
    }
    if limits.maxVertexBuffers != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_buffers = limits.maxVertexBuffers;
    }
    if limits.maxBufferSize != native::WGPU_LIMIT_U64_UNDEFINED as u64 {
        wgt_limits.max_buffer_size = limits.maxBufferSize;
    }
    if limits.maxVertexAttributes != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_attributes = limits.maxVertexAttributes;
    }
    if limits.maxVertexBufferArrayStride != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_buffer_array_stride = limits.maxVertexBufferArrayStride;
    }
    if limits.maxInterStageShaderComponents != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_inter_stage_shader_components = limits.maxInterStageShaderComponents;
    }
    // TODO: not yet in wgt
    // if limits.maxInterStageShaderVariables != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_inter_stage_shader_variables = limits.maxInterStageShaderVariables;
    // }
    // TODO: not yet in wgt
    // if limits.maxColorAttachments != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_color_attachments = limits.maxColorAttachments;
    // }
    // TODO: not yet in wgt
    // if limits.maxColorAttachmentBytesPerSample != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_color_attachment_bytes_per_sample = limits.maxColorAttachmentBytesPerSample;
    // }
    if limits.maxComputeWorkgroupStorageSize != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_storage_size = limits.maxComputeWorkgroupStorageSize;
    }
    if limits.maxComputeInvocationsPerWorkgroup != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_invocations_per_workgroup = limits.maxComputeInvocationsPerWorkgroup;
    }
    if limits.maxComputeWorkgroupSizeX != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_x = limits.maxComputeWorkgroupSizeX;
    }
    if limits.maxComputeWorkgroupSizeY != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_y = limits.maxComputeWorkgroupSizeY;
    }
    if limits.maxComputeWorkgroupSizeZ != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_z = limits.maxComputeWorkgroupSizeZ;
    }
    if limits.maxComputeWorkgroupsPerDimension != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroups_per_dimension = limits.maxComputeWorkgroupsPerDimension;
    }
    if let Some(extras) = extras {
        let limits = extras.limits;
        if limits.maxPushConstantSize != native::WGPU_LIMIT_U32_UNDEFINED {
            wgt_limits.max_push_constant_size = limits.maxPushConstantSize;
        }
        if limits.maxNonSamplerBindings != native::WGPU_LIMIT_U32_UNDEFINED {
            wgt_limits.max_non_sampler_bindings = limits.maxNonSamplerBindings;
        }
    }
    wgt_limits
}

#[derive(Debug, thiserror::Error)]
pub enum ShaderParseError {
    #[error(transparent)]
    Spirv(#[from] naga::front::spv::Error),
    #[error("GLSL Parse Error: {0:?}")]
    Glsl(Vec<naga::front::glsl::Error>),
}

#[inline]
pub fn map_shader_module<'a>(
    _: &native::WGPUShaderModuleDescriptor,
    spirv: Option<&native::WGPUShaderModuleSPIRVDescriptor>,
    wgsl: Option<&native::WGPUShaderModuleWGSLDescriptor>,
    glsl: Option<&native::WGPUShaderModuleGLSLDescriptor>,
) -> Result<wgc::pipeline::ShaderModuleSource<'a>, ShaderParseError> {
    #[cfg(feature = "wgsl")]
    if let Some(wgsl) = wgsl {
        let c_str: &CStr = unsafe { CStr::from_ptr(wgsl.code) };
        let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
        return Ok(wgc::pipeline::ShaderModuleSource::Wgsl(Cow::Borrowed(
            str_slice,
        )));
    }

    #[cfg(feature = "spirv")]
    if let Some(spirv) = spirv {
        let slice = make_slice(spirv.code, spirv.codeSize as usize);
        // Parse the given shader code and store its representation.
        let options = naga::front::spv::Options {
            adjust_coordinate_space: false, // we require NDC_Y_UP feature
            strict_capabilities: true,
            block_ctx_dump_prefix: None,
        };
        let frontend = naga::front::spv::Frontend::new(slice.iter().cloned(), &options);
        match frontend.parse() {
            Ok(module) => return Ok(wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))),
            Err(cause) => return Err(ShaderParseError::Spirv(cause)),
        };
    }

    #[cfg(feature = "glsl")]
    if let Some(glsl) = glsl {
        let c_str: &CStr = unsafe { CStr::from_ptr(glsl.code) };
        let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
        let mut options = naga::front::glsl::Options::from(
            map_shader_stage(glsl.stage)
                .expect("invalid shader stage for shader module glsl descriptor"),
        );

        let raw_defines = make_slice(glsl.defines, glsl.defineCount as usize);
        for define in raw_defines {
            let name_c_str: &CStr = unsafe { CStr::from_ptr(define.name) };
            let name_str_slice: &str = name_c_str.to_str().expect("not a valid utf-8 string");

            let value_c_str: &CStr = unsafe { CStr::from_ptr(define.value) };
            let value_str_slice: &str = value_c_str.to_str().expect("not a valid utf-8 string");

            options
                .defines
                .insert(String::from(name_str_slice), String::from(value_str_slice));
        }

        let mut frontend = naga::front::glsl::Frontend::default();
        match frontend.parse(&options, str_slice) {
            Ok(module) => return Ok(wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))),
            Err(causes) => return Err(ShaderParseError::Glsl(causes)),
        };
    }

    panic!("Shader not provided.");
}

#[inline]
pub unsafe fn map_image_copy_texture(
    native: &native::WGPUImageCopyTexture,
) -> wgc::command::ImageCopyTexture {
    wgt::ImageCopyTexture {
        texture: native
            .texture
            .as_ref()
            .expect("invalid texture for image copy texture")
            .id,
        mip_level: native.mipLevel,
        origin: map_origin3d(&native.origin),
        aspect: map_texture_aspect(native.aspect),
    }
}

#[inline]
pub unsafe fn map_image_copy_buffer(
    native: &native::WGPUImageCopyBuffer,
) -> wgc::command::ImageCopyBuffer {
    wgt::ImageCopyBuffer {
        buffer: native
            .buffer
            .as_ref()
            .expect("invalid buffer for image copy buffer")
            .id,
        layout: map_texture_data_layout(&native.layout),
    }
}

#[inline]
pub fn map_texture_data_layout(native: &native::WGPUTextureDataLayout) -> wgt::ImageDataLayout {
    wgt::ImageDataLayout {
        offset: native.offset,
        bytes_per_row: match native.bytesPerRow {
            0 => panic!("invalid bytesPerRow"),
            native::WGPU_COPY_STRIDE_UNDEFINED => None,
            _ => Some(native.bytesPerRow),
        },
        rows_per_image: match native.rowsPerImage {
            0 => panic!("invalid rowsPerImage"),
            native::WGPU_COPY_STRIDE_UNDEFINED => None,
            _ => Some(native.rowsPerImage),
        },
    }
}

#[inline]
pub fn map_color(native: &native::WGPUColor) -> wgt::Color {
    wgt::Color {
        r: native.r,
        g: native.g,
        b: native.b,
        a: native.a,
    }
}

#[inline]
pub fn map_blend_component(native: native::WGPUBlendComponent) -> wgt::BlendComponent {
    wgt::BlendComponent {
        src_factor: map_blend_factor(native.srcFactor),
        dst_factor: map_blend_factor(native.dstFactor),
        operation: map_blend_operation(native.operation),
    }
}

#[inline]
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

#[inline]
pub fn map_texture_dimension(value: native::WGPUTextureDimension) -> wgt::TextureDimension {
    match value {
        native::WGPUTextureDimension_1D => wgt::TextureDimension::D1,
        native::WGPUTextureDimension_2D => wgt::TextureDimension::D2,
        native::WGPUTextureDimension_3D => wgt::TextureDimension::D3,
        x => panic!("Unknown texture dimension: {x}"),
    }
}

#[rustfmt::skip]
#[inline]
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
        native::WGPUTextureFormat_RGB10A2Uint => Some(wgt::TextureFormat::Rgb10a2Uint),
        native::WGPUTextureFormat_RGB10A2Unorm => Some(wgt::TextureFormat::Rgb10a2Unorm),
        native::WGPUTextureFormat_RG11B10Ufloat => Some(wgt::TextureFormat::Rg11b10Float),
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
        native::WGPUTextureFormat_Stencil8 => Some(wgt::TextureFormat::Stencil8),
        native::WGPUTextureFormat_Depth16Unorm => Some(wgt::TextureFormat::Depth16Unorm),
        native::WGPUTextureFormat_Depth24Plus => Some(wgt::TextureFormat::Depth24Plus),
        native::WGPUTextureFormat_Depth24PlusStencil8 => Some(wgt::TextureFormat::Depth24PlusStencil8),
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
        native::WGPUTextureFormat_BC6HRGBFloat => Some(wgt::TextureFormat::Bc6hRgbFloat),
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
#[inline]
pub fn to_native_texture_format(rs_type: wgt::TextureFormat) -> Option<native::WGPUTextureFormat> {
    use wgt::{AstcBlock, AstcChannel};

    match rs_type {
        // unimplemented in webgpu.h
        wgt::TextureFormat::R16Unorm => None,
        wgt::TextureFormat::R16Snorm => None,
        wgt::TextureFormat::Rg16Unorm => None,
        wgt::TextureFormat::Rg16Snorm => None,
        wgt::TextureFormat::Rgba16Unorm => None,
        wgt::TextureFormat::Rgba16Snorm => None,
        wgt::TextureFormat::NV12 => None,
        wgt::TextureFormat::Astc { block:_, channel: AstcChannel::Hdr } => None,

        wgt::TextureFormat::R8Unorm => Some(native::WGPUTextureFormat_R8Unorm),
        wgt::TextureFormat::R8Snorm => Some(native::WGPUTextureFormat_R8Snorm),
        wgt::TextureFormat::R8Uint => Some(native::WGPUTextureFormat_R8Uint),
        wgt::TextureFormat::R8Sint => Some(native::WGPUTextureFormat_R8Sint),
        wgt::TextureFormat::R16Uint => Some(native::WGPUTextureFormat_R16Uint),
        wgt::TextureFormat::R16Sint => Some(native::WGPUTextureFormat_R16Sint),
        wgt::TextureFormat::R16Float => Some(native::WGPUTextureFormat_R16Float),
        wgt::TextureFormat::Rg8Unorm => Some(native::WGPUTextureFormat_RG8Unorm),
        wgt::TextureFormat::Rg8Snorm => Some(native::WGPUTextureFormat_RG8Snorm),
        wgt::TextureFormat::Rg8Uint => Some(native::WGPUTextureFormat_RG8Uint),
        wgt::TextureFormat::Rg8Sint => Some(native::WGPUTextureFormat_RG8Sint),
        wgt::TextureFormat::R32Float => Some(native::WGPUTextureFormat_R32Float),
        wgt::TextureFormat::R32Uint => Some(native::WGPUTextureFormat_R32Uint),
        wgt::TextureFormat::R32Sint => Some(native::WGPUTextureFormat_R32Sint),
        wgt::TextureFormat::Rg16Uint => Some(native::WGPUTextureFormat_RG16Uint),
        wgt::TextureFormat::Rg16Sint => Some(native::WGPUTextureFormat_RG16Sint),
        wgt::TextureFormat::Rg16Float => Some(native::WGPUTextureFormat_RG16Float),
        wgt::TextureFormat::Rgba8Unorm => Some(native::WGPUTextureFormat_RGBA8Unorm),
        wgt::TextureFormat::Rgba8UnormSrgb => Some(native::WGPUTextureFormat_RGBA8UnormSrgb),
        wgt::TextureFormat::Rgba8Snorm => Some(native::WGPUTextureFormat_RGBA8Snorm),
        wgt::TextureFormat::Rgba8Uint => Some(native::WGPUTextureFormat_RGBA8Uint),
        wgt::TextureFormat::Rgba8Sint => Some(native::WGPUTextureFormat_RGBA8Sint),
        wgt::TextureFormat::Bgra8Unorm => Some(native::WGPUTextureFormat_BGRA8Unorm),
        wgt::TextureFormat::Bgra8UnormSrgb => Some(native::WGPUTextureFormat_BGRA8UnormSrgb),
        wgt::TextureFormat::Rgb10a2Uint => Some(native::WGPUTextureFormat_RGB10A2Uint),
        wgt::TextureFormat::Rgb10a2Unorm => Some(native::WGPUTextureFormat_RGB10A2Unorm),
        wgt::TextureFormat::Rg11b10Float => Some(native::WGPUTextureFormat_RG11B10Ufloat),
        wgt::TextureFormat::Rgb9e5Ufloat => Some(native::WGPUTextureFormat_RGB9E5Ufloat),
        wgt::TextureFormat::Rg32Float => Some(native::WGPUTextureFormat_RG32Float),
        wgt::TextureFormat::Rg32Uint => Some(native::WGPUTextureFormat_RG32Uint),
        wgt::TextureFormat::Rg32Sint => Some(native::WGPUTextureFormat_RG32Sint),
        wgt::TextureFormat::Rgba16Uint => Some(native::WGPUTextureFormat_RGBA16Uint),
        wgt::TextureFormat::Rgba16Sint => Some(native::WGPUTextureFormat_RGBA16Sint),
        wgt::TextureFormat::Rgba16Float => Some(native::WGPUTextureFormat_RGBA16Float),
        wgt::TextureFormat::Rgba32Float => Some(native::WGPUTextureFormat_RGBA32Float),
        wgt::TextureFormat::Rgba32Uint => Some(native::WGPUTextureFormat_RGBA32Uint),
        wgt::TextureFormat::Rgba32Sint => Some(native::WGPUTextureFormat_RGBA32Sint),
        wgt::TextureFormat::Stencil8 => Some(native::WGPUTextureFormat_Stencil8),
        wgt::TextureFormat::Depth16Unorm => Some(native::WGPUTextureFormat_Depth16Unorm),
        wgt::TextureFormat::Depth24Plus => Some(native::WGPUTextureFormat_Depth24Plus),
        wgt::TextureFormat::Depth24PlusStencil8 => Some(native::WGPUTextureFormat_Depth24PlusStencil8),
        wgt::TextureFormat::Depth32Float => Some(native::WGPUTextureFormat_Depth32Float),
        wgt::TextureFormat::Depth32FloatStencil8 => Some(native::WGPUTextureFormat_Depth32FloatStencil8),
        wgt::TextureFormat::Bc1RgbaUnorm => Some(native::WGPUTextureFormat_BC1RGBAUnorm),
        wgt::TextureFormat::Bc1RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC1RGBAUnormSrgb),
        wgt::TextureFormat::Bc2RgbaUnorm => Some(native::WGPUTextureFormat_BC2RGBAUnorm),
        wgt::TextureFormat::Bc2RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC2RGBAUnormSrgb),
        wgt::TextureFormat::Bc3RgbaUnorm => Some(native::WGPUTextureFormat_BC3RGBAUnorm),
        wgt::TextureFormat::Bc3RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC3RGBAUnormSrgb),
        wgt::TextureFormat::Bc4RUnorm => Some(native::WGPUTextureFormat_BC4RUnorm),
        wgt::TextureFormat::Bc4RSnorm => Some(native::WGPUTextureFormat_BC4RSnorm),
        wgt::TextureFormat::Bc5RgUnorm => Some(native::WGPUTextureFormat_BC5RGUnorm),
        wgt::TextureFormat::Bc5RgSnorm => Some(native::WGPUTextureFormat_BC5RGSnorm),
        wgt::TextureFormat::Bc6hRgbUfloat => Some(native::WGPUTextureFormat_BC6HRGBUfloat),
        wgt::TextureFormat::Bc6hRgbFloat => Some(native::WGPUTextureFormat_BC6HRGBFloat),
        wgt::TextureFormat::Bc7RgbaUnorm => Some(native::WGPUTextureFormat_BC7RGBAUnorm),
        wgt::TextureFormat::Bc7RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC7RGBAUnormSrgb),
        wgt::TextureFormat::Etc2Rgb8Unorm => Some(native::WGPUTextureFormat_ETC2RGB8Unorm),
        wgt::TextureFormat::Etc2Rgb8UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGB8UnormSrgb),
        wgt::TextureFormat::Etc2Rgb8A1Unorm => Some(native::WGPUTextureFormat_ETC2RGB8A1Unorm),
        wgt::TextureFormat::Etc2Rgb8A1UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGB8A1UnormSrgb),
        wgt::TextureFormat::Etc2Rgba8Unorm => Some(native::WGPUTextureFormat_ETC2RGBA8Unorm),
        wgt::TextureFormat::Etc2Rgba8UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGBA8UnormSrgb),
        wgt::TextureFormat::EacR11Unorm => Some(native::WGPUTextureFormat_EACR11Unorm),
        wgt::TextureFormat::EacR11Snorm => Some(native::WGPUTextureFormat_EACR11Snorm),
        wgt::TextureFormat::EacRg11Unorm => Some(native::WGPUTextureFormat_EACRG11Unorm),
        wgt::TextureFormat::EacRg11Snorm => Some(native::WGPUTextureFormat_EACRG11Snorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC4x4Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC4x4UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC5x4Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC5x4UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC5x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC5x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC6x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC6x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC6x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC6x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x8Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x8UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x8Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x8UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x10Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x10UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC12x10Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC12x10UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC12x12Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC12x12UnormSrgb),
    }
}

#[inline]
pub fn map_stencil_face_state(
    value: native::WGPUStencilFaceState,
    mode: &str,
) -> wgt::StencilFaceState {
    wgt::StencilFaceState {
        compare: map_compare_function(value.compare)
            .unwrap_or_else(|_| panic!("invalid compare function for {mode} stencil face state")),
        fail_op: map_stencil_operation(value.failOp)
            .unwrap_or_else(|_| panic!("invalid fail op for {mode} stencil face state")),
        depth_fail_op: map_stencil_operation(value.depthFailOp)
            .unwrap_or_else(|_| panic!("invalid depth fail op for {mode} stencil face state")),
        pass_op: map_stencil_operation(value.passOp)
            .unwrap_or_else(|_| panic!("invalid pass op for {mode} stencil face state")),
    }
}

#[inline]
pub fn map_primitive_state(
    _: &native::WGPUPrimitiveState,
    depth_clip_control: Option<&native::WGPUPrimitiveDepthClipControl>,
) -> bool {
    if let Some(depth_clip_control) = depth_clip_control {
        return depth_clip_control.unclippedDepth != 0;
    }

    false
}

#[inline]
pub fn map_storage_report(report: wgc::registry::RegistryReport) -> native::WGPURegistryReport {
    native::WGPURegistryReport {
        numAllocated: report.num_allocated,
        numKeptFromUser: report.num_kept_from_user,
        numReleasedFromUser: report.num_released_from_user,
        numDestroyedFromUser: report.num_destroyed_from_user,
        numError: report.num_error,
        elementSize: report.element_size,
    }
}

#[inline]
pub fn map_hub_report(report: wgc::hub::HubReport) -> native::WGPUHubReport {
    native::WGPUHubReport {
        adapters: map_storage_report(report.adapters),
        devices: map_storage_report(report.devices),
        queues: map_storage_report(report.queues),
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
    report: wgc::global::GlobalReport,
) {
    native_report.surfaces = map_storage_report(report.surfaces);

    #[cfg(any(
        windows,
        all(
            unix,
            not(target_os = "emscripten"),
            not(target_os = "ios"),
            not(target_os = "macos")
        )
    ))]
    if let Some(vulkan) = report.vulkan {
        native_report.vulkan = map_hub_report(vulkan);
        native_report.backendType = native::WGPUBackendType_Vulkan;
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    if let Some(metal) = report.metal {
        native_report.metal = map_hub_report(metal);
        native_report.backendType = native::WGPUBackendType_Metal;
    }

    #[cfg(windows)]
    if let Some(dx12) = report.dx12 {
        native_report.dx12 = map_hub_report(dx12);
        native_report.backendType = native::WGPUBackendType_D3D12;
    }

    #[cfg(all(unix, not(target_os = "ios"), not(target_os = "macos")))]
    if let Some(gl) = report.gl {
        native_report.gl = map_hub_report(gl);
        native_report.backendType = native::WGPUBackendType_OpenGL;
    }
}

#[inline]
pub fn map_features(features: &[native::WGPUFeatureName]) -> wgt::Features {
    let mut temp = wgt::Features::empty();

    features.iter().for_each(|f| {
        if let Some(feature) = map_feature(*f) {
            temp.insert(feature);
        };
    });

    temp
}

#[inline]
pub fn features_to_native(features: wgt::Features) -> Vec<native::WGPUFeatureName> {
    let mut temp = Vec::new();

    if features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
        temp.push(native::WGPUFeatureName_DepthClipControl);
    }
    if features.contains(wgt::Features::DEPTH32FLOAT_STENCIL8) {
        temp.push(native::WGPUFeatureName_Depth32FloatStencil8);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_BC) {
        temp.push(native::WGPUFeatureName_TextureCompressionBC);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ETC2) {
        temp.push(native::WGPUFeatureName_TextureCompressionETC2);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC) {
        temp.push(native::WGPUFeatureName_TextureCompressionASTC);
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY) {
        temp.push(native::WGPUFeatureName_TimestampQuery);
    }
    if features.contains(wgt::Features::INDIRECT_FIRST_INSTANCE) {
        temp.push(native::WGPUFeatureName_IndirectFirstInstance);
    }
    if features.contains(wgt::Features::SHADER_F16) {
        temp.push(native::WGPUFeatureName_ShaderF16);
    }
    if features.contains(wgt::Features::RG11B10UFLOAT_RENDERABLE) {
        temp.push(native::WGPUFeatureName_RG11B10UfloatRenderable);
    }
    if features.contains(wgt::Features::BGRA8UNORM_STORAGE) {
        temp.push(native::WGPUFeatureName_BGRA8UnormStorage);
    }
    //if features.contains(wgt::Features::FLOAT32_FILTERABLE) {
    //    temp.push(native::WGPUFeatureName_Float32Filterable);
    //} -> not yet available in wgpu-core

    // wgpu-rs only features
    if features.contains(wgt::Features::PUSH_CONSTANTS) {
        temp.push(native::WGPUNativeFeature_PushConstants);
    }
    if features.contains(wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        temp.push(native::WGPUNativeFeature_TextureAdapterSpecificFormatFeatures);
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT) {
        temp.push(native::WGPUNativeFeature_MultiDrawIndirect);
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT_COUNT) {
        temp.push(native::WGPUNativeFeature_MultiDrawIndirectCount);
    }
    if features.contains(wgt::Features::VERTEX_WRITABLE_STORAGE) {
        temp.push(native::WGPUNativeFeature_VertexWritableStorage);
    }
    if features.contains(wgt::Features::TEXTURE_BINDING_ARRAY) {
        temp.push(native::WGPUNativeFeature_TextureBindingArray);
    }
    if features
        .contains(wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
    {
        temp.push(native::WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing);
    }
    if features.contains(wgt::Features::PIPELINE_STATISTICS_QUERY) {
        temp.push(native::WGPUNativeFeature_PipelineStatisticsQuery);
    }

    temp
}

#[inline]
#[rustfmt::skip]
pub fn map_feature(feature: native::WGPUFeatureName) -> Option<wgt::Features> {
    use wgt::Features;

    match feature {
        native::WGPUFeatureName_DepthClipControl => Some(Features::DEPTH_CLIP_CONTROL),
        native::WGPUFeatureName_Depth32FloatStencil8 => Some(Features::DEPTH32FLOAT_STENCIL8),
        native::WGPUFeatureName_TextureCompressionBC => Some(Features::TEXTURE_COMPRESSION_BC),
        native::WGPUFeatureName_TextureCompressionETC2 => Some(Features::TEXTURE_COMPRESSION_ETC2),
        native::WGPUFeatureName_TextureCompressionASTC => Some(Features::TEXTURE_COMPRESSION_ASTC),
        native::WGPUFeatureName_TimestampQuery => Some(Features::TIMESTAMP_QUERY),
        native::WGPUFeatureName_IndirectFirstInstance => Some(Features::INDIRECT_FIRST_INSTANCE),
        native::WGPUFeatureName_ShaderF16 => Some(Features::SHADER_F16),
        native::WGPUFeatureName_RG11B10UfloatRenderable => Some(Features::RG11B10UFLOAT_RENDERABLE),
        native::WGPUFeatureName_BGRA8UnormStorage => Some(Features::BGRA8UNORM_STORAGE),
        //native::WGPUFeatureName_Float32Filterable => Some(Features::FLOAT32_FILTERABLE),

        // wgpu-rs only features
        native::WGPUNativeFeature_PushConstants => Some(Features::PUSH_CONSTANTS),
        native::WGPUNativeFeature_TextureAdapterSpecificFormatFeatures => Some(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
        native::WGPUNativeFeature_MultiDrawIndirect => Some(Features::MULTI_DRAW_INDIRECT),
        native::WGPUNativeFeature_MultiDrawIndirectCount => Some(Features::MULTI_DRAW_INDIRECT_COUNT),
        native::WGPUNativeFeature_VertexWritableStorage => Some(Features::VERTEX_WRITABLE_STORAGE),
        native::WGPUNativeFeature_TextureBindingArray => Some(Features::TEXTURE_BINDING_ARRAY),
        native::WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing => Some(Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
        native::WGPUNativeFeature_PipelineStatisticsQuery => Some(Features::PIPELINE_STATISTICS_QUERY),
        // fallback, probably not available in wgpu-core
        _ => None,
    }
}

#[inline]
pub fn to_native_present_mode(mode: wgt::PresentMode) -> Option<native::WGPUPresentMode> {
    match mode {
        wgt::PresentMode::Fifo => Some(native::WGPUPresentMode_Fifo),
        wgt::PresentMode::Immediate => Some(native::WGPUPresentMode_Immediate),
        wgt::PresentMode::Mailbox => Some(native::WGPUPresentMode_Mailbox),
        wgt::PresentMode::FifoRelaxed => Some(native::WGPUPresentMode_FifoRelaxed),
        wgt::PresentMode::AutoVsync | wgt::PresentMode::AutoNoVsync => None,
    }
}

#[inline]
pub fn to_native_composite_alpha_mode(
    mode: wgt::CompositeAlphaMode,
) -> native::WGPUCompositeAlphaMode {
    match mode {
        wgt::CompositeAlphaMode::Auto => native::WGPUCompositeAlphaMode_Auto,
        wgt::CompositeAlphaMode::Opaque => native::WGPUCompositeAlphaMode_Opaque,
        wgt::CompositeAlphaMode::PreMultiplied => native::WGPUCompositeAlphaMode_Premultiplied,
        wgt::CompositeAlphaMode::PostMultiplied => native::WGPUCompositeAlphaMode_Unpremultiplied,
        wgt::CompositeAlphaMode::Inherit => native::WGPUCompositeAlphaMode_Inherit,
    }
}

#[inline]
pub fn map_bind_group_entry<'a>(
    entry: &'a native::WGPUBindGroupEntry,
    extras: Option<&native::WGPUBindGroupEntryExtras>,
) -> wgc::binding_model::BindGroupEntry<'a> {
    if let Some(buffer) = unsafe { entry.buffer.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::Buffer(
                wgc::binding_model::BufferBinding {
                    buffer_id: buffer.id,
                    offset: entry.offset,
                    size: match entry.size {
                        0 => panic!("invalid size"),
                        WGPU_WHOLE_SIZE => None,
                        _ => Some(unsafe { NonZeroU64::new_unchecked(entry.size) }),
                    },
                },
            ),
        };
    } else if let Some(sampler) = unsafe { entry.sampler.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::Sampler(sampler.id),
        };
    } else if let Some(texture_view) = unsafe { entry.textureView.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::TextureView(texture_view.id),
        };
    } else if let Some(extras) = extras {
        if let Some(texture_views) = unsafe { extras.textureViews.as_ref() } {
            let arr = make_slice(texture_views, extras.textureViewCount)
                .iter()
                .map(|v| {
                    unsafe { v.as_ref() }
                        .expect("invalid texture views for bind group entry extras")
                        .id
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::TextureViewArray(arr),
            };
        } else if let Some(samplers) = unsafe { extras.samplers.as_ref() } {
            let arr = make_slice(samplers, extras.samplerCount)
                .iter()
                .map(|v| {
                    unsafe { v.as_ref() }
                        .expect("invalid sampler for bind group entry extras")
                        .id
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::SamplerArray(arr),
            };
        } else if let Some(buffers) = unsafe { extras.buffers.as_ref() } {
            let arr = make_slice(buffers, extras.bufferCount)
                .iter()
                .map(|v| wgc::binding_model::BufferBinding {
                    buffer_id: unsafe { v.as_ref() }
                        .expect("invalid buffers for bind group entry extras")
                        .id,
                    offset: entry.offset,
                    size: std::num::NonZeroU64::new(entry.size),
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::BufferArray(arr),
            };
        }
    }

    panic!("invalid bind group entry for bind group descriptor");
}

#[inline]
pub fn map_bind_group_layout_entry(
    entry: &native::WGPUBindGroupLayoutEntry,
    extras: Option<&native::WGPUBindGroupLayoutEntryExtras>,
) -> wgt::BindGroupLayoutEntry {
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
                _ => panic!("invalid sample type for texture binding layout"),
            },
            view_dimension: match entry.texture.viewDimension {
                native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                native::WGPUTextureViewDimension_CubeArray => wgt::TextureViewDimension::CubeArray,
                native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                _ => panic!("invalid texture view dimension for texture binding layout"),
            },
            multisampled: entry.texture.multisampled != 0,
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
            _ => panic!("invalid sampler binding type for sampler binding layout"),
        }
    } else if is_storage_texture {
        wgt::BindingType::StorageTexture {
            access: map_storage_texture_access(entry.storageTexture.access)
                .expect("invalid storage texture access for storage texture binding layout"),
            format: map_texture_format(entry.storageTexture.format)
                .expect("invalid texture format for storage texture binding layout"),
            view_dimension: match entry.storageTexture.viewDimension {
                native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                native::WGPUTextureViewDimension_CubeArray => wgt::TextureViewDimension::CubeArray,
                native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                _ => {
                    panic!("invalid texture view dimension for storage texture binding layout")
                }
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
                _ => panic!("invalid buffer binding type for buffer binding layout"),
            },
            has_dynamic_offset: entry.buffer.hasDynamicOffset != 0,
            min_binding_size: {
                assert_ne!(
                    entry.buffer.minBindingSize, WGPU_WHOLE_SIZE,
                    "invalid min binding size for buffer binding layout, use 0 instead"
                );

                NonZeroU64::new(entry.buffer.minBindingSize)
            },
        }
    } else {
        panic!("invalid bind group layout entry for bind group layout descriptor");
    };

    wgt::BindGroupLayoutEntry {
        ty,
        binding: entry.binding,
        visibility: wgt::ShaderStages::from_bits(entry.visibility)
            .expect("invalid visibility for bind group layout entry"),
        count: extras.and_then(|v| NonZeroU32::new(v.count)),
    }
}

pub fn map_query_set_index(index: u32) -> Option<u32> {
    match index {
        native::WGPU_QUERY_SET_INDEX_UNDEFINED => None,
        _ => Some(index),
    }
}

#[inline]
pub fn map_query_set_descriptor<'a>(
    desc: &native::WGPUQuerySetDescriptor,
    extras: Option<&native::WGPUQuerySetDescriptorExtras>,
) -> wgt::QuerySetDescriptor<wgc::Label<'a>> {
    wgt::QuerySetDescriptor {
        label: ptr_into_label(desc.label),
        count: desc.count,
        ty: match (desc.type_, extras) {
            (native::WGPUQueryType_Occlusion, _) => wgt::QueryType::Occlusion,
            (native::WGPUQueryType_Timestamp, _) => wgt::QueryType::Timestamp,
            (native::WGPUNativeQueryType_PipelineStatistics, Some(extras)) => {
                let mut types = wgt::PipelineStatisticsTypes::empty();

                make_slice(extras.pipelineStatistics, extras.pipelineStatisticCount)
                    .iter()
                    .for_each(|f| {
                        types.insert(match *f {
                            native::WGPUPipelineStatisticName_VertexShaderInvocations => {
                                wgt::PipelineStatisticsTypes::VERTEX_SHADER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ClipperInvocations => {
                                wgt::PipelineStatisticsTypes::CLIPPER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ClipperPrimitivesOut => {
                                wgt::PipelineStatisticsTypes::CLIPPER_PRIMITIVES_OUT
                            }
                            native::WGPUPipelineStatisticName_FragmentShaderInvocations => {
                                wgt::PipelineStatisticsTypes::FRAGMENT_SHADER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ComputeShaderInvocations => {
                                wgt::PipelineStatisticsTypes::COMPUTE_SHADER_INVOCATIONS
                            }
                            _ => panic!("invalid pipeline statistics name"),
                        });
                    });

                wgt::QueryType::PipelineStatistics(types)
            }
            _ => panic!("invalid query type"),
        },
    }
}

#[inline]
pub fn map_texture_usage_flags(flags: native::WGPUTextureUsage) -> wgt::TextureUsages {
    let mut temp = wgt::TextureUsages::empty();
    if (flags & native::WGPUTextureUsage_CopySrc) != 0 {
        temp.insert(wgt::TextureUsages::COPY_SRC);
    }
    if (flags & native::WGPUTextureUsage_CopyDst) != 0 {
        temp.insert(wgt::TextureUsages::COPY_DST);
    }
    if (flags & native::WGPUTextureUsage_TextureBinding) != 0 {
        temp.insert(wgt::TextureUsages::TEXTURE_BINDING);
    }
    if (flags & native::WGPUTextureUsage_RenderAttachment) != 0 {
        temp.insert(wgt::TextureUsages::RENDER_ATTACHMENT);
    }
    temp
}

pub enum CreateSurfaceParams {
    Raw(
        (
            raw_window_handle::RawDisplayHandle,
            raw_window_handle::RawWindowHandle,
        ),
    ),
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    Metal(*mut std::ffi::c_void),
}

pub unsafe fn map_surface(
    _: &native::WGPUSurfaceDescriptor,
    win: Option<&native::WGPUSurfaceDescriptorFromWindowsHWND>,
    xcb: Option<&native::WGPUSurfaceDescriptorFromXcbWindow>,
    xlib: Option<&native::WGPUSurfaceDescriptorFromXlibWindow>,
    wl: Option<&native::WGPUSurfaceDescriptorFromWaylandSurface>,
    metal: Option<&native::WGPUSurfaceDescriptorFromMetalLayer>,
    android: Option<&native::WGPUSurfaceDescriptorFromAndroidNativeWindow>,
) -> CreateSurfaceParams {
    if let Some(win) = win {
        let display_handle = raw_window_handle::WindowsDisplayHandle::new();
        let mut window_handle =
            raw_window_handle::Win32WindowHandle::new(NonZeroIsize::new_unchecked(win.hwnd as _));
        window_handle.hinstance = NonZeroIsize::new(win.hinstance as _);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Windows(display_handle),
            raw_window_handle::RawWindowHandle::Win32(window_handle),
        ));
    }

    if let Some(xcb) = xcb {
        let connection = NonNull::<std::ffi::c_void>::new_unchecked(xcb.connection);
        let display_handle = raw_window_handle::XcbDisplayHandle::new(Some(connection), 0);
        let window_handle =
            raw_window_handle::XcbWindowHandle::new(NonZeroU32::new_unchecked(xcb.window));

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Xcb(display_handle),
            raw_window_handle::RawWindowHandle::Xcb(window_handle),
        ));
    }

    if let Some(xlib) = xlib {
        let display = NonNull::<std::ffi::c_void>::new_unchecked(xlib.display);
        let display_handle = raw_window_handle::XlibDisplayHandle::new(Some(display), 0);
        let window_handle = raw_window_handle::XlibWindowHandle::new(xlib.window as _);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Xlib(display_handle),
            raw_window_handle::RawWindowHandle::Xlib(window_handle),
        ));
    }

    if let Some(wl) = wl {
        let display = NonNull::<std::ffi::c_void>::new_unchecked(wl.display);
        let surface = NonNull::<std::ffi::c_void>::new_unchecked(wl.surface);
        let display_handle = raw_window_handle::WaylandDisplayHandle::new(display);
        let window_handle = raw_window_handle::WaylandWindowHandle::new(surface);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Wayland(display_handle),
            raw_window_handle::RawWindowHandle::Wayland(window_handle),
        ));
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if let Some(metal) = metal {
        return CreateSurfaceParams::Metal(metal.layer);
    }

    if let Some(android) = android {
        let display_handle = raw_window_handle::AndroidDisplayHandle::new();
        let window_handle =
            raw_window_handle::AndroidNdkWindowHandle::new(NonNull::new_unchecked(android.window));

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Android(display_handle),
            raw_window_handle::RawWindowHandle::AndroidNdk(window_handle),
        ));
    }

    panic!("Error: Unsupported Surface");
}
