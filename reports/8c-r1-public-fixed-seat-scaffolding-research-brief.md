# Research brief — Unit 8C-R1 (C-11 follow-on: public/fixed-seat scaffolding) next spec

> **You are Session 2: a locked deep-research-and-author session.** Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-23_820487d.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `820487d` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `820487d`, not the cited string.

**The determination is already made (do not re-open it).** This brief does not ask you to
decide *what* to build next — that decision is locked. The repository workflow
(`specs/README.md` → "Workflow": *pick the lowest non-`Done` unit from the active-epoch
tracker*) selects the next unit deterministically, and that unit is **Unit 8C-R1 — "C-11
follow-on — public/fixed-seat scaffolding."** Your job is to **confirm-and-document** that
determination with cited evidence and then **author the bounded implementation spec for
Unit 8C-R1**. The evidence that fixes the determination, all verifiable in the tree at
`820487d`:

- `specs/README.md` active-epoch tracker: rows above 8C-R1 (`8C`, `8M`, Gate 15/15.1/16/17,
  Infra A–D, Phase 0) are all `Done`; **8C-R1 is the lowest row whose status is `Not started`**
  (a `(seed; unwritten)` row). 8C-R2/R3/R4 sit below it; Gate 18 (Spades) sits below those and
  is explicitly *"Pending … 8C-R1…8C-R4 closed / explicitly not applicable / accepted-excepted."*
