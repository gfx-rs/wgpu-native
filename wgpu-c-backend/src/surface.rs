use wgpu::custom::*;
use wgpu_native::{native, *};

use crate::conv;
use crate::device::CDevice;
use crate::resource::CTexture;

// ── CSurface ──────────────────────────────────────────────────────────────────

pub struct CSurface {
    pub(crate) ptr: native::WGPUSurface,
}

impl std::fmt::Debug for CSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CSurface").field("ptr", &self.ptr).finish()
    }
}

unsafe impl Send for CSurface {}
unsafe impl Sync for CSurface {}

impl Drop for CSurface {
    fn drop(&mut self) {
        unsafe { wgpuSurfaceRelease(self.ptr) };
    }
}

impl SurfaceInterface for CSurface {
    fn get_capabilities(&self, adapter: &DispatchAdapter) -> wgpu::SurfaceCapabilities {
        let adapter_ptr = adapter.as_custom::<crate::adapter::CAdapter>().unwrap().ptr;
        // Chain an extras so wgpu-native also reports per-format color spaces.
        let mut fmt_extras: native::WGPUSurfaceCapabilitiesExtras = unsafe { std::mem::zeroed() };
        fmt_extras.chain.sType = native::WGPUSType_SurfaceCapabilitiesExtras;
        let mut caps: native::WGPUSurfaceCapabilities = unsafe { std::mem::zeroed() };
        caps.nextInChain = std::ptr::from_mut(&mut fmt_extras.chain);
        unsafe { wgpuSurfaceGetCapabilities(self.ptr, adapter_ptr, Some(&mut caps)) };

        let formats = unsafe {
            std::slice::from_raw_parts(caps.formats, caps.formatCount)
                .iter()
                .filter_map(|&f| conv::map_texture_format(f))
                .collect()
        };
        let present_modes = unsafe {
            std::slice::from_raw_parts(caps.presentModes, caps.presentModeCount)
                .iter()
                .filter_map(|&m| conv::map_present_mode(m).into())
                .collect()
        };
        let alpha_modes = unsafe {
            std::slice::from_raw_parts(caps.alphaModes, caps.alphaModeCount)
                .iter()
                .map(|&m| conv::map_composite_alpha(m))
                .collect()
        };
        let usages = conv::map_texture_usage(caps.usages);

        let format_capabilities = unsafe {
            std::slice::from_raw_parts(
                fmt_extras.formatCapabilities,
                fmt_extras.formatCapabilityCount,
            )
            .iter()
            .filter_map(|fc| {
                conv::map_texture_format(fc.format).map(|format| {
                    wgpu::wgt::SurfaceFormatCapabilities {
                        format,
                        color_spaces: conv::map_surface_color_spaces(fc.colorSpaces),
                    }
                })
            })
            .collect()
        };

        unsafe { wgpuSurfaceCapabilitiesFreeMembers(caps) };

        wgpu::SurfaceCapabilities {
            formats,
            present_modes,
            alpha_modes,
            usages,
            format_capabilities,
        }
    }

    fn configure(&self, device: &DispatchDevice, config: &wgpu::SurfaceConfiguration) {
        let device_ptr = device.as_custom::<CDevice>().unwrap().ptr;
        let view_formats: Vec<native::WGPUTextureFormat> = config
            .view_formats
            .iter()
            .map(|&f| conv::texture_format_to_native(f))
            .collect();
        let mut sc_extras = native::WGPUSurfaceConfigurationExtras {
            chain: native::WGPUChainedStruct {
                next: std::ptr::null_mut(),
                sType: native::WGPUSType_SurfaceConfigurationExtras,
            },
            desiredMaximumFrameLatency: config.desired_maximum_frame_latency,
            colorSpace: conv::surface_color_space_to_native(config.color_space),
        };
        let c_config = native::WGPUSurfaceConfiguration {
            nextInChain: std::ptr::from_mut::<native::WGPUChainedStruct>(&mut sc_extras.chain),
            device: device_ptr,
            format: conv::texture_format_to_native(config.format),
            usage: conv::texture_usage_to_native(config.usage),
            width: config.width,
            height: config.height,
            viewFormatCount: view_formats.len(),
            viewFormats: if view_formats.is_empty() {
                std::ptr::null()
            } else {
                view_formats.as_ptr()
            },
            // present_mode_to_native returns None for modes wgpu-native can't represent;
            // fall back to Fifo (guaranteed supported) rather than panicking.
            presentMode: conv::present_mode_to_native(config.present_mode)
                .unwrap_or(native::WGPUPresentMode_Fifo),
            alphaMode: conv::composite_alpha_to_native(config.alpha_mode),
        };
        unsafe { wgpuSurfaceConfigure(self.ptr, Some(&c_config)) };
    }

    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        wgpu::SurfaceStatus,
        DispatchSurfaceOutputDetail,
    ) {
        let mut surface_texture: native::WGPUSurfaceTexture = unsafe { std::mem::zeroed() };
        unsafe { wgpuSurfaceGetCurrentTexture(self.ptr, Some(&mut surface_texture)) };

        let status = conv::surface_status_from_native(surface_texture.status);
        let texture = if !surface_texture.texture.is_null() {
            Some(DispatchTexture::custom(CTexture {
                ptr: surface_texture.texture,
            }))
        } else {
            None
        };

        let detail = DispatchSurfaceOutputDetail::custom(CSurfaceOutputDetail {
            surface_ptr: self.ptr,
        });

        (texture, status, detail)
    }
}

// ── CSurfaceOutputDetail ──────────────────────────────────────────────────────

pub struct CSurfaceOutputDetail {
    pub(crate) surface_ptr: native::WGPUSurface,
}

impl std::fmt::Debug for CSurfaceOutputDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CSurfaceOutputDetail").finish()
    }
}

unsafe impl Send for CSurfaceOutputDetail {}
unsafe impl Sync for CSurfaceOutputDetail {}

impl SurfaceOutputDetailInterface for CSurfaceOutputDetail {
    fn texture_discard(&self) {
        unsafe { wgpuSurfaceDiscardTexture(self.surface_ptr) };
    }

    // v30 added `texture_release` (called only on drop-during-panic to release
    // the acquired frame). The C API has no dedicated release primitive, so
    // discard performs the same best-effort cleanup.
    fn texture_release(&self) {
        unsafe { wgpuSurfaceDiscardTexture(self.surface_ptr) };
    }
}
