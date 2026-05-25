---
title: "AGENTS.md — Aetheric Project Guidelines"
author: "Mohamed Hammad & Spacecraft Software"
date: 2026-05-25
version: "2.7"
document-id: AE-AGENTS-001
references: "AE-PRD-MASTER-001 v2.7, AE-IMPL-001 v2.7, AE-TODO-001 v2.7, guile-functional-concurrent (skill)"
license: AGPL-3.0-or-later
tags: [agents, guidelines, aetheric, spacecraft-software, safety, concurrency]
---

# AGENTS.md — Aetheric Project Guidelines (v2.7)

<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Copyright (c) 2026 Mohamed Hammad & Spacecraft Software -->

**Project:** Aetheric  **PRD:** AE-PRD-MASTER-001 v2.7  **Date:** 2026-05-25

This file is read automatically by Claude Code, Cursor, Grok, etc.
**Follow every rule strictly. Violations are rejected in code review.**

---

## 0. Repository State — Read This First

This repository (`/spacecraft-software/aetheric`) is currently a **design and specification repository**. It contains the product requirements, implementation plan, and task tracker for the Aetheric text editor. **No source code has been written yet.**

### Files that actually exist in this repository

| File | Purpose |
|---|---|
| `Aetheric-Master-PRD-v2.7.md` | Product Requirements Document (788 lines) — the authoritative spec |
| `IMPL-Aetheric-v2.7.md` | Implementation Plan (1003 lines) — Cargo workspace, module breakdown, Cap'n Proto schemas, Guile conventions, CI pipeline, Nix packaging |
| `TODO-Aetheric-v2.7.md` | Implementation TODO (477 lines) — 268 tasks across 5 phases + AI track; all marked `[ ]` not started |
| `AGENTS.md` | This file — agent guidelines |
| `AGENTS-Aetheric-v2.7.md` | Versioned backup of this file |
| `Font/Anta-Regular.ttf` | Project typeface (OFL-licensed) |
| `Font/OFL.txt` | Font license |
| `Logo/*.png`, `Logo/*.jpg` | Logo and branding assets |
| `Anta.zip` | Font archive |

### What does NOT exist yet

There is **no** `Cargo.toml`, no `src/` directory, no `.scm` source files, no `schemas/` directory, no `tests/`, no `nix/` flake, and no CI configuration. Everything described in §1–§6 below is the **planned architecture and coding standards** that must be followed as the 268 implementation tasks are executed.

When you (the agent) are asked to work on this project, your first step is to determine whether the task is:

1. **Editing a design document** (PRD, IMPL, TODO, or this file) — use the document structure and style already present in those files.
2. **Implementing source code** — follow the architecture, module layout, and conventions from `IMPL-Aetheric-v2.7.md` and the rules below. You will likely be creating files from scratch.
3. **Updating AGENTS.md** — keep it in sync with the PRD/IMPL as the project evolves.

**Target v1.0:** 2027-Q3. **Current phase:** Pre-implementation (Phase 1 scheduled for 2026-Q3).

---

## 1. Project Overview (Planned Architecture)

Aetheric is a next-generation, GPU-accelerated, Guile Scheme-extensible text editor. It runs as exactly two OS processes:

- **RMS Microkernel** *(Rust, headless)*: Stratum (rope), Nova (GPU renderer), Penumbra (TTY renderer), Morpheus (IPC), Orion (tasks), Lumen (logging). Never runs scripts.
- **Majestic** *(GNU Guile 3.0+)*: Astrolabe (keymaps), Halo (extensions), Oracle (help), Architect (REPL), Boxship (packages), Celestial (Org), Nexus (Git), **`(aetheric agent)` (AI runtime), Seraph (AI guardrails)**, LSP, config. Written pure-first + concurrent (Fibers/CSP) per §4.
- **Bridge:** `librms_ipc.so` built by **SingleScrew (SS)**; loaded via `(aetheric rms-ipc)`.

