use log::{LevelFilter, Metadata, Record};
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

pub type LogCallback = unsafe extern "C" fn(level: c_int, msg: *const c_char);

struct LogProxy {
    callback: LogCallback,
}

impl log::Log for LogProxy {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let callback: LogCallback = self.callback;
            let msg = record.args().to_string();
            let msg_c = CString::new(msg).unwrap();
            unsafe {
                callback(record.level() as c_int, msg_c.as_ptr());
            }
            // We do not use std::mem::forget(msg_c), so Rust will reclaim the memory
            // once msg_c gets cleared. The callback should thus make a copy.
        }
    }

    fn flush(&self) {}
}

// Store the logger instance as a mutable nullable static object
static mut LOGGER: Option<LogProxy> = None;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

// Note: this function may only be called once in the lifetime of a program.
#[no_mangle]
pub unsafe extern "C" fn wgpu_set_log_callback(callback: LogCallback) {
    // Check if the logger has already been set
    match &LOGGER {
        Some(_) => panic!("The logger callback can only be set once."),
        None => (),
    }
    // Instantiate logger, store as static object, and make it THE logger.
    let logger: LogProxy = LogProxy { callback };
    LOGGER = Some(logger);
    log::set_logger(LOGGER.as_ref().unwrap()).unwrap();
    // The max_level is Off by default. If not set yet, set it to Warn instead.
    if log::max_level() == LevelFilter::Off {
        log::set_max_level(LevelFilter::Warn);
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_set_log_level(level: LogLevel) -> c_int {
    let level_val = level as i32;
    if level_val < LogLevel::Off as i32 || level_val > LogLevel::Trace as i32 {
        return -1;
    }
    let level_filter = match level {
        LogLevel::Off => LevelFilter::Off,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    };
    log::set_max_level(level_filter);
    return 0;
}
