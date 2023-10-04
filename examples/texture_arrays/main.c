#include "framework.h"
#include "webgpu-headers/webgpu.h"
#include "wgpu.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(WGPU_TARGET_MACOS)
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

#define LOG_PREFIX "[texture_arrays]"
#define MAX_FEATURE_ARRAY_LENGTH 64

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

typedef struct vertex {
  float pos[2];
  float tex_coord[2];
  uint32_t index;
} vertex;

static const WGPUVertexAttribute vertex_attributes[] = {
    (const WGPUVertexAttribute){
        .format = WGPUVertexFormat_Float32x2,
        .offset = 0,
        .shaderLocation = 0,
    },
    (const WGPUVertexAttribute){
        .format = WGPUVertexFormat_Float32x2,
        .offset = 0 + 8, // 0 + sizeof(Float32x2)
        .shaderLocation = 1,
    },
    (const WGPUVertexAttribute){
        .format = WGPUVertexFormat_Sint32,
        .offset = 0 + 8 + 8, // 0 + sizeof(Float32x2) + sizeof(Float32x2)
        .shaderLocation = 2,
    },
};

static const WGPUIndexFormat index_format = WGPUIndexFormat_Uint16;

static const vertex vertices[] = {
    // left rectangle
    {{-1, -1}, {0, 1}, 0},
    {{-1, 1}, {0, 0}, 0},
    {{0, 1}, {1, 0}, 0},
    {{0, -1}, {1, 1}, 0},
    // right rectangle
    {{0, -1}, {0, 1}, 1},
    {{0, 1}, {0, 0}, 1},
    {{1, 1}, {1, 0}, 1},
    {{1, -1}, {1, 1}, 1},
};

static const uint16_t indices[] = {
    // Left rectangle
    0, 1, 2, // 1st
    2, 0, 3, // 2nd
    // Right rectangle
    4, 5, 6, // 1st
    6, 4, 7, // 2nd
};

static const uint8_t red_texture_data[4] = {255, 0, 0, 255};
static const uint8_t green_texture_data[4] = {0, 255, 0, 255};
static const uint8_t blue_texture_data[4] = {0, 0, 255, 255};
static const uint8_t white_texture_data[4] = {255, 255, 255, 255};

int main(int argc, char *argv[]) {
  UNUSED(argc)
  UNUSED(argv)
  frmwrk_setup_logging(WGPULogLevel_Warn);

#if defined(WGPU_TARGET_LINUX_WAYLAND)
  glfwInitHint(GLFW_PLATFORM, GLFW_PLATFORM_WAYLAND);
#endif
  assert(glfwInit());

  struct demo demo = {0};
  demo.instance = wgpuCreateInstance(NULL);
  assert(demo.instance);

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  GLFWwindow *window = glfwCreateWindow(
      640, 480, "texture_arrays [wgpu-native + glfw]", NULL, NULL);
  assert(window);
  glfwSetWindowUserPointer(window, (void *)&demo);
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
    assert(demo.surface);
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
    assert(demo.surface);
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
    assert(demo.surface);
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
    assert(demo.surface);
  }
