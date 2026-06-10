# OUTSURPOL-003: Outcome announcement reliability, standings order, and decisive-section disclosure

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — `apps/web/src` presentation/interaction only (live-region timing, render ordering, disclosure defaults). No Rust, WASM, schema, or hash surface moves.
**Deps**: OUTSURPOL-001 (badge/row styling the ordering and parity rules render against), OUTSURPOL-002 (humanized copy the announcement sentence reuses).

## Problem

Three interaction/accessibility gaps remain after styling (001) and copy (002):

1. **Announcement timing.** The panel's `role="status"` container (`OutcomeExplanationPanel.tsx:106`) mounts *together with its content* when the match turns terminal. Live regions injected with their content frequently fail to announce — the region must exist in the DOM before the message lands (WCAG 4.1.3 / ARIA22 practice; Soueidan, "Accessible notifications with ARIA live regions"). UI-INTERACTION.md §16 requires "the terminal summary is programmatically exposed as a status/result message" — the current wiring satisfies the markup letter but not reliable announcement.
2. **Standings order.** `finalStanding` renders in payload (seat) order, so the loser can sit above the winner. Outcome-screen practice is rank order, winner first, with ties rendered identically.
3. **Disclosure defaults.** Every breakdown section is collapsed by default (`initiallyExpanded = false`, `OutcomeExplanationPanel.tsx:84`). When the section *is* the decisive explanation (a tiebreak or showdown detail that decided the game), collapsing it hides exactly what the surface exists to answer; progressive-disclosure guidance (NN/g; GOV.UK details component) says frequently-needed content must be up front.

## Assumption Reassessment (2026-06-10)

1. Verified current code: every board already renders a persistent `.board-status` `<div role="status">` during play (e.g. `RaceBoard.tsx:50-52`) — a pre-existing live region the terminal announcement can reuse without new always-mounted DOM. The panel's own `role="status"`/`aria-live="polite"` div is `OutcomeExplanationPanel.tsx:106-110`; `initiallyExpanded` exists and is threaded through section state (`:84,135,148`); standings map in payload order (`:113`); `emphasized` and `result` arrive from Rust/board data (`:115,121`).
2. Verified docs: UI-INTERACTION.md §16 — TS "manage[s] disclosure/focus state"; the decisive cause arrives from Rust (`decisive_cause`, `decisiveCause`) and may *drive presentation state* (which section opens) without TS recomputing it. §16 also requires reduced-motion parity and color-independent standing — both must survive this ticket.
3. Cross-artifact boundary under audit: the e2e smoke (`outcome-explanation.smoke.mjs:131-152`) currently asserts the *panel* exposes non-empty `role="status"` text; re-routing the announcement changes what that assertion checks. The smoke must be updated in the same diff to assert (a) a status node that pre-exists terminal carries the result+cause sentence, and (b) the panel summary remains visible text. No assertion may be deleted without an equal-or-stronger replacement (AGENT-DISCIPLINE §4).
4. FOUNDATIONS §2 restated: ordering rows by Rust-supplied `emphasized`/`result` keys, choosing a default-open section from the Rust-supplied `decisive_cause`, and echoing the summary sentence into an existing status node are presentation decisions over computed facts — TS still computes no winner, rank, or cause.
5. §11 no-leak firewall: the announcement sentence reuses the already-public summary string; no new DOM attributes, storage, or copy channels. The no-leak smoke surfaces (body text, attributes, storage, console) continue to pass unchanged.
6. Assumption (one-line-correctable): focus is **not** programmatically moved to the panel at terminal — the panel sits adjacent to the board in reading order and is not a modal; we rely on the status announcement plus DOM order. If play-testing shows the panel lands far off-screen in some game, a `tabindex="-1"` heading focus can be a follow-up.

## Architecture Check

1. Reusing the boards' existing persistent status node beats always-mounting a hidden live region inside the panel: the region provably exists before terminal, no duplicate announcements (the panel's summary div drops `aria-live` in the same change), and boards already own per-play status copy.
2. Rank-first ordering and decisive-section defaults are derived in the shared adapter (`outcomeSurfaceData`) from Rust-supplied fields — one implementation for all 10 games; no per-board sorting logic.
3. No backwards-compatibility aliasing/shims; no `engine-core`/`game-stdlib`/Rust change; additive optional fields only (e.g. a per-section `defaultOpen`).

## Verification Layers

1. Announcement region pre-exists terminal → e2e: query the `role="status"` node *before* the final action, assert the same node carries the result+cause text after terminal (extend `outcome-explanation.smoke.mjs`).
2. No double announcement → grep/manual review: exactly one `aria-live`/`role="status"` carrier for the outcome sentence after the change.
3. Winner-first ordering and draw parity → e2e: first `.outcome-standing-row` is the `emphasized` row in the Three Marks win flow; the draw flow has zero `.emphasized` rows and identical row treatment.
4. Decisive section open by default → e2e: in a flow whose `decisive_cause` maps to a breakdown section, that section's button reads `aria-expanded="true"` at mount; keyboard toggle still works (existing assertions).
5. No outcome logic in TS → grep-proof (`determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome` absent); ordering uses only Rust-supplied `emphasized`/`result` keys.

