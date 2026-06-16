# Research Brief — River Ledger: showcase UX/presentation excellence pass (with focused correctness re-audit)

_Paste this entire document into a fresh ChatGPT-Pro deep-research session and upload the manifest
named in §1. You (Session 2) have none of the authoring session's context; everything you need is in
this brief plus the uploaded manifest. This is a **locked / no-questions** brief: produce the
deliverable directly — do not interview, do not ask clarifying questions._

---

## 1. Context

The uploaded manifest — **`manifest_2026-06-16_f1d6d39.txt`** — is the exact path inventory of the
`joeloverbeck/rulepath` repository: a Rust-first, rule-enforcing, replayable, testable card/board-game
platform where **Rust owns all behavior and TypeScript/React present only**. The foundation docs are an
ordered, layered authority indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution) →
`ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier documents
govern later ones, and accepted ADRs supersede them only by explicitly naming the affected sections.

**Fetch every file from commit `f1d6d39` (`f1d6d392eecf28af5d8b9e70fa86cc7168bf671d`, branch `main`)** —
the manifest reflects exactly that tree. If any file you reference cites a different "commit of record,"
note the divergence and use `f1d6d39`. (In particular, the two prior River Ledger reports in `reports/`
were authored against commit `351dc1e`; that is *their* baseline, not yours — read them for their
findings, but verify current state against `f1d6d39`, because work has landed since.)

The subject is the game **River Ledger** (`games/river_ledger/`, game id `river_ledger`): a 3–6 seat,
fixed-limit, hidden-information community-card game in the Texas Hold 'Em rules family (roadmap Gate 15).
It is the platform's **most complex game and its intended star showcase** once the app goes public. The
owner has already run a correctness-and-presentation iteration pass and is **generally happy** with the
result; this brief commissions the **next** iteration: a deep, externally-researched push to make the
interface **as good as it can be** for public launch.

### Grounding facts established before this brief (treat as given; verify against `f1d6d39`, don't rediscover from scratch)

- **A prior research pass already landed.** An earlier brief
  (`reports/river-ledger-correctness-ux-research-brief.md`) produced a report
  (`reports/river-ledger-correctness-and-presentation-report.md`) whose recommendations have since been
  **implemented** through the `RIVLEDOUT-*` and `RIVLEDSHO-*` ticket series. As of `f1d6d39`, the build
  already has: Rust-authored showdown explanation fields (`headline`, `decisive_comparison`,
  `comparison_basis`, per-seat `hand_name`, `rank_explanation`, `comparison_note`, `best_five`,
  `best_five_accessibility_label`, `category_ladder_position`); a real card component
  (`apps/web/src/components/RiverLedgerCard.tsx`) with rank + suit glyph + suit word; a hand-ranking
  reference; a terminal teaching aid; action-metadata copy; seat turn affordances; and a presentation-copy
  audit. **This brief is a delta on that implemented state. Do NOT re-recommend already-shipped work as if
  it were missing — build on it, and only call out where it falls short of showcase quality.**
- **River Ledger is deliberately a fixed-limit, abstract-contribution Hold'em.** All-in handling, side
  pots, no-limit, and pot-limit are **explicitly out of scope by design** and deferred to Gate 15.1 (see
  `RULES.md` `RL-OOS-ALLIN-001`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, `RL-BET-AMB-001`,
  `RL-OOS-NOLIMIT-001`). **Do not flag the absence of side pots / all-in / no-limit / stacks / rake as a
  bug or as a redesign gap.** The correctness oracle is `games/river_ledger/docs/RULES.md` and its stable
  `RL-*` rule IDs — **not** casino No-Limit Hold'em.
- **The prior correctness audit found River Ledger substantially correct** against its `RL-*` contract
  (no evaluator/showdown/pot/betting defect; the only material defect was the then-illegible showdown
  presentation, now addressed). Your correctness work (§4, Part A) is therefore a **fresh focused
  re-audit / regression check**, not a from-scratch audit — confirm the verdict still holds at `f1d6d39`
  and surface anything the prior pass missed or any regression the implementation work introduced.

### Concrete current awkwardness (captured live from the running build before this brief)

Captured by driving `http://127.0.0.1:4173/` at commit `f1d6d39` (seed 79, 6-seat human-vs-bot,
checkdown to showdown). These are verified observations on the *current* build — the starting material to
improve. Verify each against the code/markup before designing a fix; treat them as the floor, not the
ceiling, of what to improve.

