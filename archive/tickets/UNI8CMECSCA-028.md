# UNI8CMECSCA-028: Pilot-consolidation audit — no speculative API survives without a caller

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Conditional — Yes (may trim unused API in `engine-core` / `game-stdlib` / `game-test-support` as surfaced); None if every helper already has a real caller
**Deps**: UNI8CMECSCA-006, UNI8CMECSCA-009, UNI8CMECSCA-011, UNI8CMECSCA-014, UNI8CMECSCA-015, UNI8CMECSCA-017, UNI8CMECSCA-020, UNI8CMECSCA-021, UNI8CMECSCA-023, UNI8CMECSCA-024, UNI8CMECSCA-025, UNI8CMECSCA-026, UNI8CMECSCA-027

## Problem

After the pilots land, audit that every accepted helper has at least one real game caller, every pilot still retains its game-specific behavior assertions, and no helper accumulated a mechanic-policy flag to satisfy a pilot. If a helper has no convincing caller, remove it rather than keep speculative API; if a pilot dropped a specific assertion for a generic one, restore it. This in-ticket audit gates the build (self-deciding): it either records "all helpers have real callers, no policy flags" or trims the offending API.

## Outcome

Completed: 2026-06-22

Audit result: all accepted helpers have real game/tool callers, no accepted
helper exposes a mechanic-policy flag, and no API trim was needed.

Call-site inventory:

1. C-01 `EffectEnvelope::{public,private_to}` → `games/race_to_n/src/effects.rs`, `games/river_ledger/src/effects.rs`.
2. C-02 canonical `SeatId` grammar → `crates/wasm-api/src/seats.rs`, `games/race_to_n/src/ids.rs`, `games/river_ledger/src/ids.rs`.
3. C-03 `game-stdlib::seat::{SeatCount,SeatCountRange,next_ring_index}` → `games/race_to_n/src/setup.rs`, `games/river_ledger/src/setup.rs`.
4. C-04/C-05 action-tree v1 + stable-byte writer → `games/race_to_n/src/replay_support.rs`, `games/draughts_lite/src/replay_support.rs`, plus pilot byte-contract tests.
5. C-06/C-07 `game-test-support::no_leak` matrix → `games/high_card_duel/tests/visibility.rs`, `games/river_ledger/tests/visibility.rs`.
6. C-08 profile drivers → `games/race_to_n/tests/replay_tests.rs`, `games/river_ledger/tests/replay.rs`, `games/vow_tide/tests/replay.rs`, `games/briar_circuit/tests/replay.rs`, with CLI dispatch in `tools/replay-check/src/main.rs` and `tools/fixture-check/src/main.rs`.
7. C-09 `DeterministicRng::next_index_unbiased_v1` → `games/river_ledger/src/setup.rs`.

Policy-flag grep across `crates/engine-core/src`,
`crates/game-stdlib/src/seat.rs`, and `crates/game-test-support/src` found no
helper signature carrying the audited policy terms (`is_trick_game`,
`private_hand`, `all_in`, `team_count`, `pass_direction`, `reveal_on`, deal /
projection / scoring / terminal / pot / betting / trick / team / graph /
reaction policy). The only match was the `game-test-support` crate-level
production-boundary comment.

Verification:

1. `cargo fmt --all --check`
2. `cargo test --workspace`
3. `bash scripts/boundary-check.sh`
4. `cargo run -p replay-check -- --game race_to_n --all`
5. `cargo run -p replay-check -- --game draughts_lite --all`
6. `cargo run -p replay-check -- --game high_card_duel --all`
7. `cargo run -p replay-check -- --game river_ledger --all`
8. `cargo run -p replay-check -- --game vow_tide --all`
9. `cargo run -p replay-check -- --game briar_circuit --all`

## Assumption Reassessment (2026-06-22)

