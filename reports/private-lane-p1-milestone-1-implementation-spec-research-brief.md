# Private Lane P1 — Milestone 1 implementation-spec research brief

You are an external deep researcher (ChatGPT-Pro). Produce the deliverable in
§7 **directly**. The requirements below are final — **do not interview, do not
ask clarifying questions**. The interview already happened; this brief is the
result.

---

## 0. Your inputs (read this first)

Your full input set is exactly:

1. **This prompt.**
2. **The uploaded repository manifest** `manifest_2026-06-28_a0117ec.txt` — the
   path inventory of the `joeloverbeck/rulepath` repository at one exact commit.
3. **Two uploaded source PDFs**, provided by the user out of band: a **rules /
   living-rules rulebook PDF** and a **playbook PDF** for *the first private
   licensed game* (a GMT COIN-series title). These are **private licensed IP**.
   They are your authoritative rules source for the game's mechanics, deck, and
   scenario.

The manifest **deliberately excludes** the two PDFs and any private-game
material. That is **intentional, not a gap**: the subject game is private
licensed IP that is kept out of the public repository by design (the PDFs sit
untracked on the author's disk and are not committed). Treat the PDFs as
in-scope source material delivered separately from the repo.

This brief itself is a **public-repository artifact** (it lives in the tracked
`reports/` tree), so it stays **opaque**: it never names the licensed title and
never names the PDF filenames. Your **deliverable**, by contrast, is destined
for a **private repository**, so it MAY name the title and describe the game's
mechanics — subject to the IP discipline in §9.

Fetch every repository file you read from this exact commit:

```
https://raw.githubusercontent.com/joeloverbeck/rulepath/a0117ec6097c1b980bbc0f0c3b6bcbc864deb4e1/<manifest path>
```

The manifest reflects that commit's tree. If any other report or doc you
encounter cites a different "commit of record," ignore that string and use the
commit above.

---

## 1. Context

The uploaded manifest is the path inventory of the `joeloverbeck/rulepath`
repository — a **Rust-first, rule-enforcing, replayable, testable card/board-game
platform where Rust owns all behavior and TypeScript/React present only**. The
foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` →
`ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier documents
govern later ones, and accepted ADRs supersede them only by explicitly naming
the affected sections. Fetch every file from commit `a0117ec` — the manifest
reflects that tree.

The repository has just completed a non-feature **doctrine/law/template
readiness pass** (spec `archive/specs/private-lane-foundation-readiness.md`,
tracker unit `PLP1-RDY`, status `Done`) that **opened a sanctioned private game
lane** running in parallel with the unfinished public ladder. That pass authored
and accepted three ADRs — **0010** (sanctioned parallel private-game lane),
**0011** (constrained typed Rust event-card mechanism), and **0012** (private
repository / CI federation / catalog overlay) — and amended the constitution,
IP policy, boundary docs, area docs, and templates accordingly.

This brief commissions the **next** step: the **implementation spec for Private
Lane P1, milestone 1** — the first private licensed game. The readiness work is
**done**; do not re-propose it. Build on it.

---

## 2. Read in full (authority order)

Read these repository files in full, in this order, before producing. Each is
load-bearing for *this* target.

**Constitution & decisions**

- `docs/README.md` — authority order and the layering rule; the ADR status index (0010/0011/0012 are `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: priority order, **§1.1 sanctioned private-game lane (timing carve-out)**, §10 IP conservatism, §11 universal invariants, §12 stop conditions (incl. the private-lane stops), §13 ADR triggers. Every spec decision must satisfy these.
- `docs/adr/0010-sanctioned-parallel-private-game-lane.md` — authorizes the parallel private lane (timing only); defines the non-contamination rule the spec runs under.
- `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md` — the *only* sanctioned shape for event cards: typed inert content + Rust behavior; bans YAML/DSL/untyped effect rows. The spec's entire event-deck design must conform.
- `docs/adr/0012-private-repository-ci-catalog-overlay.md` — the default private-repo/build/catalog/CI architecture the milestone lives in; the rejected alternatives you must not silently revive.

**Boundary, IP, architecture**

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — **§10A typed Rust card-effect registries**: the inert-content-vs-Rust-behavior line the event deck must respect; `engine-core` stays noun-free.
- `docs/IP-POLICY.md` — **§9 / §9A sanctioned lane / §9B default private repo**: the public no-leak checklist, opaque-placeholder rule, and "if it ships to an unauthorized browser, it has shipped."
- `docs/ARCHITECTURE.md` — workspace shape, action/view/effect/replay/determinism model, and **§11A the sanctioned private overlay lane** + large-action-tree guidance.

**Official-game contract, mechanic atlas, scaffolding**

- `docs/OFFICIAL-GAME-CONTRACT.md` — **§1A completion profiles** (`private-milestone-1-rule-complete`, `private-release-candidate`, `public-release-candidate`), the **P1-M1 capability/non-goals note**, and the **required private-spec field set** the deliverable must carry.
- `docs/MECHANIC-ATLAS.md` — **§2 private-stress categories** (card-driven initiative/eligibility, asymmetric faction menus, operation/special-activity coupling, propaganda upkeep, conditional event branches, persistent/temporary effects, faction victory tracks) and **§10A promotion-debt register** (currently *None* — confirm and document, don't invent debt).
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the reuse-first audit the spec owes (which existing behavior-free scaffolding to reuse, what to register, what prior-game refactors, if any).

**Multi-seat, bots, client, UI**

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — **§6A asymmetric-faction + 5-viewer no-leak floor** (public observer + 4 faction viewers; pairwise redaction across every surface). The milestone's no-leak proof floor.
- `docs/AI-BOTS.md` — **§4B private asymmetric sourcing + no-flowchart rule**, and Level-0 deferral language; the bot posture for M1.
- `docs/WASM-CLIENT-BOUNDARY.md` — private catalog/overlay semantics and the public-cleanliness boundary the (deferred-or-included) web overlay must honor.
- `docs/UI-INTERACTION.md` — legal-only interaction, effect-driven animation, accessibility, and the private web-overlay/large-asymmetric-UI guidance (load-bearing only if you recommend including the overlay in M1).

**Testing, evidence, trace, sources, agent discipline**

- `docs/TESTING-REPLAY-BENCHMARKING.md` — **§8 private large-game coverage**: the proof plan (seat counts, largest fixtures, 5-viewer matrix, event-deck/large-action-tree/upkeep/terminal traces, native replay/hash + browser smoke, benchmark targets).
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — `visibility_class = private-source` evidence profiles and the receipt fields the spec's evidence plan must use.
- `docs/TRACE-SCHEMA-v1.md` — the trace/replay-fixture schema law (large-event clarification; **no migration** is authorized — design within it).
- `docs/SOURCES.md` — the recorded external prior art (Rally the Troops/GMT, VASSAL, boardgame.io, OpenSpiel, GitHub reusable workflows, Cargo tooling) with each Rulepath lesson and non-adoption; the starting point for your online research.
- `docs/AGENT-DISCIPLINE.md` — bounded-task / forbidden-change / failing-test law the Work-breakdown items must be decomposable under.

**Specs & templates**

- `specs/README.md` — the **Private lane tracker** (`P1-M1` row, `Doctrine pending`) and the canonical **Spec format** the deliverable follows; the workflow that admits this unit.
- `archive/specs/private-lane-foundation-readiness.md` — the **implemented readiness baseline**. Its WB items, ADR decisions, and seeded-forward deferrals are *done* (the private repo, the public seam *extraction*, and this implementation spec were explicitly seeded forward). Build on it; do **not** re-recommend it as if missing.
- `templates/**` — the full template set the spec and its downstream tickets are built against. Read at least: `templates/README.md`, `templates/GAME-EVENT-COVERAGE.md` (**new** — the event-deck coverage matrix), `templates/PRIVATE-RELEASE-CHECKLIST.md` (**new**), `templates/AGENT-TASK.md` (private-source fields), `templates/GAME-AI.md` + `templates/COMPETENT-PLAYER.md` (per-faction policies + AI deferral), `templates/GAME-IMPLEMENTATION-ADMISSION.md` (private-lane ADR gates), `templates/GAME-EVIDENCE.md` (private completion-profile rows), `templates/GAME-RULES.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-RULE-COVERAGE.md`, `templates/GAME-SOURCES.md`, `templates/GAME-UI.md`, `templates/GAME-BENCHMARKS.md`, `templates/GAME-HOW-TO-PLAY.md`, `templates/PRIMITIVE-PRESSURE-LEDGER.md`.

### Code seams to inspect directly (inspect, not read-fully)

These are **not** pasted and **not** part of the read-in-full set above. Read
them *in the repo* to learn the shapes the private game crate will mirror:

- `crates/engine-core` — the generic, **noun-free** contract kernel the private game extends without contaminating (no `faction`/`card`/`deck`/`operation` nouns may enter it).
- One representative `games/*` module **with the most events / largest action tree** (e.g. the event-complexity or asymmetric-area-control games) plus its `docs/` and golden traces — the canonical shape for rules/views/effects/traces/evidence the spec should mirror at COIN scale.
- `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` — the evidence harnesses the milestone's acceptance evidence is gathered through (and which a Level-0 random-legal driver must feed).
- `apps/web` catalog + renderer registration seam — the public-catalog-only boundary and the generic seam shape any private renderer overlay would register behind.

---

## 3. Settled intentions (these are decided — do not reopen)

1. **The target is fixed.** This is **Private Lane P1, milestone 1**. It is admitted because `PLP1-RDY` is `Done`, ADRs **0010/0011/0012** are `Accepted`, and `MECHANIC-ATLAS §10A` promotion-debt is *None*. **Confirm-and-document** this admission with that evidence; do **not** re-open "what should we build next."
2. **The game** is *the first private licensed game* — the GMT COIN-series title identified by the two uploaded PDFs — handled as private licensed IP.
3. **Milestone-1 capability set (locked):** full **4-faction hotseat**; **all operations**; **all special activities**; **propaganda rounds**; **victory / terminal detection**; and **every event card in the deck resolved**. **One standard full-length scenario/setup only** — other scenarios, short/teaching setups, and campaign variants are later milestones. State these non-goals explicitly.
4. **Repository placement: a separate PRIVATE repository (ADR-0012)** that pins a public Rulepath commit and owns the private game crate(s), docs, fixtures, traces, e2e, renderer overlay, private catalog, private CI, and private WASM/web build. The deliverable spec lives **there**, not in the public `specs/` tree. The public repository gains **only generic, private-free extension seams** — never a workspace member, catalog row, CI manifest, or submodule/optional dependency that names the private game.
5. **Event cards use the constrained typed Rust event-card mechanism (ADR-0011 / boundary §10A).** Card identity, deck order, and inert display metadata MAY be typed static content in the private crate. **Every** condition, selector, trigger, rule override, target choice, legality hook, state transition, visibility decision, diagnostic, and semantic effect is Rust behavior (functions / match arms / traits). **No** YAML, scripts, untyped JSON/TOML/RON effect rows, table-row selectors, or declarative effect language. Design the full event deck under this constraint.
6. **Bots in M1:** **no designed faction AI**, and **no publisher flowchart / priority chart / solitaire-bot procedure** copied into bot policy, tests, or strategy docs — ever (AI-BOTS §4B). M1 **includes a Level-0 random-legal move driver** (strategy-free, uniform over the legal-action API) **solely to drive** simulate / replay-check / property / determinism evidence for a monster-scale game. Designed per-faction AIs are an explicit **later** milestone; record them as deferred with the closure gate named (`private-release-candidate`).
7. **Surface scope is a bounded delegation to you.** Recommend whether milestone 1 includes the **private web/WASM renderer overlay** (browser-playable hotseat) or is **Rust rules core + simulator/CLI hotseat** with the overlay deferred. **Required default: Rust-core-first (overlay deferred)** — choose it unless the rules clearly force otherwise, and justify the choice. Either way the milestone capability set in (3) is unchanged. This is the *only* open design choice; everything else here is locked.
8. **Completion-profile target:** `private-milestone-1-rule-complete` (OFFICIAL-GAME-CONTRACT §1A). The spec must carry the §1A **required private-spec field set** (opaque lane id, pinned public commit, ADR 0010/0011/0012 gates, completion-profile target, private-repo boundary, private-source receipt plan, 5-viewer no-leak matrix, event/card coverage owned only by typed Rust, bot plan/deferral, public back-leak sweep).
9. **No-leak floor: the 5-viewer matrix** (public observer + 4 faction viewers) per MULTI-SEAT §6A, with pairwise redaction across view payloads, action trees, previews, diagnostics, effects, bot explanations, candidate rankings, replay exports, DOM/test IDs, logs, storage, and fixtures. Public-observer safety holds even though the build is never publicly released (private screenshots/replays can still leak).
10. **Determinism & trace discipline hold.** Setup, legal-action generation, transitions, view projection, replay, hashes, fixtures, and serialization order are deterministic and Rust-owned, designed within the existing `TRACE-SCHEMA-v1` (no public trace/hash migration is authorized by this work).

`assumption:` the deliverable is **one** implementation spec (not also tickets). You decompose it inside the private repository with the project's private analog of the `/reassess-spec` → `/spec-to-tickets` flow; the spec must therefore be *reassessable and decomposable* (bounded Work-breakdown items, explicit exit criteria).

`assumption:` M1 targets `private-milestone-1-rule-complete`, **not** `private-release-candidate`. Closing the Level-0 bot deferral, designed faction AIs, and the public/private release sweep are later-milestone gates the spec names but does not satisfy.

---

## 4. The task

Author **one private-repository implementation spec** that turns Private Lane P1
milestone 1 into a concrete, bounded, reviewable plan: a deterministic, Rust-owned
implementation of the first private licensed game covering full 4-faction hotseat
play — all operations, all special activities, propaganda rounds, victory/terminal
detection, and **every event card resolved** — for a single standard full-length
scenario, under the sanctioned private lane (ADRs 0010/0011/0012), with a Level-0
random-legal driver for evidence and **no designed AI**. This is a **new spec**
(the private-lane analog of a roadmap-gate spec), grounded in the uploaded rules
PDFs for game facts and in the foundation docs for every boundary, determinism,
no-leak, IP, and evidence obligation.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above, and
read the two uploaded PDFs thoroughly for the rules, the full event deck, the
operations/special-activities matrix, the propaganda sequence, and the standard
scenario setup. Research online as deeply as needed — similar implementations,
research papers, and prior art — wherever it sharpens the deliverable. Cite
sources for any external claim that shapes a decision.

Genuinely-relevant external angles (start from `docs/SOURCES.md`, which already
records the Rulepath lesson and non-adoption for several): COIN-series engine
structure and the card-driven "sequence of play" / eligibility model; open
implementations of card-driven wargames (e.g. Rally the Troops) and what their
rule/UI boundary teaches; VASSAL-style module boundaries (engine without rules
authority); boardgame.io and OpenSpiel **public/private player-view and
information-state separation** as a no-leak design reference (but **not** their
RL/search approach — public v1/v2 bots exclude MCTS/ISMCTS/Monte Carlo/ML/RL,
and M1 has no designed AI at all); deterministic event-resolution and
large-action-tree handling; GitHub reusable-workflow + pinned-commit CI
federation and Cargo workspace/dependency patterns for a private repo that
pins a public commit.

Let research **sharpen** the locked scope; it must not **expand** it. Do not
adopt any external framework that would put rules in TypeScript, encode behavior
in data/YAML/DSL, add game nouns to `engine-core`, or copy a publisher flowchart.

---

## 6. Doctrine & constraints (honor all)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its §11 universal invariants and clear its §12 stop conditions (including the private-lane stops: no private content in public surfaces; no private monster-game shaping public architecture; no implementation before the authorizing ADRs are accepted — they are). A genuine divergence would require a *new* accepted ADR first, never designing against the constitution silently.
- **Authority order:** foundation docs govern area docs govern specs govern tickets. If the spec ever conflicts with architecture or the constitution, the spec is wrong.
- `engine-core` stays generic and **noun-free** — no `faction`, `card`, `deck`, `operation`, `eligibility`, `board`, `grid`, `hand`, etc. COIN nouns live in the private game crate; shared helpers only via the mechanic atlas / scaffolding register with public-safe evidence (private pressure alone never earns a public helper).
- **TypeScript never decides legality.** Legal actions, validation, effects, views, visibility, victory, and any bot decisions all come from Rust/WASM.
- **No YAML and no DSL.** Static data is typed content/parameters/metadata only — never selectors, conditions, triggers, overrides, or effect formulas (ADR-0011 is the exact boundary).
- **Determinism:** replay, hashes, RNG, serialization order, and traces stay deterministic, designed within `TRACE-SCHEMA-v1`; no public trace/hash migration.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot explanations, candidate rankings, or replay exports — proven against the 5-viewer matrix.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots**, and **no publisher flowchart/priority chart** in bot policy/tests/strategy docs. M1 ships only a strategy-free Level-0 random-legal driver.
- **Never weaken tests to get green** (AGENT-DISCIPLINE §4).
- **Deliver complete files / coherent complete sections, not diffs** (this is a spec, so: a complete spec document).

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **Filename:** `private-lane-p1-milestone-1-implementation-spec.md` (opaque stem,
  mirroring this brief's slug).
- **Shape:** a **new spec** that lands in the **private repository** (its spec
  directory), **not** the public `specs/` tree. It is the private-lane analog of
  a roadmap-gate spec and must be directly reassessable/decomposable into private
  tickets.
- **Structure:** follow the `specs/README.md` "Spec format" — Header (Spec ID,
  unit `P1-M1`, status, authority order) → Objective → Scope (in / out / not
  allowed, carrying the §3 non-goals) → Deliverables (the private game crate
  tree, event registry, views, traces, evidence, and — if you recommend it — the
  renderer overlay) → Work breakdown (bounded, dependency-ordered candidate
  AGENT-TASKs) → Exit criteria (row-for-row to the M1 capability set and the
  `private-milestone-1-rule-complete` profile) → Acceptance evidence
  (unit/rule/golden/property/sim/replay/serialization + determinism/hash + the
  5-viewer no-leak matrix + native replay & browser smoke as applicable +
  benchmarks per TESTING §8) → FOUNDATIONS & boundary alignment → Forbidden
  changes → Documentation updates (private docs/templates) → Sequencing →
  Assumptions. **Add the OFFICIAL-GAME-CONTRACT §1A required private-spec field
  set**, a **GAME-EVENT-COVERAGE-shaped event-deck coverage plan** (every card,
  its branch kind, its Rust owner, its visibility/replay impact, its private
  source receipt id), the **mechanical-scaffolding reuse-first audit** (reuse /
  register / prior-game-refactor disposition, with `not applicable` rows
  justified), and a **PRIVATE-RELEASE-CHECKLIST-aware** statement of what M1 does
  and does **not** satisfy.
- **Surface-scope recommendation (§3.7):** state your Rust-core-vs-overlay
  recommendation with the required Rust-core-first default and a short
  justification, and write the Scope/Deliverables/Evidence accordingly.

Locked / no-questions instruction:

> Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final.
> If a genuine contradiction makes a requirement impossible, state it in the
> deliverable and proceed with the most faithful interpretation.

---

## 8. IP discipline (private licensed subject)

- The **deliverable** (private repo) MAY name the title and describe mechanics in
  **original Rulepath prose**. It MUST **not** reproduce licensed expression:
  no copied rulebook prose, card text, examples, diagrams, tables, art, icons,
  trade dress, or — critically — **publisher flowchart / non-player / priority-chart
  text**. Summarize every rule and event in your own words; reference the PDFs by
  **private-source receipt id** (per `EVIDENCE-FIXTURE-CONTRACT` /
  `GAME-SOURCES`), not by quoting them.
- Keep the subject material **out of the public repository and every public
  surface**. Nothing in this brief, the manifest, public source, public docs,
  public CI, public traces, public bundles, or public WASM/JS may name the title,
  a card/event/faction/scenario id, a private fixture/e2e filename, or a catalog
  string. The private game lives only in the private repository and private build
  artifacts.
- This subject material is **deliberately excluded** from the uploaded manifest
  because it is private IP that does not belong in the public repo — its absence
  is by design.
- Governing law: `docs/IP-POLICY.md` (esp. §9/§9A/§9B and the public no-leak
  checklist) and `docs/FOUNDATIONS.md` §10.

---

## 9. Self-check (run before returning)

- The deliverable is exactly the one spec named in §7, with the §3 capability set
  and non-goals, and is structured per `specs/README.md` Spec format **plus** the
  OFFICIAL-GAME-CONTRACT §1A required private-spec field set.
- The admission is **confirmed and documented** (PLP1-RDY `Done`; ADRs
  0010/0011/0012 `Accepted`; `MECHANIC-ATLAS §10A` debt *None*) — not reopened —
  and the readiness work is treated as the done baseline, never re-proposed.
- The full event deck is designed under the **ADR-0011 typed-Rust** boundary —
  no YAML/DSL/untyped effect rows; `engine-core` gains no COIN noun.
- The plan lands in a **separate private repository** pinning a public commit; no
  public workspace member / catalog row / CI manifest / submodule names the game.
- Bots: **Level-0 random-legal driver only**; designed AI deferred with a named
  closure gate; **no publisher flowchart** anywhere.
- The **5-viewer no-leak matrix** is part of the acceptance evidence; determinism
  and `TRACE-SCHEMA-v1` are respected with no public trace/hash migration.
- The surface-scope recommendation states the Rust-core-first default and a
  justification.
- **No licensed expression** is reproduced; the title and all private ids stay out
  of every public surface; PDFs are cited by receipt id.
- Every external claim that shaped a decision is **cited**.
- The §1 fetch-baseline commit `a0117ec` contains every repository file named in
  §2 (it does — confirmed against the manifest).
