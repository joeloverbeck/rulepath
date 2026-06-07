# GAT6DIRFLI-007: Public view, visibility projection & UI metadata

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip/src/visibility.rs` (public view projection, no-leak handling) and `games/directional_flip/src/ui.rs` (Rust-owned UI labels, token metadata, accessibility labels, preview copy).
**Deps**: 005

## Problem

The browser receives only viewer-safe public views; perfect information does not excuse dumping engine internals (FOUNDATIONS §11, spec §7.4). This ticket builds the public view projection (board cells/owners, active seat, legal targets + preview metadata, counts, terminal outcome, last-action summary, bot rationale) and the Rust-owned UI metadata (`ui.rs`: token shapes, labels, legal-target labels, accessibility labels, preview copy) the renderer consumes. Realizes `DF-VIEW-001`.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/src/visibility.rs` and `games/column_four/src/ui.rs` are the structural precedents (public view projection + Rust UI metadata). `games/directional_flip/src/state.rs` (GAT6DIRFLI-004) holds the authoritative state to project; `rules.rs` (005) supplies counts and terminal outcome.
2. Spec §7.4 (allowed view fields vs. forbidden internals) and rule id `DF-VIEW-001` are authoritative. `directional_flip` is a perfect-information game, so the public view is full-board — but must still exclude raw RNG state, stale-token internals beyond safe diagnostics, and behavior-like rule data.
3. Cross-crate boundary under audit: `games/directional_flip` ↔ the `engine-core` public/private view contract. The projection conforms to the generic view contract; `ui.rs` metadata is consumed downstream by `wasm-api` (015) and the renderer (017). Confirm field names against `docs/ENGINE-GAME-DATA-BOUNDARY.md`, not FOUNDATIONS prose.
4. FOUNDATIONS §11 (public/private views viewer-safe; hidden information must not leak) and §7 (Rust supplies viewer-safe views; TS presents) motivate this ticket: restate before coding — even in a perfect-information game the view is a deliberate projection, never a state dump.
5. This ticket is the **no-leak visibility firewall** for `directional_flip` (FOUNDATIONS §11, §12). Confirm no path lets engine internals, raw RNG state, hidden debug structures, or behavior-like rule data reach the view payload, DOM, logs, previews, or (later) replay exports. The view must be deterministic given state (no wall-clock, no iteration-order leakage).

## Architecture Check

1. A single Rust projection function (state → public view) plus a single `ui.rs` metadata source keeps the viewer-safe contract in one auditable place and lets the renderer be purely presentational — the §7 division.
2. No backwards-compatibility shims; new projection + metadata.
3. `engine-core` stays noun-free — the view struct and UI metadata are game-local within the generic view contract; no `board`/`cell` vocabulary enters the kernel (§3).

## Verification Layers

1. No-leak invariant -> no-leak visibility test (`DF-VIEW-001`): the public view (and the UI metadata it carries) contains no hidden/internal/RNG/stale-token state; only spec §7.4 allowed fields appear.
2. View completeness -> schema/serialization validation: the view conforms to the `engine-core` public-view contract and round-trips through serialization with stable field order.
3. UI metadata sufficiency -> manual review: `ui.rs` supplies token shapes/labels/legal-target/accessibility labels and preview copy the renderer (017) needs, with non-color-only encoding hooks (shape/pattern, per spec §12.3).

## What to Change

### 1. Public view projection

In `visibility.rs`, project state → public view per spec §7.4: cells + owners, active seat, legal targets + Rust preview metadata, per-seat counts, terminal outcome, last-action/effects summary, post-bot rationale, and UI metadata. Exclude all forbidden internals.

### 2. Rust UI metadata

In `ui.rs`, author the Rust-owned UI labels, token shape/pattern metadata (for non-color-only encoding), accessibility labels, legal-target labels, and preview copy strings — all viewer-safe.

## Files to Touch

- `games/directional_flip/src/visibility.rs` (new)
- `games/directional_flip/src/ui.rs` (new)
- `games/directional_flip/src/lib.rs` (modify — export visibility + ui modules)

## Out of Scope

- Effects (GAT6DIRFLI-008) and replay export (009) — though both must respect this no-leak contract.
- The TypeScript renderer (GAT6DIRFLI-017), which consumes this metadata but adds no legality.
- Bot rationale generation (GAT6DIRFLI-011); this view only carries it through safely.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — visibility tests pass, including a negative no-leak assertion.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. The public view leaks no hidden/internal state through any field (FOUNDATIONS §11, `DF-VIEW-001`).
2. View projection is deterministic given state; no nondeterministic input enters it (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/visibility.rs` — public view contains only allowed fields; explicit negative assertions that RNG/internal/stale-token state is absent (expanded in GAT6DIRFLI-012).

### Commands

1. `cargo test -p directional_flip visibility`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped visibility tests are the correct boundary; the browser-payload no-leak smoke is GAT6DIRFLI-018.
