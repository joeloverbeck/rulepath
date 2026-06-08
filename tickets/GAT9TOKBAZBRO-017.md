# GAT9TOKBAZBRO-017: Trailing game docs (MECHANICS/UI/AI/ADMISSION/PUBLIC-RELEASE/COMPETENT-PLAYER/BOT-STRATEGY)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/token_bazaar/docs/{MECHANICS,UI,AI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md`)
**Deps**: GAT9TOKBAZBRO-008, GAT9TOKBAZBRO-015

## Problem

The official-game contract requires the full per-game doc set. RULES/SOURCES
(GAT9TOKBAZBRO-001), RULE-COVERAGE (GAT9TOKBAZBRO-012), and BENCHMARKS
(GAT9TOKBAZBRO-011) are authored with their validators; this ticket completes the
remaining seven docs that describe the landed implementation — mechanics
inventory, UI metadata, bot AI + competent-player guidance + bot-strategy
evidence, the implementation-admission record, and the public-release checklist.

## Assumption Reassessment (2026-06-08)

1. The implementation these docs describe exists: rules/effects/visibility/ui/bots
   (GAT9TOKBAZBRO-003…008) and the web board (-015). The sibling
   `games/high_card_duel/docs/{MECHANICS,UI,AI,GAME-IMPLEMENTATION-ADMISSION,
   PUBLIC-RELEASE-CHECKLIST,BOT-STRATEGY-EVIDENCE-PACK,COMPETENT-PLAYER}.md`
   establish the house pattern (verified present), authored from the matching
   `templates/GAME-*.md` / `templates/BOT-STRATEGY-EVIDENCE-PACK.md` /
   `templates/COMPETENT-PLAYER.md`.
2. The doc contents are fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Required game docs" ("Docs must include…": rules summary, resource conservation
   + supply-return notes, market refill semantics, terminal + tie-break rules, bot
   level/rationale/evidence, no-leak notes even though public, benchmark operations,
   source/IP note, mechanic atlas update note). The MECHANICS doc records the
   resource/accounting first-use evidence (the atlas row itself lands in the
   capstone, -018).
3. Cross-artifact boundary under audit: these docs reference the bot (`AI.md`,
   `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md` cite the Level 1 policy +
   the two evidence fixtures from -008) and the UI (`UI.md` cites the board +
   `ui.rs` metadata from -006/-015). `node scripts/check-doc-links.mjs` validates
   the doc graph; the bot-strategy evidence pack's fixtures must match the -008
   tests (no invented evidence).

## Architecture Check

1. Trailing these descriptive docs (after the implementation + bot + UI land)
   keeps them accurate to shipped behavior rather than aspirational; the
   tool-validated docs (RULES/RULE-COVERAGE/BENCHMARKS) are deliberately NOT here —
   they co-land with their validators.
2. No backwards-compatibility aliasing/shims — new docs.
3. Docs-only: no `engine-core` noun introduced; the resource/economy nouns are
   documented as game-local. No `game-stdlib` change. The MECHANICS doc states
   resource/accounting stays local first-use (atlas candidate, no promotion debt).

## Verification Layers

1. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
2. No-leak note present + bot evidence matches tests -> manual review against the
   -008 bot fixtures (rationale is public-safe; no candidate/debug tables described).
3. Boundary cleanliness -> `bash scripts/boundary-check.sh` (no economy noun leaks
   via doc paths).

## What to Change

### 1. `MECHANICS.md`

Resource/accounting mechanic inventory + conservation/supply-return notes; record
resource accounting as first official public-economy use, kept local.

### 2. `UI.md`

Board layout, the Rust `ui.rs` metadata, color-independence + keyboard-reachability
affordances, and the no-leak note (public game, but no debug/candidate exposure).

### 3. `AI.md` + `COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md`

Level 0 + Level 1 description, the Level 1 policy + public rationale, competent-play
guidance, and the evidence fixtures (contract fulfillment + collect-toward-target)
from GAT9TOKBAZBRO-008.

### 4. `GAME-IMPLEMENTATION-ADMISSION.md` + `PUBLIC-RELEASE-CHECKLIST.md`

The admission record (contract coverage) and the public-release checklist
(IP/no-leak/a11y items).

## Files to Touch

- `games/token_bazaar/docs/MECHANICS.md` (new)
- `games/token_bazaar/docs/UI.md` (new)
- `games/token_bazaar/docs/AI.md` (new)
- `games/token_bazaar/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/token_bazaar/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `games/token_bazaar/docs/COMPETENT-PLAYER.md` (new)
- `games/token_bazaar/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- RULES.md / SOURCES.md (GAT9TOKBAZBRO-001), RULE-COVERAGE.md (GAT9TOKBAZBRO-012),
  BENCHMARKS.md (GAT9TOKBAZBRO-011).
- The `docs/MECHANIC-ATLAS.md` first-use row + status flips (GAT9TOKBAZBRO-018).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — no broken links from the new docs.
2. `bash scripts/boundary-check.sh` — no economy noun leaks via doc paths.
3. Manual review: every "Docs must include…" item is present; bot evidence matches
   the -008 fixtures.

### Invariants

1. Docs describe shipped behavior (no aspirational claims); bot rationale described
   as public-safe with no candidate/debug tables.
2. Resource/economy nouns appear only in game-local docs; MECHANICS states
   first-use-kept-local.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. Narrower command boundary: doc accuracy vs the implementation is a manual review;
   the doc graph + boundary checks are the runnable surfaces.
