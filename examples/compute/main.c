#include "webgpu-headers/webgpu.h"
#include "wgpu.h"

#include "unused.h"
#include "framework.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void handle_device_lost(WGPUDeviceLostReason reason, char const *message, void *userdata) {
  UNUSED(userdata);

  printf("DEVICE LOST (%d): %s\n", reason, message);
}

static void handle_uncaptured_error(WGPUErrorType type, char const *message, void *userdata) {
  UNUSED(userdata);

  printf("UNCAPTURED ERROR (%d): %s\n", type, message);
}

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
          .nextInChain = NULL,
          .label = "Device",
          .requiredLimits =
              &(WGPURequiredLimits){
                  .nextInChain = NULL,
                  .limits =
                      (WGPULimits){
                          .maxBindGroups = WGPU_LIMIT_U32_UNDEFINED,
                          .maxTextureDimension1D = WGPU_LIMIT_U32_UNDEFINED,
                          .maxTextureDimension2D = WGPU_LIMIT_U32_UNDEFINED,
                          .maxTextureDimension3D = WGPU_LIMIT_U32_UNDEFINED,
                          .maxTextureArrayLayers = WGPU_LIMIT_U32_UNDEFINED,
                          .maxDynamicUniformBuffersPerPipelineLayout = WGPU_LIMIT_U32_UNDEFINED,
                          .maxDynamicStorageBuffersPerPipelineLayout = WGPU_LIMIT_U32_UNDEFINED,
                          .maxSampledTexturesPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,
                          .maxSamplersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,
                          .maxStorageBuffersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,
                          .maxStorageTexturesPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,
                          .maxUniformBuffersPerShaderStage = WGPU_LIMIT_U32_UNDEFINED,
                          .maxUniformBufferBindingSize = WGPU_LIMIT_U64_UNDEFINED,
                          .maxStorageBufferBindingSize = WGPU_LIMIT_U64_UNDEFINED,
                          .minUniformBufferOffsetAlignment = WGPU_LIMIT_U32_UNDEFINED,
                          .minStorageBufferOffsetAlignment = WGPU_LIMIT_U32_UNDEFINED,
                          .maxVertexBuffers = WGPU_LIMIT_U32_UNDEFINED,
                          .maxVertexAttributes = WGPU_LIMIT_U32_UNDEFINED,
                          .maxVertexBufferArrayStride = WGPU_LIMIT_U32_UNDEFINED,
                          .maxInterStageShaderComponents = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeWorkgroupStorageSize = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeInvocationsPerWorkgroup = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeWorkgroupSizeX = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeWorkgroupSizeY = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeWorkgroupSizeZ = WGPU_LIMIT_U32_UNDEFINED,
                          .maxComputeWorkgroupsPerDimension = WGPU_LIMIT_U32_UNDEFINED,
                      },
              },
          .defaultQueue =
              (WGPUQueueDescriptor){
                  .nextInChain = NULL,
                  .label = NULL,
              },
      },
      request_device_callback, (void *)&device);

  wgpuDeviceSetUncapturedErrorCallback(device, handle_uncaptured_error, NULL);
  wgpuDeviceSetDeviceLostCallback(device, handle_device_lost, NULL);

  WGPUShaderModuleDescriptor shaderSource = load_wgsl("shader.wgsl");
  WGPUShaderModule shader = wgpuDeviceCreateShaderModule(device, &shaderSource);

  WGPUBuffer stagingBuffer =
      wgpuDeviceCreateBuffer(device, &(WGPUBufferDescriptor){
                                         .nextInChain = NULL,
                                         .label = "StagingBuffer",
                                         .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                                         .size = numbersSize,
                                         .mappedAtCreation = false,
                                     });
  WGPUBuffer storageBuffer =
      wgpuDeviceCreateBuffer(device, &(WGPUBufferDescriptor){
                                         .nextInChain = NULL,
                                         .label = "StorageBuffer",
                                         .usage = WGPUBufferUsage_Storage |
                                                  WGPUBufferUsage_CopyDst | WGPUBufferUsage_CopySrc,
                                         .size = numbersSize,
                                         .mappedAtCreation = false,
                                     });

  WGPUComputePipeline computePipeline =
      wgpuDeviceCreateComputePipeline(device, &(WGPUComputePipelineDescriptor){
                                                  .nextInChain = NULL,
                                                  .label = "Compute Pipeline",
                                                  .layout = NULL,
                                                  .compute =
                                                      (WGPUProgrammableStageDescriptor){
                                                          .module = shader,
                                                          .entryPoint = "main",
                                                      },
                                              });

  WGPUBindGroupLayout bindGroupLayout = wgpuComputePipelineGetBindGroupLayout(computePipeline, 0);

  WGPUBindGroup bindGroup =
      wgpuDeviceCreateBindGroup(device, &(WGPUBindGroupDescriptor){
                                            .nextInChain = NULL,
                                            .label = "Bind Group",
                                            .layout = bindGroupLayout,
                                            .entries =
                                                (WGPUBindGroupEntry[]){
                                                    (WGPUBindGroupEntry){
                                                        .binding = 0,
                                                        .buffer = storageBuffer,
                                                        .offset = 0,
                                                        .size = numbersSize,
                                                    },
                                                },
                                            .entryCount = 1,
                                        });

  WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(
      device, &(WGPUCommandEncoderDescriptor){.label = "Command Encoder"});

  WGPUComputePassEncoder computePass = wgpuCommandEncoderBeginComputePass(
      encoder, &(WGPUComputePassDescriptor){.label = "Compute Pass"});

  wgpuComputePassEncoderSetPipeline(computePass, computePipeline);
  wgpuComputePassEncoderSetBindGroup(computePass, 0, bindGroup, 0, NULL);
  wgpuComputePassEncoderDispatchWorkgroups(computePass, numbersLength, 1, 1);
  wgpuComputePassEncoderEnd(computePass);
  wgpuCommandEncoderCopyBufferToBuffer(encoder, storageBuffer, 0, stagingBuffer, 0, numbersSize);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  WGPUCommandBuffer cmdBuffer =
      wgpuCommandEncoderFinish(encoder, &(WGPUCommandBufferDescriptor){.label = NULL});
  wgpuQueueWriteBuffer(queue, storageBuffer, 0, &numbers, numbersSize);
  wgpuQueueSubmit(queue, 1, &cmdBuffer);

  wgpuBufferMapAsync(stagingBuffer, WGPUMapMode_Read, 0, numbersSize, readBufferMap, NULL);
  wgpuDevicePoll(device, true, NULL);

  uint32_t *times = (uint32_t *)wgpuBufferGetMappedRange(stagingBuffer, 0, numbersSize);

  printf("Times: [%d, %d, %d, %d]\n", times[0], times[1], times[2], times[3]);

  wgpuBufferUnmap(stagingBuffer);

  return 0;
}
