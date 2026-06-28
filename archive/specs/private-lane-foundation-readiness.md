# Private Lane — Foundation Readiness

| Field | Value |
|---|---|
| Spec ID | `private-lane-foundation-readiness` |
| Unit | `PLP1-RDY` (active-epoch tracker; private-lane readiness interlock) |
| Roadmap stage | Public scaling phase — **non-feature** doctrine/law/template interlock that opens the sanctioned private lane (Private Lane P1) parallel to the public ladder |
| Roadmap build gate | Private-lane readiness (gates Private Lane P1 / amends Gate P) |
| Status | `Done` |
| Date | 2026-06-28 |
| Owner | Rulepath maintainers |
| Primary targets | new `docs/adr/0010-*.md`, `docs/adr/0011-*.md`, `docs/adr/0012-*.md`; the foundation/area doc set in `docs/**`; `templates/**` (incl. two new templates); `docs/ROADMAP.md`; `specs/README.md` |
| Browser implementation | Not applicable; governance / law / template / ADR pass only |
| Authority order | `docs/FOUNDATIONS.md` → `docs/README.md` → accepted `docs/adr/0010-0012` (+ 0007 as limited) → `docs/IP-POLICY.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md` / `docs/ARCHITECTURE.md` / `docs/WASM-CLIENT-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` / `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` → `docs/AGENT-DISCIPLINE.md` → this spec |

Where this spec and a foundation document disagree, the foundation document
wins. This spec is seeded from one advisory change plan —
[`../reports/private-lane-p1-foundation-readiness-change-plan.md`](../../reports/private-lane-p1-foundation-readiness-change-plan.md)
— authored against target commit `142ddfa`, which is the current `main` at
authoring time (`reports/manifest_2026-06-28_142ddfa.txt`). This spec does not
assume the plan's per-file claims are permanent: AGENT-TASK decomposition
(`/reassess-spec`) re-reads each target file against live `main` before editing,
and the plan's item IDs (`A-01`…`E-03`) and the exact draft blocks it carries are
traceability anchors, not pre-approved final wording. The verbatim constitution
and ADR-decision blocks in §§4–5 below are the load-bearing exceptions: they are
embedded here in full because they are the constitutional change this spec
exists to land, and the user directed that warranted foundational changes be
included in the spec.

> **IP / naming.** This is a **public** spec. Per the doctrine it establishes
> (report `A-03`, `E-02`), it uses an **opaque private-lane identifier**
> ("Private Lane P1", "the first private licensed game") and does **not** name
> the licensed GMT COIN-series title in its body. The specific title, rules
> references, card identities, fixtures, e2e names, and renderers live only in
> the private lane (private repository), never in public source. The seeding
> report's filename names the title for traceability; the maintainers may rename
> that report or the public anchor if they prefer stricter opacity
> (Assumption A5).

> **Reader orientation.** This is a **doctrine / readiness interlock** pass, not
> a gameplay gate and not the private game's implementation spec. It converts the
> change plan into the constitutional, IP-policy, boundary, template, roadmap, and
> spec-index groundwork that a sanctioned **parallel private lane** needs *before*
> any private implementation begins. It authors and accepts three ADRs
> (`0010` sanctioned private-game lane, `0011` constrained typed Rust event-card
> mechanism, `0012` private repository / CI / catalog overlay), amends
> `FOUNDATIONS.md` for **timing only**, and records the Part C VCS/CI/catalog
> design as **written doctrine + documented seam plans** — it executes **no
> public Rust/CI/TypeScript refactor**, creates **no private repository**, and
> implements **no game**. It writes **no game, kernel, shared-helper, trace,
> fixture, hash, RNG, or benchmark code**, and changes no game behavior or
> viewer-visible bytes. The actual public seam extraction, the private repository,
> and the private game implementation spec are **seeded forward** (§11), not done
> here.

## Objective

Make Rulepath ready to host its first sanctioned **private licensed** game
(Private Lane P1) running in parallel with the unfinished public ladder, while
preserving every non-negotiable: Rust owns behavior; `engine-core` stays
noun-free; no untyped DSL/YAML; hidden information is viewer-filtered;
determinism and trace/hash discipline hold; no private licensed IP reaches any
public surface; v1/v2 bots still exclude MCTS/ISMCTS/Monte Carlo/ML/RL.

Concretely, close the four foundation-readiness gaps the change plan identifies
(report §1):

