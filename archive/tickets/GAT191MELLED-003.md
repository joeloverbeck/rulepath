# GAT191MELLED-003: `NextRoundDealt` effect + `round_score_index` correction

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger` (`src/effects.rs`); `crates/wasm-api` (`src/games/meldfall.rs`)
**Deps**: GAT191MELLED-001

## Problem

The multi-round transition needs a public effect to announce that a fresh round
was dealt, and the existing `round_score` effect currently reports the wrong round
index. `round_score_index` (`crates/wasm-api/src/games/meldfall.rs:230`) is
hardcoded to `0` with an "intentionally deferred" comment, and there is no
round-transition effect at all. This ticket adds the `NextRoundDealt` effect
(definition + both encodings + viewer projection) and corrects `round_score_index`
to report the true settled-round count from `MatchState` (GAT191MELLED-001).

## Assumption Reassessment (2026-06-26)

1. `MeldfallEffect` is defined in `games/meldfall_ledger/src/effects.rs:16`, and
   `effect_stable_string` (effects.rs:105) is an **exhaustive** match over it
   (arms for `Draw`, `StockDrawPrivate`, `Meld`, `LayOff`, `Discard`, `RoundScore`,
   `MatchTerminal`). A new variant requires a new arm here, which the compiler
   enforces.
2. The wasm bridge imports the enum (`crates/wasm-api/src/games/meldfall.rs:12`)
   and owns (a) the JSON encoder — an exhaustive match emitting `{"kind":"..."}`
   strings (meldfall.rs ~470-501) — and (b) `round_score_index` (meldfall.rs:230-236),
   hardcoded `0` with the deferred comment. Both are this ticket's edit surface;
   the comment is removed/superseded per spec §10.
3. Cross-artifact boundary under audit: the effect-envelope contract. The web
   `describeEffect` switch keys on `payload.type ?? payload.kind`
   (`apps/web/src/components/effectFeedback.ts:16`); meldfall effects carry `kind`
   (no `type`). The web consumer case is authored in GAT191MELLED-005 — out of
   scope here, but the kind string chosen here is its contract.
4. FOUNDATIONS §11 no-leak firewall restated: the new effect is public and carries
   only counts/seat labels (`next_round_number`, `next_lead_seat`, the new dealer
   seat) — never stock order or opponent hands. `EffectEnvelope::public` is the
   correct visibility scope (matching the existing `RoundScore`/`MatchTerminal`
   public effects at meldfall.rs:214-223).
5. Schema/contract extension: the extended structure is `MeldfallEffect` and its
   two encodings (stable string in `effects.rs`, JSON in the wasm bridge). The
   extension is additive (a new variant). Rust enforces the two exhaustive-match
   consumers; the third consumer — the web `describeEffect` switch — is updated in
   GAT191MELLED-005. `high_card_duel` already owns the `refill_started` kind in
   that shared switch, so this effect MUST use the distinct kind `next_round_dealt`
   (spec §3.1.5).

## Architecture Check

1. A distinct meldfall-owned effect kind `next_round_dealt` — mirroring only the
   *field shape* of other games' `refill_started`, not the kind string — keeps the
   shared web `describeEffect` switch unambiguous and avoids colliding with
   `high_card_duel`'s case. Reusing `refill_started` would route meldfall through
   another game's presentation.
2. No backwards-compatibility shim: `round_score_index` is corrected in place to
   return the true settled count, and the stale deferred comment is removed — no
   alias or transitional path.
3. `engine-core` is untouched; the effect lives in the game crate + wasm bridge;
   no `game-stdlib` change.

## Verification Layers

1. `NextRoundDealt` variant present with both exhaustive encodings -> codebase grep-proof + `cargo build` (exhaustive matches compile).
2. Kind string is `next_round_dealt`, distinct from `refill_started` -> grep-proof that no meldfall encoder emits `refill_started`.
3. `round_score_index` returns the settled count (0 for round 0) -> in-module unit test in the wasm bridge.
4. Effect is viewer-safe (public counts/seat labels only) -> stable-string smoke asserting no card/hand/stock field is encoded.

## What to Change

### 1. Add the `NextRoundDealt` variant (`effects.rs`)

Add `MeldfallEffect::NextRoundDealt { next_round_number, next_lead_seat,
new_dealer }` (seat fields as the crate's seat type), its `effect_stable_string`
arm, and an in-module `#[cfg(test)]` smoke asserting the stable string carries only
public fields.

### 2. JSON encoding + viewer projection + `round_score_index` (`meldfall.rs`)

Add the JSON encoder arm emitting `{"kind":"next_round_dealt","next_round_number":…,
"next_lead_seat":…,"new_dealer":…}` and the public viewer projection. Change
`round_score_index` to return the `MatchState` rounds-settled counter
(GAT191MELLED-001) instead of `0`, and remove/supersede the "intentionally
deferred" comment. Add an in-module unit test that `round_score_index` reports 0
before the first settlement and the settled count thereafter.

## Files to Touch

- `games/meldfall_ledger/src/effects.rs` (modify)
- `crates/wasm-api/src/games/meldfall.rs` (modify)

## Out of Scope

- Emitting the effect / wiring the apply path to call the transition — GAT191MELLED-004
  (this ticket defines the effect; it is not yet pushed).
- The web `describeEffect` `next_round_dealt` case — GAT191MELLED-005.
- The transition logic itself — GAT191MELLED-002.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger effects` — the effect stable-string smoke passes.
2. `cargo test -p wasm-api` — the `round_score_index` unit test passes.
3. `cargo build --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` — exhaustive matches compile cleanly.

### Invariants

1. `MeldfallEffect::NextRoundDealt` uses the kind `next_round_dealt` (never
   `refill_started`) and is encoded as a public effect carrying only counts/seat labels.
2. `round_score_index` returns the true number of rounds settled before the current
   settlement (0 for the first round), derived from `MatchState`, not the settling seat.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/src/effects.rs` — in-module smoke asserting the
   `NextRoundDealt` stable string carries only public count/seat fields.
2. `crates/wasm-api/src/games/meldfall.rs` — in-module unit test that
   `round_score_index` returns 0 for round 0 and the settled count afterward.

### Commands

1. `cargo test -p meldfall_ledger effects && cargo test -p wasm-api`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. The two crate-scoped test filters are the correct boundary (effect definition in
   the game crate, `round_score_index` in the bridge); end-to-end emission is
   verified in GAT191MELLED-004.

## Outcome

Completed: 2026-06-26

Added the public `MeldfallEffect::NextRoundDealt` variant with stable-string and
WASM JSON encodings using the distinct `next_round_dealt` kind. The payload
contains only the next round number, new dealer seat, and next lead seat. Updated
`round_score_index` to read `MatchState::rounds_settled` and removed the deferred
single-round hardcode.

Deviations: the initial in-module effect test was placed before a helper and
triggered clippy's `items_after_test_module` lint; it was moved to the end of
`effects.rs` and the affected checks were rerun successfully.

Verification:

- `cargo test -p meldfall_ledger effects`
- `cargo test -p wasm-api meldfall_round_score_index_is_the_round_not_the_finishing_seat`
- `cargo test -p wasm-api`
- `cargo fmt --all --check`
- `cargo build --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `rg -n 'refill_started' games/meldfall_ledger crates/wasm-api/src/games/meldfall.rs`
  found no Meldfall encoder use of that kind
- `rg -n 'next_round_dealt' games/meldfall_ledger crates/wasm-api/src/games/meldfall.rs`
  confirmed the new bridge kind
