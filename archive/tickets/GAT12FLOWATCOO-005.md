# GAT12FLOWATCOO-005: Budgeted action phase — tree, validation, application

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/flood_watch/src/actions.rs`, `src/rules.rs` (legal-action tree, validation, application, role-power modifiers)
**Deps**: GAT12FLOWATCOO-004

## Problem

A `flood_watch` turn is a multi-action budget: the active seat submits several validated commands against a tracked budget, and the legal tree regenerates after each action with remaining-budget metadata. This is the multi-action-budget shape the gate proves. The active seat's tree carries `bail`/`reinforce`/`forecast`/`end_turn`; the teammate's tree is empty with safe waiting metadata; `end_turn` is always present so a turn can never stall. Role powers (Pumpwright bails 2 levels, Levee Warden places 2 levees) are applied in Rust application logic, never in TypeScript.

## Assumption Reassessment (2026-06-11)

1. The GAT12FLOWATCOO-004 state model provides `Phase::Action { budget_remaining }`, `active_seat`, `roles`, and bounded `DistrictState`. `games/secret_draft/src/visibility.rs`/`ui.rs` is the verified exemplar for empty-tree-with-safe-waiting-metadata (grep-confirmed `waiting`/`waiting_copy` fields); `games/masked_claims` is the exemplar for the active/responder tree split and per-choice metadata.
2. The spec (§Implementation reference "Legal action tree" + "Validation", Work-breakdown item 4, Assumptions A4) fixes: active-seat tree = `bail/<district>` (level ≥ 1), `reinforce/<district>` (below levee cap), `forecast` (top card unrevealed), `end_turn` (always); choice metadata carries remaining budget, the role's effect magnitude as a Rust-supplied preview, and public district facts only. Validation rejects stale freshness token, non-seated/non-active actor, terminal phase, bail-on-dry, reinforce-at-cap, forecast-when-revealed, malformed paths, and zero-budget submissions.
3. Cross-artifact boundary under audit: the action-tree and command-envelope are the `engine-core` generic contracts (`ActionTree`, `ActionPath`, `CommandEnvelope`, `Diagnostic`) the WASM bridge and bots consume. Action paths (`bail/<district>`, etc.) are a stability contract — golden traces and replay pin them, so they must not churn after this ticket. Diagnostics must be viewer-safe and never reference the undrawn deck.
4. FOUNDATIONS §2 (Rust owns legal-action generation and validation) and §7 (public UI is legal-only; TypeScript invents no legality) motivate this ticket: the tree is the sole source of legality, and the teammate's empty-but-non-null tree plus always-present `end_turn` guarantee the UI never needs to synthesize an option or guess a stall.
5. Enforcement surface: validation is the §11 fail-closed surface. Every rejection is deterministic and blocking with a viewer-safe diagnostic; a zero-budget submission is validated even though the tree never offers it (defense in depth). Role-power magnitude is public knowledge, so previewing it leaks nothing.

## Architecture Check

1. Regenerating the tree after each action with remaining-budget metadata (rather than returning a fixed per-turn batch) keeps legality exact at every step and matches the proven per-action command/replay contract — the UI renders exactly what is legal now.
2. No backwards-compatibility aliasing/shims; built on the GAT12FLOWATCOO-004 state.
3. `engine-core` stays noun-free — `bail`/`reinforce`/`forecast` action kinds and role-power magnitudes live in `games/flood_watch`; only the generic `ActionTree`/`CommandEnvelope`/`Diagnostic` contracts are used.

## Verification Layers

1. Legal-tree correctness per action kind -> rule tests for bail level-bounds, reinforce levee-cap, forecast availability, end-turn always present.
2. Budget tracking and regeneration -> rule test: tree's remaining-budget metadata decrements per action; tree regenerates between actions.
3. No stall -> property test: the action-phase tree always contains `end_turn`.
4. Fail-closed validation, viewer-safe diagnostics -> diagnostics tests for stale token, wrong/non-active seat, dry-bail, cap-reinforce, forecast-when-revealed, zero-budget — each rejected with no undrawn-deck reference (no-leak).
5. Role powers game-local -> grep-proof that role magnitudes are applied in `src/rules.rs`, not surfaced as TS legality (`boundary-check.sh`-adjacent; full check in GAT12FLOWATCOO-015).

## What to Change

### 1. `games/flood_watch/src/actions.rs`

Generate the legal action tree for the active seat (`bail/<district>`, `reinforce/<district>`, `forecast`, `end_turn`) with remaining-budget and role-magnitude preview metadata and public district facts; return the empty waiting tree with safe metadata for the teammate; empty trees in terminal. Define the action-path encoding (stable).

### 2. `games/flood_watch/src/rules.rs`

Implement validation (all rejections in Assumption 2, fail-closed, viewer-safe diagnostics) and application of each action against the budget: `bail` reduces flood level (Pumpwright by 2, floor 0), `reinforce` adds a levee (Levee Warden 2, capped), `forecast` publicly reveals the top card. Decrement the budget; spending the final point sets up the environment-phase trigger handled in GAT12FLOWATCOO-006. Role-power magnitudes applied here only.

## Files to Touch

- `games/flood_watch/src/actions.rs` (modify — fill the stub)
- `games/flood_watch/src/rules.rs` (modify — add validation + action application; environment resolution lands in GAT12FLOWATCOO-006)

## Out of Scope

- Environment-phase draw/resolution and the turn-ending automation trigger body (GAT12FLOWATCOO-006).
- Terminal detection and the shared outcome (GAT12FLOWATCOO-007).
- Public/private projection and the `ForecastRevealed` effect payload's visibility filtering (GAT12FLOWATCOO-008) — application records the reveal; projection is built there.

## Acceptance Criteria

### Tests That Must Pass

1. Rule tests cover legality for every action kind (bail bounds, reinforce cap, forecast availability, end-turn), budget tracking and exhaustion, and role-power application for both roles.
2. Diagnostics tests cover stale token, wrong-seat, non-active seat, out-of-budget, bail-on-dry, reinforce-at-cap, forecast-when-revealed — all viewer-safe, no undrawn-deck hint.
3. A property test asserts the action-phase tree always contains `end_turn`.

### Invariants

1. TypeScript decides no legality; the Rust tree is the sole legality source, and the teammate tree is empty-but-non-null with `end_turn` always present for the active seat.
2. Diagnostics never reference the undrawn event-deck order or identities.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/rules.rs` — action legality, budget tracking, role-power application (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/property.rs` — `end_turn`-always-present no-stall invariant (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test rules`
2. `cargo test -p flood_watch`
3. The environment-phase and full-game simulation are exercised once GAT12FLOWATCOO-006 lands; the action-phase rule/diagnostic/property tests are the correct boundary for this diff.

## Outcome

Accepted on 2026-06-11. Implemented the Flood Watch action-phase legal tree,
validation, and budgeted application in Rust. The active seat receives
`bail/<district>`, `reinforce/<district>`, `forecast`, and `end_turn` choices
with public metadata; inactive seats receive an empty waiting tree with safe
metadata. Role powers, budget spending, final-budget environment handoff, and
fail-closed diagnostics are covered in rule and property tests.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p flood_watch --test rules`
3. `cargo test -p flood_watch --test property`
4. `cargo clippy -p flood_watch --all-targets -- -D warnings`
5. `cargo test -p flood_watch`
