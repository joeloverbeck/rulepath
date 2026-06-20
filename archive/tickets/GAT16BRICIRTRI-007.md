# GAT16BRICIRTRI-007: Trick play legality and resolution

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/briar_circuit/src/rules.rs`, `src/actions.rs` (play family), trick substate in `state.rs`
**Deps**: 005

## Problem

This ticket implements Briar Circuit's core trick loop: the forced 2♣ opening, follow-suit obligation, the first-trick point-card restriction with its no-alternative exception, the hearts-broken lead rules (and the only-hearts exception that breaks hearts), the led-suit comparator, capture to the winner, and winner-leads-next sequencing. Legality generation and apply-path validation use the exact ordered rule in spec §3.1; TypeScript never restates it.

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/src/{cards,state,setup}.rs` and the pass phase exist after GAT16BRICIRTRI-005/006; this ticket fills `rules.rs` and the `play/<card-id>` action family in `actions.rs` (shared with the pass family from 006 — coordinate the mechanical merge). Action-tree/diagnostic contracts come from `engine-core`.
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 "Exact legality order" (the five ordered clauses) and the apply-path diagnostic list, plus Appendix A `BC-PLAY-001..007`/`BC-TRICK-001/002` and Appendix B.3 (`BC_TWO_CLUBS_MUST_OPEN`/`BC_MUST_FOLLOW_SUIT`/`BC_FIRST_TRICK_POINT_FORBIDDEN`/`BC_HEARTS_NOT_BROKEN`) fix the behavior.
3. Cross-artifact boundary under audit: the legal-card generator is the single legality authority consumed by bots (GAT16BRICIRTRI-011), the WASM action tree (013), and the web renderer (014); the apply path revalidates against the same rule. UI must not re-derive legality (§2).
4. FOUNDATIONS §11 no-leak applies to diagnostics: a stable public reason (wrong seat, stale token, unowned card, opening-lead violation, must-follow-suit, first-trick-point, hearts-not-broken) must not echo hidden alternatives or another seat's holdings. Legal-action generation is the only legality source (§2).

## Architecture Check

1. Encoding the §3.1 ordered clauses as one Rust legality function (used by both generation and apply-path revalidation) prevents drift between "what's offered" and "what's accepted," and keeps the exact ordering in one place.
2. No backwards-compatibility aliasing/shims — fills the `PlayingTrick` stubs.
3. `engine-core` stays free of trick/suit nouns (§3); no `game-stdlib` comparator (§4) — the second-use decision (003) keeps the comparator local; Gate 17 hard-gates a possible narrow comparator.

## Verification Layers

1. The five-clause legality order (2♣ open → follow-suit → first-trick points → hearts-unbroken lead → all legal) -> `tests/rules.rs` positive/negative per clause + `tests/property.rs` (legal set never empty in a non-terminal acting state; follow-suit closure).
2. Highest led-suit rank wins; off-suit never wins; winner captures and leads next -> `tests/property.rs` (`BC-TRICK-001/002`) + golden trace (authored in 010).
3. First-trick no-alternative exception allows all held cards when only point cards remain -> `tests/rules.rs` (`BC-PLAY-004`).
4. Diagnostics expose no hidden alternative/opponent content -> `tests/visibility.rs` diagnostic-scope check (extends 006).

## What to Change

### 1. `games/briar_circuit/src/rules.rs`

The exact §3.1 legality order for generation and apply-path revalidation; the led-suit comparator; capture transfer to the winner's record; winner-leads-next; hearts-broken state update (a played heart breaks hearts; Q♠ alone does not).

### 2. `games/briar_circuit/src/actions.rs` (play family)

`play/<card-id>` legal-action generation from the current state and stable diagnostics on apply.

### 3. `games/briar_circuit/src/state.rs` (trick substate)

Current trick (ordered public plays), leader/active seat, trick index, hearts-broken flag, and per-seat captured records.

## Files to Touch

- `games/briar_circuit/src/rules.rs` (modify; created by 004)
- `games/briar_circuit/src/actions.rs` (modify; created by 004; shares file with 006)
- `games/briar_circuit/src/state.rs` (modify; created by 004)
- `games/briar_circuit/tests/rules.rs` (modify; created by 004)
- `games/briar_circuit/tests/property.rs` (modify; created by 004)

## Out of Scope

- Hand/match scoring, moon, threshold/tie, outcome (GAT16BRICIRTRI-008).
- Semantic-effect filtering and the full pairwise visibility matrix (GAT16BRICIRTRI-009).
- Golden traces for the play rules (authored in GAT16BRICIRTRI-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test rules` — every §3.1 clause and `BC-PLAY-*`/`BC-TRICK-*` diagnostic, positive and negative.
2. `cargo test -p briar_circuit --test property` — legal set non-empty in acting states, follow-suit closure, off-suit never wins, captured cards partition played cards.
3. `cargo test -p briar_circuit --test visibility` — play diagnostics expose no hidden alternative or opponent content.

### Invariants

1. Legality is generated and revalidated only in Rust (§2); UI never re-derives it.
2. Off-suit cards never win; the winner captures all four and leads next (`BC-TRICK-001/002`).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/rules.rs` — clause-by-clause legality + diagnostics (extends 004/005/006).
2. `games/briar_circuit/tests/property.rs` — trick invariants (legal-set, follow-suit closure, capture partition).
3. `games/briar_circuit/tests/visibility.rs` — diagnostic no-leak (extends 006).

### Commands

1. `cargo test -p briar_circuit --test rules --test property --test visibility`
2. `cargo test -p briar_circuit`
3. A per-test scope is correct because the deliverable is play legality/resolution; scoring, full visibility, and traces are later tickets.

## Outcome

Completed on 2026-06-21. Implemented the Rust-owned trick legality rule in
the exact spec order, play action parsing/application, stable play diagnostics,
heart-breaking state transitions, led-suit winner selection, trick capture, and
winner-leads-next sequencing. Hand closeout now transitions to `ScoringHand`
with raw captured points so GAT16BRICIRTRI-008 can replace/extend scoring
policy without needing to finish trick mechanics.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p briar_circuit --test rules --test property --test visibility`
3. `cargo test -p briar_circuit`
