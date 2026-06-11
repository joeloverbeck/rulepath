# GAT13FROCONASY-007: Public projection, visibility, and replay surfaces

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/frontier_control/src/{visibility,replay_support}.rs` (single public projection, identity effect filter, action/effect/view hashes, full-timeline replay export/import)
**Deps**: GAT13FROCONASY-006

## Problem

Frontier Control is perfect-information: every projection must be identical for both seats and observers, including the Rust-computed supply-connectivity set. The visibility layer is a single public projection with an identity effect filter; replay export/import carries the full public command/effect timeline with no redaction surface. Traces must carry the explicit `not_applicable` markers Trace Schema v1 §5 requires for hidden-information, stochastic, and private-view surfaces, recording that there is no seed-derived game state to protect (ADR 0004 not engaged).

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/src/visibility.rs` and `replay_support.rs` are exemplars, but flood_watch carries viewer-scoped export (hidden info); frontier_control's perfect-information scope **removes** that branch — the effect filter is the identity projection and export is the full public timeline (no viewer-aware redaction), so this is one ticket, not the visibility/replay split flood_watch used.
2. Spec §Visibility and replay model defines the single public projection (sites, supplied/cut flags from the last scoring, scores, round/turn/budget, factions, terminal), all-viewer output-equivalence, and the Trace Schema v1 §5 `not_applicable` markers; `docs/TRACE-SCHEMA-v1.md` §5 (verified) names the four markers (hidden-info redaction, stochastic events, private-view hashes, preview hashes).
3. Cross-crate boundary under audit: the public view conforms to the generic `engine-core` public/private-view contract; the supplied/cut set computed in GAT13FROCONASY-006 is projected here so the UI never re-derives connectivity; action/effect/view/state hashes feed `replay-check`.
4. FOUNDATIONS §11 universal invariants under audit: public/private views are viewer-safe; replay/hashes/serialization order are deterministic; for a perfect-information game the hidden-info and RNG invariants are satisfied trivially and documented as such.
5. §11 no-leak firewall + deterministic replay/hash enforcement surface: `get_view` for `seat_0`, `seat_1`, and observer must be output-equivalent; the dev panel and replay export contain only that same public projection; determinism is over (seats, variant, command stream) with no game-rule seed. This is the surface GAT13FROCONASY-009's visibility/replay tests and GAT13FROCONASY-012's WASM export bind to.
6. Schema extension: the public-view shape and replay export/import format are introduced for this game additively (no existing consumer); the trace `not_applicable` block is additive metadata per Trace Schema v1 §5.

## Architecture Check

1. A single identity-projection visibility layer is the minimal correct design for perfect information — adding a viewer-scoped export path (as hidden-info games need) would be dead complexity and a larger leak surface; the perfect-information posture is the simplification.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; projection/replay use existing generic contracts; no `game-stdlib` helper introduced.

## Verification Layers

1. All-viewer output-equivalence (§11 no-leak) -> no-leak visibility test (`get_view` identical for `seat_0`/`seat_1`/observer; dev panel + export carry only the public projection).
2. Deterministic replay/hash -> deterministic replay-hash check (`replay-export-import.trace.json`; same command stream reproduces state/effect/action-tree/view hashes — exercised in GAT13FROCONASY-009 and `replay-check` in GAT13FROCONASY-013).
3. Trace `not_applicable` discipline (§11) -> golden trace / schema validation (every trace carries the Trace Schema v1 §5 markers).

## What to Change

### 1. Public projection (`visibility.rs`)

Build the single public view (including the projected supplied/cut set); the effect filter is the identity projection; emit stable summaries and action/effect/view hashes.

### 2. Replay support (`replay_support.rs`)

Export/import the full public command/effect timeline losslessly; determinism over (seats, variant, command stream); emit the Trace Schema v1 §5 `not_applicable` markers for hidden-info/stochastic/private-view/preview surfaces.

## Files to Touch

- `games/frontier_control/src/visibility.rs` (modify)
- `games/frontier_control/src/replay_support.rs` (modify)

## Out of Scope

- Bots (GAT13FROCONASY-008).
- The full test/trace suite (GAT13FROCONASY-009) — stubs only here.
- WASM export wiring (GAT13FROCONASY-012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` visibility tests: `get_view` output-equivalent for both seats and observer; export/import lossless.
2. Replay test: identical (seats, variant, command stream) reproduces all hashes and the scoring breakdown/terminal.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. No path lets any viewer-specific projection diverge; there is one public view (§11).
2. Determinism holds over the command stream with no game-rule seed; traces record hidden-info/stochastic surfaces as `not_applicable`.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/visibility.rs` — all-viewer equivalence + dev-panel/export public-only (stubbed; expanded in GAT13FROCONASY-009).
2. `games/frontier_control/tests/replay.rs` — hash reproduction + lossless export/import (stubbed; expanded in GAT13FROCONASY-009).

### Commands

1. `cargo test -p frontier_control visibility replay`
2. `cargo test -p frontier_control`
3. Crate-scoped tests are the correct boundary; cross-tool `replay-check --all` runs after trace registration in GAT13FROCONASY-013.