- `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register: **`Current debt: None`** (last
  reviewed at Gate 17 Vow Tide closeout). No primitive-promotion debt is open, so nothing must
  close ahead of 8C-R1. (8C-R1 is a *code-scaffolding retrofit*, not a mechanic-ladder gate, so
  the atlas "close debt before the next mechanic-ladder gate" interlock does not gate it either.)
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` (the parent spec): §5
  "Required C-11 forward-wave seeds" defines 8C-R1's exact game set and audit shape; work item
  **8C-030** created the four `8C-R1…R4` seed rows in `specs/README.md`; exit criteria **EC-28**
  ("Four bounded C-11 seeds cover every official game exactly once … They remain unimplemented")
  and **EC-30** ("Gate 18 remains after the seeded C-11 waves are closed, not applicable, or
  explicitly excepted") fix the sequencing.

So this is a **new-spec** task whose subject is a behavior-free **mechanical-scaffolding
retrofit/audit unit**, not a new game.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground
truth. The user named **`docs/**` (all files and folders), `templates/**`, and
`specs/README.md`** as the mandatory floor; read the entire `docs/**` and `templates/**` trees.
Each line below states why a file is load-bearing *for the 8C-R1 spec*; files in the named
floor that only bear incidentally are covered by one directory-level reason per tier.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect; the ADR status index (0004/0008/0009 all `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — boundary & scaffolding law (the crux for this unit)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the Rust/WASM boundary, and the action/view/effect/replay/determinism model; the "narrowest lawful owner wins" ownership matrix the retrofit must honor.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; the retrofit may adopt shared helpers but must never move game behavior across this line or add nouns to `engine-core`.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law; **§10** initial atlas table (board-space promotion already closed for three_marks/column_four/directional_flip; public-accounting deferred for token_bazaar), **§10A** empty debt register, and the non-promotion boundary the scaffolding lane must not breach.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — **the governing register for this unit**: the Entry Schema, Decision States, Non-Promotion List, the landed `MSC-8C-001…010` entries, and the per-entry "next review trigger = C-11 game audits" — every R1 migration adds/updates a register entry here.
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted governance ADR for the scaffolding lane; defines what may be extracted and the register-first discipline.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2; **every hash-bearing surface flip in R1 follows its per-surface migration protocol** (characterize → parallel surface → classify unchanged/parallel-new/intentional-migration → compatibility window → rollback).
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — hidden-information replay/export visibility taxonomy; load-bearing because the C-08 profile drivers and any export-surface audit must preserve it (these six games are public, but public-export observer-safety still applies).

**Tier 3 — evidence, determinism & multi-seat law**
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden-trace obligations, deterministic replay/hash discipline, no-leak tests, benchmarks, CI; the acceptance-evidence backbone of the spec.
- `docs/TRACE-SCHEMA-v1.md` — trace/replay-fixture schema law; load-bearing because C-04/C-05/C-08 audits touch trace bytes and any change is an ADR-0009 migration packet, never a silent edit.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the evidence-fixture profile contract (ADR 0009); separates command traces, setup/domain fixtures, and viewer-scoped exports — the C-08 profile-driver audit conforms to this.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat/surface obligations; mostly fixed-two/fixed-seat for R1's games, but the canonical-seat-grammar (C-02) and seat-count (C-03) audits must respect its seat-declaration and viewer rules.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, the failing-test protocol the spec's work breakdown and "one surface per diff" rollback rule must encode.

**Tier 4 — the named floor remainder + planning artifacts**
- Remaining `docs/**` — `OFFICIAL-GAME-CONTRACT.md`, `AI-BOTS.md`, `UI-INTERACTION.md`, `IP-POLICY.md`, `SOURCES.md`, `WASM-CLIENT-BOUNDARY.md`, `ROADMAP.md`, `archival-workflow.md`, `adr/0001…0003/0005/0006/0007`, `adr/ADR-TEMPLATE.md` — part of the mandatory `docs/**` floor; mostly background for this behavior-free unit (no new game, no UI, no bot change). Read for completeness; `WASM-CLIENT-BOUNDARY.md` is the one with direct bearing because C-02's legacy seat-alias adapter lives at the `wasm-api` boundary.
- All of `templates/**` — `AGENT-TASK.md` is the packet shape the spec's work breakdown decomposes into (each work item is a candidate AGENT-TASK); the `GAME-*` and evidence/admission templates show the evidence vocabulary the spec's acceptance section should reference. Part of the mandatory floor.
- `specs/README.md` — the living tracker (proves 8C-R1 is the lowest non-`Done` unit), the 12-section **Spec format** your deliverable must follow, and the **Workflow** that fixes the determination.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` — **the parent spec; read it closely.** §5 "Required C-11 forward-wave seeds" (8C-R1's exact game set + admission/exit shape), the §4.3 API defaults for C-01…C-05, the §5 ADR-0009 migration protocol (Wave-2 subsection), the pilot-discharge rules ("pilots discharge only their named surfaces"), and EC-28/EC-30. R1 is the *residual audit* of surfaces these pilots did **not** cover.
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the source report for Part C / C-01…C-11; the origin of the C-11 retrofit obligation R1 discharges.
- `reports/8c-mechanical-scaffolding-characterization.md` — the 8C characterization packet; the **precedent format** for the per-surface characterization R1 requires before any migration (bytes/hashes/seat-spellings/RNG-vectors).

**Code seams to inspect directly (read in the repo; *inspect, not read-fully*, and not pasted here):**
- For **each of the six R1 games** (`games/race_to_n`, `games/draughts_lite`, `games/three_marks`, `games/column_four`, `games/directional_flip`, `games/token_bazaar`): `src/effects.rs` (C-01 envelope-constructor candidates), `src/ids.rs` (C-02 seat-ID formatting), `src/setup.rs` (C-03 seat-count/validation), `src/replay_support.rs` + `tests/golden_traces/*.trace.json` (C-04/C-05 action-tree & stable-byte/hash surfaces; C-08 replay-command profile), `data/manifest.toml` + `data/fixtures/*` where present (C-08 fixture/profile surfaces). These reveal which helper applies to which surface and which are already canonical / already shipped.
- Shared landing homes already in place: `crates/engine-core/src/{action.rs,replay.rs,rng.rs}` (C-01/C-04/C-05/C-09 homes), `crates/game-stdlib/src/seat.rs` (C-03 home), `crates/game-test-support/src/{no_leak.rs,profiles.rs}` (C-07/C-08 homes — note C-07 is two-seat/N-seat hidden-info, largely R2/R4 not R1), `crates/wasm-api/src/seats.rs` (C-02 legacy-alias adapter + per-game trace seat emission).
- Tooling & guards: `tools/replay-check`, `tools/fixture-check` (validator owners; thin profile dispatch only), `scripts/boundary-check.sh` (noun-free kernel gate), `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs` (note: R1 adds no game, so the catalog check is a regression guard, not a closeout surface).

---

## 3. Settled intentions (these make this brief locked — do not re-decide them)

1. **Target locked: Unit 8C-R1**, "C-11 follow-on — public/fixed-seat scaffolding." Confirm-and-document the determination with the §1 evidence; **do not** re-open "what's next," and **do not** expand into 8C-R2/R3/R4 or Gate 18. This is plumbing/audit work, not a new game.
2. **Bounded game set — exactly six, none added, none dropped:** `race_to_n` (residual audit), `draughts_lite` (residual audit), `three_marks`, `column_four`, `directional_flip`, `token_bazaar`. (`race_to_n` and `draughts_lite` are "residual" because their 8C pilots already discharged named surfaces; R1 audits only what those pilots did **not** cover.)
3. **Scope = applicability audit + per-surface migration of the public/fixed-seat helper subset:** **C-01** (effect-envelope constructors), **C-02** (canonical `seat_<n>` grammar + WASM import-alias adapter), **C-03** (`SeatCount`/range/ring-index plumbing), **C-04** (action-tree encoding/hash v1), **C-05** (`StableBytesWriter` v1), **C-08** (evidence-profile drivers, primarily `replay-command-v1` and the public/setup/domain profiles these games actually have). C-06 (the test-support crate) and C-07 (pairwise no-leak geometry) already exist and are largely hidden-info concerns for R2/R4 — touch them in R1 only if a public game has a real applicable surface; otherwise mark `not applicable` with rationale. C-09 (unbiased RNG) and C-10 (non-promotion affirmation) carry forward as audit checkpoints only — no public/fixed-seat game in this set adopts the unbiased sampler in R1 unless a real characterized surface demands it.
4. **Per (game × applicable helper) the audit resolves to exactly one of:** *migrate one surface per reviewable diff*, *record accepted not-applicable*, or *record accepted exception with named review trigger* — matching the seed's admission/exit ("migrate one surface per diff or record accepted not-applicable/exception") and the parent spec's ADR-0009 per-surface migration protocol. Use explicit `not applicable` rows over silent omissions (`specs/README.md` spec-format rule). Every official surface in the six games is assigned a verdict; there is no unowned "remaining cleanup" bucket.
5. **Do not re-recommend already-shipped 8C work as if missing (the sharp-delta rule).** The following landed in Unit 8C (`Done`, 2026-06-22) and must be *built on*, not rebuilt: the C-01…C-10 infrastructure in `engine-core`/`game-stdlib`; the dev-only `crates/game-test-support` crate; register entries `MSC-8C-001…010`; and the 8C pilots (Race to N, Draughts Lite, High Card Duel, River Ledger, Vow Tide, Briar Circuit) — each of which **discharges only its named surfaces**. The Gate 7.1 `game-stdlib::board_space` back-port for `three_marks`/`column_four`/`directional_flip` is also shipped and **out of R1 scope**. R1 audits the *residual, unapplied* surfaces only.
6. **Hard constraints carried into the spec** (from FOUNDATIONS, the boundary doc, ADR 0008/0009/0004, and the parent spec's "Not allowed" list): no behavioral promotion through the scaffolding lane; no shared helper that decides legality/setup/reveal/projection/scoring/outcome/bot-choice; **no silent byte/hash/seat-ID/visibility/RNG-consumption change** — every hash-bearing flip is a named ADR-0009 migration with characterization, before/after evidence, compatibility window, and rollback; **no blanket golden regeneration or "update snapshots" sweep**; `engine-core` stays noun-free; one surface per reviewable diff; no test deletion/weakening (AGENT-DISCIPLINE failing-test protocol); no YAML/DSL/selectors/triggers in data.
7. **Documentation amendments are routine and spec-driven only:** new/updated `MSC-` entries in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` per migrated surface, and `specs/README.md` status flips. **No foundation-doc or ADR amendment is expected.** If you find a foundation/ADR change is genuinely required, do **not** design against doctrine — surface it inside the spec as a flagged FOUNDATIONS §13 ADR-trigger / blocking note for maintainers, per the user's "unless the docs themselves need to be amended, in which case it should be included in the spec."
8. **Deliverable framing — intermediate spec artifact (not a final drop-in file).** See §7.
9. `assumption:` the unit slug/label defaults to `8c-r1-public-fixed-seat-scaffolding`; the exact label is one-line-correctable downstream at `/reassess-spec` time. (`8C-R1` is the fixed unit ID from the tracker.)

---

## 4. The task

Author the bounded **implementation spec for Unit 8C-R1** (`new-spec`, subject = behavior-free
mechanical-scaffolding retrofit/audit). The spec turns the 8C-R1 seed row into a concrete,
reviewable plan: it confirms-and-documents the determination with cited evidence; states the
objective and the locked six-game scope; lays out the applicability audit of helpers
C-01…C-05/C-08 per game (with a per-surface verdict matrix); decomposes the work into bounded
candidate AGENT-TASK items each naming exact files/symbols/affected-hash-and-visibility
surfaces/rollback scope and obeying the ADR-0009 one-surface-per-diff migration protocol; and
maps exit criteria and acceptance evidence (commands, focused tests, register entries) to the
seed's admission/exit and to the parent spec's EC-28/EC-30 sequencing. The spec must be aligned
with `docs/**`; where a doc genuinely needs amending, fold that into the spec as a flagged
ADR-trigger rather than editing doctrine.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular, read
the actual current surfaces of the six R1 games to ground every per-surface verdict (do not
guess applicability; verify it against `src/*.rs`, `data/`, and `tests/golden_traces/*`).

Research online as deeply as it sharpens the deliverable — but calibrate to the target: this is
a behavior-free plumbing retrofit governed by already-accepted ADRs, so external prior art is a
sharpening aid, not the crux. Useful angles if you pursue them: characterization-testing
discipline for legacy byte/hash surfaces (Feathers-style), canonical/versioned serialization
framing, and shared dev-only test-scaffolding patterns. Cite any external source that shapes a
recommendation. Do not let online research expand the locked scope.

---

## 6. Doctrine & constraints (honor; trimmed to what this unit engages)

- `docs/FOUNDATIONS.md` is the constitution — every decision must satisfy its §11 universal
  invariants and clear its §12 stop conditions; a genuine divergence requires an accepted ADR
  superseding the affected principle first (never design against it silently). If you hit a §13
  ADR trigger, flag it in the spec.
- Authority order: foundation docs govern area docs govern specs govern tickets; if a lower
  layer conflicts with a higher one, the lower layer is wrong and must stop, not design around it.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  etc.; typed mechanic nouns belong in `games/*`, shared helpers in `game-stdlib`/`game-test-support`
  only via the register and only when behavior-free.
- **TypeScript never decides legality** and never normalizes/repairs seat IDs — C-02 canonical
  parse/format authority is Rust only; legacy aliases are import-only at the `wasm-api` boundary.
- **No YAML and no DSL without an accepted ADR.** Profile/fixture data stays typed
  content/parameters/metadata — never selectors, conditions, triggers, or formulas.
- **Determinism:** replay, hashes, RNG consumption, serialization order, and traces stay
  deterministic; any change is an explicit, characterized, versioned ADR-0009 migration.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, or replay/export exports; test canaries never appear in committed artifacts.
- **No behavioral promotion** through the scaffolding lane (the §3.3 "Not allowed" list of the
  parent spec and the register Non-Promotion List).
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (AGENT-DISCIPLINE §4); a generic assertion never replaces a specific game assertion.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**: the Unit 8C-R1 implementation spec.

- **Shape: intermediate spec artifact — NOT a ready-to-drop final repo file.** The user saves
  your output and runs the repository's new-spec pipeline (`/reassess-spec` → `/spec-to-tickets`)
  to revalidate it against the codebase and decompose it into `tickets/`. The eventual target
  path is `archive/specs/8c-r1-public-fixed-seat-scaffolding.md` (label one-line-correctable), but your
  deliverable **does not land there directly** and must not be presented as already-final or as
  skipping the reassess/decompose step. Suggest the user save it under `reports/` (or `specs/`
  for the reassess step) as a working artifact.
- **Structure: follow the `specs/README.md` 12-section spec format** exactly — (1) Header
  [Spec ID `8C-R1`, stage, unit, status `Planned` on authoring, date, owner, authority order],
  (2) Objective [sourced from the parent spec §5 seed + ROADMAP framing], (3) Scope
  [in/out/not-allowed — carry the seed's bounds and the parent spec's "Not allowed" list],
  (4) Deliverables [concrete artifact tree: the six games' touched surfaces + register entries +
  characterization report + `specs/README.md` flip], (5) Work breakdown [bounded candidate
  AGENT-TASK items in dependency order: an admission/characterization wave, then per-game/per-helper
  audit-and-migrate waves, then a consolidation + register + status-flip closeout — each item
  names exact files/symbols/affected hash & visibility surfaces/rollback scope], (6) Exit criteria
  [row-for-row, mapped to the seed's admission/exit and to parent EC-28/EC-30; include a
  per-(game × helper) verdict-coverage criterion], (7) Acceptance evidence [the command set
  — `cargo fmt/test`, `replay-check`/`fixture-check` for the six games, `boundary-check.sh`,
  `check-doc-links.mjs`, `check-catalog-docs.mjs` as a regression guard, register before/after],
  (8) FOUNDATIONS & boundary alignment [principles engaged + stance], (9) Forbidden changes,
  (10) Documentation updates required [register entries + this index flip; explicitly mark
  `apps/web/README.md` **not applicable** — no game/web surface], (11) Sequencing [predecessor
  8C `Done`; successors 8C-R2…R4 then Gate 18; admission rule], (12) Assumptions
  [one-line-correctable, including the slug].
- **Include a per-surface audit matrix** (game × helper C-01…C-05/C-08, with each cell a verdict:
  *migrate / not-applicable / exception / already-discharged-by-8C-pilot*) as the spine of the
  Scope/Work-breakdown sections — this is what makes R1 a bounded audit rather than an open sweep.
- **Carry any `assumption:` lines** from §3 into the spec's §12 so the user can override them.
- Append the **locked / no-questions** posture in the spec preamble: it is an authored plan to be
  reassessed and decomposed, not executed code.

---

## 8. Self-check (run against your own output before returning)

- The determination is confirmed-and-documented with the §1 evidence (lowest non-`Done` row;
  atlas §10A empty; EC-28/EC-30), **not** re-opened, and the scope never drifts to R2/R3/R4 or
  Gate 18.
- Exactly the six named games appear; every (game × applicable helper) surface carries an
  explicit verdict; no unowned cleanup bucket; no seventh game.
- No already-shipped 8C/Gate-7.1 work is re-proposed as missing; pilots are treated as
  discharging only their named surfaces.
- Every hash/byte/seat/visibility/RNG-touching item is framed as a named ADR-0009 per-surface
  migration with characterization + before/after + compatibility window + rollback; no blanket
  golden regeneration; one surface per diff.
- No new doctrine weakens an upstream foundation doc or silently amends an accepted ADR; any
  genuinely required doc change is surfaced as a flagged ADR-trigger inside the spec, not enacted.
- The deliverable is the single Unit 8C-R1 spec, framed as an intermediate artifact for
  `/reassess-spec` → `/spec-to-tickets`, following the `specs/README.md` 12-section format, and
  marks `apps/web/README.md` not applicable.
- Every external claim that shaped a recommendation is cited.
- The `820487d` fetch-baseline commit contains every file named in the §2 read-in-full list.
