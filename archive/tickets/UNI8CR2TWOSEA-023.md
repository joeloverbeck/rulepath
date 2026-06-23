# UNI8CR2TWOSEA-023: Masked Claims — game-test-support dev-only dependency

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/masked_claims/Cargo.toml` `[dev-dependencies]`
**Deps**: 016

## Problem

Spec §3.10 / task `8C-R2-503`: Masked Claims must list `game-test-support` as a **dev-only** dependency to enable the C-07 no-leak harness and C-08 profile drivers. R2 adds it under `[dev-dependencies]` only; no normal/build/WASM/tool edge may be created. Shares `Cargo.toml` with `-016` (hence `Deps: 016`).

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/Cargo.toml` does not currently depend on `game-test-support` (confirmed in the reassess pass); HCD already has the dev-dependency.
2. Spec §3.10/§3.12/§9: dev-dependency only; a normal/build/WASM/browser/tool edge on `game-test-support` is forbidden.
3. Cross-crate boundary under audit: the `game-test-support` dev-only edge — supplies `no_leak`/`profiles` harnesses to tests only (`crates/game-test-support/src/{no_leak,profiles}.rs`), never production code.
4. §4 / §11 mechanical-scaffolding discipline: `game-test-support` is behavior-free dev infrastructure; the inverse normal-edge tree must show zero normal edges after this change (`cargo tree --workspace -e normal --invert game-test-support`).

## Architecture Check

1. A dev-only edge keeps the test harness out of the shipped game crate — exactly the §4 mechanical-scaffolding boundary, cleaner than a normal dependency.
2. No backwards-compat alias; a single `[dev-dependencies]` line is added.
3. `engine-core` is untouched; `game-test-support` is dev-only infrastructure, not a `game-stdlib` promotion.

## Verification Layers

1. Dev-only edge, no normal edge -> dependency check (`cargo tree --workspace -e normal --invert game-test-support` shows no `masked_claims` edge) + `bash scripts/boundary-check.sh`.
2. Tests can resolve the harness -> `cargo test -p masked_claims`.
3. Manifest placement under `[dev-dependencies]` -> codebase grep-proof in `Cargo.toml`.

## What to Change

### 1. Add the dev-only dependency

Add `game-test-support` under `[dev-dependencies]` in `games/masked_claims/Cargo.toml`.

## Files to Touch

- `games/masked_claims/Cargo.toml` (modify; serialized after `-016`)
- `Cargo.lock` (modify; records the dev-dependency edge)

## Out of Scope

- Any test that *uses* the harness (C-07 `-027`, C-08 profile drivers `-031`/`-035`/`-039`).
- Any normal/build/WASM/tool dependency edge.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green (harness resolves at dev time).
2. `cargo tree --workspace -e normal --invert game-test-support` shows no normal `masked_claims` edge; `bash scripts/boundary-check.sh` passes.

### Invariants

1. `game-test-support` appears only under `[dev-dependencies]`.
2. No production/WASM/tool target gains a `game-test-support` edge.

## Test Plan

### New/Modified Tests

1. `None — dependency-manifest change; the inverse normal-edge tree and boundary script are the regression guard.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo test -p masked_claims`

## Outcome

Completed on 2026-06-23.

Added `game-test-support` under `games/masked_claims/Cargo.toml`
`[dev-dependencies]` only, with the corresponding `Cargo.lock` package edge.
No normal/build/WASM/tool dependency edge was added, and no harness-using tests
were added in this ticket.

Verification passed:

1. `cargo tree --workspace -e normal --invert game-test-support` showed only
   the `game-test-support` package itself, with no `masked_claims` normal edge.
2. `bash scripts/boundary-check.sh`
3. `cargo test -p masked_claims`
