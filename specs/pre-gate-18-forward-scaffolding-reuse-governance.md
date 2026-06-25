# Pre-Gate-18 — Forward Scaffolding-Reuse Governance

**Status**: Done

| Field | Value |
|---|---|
| Spec ID | `pre-gate-18-forward-scaffolding-reuse-governance` |
| Unit | `8F` (active-epoch tracker) |
| Roadmap stage | Public scaling phase — pre-Gate-18 **non-feature** governance interlock |
| Roadmap build gate | Pre-Gate-18 — forward scaffolding-reuse governance (blocks Gate 18) |
| Status | Done |
| Date | 2026-06-25 |
| Owner | Rulepath maintainers |
| Primary targets | `docs/adr/0008-*.md` (append-only), the foundation/area doc set, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/ROADMAP.md`, `specs/README.md`, `templates/**`, new `ci/scaffolding-audits.json` + `scripts/check-scaffolding-governance.*` + Gate 1 wiring |
| Browser implementation | Not applicable; governance / law / template / CI-receipt pass only |
| Authority order | `docs/FOUNDATIONS.md` → `docs/README.md` → accepted `docs/adr/0008-*.md` (+ 0009) → `docs/MECHANIC-ATLAS.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/AGENT-DISCIPLINE.md` → this spec |

Where this spec and a foundation document disagree, the foundation document
wins. This spec is seeded from one advisory change plan —
[`../reports/forward-scaffolding-reuse-governance-change-plan.md`](../reports/forward-scaffolding-reuse-governance-change-plan.md)
— authored against target commit `5ed1664`, which is the current `main` at
authoring time (`manifest_2026-06-25_5ed1664.txt`). This spec does not assume the
plan's per-file claims are permanent: AGENT-TASK decomposition (`/reassess-spec`)
re-reads each target file against live `main` before editing, and the plan's
§-locations and the exact draft blocks it carries are traceability anchors, not
pre-approved final wording.

> Reader orientation: this is a **governance interlock** pass, not a gameplay
> gate. It is a forward delta on the shipped 8M reuse-doctrine realignment and the
> completed 8C / 8C-R1…R4 retroactive retrofit program. Those units gave Rulepath
> a *retroactive* scaffolding doctrine (ADR 0008, the register, the 17-game
> retrofit waves). This unit converts that doctrine into a **standing, forward,
> per-new-game obligation**: every new official game must complete a reuse-first
> audit, register every new behavior-free scaffolding shape on first use, and
> queue or explicitly dispose any prior-game refactoring its work exposes — with a
> Gate 1 receipt check that proves the obligation ran. It writes **no game, kernel,
> shared-helper, trace, fixture, hash, RNG, or benchmark code**, and it changes no
> game behavior or viewer-visible bytes. Spades (Gate 18) becomes the first game
> admitted under the standing rule.

## Objective

Institutionalize accepted ADR 0008's mechanical-scaffolding lane as a standing,
forward, per-new-game obligation before Gate 18. Concretely, close the three
forward-governance gaps the change plan identifies (plan §3):

1. **reuse-first audit** — every new official game completes a mandatory audit of
   the scaffolding register and the lawful shared homes before serious
   implementation; a `not applicable` result requires a rationale, never silence;
2. **register-new** — every newly invented behavior-free scaffolding shape gets a
   first-use register entry (`candidate` / `local-only` / `rejected`) before game
   closeout, with first use explicitly **not** authorizing promotion; and
3. **queue-or-dispose** — when a new game's scaffolding makes a prior official
   game a matching duplicate, the closeout either queues a named bounded follow-on
   refactor unit in `specs/README.md` or records an accepted `local-only` /
   `deferred` / `rejected` disposition with rationale, owner, evidence, and a next
   review trigger.

Land the authority-ordered doctrine, workflow, register-cadence, agent-law,
testing-law, roadmap, tracker, and template amendments — plus a tractable Gate 1
CI receipt check — **without** implementing a game, promoting a helper, or
changing any deterministic or viewer-visible byte. Because the obligation amends
`FOUNDATIONS.md` §11/§12 and operationalizes the decision recorded in an accepted
ADR (a §13 ADR-trigger surface), the **ADR 0008 append-only extension is authored
and accepted first**, and the foundation/area edits it gates land only after.

## Scope

### In scope (change plan §5–§8)

- **ADR 0008 forward-obligation extension** (plan §7): an explicitly dated,
  **append-only** extension to accepted `docs/adr/0008-mechanical-scaffolding-governance.md`
  — a status-note addition, the complete replacement `Affected foundation
  sections` field, the forward per-new-game obligation subsection appended to the
  end of the `Decision` section (before `Alternatives considered`), and the new
  migration-matrix rows. It does **not** change the lane, allowed homes,
  Non-Promotion List, semantic-identity rule, second-use review, or pre-third-copy
  hard-decision threshold. (Fallback — a short successor ADR `0010` that *extends*
  but does not supersede ADR 0008 — applies **only** if `/reassess-spec` finds an
  immutable-accepted-ADR policy or that the proposal changes the lane's
  architecture rather than operationalizing it; plan §7.6.)
- **Constitution amendments** (plan §5.2): add the three forward invariants to
  `FOUNDATIONS.md` §11 (after the existing mechanical-scaffolding invariant at
  L204–205) and the four forward stop conditions to §12 (after the promotion-debt
  stop conditions). §4 behavioral first/second/third-use wording and §13 ADR
  triggers are unchanged.
- **Architecture / boundary amendments** (plan §5.3, §5.4): a forward
  mechanical-scaffolding conformance section in `ARCHITECTURE.md` (new §3B + §14
  acceptance bullet + the truthfulness fix changing the `game-test-support` owner
  label from "future" to present-tense), and a parallel forward-conformance
  boundary section in `ENGINE-GAME-DATA-BOUNDARY.md` (new §13A).
- **Official-game workflow amendments** (plan §5.5): insert the reuse-first audit
  and scaffolding-closeout steps into the `OFFICIAL-GAME-CONTRACT.md` §3 workflow
  block, add the forward-obligation subsection, and extend the §12 acceptance
  cluster.
- **Atlas seam amendment** (plan §5.6): add a parallel mechanical-scaffolding
  check (new §5B) and stage-advancement bullets (§11) to `MECHANIC-ATLAS.md`
  **without changing §4 or §5A** behavioral text.
- **Register cadence amendments** (plan §5.7): broaden the `candidate` decision
  state to be first-use-safe, add the Forward Per-Game Maintenance Cadence and
  Automatic Prior-Game Refactor Trigger sections, update the Current Entries
  intro, and extend the Review Checklist in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
- **Agent / testing / roadmap / tracker law** (plan §5.8, §5.9, §5.10, §5.14): a
  new `AGENT-DISCIPLINE.md` §8B + §13 review bullets; the mechanical-scaffolding
  governance check section + §17 CI-expectations bullet in
  `TESTING-REPLAY-BENCHMARKING.md`; the pre-Gate-18 governance subsection + Gate
  18 admission/exit additions in `ROADMAP.md`; and the active-epoch intro,
  interlock note, spec-format paragraph, workflow rewrite, the new `8F` row, and
  the Gate-18 block in `specs/README.md`.
- **Template amendments** (plan §5.15–§5.19): make the lifecycle executable in
  `templates/README.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-MECHANICS.md`
  (reuse-first audit table + register-update target), `GAME-EVIDENCE.md`
  (pre-code audit + post-build register-freshness + prior-game disposition), and
  `AGENT-TASK.md` (new-game scaffolding reuse/track section), plus an explicit
  no-change proof for `templates/PRIMITIVE-PRESSURE-LEDGER.md`.
- **Mechanical enforcement** (plan §6): add `ci/scaffolding-audits.json`
  (non-runtime evidence receipt), `scripts/check-scaffolding-governance.mjs`,
  `scripts/check-scaffolding-governance.test.mjs`, `scripts/testdata/scaffolding-governance/**`
  fixtures, and the Gate 1 `repo-checks` invocation in
  `.github/workflows/gate-1-game-smoke.yml` (a new Node repo check grouped with
  the existing `scripts/check-*.mjs` checks — inserted immediately before the
  `Docs link check` step, i.e. after the boundary/WASM/UI/preview/e2e steps, not
  adjacent to `Engine boundary`).
- **Legacy bootstrap** (plan §6.4): represent the frozen 17-game corpus via
  `coverage: "legacy-8c-covered"` pointers to the existing 8C/R1–R4 evidence, with
  the legacy set frozen so no game added after 8F can claim it; future games must
  use `coverage: "forward-v1"`.
- Reconcile doc links, catalog docs, boundary check, tracker state, and final
  Unit 8F closeout evidence.

### Out of scope (the next units, not this pass)

| Surface | Why deferred | Where it lands |
|---|---|---|
| Gate 18 / Spades design, rules, UI, bots, fixtures, traces, benchmarks, docs | Gameplay | Existing Gate 18 seed row (becomes the first `forward-v1` audit) |
| Extraction, promotion, redesign, or relocation of any Rust helper | Code + ADR-gated migration | A future scaffold-refactor unit queued by a real future game closeout |
| A fresh 17-game semantic-duplication audit | Mass retrofit already done by 8C/R1–R4 | N/A — legacy set is frozen and pointer-only |
| Migration of existing game call sites beyond the 8C/R1–R4 evidence pointers | Game code + ADR 0009 migration | A future bounded refactor unit |
| Generic clone detection, AST-wide semantic-equivalence inference, "CI decides architecture" | Infeasible / out of the tractable-gate design (plan §6.2) | Never (explicitly rejected) |
| Any runtime configuration format, policy engine, or scaffolding-registry DSL | Violates the static-data / no-DSL law | Never (explicitly rejected) |
| Reopening or amending ADR 0009 | No byte/hash/fixture/export/RNG/visibility migration is authorized | Future ADR-0009-gated migration unit |

### Not allowed

This governance pass MUST NOT:

- change, paraphrase, weaken, or bypass the behavioral third-use hard gate in
  `docs/MECHANIC-ATLAS.md` §4 (it stays byte-for-byte), or alter §5A;
- change ADR 0008's allowed homes, Non-Promotion List, semantic-identity rule,
  second-use review, or pre-third-copy hard-decision threshold — the extension is
  append-only and operational, not a reversal;
- add any game/mechanic/genre/rule/scoring/trick/betting/pot/team/topology/
  accounting/reaction/deal/reveal/projection noun to `engine-core`, or promote any
  helper into `game-stdlib` outside the `MECHANIC-ATLAS.md` discipline;
- move legality, outcome, visibility, or rule decisions into TypeScript;
- add YAML, a selector/condition/trigger DSL, generated policy code, or a runtime
  scaffolding registry — `ci/scaffolding-audits.json` is a finite reviewed
  evidence receipt that rejects unknown fields by default and selects no behavior;
- change RNG draw order, serialization, stable bytes, trace schema, fixture
  hashes, replay export, viewer authorization, no-leak behavior, or benchmark
  floors; any future such change routes through ADR 0009 with explicit migration
  evidence;
- regenerate goldens or fixtures to make the governance unit pass;
- use an environment variable, workflow flag, branch label, comment directive, or
  undocumented allowlist to bypass the new check (plan §6.8);
- grow the frozen `legacy-8c-covered` set, or let a post-8F game claim legacy
  coverage;
- author or implement Gate 18 before Unit 8F is `Done`;
- weaken or supersede a FOUNDATIONS principle or an accepted ADR by **editorial
  edit** — the only meaning change here is additive (three new invariants / four
  new stop conditions), gated by the ADR 0008 extension; route any deeper meaning
  change through a named-section ADR supersession.

## Deliverables

Authority-ordered per the change plan §5 disposition matrix. `/reassess-spec`
verifies each target against live `main`; the plan's §-locations and exact draft
blocks are anchors. **Dependency order is load-bearing — the ADR extension is
appended and accepted before the foundation edits it gates.**

| # | Artifact | Required change | Plan §  |
|---:|---|---|---|
| D1 | `docs/adr/0008-mechanical-scaffolding-governance.md` | **Append-only** dated forward-obligation extension: status-note addition (§7.2), complete `Affected foundation sections` replacement (§7.3), forward per-new-game obligation subsection at the end of `Decision` before `Alternatives considered` (§7.4), migration-matrix rows (§7.5). Original 2026-06-22 context/decision text untouched; behavioral third-use gate unchanged. | §7.1–§7.5 |
| D2 | `docs/FOUNDATIONS.md` | §11: add the three forward invariants after the existing mechanical-scaffolding invariant (L204–205). §12: add the four forward stop conditions after the promotion-debt stop conditions. §4 and §13 unchanged. | §5.2 |
| D3 | `docs/ARCHITECTURE.md` | §3A truthfulness fix (`game-test-support` "future" → present-tense); new §3B *Forward mechanical-scaffolding conformance* after §3A, before §3.1; §14 acceptance-check bullet. | §5.3 |
| D4 | `docs/ENGINE-GAME-DATA-BOUNDARY.md` | New §13A *Forward mechanical-scaffolding conformance boundary* after §13, before §14, parallel to (not replacing) the behavioral §13 process. | §5.4 |
| D5 | `docs/OFFICIAL-GAME-CONTRACT.md` | §3 workflow block: add the reuse-first audit and scaffolding-closeout steps; add the *Mechanical-scaffolding forward obligation* subsection after the workflow explanation; replace the §12 mechanic/scaffolding acceptance bullet cluster. | §5.5 |
| D6 | `docs/MECHANIC-ATLAS.md` | New §5B *Parallel mechanical-scaffolding check* after §5A; §11 stage-advancement bullets. **§4 and §5A word-for-word unchanged** (preservation criterion). | §5.6 |
| D7 | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | First-use-safe `candidate` decision-state definition; Forward Per-Game Maintenance Cadence + Automatic Prior-Game Refactor Trigger sections (after Non-Promotion List, before Current Entries); Current Entries intro paragraph; Review Checklist additions. | §5.7 |
| D8 | `docs/AGENT-DISCIPLINE.md` | New §8B *New-game scaffolding reuse-and-track protocol* after §8A (applies even when `Task profile` ≠ `scaffold-refactor`); §13 review-check bullets. | §5.8 |
| D9 | `docs/TESTING-REPLAY-BENCHMARKING.md` | *Mechanical-scaffolding governance check* section (two enforcement layers, false-positive controls, no-bypass clause, ADR-0009 deferral) immediately before/after §17; §17 CI-expectations bullet. Final §-numbering normalized at reassess. | §5.9 |
| D10 | `docs/ROADMAP.md` | Pre-Gate-18 forward scaffolding-reuse governance subsection (purpose + exit + not-allowed) after Gate 17, before Gate 18; Gate 18 admission sentence; Gate 18 exit bullet. **Reconcile with the existing "Pre-Gate-18 debt interlock" prose note (ROADMAP §1, currently L47-57)** — that note already references mechanical-scaffolding debt + ADR 0008 + the register in a *retroactive* (Gates 15-17 debt-closure) framing; the new forward subsection must extend or cross-reference it (one coherent pre-Gate-18 obligation), not stand as a second divergent statement. | §5.10 |
| D11 | `specs/README.md` | Active-epoch intro/interlock-note replacement; insert the `8F` row after `8C-R4`; replace the Gate 18 row to block on `8F`; spec-format paragraph requiring per-new-game audit fields; Workflow-section rewrite folding in the scaffolding audit. On **exit**, flip `8F` to `Done`. (The `8F` row is added `Planned` and the Gate-18 block applied at spec-authoring time — see Documentation updates.) | §5.14 |
| D12 | `templates/README.md` | Add the register to the authority list after `MECHANIC-ATLAS.md`; add the two-stage audit lifecycle paragraph; replace the `GAME-MECHANICS` / `GAME-IMPLEMENTATION-ADMISSION` / `AGENT-TASK` / `GAME-EVIDENCE` index rows. | §5.15 |
| D13 | `templates/GAME-IMPLEMENTATION-ADMISSION.md` | Replace the "Novel Mechanics and Pressure" table with the reuse-first-audit gating rows; add the three required-evidence-profile rows; add the admission-blocked paragraph. | §5.16 |
| D14 | `templates/GAME-MECHANICS.md` | Add the *Mechanical scaffolding reuse-first audit* table + audit rules; replace "Required repo atlas update" with the atlas/register update section; append the review checks. | §5.17 |
| D15 | `templates/GAME-EVIDENCE.md` | Replace the "Mechanic and Scaffolding Decisions" table with the pre-code-audit / register-freshness / prior-game-disposition / CI-audit rows; append the receipt-review checks. | §5.18 |
| D16 | `templates/AGENT-TASK.md` | Add the register to the authority list; add the *New-game scaffolding reuse/track status* section; add acceptance-evidence, implementation-boundary, documentation-required rows; append forbidden-changes + review checks. Keep the existing Scaffold-Refactor Profile. | §5.19 |
| D17 | `templates/PRIMITIVE-PRESSURE-LEDGER.md` | **No change.** Record an explicit no-change proof: it already redirects behavior-free plumbing to the register and keeps the behavioral third-game gate authoritative; adding forward-scaffolding fields would create two competing owners. | §5.20 |
| D18 | `ci/scaffolding-audits.json` (new) | Non-runtime evidence receipt (`schema_version: 1`), bootstrapped to the frozen 17-game `legacy-8c-covered` pointers (plan §6.3, §6.4). No selectors/formulas/triggers; unknown fields rejected. | §6.3, §6.4 |
| D19 | `scripts/check-scaffolding-governance.mjs` (new) | Schema / set-equality vs `ci/games.json` + real `games/` dirs / path-ID / register-freshness / prior-game-scheduling / migration-authority / known-shape fingerprint checks; compact success summary; no env/label bypass. | §6.5, §6.6, §6.9 |
| D20 | `scripts/check-scaffolding-governance.test.mjs` + `scripts/testdata/scaffolding-governance/**` (new) | Node test suite + minimal synthetic fixtures: passing receipts, missing-game, stale/missing paths, unknown IDs, unqueued prior site, invalid exception, forbidden legacy claim, and false-positive (legitimate behavior-bearing local code) cases. | §6.5–§6.7, §6.10 |
| D21 | `.github/workflows/gate-1-game-smoke.yml` | **Amend.** One `repo-checks` step (`node scripts/check-scaffolding-governance.mjs`) inserted **immediately before the `Docs link check` step**, grouped with the existing Node `scripts/check-*.mjs` repo/doc checks (plan §6.1's "conceptually adjacent to `boundary-check.sh`, `check-doc-links.mjs`, `check-catalog-docs.mjs`"). Note: in the live workflow `Engine boundary` (L170-171) and the doc-drift checks (~L191) are **not** adjacent — four smoke/e2e steps sit between them — so "after `Engine boundary`" alone is under-specified; the `Docs link check` anchor pins the single insertion site. Gate 0 and Gate 2 unchanged. | §6.1, §6.10 |

### Foundational-document changes (embedded for the constitution and governing decision)

The user directive asks that foundational-document changes be included in the
spec. The full per-file exact draft blocks live in change-plan §5–§7 (carried as
anchors that `/reassess-spec` re-verifies). The two load-bearing **foundational**
changes — the constitution and the governing decision — are embedded here so the
spec is self-authoritative on them:

**`FOUNDATIONS.md` §11 — three forward invariants (D2), inserted after L204–205:**

```markdown
- Every new official game completes a mechanical-scaffolding reuse-first audit
  before serious implementation. The audit reviews the mechanical-scaffolding
  register and the lawful shared homes, reuses matching promoted scaffolding or
  records an accepted exception, and identifies any new behavior-free
  scaffolding the game will introduce.
- Every new behavior-free scaffolding shape introduced by an official game is
  recorded in the mechanical-scaffolding register with behavior exclusions,
  affected hash/visibility/determinism surfaces, a decision state, and a next
  review trigger. First use does not authorize promotion.
- When a new game's scaffolding makes an earlier official game a matching
  duplicate, the new-game closeout either queues a named bounded follow-on
  refactoring unit or records an accepted `local-only`, `deferred`, or `rejected`
  disposition with rationale and next review trigger. Existing pre-third-copy
  and promotion-debt blocking rules remain in force.
```

**`FOUNDATIONS.md` §12 — four forward stop conditions (D2):**

```markdown
- a new official game starts serious implementation without a completed
  mechanical-scaffolding reuse-first audit;
- a new official game closes while a newly introduced behavior-free scaffolding
  shape is absent from the mechanical-scaffolding register;
- a new official game identifies matching prior-game scaffolding but leaves the
  retrofit as an unnamed TODO instead of a tracker unit or an accepted
  no-refactor disposition;
- a known promoted scaffolding helper is reimplemented locally without a
  register-backed exception;
```

**`docs/adr/0008-*.md` — forward per-new-game obligation extension (D1),** appended
to the end of the `Decision` section before `Alternatives considered` (full text
in change-plan §7.4; the seven-point obligation — reuse-first audit, reuse-unless-
accepted-exception, first-use registration without promotion, queue real
prior-game migration, accepted no-unit disposition fields, Gate 1 receipt check,
ADR-0009 deferral — with the closing guarantee that the behavioral third-use gate
remains word-for-word effective and no new home/helper/Non-Promotion change is
authorized). The status-note (§7.2), `Affected foundation sections` replacement
(§7.3), and migration-matrix rows (§7.5) land in the same append-only edit.

## Work breakdown

Each item is a candidate AGENT-TASK; `/reassess-spec` then `/spec-to-tickets`
split these into one ticket per reviewable diff. Dependency order is load-bearing.
Task packets carry the `Scaffold-Refactor Profile` as `Governance only` / `not
applicable` — never to imply a source migration (plan §8.5).

| # | Candidate task | Depends on | Plan anchor | Notes |
|---:|---|---|---|---|
| WB1 | Freeze the amendment inventory, frozen 17-game legacy set, accepted baseline, and the exact §4 behavioral-gate text fixture (checksum/copy). | — | FSGOV-001 / §9.1 | 17-game set equals `ci/games.json`; copied §4 gate text matches source; no helper/game diff. |
| WB2 | Append the dated ADR 0008 forward-obligation extension and introduce the obligation at constitution level (`FOUNDATIONS.md` §11/§12). | WB1 | FSGOV-002 / §5.2, §7 | Authority review; affected-sections complete; original ADR decision still visible; exact §4 gate text unchanged. |
| WB3 | Land architecture, boundary, official-workflow, atlas-seam, and register-cadence amendments. | WB2 | FSGOV-003 / §5.3–§5.7 | Doc links; lane/home/Non-Promotion review; first-use + queue-or-dispose examples internally consistent; §4/§5A atlas text untouched. |
| WB4 | Land agent, testing, roadmap, and tracker law, including the blocking Gate 18 interlock. | WB3 | FSGOV-004 / §5.8–§5.10, §5.14 | Lowest-non-`Done` order shows `8F` before Gate 18; workflow includes both behavioral atlas debt and the scaffolding audit. |
| WB5 | Make the lifecycle executable in every affected per-game template; record the explicit primitive-pressure-ledger no-change proof. | WB3 | FSGOV-005 / §5.15–§5.20 | A filled synthetic template packet proves pre-code audit, post-build registration, and prior-game disposition are all non-silent. |
| WB6 | Add the versioned audit receipt and bounded historical bootstrap. | WB3, WB5 | FSGOV-006 / §6.3, §6.4 | All 17 existing games use frozen `legacy-8c-covered`; every pointer resolves; no future game can use that coverage. |
| WB7 | Implement the receipt/register/tracker validator and high-confidence promoted-shape fingerprints, with passing + failing + false-positive fixtures. | WB6 | FSGOV-007 / §6.5–§6.7 | Failures for missing game, stale/missing paths, unknown IDs, unqueued prior site, invalid exception, forbidden legacy claim; negatives for legitimate local/test repetition. |
| WB8 | Wire the checker into Gate 1 and run the complete repository verification set. | WB7 | FSGOV-008 / §6.1, §6.10 | Local checker pass; Node tests pass; existing Gate 1 checks pass; workflow syntax/ordering review. |
| WB9 | Reconcile all documents, publish closeout evidence, flip `8F` to `Done` without admitting Gate 18 early. | WB8 | FSGOV-009 / §8.8 | Exit-criteria matrix complete; no unauthorized code/data diff; Gate 18 remains `Not started`. |

## Exit criteria

Mapped to the three forward gaps (plan §8.6) plus cross-cutting preservation.

### Gap 1 — reuse-first audit is standing and blocking

- [x] `FOUNDATIONS.md`, the official-game workflow, agent law, active tracker, and
      every affected new-game template require a pre-implementation
      mechanical-scaffolding reuse-first audit.
- [x] A `not applicable` audit requires a rationale and evidence link; omission is
      not accepted.
- [x] Gate 1 fails when an official game lacks a valid audit record or a post-8F
      game claims frozen legacy coverage.
- [x] Known promoted-shape fingerprints fail when a parallel implementation has
      neither shared-helper use nor a valid register-backed exception.

### Gap 2 — register-new is standing and closed at game completion

- [x] The register and official-game closeout require every newly invented
      behavior-free scaffolding shape to receive a first-use `candidate` /
      `local-only` / `rejected` entry before the game closes.
- [x] The mechanics and evidence templates name the register as a required update
      surface and carry a post-implementation freshness receipt.
- [x] First-use registration is explicitly non-promotional and cannot place a
      Non-Promotion List behavior into a scaffolding home.
- [x] Gate 1 validates every cited register ID and declared source/evidence path
      against committed files.

### Gap 3 — prior-game refactoring is queued or explicitly disposed

- [x] The register, official-game contract, roadmap, tracker workflow, evidence
      template, and CI receipt all require prior matching official games to be
      named.
- [x] Real characterization or migration work has a named bounded unit in
      `specs/README.md` before the new game closes.
- [x] No-unit cases are accepted only through a register-backed `local-only` /
      `deferred` / `rejected` decision with rationale, owner, evidence, and a
      next-review trigger.
- [x] Gate 1 fails when a declared prior match has neither a tracker unit nor a
      valid no-unit disposition.

### Cross-cutting preservation

- [x] The exact behavioral third-use hard-gate sentence in `MECHANIC-ATLAS.md` §4
      is byte-for-byte unchanged (and §5A is unchanged).
- [x] ADR 0008's lane, allowed homes, Non-Promotion List, semantic-identity
      requirement, second-use review, and pre-third-copy threshold are unchanged;
      the extension is visibly append-only and dated.
- [x] No game, helper, TypeScript behavior, trace, fixture, hash, RNG,
      serialization, visibility, no-leak, benchmark-threshold, or catalog entry
      changes as part of Unit 8F.
- [x] `engine-core` remains noun-free and `game-test-support` remains dev-only.
- [x] No YAML, DSL, selector, condition, trigger language, or runtime governance
      registry is introduced; `ci/scaffolding-audits.json` rejects unknown fields.
- [x] Every byte/hash/visibility migration field in the new receipt says `none` or
      cites ADR 0009 plus an explicit migration artifact.
- [x] Gate 18 remains `Not started` until all Unit 8F criteria pass and the `8F`
      tracker row is `Done`.

## Acceptance evidence

Game-level evidence (rules/traces/benchmarks/bot-legality/UI smoke) is **not
applicable** — this unit has no game and writes no game/kernel/trace/fixture code.
The only new executable surface is the governance checker and its tests.

Minimum acceptance transcript (plan §8.7):

```bash
node scripts/check-scaffolding-governance.mjs
node --test scripts/check-scaffolding-governance.test.mjs
node scripts/check-ci-games.mjs
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
bash scripts/boundary-check.sh
cargo tree --workspace -e normal,build --invert game-test-support
git diff --check
```

Targeted evidence beyond command success:

- a source comparison proving the behavioral third-use sentence is unchanged;
- a changed-file audit showing no `games/**`, runtime `crates/**`, `apps/web/**`,
  trace, fixture, benchmark-threshold, or generated-artifact mutation
  (`git status --porcelain -- crates/ games/ tools/ apps/ '**/*.trace.json'`
  returns nothing beyond the new `scripts/`/`ci/` governance files);
