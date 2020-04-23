use std::sync::Arc;

mod command;
mod device;
mod logging;

pub use self::command::*;
pub use self::device::*;
pub use self::logging::*;

type Global = core::hub::Global<core::hub::IdentityManagerFactory>;

lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Global> = Arc::new(Global::new("wgpu", core::hub::IdentityManagerFactory));
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_get_version() -> *const std::os::raw::c_char {
    let version = env!("CARGO_PKG_VERSION");
    let version_c = std::ffi::CString::new(version).unwrap();
    let version_p = version_c.as_ptr();
    std::mem::forget(version_c); // see https://thefullsnack.com/en/string-ffi-rust.html
    return version_p;
}
