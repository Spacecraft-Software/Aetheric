// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

/// Default bound for command channels between Majestic and RMS.
///
/// Chosen as 256 to provide backpressure without stalling the interactive
/// thread under normal load. Increasing this delays backpressure but uses
/// more memory; decreasing it tightens backpressure at the cost of latency.
pub const COMMAND_CHANNEL_BOUND: usize = 256;
