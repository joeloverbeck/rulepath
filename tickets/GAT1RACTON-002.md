# GAT1RACTON-002: engine-core action-tree, freshness-token & deterministic RNG contracts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/engine-core` gains generic, noun-free contracts: action tree / action node / action choice (extending the existing `ActionPath`), a freshness-token version marker, and the deterministic RNG contract.
**Deps**: GAT1RACTON-001

## Problem

`race_to_n` needs a decision-point contract surface in the kernel: a legal
**action tree** the game produces and the UI presents, a **freshness token** to
reject stale UI submissions gracefully (ARCHITECTURE §5), and a **deterministic
RNG contract** all randomness passes through (ARCHITECTURE §9, FOUNDATIONS §2).
These are generic infrastructure contracts FOUNDATIONS §3 already authorizes
(`action tree`, `action path`, deterministic RNG contract) plus the freshness
token named in ARCHITECTURE §5. They must enter `engine-core` noun-free.

## Assumption Reassessment (2026-06-05)

1. `crates/engine-core/src/lib.rs` currently defines `GameId`, `MatchId`,
   `SeatId`, `PlayerId`, `RulesVersion`, `SchemaVersion`, `Seed`,
   `VisibilityScope`, `Actor`, `Viewer`, `ActionPath` (`segments: Vec<String>`),
   `CommandEnvelope`, `Diagnostic`, `EffectEnvelope<T>` (verified
   `grep -nE 'pub struct|pub enum' crates/engine-core/src/lib.rs`). No
   `ActionTree`, `ActionNode`, `ActionChoice`, `FreshnessToken`, or RNG contract
   exists yet — all additive.
2. The spec's WB2 (corrected by `/reassess-spec` this session) enumerates the
   freshness-token contract as a generic version marker per ARCHITECTURE §5
   (`specs/gate-1-race-to-n.md` §2 in-scope + WB2). ARCHITECTURE §5:104 lists
   *Freshness token* with authority Rust.
3. Shared boundary under audit: `engine-core` is the contract kernel consumed by
   `ai-core`, `games/*`, and `wasm-api` (ARCHITECTURE §2). Every type added here
   must be generic — it may know an opaque game-defined payload exists but not
   what it means mechanically (FOUNDATIONS §3 decision rule).
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) and §2 (deterministic
   randomness lives in Rust) motivate this ticket. The kernel-change protocol
   (AGENT-DISCIPLINE §5) is answered: (a) `race_to_n` requires it; (b) it cannot
   live in `games/*` because every game and the UI share the action-tree/RNG
   contract; (c) it is not a `game-stdlib` helper because it is a contract, not a
   mechanic; (d) it introduces no game/mechanic/strategy/renderer/network/storage
   noun; (e) it preserves determinism (the RNG contract IS the determinism
   surface); (f) no ADR — it instantiates already-authorized §3 vocabulary, not a
   vocabulary change.
5. Determinism enforcement surface: the deterministic RNG contract is the surface
   later replay/hash determinism (GAT1RACTON-008) and bot determinism
   (GAT1RACTON-007) depend on. Confirm it exposes no wall-clock/OS-entropy seeding
   path in canonical forms (ARCHITECTURE §9; FOUNDATIONS §11) — seeding is via the
   existing `Seed(u64)` only. The freshness token is a substrate for the
   stale-rejection enforcement that GAT1RACTON-005's validation implements; it
   introduces no leakage path (it is a version integer, viewer-independent).

## Architecture Check

1. Modeling the action tree as a generic structure of nodes/choices with stable
   path segments (matching the existing `ActionPath { segments: Vec<String> }`)
   keeps the kernel presentation-agnostic — the UI builds controls from it
   without the kernel knowing any mechanic. Alternative (game-specific action
   enums in the kernel) is a §3 boundary failure.
2. No backwards-compatibility aliasing — `ActionPath` is extended in place by
   sibling types, not duplicated.
