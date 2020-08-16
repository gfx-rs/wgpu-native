/* Generated with cbindgen:0.14.3 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen toolkit/library/rust/ --lockfile Cargo.lock --crate wgpu-remote -o dom/webgpu/ffi/wgpu_ffi_generated.h`
 */

typedef unsigned long long WGPUNonZeroU64;
typedef unsigned long long WGPUOption_AdapterId;
typedef unsigned long long WGPUOption_SurfaceId;
typedef unsigned long long WGPUOption_TextureViewId;

typedef struct WGPUChainedStruct WGPUChainedStruct;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Bound uniform/storage buffer offsets must be aligned to this number.
 */
#define WGPUBIND_BUFFER_ALIGNMENT 256

/**
 * Buffer-Texture copies on command encoders have to have the `bytes_per_row`
 * aligned to this number.
 *
 * This doesn't apply to `Queue::write_texture`.
 */
#define WGPUCOPY_BYTES_PER_ROW_ALIGNMENT 256

#define WGPUDEFAULT_BIND_GROUPS 4

#define WGPUDESIRED_NUM_FRAMES 3

#define WGPUMAX_ANISOTROPY 16

#define WGPUMAX_COLOR_TARGETS 4

#define WGPUMAX_MIP_LEVELS 16

#define WGPUMAX_VERTEX_BUFFERS 16

typedef enum WGPUAddressMode {
  WGPUAddressMode_ClampToEdge = 0,
  WGPUAddressMode_Repeat = 1,
  WGPUAddressMode_MirrorRepeat = 2,
} WGPUAddressMode;

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

typedef enum WGPUBindingType {
  WGPUBindingType_UniformBuffer = 0,
  WGPUBindingType_StorageBuffer = 1,
  WGPUBindingType_ReadonlyStorageBuffer = 2,
  WGPUBindingType_Sampler = 3,
  WGPUBindingType_ComparisonSampler = 4,
  WGPUBindingType_SampledTexture = 5,
  WGPUBindingType_ReadonlyStorageTexture = 6,
  WGPUBindingType_WriteonlyStorageTexture = 7,
} WGPUBindingType;

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

typedef enum WGPUCompareFunction {
  WGPUCompareFunction_Undefined = 0,
  WGPUCompareFunction_Never = 1,
  WGPUCompareFunction_Less = 2,
  WGPUCompareFunction_Equal = 3,
  WGPUCompareFunction_LessEqual = 4,
  WGPUCompareFunction_Greater = 5,
  WGPUCompareFunction_NotEqual = 6,
  WGPUCompareFunction_GreaterEqual = 7,
  WGPUCompareFunction_Always = 8,
} WGPUCompareFunction;

typedef enum WGPUCullMode {
  WGPUCullMode_None = 0,
  WGPUCullMode_Front = 1,
  WGPUCullMode_Back = 2,
} WGPUCullMode;

typedef enum WGPUFilterMode {
  WGPUFilterMode_Nearest = 0,
  WGPUFilterMode_Linear = 1,
} WGPUFilterMode;

typedef enum WGPUFrontFace {
  WGPUFrontFace_Ccw = 0,
  WGPUFrontFace_Cw = 1,
} WGPUFrontFace;

typedef enum WGPUIndexFormat {
  WGPUIndexFormat_Uint16 = 0,
  WGPUIndexFormat_Uint32 = 1,
} WGPUIndexFormat;

typedef enum WGPUInputStepMode {
  WGPUInputStepMode_Vertex = 0,
  WGPUInputStepMode_Instance = 1,
} WGPUInputStepMode;

