// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

//! Public schema re-exports. Ordinals are permanent — never reuse.

/// Core schema: EditorEvent + CoreCommand.
pub mod core {
    pub use crate::core_capnp::*;
}
/// Orchestration schema: Celestial, Nexus, AI streaming.
pub mod orchestration {
    pub use crate::orchestration_capnp::*;
}
