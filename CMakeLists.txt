cmake_minimum_required(VERSION 3.16)
# TODO: Get version of wgpu-native and set that up correctly here.
project(wgpu_native LANGUAGES C)

set(WGPU_NATIVE_USER_CARGO_BUILD_OPTIONS "" CACHE STRING "Additional cargo flags (such as --features) to apply to the build command")
option(BUILD_SHARED_LIBS "Build using shared libraries" OFF)
option(WGPU_NATIVE_ALWAYS_BUILD "If cmake should always invoke cargo to build wgpu_native" ON)

# TODO: What to do about RelWithDebInfo?
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(WGPU_NATIVE_BUILD_TYPE_FLAG "--release")
    set(WGPU_NATIVE_BUILD_TYPE "release")
else()
    set(WGPU_NATIVE_BUILD_TYPE "debug")
endif()

if(DEFINED ENV{CARGO_BUILD_TARGET})
    set(WGPU_NATIVE_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/$ENV{CARGO_BUILD_TARGET}/${WGPU_NATIVE_BUILD_TYPE})
else()
    set(WGPU_NATIVE_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/${WGPU_NATIVE_BUILD_TYPE})
endif()

if(ANDROID)
    if(ANDROID_ABI STREQUAL "armeabi-v7a")
        set(ANDROID_TARGET "armv7-linux-androideabi")
        set(ANDROID_ARCH_SHORT "arm")
    elseif(ANDROID_ABI STREQUAL "arm64-v8a")
        set(ANDROID_TARGET "aarch64-linux-android")
        set(ANDROID_ARCH_SHORT "aarch64")
    elseif(ANDROID_ABI STREQUAL "x86")
        set(ANDROID_TARGET "i686-linux-android")
        set(ANDROID_ARCH_SHORT "i386")
    elseif(ANDROID_ABI STREQUAL "x86_64")
        set(ANDROID_TARGET "x86_64-linux-android")
        set(ANDROID_ARCH_SHORT "x86_64")
    endif()

    set(WGPU_NATIVE_BUILD_TARGET "--target=${ANDROID_TARGET}")
    # TODO: Is this needed given CARGO_BUILD_TARGET above?
    set(WGPU_NATIVE_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/${ANDROID_TARGET}/${WGPU_NATIVE_BUILD_TYPE})

    if(BUILD_SHARED_LIBS)
        # TODO: Is this actually right?
        message(FATAL_ERROR "Wasmtime cannot be built with BUILD_SHARED_LIBS on Android")
    endif()
endif()

# TODO: Add mapping for iOS / etc builds to the right rust targets as well.

if(BUILD_SHARED_LIBS)
    if(WIN32)
        # TODO: Is this actually right?
        set(WGPU_NATIVE_BUILD_PRODUCT ${WGPU_NATIVE_LIB_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}wgpu_native.dll.lib)
    else()
        set(WGPU_NATIVE_BUILD_PRODUCT ${WGPU_NATIVE_LIB_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}wgpu_native${CMAKE_SHARED_LIBRARY_SUFFIX})
    endif()
else()
    set(WGPU_NATIVE_BUILD_PRODUCT ${WGPU_NATIVE_LIB_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}wgpu_native${CMAKE_STATIC_LIBRARY_SUFFIX})
endif()

if (WIN32)
    set(OS_LIBRARIES d3dcompiler ws2_32 userenv bcrypt ntdll opengl32)
elseif(UNIX AND NOT APPLE)
    set(OS_LIBRARIES "-lm -ldl")
elseif(APPLE)
    set(OS_LIBRARIES "-framework CoreFoundation -framework QuartzCore -framework Metal")
endif()

find_program(WGPU_NATIVE_CARGO_BINARY cargo)

# TODO: Use Corrosion instead?
include(ExternalProject)
ExternalProject_Add(
    wgpu_native-crate
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${WGPU_NATIVE_PREBUILD_COMMAND} ${WGPU_NATIVE_CARGO_BINARY} build ${WGPU_NATIVE_BUILD_TYPE_FLAG} ${WGPU_NATIVE_USER_CARGO_BUILD_OPTIONS} ${WGPU_NATIVE_BUILD_TARGET}
    INSTALL_COMMAND ""
    BUILD_ALWAYS ${WGPU_NATIVE_ALWAYS_BUILD}
    BUILD_BYPRODUCTS ${WGPU_NATIVE_BUILD_PRODUCT})

add_library(wgpu_native INTERFACE)
# TODO: How should we really name these? I don't know that the _native suffix adds any value for anyone.
add_library(wgpu::wgpu ALIAS wgpu_native)
add_dependencies(wgpu_native wgpu_native-crate)
target_link_libraries(wgpu_native INTERFACE ${WGPU_NATIVE_BUILD_PRODUCT} ${OS_LIBRARIES})

if(BUILD_SHARED_LIBS)
    target_compile_definitions(wgpu_native INTERFACE WGPU_SHARED_LIBRARY)
endif()

target_include_directories(wgpu_native INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/ffi>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/ffi/webgpu-headers>
    $<INSTALL_INTERFACE:include/webgpu>)

export(TARGETS wgpu_native NAMESPACE wgpu::
    FILE "${CMAKE_BINARY_DIR}/lib/cmake/wgpu_native/WgpuNativeTargets.cmake")

# TODO: install(EXPORTS ...)
include(GNUInstallDirs)
install(
    FILES
        ${CMAKE_CURRENT_SOURCE_DIR}/ffi/wgpu.h
        ${CMAKE_CURRENT_SOURCE_DIR}/ffi/webgpu-headers/webgpu.h
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/webgpu)
install(FILES ${WGPU_NATIVE_BUILD_PRODUCT}
    DESTINATION ${CMAKE_INSTALL_LIBDIR})
