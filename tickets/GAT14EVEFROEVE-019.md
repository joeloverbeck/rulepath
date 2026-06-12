# GAT14EVEFROEVE-019: Capstone — admission docs, status hygiene, and gate closeout

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — documentation / status reconciliation (`games/event_frontier/docs/{GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md`; `specs/README.md`; `specs/gate-14-event-frontier-event-complexity-capstone.md` Status; `progress.md`)
**Deps**: GAT14EVEFROEVE-012, GAT14EVEFROEVE-013, GAT14EVEFROEVE-015, GAT14EVEFROEVE-018

## Problem

Gate 14 — the public ladder's capstone — closes with the final admission evidence and status hygiene: the two remaining per-game docs (`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`), the `specs/README.md` index `Done` flip and the spec's own Status flip (only after exit criteria pass with evidence), the `progress.md` update, and the confirmation that `docs/MECHANIC-ATLAS.md` §10A still records `_None_`. This ticket introduces no production logic; it records that the prior tickets composed a passing gate and that public Rulepath stands without private experiments.

## Assumption Reassessment (2026-06-12)

1. The gate's evidence-producing tickets landed: verified the leaf set — benchmarks (ticket 012), bot evidence (ticket 013), native tools + CI (ticket 015), and the browser smoke + catalog reconciliation (ticket 018) — collectively gate every prior implementation ticket; the eleven other per-game docs were authored across tickets 001/002/012/013/014/015/016. The two remaining docs (`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`) complete the thirteen-doc set.
2. The status surfaces are current: verified `specs/README.md` has a Gate 14 row pointing at `gate-14-event-frontier-event-complexity-capstone.md` with status `Planned` (flip to `Done` here), the spec carries a `Status` field (flip to `Done`), `progress.md` exists, and `docs/MECHANIC-ATLAS.md` §10A reads `_None_` (confirm unchanged — the gate's ledger decisions were defer/reject + non-use, opening no promotion debt).
3. Cross-artifact boundary under audit: this is the status-reconciliation surface; per the §Ticket-shapes `Done`-flip default the capstone owns the `specs/README.md` index row, the spec Status, and `progress.md`, but **not** the root `README.md` catalog list (that co-landed with the web-smoke ticket 018 per the validator-consumed-catalog-docs rule). The flip happens only after exit criteria pass with evidence.
4. FOUNDATIONS §6 (evidence-heavy admission) and §11 (full coverage; bounded reviewable output) motivate this ticket. Restated before trusting the spec: the `Done` flip is gated on the full public verification suite passing with `event_frontier` registered; no private content, names, or dependencies exist anywhere; no Gate P / private red-team preparation is started.

## Architecture Check

1. A single trailing capstone for admission docs + status hygiene keeps the `Done` flip atomic and gated on the leaf set, rather than scattering status edits across implementation tickets where they would go stale.
2. No backwards-compatibility aliasing/shims — new docs + status edits only.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; confirms §10A `_None_` (no promotion debt opened by this gate).

## Verification Layers

1. Full public suite (exit criteria) -> `cargo test --workspace`, the five `event_frontier` tool CLIs, `bash scripts/boundary-check.sh`, the doc-link/catalog/player-rules/outcome checks, web build, and smoke layers all pass with `event_frontier` registered.
2. Admission docs -> `GAME-IMPLEMENTATION-ADMISSION.md` + `PUBLIC-RELEASE-CHECKLIST.md` instantiate their templates and record the passing evidence; `node scripts/check-doc-links.mjs` passes.
3. Status hygiene -> `specs/README.md` Gate 14 row and the spec `Status` read `Done`; `progress.md` updated; grep `docs/MECHANIC-ATLAS.md` §10A still `_None_`.
4. No private content -> grep the change set for any private/licensed names or dependencies (none).

## What to Change

### 1. Admission docs

Author `games/event_frontier/docs/GAME-IMPLEMENTATION-ADMISSION.md` (from `templates/GAME-IMPLEMENTATION-ADMISSION.md`) and `PUBLIC-RELEASE-CHECKLIST.md` (from `templates/PUBLIC-RELEASE-CHECKLIST.md`), recording the passing exit-criteria evidence.

### 2. Status hygiene

Flip the `specs/README.md` Gate 14 row to `Done` and the spec's own `Status` to `Done` (only after exit criteria pass with evidence); update `progress.md`; confirm `docs/MECHANIC-ATLAS.md` §10A still records `_None_`. Do **not** edit `docs/ROADMAP.md` as a progress diary; do **not** start any Gate P work.

## Files to Touch

- `games/event_frontier/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/event_frontier/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `specs/README.md` (modify) — Gate 14 row `Done`
- `specs/gate-14-event-frontier-event-complexity-capstone.md` (modify) — `Status: Done`
- `progress.md` (modify)

## Out of Scope

- Any production logic or rule/UI change — this ticket records completion only.
- The root `README.md` / `apps/web/README.md` catalog lists (ticket 018) and `docs/MECHANIC-ATLAS.md` row writes (ticket 002) — this ticket only *confirms* §10A `_None_`.
- Any Gate P / private red-team preparation — forbidden; Gate 14 closes with public Rulepath standing alone.

## Acceptance Criteria

### Tests That Must Pass

1. The full public verification suite passes with `event_frontier` registered: `cargo test --workspace`, the five tool CLIs, `boundary-check.sh`, doc-link/catalog/player-rules/outcome checks, web build, and smoke layers.
2. `node scripts/check-doc-links.mjs` passes with the two new admission docs present.
3. `specs/README.md` Gate 14 row and the spec `Status` read `Done`; `grep -n "_None_" docs/MECHANIC-ATLAS.md` confirms §10A unchanged.

### Invariants

1. The `Done` flip happens only after exit criteria pass with evidence; no private content, names, or dependencies exist anywhere in the change.
2. No Gate P / private red-team work is started; §10A remains `_None_`.

## Test Plan

### New/Modified Tests

1. `None — capstone/status-reconciliation ticket; verification is the full public suite (exercised by prior tickets) plus the doc-link check and status greps.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh`
2. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-player-rules.mjs && node scripts/check-outcome-explanations.mjs`
3. The full-suite run is the correct boundary — the capstone asserts the composed gate passes end to end; it adds no logic of its own.