#else
#error "Unsupported WGPU_TARGET"
#endif

  wgpuInstanceRequestAdapter(demo.instance,
                             &(const WGPURequestAdapterOptions){
                                 .compatibleSurface = demo.surface,
                             },
                             handle_request_adapter, &demo);
  assert(demo.adapter);

  WGPUSurfaceCapabilities surface_capabilities = {0};
  wgpuSurfaceGetCapabilities(demo.surface, demo.adapter, &surface_capabilities);

  WGPUFeatureName adapter_features[MAX_FEATURE_ARRAY_LENGTH] = {0};
  size_t adapter_feature_count =
      wgpuAdapterEnumerateFeatures(demo.adapter, adapter_features);
  assert(adapter_feature_count <= MAX_FEATURE_ARRAY_LENGTH);
  bool adapter_has_required_features = false;
  bool adapter_has_optional_features = false;
  for (size_t i = 0; i < adapter_feature_count; i++) {
    switch ((uint32_t)adapter_features[i]) {
    case WGPUNativeFeature_TextureBindingArray:
      adapter_has_required_features = true;
      break;
    case WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing:
      adapter_has_optional_features = true;
      break;
    }
  }
  assert(
          adapter_has_required_features /* Adapter must support WGPUNativeFeature_TextureBindingArray feature for this example */);

  WGPUFeatureName required_device_features[2] = {
      (WGPUFeatureName)WGPUNativeFeature_TextureBindingArray,
  };
  size_t required_device_feature_count = 1;
  if (adapter_has_optional_features) {
    required_device_features[required_device_feature_count] = (WGPUFeatureName)
        WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing;
    required_device_feature_count++;
  }

  wgpuAdapterRequestDevice(
      demo.adapter,
      &(WGPUDeviceDescriptor){
          .requiredFeatureCount = required_device_feature_count,
          .requiredFeatures = required_device_features,
      },
      handle_request_device, &demo);
  assert(demo.device);

  WGPUQueue queue = wgpuDeviceGetQueue(demo.device);
  assert(queue);

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
    printf(LOG_PREFIX " initial window size: width=%d height=%d\n", width,
           height);
    demo.config.width = width;
    demo.config.height = height;
  }
  wgpuSurfaceConfigure(demo.surface, &demo.config);

  char *fragment_entry_point = NULL;
  bool use_uniform_workaround = false;
  if (adapter_has_optional_features) {
    fragment_entry_point = "non_uniform_main";
  } else {
    use_uniform_workaround = true;
    fragment_entry_point = "uniform_main";
  }

  WGPUShaderModule base_shader_module =
      frmwrk_load_shader_module(demo.device, "indexing.wgsl");
  assert(base_shader_module);

  WGPUShaderModule fragment_shader_module = NULL;
  if (!use_uniform_workaround) {
    fragment_shader_module =
        frmwrk_load_shader_module(demo.device, "non_uniform_indexing.wgsl");
  } else {
    fragment_shader_module =
        frmwrk_load_shader_module(demo.device, "indexing.wgsl");
  }
  assert(fragment_shader_module);
  printf("Using fragment entry point: '%s'\n", fragment_entry_point);

  WGPUBuffer vertex_buffer = frmwrk_device_create_buffer_init(
      demo.device, &(frmwrk_buffer_init_descriptor){
                       .label = "Vertex Buffer",
                       .content = (void *)vertices,
                       .content_size = sizeof(vertices),
                       .usage = WGPUBufferUsage_Vertex,
                   });
  assert(vertex_buffer);

  WGPUBuffer index_buffer = frmwrk_device_create_buffer_init(
      demo.device, &(frmwrk_buffer_init_descriptor){
                       .label = "Index Buffer",
                       .content = (void *)indices,
                       .content_size = sizeof(indices),
                       .usage = WGPUBufferUsage_Index,
                   });
  assert(index_buffer);

  uint32_t texture_index_buffer_contents[128] = {
      [0] = 0,
      [64] = 1,
  };
  WGPUBuffer texture_index_buffer = frmwrk_device_create_buffer_init(
      demo.device, &(frmwrk_buffer_init_descriptor){
                       .label = "Texture Index Buffer",
                       .content = texture_index_buffer_contents,
                       .content_size = sizeof(texture_index_buffer_contents),
                       .usage = WGPUBufferUsage_Uniform,
                   });
  assert(texture_index_buffer);

  const WGPUExtent3D extent_3d_default = (const WGPUExtent3D){
      .width = 1,
      .height = 1,
      .depthOrArrayLayers = 1,
  };

