#include "framework.h"
#include "stb_image_write.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define LOG_PREFIX "[capture]"

const size_t IMAGE_WIDTH = 100;
const size_t IMAGE_HEIGHT = 200;
const size_t COPY_BYTES_PER_ROW_ALIGNMENT = 256;

static void handle_request_adapter(WGPURequestAdapterStatus status,
                                   WGPUAdapter adapter, WGPUStringView message,
                                   void *userdata1, void *userdata2) {
  UNUSED(status)
  UNUSED(message)
  UNUSED(userdata2)
  *(WGPUAdapter *)userdata1 = adapter;
}
static void handle_request_device(WGPURequestDeviceStatus status,
                                  WGPUDevice device, WGPUStringView message,
                                  void *userdata1, void *userdata2) {
  UNUSED(status)
  UNUSED(message)
  UNUSED(userdata2)
  *(WGPUDevice *)userdata1 = device;
}
static void handle_buffer_map(WGPUMapAsyncStatus status, 
                              WGPUStringView message,
                              void *userdata1, void *userdata2) {
  UNUSED(message)
  UNUSED(userdata1)
  UNUSED(userdata2)
  printf(LOG_PREFIX " buffer_map status=%#.8x\n", status);
}

typedef struct BufferDimensions {
  size_t width;
  size_t height;
  size_t unpadded_bytes_per_row;
  size_t padded_bytes_per_row;
} BufferDimensions;

static void buffer_dimensions_init(BufferDimensions *r, size_t width,
                                   size_t height) {
  assert(r);

  const size_t bytes_per_pixel = sizeof(uint32_t);
  const size_t unpadded_bytes_per_row = width * bytes_per_pixel;
  const size_t padded_bytes_per_row_padding =
      (COPY_BYTES_PER_ROW_ALIGNMENT -
       unpadded_bytes_per_row % COPY_BYTES_PER_ROW_ALIGNMENT) %
      COPY_BYTES_PER_ROW_ALIGNMENT;
  const size_t padded_bytes_per_row =
      unpadded_bytes_per_row + padded_bytes_per_row_padding;

  r->width = width;
  r->height = height;
  r->unpadded_bytes_per_row = unpadded_bytes_per_row;
  r->padded_bytes_per_row = padded_bytes_per_row;
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  frmwrk_setup_logging(WGPULogLevel_Warn);

  WGPUInstance instance = wgpuCreateInstance(NULL);
  assert(instance);

  WGPUAdapter adapter = NULL;
  wgpuInstanceRequestAdapter(instance, NULL,
                             (const WGPURequestAdapterCallbackInfo){
                                 .callback = handle_request_adapter,
                                 .userdata1 = &adapter
                             });
  assert(adapter);

  WGPUDevice device = NULL;
  wgpuAdapterRequestDevice(adapter, NULL,
                           (const WGPURequestDeviceCallbackInfo){ 
                               .callback = handle_request_device,
                               .userdata1 = &device
                           });
  assert(device);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  assert(queue);

  BufferDimensions buffer_dimensions = {0};
  buffer_dimensions_init(&buffer_dimensions, IMAGE_WIDTH, IMAGE_HEIGHT);

  const size_t buffer_size =
      (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height);

  WGPUBuffer output_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = {"output_buffer", WGPU_STRLEN},
                  .size = buffer_size,
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .mappedAtCreation = false,
              });
  assert(output_buffer);

  const WGPUExtent3D texture_extent = (const WGPUExtent3D){
      .width = buffer_dimensions.width,
      .height = buffer_dimensions.height,
      .depthOrArrayLayers = 1,
  };

  WGPUTexture texture = wgpuDeviceCreateTexture(
      device,
      &(const WGPUTextureDescriptor){
          .label = {"texture", WGPU_STRLEN},
          .size = texture_extent,
          .mipLevelCount = 1,
          .sampleCount = 1,
          .dimension = WGPUTextureDimension_2D,
          .format = WGPUTextureFormat_RGBA8UnormSrgb,
          .usage = WGPUTextureUsage_RenderAttachment | WGPUTextureUsage_CopySrc,
      });
  assert(texture);
  WGPUTextureView texture_view = wgpuTextureCreateView(texture, NULL);
  assert(texture_view);

  WGPUCommandEncoder command_encoder = wgpuDeviceCreateCommandEncoder(
      device, &(const WGPUCommandEncoderDescriptor){
                  .label = {"command_encoder", WGPU_STRLEN},
              });
  assert(command_encoder);

  WGPURenderPassEncoder render_pass_encoder = wgpuCommandEncoderBeginRenderPass(
      command_encoder, &(const WGPURenderPassDescriptor){
                           .label = {"rende_pass_encoder", WGPU_STRLEN},
                           .colorAttachmentCount = 1,
                           .colorAttachments =
                               (const WGPURenderPassColorAttachment[]){
                                   (const WGPURenderPassColorAttachment){
                                       .view = texture_view,
                                       .loadOp = WGPULoadOp_Clear,
                                       .storeOp = WGPUStoreOp_Store,
                                       .depthSlice = WGPU_DEPTH_SLICE_UNDEFINED,
                                       .clearValue =
                                           (const WGPUColor){
                                               .r = 1,
                                               .g = 0,
                                               .b = 0,
                                               .a = 1,
                                           },
                                   },
                               },
                       });
  assert(render_pass_encoder);

  wgpuRenderPassEncoderEnd(render_pass_encoder);
  wgpuRenderPassEncoderRelease(render_pass_encoder);

  wgpuCommandEncoderCopyTextureToBuffer(
      command_encoder,
      &(const WGPUImageCopyTexture){
          .texture = texture,
          .mipLevel = 0,
          .origin = (const WGPUOrigin3D){.x = 0, .y = 0, .z = 0},
          .aspect = WGPUTextureAspect_All,
      },
      &(const WGPUImageCopyBuffer){
          .buffer = output_buffer,
          .layout =
              (const WGPUTextureDataLayout){
                  .offset = 0,
                  .bytesPerRow = buffer_dimensions.padded_bytes_per_row,
                  .rowsPerImage = WGPU_COPY_STRIDE_UNDEFINED,
              },
      },
      &texture_extent);

  WGPUCommandBuffer command_buffer = wgpuCommandEncoderFinish(
      command_encoder, &(const WGPUCommandBufferDescriptor){
                           .label = {"command_buffer", WGPU_STRLEN},
                       });
  assert(command_buffer);

  wgpuQueueSubmit(queue, 1, (const WGPUCommandBuffer[]){command_buffer});

  wgpuBufferMapAsync(output_buffer, WGPUMapMode_Read, 0, buffer_size,
                     (const WGPUBufferMapCallbackInfo){
                         .callback = handle_buffer_map
                     });
  wgpuDevicePoll(device, true, NULL);

  uint8_t *buf =
      (uint8_t *)wgpuBufferGetConstMappedRange(output_buffer, 0, buffer_size);
  assert(buf);

  assert(stbi_write_png("red.png", buffer_dimensions.width,
                        buffer_dimensions.height, 4, buf,
                        buffer_dimensions.padded_bytes_per_row));

  wgpuBufferUnmap(output_buffer);
  wgpuCommandBufferRelease(command_buffer);
  wgpuCommandEncoderRelease(command_encoder);
  wgpuTextureViewRelease(texture_view);
  wgpuTextureRelease(texture);
  wgpuBufferRelease(output_buffer);
  wgpuQueueRelease(queue);
  wgpuDeviceRelease(device);
  wgpuAdapterRelease(adapter);
  wgpuInstanceRelease(instance);
  return EXIT_SUCCESS;
}
