/* Generated with cbindgen:0.14.4 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen toolkit/library/rust/ --lockfile Cargo.lock --crate wgpu-remote -o dom/webgpu/ffi/wgpu_ffi_generated.h`
 */

typedef unsigned long long WGPUNonZeroU64;
typedef unsigned long WGPUOption_NonZeroU32;
typedef unsigned long WGPUOption_NonZeroU64;
typedef unsigned long long WGPUOption_AdapterId;
typedef unsigned long long WGPUOption_BufferId;
typedef unsigned long long WGPUOption_SamplerId;
typedef unsigned long long WGPUOption_SurfaceId;
typedef unsigned long long WGPUOption_TextureViewId;

typedef struct WGPUChainedStruct WGPUChainedStruct;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Buffer-Texture copies must have [`bytes_per_row`] aligned to this number.
 *
 * This doesn't apply to [`Queue::write_texture`].
 *
 * [`bytes_per_row`]: TextureDataLayout::bytes_per_row
 */
#define WGPUCOPY_BYTES_PER_ROW_ALIGNMENT 256

#define WGPUDESIRED_NUM_FRAMES 3

#define WGPUMAX_ANISOTROPY 16

#define WGPUMAX_COLOR_TARGETS 4

#define WGPUMAX_MIP_LEVELS 16

#define WGPUMAX_VERTEX_BUFFERS 16

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
} WGPUAddressMode;

/**
 * Backends supported by wgpu.
 */
enum WGPUBackend {
  WGPUBackend_Empty = 0,
  WGPUBackend_Vulkan = 1,
  WGPUBackend_Metal = 2,
  WGPUBackend_Dx12 = 3,
  WGPUBackend_Dx11 = 4,
  WGPUBackend_Gl = 5,
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
  WGPUBlendFactor_Zero = 0,
  WGPUBlendFactor_One = 1,
  WGPUBlendFactor_SrcColor = 2,
  WGPUBlendFactor_OneMinusSrcColor = 3,
  WGPUBlendFactor_SrcAlpha = 4,
  WGPUBlendFactor_OneMinusSrcAlpha = 5,
  WGPUBlendFactor_DstColor = 6,
  WGPUBlendFactor_OneMinusDstColor = 7,
  WGPUBlendFactor_DstAlpha = 8,
  WGPUBlendFactor_OneMinusDstAlpha = 9,
  WGPUBlendFactor_SrcAlphaSaturated = 10,
  WGPUBlendFactor_BlendColor = 11,
  WGPUBlendFactor_OneMinusBlendColor = 12,
} WGPUBlendFactor;

/**
 * Alpha blend operation.
 *
 * Alpha blending is very complicated: see the OpenGL or Vulkan spec for more information.
 */
typedef enum WGPUBlendOperation {
  WGPUBlendOperation_Add = 0,
  WGPUBlendOperation_Subtract = 1,
  WGPUBlendOperation_ReverseSubtract = 2,
  WGPUBlendOperation_Min = 3,
  WGPUBlendOperation_Max = 4,
} WGPUBlendOperation;

typedef enum WGPUBufferMapAsyncStatus {
  WGPUBufferMapAsyncStatus_Success,
  WGPUBufferMapAsyncStatus_Error,
  WGPUBufferMapAsyncStatus_Unknown,
  WGPUBufferMapAsyncStatus_ContextLost,
} WGPUBufferMapAsyncStatus;

enum WGPUCDeviceType {
  /**
   * Other.
   */
  WGPUCDeviceType_Other = 0,
  /**
   * Integrated GPU with shared CPU/GPU memory.
   */
  WGPUCDeviceType_IntegratedGpu,
  /**
   * Discrete GPU with separate CPU/GPU memory.
   */
  WGPUCDeviceType_DiscreteGpu,
  /**
   * Virtual / Hosted.
   */
  WGPUCDeviceType_VirtualGpu,
  /**
   * Cpu / Software Rendering.
   */
  WGPUCDeviceType_Cpu,
};
typedef uint8_t WGPUCDeviceType;

/**
 * Comparison function used for depth and stencil operations.
 */
typedef enum WGPUCompareFunction {
  /**
   * Invalid value, do not use
   */
  WGPUCompareFunction_Undefined = 0,
  /**
   * Function never passes
   */
  WGPUCompareFunction_Never = 1,
  /**
   * Function passes if new value less than existing value
   */
  WGPUCompareFunction_Less = 2,
  /**
   * Function passes if new value is equal to existing value
   */
  WGPUCompareFunction_Equal = 3,
  /**
   * Function passes if new value is less than or equal to existing value
   */
  WGPUCompareFunction_LessEqual = 4,
  /**
   * Function passes if new value is greater than existing value
   */
  WGPUCompareFunction_Greater = 5,
  /**
   * Function passes if new value is not equal to existing value
   */
  WGPUCompareFunction_NotEqual = 6,
  /**
   * Function passes if new value is greater than or equal to existing value
   */
  WGPUCompareFunction_GreaterEqual = 7,
  /**
   * Function always passes
   */
  WGPUCompareFunction_Always = 8,
} WGPUCompareFunction;

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

/**
 * Format of indices used with pipeline.
 */
typedef enum WGPUIndexFormat {
  /**
   * Indices are 16 bit unsigned integers.
   */
  WGPUIndexFormat_Uint16 = 0,
  /**
   * Indices are 32 bit unsigned integers.
   */
  WGPUIndexFormat_Uint32 = 1,
} WGPUIndexFormat;

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
 * Power Preference when choosing a physical adapter.
 */
