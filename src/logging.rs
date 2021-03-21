use crate::{map_enum, native};
use log::{Level, LevelFilter, Metadata, Record};
use std::ffi::CString;

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
