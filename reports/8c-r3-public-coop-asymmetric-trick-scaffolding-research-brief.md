# Research brief — Unit 8C-R3 (C-11 follow-on: public/co-op/asymmetric/trick scaffolding) next spec

> **You are Session 2: a locked deep-research-and-author session.** Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-24_be1af6f.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `be1af6f` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `be1af6f`, not the cited string.

**The determination is already made (do not re-open it).** This brief does not ask you to
decide *what* to build next — that decision is locked. The repository workflow
(`specs/README.md` → "Workflow": *pick the lowest non-`Done` unit from the active-epoch
tracker*) selects the next unit deterministically, and that unit is **Unit 8C-R3 — "C-11
follow-on — public/co-op/asymmetric/trick support."** Your job is to **confirm-and-document**
that determination with cited evidence and then **author the bounded implementation spec for
Unit 8C-R3**. The evidence that fixes the determination, all verifiable in the tree at
`be1af6f`:

- `specs/README.md` active-epoch tracker: row `8C-R2` is now **`Done`** (completed 2026-06-23),
  and **`8C-R3` is the lowest row whose status is `Not started`** (a `(seed; unwritten)` row).
  `8C-R4` sits below it; Gate 18 (Spades) sits below that and is explicitly *"Pending …
  **8C-R1…8C-R4** closed / explicitly not applicable / accepted-excepted."*
