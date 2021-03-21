use crate::{check_error, make_slice, map_enum, native, OwnedLabel, GLOBAL};
use std::{borrow::Cow, num::NonZeroU32};
use wgc::{
    command::{compute_ffi, render_ffi},
    gfx_select, id,
};

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    encoder: id::CommandEncoderId,
    desc: &native::WGPUCommandBufferDescriptor,
) -> id::CommandBufferId {
    let desc = wgt::CommandBufferDescriptor {
        label: OwnedLabel::new(desc.label).into_cow(),
    };

    check_error(gfx_select!(encoder => GLOBAL.command_encoder_finish(encoder, &desc)))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToBuffer(
    command_encoder: id::CommandEncoderId,
    source: id::BufferId,
    source_offset: u64,
    destination: id::BufferId,
    destination_offset: u64,
    size: u64,
) {
    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_buffer_to_buffer(
        command_encoder,
        source, 
        source_offset,
        destination,
        destination_offset,
        size))
    .expect("Unable to copy buffer to buffer")
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    command_encoder: id::CommandEncoderId,
    source: &native::WGPUImageCopyTexture,
    destination: &native::WGPUImageCopyBuffer,
    copy_size: &native::WGPUExtent3D,
) {
    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_texture_to_buffer(
        command_encoder,
        &map_image_copy_texture(source),
        &map_image_copy_buffer(destination),
        &map_extent3d(copy_size)))
    .expect("Unable to copy texture to buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginComputePass(
    encoder: id::CommandEncoderId,
    descriptor: &native::WGPUComputePassDescriptor,
) -> id::ComputePassEncoderId {
    let desc = wgc::command::ComputePassDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
    };
    let pass = wgc::command::ComputePass::new(encoder, &desc);
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    encoder: id::CommandEncoderId,
    descriptor: native::WGPURenderPassDescriptor,
) -> id::RenderPassEncoderId {
    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::DepthStencilAttachmentDescriptor {
            attachment: desc.attachment,
            depth: wgc::command::PassChannel {
                load_op: map_load_op(desc.depthLoadOp),
                store_op: map_store_op(desc.depthStoreOp),
                clear_value: desc.clearDepth,
                read_only: desc.depthReadOnly,
            },
            stencil: wgc::command::PassChannel {
                load_op: map_load_op(desc.stencilLoadOp),
                store_op: map_store_op(desc.stencilStoreOp),
                clear_value: desc.clearStencil,
                read_only: desc.stencilReadOnly,
            },
        }
    });
    let desc = wgc::command::RenderPassDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
        color_attachments: Cow::Owned(
            make_slice(
                descriptor.colorAttachments,
                descriptor.colorAttachmentCount as usize,
            )
            .iter()
            .map(|color_attachment| wgc::command::ColorAttachmentDescriptor {
                attachment: color_attachment.attachment,
                resolve_target: Some(color_attachment.resolveTarget),
                channel: wgc::command::PassChannel {
                    load_op: map_load_op(color_attachment.loadOp),
                    store_op: map_store_op(color_attachment.storeOp),
                    clear_value: wgt::Color {
                        r: color_attachment.clearColor.r,
                        g: color_attachment.clearColor.g,
                        b: color_attachment.clearColor.b,
                        a: color_attachment.clearColor.a,
                    },
                    read_only: false,
                },
            })
            .collect(),
        ),
        depth_stencil_attachment: depth_stencil_attachment.as_ref(),
    };
    let pass = wgc::command::RenderPass::new(encoder, &desc);
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEndPass(pass: id::ComputePassEncoderId) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_compute_pass(encoder_id, &pass))
        .expect("Unable to end compute pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEndPass(pass: id::RenderPassEncoderId) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_render_pass(encoder_id, &pass))
        .expect("Unable to end render pass");
}

// TODO: Move these out of wgc
#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: id::ComputePassEncoderId,
    pipeline_id: id::ComputePipelineId,
) {
    let pass = pass.as_mut().expect("Compute pass invalid");
    compute_ffi::wgpu_compute_pass_set_pipeline(pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: id::RenderPassEncoderId,
    pipeline_id: id::RenderPipelineId,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_pipeline(pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: id::ComputePassEncoderId,
    groupIndex: u32,
    group: id::BindGroupId,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("Compute pass invalid");
    compute_ffi::wgpu_compute_pass_set_bind_group(
        pass,
        groupIndex,
        group,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: id::RenderPassEncoderId,
    groupIndex: u32,
    group: id::BindGroupId,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_bind_group(
        pass,
        groupIndex,
        group,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatch(
    pass: id::ComputePassEncoderId,
    x: u32,
    y: u32,
    z: u32,
) {
    let pass = pass.as_mut().expect("Compute pass invalid");
    compute_ffi::wgpu_compute_pass_dispatch(pass, x, y, z);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: id::RenderPassEncoderId,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_draw(
        pass,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

pub fn map_extent3d(native: &native::WGPUExtent3D) -> wgt::Extent3d {
    wgt::Extent3d {
        width: native.width,
        height: native.height,
        depth_or_array_layers: native.depth,
    }
}

pub fn map_origin3d(native: &native::WGPUOrigin3D) -> wgt::Origin3d {
    wgt::Origin3d {
        x: native.x,
        y: native.y,
        z: native.z,
    }
}

pub fn map_image_copy_texture(
    native: &native::WGPUImageCopyTexture,
) -> wgc::command::ImageCopyTexture {
    wgt::ImageCopyTexture {
        texture: native.texture,
        mip_level: native.mipLevel,
        origin: map_origin3d(&native.origin),
    }
}

pub fn map_image_copy_buffer(
    native: &native::WGPUImageCopyBuffer,
) -> wgc::command::ImageCopyBuffer {
    wgt::ImageCopyBuffer {
        buffer: native.buffer,
        layout: map_texture_data_layout(&native.layout),
    }
}

pub fn map_texture_data_layout(native: &native::WGPUTextureDataLayout) -> wgt::ImageDataLayout {
    wgt::ImageDataLayout {
        offset: native.offset,
        bytes_per_row: NonZeroU32::new(native.bytesPerRow),
        rows_per_image: NonZeroU32::new(native.rowsPerImage),
    }
}

map_enum!(
    map_load_op,
    WGPULoadOp,
    wgc::command::LoadOp,
    "Unknown load op",
    Clear,
    Load
);
map_enum!(
    map_store_op,
    WGPUStoreOp,
    wgc::command::StoreOp,
    "Unknown store op",
    Clear,
    Store
);
