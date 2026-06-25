# Deep-research brief — Rulepath forward scaffolding-reuse governance (per-new-game obligation)

You are ChatGPT-Pro running a **locked deep-research session**. Everything you need is in this
prompt plus the uploaded manifest. **Do not interview, do not ask clarifying questions** — the
requirements below are final. Produce the deliverable directly (see §7).

---

## 1. Context

The uploaded file `manifest_2026-06-25_5ed1664.txt` is the complete path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.

The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
`OFFICIAL-GAME-CONTRACT.md` → `MECHANIC-ATLAS.md` → `MECHANICAL-SCAFFOLDING-REGISTER.md` →
the area docs → `ROADMAP.md` → `IP-POLICY.md` → `AGENT-DISCIPLINE.md` → `SOURCES.md` →
`WASM-CLIENT-BOUNDARY.md`. Earlier documents govern later ones; accepted ADRs (`docs/adr/`)
supersede a foundation doc **only** by explicitly naming the affected sections and landing the
updates.

**Fetch every file from commit `5ed1664` (full SHA `5ed1664de53eed9d51615786344905e3c05619d4`),
the current verified `main` HEAD — the manifest reflects exactly that tree.** If any file you read
cites a different "commit of record" or "target commit" (e.g. the prior change plan cites
`db0c50b`), that is that document's *own* historical baseline; note the divergence where it
matters and use `5ed1664`, not the cited string.

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing. Earlier entries govern later ones.

**Constitution & boundary (authority spine):**

- `docs/README.md` — the authority order and the layering rule; this deliverable amends several
  docs and must respect which governs which.
- `docs/FOUNDATIONS.md` — the constitution: §11 universal invariants (including the promoted-
  primitive adoption invariant), §12 stop conditions, §13 ADR triggers. Any new standing
  obligation must satisfy these and must not silently weaken them.
- `docs/ARCHITECTURE.md` — the ownership / reuse matrix (§3A) naming `engine-core`,
  `game-stdlib`, `game-test-support`, `wasm-api` homes; the forward reuse rule must point at the
  correct homes.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — §13 promotion and conformance process; the boundary the
  forward obligation must respect (narrowest-layer-wins, noun-free `engine-core`).

**Workflow docs the new obligation attaches to:**

- `docs/OFFICIAL-GAME-CONTRACT.md` — §3 official-game workflow; today it lists a one-time
  "primitive-pressure comparison" research step but no *standing* per-game reuse+track step.
  This is the primary insertion point for the forward obligation.
- `docs/MECHANIC-ATLAS.md` — §4 behavioral third-use hard gate, §5A promotion conformance,
  §10/§10A registers; the behavioral sibling that the scaffolding lane runs parallel to. The
  forward obligation must reuse this doc's shape **without** weakening the behavioral gate or
  overloading the atlas with behavior-free plumbing.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the register being wired into the forward
  workflow. Read its Entry Schema, Decision States (`candidate` / `local-only` / `promoted` /
  `promotion-debt-open` / `deferred` / `rejected`), Non-Promotion List, and the MSC-8C-001…010
  entries with their R1–R4 receipt tables and per-row "Next review trigger" fields. Note that its
  maintenance cadence is currently described reactively (entries land when an owning ticket
  proves evidence), not as a standing forward per-game obligation.
- `docs/AGENT-DISCIPLINE.md` — §8/§8A coding-agent law (bounded tasks, third-use gate,
  forbidden changes, failing-test protocol); where the per-game **reuse-first audit** and
  **register-new** steps must become standing agent obligations rather than one-off spec text.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — the test taxonomy and CI gate structure; the home in
  which a **new mechanical CI check** (the enforcement, see §3) is described and slotted into the
  CI gates.
- `docs/ROADMAP.md` — the prescriptive ladder; confirms Gate 18 (Spades) placement and is where a
  blocking governance interlock before Gate 18 would be admitted as law.

**ADRs (build on; do not re-propose or silently amend):**

- `docs/adr/0008-mechanical-scaffolding-governance.md` — **the accepted lane this brief builds
  on.** Its "Decision rule" already states: second-exact-duplication → keep-local-with-rationale /
  register / propose-narrow-extraction; pre-third-copy → hard decision. The gap is that this rule
  is not yet wired as a *standing forward per-new-game* obligation in the workflow docs/templates,
  and the register's own maintenance cadence is reactive. Do **not** re-propose ADR 0008's lane,
  register, allowed homes, or exclusions as if missing.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the adjacent accepted taxonomy governing any
  hash/fixture/export migration; the forward obligation must defer all byte/hash changes to it.
