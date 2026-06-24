# 8CR4NSEAPRITRI-020: River Ledger C-07 runout/multipot export no-leak matrix

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/river_ledger/tests/`; reveal/allocation policy unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

River's automatic all-in runout and multipot showdown export surfaces are not yet covered by hidden-future-card and folded/non-winning-hole-card absence checks in the shared pairwise geometry (MSC-8C, C-07 residual). Add those absence checks over automatic runout and multipot public/seat-private exports, leaving River's reveal/allocation policy unchanged and not treating public pot accounting as secret (spec §3.8 River residual, §5.8).

## Assumption Reassessment (2026-06-24)

1. `games/river_ledger/src/replay_support.rs::{export_public_replay, import_public_export}` exist; the multipot traces `three-way-main-two-side-pots`, `all-all-in-runout`, and `uncalled-return` exist under `tests/golden_traces/`; the shared harness `assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 classifies the runout/multipot export absence checks as `migrate`; River's showdown reveal policy authorizes any reveal and is not changed here. The pilot base matrix baseline comes from `-001`.
3. Cross-artifact: the export surfaces (`export_public_replay`, the seat-private export path) are game-owned; the shared harness owns enumeration/reporting only. This ticket and `-019` are mutually independent appends to River test files.
4. §11 no-leak firewall motivates this ticket: hidden future board/deck tail during automatic runout must be absent for all viewers; non-winning/folded hole cards in multipot exports must be absent unless River's existing showdown policy authorizes reveal.
5. Enforcement surface = pairwise source-seat × viewer products over runout/multipot public + seat-private exports; canaries are in-memory only, never committed.

## Architecture Check

1. Adding the runout/multipot absence checks completes River's residual no-leak coverage without re-deriving any base row — coverage lives in tests, reveal/allocation stays in River.
2. No backwards-compatibility shim is introduced; existing focused tests remain. No game-specific assertion is deleted.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and decides no secrecy (§4 mechanical-scaffolding lane).

## Verification Layers

1. Hidden future board/deck tail absent for all viewers during runout -> no-leak visibility test via `assert_pairwise_no_leak`.
2. Folded/non-winning hole cards absent in multipot public + seat-private exports unless policy authorizes reveal -> no-leak export test over `export_public_replay`/seat-private export.
3. Public pot accounting treated as public, not secret -> no-leak test asserting absence-of-card, with allocation explanation present as public accounting.

## What to Change

### 1. Add runout/multipot export absence matrix

In `games/river_ledger/tests/visibility.rs` (and `tests/replay.rs` or a narrowly added module), add hidden-future-card and folded/non-winning-hole-card absence checks over automatic runout and multipot public/seat-private exports for counts 3–6, using the named multipot traces as inputs.

## Files to Touch

- `games/river_ledger/tests/visibility.rs` (modify)
- `games/river_ledger/tests/replay.rs` (modify; or a narrowly added no-leak module)

## Out of Scope

- Rebuilding any ticket-021 base no-leak row (pilot credit).
- The stack-lifecycle matrix (`-019`).
- Any reveal, allocation, or accounting policy change; rewriting any export byte.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the runout/multipot export absence matrix.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with export bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. No hidden future card or folded/non-winning hole card reaches an unauthorized viewer; public accounting stays public.
2. No canary token appears in any committed trace, fixture, export, snapshot, log, or test identifier.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` / `tests/replay.rs` — runout + multipot export no-leak absence matrix for counts 3–6.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game visibility/replay test is the correct boundary: export no-leak is a game-local projection property.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a River replay-export no-leak matrix using `assert_pairwise_no_leak` over observer plus every seat viewer.
2. Pinned the named runout/multipot fixture files and exercised the current `export_public_replay` authority for all-all-in runout, three-way side-pot, uncalled-return, public-observer multipot, and seat-private multipot surfaces.
3. Asserted future cards are absent for every viewer, private cards are present only for the owning seat viewer, and public accounting/export steps remain visible.

Deviations:

1. `export_public_replay` currently accepts replay seed/count/commands, not the placeholder fixture `setup_options`; the named fixture files are pinned as reviewed inputs, while the pairwise matrix uses the same seed/count/command surfaces supported by the current exporter.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p river_ledger` — passed.
3. `cargo run -p replay-check -- --game river_ledger --all` — passed.
4. `bash scripts/boundary-check.sh` — passed.
