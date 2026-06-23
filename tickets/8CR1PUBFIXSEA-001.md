# 8CR1PUBFIXSEA-001: Characterization baseline and locked determination

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new governance/characterization report (`reports/8c-r1-public-fixed-seat-scaffolding-characterization.md`); no code, schema, or trace change
**Deps**: None

## Problem

Unit 8C-R1 forbids any helper migration before a complete verdict matrix and byte/hash/seat baseline exist (spec §3.2, §5.2, EC-R1-03/EC-R1-17). This ticket authors the append-only characterization report named in spec §4.2 — merging the determination freeze (candidate `8C-R1-001`) and current-surface characterization (`8C-R1-002`) into one reviewable new file. Every migration ticket in this batch depends on it: a migration without a pre-recorded baseline cannot prove a byte/hash change was intentional and bounded.

## Assumption Reassessment (2026-06-23)

1. The six games' audited surfaces exist where the spec claims: `games/{draughts_lite,three_marks,column_four,directional_flip,token_bazaar}/src/{effects,ids,setup,replay_support}.rs`, `crates/wasm-api/src/seats.rs`, and `crates/game-test-support/src/profiles.rs` — confirmed during reassessment of `specs/8c-r1-public-fixed-seat-scaffolding.md`.
2. Governance state is current: `specs/README.md` records Unit `8C` as `Done` and `8C-R1` as the lowest non-`Done` row; `docs/MECHANIC-ATLAS.md` §10A reads `Current debt: None`; the parent `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` work item `8C-030` and exit criteria EC-28/EC-30 seed exactly this six-game wave.
3. Cross-artifact: the characterization report is the shared baseline every later ticket (`-002`…`-036`) reads before touching a byte-bearing surface; the report is the contract under audit here, not any one game module.
4. §11 universal acceptance invariant (determinism/replay-hash) motivates this ticket: legacy `action_tree_hash` values, effect/state/view/replay bytes, and seat spellings must be pinned BEFORE migration so a later diff proves intentional change vs. drift.
5. The deterministic replay/hash surfaces pinned here (the per-game legacy `action_tree_hash`, the six `wasm-exported.trace.json` documents, the `PublicReplayExport` bytes) are recorded read-only; recording a baseline introduces no hidden-information leak (§11 no-leak firewall) and changes no canonical byte (§11/§13). The §7.3 sentinels are re-read from the current tree, not copied from the spec.

## Architecture Check

1. A single report-first baseline is cleaner than each migration ticket independently re-deriving "what was here before" — it gives one auditable rollback reference and prevents per-diff baseline drift.
2. No backwards-compatibility shim or alias is introduced; the report records existing state only.
3. `engine-core` is untouched and stays free of mechanic nouns (§3); no `game-stdlib` change is proposed (§4).

## Verification Layers

1. Verdict matrix completeness (every game × helper cell has a final verdict + owner) -> manual review against spec §3.2–§3.8.
2. Legacy action-tree hash sentinels match the current traces -> golden trace / deterministic replay-hash check (`replay-check --all` per game; compare to spec §7.3 sentinels re-read from the tree).
3. Recorded byte digests of all candidate artifacts -> codebase grep-proof / file-read of the six `wasm-exported.trace.json` and per-game `replay_support.rs` hashes.

## What to Change

### 1. Determination and verdict matrices

Record the locked next-unit proof (8C `Done`, atlas debt `None`, parent EC-28/EC-30 mapping), the exact six-game set, and the final primary + sub-surface verdict matrices from spec §3.2–§3.8.

### 2. Per-surface byte/hash baseline

Pin current constructor shapes, parser spellings, exact-count diagnostics, legacy and (where present) v1 action-tree hashes, profile classes, the six `wasm-exported.trace.json` before-digests, and an explicit `not applicable` for C-09 RNG vectors where absent.

### 3. Checkpoint applicability stubs

Record the C-06/C-07/C-09/C-10 applicability conclusions and every accepted-exception owner/trigger as append-only rows for `-037` to complete.

## Files to Touch

- `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` (new)

## Out of Scope

- Any code, hash, seat, visibility, or golden-trace change (this ticket is report-only).
- The register receipts and checkpoint conclusions completed in `-037`.
- Re-deriving or regenerating any expected hash; sentinels are re-read, not regenerated.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace --all-targets` is green at baseline (proves the recorded baseline is the current passing state).
2. `cargo run -p replay-check -- --game <each of six> --all` passes and its legacy action-tree hashes match the §7.3 sentinels re-read from the tree.
3. The report contains every item enumerated in spec §4.2 (matrices, seat spellings, legacy/v1 hashes, the six trace before-digests, profile classifications, checkpoint conclusions, exception rows).

### Invariants

1. The report records existing state only — no canonical byte, hash, or seat ID is altered.
2. Every primary matrix cell and helper sub-surface has exactly one verdict and a named owner before any migration begins.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the existing workspace/replay suites named in Assumption Reassessment are the regression guard.`

### Commands

1. `cargo run -p replay-check -- --game token_bazaar --all` (representative sentinel re-read).
2. `cargo test --workspace --all-targets` (full-pipeline baseline-green proof).
3. A narrower per-game command is the correct boundary for sentinel pinning; the workspace run proves the baseline is globally green before migrations start.
