# ACTCONMAT-013: Capstone — acceptance evidence + spec closeout

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — verification-only acceptance evidence + status reconciliation (`specs/README.md` index row, the spec's own `Status`).
**Deps**: ACTCONMAT-007, ACTCONMAT-011, ACTCONMAT-012

## Problem

The spec's §9 exit criteria and acceptance evidence must be exercised end-to-end across the implementation tickets, and the spec flipped to `Done` with its `specs/README.md` index row updated. This capstone introduces no new production logic; it runs the gate's verification set and reconciles status once every prior ticket has landed.

## Assumption Reassessment (2026-06-12)

1. Every implementation surface is delivered by ACTCONMAT-001…010 (label resolution, reserved keys, cost rendering, composer, faction context, turn report, detail tier, rules surface, variant selector, runtime guard); the doc amendments by ACTCONMAT-012; the bot-why disposition by ACTCONMAT-011. The `Deps` leaf set (007, 011, 012) transitively covers 001–006, 008, 009, 010 (012 `Deps` each of those; 007 and 011 are the only other leaves).
2. Spec §9 (exit criteria 1–9) + §12 (Documentation updates: `specs/README.md` index row flipped to `Done` at WB10 with evidence). The spec is `specs/action-consequence-and-match-context-shared-surfaces.md`, currently `Status: Planned`, listed in `specs/README.md` as `Planned`.
3. Cross-artifact boundary under audit: the full verification pipeline (Rust gate 0, per-game gates, web smokes, doc/catalog/player-rules guards) and the status-reconciliation surfaces (`specs/README.md` index row + the spec Status). This capstone exercises upstream tickets; it does not modify their files.
4. FOUNDATIONS §11 (universal acceptance invariants): the capstone verifies the change satisfies the invariants (Rust behavior authority, no-leak, deterministic replay/hash, viewer-safe views) by re-running the named evidence commands — not by re-implementing any of them.

## Architecture Check

1. A trailing verification-only capstone gives a single place that proves the gate's exit criteria pass coherently, re-enumerating expected counts from fixtures rather than hardcoding. Assigning the `Done`-flip here (gated on exit evidence) keeps the completion narrative honest.
2. No shim: no production logic; the status flip is the only mutation beyond running checks.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. EF normal-mode cost/consequence/turn-report closure (exit criterion 1) -> UI smoke (`event-frontier.smoke.mjs`) sub-cases.
2. No raw identifiers catalog-wide + negative test (criterion 2) -> `a11y-noleak.smoke.mjs`.
3. Composer byte-identical submission (criterion 3) -> `replay-check` + composer smoke.
4. Faction identity / variant reachability (criterion 4) -> UI smoke.
5. Turn report + flood_watch parity (criterion 5) -> `smoke:effects` / `flood-watch.smoke.mjs`.
6. Details + edict scope (criterion 6) -> `event-frontier.smoke.mjs`.
7. How-to-Play correctness/economy (criterion 7) -> `check-player-rules.mjs`.
8. Rust + web gates green (criterion 8) -> gate-0 + per-game gates + web smokes.
9. Doc amendments + index flip (criterion 9) -> `check-doc-links.mjs` / `check-catalog-docs.mjs` + grep-proof.

## What to Change

### 1. Run the acceptance-evidence set

Execute the §9 exit-criteria commands (gate 0, per-game gates for `event_frontier`/`flood_watch`, web smokes, doc/catalog/player-rules guards) and record pass/fail; re-enumerate expected counts (e.g. 21 EF cards with details) from fixtures at run time.

### 2. Flip status to Done

Set the spec `Status` to `Done` and update its `specs/README.md` index row to `Done` with the evidence reference.

## Files to Touch

- `specs/README.md` (modify; index row → `Done`)
- `specs/action-consequence-and-match-context-shared-surfaces.md` (modify; `Status` → `Done`)

## Out of Scope

- Any production/implementation logic — the upstream tickets own their files; this capstone exercises them, it does not modify them.
- Archival/move of the spec — follow `docs/archival-workflow.md` separately if the repo convention moves completed specs.

## Acceptance Criteria

### Tests That Must Pass

1. The full §9 exit-criteria command set passes (enumerated below in Commands), each mapped to its exit criterion.
2. `specs/README.md` and the spec both read `Status: Done` with evidence cited.
3. `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` pass post-flip.

### Invariants

1. The capstone adds no production logic; the only mutations are status reconciliation (§Ticket-shapes `Done`-flip default).
2. Every §9 exit criterion maps to a re-runnable evidence command (no exit criterion verified by assertion alone).

## Test Plan

### New/Modified Tests

1. None — verification-only capstone; it runs existing pipeline checks and the smokes authored by ACTCONMAT-003/004/005/006/007/009/010.

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace && cargo test --workspace`
2. `cargo run -p simulate -- --game event_frontier --games 1000 && cargo run -p replay-check -- --game event_frontier --all && cargo run -p fixture-check -- --game event_frontier && cargo run -p rule-coverage -- --game event_frontier` (repeat for `flood_watch` where touched)
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects && npm --prefix apps/web run smoke:e2e && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-player-rules.mjs && bash scripts/boundary-check.sh`

## Outcome

Ran the full capstone evidence set and flipped the spec plus `specs/README.md` index row to `Done`. Added the capstone evidence summary to the spec's acceptance evidence section.

Verification:

1. Rust hygiene: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace`, and `cargo test --workspace` passed.
2. `event_frontier`: `simulate --games 1000` passed with `simulation_pass_rate_percent=100.00`; `replay-check --all`, `fixture-check`, and `rule-coverage` passed.
3. `flood_watch`: `simulate --games 1000`, `replay-check --all`, `fixture-check`, and `rule-coverage` passed.
4. Web: `smoke:wasm`, `smoke:ui`, `smoke:effects`, and `smoke:e2e` passed.
5. Docs/boundary: `check-doc-links`, `check-catalog-docs`, `check-player-rules`, `check-presentation-copy`, and `boundary-check` passed.
6. Static detail enumeration: 21 Event Frontier card ids, 21 labels, and 21 detail entries.
