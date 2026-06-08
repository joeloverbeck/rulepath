# GAT91SECDRACOM-018: Capstone — acceptance evidence + MECHANIC-ATLAS §10B first-use + status reconciliation

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (verification + docs/status reconciliation) — `specs/README.md`, `docs/MECHANIC-ATLAS.md`, `progress.md` (modify) and the spec's own Status. Exercises every prior ticket end-to-end; introduces no production logic.
**Deps**: GAT91SECDRACOM-010, GAT91SECDRACOM-011, GAT91SECDRACOM-012, GAT91SECDRACOM-016, GAT91SECDRACOM-017

## Problem

Gate 9.1 needs a single closeout that runs the full acceptance-evidence suite end-to-end, records the `simultaneous commitment/reveal` mechanic as a realized first official local use in the atlas, and reconciles status surfaces (`specs/README.md` index, spec Status, `progress.md`). This is the gate's `Done`-flip, gated on every prior ticket passing.

## Assumption Reassessment (2026-06-08)

1. `archive/tickets/GAT9TOKBAZBRO-018.md` ("Capstone — acceptance evidence + MECHANIC-ATLAS first-use + status reconciliation") is the direct precedent. The deps here are the leaf set covering the full chain: golden traces (010), benchmarks/gate-2 (011), tools (012), web/e2e/gate-1/catalog README (016), trailing docs (017). 016 transitively covers 013/014/015; 012 covers 008/010; so this set reaches every prior ticket.
2. `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register reads `_None_` (verified line 200) and §10B already carries a `simultaneous commitment/reveal | secret_draft …` candidate row (verified line 208). Per the spec's §Documentation-updates (as updated by reassessment finding M3), this ticket CONVERTS that existing §10B row from candidate pressure to realized first official local use — it does NOT add a duplicate note, and does NOT promote (first use stays local). Confirm §10A remains `_None_` after implementation.
3. `specs/README.md` index has a Gate 9 row (Stage 7, Done) and a Gate 10 row (Stage 9, Not started); the spec's §Documentation-updates directs adding a Gate 9.1 row (Stage 8) between them, status `Planned` on acceptance then `Done` after evidence. This ticket performs the `Done` flip (the spec itself / a planning step adds the `Planned` row).
4. §4 (atlas process) + §11 (evidence coverage, open-debt closure before next gate) are the motivating principles: restate before trusting spec — no `game-stdlib` promotion occurs; the atlas records first-use; §10A stays `_None_` so Gate 10 is not blocked by open promotion debt. All acceptance evidence (native tests, traces, replay, fixture, rule-coverage, benches, web smoke, e2e, boundary, doc-links, catalog-docs) passes with stable hashes and no hidden-choice leak.
5. Capstone constraint: introduces no new production logic; Files to Touch are the status-reconciliation doc surfaces only (the §Ticket-shapes `Done`-flip default), not the upstream tickets' files. ROADMAP.md is NOT edited as a progress diary (spec §Documentation-updates).

## Architecture Check

1. A single evidence+status capstone gated on the leaf set is cleaner than scattering the `Done` flip across tickets: it guarantees the index/spec/atlas/progress surfaces flip only once all evidence passes, and keeps the causal narrative (game shipped → status reconciled) intact.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; no `game-stdlib` promotion (atlas first-use only, §4); `engine-core` noun-freedom re-verified via boundary-check as part of acceptance.

## Verification Layers

1. Native evidence -> `cargo test -p secret_draft` + `cargo test --workspace` pass.
2. Pipeline evidence -> `simulate --games 1000`, `replay-check --all`, `fixture-check`, `rule-coverage` for `secret_draft` pass.
3. Bench evidence -> `cargo bench -p secret_draft -- legal_actions` + bench-report at smoke floors pass.
4. Web evidence -> `smoke:wasm`, `smoke:ui`, `smoke:preview`, `smoke:e2e` pass; `secret-draft.smoke.mjs` no-leak/a11y passes.
5. Boundary + docs -> `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs` pass.
6. Status reconciliation -> grep-proof: `specs/README.md` Gate 9.1 row `Done`; spec Status `Done`; §10B row converted to first-use; §10A still `_None_`; `progress.md` updated.

## What to Change

### 1. Run + record acceptance evidence

Execute the full acceptance-evidence suite (spec §Acceptance evidence) end-to-end and record results; re-derive any expected counts from fixtures at run time.

### 2. `docs/MECHANIC-ATLAS.md`

Convert the existing §10B `simultaneous commitment/reveal` row from candidate pressure to realized first official local use for `simultaneous commitment/reveal + visible draft-pool removal` (keep local; do not promote). Confirm §10A remains `_None_`.

### 3. `specs/README.md`

Flip the Gate 9.1 row status to `Done` (after the `Planned` row exists).

### 4. Spec Status + `progress.md`

Flip `specs/gate-9-1-secret-draft-commitment-reveal.md` Status to `Done`; update `progress.md` to record Gate 9.1 completion. Do NOT edit `docs/ROADMAP.md`.

## Files to Touch

- `specs/README.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)
- `progress.md` (modify)
- `specs/gate-9-1-secret-draft-commitment-reveal.md` (modify — Status flip only)

## Out of Scope

- Any production logic, renderer, or test change (exercised, not modified, here).
- Root `README.md` / `apps/web/README.md` catalog entries (landed in GAT91SECDRACOM-016).
- `docs/ROADMAP.md` edits (ladder law, not a progress diary).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` and all four `secret_draft` CLI tools pass; `cargo bench -p secret_draft -- legal_actions` runs.
2. `npm --prefix apps/web run smoke:wasm && smoke:ui && smoke:preview && smoke:e2e` pass.
3. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs` pass.

### Invariants

1. All ROADMAP §11 exit lines (incl. the deferred simultaneous-choice + pending-seat rows) are met with evidence and no hidden-choice leak (§11).
2. §10A promotion-debt register stays `_None_`; no `game-stdlib` promotion — Gate 10 is not blocked by open debt (§4/§11).

## Test Plan

### New/Modified Tests

1. `None — capstone/acceptance ticket; it runs the existing suites and reconciles status surfaces, introducing no new tests or production logic.`

### Commands

1. `cargo test --workspace && cargo run -p simulate -- --game secret_draft --games 1000 && cargo run -p replay-check -- --game secret_draft --all && cargo run -p fixture-check -- --game secret_draft && cargo run -p rule-coverage -- --game secret_draft`
2. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:preview && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
3. The full suite is the correct boundary for a capstone — it exercises every prior ticket end-to-end and gates the `Done` flip on that evidence.
