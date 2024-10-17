#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "framework.h"
#include "webgpu-headers/webgpu.h"

#define LOG_PREFIX "[push_constants]"

static void handle_request_adapter(WGPURequestAdapterStatus status,
                                   WGPUAdapter adapter, char const *message,
                                   void *userdata) {
  UNUSED(status)
  UNUSED(message)
  *(WGPUAdapter *)userdata = adapter;
}
static void handle_request_device(WGPURequestDeviceStatus status,
                                  WGPUDevice device, char const *message,
                                  void *userdata) {
  UNUSED(status)
  UNUSED(message)
  *(WGPUDevice *)userdata = device;
}
static void handle_buffer_map(WGPUBufferMapAsyncStatus status, void *userdata) {
  UNUSED(userdata)
  printf(LOG_PREFIX " buffer_map status=%#.8x\n", status);
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  frmwrk_setup_logging(WGPULogLevel_Warn);

  uint32_t numbers[] = {0, 0, 0, 0};
  uint32_t numbers_size = sizeof(numbers);
  uint32_t numbers_length = numbers_size / sizeof(uint32_t);

  WGPUInstance instance = wgpuCreateInstance(NULL);
  assert(instance);

  WGPUAdapter adapter = NULL;
  wgpuInstanceRequestAdapter(instance, NULL, handle_request_adapter,
                             (void *)&adapter);
  assert(adapter);

  WGPUSupportedLimitsExtras supported_limits_extras = {
      .chain =
          {
              .sType = WGPUSType_SupportedLimitsExtras,
          },
      .limits =
          {
              .maxPushConstantSize = 0,
          },
  };
  WGPUSupportedLimits supported_limits = {
      .nextInChain = &supported_limits_extras.chain,
  };
  wgpuAdapterGetLimits(adapter, &supported_limits);

  WGPURequiredLimitsExtras required_limits_extras = {
      .chain =
          {
              .sType = WGPUSType_RequiredLimitsExtras,
          },
      .limits = supported_limits_extras.limits,
  };
  WGPURequiredLimits required_limits = {
      .nextInChain = &required_limits_extras.chain,
      .limits = supported_limits.limits,
  };

  WGPUFeatureName requiredFeatures[] = {
      WGPUNativeFeature_PushConstants,
  };
  WGPUDeviceDescriptor device_desc = {
      .label = "compute_device",
      .requiredFeatures = requiredFeatures,
      .requiredFeatureCount = 1,
      .requiredLimits = &required_limits,
  };

  WGPUDevice device = NULL;
  wgpuAdapterRequestDevice(adapter, &device_desc, handle_request_device,
                           (void *)&device);
  assert(device);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  assert(queue);

  WGPUShaderModule shader_module =
      frmwrk_load_shader_module(device, "shader.wgsl");
  assert(shader_module);

  WGPUBuffer storage_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = "storage_buffer",
                  .usage = WGPUBufferUsage_Storage | WGPUBufferUsage_CopyDst |
                           WGPUBufferUsage_CopySrc,
                  .size = numbers_size,
                  .mappedAtCreation = false,
              });
  assert(storage_buffer);

  WGPUBuffer staging_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = "staging_buffer",
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .size = numbers_size,
                  .mappedAtCreation = false,
              });
  assert(staging_buffer);

  WGPUPushConstantRange push_constant_range = {
      .stages = WGPUShaderStage_Compute,
      .start = 0,
      .end = sizeof(uint32_t),
  };

  WGPUPipelineLayoutExtras pipeline_layout_extras = {
      .chain =
          {
              .sType = WGPUSType_PipelineLayoutExtras,
          },
      .pushConstantRangeCount = 1,
      .pushConstantRanges = &push_constant_range,
  };

  WGPUBindGroupLayoutEntry bind_group_layout_entries[] = {
      {
          .binding = 0,
          .visibility = WGPUShaderStage_Compute,
          .buffer =
              {
                  .type = WGPUBufferBindingType_Storage,
              },
      },
  };
  WGPUBindGroupLayoutDescriptor bind_group_layout_desc = {
      .label = "bind_group_layout",
      .nextInChain = NULL,
      .entryCount = 1,
      .entries = bind_group_layout_entries,
  };
  WGPUBindGroupLayout bind_group_layout =
      wgpuDeviceCreateBindGroupLayout(device, &bind_group_layout_desc);
  assert(bind_group_layout);

  WGPUPipelineLayoutDescriptor pipeline_layout_desc = {
      .label = "pipeline_layout",
      .nextInChain = &pipeline_layout_extras.chain,
      .bindGroupLayouts = &bind_group_layout,
      .bindGroupLayoutCount = 1,
  };
  WGPUPipelineLayout pipeline_layout =
      wgpuDeviceCreatePipelineLayout(device, &pipeline_layout_desc);
  assert(pipeline_layout);

  WGPUComputePipeline compute_pipeline = wgpuDeviceCreateComputePipeline(
      device, &(const WGPUComputePipelineDescriptor){
                  .label = "compute_pipeline",
                  .compute =
                      (const WGPUProgrammableStageDescriptor){
                          .module = shader_module,
                          .entryPoint = "main",
                      },
                  .layout = pipeline_layout,
              });
  assert(compute_pipeline);

  WGPUBindGroup bind_group = wgpuDeviceCreateBindGroup(
      device, &(const WGPUBindGroupDescriptor){
                  .label = "bind_group",
                  .layout = bind_group_layout,
                  .entryCount = 1,
                  .entries =
                      (const WGPUBindGroupEntry[]){
                          (const WGPUBindGroupEntry){
                              .binding = 0,
                              .buffer = storage_buffer,
                              .offset = 0,
                              .size = numbers_size,
                          },
                      },
              });
  assert(bind_group);

  WGPUCommandEncoder command_encoder = wgpuDeviceCreateCommandEncoder(
      device, &(const WGPUCommandEncoderDescriptor){
                  .label = "command_encoder",
              });
  assert(command_encoder);

  WGPUComputePassEncoder compute_pass_encoder =
      wgpuCommandEncoderBeginComputePass(command_encoder,
                                         &(const WGPUComputePassDescriptor){
                                             .label = "compute_pass",
                                         });
  assert(compute_pass_encoder);

  wgpuComputePassEncoderSetPipeline(compute_pass_encoder, compute_pipeline);
  wgpuComputePassEncoderSetBindGroup(compute_pass_encoder, 0, bind_group, 0,
                                     NULL);

  for (uint32_t i = 0; i < numbers_length; i++) {
    uint32_t pushConst = i;
    wgpuComputePassEncoderSetPushConstants(compute_pass_encoder, 0,
                                           sizeof(uint32_t), &pushConst);

    wgpuComputePassEncoderDispatchWorkgroups(compute_pass_encoder,
                                             numbers_length, 1, 1);
  }

  wgpuComputePassEncoderEnd(compute_pass_encoder);
  wgpuComputePassEncoderRelease(compute_pass_encoder);

  wgpuCommandEncoderCopyBufferToBuffer(command_encoder, storage_buffer, 0,
                                       staging_buffer, 0, numbers_size);

  WGPUCommandBuffer command_buffer = wgpuCommandEncoderFinish(
      command_encoder, &(const WGPUCommandBufferDescriptor){
                           .label = "command_buffer",
                       });
  assert(command_buffer);

  wgpuQueueWriteBuffer(queue, storage_buffer, 0, &numbers, numbers_size);
  wgpuQueueSubmit(queue, 1, &command_buffer);

  wgpuBufferMapAsync(staging_buffer, WGPUMapMode_Read, 0, numbers_size,
                     handle_buffer_map, NULL);
  wgpuDevicePoll(device, true, NULL);

  uint32_t *buf =
      (uint32_t *)wgpuBufferGetMappedRange(staging_buffer, 0, numbers_size);
  assert(buf);

  printf("times: [%d, %d, %d, %d]\n", buf[0], buf[1], buf[2], buf[3]);

  wgpuBufferUnmap(staging_buffer);
  wgpuCommandBufferRelease(command_buffer);
  wgpuCommandEncoderRelease(command_encoder);
  wgpuBindGroupRelease(bind_group);
  wgpuBindGroupLayoutRelease(bind_group_layout);
  wgpuComputePipelineRelease(compute_pipeline);
  wgpuBufferRelease(storage_buffer);
  wgpuBufferRelease(staging_buffer);
  wgpuShaderModuleRelease(shader_module);
  wgpuQueueRelease(queue);
  wgpuDeviceRelease(device);
  wgpuAdapterRelease(adapter);
  wgpuInstanceRelease(instance);
  return EXIT_SUCCESS;
}
