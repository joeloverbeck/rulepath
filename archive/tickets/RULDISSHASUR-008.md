# RULDISSHASUR-008: Closeout — acceptance evidence + specs/README Done-flip

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — verification-only capstone + `specs/README.md` status reconciliation; no production logic, no code surface.
**Deps**: RULDISSHASUR-007, RULDISSHASUR-001

## Problem

Once the contract, scripts, all nine player docs, the panel, the access points, and the smoke have landed, the feature needs a single closeout: run the spec's exit-criteria command set, record acceptance evidence, and flip the spec's `specs/README.md` index row from `Planned` to `Done`. This capstone introduces no production logic — it exercises the pipeline the earlier tickets composed. Source: `specs/rules-display-shared-surface.md` §11 (acceptance and exit criteria), §10.7 (verification commands).

## Assumption Reassessment (2026-06-09)

1. RULDISSHASUR-001 added the `rules-display-shared-surface.md` row to `specs/README.md` at `Planned`; this ticket flips it to `Done`. The exit-criteria commands resolve to real surfaces: `scripts/check-catalog-docs.mjs`, `scripts/copy-player-rules.mjs` + `scripts/check-player-rules.mjs` (RULDISSHASUR-002), `npm --prefix apps/web run build` / `smoke:e2e` / `smoke:ui`, and `cargo test --workspace`.
2. The exit/acceptance rows are spec §11; the verification command set is spec §10.7.
3. Cross-artifact boundary under audit: this ticket exercises every prior ticket's output (template/docs, scripts, nine docs + assets, panel, access points, smoke) end-to-end without modifying them, and reconciles status only in `specs/README.md` (the index row it owns per the Done-flip default).
4. FOUNDATIONS principle restated: §11 evidence coverage — every exit row must map to a re-runnable command with a recorded pass result, so completion is evidenced, not asserted.

## Architecture Check

1. A single verification-only capstone that re-runs the exit commands and flips the index row keeps completion gated on evidence and avoids scattering the status flip across implementation tickets.
2. No backwards-compatibility shims; the only file change is the index status value.
3. `engine-core`/`game-stdlib` untouched; the capstone inspects, it does not implement.

## Verification Layers

1. All exit rows pass → run the spec §10.7 command set; each command green and mapped to its spec §11 row.
2. Index reconciled → grep-proof the `specs/README.md` `rules-display-shared-surface.md` row reads `Done`.
3. No existing gate weakened → `cargo test --workspace` and the existing web smokes remain green (regression evidence).
4. Evidence recorded → manual: each spec §11 acceptance row is mapped to a re-run command and its result in the closeout summary.

## What to Change

### 1. Run the exit-criteria command set and record acceptance evidence

Execute spec §10.7: `node scripts/check-catalog-docs.mjs`, `node scripts/copy-player-rules.mjs`, `node scripts/check-player-rules.mjs`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:e2e`, `npm --prefix apps/web run smoke:ui`, `cargo test --workspace`. Record each result against the matching spec §11 acceptance row (catalog-complete docs; pilot text; web renders player docs not `RULES.md`; reachable from picker/setup/in-play; deterministic static delivery; no TS legality drift; no hidden-info leak; a11y baseline; IP-safe prose; existing gates intact; permanent docs contract).

### 2. Flip `specs/README.md` index row to `Done`

Update the `rules-display-shared-surface.md` row from `Planned` to `Done` (the spec body itself is not edited by this ticket).

## Files to Touch

- `specs/README.md` (modify — status reconciliation)

## Out of Scope

- Any new production logic, component, script, or doc content (all owned by RULDISSHASUR-001…007).
- Modifying any upstream ticket's files — the capstone exercises them, it does not change them.
- Editing the `specs/rules-display-shared-surface.md` spec body (route any spec defect to `/reassess-spec`).
- Any Rust/engine/WASM change.

## Acceptance Criteria

### Tests That Must Pass

1. Every spec §10.7 command passes: `node scripts/check-catalog-docs.mjs`, `node scripts/copy-player-rules.mjs`, `node scripts/check-player-rules.mjs`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:e2e`, `npm --prefix apps/web run smoke:ui`, `cargo test --workspace`.
2. `node scripts/check-doc-links.mjs` passes after the index edit.
3. Each spec §11 acceptance row has a recorded pass result in the closeout summary.

### Invariants

1. The `specs/README.md` row reads `Done` only because every exit criterion passed with re-run evidence (no silent completion claim — FOUNDATIONS §11).
2. No existing test, smoke, or gate was weakened to reach green (`docs/AGENT-DISCIPLINE.md`).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; the spec §10.7 commands and existing pipeline/smoke coverage named above are the verification, and the only file change is the index status value.`

### Commands

1. `node scripts/check-catalog-docs.mjs && node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && npm --prefix apps/web run smoke:ui`
3. `cargo test --workspace` (regression evidence — expected green because the feature requires no Rust change)

## Outcome

Completed: 2026-06-09

What changed:

- Flipped the `specs/README.md` row for `rules-display-shared-surface.md` from `Planned` to `Done` after rerunning the exit command set.
- Updated `scripts/check-catalog-docs.mjs` so the cross-game `rules-display.smoke.mjs` is treated like other non-game smokes instead of being parsed as a stale catalog game smoke.

Acceptance evidence:

- Catalog-complete docs and permanent docs contract: `node scripts/check-catalog-docs.mjs` passed after the non-game smoke allowlist correction.
- Static player-doc delivery and drift guard: `node scripts/copy-player-rules.mjs` copied 9 catalog games; `node scripts/check-player-rules.mjs` passed.
- Web renders the player docs surface and keeps TypeScript presentation-only: `npm --prefix apps/web run build` passed.
- Picker/setup/in-play reachability, a11y baseline, no hidden-info leak, deterministic static delivery, and no match mutation: `npm --prefix apps/web run smoke:e2e` passed, including `rules-display.smoke.mjs`.
- Existing UI gate intact: `npm --prefix apps/web run smoke:ui` passed.
- Existing Rust/WASM behavior gates intact: `cargo test --workspace` passed.
