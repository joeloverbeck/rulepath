# GAT7DRALITCOM-022: Capstone — exit criteria, mechanic-atlas finalize, status & picker Done-flip

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — verification + status/docs finalize only (`docs/MECHANIC-ATLAS.md` as-built finalize, `specs/gate-7-draughts-lite-compound-action-tree.md` Status → Done, `specs/README.md` index row flip). Introduces no production logic.
**Deps**: 013, 020, 021

## Problem

Gate 7 is accepted only when its exit criteria pass end-to-end. This capstone exercises the full pipeline the prior tickets composed (no new production logic), finalizes the `docs/MECHANIC-ATLAS.md` movement/capture/forced-continuation row to its as-built outcome, flips the spec Status to `Done`, and updates the `specs/README.md` index row from `Not started` to `Done`, admitting the next gate.

## Assumption Reassessment (2026-06-07)

1. Every prior ticket's surface exists: rules/action-tree/validate-apply/effects (005–008), view (009), replay (010), bots (012), tests (013), traces/fixture (014), benches (015), WASM (016), tools (017), web + a11y (018/019), CI (020), docs (021). `docs/MECHANIC-ATLAS.md` was edited in GAT7DRALITCOM-002 (reopen decision); this ticket finalizes the `draughts_lite` "movement/capture/forced continuation" row to the as-built outcome. `specs/README.md` currently shows `| 5 | Gate 7 | draughts_lite — not yet specced | Not started |` (verified line 33).
2. The exit criteria are fixed by `docs/ROADMAP.md` §9 (action trees in CLI+web; forced continuations replay; UI guides path construction; baseline bot follows forced rules; legal-tree + bot benchmarks) and the spec §R23 acceptance set; `specs/README.md` requires flipping to `Done` only after the exit-criteria section is satisfied with evidence.
3. Cross-artifact boundary under audit: this ticket touches no code — it runs the acceptance commands across crates/tools/web and updates two status docs + one atlas row. It must not modify any upstream ticket's production files. The atlas row finalize is sequential after GAT7DRALITCOM-002 (not a parallel edit).
4. FOUNDATIONS §6/§11 motivate this ticket: restate before flipping — the gate is `Done` only when tests/traces/sims/benchmarks/web smoke/boundary/doc-link checks pass; status must not claim completion the evidence does not support (spec §R20 "Do not introduce misleading gate-status claims").

## Architecture Check

1. A single acceptance capstone that re-runs the exit-criteria commands (rather than scattering the `Done`-flip into an implementation ticket) keeps completion gated on evidence and gives one place to read the gate's acceptance state.
2. No backwards-compatibility shims; status/doc finalize only.
3. `engine-core` untouched (§3); this ticket exercises, it does not modify, the pipeline.

## Verification Layers

1. Rust acceptance -> `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.
2. Game pipeline -> `simulate`/`replay-check`/`fixture-check`/`rule-coverage --game draughts_lite` all pass; `bash scripts/boundary-check.sh`; `node scripts/check-doc-links.mjs`.
3. Web acceptance -> `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:e2e && npm --prefix apps/web run build`.
4. Benchmarks -> `cargo bench -p draughts_lite -- --test` runs; thresholds documented.
5. Status integrity -> grep-proof: `specs/README.md` Gate 7 row reads `Done` and points at the spec file; the spec Status reads `Done`; the atlas row reflects the as-built primitive outcome.

## What to Change

### 1. Run exit-criteria acceptance

Execute the full acceptance command set (below) and record pass/fail; the gate flips only on all-pass.

### 2. Finalize atlas + status

Finalize the `docs/MECHANIC-ATLAS.md` `draughts_lite` movement/capture/forced-continuation row (and the board-space row outcome) to the as-built result; set the spec `Status` to `Done`; flip the `specs/README.md` Gate 7 row to `Done` with a link to the spec, admitting Gate 8.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify — as-built finalize of the `draughts_lite` row)
- `specs/gate-7-draughts-lite-compound-action-tree.md` (modify — Status → Done)
- `specs/README.md` (modify — Gate 7 index row → Done)

## Out of Scope

- Any production code or test change (owned by GAT7DRALITCOM-004–020).
- Authoring game docs (GAT7DRALITCOM-021).
- The initial primitive-pressure reopen decision (GAT7DRALITCOM-002; this only finalizes the as-built row).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.
2. `cargo run -p simulate -- --game draughts_lite --games 1000 && cargo run -p replay-check -- --game draughts_lite --all && cargo run -p fixture-check -- --game draughts_lite && cargo run -p rule-coverage -- --game draughts_lite`.
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:e2e && npm --prefix apps/web run build && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`.

### Invariants

1. The gate flips to `Done` only with all exit-criteria evidence passing (FOUNDATIONS §6; spec §R23; `specs/README.md` admission rule).
2. Status docs make no completion claim the evidence does not support (FOUNDATIONS §11; spec §R20).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it runs existing crate/tool/web checks and updates status docs. No new test or production file.`

### Commands

1. `cargo test --workspace && cargo run -p simulate -- --game draughts_lite --games 1000 && cargo run -p replay-check -- --game draughts_lite --all`
2. `cargo run -p fixture-check -- --game draughts_lite && cargo run -p rule-coverage -- --game draughts_lite && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
3. The full cross-surface acceptance run is the correct boundary for a capstone — it exercises every prior ticket end-to-end without modifying their files.

## Completion Evidence

Completed 2026-06-07.

Outcome:

1. Finalized `docs/MECHANIC-ATLAS.md` to the Gate 7 as-built board-space promotion outcome.
2. Flipped `specs/gate-7-draughts-lite-compound-action-tree.md` to `Done`.
3. Flipped the Gate 7 row in `specs/README.md` to `Done` and linked the active spec.
4. Fixed current Rust 1.93 clippy hygiene required by the capstone `-D warnings` lane:
   `Parity::of` uses `is_multiple_of`, `PieceId::new` uses inclusive-range
   containment, and the replay-check helper passes a slice directly.

Verification:

1. `cargo fmt --all --check` — pass.
2. `cargo clippy --workspace --all-targets -- -D warnings` — pass.
3. `cargo test --workspace` — pass.
4. `cargo run -p simulate -- --game draughts_lite --games 1000` — pass.
5. `cargo run -p replay-check -- --game draughts_lite --all` — pass.
6. `cargo run -p fixture-check -- --game draughts_lite` — pass.
7. `cargo run -p rule-coverage -- --game draughts_lite` — pass.
8. `npm --prefix apps/web run smoke:wasm` — pass.
9. `npm --prefix apps/web run smoke:e2e` — pass.
10. `npm --prefix apps/web run build` — pass.
11. `bash scripts/boundary-check.sh` — pass.
12. `node scripts/check-doc-links.mjs` — pass.
13. `cargo bench -p draughts_lite -- --test` — pass; all benchmark rows reported `pass=true`.
