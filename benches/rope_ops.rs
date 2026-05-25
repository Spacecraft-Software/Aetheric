// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_stratum::{RopeBackend, Stratum};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_insert_mid(c: &mut Criterion) {
    let mut rope = Stratum::new();
    rope.insert(0, &"x".repeat(1024 * 1024));

    c.bench_function("stratum_insert_1char_mid_1mib", |b| {
        b.iter(|| {
            let mut r = rope.clone();
            r.insert(512 * 1024, black_box("a"));
        })
    });
}

fn bench_snapshot_clone(c: &mut Criterion) {
    let rope = Stratum::new();

    c.bench_function("stratum_snapshot_clone", |b| {
        b.iter(|| {
            let _ = black_box(rope.clone());
        })
    });
}

criterion_group!(benches, bench_insert_mid, bench_snapshot_clone);
criterion_main!(benches);
