# Research brief — Victory-explanation shared surface (cross-game outcome display)

> **You are ChatGPT-Pro Session 2, the deep researcher and author.** This prompt is
> self-contained: you have only this prompt plus one uploaded file (the manifest named in
> §1). The requirements below are **final** — produce the deliverable directly. Do **not**
> interview, do **not** ask clarifying questions. The interview already happened. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded manifest `reports/manifest_2026-06-09_e0e7735.txt` is the path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs (`OFFICIAL-GAME-CONTRACT.md`, `MECHANIC-ATLAS.md`, `AI-BOTS.md`,
`UI-INTERACTION.md`, `TESTING-REPLAY-BENCHMARKING.md`) → `ROADMAP.md` → `IP-POLICY.md` →
`AGENT-DISCIPLINE.md` → `WASM-CLIENT-BOUNDARY.md`. Earlier documents govern later ones, and
accepted ADRs supersede them only by explicitly naming the affected sections.

**Fetch every file from commit `e0e7735` (the repository HEAD; the working tree is clean) —
the uploaded manifest reflects exactly that commit's tree.** If any file you read cites a
different "commit of record," note the divergence and use `e0e7735`.

### The problem that motivates this work

The human owner played `poker_lite` (display name "Crest Ledger"), **won, and could not tell
why he won** — which combination of victory logic decided the match. This is not a one-off:
the roadmap marches toward genuinely complex scoring (trick-taking, then full poker hand
ranking — which the owner finds hard to follow even in real life), and the current UI gives no
principled way to explain an outcome. The repository must establish — *now, before that
complexity lands* — a disciplined, enforced way for the web UI to show **why a result
happened**, across every game, so a player finishes a match thinking *"I fully understand why I
won/lost. I want to play again."*

Grounding facts (verified in-repo at `e0e7735`; confirm and deepen them yourself):

- There is **no shared outcome/victory surface**. Each game's React board shows a minimal
  "turn pill" and, at most, an ad-hoc terminal panel. There is no `OutcomePanel`/`ResultPanel`
  shared across games.
- Public views mostly expose **who won, not why.** `directional_flip` is the lone good case —
  its `TerminalView` carries `final_score`. `column_four`/`three_marks` expose the winning
  `line`. Most others (`draughts_lite`, `high_card_duel`, `secret_draft`, `token_bazaar`)
  expose only the winning seat in the terminal view; score breakdowns and tiebreaker reasons
  are computed in Rust but not projected.
- `poker_lite` is the worked failing case: `showdown_strength`/`compare_showdown` in
  `games/poker_lite/src/rules.rs` compute a `ShowdownStrength { pair_flag, private_rank_value }`
  comparison, but **that comparison never crosses into the public view** — `ShowdownView`
  exposes the revealed cards and the winning seat, not the strength reasoning. That is exactly
  why the win was inscrutable.
- The just-completed **rules-display-shared-surface** work (archived spec + `RULDISSHASUR-*`
  tickets + a locked research brief) is the structural precedent: a catalog-complete, shared,
  static-fed, accessible cross-game surface with a per-game content contract and a CI staleness
  guard. **This new spec is its sibling — the same shape, applied to outcome explanation.**

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing. Each is load-bearing for this target.

**Foundation law (governs everything):**

- `docs/README.md` — the authority order and the layering rule; confirms specs conform to and
  never override foundation law.
- `docs/FOUNDATIONS.md` — the constitution: product priority, behavior authority, **§11
  universal acceptance invariants, §12 stop conditions, §13 ADR triggers**. Every decision in
  the deliverable must satisfy §11 and clear §12; any genuine divergence needs an accepted ADR.
- `docs/ARCHITECTURE.md` — the action / public-view / semantic-effect / replay / determinism
  model. The victory rationale must fit this model (it is view + effect data, not new behavior).
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — **the load-bearing boundary doc.** §2 layer
  responsibilities (scoring/terminal detection live in `games/*`), §4 typed-module authoring,
  §5 allowed static data (including **explanation templates keyed to Rust effects/actions**),
  §11 UI-metadata boundary, **§12 explanation-template boundary** (the safe-interpolation rule).
  This defines where the "why" data may live and how explanation text may be authored.

