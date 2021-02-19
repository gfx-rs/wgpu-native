/* Generated with cbindgen:0.17.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `cbindgen --config cbindgen.toml --crate wgpu-native --output ffi/wgpu.h`
 */

typedef unsigned long long WGPUOption_AdapterId;
typedef unsigned long long WGPUOption_BufferId;
typedef unsigned long long WGPUOption_SamplerId;
typedef unsigned long long WGPUOption_SurfaceId;
typedef unsigned long long WGPUOption_TextureViewId;
typedef unsigned long long WGPUOption_BufferSize;
typedef unsigned long long WGPUOption_PipelineLayoutId;

typedef struct WGPUChainedStruct WGPUChainedStruct;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define WGPUMAX_BIND_GROUPS 8

#define WGPUMAX_COLOR_TARGETS 4

#define WGPUMAX_MIP_LEVELS 16

#define WGPUMAX_VERTEX_BUFFERS 16

#define WGPUMAX_ANISOTROPY 16

#define WGPUSHADER_STAGE_COUNT 3

#define WGPUDESIRED_NUM_FRAMES 3

/**
 * Buffer-Texture copies must have [`bytes_per_row`] aligned to this number.
 *
 * This doesn't apply to [`Queue::write_texture`].
 *
 * [`bytes_per_row`]: TextureDataLayout::bytes_per_row
 */
#define WGPUCOPY_BYTES_PER_ROW_ALIGNMENT 256

/**
 * Alignment all push constants need
 */
#define WGPUPUSH_CONSTANT_ALIGNMENT 4

/**
 * Maximum queries in a query set
 */
#define WGPUQUERY_SET_MAX_QUERIES 8192

/**
 * Size of a single piece of query data.
 */
#define WGPUQUERY_SIZE 8

/**
 * How edges should be handled in texture addressing.
 */
typedef enum WGPUAddressMode {
  /**
   * Clamp the value to the edge of the texture
   *
   * -0.25 -> 0.0
   * 1.25  -> 1.0
   */
  WGPUAddressMode_ClampToEdge = 0,
  /**
   * Repeat the texture in a tiling fashion
   *
   * -0.25 -> 0.75
   * 1.25 -> 0.25
   */
  WGPUAddressMode_Repeat = 1,
  /**
   * Repeat the texture, mirroring it every repeat
   *
   * -0.25 -> 0.25
   * 1.25 -> 0.75
   */
  WGPUAddressMode_MirrorRepeat = 2,
  /**
   * Clamp the value to the border of the texture
   * Requires feature [`Features::ADDRESS_MODE_CLAMP_TO_BORDER`]
   *
   * -0.25 -> border
   * 1.25 -> border
   */
  WGPUAddressMode_ClampToBorder = 3,
} WGPUAddressMode;

/**
 * Backends supported by wgpu.
 */
enum WGPUBackend {
  /**
   * Dummy backend, used for testing.
   */
  WGPUBackend_Empty = 0,
  /**
   * Vulkan API
   */
  WGPUBackend_Vulkan = 1,
  /**
   * Metal API (Apple platforms)
   */
  WGPUBackend_Metal = 2,
  /**
   * Direct3D-12 (Windows)
   */
  WGPUBackend_Dx12 = 3,
  /**
   * Direct3D-11 (Windows)
   */
  WGPUBackend_Dx11 = 4,
  /**
   * OpenGL ES-3 (Linux, Android)
   */
  WGPUBackend_Gl = 5,
  /**
   * WebGPU in the browser
   */
  WGPUBackend_BrowserWebGpu = 6,
};
typedef uint8_t WGPUBackend;

enum WGPUBindingType {
  WGPUBindingType_UniformBuffer = 0,
  WGPUBindingType_StorageBuffer = 1,
  WGPUBindingType_ReadonlyStorageBuffer = 2,
  WGPUBindingType_Sampler = 3,
  WGPUBindingType_ComparisonSampler = 4,
  WGPUBindingType_SampledTexture = 5,
  WGPUBindingType_ReadonlyStorageTexture = 6,
  WGPUBindingType_WriteonlyStorageTexture = 7,
};
typedef uint32_t WGPUBindingType;

/**
 * Alpha blend factor.
 *
 * Alpha blending is very complicated: see the OpenGL or Vulkan spec for more information.
 */
typedef enum WGPUBlendFactor {
  /**
   * 0.0
   */
  WGPUBlendFactor_Zero = 0,
  /**
   * 1.0
   */
  WGPUBlendFactor_One = 1,
  /**
   * S.color
   */
  WGPUBlendFactor_SrcColor = 2,
  /**
   * 1.0 - S.color
   */
  WGPUBlendFactor_OneMinusSrcColor = 3,
  /**
   * S.alpha
   */
  WGPUBlendFactor_SrcAlpha = 4,
  /**
   * 1.0 - S.alpha
   */
  WGPUBlendFactor_OneMinusSrcAlpha = 5,
  /**
   * D.color
   */
  WGPUBlendFactor_DstColor = 6,
  /**
   * 1.0 - D.color
   */
  WGPUBlendFactor_OneMinusDstColor = 7,
  /**
   * D.alpha
   */
  WGPUBlendFactor_DstAlpha = 8,
  /**
   * 1.0 - D.alpha
   */
  WGPUBlendFactor_OneMinusDstAlpha = 9,
  /**
   * min(S.alpha, 1.0 - D.alpha)
   */
  WGPUBlendFactor_SrcAlphaSaturated = 10,
  /**
   * Constant
   */
  WGPUBlendFactor_BlendColor = 11,
  /**
   * 1.0 - Constant
   */
  WGPUBlendFactor_OneMinusBlendColor = 12,
} WGPUBlendFactor;

/**
 * Alpha blend operation.
 *
 * Alpha blending is very complicated: see the OpenGL or Vulkan spec for more information.
 */
typedef enum WGPUBlendOperation {
  /**
   * Src + Dst
   */
  WGPUBlendOperation_Add = 0,
  /**
   * Src - Dst
   */
  WGPUBlendOperation_Subtract = 1,
  /**
   * Dst - Src
   */
  WGPUBlendOperation_ReverseSubtract = 2,
  /**
   * min(Src, Dst)
   */
  WGPUBlendOperation_Min = 3,
  /**
   * max(Src, Dst)
   */
  WGPUBlendOperation_Max = 4,
} WGPUBlendOperation;

typedef enum WGPUBufferMapAsyncStatus {
  WGPUBufferMapAsyncStatus_Success,
  WGPUBufferMapAsyncStatus_Error,
  WGPUBufferMapAsyncStatus_Unknown,
  WGPUBufferMapAsyncStatus_ContextLost,
} WGPUBufferMapAsyncStatus;

enum WGPUDeviceType {
  /**
   * Other.
   */
  WGPUDeviceType_Other = 0,
  /**
   * Integrated GPU with shared CPU/GPU memory.
   */
  WGPUDeviceType_IntegratedGpu,
  /**
   * Discrete GPU with separate CPU/GPU memory.
   */
  WGPUDeviceType_DiscreteGpu,
  /**
   * Virtual / Hosted.
   */
  WGPUDeviceType_VirtualGpu,
  /**
   * Cpu / Software Rendering.
   */
  WGPUDeviceType_Cpu,
};
typedef uint8_t WGPUDeviceType;

enum WGPUCompareFunction {
  WGPUCompareFunction_Undefined,
  WGPUCompareFunction_Never,
  WGPUCompareFunction_Less,
  WGPUCompareFunction_LessEqual,
  WGPUCompareFunction_Greater,
  WGPUCompareFunction_GreaterEqual,
  WGPUCompareFunction_Equal,
  WGPUCompareFunction_NotEqual,
  WGPUCompareFunction_Always,
};
typedef uint32_t WGPUCompareFunction;

/**
 * Type of faces to be culled.
 */
typedef enum WGPUCullMode {
  /**
   * No faces should be culled
   */
  WGPUCullMode_None = 0,
  /**
   * Front faces should be culled
   */
  WGPUCullMode_Front = 1,
  /**
   * Back faces should be culled
   */
  WGPUCullMode_Back = 2,
} WGPUCullMode;

/**
 * Texel mixing mode when sampling between texels.
 */
typedef enum WGPUFilterMode {
  /**
   * Nearest neighbor sampling.
   *
   * This creates a pixelated effect when used as a mag filter
   */
  WGPUFilterMode_Nearest = 0,
  /**
   * Linear Interpolation
   *
   * This makes textures smooth but blurry when used as a mag filter.
   */
  WGPUFilterMode_Linear = 1,
} WGPUFilterMode;

/**
 * Winding order which classifies the "front" face.
 */
typedef enum WGPUFrontFace {
  /**
   * Triangles with vertices in counter clockwise order are considered the front face.
   *
   * This is the default with right handed coordinate spaces.
   */
  WGPUFrontFace_Ccw = 0,
  /**
   * Triangles with vertices in clockwise order are considered the front face.
   *
   * This is the default with left handed coordinate spaces.
   */
  WGPUFrontFace_Cw = 1,
} WGPUFrontFace;

enum WGPUIndexFormat {
  WGPUIndexFormat_Undefined = 0,
  WGPUIndexFormat_Uint16 = 1,
  WGPUIndexFormat_Uint32 = 2,
};
typedef uint32_t WGPUIndexFormat;

/**
 * Rate that determines when vertex data is advanced.
 */
typedef enum WGPUInputStepMode {
  /**
   * Input data is advanced every vertex. This is the standard value for vertex data.
   */
  WGPUInputStepMode_Vertex = 0,
  /**
   * Input data is advanced every instance.
   */
  WGPUInputStepMode_Instance = 1,
} WGPUInputStepMode;

/**
 * Operation to perform to the output attachment at the start of a renderpass.
 */
typedef enum WGPULoadOp {
  /**
   * Clear the output attachment with the clear color. Clearing is faster than loading.
   */
  WGPULoadOp_Clear = 0,
  /**
   * Do not clear output attachment.
   */
  WGPULoadOp_Load = 1,
} WGPULoadOp;

typedef enum WGPULogLevel {
  WGPULogLevel_Off = 0,
  WGPULogLevel_Error = 1,
  WGPULogLevel_Warn = 2,
  WGPULogLevel_Info = 3,
  WGPULogLevel_Debug = 4,
  WGPULogLevel_Trace = 5,
} WGPULogLevel;

/**
 * Type of drawing mode for polygons
 */
typedef enum WGPUPolygonMode {
  /**
   * Polygons are filled
   */
  WGPUPolygonMode_Fill = 0,
  /**
   * Polygons are drawn as line segments
   */
  WGPUPolygonMode_Line = 1,
  /**
   * Polygons are drawn as points
   */
  WGPUPolygonMode_Point = 2,
} WGPUPolygonMode;

/**
 * Power Preference when choosing a physical adapter.
 */
typedef enum WGPUPowerPreference {
  /**
   * Adapter that uses the least possible power. This is often an integerated GPU.
   */
  WGPUPowerPreference_LowPower = 0,
  /**
   * Adapter that has the highest performance. This is often a discrete GPU.
   */
  WGPUPowerPreference_HighPerformance = 1,
} WGPUPowerPreference;

/**
 * Behavior of the presentation engine based on frame rate.
 */
