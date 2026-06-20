# Research brief — Gate 15.1: River Ledger all-in / side pots (spec authoring)

**For:** ChatGPT-Pro deep-research session (Session 2). You are locked and self-contained:
the requirements below are final. Read the uploaded manifest plus this prompt, explore the
repository and the web as deeply as needed, and **produce the deliverable directly**. Do not
interview or ask clarifying questions — the determination and scope were already settled.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-20_cd158ac.txt`) is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file from commit `cd158ac` (verified repo HEAD; clean working tree). The
uploaded manifest reflects exactly that tree.** If any repository file you read cites a
different "commit of record," that is that document's own historical baseline — note the
divergence and use `cd158ac`.

This brief commissions **one downstream spec**: the next roadmap-gate implementation spec for
the Rulepath public scaling phase. The "what is next" determination is already made (see §3);
your job is to author the spec that implements it, grounded in the repo's law and sharpened by
deep external research into poker side-pot/all-in accounting.

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing. The user explicitly requires the entire
`docs/**` tree and the entire `templates/**` tree to be read; reasons below are why each is
load-bearing **for a Gate 15.1 all-in/side-pot game-extension spec**.

**Tier 1 — Foundation & area law (`docs/**`, authority order):**

- `docs/README.md` — the authority order and the layering rule; tells you which doc wins on conflict.
- `docs/FOUNDATIONS.md` — the constitution: priority order, **§11 universal invariants**, **§12 stop conditions**, **§13 ADR triggers**; every line of the spec must satisfy these.
- `docs/ARCHITECTURE.md` — workspace/crate layout (`engine-core`, `game-stdlib`, `ai-core`, `wasm-api`, `games/*`, `tools/*`, `apps/web`); where all-in/side-pot logic is allowed to live.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — engine-core noun-free / game-stdlib / static-data boundary; side-pot allocation and all-in caps must be typed Rust behavior, never static-data formulas.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — the N-seat setup/projection/no-leak contract; side-pot **eligibility** and per-seat stacks are N-seat public surfaces governed here.
- `docs/OFFICIAL-GAME-CONTRACT.md` — the per-official-game admission/closeout contract: which per-game docs must exist, catalog/smoke reconciliation, public-release gating (§10/§12 referenced by `specs/README.md`).
- `docs/AI-BOTS.md` — bot policy law + hidden-information safety; bots must produce legal actions when a stack < the amount owed (call-all-in or fold), using only authorized seat views; no public MCTS/ISMCTS/Monte Carlo/ML/RL.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — required test classes, golden-trace discipline, replay/hash determinism, the N-player no-leak harness, and benchmark expectations the spec's acceptance evidence must map to.
- `docs/IP-POLICY.md` — public/IP-safe, no-casino-trade-dress boundary; side-pot presentation and naming must stay neutral and original.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law. **Load-bearing:** §10 River Ledger entry explicitly says *"Reopen for Gate 15.1 side-pot/all-in work"*; §10A open promotion-debt register is **empty** (no interlock blocks this gate); §9A arms the side-pot pressure as *"public accounting/allocation pressure — keep local unless the ledger proves a narrow behavior-free helper."*
- `docs/UI-INTERACTION.md` — UI interaction/presentation law; the spec's UI surface (per-pot breakdown, all-in indicators, eligibility, showdown allocation) must conform without TypeScript legality.
- `docs/WASM-CLIENT-BOUNDARY.md` — the Rust↔browser JSON bridge contract; new side-pot/stack fields cross this boundary as projected state only.
- `docs/TRACE-SCHEMA-v1.md` — the trace schema; new all-in/side-pot semantic effects and golden traces must fit it (or justify an explicit, migrated change).
- `docs/SOURCES.md` — repo-level source/IP notes feeding `GAME-SOURCES.md`.
- `docs/AGENT-DISCIPLINE.md` — operational law for coding agents: bounded tasks, forbidden changes, the failing-test protocol; the spec's work breakdown decomposes into bounded AGENT-TASKs that obey this.
- `docs/ROADMAP.md` — the prescriptive ladder (law). **§15 "Gate 15.1" is the exact scope this spec implements**; also confirms public-scaling-phase admission and Gate P tail.
- `docs/archival-workflow.md` — how completed specs are archived; relevant to where this spec lives and moves.
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR admitting the public scaling phase (Gate 15+); the spec builds on it and must not silently amend it.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — hidden-information replay/export taxonomy; side-pot eligibility/stack data must respect it across replay exports.
- `docs/adr/0001-stage-1-random-playout-budget.md`, `0002-ci-benchmark-gating-lanes.md`, `0003-ci-calibrated-benchmark-thresholds.md`, `0005-variance-aware-ci-benchmark-floors.md` — the benchmark/CI ADR set the acceptance evidence (simulate throughput, benchmark lanes/floors) must honor.
- `docs/adr/0006-blackjack-lite-roadmap-placement.md` — context on deferred card-game placement; confirms no casino-game scope creep.
- `docs/adr/ADR-TEMPLATE.md` — the ADR shape (in case any 15.1 decision crosses an ADR trigger; do not assume one is needed — most likely the atlas decision is "keep game-local").

**Tier 2 — Planning artifacts:**

- `specs/README.md` — the living spec index and 12-section spec format. **Load-bearing:** Order 6 = Gate 15.1, the lowest non-`Done` unit; this is the determination evidence and the structure your output must follow.
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md` — **the spec Gate 15.1 extends.** Mirror its section structure. It explicitly defers all-in/side-pots to 15.1 (e.g. "Side pots | Out of scope and deferred to Gate 15.1"; "Gate 15.1 side pots remain a successor, not hidden Gate 15 scope") and sketches the 15.1 successor scope near its end. This is the **shipped baseline you build on, not rebuild**.
- `archive/specs/infra-a-d-n-seat-public-infrastructure.md` — the N-seat setup/catalog metadata, seat-keyed simulator summaries, multi-seat shell frame, and pairwise no-leak harness that the chip-stack model and side-pot surfaces plug into.

**Tier 3 — Templates (`templates/**`, all read):** these are the per-game artifacts the spec's
"Documentation updates required" section must enumerate as work items for the all-in/side-pot delta.

- `templates/README.md` — template authority and the universal completion rule.
- `templates/AGENT-TASK.md` — the bounded execution packet the work breakdown decomposes into.
- `templates/GAME-RULES.md` — original rules summary with stable rule IDs (new `RL-*` all-in/side-pot rule family).
- `templates/GAME-RULE-COVERAGE.md` — rule→implementation→test→trace traceability matrix (new rows + UI rows).
- `templates/GAME-MECHANICS.md` — game-local mechanic inventory + atlas pressure (resource/accounting row gains stacks/side-pots).
- `templates/GAME-HOW-TO-PLAY.md` — player-facing original prose (explain all-in, eligibility, side pots).
- `templates/GAME-UI.md` — UI plan and Rust/TS boundary (per-pot breakdown, all-in indicator, eligibility, allocation).
- `templates/GAME-AI.md` — per-game bot registry/status (bot inputs/policy under all-in).
- `templates/GAME-BENCHMARKS.md` — performance plan/report (allocation hot path, larger trace set).
- `templates/GAME-SOURCES.md` — source-use/IP notes for the rules basis.
- `templates/GAME-IMPLEMENTATION-ADMISSION.md` — the pre-coding admission receipt (supersedes the base's "all-in/side-pots out of scope" statement).
- `templates/PRIMITIVE-PRESSURE-LEDGER.md` — the atlas hard-gate decision record (the side-pot/all-in keep-local-or-promote decision).
- `templates/COMPETENT-PLAYER.md` — human strategy analysis feeding the L2 bot (stack-aware/all-in heuristics).
- `templates/BOT-STRATEGY-EVIDENCE-PACK.md` — authored-policy design input for the L2 bot.
- `templates/PUBLIC-RELEASE-CHECKLIST.md` — final pre-public gate.

**Code seams to inspect directly (read in the repo; *inspect, not read-fully*; not part of the
read-in-full set above):**

- `games/river_ledger/src/{state,pot,actions,rules,showdown,visibility,effects,bots,setup,ids,cards,evaluator,ui}.rs` — the single-pot contribution ledger, fixed-unit action generation, single-pot allocation/remainder, per-seat projections, semantic effects, and bots that Gate 15.1 extends. Confirm in code (do not trust this brief's summary) that there is currently **no per-seat chip stack, no insufficient-chip branch, and no multi-pot data structure**.
- `games/river_ledger/tests/` and its golden traces — the correctness oracle and the no-leak/replay/property/serialization test shapes the spec must extend (new all-in/side-pot golden traces).
- `games/river_ledger/docs/` — the already-filled per-game docs (RULES, RULE-COVERAGE, MECHANICS, HOW-TO-PLAY, UI, AI, SOURCES, PRIMITIVE-PRESSURE-LEDGER, IMPLEMENTATION-ADMISSION, COMPETENT-PLAYER, BENCHMARKS) the spec must update — read these to frame the delta, not a cold start.
- `crates/wasm-api/src/games/river*.rs` — the bridge that must marshal new stack/side-pot projected fields.
- `apps/web` River Ledger renderer, catalog entry, and `e2e` smoke — the presentation/smoke surfaces the spec's UI + closeout work items name.

---

## 3. Settled intentions (these make you locked)

These decisions are final. Do not re-open them; do not ask about them.

1. **The determination is locked: the next spec is Gate 15.1 — River Ledger all-in / side
   pots.** Open your spec by **confirming and documenting** this with the in-repo evidence,
   not by re-deciding "what is next": `specs/README.md` Order 6 is the lowest non-`Done` unit;
   `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is empty (no interlock blocks the
   next gate); the atlas River Ledger entry explicitly says *"Reopen for Gate 15.1 side-pot/all-in
   work"*; `docs/ROADMAP.md` §15.1 defines the scope; ADR 0007 admits the phase. A spec that
   re-opens the determination violates this contract.

2. **Build on the shipped Gate 15 baseline — do not rebuild it.** Gate 15 is `Done`: a single
   growing pot, 3–6 seats, fixed-limit capped-raise betting, a per-seat contribution ledger, a
   7-card hand evaluator, button-ordered split/remainder allocation, Rust-authored showdown
   explanation, and N-player no-leak. Several River Ledger UX/correctness specs also shipped
   after it. **Do not re-propose any shipped base mechanic as if it were missing.** The spec is a
   pure delta: it adds the all-in/side-pot layer on top of that base. The base spec already
   names this successor scope — honor that hand-off.

3. **Stack model — bounded delegation (decide it, with justification).** All-in is only
   meaningful with finite per-seat chips, and the base game has **no chip-stack model** (bets are
   fixed units against a growing pot; nothing caps a seat by chips). The spec **must require
   introducing a per-seat chip-stack model**, and must present this as a decided sub-choice
   between exactly two enumerated options, choosing one with a written justification:
   - **(a) Equal fixed starting stacks for all seats** — *required default*: deterministic,
     classic, simplest golden traces; all-ins arise when a seat's stack < the fixed amount owed.
   - **(b) Configurable per-seat starting stacks via setup metadata** — richer; enables
     asymmetric short-stack scenarios and more side-pot golden traces; larger surface/test pressure.
   Pick one, justify it against repo doctrine (determinism, N-seat setup metadata in Infra A,
   test/no-leak cost), and write the spec around it. Everything else in the determination and
   scope stays locked; only this is handed to you.

4. **Scope is strict — Gate 15.1 only.** In scope: all-in legal/terminal states; all-in
   contribution caps; side-pot construction (layered contribution buckets); per-pot eligibility;
   per-pot allocation with split winners and deterministic remainder/odd-chip order; terminal &
   showdown explanation of **every public allocation** without revealing a viewer's still-hidden
   cards; bots that act legally under all-in; the per-seat stack model from intention 3; full
   test/trace/replay/no-leak/benchmark + doc coverage. **Out of scope / not allowed** (carry
   ROADMAP's lists): no-limit or pot-limit betting, real-money/casino features or trade dress,
   tournament/product mimicry, hidden card/deck/stack leakage, omniscient bots, public
   MCTS/ISMCTS/Monte Carlo/ML/RL, static-data behavior, accidental trace/hash migration, and any
   unrelated River Ledger polish creeping into this gate.

5. `assumption:` the gate **retains the fixed-limit capped-raise betting structure** — all-ins
   arise when a seat cannot cover the fixed amount owed, not from a switch to no-limit (the
   user can override this; ROADMAP §15.1 does not ask for a betting-structure change, and no-limit
   is in the base "Not allowed" list).

6. `assumption:` the atlas decision for the side-pot/all-in shape is expected to land as
   **keep game-local / no `game-stdlib` promotion** (per §9A "keep local unless the ledger
   proves a narrow behavior-free helper" and §10A empty debt). The spec must still **write the
   primitive-pressure ledger entry** that records this decision with its full field set; do not
   silently skip the atlas obligation. If your research surfaces genuinely repeated behavior-free
   allocation pressure, present the promote-vs-keep-local decision explicitly rather than
   defaulting.

---

## 4. The task

Author a single **new roadmap-gate implementation spec** — target type: **new-spec** — that
turns ROADMAP §15.1 into a concrete, reviewable, bounded plan for adding all-in and side-pot
support to the already-shipped River Ledger game, ready to be decomposed into `tickets/`
AGENT-TASK packets. The spec must be grounded in the foundation/area law (§2 Tier 1), mirror
the structure of the Gate 15 base spec and the `specs/README.md` 12-section format, frame the
work strictly as a delta on the shipped base (§3.2), resolve the stack-model sub-decision
(§3.3), and be sharpened by deep external research into canonical poker side-pot/all-in
accounting (§5). It must enumerate — but not perform — the per-game documentation updates the
delta requires, and map its exit criteria row-for-row to ROADMAP §15.1's exit list.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — especially the
`games/river_ledger/` source, tests, golden traces, and docs, so the spec's delta is grounded
in the real shipped surface and not in this brief's secondhand summary.

Research online as deeply as needed — similar implementations, research papers, and prior art —
wherever it sharpens the deliverable. **Cite sources for any external claim that shapes a
decision.** Priorities for the deep research:

- The **canonical side-pot allocation algorithm** (layered "contribution-cap" / side-pot-ladder
  construction): how to bucket per-seat total contributions into a main pot plus ordered side
  pots, and how eligibility for each pot is derived from who contributed at each cap layer.
- **All-in edge cases**: uneven simultaneous all-ins; a raise that is "all-in for less" than a
  full raise and whether it reopens the betting (and the standard rule that it does not, in
  limit play); dead/odd chips and deterministic odd-chip award order; multi-way ties spanning
  different pots; returning uncalled excess to a lone over-bettor; folded-but-contributed chips.
- **Determinism & explanation**: how reference engines order remainder/odd-chip distribution,
  and how to explain each pot's winners and amounts publicly without leaking a viewer's hidden
  cards — mapping cleanly onto Rulepath's Rust-authored showdown-explanation model.
- **Reference implementations** (e.g. reputable open-source poker engines / hand-history
  standards) for side-pot construction and all-in semantics, to validate the algorithm choice
  and surface edge cases worth a golden trace.

Convert what you learn into concrete spec content: named rules, a precise allocation algorithm
description, an enumerated edge-case list that becomes golden traces, and acceptance criteria —
not a literature survey.

---

## 6. Doctrine & constraints (honor these in the spec)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first, never designing against it silently. If
  the work hits a §13 ADR trigger (e.g. a replay/hash/visibility/data-policy/kernel-boundary
  change), the spec must say so and route it to ADR rather than absorbing it.
- **Authority order**: foundation docs govern area docs govern specs govern tickets; where a spec
  and a foundation doc disagree, the foundation doc wins. The spec must not redefine any
  foundation contract.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  `pot`, `seat`, etc.; all-in/side-pot/stack nouns live in `games/river_ledger`; shared helpers
  reach `game-stdlib` only through an accepted mechanic-atlas decision.
- **TypeScript never decides legality.** Legal all-in actions, eligibility, caps, allocation,
  views, and bot decisions all come from Rust/WASM; TS renders projected state only.
- **No YAML and no DSL.** Stack sizes, blind/bet units, and side-pot parameters are typed
  content/parameters/metadata — never selectors, conditions, or allocation logic in data.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic;
  any trace/hash change is explicit and migrated with a trace note, never accidental.
- **No hidden-information leaks** into browser payloads, DOM, storage, logs, effect logs, bot
  explanations, candidate rankings, or replay exports — including new stack/eligibility/side-pot
  fields. The pairwise no-leak harness (Infra D) must cover them.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots.** Bots use legal action APIs and authorized
  seat views only.
- **Never delete or weaken tests to get green** — the spec's acceptance evidence follows the
  AGENT-DISCIPLINE §4 failing-test protocol.
- The spec **delivers complete sections, not diffs**, and decomposes into bounded AGENT-TASKs;
  it does not itself perform ticket decomposition or write code.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-15-1-river-ledger-all-in-side-pots.md`** — **new file** (no existing file is
  replaced). It must follow the **canonical 12-section spec format** defined in
  `specs/README.md` ("Spec format": Header, Objective, Scope, Deliverables, Work breakdown, Exit
  criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes,
  Documentation updates required, Sequencing, Assumptions), mirroring the structure of
  `archive/specs/gate-15-river-ledger-texas-holdem-base.md`. Concretely it must include:
  - a **Header** (Spec ID, stage/gate = Gate 15.1, status `Not started` → to be `Planned` on
    acceptance, date, owner, authority order);
  - an **Objective** sourced from ROADMAP §15.1, opening with the locked determination + its
    evidence (§3.1);
  - **Scope** (in scope / out of scope / not allowed) carrying ROADMAP's lists and §3.4;
  - **Deliverables** grounded in ARCHITECTURE (which `games/river_ledger` modules/types/effects
    change, new stack/side-pot data, WASM bridge fields, web surfaces);
  - a **Work breakdown** of bounded, dependency-ordered items, each a candidate AGENT-TASK,
    including the stack-model decision (§3.3), the side-pot allocation algorithm, the
    primitive-pressure-ledger entry (§3.6), and the doc updates;
  - **Exit criteria** mapped row-for-row to ROADMAP §15.1's exit list;
  - **Acceptance evidence** (named test classes, the pairwise no-leak matrix, a golden-trace
    minimum set covering the researched edge cases, replay/hash/serialization checks, benchmark
    expectations, the per-game command suite from CLAUDE.md);
  - **FOUNDATIONS & boundary alignment** (principles engaged + the mechanic-atlas stance);
  - **Forbidden changes**, **Documentation updates required** (enumerating every `templates/**`
    -derived per-game doc plus `specs/README.md` status flip and the web-shell catalog README
    closeout per `specs/README.md` item 10 / OFFICIAL-GAME-CONTRACT §10/§12), **Sequencing**
    (predecessor Gate 15; successor Gate 16 Hearts), and **Assumptions** (one-line-correctable,
    carrying §3.5/§3.6).

The per-game documentation edits are **enumerated as work items inside the spec, not performed
in this deliverable**. Do not produce ticket files, code, or filled per-game docs.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run before returning)

- The spec **confirms-and-documents** Gate 15.1 with in-repo evidence and never re-opens "what
  is next" (§3.1).
- It is framed as a **pure delta** on the shipped Gate 15 base and re-proposes no shipped base
  mechanic as missing (§3.2).
- The **stack-model sub-decision is resolved** with one of the two enumerated options and a
  written justification (§3.3).
- Scope honors §3.4 (no no-limit/casino/scope creep) and the spec carries ROADMAP §15.1's
  "Not allowed" list.
- A **primitive-pressure-ledger entry is specified** with its full field set (§3.6); any
  promote-vs-keep-local call is explicit.
- The spec follows the **12-section format** and mirrors the Gate 15 base spec; exit criteria
  map row-for-row to ROADMAP §15.1.
- Every external claim that shaped a decision is **cited**; the side-pot algorithm and edge-case
  golden-trace set are concrete, not a literature survey.
- No proposed change weakens an upstream foundation doc or silently amends an accepted ADR;
  hidden-info, determinism, noun-free-kernel, and TS-no-legality constraints all hold.
- The deliverable set is **exactly** `specs/gate-15-1-river-ledger-all-in-side-pots.md` (new),
  per §7 — no extra files, no code, no tickets.
- The §1 fetch-baseline commit `cd158ac` contains every file named in the §2 read-in-full list
  (it does; the uploaded manifest is that tree).
