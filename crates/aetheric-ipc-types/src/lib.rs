// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Owned Cap'n Proto types for the Constellation IPC schema.

pub mod commands;
pub mod convert;
pub mod error;
pub mod events;
pub mod schema;

// Re-export generated modules at crate root so internal cross-references resolve.
pub mod core_capnp {
    #![allow(missing_docs)]
    include!(concat!(env!("OUT_DIR"), "/core_capnp.rs"));
}
pub mod orchestration_capnp {
    #![allow(missing_docs)]
    include!(concat!(env!("OUT_DIR"), "/orchestration_capnp.rs"));
}

#[doc(inline)]
pub use commands::{CoreCommand, OrchCommand, OrchEvent};
#[doc(inline)]
pub use convert::{CommandBuilder, EventReader};
#[doc(inline)]
pub use error::IpcError;
#[doc(inline)]
pub use events::EditorEvent;

#[cfg(test)]
mod tests {
    use super::*;
    use capnp::message::Builder;

    #[test]
    fn round_trip_key_press() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            root.set_key_press("hello");
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(event, EditorEvent::KeyPress("hello".to_string()));
    }

    #[test]
    fn round_trip_window_resize() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            let mut dims = root.init_window_resize();
            dims.set_width_mm(1920.0);
            dims.set_height_mm(1080.0);
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(
            event,
            EditorEvent::WindowResize {
                width_mm: 1920.0,
                height_mm: 1080.0,
            }
        );
    }

    #[test]
    fn round_trip_buffer_opened() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            let mut handle = root.init_buffer_opened();
            handle.set_buffer_id(42);
            handle.set_path("/tmp/test.txt");
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(
            event,
            EditorEvent::BufferOpened {
                buffer_id: 42,
                path: "/tmp/test.txt".to_string(),
            }
        );
    }

    #[test]
    fn round_trip_snapshot_ready() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            let mut handle = root.init_snapshot_ready();
            handle.set_snapshot_id(7);
            handle.set_buffer_id(42);
            handle.set_version(1);
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(
            event,
            EditorEvent::SnapshotReady {
                snapshot_id: 7,
                buffer_id: 42,
                version: 1,
            }
        );
    }

    #[test]
    fn round_trip_snapshot_released() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            root.set_snapshot_released(7);
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(event, EditorEvent::SnapshotReleased(7));
    }

    #[test]
    fn round_trip_buffer_closed() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            root.set_buffer_closed(42);
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(event, EditorEvent::BufferClosed(42));
    }

    #[test]
    fn round_trip_error() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            let mut err = root.init_error();
            err.set_code(404);
            err.set_message("not found");
            err.set_cmd_id(99);
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(
            event,
            EditorEvent::Error {
                code: 404,
                message: "not found".to_string(),
                cmd_id: 99,
            }
        );
    }

    #[test]
    fn round_trip_client_shutdown() {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<core_capnp::editor_event::Builder>();
            root.set_client_shutdown(());
        }
        let reader = builder.get_root_as_reader::<core_capnp::editor_event::Reader>().unwrap();
        let event = EditorEvent::try_from_reader(reader).unwrap();
        assert_eq!(event, EditorEvent::ClientShutdown);
    }
}
