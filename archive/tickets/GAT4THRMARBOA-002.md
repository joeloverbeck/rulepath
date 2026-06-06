# GAT4THRMARBOA-002: Three Marks crate scaffold — identity, state, setup, variants, static data

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/three_marks` (`lib.rs`, `ids.rs`, `state.rs`, `setup.rs`, `variants.rs`), new static data (`data/manifest.toml`, `data/variants.toml`), workspace `Cargo.toml` member addition
**Deps**: GAT4THRMARBOA-001

## Problem

Gate 4 needs a self-contained, workspace-registered Rust crate for `three_marks` that owns game-local state (board occupancy, active seat, turn count, terminal outcome, winning line), deterministic setup for `three_marks_standard`, typed variant metadata, and stable cell ids — the foundation every later Three Marks ticket builds on. No `games/three_marks` crate exists yet.

## Assumption Reassessment (2026-06-06)

1. The crate layout mirrors `games/race_to_n/src/` (`lib.rs`, `ids.rs`, `state.rs`, `setup.rs`, `variants.rs`, plus `actions.rs`/`rules.rs`/`effects.rs`/`visibility.rs`/`bots.rs`/`replay_support.rs` landing in later tickets) and `games/race_to_n/data/` (`manifest.toml`, `variants.toml`, `fixtures/`). Verified those files exist; `race_to_n` exposes `load_manifest`/`load_variants` (referenced by `tools/fixture-check/src/main.rs:190-194`) — `three_marks` mirrors that public API.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §5.1 (file tree), §7.3 (static data is typed metadata, never rule behaviour), §8.1 (setup row), and §24 (cell ids `r1c1`..`r3c3`, first seat places first, rules version string `three_marks-rules-v1`). RULES.md/SOURCES.md from GAT4THRMARBOA-001 are the authoritative rule reference.
3. Cross-crate boundary under audit: the root workspace `Cargo.toml` `members` list, and the generic `engine-core` contracts the crate consumes (`game id`, `seat id`, `rules version`, `seed`, serialization/hash boundary — FOUNDATIONS §3). `three_marks` depends on `engine-core` exactly as `race_to_n` does (`games/race_to_n/Cargo.toml`).
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) and §5 (static data is typed content, not behaviour) motivate this ticket: board/cell/occupancy nouns are game-local and MUST NOT enter `engine-core`; `manifest.toml`/`variants.toml` may carry public name, variant id, rules version, seat count, and UI-copy identifiers but no selectors/branches/triggers.
5. Substrate for a deferred enforcement surface: `state.rs` defines the serialization/hash seed surfaces that GAT4THRMARBOA-007 (replay/hash) will hash and replay. Confirm here that state fields use stable, deterministically-ordered representations (no `HashMap` iteration order, no wall-clock) so the later deterministic replay/hash surface (§11/§13) has nothing to undo. No hidden state exists (perfect information), so there is no leak path to introduce.
6. New static-data manifest entries: `data/manifest.toml` and `data/variants.toml` extend the typed static-data manifest pattern `docs/ENGINE-GAME-DATA-BOUNDARY.md` governs. Consumers are `tools/fixture-check` (GAT4THRMARBOA-014) and `crates/wasm-api` catalog (GAT4THRMARBOA-009); the schema is game-local and additive (a second game's manifest, parsed by reject-unknown-field typed loaders mirroring `race_to_n`).

## Architecture Check

1. A dedicated game crate keeps all board/cell/occupancy vocabulary local (§3) and lets the workspace, WASM bridge, and tools depend on it the same way they depend on `race_to_n` — cleaner than threading a second game through a shared module. Alternative (generalizing `race_to_n` into a multi-game crate) is rejected: it would invent a premature abstraction with no third-use pressure (§4).
2. No backwards-compatibility aliasing/shims — new crate; `race_to_n` is untouched.
3. `engine-core` gains no board/grid/cell nouns (board occupancy lives only in `games/three_marks/src/state.rs`); no `game-stdlib` helper is extracted (first use, local-only per spec §17 and `docs/MECHANIC-ATLAS.md`).

## Verification Layers

1. Kernel-boundary invariant (`engine-core` noun-free) -> codebase grep-proof (`scripts/boundary-check.sh`; no board/cell/grid noun added to `crates/engine-core`).
2. Deterministic setup invariant -> simulation/unit check (`setup` produces an empty 3×3 board, active seat = first seat, turn count 0, non-terminal — unit test in `setup.rs`/`state.rs`).
3. Static-data-is-typed invariant -> schema/serialization validation (typed `manifest.toml`/`variants.toml` parse with reject-unknown-field loaders; no behaviour-looking fields) per `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
4. Workspace-membership invariant -> simulation/CLI run (`cargo build -p three_marks` succeeds; crate participates in `cargo build --workspace`).

