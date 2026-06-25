# GAT19MELLEDFIV-004: Variable 2–6 setup, deterministic single-deck deal, and card model

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{cards,setup}.rs`; setup golden traces
**Deps**: GAT19MELLEDFIV-003

## Problem

Meldfall Ledger needs the local card model and deterministic setup: a 52-card single deck, variable 2–6 seat declaration with diagnostics for unsupported counts, the deal (13 cards for 2 seats, 7 for 3–6), one face-up initial discard, the rest as face-down stock, and dealer/start-seat rotation (left of dealer acts first, clockwise). All randomness uses the engine's deterministic RNG contract.

## Assumption Reassessment (2026-06-25)

1. `games/river_ledger/src/setup.rs` (N-seat deterministic shuffle/deal, private-hand distribution) and `games/vow_tide/src/setup.rs` (variable seat-count declaration + diagnostics) are the patterns; `crates/game-stdlib/src/seat.rs` exposes `SeatCount`/`SeatCountRange` validators reused here (confirmed during reassessment).
2. Spec §3.1 (Deal/Seat-count rows) and Appendix A.2 (deal counts: 13 for 2, 7 for 3–6; left-of-dealer clockwise) define behavior; the crate skeleton (`cards.rs`, `setup.rs` stubs, `ids.rs`) exists from GAT19MELLEDFIV-003.
3. Cross-artifact: `cards.rs` defines the local `Suit`/`Rank`/`CardId` nouns this game owns — the shared boundary under audit is the `engine-core` kernel, which must NOT learn these nouns.
4. FOUNDATIONS §2 deterministic randomness: shuffle/deal use only `engine-core`'s deterministic RNG contract (no wall-clock seeding); identical seed + seat count produce identical deals — the setup traces assert this.
5. FOUNDATIONS §11 no-leak: the initial deal places hidden cards into private hands and the unseen stock; setup traces must show the public observer seeing counts only, never hand cards or stock order. The redaction enforcement surface itself lands in GAT19MELLEDFIV-012/013, but this ticket must not introduce a leak path (deal output keyed by seat).

## Architecture Check

1. A dedicated `cards.rs` + `setup.rs` pair keeps deck construction and dealing local and deterministic, reusing only the behavior-free `seat` validators rather than inventing a shared shuffle helper (first official use stays local).
2. No backwards-compatibility shims.
3. `engine-core` stays noun-free — `Suit`/`Rank`/`CardId` are crate-local; `game-stdlib` gains no rummy helper.

## Verification Layers

1. Deck is 52 unique cards; deal counts correct per seat count -> `cargo test -p meldfall_ledger` (setup unit tests).
2. Deterministic deal (seed + seat count -> identical layout) -> golden trace / deterministic replay check on setup traces.
3. Unsupported seat counts rejected with diagnostics -> rule/unit test asserting setup diagnostic for 1 and 7 seats.

## What to Change

### 1. `cards.rs`

Local `Suit`, `Rank`, `CardId`, card point values (ace 15, K/Q/J/10 = 10, 2–9 pip), deterministic 52-card deck construction, rank ordering, and ace-low/ace-high run helpers (no around-the-corner).

### 2. `setup.rs`

Variable 2–6 seat declaration reusing `game_stdlib::seat` validators, setup diagnostics for unsupported counts, deterministic shuffle/deal via the engine RNG contract, 13/7-card deal, initial face-up discard, face-down stock, dealer/start-seat rotation.

### 3. Setup golden traces

`tests/golden_traces/` setup traces for 2p (13-card), 4p default (7-card), 6p max-seat, and invalid seat counts (below/above), each asserting public-observer count-only visibility.

## Files to Touch

- `games/meldfall_ledger/src/cards.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/setup.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/golden_traces/setup-2p-13-card-deal.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/setup-4p-default.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/setup-6p-max-seat.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/invalid-seat-count-below.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/invalid-seat-count-above.trace.json` (new)
- `games/meldfall_ledger/tests/rules.rs` (modify; created by GAT19MELLEDFIV-003 — setup/deal cases)

## Out of Scope

- Turn flow, melds, draw/discard, scoring (later pipeline tickets).
- The export/no-leak harness (GAT19MELLEDFIV-013) — setup traces here assert basic count-only public visibility only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` setup tests: deck = 52 unique, deal counts (13 for 2; 7 for 3–6), initial discard + stock sizes.
2. Invalid seat counts (1, 7) produce a setup diagnostic, not a panic.
3. `cargo run -p replay-check -- --game meldfall_ledger --all` replays the setup traces deterministically (registration lands in GAT19MELLEDFIV-016; until then the traces are asserted by `cargo test`).

### Invariants

1. Card identities never appear in public-observer projections from setup (count-only).
2. Shuffle/deal is deterministic under the engine RNG contract; no `std::time` seeding.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — setup/deal/diagnostic unit tests.
2. `games/meldfall_ledger/tests/golden_traces/setup-*.trace.json` — deterministic setup traces (2p/4p/6p/invalid).

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. `replay-check` registration is GAT19MELLEDFIV-016; setup-trace determinism is asserted here via `cargo test` until then.
