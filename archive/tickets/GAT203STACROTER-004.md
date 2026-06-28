# GAT203STACROTER-004: Starbridge outcome-explanation docs reconcile + `Done`-flip capstone

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — game-local docs (`UI.md`, `GAME-EVIDENCE.md`, `RULE-COVERAGE.md`), the `specs/README.md` index row, and the spec Status (docs/status-only)
**Deps**: GAT203STACROTER-001, GAT203STACROTER-002, GAT203STACROTER-003

## Problem

Once the rationale is projected (001), serialized (002), and rendered (003), the Gate 20.3 documentation and tracking surfaces must be reconciled and the gate closed: `UI.md` should note the terminal explanation is now Rust-projected and rendered (and carry the fully-qualified turn-limit template-key literal so `check-outcome-explanations.mjs` enforces it), `GAME-EVIDENCE.md` / `RULE-COVERAGE.md` should record the fix and link the new tests, and the `specs/README.md` row + the spec Status flip to `Done`. This capstone also runs the spec's full exit/acceptance command set end to end.

## Assumption Reassessment (2026-06-28)

1. The docs exist and are the reconcile targets: `games/starbridge_crossing/docs/UI.md` (the "Outcome / victory explanation" section names both decisive causes at `:69`–`:83`, but carries the fully-qualified key literal only for `finish_order_complete` at `:73`), `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `games/starbridge_crossing/docs/RULE-COVERAGE.md`. Confirmed.
2. `specs/README.md` row 11.3 (`gate-20-3-...`) reads Status `Planned`; the spec's §10 Documentation-updates and §5 WB-004 drive the index flip + Status `Done` at closeout. Confirmed.
3. Cross-artifact boundary under audit: `scripts/check-outcome-explanations.mjs` extracts `starbridge_crossing.<key>` literals from the `UI.md` outcome body (`extractTemplateKeys`) and requires each in `outcomeExplanationTemplates.ts`. Adding the `starbridge_crossing.turn_limit_progress_vector` literal here makes the gate enforce the key that GAT203STACROTER-003 added — so this ticket MUST land after 003 (its `Deps`), or the gate would demand a key not yet present.

## Architecture Check

1. A single trailing docs+closeout ticket lets the `UI.md` key literal, the evidence rows, and the `Done`-flip land atomically once all three implementation surfaces exist coherently — avoiding a staleness window where docs cite an unrendered surface.
2. No shim, no behavior change — docs and status only.
3. `engine-core` / `game-stdlib` untouched; no mechanic noun introduced.

## Verification Layers

1. Docs reconciled → grep-proof: `UI.md` contains the literal `starbridge_crossing.turn_limit_progress_vector`; `GAME-EVIDENCE.md` / `RULE-COVERAGE.md` cite the new tests.
2. Gate enforces both keys → `node scripts/check-outcome-explanations.mjs` passes with the qualified turn-limit key now required and present.
3. Exit criteria met → the spec's full acceptance command set (CI gate 0 + gate 1) runs green; `specs/README.md` row 11.3 and the spec Status read `Done`.

## What to Change

### 1. Reconcile the game docs

`UI.md`: note the terminal outcome explanation is now projected by Rust (`terminal_rationale`) and rendered via `OutcomeExplanationPanel`, and add the fully-qualified `starbridge_crossing.turn_limit_progress_vector` key literal to the outcome section. `GAME-EVIDENCE.md`: fix-receipt / outcome-explanation row. `RULE-COVERAGE.md`: link the new `SC-FINISH-004` / `SC-FINISH-006` outcome tests if it tracks that surface.

### 2. Flip the tracker + spec status

Update `specs/README.md` row 11.3 to `Done` (with a one-line completion note) and set the spec's Status field to `Done`.

## Files to Touch

- `games/starbridge_crossing/docs/UI.md` (modify)
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` (modify)
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-20-3-starbridge-crossing-terminal-outcome-explanation.md` (modify)

## Out of Scope

- Any production code, test, or deterministic artifact — those are GAT203STACROTER-001/002/003.
- The web-shell catalog docs (no renderer-list/smoke-list membership change — Starbridge is already listed; only the terminal-surface rendering changed).

## Acceptance Criteria

### Tests That Must Pass

1. CI gate 0: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace`, `cargo test --workspace`.
2. CI gate 1 (game): `cargo run -p replay-check -- --game starbridge_crossing --all`, `cargo run -p fixture-check -- --game starbridge_crossing`, `cargo run -p rule-coverage -- --game starbridge_crossing`, `bash scripts/boundary-check.sh`, `node scripts/check-outcome-explanations.mjs`, `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`.
3. `node scripts/check-doc-links.mjs`.

### Invariants

1. `specs/README.md` row 11.3 and the spec Status both read `Done` only after gate 0 + gate 1 pass.
2. `check-outcome-explanations.mjs` passes with the qualified turn-limit key required and present.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `node scripts/check-outcome-explanations.mjs && node scripts/check-doc-links.mjs`
2. `cargo test --workspace && cargo run -p replay-check -- --game starbridge_crossing --all && npm --prefix apps/web run smoke:e2e`
3. The doc gates + full acceptance set are the correct boundary — this ticket introduces no production logic, only docs/status reconciliation gated on the prior tickets passing.

## Outcome

Completed 2026-06-28.

Reconciled the Starbridge outcome-explanation documentation and status surfaces:
`UI.md` now states that Rust projects terminal `terminal_rationale` and the
browser renders it through `OutcomeExplanationPanel`; the outcome section names
both fully-qualified template keys, including
`starbridge_crossing.turn_limit_progress_vector`. `GAME-EVIDENCE.md` records the
Gate 20.3 receipt, and `RULE-COVERAGE.md` now links `SC-FINISH-004`,
`SC-FINISH-006`, and `SC-UI-001..003` to the Rust/WASM/browser evidence. The spec
and `specs/README.md` were flipped to `Done`.

Verification recorded for the capstone:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo run -p simulate -- --game starbridge_crossing --games 1000`
- `cargo run -p replay-check -- --game starbridge_crossing --all`
- `cargo run -p fixture-check -- --game starbridge_crossing`
- `cargo run -p rule-coverage -- --game starbridge_crossing`
- `bash scripts/boundary-check.sh`
- `node scripts/check-outcome-explanations.mjs`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/starbridge-crossing.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `node scripts/check-doc-links.mjs`
