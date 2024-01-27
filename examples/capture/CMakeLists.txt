cmake_minimum_required(VERSION 3.20)
project(capture LANGUAGES C)

add_executable(capture main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

add_definitions(-DSTB_IMAGE_WRITE_IMPLEMENTATION)

include_directories(${CMAKE_SOURCE_DIR}/framework)

target_link_libraries(capture framework ${WGPU_LIBRARY})
