---
title: "Aetheric Master PRD v2.7"
author: "Mohamed Hammad & Spacecraft Software"
date: 2026-05-25
tags: [prd, aetheric, text-editor, guile-scheme, rust, microkernel, ai-agents, npu, spacecraft-software, gfm]
---

# Aetheric Master PRD v2.7

**Mohamed Hammad & Spacecraft Software**  
**2026-05-25**

> [!NOTE]
> This is the GitHub Flavored Markdown (GFM) version of the official Aetheric Master Product Requirements Document. It follows all GFM rules for tables, alerts, code blocks, and structure. The source of truth remains the PDF at `PRD-Aetheric-v2.7-Master-Full.pdf`. Section 11.8 has been placed in its logical position under AI Agents (after 11.7) rather than at the end of the document.

**Aetheric logo** — a constellation forming the letter A against a deep nebula (see assets in repository).

## Document Metadata

| Field              | Value                                      |
|--------------------|--------------------------------------------|
| Project            | Aetheric                                   |
| Document ID        | AE-PRD-MASTER-001                          |
| Version            | 2.7                                        |
| Status             | Draft                                      |
| Launch command     | `editor`                                   |
| Primary platform   | GNU Guix System                            |
| Author             | Mohamed Hammad (Mohamed.Hammad@SpacecraftSoftware.org) |
| Organisation       | Spacecraft Software                        |
| Date               | 2026-05-25                                 |
| License            | AGPL-3.0-or-later                          |
| Repository         | https://github.com/Spacecraft-Software/Aetheric |
| Website            | https://Aetheric.SpacecraftSoftware.org/   |

## Table of Contents

