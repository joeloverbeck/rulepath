# GAT13FROCONASY-017: Capstone — admission docs, status hygiene, and gate closeout

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — documentation / status reconciliation (`games/frontier_control/docs/{GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md`; `specs/README.md`; `specs/gate-13-frontier-control-asymmetric-area-control-proof.md` Status; `progress.md`)
**Deps**: GAT13FROCONASY-010, GAT13FROCONASY-011, GAT13FROCONASY-013, GAT13FROCONASY-016

## Problem

The gate closes with the final admission evidence and status hygiene: the two remaining per-game docs (`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`), the `specs/README.md` index `Done` flip and the spec's own Status flip (only after exit criteria pass with evidence), and the `progress.md` update. This ticket introduces no production logic; it records that the prior tickets composed a passing gate.

## Assumption Reassessment (2026-06-11)

1. `templates/GAME-IMPLEMENTATION-ADMISSION.md` and `templates/PUBLIC-RELEASE-CHECKLIST.md` are the instantiation sources; `specs/README.md` carries the Gate 13 index row (verified `Planned`, pointing at this spec) and Gate 12 reads `Done`; `progress.md` exists with per-gate entries (verified). The leaf tickets (GAT13FROCONASY-010 bench, -011 bot docs, -013 tools/CI, -016 browser smoke) transitively gate every prior implementation ticket.
2. Spec §Documentation-updates directs the index `Planned`→`Done` flip only after exit criteria pass, the `progress.md`/root-README updates after evidence, and **no** `docs/ROADMAP.md` progress edit; §Sequencing confirms Gate 12 `Done` with no open promotion debt.
3. Cross-artifact boundary under audit: this capstone owns the status-reconciliation surfaces (`specs/README.md` index row, spec Status, `progress.md`) — distinct from the root-README catalog list, which co-lands with the web-smoke ticket (GAT13FROCONASY-016) per the validator-consumed-catalog-docs rule; the atlas first-use rows were already recorded in GAT13FROCONASY-002.
4. FOUNDATIONS §6 (evidence-heavy) and §11 are under audit: the `Done` flip is gated on the full acceptance-evidence set passing (tests, traces, replay/fixture/rule-coverage, benchmarks, simulations, boundary check, browser smoke) — browser playability alone is not "done".

## Architecture Check

1. A single trailing capstone for status hygiene (vs scattering Status flips across tickets) keeps the `Done` marker gated on the complete leaf set and gives one auditable closeout diff.
2. No backwards-compatibility aliasing/shims.
3. `engine-core`/`game-stdlib` untouched; docs + status surfaces only.

## Verification Layers

1. Exit-criteria pass (§6/§11) -> full-pipeline verification (the spec §Acceptance-evidence command set re-run green before the `Done` flip).
2. Status hygiene -> codebase grep-proof (`specs/README.md` Gate 13 row reads `Done`; spec Status `Done`; `progress.md` carries the frontier_control entry).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Admission + release docs

Instantiate `GAME-IMPLEMENTATION-ADMISSION.md` and `PUBLIC-RELEASE-CHECKLIST.md` from templates, recording the passing evidence and unresolved-issue notes.

### 2. Status hygiene

Flip the `specs/README.md` Gate 13 index row to `Done`, flip the spec's own Status to `Done`, and add the frontier_control entry to `progress.md`. Do not edit `docs/ROADMAP.md` as a progress diary.

## Files to Touch

- `games/frontier_control/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/frontier_control/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `specs/README.md` (modify)
- `specs/gate-13-frontier-control-asymmetric-area-control-proof.md` (modify)
- `progress.md` (modify)

## Out of Scope

- Any production logic, Rust behavior, or web rendering (all prior tickets).
- The root `README.md` / `apps/web/README.md` catalog lists (GAT13FROCONASY-016).
- Editing `docs/ROADMAP.md`.

## Acceptance Criteria

### Tests That Must Pass

1. The full spec §Acceptance-evidence command set re-runs green (cargo test/clippy/fmt, simulate, replay-check, fixture-check, rule-coverage, boundary-check, web smoke:wasm/ui/effects/e2e, doc/catalog/player-rules/outcome checks).
2. `node scripts/check-doc-links.mjs` passes with the two new docs in place.
3. `grep -n 'frontier_control' specs/README.md progress.md` confirms the `Done` index row and the progress entry.

### Invariants

1. The `Done` flip happens only after the complete acceptance-evidence set passes (§6/§11) — not on browser playability alone.
2. `docs/ROADMAP.md` is not edited as a progress diary.

## Test Plan

### New/Modified Tests

1. `None — documentation/status-reconciliation ticket; verification is the full re-run of the spec's acceptance-evidence command set named above.`

### Commands

1. `node scripts/check-doc-links.mjs && grep -n 'frontier_control' specs/README.md progress.md`
2. `cargo test --workspace && cargo run -p simulate -- --game frontier_control --games 1000 && cargo run -p replay-check -- --game frontier_control --all && bash scripts/boundary-check.sh && node scripts/check-catalog-docs.mjs`
3. Re-running the full acceptance set is the correct capstone boundary; this ticket adds no logic of its own.
