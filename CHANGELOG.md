# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

#### Table of Contents

- [Unreleased](#unreleased)
- [Diffs](#diffs)

## Unreleased

### Changed

- Updated all wgpu crates to v29
- MSRV bumped from 1.82 to 1.87.
- **Push constants renamed to immediates.** This matches the upstream wgpu rename.
  - `WGPUNativeFeature_PushConstants` -> `WGPUNativeFeature_Immediates`
  - `WGPUNativeLimits::maxPushConstantSize` -> `WGPUNativeLimits::maxImmediateSize`
  - `WGPUPipelineLayoutExtras` no longer takes an array of `WGPUPushConstantRange`. It now takes a single `uint32_t immediateDataSize` representing the total size in bytes.
    ```c
    // Before
    WGPUPushConstantRange range = { .stages = WGPUShaderStage_Compute, .start = 0, .end = 4 };
    WGPUPipelineLayoutExtras extras = {
        .chain = { .sType = WGPUSType_PipelineLayoutExtras },
        .pushConstantRangeCount = 1,
        .pushConstantRanges = &range,
    };

    // After
    WGPUPipelineLayoutExtras extras = {
        .chain = { .sType = WGPUSType_PipelineLayoutExtras },
        .immediateDataSize = sizeof(uint32_t),
    };
    ```
  - `wgpuRenderPassEncoderSetPushConstants` -> `wgpuRenderPassEncoderSetImmediates` (removed `stages` parameter)
  - `wgpuComputePassEncoderSetPushConstants` -> `wgpuComputePassEncoderSetImmediates`
  - `wgpuRenderBundleEncoderSetPushConstants` -> `wgpuRenderBundleEncoderSetImmediates` (removed `stages` parameter)
    ```c
    // Before
    wgpuComputePassEncoderSetPushConstants(encoder, 0, sizeof(uint32_t), &data);
    wgpuRenderPassEncoderSetPushConstants(encoder, WGPUShaderStage_Vertex, 0, sizeof(uint32_t), &data);

    // After
    wgpuComputePassEncoderSetImmediates(encoder, 0, sizeof(uint32_t), &data);
    wgpuRenderPassEncoderSetImmediates(encoder, 0, sizeof(uint32_t), &data);
    ```
  - WGSL shaders must use `var<immediate>` instead of `var<push_constant>`.
- `WGPUNativeFeature_ShaderPrimitiveIndex` removed. Use the standard `WGPUFeatureName_PrimitiveIndex` from `webgpu.h` instead.
- `wgpuQueueGetNativeMetalCommandQueue` now returns `NULL`. The raw `MTLCommandQueue` is no longer publicly accessible in wgpu-hal v29.

### Added

- `WGPUNativeDisplayHandle` tagged union for passing a platform display connection at instance creation. Set `WGPUInstanceExtras::displayHandle` to provide an Xlib, XCB, or Wayland display handle. Required by the GLES backend on Wayland; other backends ignore it. Zero-initialization yields `WGPUNativeDisplayHandleType_None`.
  ```c
  WGPUInstanceExtras extras = {
      .chain = { .sType = WGPUSType_InstanceExtras },
      .displayHandle = {
          .type = WGPUNativeDisplayHandleType_Wayland,
          .data.wayland = { .display = wl_display },
      },
  };
  ```
- `WGPUSurfaceGetCurrentTextureStatus_Occluded` native extension value for `WGPUSurfaceGetCurrentTextureStatus`. Returned by `wgpuSurfaceGetCurrentTexture` when the window is not visible (e.g. minimized or fully behind another window). Currently only produced by the Metal backend on macOS, where acquiring a drawable while occluded would otherwise block for up to one second waiting for vsync. When you receive this status, no texture is returned and the surface remains valid -- skip rendering for the current frame and retry once the window becomes visible again. No reconfiguration is needed.

### Removed

- `WGPUPushConstantRange` struct.
- `foreign-types-shared` dependency (no longer needed after Metal backend switched to `objc2`).
- `raw-window-handle` feature on `wgpu-core` dependency (removed upstream).

## Diffs

- [Unreleased](https://github.com/gfx-rs/wgpu-native/compare/v27.0.4.1...HEAD)