- checker fixtures: a hypothetical new game reimplementing a known promoted shape
  **fails**; a legitimate behavior-bearing local shape outside the fingerprint
  rule **passes** with the correct register decision; a prior match naming a
  nonexistent tracker unit **fails**; a frozen legacy game pointing to its
  8C/R-wave receipt **passes**;
- a closeout matrix mapping every changed law/template/check to one of the three
  forward gaps.

If any check fails, follow the failing-test protocol (`AGENT-DISCIPLINE` §4):
determine whether the check is valid, then whether the fault is in the system
under test or the check, then fix the correct side. No task may weaken a checker
fixture merely to close the series.

## FOUNDATIONS & boundary alignment

This unit writes **doctrine + a tractable governance check**, so the alignment
below covers the *doctrine being written and the check enforcing it*, not game
code. The full read of `docs/FOUNDATIONS.md` (this session) confirms each cited
section's wording.

| Principle | Stance | Rationale |
|---|---|---|
| §11 Mechanical-scaffolding invariant (existing) | Engaged → strengthened | The new invariants extend the existing "behavior-free, registered, rejected/rerouted" invariant (L204–205) into a standing forward audit/register-new/queue-or-dispose duty; gated by the accepted-ADR-0008 append-only extension, never an editorial meaning change to an unrelated principle. |
| §11 Promotion-adoption + open-promotion-debt invariants | Aligned | The queue-or-dispose rule reinforces the existing promotion-debt-closes-before-next-gate invariant; it adds observability, not a new threshold. |
| §12 Stop conditions | Engaged → extended | Four additive stop conditions (no-audit start, unregistered new shape at close, unnamed-TODO prior match, local reimplementation of a known helper) sit beside the existing promotion-debt stop conditions; they fire *more* stops, never fewer. |
| §4 `game-stdlib` earned / behavioral third-use gate | Aligned (load-bearing, preserved) | The scaffolding lane stays parallel and narrower; §4 atlas wording and the behavioral third-use hard gate are byte-for-byte unchanged. The check explicitly excludes Non-Promotion List behavior from automatic classification. |
| §3 `engine-core` noun-free kernel | Aligned | No mechanic noun is added; `ci/scaffolding-audits.json` is CI evidence metadata, not loaded by any game/WASM/browser path; `boundary-check.sh` still gates. |
| §5 Static data is not behavior | Aligned | The audit receipt is finite reviewed metadata with no selectors/conditions/triggers and unknown-field rejection; no YAML/DSL/runtime registry is introduced. |
| §13 ADR triggers | Engaged → ADR 0008 extension | Operationalizing an accepted ADR's decision and amending the constitution's invariants routes through the append-only ADR 0008 extension authored/accepted first; any future byte/hash/visibility migration routes through ADR 0009. |

