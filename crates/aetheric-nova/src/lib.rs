// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Nova — GPU display engine (wgpu + cosmic-text + glyphon).

pub mod font;
pub mod glyph;
pub mod overlay;
pub mod renderer;
pub mod surface;
pub mod syntax;
pub mod text_layout;

/// Nova renderer handle.
#[derive(Debug)]
pub struct Nova;

impl Nova {
    /// Create a new Nova renderer.
    ///
    /// # Errors
    ///
    /// Returns an error if wgpu cannot find a suitable adapter.
    ///
    /// # Panics
    ///
    /// This function is currently a stub and will panic.
    pub fn new() -> anyhow::Result<Self> {
        todo!("AE-P2-001: implement Nova initialisation")
    }
}
