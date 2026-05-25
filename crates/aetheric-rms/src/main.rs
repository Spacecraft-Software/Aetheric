// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_lumen::config::{LumenConfig, OutputFormat};
use aetheric_orion::config::Config;
use anyhow::Result;
use clap::Parser;

mod cli;

use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    aetheric_lumen::init(LumenConfig {
        level: match args.log_level.as_str() {
            "trace" => aetheric_lumen::config::LogLevel::Trace,
            "debug" => aetheric_lumen::config::LogLevel::Debug,
            "warn" => aetheric_lumen::config::LogLevel::Warn,
            "error" => aetheric_lumen::config::LogLevel::Error,
            _ => aetheric_lumen::config::LogLevel::Info,
        },
        format: OutputFormat::Human,
    })?;

    let config = Config {
        socket_path: args.socket_path,
        frame_rate: args.frame_rate,
        tty: args.tty,
        offline: args.offline,
        no_restore: args.no_restore,
        worker_threads: 0,
    };

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let orion = aetheric_orion::Orion::new(config);
        orion.run().await
    })
}
