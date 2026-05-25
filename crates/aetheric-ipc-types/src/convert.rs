// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::core_capnp::core_command;

/// Trait for building a `CoreCommand` into a Cap'n Proto message.
pub trait CommandBuilder {
    /// Build the command into the provided Cap'n Proto builder.
    ///
    /// # Errors
    ///
    /// Returns an error if the message exceeds Cap'n Proto size limits.
    fn build_into(&self, builder: core_command::Builder<'_>) -> Result<(), crate::IpcError>;
}

/// Trait for reading an `EditorEvent` from a Cap'n Proto message.
pub trait EventReader {
    /// Read an event from the provided Cap'n Proto reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the union discriminant is unknown.
    fn read_from(
        reader: crate::core_capnp::editor_event::Reader<'_>,
    ) -> Result<Self, crate::IpcError>
    where
        Self: Sized;
}

impl CommandBuilder for crate::CoreCommand {
    fn build_into(&self, builder: core_command::Builder<'_>) -> Result<(), crate::IpcError> {
        self.to_builder(builder)
    }
}

impl EventReader for crate::EditorEvent {
    fn read_from(
        reader: crate::core_capnp::editor_event::Reader<'_>,
    ) -> Result<Self, crate::IpcError> {
        Self::try_from_reader(reader)
    }
}
