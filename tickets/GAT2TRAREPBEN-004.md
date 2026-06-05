# GAT2TRAREPBEN-004: Implement `tools/replay-check`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replaces the `tools/replay-check` no-op placeholder with a real replay checker that calls `games/race_to_n` `replay_support` and parses Trace Schema v1 JSON.
**Deps**: GAT2TRAREPBEN-002, GAT2TRAREPBEN-003

## Problem

Replay checking currently lives only inside `race_to_n` in-crate tests, and
`tools/replay-check` is a 16-line no-op. Gate 2 requires a real CLI that replays
golden traces through Rust behavior and fails loudly on any hash/diagnostic/outcome
drift, so CI can gate on it rather than only on in-crate replay tests (spec §D3).

## Assumption Reassessment (2026-06-05)

1. `tools/replay-check/src/main.rs` is a 16-line `run_placeholder` no-op; the crate
   is registered in the workspace `Cargo.toml` `members` list. `replay_support`
   (GAT2TRAREPBEN-002) provides the canonical replay evaluation; the migrated Trace
   Schema v1 JSON traces (GAT2TRAREPBEN-003) are the inputs.
2. Spec §D3 fixes the CLI (`--game race_to_n --trace <path>`, `--all`,
   `--directory <dir>`), the required comparisons (state/effect/action-tree/
   public-view/private-view-where-applicable/diagnostic/terminal/outcome), and the
   required failure output fields. Canonical schema = `docs/TRACE-SCHEMA-v1.md`
   (GAT2TRAREPBEN-001).
3. Cross-crate boundary under audit: `replay-check` depends on the `replay_support`
   public API and the Trace Schema v1 contract. It must replay *through* Rust
   behavior, never interpret trace data as rules.
4. FOUNDATIONS §2 (Rust owns replay): restate that the tool is a presentation of
   Rust replay behavior; all legality/transition/hashing stays in `games/race_to_n`.
5. §11 fail-closed determinism enforcement surface: `replay-check` IS a fail-closed
   gate. Confirm it exits non-zero on any drift, malformed trace, unsupported game,
   unsupported schema version, duplicate trace ID, missing expected surface, or
   invalid migration note — and that its failure output is viewer-safe (perfect-info
   game; no hidden information to leak).
6. Schema consumer: `replay-check` is a new consumer of Trace Schema v1. It must
   reject unknown `schema_version` values and unknown fields rather than silently
   ignoring them (§11 unknown-field rejection).

## Architecture Check

1. Reusing `replay_support` (rather than re-deriving hashes in the tool) keeps a
   single replay authority — a re-implementation could drift from the tests and
   make drift detection unsound.
2. No backwards-compatibility shims; legacy `.trace` import is gated behind an
   explicit migration mode only.
3. `engine-core` untouched; `replay-check` reaches game behavior through the
   `games/race_to_n` public API, introducing no kernel noun.

## Verification Layers

1. Drift detection → deterministic replay-hash check: an intentionally corrupted
   expected hash fails with non-zero exit and the required failure fields.
2. Malformed/unsupported input handling → schema validation: a malformed trace and
   an unknown `schema_version` each exit non-zero.
3. End-to-end CLI behavior → simulation/CLI run: `--all` passes on valid traces.
4. Kernel boundary → grep-proof: no `race_to_n` noun added to `engine-core`.

## What to Change

### 1. CLI parsing (`tools/replay-check/src/main.rs`)

Implement `--game race_to_n` (only), `--trace <path>`, `--directory <dir>`, and
`--all`, plus an explicit legacy-migration import mode.

### 2. Replay + comparison

Parse Trace Schema v1 JSON, replay setup/options/commands through `replay_support`,
and compare every expected surface. Emit the spec §D3 failure output (trace path,
trace ID, game ID, versions, command index/checkpoint, expected vs actual,
diagnostic mismatch, reproducing replay command, migration-note reminder). Exit
non-zero on any drift or malformed input.

### 3. `tools/replay-check/Cargo.toml`

Add the `games/race_to_n` (and `engine-core` as needed) dependencies.

## Files to Touch

- `tools/replay-check/src/main.rs` (modify) — replace no-op with real checker
- `tools/replay-check/Cargo.toml` (modify) — add game/engine deps

## Out of Scope

- Authoring/migrating the traces themselves (GAT2TRAREPBEN-003).
- `fixture-check` schema-strictness checks (GAT2TRAREPBEN-005).
- CI wiring (GAT2TRAREPBEN-013).
- Multi-game registry abstraction beyond what the CLI needs.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game race_to_n --all` — passes on the valid migrated traces.
2. A corrupted expected hash → `cargo run -p replay-check -- --game race_to_n --trace <path>` exits non-zero with the required failure fields.
3. A malformed trace and an unknown `schema_version` each exit non-zero.

### Invariants

1. Replay executes through Rust `replay_support`, never by interpreting trace data as rules (§2).
2. Any drift/malformed/unsupported input is fail-closed (non-zero exit), never a silent pass (§11).

## Test Plan

### New/Modified Tests

1. `tools/replay-check/src/main.rs` — in-crate tests (or `tools/replay-check/tests/`) for pass, corrupted-hash-fail, malformed-fail, unknown-version-fail.
2. Reuse the migrated golden traces (GAT2TRAREPBEN-003) as fixtures.

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all`
2. `cargo test -p replay-check`
3. `cargo run -p replay-check -- --game race_to_n --directory games/race_to_n/tests/golden_traces` — directory-mode smoke.
