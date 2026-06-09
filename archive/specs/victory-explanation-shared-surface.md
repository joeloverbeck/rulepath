# victory-explanation-shared-surface — Shared end-of-match outcome explanation surface

- **Filename:** `specs/victory-explanation-shared-surface.md`
- **Spec ID:** `victory-explanation-shared-surface`
- **Target type:** New spec
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate
- **Roadmap build gate:** None. This is a non-gate sibling of `rules-display-shared-surface`, to be completed before resuming the next gameplay gate.
- **Status:** Done
- **Date:** 2026-06-09
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → this spec.
- **Subordination:** This spec is subordinate to the foundation docs, area docs, accepted ADRs, and the official-game contract. It drafts lift-ready amendments, but does not silently amend or weaken upstream law.

> **Reader orientation.** This spec retains its working `§1`–`§21` section numbering rather than the canonical Rulepath section labels. The canonical sections map as: Objective → §1; Scope / non-goals → §3; design decisions, deliverables, and CI plan → §6–§12; Acceptance criteria → §14; Work breakdown (ticket series) → §15; lift-ready doc/template amendments → §16–§18; Assumptions → §21. All concrete references in this spec were validated against the current `main` working tree during reassessment.

---

## 1. Objective

Build a mandatory, shared, cross-game **Outcome Explanation** surface in `apps/web` so every terminal match tells a player **why the result happened**.

At terminal, the web UI must show:

1. the final outcome: winner, draw, split, or game-specific terminal result;
2. the **decisive cause**: the rule, comparison, threshold, winning configuration, terminal condition, or tiebreaker that actually decided this match;
3. a **viewer-safe per-player breakdown**: final score/standing and the components or revealed facts needed to understand the result; and
4. enough accessible, pleasant presentation that a player can finish a match thinking: **“I understand why I won or lost. I want to play again.”**

The work is a direct sibling of the completed `rules-display-shared-surface` spec. The rules surface answered “How do I play?” before and during a match. This spec answers “Why did this match end this way?” after terminal. It uses the same discipline: catalog-complete, shared UI, no TypeScript behavior authority, no hidden-information leaks, accessible presentation, and CI coverage.

---

## 2. Current state and motivating gap

The repository today has no shared `OutcomePanel`, `ResultPanel`, or cross-game victory-explanation surface. Each board component owns its own small terminal/turn status. Some games already project a useful terminal fact — such as `column_four` / `three_marks` winning lines and `directional_flip` final scores — but there is no uniform result-explanation surface.

The decisive product failure is verified in `poker_lite`: Rust computes `ShowdownStrength { pair_flag, private_rank_value }` and compares those values in `showdown_strength` / `compare_showdown` (`games/poker_lite/src/rules.rs`), but `ShowdownView` and `TerminalView` (`games/poker_lite/src/visibility.rs`) expose only the revealed cards and terminal outcome. The player can see who won, but not whether the win came from pair-vs-rank strength or from the private-rank tiebreaker.

---

## 3. Goal, scope, and non-goals

### 3.1 Goal

Every official, web-exposed game must project a Rust-owned, replay-deterministic, viewer-safe outcome rationale and render it through one shared web surface.

The surface must be useful for simple and complex games. For `race_to_n`, it may say “Seat 1 reached 21 exactly.” For `poker_lite`, it must explain the showdown strength ladder that decided the pool. For future trick-taking and full poker work, this surface prevents “I won, but I have no idea why” from becoming systemic debt.

### 3.2 In scope

- A shared React outcome explanation surface in `apps/web`.
- Per-game Rust public/terminal-view enrichment for all nine current catalog games.
- Per-game viewer-safe breakdowns: decisive cause plus every player’s final standing/components.
- Static explanation templates keyed to Rust-owned effect/action/rationale IDs, with only safe interpolation.
- `apps/web/src/wasm/client.ts` type updates for the new viewer-safe payloads.
- Replacement or demotion of ad-hoc per-board terminal panels so the shared surface is canonical.
- A CI coverage/staleness/no-leak guard analogous to the rules-display catalog guard.
- Smoke, accessibility, replay, and hidden-information tests for the surface.
- Lift-ready proposed amendments to `docs/UI-INTERACTION.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, and the game templates, included as appendices in this spec.

### 3.3 Out of scope

| Non-goal | Status | Reason |
|---|---:|---|
| Coaching / “what would have changed it” advice | Forbidden | This expands the leak surface and drifts toward strategy/AI advice. The surface explains the actual result only. |
| Turning-point analysis | Forbidden | It requires counterfactual reasoning and can expose hidden or strategic information. |
| Runtime LLM explanation | Forbidden | Outcome text must come from Rust-owned facts plus authored static templates. |
| TypeScript scoring or tiebreaking | Forbidden | TypeScript renders the supplied payload only. It must not compare card ranks, scores, lines, or tiebreaker ladders to decide why a player won. |
| Generic `engine-core` outcome/winner/score type | Rejected for this spec | Current evidence supports per-game typed rationales. Kernel vocabulary stays noun-free. |
| `game-stdlib` scoring helper | Deferred | If repeated shape pressure later warrants it, route through `docs/MECHANIC-ATLAS.md` and the primitive-pressure process. |
| YAML or behavior DSL | Forbidden | Static data remains typed presentation content/templates only. No selectors, triggers, conditionals, or behavior-like rules. |
| Hidden-information inspection | Forbidden | A viewer sees only facts already public to that viewer. Yielded hands and unrevealed commitments stay hidden. |
| Trace/replay/hash semantic redesign | Not in scope | View/effect schema additions must be deterministic and migrated through ordinary golden-trace updates, not by changing replay doctrine. |

---

## 4. Foundation and boundary alignment

| Authority | Constraint engaged | Spec alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §11 | Rust owns behavior; TypeScript presents only; determinism; hidden-info safety; no YAML/DSL; engine-core noun-free; public UI must be polished and accessible. | Rust computes and projects the rationale. TS renders only. Rationale fields are deterministic view/effect data. Static templates are inert presentation. |
| `docs/FOUNDATIONS.md` §12 | Stop if TS decides legality/outcome, hidden data leaks, engine-core gets game nouns, or public UI becomes debug-first. | These are explicit stop conditions for every ticket in this spec. |
| `docs/FOUNDATIONS.md` §13 | ADR required for engine-core vocabulary/responsibility changes, visibility-contract changes, replay/hash semantic changes, YAML/DSL, or platform changes. | No ADR is required for the chosen design; see §13. |
| `docs/ARCHITECTURE.md` | Action/effect/view/replay pipeline; semantic effects are ordered, deterministic, replayable, and viewer-safe; renderers settle to public view. | Outcome rationale is projected view/effect data at terminal. It does not add behavior after the fact. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` §2 | Scoring and terminal detection live in `games/*`; `apps/web` is presentation. | Every game owns its own rationale type and computes the decisive cause in Rust. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12 | Static data may include UI metadata and explanation templates keyed to Rust effects/actions; templates cannot carry behavior and can interpolate only safe facts. | Outcome templates are keyed to game-owned terminal effects/rationale IDs and interpolate only Rust-projected public/viewer-safe fields. |
| `docs/OFFICIAL-GAME-CONTRACT.md` §5/§10 | Official games must document scoring/terminal conditions and expose web-safe public views/effects. | This spec adds outcome explanation as a mandatory official/web-exposed requirement. |
| `docs/UI-INTERACTION.md` | Public UI target, legal-only interaction, effect-driven animation, accessibility and no-leak discipline. | Adds the missing outcome/victory-display target and acceptance-check lines. |
| `docs/WASM-CLIENT-BOUNDARY.md` | Browser payload whitelist; dev panel may show only viewer-safe payloads. | Outcome rationale is part of viewer-safe public/private projections and must not appear in logs, DOM, test IDs, replay exports, or dev tools with hidden facts. |
| `docs/AI-BOTS.md` | Hidden-information safety applies to bot explanations and candidate ranking. | Outcome explanations obey the same no-leak rules; no hidden state, opponent private data, seed-derived tails, or strategy advice. |
| `docs/ROADMAP.md` | Gate ladder approaches trick-taking and fuller poker complexity. | This non-gate infrastructure work happens now, before complexity increases. |

