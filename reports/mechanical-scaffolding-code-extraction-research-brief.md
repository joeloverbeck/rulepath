# Research brief — Mechanical-scaffolding code extraction (Rulepath unit 8C / report Part C)

You are an external deep-research session (ChatGPT-Pro). You receive this prompt
plus one uploaded file: the repository path manifest. You have no other context
from the session that authored this brief — everything you need is below or in
the manifest. **Produce the deliverable directly. Do not interview, do not ask
clarifying questions** (see §7). The determination and scope are already locked;
your job is to confirm-and-document them and fill in the deep design + external
prior art.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-22_28c9893.txt`) is the path inventory
of the `joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing,
replayable, testable card/board-game platform where **Rust owns all behavior and
TypeScript/React present only.** The foundation docs are an ordered, layered
authority indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution) →
`ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`;
earlier documents govern later ones, and accepted ADRs supersede them only by
explicitly naming the affected sections.

**Fetch every file from commit `28c9893`** (`git rev-parse HEAD` at brief-authoring
time) — the uploaded manifest is exactly that commit's tree
(`git ls-tree -r --name-only 28c9893`). One adjacent file, the report you build
on, was *authored* against commit `db0c50b` (an earlier `HEAD`); that is the
report's own baseline. Use the manifest's commit `28c9893` for all reads — at
`28c9893` every file named in §2 below is present and verified — and treat the
report's `db0c50b` references as historical traceability, not as the tree to read.

**Repo-structure correction (carry this — the commissioning request was wrong on
it):** the request named `docs/0-foundation/*`, `docs/1-architecture/*`,
`docs/2-execution/*`, `docs/3-reference/*`, `docs/4-specs/*`. **Those numbered
subdirectories do not exist.** The `docs/` tree is *flat* (`docs/*.md` plus
`docs/adr/*.md`); the numbered "tiers" are only the conceptual layering expressed
in `docs/README.md`'s authority table. Implementation specs live in `specs/`
(and completed ones in `archive/specs/`), not under `docs/`. Read the real flat
paths listed in §2, in the `docs/README.md` authority order.

---

## 2. Read in full (authority order)

Read these in full, in this order, before producing anything.

**Constitution & architecture (highest authority):**

- `docs/README.md` — the authority-order table, the layering rule, and the ADR
  status index; tells you which document wins when two disagree.
- `docs/FOUNDATIONS.md` — the constitution: product priority, behavior authority,
  §4 (`game-stdlib` earned + the new mechanical-scaffolding lane), §11 universal
  invariants, §12 stop conditions, §13 ADR triggers. Every extraction decision in
  the deliverable must satisfy these.
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the Rust/WASM
  boundary, the effect/view/action-tree/replay/determinism model, and the
  **ownership matrix** (kernel-contract ergonomics vs. `game-stdlib` vs. the
  dev-only `game-test-support` layer) that decides where each extracted helper
  may land.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact boundary and the **four reuse
  lanes** (kernel ergonomics / mechanical scaffolding / behavioral mechanics /
  typed content) with narrowest-layer-wins; the noun-free `engine-core` rule the
  extraction must not break.

**Governing ADRs (accepted; supersede foundation sections they name):**

- `docs/adr/0008-mechanical-scaffolding-governance.md` — **the governing ADR for
  this entire unit.** Defines the mechanical-scaffolding category and exclusions,
  the allowed homes (`engine-core` ergonomics, `game-stdlib`, the future dev-only
  `game-test-support`, `wasm-api` adapters), the evidence fields, and the
  category decision rule (review at second exact duplication, hard decision before
  a third copy; promote only when semantic identity is proven, the API is
  noun-free / correctly game-layer-typed, behavior-neutral, deterministic,
  leak-safe, and migration-complete). `Status: Accepted`.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — governs **all** hash, fixture,
  visibility, and byte migration in C-04 / C-05 / C-08: artifact classes,
  visibility classes, validators, version identifiers, canonical-byte authority,
  hash-surface versions, compatibility windows. `Status: Accepted`.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the hidden-information
  replay/export no-leak taxonomy the new test harnesses (C-07/C-08) must
  **preserve or strengthen**, never relocate out of the games.

