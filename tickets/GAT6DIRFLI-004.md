# GAT6DIRFLI-004: Crate skeleton & workspace wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new crate `games/directional_flip` (`Cargo.toml`, `src/lib.rs`, `src/ids.rs`, `src/state.rs`, `src/setup.rs`, `src/variants.rs`, `data/*`); workspace registration in root `Cargo.toml`.
**Deps**: 002

## Problem

Before rules, effects, bots, and views can be implemented, `directional_flip` needs a compiling crate skeleton: stable identifiers, typed board state, the standard setup constructor, the single fixed variant, inert static data, and workspace registration. This mirrors the `games/column_four` skeleton (GAT5COLFOUPUB-002 precedent). It is gated on GAT6DIRFLI-002 because the primitive-pressure decision (§4 hard gate) must be recorded before deep implementation begins (spec §10, ROADMAP Gate 6).

## Assumption Reassessment (2026-06-07)

1. The sibling `games/column_four/` provides the exact skeleton shape: `src/{lib,ids,state,setup,variants}.rs`, `data/{manifest.toml,variants.toml}`, `data/fixtures/column_four_standard.fixture.json`. The cell-id convention is `r1c1…r8c8`, row 1 top — matching `games/three_marks/src/ids.rs` (enum `R1C1` → `"r1c1"`); seats use `seat_0`/`seat_1` per `games/column_four/src/ids.rs` (`ColumnFourSeat::as_str` → `"seat_0"`). `games/directional_flip/` does not exist yet.
2. Root `Cargo.toml` lists workspace members explicitly (lines 3–17 include `games/race_to_n`, `games/three_marks`, `games/column_four`); `games/directional_flip` must be added there. Spec §8.1 and §20 (assumptions: cell-id, setup cells, seat conventions) are authoritative.
3. Cross-artifact boundary under audit: the workspace `members` list and the engine-core contract types (`engine-core` seat/match/action vocabulary the game state references). Adding a member must not change build order semantics for existing crates; the skeleton depends on `engine-core` only via its generic contracts.
4. FOUNDATIONS §5 (static data is not behavior) motivates the data files: restate before authoring — `data/manifest.toml` and `data/variants.toml` may carry ids, names, dimensions, labels, and setup constants only; no selectors, conditions, triggers, loops, or rule DSL. The fixture JSON is typed content/fixture data, strict-parsed (unknown fields rejected), consistent with `docs/TRACE-SCHEMA-v1.md` and `tools/fixture-check`.

## Architecture Check

1. A thin skeleton (ids + state + setup + variant + data) compiled and registered first gives every downstream ticket a stable target and keeps each later diff (rules, effects, bots) reviewable in isolation — the same staging `column_four` used.
2. No backwards-compatibility shims; this is a new crate.
3. `engine-core` is untouched — the game state lives entirely in `games/directional_flip` and references only generic engine-core contracts; no mechanic noun (board/cell/flip) enters the kernel (§3). `game-stdlib` is consumed only if GAT6DIRFLI-003 promoted a helper; otherwise the skeleton is self-contained.

## Verification Layers

1. Crate compiles & is registered -> codebase grep-proof + build: `games/directional_flip` appears in root `Cargo.toml` members and `cargo build -p directional_flip` succeeds.
2. Static-data inertness -> schema/serialization validation: `data/*.toml` and the fixture parse strictly (unknown/behavior-looking fields rejected), consistent with `tools/fixture-check` expectations.
3. Setup correctness -> codebase grep-proof / unit assertion: the standard setup places first seat at `r4c5`/`r5c4`, second at `r4c4`/`r5c5`, active = first seat (spec §6.2). Full setup tests land in GAT6DIRFLI-012.
4. Kernel boundary -> FOUNDATIONS alignment check (§3): `bash scripts/boundary-check.sh` passes.

## What to Change

### 1. Crate manifest & lib

`games/directional_flip/Cargo.toml` (depend on `engine-core`, and `game-stdlib` only if GAT6DIRFLI-003 promoted a helper) and `src/lib.rs` (module wiring, public crate API surface mirroring `column_four`).

### 2. Identifiers & state

`src/ids.rs` (seat ids `seat_0`/`seat_1`, cell ids `r1c1…r8c8` with parse/format/display, action-segment ids), `src/state.rs` (typed 8×8 occupancy, active seat, freshness token, consecutive forced-pass count, terminal outcome, stable summaries).

### 3. Setup & variant

`src/setup.rs` (standard constructor + fixture loading) and `src/variants.rs` (single `directional_flip_standard` id + non-behavioral metadata).

### 4. Static data

`data/manifest.toml`, `data/variants.toml`, `data/fixtures/directional_flip_standard.fixture.json` — inert typed content only.

### 5. Workspace registration

Add `games/directional_flip` to root `Cargo.toml` `members`.

## Files to Touch

- `games/directional_flip/Cargo.toml` (new)
- `games/directional_flip/src/lib.rs` (new)
- `games/directional_flip/src/ids.rs` (new)
- `games/directional_flip/src/state.rs` (new)
- `games/directional_flip/src/setup.rs` (new)
- `games/directional_flip/src/variants.rs` (new)
- `games/directional_flip/data/manifest.toml` (new)
- `games/directional_flip/data/variants.toml` (new)
- `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` (new)
- `Cargo.toml` (modify — add workspace member)

## Out of Scope

- Legality, flip resolution, forced pass, terminal, scoring (GAT6DIRFLI-005).
- Action tree / previews (006), effects (008), views (007), bots (011).
- `wasm-api` registration (GAT6DIRFLI-015) and tool registration (GAT6DIRFLI-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p directional_flip` — crate compiles.
2. `grep -q 'games/directional_flip' Cargo.toml` — workspace member registered.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Static data carries no behavior-looking fields (FOUNDATIONS §5).
2. No mechanic noun enters `engine-core` (FOUNDATIONS §3).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` — standard-setup fixture (consumed by setup + fixture-check); full assertions in GAT6DIRFLI-012/016.

### Commands

1. `cargo build -p directional_flip`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. A build + boundary check is the correct boundary here; behavioral tests arrive with the rules engine (GAT6DIRFLI-005/012).
