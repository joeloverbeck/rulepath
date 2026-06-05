# GAT0REPSKE-003: `tools/*`, `benches/`, and `games/` placeholders

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds seven `tools/*` placeholder crates, a `benches/` placeholder, and an empty `games/` placeholder; registers the new crate members in the workspace.
**Deps**: GAT0REPSKE-001

## Problem

The architecture names seven tool crates plus `benches/` and `games/` trees. Standing them up as compiling no-op placeholders reserves the structure and proves the workspace builds with every member present, before Gate 1+ fills them with real functionality. `games/` stays empty — the first game crate arrives in Gate 1.

## Assumption Reassessment (2026-06-05)

1. `tools/`, `benches/`, and `games/` are absent (greenfield, verified `test -e` 2026-06-05); the root `Cargo.toml` is created by GAT0REPSKE-001 (declared `Deps`). The seven tool names are taken verbatim from `docs/ARCHITECTURE.md` §1: `simulate`, `replay-check`, `trace-viewer`, `rule-coverage`, `bench-report`, `seed-reducer`, `fixture-check`.
2. Spec §2/§3 WB3 + `docs/ARCHITECTURE.md` §1 (target tree) and §3 (the `tools/*` ownership row: simulation, replay checking, trace inspection, rule coverage, benchmark reports, seed reduction, fixture validation — never game behavior).
3. Cross-artifact boundary under audit: the root workspace `Cargo.toml` members list created by GAT0REPSKE-001. This ticket extends it additively with the tool/bench members; it must not edit or remove the four crate members from 001.

## Architecture Check

1. Compiling no-op placeholders (a `--help`/version path) prove workspace wiring without speculative tool logic — cleaner than scaffolding real tool behavior that Gate 2 will define.
2. No backwards-compatibility shims.
3. `tools/*` own no game/rule behavior (`docs/ARCHITECTURE.md` §3); `engine-core` is untouched; `games/` stays empty (first game is Gate 1) with no game noun.

## Verification Layers

1. Seven tool crates compile and run a no-op `--help`/version -> simulation/CLI run (`cargo run -p <tool> -- --help`).
2. Workspace membership additive over 001 + builds -> schema/serialization validation (`cargo build` over the workspace with the new members).
3. `games/` is an empty placeholder with no game noun -> codebase grep-proof / manual review.

## What to Change

### 1. Seven `tools/*` placeholder bin crates

`tools/{simulate,replay-check,trace-viewer,rule-coverage,bench-report,seed-reducer,fixture-check}` — each a bin crate printing a version / no-op help and exiting cleanly.

### 2. `benches/` placeholder

A `benches/` placeholder (a crate or a directory placeholder per implementation choice).

### 3. `games/` empty placeholder

`games/` with a `.gitkeep` or `README.md` placeholder only — no crate, no game noun.

### 4. Workspace manifest

Add the tool (and any bench) members to the root `Cargo.toml` created by GAT0REPSKE-001.

## Files to Touch

- `tools/simulate/Cargo.toml` (new) + `tools/simulate/src/main.rs` (new)
- `tools/replay-check/Cargo.toml` (new) + `tools/replay-check/src/main.rs` (new)
- `tools/trace-viewer/Cargo.toml` (new) + `tools/trace-viewer/src/main.rs` (new)
- `tools/rule-coverage/Cargo.toml` (new) + `tools/rule-coverage/src/main.rs` (new)
- `tools/bench-report/Cargo.toml` (new) + `tools/bench-report/src/main.rs` (new)
- `tools/seed-reducer/Cargo.toml` (new) + `tools/seed-reducer/src/main.rs` (new)
- `tools/fixture-check/Cargo.toml` (new) + `tools/fixture-check/src/main.rs` (new)
- `benches/README.md` (new) (or a placeholder bench crate)
- `games/.gitkeep` (new) (or `games/README.md`)
- `Cargo.toml` (modify) — created by GAT0REPSKE-001

## Out of Scope

- Real tool functionality — trace serialization, replay checker, stable hashes, benchmark harness, fixture validation (Gate 2).
- Any game crate in `games/` (Gate 1).
- Private licensed or private-monster-game names anywhere (spec §8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build` over the workspace (including all tool/bench members) succeeds.
2. Each of the seven tools runs `--help`/version as a no-op without error.
3. `games/` contains only a placeholder (no crate, no game noun).

### Invariants

1. `tools/*` contain no game or rule behavior (ARCHITECTURE.md §3).
2. The workspace members list is additive over GAT0REPSKE-001 — the four original crates are unchanged.

## Test Plan

### New/Modified Tests

1. Per-tool smoke (`cargo run -p <tool> -- --help` exits 0) — proves each placeholder runs.

### Commands

1. `for t in simulate replay-check trace-viewer rule-coverage bench-report seed-reducer fixture-check; do cargo run -p $t -- --help || exit 1; done`
2. `cargo build` — full workspace build with new members.
3. `ls games/` — narrow check that `games/` holds only a placeholder (no crate).

## Outcome

Completed: 2026-06-05

What changed:
- Added seven placeholder binary crates under `tools/`: `simulate`, `replay-check`, `trace-viewer`, `rule-coverage`, `bench-report`, `seed-reducer`, and `fixture-check`.
- Registered the seven tool crates as workspace members without removing the four crate members from GAT0REPSKE-001.
- Added `benches/README.md` and an empty `games/.gitkeep` placeholder.

Deviations from original plan:
- None.

Verification results:
- `cargo fmt --all` passed.
- `cargo build` passed over the workspace with all tool members.
- `cargo test` passed over the workspace.
- `cargo run -p <tool> -- --help` passed for all seven tools and printed placeholder help/version output.
- `find games -maxdepth 2 -type f -print | sort` returned only `games/.gitkeep`.