- `docs/adr/ADR-TEMPLATE.md` — the foundation-level ADR template, in case the recommendation is to
  author a short new ADR rather than amend ADR 0008 (see §3 item 6).

**Specs & templates (where the obligation is executed):**

- `specs/README.md` — the living spec index and progress tracker. Gate 18 (Spades) is the lowest
  non-`Done` unit; units 8M, 8C, and 8C-R1…R4 (the scaffolding lane) are all `Done`. This is where
  the new pre-Gate-18 governance unit and the "Workflow" step list are amended.
- `templates/README.md` — the per-game template lifecycle ordering; where the forward
  reuse/track hooks attach in the recommended authoring order.
- `templates/GAME-IMPLEMENTATION-ADMISSION.md` — the pre-code admission gate; it already has a
  "mechanical-scaffolding decision, if needed" row linking the register. Extend this into a
  mandatory **reuse-first audit** row.
- `templates/GAME-MECHANICS.md` — its "Required repo atlas update" section names MECHANIC-ATLAS +
  PRIMITIVE-PRESSURE-LEDGER but **omits the scaffolding register** — the concrete locus of gap (2).
- `templates/GAME-EVIDENCE.md` — the canonical per-game evidence receipt; already carries a
  "Mechanical scaffolding register decision" row (pre-implementation). Extend it with a
  post-implementation **new-scaffolding / refactoring-flag** receipt.
- `templates/AGENT-TASK.md` — the bounded task packet; carries the "Scaffold-Refactor Profile" and
  mechanics/primitive-pressure status. This is where a per-task reuse/track obligation executes.
- `templates/PRIMITIVE-PRESSURE-LEDGER.md` — the behavioral ledger; its "Behavioral scope only"
  clause already redirects non-behavioral plumbing to the scaffolding register. Keep that boundary
  crisp; the forward obligation must not blur behavioral mechanics into scaffolding or vice versa.

**Delta baseline — already implemented; build on, do NOT rebuild:**

- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the prior change plan that produced ADR
  0008/0009, the scaffolding register, `GAME-EVIDENCE.md`, and completion profiles. Its
  recommendations shipped (unit 8M). Treat it as the implemented baseline.
- `reports/doc-and-template-overhaul-from-game-evidence-research-brief.md` — the prior locked brief
  whose section anatomy and scope this delta extends; reuse its structure, re-center the
  forward-obligation gap.
- `archive/specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md` — the **8M** spec that
  shipped the *retroactive* reuse doctrine, ADR 0008/0009, and the register. The standing-forward
  obligation is what 8M did **not** institutionalize.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` — the **8C** unit that created
  the register entries and the shared helpers; shows how scaffolding candidates were evaluated.
- `archive/specs/8c-r4-n-seat-private-trick-scaffolding-intermediate-spec.md` and
  `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` — one representative
  **8C-R\*** refactoring wave (the others, R1–R3, follow the same shape). These waves were *one-off
  spec text* that retrofitted older games onto new scaffolding. The deliverable generalizes this
  one-off pattern into a **standing rule** that auto-queues such a wave whenever a new game's
  scaffolding makes older games duplicative.

### Code seams to inspect directly (*inspect, not read-fully* — not in the manifest read-set above)

- `scripts/boundary-check.sh`, `scripts/check-catalog-docs.mjs`, `scripts/check-doc-links.mjs` —
  the existing mechanical CI gates and their wiring; the model and integration pattern for the new
  scaffolding-duplication CI check.
- `crates/engine-core/src/` — the promoted behavior-free scaffolding a new game must reuse
  (`EffectEnvelope` constructors, action-tree v1 encoding, `StableBytesWriter`).
- `crates/game-stdlib/src/` — the earned game-layer seat scaffolding (`seat` count/ring helpers,
  canonical seat-ID grammar).
- `crates/game-test-support/` — the dev-only test/evidence scaffolding crate (the `game-test-support`
  home named in ADR 0008).
- `games/briar_circuit/` or `games/vow_tide/` — the two most-recent games; a concrete picture of
  what a game built "under the new obligation" should look like and which scaffolding it already
  reuses.

---

## 3. Settled intentions (locked — these pre-empt every clarifying question)

The maintainer interview resolved the following. Treat each as a committed decision.

1. **The maintainer's three concerns are sound, and the deliverable affirms them — but the framing
   is a *sharp delta*, not a cold start.** The *retroactive* half of "games reuse scaffolding;
   scaffolding is tracked; refactoring waves consolidate duplicates" is **already shipped**:
   - accepted **ADR 0008** (mechanical-scaffolding governance lane, allowed homes, exclusions,
     second-use/third-copy decision rule);
   - `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (entry schema, decision states, Non-Promotion List,
     entries MSC-8C-001…010 with R1–R4 receipts);
   - the behavioral **third-use hard gate** in `MECHANIC-ATLAS.md` §4 (kept intact, deliberately
     separate from the scaffolding lane);
   - `GAME-EVIDENCE.md`'s register-decision receipt row and `GAME-IMPLEMENTATION-ADMISSION.md`'s
     pre-code scaffolding-decision row;
   - the **8C** extraction unit and the **8C-R1…R4** retrofit waves (all `Done`).
   **Session 2 MUST NOT re-recommend any of this as if missing.** Build on it; improve only where it
   falls short of the forward gap below.

