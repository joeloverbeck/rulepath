# GAT6DIRFLI-005: Rules core — directional scan, legality, flip, forced pass, terminal, scoring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/directional_flip/src/rules.rs` (validation, apply, directional scan/flip collection, forced pass, terminal detection, scoring, diagnostics).
**Deps**: 004

## Problem

This is the behavior core of `directional_flip`: Rust-owned placement legality (bracketing in any of eight directions), flip resolution (all bracketed discs in every qualifying direction, deterministic order), explicit forced pass (only when no legal placement exists), terminal detection (double forced pass, or no continuation), and disc-count scoring with draws. Per FOUNDATIONS §2, all of this is Rust; per §3, no mechanic noun leaks into `engine-core`. This realizes spec §6.3–§6.6 and rule ids `DF-LEGAL-*`, `DF-FLIP-*`, `DF-PASS-*`, `DF-TERM-*`, `DF-SCORE-*`.

## Assumption Reassessment (2026-06-07)

1. The crate skeleton (`games/directional_flip/src/{ids,state,setup}.rs`) exists from GAT6DIRFLI-004 with cell ids `r1c1…r8c8`, seats `seat_0`/`seat_1`, freshness token, and consecutive-forced-pass counter in state. `games/column_four/src/rules.rs` is the structural precedent for a Rust rules module (validation + apply + terminal + diagnostics).
2. Spec §6.3 (legality conditions), §6.4 (flip resolution + deterministic order: direction order N, NE, E, SE, S, SW, W, NW; nearest-to-farthest within a direction), §6.5 (forced pass table), and §6.6 (terminal + scoring) are authoritative. Rule ids come from `games/directional_flip/docs/RULES.md` (GAT6DIRFLI-001).
3. Cross-crate boundary under audit: `games/directional_flip` ↔ `engine-core` diagnostic/command contracts (validation must produce engine-core diagnostics distinguishing occupied / out-of-bounds / non-flipping / stale / wrong-actor / terminal, per spec §6.3). If GAT6DIRFLI-002 promoted a `game-stdlib` ray/coordinate helper, the directional scan consumes it; otherwise the scan is local. Confirm which before coding (read the ledger decision).
4. FOUNDATIONS §2 behavior authority motivates this ticket: restate before coding — setup, legal-action generation, validation, state transitions, scoring, and terminal detection are Rust-owned; nothing here may be deferred to TypeScript. The forced-pass action is Rust-generated and is the only legal action when no placement exists (spec §6.5), never synthesized client-side.
5. This ticket establishes the canonical **flip ordering** that replay/hash and effects depend on (deterministic replay/hash surface, FOUNDATIONS §11). The flip set and its order must be a pure function of state + placement (no RNG, no wall-clock, no hash-map iteration order); the same ordered flip set is later previewed (006) and emitted as effects (008) and must be byte-stable across runs. No hidden information is read (perfect-information game; legality uses only board state).

## Architecture Check

1. Centralizing scan/flip/terminal/scoring in `rules.rs` (state in `state.rs`, identifiers in `ids.rs`) keeps the legality engine independently testable and gives previews (006) and effects (008) a single authoritative flip-collection function to call — avoiding the preview/apply divergence the spec's `DF-PREVIEW-001` invariant forbids.
2. No backwards-compatibility shims; this is new logic.
3. `engine-core` stays noun-free — all `board`/`cell`/`flip`/`direction` vocabulary is confined to `games/directional_flip` (§3); any shared coordinate/ray helper lives in `game-stdlib` per the recorded §4 decision, carrying no flip/capture/legality policy.

## Verification Layers

1. Legality invariant (bracketing required) -> rule test (`DF-LEGAL-001`..`006`): occupied / out-of-bounds / non-flipping / stale / wrong-actor / terminal each rejected with a distinct diagnostic.
2. Flip-resolution invariant -> rule + property test (`DF-FLIP-001`..`004`): all bracketed discs in every qualifying direction flip; no skipped own discs; no indirect/non-line flips; order is stable (N..NW, near→far).
3. Forced-pass invariant -> rule test (`DF-PASS-001`/`002`, `DF-ACTION-002`/`003`): forced pass legal only when no placement exists, absent otherwise; double pass terminalizes.
4. Terminal/scoring invariant -> rule test (`DF-TERM-001`, `DF-SCORE-001`/`002`): terminal tree has no choices; higher count wins; equal draws.
5. Determinism of flip order -> FOUNDATIONS alignment check (§11): flip collection is a pure function (no RNG/time/iteration-order dependence). (Full golden/replay proof in 009/013.)

## What to Change

### 1. Placement legality & directional scan

In `rules.rs`, implement candidate legality per spec §6.3 (not terminal, active actor, fresh token, in-bounds empty target, at least one direction with contiguous opposing run terminated by an own disc) using an eight-direction ray scan (local, or via the promoted `game-stdlib` helper).

### 2. Flip resolution & apply

Apply a legal placement per spec §6.4: place the disc, collect every qualifying direction's bracketed run, flip in deterministic order (N, NE, E, SE, S, SW, W, NW; nearest→farthest), update freshness, reset forced-pass count, advance active seat unless terminal.

### 3. Forced pass

Generate the explicit forced-pass transition per spec §6.5; increment the consecutive-forced-pass count; terminalize on the second consecutive pass.

### 4. Terminal & scoring

Terminal detection (double pass or no continuation) and disc-count scoring with draw per spec §6.6; emit distinct diagnostics for each illegal-placement case.

## Files to Touch

- `games/directional_flip/src/rules.rs` (new)
- `games/directional_flip/src/state.rs` (modify — any accessors/transition helpers the rules need)
- `games/directional_flip/src/lib.rs` (modify — export the rules module)

## Out of Scope

- Action-tree generation & placement previews (GAT6DIRFLI-006) — this ticket exposes the flip-collection function they call.
- Semantic effects (008), public view (007), replay (009), bots (011).
- Any TypeScript (legality is Rust-only; FOUNDATIONS §2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — rules-core unit tests (legality, flip, pass, terminal, scoring) pass. (Full suite in GAT6DIRFLI-012.)
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.
3. `cargo build -p directional_flip` — compiles within the workspace.

### Invariants

1. Legality, flips, forced pass, terminal, and scoring are computed entirely in Rust; no path defers to TypeScript (FOUNDATIONS §2).
2. The flip set/order is a deterministic pure function of (state, placement) (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/rules.rs` — legality, multi-direction flip, no-skip-own, no-indirect-flip, forced-pass-only-when-no-placement, double-pass terminal, count winner, draw (seeded inline here; expanded in GAT6DIRFLI-012).

### Commands

1. `cargo test -p directional_flip rules`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped tests are the correct boundary; cross-surface golden/replay proofs live in GAT6DIRFLI-009/013 once effects and replay exist.
