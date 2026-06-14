# GAT15RIVLEDTEX-009: Pairwise N-seat no-leak harness and visibility tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/tests/visibility.rs`; no-leak golden traces
**Deps**: GAT15RIVLEDTEX-008

## Problem

N-player no-leak is first-class for Gate 15: every ordered pair of distinct seats in 3-, 4-, 5-, and 6-seat matches, plus the public observer and wrong-seat diagnostics, must be proven to leak no hidden fact through projections, effects, action diagnostics, and bot-visible payloads. This ticket builds the pairwise no-leak test harness and the no-leak golden traces.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/tests/visibility.rs` is the precedent for crate-level no-leak tests; the Infra D N-player no-leak harness (recorded `Done` in `specs/README.md`) is the seam this extends with Hold'Em hidden facts across 3–6 seats; this ticket consumes the `visibility.rs` projections + view hashes from 008.
2. `specs/...-base.md` §6 exit row 2, §7.3 (pairwise no-leak matrix), and §4.1 G15-RL-006 fix `RL-VIS-*` no-leak coverage for every `(A,B)` pair where `A != B` in `N ∈ {3,4,5,6}`.
3. Cross-artifact boundary under audit: the harness asserts on the projections/effects/diagnostics from 008 (no new production logic); the no-leak golden traces record command streams whose viewer-scoped projections expose no hidden fact.
4. FOUNDATIONS §11 acceptance invariant (pairwise seat-private redaction) motivates this ticket: facts private to seat A must not reach seat B, the public observer, effect logs, bot explanations, or diagnostics unless Rust made them public.
5. §11 no-leak firewall is the enforcement surface: hole cards, folded unrevealed cards, burn cards, deck order/tail, future community cards, raw full trace, and private diagnostics must be absent from every unauthorized viewer in all ordered pairs; this ticket is the firewall test, not new behavior. Confirm the harness exercises every ordered pair, not just a sample.

## Architecture Check

1. A data-driven pairwise sweep over `(N, A, B)` makes no-leak coverage exhaustive and regression-proof rather than spot-checked, the strongest available proof for the firewall.
2. No backwards-compatibility aliasing/shims — test-only ticket exercising 008's projections.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — tests + trace fixtures only.

## Verification Layers

1. Every ordered seat pair in 3/4/5/6 leaks no private fact -> `cargo test -p river_ledger --test visibility` pairwise sweep.
2. Public observer sees no hole/burn/deck/future-community/private-diagnostic fact -> observer no-leak test.
3. Wrong-seat/stale diagnostics carry a public reason only -> diagnostic no-leak test.
4. No-leak golden traces (`deal-private-no-leak`, `public-observer-no-leak`, `seat-private-view`, `wrong-seat-diagnostic`) -> `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; firewall proven here by the visibility tests.

## What to Change

### 1. `games/river_ledger/tests/visibility.rs`

Pairwise no-leak tests over every ordered `(A,B)` pair for `N ∈ {3,4,5,6}`; observer no-leak; wrong-seat/stale diagnostic no-leak; effect-log and action-tree no-leak assertions.

### 2. No-leak golden traces

Add `deal-private-no-leak`, `public-observer-no-leak`, `seat-private-view`, `wrong-seat-diagnostic` traces.

## Files to Touch

- `games/river_ledger/tests/visibility.rs` (new)
- `games/river_ledger/tests/golden_traces/deal-private-no-leak.trace.json` (new)
- `games/river_ledger/tests/golden_traces/public-observer-no-leak.trace.json` (new)
- `games/river_ledger/tests/golden_traces/seat-private-view.trace.json` (new)
- `games/river_ledger/tests/golden_traces/wrong-seat-diagnostic.trace.json` (new)

## Out of Scope

- Public replay export/import no-leak and serialization (GAT15RIVLEDTEX-010).
- Browser DOM/storage/log no-leak (GAT15RIVLEDTEX-018 e2e).
- Bot-explanation no-leak (GAT15RIVLEDTEX-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test visibility` — every ordered pair in 3/4/5/6, observer, wrong-seat diagnostic.
2. No unauthorized viewer's projection/effect/diagnostic contains a hole, burn, deck-tail, future-community, or private-diagnostic fact.
3. `cargo test -p river_ledger` passes overall.

### Invariants

1. Pairwise seat-private redaction holds for all `(A,B)`, observer, and diagnostics (§11).
2. The harness exercises every ordered pair, not a sample (§11 firewall completeness).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` (new) — pairwise/observer/diagnostic no-leak sweep.
2. No-leak golden traces (4 files, new) — viewer-scoped no-leak evidence.

### Commands

1. `cargo test -p river_ledger --test visibility`
2. `cargo test -p river_ledger`
3. The crate-level visibility test is the correct boundary; export and browser no-leak are exercised in 010/018.
