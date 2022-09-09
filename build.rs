use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Setup cfg aliases
    cfg_aliases::cfg_aliases! {
        // Vendors/systems
        wasm: { target_arch = "wasm32" },
        apple: { any(target_os = "ios", target_os = "macos") },
        unix_wo_apple: {all(unix, not(apple))},

        // Backends
        vulkan: { all(not(wasm), any(windows, unix_wo_apple, feature = "vulkan-portability")) },
        metal: { all(not(wasm), apple) },
        dx12: { all(not(wasm), windows) },
        dx11: { all(not(wasm), windows) },
        gl: {
            any(
                unix_wo_apple,
                feature = "angle",
                wasm
            )
        },
    }

    println!("cargo:rerun-if-changed=ffi/webgpu-headers/webgpu.h");
    println!("cargo:rerun-if-changed=ffi/wgpu.h");

    let types_to_rename = vec![
        ("WGPUAdapter", "AdapterId"),
        ("WGPUSurface", "SurfaceId"),
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
        ("WGPURenderBundleEncoder", "RenderBundleEncoderId"),
        ("WGPURenderBundle", "RenderBundleId"),
        ("WGPUQuerySet", "QuerySetId"),
        ("WGPUSwapChain", "SurfaceId"),
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
        let line = match new_name {
            // wrapping raw pointer types in Option isn't ffi safe
            "ComputePassEncoderId" | "RenderPassEncoderId" | "RenderBundleEncoderId" => {
                format!("pub type {} = wgc::id::{};", old_name, new_name)
            }

            _ => format!("pub type {} = Option<wgc::id::{}>;", old_name, new_name),
        };

        builder = builder
            .blocklist_type(old_name)
            .blocklist_type(format!("{}Impl", old_name))
            .raw_line(line);
    }

    // See https://github.com/rust-lang/rust-bindgen/issues/923
    builder = builder
        .blocklist_item("WGPU_LIMIT_U64_UNDEFINED")
        .raw_line("pub const WGPU_LIMIT_U64_UNDEFINED: u64 = 0xffffffffffffffff;");
    builder = builder
        .blocklist_item("WGPU_WHOLE_MAP_SIZE")
        .raw_line("pub const WGPU_WHOLE_MAP_SIZE: usize = usize::MAX;");
    builder = builder
        .blocklist_item("WGPU_WHOLE_SIZE")
        .raw_line("pub const WGPU_WHOLE_SIZE: usize = 0xffffffffffffffff;");

    // See https://github.com/rust-lang/rust-bindgen/issues/1780
    if let Ok("ios") = env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        let output = Command::new("xcrun")
            .args(&["--sdk", "iphoneos", "--show-sdk-path"])
            .output()
            .expect("xcrun failed")
            .stdout;
        let sdk = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
        builder = builder
            .clang_arg(format!("-isysroot {}", sdk))
            .clang_arg("--target=arm64-apple-ios");
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
