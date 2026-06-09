# GAT10POKLITBET-006: Viewer-scoped semantic effects

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/poker_lite/src/effects.rs`. Consumes `engine-core` `EffectEnvelope`/`VisibilityScope`. No kernel change.
**Deps**: GAT10POKLITBET-005

## Problem

The browser animates from Rust-emitted semantic effects, and each effect must carry the correct visibility scope so hidden information never reaches a viewer who shouldn't see it. `poker_lite` needs typed effects for the deal, pledge actions, center reveal, grouped showdown reveal, ledger resolution, and terminal — with private deals scoped to their owning seat and the showdown reveal emitted as one grouped public effect.

## Assumption Reassessment (2026-06-08)

1. The effect pattern matches `games/secret_draft/src/effects.rs` / `games/high_card_duel/src/effects.rs`: a typed effect enum wrapped in `engine-core` `EffectEnvelope<T>` with a `VisibilityScope`. Verified `EffectEnvelope<T>` and `VisibilityScope` (variants `Public`, `PrivateToSeat(SeatId)`) are exported from `crates/engine-core/src/lib.rs`, and `EffectLog<T>` (viewer-filtering log) from `crates/engine-core/src/replay.rs`.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §B3 Effects, §A2 step 7, §A4 reveal steps) enumerates the effect set: `private_crest_dealt` (PrivateToSeat, own card), `crest_deal_started`/`opening_pool_set`/`pledge_held`/`pledge_pressed`/`pledge_lifted`/`pledge_matched`/`seat_yielded` (public, counts/amounts only), `center_reveal_started`+`center_revealed` (public grouped), `showdown_reveal_started`+`showdown_revealed` (public grouped, both private cards + center), `ledger_resolved`, `terminal`, and `bot_chose_action` (public: policy id + action family only; private-to-actor may add own strength bucket, never opponent card).
3. Cross-artifact boundary under audit: `engine-core`'s `EffectEnvelope`/`VisibilityScope`/`EffectLog` contract and the `TerminalOutcome`/transition outputs from `rules.rs` (GAT10POKLITBET-005). Effects are produced by transitions and consumed by `replay_support.rs` (009) and the web renderer (015) — the envelope scope is the no-leak contract between them.
4. FOUNDATIONS §11 (semantic effects drive animation; hidden info does not leak via effect logs) and §7 (animation driven by Rust effects) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak firewall surface under audit (§11/§12): `private_crest_dealt` MUST be `PrivateToSeat` carrying only the owning seat's card; public deal/pledge effects carry counts/amounts only (no card id/rank); the showdown reveal MUST be a single grouped `showdown_revealed` effect (no per-card partial reveal that leaks order-dependent private state); `seat_yielded` carries no private card; `bot_chose_action` public payload never carries a private card. This is a primary effect-log no-leak surface (string-search tests in GAT10POKLITBET-007/008).

## Architecture Check

1. Attaching `VisibilityScope` at emission time (rather than redacting a flat log per viewer downstream) makes viewer-safety a property of each effect, so the `EffectLog` filter cannot accidentally surface a hidden card. Matches the sibling effect model.
2. No backwards-compatibility aliasing/shims — new module.
3. `engine-core` stays noun-free (crest/pledge effects are crate-local typed effects over the generic envelope, §3); no `game-stdlib` promotion (§4).

## Verification Layers

1. Visibility-scope correctness (private deal scoped to owner; public effects carry no card id) -> `cargo test -p poker_lite` effect-scope unit tests.
2. Grouped reveal (showdown is one `showdown_revealed` effect, not per-card) -> effect-emission test asserting a single grouped reveal.
3. Effect→animation contract (effects carry semantic payload sufficient for the renderer) -> schema review against the sibling renderer's consumption (deferred UI assertion to GAT10POKLITBET-015).
4. No-leak over effect JSON -> filtered-log assertion (full string-search sweep in GAT10POKLITBET-007/008).

## What to Change

### 1. `games/poker_lite/src/effects.rs`

Define the typed effect enum and emission helpers per §B3, each wrapped with the correct `VisibilityScope`. Private deal → `PrivateToSeat`; public pledge/deal/ledger/terminal → `Public` with counts/amounts only; center and showdown reveals as grouped public effects; `bot_chose_action` with a public (policy id + family) variant and an optional private-to-actor strength-bucket variant. Wire emission into the `rules.rs` transitions.

## Files to Touch

- `games/poker_lite/src/effects.rs` (new)
- `games/poker_lite/src/rules.rs` (modify — emit effects from transitions)
- `games/poker_lite/src/lib.rs` (modify — add `mod effects;` + re-exports)

## Out of Scope

- Public/private view projection and the exhaustive no-leak string-search sweep (GAT10POKLITBET-007).
- Replay export/import of effects (GAT10POKLITBET-009).
- Bot explanation content beyond the effect-shape contract (GAT10POKLITBET-010/011).
- Web effect-feedback copy (GAT10POKLITBET-015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` effect-scope tests: `private_crest_dealt` is `PrivateToSeat`; public effects carry no card id/rank.
2. Grouped-reveal test: showdown emits exactly one `showdown_revealed` effect carrying both private cards + center.
3. `cargo test -p poker_lite --test rules` still passes after effects are wired into transitions.

### Invariants

1. Every hidden-card-bearing effect is `PrivateToSeat` (deal) or gated behind its rule-defined reveal point (center/showdown) (§11).
2. Animation-driving payload is Rust-emitted; the renderer never infers state from diffs (§7/§11).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/effects.rs` (inline `#[cfg(test)]`) — per-effect visibility scope + grouped-reveal shape.

### Commands

1. `cargo test -p poker_lite`
2. `cargo test -p poker_lite --test rules`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-09

Changed:

- Added `games/poker_lite/src/effects.rs` with typed semantic effects and visibility-scoped helpers for private deal, public deal/pool setup, pledge actions, yield, center reveal, grouped showdown reveal, ledger resolution, terminal, and bot-choice effect shapes.
- Wired `games/poker_lite/src/rules.rs` transitions to return `Vec<EffectEnvelope<PokerLiteEffect>>`.
- Ensured private setup deal effects are `PrivateToSeat`, public setup/pledge effects carry counts and public accounting only, and showdown reveal is emitted as a single grouped `ShowdownRevealed` payload.
- Re-exported effect helpers and `PokerLiteEffect` from `games/poker_lite/src/lib.rs`.
- Extended rules integration coverage to assert transition-emitted showdown effects contain exactly one grouped `ShowdownRevealed` payload.

Deviations from original plan:

- Setup effects are exposed through `setup_effects(&state)` rather than changing `setup_match`'s return type; this keeps the established setup signature intact while providing the Rust-emitted effect stream for callers that need it.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed: 20 unit tests, 10 integration tests, and 0 doc tests.
- `cargo test -p poker_lite --test rules` passed: 10 integration tests.
- `bash scripts/boundary-check.sh` passed.
