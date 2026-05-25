// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::io;
use std::os::unix::io::RawFd;
use std::os::unix::net::UnixStream;

/// A connection to the RMS microkernel.
pub struct Conn {
    _stream: UnixStream,
    /// Read end of the POSIX pipe used for notification.
    read_fd: RawFd,
    /// Write end of the POSIX pipe.
    write_fd: RawFd,
}

impl Conn {
    /// Connect to the RMS microkernel at the given socket path.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket cannot be connected or if the
    /// notification pipe cannot be created.
    pub fn connect(path: &str) -> io::Result<Self> {
        let stream = UnixStream::connect(path)?;
        // SAFETY: pipe(2) is a standard POSIX call.
        let mut fds = [0; 2];
        let ret = unsafe { libc::pipe(fds.as_mut_ptr()) };
        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            _stream: stream,
            read_fd: fds[0],
            write_fd: fds[1],
        })
    }

    /// Return the notification file descriptor (read end of the pipe).
    pub fn notify_fd(&self) -> RawFd {
        self.read_fd
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        // SAFETY: close(2) on fds we own.
        unsafe {
            libc::close(self.read_fd);
            libc::close(self.write_fd);
        }
    }
}
