# GAT5COLFOUPUB-004: Column Four public view & visibility projection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/column_four/src/visibility.rs`, `games/column_four/src/ui.rs`
**Deps**: 003

## Problem

The web shell must render the board and controls without inferring rules, so Rust must project a viewer-safe public view carrying occupancy, active seat, legal columns, terminal kind, winning line, and preview metadata. A perfect-information game still must declare its private-view status and an empty hidden-fields set so the no-leak contract is explicit (spec §8.1, §8.2).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/src/visibility.rs` and `src/ui.rs` are the template: a Rust public-view projection (cells, legal targets, terminal kind, winning line, status) and UI metadata. Verified both modules exist; this ticket mirrors them for a 7×6 board with column summaries and a Rust-provided landing preview.
2. Spec §8.1 (required public-view concepts) and §8.2 (action-tree requirements) define fields: schema/rules version, `game_id`, display name, variant id, board rows=6/cols=7, full cell occupancy in stable order, active seat (non-terminal), ply, status, freshness token, legal column targets, terminal kind (non-terminal/win/draw), winning seat + winning-line cell ids on win, draw marker, `not_applicable_perfect_information` private status, empty hidden-fields array, per-column summaries (id/label/full/legal-target/landing preview). Rule/coordinate types come from 002/003.
3. Cross-artifact boundary under audit: the public/private-view contract in `docs/ARCHITECTURE.md` and `docs/ENGINE-GAME-DATA-BOUNDARY.md`, and the no-leak firewall (`docs/WASM-CLIENT-BOUNDARY.md`). The projection emits only viewer-safe fields; it adds no mechanic noun to `engine-core`.
4. FOUNDATIONS §11 (public/private views are viewer-safe; hidden information does not leak) motivates this ticket. Restating: for a perfect-information game the public view carries the whole board, but candidate rankings, bot internals, or any non-viewer field MUST NOT appear; the private-view status is an explicit `not_applicable_perfect_information` marker, not an omission.

## Architecture Check

1. Rust-owned projection with explicit column summaries lets the renderer be pure coordinate-to-pixel mapping — cleaner and safer than letting TypeScript derive legality/landing from raw occupancy (a §12 stop condition).
2. No backwards-compatibility aliasing/shims — new projection on the 003 rule engine.
3. `engine-core` stays free of mechanic nouns (view fields are game-local, projected into the generic public-view envelope); `game-stdlib` untouched.

## Verification Layers

1. Field-completeness invariant -> unit test asserting every spec §8.1 required field is present in the projected view for non-terminal, win, and draw states.
2. No-leak invariant -> no-leak visibility test: the public view exposes no hidden/internal/candidate-ranking field; private status is `not_applicable_perfect_information`, hidden-fields array empty.
3. Legal-target consistency invariant -> unit test: the view's legal column targets exactly match the rule engine's legal columns (full columns excluded), cross-checked against GAT5COLFOUPUB-003.
4. Stable-order invariant -> unit test: cell occupancy and column summaries serialize in the documented stable coordinate order (deterministic).

## What to Change

### 1. `games/column_four/src/visibility.rs`

Project the Rust state into the viewer-safe public view with all spec §8.1 fields, including per-column summaries (column id, label, full/non-full, legal-target id when legal, Rust-provided landing-row preview) and winning-line cell ids on terminal win. Provide the private-view status `not_applicable_perfect_information` and an empty hidden-fields array.

### 2. `games/column_four/src/ui.rs`

Typed UI metadata sufficient for legal controls, neutral column labels (`Column 1`..`Column 7`), accessibility names, and preview anchoring — no behavior, presentation metadata only.

## Files to Touch

- `games/column_four/src/visibility.rs` (new)
- `games/column_four/src/ui.rs` (new)

## Out of Scope

- Semantic effects (GAT5COLFOUPUB-005) and replay projection reuse (006).
- The TypeScript `ColumnFourPublicView` type and renderer (GAT5COLFOUPUB-014) — this ticket owns the Rust projection only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four visibility` — required-field, legal-target consistency, and stable-order tests pass.
2. `cargo test -p column_four` — no regression to the rule engine.
3. Manual/no-leak review: no public-view field exposes hidden or internal state.

### Invariants

1. The public view carries only viewer-safe fields; private status is the explicit perfect-information marker; hidden-fields array is empty.
2. View legal targets equal the rule engine's legal columns for every state.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/visibility.rs` (unit tests) — field completeness across non-terminal/win/draw, legal-target equality with rules, stable serialization order, empty hidden-fields.
2. `games/column_four/src/ui.rs` (unit test) — neutral column labels and accessibility-name metadata present.

### Commands

1. `cargo test -p column_four visibility`
2. `cargo test -p column_four`
3. `cargo clippy -p column_four --all-targets -- -D warnings`
