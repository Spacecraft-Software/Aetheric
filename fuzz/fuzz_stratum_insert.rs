// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![no_main]

use aetheric_stratum::{RopeBackend, Stratum};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let offset = u64::from_le_bytes(data[0..8].try_into().unwrap()) as usize;
    let text = std::str::from_utf8(&data[8..]).unwrap_or("");

    let mut rope = Stratum::new();
    rope.insert(0, "seed text for fuzzing");

    if offset <= rope.len_bytes() && rope.len_bytes().is_char_boundary(offset) {
        rope.insert(offset, text);
    }
});
