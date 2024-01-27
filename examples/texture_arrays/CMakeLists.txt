cmake_minimum_required(VERSION 3.20)
project(texture_arrays LANGUAGES C)

add_executable(texture_arrays main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

include_directories(${CMAKE_SOURCE_DIR}/framework)

if (WIN32)
    add_definitions(-DWGPU_TARGET_WINDOWS)
elseif(UNIX AND NOT APPLE)
    add_definitions(-DWGPU_TARGET_LINUX_X11)
elseif(APPLE)
    add_definitions(-DWGPU_TARGET_MACOS)
endif()

target_link_libraries(texture_arrays framework glfw ${WGPU_LIBRARY})
