# GAT20STACROSTA-005: Typed board topology content (121-space six-pointed star)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/starbridge_crossing/src/topology.rs`, `games/starbridge_crossing/data/manifest.toml` (typed static content)
**Deps**: GAT20STACROSTA-004

## Problem

Starbridge needs a non-rectangular 121-space six-pointed-star topology that the promoted rectangular `board_space` primitive cannot model. This ticket lands the typed topology content (stable space ids, axial/cube coordinates, home/target zones, neighbor metadata) as static data, with all legality remaining Rust-owned per `docs/FOUNDATIONS.md` §5.

## Assumption Reassessment (2026-06-27)

1. Topology lives in `games/starbridge_crossing/src/topology.rs` (game-local module owning no kernel contract, exporting no shared helper) per spec §4; it consumes the id types from `src/ids.rs` (GAT20STACROSTA-004).
2. `data/manifest.toml` follows the sibling typed-content shape (`games/meldfall_ledger/data/manifest.toml`): typed fields only (`id`, `q`, `r`, `s`, zone labels, UI anchors, neighbor ids) — no selectors/branches/triggers (§5).
3. Cross-artifact boundary: the 121-space content is the substrate every legality module (setup, step, hop, finish) reads; coordinate model `q+r+s==0` and six-direction neighbor array `[Option<StarSpaceId>; 6]` are pinned from spec Appendix A.
4. §5 (static data is typed content, not behavior) motivates this ticket: the manifest may store ids/coords/zones/neighbors but MUST NOT encode adjacency *rules*, jump legality, or blocking — those are Rust functions in `topology.rs`/`rules.rs`. Unknown fields are rejected by default (§11).
5. Substrate for deterministic replay (§11): the topology's stable manifest order is the canonical id ordering all later traces/hashes depend on; confirm the order is fixed and documented so the deferred replay/serialization surfaces (GAT20STACROSTA-011) stay deterministic and introduce no nondeterminism path.

## Architecture Check

1. Generated-or-authored typed constants + a typed loader keep the shape declarative while legality stays in Rust — cleaner than encoding behavior in data and avoids the rejected `board_space` broadening.
2. No backwards-compatibility shims.
3. `engine-core` untouched; no `game-stdlib` graph helper added (the §4 defer/reject decision in 002 holds); topology is game-local.

## Verification Layers

1. Exactly 121 stable spaces -> unit test asserting space count and unique ids.
2. Neighbor symmetry + degree range -> unit test: if A lists B as neighbor in direction d, B lists A in the opposite direction; degree within expected bounds.
3. Coordinate invariant -> unit test: every `StarCoord` satisfies `q+r+s==0`; opposite-home mapping is involutive.
4. Static-data discipline (§5) -> schema validation: `manifest.toml` parse rejects unknown/behavior-looking fields.

## What to Change

### 1. Author `src/topology.rs`

`StarCoord { q, r, s }`, `StarSpace { id, coord, zone, ui_anchor, neighbors: [Option<StarSpaceId>; 6] }`, the 121-space table (generated constants or typed loader), neighbor lookup, opposite-home mapping, and coordinate↔id conversion. All adjacency/zone *behavior* is Rust here, not in data.

### 2. Author `data/manifest.toml`

Typed content: rules/data/trace version labels, per-space `id`/`q`/`r`/`s`/zone/ui-anchor/neighbor-ids. No selectors, conditions, or triggers.

## Files to Touch

- `games/starbridge_crossing/src/topology.rs` (new)
- `games/starbridge_crossing/data/manifest.toml` (new)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod topology;`)

## Out of Scope

- Seat/home assignment and peg placement (GAT20STACROSTA-006).
- Step/hop legality (GAT20STACROSTA-007/008) — only the static topology + adjacency lookup here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --lib` (topology invariant tests)
2. `cargo fmt --check`
3. `bash scripts/boundary-check.sh`

### Invariants

1. Exactly 121 spaces with stable deterministic ordering.
2. Neighbor relation is symmetric; coordinates satisfy `q+r+s==0`; manifest is typed content only (§5).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/src/topology.rs` — inline tests: 121-count, neighbor symmetry, degree range, coord invariant, opposite-home involution.
2. `games/starbridge_crossing/tests/property.rs` — (stub created here; extended in 011) topology-order determinism property.

### Commands

1. `cargo test -p starbridge_crossing --lib`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. The `--lib` boundary is correct for topology invariants; full crate test confirms no module regressions.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/src/topology.rs` with game-local star-board
  topology types, deterministic 121-space generation, cube coordinates,
  stable id-to-coordinate lookup, six-direction neighbor lookup, home-zone
  grouping, opposite-home checks, UI anchors, and manifest validation.
- Added `games/starbridge_crossing/data/manifest.toml` as an inert typed
  topology receipt with game id, generator id, space count, coordinate system,
  point order, and version labels.
- Updated `games/starbridge_crossing/src/lib.rs` to expose the topology module
  and public topology accessors.
- Added `games/starbridge_crossing/tests/property.rs` with a deterministic
  stable-order check for the generated topology.

Deviations from plan:

- Used a deterministic Rust generator identified by
  `topology_generator = "cube_star_order_4_v1"` instead of hand-authored
  per-space manifest rows. The manifest remains typed static content and
  rejects unknown or behavior-looking fields; Rust owns coordinate, zone, and
  neighbor derivation so no legality, branch, selector, or trigger behavior is
  encoded in data.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p starbridge_crossing --lib` passed: 10 unit tests.
- `cargo test -p starbridge_crossing` passed: 10 unit tests, 1 integration
  test, 0 doctests.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
