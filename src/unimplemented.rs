use crate::native;

#[no_mangle]
pub extern "C" fn wgpuGetProcAddress(
    _proc_name: *const ::std::os::raw::c_char,
) -> native::WGPUProc {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupSetLabel(
    _bind_group: native::WGPUBindGroup,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupLayoutSetLabel(
    _bind_group_layout: native::WGPUBindGroupLayout,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferGetMapState(_buffer: native::WGPUBuffer) -> native::WGPUBufferMapState {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferSetLabel(
    _buffer: native::WGPUBuffer,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandBufferSetLabel(
    _command_buffer: native::WGPUCommandBuffer,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderSetLabel(
    _command_encoder: native::WGPUCommandEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderSetLabel(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePipelineSetLabel(
    _compute_pipeline: native::WGPUComputePipeline,
    _label: *const ::std::os::raw::c_char,
) {
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
pub extern "C" fn wgpuDeviceSetLabel(
    _device: native::WGPUDevice,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceHasWGSLLanguageFeature(
    _instance: native::WGPUInstance,
    _feature: native::WGPUWGSLFeatureName,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceProcessEvents(_instance: native::WGPUInstance) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuPipelineLayoutSetLabel(
    _pipeline_layout: native::WGPUPipelineLayout,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetSetLabel(
    _query_set: native::WGPUQuerySet,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQueueSetLabel(
    _queue: native::WGPUQueue,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleSetLabel(
    _render_bundle: native::WGPURenderBundle,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetLabel(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetLabel(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineSetLabel(
    _render_pipeline: native::WGPURenderPipeline,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSamplerSetLabel(
    _sampler: native::WGPUSampler,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleGetCompilationInfo(
    _shader_module: native::WGPUShaderModule,
    _callback: native::WGPUCompilationInfoCallbackInfo,
) -> native::WGPUFuture {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleSetLabel(
    _shader_module: native::WGPUShaderModule,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceSetLabel(
    _surface: native::WGPUSurface,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureSetLabel(
    _texture: native::WGPUTexture,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureViewSetLabel(
    _texture_view: native::WGPUTextureView,
    _label: *const ::std::os::raw::c_char,
) {
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