1. **Sanction a parallel private lane** without making private pressure public
   architecture — a narrow, ADR-gated, timing-only constitutional carve-out
   (report `A-01`, `A-02`, `A-03`, `D-01`).
2. **Authorize a constrained typed Rust event-card mechanism** — a game-local
   typed registry + effect trait/match pattern, never YAML/DSL/untyped effect
   rows (report `A-04`, `D-02`).
3. **Default to a separate private repository** with a pinned public-Rulepath
   checkout and a private WASM/catalog/renderer overlay, recorded as doctrine and
   documented seam plans (report `C-01`…`C-08`, `D-03`).
4. **Add a private milestone profile and bot-deferral rule**, and **scale the
   templates** for COIN-scale private games without copying licensed material
   (report `A-05`–`A-19`, `B-01`–`B-16`, `E-01`–`E-03`).

Because items (1)–(3) amend `FOUNDATIONS.md` §1/§10/§11/§12, limit accepted
**ADR 0007**'s Gate-P-tail timing, and touch the static-data/no-DSL boundary —
all §13 ADR-trigger surfaces — the **three ADRs are authored and accepted
first**, and every foundation/area/template edit they gate lands only after.

## Scope

### In scope (report Parts A, B, D, E in full; Part C as doctrine)

- **Three ADRs, authored and accepted first** (report `D-01`/`D-02`/`D-03`):
  `0010` sanctioned parallel private-game lane; `0011` constrained typed Rust
  event-card mechanism; `0012` private repository, CI federation, and catalog
  overlay architecture. Verbatim Decision blocks in §5.
- **Constitution amendments** (report `A-01`, `A-03`, `A-04`): the §1 timing
  carve-out, the §10 IP-timing amendment, the §11 invariant extension, the §12
  stop conditions, and the §13 ADR-trigger note — verbatim draft blocks in §4.
- **Foundation / area doc amendments** (report `A-02`, `A-05`–`A-19`): ROADMAP
  private-lane section; OFFICIAL-GAME-CONTRACT private completion profiles + bot
  deferral; MECHANIC-ATLAS COIN categories + private-stress accounting;
  MECHANICAL-SCAFFOLDING-REGISTER COIN anti-examples; ARCHITECTURE private-overlay
  lane + large-action-tree guidance; MULTI-SEAT-AND-SURFACE-CONTRACT asymmetric
  factions + 5-viewer no-leak; AI-BOTS four-faction sourcing limits + no-flowchart
  rule; WASM-CLIENT-BOUNDARY private catalog semantics; UI-INTERACTION private
  web-overlay; TESTING-REPLAY-BENCHMARKING private large-game coverage;
  EVIDENCE-FIXTURE-CONTRACT private-source profiles; TRACE-SCHEMA-v1 large-event
  clarification (no migration); SOURCES external prior-art notes; AGENT-DISCIPLINE
  private-monster task discipline; README docs-map + ADR index; archival-workflow
  ADR-limited-roadmap note.
- **Part C VCS/CI/catalog doctrine + seam plans** (report `C-01`–`C-08`):
  recorded as written doctrine in `IP-POLICY.md`, `ARCHITECTURE.md`,
  `WASM-CLIENT-BOUNDARY.md`, `apps/web/README.md`, and ADR 0012 — the **default
  separate-private-repo decision**, the rejected alternatives, the catalog
  public-registry-plus-private-overlay **seam plan**, the renderer-overlay **seam
  plan**, the reusable-workflow CI-federation **seam plan**, and the
  public/private drift-check split **plan**. No `.rs`/`.mjs`/`.yml`/`.toml`
  source change in this unit.
- **Template amendments + two new templates** (report `B-01`–`B-16`): private-lane
  index guidance; AGENT-TASK private-source fields; GAME-RULES private-source +
  event-deck sections; GAME-MECHANICS COIN categories; GAME-RULE-COVERAGE split +
  new `templates/GAME-EVENT-COVERAGE.md`; GAME-SOURCES private-source receipts;
  GAME-HOW-TO-PLAY per-faction; GAME-UI private overlay/large-map; GAME-AI four
  asymmetric policies + deferral; COMPETENT-PLAYER per-faction; BOT-STRATEGY
  multi-opponent; GAME-BENCHMARKS COIN workloads; GAME-EVIDENCE private build/source
  proof; GAME-IMPLEMENTATION-ADMISSION private-lane ADR gates; PRIMITIVE-PRESSURE
  private-stress type; new `templates/PRIVATE-RELEASE-CHECKLIST.md` + PUBLIC
  cross-link.
