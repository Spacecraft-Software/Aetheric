// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_stratum::{SnapshotRegistry, Stratum};
use arc_swap::ArcSwap;
use std::sync::Arc;

/// Shared state visible to all Orion tasks.
#[derive(Debug)]
pub struct SharedState {
    /// The current rope, atomically swapped after each mutation.
    pub rope: ArcSwap<Arc<Stratum>>,
    /// Active snapshot handles.
    pub snapshots: SnapshotRegistry,
    /// Channel to signal the render task that a repaint is needed.
    pub render_tx: tokio::sync::mpsc::Sender<RenderSignal>,
}

/// Signal sent to the render task.
#[derive(Debug, Clone, Copy)]
pub enum RenderSignal {
    /// Repaint the viewport.
    Repaint,
    /// Toggle the debug overlay.
    ToggleOverlay,
    /// Shut down the render loop.
    Shutdown,
}