§12 preservation: stop if any edit would change a trace field/hash, weaken the
behavioral gate, relabel behavior (teams/reveal/betting/scoring) as plumbing, or
move behavior into data — those route to ADR 0009 or stay game-local
(Non-Promotion List). No "generalize the engine" code work happens here.

## Forbidden changes

Do not:

- change, paraphrase, or weaken the behavioral third-use hard gate, ADR 0008's
  lane/homes/Non-Promotion List/thresholds, no-leak, determinism, replay/hash
  proofs, or the no-public-MCTS/ML/RL bot law;
- supersede or weaken a FOUNDATIONS principle or accepted ADR by editorial edit —
  the only meaning change is additive and gated by the append-only ADR 0008
  extension; any deeper change routes through a named-section ADR supersession;
- add mechanic nouns to `engine-core` or promote helpers outside atlas discipline;
- **implement** any scaffolding extraction, helper, crate, harness, or per-game
  retrofit — this unit institutionalizes the obligation; extraction is a future
  ADR-gated unit;
- change any `*.trace.json` fixture, trace-schema field bytes, hash, RNG,
  serialization, export, visibility, or benchmark output;
- introduce YAML/DSL/selectors/conditions/triggers/formulas or a runtime registry;
- add an environment/branch-label/comment bypass to the checker, or grow the
  frozen legacy set;