**Area law (governs the touched subsystems):**

- `docs/OFFICIAL-GAME-CONTRACT.md` — §5 (RULES.md must document scoring + terminal conditions;
  HOW-TO-PLAY must teach scoring/winning) and §10 (UI exposure requirements). **Amendment site:
  the victory-explanation requirement becomes part of what makes a game official/web-exposed.**
- `docs/UI-INTERACTION.md` — the public visual target, legal-only interaction, effect-driven
  animation, accessibility, the §18 UI acceptance check. **Amendment site: there is currently
  no mandated "outcome/victory display" public-UI target; this spec adds one.**
- `docs/WASM-CLIENT-BOUNDARY.md` — viewer-safe browser-payload rules and the dev-panel data
  whitelist; the outcome rationale must be a viewer-safe public payload.
- `docs/AI-BOTS.md` — hidden-information safety for explanations; the no-leak discipline that
  bot/outcome explanations must honor (relevant because outcome text must never leak hidden state).
- `docs/ROADMAP.md` — the staged gate ladder; confirms where this non-gate, cross-game
  UI-infrastructure work sits relative to the gates and the approaching trick-taking/poker work.

**Planning + precedent (frame this as a delta, not a cold start):**

- `specs/README.md` — the living spec index and progress tracker (Gate 10 / `poker_lite` is the
  current in-progress gate); shows how the non-gate `rules-display-shared-surface` spec was slotted in.
- `archive/specs/rules-display-shared-surface.md` — **the precedent spec to mirror structurally**
  (positioning, content contract, catalog-complete obligation, CI guard, ticket decomposition).
- `reports/rules-display-research-brief.md` — the precedent's locked brief; reuse its authority
  ordering, static-delivery reasoning, and accessible-surface design where they transfer.

**Templates (the per-game enforcement amendment sites):**

- `templates/README.md` — template lifecycle/order and which templates a game must fill.
- `templates/GAME-UI.md` — the per-game UI plan required for web exposure; **primary template
  amendment site** (add a victory/outcome-display section).
- `templates/GAME-RULES.md` — original rules summary with stable rule IDs (R-SCORE-*, R-END-*);
  the scoring/terminal IDs the rationale must trace to.
- `templates/GAME-HOW-TO-PLAY.md` — player-facing prose with a "Scoring and winning" section;
  confirm alignment so the end-of-match explanation matches the taught rules.

**Decomposition conventions (the spec must be ticket-ready):**

- `tickets/README.md` and `tickets/_TEMPLATE.md` — the ticket authoring contract and required
  sections; the `RULDISSHASUR` series (8 tickets: contract → checks → pilot → catalog → infra →
  wiring → smoke → closeout) is the decomposition pattern to echo.

**The worked failing case (read its docs in full):**

- `games/poker_lite/docs/UI.md` — the current Crest Ledger UI contract, including its minimal
  terminal/showdown panels and no-leak requirements.
- `games/poker_lite/docs/RULES.md` — the scoring/terminal rules the explanation must faithfully
  surface (yield win, showdown win, split; pair-vs-rank strength).

### Code seams to inspect directly (inspect, **not** read-fully — read in the repo; do not paste)

- `games/*/src/visibility.rs` and `games/*/src/rules.rs` for all nine games (`column_four`,
  `directional_flip`, `draughts_lite`, `high_card_duel`, `poker_lite`, `race_to_n`,
  `secret_draft`, `three_marks`, `token_bazaar`) — what each terminal detection / scoring path
  computes vs. what the public `TerminalView`/`PublicView` actually exposes. Catalogue the gap
  per game.
- `games/poker_lite/src/rules.rs` (`showdown_strength`, `compare_showdown`) and
  `games/poker_lite/src/visibility.rs` (`ShowdownView`, `TerminalView`) — the exact
  hidden-rationale gap to close.
