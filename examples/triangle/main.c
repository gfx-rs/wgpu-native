#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(WGPU_TARGET_MACOS)
#include <Foundation/Foundation.h>
#include <QuartzCore/CAMetalLayer.h>
#endif

#include <GLFW/glfw3.h>
#if defined(WGPU_TARGET_MACOS)
#define GLFW_EXPOSE_NATIVE_COCOA
#elif defined(WGPU_TARGET_LINUX_X11)
#define GLFW_EXPOSE_NATIVE_X11
#elif defined(WGPU_TARGET_LINUX_WAYLAND)
#define GLFW_EXPOSE_NATIVE_WAYLAND
#elif defined(WGPU_TARGET_WINDOWS)
#define GLFW_EXPOSE_NATIVE_WIN32
#endif
#include <GLFW/glfw3native.h>

#define LOG_PREFIX "[triangle]"

struct demo {
  WGPUInstance instance;
  WGPUSurface surface;
  WGPUAdapter adapter;
  WGPUDevice device;
  WGPUSurfaceConfiguration config;
};

static void handle_request_adapter(WGPURequestAdapterStatus status,
                                   WGPUAdapter adapter, char const *message,
                                   void *userdata) {
  if (status == WGPURequestAdapterStatus_Success) {
    struct demo *demo = userdata;
    demo->adapter = adapter;
  } else {
    printf(LOG_PREFIX " request_adapter status=%#.8x message=%s\n", status,
           message);
  }
}
static void handle_request_device(WGPURequestDeviceStatus status,
                                  WGPUDevice device, char const *message,
                                  void *userdata) {
  if (status == WGPURequestDeviceStatus_Success) {
    struct demo *demo = userdata;
    demo->device = device;
  } else {
    printf(LOG_PREFIX " request_device status=%#.8x message=%s\n", status,
           message);
  }
}
static void handle_glfw_key(GLFWwindow *window, int key, int scancode,
                            int action, int mods) {
  UNUSED(scancode)
  UNUSED(mods)
  if (key == GLFW_KEY_R && (action == GLFW_PRESS || action == GLFW_REPEAT)) {
    struct demo *demo = glfwGetWindowUserPointer(window);
    if (!demo || !demo->instance)
      return;

    WGPUGlobalReport report;
    wgpuGenerateReport(demo->instance, &report);
    frmwrk_print_global_report(report);
  }
}
static void handle_glfw_framebuffer_size(GLFWwindow *window, int width,
                                         int height) {
  if (width == 0 && height == 0) {
    return;
  }

  struct demo *demo = glfwGetWindowUserPointer(window);
  if (!demo)
    return;

  demo->config.width = width;
  demo->config.height = height;

  wgpuSurfaceConfigure(demo->surface, &demo->config);
}

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  struct demo demo = {0};
  GLFWwindow *window = NULL;
  WGPUQueue queue = NULL;
  WGPUShaderModule shader_module = NULL;
  WGPUPipelineLayout pipeline_layout = NULL;
  WGPURenderPipeline render_pipeline = NULL;
  WGPUSurfaceCapabilities surface_capabilities = {0};
  int ret = EXIT_SUCCESS;

#define ASSERT_CHECK(expr)                                                     \
  do {                                                                         \
    if (!(expr)) {                                                             \
      int ret = EXIT_SUCCESS;                                                  \
      printf(LOG_PREFIX " assert failed %s: %s:%d\n", #expr, __FILE__,         \
             __LINE__);                                                        \
      goto cleanup_and_exit;                                                   \
    }                                                                          \
  } while (0)

  frmwrk_setup_logging(WGPULogLevel_Warn);

#if defined(WGPU_TARGET_LINUX_WAYLAND)
  glfwInitHint(GLFW_PLATFORM, GLFW_PLATFORM_WAYLAND);
#endif

  ASSERT_CHECK(glfwInit());

  demo.instance = wgpuCreateInstance(NULL);
  ASSERT_CHECK(demo.instance);

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  window =
      glfwCreateWindow(640, 480, "triangle [wgpu-native + glfw]", NULL, NULL);
  ASSERT_CHECK(window);

  glfwSetWindowUserPointer(window, (void *)&demo);
  glfwSetKeyCallback(window, handle_glfw_key);
  glfwSetFramebufferSizeCallback(window, handle_glfw_framebuffer_size);

#if defined(WGPU_TARGET_MACOS)
  {
    id metal_layer = NULL;
    NSWindow *ns_window = glfwGetCocoaWindow(window);
    [ns_window.contentView setWantsLayer:YES];
    metal_layer = [CAMetalLayer layer];
    [ns_window.contentView setLayer:metal_layer];
    demo.surface = wgpuInstanceCreateSurface(
        demo.instance,
        &(const WGPUSurfaceDescriptor){
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    const WGPUSurfaceDescriptorFromMetalLayer){
                    .chain =
                        (const WGPUChainedStruct){
                            .sType = WGPUSType_SurfaceDescriptorFromMetalLayer,
                        },
                    .layer = metal_layer,
                },
        });
    ASSERT_CHECK(demo.surface);
  }
