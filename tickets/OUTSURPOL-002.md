# OUTSURPOL-002: Humanize outcome copy — display maps for seat/enum tokens, deduplicated rows

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `apps/web/src` presentation only (display-label maps, board call-site fixes, template copy). No Rust, WASM, schema, trace, or hash surface moves.
**Deps**: None (independent of OUTSURPOL-001; review after it for visual context).

## Problem

The outcome surface leaks internal identifiers and raw enum tokens into player-facing prose, and repeats facts. Empirically confirmed at `http://127.0.0.1:4173/`:

- Race to 21: heading "**seat_0** wins", summary "**seat_0** reached 21 exactly.", standing label "**seat_0**".
- Three Marks: summary "seat_0 completed **r1c1, r1c2, r1c3**."; standing rows show header result "Winner"/"Loss" **and** a duplicate `Result: win` / `Result: loss` key-value row beneath.
- Crest Ledger (poker_lite): standing values render raw tokens `Pair: **high_card**`, `Private rank: **low**`, header results "**split**"; breakdown rows are labeled "**seat_0** contribution".

Elsewhere the shell already humanizes ("Seat 0 is active", `seatLabel()` in `PlainTricksBoard.tsx:258-260`), so the outcome panel reads notoriously rougher than the rest of the interface. Per `docs/UI-INTERACTION.md` §16 the surface must explain the result "in player-facing terms", and TypeScript "may render, lay out, interpolate safe template parameters" — label/copy mapping is squarely TS presentation work.

## Assumption Reassessment (2026-06-10)

1. Verified current code: raw values enter through three paths. (a) Board call sites — `RaceBoard.tsx:58` builds `heading: \`${view.winner} wins\`` and `label: view.winner` from the raw `SeatId`; each of the 10 board components (`RaceBoard`, `ThreeMarksBoard`, `ColumnFourBoard`, `DirectionalFlipBoard`, `DraughtsLiteBoard`, `HighCardDuelBoard`, `TokenBazaarBoard`, `SecretDraftBoard`, `PokerLiteBoard`, `PlainTricksBoard`) passes its own `outcomeSurfaceData(...)` input and must be audited. (b) The adapter fallback — `OutcomeExplanationPanel.tsx:197` `label: standing.label ?? standing.seat` surfaces the raw seat id whenever the Rust rationale omits `label` (it does: `crates/wasm-api/src/lib.rs:5710-5716` emits `final_standing` without labels). (c) Value rendering — `formatValue` (`OutcomeExplanationPanel.tsx:233-241`) stringifies Rust enum tokens (`high_card`, `low`, `win`, `split`) verbatim, both in `FieldRow` and in `renderTemplate` interpolation (`:229-231`).
2. Verified docs/specs: UI-INTERACTION.md §16 forbids TS computing the winner/cause but explicitly allows rendering, layout, and safe-parameter interpolation; `archive/specs/victory-explanation-shared-surface.md` §11.3 forbids logic in templates. A static `Record<string, string>` display map is inert copy — no conditionals, selectors, or comparisons — and passes `scripts/check-outcome-explanations.mjs` `FORBIDDEN_TEMPLATE_PATTERNS` (verified against the regexes at `scripts/check-outcome-explanations.mjs:41-48`).
3. Cross-artifact boundary under audit: the Rust→TS rationale payload (`client.ts` mirrors of `OutcomeRationaleView`) is **read-only** for this ticket. The duplicate `Result:` row originates in the Rust projection (per-seat `values` include a Result entry while `result` is also set); changing that projection would alter public-view JSON and therefore replay/public-view hashes (`wasm-api` migration-note precedent at `lib.rs:3734`) — explicitly out of scope; the dedup happens at render time.
4. FOUNDATIONS §2 restated: TypeScript decides no outcome. Mapping `seat_0 → "Seat 0"` and `high_card → "High card"` is label presentation of Rust-supplied facts; ordering, winner identity, scores, and causes continue to arrive computed from Rust.
5. §11 no-leak firewall: display maps translate already-public tokens only; they must not add copy that names hidden concepts (the smoke's forbidden-terms list `apps/web/e2e/outcome-explanation.smoke.mjs:12-22` stays authoritative). No new DOM attributes or storage writes.
6. Three Marks cell tokens (`r1c1`) are also the board's own `data-testid` vocabulary; the visible grid does not display coordinate labels. Humanize the *presentation* of the winning-line list (e.g. styled token chips or "row 1" phrasing from a static cell-token map) without recomputing line semantics; assuming chip-styled tokens (one-line-correctable if "row/column" prose is preferred).

## Architecture Check

1. One shared display-map module consumed by `formatValue`/`renderTemplate`/the adapter beats per-board string fixes: every current and future game gets humanized rendering through the same chokepoint (`outcomeSurfaceData` + `FieldRow`), and board diffs reduce to passing already-available `seatLabel` output.
2. No backwards-compatibility aliasing/shims: raw-token rendering is replaced, not kept behind a flag; the panel's public component API is unchanged (additive optional fields only, if any).
3. `engine-core`/`game-stdlib`/Rust untouched; templates stay inert copy; no mechanic nouns move anywhere.

## Verification Layers

1. No outcome logic enters TS → grep-proof: `grep -E 'determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome' apps/web/src/components/*.ts*` returns nothing; display maps are flat `Record` literals (manual review).
2. Templates/copy stay inert and registry-consistent → `node scripts/check-outcome-explanations.mjs` passes.
3. No raw tokens player-facing → e2e assertion: terminal panels for the three exercised games contain none of `seat_0`, `seat_1`, `high_card` as visible text (extend `outcome-explanation.smoke.mjs`; keep `data-testid`/attribute surfaces unrestricted — they are not player-facing copy and remain governed by the no-leak list, not this check).
4. No hidden-info regression → existing no-leak assertions in `outcome-explanation.smoke.mjs` and `a11y-noleak.smoke.mjs` stay green.
5. Rust payload untouched → grep-proof: `git diff --stat` shows no change under `crates/` or `games/`.

## What to Change

### 1. Shared display maps (new module or extension of `outcomeExplanationTemplates.ts`)

Static, inert copy maps plus tiny pure helpers:

- `seatDisplayLabel(seat: string): string` — `seat_0 → "Seat 0"`, `seat_1 → "Seat 1"`, pass-through otherwise; replaces the raw fallback at `OutcomeExplanationPanel.tsx:197` and feeds template interpolation of seat-valued params.
- `outcomeValueCopy: Record<string, string>` — known Rust display tokens → player copy (`high_card → "High card"`, `pair → "Pair"`, `low → "Low"`, `win → "Win"`, `loss → "Loss"`, `split → "Split"`, `draw → "Draw"`, plus tokens discovered during the 10-board audit). Applied in `formatValue` lookup-then-fallback; unknown tokens render unchanged.
- Apply both inside `renderTemplate` parameter interpolation so summaries read "Seat 0 reached 21 exactly."

### 2. Board call-site audit (all 10 boards)

For each `outcomeSurfaceData(...)` input: headings use the board's existing seat-label helper ("Seat 0 wins"); `templateParams` seat values and `finalStanding` labels stop passing raw ids; breakdown row labels lose raw-id prefixes ("Seat 0 contribution"). Reuse each board's local `seatLabel`-style helper or the new shared one.

### 3. Render-time dedup of the duplicate result row

In the adapter (or `FieldRow` mapping), when `standing.result` is present, skip a values row whose label is exactly `Result` (case-insensitive) — the header badge already carries it. Rust payload unchanged.

### 4. Template copy quality pass (`outcomeExplanationTemplates.ts`)

Summaries phrased as result—cause in player vocabulary; disclosure `expandedHeading`s carry information scent ("Terminal detail" → e.g. "Why the match ended"; "Target" → "Target check"). Static copy only; `requiredParams`/keys unchanged unless a game audit shows a missing param.

## Files to Touch

- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify — copy + display maps, or new sibling `outcomeDisplay.ts`)
- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify — `formatValue`/`renderTemplate`/adapter label fallback + dedup)
- `apps/web/src/components/{Race,ThreeMarks,ColumnFour,DirectionalFlip,DraughtsLite,HighCardDuel,TokenBazaar,SecretDraft,PokerLite,PlainTricks}Board.tsx` (modify — call-site labels)
- `apps/web/e2e/outcome-explanation.smoke.mjs` (modify — visible-copy assertions)

