/**
 * @file wgpu.h
 * @brief wgpu-native specific extensions to the standard WebGPU C API.
 *
 * This header defines native-only types, enumerations, structures, and functions
 * that extend the WebGPU specification defined in @c webgpu.h. All extension
 * enum values and struct type identifiers (@ref WGPUNativeSType) are allocated
 * within the @c 0x0003XXXX range reserved for wgpu-native.
 *
 * Include this header after @c webgpu.h (it is included automatically).
 */
#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu.h"

typedef struct WGPUPipelineCacheImpl* WGPUPipelineCache WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPUBlasImpl* WGPUBlas WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPUTlasImpl* WGPUTlas WGPU_OBJECT_ATTRIBUTE;

typedef enum WGPUNativeSType
{
    // Start at 0003 since that's allocated range for wgpu-native
    /** Identifies @ref WGPUDeviceExtras. */
    WGPUSType_DeviceExtras = 0x00030001,
    /** Identifies @ref WGPUNativeLimits. */
    WGPUSType_NativeLimits = 0x00030002,
    /** Identifies @ref WGPUPipelineLayoutExtras. */
    WGPUSType_PipelineLayoutExtras = 0x00030003,
    /** Identifies @ref WGPUShaderSourceGLSL. */
    WGPUSType_ShaderSourceGLSL = 0x00030004,
    /** Identifies @ref WGPUInstanceExtras. */
    WGPUSType_InstanceExtras = 0x00030006,
    /** Identifies @ref WGPUBindGroupEntryExtras. */
    WGPUSType_BindGroupEntryExtras = 0x00030007,
    /** Identifies @ref WGPUBindGroupLayoutEntryExtras. */
    WGPUSType_BindGroupLayoutEntryExtras = 0x00030008,
    /** Identifies @ref WGPUQuerySetDescriptorExtras. */
    WGPUSType_QuerySetDescriptorExtras = 0x00030009,
    /** Identifies @ref WGPUSurfaceConfigurationExtras. */
    WGPUSType_SurfaceConfigurationExtras = 0x0003000A,
    /** Identifies @ref WGPUSurfaceSourceSwapChainPanel. */
    WGPUSType_SurfaceSourceSwapChainPanel = 0x0003000B,
    /** Identifies @ref WGPUPrimitiveStateExtras. */
    WGPUSType_PrimitiveStateExtras = 0x0003000C,
    /** Identifies @ref WGPUComputePipelineDescriptorExtras. */
    WGPUSType_ComputePipelineDescriptorExtras = 0x0003000D,
    /** Identifies @ref WGPURenderPipelineDescriptorExtras. */
    WGPUSType_RenderPipelineDescriptorExtras = 0x0003000E,
    /** Identifies @ref WGPUMeshPipelineDescriptorExtras. */
    WGPUSType_MeshPipelineDescriptorExtras = 0x0003000F,
    /** Identifies @ref WGPUAdapterInfoExtras. */
    WGPUSType_AdapterInfoExtras = 0x00030010,
    /** Identifies @ref WGPUSamplerDescriptorExtras. */
    WGPUSType_SamplerDescriptorExtras = 0x00030011,
    /** Identifies @ref WGPURenderPassDescriptorExtras. */
    WGPUSType_RenderPassDescriptorExtras = 0x00030012,
    /** Identifies @ref WGPURenderBundleEncoderDescriptorExtras. */
    WGPUSType_RenderBundleEncoderDescriptorExtras = 0x00030013,
    /** Identifies @ref WGPUDeviceDescriptorExtras. */
    WGPUSType_DeviceDescriptorExtras = 0x00030014,
    /** Identifies @ref WGPUAccelerationStructureBindingLayout. */
    WGPUSType_AccelerationStructureBindingLayout = 0x00030015,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

/**
 * Additional surface-get-current-texture status codes defined by wgpu-native.
 *
 * These extend the standard @c WGPUSurfaceGetCurrentTextureStatus values.
 */
typedef enum WGPUNativeSurfaceGetCurrentTextureStatus
{
    /**
     * The surface texture was not acquired because the window is occluded
     * (e.g. minimized or fully covered by another window).
     *
     * No texture is returned and the @c texture field of
     * @c WGPUSurfaceTexture will be NULL. The surface and swapchain remain
     * valid -- there is no need to reconfigure or recreate the surface.
     *
     * Applications should skip rendering for the current frame and try
     * again once the window is no longer occluded. If you are using a
     * windowing library such as winit, listen for the window's "occluded"
     * event and request a new redraw when the window becomes visible again.
     *
     * When does this occur?
     *
     * Currently this status is only produced by the Metal backend on macOS.
     * When a window is not visible (checked via the @c NSWindow
     * @c occlusionState property), acquiring the next drawable would block
     * for up to one second waiting for vsync. wgpu-native returns
     * @c Occluded instead to avoid that hang.
     *
     * Other backends (Vulkan, DX12, GL) do not currently report this
     * status; an occluded window on those backends may produce
     * @c WGPUSurfaceGetCurrentTextureStatus_Timeout or simply succeed
     * normally.
     */
    WGPUSurfaceGetCurrentTextureStatus_Occluded = 0x00030001,
    WGPUNativeSurfaceGetCurrentTextureStatus_Force32 = 0x7FFFFFFF
} WGPUNativeSurfaceGetCurrentTextureStatus;

/**
 * Native-only device features.
 *
 * These extend the standard @c WGPUFeatureName values and can be passed to
 * @c WGPUDeviceDescriptor::requiredFeatures to request additional
 * capabilities when creating a device.
 */
typedef enum WGPUNativeFeature
{
    /**
     * Allows the use of immediate data: small, fast blocks of memory
     * that can be updated inside a render pass, compute pass, or render
     * bundle encoder.
     *
     * Enables @ref wgpuRenderPassEncoderSetImmediates,
     * @ref wgpuComputePassEncoderSetImmediates,
     * @ref wgpuRenderBundleEncoderSetImmediates,
     * non-zero @c immediateDataSize in @ref WGPUPipelineLayoutExtras,
     * and non-zero @c maxImmediateSize in @ref WGPUNativeLimits.
     *
     * A block of immediate data can be declared in WGSL with
     * @c var<immediate>:
     * @code
     * struct Immediates { example: f32, }
     * var<immediate> c: Immediates;
     * @endcode
     *
     * In GLSL, this corresponds to @c layout(immediates) @c uniform @c Name @c {..}.
     *
     * Supported platforms:
     * - DX12
     * - Vulkan
     * - Metal
     * - OpenGL (emulated with uniforms)
     * - WebGPU
     *
     * This is a web and native feature.
     */
    WGPUNativeFeature_Immediates = 0x00030001,
    /**
     * Enables device-specific texture format features.
     *
     * By default only texture format properties as defined by the WebGPU
     * specification are allowed. Enabling this feature flag extends the
     * features of each format to the ones supported by the current device.
     * Note that without this flag, read/write storage access is not allowed
     * at all.
     *
     * This extension does not enable additional formats.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureAdapterSpecificFormatFeatures = 0x00030002,
    /**
     * Allows the use of a buffer containing the actual number of draw calls.
     *
     * Enables @ref wgpuRenderPassEncoderMultiDrawIndirectCount and
     * @ref wgpuRenderPassEncoderMultiDrawIndexedIndirectCount.
     *
     * This feature being present also implies that all calls to
     * @ref wgpuRenderPassEncoderMultiDrawIndirect and
     * @ref wgpuRenderPassEncoderMultiDrawIndexedIndirect are not being
     * emulated with a series of @c draw_indirect calls.
     *
     * Supported platforms:
     * - DX12
     * - Vulkan 1.2+ (or VK_KHR_draw_indirect_count)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_MultiDrawIndirectCount = 0x00030004,
    /**
     * Enables bindings of writable storage buffers and textures visible
     * to vertex shaders.
     *
     * Note: some (tiled-based) platforms do not support vertex shaders
     * with any side-effects.
     *
     * Supported platforms:
     * - All
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_VertexWritableStorage = 0x00030005,
    /**
     * Allows the user to create uniform arrays of textures in shaders:
     *
     * - WGSL: @c var @c textures: @c binding_array<texture_2d<f32>, @c 10>
     * - GLSL: @c uniform @c texture2D @c textures[10]
     *
     * If @ref WGPUNativeFeature_StorageResourceBindingArray is supported
     * as well as this, the user may also create uniform arrays of storage
     * textures.
     *
     * This capability allows them to exist and to be indexed by dynamically
     * uniform values.
     *
     * Supported platforms:
     * - DX12
     * - Metal (with MSL 2.0+ on macOS 10.13+)
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureBindingArray = 0x00030006,
    /**
     * Allows shaders to index sampled texture and storage buffer resource
     * arrays with dynamically non-uniform values:
     *
     * e.g. @c texture_array[vertex_data]
     *
     * In order to use this capability, the corresponding GLSL extension must
     * be enabled:
     *
     * @c \#extension @c GL_EXT_nonuniform_qualifier @c : @c require
     *
     * and then used either as @c nonuniformEXT qualifier in variable
     * declaration or as @c nonuniformEXT constructor.
     *
     * WGSL and HLSL do not need any extension.
     *
     * Supported platforms:
     * - DX12
     * - Metal (with MSL 2.0+ on macOS 10.13+)
     * - Vulkan 1.2+ (or VK_EXT_descriptor_indexing)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing = 0x00030007,
    /**
     * Enables use of Pipeline Statistics Queries. These queries report the
     * count of various operations performed between the start and stop call.
     *
     * Use @ref wgpuRenderPassEncoderBeginPipelineStatisticsQuery /
     * @ref wgpuRenderPassEncoderEndPipelineStatisticsQuery (or the compute
     * pass equivalents) to start and stop a query.
     *
     * They must be resolved using @c wgpuCommandEncoderResolveQuerySet into
     * a buffer. See @ref WGPUPipelineStatisticName for the list of available
     * statistics.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_PipelineStatisticsQuery = 0x00030008,
    /**
     * Allows the user to create uniform arrays of storage buffers or
     * textures in shaders, if @ref WGPUNativeFeature_BufferBindingArray
     * or @ref WGPUNativeFeature_TextureBindingArray (respectively)
     * is also supported.
     *
     * This capability allows them to exist and to be indexed by dynamically
     * uniform values.
     *
     * Supported platforms:
     * - Metal (with MSL 2.2+ on macOS 10.13+)
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_StorageResourceBindingArray = 0x00030009,
    /**
     * Allows the user to create bind groups containing arrays with fewer
     * bindings than the @c WGPUBindGroupLayout requires.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_PartiallyBoundBindingArray = 0x0003000A,
    /**
     * Enables normalized 16-bit texture formats:
     * @ref WGPUNativeTextureFormat_R16Unorm, @ref WGPUNativeTextureFormat_R16Snorm,
     * @ref WGPUNativeTextureFormat_Rg16Unorm, @ref WGPUNativeTextureFormat_Rg16Snorm,
     * @ref WGPUNativeTextureFormat_Rgba16Unorm, @ref WGPUNativeTextureFormat_Rgba16Snorm.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureFormat16bitNorm = 0x0003000B,
    /**
     * Enables ASTC HDR family of compressed textures.
     *
     * Compressed textures sacrifice some quality in exchange for
     * significantly reduced bandwidth usage.
     *
     * Support for this feature guarantees availability of
     * @c COPY_SRC | @c COPY_DST | @c TEXTURE_BINDING for ASTC formats
     * with the HDR channel type.
     * @ref WGPUNativeFeature_TextureAdapterSpecificFormatFeatures may
     * enable additional usages.
     *
     * Supported platforms:
     * - Metal
     * - Vulkan
     * - OpenGL
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureCompressionAstcHdr = 0x0003000C,
    /**
     * Removes the WebGPU restriction that @c MAP_READ and @c MAP_WRITE
     * buffer usages must be paired exclusively with @c COPY_DST and
     * @c COPY_SRC respectively.
     *
     * This is only beneficial on systems that share memory between CPU and
     * GPU. If enabled on a system that doesn't, this can severely hinder
     * performance. Only use if you understand the consequences.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_MappablePrimaryBuffers = 0x0003000E,
    /**
     * Allows the user to create arrays of buffers in shaders:
     *
     * - WGSL: @c var<uniform> @c buffer_array: @c array<MyBuffer, @c 10>
     * - GLSL: @c uniform @c myBuffer @c { @c ... @c } @c buffer_array[10]
     *
     * This capability allows them to exist and to be indexed by dynamically
     * uniform values.
     *
     * If @ref WGPUNativeFeature_StorageResourceBindingArray is supported as
     * well as this, the user may also create arrays of storage buffers.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_BufferBindingArray = 0x0003000F,
    /**
     * Allows shaders to index storage texture resource
     * arrays with dynamically non-uniform values.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_StorageTextureArrayNonUniformIndexing = 0x00030010,
    /**
     * Allows the use of @ref WGPUSamplerBorderColor_Zero via @ref WGPUSamplerDescriptorExtras.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_AddressModeClampToZero = 0x00030011,
    /**
     * Allows the use of @ref WGPUNativeAddressMode_ClampToBorder.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_AddressModeClampToBorder = 0x00030012,
    /**
     * Allows the user to set @ref WGPUPolygonMode_Line in
     * @ref WGPUPrimitiveStateExtras::polygonMode.
     *
     * This allows drawing polygons/triangles as lines (wireframe) instead
     * of filled.
     *
     * Supported platforms:
     * - DX12
     * - Vulkan
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_PolygonModeLine = 0x00030013,
    /**
     * Allows the user to set @ref WGPUPolygonMode_Point in
     * @ref WGPUPrimitiveStateExtras::polygonMode.
     *
     * This allows only drawing the vertices of polygons/triangles instead
     * of filled.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_PolygonModePoint = 0x00030014,
    /**
     * Allows the user to enable overestimation conservative rasterization
     * via @ref WGPUPrimitiveStateExtras::conservative.
     *
     * Processing of degenerate triangles/lines is hardware specific.
     * Only triangles are supported.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ConservativeRasterization = 0x00030015,
    /**
     * Enables clear to zero for textures.
     *
     * Supported platforms:
     * - All
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ClearTexture = 0x00030016,
    /**
     * Enables multiview render passes and `builtin(view_index)` in vertex/mesh shaders.
     * 
     * Supported platforms:
     * - Vulkan
     * - Metal
     * - DX12
     * - OpenGL (web only)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_Multiview = 0x00030018,
    /**
     * Enables using 64-bit types for vertex attributes.
     *
     * Requires @ref WGPUNativeFeature_ShaderF64.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_VertexAttribute64bit = 0x00030019,
    /**
     * Allows for creation of textures of format
     * @ref WGPUNativeTextureFormat_NV12.
     *
     * Supported platforms:
     * - DX12
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureFormatNv12 = 0x0003001A,
    /**
     * Allows for the creation of ray-tracing queries within shaders.
     *
     * @b EXPERIMENTAL: Features enabled by this may have major bugs and are
     * expected to be subject to breaking changes.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_RayQuery = 0x0003001C,
    /**
     * Enables 64-bit floating point types in SPIR-V shaders.
     *
     * Note: even when supported by GPU hardware, 64-bit floating point
     * operations are frequently between 16 and 64 @e times slower than
     * equivalent operations on 32-bit floats.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderF64 = 0x0003001D,
    /**
     * Allows shaders to use i16. Not currently supported in naga, only
     * available through SPIR-V passthrough.
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderI16 = 0x0003001E,
    /**
     * Allows shaders to use the @c early_depth_test attribute.
     *
     * The attribute is applied to the fragment shader entry point and can be
     * used in two ways:
     *
     * 1. Force early depth/stencil tests:
     *    - WGSL: @c \@early_depth_test(force)
     *    - GLSL: @c layout(early_fragment_tests) @c in;
     *
     * 2. Provide a conservative depth specifier that allows an additional
     *    early depth test under certain conditions:
     *    - WGSL: @c \@early_depth_test(greater_equal/less_equal/unchanged)
     *    - GLSL: @c layout(depth_<greater/less/unchanged>) @c out @c float @c gl_FragDepth;
     *
     * Supported platforms:
     * - Vulkan
     * - GLES 3.1+
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderEarlyDepthTest = 0x00030020,
    /**
     * Allows compute and fragment shaders to use the subgroup operation
     * built-ins and perform subgroup operations (except barriers).
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_Subgroup = 0x00030021,
    /**
     * Allows vertex shaders to use the subgroup operation built-ins and
     * perform subgroup operations (except barriers).
     *
     * Supported platforms:
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_SubgroupVertex = 0x00030022,
    /**
     * Allows compute shaders to use the subgroup barrier.
     *
     * Requires @ref WGPUNativeFeature_Subgroup. Without it, enables nothing.
     *
     * Supported platforms:
     * - Vulkan
     * - Metal
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_SubgroupBarrier = 0x00030023,
    /**
     * Allows for timestamp queries directly on command encoders.
     *
     * Implies @c WGPUFeatureName_TimestampQuery is supported.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal
     * - OpenGL (with GL_ARB_timer_query)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TimestampQueryInsideEncoders = 0x00030024,
    /**
     * Allows for timestamp queries inside render and compute passes.
     *
     * Implies @c WGPUFeatureName_TimestampQuery and
     * @ref WGPUNativeFeature_TimestampQueryInsideEncoders are supported.
     *
     * Enables @ref wgpuRenderPassEncoderWriteTimestamp and
     * @ref wgpuComputePassEncoderWriteTimestamp.
     *
     * This is generally not available on tile-based rasterization GPUs.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal (AMD & Intel, not Apple GPUs)
     * - OpenGL (with GL_ARB_timer_query)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TimestampQueryInsidePasses = 0x00030025,
    /**
     * Allows shaders to use i64 and u64.
     *
     * Supported platforms:
     * - Vulkan
     * - DX12 (DXC only)
     * - Metal (with MSL 2.3+)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderInt64 = 0x00030026,
    /**
     * Allows shaders to use f32 atomic load, store, add, sub, and exchange.
     * 
     * Supported platforms:
     * - Metal (with MSL 3.0+ and Apple7+/Mac2)
     * - Vulkan (with [VK_EXT_shader_atomic_float])
     *  
     * This is a native only feature.
    */
    WGPUNativeFeature_ShaderFloat32Atomic = 0x00030027,
    /**
     * Enables image atomic fetch add, and, xor, or, min, and max for R32Uint and R32Sint textures.
     * 
     * Supported platforms:
     * - Vulkan
     * - DX12
     * - Metal (with MSL 3.1+)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureAtomic = 0x00030028,
    /**
     * Allows for creation of textures of format
     * @ref WGPUNativeTextureFormat_P010.
     *
     * Supported platforms:
     * - DX12
     * - Vulkan
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureFormatP010 = 0x00030029,
    /**
     * Enables @c ExternalTexture support.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ExternalTexture = 0x0003002A,
    /**
     * Allows the use of pipeline cache objects
     * 
     * Supported platforms:
     * - Vulkan
     * 
     * Unimplemented Platforms:
     * - DX12
     * - Metal
     */
    WGPUNativeFeature_PipelineCache = 0x0003002B,
    /**
     * Allows shaders to use i64 and u64 atomic min and max.
     *
     * Supported platforms:
     * - Vulkan (with VK_KHR_shader_atomic_int64)
     * - DX12 (with SM 6.6+)
     * - Metal (with MSL 2.4+)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderInt64AtomicMinMax = 0x0003002C,
    /**
     * Allows shaders to use all i64 and u64 atomic operations.
     *
     * Supported platforms:
     * - Vulkan (with VK_KHR_shader_atomic_int64)
     * - DX12 (with SM 6.6+)
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderInt64AtomicAllOps = 0x0003002D,
    /**
     * Enables Vulkan VK_GOOGLE_display_timing extension.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_VulkanGoogleDisplayTiming = 0x0003002E,
    /**
     * Enables Vulkan external memory for Win32 handles.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_VulkanExternalMemoryWin32 = 0x0003002F,
    /**
     * Enables R64Uint image atomic min and max.
     * 
     * Supported platforms:
     * - Vulkan (with VK_EXT_shader_image_atomic_int64)
     * - DX12 (with SM 6.6+)
     * - Metal (with MSL 3.1+)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_TextureInt64Atomic = 0x00030030,
    // TODO: not implemented yet, see https://github.com/gfx-rs/wgpu/issues/7149
    // WGPUNativeFeature_UniformBufferBindingArrays = 0x00030031,
    WGPUNativeFeature_MeshShader = 0x00030032,
    /**
     * Enables returning hit vertex data from ray tracing shaders.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_RayHitVertexReturn = 0x00030033,
    /**
     * Enables multiview in mesh shaders.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_MeshShaderMultiview = 0x00030034,
    /**
     * Enables extended vertex format support for acceleration structures.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_ExtendedAccelerationStructureVertexFormats = 0x00030035,
    /**
     * Enables passthrough shaders.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_PassthroughShaders = 0x00030036,
    /**
     * Enables shader barycentric coordinates.
     * 
     * Supported platforms:
     * - Vulkan (with VK_KHR_fragment_shader_barycentric)
     * - DX12 (with SM 6.1+)
     * - Metal (with MSL 2.2+)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderBarycentrics = 0x00030037,
    /**
     * Enables using multiview where not all texture array layers are rendered to in a single render pass/render pipeline. Making
     * use of this feature also requires enabling `Features::MULTIVIEW`.
     * 
     * Supported platforms
     * - Vulkan
     * - DX12
     * 
     * While metal supports this in theory, the behavior of `view_index` differs from vulkan and dx12 so the feature isn't exposed.
     */
    WGPUNativeFeature_SelectiveMultiview = 0x00030038,
    /**
     * Enables point topology in mesh shaders.
     *
     * This is a native only feature.
     */
    WGPUNativeFeature_MeshShaderPoints = 0x00030039,
    WGPUNativeFeature_MultisampleArray = 0x0003003A,
    /**
     * Enables cooperative matrix operations (also known as tensor cores on NVIDIA GPUs
     * or simdgroup matrix operations on Apple GPUs).
     * 
     * Cooperative matrices allow a workgroup to collectively load, store, and perform
     * matrix multiply-accumulate operations on small tiles of data, enabling
     * hardware-accelerated matrix math.
     *
     * @b EXPERIMENTAL: Features enabled by this may have major bugs and are
     * expected to be subject to breaking changes.
     * 
     * **Current limitations:** The implementation currently only supports 8x8 f32 matrices.
     * On Vulkan, support is determined by querying `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`
     * for configurations matching 8x8x8 f32. Most Vulkan implementations (NVIDIA, AMD) primarily
     * support f16 inputs at larger sizes (e.g., 16x16), so Vulkan support may be limited.
     * 
     * Supported platforms:
     * - Metal (with MSL 2.3+ and Apple7+/Mac2+, using simdgroup matrix operations)
     * - Vulkan (with [VK_KHR_cooperative_matrix](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_cooperative_matrix.html), if 8x8 f32 is supported)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_CooperativeMatrix = 0x0003003B,
    /**
     * Enables shader per-vertex attributes.
     * 
     * Supported platforms:
     * - Vulkan (with VK_KHR_fragment_shader_barycentric)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderPerVertex = 0x0003003C,
    /**
     * Enables shader `draw_index` builtin.
     * 
     * Supported platforms:
     * - GLES
     * - Vulkan
     * 
     * Potential platforms:
     * - DX12
     * - Metal
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_ShaderDrawIndex = 0x0003003D,
    /**
     * Allows the user to create arrays of acceleration structures in shaders:
     * 
     * ex.
     * - `var tlas: binding_array<acceleration_structure, 10>` (WGSL)
     * 
     * This capability allows them to exist and to be indexed by dynamically uniform values.
     * 
     * Supported platforms:
     * - DX12
     * - Vulkan
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_AccelerationStructureBindingArray = 0x0003003E,
    /**
     * Enables the `@coherent` memory decoration on storage buffer variables.
     * 
     * Backend mapping:
     * - Vulkan
     * - DX12
     * - Metal (3.2+)
     * - GLES (ES 3.1+ / GL 4.3+)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_MemoryDecorationCoherent = 0x0003003F,
    /**
     * Enables the `@volatile` memory decoration on storage buffer variables.
     * 
     * Backend mapping:
     * - Vulkan
     * - GLES (ES 3.1+ / GL 4.3+)
     * 
     * This is a native only feature.
     */
    WGPUNativeFeature_MemoryDecorationVolatile = 0x00030040,

    WGPUNativeFeature_Force32 = 0x7FFFFFFF
} WGPUNativeFeature;

/**
 * Geometry kind stored in a bottom level acceleration structure.
 */
typedef enum WGPUBlasGeometryKind
{
    WGPUBlasGeometryKind_Triangles = 0x00000000,
    WGPUBlasGeometryKind_AABBs = 0x00000001,
    WGPUBlasGeometryKind_Force32 = 0x7FFFFFFF
} WGPUBlasGeometryKind;

/**
 * Build mode for acceleration structures.
 */
typedef enum WGPUAccelerationStructureUpdateMode
{
    /** Always perform a full build. */
    WGPUAccelerationStructureUpdateMode_Build = 0x00000000,
    /** Perform an incremental update if possible. */
    WGPUAccelerationStructureUpdateMode_PreferUpdate = 0x00000001,
    WGPUAccelerationStructureUpdateMode_Force32 = 0x7FFFFFFF
} WGPUAccelerationStructureUpdateMode;

typedef WGPUFlags WGPUAccelerationStructureFlags;
/** No flags. */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_None = 0x00000000;
/** Allow incremental updates. */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_AllowUpdate = 1 << 0;
/** Allow compaction via @ref wgpuQueueCompactBlas. */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_AllowCompaction = 1 << 1;
/** Optimize for fast ray tracing (non-dynamic geometry). */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_PreferFastTrace = 1 << 2;
/** Optimize for fast build (dynamic geometry). */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_PreferFastBuild = 1 << 3;
/** Minimize memory footprint. */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_LowMemory = 1 << 4;
/** Use transform buffer during BLAS build (only valid at BLAS creation). */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_UseTransform = 1 << 5;
/** Allow retrieval of hit triangle vertices. Requires @ref WGPUNativeFeature_RayHitVertexReturn. */
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_AllowRayHitVertexReturn = 1 << 6;
static const WGPUAccelerationStructureFlags WGPUAccelerationStructureFlags_Force32 = 0x7FFFFFFF;

typedef WGPUFlags WGPUAccelerationStructureGeometryFlags;
/** No flags. */
static const WGPUAccelerationStructureGeometryFlags WGPUAccelerationStructureGeometryFlags_None = 0x00000000;
/** Geometry is opaque (no alpha testing). */
static const WGPUAccelerationStructureGeometryFlags WGPUAccelerationStructureGeometryFlags_Opaque = 1 << 0;
/** Prevent duplicate any-hit shader invocations per primitive. */
static const WGPUAccelerationStructureGeometryFlags WGPUAccelerationStructureGeometryFlags_NoDuplicateAnyHitInvocation = 1 << 1;
static const WGPUAccelerationStructureGeometryFlags WGPUAccelerationStructureGeometryFlags_Force32 = 0x7FFFFFFF;

typedef enum WGPULogLevel
{
    WGPULogLevel_Off = 0x00000000,
    /** Only error messages. */
    WGPULogLevel_Error = 0x00000001,
    /** Errors and warnings. */
    WGPULogLevel_Warn = 0x00000002,
    /** Errors, warnings, and informational messages. */
    WGPULogLevel_Info = 0x00000003,
    /** Errors, warnings, informational, and debug messages. */
    WGPULogLevel_Debug = 0x00000004,
    /** All messages, including very verbose trace-level output. */
    WGPULogLevel_Trace = 0x00000005,
    WGPULogLevel_Force32 = 0x7FFFFFFF
} WGPULogLevel;

/**
 * Bitflags selecting which graphics backends the @ref WGPUInstance should
 * enable.
 *
 * Pass in the @c backends field of @ref WGPUInstanceExtras.
 */
typedef WGPUFlags WGPUInstanceBackend;
/** All backends (the default when zero-initialized). */
static const WGPUInstanceBackend WGPUInstanceBackend_All = 0x00000000;
/**
 * Vulkan backend.
 * Supported on Windows, Linux/Android, and macOS/iOS via Vulkan Portability.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Vulkan = 1 << 0;
/**
 * OpenGL / OpenGL ES backend.
 * Supported on Linux/Android, the web via WebGL, and Windows/macOS via ANGLE.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_GL = 1 << 1;
/**
 * Metal backend.
 * Supported on macOS and iOS.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Metal = 1 << 2;
/**
 * Direct3D 12 backend.
 * Supported on Windows 10 and later.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_DX12 = 1 << 3;
/**
 * Browser WebGPU backend.
 * Supported when targeting the web through WebAssembly.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_BrowserWebGPU = 1 << 5;
/** Primary (first-tier) backends: Vulkan, Metal, DX12, and BrowserWebGPU. */
static const WGPUInstanceBackend WGPUInstanceBackend_Primary = (1 << 0) | (1 << 2) | (1 << 3) | (1 << 5);
/** Secondary (second-tier) backends: GL. */
static const WGPUInstanceBackend WGPUInstanceBackend_Secondary = (1 << 1);
static const WGPUInstanceBackend WGPUInstanceBackend_Force32 = 0x7FFFFFFF;

/**
 * Bitflags controlling instance debugging and validation behavior.
 *
 * These are not part of the WebGPU standard.
 *
 * Pass in the @c flags field of @ref WGPUInstanceExtras.
 */
typedef WGPUFlags WGPUInstanceFlag;
/** No flags set. */
static const WGPUInstanceFlag WGPUInstanceFlag_Empty = 0x00000000;
/**
 * Generate debug information in shaders and objects.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_DEBUG environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Debug = 1 << 0;
/**
 * Enable validation in the backend API, if possible:
 *
 * - On the DX12 backend, this calls @c ID3D12Debug::EnableDebugLayer.
 * - On the Vulkan backend, this enables the Vulkan Validation Layers.
 * - On the GLES backend (Windows), this enables debug output.
 * - On the GLES backend (non-Windows), this calls @c eglDebugMessageControlKHR.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_VALIDATION environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Validation = 1 << 1;
/**
 * Don't pass labels to the backend API (wgpu-hal).
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_DISCARD_HAL_LABELS environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_DiscardHalLabels = 1 << 2;
/**
 * Whether wgpu should expose adapters that run on top of non-compliant
 * adapters.
 *
 * Turning this on might mean that some of the functionality provided by the
 * wgpu adapter/device is not working or broken. This mainly applies to a
 * Vulkan driver's compliance version. If the major compliance version is 0,
 * then the driver is ignored unless this flag is set.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_AllowUnderlyingNoncompliantAdapter = 1 << 3;
/**
 * Enable GPU-based validation. Implies @ref WGPUInstanceFlag_Validation.
 * Currently only changes behavior on the DX12 and Vulkan backends.
 *
 * - D3D12: Called "GPU-based validation" (GBV).
 * - Vulkan: Called "GPU-Assisted Validation" via VK_LAYER_KHRONOS_validation.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_GPU_BASED_VALIDATION environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_GPUBasedValidation = 1 << 4;
/**
 * Validate indirect buffer content prior to issuing indirect draws/dispatches.
 *
 * This validation will transform indirect calls into no-ops if they are not
 * valid. For example, @c dispatch_workgroups_indirect arguments must be less
 * than the @c max_compute_workgroups_per_dimension device limit.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_VALIDATION_INDIRECT_CALL environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_ValidationIndirectCall = 1 << 5;
/**
 * Enable automatic timestamp normalization. When enabled,
 * @c wgpuCommandEncoderResolveQuerySet will automatically normalize timestamps
 * to nanoseconds instead of returning raw timestamp values.
 *
 * This introduces a compute shader into the resolution of query sets. When
 * enabled, the timestamp period returned by @ref wgpuQueueGetTimestampPeriod
 * will always be @c 1.0.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_AutomaticTimestampNormalization = 1 << 6;
/**
 * Use the default flags for the current build configuration.
 * In debug builds, this typically enables @ref WGPUInstanceFlag_Debug and
 * @ref WGPUInstanceFlag_Validation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Default = 1 << 24;
/**
 * Convenience alias that enables @ref WGPUInstanceFlag_Debug and
 * @ref WGPUInstanceFlag_Validation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Debugging = 1 << 25;
/**
 * Convenience alias that enables @ref WGPUInstanceFlag_Debug,
 * @ref WGPUInstanceFlag_Validation, and
 * @ref WGPUInstanceFlag_GPUBasedValidation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_AdvancedDebugging = 1 << 26;
/**
 * Modify the flags based on environment variables. Flags with environment
 * variable support (e.g. @c WGPU_DEBUG, @c WGPU_VALIDATION) will be read
 * from the process environment and applied on top of the explicitly set flags.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_WithEnv = 1 << 27;
static const WGPUInstanceFlag WGPUInstanceFlag_Force32 = 0x7FFFFFFF;

typedef enum WGPUDx12Compiler
{
    WGPUDx12Compiler_Undefined = 0x00000000,
    /**
     * Use the FXC (D3DCompile) shader compiler.
     *
     * The FXC compiler is old, slow, and unmaintained. However, it doesn't
     * require any additional DLLs to be shipped with the application.
     */
    WGPUDx12Compiler_Fxc = 0x00000001,
    /**
     * Use the DXC (DirectX Shader Compiler).
     *
     * The DXC compiler is new, fast, and maintained. However, it requires
     * @c dxcompiler.dll to be available. The path to this DLL can be
     * specified via @ref WGPUInstanceExtras::dxcPath.
     *
     * Minimum supported version: v1.8.2502.
     * Requires WDDM 2.1 (Windows 10 version 1607).
     */
    WGPUDx12Compiler_Dxc = 0x00000002,
    WGPUDx12Compiler_Force32 = 0x7FFFFFFF
} WGPUDx12Compiler;

