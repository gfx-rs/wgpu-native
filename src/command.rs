use crate::{conv, make_slice, native, OwnedLabel, GLOBAL};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{borrow::Cow, num::NonZeroU64};
use wgc::{
    command::{compute_ffi, render_ffi},
    gfx_select,
};

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUCommandBufferDescriptor>,
) -> native::WGPUCommandBuffer {
    let command_encoder = command_encoder.expect("invalid command encoder");

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandBufferDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandBufferDescriptor::default(),
    };

    let (id, error) =
        gfx_select!(command_encoder => GLOBAL.command_encoder_finish(command_encoder, &desc));
    if let Some(error) = error {
        // TODO figure out what device the encoder belongs to and call
        // handle_device_error()
        log::error!("command_encoder_finish() failed: {:?}", error);
        None
    } else {
        Some(id)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderClearBuffer(
    command_encoder: native::WGPUCommandEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let command_encoder = command_encoder.expect("invalid command encoder");
    let buffer = buffer.expect("invalid buffer");

    gfx_select!(command_encoder => GLOBAL.command_encoder_clear_buffer(
        command_encoder,
        buffer,
        offset,
        NonZeroU64::new(size)))
    .expect("Unable to clear buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToBuffer(
    command_encoder: native::WGPUCommandEncoder,
    source: native::WGPUBuffer,
    source_offset: u64,
    destination: native::WGPUBuffer,
    destination_offset: u64,
    size: u64,
) {
    let command_encoder = command_encoder.expect("invalid command encoder");
    let source = source.expect("invalid source");
    let destination = destination.expect("invalid destination");

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
    command_encoder: native::WGPUCommandEncoder,
    source: &native::WGPUImageCopyTexture,
    destination: &native::WGPUImageCopyTexture,
    copy_size: &native::WGPUExtent3D,
) {
    let command_encoder = command_encoder.expect("invalid command encoder");

    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_texture_to_texture(
        command_encoder,
        &conv::map_image_copy_texture(source),
        &conv::map_image_copy_texture(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy texture to texture")
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    command_encoder: native::WGPUCommandEncoder,
    source: &native::WGPUImageCopyTexture,
    destination: &native::WGPUImageCopyBuffer,
    copy_size: &native::WGPUExtent3D,
) {
    let command_encoder = command_encoder.expect("invalid command encoder");

    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_texture_to_buffer(
        command_encoder,
        &conv::map_image_copy_texture(source),
        &conv::map_image_copy_buffer(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy texture to buffer")
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    command_encoder: native::WGPUCommandEncoder,
    source: &native::WGPUImageCopyBuffer,
    destination: &native::WGPUImageCopyTexture,
    copy_size: &native::WGPUExtent3D,
) {
    let command_encoder = command_encoder.expect("invalid command encoder");

    gfx_select!(command_encoder => GLOBAL.command_encoder_copy_buffer_to_texture(
        command_encoder,
        &conv::map_image_copy_buffer(source),
        &conv::map_image_copy_texture(destination),
        &conv::map_extent3d(copy_size)))
    .expect("Unable to copy buffer to texture")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginComputePass(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUComputePassDescriptor>,
) -> native::WGPUComputePassEncoder {
    let command_encoder = command_encoder.expect("invalid command encoder");

    let desc = match descriptor {
        Some(descriptor) => wgc::command::ComputePassDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgc::command::ComputePassDescriptor::default(),
    };
    let pass = wgc::command::ComputePass::new(command_encoder, &desc);
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    encoder: native::WGPUCommandEncoder,
    descriptor: &native::WGPURenderPassDescriptor,
) -> native::WGPURenderPassEncoder {
    let encoder = encoder.expect("invalid command encoder");

    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::RenderPassDepthStencilAttachment {
            view: desc
                .view
                .expect("invalid texture view for depth stencil attachment"),
            depth: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.depthLoadOp),
                store_op: conv::map_store_op(desc.depthStoreOp),
                clear_value: desc.depthClearValue,
                read_only: desc.depthReadOnly,
            },
            stencil: wgc::command::PassChannel {
                load_op: conv::map_load_op(desc.stencilLoadOp),
                store_op: conv::map_store_op(desc.stencilStoreOp),
                clear_value: desc.stencilClearValue,
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
            .map(|color_attachment| {
                color_attachment
                    .view
                    .map(|view| wgc::command::RenderPassColorAttachment {
                        view,
                        resolve_target: color_attachment.resolveTarget,
                        channel: wgc::command::PassChannel {
                            load_op: conv::map_load_op(color_attachment.loadOp),
                            store_op: conv::map_store_op(color_attachment.storeOp),
                            clear_value: conv::map_color(&color_attachment.clearValue),
                            read_only: false,
                        },
                    })
            })
            .collect(),
        ),
        depth_stencil_attachment: depth_stencil_attachment.as_ref(),
    };
    let pass = wgc::command::RenderPass::new(encoder, &desc);
    Box::into_raw(Box::new(pass))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderInsertDebugMarker(
    encoder: native::WGPUCommandEncoder,
    marker_label: *const c_char,
) {
    let encoder = encoder.expect("invalid command encoder");

    gfx_select!(encoder => GLOBAL.command_encoder_insert_debug_marker(encoder, CStr::from_ptr(marker_label).to_str().unwrap()))
        .expect("Unable to insert debug marker");
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderPopDebugGroup(encoder: native::WGPUCommandEncoder) {
    let encoder = encoder.expect("invalid command encoder");

    gfx_select!(encoder => GLOBAL.command_encoder_pop_debug_group(encoder))
        .expect("Unable to pop debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPushDebugGroup(
    encoder: native::WGPUCommandEncoder,
    group_label: *const c_char,
) {
    let encoder = encoder.expect("invalid command encoder");

    gfx_select!(encoder => GLOBAL.command_encoder_push_debug_group(encoder, CStr::from_ptr(group_label).to_str().unwrap()))
        .expect("Unable to push debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass: native::WGPUComputePassEncoder) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_compute_pass(encoder_id, &pass))
        .expect("Unable to end compute pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass: native::WGPURenderPassEncoder) {
    let pass = Box::from_raw(pass);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_render_pass(encoder_id, &pass))
        .expect("Unable to end render pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: native::WGPUComputePassEncoder,
    pipeline_id: native::WGPUComputePipeline,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    let pipeline_id = pipeline_id.expect("invalid compute pipeline");

    compute_ffi::wgpu_compute_pass_set_pipeline(pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: native::WGPURenderPassEncoder,
    pipeline_id: native::WGPURenderPipeline,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let pipeline_id = pipeline_id.expect("invalid render pipeline");
    render_ffi::wgpu_render_pass_set_pipeline(pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: native::WGPUComputePassEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    let group = group.expect("invalid bind group");

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
    pass: native::WGPURenderPassEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let group = group.expect("invalid bind group");

    render_ffi::wgpu_render_pass_set_bind_group(
        pass,
        group_index,
        group,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroups(
    pass: native::WGPUComputePassEncoder,
    workgroup_count_x: u32,
    workgroup_count_y: u32,
    workgroup_count_z: u32,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    compute_ffi::wgpu_compute_pass_dispatch_workgroups(
        pass,
        workgroup_count_x,
        workgroup_count_y,
        workgroup_count_z,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroupsIndirect(
    pass: native::WGPUComputePassEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    let indirect_buffer = indirect_buffer.expect("invalid indirect buffer");

    compute_ffi::wgpu_compute_pass_dispatch_workgroups_indirect(
        pass,
        indirect_buffer,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderInsertDebugMarker(
    pass: native::WGPUComputePassEncoder,
    marker_label: *const c_char,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    compute_ffi::wgpu_compute_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPopDebugGroup(pass: native::WGPUComputePassEncoder) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    compute_ffi::wgpu_compute_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    pass: native::WGPUComputePassEncoder,
    group_label: *const c_char,
) {
    let pass = pass.as_mut().expect("invalid compute pass encoder");
    compute_ffi::wgpu_compute_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: native::WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
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
    pass: native::WGPURenderPassEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: u32,
    first_instance: u32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
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
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let buffer = buffer.expect("invalid buffer");

    render_ffi::wgpu_render_pass_draw_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let buffer = buffer.expect("invalid buffer");

    render_ffi::wgpu_render_pass_draw_indexed_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    index_format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let buffer = buffer.expect("invalid buffer");

    pass.set_index_buffer(
        buffer,
        conv::map_index_format(index_format).expect("Index format cannot be undefined"),
        offset,
        NonZeroU64::new(size),
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    pass: native::WGPURenderPassEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    let buffer = buffer.expect("invalid buffer");

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
    pass: native::WGPURenderPassEncoder,
    stages: native::WGPUShaderStage,
    offset: u32,
    size_bytes: u32,
    size: *const u8,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_set_push_constants(
        pass,
        wgt::ShaderStages::from_bits(stages as u32).expect("Invalid shader stage"),
        offset,
        size_bytes,
        size,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: native::WGPURenderPassEncoder,
    color: &native::WGPUColor,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_set_blend_constant(pass, &conv::map_color(color));
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    pass: native::WGPURenderPassEncoder,
    reference: u32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_set_stencil_reference(pass, reference);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetViewport(
    pass: native::WGPURenderPassEncoder,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    depth_min: f32,
    depth_max: f32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_set_viewport(pass, x, y, w, h, depth_min, depth_max);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    pass: native::WGPURenderPassEncoder,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_set_scissor_rect(pass, x, y, w, h);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    pass: native::WGPURenderPassEncoder,
    marker_label: *const c_char,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPopDebugGroup(pass: native::WGPURenderPassEncoder) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    pass: native::WGPURenderPassEncoder,
    group_label: *const c_char,
) {
    let pass = pass.as_mut().expect("invalid render pass encoder");
    render_ffi::wgpu_render_pass_push_debug_group(pass, group_label, 0);
}