## Out of Scope

- Any Rust/WASM change, including removing the duplicate Result entry from `OutcomeRationaleView` projections (would move public-view/replay hashes; future ticket if ever wanted).
- Custom player display names (beyond the established "Seat 0"/"Seat 1" convention).
- Visual styling (OUTSURPOL-001) and disclosure/announcement behavior (OUTSURPOL-003).
- Rewriting per-game `games/*/docs/UI.md` outcome sections (marker checks unaffected by TS copy).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` passes (template registry consistent; no forbidden patterns).
2. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs` passes, including new assertions that visible panel text contains no `seat_0`/`seat_1`/raw enum tokens for the exercised games.
3. `npm --prefix apps/web run smoke:e2e` passes (no-leak and per-game smokes green).

### Invariants

1. TypeScript renders and labels Rust-supplied facts only; winner, standings, causes, and values continue to arrive computed from Rust (FOUNDATIONS §2; UI-INTERACTION §16).
2. Display maps are inert static copy — no conditionals, comparisons, selectors, or tiebreak vocabulary (FOUNDATIONS §5; spec §11.3) — and introduce no hidden-information vocabulary (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/outcome-explanation.smoke.mjs` — assert humanized visible copy (no raw seat ids/enum tokens in panel text) for Race to 21 and Three Marks terminals.

### Commands

1. `node scripts/check-outcome-explanations.mjs` (targeted — template/copy contract)
2. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs` (targeted — rendered copy)
3. `npm --prefix apps/web run smoke:e2e` (full pipeline — all game smokes + no-leak)
