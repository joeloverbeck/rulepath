# 8CR3PUBCOOASY-803: R3 acceptance evidence and status closeout

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs/status-only — `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`, `specs/README.md`)
**Deps**: 8CR3PUBCOOASY-802

## Problem

R3 closes only after every migration, exception, and N/A is reconciled and the
full acceptance command set passes on the final tree with zero unauthorized
golden/fixture/export diffs. This capstone consolidates the characterization
report (reconciling every matrix cell, sub-surface, and hash/visibility/RNG
result), runs the spec §7.1 command set, audits the changed-file inventory, and
— only after every exit row passes — flips the `8C-R3` row in `specs/README.md`
to `Done`. `8C-R4` and Gate 18 remain untouched.

## Assumption Reassessment (2026-06-24)

1. The characterization report
   `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`
   is created by ticket 001 and appended through execution; `specs/README.md`
   holds the `8C-R3` row as `Not started` (seed) — to be flipped to `Done`. The
   register receipts land in 802 (this ticket's dependency, whose own Deps cover
   every migration).
2. Spec §6 (34 exit rows), §7.1 (command set), §7.4 (golden/fixture/export diff
   policy: default authorized changes = none), §7.5 (required reviews), and
   §5.15 tasks `8C-R3-801/803/804` scope this consolidation + acceptance + status
   flip. The spec's §10 Documentation-updates assigns the `specs/README.md` Done
   flip to closeout.
3. Cross-artifact boundary under audit: this capstone introduces no production
   logic — it exercises the pipeline the migration tickets composed and
   reconciles evidence/status; it must surface any unauthorized artifact diff as
   a stop condition, not bless new bytes.
4. FOUNDATIONS §11 (determinism/no-leak) + §12: the full acceptance run must be
   green with no deleted/weakened tests and no unauthorized golden/fixture/export
   diff; an unexpected diff stops closeout.
5. Enforcement surface: the spec §7.1 command set (fmt/clippy/build, per-crate +
   workspace tests, the four games' replay-check/fixture-check/rule-coverage,
   boundary-check, inverse `cargo tree`, doc-link + catalog-doc guards) re-run on
   the final tree; the changed-file audit; the `specs/README.md` row flip.

## Architecture Check

1. A single trailing capstone gating the status flip on the full acceptance run
   is cleaner than flipping the tracker per-ticket; it guarantees no partial
   closeout.
2. No backwards-compatibility alias — verification + status reconciliation only.
3. `engine-core`/`game-stdlib` untouched; no behavior moves; the capstone
   exercises, it does not modify, the upstream surfaces.

## Verification Layers

1. Full acceptance -> the spec §7.1 command set passes on the final tree (fmt,
   clippy, all per-crate + workspace tests, four-game replay-check/fixture-check/
   rule-coverage, boundary-check, inverse `cargo tree`, doc-link + catalog-doc).
2. Diff discipline -> changed-file audit proves zero unauthorized golden/fixture/
   export diffs (§7.4); every failure was classified before fixing.
3. Status reconciliation -> `specs/README.md` `8C-R3` row reads `Done` with
   completion date/evidence; `8C-R4` and Gate 18 rows unchanged (grep-proof).

## What to Change

### 1. Consolidate the characterization report

Reconcile every matrix cell, sub-surface, hash/visibility/RNG result, exception,
and N/A in
`reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`;
record the §7.1 command results and the changed-file inventory. No unowned
cleanup bucket.

### 2. Flip the tracker

After every §6 exit row passes, change only the `8C-R3` row in `specs/README.md`
to `Done` with completion date and evidence pointer. Leave `8C-R4` and Gate 18
pending.

## Files to Touch

- `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md` (modify; created by 8CR3PUBCOOASY-001)
- `specs/README.md` (modify)

## Out of Scope

- Any production code/test/fixture/trace change (verification + status only).
- Touching `8C-R4`, Gate 18, `docs/ROADMAP.md`, or any web/catalog surface.
- Blessing any unauthorized golden/fixture/export diff (a diff is a stop
  condition).

## Acceptance Criteria

### Tests That Must Pass

1. The full spec §7.1 command set (fmt, clippy, `cargo test --workspace`, the
   four games' `replay-check --all` / `fixture-check` / `rule-coverage`,
   `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert
   game-test-support`, `node scripts/check-doc-links.mjs`,
   `node scripts/check-catalog-docs.mjs`).
2. Changed-file audit shows zero unauthorized golden/fixture/export diffs.
3. `specs/README.md` `8C-R3` reads `Done`; `8C-R4`/Gate 18 unchanged.

### Invariants

1. The status flip is last and gated on all §6 exit rows passing.
2. No unauthorized artifact diff exists; no test was deleted or weakened.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the characterization report and `specs/README.md` and exercises the acceptance suite composed by the migration and register tickets, adding no test file.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
2. `cargo run -p replay-check -- --game plain_tricks --all && cargo run -p fixture-check -- --game plain_tricks && cargo run -p rule-coverage -- --game plain_tricks` (and flood_watch / frontier_control / event_frontier)
3. `bash scripts/boundary-check.sh && cargo tree --workspace -e normal --invert game-test-support && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs` — the full §7.1 set is the correct boundary because the capstone's scope IS the spec's exit criteria.
