# GAT7DRALITCOM-004: Crate skeleton, state, setup, ids, variants, manifest, fixture & workspace wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/draughts_lite` (`Cargo.toml`, `src/lib.rs`, `src/state.rs`, `src/setup.rs`, `src/ids.rs`, `src/variants.rs`, `data/manifest.toml`, `data/variants.toml`, `data/fixtures/`), and root `Cargo.toml` workspace membership.
**Deps**: 002

## Problem

Every later Gate 7 ticket needs the crate to exist with a serializable, hash-stable state model, deterministic setup, stable piece ids, variant/manifest data, and workspace registration. This ticket creates `games/draughts_lite` and wires it into the workspace so rules (005), action tree (006), and the rest compile against a fixed state shape. It does not implement movement/capture/validation.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/` does not exist yet (verified absent); this ticket creates the directory. `games/directional_flip/{Cargo.toml,src/state.rs,src/setup.rs,src/ids.rs,src/variants.rs,data/manifest.toml,data/variants.toml}` are the structural precedents. The state model fields are fixed by spec §R11 "State requirements" (seats, active seat, board dims + variant identity, stable per-piece records {id, owner, kind, cell}, terminal outcome, ply/command count, freshness token, effects).
2. Spec §R8 (identity: `draughts_lite`, `draughts_lite_standard`, `draughts_lite-rules-v1`, data version `1`, seats `seat_0`/`seat_1`, `seat_0` first; board 8×8, parity `row+column` odd; setup 12 men rows 1–3 / 6–8) is authoritative. `crates/engine-core` supplies the generic `Seed`, `FreshnessToken`, and seat/actor contracts the state reuses (`crates/engine-core/src/{lib.rs,replay.rs}`).
3. Cross-artifact boundary under audit: the state struct + its serialization is the contract consumed by rules (005), view/visibility (009), replay (010), and WASM (016). Serialization order must be stable for deterministic hashing (spec §R10/§R11).
4. FOUNDATIONS §2/§11 (determinism) motivate this ticket: restate before coding — state must be serializable and hash-stable with a freshness token; stable piece ids are required so TypeScript never infers piece identity by diffing cells (spec §R8 setup). Setup is Rust-owned and deterministic.
5. Static-data schema conformance (`data/manifest.toml`, `data/variants.toml`, `data/fixtures/*.json`): these follow the existing static-data manifest schema (per `directional_flip/data/*` and `docs/ENGINE-GAME-DATA-BOUNDARY.md`) — typed content/metadata only, no selectors/triggers/behavior-looking fields (FOUNDATIONS §5). The standard fixture's full assertions land in GAT7DRALITCOM-014; this ticket creates the `data/fixtures/` directory and manifest/variant entries.

## Architecture Check

1. Fixing the state shape and stable piece ids up front (vs. growing them ad hoc inside rules) gives every downstream ticket one serialization contract and prevents a later piece-identity retrofit.
2. No backwards-compatibility shims; new crate.
3. `engine-core` stays noun-free (§3) — board/piece/cell types are game-local in `games/draughts_lite`; the crate reuses only generic kernel contracts (`Seed`, `FreshnessToken`, seat ids). `game-stdlib` is consumed only if GAT7DRALITCOM-002 promoted a helper (§4).

## Verification Layers

1. Workspace membership -> `cargo build -p draughts_lite`: the crate compiles and is a workspace member.
2. Deterministic setup -> unit test: standard setup places 12 men per side on playable cells in rows 1–3 / 6–8, rows 4–5 empty, `seat_0` active, non-terminal.
3. Stable serialization -> unit test: state serializes deterministically (stable field/collection order) and round-trips.
4. Kernel boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Crate scaffold & workspace wiring

Create `games/draughts_lite/Cargo.toml` (depending on `engine-core`, and `game-stdlib` iff GAT7DRALITCOM-002 promoted a helper) and `src/lib.rs`; add `games/draughts_lite` to the root `Cargo.toml` `members` list.

### 2. State, setup, ids, variants

`state.rs`: the hash-stable state struct per §R11. `ids.rs`: constants (game id, variant id, rules version, seat ids, board dimensions) and stable piece-id scheme. `setup.rs`: deterministic standard setup. `variants.rs`: the `draughts_lite_standard` variant.

### 3. Static data

`data/manifest.toml`, `data/variants.toml` following the `directional_flip` schema; create `data/fixtures/` (standard fixture content authored in GAT7DRALITCOM-014).

## Files to Touch

- `games/draughts_lite/Cargo.toml` (new)
- `games/draughts_lite/src/lib.rs` (new)
- `games/draughts_lite/src/state.rs` (new)
- `games/draughts_lite/src/setup.rs` (new)
- `games/draughts_lite/src/ids.rs` (new)
- `games/draughts_lite/src/variants.rs` (new)
- `games/draughts_lite/data/manifest.toml` (new)
- `games/draughts_lite/data/variants.toml` (new)
- `Cargo.toml` (modify — add `games/draughts_lite` to `members`)

## Out of Scope

- Legal move/capture generation, validation, apply, promotion, terminal logic (GAT7DRALITCOM-005).
- Action tree, effects, view, replay, bots (GAT7DRALITCOM-006/007/008/009/010/012).
- The standard fixture's hash-baseline assertions (GAT7DRALITCOM-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p draughts_lite` — compiles and is a workspace member.
2. `cargo test -p draughts_lite` — setup + serialization unit tests pass.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. State is serializable and hash-stable with stable piece ids and a freshness token (FOUNDATIONS §2/§11; spec §R11).
2. Static data is typed content only — no selectors/triggers/behavior fields (FOUNDATIONS §5).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/src/setup.rs` (inline tests) — standard setup piece counts/placement, active seat, non-terminal.
2. `games/draughts_lite/src/state.rs` (inline tests) — deterministic serialization round-trip.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. Crate-scoped tests plus a workspace build are the correct boundary; the standard fixture/hash baseline is validated in GAT7DRALITCOM-014.
