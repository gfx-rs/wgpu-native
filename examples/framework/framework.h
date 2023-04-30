#ifndef FRAMEWORK_H
#define FRAMEWORK_H

#include "wgpu.h"

#define UNUSED(x) (void)x;

void frmwrk_setup_logging(WGPULogLevel level);
WGPUShaderModule frmwrk_load_shader_module(WGPUDevice device, const char *name);
void frmwrk_print_global_report(WGPUGlobalReport report);

#endif // FRAMEWORK_H
