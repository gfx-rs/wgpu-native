/**
 * Copyright 2019-2023 wgpu-native developers
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

/** @file */

/**
 * \mainpage
 *
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

#if !defined(_wgpu_EXTEND_ENUM)
#ifdef __cplusplus
#define _wgpu_EXTEND_ENUM(E, N, V) static const E N = E(V)
#else
#define _wgpu_EXTEND_ENUM(E, N, V) enum { N = (E)(V) }
#endif
#endif // !defined(_wgpu_EXTEND_ENUM)

/**
 * \defgroup Constants Constants
 * \brief Constants.
 *
 * @{
 */

/**
 * 'True' value of @ref WGPUBool.
 *
 * @remark It's not usually necessary to use this, as `true` (from
 * `stdbool.h` or C++) casts to the same value.
 */
#define WGPU_TRUE (UINT32_C(1))
/**
 * 'False' value of @ref WGPUBool.
 *
 * @remark It's not usually necessary to use this, as `false` (from
 * `stdbool.h` or C++) casts to the same value.
 */
#define WGPU_FALSE (UINT32_C(0))

/** @} */

/**
 * \defgroup UtilityTypes Utility Types
 *
 * @{
 */

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

/** @} */

/**
 * \defgroup Objects Objects
 * \brief Opaque, non-dispatchable handles to WebGPU objects.
 *
 * @{
 */

/** @} */

// Structure forward declarations
struct WGPUBindGroupEntryExtras;
struct WGPUBindGroupLayoutEntryExtras;
struct WGPUDeviceExtras;
struct WGPUImageSubresourceRange;
struct WGPUInstanceEnumerateAdapterOptions;
struct WGPUNativeLimits;
struct WGPUPrimitiveStateExtras;
struct WGPUQuerySetDescriptorExtras;
struct WGPURegistryReport;
struct WGPUSamplerDescriptorExtras;
struct WGPUShaderDefine;
struct WGPUShaderModuleDescriptorSpirV;
struct WGPUSurfaceConfigurationExtras;
struct WGPUSurfaceSourceOhosNativeWindow;
struct WGPUSurfaceSourceSwapChainPanel;
struct WGPUWaylandDisplayHandle;
struct WGPUXcbDisplayHandle;
struct WGPUXlibDisplayHandle;
struct WGPUNativeDisplayHandle;
struct WGPUHubReport;
struct WGPUShaderSourceGLSL;
struct WGPUGlobalReport;
struct WGPUInstanceExtras;

// Callback info structure forward declarations
struct WGPULogCallbackInfo;

/**
 * \defgroup Enumerations Enumerations
 * \brief Enums.
 *
 * @{
 */

_wgpu_EXTEND_ENUM(WGPUAddressMode, WGPUNativeAddressMode_ClampToBorder, 0x00000004);

/**
 * Native-only device features.
 *
 * These extend the standard @c WGPUFeatureName values and can be passed to
 * @c WGPUDeviceDescriptor::requiredFeatures to request additional
 * capabilities when creating a device.
 */
/**
 * Allows the use of immediate data: small, fast blocks of memory
 * that can be updated inside a render pass, compute pass, or render
 * bundle encoder.
 *
 * Enables @ref wgpuRenderPassEncoderSetImmediates,
 * @ref wgpuComputePassEncoderSetImmediates,
 * @ref wgpuRenderBundleEncoderSetImmediates,
 * non-zero @c immediateSize in @ref WGPUPipelineLayout,
 * and non-zero @c maxImmediateSize in @ref WGPULimits.
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_Immediates, 0x00030001);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureAdapterSpecificFormatFeatures, 0x00030002);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_MultiDrawIndirectCount, 0x00030004);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_VertexWritableStorage, 0x00030005);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureBindingArray, 0x00030006);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing, 0x00030007);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_PipelineStatisticsQuery, 0x00030008);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_StorageResourceBindingArray, 0x00030009);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_PartiallyBoundBindingArray, 0x0003000A);
/**
 * Enables normalized 16-bit texture formats:
 * @ref WGPUTextureFormat_R16Unorm, @ref WGPUTextureFormat_R16Snorm,
 * @ref WGPUTextureFormat_RG16Unorm, @ref WGPUTextureFormat_RG16Snorm,
 * @ref WGPUTextureFormat_RGBA16Unorm, @ref WGPUTextureFormat_RGBA16Snorm.
 *
 * Supported platforms:
 * - Vulkan
 * - DX12
 * - Metal
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureFormat16bitNorm, 0x0003000B);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureCompressionAstcHdr, 0x0003000C);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_MappablePrimaryBuffers, 0x0003000E);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_BufferBindingArray, 0x0003000F);
/**
 * Allows shaders to index storage texture resource
 * arrays with dynamically non-uniform values.
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_StorageTextureArrayNonUniformIndexing, 0x00030010);
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_AddressModeClampToZero, 0x00030011);
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_AddressModeClampToBorder, 0x00030012);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_PolygonModeLine, 0x00030013);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_PolygonModePoint, 0x00030014);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ConservativeRasterization, 0x00030015);
/**
 * Enables clear to zero for textures.
 *
 * Supported platforms:
 * - All
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ClearTexture, 0x00030016);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_Multiview, 0x00030018);
/**
 * Enables using 64-bit types for vertex attributes.
 *
 * Requires @ref WGPUNativeFeature_ShaderF64.
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_VertexAttribute64bit, 0x00030019);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureFormatNv12, 0x0003001A);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_RayQuery, 0x0003001C);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderF64, 0x0003001D);
/**
 * Allows shaders to use i16. Not currently supported in naga, only
 * available through SPIR-V passthrough.
 *
 * Supported platforms:
 * - Vulkan
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderI16, 0x0003001E);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderEarlyDepthTest, 0x00030020);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_Subgroup, 0x00030021);
/**
 * Allows vertex shaders to use the subgroup operation built-ins and
 * perform subgroup operations (except barriers).
 *
 * Supported platforms:
 * - Vulkan
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_SubgroupVertex, 0x00030022);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_SubgroupBarrier, 0x00030023);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TimestampQueryInsideEncoders, 0x00030024);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TimestampQueryInsidePasses, 0x00030025);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderInt64, 0x00030026);
/**
 * Allows shaders to use f32 atomic load, store, add, sub, and exchange.
 *
 * Supported platforms:
 * - Metal (with MSL 3.0+ and Apple7+/Mac2)
 * - Vulkan (with [VK_EXT_shader_atomic_float])
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderFloat32Atomic, 0x00030027);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureAtomic, 0x00030028);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureFormatP010, 0x00030029);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_PipelineCache, 0x0003002B);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderInt64AtomicMinMax, 0x0003002C);
/**
 * Allows shaders to use all i64 and u64 atomic operations.
 *
 * Supported platforms:
 * - Vulkan (with VK_KHR_shader_atomic_int64)
 * - DX12 (with SM 6.6+)
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderInt64AtomicAllOps, 0x0003002D);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_TextureInt64Atomic, 0x00030030);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderBarycentrics, 0x00030037);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_SelectiveMultiview, 0x00030038);
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_MultisampleArray, 0x0003003A);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_CooperativeMatrix, 0x0003003B);
/**
 * Enables shader per-vertex attributes.
 *
 * Supported platforms:
 * - Vulkan (with VK_KHR_fragment_shader_barycentric)
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderPerVertex, 0x0003003C);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_ShaderDrawIndex, 0x0003003D);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_AccelerationStructureBindingArray, 0x0003003E);
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
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_MemoryDecorationCoherent, 0x0003003F);
/**
 * Enables the `@volatile` memory decoration on storage buffer variables.
 *
 * Backend mapping:
 * - Vulkan
 * - GLES (ES 3.1+ / GL 4.3+)
 *
 * This is a native only feature.
 */