typedef enum WGPUGles3MinorVersion
{
    WGPUGles3MinorVersion_Automatic = 0x00000000,
    /** Request an ES 3.0 context. */
    WGPUGles3MinorVersion_Version0 = 0x00000001,
    /** Request an ES 3.1 context. */
    WGPUGles3MinorVersion_Version1 = 0x00000002,
    /** Request an ES 3.2 context. */
    WGPUGles3MinorVersion_Version2 = 0x00000003,
    WGPUGles3MinorVersion_Force32 = 0x7FFFFFFF
} WGPUGles3MinorVersion;

typedef enum WGPUPipelineStatisticName
{
    WGPUPipelineStatisticName_VertexShaderInvocations = 0x00000000,
    /**
     * Number of times the clipper is invoked. This is also the number of
     * triangles output by the vertex shader.
     */
    WGPUPipelineStatisticName_ClipperInvocations = 0x00000001,
    /**
     * Number of primitives that are not culled by the clipper. This is the
     * number of triangles that are actually on screen and will be rasterized
     * and rendered.
     */
    WGPUPipelineStatisticName_ClipperPrimitivesOut = 0x00000002,
    /**
     * Number of times the fragment shader is invoked. Accounts for fragment
     * shaders running in 2x2 blocks in order to get derivatives.
     */
    WGPUPipelineStatisticName_FragmentShaderInvocations = 0x00000003,
    /**
     * Number of times a compute shader is invoked. This will be equivalent
     * to the dispatch count times the workgroup size.
     */
    WGPUPipelineStatisticName_ComputeShaderInvocations = 0x00000004,
    WGPUPipelineStatisticName_Force32 = 0x7FFFFFFF
} WGPUPipelineStatisticName WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUNativeQueryType
{
    WGPUNativeQueryType_PipelineStatistics = 0x00030000,
    WGPUNativeQueryType_Force32 = 0x7FFFFFFF
} WGPUNativeQueryType WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUDxcMaxShaderModel
{
    WGPUDxcMaxShaderModel_V6_0 = 0x00000000,
    /** Shader Model 6.1 */
    WGPUDxcMaxShaderModel_V6_1 = 0x00000001,
    /** Shader Model 6.2 */
    WGPUDxcMaxShaderModel_V6_2 = 0x00000002,
    /** Shader Model 6.3 */
    WGPUDxcMaxShaderModel_V6_3 = 0x00000003,
    /** Shader Model 6.4 */
    WGPUDxcMaxShaderModel_V6_4 = 0x00000004,
    /** Shader Model 6.5 */
    WGPUDxcMaxShaderModel_V6_5 = 0x00000005,
    /** Shader Model 6.6 */
    WGPUDxcMaxShaderModel_V6_6 = 0x00000006,
    /** Shader Model 6.7 */
    WGPUDxcMaxShaderModel_V6_7 = 0x00000007,
    WGPUDxcMaxShaderModel_Force32 = 0x7FFFFFFF
} WGPUDxcMaxShaderModel;

