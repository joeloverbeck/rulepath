# Research Brief — River Ledger: correctness audit + presentation/usability overhaul

_Paste this entire document into a fresh ChatGPT-Pro deep-research session and upload the
manifest named in §1. You (Session 2) have none of the authoring session's context; everything
you need is in this brief plus the uploaded manifest. This is a **locked / no-questions** brief:
produce the deliverable directly — do not interview._

---

## 1. Context

The uploaded manifest — **`manifest_2026-06-15_351dc1e.txt`** — is the exact path inventory of
the `joeloverbeck/rulepath` repository: a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**. The
foundation docs are an ordered, layered authority indexed by `docs/README.md`: `FOUNDATIONS.md`
(the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs →
`ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede them only by
explicitly naming the affected sections.

**Fetch every file from commit `351dc1e` (`351dc1ec47b976aecc376022b718d8f921ca4bcb`, branch
`main`)** — the manifest reflects exactly that tree. If any file you reference cites a different
"commit of record," note the divergence and use `351dc1e`.

The subject is the game **River Ledger** (`games/river_ledger/`, game id `river_ledger`): a
3–6 seat, fixed-limit, hidden-information community-card game in the Texas Hold 'Em rules family.
The owner wants two things: (a) confirmation of whether the **implementation has correctness
errors**, and (b) a researched plan to **improve the game's presentation and usability** — with a
specific, load-bearing pain point: *after the showdown, when winners are declared, what each
player's hand actually scored and **why** is not legible*. The owner has trouble reading standard
Texas Hold 'Em hand strength, so making "what beats what, and why" intuitive is the heart of the
task.

### Grounding facts established before this brief (treat as given; verify, don't rediscover from scratch)

- River Ledger is **deliberately a fixed-limit, abstract-contribution Hold'em (roadmap Gate 15)**.
  All-in handling, side pots, no-limit, and pot-limit are **explicitly out of scope by design**
  and deferred to Gate 15.1 (see `RULES.md` `RL-OOS-ALLIN-001`, `RL-POT-ALLIN-001`,
  `RL-VAR-ALLIN-001`, `RL-BET-AMB-001`). **Do not flag the absence of side pots / all-in / no-limit
  as a bug.** The correctness oracle is `games/river_ledger/docs/RULES.md` and its stable `RL-*`
  rule IDs — **not** casino No-Limit Hold'em.
- The showdown outcome data is **already wired end-to-end** as of the most recent commits
  (RIVLEDOUT-001 "river rationale" and RIVLEDOUT-002 "river outcome UI"). A live playthrough of
  the running web build (`http://127.0.0.1:4173/`, bot-vs-human, 6 seats) confirms the post-showdown
  panel **does** render per-seat data. The problem is **legibility / formatting / missing
  explanation**, framed as a *delta* on working plumbing — **not** a from-scratch rebuild.
- Observed current showdown panel (verbatim, from the live build) — this is the concrete artifact
  to improve. Board was `4C 3D QH 6H 8H`; the panel showed, per seat:

  ```
  OUTCOME — Seat 4 wins
  "One showdown hand has the strongest Rust-evaluated five-card result."

  Seat 4   WIN              Contribution 2  Allocation 12
           Category  one_pair
           Tie break 12,10,8,6
           Best five QC QH 10S 8H 6H
  Seat 0   SHOWDOWN_LOSS    Contribution 2  Allocation 0
           Category  one_pair
           Tie break 8,12,10,6
           Best five QH 10C 8D 8H 6H
  Seat 1   SHOWDOWN_LOSS    Category High card   Tie break 12,9,8,6,5   Best five QH 9H 8H 6H 5D
  … (Seats 2,3,5 similar) …
  ```

  The decisive fact — Seat 4 won because a **Pair of Queens beats Seat 0's Pair of Eights** — is
  *nowhere stated*; the user must know that the first integer of the tie-break vector is the pair
  rank and that `12 = Queen`, `8 = Eight`. The board and hole cards also render as plain text
  blocks ("CLUBS / 4C / four") with **no suit glyphs, no red/black coloring, and no card-like
  visuals**.

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing anything:

**Foundation & area docs (authority flows downward):**

```
docs/README.md — the authority order and the layering rule that governs every recommendation.
docs/FOUNDATIONS.md — the constitution: priority order, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every finding and recommendation must satisfy these.
docs/ARCHITECTURE.md — the layer architecture and "Rust owns behavior, TS presents" division that bounds every UI recommendation.
docs/ENGINE-GAME-DATA-BOUNDARY.md — the engine-core (noun-free) / games / static-data boundary; constrains where any new field or helper may live.
docs/UI-INTERACTION.md — the UI presentation law: TS renders Rust/WASM output only and decides no legality/evaluation; the binding contract for the showdown redesign.
docs/WASM-CLIENT-BOUNDARY.md — the Rust↔browser JSON bridge contract; governs how any new Rust-authored explanation field reaches the client.
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md — per-viewer projection and authorized-reveal rules; governs what showdown data may be shown to whom without leaking hidden info.
docs/OFFICIAL-GAME-CONTRACT.md — the official-game fidelity contract; the standard the correctness audit measures rule fidelity against.
docs/IP-POLICY.md — public/private posture and no-casino-trade-dress rule (RL-UI-NOCASINO-001); bounds the visual redesign away from copied card/table art.
docs/TESTING-REPLAY-BENCHMARKING.md — determinism, no-leak tests, replay/hash discipline; the methodology any correctness claim and any new field must respect.
docs/ROADMAP.md — Gate 15 / Gate 15.1 framing that fixes what is in scope vs deferred (all-in, side pots).
games/river_ledger/docs/RULES.md — THE correctness oracle: every RL-* rule ID the audit verifies against, plus the documented ambiguities and intended deviations.
games/river_ledger/docs/MECHANICS.md — the mechanic inventory backing the rule IDs; helps map rules to modules.
games/river_ledger/docs/UI.md — the UI contract: terminal-rationale template keys, showdown-reveal fields, and what is authorized to display.
games/river_ledger/docs/HOW-TO-PLAY.md — the player-facing rules and vocabulary; the baseline the new explanatory copy must stay consistent with.
games/river_ledger/docs/RULE-COVERAGE.md — RL-* → module/test/trace mapping; your fastest index from a rule to the code and the test that proves it.
games/river_ledger/docs/COMPETENT-PLAYER.md — strategy/hand-strength notes; useful context for an intuitive strength indicator and the teaching layer.
games/river_ledger/docs/SOURCES.md — source notes for the rules; check before citing external Hold'em authorities.
games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md — the presentation/readiness gates a UX change must not regress.
```

**Code seams to inspect directly** (read these *in the repo*; they are **not** pasted and are
*inspect, not read-fully*):

- `games/river_ledger/src/evaluator.rs` — hand categories, tie-break vectors, ace-low straight, best-5-of-7 enumeration; the core of the correctness audit.
- `games/river_ledger/src/showdown.rs` — showdown resolution, winner/split determination, and the `ShowdownReveal` / explanation builder where a new Rust-authored "why X won" field would live.
- `games/river_ledger/src/pot.rs` — single-pot allocation and stable button-order split-remainder logic.
- `games/river_ledger/src/betting.rs` — betting-round closure, call price, raise-cap accounting.
- `games/river_ledger/src/rules.rs` — action application and street transitions (preflop→flop→turn→river→showdown).
- `games/river_ledger/src/actions.rs` — legal-action tree, command validation, raise-cap enforcement.
- `games/river_ledger/src/state.rs` — `ShowdownReveal`, `TerminalOutcome`, `Street`, `SeatStatus`, contribution ledger types.
- `games/river_ledger/src/visibility.rs` — view projection: `PublicView`, `OutcomeRationaleView`, `SeatOutcomeBreakdownView`, `ShowdownStrengthView` (`category`, `tie_break_vector`, `best_five`); the seam where the human-readable explanation must be authored to satisfy RL-UI-SHOWDOWN-001.
- `games/river_ledger/src/ui.rs` — UI payload assembly.
- `games/river_ledger/src/cards.rs` — `Rank`/`Suit`/`Card` and the rank→value (2–14) mapping the tie-break integers come from.
- `games/river_ledger/src/setup.rs` — seat/blind/button setup and deal.
- `apps/web/src/components/RiverLedgerBoard.tsx` — the React table: seats, board, hole cards, **the card-rendering markup itself lives here** (no separate card component), action panel, and where the outcome panel is mounted.
- `apps/web/src/components/OutcomeExplanationPanel.tsx` — the outcome panel and the `outcomeSurfaceData()` adapter that currently flattens rationale to a standing.
- `apps/web/src/components/outcomeExplanationTemplates.ts` — the static template copy (e.g. the "strongest Rust-evaluated five-card result" string that leaks engine jargon).
- `apps/web/src/wasm/client.ts` — the TS view type definitions (`RiverLedgerTerminalView`, `RiverLedgerOutcomeRationale`, card view types); shows what the bridge currently exposes vs. what a redesign needs.
- `crates/wasm-api/src/lib.rs` — the JSON bridge; the single seam any new Rust outcome field must pass through to reach the browser.
- `games/river_ledger/tests/{rules,property,visibility,serialization,replay,bots}.rs` and `games/river_ledger/tests/golden_traces/*.trace.json` — especially `full-house-tiebreak`, `flush-kicker-order`, `straight-ace-low`, `split-pot-even`, `split-pot-remainder-button-order`, `high-card-showdown`, `pair-beats-high-card`, `foldout-last-live-hand`; these are the existing correctness evidence and the oracle for what is already proven vs. unproven.

---

## 3. Settled intentions (final — these pre-empt every clarifying question)

These decisions are locked. Do not re-open them.

1. **Deliverable = one research report with recommendations.** Not a formal spec, not tickets.
   You research, audit, and recommend; the owner will later convert your report into specs/tickets
   through the repo's own pipeline. Structure the report so that conversion is easy (clear,
   independently-actionable, prioritized items) but do **not** author `RL-*` rule IDs or ticket files.
