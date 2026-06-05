# GAT0REPSKE-001: Workspace manifest and four placeholder crates

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — introduces the root workspace `Cargo.toml` and four placeholder crates: `crates/engine-core`, `crates/game-stdlib`, `crates/ai-core`, `crates/wasm-api`.
**Deps**: None

## Problem

The Rulepath repository has no Rust workspace yet — `Cargo.toml` and `crates/` are absent. Later gates (Gate 1 `race_to_n` onward) need a place to land. This ticket stands up the workspace manifest and the four foundational crates **with the correct dependency direction**, so the kernel boundary (`engine-core` depends on no other Rulepath crate) is established structurally from the first commit rather than retrofitted later.

## Assumption Reassessment (2026-06-05)

1. Code tree is greenfield — `Cargo.toml` and `crates/` are absent (verified `test -e` 2026-06-05); no name collision. The `engine-core` allowed-vocabulary / forbidden-noun lists are taken from `docs/ENGINE-GAME-DATA-BOUNDARY.md` §3.
2. `specs/gate-0-repository-skeleton.md` §3 + `docs/ARCHITECTURE.md` §1 define the four crates and the target tree; `docs/ARCHITECTURE.md` §2 defines dependency direction.
3. Cross-crate boundary under audit: the workspace dependency graph. Per `docs/ARCHITECTURE.md:54`, `engine-core` MUST NOT depend on `game-stdlib`, `ai-core`, `wasm-api`, `apps/web`, or `games/*`; `game-stdlib`/`ai-core`/`wasm-api` may depend on `engine-core` only.
4. §3 `engine-core` is a contract kernel: this is the gate that sets the kernel boundary, so `engine-core` declares only generic contract vocabulary and no mechanic noun. Per spec §3, empty marker traits/types are preferred over speculative contract surface.

## Architecture Check

1. Wiring dependency direction at crate-creation time is cleaner than retrofitting it — the boundary cannot drift if `engine-core` starts with zero dependency edges.
2. No backwards-compatibility shims or alias paths — greenfield.
3. `engine-core` stays free of mechanic nouns (§3); `game-stdlib` ships empty with no promoted helper (§4); `ai-core` carries only a bot-trait stub.

## Verification Layers

1. `engine-core` noun-free -> codebase grep-proof (mechanic-noun grep over `crates/engine-core/src` returns 0).
2. Dependency direction correct -> schema/serialization validation (`cargo tree -p engine-core` shows no Rulepath-crate dependency).
3. Crates compile + smoke tests pass -> simulation/CLI run (`cargo test` over the workspace).

## What to Change

### 1. Root workspace manifest

Create `Cargo.toml` with `[workspace]`, `resolver = "2"`, and `members` listing the four crates.

### 2. `engine-core` crate

Minimal/empty generic contract markers only (no mechanic nouns). No dependency on any other Rulepath crate.

### 3. `game-stdlib`, `ai-core`, `wasm-api` placeholder crates

Placeholder libs that may depend on `engine-core` only. `ai-core` carries a bot-trait stub; `game-stdlib` is empty; `wasm-api` is a compiling placeholder (its WASM build surface lands in GAT0REPSKE-002). Each crate gets a trivial smoke test.

## Files to Touch

- `Cargo.toml` (new)
- `crates/engine-core/Cargo.toml` (new)
- `crates/engine-core/src/lib.rs` (new)
- `crates/game-stdlib/Cargo.toml` (new)
- `crates/game-stdlib/src/lib.rs` (new)
- `crates/ai-core/Cargo.toml` (new)
- `crates/ai-core/src/lib.rs` (new)
- `crates/wasm-api/Cargo.toml` (new)
- `crates/wasm-api/src/lib.rs` (new)

## Out of Scope

- Any real mechanic or game noun in `engine-core` (spec §8).
- Populating `game-stdlib` with any helper (spec §8).
- The WASM artifact build and web shell (GAT0REPSKE-002).
- `tools/`, `benches/`, `games/` (GAT0REPSKE-003).
- YAML, DSL, networking, accounts, persistence (spec §2 not-allowed).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build` succeeds over the workspace.
2. `cargo test` passes (each crate's smoke test).
3. `cargo tree -p engine-core` lists no Rulepath crate (`game-stdlib`/`ai-core`/`wasm-api`/`games/*`/`apps/web`).

### Invariants

1. `engine-core` declares no mechanic noun (`board`, `card`, `deck`, `grid`, `suit`, `resource`, `capture`, `hand`, `pile`, `trick`, `pot`, …).
2. `engine-core` has zero intra-workspace dependency edges (ARCHITECTURE.md §2).

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/lib.rs` (+ the three sibling crates) — one trivial smoke test per crate proving it compiles and links.

### Commands

1. `cargo tree -p engine-core` — dependency-direction proof.
2. `cargo test` — full workspace build + smoke tests.
3. `grep -rniE "board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting" crates/engine-core/src` — noun-free proof (expect 0 matches).

## Outcome

Completed: 2026-06-05

What changed:
- Added the root Rust workspace manifest with the four Gate 0 placeholder crate members.
- Added `engine-core` with generic contract-only placeholder types and no Rulepath crate dependencies.
- Added empty/linkage placeholder crates for `game-stdlib`, `ai-core`, and `wasm-api`; each has a trivial smoke test.

Deviations from original plan:
- None.

Verification results:
- `cargo build` passed.
- `cargo test` passed.
- `cargo tree -p engine-core` showed only `engine-core` with no Rulepath dependencies.
- `rg -n -i "board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting" crates/engine-core/src` returned no matches.