typedef enum WGPUGLFenceBehaviour
{
    WGPUGLFenceBehaviour_Normal = 0x00000000,
    /**
     * Fences are short-circuited to always report completion immediately.
     *
     * This solves a specific issue that arose due to a bug in wgpu-core that
     * made many WebGL programs work when they shouldn't have. If you have
     * code that calls @ref wgpuDevicePoll with @c wait=true on WebGL, you
     * may need to enable this option for "wait" to behave how you expect.
     *
     * When this is set, @c wgpuQueueOnCompletedWorkDone callbacks will fire
     * the next time the device is polled, not when work is actually done on
     * the GPU.
     */
    WGPUGLFenceBehaviour_AutoFinish = 0x00000001,
    WGPUGLFenceBehaviour_Force32 = 0x7FFFFFFF
} WGPUGLFenceBehaviour;

typedef enum WGPUDx12SwapchainKind
{
    WGPUDx12SwapchainKind_Undefined = 0x00000000,
    /**
     * Use a DXGI swapchain created directly from the window's HWND.
     *
     * This does not support transparency but has better support from
     * developer tooling such as RenderDoc.
     */
    WGPUDx12SwapchainKind_DxgiFromHwnd = 0x00000001,
    /**
     * Use a DXGI swapchain created from a DirectComposition visual made
     * automatically from the window's HWND.
     *
     * This creates a single @c IDCompositionVisual over the entire window.
     * Supports transparency. If you want to manage the composition tree
     * yourself, create your own device and composition and pass the relevant
     * visual via the surface target.
     */
    WGPUDx12SwapchainKind_DxgiFromVisual = 0x00000002,
    WGPUDx12SwapchainKind_Force32 = 0x7FFFFFFF
} WGPUDx12SwapchainKind;