typedef enum WGPULoadOp {
  WGPULoadOp_Clear = 0,
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

typedef enum WGPUPowerPreference {
  WGPUPowerPreference_Default = 0,
  WGPUPowerPreference_LowPower = 1,
  WGPUPowerPreference_HighPerformance = 2,
} WGPUPowerPreference;

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

typedef enum WGPUPrimitiveTopology {
  WGPUPrimitiveTopology_PointList = 0,
  WGPUPrimitiveTopology_LineList = 1,
  WGPUPrimitiveTopology_LineStrip = 2,
  WGPUPrimitiveTopology_TriangleList = 3,
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

typedef enum WGPUStencilOperation {
  WGPUStencilOperation_Keep = 0,
  WGPUStencilOperation_Zero = 1,
  WGPUStencilOperation_Replace = 2,
  WGPUStencilOperation_Invert = 3,
  WGPUStencilOperation_IncrementClamp = 4,
  WGPUStencilOperation_DecrementClamp = 5,
  WGPUStencilOperation_IncrementWrap = 6,
  WGPUStencilOperation_DecrementWrap = 7,
} WGPUStencilOperation;

typedef enum WGPUStoreOp {
  WGPUStoreOp_Clear = 0,
  WGPUStoreOp_Store = 1,
} WGPUStoreOp;

typedef enum WGPUSwapChainStatus {
  WGPUSwapChainStatus_Good,
  WGPUSwapChainStatus_Suboptimal,
  WGPUSwapChainStatus_Timeout,
  WGPUSwapChainStatus_Outdated,
  WGPUSwapChainStatus_Lost,
  WGPUSwapChainStatus_OutOfMemory,
} WGPUSwapChainStatus;

typedef enum WGPUTextureAspect {
  WGPUTextureAspect_All,
  WGPUTextureAspect_StencilOnly,
  WGPUTextureAspect_DepthOnly,
} WGPUTextureAspect;

typedef enum WGPUTextureComponentType {
  WGPUTextureComponentType_Float,
  WGPUTextureComponentType_Sint,
  WGPUTextureComponentType_Uint,
} WGPUTextureComponentType;

typedef enum WGPUTextureDimension {
  WGPUTextureDimension_D1,
  WGPUTextureDimension_D2,
  WGPUTextureDimension_D3,
} WGPUTextureDimension;

typedef enum WGPUTextureFormat {
  WGPUTextureFormat_R8Unorm = 0,
  WGPUTextureFormat_R8Snorm = 1,
  WGPUTextureFormat_R8Uint = 2,
  WGPUTextureFormat_R8Sint = 3,
  WGPUTextureFormat_R16Uint = 4,
  WGPUTextureFormat_R16Sint = 5,
  WGPUTextureFormat_R16Float = 6,
  WGPUTextureFormat_Rg8Unorm = 7,
  WGPUTextureFormat_Rg8Snorm = 8,
  WGPUTextureFormat_Rg8Uint = 9,
  WGPUTextureFormat_Rg8Sint = 10,
  WGPUTextureFormat_R32Uint = 11,
  WGPUTextureFormat_R32Sint = 12,
  WGPUTextureFormat_R32Float = 13,
  WGPUTextureFormat_Rg16Uint = 14,
  WGPUTextureFormat_Rg16Sint = 15,
  WGPUTextureFormat_Rg16Float = 16,
  WGPUTextureFormat_Rgba8Unorm = 17,
  WGPUTextureFormat_Rgba8UnormSrgb = 18,
  WGPUTextureFormat_Rgba8Snorm = 19,
  WGPUTextureFormat_Rgba8Uint = 20,
  WGPUTextureFormat_Rgba8Sint = 21,
  WGPUTextureFormat_Bgra8Unorm = 22,
  WGPUTextureFormat_Bgra8UnormSrgb = 23,
  WGPUTextureFormat_Rgb10a2Unorm = 24,
  WGPUTextureFormat_Rg11b10Float = 25,
  WGPUTextureFormat_Rg32Uint = 26,
  WGPUTextureFormat_Rg32Sint = 27,
  WGPUTextureFormat_Rg32Float = 28,
  WGPUTextureFormat_Rgba16Uint = 29,
  WGPUTextureFormat_Rgba16Sint = 30,
  WGPUTextureFormat_Rgba16Float = 31,
  WGPUTextureFormat_Rgba32Uint = 32,
  WGPUTextureFormat_Rgba32Sint = 33,
  WGPUTextureFormat_Rgba32Float = 34,
  WGPUTextureFormat_Depth32Float = 35,
  WGPUTextureFormat_Depth24Plus = 36,
  WGPUTextureFormat_Depth24PlusStencil8 = 37,
} WGPUTextureFormat;

typedef enum WGPUTextureViewDimension {
  WGPUTextureViewDimension_D1,
  WGPUTextureViewDimension_D2,
  WGPUTextureViewDimension_D2Array,
  WGPUTextureViewDimension_Cube,
  WGPUTextureViewDimension_CubeArray,
  WGPUTextureViewDimension_D3,
} WGPUTextureViewDimension;

typedef enum WGPUVertexFormat {
  WGPUVertexFormat_Uchar2 = 0,
  WGPUVertexFormat_Uchar4 = 1,
  WGPUVertexFormat_Char2 = 2,
  WGPUVertexFormat_Char4 = 3,
  WGPUVertexFormat_Uchar2Norm = 4,
  WGPUVertexFormat_Uchar4Norm = 5,
  WGPUVertexFormat_Char2Norm = 6,
  WGPUVertexFormat_Char4Norm = 7,
  WGPUVertexFormat_Ushort2 = 8,
  WGPUVertexFormat_Ushort4 = 9,
  WGPUVertexFormat_Short2 = 10,
  WGPUVertexFormat_Short4 = 11,
  WGPUVertexFormat_Ushort2Norm = 12,
  WGPUVertexFormat_Ushort4Norm = 13,
  WGPUVertexFormat_Short2Norm = 14,
  WGPUVertexFormat_Short4Norm = 15,
  WGPUVertexFormat_Half2 = 16,
  WGPUVertexFormat_Half4 = 17,
  WGPUVertexFormat_Float = 18,
  WGPUVertexFormat_Float2 = 19,
  WGPUVertexFormat_Float3 = 20,
  WGPUVertexFormat_Float4 = 21,
  WGPUVertexFormat_Uint = 22,
  WGPUVertexFormat_Uint2 = 23,
  WGPUVertexFormat_Uint3 = 24,
  WGPUVertexFormat_Uint4 = 25,
  WGPUVertexFormat_Int = 26,
  WGPUVertexFormat_Int2 = 27,
  WGPUVertexFormat_Int3 = 28,
  WGPUVertexFormat_Int4 = 29,
} WGPUVertexFormat;

typedef WGPUNonZeroU64 WGPUId_Adapter_Dummy;

typedef WGPUId_Adapter_Dummy WGPUAdapterId;

typedef uint64_t WGPUExtensions;
/**
 * This is a native only extension. Support is planned to be added to webgpu,
 * but it is not yet implemented.
 *
 * https://github.com/gpuweb/gpuweb/issues/696
 */
#define WGPUExtensions_ANISOTROPIC_FILTERING (uint64_t)65536
/**
 * Extensions which are part of the upstream webgpu standard
 */
#define WGPUExtensions_ALL_WEBGPU (uint64_t)65535
/**
 * Extensions that require activating the unsafe extension flag
 */
#define WGPUExtensions_ALL_UNSAFE (uint64_t)18446462598732840960ULL
/**
 * Extensions that are only available when targeting native (not web)
 */
#define WGPUExtensions_ALL_NATIVE (uint64_t)18446744073709486080ULL

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

typedef uint64_t WGPUBufferAddress;

typedef uint64_t WGPUBufferSize;

typedef void (*WGPUBufferMapCallback)(WGPUBufferMapAsyncStatus status, uint8_t *userdata);

typedef WGPUNonZeroU64 WGPUId_CommandBuffer_Dummy;

typedef WGPUId_CommandBuffer_Dummy WGPUCommandBufferId;

typedef WGPUCommandBufferId WGPUCommandEncoderId;

typedef struct WGPURawPass {
  uint8_t *data;
  uint8_t *base;
  uintptr_t capacity;
  WGPUCommandEncoderId parent;
} WGPURawPass;

typedef struct WGPUComputePassDescriptor {
  uint32_t todo;
} WGPUComputePassDescriptor;

typedef WGPUNonZeroU64 WGPUId_TextureView_Dummy;

typedef WGPUId_TextureView_Dummy WGPUTextureViewId;

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

typedef struct WGPURenderPassColorAttachmentDescriptorBase_TextureViewId {
  WGPUTextureViewId attachment;
  WGPUOption_TextureViewId resolve_target;
  WGPULoadOp load_op;
  WGPUStoreOp store_op;
  WGPUColor clear_color;
} WGPURenderPassColorAttachmentDescriptorBase_TextureViewId;

typedef WGPURenderPassColorAttachmentDescriptorBase_TextureViewId WGPURenderPassColorAttachmentDescriptor;

typedef struct WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId {
  WGPUTextureViewId attachment;
  WGPULoadOp depth_load_op;
  WGPUStoreOp depth_store_op;
  float clear_depth;
  bool depth_read_only;
  WGPULoadOp stencil_load_op;
  WGPUStoreOp stencil_store_op;
  uint32_t clear_stencil;
  bool stencil_read_only;
} WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId;

typedef WGPURenderPassDepthStencilAttachmentDescriptorBase_TextureViewId WGPURenderPassDepthStencilAttachmentDescriptor;

typedef struct WGPURenderPassDescriptor {
  const WGPURenderPassColorAttachmentDescriptor *color_attachments;
  uintptr_t color_attachments_length;
  const WGPURenderPassDepthStencilAttachmentDescriptor *depth_stencil_attachment;
} WGPURenderPassDescriptor;

typedef struct WGPUTextureDataLayout {
  WGPUBufferAddress offset;
  uint32_t bytes_per_row;
  uint32_t rows_per_image;
} WGPUTextureDataLayout;

typedef struct WGPUBufferCopyView {
  WGPUBufferId buffer;
  WGPUTextureDataLayout layout;
} WGPUBufferCopyView;

typedef WGPUNonZeroU64 WGPUId_Texture_Dummy;

typedef WGPUId_Texture_Dummy WGPUTextureId;

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

typedef struct WGPUExtent3d {
  uint32_t width;
  uint32_t height;
  uint32_t depth;
} WGPUExtent3d;

typedef struct WGPUCommandBufferDescriptor {
  uint32_t todo;
} WGPUCommandBufferDescriptor;

typedef WGPURawPass *WGPUComputePassId;

typedef const char *WGPURawString;

typedef uint32_t WGPUDynamicOffset;

typedef WGPUNonZeroU64 WGPUId_ComputePipeline_Dummy;

typedef WGPUId_ComputePipeline_Dummy WGPUComputePipelineId;

typedef WGPUNonZeroU64 WGPUId_Surface;

typedef WGPUId_Surface WGPUSurfaceId;

typedef struct WGPUBufferBinding {
  WGPUBufferId buffer;
  WGPUBufferAddress offset;
  WGPUBufferSize size;
} WGPUBufferBinding;

typedef WGPUNonZeroU64 WGPUId_Sampler_Dummy;

typedef WGPUId_Sampler_Dummy WGPUSamplerId;

typedef enum WGPUBindingResource_Tag {
  WGPUBindingResource_Buffer,
  WGPUBindingResource_Sampler,
  WGPUBindingResource_TextureView,
} WGPUBindingResource_Tag;

typedef struct WGPUBindingResource_WGPUBuffer_Body {
  WGPUBufferBinding _0;
} WGPUBindingResource_WGPUBuffer_Body;

typedef struct WGPUBindingResource_WGPUSampler_Body {
  WGPUSamplerId _0;
} WGPUBindingResource_WGPUSampler_Body;

typedef struct WGPUBindingResource_WGPUTextureView_Body {
  WGPUTextureViewId _0;
} WGPUBindingResource_WGPUTextureView_Body;

typedef struct WGPUBindingResource {
  WGPUBindingResource_Tag tag;
  union {
    WGPUBindingResource_WGPUBuffer_Body buffer;
    WGPUBindingResource_WGPUSampler_Body sampler;
    WGPUBindingResource_WGPUTextureView_Body texture_view;
  };
} WGPUBindingResource;

typedef struct WGPUBindGroupEntry {
  uint32_t binding;
  WGPUBindingResource resource;
} WGPUBindGroupEntry;

typedef struct WGPUBindGroupDescriptor {
  const char *label;
  WGPUBindGroupLayoutId layout;
  const WGPUBindGroupEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupDescriptor;

typedef uint32_t WGPUShaderStage;
#define WGPUShaderStage_NONE (uint32_t)0
#define WGPUShaderStage_VERTEX (uint32_t)1
#define WGPUShaderStage_FRAGMENT (uint32_t)2
#define WGPUShaderStage_COMPUTE (uint32_t)4

typedef struct WGPUBindGroupLayoutEntry {
  uint32_t binding;
  WGPUShaderStage visibility;
  WGPUBindingType ty;
  bool multisampled;
  bool has_dynamic_offset;
  WGPUTextureViewDimension view_dimension;
  WGPUTextureComponentType texture_component_type;
  WGPUTextureFormat storage_texture_format;
} WGPUBindGroupLayoutEntry;

typedef struct WGPUBindGroupLayoutDescriptor {
  const char *label;
  const WGPUBindGroupLayoutEntry *entries;
  uintptr_t entries_length;
} WGPUBindGroupLayoutDescriptor;

typedef const char *WGPULabel;

typedef uint32_t WGPUBufferUsage;
#define WGPUBufferUsage_MAP_READ (uint32_t)1
#define WGPUBufferUsage_MAP_WRITE (uint32_t)2
#define WGPUBufferUsage_COPY_SRC (uint32_t)4
#define WGPUBufferUsage_COPY_DST (uint32_t)8
#define WGPUBufferUsage_INDEX (uint32_t)16
#define WGPUBufferUsage_VERTEX (uint32_t)32
#define WGPUBufferUsage_UNIFORM (uint32_t)64
#define WGPUBufferUsage_STORAGE (uint32_t)128
#define WGPUBufferUsage_INDIRECT (uint32_t)256

typedef struct WGPUBufferDescriptor {
  WGPULabel label;
  WGPUBufferAddress size;
  WGPUBufferUsage usage;
  bool mapped_at_creation;
} WGPUBufferDescriptor;

typedef struct WGPUCommandEncoderDescriptor {
  const char *label;
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

typedef WGPUNonZeroU64 WGPUId_RenderPipeline_Dummy;

typedef WGPUId_RenderPipeline_Dummy WGPURenderPipelineId;

typedef struct WGPURasterizationStateDescriptor {
  WGPUFrontFace front_face;
  WGPUCullMode cull_mode;
  int32_t depth_bias;
  float depth_bias_slope_scale;
  float depth_bias_clamp;
} WGPURasterizationStateDescriptor;

typedef struct WGPUBlendDescriptor {
  WGPUBlendFactor src_factor;
  WGPUBlendFactor dst_factor;
  WGPUBlendOperation operation;
} WGPUBlendDescriptor;

typedef uint32_t WGPUColorWrite;
#define WGPUColorWrite_RED (uint32_t)1
#define WGPUColorWrite_GREEN (uint32_t)2
#define WGPUColorWrite_BLUE (uint32_t)4
#define WGPUColorWrite_ALPHA (uint32_t)8
#define WGPUColorWrite_COLOR (uint32_t)7
#define WGPUColorWrite_ALL (uint32_t)15

typedef struct WGPUColorStateDescriptor {
  WGPUTextureFormat format;
  WGPUBlendDescriptor alpha_blend;
  WGPUBlendDescriptor color_blend;
  WGPUColorWrite write_mask;
} WGPUColorStateDescriptor;

typedef struct WGPUStencilStateFaceDescriptor {
  WGPUCompareFunction compare;
  WGPUStencilOperation fail_op;
  WGPUStencilOperation depth_fail_op;
  WGPUStencilOperation pass_op;
} WGPUStencilStateFaceDescriptor;

typedef struct WGPUDepthStencilStateDescriptor {
  WGPUTextureFormat format;
  bool depth_write_enabled;
  WGPUCompareFunction depth_compare;
  WGPUStencilStateFaceDescriptor stencil_front;
  WGPUStencilStateFaceDescriptor stencil_back;
  uint32_t stencil_read_mask;
  uint32_t stencil_write_mask;
} WGPUDepthStencilStateDescriptor;

typedef uint32_t WGPUShaderLocation;

typedef struct WGPUVertexAttributeDescriptor {
  WGPUBufferAddress offset;
  WGPUVertexFormat format;
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

typedef struct WGPUU32Array {
  const uint32_t *bytes;
  uintptr_t length;
} WGPUU32Array;

typedef struct WGPUShaderModuleDescriptor {
  WGPUU32Array code;
} WGPUShaderModuleDescriptor;

typedef WGPUNonZeroU64 WGPUId_SwapChain_Dummy;

typedef WGPUId_SwapChain_Dummy WGPUSwapChainId;

typedef uint32_t WGPUTextureUsage;
#define WGPUTextureUsage_COPY_SRC (uint32_t)1
#define WGPUTextureUsage_COPY_DST (uint32_t)2
#define WGPUTextureUsage_SAMPLED (uint32_t)4
#define WGPUTextureUsage_STORAGE (uint32_t)8
#define WGPUTextureUsage_OUTPUT_ATTACHMENT (uint32_t)16

typedef struct WGPUSwapChainDescriptor {
  WGPUTextureUsage usage;
  WGPUTextureFormat format;
  uint32_t width;
  uint32_t height;
  WGPUPresentMode present_mode;
} WGPUSwapChainDescriptor;

typedef struct WGPUTextureDescriptor {
  WGPULabel label;
  WGPUExtent3d size;
  uint32_t mip_level_count;
  uint32_t sample_count;
  WGPUTextureDimension dimension;
  WGPUTextureFormat format;
  WGPUTextureUsage usage;
} WGPUTextureDescriptor;

typedef WGPUDeviceId WGPUQueueId;

typedef WGPURawPass *WGPURenderPassId;

typedef WGPUNonZeroU64 WGPUId_RenderBundle_Dummy;

typedef WGPUId_RenderBundle_Dummy WGPURenderBundleId;

typedef struct WGPURequestAdapterOptions {
  WGPUPowerPreference power_preference;
  WGPUOption_SurfaceId compatible_surface;
} WGPURequestAdapterOptions;

typedef uint32_t WGPUBackendBit;

typedef void (*WGPURequestAdapterCallback)(WGPUOption_AdapterId id, void *userdata);

typedef void (*WGPULogCallback)(int level, const char *msg);

typedef struct WGPUSwapChainOutput {
  WGPUSwapChainStatus status;
  WGPUOption_TextureViewId view_id;
} WGPUSwapChainOutput;

typedef struct WGPUTextureViewDescriptor {
  WGPULabel label;
  WGPUTextureFormat format;
  WGPUTextureViewDimension dimension;
  WGPUTextureAspect aspect;
  uint32_t base_mip_level;
  uint32_t level_count;
  uint32_t base_array_layer;
  uint32_t array_layer_count;
} WGPUTextureViewDescriptor;

typedef struct WGPUAnisotropicSamplerDescriptorExt {
  const WGPUChainedStruct *next_in_chain;
  WGPUSType s_type;
  uint8_t anisotropic_clamp;
} WGPUAnisotropicSamplerDescriptorExt;































void wgpu_adapter_destroy(WGPUAdapterId adapter_id);

WGPUExtensions wgpu_adapter_extensions(WGPUAdapterId adapter_id);

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
                                         WGPUExtensions extensions,
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
WGPURawPass *wgpu_command_encoder_begin_compute_pass(WGPUCommandEncoderId encoder_id,
                                                     const WGPUComputePassDescriptor *_desc);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
WGPURawPass *wgpu_command_encoder_begin_render_pass(WGPUCommandEncoderId encoder_id,
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

void wgpu_compute_pass_destroy(WGPURawPass *pass);

void wgpu_compute_pass_dispatch(WGPURawPass *pass,
                                uint32_t groups_x,
                                uint32_t groups_y,
                                uint32_t groups_z);

void wgpu_compute_pass_dispatch_indirect(WGPURawPass *pass,
                                         WGPUBufferId buffer_id,
                                         WGPUBufferAddress offset);

void wgpu_compute_pass_end_pass(WGPUComputePassId pass_id);

void wgpu_compute_pass_insert_debug_marker(WGPURawPass *_pass, WGPURawString _label);

void wgpu_compute_pass_pop_debug_group(WGPURawPass *_pass);

void wgpu_compute_pass_push_debug_group(WGPURawPass *_pass, WGPURawString _label);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_compute_pass_set_bind_group(WGPURawPass *pass,
                                      uint32_t index,
                                      WGPUBindGroupId bind_group_id,
                                      const WGPUDynamicOffset *offsets,
                                      uintptr_t offset_length);

void wgpu_compute_pass_set_pipeline(WGPURawPass *pass, WGPUComputePipelineId pipeline_id);

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

WGPURenderPipelineId wgpu_device_create_render_pipeline(WGPUDeviceId device_id,
                                                        const WGPURenderPipelineDescriptor *desc);

WGPUSamplerId wgpu_device_create_sampler(WGPUDeviceId device_id, const WGPUSamplerDescriptor *desc);

WGPUShaderModuleId wgpu_device_create_shader_module(WGPUDeviceId device_id,
                                                    const WGPUShaderModuleDescriptor *desc);

WGPUSwapChainId wgpu_device_create_swap_chain(WGPUDeviceId device_id,
                                              WGPUSurfaceId surface_id,
                                              const WGPUSwapChainDescriptor *desc);

WGPUTextureId wgpu_device_create_texture(WGPUDeviceId device_id, const WGPUTextureDescriptor *desc);

void wgpu_device_destroy(WGPUDeviceId device_id);

WGPUExtensions wgpu_device_extensions(WGPUDeviceId device_id);

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

void wgpu_render_pass_destroy(WGPURawPass *pass);

void wgpu_render_pass_draw(WGPURawPass *pass,
                           uint32_t vertex_count,
                           uint32_t instance_count,
                           uint32_t first_vertex,
                           uint32_t first_instance);

void wgpu_render_pass_draw_indexed(WGPURawPass *pass,
                                   uint32_t index_count,
                                   uint32_t instance_count,
                                   uint32_t first_index,
                                   int32_t base_vertex,
                                   uint32_t first_instance);

void wgpu_render_pass_draw_indexed_indirect(WGPURawPass *pass,
                                            WGPUBufferId buffer_id,
                                            WGPUBufferAddress offset);

void wgpu_render_pass_draw_indirect(WGPURawPass *pass,
                                    WGPUBufferId buffer_id,
                                    WGPUBufferAddress offset);

/**
 * # Safety
 *
 * This function is unsafe because improper use may lead to memory
 * problems. For example, a double-free may occur if the function is called
 * twice on the same raw pointer.
 */
void wgpu_render_pass_end_pass(WGPURenderPassId pass_id);

void wgpu_render_pass_execute_bundles(WGPURawPass *_pass,
                                      const WGPURenderBundleId *_bundles,
                                      uintptr_t _bundles_length);

void wgpu_render_pass_insert_debug_marker(WGPURawPass *_pass, WGPURawString _label);

void wgpu_render_pass_pop_debug_group(WGPURawPass *_pass);

void wgpu_render_pass_push_debug_group(WGPURawPass *_pass, WGPURawString _label);

/**
 * # Safety
 *
 * This function is unsafe as there is no guarantee that the given pointer is
 * valid for `offset_length` elements.
 */
void wgpu_render_pass_set_bind_group(WGPURawPass *pass,
                                     uint32_t index,
                                     WGPUBindGroupId bind_group_id,
                                     const WGPUDynamicOffset *offsets,
                                     uintptr_t offset_length);

void wgpu_render_pass_set_blend_color(WGPURawPass *pass, const WGPUColor *color);

void wgpu_render_pass_set_index_buffer(WGPURawPass *pass,
                                       WGPUBufferId buffer_id,
                                       WGPUBufferAddress offset,
                                       WGPUBufferSize size);

void wgpu_render_pass_set_pipeline(WGPURawPass *pass, WGPURenderPipelineId pipeline_id);

void wgpu_render_pass_set_scissor_rect(WGPURawPass *pass,
                                       uint32_t x,
                                       uint32_t y,
                                       uint32_t w,
                                       uint32_t h);

void wgpu_render_pass_set_stencil_reference(WGPURawPass *pass, uint32_t value);

void wgpu_render_pass_set_vertex_buffer(WGPURawPass *pass,
                                        uint32_t slot,
                                        WGPUBufferId buffer_id,
                                        WGPUBufferAddress offset,
                                        WGPUBufferSize size);

void wgpu_render_pass_set_viewport(WGPURawPass *pass,
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
