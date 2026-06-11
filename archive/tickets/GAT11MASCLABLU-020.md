# GAT11MASCLABLU-020: Capstone — admission evidence and status reconciliation

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (docs/status) — new `games/masked_claims/docs/{GAME-IMPLEMENTATION-ADMISSION.md,PUBLIC-RELEASE-CHECKLIST.md}`; modifies `progress.md`, `specs/README.md`, `specs/gate-11-masked-claims-bluff-reaction-proof.md` (no Rust/behavior surface)
**Deps**: GAT11MASCLABLU-013, GAT11MASCLABLU-015, GAT11MASCLABLU-016, GAT11MASCLABLU-019

## Problem

Once every implementation, evidence, and web ticket has landed, the gate must produce its admission evidence (`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`) and reconcile status: flip the `specs/README.md` Gate 11 row and the spec Status to `Done`, update `progress.md`, and confirm the atlas reflects the realized reaction-window use. This ticket introduces no production logic — it exercises and records the pipeline the prior tickets composed.

## Assumption Reassessment (2026-06-10)

1. All prior tickets (GAT11MASCLABLU-001–019) land the behavior, evidence, tools, and web surfaces; this capstone gates on the leaf set (bot docs GAT11MASCLABLU-013, native tools/CI GAT11MASCLABLU-015, remaining docs GAT11MASCLABLU-016, web smoke GAT11MASCLABLU-019, whose transitive Deps cover the full chain). Templates `templates/GAME-IMPLEMENTATION-ADMISSION.md` and `templates/PUBLIC-RELEASE-CHECKLIST.md` are confirmed present.
2. Spec §Documentation-updates: `specs/README.md` flips Gate 11 to `Done` only after exit criteria pass with evidence; `progress.md` and root `README.md` update after implementation; do NOT edit `docs/ROADMAP.md` as a progress diary. The `specs/README.md` Gate 11 row currently reads `Planned` (confirmed line 47). The root `README.md` "current official games are" list is reconciled in GAT11MASCLABLU-019 (the mechanically-checked catalog surface), so this capstone does not re-edit it.
3. Cross-artifact boundary under audit: the `specs/README.md` index row and the spec's own Status are the status-reconciliation surfaces; the `docs/MECHANIC-ATLAS.md` §10B realized-reaction-window row was set in GAT11MASCLABLU-002 and is only *confirmed* here (grep, no edit) to avoid a shared-file overlap.
4. FOUNDATIONS §6 (a game is not done without docs/traces/replay/visibility/bot/benchmarks/coverage) and §11 (bounded, reviewable agent output) are the principles under audit — the admission doc records the evidence that each exit criterion passed.

## Architecture Check

1. A verification-only capstone that records evidence and reconciles status keeps the `Done` flip gated on the full exit-criteria run rather than scattered across implementation tickets.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; this ticket adds no code, only docs and status edits.

## Verification Layers

1. Full exit criteria re-run -> `cargo test --workspace`; `cargo run -p simulate -- --game masked_claims --games 1000`; `cargo run -p replay-check -- --game masked_claims --all`; `cargo run -p fixture-check -- --game masked_claims`; `cargo run -p rule-coverage -- --game masked_claims`; `bash scripts/boundary-check.sh`; `npm --prefix apps/web run smoke:wasm && smoke:ui && smoke:e2e`; `node scripts/{check-doc-links,check-catalog-docs,check-player-rules,check-outcome-explanations}.mjs`.
2. Status flips applied -> grep `specs/README.md` Gate 11 row reads `Done`; spec Status reads `Done`.
3. Atlas realized reaction-window use -> grep `docs/MECHANIC-ATLAS.md` §10B (confirm only; set in GAT11MASCLABLU-002).

## What to Change

### 1. `games/masked_claims/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Instantiate from `templates/GAME-IMPLEMENTATION-ADMISSION.md`: the admission evidence, the command transcript with pass/fail, and any unresolved issues.

### 2. `games/masked_claims/docs/PUBLIC-RELEASE-CHECKLIST.md`

Instantiate from `templates/PUBLIC-RELEASE-CHECKLIST.md`.

### 3. Status reconciliation

`progress.md` update; `specs/README.md` Gate 11 row → `Done`; spec `specs/gate-11-masked-claims-bluff-reaction-proof.md` Status → `Done`.

## Files to Touch

- `games/masked_claims/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/masked_claims/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `progress.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-11-masked-claims-bluff-reaction-proof.md` (modify)

## Out of Scope

- Any implementation, test, tool, or web change (all in GAT11MASCLABLU-001–019); this ticket exercises them, it does not modify them.
- Editing `docs/ROADMAP.md` as a progress diary (forbidden by the spec).
- The root `README.md` games-list and `docs/MECHANIC-ATLAS.md` rows (owned by GAT11MASCLABLU-019 and GAT11MASCLABLU-002 respectively).

## Acceptance Criteria

### Tests That Must Pass

1. Every spec §Exit-criteria / §Acceptance-evidence command re-runs green (native suite, simulation, replay/fixture/rule-coverage, boundary-check, web smokes, doc/catalog/player-rules/outcome-explanation guards).
2. `specs/README.md` Gate 11 row and the spec Status both read `Done`.
3. `node scripts/check-doc-links.mjs` passes with the two new admission docs.

### Invariants

1. The `Done` flip is gated on the full exit-criteria run passing with evidence (FOUNDATIONS §6).
2. `docs/ROADMAP.md` is not edited as a progress diary.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it re-runs the existing exit-criteria pipeline and records the evidence. No new production tests.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh`
2. `cargo run -p simulate -- --game masked_claims --games 1000 && cargo run -p replay-check -- --game masked_claims --all && cargo run -p fixture-check -- --game masked_claims && cargo run -p rule-coverage -- --game masked_claims`
3. `npm --prefix apps/web run smoke:e2e && node scripts/check-catalog-docs.mjs && node scripts/check-player-rules.mjs && node scripts/check-outcome-explanations.mjs` — the full exit-criteria boundary; the capstone passes only when every prior ticket's surface is green.

## Outcome

Created the Masked Claims implementation admission and public release checklist, flipped the Gate 11 spec and `specs/README.md` status to `Done`, updated `progress.md`, corrected the root README status to Gate 11 complete, and confirmed the mechanic atlas records the realized reaction-window use with no open promotion debt.

Verification:

1. `cargo test --workspace`
2. `bash scripts/boundary-check.sh`
3. `cargo run -p simulate -- --game masked_claims --games 1000`
4. `cargo run -p replay-check -- --game masked_claims --all`
5. `cargo run -p fixture-check -- --game masked_claims`
6. `cargo run -p rule-coverage -- --game masked_claims`
7. `npm --prefix apps/web run smoke:wasm`
8. `npm --prefix apps/web run smoke:ui`
9. `npm --prefix apps/web run smoke:e2e`
10. `node scripts/check-doc-links.mjs`
11. `node scripts/check-catalog-docs.mjs`
12. `node scripts/check-player-rules.mjs`
13. `node scripts/check-outcome-explanations.mjs`