2. **The gap to close is the standing, forward, per-new-game obligation**, in exactly the three
   parts the maintainer named:
   - **(1) reuse-first audit** — a standing requirement that *every* new game, before
     reimplementing plumbing, audits `MECHANICAL-SCAFFOLDING-REGISTER.md` and the relevant atlas
     promotions and reuses what exists. Today this is only a one-time "primitive-pressure
     comparison" research step in `OFFICIAL-GAME-CONTRACT.md` §3, not a standing gate.
   - **(2) register-new** — a standing requirement that when a new game invents new behavior-free
     scaffolding, it is added to the register. Today `GAME-MECHANICS.md`'s "Required repo atlas
     update" names the mechanic atlas + primitive-pressure ledger but **omits the scaffolding
     register**, and the register's maintenance is described reactively.
   - **(3) auto-schedule-refactor** — a standing rule that when a new game's scaffolding renders
     already-shipped games duplicative, a follow-on refactoring unit is queued automatically.
     Today this exists only as one-off 8C-R\* spec text plus per-row "Next review trigger" fields,
     not a standing rule the tracker enforces.

3. **Deliverable framing (locked):** a **change-plan markdown that also recommends a dedicated
   pre-Gate-18 governance spec unit** to land the amendments — mirroring how
   `RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` seeded unit 8M. The plan must include **draft
   amendment text** for each affected `docs/**` and `templates/**` file (concrete enough to paste,
   not just "add a section about X"), and the recommended unit's **scope, work breakdown, and exit
   criteria**.

4. **Enforcement strength (locked): prose + checklist rows + a new mechanical CI gate.** Beyond
   the standing doc/template prose and admission/evidence checklist rows, recommend a concrete
   **CI check** — analogous to `scripts/boundary-check.sh` and `scripts/check-catalog-docs.mjs` —
   that mechanically fails when a new game ships scaffolding-shaped duplication without a
   corresponding register decision (or accepted exception). Specify what it inspects, what signal
   distinguishes "duplicated behavior-free scaffolding" from legitimately game-local code, how it
   avoids false positives on intentionally-local-with-rationale shapes, where it plugs into the CI
   gate ladder, and its failure/override semantics. If a fully-precise static check is infeasible,
   say so plainly and specify the strongest *tractable* check (e.g. a register-freshness / receipt-
   presence gate keyed off a per-game scaffolding-audit manifest) rather than overclaiming.

5. **Sequencing (locked): a blocking interlock before Gate 18.** The governance unit lands and
   closes **before** Gate 18 (Spades) is implemented, so Spades is the first game built under the
   new standing obligation. Reflect this in the recommended `specs/README.md` tracker row, its
   interlock text, and the "Workflow" step list (which currently only tells authors to check the
   *mechanic atlas* for promotion debt — it must also tell them to run the scaffolding reuse-first
   audit).