The authoritative architecture description lives in `Aetheric-Master-PRD-v2.7.md` §4 and `IMPL-Aetheric-v2.7.md` §1–§5.

### RMS — Triple Meaning
| | |
|---|---|
| **R**eal **M**ulti-core-multi-thread **S**imultaneity | The microkernel saturates all CPU cores; 60 Hz min / 120 Hz target |
| **R**ichard **M**. **S**tallman | A tribute to the creator of GNU Emacs, GPL, GNU, and the Free Software Foundation |
| **R**oyal **M**ail **S**hip | The White Star Line standard of engineering excellence |

SingleScrew is named after the single-screw steamships that preceded the great Royal Mail Ships — necessary, but operating with fewer safety margins.

### Canonical Component Names (v2.7)

> Core = **RMS Microkernel** (no separate codename). IPC broker = **Morpheus**. Bridge crate = **SingleScrew/SS** → `librms_ipc.so`. GPU renderer = **Nova**; TTY/TUI front-end = **Penumbra**. Terminal *buffer* = **Nomadic** (distinct from Penumbra). Package manager = **Boxship**. AI runtime = **`(aetheric agent)`**. AI guardrails = **Seraph**.

| Codename | Layer | Role |
|---|---|---|
| RMS Microkernel | Core (Rust) | Headless server: text, render, IPC |
| Stratum | RMS | Persistent rope (crop crate, CoW B-tree) |
| Nova | RMS | GPU renderer (wgpu + cosmic-text + glyphon) |
| Penumbra | RMS | Terminal (TTY/TUI) renderer (crossterm + ratatui) |
| Morpheus | RMS | IPC broker — Cap'n Proto / Unix socket |
| Orion | RMS | tokio task pool |
| Lumen | Both | Structured logging |
| SingleScrew (SS) | Bridge cdylib | `aetheric-single-screw` → `librms_ipc.so` — **ONLY unsafe** |
| Majestic | Orchestration | Guile executive runtime (pure + Fibers/CSP) |
| Astrolabe | Majestic | Keymaps — Emacs/Vim/CUA first-run choice |
| Halo | Majestic | Extension sandbox |
| Oracle | Majestic | Help & introspection (C-h prefix) |
| Architect | Majestic | Live REPL buffer |
| Boxship | Majestic | Package manager (containership carrying package cargo) |
| Spectrum | Majestic | Theme system |
| Ephemeris | Majestic | Session + XDG state |
| Celestial | Majestic ext | Org-mode successor |
| Nexus | Majestic ext | Git interface |
| `(aetheric agent)` | Majestic ext | AI agent runtime: tool calling, memory, planning |
| Seraph | Majestic | AI guardrails: approval, sandbox, rate-limit, audit, kill switch |
| Nomadic | Majestic buffer | Embedded PTY terminal (SS Nomadic tender) |
| Constellation | Shared | Cap'n Proto schemas |
| Apogee | Both | Signed releases |

### Core Invariants (NEVER violate)

1. Majestic crash/hang must **NEVER** affect the RMS Microkernel.
2. **Exactly two unsafe sites** (both in SingleScrew): FFI boundary + POSIX pipe(2). **Never add more.** The `aetheric-agent-ffi` option is explicitly forbidden (red line per IMPL v2.7); no third unsafe boundary will ever be introduced. Update to this invariant requires a major version bump and explicit PRD change.
3. All text is valid UTF-8. All byte offsets are valid char boundaries.
4. Timestamps: ISO 8601 UTC Z (`jiff` crate). `chrono::Local` is **forbidden**.
5. WCAG 2.1 AA ≥ 4.5:1 on all themed elements — **GPU (Nova) and TTY (Penumbra)**.
6. Interactive thread **never** blocks. Every blocking operation lives in a fiber that communicates only by channel.
7. **No shared mutable state in Majestic.** Fibers share data only via channels; state advances only through pure transitions.
8. **Renderer transparency.** Majestic must behave identically under Nova and Penumbra; no front-end-specific logic (`renderer_parity` test).
9. All tokio channels (RMS) and Fibers channels feeding RMS are **bounded** (default 256).
10. **No PFA requirement.** `--offline` disables Boxship fetches, Apogee update checks, and cloud AI (local models keep working).
11. **AI edit approval (Seraph):** no agent-proposed edit reaches Stratum without `seraph-approve-edit`, unless a tool is explicitly pre-authorized in `init.scm`. All AI tools require an allow-list entry in `init.scm`.