1. **Community-card placeholders clip their own label.** Unrevealed board slots render `HIDDEN` plus a
   word that overflows the card to `Pendin…` — a literal text-overflow defect in the placeholder card.
2. **Seat panels show cryptic raw counters.** Each seat shows `STREET <n>`, `TOTAL <n>`, and
   `PRIVATE 2` with no plain-language meaning (street contribution / total contribution / hole-card
   count). `PRIVATE 2` reads as a mysterious quantity rather than "2 hole cards."
3. **A redundant, unlabeled contribution bar chart** sits beside the seat panels, duplicating the same
   per-seat numbers in a second visual encoding with no axis/legend/title.
4. **Action buttons surface irrelevant metadata.** `Fold` and `Check` choices display
   `Call price / Adds / Cap left` lines that are meaningless for those actions (e.g. Fold showing
   "Call price 2 / Adds 0 / Cap left 3").
5. **Mixed seat indexing.** Seats are labeled `Seat 0 … Seat 5`, but the mode/status line says
   **"Player 1 to act."** The 0-indexed seat vocabulary and 1-indexed "Player N" status disagree.
6. **Raw identifier leak in outcome copy.** The outcome headline correctly reads "Seat 5 wins," but the
   immediately following sentence reads **"seat_5 wins with Pair of Jacks."** — the raw internal seat id
   reaches a public player-facing string. (Confirm whether this string is Rust-authored or
   TS-interpolated, and fix at the authoring layer. This is also a presentation-copy-guard concern.)
7. **The outcome panel is very long and weakly prioritized.** At a 6-seat showdown it stacks every seat's
   full five-card best-five vertically; the winner is only weakly distinguished from losers (a thin accent
   border), "loses to Pair of Jacks" repeats per seat, and the community cards are not visually tied to
   each seat's used five. Scanning "who won and why" requires a lot of scrolling.
8. **Layout underuses width.** A large empty region sits top-left of the table beside the vertical seat
   rail; horizontal space is not used for the board/table composition.
9. **The star game has no custom catalog icon.** `apps/web/src/components/GameCatalogIcon.tsx` maps
   game ids to original SVG icons but has **no `river_ledger` entry**, so River Ledger falls back to the
   generic `FallbackIcon` on the picker and setup hero. For the intended showcase game this is a visible
   gap.

(Note: the replay viewer *does* handle River Ledger — `ReplayViewer.tsx` has an `isRiverLedgerView`
branch — so do not report replay coverage as missing. Replay is out of scope per §3.6 regardless.)

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing anything.

**Foundation & area docs (authority flows downward):**

```
docs/README.md — the authority order and the layering rule that governs every recommendation.
docs/FOUNDATIONS.md — the constitution: priority order, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every finding and recommendation must satisfy these.
docs/ARCHITECTURE.md — the layer architecture and "Rust owns behavior, TS presents" division that bounds every UI recommendation.
docs/ENGINE-GAME-DATA-BOUNDARY.md — the engine-core (noun-free) / games / static-data boundary; constrains where any new field, helper, or shared presentation surface may live.
docs/UI-INTERACTION.md — the binding UI presentation law: TS renders Rust/WASM output only and decides no legality/evaluation; §2 visual direction (cozy premium board-game table), §10/10A/10B/10C shared scheduler / orchestration / multi-seat layout / showdown surface, §16 outcome surface, §17 accessibility, §19 acceptance checks. This is the central contract for the redesign.
docs/WASM-CLIENT-BOUNDARY.md — the Rust↔browser JSON bridge contract; governs how any new Rust-authored field reaches the client.
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md — per-viewer projection, active/pending/observer rules, and seat-rail obligations for 3–7 seats; governs the multi-seat table layout and what showdown data is shown to whom.
docs/OFFICIAL-GAME-CONTRACT.md — the official-game fidelity contract; the standard the correctness re-audit measures rule fidelity against.
docs/IP-POLICY.md — public/private posture and the no-casino-trade-dress rule; bounds the ambitious visual redesign away from copied card/table art, chips, and real-money framing.
docs/TESTING-REPLAY-BENCHMARKING.md — determinism, no-leak tests, replay/hash discipline; the methodology any correctness claim and any new field must respect.
docs/AI-BOTS.md — bot law: levels, hidden-information safety, the public "why?" explanation affordance and its viewer-safe constraints; the contract for the in-scope bot-explanation surface.
docs/ROADMAP.md — Gate 15 / Gate 15.1 framing that fixes what is in scope vs deferred (all-in, side pots).
docs/AGENT-DISCIPLINE.md — coding-agent law (bounded tasks, forbidden changes, failing-test protocol); your recommendations must be convertible into bounded tickets that obey it.
```

