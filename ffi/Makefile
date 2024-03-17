.PHONY: gen

gen:
	cd ./webgpu-headers && \
	go run ./gen \
		-schema ./schema.json \
		-yaml ./webgpu.yml \
		-header ./webgpu.h \
		-yaml ../wgpu.yml \
		-header ../wgpu.h
