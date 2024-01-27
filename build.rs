use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    cfg_aliases::cfg_aliases! {
        apple: { any(target_os = "ios", target_os = "macos") },
        unix_wo_apple: { all(unix, not(apple)) },

        dx12: { all(windows, feature = "dx12") },
        metal: { all(apple, feature = "metal") },
        vulkan: { any(windows, unix_wo_apple, feature = "vulkan-portability") },
        gles: { any(windows, unix_wo_apple, feature = "angle") },
    }

    println!("cargo:rerun-if-changed=ffi/webgpu-headers/webgpu.h");
    println!("cargo:rerun-if-changed=ffi/wgpu.h");

    #[rustfmt::skip]
    let types_to_rename = vec![
        ("WGPUAdapter", "WGPUAdapterImpl"),
        ("WGPUBindGroup", "WGPUBindGroupImpl"),
        ("WGPUBindGroupLayout", "WGPUBindGroupLayoutImpl"),
        ("WGPUBuffer", "WGPUBufferImpl"),
        ("WGPUCommandBuffer", "WGPUCommandBufferImpl"),
        ("WGPUCommandEncoder", "WGPUCommandEncoderImpl"),
        ("WGPUComputePassEncoder", "WGPUComputePassEncoderImpl"),
        ("WGPUComputePipeline", "WGPUComputePipelineImpl"),
        ("WGPUDevice", "WGPUDeviceImpl"),
        ("WGPUInstance", "WGPUInstanceImpl"),
        ("WGPUPipelineLayout", "WGPUPipelineLayoutImpl"),
        ("WGPUQuerySet", "WGPUQuerySetImpl"),
        ("WGPUQueue", "WGPUQueueImpl"),
        ("WGPURenderBundle", "WGPURenderBundleImpl"),
        ("WGPURenderBundleEncoder", "WGPURenderBundleEncoderImpl"),
        ("WGPURenderPassEncoder", "WGPURenderPassEncoderImpl"),
        ("WGPURenderPipeline", "WGPURenderPipelineImpl"),
        ("WGPUSampler", "WGPUSamplerImpl"),
        ("WGPUShaderModule", "WGPUShaderModuleImpl"),
        ("WGPUSurface", "WGPUSurfaceImpl"),
        ("WGPUTexture", "WGPUTextureImpl"),
        ("WGPUTextureView", "WGPUTextureViewImpl"),
    ];
    let mut builder = bindgen::Builder::default()
        .header("ffi/wgpu.h")
        .clang_arg("-Iffi/webgpu-headers")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("wgpuGetProcAddress")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .ignore_functions()
        .layout_tests(true);

    for (old_name, new_name) in types_to_rename {
        let line = format!("pub type {old_name} = *const crate::{new_name};");
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
