#ifndef WGPU_H
#define WGPU_H

#include "wgpu.h"
#include "helper.h"

#endif

#include <framework.h>

#include <stdio.h>

void request_adapter_callback(WGPUAdapterId received, void *userdata) {
    *(WGPUAdapterId *) userdata = received;
}

void read_buffer_map(
        WGPUBufferMapAsyncStatus status,
        uint8_t *userdata) {
}

WGPULimits wgpu_limits_default() {
    return (WGPULimits) {
            .max_bind_groups = 4
    };
}

/*
 * No automatic defaults generated, have to generate the struct's defaults by hand.
 */
WGPUTextureViewDescriptor wgpu_texture_view_descriptor_default() {
    return (WGPUTextureViewDescriptor) {
            .label = NULL,
            .format = 90, // see https://github.com/gfx-rs/wgpu-native/issues/67
            .dimension = 6, // same as above
            .aspect = WGPUTextureAspect_All, // non-nullable, can use 0th element
            .array_layer_count = 0, // 0 = absent in NonZero ints
            .base_array_layer = 0,
            .base_mip_level = 0,
            .level_count = 0 // 0 = absent in NonZero ints
    };
}

int main(
        int argc,
        char *argv[]) {

    int width = 100;
    int height = 200;

    WGPUAdapterId adapter = {0};
    wgpu_request_adapter_async(
            &(WGPURequestAdapterOptions) {
                    .power_preference = WGPUPowerPreference_HighPerformance,
                    .compatible_surface = 0
            },
            2 | 4 | 8,
            request_adapter_callback,
            (void *) &adapter
    );

    WGPUDeviceId device = wgpuAdapterRequestDevice(
            adapter,
            &(WGPUDeviceDescriptor) {
                    .label = NULL,
                    0,
                    wgpu_limits_default(),
                    NULL}
    );

    BufferDimensions buffer_dimensions = buffer_dimensions_new(width, height);
    WGPUBufferId output_buffer = wgpu_device_create_buffer(
            device,
            &(WGPUBufferDescriptor) {
                    .label = NULL,
                    .size = buffer_dimensions.padded_bytes_per_row *
                            buffer_dimensions.height,
                    .usage = WGPUBufferUsage_MAP_READ |
                             WGPUBufferUsage_COPY_DST,
                    .mapped_at_creation = false}
    );

    WGPUExtent3d texture_extent = (WGPUExtent3d) {
            .width = buffer_dimensions.width,
            .height = buffer_dimensions.height,
            .depth = 1,
    };

    WGPUTextureId texture = wgpu_device_create_texture(device, &(WGPUTextureDescriptor) {
            .size = texture_extent,
            .mip_level_count = 1,
            .sample_count = 1,
            .dimension = WGPUTextureDimension_D2,
            .format = WGPUTextureFormat_Rgba8UnormSrgb,
            .usage = WGPUTextureUsage_RENDER_ATTACHMENT | WGPUTextureUsage_COPY_SRC,
            .label = NULL
    });

    WGPUCommandEncoderId encoder = wgpuDeviceCreateCommandEncoder(
            device, &(WGPUCommandEncoderDescriptor) {
                    .label = NULL,
            });

    WGPUTextureViewDescriptor texture_view_descriptor_default = wgpu_texture_view_descriptor_default();

    WGPURenderPass *render_pass = wgpu_command_encoder_begin_render_pass(encoder, &(WGPURenderPassDescriptor) {
            .label = NULL,
            .color_attachments = &(WGPUColorAttachmentDescriptor) {
                    .attachment = wgpu_texture_create_view(texture, &texture_view_descriptor_default),
                    .resolve_target = 0,
                    .channel = (WGPUPassChannel_Color) {
                            .clear_value = WGPUColor_RED,
                            .load_op = WGPULoadOp_Clear,
                            .store_op = WGPUStoreOp_Store,
                            .read_only = 0
                    }
            },
            .color_attachments_length = 1,
            .depth_stencil_attachment = NULL
    });

    wgpu_render_pass_end_pass(render_pass);

    wgpu_command_encoder_copy_texture_to_buffer(
            encoder,
            &(WGPUTextureCopyView) {
                    .texture = texture,
                    .mip_level = 0,
                    .origin = WGPUOrigin3d_ZERO
            },
            &(WGPUBufferCopyView) {
                    .buffer = output_buffer,
                    .layout = (WGPUTextureDataLayout) {
                            .offset = 0,
                            .bytes_per_row = buffer_dimensions.padded_bytes_per_row,
                            .rows_per_image = 0,
                    }
            },
            &texture_extent);

    WGPUCommandBufferId command_buffer = wgpu_command_encoder_finish(encoder, &(WGPUCommandBufferDescriptor) {
            .label = NULL
    });

    WGPUQueueId queue = wgpu_device_get_default_queue(device);

    wgpu_queue_submit(queue, &command_buffer, 1);

    wgpu_buffer_map_read_async(
            output_buffer,
            0,
            buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height,
            read_buffer_map,
            NULL);

    wgpu_device_poll(device, 1);

    uint8_t *ret = (uint8_t *) wgpu_buffer_get_mapped_range(
            output_buffer,
            0,
            buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height);

    const char *filename = "red.png";
    save_png(filename, ret, &buffer_dimensions);

    wgpu_buffer_unmap(output_buffer);

    return 0;
}
