# GAT6DIRFLI-021: Capstone — mechanic atlas finalize, exit criteria, status & picker exposure

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — governance/status + presentation exposure: `docs/MECHANIC-ATLAS.md` (finalize directional_flip row), `specs/gate-6-directional-flip.md` (Status → Done), `specs/README.md` (Gate 6 row → Done), and the public picker/catalog exposure point in `apps/web` (enable after the release checklist passes). Introduces no new game production logic.
**Deps**: 012, 019, 020

## Problem

Gate 6 exit requires that every exit-criterion and acceptance-evidence row passes, the mechanic atlas reflects the as-built game + extraction decision, public picker exposure happens only after `PUBLIC-RELEASE-CHECKLIST.md` passes, and the spec/index statuses flip to `Done` (spec §14, §15, §18, §19). This capstone exercises the full pipeline the prior tickets composed, performs the status hygiene, and flips public exposure. It adds no new production logic.

## Assumption Reassessment (2026-06-07)

1. All implementation surfaces exist by Deps: tests (012), CI (019), trailing docs incl. `PUBLIC-RELEASE-CHECKLIST.md` (020), and transitively the rules/effects/replay/bots/traces/benches/wasm/renderer/smoke chain. `specs/gate-6-directional-flip.md` currently reads `Status: Planned`; `specs/README.md` Gate 6 row reads `Planned` with a link (both set during the in-session `/reassess-spec` run, 2026-06-07). `docs/MECHANIC-ATLAS.md` §10 has the directional_flip row updated for the *decision* in GAT6DIRFLI-002; this ticket finalizes it for the *as-built* outcome (back-port done / deferred per the decision).
2. Spec §14 (exit criteria mapped to roadmap + official-game contract), §15 (acceptance evidence rows 1–25), §18 (documentation updates incl. the index `Done` flip), and §19 (admission rule: public only after contract evidence + release checklist pass) are authoritative.
3. Cross-artifact boundary under audit: the `specs/README.md` index status convention (`Planned → In progress → Done`, flip to `Done` only after exit criteria pass with evidence), the spec's own Status field, the mechanic-atlas table, and the public picker/catalog exposure point in `apps/web` (the catalog entry registered but gated in GAT6DIRFLI-017). Confirm the exposure mechanism in the as-built `GamePicker.tsx`/catalog before flipping.
4. FOUNDATIONS §12 (stop conditions) motivates the capstone: restate before flipping — the final review must confirm no stop condition remains open (no `engine-core` mechanic noun, no TS legality, no hidden-info leak, no bot bypass, primitive-pressure resolved, no invented benchmark thresholds), per spec §15 row 25 and §16 stop-conditions list. Only then may the game go public and the status flip to `Done`.

## Architecture Check

1. A single capstone that runs the whole evidence suite and performs the atomic status/exposure flips (rather than scattering `Done` flips across tickets) keeps the gate's completion gated on real passing evidence — the `specs/README.md` workflow and the §Ticket-shapes `Done`-flip default.
2. No backwards-compatibility shims; this is verification + status/exposure flips, no new production logic.
3. `engine-core` stays noun-free (final `boundary-check.sh` is part of the acceptance run); the primitive-pressure decision (GAT6DIRFLI-002) is confirmed resolved (§4).

## Verification Layers

1. Full exit-criteria pipeline -> simulation/CLI run + deterministic replay-hash check + benchmark check + browser smoke: re-run the spec §14/§15 evidence set (workspace tests, replay-check `--all`, fixture-check, rule-coverage, simulate, benches, wasm smoke, web e2e, a11y/no-leak, boundary check) — all pass.
2. Stop-conditions clear -> FOUNDATIONS alignment check (§12): final review checklist; unresolved-ADR/stops list empty (spec §15 row 25).
3. Status hygiene -> codebase grep-proof: `specs/gate-6-directional-flip.md` Status reads `Done`; `specs/README.md` Gate 6 row reads `Done`; `docs/MECHANIC-ATLAS.md` directional_flip row reflects the as-built outcome.
4. Exposure-after-checklist -> manual review: public picker exposure is enabled only after `PUBLIC-RELEASE-CHECKLIST.md` passes (spec §19).

## What to Change

### 1. Acceptance run (no new logic)

Execute the spec §14/§15 evidence set end-to-end and record results; this capstone exercises prior tickets, it does not modify their files.

### 2. Mechanic atlas finalize

Update the `docs/MECHANIC-ATLAS.md` directional_flip row(s) to the as-built outcome (promotion back-ported / deferred per the GAT6DIRFLI-002 decision), confirming stage-advancement checks (atlas §11).

### 3. Public exposure & status flips

Enable the public picker/catalog exposure point in `apps/web` (the entry gated in GAT6DIRFLI-017) once `PUBLIC-RELEASE-CHECKLIST.md` passes; flip `specs/gate-6-directional-flip.md` Status → `Done` and the `specs/README.md` Gate 6 row → `Done`.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/gate-6-directional-flip.md` (modify — Status → Done)
- `specs/README.md` (modify — Gate 6 row → Done)
- `apps/web/src/components/GamePicker.tsx` (modify — enable public exposure; exact site as surfaced in the as-built catalog from GAT6DIRFLI-017)

## Out of Scope

- Any new game production logic, tests, or docs content (owned by GAT6DIRFLI-001–020).
- Editing `docs/ROADMAP.md` for progress (spec §17 forbids) or archiving this spec (spec §19; archival is a separate process per `docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace && cargo run -p replay-check -- --game directional_flip --all && cargo run -p fixture-check -- --game directional_flip && cargo run -p rule-coverage -- --game directional_flip` — all pass.
2. `cargo run -p simulate -- --game directional_flip --games 1000 && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && bash scripts/boundary-check.sh` — all pass.
3. `grep -q 'Done' <(grep -i 'Gate 6' specs/README.md)` and the spec Status reads `Done`.

### Invariants

1. The game goes public only after every exit-criterion and the release checklist pass; no §12 stop condition remains open (FOUNDATIONS §12, spec §15 row 25, §19).
2. `engine-core` is mechanic-noun-free and the primitive-pressure decision is resolved (FOUNDATIONS §3/§4).

## Test Plan

### New/Modified Tests

1. `None — capstone/acceptance ticket; it exercises the pipeline the prior tickets built and performs status/exposure flips. Verification is the full re-runnable evidence set above.`

### Commands

1. `cargo test --workspace && cargo run -p replay-check -- --game directional_flip --all && cargo run -p rule-coverage -- --game directional_flip && cargo run -p fixture-check -- --game directional_flip`
2. `cargo run -p simulate -- --game directional_flip --games 1000 && cargo bench -p directional_flip -- legal_actions && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && bash scripts/boundary-check.sh`
3. The full evidence suite is the correct boundary for a capstone — it is the spec's §14/§15 exit set re-run end-to-end before the `Done` flip.
