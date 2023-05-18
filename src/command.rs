use crate::utils::{make_slice, OwnedLabel};
use crate::{conv, handle_device_error, native};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{borrow::Cow, num::NonZeroU64};
use wgc::{
    command::{bundle_ffi, compute_ffi, render_ffi},
    gfx_select,
};

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUCommandBufferDescriptor>,
) -> native::WGPUCommandBuffer {
    assert!(!command_encoder.is_null(), "invalid command encoder");

    // NOTE: Automatically drop the encoder
    let command_encoder = Box::from_raw(command_encoder);
    let context = &command_encoder.context;
    let command_encoder_id = command_encoder.id;

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandBufferDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandBufferDescriptor::default(),
    };

    let (command_buffer_id, error) = gfx_select!(command_encoder_id => context.command_encoder_finish(command_encoder_id, &desc));
    if let Some(error) = error {
        // TODO figure out what device the encoder belongs to and call
        // handle_device_error()
        log::error!("command_encoder_finish() failed: {:?}", error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPUCommandBufferImpl {
            context: context.clone(),
            id: command_buffer_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderClearBuffer(
    command_encoder: native::WGPUCommandEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    gfx_select!(command_encoder_id => context.command_encoder_clear_buffer(
        command_encoder_id,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        }
    ))
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
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };
    let source_buffer_id = source.as_ref().expect("invalid source").id;
    let destination_buffer_id = destination.as_ref().expect("invalid destination").id;

    gfx_select!(command_encoder_id => context.command_encoder_copy_buffer_to_buffer(
        command_encoder_id,
        source_buffer_id,
        source_offset,
        destination_buffer_id,
        destination_offset,
        size))
    .expect("Unable to copy buffer to buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToTexture(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyTexture>,
    destination: Option<&native::WGPUImageCopyTexture>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_copy_texture_to_texture(
        command_encoder_id,
        &conv::map_image_copy_texture(source.expect("invalid source")),
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))))
    .expect("Unable to copy texture to texture")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyTexture>,
    destination: Option<&native::WGPUImageCopyBuffer>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_copy_texture_to_buffer(
        command_encoder_id,
        &conv::map_image_copy_texture(source.expect("invalid source")),
        &conv::map_image_copy_buffer(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))))
    .expect("Unable to copy texture to buffer")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    command_encoder: native::WGPUCommandEncoder,
    source: Option<&native::WGPUImageCopyBuffer>,
    destination: Option<&native::WGPUImageCopyTexture>,
    copy_size: Option<&native::WGPUExtent3D>,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_copy_buffer_to_texture(
        command_encoder_id,
        &conv::map_image_copy_buffer(source.expect("invalid source")),
        &conv::map_image_copy_texture(destination.expect("invalid destination")),
        &conv::map_extent3d(copy_size.expect("invalid copy size"))))
    .expect("Unable to copy buffer to texture")
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginComputePass(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUComputePassDescriptor>,
) -> native::WGPUComputePassEncoder {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    let desc = match descriptor {
        Some(descriptor) => wgc::command::ComputePassDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgc::command::ComputePassDescriptor::default(),
    };

    Box::into_raw(Box::new(native::WGPUComputePassEncoderImpl {
        context: context.clone(),
        encoder: wgc::command::ComputePass::new(command_encoder_id, &desc),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPURenderPassDescriptor>,
) -> native::WGPURenderPassEncoder {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };
    let descriptor = descriptor.expect("invalid descriptor");

    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::RenderPassDepthStencilAttachment {
            view: desc
                .view
                .as_ref()
                .expect("invalid texture view for depth stencil attachment")
                .id,
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
                    .as_ref()
                    .map(|view| wgc::command::RenderPassColorAttachment {
                        view: view.id,
                        resolve_target: color_attachment.resolveTarget.as_ref().map(|v| v.id),
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

    Box::into_raw(Box::new(native::WGPURenderPassEncoderImpl {
        context: context.clone(),
        encoder: wgc::command::RenderPass::new(command_encoder_id, &desc),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderInsertDebugMarker(
    command_encoder: native::WGPUCommandEncoder,
    marker_label: *const c_char,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_insert_debug_marker(command_encoder_id, CStr::from_ptr(marker_label).to_str().unwrap()))
        .expect("Unable to insert debug marker");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPopDebugGroup(
    command_encoder: native::WGPUCommandEncoder,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_pop_debug_group(command_encoder_id))
        .expect("Unable to pop debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPushDebugGroup(
    command_encoder: native::WGPUCommandEncoder,
    group_label: *const c_char,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };

    gfx_select!(command_encoder_id => context.command_encoder_push_debug_group(command_encoder_id, CStr::from_ptr(group_label).to_str().unwrap()))
        .expect("Unable to push debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderResolveQuerySet(
    command_encoder: native::WGPUCommandEncoder,
    query_set: native::WGPUQuerySet,
    first_query: u32,
    query_count: u32,
    destination: native::WGPUBuffer,
    destination_offset: u64,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };
    let query_set_id = query_set.as_ref().expect("invalid query set").id;
    let destination_buffer_id = destination.as_ref().expect("invalid destination").id;

    gfx_select!(command_encoder_id => context.command_encoder_resolve_query_set(
        command_encoder_id,
        query_set_id,
        first_query,
        query_count,
        destination_buffer_id,
        destination_offset))
    .expect("Unable to resolve query set");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderWriteTimestamp(
    command_encoder: native::WGPUCommandEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let (command_encoder_id, context) = {
        let command_encoder = command_encoder.as_ref().expect("invalid command encoder");
        (command_encoder.id, &command_encoder.context)
    };
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    gfx_select!(command_encoder_id => context.command_encoder_write_timestamp(
        command_encoder_id,
        query_set_id,
        query_index))
    .expect("Unable to write timestamp");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass: native::WGPUComputePassEncoder) {
    assert!(!pass.is_null(), "invalid compute pass");

    // NOTE: Automatically drops the compute pass
    let pass = Box::from_raw(pass);
    let context = &pass.context;
    let command_encoder_id = pass.encoder.parent_id();

    gfx_select!(command_encoder_id => context.command_encoder_run_compute_pass(command_encoder_id, &pass.encoder))
        .expect("Unable to end compute pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass: native::WGPURenderPassEncoder) {
    assert!(!pass.is_null(), "invalid render pass");

    // NOTE: Automatically drops the render pass
    let pass = Box::from_raw(pass);
    let context = &pass.context;
    let command_encoder_id = pass.encoder.parent_id();

    gfx_select!(command_encoder_id => context.command_encoder_run_render_pass(command_encoder_id, &pass.encoder))
        .expect("Unable to end render pass");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: native::WGPUComputePassEncoder,
    compute_pipeline: native::WGPUComputePipeline,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let compute_pipeline_id = compute_pipeline
        .as_ref()
        .expect("invalid compute pipeline")
        .id;

    compute_ffi::wgpu_compute_pass_set_pipeline(pass, compute_pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: native::WGPURenderPassEncoder,
    render_pipeline: native::WGPURenderPipeline,
) {
    let render_pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let render_pipeline_id = render_pipeline
        .as_ref()
        .expect("invalid render pipeline")
        .id;

    render_ffi::wgpu_render_pass_set_pipeline(render_pass, render_pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: native::WGPUComputePassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;

    compute_ffi::wgpu_compute_pass_set_bind_group(
        pass,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: native::WGPURenderPassEncoder,
    group_index: u32,
    bind_group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let bind_group_id = bind_group.as_ref().expect("invalid bind group").id;

    render_ffi::wgpu_render_pass_set_bind_group(
        pass,
        group_index,
        bind_group_id,
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
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;

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
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_mut()
        .expect("invalid indirect buffer")
        .id;

    compute_ffi::wgpu_compute_pass_dispatch_workgroups_indirect(
        pass,
        indirect_buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderInsertDebugMarker(
    pass: native::WGPUComputePassEncoder,
    marker_label: *const c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPopDebugGroup(pass: native::WGPUComputePassEncoder) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    pass: native::WGPUComputePassEncoder,
    group_label: *const c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    compute_ffi::wgpu_compute_pass_begin_pipeline_statistics_query(pass, query_set_id, query_index);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPUComputePassEncoder,
) {
    let pass = &mut pass.as_mut().expect("invalid compute pass").encoder;
    compute_ffi::wgpu_compute_pass_end_pipeline_statistics_query(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: native::WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
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
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
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
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_draw_indirect(pass, buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_draw_indexed_indirect(pass, buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indirect(pass, buffer_id, offset, count);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect(pass, buffer_id, offset, count);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirectCount(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count_buffer: native::WGPUBuffer,
    count_buffer_offset: u64,
    max_count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indirect_count(
        pass,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count_buffer: native::WGPUBuffer,
    count_buffer_offset: u64,
    max_count: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;
    let count_buffer_id = count_buffer.as_ref().expect("invalid count buffer").id;

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect_count(
        pass,
        buffer_id,
        offset,
        count_buffer_id,
        count_buffer_offset,
        max_count,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    index_format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    pass.set_index_buffer(
        buffer_id,
        conv::map_index_format(index_format).expect("Index format cannot be undefined"),
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
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
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    render_ffi::wgpu_render_pass_set_vertex_buffer(
        pass,
        slot,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPushConstants(
    pass: native::WGPURenderPassEncoder,
    stages: native::WGPUShaderStageFlags,
    offset: u32,
    size_bytes: u32,
    size: *const u8,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_push_constants(
        pass,
        wgt::ShaderStages::from_bits(stages).expect("invalid shader stage"),
        offset,
        size_bytes,
        size,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: native::WGPURenderPassEncoder,
    color: Option<&native::WGPUColor>,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_blend_constant(
        pass,
        &conv::map_color(color.expect("invalid color")),
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    pass: native::WGPURenderPassEncoder,
    reference: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

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
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

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
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_set_scissor_rect(pass, x, y, w, h);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    pass: native::WGPURenderPassEncoder,
    marker_label: *const c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPopDebugGroup(pass: native::WGPURenderPassEncoder) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    pass: native::WGPURenderPassEncoder,
    group_label: *const c_char,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    render_ffi::wgpu_render_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderExecuteBundles(
    pass: native::WGPURenderPassEncoder,
    bundle_count: u32,
    bundles: *const native::WGPURenderBundle,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;

    let bundle_ids = make_slice(bundles, bundle_count as usize)
        .iter()
        .map(|v| v.as_ref().expect("invalid render bundle").id)
        .collect::<Vec<wgc::id::RenderBundleId>>();

    render_ffi::wgpu_render_pass_execute_bundles(pass, bundle_ids.as_ptr(), bundle_ids.len());
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderBeginPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
    query_set: native::WGPUQuerySet,
    query_index: u32,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    let query_set_id = query_set.as_ref().expect("invalid query set").id;

    render_ffi::wgpu_render_pass_begin_pipeline_statistics_query(pass, query_set_id, query_index);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEndPipelineStatisticsQuery(
    pass: native::WGPURenderPassEncoder,
) {
    let pass = &mut pass.as_mut().expect("invalid render pass").encoder;
    render_ffi::wgpu_render_pass_end_pipeline_statistics_query(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDraw(
    bundle: native::WGPURenderBundleEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;

    bundle_ffi::wgpu_render_bundle_draw(
        bundle,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexed(
    bundle: native::WGPURenderBundleEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;

    bundle_ffi::wgpu_render_bundle_draw_indexed(
        bundle,
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexedIndirect(
    bundle: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;

    bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
        bundle,
        indirect_buffer_id,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndirect(
    bundle: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let indirect_buffer_id = indirect_buffer
        .as_ref()
        .expect("invalid indirect buffer")
        .id;

    bundle_ffi::wgpu_render_bundle_draw_indirect(bundle, indirect_buffer_id, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderFinish(
    bundle: native::WGPURenderBundleEncoder,
    descriptor: Option<&native::WGPURenderBundleDescriptor>,
) -> native::WGPURenderBundle {
    assert!(!bundle.is_null(), "invalid render bundle");

    // NOTE: Automatically drops the bundle encoder
    let bundle = Box::from_raw(bundle);
    let context = &bundle.context;
    let bundle_encoder = bundle.encoder;
    let device_id = bundle_encoder.parent();

    let desc = match descriptor {
        Some(descriptor) => wgt::RenderBundleDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::RenderBundleDescriptor::default(),
    };

    let (render_bundle_id, error) =
        gfx_select!(device_id => context.render_bundle_encoder_finish(bundle_encoder, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device_id, &error);
        std::ptr::null_mut()
    } else {
        Box::into_raw(Box::new(native::WGPURenderBundleImpl {
            context: context.clone(),
            id: render_bundle_id,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderInsertDebugMarker(
    bundle: native::WGPURenderBundleEncoder,
    marker_label: *const c_char,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_insert_debug_marker(bundle, marker_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPopDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_pop_debug_group(bundle);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPushDebugGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_label: *const c_char,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    bundle_ffi::wgpu_render_bundle_push_debug_group(bundle, group_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetBindGroup(
    bundle: native::WGPURenderBundleEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let bind_group_id = group.as_ref().expect("invalid bind group").id;
    bundle_ffi::wgpu_render_bundle_set_bind_group(
        bundle,
        group_index,
        bind_group_id,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetIndexBuffer(
    bundle: native::WGPURenderBundleEncoder,
    buffer: native::WGPUBuffer,
    format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    bundle_ffi::wgpu_render_bundle_set_index_buffer(
        bundle,
        buffer_id,
        conv::map_index_format(format).expect("invalid index format"),
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetPipeline(
    bundle: native::WGPURenderBundleEncoder,
    pipeline: native::WGPURenderPipeline,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let pipeline_id = pipeline.as_ref().expect("invalid render pipeline").id;

    bundle_ffi::wgpu_render_bundle_set_pipeline(bundle, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetVertexBuffer(
    bundle: native::WGPURenderBundleEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let bundle = &mut bundle.as_mut().expect("invalid render bundle").encoder;
    let buffer_id = buffer.as_ref().expect("invalid buffer").id;

    bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
        bundle,
        slot,
        buffer_id,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}
