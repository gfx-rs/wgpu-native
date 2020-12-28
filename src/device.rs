use crate::{check_error, follow_chain, ChainedStruct, Label, OwnedLabel, SType, GLOBAL};

use wgc::{
    device::HostMap, gfx_select, hub::Token, id, pipeline::ShaderModuleSource,
};
use wgt::{Backend, BackendBit, DeviceType, Limits};

use libc::c_char;
use std::{
    borrow::Cow, ffi::CString, marker::PhantomData, num::NonZeroU32, num::NonZeroU64, ptr, slice,
};

pub type RequestAdapterCallback =
    unsafe extern "C" fn(id: id::AdapterId, userdata: *mut std::ffi::c_void);

// see https://github.com/rust-windowing/raw-window-handle/issues/49
struct PseudoRwh(raw_window_handle::RawWindowHandle);
unsafe impl raw_window_handle::HasRawWindowHandle for PseudoRwh {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.clone()
    }
}

#[repr(C)]
pub struct RenderBundleEncoderDescriptor {
    label: Label,
    color_formats: *const wgt::TextureFormat,
    color_formats_length: usize,
    depth_stencil_format: *const wgt::TextureFormat,
    sample_count: u32,
}

#[repr(C)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub buffer: Option<id::BufferId>,
    pub offset: wgt::BufferAddress,
    pub size: wgt::BufferSize,
    pub sampler: Option<id::SamplerId>,
    pub texture_view: Option<id::TextureViewId>,
}

