# UNI8CR2TWOSEA-001: Characterization baseline and locked determination

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new governance/characterization report (`reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md`); no code, schema, or trace change
**Deps**: None

## Problem

Unit 8C-R2 forbids any helper migration before a complete verdict matrix and byte/hash/seat/visibility/RNG baseline exist (spec §3, §4.2, §5.2, R2-EC-03). This ticket authors the append-only characterization report named in spec §4.2 — merging the determination freeze (spec task `8C-R2-001`) and full current-surface characterization (`8C-R2-002`) into one reviewable new file. Every migration ticket in this batch depends on it: a migration without a pre-recorded baseline cannot prove a byte/hash/visibility change was intentional and bounded.

## Assumption Reassessment (2026-06-23)

1. The four games' audited surfaces exist where spec §3–§4.1 claim: `games/{high_card_duel,secret_draft,poker_lite,masked_claims}/src/{effects,ids,setup,replay_support}.rs` (+ `actions.rs` for HCD/Masked), `crates/wasm-api/src/seats.rs`, `crates/game-test-support/src/{no_leak,profiles}.rs` — confirmed during this session's `/reassess-spec` pass.
2. Governance state is current: `specs/README.md` records `8C-R1` `Done` and `8C-R2` as the lowest non-`Done` row; `docs/MECHANIC-ATLAS.md` §10A reads `Current debt: None`; parent `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` §5 / EC-28 / EC-30 seed exactly this four-game wave.
3. Cross-artifact: the report is the shared baseline every later ticket (`-002`…`-044`) reads before touching a byte-bearing surface; the report — not any one game module — is the contract under audit here.
4. §11 universal acceptance invariant (determinism / replay-hash / no-leak) motivates this ticket: legacy `action_tree_hash` values (Secret/Poker), effect/state/view/replay/export bytes, seat spellings, and the pairwise no-leak matrices must be pinned BEFORE migration so a later diff proves intentional change vs. drift.
5. Deterministic replay/hash & no-leak surfaces pinned here (per-game legacy hashes, `PublicReplayExport` bytes, golden traces, the HCD C-07 pilot receipt `MSC-8C-007`) are recorded read-only; recording a baseline introduces no hidden-information leak (§11 no-leak firewall) and changes no canonical byte (§11/§13). The §7.3 anchors are re-read from the current tree, not copied from the spec.

## Architecture Check

1. A single report-first baseline is cleaner than each migration ticket independently re-deriving "what was here before" — one auditable rollback reference, no per-diff baseline drift.
2. No backwards-compatibility shim or alias is introduced; the report records existing state only.
3. `engine-core` is untouched and stays free of mechanic nouns (§3); no `game-stdlib` change is proposed (§4).

## Verification Layers

1. Verdict-matrix completeness (every game × C-01…C-10 cell + sub-surface has one verdict + owner) -> manual review against spec §3.3–§3.10.
2. Legacy action-tree hash anchors (Secret/Poker) and golden-trace digests match the current tree -> golden trace / deterministic replay-hash check (`replay-check --all` per game; compared to §7.3 anchors re-read from the tree).
3. C-07 baseline (HCD pilot receipt + the four pairwise matrices) and C-09 fixed-word/draw-count vectors -> no-leak visibility test enumeration + codebase grep-proof of the recorded RNG vectors.

## What to Change

### 1. Determination and verdict matrices

Record the locked next-unit proof (8C-R1 `Done`, atlas debt `None`, parent EC-28/EC-30 mapping), the exact four-game set, the High Card C-04 `migrate` / C-07 `already-discharged-by-8C-pilot` correction, and the primary + sub-surface verdict matrices from spec §3.3–§3.10.

### 2. Per-surface byte/hash/seat/visibility baseline

Pin current constructor shapes, parser spellings, exact-count diagnostics, legacy + v1-candidate action-tree hashes, state/effect/view/replay/export/diagnostic hash classes, profile classes, golden-trace before-digests, the C-07 matrices, and C-09 fixed RNG words / rejection counts / shuffled-dealt vectors (explicit `not applicable` for Secret C-09).

### 3. Checkpoint and exception stubs

Record C-06/C-07/C-09/C-10 applicability conclusions and every accepted-exception owner / compatibility window / rollback / next trigger as append-only rows for `-045` to complete.

## Files to Touch

- `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` (new)

## Out of Scope

- Any code, hash, seat, visibility, or golden-trace change (report-only).
- The register receipts and checkpoint conclusions completed in `-045`.
- Re-deriving or regenerating any expected hash; anchors are re-read, not regenerated.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace --all-targets` is green at baseline (proves the recorded baseline is the current passing state).
2. `cargo run -p replay-check -- --game <each of four> --all` passes and Secret/Poker legacy action-tree hashes match the §7.3 anchors re-read from the tree.
3. The report contains every item enumerated in spec §4.2 (matrices, seat spellings, legacy/v1 hashes, trace before-digests, profile classes, C-07 matrices, C-09 vectors, checkpoint conclusions, exception rows).

### Invariants

1. The report records existing state only — no canonical byte, hash, or seat ID is altered.
2. Every primary matrix cell and helper sub-surface has exactly one verdict and a named owner before any migration begins.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the existing workspace/replay suites named in Assumption Reassessment are the regression guard.`

### Commands

1. `cargo run -p replay-check -- --game high_card_duel --all` (representative anchor re-read).
2. `cargo test --workspace --all-targets` (full-pipeline baseline-green proof).
3. A per-game `replay-check` is the correct boundary for hash anchoring; the workspace run proves the baseline is globally green before migrations start.
