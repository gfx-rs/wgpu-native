#include "framework.h"

static void log_callback(WGPULogLevel level, char const *message,
                         void *userdata) {
  UNUSED(userdata)
  char *level_str;
  switch (level) {
  case WGPULogLevel_Error:
    level_str = "error";
    break;
  case WGPULogLevel_Warn:
    level_str = "warn";
    break;
  case WGPULogLevel_Info:
    level_str = "info";
    break;
  case WGPULogLevel_Debug:
    level_str = "debug";
    break;
  case WGPULogLevel_Trace:
    level_str = "trace";
    break;
  default:
    level_str = "unknown_level";
  }
  fprintf(stderr, "[wgpu] [%s] %s\n", level_str, message);
}

void frmwrk_setup_logging(WGPULogLevel level) {
  wgpuSetLogCallback(log_callback, NULL);
  wgpuSetLogLevel(level);
}

WGPUShaderModule frmwrk_load_shader_module(WGPUDevice device,
                                           const char *name) {
  FILE *file = NULL;
  char *buf = NULL;
  WGPUShaderModule shader_module = NULL;

  file = fopen(name, "rb");
  if (!file) {
    perror("fopen");
    goto cleanup;
  }

  if (fseek(file, 0, SEEK_END) != 0) {
    perror("fseek");
    goto cleanup;
  }
  long length = ftell(file);
  if (length == -1) {
    perror("ftell");
    goto cleanup;
  }
  if (fseek(file, 0, SEEK_SET) != 0) {
    perror("fseek");
    goto cleanup;
  }

  buf = malloc(length + 1);
  assert(buf);
  fread(buf, 1, length, file);
  buf[length] = 0;

  shader_module = wgpuDeviceCreateShaderModule(
      device, &(const WGPUShaderModuleDescriptor){
                  .label = name,
                  .nextInChain =
                      (const WGPUChainedStruct *)&(
                          const WGPUShaderModuleWGSLDescriptor){
                          .chain =
                              (const WGPUChainedStruct){
                                  .sType = WGPUSType_ShaderModuleWGSLDescriptor,
                              },
                          .code = buf,
                      },
              });

cleanup:
  if (file)
    fclose(file);
  if (buf)
    free(buf);
  return shader_module;
}

#define COPY_BUFFER_ALIGNMENT 4
#define MAX(A, B) ((A) > (B) ? (A) : (B))

WGPUBuffer frmwrk_device_create_buffer_init(
    WGPUDevice device, const frmwrk_buffer_init_descriptor *descriptor) {
  assert(descriptor);
  if (descriptor->content_size == 0) {
    return wgpuDeviceCreateBuffer(device, &(WGPUBufferDescriptor){
                                              .label = descriptor->label,
                                              .size = 0,
                                              .usage = descriptor->usage,
                                              .mappedAtCreation = false,
                                          });
  }

  size_t unpadded_size = descriptor->content_size;
  size_t align_mask = COPY_BUFFER_ALIGNMENT - 1;
  size_t padded_size =
      MAX((unpadded_size + align_mask) & ~align_mask, COPY_BUFFER_ALIGNMENT);
  WGPUBuffer buffer =
      wgpuDeviceCreateBuffer(device, &(WGPUBufferDescriptor){
                                         .label = descriptor->label,
                                         .size = padded_size,
                                         .usage = descriptor->usage,
                                         .mappedAtCreation = true,
                                     });
  void *buf = wgpuBufferGetMappedRange(buffer, 0, unpadded_size);
  memcpy(buf, descriptor->content, unpadded_size);
  wgpuBufferUnmap(buffer);

  return buffer;
}

#define print_registry_report(report, prefix)                                  \
  printf("%snumAllocated=%zu\n", prefix, report.numAllocated);                 \
  printf("%snumKeptFromUser=%zu\n", prefix, report.numKeptFromUser);           \
  printf("%snumReleasedFromUser=%zu\n", prefix, report.numReleasedFromUser);   \
  printf("%snumError=%zu\n", prefix, report.numError);                         \
  printf("%selementSize=%zu\n", prefix, report.elementSize)

#define print_hub_report(report, prefix)                                       \
  print_registry_report(report.adapters, prefix "adapter.");                   \
  print_registry_report(report.devices, prefix "devices.");                    \
  print_registry_report(report.queues, prefix "queues.");                      \
  print_registry_report(report.pipelineLayouts, prefix "pipelineLayouts.");    \
  print_registry_report(report.shaderModules, prefix "shaderModules.");        \
  print_registry_report(report.bindGroupLayouts, prefix "bindGroupLayouts.");  \
  print_registry_report(report.bindGroups, prefix "bindGroups.");              \
  print_registry_report(report.commandBuffers, prefix "commandBuffers.");      \
  print_registry_report(report.renderBundles, prefix "renderBundles.");        \
  print_registry_report(report.renderPipelines, prefix "renderPipelines.");    \
  print_registry_report(report.computePipelines, prefix "computePipelines.");  \
  print_registry_report(report.querySets, prefix "querySets.");                \
  print_registry_report(report.textures, prefix "textures.");                  \
  print_registry_report(report.textureViews, prefix "textureViews.");          \
  print_registry_report(report.samplers, prefix "samplers.")

void frmwrk_print_global_report(WGPUGlobalReport report) {
  printf("struct WGPUGlobalReport {\n");
  print_registry_report(report.surfaces, "\tsurfaces.");

  switch (report.backendType) {
  case WGPUBackendType_D3D12:
    print_hub_report(report.dx12, "\tdx12.");
    break;
  case WGPUBackendType_Metal:
    print_hub_report(report.metal, "\tmetal.");
    break;
  case WGPUBackendType_Vulkan:
    print_hub_report(report.vulkan, "\tvulkan.");
    break;
  case WGPUBackendType_OpenGL:
    print_hub_report(report.gl, "\tgl.");
    break;
  default:
    printf("[framework] frmwrk_print_global_report: invalid backend type: %d",
           report.backendType);
  }
  printf("}\n");
}

void frmwrk_print_adapter_info(WGPUAdapter adapter) {
  struct WGPUAdapterInfo info = {0};
  wgpuAdapterGetInfo(adapter, &info);
  printf("description: %s\n", info.description);
  printf("vendor: %s\n", info.vendor);
  printf("architecture: %s\n", info.architecture);
  printf("device: %s\n", info.device);
  printf("backend type: %u\n", info.backendType);
  printf("adapter type: %u\n", info.adapterType);
  printf("vendorID: %x\n", info.vendorID);
  printf("deviceID: %x\n", info.deviceID);
  wgpuAdapterInfoFreeMembers(info);
}