1. The accepted helpers and their pilots exist after Waves 1–3: C-01 (`EffectEnvelope` ctors → Race/River), C-02 (canonical `SeatId` → Race/River + wasm adapter), C-03 (`game-stdlib::seat` → Race/River), C-04/C-05 (action-tree v1 + writer → Race/Draughts), C-09 (`next_index_unbiased_v1` → River), C-06/C-07 (`game-test-support` no-leak → High Card/River), C-08 (profile drivers → Race/River/Vow/Briar + tools).
2. Spec §5 8C-028 fixes the audit: call-site inventory, API review, no unused framework, no "future-proof" options, no unregistered helper; if a helper has no convincing caller, remove it.
3. Cross-artifact boundary under audit: the full helper surface (`engine-core`, `game-stdlib::seat`, `game-test-support`) vs. its real callers across `games/*` and `tools/*`. The register (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) entries must each map to a landed caller.
4. FOUNDATIONS §4/§11 + ADR 0008: no speculative API survives without callers; no helper carries a mechanic-policy flag (`is_trick_game`, `private_hand`, `all_in`, `team_count`, `pass_direction`, `reveal_on`, etc.) — any such flag is a stop condition routing the work back to the game.
5. No-leak/determinism (§11): trimming unused API must not change any pilot's byte/hash/visibility output — the audit is removal-only where it acts, and re-runs the full pilot suites to prove neutrality.

## Architecture Check

1. A consolidation audit that removes uncalled API keeps the scaffolding minimal and prevents "future-proof" surface from ossifying — exactly the §4/ADR-0008 earned-not-speculative discipline.
2. No backwards-compatibility shim — uncalled API is removed, not deprecated-and-kept.
3. `engine-core` stays noun-free and `game-stdlib`/`game-test-support` carry no mechanic-policy flag (`bash scripts/boundary-check.sh`).

## Verification Layers

1. Every accepted helper has ≥1 real game/tool caller → call-site grep inventory across `games/*`, `tools/*`.
2. No helper carries a mechanic-policy flag → grep-proof on the helper signatures.
3. Each pilot retains its game-specific assertions → diff review + grep-proof those tests still exist.
4. Any API trim is byte/hash/visibility-neutral → `cargo test --workspace`, `cargo run -p replay-check` for the pilot games.

## What to Change

### 1. Call-site inventory + API review

Produce the helper→caller inventory; flag any uncalled API, any policy flag, any pilot that weakened an assertion.

### 2. Trim (only as surfaced)

Remove uncalled API; restore any assertion a pilot weakened. If nothing is uncalled and no flag exists, the ticket records the clean audit and touches no code.

## Files to Touch

- `crates/engine-core/src/{lib.rs,action.rs,replay.rs,rng.rs}` (modify; only as the audit surfaces uncalled API)
- `crates/game-stdlib/src/seat.rs` (modify; only as surfaced)
- `crates/game-test-support/src/{no_leak.rs,profiles.rs}` (modify; only as surfaced)

## Out of Scope

- Adding any new helper or pilot.
- Widening a helper to satisfy a pilot (the opposite of this ticket).
- Register status finalization (UNI8CMECSCA-029) or the `Done`-flip (UNI8CMECSCA-031).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` passes after any trim.
2. A call-site inventory shows every accepted helper has ≥1 real caller; grep shows no mechanic-policy flag on any helper.
3. `bash scripts/boundary-check.sh` and `cargo run -p replay-check -- --game race_to_n --all` (and the other pilot games) pass unchanged.

### Invariants

1. No accepted helper is callerless; no helper carries a mechanic-policy flag.
2. Any API removal is byte/hash/visibility-neutral for every pilot.

## Test Plan

### New/Modified Tests

1. `None — verification/audit ticket; it exercises the existing pilot suites and the call-site inventory, removing API only where uncalled, and adds no test.`

### Commands

1. `cargo test --workspace`
2. `bash scripts/boundary-check.sh`
3. The workspace suite plus the boundary check are the correct boundary — the audit's product is a clean call-site inventory and neutral trims, not new tests.
