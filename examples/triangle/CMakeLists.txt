cmake_minimum_required(VERSION 3.11b)

project(triangle)

set(TARGET_NAME triangle)

add_executable(triangle main.c ../framework.c)

if(EMSCRIPTEN)
    add_definitions(-DWGPU_TARGET=WGPU_TARGET_LINUX_EMSCRIPTEN)
elseif(MSVC)
    add_definitions(-DWGPU_TARGET=WGPU_TARGET_WINDOWS)
    target_compile_options(${TARGET_NAME} PRIVATE /W4)
    set(OS_LIBRARIES "userenv" "ws2_32" "Dwmapi" "dbghelp" "d3dcompiler" "D3D12" "D3D11" "DXGI" "setupapi" "Bcrypt")
elseif(APPLE)
    add_definitions(-DWGPU_TARGET=WGPU_TARGET_MACOS)
    set(OS_LIBRARIES "-framework Cocoa" "-framework CoreVideo" "-framework IOKit" "-framework QuartzCore")
    target_compile_options(${TARGET_NAME} PRIVATE -x objective-c)
else(MSVC)
if(USE_WAYLAND)
    add_definitions(-DWGPU_TARGET=WGPU_TARGET_LINUX_WAYLAND)
else(USE_WAYLAND)
    add_definitions(-DWGPU_TARGET=WGPU_TARGET_LINUX_X11)
endif(USE_WAYLAND)
    target_compile_options(${TARGET_NAME} PRIVATE -Wall -Wextra -pedantic)
endif(MSVC)

if(EMSCRIPTEN)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} \
        -Oz -g -s USE_GLFW=3")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} \
        -Oz -g -s FULL_ES2=1 \
        -s ERROR_ON_UNDEFINED_SYMBOLS=0 \
        -s MIN_WEBGL_VERSION=2 \
        --preload-file \"${CMAKE_CURRENT_SOURCE_DIR}/shader.wgsl@shader.wgsl\"")
    set(WGPU_LIBRARY "wgpu_native")
    set_target_properties(${TARGET_NAME} PROPERTIES SUFFIX .html)
    target_link_directories(${TARGET_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/../../target/wasm32-unknown-emscripten/release")
else(EMSCRIPTEN)
    find_package(glfw3 3.3 REQUIRED
        HINTS "$ENV{GLFW3_INSTALL_DIR}"
    )
    find_library(WGPU_LIBRARY wgpu_native
        HINTS "${CMAKE_CURRENT_SOURCE_DIR}/../../target/debug"
    )
    target_include_directories(${TARGET_NAME} PUBLIC $ENV{GLFW3_INCLUDE_DIR})
endif(EMSCRIPTEN)

target_include_directories(${TARGET_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../../ffi)
target_include_directories(${TARGET_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../)

target_link_libraries(${TARGET_NAME} glfw ${WGPU_LIBRARY} ${OS_LIBRARIES})