_wgpu_EXTEND_ENUM(WGPUFeatureName, WGPUNativeFeature_MemoryDecorationVolatile, 0x00030040);

_wgpu_EXTEND_ENUM(WGPUQueryType, WGPUNativeQueryType_PipelineStatistics, 0x00030000);

/**
 * Identifies @ref WGPUDeviceExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_DeviceExtras, 0x00030001);
/**
 * Identifies @ref WGPUNativeLimits.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_NativeLimits, 0x00030002);
/**
 * Identifies @ref WGPUShaderSourceGLSL.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_ShaderSourceGLSL, 0x00030003);
/**
 * Identifies @ref WGPUInstanceExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_InstanceExtras, 0x00030004);
/**
 * Identifies @ref WGPUBindGroupEntryExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_BindGroupEntryExtras, 0x00030005);
/**
 * Identifies @ref WGPUBindGroupLayoutEntryExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_BindGroupLayoutEntryExtras, 0x00030006);
/**
 * Identifies @ref WGPUQuerySetDescriptorExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_QuerySetDescriptorExtras, 0x00030007);
/**
 * Identifies @ref WGPUSurfaceConfigurationExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_SurfaceConfigurationExtras, 0x00030008);
/**
 * Identifies @ref WGPUSurfaceSourceSwapChainPanel.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_SurfaceSourceSwapChainPanel, 0x00030009);
/**
 * Identifies @ref WGPUPrimitiveStateExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_PrimitiveStateExtras, 0x0003000A);
/**
 * Identifies @ref WGPUSamplerDescriptorExtras.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_SamplerDescriptorExtras, 0x0003000B);
/**
 * Identifies @ref WGPUSurfaceSourceOhosNativeWindow.
 */
_wgpu_EXTEND_ENUM(WGPUSType, WGPUSType_SurfaceSourceOhosNativeWindow, 0x0003000C);

/**
 * Additional surface-get-current-texture status codes defined by wgpu-native.
 *
 * These extend the standard @c WGPUSurfaceGetCurrentTextureStatus values.
 */
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
_wgpu_EXTEND_ENUM(WGPUSurfaceGetCurrentTextureStatus, WGPUSurfaceGetCurrentTextureStatus_Occluded, 0x00030001);

/**
 * YUV 4:2:0 chroma subsampled format (NV12).
 * Plane 0 contains R8Unorm luminance (Y), Plane 1 contains Rg8Unorm
 * chrominance (UV) at half width and half height.
 * Requires @ref WGPUNativeFeature_TextureFormatNv12.
 */
_wgpu_EXTEND_ENUM(WGPUTextureFormat, WGPUNativeTextureFormat_NV12, 0x00030007);
/**
 * YUV 4:2:0 with 10 bits used from 16-bit channels (P010).
 * Plane 0 contains R16Unorm luminance (Y), Plane 1 contains Rg16Unorm
 * chrominance (UV) at half width and half height.
 */
_wgpu_EXTEND_ENUM(WGPUTextureFormat, WGPUNativeTextureFormat_P010, 0x00030008);

/**
 * Discriminant for WGPUNativeDisplayHandle.
 *
 * Identifies which platform's display connection is stored in the tagged union.
 * Use @ref WGPUNativeDisplayHandleType_None (the default when zero-initialized) when
 * no display handle is needed. Platforms with no display connection data (Windows,
 * macOS, iOS, Android) should use @ref WGPUNativeDisplayHandleType_None.
 */
typedef enum WGPUNativeDisplayHandleType {
    /**
     * `0`. No display handle provided.
     */
    WGPUNativeDisplayHandleType_None = 0x00000000,
    /**
     * X11 display connection via Xlib. See @ref WGPUXlibDisplayHandle.
     */
    WGPUNativeDisplayHandleType_Xlib = 0x00000001,
    /**
     * X11 display connection via XCB. See @ref WGPUXcbDisplayHandle.
     */
    WGPUNativeDisplayHandleType_Xcb = 0x00000002,
    /**
     * Wayland display connection. See @ref WGPUWaylandDisplayHandle.
     */
    WGPUNativeDisplayHandleType_Wayland = 0x00000003,
    WGPUNativeDisplayHandleType_Force32 = 0x7FFFFFFF
} WGPUNativeDisplayHandleType WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUDx12Compiler {
    /**
     * `0`.
     */
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
     */
    WGPUDx12Compiler_Dxc = 0x00000002,
    WGPUDx12Compiler_Force32 = 0x7FFFFFFF
} WGPUDx12Compiler WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUDx12SwapchainKind {
    /**
     * `0`.
     */
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
} WGPUDx12SwapchainKind WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUDxcMaxShaderModel {
    /**
     * `0`.
     */
    WGPUDxcMaxShaderModel_V6_0 = 0x00000000,
    /**
     * Shader Model 6.1
     */
    WGPUDxcMaxShaderModel_V6_1 = 0x00000001,
    /**
     * Shader Model 6.2
     */
    WGPUDxcMaxShaderModel_V6_2 = 0x00000002,
    /**
     * Shader Model 6.3
     */
    WGPUDxcMaxShaderModel_V6_3 = 0x00000003,
    /**
     * Shader Model 6.4
     */
    WGPUDxcMaxShaderModel_V6_4 = 0x00000004,
    /**
     * Shader Model 6.5
     */
    WGPUDxcMaxShaderModel_V6_5 = 0x00000005,
    /**
     * Shader Model 6.6
     */
    WGPUDxcMaxShaderModel_V6_6 = 0x00000006,
    /**
     * Shader Model 6.7
     */
    WGPUDxcMaxShaderModel_V6_7 = 0x00000007,
    WGPUDxcMaxShaderModel_Force32 = 0x7FFFFFFF
} WGPUDxcMaxShaderModel WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUGles3MinorVersion {
    /**
     * `0`.
     */
    WGPUGles3MinorVersion_Automatic = 0x00000000,
    /**
     * Request an ES 3.0 context.
     */
    WGPUGles3MinorVersion_Version0 = 0x00000001,
    /**
     * Request an ES 3.1 context.
     */
    WGPUGles3MinorVersion_Version1 = 0x00000002,
    /**
     * Request an ES 3.2 context.
     */
    WGPUGles3MinorVersion_Version2 = 0x00000003,
    WGPUGles3MinorVersion_Force32 = 0x7FFFFFFF
} WGPUGles3MinorVersion WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUGLFenceBehaviour {
    /**
     * `0`.
     */
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
} WGPUGLFenceBehaviour WGPU_ENUM_ATTRIBUTE;

