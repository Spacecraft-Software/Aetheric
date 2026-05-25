---
title: Aetheric — Implementation Plan
author: Mohamed Hammad <Mohamed.Hammad@SpacecraftSoftware.org>
date: 2026-05-25
version: "2.7"
document-id: AE-IMPL-001
references: AE-PRD-MASTER-001 v2.7, guile-functional-concurrent (skill)
license: AGPL-3.0-or-later
---

<!-- Spacecraft Software document — GitHub-Flavored Markdown (GFM)
     Palette: Void Navy #000027 bg, Molten Amber #D98E32 body, Steel Blue #4B7EB0 H1,
       Radium Green #50FA7B H2, Liquid Coolant #8BE9FD H3, Red Oxide #FF5C5C errors.
     Typography: Share Tech Mono headings, Inconsolata body.
     §7 codifies Majestic's GNU Guile conventions verbatim from the
     guile-functional-concurrent skill (Mohamed Hammad / Spacecraft Software):
     references/functional.md and references/concurrent.md are canonical. -->

# Aetheric — Implementation Plan

**v2.7 · 2026-05-25 · AE-IMPL-001**  
References: AE-PRD-MASTER-001 v2.7 · `guile-functional-concurrent` skill

| Field | Value |
|---|---|
| Document ID | AE-IMPL-001 |
| Version | 2.7 |
| References | AE-PRD-MASTER-001 v2.7 |
| Author | Mohamed Hammad (Mohamed.Hammad@SpacecraftSoftware.org) |
| Organisation | Spacecraft Software |
| Date | 2026-05-25 |
| License | AGPL-3.0-or-later |

---

## Table of Contents

