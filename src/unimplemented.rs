use crate::native;

#[no_mangle]
pub extern "C" fn wgpuGetProcAddress(_proc_name: native::WGPUStringView) -> native::WGPUProc {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateComputePipelineAsync(
    _device: native::WGPUDevice,
    _descriptor: *const native::WGPUComputePipelineDescriptor,
    _callback: native::WGPUCreateComputePipelineAsyncCallbackInfo,
) -> native::WGPUFuture {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateRenderPipelineAsync(
    _device: native::WGPUDevice,
    _descriptor: *const native::WGPURenderPipelineDescriptor,
    _callback: native::WGPUCreateRenderPipelineAsyncCallbackInfo,
) -> native::WGPUFuture {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceGetLostFuture(_device: native::WGPUDevice) -> native::WGPUFuture {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceWaitAny(
    _instance: native::WGPUInstance,
    _future_count: usize,
    _futures: *mut native::WGPUFutureWaitInfo,
    _timeout_ns: u64,
) -> native::WGPUWaitStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetTextureBindingViewDimension(
    _texture: native::WGPUTexture,
) -> native::WGPUTextureViewDimension {
    unimplemented!("Blocked on wgpu-core support: https://github.com/gfx-rs/wgpu/issues/7428");
}
