use crate::{check_error, follow_chain, ChainedStruct, Label, OwnedLabel, SType, GLOBAL, IndexFormat, make_slice};

use wgc::{
    device::HostMap, gfx_select, hub::Token, id, pipeline::ShaderModuleSource,
};
use wgt::{Backend, BackendBit, DeviceType, Limits};

use libc::c_char;
use std::{
    borrow::Cow, ffi::CString, marker::PhantomData, num::NonZeroU32, num::NonZeroU64, ptr,
};
use std::ffi::CStr;

pub type RequestAdapterCallback =
    unsafe extern "C" fn(id: id::AdapterId, userdata: *mut std::ffi::c_void);

// see https://github.com/rust-windowing/raw-window-handle/issues/49
struct PseudoRwh(raw_window_handle::RawWindowHandle);
unsafe impl raw_window_handle::HasRawWindowHandle for PseudoRwh {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.clone()
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RenderBundleEncoderDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub colorFormatsCount: u32,
    pub colorFormats: *const wgt::TextureFormat,
    pub depthStencilFormat: *const wgt::TextureFormat, // todo: remove *const when TextureFormat has Undefined variant
    pub sampleCount: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub buffer: Option<id::BufferId>,
    pub offset: u64,
    pub size: u64,
    pub sampler: Option<id::SamplerId>,
    pub textureView: Option<id::TextureViewId>,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct BindGroupDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub layout: id::BindGroupLayoutId,
    pub entryCount: u32,
    pub entries: *const BindGroupEntry,
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

#[allow(non_snake_case)]
#[repr(C)]
pub struct CDeviceDescriptor<'c> {
    nextInChain: Option<&'c ChainedStruct<'c>>,
    label: Label,
    // todo: move these to an extension
    features: wgt::Features,
    limits: CLimits,
    trace_path: *const std::os::raw::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuAdapterRequestDevice(
    adapter: id::AdapterId,
    descriptor: &CDeviceDescriptor,
) -> id::DeviceId {
    let trace_cstr = if descriptor.trace_path.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(descriptor.trace_path))
    };
    let trace_cow = trace_cstr.as_ref().map(|cstr| cstr.to_string_lossy());
    let trace_path = trace_cow
        .as_ref()
        .map(|cow| std::path::Path::new(cow.as_ref()));
    let desc = wgt::DeviceDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        features: descriptor.features,
        limits: Limits {
            max_bind_groups: descriptor.limits.max_bind_groups,
            ..Limits::default()
        },
    };
    check_error(
        gfx_select!(adapter => GLOBAL.adapter_request_device(adapter, &desc, trace_path, PhantomData)),
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

#[allow(non_snake_case)]
#[repr(C)]
pub struct CBufferDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub usage: wgt::BufferUsage,
    pub size: u64,
    pub mappedAtCreation: bool,
}

