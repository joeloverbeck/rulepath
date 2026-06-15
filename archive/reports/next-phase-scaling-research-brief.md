# Research Brief — Next-Phase Scaling: 3+ Players, Larger Surfaces, Texas Hold'Em

> **For ChatGPT-Pro (deep-research session). You are locked, no questions.** Everything you
> need is in this prompt plus the uploaded manifest. Read the files named below from the repo,
> research online as deeply as needed, and **produce the two deliverables directly** as
> downloadable markdown documents. Do not interview; do not ask clarifying questions. The
> requirements here are final.

---

## 1. Context

The uploaded manifest — `manifest_2026-06-13_a97625c.txt` — is the exact path inventory of the
`joeloverbeck/rulepath` repository: a **Rust-first, rule-enforcing, replayable, testable
card/board-game platform where Rust owns all behavior and TypeScript/React present only.** The
foundation docs are an ordered, layered authority indexed by `docs/README.md`: `FOUNDATIONS.md`
(the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs →
`ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede them only by
explicitly naming the affected sections.

**Fetch every file from commit `a97625c` (full SHA `a97625c43028d425d400bc8a4a112b9b6ffba899`)
— the uploaded manifest reflects exactly that tree.** The working tree is clean at this commit.
(One earlier same-day manifest exists in `reports/` at a different commit, `2b86670`; ignore it
and use `manifest_2026-06-13_a97625c.txt`.)

### Where the project stands (verified against the code, not just the docs)

- The repo has completed a **public mechanic ladder of 14 games** (Gates 0–14). The roadmap's
  only remaining item is **Gate P**, a deferred, isolated, non-public "private monster-game
  red-team." There is **no public ladder beyond Gate 14** — the next phase is unwritten.
- **All 14 games are hardcoded to exactly two seats.** Each game crate defines a 2-variant
  per-game `Seat` enum in `games/<game>/src/ids.rs`, and its `setup` rejects any seat count ≠ 2.
  The `simulate` tool tracks outcomes as `seat_0_wins` / `seat_1_wins`. This 2-player ceiling
  lives **entirely in the game crates and tooling**, not in the kernel.
- **`engine-core` is already seat-generic.** `Game::setup(seats: &[SeatId])` accepts any seat
  count; visibility is modeled as `VisibilityScope::PrivateToSeat(SeatId)`. Nothing in the
  kernel assumes two players.
- **The largest surfaces today are tiny**: `event_frontier` (a 6-site graph + 21-card event
  deck + 3 epochs) and `frontier_control` (a 7-site graph + asymmetric factions). These are
  orders of magnitude smaller than the eventual private aspirations (COIN-series, Imperium:
  Classics, Arkham Horror LCG).
- **The mechanic atlas has zero open promotion debt**, but several 2nd/3rd-use repeated-shape
  candidates are armed (graph-map topology, site control, faction asymmetry, public-resource
  accounting, reaction windows) and would trip the atlas's third-use hard gate as surfaces and
  player counts grow.
- **IP**: poker played with a standard 52-card deck is public-domain as a *system*; `IP-POLICY.md`
  only constrains casino-flavored *product presentation* terms. The existing `poker_lite` game
  (product name "Crest Ledger") is a separate 9-card abstract 2-seat betting proof — **not** the
  same as the proper Texas Hold'Em requested below.

The user's thesis: the app has proven small, two-player games, but has **not** proven it can
handle more players or larger surfaces — which nearly every ambitious private target requires.
The next phase must build that capability incrementally through **public** games, with Texas
Hold'Em as the first committed rung, while keeping the private monster game last.

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing anything. Earlier docs govern later ones.

**Foundation set (authority order per `docs/README.md`):**

- `docs/README.md` — the authority order and the layering rule; the map of the whole doc set.
- `docs/FOUNDATIONS.md` — the constitution: product priority, behavior authority, universal
  invariants, stop conditions, and ADR triggers; every recommendation must satisfy these or
  explicitly call for an ADR that supersedes the named principle.
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the action/view/effect/replay/
  determinism model, the WASM API shape, and the future-hosted-multiplayer stance; establishes
  that the kernel is seat-agnostic and what "promoted helper" conformance allows.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the noun-free `engine-core` rule, the `game-stdlib`
  promotion boundary, the typed-static-data and no-DSL/no-YAML law; governs any "shared helper
  for N seats / graph maps" proposal.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official (rules → coverage → mechanic
  inventory → bots → UI → outcome explanation → traces); the outcome-explanation surface and
  per-player breakdown requirements that Hold'Em's showdown must satisfy.
- `docs/MECHANIC-ATLAS.md` — the mechanic inventory, the first/second/third-use rule, the hard
  gate, the promotion-debt register (currently empty), and the armed repeated-shape candidates
  that scaling will pressure (graph-map topology, site control, faction asymmetry, resource
  accounting, reaction windows, deterministic-shuffle/private-hand).
- `docs/AI-BOTS.md` — bot law, the bot-level ladder, the v1/v2 exclusion of MCTS/ISMCTS/Monte
  Carlo/ML/RL, and hidden-information safety; the basis for whether/how bots scale to >2 seats
  with belief over multiple opponents.
- `docs/UI-INTERACTION.md` — the public visual target, legal-only interaction, effect-driven
  animation, accessibility, and the outcome/victory surface that already speaks of "every
  player"; the gap analysis for multi-seat panels, turn-order display, and showdown rendering.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden traces, deterministic replay/hash
  discipline, no-leak tests, and performance budgets; the basis for N-player no-leak proofs and
  larger-surface benchmark pressure.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema (including the `seats` array); whether the
  schema already accommodates N seats or needs an explicit, documented migration.
- `docs/ROADMAP.md` — the prescriptive staged ladder (Gates 0–14) and Gate P; the document whose
  successor phase Doc 2 effectively proposes. Note its "skip/reorder only by accepted ADR" rule.
- `docs/IP-POLICY.md` — public/private content policy, neutral naming, original-prose rule; the
  basis for the Texas Hold'Em naming/presentation caveat and the public-domain-game ladder.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law (bounded tasks, forbidden changes, failing-test
  protocol); note it explicitly calls "implement multiplayer" a *bad, vague* task — so the
  capability work must be decomposed into bounded, sequenced pieces.
- `docs/WASM-CLIENT-BOUNDARY.md` — the Rust/WASM-to-browser contract, operation groups, replay
  safety, and dev-panel whitelist; the seam where "active seat" is currently singular and
  multi-seat view projection must be reasoned about.
- `docs/SOURCES.md` — the researched bibliography and Rulepath-specific lessons; where any new
  sources you cite (poker, imperfect-information AI, multiplayer determinism, large-map systems)
  should be recommended for addition.
- `docs/archival-workflow.md` — the repo's archival conventions; relevant because the user will
  archive `specs/README.md` date-suffixed and write a new order from Doc 2 — your ordering
  recommendation should fit this workflow.

**Templates (read all — these must scale to N seats and larger surfaces):**

- `templates/README.md` and every template file: `AGENT-TASK.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`,
  `COMPETENT-PLAYER.md`, `GAME-AI.md`, `GAME-BENCHMARKS.md`, `GAME-HOW-TO-PLAY.md`,
  `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-MECHANICS.md`, `GAME-RULE-COVERAGE.md`,
  `GAME-RULES.md`, `GAME-SOURCES.md`, `GAME-UI.md`, `PRIMITIVE-PRESSURE-LEDGER.md`,
  `PUBLIC-RELEASE-CHECKLIST.md` — assess each for baked-in 2-player / small-surface assumptions
  and for fields the next phase needs (seat ranges, per-seat views, showdown/outcome breakdown,
  topology scale, N-player no-leak evidence).

**Planning + decision context:**

- `specs/README.md` — the living spec index and progress tracker; the file the user will archive
  and replace. Note the Gate P "Not started" row and the spec format your Doc 2 order must feed.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the accepted hidden-info/visibility
  contract your N-player no-leak reasoning must satisfy and not silently amend.
- `docs/adr/0006-blackjack-lite-roadmap-placement.md` — a worked precedent for deferring a
  card-game placement decision; mines the reasoning style the roadmap uses for sequencing.
- `docs/adr/ADR-TEMPLATE.md` — the ADR format any ADR you *outline* must follow.
- `reports/poker-lite-gate-10-research-brief.md` — the prior poker (betting/showdown) research;
  frame Texas Hold'Em as a **delta** from this 2-seat abstract proof, not a cold start.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)

- `crates/engine-core/src/**` — the seat/player/actor/viewer/`Game`-trait types (`SeatId`,
  `PlayerId`, `Actor`, `Viewer`, `VisibilityScope`); confirm the kernel is genuinely seat-generic.
- `games/<any one game>/src/ids.rs` and `.../src/setup.rs` — the concrete 2-seat lock (2-variant
  `Seat` enum + setup rejecting seat counts ≠ 2) that every game repeats.
- `games/event_frontier/**` and `games/frontier_control/**` — the largest current surfaces (graph
  maps, factions, event decks); the closest existing analogues to the next phase's scale.
- `games/poker_lite/**` — the existing abstract betting/showdown game (Crest Ledger) Hold'Em
  builds beyond.
- `tools/simulate/src/main.rs` — the `seat_0_wins`/`seat_1_wins` reporting that N-player work
  must generalize; sample of tooling that assumes two seats.
- `apps/web/**` — the presentation shell's seat/view handling and outcome surface; where
  multi-seat panels, turn-order display, and showdown rendering would land (presentation only).

---

## 3. Settled intentions (final — do not reopen)

These decisions are locked by the requester. **Confirm and document** them (citing the evidence
in §1); do **not** re-decide whether to pursue the next phase, and do **not** ask the user to choose.

1. **The public mechanic ladder continues past Gate 14.** The next phase is real, public work:
   grow the engine, code, and app's capability to handle **3 or more players** and **substantially
   larger game surfaces** (toward — not yet reaching — COIN-series / Imperium: Classics / Arkham
   Horror LCG scale). Anchor this confirmation in the verified facts of §1 (2-seat lock lives in
   game crates and tooling; `engine-core` is already seat-generic; zero open promotion debt;
   roadmap tail is only Gate 14 + deferred Gate P).

2. **Texas Hold'Em proper is the first committed public game of the new phase** — a real 52-card
   deck, multiple players, with a clear breakdown at showdown of *why* each player won or lost
   (the winning five-card combination and the decisive comparison). This game is locked in; only
   its feature *scope* is delegated (see §3.5).

3. **Gate P (the private monster game) stays in the roadmap but moves to the very tail of the
   queue**, after the entire new public ladder. It remains isolated, optional, and non-public,
   and must not drive public architecture.

4. **The deliverable is two separate downloadable markdown documents** (see §7). Neither
   overwrites a repository file; both are advisory inputs. The user will (a) revise the docs and
   templates themselves using Doc 1's recommendations, and (b) archive `specs/README.md`
   date-suffixed and author a new implementation order using Doc 2's recommended ordering.

5. **Three sub-decisions are deliberately delegated to you** — each requires a *recommended
   default plus justification grounded in your research*, **not** an open punt and **not** a
   question back to the user:
   - **(a) The ambition ceiling**: how far up the COIN/Imperium/Arkham scale the *public* ladder
     should ultimately climb before Gate P. Recommend the ceiling and justify it from prior art
     and the engine's current state. Gate P must remain last regardless.
   - **(b) Texas Hold'Em feature scope**: e.g., player range, blinds/betting-street model, hand
     evaluation, and whether side pots / all-in are in the first version. Recommend a scope. The
     **showdown "why each player won/lost" breakdown is non-negotiable and must be in scope.**
   - **(c) The game ladder itself**: which public-domain games, in what order, form the bridge
     from Hold'Em toward your recommended ceiling. Recommend concrete named games with mechanic
     and IP rationale.

6. Carried assumptions (the user can correct these later; treat them as defaults, not questions):
   - `assumption:` Texas Hold'Em is a **new** game that **coexists** with the existing
     `poker_lite` (Crest Ledger); it does not replace or rename it.
   - `assumption:` the foundation-doc-realignment analysis (Doc 1) covers **`docs/**` and
     `templates/**`**, with `specs/README.md` and the spec/ticket structure as authority context
     rather than primary realignment targets.
   - `assumption:` you provide **detailed change recommendations but do not finalize doc prose** —
     final wording is the user's. You **may** draft ADR outlines/triggers (using
     `docs/adr/ADR-TEMPLATE.md`) but must not present any ADR as accepted.

---

## 4. The task

This is a **foundational / doc-overhaul** target with a **new-spec (next-phase planning)**
component. Produce, through deep repo reading plus online research, two advisory markdown
documents that together (a) tell the user exactly how the foundation docs and templates must
change to admit a next phase of 3+ player and larger-surface games without weakening any upstream
law, and (b) lay out a researched, IP-safe public game ladder — Texas Hold'Em first — with the
concrete engine/app capability gaps each rung must close and a recommended phased implementation
order, keeping the private Gate P last. The goal is a credible, evidence-backed bridge from the
current proven state (small, two-player games on a seat-generic kernel) toward the ambitious
multi-player, large-surface targets the platform aspires to.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — read any game crate,
tool, or web-shell module that sharpens your analysis. Research online as deeply as needed:
similar open-source implementations (e.g. how boardgame.io, OpenSpiel, VASSAL-like systems, or
COIN/CDG digital adaptations model N seats, large maps, hidden information, and outcome
explanation), academic work on imperfect-information and multiplayer game representation,
poker hand-evaluation and showdown/side-pot resolution, deterministic multiplayer simulation, and
large-state board-game engines. **Cite sources for every external claim that shapes a decision**,
and recommend which belong in `docs/SOURCES.md`.

---

## 6. Doctrine & constraints (honor these; flag, never silently violate)

- `docs/FOUNDATIONS.md` is the constitution. Every product-behavior recommendation must satisfy
  its universal invariants and clear its stop conditions. A genuine divergence requires an
  accepted ADR that **explicitly names and supersedes** the affected principle — never design
  against it silently. Where the next phase needs such an ADR, **outline it** (do not declare it
  accepted) and say which principle it touches.
- **Authority order holds**: foundation docs govern area docs govern specs govern tickets. If a
  recommendation would put execution ahead of architecture or foundation, the recommendation is
  wrong — escalate it as a doc change instead.
- **`engine-core` stays generic and noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  `seat`-mechanic, etc. Typed mechanic nouns belong in `games/*` first; shared helpers enter
  `game-stdlib` only through the mechanic-atlas promotion process. Any "generic N-seat helper" or
  "graph-map helper" proposal must route through that boundary, not into the kernel.
- **TypeScript never decides legality.** Legal actions, validation, effects, views, outcome
  rationale, and bot decisions all come from Rust/WASM. Multi-seat panels, turn-order display, and
  showdown rendering are presentation only.
- **No YAML and no DSL without an accepted ADR.** Static data stays typed content / parameters /
  metadata — never selectors, conditions, or triggers. A larger surface is not a license for a
  map/scenario DSL; if you believe scale finally justifies one, that is an explicit ADR proposal,
  not a default.
- **Determinism is law**: replay, hashes, RNG, serialization order, and traces stay deterministic
  (or are explicitly, documentedly migrated). N seats, larger decks, and bigger maps must not
  introduce nondeterminism; the trace schema's `seats` handling must be addressed explicitly.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, or replay exports — and this must hold with **3+ players**, where one seat must
  not infer another's private cards from public betting alone. Treat N-player no-leak as a
  first-class proof obligation, consistent with `docs/adr/0004`.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2. Multi-opponent poker bots
  must be policy/belief-based within these limits; if you think multiplayer imperfect-information
  pressures this rule, flag it as an ADR question rather than assuming relief.
- **Never weaken tests to get green**; follow the failing-test protocol. Capability work is
  decomposed into bounded tasks (per `AGENT-DISCIPLINE.md`), never a single "implement
  multiplayer" task.
- **Promotion discipline**: a third repeated mechanic shape must resolve through the
  primitive-pressure ledger before proceeding, and promotions back-port to prior games or record
  explicit debt. Your ladder must say where scaling will trip these hard gates.

---

## 7. Deliverable specification

Produce **two separate downloadable markdown documents**. Both are **new advisory documents** —
neither replaces or overwrites any repository file. State, at the top of each, that it is advisory
input and that final doc/spec wording remains the user's.

### Doc 1 — `foundation-doc-realignment.md`

A per-document realignment analysis of **all of `docs/**` and `templates/**`** for the new phase.
For each document that needs change, give a concrete, detailed recommendation — **remove**,
**merge**, **correct**, or **add** — with:

- the exact document (and section/heading) affected;
- what is wrong or missing *for a 3+ player, larger-surface phase* (e.g. singular "active seat"
  assumptions, missing N-player no-leak test taxonomy, missing showdown/per-player outcome
  breakdown fields, missing topology-scale guidance, template fields baked for two seats);
- the specific change in enough detail that the user can write the final wording themselves
  (you describe the change and its intent; **you do not finalize the prose**);
- severity/priority and any cross-references in other docs/templates the change would invalidate;
- whether the change requires an **ADR** (per FOUNDATIONS's triggers) — if so, **outline** the ADR
  (problem, decision, affected sections, consequences) using `docs/adr/ADR-TEMPLATE.md`, labeled
  as a *proposed outline*, not accepted.

Cover **new documents to add** as well as edits/merges/removals (e.g., if a dedicated multi-seat
or surface/topology contract, or an N-player no-leak testing addendum, is warranted, propose it
with a purpose statement and where it sits in the authority order). Explicitly state when a
document needs **no change** rather than omitting it.

### Doc 2 — `public-game-ladder-and-implementation-order.md`

The researched next-phase plan:

- **Texas Hold'Em proper** as the first committed rung: your recommended feature scope (player
  range, betting-street/blind model, hand evaluation, side-pot/all-in stance) with justification —
  **the showdown breakdown of why each player won/lost is required and in scope** — plus the
  IP/naming/presentation caveat from `IP-POLICY.md` and how it frames as a delta from `poker_lite`.
- **Engine/app capability gaps** the phase must close, each tied to the repo evidence: N-seat
  generalization (the per-game 2-seat enum + setup-rejection pattern and the `simulate` reporting),
  larger-surface / graph-topology support and where it trips the mechanic atlas, **N-player
  hidden-information no-leak**, multi-seat view projection across the WASM boundary, multi-seat UI
  (seat panels, turn order) and the outcome/showdown surface, trace-schema `seats` handling, and
  multi-opponent bot strategy within the no-MCTS/ML limits.
- **The researched game ladder**: concrete, public-domain, IP-safe candidate games in a sequenced
  bridge from Hold'Em toward your **recommended ambition ceiling** (which you set and justify),
  each rung naming the capability it proves and the mechanic-atlas pressure it creates.
- **A recommended phased implementation-order table** the user can adapt directly into a new
  `specs/README.md` (which they will write after archiving the current one date-suffixed),
  consistent with the spec format in `specs/README.md` and the gate/sequencing conventions in
  `ROADMAP.md`. **Gate P (private monster game) appears last.** Make clear it is a recommendation
  to adapt, not a finished index.

> Produce the deliverables directly as downloadable markdown documents. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run before returning)

- [ ] Both deliverables exist as downloadable markdown, named exactly `foundation-doc-realignment.md`
      and `public-game-ladder-and-implementation-order.md`, each labeled advisory.
- [ ] Doc 1 covers **every** document in `docs/**` and `templates/**` (change or explicit
      "no change"), with section-level detail and severities, and outlines (not finalizes) any ADR.
- [ ] Doc 2 commits Texas Hold'Em first with a justified scope, includes the **required** showdown
      "why each player won/lost" breakdown, sets and justifies an ambition ceiling, recommends a
      concrete IP-safe ladder, and ends with a phased order table that places **Gate P last**.
- [ ] No recommendation weakens an upstream foundation principle or silently amends an accepted ADR
      (esp. `0004` hidden-info, the no-DSL/YAML rule, the noun-free kernel, the no-MCTS/ML bot
      rule, and determinism); any genuine divergence is raised as a *proposed* ADR outline.
- [ ] N-player hidden-information no-leak is treated as a first-class obligation for 3+ seats.
- [ ] Every external claim that shapes a decision is **cited**, with `SOURCES.md` additions noted.
- [ ] The three delegated sub-decisions (ambition ceiling, Hold'Em scope, ladder) are each
      answered with a recommended default **and** justification — none punted back to the user.
- [ ] Commit `a97625c` contains every file named in §2 (it does — the manifest reflects that tree).
