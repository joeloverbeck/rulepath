# RULDISSHASUR-003: poker_lite "Crest Ledger" pilot HOW-TO-PLAY + generated asset

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — authored Markdown player doc + its byte-identical generated static web asset; no Rust/engine, WASM, or behavior surface.
**Deps**: RULDISSHASUR-001, RULDISSHASUR-002

## Problem

`poker_lite` / "Crest Ledger" is the designated pilot for the player-rules surface. It is the highest-stakes authoring target (a hidden-information game whose docs must never leak private crests, the pre-reveal center crest, or the deck tail) and it establishes the house style the other eight docs follow. Source: `specs/rules-display-shared-surface.md` §8.3 (worked pilot text), §8.4 (pilot validation notes).

## Assumption Reassessment (2026-06-09)

1. `games/poker_lite/docs/RULES.md` exists; its version token is `poker-lite-rules-v1` on line 9 (literal). `games/poker_lite/docs/{COMPETENT-PLAYER.md,UI.md,SOURCES.md}` exist for label/strategy/IP cross-reference. The spec §8.3 pilot prose was validated faithful to `RULES.md` during in-session `/reassess-spec` (Hold/Press/Match/Lift/Yield; two rounds with 1-/2-marker pressure; center reveal only after round 1 closes without yield; showdown order pair-flag → private-rank → even split; one lift per round).
2. Pilot text source is spec §8.3; required sections come from `templates/GAME-HOW-TO-PLAY.md` (RULDISSHASUR-001); validation is `scripts/check-player-rules.mjs` (RULDISSHASUR-002).
3. Cross-artifact boundary under audit: the authored `games/poker_lite/docs/HOW-TO-PLAY.md` and its byte-identical generated copy `apps/web/public/rules/poker_lite.md` (produced by `build:rules`).
4. FOUNDATIONS principle restated: §10 (IP conservatism — original Rulepath prose, neutral "Crest Ledger"/crest vocabulary, no copied poker/casino rulebook text or trade dress) and §11 (no hidden-information leak).
5. No-leak enforcement surface: the doc describes visibility only from the player's own and the public perspective. It must NOT name any actual private crest, the hidden center crest before reveal, the deck tail, or any seed-derived value. This is authored substrate; the runtime no-leak guarantee is enforced by RULDISSHASUR-007's hidden-info smoke, which this doc must not give content to leak.

## Architecture Check

1. Transcribing the spec's already-validated pilot (rather than re-deriving from `RULES.md`) minimizes drift risk and gives the remaining eight docs a concrete style anchor.
2. No backwards-compatibility shims; the doc and asset are new.
3. `engine-core`/`game-stdlib` untouched; this is presentation content beside the formal docs, never behavior.

## Verification Layers

1. Player doc passes the contract → `node scripts/check-player-rules.mjs` reports `poker_lite` valid (required sections, hidden-info section non-"Not applicable", version token match).
2. Generated asset is current → `diff games/poker_lite/docs/HOW-TO-PLAY.md apps/web/public/rules/poker_lite.md` exits 0.
3. No hidden-information leak → manual no-leak review: the doc names no actual private/center-before-reveal/deck-tail value (negative-tested by RULDISSHASUR-007).
4. Version literal match → grep-proof the doc cites `poker-lite-rules-v1` matching `RULES.md` line 9.
5. IP-original prose → manual IP review against `docs/IP-POLICY.md` (original neutral vocabulary, no copied rulebook text/assets).

## What to Change

### 1. Author `games/poker_lite/docs/HOW-TO-PLAY.md`

Create the doc from spec §8.3 (Crest Ledger player prose) or a faithful edit preserving every rules/safety point: inert source/version metadata; At a glance; What you can see; Setup; On your turn; Actions (Hold/Press/Match/Lift/Yield); How rounds close; Scoring and winning (yield-win + showdown tiebreak); Hidden information and reveal timing; Common terms; What this page is not. Per spec §8.4, record that the per-seat contribution cap is deliberately omitted (UI offers only legal actions).

