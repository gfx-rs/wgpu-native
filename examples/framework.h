#include "wgpu.h"

WGPUShaderModuleDescriptor load_wgsl(const char *name);

void request_adapter_callback(WGPUAdapter received, void* userdata);

void request_device_callback(WGPUDevice received, void* userdata);

void readBufferMap(WGPUBufferMapAsyncStatus status, uint8_t* userdata);

void initializeLog();