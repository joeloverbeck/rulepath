# GAT101PLATRI-006: Legal action tree, validation, and diagnostics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/plain_tricks/src/actions.rs`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-005

## Problem

The game needs the Rust-owned legal action tree: one `play` family with exactly one leaf per card legal under the must-follow-suit rule, plus command validation and diagnostics. Follow-suit legality is state-dependent on the actor's hidden hand and must be computed in Rust only (FOUNDATIONS §2); non-actor viewers must receive an empty tree (hidden-info pattern).

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/state.rs` (current trick, hands, led suit) exists from GAT101PLATRI-005. The action-tree / command-envelope / diagnostic contracts come from `engine-core` (`ActionTree`, `CommandEnvelope`, `Diagnostic`) per `docs/ARCHITECTURE.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md`; mirror `games/poker_lite/src/actions.rs`.
2. Spec §5 item 5 and appendix A3 fix the legality table: leader → every card; follower holding ≥1 led-suit card → exactly those (must follow); follower void → every card (free discard). Diagnostics: stale, wrong-seat, terminal, malformed-path, not-in-hand, must-follow-suit. Diagnostics must not name unplayed cards other than echoing the actor's own attempted card.
3. Shared boundary under audit: the action-tree + command-envelope + diagnostic schema (`engine-core`), and the actor-only-tree visibility rule. Action metadata may expose public trick state only (round/trick index, led suit/card once led, actor seat) plus, in the actor's own tree, the actor's own card ids/labels.
4. FOUNDATIONS §2 (Rust owns legal-action generation and validation; TS never decides legality) and §11 (legal-only controls; no hidden-info leak via action metadata to non-actors) are under audit.
5. Enforcement surface: §11 no-leak firewall on action-tree metadata. The legal tree and diagnostics must not leak any other held card's identity to a non-actor or via diagnostics; the actor's own tree necessarily names the actor's own playable cards only. No determinism/replay change beyond legal generation.

## Architecture Check

1. A single `play` family with one leaf per legal card (vs. a generic "play any card + validate later") makes illegal moves absent by construction (FOUNDATIONS §7 legal-only), and keeps follow-suit legality in Rust.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; trick/suit/follow nouns are `plain_tricks`-local. No `game-stdlib` change.

## Verification Layers

1. Legal tree excludes off-suit cards while the actor holds the led suit; offers full hand when leading or void -> rule unit tests citing `RULES.md` IDs.
2. Non-actor viewers receive an empty tree; actor's tree only to the actor -> no-leak visibility test (extended in GAT101PLATRI-009) + unit test.
3. Diagnostics (stale/wrong-seat/terminal/malformed/not-in-hand/must-follow) fire correctly and name no other held card -> diagnostic unit tests + golden diagnostic traces (GAT101PLATRI-011).
4. Validation is fail-closed and blocking -> schema/validation unit test.

## What to Change

### 1. `games/plain_tricks/src/actions.rs`

Implement the `play` action family: enumerate legal leaves per the A3 legality table from the actor's hand and current trick state. Attach safe action metadata (public trick state + actor's own cards only). Implement path parsing and `validate_command` with stale, wrong-seat, terminal, malformed-path, not-in-hand, and must-follow-suit diagnostics; the must-follow diagnostic reads "a card of the led suit must be played" without naming other held cards. Ensure non-actor viewers get an empty tree.

## Files to Touch

- `games/plain_tricks/src/actions.rs` (new)

## Out of Scope

- Applying actions / trick resolution / scoring transitions (GAT101PLATRI-007).
- Effects (GAT101PLATRI-008) and view projection (GAT101PLATRI-009).
- WASM actor-only tree authorization wiring (GAT101PLATRI-016) — this ticket provides the Rust tree + the empty-tree-for-non-actor rule it relies on.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` action-tree/validation tests: must-follow forced (one and several legal), void free-play, leader unconstrained, not-in-hand rejected.
2. Diagnostic tests assert each diagnostic code and that no diagnostic names a held card other than the actor's own attempted card.
3. Non-actor viewer receives an empty action tree.

### Invariants

1. The legal tree never offers an off-suit card while the actor holds the led suit (FOUNDATIONS §2; spec property test).
2. Validation is deterministic, fail-closed, and blocking; no legality is computable outside Rust (FOUNDATIONS §2/§11).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/rules.rs` — legal-action generation under follow-suit, path parsing, validation, diagnostics, citing `RULES.md` rule IDs.
2. Empty-tree-for-non-actor unit test — anchors the GAT101PLATRI-009 no-leak suite.

### Commands

1. `cargo test -p plain_tricks --test rules`
2. `cargo test -p plain_tricks`
3. The per-crate rule test is the correct boundary; cross-surface no-leak/replay proofs are exercised in GAT101PLATRI-009/011.
