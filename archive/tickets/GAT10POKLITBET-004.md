# GAT10POKLITBET-004: Legal action tree, validation, and diagnostics

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/poker_lite/src/actions.rs` + `games/poker_lite/tests/rules.rs` (created here). Consumes `engine-core` `ActionTree`/`ActionPath`/`CommandEnvelope`/`Diagnostic` contracts. No kernel change.
**Deps**: GAT10POKLITBET-003

## Problem

Rust must own the legal action tree, command validation, and diagnostics for `poker_lite`. The action families `hold`/`press`/`lift`/`match`/`yield` must be generated with public-only metadata, bounded by round units `[1,2]` and the one-lift-per-round cap, and validation must reject stale, wrong-seat, terminal, malformed-path, unavailable-action, and lift-cap-exceeded commands with diagnostics that reveal no hidden cards.

## Assumption Reassessment (2026-06-08)

1. The action/validation shape matches siblings: `games/secret_draft/src/actions.rs` and `games/high_card_duel/src/actions.rs` build an `engine-core` `ActionTree`, parse `ActionPath`s, and validate `CommandEnvelope`s producing `Diagnostic`s. Verified `ActionTree`, `ActionPath`, `CommandEnvelope`, `Diagnostic` are exported from `crates/engine-core/src/lib.rs` (and `game.rs`). This ticket creates `tests/rules.rs` (extended by GAT10POKLITBET-005).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §A3 action-families table, §B "safe action metadata", §4 actions.rs) fixes the legality predicates and the allowed public metadata set: `action_family`, `round_index`, `round_unit`, `actor_seat`, `required_to_match`, `adds_to_pool`, `shared_pool_after`, `lift_cap_remaining`, `center_visible: true/false`, accessibility copy. No card id, rank, opponent strength, or inferred hidden state in metadata.
3. Cross-artifact boundary under audit: the `engine-core` action-tree / command-envelope / diagnostic contract (authoritative shape in `docs/ARCHITECTURE.md` + `docs/ENGINE-GAME-DATA-BOUNDARY.md`). Action metadata is a public-view-adjacent surface — it must carry only the fields enumerated in §A3, validated against the boundary doc, not FOUNDATIONS prose.
4. FOUNDATIONS §11 (TypeScript does not decide legality; UI is legal-only) and §2 (behavior authority) motivate this ticket. Restated: the legal tree and validation are the sole source of legality; the browser presents the tree, never invents it.
5. No-leak firewall surface under audit (§11/§12): action-tree metadata, previews, and diagnostics must not leak the hidden center card (beyond the boolean `center_visible`), either private card, the deck tail, or opponent strength. The lift-cap and wrong-seat diagnostics must describe the rule violation without echoing hidden state. This is a primary no-leak surface; string-search no-leak tests over action-tree/diagnostic JSON land in GAT10POKLITBET-007/008, but the metadata allow-list is enforced here.

## Architecture Check

1. Generating only legal leaves with a fixed public-metadata allow-list (rather than emitting all families and tagging illegality) keeps the browser legal-only by construction and removes any path for TS to re-derive legality. Matches the sibling action-tree pattern.
2. No backwards-compatibility aliasing/shims — new module.
3. `engine-core` stays noun-free — the action tree is generic; crest/pledge semantics live in `actions.rs` (§3). No `game-stdlib` promotion (§4).

## Verification Layers

1. Legal-only generation (no illegal lift after cap; no action for non-actor seat) -> `cargo test -p poker_lite --test rules` legality tests.
2. Validation fail-closed (stale/wrong-seat/terminal/malformed/unavailable/lift-cap each produce the correct blocking diagnostic) -> `tests/rules.rs` validation tests, one per diagnostic.
3. Metadata allow-list (only the §A3 public fields present; no hidden id/rank) -> schema/serialization assertion over action-tree JSON + codebase grep-proof that no card-id field is constructed in metadata.
4. Boundary/contract conformance (action tree matches `engine-core` shape) -> schema validation against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.

## What to Change

### 1. `games/poker_lite/src/actions.rs`

Build the legal `ActionTree` for the active seat per §A3: `hold` (no outstanding pledge), `press` (no outstanding pledge, not already pressed), `lift` (facing pledge, cap unused), `match` (facing pledge), `yield` (facing pledge). Attach only allow-listed public metadata. Parse `ActionPath`s. Validate `CommandEnvelope`s; emit `Diagnostic`s for stale (freshness mismatch), wrong-seat, terminal, malformed-path, unavailable-action, and lift-cap-exceeded — none echoing hidden cards.

### 2. `games/poker_lite/tests/rules.rs` (new)

Action-generation and validation tests: legal sets per phase, one-lift-cap enforcement, each diagnostic path, and a metadata allow-list assertion. (GAT10POKLITBET-005 extends this file with transition/showdown/accounting tests.)

## Files to Touch

- `games/poker_lite/src/actions.rs` (new)
- `games/poker_lite/tests/rules.rs` (new)
- `games/poker_lite/src/lib.rs` (modify — add `mod actions;` + re-exports)

## Out of Scope

- State transitions, round-close, center reveal, showdown, accounting (GAT10POKLITBET-005).
- Effects emission (GAT10POKLITBET-006).
- Exhaustive cross-surface no-leak string-search tests (GAT10POKLITBET-007/008) — this ticket enforces the metadata allow-list but not the full firewall sweep.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test rules` — legal-action generation + one-lift-cap + all six diagnostics.
2. Metadata allow-list test: action-tree leaf metadata contains only the §A3 public fields; no `card`/`rank`/hidden-center field.
3. `cargo test -p poker_lite` passes overall.

### Invariants

1. Legality comes only from the Rust action tree; an illegal lift after the cap is never offered (§11, §12 "illegal moves clickable").
2. No action metadata or diagnostic exposes a hidden card id/rank, the deck tail, or opponent strength (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/rules.rs` — legal-action generation, lift-cap, six diagnostics, metadata allow-list.

### Commands

1. `cargo test -p poker_lite --test rules`
2. `cargo test -p poker_lite`
3. `bash scripts/boundary-check.sh` — confirms no mechanic noun leaked into `engine-core` via the action-tree integration.

## Outcome

Completed: 2026-06-09

Changed:

- Added `games/poker_lite/src/actions.rs` with Rust-owned legal action tree generation for `hold`, `press`, `lift`, `match`, and `yield`.
- Added command parsing and validation for stale, wrong-seat, terminal, malformed-path, unavailable-action, and lift-cap-exceeded diagnostics.
- Added public-only action metadata for action family, round/accounting facts, lift-cap status, center-visible boolean, and accessibility copy.
- Added `games/poker_lite/tests/rules.rs` coverage for legal sets, one-lift cap behavior, metadata allow-listing, and validation diagnostics.
- Re-exported action helpers and validated action types from `games/poker_lite/src/lib.rs`.

Deviations from original plan:

- Kept this ticket to legality/validation only. State mutation, round close, reveal, showdown, and accounting transitions remain deferred to GAT10POKLITBET-005.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite --test rules` passed: 3 integration tests.
- `cargo test -p poker_lite` passed: 17 unit tests, 3 integration tests, and 0 doc tests.
- `bash scripts/boundary-check.sh` passed.
