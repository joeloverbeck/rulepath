# GAT192MELLEDSET-004: Docs reconcile + Gate 19.2 `Done`-flip capstone

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — game-local docs + `specs/README.md` index + spec Status
**Deps**: GAT192MELLEDSET-001, GAT192MELLEDSET-002, GAT192MELLEDSET-003

## Problem

Once the `last_settlement` projection (001), WASM/TS bridge (002), and web panel
(003) land, the game docs must record the new projection and its evidence, and the
gate must be closed out. This capstone reconciles `RULE-COVERAGE.md` (map the
projection to `ML-VIS-006` / `ML-SCORE-*`), `UI.md` (the settlement surface), and
`GAME-EVIDENCE.md` (re-run evidence), runs the full §7 verification set, and flips
Gate 19.2 to `Done` (spec §6.5, §6.6, §7, §8).

## Assumption Reassessment (2026-06-27)

1. The docs to reconcile all exist: `games/meldfall_ledger/docs/RULE-COVERAGE.md`,
   `UI.md`, `GAME-EVIDENCE.md`. `RULE-COVERAGE.md` is consumed by `tools/rule-coverage`
   (`cargo run -p rule-coverage -- --game meldfall_ledger`); since this gate adds no
   new rule IDs (the projection maps to existing `ML-VIS-006` and `ML-SCORE-001..007`,
   `RULES.md:92,143-149`), the update is a mapping note, not a new-rule row — no
   `rule-coverage` red window opens.
2. The Gate 19.2 row already exists in `specs/README.md` (line 110, Status `Planned`)
   — per the `/reassess-spec` I2 disposition this is a completion-time Status flip to
   `Done`, not a new row. The spec's own §1 Header Status field
   (`specs/gate-19-2-meldfall-ledger-settlement-detail-projection.md`) flips in tandem.
3. Cross-artifact boundary under audit: this ticket exercises but does not modify
   the implementation surfaces of 001/002/003 (`tickets/_TEMPLATE.md` capstone rule).
   Its own write surfaces are the three game docs plus the two status-reconciliation
   surfaces (`specs/README.md` row, spec Header Status). No production code changes.
4. FOUNDATIONS §6 evidence-coverage restated: an official-game change must carry
   updated docs + re-run evidence (tests, traces, replay, no-leak, simulations,
   benchmarks). `GAME-EVIDENCE.md` records the §7 commands re-run at closeout with
   pass status; `RULE-COVERAGE.md` maps the projection to its authorizing rules.
5. §11 determinism + no-leak closeout: the capstone confirms `replay-check
   --game meldfall_ledger --all` is byte-identical (state_hash unchanged per 001)
   and the `a11y-noleak` / `meldfall-ledger` smokes pass — the deterministic-replay
   and no-leak enforcement surfaces this gate touches, re-verified end-to-end before
   the `Done` flip.

## Architecture Check

1. A single trailing docs-reconcile + status-flip capstone is cleaner than
   scattering doc edits across 001/002/003: the `RULE-COVERAGE.md` / `UI.md` /
   `GAME-EVIDENCE.md` updates and the `Done` flip all require the full
   implementation to exist coherently first, so they land atomically once.
2. No backwards-compatibility shim; docs/status only.
3. `engine-core` untouched; no `game-stdlib` change; no behavior moves to docs.

## Verification Layers

1. Doc fidelity: `RULE-COVERAGE.md` maps the projection to `ML-VIS-006` /
   `ML-SCORE-*`; `UI.md` documents the settlement surface -> `cargo run -p
   rule-coverage -- --game meldfall_ledger` green + manual review.
2. Evidence freshness: `GAME-EVIDENCE.md` records the §7 commands re-run with pass
   status -> grep-proof of the recorded commands + their reproduction.
3. Determinism / no-leak closeout: full §7 set green -> `replay-check`,
   `fixture-check`, `simulate`, `smoke:ui`, `smoke:effects`, `a11y-noleak` runs.
4. Status reconciliation: `specs/README.md` Gate 19.2 row and the spec Header read
   `Done` -> grep-proof.

## What to Change

### 1. `RULE-COVERAGE.md`

Map the `last_settlement` projection to its authorizing rules (`ML-VIS-006`,
`ML-SCORE-001..007`); no new rule rows.