---

## 5. External UX and accessibility grounding

This spec does not use external research to re-open the locked architecture decisions. It uses external UX/accessibility guidance only to shape presentation details.

- The shared outcome surface uses **progressive disclosure**: a concise terminal summary first, with expandable per-player and rule-detail breakdowns below. Nielsen Norman Group describes progressive disclosure as deferring advanced or rarely used detail to secondary screens/sections to reduce complexity and error risk, which fits dense score/tiebreak explanations.[^nng-progressive]
- The terminal summary is treated as a **status message**. WCAG 4.1.3 covers status information about the results of an action/process that is added without a context change, so the surface must expose the result summary programmatically without forcing focus to jump unexpectedly.[^wcag-status]
- The outcome cannot rely on seat color alone. WCAG 1.4.1 requires that color not be the only means of conveying information, so the surface must use text labels, icons/shapes, headings, and ordered values in addition to color.[^wcag-color]
- Motion around terminal reveals must respect reduced-motion expectations. WCAG 2.3.3 says motion animation triggered by interaction can be disabled unless essential, so the result panel must remain fully understandable without animation.[^wcag-animation]
- Expandable breakdown sections should use a standard disclosure pattern: a button controls hidden/visible content, with `aria-expanded` and `aria-controls` when custom disclosure is used.[^wai-disclosure]

---

## 6. Committed design decisions

### D1 — Build one shared `OutcomeExplanationPanel` in `apps/web`

The implementation adds one shared React surface. Game boards may keep small local status pills, but terminal result explanation must route through the shared surface.

Recommended component boundary:

```text
apps/web/src/components/OutcomeExplanationPanel.tsx
apps/web/src/components/outcomeExplanationTemplates.ts
apps/web/e2e/outcome-explanation.smoke.mjs
scripts/check-outcome-explanations.mjs
```

The component is shared; the data remains per-game and Rust-owned.

### D2 — Per-game Rust rationale, not a generic engine type

Each `games/*` crate projects its own typed, viewer-safe outcome rationale in `visibility.rs` and/or terminal semantic effects. There is no `engine_core::Outcome`, `engine_core::Score`, `engine_core::Winner`, or `game_stdlib::Scoring` helper in this spec.

The TypeScript surface may use a common presentation shape after the payload has crossed the WASM boundary, but that shape is not behavior authority. It must not compute the decisive cause.

### D3 — Outcome explanations are part of terminal view/effect data

The decisive cause is not an after-the-fact UI guess. It is data emitted by Rust at the same point Rust declares terminal outcome or projects terminal view.

For every terminal match, Rust must expose:

- a stable `outcome_explanation` / `victory_rationale` field on the public/terminal view, or an equivalent field reachable through the game’s public terminal view;
- a terminal semantic effect with the same decisive cause when terminal is reached, when that game already uses terminal effects; and
- deterministic serialization compatible with replay/golden trace updates.

### D4 — Static templates are presentation only

Outcome copy may be authored as static TypeScript/JSON-like typed constants or Rust-side inert typed content, keyed by Rust-provided rationale/effect IDs.

Templates may contain:

- labels;
- short explanations;
- parameter placeholders for Rust-projected public fields;
- ordering/display hints; and
- accessible summary copy.

Templates must not contain:

- rule conditions;
- score comparisons;
- hidden-info filters;
- action selectors;
- tiebreaker branching;
- seed/fixture data;
- YAML front matter;
- a DSL; or
- any value that would let TS decide the winner or decisive cause.

### D5 — Explain only the actual result

The surface must explain why this terminal outcome occurred. It must not say what the losing player “should have done,” what future players should try, what alternate move would have changed the result, or which hidden card would have won.

### D6 — Catalog-complete and future-binding

No official web-exposed game is complete unless the shared outcome surface can explain its terminal result. This applies to the nine current games and to every future game at creation time.

---

## 7. Shared outcome surface design

### 7.1 Placement and timing

At terminal, the shared surface appears in the primary play area or the existing board-adjacent information column. It should be visible without requiring the player to inspect the dev panel, effect log, or raw JSON.

Required terminal layout:

1. **Outcome heading** — e.g. “Seat 1 wins”, “Draw”, “Split pool”, “Seat 0 wins by line”.
2. **One-sentence decisive cause** — the shortest faithful explanation of the result.
3. **Final standing strip** — one row/card per player with final score, standing, or allocation.
4. **Expandable breakdown** — rule/comparison/tiebreaker details, per-player components, and revealed facts.
5. **Rule reference footer** — stable rule IDs from the game’s `RULES.md`, such as `R-END-*`, `R-SCORE-*`, `TB-END-003`, or game-specific equivalents.

The board itself should still show game-native terminal state: winning lines highlighted, final cards revealed when legally revealed, pieces remaining, final market state, and so on. The shared surface explains the same state in words and structured breakdowns.

### 7.2 Progressive disclosure model

The default view must be readable in under a few seconds:

```text
Seat 1 wins.
Why: Seat 1 completed a vertical line in C2.
Final: Seat 1 — win; Seat 0 — loss.
[Show breakdown]
```

The expanded breakdown may include:

- the exact cells in a line;
- score component rows;
- tiebreaker ladder rows with the decisive rung marked;
- revealed card/strength comparison rows;
- terminal reason rows, such as “opponent has no pieces” or “opponent has no legal move”; and
- draw/split explanations.

Progressive disclosure is required because future trick-taking/poker scoring will be too dense for a single wall of terminal text.

### 7.3 Accessibility requirements

The outcome panel must meet the existing UI doctrine and add these concrete requirements:

- The terminal summary is exposed through a `role="status"` region or equivalent status-message pattern that screen readers can announce without losing context.
- The panel heading is programmatically associated with the surface, e.g. `aria-labelledby`.
- Expand/collapse controls are real buttons. If custom disclosure is used, the controls expose `aria-expanded` and `aria-controls`.
- The decisive cause is available as text, not only color, icon position, animation, or line highlight.
- Player identities are text labels. Color may reinforce but never carry the outcome alone.
- Tables or lists used for score/tiebreak breakdowns have readable row/column labels.
- On small screens, the outcome surface remains reachable after terminal without hiding legal game state needed to verify the explanation.
- Reduced-motion mode disables nonessential reveal animation and still presents the same terminal facts.

### 7.4 Replay and reduced-motion consistency

Replaying a match to terminal must produce the same outcome panel content for the same viewer. The visual scheduling may differ under reduced motion, but the content, order of decisive facts, and final standing must not differ.

Effect-driven animations may call attention to terminal events, but they cannot be the only carrier of the reason. The panel content is authoritative for the UI explanation.

### 7.5 Hidden-information and DOM safety

The no-leak rule applies to every output channel:

- visible text;
- hidden DOM text;
- `aria-label`, `title`, `alt`, and other accessibility attributes;
- `data-testid`, CSS class names, and analytics-like attributes;
- local storage/session storage;
- logs and dev panel payloads;
- effect logs;
- replay import/export;
- screenshot/smoke-test fixtures; and
- bot explanation surfaces.

