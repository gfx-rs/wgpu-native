#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define LOG_PREFIX "[pipeline_cache]"

static void handle_request_adapter(WGPURequestAdapterStatus status,
                                   WGPUAdapter adapter, WGPUStringView message,
                                   void *userdata1, void *userdata2) {
  UNUSED(status)
  UNUSED(message)
  UNUSED(userdata2)
  *(WGPUAdapter *)userdata1 = adapter;
}
static void handle_request_device(WGPURequestDeviceStatus status,
                                  WGPUDevice device, WGPUStringView message,
                                  void *userdata1, void *userdata2) {
  UNUSED(status)
  UNUSED(message)
  UNUSED(userdata2)
  *(WGPUDevice *)userdata1 = device;
}

// Chains pipeline_cache onto the descriptor, so the cache serves and records
// the compiled shader code.
static WGPUComputePipeline create_pipeline(WGPUDevice device,
                                           WGPUShaderModule shader_module,
                                           WGPUPipelineCache pipeline_cache) {
  WGPUPipelineDescriptorExtras pipeline_cache_extras = {
      .chain =
          (const WGPUChainedStruct){
              .sType = (WGPUSType)WGPUSType_PipelineDescriptorExtras,
          },
      .pipelineCache = pipeline_cache,
  };

  return wgpuDeviceCreateComputePipeline(
      device, &(const WGPUComputePipelineDescriptor){
                  .nextInChain = (const WGPUChainedStruct *)&pipeline_cache_extras,
                  .label = {"compute_pipeline", WGPU_STRLEN},
                  .compute =
                      (const WGPUComputeState){
                          .module = shader_module,
                          .entryPoint = {"main", WGPU_STRLEN},
                      },
              });
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  frmwrk_setup_logging(WGPULogLevel_Warn);

  WGPUInstance instance = wgpuCreateInstance(NULL);
  assert(instance);

  WGPUAdapter adapter = NULL;
  wgpuInstanceRequestAdapter(instance, NULL,
                             (const WGPURequestAdapterCallbackInfo){
                                 .callback = handle_request_adapter,
                                 .userdata1 = &adapter
                             });
  assert(adapter);

  // Not a failure: there is simply nothing to demonstrate on a backend that
  // does not implement pipeline caches.
  if (!wgpuAdapterHasFeature(adapter,
                             (WGPUFeatureName)WGPUNativeFeature_PipelineCache)) {
    printf(LOG_PREFIX " pipeline caches are unsupported on this adapter\n");
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);
    return EXIT_SUCCESS;
  }

  WGPUFeatureName required_device_features[1] = {
      (WGPUFeatureName)WGPUNativeFeature_PipelineCache,
  };

  WGPUDevice device = NULL;
  wgpuAdapterRequestDevice(adapter,
                           &(const WGPUDeviceDescriptor){
                               .requiredFeatureCount = 1,
                               .requiredFeatures = required_device_features,
                           },
                           (const WGPURequestDeviceCallbackInfo){
                               .callback = handle_request_device,
                               .userdata1 = &device
                           });
  assert(device);

  WGPUShaderModule shader_module =
      frmwrk_load_shader_module(device, "shader.wgsl");
  assert(shader_module);

  // Created without data, so it starts out empty; compiling a pipeline through
  // it populates it.
  WGPUPipelineCache pipeline_cache = wgpuDeviceCreatePipelineCache(
      device, &(const WGPUPipelineCacheDescriptor){
                  .label = {"pipeline_cache", WGPU_STRLEN},
              });
  assert(pipeline_cache);

  WGPUComputePipeline compute_pipeline =
      create_pipeline(device, shader_module, pipeline_cache);
  assert(compute_pipeline);

  size_t data_size = wgpuPipelineCacheGetData(pipeline_cache, NULL, 0);
  printf(LOG_PREFIX " cache data size=%zu\n", data_size);

  uint8_t *data = NULL;
  if (data_size > 0) {
    data = malloc(data_size);
    assert(data);
    size_t written = wgpuPipelineCacheGetData(pipeline_cache, data, data_size);
    assert(written == data_size);
  }

  wgpuComputePipelineRelease(compute_pipeline);
  wgpuPipelineCacheRelease(pipeline_cache);

  // Stands in for persisting the data to disk and passing it back on a later
  // run of the program.
  WGPUPipelineCache reloaded_pipeline_cache = wgpuDeviceCreatePipelineCache(
      device, &(const WGPUPipelineCacheDescriptor){
                  .label = {"reloaded_pipeline_cache", WGPU_STRLEN},
                  .data = data,
                  .dataSize = data_size,
                  .fallback = true,
              });
  assert(reloaded_pipeline_cache);

  WGPUComputePipeline reloaded_compute_pipeline =
      create_pipeline(device, shader_module, reloaded_pipeline_cache);
  assert(reloaded_compute_pipeline);

  printf(LOG_PREFIX " pipeline recreated from cache data\n");

  free(data);
  wgpuComputePipelineRelease(reloaded_compute_pipeline);
  wgpuPipelineCacheRelease(reloaded_pipeline_cache);
  wgpuShaderModuleRelease(shader_module);
  wgpuDeviceRelease(device);
  wgpuAdapterRelease(adapter);
  wgpuInstanceRelease(instance);
  return EXIT_SUCCESS;
}
