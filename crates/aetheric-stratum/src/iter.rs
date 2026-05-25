// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crop::Rope;

/// Iterator over contiguous chunks of a `crop::Rope`.
pub struct ChunkIter<'a> {
    inner: crop::iter::Chunks<'a>,
}

impl<'a> ChunkIter<'a> {
    /// Create a new chunk iterator.
    #[must_use]
    pub fn new(rope: &'a Rope) -> Self {
        Self {
            inner: rope.chunks(),
        }
    }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator over lines of a `crop::Rope`.
pub struct LineIter<'a> {
    inner: crop::iter::Lines<'a>,
}

impl<'a> LineIter<'a> {
    /// Create a new line iterator.
    #[must_use]
    pub fn new(rope: &'a Rope) -> Self {
        Self {
            inner: rope.lines(),
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|slice| slice.to_string())
    }
}