For hidden-info games, the surface may show public revealed facts and the viewer’s own private facts only if Rust has already projected them for that viewer. It must never show an opponent’s unrevealed private data, even in hidden attributes or test IDs.

### 7.6 Static template/interpolation contract

Static explanation templates are keyed by Rust-provided IDs. Example presentation-only model:

```ts
// TypeScript may render this; it must not derive it.
type OutcomeExplanationSurfaceData = {
  heading: string;
  decisiveTemplateId: string;
  decisiveParams: Record<string, string | number | boolean>;
  finalStanding: Array<{
    seat: string;
    label: string;
    result: string;
    values: Array<{ label: string; value: string | number }>;
  }>;
  breakdownSections: Array<{
    heading: string;
    rows: Array<Record<string, string | number | boolean>>;
  }>;
  ruleRefs: string[];
};
```

This shape is only an illustration for the shared surface. The source of `decisiveTemplateId`, `decisiveParams`, rows, order, and rule references is Rust. TypeScript can format, pluralize, and interpolate safe values; it cannot compute which template applies.

---

## 8. Per-game Rust rationale contract

Every official game must define a game-local terminal rationale type. The type may be named according to local conventions, such as:

```rust
pub struct OutcomeExplanationView { ... }
pub enum TerminalRationaleView { ... }
pub struct VictoryRationaleView { ... }
```

The exact Rust type is game-owned. The mandatory semantics are shared:

| Required fact | Meaning |
|---|---|
| `result_kind` | Win, loss, draw, split, yield win, showdown win, or game-local terminal class. |
| `decisive_cause` | The exact terminal rule/comparison/configuration that decided the match. |
| `decisive_rule_ids` | Stable `RULES.md` IDs that the explanation traces to. |
| `final_standing` | Every player’s final standing: winner/loss/draw/split plus score/allocation/position where applicable. |
| `per_player_breakdown` | Viewer-safe score components, revealed strengths, piece counts, line cells, or tiebreaker values needed to understand the result. |
| `terminal_trigger` | The terminal event/condition: line completed, board full, no pieces, no legal moves, round cap, turn cap, market exhaustion, exact target, yield, showdown, etc. |
| `template_key` | Stable presentation key for static copy. Key selection is Rust-owned. |
| `viewer_scope` | Implicit in the view projection: public observer, seat viewer, and private viewer payloads must differ only according to Rust visibility rules. |

### 8.1 Rust responsibilities

Rust must:

- compute the decisive cause at the same point as terminal outcome detection;
- store/project enough deterministic data for public view and replay to reproduce the explanation;
- filter hidden information before any value crosses the WASM boundary;
- expose stable rule IDs for scoring and terminal conditions;
- add or update unit tests for terminal rationale shape and no-leak cases;
- update golden traces/stable summaries intentionally when view/effect schemas grow; and
- keep the rationale game-local unless mechanic-atlas pressure justifies promotion.

### 8.2 TypeScript responsibilities

TypeScript must:

- render the supplied fields;
- look up static copy by Rust-supplied template key;
- interpolate only the supplied viewer-safe values;
- provide layout, disclosure, focus, status-message, and reduced-motion behavior;
- never compare values to determine the winner, decisive tiebreaker, or card strength; and
- never create hidden DOM/test IDs that include private values.

### 8.3 Public/private viewer variants

Hidden-information games may have distinct public observer and seat-viewer payloads. The key rule is simple: if Rust would not show the fact in the viewer’s public/private view, the outcome surface must not show it.

Examples:

- `poker_lite` yield: public and opposing seat viewers do not see the yielded loser’s private crest. The explanation says the match resolved by yield without private reveal.
- `poker_lite` showdown: revealed private crests are public because showdown reveals them; the strength comparison may be shown.
- `secret_draft` before reveal: commitments stay hidden; at terminal, the drafted/revealed items and tie-break facts are public because all six reveal rounds have completed.
- `high_card_duel`: revealed round cards/history may be used; unrevealed deck tails never appear.

---

## 9. Nine-game retrofit plan

### 9.1 `race_to_n` — Race to 21

**Current baseline.** Rust terminal detection is simple and correct: the counter advances by a legal addition, and when it reaches the target exactly, Rust sets the winner and emits `GameEnded`. `PublicView` exposes `counter`, `target`, `max_add`, `winner`, and legal additions. There is no terminal rationale object and no shared outcome surface.

**Gap.** The web UI can show who won, but the terminal explanation is ad hoc and not catalog-complete.

**Required rationale.** Add a Rust-projected rationale:

- result: `Win { winning_seat }`;
- decisive cause: winning seat advanced the counter to the target exactly;
- final standing: both seats, winner/loss;
- breakdown: `counter_before`, `addition`, `counter_after`, `target`, `max_add`;
- rule refs: the game’s scoring/terminal IDs for reaching the target exactly;
- template key: e.g. `race_to_n.exact_target_reached`.

**Surface behavior.** “Seat 0 wins because their final addition moved the counter from 18 to 21, reaching the target exactly.” The expanded breakdown shows target and final move.

**Tests.** Terminal golden trace and wasm smoke assert the exact-target rationale appears and no TS comparison computes the winner.

### 9.2 `three_marks` — Three Marks

**Current baseline.** Rust computes the first complete three-cell line, stores it in terminal outcome, and emits `LineCompleted` / `GameEnded`. `TerminalView::Win` already carries `winning_seat` and `line`; draw is also represented.

**Gap.** The decisive line exists in the view, but the UI has no shared outcome surface explaining it consistently.

**Required rationale.** Promote the existing terminal line into the shared rationale contract:

- win result: line completed;
- draw result: board full with no completed line;
- final standing: winner/loss or draw/draw;
- breakdown: line orientation if derivable by Rust, ordered cell IDs, final ply count, board-full flag for draw;
- rule refs: line-completion and draw terminal IDs;
- template keys: `three_marks.line_completed`, `three_marks.full_board_draw`.

**Surface behavior.** The board highlights the line; the panel says “Seat X wins by completing R1C1–R1C2–R1C3.” Expanded details list the final board and rule refs.

**Tests.** Smoke checks the shared outcome panel mentions the line cells; draw smoke checks “no line” / “full board” copy.

### 9.3 `column_four` — Column Four

**Current baseline.** Rust computes deterministic winning lines and terminal outcome includes `Win { seat, line }`; effects include win detection and game-ended data. `TerminalView` exposes the winning seat and line.

**Gap.** Like `three_marks`, the reason exists but is not rendered through a shared explanation surface.

**Required rationale.** Use the existing line data:

- result: win or draw;
- decisive cause: four connected cells for the winning seat, with orientation when Rust can provide it;
- final standing: winner/loss or draw/draw;
- breakdown: ordered line cells, last placed piece if available, board-full draw flag;
- rule refs: four-in-a-row terminal and draw/full-board terminal IDs;
- template keys: `column_four.line_completed`, `column_four.full_board_draw`.

**Surface behavior.** “Seat 1 wins with four connected discs on C4/R2–C4/R5.” The exact cell notation follows existing game ID conventions.

**Tests.** Existing vertical/horizontal/diagonal golden traces gain rationale assertions; smoke verifies the shared surface, not only board highlighting.

### 9.4 `directional_flip` — Directional Flip

**Current baseline.** `TerminalView` already carries `final_score` for win/draw. Terminal effects also know the terminal reason, such as full board or double forced pass.

**Gap.** This is the strongest current case, but still incomplete: the UI lacks a shared outcome surface, the terminal reason is not consistently projected as the decisive cause, and the final score is not part of a catalog-wide breakdown contract.

