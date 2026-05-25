// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![no_main]

use aetheric_stratum::{RopeBackend, Stratum};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }
    let offset = u64::from_le_bytes(data[0..8].try_into().unwrap()) as usize;
    let len = u64::from_le_bytes(data[8..16].try_into().unwrap()) as usize;

    let mut rope = Stratum::new();
    rope.insert(0, "seed text for fuzzing");

    let end = offset.saturating_add(len);
    if end <= rope.len_bytes()
        && rope.len_bytes().is_char_boundary(offset)
        && rope.len_bytes().is_char_boundary(end)
    {
        rope.delete(offset, len);
    }
});
