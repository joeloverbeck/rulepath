# 8CR4NSEAPRITRI-001: R4 characterization baseline, pilot-credit inventory, and locked determination

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — new governance/characterization report (`reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md`); no code, schema, or trace change
**Deps**: None

## Problem

Unit 8C-R4 forbids any helper migration before a complete verdict matrix and a byte/hash/seat/RNG/visibility baseline exist (spec §3.3, §4.2, §5.2, EC-R4-02/EC-R4-03/EC-R4-04). This ticket authors the characterization report named in spec §4.2 — consolidating the locked determination (candidate `8CR4NSEAT-001`), the pilot-credit-versus-residual inventory (`8CR4NSEAT-002`), and the baseline byte/ID/visibility/profile/RNG pins (`8CR4NSEAT-003`) into one reviewable new file. Every migration ticket `-002`…`-035` depends on it: a migration without a pre-recorded baseline cannot prove a byte/hash change was intentional and bounded, and the four pilot-credit games' already-discharged surfaces must be pinned to their exact receipts so R4 verifies rather than rebuilds them.

## Assumption Reassessment (2026-06-24)

1. The three games' audited surfaces exist where the spec claims: `games/{river_ledger,briar_circuit,vow_tide}/src/{actions,effects,ids,replay_support,setup,state,visibility}.rs`, `crates/wasm-api/src/{seats.rs,games/{briar,vow}.rs}`, and `crates/game-test-support/src/{profiles.rs,no_leak.rs}` — confirmed during `/reassess-spec` of this spec (HEAD `0d01901`).
2. Governance state is current: `specs/README.md` records `8C-R3` as `Done` and `8C-R4` as the lowest non-`Done` row; `docs/MECHANIC-ATLAS.md` §10A reads `Current debt: None`; the parent `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` work item `8C-030` and exit criteria EC-28/EC-30 seed exactly this three-game wave.
3. Cross-artifact: the characterization report is the shared baseline every later ticket (`-002`…`-037`) reads before touching a byte-bearing surface; the report is the contract under audit here, not any one game module. Per spec report-edit discipline, per-migration after-receipts are consolidated by the `-036` register and `-037` capstone, not appended to this report by every migration.
4. §11 universal acceptance invariant (determinism/replay-hash + no-leak) motivates this ticket: legacy `replay_support::action_tree_hash` / `action_hash` / `snapshot` values, effect/state/view/replay bytes, seat spellings, viewer/export classes, and RNG draw/index vectors must be pinned BEFORE migration so a later diff proves intentional change vs. drift.
5. The deterministic replay/hash and visibility surfaces pinned here are recorded read-only; recording a baseline introduces no hidden-information leak (§11 no-leak firewall) and changes no canonical byte (§11/§13). RNG vectors for Briar/Vow legacy modulo `DeterministicRng::next_index` are recorded without altering the algorithm.

## Architecture Check

1. A single report-first baseline is cleaner than each migration ticket independently re-deriving "what was here before" — it gives one auditable rollback reference and prevents per-diff baseline drift across mutually-independent migrations.
2. No backwards-compatibility shim or alias is introduced; the report records existing state only.
3. `engine-core` is untouched and stays free of mechanic nouns (§3); no `game-stdlib` change is proposed (§4); the existing `game-stdlib::trick_taking` promotion is recorded as out-of-lane, not reopened.

## Verification Layers

