// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::fmt;

/// Configuration for the Lumen logging subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LumenConfig {
    /// Minimum log level (e.g. "info", "debug", "warn", "error", "trace").
    pub level: LogLevel,
    /// Output format.
    pub format: OutputFormat,
}

/// Log verbosity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    /// Trace — most verbose.
    Trace,
    /// Debug.
    Debug,
    /// Info — default.
    #[default]
    Info,
    /// Warning.
    Warn,
    /// Error — least verbose.
    Error,
}

/// Output rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Human-readable coloured lines.
    #[default]
    Human,
    /// Machine-readable JSON lines.
    Json,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl LumenConfig {
    /// Default configuration: info level, human format.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for LumenConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: OutputFormat::Human,
        }
    }
}