**Required rationale.** Add or formalize:

- result: win or draw;
- decisive cause: final score comparison after the terminal trigger;
- terminal trigger: full board, double forced pass, or no legal placements, according to game rules/effects;
- final standing: score per seat and win/loss/draw;
- breakdown: final disc counts by seat, terminal reason, legal-move/pass context if terminal by pass;
- rule refs: final-score comparison and terminal-trigger IDs;
- template keys: `directional_flip.final_score_win`, `directional_flip.final_score_draw`.

**Surface behavior.** “Seat 0 wins, 34–30, after both players had no legal flips.” Expanded details show both scores and the terminal trigger.

**Tests.** Existing full-board, double-pass, and draw traces assert reason + score in the new rationale.

### 9.5 `draughts_lite` — Draughts Lite

**Current baseline.** Rust terminal effects already include `TerminalWinReason` with `OpponentNoPieces` or `OpponentNoLegalMove`. `TerminalView` currently exposes only `Win { winning_seat }` and does not carry that reason.

**Gap.** The decisive reason is computed in Rust but not consistently projected into terminal public view or the web UI.

**Required rationale.** Add to `TerminalView` / public rationale:

- result: win;
- decisive cause: opponent had no pieces or opponent had no legal move;
- final standing: winner/loss;
- breakdown: remaining piece counts by seat, king/regular count if already public and useful, legal move count for the side to act at terminal, and terminal reason;
- rule refs: no-pieces and no-legal-moves terminal IDs;
- template keys: `draughts_lite.opponent_no_pieces`, `draughts_lite.opponent_no_legal_move`.

**Surface behavior.** “Seat 1 wins because Seat 0 has no legal move.” Expanded details show remaining pieces and the losing seat’s legal move count of zero.

**Tests.** Existing `terminal-no-pieces` and `terminal-no-legal-moves` traces assert the reason in both effect and public terminal view. Browser smoke checks both terminal reasons across deterministic fixtures or scripted match paths.

### 9.6 `high_card_duel` — High Card Duel

**Current baseline.** Rust scores each revealed round and emits `RoundScored`; terminal effects include winner/draw and final score. `PublicView` has score and revealed history. `TerminalView` only distinguishes `Win { winning_seat }` versus draw.

**Gap.** Score and revealed history exist, but terminal view does not carry a decisive final-score explanation, and the UI lacks a shared result breakdown.

**Required rationale.** Add:

- result: win or draw;
- decisive cause: final score after round limit;
- final standing: final score per seat;
- breakdown: each revealed round, card ranks, round winner/tie, point delta, cumulative or final score;
- tiebreaker: none; equal final score is draw;
- rule refs: round scoring, round limit, final score/draw IDs;
- template keys: `high_card_duel.final_score_win`, `high_card_duel.final_score_draw`.

**Surface behavior.** “Seat 0 wins 3–2 after five revealed duels.” Expanded details show the five public round comparisons.

**No-leak note.** Never expose unrevealed deck order/tail, even in expanded breakdown or replay export.

**Tests.** Existing terminal, tie-round, and hidden-info public-observer traces assert no hidden deck/card leaks and that only revealed history is used.

### 9.7 `token_bazaar` — Token Bazaar

**Current baseline.** The game is fully public. Rules define terminal tie-break order: higher score wins; if tied, more fulfilled contracts wins; if still tied, higher total remaining inventory wins; if still tied, draw. Rust computes outcome with that ladder. Terminal effects carry outcome, scores, fulfilled counts, and inventory totals. `TerminalView` exposes only winner/draw.

**Gap.** The decisive tiebreaker facts exist in Rust/effects, but the public terminal view and UI do not consistently explain which rung decided the match.

**Required rationale.** Add:

- result: win or draw;
- terminal trigger: turn cap or market/contract exhaustion as applicable;
- decisive cause: score, fulfilled-contract count, inventory total, or all tied draw;
- final standing: score, fulfilled count, inventory total per seat;
- breakdown: ordered tiebreaker ladder with the decisive rung marked;
- rule refs: `TB-SCORE-004`, `TB-SCORE-005`, `TB-END-001`, `TB-END-002`, `TB-END-003` as applicable;
- template keys: `token_bazaar.score_win`, `token_bazaar.fulfilled_tiebreak_win`, `token_bazaar.inventory_tiebreak_win`, `token_bazaar.all_tied_draw`.

**Surface behavior.** “Seat 1 wins on the second tiebreaker: both seats scored 12, but Seat 1 fulfilled 4 contracts to Seat 0’s 3.” Expanded details show all three ladder rows.

**Tests.** Add deterministic terminal fixtures for score win, fulfilled-count tiebreak, inventory-total tiebreak, and all-tied draw. Browser smoke asserts the decisive rung is identified and no TS ladder logic selects it.

### 9.8 `secret_draft` — Veiled Draft

**Current baseline.** Rules define scoring components and a multi-rung tie-break ladder: total score, complete sets, highest single drafted value, distinct represented threads, fewer priority-won contested items, then draw. Rust already has `TieBreakSummary` and terminal effects carrying final scores and tiebreak summary. `TerminalView` exposes only `Win { winning_seat }` or draw.

**Gap.** The decisive score/tiebreak data is computed in Rust/effects but does not cross into terminal public view for UI explanation.

**Required rationale.** Add:

- result: win or draw;
- terminal trigger: sixth reveal / draft completion;
- decisive cause: total score or the first tie-break rung that differs; if all rungs tie, draw;
- final standing: total score, complete sets, highest single value, distinct threads, priority-conflict wins per seat;
- breakdown: drafted/revealed item list and scoring components already public at terminal;
- rule refs: scoring, reveal completion, and tie-break rule IDs;
- template keys: `secret_draft.score_win`, `secret_draft.complete_sets_tiebreak`, `secret_draft.highest_single_tiebreak`, `secret_draft.distinct_threads_tiebreak`, `secret_draft.fewer_priority_conflict_wins_tiebreak`, `secret_draft.all_tied_draw`.

**Surface behavior.** “Seat 0 wins on the third tiebreaker: total score and complete sets were tied, but Seat 0’s highest drafted item was worth 5 to Seat 1’s 4.” Expanded details show the ladder.

**No-leak note.** The terminal explanation may use revealed/drafted facts only after reveal. Pre-reveal choices, commitments, and unavailable hidden values never appear early.

**Tests.** Existing terminal tie-break and public-observer no-leak traces assert the new view fields. Add a browser smoke path that opens the expanded ladder and checks that no pre-reveal hidden commitment appears in DOM or test IDs.

### 9.9 `poker_lite` — Crest Ledger

**Current baseline.** Rust computes `ShowdownStrength { pair_flag, private_rank_value }`; `compare_showdown` compares pair status first, then private rank, then split on equal strength. `TerminalOutcome` distinguishes yield win, showdown win, and split. `ShowdownView` exposes revealed private cards and the winner after showdown, but not the strength comparison. On yield, private cards are not revealed.

**Gap.** This is the worked failing case. The owner won and could not tell why. The exact reason is that the pair-vs-rank comparison never crosses into the public/viewer-safe terminal explanation surface.

**Required rationale.** Add a `ShowdownRationaleView` or equivalent under `ShowdownView` / `TerminalView`:

- result variants:
  - `YieldWin` — winner receives shared pool because the opponent yielded;
  - `ShowdownWin` — winner receives shared pool after strength comparison;
  - `Split` — equal strength splits the pool;
- showdown decisive cause:
  - pair beats non-pair;
  - both paired/unpaired, higher private rank wins;
  - equal strength splits;
