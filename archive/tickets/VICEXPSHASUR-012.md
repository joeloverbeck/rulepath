# VICEXPSHASUR-012: Closeout — acceptance evidence + `specs/README` Done-flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — verification-only capstone + `specs/README.md` status reconciliation; no production logic, no code surface.
**Deps**: VICEXPSHASUR-011, VICEXPSHASUR-002, VICEXPSHASUR-001

## Problem

Once the contract (001), checker (002), all nine games' rationales (003–008), the shared panel (009), the web wiring (010), and the browser smoke (011) have landed, the feature needs a single closeout: run the spec's exit-criteria command set, record acceptance evidence against each §14 criterion, and flip the `specs/README.md` index row from `Planned` to `Done`. This capstone introduces no production logic — it exercises the pipeline the earlier tickets composed. Source: `archive/specs/victory-explanation-shared-surface.md` §14, §15.8.

## Assumption Reassessment (2026-06-09)

1. 001 added the `victory-explanation-shared-surface` row to `specs/README.md` at `Planned`; this ticket flips it to `Done` (a create-then-modify on the 001 row). The exit-criteria commands resolve to real surfaces: `node scripts/check-outcome-explanations.mjs` (002), `node scripts/check-catalog-docs.mjs`, `cargo test --workspace`, `cargo run -p replay-check -- --game <g> --all` per game, and `npm --prefix apps/web run build` / `smoke:e2e` / `smoke:ui`.
2. The acceptance rows are spec §14 (15 criteria); the verification command set follows the project hygiene gates (`CLAUDE.md` Commands) + the spec's per-game replay checks.
3. Cross-artifact boundary under audit: this ticket exercises every prior ticket's output (contract/docs, checker, nine rationales, panel, wiring, smoke) end-to-end without modifying them, and reconciles status only in `specs/README.md` (the row it owns per the Done-flip default).
4. FOUNDATIONS §11 evidence coverage restated: every §14 acceptance row must map to a re-runnable command with a recorded pass result, so completion is evidenced, not asserted.
5. The no-leak firewall and deterministic replay/hash gates (§11) are *exercised green* here as exit evidence (`smoke:e2e` no-leak set; `replay-check`/`cargo test` determinism); this capstone names those enforcement surfaces and introduces no new leak or nondeterminism path of its own (its only file change is the index status value).

## Architecture Check

1. A single verification-only capstone that re-runs the exit commands and flips the index row keeps completion gated on evidence and avoids scattering the status flip across implementation tickets.
2. No backwards-compatibility shims; the only file change is the `specs/README.md` index status value.
3. `engine-core`/`game-stdlib` untouched; the capstone inspects, it does not implement.

## Verification Layers

1. All exit rows pass → run the full command set; each command green and mapped to its spec §14 criterion.
2. Index reconciled → grep-proof the `specs/README.md` `victory-explanation-shared-surface` row reads `Done`.
3. No existing gate weakened → `cargo test --workspace` and the web smokes remain green (regression evidence).
4. Evidence recorded → manual: each spec §14 acceptance row is mapped to a re-run command and its result in the closeout summary.

## What to Change

### 1. Run the exit-criteria command set and record acceptance evidence

Execute and record against each spec §14 criterion: `cargo test --workspace`; `cargo run -p replay-check -- --game <g> --all` for every catalog game (rationale traces deterministic); `node scripts/check-outcome-explanations.mjs` (catalog-complete rationale contract — now green); `node scripts/check-catalog-docs.mjs` (smoke registered); `npm --prefix apps/web run build`; `npm --prefix apps/web run smoke:e2e` (panel renders all nine + no-leak + a11y + reduced-motion); `npm --prefix apps/web run smoke:ui`. Map each result to its §14 row (index listing, doc amendments, every-game rationale, `poker_lite` showdown, panel renders all nine, no divergent ad-hoc panel, CI fails-closed on missing coverage, browser smoke + no-leak, hidden-info tests, reduced-motion/screen-reader, no TS winner computation, no engine-core/game-stdlib generic noun, golden traces intentionally updated).

### 2. Flip `specs/README.md` index row to `Done`

Update the `victory-explanation-shared-surface` row from `Planned` to `Done` (the spec body itself is not edited by this ticket).

## Files to Touch

- `specs/README.md` (modify — status reconciliation)

## Out of Scope

- Any new production logic, component, script, or doc content (all owned by 001–011).
- Modifying any upstream ticket's files — the capstone exercises them, it does not change them.
- Editing the spec body (route any spec defect to `/reassess-spec`).
- Any Rust/engine/WASM change.

## Acceptance Criteria

### Tests That Must Pass

1. Every exit command passes: `cargo test --workspace`; `cargo run -p replay-check -- --game <g> --all` per game; `node scripts/check-outcome-explanations.mjs`; `node scripts/check-catalog-docs.mjs`; `npm --prefix apps/web run build`; `npm --prefix apps/web run smoke:e2e`; `npm --prefix apps/web run smoke:ui`.
2. `node scripts/check-doc-links.mjs` passes after the index edit.
3. Each spec §14 acceptance criterion has a recorded pass result in the closeout summary.

### Invariants

1. The `specs/README.md` row reads `Done` only because every §14 criterion passed with re-run evidence (no silent completion claim — FOUNDATIONS §11).
2. No existing test, smoke, or gate was weakened to reach green (`docs/AGENT-DISCIPLINE.md`).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; the exit command set and existing pipeline/smoke coverage are the verification, and the only file change is the index status value.`

### Commands

1. `cargo test --workspace && node scripts/check-outcome-explanations.mjs && node scripts/check-catalog-docs.mjs`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && npm --prefix apps/web run smoke:ui`
3. `node scripts/check-doc-links.mjs` (regression after the index edit)

## Outcome

Completed 2026-06-09.

The `victory-explanation-shared-surface` row in `specs/README.md` is `Done` after the full exit command set passed. During the closeout run, three stale support-test expectations were repaired so the existing replay/WASM fixture guards match the rationale-hash migrations landed by earlier tickets:

- `tools/replay-check/src/main.rs` corrupted-hash test now uses the current `race_to_n` final state hash.
- `tools/trace-viewer/src/main.rs` expected summary strings now use the current `race_to_n` state and public-view hashes.
- `crates/wasm-api/src/lib.rs` WASM export fixture notes now match the updated Three Marks, Draughts Lite, and Token Bazaar golden fixtures.

Acceptance evidence:

- `cargo test -p wasm-api --lib` passed.
- `cargo test --workspace` passed.
- `cargo run -p replay-check -- --game <game> --all` passed for `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`, `secret_draft`, and `poker_lite`.
- `node scripts/check-outcome-explanations.mjs` passed.
- `node scripts/check-catalog-docs.mjs` passed.
- `cargo fmt --all --check` passed.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `npm --prefix apps/web run smoke:ui` passed.
