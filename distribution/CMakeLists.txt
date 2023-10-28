cmake_minimum_required(VERSION 3.0.0...3.24 FATAL_ERROR)
project(webgpu-backend-wgpu VERSION 1.0.0)

message(STATUS "Using wgpu-native backend for WebGPU")

if (EMSCRIPTEN)

	add_library(webgpu INTERFACE)

	target_include_directories(webgpu INTERFACE
		"${CMAKE_CURRENT_SOURCE_DIR}/include-emscripten"
	)

	# This is used to advertise the flavor of WebGPU that this zip provides
	target_compile_definitions(webgpu INTERFACE WEBGPU_BACKEND_EMSCRIPTEN)

	function(target_copy_webgpu_binaries Target)
	endfunction()

else (EMSCRIPTEN)

	set(WGPU ${CMAKE_CURRENT_SOURCE_DIR})
	if (NOT ARCH)
		set(ARCH ${CMAKE_SYSTEM_PROCESSOR})
		if (ARCH STREQUAL "AMD64")
			set(ARCH "x86_64")
		endif()
	endif()

	# A pre-compiled target (IMPORTED) that is a dynamically
	# linked library (SHARED, meaning .dll, .so or .dylib).
	add_library(webgpu SHARED IMPORTED GLOBAL)

	# This is used to advertise the flavor of WebGPU that this zip provides
	target_compile_definitions(webgpu INTERFACE WEBGPU_BACKEND_WGPU)

	if(CMAKE_SYSTEM_NAME STREQUAL "Windows")

		set(WGPU_RUNTIME_LIB ${WGPU}/bin/windows-${ARCH}/wgpu_native.dll)
		set_target_properties(
			webgpu
			PROPERTIES
				IMPORTED_LOCATION "${WGPU_RUNTIME_LIB}"
				IMPORTED_IMPLIB "${WGPU}/bin/windows-${ARCH}/wgpu_native.lib"
				INTERFACE_INCLUDE_DIRECTORIES "${WGPU}/include"
		)

	elseif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

		set(WGPU_RUNTIME_LIB ${WGPU}/bin/linux-${ARCH}/libwgpu_native.so)
		set_target_properties(
			webgpu
			PROPERTIES
				IMPORTED_LOCATION "${WGPU_RUNTIME_LIB}"
				INTERFACE_INCLUDE_DIRECTORIES "${WGPU}/include"
		)

	elseif(CMAKE_SYSTEM_NAME STREQUAL "Darwin")

		set(WGPU_RUNTIME_LIB ${WGPU}/bin/macos-${ARCH}/libwgpu_native.dylib)
		set_target_properties(
			webgpu
			PROPERTIES
				IMPORTED_LOCATION "${WGPU_RUNTIME_LIB}"
				INTERFACE_INCLUDE_DIRECTORIES "${WGPU}/include"
		)

	else()

		message(FATAL_ERROR "Plateform not supported by this release of WebGPU. You may consider building it yourself from its source (see https://github.com/gfx-rs/wgpu-native)")

	endif()

	message(STATUS "Using WebGPU runtime from '${WGPU_RUNTIME_LIB}'")
	set(WGPU_RUNTIME_LIB ${WGPU_RUNTIME_LIB} PARENT_SCOPE)
	set(WGPU_RUNTIME_LIB ${WGPU_RUNTIME_LIB} CACHE INTERNAL "Path to the WebGPU library binary")

	# The application's binary must find wgpu.dll or libwgpu.so at runtime,
	# so we automatically copy it (it's called WGPU_RUNTIME_LIB in general)
	# next to the binary.
	# Also make sure that the binarie's RPATH is set to find this shared library.
	function(target_copy_webgpu_binaries Target)
		add_custom_command(
			TARGET ${Target} POST_BUILD
			COMMAND
				${CMAKE_COMMAND} -E copy_if_different
				${WGPU_RUNTIME_LIB}
				$<TARGET_FILE_DIR:${Target}>
			COMMENT
				"Copying '${WGPU_RUNTIME_LIB}' to '$<TARGET_FILE_DIR:${Target}>'..."
		)
		set_target_properties(${Target} PROPERTIES INSTALL_RPATH "./")

		if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
			# Bug fix, there might be a cleaner way to do this but no INSTALL_RPATH
			# or related target properties seem to be a solution.
			if(CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
				set(ARCH_DIR aarch64)
			else()
				set(ARCH_DIR ${CMAKE_SYSTEM_PROCESSOR})
			endif()
			add_custom_command(
				TARGET ${Target} POST_BUILD
				COMMAND
					${CMAKE_INSTALL_NAME_TOOL} "-change"
					"/Users/runner/work/wgpu-native/wgpu-native/target/${ARCH_DIR}-apple-darwin/release/deps/libwgpu_native.dylib"
					"@executable_path/libwgpu_native.dylib"
					"$<TARGET_FILE:${Target}>"
				VERBATIM
			)
		endif()
	endfunction()

endif (EMSCRIPTEN)
