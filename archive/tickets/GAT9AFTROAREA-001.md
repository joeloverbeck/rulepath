# GAT9AFTROAREA-001: Register Token Bazaar in `apps/web/README.md` (intro, Shell Surface, Smoke Layers)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation only (`apps/web/README.md`); no Rust/WASM/engine surface, no TypeScript behavior.
**Deps**: None

## Problem

The post-Gate-9 web-shell README still describes the browser shell as if its game
list ends at `high_card_duel`. Token Bazaar (`token_bazaar`) shipped in Gate 9 —
it is registered in `crates/wasm-api/src/lib.rs` (lines 53–74), has a board
renderer at `apps/web/src/components/TokenBazaarBoard.tsx`, and its E2E smoke
(`apps/web/e2e/token-bazaar.smoke.mjs`) is chained by the `smoke:e2e` package
script. Three spots in `apps/web/README.md` are therefore stale and tell a visitor
the shell ships one fewer game than it does:

1. the intro browser-games list (lines 3–6);
2. the **Shell Surface** board-renderer bullet (lines 55–56);
3. the **Smoke Layers** `smoke:e2e` description (lines 72–75).

This ticket makes those three lists truthful. It is a documentation-truthfulness
edit, not feature work.

## Assumption Reassessment (2026-06-08)

1. Verified against current code: `apps/web/README.md` omits `token_bazaar` in all
   three spots (intro lines 3–6, Shell Surface lines 55–56, Smoke Layers `smoke:e2e`
   bullet lines 72–75). The implementation it should describe exists —
   `apps/web/src/components/TokenBazaarBoard.tsx` (board renderer) and
   `crates/wasm-api/src/lib.rs` lines 53–74 (`GAME_TOKEN_BAZAAR`,
   `GAME_TOKEN_BAZAAR_DISPLAY_NAME = "Token Bazaar"`). So the omissions are stale
   copy, not missing implementation.
2. Verified against specs/docs and `apps/web/package.json`: `smoke:e2e` chains
   `e2e/token-bazaar.smoke.mjs` and does **not** chain `e2e/directional-flip.smoke.mjs`
   (both files exist under `apps/web/e2e/`). The existing README caveat — a standalone
   Directional Flip E2E file exists but is not chained by `smoke:e2e` — is therefore
   truthful for the package script and MUST be preserved unchanged. Spec:
   `specs/gate-9-aftermath-roadmap-realignment.md` (D1–D3).
3. Cross-artifact boundary under audit: `apps/web/README.md` is documentation that
   describes the web shell. Its intro is a **two-sentence paragraph** — the games
   list plus `Rust/WASM owns game behavior; TypeScript presents …` — and that second
   sentence is a §2 behavior-authority statement that MUST survive the edit (per the
   `/reassess-spec` M1 finding resolved 2026-06-08).
4. FOUNDATIONS principle restated before trusting the spec narrative: §7 (public UI
   is central product work) requires the README to describe the shipped browser
   surface truthfully; §2 (behavior authority) requires the prose to keep Rust/WASM
   as behavior owner and never imply TypeScript decides legality. The edit adds list
   entries only and preserves the §2 sentence — neither principle is weakened.

## Architecture Check

1. Append-to-existing-list edits are cleaner than reorganizing the README or
   restructuring its command docs: the three lists already enumerate the shipped
   games in a fixed order, so adding `token_bazaar` / Token Bazaar at the tail is the
   minimal truthful change and matches how every prior game was added.
2. No backwards-compatibility aliasing or shims introduced — this is additive prose.
3. `engine-core` is untouched (no mechanic nouns enter the kernel) and `game-stdlib`
   is untouched (no helper promotion). The edit lives entirely in `apps/web`
   documentation.

## Verification Layers

1. Intro / Shell Surface / Smoke Layers lists are truthful -> codebase grep-proof:
   `grep -n "Token Bazaar\|token_bazaar\|token-bazaar" apps/web/README.md` returns a
   match in each of the three sections.
2. `smoke:e2e` claim matches the package script -> grep-proof that
   `apps/web/package.json` chains `e2e/token-bazaar.smoke.mjs` and the README bullet
   names Token Bazaar (and still does NOT claim Directional Flip is chained).
