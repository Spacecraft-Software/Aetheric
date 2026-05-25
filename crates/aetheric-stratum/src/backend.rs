// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

/// Abstract rope backend — implemented by `Stratum`.
pub trait RopeBackend: Send + Sync {
    /// Total length in bytes.
    fn len_bytes(&self) -> usize;

    /// True if the rope is empty.
    fn is_empty(&self) -> bool {
        self.len_bytes() == 0
    }

    /// Total number of lines.
    fn len_lines(&self) -> usize;

    /// Insert `text` at `byte_offset`. `byte_offset` must be a char boundary.
    ///
    /// # Panics
    ///
    /// Panics if `byte_offset` is not a valid UTF-8 char boundary.
    fn insert(&mut self, byte_offset: usize, text: &str);

    /// Delete `len` bytes starting at `byte_offset`. Both must be char boundaries.
    ///
    /// # Panics
    ///
    /// Panics if the range is not valid UTF-8 char boundaries.
    fn delete(&mut self, byte_offset: usize, len: usize);

    /// Convert a byte offset to a line index.
    fn byte_to_line(&self, byte_offset: usize) -> usize;

    /// Convert a line index to the byte offset of its start.
    fn line_to_byte(&self, line: usize) -> usize;

    /// Return the full text as a `String`.
    fn to_string(&self) -> String;
}
