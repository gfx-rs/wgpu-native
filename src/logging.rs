/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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
            // We do not use std::mem::forget(s), so Rust will reclaim the memory
            // once msg_c gets cleared. The callback should thus make a copy.
        }
    }

    fn flush(&self) {}
}

// Store the logger instance as a mutable nullable static object
static mut LOGGER: Option<LogProxy> = None;

// Note: this function may only be called once in the lifetime of a program.
#[no_mangle]
pub unsafe extern "C" fn wgpu_set_log_callback(callback: LogCallback) {
    // Instantiate logger, store as static object, and make it THE logger.
    let logger: LogProxy = LogProxy { callback: callback };
    LOGGER = Some(logger);
    log::set_logger(LOGGER.as_ref().unwrap()).unwrap();
    // The max_level is Off by default. If not set yet, set it to Warn instead.
    if log::max_level() == LevelFilter::Off {
        log::set_max_level(LevelFilter::Warn);
    }
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_set_log_level(level: c_int) {
    let level_filter = match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    log::set_max_level(level_filter);
}
