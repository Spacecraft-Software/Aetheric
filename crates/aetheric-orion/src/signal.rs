// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

/// Install SIGTERM / SIGINT handlers that send `Shutdown` via the provided
/// shutdown channel.
///
/// # Errors
///
/// Returns an error if the signal handler cannot be installed.
pub async fn install_shutdown_handler() -> anyhow::Result<()> {
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = sigterm.recv() => {
            info!(name: "orion.signal.sigterm", "Received SIGTERM, initiating shutdown");
        }
        _ = sigint.recv() => {
            info!(name: "orion.signal.sigint", "Received SIGINT, initiating shutdown");
        }
    }

    Ok(())
}
