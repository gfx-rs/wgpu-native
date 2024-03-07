#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu.h"

typedef enum WGPUNativeSType {
    // Start at 0003 since that's allocated range for wgpu-native
    WGPUSType_DeviceExtras = 0x00030001,
    WGPUSType_RequiredLimitsExtras = 0x00030002,
    WGPUSType_PipelineLayoutExtras = 0x00030003,
    WGPUSType_ShaderModuleGLSLDescriptor = 0x00030004,
    WGPUSType_SupportedLimitsExtras = 0x00030005,
    WGPUSType_InstanceExtras = 0x00030006,
    WGPUSType_BindGroupEntryExtras = 0x00030007,
    WGPUSType_BindGroupLayoutEntryExtras = 0x00030008,
    WGPUSType_QuerySetDescriptorExtras = 0x00030009,
    WGPUSType_SurfaceConfigurationExtras = 0x0003000A,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

typedef enum WGPUNativeFeature {
    WGPUNativeFeature_PushConstants = 0x00030001,
    WGPUNativeFeature_TextureAdapterSpecificFormatFeatures = 0x00030002,
    WGPUNativeFeature_MultiDrawIndirect = 0x00030003,
    WGPUNativeFeature_MultiDrawIndirectCount = 0x00030004,
    WGPUNativeFeature_VertexWritableStorage = 0x00030005,
    WGPUNativeFeature_TextureBindingArray = 0x00030006,
    WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing = 0x00030007,
    WGPUNativeFeature_PipelineStatisticsQuery = 0x00030008,
    WGPUNativeFeature_StorageResourceBindingArray = 0x00030009,
    WGPUNativeFeature_PartiallyBoundBindingArray = 0x0003000A,
    WGPUNativeFeature_Force32 = 0x7FFFFFFF
} WGPUNativeFeature;

typedef enum WGPULogLevel {
    WGPULogLevel_Off = 0x00000000,
    WGPULogLevel_Error = 0x00000001,
    WGPULogLevel_Warn = 0x00000002,
    WGPULogLevel_Info = 0x00000003,
    WGPULogLevel_Debug = 0x00000004,
    WGPULogLevel_Trace = 0x00000005,
    WGPULogLevel_Force32 = 0x7FFFFFFF
} WGPULogLevel;

typedef enum WGPUInstanceBackend {
    WGPUInstanceBackend_All = 0x00000000,
    WGPUInstanceBackend_Vulkan = 1 << 0,
    WGPUInstanceBackend_GL = 1 << 1,
    WGPUInstanceBackend_Metal = 1 << 2,
    WGPUInstanceBackend_DX12 = 1 << 3,
    WGPUInstanceBackend_DX11 = 1 << 4,
    WGPUInstanceBackend_BrowserWebGPU = 1 << 5,
    WGPUInstanceBackend_Primary = WGPUInstanceBackend_Vulkan | WGPUInstanceBackend_Metal |
        WGPUInstanceBackend_DX12 |
        WGPUInstanceBackend_BrowserWebGPU,
    WGPUInstanceBackend_Secondary = WGPUInstanceBackend_GL | WGPUInstanceBackend_DX11,
    WGPUInstanceBackend_Force32 = 0x7FFFFFFF
} WGPUInstanceBackend;
typedef WGPUFlags WGPUInstanceBackendFlags;

typedef enum WGPUInstanceFlag {
    WGPUInstanceFlag_Default = 0x00000000,
    WGPUInstanceFlag_Debug = 1 << 0,
    WGPUInstanceFlag_Validation = 1 << 1,
    WGPUInstanceFlag_DiscardHalLabels = 1 << 2,
    WGPUInstanceFlag_Force32 = 0x7FFFFFFF
} WGPUInstanceFlag;
typedef WGPUFlags WGPUInstanceFlags;

typedef enum WGPUDx12Compiler {
    WGPUDx12Compiler_Undefined = 0x00000000,
    WGPUDx12Compiler_Fxc = 0x00000001,
    WGPUDx12Compiler_Dxc = 0x00000002,
    WGPUDx12Compiler_Force32 = 0x7FFFFFFF
} WGPUDx12Compiler;

typedef enum WGPUGles3MinorVersion {
    WGPUGles3MinorVersion_Automatic = 0x00000000,
    WGPUGles3MinorVersion_Version0 = 0x00000001,
    WGPUGles3MinorVersion_Version1 = 0x00000002,
    WGPUGles3MinorVersion_Version2 = 0x00000003,
    WGPUGles3MinorVersion_Force32 = 0x7FFFFFFF
} WGPUGles3MinorVersion;

typedef enum WGPUPipelineStatisticName {
    WGPUPipelineStatisticName_VertexShaderInvocations = 0x00000000,
    WGPUPipelineStatisticName_ClipperInvocations = 0x00000001,
    WGPUPipelineStatisticName_ClipperPrimitivesOut = 0x00000002,
    WGPUPipelineStatisticName_FragmentShaderInvocations = 0x00000003,
    WGPUPipelineStatisticName_ComputeShaderInvocations = 0x00000004,
    WGPUPipelineStatisticName_Force32 = 0x7FFFFFFF
} WGPUPipelineStatisticName WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUNativeQueryType {
    WGPUNativeQueryType_PipelineStatistics = 0x00030000,
    WGPUNativeQueryType_Force32 = 0x7FFFFFFF
} WGPUNativeQueryType WGPU_ENUM_ATTRIBUTE;

typedef struct WGPUInstanceExtras {
    WGPUChainedStruct chain;
    WGPUInstanceBackendFlags backends;
    WGPUInstanceFlags flags;
    WGPUDx12Compiler dx12ShaderCompiler;
    WGPUGles3MinorVersion gles3MinorVersion;
    const char * dxilPath;
    const char * dxcPath;
} WGPUInstanceExtras;

typedef struct WGPUDeviceExtras {
    WGPUChainedStruct chain;
    const char * tracePath;
} WGPUDeviceExtras;

typedef struct WGPUNativeLimits {
    uint32_t maxPushConstantSize;
    uint32_t maxNonSamplerBindings;
} WGPUNativeLimits;

typedef struct WGPURequiredLimitsExtras {
    WGPUChainedStruct chain;
    WGPUNativeLimits limits;
} WGPURequiredLimitsExtras;

typedef struct WGPUSupportedLimitsExtras {
    WGPUChainedStructOut chain;
    WGPUNativeLimits limits;
} WGPUSupportedLimitsExtras;

typedef struct WGPUPushConstantRange {
    WGPUShaderStageFlags stages;
    uint32_t start;
    uint32_t end;
} WGPUPushConstantRange;

typedef struct WGPUPipelineLayoutExtras {
    WGPUChainedStruct chain;
    size_t pushConstantRangeCount;
    WGPUPushConstantRange const * pushConstantRanges;
} WGPUPipelineLayoutExtras;

typedef uint64_t WGPUSubmissionIndex;

typedef struct WGPUWrappedSubmissionIndex {
    WGPUQueue queue;
    WGPUSubmissionIndex submissionIndex;
} WGPUWrappedSubmissionIndex;

typedef struct WGPUShaderDefine {
    char const * name;
    char const * value;
} WGPUShaderDefine;

typedef struct WGPUShaderModuleGLSLDescriptor {
    WGPUChainedStruct chain;
    WGPUShaderStage stage;
    char const * code;
    uint32_t defineCount;
    WGPUShaderDefine * defines;
} WGPUShaderModuleGLSLDescriptor;

typedef struct WGPURegistryReport {
   size_t numAllocated;
   size_t numKeptFromUser;
   size_t numReleasedFromUser;
   size_t numError;
   size_t elementSize;
} WGPURegistryReport;

typedef struct WGPUHubReport {
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
    WGPURegistryReport querySets;
    WGPURegistryReport buffers;
    WGPURegistryReport textures;
    WGPURegistryReport textureViews;
    WGPURegistryReport samplers;
} WGPUHubReport;

typedef struct WGPUGlobalReport {
    WGPURegistryReport surfaces;
    WGPUBackendType backendType;
    WGPUHubReport vulkan;
    WGPUHubReport metal;
    WGPUHubReport dx12;
    WGPUHubReport gl;
} WGPUGlobalReport;

typedef struct WGPUInstanceEnumerateAdapterOptions {
    WGPUChainedStruct const * nextInChain;
    WGPUInstanceBackendFlags backends;
} WGPUInstanceEnumerateAdapterOptions;

typedef struct WGPUBindGroupEntryExtras {
    WGPUChainedStruct chain;
    WGPUBuffer const * buffers;
    size_t bufferCount;
    WGPUSampler const * samplers;
    size_t samplerCount;
    WGPUTextureView const * textureViews;
    size_t textureViewCount;
} WGPUBindGroupEntryExtras;

typedef struct WGPUBindGroupLayoutEntryExtras {
    WGPUChainedStruct chain;
    uint32_t count;
} WGPUBindGroupLayoutEntryExtras;

typedef struct WGPUQuerySetDescriptorExtras {
    WGPUChainedStruct chain;
    WGPUPipelineStatisticName const * pipelineStatistics;
    size_t pipelineStatisticCount;
} WGPUQuerySetDescriptorExtras WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUSurfaceConfigurationExtras {
    WGPUChainedStruct chain;
    WGPUBool desiredMaximumFrameLatency;
} WGPUSurfaceConfigurationExtras WGPU_STRUCTURE_ATTRIBUTE;

typedef void (*WGPULogCallback)(WGPULogLevel level, char const * message, void * userdata);

#ifdef __cplusplus
extern "C" {
#endif

void wgpuGenerateReport(WGPUInstance instance, WGPUGlobalReport * report);
size_t wgpuInstanceEnumerateAdapters(WGPUInstance instance, WGPU_NULLABLE WGPUInstanceEnumerateAdapterOptions const * options, WGPUAdapter * adapters);

WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, size_t commandCount, WGPUCommandBuffer const * commands);

// Returns true if the queue is empty, or false if there are more queue submissions still in flight.
WGPUBool wgpuDevicePoll(WGPUDevice device, WGPUBool wait, WGPU_NULLABLE WGPUWrappedSubmissionIndex const * wrappedSubmissionIndex);

void wgpuSetLogCallback(WGPULogCallback callback, void * userdata);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

void wgpuRenderPassEncoderSetPushConstants(WGPURenderPassEncoder encoder, WGPUShaderStageFlags stages, uint32_t offset, uint32_t sizeBytes, void const * data);

void wgpuRenderPassEncoderMultiDrawIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);

void wgpuRenderPassEncoderMultiDrawIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);

void wgpuComputePassEncoderBeginPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);
void wgpuComputePassEncoderEndPipelineStatisticsQuery(WGPUComputePassEncoder computePassEncoder);
void wgpuRenderPassEncoderBeginPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex);
void wgpuRenderPassEncoderEndPipelineStatisticsQuery(WGPURenderPassEncoder renderPassEncoder);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