- per-seat showdown strength when legally revealed:
  - `pair_flag` or player-facing bucket: paired / unpaired;
  - `private_rank_value` or player-facing rank label;
  - center crest/card if relevant to explaining pair formation;
  - contribution/allocation rows;
- yield decisive cause:
  - loser yielded;
  - private crests were not revealed;
  - winner receives the shared pool;
  - viewer may see only their own private crest if Rust already exposes it in that viewer’s private view;
- rule refs: yield, showdown comparison, pair-vs-rank order, split, ledger resolution;
- template keys: `poker_lite.yield_win_no_reveal`, `poker_lite.pair_beats_high_card`, `poker_lite.private_rank_tiebreak`, `poker_lite.equal_strength_split`.

**Surface behavior.**

Showdown example:

```text
Seat 1 wins the shared pool.
Why: Seat 1 made a pair with the center crest; Seat 0 did not. A pair beats an unpaired private crest.
```

Rank-tiebreak example:

```text
Seat 0 wins the shared pool.
Why: neither player made a pair, so the higher private crest decided the showdown: 9 beats 7.
```

Yield example:

```text
Seat 0 wins because Seat 1 yielded. The match resolved without a private reveal.
```

**No-leak note.** On yield, never show the yielded loser’s private crest, strength bucket, rank value, or any inferred “would have won” detail. On showdown/split, only show strength data because the rules have revealed the private crests.

**Tests.** Add Rust visibility tests for pair-beats-high-card, high-card rank win, equal split, and yield no-reveal. Add browser smoke assertions for the public observer and each seat viewer. The no-leak test must inspect visible text, hidden text, accessibility labels, data attributes, local storage, effect log, replay export, and dev panel payloads for unrevealed private values.

---

## 10. Web implementation plan

### 10.1 TypeScript/WASM type updates

Update `apps/web/src/wasm/client.ts` so every game’s public/terminal view type includes the new rationale fields.

Rules:

- Keep types game-specific where the Rust payload is game-specific.
- Do not collapse game payloads into a fake universal outcome model that hides game facts.
- Use a shared presentation adapter only after Rust has supplied decisive cause and ordered breakdowns.
- Do not write helper functions such as `determineWinner`, `compareCards`, `findWinningLine`, `resolveTiebreak`, or `scoreOutcome` in TypeScript.

### 10.2 `OutcomeExplanationPanel` rendering

The panel accepts Rust-provided presentation data or a game-local payload plus a no-logic adapter. It renders:

- heading;
- decisive sentence;
- final standing cards/rows;
- expandable breakdown sections;
- rule references; and
- no-leak-safe debugging affordances only in developer mode, if the dev-panel whitelist allows them.

The panel owns only UI mechanics: headings, layout, disclosure state, focus, announcements, and reduced-motion behavior.

### 10.3 Board wiring

Each existing board component must route terminal result display to the shared panel:

- `RaceBoard.tsx`
- `ThreeMarksBoard.tsx`
- `ColumnFourBoard.tsx`
- `DirectionalFlipBoard.tsx`
- `DraughtsLiteBoard.tsx`
- `HighCardDuelBoard.tsx`
- `TokenBazaarBoard.tsx`
- `SecretDraftBoard.tsx`
- `PokerLiteBoard.tsx`

Local status pills may remain for in-play context, but terminal outcome explanation must not be duplicated divergently. If a board has a bespoke terminal panel, either replace it with the shared panel or reduce it to board-native visualization while the shared panel carries the text explanation.

### 10.4 Dev panel and effect log

The dev panel may show only whitelisted viewer-safe public payloads. If it displays outcome rationale for debugging, it must use the same viewer-filtered payload the UI receives. It must never show unrevealed private data, deck tails, commitments, or seed-derived hidden values.

Effect logs may show terminal effects, but the shared outcome panel is the player-facing explanation. The effect log is not allowed to be the only source of “why.”

---

## 11. Static outcome template contract

### 11.1 Location

Recommended default:

```text
apps/web/src/components/outcomeExplanationTemplates.ts
```

A future ticket may choose Rust-owned typed static data if needed, but the first implementation should use typed TypeScript constants because the templates are presentation copy only and are keyed by Rust-provided IDs.

**Precedent divergence (note for ticket authors).** This is a *new* mechanism, not a copy of the rules-display surface. The completed `rules-display-shared-surface` surface does not use a TypeScript template-constants file: `apps/web/src/components/RulesPanel.tsx` fetches static per-game markdown at runtime from `apps/web/public/rules/<game_id>.md` and renders it inline. That runtime-fetch model fits static rules prose but not outcome copy, which is per-match dynamic and must interpolate Rust-projected values by `template_key`. The keyed-constants approach here is the deliberate adaptation; do not look for a rules-display template file to mirror.

### 11.2 Template shape

Each template entry must include:

- `templateId` — stable key supplied by Rust;
- `summary` — one short sentence with safe placeholders;
- `expandedHeading` — accessible disclosure label;
- `requiredParams` — checked by tests or TypeScript types to avoid broken copy;
- `allowedGameIds` — inert coverage metadata only;
- `ruleRefLabel` — optional display label for rule references.

`allowedGameIds` is a guardrail, not behavior. It must not choose a template at runtime based on state.

### 11.3 Forbidden template content

Templates must not include:

- comparisons such as “if score A > score B”; 
- tie-breaker order logic;
- card rank ordering;
- legal action filters;
- hidden-info conditions;
- random/seed data;
- DOM/test selector values that encode private data;
- strategy advice; or
- copied rulebook text.

---

## 12. Enforcement and CI plan

### 12.1 Catalog coverage checker

Add:

```text
scripts/check-outcome-explanations.mjs
```

Responsibilities:

1. Enumerate the official web catalog using the same mechanical catalog source pattern as existing catalog-doc checks.
2. For every catalog game, assert `games/<game_id>/docs/UI.md` contains an “Outcome / victory explanation” section.
3. Assert each section names:
   - terminal result variants;
   - decisive cause variants;
   - per-player breakdown fields;
   - hidden-info redaction rules;
   - rule IDs from `RULES.md`; and
   - web smoke coverage.
4. Assert each `games/<game_id>/docs/RULES.md` documents scoring and terminal conditions with stable IDs used by the rationale.
5. Defer the `games/<game_id>/docs/HOW-TO-PLAY.md` “Scoring and winning” section assertion to the existing `scripts/check-player-rules.mjs`, which already lists “Scoring and winning” in its `REQUIRED_SECTIONS` and enforces it per catalog game (and which `apps/web/e2e/rules-display.smoke.mjs` also asserts). This checker MUST NOT duplicate that assertion; it focuses on the outcome-specific surfaces in steps 2–4 and 6–7.
6. Assert `apps/web/src/wasm/client.ts` has an outcome rationale type/field for every catalog game.
7. Assert static templates cover every `template_key` named by the game docs or fixtures.
8. Fail if a catalog game is added without a rationale contract.
9. Fail if forbidden terms or patterns indicate TypeScript outcome logic, YAML/DSL, or hidden-info leakage in template content.

The checker may be conservative and string-based at first, like the rules-display guards. It should become more typed as the new payloads settle.

### 12.2 Rust tests

Every game retrofit ticket must add or update:

- terminal outcome rationale unit tests;
- visibility/no-leak tests for public observer and seat viewers where hidden information exists;
- serialization tests for new view/effect fields;
- replay/golden trace updates for terminal paths; and
- stable-summary migration notes if summaries include new view/effect data.

### 12.3 Browser smoke tests

Add:

```text
apps/web/e2e/outcome-explanation.smoke.mjs
```

