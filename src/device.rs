use crate::{follow_chain, ChainedStruct, SType, GLOBAL};

use libc::c_char;
use std::ptr;

use wgc::{device::HostMap, device::Label, gfx_select, hub::Token, id, instance::DeviceType};
use wgt::{Backend, BackendBit, DeviceDescriptor, Limits};

use std::ffi::CString;
use std::{marker::PhantomData, slice};

pub type RequestAdapterCallback =
    unsafe extern "C" fn(id: Option<id::AdapterId>, userdata: *mut std::ffi::c_void);

// see https://github.com/rust-windowing/raw-window-handle/issues/49
struct PseudoRwh(raw_window_handle::RawWindowHandle);
unsafe impl raw_window_handle::HasRawWindowHandle for PseudoRwh {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.clone()
    }
}

pub fn wgpu_create_surface(raw_handle: raw_window_handle::RawWindowHandle) -> id::SurfaceId {
    GLOBAL.instance_create_surface(&PseudoRwh(raw_handle), PhantomData)
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

pub fn wgpu_enumerate_adapters(mask: BackendBit, allow_unsafe: bool) -> Vec<id::AdapterId> {
    let unsafe_extensions = if allow_unsafe {
        unsafe { wgt::UnsafeExtensions::allow() }
    } else {
        wgt::UnsafeExtensions::disallow()
    };

    GLOBAL.enumerate_adapters(
        unsafe_extensions,
        wgc::instance::AdapterInputs::Mask(mask, |_| PhantomData),
    )
}

/// # Safety
///
/// This function is unsafe as it calls an unsafe extern callback.
#[no_mangle]
pub unsafe extern "C" fn wgpu_request_adapter_async(
    desc: Option<&wgc::instance::RequestAdapterOptions>,
    mask: BackendBit,
    allow_unsafe: bool,
    callback: RequestAdapterCallback,
    userdata: *mut std::ffi::c_void,
) {
    let unsafe_extensions = if allow_unsafe {
        wgt::UnsafeExtensions::allow()
    } else {
        wgt::UnsafeExtensions::disallow()
    };

    let id = GLOBAL.pick_adapter(
        &desc.cloned().unwrap_or_default(),
        unsafe_extensions,
        wgc::instance::AdapterInputs::Mask(mask, |_| PhantomData),
    );
    callback(id, userdata);
}

#[repr(C)]
pub struct CLimits {
    max_bind_groups: u32,
}

impl From<wgt::Limits> for CLimits {
    fn from(other: Limits) -> Self {
        Self {
            max_bind_groups: other.max_bind_groups,
        }
    }
}

#[repr(u8)]
pub enum CDeviceType {
    /// Other.
    Other = 0,
    /// Integrated GPU with shared CPU/GPU memory.
    IntegratedGpu,
    /// Discrete GPU with separate CPU/GPU memory.
    DiscreteGpu,
    /// Virtual / Hosted.
    VirtualGpu,
    /// Cpu / Software Rendering.
    Cpu,
}

impl From<DeviceType> for CDeviceType {
    fn from(other: DeviceType) -> Self {
        match other {
            DeviceType::Other => CDeviceType::Other,
            DeviceType::IntegratedGpu => CDeviceType::IntegratedGpu,
            DeviceType::DiscreteGpu => CDeviceType::DiscreteGpu,
            DeviceType::VirtualGpu => CDeviceType::VirtualGpu,
            DeviceType::Cpu => CDeviceType::Cpu,
        }
    }
}

#[repr(C)]
pub struct CAdapterInfo {
    /// Adapter name
    pub name: *mut c_char,
    /// Length of the adapter name
    pub name_length: usize,
    /// Vendor PCI id of the adapter
    pub vendor: usize,
    /// PCI id of the adapter
    pub device: usize,
    /// Type of device
    pub device_type: CDeviceType,
    /// Backend used for device
    pub backend: Backend,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_adapter_request_device(
    adapter_id: id::AdapterId,
    extensions: wgt::Extensions,
    limits: &CLimits,
    shader_validation: bool,
    trace_path: *const std::os::raw::c_char,
) -> id::DeviceId {
    let desc = DeviceDescriptor {
        extensions,
        limits: Limits {
            max_bind_groups: limits.max_bind_groups,
            ..Limits::default()
        },
        shader_validation,
    };
    let trace_cstr = if trace_path.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(trace_path))
    };
    let trace_cow = trace_cstr.as_ref().map(|cstr| cstr.to_string_lossy());
    let trace_path = trace_cow
        .as_ref()
        .map(|cow| std::path::Path::new(cow.as_ref()));
    gfx_select!(adapter_id => GLOBAL.adapter_request_device(adapter_id, &desc, trace_path, PhantomData))
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_extensions(adapter_id: id::AdapterId) -> wgt::Extensions {
    gfx_select!(adapter_id => GLOBAL.adapter_extensions(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_limits(adapter_id: id::AdapterId) -> CLimits {
    gfx_select!(adapter_id => GLOBAL.adapter_limits(adapter_id)).into()
}

pub fn adapter_get_info(adapter_id: id::AdapterId) -> wgc::instance::AdapterInfo {
    gfx_select!(adapter_id => GLOBAL.adapter_get_info(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_destroy(adapter_id: id::AdapterId) {
    gfx_select!(adapter_id => GLOBAL.adapter_destroy(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_extensions(device_id: id::DeviceId) -> wgt::Extensions {
    gfx_select!(device_id => GLOBAL.device_extensions(device_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_limits(device_id: id::DeviceId) -> CLimits {
    gfx_select!(device_id => GLOBAL.device_limits(device_id)).into()
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_buffer(
    device_id: id::DeviceId,
    desc: &wgt::BufferDescriptor<Label>,
) -> id::BufferId {
    gfx_select!(device_id => GLOBAL.device_create_buffer(device_id, desc, PhantomData))
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

#[repr(C)]
pub struct AnisotropicSamplerDescriptorExt<'c> {
    pub next_in_chain: Option<&'c ChainedStruct<'c>>,
    pub s_type: SType,
    pub anisotropic_clamp: u8,
}

#[repr(C)]
pub struct SamplerDescriptor<'c> {
    pub next_in_chain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub address_mode_u: wgt::AddressMode,
    pub address_mode_v: wgt::AddressMode,
    pub address_mode_w: wgt::AddressMode,
    pub mag_filter: wgt::FilterMode,
    pub min_filter: wgt::FilterMode,
    pub mipmap_filter: wgt::FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: wgt::CompareFunction,
}

fn map_sampler_descriptor(
    base: &SamplerDescriptor,
    anisotropic: Option<&AnisotropicSamplerDescriptorExt>,
) -> wgt::SamplerDescriptor<Label> {
    wgt::SamplerDescriptor {
        label: base.label,
        address_mode_u: base.address_mode_u,
        address_mode_v: base.address_mode_v,
        address_mode_w: base.address_mode_w,
        mag_filter: base.mag_filter,
        min_filter: base.min_filter,
        mipmap_filter: base.mipmap_filter,
        lod_min_clamp: base.lod_min_clamp,
        lod_max_clamp: base.lod_max_clamp,
        compare: match base.compare {
            wgt::CompareFunction::Undefined => None,
            cf => Some(cf),
        },
        anisotropy_clamp: anisotropic.map(|a| a.anisotropic_clamp),
        _non_exhaustive: unsafe { wgt::NonExhaustive::new() },
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_sampler(
    device_id: id::DeviceId,
    desc: &SamplerDescriptor,
) -> id::SamplerId {
    let full_desc = follow_chain!(map_sampler_descriptor(desc, AnisotropicFiltering => AnisotropicSamplerDescriptorExt));
    gfx_select!(device_id => GLOBAL.device_create_sampler(device_id, &full_desc, PhantomData))
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
    gfx_select!(device_id => GLOBAL.device_create_bind_group_layout(device_id, desc, PhantomData)).expect("Creation of pipeline layout failed")
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_layout_destroy(bind_group_layout_id: id::BindGroupLayoutId) {
    gfx_select!(bind_group_layout_id => GLOBAL.bind_group_layout_destroy(bind_group_layout_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_pipeline_layout(
    device_id: id::DeviceId,
    desc: &wgc::binding_model::PipelineLayoutDescriptor,
) -> id::PipelineLayoutId {
    gfx_select!(device_id => GLOBAL.device_create_pipeline_layout(device_id, desc, PhantomData)).expect("Creation of pipeline layout failed")
}

#[no_mangle]
pub extern "C" fn wgpu_pipeline_layout_destroy(pipeline_layout_id: id::PipelineLayoutId) {
    gfx_select!(pipeline_layout_id => GLOBAL.pipeline_layout_destroy(pipeline_layout_id))
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
pub extern "C" fn wgpu_shader_module_destroy(shader_module_id: id::ShaderModuleId) {
    gfx_select!(shader_module_id => GLOBAL.shader_module_destroy(shader_module_id))
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
/// This function is unsafe as there is no guarantee that the given `data`
/// pointer is valid for `data_length` elements.
#[no_mangle]
pub unsafe extern "C" fn wgpu_queue_write_buffer(
    queue_id: id::QueueId,
    buffer_id: id::BufferId,
    buffer_offset: wgt::BufferAddress,
    data: *const u8,
    data_length: usize,
) {
    let slice = slice::from_raw_parts(data, data_length);
    gfx_select!(queue_id => GLOBAL.queue_write_buffer(queue_id, buffer_id, buffer_offset, slice))
}

/// # Safety
///
/// This function is unsafe as there is no guarantee that the given `data`
/// pointer is valid for `data_length` elements.
#[no_mangle]
pub unsafe extern "C" fn wgpu_queue_write_texture(
    queue_id: id::QueueId,
    texture: &wgc::command::TextureCopyView,
    data: *const u8,
    data_length: usize,
    data_layout: &wgt::TextureDataLayout,
    size: &wgt::Extent3d,
) {
    let slice = slice::from_raw_parts(data, data_length);
    gfx_select!(queue_id => GLOBAL.queue_write_texture(queue_id, texture, slice, data_layout, size))
}

/// # Safety
///
/// This function is unsafe as there is no guarantee that the given `command_buffers`
/// pointer is valid for `command_buffers_length` elements.
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
    gfx_select!(device_id => GLOBAL.device_create_render_pipeline(device_id, desc, PhantomData)).expect("Creation of render pipeline failed")
}

#[no_mangle]
pub extern "C" fn wgpu_render_pipeline_destroy(render_pipeline_id: id::RenderPipelineId) {
    gfx_select!(render_pipeline_id => GLOBAL.render_pipeline_destroy(render_pipeline_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_compute_pipeline(
    device_id: id::DeviceId,
    desc: &wgc::pipeline::ComputePipelineDescriptor,
) -> id::ComputePipelineId {
    gfx_select!(device_id => GLOBAL.device_create_compute_pipeline(device_id, desc, PhantomData)).expect("Creation of compute pipeline failed")
}

#[no_mangle]
pub extern "C" fn wgpu_compute_pipeline_destroy(compute_pipeline_id: id::ComputePipelineId) {
    gfx_select!(compute_pipeline_id => GLOBAL.compute_pipeline_destroy(compute_pipeline_id))
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
    callback: wgc::resource::BufferMapCallback,
    user_data: *mut u8,
) {
    let operation = wgc::resource::BufferMapOperation {
        host: HostMap::Read,
        callback,
        user_data,
    };

    gfx_select!(buffer_id => GLOBAL.buffer_map_async(buffer_id, start .. start + size, operation))
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_map_write_async(
    buffer_id: id::BufferId,
    start: wgt::BufferAddress,
    size: wgt::BufferAddress,
    callback: wgc::resource::BufferMapCallback,
    user_data: *mut u8,
) {
    let operation = wgc::resource::BufferMapOperation {
        host: HostMap::Write,
        callback,
        user_data,
    };

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
}

#[no_mangle]
pub extern "C" fn wgpu_swap_chain_present(swap_chain_id: id::SwapChainId) {
    gfx_select!(swap_chain_id => GLOBAL.swap_chain_present(swap_chain_id))
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_get_mapped_range(
    buffer_id: id::BufferId,
    start: wgt::BufferAddress,
    size: wgt::BufferSize,
) -> *mut u8 {
    gfx_select!(buffer_id => GLOBAL.buffer_get_mapped_range(buffer_id, start, size))
}

/// Fills the given `info` struct with the adapter info.
///
/// # Safety
///
/// The field `info.name` is expected to point to a pre-allocated memory
/// location. This function is unsafe as there is no guarantee that the
/// pointer is valid and big enough to hold the adapter name.
#[no_mangle]
pub unsafe extern "C" fn wgpu_adapter_get_info(adapter_id: id::AdapterId, info: &mut CAdapterInfo) {
    let adapter_info = gfx_select!(adapter_id => GLOBAL.adapter_get_info(adapter_id));
    let adapter_name = CString::new(adapter_info.name).unwrap();

    info.device = adapter_info.device;
    info.vendor = adapter_info.vendor;
    info.device_type = CDeviceType::from(adapter_info.device_type);
    info.backend = adapter_info.backend;

    let string_bytes = adapter_name.as_bytes_with_nul();
    let cpy_length = match std::cmp::min(info.name_length, string_bytes.len()) {
        len if len > 0 => len,
        _ => return,
    };

    // Copies the string bytes owned into a the pre-allocated memory location
    // pointed by `info.name`.
    // NOTE: this is obviousy unsafe and the caller **must** ensure the
    // memory is allocated.
    ptr::copy(string_bytes.as_ptr(), info.name as *mut u8, cpy_length - 1);
    // Manually appends the null terminator. Depending on user input length,
    // we may not copy the entire string.
    info.name
        .offset((cpy_length - 1) as isize)
        .write('\0' as c_char);
}