**River Ledger game docs:**

```
games/river_ledger/docs/RULES.md — THE correctness oracle: every RL-* rule ID the re-audit verifies against, plus documented ambiguities and intended deviations.
games/river_ledger/docs/MECHANICS.md — the mechanic inventory backing the rule IDs; maps rules to modules.
games/river_ledger/docs/UI.md — the current UI contract: board layout, viewer matrix, the showdown explanation fields already projected (headline, decisive_comparison, hand_name, rank_explanation, best_five, category_ladder_position, …), accessibility/no-leak rules, and the test matrix. This documents the implemented baseline you are improving.
games/river_ledger/docs/HOW-TO-PLAY.md — the player-facing rules and vocabulary; the baseline any new explanatory copy must stay consistent with.
games/river_ledger/docs/RULE-COVERAGE.md — RL-* → module/test/trace mapping; your fastest index from a rule to the code and the test that proves it.
games/river_ledger/docs/COMPETENT-PLAYER.md — strategy / hand-strength notes; context for the teaching layer and any strength affordance.
games/river_ledger/docs/AI.md — River Ledger bot design and explanation vocabulary; the source of truth for the in-scope bot "why?" surface.
games/river_ledger/docs/SOURCES.md — source notes for the rules; check before citing external Hold'em authorities.
games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md — the presentation/readiness gates a showcase UX change must satisfy and must not regress.
```

**Prior artifacts (read for delta framing — what is already done vs. still open):**

```
reports/river-ledger-correctness-and-presentation-report.md — the prior report whose recommendations are now implemented; your baseline for "already shipped, do not re-recommend." Note it was authored at commit 351dc1e.
reports/river-ledger-correctness-ux-research-brief.md — the prior brief; shows the previously settled scope so you extend rather than repeat it.
specs/README.md — the living spec index and progress tracker; situates Gate 15 and the shared-surface specs.
```

**Shared-surface specs (the cross-game lens — required for §3.5 / Part C):**

```
archive/specs/card-and-action-presentation-shared-surfaces.md — the shared card/action presentation contract; governs how card and action surfaces are shared across games and when a per-game shape may diverge.
archive/specs/action-consequence-and-match-context-shared-surfaces.md — shared action-consequence / match-context surfaces (the seat frame, action metadata, match context); governs the seat-panel and action-panel changes.
archive/specs/effect-animation-and-turn-orchestration.md — the shared animation scheduler and turn-orchestration contract; governs any motion/reveal/pacing recommendation and the board reveal animation.
archive/specs/catalog-setup-visual-redesign.md — the catalog/setup visual system and design-token direction; governs the catalog-icon gap and any token-level visual recommendation.
archive/specs/gate-15-river-ledger-texas-holdem-base.md — the original Gate 15 spec; the authoritative statement of River Ledger's intended scope and abstractions.
```

**Code seams to inspect directly** (read these *in the repo*; they are **not** pasted and are
*inspect, not read-fully*):

- `games/river_ledger/src/evaluator.rs` — hand categories, tie-break vectors, ace-low straight,
  best-5-of-7 enumeration; the core of the correctness re-audit.
- `games/river_ledger/src/showdown.rs` — showdown resolution, winner/split determination, and the
  human-readable explanation builder (hand names, decisive comparison, comparison notes).
