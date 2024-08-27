#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  frmwrk_setup_logging(WGPULogLevel_Warn);

  WGPUInstance instance = wgpuCreateInstance(NULL);
  assert(instance);

  const size_t adapter_count =
      wgpuInstanceEnumerateAdapters(instance, NULL, NULL);
  WGPUAdapter *adapters = malloc(sizeof(WGPUAdapter) * adapter_count);
  assert(adapters);
  wgpuInstanceEnumerateAdapters(instance, NULL, adapters);

  for (int i = 0; i < adapter_count; i++) {
    WGPUAdapter adapter = adapters[i];
    assert(adapter);

    WGPUAdapterInfo info = {0};
    wgpuAdapterGetInfo(adapter, &info);
    printf("WGPUAdapter: %d\n", i);
    printf("WGPUAdapterInfo {\n"
           "\tvendor: %s\n"
           "\tarchitecture: %s\n"
           "\tdevice: %s\n"
           "\tdescription: %s\n"
           "\tbackendType: %#.8x\n"
           "\tadapterType: %#.8x\n"
           "\tvendorID: %" PRIu32 "\n"
           "\tdeviceID: %" PRIu32 "\n"
           "}\n",
           info.vendor, info.architecture, info.device, info.description,
           info.backendType, info.adapterType, info.vendorID, info.deviceID);

    wgpuAdapterInfoFreeMembers(info);
    wgpuAdapterRelease(adapter);
  }

  free(adapters);
  wgpuInstanceRelease(instance);
}
