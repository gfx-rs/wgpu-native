#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  frmwrk_setup_logging(WGPULogLevel_Warn);

  WGPUInstance instance =
      wgpuCreateInstance(&(const WGPUInstanceDescriptor){0});
  assert(instance);

  const size_t adapter_count =
      wgpuInstanceEnumerateAdapters(instance, NULL, NULL);
  WGPUAdapter *adapters = malloc(sizeof(WGPUAdapter) * adapter_count);
  assert(adapters);
  wgpuInstanceEnumerateAdapters(instance, NULL, adapters);

  for (int i = 0; i < adapter_count; i++) {
    WGPUAdapter adapter = adapters[i];
    assert(adapter);

    WGPUAdapterProperties props;
    wgpuAdapterGetProperties(adapter, &props);
    printf("WGPUAdapter: %d\n", i);
    printf("WGPUAdapterProperties {\n"
           "\tvendorID: %" PRIu32 "\n"
           "\tvendorName: %s\n"
           "\tarchitecture: %s\n"
           "\tdeviceID: %" PRIu32 "\n"
           "\tname: %s\n"
           "\tdriverDescription: %s\n"
           "\tadapterType: %#.8x\n"
           "\tbackendType: %#.8x\n"
           "}\n",
           props.vendorID, props.vendorName, props.architecture, props.deviceID,
           props.name, props.driverDescription, props.adapterType,
           props.backendType);

    wgpuAdapterRelease(adapter);
  }

  free(adapters);
  wgpuInstanceRelease(instance);
}
