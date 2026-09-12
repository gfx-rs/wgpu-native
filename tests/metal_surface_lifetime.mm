#import <AppKit/AppKit.h>
#import <Metal/Metal.h>
#import <QuartzCore/CAMetalLayer.h>
#include <wgpu.h>

#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <future>
#include <memory>
#include <string>
#include <thread>
#include <type_traits>

using namespace std::chrono_literals;

#define CHECK(condition) do { if (!(condition)) { \
    std::fprintf(stderr, "FAIL %s:%d: %s\n", __FILE__, __LINE__, #condition); \
    std::exit(1); } } while (false)

template<auto Release, typename Handle>
auto Own(Handle handle) {
    return std::unique_ptr<std::remove_pointer_t<Handle>, decltype(Release)>{handle, Release};
}

static void PrintMessage(WGPUStringView message) {
    if (message.data) {
        const size_t length = message.length == WGPU_STRLEN ? std::strlen(message.data) : message.length;
        std::fwrite(message.data, 1, length, stderr);
        std::fputc('\n', stderr);
    }
}

int main(int argc, char** argv) {
    CHECK(argc == 2);
    const std::string test = argv[1];
    @autoreleasepool {
        [NSApplication sharedApplication];
        NSWindow* window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 64, 64)
            styleMask:NSWindowStyleMaskBorderless backing:NSBackingStoreBuffered defer:NO];
        window.releasedWhenClosed = NO;
        CAMetalLayer* layer = [CAMetalLayer layer];
        layer.device = MTLCreateSystemDefaultDevice();
        CHECK(layer.device != nil);
        layer.pixelFormat = MTLPixelFormatBGRA8Unorm;
        layer.drawableSize = CGSizeMake(64, 64);
        window.contentView.wantsLayer = YES;
        window.contentView.layer = layer;
        [window orderFront:nil];

        auto instance = Own<wgpuInstanceRelease>(wgpuCreateInstance(nullptr));
        CHECK(instance);
        WGPUSurfaceSourceMetalLayer source = WGPU_SURFACE_SOURCE_METAL_LAYER_INIT;
        source.layer = (__bridge void*)layer;
        WGPUSurfaceDescriptor surfaceDescriptor = WGPU_SURFACE_DESCRIPTOR_INIT;
        surfaceDescriptor.nextInChain = &source.chain;
        auto surface = Own<wgpuSurfaceRelease>(wgpuInstanceCreateSurface(instance.get(), &surfaceDescriptor));
        CHECK(surface);

        auto* adapterPromise = new std::promise<WGPUAdapter>;
        auto adapterFuture = adapterPromise->get_future();
        WGPURequestAdapterOptions options = WGPU_REQUEST_ADAPTER_OPTIONS_INIT;
        options.compatibleSurface = surface.get();
        options.backendType = WGPUBackendType_Metal;
        WGPURequestAdapterCallbackInfo adapterCallback = WGPU_REQUEST_ADAPTER_CALLBACK_INFO_INIT;
        adapterCallback.mode = WGPUCallbackMode_AllowSpontaneous;
        adapterCallback.userdata1 = adapterPromise;
        adapterCallback.callback = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, WGPUStringView message, void* data, void*) {
            std::unique_ptr<std::promise<WGPUAdapter>> completion{static_cast<std::promise<WGPUAdapter>*>(data)};
            if (status != WGPURequestAdapterStatus_Success) PrintMessage(message);
            CHECK(status == WGPURequestAdapterStatus_Success);
            completion->set_value(adapter);
        };
        wgpuInstanceRequestAdapter(instance.get(), &options, adapterCallback);
        CHECK(adapterFuture.wait_for(5s) == std::future_status::ready);
        auto adapter = Own<wgpuAdapterRelease>(adapterFuture.get());
        CHECK(adapter);
        WGPUAdapterInfo info = WGPU_ADAPTER_INFO_INIT;
        CHECK(wgpuAdapterGetInfo(adapter.get(), &info) == WGPUStatus_Success);
        CHECK(info.backendType == WGPUBackendType_Metal);
        CHECK(info.adapterType != WGPUAdapterType_CPU);
        PrintMessage(info.device);
        wgpuAdapterInfoFreeMembers(info);

        auto* devicePromise = new std::promise<WGPUDevice>;
        auto deviceFuture = devicePromise->get_future();
        WGPURequestDeviceCallbackInfo deviceCallback = WGPU_REQUEST_DEVICE_CALLBACK_INFO_INIT;
        deviceCallback.mode = WGPUCallbackMode_AllowSpontaneous;
        deviceCallback.userdata1 = devicePromise;
        deviceCallback.callback = [](WGPURequestDeviceStatus status, WGPUDevice device, WGPUStringView message, void* data, void*) {
            std::unique_ptr<std::promise<WGPUDevice>> completion{static_cast<std::promise<WGPUDevice>*>(data)};
            if (status != WGPURequestDeviceStatus_Success) PrintMessage(message);
            CHECK(status == WGPURequestDeviceStatus_Success);
            completion->set_value(device);
        };
        wgpuAdapterRequestDevice(adapter.get(), nullptr, deviceCallback);
        CHECK(deviceFuture.wait_for(5s) == std::future_status::ready);
        auto device = Own<wgpuDeviceRelease>(deviceFuture.get());
        auto queue = Own<wgpuQueueRelease>(wgpuDeviceGetQueue(device.get()));
        WGPUSurfaceConfiguration config = WGPU_SURFACE_CONFIGURATION_INIT;
        config.device = device.get();
        config.format = WGPUTextureFormat_BGRA8Unorm;
        config.usage = WGPUTextureUsage_RenderAttachment;
        config.width = 64;
        config.height = 64;
        wgpuSurfaceConfigure(surface.get(), &config);

        auto acquire = [&] {
            WGPUSurfaceTexture frame = WGPU_SURFACE_TEXTURE_INIT;
            wgpuSurfaceGetCurrentTexture(surface.get(), &frame);
            CHECK(frame.status == WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal ||
                  frame.status == WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal);
            CHECK(frame.texture);
            return Own<wgpuTextureRelease>(frame.texture);
        };
        auto render = [&](WGPUTexture texture) {
            auto view = Own<wgpuTextureViewRelease>(wgpuTextureCreateView(texture, nullptr));
            CHECK(view);
            WGPURenderPassColorAttachment color = WGPU_RENDER_PASS_COLOR_ATTACHMENT_INIT;
            color.view = view.get();
            color.loadOp = WGPULoadOp_Clear;
            color.storeOp = WGPUStoreOp_Store;
            color.clearValue = {0.125, 0.25, 0.5, 1.0};
            WGPURenderPassDescriptor descriptor = WGPU_RENDER_PASS_DESCRIPTOR_INIT;
            descriptor.colorAttachmentCount = 1;
            descriptor.colorAttachments = &color;
            auto encoder = Own<wgpuCommandEncoderRelease>(wgpuDeviceCreateCommandEncoder(device.get(), nullptr));
            auto pass = Own<wgpuRenderPassEncoderRelease>(wgpuCommandEncoderBeginRenderPass(encoder.get(), &descriptor));
            wgpuRenderPassEncoderEnd(pass.get());
            auto commands = Own<wgpuCommandBufferRelease>(wgpuCommandEncoderFinish(encoder.get(), nullptr));
            const WGPUCommandBuffer command = commands.get();
            wgpuQueueSubmit(queue.get(), 1, &command);
        };
        auto poll = [&] {
            CHECK(wgpuDevicePoll(device.get(), 1, nullptr, 5'000'000'000ULL) != WGPUNativePollStatus_Timeout);
        };

        if (test == "present-retained-frame" || test == "discard-retained-frame" ||
            test == "reconfigure-retained-frame" || test == "threaded-old-frame-release") {
            for (int i = 0; i != 32; ++i) {
                auto oldFrame = acquire();
                render(oldFrame.get());
                if (test == "discard-retained-frame") {
                    CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Success);
                } else {
                    CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
                }
                poll();
                if (test == "reconfigure-retained-frame") wgpuSurfaceConfigure(surface.get(), &config);
                auto newFrame = acquire();
                render(newFrame.get());
                if (test == "threaded-old-frame-release") {
                    std::thread releaseOld{[frame = std::move(oldFrame)]() mutable { frame.reset(); }};
                    CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
                    releaseOld.join();
                } else {
                    oldFrame.reset();
                    CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
                }
                poll();
            }
        } else if (test == "unconfigure-retained-frame") {
            auto frame = acquire();
            render(frame.get());
            CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
            poll();
            wgpuSurfaceUnconfigure(surface.get());
            frame.reset();
            wgpuSurfaceConfigure(surface.get(), &config);
            frame = acquire();
            CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Success);
        } else if (test == "release-surface-first") {
            auto frame = acquire();
            render(frame.get());
            poll();
            surface.reset();
            frame.reset();
        } else if (test == "unconfigure-acquired-frame") {
            auto oldFrame = acquire();
            render(oldFrame.get());
            poll();
            wgpuSurfaceUnconfigure(surface.get());
            wgpuSurfaceConfigure(surface.get(), &config);
            auto newFrame = acquire();
            oldFrame.reset();
            render(newFrame.get());
            CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
        } else if (test == "destroy-without-present" || test == "present-destroyed-frame" ||
                   test == "discard-destroyed-frame") {
            auto frame = acquire();
            wgpuTextureDestroy(frame.get());
            if (test == "present-destroyed-frame") {
                CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Error);
            } else if (test == "discard-destroyed-frame") {
                CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Error);
            }
            frame.reset();
            frame = acquire();
            render(frame.get());
            CHECK(wgpuSurfacePresent(surface.get()) == WGPUStatus_Success);
        } else if (test == "drop-without-present") {
            for (int i = 0; i != 64; ++i) {
                auto frame = acquire();
                render(frame.get());
                poll();
            }
        } else if (test == "discard-status") {
            CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Error);
            auto frame = acquire();
            CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Success);
            CHECK(wgpuSurfaceDiscardTexture(surface.get()) == WGPUStatus_Error);
        } else {
            CHECK(false && "unknown test");
        }
        poll();
        if (surface) wgpuSurfaceUnconfigure(surface.get());
        surface.reset();
        WGPUGlobalReport report{};
        wgpuGenerateReport(instance.get(), &report);
        CHECK(report.surfaces.numAllocated == 0);
        CHECK(report.surfaces.numKeptFromUser == 0);
        CHECK(report.hub.textures.numKeptFromUser == 0);
        CHECK(report.hub.textureViews.numKeptFromUser == 0);
        [window close];
        std::printf("PASS %s\n", test.c_str());
    }
}
