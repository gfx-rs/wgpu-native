#include <wgpu.h>

#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <future>
#include <memory>
#include <string>
#include <type_traits>

using namespace std::chrono_literals;

#define CHECK(condition) do { if (!(condition)) { \
    std::fprintf(stderr, "FAIL %s:%d: %s\n", __FILE__, __LINE__, #condition); \
    std::exit(1); } } while (false)

template<auto Release, typename Handle>
auto Own(Handle handle) {
    return std::unique_ptr<std::remove_pointer_t<Handle>, decltype(Release)>{handle, Release};
}

static std::string Message(WGPUStringView value) {
    return value.data ? std::string(value.data, value.length == WGPU_STRLEN ? std::strlen(value.data) : value.length) : "";
}

static void CheckScope(WGPUDevice device, WGPUErrorType expected) {
    struct Result { WGPUPopErrorScopeStatus status; WGPUErrorType type; std::string message; };
    auto* promise = new std::promise<Result>;
    auto future = promise->get_future();
    WGPUPopErrorScopeCallbackInfo callback = WGPU_POP_ERROR_SCOPE_CALLBACK_INFO_INIT;
    callback.mode = WGPUCallbackMode_AllowSpontaneous;
    callback.userdata1 = promise;
    callback.callback = [](WGPUPopErrorScopeStatus status, WGPUErrorType type, WGPUStringView message, void* data, void*) {
        std::unique_ptr<std::promise<Result>> completion{static_cast<std::promise<Result>*>(data)};
        completion->set_value({status, type, Message(message)});
    };
    wgpuDevicePopErrorScope(device, callback);
    CHECK(future.wait_for(5s) == std::future_status::ready);
    const auto result = future.get();
    if (result.type != expected) std::fprintf(stderr, "%s\n", result.message.c_str());
    CHECK(result.status == WGPUPopErrorScopeStatus_Success);
    CHECK(result.type == expected);
}

static WGPUDevice RequestDevice(WGPUAdapter adapter, bool enableNativeFeatures) {
    auto* promise = new std::promise<WGPUDevice>;
    auto future = promise->get_future();
    WGPURequestDeviceCallbackInfo callback = WGPU_REQUEST_DEVICE_CALLBACK_INFO_INIT;
    callback.mode = WGPUCallbackMode_AllowSpontaneous;
    callback.userdata1 = promise;
    callback.callback = [](WGPURequestDeviceStatus status, WGPUDevice device, WGPUStringView message, void* data, void*) {
        std::unique_ptr<std::promise<WGPUDevice>> completion{static_cast<std::promise<WGPUDevice>*>(data)};
        if (status != WGPURequestDeviceStatus_Success) std::fprintf(stderr, "%s\n", Message(message).c_str());
        CHECK(status == WGPURequestDeviceStatus_Success);
        completion->set_value(device);
    };
    WGPUDeviceDescriptor descriptor = WGPU_DEVICE_DESCRIPTOR_INIT;
    WGPUDeviceDescriptorExtras extras{};
    extras.chain.sType = static_cast<WGPUSType>(WGPUSType_DeviceDescriptorExtras);
    extras.experimentalFeaturesEnabled = true;
    const WGPUFeatureName features[] = {
        static_cast<WGPUFeatureName>(WGPUNativeFeature_RayQuery),
        static_cast<WGPUFeatureName>(WGPUNativeFeature_TextureAtomic),
        static_cast<WGPUFeatureName>(WGPUNativeFeature_TextureAdapterSpecificFormatFeatures),
    };
    if (enableNativeFeatures) {
        for (auto feature : features) CHECK(wgpuAdapterHasFeature(adapter, feature));
        descriptor.nextInChain = &extras.chain;
        descriptor.requiredFeatureCount = sizeof(features) / sizeof(features[0]);
        descriptor.requiredFeatures = features;
    }
    wgpuAdapterRequestDevice(adapter, &descriptor, callback);
    CHECK(future.wait_for(5s) == std::future_status::ready);
    return future.get();
}