Register the new smoke file in all three places that the existing `rules-display` smoke is wired, or `scripts/check-catalog-docs.mjs` will fail catalog-drift validation:

1. the `NON_GAME_SMOKE` set in `scripts/check-catalog-docs.mjs` (currently `{"shell", "a11y-noleak", "rules-display"}`);
2. the `smoke:e2e` bullet in the "Smoke Layers" section of `apps/web/README.md`; and
3. the `smoke:e2e` script in `apps/web/package.json`.

Minimum smoke coverage:

- a terminal path for every catalog game;
- panel appears at terminal;
- decisive sentence appears;
- final standing includes every player;
- expandable breakdown opens with keyboard and pointer;
- rule reference(s) appear;
- no hidden info appears in visible text, hidden DOM, labels, `data-testid`s, local storage, replay export, effect log, or dev panel;
- reduced-motion mode still shows all facts; and
- `poker_lite` includes at least pair-beats-high-card, private-rank tiebreak, split, and yield-no-reveal cases.

### 12.4 New-game creation enforcement

The game templates become the gate. A future game cannot reach web exposure unless:

- `GAME-RULES.md` names scoring/terminal rule IDs;
- `GAME-HOW-TO-PLAY.md` teaches the scoring/winning facts in player-facing language;
- `GAME-UI.md` names the exact outcome rationale payload and hidden-info rules;
- Rust public/terminal view includes a viewer-safe rationale;
- the shared outcome panel renders that game; and
- CI smoke checks the terminal explanation.

Tiny games do not get a lighter mode. They may have tiny explanations, but not silent omissions.

---

## 13. ADR determination

**No new ADR is required for this spec.**

Reasoning:

1. The design does not add game nouns to `engine-core` or change `engine-core` responsibilities.
2. It does not create a generic shared outcome/winner/score type in `engine-core` or `game-stdlib`.
3. It does not introduce YAML, a DSL, behavior selectors, triggers, or conditional static data.
4. It does not change legality, validation, scoring, hidden-information rules, replay semantics, RNG, or trace hashing doctrine.
5. It does not change the platform stack or replace React/SVG.
6. It enriches per-game public/terminal views with viewer-safe data already owned by each game. That is an implementation of the existing visibility/public-view model, not a change to the model itself.
7. It proposes stricter official-game and UI requirements, but those are lift-ready amendments to area docs/templates and remain subordinate to the foundation law. They do not supersede a foundation principle.

If implementation later tries to introduce a kernel-level `Outcome`, `Score`, `Winner`, tiebreak DSL, public/private visibility semantic change, or replay/hash semantic change, that ticket must stop and draft an ADR before proceeding.

---

## 14. Acceptance criteria

The spec is complete when all of the following are true:

1. `specs/README.md` lists `victory-explanation-shared-surface` as a non-gate, cross-game UI-infrastructure spec scheduled before resuming the gate ladder.
2. `docs/UI-INTERACTION.md` contains the outcome/victory-display public-UI target and acceptance-check lines from Appendix A, or an equivalent accepted amendment.
3. `docs/OFFICIAL-GAME-CONTRACT.md` contains the §5/§10 additions from Appendix B, or equivalent accepted amendments.
4. `templates/GAME-UI.md`, `templates/GAME-RULES.md`, and `templates/GAME-HOW-TO-PLAY.md` contain the additions from Appendix C, or equivalent accepted amendments.
5. Every current catalog game projects a Rust-owned viewer-safe terminal rationale.
6. `poker_lite` exposes showdown strength comparison for showdown/split and explicitly preserves no-reveal behavior for yield.
7. `OutcomeExplanationPanel` renders all nine games.
8. No game board has a divergent, more authoritative ad-hoc terminal explanation.
9. CI fails when a catalog game lacks outcome rationale docs, types, templates, or smoke coverage.
10. Browser smoke covers all nine games and includes no-leak assertions.
11. Hidden-info tests pass for `high_card_duel`, `secret_draft`, and `poker_lite`.
12. Reduced-motion and screen-reader outcome summary checks pass.
13. No TypeScript code computes a winner, score comparison, card strength, line, or tiebreaker decisive cause.
14. No engine-core/game-stdlib generic outcome noun is introduced.
15. Golden traces and replay/serialization snapshots are intentionally updated where view/effect schemas grow.

---

## 15. Ticket decomposition

Use an eight-ticket series echoing the completed rules-display shared-surface pattern. Suggested prefix: `VICEXPSHASUR`.

### `VICEXPSHASUR-001` — Contract, doctrine, template, and spec-index amendments

- Update `specs/README.md` to list this non-gate UI-infrastructure spec before the next gate work.
- Apply the `docs/UI-INTERACTION.md` appendix text.
- Apply the `docs/OFFICIAL-GAME-CONTRACT.md` appendix text.
- Apply the template amendments.
- Do not change gameplay behavior.

### `VICEXPSHASUR-002` — Outcome coverage checker and static template guard

- Add `scripts/check-outcome-explanations.mjs`.
- Wire it into the appropriate CI/hygiene lane.
- Add typed static template coverage checks.
- Fail new catalog games that lack an outcome rationale contract.
- Forbid YAML/DSL/behavior-looking template content.

### `VICEXPSHASUR-003` — `poker_lite` pilot rationale and no-leak proof

- Add Rust rationale for yield, showdown win, and split.
- Surface `ShowdownStrength` comparison at showdown.
- Preserve yielded-hand no-reveal behavior.
- Add public observer and seat-view no-leak tests.
- Update golden traces intentionally.

### `VICEXPSHASUR-004` — Remaining Rust rationale retrofits

- Retrofit `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`, and `secret_draft`.
- Add per-game rule IDs and docs updates.
- Add unit/visibility/serialization/replay coverage.
- Keep all rationale data game-local.

### `VICEXPSHASUR-005` — Shared `OutcomeExplanationPanel` and accessible presentation

- Add the shared React component.
- Add disclosure behavior, status-message summary, reduced-motion support, color-independent encoding, and responsive layout.
- Add static explanation template rendering.
- Do not include outcome decision logic in TypeScript.

### `VICEXPSHASUR-006` — Web type wiring and board integration

- Update `apps/web/src/wasm/client.ts` for all rationale payloads.
- Wire every board component to the shared panel.
- Remove or demote divergent ad-hoc terminal panels.
- Keep board-native visual highlights as supporting visualization.

### `VICEXPSHASUR-007` — Browser smoke, no-leak, accessibility, and replay checks

- Add `apps/web/e2e/outcome-explanation.smoke.mjs`.
- Register the smoke file in the `NON_GAME_SMOKE` set of `scripts/check-catalog-docs.mjs`, the `smoke:e2e` bullet of `apps/web/README.md`, and the `smoke:e2e` script of `apps/web/package.json` (see §12.3).
- Cover all nine games.
- Add dedicated `poker_lite` yield/showdown/split no-leak cases.
- Assert status region, disclosure keyboard operation, reduced motion, and no hidden data in DOM/storage/logs/dev panel/replay export.

### `VICEXPSHASUR-008` — Closeout, documentation alignment, and archive readiness

- Confirm all acceptance criteria.
- Update release/public checklists if needed.
- Archive tickets/spec according to repository workflow when complete.
- Record any deferred mechanic-atlas pressure explicitly instead of smuggling shared scoring primitives into kernel code.

### Forbidden changes for every ticket

No ticket may:

