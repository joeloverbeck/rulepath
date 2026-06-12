# ACTCONMAT-008: Rules surface fix + player-rules content

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RulesPanel.tsx`; plus canonical player-rules docs (`games/event_frontier/docs/HOW-TO-PLAY.md`, `games/event_frontier/docs/SOURCES.md`) and the regenerated `apps/web/public/rules/event_frontier.md` asset. No Rust/engine behavior.
**Deps**: None

## Problem

The How-to-Play renderer mangles snake_case (underscores parsed as emphasis: "eventfrontier_ _Formal rules source…"); `seat_0`/`seat_1` vocabulary appears in player-facing prose; a "Source notes for maintainers" section ships to players; and the economy (costs, caps, income) is undocumented. The renderer bug and the content gaps must both close, under an extended authoring contract.

## Assumption Reassessment (2026-06-12)

1. `RulesPanel.tsx` uses a custom `renderRulesMarkdown()` whose regex (`/(`[^`]+`|\*\*[^*]+\*\*|_[^_]+_)/g`) treats intra-word underscores as emphasis. The player rules file `games/event_frontier/docs/HOW-TO-PLAY.md` contains `seat_0`/`seat_1` prose (lines 11-12, 112-113) and a "## Source notes for maintainers" section (line 132); it has no "Costs and economy" section.
2. Spec D8 / §4.2: fix the RulesPanel underscore rendering; the player-rules authoring contract (`docs/OFFICIAL-GAME-CONTRACT.md` §5 "Player-facing rules document") gains: no maintainer sections in the player file (move to `SOURCES.md`), no seat IDs in player prose, a required "Costs and economy" section for resource games. Edit the canonical `HOW-TO-PLAY.md` accordingly. The contract amendment text is *drafted* here and lifted into the doc at ACTCONMAT-012 (spec §10 "applied at WB10, not before").
3. Cross-artifact boundary under audit (information-path): `apps/web/public/rules/event_frontier.md` is a GENERATED copy of `games/event_frontier/docs/HOW-TO-PLAY.md` (byte-identical, 142 lines), produced by `scripts/copy-player-rules.mjs` and validated by `scripts/check-player-rules.mjs`. Edits MUST target the canonical `HOW-TO-PLAY.md` and regenerate the asset — hand-editing the generated file would break the parity guard. The canonical end-state path is `HOW-TO-PLAY.md` → `copy-player-rules.mjs` → `public/rules/event_frontier.md`.
4. FOUNDATIONS §2 (TypeScript presentation-only): `RulesPanel` is a renderer; the fix is presentation only, no legality. FOUNDATIONS §5: the rules file is authored static content.

## Architecture Check

1. Editing the canonical source + regenerating (rather than the generated asset) keeps the `check-player-rules` parity guard green and respects the single-source information path. Code-span/escaped rendering for identifiers fixes the renderer at the tokenizer, not with per-string special-casing.
2. No shim: the maintainer section is relocated to `SOURCES.md`, not duplicated; seat prose is replaced with faction names, not aliased.
3. `engine-core` untouched. No `game-stdlib` change. The authoring-contract extension is a doc obligation, applied at ACTCONMAT-012.

## Verification Layers

1. RulesPanel renders identifiers correctly (no intra-word emphasis) -> UI smoke (`apps/web/e2e/rules-display.smoke.mjs` / `event-frontier.smoke.mjs`).
2. Player file has no maintainer section, no seat IDs, and a Costs-and-economy section -> codebase grep-proof on `HOW-TO-PLAY.md`.
3. Generated asset matches the canonical source -> `node scripts/check-player-rules.mjs` (parity guard).

## What to Change

### 1. RulesPanel identifier rendering

Fix `renderRulesMarkdown()` so intra-word underscores render literally (code-span or escaped), not as emphasis.

### 2. Canonical HOW-TO-PLAY.md edits

In `games/event_frontier/docs/HOW-TO-PLAY.md`: add a "Costs and economy" section (funds/provisions economy, operation costs, edict cost modifiers, income, caps); replace `seat_0`/`seat_1` prose with faction names; relocate the "Source notes for maintainers" block to `games/event_frontier/docs/SOURCES.md`.

### 3. Regenerate the rendered asset

Run `node scripts/copy-player-rules.mjs` to regenerate `apps/web/public/rules/event_frontier.md`; confirm `node scripts/check-player-rules.mjs` passes.

### 4. Catalog rules-file audit

Audit the other catalog games' player-rules files against the (drafted) extended contract; record findings (content authored per-game is out of scope here unless a game has spendable resources and lacks the economy section).

## Files to Touch

- `apps/web/src/components/RulesPanel.tsx` (modify)
- `games/event_frontier/docs/HOW-TO-PLAY.md` (modify; canonical source)
- `games/event_frontier/docs/SOURCES.md` (modify; relocated maintainer notes)
- `apps/web/public/rules/event_frontier.md` (modify; generated via `scripts/copy-player-rules.mjs` — not hand-edited)

## Out of Scope

- Editing `docs/OFFICIAL-GAME-CONTRACT.md` — the §5 additions are drafted here, lifted at ACTCONMAT-012.
- Card/edict detail prose (ACTCONMAT-007).
- Rewriting other games' rules files — audit only (a resource game lacking the economy section is flagged, not authored here).

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/rules-display.smoke.mjs` (or `event-frontier.smoke.mjs`) asserts the How-to-Play surface renders identifiers correctly.
2. Grep-proof: `HOW-TO-PLAY.md` contains a "Costs and economy" section, no `seat_0`/`seat_1`, and no maintainer/source-note section.
3. `node scripts/check-player-rules.mjs` and `node scripts/check-doc-links.mjs` pass.

### Invariants

1. Player-rules edits land in the canonical `HOW-TO-PLAY.md`; the rendered asset is regenerated, never hand-edited (§2 information-path).
2. `RulesPanel` is presentation-only; no legality decided in TS (§2).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/rules-display.smoke.mjs` — identifier-rendering assertion.
2. Docs/content changes (`HOW-TO-PLAY.md`, `SOURCES.md`) are command-verified via `scripts/check-player-rules.mjs` and `scripts/check-doc-links.mjs`; no new Rust/TS test.

### Commands

1. `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`
2. `node scripts/check-doc-links.mjs`
3. `npm --prefix apps/web run smoke:e2e`
