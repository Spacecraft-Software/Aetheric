// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

/// Framed Cap'n Proto codec: 4-byte big-endian length prefix + body.
#[derive(Debug, Clone, Default)]
pub struct CapnpCodec {
    inner: LengthDelimitedCodec,
}

impl CapnpCodec {
    /// Create a new codec with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: LengthDelimitedCodec::new(),
        }
    }
}

impl Decoder for CapnpCodec {
    type Item = bytes::BytesMut;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.inner.decode(src)
    }
}

impl Encoder<bytes::Bytes> for CapnpCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: bytes::Bytes, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        self.inner.encode(item, dst)
    }
}