/**
 * Discriminant for @ref WGPUNativeDisplayHandle.
 *
 * Identifies which platform's display connection is stored in the tagged union.
 * Use @ref WGPUNativeDisplayHandleType_None (the default when zero-initialized) when
 * no display handle is needed. Platforms with no display connection data (Windows,
 * macOS, iOS, Android) should use @ref WGPUNativeDisplayHandleType_None.
 */
typedef enum WGPUNativeDisplayHandleType
{
    /** No display handle provided. */
    WGPUNativeDisplayHandleType_None = 0x00000000,
    /** X11 display connection via Xlib. See @ref WGPUXlibDisplayHandle. */
    WGPUNativeDisplayHandleType_Xlib = 0x00000001,
    /** X11 display connection via XCB. See @ref WGPUXcbDisplayHandle. */
    WGPUNativeDisplayHandleType_Xcb = 0x00000002,
    /** Wayland display connection. See @ref WGPUWaylandDisplayHandle. */
    WGPUNativeDisplayHandleType_Wayland = 0x00000003,
    WGPUNativeDisplayHandleType_Force32 = 0x7FFFFFFF
} WGPUNativeDisplayHandleType;

/**
 * Xlib display connection data for @ref WGPUNativeDisplayHandle.
 */
typedef struct WGPUXlibDisplayHandle
{
    /** Pointer to the X11 @c Display (i.e. @c Display*). Must not be NULL. */
    void *display;
    /** X11 screen number. */
    int screen;
} WGPUXlibDisplayHandle;

/**
 * XCB display connection data for @ref WGPUNativeDisplayHandle.
 */
typedef struct WGPUXcbDisplayHandle
{
    /** Pointer to the XCB connection (i.e. @c xcb_connection_t*). Must not be NULL. */
    void *connection;
    /** X11 screen number. */
    int screen;
} WGPUXcbDisplayHandle;

/**
 * Wayland display connection data for @ref WGPUNativeDisplayHandle.
 */
typedef struct WGPUWaylandDisplayHandle
{
    /** Pointer to the Wayland display (i.e. @c wl_display*). Must not be NULL. */
    void *display;
} WGPUWaylandDisplayHandle;

/**
 * Platform display connection, passed as a field of @ref WGPUInstanceExtras.
 *
 * This is a tagged union. Set @c type to indicate which variant is active, then
 * populate the corresponding field in @c data. Zero-initialization yields
 * @ref WGPUNativeDisplayHandleType_None, meaning no display handle is provided.
 *
 * Currently required by the GLES backend when presenting on Wayland. Other
 * backends ignore this field. If the instance is created with a display handle,
 * all surfaces created from it must use the same display connection.
 */