## What to Change

### 1. Announcement routing

At terminal, boards put the one-line outcome sentence (heading + cause summary, reusing the 002 humanized copy — e.g. "Seat 0 wins — reached 21 exactly.") into their existing persistent `.board-status` `role="status"` node. The panel's `.outcome-summary` div keeps its visible text but drops `role="status"`/`aria-live` so the announcement fires exactly once, from a region that existed before terminal. Keep the sentence short and self-sufficient — it is transient for screen-reader users.

### 2. Standings ordering and draw parity (shared adapter)

`outcomeSurfaceData` orders `finalStanding` with `emphasized` (winner) rows first, preserving payload order otherwise (stable sort on Rust-supplied flags only). For draw/split results (no emphasized row), order is untouched and all rows render with identical treatment.

### 3. Decisive-section disclosure defaults

Add an optional `defaultOpen?: boolean` to `OutcomeExplanationBreakdownSection`; the adapter sets it when the section corresponds to the Rust-supplied `decisive_cause` (a static cause→section-id map per game, or boards set it directly at the call site). Sections honor `defaultOpen` over the panel-level `initiallyExpanded`. Additionally, a section whose content is trivially short (≤2 rows, no summary) may render expanded — collapsing 2 lines adds interaction cost for nothing.

### 4. Smoke updates

`outcome-explanation.smoke.mjs`: pre-terminal status-node capture + post-terminal announcement assertion; winner-first/draw-parity assertions; decisive-section `aria-expanded="true"` assertion; existing disclosure keyboard/pointer, reduced-motion, and no-leak assertions stay intact.

## Files to Touch

- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify — drop panel-level live region, ordering, `defaultOpen`)
- `apps/web/src/components/*Board.tsx` (modify — terminal sentence into the existing `.board-status` node; `defaultOpen`/cause mapping where board-supplied)
- `apps/web/e2e/outcome-explanation.smoke.mjs` (modify — assertions per §4)

## Out of Scope

- Focus moves, focus traps, or modal treatment of the panel (see Assumption 6).
- Any Rust/WASM change; any new rationale fields.
- Visual styling (OUTSURPOL-001) and copy humanization (OUTSURPOL-002) beyond reusing their output.
- Confetti/celebration animation of any kind.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs` passes with the new announcement, ordering, and disclosure assertions.
2. `npm --prefix apps/web run smoke:e2e` passes (per-game smokes unaffected by the status-node reuse).
3. `node scripts/check-outcome-explanations.mjs` passes (registry untouched or consistently extended).

### Invariants

1. Exactly one live region announces the terminal result, and it exists in the DOM before the match turns terminal (WCAG 4.1.3; UI-INTERACTION §16 "programmatically exposed as a status/result message").
2. Ordering and disclosure state derive only from Rust-supplied fields (`emphasized`, `result`, `decisive_cause`); TS computes no rank, winner, or cause (FOUNDATIONS §2), and reduced-motion output presents identical facts (UI-INTERACTION §16).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/outcome-explanation.smoke.mjs` — pre-existing-region announcement check; winner-first and draw-parity row assertions; decisive-section default-expanded assertion.

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs` (targeted)
2. `npm --prefix apps/web run smoke:e2e` (full pipeline)
3. Manual screen-reader spot check (NVDA/VoiceOver or headless a11y tree dump): terminal in Race to 21 announces "Seat 0 wins — reached 21 exactly." exactly once.

## Outcome

Completed: 2026-06-10

What changed:
- Added shared `outcomeSummaryText` and `outcomeAnnouncementText` helpers so every catalog board can reuse the same Rust-supplied, humanized outcome sentence for its persistent status region and panel.
- Removed the late-mounted live/status role from `OutcomeExplanationPanel`; the panel remains visible text while pre-existing board status regions carry terminal announcements.
- Ordered emphasized/winner standings first with stable ordering otherwise, preserving draw/split parity.
- Added `defaultOpen` support for breakdown sections and default-open handling for short/decisive sections.
- Wired all catalog board components to compute `outcomeExplanation` once and reuse it for the terminal status text and panel.
- Extended the outcome smoke with pre-terminal status-node capture, panel no-live-region assertion, winner-first ordering, draw parity, and default-open disclosure checks.

Deviations from original plan:
- Token Bazaar's latest-effect region existed before terminal but did not have `role="status"`; it now has `role="status"` so it participates in the same persistent announcement pattern as the other boards.
- No manual screen-reader pass was run; the browser smoke verifies the DOM/live-region timing contract in headless Chromium.

Verification:
- `node scripts/check-outcome-explanations.mjs` passed.
- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/outcome-explanation.smoke.mjs` passed with localhost binding allowed after sandbox `listen EPERM`.
- `npm --prefix apps/web run smoke:e2e` passed.
- Grep proof found no `determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome` matches under `apps/web/src/components`.
- `git diff --name-only | rg '^(crates|games)/'` found no Rust/game diffs.
- Grep proof found no remaining outcome-panel live/status carrier; the only outcome status query is the e2e pre-terminal status assertion.