2. **Correctness audit = whole game, showdown-weighted.** Audit setup/blinds, fixed-limit betting,
   raise cap, round closure, pot allocation and split remainder, AND hand evaluation/showdown —
   verified against the `RL-*` rule IDs in `RULES.md`. Weight depth toward the showdown/evaluator
   path, since that is where the owner's confusion and the highest correctness risk concentrate.
3. **Presentation overhaul = whole table UI, showdown first.** Cover card/suit rendering, the board,
   seats, the action panel, and turn flow — but treat the **post-showdown outcome panel as the top
   priority**, with the rest of the table UI as secondary.
4. **"Score" means hand RANKING legibility + an optional teaching strength indicator.** Texas Hold 'Em
   has no numeric score, only a hand category plus ordered kickers. The deliverable must make the
   *ranking* legible: named categories ("Pair of Queens", not `one_pair`), an explicit plain-English
   "**why this beats that**" statement ("Pair of Queens beats Pair of Eights"), the winning five
   cards shown as **visual cards with suit glyphs and red/black (and colorblind-safe) distinction**,
   and an **always-available hand-ranking reference** (the ladder of categories). **In addition**,
   explore an *optional* intuitive hand-strength indicator (e.g. a strength meter or 0–100% relative
   ranking) as a clearly-labeled **learning aid** — it must be visibly marked as a teaching aid, never
   presented as a canonical game value, and must not imply hidden information (see constraint 5). Do
   **not** invent a fake numeric "score" and present it as if the engine computed one.