impl CBufferDescriptor<'_> {
    fn to_wgpu(&self) -> wgt::BufferDescriptor<Label> {
        wgt::BufferDescriptor {
            label: self.label,
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mappedAtCreation,
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateBuffer(
    device: id::DeviceId,
    descriptor: &CBufferDescriptor,
) -> id::BufferId {
    let desc = descriptor.to_wgpu().map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device => GLOBAL.device_create_buffer(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_buffer_destroy(buffer_id: id::BufferId, now: bool) {
    gfx_select!(buffer_id => GLOBAL.buffer_drop(buffer_id, now))
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct CTextureDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub usage: wgt::TextureUsage,
    pub dimension: wgt::TextureDimension,
    pub size: wgt::Extent3d,
    pub format: wgt::TextureFormat,
    pub mipLevelCount: u32,
    pub sampleCount: u32,
}

impl CTextureDescriptor<'_> {
    fn to_wgpu(&self) -> wgt::TextureDescriptor<Label> {
        wgt::TextureDescriptor {
            label: self.label,
            size: self.size,
            mip_level_count: self.mipLevelCount,
            sample_count: self.sampleCount,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateTexture(
    device: id::DeviceId,
    descriptor: &CTextureDescriptor,
) -> id::TextureId {
    let desc = descriptor.to_wgpu().map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device => GLOBAL.device_create_texture(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_texture_destroy(texture_id: id::TextureId, now: bool) {
    gfx_select!(texture_id => GLOBAL.texture_drop(texture_id, now))
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
pub extern "C" fn wgpu_texture_view_destroy(texture_view_id: id::TextureViewId, now: bool) {
    gfx_select!(texture_view_id => GLOBAL.texture_view_drop(texture_view_id, now))
        .expect("Unable to destroy texture view")
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct BorderClampColorDescriptorExt<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub sType: SType,
    pub borderColor: wgt::SamplerBorderColor,
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

#[allow(non_snake_case)]
#[repr(C)]
pub struct SamplerDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub addressModeU: wgt::AddressMode,
    pub addressModeV: wgt::AddressMode,
    pub addressModeW: wgt::AddressMode,
    pub magFilter: wgt::FilterMode,
    pub minFilter: wgt::FilterMode,
    pub mipmapFilter: wgt::FilterMode,
    pub lodMinClamp: f32,
    pub lodMaxClamp: f32,
    pub compare: wgt::CompareFunction,
    pub maxAnisotropy: u16,
}

unsafe fn map_sampler_descriptor<'a>(
    base: &SamplerDescriptor<'a>,
    border: Option<&BorderClampColorDescriptorExt>,
) -> wgc::resource::SamplerDescriptor<'a> {
    wgc::resource::SamplerDescriptor {
        label: OwnedLabel::new(base.label).into_cow(),
        address_modes: [
            base.addressModeU,
            base.addressModeV,
            base.addressModeW,
        ],
        mag_filter: base.magFilter,
        min_filter: base.minFilter,
        mipmap_filter: base.mipmapFilter,
        lod_min_clamp: base.lodMinClamp,
        lod_max_clamp: base.lodMaxClamp,
        compare: base.compare.into(),
        border_color: border.map(|b|b.borderColor),
        anisotropy_clamp: std::num::NonZeroU8::new(base.maxAnisotropy as u8),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateSampler(
    device: id::DeviceId,
    descriptor: &SamplerDescriptor,
) -> id::SamplerId {
    let full_desc = follow_chain!(map_sampler_descriptor(descriptor, BorderClampColor => BorderClampColorDescriptorExt));
    check_error(
        gfx_select!(device => GLOBAL.device_create_sampler(device, &full_desc, PhantomData)),
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

#[repr(C)]
#[derive(PartialEq)]
pub enum BufferBindingType {
    Undefined = 0,
    Uniform = 1,
    Storage = 2,
    ReadOnlyStorage = 3,
}

/// cbindgen:field-names=[nextInChain, type, hasDynamicOffset, minBindingSize]
#[allow(non_snake_case)]
#[repr(C)]
pub struct BufferBindingLayout<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub type_: BufferBindingType,
    pub hasDynamicOffset: bool,
    pub minBindingSize: u64,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum SamplerBindingType {
    Undefined = 0,
    Filtering = 1,
    NonFiltering = 2,
    Comparison = 3,
}

/// cbindgen:field-names=[nextInChain, type]
#[allow(non_snake_case)]
#[repr(C)]
pub struct SamplerBindingLayout<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub type_: SamplerBindingType,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum TextureSampleType {
    Undefined = 0,
    Float = 1,
    UnfilterableFloat = 2,
    Depth = 3,
    Sint = 4,
    Uint = 5,
}

impl TextureSampleType {
    fn to_wgpu(&self) -> wgt::TextureSampleType {
        match self {
            TextureSampleType::Undefined => panic!("invalid"),
            TextureSampleType::Float => wgt::TextureSampleType::Float { filterable: true },
            TextureSampleType::UnfilterableFloat => wgt::TextureSampleType::Float { filterable: false },
            TextureSampleType::Depth => wgt::TextureSampleType::Depth,
            TextureSampleType::Sint => wgt::TextureSampleType::Sint,
            TextureSampleType::Uint => wgt::TextureSampleType::Uint,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct TextureBindingLayout<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub sampleType: TextureSampleType,
    pub viewDimension: wgt::TextureViewDimension,
    pub multisampled: bool,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum StorageTextureAccess {
    Undefined = 0,
    ReadOnly = 1,
    WriteOnly = 2,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct StorageTextureBindingLayout<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub access: StorageTextureAccess,
    pub format: wgt::TextureFormat,
    pub viewDimension: wgt::TextureViewDimension,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct BindGroupLayoutEntry<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub binding: u32,
    pub visibility: wgt::ShaderStage,
    pub buffer: BufferBindingLayout<'c>,
    pub sampler: SamplerBindingLayout<'c>,
    pub texture: TextureBindingLayout<'c>,
    pub storageTexture: StorageTextureBindingLayout<'c>,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct BindGroupLayoutDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub entryCount: u32,
    pub entries: *const BindGroupLayoutEntry<'c>,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: id::DeviceId,
    descriptor: &BindGroupLayoutDescriptor,
) -> id::BindGroupLayoutId {
    let mut entries = Vec::new();
    for entry in make_slice(descriptor.entries, descriptor.entryCount as usize) {
        let ty = if entry.texture.sampleType != TextureSampleType::Undefined {
            match entry.texture.sampleType {
                TextureSampleType::Undefined => unreachable!(),
                _ => wgt::BindingType::Texture {
                    view_dimension: entry.texture.viewDimension,
                    sample_type: entry.texture.sampleType.to_wgpu(),
                    multisampled: entry.texture.multisampled,
                },
            }
        } else if entry.storageTexture.access != StorageTextureAccess::Undefined {
            match entry.storageTexture.access {
                StorageTextureAccess::Undefined => unreachable!(),
                StorageTextureAccess::ReadOnly => wgt::BindingType::StorageTexture {
                    view_dimension: entry.storageTexture.viewDimension,
                    format: entry.storageTexture.format,
                    access: wgt::StorageTextureAccess::ReadOnly,
                },
                StorageTextureAccess::WriteOnly => wgt::BindingType::StorageTexture {
                    view_dimension: entry.storageTexture.viewDimension,
                    format: entry.storageTexture.format,
                    access: wgt::StorageTextureAccess::WriteOnly,
                },
            }
        } else if entry.buffer.type_ != BufferBindingType::Undefined {
            match entry.buffer.type_ {
                BufferBindingType::Undefined => unreachable!(),
                BufferBindingType::Uniform => wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Uniform,
                    has_dynamic_offset: entry.buffer.hasDynamicOffset,
                    min_binding_size: NonZeroU64::new(entry.buffer.minBindingSize),
                },
                BufferBindingType::Storage => wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: entry.buffer.hasDynamicOffset,
                    min_binding_size: NonZeroU64::new(entry.buffer.minBindingSize),
                },
                BufferBindingType::ReadOnlyStorage => wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: entry.buffer.hasDynamicOffset,
                    min_binding_size: NonZeroU64::new(entry.buffer.minBindingSize),
                },
            }
        } else if entry.sampler.type_ != SamplerBindingType::Undefined {
            match entry.sampler.type_ {
                SamplerBindingType::Undefined => unreachable!(),
                SamplerBindingType::Filtering => wgt::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                SamplerBindingType::NonFiltering => wgt::BindingType::Sampler {
                    comparison: false,
                    filtering: false,
                },
                SamplerBindingType::Comparison => wgt::BindingType::Sampler {
                    comparison: true,
                    filtering: false,
                },
            }
        } else {
            panic!("no valid layout struct")
        };
        entries.push(wgt::BindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            ty,
            count: NonZeroU32::new(0), // todo
        });
    }
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupLayoutDescriptor {
        label: label.as_cow(),
        entries: Cow::Borrowed(&entries),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_bind_group_layout(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_layout_destroy(bind_group_layout_id: id::BindGroupLayoutId) {
    gfx_select!(bind_group_layout_id => GLOBAL.bind_group_layout_drop(bind_group_layout_id))
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct PipelineLayoutDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub bindGroupLayoutCount: u32,
    pub bindGroupLayouts: *const id::BindGroupLayoutId,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: id::DeviceId,
    descriptor: &PipelineLayoutDescriptor,
) -> id::PipelineLayoutId {
    let desc = wgc::binding_model::PipelineLayoutDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        bind_group_layouts: Cow::Borrowed(make_slice(
            descriptor.bindGroupLayouts,
            descriptor.bindGroupLayoutCount as usize,
        )),
        push_constant_ranges: Cow::Borrowed(&[]),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_pipeline_layout(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_pipeline_layout_destroy(pipeline_layout_id: id::PipelineLayoutId) {
    gfx_select!(pipeline_layout_id => GLOBAL.pipeline_layout_drop(pipeline_layout_id))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: id::DeviceId,
    descriptor: &BindGroupDescriptor,
) -> id::BindGroupId {
    let entries = make_slice(descriptor.entries, descriptor.entryCount as usize)
        .iter()
        .map(|entry| wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: if let Some(id) = entry.buffer {
                wgc::binding_model::BindingResource::Buffer(wgc::binding_model::BufferBinding {
                    buffer_id: id,
                    offset: entry.offset,
                    size: wgt::BufferSize::new(entry.size),
                })
            } else if let Some(id) = entry.sampler {
                wgc::binding_model::BindingResource::Sampler(id)
            } else if let Some(id) = entry.textureView {
                wgc::binding_model::BindingResource::TextureView(id)
            } else {
                panic!("Unknown binding!");
            },
        })
        .collect::<Vec<_>>();
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::binding_model::BindGroupDescriptor {
        label: label.as_cow(),
        layout: descriptor.layout,
        entries: Cow::Borrowed(&entries),
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_bind_group(device, &desc, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_bind_group_destroy(bind_group_id: id::BindGroupId) {
    gfx_select!(bind_group_id => GLOBAL.bind_group_drop(bind_group_id))
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct ShaderModuleDescriptor<'c> {
    nextInChain: Option<&'c ChainedStruct<'c>>,
    label: Label,
    flags: wgt::ShaderFlags, // todo: make this part of nextInChain
}

#[repr(C)]
pub struct ShaderModuleSPIRVDescriptor<'c> {
    chain: ChainedStruct<'c>,
    code_size: u32,
    code: *const u32,
}

#[repr(C)]
pub struct ShaderModuleWGSLDescriptor<'c> {
    chain: ChainedStruct<'c>,
    source: *const c_char
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateShaderModule(
    device: id::DeviceId,
    descriptor: &ShaderModuleDescriptor,
) -> id::ShaderModuleId {
    let chain = descriptor.nextInChain.expect("shader required");
    let src = match chain.sType {
        SType::ShaderModuleSPIRVDescriptor => {
            let desc: &ShaderModuleSPIRVDescriptor = unsafe { std::mem::transmute(chain) };
            let slice = unsafe { make_slice(desc.code, desc.code_size as usize) };
            ShaderModuleSource::SpirV(Cow::Borrowed(slice))
        }
        SType::ShaderModuleWGSLDescriptor => {
            let desc: &ShaderModuleWGSLDescriptor = unsafe { std::mem::transmute(chain) };
            let c_str: &CStr = unsafe { CStr::from_ptr(desc.source) };
            let str_slice: &str = c_str.to_str().expect("not a valid utf-8 string");
            ShaderModuleSource::Wgsl(Cow::Borrowed(str_slice))
        }
        _ => panic!("invalid type"),
    };
    let desc = wgc::pipeline::ShaderModuleDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        flags: descriptor.flags,
    };
    check_error(
        gfx_select!(device => GLOBAL.device_create_shader_module(device, &desc, src, PhantomData)),
    )
}

#[no_mangle]
pub extern "C" fn wgpu_shader_module_destroy(shader_module_id: id::ShaderModuleId) {
    gfx_select!(shader_module_id => GLOBAL.shader_module_drop(shader_module_id))
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct CommandEncoderDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
}

impl CommandEncoderDescriptor<'_> {
    fn to_wgpu(&self) -> wgt::CommandEncoderDescriptor<Label> {
        wgt::CommandEncoderDescriptor {
            label: self.label,
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: id::DeviceId,
    descriptor: &CommandEncoderDescriptor,
) -> id::CommandEncoderId {
    let desc = &descriptor.to_wgpu().map_label(|l| OwnedLabel::new(*l).into_cow());
    check_error(
        gfx_select!(device => GLOBAL.device_create_command_encoder(device, desc, PhantomData)),
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
pub unsafe extern "C" fn wgpuDeviceCreateRenderBundleEncoder(
    device: id::DeviceId,
    descriptor: &RenderBundleEncoderDescriptor,
) -> id::RenderBundleEncoderId {
    let label = OwnedLabel::new(descriptor.label);
    let desc = wgc::command::RenderBundleEncoderDescriptor {
        label: label.as_cow(),
        color_formats: Cow::Borrowed(make_slice(descriptor.colorFormats, descriptor.colorFormatsCount as usize)),
        depth_stencil_format: descriptor.depthStencilFormat.as_ref().cloned(),
        sample_count: descriptor.sampleCount,
    };
    check_error(GLOBAL.device_create_render_bundle_encoder(device, &desc))
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
pub extern "C" fn wgpuDeviceGetDefaultQueue(device: id::DeviceId) -> id::QueueId {
    device
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
    let slice = make_slice(data, data_length);
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
    let slice = make_slice(data, data_length);
    gfx_select!(queue_id => GLOBAL.queue_write_texture(queue_id, &texture.to_wgpu(), slice, data_layout, size))
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
    let command_buffer_ids = make_slice(command_buffers, command_buffers_length);
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
    fn to_wgpu(&self) -> wgc::pipeline::ProgrammableStageDescriptor<'a> {
        wgc::pipeline::ProgrammableStageDescriptor {
            module: self.module,
            entry_point: OwnedLabel::new(self.entry_point).into_cow().unwrap(),
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct BlendDescriptor {
    pub operation: wgt::BlendOperation,
    pub srcFactor: wgt::BlendFactor,
    pub dstFactor: wgt::BlendFactor,
}

impl BlendDescriptor {
    fn to_wgpu(&self) -> wgt::BlendState {
        wgt::BlendState {
            src_factor: self.srcFactor,
            dst_factor: self.dstFactor,
            operation: self.operation,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct ColorStateDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub format: wgt::TextureFormat,
    pub alphaBlend: BlendDescriptor,
    pub colorBlend: BlendDescriptor,
    pub writeMask: wgt::ColorWrite,
}

impl ColorStateDescriptor<'_> {
    fn to_wgpu(&self) -> wgt::ColorTargetState {
        wgt::ColorTargetState {
            format: self.format,
            alpha_blend: self.alphaBlend.to_wgpu(),
            color_blend: self.colorBlend.to_wgpu(),
            write_mask: self.writeMask,
        }
    }
}

#[repr(u32)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

impl CullMode {
    fn to_wgpu(&self) -> wgt::CullMode {
        match self {
            CullMode::None => wgt::CullMode::None,
            CullMode::Front => wgt::CullMode::Front,
            CullMode::Back => wgt::CullMode::Back,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct VertexAttributeDescriptor {
    pub format: wgt::VertexFormat,
    pub offset: u64,
    pub shaderLocation: u32,
}

impl VertexAttributeDescriptor {
    fn to_wgpu(&self) -> wgt::VertexAttribute {
        wgt::VertexAttribute {
            format: self.format,
            offset: self.offset,
            shader_location: self.shaderLocation,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct VertexBufferLayoutDescriptor {
    pub arrayStride: u64,
    pub stepMode: wgt::InputStepMode,
    pub attributeCount: u32,
    pub attributes: *const VertexAttributeDescriptor,
}

impl VertexBufferLayoutDescriptor {
    unsafe fn to_wgpu(&self) -> wgc::pipeline::VertexBufferLayout {
        wgc::pipeline::VertexBufferLayout {
            array_stride: self.arrayStride,
            step_mode: self.stepMode,
            attributes: Cow::Owned(make_slice(self.attributes, self.attributeCount as usize).iter().map(|x| x.to_wgpu()).collect()),
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct VertexStateDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub indexFormat: IndexFormat,
    pub vertexBufferCount: u32,
    pub vertexBuffers: *const VertexBufferLayoutDescriptor,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RasterizationStateDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub frontFace: wgt::FrontFace,
    pub cullMode: CullMode,
    pub depthBias: i32,
    pub depthBiasSlopeScale: f32,
    pub depthBiasClamp: f32,
    pub clampDepth: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct StencilStateFaceDescriptor {
    pub compare: CompareFunction,
    pub failOp: wgt::StencilOperation,
    pub depthFailOp: wgt::StencilOperation,
    pub passOp: wgt::StencilOperation,
}

impl StencilStateFaceDescriptor {
    fn to_wgpu(&self) -> wgt::StencilFaceState {
        let compare: Option<wgt::CompareFunction> = self.compare.into();
        wgt::StencilFaceState {
            compare: compare.expect("compare to be valid"),
            fail_op: self.failOp,
            depth_fail_op: self.depthFailOp,
            pass_op: self.passOp,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct DepthStencilStateDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub format: wgt::TextureFormat,
    pub depthWriteEnabled: bool,
    pub depthCompare: CompareFunction,
    pub stencilFront: StencilStateFaceDescriptor,
    pub stencilBack: StencilStateFaceDescriptor,
    pub stencilReadMask: u32,
    pub stencilWriteMask: u32,
}

impl DepthStencilStateDescriptor<'_> {
    fn to_wgpu(&self, s: &RasterizationStateDescriptor) -> wgt::DepthStencilState {
        let depth_compare: Option<wgt::CompareFunction> = self.depthCompare.into();
        wgt::DepthStencilState {
            format: self.format,
            depth_write_enabled: self.depthWriteEnabled,
            depth_compare: depth_compare.expect("depth_compare to be valid"),
            stencil: wgt::StencilState {
                front: self.stencilFront.to_wgpu(),
                back: self.stencilBack.to_wgpu(),
                read_mask: self.stencilReadMask,
                write_mask: self.stencilWriteMask,
            },
            bias: wgt::DepthBiasState {
                constant: 0, // todo: not in webgpu-headers
                slope_scale: s.depthBiasSlopeScale,
                clamp: s.depthBiasClamp,
            },
            clamp_depth: s.clampDepth,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RenderPipelineDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub layout: Option<id::PipelineLayoutId>,
    pub vertexStage: ProgrammableStageDescriptor,
    pub fragmentStage: *const ProgrammableStageDescriptor,
    pub vertexState: VertexStateDescriptor<'c>,
    pub primitiveTopology: wgt::PrimitiveTopology,
    pub rasterizationState: RasterizationStateDescriptor<'c>,
    pub sampleCount: u32,
    pub depthStencilState: *const DepthStencilStateDescriptor<'c>,
    pub colorStateCount: u32,
    pub colorStates: *const ColorStateDescriptor<'c>,
    pub sampleMask: u32,
    pub alphaToCoverageEnabled: bool,
}

#[no_mangle]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: id::DeviceId,
    descriptor: &RenderPipelineDescriptor,
) -> id::RenderPipelineId {
    let desc = wgc::pipeline::RenderPipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout,
        vertex: wgc::pipeline::VertexState {
            stage: descriptor.vertexStage.to_wgpu(),
            buffers: Cow::Owned(make_slice(descriptor.vertexState.vertexBuffers, descriptor.vertexState.vertexBufferCount as usize).iter().map(|x|x.to_wgpu()).collect()),
        },
        primitive: wgt::PrimitiveState {
            topology: descriptor.primitiveTopology,
            strip_index_format: descriptor.vertexState.indexFormat.to_wgpu(),
            front_face: descriptor.rasterizationState.frontFace,
            cull_mode: descriptor.rasterizationState.cullMode.to_wgpu(),
            polygon_mode: Default::default() // todo: not in webgpu-headers
        },
        depth_stencil: descriptor.depthStencilState.as_ref().map(|x|x.to_wgpu(&descriptor.rasterizationState)),
        multisample: wgt::MultisampleState {
            count: descriptor.sampleCount,
            mask: descriptor.sampleMask as u64,
            alpha_to_coverage_enabled: descriptor.alphaToCoverageEnabled,
        },
        fragment: descriptor.fragmentStage.as_ref().map(|x| wgc::pipeline::FragmentState {
            stage: x.to_wgpu(),
            targets: Cow::Owned(make_slice(descriptor.colorStates, descriptor.colorStateCount as usize).iter().map(|x|x.to_wgpu()).collect()),
        }
        ),
    };
    let (id, _, error) = gfx_select!(device => GLOBAL.device_create_render_pipeline(device, &desc, PhantomData, None));
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
    pub stage: ProgrammableStageDescriptor,
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateComputePipeline(
    device: id::DeviceId,
    descriptor: &ComputePipelineDescriptor,
) -> id::ComputePipelineId {
    let desc = wgc::pipeline::ComputePipelineDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        layout: descriptor.layout,
        stage: descriptor.stage.to_wgpu(),
    };

    let (id, _, error) = gfx_select!(device => GLOBAL.device_create_compute_pipeline(device, &desc, PhantomData, None));
    if let Some(err) = error {
        panic!("{:?}", err);
    }
    id
}

#[no_mangle]
pub extern "C" fn wgpu_compute_pipeline_destroy(compute_pipeline_id: id::ComputePipelineId) {
    gfx_select!(compute_pipeline_id => GLOBAL.compute_pipeline_drop(compute_pipeline_id))
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct SwapChainDescriptor<'c> {
    pub nextInChain: Option<&'c ChainedStruct<'c>>,
    pub label: Label,
    pub usage: wgt::TextureUsage,
    pub format: wgt::TextureFormat,
    pub width: u32,
    pub height: u32,
    pub presentMode: wgt::PresentMode,
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateSwapChain(
    device: id::DeviceId,
    surface: id::SurfaceId,
    descriptor: &SwapChainDescriptor,
) -> id::SwapChainId {
    let desc = wgt::SwapChainDescriptor {
        usage: descriptor.usage,
        format: descriptor.format,
        width: descriptor.width,
        height: descriptor.height,
        present_mode: descriptor.presentMode,
    };
    gfx_select!(device => GLOBAL.device_create_swap_chain(device, surface, &desc))
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