- **Roadmap & specs placement** (report `E-01`–`E-03`): add **Private Lane P1**
  to `ROADMAP.md` beside the public gate order; add a **Private lane tracker**
  section + opaque `P1-M1` row to `specs/README.md`; record the milestone-1
  capability target + explicit non-goals as an OFFICIAL-GAME-CONTRACT profile
  note and a required private-spec field set.

### Out of scope (seeded forward — §11)

- **Any public Rust/CI/TypeScript refactor.** The catalog registry/adapter
  extraction (report `C-03`), the web renderer-registry seam (`C-04`), the
  reusable-workflow CI federation (`C-05`), and the drift/boundary check splits
  (`C-06`/`C-07`) are **documented as seam plans only**. Their implementation is a
  later forward unit, triggered when the private lane needs them.
- **Creating the private repository / workspace** (report `C-01`, execution-order
  step 8).
- **Authoring the private implementation spec** and **implementing the private
  game** (report `E-02` private spec, `E-03`, execution-order step 8). The private
  spec lives in the private repository.
- **Any game, helper, kernel, trace, fixture, hash, RNG, or benchmark code
  change**, and any change to deterministic or viewer-visible bytes.

### Not allowed (carried from ROADMAP Gate P + FOUNDATIONS §12)

- Private licensed content (title, rules prose, card text, art, screenshots,
  trade dress, flowchart text, fixtures, e2e names, catalog strings, private IDs)
  in **any** public file, public CI artifact, public doc, public trace, public
  bundle, or public WASM/JS.
- Adding the private game to public `Cargo.toml` members, public catalog
  constants, public CI manifests, or a public submodule / optional dependency
  that names it.
- COIN nouns (`faction`, `card`, `deck`, `operation`, `eligibility`, …) entering
  `engine-core`.
- YAML/DSL/untyped effect rows encoding selectors, conditions, triggers, rule
  overrides, or effect formulas.
- Private licensed evidence silently forcing public `game-stdlib` promotion or
  shaping public architecture.
- Publisher non-player flowcharts/priority charts copied into bot policy, tests,
  or strategy docs.
- Beginning any private implementation before ADRs 0010/0011/0012 are accepted.

## Deliverables

A doctrine/law/template/ADR tree only — no source tree. Concrete artifacts:

```
docs/adr/0010-sanctioned-parallel-private-game-lane.md      (new; Accepted)
docs/adr/0011-constrained-typed-rust-event-card-mechanism.md (new; Accepted)
docs/adr/0012-private-repository-ci-catalog-overlay.md      (new; Accepted)

docs/FOUNDATIONS.md                 (amend §1, §10, §11, §12, §13 — §4 blocks)
docs/ROADMAP.md                     (Private Lane P1 section; ADR-0007 limited note)
docs/IP-POLICY.md                   (sanctioned-lane section; no-leak checklist; Part C doctrine)
docs/ENGINE-GAME-DATA-BOUNDARY.md   (typed Rust card-effect registries section)
docs/OFFICIAL-GAME-CONTRACT.md      (private completion profiles; bot deferral; M1 note)
docs/MECHANIC-ATLAS.md              (COIN categories; private-stress accounting)
docs/MECHANICAL-SCAFFOLDING-REGISTER.md (COIN behavior-vs-scaffolding anti-examples)
docs/ARCHITECTURE.md                (private-overlay lane; large-action-tree; Part C seam plans)
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md (asymmetric factions; 5-viewer no-leak)
docs/AI-BOTS.md                     (four-faction sourcing limits; no-flowchart)
docs/WASM-CLIENT-BOUNDARY.md        (private catalog semantics; catalog seam plan)
docs/UI-INTERACTION.md              (private web-overlay; large asymmetric UI)
docs/TESTING-REPLAY-BENCHMARKING.md (private large-game coverage subsection)
docs/EVIDENCE-FIXTURE-CONTRACT.md   (private-source evidence profiles)
docs/TRACE-SCHEMA-v1.md             (large-event clarification; no migration)
docs/SOURCES.md                     (private-IP / event-engine prior-art notes)
docs/AGENT-DISCIPLINE.md            (private-monster task discipline / decomposition)
docs/README.md                      (docs map + ADR 0010-0012 index)
docs/archival-workflow.md           (ADR-limited roadmap-text note)
apps/web/README.md                  (private renderer-overlay + public-only catalog note — doctrine)

templates/README.md, templates/AGENT-TASK.md, templates/GAME-RULES.md,
templates/GAME-MECHANICS.md, templates/GAME-RULE-COVERAGE.md,
templates/GAME-SOURCES.md, templates/GAME-HOW-TO-PLAY.md, templates/GAME-UI.md,
templates/GAME-AI.md, templates/COMPETENT-PLAYER.md,
templates/BOT-STRATEGY-EVIDENCE-PACK.md, templates/GAME-BENCHMARKS.md,
templates/GAME-EVIDENCE.md, templates/GAME-IMPLEMENTATION-ADMISSION.md,
templates/PRIMITIVE-PRESSURE-LEDGER.md, templates/PUBLIC-RELEASE-CHECKLIST.md   (amend)
templates/GAME-EVENT-COVERAGE.md     (new)
templates/PRIVATE-RELEASE-CHECKLIST.md (new)

specs/README.md                      (Private lane tracker section + P1-M1 row; this unit's row → Done at exit)
```