#elif defined(WGPU_TARGET_LINUX_X11)
  {
    Display *x11_display = glfwGetX11Display();
    Window x11_window = glfwGetX11Window(window);
    demo.surface = wgpuInstanceCreateSurface(
        demo.instance,
        &(const WGPUSurfaceDescriptor){
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    const WGPUSurfaceDescriptorFromXlibWindow){
                    .chain =
                        (const WGPUChainedStruct){
                            .sType = WGPUSType_SurfaceDescriptorFromXlibWindow,
                        },
                    .display = x11_display,
                    .window = x11_window,
                },
        });
    ASSERT_CHECK(demo.surface);
  }
#elif defined(WGPU_TARGET_LINUX_WAYLAND)
  {
    struct wl_display *wayland_display = glfwGetWaylandDisplay();
    struct wl_surface *wayland_surface = glfwGetWaylandWindow(window);
    demo.surface = wgpuInstanceCreateSurface(
        demo.instance,
        &(const WGPUSurfaceDescriptor){
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    const WGPUSurfaceDescriptorFromWaylandSurface){
                    .chain =
                        (const WGPUChainedStruct){
                            .sType =
                                WGPUSType_SurfaceDescriptorFromWaylandSurface,
                        },
                    .display = wayland_display,
                    .surface = wayland_surface,
                },
        });
    ASSERT_CHECK(demo.surface);
  }
#elif defined(WGPU_TARGET_WINDOWS)
  {
    HWND hwnd = glfwGetWin32Window(window);
    HINSTANCE hinstance = GetModuleHandle(NULL);
    demo.surface = wgpuInstanceCreateSurface(
        demo.instance,
        &(const WGPUSurfaceDescriptor){
            .nextInChain =
                (const WGPUChainedStruct *)&(
                    const WGPUSurfaceDescriptorFromWindowsHWND){
                    .chain =
                        (const WGPUChainedStruct){
                            .sType = WGPUSType_SurfaceDescriptorFromWindowsHWND,
                        },
                    .hinstance = hinstance,
                    .hwnd = hwnd,
                },
        });
    ASSERT_CHECK(demo.surface);
  }
