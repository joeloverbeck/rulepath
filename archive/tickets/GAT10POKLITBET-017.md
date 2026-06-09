# GAT10POKLITBET-017: Trailing game docs (mechanics, UI, public-release checklist)

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — new per-game docs only (`games/poker_lite/docs/*`); no Rust/engine surface touched
**Deps**: GAT10POKLITBET-015

## Problem

The official-game doc set needs its remaining trailing members: `MECHANICS.md` (mechanic inventory), `UI.md` (UI metadata/interaction notes), and `PUBLIC-RELEASE-CHECKLIST.md` (release-readiness + IP audit). These describe surfaces that must already exist (rules, renderer), so they trail the implementation.

## Assumption Reassessment (2026-06-08)

1. The doc set matches `games/secret_draft/docs/` (`MECHANICS.md`, `UI.md`, `PUBLIC-RELEASE-CHECKLIST.md`), instantiated from `templates/GAME-MECHANICS.md`, `templates/GAME-UI.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md` (verified present this session). The surfaces they document were built in GAT10POKLITBET-005 (rules/mechanics) and GAT10POKLITBET-015 (renderer/UI).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 per-game docs, §10 documentation updates) names these three among the official doc set; `MECHANICS.md` carries the second-use/first-use mechanic stances (the per-game `PRIMITIVE-PRESSURE-LEDGER.md` is separate, in GAT10POKLITBET-018).
3. Cross-artifact boundary under audit: `MECHANICS.md` reflects the implemented mechanics (rules-core, GAT10POKLITBET-005); `UI.md` reflects the renderer + `ui.rs` metadata (GAT10POKLITBET-007/015); `PUBLIC-RELEASE-CHECKLIST.md` audits the neutral-IP posture (GAT10POKLITBET-001 SOURCES + §10 IP conservatism). Docs cite real surfaces; they author no behavior.
4. FOUNDATIONS §10 (IP conservatism — original, neutral, no trade dress) motivates the public-release checklist. Restated: the checklist confirms the public surface uses original prose/assets and neutral naming, with no casino trade dress.

## Architecture Check

1. Trailing these docs after the renderer lets `UI.md` describe the actual shipped UI and `PUBLIC-RELEASE-CHECKLIST.md` audit the real public surface — not aspirational ones. Matches the official-game doc ordering.
2. No backwards-compatibility aliasing/shims — new docs.
3. `engine-core` untouched (§3); no `game-stdlib` promotion (§4); prose only.

## Verification Layers

1. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
2. Mechanic-inventory fidelity (MECHANICS reflects implemented rules) -> manual review against `games/poker_lite/src/rules.rs` + `RULES.md`.
3. IP audit (no casino trade dress; original/neutral) -> manual IP-conservatism review against §10 + SOURCES.md.
4. Single-artifact-class ticket (docs only); the layers above are the applicable surfaces.

## What to Change

### 1. `games/poker_lite/docs/MECHANICS.md`

Instantiate from `templates/GAME-MECHANICS.md`. Inventory the Crest Ledger mechanics; note the second-use (card/private-hand after `high_card_duel`; accounting after `token_bazaar`) and first-use (bounded pledge / shared-pool) stances, cross-referencing the atlas (the binding ledger entry lands in GAT10POKLITBET-018).

### 2. `games/poker_lite/docs/UI.md`

Instantiate from `templates/GAME-UI.md`. Document the renderer's viewer modes, neutral presentation, reduced-motion behavior, and accessibility metadata as shipped in GAT10POKLITBET-015.

### 3. `games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`

Instantiate from `templates/PUBLIC-RELEASE-CHECKLIST.md`. Audit release readiness: original IP/neutral naming, no casino trade dress, no-leak posture, evidence completeness.

## Files to Touch

- `games/poker_lite/docs/MECHANICS.md` (new)
- `games/poker_lite/docs/UI.md` (new)
- `games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- `PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md` §10B notes, `progress.md`, `specs/README.md` index, spec Status flip (GAT10POKLITBET-018).
- `RULES.md`/`SOURCES.md`/`ADMISSION` (001), `RULE-COVERAGE.md` (012), `BENCHMARKS.md` (013), bot docs (011).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the three new docs.
2. `MECHANICS.md` mechanic inventory matches the implemented rules — manual checklist against `rules.rs` + `RULES.md`.
3. `PUBLIC-RELEASE-CHECKLIST.md` confirms original/neutral IP and no casino trade dress (§10).

### Invariants

1. Docs cite only real, shipped surfaces — no aspirational mechanics/UI.
2. No casino trade dress, copied prose, or proprietary naming asserted as present (§10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is `check-doc-links` plus manual mechanic-inventory and IP-conservatism review.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `ls games/poker_lite/docs/MECHANICS.md games/poker_lite/docs/UI.md games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
3. Narrower boundary: docs-only; no Rust surface, so `cargo` checks are not the verification boundary here.

## Outcome

Completed on 2026-06-09.

- Added `MECHANICS.md` with Crest Ledger's mechanic inventory, including
  second-use card/private-hand pressure after `high_card_duel`, second-use
  public accounting pressure after `token_bazaar`, and first-use bounded
  pledge/shared-pool pressure.
- Added `UI.md` documenting the shipped `PokerLiteBoard` presentation contract,
  Rust/WASM authority boundary, legal action mapping, viewer modes,
  accessibility, reduced-motion behavior, responsive layout, and no-leak
  surfaces.
- Added `PUBLIC-RELEASE-CHECKLIST.md` documenting official-game evidence,
  original/neutral IP posture, no casino trade dress, no-leak surfaces,
  legal-only UI, and the explicit GAT10POKLITBET-018 closeout constraint.
- Manual review matched the mechanics and UI prose against `RULES.md`,
  `RULE-COVERAGE.md`, `SOURCES.md`, `ui.rs`, `rules.rs`, `visibility.rs`, and
  `PokerLiteBoard.tsx`.

Verification:

- `node scripts/check-doc-links.mjs`
- `ls games/poker_lite/docs/MECHANICS.md games/poker_lite/docs/UI.md games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `rg -n "\\[[^\\]]+\\]\\(([^)]+)\\)" games/poker_lite/docs/MECHANICS.md games/poker_lite/docs/UI.md games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
