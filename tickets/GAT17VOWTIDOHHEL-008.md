# GAT17VOWTIDOHHEL-008: Trick-play legality and resolution via the promoted helper

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `games/vow_tide/src/{actions,rules,effects,state}.rs`; new play golden traces
**Deps**: 002, 006, 007

## Problem

Implement trick play: any legal lead (including trump), forced follow-suit when able, void may play any, highest-trump-else-highest-led winner, and winner-leads-next. Follow-suit selection and the winning-index comparison route through `game-stdlib::trick_taking`; all phase/seat/effect policy stays local.

## Assumption Reassessment (2026-06-21)

1. `game-stdlib::trick_taking::{follow_suit_indices, winning_play_index}` (ticket 002) provide the pure selection/comparison; `games/vow_tide/src/state.rs` (006/007) owns the current trick, led suit, trump, and active player. The bidding action family (007) precedes play in-phase.
2. Spec §3.1 + Appendix B.1.1 fix the transition order (validate→remove→append→resolve via comparator→record+increment→clear→set next leader→bump freshness→emit effects) and `VT-FOLLOW/TRICK-WIN/NEXT-LEAD-001`.
3. Cross-crate boundary under audit: Vow Tide projects local `Card` suit/rank into the helper and maps the returned stable index back to its own seat/`CardId`; the helper must not reorder action leaves or effect output.
4. FOUNDATIONS §2/§4 under audit: Rust (via the earned helper) owns winner/legality; TypeScript decides nothing; the helper carries no Vow Tide policy (trump presence is a caller-passed `Option`).
5. §11 enforcement surface: play legality is fail-closed (`VT_CARD_NOT_OWNED`/`VT_MUST_FOLLOW_SUIT`/`VT_WRONG_PHASE`/`VT_STALE_COMMAND`); the comparator is deterministic (stable first-occurrence tie); no hidden hand/stock enters play metadata.

## Architecture Check

1. Using the promoted comparator with a caller-supplied `trump: Option<Suit>` and keeping winner-leads/capture local is the exact §8.2 boundary — it reuses the pure core without leaking Vow Tide policy into `game-stdlib`.
2. No shims; extends the 007 action/rules modules.
3. `engine-core` untouched; the helper is the atlas-earned promotion.

## Verification Layers

1. Lead-any / forced-follow / void-any → `cargo test -p vow_tide --test rules` + traces.
2. Winner = highest trump else highest led; off-suit never wins → property tests + `winning_play_index` conformance.
3. Winner leads next; sum of trick counts = `H` → property tests.
4. Legal-tree ↔ validator equivalence for plays → property test.

## What to Change

### 1. Play action family + legality

`actions.rs`: `play/<card_id>` leaves = legal owned cards in canonical hand order, using `follow_suit_indices` for the led-suit subset. `rules.rs`: ownership/follow/stale validation independent of leaf membership.

### 2. Trick resolution + winner-leads

On each play apply the Appendix B.1.1 order: resolve the winner via `winning_play_index` (passing the hand's trump), map index→seat, capture the public trick, increment the winner's count, set the winner as next leader, emit deterministic play/trick-captured effects.

## Files to Touch

- `games/vow_tide/src/actions.rs` (modify)
- `games/vow_tide/src/rules.rs` (modify)
- `games/vow_tide/src/effects.rs` (modify)
- `games/vow_tide/src/state.rs` (modify)
- `games/vow_tide/tests/golden_traces/trump-may-lead.trace.json` (new)
- `games/vow_tide/tests/golden_traces/follow-suit-forced.trace.json` (new)
- `games/vow_tide/tests/golden_traces/highest-trump-wins.trace.json` (new)

## Out of Scope

- Hand scoring / schedule advance / terminal (009); visibility/effect filtering (010).
- Any trump/seat/scoring flag added to the `game-stdlib` helper; any TypeScript legality.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test rules` — lead/follow/void/winner paths.
2. `cargo test -p vow_tide --test property` — winner maximal-in-class, winner-leads, trick-count sum = `H`.
3. `cargo clippy -p vow_tide --all-targets -- -D warnings`.

### Invariants

1. A follower holding the led suit must play it; off-suit non-trumps never win.
2. Winner of each trick leads the next unless the hand ended; the comparator output equals the local rule's expectation.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/rules.rs` / `property.rs` — play legality + resolution.
2. `games/vow_tide/tests/golden_traces/{void-may-play-any,highest-led-wins-without-trump,off-suit-nontrump-never-wins,trick-winner-leads-next,one-card-hand-completes}.trace.json`.

### Commands

1. `cargo test -p vow_tide --test rules --test property`
2. `cargo test -p vow_tide`
3. Narrower command rationale: rules+property are the legality/resolution boundary; cross-game helper conformance is in 002/003/004.