- `games/*/src/effects.rs` — terminal/win-reason effects (e.g. `draughts_lite`'s
  `TerminalWinReason`) that already carry rationale data into the effect stream.
- `apps/web/src/wasm/client.ts` — the hand-written TS PublicView / Terminal types per game (the
  TS side that must learn the new rationale fields).
- `apps/web/src/components/*Board.tsx` and `apps/web/src/components/RulesPanel.tsx` — the
  existing per-game terminal panels and the shared rules-surface pattern to mirror for outcome.
- `apps/web/e2e/rules-display.smoke.mjs` — the smoke/no-leak test precedent for a shared surface.
- `crates/engine-core` and `crates/game-stdlib` — confirm there is **no** existing shared
  outcome/winner/scoring type (engine-core must stay noun-free); this bears on the §3.7 ADR
  determination.

---

## 3. Settled intentions (final — these make this brief *locked*)

These were resolved with the owner. State the deliverable around them; do not re-open them.

1. **Deliverable = one spec.** Produce a single roadmap-style spec at `specs/<slug>.md`,
   modeled structurally on `archive/specs/rules-display-shared-surface.md`. The proposed
   **foundational-doc amendments and new/changed template text are included as clearly-marked,
   lift-ready appendix sections inside that one spec** — not as separate files, and not merely
   described. Each appendix gives the actual proposed prose (or redlined replacement blocks)
   ready to drop into the target doc/template at ticket time.

2. **Positioning: a non-gate, cross-game UI-infrastructure spec, done now** — the sibling of
   rules-display — slotted into `specs/README.md` ahead of resuming the gate ladder, because the
   win-explanation gap must be closed before the roadmap reaches `plain_tricks` and full poker
   scoring. `assumption:` non-gate placement executed ahead of the next gate (the owner can
   override to instead fold it into a gate); proceed on the non-gate assumption.

3. **Scope = explain WHY, across all nine games, binding on all future games.** Every game's
   Rust public/terminal view must carry a **viewer-safe victory rationale** — the *decisive
   cause* of the result, expressed in that game's terms: the winning line (`three_marks`,
   `column_four`), the deciding score/standing and tiebreaker (`directional_flip`,
   `token_bazaar`, `secret_draft`, `high_card_duel`), the terminal reason (`draughts_lite`), the
   race threshold (`race_to_n`), and the **showdown strength comparison** (`poker_lite` — the
   pair-vs-rank reasoning currently trapped in Rust). A **shared cross-game outcome surface**
   renders it. All nine existing games are retrofitted; `poker_lite`'s hidden `ShowdownStrength`
   is explicitly fixed.

4. **Breakdown depth: decisive cause + full per-player breakdown.** The surface shows the
   deciding factor **and** a viewer-safe score/standing breakdown for **every** player — strictly
   respecting hidden information (e.g. on a `poker_lite` *yield*, the loser's private crest stays
   hidden and the explanation must say the match resolved without a private reveal; a player's
   own hidden data may be shown to that viewer only).

5. **Strictly explain the actual result — no forward-looking coaching.** Faithfully explain why
   *this* outcome occurred (the decisive rule / comparison / tiebreaker / winning configuration).
   **No** hypothetical "what would have changed it" / turning-point coaching — that is explicitly
   out of scope (it expands the leak surface and drifts toward AI advice).

6. **Mandatory + enforced for current and future games.** This is a hard requirement, not a
   recommendation. The spec must amend `docs/OFFICIAL-GAME-CONTRACT.md` (§5/§10) and
   `docs/UI-INTERACTION.md` (add an outcome/victory-display public-UI target + acceptance check),
   extend `templates/GAME-UI.md` (and, where alignment requires it, `templates/GAME-RULES.md` /
   `templates/GAME-HOW-TO-PLAY.md`), and add a **CI / coverage check** (mirroring the
   rules-display catalog guard) so every catalog game — current and future — ships a viewer-safe
   victory explanation, enforced *during new-game creation*.