typedef struct WGPUNativeDisplayHandle
{
    WGPUNativeDisplayHandleType type;
    union
    {
        WGPUXlibDisplayHandle xlib;
        WGPUXcbDisplayHandle xcb;
        WGPUWaylandDisplayHandle wayland;
    } data;
} WGPUNativeDisplayHandle;

/**
 * Chained in @ref WGPUAdapterInfo to expose wgpu-native-specific adapter properties.
 *
 * Passed as @ref WGPUAdapterInfo::nextInChain to @ref wgpuAdapterGetInfo.
 * Caller must free @ref devicePciBusId via @ref wgpuAdapterInfoFreeMembers.
 */
typedef struct WGPUAdapterInfoExtras
{
    WGPUChainedStruct chain;
    /** Whether the adapter uses memory that is shared with the CPU and
     *  benefits from keeping allocations small (e.g. integrated/mobile GPUs). */
    WGPUBool transientSavesMemory;
    /**
     * PCI bus identifier for the adapter in the form @c "bus:device.function",
     * e.g. @c "0000:01:00.0". Empty when not available.
     *
     * This is an \ref OutputString.
     */
    WGPUStringView devicePciBusId;
} WGPUAdapterInfoExtras WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUInstanceExtras
{
    WGPUChainedStruct chain;
    /**
     * Which backends to enable.
     * Zero (@ref WGPUInstanceBackend_All) enables all backends.
     */
    WGPUInstanceBackend backends;
    /**
     * Flags controlling debug/validation behavior.
     * See @ref WGPUInstanceFlag for available flags.
     */
    WGPUInstanceFlag flags;
    /**
     * Which DX12 shader compiler to use.
     * See @ref WGPUDx12Compiler. Ignored on non-DX12 backends.
     */
    WGPUDx12Compiler dx12ShaderCompiler;
    /**
     * Which OpenGL ES 3 minor version to request.
     * See @ref WGPUGles3MinorVersion. Ignored on non-GL backends.
     */
    WGPUGles3MinorVersion gles3MinorVersion;
    /**
     * Controls OpenGL fence synchronization behavior.
     * See @ref WGPUGLFenceBehaviour. Ignored on non-GL backends.
     */
    WGPUGLFenceBehaviour glFenceBehaviour;
    /**
     * File system path to @c dxcompiler.dll for dynamic DXC loading.
     * Only used when @c dx12ShaderCompiler is @ref WGPUDx12Compiler_Dxc.
     * An empty/undefined string view means the DLL will be searched for
     * on the system PATH.
     */
    WGPUStringView dxcPath;
    /**
     * Maximum HLSL shader model version that DXC should target.
     * See @ref WGPUDxcMaxShaderModel. Only used with the DXC compiler.
     */
    WGPUDxcMaxShaderModel dxcMaxShaderModel;
    /**
     * Which DX12 presentation system (swapchain kind) to use.
     * See @ref WGPUDx12SwapchainKind. Ignored on non-DX12 backends.
     */
    WGPUDx12SwapchainKind dx12PresentationSystem;

    WGPU_NULLABLE const uint8_t *budgetForDeviceCreation;
    WGPU_NULLABLE const uint8_t *budgetForDeviceLoss;

    /**
     * Platform display connection to associate with this instance.
     * Zero-initialized yields @ref WGPUNativeDisplayHandleType_None (no handle).
     */
    WGPUNativeDisplayHandle displayHandle;
} WGPUInstanceExtras;

typedef struct WGPUDeviceExtras
{
    WGPUChainedStruct chain;
    /**
     * File system path for API trace output.
     *
     * When set to a non-empty path, wgpu will record all API calls to
     * the given directory, which can later be replayed for debugging.
     * An empty/undefined string view disables tracing.
     */
    WGPUStringView tracePath;
} WGPUDeviceExtras;

typedef struct WGPUNativeLimits
{
    /** This struct chain is used as mutable in some places and immutable in others. */
    WGPUChainedStruct chain;
    /**
     * Maximum number of live non-sampler bindings.
     *
     * Default is 1,000,000. Only meaningful on D3D12.
     *
     * @b Warning: On integrated GPUs, large values can cause significant
     * system RAM consumption.
     */
    uint32_t maxNonSamplerBindings;
    /**
     * Maximum number of individual resources within binding arrays that can be accessed
     * in a single shader stage. Applies to all types of bindings except samplers.
     */
    uint32_t maxBindingArrayElementsPerShaderStage;
    /**
     * Maximum number of individual samplers within binding arrays that
     * can be accessed in a single shader stage.
     */
    uint32_t maxBindingArraySamplerElementsPerShaderStage;
    /**
     * The maximum number of views that can be used in multiview rendering.
     */
    uint32_t maxMultiviewViewCount;
} WGPUNativeLimits;

#define WGPU_NATIVE_LIMITS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUNativeLimits, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/(WGPUSType)WGPUSType_NativeLimits _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.maxNonSamplerBindings=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxBindingArrayElementsPerShaderStage=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxBindingArraySamplerElementsPerShaderStage=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxMultiviewViewCount=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
})

typedef struct WGPUPipelineLayoutExtras
{
    WGPUChainedStruct chain;
    /**
     * The number of bytes of immediate data allocated for use in shaders
     * attached to this pipeline.
     *
     * The @c var<immediate> declarations in the shader must be equal or
     * smaller than this size. If this value is non-zero,
     * @ref WGPUNativeFeature_Immediates must be enabled.
     */
    uint32_t immediateDataSize;
} WGPUPipelineLayoutExtras;

/**
 * Identifier for a particular call to @ref wgpuQueueSubmitForIndex.
 *
 * Can be passed to @ref wgpuDevicePoll to block until a particular
 * submission has finished execution.
 *
 * This type is unique to wgpu-native; there is no analogue in the
 * WebGPU specification.
 */
typedef uint64_t WGPUSubmissionIndex;

typedef struct WGPUShaderDefine
{
    WGPUStringView name;
    /** The value of the preprocessor macro (e.g. @c "1"). */
    WGPUStringView value;
} WGPUShaderDefine;

typedef struct WGPUShaderSourceGLSL
{
    WGPUChainedStruct chain;
    /** The shader stage this GLSL source targets. */
    WGPUShaderStage stage;
    /** GLSL source code. */
    WGPUStringView code;
    /** Number of entries in @c defines. */
    uint32_t defineCount;
    WGPUShaderDefine const *defines;
} WGPUShaderSourceGLSL;

typedef struct WGPUPassthroughShaderEntryPoint
{
    /** Entry point name. Required for GLSL and DXIL; used for identification in others. */
    WGPUStringView name;
    /** Workgroup size for compute shaders (Metal only). Ignored for other stages. */
    uint32_t workgroupSizeX;
    uint32_t workgroupSizeY;
    uint32_t workgroupSizeZ;
} WGPUPassthroughShaderEntryPoint;

/**
 * Descriptor for a shader module created from native/backend-specific sources.
 * At least one source appropriate for the active backend must be provided.
 *
 * This type is unique to wgpu-native; there is no analogue in the
 * WebGPU specification.
 */
typedef struct WGPUShaderModuleDescriptorPassthrough
{
    WGPUStringView label;
    /** Number of entries in @c entryPoints. */
    size_t entryPointCount;
    WGPUPassthroughShaderEntryPoint const *entryPoints;
    /** Number of 32-bit words in @c spirv. 0 if unused. */
    uint32_t spirvSize;
    uint32_t const *spirv;
    /** Byte count of @c dxil. 0 if unused. */
    size_t dxilSize;
    uint8_t const *dxil;
    /** HLSL source. @ref WGPUStringView_null if unused. */
    WGPUStringView hlsl;
    /** Byte count of @c metallib. 0 if unused. */
    size_t metallibSize;
    uint8_t const *metallib;
    /** MSL source. @ref WGPUStringView_null if unused. */
    WGPUStringView msl;
} WGPUShaderModuleDescriptorPassthrough;

typedef struct WGPURegistryReport
{
    size_t numAllocated;
    size_t numKeptFromUser;
    size_t numReleasedFromUser;
    size_t elementSize;
} WGPURegistryReport;

typedef struct WGPUHubReport
{
    WGPURegistryReport adapters;
    WGPURegistryReport devices;
    WGPURegistryReport queues;
    WGPURegistryReport pipelineLayouts;
    WGPURegistryReport shaderModules;
    WGPURegistryReport bindGroupLayouts;
    WGPURegistryReport bindGroups;
    WGPURegistryReport commandBuffers;
    WGPURegistryReport renderBundles;
    WGPURegistryReport renderPipelines;
    WGPURegistryReport computePipelines;
    WGPURegistryReport pipelineCaches;
    WGPURegistryReport querySets;
    WGPURegistryReport buffers;
    WGPURegistryReport textures;
    WGPURegistryReport textureViews;
    WGPURegistryReport samplers;
} WGPUHubReport;

typedef struct WGPUGlobalReport
{
    WGPURegistryReport surfaces;
    /** Statistics for all other resource types, grouped by backend hub. */
    WGPUHubReport hub;
} WGPUGlobalReport;

typedef struct WGPUInstanceEnumerateAdapterOptions
{
    WGPUChainedStruct const *nextInChain;
    WGPUInstanceBackend backends;
} WGPUInstanceEnumerateAdapterOptions;

typedef struct WGPUBindGroupEntryExtras
{
    WGPUChainedStruct chain;
    WGPUBuffer const *buffers;
    size_t bufferCount;
    WGPUSampler const *samplers;
    size_t samplerCount;
    WGPUTextureView const *textureViews;
    size_t textureViewCount;
    /** For AccelerationStructure bindings. NULL for non-AS bindings. */
    WGPU_NULLABLE WGPUTlas tlas;
} WGPUBindGroupEntryExtras;

typedef struct WGPUBindGroupLayoutEntryExtras
{
    WGPUChainedStruct chain;
    /**
     * Number of resources in this binding array slot. Corresponds to the
     * array size in the shader (e.g. @c binding_array<T, @c N>).
     */
    uint32_t count;
} WGPUBindGroupLayoutEntryExtras;

typedef struct WGPUQuerySetDescriptorExtras
{
    WGPUChainedStruct chain;
    WGPUPipelineStatisticName const *pipelineStatistics;
    size_t pipelineStatisticCount;
} WGPUQuerySetDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUSurfaceConfigurationExtras
{
    WGPUChainedStruct chain;
    /**
     * Desired maximum number of frames in flight (i.e. the number of monitor
     * refreshes between @c wgpuSurfaceGetCurrentTexture and presentation).
     *
     * - 1: Minimize latency (CPU and GPU cannot run in parallel).
     * - 2: Balance between latency and throughput (the default).
     * - 3+: Maximize throughput.
     */
    uint32_t desiredMaximumFrameLatency;
} WGPUSurfaceConfigurationExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUSurfaceDescriptor to make a @ref WGPUSurface wrapping a WinUI [`SwapChainPanel`](https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.controls.swapchainpanel).
 */