- `games/river_ledger/src/pot.rs` — single-pot allocation and stable button-order split-remainder logic.
- `games/river_ledger/src/betting.rs` — betting-round closure, call price, raise-cap accounting; relevant
  to the action-metadata copy fix (#4 above).
- `games/river_ledger/src/rules.rs` — action application and street transitions
  (preflop→flop→turn→river→showdown).
- `games/river_ledger/src/actions.rs` — legal-action tree, command validation, raise-cap enforcement, and
  the per-choice metadata (`call price`, `adds`, `cap left`) the action panel renders — where awkwardness
  #4 originates.
- `games/river_ledger/src/state.rs` — `ShowdownReveal`, `TerminalOutcome`, `Street`, `SeatStatus`,
  contribution ledger types.
- `games/river_ledger/src/visibility.rs` — view projection: `PublicView`, outcome rationale and per-seat
  breakdown views, showdown strength view; the seam where any new Rust-authored display field must be
  authored to satisfy the UI law. Check whether the `seat_N` raw-id string (#6) originates here.
- `games/river_ledger/src/ui.rs` — UI payload assembly and the per-game catalog identity (icon id / theme
  key / accent / accessibility label) referenced by `UI-INTERACTION.md` §10A; relevant to the catalog-icon
  gap (#9).
- `games/river_ledger/src/cards.rs` — `Rank`/`Suit`/`Card` and the rank→value (2–14) mapping.
- `games/river_ledger/src/setup.rs` — seat/blind/button setup and deal.
- `apps/web/src/components/RiverLedgerBoard.tsx` — the River Ledger table: seat rail, board slots
  (placeholder overflow #1), seat panels (#2/#3), private view, action panel mount, outcome panel mount,
  and overall layout (#5/#8).
- `apps/web/src/components/RiverLedgerCard.tsx` — the River-Ledger-only card renderer (rank/suit/tone);
  consumed by both the board and the shared outcome panel's showdown best-five.
- `apps/web/src/components/OutcomeExplanationPanel.tsx` + `outcomeExplanationTemplates.ts` — the
  **shared** outcome surface (used by ~15 games) plus its per-game templates and the River Ledger showdown
  subcomponent; the centerpiece redesign target and the #6/#7 source.
- `apps/web/src/components/SeatFrame.tsx` — the shared seat-rail/viewer-mode surface; relevant to seat
  presentation and the multi-seat layout.
- `apps/web/src/components/ActionControls.tsx` — the shared basic action surface and its cost metadata
  rendering (#4).
- `apps/web/src/components/RulesPanel.tsx`, `EffectLog.tsx`, `ModeControls.tsx`, `MatchSetup.tsx`,
  `GamePicker.tsx`, `AppShell.tsx`, `GameCatalogIcon.tsx`, `DeckFlowPanel.tsx` — shared shell surfaces;
  inspect those a recommendation touches to ground the Part C cross-game inventory.
- `apps/web/src/styles.css` — the global design-token system (typography, spacing, colors incl.
  parchment/felt/brass, radii, shadows, per-game accent tokens); the substrate for any cohesive visual
  recommendation.
- `apps/web/src/wasm/client.ts` — the TS view type definitions; shows what the bridge currently exposes vs.
  what a redesign needs.
- `crates/wasm-api/src/lib.rs` — the JSON bridge; the single seam any new Rust-authored field must pass
  through to reach the browser.
- `games/river_ledger/tests/{rules,property,visibility,serialization,replay,bots}.rs` and
  `games/river_ledger/tests/golden_traces/*.trace.json` — the correctness evidence and the oracle for what
  is already proven; especially the showdown/tie-break/split/foldout/no-leak traces.

---

## 3. Settled intentions (final — these pre-empt every clarifying question)

These decisions are locked. Do not re-open them.

1. **Deliverable = one research report with prioritized, spec-ready recommendations.** Not a formal spec,
   not tickets, not `RL-*` rule IDs. You research, re-audit, and recommend; the owner converts your report
   into a spec and then tickets through the repo's own pipeline in a later session. Structure the report so
   conversion is easy (clear, independently-actionable, prioritized items with boundary tags) but do **not**
   author rule IDs or ticket files.

2. **Primary goal = an ambitious, cohesive UX/presentation redesign of the River Ledger *play* experience.**
   Go beyond incremental fixes: propose a coherent visual + interaction redesign of the table and the
   showdown surface toward `UI-INTERACTION.md` §2's "cozy premium board-game table" target — warm, tactile,
   readable, original, restrained motion — grounded in researched poker-UI / HCI prior art and staying
   strictly inside the boundary law. The **showdown / outcome surface is the centerpiece** (highest
   priority); the rest of the in-play table is secondary but in scope. Incremental fixes to the awkwardness
   in §1 are the floor, not the deliverable.

3. **Frame everything as a delta on the already-implemented state.** The RIVLEDOUT/RIVLEDSHO work shipped
   the Rust-authored showdown explanation fields, the `RiverLedgerCard` component, the hand-ranking
   reference, the teaching aid, action-metadata copy, seat affordances, and a copy audit (see §1 grounding).
   Do **not** present these as missing. Where they fall short of showcase quality (e.g. the long, weakly
   prioritized outcome panel, the `seat_5` id leak, the action-metadata-on-Fold issue), improve them; where
   they are good, build on them.

4. **Secondary goal = a fresh, focused correctness re-audit.** Re-verify the evaluator (categories,
   kickers, ace-low straight, best-5-of-7), tie-break ordering, pot allocation + split remainder, betting /
   raise-cap / round closure, and showdown winner/split/foldout against the `RL-*` rules at `f1d6d39`. This
   is lighter than a cold audit because the prior pass found the game substantially correct — confirm that
   verdict still holds, flag any regression the implementation work introduced, and surface anything
   previously missed, each with a verdict (`correct` / `bug` / `ambiguous`), evidence (rule ID + module +
   test/trace), and severity. Explicitly reconfirm that the intended abstractions (no all-in / side pots /
   no-limit / stacks / rake) are **not** bugs.

5. **Cross-game shared-surface inventory is mandatory.** River Ledger is the subject, but its UI is built
   substantially on **shared** web surfaces. For every shared surface a recommendation touches, you must
   inventory how the **other catalog games** consume it and make per-game-aware recommendations, so a shared
   change is safe everywhere — and explicitly tag each recommendation as **River-Ledger-local** vs
   **shared-surface (affects other games)**. The shared surfaces include at least: `OutcomeExplanationPanel`
   + `outcomeExplanationTemplates.ts` (the outcome surface is rendered by ~15 games), `SeatFrame`,
   `ActionControls`, `RulesPanel`, `EffectLog`, `ModeControls`, `MatchSetup`, `GamePicker`, `AppShell`,
   `GameCatalogIcon`, `DeckFlowPanel`, and the `styles.css` design tokens. `RiverLedgerBoard.tsx` and
   `RiverLedgerCard.tsx` are River-Ledger-local (but `RiverLedgerCard` is also imported by the shared
   outcome panel for showdown best-five — note that coupling). Verify the consumer lists yourself against
   the tree; do not trust this brief's list as exhaustive.

6. **Scope boundary.** In scope: the **in-play table**, the **showdown / outcome surface** (centerpiece),
   the **bot "why?" / explanation affordance** during play, and **every shared surface touched by those**.
   Out of scope as redesign targets: the game-picker card, match setup, the How-to-Play/Rules surface, and
   the replay viewer / import-export. You may *mention* an out-of-scope surface only where a shared surface
   it reuses is being changed (e.g. a design-token or `SeatFrame` change that ripples into setup/replay),
   and the catalog-icon gap (#9) is in scope precisely because it is part of the table/showcase identity —
   but do not produce a redesign of picker/setup/rules/replay as targets in their own right.

7. **Every recommendation must honor these binding constraints** (not negotiable trade-offs — a
   recommendation that violates one is out of scope):
   - **TypeScript renders Rust-authored fields and does no winner/evaluator/legality logic**
     (`UI-INTERACTION.md`, `RL-UI-SHOWDOWN-001`). Any new human-readable hand description, category label,
     comparison narrative, bot reason, or display string is a **Rust-authored field** projected through
     `visibility.rs` → `crates/wasm-api` → the client — never synthesized or re-interpreted in TypeScript.
     Call this out explicitly wherever you recommend new copy, labels, or data. (The `seat_5` leak in §1 #6
     must be fixed at the authoring layer, and `Seat N` display naming must come from Rust/static metadata,
     not from interpolating a raw id.)
   - **No hidden-information leaks** (`RL-VIS-*`, `RL-UI-NOLEAK-001`): folded seats' hole cards, deck tail,
     future board, burn positions, and bot private reasoning must never reach any browser surface (DOM,
     a11y labels, `data-testid`, storage, logs, effect logs, replay exports, bot explanations). Any strength
     / teaching affordance derives only from authorized, revealed showdown data.
   - **No casino trade dress / real-money framing** (`docs/IP-POLICY.md`, `RL-UI-NOCASINO-001`): the
     ambitious visual redesign uses original, neutral card/table styling — no copied or evocative casino
     art, no green-felt-as-mimicry, no chip-stack/real-money framing, no branded card backs or trade-dress
     imitation. Abstract contribution units stay abstract.
   - **Determinism & engine boundaries**: `engine-core` stays noun-free (no `card`/`deck`/`hand`/`pot`/
     `bet`/`showdown` vocabulary); River Ledger nouns stay in `games/river_ledger`; no new DSL/YAML; replay,
     hashes, RNG, serialization order, and traces stay deterministic (or are explicitly migrated with
     matching coverage).
   - **Shared-surface discipline**: promotion of presentation helpers into `game-stdlib` is governed by the
     mechanic atlas and the shared-surface specs, not invented ad hoc; per-game divergence from a shared
     surface must be a recorded exception, per the shared-surface specs and `UI-INTERACTION.md` §10A/§19.
   - **Motion through the shared scheduler**: any reveal/animation/pacing recommendation routes through the
     shared effect-animation scheduler with reduced-motion support and settle-to-view, per
     `UI-INTERACTION.md` §10/§10A and `effect-animation-and-turn-orchestration.md` — no ad-hoc timers, no
     hidden-future-board preload into DOM/a11y/test-IDs.
   - **Never recommend deleting or weakening tests to get green** — correctness fixes and new display fields
     come with, or call for, strengthened coverage (`AGENT-DISCIPLINE.md` §4).

`assumption:` the exact visual language, concrete component mockups, and final copy wording are
**delegated to your design judgment** — recommend defaults with rationale (and, where the solution space is
wide, present the leading alternatives you considered and why you chose your default), rather than treating
these as open questions.

---

## 4. The task

This is an **audit + ambitious presentation-overhaul research report** ("other" target type: a focused
correctness re-audit fused with a showcase-grade UX/visual redesign). Achieve four things. **First**,
re-confirm River Ledger's *correctness* against its own `RL-*` rule contract at `f1d6d39` — evaluator,
tie-break ordering, best-5-of-7, pot/split allocation, betting/raise-cap/closure, and showdown
winner/split/foldout — stating a verdict per area with evidence and flagging any regression or
previously-missed issue. **Second**, design a researched, ambitious, cohesive redesign of the River Ledger
*play* experience toward the "cozy premium board-game table" target, with the **post-showdown "what each
hand scored and why" surface as the centerpiece**, building on the already-implemented explanation layer
and fixing the concrete awkwardness in §1 along the way. **Third**, for every **shared** web surface your
recommendations touch, inventory how the other catalog games use it and make per-game-aware, safe-everywhere
recommendations, tagging each item River-Ledger-local vs shared. **Fourth**, fold in the in-scope **bot
"why?" / explanation affordance** so a player can understand bot decisions during play, within the
viewer-safe bot-explanation law. Ground the design half in external research (real poker UIs, hand-strength
visualization, HCI/usability literature, accessibility) and both halves in the repo's own boundary law.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow `RULE-COVERAGE.md` from
any rule ID into its module and test; read whatever evaluator/showdown/pot/betting code and golden traces
you need to confirm a verdict; trace any display string from `visibility.rs` through
`crates/wasm-api/src/lib.rs` into `apps/web/`; and grep the other games' board components to verify which
shared surfaces they consume for the Part C inventory.

Research online as deeply as needed — and the owner explicitly wants this: study **similar implementations**
(open-source and commercial poker clients and hand evaluators; how established poker and card-game UIs
present the table, community cards, betting affordances, and especially the **showdown** — winning-hand
highlighting, hand naming, "why this beats that" comparison, and result legibility for novices),
**research papers and HCI/usability literature** (explaining outcomes of complex rule systems to novices;
progressive disclosure; recognition-over-recall; contrastive explanation; learnability; visual hierarchy
and information density), **web/visual design** for card legibility and a cozy-premium tabletop aesthetic
(typography, color/contrast, depth, restrained motion), and **accessibility** (colorblind-safe suit
encoding, screen-reader-friendly hand and table descriptions, WCAG contrast, reduced motion, keyboard
interaction). For the correctness half, cross-check the evaluator's category ordering, kicker rules,
ace-low straight handling, and split-remainder rule against authoritative Texas Hold 'Em hand-ranking
references. **Cite every external source that shapes a recommendation or a verdict.**

Worked example you should be able to explain end-to-end (verified live on the current build at `f1d6d39`):
a checkdown 6-seat showdown where the engine declares **Seat 5 wins with a Pair of Jacks, beating Seat 2's
Pair of Nines** ("Both hands are one pair, so the pair rank decides first: Jacks > Nines"), and renders
every revealed seat's named hand, comparison note, and best-five cards. Use this concrete state to
demonstrate how your redesigned outcome surface would make "who won, why, and how the rest stack up" legible
**at a glance** — fixing the current panel's length, weak winner emphasis, repetition, raw-id leak, and the
disconnect between community cards and each seat's used five.

---

## 6. Doctrine & constraints

Honor these throughout (they bound which recommendations are admissible):

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior recommendation must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence would require an accepted ADR
  superseding the affected principle first, never designing against it silently.
- **Authority order**: foundation docs govern area docs govern game docs/specs; if a recommendation conflicts
  with `FOUNDATIONS.md`, `ARCHITECTURE.md`, the boundary docs, or `UI-INTERACTION.md`, the recommendation is
  wrong.
- **TypeScript never decides legality or evaluation.** Legal actions, validation, views, winner, split, hand
  strength, and bot reasons all come from Rust/WASM; new human-readable text is Rust-authored
  (`RL-UI-SHOWDOWN-001`, `RL-OOS-BROWSER-001`, `docs/UI-INTERACTION.md`).
- `engine-core` stays generic and **noun-free**; River Ledger's nouns stay in `games/river_ledger`; shared
  presentation helpers are promoted only via the mechanic atlas / shared-surface specs, never ad hoc.
- **No YAML, no DSL** without an accepted ADR; static data is typed content/metadata only.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic (or are
  explicitly migrated with matching coverage updates).
- **No hidden-information leaks** into payloads, DOM, accessibility labels, test IDs, storage, logs, effect
  logs, bot explanations, or replay exports (`RL-VIS-*`, `RL-UI-NOLEAK-001`).
- **No casino trade dress / real-money framing** (`RL-UI-NOCASINO-001`, `docs/IP-POLICY.md`).
- **Shared surfaces are shared**: a change to a shared component, template map, or design token affects every
  game that consumes it; treat that ripple as a first-class constraint, per the shared-surface specs and
  `UI-INTERACTION.md` §10A/§19 (recorded exceptions for board-native divergence).
- **Motion is scheduler-owned and reduced-motion-safe**; no ad-hoc animation, no hidden-future-board preload.
- **In-scope vs deferred**: all-in, side pots, no-limit, pot-limit, tournaments, stacks, rake are **out of
  scope by design** (Gate 15.1+). Do not recommend them and do not treat their absence as a defect.
- **Never recommend deleting or weakening tests to get green** — correctness fixes and new display fields must
  come with, or call for, strengthened coverage (`AGENT-DISCIPLINE.md` §4).

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

**`river-ledger-showcase-ux-report.md`** (new file; the owner will place it under `reports/`). It must
contain, in order:

- **Part A — Focused correctness re-audit.** For each area (setup/blinds; fixed-limit betting & raise cap;
  round closure; pot allocation & split remainder; hand evaluation — categories, kickers, ace-low straight,
  best-5-of-7; tie-break ordering; showdown winner/split/foldout; visibility/no-leak at showdown), give a
  **verdict** (`correct` / `bug` / `ambiguous`), the **evidence** (cite the `RL-*` rule, the module, and the
  test/trace that does or does not prove it), and a **severity** for any defect or regression. State whether
  the prior audit's "substantially correct" verdict still holds at `f1d6d39`, and explicitly confirm the
  intended abstractions (no all-in/side pots/no-limit/stacks/rake) are **not** bugs. Include the §5 worked
  example as a verification of the tie-break encoding.

- **Part B — Showcase UX / presentation redesign** (whole play table, showdown first). Must include:
  - a **cohesive visual + interaction direction** for the River Ledger table toward the "cozy premium
    board-game table" target, grounded in cited poker-UI / HCI / visual-design research, expressed against
    the existing `styles.css` design-token system (what tokens/components to add or change, not pixel CSS);
  - a **concrete annotated redesign of the showdown / outcome surface** as ASCII/markdown mockup(s) that
    fixes the current panel's length, weak winner emphasis, per-seat repetition, raw-id leak, and the
    board-to-used-cards disconnect — making "who won and why, and how the field stacks up" legible at a
    glance, building on the already-shipped explanation fields;
  - targeted redesigns/fixes for the **in-play table**: the board/community-card slots (incl. the
    placeholder-overflow #1), seat panels (replace the cryptic `STREET/TOTAL/PRIVATE 2` counters and the
    redundant bar chart #2/#3), the action panel (the irrelevant-metadata-on-Fold/Check issue #4), the
    seat-indexing inconsistency (#5), and the table layout / wasted width (#8);
  - the **bot "why?" / explanation affordance** during play (in scope), within the viewer-safe bot-explanation
    law (`docs/AI-BOTS.md`, `games/river_ledger/docs/AI.md`);
  - the **catalog-icon gap** (#9): a recommendation for an original River Ledger catalog SVG icon + theme
    identity consistent with the catalog/setup visual system;
  - a **before/after copy table** (current strings → proposed player-facing copy), including any remaining
    engine-jargon and the `seat_5` raw-id fix;
  - for every recommendation that adds or changes player-facing text/data, the **Rust-authored field shape**
    (what `visibility.rs`/`ui.rs` should emit and how it flows through `crates/wasm-api` to the client) so the
    change honors the UI law — never a TS-side synthesis;
  - **accessibility** treatment throughout (colorblind-safe suit encoding, screen-reader hand/table
    descriptions, WCAG contrast, reduced-motion, keyboard), per `UI-INTERACTION.md` §17.

- **Part C — Shared-surface cross-game inventory & impact.** For each shared surface your recommendations
  touch (at minimum the outcome panel + templates, `SeatFrame`, `ActionControls`, `RulesPanel`, `EffectLog`,
  `ModeControls`, `GameCatalogIcon`, the design tokens, and the `RiverLedgerCard`↔outcome-panel coupling),
  state: which other catalog games consume it (verified against the tree), what changes safely for all of
  them, what must be a River-Ledger-local divergence (and whether that divergence needs a recorded exception
  per the shared-surface specs), and any per-game risk. Every Part B item must be tagged **River-Ledger-local**
  or **shared-surface** and cross-referenced here.

- **Part D — Prioritized recommendation backlog.** A ranked list of independently-actionable items, each
  tagged with the boundary constraint(s) it touches (Rust-authored vs TS-only; leak-safety; no-casino;
  determinism; shared vs local; scheduler/motion) and a rough effort/impact note — structured so the owner can
  convert items into a spec and bounded tickets. Group into slices (e.g. correctness fixes; showdown
  centerpiece; table polish; shared-surface changes; bot-why; catalog identity).

- **Sources.** All external research cited inline and collected at the end, each with an accessed date.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a requirement
> impossible, state it in the report and proceed with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- The fetch-baseline commit `f1d6d39` contains every file named in the §2 read-in-full list, and you read
  them in authority order; any "commit of record" cited in a prior report (e.g. `351dc1e`) is noted as that
  report's baseline, not yours.
- Part A's every correctness verdict cites a specific `RL-*` rule ID plus the module and the test/trace that
  bears on it; no verdict rests on casino-NLHE expectations rather than `RULES.md`; the report states whether
  the prior "substantially correct" verdict still holds and nowhere treats all-in / side pots / no-limit /
  stacks absence as a defect.
- The report does **not** re-recommend already-implemented work (Rust-authored showdown explanation fields,
  `RiverLedgerCard`, hand-ranking reference, teaching aid, action-metadata copy, seat affordances, copy
  audit) as if missing; it builds on them and improves them where they fall short.
- Every Part B recommendation that adds or changes player-facing text/labels/data routes it through a
  **Rust-authored field** (not TS evaluator/winner/legality logic), and says so explicitly; the `seat_5`
  raw-id leak is fixed at the authoring layer and display naming comes from Rust/static metadata.
- No recommendation introduces a hidden-information leak, casino trade dress, a new DSL/YAML, an ad-hoc
  (non-scheduler) animation, or a determinism break.
- Every Part B item is tagged River-Ledger-local vs shared-surface and is reconciled in Part C; the Part C
  consumer lists are verified against the tree, not assumed.
- The in-scope surfaces (in-play table, showdown centerpiece, bot "why?", touched shared surfaces, catalog
  icon) are covered; the out-of-scope surfaces (picker, setup, rules, replay) are not redesigned as targets.
- The redesign makes the §5 worked example ("Seat 5's Pair of Jacks beats Seat 2's Pair of Nines, and here's
  how the field ranks") legible to a novice at a glance, and any strength/teaching affordance is explicitly
  marked a learning aid and is leak-safe.
- The deliverable set matches §7 exactly: one report, Parts A/B/C/D plus Sources, with the cohesive visual
  direction, the concrete showdown mockup, the before/after copy table, and the Rust-authored field shapes
  present.
- Every external claim that shapes a verdict or recommendation is cited with an accessed date.
