// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use tokio::sync::mpsc;

/// Default bound for command channels between Majestic and RMS.
///
/// Chosen as 256 to provide backpressure without stalling the interactive
/// thread under normal load. Increasing this delays backpressure but uses
/// more memory; decreasing it tightens backpressure at the cost of latency.
pub const COMMAND_CHANNEL_BOUND: usize = 256;

/// Bound for render-specific command channels (e.g. viewport updates).
pub const RENDER_CHANNEL_BOUND: usize = 64;

/// Bound for high-priority control channels (e.g. shutdown signals).
pub const CONTROL_CHANNEL_BOUND: usize = 4;

/// Type alias for bounded sender used throughout Orion.
pub type Sender<T> = mpsc::Sender<T>;
/// Type alias for bounded receiver used throughout Orion.
pub type Receiver<T> = mpsc::Receiver<T>;
