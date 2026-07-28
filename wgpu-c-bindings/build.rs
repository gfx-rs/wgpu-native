use std::env;
use std::path::PathBuf;
use std::process::Command;

fn sdk_path(sdk_name: &str) -> String {
    let output = Command::new("xcrun")
        .args(["--sdk", sdk_name, "--show-sdk-path"])
        .output()
        .expect("xcrun failed")
        .stdout;
    std::str::from_utf8(&output)
        .expect("invalid output from `xcrun`")
        .trim()
        .to_owned()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let ffi_dir = manifest_dir.join("../ffi");

    let webgpu_h = ffi_dir.join("webgpu-headers/webgpu.h");
    let wgpu_h = ffi_dir.join("wgpu.h");

    println!("cargo:rerun-if-changed={}", webgpu_h.display());
    println!("cargo:rerun-if-changed={}", wgpu_h.display());
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");

    let mut builder = bindgen::Builder::default()
        .header(wgpu_h.to_str().unwrap())
        .clang_arg(format!("-I{}", ffi_dir.join("webgpu-headers").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item("WGPU.*")
        .allowlist_item("wgpu.*")
        .blocklist_function("wgpuGetProcAddress")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .layout_tests(true)
        .clang_macro_fallback();

    if let Ok(target) = env::var("TARGET") {
        match target.as_str() {
            "aarch64-apple-ios" => {
                builder = builder
                    .clang_arg("-isysroot")
                    .clang_arg(sdk_path("iphoneos"))
                    .clang_arg("--target=arm64-apple-ios");
            }
            "aarch64-apple-ios-sim" => {
                builder = builder
                    .clang_arg("-isysroot")
                    .clang_arg(sdk_path("iphonesimulator"))
                    .clang_arg("--target=arm64-apple-ios-simulator");
            }
            "x86_64-apple-ios" => {
                builder = builder
                    .clang_arg("-isysroot")
                    .clang_arg(sdk_path("iphonesimulator"))
                    .clang_arg("--target=x86_64-apple-ios-simulator");
            }
            "aarch64-apple-darwin" | "x86_64-apple-darwin" => {
                builder = builder.clang_arg("-isysroot").clang_arg(sdk_path("macosx"));
            }
            _ => {}
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
