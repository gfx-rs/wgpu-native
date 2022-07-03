#include "webgpu-headers/webgpu.h"
#include "wgpu.h"

#include "framework.h"
#include "unused.h"
#include <stdio.h>
#include <stdlib.h>

#define WGPU_TARGET_MACOS 1
#define WGPU_TARGET_LINUX_X11 2
#define WGPU_TARGET_WINDOWS 3
#define WGPU_TARGET_LINUX_WAYLAND 4

#if WGPU_TARGET == WGPU_TARGET_MACOS
#include <Foundation/Foundation.h>
#include <QuartzCore/CAMetalLayer.h>
#endif

#include <GLFW/glfw3.h>
#if WGPU_TARGET == WGPU_TARGET_MACOS
#define GLFW_EXPOSE_NATIVE_COCOA
#elif WGPU_TARGET == WGPU_TARGET_LINUX_X11
#define GLFW_EXPOSE_NATIVE_X11
#elif WGPU_TARGET == WGPU_TARGET_LINUX_WAYLAND
#define GLFW_EXPOSE_NATIVE_WAYLAND
#elif WGPU_TARGET == WGPU_TARGET_WINDOWS
#define GLFW_EXPOSE_NATIVE_WIN32
#endif
#include <GLFW/glfw3native.h>

static void handle_device_lost(WGPUDeviceLostReason reason, char const * message, void * userdata)
{
  UNUSED(userdata);

  printf("DEVICE LOST (%d): %s\n", reason, message);
}

static void handle_uncaptured_error(WGPUErrorType type, char const * message, void * userdata)
{
  UNUSED(userdata);

  printf("UNCAPTURED ERROR (%d): %s\n", type, message);
}

static void handleGlfwKey(GLFWwindow *window, int key, int scancode, int action, int mods) {
  UNUSED(window);
  UNUSED(scancode);
  UNUSED(mods);

  if (key == GLFW_KEY_R && (action == GLFW_PRESS || action == GLFW_REPEAT)) {
    WGPUGlobalReport report;
    wgpuGenerateReport(&report);
    printGlobalReport(report);
  }
}

int main() {
  initializeLog();

  if (!glfwInit()) {
    printf("Cannot initialize glfw");
    return 1;
  }

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  GLFWwindow *window = glfwCreateWindow(640, 480, "wgpu with glfw", NULL, NULL);

  if (!window) {
    printf("Cannot create window");
    return 1;
  }

  WGPUSurface surface;

#if WGPU_TARGET == WGPU_TARGET_MACOS
  {
    id metal_layer = NULL;
    NSWindow *ns_window = glfwGetCocoaWindow(window);
    [ns_window.contentView setWantsLayer:YES];
    metal_layer = [CAMetalLayer layer];
    [ns_window.contentView setLayer:metal_layer];
    surface = wgpuInstanceCreateSurface(
        NULL,
        &(WGPUSurfaceDescriptor){
            .label = NULL,
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    WGPUSurfaceDescriptorFromMetalLayer){
                    .chain =
                        (WGPUChainedStruct){
                            .next = NULL,
                            .sType = WGPUSType_SurfaceDescriptorFromMetalLayer,
                        },
                    .layer = metal_layer,
                },
        });
  }
#elif WGPU_TARGET == WGPU_TARGET_LINUX_X11
  {
    Display *x11_display = glfwGetX11Display();
    Window x11_window = glfwGetX11Window(window);
    surface = wgpuInstanceCreateSurface(
        NULL,
        &(WGPUSurfaceDescriptor){
            .label = NULL,
            .nextInChain =
                (const WGPUChainedStruct *)&(WGPUSurfaceDescriptorFromXlibWindow){
                    .chain =
                        (WGPUChainedStruct){
                            .next = NULL,
                            .sType = WGPUSType_SurfaceDescriptorFromXlibWindow,
                        },
                    .display = x11_display,
                    .window = x11_window,
                },
        });
  }
