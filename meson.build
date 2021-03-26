project('wgpu-native','c',version : '0.6.0')
cargo = find_program('cargo')

#build by cargo
run_command(cargo,'build','--release')
#mark cargo artifact to static library
wgpu_native =static_library('wgpu_native',objects : 'target/release/libwgpu_native.a')

dep_wgpu_native=declare_dependency(
link_with:wgpu_native,
include_directories: ['ffi/']
)

pkg = import('pkgconfig')
pkg.generate(name : 'wgpu-native',
             description : 'Headers for using wgpu-native',
             libraries : [wgpu_native],
             version : meson.project_version(),
             )
install_subdir('include', install_dir : '')