7. **Architecture default + ADR determination (bounded delegation to you).** Default design:
   **per-game public-view enrichment** (each `games/*` projects its own typed, viewer-safe
   rationale into its `TerminalView`/`PublicView`) + a **shared TypeScript render surface** +
   **static explanation templates keyed to Rust effects/actions** (per ENGINE-GAME-DATA-BOUNDARY
   §5/§12). This keeps `engine-core` **noun-free** and TS presentation-only. Do **not** introduce
   a generic shared outcome type in `engine-core`/`game-stdlib` unless your analysis shows clear,
   atlas-governed repeated pressure for one; if it does, route it through the mechanic-atlas
   process rather than inventing it ad hoc. **Determine whether any change here trips a
   FOUNDATIONS §13 ADR trigger** (e.g. changing public-UI requirements, view/visibility
   semantics, or engine-core vocabulary); if it does, the spec must include a drafted ADR (using
   `docs/adr/ADR-TEMPLATE.md`) as an appendix and name the exact sections it supersedes. If it
   does not, say so explicitly with reasoning. This is the one open design sub-decision delegated
   to you — everything in §3.1–§3.6 is locked.

---

## 4. The task

Author a single, ticket-ready **new spec** (target type: *new spec*) that establishes a
mandatory, enforced, cross-game **victory-explanation shared surface**: the web UI must, at the
end of every match in every game, clearly and pleasantly show **why the result happened** — the
decisive cause plus a viewer-safe per-player breakdown — driven entirely by Rust-owned,
hidden-info-safe, replay-deterministic public-view and effect data, with the requirement baked
into the official-game contract, the UI doctrine, the per-game templates, and CI so that it
binds every current and future game. The spec must read as a delta from the completed
rules-display-shared-surface work, retrofit all nine existing games (fixing `poker_lite`'s
inscrutable showdown), and ship as one document with lift-ready appendices for every proposed
doc/template amendment (and an ADR if §13 is triggered). The goal behind the deliverable: a
player finishes a match thinking *"I fully understand why I won/lost — I want to play again."*

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the code seams,
trace every game's terminal/scoring path, and verify each claim in §1 against the actual tree at
`e0e7735`; flag any contradiction prominently in the deliverable.

Research online as deeply as needed and cite sources for any external claim that shapes a
decision. Worthwhile directions: end-of-match / results-screen UX and "why did I win" affordances
in digital board/card games; how strong implementations explain poker hand strength, trick
scoring, and tiebreakers to non-expert players; progressive-disclosure patterns (summary →
expandable breakdown) for information-dense outcome screens; accessibility of results displays
(screen-reader outcome summaries, color-independent encoding, reduced motion); transparency /
"explainable outcome" patterns and any relevant HCI/usability research; and patterns for
representing scoring rationale as data + templates rather than imperative logic. Use this to make
the surface genuinely informative and pleasant — not to re-decide anything locked in §3.

---

## 6. Doctrine & constraints (honor all; they bind the spec)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first, never designing against it silently.
- **Authority order:** foundation docs govern area docs govern specs govern tickets. The spec
  conforms to the foundation set and never overrides it; amendments to foundation/area docs are
  *proposals* the spec drafts, to be enacted through their own change process.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  `score`, `winner` vocabulary; typed mechanic nouns belong in `games/*` first, shared helpers in
  `game-stdlib` only via the mechanic atlas. The victory rationale is per-game typed data.
- **TypeScript never decides legality or outcome.** The "why" is computed and projected by
  Rust/WASM; TS renders the supplied viewer-safe payload only — no TS-guessed reasoning.
- **No YAML and no DSL without an accepted ADR.** Static data stays typed content / parameters /
  metadata / explanation-templates-keyed-to-effects — never selectors, conditions, or triggers.
- **Determinism:** the rationale is part of views/effects/replay; replay, hashes, RNG,
  serialization order, and traces stay deterministic (or are explicitly migrated).
