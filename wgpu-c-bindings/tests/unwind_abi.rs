use wgpu_c_bindings::*;

#[test]
fn panic_capable_declarations_preserve_the_runtime_unwind_abi() {
    let _: unsafe extern "C-unwind" fn(WGPUDevice) = wgpuDeviceRelease;
    let _: unsafe extern "C-unwind" fn(
        WGPUDevice,
        WGPUBool,
        *const WGPUSubmissionIndex,
        u64,
    ) -> WGPUNativePollStatus = wgpuDevicePoll;
    let _: unsafe extern "C-unwind" fn(WGPUBuffer, usize, usize) -> *mut std::ffi::c_void =
        wgpuBufferGetMappedRange;
    let _: unsafe extern "C-unwind" fn(
        wgpu_native::native::WGPUDevice,
        WGPUBool,
        Option<&WGPUSubmissionIndex>,
        u64,
    ) -> WGPUNativePollStatus = wgpu_native::wgpuDevicePoll;
    let _: unsafe extern "C-unwind" fn(wgpu_native::native::WGPUInstance, WGPUBool) -> WGPUBool =
        wgpu_native::wgpuInstancePollAllDevices;
}