5. **Every recommendation must honor these binding constraints** (they are not negotiable design
   trade-offs — a recommendation that violates one is out of scope):
   - **`RL-UI-SHOWDOWN-001` / `docs/UI-INTERACTION.md`**: TypeScript renders Rust-authored fields and
     does **no** winner or evaluator logic. Therefore any human-readable hand description, category
     label normalization, or "why X beats Y" narrative should be authored as a **new Rust-authored
     field** projected through `visibility.rs` → `crates/wasm-api` → the client — **not** synthesized
     in TypeScript by re-interpreting the tie-break vector. Call this out explicitly wherever you
     recommend new copy or labels.
   - **No hidden-information leaks** (`RL-VIS-*`, `RL-UI-NOLEAK-001`): folded seats' hole cards, deck
     tail, future board, and burn positions must never reach any browser surface (DOM, a11y labels,
     test IDs, storage, logs). A strength indicator must derive only from authorized, revealed
     showdown data — never from unrevealed hands.
   - **No casino trade dress** (`RL-UI-NOCASINO-001`, `docs/IP-POLICY.md`): the visual redesign must
     use original, neutral card/table styling — no copied or evocative casino art, chips imagery as
     real-money framing, or branded table felt. Abstract contribution units stay abstract.
   - **Determinism & engine boundaries**: `engine-core` stays noun-free; no new DSL/YAML; replay,
     hashes, and serialization order stay deterministic.
