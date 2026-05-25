// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_ipc_types::EditorEvent;
use std::sync::Arc;

/// Common renderer trait implemented by Nova (GPU) and Penumbra (TTY).
pub trait Renderer: Send {
    /// Render the current snapshot into the viewport.
    fn present(&mut self, snapshot: &Arc<crate::state::SharedState>);

    /// Poll for input events (keys, resize, etc.).
    fn poll_input(&mut self) -> Vec<EditorEvent>;
}