#define COLOR_TEXTURE_DESCRIPTOR_COMMON_FIELDS                                 \
  /* clang-format off */                                                       \
  .size = extent_3d_default,                                                \
  .mipLevelCount = 1,                                                          \
  .sampleCount = 1,                                                            \
  .dimension = WGPUTextureDimension_2D,                                        \
  .format = WGPUTextureFormat_RGBA8UnormSrgb,                                  \
  .usage = WGPUTextureUsage_TextureBinding | WGPUTextureUsage_CopyDst
  /* clang-format on */

  WGPUTexture red_texture = wgpuDeviceCreateTexture(
      demo.device, &(WGPUTextureDescriptor){
                       COLOR_TEXTURE_DESCRIPTOR_COMMON_FIELDS,
                       .label = "red",
                   });
  assert(red_texture);
  WGPUTexture green_texture = wgpuDeviceCreateTexture(
      demo.device, &(WGPUTextureDescriptor){
                       COLOR_TEXTURE_DESCRIPTOR_COMMON_FIELDS,
                       .label = "green",
                   });
  assert(green_texture);
  WGPUTexture blue_texture = wgpuDeviceCreateTexture(
      demo.device, &(WGPUTextureDescriptor){
                       COLOR_TEXTURE_DESCRIPTOR_COMMON_FIELDS,
                       .label = "blue",
                   });
  assert(blue_texture);
  WGPUTexture white_texture = wgpuDeviceCreateTexture(
      demo.device, &(WGPUTextureDescriptor){
                       COLOR_TEXTURE_DESCRIPTOR_COMMON_FIELDS,
                       .label = "white",
                   });
  assert(white_texture);

  WGPUTextureView red_texture_view = wgpuTextureCreateView(red_texture, NULL);
  assert(red_texture_view);
  WGPUTextureView green_texture_view =
      wgpuTextureCreateView(green_texture, NULL);
  assert(green_texture_view);
  WGPUTextureView blue_texture_view = wgpuTextureCreateView(blue_texture, NULL);
  assert(blue_texture_view);
  WGPUTextureView white_texture_view =
      wgpuTextureCreateView(white_texture, NULL);
  assert(white_texture_view);