## Work breakdown

Bounded, dependency-ordered items, each a candidate AGENT-TASK. The ordering
follows the change plan's prioritized execution order (report §5), steps 1–7;
step 8 is out of scope. **WB-1 (the three ADRs) must complete and be accepted
before any other item**, because every later edit operationalizes an accepted
ADR.

| WB | Items (report IDs) | Depends on | Summary |
|---|---|---|---|
| WB-1a | `D-01` → `docs/adr/0010` | — | Author + accept ADR 0010 (sanctioned parallel private-game lane). Verbatim Decision in §5.1. |
| WB-1b | `D-02` → `docs/adr/0011` | — | Author + accept ADR 0011 (constrained typed Rust event-card mechanism). Verbatim Decision in §5.2. |
| WB-1c | `D-03` → `docs/adr/0012` | — | Author + accept ADR 0012 (private repo / CI federation / catalog overlay). Verbatim Decision in §5.3. |
| WB-2 | `A-01`, `A-02`, `A-03`, `A-17`, `A-18`, `A-19` | WB-1a | Constitution + roadmap + IP + agent-discipline + docs-map + archival doctrine: FOUNDATIONS §1/§10/§11/§12/§13 (§4 blocks), ROADMAP ADR-0007-limited note, IP-POLICY sanctioned-lane section + no-leak checklist, AGENT-DISCIPLINE private-monster task discipline / decomposition, README ADR index, archival ADR-limited note. |
| WB-3 | `A-04`, `A-06`, `A-07`, `B-04`, `B-05`, `B-15` | WB-1b | Event-card boundary + mechanic/scaffolding/template safety: ENGINE-GAME-DATA-BOUNDARY typed-registry section, MECHANIC-ATLAS COIN categories, MECHANICAL-SCAFFOLDING-REGISTER anti-examples, GAME-MECHANICS rows, GAME-RULE-COVERAGE split + new GAME-EVENT-COVERAGE, PRIMITIVE-PRESSURE private-stress type. |
| WB-4 | `C-01`–`C-08`, `A-08`, `A-11`, `A-12`, `B-08`, `B-16` | WB-1c | Private VCS/CI/catalog **doctrine + seam plans** (no code): IP-POLICY/ARCHITECTURE default-private-repo decision + rejected alternatives + seam plans, WASM-CLIENT-BOUNDARY private catalog semantics, UI-INTERACTION private overlay, apps/web/README private renderer note, GAME-UI template, new PRIVATE-RELEASE-CHECKLIST + PUBLIC cross-link. |
| WB-5 | `A-05`, `A-10`, `B-09`, `B-10`, `B-11`, `B-13`, `B-14` | WB-2 | Private milestone profiles + AI sourcing limits/deferral + evidence/admission: OFFICIAL-GAME-CONTRACT completion profiles + bot deferral, AI-BOTS four-faction sourcing limits + no-flowchart rule, GAME-AI, COMPETENT-PLAYER, BOT-STRATEGY, GAME-EVIDENCE, GAME-IMPLEMENTATION-ADMISSION. |
| WB-6 | `A-09`, `A-13`, `A-14`, `A-15`, `B-01`, `B-02`, `B-03`, `B-06`, `B-07`, `B-12` | WB-2, WB-3 | Scale multi-seat / testing / fixtures / trace / rules / source / how-to / bench: MULTI-SEAT-AND-SURFACE-CONTRACT, TESTING-REPLAY-BENCHMARKING, EVIDENCE-FIXTURE-CONTRACT, TRACE-SCHEMA-v1, GAME-RULES, GAME-SOURCES, GAME-HOW-TO-PLAY, GAME-BENCHMARKS, plus templates/README + AGENT-TASK (`B-01`, `B-02`). |
| WB-7 | `A-16` | — | SOURCES external prior-art notes (Rally the Troops/GMT, VASSAL, boardgame.io, OpenSpiel, GitHub reusable workflows/checkout, Cargo workspaces/git/registries), each with the Rulepath lesson + non-adoption. |
| WB-8 | `E-01`, `E-02`, `E-03` | WB-1a, WB-1c, WB-2, WB-5 | Roadmap & specs placement: ROADMAP Private Lane P1 section; specs/README Private lane tracker + opaque `P1-M1` row (`Doctrine pending`); OFFICIAL-GAME-CONTRACT M1 capability/non-goals note + required private-spec field set. |
| WB-9 | closeout | all | Run `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, `bash scripts/boundary-check.sh`; flip this spec's tracker row to `Done` with evidence; confirm no public file names the licensed title and no private ID/string entered public source. |

## §4 — Verbatim constitution amendment blocks (draft)

These are the **draft** `docs/FOUNDATIONS.md` amendments WB-2 lands (report
`A-01`, `A-03`, `A-04`). They are embedded verbatim because they are the
constitutional change this spec exists to make; `/reassess-spec` re-reads the
live constitution before WB-2 edits, so exact placement/wording may be refined,
but the **meaning** is fixed here. All changes are **timing + lane** only;
no behavior/boundary/leak invariant is weakened.

**§4.1 — new §1 subsection (after the two `MUST NOT` paragraphs at current
L18–20):**

> ### 1.1 Sanctioned private-game lane (timing carve-out)
>
> A sanctioned private-game lane MAY begin licensed implementation work before
> the public staged ladder completes, but only after an accepted ADR names the
> private lane's scope, repository isolation, CI expectations, catalog/build
> boundary, and public-architecture non-contamination rule. This carve-out
> changes the priority order for **timing only**: priority item 5 (later private
> stress tests) may run in parallel with items 1–4 once authorized. It does not
> raise private work above polished public product quality, and it does not
> authorize private content in public files, public bundles, public docs, public
> CI artifacts, public traces, public WASM/JS, or `engine-core`. Public
> architecture may gain only generic, private-free extension seams.

**§4.2 — amend the §10 private-licensed paragraph (current L185):**

> Private licensed/commercial-game stress tests are isolated, optional,
> non-public, and forbidden from shaping `engine-core`. They MAY begin early only
> inside a sanctioned private lane authorized by an accepted ADR (see §1.1);
> absent that authorization they remain late-tail work. If code or data ships to
> an unauthorized browser, it has shipped.

**§4.3 — extend the §11 invariant "Private licensed experiments remain isolated
and non-architectural.":**

> - Private licensed experiments remain isolated and non-architectural. A
>   sanctioned private lane, when an accepted ADR authorizes early timing, is
>   still isolated and non-architectural; the public repository gains only
>   generic, private-free extension seams, and no public file, bundle, doc, CI
>   artifact, trace, or WASM/JS carries private licensed content or identifiers.

**§4.4 — add §12 stop conditions (after the existing private-content stops at
current L265–266):**

> - a sanctioned private-lane implementation begins before its authorizing
>   ADR(s) are accepted;
> - a private game is added to public Cargo workspace members, public catalog
>   constants, public docs/CI manifests, or a public submodule/optional
>   dependency that names it.

**§4.5 — add a §13 ADR-trigger note (after the existing
private-licensed-influence trigger at current L286):**

> The sanctioned private-game lane (priority-order/timing change), the
> constrained typed Rust event-card mechanism (rule-like-data boundary), and the
> private repository / CI / catalog overlay (private-licensed influence on public
> architecture) are each landed through an accepted ADR — `0010`, `0011`, and
> `0012` respectively.

## §5 — Verbatim ADR Decision blocks (draft)

The Decision text WB-1 lands in each ADR (report `D-01`/`D-02`/`D-03`), embedded
verbatim. Each ADR is authored from `docs/adr/ADR-TEMPLATE.md` with the full
section set (Context, Decision, Alternatives considered, Consequences, the impact
sections, Migration notes, Review checklist), `Status: Accepted`, and the
next-integer ID after `0009`. Each flags the constitution supersession and names
the amended sections.

**§5.1 — ADR 0010 `sanctioned-parallel-private-game-lane` (Decision):**

> Create a sanctioned private-game lane that may run in parallel with the public
> roadmap after explicit ADR approval. The lane permits private licensed
> implementation work now, but only in private repositories/build artifacts. It
> does not authorize private content in public source, public docs, public CI
> artifacts, public traces, public app bundles, or `engine-core`. Public
> architecture may gain only generic, private-free extension seams.
>
> *Amends:* `FOUNDATIONS.md` priority order, private-IP invariants/stop
> conditions, ADR triggers; `ROADMAP.md` Gate P ordering; `IP-POLICY.md` private
> experiment timing. *Limits* (does not supersede) accepted ADR 0007's Gate-P
> tail for timing.

**§5.2 — ADR 0011 `constrained-typed-rust-event-card-mechanism` (Decision):**

> Authorize a game-local typed Rust event-card mechanism. Card identity, deck
> order, inert display metadata, and non-behavioral parameters may be typed
> static content in the private crate. Every condition, selector, trigger, rule
> override, target choice, legality check, state transition, visibility decision,
> diagnostic, and semantic effect is implemented as Rust behavior through
> explicit functions/match arms/traits. No YAML, script, untyped JSON/TOML effect
> rows, or declarative behavior language is allowed.
>
> *Amends:* `FOUNDATIONS.md` static-data/no-DSL section; `ENGINE-GAME-DATA-BOUNDARY.md`
> typed-content/behavior line; `MECHANIC-ATLAS.md` private event-pressure notes.
> Mechanism is game-local/private until public-safe evidence justifies any public
> helper.

**§5.3 — ADR 0012 `private-repository-ci-catalog-overlay` (Decision):**

> Default private games to a separate private repository that pins the public
> Rulepath commit and owns private game crates, docs, fixtures, renderer overlay,
> e2e, private CI manifests, and private WASM/web build. Public repo changes may
> add only generic extension seams and reusable-workflow inputs. Public catalog
> contains only public games. Private catalog entries appear only in private
> build artifacts. A public submodule/feature/optional dependency that names
> private games is rejected as the default.
>
> *Amends:* `FOUNDATIONS.md` private-architecture trigger; `IP-POLICY.md` private
> build/repo rules; `WASM-CLIENT-BOUNDARY.md` catalog boundary; `ARCHITECTURE.md`
> overlay shape. **Records doctrine + documented seam plans only — no public
> code/CI change in the readiness unit** (the catalog/renderer/CI/drift seam
> implementations are seeded forward).

## Exit criteria

Mapped to the objective and the change plan's execution order:

1. **ADRs accepted.** `docs/adr/0010`, `0011`, `0012` exist with `Status:
   Accepted`, the next-integer IDs after `0009`, and the ADR-template section set.
   Each names the FOUNDATIONS/ROADMAP/IP-POLICY sections it amends and flags the
   constitution supersession. (WB-1)
2. **Constitution amended for timing only.** FOUNDATIONS §1 carries the
   sanctioned-private-lane carve-out; §10 relaxes private-work *timing* (not
   isolation/non-public/non-engine-core); §11 invariant and §12 stop conditions
   reflect the lane; §13 notes the three ADR triggers. Rust-owns-behavior,
   noun-free `engine-core`, no-DSL, no-leak, and determinism invariants are
   unchanged. (WB-2)
3. **Typed Rust event-card boundary documented.** ENGINE-GAME-DATA-BOUNDARY and
   FOUNDATIONS describe the typed-registry/effect-trait pattern and explicitly ban
   YAML/JSON/TOML/RON/table-row selectors, conditions, and effect formulas. (WB-3)
4. **Private repo / CI / catalog recorded as doctrine + seam plans.** IP-POLICY,
   ARCHITECTURE, WASM-CLIENT-BOUNDARY, apps/web/README, and ADR 0012 carry the
   default separate-private-repo decision, the rejected alternatives table, and
   the catalog/renderer/CI/drift seam plans — with **no** `.rs`/`.mjs`/`.yml`/
   `.toml` change in this unit. (WB-4)
5. **Private milestone + AI deferral + scaled templates landed.**
   OFFICIAL-GAME-CONTRACT defines `private-milestone-1-rule-complete`,
   `private-release-candidate`, `public-release-candidate`, with Level-0 bot
   deferral explicit and bounded; all `B-*` templates are amended and the two new
   templates (`GAME-EVENT-COVERAGE.md`, `PRIVATE-RELEASE-CHECKLIST.md`) exist with
   placeholder IDs only. (WB-5, WB-6)
6. **Roadmap & specs placement done.** ROADMAP shows Private Lane P1 beside (not
   inside) the public gate order; `specs/README.md` has a Private lane tracker with
   an opaque `P1-M1` row at `Doctrine pending` linking only to the accepted ADRs.
   (WB-8)
7. **Public-tree leak-clean + links intact.** No public file names the licensed
   title or carries a private ID/card/e2e/fixture/catalog string;
   `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, and
   `bash scripts/boundary-check.sh` pass. (WB-9)