---

## 2. Build, Test & Verification (Planned Commands)

These commands do not run yet because the source tree does not exist. They are the target toolchain defined in `IMPL-Aetheric-v2.7.md` §10–§11 and must be used once scaffolding is complete.

### Rust / RMS
```nu
cargo check
cargo clippy -- -D warnings
cargo fmt -- --check
cargo test
cargo test --all-features
cargo test --test renderer_parity            # Nova vs Penumbra layout
cargo miri test                              # SingleScrew unsafe only
RUSTFLAGS="-Zsanitizer=thread" cargo test
cargo bench
```

### Guile / Majestic
```nu
guild compile -Wunbound-variable -Wunused-variable guile/aetheric/*.scm
guile scripts/guile-lint.scm                 # set! / define-macro / blocking-in-fiber
guile -s tests/run-tests.scm                 # SRFI-64 suite
guile tests/core_transitions.scm             # pure apply-event (no editor)
guile tests/fibers_suite.scm                 # channel select / isolation
guile tests/ss_bridge_smoke.scm
guile tests/agent_suite.scm                  # AI agent + Seraph approval flow
```

### Nix
```nu
nix flake check                              # flake + NixOS module + Home Manager module
```

### Full gate
```nu
cargo clippy -- -D warnings; and cargo fmt -- --check; and cargo test; and guile -s tests/run-tests.scm
```

---

## 3. Mandatory Rules

- **Plan first** for tasks > ~50 lines or architectural changes.
- Small, verifiable increments.
- No `dbg!`, `println!`, commented code, or TODOs without an issue link.
- Update tests + docs + schemas together.
- **Constellation ordinals (`@N`) are permanent — never reuse.**

### Rust Rules
- Memory safety is the highest priority (Standard §3).
- Only two unsafe blocks allowed — already documented. **Never add more.**
- No `.unwrap()` / `.expect("todo")` in production. Use `?` with `thiserror`/`anyhow`.
- Prefer borrowing over cloning; `Arc`/`ArcSwap` for snapshots.
- All inter-task communication: bounded `tokio::sync::mpsc` (bound = 256).
- All logging: `tracing` with named events `<component>.<operation>.<state>`.
- **Morpheus crate: `#![deny(unsafe_code)]` is absolute.**
- Nova and Penumbra implement the same `Renderer` trait; nothing else depends on which is active.
- Metric units only (millimetres for dimensions).

---

## 4. Guile / Majestic Rules

Majestic is **pure first** and **concurrent via message-passing**, per the `guile-functional-concurrent` skill (canonical refs: `functional.md`, `concurrent.md`) and `IMPL-Aetheric-v2.7.md` §7. Assume Guile 3.x + guile-fibers.

### 4.1 Functional Core (Non-Negotiable)
- State is an **immutable value**; change is the **pure transition** `(apply-event state event) → state'` in `(aetheric core)`, dispatched with `(ice-9 match)`. The core path uses **no `set!`** — mutation lives only at the edges (SS bridge, logging, output).
- Records via **SRFI-9** with functional updaters; omit setters to keep fields immutable. Keymaps are **persistent prefix trees** (rebind returns a new keymap).
- **Proper tail calls are mandatory** for any loop over unbounded data — named `let` with an accumulator. Prefer SRFI-1 combinators (`fold`, `filter-map`, `partition` + `let-values`) over hand-rolled recursion. Note `fold` calls `proc` with `(element accumulator)` — element first.
- SRFI-41 streams for very large buffers; SRFI-171 transducers for hot, allocation-sensitive token/diagnostic pipelines; SRFI-26 `cut`/`cute` for partial application.

