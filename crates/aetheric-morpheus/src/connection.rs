// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::bp::COMMAND_CHANNEL_BOUND;
use aetheric_ipc_types::{CoreCommand, EditorEvent};
use tokio::net::UnixStream;
use tokio::sync::mpsc;
use tracing::info;

/// Per-connection state: read task, write task, bounded channels.
#[derive(Debug)]
pub struct Connection {
    /// Commands from Majestic to RMS.
    pub cmd_rx: mpsc::Receiver<CoreCommand>,
    /// Events from RMS to Majestic.
    pub event_tx: mpsc::Sender<EditorEvent>,
}

impl Connection {
    /// Split a UnixStream into read/write halves and spawn tasks.
    ///
    /// # Panics
    ///
    /// This function is currently a stub and will panic.
    pub fn spawn(_stream: UnixStream) -> Self {
        let (_cmd_tx, cmd_rx) = mpsc::channel::<CoreCommand>(COMMAND_CHANNEL_BOUND);
        let (event_tx, _event_rx) = mpsc::channel::<EditorEvent>(COMMAND_CHANNEL_BOUND);

        info!(name: "morpheus.connection.spawn", "Connection tasks spawned (stub)");
        todo!("AE-P1-049: implement read + write tasks")
    }
}
