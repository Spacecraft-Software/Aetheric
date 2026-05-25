// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::path::PathBuf;

/// Orion runtime configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the Morpheus Unix socket.
    pub socket_path: PathBuf,
    /// Target frame rate in Hz (60 min, 120 target).
    pub frame_rate: u32,
    /// Use Penumbra (TTY) instead of Nova (GPU).
    pub tty: bool,
    /// Disable all network access.
    pub offline: bool,
    /// Skip session restore on startup.
    pub no_restore: bool,
    /// Number of tokio worker threads (0 = num_cpus).
    pub worker_threads: usize,
}

impl Config {
    /// Default configuration.
    #[must_use]
    pub fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            frame_rate: 60,
            tty: false,
            offline: false,
            no_restore: false,
            worker_threads: 0,
        }
    }
}