typedef struct WGPUSurfaceSourceSwapChainPanel
{
    WGPUChainedStruct chain;
    /**
     * A pointer to the [`ISwapChainPanelNative`](https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/win32/microsoft.ui.xaml.media.dxinterop/nn-microsoft-ui-xaml-media-dxinterop-iswapchainpanelnative)
     * interface of the SwapChainPanel that will be wrapped by the @ref WGPUSurface.
     */
    void *panelNative;
} WGPUSurfaceSourceSwapChainPanel WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Native extension for @ref WGPUAddressMode.
 *
 * Cast to @c WGPUAddressMode when storing in sampler descriptor fields.
 */
typedef enum WGPUNativeAddressMode
{
    /** Clamp to border color. Requires @ref WGPUNativeFeature_AddressModeClampToBorder. */
    WGPUNativeAddressMode_ClampToBorder = 0x00030001,
    WGPUNativeAddressMode_Force32 = 0x7FFFFFFF
} WGPUNativeAddressMode;

/**
 * Border color to use when address mode is @ref WGPUNativeAddressMode_ClampToBorder.
 *
 * Pass via @ref WGPUSamplerDescriptorExtras.
 */
typedef enum WGPUSamplerBorderColor
{
    WGPUSamplerBorderColor_TransparentBlack = 0x00000000,
    WGPUSamplerBorderColor_OpaqueBlack = 0x00000001,
    WGPUSamplerBorderColor_OpaqueWhite = 0x00000002,
    /** Requires @ref WGPUNativeFeature_AddressModeClampToZero. */
    WGPUSamplerBorderColor_Zero = 0x00000003,
    WGPUSamplerBorderColor_Force32 = 0x7FFFFFFF
} WGPUSamplerBorderColor;

/**
 * Memory allocation strategy for device creation.
 *
 * Pass via @ref WGPUDeviceDescriptorExtras.
 */
typedef enum WGPUMemoryHints
{
    /** Favour performance over memory usage (default). */
    WGPUMemoryHints_Performance = 0x00000000,
    /** Favour memory usage over performance. */
    WGPUMemoryHints_MemoryUsage = 0x00000001,
    /** Manual suballocation block size. Use @ref WGPUDeviceDescriptorExtras fields. */
    WGPUMemoryHints_Manual = 0x00000002,
    WGPUMemoryHints_Force32 = 0x7FFFFFFF
} WGPUMemoryHints;

typedef enum WGPUPolygonMode
{
    WGPUPolygonMode_Fill = 0,
    /**
     * Polygons are drawn as line segments (wireframe).
     * Requires @ref WGPUNativeFeature_PolygonModeLine.
     */
    WGPUPolygonMode_Line = 1,
    /**
     * Polygons are drawn as points (vertices only).
     * Requires @ref WGPUNativeFeature_PolygonModePoint.
     */
    WGPUPolygonMode_Point = 2,
} WGPUPolygonMode;

typedef struct WGPUPrimitiveStateExtras
{
    WGPUChainedStruct chain;
    /**
     * Controls the way each polygon is rasterized.
     * See @ref WGPUPolygonMode. Defaults to @ref WGPUPolygonMode_Fill.
     */
    WGPUPolygonMode polygonMode;
    /**
     * If set to true, the primitives are rendered with conservative
     * overestimation. Only valid when @c polygonMode is
     * @ref WGPUPolygonMode_Fill.
     * Requires @ref WGPUNativeFeature_ConservativeRasterization.
     */
    WGPUBool conservative;
} WGPUPrimitiveStateExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUSamplerDescriptor to set the border color.
 *
 * Required when any address mode is @ref WGPUNativeAddressMode_ClampToBorder.
 * Requires @ref WGPUNativeFeature_AddressModeClampToBorder (or
 * @ref WGPUNativeFeature_AddressModeClampToZero for @ref WGPUSamplerBorderColor_Zero).
 */
typedef struct WGPUSamplerDescriptorExtras
{
    WGPUChainedStruct chain;
    WGPUSamplerBorderColor borderColor;
} WGPUSamplerDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPURenderPassDescriptor to enable multiview rendering.
 *
 * Requires @ref WGPUNativeFeature_Multiview.
 */
typedef struct WGPURenderPassDescriptorExtras
{
    WGPUChainedStruct chain;
    /** Bitmask of view indices to render into. Zero disables multiview. */
    uint32_t multiviewMask;
} WGPURenderPassDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPURenderBundleEncoderDescriptor to enable multiview rendering.
 *
 * Requires @ref WGPUNativeFeature_Multiview.
 */
typedef struct WGPURenderBundleEncoderDescriptorExtras
{
    WGPUChainedStruct chain;
    /** Bitmask of view indices the bundle will render into. Zero disables multiview. */
    uint32_t multiviewMask;
} WGPURenderBundleEncoderDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUDeviceDescriptor to configure memory hints and experimental features.
 */
typedef struct WGPUDeviceDescriptorExtras
{
    WGPUChainedStruct chain;
    /** Memory allocation strategy. See @ref WGPUMemoryHints. Default: @ref WGPUMemoryHints_Performance. */
    WGPUMemoryHints memoryHints;
    /** Minimum suballocation block size in bytes. Only used when @c memoryHints is @ref WGPUMemoryHints_Manual. */
    uint64_t suballocatedDeviceMemoryBlockSizeMin;
    /** Maximum suballocation block size in bytes. Only used when @c memoryHints is @ref WGPUMemoryHints_Manual. */
    uint64_t suballocatedDeviceMemoryBlockSizeMax;
    /** If true, enables experimental wgpu features on the device. */
    WGPUBool experimentalFeaturesEnabled;
} WGPUDeviceDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Descriptor for creating a pipeline cache object via @ref wgpuDeviceCreatePipelineCache.
 *
 * Pass previously saved cache data in @c data/@c dataSize to seed the cache.
 * Leave both as zero/NULL to create an empty cache.
 * If @c fallback is true, creation errors are non-fatal and an empty cache is returned.
 *
 * Requires @ref WGPUNativeFeature_PipelineCache.
 */
typedef struct WGPUPipelineCacheDescriptor
{
    WGPUChainedStruct * nextInChain;
    WGPUStringView label;
    /** Number of bytes in @c data. */
    size_t dataSize;
    /** Previously saved cache blob, or NULL to start empty. */
    WGPU_NULLABLE uint8_t const * data;
    /**
     * If true, a cache-creation failure produces an empty (no-op) cache
     * rather than propagating an error.
     */
    WGPUBool fallback;
} WGPUPipelineCacheDescriptor WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUComputePipelineDescriptor to attach a pipeline cache.
 *
 * Requires @ref WGPUNativeFeature_PipelineCache.
 */
typedef struct WGPUComputePipelineDescriptorExtras
{
    WGPUChainedStruct chain;
    WGPU_NULLABLE WGPUPipelineCache cache;
    WGPUBool zeroInitializeWorkgroupMemory;
} WGPUComputePipelineDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPURenderPipelineDescriptor to attach a pipeline cache
 * or configure multiview rendering.
 *
 * Requires @ref WGPUNativeFeature_PipelineCache for the cache field.
 * Requires @ref WGPUNativeFeature_MultiView for the multiviewMask field.
 */
typedef struct WGPURenderPipelineDescriptorExtras
{
    WGPUChainedStruct chain;
    WGPU_NULLABLE WGPUPipelineCache cache;
    uint32_t multiviewMask;
    WGPUBool zeroInitializeWorkgroupMemory;
} WGPURenderPipelineDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Describes the mesh shader stage in a @ref WGPUMeshPipelineDescriptor.
 *
 * Requires @ref WGPUNativeFeature_MeshShader.
 */
typedef struct WGPUMeshState
{
    WGPUChainedStruct * nextInChain;
    WGPUShaderModule module;
    WGPUStringView entryPoint;
    size_t constantCount;
    WGPUConstantEntry const * constants;
} WGPUMeshState WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Describes the optional task shader stage in a @ref WGPUMeshPipelineDescriptor.
 *
 * Requires @ref WGPUNativeFeature_MeshShader.
 */
typedef struct WGPUTaskState
{
    WGPUChainedStruct * nextInChain;
    WGPUShaderModule module;
    WGPUStringView entryPoint;
    size_t constantCount;
    WGPUConstantEntry const * constants;
} WGPUTaskState WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Descriptor for @ref wgpuDeviceCreateMeshPipeline.
 *
 * A mesh pipeline replaces the vertex stage with an optional task stage
 * and a required mesh stage. All other fields mirror @ref WGPURenderPipelineDescriptor.
 *
 * Requires @ref WGPUNativeFeature_MeshShader.
 */
typedef struct WGPUMeshPipelineDescriptor
{
    WGPUChainedStruct * nextInChain;
    WGPUStringView label;
    WGPU_NULLABLE WGPUPipelineLayout layout;
    /** Optional task shader stage. NULL if no task shader is used. */
    WGPU_NULLABLE WGPUTaskState const * task;
    WGPUMeshState mesh;
    WGPUPrimitiveState primitive;
    WGPU_NULLABLE WGPUDepthStencilState const * depthStencil;
    WGPUMultisampleState multisample;
    WGPU_NULLABLE WGPUFragmentState const * fragment;
} WGPUMeshPipelineDescriptor WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUMeshPipelineDescriptor to attach a pipeline cache.
 *
 * Requires @ref WGPUNativeFeature_PipelineCache.
 */
typedef struct WGPUMeshPipelineDescriptorExtras
{
    WGPUChainedStruct chain;
    WGPU_NULLABLE WGPUPipelineCache cache;
    uint32_t multiviewMask;
    WGPUBool zeroInitializeWorkgroupMemory;
} WGPUMeshPipelineDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Chained in @ref WGPUBindGroupLayoutEntry to describe an acceleration structure binding.
 *
 * Set @c chain.sType to @ref WGPUSType_AccelerationStructureBindingLayout.
 * Requires @ref WGPUNativeFeature_RayQuery.
 */
