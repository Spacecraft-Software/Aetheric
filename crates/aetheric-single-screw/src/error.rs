// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::cell::RefCell;
use std::ffi::{c_char, c_int, CString};

thread_local! {
    static LAST_CODE: RefCell<c_int> = RefCell::new(0);
    static LAST_MSG: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

/// Set the thread-local last error.
pub fn set_last(code: c_int, msg: &str) {
    LAST_CODE.with(|c| *c.borrow_mut() = code);
    LAST_MSG.with(|m| {
        *m.borrow_mut() = CString::new(msg).unwrap_or_else(|_| CString::new("").unwrap())
    });
}

/// Clear the thread-local last error.
pub fn clear_last() {
    LAST_CODE.with(|c| *c.borrow_mut() = 0);
    LAST_MSG.with(|m| *m.borrow_mut() = CString::new("").unwrap());
}

/// Get the last error code.
pub fn last_code() -> c_int {
    LAST_CODE.with(|c| *c.borrow())
}

/// Get a pointer to the last error message.
///
/// # Safety
/// The pointer is valid only until the next SS call on this thread.
pub unsafe fn last_message_ptr() -> *const c_char {
    LAST_MSG.with(|m| m.borrow().as_ptr())
}

/// Error constants.
pub const RMS_ERR_OK: c_int = 0;
pub const RMS_ERR_IO: c_int = 1;
pub const RMS_ERR_WOULD_BLOCK: c_int = 2;
pub const RMS_ERR_DISCONNECTED: c_int = 3;
pub const RMS_ERR_VERSION: c_int = 4;
pub const RMS_ERR_PANIC: c_int = 5;
