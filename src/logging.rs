/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use log::{Record, Metadata};
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

pub type LogCallback = unsafe extern "C" fn(msg: *const c_char);

struct LogProxy {level: log::Level, callback: LogCallback}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}", record.level(), record.args());
            // println!("{}", msg);  // Print the message to stdout
            let cb: LogCallback = self.callback;
            let s = CString::new(msg).unwrap();
            let p = s.as_ptr();
            unsafe {
                cb(p);
            }
            // We do not use std::mem::forget(s), so the memory will be reclaimed
            // by Rust once this function here. The callback should thus make a
            // copy of the string. This is fine, because its probably converted to
            // an internal (unicode) string type.
        }
    }

    fn flush(&self) {}
}

// Store the logger instance as a mutable nullable static object
static mut LOGGER: Option<LogProxy> = None;

#[no_mangle]
pub unsafe extern "C" fn wgpu_set_log_callback(level: c_int, callback: LogCallback) {
    // Get level
    let level_filter = match level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    // Instantiate logger and store as the static object
    let logger: LogProxy = LogProxy{level: level_filter.to_level().unwrap(), callback: callback};
    LOGGER = Some(logger);
    // set_logger seems to require that the logger is static,
    // so we need to use LOGGER instead of logger.
    match &LOGGER {
        Some(logger) =>log::set_logger(logger)
            .map(|()| log::set_max_level(level_filter)).unwrap(),
        None => println!("{}", "Failed to set logger"),
    }
}