int main() {
    auto instance = Own<wgpuInstanceRelease>(wgpuCreateInstance(nullptr));
    auto* promise = new std::promise<WGPUAdapter>;
    auto future = promise->get_future();
    WGPURequestAdapterOptions options = WGPU_REQUEST_ADAPTER_OPTIONS_INIT;
    options.backendType = WGPUBackendType_Metal;
    WGPURequestAdapterCallbackInfo callback = WGPU_REQUEST_ADAPTER_CALLBACK_INFO_INIT;
    callback.mode = WGPUCallbackMode_AllowSpontaneous;
    callback.userdata1 = promise;
    callback.callback = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, WGPUStringView message, void* data, void*) {
        std::unique_ptr<std::promise<WGPUAdapter>> completion{static_cast<std::promise<WGPUAdapter>*>(data)};
        if (status != WGPURequestAdapterStatus_Success) std::fprintf(stderr, "%s\n", Message(message).c_str());
        CHECK(status == WGPURequestAdapterStatus_Success);
        completion->set_value(adapter);
    };
    wgpuInstanceRequestAdapter(instance.get(), &options, callback);
    CHECK(future.wait_for(5s) == std::future_status::ready);
    auto adapter = Own<wgpuAdapterRelease>(future.get());
    WGPUAdapterInfo info = WGPU_ADAPTER_INFO_INIT;
    CHECK(wgpuAdapterGetInfo(adapter.get(), &info) == WGPUStatus_Success);
    CHECK(info.backendType == WGPUBackendType_Metal);
    CHECK(info.adapterType != WGPUAdapterType_CPU);
    std::printf("Adapter: %s\n", Message(info.device).c_str());
    wgpuAdapterInfoFreeMembers(info);

    auto standardDevice = Own<wgpuDeviceRelease>(RequestDevice(adapter.get(), false));
    auto nativeDevice = Own<wgpuDeviceRelease>(RequestDevice(adapter.get(), true));
    const WGPUWgpuBufferUsage usages[] = {
        WGPUWgpuBufferUsage_BlasInput, WGPUWgpuBufferUsage_TlasInput,
        WGPUWgpuBufferUsage_BlasInput | WGPUWgpuBufferUsage_TlasInput,
    };
    for (auto usage : usages) {
        WGPUWgpuBufferUsageExtras extras{};
        extras.chain.sType = static_cast<WGPUSType>(WGPUSType_WgpuBufferUsageExtras);
        extras.usage = usage;
        // Ensure traversal does not assume that this is the first extension.
        WGPUChainedStruct prefix{&extras.chain, static_cast<WGPUSType>(0x7fffFF01)};
        WGPUBufferDescriptor descriptor = WGPU_BUFFER_DESCRIPTOR_INIT;
        descriptor.nextInChain = &prefix;
        descriptor.size = 256;
        descriptor.usage = WGPUBufferUsage_CopyDst;
        wgpuDevicePushErrorScope(standardDevice.get(), WGPUErrorFilter_Validation);
        auto invalid = Own<wgpuBufferRelease>(wgpuDeviceCreateBuffer(standardDevice.get(), &descriptor));
        CheckScope(standardDevice.get(), WGPUErrorType_Validation);
        for (auto standardUsage : {WGPUBufferUsage_None, WGPUBufferUsage_CopyDst}) {
            descriptor.usage = standardUsage;
            wgpuDevicePushErrorScope(nativeDevice.get(), WGPUErrorFilter_Validation);
            auto buffer = Own<wgpuBufferRelease>(wgpuDeviceCreateBuffer(nativeDevice.get(), &descriptor));
            CHECK(buffer);
            CHECK(wgpuBufferGetUsage(buffer.get()) == standardUsage);
            CheckScope(nativeDevice.get(), WGPUErrorType_NoError);
        }
    }
    std::puts("PASS buffer native-only/mixed usages, feature validation, and standard-only getter");

    // Native flag values 1 and 2 must not alter standard MapRead/MapWrite semantics.
    for (auto usage : {WGPUBufferUsage_MapRead | WGPUBufferUsage_CopyDst,
                       WGPUBufferUsage_MapWrite | WGPUBufferUsage_CopySrc}) {
        WGPUBufferDescriptor descriptor = WGPU_BUFFER_DESCRIPTOR_INIT;
        descriptor.size = 256;
        descriptor.usage = usage;
        wgpuDevicePushErrorScope(standardDevice.get(), WGPUErrorFilter_Validation);
        auto buffer = Own<wgpuBufferRelease>(wgpuDeviceCreateBuffer(standardDevice.get(), &descriptor));
        CHECK(wgpuBufferGetUsage(buffer.get()) == usage);
        CheckScope(standardDevice.get(), WGPUErrorType_NoError);
    }
    std::puts("PASS standard buffer usages without extension");

    WGPUNativeTextureFormatCapabilities caps{};
    CHECK(wgpuAdapterGetTextureFormatCapabilities(adapter.get(), WGPUTextureFormat_R32Uint, &caps) == WGPUStatus_Success);
    CHECK((caps.allowedWgpuUsages & WGPUWgpuTextureUsage_StorageAtomic) != 0);
    CHECK((caps.allowedUsages & ~static_cast<WGPUTextureUsage>(0x3f)) == 0);
    WGPUWgpuTextureUsageExtras extras{};
    extras.chain.sType = static_cast<WGPUSType>(WGPUSType_WgpuTextureUsageExtras);
    extras.usage = WGPUWgpuTextureUsage_StorageAtomic;
    WGPUChainedStruct prefix{&extras.chain, static_cast<WGPUSType>(0x7fffFF01)};
    WGPUTextureDescriptor descriptor = WGPU_TEXTURE_DESCRIPTOR_INIT;
    descriptor.nextInChain = &prefix;
    descriptor.format = WGPUTextureFormat_R32Uint;
    descriptor.size = {4, 4, 1};
    descriptor.usage = WGPUTextureUsage_None;
    wgpuDevicePushErrorScope(nativeDevice.get(), WGPUErrorFilter_Validation);
    auto nativeOnlyTexture = Own<wgpuTextureRelease>(wgpuDeviceCreateTexture(nativeDevice.get(), &descriptor));
    CHECK(wgpuTextureGetUsage(nativeOnlyTexture.get()) == WGPUTextureUsage_None);
    CheckScope(nativeDevice.get(), WGPUErrorType_NoError);
    descriptor.usage = WGPUTextureUsage_StorageBinding | WGPUTextureUsage_TextureBinding;
    wgpuDevicePushErrorScope(nativeDevice.get(), WGPUErrorFilter_Validation);
    auto texture = Own<wgpuTextureRelease>(wgpuDeviceCreateTexture(nativeDevice.get(), &descriptor));
    CHECK(wgpuTextureGetUsage(texture.get()) == descriptor.usage);
    CheckScope(nativeDevice.get(), WGPUErrorType_NoError);
    std::puts("PASS texture atomic creation and separated capabilities/getter");

    WGPUBindGroupLayoutEntry layoutEntry = WGPU_BIND_GROUP_LAYOUT_ENTRY_INIT;
    layoutEntry.visibility = WGPUShaderStage_Compute;
    layoutEntry.storageTexture.access = static_cast<WGPUStorageTextureAccess>(WGPUStorageTextureAccess_Atomic);
    layoutEntry.storageTexture.format = WGPUTextureFormat_R32Uint;
    layoutEntry.storageTexture.viewDimension = WGPUTextureViewDimension_2D;
    WGPUBindGroupLayoutDescriptor layoutDescriptor = WGPU_BIND_GROUP_LAYOUT_DESCRIPTOR_INIT;
    layoutDescriptor.entryCount = 1;
    layoutDescriptor.entries = &layoutEntry;
    auto layout = Own<wgpuBindGroupLayoutRelease>(wgpuDeviceCreateBindGroupLayout(nativeDevice.get(), &layoutDescriptor));
    // Unspecified views inherit usages; an explicit sampled-only view cannot be used for storage.
    for (int mode = 0; mode < 4; ++mode) {
        WGPUTextureViewDescriptor viewDescriptor = WGPU_TEXTURE_VIEW_DESCRIPTOR_INIT;
        if (mode == 2) {
            viewDescriptor.nextInChain = &prefix;
            viewDescriptor.usage = WGPUTextureUsage_StorageBinding;
        }
        if (mode == 3) viewDescriptor.usage = WGPUTextureUsage_TextureBinding;
        wgpuDevicePushErrorScope(nativeDevice.get(), WGPUErrorFilter_Validation);
        auto view = Own<wgpuTextureViewRelease>(wgpuTextureCreateView(texture.get(), mode == 0 ? nullptr : &viewDescriptor));
        CheckScope(nativeDevice.get(), WGPUErrorType_NoError);
        WGPUBindGroupEntry entry = WGPU_BIND_GROUP_ENTRY_INIT;
        entry.textureView = view.get();
        WGPUBindGroupDescriptor groupDescriptor = WGPU_BIND_GROUP_DESCRIPTOR_INIT;
        groupDescriptor.layout = layout.get();
        groupDescriptor.entryCount = 1;
        groupDescriptor.entries = &entry;
        wgpuDevicePushErrorScope(nativeDevice.get(), WGPUErrorFilter_Validation);
        auto group = Own<wgpuBindGroupRelease>(wgpuDeviceCreateBindGroup(nativeDevice.get(), &groupDescriptor));
        CheckScope(nativeDevice.get(), mode == 3 ? WGPUErrorType_Validation : WGPUErrorType_NoError);
    }
    std::puts("PASS inherited/explicit atomic texture views and standard-only restriction");
    std::puts("PASS native-usage");
}
