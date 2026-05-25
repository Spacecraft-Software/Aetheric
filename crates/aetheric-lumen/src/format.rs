// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

/// Time formatter producing ISO 8601 UTC Z timestamps: `YYYY-MM-DDTHH:MM:SS.sssZ`.
#[derive(Debug, Clone)]
pub struct JiffTime;

impl FormatTime for JiffTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = jiff::Timestamp::now();
        // `Timestamp::to_string()` produces ISO 8601 UTC Z format.
        write!(w, "{}", now)
    }
}

/// Return a `FormatTime` implementation using `jiff`.
#[must_use]
pub fn jiff_time_format() -> JiffTime {
    JiffTime
}