- write or edit game code, Rust crates, WASM, or `apps/web/**` runtime; edit
  archived specs; rewrite `docs/ROADMAP.md` as a progress diary;
- delete or weaken tests/checks/fixtures to make validation pass;
- author or implement Gate 18 before `8F` is `Done`.

## Documentation updates required

- **New:** `docs/` — no new doc files (the register/ADR/contract docs all exist);
  `ci/scaffolding-audits.json`, `scripts/check-scaffolding-governance.mjs`,
  `scripts/check-scaffolding-governance.test.mjs`,
  `scripts/testdata/scaffolding-governance/**`.
- **Edited (docs/law):** `docs/adr/0008-mechanical-scaffolding-governance.md`
  (append-only), `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`,
  `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/OFFICIAL-GAME-CONTRACT.md`,
  `docs/MECHANIC-ATLAS.md` (§4/§5A unchanged),
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/AGENT-DISCIPLINE.md`,
  `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/ROADMAP.md`.
- **Edited (templates):** `templates/README.md`,
  `templates/GAME-IMPLEMENTATION-ADMISSION.md`, `templates/GAME-MECHANICS.md`,
  `templates/GAME-EVIDENCE.md`, `templates/AGENT-TASK.md`; explicit no-change
  proof for `templates/PRIMITIVE-PRESSURE-LEDGER.md`.
- **Edited (CI):** `.github/workflows/gate-1-game-smoke.yml` (one `repo-checks`
  step, inserted immediately before the `Docs link check` step — see D21).
- **Edited (index):** `specs/README.md` — at spec-authoring time the `8F` row is
  added (`Planned`) and the Gate 18 row is updated to block on `8F`; the deeper
  intro / interlock-note / spec-format / workflow rewrites (plan §5.14 Locations
  1, 4, 5) land during implementation; on exit the `8F` row flips to `Done`.
  Locations 2 (the `8F` row) and 3 (the Gate 18 block) are **already committed**
  in the live index (`specs/README.md:103`–`:104`), so the implementing ticket
  treats them as verify-only; only Locations 1/4/5 carry real edits.
- **Not applicable:** this unit has no web-exposed game gate, so the web-shell
  catalog README (`apps/web/README.md`) is not a closeout surface
  (`check-catalog-docs.mjs` still runs as a guard).
- **Not applicable:** `docs/adr/0009-*.md` and `docs/adr/ADR-TEMPLATE.md` — no
  amendment authorized; the new law only *defers* byte/hash/visibility migration
  to ADR 0009 (plan §5.12, §5.13).

## Sequencing

- **Predecessor:** `8C-R4` is `Done` (2026-06-24); all four C-11 retrofit waves
  are closed/disposed, clearing the final C-11 Gate 18 interlock. ADRs 0008 and
  0009 are accepted; the register and 17-game corpus exist. This unit is a forward
  delta on that shipped retroactive program.
- **This unit:** governance / law / template / CI-receipt interlock. No game, no
  helper, no fixture/hash migration.
- **Successor:** Gate 18 (Spades / partnerships) — the **first** game admitted
  under the standing rule, carrying the first real `forward-v1` reuse-first audit.
- **Admission rule:** Gate 18 may be authored/implemented only when `8F` is
  `Done`. Unit 8F MUST NOT pre-decide any Spades-specific helper adoption,
  exception, register entry, or retrofit unit — Spades exercises the mechanism, it
  is not pre-audited here. The intended sequence (plan §8.8):

```text
author this spec under specs/
  -> run /reassess-spec in place
  -> accept the corrected spec
  -> run /spec-to-tickets
  -> execute bounded Unit 8F tasks
  -> satisfy and record all exit criteria
  -> flip 8F to Done
  -> author/reassess Gate 18 as the first forward-v1 game