1. Verdict-matrix completeness (every game × C-01…C-09 cell and listed sub-surface has one verdict + owner) -> manual review against spec §3.3–§3.10.
2. Pilot-credit pinning (River C-01/C-02/C-03/C-06/C-07-base/C-08-setup/C-09; Vow C-06/C-08-public/seat-private; Briar C-06/C-08-domain) maps each to its `MSC-8C-*` entry and `UNI8CMECSCA-*` ticket -> codebase grep-proof against `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and `archive/tickets/UNI8CMECSCA-{009,011,017,021,024,025,026}.md`.
3. Legacy hash / byte / seat / RNG baselines match the current tree -> golden trace / deterministic replay-hash check (`replay-check --all` per game) plus file-read of `replay_support.rs` hashers and `tests/{visibility,replay,serialization}.rs`.

## What to Change

### 1. Determination, three-game scope, and Gate-18 interlock

Record the locked next-unit proof (`8C-R3` `Done`, atlas debt `None`, parent EC-28/EC-30 mapping), the exact `{river_ledger, briar_circuit, vow_tide}` set, and the statement that R4 closeout clears only the C-11 item of Gate 18's admission interlock (other named conditions persist; Gate 18 is neither authored nor begun). Carry the four grounded corrections from the spec preamble (River chronology, Vow C-02 migrate, Vow private C-01 N/A, Briar/Vow C-09 N/A).

### 2. Pilot-credit-versus-residual inventory

Pin every `already-discharged-by-8C-pilot` cell to its exact `MSC-8C-*` + `UNI8CMECSCA-*` receipt, and every residual cell to an actual `0d01901` seam (file::symbol). Verify River C-01 envelope receipt, River ticket-021 base no-leak matrix, and the three pilot C-08 profiles without rebuilding them.

### 3. Per-surface byte/hash/seat/visibility/RNG baseline

Record current constructor shapes; canonical/accepted/rejected seat strings for Briar (`seat_0…seat_3`) and Vow (`seat_0…seat_6`); exact-count diagnostics (`BC_UNSUPPORTED_SEAT_COUNT`, `VT_INVALID_SEAT_COUNT`); legacy action-tree/`Debug`-derived hashes; viewer/export classes; profile ownership; the River 3–6 / Vow 3–7 / Briar fixed-4 seat×viewer enumeration counts; and Briar/Vow legacy `next_index` draw/index/deck/deal vectors plus the River `next_index_unbiased_v1` parity receipt — all read-only, no algorithm or golden change.

## Files to Touch

- `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` (new)

## Out of Scope

- Any code, hash, seat, visibility, RNG, or golden-trace change (this ticket is report-only).
- The register receipts and checkpoint conclusions completed in `-036`, and the command/diff audit completed in `-037`.
- Re-deriving or regenerating any expected hash; baselines are re-read, not regenerated.
- Re-implementing any pilot-discharged surface or reopening the locked unit/game set or Gate 18.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` is green at baseline (proves the recorded baseline is the current passing state).
2. `cargo run -p replay-check -- --game river_ledger --all`, `--game briar_circuit --all`, and `--game vow_tide --all` pass and their legacy hashes are recorded as sentinels.
3. The report contains every item enumerated in spec §4.2 (determination, pilot-receipt table, aggregate + sub-surface verdicts, per-surface bytes/hashes/seat strings, seat×viewer counts, RNG vectors, diff-inventory skeleton, exception/N-A ledger).

### Invariants

1. The report records existing state only — no canonical byte, hash, seat ID, or RNG vector is altered.
2. Every aggregate matrix cell and listed sub-surface has exactly one verdict and a named owner before any migration begins.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the existing workspace/replay/visibility suites named in Assumption Reassessment are the regression guard.`

### Commands

1. `cargo run -p replay-check -- --game vow_tide --all` (representative widest-matrix sentinel re-read).
2. `cargo test --workspace` (full-pipeline baseline-green proof).
3. A per-game `replay-check` is the correct boundary for sentinel pinning; the workspace run proves the baseline is globally green before migrations start.

## Outcome

Completed: 2026-06-24

What changed:
- Added `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md`
  as the report-first baseline for Unit 8C-R4.
- Recorded the locked three-game scope, Gate 18 interlock boundary, grounded
  corrections, pilot receipt inventory, aggregate and sub-surface verdicts,
  current seams, seat/viewer enumeration counts, RNG baseline, N/A/exception
  ledger, and artifact diff inventory.
- Recorded legacy replay/hash/profile sentinels without changing code,
  fixtures, golden traces, canonical bytes, seat IDs, visibility policy, or RNG
  algorithms.

Deviations:
- The ticket command list named Vow as the representative replay-check, but the
  acceptance criteria required per-game sentinel pinning. River Ledger, Briar
  Circuit, and Vow Tide replay-checks were all run and recorded.

Verification:
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `cargo run -p replay-check -- --game vow_tide --all`
- `cargo test --workspace`
