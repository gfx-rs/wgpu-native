.PHONY: gen

# TODO: change 'gen' invocation to use submodule

gen:
	cd ../../webgpu-headers && \
	go run ./gen \
		-schema ./schema.json \
		-yaml ./webgpu.yml \
		-header ./webgpu.h \
		-yaml ../wgpu-native/ffi/wgpu.yml \
		-header ../wgpu-native/ffi/wgpu.h
