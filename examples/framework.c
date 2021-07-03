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

    WGPUShaderModuleWGSLDescriptor *wgslDescriptor = malloc(sizeof(WGPUShaderModuleWGSLDescriptor));
    wgslDescriptor->chain.next = NULL;
    wgslDescriptor->chain.sType = WGPUSType_ShaderModuleWGSLDescriptor;
    wgslDescriptor->source = (const char*) bytes;
    return (WGPUShaderModuleDescriptor) {
        .nextInChain = (const WGPUChainedStruct *) wgslDescriptor,
        .label = name,
    };
}

void request_adapter_callback(WGPUAdapter received, void* userdata)
{
    *(WGPUAdapter*)userdata = received;
}

void request_device_callback(WGPUDevice received, void* userdata)
{
    *(WGPUDevice*)userdata = received;
}

void readBufferMap(WGPUBufferMapAsyncStatus status, void *userdata)
{
}

void logCallback(WGPULogLevel level, const char *msg) {
    char* level_str;
    switch(level){
        case WGPULogLevel_Error: level_str = "Error"; break;
        case WGPULogLevel_Warn: level_str = "Warn"; break;
        case WGPULogLevel_Info: level_str = "Info"; break;
        case WGPULogLevel_Debug: level_str = "Debug"; break;
        case WGPULogLevel_Trace: level_str = "Trace"; break;
        default: level_str = "Unknown Level";
    }
    printf("[%s] %s\n", level_str, msg);
}

void initializeLog() {
    wgpuSetLogCallback(logCallback);
    wgpuSetLogLevel(WGPULogLevel_Warn);
}