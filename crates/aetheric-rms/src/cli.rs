// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use clap::Parser;
use std::path::PathBuf;

/// RMS Microkernel — Aetheric headless server.
#[derive(Debug, Parser)]
#[command(name = "rms")]
#[command(about = "RMS Microkernel — headless text/render/IPC server")]
#[command(version = concat!(env!("CARGO_PKG_VERSION"), " · Spacecraft Software"))]
pub struct Args {
    /// Path to the Morpheus Unix socket.
    #[arg(long, value_name = "PATH")]
    pub socket_path: PathBuf,

    /// Log level (trace, debug, info, warn, error).
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Target frame rate in Hz.
    #[arg(long, default_value_t = 60)]
    pub frame_rate: u32,

    /// Use TTY renderer instead of GPU.
    #[arg(long)]
    pub tty: bool,

    /// Disable network fetches (Boxship, Apogee, cloud AI).
    #[arg(long)]
    pub offline: bool,

    /// Skip session restore.
    #[arg(long)]
    pub no_restore: bool,
}
