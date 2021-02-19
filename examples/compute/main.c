#ifndef WGPU_H
#define WGPU_H
#include "wgpu.h"
#endif

#include "framework.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BIND_ENTRIES_LENGTH (1)
#define BIND_GROUP_LAYOUTS_LENGTH (1)

void request_adapter_callback(WGPUAdapterId received, void *userdata) {
    *(WGPUAdapterId*)userdata = received;
}

void read_buffer_map(
    WGPUBufferMapAsyncStatus status,
    uint8_t *userdata) {
}

int main(
    int argc,
    char *argv[]) {

    if (argc != 5) {
        printf("You must pass 4 positive integers!\n");
        return 0;
    }

    uint32_t numbers[] = {
        strtoul(argv[1], NULL, 0),
        strtoul(argv[2], NULL, 0),
        strtoul(argv[3], NULL, 0),
        strtoul(argv[4], NULL, 0),
    };

    uint32_t size = sizeof(numbers);

    uint32_t numbers_length = size / sizeof(uint32_t);

    WGPUAdapterId adapter = { 0 };
    wgpu_request_adapter_async(
        NULL,
        2 | 4 | 8,
        request_adapter_callback,
        (void *) &adapter
    );

    WGPUDeviceId device = wgpu_adapter_request_device(adapter,
        &(WGPUDeviceDescriptor) {
            .label = "",
            0,
            (WGPULimits) {
                .max_bind_groups = 1
            },
            NULL}
        );

    WGPUBufferId staging_buffer = wgpu_device_create_buffer(device,
            &(WGPUBufferDescriptor){
                .label = "",
                .size = size,
                .usage = WGPUBufferUsage_MAP_READ | WGPUBufferUsage_COPY_DST,
                .mapped_at_creation = false}
            );

    WGPUBufferId storage_buffer = wgpu_device_create_buffer(device,
            &(WGPUBufferDescriptor){
                .label = "",
                .size = size,
                .usage = WGPUBufferUsage_STORAGE | WGPUBufferUsage_COPY_DST | WGPUBufferUsage_COPY_SRC,
                .mapped_at_creation = true}
            );

    uint8_t *storage_data = wgpu_buffer_get_mapped_range(storage_buffer, 0, size);

    memcpy((uint32_t *) storage_data, numbers, size);

    wgpu_buffer_unmap(storage_buffer);

    WGPUBindGroupLayoutId bind_group_layout =
        wgpu_device_create_bind_group_layout(device,
            &(WGPUBindGroupLayoutDescriptor){
                .label = "bind group layout",
                .entries = &(WGPUBindGroupLayoutEntry){
                    .binding = 0,
                    .visibility = WGPUShaderStage_COMPUTE,
                    .ty = WGPUBindingType_StorageBuffer},
                .entries_length = BIND_ENTRIES_LENGTH});

    WGPUBindGroupId bind_group = wgpu_device_create_bind_group(device,
            &(WGPUBindGroupDescriptor){
                .label = "bind group",
                .layout = bind_group_layout,
                .entries = &(WGPUBindGroupEntry){
                    .binding = 0,
                    .buffer = storage_buffer,
                    .offset = 0,
                    .size = size},
                .entries_length = BIND_ENTRIES_LENGTH});

    WGPUBindGroupLayoutId bind_group_layouts[BIND_GROUP_LAYOUTS_LENGTH] = {
        bind_group_layout};

    WGPUPipelineLayoutId pipeline_layout =
            wgpu_device_create_pipeline_layout(device,
                &(WGPUPipelineLayoutDescriptor){
                    .bind_group_layouts = bind_group_layouts,
                    .bind_group_layouts_length = BIND_GROUP_LAYOUTS_LENGTH});

    WGPUShaderModuleDescriptor source = read_file("./../data/collatz.comp.spv");
    WGPUShaderModuleId shader_module = wgpu_device_create_shader_module(
        device,
        &source);

    WGPUComputePipelineId compute_pipeline =
        wgpu_device_create_compute_pipeline(device,
            &(WGPUComputePipelineDescriptor){
                .layout = pipeline_layout,
                .stage = (WGPUProgrammableStageDescriptor){
                    .module = shader_module,
                    .entry_point = "main"
                }});

    WGPUCommandEncoderId encoder = wgpu_device_create_command_encoder(
        device, &(WGPUCommandEncoderDescriptor){
            .label = "command encoder",
        });

    WGPUComputePass* command_pass =
        wgpu_command_encoder_begin_compute_pass(encoder, &(WGPUComputePassDescriptor) { .label = "" });
    wgpu_compute_pass_set_pipeline(command_pass, compute_pipeline);

    wgpu_compute_pass_set_bind_group(command_pass, 0, bind_group, NULL, 0);
    wgpu_compute_pass_dispatch(command_pass, numbers_length, 1, 1);
    wgpu_compute_pass_end_pass(command_pass);

    WGPUQueueId queue = wgpu_device_get_default_queue(device);
    wgpu_command_encoder_copy_buffer_to_buffer(encoder, storage_buffer, 0, staging_buffer, 0, size);
    WGPUCommandBufferId command_buffer = wgpu_command_encoder_finish(encoder, NULL);
    wgpu_queue_submit(queue, &command_buffer, 1);

    wgpu_buffer_map_read_async(staging_buffer, 0, size, read_buffer_map, NULL);

    wgpu_device_poll(device, true);

    uint32_t *times = (uint32_t *) wgpu_buffer_get_mapped_range(staging_buffer, 0, size);

    printf("Times: [%d, %d, %d, %d]\n",
        times[0],
        times[1],
        times[2],
        times[3]);

    wgpu_buffer_unmap(staging_buffer);

    return 0;
}
