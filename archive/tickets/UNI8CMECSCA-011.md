# UNI8CMECSCA-011: Pilot seat-count/ring plumbing in Race to N and River Ledger

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/race_to_n/src/setup.rs`, `games/river_ledger/src/setup.rs`
**Deps**: UNI8CMECSCA-010, UNI8CMECSCA-003

## Problem

Prove `game-stdlib::seat` (UNI8CMECSCA-010) against real call sites: Race to N adopts exact-two `SeatCount` validation; River Ledger adopts 3–6 range validation and ring-index arithmetic. Each game maps the shared structural errors to its **own existing** setup/action diagnostics locally — no diagnostic text and no River betting/button policy moves into shared code. Setup acceptance/rejection, diagnostics, canonical seats, active/dealer progression, replay hashes, and serialization stay unchanged.

## Assumption Reassessment (2026-06-22)

1. `games/race_to_n/src/setup.rs` performs exact-two seat setup; `games/river_ledger/src/setup.rs` performs 3–6 seat setup with ring progression (button/active) and currently uses local index arithmetic (confirmed by grep). `game-stdlib::seat` provides `SeatCount`/`SeatCountRange`/`checked_index`/`next_ring_index` after UNI8CMECSCA-010.
2. Spec §5 8C-011 review boundary: setup acceptance/rejection, diagnostics, canonical seats, active/dealer progression, replay hashes, and serialization unchanged; no River betting/button policy moves. The UNI8CMECSCA-003 packet pins River setup bytes/diagnostics to compare against.
3. Cross-artifact boundary under audit: each game's setup module and the shared `game-stdlib::seat` errors. The games keep ownership of admission ranges and diagnostic text; only the count/ring geometry is shared.
4. FOUNDATIONS §2/§11: setup validation stays in Rust and remains fail-closed; mapping shared structural errors to local diagnostics preserves the existing blocking semantics and messages.
5. Determinism (§11/EC-07): replay hashes and serialization for Race (2) and River (3–6) must be byte-identical to baseline; ring progression order is unchanged — `next_ring_index` reproduces the existing wrap.

## Architecture Check

1. Adopting the shared count/ring geometry while keeping admission/diagnostics local is exactly the §4 boundary the helper was registered under (`MSC-8C-003`); it removes duplicated arithmetic without centralizing policy.
2. No backwards-compatibility shim — local arithmetic is replaced by the shared calls; diagnostics keep their current text.
3. `engine-core` untouched; `game-stdlib` adoption is the earned-helper's first consumers.

## Verification Layers

1. Race exact-two acceptance/rejection + diagnostics unchanged → `cargo test -p race_to_n`.
2. River 3–6 acceptance/rejection, ring/active progression, and diagnostics unchanged → `cargo test -p river_ledger`.
3. Replay hashes and serialization byte-identical → `cargo run -p replay-check -- --game race_to_n --all`, `--game river_ledger --all`, `cargo run -p fixture-check -- --game river_ledger`.
4. No betting/button policy moved → grep-proof that River's betting logic stays in `games/river_ledger`.

## What to Change

### 1. `games/race_to_n/src/setup.rs`

Use `SeatCount`/`SeatCountRange` for exact-two validation; map the structural error to Race's existing setup diagnostic locally.

### 2. `games/river_ledger/src/setup.rs`

Use `SeatCountRange::inclusive(3,6).validate(..)` and `next_ring_index`/`checked_index` for ring/active progression; map errors to River's existing diagnostics; keep betting/button policy local.

## Files to Touch

- `games/race_to_n/src/setup.rs` (modify)
- `games/river_ledger/src/setup.rs` (modify)
- `games/race_to_n/Cargo.toml` (modify — add production `game-stdlib` dependency required by the pilot)
- `games/river_ledger/Cargo.toml` (modify — add production `game-stdlib` dependency required by the pilot)

## Out of Scope

- Adopting the helper in any other game (C-11 waves).
- Moving any diagnostic text, betting, or button/dealer policy into shared code.
- Changing setup bytes, replay hashes, or serialization.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n -p river_ledger` passes (setup acceptance/rejection + diagnostics).
2. `cargo run -p replay-check -- --game race_to_n --all`, `--game river_ledger --all`, and `cargo run -p fixture-check -- --game river_ledger` pass with unchanged hashes.
3. `cargo test --workspace` passes.

### Invariants

1. Setup diagnostics, replay hashes, and serialization are identical to the UNI8CMECSCA-003 baseline.
2. No River betting/button/dealer policy is relocated to `game-stdlib`.

## Test Plan

### New/Modified Tests

1. `None — no new test; existing Race/River setup, replay, and fixture suites plus the UNI8CMECSCA-003 baseline are the regression guard.`

### Commands

1. `cargo test -p race_to_n -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger`
3. The games' suites plus `replay-check`/`fixture-check` are the correct boundary — behavior-neutrality is proven by unchanged diagnostics and byte identity.

## Outcome

Completed: 2026-06-22

What changed:
- Race to N setup now validates its exact seat count through `SeatCountRange`, mapping structural errors back to the existing `invalid_seat_count` diagnostic.
- River Ledger setup now validates its 3-6 seat range through `SeatCountRange` and uses `SeatCount::checked_index` / `next_ring_index` for setup-time button, blind, and preflop active-seat ring arithmetic while keeping modulo button policy and diagnostics local.
- Added normal `game-stdlib` dependencies to Race to N and River Ledger so the production setup code can use the helper.
- Flipped `MSC-8C-003` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from `candidate` to `accepted`.

Deviations:
- The original Files to Touch omitted the two game `Cargo.toml` files; adding `game-stdlib` as a production dependency was required for the pilot adoption to compile.
- No setup diagnostic text, replay/fixture bytes, betting policy, button/dealer policy, or shared generated seat enum was changed.

Verification:
- `cargo fmt --all --check`
- `cargo test -p race_to_n -p river_ledger`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p fixture-check -- --game river_ledger`
- `cargo test --workspace`
- `rg -n "\\b(dealer|button|blind|bet|turn|pass|partner|partnership|team|role)\\b" crates/game-stdlib/src/seat.rs` returned no matches.
- `node scripts/check-doc-links.mjs`
