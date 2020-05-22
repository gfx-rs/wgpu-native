# wgpu-native
[![Matrix](https://img.shields.io/badge/Matrix-%23wgpu%3Amatrix.org-blueviolet.svg)](https://matrix.to/#/#wgpu:matrix.org)
[![Build Status](https://github.com/gfx-rs/wgpu-native/workflows/CI/badge.svg)](https://github.com/gfx-rs/wgpu-native/actions)
[![Crates.io](https://img.shields.io/crates/v/wgpu-native.svg?label=wgpu-native)](https://crates.io/crates/wgpu-native)

This is a native WebGPU implementation in Rust, based on [wgpu-core](https://github.com/gfx-rs/wgpu).
The C API header is generated at `ffi/wgpu.h` by [cbindgen](https://github.com/eqrion/cbindgen).

# Bindings

- [gfx-rs/wgpu-rs](https://github.com/gfx-rs/wgpu-rs) - idiomatic Rust wrapper with [a few more examples](https://github.com/gfx-rs/wgpu-rs/tree/master/examples) to get a feel of the API
- [almarklein/wgpu-py](https://github.com/almarklein/wgpu-py) - experimental Python wrapper
- [porky11/wgpu](https://nest.pijul.com/porky11/wgpu) - experimental [Scopes](http://scopes.rocks) wrapper
- [cshenton/WebGPU.jl](https://github.com/cshenton/WebGPU.jl) - experimental Julia wrapper

## Pre-built binaries

- Automated 32 and 64-bit builds for MacOS, Windows and Linux are available as Github [releases](https://github.com/gfx-rs/wgpu-native/releases).

## Usage

This repository contains C-language examples that link to the native library targets and perform basic rendering and computation. Please refer to our [Getting Started](https://github.com/gfx-rs/wgpu/wiki/Getting-Started#getting-started) page at the wiki for more information.
