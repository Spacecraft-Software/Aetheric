---
title: "Aetheric — Implementation TODO"
author: "Mohamed Hammad <Mohamed.Hammad@SpacecraftSoftware.org>"
date: 2026-05-25
version: "2.7"
document-id: AE-TODO-001
references: "AE-PRD-MASTER-001 v2.7, AE-IMPL-001 v2.7, guile-functional-concurrent (skill)"
license: AGPL-3.0-or-later
tags: [todo, aetheric, implementation, roadmap, spacecraft-software]
---

# Aetheric — Implementation TODO

<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Copyright (c) 2026 Mohamed Hammad & Spacecraft Software -->
<!-- References: AE-PRD-MASTER-001 v2.7, AE-IMPL-001 v2.7, guile-functional-concurrent (skill) -->

**Version:** 2.7 | **Author:** Mohamed Hammad <Mohamed.Hammad@SpacecraftSoftware.org>  
**Date:** 2026-05-25 | **License:** AGPL-3.0-or-later

| Symbol | Meaning |
|---|---|
| `[ ]` | Not started |
| `[~]` | In progress |
| `[x]` | Complete |
| `[!]` | Blocked / needs decision |

> **Tracks.** Editor phases 1–5 and AI phases AI-1…AI-4 run in parallel. AI-1 begins
> alongside Phase 3. The RMS Microkernel and its two renderers (Nova GPU, Penumbra
> TTY) are Rust; Majestic is Guile, built per the conventions in AE-IMPL-001 §7.

---

## Phase 1 — Foundation (2026-Q3)

> Workspace, schemas, Stratum, Morpheus, SingleScrew (SS), Orion skeleton, RMS Microkernel binary.

### P1 — Workspace & Toolchain

- [x] **AE-P1-001** Create `aetheric/` repo; `git init`; add `README.md`, `NOTICE.md`, `CONTRIBUTING.md`, `LICENSE` (AGPL-3.0-or-later).
- [x] **AE-P1-002** Author `rust-toolchain.toml` pinning stable + rustfmt, clippy, rust-src, llvm-tools-preview.
- [x] **AE-P1-003** Author workspace `Cargo.toml`: 9 member crates (incl. `aetheric-nova`, `aetheric-penumbra`); `resolver = "2"`.
- [x] **AE-P1-004** Author `deny.toml`: AGPL-3.0-or-later allow-list; `[bans]` blocks tokio in `aetheric-single-screw`.
- [x] **AE-P1-005** Author `clippy.toml` per AE-IMPL-001 §10.1.
- [x] **AE-P1-006** Add SPDX header `// SPDX-License-Identifier: AGPL-3.0-or-later` to every `.rs`/`.scm`; CI grep-check.
- [ ] **AE-P1-007** Configure Ed25519 SSH commit signing; register key on GitHub; verify first commit shows "Verified".
- [~] **AE-P1-008** Scaffold CI workflow: fmt, clippy, build, test, cargo-deny, miri, WCAG, SPDX, commit-signing.
- [ ] **AE-P1-009** Add `docs/spacecraft-software-exceptions.md` — §3.1 Majestic (Guile) process exception; review 2027-06-01.
- [x] **AE-P1-010** Add `AGENTS.md` (v2.7) to repo root.
- [x] **AE-P1-011** Create `schemas/`; run `capnp id`; paste unique file ID into `schemas/core.capnp`; commit.

### P1 — Constellation Schemas

- [x] **AE-P1-012** Author `schemas/core.capnp` in full per AE-IMPL-001 §6.1 (EditorEvent 8, CoreCommand 11).
- [x] **AE-P1-013** Author `schemas/orchestration.capnp` per §6.2: Celestial, Nexus, AiStreamRequest, AiStreamChunk, OrchCommand, OrchEvent.
- [x] **AE-P1-014** Verify both compile: `capnp compile -o/dev/null schemas/*.capnp`; add to CI.

### P1 — aetheric-ipc-types

- [x] **AE-P1-015** Scaffold `Cargo.toml`: `capnp 0.20`; build-dep `capnpc 0.20`; `#![deny(unsafe_code)]`.
- [x] **AE-P1-016** Author `build.rs`: compile core + orchestration; `rerun-if-changed`.
- [x] **AE-P1-017** Author `src/events.rs`: `pub enum EditorEvent` (8 variants, owned).
- [x] **AE-P1-018** Author `src/commands.rs`: `CoreCommand` (11); `OrchCommand`; `OrchEvent` (incl. AI streaming).
- [x] **AE-P1-019** Author `src/error.rs`: `pub enum IpcError { Capnp, Io, Disconnected, Version }`.
- [x] **AE-P1-020** Author `src/convert.rs`: `EditorEvent::try_from(reader)`; `CoreCommand::to_builder()`.
- [x] **AE-P1-021** Author `src/schema.rs`: include generated capnp Rust.
- [x] **AE-P1-022** Author `src/lib.rs`: re-exports; module declarations.
- [x] **AE-P1-023** Round-trip unit tests for every EditorEvent variant.
- [x] **AE-P1-024** `cargo test -p aetheric-ipc-types`; all pass.

