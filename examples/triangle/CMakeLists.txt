cmake_minimum_required(VERSION 3.20)
project(triangle LANGUAGES C)

add_executable(triangle main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

include_directories(${CMAKE_SOURCE_DIR}/../ffi)
include_directories(${CMAKE_SOURCE_DIR}/framework)
include_directories(${DEP_GLFW_DIR}/include)

if (WIN32)
    add_definitions(-DWGPU_TARGET_WINDOWS)
    set(OS_LIBRARIES d3dcompiler ws2_32 userenv bcrypt ntdll opengl32)
elseif(UNIX AND NOT APPLE)
    add_definitions(-DWGPU_TARGET_LINUX_X11)
    set(OS_LIBRARIES "-lm -ldl")
elseif(APPLE)
    add_definitions(-DWGPU_TARGET_MACOS)
    set(OS_LIBRARIES "-framework CoreFoundation -framework QuartzCore -framework Metal")
endif()

target_link_libraries(triangle framework glfw ${WGPU_LIBRARY} ${OS_LIBRARIES})