- `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register: **`_None_` / `No open promotion
  debt remains.`** No primitive-promotion debt is open, so nothing must close ahead of 8C-R3.
  (8C-R3 is a *code-scaffolding retrofit*, not a mechanic-ladder gate, so the atlas "close debt
  before the next mechanic-ladder gate" interlock does not gate it either.)
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` (the parent spec): §5
  "Required C-11 forward-wave seeds" defines 8C-R3's exact game set and audit shape; work item
  **8C-030** created the four `8C-R1…R4` seed rows in `specs/README.md`; exit criteria **EC-28**
  ("Four bounded C-11 seeds cover every official game exactly once … They remain unimplemented")
  and **EC-30** ("Gate 18 remains after the seeded C-11 waves are closed, not applicable, or
  explicitly excepted") fix the sequencing.
- The two preceding waves — `archive/specs/8c-r1-public-fixed-seat-scaffolding.md` and
  `archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md` — are the
  **direct structural precedents**. 8C-R3 is the *third* wave in the same lane and the same
  shape, applied to a different (public / cooperative / asymmetric / trick) game set.

So this is a **new-spec** task whose subject is a behavior-free **mechanical-scaffolding
retrofit/audit unit**, not a new game — the third of four C-11 follow-on waves.

**The defining delta from 8C-R1 and 8C-R2 — read this carefully.** R1 covered six **public /
perfect-information** fixed-seat games (so it auto-N/A'd the hidden-information surfaces). R2
covered four **two-seat hidden/reaction** games (so it treated C-07 no-leak, seat-private
export, and private envelopes as uniformly *applicable*). **8C-R3 is the heterogeneous wave**:
its four games span four different visibility/structure shapes, so verdicts are neither
auto-N/A (R1) nor auto-applicable (R2) — each cell must be **classified from the real code**.
The wave's two distinguishing properties:

1. **No game in this wave was an 8C pilot, and none currently carries a `game-test-support`
   dev-dependency.** (8C pilots were Race to N, Draughts Lite, High Card Duel, River Ledger, Vow
   Tide, Briar Circuit; the C-08 fixture pilots were Race / River / Vow / Briar.) Therefore —
   unlike R2, where High Card Duel was a *residual* audit with `already-discharged-by-8C-pilot`
   cells — **R3 has zero pilot-discharged cells; every (game × applicable helper) surface is a
   fresh audit.** Verify this against the manifest/Cargo files; do not invent a pilot residual.
2. **The wave's center of gravity is setup/profile diversity (C-03) and domain/fixture profiles
   (C-08), not the no-leak geometry that centered R2.** The seed's own emphasis is *"setup/profile
   diversity, action-tree/effect hashing, and fixture/profile surfaces."* Concretely, verified at
   `be1af6f`:
   - **C-03 is profile-rich, not a uniform exact-two predicate.** `flood_watch`,
     `frontier_control`, and `event_frontier` validate not just `seats.len() != STANDARD_SEAT_COUNT`
     but also `variant.seat_count` and (flood_watch) `variant.role_order.len()` — cooperative
     role assignment and asymmetric factions. The C-03 audit must respect that diversity and
     keep role/faction/variant policy game-local.
   - **C-08 `domain-evidence-v1` / `setup-evidence-v1` is central.** All four games ship domain
     fixtures (`games/*/data/fixtures/*_standard.fixture.json` plus variants such as
     `flood_watch_deluge`, `frontier_control_highlands`, `event_frontier_hard_winter` /
     `_land_rush`), mirroring the Briar Circuit C-08 `domain-evidence-v1` pilot. Connectivity
     scoring, round scoring, event/edict resolution, and budget accounting live in these
     fixtures and **stay game-owned** — the driver validates profile shape only.
   - **C-07 verdicts are genuinely mixed and must be classified per game**: `plain_tricks` has
     real private hands (`tests/golden_traces/deal-private-no-leak.trace.json`,
     `no-leak-public-observer.trace.json`) → seat-private no-leak is *applicable*;
     `flood_watch` and `event_frontier` have a public-observer no-leak surface
     (`public-observer-no-leak.trace.json` / public current-next reveal) but **no per-seat private
     holdings** (atlas §10B records both as not full private-hand uses); `frontier_control` is a
     fully public perfect-information graph game (atlas §10B records no randomness/shuffle/hidden
     holdings). So seat-private no-leak / `seat-private-export-v1` is likely N/A for the three
     public/co-op/asymmetric games and applicable for `plain_tricks` — but **you assign each
     verdict from the code, with explicit `not applicable` rows, never silent omission.**
   - **ADR 0004 (hidden-information replay/export taxonomy)** is therefore *central for
     `plain_tricks`* and a *regression guard* for the three public games (their public-observer
     no-leak surfaces must stay leak-free), not the uniformly-central read it was for R2.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground
truth. The user named **`docs/**` (all files and folders), `templates/**`, and
`specs/README.md`** as the mandatory floor; read the entire `docs/**` and `templates/**` trees.
Each line below states why a file is load-bearing *for the 8C-R3 spec*; files in the named floor
that only bear incidentally are covered by one directory-level reason per tier.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect; the ADR status index (0004/0008/0009 all `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants (esp. the determinism and hidden-information / no-leak invariants), §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — boundary & scaffolding law (the crux for this unit)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the Rust/WASM boundary, and the action/view/effect/replay/determinism model; the "narrowest lawful owner wins" ownership matrix the retrofit must honor.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `game-test-support` / `games/*` / static-data boundary; the retrofit may adopt shared helpers but must never move game behavior (trick/graph/event/budget/scoring/projection policy) across this line or add nouns to `engine-core`.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law; **§10** initial atlas table, **§10A** empty open-promotion-debt register (proves nothing precedes this unit), and **§10B** deferred/candidate register — read §10B closely: it records the `deterministic shuffle / private hand / staged reveal` and `public resource accounting / shared ledgers` rows that **explicitly classify all four R3 games** (plain_tricks third-use trick-deal decision; flood_watch/event_frontier as not-full-private-hand uses; frontier_control as non-use; event_frontier budget/accounting deferral; the board-space §10 row records frontier_control and event_frontier as *not applicable* for `game-stdlib::board_space`). Those rows are the behavioral boundary the scaffolding lane must **not** breach.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — **the governing register for this unit**: the Entry Schema, Decision States, the **Non-Promotion List** (deal/shuffle policy, reveal timing, projection/redaction, trick lifecycle / led-suit / trump / winner-leads, graph/topology/adjacency/movement/connectivity, resource accounting / shared ledgers / scoring ledgers, teams/roles — *all the policy this wave's games embody and must keep game-local*), the landed `MSC-8C-001…010` entries with their `R1 public fixed-seat receipts` **and** `R2 two-seat hidden/reaction receipts` tables, and the per-entry "next review trigger = C-11 game audits". Every R3 migration adds/updates a register entry here (R3 receipts go under the same `MSC-8C-*` entries the way R1's and R2's did).
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted governance ADR for the scaffolding lane; defines what may be extracted and the register-first discipline.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2; **every hash-bearing surface flip in R3 follows its per-surface migration protocol** (characterize → parallel surface → classify unchanged/parallel-new/intentional-migration → compatibility window → rollback).
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the hidden-information replay/export visibility taxonomy; **central for `plain_tricks`** (private hands, deal-private no-leak, public replay export) and a **regression guard** for the three public/co-op/asymmetric games' public-observer no-leak surfaces. Private data must remain absent from every viewer-scoped or public surface.

**Tier 3 — evidence, determinism, hidden-info & multi-seat law**
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden-trace obligations, deterministic replay/hash discipline, **no-leak tests**, benchmarks, CI; the acceptance-evidence backbone of the spec.
- `docs/TRACE-SCHEMA-v1.md` — trace/replay-fixture schema law; load-bearing because C-04/C-05/C-08 audits touch trace bytes (these four games carry rich action-tree / event-resolution / replay-export traces) and any change is an ADR-0009 migration packet, never a silent edit.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the evidence-fixture profile contract (ADR 0009); separates command traces, **setup/domain fixtures**, and viewer-scoped exports — the C-08 profile-driver audit (esp. `setup-evidence-v1` and `domain-evidence-v1`, central for this wave) conforms to this.
- `docs/AI-BOTS.md` — bot policy and **bot-input / bot-explanation no-leak law**; load-bearing because each game ships bot traces (`bot-action`, `bot-coop-full-game`, `bot-vs-bot-full-game`) and the C-07 audit (where applicable) asserts over bot-input/bot-explanation surfaces; no bot may read disallowed private state.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat/surface obligations and the canonical seat-declaration/viewer rules; **directly load-bearing** because R3's games are *not* uniform fixed-two-seat — they carry cooperative role order, asymmetric factions, and variant seat counts that the canonical-seat-grammar (C-02), seat-count/variant (C-03), and viewer-scoped-surface (C-07/C-08) audits must respect without moving role/faction policy into shared code.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, the failing-test protocol the spec's work breakdown and "one surface per diff" rollback rule must encode.

**Tier 4 — the named floor remainder + planning artifacts**
- Remaining `docs/**` — `OFFICIAL-GAME-CONTRACT.md`, `UI-INTERACTION.md`, `IP-POLICY.md`, `SOURCES.md`, `WASM-CLIENT-BOUNDARY.md`, `ROADMAP.md`, `archival-workflow.md`, `adr/0001…0003/0005/0006/0007`, `adr/ADR-TEMPLATE.md` — part of the mandatory `docs/**` floor; mostly background for this behavior-free unit (no new game, no UI, no bot-strategy change). Read for completeness; `WASM-CLIENT-BOUNDARY.md` has direct bearing because C-02's legacy seat-alias adapter lives at the `wasm-api` boundary, `ROADMAP.md` fixes the public-scaling sequence, and `IP-POLICY.md` matters only as a regression guard (these are original games, not licensed content).
- All of `templates/**` — `AGENT-TASK.md` is the packet shape the spec's work breakdown decomposes into (each work item is a candidate AGENT-TASK; profile `scaffold-refactor`); the `GAME-*` and evidence/admission templates (esp. `GAME-EVIDENCE.md`, `GAME-RULE-COVERAGE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`) show the evidence vocabulary the spec's acceptance section should reference. Part of the mandatory floor.
- `specs/README.md` — the living tracker (proves 8C-R3 is the lowest non-`Done` unit), the 12-section **Spec format** your deliverable must follow, and the **Workflow** that fixes the determination.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` — **the parent spec; read it closely.** §5 "Required C-11 forward-wave seeds" (8C-R3's exact game set + admission/exit shape: *audit applicable helpers C-01…C-08 for setup/profile diversity, action-tree/effect hashing, and fixture/profile surfaces; preserve behavioral atlas decisions and do not promote trick, graph, event, budget, or scoring policy*), the §4.3 API defaults for C-01…C-05, the §5 ADR-0009 migration protocol, the pilot-discharge rules, and EC-28/EC-30.
- `archive/specs/8c-r1-public-fixed-seat-scaffolding.md` and `archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md` — **the two direct structural precedents.** Your deliverable should mirror their anatomy: the §1 "Determination" subsection, the §3.2 primary applicability and verdict matrix (game × helper, each cell `migrate` / `not-applicable` / `exception` / `already-discharged-by-8C-pilot`), the per-helper sub-surface tables, the §5 wave decomposition (admission/characterization → per-helper waves → consolidation/register/status-flip), the §6 row-for-row exit criteria, and the §7 acceptance-evidence command set and golden/fixture/diff policy. **Re-use this structure; change the game set, drop the "pilot residual" framing R2 used for High Card Duel (R3 has none), and center the setup/profile-diversity (C-03) and domain/fixture (C-08) surfaces the seed names.** Note especially R1/R2's discipline of *classifying* adjacent C-05 surfaces rather than silently omitting them, and the `Golden, fixture, and diff policy` (authorized vs unauthorized byte changes). Note also R2's "Grounded correction to the research brief" mechanism — if your own grounded read contradicts any claim in *this* brief, apply the same correction-in-spec discipline (correct it, cite the evidence, do not reopen the locked unit/game set).
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the source report for Part C / C-01…C-11; the origin of the C-11 retrofit obligation R3 discharges.
- `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` and `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` — the **precedent format** for the per-surface characterization R3 requires before any migration (bytes/hashes/seat-spellings/RNG-vectors/visibility classes, plus the accepted-exception and not-applicable tables). R2's report shows the matrix shape for a heterogeneous mix of applicable and N/A hidden-info surfaces.

**Code seams to inspect directly (read in the repo; *inspect, not read-fully*, and not pasted here):**
- For **each of the four R3 games** (`games/plain_tricks`, `games/flood_watch`, `games/frontier_control`, `games/event_frontier`): `src/effects.rs` (C-01 public **and** any seat-private envelope-constructor candidates — only `plain_tricks` likely has a private surface), `src/ids.rs` (C-02 seat-ID parse/format; note these also define non-seat IDs — `TrickCardId`, `DistrictId`/`EventKind`, `SiteId`/`FactionId` — which are **not** C-02 surfaces), `src/setup.rs` (C-03 — note the `variant.seat_count` / `variant.role_order` validation beyond the bare exact-count predicate; role/faction/variant policy stays game-local), `src/visibility.rs` + `tests/visibility.rs` (C-07 no-leak geometry where applicable; projection/redaction policy that must stay game-local), `src/state.rs` / `src/replay_support.rs` + `tests/golden_traces/*.trace.json` (C-04/C-05 action-tree & stable-byte/hash surfaces; C-08 replay-command / public-export / domain / setup profiles — note `deal-private-no-leak`, `*-no-leak`, `public-replay-export-import` / `replay-export-import`, `round-scoring-breakdown`, `event-choice-resolves-card`, `edict-activation-and-expiry`, `levee-absorption`, `loss-by-inundation` traces), `data/fixtures/*` (C-08 setup/domain fixtures — `*_standard` + variants). These reveal which helper applies to which surface; verify there is **no** existing `game-test-support` dev-dependency in any of the four `Cargo.toml` files (there is none at `be1af6f`).
- Shared landing homes already in place (built on, not rebuilt): `crates/engine-core/src/{action.rs,replay.rs,rng.rs}` (C-01/C-04/C-05/C-09 homes), `crates/game-stdlib/src/seat.rs` (C-03 home), `crates/game-test-support/src/{no_leak.rs,profiles.rs}` (C-07/C-08 homes — `profiles.rs` holds the five drivers incl. `DomainEvidenceV1Driver`/`SetupEvidenceV1Driver` central to this wave), `crates/wasm-api/src/seats.rs` (C-02 legacy-alias adapter + per-game trace seat emission).
- Tooling & guards: `tools/replay-check`, `tools/fixture-check` (validator owners; thin profile dispatch only — game behavior must not move into a tool), `scripts/boundary-check.sh` (noun-free kernel + dev-only `game-test-support` reverse-dep gate), `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs` (note: R3 adds no game, so the catalog check is a regression guard, not a closeout surface).

---

## 3. Settled intentions (these make this brief locked — do not re-decide them)

1. **Target locked: Unit 8C-R3**, "C-11 follow-on — public/co-op/asymmetric/trick support."
   Confirm-and-document the determination with the §1 evidence; **do not** re-open "what's
   next," and **do not** expand into 8C-R4 or Gate 18. This is plumbing/audit work, not a new
   game.
2. **Bounded game set — exactly four, none added, none dropped:** `plain_tricks`, `flood_watch`,
   `frontier_control`, `event_frontier`. **None is a "residual / pilot-discharged" audit** —
   unlike R2's High Card Duel, no R3 game was an 8C pilot, so there are **no
   `already-discharged-by-8C-pilot` cells**; every applicable surface is a fresh audit. (Verify
   against the 8C/C-08 pilot sets and the four games' `Cargo.toml`.)
3. **Scope = applicability audit + per-surface migration of helpers C-01…C-08**, with the seed's
   stated emphasis on **setup/profile diversity, action-tree/effect hashing, and fixture/profile
   surfaces**:
   - **C-01** effect-envelope constructors — public envelopes for all four; `seat_private` only
     where a game actually has a private effect surface (likely `plain_tricks` only; classify the
     others explicitly N/A).
   - **C-02** canonical `seat_<n>` grammar + WASM import-alias adapter (seat IDs only — *not* the
     games' card/district/site/faction IDs).
   - **C-03** `SeatCount`/range exact-count validation **plus the variant/role-order/faction
     diversity** these games carry (cooperative role order, asymmetric factions, variant seat
     counts). The helper covers behavior-free count/range structure only; role/faction/variant
     assignment policy stays game-local.
   - **C-04** action-tree encoding/hash v1; **C-05** `StableBytesWriter` v1 (action-tree surface),
     across these games' richer action trees (trick play, event/edict choices, muster/clash, levee
     actions).
   - **C-06** dev-only `game-test-support` dependency adoption (fresh for all four — none has it
     yet; production/build reverse-dep edge stays absent).
   - **C-07** pairwise no-leak assertion geometry — **applicable for `plain_tricks`** (private
     hands) and for any game's public-observer no-leak surface; **classify `frontier_control`
     (fully public), `flood_watch`, and `event_frontier` (public-observer only, no per-seat
     private holdings) per their real surfaces** — likely public-observer-applicable / seat-private
     N/A, recorded as explicit rows.
   - **C-08** evidence-profile drivers — `replay-command-v1`, `setup-evidence-v1`, **`domain-
     evidence-v1`** (central this wave: connectivity/round/event/scoring fixtures), `public-
     export-v1`, and `seat-private-export-v1` only where a game has that surface.
   - **C-09** unbiased bounded-index RNG and **C-10** non-promotion affirmation carry forward as
     audit checkpoints; adopt the unbiased sampler only if a real characterized game-rule
     bounded-index surface demands it (default: `not applicable`, recorded with rationale —
     `frontier_control` has no game randomness; the others' shuffle/deal stay game-local policy).
4. **Verdict resolution is Session 2's grounded audit, not pre-locked.** Per (game × applicable
   helper) the audit resolves to **exactly one** of: *migrate one surface per reviewable diff*,
   *accepted not-applicable with rationale*, or *accepted exception with named owner +
   compatibility + rollback + next review trigger*. (The fourth R2 verdict,
   *already-discharged-by-8C-pilot*, **does not occur in R3** — no game was a pilot.) **You
   determine each cell's verdict from the actual code at `be1af6f`** — do not guess applicability,
   and do not pre-assume a verdict the code may contradict. Use explicit `not applicable` rows
   over silent omissions (`specs/README.md` spec-format rule). Every official surface in the four
   games is assigned a verdict; there is no unowned "remaining cleanup" bucket. (This mirrors how
   R1/R2 built their matrices: the spec fixes the matrix *shape* and the per-surface protocol; the
   executing tasks pin the bytes.)
5. **Do not re-recommend already-shipped 8C / 8C-R1 / 8C-R2 work as if missing (the sharp-delta
   rule).** The following landed and must be *built on*, not rebuilt: the C-01…C-10 infrastructure
   in `engine-core`/`game-stdlib`; the dev-only `crates/game-test-support` crate (incl. the
   `no_leak` and `profiles` modules with all five drivers); register entries `MSC-8C-001…010`
   **with their R1 and R2 receipt tables**; and the 8C pilots (Race to N, Draughts Lite, High Card
   Duel, River Ledger, Vow Tide, Briar Circuit) plus the R1/R2 migrated games — each discharges
   only its named surfaces. R3 adopts the existing helpers in four *new* games; it adds no new
   helper and changes no shipped helper's contract.
6. **Trick, graph/topology, event, budget/resource, and scoring policy stay behavioral — this is
   the central guardrail of the wave.** The Non-Promotion List (register) and atlas §10/§10B
   explicitly keep game-local: `plain_tricks` deal shape, follow-suit legality, trick lifecycle,
   off-suit/winner resolution, and trick-count scoring; `flood_watch` cooperative role order,
   forecast/event pressure, levee absorption, inundation loss, and budget accounting; `frontier_
   control` site/edge graph, adjacency/movement, clash resolution, muster/reinforce caps, and
   connectivity/round scoring; `event_frontier` named graph sites/trails, event/edict resolution,
   faction operation funding / pass income / Reckoning income, capped resources, and final
   scoring. The audit **adopts scaffolding (envelopes, seat IDs, counts, action-tree bytes,
   no-leak *test geometry*, profile *metadata*) without moving any trick/graph/event/budget/
   scoring/projection policy into shared code.** A shared helper that decides *legality*, *who
   leads/wins a trick*, *graph adjacency or connectivity*, *event/edict resolution*, *how a budget
   accrues or caps*, or *how an outcome scores* is out of bounds and must be `rejected`.
7. **Hard constraints carried into the spec** (from FOUNDATIONS, the boundary doc, ADR
   0008/0009/0004, the register Non-Promotion List, and the parent + R1/R2 "Not allowed" lists):
   no behavioral promotion through the scaffolding lane; no shared helper that decides
   legality / setup / role / faction / trick / graph / event / budget / scoring / projection /
   redaction / authorization / bot-choice; **no silent byte/hash/seat-ID/visibility/RNG-
   consumption change** — every hash-bearing or visibility-bearing flip is a named ADR-0009
   per-surface migration with characterization, before/after evidence, compatibility window, and
   one-surface rollback; **no blanket golden regeneration or "update snapshots" sweep**; **no
   hidden-information leak** into payloads, DOM, storage, logs, effect logs, bot explanations,
   candidate rankings, traces, fixtures, or replay exports, and **no test canary in a committed
   artifact**; `engine-core` stays noun-free; `game-test-support` stays a `[dev-dependencies]`-only
   edge; one surface per reviewable diff; no test deletion/weakening (AGENT-DISCIPLINE failing-test
   protocol); no YAML/DSL/selectors/triggers/formulas in data (the domain fixtures stay typed
   metadata).
8. **Documentation amendments are routine and spec-driven only:** new/updated `MSC-` receipts in
   `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` per migrated/excepted/N-A surface (under the existing
   `MSC-8C-001…010` entries, as R1/R2 did), and the `specs/README.md` status flip. **No
   foundation-doc or ADR amendment is expected.** If you find a foundation/ADR change is genuinely
   required (e.g. a graph/topology, domain-fixture, or hidden-info-taxonomy gap in an accepted
   ADR), do **not** design against doctrine — surface it inside the spec as a flagged **FOUNDATIONS
   §13 ADR-trigger / blocking note** for maintainers, per the user's "unless some foundational
   document should be amended, in which case the spec should indicate it."
9. **Deliverable framing — intermediate spec artifact (not a final drop-in file).** See §7.
10. `assumption:` the unit slug/label defaults to `8c-r3-public-coop-asymmetric-trick-scaffolding`;
    the exact label is one-line-correctable downstream at `/reassess-spec` time. (`8C-R3` is the
    fixed unit ID from the tracker.)
11. `assumption:` each R3 game enforces a fixed `STANDARD_SEAT_COUNT` at the bare predicate level,
    but `flood_watch` / `frontier_control` / `event_frontier` add variant/role/faction validation
    on top (verified at `be1af6f`). If a deeper read shows any game admits a true seat *range*,
    treat C-03 accordingly and note the correction in the spec — but do not expand scope.

---

## 4. The task

Author the bounded **implementation spec for Unit 8C-R3** (`new-spec`, subject = behavior-free
mechanical-scaffolding retrofit/audit, public/co-op/asymmetric/trick wave). The spec turns the
8C-R3 seed row into a concrete, reviewable plan: it confirms-and-documents the determination with
cited evidence; states the objective and the locked four-game scope; lays out the applicability
audit of helpers C-01…C-08 per game (with a per-surface verdict matrix whose cells you fill from
the real code — **including the C-03 variant/role/faction-diversity surfaces and the C-08
`domain-evidence-v1`/`setup-evidence-v1` fixture surfaces the seed centers, and the mixed C-07
no-leak verdicts**); decomposes the work into bounded candidate AGENT-TASK items each naming exact
files/symbols/affected-hash-and-visibility surfaces/rollback scope and obeying the ADR-0009
one-surface-per-diff migration protocol; and maps exit criteria and acceptance evidence (commands,
focused tests, register entries) to the seed's admission/exit and to the parent spec's EC-28/EC-30
sequencing. The spec must keep all trick/graph/event/budget/scoring/reveal policy game-local and be
aligned with `docs/**`; where a doc genuinely needs amending, fold that into the spec as a flagged
ADR-trigger rather than editing doctrine.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular, read
the actual current surfaces of the four R3 games to ground every per-surface verdict (do not
guess applicability; verify it against `src/*.rs` incl. `setup.rs`/`effects.rs`/`visibility.rs`/
`replay_support.rs`, `data/fixtures/*`, and `tests/golden_traces/*` and `tests/visibility.rs`).
Confirm that none of the four games was an 8C pilot and none carries a `game-test-support`
dev-dependency at `be1af6f`, so you author fresh audits rather than re-propose or wrongly cite a
pilot receipt.

Research online only as deeply as it sharpens the deliverable — and calibrate to the target: this
is a behavior-free plumbing retrofit governed by already-accepted ADRs, so external prior art is a
sharpening aid, **not the crux**. Useful angles if you pursue them: characterization-testing
discipline for legacy byte/hash surfaces (Feathers-style), canonical/versioned serialization
framing, dev-only test-scaffolding patterns, table-driven domain-fixture/golden-master evidence
patterns, and information-flow / non-interference no-leak testing geometry as background for the
`plain_tricks` C-07 audit. Cite any external source that shapes a recommendation. Do not let online
research expand the locked scope.

---

## 6. Doctrine & constraints (honor; trimmed to what this unit engages)

- `docs/FOUNDATIONS.md` is the constitution — every decision must satisfy its §11 universal
  invariants (determinism, and the hidden-information / no-leak invariants for `plain_tricks` and
  the public-observer surfaces) and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first (never design against it silently). If you
  hit a §13 ADR trigger, flag it in the spec.
- Authority order: foundation docs govern area docs govern specs govern tickets; if a lower layer
  conflicts with a higher one, the lower layer is wrong and must stop, not design around it.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  `trick`, `site`, `faction`, etc.; typed mechanic nouns belong in `games/*`, shared helpers in
  `game-stdlib`/`game-test-support` only via the register and only when behavior-free.
- **TypeScript never decides legality** and never normalizes/repairs seat IDs — C-02 canonical
  parse/format authority is Rust only; legacy aliases are import-only at the `wasm-api` boundary.
- **No YAML and no DSL without an accepted ADR.** Profile/fixture/domain data stays typed
  content/parameters/metadata — never selectors, conditions, triggers, or formulas (the connectivity/
  scoring/event-resolution semantics stay in Rust, not in fixtures).
- **Determinism:** replay, hashes, RNG consumption, serialization order, and traces stay
  deterministic; any change is an explicit, characterized, versioned ADR-0009 migration.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, candidate rankings, or replay/export exports; test canaries never appear in
  committed artifacts. For `plain_tricks` the seat-private no-leak audit proves it; for the public/
  co-op/asymmetric games the public-observer no-leak surface is a regression guard.
- **No behavioral promotion** through the scaffolding lane (the parent spec's "Not allowed" list
  and the register Non-Promotion List): deal/shuffle policy, reveal timing, trick lifecycle /
  led-suit / trump / winner-leads, graph/topology/adjacency/movement/connectivity, event/edict
  resolution, budget/resource accounting, role/faction/team policy, and scoring/outcome stay
  game-owned.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (AGENT-DISCIPLINE §4); a generic assertion never replaces a specific game assertion (existing
  per-game visibility / no-leak / scoring / connectivity tests are preserved, not subsumed).

---

## 7. Deliverable specification

Produce **one downloadable markdown document**: the Unit 8C-R3 implementation spec.

- **Shape: intermediate spec artifact — NOT a ready-to-drop final repo file.** The user saves
  your output and runs the repository's new-spec pipeline (`/reassess-spec` → `/spec-to-tickets`)
  to revalidate it against the codebase and decompose it into `tickets/`. The eventual target
  path is `specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md` (label one-line-correctable;
  saved under `specs/` for the reassess step, archived to `archive/specs/` only at closeout). Your
  deliverable **does not land there directly** and must not be presented as already-final or as
  skipping the reassess/decompose step.
- **Structure: follow the `specs/README.md` 12-section spec format** exactly, mirroring the
  `8c-r1-public-fixed-seat-scaffolding.md` and `unit-8c-r2-…` anatomy — (1) Header [Spec ID `8C-R3`,
  stage = "Public scaling phase — C-11 follow-on retrofit lane", build gate `8C-R3` (precedes 8C-R4
  and Gate 18), status `Planned` on authoring, date, owner, authority order], plus a **§1
  "Determination"** subsection with the §1 locked-determination evidence above; (2) Objective
  [sourced from the parent spec §5 seed + ROADMAP framing]; (3) Scope [in/out/not-allowed — carry
  the seed's bounds and the parent + R1/R2 "Not allowed" lists, and the §6 trick/graph/event/budget/
  scoring non-promotion guardrail]; (4) Deliverables [concrete artifact tree: the four games' touched
  surfaces + register receipts under `MSC-8C-*` + a new characterization report
  `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md` + `specs/README.md`
  flip]; (5) Work breakdown [bounded candidate AGENT-TASK items (profile `scaffold-refactor`) in
  dependency order: an admission/characterization wave, then per-game/per-helper audit-and-migrate
  waves (including dedicated C-03 setup/variant, C-08 domain/setup-evidence, and — for `plain_tricks`
  — C-07/seat-private waves), then a consolidation + register + status-flip closeout — each item
  names exact files/symbols/affected hash & visibility surfaces/rollback scope, and each migration
  carries the ADR-0009 per-surface protocol R1/R2 used]; (6) Exit criteria [row-for-row, mapped to
  the seed's admission/exit and to parent EC-28/EC-30; include a per-(game × helper) verdict-coverage
  criterion, an explicit no-leak-preserved criterion, and a behavioral-non-promotion criterion];
  (7) Acceptance evidence [the command set — `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, focused crate tests incl.
  `game-test-support`, `cargo test --workspace`, `replay-check`/`fixture-check`/`rule-coverage` for
  the four games, `boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`,
  `check-doc-links.mjs`, `check-catalog-docs.mjs` as a regression guard — plus a golden/fixture/diff
  policy that names the authorized vs unauthorized byte changes, and register before/after]; (8)
  FOUNDATIONS & boundary alignment [principles engaged + stance — center determinism + the
  behavioral non-promotion of trick/graph/event/budget/scoring policy, plus the `plain_tricks`
  no-leak invariant]; (9) Forbidden changes; (10) Documentation updates required [register receipts
  + this index flip; explicitly mark `apps/web/README.md` **not applicable** — no game/web surface];
  (11) Sequencing [predecessor 8C-R2 `Done`; successors 8C-R4 then Gate 18; admission rule]; (12)
  Assumptions [one-line-correctable, including the slug and the per-game fixed-seat/variant
  assumption].
- **Include a per-surface audit matrix** (game × helper C-01…C-08, with each cell a verdict:
  *migrate / not-applicable / exception*) as the spine of the Scope/Work-breakdown sections — this
  is what makes R3 a bounded audit rather than an open sweep. Add the per-helper sub-surface tables
  R1/R2 used (effect public/private constructors; seat parser/output; exact-count **plus
  variant/role/faction validation**; action-tree v1 + adjacent state/effect/view/replay/export/
  diagnostic surfaces; C-07 pairwise matrix by source-seat × viewer × surface **where applicable**;
  C-08 profile classes including **`domain-evidence-v1`** and `setup-evidence-v1`). Fill the verdicts
  from the real code at `be1af6f`. Do **not** carry an `already-discharged-by-8C-pilot` column — no
  R3 game was a pilot.
- **Carry the `assumption:` lines** from §3 into the spec's §12 so the user can override them.
- Append the **locked / no-questions** posture in the spec preamble: it is an authored plan to be
  reassessed and decomposed, not executed code.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The determination is confirmed-and-documented with the §1 evidence (8C-R2 `Done`; 8C-R3 lowest
  non-`Done` row; atlas §10A empty; EC-28/EC-30; Gate 18 hard-gated behind R1…R4), **not**
  re-opened, and the scope never drifts to 8C-R4 or Gate 18.
- Exactly the four named games appear (`plain_tricks`, `flood_watch`, `frontier_control`,
  `event_frontier`); every (game × applicable helper) surface carries an explicit verdict; no
  unowned cleanup bucket; no fifth game.
- **No `already-discharged-by-8C-pilot` cells exist** (no R3 game was an 8C pilot); C-06 dev-dep
  adoption is treated as fresh for all four; the verdict vocabulary is migrate / not-applicable /
  exception only.
- The heterogeneous surfaces are each classified from real code, not auto-defaulted: C-03 covers
  the variant/role/faction diversity; C-08 centers `domain-evidence-v1`/`setup-evidence-v1`; C-07
  seat-private no-leak is applicable for `plain_tricks` and explicitly N/A (with rationale) for the
  public/co-op/asymmetric games' seat-private surface while their public-observer no-leak surface is
  a regression guard.
- No already-shipped 8C / 8C-R1 / 8C-R2 work is re-proposed as missing; R3 adopts existing helpers
  in four new games and adds/changes no helper contract.
- **No shared helper decides deal/shuffle, reveal timing, trick lifecycle / led-suit / trump /
  winner-leads, graph/adjacency/movement/connectivity, event/edict resolution, budget/resource
  accounting, role/faction/team, or scoring/outcome policy** — all of it stays game-local, per the
  Non-Promotion List and atlas §10/§10B.
- Every hash/byte/seat/visibility/RNG-touching item is framed as a named ADR-0009 per-surface
  migration with characterization + before/after + compatibility window + one-surface rollback;
  no blanket golden regeneration; one surface per diff. No hidden datum or test canary appears in
  any committed payload/trace/fixture/export.
- No new doctrine weakens an upstream foundation doc or silently amends an accepted ADR; any
  genuinely required doc change is surfaced as a flagged §13 ADR-trigger inside the spec, not
  enacted.
- The deliverable is the single Unit 8C-R3 spec, framed as an intermediate artifact for
  `/reassess-spec` → `/spec-to-tickets`, following the `specs/README.md` 12-section format and the
  R1/R2 anatomy, and marks `apps/web/README.md` not applicable.
- Every external claim that shaped a recommendation is cited.
- The `be1af6f` fetch-baseline commit contains every file named in the §2 read-in-full list.