typedef enum WGPUPresentMode {
  /**
   * The presentation engine does **not** wait for a vertical blanking period and
   * the request is presented immediately. This is a low-latency presentation mode,
   * but visible tearing may be observed. Will fallback to `Fifo` if unavailable on the
   * selected  platform and backend. Not optimal for mobile.
   */
  WGPUPresentMode_Immediate = 0,
  /**
   * The presentation engine waits for the next vertical blanking period to update
   * the current image, but frames may be submitted without delay. This is a low-latency
   * presentation mode and visible tearing will **not** be observed. Will fallback to `Fifo`
   * if unavailable on the selected platform and backend. Not optimal for mobile.
   */
  WGPUPresentMode_Mailbox = 1,
  /**
   * The presentation engine waits for the next vertical blanking period to update
   * the current image. The framerate will be capped at the display refresh rate,
   * corresponding to the `VSync`. Tearing cannot be observed. Optimal for mobile.
   */
  WGPUPresentMode_Fifo = 2,
} WGPUPresentMode;

/**
 * Primitive type the input mesh is composed of.
 */
typedef enum WGPUPrimitiveTopology {
  /**
   * Vertex data is a list of points. Each vertex is a new point.
   */
  WGPUPrimitiveTopology_PointList = 0,
  /**
   * Vertex data is a list of lines. Each pair of vertices composes a new line.
   *
   * Vertices `0 1 2 3` create two lines `0 1` and `2 3`
   */
  WGPUPrimitiveTopology_LineList = 1,
  /**
   * Vertex data is a strip of lines. Each set of two adjacent vertices form a line.
   *
   * Vertices `0 1 2 3` create three lines `0 1`, `1 2`, and `2 3`.
   */
  WGPUPrimitiveTopology_LineStrip = 2,
  /**
   * Vertex data is a list of triangles. Each set of 3 vertices composes a new triangle.
   *
   * Vertices `0 1 2 3 4 5` create two triangles `0 1 2` and `3 4 5`
   */
  WGPUPrimitiveTopology_TriangleList = 3,
  /**
   * Vertex data is a triangle strip. Each set of three adjacent vertices form a triangle.
   *
   * Vertices `0 1 2 3 4 5` creates four triangles `0 1 2`, `2 1 3`, `3 2 4`, and `4 3 5`
   */
  WGPUPrimitiveTopology_TriangleStrip = 4,
} WGPUPrimitiveTopology;

enum WGPUSType {
  WGPUSType_Invalid = 0,
  WGPUSType_SurfaceDescriptorFromMetalLayer = 1,
  WGPUSType_SurfaceDescriptorFromWindowsHWND = 2,
  WGPUSType_SurfaceDescriptorFromXlib = 3,
  WGPUSType_SurfaceDescriptorFromHTMLCanvasId = 4,
  WGPUSType_ShaderModuleSPIRVDescriptor = 5,
  WGPUSType_ShaderModuleWGSLDescriptor = 6,
  /**
   * Placeholder value until real value can be determined
   */
  WGPUSType_AnisotropicFiltering = 268435456,
  WGPUSType_Force32 = 2147483647,
};
typedef uint32_t WGPUSType;

/**
 * Color variation to use when sampler addressing mode is [`AddressMode::ClampToBorder`]
 */
typedef enum WGPUSamplerBorderColor {
  /**
   * [0, 0, 0, 0]
   */
  WGPUSamplerBorderColor_TransparentBlack,
  /**
   * [0, 0, 0, 1]
   */
  WGPUSamplerBorderColor_OpaqueBlack,
  /**
   * [1, 1, 1, 1]
   */
  WGPUSamplerBorderColor_OpaqueWhite,
} WGPUSamplerBorderColor;

/**
 * Operation to perform on the stencil value.
 */
typedef enum WGPUStencilOperation {
  /**
   * Keep stencil value unchanged.
   */
  WGPUStencilOperation_Keep = 0,
  /**
   * Set stencil value to zero.
   */
  WGPUStencilOperation_Zero = 1,
  /**
   * Replace stencil value with value provided in most recent call to [`RenderPass::set_stencil_reference`].
   */
  WGPUStencilOperation_Replace = 2,
  /**
   * Bitwise inverts stencil value.
   */
  WGPUStencilOperation_Invert = 3,
  /**
   * Increments stencil value by one, clamping on overflow.
   */
  WGPUStencilOperation_IncrementClamp = 4,
  /**
   * Decrements stencil value by one, clamping on underflow.
   */
  WGPUStencilOperation_DecrementClamp = 5,
  /**
   * Increments stencil value by one, wrapping on overflow.
   */
  WGPUStencilOperation_IncrementWrap = 6,
  /**
   * Decrements stencil value by one, wrapping on underflow.
   */
  WGPUStencilOperation_DecrementWrap = 7,
} WGPUStencilOperation;

/**
 * Operation to perform to the output attachment at the end of a renderpass.
 */
typedef enum WGPUStoreOp {
  /**
   * Clear the render target. If you don't care about the contents of the target, this can be faster.
   */
  WGPUStoreOp_Clear = 0,
  /**
   * Store the result of the renderpass.
   */
  WGPUStoreOp_Store = 1,
} WGPUStoreOp;

/**
 * Status of the recieved swapchain image.
 */
typedef enum WGPUSwapChainStatus {
  /**
   * No issues.
   */
  WGPUSwapChainStatus_Good,
  /**
   * The swap chain is operational, but it does no longer perfectly
   * match the surface. A re-configuration is needed.
   */
  WGPUSwapChainStatus_Suboptimal,
  /**
   * Unable to get the next frame, timed out.
   */
  WGPUSwapChainStatus_Timeout,
  /**
   * The surface under the swap chain has changed.
   */
  WGPUSwapChainStatus_Outdated,
  /**
   * The surface under the swap chain is lost.
   */
  WGPUSwapChainStatus_Lost,
} WGPUSwapChainStatus;

/**
 * Kind of data the texture holds.
 */
typedef enum WGPUTextureAspect {
  /**
   * Depth, Stencil, and Color.
   */
  WGPUTextureAspect_All,
  /**
   * Stencil.
   */
  WGPUTextureAspect_StencilOnly,
  /**
   * Depth.
   */
  WGPUTextureAspect_DepthOnly,
} WGPUTextureAspect;

enum WGPUTextureComponentType {
  WGPUTextureComponentType_Float = 0,
  WGPUTextureComponentType_Sint = 1,
  WGPUTextureComponentType_Uint = 2,
  WGPUTextureComponentType_DepthComparison = 3,
};
typedef uint32_t WGPUTextureComponentType;

/**
 * Dimensionality of a texture.
 */
typedef enum WGPUTextureDimension {
  /**
   * 1D texture
   */
  WGPUTextureDimension_D1,
  /**
   * 2D texture
   */
  WGPUTextureDimension_D2,
  /**
   * 3D texture
   */
  WGPUTextureDimension_D3,
} WGPUTextureDimension;

/**
 * Underlying texture data format.
 *
 * If there is a conversion in the format (such as srgb -> linear), The conversion listed is for
 * loading from texture in a shader. When writing to the texture, the opposite conversion takes place.
 */
