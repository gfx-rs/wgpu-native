#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu-headers/webgpu.h"

typedef enum WGPUNativeSType {
    // Start at 6 to prevent collisions with webgpu STypes
    WGPUSType_DeviceExtras = 0x60000001,
    WGPUSType_AdapterExtras = 0x60000002,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

typedef enum WGPUNativeFeature {
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

typedef void (*WGPULogCallback)(WGPULogLevel level, const char *msg);

#ifdef __cplusplus
extern "C" {
#endif

void wgpuDevicePoll(WGPUDevice device, bool force_wait);

void wgpuSetLogCallback(WGPULogCallback callback);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

void wgpuRenderPassEncoderSetPushConstants(WGPURenderPassEncoder encoder, WGPUShaderStage stages, uint32_t offset, uint32_t sizeBytes, void* const data);

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
