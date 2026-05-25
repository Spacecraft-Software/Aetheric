// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

#![no_main]

use aetheric_morpheus::codec::CapnpCodec;
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use tokio_util::codec::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut buf = BytesMut::from(data);
    let mut codec = CapnpCodec::new();
    let _ = codec.decode(&mut buf);
});
