#include "framework.h"
#include "helper.h"
#include "unused.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"

int main(int argc, char *argv[]) {
  UNUSED(argc);
  UNUSED(argv);

  initializeLog();

  int width = 100;
  int height = 200;

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
                          .maxBindGroups = 1,
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

  BufferDimensions bufferDimensions = buffer_dimensions_new(width, height);
  uint64_t bufferSize = bufferDimensions.padded_bytes_per_row * bufferDimensions.height;
  WGPUBuffer outputBuffer =
      wgpuDeviceCreateBuffer(device, &(WGPUBufferDescriptor){
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
      device, &(WGPUTextureDescriptor){
                  .nextInChain = NULL,
                  .label = NULL,
                  .size = textureExtent,
                  .mipLevelCount = 1,
                  .sampleCount = 1,
                  .dimension = WGPUTextureDimension_2D,
                  .format = WGPUTextureFormat_RGBA8UnormSrgb,
                  .usage = WGPUTextureUsage_RenderAttachment | WGPUTextureUsage_CopySrc,
              });

  WGPUCommandEncoder encoder =
      wgpuDeviceCreateCommandEncoder(device, &(WGPUCommandEncoderDescriptor){.label = NULL});

  WGPUTextureView outputAttachment =
      wgpuTextureCreateView(texture, &(WGPUTextureViewDescriptor){
                                         .nextInChain = NULL,
                                         .label = NULL,
                                         .format = WGPUTextureFormat_Undefined,
                                         .dimension = WGPUTextureViewDimension_Undefined,
                                         .aspect = WGPUTextureAspect_All,
                                         .arrayLayerCount = WGPU_ARRAY_LAYER_COUNT_UNDEFINED,
                                         .baseArrayLayer = 0,
                                         .baseMipLevel = 0,
                                         .mipLevelCount = WGPU_MIP_LEVEL_COUNT_UNDEFINED,
                                     });

  WGPURenderPassEncoder renderPass =
      wgpuCommandEncoderBeginRenderPass(encoder, &(WGPURenderPassDescriptor){
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
      &(WGPUImageCopyBuffer){.buffer = outputBuffer,
                             .layout =
                                 (WGPUTextureDataLayout){
                                     .offset = 0,
                                     .bytesPerRow = bufferDimensions.padded_bytes_per_row,
                                     .rowsPerImage = WGPU_COPY_STRIDE_UNDEFINED,
                                 }},
      &textureExtent);

  WGPUQueue queue = wgpuDeviceGetQueue(device);
  WGPUCommandBuffer cmdBuffer =
      wgpuCommandEncoderFinish(encoder, &(WGPUCommandBufferDescriptor){.label = NULL});
  wgpuQueueSubmit(queue, 1, &cmdBuffer);

  wgpuBufferMapAsync(outputBuffer, WGPUMapMode_Read, 0, bufferSize, readBufferMap, NULL);
  wgpuDevicePoll(device, true, NULL);

  uint8_t *data = (uint8_t *)wgpuBufferGetMappedRange(outputBuffer, 0, bufferSize);
  const char *filename = "red.png";
  save_png(filename, data, &bufferDimensions);

  wgpuBufferUnmap(outputBuffer);

  return 0;
}