typedef enum WGPULogLevel {
    /**
     * `0`.
     */
    WGPULogLevel_Off = 0x00000000,
    /**
     * Only error messages.
     */
    WGPULogLevel_Error = 0x00000001,
    /**
     * Errors and warnings.
     */
    WGPULogLevel_Warn = 0x00000002,
    /**
     * Errors, warnings, and informational messages.
     */
    WGPULogLevel_Info = 0x00000003,
    /**
     * Errors, warnings, informational, and debug messages.
     */
    WGPULogLevel_Debug = 0x00000004,
    /**
     * All messages, including very verbose trace-level output.
     */
    WGPULogLevel_Trace = 0x00000005,
    WGPULogLevel_Force32 = 0x7FFFFFFF
} WGPULogLevel WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUMemoryHints {
    /**
     * `0`. Same as Performance (the wgpu default).
     */
    WGPUMemoryHints_Undefined = 0x00000000,
    /**
     * Favor performance over memory usage.
     */
    WGPUMemoryHints_Performance = 0x00000001,
    /**
     * Favor memory usage over performance.
     */
    WGPUMemoryHints_MemoryUsage = 0x00000002,
    /**
     * Choose the suballocated memory block size range explicitly via
     * @ref WGPUDeviceExtras::suballocatedDeviceMemoryBlockSizeStart and
     * @ref WGPUDeviceExtras::suballocatedDeviceMemoryBlockSizeEnd.
     */
    WGPUMemoryHints_Manual = 0x00000003,
    WGPUMemoryHints_Force32 = 0x7FFFFFFF
} WGPUMemoryHints WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUPipelineStatisticName {
    /**
     * `0`.
     */
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

typedef enum WGPUPolygonMode {
    /**
     * `0`.
     */
    WGPUPolygonMode_Fill = 0x00000000,
    /**
     * Polygons are drawn as line segments (wireframe).
     * Requires @ref WGPUNativeFeature_PolygonModeLine.
     */
    WGPUPolygonMode_Line = 0x00000001,
    /**
     * Polygons are drawn as points (vertices only).
     * Requires @ref WGPUNativeFeature_PolygonModePoint.
     */
    WGPUPolygonMode_Point = 0x00000002,
    WGPUPolygonMode_Force32 = 0x7FFFFFFF
} WGPUPolygonMode WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUSamplerBorderColor {
    /**
     * `0`.
     */
    WGPUSamplerBorderColor_Undefined = 0x00000000,
    WGPUSamplerBorderColor_TransparentBlack = 0x00000001,
    WGPUSamplerBorderColor_OpaqueBlack = 0x00000002,
    WGPUSamplerBorderColor_OpaqueWhite = 0x00000003,
    WGPUSamplerBorderColor_Zero = 0x00000004,
    WGPUSamplerBorderColor_Force32 = 0x7FFFFFFF
} WGPUSamplerBorderColor WGPU_ENUM_ATTRIBUTE;

/** @} */

/**
 * \defgroup Bitflags Bitflags
 * \brief Type and constant definitions for bitflag types.
 *
 * @{
 */

/**
 * Bitflags selecting which graphics backends the @ref WGPUInstance should
 * enable.
 *
 * Pass in the @c backends field of @ref WGPUInstanceExtras.
 *
 * For reserved non-standard bitflag values, see @ref BitflagRegistry.
 */
typedef WGPUFlags WGPUInstanceBackend;
/**
 * `0`.
 * All backends (the default when zero-initialized).
 */
static const WGPUInstanceBackend WGPUInstanceBackend_All = 0x0000000000000000;
/**
 * Vulkan backend.
 * Supported on Windows, Linux/Android, and macOS/iOS via Vulkan Portability.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Vulkan = 0x0000000000000001;
/**
 * OpenGL / OpenGL ES backend.
 * Supported on Linux/Android, the web via WebGL, and Windows/macOS via ANGLE.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_GL = 0x0000000000000002;
/**
 * Metal backend.
 * Supported on macOS and iOS.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Metal = 0x0000000000000004;
/**
 * Direct3D 12 backend.
 * Supported on Windows 10 and later.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_DX12 = 0x0000000000000008;
/**
 * Browser WebGPU backend.
 * Supported when targeting the web through WebAssembly.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_BrowserWebGPU = 0x0000000000000020;
/**
 * `NativeVulkan | NativeMetal | NativeDX12 | NativeBrowserWebGPU`.
 * Primary (first-tier) backends: Vulkan, Metal, DX12, and BrowserWebGPU.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Primary = 0x000000000000002D;
/**
 * `NativeGL`.
 * Secondary (second-tier) backends: GL.
 */
static const WGPUInstanceBackend WGPUInstanceBackend_Secondary = 0x0000000000000002;

/**
 * Bitflags controlling instance debugging and validation behavior.
 *
 * These are not part of the WebGPU standard.
 *
 * Pass in the @c flags field of @ref WGPUInstanceExtras.
 *
 * For reserved non-standard bitflag values, see @ref BitflagRegistry.
 */
typedef WGPUFlags WGPUInstanceFlag;
/**
 * `0`.
 * No flags set.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Empty = 0x0000000000000000;
/**
 * Generate debug information in shaders and objects.
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_DEBUG environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Debug = 0x0000000000000001;
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
static const WGPUInstanceFlag WGPUInstanceFlag_Validation = 0x0000000000000002;
/**
 * Don't pass labels to the backend API (wgpu-hal).
 *
 * When using @ref WGPUInstanceFlag_WithEnv, takes value from the
 * @c WGPU_DISCARD_HAL_LABELS environment variable.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_DiscardHalLabels = 0x0000000000000004;
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
static const WGPUInstanceFlag WGPUInstanceFlag_AllowUnderlyingNoncompliantAdapter = 0x0000000000000008;
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
static const WGPUInstanceFlag WGPUInstanceFlag_GPUBasedValidation = 0x0000000000000010;
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
static const WGPUInstanceFlag WGPUInstanceFlag_ValidationIndirectCall = 0x0000000000000020;
/**
 * Enable automatic timestamp normalization. When enabled,
 * @c wgpuCommandEncoderResolveQuerySet will automatically normalize timestamps
 * to nanoseconds instead of returning raw timestamp values.
 *
 * This introduces a compute shader into the resolution of query sets. When
 * enabled, the timestamp period returned by @ref wgpuQueueGetTimestampPeriod
 * will always be @c 1.0.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_AutomaticTimestampNormalization = 0x0000000000000040;
/**
 * Use the default flags for the current build configuration.
 * In debug builds, this typically enables @ref WGPUInstanceFlag_Debug and
 * @ref WGPUInstanceFlag_Validation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Default = 0x0000000001000000;
/**
 * Convenience alias that enables @ref WGPUInstanceFlag_Debug and
 * @ref WGPUInstanceFlag_Validation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_Debugging = 0x0000000002000000;
/**
 * Convenience alias that enables @ref WGPUInstanceFlag_Debug,
 * @ref WGPUInstanceFlag_Validation, and
 * @ref WGPUInstanceFlag_GPUBasedValidation.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_AdvancedDebugging = 0x0000000004000000;
/**
 * Modify the flags based on environment variables. Flags with environment
 * variable support (e.g. @c WGPU_DEBUG, @c WGPU_VALIDATION) will be read
 * from the process environment and applied on top of the explicitly set flags.
 */
static const WGPUInstanceFlag WGPUInstanceFlag_WithEnv = 0x0000000008000000;

/**
 * Describes how shader bound checks should be performed.
 *
 * For reserved non-standard bitflag values, see @ref BitflagRegistry.
 */
typedef WGPUFlags WGPUShaderRuntimeChecks;
/**
 * `0`.
 * No runtime checks set.
 */
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

/** @} */

/**
 * \defgroup Callbacks Callbacks
 * \brief Callbacks through which asynchronous functions return.
 *
 * @{
 */

/**
 * See also @ref CallbackError.
 *
 * @param message
 * This parameter is @ref PassedWithoutOwnership.
 */
typedef void (*WGPULogCallback)(WGPULogLevel level, WGPUStringView message, WGPU_NULLABLE void* userdata) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup Structures Structures
 * \brief Descriptors and other transparent structures.
 *
 * @{
 */

/**
 * \defgroup CallbackInfoStructs Callback Info Structs
 * \brief Callback info structures that are used in asynchronous functions.
 *
 * @{
 */

/** @} */

/**
 * Default values can be set using @ref WGPU_BIND_GROUP_ENTRY_EXTRAS_INIT as initializer.
 */
typedef struct WGPUBindGroupEntryExtras {
    WGPUChainedStruct chain;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE WGPUBuffer const * buffers;
    /**
     * Array count for `buffers`. The `INIT` macro sets this to 0.
     */
    size_t bufferCount;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE WGPUSampler const * samplers;
    /**
     * Array count for `samplers`. The `INIT` macro sets this to 0.
     */
    size_t samplerCount;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE WGPUTextureView const * textureViews;
    /**
     * Array count for `textureViews`. The `INIT` macro sets this to 0.
     */
    size_t textureViewCount;
} WGPUBindGroupEntryExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUBindGroupEntryExtras.
 */
#define WGPU_BIND_GROUP_ENTRY_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUBindGroupEntryExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_BindGroupEntryExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.bufferCount=*/0 _wgpu_COMMA \
    /*.buffers=*/NULL _wgpu_COMMA \
    /*.samplerCount=*/0 _wgpu_COMMA \
    /*.samplers=*/NULL _wgpu_COMMA \
    /*.textureViewCount=*/0 _wgpu_COMMA \
    /*.textureViews=*/NULL _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_BIND_GROUP_LAYOUT_ENTRY_EXTRAS_INIT as initializer.
 */
typedef struct WGPUBindGroupLayoutEntryExtras {
    WGPUChainedStruct chain;
    /**
     * Number of resources in this binding array slot. Corresponds to the
     * array size in the shader (e.g. @c binding_array<T, @c N>).
     *
     * The `INIT` macro sets this to `0`.
     */
    uint32_t count;
} WGPUBindGroupLayoutEntryExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUBindGroupLayoutEntryExtras.
 */
#define WGPU_BIND_GROUP_LAYOUT_ENTRY_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUBindGroupLayoutEntryExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_BindGroupLayoutEntryExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.count=*/0 _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_DEVICE_EXTRAS_INIT as initializer.
 */
typedef struct WGPUDeviceExtras {
    WGPUChainedStruct chain;
    /**
     * File system path for API trace output.
     *
     * When set to a non-empty path, wgpu will record all API calls to
     * the given directory, which can later be replayed for debugging.
     * An empty/undefined string view disables tracing.
     *
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView tracePath;
    /**
     * Hints to the backend memory allocator.
     * Zero-initialized yields @ref WGPUMemoryHints_Undefined (Performance).
     *
     * The `INIT` macro sets this to @ref WGPUMemoryHints_Undefined.
     */
    WGPUMemoryHints memoryHints;
    /**
     * Initial suballocated device-memory block size, in bytes. Only used
     * with @ref WGPUMemoryHints_Manual.
     *
     * After running out of space in existing blocks, the backend may grow
     * subsequent block sizes up to
     * @ref WGPUDeviceExtras::suballocatedDeviceMemoryBlockSizeEnd. This does
     * not limit resource sizes: a resource that does not fit is typically
     * placed in a dedicated memory block.
     *
     * The `INIT` macro sets this to `0`.
     */
    uint64_t suballocatedDeviceMemoryBlockSizeStart;
    /**
     * See @ref WGPUDeviceExtras::suballocatedDeviceMemoryBlockSizeStart.
     *
     * The `INIT` macro sets this to `0`.
     */
    uint64_t suballocatedDeviceMemoryBlockSizeEnd;
} WGPUDeviceExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUDeviceExtras.
 */
#define WGPU_DEVICE_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUDeviceExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_DeviceExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.tracePath=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
    /*.memoryHints=*/WGPUMemoryHints_Undefined _wgpu_COMMA \
    /*.suballocatedDeviceMemoryBlockSizeStart=*/0 _wgpu_COMMA \
    /*.suballocatedDeviceMemoryBlockSizeEnd=*/0 _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_IMAGE_SUBRESOURCE_RANGE_INIT as initializer.
 */
typedef struct WGPUImageSubresourceRange {
    /**
     * The `INIT` macro sets this to @ref WGPUTextureAspect_Undefined.
     */
    WGPUTextureAspect aspect;
    /**
     * The `INIT` macro sets this to `0`.
     */
    uint32_t baseMipLevel;
    /**
     * The `INIT` macro sets this to `0`.
     */
    uint32_t mipLevelCount;
    /**
     * The `INIT` macro sets this to `0`.
     */
    uint32_t baseArrayLayer;
    /**
     * The `INIT` macro sets this to `0`.
     */
    uint32_t arrayLayerCount;
} WGPUImageSubresourceRange WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUImageSubresourceRange.
 */
#define WGPU_IMAGE_SUBRESOURCE_RANGE_INIT _wgpu_MAKE_INIT_STRUCT(WGPUImageSubresourceRange, { \
    /*.aspect=*/WGPUTextureAspect_Undefined _wgpu_COMMA \
    /*.baseMipLevel=*/0 _wgpu_COMMA \
    /*.mipLevelCount=*/0 _wgpu_COMMA \
    /*.baseArrayLayer=*/0 _wgpu_COMMA \
    /*.arrayLayerCount=*/0 _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_INSTANCE_ENUMERATE_ADAPTER_OPTIONS_INIT as initializer.
 */
typedef struct WGPUInstanceEnumerateAdapterOptions {
    WGPUChainedStruct * nextInChain;
    /**
     * The `INIT` macro sets this to @ref WGPUInstanceBackend_None.
     */
    WGPUInstanceBackend backends;
} WGPUInstanceEnumerateAdapterOptions WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUInstanceEnumerateAdapterOptions.
 */
#define WGPU_INSTANCE_ENUMERATE_ADAPTER_OPTIONS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUInstanceEnumerateAdapterOptions, { \
    /*.nextInChain=*/NULL _wgpu_COMMA \
    /*.backends=*/WGPUInstanceBackend_None _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_NATIVE_LIMITS_INIT as initializer.
 */
typedef struct WGPUNativeLimits {
    WGPUChainedStruct chain;
    /**
     * Maximum number of live non-sampler bindings.
     *
     * Default is 1,000,000. Only meaningful on D3D12.
     *
     * @b Warning: On integrated GPUs, large values can cause significant
     * system RAM consumption.
     *
     * The `INIT` macro sets this to @ref WGPU_LIMIT_U32_UNDEFINED.
     */
    uint32_t maxNonSamplerBindings;
    /**
     * Maximum number of individual resources within binding arrays that can be accessed
     * in a single shader stage. Applies to all types of bindings except samplers.
     *
     * The `INIT` macro sets this to @ref WGPU_LIMIT_U32_UNDEFINED.
     */
    uint32_t maxBindingArrayElementsPerShaderStage;
    /**
     * Maximum number of individual samplers within binding arrays that
     * can be accessed in a single shader stage.
     *
     * The `INIT` macro sets this to @ref WGPU_LIMIT_U32_UNDEFINED.
     */
    uint32_t maxBindingArraySamplerElementsPerShaderStage;
    /**
     * The maximum number of views that can be used in multiview rendering.
     *
     * The `INIT` macro sets this to @ref WGPU_LIMIT_U32_UNDEFINED.
     */
    uint32_t maxMultiviewViewCount;
} WGPUNativeLimits WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUNativeLimits.
 */
#define WGPU_NATIVE_LIMITS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUNativeLimits, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_NativeLimits _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.maxNonSamplerBindings=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxBindingArrayElementsPerShaderStage=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxBindingArraySamplerElementsPerShaderStage=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
    /*.maxMultiviewViewCount=*/WGPU_LIMIT_U32_UNDEFINED _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_PRIMITIVE_STATE_EXTRAS_INIT as initializer.
 */
typedef struct WGPUPrimitiveStateExtras {
    WGPUChainedStruct chain;
    /**
     * Controls the way each polygon is rasterized.
     * See @ref WGPUPolygonMode. Defaults to @ref WGPUPolygonMode_Fill.
     *
     * The `INIT` macro sets this to (@ref WGPUPolygonMode)0.
     */
    WGPUPolygonMode polygonMode;
    /**
     * If set to true, the primitives are rendered with conservative
     * overestimation. Only valid when @c polygonMode is
     * @ref WGPUPolygonMode_Fill.
     * Requires @ref WGPUNativeFeature_ConservativeRasterization.
     *
     * The `INIT` macro sets this to `WGPU_FALSE`.
     */
    WGPUBool conservative;
} WGPUPrimitiveStateExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUPrimitiveStateExtras.
 */
#define WGPU_PRIMITIVE_STATE_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUPrimitiveStateExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_PrimitiveStateExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.polygonMode=*/_wgpu_ENUM_ZERO_INIT(WGPUPolygonMode) _wgpu_COMMA \
    /*.conservative=*/WGPU_FALSE _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_QUERY_SET_DESCRIPTOR_EXTRAS_INIT as initializer.
 */
typedef struct WGPUQuerySetDescriptorExtras {
    WGPUChainedStruct chain;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPUPipelineStatisticName const * pipelineStatistics;
    /**
     * Array count for `pipelineStatistics`. The `INIT` macro sets this to 0.
     */
    size_t pipelineStatisticCount;
} WGPUQuerySetDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUQuerySetDescriptorExtras.
 */
#define WGPU_QUERY_SET_DESCRIPTOR_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUQuerySetDescriptorExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_QuerySetDescriptorExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.pipelineStatisticCount=*/0 _wgpu_COMMA \
    /*.pipelineStatistics=*/NULL _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_REGISTRY_REPORT_INIT as initializer.
 */
typedef struct WGPURegistryReport {
    /**
     * The `INIT` macro sets this to `0`.
     */
    size_t numAllocated;
    /**
     * The `INIT` macro sets this to `0`.
     */
    size_t numKeptFromUser;
    /**
     * The `INIT` macro sets this to `0`.
     */
    size_t numReleasedFromUser;
    /**
     * The `INIT` macro sets this to `0`.
     */
    size_t elementSize;
} WGPURegistryReport WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPURegistryReport.
 */
#define WGPU_REGISTRY_REPORT_INIT _wgpu_MAKE_INIT_STRUCT(WGPURegistryReport, { \
    /*.numAllocated=*/0 _wgpu_COMMA \
    /*.numKeptFromUser=*/0 _wgpu_COMMA \
    /*.numReleasedFromUser=*/0 _wgpu_COMMA \
    /*.elementSize=*/0 _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_SAMPLER_DESCRIPTOR_EXTRAS_INIT as initializer.
 */
typedef struct WGPUSamplerDescriptorExtras {
    WGPUChainedStruct chain;
    /**
     * The `INIT` macro sets this to @ref WGPUSamplerBorderColor_Undefined.
     */
    WGPUSamplerBorderColor samplerBorderColor;
} WGPUSamplerDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUSamplerDescriptorExtras.
 */
#define WGPU_SAMPLER_DESCRIPTOR_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUSamplerDescriptorExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_SamplerDescriptorExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.samplerBorderColor=*/WGPUSamplerBorderColor_Undefined _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_SHADER_DEFINE_INIT as initializer.
 */
typedef struct WGPUShaderDefine {
    /**
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView name;
    /**
     * The value of the preprocessor macro (e.g. @c "1").
     *
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView value;
} WGPUShaderDefine WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUShaderDefine.
 */
#define WGPU_SHADER_DEFINE_INIT _wgpu_MAKE_INIT_STRUCT(WGPUShaderDefine, { \
    /*.name=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
    /*.value=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_SHADER_MODULE_DESCRIPTOR_SPIR_V_INIT as initializer.
 */
typedef struct WGPUShaderModuleDescriptorSpirV {
    /**
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView label;
    /**
     * Number of 32-bit words in @c source.
     *
     * The `INIT` macro sets this to `0`.
     */
    uint32_t sourceSize;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    uint32_t const * source;
} WGPUShaderModuleDescriptorSpirV WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUShaderModuleDescriptorSpirV.
 */
#define WGPU_SHADER_MODULE_DESCRIPTOR_SPIR_V_INIT _wgpu_MAKE_INIT_STRUCT(WGPUShaderModuleDescriptorSpirV, { \
    /*.label=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
    /*.sourceSize=*/0 _wgpu_COMMA \
    /*.source=*/NULL _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_SURFACE_CONFIGURATION_EXTRAS_INIT as initializer.
 */
typedef struct WGPUSurfaceConfigurationExtras {
    WGPUChainedStruct chain;
    /**
     * Desired maximum number of frames in flight (i.e. the number of monitor
     * refreshes between @c wgpuSurfaceGetCurrentTexture and presentation).
     *
     * - 1: Minimize latency (CPU and GPU cannot run in parallel).
     * - 2: Balance between latency and throughput (the default).
     * - 3+: Maximize throughput.
     *
     * The `INIT` macro sets this to `0`.
     */
    uint32_t desiredMaximumFrameLatency;
} WGPUSurfaceConfigurationExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUSurfaceConfigurationExtras.
 */
#define WGPU_SURFACE_CONFIGURATION_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUSurfaceConfigurationExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_SurfaceConfigurationExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.desiredMaximumFrameLatency=*/0 _wgpu_COMMA \
})

/**
 * Chained in @ref WGPUSurfaceDescriptor to make a @ref WGPUSurface wrapping
 * an OpenHarmony @c OHNativeWindow.
 *
 * Default values can be set using @ref WGPU_SURFACE_SOURCE_OHOS_NATIVE_WINDOW_INIT as initializer.
 */
typedef struct WGPUSurfaceSourceOhosNativeWindow {
    WGPUChainedStruct chain;
    /**
     * A pointer to an OpenHarmony @c OHNativeWindow. Must not be NULL.
     *
     * The `INIT` macro sets this to `NULL`.
     */
    void * window;
} WGPUSurfaceSourceOhosNativeWindow WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUSurfaceSourceOhosNativeWindow.
 */
#define WGPU_SURFACE_SOURCE_OHOS_NATIVE_WINDOW_INIT _wgpu_MAKE_INIT_STRUCT(WGPUSurfaceSourceOhosNativeWindow, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_SurfaceSourceOhosNativeWindow _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.window=*/NULL _wgpu_COMMA \
})

