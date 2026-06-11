# GAT11MASCLABLU-013: Bot-strategy evidence docs

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (new `games/masked_claims/docs/COMPETENT-PLAYER.md`, `games/masked_claims/docs/BOT-STRATEGY-EVIDENCE-PACK.md`; no Rust surface)
**Deps**: GAT11MASCLABLU-009, GAT11MASCLABLU-010

## Problem

The official-game contract requires competent-player guidance and a bot-strategy evidence pack documenting the Level 1 claim and response policies and the balance evidence. These trail the bot implementation because the evidence is produced by the bot tests and mirrored simulations.

## Assumption Reassessment (2026-06-10)

1. `src/bots.rs` (GAT11MASCLABLU-009) and `tests/bots.rs` (GAT11MASCLABLU-010) produce the documented behavior and fixtures. Templates `templates/COMPETENT-PLAYER.md` and `templates/BOT-STRATEGY-EVIDENCE-PACK.md` are confirmed present; `games/plain_tricks/docs/{COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md` are the shape models.
2. Spec §"Bot policy" (Level 1 claim + response policies) supplies the strategy content; spec §Acceptance evidence "Balance evidence" requires mirrored Level 1 vs Level 1 per-seat win rates, with a material asymmetry (outside ~40–60%) triggering a scoring-constant retune (Assumption A4) recorded in `BENCHMARKS.md` / `COMPETENT-PLAYER.md`.
3. Cross-artifact boundary under audit: these docs document the Level 1 bot's decisions and fixtures (owned by GAT11MASCLABLU-009/010); per the official-game pattern, a Level 1 evidence pack trails and depends on the bot rather than preceding it.
4. FOUNDATIONS §8 (public bots are competent, explainable, and beatable) is the principle under audit — the evidence pack substantiates it without claiming hidden-state access or a forbidden search class.

## Architecture Check

1. Authoring the evidence after the bot tests exist keeps the documented win-rates and fixtures truthful to measured behavior rather than aspirational.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; these are game-local docs.

## Verification Layers

1. Docs consistent with implemented bot behavior and measured balance -> manual review cross-checked against `tests/bots.rs` and the mirrored simulation numbers.
2. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
3. Balance evidence recorded in both surfaces -> manual cross-check that the win-rate / A4 retune note agrees with `BENCHMARKS.md`.

## What to Change

### 1. `games/masked_claims/docs/COMPETENT-PLAYER.md`

Instantiate from `templates/COMPETENT-PLAYER.md`: competent-play guidance (claim discipline, public counting, calibrated challenging) plus the mirrored Level 1 vs Level 1 per-seat win-rate evidence and any A4 retune note.

### 2. `games/masked_claims/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Instantiate from `templates/BOT-STRATEGY-EVIDENCE-PACK.md`: the Level 1 claim policy (honest default, parameterized bluff rate, counting guard) and response policy (certain-lie detection, threshold challenge) with their evidence and fixtures.

## Files to Touch

- `games/masked_claims/docs/COMPETENT-PLAYER.md` (new)
- `games/masked_claims/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot implementation (GAT11MASCLABLU-009).
- `BENCHMARKS.md` (GAT11MASCLABLU-012); this ticket only cross-references its balance numbers.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs.
2. The documented Level 1 policies match `src/bots.rs` behavior and `tests/bots.rs` assertions.
3. The mirrored win-rate evidence and any A4 retune note are consistent with `BENCHMARKS.md`.

### Invariants

1. Bot docs describe a competent, explainable, beatable opponent with no hidden-state access (FOUNDATIONS §8).
2. Documented behavior matches measured behavior (no aspirational claims).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the bot behavior it documents is covered by GAT11MASCLABLU-009/010.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo test -p masked_claims --test bots` — confirms the behavior the docs describe still holds.
3. Doc-link + the bot suite are the correct boundary; balance numbers are produced by the mirrored simulation named in GAT11MASCLABLU-009/010.

## Outcome

Completed: 2026-06-11

What changed:

- Added `games/masked_claims/docs/COMPETENT-PLAYER.md` with competent-play guidance, hidden-information boundaries, examples, mistakes, and balance-calibration posture.
- Added `games/masked_claims/docs/BOT-STRATEGY-EVIDENCE-PACK.md` documenting the implemented Level 1 policy, allowed input view, candidate extraction, response/claim policy, explanation contract, and evidence surfaces.

Deviations from original plan:

- The docs do not claim calibrated mirrored win-rate evidence yet. They record the current executable evidence and name the 40-60 balance/Assumption A4 retune trigger as follow-up, consistent with `BENCHMARKS.md` smoke-floor posture.

Verification:

- `node scripts/check-doc-links.mjs` passed.
- `cargo test -p masked_claims --test bots` passed.