1. [Cargo Workspace Layout](#1--cargo-workspace-layout)
2. [Crate Dependency Graph](#2--crate-dependency-graph)
3. [External Dependency Registry](#3--external-dependency-registry)
4. [Module Breakdown per Crate (Rust)](#4--module-breakdown-per-crate-rust)
5. [Full Directory Tree](#5--full-directory-tree)
6. [Cap'n Proto Schemas: Constellation](#6--capn-proto-schemas-constellation)
7. [Majestic Implementation Conventions (GNU Guile)](#7--majestic-implementation-conventions-gnu-guile)
8. [Guile Module Inventory](#8--guile-module-inventory)
9. [AI Agent Subsystem](#9--ai-agent-subsystem)
10. [Lints & Static Analysis](#10--lints--static-analysis)
11. [CI Pipeline](#11--ci-pipeline)
12. [Nix Packaging](#12--nix-packaging)
13. [Release Process](#13--release-process)

---

## 1 — Cargo Workspace Layout

The RMS Microkernel and its renderers are Rust; Majestic is Guile. The microkernel ships two renderer crates — Nova (GPU) and Penumbra (terminal) — selected at launch behind a common `Renderer` trait. The AI agent runtime is Guile, so it adds no mandatory Rust crate.

```toml
[workspace]
members = [
  "crates/aetheric-rms",           # RMS Microkernel binary
  "crates/aetheric-stratum",       # persistent rope (Stratum)
  "crates/aetheric-nova",        # GPU display engine (Nova)
  "crates/aetheric-penumbra",      # terminal (TTY/TUI) front-end (Penumbra)
  "crates/aetheric-morpheus",      # IPC broker (Morpheus) [zero unsafe]
  "crates/aetheric-orion",         # tokio runtime wiring (Orion)
  "crates/aetheric-lumen",         # structured logging (Lumen)
  "crates/aetheric-ipc-types",     # owned Cap'n Proto types
  "crates/aetheric-single-screw",  # SS cdylib → librms_ipc.so [ONLY unsafe]
]
resolver = "2"
```

---

## 2 — Crate Dependency Graph

```
aetheric-rms  (binary — RMS Microkernel)
  └─ aetheric-orion
       ├─ aetheric-stratum
       │    └─ aetheric-ipc-types
       ├─ aetheric-nova          (GPU renderer; impl Renderer)
       │    └─ aetheric-ipc-types
       ├─ aetheric-penumbra        (TTY renderer; impl Renderer)
       │    └─ aetheric-ipc-types
       ├─ aetheric-morpheus        [#![deny(unsafe_code)]]
       │    └─ aetheric-ipc-types
       └─ aetheric-lumen

aetheric-single-screw  (cdylib — SingleScrew/SS)
  └─ aetheric-ipc-types            (tokio-free; safe to share)
  ⚠  MUST NOT depend on tokio (M-ISOLATE-DLL-STATE)
```

`aetheric-rms` selects Nova or Penumbra at startup; the rest of the system is renderer-agnostic.

---

## 3 — External Dependency Registry

### 3.1 — Production Dependencies (Rust)

| Crate | Ver. | Used by | Purpose |
|---|---|---|---|
| `crop` | 0.4 | stratum | Persistent CoW B-tree rope |
| `arc-swap` | 1.7 | orion | Lock-free `ArcSwap<Arc<Rope>>` snapshot reads |
| `dashmap` | 6 | orion | Concurrent snapshot handle registry |
| `tokio` | 1 | orion, morpheus | Async runtime (features: full) |
| `tokio-util` | 0.7 | morpheus | `LengthDelimitedCodec` for IPC framing |
| `capnp` | 0.20 | morpheus, single-screw, ipc-types | Cap'n Proto runtime |
| `wgpu` | 0.20 | nova | GPU (Vulkan/Metal/DX12/GLES) |
| `cosmic-text` | 0.12 | nova | Unicode shaping, bidi, font fallback |
| `glyphon` | 0.6 | nova | wgpu GPU-resident glyph atlas |
| `fontdb` | 0.21 | nova | Font discovery (XDG_DATA_DIRS) |
| `crossterm` | 0.28 | penumbra | Cross-platform terminal control + key events |
| `ratatui` | 0.28 | penumbra | TUI cell layout / widgets |
| `tree-sitter` | 0.22 | nova, penumbra | Incremental syntax parsing |
| `tracing` | 0.1 | all crates | Structured logging (M-LOG-STRUCTURED) |
| `tracing-subscriber` | 0.3 | lumen | Log rendering; env-filter; JSON mode |
| `jiff` | 0.1 | lumen | UTC timestamps (Standard §12.5) |
| `clap` | 4 | rms | CLI argument parsing |
| `anyhow` | 1 | rms, single-screw | Application-level error handling |
| `libc` | 0.2 | single-screw | POSIX `pipe(2)` / `read(2)` / `write(2)` |
| `git2` | 0.19 | orion (via IPC) | libgit2 bindings for Nexus read ops |

### 3.2 — Build Dependencies

| Crate | Ver. | Used by | Purpose |
|---|---|---|---|
| `capnpc` | 0.20 | morpheus, single-screw, ipc-types | Compiles `.capnp` → Rust |
| `cbindgen` | 0.27 | single-screw | Generates `include/rms_ipc.h` |

### 3.3 — Dev / Test Dependencies

| Crate | Ver. | Purpose |
|---|---|---|
| `criterion` | 0.5 | Benchmarking (PRD §16 metric gates) |
| `proptest` | 1 | Property-based testing for rope invariants |
| `tempfile` | 3 | Temporary socket paths in integration tests |
| `libfuzzer-sys` | 0.4 | Fuzz harness (cargo-fuzz targets) |
| `tokio-test` | 0.4 | Async test helpers |
| `assert_cmd` | 2 | CLI integration tests for the rms binary |
| `insta` | 1 | Snapshot tests for Penumbra cell output |

### 3.4 — Tree-sitter Grammar Crates (feature-gated)

| Crate | Language | Flag | Phase |
|---|---|---|---|
| `tree-sitter-rust` | Rust | `lang-rust` | 2 |
| `tree-sitter-markdown` | Markdown | `lang-markdown` | 2 |
| `tree-sitter-scheme` | Scheme | `lang-scheme` | 2 |
| `tree-sitter-toml` | TOML | `lang-toml` | 2 |
| `tree-sitter-org` | Org-mode | `lang-org` | 4 (Celestial) |
| `tree-sitter-{python,javascript,typescript,c,json}` | various | `lang-*` | 3 |

### 3.5 — Guile Runtime: Modules & Libraries (Majestic)

Majestic assumes **Guile 3.x** (JIT, suspendable ports, `(ice-9 match)`, `(ice-9 exceptions)`, `define-syntax` hygiene) and **guile-fibers**. The toolkit below is fixed; §7 sets the conventions for using it. Module names are Guile-specific — never substitute generic R6RS library paths.

| Module | Role in Majestic |
|---|---|
| `(fibers)` | `run-fibers`, `spawn-fiber` — the CSP scheduler and single entry point |
| `(fibers channels)` | `make-channel`, `put-message`, `get-message` — all inter-fiber communication |
| `(fibers operations)` | `choice-operation`, `wrap-operation`, `perform-operation`, `get-operation` — the event-loop select |
| `(fibers timers)` | `sleep`, `sleep-operation` — fiber-aware timing, frame ticks, timeouts |
| `(ice-9 match)` | Pattern-matching every `EditorEvent` / `OrchEvent` in the pure transition |
| `(ice-9 exceptions)` | Structured exceptions + `guard` for per-fiber and per-extension isolation |
| `(ice-9 threads)` | `call-with-new-thread`, `with-mutex`, condition variables — only for blocking C calls / CPU parallelism |
| `(ice-9 futures)` | `future` / `touch` — pure CPU-bound batch work (collected off the interactive fiber) |
| `(srfi srfi-1)` | List processing — `fold`, `reduce`, `filter-map`, `partition`, `unfold`, `iota` |
| `(srfi srfi-9)` | Records — immutable buffer / mode / keymap / agent / theme structures |
| `(srfi srfi-11)` | `let-values` — multiple returns (`partition`, snapshot+version) |
| `(srfi srfi-13)` | String library |
| `(srfi srfi-26)` | `cut` / `cute` — partial application in command and tool wiring |
| `(srfi srfi-41)` | Streams — lazy iteration over very large buffers |
| `(srfi srfi-64)` | The test framework for the entire Guile suite |
| `(srfi srfi-171)` | Transducers — allocation-free diagnostic / token processing pipelines |
| `(system foreign)` | `foreign-library-function` — loads `librms_ipc.so` in `(aetheric rms-ipc)` |

### 3.6 — AI Runtime Dependencies (Guile-side)

| Dependency | Kind | Purpose |
|---|---|---|
| JSON module (Guile 3 built-in, else `guile-json`) | Guile lib | Model API payloads + tool-call schemas |
| `guile-curl` / `guile-web` (or `system*` to curl) | Guile lib | HTTP client for model providers |
| `ollama` | External (optional) | Local model serving |
| `llama.cpp` / GGUF | External (optional) | Local inference; bundled GGUF is AI-OQ-01 |
| vector store (chromadb or Guile FFI) | External (optional) | Memory & RAG (AI-3) |

---

## 4 — Module Breakdown per Crate (Rust)

### 4.1 — aetheric-ipc-types

| Module | File | Exports |
|---|---|---|
| lib | `src/lib.rs` | `#![deny(unsafe_code)]`; re-exports |
| events | `src/events.rs` | `pub enum EditorEvent` (8 variants, owned) |
| commands | `src/commands.rs` | `CoreCommand` (11); `OrchCommand`; `OrchEvent` (incl. AI streaming) |
| error | `src/error.rs` | `pub enum IpcError { Capnp, Io, Disconnected, Version }` |
| convert | `src/convert.rs` | `EditorEvent::try_from(reader)`; `CoreCommand::to_builder()` |
| schema | `src/schema.rs` | `include!` for `core_capnp.rs` + `orchestration_capnp.rs` |

### 4.2 — aetheric-stratum

| Module | File | Exports |
|---|---|---|
| backend | `src/backend.rs` | `pub trait RopeBackend: Send + Sync` |
| rope | `src/rope.rs` | `pub struct Stratum(crop::Rope)`; impl RopeBackend |
| encoding | `src/encoding.rs` | `utf8_validate()`, `byte_offset_is_char_boundary()`, `utf16_to_byte()`, `byte_to_utf16()` |
| snapshot | `src/snapshot.rs` | `SnapshotRegistry(DashMap<u32, Arc<Stratum>>)`; create/release/get |
| iter | `src/iter.rs` | `ChunkIter`, `LineIter` |
| lib | `src/lib.rs` | `#![deny(unsafe_code)]`; re-exports |

### 4.3 — aetheric-nova (GPU renderer)

| Module | File | Responsibility |
|---|---|---|
| lib | `src/lib.rs` | `pub struct Nova`; `impl Renderer` |
| surface | `src/surface.rs` | wgpu Device/Queue/Surface; backend priority Vulkan→Metal→DX12→GLES |
| renderer | `src/renderer.rs` | Frame loop; acquire→shape→upload→render→present; timestamp queries |
| glyph | `src/glyph.rs` | `glyphon::TextAtlas`; cache invalidation on resize/DPI |
| text_layout | `src/text_layout.rs` | `cosmic_text::FontSystem`; shape viewport; RTL/bidi |
| syntax | `src/syntax.rs` | tree-sitter scope → Spectrum colour |
| overlay | `src/overlay.rs` | Frame-time histogram; Void Navy palette |
| font | `src/font.rs` | Load bundled TTFs; fontdb discovery |

### 4.4 — aetheric-penumbra (TTY renderer)

| Module | File | Responsibility |
|---|---|---|
| lib | `src/lib.rs` | `pub struct Penumbra`; `impl Renderer` |
| terminal | `src/terminal.rs` | crossterm raw mode, alternate screen, resize signals; restore on drop |
| grid | `src/grid.rs` | Cell grid; ratatui buffer; viewport → cells |
| paint | `src/paint.rs` | Diff-based cell repaint; minimal escape output |
| palette | `src/palette.rs` | Spectrum → 24-bit truecolor; 256/16-colour graceful fallback |
| input | `src/input.rs` | crossterm key/mouse events → `EditorEvent::KeyPress` |
| syntax | `src/syntax.rs` | tree-sitter scope → terminal styles |

> `Renderer` trait (in `aetheric-orion`): `fn present(&mut self, snapshot: &Arc<Stratum>, viewport: &Viewport, theme: &Theme)` + `fn poll_input(&mut self) -> Vec<EditorEvent>`. Nova and Penumbra are interchangeable behind it; the `renderer_parity` test asserts identical logical layout for a given snapshot.

### 4.5 — aetheric-morpheus `[#![deny(unsafe_code)]]`

| Module | File | Responsibility |
|---|---|---|
| lib | `src/lib.rs` | `pub struct MorpheusBroker`; `spawn()` |
| listener | `src/listener.rs` | Bind Unix socket; accept loop |
| connection | `src/connection.rs` | Per-connection read + write tokio tasks; bounded channels |
| codec | `src/codec.rs` | `LengthDelimitedCodec`; encode/decode |
| dispatch | `src/dispatch.rs` | Route `CoreCommand` and `OrchCommand` to the right channel |
| bp | `src/bp.rs` | `COMMAND_CHANNEL_BOUND = 256` (M-DOCUMENTED-MAGIC) |

### 4.6 — aetheric-orion

| Module | File | Exports |
|---|---|---|
| lib | `src/lib.rs` | `pub struct Orion`; `new(cfg)`; `run()` |
| renderer | `src/renderer.rs` | `pub trait Renderer`; backend selection (Nova \| Penumbra) |
| state | `src/state.rs` | `SharedState { rope: ArcSwap<Arc<Stratum>>, snapshots, render_tx }` |
| tasks | `src/tasks.rs` | `spawn_morpheus_reader/writer`, `spawn_stratum_mutator`, `spawn_render`, `spawn_snapshot_gc`, `spawn_treesitter_parse`, `spawn_git`, `spawn_export` |
| channels | `src/channels.rs` | Named bounds (256 / 4 / 64) |
| config | `src/config.rs` | `Config { worker_threads, socket_path, frame_rate, renderer }` |
| signal | `src/signal.rs` | SIGTERM / Ctrl-C → `CoreCommand::Shutdown` |

### 4.7 — aetheric-lumen

| Module | File | Exports |
|---|---|---|
| lib | `src/lib.rs` | `pub fn init(cfg: LumenConfig)` |
| format | `src/format.rs` | `YYYY-MM-DDTHH:MM:SS.sssZ` via jiff; JSON mode |
| config | `src/config.rs` | `LumenConfig { level, format }` |

### 4.8 — aetheric-single-screw (SS cdylib → librms_ipc.so)

| File | Responsibility |
|---|---|
| `src/lib.rs` | 18 `#[no_mangle] extern "C"` fns; `ffi_guard!(catch_unwind)`; `RmsConn*`, `RmsMsg*`, `RmsCmd*` |
| `src/conn.rs` | `Conn`; background reader thread; POSIX pipe notification pair (the fd behind `rms_fd()`) |
| `src/proto.rs` | Owned `Event` + `Cmd` enums; `read_event()`; `write_cmd()` |
| `src/error.rs` | Thread-local `LAST_CODE` + `LAST_MSG` |
| `include/rms_ipc.h` | C ABI contract (cbindgen-generated; committed) |
| `guile/rms-ipc.scm` | `(aetheric rms-ipc)` — 18 bindings + the fiber-aware reader (§7.3) |
| `build.rs` | capnpc compile; cbindgen generate header |

### 4.9 — aetheric-rms (RMS Microkernel binary)

| File | Responsibility |
|---|---|
| `src/main.rs` | clap CLI; `lumen::init()`; choose renderer (`--tty`/auto); build Orion config; `orion.run()` |
| `src/cli.rs` | `Args` (`--socket-path`, `--log-level`, `--frame-rate`, `--tty`, `--offline`, `--no-restore`); `--version` attribution block (Standard §13.2) |

---

## 5 — Full Directory Tree

```
aetheric/
├── Cargo.toml                         # workspace manifest
├── rust-toolchain.toml
├── deny.toml
├── clippy.toml
├── AGENTS.md                          # machine-readable agent guidelines
├── schemas/
│   ├── core.capnp                     # EditorEvent + CoreCommand
│   └── orchestration.capnp           # Celestial, Nexus, AI streaming
├── crates/
│   ├── aetheric-rms/                  # RMS Microkernel binary
│   ├── aetheric-stratum/
│   ├── aetheric-nova/               # GPU renderer
│   ├── aetheric-penumbra/             # TTY renderer
│   ├── aetheric-morpheus/             # IPC broker [#![deny(unsafe_code)]]
│   ├── aetheric-orion/                # Renderer trait + task pool
│   ├── aetheric-lumen/
│   ├── aetheric-ipc-types/
│   └── aetheric-single-screw/         # SS cdylib → librms_ipc.so [ONLY unsafe]
│       ├── include/rms_ipc.h
│       ├── guile/rms-ipc.scm
│       └── src/{lib,conn,proto,error}.rs
├── guile/
│   ├── aetheric/
│   │   ├── rms-ipc.scm                # SS bridge wrappers (fiber-aware reader)
│   │   ├── core.scm                   # pure state + apply-event transition
│   │   ├── editor.scm                 # (aetheric editor) public API
│   │   ├── astrolabe.scm              # keymaps (persistent trees) + first-run
│   │   ├── halo.scm                   # extension sandbox (per-fiber guard)
│   │   ├── oracle.scm                 # help & introspection
│   │   ├── architect.scm              # REPL buffer
│   │   ├── boxship.scm                # package manager
│   │   ├── spectrum.scm               # theme system
│   │   ├── ephemeris.scm              # session + XDG state
│   │   ├── lsp.scm                    # LSP client (fiber pair per server)
│   │   ├── celestial.scm              # Org-mode successor
│   │   ├── nexus.scm                  # Git interface
│   │   ├── construct.scm              # directory browser
│   │   ├── agent.scm                  # AI agent runtime + tools
│   │   └── seraph.scm                 # AI guardrails / policy engine
│   └── main.scm                       # run-fibers entry point + event loop
├── assets/
│   ├── fonts/{JetBrainsMono,NotoSansArabic,NotoColorEmoji}.ttf
│   └── logo/aetheric-logo.jpg
├── tests/
│   ├── ipc_integration.rs
│   ├── stratum_proptest.rs
│   ├── renderer_parity.rs             # Nova vs Penumbra logical layout
│   ├── run-tests.scm                  # SRFI-64 suite entry
│   ├── core_transitions.scm           # pure apply-event tests (no editor needed)
│   ├── fibers_suite.scm               # channel select, backpressure, isolation
│   ├── ss_bridge_smoke.scm
│   ├── celestial_suite.scm
│   ├── nexus_suite.scm
│   └── agent_suite.scm                # AI agent + Seraph approval flow
├── benches/{rope_ops.rs,ipc_roundtrip.rs,frame_latency.rs,ai_bench.scm}
├── fuzz/{fuzz_stratum_insert.rs,fuzz_stratum_delete.rs,fuzz_morpheus_decoder.rs}
├── scripts/{wcag_contrast.py,check_spdx.sh,guile-lint.scm}
├── docs/{spacecraft-software-exceptions.md,CREDITS.md}
├── nix/
│   ├── flake.nix
│   ├── nixos-module.nix               # programs.aetheric (NixOS)
│   └── hm-module.nix                  # programs.aetheric (Home Manager)
├── keys/aetheric-release.pub
├── README.md  NOTICE.md  CONTRIBUTING.md  LICENSE
```

---

## 6 — Cap'n Proto Schemas: Constellation

### 6.1 — schemas/core.capnp (EditorEvent + CoreCommand)

```capnp
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Mohamed Hammad & Spacecraft Software
# Constellation Core Schema v0.1 — ordinals are permanent, never reuse.
@0xdf4a782b90ce8a11;

struct WindowDimensions { widthMm @0 :Float32; heightMm @1 :Float32; }
struct BufferHandle { bufferId @0 :UInt32; path @1 :Text; }
struct SnapshotHandle { snapshotId @0 :UInt32; bufferId @1 :UInt32; version @2 :UInt64; }
struct InsertPayload { bufferId @0 :UInt32; byteOffset @1 :UInt64; content @2 :Text; }
struct DeletePayload { bufferId @0 :UInt32; byteOffset @1 :UInt64; length  @2 :UInt64; }

struct DiagnosticAnnotation {
  bufferId  @0 :UInt32; startByte @1 :UInt64; endByte @2 :UInt64;
  severity  @3 :DiagnosticSeverity; message @4 :Text; source @5 :Text; code @6 :Text;
}
enum DiagnosticSeverity { error @0; warning @1; information @2; hint @3; }

struct ViewportCommand { bufferId @0 :UInt32; startLine @1 :UInt64; lineCount @2 :UInt32; }
struct RmsError { code @0 :UInt32; message @1 :Text; cmdId @2 :UInt64; }

struct EditorEvent {
  union {
    keyPress @0 :Text;            windowResize @1 :WindowDimensions;
    clientShutdown @2 :Void;      bufferOpened @3 :BufferHandle;
    snapshotReady @4 :SnapshotHandle; snapshotReleased @5 :UInt32;
    bufferClosed @6 :UInt32;      error @7 :RmsError;
  }
}

struct CoreCommand {
  id @0 :UInt64;
  union {
    openBuffer @1 :Text;          insertText @2 :InsertPayload;
    deleteText @3 :DeletePayload; requestSnapshot @4 :UInt32;
    releaseSnapshot @5 :UInt32;   closeBuffer @6 :UInt32;
    setDiagnostics @7 :List(DiagnosticAnnotation); setViewport @8 :ViewportCommand;
    toggleOverlay @9 :Void;       shutdown @10 :Void;
  }
}
```

### 6.2 — schemas/orchestration.capnp (Celestial, Nexus, AI streaming)

```capnp
# SPDX-License-Identifier: AGPL-3.0-or-later
@0xae7c3f19d82b6e04;
using Core = import "core.capnp";

struct CelestialExportRequest { bufferId @0 :UInt32; format @1 :ExportFormat; outputPath @2 :Text; }
enum ExportFormat { pdf @0; html @1; markdown @2; latex @3; }
struct CelestialAgendaQuery { rangeStartIso @0 :Text; rangeEndIso @1 :Text; tags @2 :List(Text); }
struct CelestialAgendaItem {
  heading @0 :Text; state @1 :Text; deadline @2 :Text; scheduled @3 :Text;
  tags @4 :List(Text); priority @5 :Text; bufferId @6 :UInt32; byteOffset @7 :UInt64;
}
struct NexusStatusEntry { path @0 :Text; indexState @1 :GitState; wdState @2 :GitState; }
enum GitState { unmodified @0; modified @1; added @2; deleted @3; renamed @4; copied @5; untracked @6; ignored @7; }
struct NexusCommit { hash @0 :Text; shortHash @1 :Text; author @2 :Text; authorEmail @3 :Text; timestampZ @4 :Text; message @5 :Text; }
struct NexusHunkRange { startLine @0 :UInt32; lineCount @1 :UInt32; }
struct NexusHunk { oldRange @0 :NexusHunkRange; newRange @1 :NexusHunkRange; header @2 :Text; lines @3 :List(Text); }
struct NexusStageHunk { path @0 :Text; hunkIndex @1 :UInt32; }
struct AiStreamRequest { requestId @0 :UInt64; prompt @1 :Text; contextBuf @2 :UInt32; }
struct AiStreamChunk   { requestId @0 :UInt64; content @1 :Text; done @2 :Bool; }

struct OrchCommand {
  id @0 :UInt64;
  union {
    celestialExport @1 :CelestialExportRequest; celestialAgendaQuery @2 :CelestialAgendaQuery;
    nexusStatusRequest @3 :UInt32; nexusCommitLogRequest @4 :UInt32;
    nexusStageHunk @5 :NexusStageHunk; nexusUnstageHunk @6 :NexusStageHunk;
    nexusCommit @7 :Text; aiStreamRequest @8 :AiStreamRequest; aiStreamCancel @9 :UInt64;
  }
}
struct OrchEvent {
  union {
    exportDone @0 :ExportDoneResult; agendaResult @1 :List(CelestialAgendaItem);
    nexusStatusResult @2 :List(NexusStatusEntry); nexusCommitLogResult @3 :List(NexusCommit);
    nexusDiff @4 :List(NexusHunk); aiStreamChunk @5 :AiStreamChunk; orchError @6 :Core.RmsError;
  }
}
struct ExportDoneResult { path @0 :Text; sizeBytes @1 :UInt64; }
```

---

## 7 — Majestic Implementation Conventions (GNU Guile)

> Majestic is **pure first** and **concurrent via message-passing**. These conventions are binding on every `.scm` file and follow the project's `guile-functional-concurrent` skill; `references/functional.md` and `references/concurrent.md` are canonical. The core stance: (1) functions take values and return values, side effects pushed to the edges; (2) message-passing over shared state; (3) proper tail calls are not optional; (4) hygiene over `define-macro`.

### 7.1 — The Functional Core

State is an immutable value; change is a **pure transition** `(apply-event state event) → state'`. No core `.scm` uses `set!`; mutation lives only at the edges (the SS bridge, logging, terminal/GPU output). Express computations with `let`/`let*`/`letrec` locals rather than internal `define`.

**The single transition function** — `(ice-9 match)` over every event, returns a fresh state, never mutates:

```scheme
(define-module (aetheric core)
  #:use-module (ice-9 match)
  #:use-module (srfi srfi-1)
  #:use-module (srfi srfi-9)
  #:export (initial-state apply-event))

(define (apply-event state event)
  (match event
    (('rms   . ('key-press k))      (dispatch-key state k))
    (('rms   . ('window-resize d))  (resize-viewport state d))
    (('rms   . ('buffer-opened h))  (register-buffer state h))
    (('lsp   diags)                 (set-diagnostics state diags))
    (('ai    . ('token id t))       (append-ai-token state id t))
    (('ai    . ('done id))          (finalize-ai-stream state id))
    (('timer . _)                   (run-idle-timers state))
    (_                              state)))            ; unknown → unchanged
```

**Records are immutable (SRFI-9)** — omit the setter to make a field immutable (the functional default); provide functional updaters:

```scheme
(define-record-type <buffer>
  (make-buffer id major-mode minor-modes locals)
  buffer?
  (id          buffer-id)            ; no setter → immutable
  (major-mode  buffer-major-mode)
  (minor-modes buffer-minor-modes)
  (locals      buffer-locals))

(define (buffer-with-mode buf mode)  ; returns a NEW buffer
  (make-buffer (buffer-id buf) mode (buffer-minor-modes buf) (buffer-locals buf)))
```

**Keymaps are persistent prefix trees**; rebinding returns a new keymap that structurally shares the rest, so live reconfiguration never disturbs in-flight dispatch:

```scheme
(define (keymap-bind km keys closure)
  (make-keymap (ptree-insert (keymap-table km) keys closure)))  ; old km untouched
```

**Hooks run via SRFI-1 `fold`** — `(fold proc init lst)` calls `proc` with `(element accumulator)`, element first; getting this backwards is the classic Guile bug:

```scheme
(define (run-hooks state hooks)
  (fold (lambda (hook st) (hook st)) state hooks))
```

**Prefer the combinator over hand-rolled recursion**; reach for SRFI-1 directly:

```scheme
(use-modules (srfi srfi-1) (srfi srfi-11) (srfi srfi-26))
(filter-map (lambda (b) (and (buffer-dirty? b) (buffer-id b))) buffers)   ; dirty ids
(let-values (((vis hidden) (partition buffer-visible? buffers))) ...)     ; two values
(map (cut send-command conn <>) cmds)                                     ; partial app
```

**Tail recursion is mandatory** for any loop over unbounded data — a named `let` with an accumulator (the dispatch loop, the ReAct loop §9, and stream consumers are all in tail position). For lazy iteration over very large buffers use SRFI-41 streams, not eager lists; for hot, allocation-sensitive token/diagnostic transforms use SRFI-171 transducers. `cut` re-evaluates its args each call; use `cute` when an argument is an expensive constant.

**Pattern matching** uses `(ice-9 match)`; `match-lambda` and `match-let` are available in argument and binding position and are preferred over nested `car`/`cdr`/`cond`.

### 7.2 — Concurrency Model: the Decision Tree, Applied

Majestic follows the skill's decision tree; each concurrent concern maps to exactly one model. When in doubt for I/O work, choose Fibers.

| Concern | Nature | Model | Rationale |
|---|---|---|---|
| RMS event intake (via SS bridge) | I/O-bound | **Fibers + channels** | One reader fiber turns `rms_fd()` into a CSP source (§7.3) |
| LSP servers | I/O-bound, many | **Fibers + channels** | Fiber pair per server; worker-pool fan-out for requests (§7.4) |
| AI token streaming | I/O-bound | **Fibers + channels** | Tokens arrive on a channel and fold into `ai-mode` state |
| Nexus git subprocesses | I/O-bound | **Fibers + channels** | One fiber per git op; result over a channel |
| Celestial backlink index | I/O + light CPU | **Fibers** | Background fiber; progress on a channel |
| Event-loop select / timeouts | composition | **`choice-operation`** | Race RMS / LSP / AI / frame-tick events (§7.4) |
| Celestial PDF export | CPU-bound | **delegate to RMS (Orion), else thread→channel** | Heavy LaTeX render belongs off the interactive fiber (§7.6) |
| RAG embedding (AI-3) | CPU-bound, pure | **`ice-9 futures`**, collected via §7.6 | Pure batch compute on the worker pool |
| Lazy config / fonts | compute-once | **promises** (`delay`/`force`) | Parse `init.scm` once, memoized |
| `current-buffer` / `current-conn` etc. | per-task context | **parameters** | Fiber-local; no globals, no races |

`run-fibers` is the **single entry point** (`guile/main.scm`); every fiber is spawned inside it. It may take `#:parallelism` to spread fibers across OS threads when CPU parallelism helps.

### 7.3 — The SingleScrew ↔ Fibers Bridge (the critical integration)

The skill's number-one pitfall: *a blocking call inside a fiber stalls every fiber on that scheduler thread.* `rms_recv()` blocks — so Majestic must **never** call it from a fiber. This is exactly why the SS bridge exposes `rms_fd()` (a pollable notification fd) and `rms_try_recv()` (non-blocking). We turn the C bridge into a clean CSP source with a dedicated reader fiber:

```scheme
(define-module (aetheric rms-ipc)
  #:use-module (system foreign)
  #:use-module (fibers channels)
  #:export (rms-connect rms-fd rms-try-recv rms-send-insert-text
            rms-send-open-buffer spawn-rms-reader))

;; A dedicated reader fiber bridges the notification fd into a channel.
;; run-fibers installs suspendable ports, so reading one byte from the
;; notification port suspends ONLY this fiber — the scheduler keeps running.
(define (spawn-rms-reader conn rms-ch)
  (spawn-fiber
    (lambda ()
      (let ((port (rms-notify-port conn)))   ; unbuffered input port over rms_fd()
        (let loop ()
          (get-u8 port)                       ; fiber-aware wait for readability
          (drain-events conn rms-ch)          ; non-blocking drain
          (loop))))))

(define (drain-events conn ch)
  (let loop ()
    (let ((ev (rms-try-recv conn)))           ; NULL + WOULD_BLOCK ⇒ #f
      (when ev
        (put-message ch ev)
        (loop)))))                            ; stop when the queue is empty
```

`rms_send()` returns quickly, so sends may be issued directly from the dispatch fiber. **Rule:** the only contact with the blocking C ABI is the dedicated reader fiber waiting on the suspendable port; everything else speaks channels.

### 7.4 — The Event Loop, Worker Pools & Pipelines

**Event loop** — a CSP *select* via `choice-operation` (Guile's answer to Go's `select`); it never blocks on a single source and folds each event through the pure transition in tail position. Channels are **unbuffered rendezvous** by default, giving natural backpressure:

```scheme
(define-module (aetheric main)
  #:use-module (fibers) #:use-module (fibers channels)
  #:use-module (fibers operations) #:use-module (fibers timers)
  #:use-module (aetheric core))

(define (event-loop conn rms-ch lsp-ch ai-ch)
  (let loop ((state (initial-state)))
    (let ((event
            (perform-operation
              (choice-operation
                (wrap-operation (get-operation rms-ch) (lambda (e) (cons 'rms e)))
                (wrap-operation (get-operation lsp-ch) (lambda (m) (cons 'lsp m)))
                (wrap-operation (get-operation ai-ch)  (lambda (t) (cons 'ai  t)))
                (wrap-operation (sleep-operation 0.016) (lambda _ (cons 'timer #f)))))))
      (loop (apply-event state event)))))      ; pure, tail call
```

**LSP servers** — one fiber pair per server bridges stdio JSON-RPC to channels; concurrent requests fan out worker-pool style. Channels do not close, so the `'done` sentinel signals completion (one per consumer for pools):

```scheme
(define (spawn-lsp-server cmd req-ch resp-ch)
  (let ((proc (open-lsp-process cmd)))
    (spawn-fiber (lambda () (lsp-writer proc req-ch)))    ; requests → stdin
    (spawn-fiber (lambda () (lsp-reader proc resp-ch))))) ; stdout → channel
```

**Diagnostics pipeline** — raw LSP payloads flow through staged fibers (parse → normalize → `DiagnosticAnnotation`), each stage connected by a channel for backpressure, then leave as a single `set-diagnostics` CoreCommand. Where the transform is pure and hot, implement the stage with SRFI-171 transducers to avoid intermediate allocation.

### 7.5 — Per-Task Context via Parameters

Context that would otherwise be global is a fiber-local parameter (`make-parameter` is per-thread *and* per-fiber), so each fiber sees its own binding with no races:

```scheme
(define current-conn   (make-parameter #f))   ; the RMS connection
(define current-buffer (make-parameter #f))
(define current-keymap (make-parameter #f))
(define current-theme  (make-parameter #f))

(parameterize ((current-buffer buf))
  (run-mode-hooks))                            ; sees buf only in this dynamic extent
```

### 7.6 — CPU-bound Work Without Stalling the Scheduler

`touch` on a future or `join-thread` **blocks the OS thread**, which would stall the fiber scheduler — forbidden on any fiber. Futures are for CPU-bound *pure* work only; using them for I/O ties up the compute pool. Two sanctioned patterns:

1. **Delegate to the RMS Microkernel.** Heavy, side-effecting jobs (Celestial PDF/LaTeX export, large `git status`) go out as an `OrchCommand`; Orion runs them on a tokio task and replies with an `OrchEvent`, which arrives on a channel the event loop already selects on. This is the default.
2. **Thread → channel.** For genuinely Guile-side CPU work (e.g. AI-3 RAG embedding), run it on a POSIX thread (or `future`) and have a small helper deliver the result to a channel; the requesting fiber waits with a fiber-aware `get-message`:

```scheme
(use-modules (ice-9 threads) (fibers channels))
(define (compute-async thunk done-ch)
  (call-with-new-thread
    (lambda () (put-message done-ch (thunk)))))  ; result re-enters CSP land
```

Never `(touch f)` or `(join-thread t)` on a fiber. On the rare `ice-9 threads` path that shares state, use `with-mutex` (never manual lock/unlock), keep the critical section tiny, never block while holding the lock, fix one global lock order, and always re-check a condition variable's predicate in a loop (spurious wakeups).

### 7.7 — Error Isolation (per fiber, per extension)

A failure in one fiber must never take down the scheduler, and a crashing Halo extension must never crash Majestic. Wrap every fiber body and every extension call in `guard`; use `with-exception-handler #:unwind? #t` for non-unwinding-to-unwinding bridges, and `dynamic-wind` to guarantee resource cleanup across non-local exits:

```scheme
(define-module (aetheric halo)
  #:use-module (ice-9 exceptions)
  #:use-module (aetheric lumen))

(define (guarded-fiber name thunk)
  (spawn-fiber
    (lambda ()
      (guard (exn (#t (lumen-error (format #f "fiber.~a.crash" name)
                                   (exception-message exn))))
        (thunk)))))

(define (halo-safe-call ext proc)               ; extension boundary
  (guard (exn ((error? exn)
               (lumen-error "extension.error" (exception-message exn))
               (surface-diagnostic ext exn)
               #f))
    (proc)))

;; Resource lifetimes (PTY, sockets, LSP processes) are dynamic-wind-guarded.
(define (with-lsp-process cmd proc)
  (let ((p #f))
    (dynamic-wind
      (lambda () (set! p (open-lsp-process cmd)))
      (lambda () (proc p))
      (lambda () (when p (close-lsp-process p))))))
```

Expected failure paths return `(values 'ok x)` / `(values 'error reason)` rather than raising; exceptions are reserved for genuinely exceptional conditions.

### 7.8 — Macros & the Configuration DSL

`init.scm` is a declarative DSL built from **hygienic** `syntax-rules` macros (never `define-macro`, which captures variables). They expand to plain data plus closures, so the Architect REPL can evaluate them live:

```scheme
(define-syntax define-keymap
  (syntax-rules ()
    ((_ name (keys cmd) ...)
     (register-keymap 'name (list (cons keys cmd) ...)))))

(define-syntax ai-config
  (syntax-rules (default-model provider api-key base-url)
    ((_ (default-model m) (provider p) (api-key k) (base-url u))
     (install-ai-config (make-ai-config m 'p k u)))))
```

Use `syntax-case` only where identifiers must be inspected or deliberately introduced — e.g. a `define-mode` that binds a conventional `self` identifier in mode bodies:

```scheme
(define-syntax define-mode
  (lambda (stx)
    (syntax-case stx ()
      ((_ name clause ...)
       (with-syntax ((self (datum->syntax #'name 'self)))
         #'(register-mode 'name (lambda (self) clause ...)))))))
```

Registration is concentrated at load time (the edge); the runtime core stays pure.

### 7.9 — Style Rules & Enforced Pitfalls

Binding conventions, enforced in review and (where mechanizable) by the Guile linter in CI:

- **Naming:** `kebab-case`; predicates end in `?` (`buffer-dirty?`); the rare mutator ends in `!`.
- **Avoid `set!`**; isolate any unavoidable mutable state behind a channel or parameter.
- **Named `let` loops**, not `do`. Every unbounded loop is tail-recursive.
- **`run-fibers` is the sole fiber entry point**; spawn all fibers within it.
- **Fiber-aware I/O only inside fibers** — never a blocking C call, raw `read`, or `ice-9 threads` `sleep` on a fiber (§7.3, §7.6); use `(fibers timers)` `sleep`.
- **Channels carry concurrent state**; never share a mutable variable between fibers ("cooperative" still races at every suspension point).
- **`with-mutex`**, never manual lock/unlock; tiny critical sections; never block while holding a lock; one global lock order.
- **One `'done` sentinel per consumer** to terminate a channel loop (pools send N).
- **Comments:** `;` inline, `;;` block, `;;;` section header, `;;;;` file header. `pk` ("peek") is the only sanctioned debug print — it returns its last argument, so it can wrap an expression — and none may survive into a commit.

### 7.10 — Module Import Conventions

Each Majestic module declares imports explicitly via `define-module`; selective import (`#:select`) avoids clashes. Hot modules are AOT-compiled with `guild compile` (Guile 3 also JITs at runtime).

```scheme
(define-module (aetheric editor)
  #:use-module (srfi srfi-1)
  #:use-module (srfi srfi-9)
  #:use-module (srfi srfi-26)
  #:use-module (ice-9 match)
  #:use-module ((aetheric rms-ipc) #:select (rms-send-insert-text rms-send-open-buffer))
  #:export (buffer-open buffer-close buffer-insert buffer-delete
            buffer-local-set buffer-local-ref current-buffer with-buffer
            hook-add hook-run))
```

### 7.11 — Functional Toolkit → Where Aetheric Uses It

| SRFI / module | Aetheric usage |
|---|---|
| SRFI-1 (`fold`, `reduce`, `filter-map`, `partition`, `unfold`, `iota`) | Hook running, dispatch fan-out, buffer/diagnostic transforms |
| SRFI-9 records | Buffer, mode, keymap, theme, agent, ai-config structures (immutable) |
| SRFI-11 `let-values` | `partition` results; snapshot + version pairs |
| SRFI-13 strings | Key-sequence formatting, path handling |
| SRFI-26 `cut`/`cute` | Command and tool wiring; point-free senders |
| SRFI-41 streams | Lazy iteration over very large buffers |
| SRFI-64 | The whole Guile test suite (`tests/*.scm`) |
| SRFI-171 transducers | Allocation-free token / diagnostic pipelines |
| `(ice-9 match)` | The `apply-event` transition; LSP/agent message dispatch |
| `(ice-9 exceptions)` + `guard` | Per-fiber and per-extension isolation |
| promises (`delay`/`force`) | One-shot config and font initialization |
| parameters | `current-conn` / `current-buffer` / `current-keymap` / `current-theme` |

---

## 8 — Guile Module Inventory

| Module | File | Imports (key) | Key exports / responsibility |
|---|---|---|---|
| `(aetheric rms-ipc)` | `rms-ipc.scm` | `(system foreign)`, `(fibers channels)` | 18 ABI wrappers; `spawn-rms-reader`; fiber-aware drain (§7.3) |
| `(aetheric core)` | `core.scm` | `(ice-9 match)`, `(srfi srfi-1 9)` | `initial-state`, `apply-event` — pure transition |
| `(aetheric editor)` | `editor.scm` | `(srfi srfi-1 9 26)`, `(ice-9 match)` | buffer/hook public API; `current-buffer` parameter |
| `(aetheric astrolabe)` | `astrolabe.scm` | `(srfi srfi-1)`, `(ice-9 match)` | persistent-tree keymaps; first-run profile; Emacs/Vim/CUA |
| `(aetheric halo)` | `halo.scm` | `(ice-9 exceptions)` | `halo-require/unload`; per-extension `guard` isolation |
| `(aetheric oracle)` | `oracle.scm` | `(ice-9 match)` | `describe-key/function/variable/mode`, `apropos` |
| `(aetheric architect)` | `architect.scm` | `(ice-9 exceptions)` | live REPL buffer; evaluates against the running image |
| `(aetheric boxship)` | `boxship.scm` | `(srfi srfi-1)`, `(fibers channels)` | `boxship-install/update/remove/list/audit`; signed fetch |
| `(aetheric spectrum)` | `spectrum.scm` | `(srfi srfi-1)` | `theme-load/set/list`; Void default; Nova + Penumbra |
| `(aetheric ephemeris)` | `ephemeris.scm` | promises | `session-save/restore`; XDG dirs; `profile.scm` |
| `(aetheric lsp)` | `lsp.scm` | `(fibers ...)`, `(ice-9 match)` | fiber pair per server; stdio JSON-RPC ↔ channels |
| `(aetheric celestial)` | `celestial.scm` | `(srfi srfi-1)`, `(fibers ...)` | outliner/tasks/agenda/export; background fibers |
| `(aetheric nexus)` | `nexus.scm` | `(fibers ...)`, `(ice-9 match)` | git ops as fibers; status/commit/log/rebase/blame |
| `(aetheric construct)` | `construct.scm` | `(srfi srfi-1)` | Dired-style directory buffer |
| `(aetheric agent)` | `agent.scm` | `(fibers ...)`, `(ice-9 match)` | runtime, tool calling, ReAct (§9) |
| `(aetheric seraph)` | `seraph.scm` | `(ice-9 exceptions)` | guardrails: approval, sandbox, rate-limit, audit, kill |
| `guile/main.scm` | `main.scm` | `(fibers ...)` | `run-fibers` entry; spawn readers; `event-loop` (§7.4) |

---

## 9 — AI Agent Subsystem

All Guile, concurrent-functional, every side-effect gated through **Seraph**.

### 9.1 — `(aetheric agent)`

| Procedure | Responsibility |
|---|---|
| `ai-config` | `syntax-rules` config macro (§7.8) |
| `ai-provider` | Unified client record: Ollama / OpenAI-compatible / llama.cpp |
| `agent-chat` | One-shot chat; tokens stream onto a channel into `ai-mode` |
| `agent-run` | ReAct loop (below) — tail-recursive |
| `register-tool` / `execute-tool` | JSON-schema tools; `execute-tool` routes through Seraph |
| `ai-mode` | Major mode; streaming Markdown; `/model` `/reset` `/tools` |

The ReAct loop is tail-recursive and pure aside from the gated `execute-tool` effect:

```scheme
(define (agent-run goal)
  (let loop ((thoughts '()))
    (let* ((action (llm-reason goal thoughts))
           (result (execute-tool action)))      ; routed through Seraph
      (if (goal-achieved? result)
          result
          (loop (cons (list action result) thoughts))))))
```

Streaming uses the same CSP discipline as §7.4: a provider fiber issues the request and `put-message`s each token chunk onto `ai-ch`; the event loop folds chunks into `ai-mode` buffer state via `apply-event`.

### 9.2 — `(aetheric seraph)` — Guardrails

| Procedure | Guardrail | Default |
|---|---|---|
| `seraph-approve-edit` | Unified diff + Apply/Edit/Reject before any buffer mutation | ON |
| `seraph-sandbox-exec` | Shell/tools in `nomadic-mode` PTY or Halo namespace | Always |
| `seraph-rate-limit` | Per-model token budget + cooldown | Configurable |
| `seraph-sanitize-prompt` | Strip/escape user content in system prompts | Always |
| `seraph-audit-log` | Append to `$XDG_STATE_HOME/aetheric/agent.log` (ISO 8601 UTC Z) | Always |
| `agent-stop-all` | `M-x agent-stop-all` → SIGTERM model processes | Always |

**Invariant:** no agent edit reaches Stratum without `seraph-approve-edit`, unless a tool is explicitly pre-authorized in `init.scm`. Verified by `tests/agent_suite.scm` (SRFI-64).

### 9.3 — AI Task Map

| Phase | Tasks | Target |
|---|---|---|
| AI-1 Foundation | AE-AI-001 … 007 | 2026-Q3 (with Phase 3) |
| AI-2 Interactive Buffer | AE-AI-008 … 012 | 2026-Q4 |
| AI-3 Advanced Agentic | AE-AI-013 … 017 | 2027-Q1 |
| AI-4 Performance & Polish | AE-AI-018 … 022 | 2027-Q2 |

---

## 10 — Lints & Static Analysis

### 10.1 — Workspace Cargo.toml Lints

```toml
[workspace.lints.rust]
missing_debug_implementations = "warn"
unsafe_op_in_unsafe_fn        = "warn"
unused_lifetimes              = "warn"

[workspace.lints.clippy]
correctness  = { level = "warn", priority = -1 }
complexity   = { level = "warn", priority = -1 }
pedantic     = { level = "warn", priority = -1 }
perf         = { level = "warn", priority = -1 }
style        = { level = "warn", priority = -1 }
suspicious   = { level = "warn", priority = -1 }
undocumented_unsafe_blocks  = "warn"
unnecessary_safety_comment  = "warn"
```

### 10.2 — deny.toml

```toml
[licenses]
allow = ["AGPL-3.0-or-later","MIT","Apache-2.0","ISC","Unicode-DFS-2016",
         "OFL-1.1","BSD-2-Clause","BSD-3-Clause","Zlib","CC0-1.0"]
deny  = ["GPL-2.0","LGPL-2.0","AGPL-3.0"]

[advisories]
vulnerability = "deny"
unsound       = "deny"
unmaintained  = "warn"

[bans]
deny = [{ crate = "tokio", wrappers = ["aetheric-single-screw"] }]
```

### 10.3 — Guile Static Checks (`scripts/guile-lint.scm`)

- `guild compile -Wunbound-variable -Wunused-variable -Warity-mismatch` on every `.scm` (warnings fail CI).
- The linter rejects, per §7.9: `set!` in core modules; `define-macro` anywhere; `(touch …)` / `(join-thread …)` inside a fiber body; a blocking `(read …)` or `(usleep …)` inside a fiber; and any `pk` left in committed code.

---

## 11 — CI Pipeline

| Job | Command | Blocks merge? |
|---|---|---|
| Format | `cargo fmt --check` | Yes |
| Clippy | `cargo clippy -- -D warnings` | Yes |
| Build | `cargo build --release --all-targets` | Yes |
| Test (Rust) | `cargo test --workspace` | Yes |
| Renderer parity | `cargo test --test renderer_parity` | Yes (Phase 3+) |
| Guile compile | `guild compile -Wunbound-variable -Wunused-variable guile/**/*.scm` | Yes |
| Guile lint | `guile scripts/guile-lint.scm` (set! / define-macro / blocking-in-fiber) | Yes |
| Test (Guile) | `guile -s tests/run-tests.scm` (SRFI-64) | Yes |
| Core transitions | `guile tests/core_transitions.scm` (pure, no editor) | Yes |
| Fibers suite | `guile tests/fibers_suite.scm` | Yes |
| SS bridge smoke | `guile tests/ss_bridge_smoke.scm` | Yes |
| Celestial / Nexus | `guile tests/{celestial,nexus}_suite.scm` | Yes (Phase 3/4) |
| AI agent suite | `guile tests/agent_suite.scm` | Yes (AI-2+) |
| Deny | `cargo deny check` | Yes |
| Miri (SS only) | `cargo miri test -p aetheric-single-screw` | Yes |
| ThreadSanitizer | `RUSTFLAGS=-Zsanitizer=thread cargo test` | Yes |
| Bench regression | `cargo bench` (fail if > 10%) | Yes |
| WCAG contrast | `python3 scripts/wcag_contrast.py` | Yes |
| SPDX / AGENTS sync | `scripts/check_spdx.sh` + `check_agents_sync.sh` | Yes |
| Nix | `nix flake check` (flake + NixOS + Home Manager) | Yes |
| Commit signing | All commits show "Verified" | Yes |

> The AI agent suite must include a **Seraph approval-flow test**: an agent proposes an edit, and the test asserts the edit does not reach Stratum until approval is granted, and that `agent-stop-all` terminates model processes.

---

## 12 — Nix Packaging

One flake ships three artifacts; `nix flake check` validates all of them.

### 12.1 — Flake

`nix/flake.nix` exposes `packages.<system>.aetheric` (the `rms` binary + `aetheric-majestic` + `librms_ipc.so`, built with crane for the Rust crates and a Guile derivation for the `.scm` sources, AOT-compiled with `guild compile`), a `devShells.<system>.default` with the Rust toolchain, Guile 3.x, guile-fibers, and Cap'n Proto tooling, plus the two modules below.

### 12.2 — NixOS Module

`nix/nixos-module.nix` provides system-wide installation:

```nix
{ config, lib, pkgs, ... }:
{
  options.programs.aetheric.enable = lib.mkEnableOption "Aetheric editor";
  config = lib.mkIf config.programs.aetheric.enable {
    environment.systemPackages = [ pkgs.aetheric ];
    fonts.packages = with pkgs; [ jetbrains-mono noto-fonts noto-fonts-emoji ];
  };
}
```

### 12.3 — Home Manager Module

`nix/hm-module.nix` provides per-user, declarative configuration — the user versions their entire Aetheric setup. It installs the binaries into the user profile and generates `init.scm` and `profile.scm` under `$XDG_CONFIG_HOME/aetheric/` from typed options:

```nix
{ config, lib, pkgs, ... }:
let cfg = config.programs.aetheric; in {
  options.programs.aetheric = {
    enable = lib.mkEnableOption "Aetheric editor (Home Manager)";
    package = lib.mkPackageOption pkgs "aetheric" { };
    keybindingProfile = lib.mkOption {
      type = lib.types.enum [ "emacs" "vim" "cua" ]; default = "vim";
    };
    theme = lib.mkOption { type = lib.types.str; default = "void"; };
    extensions = lib.mkOption { type = lib.types.listOf lib.types.str; default = [ ]; };
    ai = {
      provider = lib.mkOption { type = lib.types.str; default = "ollama"; };
      defaultModel = lib.mkOption { type = lib.types.str; default = "llama3.2:3b"; };
    };
    extraConfig = lib.mkOption { type = lib.types.lines; default = ""; };
  };
  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."aetheric/profile.scm".text =
      "(use-modules (aetheric astrolabe))\n"
      + "(astrolabe/switch-profile '${cfg.keybindingProfile})\n";
    xdg.configFile."aetheric/init.scm".text = ''
      (use-modules (aetheric editor) (aetheric spectrum) (aetheric boxship) (aetheric agent))
      (theme-set! "${cfg.theme}")
      ${lib.concatMapStrings (e: "(boxship-require '${e})\n") cfg.extensions}
      (ai-config (default-model "${cfg.ai.defaultModel}")
                 (provider '${cfg.ai.provider})
                 (api-key (getenv "OPENAI_API_KEY"))
                 (base-url "http://localhost:11434"))
      ${cfg.extraConfig}
    '';
  };
}
```

Example user configuration:

```nix
programs.aetheric = {
  enable = true;
  keybindingProfile = "vim";
  theme = "void";
  extensions = [ "celestial" "nexus" ];
  ai = { provider = "ollama"; defaultModel = "llama3.2:3b"; };
};
```

---

## 13 — Release Process

1. Version bump: workspace `Cargo.toml`; `CHANGELOG.md` entry.
2. Tag: `git tag -s vX.Y.Z` (Ed25519 SSH signed).
3. Build Tier 1 tarballs for all 5 target triples via CI (each with `rms`, `librms_ipc.so`, Majestic).
4. Sign: `minisign -Sm aetheric-vX.Y.Z-<triple>.tar.gz` (Apogee key).
5. Publish signed tarballs + `.minisig` to GitHub Releases.
6. Update the Nix flake (and both modules) to the release hash; `nix flake check`.
7. Update the Boxship registry index; sign with the Apogee key.
8. (If AI-OQ-01 approved) publish the bundled GGUF artifact + signature separately.

---



---

*Copyright (c) 2026 Mohamed Hammad & Spacecraft Software. AGPL-3.0-or-later.*  
*Contact: Mohamed.Hammad@SpacecraftSoftware.org | <https://Aetheric.SpacecraftSoftware.org/>*
