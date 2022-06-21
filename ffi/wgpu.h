#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu-headers/webgpu.h"

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
    WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES = 0x10000000
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

typedef void (*WGPULogCallback)(WGPULogLevel level, const char *msg);

#ifdef __cplusplus
extern "C" {
#endif

WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, uint32_t commandCount, WGPUCommandBuffer const * commands);

// Returns true if the queue is empty, or false if there are more queue submissions still in flight.
bool wgpuDevicePoll(WGPUDevice device, bool wait, WGPUWrappedSubmissionIndex const * wrappedSubmissionIndex);

void wgpuSetLogCallback(WGPULogCallback callback);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

// Returns resource usage C string; caller owns the string and must free() it
char* wgpuGetResourceUsageString();

// Returns slice of supported texture formats; caller owns the formats slice and must free() it
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

#ifdef __cplusplus
} // extern "C"
#endif

#endif
