#include "framework.h"
#include <stdio.h>
#include <stdlib.h>

#define LOG_PREFIX "[compute]"

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
static void handle_device_lost(WGPUDeviceLostReason reason, char const *message,
                               void *userdata) {
  UNUSED(userdata)
  printf(LOG_PREFIX " device_lost reason=%#.8x message=%s\n", reason, message);
}
static void handle_uncaptured_error(WGPUErrorType type, char const *message,
                                    void *userdata) {
  UNUSED(userdata)
  printf(LOG_PREFIX " uncaptured_error type=%#.8x message=%s\n", type, message);
}
static void handle_buffer_map(WGPUBufferMapAsyncStatus status, void *userdata) {
  UNUSED(userdata)
  printf(LOG_PREFIX " buffer_map status=%#.8x\n", status);
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  WGPUInstance instance = NULL;
  WGPUAdapter adapter = NULL;
  WGPUDevice device = NULL;
  WGPUShaderModule shader_module = NULL;
  WGPUBuffer staging_buffer = NULL;
  WGPUBuffer storage_buffer = NULL;
  WGPUComputePipeline compute_pipeline = NULL;
  WGPUBindGroupLayout bind_group_layout = NULL;
  WGPUBindGroup bind_group = NULL;
  int ret = EXIT_SUCCESS;

#define ASSERT_CHECK(expr)                                                     \
  do {                                                                         \
    if (!(expr)) {                                                             \
      ret = EXIT_FAILURE;                                                      \
      printf(LOG_PREFIX " assert failed (%s): %s:%d\n", #expr, __FILE__,       \
             __LINE__);                                                        \
      goto cleanup_and_exit;                                                   \
    }                                                                          \
  } while (0)

  frmwrk_setup_logging(WGPULogLevel_Warn);

  uint32_t numbers[] = {1, 2, 3, 4};
  uint32_t numbers_size = sizeof(numbers);
  uint32_t numbers_length = numbers_size / sizeof(uint32_t);

  instance = wgpuCreateInstance(&(const WGPUInstanceDescriptor){0});
  ASSERT_CHECK(instance);

  wgpuInstanceRequestAdapter(instance, NULL, handle_request_adapter,
                             (void *)&adapter);
  ASSERT_CHECK(adapter);

  wgpuAdapterRequestDevice(adapter, NULL, handle_request_device,
                           (void *)&device);
  ASSERT_CHECK(device);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  ASSERT_CHECK(queue);

  wgpuDeviceSetUncapturedErrorCallback(device, handle_uncaptured_error, NULL);
  wgpuDeviceSetDeviceLostCallback(device, handle_device_lost, NULL);

  shader_module = frmwrk_load_shader_module(device, "shader.wgsl");
  ASSERT_CHECK(shader_module);

  staging_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = "staging_buffer",
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .size = numbers_size,
                  .mappedAtCreation = false,
              });
  ASSERT_CHECK(staging_buffer);

  storage_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = "storage_buffer",
                  .usage = WGPUBufferUsage_Storage | WGPUBufferUsage_CopyDst |
                           WGPUBufferUsage_CopySrc,
                  .size = numbers_size,
                  .mappedAtCreation = false,
              });
  ASSERT_CHECK(storage_buffer);

  compute_pipeline = wgpuDeviceCreateComputePipeline(
      device, &(const WGPUComputePipelineDescriptor){
                  .label = "compute_pipeline",
                  .compute =
                      (const WGPUProgrammableStageDescriptor){
                          .module = shader_module,
                          .entryPoint = "main",
                      },
              });
  ASSERT_CHECK(compute_pipeline);

  bind_group_layout =
      wgpuComputePipelineGetBindGroupLayout(compute_pipeline, 0);
  ASSERT_CHECK(bind_group_layout);

  bind_group = wgpuDeviceCreateBindGroup(
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
  ASSERT_CHECK(bind_group);

  WGPUCommandEncoder command_encoder = wgpuDeviceCreateCommandEncoder(
      device, &(const WGPUCommandEncoderDescriptor){
                  .label = "command_encoder",
              });
  ASSERT_CHECK(command_encoder);

  WGPUComputePassEncoder compute_pass_encoder =
      wgpuCommandEncoderBeginComputePass(command_encoder,
                                         &(const WGPUComputePassDescriptor){
                                             .label = "compute_pass",
                                         });
  ASSERT_CHECK(compute_pass_encoder);

  wgpuComputePassEncoderSetPipeline(compute_pass_encoder, compute_pipeline);
  wgpuComputePassEncoderSetBindGroup(compute_pass_encoder, 0, bind_group, 0,
                                     NULL);
  wgpuComputePassEncoderDispatchWorkgroups(compute_pass_encoder, numbers_length,
                                           1, 1);
  wgpuComputePassEncoderEnd(compute_pass_encoder);
  // compute_pass_encoder is unusable after wgpuComputePassEncoderEnd()
  compute_pass_encoder = NULL;

  wgpuCommandEncoderCopyBufferToBuffer(command_encoder, storage_buffer, 0,
                                       staging_buffer, 0, numbers_size);

  WGPUCommandBuffer command_buffer = wgpuCommandEncoderFinish(
      command_encoder, &(const WGPUCommandBufferDescriptor){
                           .label = "command_buffer",
                       });
  ASSERT_CHECK(command_buffer);
  // command_encoder is unusable after wgpuCommandEncoderFinish()
  command_encoder = NULL;

  wgpuQueueWriteBuffer(queue, storage_buffer, 0, &numbers, numbers_size);
  wgpuQueueSubmit(queue, 1, &command_buffer);

  wgpuBufferMapAsync(staging_buffer, WGPUMapMode_Read, 0, numbers_size,
                     handle_buffer_map, NULL);
  wgpuDevicePoll(device, true, NULL);

  uint32_t *buf =
      (uint32_t *)wgpuBufferGetMappedRange(staging_buffer, 0, numbers_size);
  ASSERT_CHECK(buf);

  printf("times: [%d, %d, %d, %d]\n", buf[0], buf[1], buf[2], buf[3]);

  wgpuBufferUnmap(staging_buffer);
  // mapped buffer is unusable after wgpuBufferUnmap()
  buf = NULL;

cleanup_and_exit:
  if (bind_group)
    wgpuBindGroupDrop(bind_group);
  if (bind_group_layout)
    wgpuBindGroupLayoutDrop(bind_group_layout);
  if (compute_pipeline)
    wgpuComputePipelineDrop(compute_pipeline);
  if (storage_buffer)
    wgpuBufferDrop(storage_buffer);
  if (staging_buffer)
    wgpuBufferDrop(staging_buffer);
  if (shader_module)
    wgpuShaderModuleDrop(shader_module);
  if (device)
    wgpuDeviceDrop(device);
  if (adapter)
    wgpuAdapterDrop(adapter);
  if (instance)
    wgpuInstanceDrop(instance);

  return ret;
}
