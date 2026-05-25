// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Structured logging for Aetheric. ISO 8601 UTC Z timestamps via `jiff`.

pub mod config;
pub mod format;

use config::{LumenConfig, OutputFormat};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialise the global tracing subscriber with the given configuration.
///
/// # Errors
///
/// Returns an error if the subscriber has already been set or if the
/// environment-filter directive is invalid.
pub fn init(cfg: LumenConfig) -> anyhow::Result<()> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(cfg.level.to_string()));

    match cfg.format {
        OutputFormat::Human => {
            let subscriber = tracing_subscriber::fmt()
                .with_timer(format::jiff_time_format())
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
        OutputFormat::Json => {
            let subscriber = tracing_subscriber::fmt()
                .json()
                .with_timer(format::jiff_time_format())
                .with_env_filter(filter)
                .finish();
            tracing::subscriber::set_global_default(subscriber)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jiff_time_contains_z_suffix() {
        let now = jiff::Timestamp::now();
        let s = now.to_string();
        assert!(s.ends_with('Z'), "timestamp {} does not end with Z", s);
    }

    #[test]
    fn lumen_config_default() {
        let cfg = LumenConfig::new();
        assert_eq!(cfg.level, config::LogLevel::Info);
        assert_eq!(cfg.format, OutputFormat::Human);
    }
}