### 2. Generate + commit `apps/web/public/rules/poker_lite.md`

Run `npm --prefix apps/web run build:rules` and commit the byte-identical generated copy.

## Files to Touch

- `games/poker_lite/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/poker_lite.md` (new — generated, byte-identical copy)

## Out of Scope

- The other eight games' docs (RULDISSHASUR-004).
- The copy/check scripts themselves (RULDISSHASUR-002).
- Any `apps/web` component/state/render change (RULDISSHASUR-005/-006); the panel that displays this asset is built separately.
- Strategy advice (belongs in `COMPETENT-PLAYER.md`); any Rust/engine change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-player-rules.mjs` reports `poker_lite` valid (other games may still be missing until RULDISSHASUR-004).
2. `diff games/poker_lite/docs/HOW-TO-PLAY.md apps/web/public/rules/poker_lite.md` exits 0.
3. `grep -q 'poker-lite-rules-v1' games/poker_lite/docs/HOW-TO-PLAY.md` succeeds and the doc contains a non-"Not applicable" `Hidden information and reveal timing` section.

### Invariants

1. The doc contains no actual hidden value (opponent private crest, pre-reveal center crest, deck tail, seed-derived data) — only own-perspective and public-perspective visibility prose.
2. Prose is original Rulepath wording with neutral names; no copied external rulebook text or proprietary assets (`docs/IP-POLICY.md`).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/docs/HOW-TO-PLAY.md` — the authored player doc, validated by `check-player-rules.mjs`.
2. `apps/web/public/rules/poker_lite.md` — generated asset, drift-checked against source.

### Commands

1. `node scripts/check-player-rules.mjs` and `diff games/poker_lite/docs/HOW-TO-PLAY.md apps/web/public/rules/poker_lite.md`
2. `npm --prefix apps/web run build:rules`
3. A doc-content + check-script run is the correct boundary; no Rust/test code changes, so `cargo`/full web smoke is exercised later (RULDISSHASUR-007).

## Outcome

Completed: 2026-06-09

What changed:

- Added `games/poker_lite/docs/HOW-TO-PLAY.md` as the Crest Ledger pilot player-rules document.
- Generated the byte-identical static web asset at `apps/web/public/rules/poker_lite.md`.
- Refined the RULDISSHASUR-002 scripts with `RULEPATH_PLAYER_RULES_GAME_IDS` so staged authoring can validate/generate `poker_lite` before the other eight docs exist; default unfiltered checks remain full-catalog.
- Amended `archive/tickets/RULDISSHASUR-002.md` so the archived outcome reflects that script refinement.

Deviations from original plan:

- The pilot uses ASCII hyphens/apostrophes in place of typographic punctuation.
- `build:rules` was run with `RULEPATH_PLAYER_RULES_GAME_IDS=poker_lite` because the default full-catalog script correctly fails until RULDISSHASUR-004 authors the remaining eight docs.

Verification results:

- `env RULEPATH_PLAYER_RULES_GAME_IDS=poker_lite npm --prefix apps/web run build:rules` passed (`copied player rules for 1 catalog games`).
- `env RULEPATH_PLAYER_RULES_GAME_IDS=poker_lite npm --prefix apps/web run check:rules` passed (`player-rules check passed — 1 catalog games validated`).
- `diff games/poker_lite/docs/HOW-TO-PLAY.md apps/web/public/rules/poker_lite.md` exited 0.
- `grep -q 'poker-lite-rules-v1' games/poker_lite/docs/HOW-TO-PLAY.md` exited 0.
- Unfiltered `node scripts/check-player-rules.mjs` failed only on the remaining eight missing game docs/assets; `poker_lite` had no reported errors.
- Manual no-leak/IP review: the doc uses original neutral Crest Ledger prose, explains own/public visibility and reveal timing, and names no actual private crest value, pre-reveal center value, deck-tail value, or seed-derived value.
- `git diff --check` passed.