/**
 * Chained in @ref WGPUSurfaceDescriptor to make a @ref WGPUSurface wrapping a WinUI [`SwapChainPanel`](https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.controls.swapchainpanel).
 *
 * Default values can be set using @ref WGPU_SURFACE_SOURCE_SWAP_CHAIN_PANEL_INIT as initializer.
 */
typedef struct WGPUSurfaceSourceSwapChainPanel {
    WGPUChainedStruct chain;
    /**
     * A pointer to the [`ISwapChainPanelNative`](https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/win32/microsoft.ui.xaml.media.dxinterop/nn-microsoft-ui-xaml-media-dxinterop-iswapchainpanelnative)
     * interface of the SwapChainPanel that will be wrapped by the @ref WGPUSurface.
     *
     * The `INIT` macro sets this to `NULL`.
     */
    void * panelNative;
} WGPUSurfaceSourceSwapChainPanel WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUSurfaceSourceSwapChainPanel.
 */
#define WGPU_SURFACE_SOURCE_SWAP_CHAIN_PANEL_INIT _wgpu_MAKE_INIT_STRUCT(WGPUSurfaceSourceSwapChainPanel, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_SurfaceSourceSwapChainPanel _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.panelNative=*/NULL _wgpu_COMMA \
})

/**
 * Wayland display connection data for @ref WGPUNativeDisplayHandle.
 *
 * Default values can be set using @ref WGPU_WAYLAND_DISPLAY_HANDLE_INIT as initializer.
 */
