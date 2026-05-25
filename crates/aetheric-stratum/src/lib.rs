// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Persistent rope (Stratum) — CoW B-tree text storage.

pub mod backend;
pub mod encoding;
pub mod iter;
pub mod rope;
pub mod snapshot;

#[doc(inline)]
pub use backend::RopeBackend;
#[doc(inline)]
pub use rope::Stratum;
#[doc(inline)]
pub use snapshot::SnapshotRegistry;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stratum_insert_preserves_content() {
        let mut s = Stratum::new();
        s.insert(0, "hello world");
        assert_eq!(s.to_string(), "hello world");
    }

    #[test]
    fn stratum_delete_mid() {
        let mut s = Stratum::new();
        s.insert(0, "hello world");
        s.delete(5, 6); // delete " world"
        assert_eq!(s.to_string(), "hello");
    }

    #[test]
    fn stratum_byte_to_line() {
        let mut s = Stratum::new();
        s.insert(0, "a\nb\nc");
        assert_eq!(s.byte_to_line(0), 0);
        assert_eq!(s.byte_to_line(2), 1);
        assert_eq!(s.byte_to_line(4), 2);
    }

    #[test]
    fn stratum_line_to_byte() {
        let mut s = Stratum::new();
        s.insert(0, "a\nb\nc");
        assert_eq!(s.line_to_byte(0), 0);
        assert_eq!(s.line_to_byte(1), 2);
        assert_eq!(s.line_to_byte(2), 4);
    }

    #[test]
    fn stratum_len_bytes() {
        let mut s = Stratum::new();
        s.insert(0, "hello");
        assert_eq!(s.len_bytes(), 5);
    }

    #[test]
    fn stratum_len_lines() {
        let mut s = Stratum::new();
        s.insert(0, "a\nb\nc");
        assert_eq!(s.len_lines(), 3);
    }

    #[test]
    fn snapshot_registry_create_release() {
        let reg = SnapshotRegistry::new();
        let s = Stratum::new();
        let id = reg.create(std::sync::Arc::new(s));
        assert!(id > 0);
        assert!(reg.release(id));
        assert!(!reg.release(id));
    }

    #[test]
    fn encoding_utf8_validate() {
        assert!(encoding::utf8_validate(b"hello"));
        assert!(!encoding::utf8_validate(&[0x80, 0x81]));
    }

    #[test]
    fn encoding_byte_offset_is_char_boundary() {
        assert!(encoding::byte_offset_is_char_boundary("hello", 2));
        assert!(!encoding::byte_offset_is_char_boundary("héllo", 2)); // é is 2 bytes
    }
}
