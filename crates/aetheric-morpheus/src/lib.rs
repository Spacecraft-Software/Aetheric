// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! IPC broker (Morpheus) — Cap'n Proto over Unix socket.
//!
//! All inter-task channels are bounded (default 256) per AE-AGENTS-001.

pub mod bp;
pub mod codec;
pub mod connection;
pub mod dispatch;
pub mod listener;

use std::path::Path;
use tokio::net::UnixListener;
use tracing::info;

/// Morpheus broker state.
#[derive(Debug)]
pub struct MorpheusBroker;

impl MorpheusBroker {
    /// Create a new broker (does not bind yet).
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Bind the Unix socket and start accepting connections.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket cannot be bound or if the parent
    /// directory does not exist.
    ///
    /// # Panics
    ///
    /// This function is currently a stub and will panic.
    pub async fn spawn(self, socket_path: &Path) -> anyhow::Result<()> {
        let _listener = UnixListener::bind(socket_path)?;
        info!(name: "morpheus.bind", "Morpheus bound to {}", socket_path.display());
        todo!("AE-P1-052: implement accept loop and dispatch")
    }
}

impl Default for MorpheusBroker {
    fn default() -> Self {
        Self::new()
    }
}
