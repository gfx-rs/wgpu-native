#include "webgpu-headers/webgpu.h"
#include "wgpu.h"

#include "framework.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  uint32_t numbers[] = {1, 2, 3, 4};
  uint32_t numbersSize = sizeof(numbers);
  uint32_t numbersLength = numbersSize / sizeof(uint32_t);

  initializeLog();
  WGPUAdapter adapter;
  wgpuInstanceRequestAdapter(NULL,
                             &(WGPURequestAdapterOptions){
                                 .nextInChain = NULL,
                                 .compatibleSurface = NULL,
                             },
                             request_adapter_callback, (void *)&adapter);

  WGPUDevice device;
  wgpuAdapterRequestDevice(
      adapter,
      &(WGPUDeviceDescriptor){
          .nextInChain =
              (const WGPUChainedStruct *)&(WGPUDeviceExtras){
                  .chain =
                      (WGPUChainedStruct){
                          .next = NULL,
                          .sType = (WGPUSType)WGPUSType_DeviceExtras,
                      },

                  .label = "Device",
                  .tracePath = NULL,
              },
          .requiredLimits =
              &(WGPURequiredLimits){
                  .nextInChain = NULL,
                  .limits =
                      (WGPULimits){
                          .maxBindGroups = 1,
                      },
              },
          .defaultQueue =
            (WGPUQueueDescriptor){
                .nextInChain = NULL,
                .label = NULL,
            },
      },
      request_device_callback, (void *)&device);

  WGPUShaderModuleDescriptor shaderSource = load_wgsl("shader.wgsl");
  WGPUShaderModule shader = wgpuDeviceCreateShaderModule(device, &shaderSource);

  WGPUBuffer stagingBuffer = wgpuDeviceCreateBuffer(
      device, &(WGPUBufferDescriptor){
                  .nextInChain = NULL,
                  .label = "StagingBuffer",
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .size = numbersSize,
                  .mappedAtCreation = false,
              });
  WGPUBuffer storageBuffer = wgpuDeviceCreateBuffer(
      device, &(WGPUBufferDescriptor){
                  .nextInChain = NULL,
                  .label = "StorageBuffer",
                  .usage = WGPUBufferUsage_Storage | WGPUBufferUsage_CopyDst |
                           WGPUBufferUsage_CopySrc,
                  .size = numbersSize,
                  .mappedAtCreation = false,
              });

  WGPUBindGroupLayout bindGroupLayout = wgpuDeviceCreateBindGroupLayout(
      device, &(WGPUBindGroupLayoutDescriptor){
                  .label = "Bind Group Layout",
                  .entries =
                      &(WGPUBindGroupLayoutEntry){
                          .nextInChain = NULL,
                          .binding = 0,
                          .visibility = WGPUShaderStage_Compute,
                          .buffer =
                              (WGPUBufferBindingLayout){
                                  .type = WGPUBufferBindingType_Storage,
                              },
                          .sampler =
                              (WGPUSamplerBindingLayout){
                                  .type = WGPUSamplerBindingType_Undefined,
                              },
                          .texture =
                              (WGPUTextureBindingLayout){
                                  .sampleType = WGPUTextureSampleType_Undefined,
                              },
                          .storageTexture =
                              (WGPUStorageTextureBindingLayout){
                                  .access = WGPUStorageTextureAccess_Undefined,
                              }},
                  .entryCount = 1});

  WGPUBindGroup bindGroup = wgpuDeviceCreateBindGroup(
      device, &(WGPUBindGroupDescriptor){
                  .label = "Bind Group",
                  .layout = bindGroupLayout,
                  .entries = &(WGPUBindGroupEntry){.binding = 0,
                                                   .buffer = storageBuffer,
                                                   .offset = 0,
                                                   .size = numbersSize},
                  .entryCount = 1});

  WGPUBindGroupLayout bindGroupLayouts[1] = {bindGroupLayout};
  WGPUPipelineLayout pipelineLayout = wgpuDeviceCreatePipelineLayout(
      device,
      &(WGPUPipelineLayoutDescriptor){.bindGroupLayouts = bindGroupLayouts,
                                      .bindGroupLayoutCount = 1});

  WGPUComputePipeline computePipeline = wgpuDeviceCreateComputePipeline(
      device, &(WGPUComputePipelineDescriptor){
                  .layout = pipelineLayout,
                  .compute = (WGPUProgrammableStageDescriptor){
                      .module = shader, .entryPoint = "main"}});

  WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(
      device, &(WGPUCommandEncoderDescriptor){.label = "Command Encoder"});

  WGPUComputePassEncoder computePass = wgpuCommandEncoderBeginComputePass(
      encoder, &(WGPUComputePassDescriptor){.label = "Compute Pass"});

  wgpuComputePassEncoderSetPipeline(computePass, computePipeline);
  wgpuComputePassEncoderSetBindGroup(computePass, 0, bindGroup, 0, NULL);
  wgpuComputePassEncoderDispatch(computePass, numbersLength, 1, 1);
  wgpuComputePassEncoderEnd(computePass);
  wgpuCommandEncoderCopyBufferToBuffer(encoder, storageBuffer, 0, stagingBuffer,
                                       0, numbersSize);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  WGPUCommandBuffer cmdBuffer = wgpuCommandEncoderFinish(
      encoder, &(WGPUCommandBufferDescriptor){.label = NULL});
  wgpuQueueWriteBuffer(queue, storageBuffer, 0, &numbers, numbersSize);
  wgpuQueueSubmit(queue, 1, &cmdBuffer);

  wgpuBufferMapAsync(stagingBuffer, WGPUMapMode_Read, 0, numbersSize,
                     readBufferMap, NULL);
  wgpuDevicePoll(device, true);

  uint32_t *times =
      (uint32_t *)wgpuBufferGetMappedRange(stagingBuffer, 0, numbersSize);

  printf("Times: [%d, %d, %d, %d]\n", times[0], times[1], times[2], times[3]);

  wgpuBufferUnmap(stagingBuffer);

  return 0;
}
