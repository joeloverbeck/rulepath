# Research Brief — Cuba Libre (Private COIN) Foundation-Readiness Change Plan

**For:** ChatGPT-Pro deep-research session (Session 2).
**From:** Rulepath maintainer, via a repo-grounded authoring session (Session 1).
**Status:** LOCKED. Do not interview, do not ask clarifying questions. The
requirements interview already happened. Produce the deliverable directly.

---

## 0. How to use this brief (read first)

You are producing **one downloadable markdown document**: a *change plan* that
tells the Rulepath maintainers exactly how to amend their repository's
foundation documents, templates, and spec index so the repo can host a complex
**private, licensed** game — **Cuba Libre**, a GMT *COIN*-series counter­insurgency
board game — as the project's **first private game**, started **now**.

This is **preparatory doctrine work, not implementation.** You are not designing
the game, not writing Rust, not writing the game's spec. You are deciding what
must change in `docs/**`, `templates/**`, `specs/README.md`, plus the
version-control / CI / app-catalog plumbing and any new ADRs, so that a later
session *could* implement Cuba Libre without violating — or being blocked by —
the existing constitution.

Your inputs are three things, and only these:

1. **This brief** (self-contained; it carries every locked decision).
2. **The uploaded repository manifest** — `manifest_2026-06-28_142ddfa.txt`,
   the exact file listing of the repository at the baseline commit. Treat any
   path in it as fetchable from that commit; treat any path *not* in it as
   nonexistent.
3. **Two PDFs uploaded to you in the web app**:
   - `CL-PLAYBOOK-2018.pdf` — the Cuba Libre playbook (extended examples, the
     non-player faction flowcharts, designer notes).
   - `CL-RULES-2019-LivingRules.pdf` — the Cuba Libre living rules (the rules of
     play and the full event-deck card details).

   **These PDFs are NOT in the repository and never will be** — they are private
   licensed IP that must not live in a public repo. The manifest will *not* list
   them. Read them from the upload to understand what the game actually requires;
   do **not** quote their prose into the deliverable (see §8, IP discipline).

**Baseline commit:** `142ddfa` (full:
`142ddfae2be3ae2d7c861ab65f2c786a49de54ac`), branch `main`. Fetch every
repository file you read from this exact commit so your analysis matches the
manifest. If your own self-check finds a read-list path missing at this commit,
note it in the deliverable rather than guessing.

**Deep research is encouraged and expected.** This is not a repo-only
realignment. Research COIN/CDG card-driven engines, multi-faction asymmetric
state modeling, conditional event-card implementation strategies in typed
languages, private-IP monorepo/submodule architectures, and CI federation across
public+private repositories. Bring back named prior art and cite it. The depth
is bounded only by *relevance to the change plan* — research to sharpen the
recommendations, not to expand scope beyond §3.

---

## 1. Baseline, manifest, and the prior cycle this brief extends

- **Repository:** Rulepath — a Rust-first, rule-enforcing, replayable, testable
  card/board-game platform. **Rust owns all behavior; TypeScript/React present
  only.** It ships public, public-domain/original/permissioned games through a
  staged ladder.
- **Baseline commit:** `142ddfa` on `main`. The uploaded
  `manifest_2026-06-28_142ddfa.txt` is `git ls-tree -r --name-only` at exactly
  this commit. It is your authoritative file inventory.