**Area law the extraction touches:**

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the register governed by ADR 0008:
  the per-entry **Entry Schema**, decision states, the **Non-Promotion List**, and
  the current empty entry table. **Every helper this unit extracts must first land
  an entry here.**
- `docs/MECHANIC-ATLAS.md` — the *behavioral* third-use hard gate (must stay
  intact), §10A open-promotion-debt register (currently *empty* — confirm), and
  the behavioral/scaffolding split. The C-10 non-promotion boundary lives at the
  atlas↔register seam.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — shared test-support law, named fixture
  profiles, no-leak tests, deterministic replay/hash discipline, and the
  **hash-migration protocol** that governs C-06/C-07/C-08 and any byte change.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the named fixture profiles
  (`replay-command-v1`, `public-export-v1`, `seat-private-export-v1`,
  `setup-evidence-v1`, `domain-evidence-v1`), validator ownership, and visibility
  classification that the C-08 replay-profile drivers must validate against.
- `docs/TRACE-SCHEMA-v1.md` — the command/replay trace schema; its field *bytes*
  must not change in this unit except by an explicit, versioned ADR-0009-governed
  migration.
- `docs/WASM-CLIENT-BOUNDARY.md` — the canonical going-forward seat grammar
  (`seat_<zero-based>`) and the alias policy enumerating the divergent in-corpus
  forms (`seat-<n>`, `seat_<n>`, `seat-a`) that C-02 must reconcile.
- `docs/AGENT-DISCIPLINE.md` — bounded-task law, the failing-test protocol, the
  scaffold-refactor protocol, and the wave discipline this unit's work breakdown
  must follow.
- `docs/ROADMAP.md` — the staged ladder, the pre-Gate-18 and per-gate
  scaffolding/trace debt interlocks, and where unit 8C sits relative to Gate 18.