### 4.2 Event Loop (Non-Negotiable)
- `run-fibers` is the **single entry point** (`guile/main.scm`); spawn all fibers inside it.
- The interactive loop is a CSP **select** via `choice-operation` over the RMS, LSP, AI, and frame-tick channels; it folds each event through `apply-event` in tail position and **never blocks** on a single source. Channels are unbuffered rendezvous (natural backpressure). Target: < 5 ms key-to-CoreCommand.

### 4.3 The SingleScrew ↔ Fibers Bridge (Critical)
- `rms_recv()` **blocks** — never call it from a fiber (a blocking call stalls every fiber on the scheduler thread). The dedicated **`spawn-rms-reader`** fiber waits on `rms_fd()` wrapped as a suspendable port (`get-u8` suspends only that fiber), then drains with non-blocking `rms_try_recv()` onto a channel. Everything else speaks channels. `rms_send()` returns quickly and may be issued from the dispatch fiber.

### 4.4 Concurrency Model (Decision Tree)
- I/O-bound (RMS intake, LSP, AI streaming, git, network) → **Fibers + channels**.
- CPU-bound → **delegate to RMS via OrchCommand**, else a POSIX thread / `ice-9 futures` that reports back **over a channel**. Never `(touch …)` or `(join-thread …)` on a fiber.
- Composable select/timeout → `choice-operation` + `sleep-operation`. Lazy compute-once → promises (`delay`/`force`). Per-task context → **parameters** (`current-conn`, `current-buffer`, `current-keymap`, `current-theme`); fiber-local, no globals.
- The rare shared-state thread path: `with-mutex` only (never manual lock/unlock), tiny critical sections, never block while holding a lock, one global lock order, re-check condition variables in a loop.

### 4.5 Configuration
- All state in live Guile data from `init.scm`. Redefine anything at the Architect REPL, no restart. Config: `$XDG_CONFIG_HOME/aetheric/`. First-run profile: `profile.scm`. AI config: `ai-config`. Config DSL is **hygienic `syntax-rules`** (never `define-macro`; `syntax-case` only to introduce identifiers deliberately).

### 4.6 Extension System (Halo)
- Fresh module namespace per extension; shared state via `(aetheric editor)` only; must export `(on-unload)`.
- `guard` / `with-exception-handler` everywhere — **never let an extension crash Majestic**; `dynamic-wind` guards resource cleanup (PTY, sockets, LSP processes).
- Direct `(aetheric rms-ipc)` requires an explicit privilege grant in `init.scm`.

### 4.7 Keymap (Astrolabe)
- Persistent-tree keymaps; closures may return futures. First-run `[E]macs [V]im [C]UA` → `profile.scm`. `(astrolabe/switch-profile 'emacs)`. A minor mode is a keymap→keymap function; profiles are layerable and all three must remain functional.

### 4.8 Buffer Model
```
buffer-id (u32) | major-mode (one) | minor-modes (list) | vars (immutable map)
```
Mode state is strictly buffer-local. Terminal buffer mode: `nomadic-mode` (Nomadic). AI buffer mode: `ai-mode`. Buffers behave identically under Nova and Penumbra.

### 4.9 LSP, Boxship, Errors
- LSP servers are child processes, one reader/writer **fiber pair** each (stdio JSON-RPC ↔ channels); diagnostics → `setDiagnostics` CoreCommand. Syntax/indent/bracket: Tree-sitter inside RMS — never via LSP.
- Boxship: declarative in `init.scm`; all network explicit + signed (Apogee Ed25519 + SHA-256); `--offline` hard-disables fetches.
- Errors: `guard` everywhere; log via Lumen (ISO 8601 UTC Z). Expected failures return `(values 'ok x)` / `(values 'error reason)`; exceptions for the genuinely exceptional.