### P1 — aetheric-stratum

- [x] **AE-P1-025** Scaffold `Cargo.toml`: `crop 0.4`, `arc-swap 1.7`, `dashmap 6`; dev: `criterion`, `proptest`.
- [x] **AE-P1-026** Author `src/backend.rs`: `pub trait RopeBackend: Send + Sync`.
- [x] **AE-P1-027** Author `src/rope.rs`: `pub struct Stratum(crop::Rope)`; impl RopeBackend.
- [x] **AE-P1-028** Author `src/encoding.rs`: `utf8_validate()`, `byte_offset_is_char_boundary()`, `utf16_to_byte()`, `byte_to_utf16()`.
- [x] **AE-P1-029** Author `src/snapshot.rs`: `SnapshotRegistry(DashMap<u32, Arc<Stratum>>)`; create/release/get.
- [x] **AE-P1-030** Author `src/iter.rs`: `ChunkIter`, `LineIter`.
- [x] **AE-P1-031** Author `src/lib.rs`: `#![deny(unsafe_code)]`; re-exports.
- [~] **AE-P1-032** proptest: insert/delete round-trips preserve content.
- [~] **AE-P1-033** proptest: all `byte_to_line`/`line_to_byte` offsets are valid char boundaries.
- [~] **AE-P1-034** criterion: 1-char insert mid-point of 1 MiB rope; p50 < 1 ms.
- [~] **AE-P1-035** criterion: snapshot clone p50 < 1 µs.
- [~] **AE-P1-036** Fuzz `fuzz_stratum_insert.rs`: arbitrary (offset, content) — no panic.
- [~] **AE-P1-037** Fuzz `fuzz_stratum_delete.rs`: arbitrary (offset, len) — no panic.
- [~] **AE-P1-038** `cargo miri test -p aetheric-stratum`; fix findings.
- [x] **AE-P1-039** `cargo test -p aetheric-stratum`; all pass.

### P1 — aetheric-lumen

- [x] **AE-P1-040** Scaffold `Cargo.toml`: `tracing 0.1`, `tracing-subscriber 0.3`, `jiff 0.1`.
- [x] **AE-P1-041** Author `src/config.rs`: `LumenConfig { level, format }`; `enum OutputFormat { Human, Json }`.
- [x] **AE-P1-042** Author `src/format.rs`: `YYYY-MM-DDTHH:MM:SS.sssZ` via `jiff::Timestamp::now()`.
- [x] **AE-P1-043** Author `src/lib.rs`: `pub fn init(cfg)`.
- [ ] **AE-P1-044** Test: JSON output contains Z-suffix timestamp.
- [x] **AE-P1-045** `cargo test -p aetheric-lumen`; all pass.

### P1 — aetheric-morpheus `[#![deny(unsafe_code)]]`

- [~] **AE-P1-046** Scaffold: `tokio 1`, `tokio-util 0.7`, `capnp 0.20`, `aetheric-ipc-types`, `tracing 0.1`.
- [~] **AE-P1-047** Author `src/bp.rs`: `COMMAND_CHANNEL_BOUND = 256` (M-DOCUMENTED-MAGIC).
- [~] **AE-P1-048** Author `src/codec.rs`: `LengthDelimitedCodec`; encode/decode.
- [~] **AE-P1-049** Author `src/connection.rs`: read + write tasks; bounded channels.
- [~] **AE-P1-050** Author `src/dispatch.rs`: route CoreCommand **and OrchCommand**.
- [~] **AE-P1-051** Author `src/listener.rs`: bind UnixListener; accept loop.
- [~] **AE-P1-052** Author `src/lib.rs`: `MorpheusBroker`; `spawn()`.
- [ ] **AE-P1-053** Integration test: mock Majestic `openBuffer` → `bufferOpened`; round-trip < 2 ms.
- [ ] **AE-P1-054** Fuzz `fuzz_morpheus_decoder.rs`.
- [ ] **AE-P1-055** Verify `#![deny(unsafe_code)]`; CI job confirms flag.
- [ ] **AE-P1-056** `cargo test -p aetheric-morpheus`; all pass.

### P1 — aetheric-single-screw (SS)

