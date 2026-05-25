// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::core_capnp::editor_event;

/// Owned representation of an event sent from RMS to Majestic.
#[derive(Debug, Clone, PartialEq)]
pub enum EditorEvent {
    /// A key was pressed (UTF-8 string).
    KeyPress(String),
    /// The window was resized.
    WindowResize {
        /// Width in millimetres.
        width_mm: f32,
        /// Height in millimetres.
        height_mm: f32,
    },
    /// The client requested shutdown.
    ClientShutdown,
    /// A buffer was opened.
    BufferOpened {
        /// Buffer id.
        buffer_id: u32,
        /// File path.
        path: String,
    },
    /// A snapshot is ready for reading.
    SnapshotReady {
        /// Snapshot handle.
        snapshot_id: u32,
        /// Source buffer.
        buffer_id: u32,
        /// Rope version at snapshot time.
        version: u64,
    },
    /// A snapshot handle was released.
    SnapshotReleased(u32),
    /// A buffer was closed.
    BufferClosed(u32),
    /// An error occurred in RMS.
    Error {
        /// Error code.
        code: u32,
        /// Human-readable message.
        message: String,
        /// Command id that triggered the error.
        cmd_id: u64,
    },
}

impl EditorEvent {
    /// Convert from a Cap'n Proto reader to an owned event.
    ///
    /// # Errors
    ///
    /// Returns an error if the union discriminant is unknown or if a text
    /// field contains invalid UTF-8.
    pub fn try_from_reader(reader: editor_event::Reader<'_>) -> Result<Self, crate::IpcError> {
        use editor_event::Which;

        Ok(match reader.which().map_err(|_| crate::IpcError::Capnp)? {
            Which::KeyPress(r) => {
                let s = r?.to_string()?;
                EditorEvent::KeyPress(s)
            }
            Which::WindowResize(r) => {
                let dims = r.map_err(|_| crate::IpcError::Capnp)?;
                EditorEvent::WindowResize {
                    width_mm: dims.get_width_mm(),
                    height_mm: dims.get_height_mm(),
                }
            }
            Which::ClientShutdown(()) => EditorEvent::ClientShutdown,
            Which::BufferOpened(r) => {
                let h = r.map_err(|_| crate::IpcError::Capnp)?;
                EditorEvent::BufferOpened {
                    buffer_id: h.get_buffer_id(),
                    path: h.get_path()?.to_string()?,
                }
            }
            Which::SnapshotReady(r) => {
                let h = r.map_err(|_| crate::IpcError::Capnp)?;
                EditorEvent::SnapshotReady {
                    snapshot_id: h.get_snapshot_id(),
                    buffer_id: h.get_buffer_id(),
                    version: h.get_version(),
                }
            }
            Which::SnapshotReleased(id) => EditorEvent::SnapshotReleased(id),
            Which::BufferClosed(id) => EditorEvent::BufferClosed(id),
            Which::Error(r) => {
                let e = r.map_err(|_| crate::IpcError::Capnp)?;
                EditorEvent::Error {
                    code: e.get_code(),
                    message: e.get_message()?.to_string()?,
                    cmd_id: e.get_cmd_id(),
                }
            }
        })
    }
}
