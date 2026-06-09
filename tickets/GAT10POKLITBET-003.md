# GAT10POKLITBET-003: Deterministic setup and internal state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/poker_lite/src/setup.rs`, `games/poker_lite/src/state.rs`. No `engine-core` / `game-stdlib` change.
**Deps**: GAT10POKLITBET-002

## Problem

`poker_lite` needs a deterministic setup (six-card deck construction, seeded shuffle, private deal, hidden center card, opening contributions) and an internal state type that strictly separates hidden information (private cards, center card before reveal, deck tail) from public information (contributions, shared pool, round state, phase). This is the substrate the rules engine, visibility projection, and replay all build on.

## Assumption Reassessment (2026-06-08)

1. The seeded-shuffle discipline already exists: `games/high_card_duel/src/setup.rs` performs a deterministic local shuffle using `engine-core`'s seeded RNG (`SeededRng::from_seed`, `DeterministicRng` trait in `crates/engine-core/src/rng.rs`). `poker_lite` reuses the same contract — no new kernel RNG concept (spec §4 setup.rs, §A2 step 2).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §A2 Setup steps 1–7 and §B1 `PokerLiteState`) fixes setup order (deck in stable id order → shuffle → deal seat_0, seat_1, center, leave deck tail) and the internal state fields (`phase`, `active_seat`, `private_cards: [_;2]`, `center_card`, `center_visible`, `deck_tail`, `contributions: [u8;2]`, `shared_pool: u8`, `round`, `terminal_outcome`, `freshness_token`). Initial: `contributions=[1,1]`, `shared_pool=2`, `phase=PledgeRound{0}`, `active_seat=seat_0`.
3. Cross-artifact boundary under audit: the `engine-core` RNG contract (`crates/engine-core/src/rng.rs`) consumed by setup, and the internal/public state split that `visibility.rs` (GAT10POKLITBET-007) and `replay_support.rs` (009) project from. `private_cards`, hidden `center_card`, and `deck_tail` are internal-only fields — never serialized into a public projection.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: setup and deterministic randomness are Rust-owned; TypeScript never reconstructs the deck. Restated before trusting the spec narrative.
5. Determinism + no-leak substrate surface under audit (§11): seeded setup must produce identical state for identical seed+version (replay determinism, enforced in 009); the hidden-field separation here is the substrate the no-leak firewall (GAT10POKLITBET-007) and ADR 0004 export taxonomy (009) enforce. Confirm setup introduces no path where deck tail / hidden cards could reach a public field, and no wall-clock / nondeterministic input enters state.

## Architecture Check

1. A typed internal state with hidden fields physically distinct from public fields makes the no-leak firewall a projection concern (drop hidden fields) rather than a redaction concern (scrub a shared blob) — cleaner and harder to leak through. Matches `high_card_duel` / `secret_draft` internal-state shape.
2. No backwards-compatibility aliasing/shims — new modules.
3. `engine-core` stays noun-free (deck/card/crest types are crate-local, §3); RNG comes from the existing kernel contract, not a new one; no `game-stdlib` promotion (§4).

## Verification Layers

1. Deterministic setup (same seed+version → same deal) -> `cargo test -p poker_lite` setup determinism unit test (replay-hash style assertion deferred to 009).
2. Setup correctness (deal order, opening contributions, shared pool = 2) -> setup unit test against spec §A2.
3. Internal/public field separation (hidden fields exist and are not `pub`-exposed in a public-view-shaped accessor) -> codebase grep-proof + state unit test; full no-leak assertion deferred to GAT10POKLITBET-007.
4. Boundary cleanliness -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. `games/poker_lite/src/setup.rs`

Construct the six-card deck in stable id order; shuffle with `engine-core` seeded RNG (mirror `high_card_duel`'s local shuffle helper); deal top→`seat_0`, next→`seat_1`, next→`center_card`, remainder→internal `deck_tail`; set `phase=PledgeRound{round_index:0}`, `active_seat=seat_0`, `contributions=[1,1]`, `shared_pool=2`, `round_state`=no outstanding pledge.

### 2. `games/poker_lite/src/state.rs`

Define `PokerLiteState`, `Phase`, `PledgeRoundState`, `TerminalOutcome` (`YieldWin`/`ShowdownWin`/`Split` per spec §A1), and a `FreshnessToken`. Keep `private_cards`, hidden `center_card`, `deck_tail` internal; provide stable internal serialization helpers for replay/test use (not public projection).

## Files to Touch

- `games/poker_lite/src/setup.rs` (new)
- `games/poker_lite/src/state.rs` (new)
- `games/poker_lite/src/lib.rs` (modify — add `mod setup; mod state;` and re-exports)

## Out of Scope

- Legal action generation, validation, transitions, accounting (GAT10POKLITBET-004/005).
- Public/private view projection and no-leak tests (GAT10POKLITBET-007).
- Effects and replay export (GAT10POKLITBET-006/009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` setup determinism test: identical seed → identical `private_cards`/`center_card`/`deck_tail`/`contributions`.
2. Setup correctness test: opening `contributions=[1,1]`, `shared_pool=2`, `phase=PledgeRound{0}`, `active_seat=seat_0`.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Setup is fully deterministic from `(seed, rules version)`; no nondeterministic input enters state (§2/§11).
2. Hidden fields (`private_cards`, pre-reveal `center_card`, `deck_tail`) are internal-only and never part of a public-view-shaped accessor (§11 no-leak substrate).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/setup.rs` (inline `#[cfg(test)]`) — deterministic deal + opening accounting.
2. `games/poker_lite/src/state.rs` (inline `#[cfg(test)]`) — state construction + internal serialization round-trip.

### Commands

1. `cargo test -p poker_lite`
2. `cargo build -p poker_lite`
3. `bash scripts/boundary-check.sh` — boundary surface; deterministic replay-hash assertions are the correct boundary for GAT10POKLITBET-009, not here.
