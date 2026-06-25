# GAT19MELLEDFIV-008: Lay-off onto any player's meld with score-credit attribution (first-use primitive)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{rules,effects}.rs`; lay-off golden traces; first-use ledger entry ML-PP-004
**Deps**: GAT19MELLEDFIV-007

## Problem

A seat may lay off a card onto any existing public meld — its own or an opponent's — when the resulting meld stays legal, with score credit going to the seat that plays the card (not the meld's `origin_seat`). Meldfall Ledger needs this lay-off legality, the score-credit attribution, the public effect, and invalid-lay-off diagnostics. First official use of lay-off-onto-any-tableau (`ML-PP-004`, `local-only`).

## Assumption Reassessment (2026-06-25)

1. No existing game has lay-off behavior (confirmed during reassessment). The tableau model with separate `origin_seat`/`played_by` exists from GAT19MELLEDFIV-007; meld legality from GAT19MELLEDFIV-006.
2. Spec §3.1 (Laying off row), Appendix A.2 (Lay-off row: add to any combination, credit the adder), Appendix B.3 (score-credit model), and Appendix D (`ML-PP-004`) define behavior.
3. Cross-artifact: the boundary is the tableau `MeldGroup` legality + the score-credit ledger — a lay-off must re-validate the target meld and attribute the card to the playing seat without altering `origin_seat`.
4. FOUNDATIONS §4: lay-off-onto-any-tableau is rummy-specific, first use, `local-only` (`ML-PP-004`); no helper promotion.
5. FOUNDATIONS §11 no-leak: a laid-off card becomes public; the effect and diagnostics must name only the played (now-public) card, never the rest of the playing seat's hand.

## Architecture Check

1. Reusing the GAT19MELLEDFIV-007 ownership split lets lay-offs credit the playing seat purely by appending a `TableCard` with the right `played_by`, no meld-ownership rewrite.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `ML-PP-004` is first-use local-only, not a promotion.

## Verification Layers

1. Legal lay-off onto own and opponent melds accepted; illegal (gap/wrong rank) rejected -> `cargo test -p meldfall_ledger` lay-off tests.
2. Score credit attributed to the playing seat -> golden trace `layoff-onto-opponent-tableau-score-credit.trace.json` asserting `played_by`.
3. Invalid lay-off emits a viewer-safe diagnostic -> rule test + `invalid-layoff-gap-or-wrong-rank.trace.json`.

## What to Change

### 1. `rules.rs` — lay-off legality

Lay-off onto any public `MeldGroup` (own or opponent), re-validating the resulting meld; attribute the card's `played_by` to the playing seat; viewer-safe invalid-lay-off diagnostics.

### 2. `effects.rs` — lay-off effect

Public lay-off effect group naming the played card, target meld, and score-credit seat.

### 3. Lay-off golden traces + ledger entry

`layoff-onto-own-tableau.trace.json`, `layoff-onto-opponent-tableau-score-credit.trace.json`, `invalid-layoff-gap-or-wrong-rank.trace.json`; record `ML-PP-004` (`local-only`).

## Files to Touch

- `games/meldfall_ledger/src/rules.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/effects.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/rules.rs` (modify — lay-off cases)
- `games/meldfall_ledger/tests/golden_traces/layoff-onto-own-tableau.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/layoff-onto-opponent-tableau-score-credit.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/invalid-layoff-gap-or-wrong-rank.trace.json` (new)

## Out of Scope

- Draw/discard pickup (GAT19MELLEDFIV-009), scoring totals (GAT19MELLEDFIV-011).
- Moving/rearranging already-tabled melds (spec out-of-scope) — only legal extension is allowed.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` lay-off tests: legal lay-off onto own and opponent melds accepted; gap/wrong-rank rejected with diagnostic.
2. Score credit goes to the playing seat (`played_by`), not the meld's `origin_seat`.
3. `cargo test --workspace` passes.

### Invariants

1. A laid-off card is public; the effect/diagnostic names only public cards (FOUNDATIONS §11).
2. Lay-off legality is Rust-owned (FOUNDATIONS §2); first-use stays local (FOUNDATIONS §4).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — lay-off legality + score-credit attribution.
2. `games/meldfall_ledger/tests/golden_traces/layoff-*.trace.json` + `invalid-layoff-*.trace.json` — accept/reject + credit traces.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. Cumulative score impact is verified in GAT19MELLEDFIV-011; this ticket verifies per-card credit attribution.

## Outcome

Completed: 2026-06-26

- Added Rust-owned `lay_off_card` legality for extending any public meld owned by any seat, with target lookup, active-hand ownership validation, duplicate prevention, resulting-meld revalidation, and ordered run extension checks for prepend/append.
- Preserved `origin_seat` while assigning the laid-off `TableCard`'s `played_by` and `score_credit_owner` to the laying-off seat.
- Updated round-played score hooks for the laying-off seat and returned a public `MeldfallEffect::LayOff` naming only the now-public laid-off card, target meld, score-credit owner, and position.
- Added viewer-safe invalid lay-off diagnostics for gaps, wrong-rank extensions, unknown target melds, and unowned cards; invalid lay-offs leave round state unchanged.
- Added rule tests for own lay-off, opponent-meld lay-off with score credit, and invalid gap/wrong-rank cases.
- Added lay-off golden traces for own tableau, opponent tableau score credit, and invalid gap/wrong-rank rejection, recording `ML-PP-004` as first official use, `local-only`.

Verification:

- `cargo fmt --all --check`
- `cargo test -p meldfall_ledger`
- `cargo test --workspace`