6. **Frame the showdown fix as a delta on already-wired plumbing**, per the grounding facts in §1 —
   the `category` / `tie_break_vector` / `best_five` data already flows to the client. The gap is the
   missing *human-readable* layer and the missing *visual/educational* presentation, not the data
   pipeline. (If your inspection finds the data is in fact NOT reaching a given surface, report that as
   a finding — but the baseline assumption is "wired but illegible.")

`assumption:` whether the always-available hand-ranking reference is a persistent legend, a
just-in-time tooltip, or a collapsible panel — and the exact visual form of the strength indicator —
are **delegated to your design judgment**; recommend a default with rationale rather than treating
these as open questions.

---

## 4. The task

This is an **audit + presentation-overhaul research report** ("other" target type: a correctness
audit fused with a UX overhaul). Achieve two goals. **First**, determine whether River Ledger's
implementation is *correct* against its own `RL-*` rule contract — with the hand evaluator, tie-break
ordering, best-5-of-7 selection, pot/split allocation, and showdown winner determination as the focus,
and the surrounding betting/setup machinery covered more lightly — and state a clear verdict per area
with evidence. **Second**, design a researched, prioritized plan to make River Ledger's presentation
and usability markedly better, with the **post-showdown "what each hand scored and why" experience as
the centerpiece**: a redesign that a player who struggles to read Texas Hold 'Em hand strength can
understand at a glance. Ground both halves in external research (real poker UIs, hand-strength
visualization, HCI/usability literature, accessibility) and in the repo's own boundary law.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow `RULE-COVERAGE.md`
from any rule ID into its module and test, read whatever evaluator/showdown/pot code and golden traces
you need to confirm a verdict, and trace the outcome payload from `visibility.rs` through
`crates/wasm-api/src/lib.rs` into `apps/web/`.

Research online as deeply as needed — and the owner explicitly wants this: study **similar
implementations** (open-source poker clients and hand evaluators; how established poker UIs present
showdown results, winning-hand highlighting, and hand naming), **research papers and HCI/usability
literature** (explaining outcomes of complex rule systems to novices; progressive disclosure;
learnability), **web/visual design** for card legibility, and **accessibility** (colorblind-safe suit
encoding, screen-reader-friendly hand descriptions, WCAG contrast). For the correctness half, cross-check
the evaluator's category ordering, kicker rules, ace-low straight handling, and split-remainder rule
against authoritative Texas Hold 'Em hand-ranking references. **Cite every external source that shapes a
recommendation or a verdict.**

Worked example you should be able to explain end-to-end in the report (from the live capture in §1):
board `4C 3D QH 6H 8H`, Seat 4 hole gives `QC QH 10S 8H 6H` (Pair of Queens, kickers 10/8/6) and Seat 0
gives `QH 10C 8D 8H 6H` (Pair of Eights, kickers Q/10/6). Seat 4 wins because Queens (pair rank 12) outrank
Eights (pair rank 8). Show exactly how the redesigned panel would make that legible to a novice, and
confirm the engine's tie-break vectors (`12,10,8,6` vs `8,12,10,6`) encode this correctly.

---

## 6. Doctrine & constraints

Honor these throughout (they bound which recommendations are admissible):

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior recommendation must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence would require an
  accepted ADR superseding the affected principle first, never designing against it silently.
- **Authority order**: foundation docs govern area docs govern game docs/specs; if a recommendation
  conflicts with `FOUNDATIONS.md`, `ARCHITECTURE.md`, or the boundary docs, the recommendation is wrong.
- **TypeScript never decides legality or evaluation.** Legal actions, validation, views, winner, split,
  and hand strength all come from Rust/WASM (`RL-UI-SHOWDOWN-001`, `RL-OOS-BROWSER-001`,
  `docs/UI-INTERACTION.md`). New human-readable outcome text is Rust-authored.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `pot`, `bet`, `showdown`,
  evaluator vocabulary. River Ledger's nouns stay in `games/river_ledger`; no promotion to shared
  helpers is authorized (`RL-OOS-ENGINE-001`).
