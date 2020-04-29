use crate::GLOBAL;

use wgc::{device::Label, gfx_select, hub::Token, id};
use wgt::{BackendBit, DeviceDescriptor, Limits};

use std::{marker::PhantomData, slice};

pub type RequestAdapterCallback =
    unsafe extern "C" fn(id: Option<id::AdapterId>, userdata: *mut std::ffi::c_void);

pub fn wgpu_create_surface(raw_handle: raw_window_handle::RawWindowHandle) -> id::SurfaceId {
    GLOBAL.instance_create_surface(raw_handle, PhantomData)
}

#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
#[no_mangle]
pub extern "C" fn wgpu_create_surface_from_xlib(
    display: *mut *const std::ffi::c_void,
    window: libc::c_ulong,
) -> id::SurfaceId {
    use raw_window_handle::unix::XlibHandle;
    wgpu_create_surface(raw_window_handle::RawWindowHandle::Xlib(XlibHandle {
        window,
        display: display as *mut _,
        ..XlibHandle::empty()
    }))
}

#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
#[no_mangle]
pub extern "C" fn wgpu_create_surface_from_wayland(
    surface: *mut std::ffi::c_void,
    display: *mut std::ffi::c_void,
) -> id::SurfaceId {
    use raw_window_handle::unix::WaylandHandle;
    wgpu_create_surface(raw_window_handle::RawWindowHandle::Wayland(WaylandHandle {
        surface,
        display,
        ..WaylandHandle::empty()
    }))
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn wgpu_create_surface_from_android(
    a_native_window: *mut std::ffi::c_void,
) -> id::SurfaceId {
    use raw_window_handle::android::AndroidHandle;
    wgpu_create_surface(raw_window_handle::RawWindowHandle::Android(AndroidHandle {
        a_native_window,
        ..AndroidHandle::empty()
    }))
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn wgpu_create_surface_from_metal_layer(
    layer: *mut std::ffi::c_void,
) -> id::SurfaceId {
    let surface = wgc::instance::Surface {
        #[cfg(feature = "vulkan-portability")]
        vulkan: None, //TODO: currently requires `NSView`
        metal: GLOBAL
            .instance
            .metal
            .create_surface_from_layer(layer as *mut _, cfg!(debug_assertions)),
    };

    GLOBAL
        .surfaces
        .register_identity(PhantomData, surface, &mut Token::root())
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn wgpu_create_surface_from_windows_hwnd(
    _hinstance: *mut std::ffi::c_void,
    hwnd: *mut std::ffi::c_void,
) -> id::SurfaceId {
    use raw_window_handle::windows::WindowsHandle;
    wgpu_create_surface(raw_window_handle::RawWindowHandle::Windows(
        raw_window_handle::windows::WindowsHandle {
            hwnd,
            ..WindowsHandle::empty()
        },
    ))
}

pub fn wgpu_enumerate_adapters(mask: BackendBit) -> Vec<id::AdapterId> {
    GLOBAL.enumerate_adapters(wgc::instance::AdapterInputs::Mask(mask, |_| PhantomData))
}

/// # Safety
///
/// This function is unsafe as it calls an unsafe extern callback.
#[no_mangle]
pub unsafe extern "C" fn wgpu_request_adapter_async(
    desc: Option<&wgc::instance::RequestAdapterOptions>,
    mask: BackendBit,
    callback: RequestAdapterCallback,
    userdata: *mut std::ffi::c_void,
) {
    let id = GLOBAL.pick_adapter(
        &desc.cloned().unwrap_or_default(),
        wgc::instance::AdapterInputs::Mask(mask, |_| PhantomData),
    );
    callback(id, userdata);
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_adapter_request_device(
    adapter_id: id::AdapterId,
    desc: Option<&DeviceDescriptor>,
    trace_path: *const std::os::raw::c_char,
) -> id::DeviceId {
    let desc = &desc.cloned().unwrap_or_default();
    let trace_cstr = if trace_path.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(trace_path))
    };
    let trace_cow = trace_cstr.as_ref().map(|cstr| cstr.to_string_lossy());
    let trace_path = trace_cow
        .as_ref()
        .map(|cow| std::path::Path::new(cow.as_ref()));
    gfx_select!(adapter_id => GLOBAL.adapter_request_device(adapter_id, desc, trace_path, PhantomData))
}

pub fn adapter_get_info(adapter_id: id::AdapterId) -> wgc::instance::AdapterInfo {
    gfx_select!(adapter_id => GLOBAL.adapter_get_info(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_destroy(adapter_id: id::AdapterId) {
    gfx_select!(adapter_id => GLOBAL.adapter_destroy(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_get_limits(_device_id: id::DeviceId, limits: &mut Limits) {
    *limits = Limits::default(); // TODO
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_buffer(
    device_id: id::DeviceId,
    desc: &wgt::BufferDescriptor<Label>,
) -> id::BufferId {
    gfx_select!(device_id => GLOBAL.device_create_buffer(device_id, desc, PhantomData))
}

/// # Safety
///
/// This function is unsafe as there is no guarantee that the given pointer
/// dereferenced in this function is valid.
#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_buffer_mapped(
    device_id: id::DeviceId,
    desc: &wgt::BufferDescriptor<Label>,
    mapped_ptr_out: *mut *mut u8,
) -> id::BufferId {
    let (id, ptr) =
        gfx_select!(device_id => GLOBAL.device_create_buffer_mapped(device_id, desc, PhantomData));
    *mapped_ptr_out = ptr;
    id
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_destroy(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_destroy(buffer_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_texture(
    device_id: id::DeviceId,
    desc: &wgt::TextureDescriptor<Label>,
) -> id::TextureId {
    gfx_select!(device_id => GLOBAL.device_create_texture(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_texture_destroy(texture_id: id::TextureId) {
    gfx_select!(texture_id => GLOBAL.texture_destroy(texture_id))
}

#[no_mangle]
pub extern "C" fn wgpu_texture_create_view(
    texture_id: id::TextureId,
    desc: Option<&wgt::TextureViewDescriptor<Label>>,
) -> id::TextureViewId {
    gfx_select!(texture_id => GLOBAL.texture_create_view(texture_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_texture_view_destroy(texture_view_id: id::TextureViewId) {
    gfx_select!(texture_view_id => GLOBAL.texture_view_destroy(texture_view_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_sampler(
    device_id: id::DeviceId,
    desc: &wgt::SamplerDescriptor<Label>,
) -> id::SamplerId {
    gfx_select!(device_id => GLOBAL.device_create_sampler(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_sampler_destroy(sampler_id: id::SamplerId) {
    gfx_select!(sampler_id => GLOBAL.sampler_destroy(sampler_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_bind_group_layout(
    device_id: id::DeviceId,
    desc: &wgc::binding_model::BindGroupLayoutDescriptor,
) -> id::BindGroupLayoutId {
    gfx_select!(device_id => GLOBAL.device_create_bind_group_layout(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_pipeline_layout(
    device_id: id::DeviceId,
    desc: &wgc::binding_model::PipelineLayoutDescriptor,
) -> id::PipelineLayoutId {
    gfx_select!(device_id => GLOBAL.device_create_pipeline_layout(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_bind_group(
    device_id: id::DeviceId,
    desc: &wgc::binding_model::BindGroupDescriptor,
) -> id::BindGroupId {
    gfx_select!(device_id => GLOBAL.device_create_bind_group(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_destroy(bind_group_id: id::BindGroupId) {
    gfx_select!(bind_group_id => GLOBAL.bind_group_destroy(bind_group_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_shader_module(
    device_id: id::DeviceId,
    desc: &wgc::pipeline::ShaderModuleDescriptor,
) -> id::ShaderModuleId {
    gfx_select!(device_id => GLOBAL.device_create_shader_module(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_command_encoder(
    device_id: id::DeviceId,
    desc: Option<&wgt::CommandEncoderDescriptor>,
) -> id::CommandEncoderId {
    let desc = &desc.cloned().unwrap_or_default();
    gfx_select!(device_id => GLOBAL.device_create_command_encoder(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_destroy(command_encoder_id: id::CommandEncoderId) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_destroy(command_encoder_id))
}

#[no_mangle]
pub extern "C" fn wgpu_command_buffer_destroy(command_buffer_id: id::CommandBufferId) {
    gfx_select!(command_buffer_id => GLOBAL.command_buffer_destroy(command_buffer_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_get_default_queue(device_id: id::DeviceId) -> id::QueueId {
    device_id
}

/// # Safety
///
/// This function is unsafe as there is no guarantee that the given pointer is
/// valid for `command_buffers_length` elements.
#[no_mangle]
pub unsafe extern "C" fn wgpu_queue_submit(
    queue_id: id::QueueId,
    command_buffers: *const id::CommandBufferId,
    command_buffers_length: usize,
) {
    let command_buffer_ids = slice::from_raw_parts(command_buffers, command_buffers_length);
    gfx_select!(queue_id => GLOBAL.queue_submit(queue_id, command_buffer_ids))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_render_pipeline(
    device_id: id::DeviceId,
    desc: &wgc::pipeline::RenderPipelineDescriptor,
) -> id::RenderPipelineId {
    gfx_select!(device_id => GLOBAL.device_create_render_pipeline(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_compute_pipeline(
    device_id: id::DeviceId,
    desc: &wgc::pipeline::ComputePipelineDescriptor,
) -> id::ComputePipelineId {
    gfx_select!(device_id => GLOBAL.device_create_compute_pipeline(device_id, desc, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_swap_chain(
    device_id: id::DeviceId,
    surface_id: id::SurfaceId,
    desc: &wgt::SwapChainDescriptor,
) -> id::SwapChainId {
    gfx_select!(device_id => GLOBAL.device_create_swap_chain(device_id, surface_id, desc))
}

#[no_mangle]
pub extern "C" fn wgpu_device_poll(device_id: id::DeviceId, force_wait: bool) {
    gfx_select!(device_id => GLOBAL.device_poll(device_id, force_wait))
}

#[no_mangle]
pub extern "C" fn wgpu_device_destroy(device_id: id::DeviceId) {
    gfx_select!(device_id => GLOBAL.device_destroy(device_id))
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_map_read_async(
    buffer_id: id::BufferId,
    start: wgt::BufferAddress,
    size: wgt::BufferAddress,
    callback: wgc::device::BufferMapReadCallback,
    userdata: *mut u8,
) {
    let operation = wgc::resource::BufferMapOperation::Read { callback, userdata };

    gfx_select!(buffer_id => GLOBAL.buffer_map_async(buffer_id, start .. start + size, operation))
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_map_write_async(
    buffer_id: id::BufferId,
    start: wgt::BufferAddress,
    size: wgt::BufferAddress,
    callback: wgc::device::BufferMapWriteCallback,
    userdata: *mut u8,
) {
    let operation = wgc::resource::BufferMapOperation::Write { callback, userdata };

    gfx_select!(buffer_id => GLOBAL.buffer_map_async(buffer_id, start .. start + size, operation))
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_unmap(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id))
}

#[no_mangle]
pub extern "C" fn wgpu_swap_chain_get_next_texture(
    swap_chain_id: id::SwapChainId,
) -> wgc::swap_chain::SwapChainOutput {
    gfx_select!(swap_chain_id => GLOBAL.swap_chain_get_next_texture(swap_chain_id, PhantomData))
        .unwrap_or(wgc::swap_chain::SwapChainOutput { view_id: None })
}

#[no_mangle]
pub extern "C" fn wgpu_swap_chain_present(swap_chain_id: id::SwapChainId) {
    gfx_select!(swap_chain_id => GLOBAL.swap_chain_present(swap_chain_id))
}
