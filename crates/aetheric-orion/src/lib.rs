// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Orion — tokio runtime wiring for the RMS Microkernel.

pub mod channels;
pub mod config;
pub mod renderer;
pub mod signal;
pub mod state;
pub mod tasks;

use std::path::PathBuf;

/// RMS Microkernel runtime.
#[derive(Debug)]
pub struct Orion {
    config: config::Config,
}

impl Orion {
    /// Create a new Orion runtime with the given configuration.
    #[must_use]
    pub fn new(config: config::Config) -> Self {
        Self { config }
    }

    /// Run the runtime until shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error if any subsystem fails to start.
    ///
    /// # Panics
    ///
    /// This function is currently a stub and will panic.
    pub async fn run(self) -> anyhow::Result<()> {
        let _ = self.config;
        todo!("AE-P1-076: implement Orion runtime")
    }
}

/// Shared configuration for the RMS microkernel.
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the Morpheus Unix socket.
    pub socket_path: PathBuf,
    /// Target frame rate in Hz.
    pub frame_rate: u32,
    /// Use TTY renderer instead of GPU.
    pub tty: bool,
    /// Disable network fetches.
    pub offline: bool,
    /// Skip session restore.
    pub no_restore: bool,
}
