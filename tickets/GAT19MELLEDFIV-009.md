# GAT19MELLEDFIV-009: Draw/discard zones and multi-card discard-pile pickup with immediate-use commitment (first-use primitive)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/meldfall_ledger/src/{actions,rules,effects}.rs`; draw/pickup golden traces; first-use ledger entry ML-PP-003
**Deps**: GAT19MELLEDFIV-005, GAT19MELLEDFIV-006, GAT19MELLEDFIV-007, GAT19MELLEDFIV-008

## Problem

This is the signature mechanic: a seat draws from the stock OR takes a card from the public ordered discard pile plus all newer cards above it. The chosen deepest card carries an **immediate-use commitment** — it must be used in a new meld or lay-off during the same turn (Rulepath applies this strictly to the top discard too). A committed card cannot be discarded instead. First official use of draw/discard zones with multi-card pickup (`ML-PP-003`, `local-only`).

## Assumption Reassessment (2026-06-25)

1. Existing card games have decks/hands/community cards but no Rummy-500 discard-tail pickup with immediate-use commitment (confirmed during reassessment). `actions.rs` (`DrawFromStock`/`DrawFromDiscard{index}`), `RoundState.pending_pickup`, meld (GAT19MELLEDFIV-006), tableau (007), and lay-off (008) exist.
2. Spec §3.1 (Discard-pile pickup row), Appendix A.2 (Discard-pile draw + Discard-after-pickup rows), Appendix B.2 (action model `pending_pickup.required_card`), and Appendix D (`ML-PP-003`) define behavior; external prior art (timpalpant/rummy "must play picked discard") corroborates the commitment.
3. Cross-artifact: the action-tree (`actions.rs`) and the `pending_pickup` state field are the shared boundary; the Rust legal action tree must expose only valid discard indices, and apply must enforce the commitment before allowing turn-finish.
4. FOUNDATIONS §2 behavior authority: pickup legality, the immediate-use commitment, and the "cannot discard the committed card" rule are all Rust-decided; TypeScript never filters pickup choices.
5. FOUNDATIONS §11 no-leak: the discard pile is public (drawn cards public before pickup); the stock is hidden (stock draw shows the drawn card only to the acting seat). The action tree/effects must keep stock order hidden while the discard tail is public — leak enforcement harness is GAT19MELLEDFIV-013, but this ticket must not surface stock order in the action tree or effects. First-use primitive recorded `ML-PP-003`.

## Architecture Check

1. Modeling the commitment as `pending_pickup` state checked at meld/lay-off and at turn-finish keeps the rule in one place, validated by Rust, with the action tree narrowing legal continuations — no TS legality.
2. No backwards-compatibility shims.
3. `engine-core` untouched; draw/discard/pile nouns crate-local; `ML-PP-003` first-use local-only, no promotion.

## Verification Layers

1. Stock draw and discard-index draw legal; multi-card pickup takes the selected card + all newer -> `cargo test -p meldfall_ledger` + `multi-card-discard-pickup-melds-deepest.trace.json`.
2. Immediate-use commitment enforced (deep + top discard); committed card cannot be discarded -> `invalid-discard-pickup-without-use.trace.json`, `top-discard-pickup-also-requires-use.trace.json`.
3. Stock order stays hidden; only acting seat sees the stock-drawn card -> `deterministic-stock-draw-no-leak.trace.json` (full matrix in GAT19MELLEDFIV-013).

## What to Change

### 1. `actions.rs` — draw + pickup action tree

`DrawFromStock` / `DrawFromDiscard{selected_discard_index}`; the legal action tree exposes only valid discard indices and, after a pickup, only continuations that satisfy the commitment; `pending_pickup.required_card` set on discard pickup.

### 2. `rules.rs` — pickup legality + commitment

"Take selected card plus all newer cards" semantics; immediate-use commitment (deep and top discard) enforced at meld/lay-off and at turn-finish; block discarding the committed card; viewer-safe diagnostics.

### 3. `effects.rs` + draw/pickup golden traces + ledger entry

Public/seat-private effect groups for stock draw (public count change + acting-seat-only drawn card) and discard pickup (public). Traces: `draw-source-choice-stock-vs-discard`, `multi-card-discard-pickup-melds-deepest`, `invalid-discard-pickup-without-use`, `top-discard-pickup-also-requires-use`, `deterministic-stock-draw-no-leak`. Record `ML-PP-003` (`local-only`).

## Files to Touch

- `games/meldfall_ledger/src/actions.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/rules.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/effects.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/rules.rs` (modify — draw/pickup/commitment cases)
- `games/meldfall_ledger/tests/golden_traces/draw-source-choice-stock-vs-discard.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/multi-card-discard-pickup-melds-deepest.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/invalid-discard-pickup-without-use.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/top-discard-pickup-also-requires-use.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/deterministic-stock-draw-no-leak.trace.json` (new)

## Out of Scope

- Going out / stock exhaustion (GAT19MELLEDFIV-010), scoring (GAT19MELLEDFIV-011).
- Frozen-pile / reshuffle variants (spec out-of-scope).
- The full pairwise no-leak matrix (GAT19MELLEDFIV-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger`: stock + discard-index draw legal; multi-card pickup takes selected + newer; deep selection rejected if the committed card is not used immediately; top discard also requires immediate use.
2. The committed card cannot be discarded; turn-finish blocked while the commitment is unsatisfied.
3. `cargo test --workspace` passes.

### Invariants

1. Pickup legality and the immediate-use commitment are Rust-decided (FOUNDATIONS §2); no TS filtering.
2. Stock order stays hidden in the action tree and effects (FOUNDATIONS §11); the discard tail is public.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — draw-source, multi-card pickup, commitment (deep + top), no-discard-of-committed.
2. `games/meldfall_ledger/tests/golden_traces/{draw-source-choice,multi-card-discard-pickup,invalid-discard-pickup,top-discard-pickup,deterministic-stock-draw}-*.trace.json`.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. The full multi-viewer stock-order no-leak matrix is GAT19MELLEDFIV-013; this ticket asserts the single-trace no-leak + commitment legality.
