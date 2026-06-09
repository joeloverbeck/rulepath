# GAT101PLATRI-005: Deterministic setup and internal state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/plain_tricks/src/setup.rs`, `games/plain_tricks/src/state.rs`. No `engine-core`/`game-stdlib` change (unless GAT101PLATRI-002 decided *promote*, in which case setup consumes the GAT101PLATRI-003 helper).
**Deps**: GAT101PLATRI-004

## Problem

The game needs deterministic per-round setup (18-card deck construction, seeded shuffle, 6+6 deal with a hidden 6-card tail, deal rotation across two rounds from one continuing RNG stream) and the internal state model (phase, hands, tail, current trick, trick counts, round index, leader, completed-trick history, terminal outcome, effect history, freshness token). Hidden hands and the tail are internal-only.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{ids,lib}.rs` and data manifests exist from GAT101PLATRI-004. The shuffle discipline mirrors `games/high_card_duel/src/setup.rs` and `games/poker_lite/src/setup.rs` (seeded deterministic RNG, no new kernel RNG concept).
2. Spec §4 (`state.rs`, `setup.rs`) and appendix A2 fix setup: 18-card deck in stable id order, seeded shuffle, deal 6/6 + 6 internal tail, round 1 seat_0 leads, round 2 reshuffle from the continuing stream and seat_1 leads.
3. Shared boundary under audit: the deterministic-RNG `Seed` contract from `engine-core` (consumed, not modified) and — conditionally — the GAT101PLATRI-003 `game-stdlib` shuffle helper. Per the decomposition pattern, this ticket `Deps` on the stable predecessor (004) and consumes the conditional helper only if GAT101PLATRI-002 decided *promote*; otherwise the shuffle stays local here.
4. FOUNDATIONS §2 (deterministic randomness owned by Rust) is under audit.
5. Enforcement surface: deterministic replay/hash & the no-leak firewall (§11). The shuffle/deal must be byte-reproducible from the seed, and the tail + both hands must live only in internal state — never in any view/effect/export projection (those projections are built in GAT101PLATRI-008/009/011). No nondeterministic input (wall-clock) enters setup.

## Architecture Check

1. Building setup + internal state as one diff (vs. splitting deck-build from state) is cohesive: the state shape is defined by what setup produces, and both are pre-legality.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; if *promote*, the shuffle calls the earned `game-stdlib` helper (FOUNDATIONS §4), else it stays local — no kernel noun either way.

## Verification Layers

1. Deterministic shuffle/deal from seed -> deterministic replay-hash check (internal serialization helper + later golden traces).
2. Tail + both hands are internal-only at the state layer -> codebase grep-proof (no public accessor) + forward no-leak tests in GAT101PLATRI-009.
3. Deal rotation (seat_0 round 1, seat_1 round 2) from one continuing RNG stream -> unit test in GAT101PLATRI-007/golden trace `round-close-deal-rotation`.
4. `engine-core` noun-free -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. `games/plain_tricks/src/setup.rs`

Construct the 18-card deck in stable id order; seeded shuffle (local, or via the GAT101PLATRI-003 helper if promoted); deal 6/6 with a 6-card internal tail; round-2 reshuffle from the continuing RNG stream with deal/lead rotation. Deal order is a stable implementation detail covered by golden traces later.

### 2. `games/plain_tricks/src/state.rs`

Define internal state: phase, seats, per-round shuffled deck, private hands, hidden tail, current trick (led card/suit, plays), trick counts, round index, round leader, completed-trick history, terminal outcome, effect history, freshness token. Add stable internal serialization helpers. Hands and tail are internal-only with no public accessor.

## Files to Touch

- `games/plain_tricks/src/setup.rs` (new)
- `games/plain_tricks/src/state.rs` (new)

## Out of Scope

- Legal action generation / validation (GAT101PLATRI-006).
- Trick resolution / scoring / rotation transitions (GAT101PLATRI-007).
- View projection and effects (GAT101PLATRI-008/009) — setup only stores internal state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` setup/state unit tests pass (deck size 18, 6+6 deal, 6 tail, rotation).
2. Identical seed produces byte-identical internal serialization across runs.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Setup is deterministic from `Seed`; no wall-clock/nondeterministic input enters canonical state (FOUNDATIONS §2/§11).
2. Hidden hands and the tail exist only in internal state with no public accessor (FOUNDATIONS §11 no-leak firewall, enforced by tests in GAT101PLATRI-009).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/rules.rs` (or `src/setup.rs` unit tests) — deck construction, deal sizes, tail size, deal rotation, determinism.
2. Internal serialization round-trip test — stable field order from a fixed seed.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo test -p plain_tricks && bash scripts/boundary-check.sh`
3. Per-crate scope is correct: setup/state determinism is fully provable within the crate; cross-tool replay belongs to GAT101PLATRI-011.
