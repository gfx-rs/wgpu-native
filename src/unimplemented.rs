use crate::native;

#[no_mangle]
pub extern "C" fn wgpuGetProcAddress(
    _device: native::WGPUDevice,
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
pub extern "C" fn wgpuBufferGetSize(_buffer: native::WGPUBuffer) -> u64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferGetUsage(_buffer: native::WGPUBuffer) -> native::WGPUBufferUsage {
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
    _callback: native::WGPUCreateComputePipelineAsyncCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateRenderPipelineAsync(
    _device: native::WGPUDevice,
    _descriptor: *const native::WGPURenderPipelineDescriptor,
    _callback: native::WGPUCreateRenderPipelineAsyncCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDevicePopErrorScope(
    _device: native::WGPUDevice,
    _callback: native::WGPUErrorCallback,
    _userdata: *mut ::std::os::raw::c_void,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDevicePushErrorScope(
    _device: native::WGPUDevice,
    _filter: native::WGPUErrorFilter,
) {
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
pub extern "C" fn wgpuQuerySetDestroy(_query_set: native::WGPUQuerySet) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetGetCount(_query_set: native::WGPUQuerySet) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetGetType(_query_set: native::WGPUQuerySet) -> native::WGPUQueryType {
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
pub extern "C" fn wgpuQueueOnSubmittedWorkDone(
    _queue: native::WGPUQueue,
    _callback: native::WGPUQueueWorkDoneCallback,
    _userdata: *mut ::std::os::raw::c_void,
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
pub extern "C" fn wgpuRenderBundleEncoderSetLabel(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderBeginOcclusionQuery(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _query_index: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderEndOcclusionQuery(
    _render_pass_encoder: native::WGPURenderPassEncoder,
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
    _callback: native::WGPUCompilationInfoCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
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