#define COLOR_IMAGE_COPY_TEXTURE_COMMON_FIELDS                                 \
  /* clang-format off */                                                       \
  .mipLevel = 0,                                                               \
  .origin = (const WGPUOrigin3D){.x = 0, .y = 0, .z = 0},                      \
  .aspect = WGPUTextureAspect_All
  /* clang-format on */

  const WGPUTextureDataLayout texture_data_layout_common =
      (const WGPUTextureDataLayout){
          .offset = 0,
          .bytesPerRow = 4,
          .rowsPerImage = WGPU_COPY_STRIDE_UNDEFINED,
      };

  wgpuQueueWriteTexture(queue,
                        &(const WGPUImageCopyTexture){
                            .texture = red_texture,
                            COLOR_IMAGE_COPY_TEXTURE_COMMON_FIELDS,
                        },
                        red_texture_data, sizeof(red_texture_data),
                        &texture_data_layout_common, &extent_3d_default);
  wgpuQueueWriteTexture(queue,
                        &(const WGPUImageCopyTexture){
                            .texture = green_texture,
                            COLOR_IMAGE_COPY_TEXTURE_COMMON_FIELDS,
                        },
                        green_texture_data, sizeof(green_texture_data),
                        &texture_data_layout_common, &extent_3d_default);
  wgpuQueueWriteTexture(queue,
                        &(const WGPUImageCopyTexture){
                            .texture = blue_texture,
                            COLOR_IMAGE_COPY_TEXTURE_COMMON_FIELDS,
                        },
                        blue_texture_data, sizeof(blue_texture_data),
                        &texture_data_layout_common, &extent_3d_default);
  wgpuQueueWriteTexture(queue,
                        &(const WGPUImageCopyTexture){
                            .texture = white_texture,
                            COLOR_IMAGE_COPY_TEXTURE_COMMON_FIELDS,
                        },
                        white_texture_data, sizeof(white_texture_data),
                        &texture_data_layout_common, &extent_3d_default);

  WGPUSampler sampler = wgpuDeviceCreateSampler(demo.device, NULL);
  assert(sampler);

  const WGPUBindGroupLayoutEntry bind_group_layout_entries[] = {
      (const WGPUBindGroupLayoutEntry){
          .binding = 0,
          .visibility = WGPUShaderStage_Fragment,
          .texture =
              (const WGPUTextureBindingLayout){
                  .sampleType = WGPUTextureSampleType_Float,
                  .viewDimension = WGPUTextureViewDimension_2D,
                  .multisampled = false,
              },
          .nextInChain =
              (const WGPUChainedStruct *)&(WGPUBindGroupLayoutEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType =
                              (WGPUSType)WGPUSType_BindGroupLayoutEntryExtras,
                      },
                  .count = 2,
              },
      },
      (const WGPUBindGroupLayoutEntry){
          .binding = 1,
          .visibility = WGPUShaderStage_Fragment,
          .texture =
              (const WGPUTextureBindingLayout){
                  .sampleType = WGPUTextureSampleType_Float,
                  .viewDimension = WGPUTextureViewDimension_2D,
                  .multisampled = false,
              },
          .nextInChain =
              (const WGPUChainedStruct *)&(WGPUBindGroupLayoutEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType =
                              (WGPUSType)WGPUSType_BindGroupLayoutEntryExtras,
                      },
                  .count = 2,
              },
      },
      (const WGPUBindGroupLayoutEntry){
          .binding = 2,
          .visibility = WGPUShaderStage_Fragment,
          .sampler =
              (const WGPUSamplerBindingLayout){
                  .type = WGPUSamplerBindingType_Filtering,
              },
          .nextInChain =
              (const WGPUChainedStruct *)&(WGPUBindGroupLayoutEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType =
                              (WGPUSType)WGPUSType_BindGroupLayoutEntryExtras,
                      },
                  .count = 2,
              },
      },
      (const WGPUBindGroupLayoutEntry){
          .binding = 3,
          .visibility = WGPUShaderStage_Fragment,
          .buffer =
              (const WGPUBufferBindingLayout){
                  .type = WGPUBufferBindingType_Uniform,
                  .hasDynamicOffset = true,
                  .minBindingSize = 4,
              },
      },
  };
  WGPUBindGroupLayout bind_group_layout = wgpuDeviceCreateBindGroupLayout(
      demo.device, &(const WGPUBindGroupLayoutDescriptor){
                       .label = "bind group layout",
                       .entryCount = sizeof(bind_group_layout_entries) /
                                     sizeof(bind_group_layout_entries[0]),
                       .entries = bind_group_layout_entries,
                   });
  assert(bind_group_layout);

  const WGPUBindGroupEntry bind_group_entries[] = {
      (const WGPUBindGroupEntry){
          .binding = 0,
          .nextInChain =
              (const WGPUChainedStruct *)&(const WGPUBindGroupEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType = (WGPUSType)WGPUSType_BindGroupEntryExtras,
                      },
                  .textureViewCount = 2,
                  .textureViews =
                      (const WGPUTextureView[]){
                          red_texture_view,
                          green_texture_view,
                      },
              },
      },
      (const WGPUBindGroupEntry){
          .binding = 1,
          .nextInChain =
              (const WGPUChainedStruct *)&(const WGPUBindGroupEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType = (WGPUSType)WGPUSType_BindGroupEntryExtras,
                      },
                  .textureViewCount = 2,
                  .textureViews =
                      (const WGPUTextureView[]){
                          blue_texture_view,
                          white_texture_view,
                      },
              },
      },
      (const WGPUBindGroupEntry){
          .binding = 2,
          .nextInChain =
              (const WGPUChainedStruct *)&(const WGPUBindGroupEntryExtras){
                  .chain =
                      (const WGPUChainedStruct){
                          .sType = (WGPUSType)WGPUSType_BindGroupEntryExtras,
                      },
                  .samplerCount = 2,
                  .samplers =
                      (const WGPUSampler[]){
                          sampler,
                          sampler,
                      },
              },
      },
      (const WGPUBindGroupEntry){
          .binding = 3,
          .buffer = texture_index_buffer,
          .offset = 0,
          .size = 4,
      },
  };
  WGPUBindGroup bind_group = wgpuDeviceCreateBindGroup(
      demo.device, &(const WGPUBindGroupDescriptor){
                       .layout = bind_group_layout,
                       .label = "bind group",
                       .entryCount = sizeof(bind_group_entries) /
                                     sizeof(bind_group_entries[0]),
                       .entries = bind_group_entries,
                   });
  assert(bind_group);

  WGPUPipelineLayout pipeline_layout = wgpuDeviceCreatePipelineLayout(
      demo.device, &(const WGPUPipelineLayoutDescriptor){
                       .label = "main",
                       .bindGroupLayoutCount = 1,
                       .bindGroupLayouts =
                           (const WGPUBindGroupLayout[]){
                               bind_group_layout,
                           },
                   });
  assert(pipeline_layout);

  WGPURenderPipeline pipeline = wgpuDeviceCreateRenderPipeline(
      demo.device,
      &(const WGPURenderPipelineDescriptor){
          .layout = pipeline_layout,
          .vertex =
              (const WGPUVertexState){
                  .module = base_shader_module,
                  .entryPoint = "vert_main",
                  .bufferCount = 1,
                  .buffers =
                      (const WGPUVertexBufferLayout[]){
                          (const WGPUVertexBufferLayout){
                              .arrayStride = sizeof(vertex),
                              .stepMode = WGPUVertexStepMode_Vertex,
                              .attributeCount = sizeof(vertex_attributes) /
                                                sizeof(vertex_attributes[0]),
                              .attributes = vertex_attributes,
                          },
                      },
              },
          .fragment =
              &(const WGPUFragmentState){
                  .module = fragment_shader_module,
                  .entryPoint = fragment_entry_point,
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
                  .frontFace = WGPUFrontFace_CCW,
                  .topology = WGPUPrimitiveTopology_TriangleList,
              },
          .multisample =
              (const WGPUMultisampleState){
                  .count = 1,
                  .mask = 0xFFFFFFFF,
              },
      });
  assert(pipeline);

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
      wgpuTextureRelease(surface_texture.texture);
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
                                                     .g = 0.0,
                                                     .b = 0.0,
                                                     .a = 1.0,
                                                 },
                                         },
                                     },

                             });
    assert(render_pass_encoder);

    wgpuRenderPassEncoderSetPipeline(render_pass_encoder, pipeline);
    wgpuRenderPassEncoderSetVertexBuffer(render_pass_encoder, 0, vertex_buffer,
                                         0, WGPU_WHOLE_SIZE);
    wgpuRenderPassEncoderSetIndexBuffer(render_pass_encoder, index_buffer,
                                        index_format, 0, WGPU_WHOLE_SIZE);
    if (use_uniform_workaround) {
      wgpuRenderPassEncoderSetBindGroup(render_pass_encoder, 0, bind_group, 1,
                                        (const uint32_t[]){0});
      wgpuRenderPassEncoderDrawIndexed(render_pass_encoder, 6, 1, 0, 0, 0);
      wgpuRenderPassEncoderSetBindGroup(render_pass_encoder, 0, bind_group, 1,
                                        (const uint32_t[]){256});
      wgpuRenderPassEncoderDrawIndexed(render_pass_encoder, 6, 1, 6, 0, 0);
    } else {
      wgpuRenderPassEncoderSetBindGroup(render_pass_encoder, 0, bind_group, 1,
                                        (const uint32_t[]){0});
      wgpuRenderPassEncoderDrawIndexed(render_pass_encoder, 12, 1, 0, 0, 0);
    }
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

  wgpuRenderPipelineRelease(pipeline);
  wgpuPipelineLayoutRelease(pipeline_layout);
  wgpuBindGroupRelease(bind_group);
  wgpuBindGroupLayoutRelease(bind_group_layout);
  wgpuSamplerRelease(sampler);
  wgpuTextureViewRelease(white_texture_view);
  wgpuTextureViewRelease(blue_texture_view);
  wgpuTextureViewRelease(green_texture_view);
  wgpuTextureViewRelease(red_texture_view);
  wgpuTextureRelease(white_texture);
  wgpuTextureRelease(blue_texture);
  wgpuTextureRelease(green_texture);
  wgpuTextureRelease(red_texture);
  wgpuBufferRelease(texture_index_buffer);
  wgpuBufferRelease(index_buffer);
  wgpuBufferRelease(vertex_buffer);
  wgpuShaderModuleRelease(fragment_shader_module);
  wgpuShaderModuleRelease(base_shader_module);
  wgpuQueueRelease(queue);
  wgpuDeviceRelease(demo.device);
  wgpuSurfaceCapabilitiesFreeMembers(surface_capabilities);
  wgpuAdapterRelease(demo.adapter);
  wgpuSurfaceRelease(demo.surface);
  glfwDestroyWindow(window);
  wgpuInstanceRelease(demo.instance);
  glfwTerminate();
  return 0;
}
