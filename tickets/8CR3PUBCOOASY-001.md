# 8CR3PUBCOOASY-001: Characterization baseline and verdict matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None (characterization report — `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`)
**Deps**: None

## Problem

Unit 8C-R3 adopts already-shipped C-01…C-10 mechanical scaffolding across four
already-shipped games (`plain_tricks`, `flood_watch`, `frontier_control`,
`event_frontier`) as a byte/hash/visibility-neutral retrofit. Every later
migration ticket must prove it changed *only* the selected scaffolding surface
and left every existing byte, hash, seat spelling, visibility result, and RNG
draw identical. That proof is impossible without a pinned pre-migration
baseline. This ticket captures that baseline and the per-surface verdict matrix
so all 47 migration tickets have an authoritative "before" to diff against.

## Assumption Reassessment (2026-06-24)

1. The four games and their scaffolding seams exist as the spec states:
   `games/plain_tricks/src/{effects,ids,setup,replay_support,visibility}.rs`,
   and the analogous trees for the other three (confirmed during
   `/reassess-spec`). None of the four `Cargo.toml` files lists
   `game-test-support` pre-R3 (confirmed absent in all four).
2. The spec (`specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md` §4.2)
   enumerates the exact report contents; `specs/README.md` records `8C-R2`
   `Done` and `8C-R3` as the lowest non-`Done` row; `docs/MECHANIC-ATLAS.md`
   §10A reads `_None_` (no open promotion debt).
3. Cross-artifact boundary under audit: the pre-migration byte/hash surfaces —
   golden traces under each `games/*/tests/golden_traces/`, the four fixtures
   sets under `games/*/data/fixtures/`, local `action_tree_hash` vectors
   (`plain_tricks/src/replay_support.rs`; `{flood_watch,frontier_control,event_frontier}/src/visibility.rs`),
   viewer-scoped exports (`plain_tricks/src/replay_support.rs::export_public_replay`),
   and C-09 shuffle/deal/RNG vectors (`*/src/setup.rs`).
4. FOUNDATIONS §11 motivates this ticket: replay, hashes, serialization order,
   RNG, and traces "remain deterministic or are explicitly migrated," and
   hidden information must not leak. The baseline is the instrument that makes
   "remained deterministic / did not leak" checkable rather than asserted.
5. Enforcement surfaces named for later diffing: deterministic replay/hash
   (`replay-check --all` per game), no-leak firewall (each game's
   `tests/visibility.rs` pairwise expectations), and RNG determinism (fixed-word
   shuffle/deal vectors in `*/src/setup.rs`). This ticket records their current
   values; it introduces no leakage or nondeterminism path.

## Architecture Check

1. A single front-loaded baseline ticket is cleaner than each migration
   re-deriving "before" state: one authoritative digest set, captured once,
   referenced by every migration's neutrality proof.
2. No backwards-compatibility shim — this is an evidence document, not code.
3. `engine-core` is untouched and stays noun-free; no `game-stdlib` promotion is
   proposed (the unit adopts shipped helpers; the third-use ledger stays
   `_None_`).

## Verification Layers

1. Baseline completeness -> manual review against spec §4.2 item list (every
   golden trace, fixture, export, action-tree hash, C-03 vector, C-07 matrix,
   C-08 caller, C-09 vector named).
2. Determinism of recorded digests -> `replay-check --all` per game + `cargo
   test --workspace` rerun, digests recorded from the executing tree (not
   copied from the spec, which deliberately omits hash literals).
3. Verdict-matrix coverage -> grep-proof that every game × C-01…C-08 aggregate
   cell and every listed sub-surface has exactly one verdict (migrate /
   not-applicable / exception) in the report.

## What to Change

### 1. Create the characterization report

Create `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`
populated per spec §4.2: repository/commit statement; locked determination and
four-game inventory; proof none of the four manifests carried `game-test-support`
pre-R3 and none was an 8C/C-08 pilot; the C-01…C-10 file/symbol inventory with
every explicit N/A and exception; before-state digests for every named golden
trace, fixture, public/seat-private export, local action-tree hash vector, and
replay/checkpoint hash; canonical seat accept/reject + WASM alias/output
vectors; C-03 accept/reject count vectors and exact diagnostics; per-game
C-04/C-05 representative trees with existing local hash; the C-07 viewer ×
surface matrices; C-08 profile metadata/owner/field tables; C-09 fixed-word
inputs, rejection paths, draw counts, and shuffled-deck outputs.

### 2. Record the verdict matrix

Reproduce the §3.3 aggregate matrix and §3.4–§3.10 sub-surface verdicts as the
authoritative per-surface ledger the register ticket (802) and capstone (803)
reconcile against.

## Files to Touch

- `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md` (new)

## Out of Scope

- Any production code, test, fixture, trace, or export change (this is
  evidence-only; the report is the only artifact).
- Freezing hash literals into the spec (the spec deliberately defers these to
  the executing tree).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — green baseline recorded.
2. `cargo run -p replay-check -- --game plain_tricks --all` (and the other three
   games) — passing, digests captured.
3. Report contains a verdict for every game × C-01…C-08 cell and every named
   sub-surface (manual + grep review).

### Invariants

1. Every baseline digest is measured from the executing tree at this commit.
2. No production or evidence byte outside the new report file changes.

## Test Plan

### New/Modified Tests

1. `None — governance/characterization report; verification is command-based and the existing workspace/replay suites named above are the baseline source.`

### Commands

1. `cargo test --workspace`
2. `cargo run -p replay-check -- --game plain_tricks --all && cargo run -p replay-check -- --game flood_watch --all && cargo run -p replay-check -- --game frontier_control --all && cargo run -p replay-check -- --game event_frontier --all`
3. Narrower per-game `replay-check`/`fixture-check` runs are the correct boundary because R3 touches only these four games; workspace test confirms no incidental drift.
