# 8CR4NSEAPRITRI-037: R4 verification, artifact-diff audit, and closeout/tracker flip

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — verification + status-flip capstone (`specs/README.md`, register closeout block, characterization report); no code/byte change
**Deps**: 8CR4NSEAPRITRI-036

## Problem

R4 closes only when the full acceptance command set passes, every changed artifact is classified against the admission inventory, and the tracker flips — exercising every prior ticket end-to-end (spec §5.11 tasks 902/903, §6 EC-R4-01…30, §7.1, §7.3, §11.3). This capstone runs the §7.1 commands, audits the candidate byte-diff (`unchanged` / `parallel-new` / authorized `intentional-migration`), adds the Unit 8C-R4 register closeout block, and flips only the `8C-R4` row to `Done` — clearing the final C-11 item of Gate 18's admission interlock without touching Gate 18.

## Assumption Reassessment (2026-06-24)

1. `specs/README.md` records `8C-R4` as the lowest non-`Done` row (confirmed during `/reassess-spec`); the §7.1 command set (`cargo fmt/clippy/test`, `replay-check`/`fixture-check`/`rule-coverage` per game, `boundary-check.sh`, inverse `cargo tree`, `check-doc-links`/`check-catalog-docs`) resolves against the workspace.
2. Spec §6 exit criteria EC-R4-01…30 and §7.3 golden/fixture/export/hash policy define the audit; the register receipts come from `-036` (transitive head over all migrations).
3. Cross-artifact: this is the verification + status-reconciliation capstone; it exercises every prior ticket but modifies no upstream source. It owns the `specs/README.md` row, the register closeout block, and the report's final audit section.
4. §11 acceptance invariants + §7.3 motivate this ticket: the default artifact result is existing bytes `unchanged`, new action-tree-v1 vectors/profile adapters/no-leak adapters `parallel-new`, with zero unauthorized changes; a green v1 hash is never used to replace a legacy authority.
5. Enforcement surface = the full §7.1 command set + `git diff --name-only/--stat/--check` byte-digest audit mapping every changed path to its owning packet and approved surface; no blanket golden regeneration.

## Architecture Check

1. A single trailing verification + closeout capstone is cleaner than scattering exit evidence across migrations — it exercises the composed pipeline and reconciles status once.
2. No backwards-compatibility shim is introduced and no production logic is added; the capstone exercises, it does not modify, the upstream surfaces.
3. `engine-core` stays noun-free (§3); no helper is broadened (§4); the `apps/web/README.md` surface is explicitly N/A (catalog checks are regression guards only).

## Verification Layers

1. Full acceptance command set passes -> simulation/CLI + replay-hash + fixture + rule-coverage runs per spec §7.1 (or a failure resolved via the valid-test/SUT-or-test protocol).
2. Every changed artifact classified, zero unauthorized -> golden/fixture/export byte-diff audit (`git diff --name-only/--stat/--check`) against the §7.3 inventory.
3. Tracker flip + interlock statement correct -> codebase grep-proof (`specs/README.md` `8C-R4` reads `Done`; closeout states the final C-11 Gate-18 interlock is cleared while Gate 18 stays unstarted).

## What to Change

### 1. Run the full acceptance command set and artifact-diff audit

Run the §7.1 command set and the `git diff` byte-digest audit; classify every changed golden/fixture/export/trace/hash path as `unchanged`, `parallel-new`, or separately-authorized `intentional-migration`, with zero unauthorized changes. Record results in the report.

### 2. Closeout block and tracker flip

Add the Unit 8C-R4 closeout block to `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and flip only the `specs/README.md` `8C-R4` row to `Done`, linking final evidence and stating that all four C-11 waves are closed/disposed (parent EC-28/EC-30) and the final C-11 Gate-18 admission interlock is cleared. Do not alter Gate 18's scope or status.

## Files to Touch

- `specs/README.md` (modify; flip only the `8C-R4` row to `Done`)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify; add the Unit 8C-R4 closeout block)
- `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` (modify; final commands, diff inventory, rollback map)

## Out of Scope

- Authoring, starting, or altering Gate 18 (scope/status untouched).
- Any source/byte change, golden regeneration, or accepting changed expected hashes.
- Editing any foundation doc or ADR by default (a genuine gap is a flagged §8.4 blocking trigger).

## Acceptance Criteria

### Tests That Must Pass

1. The full §7.1 command set passes: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, per-game `replay-check`/`fixture-check`/`rule-coverage`, `boundary-check.sh`, the two inverse `cargo tree`, `check-doc-links.mjs`, `check-catalog-docs.mjs`.
2. `git diff --name-only/--stat/--check <r4-baseline>...HEAD` shows every changed path mapped to its owning packet; existing golden/fixture/export bytes and legacy hashes are `unchanged`; new v1/profile/no-leak surfaces are `parallel-new`; unauthorized changes are zero.
3. `specs/README.md` `8C-R4` reads `Done` and the closeout states the final C-11 Gate-18 interlock is cleared while Gate 18 remains unstarted.

### Invariants

1. No legacy byte/hash/seat/visibility/RNG authority changed without a separately-accepted ADR-0009 packet; no blanket golden regeneration occurred.
2. Only the `8C-R4` tracker row flips to `Done`; Gate 18 is untouched.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo test --workspace`
2. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p replay-check -- --game briar_circuit --all && cargo run -p replay-check -- --game vow_tide --all`
3. `git diff --stat <r4-baseline>...HEAD` (artifact-diff audit) — the narrower correctness boundary that proves no unauthorized byte changed.

## Outcome

Completed: 2026-06-24

What changed:
- Ran the full §7.1 capstone command set and recorded final command evidence in the characterization report.
- Recorded the final artifact-diff audit from baseline `9c5b4c8730fc917af88aefdfae7e641c258e94d5`, with existing golden/fixture/export/legacy hash bytes unchanged and unauthorized artifact changes at zero.
- Added the Unit 8C-R4 final closeout block to the mechanical scaffolding register, flipped only the `8C-R4` tracker row to `Done`, and archived the completed R4 spec.
- Stated that all four C-11 follow-on waves are closed or explicitly disposed, clearing the final C-11 Gate 18 admission interlock while Gate 18 remains unstarted.

Deviations:
- None.

Verification:
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p engine-core`
- `cargo test -p game-stdlib`
- `cargo test -p game-test-support`
- `cargo test -p wasm-api`
- `cargo test -p river_ledger`
- `cargo test -p briar_circuit`
- `cargo test -p vow_tide`
- `cargo test -p replay-check`
- `cargo test -p fixture-check`
- `cargo test -p rule-coverage`
- `cargo test --workspace`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `cargo run -p replay-check -- --game vow_tide --all`
- `cargo run -p fixture-check -- --game river_ledger`
- `cargo run -p fixture-check -- --game briar_circuit`
- `cargo run -p fixture-check -- --game vow_tide`
- `cargo run -p rule-coverage -- --game river_ledger`
- `cargo run -p rule-coverage -- --game briar_circuit`
- `cargo run -p rule-coverage -- --game vow_tide`
- `bash scripts/boundary-check.sh`
- `cargo tree --workspace -e normal --invert game-test-support`
- `cargo tree --workspace -e normal,build --invert game-test-support`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `git diff --name-only 9c5b4c8730fc917af88aefdfae7e641c258e94d5...HEAD`
- `git diff --stat 9c5b4c8730fc917af88aefdfae7e641c258e94d5...HEAD`
- `git diff --check 9c5b4c8730fc917af88aefdfae7e641c258e94d5...HEAD`
