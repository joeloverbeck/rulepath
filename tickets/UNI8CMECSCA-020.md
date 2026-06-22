# UNI8CMECSCA-020: Pilot the no-leak harness in High Card Duel (two-seat)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/tests/visibility.rs`, `games/high_card_duel/Cargo.toml` (`[dev-dependencies]`)
**Deps**: UNI8CMECSCA-019, UNI8CMECSCA-003

## Problem

Prove the no-leak harness (UNI8CMECSCA-019) against High Card Duel's two-seat hidden-information surfaces. The public observer and both seat viewers must cover view, action tree, diagnostics, effects, replay/export, and any bot explanation/candidate surface present. Existing reveal-specific assertions are retained or made strictly stronger — never replaced by a weaker generic smoke. This is the first `game-test-support` consumer (added under `[dev-dependencies]`).

## Assumption Reassessment (2026-06-22)

1. `games/high_card_duel/tests/visibility.rs` and `tests/replay.rs` exist with two-seat hidden-card visibility/reveal assertions; `tests/golden_traces/{public-replay-export-import,seat-private-view}.trace.json` exist. `game-test-support::no_leak` exists after UNI8CMECSCA-019. The game still owns private-card projection, reveal, and export policy.
2. Spec §4.5 + §5 8C-020 fix the boundary: two-seat C-07 geometry across view/action/effect/export surfaces; the game keeps private-card projection/reveal/export; existing specific tests are retained or strengthened. The UNI8CMECSCA-003 packet pins High Card's public/seat-private artifacts and canary scopes.
3. Cross-artifact boundary under audit: High Card's visibility test surface and the generic harness. `game-test-support` is added only under `[dev-dependencies]` (no normal edge — guarded by UNI8CMECSCA-018).
4. FOUNDATIONS §11 no-leak firewall + ADR 0004 (EC-16): facts private to one seat must not reach the other seat, the public observer, DOM/logs, or replay exports; canaries are test-generated and absent from committed fixtures (EC-18).
5. No-leak/visibility surface under audit (§11/EC-16): the pilot enumerates source-seat × {observer, seat0, seat1} × surfaces and asserts `MustBeAbsent`/`MustBePresent`/`NotApplicable` via game-supplied snapshot/expectation/containment closures; reveal semantics stay game-owned and their specific assertions are preserved or strengthened.

## Architecture Check

1. Driving the shared geometry from real two-seat tests proves the harness without moving reveal/projection policy out of the game.
2. No backwards-compatibility shim — the harness is adopted; existing specific tests remain (or strengthen), not aliased away.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` enters only as a dev-dependency.

## Verification Layers

1. Full declared matrix passes for observer + both seats across view/action/diagnostic/effect/replay-export (and any bot explanation/candidate surface) → `cargo test -p high_card_duel`.
2. Existing reveal-specific assertions retained or strengthened → diff review + grep-proof they still exist.
3. No private canary in committed public/seat-private artifacts → grep-proof on `games/high_card_duel/tests/golden_traces/*`.
4. `game-test-support` is a dev-only edge → `cargo tree --workspace -e normal --invert game-test-support` (no high_card_duel normal edge).

## What to Change

### 1. `games/high_card_duel/Cargo.toml`

Add `game-test-support` under `[dev-dependencies]`.

### 2. `games/high_card_duel/tests/visibility.rs`

Adopt `assert_pairwise_no_leak` for the two-seat matrix with game-supplied snapshot/expectation/containment closures and test-generated canaries; keep (or strengthen) the existing reveal-specific assertions.

## Files to Touch

- `games/high_card_duel/Cargo.toml` (modify)
- `games/high_card_duel/tests/visibility.rs` (modify)

## Out of Scope

- The N-seat pilot (River UNI8CMECSCA-021).
- Moving private-card projection/reveal/export policy into the harness.
- Any normal/build dependency on `game-test-support`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` passes, including the full pairwise matrix for observer + both seats.
2. The pre-existing reveal-specific visibility assertions remain present (or are strengthened).
3. `cargo run -p replay-check -- --game high_card_duel --all` passes.

### Invariants

1. No seat-private fact leaks to the other seat, the observer, or any export surface.
2. No test canary appears in committed public/seat-private artifacts; `game-test-support` is dev-only.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs` — pairwise no-leak matrix (observer + seat0 + seat1) layered over the retained reveal-specific assertions.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
3. The game's visibility suite plus `replay-check` are the correct boundary — the matrix and the export round-trips are exercised together.
