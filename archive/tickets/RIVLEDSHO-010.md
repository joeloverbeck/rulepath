# RIVLEDSHO-010: Terminal-only teaching-strength aid (category-ladder position)

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `games/river_ledger/src/showdown.rs` (or `visibility.rs`), `apps/web/src/wasm/client.ts`, `apps/web/src/components/OutcomeExplanationPanel.tsx`
**Deps**: RIVLEDSHO-007

## Problem

Optional learning aid: after showdown, show how strong the winning category is on the ranking ladder — e.g. "Pair is category 8 of 9 from strongest to weakest" — derived only from the revealed final hand, terminal-only, and visibly labeled "teaching aid, not a game value." No equity meter, no pre-showdown meter (spec WB10 / D6). This ticket is **optional** (spec Assumption A5): it may be dropped entirely with no effect on any other ticket.

## Assumption Reassessment (2026-06-15)

1. Verified against current code/tickets: the showdown explanation (RIVLEDSHO-001) supplies the revealed `category`, and the hand-ranking ladder (RIVLEDSHO-007) supplies the category order — the ladder position is derivable from the already-revealed final hand without any hidden input.
2. Verified against specs/docs: spec §6 D6 + §8 WB10 + Assumption A5 (droppable, no dependents); `RULES.md` `RL-UI-SHOWDOWN-001`, `RL-UI-NOLEAK-001`.
3. Cross-artifact boundary under audit: Rust computes the ladder position from the revealed hand → WASM/`client.ts` → the terminal panel; it adds an additive terminal field gated by the same reveal authorization as the rest of the explanation.
4. FOUNDATIONS §11 no-leak invariant motivates this ticket: the aid derives only from authorized revealed showdown hands.
5. §11 no-leak firewall is the enforcement surface: the value must NOT be derived from deck tail, folded unrevealed hands, future board, or opponent private cards before authorized reveal; it is terminal-only and labeled non-canonical. Confirm the computation reads only the revealed final hand and is absent for folded/non-revealed seats.

## Architecture Check

1. A Rust-computed category-ladder position (deterministic, from the revealed hand only) avoids a fake numeric "score" and keeps the strength claim Rust-authored and leak-safe — the only admissible form per the spec.
2. No backwards-compatibility aliasing/shims; additive terminal field + a labeled render.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — game-local computation + `apps/web` render.

## Verification Layers

1. The ladder-position field is computed only from the revealed final hand and is absent for folded/non-revealed seats -> `games/river_ledger/tests/visibility.rs` no-leak/reveal-scope test.
2. The aid renders terminal-only with a visible non-canonical label -> `npm --prefix apps/web run smoke:ui`.
3. No hidden-derived input (deck tail, folded/unrevealed hands, future board, opponent private cards) reaches the value -> manual review + the §11 no-leak visibility test (§11 firewall).

## What to Change

### 1. Rust ladder-position computation

In `showdown.rs`/`visibility.rs`, compute an additive terminal field giving the revealed hand's category-ladder position (e.g. `category_rank_of_total`), derived only from the revealed final hand, projected under the existing reveal authorization.

### 2. Bridge + render

Add the `client.ts` type; render the aid in the terminal panel, visibly labeled "teaching aid, not a game value."

## Files to Touch

- `games/river_ledger/src/showdown.rs` (modify) — or `games/river_ledger/src/visibility.rs`
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify)

## Out of Scope

- Any pre-showdown odds/equity meter (forbidden — spec §3.3).
- A canonical numeric hand score (forbidden — spec §3.3).
- Proceeding at all if WB10 is dropped per Assumption A5 (this ticket is then not-applicable; the hand-ranking reference RIVLEDSHO-007 ships independently).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — ladder-position field derived only from the revealed hand; absent for folded/non-revealed seats.
2. `cargo test -p river_ledger --test visibility` — no-leak: the field reaches no unauthorized viewer.
3. `npm --prefix apps/web run smoke:ui` — the aid renders terminal-only with its non-canonical label.

### Invariants

1. The strength value derives only from authorized revealed showdown hands — never from hidden state (§11 no-leak firewall).
2. It is terminal-only and labeled non-canonical; no equity/score meter is introduced (spec §3.3).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` (modify) — no-leak coverage for the ladder-position field.
2. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert the labeled teaching aid renders terminal-only.

### Commands

1. `cargo test -p river_ledger --test visibility`
2. `npm --prefix apps/web run smoke:ui`
3. The crate no-leak test plus the UI smoke is the correct boundary; the aid carries no behavior to replay-check.

## Outcome

Completed: 2026-06-15

Changes:
- Added additive `CategoryLadderPosition` data to revealed River Ledger showdown strength, computed in Rust from the already-evaluated/revealed hand category.
- Projected the field only through existing revealed `strength`; folded/non-revealed seats still carry `strength: None`.
- Exposed the field through the WASM bridge and TypeScript client type.
- Rendered the terminal-only aid in the River Ledger showdown panel with the visible label `Teaching aid, not a game value`.
- Added visibility, bridge, and browser assertions for no-leak reveal scope and the worked-example `One pair is category 8 of 9 from strongest to weakest.` text.

Verification:
- `cargo fmt --all --check`
- `cargo test -p river_ledger --test visibility`
- `cargo test -p river_ledger`
- `cargo test -p wasm-api`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `git diff --check`

Notes:
- `apps/web/scripts/smoke-ui.mjs` remained unchanged because it does not mount the DOM teaching aid. The DOM assertion lives in `river-ledger.smoke.mjs`, and the required `smoke:ui` command still passed.