typedef struct WGPUAccelerationStructureBindingLayout
{
    WGPUChainedStruct chain;
    /** Enable vertex return. Requires @ref WGPUNativeFeature_RayHitVertexReturn. */
    WGPUBool vertexReturn;
} WGPUAccelerationStructureBindingLayout WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Descriptor for creating a bottom level acceleration structure.
 * Pass to @ref wgpuDeviceCreateBlas.
 */
typedef struct WGPUBlasDescriptor
{
    WGPUChainedStruct * nextInChain;
    WGPUStringView label;
    WGPUAccelerationStructureFlags flags;
    WGPUAccelerationStructureUpdateMode updateMode;
} WGPUBlasDescriptor WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Descriptor for creating a top level acceleration structure.
 * Pass to @ref wgpuDeviceCreateTlas.
 */
typedef struct WGPUTlasDescriptor
{
    WGPUChainedStruct * nextInChain;
    WGPUStringView label;
    uint32_t maxInstances;
    WGPUAccelerationStructureFlags flags;
    WGPUAccelerationStructureUpdateMode updateMode;
} WGPUTlasDescriptor WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Size attributes for one group of triangle geometry in a BLAS.
 * Used in @ref WGPUBlasSizeDescriptors.
 */
typedef struct WGPUBlasTriangleGeometrySizeDescriptor
{
    WGPUVertexFormat vertexFormat;
    uint32_t vertexCount;
    /** @ref WGPUIndexFormat_Undefined means no index buffer. */
    WGPUIndexFormat indexFormat;
    /** Ignored when @ref indexFormat is @ref WGPUIndexFormat_Undefined. */
    uint32_t indexCount;
    WGPUAccelerationStructureGeometryFlags flags;
} WGPUBlasTriangleGeometrySizeDescriptor;

/**
 * Size attributes for one group of AABB geometry in a BLAS.
 * Used in @ref WGPUBlasSizeDescriptors.
 */
typedef struct WGPUBlasAABBGeometrySizeDescriptor
{
    uint32_t primitiveCount;
    WGPUAccelerationStructureGeometryFlags flags;
} WGPUBlasAABBGeometrySizeDescriptor;

/**
 * Size descriptors for a BLAS.
 * Set @ref kind and fill the matching pair of fields.
 * The other pair should be NULL/0.
 * Pass to @ref wgpuDeviceCreateBlas.
 */
typedef struct WGPUBlasSizeDescriptors
{
    WGPUBlasGeometryKind kind;
    /** Triangle geometry descriptors (used when kind == Triangles). */
    WGPUBlasTriangleGeometrySizeDescriptor const *triangleDescriptors;
    size_t triangleDescriptorCount;
    /** AABB geometry descriptors (used when kind == AABBs). */
    WGPUBlasAABBGeometrySizeDescriptor const *aabbDescriptors;
    size_t aabbDescriptorCount;
} WGPUBlasSizeDescriptors;

/**
 * Triangle geometry for a BLAS build entry.
 * Used in @ref WGPUBlasBuildEntry.
 */
typedef struct WGPUBlasTriangleGeometry
{
    WGPUChainedStruct const *nextInChain;
    WGPUBlasTriangleGeometrySizeDescriptor const *size;
    WGPUBuffer vertexBuffer;
    /** NULL means no index buffer. */
    WGPU_NULLABLE WGPUBuffer indexBuffer;
    /** NULL means no transform. */
    WGPU_NULLABLE WGPUBuffer transformBuffer;
    uint32_t firstVertex;
    uint64_t vertexStride;
    /** Ignored when @ref indexBuffer is NULL. */
    uint32_t firstIndex;
    /** Ignored when @ref transformBuffer is NULL. */
    uint64_t transformBufferOffset;
} WGPUBlasTriangleGeometry;

/**
 * AABB geometry for a BLAS build entry.
 * Used in @ref WGPUBlasBuildEntry.
 */
typedef struct WGPUBlasAABBGeometry
{
    WGPUChainedStruct const *nextInChain;
    WGPUBlasAABBGeometrySizeDescriptor const *size;
    uint64_t stride;
    WGPUBuffer aabbBuffer;
    uint32_t primitiveOffset;
} WGPUBlasAABBGeometry;

/**
 * Describes one BLAS and its geometries to build.
 * Set @ref geometryKind and fill the matching pair of fields.
 * The other pair should be NULL/0.
 * Passed as an array to @ref wgpuCommandEncoderBuildAccelerationStructures.
 */
typedef struct WGPUBlasBuildEntry
{
    WGPUBlas blas;
    WGPUBlasGeometryKind geometryKind;
    /** Triangle geometries (used when geometryKind == Triangles). */
    WGPUBlasTriangleGeometry const *triangleGeometries;
    size_t triangleGeometryCount;
    /** AABB geometries (used when geometryKind == AABBs). */
    WGPUBlasAABBGeometry const *aabbGeometries;
    size_t aabbGeometryCount;
} WGPUBlasBuildEntry;

/**
 * One instance slot in a @ref WGPUTlasPackage.
 * Set @ref blas to NULL to mark the slot as inactive.
 */
typedef struct WGPUTlasInstance
{
    /** NULL means this slot is inactive. */
    WGPU_NULLABLE WGPUBlas blas;
    /** Affine transform 3x4, row major (3 rows, 4 columns). */
    float transform[12];
    /** Custom index passed to the shader (must fit in 24 bits). */
    uint32_t customData;
    /** Visibility mask for ray filtering. */
    uint8_t mask;
} WGPUTlasInstance;

/**
 * Describes one TLAS and its instances to build.
 * Passed as an array to @ref wgpuCommandEncoderBuildAccelerationStructures.
 */
typedef struct WGPUTlasPackage
{
    WGPUTlas tlas;
    WGPUTlasInstance const *instances;
    size_t instanceCount;
    /** First unmodified instance index (for partial updates). */
    uint32_t lowestUnmodified;
} WGPUTlasPackage;

/**
 * Internal counters from the HAL layer, returned by @ref wgpuDeviceGetInternalCounters.
 */
typedef struct WGPUHalCounters
{
    int64_t buffers;
    int64_t textures;
    int64_t textureViews;
    int64_t bindGroups;
    int64_t bindGroupLayouts;
    int64_t renderPipelines;
    int64_t computePipelines;
    int64_t pipelineLayouts;
    int64_t samplers;
    int64_t commandEncoders;
    int64_t shaderModules;
    int64_t querySets;
    int64_t fences;
    int64_t bufferMemory;
    int64_t textureMemory;
    int64_t accelerationStructureMemory;
    int64_t memoryAllocations;
} WGPUHalCounters;

/**
 * All internal counters, returned by @ref wgpuDeviceGetInternalCounters.
 */
typedef struct WGPUInternalCounters
{
    WGPUHalCounters hal;
} WGPUInternalCounters;

/** Callback invoked when a BLAS compaction is ready. */
typedef void (*WGPUBlasCompactCallback)(WGPUBool success, void *userdata1, void *userdata2);

typedef struct WGPUBlasCompactCallbackInfo
{
    WGPUChainedStruct const *nextInChain;
    WGPUCallbackMode mode;
    WGPUBlasCompactCallback callback;
    void *userdata1;
    void *userdata2;
} WGPUBlasCompactCallbackInfo;

typedef void (*WGPULogCallback)(WGPULogLevel level, WGPUStringView message, void *userdata);

typedef enum WGPUNativeTextureFormat
{
    // From Features::TEXTURE_FORMAT_16BIT_NORM
    WGPUNativeTextureFormat_R16Unorm = 0x00030001,
    /**
     * Red channel only. 16-bit signed integer per channel.
     * [-32767, 32767] converted to/from float [-1, 1] in shader.
     * Requires @ref WGPUNativeFeature_TextureFormat16bitNorm.
     */
    WGPUNativeTextureFormat_R16Snorm = 0x00030002,
    /**
     * Red and green channels. 16-bit unsigned integer per channel.
     * [0, 65535] converted to/from float [0, 1] in shader.
     * Requires @ref WGPUNativeFeature_TextureFormat16bitNorm.
     */
    WGPUNativeTextureFormat_Rg16Unorm = 0x00030003,
    /**
     * Red and green channels. 16-bit signed integer per channel.
     * [-32767, 32767] converted to/from float [-1, 1] in shader.
     * Requires @ref WGPUNativeFeature_TextureFormat16bitNorm.
     */
    WGPUNativeTextureFormat_Rg16Snorm = 0x00030004,
    /**
     * Red, green, blue, and alpha channels. 16-bit unsigned integer per channel.
     * [0, 65535] converted to/from float [0, 1] in shader.
     * Requires @ref WGPUNativeFeature_TextureFormat16bitNorm.
     */
    WGPUNativeTextureFormat_Rgba16Unorm = 0x00030005,
    /**
     * Red, green, blue, and alpha channels. 16-bit signed integer per channel.
     * [-32767, 32767] converted to/from float [-1, 1] in shader.
     * Requires @ref WGPUNativeFeature_TextureFormat16bitNorm.
     */
    WGPUNativeTextureFormat_Rgba16Snorm = 0x00030006,
    /**
     * YUV 4:2:0 chroma subsampled format (NV12).
     * Plane 0 contains R8Unorm luminance (Y), Plane 1 contains Rg8Unorm
     * chrominance (UV) at half width and half height.
     * Requires @ref WGPUNativeFeature_TextureFormatNv12.
     */
    WGPUNativeTextureFormat_NV12 = 0x00030007,
    /**
     * YUV 4:2:0 with 10 bits used from 16-bit channels (P010).
     * Plane 0 contains R16Unorm luminance (Y), Plane 1 contains Rg16Unorm
     * chrominance (UV) at half width and half height.
     */
    WGPUNativeTextureFormat_P010 = 0x00030008,
} WGPUNativeTextureFormat;

typedef struct WGPUImageSubresourceRange {
    WGPUTextureAspect aspect;
    uint32_t baseMipLevel;
    uint32_t mipLevelCount;
    uint32_t baseArrayLayer;
    uint32_t arrayLayerCount;
} WGPUImageSubresourceRange WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Describes how shader bound checks should be performed.
 */
typedef WGPUFlags WGPUShaderRuntimeChecks;

static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_None = 0x0000000000000000;
/**
 * Enforce bounds checks in shaders, even if the underlying driver doesn’t support doing so natively.
 */
static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_BoundsChecks = 0x0000000000000001;
/**
 * If not set, the caller MUST ensure that all passed shaders do not contain any infinite loops.
 */
