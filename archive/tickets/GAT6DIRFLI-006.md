# GAT6DIRFLI-006: Action tree & exact placement previews

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/directional_flip/src/actions.rs` (flat action tree, legal-target metadata, forced-pass action, Rust-generated placement previews).
**Deps**: 005

## Problem

The public UI and bots act only through the Rust-generated legal action tree; no client may synthesize a path Rust did not expose (FOUNDATIONS §2, §7). This ticket builds the flat action tree (`place/<cell-id>`, `pass/forced`) with deterministic ordering and per-action metadata, and the exact placement previews (target cell, ordered flip set, optional direction grouping, viewer-safe explanation). The defining invariant (spec §7.2, `DF-PREVIEW-001`) is that **the preview flip set equals the apply flip set exactly**.

## Assumption Reassessment (2026-06-07)

1. `games/directional_flip/src/rules.rs` (GAT6DIRFLI-005) exposes the authoritative flip-collection function; the preview MUST call that same function, not a re-implementation, so preview and apply cannot diverge. `games/column_four/src/actions.rs` is the structural precedent for Rust action-tree generation with metadata.
2. Spec §7.1 (flat segments, required metadata: `action_kind`, `cell_id`, `row`, `column`, display + accessibility labels, preview, viewer-safe explanation; row-major deterministic ordering), §7.2 (preview contents), and rule ids `DF-ACTION-001`..`003`, `DF-PREVIEW-001` are authoritative.
3. Cross-artifact boundary under audit: the action-tree / action-path / preview shape is consumed by `wasm-api` (GAT6DIRFLI-015), the web renderer (017), bots (011), and replay (009). The action path/segment vocabulary conforms to the generic `engine-core` action-tree contract — `directional_flip` supplies game-specific segments (`place/<cell-id>`, `pass/forced`) within that generic shape, not a new kernel contract.
4. FOUNDATIONS §2/§7 motivate this ticket: restate before coding — Rust owns legal action generation and previews; the UI presents them and never invents legality or pass availability. Forced pass appears as the sole legal action only when rules-core reports no placement (spec §6.5); the action tree must not contain it otherwise (`DF-ACTION-003`).
5. The preview carries a **viewer-safe explanation** and an ordered flip set destined for browser payloads — this is a no-leak surface (FOUNDATIONS §11). Confirm the preview exposes only perfect-information facts (target, flips, direction grouping, human-readable text) and no engine internals, RNG state, or stale-token internals. The preview flip set must be deterministic (it reuses the §6.4 canonical order from GAT6DIRFLI-005), so it stays replay/hash-stable.

## Architecture Check

1. Generating previews by calling the single rules-core flip-collection function (rather than a parallel preview path) structurally guarantees `DF-PREVIEW-001` — there is no second code path to drift.
2. No backwards-compatibility shims; new generation logic.
3. `engine-core` stays noun-free — action segments and previews are game-local strings/structs within the generic action-tree contract; no `flip`/`cell` vocabulary enters the kernel (§3).

## Verification Layers

1. Legal-only action tree -> rule test (`DF-ACTION-001`/`003`): tree contains only legal placements when placements exist; `pass/forced` is the sole entry only when none exist.
2. Deterministic ordering -> rule test: placement choices are row-major by cell id, stable across runs.
3. Preview == apply -> golden trace + property test (`DF-PREVIEW-001`): for every legal target, the previewed ordered flip set equals the set produced by applying the placement.
4. Preview no-leak -> no-leak visibility test: preview payload contains only viewer-safe fields (no RNG/internal/stale-token state) (FOUNDATIONS §11).

## What to Change

### 1. Action-tree generation

In `actions.rs`, generate the flat action tree from a state: one `place/<cell-id>` per legal target with metadata (`action_kind`, `cell_id`, `row`, `column`, display label, accessibility label, viewer-safe explanation, preview), sorted row-major; or exactly one `pass/forced` action (with reason code) when rules-core reports no legal placement; or an empty/terminal tree when terminal (spec §6.5/§7.1).

### 2. Placement previews

For each legal target, build the Rust preview per spec §7.2 by invoking the rules-core flip-collection function: target cell id, row/column, accessible label, ordered flip-cell list, optional direction grouping (same direction order as effects), viewer-safe explanation text, and a stable preview id if needed for the view/effect bridge.

## Files to Touch

- `games/directional_flip/src/actions.rs` (new)
- `games/directional_flip/src/lib.rs` (modify — export the actions module)

## Out of Scope

- Flip/legality computation itself (owned by GAT6DIRFLI-005; this ticket consumes it).
- Semantic effects (GAT6DIRFLI-008), public view projection (007).
- Any client-side action synthesis or preview computation (forbidden; FOUNDATIONS §2, spec §12.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — action-tree + preview tests pass, including a preview==apply assertion over all legal targets of representative states.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Every legal action path is Rust-generated; the preview flip set equals the apply flip set exactly (FOUNDATIONS §2, spec `DF-PREVIEW-001`).
2. Preview payloads expose only viewer-safe perfect-information facts (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/rules.rs` — action-tree legality/ordering + `DF-PREVIEW-001` preview==apply property (expanded in GAT6DIRFLI-012; a golden trace lands in 013).

### Commands

1. `cargo test -p directional_flip actions`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct; the cross-surface preview-flip-set golden trace is GAT6DIRFLI-013 (needs replay support).

## Outcome

Completed: 2026-06-07

What changed:

- Added `games/directional_flip/src/actions.rs`.
- Implemented Rust-generated flat action trees for active actors: row-major `place/<cell-id>` choices when legal placements exist, exactly one `pass/forced` choice when no placement exists, and empty trees for terminal or non-active actors.
- Added typed `PlacementPreview` and `DirectionPreview` builders that consume the rules-core `Placement`/`FlipRun` data instead of reimplementing flip scanning.
- Added viewer-safe metadata for action kind, target cell, row, column, preview id, ordered flip cells, direction groups, and explanation.
- Exported action-tree and preview APIs from `games/directional_flip/src/lib.rs`.

Deviations from original plan:

- The generic `engine-core::ActionPreview` shape is currently only `Available`/`Unavailable`, so detailed preview data is carried as game-local typed structs and string metadata on the action choice. Later WASM/view tickets can map that surface without adding mechanic nouns to `engine-core`.
- Tests were seeded inline in `actions.rs`; GAT6DIRFLI-012 can expand or move them when the broader test suite lands.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p directional_flip actions` passed: 6 action tests passed.
- `cargo build -p directional_flip` passed.
- `cargo test -p directional_flip` passed: 18 tests passed.
- `bash scripts/boundary-check.sh` passed.