## Acceptance evidence

| Layer | Evidence |
|---|---|
| Game-level (unit/rule/golden/property/sim/replay/serialization) | **Not applicable** — no game, no rule code, no fixtures/traces changed in this unit. |
| Determinism / hash / RNG | **Not applicable** — no trace schema, hash, RNG, or serialization-order change (TRACE-SCHEMA clarification is documentation only; report `A-15`). |
| Boundary check | `bash scripts/boundary-check.sh` passes — `engine-core` gains no COIN nouns. |
| Doc-link integrity | `node scripts/check-doc-links.mjs` passes after the spec, ADRs, and index links land. |
| Catalog-docs drift | `node scripts/check-catalog-docs.mjs` passes — no private game enters public catalog/README/smoke surfaces. |
| IP / no-leak (doc-level) | Manual public-tree scan: no licensed title, private ID, card ID, e2e/fixture filename, rules prose, or catalog string in any public file; new no-leak checklist (report `A-03`) present in IP-POLICY. |
| ADR review checklist | Each of 0010/0011/0012 completes the ADR-template Review checklist (Determinism / Replay-hash / Visibility / Data-Rust-boundary / `engine-core`-contamination / UI / Bot / IP / Benchmark impact). |

## FOUNDATIONS & boundary alignment

The deliverable is a *plan/doctrine* artifact; the table covers the
**product-behavior the plan governs** and the principles the ADRs engage.

