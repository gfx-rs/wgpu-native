#include "unused.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <stdio.h>
#include <stdlib.h>

WGPUShaderModuleDescriptor load_wgsl(const char *name) {
  FILE *file = fopen(name, "rb");
  if (!file) {
    printf("Unable to open %s\n", name);
    exit(1);
  }
  fseek(file, 0, SEEK_END);
  long length = ftell(file);
  unsigned char *bytes = malloc(length + 1);
  fseek(file, 0, SEEK_SET);
  fread(bytes, 1, length, file);
  fclose(file);
  bytes[length] = 0;

  WGPUShaderModuleWGSLDescriptor *wgslDescriptor =
      malloc(sizeof(WGPUShaderModuleWGSLDescriptor));
  wgslDescriptor->chain.next = NULL;
  wgslDescriptor->chain.sType = WGPUSType_ShaderModuleWGSLDescriptor;
  wgslDescriptor->code = (const char *)bytes;
  return (WGPUShaderModuleDescriptor){
      .nextInChain = (const WGPUChainedStruct *)wgslDescriptor,
      .label = name,
  };
}

void request_adapter_callback(WGPURequestAdapterStatus status,
                              WGPUAdapter received, const char *message,
                              void *userdata) {
  UNUSED(status);
  UNUSED(message);

  *(WGPUAdapter *)userdata = received;
}

void request_device_callback(WGPURequestDeviceStatus status,
                             WGPUDevice received, const char *message,
                             void *userdata) {
  UNUSED(status);
  UNUSED(message);

  *(WGPUDevice *)userdata = received;
}

void readBufferMap(WGPUBufferMapAsyncStatus status, void *userdata) {
  UNUSED(status);
  UNUSED(userdata);
}

void logCallback(WGPULogLevel level, const char *msg, void *userdata) {
  UNUSED(userdata);

  char *level_str;
  switch (level) {
  case WGPULogLevel_Error:
    level_str = "Error";
    break;
  case WGPULogLevel_Warn:
    level_str = "Warn";
    break;
  case WGPULogLevel_Info:
    level_str = "Info";
    break;
  case WGPULogLevel_Debug:
    level_str = "Debug";
    break;
  case WGPULogLevel_Trace:
    level_str = "Trace";
    break;
  default:
    level_str = "Unknown Level";
  }
  printf("[%s] %s\n", level_str, msg);
}

void initializeLog(void) {
  wgpuSetLogCallback(logCallback, NULL);
  wgpuSetLogLevel(WGPULogLevel_Warn);
}

#define printStorageReport(report, prefix)                                     \
  printf("%snumOccupied=%zu\n", prefix, report.numOccupied);                   \
  printf("%snumVacant=%zu\n", prefix, report.numVacant);                       \
  printf("%snumError=%zu\n", prefix, report.numError);                         \
  printf("%selementSize=%zu\n", prefix, report.elementSize)

#define printHubReport(report, prefix)                                         \
  printStorageReport(report.adapters, prefix "adapter.");                      \
  printStorageReport(report.devices, prefix "devices.");                       \
  printStorageReport(report.pipelineLayouts, prefix "pipelineLayouts.");       \
  printStorageReport(report.shaderModules, prefix "shaderModules.");           \
  printStorageReport(report.bindGroupLayouts, prefix "bindGroupLayouts.");     \
  printStorageReport(report.bindGroups, prefix "bindGroups.");                 \
  printStorageReport(report.commandBuffers, prefix "commandBuffers.");         \
  printStorageReport(report.renderBundles, prefix "renderBundles.");           \
  printStorageReport(report.renderPipelines, prefix "renderPipelines.");       \
  printStorageReport(report.computePipelines, prefix "computePipelines.");     \
  printStorageReport(report.querySets, prefix "querySets.");                   \
  printStorageReport(report.textures, prefix "textures.");                     \
  printStorageReport(report.textureViews, prefix "textureViews.");             \
  printStorageReport(report.samplers, prefix "samplers.")

void printGlobalReport(WGPUGlobalReport report) {
  printf("struct WGPUGlobalReport {\n");
  printStorageReport(report.surfaces, "\tsurfaces.");

  switch (report.backendType) {
  case WGPUBackendType_D3D11:
    printHubReport(report.dx11, "\tdx11.");
    break;
  case WGPUBackendType_D3D12:
    printHubReport(report.dx12, "\tdx12.");
    break;
  case WGPUBackendType_Metal:
    printHubReport(report.metal, "\tmetal.");
    break;
  case WGPUBackendType_Vulkan:
    printHubReport(report.vulkan, "\tvulkan.");
    break;
  case WGPUBackendType_OpenGL:
    printHubReport(report.gl, "\tgl.");
    break;
  default:
    printf("WARN:printGlobalReport: invalid backened type: %d",
           report.backendType);
  }
  printf("}\n");
}

void printAdapterFeatures(WGPUAdapter adapter) {
  size_t count = wgpuAdapterEnumerateFeatures(adapter, NULL);
  WGPUFeatureName *features =
      (WGPUFeatureName *)malloc(count * sizeof(WGPUFeatureName));
  wgpuAdapterEnumerateFeatures(adapter, features);

  printf("[]WGPUFeatureName {\n");

  for (size_t i = 0; i < count; i++) {
    uint32_t feature = features[i];
    switch (feature) {
    case WGPUFeatureName_DepthClipControl:
      printf("\tDepthClipControl\n");
      break;

    case WGPUFeatureName_Depth24UnormStencil8:
      printf("\tDepth24UnormStencil8\n");
      break;

    case WGPUFeatureName_Depth32FloatStencil8:
      printf("\tDepth32FloatStencil8\n");
      break;

    case WGPUFeatureName_TimestampQuery:
      printf("\tTimestampQuery\n");
      break;

    case WGPUFeatureName_PipelineStatisticsQuery:
      printf("\tPipelineStatisticsQuery\n");
      break;

    case WGPUFeatureName_TextureCompressionBC:
      printf("\tTextureCompressionBC\n");
      break;

    case WGPUFeatureName_TextureCompressionETC2:
      printf("\tTextureCompressionETC2\n");
      break;

    case WGPUFeatureName_TextureCompressionASTC:
      printf("\tTextureCompressionASTC\n");
      break;

    case WGPUFeatureName_IndirectFirstInstance:
      printf("\tIndirectFirstInstance\n");
      break;

    case WGPUNativeFeature_PUSH_CONSTANTS:
      printf("\tWGPUNativeFeature_PUSH_CONSTANTS\n");
      break;

    case WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES:
      printf("\tWGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES\n");
      break;

    case WGPUNativeFeature_MULTI_DRAW_INDIRECT:
      printf("\tWGPUNativeFeature_MULTI_DRAW_INDIRECT\n");
      break;

    case WGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT:
      printf("\tWGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT\n");
      break;

    case WGPUNativeFeature_VERTEX_WRITABLE_STORAGE:
      printf("\tWGPUNativeFeature_VERTEX_WRITABLE_STORAGE\n");
      break;

    default:
      printf("\tUnknown=%d\n", feature);
    }
  }

  printf("}\n");

  free(features);
}