- clone or branch-fetch repository state as evidence for this spec;
- introduce TypeScript outcome/scoring/tiebreak logic;
- introduce generic outcome/winner/score nouns into `engine-core`;
- add YAML or a DSL;
- leak hidden information through any browser channel;
- weaken tests to pass;
- remove no-leak assertions;
- replace the shared surface with nine bespoke panels; or
- add coaching/counterfactual advice.

---

## 16. Appendix A — Lift-ready `docs/UI-INTERACTION.md` amendment

### A.1 Add this section as a new numbered section in `docs/UI-INTERACTION.md`

> Insertion anchor: `docs/UI-INTERACTION.md` has no standalone "shared rules / How-to-Play surface" section — the rules surface is described in prose inside §16 "Accessibility baseline". Add the section below as a new section after §15 "Bot explanation UI" (renumbering the subsequent §16 "Accessibility baseline", §17 "Responsive behavior", and §18 "UI acceptance check" accordingly), or in an equivalent placement agreed at implementation. The §18 "UI acceptance check" section named in A.2 is the correct target for those acceptance lines.

```markdown
## Outcome / victory explanation surface

Every official web-exposed game MUST render a shared outcome explanation surface when a match becomes terminal.

The outcome surface answers “why did this result happen?” in player-facing terms. It is mandatory for every catalog game, including tiny games. Small games may have small explanations, but they may not omit the surface.

The surface MUST show:

1. the final result: winner, draw, split, or game-specific terminal result;
2. the decisive cause of that actual result, such as a completed line, exact target reached, terminal no-move/no-piece reason, final score comparison, showdown strength comparison, or terminal tiebreaker rung;
3. a viewer-safe final standing for every player;
4. a viewer-safe per-player breakdown sufficient to understand the result; and
5. stable rule references back to the game’s scoring and terminal rules.

The surface MUST be driven by Rust-owned public/terminal view data and/or Rust-owned terminal semantic effects. TypeScript may render, lay out, interpolate safe template parameters, and manage disclosure/focus state. TypeScript MUST NOT compute the winner, score comparison, showdown strength, winning line, terminal tiebreaker, or decisive cause.

The surface MUST explain only the actual result. It MUST NOT provide coaching, counterfactual “what would have changed it” advice, hidden turning-point analysis, or AI strategy commentary.

Hidden-information games MUST use the same viewer-safe projection discipline as the rest of the UI. The outcome surface MUST NOT expose hidden information in visible text, hidden DOM text, accessibility labels, `data-testid`s, CSS classes, storage, logs, effect logs, replay exports, dev panels, or bot explanation surfaces. If a result resolves without reveal, the explanation must say so without revealing or implying unrevealed private facts.

The surface MUST be accessible:

- the terminal summary is programmatically exposed as a status/result message;
- the decisive cause is available as text and not only by color, animation, icon, or board highlight;
- player identity and standing are color-independent;
- expandable breakdowns use accessible disclosure controls;
- score/tiebreak tables or lists have readable labels; and
- reduced-motion mode preserves the same information without relying on animation.
```

### A.2 Add these lines to the UI acceptance check section

```markdown
- Terminal matches render the shared outcome explanation surface.
- The surface shows the final result, decisive cause, every player’s final standing, and a viewer-safe per-player breakdown.
- The decisive cause is supplied by Rust public/terminal view data and/or Rust terminal semantic effects; TypeScript does not compute the result explanation.
- The surface explains only the actual result and contains no coaching, counterfactuals, or strategy advice.
- Hidden-information games prove that outcome explanations do not leak unrevealed private state through text, DOM attributes, accessibility labels, test IDs, storage, logs, effect logs, replay export, dev panels, or bot explanation surfaces.
- The terminal summary is accessible to screen readers as a status/result message; expanded breakdowns are keyboard-accessible and color-independent; reduced-motion mode preserves all outcome facts.
```

---

## 17. Appendix B — Lift-ready `docs/OFFICIAL-GAME-CONTRACT.md` amendment

### B.1 Add to §5 documentation obligations

```markdown
### Outcome explanation documentation

Every official game MUST document how terminal results are explained to players.

`RULES.md` MUST identify stable scoring and terminal-condition rule IDs that the outcome explanation can cite. Scoring, victory, draw, split, yield, showdown, and tiebreaker rules MUST be explicit enough that a terminal rationale can trace back to them.

`HOW-TO-PLAY.md` MUST teach the same scoring and winning facts in player-facing language, especially the decisive factors that can appear in the end-of-match outcome surface.

`UI.md` MUST include an “Outcome / victory explanation” section naming:

- each terminal result variant;
- the Rust public/terminal view field or terminal effect that carries the decisive cause;
- the per-player breakdown fields shown to the viewer;
- hidden-information redaction rules, including any no-reveal terminal outcomes;
- the static explanation template keys, if used; and
- the smoke/no-leak coverage that proves the surface is safe.

Documentation may be concise for simple games, but omission is not allowed.
```

### B.2 Add to §10 web-exposure requirements

```markdown
### Outcome explanation requirement

A game is not web-exposed official unless every terminal outcome renders through the shared outcome explanation surface.

The game’s Rust implementation MUST project a viewer-safe terminal rationale through its public/terminal view and/or terminal semantic effect. The rationale MUST include the decisive cause of the actual result and a viewer-safe per-player final breakdown.

The web UI MUST render that Rust-owned rationale without computing outcome logic in TypeScript. TypeScript may format and display safe values, but it MUST NOT decide the winner, score comparison, tiebreaker rung, card/showdown strength, winning line, terminal reason, or any other behavior fact.

Hidden-information games MUST prove that outcome explanations leak no unrevealed private data through public view, private view, effect log, DOM, accessibility attributes, test IDs, logs, storage, replay export, bot explanations, or dev tools.

The official-game admission and public-release checks MUST fail when a catalog game lacks outcome-rationale docs, Rust view/effect payloads, shared-surface wiring, or smoke/no-leak coverage.
```

---

## 18. Appendix C — Lift-ready template amendments

### C.1 Add to `templates/GAME-UI.md`

```markdown
## Outcome / victory explanation

Describe the end-of-match explanation shown by the shared web outcome surface.

This section is mandatory for every web-exposed official game. Tiny games may provide a tiny explanation, but they may not omit the section.

### Terminal result variants

List every terminal result variant the game can produce.

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `<win/draw/split/yield/etc.>` | `<TerminalView field / PublicView field / terminal effect>` | `<one sentence>` | `<R-END-*/R-SCORE-*>` |

### Decisive cause payload

Name the Rust-owned public/terminal view fields and/or terminal semantic effects that carry the decisive cause.

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `<line completed / score comparison / tiebreaker / showdown strength / no legal move / exact target>` | `<field names>` | `<game_id.template_key>` | `<viewer-safe notes>` |

TypeScript MUST NOT compute these cause variants. It renders the Rust-projected value only.

### Per-player final breakdown

List every value shown for every player at terminal.

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| `<score/line/strength/piece count/allocation/etc.>` | `<Rust field/effect>` | `<yes/no>` | `<yes/no>` | `<redaction/reveal rule>` |

### No-leak rules

State what the outcome surface must never reveal.

- Visible text:
- Hidden DOM/accessibility attributes:
- `data-testid`/selectors:
- Storage/logs/dev panel:
- Effect log/replay export:
- Bot explanations/candidate rankings:

For hidden-information games, explicitly cover no-reveal terminal outcomes. Example: a yielded private card/crest remains hidden and the outcome surface says the result resolved without private reveal.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include coaching, counterfactuals, “what would have changed it,” turning-point analysis, or strategy advice.

### Accessibility and reduced motion

Confirm:

- terminal summary is exposed as a status/result message;
- decisive cause is text, not color-only or animation-only;
- player standing is color-independent;
- expanded breakdown is keyboard accessible;
- reduced-motion mode preserves all facts; and
- replay terminal renders the same outcome content for the same viewer.

### Smoke and tests

List the terminal smoke/no-leak cases required for this game.

| Test case | Terminal path | Required assertion |
|---|---|---|
| `<case>` | `<fixture/trace/scripted path>` | `<summary, breakdown, no-leak assertion>` |
```