- [~] **AE-P1-057** Scaffold: `crate-type=["cdylib","rlib"]`; `capnp`, `libc`, `tracing`, `jiff`, `anyhow`; build: `capnpc`, `cbindgen`; **NO tokio**.
- [~] **AE-P1-058** Author `build.rs`: capnpc compile; cbindgen → `include/rms_ipc.h`.
- [~] **AE-P1-059** Author `src/error.rs`: thread-local `LAST_CODE`/`LAST_MSG`; set/clear/last_code/last_message_ptr.
- [~] **AE-P1-060** Author `src/proto.rs`: owned `Event`/`Cmd`; `read_event()`; `write_cmd()`.
- [~] **AE-P1-061** Author `src/conn.rs`: `Conn`; reader thread; POSIX pipe pair (fd behind `rms_fd()`); `impl Drop`.
- [~] **AE-P1-062** Document both unsafe sites with `// SAFETY:` (FFI+catch_unwind; pipe/read/write).
- [~] **AE-P1-063** Author `src/lib.rs`: `ffi_guard!`; 18 `extern "C"` fns (`RmsConn*`/`RmsMsg*`/`RmsCmd*`).
- [ ] **AE-P1-064** Implement all 18 ABI fns per `include/rms_ipc.h`.
- [ ] **AE-P1-065** Test: panic inside extern "C" → `rms_last_error()` = RMS_ERR_PANIC; no crash.
- [ ] **AE-P1-066** Author `guile/rms-ipc.scm`: `(define-module (aetheric rms-ipc) …)`; foreign-library bindings; **`spawn-rms-reader`** fiber-aware reader + `drain-events` (AE-IMPL §7.3).
- [ ] **AE-P1-067** Author `tests/ss_bridge_smoke.scm`: `rms-version`; connect; `open-buffer`; drain; assert `bufferOpened`.
- [ ] **AE-P1-068** `cargo miri test -p aetheric-single-screw`; both sites pass.
- [ ] **AE-P1-069** `cargo build --release -p aetheric-single-screw`; verify `librms_ipc.so`.
- [ ] **AE-P1-070** `guile tests/ss_bridge_smoke.scm`; all assertions pass.

### P1 — aetheric-orion (skeleton)

- [~] **AE-P1-071** Scaffold: `tokio 1`, `arc-swap 1.7`, `aetheric-{stratum,morpheus,ipc-types,lumen}`.
- [~] **AE-P1-072** Author `src/renderer.rs`: `pub trait Renderer { present(...); poll_input(...) }` (stub).
- [~] **AE-P1-073** Author `src/state.rs`: `SharedState { rope: ArcSwap<Arc<Stratum>>, snapshots, render_tx }`.
- [~] **AE-P1-074** Author `src/channels.rs`: named bounds (256 / 4 / 64).
- [~] **AE-P1-075** Author `src/tasks.rs`: mutator + morpheus reader/writer stubs.
- [~] **AE-P1-076** Author `src/{runtime,signal,config,lib}.rs`: `Orion::new(cfg)`, `run()`.
- [ ] **AE-P1-077** Integration test: `InsertText` → `RequestSnapshot` → `SnapshotReady`; verify content.
- [ ] **AE-P1-078** `cargo test -p aetheric-orion`; all pass.

### P1 — aetheric-rms (RMS Microkernel binary)

- [~] **AE-P1-079** Scaffold: `clap 4`, `anyhow 1`, `aetheric-orion`, `aetheric-lumen`.
- [~] **AE-P1-080** Author `src/cli.rs`: `Args` (`--socket-path`, `--log-level`, `--frame-rate`, `--tty`, `--offline`, `--no-restore`); `--version` attribution (Standard §13.2).
- [~] **AE-P1-081** Author `src/main.rs`: parse Args; `lumen::init()`; build Orion config; `orion.run()`.
- [ ] **AE-P1-082** Integration test: `--help` output; `--version` contains "Spacecraft Software".
- [ ] **AE-P1-083** `cargo build --release -p aetheric-rms`; `rms` binary produced.

### P1 — Validation

- [ ] **AE-P1-084** Full CI: fmt, clippy, build, test, cargo-deny, miri — green.
- [ ] **AE-P1-085** 1-hour cargo-fuzz soak on stratum targets; zero crashes.
- [ ] **AE-P1-086** Headless end-to-end: `rms` + `guile tests/ss_bridge_smoke.scm`.
- [ ] **AE-P1-087** PRD §16 Phase 1 metrics: rope insert < 1 ms; snapshot clone < 1 µs; IPC round-trip < 2 ms.
- [ ] **AE-P1-088** `CHANGELOG.md` entry; tag `v0.1.0-alpha` (signed).

---

## Phase 2 — Rendering + Keymaps (2026-Q3–Q4)

> Nova GPU pipeline, Tree-sitter, the functional core + Fibers/CSP event loop, Astrolabe first-run, Oracle, Architect.

### P2 — aetheric-nova

- [ ] **AE-P2-001** Add deps: `wgpu 0.20`, `cosmic-text 0.12`, `glyphon 0.6`, `fontdb 0.21`, `tree-sitter 0.22` + grammars (rust, markdown, scheme, toml). `impl Renderer`.
- [ ] **AE-P2-002** Author `src/font.rs`: bundled TTFs from `assets/fonts/`; system via `XDG_DATA_DIRS`.
- [ ] **AE-P2-003** Author `src/surface.rs`: wgpu init; Vulkan→Metal→DX12→GLES.
- [ ] **AE-P2-004** Author `src/glyph.rs`: `glyphon::TextAtlas`; cache invalidation on resize/DPI.
- [ ] **AE-P2-005** Author `src/text_layout.rs`: `cosmic_text::FontSystem`; bidi/RTL.
- [ ] **AE-P2-006** Author `src/syntax.rs`: tree-sitter scope → Spectrum colour.
- [ ] **AE-P2-007** Author `src/overlay.rs`: frame-time histogram; Void Navy; toggleable.
- [ ] **AE-P2-008** Author `src/renderer.rs`: acquire→shape→upload→render→present; timestamp queries.
- [ ] **AE-P2-009** Author `src/lib.rs`: `pub struct Nova`; `impl Renderer`.
- [ ] **AE-P2-010** Screenshot regression: 10-line Rust snippet; pixel diff < 1% vs golden.
- [ ] **AE-P2-011** criterion `frame_latency.rs`: headless wgpu; p95 < 16 ms.
- [ ] **AE-P2-012** WCAG: all overlay colour pairs ≥ 4.5:1.

