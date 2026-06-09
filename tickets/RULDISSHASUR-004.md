# RULDISSHASUR-004: Author HOW-TO-PLAY for the remaining eight games + assets + CI wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — eight authored Markdown player docs + their generated static assets + one CI workflow step; no Rust/engine, WASM, or behavior surface.
**Deps**: RULDISSHASUR-002, RULDISSHASUR-003

## Problem

To make the rules surface catalog-complete, the eight non-pilot catalog games each need a player-facing `HOW-TO-PLAY.md` following the pilot's house style, plus their generated web assets. Completing all nine lets the fail-closed coverage guard run green and be wired into CI. Source: `specs/rules-display-shared-surface.md` §8.1 (authoring workflow), §8.2 (per-game inventory), §10.2 (build-copy CI).

## Assumption Reassessment (2026-06-09)

1. The eight games each have `games/<id>/docs/RULES.md` with a literal version token on line 9 — note the mixed forms: `race_to_n-rules-v1`, `three_marks-rules-v1`, `column_four-rules-v1`, `directional_flip-rules-v1`, `draughts_lite-rules-v1`, `high-card-duel-rules-v1`, `token-bazaar-rules-v1`, `secret-draft-rules-v1`. All eight have `UI.md`; all except `race_to_n` and `three_marks` have `COMPETENT-PLAYER.md` (authoring reads it only where present, to avoid duplicating strategy). Display names: `secret_draft` → "Veiled Draft", others per catalog.
2. Authoring workflow + per-game focus come from spec §8.1/§8.2; required sections from `templates/GAME-HOW-TO-PLAY.md` (RULDISSHASUR-001); the pilot exemplar is `games/poker_lite/docs/HOW-TO-PLAY.md` (RULDISSHASUR-003); validation is `scripts/check-player-rules.mjs` (RULDISSHASUR-002).
3. Cross-artifact boundary under audit: eight source docs + eight generated `apps/web/public/rules/<id>.md` assets + the `gate-1-game-smoke.yml` CI gate. The `check:rules` step is wired into CI *here* (not in RULDISSHASUR-002) because it only goes green once all nine docs+assets exist — co-landing it with the completing ticket avoids a multi-PR red-CI window.
4. FOUNDATIONS principle restated: §11 (no hidden-information leak) and §10 (IP — original neutral prose). `secret_draft`/"Veiled Draft" and `high_card_duel` are hidden-information games.
5. No-leak enforcement surface: the two hidden-info docs describe visibility from the player's own and public perspective only (never pending secret choices, opponent hand, deck order, seed-derived values); the six perfect-info docs explicitly mark `Hidden information and reveal timing` as `Not applicable`. Negative-tested by RULDISSHASUR-007's hidden-info smoke.

## Architecture Check

1. Authoring against each `RULES.md`/`UI.md` (player labels) with the pilot as style anchor keeps all nine docs uniform and reviewable; co-landing CI wiring with completion is cleaner than an early red window.
2. No backwards-compatibility shims; docs/assets/CI step are additive.
3. `engine-core`/`game-stdlib` untouched; player prose lives beside the formal docs, never in the engine.

## Verification Layers

1. Catalog-complete and valid → `node scripts/check-player-rules.mjs` reports all nine games valid.
2. Eight assets current → `node scripts/copy-player-rules.mjs` then per-id `diff` exits 0.
3. Hidden-info safety (`secret_draft`, `high_card_duel`) → manual no-leak review (negative-tested in RULDISSHASUR-007).
4. Perfect-info games mark hidden-info N/A → grep-proof each of the six contains `Not applicable — this is a perfect-information game.`
5. CI gate runs green → `gate-1-game-smoke.yml` invokes `check-player-rules` and the workflow passes.

## What to Change

### 1. Author eight `games/<id>/docs/HOW-TO-PLAY.md`

Per spec §8.2 inventory: `race_to_n` (exact-count race), `three_marks` (3×3 line), `column_four` (gravity connect-four), `directional_flip` (bracket/flip, forced pass), `draughts_lite` (mandatory capture, multi-jump, promotion), `high_card_duel` (private hand, simultaneous reveal — hidden-info), `token_bazaar` (collect/exchange/fulfill), `secret_draft`/"Veiled Draft" (secret simultaneous picks — hidden-info). Each: all player-visible action labels explained, all scoring/win/draw/terminal outcomes explained, inert source/version metadata citing the literal `RULES.md` token.

### 2. Generate + commit eight `apps/web/public/rules/<id>.md`

Run `npm --prefix apps/web run build:rules` and commit the byte-identical copies.

### 3. Wire CI

Add `node scripts/copy-player-rules.mjs` and `node scripts/check-player-rules.mjs` steps to `.github/workflows/gate-1-game-smoke.yml` (beside the existing `check-catalog-docs.mjs` step).

## Files to Touch

- `games/race_to_n/docs/HOW-TO-PLAY.md` (new)
- `games/three_marks/docs/HOW-TO-PLAY.md` (new)
- `games/column_four/docs/HOW-TO-PLAY.md` (new)
- `games/directional_flip/docs/HOW-TO-PLAY.md` (new)
- `games/draughts_lite/docs/HOW-TO-PLAY.md` (new)
- `games/high_card_duel/docs/HOW-TO-PLAY.md` (new)
- `games/token_bazaar/docs/HOW-TO-PLAY.md` (new)
- `games/secret_draft/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/race_to_n.md` (new — generated)
- `apps/web/public/rules/three_marks.md` (new — generated)
- `apps/web/public/rules/column_four.md` (new — generated)
- `apps/web/public/rules/directional_flip.md` (new — generated)
- `apps/web/public/rules/draughts_lite.md` (new — generated)
- `apps/web/public/rules/high_card_duel.md` (new — generated)
- `apps/web/public/rules/token_bazaar.md` (new — generated)
- `apps/web/public/rules/secret_draft.md` (new — generated)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- The `poker_lite` pilot doc/asset (RULDISSHASUR-003).
- The copy/check scripts themselves (RULDISSHASUR-002).
- Any `apps/web` component/state/render change (RULDISSHASUR-005/-006).
- Strategy advice (`COMPETENT-PLAYER.md`); any Rust/engine change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-player-rules.mjs` reports all nine games valid (exit 0).
2. `node scripts/copy-player-rules.mjs` leaves every `apps/web/public/rules/<id>.md` byte-identical to source (drift check clean).
3. The `gate-1-game-smoke.yml` `check:rules` step passes in CI.

### Invariants

1. No hidden-info doc names an actual private/hidden/seed-derived value; every perfect-info doc explicitly marks hidden information not applicable.
2. Every player-visible action label and every scoring/win/draw/terminal outcome is explained; prose is original and neutral (`docs/IP-POLICY.md`).

## Test Plan

### New/Modified Tests

1. Eight `games/<id>/docs/HOW-TO-PLAY.md` — authored player docs validated by `check-player-rules.mjs`.
2. `.github/workflows/gate-1-game-smoke.yml` — adds the copy+check CI steps (the workflow run is the integration test).

### Commands

1. `node scripts/check-player-rules.mjs` (all nine green) and `node scripts/copy-player-rules.mjs`
2. `node scripts/check-catalog-docs.mjs` (catalog/doc consistency unaffected)
3. `npm --prefix apps/web run build` (build:rules runs before vite build; confirms assets regenerate cleanly)