typedef struct WGPUWaylandDisplayHandle {
    /**
     * Pointer to the Wayland display (i.e. @c wl_display*). Must not be NULL.
     *
     * The `INIT` macro sets this to `NULL`.
     */
    void * display;
} WGPUWaylandDisplayHandle WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUWaylandDisplayHandle.
 */
#define WGPU_WAYLAND_DISPLAY_HANDLE_INIT _wgpu_MAKE_INIT_STRUCT(WGPUWaylandDisplayHandle, { \
    /*.display=*/NULL _wgpu_COMMA \
})

/**
 * XCB display connection data for @ref WGPUNativeDisplayHandle.
 *
 * Default values can be set using @ref WGPU_XCB_DISPLAY_HANDLE_INIT as initializer.
 */
typedef struct WGPUXcbDisplayHandle {
    /**
     * Pointer to the XCB connection (i.e. @c xcb_connection_t*). Must not be NULL.
     *
     * The `INIT` macro sets this to `NULL`.
     */
    void * connection;
    /**
     * X11 screen number.
     *
     * The `INIT` macro sets this to `0`.
     */
    int32_t screen;
} WGPUXcbDisplayHandle WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUXcbDisplayHandle.
 */
#define WGPU_XCB_DISPLAY_HANDLE_INIT _wgpu_MAKE_INIT_STRUCT(WGPUXcbDisplayHandle, { \
    /*.connection=*/NULL _wgpu_COMMA \
    /*.screen=*/0 _wgpu_COMMA \
})

