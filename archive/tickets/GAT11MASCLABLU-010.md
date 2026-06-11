# GAT11MASCLABLU-010: Native test suite (rules, property, replay, serialization, visibility, bots)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/masked_claims/tests/{rules,property,replay,serialization,visibility,bots}.rs`
**Deps**: GAT11MASCLABLU-008, GAT11MASCLABLU-009

## Problem

The official-game contract (FOUNDATIONS §6) requires the full native evidence set: rule legality + diagnostics, property invariants, deterministic replay, serialization stability, visibility/no-leak, and bot legality. These prove the implemented behavior holds across many seeds and that hidden information never leaks.

## Assumption Reassessment (2026-06-10)

1. The full pipeline (GAT11MASCLABLU-004–009) provides the behavior under test. The integration-test shape follows the hidden-information-game convention — `tests/rules.rs` as a dedicated file — matching `high_card_duel`, `secret_draft`, `poker_lite`, `token_bazaar` (all confirmed to carry `tests/rules.rs`). Per the spec's Native-tests note, do NOT mirror `plain_tricks`, the sole game that inlines its rule tests in `src/rules.rs`.
2. Spec §Acceptance evidence "Native rules, replay, visibility, and bot evidence" enumerates the required coverage: claim/reaction legality, diagnostics (stale/wrong-seat/wrong-phase/unowned/out-of-range), property invariants (fifteen-tile conservation, exactly one open window, no reveal of accepted masks, terminal at the fixed turn count, score consistency, no panics), replay (state/effect/action-tree/view hashes, reveal ordering, terminal), serialization (stable summaries, unknown-field rejection), visibility/no-leak (all enumerated surfaces), and bot legality + determinism + hidden-state independence.
3. Cross-artifact boundary under audit: each suite maps a distinct invariant class to its own proof surface; this ticket must not collapse them into one generic "validation" file.
4. FOUNDATIONS §11 acceptance invariants (determinism, no-leak firewall, fail-closed validation, bot legality) are the principles under audit.
5. No-leak and determinism enforcement surfaces: `tests/visibility.rs` searches public views, opponent seat views, action trees, previews, diagnostics, effect payloads, command summaries, export/import timelines, bot explanations, and candidate rankings for unrevealed tile IDs; `tests/replay.rs` proves identical seed + seats + options + command stream reproduces all hashes. Follow the failing-test protocol — never weaken a test to get green.

## Architecture Check

1. One test file per invariant class keeps each proof surface legible and independently reviewable, and matches the hidden-info-game convention in the repo.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; no `game-stdlib` test helper is promoted.

## Verification Layers

1. Claim/reaction legality + diagnostics -> `tests/rules.rs`.
2. Conservation / single-window / no-accepted-reveal / terminal / score-consistency / no-panics -> `tests/property.rs`.
3. Deterministic state/effect/action-tree/view hashes + reveal ordering + terminal -> `tests/replay.rs`.
4. Stable summaries + unknown-field rejection (manifest, variants, fixtures, export, internal trace) -> `tests/serialization.rs`.
5. No unrevealed tile ID across all enumerated surfaces -> `tests/visibility.rs`.
6. Level 0/1 legality, determinism, hidden-state independence -> `tests/bots.rs`.

## What to Change

### 1. The six test files

`tests/rules.rs`, `tests/property.rs`, `tests/replay.rs`, `tests/serialization.rs`, `tests/visibility.rs`, `tests/bots.rs` — each covering its invariant class per the spec Acceptance-evidence section.

## Files to Touch

- `games/masked_claims/tests/rules.rs` (new)
- `games/masked_claims/tests/property.rs` (new)
- `games/masked_claims/tests/replay.rs` (new)
- `games/masked_claims/tests/serialization.rs` (new)
- `games/masked_claims/tests/visibility.rs` (new)
- `games/masked_claims/tests/bots.rs` (new)

## Out of Scope

- Golden traces and fixture content (GAT11MASCLABLU-011).
- Benchmarks (GAT11MASCLABLU-012).
- Tool/CLI registration that runs simulation/replay-check at scale (GAT11MASCLABLU-015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` passes all six suites.
2. The visibility suite finds zero unrevealed tile IDs across every enumerated surface.
3. The replay suite confirms identical inputs reproduce all hashes and the reveal ordering.

### Invariants

1. Determinism, no-leak, fail-closed validation, and bot legality are each proven by a distinct suite (FOUNDATIONS §6/§11).
2. No test is weakened or deleted to get green (AGENT-DISCIPLINE failing-test protocol).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs` — one per invariant class.

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. `cargo test --workspace` is the correct full-pipeline boundary once registered, but per-crate test is the reviewable boundary for this ticket; scaled simulation lands in GAT11MASCLABLU-015.

## Outcome

Completed: 2026-06-11

What changed:

- Added six native integration suites under `games/masked_claims/tests/`: `rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, and `bots.rs`.
- Covered claim/reaction validation, fail-closed diagnostics, accept/challenge resolution, many-seed invariant checks, deterministic replay surfaces, stable view/export serialization, no-leak visibility surfaces, and Level 0/Level 1 bot legality.
- Kept each invariant class in its own file, matching the hidden-information-game convention.

Deviations from original plan:

- Full golden-trace fixture assertions and scaled simulation remain for the later trace/tool tickets. This ticket adds native integration coverage over the public crate API.

Verification:

- `cargo test -p masked_claims` passed.
- `cargo clippy -p masked_claims --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
