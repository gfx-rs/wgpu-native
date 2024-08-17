WILDCARD_SOURCE:=$(wildcard src/*.rs)

GIT_TAG=$(shell git describe --abbrev=0 --tags)
GIT_TAG_FULL=$(shell git describe --tags)
OS_NAME=

EXTRA_BUILD_ARGS=
TARGET_DIR=target
ifdef TARGET
	EXTRA_BUILD_ARGS=--target $(TARGET)
	TARGET_DIR=target/$(TARGET)
endif

ifndef ARCHIVE_NAME
	ARCHIVE_NAME=wgpu-$(TARGET)
endif

ifeq ($(OS),Windows_NT)
	OS_NAME=windows
else
	UNAME_S:=$(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		OS_NAME=linux
	endif
	ifeq ($(UNAME_S),Darwin)
		OS_NAME=macos
	endif
endif

MKDIR_CMD:=mkdir -p

ifeq ($(origin MSYSTEM),undefined) # MSYSTEM env var is defined on msys2
	ifeq ($(OS),Windows_NT)
		SHELL:=cmd
		MKDIR_CMD=powershell -Command md -Force
	endif
endif

.PHONY: check test doc clear \
	lib-native lib-native-release \
	example-capture example-compute example-triangle \
	example-capture-release example-compute-release example-triangle-release \
	run-example-capture run-example-compute run-example-triangle \
	run-example-capture-release run-example-compute-release run-example-triangle-release

package: lib-native lib-native-release
	mkdir -p dist
	echo "$(GIT_TAG_FULL)" > dist/commit-sha
	for RELEASE in debug release; do \
		ARCHIVE=$(ARCHIVE_NAME)-$$RELEASE.zip; \
		LIBDIR=$(TARGET_DIR)/$$RELEASE; \
		rm -f dist/$$ARCHIVE; \
		if [ $(OS_NAME) = windows ]; then \
			if [[ "$(TARGET)" == *"gnu"* ]]; then \
				7z a -tzip dist/$$ARCHIVE ./$$LIBDIR/wgpu_native.dll ./$$LIBDIR/libwgpu_native.dll.a ./$$LIBDIR/libwgpu_native.a ./ffi/webgpu-headers/*.h ./ffi/wgpu.h ./dist/commit-sha; \
			else \
				7z a -tzip dist/$$ARCHIVE ./$$LIBDIR/wgpu_native.dll ./$$LIBDIR/wgpu_native.dll.lib ./$$LIBDIR/wgpu_native.pdb ./$$LIBDIR/wgpu_native.lib ./ffi/webgpu-headers/*.h ./ffi/wgpu.h ./dist/commit-sha; \
			fi; \
		else \
			zip -j dist/$$ARCHIVE ./$$LIBDIR/libwgpu_native.so ./$$LIBDIR/libwgpu_native.dylib ./$$LIBDIR/libwgpu_native.a ./ffi/webgpu-headers/*.h ./ffi/wgpu.h ./dist/commit-sha; \
		fi; \
	done

clean:
	cargo clean
	rm -Rf examples/build

check:
	cargo check --all

test:
	cargo test --all

doc:
	cargo doc --all

clear:
	cargo clean

lib-native: Cargo.lock Cargo.toml Makefile $(WILDCARD_SOURCE)
	cargo build $(EXTRA_BUILD_ARGS)

lib-native-release: Cargo.lock Cargo.toml Makefile $(WILDCARD_SOURCE)
	cargo build --release $(EXTRA_BUILD_ARGS)

examples-debug: lib-native
	cd examples && $(MKDIR_CMD) "build/Debug" && cd build/Debug && cmake -GNinja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ../..

examples-release: lib-native-release
	cd examples && $(MKDIR_CMD) "build/RelWithDebInfo" && cd build/RelWithDebInfo && cmake -GNinja -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ../..

example-capture: examples-debug
	cd examples/build/Debug && cmake --build . --target capture

run-example-capture: example-capture
	cd examples/capture && "../build/Debug/capture/capture"

example-capture-release: examples-release
	cd examples/build/RelWithDebInfo && cmake --build . --target capture

run-example-capture-release: example-capture-release
	cd examples/capture && "../build/RelWithDebInfo/capture/capture"

example-compute: examples-debug
	cd examples/build/Debug && cmake --build . --target compute

run-example-compute: example-compute
	cd examples/compute && "../build/Debug/compute/compute"

example-compute-release: examples-release
	cd examples/build/RelWithDebInfo && cmake --build . --target compute

run-example-compute-release: example-compute-release
	cd examples/compute && "../build/RelWithDebInfo/compute/compute"

example-enumerate_adapters: examples-debug
	cd examples/build/Debug && cmake --build . --target enumerate_adapters

run-example-enumerate_adapters: example-enumerate_adapters
	cd examples/enumerate_adapters && "../build/Debug/enumerate_adapters/enumerate_adapters"

example-enumerate_adapters-release: examples-release
	cd examples/build/RelWithDebInfo && cmake --build . --target enumerate_adapters

run-example-enumerate_adapters-release: example-enumerate_adapters-release
	cd examples/triangle && "../build/RelWithDebInfo/enumerate_adapters/enumerate_adapters"

example-texture_arrays: examples-debug
	cd examples/build/Debug && cmake --build . --target texture_arrays

run-example-texture_arrays: example-texture_arrays
	cd examples/texture_arrays && "../build/Debug/texture_arrays/texture_arrays"

example-texture_arrays-release: examples-release
	cd examples/build/RelWithDebInfo && cmake --build . --target texture_arrays

run-example-texture_arrays-release: example-texture_arrays-release
	cd examples/texture_arrays && "../build/RelWithDebInfo/texture_arrays/texture_arrays"

example-triangle: examples-debug
	cd examples/build/Debug && cmake --build . --target triangle

run-example-triangle: example-triangle
	cd examples/triangle && "../build/Debug/triangle/triangle"

example-triangle-release: examples-release
	cd examples/build/RelWithDebInfo && cmake --build . --target triangle

run-example-triangle-release: example-triangle-release
	cd examples/triangle && "../build/RelWithDebInfo/triangle/triangle"