6. **`assumption:` ADR mechanism is a bounded delegation to Session 2.** Whether to (a) **amend
   ADR 0008's decision rule** to add the standing forward per-new-game obligation by naming the
   newly-affected doc sections, or (b) **author a short new ADR** that extends ADR 0008, is
   delegated to Session 2 to **recommend with a required default and justification**. Default
   expectation absent a strong reason otherwise: amend/extend ADR 0008 in place (it is the
   governing decision and explicitly anticipated downstream wiring), updating its "Affected
   foundation sections" list — but Session 2 picks and justifies. This is a scoped sub-choice; it
   does **not** reopen the locked framing, enforcement, or sequencing decisions.

7. **Hard constraints carried into every recommendation:**
   - The behavioral **third-use hard gate** (`MECHANIC-ATLAS.md` §4) stays word-for-word; the
     scaffolding lane stays *parallel and narrower*, never a replacement, and never lowers the
     behavioral boundary.
   - `engine-core` stays generic and **noun-free**; promotions respect narrowest-layer-wins and the
     ADR 0008 Non-Promotion List (deal/reveal/projection, betting/pot, trick lifecycle, teams,
     graph/topology, accounting, reaction windows, scoring/outcome — all stay behavioral).
   - No new YAML, DSL, selectors, conditions, or triggers; scaffolding is typed Rust infrastructure
     only.
   - Determinism and the no-leak firewall are preserved; any byte/hash/visibility change defers to
     ADR 0009 and an explicit migration, never silent regeneration.
   - The doc authority order is respected: no lower doc or template may contradict a foundation doc,
     and the obligation is introduced top-down (constitution/boundary → workflow docs → templates →
     CI), not bolted onto templates alone.

---

## 4. The task

This is a **foundational / doc-overhaul (governance) target.** Produce a single advisory
change-plan that institutionalizes a **standing, forward, per-new-game mechanical-scaffolding
reuse-and-tracking obligation** in Rulepath's `docs/**` and `templates/**`, closing the three gaps
in §3.2 while building strictly on the already-shipped ADR 0008 / register / third-use-gate
machinery. The plan must (a) state the implemented baseline and explicitly avoid re-proposing it;
(b) supply concrete draft amendment text for each affected doc/template; (c) specify the new
mechanical CI gate; (d) recommend the ADR mechanism (amend 0008 vs. new ADR) with a default and
justification; and (e) recommend a dedicated **pre-Gate-18 governance spec unit** — with scope,
work breakdown, exit criteria, and the `specs/README.md` tracker/interlock/workflow edits — that
lands the amendments as a blocking interlock before Gate 18.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — especially the register
entries, the 8C/8C-R\* specs and characterization reports, and the existing CI scripts — to ground
every recommendation in the actual current state.

Research online where it sharpens the deliverable, and cite sources for any external claim that
shapes a decision. **Calibration:** this is a behavior-free governance/process target under an
already-accepted ADR, so external prior art is a *sharpening aid, not the crux* — useful angles
include the "rule of three" refactoring discipline (Fowler), paved-road / golden-path internal-
platform governance, monorepo code-reuse and DRY/WET economics, software-reuse cost models, and
ADR / decision-registry maintenance practice. Do **not** let online research expand the locked
scope, invent new architecture, or relitigate ADR 0008's accepted lane; use it only to pressure-
test and strengthen the forward-obligation design.

---

## 6. Doctrine & constraints

Honor these throughout (trimmed to what this target engages):

- `docs/FOUNDATIONS.md` is the constitution — every recommendation must satisfy its §11 universal
  invariants and clear its §12 stop conditions; a genuine divergence requires an accepted ADR
  explicitly superseding the affected principle first, never designing against it silently.
- Authority order: foundation docs govern area docs govern specs govern tickets/templates. Introduce
  the obligation top-down; a template change that isn't backed by doctrine in the governing doc is
  wrong.
- `engine-core` stays generic and **noun-free**; shared helpers enter `game-stdlib` only via the
  mechanic atlas, and behavior-free scaffolding only via the ADR 0008 register and its narrow homes.
- **TypeScript never decides legality** — the obligation concerns Rust-owned scaffolding only; no
  presentation-layer rule authority.
- **No YAML and no DSL** — scaffolding stays typed Rust infrastructure; no selectors, conditions, or
  triggers.
- **Determinism** and the **no-leak firewall** stay intact; byte/hash/visibility changes defer to
  ADR 0009 and explicit migration.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots.**
- **Never weaken the behavioral third-use gate or any test** to make reuse cheaper — the scaffolding
  lane is additive and narrower, never a loosening.

---

## 7. Deliverable specification