### P2 — Orion ↔ Nova Integration

- [ ] **AE-P2-013** Wire `render_tx` into `SharedState`; emit repaint after Stratum mutations.
- [ ] **AE-P2-014** Implement `spawn_render()`; select Nova via Renderer trait; wire `ToggleOverlay`/`SetViewport`.
- [ ] **AE-P2-015** Implement `spawn_treesitter_parse()`: incremental parse → token table; Nova reads atomically.

### P2 — Majestic Functional Core + Fibers Event Loop (per AE-IMPL §7)

- [ ] **AE-P2-016** Author `guile/aetheric/core.scm`: immutable state + **pure `apply-event`** via `(ice-9 match)`; SRFI-9 buffer/keymap records with functional updaters; no `set!`.
- [ ] **AE-P2-017** Author `guile/main.scm`: `run-fibers` single entry; `spawn-rms-reader`; **`event-loop`** as a `choice-operation` select over rms/lsp/ai/frame-tick channels, folding each event through `apply-event` in tail position.
- [ ] **AE-P2-018** Define context parameters: `current-conn`, `current-buffer`, `current-keymap`, `current-theme` (`make-parameter`).
- [ ] **AE-P2-019** Author `scripts/guile-lint.scm`: reject `set!` in core, `define-macro` anywhere, `touch`/`join-thread`/blocking read inside a fiber, stray `pk`.
- [ ] **AE-P2-020** Author `tests/core_transitions.scm` (SRFI-64): pure `apply-event` cases — no editor/GPU needed.
- [ ] **AE-P2-021** Author `tests/fibers_suite.scm`: channel select, rendezvous backpressure, per-fiber `guard` isolation.

### P2 — Astrolabe: First-Run Keybinding Selector

- [ ] **AE-P2-022** Author `guile/aetheric/astrolabe.scm`: **persistent prefix-tree** keymaps (functional rebind); `keymap-bind`, `keymap-lookup`.
- [ ] **AE-P2-023** Implement `first-run-profile-select`: prompt `[E]macs [V]im [C]UA`; store in `profile.scm`.
- [ ] **AE-P2-024** Emacs profile: `C-x C-s`, `M-x`, `C-h`, `C-x C-f`, `C-x C-c`.
- [ ] **AE-P2-025** Vim profile: modal state machine; hjkl; w/b/e; d/y/p; `:w :q`; text objects.
- [ ] **AE-P2-026** CUA profile: Ctrl+S/O/Z/X/C/V/A/F/W; arrows; selection.
- [ ] **AE-P2-027** `astrolabe/switch-profile`: runtime switch (minor mode = keymap→keymap function).
- [ ] **AE-P2-028** `astrolabe-dispatch-key`: buffer-local keymap first, global fallback.
- [ ] **AE-P2-029** Test: first-run on clean config → prompt → choice stored.
- [ ] **AE-P2-030** Test: all three profiles functional; layering verified (CUA base + Vim text-objects).

### P2 — Oracle & Architect

- [ ] **AE-P2-031** Author `guile/aetheric/oracle.scm`: `describe-key/function/variable/mode`, `apropos`, `describe-bindings`; reads live env.
- [ ] **AE-P2-032** `oracle-buffer`: help buffer with live Guile hyperlinks.
- [ ] **AE-P2-033** Author `guile/aetheric/architect.scm`: `architect-buffer`, `architect-eval`; history, multi-line, error display.
- [ ] **AE-P2-034** Test: evaluate `(buffer-open conn "/tmp/x")` at REPL without restart; describe-* finds it.

### P2 — Validation

- [ ] **AE-P2-035** End-to-end: type 10 chars → rope → Nova render; frame < 16 ms.
- [ ] **AE-P2-036** ThreadSanitizer on RMS: zero races.
- [ ] **AE-P2-037** `guile-lint.scm` clean; `guild compile` warning-free.
- [ ] **AE-P2-038** `CHANGELOG.md` entry; tag `v0.2.0-alpha`.

---

## Phase 3 — Ecosystem (2026-Q4–2027-Q1)

> Editor API, LSP (fiber pairs), session, Halo, Construct, Nomadic, Nexus v1, Boxship v1, **Penumbra TTY front-end**.

### P3 — (aetheric editor) API

- [ ] **AE-P3-001** Author `guile/aetheric/editor.scm`: buffer-open/close/insert/delete, buffer-local-set/ref, current-buffer, with-buffer.
- [ ] **AE-P3-002** Hook system: `hook-add`, `hook-remove`, `hook-run` (SRFI-1 `fold`); standard hooks.
- [ ] **AE-P3-003** Test: extension registers `after-open` hook; fired; hot-unload removes it.

