# GAT20STACROSTA-005: Typed board topology content (121-space six-pointed star)

**Status**: PENDING
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
