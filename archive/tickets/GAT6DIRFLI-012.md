# GAT6DIRFLI-012: Rust test suite

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/directional_flip/tests/` (rules, property, visibility, serialization, bots) covering the spec §8.6 rule matrix.
**Deps**: 006, 007, 009, 011

## Problem

Per FOUNDATIONS §6/§11 and `docs/OFFICIAL-GAME-CONTRACT.md`, an official game needs unit/rule/property/visibility/serialization/bot tests with no silent coverage gaps. This ticket assembles the full `directional_flip` Rust test suite over the surfaces built in GAT6DIRFLI-005–011, covering the required matrix in spec §8.6 and the `DF-*` rule ids. (Golden traces and replay-hash corpora are GAT6DIRFLI-013.)

## Assumption Reassessment (2026-06-07)

1. The surfaces under test exist: `rules.rs` (005), `actions.rs` (006), `visibility.rs`+`ui.rs` (007), `effects.rs` (008), `replay_support.rs` (009), `bots.rs` (011). The `column_four` precedent folds serialization/unknown-field coverage into `tests/replay.rs`/`tests/rules.rs`; `three_marks` uses a standalone `tests/serialization_tests.rs`. Spec §8.6 permits either layout as long as coverage is not dropped (per the reassessed §8.6 note).
2. Spec §8.6 (required test coverage list) and §21 (`DF-*` rule-id seed) are authoritative; each listed behavior maps to a named test.
3. Cross-artifact boundary under audit: the tests assert invariants spanning rules ↔ actions ↔ effects ↔ visibility ↔ replay ↔ bots. Each distinct invariant gets its own proof surface (no collapsing into one generic "validation").
4. FOUNDATIONS §11 acceptance invariants motivate the suite: restate before authoring — unknown fields rejected by default, behavior-looking fields blocked, views viewer-safe, replay/hash deterministic, bots legal. The serialization tests must prove **fail-closed** unknown-field rejection (§5/§11), not merely happy-path round-trip.
5. This ticket exercises the deterministic replay/hash & serialization surfaces and the no-leak firewall: confirm the serialization tests reject unknown and behavior-looking fields (`DF-SER-001`), the visibility tests assert no hidden-state leak (`DF-VIEW-001`), and the replay tests prove determinism across export/import/step/reset (`DF-REPLAY-001`).

## Architecture Check

1. A dedicated test crate-dir mapping each `DF-*` id to a named test gives `RULE-COVERAGE.md` (GAT6DIRFLI-016) a concrete, tool-checkable backing and makes coverage gaps visible rather than silent.
2. No backwards-compatibility shims; new tests only.
3. `engine-core` untouched; the suite includes a boundary assertion that no mechanic noun leaked into the kernel (§3).

## Verification Layers

1. Rule legality/flip/pass/terminal/scoring -> rule tests (`DF-SETUP-001`, `DF-LEGAL-001`..`006`, `DF-FLIP-001`..`004`, `DF-PASS-001`/`002`, `DF-TERM-001`, `DF-SCORE-001`/`002`).
2. Action/preview -> rule + property tests (`DF-ACTION-001`..`003`, `DF-PREVIEW-001` preview==apply).
3. Visibility no-leak -> no-leak visibility test (`DF-VIEW-001`).
4. Serialization fail-closed -> schema/serialization validation (`DF-SER-001`): unknown + behavior-looking fields rejected.
5. Replay determinism -> deterministic replay-hash check (`DF-REPLAY-001`).
6. Bot legality/determinism -> bot legality check (`DF-BOT-001`/`002`).

## What to Change

### 1. Rule & property tests

`tests/rules.rs` and `tests/property.rs` covering spec §8.6: standard setup, first legal moves, bracketing required, all-direction flips, multi-direction flip, no-skip-own, no-indirect-flip, occupied/non-flipping/stale/wrong-actor rejection, terminal-no-choices, forced-pass-only/forbidden, double-pass terminal, full-board terminal, count winner, draw, preview==apply, deterministic effect order, stable action segments, action-tree-legal-only.

### 2. Visibility, serialization, bot tests

`tests/visibility.rs` (no hidden state), `tests/serialization.rs` **or** folded into `tests/replay.rs`/`tests/rules.rs` per the column_four precedent (unknown-field + behavior-looking rejection), `tests/bots.rs` (random legal validates; Level 2-lite deterministic + legal + safe explanations).

## Files to Touch

- `games/directional_flip/tests/rules.rs` (new — may be extended from the inline tests seeded in 005/006)
- `games/directional_flip/tests/property.rs` (new)
- `games/directional_flip/tests/visibility.rs` (new — extended from 007)
- `games/directional_flip/tests/replay.rs` (new — extended from 009; may host serialization coverage per column_four precedent)
- `games/directional_flip/tests/bots.rs` (new — extended from 011)

## Out of Scope

- Golden trace fixtures and the `replay-check` corpus run (GAT6DIRFLI-013).
- Benchmarks (014), WASM smoke (015), browser smoke (018).
- `RULE-COVERAGE.md` authoring (GAT6DIRFLI-016) — this suite is what it maps to.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — full suite passes.
2. `cargo test --workspace` — no regression elsewhere.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Every spec §8.6 behavior and every `DF-*` rule id has a named test (no silent gap) (FOUNDATIONS §6).
2. Serialization is fail-closed (unknown/behavior-looking fields rejected); views no-leak; replay deterministic (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/{rules,property,visibility,replay,bots}.rs` — the full matrix above, one named test per `DF-*` obligation.

### Commands

1. `cargo test -p directional_flip`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Workspace test + boundary check is the correct boundary; trace/replay-check tool runs are GAT6DIRFLI-013/016.

## Outcome

Added the integration test suite under `games/directional_flip/tests/`:

1. `rules.rs` names and exercises setup, action generation, validation fail-closed behavior, flips, forced pass, terminal, and scoring rule IDs.
2. `property.rs` covers preview/apply consistency and bounded random legal playout termination.
3. `visibility.rs` covers public-view no-leak and terminal action absence.
4. `replay.rs` covers deterministic replay/hash behavior plus fail-closed replay/static-data serialization.
5. `bots.rs` covers Level 0/Level 2 legality, determinism, safe explanations, and bot effects.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p directional_flip`
3. `cargo test --workspace`
4. `bash scripts/boundary-check.sh`
5. `node scripts/check-doc-links.mjs`