### P3 — (aetheric lsp) — fiber pair per server

- [ ] **AE-P3-004** Author `guile/aetheric/lsp.scm`: `spawn-lsp-server` (reader/writer fiber pair); start/stop, completion, hover, goto-def, rename, diagnostics.
- [ ] **AE-P3-005** initialize/initialized handshake; capability detection.
- [ ] **AE-P3-006** `publishDiagnostics` → diagnostics pipeline (staged fibers / SRFI-171) → single `setDiagnostics`.
- [ ] **AE-P3-007** Add grammars: python, javascript, typescript, c, json.
- [ ] **AE-P3-008** Integration test: rust-analyzer (or mock); diagnostics in overlay.

### P3 — (aetheric ephemeris), Halo, Construct

- [ ] **AE-P3-009** Author `ephemeris.scm`: XDG paths; `session-save/restore` (lazy config via `delay`/`force`); `--no-restore`.
- [ ] **AE-P3-010** Author `halo.scm`: `halo-require` (fresh namespace, `(on-load)`, per-call `guard`); `halo-unload-module` (`(on-unload)`).
- [ ] **AE-P3-011** Test: crashing extension → Majestic continues; unload → hook stops firing.
- [ ] **AE-P3-012** Author `construct.scm`: `construct-mode`, open-dir, navigate, copy/move/delete/mark.
- [ ] **AE-P3-013** Test: open `~/`; navigate; open file; verify buffer-id.

### P3 — Nomadic Terminal Buffer

- [ ] **AE-P3-014** Implement `nomadic-mode`: embedded PTY terminal via Guile POSIX subprocess; PTY I/O on fibers.
- [ ] **AE-P3-015** Test: open Nomadic; `echo hello`; appears in buffer.
- [ ] **AE-P3-016** Document in AGENTS.md: "Nomadic = embedded PTY buffer (SS Nomadic tender); distinct from Penumbra (the TTY front-end)." Also the Seraph shell sandbox (AE-AI-009).

### P3 — Nexus Git (v1)

- [ ] **AE-P3-017** Author `nexus.scm`: status, stage-hunk, unstage-hunk, commit, log (each git op a fiber).
- [ ] **AE-P3-018** `OrchCommand::NexusStatusRequest` in Orion (git2 task) → `NexusStatusResult`.
- [ ] **AE-P3-019** Status buffer with Spectrum colours (Radium Green staged, Red Oxide unstaged).
- [ ] **AE-P3-020** Commit buffer with `C-c C-c` (all profiles); add `git2 = "0.19"` to orion.
- [ ] **AE-P3-021** Test: status correct on a test repo.

### P3 — Boxship Package Manager (v1)

- [ ] **AE-P3-022** Author `boxship.scm`: `boxship-install/update/remove/list/audit`; `boxship-require`.
- [ ] **AE-P3-023** Registry fetch + Apogee Ed25519 sig + SHA-256 content hash (all three verified pre-execution).
- [ ] **AE-P3-024** `--offline` hard-disables fetches.
- [ ] **AE-P3-025** Test: mock registry; install; verify hash + sig checks run.

### P3 — Penumbra TTY/TUI Front-End

- [ ] **AE-P3-026** Scaffold `aetheric-penumbra`: `crossterm 0.28`, `ratatui 0.28`, `tree-sitter`; `impl Renderer`.
- [ ] **AE-P3-027** Author `src/terminal.rs`: raw mode, alternate screen, resize signals; restore on drop.
- [ ] **AE-P3-028** Author `src/grid.rs`: cell grid; viewport → ratatui buffer.
- [ ] **AE-P3-029** Author `src/paint.rs`: diff-based cell repaint; minimal escape output.
- [ ] **AE-P3-030** Author `src/palette.rs`: Spectrum → 24-bit truecolor; 256/16-colour fallback.
- [ ] **AE-P3-031** Author `src/input.rs`: crossterm keys → `EditorEvent::KeyPress`.
- [ ] **AE-P3-032** Author `src/syntax.rs`: tree-sitter scope → terminal styles.
- [ ] **AE-P3-033** Wire renderer selection in `aetheric-rms`: `--tty` flag + auto-detect (no display server).
- [ ] **AE-P3-034** Author `tests/renderer_parity.rs`: Nova vs Penumbra produce identical logical layout for a snapshot.
- [ ] **AE-P3-035** WCAG: Penumbra truecolor palette ≥ 4.5:1; fallback documented.
- [ ] **AE-P3-036** Live test: `aetheric --tty` over SSH on a headless host; edit + save round-trip.

### P3 — Validation

- [ ] **AE-P3-037** Full CI incl. Guile tests + renderer parity; green.
- [ ] **AE-P3-038** Live: open Rust file; LSP diagnostics < 2 s; goto-def works.
- [ ] **AE-P3-039** Live: Nexus status; stage; commit; git log shows commit.
- [ ] **AE-P3-040** Live: session save → restart → restore; Nomadic opens; Penumbra session works.
- [ ] **AE-P3-041** `cargo bench` — §16 metrics within targets; no regression.
- [ ] **AE-P3-042** `CHANGELOG.md` entry; tag `v0.3.0-alpha`.

