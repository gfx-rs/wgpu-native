#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu-headers/webgpu.h"

typedef enum WGPUNativeSType {
    // Start at 6 to prevent collisions with webgpu STypes
    WGPUSType_DeviceExtras = 0x60000001,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;


typedef struct WGPUDeviceExtras {
    WGPUChainedStruct chain;
    uint32_t maxBindGroups;
    const char* label;
    const char* tracePath;
} WGPUDeviceExtras;

typedef enum WGPULogLevel {
    WGPULogLevel_Off = 0x00000000,
    WGPULogLevel_Error = 0x00000001,
    WGPULogLevel_Warn = 0x00000002,
    WGPULogLevel_Info = 0x00000003,
    WGPULogLevel_Debug = 0x00000004,
    WGPULogLevel_Trace = 0x00000005,
    WGPULogLevel_Force32 = 0x7FFFFFFF
} WGPULogLevel;

typedef void (*WGPULogCallback)(WGPULogLevel level, const char *msg);

void wgpuDevicePoll(WGPUDevice device, bool force_wait);

void wgpuSetLogCallback(WGPULogCallback callback);

void wgpuSetLogLevel(WGPULogLevel level);

#endif 