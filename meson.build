
project('wgpu-native', 'c', version : '0.0.0', license : 'MIT OR Apache-2.0')

inc_dirs = [ ]

subdir('ffi')

cargo = find_program('cargo')
#determine which dir contain platform library
if ['linux','android','darwin','cygwin','freebsd','netbsd','openbsd'].contains(host_machine.system())
    lib_path='target/release/libwgpu_native.a'
else
    lib_path='target/release/wgpu_native.lib'
endif

#build by cargo
run_command(cargo,'build','--release', check: true)
#mark cargo artifact to static library
wgpu_native = static_library('wgpu_native', objects : lib_path, install : true)

dep_wgpu_native = declare_dependency(
link_with:wgpu_native,
include_directories: inc_dirs
)

pkg = import('pkgconfig')
pkg.generate(name : 'wgpu-native',
             description : 'Headers for using wgpu-native',
             libraries : [wgpu_native],
             version : meson.project_version(),
             )
