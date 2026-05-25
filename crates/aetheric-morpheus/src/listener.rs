// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::path::Path;
use tokio::net::UnixListener;
use tracing::info;

/// Bind a Unix socket at `path` and return the listener.
///
/// # Errors
///
/// Returns an error if the socket cannot be bound.
pub async fn bind(path: &Path) -> std::io::Result<UnixListener> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    // Remove stale socket file if it exists.
    let _ = tokio::fs::remove_file(path).await;
    let listener = UnixListener::bind(path)?;
    info!(name: "morpheus.listener.bind", "Bound Unix socket at {}", path.display());
    Ok(listener)
}