## What to Change

### 1. Workspace registration

Add `"games/three_marks"` to the root `Cargo.toml` `members` list (after `games/race_to_n`).

### 2. `games/three_marks/Cargo.toml`

Mirror `games/race_to_n/Cargo.toml`: package name `three_marks`, workspace edition/license/repo, dependency on `engine-core` (and a typed-TOML dependency matching `race_to_n`). Bench/test `[[bench]]`/`[[bench]]`-style entries are added in their respective tickets (008).

### 3. `src/ids.rs`

Define stable cell ids `r1c1`..`r3c3` (3×3), seat ids/order, `game_id = "three_marks"`, `rules_version = "three_marks-rules-v1"`, variant id `three_marks_standard` — mirroring `games/race_to_n/src/ids.rs` conventions.

### 4. `src/state.rs`

Game-local state: per-cell occupancy (empty / owned-by-seat), active seat, turn/ply count, terminal outcome (none / win{seat, line cells} / draw), winning line. Deterministically-ordered, serialization-stable representation (no hash-map iteration order).

### 5. `src/setup.rs`

Deterministic initial state for `three_marks_standard`: empty board, active seat = documented first seat, rules version surface, freshness/hash seed consistent with `race_to_n` conventions. Include a unit test asserting the empty-board / first-seat / non-terminal invariant.

### 6. `src/variants.rs` + `src/lib.rs` + `data/`

`variants.rs`: typed `three_marks_standard` variant. `lib.rs`: crate root re-exporting the public surface and `load_manifest`/`load_variants` (mirror `race_to_n`). `data/manifest.toml`, `data/variants.toml`: typed metadata (public name, variant id, rules version, seat count, UI-copy identifiers, fixture metadata) — no rule behaviour. `data/fixtures/.gitkeep`.

## Files to Touch

- `Cargo.toml` (modify)
- `games/three_marks/Cargo.toml` (new)
- `games/three_marks/src/lib.rs` (new)
- `games/three_marks/src/ids.rs` (new)
- `games/three_marks/src/state.rs` (new)
- `games/three_marks/src/setup.rs` (new)
- `games/three_marks/src/variants.rs` (new)
- `games/three_marks/data/manifest.toml` (new)
- `games/three_marks/data/variants.toml` (new)
- `games/three_marks/data/fixtures/.gitkeep` (new)

## Out of Scope

- Action generation, validation, rules, win/draw detection (GAT4THRMARBOA-003).
- Effects, view projection, bots, replay, benchmarks (004–008).
- Any `engine-core`/`game-stdlib` change or helper extraction (forbidden, spec §4/§17).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p three_marks` — crate compiles and is a workspace member.
2. `cargo test -p three_marks setup` — setup unit test (empty board, first active seat, non-terminal, turn 0) passes.
3. `bash scripts/boundary-check.sh` — `engine-core` remains free of board/cell/grid/occupancy nouns.

### Invariants

1. All board/cell/occupancy vocabulary lives only under `games/three_marks/`; `crates/engine-core` is unchanged.
2. `manifest.toml`/`variants.toml` contain only typed metadata; loaders reject unknown fields (mirroring `race_to_n`).

## Test Plan

### New/Modified Tests

1. `games/three_marks/src/setup.rs` (inline `#[cfg(test)]`) — asserts deterministic initial state for `three_marks_standard`.
2. `Cargo.toml` workspace members — `three_marks` participates in `cargo build --workspace` / `cargo test --workspace`.

### Commands

1. `cargo build -p three_marks && cargo test -p three_marks`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. Full rule/property coverage is intentionally out of this ticket's boundary (lands in 003); the setup unit test is the correct verification depth for a scaffold diff.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/three_marks` as a workspace crate with `Cargo.toml`, `src/lib.rs`, `src/ids.rs`, `src/state.rs`, `src/setup.rs`, `src/variants.rs`, typed static data, and `data/fixtures/.gitkeep`.
- Registered `games/three_marks` in the root workspace.
- Added game-local identity constants, stable cell IDs `r1c1` through `r3c3`, two-seat identity, deterministic setup, empty-board state, terminal outcome scaffolding, a stable snapshot byte surface, and strict flat TOML loaders that reject unknown and behavior-looking fields.

Deviations from original plan:

- The scaffold intentionally does not implement the `engine_core::Game` trait yet because legal action generation, validation, effects, and view projection land in later tickets. No action/rules/effects modules were added in this ticket.
- Static data remains metadata-oriented; fixed rule meaning stays in Rust code.

Verification results:

- `cargo fmt --all --check`
- `cargo build -p three_marks`
- `cargo test -p three_marks setup`
- `cargo test -p three_marks`
- `cargo build --workspace`
- `bash scripts/boundary-check.sh`
