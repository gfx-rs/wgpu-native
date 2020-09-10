use crate::GLOBAL;

pub use wgc::command::{bundle_ffi::*, compute_ffi::*, render_ffi::*, ComputePass, RenderPass};

use wgc::{gfx_select, id};

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_finish(
    encoder_id: id::CommandEncoderId,
    desc: Option<&wgt::CommandBufferDescriptor>,
) -> id::CommandBufferId {
    let desc = &desc.cloned().unwrap_or_default();
    gfx_select!(encoder_id => GLOBAL.command_encoder_finish(encoder_id, desc))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_buffer_to_buffer(
    command_encoder_id: id::CommandEncoderId,
    source: id::BufferId,
    source_offset: wgt::BufferAddress,
    destination: id::BufferId,
    destination_offset: wgt::BufferAddress,
    size: wgt::BufferAddress,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_buffer_to_buffer(
        command_encoder_id,
        source, source_offset,
        destination,
        destination_offset,
        size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_buffer_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &wgc::command::BufferCopyView,
    destination: &wgc::command::TextureCopyView,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_buffer_to_texture(
        command_encoder_id,
        source,
        destination,
        copy_size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_buffer(
    command_encoder_id: id::CommandEncoderId,
    source: &wgc::command::TextureCopyView,
    destination: &wgc::command::BufferCopyView,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_buffer(
        command_encoder_id,
        source,
        destination,
        copy_size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &wgc::command::TextureCopyView,
    destination: &wgc::command::TextureCopyView,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_texture(
        command_encoder_id,
        source,
        destination,
        copy_size))
}

type RenderPassColorAttachmentDescriptor =
    wgt::RenderPassColorAttachmentDescriptorBase<wgc::id::TextureViewId>;
type RenderPassDepthStencilAttachmentDescriptor =
    wgt::RenderPassDepthStencilAttachmentDescriptorBase<wgc::id::TextureViewId>;

#[repr(C)]
pub struct RenderPassDescriptor<'a> {
    color_attachments: &'a RenderPassColorAttachmentDescriptor,
    color_attachments_length: usize,
    depth_stencil_attachment: Option<&'a RenderPassDepthStencilAttachmentDescriptor>,
}

impl<'a> RenderPassDescriptor<'a> {
    fn to_wgpu_type(&self) -> wgc::command::RenderPassDescriptor<'a> {
        let color_attachments = unsafe {
            std::slice::from_raw_parts(self.color_attachments, self.color_attachments_length)
        };

        wgc::command::RenderPassDescriptor {
            color_attachments,
            depth_stencil_attachment: self.depth_stencil_attachment,
        }
    }
}

/// # Safety
///
/// This function is unsafe because improper use may lead to memory
/// problems. For example, a double-free may occur if the function is called
/// twice on the same raw pointer.
#[no_mangle]
pub unsafe extern "C" fn wgpu_command_encoder_begin_render_pass(
    encoder_id: id::CommandEncoderId,
    desc: &RenderPassDescriptor,
) -> *mut RenderPass {
    let pass = wgc::command::RenderPass::new(encoder_id, desc.to_wgpu_type());
    Box::into_raw(Box::new(pass))
}

/// # Safety
///
/// This function is unsafe because improper use may lead to memory
/// problems. For example, a double-free may occur if the function is called
/// twice on the same raw pointer.
#[no_mangle]
pub unsafe extern "C" fn wgpu_render_pass_end_pass(pass: *mut RenderPass) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_render_pass(encoder_id, &pass));
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_render_pass_destroy(pass: *mut RenderPass) {
    let _ = Box::from_raw(pass);
}

/// # Safety
///
/// This function is unsafe because improper use may lead to memory
/// problems. For example, a double-free may occur if the function is called
/// twice on the same raw pointer.
#[no_mangle]
pub unsafe extern "C" fn wgpu_command_encoder_begin_compute_pass(
    encoder_id: id::CommandEncoderId,
    _desc: Option<&wgc::command::ComputePassDescriptor>,
) -> *mut ComputePass {
    let pass = wgc::command::ComputePass::new(encoder_id);
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_compute_pass_end_pass(pass: *mut ComputePass) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_compute_pass(encoder_id, &pass));
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_compute_pass_destroy(pass: *mut ComputePass) {
    let _ = Box::from_raw(pass);
}