- **No YAML, no DSL** without an accepted ADR; static data is typed content/metadata only.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic (or are
  explicitly migrated with matching coverage updates).
- **No hidden-information leaks** into payloads, DOM, accessibility labels, test IDs, storage, logs,
  effect logs, bot explanations, or replay exports (`RL-VIS-*`, `RL-UI-NOLEAK-001`).
- **No casino trade dress / real-money framing** (`RL-UI-NOCASINO-001`, `docs/IP-POLICY.md`).
- **In-scope vs deferred**: all-in, side pots, no-limit, pot-limit, tournaments are **out of scope by
  design** (Gate 15.1+). Do not recommend them and do not treat their absence as a defect.
- **Never recommend deleting or weakening tests to get green** — correctness fixes must come with, or
  call for, strengthened coverage (`AGENT-DISCIPLINE §4`).

---

## 7. Deliverable specification

Produce **one new downloadable markdown document**:

**`river-ledger-correctness-and-presentation-report.md`** (new file; the owner will place it under
`reports/`). It must contain, in order:

- **Part A — Correctness audit.** For each area (setup/blinds; fixed-limit betting & raise cap; round
  closure; pot allocation & split remainder; hand evaluation — categories, kickers, ace-low straight,
  best-5-of-7; tie-break ordering; showdown winner/split; visibility/no-leak at showdown), give a
  **verdict** (`correct` / `bug` / `ambiguous`), the **evidence** (cite the `RL-*` rule, the module,
  and the test/trace that does or does not prove it), and a **severity** for any defect. Explicitly
  state that the intended abstractions (no all-in/side pots/no-limit) are confirmed *not* bugs. Include
  the worked example from §5 as a verification of the tie-break encoding.
- **Part B — Presentation & usability overhaul** (whole table, showdown first). Must include:
  - a **concrete annotated redesign of the showdown outcome panel** as an ASCII/markdown mockup,
    showing named categories, the explicit "why this beats that" line, visual cards, winner
    highlighting, and the hand-ranking reference;
  - a **before/after copy table** (current engine-jargon strings → proposed player-facing copy,
    including replacing "strongest Rust-evaluated five-card result");
  - the **proposed new Rust-authored field shape** (what `visibility.rs` should emit and how it flows
    through `crates/wasm-api` to the client) so the new copy honors `RL-UI-SHOWDOWN-001`;
  - **card/suit rendering recommendations** for the board, hole cards, and best-five, including
    colorblind-safe suit encoding and screen-reader hand descriptions;
  - the **optional strength-indicator teaching-aid exploration**, clearly framed as a learning aid with
    its leak-safety and "not a canonical value" caveats;
  - secondary table-UI/usability improvements (seats, action panel, turn flow) at lower priority.
- **Part C — Prioritized recommendation backlog.** A ranked list of independently-actionable items,
  each tagged with the boundary constraint(s) it touches (Rust-authored vs TS-only, leak-safety,
  no-casino, determinism) and a rough effort/impact note — structured so the owner can convert items
  into specs/tickets.
- **Sources.** All external research cited inline and collected at the end.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the report and proceed with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- The fetch-baseline commit `351dc1e` contains every file named in the §2 read-in-full list, and you
  read them in authority order.
- Every correctness verdict cites a specific `RL-*` rule ID plus the module and the test/trace that
  bears on it; no verdict rests on casino-NLHE expectations rather than `RULES.md`.
- The report nowhere treats all-in / side pots / no-limit absence as a defect.
- Every presentation recommendation that adds player-facing text or labels routes it through a
  **Rust-authored field**, not TypeScript evaluator/winner logic — and you say so explicitly.
- No recommendation introduces a hidden-information leak, casino trade dress, a new DSL/YAML, or a
  determinism break.
- The showdown redesign makes the §5 worked example ("Queens beat Eights") legible to a novice, and the
  strength indicator (if recommended) is explicitly marked a teaching aid and is leak-safe.
- The deliverable set matches §7 exactly: one report, Parts A/B/C plus Sources, with the concrete
  showdown mockup, before/after copy table, and new-field shape present.
- Every external claim that shapes a verdict or recommendation is cited.