---

## Phase 4 — Celestial & Ecosystem Launch (2027-Q1–Q2)

> Full Celestial, Nexus advanced, Spectrum CI gate, Nix (flake + NixOS + Home Manager), Boxship registry, public alpha.

### P4 — orchestration.capnp Finalisation

- [ ] **AE-P4-001** Finalise `orchestration.capnp`: all Celestial/Nexus/AI variants; verify compile.
- [ ] **AE-P4-002** Add `OrchCommand`/`OrchEvent` handling to Morpheus dispatch.

### P4 — Celestial

- [ ] **AE-P4-003** Add `tree-sitter-org` (lang-org); map scopes to Spectrum.
- [ ] **AE-P4-004** `celestial-mode` core: stars headings; `TAB`/`S-TAB` fold; subtree ops.
- [ ] **AE-P4-005** Task state cycling: `TODO → IN-PROGRESS → DONE → (nil)`.
- [ ] **AE-P4-006** Priorities (A/B/C): `C-c ,`.
- [ ] **AE-P4-007** Deadlines/scheduled; Org-compatible `DEADLINE:`/`SCHEDULED:`.
- [ ] **AE-P4-008** Agenda buffer (day/week/month) via `CelestialAgendaQuery`; background fiber.
- [ ] **AE-P4-009** Quick-capture templates: `C-c c`.
- [ ] **AE-P4-010** Tables: keyboard alignment; `TAB` nav; Guile formulas.
- [ ] **AE-P4-011** Literate source blocks: `#+BEGIN_SRC … #+END_SRC`; `C-c C-c` via Architect/subprocess.
- [ ] **AE-P4-012** `[[Link]]` wiki-links; backlink index (background fiber); backlink buffer.
- [ ] **AE-P4-013** Export: `CelestialExportRequest` → Orion task → PDF (LaTeX)/HTML/Markdown; `ExportDoneResult`.
- [ ] **AE-P4-014** Org-mode import: parse `.org` headings/properties/timestamps.
- [ ] **AE-P4-015** Test: round-trip — create file (headings/tasks/table/source) → export HTML → verify.
- [ ] **AE-P4-016** Test: Org import corpus; structure + states preserved.
- [ ] **AE-P4-017** Perf: agenda build 100 tasks < 200 ms (§16).

### P4 — Nexus Advanced

- [ ] **AE-P4-018** Interactive rebase buffer: pick/squash/reword/drop; `C-c C-c`.
- [ ] **AE-P4-019** Inline blame: `nexus-blame-toggle`.
- [ ] **AE-P4-020** Branch management: create/checkout/merge/delete; branch graph.
- [ ] **AE-P4-021** Stash: push/pop; stash list buffer.
- [ ] **AE-P4-022** Celestial ↔ Nexus: `nexus-link-task`; auto-close TODO from commit message.
- [ ] **AE-P4-023** Perf: `git status` 10k-file repo < 500 ms (§16).

### P4 — Spectrum WCAG Gate

- [ ] **AE-P4-024** Publish `assets/themes/void.scm` — all scope → token mappings.
- [ ] **AE-P4-025** Extend `scripts/wcag_contrast.py` for Celestial/Nexus/AI/Penumbra pairs.
- [ ] **AE-P4-026** CI gate: any pair < 4.5:1 blocks PR (GPU and TTY).

### P4 — Nix Packaging

- [ ] **AE-P4-027** Author `nix/flake.nix`: packages `rms`, `aetheric-majestic`, `librms_ipc`; crane Rust build + Guile derivation (`guild compile`); dev shell.
- [ ] **AE-P4-028** Author `nix/nixos-module.nix`: `programs.aetheric.enable`; installs both processes + bridge + fonts.
- [ ] **AE-P4-029** Author `nix/hm-module.nix`: `programs.aetheric` (Home Manager) — typed options (keybindingProfile, theme, extensions, ai, extraConfig); generate `init.scm` + `profile.scm`.
- [ ] **AE-P4-030** Add `nix flake check` to CI (flake + NixOS + HM module evaluate).
- [ ] **AE-P4-031** Test: HM module on a sample home; `init.scm`/`profile.scm` materialise correctly.

### P4 — Apogee & Public Alpha

- [ ] **AE-P4-032** Generate Apogee Ed25519 key pair (`minisign -G`); commit public key.
- [ ] **AE-P4-033** Release CI: `v*` tag → Tier 1 tarballs → sign → upload.
- [ ] **AE-P4-034** `README.md`: overview, install (incl. Nix HM + `--tty`), config guide, keymap reference.
- [ ] **AE-P4-035** `docs/CREDITS.md`: crop, wgpu, cosmic-text, glyphon, crossterm, ratatui, Tree-sitter, Cap'n Proto, GNU Guile, guile-fibers, libgit2.
- [ ] **AE-P4-036** Full CI green; tag `v0.4.0-alpha`; publish signed tarballs; announce.