Produce **one** downloadable markdown document:

- **Filename:** `forward-scaffolding-reuse-governance-change-plan.md`
- **Shape: an *intermediate advisory artifact* — NOT final repository law and NOT a ready-to-
  decompose spec.** It is the analogue of the existing `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-
  PLAN.md`. Downstream, the maintainer will save its recommended spec into
  `specs/<unit>-forward-scaffolding-reuse-governance.md`, then run `/reassess-spec` (in place under
  `specs/`) and `/spec-to-tickets`. Do **not** present it as a finished spec that skips reassessment.

The document must contain, at minimum:

1. **Executive summary** — affirm the maintainer's three concerns; state the implemented baseline
   (ADR 0008, register, third-use gate, 8C/8C-R\*, GAME-EVIDENCE/admission rows) and that this plan
   builds on it rather than rebuilding; name the three forward gaps and the closing moves.
2. **Per-file amendment set** — for each affected `docs/**` and `templates/**` file, the exact
   section to add/edit and **draft text** (paste-ready prose / table rows / checklist items),
   honoring authority order. Minimum coverage: `OFFICIAL-GAME-CONTRACT.md` §3 workflow,
   `AGENT-DISCIPLINE.md` (standing reuse-first + register-new agent obligation),
   `MECHANIC-ATLAS.md` / `MECHANICAL-SCAFFOLDING-REGISTER.md` (forward maintenance cadence +
   auto-refactor trigger), `TESTING-REPLAY-BENCHMARKING.md` (the CI gate), `ROADMAP.md` /
   `specs/README.md` (interlock + workflow step), and the templates `GAME-IMPLEMENTATION-ADMISSION.md`,
   `GAME-MECHANICS.md` (add the omitted register to "Required repo atlas update"), `GAME-EVIDENCE.md`,
   `AGENT-TASK.md`, `PRIMITIVE-PRESSURE-LEDGER.md`. Mark any file you conclude needs *no* change as
   an explicit `not applicable` row with rationale.
3. **CI gate specification** — what the new check inspects, its precision/false-positive strategy,
   where it slots into the CI gate ladder, its override/exception semantics, and the strongest
   tractable fallback if a precise static check is infeasible.
4. **ADR mechanism recommendation** — amend ADR 0008 vs. new ADR, with default + justification and
   the exact "Affected foundation sections" delta.
5. **Recommended governance spec unit** — proposed unit id/slug (consistent with the `specs/README.md`
   naming and the pre-Gate-18 lane), objective, scope (in/out/not-allowed), work breakdown as
   candidate AGENT-TASKs, exit criteria mapped to the gaps, and the precise `specs/README.md`
   tracker row + interlock text + "Workflow" step edit establishing the blocking-before-Gate-18
   sequencing.
6. **Determinism / no-leak / boundary impact statement** — confirm no behavioral gate weakening, no
   `engine-core` nouns, no DSL/YAML, no silent byte/hash/visibility change.

Locked / no-questions:

> Produce the deliverable directly as a downloadable markdown document. Do not interview,
> do not ask clarifying questions — the requirements above are final. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed
> with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- The deliverable is exactly the one markdown document named in §7, in the intermediate-artifact
  shape (not presented as a final spec).
- The implemented baseline (ADR 0008, register, third-use gate, 8C/8C-R\*, GAME-EVIDENCE/admission
  rows) is stated and **not** re-proposed as missing; every recommendation is a forward-obligation
  delta.
- All three forward gaps — reuse-first audit, register-new, auto-schedule-refactor — are closed with
  concrete draft text.
- Every affected doc/template has paste-ready amendment text or an explicit `not applicable` row;
  changes are introduced top-down in authority order and contradict no foundation doc.
- The behavioral third-use gate is preserved word-for-word; the scaffolding lane stays parallel and
  narrower; the ADR 0008 Non-Promotion List is respected; `engine-core` stays noun-free; no DSL/YAML;
  determinism and no-leak preserved (byte/hash/visibility changes deferred to ADR 0009).
- The CI gate, ADR mechanism, and pre-Gate-18 governance unit (with the `specs/README.md`
  interlock/workflow edits and blocking-before-Gate-18 sequencing) are all specified.
- Every external claim that shaped a decision is cited.
- All repository claims were derived from files fetched at commit `5ed1664`; any other "commit of
  record" encountered in a read file was flagged, not adopted.
