use crate::native::{Handle, IntoHandle, IntoHandleWithContext, UnwrapId};
use crate::{conv, handle_device_error, make_slice, native, Context, OwnedLabel};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use std::{borrow::Cow, num::NonZeroU64};
use wgc::command::{ComputePass, RenderBundleEncoder, RenderPass};
use wgc::{
    command::{bundle_ffi, compute_ffi, render_ffi},
    gfx_select,
};

fn unwrap_render_bundle_encoder<'a>(
    handle: *mut native::WGPURenderBundleEncoderImpl,
) -> (&'a mut RenderBundleEncoder, &'a Arc<Context>) {
    unsafe {
        let v = handle.as_mut().expect("invalid render bundle encoder");
        (&mut v.encoder, &v.context)
    }
}

fn unwrap_compute_pass_encoder<'a>(
    handle: *mut native::WGPUComputePassEncoderImpl,
) -> (&'a mut ComputePass, &'a Arc<Context>) {
    unsafe {
        let v = handle.as_mut().expect("invalid compute pass encoder");
        (&mut v.encoder, &v.context)
    }
}

fn unwrap_render_pass_encoder<'a>(
    handle: *mut native::WGPURenderPassEncoderImpl,
) -> (&'a mut RenderPass, &'a Arc<Context>) {
    unsafe {
        let v = handle.as_mut().expect("invalid render pass encoder");
        (&mut v.encoder, &v.context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    command_encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPUCommandBufferDescriptor>,
) -> native::WGPUCommandBuffer {
    // NOTE: Automatically drop the encoder
    let (command_encoder, context) = {
        if command_encoder.is_null() {
            panic!("invalid render command encoder");
        }
        let v = Box::from_raw(command_encoder);
        (v.id, v.context)
    };

    let desc = match descriptor {
        Some(descriptor) => wgt::CommandBufferDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::CommandBufferDescriptor::default(),
    };

    let (id, error) =
        gfx_select!(command_encoder => context.command_encoder_finish(command_encoder, &desc));
    if let Some(error) = error {
        // TODO figure out what device the encoder belongs to and call
        // handle_device_error()
        log::error!("command_encoder_finish() failed: {:?}", error);
        std::ptr::null_mut()
    } else {
        id.into_handle_with_context(&context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderClearBuffer(
    command_encoder: native::WGPUCommandEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let (command_encoder, _) = command_encoder.unwrap_handle();
    let (buffer, context) = buffer.unwrap_handle();

    gfx_select!(command_encoder => context.command_encoder_clear_buffer(
        command_encoder,
        buffer,
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
    let (command_encoder, _) = command_encoder.unwrap_handle();
    let (source, _) = source.unwrap_handle();
    let (destination, context) = destination.unwrap_handle();

    gfx_select!(command_encoder => context.command_encoder_copy_buffer_to_buffer(
        command_encoder,
        source,
        source_offset,
        destination,
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
    let (command_encoder, context) = command_encoder.unwrap_handle();

    gfx_select!(command_encoder => context.command_encoder_copy_texture_to_texture(
        command_encoder,
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
    let (command_encoder, context) = command_encoder.unwrap_handle();

    gfx_select!(command_encoder => context.command_encoder_copy_texture_to_buffer(
        command_encoder,
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
    let (command_encoder, context) = command_encoder.unwrap_handle();

    gfx_select!(command_encoder => context.command_encoder_copy_buffer_to_texture(
        command_encoder,
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
    let (command_encoder, context) = command_encoder.unwrap_handle();

    let desc = match descriptor {
        Some(descriptor) => wgc::command::ComputePassDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgc::command::ComputePassDescriptor::default(),
    };
    let pass = wgc::command::ComputePass::new(command_encoder, &desc);
    native::WGPUComputePassEncoderImpl {
        context: context.clone(),
        encoder: pass,
    }
    .into_handle()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    encoder: native::WGPUCommandEncoder,
    descriptor: Option<&native::WGPURenderPassDescriptor>,
) -> native::WGPURenderPassEncoder {
    let (encoder, context) = encoder.unwrap_handle();
    let descriptor = descriptor.expect("invalid descriptor");

    let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|desc| {
        wgc::command::RenderPassDepthStencilAttachment {
            view: desc
                .view
                .as_option()
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
                color_attachment.view.as_option().map(|view| {
                    wgc::command::RenderPassColorAttachment {
                        view,
                        resolve_target: color_attachment.resolveTarget.as_option(),
                        channel: wgc::command::PassChannel {
                            load_op: conv::map_load_op(color_attachment.loadOp),
                            store_op: conv::map_store_op(color_attachment.storeOp),
                            clear_value: conv::map_color(&color_attachment.clearValue),
                            read_only: false,
                        },
                    }
                })
            })
            .collect(),
        ),
        depth_stencil_attachment: depth_stencil_attachment.as_ref(),
    };
    let pass = wgc::command::RenderPass::new(encoder, &desc);
    native::WGPURenderPassEncoderImpl {
        context: context.clone(),
        encoder: pass,
    }
    .into_handle()
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderInsertDebugMarker(
    encoder: native::WGPUCommandEncoder,
    marker_label: *const c_char,
) {
    let (encoder, context) = encoder.unwrap_handle();

    gfx_select!(encoder => context.command_encoder_insert_debug_marker(encoder, CStr::from_ptr(marker_label).to_str().unwrap()))
        .expect("Unable to insert debug marker");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPopDebugGroup(encoder: native::WGPUCommandEncoder) {
    let (encoder, context) = encoder.unwrap_handle();

    gfx_select!(encoder => context.command_encoder_pop_debug_group(encoder))
        .expect("Unable to pop debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuCommandEncoderPushDebugGroup(
    encoder: native::WGPUCommandEncoder,
    group_label: *const c_char,
) {
    let (encoder, context) = encoder.unwrap_handle();

    gfx_select!(encoder => context.command_encoder_push_debug_group(encoder, CStr::from_ptr(group_label).to_str().unwrap()))
        .expect("Unable to push debug group");
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass_handle: native::WGPUComputePassEncoder) {
    let (pass, context) = unwrap_compute_pass_encoder(pass_handle);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => context.command_encoder_run_compute_pass(encoder_id, pass))
        .expect("Unable to end compute pass");

    // NOTE: Automatically drops the encoder
    pass_handle.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass_handle: native::WGPURenderPassEncoder) {
    let (pass, context) = unwrap_render_pass_encoder(pass_handle);
    let encoder_id = pass.parent_id();
    gfx_select!(encoder_id => context.command_encoder_run_render_pass(encoder_id, pass))
        .expect("Unable to end render pass");

    // NOTE: Automatically drops the encoder
    pass_handle.drop();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: native::WGPUComputePassEncoder,
    pipeline_id: native::WGPUComputePipeline,
) {
    let (pass, _) = unwrap_compute_pass_encoder(pass);
    let (pipeline_id, _) = pipeline_id.unwrap_handle();

    compute_ffi::wgpu_compute_pass_set_pipeline(pass, pipeline_id);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: native::WGPURenderPassEncoder,
    pipeline_id: native::WGPURenderPipeline,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (pipeline_id, _) = pipeline_id.unwrap_handle();
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
    let (pass, _) = unwrap_compute_pass_encoder(pass);
    let (group, _) = group.unwrap_handle();

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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (group, _) = group.unwrap_handle();

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
    let (pass, _) = unwrap_compute_pass_encoder(pass);
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
    let (pass, _) = unwrap_compute_pass_encoder(pass);
    let (indirect_buffer, _) = indirect_buffer.unwrap_handle();

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
    let (pass, _) = unwrap_compute_pass_encoder(pass);
    compute_ffi::wgpu_compute_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPopDebugGroup(pass: native::WGPUComputePassEncoder) {
    let (pass, _) = unwrap_compute_pass_encoder(pass);
    compute_ffi::wgpu_compute_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    pass: native::WGPUComputePassEncoder,
    group_label: *const c_char,
) {
    let (pass, _) = unwrap_compute_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_draw_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_draw_indexed_indirect(pass, buffer, indirect_offset);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_multi_draw_indirect(pass, buffer, offset, count);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderMultiDrawIndexedIndirect(
    pass: native::WGPURenderPassEncoder,
    buffer: native::WGPUBuffer,
    offset: u64,
    count: u32,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect(pass, buffer, offset, count);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();
    let (count_buffer, _) = count_buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_multi_draw_indirect_count(
        pass,
        buffer,
        offset,
        count_buffer,
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();
    let (count_buffer, _) = count_buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_multi_draw_indexed_indirect_count(
        pass,
        buffer,
        offset,
        count_buffer,
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    pass.set_index_buffer(
        buffer,
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    let (buffer, _) = buffer.unwrap_handle();

    render_ffi::wgpu_render_pass_set_vertex_buffer(
        pass,
        slot,
        buffer,
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    render_ffi::wgpu_render_pass_set_push_constants(
        pass,
        wgt::ShaderStages::from_bits(stages).expect("Invalid shader stage"),
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
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
    let (pass, _) = unwrap_render_pass_encoder(pass);
    render_ffi::wgpu_render_pass_set_scissor_rect(pass, x, y, w, h);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    pass: native::WGPURenderPassEncoder,
    marker_label: *const c_char,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    render_ffi::wgpu_render_pass_insert_debug_marker(pass, marker_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPopDebugGroup(pass: native::WGPURenderPassEncoder) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    render_ffi::wgpu_render_pass_pop_debug_group(pass);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    pass: native::WGPURenderPassEncoder,
    group_label: *const c_char,
) {
    let (pass, _) = unwrap_render_pass_encoder(pass);
    render_ffi::wgpu_render_pass_push_debug_group(pass, group_label, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderPassEncoderExecuteBundles(
    render_pass_encoder: native::WGPURenderPassEncoder,
    bundles_count: u32,
    bundles: *const wgc::id::RenderBundleId,
) {
    let (render_pass_encoder, _) = unwrap_render_pass_encoder(render_pass_encoder);

    render_ffi::wgpu_render_pass_execute_bundles(
        render_pass_encoder,
        bundles,
        bundles_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDraw(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    bundle_ffi::wgpu_render_bundle_draw(
        render_bundle_encoder,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexed(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    bundle_ffi::wgpu_render_bundle_draw_indexed(
        render_bundle_encoder,
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndexedIndirect(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (indirect_buffer, _) = indirect_buffer.unwrap_handle();
    bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
        render_bundle_encoder,
        indirect_buffer,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderDrawIndirect(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    indirect_buffer: native::WGPUBuffer,
    indirect_offset: u64,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (indirect_buffer, _) = indirect_buffer.unwrap_handle();
    bundle_ffi::wgpu_render_bundle_draw_indirect(
        render_bundle_encoder,
        indirect_buffer,
        indirect_offset,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderFinish(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    descriptor: Option<&native::WGPURenderBundleDescriptor>,
) -> native::WGPURenderBundle {
    // NOTE: Automatically drop the encoder
    let (render_bundle_encoder, context) = {
        if render_bundle_encoder.is_null() {
            panic!("invalid render bundle encoder");
        }
        let v = Box::from_raw(render_bundle_encoder);
        (v.encoder, v.context)
    };

    let device = render_bundle_encoder.parent();

    let desc = match descriptor {
        Some(descriptor) => wgt::RenderBundleDescriptor {
            label: OwnedLabel::new(descriptor.label).into_cow(),
        },
        None => wgt::RenderBundleDescriptor::default(),
    };

    let (render_bundle, error) = gfx_select!(device => context.render_bundle_encoder_finish(render_bundle_encoder, &desc, ()));
    if let Some(error) = error {
        handle_device_error(device, &error);
        std::ptr::null_mut()
    } else {
        render_bundle.into_handle_with_context(&context)
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderInsertDebugMarker(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    marker_label: *const c_char,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    bundle_ffi::wgpu_render_bundle_insert_debug_marker(render_bundle_encoder, marker_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPopDebugGroup(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    bundle_ffi::wgpu_render_bundle_pop_debug_group(render_bundle_encoder);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderPushDebugGroup(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    group_label: *const c_char,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    bundle_ffi::wgpu_render_bundle_push_debug_group(render_bundle_encoder, group_label);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetBindGroup(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    group_index: u32,
    group: native::WGPUBindGroup,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (group, _) = group.unwrap_handle();
    bundle_ffi::wgpu_render_bundle_set_bind_group(
        render_bundle_encoder,
        group_index,
        group,
        dynamic_offsets,
        dynamic_offset_count as usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetIndexBuffer(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    buffer: native::WGPUBuffer,
    format: native::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (buffer, _) = buffer.unwrap_handle();

    bundle_ffi::wgpu_render_bundle_set_index_buffer(
        render_bundle_encoder,
        buffer,
        conv::map_index_format(format).unwrap(),
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
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    pipeline: native::WGPURenderPipeline,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (pipeline, _) = pipeline.unwrap_handle();

    bundle_ffi::wgpu_render_bundle_set_pipeline(render_bundle_encoder, pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn wgpuRenderBundleEncoderSetVertexBuffer(
    render_bundle_encoder: native::WGPURenderBundleEncoder,
    slot: u32,
    buffer: native::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let (render_bundle_encoder, _) = unwrap_render_bundle_encoder(render_bundle_encoder);
    let (buffer, _) = buffer.unwrap_handle();

    bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
        render_bundle_encoder,
        slot,
        buffer,
        offset,
        match size {
            0 => panic!("invalid size"),
            conv::WGPU_WHOLE_SIZE => None,
            _ => Some(NonZeroU64::new_unchecked(size)),
        },
    );
}