---

## Phase 5 — Production Hardening (2027-Q2–Q3)

> 24h fuzz soaks, ThreadSanitizer/Miri clean, §15 audit, OQ decisions, v1.0.

### P5 — Deep Fuzzing & Sanitizers

- [ ] **AE-P5-001** 24h soak `fuzz_stratum_insert`: zero crashes, zero Miri violations. Required for v1.0.
- [ ] **AE-P5-002** 24h soak `fuzz_stratum_delete`: zero crashes.
- [ ] **AE-P5-003** 24h soak `fuzz_morpheus_decoder`: zero crashes.
- [ ] **AE-P5-004** Commit fuzz corpus to `fuzz/corpus/`.
- [ ] **AE-P5-005** Full ThreadSanitizer: `--workspace`; zero races.
- [ ] **AE-P5-006** Full Miri: `cargo miri test --workspace`; zero UB.
- [ ] **AE-P5-007** Nightly TSan CI job on `main`.

### P5 — Performance Final

- [ ] **AE-P5-008** Measure all §16 metrics (Nova + Penumbra) on reference hardware; `docs/perf-report-v1.0.md`.
- [ ] **AE-P5-009** Evaluate OQ-06: static `libguile.a` vs. system Guile; document.
- [ ] **AE-P5-010** Evaluate OQ-09: crop vs. ropey after fuzz; document.

### P5 — §15 Compliance Audit

- [ ] **AE-P5-011** Every `.rs`/`.scm`: SPDX header present.
- [ ] **AE-P5-012** `--version` attribution matches Standard §13.2.
- [ ] **AE-P5-013** `spacecraft-software-exceptions.md`: §3.1 complete; review 2027-06-01.
- [ ] **AE-P5-014** All §15 rows have passing CI jobs.
- [ ] **AE-P5-015** DCO `Signed-off-by` on all `main` commits.
- [ ] **AE-P5-016** All `main` commits show "Verified".
- [ ] **AE-P5-017** `cargo deny check`: zero advisories/licence violations.
- [ ] **AE-P5-018** WCAG: all themes ≥ 4.5:1 (GPU + TTY); `wcag_contrast.py` clean.
- [ ] **AE-P5-019** `guile-lint.scm` + `guild compile` warning-free across all `.scm`.
- [ ] **AE-P5-020** AGENTS.md in sync with AE-PRD-MASTER-001 v2.7 final.

### P5 — OQ Decisions

- [ ] **AE-P5-021** OQ-01: Guile Windows Tier 1 → Steel fallback RFC if failed 2 quarters.
- [ ] **AE-P5-022** OQ-02: Bidirectional streaming RPC for AI → extend `orchestration.capnp` if needed.
- [ ] **AE-P5-023** OQ-03: Multi-window support → document post-v1.0 or implement.
- [ ] **AE-P5-024** OQ-04: Boxship content-addressed store → document.
- [ ] **AE-P5-025** OQ-05: Penumbra inline images (Kitty/Sixel) → document post-v1.0.
- [ ] **AE-P5-026** OQ-07: AI cloud vs. local default → implement chosen path.
- [ ] **AE-P5-027** OQ-08: Nexus git2 vs. git CLI → document.

### P5 — v1.0 Release

- [ ] **AE-P5-028** Bump all crate versions to `1.0.0`.
- [ ] **AE-P5-029** Final `CHANGELOG.md` v1.0 entry.
- [ ] **AE-P5-030** Build Tier 1 tarballs for all 5 target triples.
- [ ] **AE-P5-031** Sign all tarballs with Apogee key; generate `.minisig`.
- [ ] **AE-P5-032** Tag `v1.0.0` (Ed25519 signed annotated tag).
- [ ] **AE-P5-033** Publish to GitHub Releases.
- [ ] **AE-P5-034** Update Nix flake + both modules to `v1.0.0`; `nix flake check`.
- [ ] **AE-P5-035** Update Boxship registry index; sign with Apogee key.
- [ ] **AE-P5-036** Schedule §3.1 exception review for 2027-06-01.

---

## AI Track — Agent Subsystem (parallel with Phases 3–5)

> All Guile, concurrent-functional (AE-IMPL §7, §9); every side-effect gated through **Seraph**.

### AI-1 — Foundation (2026-Q3, with Phase 3)

- [ ] **AE-AI-001** Wire `orchestration.capnp` AI streaming (`AiStreamRequest`/`AiStreamChunk`) into Morpheus dispatch.
- [ ] **AE-AI-002** Create `guile/aetheric/agent.scm` skeleton + HTTP client (guile-curl / guile-web / `system*` curl).
- [ ] **AE-AI-003** Unified `ai-provider`: Ollama (`/api/generate`, `/api/chat`) + OpenAI-compatible (`/v1/chat/completions`).
- [ ] **AE-AI-004** `ai-config` `syntax-rules` macro in `init.scm` (default-model, provider, api-key, base-url).
- [ ] **AE-AI-005** `agent-chat` in Architect; tokens stream onto a channel into `ai-mode`.
- [ ] **AE-AI-006** `ai-mode` major-mode stub (buffer-local keymap; streaming sink; works under Nova + Penumbra).
- [ ] **AE-AI-007** Create `guile/aetheric/seraph.scm`; `seraph-approve-edit` + `agent-execute-with-approval` (diff before apply). Default ON.
- [ ] **AE-AI-007.5** NPU auto-detection & acceleration in `ai-config` (device 'npu | 'auto; OpenVINO/llama.cpp backends per PRD §11.8 / OBJ-20); declarative power/thermal policy; integrate with local providers (Ollama, llama.cpp).

