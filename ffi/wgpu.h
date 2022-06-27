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
//      WGPUTextureFormat* formats = wgpuSurfaceGetSupportedFormats(surface, adapter, &count);
//      WGPU_FREE(WGPUTextureFormat, str, count); // notice `WGPUTextureFormat` instead of `WGPUTextureFormat *`
//
#define WGPU_FREE(type, ptr, len) wgpuFree(ptr, len * sizeof(type), _Alignof(type))

typedef enum WGPUNativeSType {
    // Start at 6 to prevent collisions with webgpu STypes
    WGPUSType_DeviceExtras = 0x60000001,
    WGPUSType_AdapterExtras = 0x60000002,
    WGPUSType_RequiredLimitsExtras = 0x60000003,
    WGPUSType_PipelineLayoutExtras = 0x60000004,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

typedef enum WGPUNativeFeature {
    WGPUNativeFeature_PUSH_CONSTANTS = 0x04000000,
    WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES = 0x10000000,
    WGPUNativeFeature_VERTEX_WRITABLE_STORAGE = 0x1000000000
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
    WGPUNativeFeature nativeFeatures;
    const char* label;
    const char* tracePath;
} WGPUDeviceExtras;

typedef struct WGPURequiredLimitsExtras {
    WGPUChainedStruct chain;
    uint32_t maxPushConstantSize;
} WGPURequiredLimitsExtras;

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

typedef void (*WGPULogCallback)(WGPULogLevel level, const char *msg);

#ifdef __cplusplus
extern "C" {
#endif

void wgpuGenerateReport(WGPUGlobalReport * report);

WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, uint32_t commandCount, WGPUCommandBuffer const * commands);

// Returns true if the queue is empty, or false if there are more queue submissions still in flight.
bool wgpuDevicePoll(WGPUDevice device, bool wait, WGPUWrappedSubmissionIndex const * wrappedSubmissionIndex);

void wgpuSetLogCallback(WGPULogCallback callback);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

// Returns slice of supported texture formats
// caller owns the formats slice and must WGPU_FREE() it
WGPUTextureFormat const * wgpuSurfaceGetSupportedFormats(WGPUSurface surface, WGPUAdapter adapter, size_t * count);

void wgpuRenderPassEncoderSetPushConstants(WGPURenderPassEncoder encoder, WGPUShaderStageFlags stages, uint32_t offset, uint32_t sizeBytes, void* const data);

void wgpuBufferDrop(WGPUBuffer buffer);
void wgpuCommandEncoderDrop(WGPUCommandEncoder commandEncoder);
void wgpuDeviceDrop(WGPUDevice device);
void wgpuQuerySetDrop(WGPUQuerySet querySet);
void wgpuRenderPipelineDrop(WGPURenderPipeline renderPipeline);
void wgpuTextureDrop(WGPUTexture texture);
void wgpuTextureViewDrop(WGPUTextureView textureView);
void wgpuSamplerDrop(WGPUSampler sampler);
void wgpuBindGroupLayoutDrop(WGPUBindGroupLayout bindGroupLayout);
void wgpuPipelineLayoutDrop(WGPUPipelineLayout pipelineLayout);
void wgpuBindGroupDrop(WGPUBindGroup bindGroup);
void wgpuShaderModuleDrop(WGPUShaderModule shaderModule);
void wgpuCommandBufferDrop(WGPUCommandBuffer commandBuffer);
void wgpuRenderBundleDrop(WGPURenderBundle renderBundle);
void wgpuComputePipelineDrop(WGPUComputePipeline computePipeline);

// must be used to free the strings & slices returned by the library,
// for other wgpu objects use appropriate drop functions.
void wgpuFree(void* ptr, size_t size, size_t align);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