#[repr(C)]
pub struct BindGroupDescriptor {
    pub label: Label,
    pub layout: id::BindGroupLayoutId,
    pub entries: *const BindGroupEntry,
    pub entries_length: usize,
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
pub unsafe extern "C" fn wgpu_create_surface_from_metal_layer(
    layer: *mut std::ffi::c_void,
) -> id::SurfaceId {
    let surface = wgc::instance::Surface {
        #[cfg(feature = "vulkan-portability")]
        vulkan: None, //TODO: currently requires `NSView`
        metal: GLOBAL
            .instance
            .metal
            .as_ref()
            .map(|inst| inst.create_surface_from_layer(std::mem::transmute(layer))),
    };

    let id = GLOBAL.surfaces.process_id(PhantomData);
    GLOBAL.surfaces.register(id, surface, &mut Token::root());

    id
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

#[no_mangle]
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
    let id = GLOBAL
        .request_adapter(
            &desc.cloned().unwrap_or(wgt::RequestAdapterOptions {
                power_preference: wgt::PowerPreference::default(),
                compatible_surface: None,
            }),
            wgc::instance::AdapterInputs::Mask(mask, |_| PhantomData),
        )
        .expect("Unable to request adapter");
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

#[repr(C)]
pub struct CDeviceDescriptor {
    label: Label,
    features: wgt::Features,
    limits: CLimits,
    trace_path: *const std::os::raw::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_adapter_request_device(
    adapter_id: id::AdapterId,
    desc: &CDeviceDescriptor,
) -> id::DeviceId {
    let trace_cstr = if desc.trace_path.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(desc.trace_path))
    };
    let trace_cow = trace_cstr.as_ref().map(|cstr| cstr.to_string_lossy());
    let trace_path = trace_cow
        .as_ref()
        .map(|cow| std::path::Path::new(cow.as_ref()));
    let desc = wgt::DeviceDescriptor {
        label: OwnedLabel::new(desc.label).into_cow(),
        features: desc.features,
        limits: Limits {
            max_bind_groups: desc.limits.max_bind_groups,
            ..Limits::default()
        },
    };
    check_error(
        gfx_select!(adapter_id => GLOBAL.adapter_request_device(adapter_id, &desc, trace_path, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_features(adapter_id: id::AdapterId) -> wgt::Features {
    gfx_select!(adapter_id => GLOBAL.adapter_features(adapter_id))
        .expect("Unable to get adapter features")
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_limits(adapter_id: id::AdapterId) -> CLimits {
    gfx_select!(adapter_id => GLOBAL.adapter_limits(adapter_id))
        .expect("Unable to get adapter limits")
        .into()
}

pub fn adapter_get_info(adapter_id: id::AdapterId) -> wgt::AdapterInfo {
    gfx_select!(adapter_id => GLOBAL.adapter_get_info(adapter_id))
        .expect("Unable to get adapter info")
}

#[no_mangle]
pub extern "C" fn wgpu_adapter_destroy(adapter_id: id::AdapterId) {
    gfx_select!(adapter_id => GLOBAL.adapter_drop(adapter_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_features(device_id: id::DeviceId) -> wgt::Features {
    gfx_select!(device_id => GLOBAL.device_features(device_id))
        .expect("Unable to get device features")
}

#[no_mangle]
pub extern "C" fn wgpu_device_limits(device_id: id::DeviceId) -> CLimits {
    gfx_select!(device_id => GLOBAL.device_limits(device_id))
        .expect("Unable to get device limits")
        .into()
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_buffer(
    device_id: id::DeviceId,
    desc: &wgt::BufferDescriptor<Label>,
) -> id::BufferId {
    let desc = desc.map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_buffer(device_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_destroy(buffer_id: id::BufferId, now: bool) {
    gfx_select!(buffer_id => GLOBAL.buffer_drop(buffer_id, now))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_texture(
    device_id: id::DeviceId,
    desc: &wgt::TextureDescriptor<Label>,
) -> id::TextureId {
    let desc = desc.map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_texture(device_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_texture_destroy(texture_id: id::TextureId, wait: bool) {
    gfx_select!(texture_id => GLOBAL.texture_drop(texture_id, wait))
}

#[repr(C)]
pub struct TextureViewDescriptor {
    pub label: Label,
    pub format: Option<wgt::TextureFormat>,
    pub dimension: Option<wgt::TextureViewDimension>,
    pub aspect: wgt::TextureAspect,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub array_layer_count: u32,
}

#[no_mangle]
pub extern "C" fn wgpu_texture_create_view(
    texture_id: id::TextureId,
    desc: Option<&TextureViewDescriptor>,
) -> id::TextureViewId {
    let desc = desc
        .map(|desc| wgc::resource::TextureViewDescriptor {
            label: OwnedLabel::new(desc.label).into_cow(),
            format: desc.format,
            dimension: desc.dimension,
            aspect: desc.aspect,
            base_mip_level: desc.base_mip_level,
            level_count: NonZeroU32::new(desc.level_count),
            base_array_layer: desc.base_array_layer,
            array_layer_count: NonZeroU32::new(desc.array_layer_count),
        })
        .unwrap_or(wgc::resource::TextureViewDescriptor::default());

    check_error(
        gfx_select!(texture_id => GLOBAL.texture_create_view(texture_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_texture_view_destroy(texture_view_id: id::TextureViewId) {
    gfx_select!(texture_view_id => GLOBAL.texture_view_drop(texture_view_id))
        .expect("Unable to destroy texture view")
}

#[repr(C)]
pub struct AnisotropicSamplerDescriptorExt<'c> {
    pub next_in_chain: Option<&'c ChainedStruct<'c>>,
    pub s_type: SType,
    pub anisotropic_clamp: u8,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum CompareFunction {
    Undefined,
    Never,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Always,
}

impl Into<Option<wgt::CompareFunction>> for CompareFunction {
    fn into(self) -> Option<wgt::CompareFunction> {
        match self {
            CompareFunction::Undefined => None,
            CompareFunction::Never => Some(wgt::CompareFunction::Never),
            CompareFunction::Less => Some(wgt::CompareFunction::Less),
            CompareFunction::LessEqual => Some(wgt::CompareFunction::LessEqual),
            CompareFunction::Greater => Some(wgt::CompareFunction::Greater),
            CompareFunction::GreaterEqual => Some(wgt::CompareFunction::GreaterEqual),
            CompareFunction::Equal => Some(wgt::CompareFunction::Equal),
            CompareFunction::NotEqual => Some(wgt::CompareFunction::NotEqual),
            CompareFunction::Always => Some(wgt::CompareFunction::Less),
        }
    }
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
    pub compare: CompareFunction,
    pub border_color: Option<wgt::SamplerBorderColor>,
}

unsafe fn map_sampler_descriptor<'a>(
    base: &SamplerDescriptor<'a>,
    anisotropic: Option<&AnisotropicSamplerDescriptorExt>,
) -> wgc::resource::SamplerDescriptor<'a> {
    wgc::resource::SamplerDescriptor {
        label: OwnedLabel::new(base.label).into_cow(),
        address_modes: [
            base.address_mode_u,
            base.address_mode_v,
            base.address_mode_w,
        ],
        mag_filter: base.mag_filter,
        min_filter: base.min_filter,
        mipmap_filter: base.mipmap_filter,
        lod_min_clamp: base.lod_min_clamp,
        lod_max_clamp: base.lod_max_clamp,
        compare: base.compare.into(),
        border_color: base.border_color,
        anisotropy_clamp: anisotropic.and_then(|a| std::num::NonZeroU8::new(a.anisotropic_clamp)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_sampler(
    device_id: id::DeviceId,
    desc: &SamplerDescriptor,
) -> id::SamplerId {
    let full_desc = follow_chain!(map_sampler_descriptor(desc, AnisotropicFiltering => AnisotropicSamplerDescriptorExt));
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_sampler(device_id, &full_desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_sampler_destroy(sampler_id: id::SamplerId) {
    gfx_select!(sampler_id => GLOBAL.sampler_drop(sampler_id))
}

#[repr(u32)]
pub enum BindingType {
    UniformBuffer = 0,
    StorageBuffer = 1,
    ReadonlyStorageBuffer = 2,
    Sampler = 3,
    ComparisonSampler = 4,
    SampledTexture = 5,
    ReadonlyStorageTexture = 6,
    WriteonlyStorageTexture = 7,
}

#[repr(u32)]
pub enum TextureComponentType {
    Float = 0,
    Sint = 1,
    Uint = 2,
    DepthComparison = 3,
}

impl TextureComponentType {
    fn into_wgpu(&self, filterable: bool) -> wgt::TextureSampleType {
        match self {
            TextureComponentType::Float => wgt::TextureSampleType::Float { filterable },
            TextureComponentType::Sint => wgt::TextureSampleType::Sint,
            TextureComponentType::Uint => wgt::TextureSampleType::Uint,
            TextureComponentType::DepthComparison => wgt::TextureSampleType::Depth,
        }
    }
}
#[repr(C)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: wgt::ShaderStage,
    pub ty: BindingType,
    pub has_dynamic_offset: bool,
    pub min_buffer_binding_size: u64,
    pub multisampled: bool,
    pub filtering: bool,
    pub view_dimension: wgt::TextureViewDimension,
    pub texture_component_type: TextureComponentType,
    pub storage_texture_format: wgt::TextureFormat,
    pub count: u32,
}

#[repr(C)]
pub struct BindGroupLayoutDescriptor {
    pub label: Label,
    pub entries: *const BindGroupLayoutEntry,
    pub entries_length: usize,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_bind_group_layout(
    device_id: id::DeviceId,
    desc: &BindGroupLayoutDescriptor,
) -> id::BindGroupLayoutId {
    let mut entries = Vec::new();
    for entry in slice::from_raw_parts(desc.entries, desc.entries_length) {
        let ty = match entry.ty {
            BindingType::UniformBuffer => wgt::BindingType::Buffer {
                ty: wgt::BufferBindingType::Uniform,
                has_dynamic_offset: entry.has_dynamic_offset,
                min_binding_size: NonZeroU64::new(entry.min_buffer_binding_size),
            },
            BindingType::StorageBuffer => wgt::BindingType::Buffer {
                ty: wgt::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: entry.has_dynamic_offset,
                min_binding_size: NonZeroU64::new(entry.min_buffer_binding_size),
            },
            BindingType::ReadonlyStorageBuffer => wgt::BindingType::Buffer {
                ty: wgt::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: entry.has_dynamic_offset,
                min_binding_size: NonZeroU64::new(entry.min_buffer_binding_size),
            },
            BindingType::Sampler => wgt::BindingType::Sampler {
                comparison: false,
                filtering: entry.filtering,
            },
            BindingType::ComparisonSampler => wgt::BindingType::Sampler {
                comparison: true,
                filtering: entry.filtering,
            },
            BindingType::SampledTexture => wgt::BindingType::Texture {
                view_dimension: entry.view_dimension,
                sample_type: entry.texture_component_type.into_wgpu(entry.filtering),
                multisampled: entry.multisampled,
            },
            BindingType::ReadonlyStorageTexture => wgt::BindingType::StorageTexture {
                view_dimension: entry.view_dimension,
                format: entry.storage_texture_format,
                access: wgt::StorageTextureAccess::ReadOnly,
            },
            BindingType::WriteonlyStorageTexture => wgt::BindingType::StorageTexture {
                view_dimension: entry.view_dimension,
                format: entry.storage_texture_format,
                access: wgt::StorageTextureAccess::WriteOnly,
            },
        };
        entries.push(wgt::BindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            ty,
            count: NonZeroU32::new(entry.count),
        });
    }
    let label = OwnedLabel::new(desc.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_bind_group_layout(device_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_layout_destroy(bind_group_layout_id: id::BindGroupLayoutId) {
    gfx_select!(bind_group_layout_id => GLOBAL.bind_group_layout_drop(bind_group_layout_id))
}

#[repr(C)]
pub struct PipelineLayoutDescriptor {
    label: Label,
    bind_group_layouts: *const id::BindGroupLayoutId,
    bind_group_layouts_length: usize,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_pipeline_layout(
    device_id: id::DeviceId,
    desc_base: &PipelineLayoutDescriptor,
) -> id::PipelineLayoutId {
    let desc = wgc::binding_model::PipelineLayoutDescriptor {
        label: OwnedLabel::new(desc_base.label).into_cow(),
        bind_group_layouts: Cow::Borrowed(slice::from_raw_parts(
            desc_base.bind_group_layouts,
            desc_base.bind_group_layouts_length,
        )),
        push_constant_ranges: Cow::Borrowed(&[]),
    };
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_pipeline_layout(device_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_pipeline_layout_destroy(pipeline_layout_id: id::PipelineLayoutId) {
    gfx_select!(pipeline_layout_id => GLOBAL.pipeline_layout_drop(pipeline_layout_id))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_bind_group(
    device_id: id::DeviceId,
    desc: &BindGroupDescriptor,
) -> id::BindGroupId {
    let entries = slice::from_raw_parts(desc.entries, desc.entries_length)
        .iter()
        .map(|entry| wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: if let Some(id) = entry.buffer {
                wgc::binding_model::BindingResource::Buffer(wgc::binding_model::BufferBinding {
                    buffer_id: id,
                    offset: entry.offset,
                    size: Some(entry.size),
                })
            } else if let Some(id) = entry.sampler {
                wgc::binding_model::BindingResource::Sampler(id)
            } else if let Some(id) = entry.texture_view {
                wgc::binding_model::BindingResource::TextureView(id)
            } else {
                panic!("Unknown binding!");
            },
        })
        .collect::<Vec<_>>();
    let label = OwnedLabel::new(desc.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: desc.layout,
        entries: Cow::Borrowed(&entries),
    };
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_bind_group(device_id, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_destroy(bind_group_id: id::BindGroupId) {
    gfx_select!(bind_group_id => GLOBAL.bind_group_drop(bind_group_id))
}

#[repr(C)]
pub struct ShaderModuleDescriptor {
    label: Label,
    bytes: *const u32,
    length: usize,
    flags: wgt::ShaderFlags,
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_shader_module(
    device_id: id::DeviceId,
    desc: &ShaderModuleDescriptor,
) -> id::ShaderModuleId {
    let slice = unsafe { std::slice::from_raw_parts(desc.bytes, desc.length) };
    let src = ShaderModuleSource::SpirV(Cow::Borrowed(slice));
    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: OwnedLabel::new(desc.label).into_cow(),
        flags: desc.flags,
    };
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_shader_module(device_id, &desc, src, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_shader_module_destroy(shader_module_id: id::ShaderModuleId) {
    gfx_select!(shader_module_id => GLOBAL.shader_module_drop(shader_module_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_command_encoder(
    device_id: id::DeviceId,
    desc: &wgt::CommandEncoderDescriptor<Label>,
) -> id::CommandEncoderId {
    let desc = &desc.map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device_id => GLOBAL.device_create_command_encoder(device_id, desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_destroy(command_encoder_id: id::CommandEncoderId) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_drop(command_encoder_id))
}

#[no_mangle]
pub extern "C" fn wgpu_command_buffer_destroy(command_buffer_id: id::CommandBufferId) {
    gfx_select!(command_buffer_id => GLOBAL.command_buffer_drop(command_buffer_id))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_render_bundle_encoder(
    device_id: id::DeviceId,
    desc: &RenderBundleEncoderDescriptor,
) -> id::RenderBundleEncoderId {
    let label = OwnedLabel::new(desc.label);
    let desc = wgc::command::RenderBundleEncoderDescriptor {
        label: label.as_cow(),
        color_formats: if desc.color_formats_length != 0 {
            Cow::Borrowed(slice::from_raw_parts(
                desc.color_formats,
                desc.color_formats_length,
            ))
        } else {
            Cow::Borrowed(&[])
        },
        depth_stencil_format: desc.depth_stencil_format.as_ref().cloned(),
        sample_count: desc.sample_count,
    };
    check_error(GLOBAL.device_create_render_bundle_encoder(device_id, &desc))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_render_bundle_encoder_finish(
    bundle_encoder_id: id::RenderBundleEncoderId,
    desc: Option<&wgt::RenderBundleDescriptor<Label>>,
) -> id::RenderBundleId {
    let bundle = *Box::from_raw(bundle_encoder_id);
    let desc = desc
        .map(|d| d.map_label(|l| OwnedLabel::new(*l).into_cow()))
        .unwrap_or(wgt::RenderBundleDescriptor { label: None });
    check_error(
        gfx_select!(bundle.parent() => GLOBAL.render_bundle_encoder_finish(bundle, &desc, PhantomData)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_render_bundle_destroy(render_bundle_id: id::RenderBundleId) {
    gfx_select!(render_bundle_id => GLOBAL.render_bundle_drop(render_bundle_id))
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
        .expect("Unable to write buffer")
}

/// # Safety
///
/// This function is unsafe as there is no guarantee that the given `data`
/// pointer is valid for `data_length` elements.
#[no_mangle]
pub unsafe extern "C" fn wgpu_queue_write_texture(
    queue_id: id::QueueId,
    texture: &crate::command::TextureCopyViewC,
    data: *const u8,
    data_length: usize,
    data_layout: &wgt::TextureDataLayout,
    size: &wgt::Extent3d,
) {
    let slice = slice::from_raw_parts(data, data_length);
    gfx_select!(queue_id => GLOBAL.queue_write_texture(queue_id, &texture.into_wgpu(), slice, data_layout, size))
        .expect("Unable to write texture")
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
        .expect("Unable to submit queue")
}

#[repr(C)]
#[derive(Clone)]
pub struct ProgrammableStageDescriptor {
    pub module: id::ShaderModuleId,
    pub entry_point: Label,
}

impl<'a> ProgrammableStageDescriptor {
    fn into_wgpu(&self) -> wgc::pipeline::ProgrammableStageDescriptor<'a> {
        wgc::pipeline::ProgrammableStageDescriptor {
            module: self.module,
            entry_point: OwnedLabel::new(self.entry_point).into_cow().unwrap(),
        }
    }
}

#[repr(C)]
pub struct VertexBufferDescriptor {
    pub stride: wgt::BufferAddress,
    pub step_mode: wgt::InputStepMode,
    pub attributes: *const wgt::VertexAttributeDescriptor,
    pub attributes_length: usize,
}

#[repr(C)]
pub struct VertexStateDescriptor {
    pub index_format: super::IndexFormat,
    pub vertex_buffers: *const VertexBufferDescriptor,
    pub vertex_buffers_length: usize,
}

#[repr(C)]
pub struct RenderPipelineDescriptor {
    pub label: Label,
    pub layout: Option<id::PipelineLayoutId>,
    pub vertex_stage: ProgrammableStageDescriptor,
    pub fragment_stage: *const ProgrammableStageDescriptor,
    pub rasterization_state: *const wgt::RasterizationStateDescriptor,
    pub primitive_topology: wgt::PrimitiveTopology,
    pub color_states: *const wgt::ColorStateDescriptor,
    pub color_states_length: usize,
    pub depth_stencil_state: *const wgt::DepthStencilStateDescriptor,
    pub vertex_state: VertexStateDescriptor,
    pub sample_count: u32,
    pub sample_mask: u32,
    pub alpha_to_coverage: bool,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_device_create_render_pipeline(
    device_id: id::DeviceId,
    desc_base: &RenderPipelineDescriptor,
) -> id::RenderPipelineId {
    let mut vertex_buffers = Vec::new();
    for vertex_buffer in slice::from_raw_parts(
        desc_base.vertex_state.vertex_buffers,
        desc_base.vertex_state.vertex_buffers_length,
    ) {
        vertex_buffers.push(wgc::pipeline::VertexBufferDescriptor {
            stride: vertex_buffer.stride,
            step_mode: vertex_buffer.step_mode,
            attributes: Cow::Borrowed(slice::from_raw_parts(
                vertex_buffer.attributes,
                vertex_buffer.attributes_length,
            )),
        });
    }

    let vertex_state = wgc::pipeline::VertexStateDescriptor {
        index_format: desc_base.vertex_state.index_format.into_wgpu(),
        vertex_buffers: Cow::Borrowed(&vertex_buffers),
    };

    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(desc_base.label).into_cow(),
        layout: desc_base.layout,
        vertex_stage: desc_base.vertex_stage.into_wgpu(),
        fragment_stage: desc_base.fragment_stage.as_ref().map(|f| f.into_wgpu()),
        rasterization_state: desc_base.rasterization_state.as_ref().map(|r| r.clone()),
        primitive_topology: desc_base.primitive_topology,
        color_states: Cow::Borrowed(slice::from_raw_parts(
            desc_base.color_states,
            desc_base.color_states_length,
        )),
        depth_stencil_state: desc_base.depth_stencil_state.as_ref().map(|d| d.clone()),
        vertex_state,
        sample_count: desc_base.sample_count,
        sample_mask: desc_base.sample_mask,
        alpha_to_coverage_enabled: desc_base.alpha_to_coverage,
    };
    let (id, _, error) = gfx_select!(device_id => GLOBAL.device_create_render_pipeline(device_id, &desc, PhantomData, None));
    if let Some(err) = error {
        panic!("{:?}", err);
    }
    id
}

#[no_mangle]
pub extern "C" fn wgpu_render_pipeline_destroy(render_pipeline_id: id::RenderPipelineId) {
    gfx_select!(render_pipeline_id => GLOBAL.render_pipeline_drop(render_pipeline_id))
}

#[repr(C)]
pub struct ComputePipelineDescriptor {
    pub label: Label,
    pub layout: Option<id::PipelineLayoutId>,
    pub compute_stage: ProgrammableStageDescriptor,
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_compute_pipeline(
    device_id: id::DeviceId,
    desc: &ComputePipelineDescriptor,
) -> id::ComputePipelineId {
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(desc.label).into_cow(),
        layout: desc.layout,
        compute_stage: desc.compute_stage.into_wgpu(),
    };

    let (id, _, error) = gfx_select!(device_id => GLOBAL.device_create_compute_pipeline(device_id, &desc, PhantomData, None));
    if let Some(err) = error {
        panic!("{:?}", err);
    }
    id
}

#[no_mangle]
pub extern "C" fn wgpu_compute_pipeline_destroy(compute_pipeline_id: id::ComputePipelineId) {
    gfx_select!(compute_pipeline_id => GLOBAL.compute_pipeline_drop(compute_pipeline_id))
}

#[no_mangle]
pub extern "C" fn wgpu_device_create_swap_chain(
    device_id: id::DeviceId,
    surface_id: id::SurfaceId,
    desc: &wgt::SwapChainDescriptor,
) -> id::SwapChainId {
    gfx_select!(device_id => GLOBAL.device_create_swap_chain(device_id, surface_id, desc))
        .expect("Unable to create swap chain")
}

#[no_mangle]
pub extern "C" fn wgpu_device_poll(device_id: id::DeviceId, force_wait: bool) {
    gfx_select!(device_id => GLOBAL.device_poll(device_id, force_wait))
        .expect("Unable to poll device")
}

#[no_mangle]
pub extern "C" fn wgpu_device_destroy(device_id: id::DeviceId) {
    gfx_select!(device_id => GLOBAL.device_drop(device_id))
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
        .expect("Unable to map buffer")
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
        .expect("Unable to map buffer")
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_unmap(buffer_id: id::BufferId) {
    gfx_select!(buffer_id => GLOBAL.buffer_unmap(buffer_id)).expect("Unable to unmap buffer")
}

#[no_mangle]
pub extern "C" fn wgpu_swap_chain_get_current_texture_view(
    swap_chain_id: id::SwapChainId,
) -> Option<id::TextureViewId> {
    gfx_select!(swap_chain_id => GLOBAL.swap_chain_get_current_texture_view(swap_chain_id, PhantomData))
        .expect("Unable to get swap chain texture view")
        .view_id
}

#[no_mangle]
pub extern "C" fn wgpu_swap_chain_present(swap_chain_id: id::SwapChainId) -> wgt::SwapChainStatus {
    gfx_select!(swap_chain_id => GLOBAL.swap_chain_present(swap_chain_id))
        .expect("Unable to present swap chain")
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_get_mapped_range(
    buffer_id: id::BufferId,
    start: wgt::BufferAddress,
    size: wgt::BufferSize,
) -> *mut u8 {
    gfx_select!(buffer_id => GLOBAL.buffer_get_mapped_range(buffer_id, start, Some(size)))
        .expect("Unable to get mapped range")
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
    let adapter_info = gfx_select!(adapter_id => GLOBAL.adapter_get_info(adapter_id))
        .expect("Unable to get adapter info");
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