```

## Assumptions

- A1 (one-line-correctable): The deliverable is a **single** governance unit (`8F`)
  covering all of change-plan §5–§8 — assuming the maintainer wants the
  authority-ordered amendment set, templates, and CI check landed together (as one
  reviewable-per-diff series), matching the 8M precedent. Alternative considered:
  split the CI check (§6) into a separate tooling unit; rejected because the law
  amendments reference the receipt/check and would ship incomplete without it.
- A2 (resolved at `/reassess-spec`, 2026-06-25): The ADR mechanism is the
  **append-only ADR 0008 extension** (plan §7.1 default), not a new successor ADR.
  The immutable-accepted-ADR check ran this session and found **none**:
  `docs/adr/ADR-TEMPLATE.md` carries only a "Superseded decision, if any" field
  under "Required scaling / supersession fields" (no "must-not-edit-once-accepted"
  rule), and ADR 0008 itself records `Superseded decision: none`. **Default holds —
  D1's append-only extension stands; the §7.6 fallback short ADR `0010` is not
  triggered.**
- A3 (one-line-correctable): The constitution change is **additive** (three new
  §11 invariants + four new §12 stop conditions) and therefore meaning-preserving
  for existing principles given the accepted ADR 0008 extension — assuming any edit
  that would change an existing invariant's meaning beyond the additive duty is
  split out and ADR-gated rather than landed here.
- A4 (one-line-correctable): `Spades` is the working Gate-18 game; the actual
  Rulepath neutral game ID for Gate 18 is chosen in the Gate 18 spec, not here —
  assuming `8F` must not bind a Gate-18 game ID. The `legacy-8c-covered` freeze and
  `forward-v1`-for-future-games rule are ID-agnostic.
- A5 (one-line-correctable): The change plan's per-file evidence (commit `5ed1664`,
  current `main` per the dated manifest) is re-verified per target file at
  `/reassess-spec` time; the plan's §-locations are traceability anchors, not a
  substitute for the live re-read. The full per-file exact draft blocks (plan
  §5–§7) are the authoritative drafts; this spec embeds only the constitutional and
  ADR-decision changes verbatim.
- A6 (one-line-correctable): `ci/scaffolding-audits.json` is a finite reviewed
  evidence receipt, not runtime data — assuming the unknown-field-rejection +
  no-selectors design (plan §6.3, §10.6) keeps it on the static-data side of the
  no-DSL law; if reassessment finds it trends toward behavior, escalate before
  landing it.
- A7 (validated this session): edit-target premises confirmed present at `5ed1664`
  — `ci/games.json` (17 games), the Gate 1 `repo-checks` job with `Engine
  boundary`/`boundary-check.sh` (lines 170–171) before the doc-drift checks, ADR
  0008 `Status: Accepted` with `Affected foundation sections` / `Migration matrix:`
  / `## Decision` → `## Alternatives considered`, `FOUNDATIONS.md` §11 mechanical-
  scaffolding invariant (L204–205) and §12 promotion-debt stop conditions, and the
  `scripts/check-*.mjs` / `boundary-check.sh` acceptance commands.

