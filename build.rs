use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=ffi/webgpu-headers/webgpu.h");
    println!("cargo:rerun-if-changed=ffi/wgpu.h");

    let types_to_rename = vec![
        ("WGPUAdapter", "AdapterId"),
        ("WGPUSurface", "SurfaceId"),
        ("WGPUSwapchain", "SwapChainId"),
        ("WGPUDevice", "DeviceId"),
        ("WGPUQueue", "QueueId"),
        ("WGPUBuffer", "BufferId"),
        ("WGPUTextureView", "TextureViewId"),
        ("WGPUTexture", "TextureId"),
        ("WGPUSampler", "SamplerId"),
        ("WGPUBindGroupLayout", "BindGroupLayoutId"),
        ("WGPUPipelineLayout", "PipelineLayoutId"),
        ("WGPUBindGroup", "BindGroupId"),
        ("WGPUShaderModule", "ShaderModuleId"),
        ("WGPURenderPipeline", "RenderPipelineId"),
        ("WGPUComputePipeline", "ComputePipelineId"),
        ("WGPUCommandEncoder", "CommandEncoderId"),
        ("WGPUCommandBuffer", "CommandBufferId"),
        ("WGPURenderPassEncoder", "RenderPassEncoderId"),
        ("WGPUComputePassEncoder", "ComputePassEncoderId"),
        ("WGPURenderBundleEncoder", "ComputePipelineId"),
        ("WGPURenderBundle", "RenderBundleId"),
        ("WGPUQuerySet", "QuerySetId"),
    ];
    let mut builder = bindgen::Builder::default()
        .header("ffi/webgpu-headers/webgpu.h")
        .header("ffi/wgpu.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blacklist_type("(^WGPUProc).*")
        .blacklist_function("wgpuGetProcAddress")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .ignore_functions()
        .layout_tests(true);

    for (old_name, new_name) in types_to_rename {
        builder = builder
            .blacklist_type(old_name)
            .blacklist_type(format!("{}Impl", old_name))
            .raw_line(format!("type {} = wgc::id::{};", old_name, new_name));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