**Spec layer (the deliverable's own format + immediate predecessors):**

- `specs/README.md` — the **living progress tracker** (read the active-epoch
  table: row `8C` is the lowest non-`Done` unit; row `9`/Gate 18 names 8C as its
  precondition) and the **canonical 12-section spec format** the deliverable must
  follow.
- `archive/specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md` — the
  predecessor unit **8M** (`Done`). Its **Out-of-scope table** (the rows pointing
  Part C C-01..C-09, the `game-test-support` crate, and the C-11 retrofit waves to
  "the forward code unit") **fixes the boundary of 8C exactly.** Build on what 8M
  shipped; do not re-do it.
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the **source report.** Its
  **Part C (items C-01 … C-11) and Batches 3–4** are the concrete scope of this
  unit. Its Part A/B/D already shipped as 8M (see §3). Treat the report's per-file
  claims as traceability anchors to re-verify against commit `28c9893`, not as
  ground truth.

**Lighter cross-reference reads (so the deliverable contradicts nothing
downstream):**

- `docs/AI-BOTS.md` — hidden-information safety for bot explanations / candidate
  rankings, a no-leak surface the C-07 harness must cover.
- `docs/IP-POLICY.md` — the public/private leak boundary; test canaries must never
  enter public fixtures.
- `templates/AGENT-TASK.md` and `templates/GAME-EVIDENCE.md` — the downstream
  ticket-decomposition and per-game evidence-receipt shapes the spec's work
  breakdown and acceptance evidence should reference (reference, not copy).
- Directory floor — skim the remaining `docs/*.md`
  (`OFFICIAL-GAME-CONTRACT.md`, `UI-INTERACTION.md`,
  `MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `SOURCES.md`, `archival-workflow.md`) for
  any cross-reference the extraction or the spec's documentation-updates section
  must keep truthful.

### Code seams to inspect directly (inspect in the repo, *not* read-fully here)

These are the duplication sites the extraction consolidates. Read them in the
repo to ground each helper's API and migration set; do not paste them.

- `crates/engine-core/src/{lib.rs,action.rs,replay.rs,rng.rs}` — `EffectEnvelope<T>`,
  `VisibilityScope`, `SeatId`, `ActionTree`, `DeterministicRng::next_index`: the
  generic types C-01/C-02/C-04/C-05/C-09 extend with ergonomics.
- `crates/game-stdlib/src/{trick_taking.rs,board_space.rs}` — the *already-earned*
  shared helpers; build beside them, don't duplicate.
- `crates/wasm-api/src/seats.rs` — per-game seat-string branches C-02 must unify
  behind the canonical grammar + alias adapter.
- `games/*/src/effects.rs` — repeated `public_effect()` / private-effect factories
  (C-01). E.g. `race_to_n`, `three_marks`, `token_bazaar`, `draughts_lite`.
- `games/*/src/ids.rs` — divergent seat parse/format + ring arithmetic (C-02/C-03):
  `river_ledger` (`RiverLedgerSeat`, `seats_for_count`, `next_in_count`),
  `briar_circuit` (`enum` + `next_clockwise`, pass-target math),
  `vow_tide` (`enum`, `next_clockwise(seat_count)`, `canonical_seat_ids`).
- `games/*/src/replay_support.rs` — hand-built `effect_stable_string`,
  `effect_hash`, `action_tree_hash` (C-04/C-05): `race_to_n`, `column_four`,
  `event_frontier`.
- `games/*/tests/{visibility.rs,replay.rs}` — duplicated pairwise-no-leak matrices
  and replay drivers (C-06/C-07/C-08): `river_ledger`, `briar_circuit`,
  `vow_tide`, `poker_lite`, `high_card_duel`, `secret_draft`.
- `tools/{replay-check,fixture-check}` — the cross-game replay/fixture CLIs whose
  per-game registrations interact with the new profile drivers.

---

## 3. Settled intentions (final — these make this brief locked)

These were resolved before you were commissioned. Treat each as a committed
decision; do **not** re-open any of them.

1. **The next spec to create is unit `8C` — "Mechanical-scaffolding code
   extraction (Part C)."** It is the lowest non-`Done` row in the `specs/README.md`
   active-epoch tracker, and it is now **fully unblocked**: ADR 0008 `Accepted`,
   ADR 0009 `Accepted`, predecessor unit 8M `Done`, and `MECHANIC-ATLAS` §10A
   open-promotion-debt register **empty**. Your deliverable **confirms and
   documents** this determination — citing exactly that evidence — and does **not**
   re-litigate "what should come next." Gate 18 (Spades) is explicitly *after* 8C
   and names 8C's debt closure as its own precondition; you are not writing Gate 18.

2. **Sharp delta — the doc/ADR/template work is already shipped; do not
   re-recommend it.** Unit 8C implements **Part C / Batches 3–4** of
   `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`. That report's **Part A, Part
   B, and Part D** — the authority-map hygiene, ADR 0008 and ADR 0009 themselves,
   the `MECHANICAL-SCAFFOLDING-REGISTER.md`, the `EVIDENCE-FIXTURE-CONTRACT.md`,
   the `GAME-EVIDENCE.md` receipt, the foundation/area-doc amendments, and the
   template slimming — **already landed as unit 8M** (`PREGAT18REUDOC-001..023`).
   Build on that shipped baseline. Do **not** propose re-authoring the ADRs, the
   register, the contract, or the template realignment as if missing. The *only*
   thing Part C leaves unbuilt is the **code**.

3. **Scope of the 8C spec = full Part C infrastructure + pilot adoptions, with the
   remaining per-game retrofit waves seeded forward as bounded follow-on specs.**
   In scope for the 8C spec:
   - **C-01** generic `EffectEnvelope` constructors (e.g. `public`, `private_to`);
   - **C-02** canonical seat-ID parse/format helpers with the `seat_<zero-based>`
     grammar and an import-only legacy-alias adapter;
   - **C-03** typed seat-count validation + ring-index arithmetic (no
     macro-generated seat enum — defer that);
   - **C-04** versioned canonical action-tree encoding + `stable_hash_vN`;
   - **C-05** an explicit versioned stable-byte writer (no reflection/derive
     "magic" hashing);
   - **C-09** documented bounded-index RNG sampling contract (keep shuffle/deal
     game-local; characterize bytes first);
   - **C-06** the new **dev-only `crates/game-test-support`** crate;
   - **C-07** the pairwise-no-leak assertion harness (geometry only — games still
     own projection/authorization);
   - **C-08** the profile-driven replay-fixture drivers (validating the
     EVIDENCE-FIXTURE-CONTRACT profiles);
   - **C-10** publishing/affirming the non-promotion boundary into the register;
   - **pilot adoptions** on a small representative game set to *prove* each helper
     has real callers and unchanged-or-migrated hashes.
   Seeded **forward** (not implemented in the 8C spec): the **C-11 per-game
   retrofit waves** across the remaining ~17 games — these become bounded
   follow-on specs/units after 8C, exactly as 8M's out-of-scope table prescribes.

4. **Non-negotiable discipline (locked):**
   - **Characterize-first; no blanket golden/hash regeneration.** Every byte-,
     hash-, or visibility-affecting move (C-04/C-05/C-08) is governed by ADR 0009:
     characterize current bytes, add the versioned writer/encoding, migrate one
     surface at a time with before/after evidence and a rollback point. Never
     "regenerate all goldens to green."
   - **The behavioral third-use hard gate, no-leak, determinism, and the noun-free
     `engine-core` rule all stay intact.** This unit extracts *behavior-free
     plumbing only*; anything on the register's Non-Promotion List (deal/reveal/
     projection/betting/pot/trick-lifecycle/teams/graph/accounting/reaction/
     scoring) stays game-local.
   - **Every promoted helper lands a `MECHANICAL-SCAFFOLDING-REGISTER.md` entry —
     with its semantic-risk class, home, affected hashes, visibility impact,
     migration set, and acceptance evidence — *before* extraction.**
   - **`game-test-support` is dev/test-only** — production crates must not depend on
     it; it cannot implement legality, projection, or redaction.

5. `assumption:` You **may recommend** (this is a *bounded delegation*, not a
   re-determination): the exact pilot-game set for each extraction wave, and each
   helper's precise landing home among the ADR-0008-allowed options
   (`engine-core` ergonomics vs. `game-stdlib` vs. `game-test-support`). For each,
   enumerate the options, pick a **required default**, and justify it against ADR
   0008's decision rule and the ARCHITECTURE ownership matrix. The user can later
   override these; everything else in §3 is fixed.

---

## 4. The task

Produce a **new-spec design / work-indication document** for Rulepath unit **8C —
Mechanical-scaffolding code extraction (Part C)**: a precise, reviewable plan that
Claude Code will convert into `specs/<8c-slug>.md`. It must turn report Part C
(C-01..C-11, Batches 3–4) into a bounded, dependency-ordered, ADR-governed
extraction plan that consolidates the duplicated mechanical scaffolding across the
`games/*` crates into the allowed shared homes — **without** changing any
behavior, weakening any invariant, regenerating hashes wholesale, or leaking
hidden information. The plan must be deep enough that `/reassess-spec` and
`/spec-to-tickets` can decompose it into one ticket per reviewable diff, and it
must explicitly carry the characterize-first / pilot-then-seed-waves discipline of
§3. This is a **new spec**, not a fix or an overhaul.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — in
particular the code seams — to ground each helper's real API, duplicate sites, and
migration set against commit `28c9893`.

Research online as deeply as needed — similar implementations, research papers,
and prior art — wherever it sharpens the deliverable. Cite sources for any
external claim that shapes a decision. High-value external axes for this unit:

- **Deterministic / canonical binary serialization** for stable hashing — explicit
  length-delimited / tagged-field encodings vs. reflection/derive hashing; why
  canonical-bytes-by-construction beats "serialize then hash" (e.g. canonical CBOR,
  protobuf canonical-form pitfalls, `bincode`/`borsh` determinism caveats,
  Merkle/content-addressing field-ordering lessons).
- **Characterization ("golden master") testing** and safe hash/serialization
  migration with versioned encodings and compatibility windows — how mature
  systems migrate on-disk/replay formats without silent semantic drift.
- **Shared dev/test-support crate patterns in Rust workspaces** —
  `[dev-dependencies]`-only crates, test-only feature gating, keeping test helpers
  out of the production dependency graph, property/matrix test harness design.
- **Seat / player-index identifier modeling** — newtype + parse/format round-trip
  discipline, legacy-alias import adapters, modular ring arithmetic for turn order.
- **Deterministic bounded-index RNG sampling** — unbiased rejection sampling vs.
  modulo bias, and why centralizing the *index sampler* is safer than centralizing
  *shuffle/deal policy*.
- Any reusable lessons from comparable rules-engine / game-server codebases that
  separated **behavior-free plumbing** from **behavioral mechanics** — the exact
  distinction ADR 0008 draws.

---

## 6. Doctrine & constraints (honor all)

- `docs/FOUNDATIONS.md` is the constitution — every decision must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence
  needs an accepted ADR superseding the affected principle first, never a silent
  design-around. ADRs 0008 and 0009 are the accepted ADRs that authorize this
  unit; stay inside what they name.
- **Authority order:** foundation docs govern area docs govern specs govern
  tickets. If your plan ever conflicts with architecture or foundation, the plan
  is wrong — fix the plan.
- **`engine-core` stays generic and noun-free** — no `board`, `card`, `deck`,
  `grid`, `hand`, `pot`, `trick`, `faction`, `partnership`, etc. The extraction
  targets nouns *already* in the kernel's allowed vocabulary (effect envelope,
  seat id, action tree, visibility scope). `game-stdlib` stays earned and narrow;
  `game-test-support` is dev-only.
- **TypeScript never decides legality.** Legality, validation, effects, views, and
  bot decisions stay in Rust/WASM. C-02's seat grammar reaffirms Rust owns
  parse/format.
- **No YAML and no DSL** — static data stays typed content/parameters/metadata;
  never selectors, conditions, triggers, or formulas.
- **Determinism:** replay, hashes, RNG, serialization order, and traces stay
  deterministic; any change is an explicit, versioned, ADR-0009-governed migration
  with before/after evidence — **never a blanket regeneration**.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs,
  bot explanations, candidate rankings, replay exports, traces, fixtures, or
  tests. The no-leak harness proves geometry; games keep projection/redaction.
  ADR 0004 stays authoritative.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2 (unaffected
  here, but do not introduce anything that pressures it).
- **Never delete or weaken tests/checks to get green** — follow the failing-test
  protocol (`AGENT-DISCIPLINE` §4). Characterization tests are added, not relaxed.
- **Respect the boundary the register draws:** anything on the Non-Promotion List
  is behavioral and stays game-local; a proposed helper that touches one defaults
  to `rejected` for this register.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **New file** (Claude Code will save it under `reports/` and convert it into
  `specs/<8c-slug>.md`): the **8C spec-design / work-indication document**. It does
  **not** replace any existing file. Suggested internal title: *"Spec design — Unit
  8C: Mechanical-scaffolding code extraction (Part C)."*

It MUST follow the canonical 12-section Rulepath spec format defined in
`specs/README.md` ("Spec format"):

1. **Header** — Spec ID, stage (public scaling phase, unit 8C), gate/unit, status
   (`Planned` on authoring), date, owner, authority order.
2. **Objective** — what the unit achieves, sourced from the ROADMAP interlocks and
   report Part C.
3. **Scope** — in scope / out of scope / not allowed. In scope: C-01..C-10 +
   pilot adoptions per §3.3. Out of scope: the C-11 per-game retrofit waves
   (seeded forward), Gate 18, any doc/ADR/template work already shipped by 8M.
   Not allowed: carry the §6 / 8M "Not allowed" prohibitions (no noun in kernel,
   no blanket hash regen, no leak, no behavioral promotion, no YAML/DSL, no
   test weakening).
4. **Deliverables** — concrete artifacts/tree: each extracted helper with its
   landing crate/module, the new `crates/game-test-support` crate, the harnesses,
   the per-helper `MECHANICAL-SCAFFOLDING-REGISTER.md` entries, and the pilot
   adoptions — grounded in ARCHITECTURE's ownership matrix.
5. **Work breakdown** — bounded items, each a candidate AGENT-TASK, in **dependency
   order** (characterization → versioned writer/encoding types → action-tree
   encoding → `game-test-support` → harnesses → pilot adoptions; ADR-0009-gated
   items flagged). Map each item back to its report `C-NN` ID for traceability.
6. **Exit criteria** — mapped row-for-row to the unit's obligations; include
   "unchanged hashes or explicit per-surface migration notes," "no-leak matrices
   pass for all supported seat counts," "boundary-check passes," "every promoted
   helper has a register entry."
7. **Acceptance evidence** — the exact tests/traces/benchmarks/CLIs that prove it
   (e.g. `cargo test --workspace`, `replay-check`, `fixture-check`,
   `boundary-check.sh`, the no-leak harness), plus the characterization evidence
   per migrated surface.
8. **FOUNDATIONS & boundary alignment** — principles engaged (§3 noun-free, §4
   scaffolding lane, §11 invariants, §12 stop conditions, §13 triggers) with
   stance and rationale; show how ADR 0008/0009 authorize each move.
9. **Forbidden changes** — unit-specific prohibitions (the §6 list, concretized).
10. **Documentation updates required** — including the `specs/README.md` status
    flips (8C → `Planned`, later → `Done`) and the register entries. This unit has
    **no web-exposed game gate**, so `apps/web/README.md` is *not applicable* as a
    closeout surface (note it explicitly; `check-catalog-docs.mjs` still runs as a
    guard).
11. **Sequencing** — predecessor (8M `Done`) / successor (the C-11 retrofit waves,
    then Gate 18); the admission rule for Gate 18 that this unit's closure
    satisfies.
12. **Assumptions** — one-line-correctable, including the §3.5 bounded delegations
    (pilot set + per-helper home) as labeled assumptions the maintainer can
    override.

Plus a clearly-marked **"Required documentation amendments"** section (the user's
explicit ask: the plan aligns with `docs/**` unless docs need amending, in which
case state exactly how). Expected content: the new per-helper
`MECHANICAL-SCAFFOLDING-REGISTER.md` entries this unit will add; any
`ROADMAP.md`/`specs/README.md` interlock wording the unit closes; and an explicit
statement if you find **no** doc amendment is needed beyond register entries +
status flips. If you discover a genuine doc gap or contradiction that blocks the
extraction, name the doc, the section, and the precise amendment — but do **not**
silently design against any accepted ADR or foundation section.

**Locked / no-questions:**

> Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final.
> If a genuine contradiction makes a requirement impossible, state it in the
> deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run before returning)

- The deliverable confirms **8C** as the next unit and cites the locking evidence
  (lowest non-`Done` row; ADRs 0008/0009 `Accepted`; 8M `Done`; §10A debt register
  empty) — it does **not** re-open the determination.
- It implements **only report Part C / Batches 3–4 (code)** and does **not**
  re-recommend the Part A/B/D doc/ADR/template work already shipped as 8M.
- Scope matches §3.3: full infra (C-01..C-10) + `game-test-support` + harnesses +
  pilot adoptions in the 8C spec; C-11 retrofit waves seeded forward.
- Every byte/hash/visibility move is ADR-0009-governed, versioned, and
  characterize-first — **no blanket golden regeneration** anywhere.
- The behavioral third-use gate, no-leak (ADR 0004), determinism, and noun-free
  `engine-core` are preserved; nothing on the register Non-Promotion List is
  promoted; every promoted helper has a register entry.
- The document follows the 12-section `specs/README.md` spec format exactly and
  includes the "Required documentation amendments" section.
- Every external claim that shaped a decision is cited.
- Every file referenced is present at commit `28c9893` (the manifest's tree); the
  `db0c50b` references are flagged as the report's historical baseline only.
- The deliverable set matches §7 exactly: one new markdown document.
