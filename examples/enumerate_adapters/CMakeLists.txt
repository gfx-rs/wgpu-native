cmake_minimum_required(VERSION 3.20)
project(enumerate_adapters LANGUAGES C)

add_executable(enumerate_adapters main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

include_directories(${CMAKE_SOURCE_DIR}/../ffi)
include_directories(${CMAKE_SOURCE_DIR}/framework)

if (WIN32)
    set(OS_LIBRARIES d3dcompiler ws2_32 userenv bcrypt ntdll opengl32)
elseif(UNIX AND NOT APPLE)
    set(OS_LIBRARIES "-lm -ldl")
elseif(APPLE)
    set(OS_LIBRARIES "-framework CoreFoundation -framework QuartzCore -framework Metal")
endif()

target_link_libraries(enumerate_adapters framework ${WGPU_LIBRARY} ${OS_LIBRARIES})
