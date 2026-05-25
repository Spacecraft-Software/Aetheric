// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_ipc_types::{CoreCommand, EditorEvent};
use capnp::message::Builder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_event_roundtrip(c: &mut Criterion) {
    c.bench_function("ipc_event_roundtrip_keypress", |b| {
        b.iter(|| {
            let mut builder = Builder::new_default();
            {
                let mut root = builder.init_root::<aetheric_ipc_types::core_capnp::editor_event::Builder>();
                root.set_key_press("hello world");
            }
            let reader = builder.get_root_as_reader::<aetheric_ipc_types::core_capnp::editor_event::Reader>().unwrap();
            let _ = black_box(EditorEvent::try_from_reader(reader).unwrap());
        })
    });
}

criterion_group!(benches, bench_event_roundtrip);
criterion_main!(benches);