### 2. `UI.md`

Document the persistent settlement surface: the round-end reason and the per-seat
tabled-positive / in-hand-penalty / delta / cumulative / rank breakdown sourced
from `view.last_settlement`.

### 3. `GAME-EVIDENCE.md`

Record the §7 verification commands re-run at closeout with their pass status and
the no-leak / replay-parity evidence.

### 4. `Done`-flip (`specs/README.md`, spec Header)

Flip the Gate 19.2 `specs/README.md` row Status (line 110) and the spec's §1 Header
Status field from `Planned` to `Done`, with a one-line completion note.

## Files to Touch

- `games/meldfall_ledger/docs/RULE-COVERAGE.md` (modify) — map projection to `ML-VIS-006`/`ML-SCORE-*`
- `games/meldfall_ledger/docs/UI.md` (modify) — settlement surface
- `games/meldfall_ledger/docs/GAME-EVIDENCE.md` (modify) — re-run evidence
- `specs/README.md` (modify) — Gate 19.2 row Status → `Done`
- `specs/gate-19-2-meldfall-ledger-settlement-detail-projection.md` (modify) — Header Status → `Done`

## Out of Scope

- Any production-code change to 001/002/003 surfaces (this ticket exercises, not modifies).
- Any `RULES.md` rule-text change (spec §3.2) — `RULES.md` is unchanged.
- New rule IDs or a rule-coverage schema change — the projection maps to existing rules.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game meldfall_ledger` — green with the mapped projection.
2. `cargo run -p replay-check -- --game meldfall_ledger --all` and `cargo run -p
   fixture-check -- --game meldfall_ledger` — byte-identical / green.
3. `cargo run -p simulate -- --game meldfall_ledger --games 1000`,
   `npm --prefix apps/web run smoke:ui`, `npm --prefix apps/web run smoke:effects`,
   `node scripts/check-catalog-docs.mjs` — all green.
4. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace` — Gate 0 green.

### Invariants

1. `RULES.md` is unchanged; `RULE-COVERAGE.md` maps the projection to `ML-VIS-006`
   and `ML-SCORE-*`.
2. The Gate 19.2 `specs/README.md` row and the spec Header both read `Done`; no
   replay/hash change was required (or, if it was, an accepted ADR + migration note
   is linked first per spec §8).

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named
   docs/status surfaces and exercises the prior tickets' acceptance suite (§7),
   adding no test file.`

### Commands

1. `cargo run -p rule-coverage -- --game meldfall_ledger`
2. `cargo run -p replay-check -- --game meldfall_ledger --all && cargo run -p fixture-check -- --game meldfall_ledger`
3. `cargo run -p simulate -- --game meldfall_ledger --games 1000 && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects && node scripts/check-catalog-docs.mjs`
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-27

Reconciled the capstone docs and status surfaces after the `last_settlement`
projection, WASM/TypeScript bridge, and web panel landed:

- `RULE-COVERAGE.md` now maps the persistent settlement projection to
  `ML-VIS-006` and `ML-SCORE-002` through `ML-SCORE-007`.
- `UI.md` documents the Rust-owned `view.last_settlement` panel, its fields,
  and the no-TypeScript-settlement-math boundary.
- `GAME-EVIDENCE.md` records the Gate 19.2 receipt and verification commands.
- `specs/README.md` and the Gate 19.2 spec header now mark the gate `Done`.
- `RULES.md` was unchanged.

Verification passed:

- `cargo fmt --all --check`
- `cargo run -p rule-coverage -- --game meldfall_ledger`
- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo run -p fixture-check -- --game meldfall_ledger`
- `cargo run -p simulate -- --game meldfall_ledger --games 1000 --action-cap 20000`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `node scripts/check-catalog-docs.mjs`
- `node apps/web/e2e/meldfall-ledger.smoke.mjs`
- `node apps/web/e2e/a11y-noleak.smoke.mjs`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

The simulator command used an explicit `--action-cap 20000` as the bounded
multi-round verifier guard; it completed 1000/1000 games with
`bounded_nonterminal_at_cap=0`. No replay/hash migration, ADR, or production code
change was required for this capstone.
