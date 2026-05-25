# Aetheric

<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Copyright (c) 2026 Mohamed Hammad & Spacecraft Software -->

**Aetheric** is a next-generation, GPU-accelerated, GNU Guile Scheme-extensible text
designed to be the definitive long-term editing environment for power users.

## Two-Process Architecture

- **RMS Microkernel** (Rust, headless) — owns text storage (Stratum), rendering
  (Nova GPU / Penumbra TTY), and binary IPC (Morpheus). Never evaluates scripts.
- **Majestic** (GNU Guile 3.0+) — the executive brain: keymaps, modes, hooks,
  extensions, LSP clients, packages, Celestial outliner, Nexus Git interface,
  Architect agentic surface, and the AI agent runtime.

The two processes communicate over **Morpheus**, a zero-copy Cap'n Proto IPC
broker. Majestic loads the C-ABI bridge `librms_ipc.so` via `(aetheric rms-ipc)`.

## Launch

```sh
editor .           # open current directory
editor file.rs     # open a file
editor --tty       # terminal front-end (Penumbra)
editor --offline   # no network fetches
```

## Building

```sh
cargo build --release
guile -s tests/run-tests.scm
```

## License

AGPL-3.0-or-later. See `LICENSE`.

---
*Copyright (c) 2026 Mohamed Hammad & Spacecraft Software*