3. §2 behavior-authority sentence preserved -> grep-proof
   `grep -c "Rust/WASM owns game behavior" apps/web/README.md` is `1` after the edit.
4. Doc-link integrity -> `node scripts/check-doc-links.mjs` passes (single-layer doc
   check; the README adds no new links, so this confirms no link regression).

## What to Change

### 1. Intro games list (lines 3–6)

Add `token_bazaar` / Token Bazaar to the end of the games list and **preserve the
trailing `Rust/WASM owns game behavior; …replay projections.` sentence verbatim**.
Resulting paragraph:

```markdown
`apps/web` is the static React shell for Rulepath's local browser games:
`race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column
Four, `directional_flip` / Directional Flip, `draughts_lite` / Draughts Lite,
`high_card_duel` / High Card Duel, and `token_bazaar` / Token Bazaar. Rust/WASM
owns game behavior; TypeScript presents Rust-provided catalog entries, views,
action trees, effects, diagnostics, bot turns, and replay projections.
```

### 2. Shell Surface renderer bullet (lines 55–56)

Add Token Bazaar to the first-class board-renderer list:

```markdown
- first-class board renderers for Three Marks, Column Four, Directional Flip,
  Draughts Lite, High Card Duel, and Token Bazaar;
```

### 3. Smoke Layers `smoke:e2e` bullet (lines 72–75)

Add Token Bazaar to the `smoke:e2e` game list and keep the Directional Flip caveat:

```markdown
- `smoke:e2e`: Puppeteer rendered-browser smoke plus accessibility/no-leak smoke
  for the shell, Three Marks, Column Four, Draughts Lite, High Card Duel, and
  Token Bazaar. A standalone Directional Flip E2E smoke file also exists under
  `e2e/`, but is not chained by `smoke:e2e`.
```

## Files to Touch

- `apps/web/README.md` (modify)

## Out of Scope

- Root `README.md`, `progress.md`, `docs/ROADMAP.md`, `docs/MECHANIC-ATLAS.md`,
  `docs/SOURCES.md` — all validated already-correct; do not touch.
- `apps/web/package.json` — do not chain `directional-flip.smoke.mjs`; do not alter
  the `smoke:e2e` script. The README caveat stays truthful only while the script is
  unchanged.
- Any Rust crate, WASM logic, tool, CI workflow, trace, fixture, benchmark, or
  archived spec.
- The optional `specs/README.md` maintenance row and the handoff/doc-link
  acceptance — owned by GAT9AFTROAREA-002.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "Token Bazaar\|token_bazaar\|token-bazaar" apps/web/README.md` shows a
   match in the intro, the Shell Surface bullet, and the Smoke Layers `smoke:e2e`
   bullet.
2. `grep -c "Rust/WASM owns game behavior" apps/web/README.md` returns `1` (the §2
   sentence was not dropped).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The edit is documentation-only — `git diff --name-only` lists only
   `apps/web/README.md`.
2. The README's `smoke:e2e` description never claims `directional-flip.smoke.mjs`
   is chained by `smoke:e2e` (it is not, per `apps/web/package.json`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "Token Bazaar\|token_bazaar\|token-bazaar" apps/web/README.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command set is the correct boundary because no code or test changes —
   the shipped browser surface (renderer, WASM registration, chained smoke) already
   exists and is covered by Gate 9's own tests; this ticket only reconciles the prose.

## Outcome

Completed: 2026-06-08

What changed:

- Added `token_bazaar` / Token Bazaar to the `apps/web/README.md` intro local-games list.
- Added Token Bazaar to the Shell Surface board-renderer list.
- Added Token Bazaar to the `smoke:e2e` Smoke Layers description while preserving the Directional Flip caveat.

Deviations from original plan:

- None.

Verification results:

- `grep -n "Token Bazaar\|token_bazaar\|token-bazaar" apps/web/README.md` showed matches in the intro, Shell Surface, and Smoke Layers sections.
- `grep -c "Rust/WASM owns game behavior" apps/web/README.md` returned `1`.
- `node scripts/check-doc-links.mjs` passed: `Checked 26 markdown files`.