### AI-2 — Interactive Agent Buffer (2026-Q4)

- [ ] **AE-AI-008** Full `ai-mode`: streaming Markdown; multi-line input; `/model` `/reset` `/tools`.
- [ ] **AE-AI-009** Tool-calling framework: JSON-schema tools; built-ins `edit-buffer`, `insert-text`, `run-shell` (via `seraph-sandbox-exec` in `nomadic-mode`), `celestial-create-task`, `nexus-commit`.
- [ ] **AE-AI-010** ReAct loop (`agent-run`) — tail-recursive; each `execute-tool` routed through Seraph.
- [ ] **AE-AI-011** Context management: sliding window + summarization.
- [ ] **AE-AI-012** Integration test: "refactor to pattern matching" → diff + approval; assert no edit reaches Stratum pre-approval.

### AI-3 — Advanced Agentic (2027-Q1)

- [ ] **AE-AI-013** Multi-agent: Planner (Celestial), Coder, Reviewer, Orchestrator.
- [ ] **AE-AI-014** Long-running agents on background Fibers + Celestial agenda progress.
- [ ] **AE-AI-015** Memory & RAG: vector store (chromadb or Guile FFI); embedding via thread→channel (AE-IMPL §7.6).
- [ ] **AE-AI-016** Voice/multimodal input (stretch): local Whisper or cloud STT.
- [ ] **AE-AI-017** Agent marketplace via Boxship: publish/share as Apogee-signed packages.

### AI-4 — Performance & Polish (2027-Q2)

- [ ] **AE-AI-018** Profile and optimize pure-Guile token-streaming + tool-dispatch path (p95 ≤ 50 ms target); no FFI or additional unsafe Rust allowed per IMPL v2.7 red line. Update AGENTS.md + PRD §22.1 with "exactly two unsafe sites" (SingleScrew only).
- [ ] **AE-AI-019** `ai-bench` suite: tokens/sec, time-to-first-token, edit success rate.
- [ ] **AE-AI-020** WCAG-compliant AI themes (Nova + Penumbra); add to `wcag_contrast.py`.
- [ ] **AE-AI-021** Full offline mode: bundle small GGUF (opt-in; gated by AI-OQ-01).
- [ ] **AE-AI-022** Telemetry opt-in (privacy-first): model, success rate, latency — never prompt content.

### AI — Seraph Guardrail Verification (gates AI-2+)

- [ ] **AE-AI-G1** `seraph-rate-limit`: per-model token budget + cooldown; configurable.
- [ ] **AE-AI-G2** `seraph-sanitize-prompt`: strip/escape user content in system prompts; injection test.
- [ ] **AE-AI-G3** `seraph-audit-log`: append to `$XDG_STATE_HOME/aetheric/agent.log` (ISO 8601 UTC Z).
- [ ] **AE-AI-G4** `agent-stop-all` kill switch: SIGTERM all model processes; test verifies termination.
- [ ] **AE-AI-G5** `tests/agent_suite.scm` (SRFI-64): full Seraph approval-flow + tool allow-list enforcement.

---

## Task Summary

| Phase | Tasks | Target |
|---|---|---|
| Phase 1 — Foundation | AE-P1-001 … 088 (88) | 2026-Q3 |
| Phase 2 — Rendering + Keymaps + Core | AE-P2-001 … 038 (38) | 2026-Q3–Q4 |
| Phase 3 — Ecosystem + Penumbra | AE-P3-001 … 042 (42) | 2026-Q4–2027-Q1 |
| Phase 4 — Celestial & Launch + Nix | AE-P4-001 … 036 (36) | 2027-Q1–Q2 |
| Phase 5 — Production Hardening | AE-P5-001 … 036 (36) | 2027-Q2–Q3 |
| AI-1 — Foundation | AE-AI-001 … 007.5 (8) | 2026-Q3 |
| AI-2 — Interactive Buffer | AE-AI-008 … 012 (5) | 2026-Q4 |
| AI-3 — Advanced Agentic | AE-AI-013 … 017 (5) | 2027-Q1 |
| AI-4 — Performance & Polish | AE-AI-018 … 022 (5) | 2027-Q2 |
| AI — Seraph Guardrails | AE-AI-G1 … G5 (5) | with AI-2 |
| **Total** | **268 tasks** | **v1.0: 2027-Q3** |

---

*Copyright (c) 2026 Mohamed Hammad & Spacecraft Software. AGPL-3.0-or-later.*  
*Contact: Mohamed.Hammad@SpacecraftSoftware.org | https://Aetheric.SpacecraftSoftware.org/*