| Principle | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| §1 Priority order | **tensions → amended via ADR** | Permits private item-5 work to run in parallel with items 1–4 *for timing only* @ a new §1 carve-out gated by accepted ADR 0010; public product quality still outranks private pressure. This is the warranted constitutional change, landed ADR-first. |
| §2 Behavior authority | aligns | All event-card resolution, operation legality, special activities, propaganda, victory, views, effects, replay, and bots stay Rust-owned @ ADR 0011 typed-registry boundary; TypeScript stays presenter-only. |
| §3 `engine-core` noun-free | aligns | COIN nouns (`faction`, `card`, `deck`, `operation`, `eligibility`) stay in the private game crate @ the boundary; `boundary-check.sh` runs unchanged; ADR 0012 adds only generic public seams. |
| §5 Static data is typed content, not behavior | aligns | ADR 0011 authorizes a typed Rust registry + match/trait effects @ the static-data boundary and bans YAML/DSL/untyped effect rows — selectors/conditions/triggers/overrides remain Rust. |
| §10 IP conservatism | **tensions → amended via ADR (timing) + reinforced** | §10's "late" timing is relaxed for a sanctioned lane @ ADR 0010; isolation, non-public, non-engine-core, and the "shipped to an unauthorized browser = shipped" rule are preserved and strengthened by the no-leak checklist + opaque-placeholder rule. |
| §11 invariants | aligns | "Private licensed experiments remain isolated and non-architectural" is extended, not weakened; public repo gains only generic private-free seams; all leak/determinism/bot invariants hold. |
| §12 stop conditions | aligns | Reinforces "private content enters public surfaces" and "private work shapes public architecture" as stops; adds "implementation begins before authorizing ADRs accepted." |
| §13 ADR triggers | aligns | Each change is landed through the triggered ADR — priority-order change (0010), rule-like-data boundary (0011), private-licensed influence on public architecture (0012). |

