// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

/// Validate that `bytes` is valid UTF-8.
#[must_use]
pub fn utf8_validate(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Check that `offset` falls on a valid UTF-8 character boundary in `text`.
#[must_use]
pub fn byte_offset_is_char_boundary(text: &str, offset: usize) -> bool {
    text.is_char_boundary(offset)
}

/// Convert a UTF-16 code-unit offset to a byte offset in `text`.
///
/// # Panics
///
/// Panics if `utf16_offset` is out of range.
#[must_use]
pub fn utf16_to_byte(text: &str, utf16_offset: usize) -> usize {
    let mut utf16_count = 0;
    for (byte_idx, ch) in text.char_indices() {
        if utf16_count == utf16_offset {
            return byte_idx;
        }
        utf16_count += ch.len_utf16();
    }
    if utf16_count == utf16_offset {
        text.len()
    } else {
        panic!(
            "utf16_offset {utf16_offset} out of range for text length {utf16_count} UTF-16 units"
        )
    }
}

/// Convert a byte offset to a UTF-16 code-unit offset in `text`.
///
/// # Panics
///
/// Panics if `byte_offset` is not a char boundary.
#[must_use]
pub fn byte_to_utf16(text: &str, byte_offset: usize) -> usize {
    assert!(
        text.is_char_boundary(byte_offset),
        "byte_offset {byte_offset} is not a char boundary"
    );
    text[..byte_offset].chars().map(|ch| ch.len_utf16()).sum()
}
