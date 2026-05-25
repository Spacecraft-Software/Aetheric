# Contributing to Aetheric

<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Copyright (c) 2026 Mohamed Hammad & Spacecraft Software -->

Thank you for your interest in Aetheric. This project follows the
[Spacecraft Software Standard](https://Construct.SpacecraftSoftware.org/).

## Before You Start

1. Read `AGENTS.md` — it contains the coding conventions and invariants for this
   repository.
2. Read the relevant sections of `Aetheric-Master-PRD-v2.7.md` and
   `IMPL-Aetheric-v2.7.md` for architectural context.
3. All commits must be signed (Ed25519 SSH).

## Development Workflow

1. **Plan first** for changes > ~50 lines or architectural changes.
2. Small, verifiable increments.
3. No `dbg!`, `println!`, commented code, or TODOs without an issue link.
4. Update tests + docs + schemas together.
5. Run the full gate before pushing:
   ```sh
   cargo clippy -- -D warnings
   cargo fmt -- --check
   cargo test
   guile -s tests/run-tests.scm
   ```

## Code of Conduct

Be respectful, constructive, and patient. We are building a tool that should
last decades — correctness and clarity matter more than speed.

## Contact

Mohamed Hammad <Mohamed.Hammad@SpacecraftSoftware.org>
