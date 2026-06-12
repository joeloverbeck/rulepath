# CARACTPRES-010: Closeout — doc amendments, README reconciliation, spec Done-flip

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation and status surfaces only
**Deps**: CARACTPRES-004, CARACTPRES-009 (leaf set; transitively covers 001–003 and 005–008)

## Problem

The spec's contract must outlive its implementation: `docs/UI-INTERACTION.md` and `docs/OFFICIAL-GAME-CONTRACT.md` need the lift-ready amendment text (spec §17) so the component-metadata / deck-surface / staged-construction / copy-guard obligations bind every future game; `apps/web/README.md`'s Shell Surface must name the two shared components; the per-game UI docs need their metadata notes; and the spec + `specs/README.md` index row flip to `Done` only after the distributed exit-criteria evidence (spec §9, carried by tickets 005–009) has passed. This is the cross-cutting docs ticket plus the status-reconciliation capstone duty (acceptance evidence is distributed, not re-run here).

## Assumption Reassessment (2026-06-12)

1. All target docs verified to exist this session: `docs/UI-INTERACTION.md` (§19 acceptance list ends at line ~343), `docs/OFFICIAL-GAME-CONTRACT.md` (UI-metadata payload row at line 20, acceptance line 207), `apps/web/README.md` (Shell Surface section present), `games/event_frontier/docs/UI.md`, `games/flood_watch/docs/UI.md`, `specs/README.md` (index row added at spec authoring, status `Planned`), and the spec itself (Status field in its header).
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §12 (documentation updates), §17 (lift-ready amendment text — applied verbatim-or-tightened at this ticket, "applied at WB9, not before"), §9 criterion 9, and `specs/README.md` §Spec index rule: flip to `Done` only with exit-criteria evidence.
3. Cross-artifact boundary under audit: the law-doc surface. The amendments add acceptance-check lines and a contract clause; they change no existing principle's meaning — so no ADR trigger fires (FOUNDATIONS §13 governs principle *changes*; spec §4 confirms no trigger). If implementation drift since the spec was written makes an amendment read as a semantic change to existing law, STOP and route to an ADR rather than landing it silently.
4. FOUNDATIONS §11 restated ("Tests, traces, simulations, benchmarks, docs, and source notes cover the change"): this ticket is that invariant's docs leg for the whole spec; the Done-flip is gated on citing the distributed evidence (005–009 smoke/guard results, 001–003 crate gates) in the flip commit's ticket notes, not on re-running it here.

## Architecture Check

1. One trailing docs+flip ticket beats per-ticket doc dribble for these surfaces: the README component list, contract clause, and acceptance lines each describe the *completed* set (component names, guard, audit table) and would be stale or speculative if landed earlier; per the decomposition pattern, the `Done`-flip belongs with the ticket gated on all evidence.
2. No backwards-compatibility aliasing/shims: amendment text replaces nothing — it appends acceptance lines and a clause; the spec's §17 text is the source.
3. `engine-core`/`game-stdlib` untouched; markdown only.

## Verification Layers

1. Amendments landed faithfully -> codebase grep-proof: exact-string matches for the §17 acceptance lines in `docs/UI-INTERACTION.md` §19 and the metadata clause in `docs/OFFICIAL-GAME-CONTRACT.md`.
2. Doc integrity -> `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` green.
3. Index/status consistency -> grep-proof: spec header Status and `specs/README.md` row both read `Done`; row format keeps link + scope summary per the index convention.
4. Evidence gating -> manual review: the distributed exit-criteria evidence (per spec §9 mapped to 005–009 / 001–003) is cited before the flip; any unmet criterion blocks this ticket.

## What to Change

### 1. Law-doc amendments (spec §17 text)

Append the four acceptance lines to `docs/UI-INTERACTION.md` §19; add the component-display-metadata clause to `docs/OFFICIAL-GAME-CONTRACT.md`'s UI-metadata surface.

### 2. README + game-doc reconciliation

Add `DeckFlowPanel` and `ActionPathBuilder` to `apps/web/README.md` Shell Surface (the action-audit table landed in 008 — verify it reads coherently beside the new rows); add presentation-metadata notes to `games/event_frontier/docs/UI.md` and `games/flood_watch/docs/UI.md`.

### 3. Status flip

Set the spec's header Status to `Done` and flip the `specs/README.md` row, citing the distributed acceptance evidence.

## Files to Touch

- `docs/UI-INTERACTION.md` (modify)
- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)
- `apps/web/README.md` (modify)
- `games/event_frontier/docs/UI.md` (modify)
- `games/flood_watch/docs/UI.md` (modify)
- `specs/card-and-action-presentation-shared-surfaces.md` (modify — Status field)
- `specs/README.md` (modify)

## Out of Scope

- Re-running the distributed acceptance evidence (owned by 001–009; this ticket cites it).
- Any semantic change to existing law-doc principles (ADR territory — stop condition above).
- Picker/setup polish successor spec (remains a §13 candidate note in the spec, untouched).
- Code or test changes of any kind.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` green.
2. Grep-proofs: §17 amendment lines present in both law docs; Shell Surface names both components; both Status surfaces read `Done`.
3. Full hygiene snapshot at flip time: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` and `npm --prefix apps/web run build` green (the closeout evidence boundary).

### Invariants

1. The spec index is truthful: `Done` appears only with passing distributed evidence (specs/README.md status rule; FOUNDATIONS §11 docs invariant).
2. Law-doc amendments are additive acceptance/contract text, never silent principle changes (§13).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
2. `cargo test --workspace && npm --prefix apps/web run build`
3. Narrow boundary rationale: markdown-only diff; the workspace commands are the flip-time evidence snapshot, not new verification surface.