/**
 * Xlib display connection data for @ref WGPUNativeDisplayHandle.
 *
 * Default values can be set using @ref WGPU_XLIB_DISPLAY_HANDLE_INIT as initializer.
 */
typedef struct WGPUXlibDisplayHandle {
    /**
     * Pointer to the X11 @c Display (i.e. @c Display*). Must not be NULL.
     *
     * The `INIT` macro sets this to `NULL`.
     */
    void * display;
    /**
     * X11 screen number.
     *
     * The `INIT` macro sets this to `0`.
     */
    int32_t screen;
} WGPUXlibDisplayHandle WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUXlibDisplayHandle.
 */
#define WGPU_XLIB_DISPLAY_HANDLE_INIT _wgpu_MAKE_INIT_STRUCT(WGPUXlibDisplayHandle, { \
    /*.display=*/NULL _wgpu_COMMA \
    /*.screen=*/0 _wgpu_COMMA \
})

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
typedef struct WGPUNativeDisplayHandle {
    WGPUNativeDisplayHandleType type;

    union {
        WGPUXlibDisplayHandle xlib;
        WGPUXcbDisplayHandle xcb;
        WGPUWaylandDisplayHandle wayland;
    } data;
} WGPUNativeDisplayHandle WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Default values can be set using @ref WGPU_HUB_REPORT_INIT as initializer.
 */
typedef struct WGPUHubReport {
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport adapters;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport devices;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport queues;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport pipelineLayouts;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport shaderModules;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport bindGroupLayouts;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport bindGroups;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport commandBuffers;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport renderBundles;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport renderPipelines;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport computePipelines;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport pipelineCaches;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport querySets;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport buffers;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport textures;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport textureViews;
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport samplers;
} WGPUHubReport WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUHubReport.
 */
#define WGPU_HUB_REPORT_INIT _wgpu_MAKE_INIT_STRUCT(WGPUHubReport, { \
    /*.adapters=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.devices=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.queues=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.pipelineLayouts=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.shaderModules=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.bindGroupLayouts=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.bindGroups=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.commandBuffers=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.renderBundles=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.renderPipelines=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.computePipelines=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.pipelineCaches=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.querySets=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.buffers=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.textures=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.textureViews=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.samplers=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_SHADER_SOURCE_G_L_S_L_INIT as initializer.
 */
typedef struct WGPUShaderSourceGLSL {
    WGPUChainedStruct chain;
    /**
     * The shader stage this GLSL source targets.
     *
     * The `INIT` macro sets this to @ref WGPUShaderStage_None.
     */
    WGPUShaderStage stage;
    /**
     * GLSL source code.
     *
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView code;
    /**
     * Array count for `defines`. The `INIT` macro sets this to 0.
     */
    uint32_t defineCount;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE WGPUShaderDefine const * defines;
} WGPUShaderSourceGLSL WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUShaderSourceGLSL.
 */
#define WGPU_SHADER_SOURCE_G_L_S_L_INIT _wgpu_MAKE_INIT_STRUCT(WGPUShaderSourceGLSL, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_ShaderSourceGLSL _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.stage=*/WGPUShaderStage_None _wgpu_COMMA \
    /*.code=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
    /*.defineCount=*/0 _wgpu_COMMA \
    /*.defines=*/NULL _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_GLOBAL_REPORT_INIT as initializer.
 */
typedef struct WGPUGlobalReport {
    /**
     * The `INIT` macro sets this to @ref WGPU_REGISTRY_REPORT_INIT.
     */
    WGPURegistryReport surfaces;
    /**
     * Statistics for all other resource types, grouped by backend hub.
     *
     * The `INIT` macro sets this to @ref WGPU_HUB_REPORT_INIT.
     */
    WGPUHubReport hub;
} WGPUGlobalReport WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUGlobalReport.
 */
#define WGPU_GLOBAL_REPORT_INIT _wgpu_MAKE_INIT_STRUCT(WGPUGlobalReport, { \
    /*.surfaces=*/WGPU_REGISTRY_REPORT_INIT _wgpu_COMMA \
    /*.hub=*/WGPU_HUB_REPORT_INIT _wgpu_COMMA \
})

/**
 * Default values can be set using @ref WGPU_INSTANCE_EXTRAS_INIT as initializer.
 */
typedef struct WGPUInstanceExtras {
    WGPUChainedStruct chain;
    /**
     * Which backends to enable.
     * Zero (@ref WGPUInstanceBackend_All) enables all backends.
     *
     * The `INIT` macro sets this to @ref WGPUInstanceBackend_None.
     */
    WGPUInstanceBackend backends;
    /**
     * Flags controlling debug/validation behavior.
     * See @ref WGPUInstanceFlag for available flags.
     *
     * The `INIT` macro sets this to @ref WGPUInstanceFlag_None.
     */
    WGPUInstanceFlag flags;
    /**
     * Which DX12 shader compiler to use.
     * See @ref WGPUDx12Compiler. Ignored on non-DX12 backends.
     *
     * The `INIT` macro sets this to @ref WGPUDx12Compiler_Undefined.
     */
    WGPUDx12Compiler dx12ShaderCompiler;
    /**
     * Which OpenGL ES 3 minor version to request.
     * See @ref WGPUGles3MinorVersion. Ignored on non-GL backends.
     *
     * The `INIT` macro sets this to (@ref WGPUGles3MinorVersion)0.
     */
    WGPUGles3MinorVersion gles3MinorVersion;
    /**
     * Controls OpenGL fence synchronization behavior.
     * See @ref WGPUGLFenceBehaviour. Ignored on non-GL backends.
     *
     * The `INIT` macro sets this to (@ref WGPUGLFenceBehaviour)0.
     */
    WGPUGLFenceBehaviour glFenceBehaviour;
    /**
     * File system path to @c dxcompiler.dll for dynamic DXC loading.
     * Only used when @c dx12ShaderCompiler is @ref WGPUDx12Compiler_Dxc.
     * An empty/undefined string view means the DLL will be searched for
     * on the system PATH.
     *
     * This is a \ref NonNullInputString.
     *
     * The `INIT` macro sets this to @ref WGPU_STRING_VIEW_INIT.
     */
    WGPUStringView dxcPath;
    /**
     * Maximum HLSL shader model version that DXC should target.
     * See @ref WGPUDxcMaxShaderModel. Only used with the DXC compiler.
     *
     * The `INIT` macro sets this to (@ref WGPUDxcMaxShaderModel)0.
     */
    WGPUDxcMaxShaderModel dxcMaxShaderModel;
    /**
     * Which DX12 presentation system (swapchain kind) to use.
     * See @ref WGPUDx12SwapchainKind. Ignored on non-DX12 backends.
     *
     * The `INIT` macro sets this to @ref WGPUDx12SwapchainKind_Undefined.
     */
    WGPUDx12SwapchainKind dx12PresentationSystem;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE uint8_t const * budgetForDeviceCreation;
    /**
     * The `INIT` macro sets this to `NULL`.
     */
    WGPU_NULLABLE uint8_t const * budgetForDeviceLoss;
    /**
     * Platform display connection to associate with this instance.
     * Zero-initialized yields @ref WGPUNativeDisplayHandleType_None (no handle).
     *
     * The `INIT` macro sets this to @ref WGPU_NATIVE_DISPLAY_HANDLE_INIT.
     */
    WGPUNativeDisplayHandle displayHandle;
} WGPUInstanceExtras WGPU_STRUCTURE_ATTRIBUTE;

