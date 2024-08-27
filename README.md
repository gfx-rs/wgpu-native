# wgpu-native
[![Matrix](https://img.shields.io/badge/Matrix-%23wgpu%3Amatrix.org-blueviolet.svg)](https://matrix.to/#/#wgpu:matrix.org)
[![Build Status](https://github.com/gfx-rs/wgpu-native/workflows/CI/badge.svg)](https://github.com/gfx-rs/wgpu-native/actions)

This is a native WebGPU implementation in Rust, based on [wgpu-core](https://github.com/gfx-rs/wgpu).
The bindings are based on the WebGPU-native header found at `ffi/webgpu-headers/webgpu.h` and wgpu-native specific items in `ffi/wgpu.h`

# Bindings

- [gfx-rs/wgpu-rs](https://github.com/gfx-rs/wgpu/tree/master/wgpu) - idiomatic Rust wrapper with [a few more examples](https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples) to get a feel of the API
- [pygfx/wgpu-py](https://github.com/pygfx/wgpu-py) - Python wrapper
- [trivaxy/wgpu.NET](https://github.com/trivaxy/WGPU.NET) - Raw .NET bindings with optional wrappers
- [dotnet/Silk.NET](https://github.com/dotnet/Silk.NET) - Raw .NET bindings
- [Alimer.Bindings.WebGPU](https://github.com/amerkoleci/Alimer.Bindings.WebGPU) - Cross platform .NET bindings for WebGPU
- [wgpu.cr](https://github.com/chances/wgpu-crystal) - Crystal wrapper
- [bindc-wgpu](https://github.com/gecko0307/bindbc-wgpu) - D wrapper ([package](https://code.dlang.org/packages/bindbc-wgpu))
- [porky11/wgpu](https://gitlab.com/scopes-libraries/wgpu) - experimental [Scopes](http://scopes.rocks) wrapper
- [cshenton/WebGPU.jl](https://github.com/cshenton/WebGPU.jl) - experimental Julia wrapper
- [dvijaha/WGPUNative.jl](https://github.com/dvijaha/WGPUNative.jl) - stable Julia wrapper
- [kgpu/wgpuj](https://github.com/kgpu/kgpu/tree/master/wgpuj) - Java/Kotlin wrapper
- [wgpu4k/wgpu4k](https://github.com/wgpu4k/wgpu4k) - WIP Kotlin Multi Platform wrapper
- [rajveermalviya/go-webgpu](https://github.com/rajveermalviya/go-webgpu) - Go wrapper
- [WebGPU-C++](https://github.com/eliemichel/WebGPU-Cpp) - Auto-generated C++ wrapper (developed for the [Learn WebGPU native](https://eliemichel.github.io/LearnWebGPU) course)
- [jai_wgpu_native](https://github.com/SogoCZE/jai_wgpu_native) - Raw Jai bindings
- [WebGPU::Direct](https://github.com/atrodo/WebGPU-Direct) - Perl wrapper ([package](https://metacpan.org/pod/WebGPU::Direct))

## Pre-built binaries

Automated 32 and 64-bit builds for MacOS, Windows and Linux are available as Github [releases](https://github.com/gfx-rs/wgpu-native/releases). Details can be found in the  [Binary Releases](https://github.com/gfx-rs/wgpu-native/wiki/Binary-Releases) page in the wiki.

## Usage

This repository contains C-language examples that link to the native library targets and perform basic rendering and computation. Please refer to our [Getting Started](https://github.com/gfx-rs/wgpu-native/wiki/Getting-Started) page at the wiki for more information.

There's also a (small) [contributor guide](https://github.com/gfx-rs/wgpu-native/wiki/Contributing).