### 4.10 AI Agents — `(aetheric agent)` + Seraph (Critical)
- **Safety first.** Every agent edit goes through `seraph-approve-edit` (unified diff + Apply/Edit/Reject), default ON. No silent changes.
- **Tools are allow-listed** in `init.scm` (`edit-buffer`, `insert-text`, `run-shell`, `celestial-create-task`, `nexus-commit`, …). Never execute unapproved code.
- **Sandboxed execution** via `seraph-sandbox-exec` in a `nomadic-mode` PTY or a Halo namespace.
- **Prompt-injection defense** (`seraph-sanitize-prompt`), **audit log** (`seraph-audit-log` → `$XDG_STATE_HOME/aetheric/agent.log`), **kill switch** (`M-x agent-stop-all`).
- **Local-first.** Default to local models; cloud is opt-in with a cost estimate; `--offline` disables cloud. Streaming into `ai-mode` < 100 ms, off the interactive thread. The ReAct loop is tail-recursive; each `execute-tool` is routed through Seraph.

### 4.11 Style & Pitfalls (enforced by `guile-lint.scm`)
- `kebab-case`; predicates `?`; the rare mutator `!`. Avoid `set!`. Named `let`, not `do`.
- Fiber-aware I/O only inside fibers (`(fibers timers)` `sleep`, suspendable ports); never a blocking C call or `ice-9 threads` sleep on a fiber.
- One `'done` sentinel per consumer to terminate a channel loop. Comments: `;` `;;` `;;;` `;;;;`. `pk` is the only debug print and must not survive into a commit.
- Majestic cold-start < 300 ms (target < 150 ms warm).

---

## 5. Recommended Agent Workflow

1. Read `AGENTS.md` + relevant PRD/IMPL sections + nearby code + `schemas/`.
2. Propose a plan (especially for event-loop, IPC, schema, renderer, extension, or AI-agent changes).
3. Implement in small, verifiable steps.
4. Run the full gate after each step (include `renderer_parity`, `guile-lint`, and `agent_suite` where relevant).
5. Update tests + docs + schemas together.
6. Final review: isolation preserved? Interactive thread non-blocking? Renderer-agnostic? No new unsafe? Pure core (no `set!`)? Seraph approval intact for any AI edit path?

---

## 6. Priority Order

1. **Process isolation** — Majestic crash ≠ RMS Microkernel crash
2. **Interactive thread never blocks** — < 5 ms
3. **Memory safety** (Rust) + **extension isolation** (Halo) + **AI safety** (Seraph)
4. **Clarity and live reconfigurability** over cleverness
5. **Spacecraft Software Standard** compliance

---

## 7. Reference Documents

When working on this project, consult these files in the repository root (all are GitHub-Flavored Markdown):

| Document | ID | What it covers |
|---|---|---|
| `Aetheric-Master-PRD-v2.7.md` | AE-PRD-MASTER-001 | Vision, architecture, component registry, AI agents, security model, distribution |
| `IMPL-Aetheric-v2.7.md` | AE-IMPL-001 | Cargo workspace layout, module breakdown per crate, Cap'n Proto schemas, Guile conventions, CI pipeline, Nix packaging, release process |
| `TODO-Aetheric-v2.7.md` | AE-TODO-001 | 268 implementation tasks across 5 phases + AI track; target v1.0 = 2027-Q3 |

Keep `AGENTS.md` in sync with the PRD and IMPL as the project evolves.

---

*AGENTS.md is the single source of truth for agent behaviour on Aetheric.*
*Copyright (c) 2026 Mohamed Hammad & Spacecraft Software · GNU AGPL v3.0-or-later*
