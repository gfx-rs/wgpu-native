use crate::{map_enum, native};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Metadata, Record};
use std::{ffi::CString, sync::Mutex};

#[no_mangle]
pub extern "C" fn wgpuGetVersion() -> std::os::raw::c_uint {
    // Take the string of WGPU_NATIVE_VERSION, strip any leading v's, split on dots,
    // and map the first 4 parts to the bytes of an uint32, consuming MSB first.
    // e.g. "v4.1"      -> 0x04010000
    //      "5.4.3.2.1" -> 0x05040302
    let static_str = match option_env!("WGPU_NATIVE_VERSION") {
        Some(s) => s.trim().trim_start_matches('v'),
        None => "",
    };
    let mut version: u32 = 0;
    for (index, part) in (0..).zip(static_str.split('.')) {
        let versionpart: u32 = match part.parse::<u32>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let shift: i32 = 8 * (3 - index);
        if shift < 0 {
            break;
        }
        version += versionpart << shift;
    }
    version
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let (callback, userdata) = {
            let logger = LOGGER_INFO.lock().unwrap();
            (logger.callback, logger.userdata)
        };

        if let Some(callback) = callback {
            let msg = record.args().to_string();
            let msg_c = CString::new(msg).unwrap();
            let level = match record.level() {
                Level::Error => native::WGPULogLevel_Error,
                Level::Warn => native::WGPULogLevel_Warn,
                Level::Info => native::WGPULogLevel_Info,
                Level::Debug => native::WGPULogLevel_Debug,
                Level::Trace => native::WGPULogLevel_Trace,
            };

            unsafe {
                callback(level, msg_c.as_ptr(), userdata);
            }

            // We do not use std::mem::forget(msg_c), so Rust will reclaim the memory
            // once msg_c gets cleared. The callback should thus make a copy.
        }
    }

    fn flush(&self) {}
}

struct LoggerInfo {
    initialized: bool,
    callback: native::WGPULogCallback,
    userdata: *mut std::os::raw::c_void,
}
unsafe impl Send for LoggerInfo {}

lazy_static! {
    static ref LOGGER_INFO: Mutex<LoggerInfo> = Mutex::new(LoggerInfo {
        initialized: false,
        callback: None,
        userdata: std::ptr::null_mut(),
    });
}

#[no_mangle]
pub extern "C" fn wgpuSetLogCallback(
    callback: native::WGPULogCallback,
    userdata: *mut std::os::raw::c_void,
) {
    let mut logger = LOGGER_INFO.lock().unwrap();
    logger.callback = callback;
    logger.userdata = userdata;
    if !logger.initialized {
        logger.initialized = true;
        log::set_logger(&Logger).unwrap();
        if log::max_level() == LevelFilter::Off {
            log::set_max_level(LevelFilter::Warn);
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpuSetLogLevel(level: native::WGPULogLevel) {
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
