#ifndef WGPU_H
#define WGPU_H
#include "wgpu.h"
#endif

#include <stdio.h>
#include <stdlib.h>

WGPUShaderModuleDescriptor read_file(const char *name) {
    FILE *file = fopen(name, "rb");
    if (!file) {
        printf("Unable to open %s\n", name);
        exit(1);
    }
    fseek(file, 0, SEEK_END);
    long length = ftell(file);
    unsigned char *bytes = malloc(length);
    fseek(file, 0, SEEK_SET);
    fread(bytes, 1, length, file);
    fclose(file);

    WGPUShaderModuleSPIRVDescriptor *spirvDescriptor = malloc(sizeof(WGPUShaderModuleSPIRVDescriptor));
    spirvDescriptor->chain = (WGPUChainedStruct) {
        .next = NULL,
        .sType = WGPUSType_ShaderModuleSPIRVDescriptor
    };
    spirvDescriptor->code = (uint32_t *) bytes;
    spirvDescriptor->codeSize = length / 4;
    return (WGPUShaderModuleDescriptor) {
        .nextInChain = (const WGPUChainedStruct *) spirvDescriptor,
        .label = NULL,
        .flags = WGPUShaderFlags_VALIDATION,
    };
}
