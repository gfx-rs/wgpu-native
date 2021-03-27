
project('wgpu-native', 'c', version : '0.6.0', license : 'MPL 2.0')

inc_dirs = [include_directories('ffi', is_system: true)]

cargo = find_program('cargo')
#determin which dir contain platform library
if ['linux','android','darwin','cygwin','freebsd','netbsd','openbsd'].contains(host_machine.system())
    lib_path='target/release/libwgpu_native.a'
else
    lib_path='target/release/wgpu_native.lib'
endif

#build by cargo
run_command(cargo,'build','--release')
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
install_subdir('include', install_dir : '')
