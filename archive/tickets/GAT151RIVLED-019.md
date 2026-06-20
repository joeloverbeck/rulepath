# GAT151RIVLED-019: Per-game docs sync (v2 cutover)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (docs + generated player-rules) — `games/river_ledger/docs/*`, generated `apps/web/public/rules/river_ledger.md` (+ manifest), `docs/MECHANIC-ATLAS.md`
**Deps**: GAT151RIVLED-014, GAT151RIVLED-016, GAT151RIVLED-017, GAT151RIVLED-018

## Problem

Reconcile every per-game doc surface for the v2 all-in/side-pot delta and the rules/data version bump: author the new `RL-*` rule prose and old-rule migration table in `RULES.md`, map all rules in `RULE-COVERAGE.md`, sync `HOW-TO-PLAY.md` (and regenerate the public player-rules asset), and update `MECHANICS`/`UI`/`AI`/`COMPETENT-PLAYER`/`BOT-STRATEGY-EVIDENCE-PACK`/`SOURCES`. This lands the doc side of the v2 cutover, resolving the `check-player-rules` / `rule-coverage` red window opened at GAT151RIVLED-011.

## Assumption Reassessment (2026-06-20)

1. Code/docs: the rule families + supersession map were planned in GAT151RIVLED-001 (ADMISSION); `RULES.md` / `RULE-COVERAGE.md` still carry v1 rule IDs (`RL-POT-SINGLE-001`/`-002`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, `RL-OOS-ALLIN-001`). `apps/web/public/rules/river_ledger.md` is generated from `HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`, guarded by `scripts/check-player-rules.mjs`.
2. Docs: spec §10 enumerates the per-game doc surfaces; `BENCHMARKS.md` is owned by GAT151RIVLED-018 (validator co-location) and the status/index/catalog surfaces by GAT151RIVLED-020 — this ticket owns the rules/coverage/mechanics/UI/AI/player docs.
3. Cross-artifact boundary under audit: `RULES.md` rules-version line ↔ `HOW-TO-PLAY.md` cited formal-rules version ↔ generated `river_ledger.md` ↔ `manifest.json`, all enforced by `check-player-rules`; the generated asset must be produced by the script, never hand-edited.
4. (rules/version reconciliation) Restate: this ticket aligns the doc version surfaces with the v2 bump from GAT151RIVLED-011, closing the red window; it changes no behavior. The supersession entries replace the legacy IDs with explicit migration rows, not silent deletion.
5. (player-rules generation) `apps/web/public/rules/river_ledger.md` is a generated artifact — edit `HOW-TO-PLAY.md` and re-run `scripts/copy-player-rules.mjs`; the parity check `scripts/check-player-rules.mjs` is the exit gate, not a hand edit.

## Architecture Check

1. Reconciling all per-game docs in one v2-cutover ticket (after behavior, traces, and benches land) gives `check-player-rules` and `rule-coverage` a single coherent green point rather than staggered partial states.
2. No backwards-compatibility shims; superseded rule IDs get explicit migration rows.
3. Docs only — no `engine-core`/`games` behavior change; the generated player-rules asset is produced by the canonical script.

## Verification Layers

1. New `RL-*` prose + migration table complete -> `rule-coverage --game river_ledger` clean (every rule mapped).
2. Player-rules parity (HOW-TO-PLAY ↔ generated asset ↔ version) -> `check-player-rules.mjs`.
3. Doc links resolve -> `check-doc-links.mjs`.
4. No hand-edited generated asset -> generated via `copy-player-rules.mjs`, confirmed by the parity check.

## What to Change

### 1. Rules + coverage + sources

Author the `RL-STACK-*`/`RL-ALLIN-*`/`RL-POT-*`/outcome/visibility/bot/replay rule prose and the old-rule migration table in `RULES.md`; map every new/changed rule to modules/tests/traces in `RULE-COVERAGE.md`; cite the §7.8 external sources and River Ledger divergence in `SOURCES.md`.

### 2. Player + mechanic + bot docs + regen

Sync `HOW-TO-PLAY.md` (neutral all-in/side-pot/return/odd-unit prose) and regenerate `apps/web/public/rules/river_ledger.md` + `manifest.json` via `copy-player-rules.mjs`; extend `MECHANICS.md`/`UI.md`/`AI.md`/`COMPETENT-PLAYER.md`/`BOT-STRATEGY-EVIDENCE-PACK.md`; record the Gate 15.1 atlas decision pointer in `docs/MECHANIC-ATLAS.md`.

## Files to Touch

- `games/river_ledger/docs/RULES.md` (modify)
- `games/river_ledger/docs/RULE-COVERAGE.md` (modify)
- `games/river_ledger/docs/MECHANICS.md` (modify)
- `games/river_ledger/docs/UI.md` (modify)
- `games/river_ledger/docs/AI.md` (modify)
- `games/river_ledger/docs/COMPETENT-PLAYER.md` (modify)
- `games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (modify)
- `games/river_ledger/docs/HOW-TO-PLAY.md` (modify)
- `games/river_ledger/docs/SOURCES.md` (modify)
- `apps/web/public/rules/river_ledger.md` (modify; regenerated via `scripts/copy-player-rules.mjs`)
- `apps/web/public/rules/manifest.json` (modify; regenerated)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- `BENCHMARKS.md` (GAT151RIVLED-018), `GAME-IMPLEMENTATION-ADMISSION.md`/`PRIMITIVE-PRESSURE-LEDGER.md` (GAT151RIVLED-001/-002).
- `specs/README.md` index flip, `apps/web/README.md` catalog list, public-release checklist, archival (GAT151RIVLED-020).
- Any behavior/code change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game river_ledger` — every rule maps to exact evidence (fully green).
2. `node scripts/check-player-rules.mjs` — HOW-TO-PLAY ↔ generated asset ↔ version parity (red window closed).
3. `node scripts/check-doc-links.mjs` — all doc links resolve.

### Invariants

1. The generated `apps/web/public/rules/river_ledger.md` is produced by `copy-player-rules.mjs`, never hand-edited.
2. Every superseded legacy rule ID has an explicit migration row; no silent deletion.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based via rule-coverage, check-player-rules, and check-doc-links named above.`

### Commands

1. `cargo run -p rule-coverage -- --game river_ledger`
2. `node scripts/check-player-rules.mjs`
3. `node scripts/check-doc-links.mjs` — the doc-validator trio is the correct boundary; behavior is verified by earlier tickets.

## Outcome

Completed 2026-06-20. Reconciled River Ledger per-game docs for the v2 all-in/side-pot cutover: `RULES.md` now has concrete `RL-STACK-*`, `RL-ALLIN-*`, `RL-POT-*`, visibility, replay, bot, and UI rule IDs plus a legacy-rule migration table; `RULE-COVERAGE.md` maps every new and superseded ID to tests, traces, benchmarks, WASM/web proof, or documented migration evidence. Updated player, mechanics, UI, AI, competent-player, bot evidence, source, and atlas surfaces to describe finite stacks, all-in action metadata, side-pot eligibility, returns, and Rust-owned terminal allocation.

Regenerated `apps/web/public/rules/river_ledger.md` and the manifest via `node scripts/copy-player-rules.mjs`; no hand edit was made to the generated public rules asset.

Verification passed:

1. `cargo run -p rule-coverage -- --game river_ledger`
2. `node scripts/check-player-rules.mjs`
3. `node scripts/check-doc-links.mjs`
