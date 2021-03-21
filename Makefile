RUST_BACKTRACE:=1
EXCLUDES:=

GENERATOR_PLATFORM:=

FFI_DIR:=ffi
BUILD_DIR:=build
CREATE_BUILD_DIR:=
OUTPUT_DIR:=
LIB_NAME:=wgpu_native

WILDCARD_SOURCE:=$(wildcard src/*.rs)

GIT_TAG=$(shell git describe --abbrev=0 --tags)
GIT_TAG_FULL=$(shell git describe --tags)
OS_NAME=

ifeq (,$(TARGET))
	CHECK_TARGET_FLAG=
else
	CHECK_TARGET_FLAG=--target $(TARGET)
endif

ifeq ($(OS),Windows_NT)
	CREATE_BUILD_DIR=if exist "$(BUILD_DIR)" rmdir /s /q $(BUILD_DIR) && mkdir $(BUILD_DIR)
	GENERATOR_PLATFORM=-DCMAKE_GENERATOR_PLATFORM=x64
	OUTPUT_DIR=build/Debug
else
	CREATE_BUILD_DIR=mkdir -p $(BUILD_DIR)
	OUTPUT_DIR=build
endif

ifeq ($(OS),Windows_NT)
	LIB_EXTENSION=dll
	OS_NAME=windows
else
	UNAME_S:=$(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		LIB_EXTENSION=so
		OS_NAME=linux
	endif
	ifeq ($(UNAME_S),Darwin)
		LIB_EXTENSION=dylib
		OS_NAME=macos
	endif
endif


.PHONY: all check test doc clear \
	example-compute example-triangle \
	run-example-compute run-example-triangle  \
	lib-native lib-native-release

all: example-compute example-triangle example-capture

package: lib-native lib-native-release
	mkdir -p dist
	echo "$(GIT_TAG_FULL)" > dist/commit-sha
	for RELEASE in debug release; do \
		ARCHIVE=wgpu-$$RELEASE-$(OS_NAME)-$(GIT_TAG).zip; \
		rm -f dist/$$ARCHIVE; \
		if [ $(OS_NAME) = windows ]; then \
			7z a -tzip dist/$$ARCHIVE ./target/$$RELEASE/$(LIB_NAME).$(LIB_EXTENSION) ./target/$$RELEASE/$(LIB_NAME).$(LIB_EXTENSION).lib ./ffi/*.h ./dist/commit-sha; \
		else \
			zip -j dist/$$ARCHIVE target/$$RELEASE/lib$(LIB_NAME).$(LIB_EXTENSION) ffi/*.h dist/commit-sha; \
		fi; \
	done

clean:
	cargo clean
	rm -Rf examples/compute/build examples/triangle/build

check:
	cargo check --all

test:
	cargo test --all

doc:
	cargo doc --all

clear:
	cargo clean

lib-native: Cargo.lock Cargo.toml Makefile $(WILDCARD_SOURCE)
	cargo build

lib-native-release: Cargo.lock Cargo.toml Makefile $(WILDCARD_SOURCE)
	cargo build --release

example-compute: lib-native examples/compute/main.c
	cd examples/compute && $(CREATE_BUILD_DIR) && cd build && cmake -DCMAKE_BUILD_TYPE=Debug .. $(GENERATOR_PLATFORM) && cmake --build .

run-example-compute: example-compute
	cd examples/compute && "$(OUTPUT_DIR)/compute" 1 2 3 4

example-triangle: lib-native examples/triangle/main.c
	cd examples/triangle && $(CREATE_BUILD_DIR) && cd build && cmake -DCMAKE_BUILD_TYPE=Debug .. $(GENERATOR_PLATFORM) && cmake --build .

run-example-triangle: example-triangle
	cd examples/triangle && "$(OUTPUT_DIR)/triangle"

build-helper:
	cargo build -p helper

example-capture: lib-native build-helper examples/capture/main.c
	cd examples/capture && $(CREATE_BUILD_DIR) && cd build && cmake -DCMAKE_BUILD_TYPE=Debug .. $(GENERATOR_PLATFORM) && cmake --build .

run-example-capture: example-capture
	cd examples/capture && "$(OUTPUT_DIR)/capture"
