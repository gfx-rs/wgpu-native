#include "wgpu.h"

WGPUShaderModuleDescriptor load_wgsl(const char *name);

void request_adapter_callback(WGPURequestAdapterStatus status, WGPUAdapter received,
                              const char *message, void *userdata);

void request_device_callback(WGPURequestDeviceStatus status, WGPUDevice received,
                             const char *message, void *userdata);

void readBufferMap(WGPUBufferMapAsyncStatus status, void *userdata);

void initializeLog();

void printGlobalReport(WGPUGlobalReport report);
void printAdapterFeatures(WGPUAdapter adapter);