§12 stop conditions cleared: no `engine-core` noun growth; no procedural static
data; no YAML/DSL without ADR; no legality in TypeScript; no hidden-info leak; no
private content in public surfaces; no private work shaping public architecture
without an accepted ADR; bounded, reviewable, decomposed scope.

## Forbidden changes

- No edit to any `.rs`, `.mjs`, `.yml`, `.toml`, `.json`, or web/source file —
  Part C is doctrine + seam plans only (Scope, §11 deferrals).
- No new ADR may *supersede* ADR 0007 wholesale; 0010 **limits** ADR 0007's
  Gate-P-tail *timing* and leaves its isolation/non-public/non-architectural
  intent intact (report `A-02`, `A-19`).
- No private licensed title, ID, card text, rules prose, art, screenshot, trade
  dress, flowchart text, fixture/e2e name, or catalog string in any public file.
- No COIN noun added to `engine-core`; no behavior-bearing static data; no
  YAML/DSL.
- No relaxation of the Rust-owns-behavior, no-leak, determinism, or v1/v2 bot-ban
  invariants — only the §1/§10 **timing** of private work changes.
- No private game added to public workspace members, catalog constants, CI
  manifests, or a public submodule/optional dependency.

## Documentation updates required

- The ADR index in `docs/README.md` lists `0010`/`0011`/`0012` after acceptance
  (report `A-18`).
