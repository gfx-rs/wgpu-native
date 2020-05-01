use std::sync::Arc;

mod command;
mod device;
mod logging;

pub use self::command::*;
pub use self::device::*;
pub use self::logging::*;

type Global = wgc::hub::Global<wgc::hub::IdentityManagerFactory>;

lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Global> = Arc::new(Global::new("wgpu", wgc::hub::IdentityManagerFactory));
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_get_version() -> std::os::raw::c_uint {
    let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    return (major << 16) + (minor << 8) + patch;
}
