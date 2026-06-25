# GAT19MELLEDFIV-010: Turn lifecycle, going out, and stock-exhaustion round settlement

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{rules,state}.rs`; go-out / stock-exhaustion golden traces
**Deps**: GAT19MELLEDFIV-008, GAT19MELLEDFIV-009

## Problem

Meldfall Ledger needs the turn phase model and round-ending conditions: draw -> optional melds/lay-offs -> discard (or go out). A seat goes out by melding/laying off its entire hand (no final discard required) OR by melding/laying off all but one card and discarding it. If the stock is exhausted and the active seat cannot or will not legally draw from the discard pile, the round ends and settles. Stale/wrong-phase actions produce diagnostics.

## Assumption Reassessment (2026-06-25)

1. Draw (GAT19MELLEDFIV-009), meld (006), lay-off (008), and the `RoundState.phase` field (005) exist; turn-phase patterns follow `games/blackglass_pact/src/rules.rs` / `games/vow_tide/src/rules.rs` (confirmed during reassessment).
2. Spec §3.1 (Turn flow / Going out / Stock exhaustion rows), Appendix A.2 (Melding-timing / Going-out / Stock-exhaustion rows) define the lifecycle; no floating, no discard-required go-out, no reshuffle.
3. Cross-artifact: the boundary is the `TurnPhase` state machine + the `pending_pickup` commitment (009) — turn-finish and go-out must both respect an unsatisfied commitment.
4. FOUNDATIONS §2: phase legality, go-out detection, and stock-exhaustion settlement are Rust-owned; TypeScript never decides whether a seat may go out.
5. FOUNDATIONS §11 determinism: round termination is a deterministic function of state; the settlement traces replay byte-identically (full scoring is GAT19MELLEDFIV-011; this ticket fixes the round-end transition).

## Architecture Check

1. Encoding the draw/meld/lay-off/discard phases as an explicit `TurnPhase` machine keeps go-out and stock-exhaustion as deterministic transitions, with diagnostics for out-of-phase actions, rather than scattered guards.
2. No backwards-compatibility shims.
3. `engine-core` untouched; the state machine is crate-local.

## Verification Layers

1. Go-out by final discard and go-out without final discard both terminate the round -> `go-out-by-final-discard.trace.json`, `go-out-without-final-discard.trace.json`.
2. Stock exhaustion settles the round when no legal/accepted discard draw remains -> `stock-exhausted-round-settlement.trace.json`.
3. Out-of-phase / wrong-seat actions diagnosed, not panicked -> `cargo test -p meldfall_ledger` lifecycle tests.

## What to Change

### 1. `rules.rs` — turn lifecycle

Draw -> meld/lay-off (zero or more) -> discard / go-out phase model; go-out by emptying the hand (no final discard) or by discarding the last card; stale/wrong-phase/wrong-seat diagnostics; turn-finish blocked while a pickup commitment is unsatisfied.

### 2. `state.rs` + `rules.rs` — stock exhaustion

Detect stock exhaustion; when the active seat cannot/will-not legally draw from the discard pile, end and settle the round (settlement scoring is GAT19MELLEDFIV-011; this ticket owns the transition + next-round setup hook).

### 3. Lifecycle golden traces

`discard-after-draw-turn-end.trace.json`, `go-out-by-final-discard.trace.json`, `go-out-without-final-discard.trace.json`, `stock-exhausted-round-settlement.trace.json`.

## Files to Touch

- `games/meldfall_ledger/src/rules.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/state.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/rules.rs` (modify — lifecycle/go-out/exhaustion cases)
- `games/meldfall_ledger/tests/golden_traces/discard-after-draw-turn-end.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/go-out-by-final-discard.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/go-out-without-final-discard.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/stock-exhausted-round-settlement.trace.json` (new)

## Out of Scope

- Score computation and cumulative match totals (GAT19MELLEDFIV-011).
- Floating / discard-required go-out / reshuffle (spec out-of-scope).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger`: normal draw/meld/discard turn ends; go-out with and without final discard both terminate the round.
2. Stock exhaustion with no legal/accepted discard draw settles the round; out-of-phase actions diagnosed.
3. `cargo test --workspace` passes.

### Invariants

1. Go-out and stock-exhaustion detection are Rust-owned and deterministic (FOUNDATIONS §2/§11).
2. Turn-finish/go-out respects an unsatisfied pickup commitment (GAT19MELLEDFIV-009).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — phase machine, both go-out paths, stock-exhaustion, diagnostics.
2. `games/meldfall_ledger/tests/golden_traces/{discard-after-draw,go-out-by-final-discard,go-out-without-final-discard,stock-exhausted}-*.trace.json`.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. Score values at settlement are asserted in GAT19MELLEDFIV-011; this ticket asserts the round-end transition.