/**
 * Initializer for @ref WGPUInstanceExtras.
 */
#define WGPU_INSTANCE_EXTRAS_INIT _wgpu_MAKE_INIT_STRUCT(WGPUInstanceExtras, { \
    /*.chain=*/_wgpu_MAKE_INIT_STRUCT(WGPUChainedStruct, { \
        /*.next=*/NULL _wgpu_COMMA \
        /*.sType=*/WGPUSType_InstanceExtras _wgpu_COMMA \
    }) _wgpu_COMMA \
    /*.backends=*/WGPUInstanceBackend_None _wgpu_COMMA \
    /*.flags=*/WGPUInstanceFlag_None _wgpu_COMMA \
    /*.dx12ShaderCompiler=*/WGPUDx12Compiler_Undefined _wgpu_COMMA \
    /*.gles3MinorVersion=*/_wgpu_ENUM_ZERO_INIT(WGPUGles3MinorVersion) _wgpu_COMMA \
    /*.glFenceBehaviour=*/_wgpu_ENUM_ZERO_INIT(WGPUGLFenceBehaviour) _wgpu_COMMA \
    /*.dxcPath=*/WGPU_STRING_VIEW_INIT _wgpu_COMMA \
    /*.dxcMaxShaderModel=*/_wgpu_ENUM_ZERO_INIT(WGPUDxcMaxShaderModel) _wgpu_COMMA \
    /*.dx12PresentationSystem=*/WGPUDx12SwapchainKind_Undefined _wgpu_COMMA \
    /*.budgetForDeviceCreation=*/NULL _wgpu_COMMA \
    /*.budgetForDeviceLoss=*/NULL _wgpu_COMMA \
    /*.displayHandle=*/WGPU_NATIVE_DISPLAY_HANDLE_INIT _wgpu_COMMA \
})

/** @} */

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(WGPU_SKIP_PROCS)
// Global procs
/**
 * Proc pointer type for @ref wgpuGenerateReport:
 * > @copydoc wgpuGenerateReport
 */
typedef void (*WGPUProcGenerateReport)(WGPUInstance instance, WGPUGlobalReport * report) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuSetLogCallback:
 * > @copydoc wgpuSetLogCallback
 */
typedef void (*WGPUProcSetLogCallback)(WGPULogCallback callback, WGPU_NULLABLE void * userdata) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuSetLogLevel:
 * > @copydoc wgpuSetLogLevel
 */
typedef void (*WGPUProcSetLogLevel)(WGPULogLevel level) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuGetVersion:
 * > @copydoc wgpuGetVersion
 */
typedef uint32_t (*WGPUProcGetVersion)() WGPU_FUNCTION_ATTRIBUTE;

// Procs of CommandEncoder
/**
 * Proc pointer type for @ref wgpuCommandEncoderClearTexture:
 * > @copydoc wgpuCommandEncoderClearTexture
 */
typedef void (*WGPUProcCommandEncoderClearTexture)(WGPUCommandEncoder commandEncoder, WGPUTexture texture, WGPUImageSubresourceRange const * range) WGPU_FUNCTION_ATTRIBUTE;

// Procs of ComputePassEncoder
/**
 * Proc pointer type for @ref wgpuComputePassEncoderBeginPipelineStatisticsQuery:
 * > @copydoc wgpuComputePassEncoderBeginPipelineStatisticsQuery
 */
