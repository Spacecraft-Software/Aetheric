// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::path::Path;

fn main() {
    let schema_dir = Path::new("../../schemas");
    capnpc::CompilerCommand::new()
        .src_prefix(schema_dir)
        .file(schema_dir.join("core.capnp"))
        .file(schema_dir.join("orchestration.capnp"))
        .run()
        .expect("capnp compile");

    println!("cargo:rerun-if-changed=../../schemas/core.capnp");
    println!("cargo:rerun-if-changed=../../schemas/orchestration.capnp");
}
