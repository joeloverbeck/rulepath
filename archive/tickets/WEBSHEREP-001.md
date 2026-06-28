# WEBSHEREP-001: Raise authoritative Rust replay import-size bound

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`src/constants.rs` bound value, `src/tests.rs` round-trip test); no schema/serialization/hash change
**Deps**: None

## Problem

The web shell exports a valid full-length replay (Starbridge 6-seat, 2000 plies, ≈549 KiB) that it then refuses to import. The authoritative size guard lives in Rust: `import_replay` rejects any document larger than `MAX_REPLAY_IMPORT_BYTES = 128 * 1024` with the `replay_too_large` diagnostic (`crates/wasm-api/src/lib.rs:3477`, `crates/wasm-api/src/constants.rs:116`). Because the legitimate export exceeds 128 KiB, the documented replay round-trip (`SC-REPLAY-001`, FOUNDATIONS §9/§11 local replay import/export) is broken for every full-length game. Raising this Rust bound is the necessary and sufficient fix for the importer authority; the TypeScript shadow is handled separately (WEBSHEREP-002).

## Assumption Reassessment (2026-06-28)

1. `MAX_REPLAY_IMPORT_BYTES` is defined once at `crates/wasm-api/src/constants.rs:116` (`pub(crate) const … = 128 * 1024;`) and consumed once at `crates/wasm-api/src/lib.rs:3477` inside `import_replay`, which emits `diagnostic_string("replay_too_large", …)`. The existing oversize-rejection test at `crates/wasm-api/src/tests.rs:2060` is keyed to `MAX_REPLAY_IMPORT_BYTES + 1`, so it stays valid under any bound increase (it asserts the bound is enforced, not a literal byte count).
2. Spec `specs/web-shell-replay-import-size-roundtrip.md` §2.2/§4 (Deliverable 1) and §5 (selected resolution, option 1+3 hybrid) require deriving the bound from a stated rule, not a bare magic number. The 561,667-char defect evidence (§2.1) and ≈549 KiB largest-export figure (§12 A2) are the sizing inputs.
3. Shared boundary under audit: the WASM importer contract in `docs/WASM-CLIENT-BOUNDARY.md` §Replay Safety. `import_replay` (`lib.rs:3476`) is the single authoritative parser/validator; the bound is a pre-parse validation threshold, not part of the replay/trace schema (governed by ADR 0009, not engaged).
4. FOUNDATIONS §2/§11 place validation authority in Rust. This ticket keeps the size decision fail-closed and blocking in Rust (warnings vs blockers undisturbed); it does not move any acceptance decision into TypeScript.
5. Enforcement surface: the `replay_too_large` fail-closed guard and the deterministic replay/hash surface. Raising the threshold changes no serialization order, no canonical bytes, and adds no export path, so it introduces no hidden-information leak (§11 no-leak firewall) and no replay/hash nondeterminism (§11/§13); ADR 0009 replay/trace-schema semantics stay untouched.

## Architecture Check

1. Raising one authoritative constant (with a derivation comment) is cleaner than per-game special-casing or structural command-count checks: the guard stays a single fail-closed pre-parse threshold, and `import_replay` remains the sole authority. Validating document shape would duplicate Rust's own parser and edge toward a second validation site.
2. No backwards-compatibility shim or alias: the constant's value changes in place; no parallel bound is introduced.
3. No `engine-core` mechanic noun involved; no `game-stdlib` promotion. Change is confined to `wasm-api`.

## Verification Layers

1. Authoritative bound admits the largest legitimate export -> Rust round-trip `cargo test` in `wasm-api`: build a full-length 6-seat Starbridge export (> the old 128 KiB bound) and assert `import_replay` round-trips it.
2. Oversize input still rejected fail-closed -> existing `tests.rs` oversize test (`MAX_REPLAY_IMPORT_BYTES + 1` → `replay_too_large`) passes against the raised bound.
3. No replay/hash/serialization drift -> `cargo run -p replay-check -- --game starbridge_crossing --all` unchanged; golden traces unchanged (ADR 0009 not engaged).

## What to Change

### 1. Derive and raise `MAX_REPLAY_IMPORT_BYTES`

In `crates/wasm-api/src/constants.rs`, raise the bound to a principled ceiling at least an order of magnitude above the largest legitimate catalog self-export (≈549 KiB today) with headroom for future games and format growth, while still bounding a pathological multi-hundred-MB paste — e.g. `8 * 1024 * 1024` (8 MiB, ≈15× the current max). Record the derivation rule as a code comment (largest legitimate export + headroom; not a bare constant). The `import_replay` guard at `lib.rs:3477` is unchanged in logic — it continues to fail-closed above the new value.

### 2. Add a full-length round-trip test

In `crates/wasm-api/src/tests.rs`, add a test that deterministically produces a full-length 6-seat Starbridge export exceeding the old 128 KiB bound (via the game's seeded export path; an equivalent comparable-size fixture is acceptable per spec §4), asserts `import_replay` returns Ok, and asserts the document length exceeds `128 * 1024` (so the test would have failed before this bump). Leave the existing oversize-rejection test in place.

## Files to Touch

- `crates/wasm-api/src/constants.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)

## Out of Scope

- The TypeScript pre-check in `ReplayImportExport.tsx` (WEBSHEREP-002).
- The UI e2e round-trip smoke (WEBSHEREP-003) and docs (WEBSHEREP-004).
- Any change to `import_replay` parsing/dispatch/validation semantics beyond the size-bound constant.
- Replay/trace schema, serialization order, or hash changes (ADR 0009 not engaged).

## Acceptance Criteria

### Tests That Must Pass

1. New `wasm-api` test: a full-length 6-seat Starbridge export (> 128 KiB) round-trips through `import_replay` (returns Ok; viewer-importable summary).
2. Existing oversize test (`MAX_REPLAY_IMPORT_BYTES + 1` → `replay_too_large`) still passes.
3. `cargo test -p wasm-api` and `cargo test --workspace` green; `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`.

### Invariants

1. `import_replay` remains the single authoritative size guard; the bound is fail-closed and blocking.
2. No replay/trace schema, serialization order, or hash change (`replay-check --game starbridge_crossing --all` unchanged).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/tests.rs` — new full-length Starbridge round-trip test asserting `import_replay` accepts a > 128 KiB legitimate export; existing oversize test retained.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game starbridge_crossing --all`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-28

Raised the authoritative Rust replay import bound from 128 KiB to 8 MiB in
`MAX_REPLAY_IMPORT_BYTES`, with a derivation comment tied to Starbridge
Crossing's 6-seat 2000-ply self-export size and bounded local-import safety.
Added `starbridge_full_length_export_imports_above_legacy_size_cap`, which
drives a real 6-seat Starbridge bot match to the 2000-ply turn-limit terminal,
asserts the exported replay exceeds the old 128 KiB cap, imports it through
`import_replay`, and steps the imported replay to the terminal cursor.

Deviation from plan: the full-length import test exposed that the generic
Starbridge replay import branch was replaying exports with the default
Starbridge seat count instead of the exported 6-seat setup. The fix keeps the
schema bytes unchanged, exposes the existing root `seats` array from the
generic replay parser when it is a string array, tolerates older/object-shaped
seat arrays for other games, and uses the exported or command-inferred
Starbridge seat count during import.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p wasm-api starbridge_full_length_export_imports_above_legacy_size_cap -- --nocapture` passed.
- `cargo test -p wasm-api` passed: 68 unit tests, API surface snapshot, and doctests.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed for all Starbridge traces.
