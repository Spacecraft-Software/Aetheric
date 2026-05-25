// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::backend::RopeBackend;
use std::sync::Arc;

/// Persistent rope backed by `crop::Rope`.
#[derive(Debug, Clone)]
pub struct Stratum(crop::Rope);

impl Stratum {
    /// Create a new empty Stratum.
    #[must_use]
    pub fn new() -> Self {
        Self(crop::Rope::new())
    }

    /// Create from a `String`, validating UTF-8.
    ///
    /// # Errors
    ///
    /// Returns the input string unchanged if it is not valid UTF-8.
    pub fn from_string(s: String) -> Result<Self, String> {
        // crop::Rope requires valid UTF-8
        let _ = std::str::from_utf8(s.as_bytes()).map_err(|_| s.clone())?;
        Ok(Self(crop::Rope::from(s)))
    }

    /// Return an `Arc<Stratum>` for cheap cloning.
    #[must_use]
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Default for Stratum {
    fn default() -> Self {
        Self::new()
    }
}

impl RopeBackend for Stratum {
    fn len_bytes(&self) -> usize {
        self.0.byte_len()
    }

    fn len_lines(&self) -> usize {
        self.0.line_len()
    }

    fn insert(&mut self, byte_offset: usize, text: &str) {
        assert!(
            self.0.is_char_boundary(byte_offset),
            "byte_offset must be a char boundary"
        );
        self.0.insert(byte_offset, text);
    }

    fn delete(&mut self, byte_offset: usize, len: usize) {
        let end = byte_offset + len;
        assert!(
            self.0.is_char_boundary(byte_offset) && self.0.is_char_boundary(end),
            "delete range must be valid char boundaries"
        );
        self.0.delete(byte_offset..end);
    }

    fn byte_to_line(&self, byte_offset: usize) -> usize {
        self.0.line_of_byte(byte_offset)
    }

    fn line_to_byte(&self, line: usize) -> usize {
        self.0.byte_of_line(line)
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
