#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu-headers/webgpu.h"

// must be used to free the strings & slices returned by the library,
// for other wgpu objects use appropriate drop functions.
//
// first parameter `type` has to be type of derefrenced value
// for example ->
//
//      size_t count;
//      const WGPUTextureFormat* formats = wgpuSurfaceGetSupportedFormats(surface, adapter, &count);
//      WGPU_FREE(WGPUTextureFormat, formats, count); // notice `WGPUTextureFormat` instead of `WGPUTextureFormat *`
//
#define WGPU_FREE(type, ptr, len) wgpuFree((void *)ptr, len * sizeof(type), _Alignof(type))

typedef enum WGPUNativeSType {
    // Start at 6 to prevent collisions with webgpu STypes
    WGPUSType_DeviceExtras = 0x60000001,
    WGPUSType_AdapterExtras = 0x60000002,
    WGPUSType_RequiredLimitsExtras = 0x60000003,
    WGPUSType_PipelineLayoutExtras = 0x60000004,
    WGPUSType_ShaderModuleGLSLDescriptor = 0x60000005,
    WGPUSType_SupportedLimitsExtras = 0x60000003,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

typedef enum WGPUNativeFeature {
    WGPUNativeFeature_PUSH_CONSTANTS = 0x60000001,
    WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES = 0x60000002,
    WGPUNativeFeature_MULTI_DRAW_INDIRECT = 0x60000003,
    WGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT = 0x60000004,
    WGPUNativeFeature_VERTEX_WRITABLE_STORAGE = 0x60000005,
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

typedef struct WGPUAdapterExtras {
    WGPUChainedStruct chain;
    WGPUBackendType backend;
} WGPUAdapterExtras;

typedef struct WGPUDeviceExtras {
    WGPUChainedStruct chain;
    const char* tracePath;
} WGPUDeviceExtras;

typedef struct WGPURequiredLimitsExtras {
    WGPUChainedStruct chain;
    uint32_t maxPushConstantSize;
    uint64_t maxBufferSize;
} WGPURequiredLimitsExtras;

typedef struct WGPUSupportedLimitsExtras {
    WGPUChainedStructOut chain;
    uint32_t maxPushConstantSize;
    uint64_t maxBufferSize;
} WGPUSupportedLimitsExtras;

typedef struct WGPUPushConstantRange {
    WGPUShaderStageFlags stages;
    uint32_t start;
    uint32_t end;
} WGPUPushConstantRange;

typedef struct WGPUPipelineLayoutExtras {
    WGPUChainedStruct chain;
    uint32_t pushConstantRangeCount;
    WGPUPushConstantRange* pushConstantRanges;
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
    WGPUShaderDefine* defines;
} WGPUShaderModuleGLSLDescriptor;

typedef struct WGPUStorageReport {
    size_t numOccupied;
    size_t numVacant;
    size_t numError;
    size_t elementSize;
} WGPUStorageReport;

typedef struct WGPUHubReport {
    WGPUStorageReport adapters;
    WGPUStorageReport devices;
    WGPUStorageReport pipelineLayouts;
    WGPUStorageReport shaderModules;
    WGPUStorageReport bindGroupLayouts;
    WGPUStorageReport bindGroups;
    WGPUStorageReport commandBuffers;
    WGPUStorageReport renderBundles;
    WGPUStorageReport renderPipelines;
    WGPUStorageReport computePipelines;
    WGPUStorageReport querySets;
    WGPUStorageReport buffers;
    WGPUStorageReport textures;
    WGPUStorageReport textureViews;
    WGPUStorageReport samplers;
} WGPUHubReport;

typedef struct WGPUGlobalReport {
    WGPUStorageReport surfaces;
    WGPUBackendType backendType;
    WGPUHubReport vulkan;
    WGPUHubReport metal;
    WGPUHubReport dx12;
    WGPUHubReport dx11;
    WGPUHubReport gl;
} WGPUGlobalReport;

typedef void (*WGPULogCallback)(WGPULogLevel level, char const * message, void * userdata);

#ifdef __cplusplus
extern "C" {
#endif

void wgpuGenerateReport(WGPUInstance instance, WGPUGlobalReport* report);

WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, uint32_t commandCount, WGPUCommandBuffer const * commands);

// Returns true if the queue is empty, or false if there are more queue submissions still in flight.
bool wgpuDevicePoll(WGPUDevice device, bool wait, WGPUWrappedSubmissionIndex const * wrappedSubmissionIndex);

void wgpuSetLogCallback(WGPULogCallback callback, void * userdata);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

// Returns slice of supported texture formats
// caller owns the formats slice and must WGPU_FREE() it
WGPUTextureFormat const * wgpuSurfaceGetSupportedFormats(WGPUSurface surface, WGPUAdapter adapter, size_t * count);

// Returns slice of supported present modes
// caller owns the present modes slice and must WGPU_FREE() it
WGPUPresentMode const * wgpuSurfaceGetSupportedPresentModes(WGPUSurface surface, WGPUAdapter adapter, size_t * count);

void wgpuRenderPassEncoderSetPushConstants(WGPURenderPassEncoder encoder, WGPUShaderStageFlags stages, uint32_t offset, uint32_t sizeBytes, void* const data);

void wgpuRenderPassEncoderMultiDrawIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);

void wgpuRenderPassEncoderMultiDrawIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);

void wgpuInstanceDrop(WGPUInstance instance);
void wgpuAdapterDrop(WGPUAdapter adapter);
void wgpuBindGroupDrop(WGPUBindGroup bindGroup);
void wgpuBindGroupLayoutDrop(WGPUBindGroupLayout bindGroupLayout);
void wgpuBufferDrop(WGPUBuffer buffer);
void wgpuCommandBufferDrop(WGPUCommandBuffer commandBuffer);
void wgpuCommandEncoderDrop(WGPUCommandEncoder commandEncoder);
void wgpuComputePipelineDrop(WGPUComputePipeline computePipeline);
void wgpuDeviceDrop(WGPUDevice device);
void wgpuPipelineLayoutDrop(WGPUPipelineLayout pipelineLayout);
void wgpuQuerySetDrop(WGPUQuerySet querySet);
void wgpuRenderBundleDrop(WGPURenderBundle renderBundle);
void wgpuRenderPipelineDrop(WGPURenderPipeline renderPipeline);
void wgpuSamplerDrop(WGPUSampler sampler);
void wgpuShaderModuleDrop(WGPUShaderModule shaderModule);
void wgpuSurfaceDrop(WGPUSurface surface);
void wgpuSwapChainDrop(WGPUSwapChain swapChain);
void wgpuTextureDrop(WGPUTexture texture);
void wgpuTextureViewDrop(WGPUTextureView textureView);

// must be used to free the strings & slices returned by the library,
// for other wgpu objects use appropriate drop functions.
void wgpuFree(void* ptr, size_t size, size_t align);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
