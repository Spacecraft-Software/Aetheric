// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use std::env;
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

    // Generate C header via cbindgen
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::default();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("cbindgen")
        .write_to_file("include/rms_ipc.h");
}