typedef enum WGPUPowerPreference {
  /**
   * Prefer low power when on battery, high performance when on mains.
   */
  WGPUPowerPreference_Default = 0,
  /**
   * Adapter that uses the least possible power. This is often an integerated GPU.
   */
  WGPUPowerPreference_LowPower = 1,
  /**
   * Adapter that has the highest performance. This is often a discrete GPU.
   */
  WGPUPowerPreference_HighPerformance = 2,
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
  WGPUSwapChainStatus_Good,
  WGPUSwapChainStatus_Suboptimal,
  WGPUSwapChainStatus_Timeout,
  WGPUSwapChainStatus_Outdated,
  WGPUSwapChainStatus_Lost,
  WGPUSwapChainStatus_OutOfMemory,
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

/**
 * Type of data shaders will read from a texture.
 *
 * Only relevant for [`BindingType::SampledTexture`] bindings. See [`TextureFormat`] for more information.
 */
typedef enum WGPUTextureComponentType {
  /**
   * They see it as a floating point number `texture1D`, `texture2D` etc
   */
  WGPUTextureComponentType_Float,
  /**
   * They see it as a signed integer `itexture1D`, `itexture2D` etc
   */
  WGPUTextureComponentType_Sint,
  /**
   * They see it as a unsigned integer `utexture1D`, `utexture2D` etc
   */
  WGPUTextureComponentType_Uint,
} WGPUTextureComponentType;

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
   * Red, green, and blue channels. 11 bit float with no sign bit for RG channels. 10 bit float with no sign bti for blue channel. Float in shader.
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
   * Two unsigned shorts (i16). `ivec2` in shaders.
   */
  WGPUVertexFormat_Short2 = 10,
  /**
   * Four unsigned shorts (i16). `ivec4` in shaders.
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
} WGPUVertexFormat;

typedef struct WGPUComputePass WGPUComputePass;

typedef struct WGPUOption_BufferSize WGPUOption_BufferSize;

typedef struct WGPURenderBundleEncoder WGPURenderBundleEncoder;

typedef struct WGPURenderPass WGPURenderPass;

typedef WGPUNonZeroU64 WGPUId_Adapter_Dummy;

typedef WGPUId_Adapter_Dummy WGPUAdapterId;

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
 * Features which are part of the upstream webgpu standard
 */
#define WGPUFeatures_ALL_WEBGPU (uint64_t)65535
/**
 * Features that require activating the unsafe feature flag
 */
#define WGPUFeatures_ALL_UNSAFE (uint64_t)18446462598732840960ULL
/**
 * Features that are only available when targeting native (not web)
 */
#define WGPUFeatures_ALL_NATIVE (uint64_t)18446744073709486080ULL

typedef struct WGPUCAdapterInfo {
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
  WGPUCDeviceType device_type;
  /**
   * Backend used for device
   */
  WGPUBackend backend;
} WGPUCAdapterInfo;

typedef struct WGPUCLimits {
  uint32_t max_bind_groups;
} WGPUCLimits;

typedef WGPUNonZeroU64 WGPUId_Device_Dummy;

typedef WGPUId_Device_Dummy WGPUDeviceId;

typedef WGPUNonZeroU64 WGPUId_BindGroup_Dummy;

typedef WGPUId_BindGroup_Dummy WGPUBindGroupId;

typedef WGPUNonZeroU64 WGPUId_BindGroupLayout_Dummy;

typedef WGPUId_BindGroupLayout_Dummy WGPUBindGroupLayoutId;

typedef WGPUNonZeroU64 WGPUId_Buffer_Dummy;

typedef WGPUId_Buffer_Dummy WGPUBufferId;

/**
 * Integral type used for buffer offsets.
 */
typedef uint64_t WGPUBufferAddress;

/**
 * Integral type used for buffer slice sizes.
 */
typedef WGPUNonZeroU64 WGPUBufferSize;

typedef void (*WGPUBufferMapCallback)(WGPUBufferMapAsyncStatus status, uint8_t *userdata);

typedef WGPUNonZeroU64 WGPUId_CommandBuffer_Dummy;

typedef WGPUId_CommandBuffer_Dummy WGPUCommandBufferId;

typedef WGPUCommandBufferId WGPUCommandEncoderId;

typedef struct WGPUComputePassDescriptor {
  uint32_t todo;
} WGPUComputePassDescriptor;

typedef WGPUNonZeroU64 WGPUId_TextureView_Dummy;

typedef WGPUId_TextureView_Dummy WGPUTextureViewId;

/**
 * RGBA double precision color.
 *
 * This is not to be used as a generic color type, only for specific wgpu interfaces.
 */
typedef struct WGPUColor {
  double r;
  double g;
  double b;
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
  WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  WGPUStoreOp store_op;
  /**
   * If load_op is [`LoadOp::Clear`], the attachement will be cleared to this color.
   */
  WGPUColor clear_value;
  /**
   * If true, the relevant channel is not changed by a renderpass, and the corresponding attachment
   * can be used inside the pass by other read-only usages.
   */
  bool read_only;
} WGPUPassChannel_Color;

/**
 * Describes a color attachment to a [`RenderPass`].
 */
typedef struct WGPURenderPassColorAttachmentDescriptorBase_TextureViewId {
  /**
   * Texture attachment to render to. Must contain [`TextureUsage::OUTPUT_ATTACHMENT`].
   */
  WGPUTextureViewId attachment;
  /**
   * MSAA resolve target. Must contain [`TextureUsage::OUTPUT_ATTACHMENT`]. Must be `None` if
   * attachment has 1 sample (does not have MSAA). This is not mandatory for rendering with multisampling,
   * you can choose to resolve later or manually.
   */
  WGPUOption_TextureViewId resolve_target;
  /**
   * Color channel.
   */
  WGPUPassChannel_Color channel;
} WGPURenderPassColorAttachmentDescriptorBase_TextureViewId;

typedef WGPURenderPassColorAttachmentDescriptorBase_TextureViewId WGPURenderPassColorAttachmentDescriptor;

/**
 * Describes an individual channel within a render pass, such as color, depth, or stencil.
 */
typedef struct WGPUPassChannel_f32 {
  /**
   * Operation to perform to the output attachment at the start of a renderpass. This must be clear if it
   * is the first renderpass rendering to a swap chain image.
   */
  WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  WGPUStoreOp store_op;
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
  WGPULoadOp load_op;
  /**
   * Operation to perform to the output attachment at the end of a renderpass.
   */
  WGPUStoreOp store_op;
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
 * Describes a depth/stencil attachment to a [`RenderPass`].
 */
typedef struct WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId {
  /**
   * Texture attachment to render to. Must contain [`TextureUsage::OUTPUT_ATTACHMENT`] and be a valid
   * texture type for a depth/stencil attachment.
   */
  WGPUTextureViewId attachment;
  /**
   * Depth channel.
   */
  WGPUPassChannel_f32 depth;
  /**
   * Stencil channel.
   */
  WGPUPassChannel_u32 stencil;
} WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId;

typedef WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId WGPURenderPassDepthStencilAttachmentDescriptor;

typedef struct WGPURenderPassDescriptor {
  const WGPURenderPassColorAttachmentDescriptor *color_attachments;
  uintptr_t color_attachments_length;
  const WGPURenderPassDepthStencilAttachmentDescriptor *depth_stencil_attachment;
} WGPURenderPassDescriptor;

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
  WGPUBufferId buffer;
  WGPUTextureDataLayout layout;
} WGPUBufferCopyView;

typedef WGPUNonZeroU64 WGPUId_Texture_Dummy;

typedef WGPUId_Texture_Dummy WGPUTextureId;

/**
 * Origin of a copy to/from a texture.
 */
typedef struct WGPUOrigin3d {
  uint32_t x;
  uint32_t y;
  uint32_t z;
} WGPUOrigin3d;
#define WGPUOrigin3d_ZERO (WGPUOrigin3d){ .x = 0, .y = 0, .z = 0 }

typedef struct WGPUTextureCopyView {
  WGPUTextureId texture;
  uint32_t mip_level;
  WGPUOrigin3d origin;
} WGPUTextureCopyView;

/**
 * Extent of a texture related operation.
 */
typedef struct WGPUExtent3d {
  uint32_t width;
  uint32_t height;
  uint32_t depth;
} WGPUExtent3d;

/**
 * Describes a [`CommandBuffer`].
 */
typedef struct WGPUCommandBufferDescriptor {
  /**
   * Set this member to zero
   */
  uint32_t todo;
} WGPUCommandBufferDescriptor;

typedef const char *WGPURawString;

/**
 * Integral type used for dynamic bind group offsets.
 */
typedef uint32_t WGPUDynamicOffset;

typedef WGPUNonZeroU64 WGPUId_ComputePipeline_Dummy;

typedef WGPUId_ComputePipeline_Dummy WGPUComputePipelineId;

typedef WGPUNonZeroU64 WGPUId_Surface;

typedef WGPUId_Surface WGPUSurfaceId;

typedef const char *WGPULabel;

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
  const WGPUBindGroupEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupDescriptor;

/**
 * Describes the shader stages that a binding will be visible from.
 *
 * These can be combined so something that is visible from both vertex and fragment shaders can be defined as:
 *
 * `ShaderStage::VERTEX | ShaderStage::FRAGMENT`
 */
typedef uint32_t WGPUShaderStage;
/**
 * Binding is not visible from any shader stage
 */
#define WGPUShaderStage_NONE (uint32_t)0
/**
 * Binding is visible from the vertex shader of a render pipeline
 */
#define WGPUShaderStage_VERTEX (uint32_t)1
/**
 * Binding is visible from the fragment shader of a render pipeline
 */
#define WGPUShaderStage_FRAGMENT (uint32_t)2
/**
 * Binding is visible from the compute shader of a compute pipeline
 */
#define WGPUShaderStage_COMPUTE (uint32_t)4

typedef struct WGPUBindGroupLayoutEntry {
  uint32_t binding;
  WGPUShaderStage visibility;
  WGPUBindingType ty;
  bool has_dynamic_offset;
  WGPUOption_NonZeroU64 min_buffer_binding_size;
  bool multisampled;
  WGPUTextureViewDimension view_dimension;
  WGPUTextureComponentType texture_component_type;
  WGPUTextureFormat storage_texture_format;
  WGPUOption_NonZeroU32 count;
} WGPUBindGroupLayoutEntry;

typedef struct WGPUBindGroupLayoutDescriptor {
  WGPULabel label;
  const WGPUBindGroupLayoutEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupLayoutDescriptor;

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
 * Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_buffer_to_texture`],
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
 * Allow a buffer to be a [`BindingType::UniformBuffer`] inside a bind group.
 */
#define WGPUBufferUsage_UNIFORM (uint32_t)64
/**
 * Allow a buffer to be a [`BindingType::StorageBuffer`] inside a bind group.
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
 * Describes a [`CommandEncoder`].
 */
typedef struct WGPUCommandEncoderDescriptor {
  /**
   * Debug label for the command encoder. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
} WGPUCommandEncoderDescriptor;

typedef WGPUNonZeroU64 WGPUId_PipelineLayout_Dummy;

typedef WGPUId_PipelineLayout_Dummy WGPUPipelineLayoutId;

typedef WGPUNonZeroU64 WGPUId_ShaderModule_Dummy;

typedef WGPUId_ShaderModule_Dummy WGPUShaderModuleId;

typedef struct WGPUProgrammableStageDescriptor {
  WGPUShaderModuleId module;
  WGPURawString entry_point;
} WGPUProgrammableStageDescriptor;

typedef struct WGPUComputePipelineDescriptor {
  WGPUPipelineLayoutId layout;
  WGPUProgrammableStageDescriptor compute_stage;
} WGPUComputePipelineDescriptor;

typedef struct WGPUPipelineLayoutDescriptor {
  const WGPUBindGroupLayoutId *bind_group_layouts;
  uintptr_t bind_group_layouts_length;
} WGPUPipelineLayoutDescriptor;

typedef WGPURenderBundleEncoder *WGPURenderBundleEncoderId;

typedef struct WGPURenderBundleEncoderDescriptor {
  WGPULabel label;
  const WGPUTextureFormat *color_formats;
  uintptr_t color_formats_length;
  const WGPUTextureFormat *depth_stencil_format;
  uint32_t sample_count;
} WGPURenderBundleEncoderDescriptor;

typedef WGPUNonZeroU64 WGPUId_RenderPipeline_Dummy;

typedef WGPUId_RenderPipeline_Dummy WGPURenderPipelineId;

/**
 * Describes the state of the rasterizer in a render pipeline.
 */
typedef struct WGPURasterizationStateDescriptor {
  WGPUFrontFace front_face;
  WGPUCullMode cull_mode;
  int32_t depth_bias;
  float depth_bias_slope_scale;
  float depth_bias_clamp;
} WGPURasterizationStateDescriptor;

/**
 * Describes the blend state of a pipeline.
 *
 * Alpha blending is very complicated: see the OpenGL or Vulkan spec for more information.
 */
typedef struct WGPUBlendDescriptor {
  WGPUBlendFactor src_factor;
  WGPUBlendFactor dst_factor;
  WGPUBlendOperation operation;
} WGPUBlendDescriptor;

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
typedef struct WGPUColorStateDescriptor {
  /**
   * The [`TextureFormat`] of the image that this pipeline will render to. Must match the the format
   * of the corresponding color attachment in [`CommandEncoder::begin_render_pass`].
   */
  WGPUTextureFormat format;
  /**
   * The alpha blending that is used for this pipeline.
   */
  WGPUBlendDescriptor alpha_blend;
  /**
   * The color blending that is used for this pipeline.
   */
  WGPUBlendDescriptor color_blend;
  /**
   * Mask which enables/disables writes to different color/alpha channel.
   */
  WGPUColorWrite write_mask;
} WGPUColorStateDescriptor;

/**
 * Describes stencil state in a render pipeline.
 *
 * If you are not using stencil state, set this to [`StencilStateFaceDescriptor::IGNORE`].
 */
typedef struct WGPUStencilStateFaceDescriptor {
  /**
   * Comparison function that determines if the fail_op or pass_op is used on the stencil buffer.
   */
  WGPUCompareFunction compare;
  /**
   * Operation that is preformed when stencil test fails.
   */
  WGPUStencilOperation fail_op;
  /**
   * Operation that is performed when depth test fails but stencil test succeeds.
   */
  WGPUStencilOperation depth_fail_op;
  /**
   * Operation that is performed when stencil test success.
   */
  WGPUStencilOperation pass_op;
} WGPUStencilStateFaceDescriptor;

/**
 * Describes the depth/stencil state in a render pipeline.
 */
typedef struct WGPUDepthStencilStateDescriptor {
  /**
   * Format of the depth/stencil buffer, must be special depth format. Must match the the format
   * of the depth/stencil attachment in [`CommandEncoder::begin_render_pass`].
   */
  WGPUTextureFormat format;
  /**
   * If disabled, depth will not be written to.
   */
  bool depth_write_enabled;
  /**
   * Comparison function used to compare depth values in the depth test.
   */
  WGPUCompareFunction depth_compare;
  /**
   * Stencil state used for front faces.
   */
  WGPUStencilStateFaceDescriptor stencil_front;
  /**
   * Stencil state used for back faces.
   */
  WGPUStencilStateFaceDescriptor stencil_back;
  /**
   * Stencil values are AND'd with this mask when reading and writing from the stencil buffer. Only low 8 bits are used.
   */
  uint32_t stencil_read_mask;
  /**
   * Stencil values are AND'd with this mask when writing to the stencil buffer. Only low 8 bits are used.
   */
  uint32_t stencil_write_mask;
} WGPUDepthStencilStateDescriptor;

/**
 * Integral type used for binding locations in shaders.
 */
typedef uint32_t WGPUShaderLocation;

/**
 * Vertex inputs (attributes) to shaders.
 *
 * Arrays of these can be made with the [`vertex_attr_array`] macro. Vertex attributes are assumed to be tightly packed.
 */
typedef struct WGPUVertexAttributeDescriptor {
  /**
   * Byte offset of the start of the input
   */
  WGPUBufferAddress offset;
  /**
   * Format of the input
   */
  WGPUVertexFormat format;
  /**
   * Location for this input. Must match the location in the shader.
   */
  WGPUShaderLocation shader_location;
} WGPUVertexAttributeDescriptor;

typedef struct WGPUVertexBufferLayoutDescriptor {
  WGPUBufferAddress array_stride;
  WGPUInputStepMode step_mode;
  const WGPUVertexAttributeDescriptor *attributes;
  uintptr_t attributes_length;
} WGPUVertexBufferLayoutDescriptor;

typedef struct WGPUVertexStateDescriptor {
  WGPUIndexFormat index_format;
  const WGPUVertexBufferLayoutDescriptor *vertex_buffers;
  uintptr_t vertex_buffers_length;
} WGPUVertexStateDescriptor;

typedef struct WGPURenderPipelineDescriptor {
  WGPUPipelineLayoutId layout;
  WGPUProgrammableStageDescriptor vertex_stage;
  const WGPUProgrammableStageDescriptor *fragment_stage;
  WGPUPrimitiveTopology primitive_topology;
  const WGPURasterizationStateDescriptor *rasterization_state;
  const WGPUColorStateDescriptor *color_states;
  uintptr_t color_states_length;
  const WGPUDepthStencilStateDescriptor *depth_stencil_state;
  WGPUVertexStateDescriptor vertex_state;
  uint32_t sample_count;
  uint32_t sample_mask;
  bool alpha_to_coverage_enabled;
} WGPURenderPipelineDescriptor;

typedef WGPUNonZeroU64 WGPUId_Sampler_Dummy;

typedef WGPUId_Sampler_Dummy WGPUSamplerId;

typedef struct WGPUChainedStruct {
  const WGPUChainedStruct *next;
  WGPUSType s_type;
} WGPUChainedStruct;

typedef struct WGPUSamplerDescriptor {
  const WGPUChainedStruct *next_in_chain;
  WGPULabel label;
  WGPUAddressMode address_mode_u;
  WGPUAddressMode address_mode_v;
  WGPUAddressMode address_mode_w;
  WGPUFilterMode mag_filter;
  WGPUFilterMode min_filter;
  WGPUFilterMode mipmap_filter;
  float lod_min_clamp;
  float lod_max_clamp;
  WGPUCompareFunction compare;
} WGPUSamplerDescriptor;

typedef struct WGPUShaderSource {
  const uint32_t *bytes;
  uintptr_t length;
} WGPUShaderSource;

typedef WGPUNonZeroU64 WGPUId_SwapChain_Dummy;

typedef WGPUId_SwapChain_Dummy WGPUSwapChainId;

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
 * Allows a texture to be a [`BindingType::SampledTexture`] in a bind group.
 */
#define WGPUTextureUsage_SAMPLED (uint32_t)4
/**
 * Allows a texture to be a [`BindingType::StorageTexture`] in a bind group.
 */
#define WGPUTextureUsage_STORAGE (uint32_t)8
/**
 * Allows a texture to be a output attachment of a renderpass.
 */
#define WGPUTextureUsage_OUTPUT_ATTACHMENT (uint32_t)16

/**
 * Describes a [`SwapChain`].
 */
typedef struct WGPUSwapChainDescriptor {
  /**
   * The usage of the swap chain. The only supported usage is OUTPUT_ATTACHMENT
   */
  WGPUTextureUsage usage;
  /**
   * The texture format of the swap chain. The only formats that are guaranteed are
   * `Bgra8Unorm` and `Bgra8UnormSrgb`
   */
  WGPUTextureFormat format;
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
  WGPUPresentMode present_mode;
} WGPUSwapChainDescriptor;

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
  WGPUExtent3d size;
  /**
   * Mip count of texture. For a texture with no extra mips, this must be 1.
   */
  uint32_t mip_level_count;
  /**
   * Sample count of texture. If this is not 1, texture must have [`BindingType::SampledTexture::multisampled`] set to true.
   */
  uint32_t sample_count;
  /**
   * Dimensions of the texture.
   */
  WGPUTextureDimension dimension;
  /**
   * Format of the texture.
   */
  WGPUTextureFormat format;
  /**
   * Allowed usages of the texture. If used in other ways, the operation will panic.
   */
  WGPUTextureUsage usage;
} WGPUTextureDescriptor;

typedef WGPUDeviceId WGPUQueueId;

typedef WGPUNonZeroU64 WGPUId_RenderBundle;

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

typedef struct WGPURequestAdapterOptions {
  WGPUPowerPreference power_preference;
  WGPUOption_SurfaceId compatible_surface;
} WGPURequestAdapterOptions;

/**
 * Represents the backends that wgpu will use.
 */
typedef uint32_t WGPUBackendBit;

typedef void (*WGPURequestAdapterCallback)(WGPUOption_AdapterId id, void *userdata);

typedef void (*WGPULogCallback)(int level, const char *msg);

typedef struct WGPUSwapChainOutput {
  WGPUSwapChainStatus status;
  WGPUOption_TextureViewId view_id;
} WGPUSwapChainOutput;

/**
 * Describes a [`TextureView`].
 */
typedef struct WGPUTextureViewDescriptor {
  /**
   * Debug label of the texture view. This will show up in graphics debuggers for easy identification.
   */
  WGPULabel label;
  /**
   * Format of the texture view. At this time, it must be the same as the underlying format of the texture.
   */
  WGPUTextureFormat format;
  /**
   * The dimension of the texture view. For 1D textures, this must be `1D`. For 2D textures it must be one of
   * `D2`, `D2Array`, `Cube`, and `CubeArray`. For 3D textures it must be `3D`
   */
  WGPUTextureViewDimension dimension;
  /**
   * Aspect of the texture. Color textures must be [`TextureAspect::All`].
   */
  WGPUTextureAspect aspect;
  /**
   * Base mip level.
   */
  uint32_t base_mip_level;
  /**
   * Mip level count. Must be at least one. base_mip_level + level_count must be less or equal to underlying texture mip count.
   */
  uint32_t level_count;
  /**
   * Base array layer.
   */
  uint32_t base_array_layer;
  /**
   * Layer count. Must be at least one. base_array_layer + array_layer_count must be less or equal to the underlying array count.
   */
  uint32_t array_layer_count;
} WGPUTextureViewDescriptor;

typedef struct WGPUAnisotropicSamplerDescriptorExt {
  const WGPUChainedStruct *next_in_chain;
  WGPUSType s_type;
  uint8_t anisotropic_clamp;
} WGPUAnisotropicSamplerDescriptorExt;































void wgpu_adapter_destroy(WGPUAdapterId adapter_id);

WGPUFeatures wgpu_adapter_features(WGPUAdapterId adapter_id);

/**
 * Fills the given `info` struct with the adapter info.
 *
 * # Safety
 *
 * The field `info.name` is expected to point to a pre-allocated memory
 * location. This function is unsafe as there is no guarantee that the
 * pointer is valid and big enough to hold the adapter name.
 */
void wgpu_adapter_get_info(WGPUAdapterId adapter_id, WGPUCAdapterInfo *info);

WGPUCLimits wgpu_adapter_limits(WGPUAdapterId adapter_id);

WGPUDeviceId wgpu_adapter_request_device(WGPUAdapterId adapter_id,
                                         WGPUFeatures features,
                                         const WGPUCLimits *limits,
                                         bool shader_validation,
                                         const char *trace_path);

void wgpu_bind_group_destroy(WGPUBindGroupId bind_group_id);

void wgpu_bind_group_layout_destroy(WGPUBindGroupLayoutId bind_group_layout_id);

void wgpu_buffer_destroy(WGPUBufferId buffer_id);

uint8_t *wgpu_buffer_get_mapped_range(WGPUBufferId buffer_id,
                                      WGPUBufferAddress start,
                                      WGPUBufferSize size);

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

void wgpu_command_buffer_destroy(WGPUCommandBufferId command_buffer_id);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
WGPUComputePass *wgpu_command_encoder_begin_compute_pass(WGPUCommandEncoderId encoder_id,
                                                         const WGPUComputePassDescriptor *_desc);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
WGPURenderPass *wgpu_command_encoder_begin_render_pass(WGPUCommandEncoderId encoder_id,
                                                       const WGPURenderPassDescriptor *desc);

void wgpu_command_encoder_copy_buffer_to_buffer(WGPUCommandEncoderId command_encoder_id,
                                                WGPUBufferId source,
                                                WGPUBufferAddress source_offset,
                                                WGPUBufferId destination,
                                                WGPUBufferAddress destination_offset,
                                                WGPUBufferAddress size);

void wgpu_command_encoder_copy_buffer_to_texture(WGPUCommandEncoderId command_encoder_id,
                                                 const WGPUBufferCopyView *source,
                                                 const WGPUTextureCopyView *destination,
                                                 const WGPUExtent3d *copy_size);

void wgpu_command_encoder_copy_texture_to_buffer(WGPUCommandEncoderId command_encoder_id,
                                                 const WGPUTextureCopyView *source,
                                                 const WGPUBufferCopyView *destination,
                                                 const WGPUExtent3d *copy_size);

void wgpu_command_encoder_copy_texture_to_texture(WGPUCommandEncoderId command_encoder_id,
                                                  const WGPUTextureCopyView *source,
                                                  const WGPUTextureCopyView *destination,
                                                  const WGPUExtent3d *copy_size);

void wgpu_command_encoder_destroy(WGPUCommandEncoderId command_encoder_id);

WGPUCommandBufferId wgpu_command_encoder_finish(WGPUCommandEncoderId encoder_id,
                                                const WGPUCommandBufferDescriptor *desc);

void wgpu_compute_pass_destroy(WGPUComputePass *pass);

void wgpu_compute_pass_dispatch(WGPUComputePass *pass,
                                uint32_t groups_x,
                                uint32_t groups_y,
                                uint32_t groups_z);

void wgpu_compute_pass_dispatch_indirect(WGPUComputePass *pass,
                                         WGPUBufferId buffer_id,
                                         WGPUBufferAddress offset);

void wgpu_compute_pass_end_pass(WGPUComputePass *pass);

void wgpu_compute_pass_insert_debug_marker(WGPUComputePass *pass,
                                           WGPURawString label,
                                           uint32_t color);

void wgpu_compute_pass_pop_debug_group(WGPUComputePass *pass);

void wgpu_compute_pass_push_debug_group(WGPUComputePass *pass, WGPURawString label, uint32_t color);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_compute_pass_set_bind_group(WGPUComputePass *pass,
                                      uint32_t index,
                                      WGPUBindGroupId bind_group_id,
                                      const WGPUDynamicOffset *offsets,
                                      uintptr_t offset_length);

void wgpu_compute_pass_set_pipeline(WGPUComputePass *pass, WGPUComputePipelineId pipeline_id);

void wgpu_compute_pipeline_destroy(WGPUComputePipelineId compute_pipeline_id);

WGPUSurfaceId wgpu_create_surface_from_android(void *a_native_window);

WGPUSurfaceId wgpu_create_surface_from_metal_layer(void *layer);

WGPUSurfaceId wgpu_create_surface_from_wayland(void *surface, void *display);

WGPUSurfaceId wgpu_create_surface_from_windows_hwnd(void *_hinstance, void *hwnd);

WGPUSurfaceId wgpu_create_surface_from_xlib(const void **display, unsigned long window);

WGPUBindGroupId wgpu_device_create_bind_group(WGPUDeviceId device_id,
                                              const WGPUBindGroupDescriptor *desc);

WGPUBindGroupLayoutId wgpu_device_create_bind_group_layout(WGPUDeviceId device_id,
                                                           const WGPUBindGroupLayoutDescriptor *desc);

WGPUBufferId wgpu_device_create_buffer(WGPUDeviceId device_id, const WGPUBufferDescriptor *desc);

WGPUCommandEncoderId wgpu_device_create_command_encoder(WGPUDeviceId device_id,
                                                        const WGPUCommandEncoderDescriptor *desc);

WGPUComputePipelineId wgpu_device_create_compute_pipeline(WGPUDeviceId device_id,
                                                          const WGPUComputePipelineDescriptor *desc);

WGPUPipelineLayoutId wgpu_device_create_pipeline_layout(WGPUDeviceId device_id,
                                                        const WGPUPipelineLayoutDescriptor *desc);

WGPURenderBundleEncoderId wgpu_device_create_render_bundle_encoder(WGPUDeviceId device_id,
                                                                   const WGPURenderBundleEncoderDescriptor *desc);

WGPURenderPipelineId wgpu_device_create_render_pipeline(WGPUDeviceId device_id,
                                                        const WGPURenderPipelineDescriptor *desc);

WGPUSamplerId wgpu_device_create_sampler(WGPUDeviceId device_id, const WGPUSamplerDescriptor *desc);

WGPUShaderModuleId wgpu_device_create_shader_module(WGPUDeviceId device_id,
                                                    WGPUShaderSource source);

WGPUSwapChainId wgpu_device_create_swap_chain(WGPUDeviceId device_id,
                                              WGPUSurfaceId surface_id,
                                              const WGPUSwapChainDescriptor *desc);

WGPUTextureId wgpu_device_create_texture(WGPUDeviceId device_id, const WGPUTextureDescriptor *desc);

void wgpu_device_destroy(WGPUDeviceId device_id);

WGPUFeatures wgpu_device_features(WGPUDeviceId device_id);

WGPUQueueId wgpu_device_get_default_queue(WGPUDeviceId device_id);

WGPUCLimits wgpu_device_limits(WGPUDeviceId device_id);

void wgpu_device_poll(WGPUDeviceId device_id, bool force_wait);

unsigned int wgpu_get_version(void);

void wgpu_pipeline_layout_destroy(WGPUPipelineLayoutId pipeline_layout_id);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given `command_buffers`
 * pointer is valid for `command_buffers_length` elements.
 */
void wgpu_queue_submit(WGPUQueueId queue_id,
                       const WGPUCommandBufferId *command_buffers,
                       uintptr_t command_buffers_length);

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
                              const WGPUTextureCopyView *texture,
                              const uint8_t *data,
                              uintptr_t data_length,
                              const WGPUTextureDataLayout *data_layout,
                              const WGPUExtent3d *size);

void wgpu_render_bundle_destroy(WGPURenderBundleId render_bundle_id);

void wgpu_render_bundle_draw(WGPURenderBundleEncoder *bundle,
                             uint32_t vertex_count,
                             uint32_t instance_count,
                             uint32_t first_vertex,
                             uint32_t first_instance);

void wgpu_render_bundle_draw_indexed(WGPURenderBundleEncoder *bundle,
                                     uint32_t index_count,
                                     uint32_t instance_count,
                                     uint32_t first_index,
                                     int32_t base_vertex,
                                     uint32_t first_instance);

void wgpu_render_bundle_draw_indirect(WGPURenderBundleEncoder *bundle,
                                      WGPUBufferId buffer_id,
                                      WGPUBufferAddress offset);

WGPURenderBundleId wgpu_render_bundle_encoder_finish(WGPURenderBundleEncoderId bundle_encoder_id,
                                                     const WGPURenderBundleDescriptor_Label *desc);

void wgpu_render_bundle_insert_debug_marker(WGPURenderBundleEncoder *_bundle, WGPURawString _label);

void wgpu_render_bundle_pop_debug_group(WGPURenderBundleEncoder *_bundle);

void wgpu_render_bundle_push_debug_group(WGPURenderBundleEncoder *_bundle, WGPURawString _label);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_render_bundle_set_bind_group(WGPURenderBundleEncoder *bundle,
                                       uint32_t index,
                                       WGPUBindGroupId bind_group_id,
                                       const WGPUDynamicOffset *offsets,
                                       uintptr_t offset_length);

void wgpu_render_bundle_set_index_buffer(WGPURenderBundleEncoder *bundle,
                                         WGPUBufferId buffer_id,
                                         WGPUBufferAddress offset,
                                         WGPUOption_BufferSize size);

void wgpu_render_bundle_set_pipeline(WGPURenderBundleEncoder *bundle,
                                     WGPURenderPipelineId pipeline_id);

void wgpu_render_bundle_set_vertex_buffer(WGPURenderBundleEncoder *bundle,
                                          uint32_t slot,
                                          WGPUBufferId buffer_id,
                                          WGPUBufferAddress offset,
                                          WGPUOption_BufferSize size);

void wgpu_render_pass_bundle_indexed_indirect(WGPURenderBundleEncoder *bundle,
                                              WGPUBufferId buffer_id,
                                              WGPUBufferAddress offset);

void wgpu_render_pass_destroy(WGPURenderPass *pass);

void wgpu_render_pass_draw(WGPURenderPass *pass,
                           uint32_t vertex_count,
                           uint32_t instance_count,
                           uint32_t first_vertex,
                           uint32_t first_instance);

void wgpu_render_pass_draw_indexed(WGPURenderPass *pass,
                                   uint32_t index_count,
                                   uint32_t instance_count,
                                   uint32_t first_index,
                                   int32_t base_vertex,
                                   uint32_t first_instance);

void wgpu_render_pass_draw_indexed_indirect(WGPURenderPass *pass,
                                            WGPUBufferId buffer_id,
                                            WGPUBufferAddress offset);

void wgpu_render_pass_draw_indirect(WGPURenderPass *pass,
                                    WGPUBufferId buffer_id,
                                    WGPUBufferAddress offset);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
void wgpu_render_pass_end_pass(WGPURenderPass *pass);

void wgpu_render_pass_insert_debug_marker(WGPURenderPass *pass,
                                          WGPURawString label,
                                          uint32_t color);

void wgpu_render_pass_multi_draw_indexed_indirect(WGPURenderPass *pass,
                                                  WGPUBufferId buffer_id,
                                                  WGPUBufferAddress offset,
                                                  uint32_t count);

void wgpu_render_pass_multi_draw_indexed_indirect_count(WGPURenderPass *pass,
                                                        WGPUBufferId buffer_id,
                                                        WGPUBufferAddress offset,
                                                        WGPUBufferId count_buffer_id,
                                                        WGPUBufferAddress count_buffer_offset,
                                                        uint32_t max_count);

void wgpu_render_pass_multi_draw_indirect(WGPURenderPass *pass,
                                          WGPUBufferId buffer_id,
                                          WGPUBufferAddress offset,
                                          uint32_t count);

void wgpu_render_pass_multi_draw_indirect_count(WGPURenderPass *pass,
                                                WGPUBufferId buffer_id,
                                                WGPUBufferAddress offset,
                                                WGPUBufferId count_buffer_id,
                                                WGPUBufferAddress count_buffer_offset,
                                                uint32_t max_count);

void wgpu_render_pass_pop_debug_group(WGPURenderPass *pass);

void wgpu_render_pass_push_debug_group(WGPURenderPass *pass, WGPURawString label, uint32_t color);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_render_pass_set_bind_group(WGPURenderPass *pass,
                                     uint32_t index,
                                     WGPUBindGroupId bind_group_id,
                                     const WGPUDynamicOffset *offsets,
                                     uintptr_t offset_length);

void wgpu_render_pass_set_blend_color(WGPURenderPass *pass, const WGPUColor *color);

void wgpu_render_pass_set_index_buffer(WGPURenderPass *pass,
                                       WGPUBufferId buffer_id,
                                       WGPUBufferAddress offset,
                                       WGPUOption_BufferSize size);

void wgpu_render_pass_set_pipeline(WGPURenderPass *pass, WGPURenderPipelineId pipeline_id);

void wgpu_render_pass_set_scissor_rect(WGPURenderPass *pass,
                                       uint32_t x,
                                       uint32_t y,
                                       uint32_t w,
                                       uint32_t h);

void wgpu_render_pass_set_stencil_reference(WGPURenderPass *pass, uint32_t value);

void wgpu_render_pass_set_vertex_buffer(WGPURenderPass *pass,
                                        uint32_t slot,
                                        WGPUBufferId buffer_id,
                                        WGPUBufferAddress offset,
                                        WGPUOption_BufferSize size);

void wgpu_render_pass_set_viewport(WGPURenderPass *pass,
                                   float x,
                                   float y,
                                   float w,
                                   float h,
                                   float depth_min,
                                   float depth_max);

void wgpu_render_pipeline_destroy(WGPURenderPipelineId render_pipeline_id);

/**
 * # Safety
 *
 * This function is unsafe as it calls an unsafe extern callback.
 */
void wgpu_request_adapter_async(const WGPURequestAdapterOptions *desc,
                                WGPUBackendBit mask,
                                bool allow_unsafe,
                                WGPURequestAdapterCallback callback,
                                void *userdata);

void wgpu_sampler_destroy(WGPUSamplerId sampler_id);

void wgpu_set_log_callback(WGPULogCallback callback);

int wgpu_set_log_level(WGPULogLevel level);

void wgpu_shader_module_destroy(WGPUShaderModuleId shader_module_id);

WGPUSwapChainOutput wgpu_swap_chain_get_next_texture(WGPUSwapChainId swap_chain_id);

void wgpu_swap_chain_present(WGPUSwapChainId swap_chain_id);

WGPUTextureViewId wgpu_texture_create_view(WGPUTextureId texture_id,
                                           const WGPUTextureViewDescriptor *desc);

void wgpu_texture_destroy(WGPUTextureId texture_id);

void wgpu_texture_view_destroy(WGPUTextureViewId texture_view_id);
