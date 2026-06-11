# GAT12FLOWATCOO-013: Bot-strategy evidence docs and cooperative balance

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/docs/COMPETENT-PLAYER.md`, `games/flood_watch/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (no code surface)
**Deps**: GAT12FLOWATCOO-011, GAT12FLOWATCOO-012

## Problem

The Level 1 cooperative bots need their evidence docs, and the gate's cooperative balance must be on record: a Level 1 + Level 1 win rate inside the target band (roughly 35–75% on the standard scenario, Assumption A5), measured by simulation, with a result outside the band triggering a scenario-constant retune before public polish. The evidence is produced by the bot tests and simulation that already landed, so these docs trail them.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-010 implemented `FloodWatchLevel1Bot` and GAT12FLOWATCOO-011 the bot tests + `bot-coop-full-game` trace; GAT12FLOWATCOO-012 the benches. The win-rate evidence comes from `cargo run -p simulate -- --game flood_watch` cooperative playouts. `games/masked_claims/docs/COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md` are the verified exemplars; `templates/COMPETENT-PLAYER.md` + `templates/BOT-STRATEGY-EVIDENCE-PACK.md` exist.
2. The spec (§Acceptance evidence "Balance evidence", Work-breakdown item 11, Assumptions A5/A9) fixes: Level 1 + Level 1 cooperative simulation across both scenarios reports win rates; a result outside the A5 band triggers a scenario-constant retune recorded in `BENCHMARKS.md`/`COMPETENT-PLAYER.md` before public polish. Level 1 (not Level 2) bots satisfy the gate (A9), so this is a Level-1 evidence pack documenting the bot's fixtures — placed trailing per the official-game-gate pattern.
3. Cross-artifact boundary under audit: these docs cite the bot policy (`src/bots.rs`), the bot tests/fixtures (`tests/bots.rs`, `bot-coop-full-game.trace.json`), and the simulation command. They must stay consistent with the implemented policy and the measured win rates; a retune (if triggered) edits scenario constants in `data/variants.toml` — which would be a follow-up change owned by GAT12FLOWATCOO-003's data, noted here, not silently re-scoped.
4. FOUNDATIONS §8 (public bots are explainable, fair, beatable) and §6 (evidence-heavy) motivate this ticket: the evidence pack documents how the bot decides and why it is beatable, and the balance band makes "tense but winnable" measurable.
5. Enforcement surface: not a no-leak/determinism surface, but the evidence pack must not reproduce any undrawn-deck reasoning — the bot's documented inputs are the public view + composition counts + seed only (consistent with GAT12FLOWATCOO-010).

## Architecture Check

1. Trailing placement (after the bot tests produce the evidence) is correct for a Level-1 evidence pack: the docs report measured fixtures rather than asserting an unbuilt policy, and the balance band is recorded from real simulation output.
2. No backwards-compatibility aliasing/shims; net-new docs.
3. `engine-core`/`game-stdlib` untouched; docs are game-local.

## Verification Layers

1. Bot policy documented and consistent -> manual review against `src/bots.rs` + grep-proof the doc names the priority order and tie-break.
2. Balance band measured -> simulation/CLI run: `simulate --game flood_watch` Level1+Level1 win rate recorded for both scenarios; in-band or retune noted.
3. Doc link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/flood_watch/docs/COMPETENT-PLAYER.md`

Instantiate from `templates/COMPETENT-PLAYER.md`; describe competent cooperative play (role-efficient bailing/reinforcing, forecast use, expected-threat counting) and record the Level 1 + Level 1 win-rate band per scenario with the A5 retune note.

### 2. `games/flood_watch/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Instantiate from `templates/BOT-STRATEGY-EVIDENCE-PACK.md`; document the Level 1 priority policy, its fixtures/tests, determinism + hidden-order invariance evidence, and the beatability/fairness posture.

## Files to Touch

- `games/flood_watch/docs/COMPETENT-PLAYER.md` (new)
- `games/flood_watch/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot code (GAT12FLOWATCOO-010) and bot tests/traces (GAT12FLOWATCOO-011).
- Scenario-constant retune itself, if triggered — note the trigger; the data edit belongs to the data owner and is a follow-up, not silently re-scoped here.
- The remaining trailing docs (GAT12FLOWATCOO-016) and closeout docs (GAT12FLOWATCOO-019).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with both docs present.
2. The Level 1 + Level 1 win rate for both scenarios is recorded from a real `simulate` run; if outside the A5 band, the retune is noted per A5.
3. The documented priority order matches `src/bots.rs` (manual review + grep).

### Invariants

1. The evidence pack documents only public-view + composition + seed inputs; no undrawn-deck reasoning.
2. The bot is documented as beatable, fair, and deterministic under declared inputs (§8).

## Test Plan

### New/Modified Tests

1. `None — documentation ticket; verification is the simulation run feeding the recorded win-rate band plus doc-link integrity. Bot determinism/legality tests live in GAT12FLOWATCOO-011.`

### Commands

1. `cargo run -p simulate -- --game flood_watch --games 1000` (win-rate evidence; requires GAT12FLOWATCOO-015 registration)
2. `node scripts/check-doc-links.mjs`
3. The simulation run is the correct evidence boundary for balance; full rule-coverage cross-checking of these docs is exercised by GAT12FLOWATCOO-015's `rule-coverage` registration.