- `docs/ROADMAP.md` records Private Lane P1 and the ADR-0007-limited note (`A-02`,
  `E-01`).
- `specs/README.md`: (a) this spec's row flips `Planned` → `Done` at exit; (b) a
  new **Private lane tracker** section with the opaque `P1-M1` row at `Doctrine
  pending` (`E-02`). At authoring time only the row-add + status flip + interlock
  note land; the full tracker-section content is a WB-8 deliverable.
- This unit is **not** a web-exposed game gate; the `apps/web/README.md` change is
  a **doctrine** note (private renderer overlay; public-catalog-only), not a
  catalog row, so `scripts/check-catalog-docs.mjs` must still see zero private
  games.

## Sequencing

- **Predecessor:** none required to *begin* (the lane is parallel). The public
  scaling phase (Gates 21–23) is unaffected and retains its order; this unit does
  **not** reorder or block any public gate.
- **Successor / what this unblocks:** the sanctioned **Private Lane P1**. Its
  admission interlock (ROADMAP + the `P1-M1` tracker row) requires: ADRs
  0010/0011/0012 accepted (WB-1), this readiness spec `Done` (WB-9), then — in a
  **later** session, out of scope here — the private repository created and the
  private implementation spec authored *in the private repository* (report `C-01`,
  `E-02`, `E-03`, execution-order step 8).
- **Admission rule:** no private implementation begins until WB-1 ADRs are
  accepted and this spec is `Done`; this is a FOUNDATIONS §12 stop condition added
  by WB-2.
- **Relationship to Gate P:** Gate P remains the public roadmap's private-tail
  marker; Private Lane P1 is the *active, parallel* realization of that intent
  under ADR 0010. Gate P's `specs/README.md` interlock note is updated to point at
  this readiness unit and the three ADRs.

## Assumptions (one-line-correctable)

- **A1** — Scope is doctrine-readiness only (Parts A/B/D/E in full; Part C as
  doctrine + seam plans) — assuming the public seam *extraction* (`C-03`/`C-04`/
  `C-05`), the private repo, and the game implementation are later forward units
  (confirmed by the user at brainstorm time; report execution-order step 8).
- **A2** — One spec captures all in-scope work — assuming the user's "create **a**
  spec" fixes the count at one; the alternative split (one spec per Part) is
  recorded here, not taken.
- **A3** — The three ADRs are authored **and accepted** within this unit (as 8M
  did for ADR 0008/0009) — assuming acceptance is the maintainer's act during
  WB-1, recorded `Status: Accepted` before later WBs edit the docs they gate.
- **A4** — ADR 0010 **limits** (does not supersede) accepted ADR 0007 — assuming
  the partial-supersession + archival note (report `A-19`) is the right handling;
  if repo policy treats accepted ADRs as immutable, 0010 still only *limits*
  timing and the archival note records it.
- **A5** — This public spec uses an opaque private-lane identifier and does not
  name the licensed title — assuming the maintainers have not license-reviewed the
  title for public naming; if they have, the title may be named and the seeding
  report kept as-is.
- **A6** — `private-lane-foundation-readiness.md` is the right filename/Spec ID
  and `PLP1-RDY` the tracker unit key — assuming the opaque-naming convention;
  rename in one line if a different convention is preferred.
- **A7** — No external research is needed to author this readiness spec —
  assuming the change plan's already-cited prior art (report §2.4, Appendix B) is
  sufficient; the `B-*`/`A-16` template/source work records those citations rather
  than re-researching them.