### C.2 Add to `templates/GAME-RULES.md`

```markdown
## Outcome explanation traceability

Every scoring and terminal rule that can decide a match MUST have a stable rule ID and enough detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| `<score component / tiebreaker / line / showdown strength / terminal reason>` | `<R-SCORE-*/R-END-*>` | `<condition decided by Rust>` | `<player-facing wording constraint>` |

This table is traceability only. It is not a behavior DSL, selector table, or TypeScript decision source. Rust remains the source of scoring, terminal detection, and rationale projection.
```

### C.3 Expand the existing "Scoring and winning" section in `templates/GAME-HOW-TO-PLAY.md`

> `templates/GAME-HOW-TO-PLAY.md` already carries a `## Scoring and winning` section (authored under the `rules-display-shared-surface` work). This appendix expands that existing section with the alignment and content requirements below; it does not add a new section.

```markdown
## Scoring and winning

Explain how the match ends and why a player wins, loses, draws, or splits.

This section must align with the formal scoring and terminal rule IDs in `RULES.md` and with the `Outcome / victory explanation` section in `UI.md`.

Include, in player-facing language:

- the normal way the game ends;
- every score or standing component that can decide the result;
- every tiebreaker in the order Rust applies it;
- every draw/split condition;
- every no-reveal terminal condition, if hidden information is involved; and
- the kind of explanation the player will see at the end of the match.

Do not include strategy advice, optimal-play coaching, counterfactual examples, or hidden information that would not be visible to the relevant viewer.
```

---

## 19. Appendix D — No ADR appendix required

No drafted ADR is included because §13 determines that this spec does not trip a `FOUNDATIONS.md` §13 ADR trigger.

This appendix is intentionally present so ticket authors do not mistake the absence of an ADR draft for an omission. If later implementation changes the design by introducing engine-core outcome vocabulary, shared scoring primitives, a DSL/YAML behavior layer, visibility semantic changes, or replay/hash semantic changes, the work must stop and a new ADR must be drafted from `docs/adr/ADR-TEMPLATE.md` before proceeding.

---

## Closeout

Completed: 2026-06-09

Implementation summary:

- Added the permanent official-game outcome explanation contract across UI doctrine, official-game docs, game templates, and the spec index.
- Added the fail-closed catalog guard and static presentation-template coverage check for all nine web-exposed games.
- Added Rust-owned, viewer-safe terminal rationale projections for `poker_lite`, `three_marks`, `column_four`, `race_to_n`, `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`, and `secret_draft`.
- Added the shared `OutcomeExplanationPanel`, static keyed templates, WASM client types, and board integrations for all nine games.
- Added and registered the browser outcome-explanation smoke, covering terminal rendering, disclosure interaction, reduced-motion stability, and no hidden-information leaks.
- Closed the capstone by flipping `specs/README.md` to `Done` and repairing stale support-test expectations so existing replay/WASM fixture guards match the rationale-hash migrations.

Acceptance evidence:

- `cargo test --workspace` passed.
- `cargo run -p replay-check -- --game <game> --all` passed for `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`, `secret_draft`, and `poker_lite`.
- `node scripts/check-outcome-explanations.mjs` passed.
- `node scripts/check-catalog-docs.mjs` passed.
- `cargo fmt --all --check` passed.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `node scripts/check-doc-links.mjs` passed after the `Done` index flip.

---

## 20. Source notes

### 20.1 Repository source notes

All concrete references in this spec — the nine games' `src/rules.rs`, `src/visibility.rs`, `src/effects.rs`, and `docs/{UI,RULES,HOW-TO-PLAY}.md`; `apps/web/src/wasm/client.ts`, `apps/web/src/components/*Board.tsx`, `apps/web/src/components/RulesPanel.tsx`, `apps/web/e2e/rules-display.smoke.mjs`; `crates/engine-core/src/*`, `crates/game-stdlib/src/*`; the area docs and templates named in the Authority order; and the `archive/specs/rules-display-shared-surface.md` precedent — were validated against the current working tree during reassessment. See §21 Assumptions for the load-bearing validated premises.

### 20.2 External source notes

[^nng-progressive]: Nielsen Norman Group, “Progressive Disclosure,” <https://www.nngroup.com/articles/progressive-disclosure/>.
[^wcag-status]: W3C WAI, “Understanding Success Criterion 4.1.3: Status Messages,” <https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html>.
[^wcag-color]: W3C WAI, “Understanding Success Criterion 1.4.1: Use of Color,” <https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html>.
[^wcag-animation]: W3C, “Web Content Accessibility Guidelines (WCAG) 2.2,” Success Criterion 2.3.3 Animation from Interactions, <https://www.w3.org/TR/WCAG22/>.
[^wai-disclosure]: W3C WAI-ARIA Authoring Practices Guide, “Disclosure (Show/Hide) Pattern,” <https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/>.

---

## 21. Assumptions

These premises were validated against the current working tree during reassessment. A one-line correction here is cheaper than discovering the drift at task time.

- **The `rules-display-shared-surface` precedent is complete and archived.** It lives at `archive/specs/rules-display-shared-surface.md` (Status `Done` in `specs/README.md`) and shipped a 6-ticket series (`RULSUR-001`–`006`); this spec's 8-ticket `VICEXPSHASUR` series intentionally expands on it for the heavier 9-game Rust-rationale retrofit.
- **Every per-game Rust claim in §9 is present in current code.** Verified: `poker_lite` `ShowdownStrength { pair_flag, private_rank_value }`, `showdown_strength`/`compare_showdown`, `ShowdownView`/`TerminalView`, and yield-no-reveal (`games/poker_lite/src/{rules,visibility}.rs`); `draughts_lite` `TerminalWinReason { OpponentNoPieces, OpponentNoLegalMove }` (effects) absent from `TerminalView`; `directional_flip` `TerminalView` carrying `final_score`; `three_marks`/`column_four` `Win { winning_seat, line }` (field is `winning_seat`, not `seat`); `secret_draft` `TieBreakSummary`; and `token_bazaar` rule IDs `TB-SCORE-004`/`005`, `TB-END-001`/`002`/`003`.
- **The official web catalog is enumerated mechanically from `crates/wasm-api/src/lib.rs`** via the `const GAME_<NAME>: &str = "<id>";` pattern (`CATALOG_RE` in `scripts/check-player-rules.mjs`); `scripts/check-outcome-explanations.mjs` reuses that source.
- **`engine-core` and `game-stdlib` are clean for D2.** `crates/engine-core/src/*` is noun-free and exposes no `Outcome`/`Score`/`Winner`; `crates/game-stdlib/src/*` holds only `board_space` (no `Scoring` helper).
- **The doc/template amendment targets exist with the structure the appendices assume**, except the `docs/UI-INTERACTION.md` insertion anchor (see §A.1 note) and the already-present `## Scoring and winning` section in `templates/GAME-HOW-TO-PLAY.md` (see §C.3 note).
- **`docs/ROADMAP.md` (lines 57–59) admits cross-game UI-infrastructure specs outside the mechanic-gate ladder**, so this non-gate spec is index-admissible alongside the `rules-display-shared-surface` precedent. The `specs/README.md` index row for this spec must be added by `VICEXPSHASUR-001`.