- **No hidden-information leaks** into payloads, DOM text/attributes, `data-testid`s, storage,
  logs, effect logs, bot/outcome explanations, or replay exports. A reveal only shows what the
  rules have made public to that viewer (yielded hands stay hidden; a viewer sees only their own
  private data).
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4). The spec must *add* no-leak/outcome smoke coverage.
- **Deliver complete files or coherent complete sections, not diffs** (repo authoring rule) —
  applies to the spec's drafted appendices: give complete, lift-ready prose blocks.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **NEW — `specs/<slug>.md`** (choose a clear kebab-case slug, e.g.
  `victory-explanation-shared-surface.md`; state the exact filename you chose at the top).
  Structure it like `archive/specs/rules-display-shared-surface.md` (status header, motivation,
  scope/non-scope, authority/constraints, the cross-game design, the per-game retrofit plan, the
  enforcement plan, acceptance/verification, and a ticket-decomposition outline echoing the
  `RULDISSHASUR` 8-ticket shape). It must contain at least:
  1. **The shared outcome surface design** — a cross-game React surface (mirroring the rules
     panel's catalog-complete, accessible, no-leak posture) that renders, at terminal: the
     decisive cause, the per-player viewer-safe breakdown, and the final standing — with
     progressive disclosure, accessibility (screen-reader outcome summary, color-independent
     encoding, reduced motion), and reduced-motion/replay consistency.
  2. **The per-game Rust rationale contract** — what each `games/*` must project into its
     viewer-safe public/terminal view (and/or terminal effect) to make its outcome explainable,
     stated generically and noun-free at the boundary, then per-game.
  3. **The nine-game retrofit plan** — one subsection per game naming the current gap and the
     concrete rationale to expose; `poker_lite` called out explicitly (surface the
     pair-vs-rank `ShowdownStrength` comparison; keep yielded hands hidden).
  4. **The enforcement plan** — the CI/coverage check; the §10 official-contract gate; the
     UI-acceptance-check addition.
  5. **Appendices (lift-ready proposed text):** (a) `docs/UI-INTERACTION.md` amendment — the new
     outcome/victory-display public-UI target + acceptance-check lines; (b)
     `docs/OFFICIAL-GAME-CONTRACT.md` amendment — §5/§10 additions; (c) `templates/GAME-UI.md`
     addition — the victory/outcome-display section (and any `GAME-RULES.md`/`GAME-HOW-TO-PLAY.md`
     alignment edits); (d) **if and only if §3.7 finds a §13 trigger,** a drafted ADR using
     `docs/adr/ADR-TEMPLATE.md` naming the superseded sections; if no trigger, a short subsection
     stating that and why.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable is exactly **one** spec file at `specs/<slug>.md`, with all proposed doc/template
  amendments (and any ADR) as lift-ready appendix sections inside it — not separate files, not
  mere descriptions (§3.1, §7).
- Positioning is a **non-gate cross-game UI-infrastructure spec done now**, framed as a delta from
  `rules-display-shared-surface` (§3.2).
- All **nine** games have a named gap + concrete rationale-to-expose; `poker_lite`'s showdown
  comparison is explicitly surfaced while yielded hands stay hidden (§3.3, §3.4).
- The display **explains only the actual result** — no what-if/coaching (§3.5).
- The requirement is **mandatory and enforced** via contract + UI doctrine + templates + a CI
  check binding current and future games (§3.6).
- The architecture keeps **Rust as authority, TS presentation-only, engine-core noun-free**, with
  static explanation templates keyed to Rust effects; the §13 ADR question is explicitly resolved
  (drafted ADR if triggered, reasoned "no trigger" if not) (§3.7, §6).
- No proposed change **leaks hidden information** or weakens determinism/replay/tests, and the
  spec adds no-leak + outcome smoke coverage (§6).
- No proposed amendment silently weakens an upstream foundation doc or amends an accepted ADR
  without naming it.
- Every external claim that shaped a decision is **cited** (§5).
- The §1 fetch-baseline commit `e0e7735` contains every file named in the §2 read-in-full list
  (it does, as of authoring).
