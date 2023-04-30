#include "framework.h"
#include "stb_image_write.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define LOG_PREFIX "[capture]"

#define COPY_BYTES_PER_ROW_ALIGNMENT 256

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
  const size_t align = COPY_BYTES_PER_ROW_ALIGNMENT;
  const size_t padded_bytes_per_row_padding =
      (align - unpadded_bytes_per_row % align) % align;
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
  WGPUInstance instance = NULL;
  WGPUAdapter adapter = NULL;
  WGPUDevice device = NULL;
  WGPUBuffer output_buffer = NULL;
  WGPUTexture texture = NULL;
  WGPUTextureView texture_view = NULL;
  uint8_t *buf = NULL;
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

  const size_t width = 100;
  const size_t height = 200;

  frmwrk_setup_logging(WGPULogLevel_Warn);

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

  BufferDimensions buffer_dimensions = {0};
  buffer_dimensions_init(&buffer_dimensions, width, height);

  const size_t buffer_size =
      (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height);

  output_buffer = wgpuDeviceCreateBuffer(
      device, &(const WGPUBufferDescriptor){
                  .label = "output_buffer",
                  .size = buffer_size,
                  .usage = WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                  .mappedAtCreation = false,
              });
  ASSERT_CHECK(output_buffer);

  const WGPUExtent3D texture_extent = (const WGPUExtent3D){
      .width = buffer_dimensions.width,
      .height = buffer_dimensions.height,
      .depthOrArrayLayers = 1,
  };

  texture = wgpuDeviceCreateTexture(
      device,
      &(const WGPUTextureDescriptor){
          .label = "texture",
          .size = texture_extent,
          .mipLevelCount = 1,
          .sampleCount = 1,
          .dimension = WGPUTextureDimension_2D,
          .format = WGPUTextureFormat_RGBA8UnormSrgb,
          .usage = WGPUTextureUsage_RenderAttachment | WGPUTextureUsage_CopySrc,
      });
  ASSERT_CHECK(texture);

  texture_view = wgpuTextureCreateView(texture, NULL);
  ASSERT_CHECK(texture_view);

  WGPUCommandEncoder command_encoder = wgpuDeviceCreateCommandEncoder(
      device, &(const WGPUCommandEncoderDescriptor){
                  .label = "command_encoder",
              });
  ASSERT_CHECK(command_encoder);

  WGPURenderPassEncoder render_pass_encoder = wgpuCommandEncoderBeginRenderPass(
      command_encoder, &(const WGPURenderPassDescriptor){
                           .label = "rende_pass_encoder",
                           .colorAttachmentCount = 1,
                           .colorAttachments =
                               (const WGPURenderPassColorAttachment[]){
                                   (const WGPURenderPassColorAttachment){
                                       .view = texture_view,
                                       .loadOp = WGPULoadOp_Clear,
                                       .storeOp = WGPUStoreOp_Store,
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
  ASSERT_CHECK(render_pass_encoder);

  wgpuRenderPassEncoderEnd(render_pass_encoder);

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
                           .label = "command_buffer",
                       });
  ASSERT_CHECK(command_buffer);

  wgpuQueueSubmit(queue, 1, (const WGPUCommandBuffer[]){command_buffer});

  wgpuBufferMapAsync(output_buffer, WGPUMapMode_Read, 0, buffer_size,
                     handle_buffer_map, NULL);
  wgpuDevicePoll(device, true, NULL);

  buf = (uint8_t *)wgpuBufferGetConstMappedRange(output_buffer, 0, buffer_size);
  ASSERT_CHECK(buf);

  ASSERT_CHECK(stbi_write_png("red.png", buffer_dimensions.width,
                              buffer_dimensions.height, 4, buf,
                              buffer_dimensions.padded_bytes_per_row));

cleanup_and_exit:
  if (buf) {
    wgpuBufferUnmap(output_buffer);
    // mapped buf is unusable after wgpuBufferUnmap()
    buf = NULL;
  }
  if (texture_view)
    wgpuTextureViewDrop(texture_view);
  if (texture)
    wgpuTextureDrop(texture);
  if (output_buffer)
    wgpuBufferDrop(output_buffer);
  if (device)
    wgpuDeviceDrop(device);
  if (adapter)
    wgpuAdapterDrop(adapter);
  if (instance)
    wgpuInstanceDrop(instance);

  return ret;
}