3. `engine-core` stays noun-free: `ActionTree`/`ActionNode`/`ActionChoice`/
   `FreshnessToken`/RNG are generic contract nouns from FOUNDATIONS §3's allowed
   list; no `pile`/`board`/`deck`/`track` enters the kernel. `game-stdlib`
   untouched.

## Verification Layers

1. Kernel noun-freeness -> codebase grep-proof (`scripts/boundary-check.sh` and
   a grep over `crates/engine-core/src` for mechanic nouns returns clean).
2. Action-tree/path conformance -> schema/serialization validation (types match
   ARCHITECTURE §5 action-tree/action-path/freshness-token vocabulary).
3. RNG determinism -> deterministic replay-hash check (a unit test: same `Seed`
   yields identical draw sequence; no `std::time`/OS entropy in canonical path,
   ARCHITECTURE §9).
4. Cross-crate contract: action-tree types are consumed by `games/*` and
   `wasm-api`; RNG by `ai-core` and `games/*` — each mapped above to its own
   proof surface.

## What to Change

### 1. Action-tree contract

Add generic `ActionTree`, `ActionNode`, and `ActionChoice` types to
`engine-core` (in `lib.rs` or a new `action.rs` module re-exported from `lib.rs`).
A choice carries a stable ID/path segment, label/metadata, accessibility text,
tags, and an optional preview hook (ARCHITECTURE §5). Flat trees are expressible
(simple games MAY expose flat action trees, ARCHITECTURE §5). Reuse the existing
`ActionPath` as the selected route.

### 2. Freshness-token contract

Add a generic `FreshnessToken` (a version marker — e.g. newtype over a monotonic
counter/version integer) per ARCHITECTURE §5. It is viewer-independent and
carries no game state. Extend `CommandEnvelope`/validation inputs conceptually so
GAT1RACTON-005 can reject stale submissions; the token type lives here.

### 3. Deterministic RNG contract

Add the deterministic RNG contract (a trait + a concrete deterministic generator
seeded from `Seed`) per ARCHITECTURE §9 / FOUNDATIONS §2. No wall-clock or OS
entropy. This is the single randomness surface for bots and any game randomness.

### 4. Module organization

If splitting into modules (`action.rs`, `rng.rs`), re-export from `lib.rs` so the
public path stays `engine_core::ActionTree` etc.

## Files to Touch

- `crates/engine-core/src/lib.rs` (modify)
- `crates/engine-core/src/action.rs` (new, optional — module split)
- `crates/engine-core/src/rng.rs` (new, optional — module split)

## Out of Scope

- Effect cursor/log, hash contract, game-entry trait, replay/command-stream +
  serialization boundary (GAT1RACTON-003).
- Any `race_to_n`-specific type (GAT1RACTON-004+).
- Wiring the RNG into a bot (GAT1RACTON-007).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` — new unit tests for action-tree construction, freshness-token equality/ordering, and RNG determinism pass.
2. `cargo clippy -p engine-core --all-targets -- -D warnings` — clean.
3. `bash scripts/boundary-check.sh` — `engine-core` boundary check passes (no mechanic nouns).

### Invariants

1. `engine-core` contains only generic contract nouns (FOUNDATIONS §3, §11).
2. Identical `Seed` produces an identical RNG draw sequence (deterministic; ARCHITECTURE §9).

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/lib.rs` (or `rng.rs`) `#[cfg(test)]` — RNG determinism: same seed → same sequence.
2. `crates/engine-core/src/lib.rs` (or `action.rs`) `#[cfg(test)]` — action tree builds flat choices with stable path segments; freshness token compares by version.

### Commands

1. `cargo test -p engine-core`
2. `cargo build --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. `grep -rniE 'board|deck|pile|card|track|resource|suit|hand' crates/engine-core/src` — expect zero mechanic-noun hits (kernel-boundary proof).
