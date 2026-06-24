# 8CR4NSEAPRITRI-021: Briar Circuit C-07 pass-phase pairwise no-leak matrix

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/briar_circuit/tests/visibility.rs`; pass target/reveal/exchange policy unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Briar's fixed-four private-hand and pass-commitment/exchange geometry is not yet enumerated as a shared pairwise no-leak matrix across all four source seats × observer/all four viewers (MSC-8C, C-07). Enumerate it for pass selection, commitment, and atomic exchange, pinning owner-only cards and non-owner absence, with pass target/reveal/atomic-exchange policy kept local (spec §3.8 Briar, §5.8).

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/tests/visibility.rs` exists; `visibility::{project_pass_view, project_action_previews, effect_envelopes, filter_effects_for_viewer}` exist; the shared harness `assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 classifies the fixed-four pass-phase matrix as `migrate`; the shared harness owns enumeration/reporting, Briar owns hidden-data construction and reveal timing.
3. Cross-artifact: the pass-selection/exchange visibility contract is game-owned under ADR 0004; this ticket and `-022` are mutually independent appends to `games/briar_circuit/tests/visibility.rs`. Baseline projections come from `-001`.
4. §11 no-leak firewall motivates this ticket: a selected pass card before confirm is visible only where the current owner view allows and absent for non-owners/observer; sent/received pass cards during atomic exchange are present only to the affected seat under existing policy.
5. Enforcement surface = source seat × observer/all four viewers over pass select/commit/exchange surfaces (view/action tree/preview/filtered effects/export); canaries are in-memory only, never committed.

## Architecture Check

1. Enumerating the full four-seat × viewer product through the shared geometry is cleaner than per-seat ad-hoc assertions and gives deterministic structured failure context.
2. No backwards-compatibility shim is introduced; existing focused pass/no-leak tests remain. No game-specific assertion is deleted.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and decides no reveal timing (§4 mechanical-scaffolding lane).

## Verification Layers

1. Selected/sent/received pass cards hidden from observer + non-owner seats across select/commit/exchange -> no-leak visibility test via `assert_pairwise_no_leak`.
2. Owner-only visibility where current owner view allows -> no-leak test over `project_pass_view`/`project_action_previews`/`filter_effects_for_viewer`.
3. Pass target/reveal/atomic-exchange policy unchanged -> codebase grep-proof (no `src/` policy edit; tests only).

## What to Change

### 1. Enumerate the fixed-four pass-phase matrix

In `games/briar_circuit/tests/visibility.rs`, for each source seat `seat_0…seat_3` run observer plus all four seat viewers over pass selection, commitment, and atomic exchange via `assert_pairwise_no_leak`, asserting owner-only cards and non-owner/observer absence. Pass target/reveal/exchange policy stays local.

## Files to Touch

- `games/briar_circuit/tests/visibility.rs` (modify)

## Out of Scope

- The play/export/bot matrix (`-022`).
- Any pass-target, reveal-timing, or atomic-exchange policy change (game-local).
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the four-seat × viewer pass-phase no-leak matrix.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes (no production behavior changed).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. No pass card reaches observer or a non-owning seat across select/commit/exchange; the affected seat sees only its authorized surfaces.
2. No canary token appears in any committed trace, fixture, export, snapshot, log, or test identifier.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` — four source seats × observer/all four viewers over pass select/commit/exchange.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game visibility test is the correct boundary: pass no-leak is a game-local projection property through the shared harness.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a shared pairwise no-leak matrix for Briar pass-phase selected, committed, and exchanged card canaries over observer plus all four seat viewers.
2. Covered view, pass view, filtered effects, action previews, and viewer export where each surface applies; exchange allows sender/receiver effects and target post-exchange ownership only.
3. Retained the existing focused pass visibility tests and left pass target, reveal, and exchange policy unchanged.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p briar_circuit` - passed.
3. `cargo run -p replay-check -- --game briar_circuit --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.
