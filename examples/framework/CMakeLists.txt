cmake_minimum_required(VERSION 3.20)
project(framework LANGUAGES C)

add_library(framework framework.c)

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

target_link_libraries(framework ${WGPU_LIBRARY})
