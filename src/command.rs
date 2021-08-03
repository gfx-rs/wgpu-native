use crate::{check_error, conv, make_slice, native, OwnedLabel, GLOBAL};
use std::{borrow::Cow, num::NonZeroU64};
use wgc::{
    command::{compute_ffi, render_ffi},
    gfx_select, id,
};

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    encoder: id::CommandEncoderId,
    descriptor: &native::WGPUCommandBufferDescriptor,
) -> id::CommandBufferId {
    let desc = wgt::CommandBufferDescriptor {
        label: OwnedLabel::new(descriptor.label).into_cow(),
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
pub extern "C" fn wgpuCommandEncoderCopyTextureToTexture(
    command_encoder: id::CommandEncoderId,
    source: &native::WGPUImageCopyTexture,
    destination: &native::WGPUImageCopyTexture,
    copy_size: &native::WGPUExtent3D,
) {
    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_texture_to_texture(
        command_encoder,
        &conv::map_image_copy_texture(source),
        &conv::map_image_copy_texture(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy texture to texture")
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
        &conv::map_image_copy_texture(source),
        &conv::map_image_copy_buffer(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy texture to buffer")
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    command_encoder: id::CommandEncoderId,
    source: &native::WGPUImageCopyBuffer,
    destination: &native::WGPUImageCopyTexture,
    copy_size: &native::WGPUExtent3D,
) {
    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_buffer_to_texture(
        command_encoder,
        &conv::map_image_copy_buffer(source),
        &conv::map_image_copy_texture(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy buffer to texture")
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
    descriptor: &native::WGPURenderPassDescriptor,
) -> id::RenderPassEncoderId {
    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::RenderPassDepthStencilAttachment {
            view: desc.view,
            depth: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.depthLoadOp),
                store_op: conv::map_store_op(desc.depthStoreOp),
                clear_value: desc.clearDepth,
                read_only: desc.depthReadOnly,
            },
            stencil: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.stencilLoadOp),
                store_op: conv::map_store_op(desc.stencilStoreOp),
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
            .map(|color_attachment| wgc::command::RenderPassColorAttachment {
                view: color_attachment.view,
                resolve_target: Some(color_attachment.resolveTarget),
                channel: wgc::command::PassChannel {
                    load_op: conv::map_load_op(color_attachment.loadOp),
                    store_op: conv::map_store_op(color_attachment.storeOp),
                    clear_value: conv::map_color(&color_attachment.clearColor),
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
    group_index: u32,
    group: id::BindGroupId,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("Compute pass invalid");
    compute_ffi::wgpu_compute_pass_set_bind_group(
        pass,
        group_index,
        group,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: id::RenderPassEncoderId,
    group_index: u32,
    group: id::BindGroupId,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_bind_group(
        pass,
        group_index,
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
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchIndirect(
    pass: id::ComputePassEncoderId,
    indirect_buffer: id::BufferId,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("Compute pass invalid");
    compute_ffi::wgpu_compute_pass_dispatch_indirect(pass, indirect_buffer, indirect_offset);
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

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexed(
    pass: id::RenderPassEncoderId,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: u32,
    first_instance: u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_draw_indexed(
        pass,
        index_count,
        instance_count,
        first_index,
        base_vertex as i32,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndirect(
    pass: id::RenderPassEncoderId,
    buffer: id::BufferId,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_draw_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: id::RenderPassEncoderId,
    buffer: id::BufferId,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_draw_indexed_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: id::RenderPassEncoderId,
    buffer: id::BufferId,
    index_format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    pass.set_index_buffer(
        buffer,
        conv::map_index_format(index_format).expect("Index format cannot be undefined"),
        offset,
        NonZeroU64::new(size),
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    pass: id::RenderPassEncoderId,
    slot: u32,
    buffer: id::BufferId,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_vertex_buffer(
        pass,
        slot,
        buffer,
        offset,
        NonZeroU64::new(size),
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPushConstants(
    pass: id::RenderPassEncoderId,
    stages: &native::WGPUShaderStage,
    offset: u32,
    size_bytes: u32,
    size: *const u8,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_push_constants(
        pass,
        wgt::ShaderStages::from_bits(*stages as u32).expect("Invalid shader stage"),
        offset,
        size_bytes,
        size,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: id::RenderPassEncoderId,
    color: &native::WGPUColor,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_blend_constant(pass, &conv::map_color(color));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    pass: id::RenderPassEncoderId,
    reference: u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_stencil_reference(pass, reference);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetViewport(
    pass: id::RenderPassEncoderId,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    depth_min: f32,
    depth_max: f32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_viewport(pass, x, y, w, h, depth_min, depth_max);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    pass: id::RenderPassEncoderId,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    let pass = pass.as_mut().expect("Render pass invalid");
    render_ffi::wgpu_render_pass_set_scissor_rect(pass, x, y, w, h);
}
