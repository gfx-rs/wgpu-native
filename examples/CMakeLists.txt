cmake_minimum_required(VERSION 3.20)
project(examples LANGUAGES C)

set(CMAKE_C_STANDARD 99)

if(APPLE)
    add_compile_options(-x objective-c)
endif()

cmake_path(GET CMAKE_SOURCE_DIR PARENT_PATH PROJECT_ROOT_DIR)
include(FetchContent)
FetchContent_Declare(
    wgpu_native
    SOURCE_DIR ${PROJECT_ROOT_DIR}
)
FetchContent_MakeAvailable(wgpu_native)
set(WGPU_LIBRARY wgpu::wgpu)

add_subdirectory(framework)

add_subdirectory(capture)
add_subdirectory(compute)
add_subdirectory(enumerate_adapters)
add_subdirectory(texture_arrays)
add_subdirectory(triangle)

set(GLFW_BUILD_DOCS OFF)
set(GLFW_USE_WAYLAND ON)
add_subdirectory(vendor/glfw)
