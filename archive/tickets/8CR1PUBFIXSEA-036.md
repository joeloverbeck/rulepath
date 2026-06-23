# 8CR1PUBFIXSEA-036: Token Bazaar C-08 public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/token_bazaar` (`tests/replay.rs`); export bytes unchanged
**Deps**: 8CR1PUBFIXSEA-032, 8CR1PUBFIXSEA-018

## Problem

Token Bazaar has no `public-export-v1` profile adapter around its `PublicReplayExport` round trip. Add a parallel `PublicExportV1Driver` adapter (public visibility, Rust/WASM export byte authority) validating the export profile, then delegating to the existing `export_public_replay` / `import_public_export` round trip and hidden-absence assertions. C-02 (`-018`) owns the seat-spelling bytes of the export, so this ticket introduces no second export-format change. The dev-only `game-test-support` edge is already added by `-032`.

## Assumption Reassessment (2026-06-23)

1. `crates/game-test-support/src/profiles.rs` exposes `PublicExportV1Driver` and the `PUBLIC_EXPORT_V1`/`PROFILE_VERSION_V1` constants; `games/token_bazaar/src/replay_support.rs` defines `PublicReplayExport`, `export_public_replay`, and `import_public_export`; `games/token_bazaar/tests/replay.rs` exists. The dev-only `game-test-support` dependency is added by `-032`. Confirmed during reassessment.
2. Spec §3.7 and §5.9 (task `8C-R1-520`) classify Token Bazaar `public-export-v1` as `migrate`; the driver validates public visibility + Rust/WASM byte authority and delegates to the existing export/import round trip. C-02 (`-018`) owns the export seat bytes — this ticket must not introduce a second export format change. MSC-8C-008 owns evidence-profile drivers; ADR-0004 keeps public export observer-safe.
3. Cross-artifact: the export surface is `games/token_bazaar/src/replay_support.rs` (`PublicReplayExport`); the adapter wraps it read-only. The final public byte surface is characterized once `-018` lands (hence the dependency). Before-baseline from `-001`.
4. §11 no-leak firewall and ADR-0004 motivate this ticket: the public export must remain observer-safe; the driver asserts hidden-absence and claims no seat-private data.
5. Enforcement surface = the public-export visibility and Rust/WASM export byte authority; the adapter validates `public-export-v1` metadata, exercises the existing round trip and hidden-absence assertions, and changes no export byte — no leak, no second format change.

## Architecture Check

1. A parallel typed public-export adapter classifies the export evidence without re-implementing or re-formatting the export — the C-02 output ticket owns the seat bytes.
2. No backwards-compatibility shim is introduced; the adapter is additive dev-only test code.
3. `engine-core` stays noun-free (§3); `game-test-support` stays dev-only (§4/C-06); export behavior stays in the game (§2).

## Verification Layers

1. `public-export-v1` metadata validated (public visibility, Rust/WASM byte authority) -> `PublicExportV1Driver` assertion in `tests/replay.rs`.
2. Export/import round trip + hidden-absence still pass after delegation -> `cargo test -p token_bazaar` + no-leak visibility assertion.
3. Export bytes unchanged (C-02 owns seat bytes) -> `cargo run -p replay-check -- --game token_bazaar --all` + `git diff` shows no export change here.

## What to Change

### 1. Add the public-export profile adapter test

In `games/token_bazaar/tests/replay.rs`, build a `PublicExportV1Driver` with public visibility and Rust/WASM export byte authority; validate; then delegate to the existing `export_public_replay` / `import_public_export` round trip and the hidden-absence assertions. Do not introduce a second export-format change.

## Files to Touch

- `games/token_bazaar/tests/replay.rs` (modify; dev-dep edge created by 8CR1PUBFIXSEA-032)

## Out of Scope

- Any change to `PublicReplayExport` format or export seat bytes (owned by C-02 `-018`).
- Re-adding the `game-test-support` dev-dependency (owned by `-032`).
- Adding a `seat-private-export-v1` claim (observer and seat views are identical; no seat-private export exists).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` is green, including the new public-export-profile validation and the hidden-absence assertion.
2. `cargo run -p replay-check -- --game token_bazaar --all` passes; export bytes byte-identical to the post-`-018` baseline.
3. The export/import round trip remains observer-safe (no-leak assertion passes).

### Invariants

1. The public export remains observer-safe; no seat-private data is claimed or exposed.
2. No second export-format change is introduced; C-02 owns the export seat bytes.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/replay.rs` — `public-export-v1` profile validation wrapping the existing export/import round trip and hidden-absence assertions.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo run -p replay-check -- --game token_bazaar --all`
3. The per-game export round trip plus replay-check are the correct boundary: this is the game's public-export projection, byte-owned by C-02.

## Outcome

Completed on 2026-06-23.

Added `public_export_v1_driver_round_trips_public_export_fixture` in
`games/token_bazaar/tests/replay.rs`. The test builds a typed
`ProfileArtifact` with `public-export-v1` / `v1`, public visibility,
`token_bazaar::replay_support` validator ownership, and
`token_bazaar::replay_support` canonical byte authority, then validates with
`PublicExportV1Driver::new("token_bazaar::replay_support")` before
regenerating the public export from `wasm-exported.trace.json`, importing it,
checking the pinned public-export hash, checking hidden/debug/internal absence,
and delegating to the existing fixture replay/export assertions.

No `PublicReplayExport` format, export seat bytes, golden trace bytes, or
`replay_support` bytes changed.

Verification:

1. `cargo test -p token_bazaar public_export_v1_driver_round_trips_public_export_fixture -- --exact`
2. `cargo test -p token_bazaar`
3. `cargo run -p replay-check -- --game token_bazaar --all`
4. `cargo fmt --all -- --check`
5. `git diff --name-only -- games/token_bazaar/tests/golden_traces games/token_bazaar/src/replay_support.rs`