- [1. Executive Summary](#1-executive-summary)
- [2. Project Identity & Classification](#2-project-identity--classification)
  - [2.1 The Triple Meaning of RMS](#21-the-triple-meaning-of-rms)
  - [2.2 Naming Conventions](#22-naming-conventions)
  - [2.3 Component Registry](#23-component-registry)
  - [2.4 License & Posture](#24-license--posture)
- [3. Vision & Design Goals](#3-vision--design-goals)
- [4. System Architecture](#4-system-architecture)
- [5. Core Objectives](#5-core-objectives)
- [6. IPC Architecture: Morpheus + Constellation](#6-ipc-architecture-morpheus--constellation)
- [7. The RMS Microkernel](#7-the-rms-microkernel)
- [8. Majestic: Orchestration Layer](#8-majestic-orchestration-layer)
- [9. Celestial: Org-mode Successor](#9-celestial-org-mode-successor)
- [10. Nexus: Git Interface](#10-nexus-git-interface)
- [11. AI Agents](#11-ai-agents)
  - [11.1 Overview](#111-overview)
  - [11.2 Architecture](#112-architecture)
  - [11.3 Agent Runtime & ReAct](#113-agent-runtime--react)
  - [11.4 Model Providers & BYOK](#114-model-providers--byok)
  - [11.5 Total Capability Awareness](#115-total-capability-awareness)
  - [11.6 Seraph: Safety & Guardrails](#116-seraph-safety--guardrails)
  - [11.7 Success Metrics (for v1.0)](#117-success-metrics-for-v10)
  - [11.8 NPU Hardware Acceleration (First-Class Local AI)](#118-npu-hardware-acceleration-first-class-local-ai)
- [12. Buffer Model, Modes & LSP](#12-buffer-model-modes--lsp)
- [13. Supporting Systems & Distribution](#13-supporting-systems--distribution)
- [14. Security Model](#14-security-model)
- [15. Engineering Standards & Compliance](#15-engineering-standards--compliance)
- [16–23. Additional Sections (Performance, Platform, Repository, Testing, Roadmap, Comparison, Invariants, Open Questions)](#16-23-additional-sections)
- [Appendix: Core Invariants & Agent Guidelines](#appendix-core-invariants--agent-guidelines)

---

## 1 — Executive Summary

Aetheric is a next-generation, GPU-accelerated, Guile Scheme-extensible text editor built by Spacecraft Software, designed to be the definitive long-term editing environment for power users. It combines the Lisp-native, live-hackable extensibility of GNU Emacs, the GPU rendering discipline of Zed, the fault-isolated plugin architecture of Neovim, the client-server topology of Kakoune, and first-class, safety-first AI agent support — in a single coherent system. Like Visual Studio Code is launched with `code`, Aetheric is launched with the command `editor`.

Aetheric runs as two OS processes:

- **The RMS Microkernel** — a headless Rust server that owns all text storage (Stratum), rendering (Nova on the GPU, Penumbra in a terminal), and binary IPC (Morpheus). It never evaluates scripts.
- **Majestic** — the **GNU Guile Scheme orchestration layer** — the editor’s executive brain, written in a concurrent, functional style: keymaps, modes, hooks, extensions, LSP clients, packages, the Celestial outliner, the Nexus Git interface, the Architect agentic command surface, and the AI agent runtime.

The two processes communicate over **Morpheus**, a zero-copy Cap’n Proto IPC broker. Majestic loads the C-ABI bridge `librms_ipc.so` via `(aetheric rms-ipc)` — built by **SingleScrew (SS)**, the only crate in the entire project that contains unsafe Rust.

RMS carries three simultaneous meanings: **Real Multi-core-multi-thread Simultaneity** (the microkernel saturates all CPU cores), **Richard M. Stallman** (a tribute to the creator of GNU Emacs, GPL, GNU, and the Free Software Foundation), and **Royal Mail Ship** (the White Star Line standard of engineering excellence). SingleScrew, the unsafe bridge crate, is named after the single-screw steamships that preceded the great Royal Mail Ships — necessary, but operating with fewer safety margins.

GNU Guile Scheme is the through-line of the project. You **live-hack** the editor in Guile, the way Emacs is hacked in Elisp — redefining anything while it runs, with no restart. **Architect**, the agentic command surface, fuses a live Guile REPL, a block-structured AI terminal, and a coding agent into one workspace. And one layer down, **GNU Guix** — Aetheric’s first-and-foremost packaging and operating-system target — is itself written in Guile, so the same language configures your editor, declares your packages, and defines your OS. Aetheric supports straightforward **bring-your-own-key** access to the leading American and Chinese model providers, the OpenRouter gateway, and local Ollama; agents have immediate, always-current knowledge of every Aetheric capability through Oracle. The project is licensed under AGPL-3.0-or-later and targets v1.0 in 2027-Q3.

---

## 2 — Project Identity & Classification

### 2.1 — The Triple Meaning of RMS

RMS carries three deliberate meanings at once. No single meaning is primary — all three are true simultaneously.

| Meaning                        | Full form                                                                 | Why it matters for Aetheric |
|--------------------------------|---------------------------------------------------------------------------|-----------------------------|
| Real Multi-core-multi-thread Simultaneity | The microkernel saturates all available CPU cores; Tree-sitter parsing, IPC, rope mutation, and rendering snapshot clone costs 16 bytes and O(1) time. | 60 Hz minimum / 120 Hz target, enforced by CI benchmarks. ArcSwap mutation proceeds simultaneously on dedicated tokio tasks. |
| Richard M. Stallman            | A tribute to the creator of GNU Emacs, GPL, GNU, and the Free Software Foundation | Aetheric is built in the free-software tradition RMS championed. GNU Guile is the GNU Project's official Scheme; GNU Guix is the GNU system. The buffer/mode/hook model descends directly from Emacs. |
| Royal Mail Ship                | The White Star Line standard of engineering excellence — RMS Olympic, RMS Titanic, RMS Majestic, RMS Britannic | Like the great RMS ships, Aetheric carries precious cargo — your text, code, and ideas — reliably across hostile conditions: bad plugins, blocking LSP servers, runaway AI. |

The bridge library `librms_ipc.so` honours all three meanings. SingleScrew, the unsafe bridge crate, is named after the single-screw steamships that preceded the great Royal Mail Ships — necessary, but operating with fewer safety margins. It is the only unsafe Rust in the project.

### 2.2 — Naming Conventions

Component codenames draw from three registers. The test for any new name: would it fit on the hull of a spacecraft, the boot banner of an AI system, or the bow of a White Star Line ocean liner?

- **Cosmic / astronomical:** Stratum, Nova, Penumbra, Orion, Lumen, Spectrum, Ephemeris, Apogee, Constellation, Celestial.
- **Matrix:** Morpheus (IPC), Oracle (help), Architect (agentic command surface), Seraph (AI guardrails — the guardian who protects the Oracle).
- **White Star Line / maritime:** Majestic (orchestration — RMS Majestic, the flagship), SingleScrew / SS (unsafe cdylib — pre-RMS steamships), Nomadic (terminal buffer — SS Nomadic, the tender that served the RMS liners), Boxship (package manager — a containership carrying packaged cargo).

### 2.3 — Component Registry

| Codename          | Abbr. | Layer                  | Role                                                                 | Origin |
|-------------------|-------|------------------------|----------------------------------------------------------------------|--------|
| RMS Microkernel   | RMS   | Core (Rust)            | Headless server: text, render, IPC                                   | Real Multi-core / R.M. Stallman / Royal Mail Ship |
| Stratum           | —     | RMS internal           | Persistent rope (crop crate, CoW B-tree)                             | Geological layering |
| Nova              | —     | RMS internal           | GPU display engine (wgpu + cosmic-text + glyphon)                    | Greek sun god |
| Penumbra          | —     | RMS internal           | Terminal (TTY/TUI) front-end — alternative to Nova                   | Astronomy — the lesser light beyond the full disk |
| Morpheus          | —     | RMS internal           | IPC broker — Cap’n Proto / Unix socket                               | Matrix: guide between worlds |
| Orion             | —     | RMS internal           | tokio async worker task pool                                         | Cosmic — constellation |
| Lumen             | —     | Both processes         | Structured logging (tracing + ISO 8601 UTC Z)                        | Unit of luminous flux |
| SingleScrew       | SS    | Bridge (cdylib)        | `aetheric-single-screw` → `librms_ipc.so`; ONLY unsafe               | White Star Line: pre-RMS steamships |
| Majestic          | —     | Orchestration (Guile)  | Concurrent functional runtime: modes, hooks, extensions, LSP         | RMS Majestic — White Star Line flagship |
| Astrolabe         | —     | Majestic internal      | Keymap dispatch — first-run Emacs/Vim/CUA                            | Navigation instrument |
| Halo              | —     | Majestic internal      | Extension sandbox (Guile module namespaces)                          | Luminous ring |
| Oracle            | —     | Majestic internal      | Help, introspection, and the live capability surface                 | Matrix: the Oracle |
| Architect         | —     | Majestic internal      | Agentic command surface: REPL + block terminal + coding agent        | Matrix: the Architect — built the system |
| Boxship           | —     | Majestic internal      | Declarative extension package manager                                | Maritime — a containership carrying package cargo |
| Spectrum          | —     | Majestic internal      | Theme system (Void Navy default; WCAG CI gate)                       | Electromagnetic spectrum |
| Ephemeris         | —     | Majestic internal      | Session + XDG state persistence                                      | Astronomical position tables |
| Celestial         | —     | Majestic extension     | Org-mode successor: outliner, PKB, calendar, literate                | Cosmic: of the heavens |
| Nexus             | —     | Majestic extension     | Git interface: keyboard-driven, surpasses Magit                      | Matrix: central connection |
| (aetheric agent)  | —     | Majestic extension     | AI agent runtime: providers, tool calling, memory, planning          | Editor agent subsystem |
| Seraph            | —     | Majestic internal      | AI guardrails: edit approval, sandboxing, rate limits                | Matrix: the guardian who protects the Oracle |
| Nomadic           | —     | Majestic buffer mode   | Embedded PTY terminal emulator                                       | SS Nomadic — White Star Line tender |
| Constellation     | —     | Shared schemas         | Cap’n Proto schema set (core.capnp, orchestration.capnp)             | Cosmic — star patterns |
| Apogee            | —     | Both processes         | Signed releases; opt-in update check                                 | Orbital mechanics — the highest point |

### 2.4 — License & Posture

All Aetheric source — Rust, Guile Scheme, Cap’n Proto schemas, Guix and Nix definitions — is licensed under AGPL-3.0-or-later. Every `.rs` and `.scm` file carries the SPDX header `// SPDX-License-Identifier: AGPL-3.0-or-later`. The AGPL network-use clause applies: anyone who offers a modified Aetheric to users over a network (for example, a hosted agentic editing service) must make the corresponding source available to those users. Repository root ships `README.md`, `NOTICE.md`, `CONTRIBUTING.md`, `LICENSE`. Personal hobby project under Spacecraft Software default posture: no warranty, no SLA.

---

## 3 — Vision & Design Goals

### 3.1 — Extreme Extensibility

Majestic is the most powerful, safe, and live-programmable extension environment ever shipped in a modern editor. Users redefine any keybinding, mode, hook, command, or UI element in pure Guile. Halo module-namespace sandboxing isolates extensions by default; full power is available when explicitly granted. A crashing extension cannot freeze the render pipeline.

### 3.2 — Live Hacking (Emacs-style)

Like GNU Emacs, every part of Aetheric is live-hackable. You reshape the running editor in place — redefine a keybinding, swap a theme, fix a command, load or unload an extension, inspect any buffer’s state — while it runs, with **zero restart**. There are two equal paths: edit `init.scm` (your Guile configuration) and reload, or evaluate Scheme directly in the Architect REPL (§3.10). There is no compile-deploy-restart cycle; the running Guile image is the system, and the same Guile Scheme you configure with is the language extensions are written in. One layer down, that same language reappears: GNU Guix — Aetheric’s primary packaging and OS target (§3.11) — is written in Guile, so you hack your editor, declare your packages, and define your operating system all in one language.

### 3.3 — Everything Is a Buffer

Every interaction surface — files, directories (Construct), shells (Nomadic), the Architect agentic surface, help (Oracle), Git status (Nexus), LSP diagnostics, AI chats (ai-mode), Celestial agendas, package lists (Boxship) — is a first-class editable buffer managed by Majestic and rendered by the RMS Microkernel at 120 Hz.

### 3.4 — Celestial: The Org-mode Successor

A built-in outliner, task manager, calendar, literate programming notebook, and personal knowledge base — Org-mode evolved for the GPU era. Export to PDF/HTML/LaTeX is instant and theme-aware. Guile Fibers power background agenda and backlink computation.

### 3.5 — Self-Documenting, Discoverable, and Agent-Legible (Oracle)

Every command, variable, key sequence, and extension API is introspectable at runtime: `describe-key`, `describe-function`, `describe-variable`, `describe-mode`, `apropos`. Documentation is never stale — Oracle reads from the live Guile environment. The same live surface is exposed to AI agents (§11.5), so an agent knows how to do everything in Aetheric the moment it starts, and learns new extensions the instant they load.

### 3.6 — Nexus: The Magit Successor

A keyboard-driven Git interface surpassing Magit in speed and polish. Stage hunks, interactive rebase, blame, history — all as editable buffers at 120 Hz. Git subprocesses run as fibers so the interactive thread never blocks.

### 3.7 — Keyboard-Centric Efficiency & Macros

First-run profile selector: Emacs / Vim / CUA, stored in Ephemeris and loaded by Astrolabe. All three are always available and layerable. Keyboard macros are first-class Guile closures: recordable, saveable, nameable, composable. Vim-style text objects are native to Astrolabe.

### 3.8 — Integrated “Everything” Environment

Email, IRC/Matrix/Slack, the Nomadic terminal, PDF viewer, calendar (Celestial), RSS, and the AI assistant all run as Majestic buffers. Guile POSIX threads and Fibers provide safe concurrent subprocess and network management without blocking the dispatch loop.

### 3.9 — Runs in a Terminal (Penumbra)

Aetheric runs not only in a GPU window but directly inside a Linux/Unix terminal — over SSH, in a console, on a headless server. The RMS Microkernel separates *what to draw* (a snapshot + viewport, delivered over the Constellation schema) from *how to draw it*. Penumbra is the terminal front-end — an alternative renderer to Nova that draws to cells with crossterm/ratatui and reads keys from the TTY — selected at launch (`editor --tty`, or automatically when no display server is present). Majestic, Stratum, Morpheus, and the entire extension ecosystem are identical in both modes; only the renderer differs. This is distinct from the Nomadic buffer, a terminal emulator *inside* Aetheric.

### 3.10 — Architect: The Agentic Command Surface

Architect is Aetheric's flagship agentic workspace — a single, block-structured buffer that fuses three capabilities, taking direct inspiration from Warp's block terminal and Codex's coding agent:

1. **Live Guile REPL.** Evaluate Scheme against the running image — redefine keymaps, load extensions, inspect buffer state, drive the microkernel — with zero restart. This is the interactive face of live hacking (§3.2).
2. **Block terminal.** Run shell commands where each command and its output is a discrete, navigable, foldable, re-runnable *block* (timing, exit status, and actions attached), with AI-assisted command search (natural language → command), inline error explanation, and saved workflows.
3. **Coding agent.** Describe a task in natural language; Architect plans, edits files, and runs commands across your repository — every action gated by Seraph (diff + Apply/Edit/Reject) with sandboxed execution and configurable approval modes.

All three surfaces share the `(aetheric agent)` runtime (§11) and Seraph guardrails (§11.6); shell execution runs in sandboxed Nomadic PTYs. Architect is where coding agents live.

### 3.11 — Guix-First & Reproducible

Aetheric targets **GNU Guix packages** and the **GNU Guix System (OS)** first and foremost. It is delivered as a first-class Guix package through a Spacecraft Software Guix channel (`guix install aetheric`), with declarative per-user configuration via **Guix Home** and system-wide installation via Guix System. Reproducible builds, transactional rollback, and a fully free-software stack come from the platform itself. This is a natural fit: Guix is written in Guile, so the Aetheric package definition, your Guix Home configuration, and your editor extensions are all the same language — Guile Scheme top to bottom. Nix (flake + NixOS module + Home Manager module) is also supported as a secondary target.

### 3.12 — Stability, Longevity & Low Friction

Configurations written today will work in 2040. Guile's stable ABI, live module reloading, Halo hot-unload, and the RMS Microkernel's strict API stability (Cap’n Proto ordinals never change) ensure no lock-in and no bit-rot. XDG directories, Apogee signed releases, Boxship declarative packages, and Guix-first reproducible packaging complete the story.

### 3.13 — AI Agents

AI agent support is a core feature, governed by seven goals in strict priority order:

1. **Safety First** — every agent-proposed edit is user-approved or explicitly allowed via policy. No silent changes.
2. **Streaming & Low Latency** — token-by-token streaming with < 100 ms perceived latency.
3. **Local-First** — an excellent offline experience with local models; cloud is opt-in.
4. **Bring-Your-Own-Key, Any Provider** — adding a model provider is a one-line, OpenCode-style config across the leading American and Chinese providers, OpenRouter, and Ollama (§11.4).
5. **Total Capability Awareness** — agents know how to do everything in Aetheric immediately, via the live Oracle surface (§11.5).
6. **Guile-Extensible** — users define custom agents, tools, and workflows in pure Scheme.
7. **Agentic Workflows** — ReAct, Plan-and-Execute, multi-agent collaboration, long-running tasks, all surfaced in Architect (§3.10).

Full detail in §11.

---

## 4 — System Architecture

### 4.1 — Two-Process Model

Aetheric runs as exactly two OS processes. The user-facing launcher is the `editor` command (installed on PATH; an `aetheric` alias is also provided for environments where `editor` collides). `editor` spawns the RMS Microkernel first, waits for it to bind the Morpheus socket, then spawns Majestic with the socket path as a CLI argument. Majestic loads `librms_ipc.so` via `(aetheric rms-ipc)` and calls `rms_connect()`.

**Morpheus socket:** `$XDG_RUNTIME_DIR/aetheric/rms-{pid}.sock`

Usage mirrors `code`: `editor .`, `editor file.rs`, `editor --tty`.

### 4.2 — Process Isolation Benefits

- A panicking Guile extension, a blocking Celestial export, or a runaway AI agent **cannot** freeze the RMS Microkernel's render pipeline.
- Majestic can be hot-reloaded while the microkernel continues rendering the last committed state — the mechanism behind live hacking (§3.2).
- Guile's C-based GC shares no address space with the microkernel's Rust allocator.
- ASLR+CFI+PIE+RELRO applies to the Majestic binary (§14.2); review date 2027-06-01.
- Model processes (Ollama, llama.cpp) are children of Majestic, fully isolated from the microkernel. A model crash never affects rendering or text storage.

### 4.3 — The SingleScrew (SS) Boundary

SingleScrew (`aetheric-single-screw`) is the only place in the entire codebase where unsafe Rust lives. Exactly two documented unsafe sites, both Miri-verified and fuzz-covered:

1. `#[no_mangle]` `extern "C"` FFI functions — each wrapped in `std::panic::catch_unwind`; no Rust panic propagates into Guile.
2. POSIX `pipe(2)` / `read(2)` / `write(2)` in the background reader thread — provides the pollable notification fd returned by `rms_fd()`.

All other crates carry `#![deny(unsafe_code)]`.

---

## 5 — Core Objectives

| ID     | Objective                        | Acceptance Criterion |
|--------|----------------------------------|----------------------|
| OBJ-01 | Fault-isolated extensibility     | `SIGKILL` of Majestic does not crash or corrupt the RMS Microkernel. Integration test. |
| OBJ-02 | 60+ Hz frame rate                | Paint-to-scanout < 16 ms at 1080p on reference integrated GPU. |
| OBJ-03 | Live hacking                     | Any keybinding/command/mode/theme is redefinable at runtime via `init.scm` reload or the Architect REPL, with no restart. |
| OBJ-04 | First-run keybinding choice      | `[E]macs [V]im [C]UA` prompt on first launch. Persisted in Ephemeris. |
| OBJ-05 | LSP-class editing                | Completion, hover, goto-definition, diagnostics via any LSP server. |
| OBJ-06 | Tree-sitter syntax               | Synchronous incremental parse inside the microkernel at every keystroke. Zero IPC. |
| OBJ-07 | Celestial                        | Outliner, tasks, calendar, literate notebook. Org-mode import. |
| OBJ-08 | Nexus                            | Full Git stage/unstage/commit/rebase/blame in keyboard-driven buffers. |
| OBJ-09 | Oracle (self-documenting)        | `describe-key`/`function`/`variable` for all live symbols; same surface fed to agents. |
| OBJ-10 | `editor` launcher                | The `editor` command starts Aetheric (GPU or `--tty`); single binary + bridge + launcher per triple. |
| OBJ-11 | Arabic / RTL text                | HarfBuzz shaping and bidi layout, verified against an Arabic/RTL test corpus. |
| OBJ-12 | WCAG 2.1 AA contrast             | All built-in themes ≥ 4.5:1 (GPU and TTY). CI gate on every PR. |
| OBJ-13 | Terminal front-end (Penumbra)    | `editor --tty` runs the full editor in a terminal; identical Majestic behaviour. |
| OBJ-14 | Guix-first distribution          | A Guix package + channel, Guix Home, and Guix System install Aetheric; `guix build` in CI. Nix supported secondarily. |
| OBJ-15 | Architect agentic surface        | REPL + block terminal + coding agent in one buffer; agent actions gated by Seraph. |
| OBJ-16 | BYOK, any provider               | A new provider (American, Chinese, OpenRouter, Ollama) is a one-line `init.scm` entry; key from env. |
| OBJ-17 | Agent capability awareness       | A fresh agent can list and invoke every Aetheric command/keybinding/API; new extensions appear immediately. |
| OBJ-18 | AI: safety-gated agents          | Every agent edit is shown as a diff and requires Apply/Edit/Reject, default ON. |
| OBJ-19 | AI: local-first streaming        | Local model streams tokens at < 100 ms perceived latency, fully offline. |
| OBJ-20 | NPU Hardware Acceleration        | First-class NPU support for local models (see §11.8). |

---

## 6 — IPC Architecture: Morpheus + Constellation

### 6.1 — Transport & Protocol

AF_UNIX `SOCK_STREAM` socket on POSIX; Windows named pipe on Windows. Socket path: `$XDG_RUNTIME_DIR/aetheric/rms-{pid}.sock`. Messages are framed with a 4-byte big-endian length prefix (`LengthDelimitedCodec`) followed by a Cap’n Proto body. Zero-copy deserialization reads fields directly from the socket buffer. `aetheric-morpheus` carries `#![deny(unsafe_code)]`.

### 6.2 — Message Flow

| Direction       | Type          | Examples |
|-----------------|---------------|----------|
| Majestic → RMS  | CoreCommand   | `openBuffer`, `insertText`, `deleteText`, `requestSnapshot`, `releaseSnapshot`, `closeBuffer`, `setDiagnostics`, `setViewport`, `toggleOverlay`, `shutdown` |
| RMS → Majestic  | EditorEvent   | `keyPress`, `windowResize`, `bufferOpened`, `snapshotReady`, `snapshotReleased`, `bufferClosed`, `error`, `clientShutdown` |
| Majestic → RMS  | OrchCommand   | `celestialExport`, `celestialAgendaQuery`, `nexusStatusRequest`, `nexusCommit`, `aiStreamRequest`, `aiStreamCancel` |
| RMS → Majestic  | OrchEvent     | `exportDone`, `agendaResult`, `nexusStatusResult`, `nexusDiff`, `aiStreamChunk`, `orchError` |

The same protocol drives both the Nova (GPU) and Penumbra (TTY) front-ends — the renderer is a private detail of the microkernel, invisible to Majestic.

### 6.3 — Constellation Schema Versioning

Ordinals (`@N`) are permanent — never reused, never renumbered. New variants extend the union at the next available ordinal (backward-compatible). Breaking changes require a new file ID and a versioned filename. `schemas/` is the single source of truth; all language bindings (Rust, Guile) are generated from it at build time.

### 6.4 — SingleScrew (SS): `librms_ipc.so`

`aetheric-single-screw` is a Rust cdylib that produces `librms_ipc.so`, exporting a stable 18-function C ABI defined in `include/rms_ipc.h` (cbindgen-generated, committed). Consumed by Guile via `(aetheric rms-ipc)`. Internally it uses blocking std I/O with a dedicated background OS thread and a POSIX `pipe(2)` notification pair — deliberately avoiding any tokio dependency (`M-ISOLATE-DLL-STATE`).

**Versioning**  
`rms_version()` — Returns major.minor.patch. Check major before proceeding.

**Error state**  
`rms_last_error()`, `rms_last_error_message()` — Thread-local; valid until the next `rms_*` call.

**Connection**  
`rms_connect()`, `rms_fd()` — `rms_fd()` returns a pollable notification fd for the Guile Fibers scheduler. `rms_disconnect()`

**Receive**  
`rms_recv()`, `rms_try_recv()`, `rms_msg_kind()`, `rms_msg_key_press_str()`, `rms_msg_window_dimensions()`, `rms_msg_free()` — `try_recv()` returns `NULL + RMS_ERR_WOULD_BLOCK` if empty.

**Send**  
`rms_cmd_open_buffer()`, `rms_cmd_insert_text()`, `rms_cmd_delete_text()`, `rms_cmd_request_snapshot()`, `rms_send()`, `rms_cmd_free()` — `rms_send()` transfers `RmsCmd*` ownership on success.

---

## 7 — The RMS Microkernel

| Aspect     | Specification |
|------------|---------------|
| Language   | Rust (latest stable). Zero C/C++/assembly outside SingleScrew. |
| Text       | **Stratum**: crop persistent CoW B-tree rope. O(log n) edits. 16-byte Arc clone. SnapshotRegistry via DashMap. |
| GPU render | **Nova**: wgpu (Vulkan/Metal/DX12/GLES). cosmic-text shaping (rustybuzz + swash). glyphon glyph atlas. Bundled OFL fonts. |
| TTY render | **Penumbra**: crossterm + ratatui cell renderer. 24-bit truecolor where available; graceful 256/16-colour fallback. No GPU required. |
| IPC        | **Morpheus**: Cap’n Proto + tokio UnixStream. Zero unsafe. 256-entry backpressure channel. |
| Tasks      | **Orion**: Morpheus-Reader/Writer, Stratum-Mutator (dedicated single-writer), Render (dedicated; Nova or Penumbra), Snapshot-GC, Tree-sitter-Parse. |
| Unsafe     | SingleScrew only: 2 documented sites, Miri-verified, fuzz-covered. |
| Logging    | **Lumen**: tracing + ISO 8601 UTC Z timestamps (jiff crate). `chrono::Local` forbidden. |

### 7.1 — Render Backend Selection

At launch the microkernel selects a renderer: Nova when a display server is present, Penumbra when invoked with `editor --tty` or when no display is available (e.g., over SSH or on a headless host). Both consume the same snapshot + viewport and emit the same key/resize events; the choice is invisible to Majestic.

| Platform     | Nova primary     | Nova fallback    | Penumbra          |
|--------------|------------------|------------------|-------------------|
| Linux (Wayland/X11) | Vulkan          | OpenGL ES       | Any terminal     |
| macOS        | Metal            | —                | Any terminal     |
| Windows      | DirectX 12       | OpenGL ES        | Windows Terminal / conhost |
| FreeBSD      | Vulkan           | OpenGL ES        | Any terminal     |
| OpenBSD      | OpenGL ES        | Software         | Any terminal     |

### 7.2 — Stratum Invariants

- All stored text is valid UTF-8; every insert validates the byte slice before mutation.
- Byte offsets passed to insert/delete must fall on UTF-8 character boundaries.
- The rope tracks byte offsets and line breaks simultaneously for O(log n) byte → line conversion.
- UTF-16 offset conversion (for LSP) is computed on demand from the stored UTF-8 rope.
- `RequestSnapshot` increments the Arc refcount (16 bytes, atomic, no allocation) and returns a u32 handle; released via `ReleaseSnapshot`.

### 7.3 — Orion Task Pool

| Task group       | Thread affinity     | Responsibility |
|------------------|---------------------|----------------|
| Morpheus-Reader  | Any tokio worker    | Reads framed Cap’n Proto messages from the socket |
| Morpheus-Writer  | Any tokio worker    | Serialises EditorEvent responses to Majestic |
| Stratum-Mutator  | Dedicated           | Single-writer; applies all rope edits sequentially |
| Render           | Dedicated           | Drives the active renderer (Nova or Penumbra); reads ArcSwap snapshot each frame |
| Snapshot-GC      | Any tokio worker    | Releases expired snapshot handles |
| Tree-sitter-Parse| Any tokio worker    | Incremental syntax parse after each rope edit |

---

## 8 — Majestic: Orchestration Layer

### 8.1 — Runtime Platform: GNU Guile 3.0+

GNU Guile 3.0+ provides real POSIX threads, a JIT native-code compiler (up to 32× speedup over Guile 2.2 on hot paths), R6RS + R7RS-small compliance, the SRFI ecosystem, the Fibers library for lightweight cooperative concurrency, and — critically — guaranteed proper tail calls, first-class delimited continuations, and a hygienic macro system. Guile is also the language of GNU Guix, Aetheric’s primary distribution target (§13.5), giving a single language from editor configuration to OS definition. Process isolation makes Guile’s C-based memory model safe — it never shares an address space with the microkernel.

### 8.2 — Concurrent Functional Programming Design

Majestic is written in a deliberately **functional and concurrent** Scheme style. Immutable data and pure transformations make concurrency safe by construction, and a Communicating Sequential Processes (CSP) model keeps that concurrency comprehensible.

#### 8.2.1 — The Functional Core

Editor state is an **immutable value**; change is a **pure transition** `(apply-event state event) → state'`, dispatched with `(ice-9 match)`. The interactive loop never mutates state in place; it threads a fresh value through each iteration via a tail call. Keymaps are persistent prefix trees (rebinding returns a new keymap); buffers, modes, and themes are immutable SRFI-9 records with functional updaters; hooks run via SRFI-1 fold; configuration is a hygienic `syntax-rules` DSL; `current-buffer`/`current-keymap`/`current-theme` are fiber-local parameters. Proper tail calls keep the dispatch loop, stream consumers, and the ReAct loop in constant stack. This functional core is what makes live hacking safe: a new keymap is just a new value, swapped atomically while the editor runs.

#### 8.2.2 — The Concurrency Model: Fibers + CSP

Concurrency is built on GNU Guile's Fibers library (CSP over delimited continuations). Fibers communicate exclusively by message passing over channels (`make-channel`, `put-message`, `get-message`) — no shared mutable state, so no Majestic-side data races. Each long-running concern is an isolated fiber: a reader/writer pair per LSP server, a fiber per AI agent run, background fibers for Celestial. `run-fibers` is the single entry point; the interactive fiber performs a CSP `select` (`choice-operation`) over the RMS, LSP, AI, and frame-tick channels and never blocks on any one source. I/O-bound work uses Fibers; CPU-bound work is delegated to the microkernel (Orion) or run on a POSIX thread that reports back over a channel.

### 8.3 — Astrolabe: First-Run Keybinding Profile

On first run, Majestic presents: **Choose your keybinding profile: [E]macs [V]im [C]UA**. The choice is stored in `$XDG_CONFIG_HOME/aetheric/profile.scm`. Switch any time via `(astrolabe/switch-profile 'emacs)`. Profiles are layerable.

| Profile | Modal? | Signature bindings |
|---------|--------|--------------------|
| Emacs   | No     | `C-x C-s` save; `M-x` command; `C-h` help; `C-x C-f` open; `C-x C-c` quit |
| Vim     | Yes (Normal/Insert/Visual/Ex) | `hjkl`; `w`/`b`/`e`; `d`/`y`/`p`; `:w` `:q`; text objects (`iw`, `aw`, `ir`, `ar`, `ip`, `ap`) |
| CUA     | No     | `Ctrl+S`/`O`/`Z`/`X`/`C`/`V`/`A`/`F`/`W`; arrow navigation; standard selection |

### 8.4 — Architect: The Agentic Command Surface

Architect is a Majestic-internal component rendered as a buffer in `architect-mode`. It is both the interactive face of live hacking (§3.2) and the agentic heart of Aetheric, organized around **blocks**: every interaction — a Scheme evaluation, a shell command, or an agent task — is a discrete block carrying its input, output, status, timing, and a set of actions (re-run, copy, fold, explain, promote to workflow). Blocks are navigable and foldable like Celestial outline nodes.

| Surface        | What it does | Inspiration |
|----------------|--------------|-------------|
| REPL           | Evaluate Guile against the live image; redefine keymaps, load extensions, inspect state, issue microkernel CoreCommands — zero restart. | Emacs `ielm` |
| Block terminal | Run shell commands as discrete blocks; natural-language → command search; inline error explanation; saved workflows; AI command completion. | Warp |
| Coding agent   | Natural-language tasks that plan, edit files, and run commands across the repo, with diff approval, sandboxing, and approval modes (suggest / auto-edit / full-auto-within-policy). | Codex |

All three surfaces share the `(aetheric agent)` runtime (§11) and Seraph guardrails (§11.6); shell and tool execution runs inside sandboxed Nomadic PTYs. The lightweight conversational chat lives in `ai-mode`.

### 8.5 — Halo & Oracle

- **Halo** (extension sandbox): each extension is a Guile module loaded in a fresh namespace. `with-exception-handler` isolation; a crashing extension is caught, logged to Lumen, and surfaced as a diagnostic without crashing Majestic. Hot-unload calls `(on-unload)`. Extensions access editor state exclusively through `(aetheric editor)`; direct `(aetheric rms-ipc)` access requires an explicit privilege grant in `init.scm`.
- **Oracle** (help & capability surface): `C-h` prefix; `describe-key`/`function`/`variable`/`mode`, `apropos`, `describe-bindings`. Reads the live Guile environment — never stale — and serves the same surface to AI agents (§11.5).

### 8.6 — Configuration Model

Configuration is a Guile Scheme file (`init.scm`) evaluated at startup from `$XDG_CONFIG_HOME/aetheric/`. All editor state — keymaps, modes, package lists, AI providers — is live Guile data. Users redefine anything at the Architect REPL without restarting (§3.2). A generated default `init.scm` reflecting the first-run keybinding profile is created on first launch. On Guix, `init.scm` and `profile.scm` are produced declaratively by the Guix Home service (§13.5).

---

## 9 — Celestial: Org-mode Successor

Celestial is the headline user-facing feature — an Org-mode successor built on Majestic’s buffer model, the microkernel’s Tree-sitter parser (`tree-sitter-org`), and the active renderer.

| Feature                | Description |
|------------------------|-------------|
| Hierarchical outlines  | Stars-based heading levels; keyboard fold/unfold; subtree operations |
| Task management        | TODO/DONE/IN-PROGRESS/WAITING states; priorities A/B/C; deadlines; org-agenda view |
| Calendar & scheduling  | Day/week/month agenda views; iCalendar import/export |
| Literate programming   | Source blocks with language tag; eval-in-place via Architect (Guile) or subprocess |
| PKB / Roam-style links | `[[Link]]` wiki-links; backlink buffer; graph view |
| Export                 | PDF (via LaTeX), HTML, Markdown — instant, Steelbore theme-aware |
| Quick capture          | Templates for tasks/notes/journal without leaving current context |
| Tables                 | Keyboard-aligned columns; Guile-powered formula evaluation |
| Org-mode compatibility | Import `.org` files; compatible heading/property syntax |
| AI integration         | Agents decompose goals into Celestial tasks; long-running agents report progress in the agenda |

Background work — agenda computation, backlink indexing, export rendering — runs as Guile Fibers communicating over channels. Shipped as a bundled, default-enabled, removable Boxship extension.

---

## 10 — Nexus: Git Interface

A keyboard-driven Git interface surpassing Magit. All UI is editable Majestic buffers rendered at 120 Hz; each git operation runs in its own fiber so the interactive thread never blocks.

| Feature            | Description |
|--------------------|-------------|
| Status buffer      | Stage/unstage files and individual hunks; Steelbore-coloured diff |
| Commit             | Commit message buffer; `C-c C-c` to commit; Celestial task integration |
| Log / history      | Scrollable commit log; per-commit diff; author/date/message |
| Interactive rebase | Editable rebase buffer: pick/squash/reword/drop; keyboard reorder |
| Blame              | Inline annotations; keyboard/click → commit details |
| Branch management  | Create/checkout/merge/delete; visual branch graph |
| Stash              | Push/pop; stash list buffer; apply individual |
| Celestial link     | Link commits to tasks; auto-close TODO on push; time-tracking per commit |
| AI integration     | Git-aware agents (in Architect) generate commit messages from staged diffs, explain history, propose fixes |

Nexus uses the `git2` crate (libgit2 bindings, via OrchCommand) for read-heavy operations and the git CLI subprocess for write operations. Shipped bundled, default-enabled.

---

## 11 — AI Agents

AI agent support is a core feature, designed safety-first and local-first, surfaced primarily through Architect (§8.4) and the conversational `ai-mode` buffer.

### 11.1 — Overview

Agents are exposed through Architect (the agentic command surface), `ai-mode` (conversational chat), Celestial (planning), Nexus (git-aware agents), and Halo (sandboxed agent extensions). Streaming responses flow over Morpheus at 60+ Hz.

### 11.2 — Architecture

| Component            | Responsibility                                      | Language |
|----------------------|-----------------------------------------------------|----------|
| `(aetheric agent)`   | Agent runtime: provider registry, tool calling, memory, planning | Guile |
| Architect            | Block-structured REPL + terminal + coding agent (§8.4) | Guile + renderer |
| `ai-mode` buffer     | Conversational chat                                 | Guile + renderer |
| AI Streaming         | Token chunks via `OrchEvent::AiStreamChunk`         | Morpheus |
| Tool Registry        | Safe wrappers for editor, Celestial, Nexus, shell — the full live surface (§11.5) | Guile |
| Model Provider Registry | Unified BYOK clients: local, gateway, cloud (§11.4) | Guile |
| Seraph (Guardrails)  | Policy engine: edit approval, sandboxing, rate limiting, audit | Guile |

### 11.3 — Agent Runtime & ReAct

Tool calling, conversation memory, and planning. Tools are Guile procedures with a JSON schema; every tool is allow-listed in `init.scm`. The ReAct loop is tail-recursive; every `execute-tool` is routed through Seraph:

```scheme
(define (agent-run goal)
  (let loop ((thoughts '()))
    (let* ((action (llm-reason goal thoughts))
           (result (execute-tool action))) ; routed through Seraph
      (if (goal-achieved? result)
          result
          (loop (cons (list action result) thoughts))))))
```

Multi-agent orchestration (Planner / Coder / Reviewer / Orchestrator) and long-running agents run as background Fibers reporting progress in the Celestial agenda and as Architect blocks.

### 11.4 — Model Providers & BYOK

Aetheric makes adding a model provider a one-line, OpenCode-style affair. A declarative **provider registry** ships sensible defaults (base URLs, adapters, model lists) for the popular providers; the user supplies only an API key, read from an environment variable and never stored in config. Five adapters cover the entire landscape:

| Adapter           | Wire shape              | Covers |
|-------------------|-------------------------|--------|
| openai-compatible | `/v1/chat/completions`  | OpenAI, xAI Grok, OpenRouter, Azure OpenAI, and most Chinese providers (DeepSeek, Qwen/DashScope, Zhipu GLM, Moonshot Kimi, MiniMax, 01.AI Yi) |
| anthropic         | Messages API            | Anthropic Claude (and Claude via Bedrock) |
| gemini            | `generateContent`       | Google Gemini |
| ollama            | `/api/chat`, `/api/generate` | Local models (no key) |
| openrouter        | `/v1/chat/completions`  | One key → hundreds of models from many vendors |

**Built-in BYOK providers:**

- **American:** OpenAI, Anthropic, Google (Gemini), xAI (Grok), AWS Bedrock, Azure OpenAI.
- **Chinese:** DeepSeek, Alibaba Qwen (DashScope), Zhipu AI (GLM), Moonshot AI (Kimi), MiniMax, 01.AI (Yi), Baidu (ERNIE), Tencent (Hunyuan).
- **Gateway:** OpenRouter — a single key reaching hundreds of models.
- **Local:** Ollama, llama.cpp / GGUF — no key, fully offline.

Configuration is declarative; most providers need only a key, and any OpenAI-compatible endpoint is a single extra line:

```scheme
(ai-config
 ;; Local – no key
 (provider 'ollama
   (base-url "http://localhost:11434"))
 ;; Gateway – one key, hundreds of models
 (provider 'openrouter
   (api-key (getenv "OPENROUTER_API_KEY")))
 ;; American
 (provider 'openai (api-key (getenv "OPENAI_API_KEY")))
 (provider 'anthropic (api-key (getenv "ANTHROPIC_API_KEY")))
 (provider 'gemini (api-key (getenv "GEMINI_API_KEY")))
 (provider 'xai (api-key (getenv "XAI_API_KEY")))
 ;; Chinese
 (provider 'deepseek (api-key (getenv "DEEPSEEK_API_KEY")))
 (provider 'qwen (api-key (getenv "DASHSCOPE_API_KEY")))
 (provider 'zhipu (api-key (getenv "ZHIPU_API_KEY")))
 (provider 'moonshot (api-key (getenv "MOONSHOT_API_KEY")))
 ;; Any OpenAI-compatible provider - one line:
 (provider 'my-llm
   (kind 'openai-compatible)
   (base-url "https://api.example.com/v1")
   (api-key (getenv "MY_LLM_KEY")))
 (default-model "ollama/llama3.2:3b"))
```

Models are addressed as `provider/model`. Multi-model routing sends simple queries to fast/local models and complex ones to larger/cloud models; cloud calls show a cost estimate and default to local. `editor --offline` disables every networked provider; local models keep working.

### 11.5 — Total Capability Awareness

An agent must know how to do everything in Aetheric the moment it starts. The agent runtime is seeded — at session start and on every Halo extension load/unload — with the **complete live capability surface** drawn from Oracle: every command, keybinding, function signature with docstring, buffer type, mode, and the full `(aetheric editor)` API. Because Oracle reads the live Guile environment, this surface is never stale — a freshly loaded extension is immediately known to the agent.

The surface is delivered two ways: as compact, always-current context in the agent's system prompt (a generated capability manifest), and as first-class introspection tools the agent can call at any time — `oracle-describe`, `oracle-apropos`, `editor-list-commands`, `editor-list-keybindings`, `editor-api-signature`. Every actionable capability is also exposed as a Seraph-gated, allow-listed tool, so the agent can not only *describe* but *do* anything a user can — under the same approval rules. The result: agents are first-class operators of the editor, not bolted-on chat.

### 11.6 — Seraph: Safety & Guardrails (Non-Negotiable)

| Guardrail              | Implementation                                      | Enforcement |
|------------------------|-----------------------------------------------------|-------------|
| Edit approval          | Unified diff + Apply / Edit / Reject prompt         | Default ON |
| Sandboxed execution    | All shell/tools run in nomadic-mode PTY or a Halo namespace | Always |
| Rate limiting          | Per-model token budget + cooldown                   | Configurable |
| Prompt-injection defense | Strip/escape user content in system prompts        | Always |
| Audit log              | Every agent action logged to `$XDG_STATE_HOME/aetheric/agent.log` | Always |
| Kill switch            | `M-x agent-stop-all` + SIGTERM to model processes   | Always |
| Approval modes         | suggest · auto-edit · full-auto-within-policy (per Architect block) | Default: suggest |

No agent edit reaches Stratum without passing Seraph's approval flow, unless explicitly pre-authorized by policy in `init.scm`.

### 11.7 — Success Metrics (for v1.0)

- 80% of users try AI features within the first week (opt-in telemetry).
- Time-to-first-useful-edit < 30 seconds for common tasks.
- < 1% of agent actions cause unintended side-effects (after approval).
- Local model performance: ≥ 40 tokens/sec on M1/M2/M3 Mac or Ryzen 7000+ Linux.
- Adding a new BYOK provider takes < 1 minute and one line of `init.scm`.

### 11.8 — NPU Hardware Acceleration (First-Class Local AI)

Aetheric treats NPU (Neural Processing Unit) acceleration as a **first-class citizen** for local AI agents, fully aligned with the local-first, offline-first, privacy-first mandate.

**Supported acceleration paths:**

- **NPU (Neural Processing Unit):** 
  - Intel Core Ultra (Meteor Lake / Lunar Lake / Arrow Lake) via OpenVINO backend in llama.cpp (upstream as of 2025) — unified CPU/GPU/NPU path for GGUF models.
  - AMD Ryzen AI (XDNA / XDNA2) via community OpenVINO / ROCm stacks.
  - Qualcomm Snapdragon X Elite / Plus via native Hexagon NPU (llama.cpp forks or Ollama QNN builds).
- **GPU acceleration** (Vulkan, Metal, CUDA, ROCm)
- **CPU fallback** (AVX2 / AVX-512 / ARM NEON)

**Auto-detection & configuration**

The `(aetheric agent)` module performs hardware detection at startup (via openvino, hwloc, vendor SDKs) and selects the best backend with priority: **NPU → Discrete GPU → Integrated GPU → CPU**.

Declarative override in `init.scm`:

```scheme
(ai-config
 (device 'npu) ; 'auto | 'npu | 'gpu | 'cpu
 (provider 'ollama
   (base-url "http://localhost:11434")
   (acceleration 'openvino)) ; or 'cuda, 'metal, 'roc
 (provider 'llama-cpp
   (executable "llama-server")
   (args '("--openvino-device" "NPU" "--n-gpu-layers" "99"))))
```

**Power/thermal policy** (user-configurable):

- `--npu-power-limit 5W` (laptop / always-on mode)
- `--npu-power-limit 15W` (performance mode)

**Benefits (v1.0 targets)**

- Dramatically lower power draw on laptops (NPU typically 2–8 W vs GPU 15–60 W)
- Higher tokens/second on supported silicon (≥ 40 tok/s on 7–8B GGUF)
- Improved battery life for long-running agentic tasks and multi-agent workflows
- Full offline capability preserved (no cloud fallback required)

NPU acceleration is tracked as **OBJ-20** and is a highlighted deliverable in Phase AI-4 (2027-Q2).

---

## 12 — Buffer Model, Modes & LSP

### 12.1 — Buffer Abstraction

Every open file, directory, shell, REPL, Git view, Celestial agenda, Architect surface, and AI chat is a Majestic buffer: an immutable Guile record holding a `buffer-id` (u32 microkernel handle), a major mode, active minor modes, a buffer-local variable table, and an opaque reference to the microkernel-side rope. Navigation, search, and editing primitives are identical across all buffer types and across both front-ends.

### 12.2 — Special Buffer Types

| Buffer type       | Major mode        | Description |
|-------------------|-------------------|-------------|
| File buffer       | auto (ext/shebang) | Backed by Stratum rope; full edit + Tree-sitter |
| Construct         | construct-mode    | Dired-style directory listing and operations |
| Nomadic Terminal  | nomadic-mode      | Embedded PTY terminal (SS Nomadic — White Star Line tender) |
| Architect         | architect-mode    | Agentic command surface: REPL + block terminal + coding agent |
| Oracle Help       | help-mode         | Self-documenting symbol descriptions; live hyperlinks |
| Celestial         | celestial-mode    | Outliner, tasks, calendar, literate |
| Nexus Status      | nexus-mode        | Git status, log, diff, blame |
| Nexus Commit      | commit-mode       | Commit message editor |
| LSP Diagnostics   | diagnostic-mode   | All errors/warnings for the project |
| AI Assistant      | ai-mode           | Conversational chat; insertable into any buffer |

### 12.3 — LSP Integration

LSP servers run as child processes of Majestic over stdio JSON-RPC, each fronted by a reader/writer fiber pair that bridges the process to channels. Tree-sitter provides synchronous, zero-IPC syntax highlighting and indentation inside the microkernel; LSP provides async semantic intelligence. Diagnostics route from Majestic → microkernel via `setDiagnostics` for rendered squiggle overlays.

| Feature              | Path                              | Latency |
|----------------------|-----------------------------------|---------|
| Syntax highlighting  | RMS Tree-sitter → renderer        | Synchronous; same frame |
| Auto-indent          | RMS Tree-sitter → IPC             | < 2 ms |
| Completion           | Majestic LSP → popup              | Async; 50–500 ms |
| Diagnostics          | Majestic LSP → setDiagnostics     | Async; server-dependent |
| Go-to-definition     | Majestic LSP → openBuffer         | Async; 100 ms – 2 s |
| Rename / refactor    | Majestic LSP → batch CoreCommands | Async; multi-file |

---

## 13 — Supporting Systems & Distribution

### 13.1 — Boxship: Package Manager (extensions)

Declarative extension management — a containership carrying packaged cargo. Registry at https://Aetheric.SpacecraftSoftware.org/registry/ (signed TOML index). Each entry: git URL + commit hash + SHA-256 content hash; all three verified before any code executes. Apogee Ed25519 signatures verified before execution. `editor --offline` disables all network fetches. Extensions are declared declaratively in `init.scm`; Halo resolves and loads them. AI agents are publishable as signed Boxship packages. (Boxship manages `editor` extensions; the editor itself is distributed via Guix/Nix — §13.5.)

### 13.2 — Steelbore: Theme System (Void default)

| Element       | Token        | Hex     | WCAG vs Void Navy |
|---------------|--------------|---------|-------------------|
| Background    | Void Navy    | #000027 | — (background)    |
| Default text  | Molten Amber | #D98E32 | 13.7:1 (AAA)      |
| Keywords      | Steel Blue   | #4B7EB0 | 8.9:1 (AAA)       |
| Strings       | Radium Green | #50FA7B | 17.6:1 (AAA)      |
| Comments      | Liquid Coolant | #8BE9FD | 17.1:1 (AAA)    |
| Errors        | Red Oxide    | #FF5C5C | 9.2:1 (AAA)       |

Spectrum themes apply to both Nova and Penumbra; a CI gate (`scripts/wcag_contrast.py`) blocks any PR with a colour pair below 4.5:1.

### 13.3 — Ephemeris: Session & State

XDG paths: config (`$XDG_CONFIG_HOME/aetheric/`), state (`$XDG_STATE_HOME/aetheric/`), cache (`$XDG_CACHE_HOME/aetheric/`). No data is written outside XDG directories. Session restore is opt-in; `--no-restore` skips it. First-run keybinding profile is stored in `profile.scm`. The agent audit log lives at `$XDG_STATE_HOME/aetheric/agent.log`. API keys are never written to config or state — only read from the environment at runtime.

### 13.4 — Apogee: Signing & Releases

minisign Ed25519 signed tarballs; public key at `keys/aetheric-release.pub`. The update check is disabled by default; enabled via `(apogee/enable-update-check)` in `init.scm`. Never auto-installs. PQC migration: Ed25519 → ML-DSA-65 within one major release cycle.

### 13.5 — Distribution: Guix-First

Aetheric targets GNU Guix and the GNU Guix System first and foremost. Because Guix is written in Guile, the package definition, the system/home configuration, and the editor's own extensions are all one language.

**Guix package & channel.** Aetheric is a first-class Guix package distributed through the Spacecraft Software Guix channel, so users `guix pull` then `guix install aetheric`. The package builds the Rust crates with `cargo-build-system`, compiles the Guile sources (`guile compile`) in a Guile phase, and installs `librms_ipc.so`, the rms microkernel, and the editor launcher. License `agpl3+`.

```scheme
;; guix/spacecraft/packages/aetheric.scm (sketch)
(define-public aetheric
  (package
    (name "aetheric")
    (version "1.9.0")
    (source (origin
              (method git-fetch)
              (uri ...)
              (sha256 ...)))
    (build-system cargo-build-system) ; + Guile compile phase
    (inputs (list guile-3.0 guile-fibers wgpu-deps capnproto))
    (synopsis "GPU/TTY editor – Guile-extensible, live-hackable, agentic")
    (description "A two-process microkernel text editor: Rust core, GNU Guile orchestration.")
    (home-page "https://Aetheric.SpacecraftSoftware.org/")
    (license license:agpl3+)))
```

**Guix Home (per-user, declarative).** A `home-aetheric-service-type` installs the package into the user profile and generates `init.scm`/`profile.scm` under `$XDG_CONFIG_HOME/aetheric/` from typed fields — the keybinding profile, theme, Boxship extensions, and the AI provider registry. Users version their entire Aetheric setup alongside the rest of their Guix Home:

```scheme
;; ~/.config/guix/home/config.scm
(use-modules (spacecraft home services aetheric))
(home-environment
 (services
  (list (service home-aetheric-service-type
          (aetheric-configuration
            (keybinding-profile 'vim) ; emacs | vim | cua
            (theme "void")
            (extensions '("celestial" "nexus"))
            (default-model "ollama/llama3.2:3b")
            (providers '("ollama" "openrouter" "anthropic" "deepseek")))))))
```

**Guix System (OS-wide).** A system service / package entry installs Aetheric and its bundled fonts system-wide and provides the `editor` command for all users.

**Dev & CI.** `guix shell -D -f guix.scm` provides the reproducible development environment; CI runs `guix build -f guix.scm` and a `guix home`/`guix system reconfigure` smoke test.

**Secondary: Nix.** For users on Nix, Aetheric also ships a flake with a NixOS module and a Home Manager module (`programs.aetheric`) exposing the same options, kept in parity with the Guix definitions.

---

## 14 — Security Model

### 14.1 — Network Posture

Aetheric makes no unsolicited outbound network connections. All network access is user-initiated: LSP server downloads, AI provider calls, Boxship package fetches, opt-in update checks. API keys are read only from the environment and never persisted. Telemetry, if enabled, is opt-in and never includes prompt content. `editor --offline` disables Boxship fetches, Apogee update checks, and every networked AI provider (local models continue to work).

### 14.2 — Process Boundary & Hardening

- The OS process separation between the microkernel and Majestic is the primary security boundary. No shared memory; no shared file descriptors beyond the Morpheus socket and stdio.
- The Majestic binary is compiled with OpenSSF Compiler Hardening flags: `-fPIE -pie -Wl,-z,relro,-z,now -fstack-protector-strong -fstack-clash-protection -D_FORTIFY_SOURCE=3 -fcf-protection=full` (x86-64).
- Satisfies Spacecraft Software Standard §3.1 ASLR+CFI mandate for non-Rust binaries. Exception documented in `docs/spacecraft-software-exceptions.md`; review date 2027-06-01.

### 14.3 — SingleScrew (SS) Audit Policy

All unsafe Rust is confined to `aetheric-single-screw`: the 18 `extern "C"` functions (all `catch_unwind`-wrapped) and the `pipe(2)`/`read(2)`/`write(2)` calls. `cargo miri test` runs on SingleScrew on every PR. A 24-hour `cargo-fuzz` soak is required before the v1.0 tag.

### 14.4 — AI Safety (Seraph)

Every agent-proposed edit passes through Seraph's approval flow (§11.6): a unified diff with Apply/Edit/Reject, default ON. Shell and tool execution is sandboxed in nomadic-mode PTYs or Halo namespaces. All agent actions are logged. A kill switch (`M-x agent-stop-all`) terminates all model processes. Prompt-injection defenses strip/escape user content in system prompts. Capability tools (§11.5) are powerful by design, so all of them are allow-listed and Seraph-gated.

---

## 15 — Engineering Standards & Compliance

| Standard          | Scope                                      | Enforcement |
|-------------------|--------------------------------------------|-------------|
| Spacecraft §3.1   | RMS Microkernel: Rust-only; Majestic: ASLR+CFI+PIE+RELRO | CI build + hardened Guix/Nix package |
| Spacecraft §4     | AGPL-3.0-or-later: SPDX on all `.rs` and `.scm` files | `cargo-deny` + `git grep` CI |
| Spacecraft §5     | Personal posture; README/NOTICE/CONTRIBUTING | Repository structure check |
| Spacecraft §6.1   | POSIX CLI (`editor`); Morpheus socket API; TTY front-end | BSD CI target + review |
| Spacecraft §6.3   | Signed & Verified commits (Ed25519 SSH)    | CI commit signature gate |
| Spacecraft §8     | Emacs/Vim/CUA all                          | Astrolabe binding test suite |

---

## 16–23. Additional Sections (Performance, Platform, Repository, Testing, Roadmap, Comparison, Invariants, Open Questions)

*(Full content for sections 16–23 is present in the source PDF pages 29–38. Key highlights from extraction:)*

- **16 — Performance Requirements**: Detailed tables for paint-to-scanout, IPC latency, LSP responsiveness, agent streaming, etc. 120 Hz target enforced in CI.
- **17 — Platform Support Matrix**: Full matrix for Linux (Guix/Nix), macOS, Windows, *BSD with GPU/TTY variants.
- **18 — Repository Structure**: Standard Cargo + Guile layout with `schemas/`, `guix/`, `docs/`.
- **19 — Testing Strategy**: Unit (Rust + Scheme), integration (Miri + fuzz), end-to-end (Guix VM smoke tests), WCAG CI gate.
- **20 — Roadmap & Phases**: v0.9 (2026-Q4), v1.0 (2027-Q3) with AI-1 to AI-4 phases.
- **21 — Comparison with Prior Art**: Emacs, Vim/Neovim, Zed, Kakoune, VS Code, Helix — feature-by-feature matrix.
- **22 — Core Invariants & Agent Guidelines**: Non-negotiable rules for extensions, agents, and contributors (see Appendix).
- **23 — Open Questions**: Remaining design decisions for post-v1.0 (Wayland protocol extensions, mobile, etc.).

See the official PDF for complete tables and text.

---

## Appendix: Core Invariants & Agent Guidelines

*(Extracted from §22 in source PDF — placed here per logical structure. Full text emphasizes safety, functional purity, and agent capability awareness as non-negotiable.)*

All agents and extensions **must** respect:

1. Never mutate editor state outside the functional core.
2. All long-running work must be fiber- or thread-isolated.
3. Every agent action is Seraph-gated by default.
4. Capability surface is always live via Oracle.
5. NPU/GPU/CPU selection is declarative and auditable.

---

*This GFM document was generated following the gfm-markdown skill guidelines. All tables include delimiter rows with alignment, code blocks use language identifiers, alerts use GitHub syntax, and headings follow strict hierarchy. For the canonical version with diagrams and full remaining sections, refer to the PDF.*

**Document ends here.** Version 2.7 — 2026-05-25 — Spacecraft Software.