#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

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
  WGPUSwapChainDescriptor config;
  WGPUSwapChain swapchain;
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
  if (!demo || !demo->swapchain)
    return;

  demo->config.width = width;
  demo->config.height = height;

  if (demo->swapchain)
    wgpuSwapChainDrop(demo->swapchain);
  demo->swapchain =
      wgpuDeviceCreateSwapChain(demo->device, demo->surface, &demo->config);
  assert(demo->swapchain);
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
  WGPUTextureView next_texture = NULL;
  WGPUCommandEncoder command_encoder = NULL;
  WGPURenderPassEncoder render_pass_encoder = NULL;
  WGPUCommandBuffer command_buffer = NULL;
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

  demo.instance = wgpuCreateInstance(&(const WGPUInstanceDescriptor){0});
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

  wgpuDeviceSetUncapturedErrorCallback(demo.device, handle_uncaptured_error,
                                       NULL);
  wgpuDeviceSetDeviceLostCallback(demo.device, handle_device_lost, NULL);

  shader_module = frmwrk_load_shader_module(demo.device, "shader.wgsl");
  ASSERT_CHECK(shader_module);

  pipeline_layout = wgpuDeviceCreatePipelineLayout(
      demo.device, &(const WGPUPipelineLayoutDescriptor){
                       .label = "pipeline_layout",
                   });
  ASSERT_CHECK(pipeline_layout);

  WGPUTextureFormat surface_preferred_format =
      wgpuSurfaceGetPreferredFormat(demo.surface, demo.adapter);
  ASSERT_CHECK(surface_preferred_format != WGPUTextureFormat_Undefined);

  render_pipeline = wgpuDeviceCreateRenderPipeline(
      demo.device, &(const WGPURenderPipelineDescriptor){
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
                                           .format = surface_preferred_format,
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

  demo.config = (WGPUSwapChainDescriptor){
      .usage = WGPUTextureUsage_RenderAttachment,
      .format = surface_preferred_format,
      .presentMode = WGPUPresentMode_Fifo,
  };

  {
    int width, height;
    glfwGetWindowSize(window, &width, &height);
    demo.config.width = width;
    demo.config.height = height;
  }

  demo.swapchain =
      wgpuDeviceCreateSwapChain(demo.device, demo.surface, &demo.config);
  ASSERT_CHECK(demo.swapchain);

  while (!glfwWindowShouldClose(window)) {
    glfwPollEvents();

    next_texture = wgpuSwapChainGetCurrentTextureView(demo.swapchain);
    ASSERT_CHECK(next_texture);

    command_encoder = wgpuDeviceCreateCommandEncoder(
        demo.device, &(const WGPUCommandEncoderDescriptor){
                         .label = "command_encoder",
                     });
    ASSERT_CHECK(command_encoder);

    render_pass_encoder = wgpuCommandEncoderBeginRenderPass(
        command_encoder, &(const WGPURenderPassDescriptor){
                             .label = "render_pass_encoder",
                             .colorAttachmentCount = 1,
                             .colorAttachments =
                                 (const WGPURenderPassColorAttachment[]){
                                     (const WGPURenderPassColorAttachment){
                                         .view = next_texture,
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
    ASSERT_CHECK(render_pass_encoder);

    wgpuRenderPassEncoderSetPipeline(render_pass_encoder, render_pipeline);
    wgpuRenderPassEncoderDraw(render_pass_encoder, 3, 1, 0, 0);
    wgpuRenderPassEncoderEnd(render_pass_encoder);
    // wgpuRenderPassEncoderEnd() drops render_pass_encoder
    render_pass_encoder = NULL;

    wgpuTextureViewDrop(next_texture);
    next_texture = NULL;

    command_buffer = wgpuCommandEncoderFinish(
        command_encoder, &(const WGPUCommandBufferDescriptor){
                             .label = "command_buffer",
                         });
    ASSERT_CHECK(command_buffer);
    // wgpuCommandEncoderFinish() drops command_encoder
    command_encoder = NULL;

    wgpuQueueSubmit(queue, 1, (const WGPUCommandBuffer[]){command_buffer});
    // wgpuQueueSubmit() drops command_buffer
    command_buffer = NULL;

    wgpuSwapChainPresent(demo.swapchain);
  }

cleanup_and_exit:
  if (command_buffer)
    wgpuCommandBufferDrop(command_buffer);
  if (render_pass_encoder)
    wgpuRenderPassEncoderDrop(render_pass_encoder);
  if (command_encoder)
    wgpuCommandEncoderDrop(command_encoder);
  if (next_texture)
    wgpuTextureViewDrop(next_texture);
  if (render_pipeline)
    wgpuRenderPipelineDrop(render_pipeline);
  if (pipeline_layout)
    wgpuPipelineLayoutDrop(pipeline_layout);
  if (shader_module)
    wgpuShaderModuleDrop(shader_module);
  if (demo.swapchain)
    wgpuSwapChainDrop(demo.swapchain);
  if (queue)
    wgpuQueueDrop(queue);
  if (demo.device)
    wgpuDeviceDrop(demo.device);
  if (demo.adapter)
    wgpuAdapterDrop(demo.adapter);
  if (demo.surface)
    wgpuSurfaceDrop(demo.surface);
  if (window)
    glfwDestroyWindow(window);
  if (demo.instance)
    wgpuInstanceDrop(demo.instance);

  glfwTerminate();
  return 0;
}