typedef void (*WGPUProcComputePassEncoderBeginPipelineStatisticsQuery)(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuComputePassEncoderEndPipelineStatisticsQuery:
 * > @copydoc wgpuComputePassEncoderEndPipelineStatisticsQuery
 */
typedef void (*WGPUProcComputePassEncoderEndPipelineStatisticsQuery)(WGPUComputePassEncoder computePassEncoder) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuComputePassEncoderWriteTimestamp:
 * > @copydoc wgpuComputePassEncoderWriteTimestamp
 */
typedef void (*WGPUProcComputePassEncoderWriteTimestamp)(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Device
/**
 * Proc pointer type for @ref wgpuDeviceCreateShaderModuleSpirV:
 * > @copydoc wgpuDeviceCreateShaderModuleSpirV
 */
typedef WGPUShaderModule (*WGPUProcDeviceCreateShaderModuleSpirV)(WGPUDevice device, WGPUShaderModuleDescriptorSpirV const * descriptor) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuDeviceCreateShaderModuleTrusted:
 * > @copydoc wgpuDeviceCreateShaderModuleTrusted
 */
typedef WGPUShaderModule (*WGPUProcDeviceCreateShaderModuleTrusted)(WGPUDevice device, WGPUShaderModuleDescriptor const * descriptor, WGPUShaderRuntimeChecks runtimeChecks) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuDeviceGetNativeMetalDevice:
 * > @copydoc wgpuDeviceGetNativeMetalDevice
 */
typedef void * (*WGPUProcDeviceGetNativeMetalDevice)(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuDevicePoll:
 * > @copydoc wgpuDevicePoll
 */
typedef WGPUBool (*WGPUProcDevicePoll)(WGPUDevice device, WGPUBool wait, WGPU_NULLABLE WGPUSubmissionIndex const * submissionIndex) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuDeviceStartGraphicsDebuggerCapture:
 * > @copydoc wgpuDeviceStartGraphicsDebuggerCapture
 */
typedef WGPUBool (*WGPUProcDeviceStartGraphicsDebuggerCapture)(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuDeviceStopGraphicsDebuggerCapture:
 * > @copydoc wgpuDeviceStopGraphicsDebuggerCapture
 */
typedef void (*WGPUProcDeviceStopGraphicsDebuggerCapture)(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Instance
/**
 * Proc pointer type for @ref wgpuInstanceEnumerateAdapters:
 * > @copydoc wgpuInstanceEnumerateAdapters
 */
typedef size_t (*WGPUProcInstanceEnumerateAdapters)(WGPUInstance instance, WGPU_NULLABLE WGPUInstanceEnumerateAdapterOptions const * options, WGPUAdapter * adapters) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Queue
/**
 * Proc pointer type for @ref wgpuQueueGetNativeMetalCommandQueue:
 * > @copydoc wgpuQueueGetNativeMetalCommandQueue
 */
typedef void * (*WGPUProcQueueGetNativeMetalCommandQueue)(WGPUQueue queue) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuQueueGetTimestampPeriod:
 * > @copydoc wgpuQueueGetTimestampPeriod
 */
typedef float (*WGPUProcQueueGetTimestampPeriod)(WGPUQueue queue) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuQueueSubmitForIndex:
 * > @copydoc wgpuQueueSubmitForIndex
 */
typedef WGPUSubmissionIndex (*WGPUProcQueueSubmitForIndex)(WGPUQueue queue, size_t commandCount, WGPUCommandBuffer const * commands) WGPU_FUNCTION_ATTRIBUTE;

// Procs of RenderPassEncoder
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderBeginPipelineStatisticsQuery:
 * > @copydoc wgpuRenderPassEncoderBeginPipelineStatisticsQuery
 */
typedef void (*WGPUProcRenderPassEncoderBeginPipelineStatisticsQuery)(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderEndPipelineStatisticsQuery:
 * > @copydoc wgpuRenderPassEncoderEndPipelineStatisticsQuery
 */
typedef void (*WGPUProcRenderPassEncoderEndPipelineStatisticsQuery)(WGPURenderPassEncoder renderPassEncoder) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderMultiDrawIndexedIndirect:
 * > @copydoc wgpuRenderPassEncoderMultiDrawIndexedIndirect
 */
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndexedIndirect)(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, uint32_t count) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderMultiDrawIndexedIndirectCount:
 * > @copydoc wgpuRenderPassEncoderMultiDrawIndexedIndirectCount
 */
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndexedIndirectCount)(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer countBuffer, uint64_t countBufferOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderMultiDrawIndirect:
 * > @copydoc wgpuRenderPassEncoderMultiDrawIndirect
 */
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndirect)(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, uint32_t count) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderMultiDrawIndirectCount:
 * > @copydoc wgpuRenderPassEncoderMultiDrawIndirectCount
 */
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndirectCount)(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer countBuffer, uint64_t countBufferOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Proc pointer type for @ref wgpuRenderPassEncoderWriteTimestamp:
 * > @copydoc wgpuRenderPassEncoderWriteTimestamp
 */
typedef void (*WGPUProcRenderPassEncoderWriteTimestamp)(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Texture
/**
 * Proc pointer type for @ref wgpuTextureGetNativeMetalTexture:
 * > @copydoc wgpuTextureGetNativeMetalTexture
 */
typedef void * (*WGPUProcTextureGetNativeMetalTexture)(WGPUTexture texture) WGPU_FUNCTION_ATTRIBUTE;

#endif  // !defined(WGPU_SKIP_PROCS)

#if !defined(WGPU_SKIP_DECLARATIONS)
/**
 * \defgroup GlobalFunctions Global Functions
 * \brief Functions that are not specific to an object.
 *
 * @{
 */
WGPU_EXPORT void wgpuGenerateReport(WGPUInstance instance, WGPUGlobalReport * report) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuSetLogCallback(WGPULogCallback callback, WGPU_NULLABLE void * userdata) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuSetLogLevel(WGPULogLevel level) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT uint32_t wgpuGetVersion() WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup Methods Methods
 * \brief Functions that are relative to a specific object.
 *
 * @{
 */

/**
 * \defgroup WGPUCommandEncoderMethods WGPUCommandEncoder methods
 * \brief Functions whose first argument has type WGPUCommandEncoder.
 *
 * @{
 */
WGPU_EXPORT void wgpuCommandEncoderClearTexture(WGPUCommandEncoder commandEncoder, WGPUTexture texture, WGPUImageSubresourceRange const * range) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPUComputePassEncoderMethods WGPUComputePassEncoder methods
 * \brief Functions whose first argument has type WGPUComputePassEncoder.
 *
 * @{
 */
WGPU_EXPORT void wgpuComputePassEncoderBeginPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuComputePassEncoderEndPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuComputePassEncoderWriteTimestamp(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPUDeviceMethods WGPUDevice methods
 * \brief Functions whose first argument has type WGPUDevice.
 *
 * @{
 */
/**
 * @returns
 * This value is @ref ReturnedWithOwnership.
 */
WGPU_EXPORT WGPUShaderModule wgpuDeviceCreateShaderModuleSpirV(WGPUDevice device, WGPUShaderModuleDescriptorSpirV const * descriptor) WGPU_FUNCTION_ATTRIBUTE;
/**
 * @returns
 * This value is @ref ReturnedWithOwnership.
 */
WGPU_EXPORT WGPUShaderModule wgpuDeviceCreateShaderModuleTrusted(WGPUDevice device, WGPUShaderModuleDescriptor const * descriptor, WGPUShaderRuntimeChecks runtimeChecks) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Returns the backend-native `id<MTLDevice>` as an opaque pointer.
 *
 * The returned pointer is borrowed and remains valid only while `device` is alive.
 * Ownership is retained by wgpu-native; callers must not release or destroy it.
 * Returns NULL when the active backend is not Metal or when the handle is unavailable.
 */
WGPU_EXPORT void * wgpuDeviceGetNativeMetalDevice(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Returns true if the queue is empty, or false if there are more queue submissions still in flight.
 */
WGPU_EXPORT WGPUBool wgpuDevicePoll(WGPUDevice device, WGPUBool wait, WGPU_NULLABLE WGPUSubmissionIndex const * submissionIndex) WGPU_FUNCTION_ATTRIBUTE;
/**
 * Returns true if the capture was successfully started, or false if it failed to start or is not supported on the current platform.
 */
WGPU_EXPORT WGPUBool wgpuDeviceStartGraphicsDebuggerCapture(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuDeviceStopGraphicsDebuggerCapture(WGPUDevice device) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPUInstanceMethods WGPUInstance methods
 * \brief Functions whose first argument has type WGPUInstance.
 *
 * @{
 */
WGPU_EXPORT size_t wgpuInstanceEnumerateAdapters(WGPUInstance instance, WGPU_NULLABLE WGPUInstanceEnumerateAdapterOptions const * options, WGPUAdapter * adapters) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPUQueueMethods WGPUQueue methods
 * \brief Functions whose first argument has type WGPUQueue.
 *
 * @{
 */
/**
 * Returns the backend-native `id<MTLCommandQueue>` as an opaque pointer.
 *
 * The returned pointer is borrowed and remains valid only while `queue` is alive.
 * Ownership is retained by wgpu-native; callers must not release or destroy it.
 * Returns NULL when the active backend is not Metal or when the handle is unavailable.
 */
WGPU_EXPORT void * wgpuQueueGetNativeMetalCommandQueue(WGPUQueue queue) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT float wgpuQueueGetTimestampPeriod(WGPUQueue queue) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, size_t commandCount, WGPUCommandBuffer const * commands) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPURenderPassEncoderMethods WGPURenderPassEncoder methods
 * \brief Functions whose first argument has type WGPURenderPassEncoder.
 *
 * @{
 */
WGPU_EXPORT void wgpuRenderPassEncoderBeginPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderEndPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndexedIndirect(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, uint32_t count) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer countBuffer, uint64_t countBufferOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndirect(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, uint32_t count) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndirectCount(WGPURenderPassEncoder renderPassEncoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer countBuffer, uint64_t countBufferOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderWriteTimestamp(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/**
 * \defgroup WGPUTextureMethods WGPUTexture methods
 * \brief Functions whose first argument has type WGPUTexture.
 *
 * @{
 */
/**
 * Returns the backend-native `id<MTLTexture>` as an opaque pointer.
 *
 * The returned pointer is borrowed and remains valid only while `texture` is alive.
 * Ownership is retained by wgpu-native; callers must not release or destroy it.
 * Returns NULL when the active backend is not Metal or when the handle is unavailable.
 */
WGPU_EXPORT void * wgpuTextureGetNativeMetalTexture(WGPUTexture texture) WGPU_FUNCTION_ATTRIBUTE;

/** @} */

/** @} */

#endif  // !defined(WGPU_SKIP_DECLARATIONS)

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WGPU_H_
