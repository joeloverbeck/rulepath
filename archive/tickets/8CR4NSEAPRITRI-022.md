# 8CR4NSEAPRITRI-022: Briar Circuit C-07 play/export/bot pairwise no-leak matrix

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/briar_circuit/tests/`; reveal timing + bot policy unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Briar's private hands before play, public cards after authorized play, and bot input/explanation/candidate surfaces are not yet covered as a shared pairwise no-leak matrix across the four source seats × observer/all viewers (MSC-8C, C-07). Cover view, export, diagnostics, bot input/explanation, and action choices, with reveal timing and bot policy kept Briar-owned (spec §3.8 Briar, §5.8).

## Assumption Reassessment (2026-06-24)

1. `visibility::project_view`, `replay_support::export_viewer_timeline`, the Briar bot explanation/candidate surfaces, and `games/briar_circuit/tests/visibility.rs` exist; the shared harness `assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 classifies the play/export/bot matrix as `migrate`; reveal timing stays Briar-owned. This ticket and `-021` are mutually independent appends to `games/briar_circuit/tests/visibility.rs`.
3. Cross-artifact: projection (ADR 0004), export, and bot input are game-owned; the shared harness owns enumeration/reporting. Baseline projections/exports come from `-001`.
4. §11 no-leak firewall + §8 bot-input rules motivate this ticket: other hands and pass provenance must be absent from diagnostics/public export/bot explanation/candidates; played/captured cards become public only after game-authorized play; moon/scoring totals are public without private card identities.
5. Enforcement surface = source seat × observer/all four viewers over view/export/diagnostic/bot-input/bot-explanation/candidate surfaces; bots receive only legal viewer-authorized input and explanations leak no raw disallowed IDs; canaries in-memory only.

## Architecture Check

1. One shared matrix over view/export/bot surfaces is cleaner than scattering per-surface assertions and proves the bot path uses only allowed views.
2. No backwards-compatibility shim is introduced; existing focused rule/no-leak tests remain. No game-specific assertion is deleted; no bot strategy changes.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4 mechanical-scaffolding lane).

## Verification Layers

1. Private hands hidden pre-play; public cards present post-authorized-play, for all viewers -> no-leak visibility test via `assert_pairwise_no_leak`.
2. No other-hand/pass-provenance leak into diagnostics/public export/bot explanation/candidates -> no-leak test over `export_viewer_timeline` + bot surfaces.
3. Bot uses only legal viewer-authorized input -> bot legality check over the Briar bot input/candidate path.

## What to Change

### 1. Add the play/export/bot no-leak matrix

In `games/briar_circuit/tests/visibility.rs` (and the bot/replay test modules as needed), enumerate four source seats × observer/all four viewers over `project_view`, `export_viewer_timeline`, diagnostics, bot input/explanation, and action choices via `assert_pairwise_no_leak`. Assert private-hand absence pre-play, public cards post-authorized-play, and no raw disallowed IDs in explanations.

## Files to Touch

- `games/briar_circuit/tests/visibility.rs` (modify)
- `games/briar_circuit/tests/replay.rs` (modify; or a narrowly added bot/export no-leak module)

## Out of Scope

- The pass-phase matrix (`-021`).
- Any reveal-timing, scoring, or bot-strategy change (game-local).
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the play/export/bot four-seat × viewer no-leak matrix.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes (no production behavior changed).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. No private hand or pass provenance reaches an unauthorized view/export/diagnostic/bot surface; public cards appear only after authorized play.
2. No canary token appears in any committed artifact; bots use only legal viewer-authorized input.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` / `tests/replay.rs` — play/export/bot no-leak matrix over four seats × observer/all viewers.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game visibility/bot test is the correct boundary: no-leak across play/export/bot is a game-local projection property.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a shared pairwise no-leak matrix for Briar play/export/bot surfaces over observer plus all four seat viewers.
2. Covered private pre-play card canaries and public after-play card canaries across view, filtered effects, action previews, viewer export, diagnostics, bot legal choices, and bot explanations.
3. Kept reveal timing, scoring, diagnostics, and bot strategy unchanged; the change is test-only.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p briar_circuit` - passed.
3. `cargo run -p replay-check -- --game briar_circuit --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.