#else
#error "Unsupported WGPU_TARGET"
#endif

  wgpuInstanceRequestAdapter(demo.instance,
                             &(const WGPURequestAdapterOptions){
                                 .compatibleSurface = demo.surface,
                             },
                             handle_request_adapter, &demo);
  ASSERT_CHECK(demo.adapter);

  wgpuAdapterRequestDevice(demo.adapter, NULL, handle_request_device, &demo);
  ASSERT_CHECK(demo.device);

  queue = wgpuDeviceGetQueue(demo.device);
  ASSERT_CHECK(queue);

  shader_module = frmwrk_load_shader_module(demo.device, "shader.wgsl");
  ASSERT_CHECK(shader_module);

  pipeline_layout = wgpuDeviceCreatePipelineLayout(
      demo.device, &(const WGPUPipelineLayoutDescriptor){
                       .label = "pipeline_layout",
                   });
  ASSERT_CHECK(pipeline_layout);

  wgpuSurfaceGetCapabilities(demo.surface, demo.adapter, &surface_capabilities);

  render_pipeline = wgpuDeviceCreateRenderPipeline(
      demo.device,
      &(const WGPURenderPipelineDescriptor){
          .label = "render_pipeline",
          .layout = pipeline_layout,
          .vertex =
              (const WGPUVertexState){
                  .module = shader_module,
                  .entryPoint = "vs_main",
              },
          .fragment =
              &(const WGPUFragmentState){
                  .module = shader_module,
                  .entryPoint = "fs_main",
                  .targetCount = 1,
                  .targets =
                      (const WGPUColorTargetState[]){
                          (const WGPUColorTargetState){
                              .format = surface_capabilities.formats[0],
                              .writeMask = WGPUColorWriteMask_All,
                          },
                      },
              },
          .primitive =
              (const WGPUPrimitiveState){
                  .topology = WGPUPrimitiveTopology_TriangleList,
              },
          .multisample =
              (const WGPUMultisampleState){
                  .count = 1,
                  .mask = 0xFFFFFFFF,
              },
      });
  ASSERT_CHECK(render_pipeline);

  demo.config = (const WGPUSurfaceConfiguration){
      .device = demo.device,
      .usage = WGPUTextureUsage_RenderAttachment,
      .format = surface_capabilities.formats[0],
      .presentMode = WGPUPresentMode_Fifo,
      .alphaMode = surface_capabilities.alphaModes[0],
  };

  {
    int width, height;
    glfwGetWindowSize(window, &width, &height);
    demo.config.width = width;
    demo.config.height = height;
  }

  wgpuSurfaceConfigure(demo.surface, &demo.config);

  while (!glfwWindowShouldClose(window)) {
    glfwPollEvents();

    WGPUSurfaceTexture surface_texture;
    wgpuSurfaceGetCurrentTexture(demo.surface, &surface_texture);
    switch (surface_texture.status) {
    case WGPUSurfaceGetCurrentTextureStatus_Success:
      // All good, could check for `surface_texture.suboptimal` here.
      break;
    case WGPUSurfaceGetCurrentTextureStatus_Timeout:
    case WGPUSurfaceGetCurrentTextureStatus_Outdated:
    case WGPUSurfaceGetCurrentTextureStatus_Lost: {
      // Skip this frame, and re-configure surface.
      if (surface_texture.texture != NULL) {
        wgpuTextureRelease(surface_texture.texture);
      }
      int width, height;
      glfwGetWindowSize(window, &width, &height);
      if (width != 0 && height != 0) {
        demo.config.width = width;
        demo.config.height = height;
        wgpuSurfaceConfigure(demo.surface, &demo.config);
      }
      continue;
    }
    case WGPUSurfaceGetCurrentTextureStatus_OutOfMemory:
    case WGPUSurfaceGetCurrentTextureStatus_DeviceLost:
    case WGPUSurfaceGetCurrentTextureStatus_Force32:
      // Fatal error
      printf(LOG_PREFIX " get_current_texture status=%#.8x\n",
             surface_texture.status);
      abort();
    }
    assert(surface_texture.texture);

    WGPUTextureView frame =
        wgpuTextureCreateView(surface_texture.texture, NULL);
    assert(frame);

    WGPUCommandEncoder command_encoder = wgpuDeviceCreateCommandEncoder(
        demo.device, &(const WGPUCommandEncoderDescriptor){
                         .label = "command_encoder",
                     });
    assert(command_encoder);

    WGPURenderPassEncoder render_pass_encoder =
        wgpuCommandEncoderBeginRenderPass(
            command_encoder, &(const WGPURenderPassDescriptor){
                                 .label = "render_pass_encoder",
                                 .colorAttachmentCount = 1,
                                 .colorAttachments =
                                     (const WGPURenderPassColorAttachment[]){
                                         (const WGPURenderPassColorAttachment){
                                             .view = frame,
                                             .loadOp = WGPULoadOp_Clear,
                                             .storeOp = WGPUStoreOp_Store,
                                             .clearValue =
                                                 (const WGPUColor){
                                                     .r = 0.0,
                                                     .g = 1.0,
                                                     .b = 0.0,
                                                     .a = 1.0,
                                                 },
                                         },
                                     },
                             });
    assert(render_pass_encoder);

    wgpuRenderPassEncoderSetPipeline(render_pass_encoder, render_pipeline);
    wgpuRenderPassEncoderDraw(render_pass_encoder, 3, 1, 0, 0);
    wgpuRenderPassEncoderEnd(render_pass_encoder);

    WGPUCommandBuffer command_buffer = wgpuCommandEncoderFinish(
        command_encoder, &(const WGPUCommandBufferDescriptor){
                             .label = "command_buffer",
                         });
    assert(command_buffer);

    wgpuQueueSubmit(queue, 1, (const WGPUCommandBuffer[]){command_buffer});
    wgpuSurfacePresent(demo.surface);

    wgpuCommandBufferRelease(command_buffer);
    wgpuRenderPassEncoderRelease(render_pass_encoder);
    wgpuCommandEncoderRelease(command_encoder);
    wgpuTextureViewRelease(frame);
    wgpuTextureRelease(surface_texture.texture);
  }

cleanup_and_exit:
  if (render_pipeline)
    wgpuRenderPipelineRelease(render_pipeline);
  if (pipeline_layout)
    wgpuPipelineLayoutRelease(pipeline_layout);
  if (shader_module)
    wgpuShaderModuleRelease(shader_module);
  wgpuSurfaceCapabilitiesFreeMembers(surface_capabilities);
  if (queue)
    wgpuQueueRelease(queue);
  if (demo.device)
    wgpuDeviceRelease(demo.device);
  if (demo.adapter)
    wgpuAdapterRelease(demo.adapter);
  if (demo.surface)
    wgpuSurfaceRelease(demo.surface);
  if (window)
    glfwDestroyWindow(window);
  if (demo.instance)
    wgpuInstanceRelease(demo.instance);

  glfwTerminate();
  return 0;
}
