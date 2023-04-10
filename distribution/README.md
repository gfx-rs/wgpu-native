WebGPU distribution
===================

This is pre-compiled distribution of [WebGPU native](https://github.com/webgpu-native/webgpu-headers) based on [`wgpu-native`](https://github.com/gfx-rs/wgpu-native) and ready to be used in a [CMake](https://cmake.org/) project.

Suggested usage
---------------

 1. Unzip the content of this archive into your source tree in a directory called `webgpu/`

 2. Edit you source tree's `CMakeLists.txt`:

```CMake
# [...] Define your application target, called for instance `App`

# Include this archive as a subdirectory
add_subdirectory(webgpu)

# You now have a target `webgpu` against which you can link your application:
target_link_library(App PRIVATE webgpu)

# The application's binary must find wgpu.dll or libwgpu.so at runtime,
# so we automatically copy it next to the binary.
target_copy_webgpu_binaries(App)

# (Alternatively you can use the ${WGPU_RUNTIME_LIB} variable to get the
# binary that must be copied and handle it yourselves.)
```

Notes
-----

### Preprocessor variable

In order to statically distinguish this distribution of WebGPU from other possible backends, it defines the following preprocessor variable:

```C
#define WEBGPU_BACKEND_WGPU
```

### Emscripten support

The CMakeLists provided with this distribution is designed to transparently work with emscripten's `emcmake`:

```bash
# Activate your emscripten installation
source /path/to/emsdk_env.sh
# Configure the project by prefixing with "emcmake"
emcmake cmake -B build-wasm
# Build the project as usual
cmake --build build-wasm
```

Reference
---------

When adding this distribution as a subdirectory in your CMake project, you get:

 - `webgpu`: A CMake target that provides the standard `webgpu.h` header and non-standard extensions `wgpu.h` as well as their implementation as a runtime library.
 - `${WGPU_RUNTIME_LIB}`: A CMake variable containing the path to the runtime library that your program needs to locate (.dll on Windows, .so on linux, .dylib on macOS).
 - `target_copy_webgpu_binaries(Target)`: A helper CMake function to automatically copy the runtime library next to the compiled application produced by target `Target`, so that it finds it out of the box.
 - `#define WEBGPU_BACKEND_WGPU`: A preprocessor variable defined in any target linking agains `webgpu` that can be used to detect that this distribution is based on a `wgpu-native` backend.

The `webgpu` target provides the following headers:

```C
#include <webgpu/webgpu.h> // standard webgpu-native header
#include <webgpu/wgpu.h> // non-standard extensions
```
