// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

//! Owned `Event` and `Cmd` enums for the SingleScrew bridge.

/// Owned event received from RMS.
#[derive(Debug, Clone)]
pub enum Event {
    /// Key press (UTF-8).
    KeyPress(String),
    /// Window resize.
    WindowResize { width_mm: f32, height_mm: f32 },
    /// Shutdown requested.
    ClientShutdown,
    /// Buffer opened.
    BufferOpened { buffer_id: u32, path: String },
    /// Snapshot ready.
    SnapshotReady {
        snapshot_id: u32,
        buffer_id: u32,
        version: u64,
    },
    /// Snapshot released.
    SnapshotReleased(u32),
    /// Buffer closed.
    BufferClosed(u32),
    /// Error from RMS.
    Error {
        code: u32,
        message: String,
        cmd_id: u64,
    },
}

/// Owned command to send to RMS.
#[derive(Debug, Clone)]
pub enum Cmd {
    /// Open buffer.
    OpenBuffer(String),
    /// Insert text.
    InsertText {
        buffer_id: u32,
        byte_offset: u64,
        content: String,
    },
    /// Delete text.
    DeleteText {
        buffer_id: u32,
        byte_offset: u64,
        length: u64,
    },
    /// Request snapshot.
    RequestSnapshot(u32),
    /// Release snapshot.
    ReleaseSnapshot(u32),
    /// Close buffer.
    CloseBuffer(u32),
    /// Shutdown.
    Shutdown,
}

/// Read an event from the connection (blocking).
///
/// # Errors
///
/// Returns an error on I/O failure.
pub fn read_event() -> std::io::Result<Event> {
    todo!("AE-P1-060: implement read_event")
}

/// Write a command to the connection.
///
/// # Errors
///
/// Returns an error on I/O failure.
pub fn write_cmd(_cmd: &Cmd) -> std::io::Result<()> {
    todo!("AE-P1-060: implement write_cmd")
}