#elif WGPU_TARGET == WGPU_TARGET_LINUX_WAYLAND
  {
    struct wl_display *wayland_display = glfwGetWaylandDisplay();
    struct wl_surface *wayland_surface = glfwGetWaylandWindow(window);
    surface = wgpuInstanceCreateSurface(
        NULL,
        &(WGPUSurfaceDescriptor){
            .label = NULL,
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    WGPUSurfaceDescriptorFromWaylandSurface){
                    .chain =
                        (WGPUChainedStruct){
                            .next = NULL,
                            .sType =
                                WGPUSType_SurfaceDescriptorFromWaylandSurface,
                        },
                    .display = wayland_display,
                    .surface = wayland_surface,
                },
        });
  }
#elif WGPU_TARGET == WGPU_TARGET_WINDOWS
  {
    HWND hwnd = glfwGetWin32Window(window);
    HINSTANCE hinstance = GetModuleHandle(NULL);
    surface = wgpuInstanceCreateSurface(
        NULL,
        &(WGPUSurfaceDescriptor){
            .label = NULL,
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    WGPUSurfaceDescriptorFromWindowsHWND){
                    .chain =
                        (WGPUChainedStruct){
                            .next = NULL,
                            .sType = WGPUSType_SurfaceDescriptorFromWindowsHWND,
                        },
                    .hinstance = hinstance,
                    .hwnd = hwnd,
                },
        });
  }
