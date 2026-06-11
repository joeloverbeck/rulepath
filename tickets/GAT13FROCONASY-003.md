# GAT13FROCONASY-003: Crate skeleton, workspace registration, and static data

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/frontier_control` (skeleton modules + `Cargo.toml`); root `Cargo.toml` workspace member; new typed static data (`data/manifest.toml`, `data/variants.toml`, two fixtures)
**Deps**: GAT13FROCONASY-002

## Problem

Frontier Control needs its crate scaffold and typed static map data before any behavior lands. The two maps (`frontier_control_standard`, `frontier_control_highlands`) must be typed content — site IDs, labels, edge pairs, fort flags, stake values, start units, caps, budgets, round counts, faction labels — with **no** behavior fields, proving the map is content not behavior (FOUNDATIONS §5). This is the gate's highest-risk data boundary.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch` is the file-for-file template: `src/{actions,bots,effects,ids,lib,replay_support,rules,setup,state,ui,variants,visibility}.rs` (all 12 verified present), `data/{manifest,variants}.toml` + `data/fixtures/`, with variant IDs `flood_watch_standard`/`flood_watch_deluge` (verified `crates/wasm-api/src/lib.rs:171-172` and `games/flood_watch/data/variants.toml`). Frontier mirrors this with variant IDs `frontier_control_standard`/`frontier_control_highlands` (full-game-id prefix, per the reassessed convention).
2. Root `Cargo.toml` registers game crates as a `members` list (verified `"games/flood_watch",` present); spec §Deliverables names the manifest/variants/fixture targets.
3. Cross-crate boundary under audit: the static-data schema (`manifest.toml`/`variants.toml`/`*.fixture.json`) is validated by `tools/fixture-check` and the crate's own serialization tests; field set authored here must reject unknown and behavior-looking fields (`when`/`condition`/`trigger`/etc.).
4. FOUNDATIONS §5 (static data is typed content, not behavior) is the principle under audit: edges are ID pairs, forts are flags, stake values are integers; nothing in data says what movement, clashes, or scoring *do*.
5. Schema introduced: a new static-data manifest/variant/fixture entry set is added (additive — a brand-new game's data, no existing consumer); `fixture-check` registration (GAT13FROCONASY-013) is the consumer and is additive-only.

## Architecture Check

1. Establishing the skeleton + data first gives every later pipeline ticket a compiling crate and validated map to build on; mirroring flood_watch's module split keeps the codebase uniform and reviewable.
2. No backwards-compatibility aliasing/shims — new crate.
3. `engine-core` is untouched (the new crate depends on existing generic contracts only); no `game-stdlib` promotion. Map nouns (site/edge/fort/stake) stay local to `games/frontier_control`.

## Verification Layers

1. Workspace wiring -> `cargo build -p frontier_control` (crate compiles and is a workspace member).
2. Static-data typed-content invariant (§5) -> schema/serialization validation (manifest/variants/fixtures parse into typed structs; unknown + behavior-looking fields rejected — asserted by the serialization tests authored in GAT13FROCONASY-009, with skeleton parsers here).
3. Boundary cleanliness -> `bash scripts/boundary-check.sh` (no new mechanic noun reaches `engine-core`).

## What to Change

### 1. Crate skeleton + workspace registration

Create `games/frontier_control/Cargo.toml` and the twelve `src/*.rs` module stubs mirroring `games/flood_watch`. Add `"games/frontier_control",` to the root `Cargo.toml` members list.

### 2. Typed static data

Author `data/manifest.toml`, `data/variants.toml` (both variants: seven-site standard per A3; highlands with a different graph/starts/values/round count), and `data/fixtures/frontier_control_standard.fixture.json` + `data/fixtures/frontier_control_highlands.fixture.json`. Fields: typed metadata, site IDs/labels, edge lists (ID pairs), fort flags, per-site stake values, start units, caps, budgets, round counts, faction labels. No selectors/conditions/triggers/expressions.

### 3. Parser stubs

Skeleton manifest/variant/fixture parsers in `setup.rs`/`variants.rs` that reject unknown and behavior-looking fields (full validation logic lands in GAT13FROCONASY-004).

## Files to Touch

- `games/frontier_control/Cargo.toml` (new)
- `games/frontier_control/src/actions.rs` (new)
- `games/frontier_control/src/bots.rs` (new)
- `games/frontier_control/src/effects.rs` (new)
- `games/frontier_control/src/ids.rs` (new)
- `games/frontier_control/src/lib.rs` (new)
- `games/frontier_control/src/replay_support.rs` (new)
- `games/frontier_control/src/rules.rs` (new)
- `games/frontier_control/src/setup.rs` (new)
- `games/frontier_control/src/state.rs` (new)
- `games/frontier_control/src/ui.rs` (new)
- `games/frontier_control/src/variants.rs` (new)
- `games/frontier_control/src/visibility.rs` (new)
- `games/frontier_control/data/manifest.toml` (new)
- `games/frontier_control/data/variants.toml` (new)
- `games/frontier_control/data/fixtures/frontier_control_standard.fixture.json` (new)
- `games/frontier_control/data/fixtures/frontier_control_highlands.fixture.json` (new)
- `Cargo.toml` (modify)

## Out of Scope

- Full state model, setup logic, and map-graph validation (GAT13FROCONASY-004).
- Actions, scoring, visibility, replay, bots (later tickets).
- Tool/WASM registration (GAT13FROCONASY-012/013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p frontier_control` succeeds; `cargo build --workspace` stays green.
2. `cargo fmt --all --check` and `cargo clippy -p frontier_control --all-targets -- -D warnings` pass.
3. `bash scripts/boundary-check.sh` passes (no mechanic noun in `engine-core`).

### Invariants

1. Static data carries only typed content/metadata; no behavior-looking field is accepted by the parser stubs.
2. Variant IDs and fixture filenames use the `frontier_control_<variant>` convention.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/src/setup.rs` — parser-stub unit tests asserting unknown/behavior-looking fields are rejected (expanded in GAT13FROCONASY-009).

### Commands

1. `cargo build -p frontier_control`
2. `cargo test -p frontier_control` (skeleton-level parser tests) and `cargo build --workspace`
3. Building the single crate plus workspace is the correct boundary; full data-driven validation is exercised once setup logic lands in GAT13FROCONASY-004.
