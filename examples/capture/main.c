#include "framework.h"
#include "helper.h"
#include "unused.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <stdio.h>

static void handle_device_lost(WGPUDeviceLostReason reason, char const *message,
                               void *userdata) {
  UNUSED(userdata);

  printf("DEVICE LOST (%d): %s\n", reason, message);
}

static void handle_uncaptured_error(WGPUErrorType type, char const *message,
                                    void *userdata) {
  UNUSED(userdata);

  printf("UNCAPTURED ERROR (%d): %s\n", type, message);
}

int main(int argc, char *argv[]) {
  UNUSED(argc);
  UNUSED(argv);

  initializeLog();

  int width = 100;
  int height = 200;

  WGPUInstance instance = wgpuCreateInstance(&(WGPUInstanceDescriptor) {.nextInChain = NULL});

  WGPUAdapter adapter;
  wgpuInstanceRequestAdapter(instance,
                             &(WGPURequestAdapterOptions){
                                 .nextInChain = NULL,
                                 .compatibleSurface = NULL,
                             },
                             request_adapter_callback, (void *)&adapter);

  WGPUDevice device;
  wgpuAdapterRequestDevice(adapter,
                           &(WGPUDeviceDescriptor){
                               .nextInChain = NULL,
                               .label = "Device",
                               .requiredLimits =
                                   &(WGPURequiredLimits){
                                       .nextInChain = NULL,
                                       .limits = WGPULimits_DEFAULT,
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

  BufferDimensions bufferDimensions = buffer_dimensions_new(width, height);
  uint64_t bufferSize =
      bufferDimensions.padded_bytes_per_row * bufferDimensions.height;
  WGPUBuffer outputBuffer = wgpuDeviceCreateBuffer(
      device, &(WGPUBufferDescriptor){
                  .nextInChain = NULL,
                  .label = "Output Buffer",
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .size = bufferSize,
                  .mappedAtCreation = false,
              });

  WGPUExtent3D textureExtent = (WGPUExtent3D){
      .width = bufferDimensions.width,
      .height = bufferDimensions.height,
      .depthOrArrayLayers = 1,
  };
  WGPUTexture texture = wgpuDeviceCreateTexture(
      device,
      &(WGPUTextureDescriptor){
          .nextInChain = NULL,
          .label = NULL,
          .size = textureExtent,
          .mipLevelCount = 1,
          .sampleCount = 1,
          .dimension = WGPUTextureDimension_2D,
          .format = WGPUTextureFormat_RGBA8UnormSrgb,
          .usage = WGPUTextureUsage_RenderAttachment | WGPUTextureUsage_CopySrc,
      });

  WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(
      device, &(WGPUCommandEncoderDescriptor){.label = NULL});

  WGPUTextureView outputAttachment = wgpuTextureCreateView(texture, NULL);

  WGPURenderPassEncoder renderPass = wgpuCommandEncoderBeginRenderPass(
      encoder, &(WGPURenderPassDescriptor){
                   .colorAttachments =
                       &(WGPURenderPassColorAttachment){
                           .view = outputAttachment,
                           .resolveTarget = NULL,
                           .loadOp = WGPULoadOp_Clear,
                           .storeOp = WGPUStoreOp_Store,
                           .clearValue =
                               (WGPUColor){
                                   .r = 1.0,
                                   .g = 0.0,
                                   .b = 0.0,
                                   .a = 1.0,
                               },
                       },
                   .colorAttachmentCount = 1,
                   .depthStencilAttachment = NULL,
               });
  wgpuRenderPassEncoderEnd(renderPass);

  wgpuCommandEncoderCopyTextureToBuffer(
      encoder,
      &(WGPUImageCopyTexture){
          .texture = texture,
          .mipLevel = 0,
          .origin =
              (WGPUOrigin3D){
                  .x = 0,
                  .y = 0,
                  .z = 0,
              },
      },
      &(WGPUImageCopyBuffer){
          .buffer = outputBuffer,
          .layout =
              (WGPUTextureDataLayout){
                  .offset = 0,
                  .bytesPerRow = bufferDimensions.padded_bytes_per_row,
                  .rowsPerImage = WGPU_COPY_STRIDE_UNDEFINED,
              }},
      &textureExtent);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  WGPUCommandBuffer cmdBuffer = wgpuCommandEncoderFinish(
      encoder, &(WGPUCommandBufferDescriptor){.label = NULL});
  wgpuQueueSubmit(queue, 1, &cmdBuffer);

  wgpuBufferMapAsync(outputBuffer, WGPUMapMode_Read, 0, bufferSize,
                     readBufferMap, NULL);
  wgpuDevicePoll(device, true, NULL);

  uint8_t *data =
      (uint8_t *)wgpuBufferGetMappedRange(outputBuffer, 0, bufferSize);
  const char *filename = "red.png";
  save_png(filename, data, &bufferDimensions);

  wgpuBufferUnmap(outputBuffer);

  return 0;
}
