# GAT1RACTON-004: race_to_n crate foundation — ids, state, setup, variant, static data

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/race_to_n` (depends on `engine-core` + `ai-core` traits), workspace member registration, typed state/ids/setup, single declared variant, and typed static data (`manifest.toml`, `variants.toml`, `fixtures/`).
**Deps**: GAT1RACTON-001, GAT1RACTON-003

## Problem

The `race_to_n` game crate needs its foundation before rules: the module
skeleton (ARCHITECTURE §11), typed ids and internal state, deterministic setup,
the single pinned variant, and typed static data carrying metadata + variant
selection only (FOUNDATIONS §5). This crate implements the generic game-entry
contract from GAT1RACTON-003 and is the substrate every later `race_to_n` ticket
builds on.

## Assumption Reassessment (2026-06-05)

1. `games/` contains only `.gitkeep` (verified `ls -A games/`); the workspace
   `Cargo.toml` members list ends at `tools/fixture-check` and does **not**
   include `games/race_to_n` (verified `cat Cargo.toml`). This ticket creates the
   crate and adds it to `members`.
2. The variant + win condition is pinned by GAT1RACTON-001's `RULES.md`
   (`games/race_to_n/docs/RULES.md`); setup/state here MUST match it. The
   recommended module shape is `docs/ARCHITECTURE.md` §11
   (`lib.rs ids.rs state.rs setup.rs ...`).
3. Cross-crate boundary under audit: this crate's dependency edges must be
   exactly `games/race_to_n -> engine-core + ai-core traits` (ARCHITECTURE §2);
   it MUST NOT depend on `engine-core` internals or any sibling game. It
   implements `engine_core::Game` (GAT1RACTON-003) over its own opaque payload.
4. FOUNDATIONS §5 (static data is typed content, not behavior) motivates the data
   deliverable: `manifest.toml`/`variants.toml` carry typed metadata + variant
   selection only — no selectors, conditions, branches, or triggers. Unknown
   fields are rejected by default (FOUNDATIONS §11; AGENT-DISCIPLINE §6 — TOML for
   manifests, reject unknown fields, flag behavior-looking keys like
   `when`/`if`/`selector`/`trigger`).
5. Static-data enforcement surface: the manifest/variant deserialization is the
   first place the FOUNDATIONS §11 unknown-field-rejection invariant is enforced
   in a game. Confirm deserialization is fail-closed (deny-unknown-fields) and
   contains no behavior-looking fields; the broader serialization/round-trip
   enforcement is GAT1RACTON-006. The state model introduces no hidden
   information (perfect-information game) — no leakage path for later view
   projection to undo.
6. Schema/contract: this crate consumes the `engine-core` game-entry +
   serialization contracts (additive use, no modification of the kernel). Static
   data is a new typed manifest entry per `docs/ENGINE-GAME-DATA-BOUNDARY.md`
   conventions; no existing manifest schema is altered.

## Architecture Check

1. Separating internal `state` (mutated by rules) from later viewer-safe views
   (GAT1RACTON-006) follows ARCHITECTURE §6 (internal state and views are
   different types). Typed setup keyed off `Seed` + seats + options keeps setup
   deterministic (ARCHITECTURE §4). Alternative (untyped/dynamic state) breaks
   determinism and type-safety.
2. No backwards-compatibility shims — new crate.
3. `engine-core` untouched here (it was extended in 002/003); game-specific nouns
   live correctly inside this game module (FOUNDATIONS §3 — "a game-specific type
   inside a game module is correct"). `game-stdlib` untouched (first use is
   local-only, FOUNDATIONS §4).

## Verification Layers

1. Dependency-edge correctness -> codebase grep-proof (`games/race_to_n/Cargo.toml`
   depends only on `engine-core` + `ai-core`; `scripts/boundary-check.sh` passes).
2. Static-data discipline -> schema/serialization validation (manifest/variants
   deserialize fail-closed; unknown-field rejection unit test; no behavior-looking
   keys, FOUNDATIONS §5/§11).
3. Deterministic setup -> deterministic replay-hash check (same seed+seats+options
   → identical initial state, unit test).
4. Cross-crate: crate compiles against the `engine_core::Game` contract (build
   proof) — the trait impl is completed incrementally by 005/006.

## What to Change

### 1. Crate skeleton + workspace registration

Create `games/race_to_n/Cargo.toml` (deps: `engine-core`, `ai-core`) and add
`"games/race_to_n"` to the workspace `members` in root `Cargo.toml`. Create
`src/lib.rs` re-exporting the modules.

### 2. ids + state + setup + variant modules

`src/ids.rs` (typed ids), `src/state.rs` (internal state for the pinned variant —
the counter/heap and turn marker), `src/setup.rs` (deterministic setup from
`Seed`/seats/options), `src/variants.rs` (the single declared variant). Begin the
`engine_core::Game` impl in `lib.rs` (setup wired; rules/effects/views land in
005/006).

### 3. Typed static data

`data/manifest.toml` (typed metadata — display name, neutral component names,
rules/data version), `data/variants.toml` (typed variant selection), `data/fixtures/`
(seed dir for golden-trace/serialization fixtures used by 008). Deserialize with
deny-unknown-fields.

## Files to Touch

- `Cargo.toml` (modify) — add `games/race_to_n` to `members`
- `games/race_to_n/Cargo.toml` (new)
- `games/race_to_n/src/lib.rs` (new)
- `games/race_to_n/src/ids.rs` (new)
- `games/race_to_n/src/state.rs` (new)
- `games/race_to_n/src/setup.rs` (new)
- `games/race_to_n/src/variants.rs` (new)
- `games/race_to_n/data/manifest.toml` (new)
- `games/race_to_n/data/variants.toml` (new)
- `games/race_to_n/data/fixtures/.gitkeep` (new)

## Out of Scope

- Legal action generation, validation, transitions, terminal detection, effects
  (GAT1RACTON-005).
- View projection + serialization round-trips (GAT1RACTON-006).
- Bot, simulation, benchmarks, wasm, docs-finalize (007/009/010/011/014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p race_to_n` — crate compiles and is a workspace member.
2. `cargo test -p race_to_n` — setup determinism + manifest/variants deny-unknown-fields unit tests pass.
3. `bash scripts/boundary-check.sh` — dependency-edge boundary check passes.

### Invariants

1. `games/race_to_n` depends only on `engine-core` + `ai-core` (ARCHITECTURE §2).
2. Static data carries no behavior-looking fields and rejects unknown fields (FOUNDATIONS §5/§11).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/src/setup.rs` `#[cfg(test)]` — identical seed/seats/options → identical initial state.
2. `games/race_to_n/src/lib.rs` `#[cfg(test)]` — `manifest.toml`/`variants.toml` parse; an unknown field is rejected.

### Commands

1. `cargo test -p race_to_n`
2. `cargo build --workspace`
3. `grep -nE 'when|if|then|selector|condition|trigger|script' games/race_to_n/data/*.toml` — expect zero behavior-looking keys (static-data discipline proof).