#else
#error "Unsupported WGPU_TARGET"
#endif

  WGPUAdapter adapter;
  wgpuInstanceRequestAdapter(NULL,
                             &(WGPURequestAdapterOptions){
                                 .nextInChain = NULL,
                                 .compatibleSurface = surface,
                             },
                             request_adapter_callback, (void *)&adapter);

  printAdapterFeatures(adapter);

  WGPUDevice device;
  wgpuAdapterRequestDevice(
      adapter,
      &(WGPUDeviceDescriptor){
          .nextInChain =
              (const WGPUChainedStruct *)&(WGPUDeviceExtras){
                  .chain =
                      (WGPUChainedStruct){
                          .next = NULL,
                          .sType = WGPUSType_DeviceExtras,
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

  wgpuDeviceSetUncapturedErrorCallback(device, handle_uncaptured_error, NULL);
  wgpuDeviceSetDeviceLostCallback(device, handle_device_lost, NULL);

  WGPUShaderModuleDescriptor shaderSource = load_wgsl("shader.wgsl");
  WGPUShaderModule shader = wgpuDeviceCreateShaderModule(device, &shaderSource);

  WGPUPipelineLayout pipelineLayout = wgpuDeviceCreatePipelineLayout(
      device, &(WGPUPipelineLayoutDescriptor){.bindGroupLayouts = NULL,
                                              .bindGroupLayoutCount = 0});

  WGPUTextureFormat swapChainFormat =
      wgpuSurfaceGetPreferredFormat(surface, adapter);

  WGPURenderPipeline pipeline = wgpuDeviceCreateRenderPipeline(
      device,
      &(WGPURenderPipelineDescriptor){
          .label = "Render pipeline",
          .layout = pipelineLayout,
          .vertex =
              (WGPUVertexState){
                  .module = shader,
                  .entryPoint = "vs_main",
                  .bufferCount = 0,
                  .buffers = NULL,
              },
          .primitive =
              (WGPUPrimitiveState){
                  .topology = WGPUPrimitiveTopology_TriangleList,
                  .stripIndexFormat = WGPUIndexFormat_Undefined,
                  .frontFace = WGPUFrontFace_CCW,
                  .cullMode = WGPUCullMode_None},
          .multisample =
              (WGPUMultisampleState){
                  .count = 1,
                  .mask = ~0,
                  .alphaToCoverageEnabled = false,
              },
          .fragment =
              &(WGPUFragmentState){
                  .module = shader,
                  .entryPoint = "fs_main",
                  .targetCount = 1,
                  .targets =
                      &(WGPUColorTargetState){
                          .format = swapChainFormat,
                          .blend =
                              &(WGPUBlendState){
                                  .color =
                                      (WGPUBlendComponent){
                                          .srcFactor = WGPUBlendFactor_One,
                                          .dstFactor = WGPUBlendFactor_Zero,
                                          .operation = WGPUBlendOperation_Add,
                                      },
                                  .alpha =
                                      (WGPUBlendComponent){
                                          .srcFactor = WGPUBlendFactor_One,
                                          .dstFactor = WGPUBlendFactor_Zero,
                                          .operation = WGPUBlendOperation_Add,
                                      }},
                          .writeMask = WGPUColorWriteMask_All},
              },
          .depthStencil = NULL,
      });

  int prevWidth = 0;
  int prevHeight = 0;
  glfwGetWindowSize(window, &prevWidth, &prevHeight);

  WGPUSwapChain swapChain =
      wgpuDeviceCreateSwapChain(device, surface,
                                &(WGPUSwapChainDescriptor){
                                    .usage = WGPUTextureUsage_RenderAttachment,
                                    .format = swapChainFormat,
                                    .width = prevWidth,
                                    .height = prevHeight,
                                    .presentMode = WGPUPresentMode_Fifo,
                                });

  glfwSetKeyCallback(window, handleGlfwKey);

  while (!glfwWindowShouldClose(window)) {

    WGPUTextureView nextTexture = NULL;

    for (int attempt = 0; attempt < 2; attempt++) {

      int width = 0;
      int height = 0;
      glfwGetWindowSize(window, &width, &height);

      if (width != prevWidth || height != prevHeight) {
        prevWidth = width;
        prevHeight = height;

        swapChain = wgpuDeviceCreateSwapChain(
            device, surface,
            &(WGPUSwapChainDescriptor){
                .usage = WGPUTextureUsage_RenderAttachment,
                .format = swapChainFormat,
                .width = prevWidth,
                .height = prevHeight,
                .presentMode = WGPUPresentMode_Fifo,
            });
      }

      nextTexture = wgpuSwapChainGetCurrentTextureView(swapChain);

      if (attempt == 0 && !nextTexture) {
        printf("wgpuSwapChainGetCurrentTextureView() failed; trying to create a new swap chain...\n");
        prevWidth = 0;
        prevHeight = 0;
        continue;
      }

      break;
    }

    if (!nextTexture) {
      printf("Cannot acquire next swap chain texture\n");
      return 1;
    }

    WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(
        device, &(WGPUCommandEncoderDescriptor){.label = "Command Encoder"});

    WGPURenderPassEncoder renderPass = wgpuCommandEncoderBeginRenderPass(
        encoder, &(WGPURenderPassDescriptor){
                     .colorAttachments =
                         &(WGPURenderPassColorAttachment){
                             .view = nextTexture,
                             .resolveTarget = 0,
                             .loadOp = WGPULoadOp_Clear,
                             .storeOp = WGPUStoreOp_Store,
                             .clearValue =
                                 (WGPUColor){
                                     .r = 0.0,
                                     .g = 1.0,
                                     .b = 0.0,
                                     .a = 1.0,
                                 },
                         },
                     .colorAttachmentCount = 1,
                     .depthStencilAttachment = NULL,
                 });

    wgpuRenderPassEncoderSetPipeline(renderPass, pipeline);
    wgpuRenderPassEncoderDraw(renderPass, 3, 1, 0, 0);
    wgpuRenderPassEncoderEnd(renderPass);
    wgpuTextureViewDrop(nextTexture);

    WGPUQueue queue = wgpuDeviceGetQueue(device);
    WGPUCommandBuffer cmdBuffer = wgpuCommandEncoderFinish(
        encoder, &(WGPUCommandBufferDescriptor){.label = NULL});
    wgpuQueueSubmit(queue, 1, &cmdBuffer);
    wgpuSwapChainPresent(swapChain);

    glfwPollEvents();
  }

  glfwDestroyWindow(window);
  glfwTerminate();

  return 0;
}
