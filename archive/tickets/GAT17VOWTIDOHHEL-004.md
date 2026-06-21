# GAT17VOWTIDOHHEL-004: Back-port `briar_circuit` to the promoted helper

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `games/briar_circuit/src/rules.rs`, `games/briar_circuit/Cargo.toml`, `games/briar_circuit/docs/{PRIMITIVE-PRESSURE-LEDGER,MECHANICS}.md`
**Deps**: 002

## Problem

`briar_circuit` (Hearts) is the second close trick-taking use. FOUNDATIONS §4/§11 require it to adopt the promoted `game-stdlib::trick_taking` helper in-gate with no debt. This ticket migrates its local led-suit selection and winner comparison to the helper (trumpless), while preserving every Hearts policy (forced 2♣ opening, hearts-broken lead restriction, first-trick point restriction) and all observable behavior.

## Assumption Reassessment (2026-06-21)

1. `games/briar_circuit/src/rules.rs` computes follow-suit legality in `legal_cards_for_playing_state(hand, play)` (`:238`, filter led suit else all at `:251`) and the winner in `trick_winner(plays)` (`:288`, led-suit max-by-rank, no trump). Hearts restrictions live alongside: forced 2♣ (`:244`), hearts-broken (`:116`), first-trick points (`:112`).
2. `games/briar_circuit/Cargo.toml` depends only on `ai-core`, `engine-core` — `game-stdlib` must be added.
3. Cross-crate boundary under audit: only the pure "matching led suit or all" core and the pure winner-index computation move to the helper; the Hearts opening/point/hearts-broken restrictions that further filter the result stay local. Golden traces + state/action/effect hashes are the preservation contract.
4. FOUNDATIONS §11 (promoted-primitive adoption + deterministic replay/hash) is the principle under audit; this is behavior-preserving conformance — change rationale is the §4 in-gate adoption mandate.
5. Replay/hash enforcement surface: capture pre-change baselines and compare byte-for-byte; the helper must run *inside* (not replace) the local first-trick/hearts filters so Hearts legality and effect ordering are unchanged. No hidden-info path changes.

## Architecture Check

1. Calling the helper for the pure subset and keeping the Hearts policy local is the minimal conformance — it removes duplication without entangling the helper with any restriction.
2. No shims; the local pure computations are replaced in place.
3. `engine-core` untouched; the adopted helper is the atlas-earned promotion from 002.

## Verification Layers

1. Hearts legality (2♣/hearts-broken/first-trick) + winner unchanged → `cargo test -p briar_circuit` (rules/property suites).
2. Deterministic replay/hash preserved → `cargo run -p replay-check -- --game briar_circuit --all`.
3. No-leak preserved → `cargo test -p briar_circuit --test visibility`.
4. Adoption recorded → grep `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` for the Gate 17 conformance receipt.

## What to Change

### 1. Adopt the helper in `rules.rs`

Add `game-stdlib` to `Cargo.toml`. Inside `legal_cards_for_playing_state`, compute the base led-suit subset via `follow_suit_indices` (projecting `CardId::card().suit`) before applying the local first-trick/hearts filters. Replace the `trick_winner` led-suit max with `winning_play_index` (`trump = None`), mapping the index back to the local `TrickPlay`.

### 2. Ledger + mechanics receipt

Record the Gate 17 decision/back-port receipt in `PRIMITIVE-PRESSURE-LEDGER.md` and `MECHANICS.md`.

## Files to Touch

- `games/briar_circuit/src/rules.rs` (modify)
- `games/briar_circuit/Cargo.toml` (modify)
- `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)
- `games/briar_circuit/docs/MECHANICS.md` (modify)

## Out of Scope

- Any change to Hearts rules, action order, diagnostics, effects, visibility, bots, or UI beyond the behavior-preserving swap.
- `plain_tricks` back-port (003); Vow Tide code.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` — all pre-existing tests unchanged-green.
2. `cargo run -p replay-check -- --game briar_circuit --all` — traces/hashes byte-identical.
3. `cargo bench -p briar_circuit` — no material regression vs the 002 baseline.

### Invariants

1. No observable behavior, hash, trace, or Hearts-policy change.
2. No promotion debt opened; the ledger records adoption.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/rules.rs` / `property.rs` / `visibility.rs` — existing tests must pass unmodified; regression note only if a latent defect is proved.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. Narrower command rationale: replay-check `--all` plus the visibility suite is the determinism + no-leak boundary proving the swap changed nothing.

## Outcome

Completed: 2026-06-21

What changed:

- Added `game-stdlib` as a `briar_circuit` dependency.
- Replaced Briar Circuit's base led-suit subset in `legal_cards_for_playing_state` with `game_stdlib::trick_taking::follow_suit_indices`, then kept the existing 2 clubs, first-trick point, and hearts-broken filters local.
- Replaced the trumpless led-suit winner comparison in `trick_winner` with `game_stdlib::trick_taking::winning_play_index(..., trump = None, ...)`, preserving local winner-to-seat mapping and winner-leads state mutation.
- Updated `games/briar_circuit/docs/MECHANICS.md` and `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` with the Gate 17 helper-conformance receipt and explicit Hearts-policy non-goals.

Deviations from plan:

- None. No Hearts rule, action path, diagnostic, effect, visibility, bot, UI, trace, scoring, or terminal behavior was intentionally changed.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p briar_circuit` passed: 1 library test, 6 bot tests, 12 property tests, 9 replay tests, 21 rules tests, 4 serialization tests, 8 visibility tests, and doc tests.
- `cargo run -p replay-check -- --game briar_circuit --all` passed; every Briar Circuit golden trace was accepted and the command ended with `replay-check: all traces passed`.
- `cargo bench -p briar_circuit` passed; all 21 reported operations exceeded thresholds, including `full_seeded_match_terminal` at `41553.09 matches_per_second` against the `100.00` floor in this local run.
