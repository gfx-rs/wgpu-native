cmake_minimum_required(VERSION 3.20)
project(compute LANGUAGES C)

add_executable(compute main.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

include_directories(${CMAKE_SOURCE_DIR}/framework)

target_link_libraries(compute framework ${WGPU_LIBRARY})