## Outcome

Completed 2026-06-25. Unit 8F is closed as a governance-only interlock: ADR
0008 carries the dated append-only forward-obligation extension; `FOUNDATIONS.md`
§11/§12 carry the forward invariants and stop conditions; the
architecture/boundary/contract/atlas/register/agent/testing/roadmap docs and the
five per-game templates carry the standing lifecycle; `ci/scaffolding-audits.json`
and `scripts/check-scaffolding-governance.*` exist with the frozen 17-game legacy
bootstrap and Gate 1 wiring; `specs/README.md` marks `8F` `Done`; Gate 18 remains
`Not started`.

Three-gap closeout map:

| Gap | Closed by |
|---|---|
| reuse-first audit | ADR/foundation stop conditions; official-game workflow; agent law; roadmap/tracker interlock; `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-MECHANICS.md`, `GAME-EVIDENCE.md`, and `AGENT-TASK.md`; Gate 1 checker receipt enforcement. |
| register-new | register cadence and first-use-safe decision states; mechanics/evidence/admission template rows; `ci/scaffolding-audits.json` register fields; checker path/MSC validation and unknown-field rejection. |
| queue-or-dispose | register prior-game trigger; official-game closeout and roadmap/tracker workflow; evidence/admission/template rows; checker prior-match follow-on/no-unit validation; Gate 18 blocked until `8F` is `Done`. |

