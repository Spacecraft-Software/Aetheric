// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::fmt;

/// Errors that can occur during IPC operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    /// Cap'n Proto encoding or decoding failure.
    Capnp,
    /// Underlying I/O error.
    Io,
    /// Peer disconnected.
    Disconnected,
    /// Protocol version mismatch.
    Version,
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::Capnp => write!(f, "capnp codec error"),
            IpcError::Io => write!(f, "I/O error"),
            IpcError::Disconnected => write!(f, "peer disconnected"),
            IpcError::Version => write!(f, "protocol version mismatch"),
        }
    }
}

impl std::error::Error for IpcError {}

impl From<capnp::Error> for IpcError {
    fn from(_: capnp::Error) -> Self {
        IpcError::Capnp
    }
}

impl From<capnp::NotInSchema> for IpcError {
    fn from(_: capnp::NotInSchema) -> Self {
        IpcError::Capnp
    }
}

impl From<std::str::Utf8Error> for IpcError {
    fn from(_: std::str::Utf8Error) -> Self {
        IpcError::Capnp
    }
}
