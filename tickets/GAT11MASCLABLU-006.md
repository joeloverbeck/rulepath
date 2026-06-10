# GAT11MASCLABLU-006: Reaction-window phase and pending-response tree

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/src/effects.rs`, extends `src/actions.rs` and `src/rules.rs`, `src/lib.rs`
**Deps**: GAT11MASCLABLU-005

## Problem

This is the core interaction class Gate 11 proves: applying a claim suspends resolution and opens a reaction window in which only the responding seat has legal actions, and the log explains who may respond and why. The claimant's tree is empty with waiting metadata. No effect payload may carry the pedestal tile ID.

## Assumption Reassessment (2026-06-10)

1. `src/actions.rs` claim tree and `src/rules.rs` validation from GAT11MASCLABLU-005 provide the claim apply-point. The actor-specific empty-tree-while-waiting pattern is already proven by `games/secret_draft` (spec §3 alignment); the effect-envelope shape model is `games/plain_tricks/src/effects.rs`.
2. Spec §"Turn flow" steps 1–2 and §"Semantic effect model": applying a claim moves the tile face-down to the pedestal and emits `ClaimPlaced { turn, claimant, declared_grade }` then `ReactionWindowOpened { turn, responder, choices: [accept, challenge] }` with log copy explaining who may respond and why; the responder tree is exactly `respond/accept` and `respond/challenge`; the claimant tree is empty with waiting metadata.
3. Cross-artifact boundary under audit: the effect-envelope visibility contract (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`). `ClaimPlaced` and `ReactionWindowOpened` are Public and carry the declared grade only; the "who may respond and why" line is Rust-emitted copy, not TS-assembled.
4. FOUNDATIONS §2 (window membership and timing are Rust-owned) is the principle under audit; React only renders the waiting state and prompt.
5. §11 no-leak firewall enforcement surface: effect payloads and reaction-window metadata. Confirm `ClaimPlaced`/`ReactionWindowOpened` carry no tile ID, and the responder's choice metadata derives only from public state (declared grade, public counting facts).

## Architecture Check

1. A reaction-window `Phase` whose responder tree *replaces* the global action space gives legality-by-construction — the claimant cannot act because no actions exist, not because validation rejects them.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free (`reaction`/`response window`/`pending` nouns are game-local); no `game-stdlib` reaction-window or pending-response helper — broad generalization is `ADR-required` per `docs/MECHANIC-ATLAS.md`.

## Verification Layers

1. Reaction-window legality (responder-only; claimant empty) -> rule test (full suite in GAT11MASCLABLU-010).
2. "Who may respond and why" log copy -> effect-text assertion + golden trace `claim-pending-window` (GAT11MASCLABLU-011).
3. Effect payload no-leak (no tile ID in `ClaimPlaced`/`ReactionWindowOpened`) -> no-leak visibility test.
4. Out-of-window diagnostics -> diagnostics rule test: claim during a window, response outside a window, wrong-seat response, stale submission — all viewer-safe.

## What to Change

### 1. `src/effects.rs`

Define the public effects `ClaimPlaced { turn, claimant, declared_grade }` and `ReactionWindowOpened { turn, responder, choices }` with viewer-safe log copy naming the responder, the allowed responses, and the reason (a claim is pending on the pedestal). No tile ID in any payload.

### 2. `src/actions.rs` (reaction window)

Responder tree: exactly `respond/accept` and `respond/challenge`, with viewer-safe metadata (declared grade, public counting facts derived from public state only). Claimant tree: empty with "waiting for the other seat to respond" metadata.

### 3. `src/rules.rs` (window membership + apply-claim)

Applying a claim transitions to `Phase::Reaction { responder }` and emits the two effects. Validate response submissions: wrong-phase, wrong-seat, stale freshness — viewer-safe diagnostics, exactly one open window at a time.

## Files to Touch

- `games/masked_claims/src/effects.rs` (new)
- `games/masked_claims/src/actions.rs` (modify)
- `games/masked_claims/src/rules.rs` (modify)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Conditional resolution, reveal, scoring, terminal, tie-breaks (GAT11MASCLABLU-007).
- Public/seat view projection and replay surfaces (GAT11MASCLABLU-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` reaction-window legality and diagnostics tests pass.
2. Exactly one reaction window is open at a time; the claimant has no legal actions during it.
3. `ClaimPlaced` and `ReactionWindowOpened` payloads carry the declared grade and no tile ID.

### Invariants

1. Window membership and timing are Rust-owned; React renders only the prompt/waiting state (FOUNDATIONS §2).
2. The pedestal tile ID does not appear in any reaction-phase effect payload or metadata (FOUNDATIONS §11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/rules.rs` `#[cfg(test)]` — window legality, single-open-window invariant, response diagnostics.
2. `games/masked_claims/src/effects.rs` `#[cfg(test)]` — `ReactionWindowOpened` log copy states who may respond and why; no tile ID in payloads.

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. Unit-level boundary; the golden-trace proof of the window's log line is exercised in GAT11MASCLABLU-011.
