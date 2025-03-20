cmake_minimum_required(VERSION 3.20)
project(triangle LANGUAGES C)

add_executable(triangle main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

include_directories(${CMAKE_SOURCE_DIR}/../ffi)
include_directories(${CMAKE_SOURCE_DIR}/../ffi/webgpu-headers)
include_directories(${CMAKE_SOURCE_DIR}/framework)

if (WIN32)
    add_definitions(-DGLFW_EXPOSE_NATIVE_WIN32)
    set(OS_LIBRARIES d3dcompiler ws2_32 userenv bcrypt ntdll opengl32 Propsys RuntimeObject)
elseif(UNIX AND NOT APPLE)
    add_definitions(-DGLFW_EXPOSE_NATIVE_X11)
    add_definitions(-DGLFW_EXPOSE_NATIVE_WAYLAND)
    set(OS_LIBRARIES "-lm -ldl")
elseif(APPLE)
    add_definitions(-DGLFW_EXPOSE_NATIVE_COCOA)
    set(OS_LIBRARIES "-framework CoreFoundation -framework QuartzCore -framework Metal")
endif()

target_link_libraries(triangle framework glfw ${WGPU_LIBRARY} ${OS_LIBRARIES})
