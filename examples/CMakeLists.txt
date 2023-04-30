cmake_minimum_required(VERSION 3.20)
project(examples LANGUAGES C)

if(APPLE)
    add_compile_options(-x objective-c)
endif()

cmake_path(GET CMAKE_SOURCE_DIR PARENT_PATH PROJECT_ROOT_DIR)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(WGPU_NATIVE_LIB_TYPE debug)
else()
    set(WGPU_NATIVE_LIB_TYPE release)
endif()

if(DEFINED ENV{CARGO_BUILD_TARGET})
    set(WGPU_NATIVE_LIB_DIR ${PROJECT_ROOT_DIR}/target/$ENV{CARGO_BUILD_TARGET}/${WGPU_NATIVE_LIB_TYPE})
else()
    set(WGPU_NATIVE_LIB_DIR ${PROJECT_ROOT_DIR}/target/${WGPU_NATIVE_LIB_TYPE})
endif()

find_library(
    WGPU_LIBRARY NAMES libwgpu_native.a wgpu_native.lib wgpu_native
    HINTS ${WGPU_NATIVE_LIB_DIR}
    REQUIRED
)

add_subdirectory(framework)

add_subdirectory(capture)
add_subdirectory(compute)
add_subdirectory(triangle)

set(GLFW_USE_WAYLAND 1)
set(DEP_GLFW_DIR ${CMAKE_SOURCE_DIR}/vendor/glfw)
add_subdirectory(${DEP_GLFW_DIR})