`templates/PRIMITIVE-PRESSURE-LEDGER.md` was intentionally unchanged. It already
states behavioral-scope-only ownership, rejects behavior-free plumbing there, and
routes mechanical scaffolding to `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`; adding
forward-scaffolding lifecycle fields would create two competing owners and risk
weakening the behavioral/scaffolding distinction.

Verification transcript:

- `node scripts/check-scaffolding-governance.mjs` — OK (`games=17`, `legacy=17`, `forward-v1=0`).
- `node --test scripts/check-scaffolding-governance.test.mjs` — 9 tests passed.
- `node scripts/check-ci-games.mjs` — 17 games in sync with `games/`.
- `node scripts/check-doc-links.mjs` — checked 31 markdown files.
- `node scripts/check-catalog-docs.mjs` — catalog/docs check passed for 17 games.
- `bash scripts/boundary-check.sh` — `engine-core` boundary and `game-test-support` dev-only checks passed.
- `cargo tree --workspace -e normal,build --invert game-test-support` — only `game-test-support v0.1.0` printed.
- `git diff --check` — clean.
- `git status --porcelain -- crates/ games/ tools/ apps/ '**/*.trace.json'` — no output.
- `grep -Fx` confirmed the `MECHANIC-ATLAS.md` third-official-game hard-gate row still exactly matches the change-plan fixture.
- `git diff -- docs/MECHANIC-ATLAS.md docs/adr/0008-mechanical-scaffolding-governance.md templates/PRIMITIVE-PRESSURE-LEDGER.md` — no output at capstone closeout.

No game, helper, TypeScript behavior, trace, fixture, hash, RNG, serialization,
visibility, no-leak, benchmark-threshold, catalog-entry, YAML, DSL, or runtime
governance registry change was introduced by this capstone.
