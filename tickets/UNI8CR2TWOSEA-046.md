# UNI8CR2TWOSEA-046: R2 acceptance run and Done status flip

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — verification + status-only capstone: runs the full §7 acceptance command set and flips the `8C-R2` row in `specs/README.md`; no code, schema, or trace change
**Deps**: 045

## Problem

Spec §6 + §5.14 tasks `8C-R2-803`/`804`: run the full §7 acceptance command set and the changed-file audit, prove all focused and workspace evidence is green with zero unauthorized golden/fixture diffs, then flip only `8C-R2` to `Done` in `specs/README.md` with date and outcome. `8C-R3`, `8C-R4`, and Gate 18 remain untouched. This capstone exercises the pipeline the prior tickets composed; it introduces no new production logic.

## Assumption Reassessment (2026-06-23)

1. `specs/README.md` records `8C-R2` as `Planned` (after the accepted spec lands) and is the lowest non-`Done` row; the register/report reconciliation (`-045`) and all migrations (`-002`…`-044`) have landed via the `Deps` chain.
2. Spec §6 (R2-EC-01…30) + §5.14: flip only `8C-R2`; leave `8C-R3`/`8C-R4`/Gate 18 status untouched (parent EC-30 keeps Gate 18 blocked); record date/outcome.
3. Cross-artifact under audit: the full §7 acceptance command set (workspace + per-game `replay-check`/`fixture-check` + boundary + dev-dep inverse tree + doc-link/catalog gates) and the changed-file inventory — verification surfaces, not new code.
4. §12 stop conditions: any unexpected golden/fixture byte diff is a stop condition, not an invitation to bless new bytes; any failure is classified (SUT vs test fault) before fixing, and benchmark/CI thresholds are not relaxed to close the unit.

## Architecture Check

1. A single verification + status capstone gating the `Done` flip on the full acceptance run is cleaner than flipping status from any individual migration ticket — status lands last, after evidence passes.
2. No backwards-compat alias; no production logic added; the capstone only runs existing acceptance commands and reconciles status.
3. `engine-core` untouched; no `game-stdlib` change; boundary checks are part of the acceptance set.

## Verification Layers

1. Full workspace + per-game evidence green -> `cargo test --workspace --all-targets`, `replay-check`/`fixture-check` per game, `cargo fmt --all -- --check`.
2. Boundary + dev-only edge + doc gates -> `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`, `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`.
3. Zero unauthorized golden/fixture diffs -> changed-file inventory audit (git diff over `tests/golden_traces/**` and `data/fixtures/**`) + FOUNDATIONS §12 alignment check.
4. Single-row status flip -> codebase grep-proof in `specs/README.md` (`8C-R2` → `Done`; `8C-R3`/`8C-R4`/Gate 18 unchanged).

## What to Change

### 1. Run the full §7 acceptance command set

Execute the spec §7.1 command set and record command, exit status, output summary, and changed-artifact inventory; classify any failure (SUT vs test fault) before fixing.

### 2. Flip the R2 status row

In `specs/README.md`, flip only the `8C-R2` row to `Done` with date and outcome; leave `8C-R3`, `8C-R4`, and Gate 18 untouched.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Any code/test/trace change beyond a classified SUT fix surfaced by the acceptance run (which routes to the owning ticket, not this capstone).
- Any change to `8C-R3`/`8C-R4`/Gate 18 status; any golden/fixture mass-regeneration or threshold relaxation.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all -- --check`, `cargo test --workspace --all-targets`, and the per-game `replay-check`/`fixture-check` runs are all green.
2. `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`, `node scripts/check-doc-links.mjs`, and `node scripts/check-catalog-docs.mjs` pass.
3. The changed-file inventory shows zero unauthorized golden/fixture diffs.

### Invariants

1. Only the `8C-R2` row changes status; `8C-R3`/`8C-R4`/Gate 18 are untouched.
2. No benchmark/CI threshold is relaxed and no golden/fixture is mass-regenerated to close the unit.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it exercises the prior tickets' acceptance suite and reconciles the `specs/README.md` status, adding no test file.`

### Commands

1. `cargo test --workspace --all-targets`
2. `cargo run -p replay-check -- --game high_card_duel --all` (and `secret_draft`/`poker_lite`/`masked_claims`); `cargo run -p fixture-check -- --game high_card_duel` (and the other three)
3. `bash scripts/boundary-check.sh && cargo tree --workspace -e normal --invert game-test-support && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
