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
        ("WGPURenderBundleEncoder", "ComputePipelineId"),
        ("WGPURenderBundle", "RenderBundleId"),
        ("WGPUQuerySet", "QuerySetId"),
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
        builder = builder
            .blocklist_type(old_name)
            .blocklist_type(format!("{}Impl", old_name))
            .raw_line(format!("type {} = wgc::id::{};", old_name, new_name));
    }

    // WGPUBindGroupEntry.{buffer, sampler, textureView} is nullable
    builder = builder.blocklist_item("WGPUBindGroupEntry").raw_line(
        "#[repr(C)]
            pub struct WGPUBindGroupEntry {
                pub nextInChain: *const WGPUChainedStruct,
                pub binding: u32,
                pub buffer: Option<WGPUBuffer>,
                pub offset: u64,
                pub size: u64,
                pub sampler: Option<WGPUSampler>,
                pub textureView: Option<WGPUTextureView>,
            }",
    );

    // WGPURequestAdapterOptions.compatibleSurface is nullable
    builder = builder
        .blocklist_item("WGPURequestAdapterOptions")
        .raw_line(
            "#[repr(C)]
            pub struct WGPURequestAdapterOptions {
                pub nextInChain: *const WGPUChainedStruct,
                pub compatibleSurface: Option<WGPUSurface>,
                pub powerPreference: WGPUPowerPreference,
                pub forceFallbackAdapter: bool,
            }",
        );

    // WGPURenderPassColorAttachment.{view, resolveTarget} is nullable
    builder = builder
        .blocklist_item("WGPURenderPassColorAttachment")
        .raw_line(
            "#[repr(C)]
            pub struct WGPURenderPassColorAttachment {
                pub view: Option<WGPUTextureView>,
                pub resolveTarget: Option<WGPUTextureView>,
                pub loadOp: WGPULoadOp,
                pub storeOp: WGPUStoreOp,
                pub clearValue: WGPUColor,
            }",
        );

    // WGPUComputePipelineDescriptor.layout is nullable
    builder = builder
        .blocklist_item("WGPUComputePipelineDescriptor")
        .raw_line(
            "#[repr(C)]
            pub struct WGPUComputePipelineDescriptor {
                pub nextInChain: *const WGPUChainedStruct,
                pub label: *const ::std::os::raw::c_char,
                pub layout: Option<WGPUPipelineLayout>,
                pub compute: WGPUProgrammableStageDescriptor,
            }",
        );

    // WGPURenderPipelineDescriptor.layout is nullable
    builder = builder
        .blocklist_item("WGPURenderPipelineDescriptor")
        .raw_line(
            "#[repr(C)]
            pub struct WGPURenderPipelineDescriptor {
                pub nextInChain: *const WGPUChainedStruct,
                pub label: *const ::std::os::raw::c_char,
                pub layout: Option<WGPUPipelineLayout>,
                pub vertex: WGPUVertexState,
                pub primitive: WGPUPrimitiveState,
                pub depthStencil: *const WGPUDepthStencilState,
                pub multisample: WGPUMultisampleState,
                pub fragment: *const WGPUFragmentState,
            }",
        );

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
