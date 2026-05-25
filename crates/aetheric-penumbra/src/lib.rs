// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Penumbra — terminal (TTY/TUI) front-end (crossterm + ratatui).

pub mod grid;
pub mod input;
pub mod paint;
pub mod palette;
pub mod syntax;
pub mod terminal;

/// Penumbra renderer handle.
#[derive(Debug)]
pub struct Penumbra;

impl Penumbra {
    /// Create a new Penumbra renderer.
    ///
    /// # Errors
    ///
    /// Returns an error if the terminal cannot be initialised.
    ///
    /// # Panics
    ///
    /// This function is currently a stub and will panic.
    pub fn new() -> anyhow::Result<Self> {
        todo!("AE-P3-026: implement Penumbra initialisation")
    }
}
