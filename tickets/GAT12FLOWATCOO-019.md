# GAT12FLOWATCOO-019: Capstone — admission docs, status hygiene, and gate closeout

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/docs/{GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md` (new); `progress.md`, `specs/README.md`, `specs/gate-12-flood-watch-cooperative-event-pressure-proof.md` (modify — status hygiene). No production logic.
**Deps**: GAT12FLOWATCOO-013, GAT12FLOWATCOO-015, GAT12FLOWATCOO-018

## Problem

The gate needs its closeout: the final two per-game docs (`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`) reflecting the implemented-and-verified state, the repository status hygiene (`progress.md`, the `specs/README.md` index `Done`-flip, the spec's own Status → `Done`), a final confirmation that `docs/MECHANIC-ATLAS.md` §10A still records `_None_` (no promotion debt opened during the gate), and the command transcript / unresolved-issues record. This ticket introduces no new production logic — it exercises and records what the prior tickets composed.

## Assumption Reassessment (2026-06-11)

1. The leaf set is implemented: GAT12FLOWATCOO-018 (web smoke + catalog), GAT12FLOWATCOO-015 (native tools + gate-1 + boundary-check), GAT12FLOWATCOO-013 (bot evidence); their transitive `Deps` cover the full pipeline (001–017). `games/masked_claims/docs/{GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md` are the verified exemplars; templates `templates/GAME-IMPLEMENTATION-ADMISSION.md` + `templates/PUBLIC-RELEASE-CHECKLIST.md` exist.
2. The spec (§Deliverables "Repository docs", Work-breakdown item 14, §Documentation-updates, §Sequencing) fixes: spec-index `Done`-flip, atlas §10A confirmation, `progress.md` + root `README.md` after implementation, catalog closeout surfaces, command transcript, unresolved issues, and "do not edit ROADMAP as progress diary". The root `README.md` catalog/games list is reconciled in GAT12FLOWATCOO-018 (the mechanically-checked catalog surface), so this ticket does **not** touch root `README.md` (avoids a shared-file overlap); `progress.md` is this ticket's.
3. Cross-artifact boundary under audit: the `specs/README.md` Gate 12 row currently reads `Planned` (verified) and points at the spec; the spec's Status field reads `Planned` (verified). Both flip to `Done` only after exit criteria pass with evidence. `docs/MECHANIC-ATLAS.md` §10A reads `_None_` (verified, GAT12FLOWATCOO-002 confirmed it was not opened) — this ticket re-verifies no debt was opened, not re-authors the atlas.
4. FOUNDATIONS §6 (admission evidence + public-release checklist are part of the done contract) and §11 (the change is covered by docs/traces/sims/benchmarks; agent output bounded) motivate this ticket. The §12 stop condition "official games lack docs…" is closed here — every doc/trace/sim/benchmark exists.
5. Enforcement surface: this is the status-reconciliation surface; the `Done`-flip is gated on exit evidence passing (it must not flip while any acceptance command fails). The atlas §10A re-confirmation is the §4 promotion-debt closure check — it must still read `_None_`.

## Architecture Check

1. A single trailing capstone for status hygiene + closeout docs keeps the `Done`-flip gated on the full leaf set and records the gate's evidence in one reviewable diff, rather than scattering status edits across implementation tickets where they would flip prematurely.
2. No backwards-compatibility aliasing/shims; net-new closeout docs + status-line edits.
3. `engine-core`/`game-stdlib` untouched; this ticket introduces no production logic and confirms no promotion debt was opened (§4).

## Verification Layers

1. Exit criteria pass end-to-end -> simulation/CLI + test runs: the full acceptance command set (cargo test, simulate, replay-check, fixture-check, rule-coverage, boundary-check, web smokes, check scripts) passes before the `Done`-flip.
2. Status hygiene -> grep-proof: `specs/README.md` Gate 12 row and the spec Status read `Done`; `progress.md` records the gate.
3. No promotion debt -> grep-proof: `docs/MECHANIC-ATLAS.md` §10A still reads `_None_`.
4. Closeout docs present -> `node scripts/check-doc-links.mjs` passes with all thirteen per-game docs present.

## What to Change

### 1. Closeout docs

Instantiate `games/flood_watch/docs/GAME-IMPLEMENTATION-ADMISSION.md` (the admission evidence: what was built, command transcript, unresolved issues) and `games/flood_watch/docs/PUBLIC-RELEASE-CHECKLIST.md` from their templates, reflecting the implemented-and-verified state.

### 2. Status hygiene

Flip the `specs/README.md` Gate 12 row to `Done`; flip the spec's own Status field to `Done`; update `progress.md` to record the gate. Re-confirm `docs/MECHANIC-ATLAS.md` §10A still reads `_None_` (no edit unless debt exists — it must not). Do not edit `docs/ROADMAP.md`.

## Files to Touch

- `games/flood_watch/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/flood_watch/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `progress.md` (modify)
- `specs/README.md` (modify — Gate 12 row `Done`)
- `specs/gate-12-flood-watch-cooperative-event-pressure-proof.md` (modify — Status `Done`)

## Out of Scope

- Root `README.md` catalog/games list (reconciled in GAT12FLOWATCOO-018 — avoid the shared-file overlap).
- `docs/ROADMAP.md` (must not be edited as a progress diary).
- Any production logic, test, or trace change — the capstone exercises the pipeline, it does not modify it.

## Acceptance Criteria

### Tests That Must Pass

1. The full acceptance command set passes (native tests, `simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`, `boundary-check.sh`, `smoke:wasm`/`smoke:ui`/`smoke:effects`/`smoke:e2e`, `check-catalog-docs`, `check-player-rules`, `check-outcome-explanations`, `check-doc-links`).
2. `specs/README.md` Gate 12 row and the spec Status both read `Done`; `progress.md` records the gate.
3. `docs/MECHANIC-ATLAS.md` §10A still reads `_None_`; all thirteen per-game docs are present and link-checkable.

### Invariants

1. The `Done`-flip happens only after exit criteria pass with evidence; no promotion debt was opened during the gate.
2. `docs/ROADMAP.md` is not edited; root `README.md` is owned by GAT12FLOWATCOO-018.

## Test Plan

### New/Modified Tests

1. `None — capstone/closeout ticket; verification is the full re-runnable acceptance command set plus status-line grep-proofs. No new production logic or tests are introduced.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
2. `cargo run -p simulate -- --game flood_watch --games 1000 && cargo run -p replay-check -- --game flood_watch --all && cargo run -p fixture-check -- --game flood_watch && cargo run -p rule-coverage -- --game flood_watch`
3. `node scripts/check-catalog-docs.mjs && node scripts/check-player-rules.mjs && node scripts/check-outcome-explanations.mjs` — the gate's full closeout boundary; web smokes (`smoke:e2e` etc.) run per GAT12FLOWATCOO-018.
