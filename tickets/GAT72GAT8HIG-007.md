# GAT72GAT8HIG-007: Round transitions — commit / reveal / score / refill / terminal

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/rules.rs` (apply/transition path), emitting effects from `src/effects.rs`
**Deps**: GAT72GAT8HIG-005, GAT72GAT8HIG-006

## Problem

Gate 8 needs the Rust-owned state transitions that apply a validated commit,
reveal both committed cards simultaneously, score the round (higher rank wins;
tie scores none), move revealed cards to public history, refill hands from the
internal deck, alternate the lead seat, advance rounds, and reach terminal after
round six — emitting the correct public/private effects at each step.

## Assumption Reassessment (2026-06-07)

1. Verified the inputs exist after prior tickets: state/setup (`state.rs`,
   `setup.rs` from 004), legality/validation (`actions.rs`/`rules.rs` from 006),
   and effect families (`effects.rs` from 005). Sibling `apply_action` in
   `games/draughts_lite::apply_action` is the transition-shape precedent.
2. Verified against the spec: §4.2.2 `HCD-ROUND-001..013` fixes lead/reply roles,
   face-down commit, simultaneous reveal, higher-rank scoring, tie handling,
   revealed→history, refill-to-three starting with next lead seat, lead
   alternation (odd rounds `seat_0`, even `seat_1`), six-round terminal, and
   winner/draw policy.
3. Cross-artifact boundary under audit: the state-transition + effect-emission
   contract — transitions are the sole writer of score/reveal/history and the
   sole emitter of `hcd_cards_revealed` (the first public surfacing of committed
   cards).
4. FOUNDATIONS principle under audit (§2 behavior authority): all transition
   logic (reveal timing, scoring, refill, terminal) is Rust; TypeScript never
   recomputes it.
5. Enforcement surface named: deterministic replay/hash (§11/§13) and the no-leak
   firewall (§11). Confirm reveal is simultaneous (reply never sees the lead card
   before `hcd_cards_revealed`), the unrevealed deck tail is never surfaced by a
   transition, and the transition sequence is deterministic so the internal trace
   replays identically (proven in 009/012).

## Architecture Check

1. Centralizing all writes (reveal/score/refill/terminal) behind the apply path
   keeps reveal-timing and conservation single-sourced — cleaner than scattering
   mutations across modules.
2. No backwards-compatibility shims — new game-local transitions.
3. `engine-core` untouched; no mechanic noun in the kernel; no `game-stdlib`
   change (cards stay local, §4).

## Verification Layers

1. Simultaneous reveal -> golden trace / no-leak test: reply commit precedes reveal; `hcd_cards_revealed` is the first public exposure of both cards.
2. Scoring + tie -> unit test: higher rank scores one; tie scores none; score increments only from reveal/compare.
3. Refill + lead alternation + terminal -> unit test: refill to three from the next lead seat; odd/even lead seats; terminal after round six with winner/draw policy.
4. Card conservation -> property test (extended in 011): no duplicate/lost card across deck/hands/commit/revealed zones per transition.

## What to Change

### 1. `rules.rs` apply/transition path

Implement `HCD-ROUND-001..013`: apply validated commit (remove from own hand,
emit `hcd_commit_face_down` Public + `hcd_own_commit_confirmed` private);
simultaneous reveal (`hcd_cards_revealed`); score (`hcd_round_scored`, tie =
no point); revealed→public history; refill (`hcd_hand_count_changed`,
`hcd_refill_started`) starting with the next lead seat; lead alternation; round
advance; terminal after six rounds (`hcd_terminal`) with higher-score-wins /
equal-draw.

## Files to Touch

- `games/high_card_duel/src/rules.rs` (modify — transition path)

## Out of Scope

- View projection (008), replay export (009), bots (010).
- Legality/validation generation (already in 006; this ticket consumes it).

## Acceptance Criteria

### Tests That Must Pass

1. `lead_commit_removes_card_from_own_hand`, `reply_commit_cannot_see_lead_identity`, `both_commitments_reveal_together`.
2. `higher_rank_scores_one_point`, `tie_round_scores_no_points`.
3. `refill_restores_hand_size_when_deck_available`, `lead_alternates_by_round`, `terminal_after_six_rounds`, `terminal_winner_and_draw_policy`.

### Invariants

1. Reveal is simultaneous; the reply seat cannot learn the lead card before `hcd_cards_revealed` (§11).
2. Score changes only from reveal/compare; round count is monotonic; terminal has no gameplay actions.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/rules.rs` — transition/scoring/refill/terminal cases.
2. `games/high_card_duel/tests/property.rs` — card-conservation invariant (seeded; extended by 011).

### Commands

1. `cargo test -p high_card_duel --test rules`
2. `cargo test -p high_card_duel`
3. Rule + property tests are the correct boundary; full multi-seed conservation and replay reproduction land in 011/012.