typedef enum WGPUTextureFormat {
  /**
   * Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
   */
  WGPUTextureFormat_R8Unorm = 0,
  /**
   * Red channel only. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
   */
  WGPUTextureFormat_R8Snorm = 1,
  /**
   * Red channel only. 8 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_R8Uint = 2,
  /**
   * Red channel only. 8 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_R8Sint = 3,
  /**
   * Red channel only. 16 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_R16Uint = 4,
  /**
   * Red channel only. 16 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_R16Sint = 5,
  /**
   * Red channel only. 16 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_R16Float = 6,
  /**
   * Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
   */
  WGPUTextureFormat_Rg8Unorm = 7,
  /**
   * Red and green channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
   */
  WGPUTextureFormat_Rg8Snorm = 8,
  /**
   * Red and green channels. 8 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rg8Uint = 9,
  /**
   * Red and green channel s. 8 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rg8Sint = 10,
  /**
   * Red channel only. 32 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_R32Uint = 11,
  /**
   * Red channel only. 32 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_R32Sint = 12,
  /**
   * Red channel only. 32 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_R32Float = 13,
  /**
   * Red and green channels. 16 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rg16Uint = 14,
  /**
   * Red and green channels. 16 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rg16Sint = 15,
  /**
   * Red and green channels. 16 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_Rg16Float = 16,
  /**
   * Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
   */
  WGPUTextureFormat_Rgba8Unorm = 17,
  /**
   * Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   */
  WGPUTextureFormat_Rgba8UnormSrgb = 18,
  /**
   * Red, green, blue, and alpha channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
   */
  WGPUTextureFormat_Rgba8Snorm = 19,
  /**
   * Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rgba8Uint = 20,
  /**
   * Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rgba8Sint = 21,
  /**
   * Blue, green, red, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
   */
  WGPUTextureFormat_Bgra8Unorm = 22,
  /**
   * Blue, green, red, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   */
  WGPUTextureFormat_Bgra8UnormSrgb = 23,
  /**
   * Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. [0, 1023] ([0, 3] for alpha) converted to/from float [0, 1] in shader.
   */
  WGPUTextureFormat_Rgb10a2Unorm = 24,
  /**
   * Red, green, and blue channels. 11 bit float with no sign bit for RG channels. 10 bit float with no sign bit for blue channel. Float in shader.
   */
  WGPUTextureFormat_Rg11b10Float = 25,
  /**
   * Red and green channels. 32 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rg32Uint = 26,
  /**
   * Red and green channels. 32 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rg32Sint = 27,
  /**
   * Red and green channels. 32 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_Rg32Float = 28,
  /**
   * Red, green, blue, and alpha channels. 16 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rgba16Uint = 29,
  /**
   * Red, green, blue, and alpha channels. 16 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rgba16Sint = 30,
  /**
   * Red, green, blue, and alpha channels. 16 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_Rgba16Float = 31,
  /**
   * Red, green, blue, and alpha channels. 32 bit integer per channel. Unsigned in shader.
   */
  WGPUTextureFormat_Rgba32Uint = 32,
  /**
   * Red, green, blue, and alpha channels. 32 bit integer per channel. Signed in shader.
   */
  WGPUTextureFormat_Rgba32Sint = 33,
  /**
   * Red, green, blue, and alpha channels. 32 bit float per channel. Float in shader.
   */
  WGPUTextureFormat_Rgba32Float = 34,
  /**
   * Special depth format with 32 bit floating point depth.
   */
  WGPUTextureFormat_Depth32Float = 35,
  /**
   * Special depth format with at least 24 bit integer depth.
   */
  WGPUTextureFormat_Depth24Plus = 36,
  /**
   * Special depth/stencil format with at least 24 bit integer depth and 8 bits integer stencil.
   */
  WGPUTextureFormat_Depth24PlusStencil8 = 37,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
   * [0, 63] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
   *
   * Also known as DXT1.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc1RgbaUnorm = 38,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
   * Srgb-color [0, 63] ([0, 15] for alpha) converted to/from linear-color float [0, 1] in shader.
   *
   * Also known as DXT1.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc1RgbaUnormSrgb = 39,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
   * [0, 63] ([0, 15] for alpha) converted to/from float [0, 1] in shader.
   *
   * Also known as DXT3.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc2RgbaUnorm = 40,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
   * Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
   *
   * Also known as DXT3.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc2RgbaUnormSrgb = 41,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
   * [0, 63] ([0, 255] for alpha) converted to/from float [0, 1] in shader.
   *
   * Also known as DXT5.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc3RgbaUnorm = 42,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
   * Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
   *
   * Also known as DXT5.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc3RgbaUnormSrgb = 43,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * Also known as RGTC1.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc4RUnorm = 44,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
   * [-127, 127] converted to/from float [-1, 1] in shader.
   *
   * Also known as RGTC1.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc4RSnorm = 45,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * Also known as RGTC2.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc5RgUnorm = 46,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
   * [-127, 127] converted to/from float [-1, 1] in shader.
   *
   * Also known as RGTC2.
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc5RgSnorm = 47,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit unsigned float RGB. Float in shader.
   *
   * Also known as BPTC (float).
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc6hRgbUfloat = 48,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit signed float RGB. Float in shader.
   *
   * Also known as BPTC (float).
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc6hRgbSfloat = 49,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * Also known as BPTC (unorm).
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc7RgbaUnorm = 50,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * Also known as BPTC (unorm).
   *
   * [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Bc7RgbaUnormSrgb = 51,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbUnorm = 52,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbUnormSrgb = 53,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
   * [0, 255] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbA1Unorm = 54,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
   * Srgb-color [0, 255] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbA1UnormSrgb = 55,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbA8Unorm = 56,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Etc2RgbA8UnormSrgb = 57,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer R.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_EacRUnorm = 58,
  /**
   * 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer R.
   * [-127, 127] converted to/from float [-1, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_EacRSnorm = 59,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer R + 8 bit integer G.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_EtcRgUnorm = 60,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer R + 8 bit integer G.
   * [-127, 127] converted to/from float [-1, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_EtcRgSnorm = 61,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc4x4RgbaUnorm = 62,
  /**
   * 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc4x4RgbaUnormSrgb = 63,
  /**
   * 5x4 block compressed texture. 16 bytes per block (6.4 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc5x4RgbaUnorm = 64,
  /**
   * 5x4 block compressed texture. 16 bytes per block (6.4 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc5x4RgbaUnormSrgb = 65,
  /**
   * 5x5 block compressed texture. 16 bytes per block (5.12 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc5x5RgbaUnorm = 66,
  /**
   * 5x5 block compressed texture. 16 bytes per block (5.12 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc5x5RgbaUnormSrgb = 67,
  /**
   * 6x5 block compressed texture. 16 bytes per block (4.27 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc6x5RgbaUnorm = 68,
  /**
   * 6x5 block compressed texture. 16 bytes per block (4.27 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc6x5RgbaUnormSrgb = 69,
  /**
   * 6x6 block compressed texture. 16 bytes per block (3.56 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc6x6RgbaUnorm = 70,
  /**
   * 6x6 block compressed texture. 16 bytes per block (3.56 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc6x6RgbaUnormSrgb = 71,
  /**
   * 8x5 block compressed texture. 16 bytes per block (3.2 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x5RgbaUnorm = 72,
  /**
   * 8x5 block compressed texture. 16 bytes per block (3.2 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x5RgbaUnormSrgb = 73,
  /**
   * 8x6 block compressed texture. 16 bytes per block (2.67 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x6RgbaUnorm = 74,
  /**
   * 8x6 block compressed texture. 16 bytes per block (2.67 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x6RgbaUnormSrgb = 75,
  /**
   * 10x5 block compressed texture. 16 bytes per block (2.56 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x5RgbaUnorm = 76,
  /**
   * 10x5 block compressed texture. 16 bytes per block (2.56 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x5RgbaUnormSrgb = 77,
  /**
   * 10x6 block compressed texture. 16 bytes per block (2.13 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x6RgbaUnorm = 78,
  /**
   * 10x6 block compressed texture. 16 bytes per block (2.13 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x6RgbaUnormSrgb = 79,
  /**
   * 8x8 block compressed texture. 16 bytes per block (2 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x8RgbaUnorm = 80,
  /**
   * 8x8 block compressed texture. 16 bytes per block (2 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc8x8RgbaUnormSrgb = 81,
  /**
   * 10x8 block compressed texture. 16 bytes per block (1.6 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x8RgbaUnorm = 82,
  /**
   * 10x8 block compressed texture. 16 bytes per block (1.6 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x8RgbaUnormSrgb = 83,
  /**
   * 10x10 block compressed texture. 16 bytes per block (1.28 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x10RgbaUnorm = 84,
  /**
   * 10x10 block compressed texture. 16 bytes per block (1.28 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc10x10RgbaUnormSrgb = 85,
  /**
   * 12x10 block compressed texture. 16 bytes per block (1.07 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc12x10RgbaUnorm = 86,
  /**
   * 12x10 block compressed texture. 16 bytes per block (1.07 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc12x10RgbaUnormSrgb = 87,
  /**
   * 12x12 block compressed texture. 16 bytes per block (0.89 bit/px). Complex pallet. 8 bit integer RGBA.
   * [0, 255] converted to/from float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc12x12RgbaUnorm = 88,
  /**
   * 12x12 block compressed texture. 16 bytes per block (0.89 bit/px). Complex pallet. 8 bit integer RGBA.
   * Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
   *
   * [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
   */
  WGPUTextureFormat_Astc12x12RgbaUnormSrgb = 89,
} WGPUTextureFormat;

/**
 * Dimensions of a particular texture view.
 */
typedef enum WGPUTextureViewDimension {
  /**
   * A one dimensional texture. `texture1D` in glsl shaders.
   */
  WGPUTextureViewDimension_D1,
  /**
   * A two dimensional texture. `texture2D` in glsl shaders.
   */
  WGPUTextureViewDimension_D2,
  /**
   * A two dimensional array texture. `texture2DArray` in glsl shaders.
   */
  WGPUTextureViewDimension_D2Array,
  /**
   * A cubemap texture. `textureCube` in glsl shaders.
   */
  WGPUTextureViewDimension_Cube,
  /**
   * A cubemap array texture. `textureCubeArray` in glsl shaders.
   */
  WGPUTextureViewDimension_CubeArray,
  /**
   * A three dimensional texture. `texture3D` in glsl shaders.
   */
  WGPUTextureViewDimension_D3,
} WGPUTextureViewDimension;

/**
 * Vertex Format for a Vertex Attribute (input).
 */
typedef enum WGPUVertexFormat {
  /**
   * Two unsigned bytes (u8). `uvec2` in shaders.
   */
  WGPUVertexFormat_Uchar2 = 0,
  /**
   * Four unsigned bytes (u8). `uvec4` in shaders.
   */
  WGPUVertexFormat_Uchar4 = 1,
  /**
   * Two signed bytes (i8). `ivec2` in shaders.
   */
  WGPUVertexFormat_Char2 = 2,
  /**
   * Four signed bytes (i8). `ivec4` in shaders.
   */
  WGPUVertexFormat_Char4 = 3,
  /**
   * Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
   */
  WGPUVertexFormat_Uchar2Norm = 4,
  /**
   * Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
   */
  WGPUVertexFormat_Uchar4Norm = 5,
  /**
   * Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
   */
  WGPUVertexFormat_Char2Norm = 6,
  /**
   * Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
   */
  WGPUVertexFormat_Char4Norm = 7,
  /**
   * Two unsigned shorts (u16). `uvec2` in shaders.
   */
  WGPUVertexFormat_Ushort2 = 8,
  /**
   * Four unsigned shorts (u16). `uvec4` in shaders.
   */
  WGPUVertexFormat_Ushort4 = 9,
  /**
   * Two signed shorts (i16). `ivec2` in shaders.
   */
  WGPUVertexFormat_Short2 = 10,
  /**
   * Four signed shorts (i16). `ivec4` in shaders.
   */
  WGPUVertexFormat_Short4 = 11,
  /**
   * Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
   */
  WGPUVertexFormat_Ushort2Norm = 12,
  /**
   * Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
   */
  WGPUVertexFormat_Ushort4Norm = 13,
  /**
   * Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
   */
  WGPUVertexFormat_Short2Norm = 14,
  /**
   * Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
   */
  WGPUVertexFormat_Short4Norm = 15,
  /**
   * Two half-precision floats (no Rust equiv). `vec2` in shaders.
   */
  WGPUVertexFormat_Half2 = 16,
  /**
   * Four half-precision floats (no Rust equiv). `vec4` in shaders.
   */
  WGPUVertexFormat_Half4 = 17,
  /**
   * One single-precision float (f32). `float` in shaders.
   */
  WGPUVertexFormat_Float = 18,
  /**
   * Two single-precision floats (f32). `vec2` in shaders.
   */
  WGPUVertexFormat_Float2 = 19,
  /**
   * Three single-precision floats (f32). `vec3` in shaders.
   */
  WGPUVertexFormat_Float3 = 20,
  /**
   * Four single-precision floats (f32). `vec4` in shaders.
   */
  WGPUVertexFormat_Float4 = 21,
  /**
   * One unsigned int (u32). `uint` in shaders.
   */
  WGPUVertexFormat_Uint = 22,
  /**
   * Two unsigned ints (u32). `uvec2` in shaders.
   */
  WGPUVertexFormat_Uint2 = 23,
  /**
   * Three unsigned ints (u32). `uvec3` in shaders.
   */
  WGPUVertexFormat_Uint3 = 24,
  /**
   * Four unsigned ints (u32). `uvec4` in shaders.
   */
  WGPUVertexFormat_Uint4 = 25,
  /**
   * One signed int (i32). `int` in shaders.
   */
  WGPUVertexFormat_Int = 26,
  /**
   * Two signed ints (i32). `ivec2` in shaders.
   */
  WGPUVertexFormat_Int2 = 27,
  /**
   * Three signed ints (i32). `ivec3` in shaders.
   */
  WGPUVertexFormat_Int3 = 28,
  /**
   * Four signed ints (i32). `ivec4` in shaders.
   */
  WGPUVertexFormat_Int4 = 29,
  /**
   * One double-precision float (f64). `double` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
   */
  WGPUVertexFormat_Double = 30,
  /**
   * Two double-precision floats (f64). `dvec2` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
   */
  WGPUVertexFormat_Double2 = 31,
  /**
   * Three double-precision floats (f64). `dvec3` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
   */
  WGPUVertexFormat_Double3 = 32,
  /**
   * Four double-precision floats (f64). `dvec4` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
   */
  WGPUVertexFormat_Double4 = 33,
} WGPUVertexFormat;

typedef struct WGPUComputePass WGPUComputePass;

typedef struct WGPURenderBundleEncoder WGPURenderBundleEncoder;

typedef struct WGPURenderPass WGPURenderPass;

typedef uint64_t WGPUId_CommandBuffer_Dummy;

typedef WGPUId_CommandBuffer_Dummy WGPUCommandBufferId;

typedef WGPUCommandBufferId WGPUCommandEncoderId;

typedef const char *WGPULabel;

typedef struct WGPUCommandBufferDescriptor {
  WGPULabel label;
} WGPUCommandBufferDescriptor;

typedef uint64_t WGPUId_Buffer_Dummy;

typedef WGPUId_Buffer_Dummy WGPUBufferId;

/**
 * Integral type used for buffer offsets.
 */
typedef uint64_t WGPUBufferAddress;

/**
 * Layout of a texture in a buffer's memory.
 */
typedef struct WGPUTextureDataLayout {
  /**
   * Offset into the buffer that is the start of the texture. Must be a multiple of texture block size.
   * For non-compressed textures, this is 1.
   */
  WGPUBufferAddress offset;
  /**
   * Bytes per "row" of the image. This represents one row of pixels in the x direction. Compressed
   * textures include multiple rows of pixels in each "row". May be 0 for 1D texture copies.
   *
   * Must be a multiple of 256 for [`CommandEncoder::copy_buffer_to_texture`] and [`CommandEncoder::copy_texture_to_buffer`].
   * [`Queue::write_texture`] does not have this requirement.
   *
   * Must be a multiple of the texture block size. For non-compressed textures, this is 1.
   */
  uint32_t bytes_per_row;
  /**
   * Rows that make up a single "image". Each "image" is one layer in the z direction of a 3D image. May be larger
   * than `copy_size.y`.
   *
   * May be 0 for 2D texture copies.
   */
  uint32_t rows_per_image;
} WGPUTextureDataLayout;

typedef struct WGPUBufferCopyView {
  struct WGPUTextureDataLayout layout;
  WGPUBufferId buffer;
} WGPUBufferCopyView;

typedef uint64_t WGPUId_Texture_Dummy;

typedef WGPUId_Texture_Dummy WGPUTextureId;

/**
 * Origin of a copy to/from a texture.
 */
typedef struct WGPUOrigin3d {
  /**
   *
   */
  uint32_t x;
  /**
   *
   */
  uint32_t y;
  /**
   *
   */
  uint32_t z;
} WGPUOrigin3d;
/**
 * Zero origin.
 */
#define WGPUOrigin3d_ZERO (WGPUOrigin3d){ .x = 0, .y = 0, .z = 0 }

typedef struct WGPUTextureCopyView {
  WGPUTextureId texture;
  uint32_t mip_level;
  struct WGPUOrigin3d origin;
} WGPUTextureCopyView;

/**
 * Extent of a texture related operation.
 */
typedef struct WGPUExtent3d {
  /**
   *
   */
  uint32_t width;
  /**
   *
   */
  uint32_t height;
  /**
   *
   */
  uint32_t depth;
} WGPUExtent3d;

typedef uint64_t WGPUId_TextureView_Dummy;

typedef WGPUId_TextureView_Dummy WGPUTextureViewId;

/**
 * RGBA double precision color.
 *
 * This is not to be used as a generic color type, only for specific wgpu interfaces.
 */
typedef struct WGPUColor {
  /**
   *
   */
  double r;
  /**
   *
   */
  double g;
  /**
   *
   */
  double b;
  /**
   *
   */
  double a;
} WGPUColor;
#define WGPUColor_TRANSPARENT (WGPUColor){ .r = 0.0, .g = 0.0, .b = 0.0, .a = 0.0 }
#define WGPUColor_BLACK (WGPUColor){ .r = 0.0, .g = 0.0, .b = 0.0, .a = 1.0 }
#define WGPUColor_WHITE (WGPUColor){ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1.0 }
#define WGPUColor_RED (WGPUColor){ .r = 1.0, .g = 0.0, .b = 0.0, .a = 1.0 }
#define WGPUColor_GREEN (WGPUColor){ .r = 0.0, .g = 1.0, .b = 0.0, .a = 1.0 }
#define WGPUColor_BLUE (WGPUColor){ .r = 0.0, .g = 0.0, .b = 1.0, .a = 1.0 }

/**
 * Describes an individual channel within a render pass, such as color, depth, or stencil.
 */
typedef struct WGPUPassChannel_Color {
  /**
   * Operation to perform to the output attachment at the start of a renderpass. This must be clear if it
   * is the first renderpass rendering to a swap chain image.
   */
  enum WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  enum WGPUStoreOp store_op;
  /**
   * If load_op is [`LoadOp::Clear`], the attachement will be cleared to this color.
   */
  struct WGPUColor clear_value;
  /**
   * If true, the relevant channel is not changed by a renderpass, and the corresponding attachment
   * can be used inside the pass by other read-only usages.
   */
  bool read_only;
} WGPUPassChannel_Color;

/**
 * Describes a color attachment to a render pass.
 */
typedef struct WGPUColorAttachmentDescriptor {
  /**
   * The view to use as an attachment.
   */
  WGPUTextureViewId attachment;
  /**
   * The view that will receive the resolved output if multisampling is used.
   */
  WGPUOption_TextureViewId resolve_target;
  /**
   * What operations will be performed on this color attachment.
   */
  struct WGPUPassChannel_Color channel;
} WGPUColorAttachmentDescriptor;

/**
 * Describes an individual channel within a render pass, such as color, depth, or stencil.
 */
typedef struct WGPUPassChannel_f32 {
  /**
   * Operation to perform to the output attachment at the start of a renderpass. This must be clear if it
   * is the first renderpass rendering to a swap chain image.
   */
  enum WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  enum WGPUStoreOp store_op;
  /**
   * If load_op is [`LoadOp::Clear`], the attachement will be cleared to this color.
   */
  float clear_value;
  /**
   * If true, the relevant channel is not changed by a renderpass, and the corresponding attachment
   * can be used inside the pass by other read-only usages.
   */
  bool read_only;
} WGPUPassChannel_f32;

/**
 * Describes an individual channel within a render pass, such as color, depth, or stencil.
 */
typedef struct WGPUPassChannel_u32 {
  /**
   * Operation to perform to the output attachment at the start of a renderpass. This must be clear if it
   * is the first renderpass rendering to a swap chain image.
   */
  enum WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  enum WGPUStoreOp store_op;
  /**
   * If load_op is [`LoadOp::Clear`], the attachement will be cleared to this color.
   */
  uint32_t clear_value;
  /**
   * If true, the relevant channel is not changed by a renderpass, and the corresponding attachment
   * can be used inside the pass by other read-only usages.
   */
  bool read_only;
} WGPUPassChannel_u32;

/**
 * Describes a depth/stencil attachment to a render pass.
 */
typedef struct WGPUDepthStencilAttachmentDescriptor {
  /**
   * The view to use as an attachment.
   */
  WGPUTextureViewId attachment;
  /**
   * What operations will be performed on the depth part of the attachment.
   */
  struct WGPUPassChannel_f32 depth;
  /**
   * What operations will be performed on the stencil part of the attachment.
   */
  struct WGPUPassChannel_u32 stencil;
} WGPUDepthStencilAttachmentDescriptor;

typedef struct WGPURenderPassDescriptor {
  const struct WGPUColorAttachmentDescriptor *color_attachments;
  uintptr_t color_attachments_length;
  const struct WGPUDepthStencilAttachmentDescriptor *depth_stencil_attachment;
  WGPULabel label;
} WGPURenderPassDescriptor;

typedef struct WGPUComputePassDescriptor {
  WGPULabel label;
} WGPUComputePassDescriptor;

typedef uint64_t WGPUId_Surface;

typedef WGPUId_Surface WGPUSurfaceId;

/**
 * Options for requesting adapter.
 */
typedef struct WGPURequestAdapterOptions {
  /**
   * Power preference for the adapter.
   */
  enum WGPUPowerPreference power_preference;
  /**
   * Surface that is required to be presentable with the requested adapter. This does not
   * create the surface, only guarantees that the adapter can present to said surface.
   */
  WGPUOption_SurfaceId compatible_surface;
} WGPURequestAdapterOptions;

typedef struct WGPURequestAdapterOptions WGPURequestAdapterOptions;

/**
 * Represents the backends that wgpu will use.
 */
typedef uint32_t WGPUBackendBit;

typedef uint64_t WGPUId_Adapter_Dummy;

typedef WGPUId_Adapter_Dummy WGPUAdapterId;

typedef void (*WGPURequestAdapterCallback)(WGPUAdapterId id, void *userdata);

typedef uint64_t WGPUId_Device_Dummy;

typedef WGPUId_Device_Dummy WGPUDeviceId;

/**
 * Features that are not guaranteed to be supported.
 *
 * These are either part of the webgpu standard, or are extension features supported by
 * wgpu when targeting native.
 *
 * If you want to use a feature, you need to first verify that the adapter supports
 * the feature. If the adapter does not support the feature, requesting a device with it enabled
 * will panic.
 */
typedef uint64_t WGPUFeatures;
/**
 * By default, polygon depth is clipped to 0-1 range. Anything outside of that range
 * is rejected, and respective fragments are not touched.
 *
 * With this extension, we can force clamping of the polygon depth to 0-1. That allows
 * shadow map occluders to be rendered into a tighter depth range.
 *
 * Supported platforms:
 * - desktops
 * - some mobile chips
 *
 * This is a web and native feature.
 */
#define WGPUFeatures_DEPTH_CLAMPING (uint64_t)1
/**
 * Enables BCn family of compressed textures. All BCn textures use 4x4 pixel blocks
 * with 8 or 16 bytes per block.
 *
 * Compressed textures sacrifice some quality in exchange for significantly reduced
 * bandwidth usage.
 *
 * Support for this feature guarantees availability of [`TextureUsage::COPY_SRC | TextureUsage::COPY_DST | TextureUsage::SAMPLED`] for BCn formats.
 * [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
 *
 * Supported Platforms:
 * - desktops
 *
 * This is a web and native feature.
 */
#define WGPUFeatures_TEXTURE_COMPRESSION_BC (uint64_t)2
/**
 * Enables use of Timestamp Queries. These queries tell the current gpu timestamp when
 * all work before the query is finished. Call [`CommandEncoder::write_timestamp`],
 * [`RenderPassEncoder::write_timestamp`], or [`ComputePassEncoder::write_timestamp`] to
 * write out a timestamp.
 *
 * They must be resolved using [`CommandEncoder::resolve_query_sets`] into a buffer,
 * then the result must be multiplied by the timestamp period [`Device::get_timestamp_period`]
 * to get the timestamp in nanoseconds. Multiple timestamps can then be diffed to get the
 * time for operations between them to finish.
 *
 * Due to gfx-hal limitations, this is only supported on vulkan for now.
 *
 * Supported Platforms:
 * - Vulkan (works)
 * - DX12 (future)
 *
 * This is a web and native feature.
 */
#define WGPUFeatures_TIMESTAMP_QUERY (uint64_t)4
/**
 * Enables use of Pipeline Statistics Queries. These queries tell the count of various operations
 * performed between the start and stop call. Call [`RenderPassEncoder::begin_pipeline_statistics_query`] to start
 * a query, then call [`RenderPassEncoder::end_pipeline_statistics_query`] to stop one.
 *
 * They must be resolved using [`CommandEncoder::resolve_query_sets`] into a buffer.
 * The rules on how these resolve into buffers are detailed in the documentation for [`PipelineStatisticsTypes`].
 *
 * Due to gfx-hal limitations, this is only supported on vulkan for now.
 *
 * Supported Platforms:
 * - Vulkan (works)
 * - DX12 (future)
 *
 * This is a web and native feature.
 */
#define WGPUFeatures_PIPELINE_STATISTICS_QUERY (uint64_t)8
/**
 * Webgpu only allows the MAP_READ and MAP_WRITE buffer usage to be matched with
 * COPY_DST and COPY_SRC respectively. This removes this requirement.
 *
 * This is only beneficial on systems that share memory between CPU and GPU. If enabled
 * on a system that doesn't, this can severely hinder performance. Only use if you understand
 * the consequences.
 *
 * Supported platforms:
 * - All
 *
 * This is a native only feature.
 */
#define WGPUFeatures_MAPPABLE_PRIMARY_BUFFERS (uint64_t)65536
/**
 * Allows the user to create uniform arrays of sampled textures in shaders:
 *
 * eg. `uniform texture2D textures[10]`.
 *
 * This capability allows them to exist and to be indexed by compile time constant
 * values.
 *
 * Supported platforms:
 * - DX12
 * - Metal (with MSL 2.0+ on macOS 10.13+)
 * - Vulkan
 *
 * This is a native only feature.
 */
#define WGPUFeatures_SAMPLED_TEXTURE_BINDING_ARRAY (uint64_t)131072
/**
 * Allows shaders to index sampled texture arrays with dynamically uniform values:
 *
 * eg. `texture_array[uniform_value]`
 *
 * This capability means the hardware will also support SAMPLED_TEXTURE_BINDING_ARRAY.
 *
 * Supported platforms:
 * - DX12
 * - Metal (with MSL 2.0+ on macOS 10.13+)
 * - Vulkan's shaderSampledImageArrayDynamicIndexing feature
 *
 * This is a native only feature.
 */
#define WGPUFeatures_SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING (uint64_t)262144
/**
 * Allows shaders to index sampled texture arrays with dynamically non-uniform values:
 *
 * eg. `texture_array[vertex_data]`
 *
 * In order to use this capability, the corresponding GLSL extension must be enabled like so:
 *
 * `#extension GL_EXT_nonuniform_qualifier : require`
 *
 * and then used either as `nonuniformEXT` qualifier in variable declaration:
 *
 * eg. `layout(location = 0) nonuniformEXT flat in int vertex_data;`
 *
 * or as `nonuniformEXT` constructor:
 *
 * eg. `texture_array[nonuniformEXT(vertex_data)]`
 *
 * HLSL does not need any extension.
 *
 * This capability means the hardware will also support SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
 * and SAMPLED_TEXTURE_BINDING_ARRAY.
 *
 * Supported platforms:
 * - DX12
 * - Metal (with MSL 2.0+ on macOS 10.13+)
 * - Vulkan 1.2+ (or VK_EXT_descriptor_indexing)'s shaderSampledImageArrayNonUniformIndexing feature)
 *
 * This is a native only feature.
 */
#define WGPUFeatures_SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING (uint64_t)524288
/**
 * Allows the user to create unsized uniform arrays of bindings:
 *
 * eg. `uniform texture2D textures[]`.
 *
 * If this capability is supported, SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING is very likely
 * to also be supported
 *
 * Supported platforms:
 * - DX12
 * - Vulkan 1.2+ (or VK_EXT_descriptor_indexing)'s runtimeDescriptorArray feature
 *
 * This is a native only feature.
 */
#define WGPUFeatures_UNSIZED_BINDING_ARRAY (uint64_t)1048576
/**
 * Allows the user to call [`RenderPass::multi_draw_indirect`] and [`RenderPass::multi_draw_indexed_indirect`].
 *
 * Allows multiple indirect calls to be dispatched from a single buffer.
 *
 * Supported platforms:
 * - DX12
 * - Metal
 * - Vulkan
 *
 * This is a native only feature.
 */
#define WGPUFeatures_MULTI_DRAW_INDIRECT (uint64_t)2097152
/**
 * Allows the user to call [`RenderPass::multi_draw_indirect_count`] and [`RenderPass::multi_draw_indexed_indirect_count`].
 *
 * This allows the use of a buffer containing the actual number of draw calls.
 *
 * Supported platforms:
 * - DX12
 * - Vulkan 1.2+ (or VK_KHR_draw_indirect_count)
 *
 * This is a native only feature.
 */
#define WGPUFeatures_MULTI_DRAW_INDIRECT_COUNT (uint64_t)4194304
/**
 * Allows the use of push constants: small, fast bits of memory that can be updated
 * inside a [`RenderPass`].
 *
 * Allows the user to call [`RenderPass::set_push_constants`], provide a non-empty array
 * to [`PipelineLayoutDescriptor`], and provide a non-zero limit to [`Limits::max_push_constant_size`].
 *
 * A block of push constants can be declared with `layout(push_constant) uniform Name {..}` in shaders.
 *
 * Supported platforms:
 * - DX12
 * - Vulkan
 * - Metal
 * - DX11 (emulated with uniforms)
 * - OpenGL (emulated with uniforms)
 *
 * This is a native only feature.
 */
#define WGPUFeatures_PUSH_CONSTANTS (uint64_t)8388608
/**
 * Allows the use of [`AddressMode::ClampToBorder`].
 *
 * Supported platforms:
 * - DX12
 * - Vulkan
 * - Metal (macOS 10.12+ only)
 * - DX11
 * - OpenGL
 *
 * This is a web and native feature.
 */
#define WGPUFeatures_ADDRESS_MODE_CLAMP_TO_BORDER (uint64_t)16777216
/**
 * Allows the user to set a non-fill polygon mode in [`RasterizationStateDescriptor::polygon_mode`]
 *
 * This allows drawing polygons/triangles as lines (wireframe) or points instead of filled
 *
 * Supported platforms:
 * - DX12
 * - Vulkan
 *
 * This is a native only feature.
 */
#define WGPUFeatures_NON_FILL_POLYGON_MODE (uint64_t)33554432
/**
 * Enables ETC family of compressed textures. All ETC textures use 4x4 pixel blocks.
 * ETC2 RGB and RGBA1 are 8 bytes per block. RTC2 RGBA8 and EAC are 16 bytes per block.
 *
 * Compressed textures sacrifice some quality in exchange for significantly reduced
 * bandwidth usage.
 *
 * Support for this feature guarantees availability of [`TextureUsage::COPY_SRC | TextureUsage::COPY_DST | TextureUsage::SAMPLED`] for ETC2 formats.
 * [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
 *
 * Supported Platforms:
 * - Intel/Vulkan
 * - Mobile (some)
 *
 * This is a native-only feature.
 */
#define WGPUFeatures_TEXTURE_COMPRESSION_ETC2 (uint64_t)67108864
/**
 * Enables ASTC family of compressed textures. ASTC textures use pixel blocks varying from 4x4 to 12x12.
 * Blocks are always 16 bytes.
 *
 * Compressed textures sacrifice some quality in exchange for significantly reduced
 * bandwidth usage.
 *
 * Support for this feature guarantees availability of [`TextureUsage::COPY_SRC | TextureUsage::COPY_DST | TextureUsage::SAMPLED`] for ASTC formats.
 * [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
 *
 * Supported Platforms:
 * - Intel/Vulkan
 * - Mobile (some)
 *
 * This is a native-only feature.
 */
#define WGPUFeatures_TEXTURE_COMPRESSION_ASTC_LDR (uint64_t)134217728
/**
 * Enables device specific texture format features.
 *
 * See `TextureFormatFeatures` for a listing of the features in question.
 *
 * By default only texture format properties as defined by the WebGPU specification are allowed.
 * Enabling this feature flag extends the features of each format to the ones supported by the current device.
 * Note that without this flag, read/write storage access is not allowed at all.
 *
 * This extension does not enable additional formats.
 *
 * This is a native-only feature.
 */
#define WGPUFeatures_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES (uint64_t)268435456
/**
 * Enables 64-bit floating point types in SPIR-V shaders.
 *
 * Note: even when supported by GPU hardware, 64-bit floating point operations are
 * frequently between 16 and 64 _times_ slower than equivelent operations on 32-bit floats.
 *
 * Supported Platforms:
 * - Vulkan
 *
 * This is a native-only feature.
 */
#define WGPUFeatures_SHADER_FLOAT64 (uint64_t)536870912
/**
 * Enables using 64-bit types for vertex attributes.
 *
 * Requires SHADER_FLOAT64.
 *
 * Supported Platforms: N/A
 *
 * This is a native-only feature.
 */
#define WGPUFeatures_VERTEX_ATTRIBUTE_64BIT (uint64_t)1073741824
/**
 * Features which are part of the upstream WebGPU standard.
 */
#define WGPUFeatures_ALL_WEBGPU (uint64_t)65535
/**
 * Features that are only available when targeting native (not web).
 */
#define WGPUFeatures_ALL_NATIVE (uint64_t)18446744073709486080ULL

typedef struct WGPULimits {
  uint32_t max_bind_groups;
} WGPULimits;

typedef struct WGPUDeviceDescriptor {
  WGPULabel label;
  WGPUFeatures features;
  struct WGPULimits limits;
  const char *trace_path;
} WGPUDeviceDescriptor;

/**
 * Different ways that you can use a buffer.
 *
 * The usages determine what kind of memory the buffer is allocated from and what
 * actions the buffer can partake in.
 */
typedef uint32_t WGPUBufferUsage;
/**
 * Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
 * This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
 *
 * If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
 * may have is COPY_DST.
 */
#define WGPUBufferUsage_MAP_READ (uint32_t)1
/**
 * Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
 * This does not include creating a buffer with `mapped_at_creation` set.
 *
 * If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
 * may have is COPY_SRC.
 */
#define WGPUBufferUsage_MAP_WRITE (uint32_t)2
/**
 * Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
 * operation.
 */
#define WGPUBufferUsage_COPY_SRC (uint32_t)4
/**
 * Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
 * or [`Queue::write_buffer`] operation.
 */
#define WGPUBufferUsage_COPY_DST (uint32_t)8
/**
 * Allow a buffer to be the index buffer in a draw operation.
 */
#define WGPUBufferUsage_INDEX (uint32_t)16
/**
 * Allow a buffer to be the vertex buffer in a draw operation.
 */
#define WGPUBufferUsage_VERTEX (uint32_t)32
/**
 * Allow a buffer to be a [`BufferBindingType::Uniform`] inside a bind group.
 */
#define WGPUBufferUsage_UNIFORM (uint32_t)64
/**
 * Allow a buffer to be a [`BufferBindingType::Storage`] inside a bind group.
 */
#define WGPUBufferUsage_STORAGE (uint32_t)128
/**
 * Allow a buffer to be the indirect buffer in an indirect draw call.
 */
#define WGPUBufferUsage_INDIRECT (uint32_t)256

/**
 * Describes a [`Buffer`].
 */
typedef struct WGPUBufferDescriptor {
  /**
   * Debug label of a buffer. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
  /**
   * Size of a buffer.
   */
  WGPUBufferAddress size;
  /**
   * Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
   * will panic.
   */
  WGPUBufferUsage usage;
  /**
   * Allows a buffer to be mapped immediately after they are made. It does not have to be [`BufferUsage::MAP_READ`] or
   * [`BufferUsage::MAP_WRITE`], all buffers are allowed to be mapped at creation.
   */
  bool mapped_at_creation;
} WGPUBufferDescriptor;

/**
 * Different ways that you can use a texture.
 *
 * The usages determine what kind of memory the texture is allocated from and what
 * actions the texture can partake in.
 */
typedef uint32_t WGPUTextureUsage;
/**
 * Allows a texture to be the source in a [`CommandEncoder::copy_texture_to_buffer`] or
 * [`CommandEncoder::copy_texture_to_texture`] operation.
 */
#define WGPUTextureUsage_COPY_SRC (uint32_t)1
/**
 * Allows a texture to be the destination in a  [`CommandEncoder::copy_texture_to_buffer`],
 * [`CommandEncoder::copy_texture_to_texture`], or [`Queue::write_texture`] operation.
 */
#define WGPUTextureUsage_COPY_DST (uint32_t)2
/**
 * Allows a texture to be a [`BindingType::Texture`] in a bind group.
 */
#define WGPUTextureUsage_SAMPLED (uint32_t)4
/**
 * Allows a texture to be a [`BindingType::StorageTexture`] in a bind group.
 */
#define WGPUTextureUsage_STORAGE (uint32_t)8
/**
 * Allows a texture to be an output attachment of a renderpass.
 */
#define WGPUTextureUsage_RENDER_ATTACHMENT (uint32_t)16

/**
 * Describes a [`Texture`].
 */
typedef struct WGPUTextureDescriptor {
  /**
   * Debug label of the texture. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
  /**
   * Size of the texture. For a regular 1D/2D texture, the unused sizes will be 1. For 2DArray textures, Z is the
   * number of 2D textures in that array.
   */
  struct WGPUExtent3d size;
  /**
   * Mip count of texture. For a texture with no extra mips, this must be 1.
   */
  uint32_t mip_level_count;
  /**
   * Sample count of texture. If this is not 1, texture must have [`BindingType::Texture::multisampled`] set to true.
   */
  uint32_t sample_count;
  /**
   * Dimensions of the texture.
   */
  enum WGPUTextureDimension dimension;
  /**
   * Format of the texture.
   */
  enum WGPUTextureFormat format;
  /**
   * Allowed usages of the texture. If used in other ways, the operation will panic.
   */
  WGPUTextureUsage usage;
} WGPUTextureDescriptor;

typedef struct WGPUTextureViewDescriptor {
  WGPULabel label;
  WGPUTextureFormat format;
  WGPUTextureViewDimension dimension;
  enum WGPUTextureAspect aspect;
  uint32_t base_mip_level;
  uint32_t level_count;
  uint32_t base_array_layer;
  uint32_t array_layer_count;
} WGPUTextureViewDescriptor;

typedef uint64_t WGPUId_Sampler_Dummy;

typedef WGPUId_Sampler_Dummy WGPUSamplerId;

typedef struct WGPUChainedStruct {
  const struct WGPUChainedStruct *next;
  WGPUSType s_type;
} WGPUChainedStruct;

typedef struct WGPUSamplerDescriptor {
  const struct WGPUChainedStruct *next_in_chain;
  WGPULabel label;
  enum WGPUAddressMode address_mode_u;
  enum WGPUAddressMode address_mode_v;
  enum WGPUAddressMode address_mode_w;
  enum WGPUFilterMode mag_filter;
  enum WGPUFilterMode min_filter;
  enum WGPUFilterMode mipmap_filter;
  float lod_min_clamp;
  float lod_max_clamp;
  WGPUCompareFunction compare;
  WGPUSamplerBorderColor border_color;
} WGPUSamplerDescriptor;

typedef uint64_t WGPUId_BindGroupLayout_Dummy;

typedef WGPUId_BindGroupLayout_Dummy WGPUBindGroupLayoutId;

/**
 * Describes the shader stages that a binding will be visible from.
 *
 * These can be combined so something that is visible from both vertex and fragment shaders can be defined as:
 *
 * `ShaderStage::VERTEX | ShaderStage::FRAGMENT`
 */
typedef uint32_t WGPUShaderStage;
/**
 * Binding is not visible from any shader stage.
 */
#define WGPUShaderStage_NONE (uint32_t)0
/**
 * Binding is visible from the vertex shader of a render pipeline.
 */
#define WGPUShaderStage_VERTEX (uint32_t)1
/**
 * Binding is visible from the fragment shader of a render pipeline.
 */
#define WGPUShaderStage_FRAGMENT (uint32_t)2
/**
 * Binding is visible from the compute shader of a compute pipeline.
 */
#define WGPUShaderStage_COMPUTE (uint32_t)4

typedef struct WGPUBindGroupLayoutEntry {
  uint32_t binding;
  WGPUShaderStage visibility;
  WGPUBindingType ty;
  bool has_dynamic_offset;
  uint64_t min_buffer_binding_size;
  bool multisampled;
  bool filtering;
  enum WGPUTextureViewDimension view_dimension;
  WGPUTextureComponentType texture_component_type;
  enum WGPUTextureFormat storage_texture_format;
  uint32_t count;
} WGPUBindGroupLayoutEntry;

typedef struct WGPUBindGroupLayoutDescriptor {
  WGPULabel label;
  const struct WGPUBindGroupLayoutEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupLayoutDescriptor;

typedef uint64_t WGPUId_PipelineLayout_Dummy;

typedef WGPUId_PipelineLayout_Dummy WGPUPipelineLayoutId;

typedef struct WGPUPipelineLayoutDescriptor {
  WGPULabel label;
  const WGPUBindGroupLayoutId *bind_group_layouts;
  uintptr_t bind_group_layouts_length;
} WGPUPipelineLayoutDescriptor;

typedef uint64_t WGPUId_BindGroup_Dummy;

typedef WGPUId_BindGroup_Dummy WGPUBindGroupId;

/**
 * Integral type used for buffer slice sizes.
 */
typedef uint64_t WGPUBufferSize;

typedef struct WGPUBindGroupEntry {
  uint32_t binding;
  WGPUOption_BufferId buffer;
  WGPUBufferAddress offset;
  WGPUBufferSize size;
  WGPUOption_SamplerId sampler;
  WGPUOption_TextureViewId texture_view;
} WGPUBindGroupEntry;

typedef struct WGPUBindGroupDescriptor {
  WGPULabel label;
  WGPUBindGroupLayoutId layout;
  const struct WGPUBindGroupEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupDescriptor;

typedef uint64_t WGPUId_ShaderModule_Dummy;

typedef WGPUId_ShaderModule_Dummy WGPUShaderModuleId;

/**
 * Flags controlling the shader processing.
 *
 * Note: These flags are internal tweaks, they don't affect the API.
 */
typedef uint32_t WGPUShaderFlags;
/**
 * If enabled, `wgpu` will parse the shader with `Naga`
 * and validate it both internally and with regards to
 * the given pipeline interface.
 */
#define WGPUShaderFlags_VALIDATION (uint32_t)1
/**
 * If enabled, `wgpu` will attempt to operate on `Naga`'s internal
 * representation of the shader module for both validation and translation
 * into the backend shader language, on backends where `gfx-hal` supports this.
 */
#define WGPUShaderFlags_EXPERIMENTAL_TRANSLATION (uint32_t)2

typedef struct WGPUShaderModuleDescriptor {
  const struct WGPUChainedStruct *next_in_chain;
  WGPULabel label;
  WGPUShaderFlags flags;
} WGPUShaderModuleDescriptor;

/**
 * Describes a [`CommandEncoder`].
 */
typedef struct WGPUCommandEncoderDescriptor {
  /**
   * Debug label for the command encoder. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
} WGPUCommandEncoderDescriptor;

typedef struct WGPURenderBundleEncoder *WGPURenderBundleEncoderId;

typedef struct WGPURenderBundleEncoderDescriptor {
  WGPULabel label;
  const enum WGPUTextureFormat *color_formats;
  uintptr_t color_formats_length;
  const enum WGPUTextureFormat *depth_stencil_format;
  uint32_t sample_count;
} WGPURenderBundleEncoderDescriptor;

typedef uint64_t WGPUId_RenderBundle;

typedef WGPUId_RenderBundle WGPURenderBundleId;

/**
 * Describes a [`RenderBundle`].
 */
typedef struct WGPURenderBundleDescriptor_Label {
  /**
   * Debug label of the render bundle encoder. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
} WGPURenderBundleDescriptor_Label;

typedef WGPUDeviceId WGPUQueueId;

typedef uint64_t WGPUId_RenderPipeline_Dummy;

typedef WGPUId_RenderPipeline_Dummy WGPURenderPipelineId;

typedef struct WGPUProgrammableStageDescriptor {
  WGPUShaderModuleId module;
  WGPULabel entry_point;
} WGPUProgrammableStageDescriptor;

/**
 * Integral type used for binding locations in shaders.
 */
typedef uint32_t WGPUShaderLocation;

/**
 * Vertex inputs (attributes) to shaders.
 *
 * Arrays of these can be made with the [`vertex_attr_array`] macro. Vertex attributes are assumed to be tightly packed.
 */
typedef struct WGPUVertexAttribute {
  /**
   * Format of the input
   */
  enum WGPUVertexFormat format;
  /**
   * Byte offset of the start of the input
   */
  WGPUBufferAddress offset;
  /**
   * Location for this input. Must match the location in the shader.
   */
  WGPUShaderLocation shader_location;
} WGPUVertexAttribute;

typedef struct WGPUVertexBufferLayout {
  WGPUBufferAddress array_stride;
  enum WGPUInputStepMode step_mode;
  const struct WGPUVertexAttribute *attributes;
  uintptr_t attributes_count;
} WGPUVertexBufferLayout;

typedef struct WGPUVertexState {
  struct WGPUProgrammableStageDescriptor stage;
  const struct WGPUVertexBufferLayout *buffers;
  uintptr_t buffer_count;
} WGPUVertexState;

typedef struct WGPUPrimitiveState {
  enum WGPUPrimitiveTopology topology;
  WGPUIndexFormat strip_index_format;
  enum WGPUFrontFace front_face;
  enum WGPUCullMode cull_mode;
  enum WGPUPolygonMode polygon_mode;
} WGPUPrimitiveState;

/**
 * Describes stencil state in a render pipeline.
 *
 * If you are not using stencil state, set this to [`StencilFaceState::IGNORE`].
 */
typedef struct WGPUStencilFaceState {
  /**
   * Comparison function that determines if the fail_op or pass_op is used on the stencil buffer.
   */
  WGPUCompareFunction compare;
  /**
   * Operation that is preformed when stencil test fails.
   */
  enum WGPUStencilOperation fail_op;
  /**
   * Operation that is performed when depth test fails but stencil test succeeds.
   */
  enum WGPUStencilOperation depth_fail_op;
  /**
   * Operation that is performed when stencil test success.
   */
  enum WGPUStencilOperation pass_op;
} WGPUStencilFaceState;

/**
 * State of the stencil operation (fixed-pipeline stage).
 */
typedef struct WGPUStencilState {
  /**
   * Front face mode.
   */
  struct WGPUStencilFaceState front;
  /**
   * Back face mode.
   */
  struct WGPUStencilFaceState back;
  /**
   * Stencil values are AND'd with this mask when reading and writing from the stencil buffer. Only low 8 bits are used.
   */
  uint32_t read_mask;
  /**
   * Stencil values are AND'd with this mask when writing to the stencil buffer. Only low 8 bits are used.
   */
  uint32_t write_mask;
} WGPUStencilState;

/**
 * Describes the biasing setting for the depth target.
 */
typedef struct WGPUDepthBiasState {
  /**
   * Constant depth biasing factor, in basic units of the depth format.
   */
  int32_t constant;
  /**
   * Slope depth biasing factor.
   */
  float slope_scale;
  /**
   * Depth bias clamp value (absolute).
   */
  float clamp;
} WGPUDepthBiasState;

/**
 * Describes the depth/stencil state in a render pipeline.
 */
typedef struct WGPUDepthStencilState {
  /**
   * Format of the depth/stencil buffer, must be special depth format. Must match the the format
   * of the depth/stencil attachment in [`CommandEncoder::begin_render_pass`].
   */
  enum WGPUTextureFormat format;
  /**
   * If disabled, depth will not be written to.
   */
  bool depth_write_enabled;
  /**
   * Comparison function used to compare depth values in the depth test.
   */
  WGPUCompareFunction depth_compare;
  /**
   * Stencil state.
   */
  struct WGPUStencilState stencil;
  /**
   * Depth bias state.
   */
  struct WGPUDepthBiasState bias;
  /**
   * If enabled polygon depth is clamped to 0-1 range instead of being clipped.
   *
   * Requires `Features::DEPTH_CLAMPING` enabled.
   */
  bool clamp_depth;
} WGPUDepthStencilState;

/**
 * Describes the multi-sampling state of a render pipeline.
 */
typedef struct WGPUMultisampleState {
  /**
   * The number of samples calculated per pixel (for MSAA). For non-multisampled textures,
   * this should be `1`
   */
  uint32_t count;
  /**
   * Bitmask that restricts the samples of a pixel modified by this pipeline. All samples
   * can be enabled using the value `!0`
   */
  uint64_t mask;
  /**
   * When enabled, produces another sample mask per pixel based on the alpha output value, that
   * is ANDed with the sample_mask and the primitive coverage to restrict the set of samples
   * affected by a primitive.
   *
   * The implicit mask produced for alpha of zero is guaranteed to be zero, and for alpha of one
   * is guaranteed to be all 1-s.
   */
  bool alpha_to_coverage_enabled;
} WGPUMultisampleState;

/**
 * Describes the blend state of a pipeline.
 *
 * Alpha blending is very complicated: see the OpenGL or Vulkan spec for more information.
 */
typedef struct WGPUBlendState {
  /**
   * Multiplier for the source, which is produced by the fragment shader.
   */
  enum WGPUBlendFactor src_factor;
  /**
   * Multiplier for the destination, which is stored in the target.
   */
  enum WGPUBlendFactor dst_factor;
  /**
   * The binary operation applied to the source and destination,
   * multiplied by their respective factors.
   */
  enum WGPUBlendOperation operation;
} WGPUBlendState;

/**
 * Color write mask. Disabled color channels will not be written to.
 */
typedef uint32_t WGPUColorWrite;
/**
 * Enable red channel writes
 */
#define WGPUColorWrite_RED (uint32_t)1
/**
 * Enable green channel writes
 */
#define WGPUColorWrite_GREEN (uint32_t)2
/**
 * Enable blue channel writes
 */
#define WGPUColorWrite_BLUE (uint32_t)4
/**
 * Enable alpha channel writes
 */
#define WGPUColorWrite_ALPHA (uint32_t)8
/**
 * Enable red, green, and blue channel writes
 */
#define WGPUColorWrite_COLOR (uint32_t)7
/**
 * Enable writes to all channels.
 */
#define WGPUColorWrite_ALL (uint32_t)15

/**
 * Describes the color state of a render pipeline.
 */
typedef struct WGPUColorTargetState {
  /**
   * The [`TextureFormat`] of the image that this pipeline will render to. Must match the the format
   * of the corresponding color attachment in [`CommandEncoder::begin_render_pass`].
   */
  enum WGPUTextureFormat format;
  /**
   * The alpha blending that is used for this pipeline.
   */
  struct WGPUBlendState alpha_blend;
  /**
   * The color blending that is used for this pipeline.
   */
  struct WGPUBlendState color_blend;
  /**
   * Mask which enables/disables writes to different color/alpha channel.
   */
  WGPUColorWrite write_mask;
} WGPUColorTargetState;

typedef struct WGPUFragmentState {
  struct WGPUProgrammableStageDescriptor stage;
  const struct WGPUColorTargetState *targets;
  uintptr_t target_count;
} WGPUFragmentState;

typedef struct WGPURenderPipelineDescriptor {
  WGPULabel label;
  WGPUOption_PipelineLayoutId layout;
  struct WGPUVertexState vertex;
  struct WGPUPrimitiveState primitive;
  const struct WGPUDepthStencilState *depth_stencil;
  struct WGPUMultisampleState multisample;
  const struct WGPUFragmentState *fragment;
} WGPURenderPipelineDescriptor;

typedef uint64_t WGPUId_ComputePipeline_Dummy;

typedef WGPUId_ComputePipeline_Dummy WGPUComputePipelineId;

typedef struct WGPUComputePipelineDescriptor {
  WGPULabel label;
  WGPUOption_PipelineLayoutId layout;
  struct WGPUProgrammableStageDescriptor stage;
} WGPUComputePipelineDescriptor;

typedef uint64_t WGPUId_SwapChain_Dummy;

typedef WGPUId_SwapChain_Dummy WGPUSwapChainId;

/**
 * Describes a [`SwapChain`].
 */
typedef struct WGPUSwapChainDescriptor {
  /**
   * The usage of the swap chain. The only supported usage is `RENDER_ATTACHMENT`.
   */
  WGPUTextureUsage usage;
  /**
   * The texture format of the swap chain. The only formats that are guaranteed are
   * `Bgra8Unorm` and `Bgra8UnormSrgb`
   */
  enum WGPUTextureFormat format;
  /**
   * Width of the swap chain. Must be the same size as the surface.
   */
  uint32_t width;
  /**
   * Height of the swap chain. Must be the same size as the surface.
   */
  uint32_t height;
  /**
   * Presentation mode of the swap chain. FIFO is the only guaranteed to be supported, though
   * other formats will automatically fall back to FIFO.
   */
  enum WGPUPresentMode present_mode;
} WGPUSwapChainDescriptor;

typedef void (*WGPUBufferMapCallback)(enum WGPUBufferMapAsyncStatus status, uint8_t *userdata);

typedef struct WGPUAdapterInfo {
  /**
   * Adapter name
   */
  char *name;
  /**
   * Length of the adapter name
   */
  uintptr_t name_length;
  /**
   * Vendor PCI id of the adapter
   */
  uintptr_t vendor;
  /**
   * PCI id of the adapter
   */
  uintptr_t device;
  /**
   * Type of device
   */
  WGPUDeviceType device_type;
  /**
   * Backend used for device
   */
  WGPUBackend backend;
} WGPUAdapterInfo;

typedef void (*WGPULogCallback)(int level, const char *msg);

/**
 * Integral type used for dynamic bind group offsets.
 */
typedef uint32_t WGPUDynamicOffset;

typedef const char *WGPURawString;

typedef uint64_t WGPUId_QuerySet_Dummy;

typedef WGPUId_QuerySet_Dummy WGPUQuerySetId;

typedef struct WGPUAnisotropicSamplerDescriptorExt {
  const struct WGPUChainedStruct *next_in_chain;
  WGPUSType s_type;
  uint8_t anisotropic_clamp;
} WGPUAnisotropicSamplerDescriptorExt;

typedef struct WGPUShaderModuleSPIRVDescriptor {
  struct WGPUChainedStruct chain;
  uint32_t code_size;
  const uint32_t *code;
} WGPUShaderModuleSPIRVDescriptor;

typedef struct WGPUShaderModuleWGSLDescriptor {
  struct WGPUChainedStruct chain;
  const char *source;
} WGPUShaderModuleWGSLDescriptor;































/**
 * Bound uniform/storage buffer offsets must be aligned to this number.
 */
#define WGPUBIND_BUFFER_ALIGNMENT 256

/**
 * Buffer to buffer copy offsets and sizes must be aligned to this number.
 */
#define WGPUCOPY_BUFFER_ALIGNMENT 4

/**
 * Vertex buffer strides have to be aligned to this number.
 */
#define WGPUVERTEX_STRIDE_ALIGNMENT 4

unsigned int wgpu_get_version(void);

WGPUCommandBufferId wgpu_command_encoder_finish(WGPUCommandEncoderId encoder_id,
                                                const struct WGPUCommandBufferDescriptor *desc_base);

void wgpu_command_encoder_copy_buffer_to_buffer(WGPUCommandEncoderId command_encoder_id,
                                                WGPUBufferId source,
                                                WGPUBufferAddress source_offset,
                                                WGPUBufferId destination,
                                                WGPUBufferAddress destination_offset,
                                                WGPUBufferAddress size);

void wgpu_command_encoder_copy_buffer_to_texture(WGPUCommandEncoderId command_encoder_id,
                                                 const struct WGPUBufferCopyView *source,
                                                 const struct WGPUTextureCopyView *destination,
                                                 const struct WGPUExtent3d *copy_size);

void wgpu_command_encoder_copy_texture_to_buffer(WGPUCommandEncoderId command_encoder_id,
                                                 const struct WGPUTextureCopyView *source,
                                                 const struct WGPUBufferCopyView *destination,
                                                 const struct WGPUExtent3d *copy_size);

void wgpu_command_encoder_copy_texture_to_texture(WGPUCommandEncoderId command_encoder_id,
                                                  const struct WGPUTextureCopyView *source,
                                                  const struct WGPUTextureCopyView *destination,
                                                  const struct WGPUExtent3d *copy_size);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
struct WGPURenderPass *wgpu_command_encoder_begin_render_pass(WGPUCommandEncoderId encoder_id,
                                                              const struct WGPURenderPassDescriptor *desc);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
void wgpu_render_pass_end_pass(struct WGPURenderPass *pass);

void wgpu_render_pass_destroy(struct WGPURenderPass *pass);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
struct WGPUComputePass *wgpu_command_encoder_begin_compute_pass(WGPUCommandEncoderId encoder_id,
                                                                const struct WGPUComputePassDescriptor *desc);

void wgpu_compute_pass_end_pass(struct WGPUComputePass *pass);

void wgpu_compute_pass_destroy(struct WGPUComputePass *pass);

void wgpu_render_pass_set_index_buffer(struct WGPURenderPass *pass,
                                       WGPUBufferId buffer_id,
                                       WGPUIndexFormat index_format,
                                       WGPUBufferAddress offset,
                                       WGPUOption_BufferSize size);

void wgpu_render_bundle_set_index_buffer(struct WGPURenderBundleEncoder *bundle,
                                         WGPUBufferId buffer_id,
                                         WGPUIndexFormat index_format,
                                         WGPUBufferAddress offset,
                                         WGPUOption_BufferSize size);

WGPUSurfaceId wgpu_create_surface_from_xlib(const void **display, unsigned long window);

WGPUSurfaceId wgpu_create_surface_from_wayland(void *surface, void *display);

WGPUSurfaceId wgpu_create_surface_from_android(void *a_native_window);

WGPUSurfaceId wgpu_create_surface_from_metal_layer(void *layer);

WGPUSurfaceId wgpu_create_surface_from_windows_hwnd(void *_hinstance, void *hwnd);

/**
 * # Safety
 *
 * This function is unsafe as it calls an unsafe extern callback.
 */
void wgpu_request_adapter_async(const WGPURequestAdapterOptions *desc,
                                WGPUBackendBit mask,
                                WGPURequestAdapterCallback callback,
                                void *userdata);

WGPUDeviceId wgpu_adapter_request_device(WGPUAdapterId adapter_id,
                                         const struct WGPUDeviceDescriptor *desc);

WGPUFeatures wgpu_adapter_features(WGPUAdapterId adapter_id);

struct WGPULimits wgpu_adapter_limits(WGPUAdapterId adapter_id);

void wgpu_adapter_destroy(WGPUAdapterId adapter_id);

WGPUFeatures wgpu_device_features(WGPUDeviceId device_id);

struct WGPULimits wgpu_device_limits(WGPUDeviceId device_id);

WGPUBufferId wgpu_device_create_buffer(WGPUDeviceId device_id,
                                       const struct WGPUBufferDescriptor *desc);

void wgpu_buffer_destroy(WGPUBufferId buffer_id, bool now);

WGPUTextureId wgpu_device_create_texture(WGPUDeviceId device_id,
                                         const struct WGPUTextureDescriptor *desc);

void wgpu_texture_destroy(WGPUTextureId texture_id, bool now);

WGPUTextureViewId wgpu_texture_create_view(WGPUTextureId texture_id,
                                           const struct WGPUTextureViewDescriptor *desc);

void wgpu_texture_view_destroy(WGPUTextureViewId texture_view_id, bool now);

WGPUSamplerId wgpu_device_create_sampler(WGPUDeviceId device_id,
                                         const struct WGPUSamplerDescriptor *desc);

void wgpu_sampler_destroy(WGPUSamplerId sampler_id);

WGPUBindGroupLayoutId wgpu_device_create_bind_group_layout(WGPUDeviceId device_id,
                                                           const struct WGPUBindGroupLayoutDescriptor *desc);

void wgpu_bind_group_layout_destroy(WGPUBindGroupLayoutId bind_group_layout_id);

WGPUPipelineLayoutId wgpu_device_create_pipeline_layout(WGPUDeviceId device_id,
                                                        const struct WGPUPipelineLayoutDescriptor *desc_base);

void wgpu_pipeline_layout_destroy(WGPUPipelineLayoutId pipeline_layout_id);

WGPUBindGroupId wgpu_device_create_bind_group(WGPUDeviceId device_id,
                                              const struct WGPUBindGroupDescriptor *desc);

void wgpu_bind_group_destroy(WGPUBindGroupId bind_group_id);

WGPUShaderModuleId wgpu_device_create_shader_module(WGPUDeviceId device_id,
                                                    const struct WGPUShaderModuleDescriptor *desc);

void wgpu_shader_module_destroy(WGPUShaderModuleId shader_module_id);

WGPUCommandEncoderId wgpu_device_create_command_encoder(WGPUDeviceId device_id,
                                                        const struct WGPUCommandEncoderDescriptor *desc);

void wgpu_command_encoder_destroy(WGPUCommandEncoderId command_encoder_id);

void wgpu_command_buffer_destroy(WGPUCommandBufferId command_buffer_id);

WGPURenderBundleEncoderId wgpu_device_create_render_bundle_encoder(WGPUDeviceId device_id,
                                                                   const struct WGPURenderBundleEncoderDescriptor *desc);

WGPURenderBundleId wgpu_render_bundle_encoder_finish(WGPURenderBundleEncoderId bundle_encoder_id,
                                                     const struct WGPURenderBundleDescriptor_Label *desc);

void wgpu_render_bundle_destroy(WGPURenderBundleId render_bundle_id);

WGPUQueueId wgpu_device_get_default_queue(WGPUDeviceId device_id);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given `data`
 * pointer is valid for `data_length` elements.
 */
void wgpu_queue_write_buffer(WGPUQueueId queue_id,
                             WGPUBufferId buffer_id,
                             WGPUBufferAddress buffer_offset,
                             const uint8_t *data,
                             uintptr_t data_length);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given `data`
 * pointer is valid for `data_length` elements.
 */
void wgpu_queue_write_texture(WGPUQueueId queue_id,
                              const struct WGPUTextureCopyView *texture,
                              const uint8_t *data,
                              uintptr_t data_length,
                              const struct WGPUTextureDataLayout *data_layout,
                              const struct WGPUExtent3d *size);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given `command_buffers`
 * pointer is valid for `command_buffers_length` elements.
 */
void wgpu_queue_submit(WGPUQueueId queue_id,
                       const WGPUCommandBufferId *command_buffers,
                       uintptr_t command_buffers_length);

WGPURenderPipelineId wgpu_device_create_render_pipeline(WGPUDeviceId device_id,
                                                        const struct WGPURenderPipelineDescriptor *desc_base);

void wgpu_render_pipeline_destroy(WGPURenderPipelineId render_pipeline_id);

WGPUComputePipelineId wgpu_device_create_compute_pipeline(WGPUDeviceId device_id,
                                                          const struct WGPUComputePipelineDescriptor *desc);

void wgpu_compute_pipeline_destroy(WGPUComputePipelineId compute_pipeline_id);

WGPUSwapChainId wgpu_device_create_swap_chain(WGPUDeviceId device_id,
                                              WGPUSurfaceId surface_id,
                                              const struct WGPUSwapChainDescriptor *desc);

void wgpu_device_poll(WGPUDeviceId device_id, bool force_wait);

void wgpu_device_destroy(WGPUDeviceId device_id);

void wgpu_buffer_map_read_async(WGPUBufferId buffer_id,
                                WGPUBufferAddress start,
                                WGPUBufferAddress size,
                                WGPUBufferMapCallback callback,
                                uint8_t *user_data);

void wgpu_buffer_map_write_async(WGPUBufferId buffer_id,
                                 WGPUBufferAddress start,
                                 WGPUBufferAddress size,
                                 WGPUBufferMapCallback callback,
                                 uint8_t *user_data);

void wgpu_buffer_unmap(WGPUBufferId buffer_id);

WGPUOption_TextureViewId wgpu_swap_chain_get_current_texture_view(WGPUSwapChainId swap_chain_id);

enum WGPUSwapChainStatus wgpu_swap_chain_present(WGPUSwapChainId swap_chain_id);

uint8_t *wgpu_buffer_get_mapped_range(WGPUBufferId buffer_id,
                                      WGPUBufferAddress start,
                                      WGPUBufferSize size);

/**
 * Fills the given `info` struct with the adapter info.
 *
 * # Safety
 *
 * The field `info.name` is expected to point to a pre-allocated memory
 * location. This function is unsafe as there is no guarantee that the
 * pointer is valid and big enough to hold the adapter name.
 */
void wgpu_adapter_get_info(WGPUAdapterId adapter_id, struct WGPUAdapterInfo *info);

void wgpu_set_log_callback(WGPULogCallback callback);

int wgpu_set_log_level(enum WGPULogLevel level);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_render_bundle_set_bind_group(struct WGPURenderBundleEncoder *bundle,
                                       uint32_t index,
                                       WGPUBindGroupId bind_group_id,
                                       const WGPUDynamicOffset *offsets,
                                       uintptr_t offset_length);

void wgpu_render_bundle_set_pipeline(struct WGPURenderBundleEncoder *bundle,
                                     WGPURenderPipelineId pipeline_id);

void wgpu_render_bundle_set_vertex_buffer(struct WGPURenderBundleEncoder *bundle,
                                          uint32_t slot,
                                          WGPUBufferId buffer_id,
                                          WGPUBufferAddress offset,
                                          WGPUOption_BufferSize size);

void wgpu_render_bundle_set_push_constants(struct WGPURenderBundleEncoder *pass,
                                           WGPUShaderStage stages,
                                           uint32_t offset,
                                           uint32_t size_bytes,
                                           const uint8_t *data);

void wgpu_render_bundle_draw(struct WGPURenderBundleEncoder *bundle,
                             uint32_t vertex_count,
                             uint32_t instance_count,
                             uint32_t first_vertex,
                             uint32_t first_instance);

void wgpu_render_bundle_draw_indexed(struct WGPURenderBundleEncoder *bundle,
                                     uint32_t index_count,
                                     uint32_t instance_count,
                                     uint32_t first_index,
                                     int32_t base_vertex,
                                     uint32_t first_instance);

void wgpu_render_bundle_draw_indirect(struct WGPURenderBundleEncoder *bundle,
                                      WGPUBufferId buffer_id,
                                      WGPUBufferAddress offset);

void wgpu_render_pass_bundle_indexed_indirect(struct WGPURenderBundleEncoder *bundle,
                                              WGPUBufferId buffer_id,
                                              WGPUBufferAddress offset);

void wgpu_render_bundle_push_debug_group(struct WGPURenderBundleEncoder *_bundle,
                                         WGPURawString _label);

void wgpu_render_bundle_pop_debug_group(struct WGPURenderBundleEncoder *_bundle);

void wgpu_render_bundle_insert_debug_marker(struct WGPURenderBundleEncoder *_bundle,
                                            WGPURawString _label);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_compute_pass_set_bind_group(struct WGPUComputePass *pass,
                                      uint32_t index,
                                      WGPUBindGroupId bind_group_id,
                                      const WGPUDynamicOffset *offsets,
                                      uintptr_t offset_length);

void wgpu_compute_pass_set_pipeline(struct WGPUComputePass *pass,
                                    WGPUComputePipelineId pipeline_id);

void wgpu_compute_pass_set_push_constant(struct WGPUComputePass *pass,
                                         uint32_t offset,
                                         uint32_t size_bytes,
                                         const uint8_t *data);

void wgpu_compute_pass_dispatch(struct WGPUComputePass *pass,
                                uint32_t groups_x,
                                uint32_t groups_y,
                                uint32_t groups_z);

void wgpu_compute_pass_dispatch_indirect(struct WGPUComputePass *pass,
                                         WGPUBufferId buffer_id,
                                         WGPUBufferAddress offset);

void wgpu_compute_pass_push_debug_group(struct WGPUComputePass *pass,
                                        WGPURawString label,
                                        uint32_t color);

void wgpu_compute_pass_pop_debug_group(struct WGPUComputePass *pass);

void wgpu_compute_pass_insert_debug_marker(struct WGPUComputePass *pass,
                                           WGPURawString label,
                                           uint32_t color);

void wgpu_compute_pass_write_timestamp(struct WGPUComputePass *pass,
                                       WGPUQuerySetId query_set_id,
                                       uint32_t query_index);

void wgpu_compute_pass_begin_pipeline_statistics_query(struct WGPUComputePass *pass,
                                                       WGPUQuerySetId query_set_id,
                                                       uint32_t query_index);

void wgpu_compute_pass_end_pipeline_statistics_query(struct WGPUComputePass *pass);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_render_pass_set_bind_group(struct WGPURenderPass *pass,
                                     uint32_t index,
                                     WGPUBindGroupId bind_group_id,
                                     const WGPUDynamicOffset *offsets,
                                     uintptr_t offset_length);

void wgpu_render_pass_set_pipeline(struct WGPURenderPass *pass, WGPURenderPipelineId pipeline_id);

void wgpu_render_pass_set_vertex_buffer(struct WGPURenderPass *pass,
                                        uint32_t slot,
                                        WGPUBufferId buffer_id,
                                        WGPUBufferAddress offset,
                                        WGPUOption_BufferSize size);

void wgpu_render_pass_set_blend_color(struct WGPURenderPass *pass, const struct WGPUColor *color);

void wgpu_render_pass_set_stencil_reference(struct WGPURenderPass *pass, uint32_t value);

void wgpu_render_pass_set_viewport(struct WGPURenderPass *pass,
                                   float x,
                                   float y,
                                   float w,
                                   float h,
                                   float depth_min,
                                   float depth_max);

void wgpu_render_pass_set_scissor_rect(struct WGPURenderPass *pass,
                                       uint32_t x,
                                       uint32_t y,
                                       uint32_t w,
                                       uint32_t h);

void wgpu_render_pass_set_push_constants(struct WGPURenderPass *pass,
                                         WGPUShaderStage stages,
                                         uint32_t offset,
                                         uint32_t size_bytes,
                                         const uint8_t *data);

void wgpu_render_pass_draw(struct WGPURenderPass *pass,
                           uint32_t vertex_count,
                           uint32_t instance_count,
                           uint32_t first_vertex,
                           uint32_t first_instance);

void wgpu_render_pass_draw_indexed(struct WGPURenderPass *pass,
                                   uint32_t index_count,
                                   uint32_t instance_count,
                                   uint32_t first_index,
                                   int32_t base_vertex,
                                   uint32_t first_instance);

void wgpu_render_pass_draw_indirect(struct WGPURenderPass *pass,
                                    WGPUBufferId buffer_id,
                                    WGPUBufferAddress offset);

void wgpu_render_pass_draw_indexed_indirect(struct WGPURenderPass *pass,
                                            WGPUBufferId buffer_id,
                                            WGPUBufferAddress offset);

void wgpu_render_pass_multi_draw_indirect(struct WGPURenderPass *pass,
                                          WGPUBufferId buffer_id,
                                          WGPUBufferAddress offset,
                                          uint32_t count);

void wgpu_render_pass_multi_draw_indexed_indirect(struct WGPURenderPass *pass,
                                                  WGPUBufferId buffer_id,
                                                  WGPUBufferAddress offset,
                                                  uint32_t count);

void wgpu_render_pass_multi_draw_indirect_count(struct WGPURenderPass *pass,
                                                WGPUBufferId buffer_id,
                                                WGPUBufferAddress offset,
                                                WGPUBufferId count_buffer_id,
                                                WGPUBufferAddress count_buffer_offset,
                                                uint32_t max_count);

void wgpu_render_pass_multi_draw_indexed_indirect_count(struct WGPURenderPass *pass,
                                                        WGPUBufferId buffer_id,
                                                        WGPUBufferAddress offset,
                                                        WGPUBufferId count_buffer_id,
                                                        WGPUBufferAddress count_buffer_offset,
                                                        uint32_t max_count);

void wgpu_render_pass_push_debug_group(struct WGPURenderPass *pass,
                                       WGPURawString label,
                                       uint32_t color);

void wgpu_render_pass_pop_debug_group(struct WGPURenderPass *pass);

void wgpu_render_pass_insert_debug_marker(struct WGPURenderPass *pass,
                                          WGPURawString label,
                                          uint32_t color);

void wgpu_render_pass_write_timestamp(struct WGPURenderPass *pass,
                                      WGPUQuerySetId query_set_id,
                                      uint32_t query_index);

void wgpu_render_pass_begin_pipeline_statistics_query(struct WGPURenderPass *pass,
                                                      WGPUQuerySetId query_set_id,
                                                      uint32_t query_index);

void wgpu_render_pass_end_pipeline_statistics_query(struct WGPURenderPass *pass);