- **This brief is a sibling of a prior, already-executed doc/template overhaul
  cycle.** Two reports in `reports/` are your delta baseline and your structural
  template:
  - `reports/doc-and-template-overhaul-from-game-evidence-research-brief.md` —
    the prior brief.
  - `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the prior deliverable.
    **Mirror its section anatomy** (see §7). **Do not re-recommend work it
    already shipped** (see §3, "Implemented baseline").

The prior cycle was about *public* multi-seat games. This cycle is the next,
sharply different axis: a **private, licensed, far-more-complex** game that the
current doctrine explicitly defers and isolates. Frame your plan as the delta:
what must change *now* to legitimize and support a private monster game, on top
of the public-game doctrine already in place.

---

## 2. Read-in-full list (authority-ordered; fetch each at commit `142ddfa`)

Read the foundation documents in the order below (this is the repo's own
authority order from `docs/README.md`). For each, you are looking for the
specific rules that **strain, block, or must be amended** to support a private
COIN game. The one-line reason after each path is why it is load-bearing *for
this target*.

### Constitution + index
- `docs/FOUNDATIONS.md` — the constitution; §1 priority order, §2 behavior
  authority, §5 no-conditional-static-data/no-DSL, §10 IP conservatism, §11
  invariants, §12 stop conditions, §13 ADR triggers are all directly implicated.
- `docs/README.md` — the doc map, decision hierarchy, ADR status index; tells
  you which docs are foundation law vs. subordinate.

### Boundary / architecture / mechanic-ladder tier
- `docs/ARCHITECTURE.md` — action-tree/view/effect/replay model and reuse-ownership
  matrix; COIN operations + special activities + sequence-of-play stress the
  progressive action-tree and the binary public/private view model.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact engine/game/data boundary, the
  forbidden-static-data list, and the future-DSL policy; the ~90 conditional event
  cards collide here most directly.
- `docs/OFFICIAL-GAME-CONTRACT.md` — definition of "official/done," requirements-first
  workflow, rule-coverage matrix, mechanic inventory; scales poorly to a 4-faction,
  90-card game and has no private-game completion profile.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure ledger, first/second/third-use gate,
  inventory categories, open promotion-debt register; has no category for conditional
  card effects, asymmetric-faction operations, or propaganda-round automation.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — behavior-free scaffolding lane (ADR
  0008); faction routing and event-card payload handling are tempting-but-forbidden
  scaffolding candidates that must stay behavioral.

### Private / roadmap / AI / N-seat / evidence tier
- `docs/ROADMAP.md` — the staged ladder; private red-team work is the "Gate P tail,"
  admitted only after the public ladder completes. Starting Cuba Libre now contradicts
  this ordering and needs an ADR.
- `docs/IP-POLICY.md` — public/private content separation; private experiments are
  late, in private repos, excluded from public CI/builds/docs. Cataloging a private
  game and running its CI now strains §9/§10 here.
- `docs/AI-BOTS.md` — bot levels, the v1/v2 ban on MCTS/ISMCTS/Monte Carlo/ML/RL, the
  N-player imperfect-info belief-model requirements, Level 2 competent-player intake;
  the future "researched, non-flowchart" faction AI must fit inside this.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — seat-range declaration, roles/factions
  visibility, pairwise no-leak matrix, larger-surface budgets; 4 asymmetric factions
  and COIN eligibility/sequence-of-play push every section.
- `docs/AGENT-DISCIPLINE.md` — bounded-task law, forbidden kernel changes, the
  new-game scaffolding reuse-and-track protocol, IP protocol; a monster game's
  decomposition and its IP isolation both live here.
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM→browser catalog/operation groups, payload
  filtering, viewer-scoped replay, canonical seat grammar; how a private game would (or
  would not) surface through `list_games` and stay leak-safe.
- `docs/UI-INTERACTION.md` — public visual target, game picker, multi-seat layout,
  legal-only interaction, outcome/victory surface; how a private game appears in the
  catalog and renders faction-asymmetric play.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden traces, no-leak tests,
  benchmarks, CI gates; the coverage a private game's CI must run.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — evidence-fixture profiles (ADR 0009); how a
  90-card, 4-seat game's fixtures and viewer-scoped exports are profiled.
- `docs/TRACE-SCHEMA-v1.md` — trace/replay-fixture schema law; whether COIN's larger
  event/effect vocabulary fits the v1 schema.
- `docs/SOURCES.md` — researched bibliography and Rulepath lessons; where a private-IP
  sourcing note for licensed games would land.
- `docs/archival-workflow.md` — how superseded specs/docs are archived; relevant if you
  recommend merging/removing/replacing any doc.

### ADRs
- `docs/adr/ADR-TEMPLATE.md` — the ADR format any new ADR you propose must follow.
- Skim all of `docs/adr/0001`…`0009`; read in full the load-bearing ones:
  `0004` (hidden-info replay/export taxonomy), `0007` (next public scaling phase &
  Gate-P-tail placement — the ordering you may be amending), `0008` (mechanical
  scaffolding governance), `0009` (replay/fixture/hash taxonomy).

### Templates (read every file in `templates/**`)
`templates/README.md`, `AGENT-TASK.md`, `GAME-RULES.md`, `GAME-MECHANICS.md`,
`GAME-RULE-COVERAGE.md`, `GAME-SOURCES.md`, `GAME-HOW-TO-PLAY.md`, `GAME-UI.md`,
`GAME-AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`,
`GAME-BENCHMARKS.md`, `GAME-EVIDENCE.md`, `GAME-IMPLEMENTATION-ADMISSION.md`,
`PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md` — assess each for a
4-faction, 90-event-card private COIN game. Known gaps to confirm/expand:
single-faction assumption in `GAME-HOW-TO-PLAY.md`; 90-row scale in
`GAME-RULE-COVERAGE.md`; four separate faction policies in
`COMPETENT-PLAYER.md`/`BOT-STRATEGY-EVIDENCE-PACK.md`/`GAME-AI.md`; and
`PUBLIC-RELEASE-CHECKLIST.md` having **no private-release analogue**.

### Specs index + sibling cycle
- `specs/README.md` — the living gate/progress index; where a private Cuba Libre
  lane/gate and its spec fields must be placed.
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — **mirror this anatomy**; it is
  also your "already shipped, do not re-recommend" reference.
- `reports/doc-and-template-overhaul-from-game-evidence-research-brief.md` — the prior
  brief, for continuity of voice and scope.

### VCS / CI / catalog code seams (inspect to ground Part C; do not treat as foundation law)
- `Cargo.toml` — the `[workspace]` members list; how `games/*`, `crates/*`, `tools/*`
  are wired, and how an out-of-tree private game crate would (or would not) join.
- `crates/wasm-api/src/constants.rs` — the game registry (`GAME_*` id +
  `_DISPLAY_NAME` pairs); the single source of truth for cataloged games.
- `crates/wasm-api/src/catalog.rs` — how the catalog/`list_games` metadata is assembled
  for the browser.
- `ci/games.json` — per-game CI matrix entries (id, sim flags, e2e script).
- `.github/workflows/gate-0-hygiene.yml`, `gate-1-game-smoke.yml`,
  `gate-2-benchmarks.yml` — the three CI gates a private game's repo must run for full
  coverage.
- `scripts/boundary-check.sh` — engine-core noun-free enforcement.
- `scripts/check-catalog-docs.mjs` — catalog↔docs drift check (constrains how a private
  game is registered/documented).
- `scripts/check-scaffolding-governance.mjs` — scaffolding-register audit.

You may explore beyond this list (e.g. `games/*` for how an existing game is
wired end-to-end, `tools/*`, other `scripts/*`). The list is the floor, not the
ceiling.

---

## 3. Locked decisions (the requirements — do not re-open)

These were settled in the Session-1 interview. They are **decisions, not
suggestions.** Build the plan on them; do not re-litigate them or ask about them.

**D1 — Deliverable = a single consolidated change-plan markdown.**
Recommendations only: per-file change entries plus ADR *stubs to author*. **Do
not** produce rewritten doc/template files or finished ADR text. A later session
executes the plan. Mirror the anatomy of
`reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`.

**D2 — Full doctrine revision is authorized.** You may propose amending the
constitution itself — `FOUNDATIONS.md` §1 priority order, the `ROADMAP.md`
ladder ordering (incl. the Gate-P-tail placement from ADR 0007), and
`IP-POLICY.md` — to **carve a sanctioned private-game lane that can be worked on
NOW**, in parallel with the unfinished public ladder. The hard constraint that
must survive: **private licensed IP stays isolated from public architecture,
public bundles (JS/WASM), public CI artifacts, and public docs.** "Workable now"
must not mean "private IP shapes `engine-core` or leaks into public surfaces."
Name every ADR this requires (at minimum, expect one that supersedes/limits the
relevant parts of ADR 0007 and the FOUNDATIONS priority order).

**D3 — A constrained event-card mechanism may be recommended (with an ADR).**
The ~90 event cards have conditional/triggered text and collide with
FOUNDATIONS §5 (no selectors/conditions/triggers/conditional-effects in static
data) and §14/§5 (no DSL without an ADR and repeated painful evidence). You may,
**if your deep research justifies it**, recommend a *constrained, typed,
Rust-owned* card-effect mechanism under a **new ADR** — still no untyped DSL, no
YAML, no data-driven rule behavior. **Default if research does not justify a new
mechanism: typed-Rust-per-card** (each card is a hand-written typed Rust effect),
and the plan says so explicitly. Either way, the plan must analyze the §5/§14
boundary head-on: what stays typed content (card identity, parameters, display
metadata) vs. what must be Rust behavior (the conditional/triggered effect), and
how to keep the line from sliding into a forbidden data-driven rules engine.
Survey how other card-driven / COIN engines structure event resolution and feed
that into the recommendation.

**D4 — Private VCS/CI/catalog architecture is in scope and is a bounded
delegation to you.** Recommend, with deep research, **how private games should
live in version control** so that:
  - the **private repository runs the GitHub CI workflows** (gate-0 hygiene,
    gate-1 game smoke + per-game matrix, gate-2 benchmarks) — i.e. a private game
    is held to the *same* full testing/coverage bar as public games; and
  - the **normal web-app catalog surfaces private games** appropriately. The
    maintainer's working hypothesis is "*show private games in the normal lists
    when the private repo is present, hide them otherwise.*" **Treat this as a
    hypothesis to validate or improve, not a settled decision** — evaluate it
    against leak-safety, build reproducibility, and the IP-isolation constraint.

  Enumerate the realistic options (e.g. git submodule of a private repo into the
  public workspace; a fully separate private repo that vendors/depends on the
  public crates and runs the shared workflows; a workspace overlay / path-based
  private members file; a private cargo registry), compare them on isolation,
  CI coverage, catalog integration, and reproducibility, and **give a required
  default recommendation with justification.** Cover concretely how this
  interacts with: `Cargo.toml` `[workspace] members`, `crates/wasm-api/src/constants.rs`
  registration, `ci/games.json`, `crates/wasm-api/src/catalog.rs`,
  `scripts/check-catalog-docs.mjs`, and the three workflow files — i.e. what new
  seam (a conditional/overlay registration, a private members manifest, a
  build-time feature flag) is needed so a private game appears only when its repo
  is present and never bundles into the public build.

**D5 — Roadmap + specs placement is included.** Propose where the private Cuba
Libre **milestone-1** sits: a roadmap lane/gate in `ROADMAP.md`, a
`specs/README.md` index entry, and the private-game spec fields a future spec
must carry (mechanical-scaffolding reuse audit, no-leak matrix scope, seat-range
declaration, surface budgets, private-release readiness). **Do not write the game
spec itself** — only its placement and the field requirements.

**D6 — AI is milestone-2-and-later; do not design it.** Milestone-1 has **no
complex AI** (a Level-0 random-legal bot per the official-game contract is the
floor; confirm whether even that is required for a private milestone-1 or can be
deferred, and recommend). The eventual faction AI will be **researched online for
competent play, not transcribed from the COIN non-player flowcharts**, and must
stay inside `AI-BOTS.md` law (no MCTS/ISMCTS/Monte Carlo/ML/RL in v1/v2). The plan
should **flag** the changes a future 4-faction researched AI will need in
`AI-BOTS.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md`,
and `templates/GAME-AI.md` (four asymmetric faction policies, belief-model and
no-leak obligations, coalition/kingmaking handling) — **but does not design the
AI.** Note explicitly that the "no flowcharts" stance means the COIN playbook
flowcharts are *not* a sanctioned bot source.

**D7 — Milestone-1 capability target (what the doctrine must be able to host).**
Full **4-faction hotseat**; **all operations and special activities**;
**propaganda rounds**; **victory/terminal detection**; **all event cards
resolved.** This is the capability the amended docs/templates/specs must be able
to support — your plan's job is to ensure none of these is doctrinally blocked or
un-templated.

**D8 — Deep external research is encouraged (maintainer requirement).** Research
as much as needed: COIN/CDG event engines and prior digital COIN implementations,
multi-faction asymmetric turn/eligibility systems, typed conditional-effect
patterns, private-IP monorepo and CI-federation patterns, conditional app-catalog
inclusion. Cite named sources in a method/evidence section.

### Implemented baseline — do NOT re-recommend (sharp delta)

The prior overhaul cycle already shipped the following; treat them as **present
and correct**, build on them, and do **not** propose them as missing:

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` exists, with seat-range declaration,
  pairwise no-leak matrix, larger-surface budgets, and per-seat outcome
  obligations. (Improve/extend for asymmetric factions if needed — but it is not
  missing.)
- The public scaling phase (Gates 15–23 + the Gate-P tail) is already in
  `ROADMAP.md`, and **ADR 0007 is accepted**. Your D2 work *amends/supersedes*
  the Gate-P-tail ordering for private games — frame it as a delta to an existing
  accepted decision, not as filling a void.
- ADR 0008 (mechanical-scaffolding governance) and the
  `MECHANICAL-SCAFFOLDING-REGISTER.md`, ADR 0009 (replay/fixture/hash taxonomy)
  and `EVIDENCE-FIXTURE-CONTRACT.md`, and ADR 0004 (hidden-info replay/export)
  all exist. Reuse them; do not reinvent them.
- N-seat template fields and the per-game evidence/completion-profile machinery
  already exist across `templates/**`. Extend for the COIN scale; don't re-add
  what's there.

Verify each of these against the actual files at `142ddfa` before relying on this
list, and call out in the deliverable if any has drifted.

---

## 4. The core tensions to resolve (your analytical spine)

Organize the plan so it visibly resolves each of these. These are the load-bearing
conflicts between "what Cuba Libre needs" and "what the constitution currently
says." For each, the plan must state: the conflict, the amendment(s), the
doctrine-check (does the amendment preserve the non-negotiables?), and the ADR if
any.

1. **Private-game timing vs. public-first priority.** FOUNDATIONS §1 ranks
   "later private stress tests" 5th and forbids private pressure shaping public
   architecture; ROADMAP makes private work the Gate-P tail after Gate 23;
   IP-POLICY §9 says private experiments are late and isolated. Cuba Libre starts
   now. → Resolve via D2 (sanctioned private lane + ADR) while preserving "no
   private IP in public architecture/bundles/CI/docs."

2. **Conditional event deck vs. no-conditional-static-data / no-DSL.**
   FOUNDATIONS §5 and `ENGINE-GAME-DATA-BOUNDARY.md` forbid selectors/triggers/
   conditional effects in static data and forbid a DSL without an ADR. ~90 cards
   are inherently conditional. → Resolve via D3 (typed mechanism + ADR, or
   typed-Rust-per-card default), with the boundary line drawn explicitly.

3. **Private IP isolation vs. cataloged/CI-covered private game.** The game must
   run the full CI bar and optionally appear in the app catalog, yet no licensed
   prose/art/data may enter public bundles, CI artifacts, or docs, and "if it
   ships to an unauthorized browser, it has shipped." → Resolve via D4
   (VCS/CI/catalog architecture with a present-only, never-public-bundled seam).

4. **4-faction asymmetric COIN vs. current N-seat/mechanic doctrine.** Eligibility/
   sequence-of-play, asymmetric operations + special activities, propaganda rounds,
   and faction-asymmetric victory have no mechanic-atlas category and stress the
   action-tree/view model and the rule-coverage scale. → Resolve via mechanic-atlas
   categories, MULTI-SEAT extensions, OFFICIAL-GAME-CONTRACT scaling guidance, and
   template changes. Flag (do not resolve) the third-use primitive-pressure
   implications for *future* COIN games.

5. **Researched non-flowchart faction AI vs. AI-BOTS law.** Future AI must be
   competent across 4 asymmetric factions without MCTS/ISMCTS/ML/RL and without
   copying the licensed flowcharts. → Flag the AI-BOTS / competent-player / evidence-pack
   template changes (D6); do not design the AI.

6. **Evidence/testing scale.** 90 cards × 4 factions × per-seat no-leak proofs ×
   golden traces × benchmarks is an order of magnitude beyond current games. →
   Address scaling guidance in OFFICIAL-GAME-CONTRACT, TESTING-REPLAY-BENCHMARKING,
   GAME-RULE-COVERAGE, GAME-BENCHMARKS, EVIDENCE-FIXTURE-CONTRACT, and the
   private-release readiness analogue to PUBLIC-RELEASE-CHECKLIST.

---

## 5. Deliverable structure (what to produce)

Produce **one markdown file** named:

`cuba-libre-private-game-foundation-readiness-change-plan.md`

Mirror the anatomy of `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`, adapted
to this target:

1. **Executive summary** — the thesis and the highest-leverage moves (expect:
   the sanctioned-private-lane ADR, the event-card mechanism decision, the
   VCS/CI/catalog architecture).
2. **Method & evidence base** — baseline commit `142ddfa`; which repo docs/
   templates/seams you read; how you read the two PDFs; the external prior art you
   researched (named, cited); how you verified manifest correspondence.
3. **Implemented baseline acknowledgment** — restate §3's "do not re-recommend"
   list and confirm you built on it.
4. **The change plan**, in parts. For **every** entry use a consistent record:
   `ID · Target file(s) · Type (amend / new / merge / remove / new-ADR) ·
   Evidence (what in the PDFs or repo forces it) · Proposed change (precise, but
   prose-level — not a finished rewrite) · Rationale · Doctrine-check (which
   invariants/stop-conditions it touches and why it stays compliant) · Priority ·
   Depends-on`. Group into:
   - **Part A — Foundation docs** (`docs/**`): FOUNDATIONS, ENGINE-GAME-DATA-BOUNDARY,
     OFFICIAL-GAME-CONTRACT, MECHANIC-ATLAS, MECHANICAL-SCAFFOLDING-REGISTER,
     ARCHITECTURE, MULTI-SEAT, AI-BOTS, IP-POLICY, ROADMAP, WASM-CLIENT-BOUNDARY,
     UI-INTERACTION, TESTING-REPLAY-BENCHMARKING, EVIDENCE-FIXTURE-CONTRACT,
     TRACE-SCHEMA-v1, SOURCES, AGENT-DISCIPLINE, README, archival-workflow — only
     those that actually need to change.
   - **Part B — Templates** (`templates/**`): per-template changes; new templates
     if needed (e.g. a private-release checklist, a per-faction how-to-play split,
     an event-card coverage template).
   - **Part C — VCS / CI / catalog & code-seam doctrine**: the private-game
     repository architecture (D4) and the documentation/governance changes it
     implies (note: doctrine/process recommendations, plus identifying the code
     seams a later implementation session will build — *not* the code itself).
   - **Part D — Doctrine & ADRs**: each new ADR as a **stub** (title, status
     `Proposed`, context, the decision in one paragraph, the FOUNDATIONS/ROADMAP/
     IP-POLICY sections it would amend, consequences) — following
     `docs/adr/ADR-TEMPLATE.md`. Expect at least: sanctioned-private-lane (amends
     §1 + ADR 0007), event-card mechanism (if recommended), private VCS/CI/catalog
     architecture. Do not write full ADR bodies.
   - **Part E — Roadmap & specs placement** (D5): the Cuba Libre milestone-1
     roadmap lane/gate and `specs/README.md` entry, plus the private-game spec
     field requirements.
5. **Prioritized execution order** — a sequence honoring dependencies (e.g.
   the sanctioned-private-lane ADR before the roadmap placement; the event-card
   ADR before any rule-coverage template scaling).
6. **Risks & rejected ideas** — what you considered and rejected and why
   (e.g. an untyped card DSL — rejected; bundling private content behind a runtime
   flag in the public build — rejected; etc.).
7. **Self-check** — confirm: every change preserves the non-negotiables (Rust owns
   behavior; engine-core stays noun-free; no untyped DSL/YAML; no hidden-info leak;
   no private IP in public surfaces; determinism preserved); every read-list path
   resolved at `142ddfa`; the deliverable re-recommends nothing from §3's
   implemented baseline; no licensed prose was copied from the PDFs.

---

## 6. Hard constraints that every recommendation must preserve

These are Rulepath non-negotiables (FOUNDATIONS §11 invariants, §12 stop
conditions). Any amendment you propose must keep them true. If a Cuba Libre need
*genuinely* requires relaxing one, that is an explicit ADR with a doctrine-check —
never a silent erosion.

- **Rust owns all behavior** (setup, legality, validation, transitions, scoring,
  RNG, effects, views, replay, serialization, bot decisions). TypeScript never
  decides legality.
- **`engine-core` stays generic and noun-free** — no `faction`, `card`, `deck`,
  `board`, `operation`, etc. Typed COIN nouns live in the game module first; they
  reach `game-stdlib` only through the mechanic-atlas process, never `engine-core`.
- **No YAML and no DSL without an accepted ADR.** Static data is typed
  content/parameters/metadata/fixtures/traces only — never selectors, conditions,
  or triggers. (D3 is the controlled exception process, not a loophole.)
- **No hidden-information leak** into payloads, DOM, storage, logs, effect logs,
  bot explanations, candidate rankings, replay exports, or tests — pairwise across
  all four factions and the public observer.
- **Determinism preserved** — replay, hashes, RNG, serialization order, traces stay
  deterministic or are explicitly migrated.
- **No private licensed IP in public surfaces** — not in public files, public CI
  artifacts, public docs, public traces, public bundles, or public WASM/JS. "If it
  ships to an unauthorized browser, it has shipped."
- **No MCTS/ISMCTS/Monte Carlo/ML/RL bots** in v1/v2.
- **Deliver complete, reviewable recommendations** — the plan itself is bounded and
  reviewable; it does not hand a future implementer an unbounded "implement Cuba
  Libre" task.

---

## 7. Style, voice, and scope discipline

- **Match the prior deliverable.** Read `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`
  and reuse its record format, heading style, and priority/dependency conventions.
- **Precise but not finished.** Each change entry describes *what* to change and
  *why*, at prose level. It does not contain the rewritten file. ADRs are stubs.
- **No scope inflation.** Commission exactly what §3 and §5 describe. Resist
  "while we're at it" additions (e.g. don't redesign the trace schema unless COIN
  genuinely forces it; don't design the bots; don't write the game spec).
- **Cite evidence.** When a change is driven by the PDFs, say what mechanic forces
  it (e.g. "eligibility/initiative track in the sequence of play," "the Pivotal/
  Capability event cards") **without quoting licensed prose**. When driven by repo
  doctrine, cite the doc + section.
- **Flag, don't solve, the deferred items** — future faction AI, future COIN
  third-use primitive pressure, hosted multiplayer. Name them as forward
  obligations with triggers, not as work to do now.

---

## 8. IP discipline (critical — this is the whole reason the PDFs aren't in the repo)

- The two PDFs are **private licensed IP**. Use them to understand requirements.
  **Do not** copy or closely paraphrase their rules prose, card text, flowchart
  text, board art, or component names into the deliverable.
- When you must reference a card or mechanic, refer to it **functionally and
  generically** ("a conditional event card that shifts support in a named space")
  rather than reproducing its title/text. The change plan is a public-repo-bound
  artifact in spirit; write it as if it could live in the public repo (it
  recommends changes to public-repo docs), so it must itself be IP-clean.
- The deliverable must **not** instruct anyone to put the PDFs, licensed prose,
  card text, or art into the repository. The private-game architecture (D4) exists
  precisely so licensed content stays out of the public tree and public bundles.
- Prefer neutral/original phrasing and IDs throughout, consistent with
  `docs/IP-POLICY.md`.

---

## 9. Final reminders

- You are **locked**: produce the deliverable directly. Do not ask questions.
- Inputs are this brief + `manifest_2026-06-28_142ddfa.txt` + the two uploaded
  PDFs. Fetch repo files at commit `142ddfa`.
- Output is **one** markdown change-plan:
  `cuba-libre-private-game-foundation-readiness-change-plan.md`.
- Honor the locked decisions D1–D8, resolve the six tensions in §4, preserve the
  §6 non-negotiables, keep IP discipline (§8), and re-recommend nothing from the
  §3 implemented baseline.
