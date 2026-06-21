# GAT17VOWTIDOHHEL-014: Bot-strategy documentation (AI, competent-player, evidence pack)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — game-local bot docs only (`games/vow_tide/docs/{AI,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md`)
**Deps**: 012, 013

## Problem

Document the bot contract and strategy landscape: `AI.md` (L0/L1 authorized fields, memory, tie-breaks, explanation schema, exclusions, simulations), `COMPETENT-PLAYER.md` (sourced strategy, novice traps, contract-relative phases, lawful-inference boundary, future-L2 criteria), and `BOT-STRATEGY-EVIDENCE-PACK.md` (status `L2 not admitted`, deferred fields). These trail the bot ticket because the L1 evidence documents the bot's fixtures.

## Assumption Reassessment (2026-06-21)

1. `games/vow_tide/src/bots.rs` (012) + `tests/bots.rs` + the simulator (013) produce the fixtures/explanations these docs describe; `templates/GAME-AI.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md` are the structural source. Sibling `games/briar_circuit/docs/{AI,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md` is the precedent.
2. Spec Appendix C fixes content: `AI.md` carries the L1 weights (authored parameters, not data/rules), `COMPETENT-PLAYER.md` the bid-calibration/contract-relative analysis, `BOT-STRATEGY-EVIDENCE-PACK.md` the deferred-L2 / `L3 not applicable` posture.
3. Cross-artifact boundary under audit: the L1 weights documented in `AI.md` must match the policy implemented in `bots.rs` (no second source of truth in data) — the docs↔code parity is the contract under audit.
4. FOUNDATIONS §8 is the principle under audit: bots are explainable product opponents, not research AI; the evidence pack records why no L2/search is claimed.

## Architecture Check

1. Trailing the bot ticket lets the docs cite real fixtures and the as-implemented weights, avoiding a docs/code drift window.
2. No shims; new docs.
3. `engine-core`/`game-stdlib` untouched; the L1 weights stay authored parameters in `AI.md`, never TOML behavior.

## Verification Layers

1. Docs complete + link-clean → `node scripts/check-doc-links.mjs`.
2. `AI.md` weights match `bots.rs` policy → manual cross-check against the implemented L1.
3. Evidence pack status `L2 not admitted` / `L3 not applicable` → grep the doc.

## What to Change

### 1. `AI.md`

L0/L1 contract: exact authorized fields (§C.2.1), deterministic tie-breaks, explanation schema, exclusions, the L1 weight set, and the simulation summary shape.

### 2. `COMPETENT-PLAYER.md`

Sourced strategy landscape, bid calibration, dealer-hook effect, secure-vs-shed distinction, zero-bid risk, novice traps, lawful-inference boundary, future-L2 competence criteria.

### 3. `BOT-STRATEGY-EVIDENCE-PACK.md`

Status `L2 not admitted / intentionally deferred`; `L3 not applicable` (imperfect information); the evidence required for any later L2.

## Files to Touch

- `games/vow_tide/docs/AI.md` (new)
- `games/vow_tide/docs/COMPETENT-PLAYER.md` (new)
- `games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot code/tests (012), simulations (013), MECHANICS/UI docs (021).
- Any L2 implementation or strategy claim beyond the deferred posture.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — docs link-clean.
2. Manual review: `AI.md` weights match the implemented L1; evidence pack records deferred L2.

### Invariants

1. The L1 weight set has a single source of truth (`AI.md` prose + `bots.rs`), never static data.
2. No L2 competence is claimed without an accepted evidence pack.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; the bot behavior these docs describe is verified by ticket 012's `tests/bots.rs` and 013's simulations.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo test -p vow_tide --test bots` (confirms the documented behavior still holds)
3. Narrower command rationale: docs are prose; their factual backing is the already-green bot suite + simulations.
