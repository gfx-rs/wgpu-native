use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=ffi/webgpu-headers/webgpu.h");
    println!("cargo:rerun-if-changed=ffi/wgpu.h");

    let types_to_rename = vec![
        ("WGPUInstance", "WGPUInstanceImpl"),
        ("WGPUAdapter", "WGPUAdapterImpl"),
        ("WGPUSurface", "WGPUSurfaceImpl"),
        ("WGPUDevice", "WGPUDeviceImpl"),
        ("WGPUQueue", "WGPUQueueImpl"),
        ("WGPUBuffer", "WGPUBufferImpl"),
        ("WGPUTextureView", "WGPUTextureViewImpl"),
        ("WGPUTexture", "WGPUTextureImpl"),
        ("WGPUSampler", "WGPUSamplerImpl"),
        ("WGPUBindGroupLayout", "WGPUBindGroupLayoutImpl"),
        ("WGPUPipelineLayout", "WGPUPipelineLayoutImpl"),
        ("WGPUBindGroup", "WGPUBindGroupImpl"),
        ("WGPUShaderModule", "WGPUShaderModuleImpl"),
        ("WGPURenderPipeline", "WGPURenderPipelineImpl"),
        ("WGPUComputePipeline", "WGPUComputePipelineImpl"),
        ("WGPUCommandEncoder", "WGPUCommandEncoderImpl"),
        ("WGPUCommandBuffer", "WGPUCommandBufferImpl"),
        ("WGPURenderPassEncoder", "WGPURenderPassEncoderImpl"),
        ("WGPUComputePassEncoder", "WGPUComputePassEncoderImpl"),
        ("WGPURenderBundleEncoder", "WGPURenderBundleEncoderImpl"),
        ("WGPURenderBundle", "WGPURenderBundleImpl"),
        ("WGPUQuerySet", "WGPUQuerySetImpl"),
        ("WGPUSwapChain", "WGPUSwapChainImpl"),
    ];
    let mut builder = bindgen::Builder::default()
        .header("ffi/webgpu-headers/webgpu.h")
        .header("ffi/wgpu.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type("(^WGPUProc).*")
        .blocklist_function("wgpuGetProcAddress")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .ignore_functions()
        .layout_tests(true);

    for (old_name, new_name) in types_to_rename {
        let line = format!("pub type {old_name} = *mut {new_name};");
        builder = builder
            .blocklist_type(old_name)
            .blocklist_type(format!("{old_name}Impl"))
            .raw_line(line);
    }

    // See https://github.com/rust-lang/rust-bindgen/issues/1780
    if let Ok("ios") = env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        let output = Command::new("xcrun")
            .args(["--sdk", "iphoneos", "--show-sdk-path"])
            .output()
            .expect("xcrun failed")
            .stdout;
        let sdk = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
        builder = builder
            .clang_arg(format!("-isysroot {sdk}"))
            .clang_arg("--target=arm64-apple-ios");
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
