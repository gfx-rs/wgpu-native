use crate::{check_error, Label, OwnedLabel, GLOBAL};

pub use wgc::command::{
    bundle_ffi, compute_ffi, render_ffi, ColorAttachmentDescriptor, ComputePass,
    DepthStencilAttachmentDescriptor, RenderBundleEncoder, RenderCommand, RenderPass,
};

use std::borrow::Cow;
use wgc::{gfx_select, id};
use wgt::{BufferAddress, BufferSize};

#[repr(C)]
pub struct CommandBufferDescriptor {
    pub label: Label,
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_command_encoder_finish(
    encoder_id: id::CommandEncoderId,
    desc_base: Option<&CommandBufferDescriptor>,
) -> id::CommandBufferId {
    let desc = wgt::CommandBufferDescriptor {
        label: desc_base.and_then(|c| OwnedLabel::new(c.label).into_cow()),
    };

    check_error(gfx_select!(encoder_id => GLOBAL.command_encoder_finish(encoder_id, &desc)))
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
    .expect("Unable to copy buffer to buffer")
}

#[repr(C)]
pub struct BufferCopyViewC {
    pub layout: wgt::TextureDataLayout,
    pub buffer: id::BufferId,
}

impl BufferCopyViewC {
    pub fn into_wgpu(&self) -> wgc::command::BufferCopyView {
        wgc::command::BufferCopyView {
            layout: self.layout.clone(),
            buffer: self.buffer,
        }
    }
}

#[repr(C)]
pub struct TextureCopyViewC {
    pub texture: id::TextureId,
    pub mip_level: u32,
    pub origin: wgt::Origin3d,
}

impl TextureCopyViewC {
    pub fn into_wgpu(&self) -> wgc::command::TextureCopyView {
        wgc::command::TextureCopyView {
            texture: self.texture,
            mip_level: self.mip_level,
            origin: self.origin,
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_buffer_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &BufferCopyViewC,
    destination: &TextureCopyViewC,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_buffer_to_texture(
        command_encoder_id,
        &source.into_wgpu(),
        &destination.into_wgpu(),
        copy_size))
    .expect("Unable to copy buffer to texture")
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_buffer(
    command_encoder_id: id::CommandEncoderId,
    source: &TextureCopyViewC,
    destination: &BufferCopyViewC,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_buffer(
        command_encoder_id,
        &source.into_wgpu(),
        &destination.into_wgpu(),
        copy_size))
    .expect("Unable to copy texture to buffer")
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &TextureCopyViewC,
    destination: &TextureCopyViewC,
    copy_size: &wgt::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_texture(
        command_encoder_id,
        &source.into_wgpu(),
        &destination.into_wgpu(),
        copy_size))
    .expect("Unable to copy texture to texture")
}

#[repr(C)]
pub struct RenderPassDescriptor<'a> {
    color_attachments: &'a ColorAttachmentDescriptor,
    color_attachments_length: usize,
    depth_stencil_attachment: Option<&'a DepthStencilAttachmentDescriptor>,
    label: Label,
}

impl<'a> RenderPassDescriptor<'a> {
    fn to_wgpu_type(&self) -> wgc::command::RenderPassDescriptor<'a> {
        let color_attachments = Cow::Borrowed(unsafe {
            std::slice::from_raw_parts(self.color_attachments, self.color_attachments_length)
        });

        wgc::command::RenderPassDescriptor {
            color_attachments,
            depth_stencil_attachment: self.depth_stencil_attachment,
            label: OwnedLabel::new(self.label).into_cow(),
        }
    }
}

#[repr(C)]
pub struct ComputePassDescriptor {
    label: Label,
}

impl<'a> ComputePassDescriptor {
    fn to_wgpu_type(&self) -> wgc::command::ComputePassDescriptor {
        wgc::command::ComputePassDescriptor {
            label: OwnedLabel::new(self.label).into_cow(),
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
    let pass = wgc::command::RenderPass::new(encoder_id, &desc.to_wgpu_type());
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
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_render_pass(encoder_id, &pass))
        .expect("Unable to end render pass");
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
    desc: &ComputePassDescriptor,
) -> *mut ComputePass {
    let pass = wgc::command::ComputePass::new(encoder_id, &desc.to_wgpu_type());
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_compute_pass_end_pass(pass: *mut ComputePass) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_compute_pass(encoder_id, &pass))
        .expect("Unable to end compute pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_compute_pass_destroy(pass: *mut ComputePass) {
    let _ = Box::from_raw(pass);
}

#[no_mangle]
pub extern "C" fn wgpu_render_pass_set_index_buffer(
    pass: &mut RenderPass,
    buffer_id: id::BufferId,
    index_format: super::IndexFormat,
    offset: BufferAddress,
    size: Option<BufferSize>,
) {
    pass.set_index_buffer(
        buffer_id,
        index_format
            .into_wgpu()
            .expect("IndexFormat cannot be undefined"),
        offset,
        size,
    );
}

#[no_mangle]
pub extern "C" fn wgpu_render_bundle_set_index_buffer(
    bundle: &mut RenderBundleEncoder,
    buffer_id: id::BufferId,
    index_format: super::IndexFormat,
    offset: BufferAddress,
    size: Option<BufferSize>,
) {
    bundle.set_index_buffer(
        buffer_id,
        index_format
            .into_wgpu()
            .expect("IndexFormat cannot be undefined"),
        offset,
        size,
    );
}
