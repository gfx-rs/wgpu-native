#include "wgpu.h"

#define WGPULimits_DEFAULT                                                     \
  (WGPULimits) {                                                               \
    .maxBindGroups = WGPU_LIMIT_U32_UNDEFINED,                                 \
    .maxTextureDimension1D = WGPU_LIMIT_U32_UNDEFINED,                         \
    .maxTextureDimension2D = WGPU_LIMIT_U32_UNDEFINED,                         \
    .maxTextureDimension3D = WGPU_LIMIT_U32_UNDEFINED,                         \
    .maxTextureArrayLayers = WGPU_LIMIT_U32_UNDEFINED,                         \
    .maxDynamicUniformBuffersPerPipelineLayout = WGPU_LIMIT_U32_UNDEFINED,     \
    .maxDynamicStorageBuffersPerPipelineLayout = WGPU_LIMIT_U32_UNDEFINED,     \
    .maxSampledTexturesPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,              \
    .maxSamplersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,                     \
    .maxStorageBuffersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,               \
    .maxStorageTexturesPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,              \
    .maxUniformBuffersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,               \
    .maxUniformBufferBindingSize = WGPU_LIMIT_U64_UNDEFINED,                   \
    .maxStorageBufferBindingSize = WGPU_LIMIT_U64_UNDEFINED,                   \
    .minUniformBufferOffsetAlignment = WGPU_LIMIT_U32_UNDEFINED,               \
    .minStorageBufferOffsetAlignment = WGPU_LIMIT_U32_UNDEFINED,               \
    .maxVertexBuffers = WGPU_LIMIT_U32_UNDEFINED,                              \
    .maxVertexAttributes = WGPU_LIMIT_U32_UNDEFINED,                           \
    .maxVertexBufferArrayStride = WGPU_LIMIT_U32_UNDEFINED,                    \
    .maxInterStageShaderComponents = WGPU_LIMIT_U32_UNDEFINED,                 \
    .maxComputeWorkgroupStorageSize = WGPU_LIMIT_U32_UNDEFINED,                \
    .maxComputeInvocationsPerWorkgroup = WGPU_LIMIT_U32_UNDEFINED,             \
    .maxComputeWorkgroupSizeX = WGPU_LIMIT_U32_UNDEFINED,                      \
    .maxComputeWorkgroupSizeY = WGPU_LIMIT_U32_UNDEFINED,                      \
    .maxComputeWorkgroupSizeZ = WGPU_LIMIT_U32_UNDEFINED,                      \
    .maxComputeWorkgroupsPerDimension = WGPU_LIMIT_U32_UNDEFINED,              \
  }

WGPUShaderModuleDescriptor load_wgsl(const char *name);

void request_adapter_callback(WGPURequestAdapterStatus status,
                              WGPUAdapter received, const char *message,
                              void *userdata);

void request_device_callback(WGPURequestDeviceStatus status,
                             WGPUDevice received, const char *message,
                             void *userdata);

void readBufferMap(WGPUBufferMapAsyncStatus status, void *userdata);

void initializeLog(void);

void printGlobalReport(WGPUGlobalReport report);
void printAdapterFeatures(WGPUAdapter adapter);