static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_ForceLoopBounding = 0x0000000000000002;
/**
 * If not set, the caller MUST ensure that in all passed shaders every function operating on a ray
 * query must obey these rules (functions using wgsl naming).
 */
static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_RayQueryInitializationTracking = 0x0000000000000004;
/**
 * If not set, task shaders will not validate that the mesh shader grid they dispatch is within legal limits.
 */
static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_TaskShaderDispatchTracking = 0x0000000000000008;
/**
 * If not set, mesh shaders won’t clamp the output primitives’ vertex indices, which can lead to
 * undefined behavior and arbitrary memory access.
 */
static const WGPUShaderRuntimeChecks WGPUShaderRuntimeChecks_MeshShaderPrimitiveIndicesClamp = 0x0000000000000010;

/**
 * Bitmask of texture format feature flags returned by
 * @ref wgpuAdapterGetTextureFormatCapabilities.
 *
 * Bit values match those of @c wgpu_types::TextureFormatFeatureFlags.
 */
typedef uint32_t WGPUNativeTextureFormatFeatureFlags;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_None = 0x00000000;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_Filterable = 0x00000001;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_MultisampleX2 = 0x00000002;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_MultisampleX4 = 0x00000004;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_MultisampleX8 = 0x00000008;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_MultisampleX16 = 0x00000010;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_MultisampleResolve = 0x00000020;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_StorageReadOnly = 0x00000040;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_StorageWriteOnly = 0x00000080;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_StorageReadWrite = 0x00000100;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_StorageAtomic = 0x00000200;
static const WGPUNativeTextureFormatFeatureFlags WGPUNativeTextureFormatFeatureFlags_Blendable = 0x00000400;

/**
 * Texture format capabilities returned by @ref wgpuAdapterGetTextureFormatCapabilities.
 */
typedef struct WGPUNativeTextureFormatCapabilities {
    /** The set of @ref WGPUTextureUsage bits supported for this format on this adapter. */
    WGPUTextureUsage allowedUsages;
    /** Feature flags for this format (see @ref WGPUNativeTextureFormatFeatureFlags). */
    WGPUNativeTextureFormatFeatureFlags flags;
} WGPUNativeTextureFormatCapabilities WGPU_STRUCTURE_ATTRIBUTE;

#ifdef __cplusplus
extern "C"
{
#endif

    void wgpuGenerateReport(WGPUInstance instance, WGPUGlobalReport *report);
    size_t wgpuInstanceEnumerateAdapters(WGPUInstance instance, WGPU_NULLABLE WGPUInstanceEnumerateAdapterOptions const *options, WGPUAdapter *adapters);

    /**
     * Query the actual texture format capabilities supported by this adapter.
     *
     * Fills @p capabilities with the allowed usages and feature flags for @p format.
     * Returns @ref WGPUStatus_Error if @p format is not recognized.
     */
    WGPUStatus wgpuAdapterGetTextureFormatCapabilities(WGPUAdapter adapter, WGPUTextureFormat format, WGPUNativeTextureFormatCapabilities *capabilities);
    /**
     * Poll all devices owned by this instance.
     *
     * If @c wait is true, blocks until all devices are idle.
     * Returns true if all device queues are empty, false if work remains in flight.
     */
    WGPUBool wgpuInstancePollAllDevices(WGPUInstance instance, WGPUBool wait);

    WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, size_t commandCount, WGPUCommandBuffer const *commands);
    float wgpuQueueGetTimestampPeriod(WGPUQueue queue);

    // Returns true if the queue is empty, or false if there are more queue submissions still in flight.
    WGPUBool wgpuDevicePoll(WGPUDevice device, WGPUBool wait, WGPU_NULLABLE WGPUSubmissionIndex const *submissionIndex);
    WGPUShaderModule wgpuDeviceCreateShaderModulePassthrough(WGPUDevice device, WGPUShaderModuleDescriptorPassthrough const *descriptor);

    void wgpuSetLogCallback(WGPULogCallback callback, void *userdata);

    void wgpuSetLogLevel(WGPULogLevel level);

    uint32_t wgpuGetVersion(void);

    /**
     * Returns the backend-native `id<MTLDevice>` as an opaque pointer.
     *
     * The returned pointer is borrowed and remains valid only while `device` is alive.
     * Ownership is retained by wgpu-native; callers must not release or destroy it.
     * Returns NULL when the active backend is not Metal or when the handle is unavailable.
     */
    void *wgpuDeviceGetNativeMetalDevice(WGPUDevice device);

    /**
     * Returns the backend-native `id<MTLCommandQueue>` as an opaque pointer.
     *
     * The returned pointer is borrowed and remains valid only while `queue` is alive.
     * Ownership is retained by wgpu-native; callers must not release or destroy it.
     * Returns NULL when the active backend is not Metal or when the handle is unavailable.
     */
    void *wgpuQueueGetNativeMetalCommandQueue(WGPUQueue queue);

    /**
     * Returns the backend-native `id<MTLTexture>` as an opaque pointer.
     *
     * The returned pointer is borrowed and remains valid only while `texture` is alive.
     * Ownership is retained by wgpu-native; callers must not release or destroy it.
     * Returns NULL when the active backend is not Metal or when the handle is unavailable.
     */
    void *wgpuTextureGetNativeMetalTexture(WGPUTexture texture);

    void wgpuRenderPassEncoderSetImmediates(WGPURenderPassEncoder encoder, uint32_t offset, uint32_t sizeBytes, void const *data);
    void wgpuComputePassEncoderSetImmediates(WGPUComputePassEncoder encoder, uint32_t offset, uint32_t sizeBytes, void const *data);
    void wgpuRenderBundleEncoderSetImmediates(WGPURenderBundleEncoder encoder, uint32_t offset, uint32_t sizeBytes, void const *data);

    void wgpuRenderPassEncoderMultiDrawIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);
    void wgpuRenderPassEncoderMultiDrawIndexedIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);

    void wgpuRenderPassEncoderMultiDrawIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);
    void wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);

    void wgpuRenderPassEncoderDrawMeshTasks(WGPURenderPassEncoder encoder, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);
    void wgpuRenderPassEncoderDrawMeshTasksIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset);
    void wgpuRenderPassEncoderMultiDrawMeshTasksIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);
    void wgpuRenderPassEncoderMultiDrawMeshTasksIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer countBuffer, uint64_t countBufferOffset, uint32_t maxCount);

    void wgpuComputePassEncoderBeginPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);
    void wgpuComputePassEncoderEndPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder);
    void wgpuRenderPassEncoderBeginPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);
    void wgpuRenderPassEncoderEndPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder);

    void wgpuComputePassEncoderWriteTimestamp(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);
    void wgpuRenderPassEncoderWriteTimestamp(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);

    // Returns true if the capture was successfully started, or false if it failed to start or is not supported on the current platform.
    WGPUBool wgpuDeviceStartGraphicsDebuggerCapture(WGPUDevice device);
    void wgpuDeviceStopGraphicsDebuggerCapture(WGPUDevice device);

    void wgpuCommandEncoderClearTexture(WGPUCommandEncoder commandEncoder, WGPUTexture texture, WGPUImageSubresourceRange const * range);

    WGPUShaderModule wgpuDeviceCreateShaderModuleTrusted(WGPUDevice device, WGPUShaderModuleDescriptor const * descriptor, WGPUShaderRuntimeChecks runtimeChecks);

    WGPURenderPipeline wgpuDeviceCreateMeshPipeline(WGPUDevice device, WGPUMeshPipelineDescriptor const * descriptor);
    WGPUPipelineCache wgpuDeviceCreatePipelineCache(WGPUDevice device, WGPUPipelineCacheDescriptor const * descriptor);
    /**
     * Retrieve serialized cache data.
     *
     * Call with @c data = NULL to obtain the required buffer size.
     * Allocate a buffer of that size and call again with @c data pointing to it.
     * Returns 0 if the cache has no data or an error occurred.
     */
    size_t wgpuPipelineCacheGetData(WGPUPipelineCache cache, void * data);
    void wgpuPipelineCacheAddRef(WGPUPipelineCache cache);
    void wgpuPipelineCacheRelease(WGPUPipelineCache cache);

    // ── Acceleration structures ───────────────────────────────────────────────

    /** Create a bottom level acceleration structure. */
    WGPUBlas wgpuDeviceCreateBlas(WGPUDevice device, WGPUBlasDescriptor const *descriptor, WGPUBlasSizeDescriptors sizes);
    /** Create a top level acceleration structure. */
    WGPUTlas wgpuDeviceCreateTlas(WGPUDevice device, WGPUTlasDescriptor const *descriptor);

    /**
     * Return the device address handle for a BLAS.
     * Returns 0 if the handle is not available.
     */
    uint64_t wgpuBlasGetHandle(WGPUBlas blas);

    void wgpuBlasAddRef(WGPUBlas blas);
    void wgpuBlasRelease(WGPUBlas blas);
    void wgpuTlasAddRef(WGPUTlas tlas);
    void wgpuTlasRelease(WGPUTlas tlas);

    /** Begin preparing a BLAS for compaction. Calls @p callbackInfo when ready. */
    void wgpuBlasPrepareCompactAsync(WGPUBlas blas, WGPUBlasCompactCallbackInfo callbackInfo);
    /** Returns true if the BLAS is ready to be compacted via @ref wgpuQueueCompactBlas. */
    WGPUBool wgpuBlasReadyForCompaction(WGPUBlas blas);

    /**
     * Compact a BLAS. Returns a new (smaller) BLAS handle.
     * The old handle remains valid and must be released separately.
     */
    WGPUBlas wgpuQueueCompactBlas(WGPUQueue queue, WGPUBlas blas);

    /** Query internal wgpu-core/HAL resource counters for debugging. */
    WGPUInternalCounters wgpuDeviceGetInternalCounters(WGPUDevice device);

    /** Build acceleration structures on the command encoder. */
    void wgpuCommandEncoderBuildAccelerationStructures(
        WGPUCommandEncoder commandEncoder,
        size_t blasEntryCount, WGPUBlasBuildEntry const *blasEntries,
        size_t tlasPackageCount, WGPUTlasPackage const *tlasPackages);

    /**
     * Mark acceleration structures as already built (for external builds).
     * Registers them with the wgpu-core tracker without issuing GPU commands.
     */
    void wgpuCommandEncoderMarkAccelerationStructuresBuilt(
        WGPUCommandEncoder commandEncoder,
        size_t blasCount, WGPUBlas const *blases,
        size_t tlasCount, WGPUTlas const *tlases);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
