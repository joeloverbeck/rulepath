# GAT18BLAPACSPA-006: trick play with promoted-helper reuse and broken-spades policy

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/blackglass_pact` (rules/state/effects), reuses `game-stdlib::trick_taking` unchanged + golden traces
**Deps**: GAT18BLAPACSPA-005

## Problem

Implement the `PlayingTrick` phase: first leader left of dealer; legal-lead / follow-suit / broken-spades policy; spades-always-trump trick resolution via the promoted comparator with caller-projected `Some(Suit::Spades)`; winner-leads-next. The two promoted `game-stdlib::trick_taking` helpers are reused **unchanged** (no third-use gate fires) with all partnership/broken-spades/orchestration kept local (spec §2.3, §3.1 play rows, §8.2, `BP-PLAY-*`, candidate task `GAT18-BLAPAC-006`).

## Assumption Reassessment (2026-06-25)

1. Actual promoted signatures (verified `crates/game-stdlib/src/trick_taking.rs:22,49`): `follow_suit_indices<T, S: Copy + Eq>(held, led_suit, suit_of) -> Vec<usize>` and `winning_play_index<T, S: Copy + Eq, R: Copy + Ord>(plays, led_suit, trump, suit_of, rank_of) -> Option<usize>`. Spades-as-always-trump fits via `trump = Some(Suit::Spades)` with no helper change (spec Assumption 14).
2. Spec §3.1 pins follow-suit, the broken-spades lead restriction (with all-spades exception), and off-suit-spade-breaks; spec §8.2 pins the integration boundary.
3. Cross-crate boundary under audit: `game-stdlib::trick_taking` is consumed read-only; no partnership/broken-spades/bidding/scoring is added to the helper (FOUNDATIONS §4 / spec §9 #2).
4. FOUNDATIONS §4 (`game-stdlib` earned) motivates this ticket: this is the documented **reuse** of an already-promoted helper, not a new promotion — the conformance test proves the helper result against an independent oracle.

## Architecture Check

1. Reusing the behavior-free comparator with `Some(Spades)` (vs. duplicating trick logic locally) honors the Gate 17 promotion and keeps trump/winner selection in one tested home; game-local code owns legality and mutation.
2. No shims; helper signatures are not widened to fit a test.
3. `engine-core` untouched; `game-stdlib` is reused unchanged (no earned-promotion event).

## Verification Layers

1. `follow_suit_indices` / `winning_play_index(Some(Spades))` reused unchanged -> conformance tests vs. an independent oracle over exhaustive/strong-sampled four-play suit/rank combinations + `promoted-*-helper-conformance` traces.
2. Broken-spades lead policy (block before broken; only-spades exception; off-suit spade breaks) -> rules/tree tests + spade-lead traces.
3. Winner-leads-next; exactly four plays per trick, 13 tricks per hand -> property test + transition traces.

## What to Change

### 1. Lead and follow-suit legality

`rules.rs`: game-local lead legality (broken-spades restriction + all-spades exception), then `follow_suit_indices` for followers after ownership/active checks; `spades_broken` mutation on first legal off-suit/lead spade.

### 2. Trick resolution + transition

`rules.rs`/`state.rs`: after four plays, `winning_play_index(plays, led_suit, Some(Suit::Spades), suit_of, rank_of)`; winner leads next; advance `trick_index`.

### 3. Effects + traces

`effects.rs`: `CardPlayed`/`SpadesBroken`/`TrickCaptured` (public). Add play + helper-conformance golden traces (spec §7.6 #25–#36).

## Files to Touch

- `games/blackglass_pact/src/{rules,state,effects}.rs` (modify)
- `games/blackglass_pact/tests/{rules,property}.rs` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — play + helper conformance)

## Out of Scope

- Hand scoring / bags / terminal (GAT18BLAPACSPA-007).
- Any change to `game-stdlib::trick_taking` (forbidden — reuse only).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test rules` (lead/follow/broken-spades + diagnostics).
2. `cargo test -p blackglass_pact --test property` (helper-vs-oracle comparator agreement; 4-plays/13-tricks).
3. `cargo test -p game-stdlib` (promoted helpers unchanged and green).

### Invariants

1. The two promoted helpers are reused with unchanged signatures; spades is the caller-projected trump.
2. No partnership, broken-spades, or orchestration policy leaks into `game-stdlib`.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/rules.rs` — broken-spades + follow-suit legality and diagnostics.
2. `games/blackglass_pact/tests/property.rs` — comparator conformance vs. independent oracle.
3. `games/blackglass_pact/tests/golden_traces/promoted-winner-index-helper-conformance.trace.json` — reuse evidence.

### Commands

1. `cargo test -p blackglass_pact --test rules --test property`
2. `cargo test -p game-stdlib && cargo test -p blackglass_pact`
3. Crate + game-stdlib scope is correct; helper non-mutation is proven by game-stdlib staying green.

## Outcome

Completed: 2026-06-25

Implemented trick play and promoted-helper reuse:

- Added `play/<card-id>` parsing, legal play leaves, active-seat/ownership/follow-suit validation, and stable play diagnostics.
- Added game-local broken-spades lead policy with the all-spades lead exception and off-suit-spade break mutation.
- Reused `game_stdlib::trick_taking::follow_suit_indices` unchanged for follower legality and `winning_play_index(..., Some(Suit::Spades), ...)` unchanged for trick winners.
- Added public `CardPlayed`, `SpadesBroken`, and `TrickCaptured` effects.
- Added trick mutation: remove played cards from private hands, resolve after four plays, increment winner trick count, winner leads the next trick, and move to `HandScoring` after trick 13.
- Added play/helper conformance golden trace JSON evidence.

Deviations from plan: introduced a local `Phase::HandScoring { completed_tricks }` placeholder so the thirteenth trick has a truthful endpoint without implementing scoring early. Scoring and terminal behavior remain deferred to GAT18BLAPACSPA-007.

Verification:

- `cargo test -p blackglass_pact --test rules --test property` passed (14 rules tests, 11 property tests).
- `cargo test -p game-stdlib` passed (28 unit tests, doc tests).
- `cargo test -p blackglass_pact` passed (1 lib test, 11 property tests, 14 rules tests, 2 serialization tests, 1 visibility test).
- `cargo fmt --all --check` passed.
- `git diff --check` passed.
