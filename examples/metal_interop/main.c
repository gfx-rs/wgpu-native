#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <stdio.h>
#include <stdlib.h>

#define LOG_PREFIX "[metal_interop]"

static void handle_request_device(WGPURequestDeviceStatus status,
                                  WGPUDevice device,
                                  WGPUStringView message,
                                  void *userdata1,
                                  void *userdata2) {
  UNUSED(status)
  UNUSED(message)
  UNUSED(userdata2)
  *(WGPUDevice *)userdata1 = device;
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  frmwrk_setup_logging(WGPULogLevel_Warn);

  WGPUInstance instance = wgpuCreateInstance(NULL);
  if (!instance) {
    fprintf(stderr, LOG_PREFIX " failed to create instance\n");
    return EXIT_FAILURE;
  }

  const size_t adapter_count = wgpuInstanceEnumerateAdapters(instance, NULL, NULL);
  if (adapter_count == 0) {
    fprintf(stderr, LOG_PREFIX " no adapters available\n");
    wgpuInstanceRelease(instance);
    return EXIT_SUCCESS;
  }
  WGPUAdapter *adapters = calloc(adapter_count, sizeof(WGPUAdapter));
  if (!adapters) {
    fprintf(stderr, LOG_PREFIX " failed to allocate adapter array\n");
    wgpuInstanceRelease(instance);
    return EXIT_FAILURE;
  }
  wgpuInstanceEnumerateAdapters(instance, NULL, adapters);
  WGPUAdapter adapter = adapters[0];
  for (size_t i = 1; i < adapter_count; ++i) {
    wgpuAdapterRelease(adapters[i]);
  }
  free(adapters);
  if (!adapter) {
    fprintf(stderr, LOG_PREFIX " failed to enumerate adapter\n");
    wgpuInstanceRelease(instance);
    return EXIT_FAILURE;
  }

  WGPUAdapterInfo info = {0};
  wgpuAdapterGetInfo(adapter, &info);

  WGPUDevice device = NULL;
  wgpuAdapterRequestDevice(adapter, NULL,
                           (const WGPURequestDeviceCallbackInfo){
                               .mode = WGPUCallbackMode_AllowSpontaneous,
                               .callback = handle_request_device,
                               .userdata1 = &device,
                           });
  if (!device) {
    fprintf(stderr, LOG_PREFIX " failed to request device\n");
    wgpuAdapterInfoFreeMembers(info);
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);
    return EXIT_FAILURE;
  }

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  if (!queue) {
    fprintf(stderr, LOG_PREFIX " failed to get queue\n");
    wgpuDeviceRelease(device);
    wgpuAdapterInfoFreeMembers(info);
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);
    return EXIT_FAILURE;
  }

  WGPUTexture texture = wgpuDeviceCreateTexture(
      device,
      &(const WGPUTextureDescriptor){
          .label = {"interop_texture", WGPU_STRLEN},
          .size = (WGPUExtent3D){.width = 4, .height = 4, .depthOrArrayLayers = 1},
          .mipLevelCount = 1,
          .sampleCount = 1,
          .dimension = WGPUTextureDimension_2D,
          .format = WGPUTextureFormat_RGBA8Unorm,
          .usage = WGPUTextureUsage_TextureBinding | WGPUTextureUsage_CopyDst,
      });

  if (!texture) {
    fprintf(stderr, LOG_PREFIX " failed to create texture\n");
    wgpuQueueRelease(queue);
    wgpuDeviceRelease(device);
    wgpuAdapterInfoFreeMembers(info);
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);
    return EXIT_FAILURE;
  }

  void *native_device = wgpuDeviceGetNativeMetalDevice(device);
  void *native_queue = wgpuQueueGetNativeMetalCommandQueue(queue);
  void *native_texture = wgpuTextureGetNativeMetalTexture(texture);

  const int is_metal = (info.backendType == WGPUBackendType_Metal);
  printf(LOG_PREFIX " backend=%u native_device=%p native_queue=%p native_texture=%p\n",
         info.backendType, native_device, native_queue, native_texture);

  int ok = 1;
  if (is_metal) {
    if (!native_device || !native_texture) {
      fprintf(stderr, LOG_PREFIX " expected non-null native Metal handles on Metal backend\n");
      ok = 0;
    }
    if (!native_queue) {
      printf(LOG_PREFIX " native_queue is not available (wgpu-hal does not expose the raw MTLCommandQueue)\n");
    }
  } else {
    printf(LOG_PREFIX " non-Metal backend: skipping strict non-null check\n");
  }

  wgpuTextureRelease(texture);
  wgpuQueueRelease(queue);
  wgpuDeviceRelease(device);
  wgpuAdapterInfoFreeMembers(info);
  wgpuAdapterRelease(adapter);
  wgpuInstanceRelease(instance);

  return ok ? EXIT_SUCCESS : EXIT_FAILURE;
}
