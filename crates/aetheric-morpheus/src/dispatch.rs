// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use aetheric_ipc_types::{CoreCommand, OrchCommand};
use tracing::info;

/// Route an incoming `CoreCommand` to the appropriate task channel.
pub fn dispatch_core(cmd: CoreCommand) {
    info!(name: "morpheus.dispatch.core", "Dispatching {:?}", cmd);
    todo!("AE-P1-050: route CoreCommand")
}

/// Route an incoming `OrchCommand` to the appropriate task channel.
pub fn dispatch_orch(cmd: OrchCommand) {
    info!(name: "morpheus.dispatch.orch", "Dispatching {:?}", cmd);
    todo!("AE-P1-050: route OrchCommand")
}
