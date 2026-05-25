// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![warn(missing_docs)]

//! SingleScrew (SS) — the only unsafe crate in Aetheric.
//!
//! Exactly two documented unsafe sites:
//! 1. `extern "C"` FFI boundary (`catch_unwind` wrapped).
//! 2. POSIX `pipe(2)` / `read(2)` / `write(2)` in the background reader thread.
//!
//! Produces `librms_ipc.so`, consumed by Majestic via `(aetheric rms-ipc)`.

pub mod conn;
pub mod error;
pub mod proto;

use conn::Conn;
use std::ffi::{c_char, c_int, CStr, CString};
use std::panic::catch_unwind;

/// Opaque connection handle.
pub enum RmsConn {}
/// Opaque message handle (received event).
pub enum RmsMsg {}
/// Opaque command handle (to be sent).
pub enum RmsCmd {}

/// Return the SS version string.
///
/// # Safety
/// Caller must later free the returned pointer with `rms_free_string`.
#[no_mangle]
pub unsafe extern "C" fn rms_version() -> *mut c_char {
    match catch_unwind(std::panic::AssertUnwindSafe(
        || -> anyhow::Result<*mut c_char> {
            let s = CString::new(env!("CARGO_PKG_VERSION"))?;
            Ok(s.into_raw())
        },
    )) {
        Ok(Ok(ptr)) => ptr,
        Ok(Err(e)) => {
            error::set_last(error::RMS_ERR_IO, &format!("{e}"));
            std::ptr::null_mut()
        }
        Err(_) => {
            error::set_last(error::RMS_ERR_PANIC, "panic in extern C function");
            std::ptr::null_mut()
        }
    }
}

/// Return the last error code.
#[no_mangle]
pub extern "C" fn rms_last_error() -> c_int {
    error::last_code()
}

/// Return the last error message.
///
/// # Safety
/// Caller must not free the returned pointer; it is thread-local and valid
/// until the next SS call on this thread.
#[no_mangle]
pub unsafe extern "C" fn rms_last_error_message() -> *const c_char {
    error::last_message_ptr()
}

/// Connect to the RMS microkernel at the given socket path.
///
/// # Safety
/// `path` must be a valid NUL-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn rms_connect(path: *const c_char) -> *mut RmsConn {
    match catch_unwind(std::panic::AssertUnwindSafe(
        || -> anyhow::Result<*mut RmsConn> {
            let path = unsafe { CStr::from_ptr(path) }.to_str()?;
            let conn = Conn::connect(path)?;
            let boxed = Box::new(conn);
            Ok(Box::into_raw(boxed).cast::<RmsConn>())
        },
    )) {
        Ok(Ok(ptr)) => ptr,
        Ok(Err(e)) => {
            error::set_last(error::RMS_ERR_IO, &format!("{e}"));
            std::ptr::null_mut()
        }
        Err(_) => {
            error::set_last(error::RMS_ERR_PANIC, "panic in extern C function");
            std::ptr::null_mut()
        }
    }
}

/// Disconnect and free a connection.
///
/// # Safety
/// `conn` must be a valid pointer returned by `rms_connect`, or null.
#[no_mangle]
pub unsafe extern "C" fn rms_disconnect(conn: *mut RmsConn) {
    if conn.is_null() {
        return;
    }
    let _ = catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: conn is a valid Box<Conn> cast.
        let _ = unsafe { Box::from_raw(conn.cast::<Conn>()) };
    }));
}

/// Return a pollable notification file descriptor.
///
/// # Safety
/// `conn` must be a valid pointer returned by `rms_connect`.
#[no_mangle]
pub unsafe extern "C" fn rms_fd(conn: *const RmsConn) -> c_int {
    match catch_unwind(std::panic::AssertUnwindSafe(|| -> anyhow::Result<c_int> {
        let conn = unsafe { &*conn.cast::<Conn>() };
        Ok(conn.notify_fd())
    })) {
        Ok(Ok(fd)) => fd,
        Ok(Err(e)) => {
            error::set_last(error::RMS_ERR_IO, &format!("{e}"));
            -1
        }
        Err(_) => {
            error::set_last(error::RMS_ERR_PANIC, "panic in extern C function");
            -1
        }
    }
}

/// Free a string returned by SS.
///
/// # Safety
/// `s` must be a pointer returned by `rms_version` or null.
#[no_mangle]
pub unsafe extern "C" fn rms_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = unsafe { CString::from_raw(s) };
    }));
}

/// Error constants exposed to C.
pub const RMS_ERR_OK: c_int = 0;
pub const RMS_ERR_IO: c_int = 1;
pub const RMS_ERR_WOULD_BLOCK: c_int = 2;
pub const RMS_ERR_DISCONNECTED: c_int = 3;
pub const RMS_ERR_VERSION: c_int = 4;
pub const RMS_ERR_PANIC: c_int = 5;
