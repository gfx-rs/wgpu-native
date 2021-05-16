use crate::{map_enum, native};
use log::{Level, LevelFilter, Metadata, Record};
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn wgpuGetVersion() -> std::os::raw::c_uint {
    // Take the string of WGPU_NATIVE_VERSION, strip any leading v's, split on dots,
    // and map the first 4 parts to the bytes of an uint32, consuming MSB first.
    // e.g. "v4.1"      -> 0x04010000
    //      "5.4.3.2.1" -> 0x05040302
    let static_str = match option_env!("WGPU_NATIVE_VERSION") {
        Some(s) => s.trim().trim_start_matches("v"),
        None => "",
    };
    let mut version: u32 = 0;
    let mut index: i32 = 0;
    for part in static_str.split(".") {
        let versionpart: u32 = match part.parse::<u32>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let shift: i32 = 8 * (3 - index);
        if shift < 0 {
            break;
        }
        version += versionpart << shift;
        index += 1;
    }
    version
}

struct Logger {
    callback: native::WGPULogCallback,
    initialized: bool,
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        unsafe {
            if self.enabled(record.metadata()) && LOGGER.callback.is_some() {
                let callback = LOGGER.callback.unwrap();
                let msg = record.args().to_string();
                let msg_c = CString::new(msg).unwrap();
                let level = match record.level() {
                    Level::Error => native::WGPULogLevel_Error,
                    Level::Warn => native::WGPULogLevel_Warn,
                    Level::Info => native::WGPULogLevel_Info,
                    Level::Debug => native::WGPULogLevel_Debug,
                    Level::Trace => native::WGPULogLevel_Trace,
                };
                callback(level, msg_c.as_ptr());

                // We do not use std::mem::forget(msg_c), so Rust will reclaim the memory
                // once msg_c gets cleared. The callback should thus make a copy.
            }
        }
    }

    fn flush(&self) {}
}

static mut LOGGER: Logger = Logger {
    callback: None,
    initialized: false,
};

#[no_mangle]
pub unsafe extern "C" fn wgpuSetLogCallback(callback: native::WGPULogCallback) {
    if !LOGGER.initialized {
        LOGGER.initialized = true;
        log::set_logger(&LOGGER).unwrap();
        if log::max_level() == LevelFilter::Off {
            log::set_max_level(LevelFilter::Warn);
        }
    }

    LOGGER.callback = callback;
}

#[no_mangle]
pub unsafe extern "C" fn wgpuSetLogLevel(level: native::WGPULogLevel) {
    log::set_max_level(map_log_level(level));
}

map_enum!(
    map_log_level,
    WGPULogLevel,
    LevelFilter,
    "Unknown log level",
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace
);
