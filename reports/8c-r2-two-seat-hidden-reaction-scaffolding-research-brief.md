# Research brief — Unit 8C-R2 (C-11 follow-on: two-seat hidden/reaction scaffolding) next spec

> **You are Session 2: a locked deep-research-and-author session.** Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-23_e06bdb0.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `e06bdb0` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `e06bdb0`, not the cited string.

**The determination is already made (do not re-open it).** This brief does not ask you to
decide *what* to build next — that decision is locked. The repository workflow
(`specs/README.md` → "Workflow": *pick the lowest non-`Done` unit from the active-epoch
tracker*) selects the next unit deterministically, and that unit is **Unit 8C-R2 — "C-11
follow-on — two-seat hidden/reaction scaffolding."** Your job is to **confirm-and-document**
that determination with cited evidence and then **author the bounded implementation spec for
Unit 8C-R2**. The evidence that fixes the determination, all verifiable in the tree at
`e06bdb0`:

- `specs/README.md` active-epoch tracker: row `8C-R1` is now **`Done`** (completed 2026-06-23),
  and **`8C-R2` is the lowest row whose status is `Not started`** (a `(seed; unwritten)` row).
  `8C-R3`/`8C-R4` sit below it; Gate 18 (Spades) sits below those and is explicitly *"Pending …
  **8C-R1…8C-R4** closed / explicitly not applicable / accepted-excepted."*
- `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register: **`Current debt: None`** (last
  reviewed at the Gate 17 Vow Tide closeout, 2026-06-21). No primitive-promotion debt is open,
  so nothing must close ahead of 8C-R2. (8C-R2 is a *code-scaffolding retrofit*, not a
  mechanic-ladder gate, so the atlas "close debt before the next mechanic-ladder gate"
  interlock does not gate it either.)
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` (the parent spec): §5
  "Required C-11 forward-wave seeds" defines 8C-R2's exact game set and audit shape; work item
  **8C-030** created the four `8C-R1…R4` seed rows in `specs/README.md`; exit criteria **EC-28**
  ("Four bounded C-11 seeds cover every official game exactly once … They remain unimplemented")
  and **EC-30** ("Gate 18 remains after the seeded C-11 waves are closed, not applicable, or
  explicitly excepted") fix the sequencing.
- The immediately-preceding wave, `archive/specs/8c-r1-public-fixed-seat-scaffolding.md`, is the
  **direct structural precedent** for this spec. 8C-R2 is the *next* wave in the same lane and
  the same shape, applied to a different (hidden-information) game set.

So this is a **new-spec** task whose subject is a behavior-free **mechanical-scaffolding
retrofit/audit unit**, not a new game — the second of four C-11 follow-on waves.

**The defining delta from 8C-R1 — read this carefully.** 8C-R1 covered six **public /
perfect-information** games, so it recorded C-07 (pairwise no-leak geometry) and the
`seat-private-export-v1` profile as `not applicable`. **8C-R2 is the hidden-information /
reaction wave.** All four R2 games carry private hands/commitments, viewer-filtered effects,
redacted reveals, and (for `masked_claims`) reaction windows. Therefore in R2 the
hidden-information surfaces are *live, applicable* surfaces, not auto-N/A:

- **C-07 pairwise no-leak assertion geometry** (the shared `game-test-support::no_leak` harness)
  applies to `secret_draft`, `poker_lite`, and `masked_claims`; `high_card_duel` already piloted
  it in Unit 8C (`MSC-8C-007` / UNI8CMECSCA-020) and is a *residual* audit.
- **C-08 `seat-private-export-v1`** (and possibly `domain-evidence-v1`) profiles apply where a
  game has seat-private export / viewer-scoped fixtures.
- **C-01 private effect envelopes** (`EffectEnvelope::seat_private`) apply alongside the public
  constructors R1 covered.
- **ADR 0004 (hidden-information replay/export taxonomy)** is consequently a *central* read for
  this unit, not the minor background read it was for R1.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground
truth. The user named **`docs/**` (all files and folders), `templates/**`, and
`specs/README.md`** as the mandatory floor; read the entire `docs/**` and `templates/**` trees.
Each line below states why a file is load-bearing *for the 8C-R2 spec*; files in the named floor
that only bear incidentally are covered by one directory-level reason per tier.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect; the ADR status index (0004/0008/0009 all `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants (esp. the hidden-information / no-leak invariants this wave centers on), §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — boundary & scaffolding law (the crux for this unit)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the Rust/WASM boundary, and the action/view/effect/replay/determinism model; the "narrowest lawful owner wins" ownership matrix the retrofit must honor.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `game-test-support` / `games/*` / static-data boundary; the retrofit may adopt shared helpers but must never move game behavior (reveal/reaction/projection policy) across this line or add nouns to `engine-core`.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law; **§10** initial atlas table, **§10A** empty open-promotion-debt register (proves nothing precedes this unit), and **§10B** deferred/candidate register — read §10B closely: it records `deterministic shuffle / private hand / staged reveal`, `simultaneous commitment/reveal`, `reaction window/pending response`, and `bounded pledge rounds` as **deferred/local-only behavioral pressure** for exactly these four games. Those rows are the boundary the scaffolding lane must **not** breach.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — **the governing register for this unit**: the Entry Schema, Decision States, the **Non-Promotion List** (reveal timing, projection/redaction, reaction windows, betting/pledge, scoring/outcome — all the policy this wave's games embody and must keep game-local), the landed `MSC-8C-001…010` entries with their `R1 public fixed-seat receipts` tables, and the per-entry "next review trigger = C-11 game audits". Every R2 migration adds/updates a register entry here (the R2 receipts go under the same `MSC-8C-*` entries the way R1's did).
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted governance ADR for the scaffolding lane; defines what may be extracted and the register-first discipline.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2; **every hash-bearing surface flip in R2 follows its per-surface migration protocol** (characterize → parallel surface → classify unchanged/parallel-new/intentional-migration → compatibility window → rollback).
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — **central for this wave**: the hidden-information replay/export visibility taxonomy. The C-07 no-leak audit, the `seat-private-export-v1` / public-export profile audits, and any view/effect/export surface in these four games must preserve this taxonomy exactly; private data must remain absent from every viewer-scoped or public surface.

**Tier 3 — evidence, determinism, hidden-info & multi-seat law**
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden-trace obligations, deterministic replay/hash discipline, **no-leak tests**, benchmarks, CI; the acceptance-evidence backbone of the spec.
- `docs/TRACE-SCHEMA-v1.md` — trace/replay-fixture schema law; load-bearing because C-04/C-05/C-08 audits touch trace bytes (including seat-private and public-export traces) and any change is an ADR-0009 migration packet, never a silent edit.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the evidence-fixture profile contract (ADR 0009); separates command traces, setup/domain fixtures, and **viewer-scoped exports** — the C-08 profile-driver audit (including `seat-private-export-v1`) conforms to this.
- `docs/AI-BOTS.md` — bot policy and **bot-input / bot-explanation no-leak law**; load-bearing because the C-07 pairwise matrix for these hidden-info games asserts over bot-input and bot-explanation surfaces (the High Card pilot already did), and no bot may read disallowed private state.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat/surface obligations and the canonical seat-declaration/viewer rules; R2's games are fixed-two-seat, but the canonical-seat-grammar (C-02), seat-count (C-03), and viewer-scoped-surface (C-07/C-08) audits must respect it.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, the failing-test protocol the spec's work breakdown and "one surface per diff" rollback rule must encode.

**Tier 4 — the named floor remainder + planning artifacts**
- Remaining `docs/**` — `OFFICIAL-GAME-CONTRACT.md`, `UI-INTERACTION.md`, `IP-POLICY.md`, `SOURCES.md`, `WASM-CLIENT-BOUNDARY.md`, `ROADMAP.md`, `archival-workflow.md`, `adr/0001…0003/0005/0006/0007`, `adr/ADR-TEMPLATE.md` — part of the mandatory `docs/**` floor; mostly background for this behavior-free unit (no new game, no UI, no bot-strategy change). Read for completeness; `WASM-CLIENT-BOUNDARY.md` is the one with direct bearing because C-02's legacy seat-alias adapter lives at the `wasm-api` boundary, and `IP-POLICY.md` matters only as a regression guard (these are original games, not licensed content).
- All of `templates/**` — `AGENT-TASK.md` is the packet shape the spec's work breakdown decomposes into (each work item is a candidate AGENT-TASK; profile `scaffold-refactor`); the `GAME-*` and evidence/admission templates show the evidence vocabulary the spec's acceptance section should reference. Part of the mandatory floor.
- `specs/README.md` — the living tracker (proves 8C-R2 is the lowest non-`Done` unit), the 12-section **Spec format** your deliverable must follow, and the **Workflow** that fixes the determination.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` — **the parent spec; read it closely.** §5 "Required C-11 forward-wave seeds" (8C-R2's exact game set + admission/exit shape: *"Audit applicable helpers C-01…C-08, especially private effects/views, replay/export, and no-leak geometry … adopt scaffolding without moving reveal/reaction policy; High Card's 8C pilot discharges only its named no-leak surface"*), the §4.3 API defaults for C-01…C-05, the §5 ADR-0009 migration protocol, the pilot-discharge rules, and EC-28/EC-30.
- `archive/specs/8c-r1-public-fixed-seat-scaffolding.md` — **the direct structural precedent.** Your deliverable should mirror its anatomy: the §1 "Locked determination", the §3.2 primary applicability and verdict matrix (game × helper, each cell `migrate` / `not-applicable` / `exception` / `already-discharged-by-8C-pilot`), the per-helper sub-surface tables (§3.3–§3.8), the §5 wave decomposition (admission/characterization → per-helper waves → consolidation/register/status-flip), the §6 row-for-row exit criteria, and the §7 acceptance-evidence command set and golden/fixture/diff policy. **Re-use this structure; change the game set, and add the hidden-information surfaces (C-07, seat-private export, private envelopes) that R1 marked N/A.** Note especially R1's "Adjacent C-05 surfaces … explicitly classified rather than silently omitted" discipline and its `7.4 Golden, fixture, and diff policy` (authorized vs unauthorized byte changes).
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the source report for Part C / C-01…C-11; the origin of the C-11 retrofit obligation R2 discharges.
- `reports/8c-mechanical-scaffolding-characterization.md` and `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` — the **precedent format** for the per-surface characterization R2 requires before any migration (bytes/hashes/seat-spellings/RNG-vectors/visibility classes, plus the accepted-exception tables). The 8C report pins High Card Duel's pilot receipts your residual audit must cite rather than recreate.

**Code seams to inspect directly (read in the repo; *inspect, not read-fully*, and not pasted here):**
- For **each of the four R2 games** (`games/high_card_duel`, `games/secret_draft`, `games/poker_lite`, `games/masked_claims`): `src/effects.rs` (C-01 public **and seat-private** envelope-constructor candidates), `src/ids.rs` (C-02 seat-ID parse/format), `src/setup.rs` (C-03 exact-two-seat validation — each currently uses a hand-written `seats.len() != STANDARD_SEAT_COUNT as usize` predicate), `src/visibility.rs` + `tests/visibility.rs` (C-07 no-leak geometry; projection/redaction policy that must stay game-local), `src/replay_support.rs` + `tests/golden_traces/*.trace.json` (C-04/C-05 action-tree & stable-byte/hash surfaces; C-08 replay-command / public-export / **seat-private-export** profiles — note the `seat-private-view`, `*-no-leak`, `*-redacted`, and `public-replay-export-import` traces), `data/fixtures/*` where present (C-08 setup/domain fixtures). These reveal which helper applies to which surface and which are already canonical / already shipped by High Card's pilot.
- Shared landing homes already in place (built on, not rebuilt): `crates/engine-core/src/{action.rs,replay.rs,rng.rs}` (C-01/C-04/C-05/C-09 homes), `crates/game-stdlib/src/seat.rs` (C-03 home), `crates/game-test-support/src/{no_leak.rs,profiles.rs}` (**C-07/C-08 homes — now central to this wave**), `crates/wasm-api/src/seats.rs` (C-02 legacy-alias adapter + per-game trace seat emission).
- Tooling & guards: `tools/replay-check`, `tools/fixture-check` (validator owners; thin profile dispatch only — game behavior must not move into a tool), `scripts/boundary-check.sh` (noun-free kernel + dev-only `game-test-support` reverse-dep gate), `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs` (note: R2 adds no game, so the catalog check is a regression guard, not a closeout surface).

---

## 3. Settled intentions (these make this brief locked — do not re-decide them)

1. **Target locked: Unit 8C-R2**, "C-11 follow-on — two-seat hidden/reaction scaffolding."
   Confirm-and-document the determination with the §1 evidence; **do not** re-open "what's
   next," and **do not** expand into 8C-R3/R4 or Gate 18. This is plumbing/audit work, not a new
   game.
2. **Bounded game set — exactly four, none added, none dropped:** `high_card_duel` (residual
   audit), `secret_draft`, `poker_lite`, `masked_claims`. (`high_card_duel` is "residual" because
   its 8C pilots already discharged named surfaces — notably the C-07 pairwise no-leak harness
   (`MSC-8C-007`) and the C-04 action-tree v1 pilot (`MSC-8C-004`); R2 audits only what those
   pilots did **not** cover.)
3. **Scope = applicability audit + per-surface migration of helpers C-01…C-08**, with the seed's
   stated emphasis on **private effects/views, replay/export, and no-leak geometry**:
   - **C-01** effect-envelope constructors — public **and** `seat_private` envelopes.
   - **C-02** canonical `seat_<n>` grammar + WASM import-alias adapter.
   - **C-03** `SeatCount`/range exact-two-seat validation (each game currently hand-rolls it).
   - **C-04** action-tree encoding/hash v1; **C-05** `StableBytesWriter` v1 (action-tree surface).
   - **C-06** dev-only `game-test-support` dependency discipline (checkpoint).
   - **C-07** pairwise no-leak assertion geometry — **applicable this wave** for the
     hidden-information games (R1 marked it N/A; R2 does not).
   - **C-08** evidence-profile drivers — `replay-command-v1`, `setup-evidence-v1`,
     `public-export-v1`, and **`seat-private-export-v1`** / `domain-evidence-v1` where the game
     actually has that surface.
   - **C-09** unbiased bounded-index RNG and **C-10** non-promotion affirmation carry forward as
     audit checkpoints; adopt the unbiased sampler only if a real characterized game-rule
     bounded-index surface demands it (default: `not applicable`, recorded with rationale).
4. **Verdict resolution is Session 2's grounded audit, not pre-locked.** Per (game × applicable
   helper) the audit resolves to **exactly one** of: *migrate one surface per reviewable diff*,
   *accepted not-applicable with rationale*, *accepted exception with named owner + compatibility
   + rollback + next review trigger*, or *already-discharged-by-8C-pilot* (verify the receipt,
   do not rebuild). **You determine each cell's verdict from the actual code at `e06bdb0`** —
   do not guess applicability, and do not pre-assume a verdict the code may contradict. Use
   explicit `not applicable` rows over silent omissions (`specs/README.md` spec-format rule).
   Every official surface in the four games is assigned a verdict; there is no unowned "remaining
   cleanup" bucket. (This mirrors how 8C-R1 built its matrix: the spec fixes the matrix *shape*
   and the per-surface protocol; the executing tasks pin the bytes.)
5. **Do not re-recommend already-shipped 8C/8C-R1 work as if missing (the sharp-delta rule).**
   The following landed and must be *built on*, not rebuilt: the C-01…C-10 infrastructure in
   `engine-core`/`game-stdlib`; the dev-only `crates/game-test-support` crate (incl. the
   `no_leak` and `profiles` modules); register entries `MSC-8C-001…010` **with their R1 receipt
   tables**; and the 8C pilots (Race to N, Draughts Lite, **High Card Duel**, River Ledger, Vow
   Tide, Briar Circuit), each of which **discharges only its named surfaces**. High Card Duel's
   pilot already covers its C-04 action-tree v1 surface and its C-07 observer/seat-0/seat-1
   pairwise no-leak matrix — R2 verifies those receipts and audits only High Card's *residual,
   unapplied* surfaces (e.g. C-01 private/public constructors, C-02 parser, C-03 setup, C-08
   profiles) that the pilot did not name.
6. **Reveal and reaction policy stays behavioral — this is the central guardrail of the wave.**
   The Non-Promotion List (register) and atlas §10B explicitly keep game-local: `secret_draft`
   simultaneous commitment/reveal + visible draft-pool removal; `masked_claims` reaction
   window / pending-response policy and claim-path redaction; `poker_lite` bounded pledge rounds,
   showdown reveal, and shared-pool allocation; and every game's projection/redaction and reveal
   timing. The audit **adopts scaffolding (envelopes, seat IDs, counts, action-tree bytes,
   no-leak *test geometry*, profile *metadata*) without moving any reveal/reaction/projection
   policy into shared code.** A shared helper that decides *what is hidden*, *when it reveals*,
   *who may respond*, or *how a pot/pledge resolves* is out of bounds and must be `rejected`.
7. **Hard constraints carried into the spec** (from FOUNDATIONS, the boundary doc, ADR
   0008/0009/0004, the register Non-Promotion List, and the parent + R1 "Not allowed" lists):
   no behavioral promotion through the scaffolding lane; no shared helper that decides
   legality / setup / reveal timing / authorization / projection / redaction / scoring / outcome /
   bot-choice; **no silent byte/hash/seat-ID/visibility/RNG-consumption change** — every
   hash-bearing or visibility-bearing flip is a named ADR-0009 per-surface migration with
   characterization, before/after evidence, compatibility window, and one-surface rollback;
   **no blanket golden regeneration or "update snapshots" sweep**; **no hidden-information leak**
   into payloads, DOM, storage, logs, effect logs, bot explanations, candidate rankings, traces,
   fixtures, or replay exports, and **no test canary in a committed artifact**; `engine-core`
   stays noun-free; `game-test-support` stays a `[dev-dependencies]`-only edge; one surface per
   reviewable diff; no test deletion/weakening (AGENT-DISCIPLINE failing-test protocol); no
   YAML/DSL/selectors/triggers in data.
8. **Documentation amendments are routine and spec-driven only:** new/updated `MSC-` receipts in
   `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` per migrated/excepted/N-A surface (under the existing
   `MSC-8C-001…010` entries, as R1 did), and the `specs/README.md` status flip. **No
   foundation-doc or ADR amendment is expected.** If you find a foundation/ADR change is genuinely
   required (e.g. a hidden-info-taxonomy gap in ADR 0004), do **not** design against doctrine —
   surface it inside the spec as a flagged **FOUNDATIONS §13 ADR-trigger / blocking note** for
   maintainers, per the user's "unless the docs themselves need to be amended, in which case it
   should be included in the spec."
9. **Deliverable framing — intermediate spec artifact (not a final drop-in file).** See §7.
10. `assumption:` the unit slug/label defaults to `8c-r2-two-seat-hidden-reaction-scaffolding`;
    the exact label is one-line-correctable downstream at `/reassess-spec` time. (`8C-R2` is the
    fixed unit ID from the tracker.)
11. `assumption:` the four R2 games are all fixed-two-seat (verified: each `setup.rs` rejects on
    `seats.len() != STANDARD_SEAT_COUNT as usize`). If a deeper read shows any game admits a seat
    *range*, treat C-03 accordingly and note the correction in the spec — but do not expand scope.

---

## 4. The task

Author the bounded **implementation spec for Unit 8C-R2** (`new-spec`, subject = behavior-free
mechanical-scaffolding retrofit/audit, hidden-information wave). The spec turns the 8C-R2 seed
row into a concrete, reviewable plan: it confirms-and-documents the determination with cited
evidence; states the objective and the locked four-game scope; lays out the applicability audit
of helpers C-01…C-08 per game (with a per-surface verdict matrix whose cells you fill from the
real code, **including the C-07 no-leak / seat-private-export / private-envelope surfaces that
R1 marked N/A**); decomposes the work into bounded candidate AGENT-TASK items each naming exact
files/symbols/affected-hash-and-visibility surfaces/rollback scope and obeying the ADR-0009
one-surface-per-diff migration protocol; and maps exit criteria and acceptance evidence
(commands, focused tests, register entries) to the seed's admission/exit and to the parent
spec's EC-28/EC-30 sequencing. The spec must keep all reveal/reaction policy game-local and be
aligned with `docs/**`; where a doc genuinely needs amending, fold that into the spec as a
flagged ADR-trigger rather than editing doctrine.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular, read
the actual current surfaces of the four R2 games to ground every per-surface verdict (do not
guess applicability; verify it against `src/*.rs` incl. `effects.rs`/`visibility.rs`, `data/`,
and `tests/golden_traces/*` and `tests/visibility.rs`). Confirm precisely which High Card Duel
surfaces the 8C pilot already discharged so you cite the receipt rather than re-propose the work.

Research online as deeply as it sharpens the deliverable — but calibrate to the target: this is
a behavior-free plumbing retrofit governed by already-accepted ADRs, so external prior art is a
sharpening aid, not the crux. Useful angles if you pursue them: characterization-testing
discipline for legacy byte/hash surfaces (Feathers-style), canonical/versioned serialization
framing, dev-only test-scaffolding patterns, and **table-driven / matrix no-leak (information-
flow / non-interference) testing geometry** as background for the C-07 audit. Cite any external
source that shapes a recommendation. Do not let online research expand the locked scope.

---

## 6. Doctrine & constraints (honor; trimmed to what this unit engages)

- `docs/FOUNDATIONS.md` is the constitution — every decision must satisfy its §11 universal
  invariants (especially hidden-information / no-leak invariants) and clear its §12 stop
  conditions; a genuine divergence requires an accepted ADR superseding the affected principle
  first (never design against it silently). If you hit a §13 ADR trigger, flag it in the spec.
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
  explanations, candidate rankings, or replay/export exports; test canaries never appear in
  committed artifacts. This is the central invariant of the wave — the C-07 audit exists to prove
  it, and every C-01-private / C-08-seat-private / view / export surface must preserve it.
- **No behavioral promotion** through the scaffolding lane (the parent spec's "Not allowed" list
  and the register Non-Promotion List): reveal timing, commitment/reveal policy, reaction
  windows / pending responder policy, projection/redaction, betting/pledge/pot policy, and
  scoring/outcome stay game-owned.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (AGENT-DISCIPLINE §4); a generic assertion never replaces a specific game assertion (existing
  per-game visibility / no-leak tests are preserved, not subsumed).

---

## 7. Deliverable specification

Produce **one downloadable markdown document**: the Unit 8C-R2 implementation spec.

- **Shape: intermediate spec artifact — NOT a ready-to-drop final repo file.** The user saves
  your output and runs the repository's new-spec pipeline (`/reassess-spec` → `/spec-to-tickets`)
  to revalidate it against the codebase and decompose it into `tickets/`. The eventual target
  path is `specs/8c-r2-two-seat-hidden-reaction-scaffolding.md` (label one-line-correctable;
  saved under `specs/` for the reassess step, archived to `archive/specs/` only at closeout). Your
  deliverable **does not land there directly** and must not be presented as already-final or as
  skipping the reassess/decompose step.
- **Structure: follow the `specs/README.md` 12-section spec format** exactly, mirroring the
  `8c-r1-public-fixed-seat-scaffolding.md` anatomy — (1) Header [Spec ID `8C-R2`, stage =
  "Public scaling phase — C-11 follow-on retrofit lane", build gate `8C-R2` (precedes 8C-R3/R4
  and Gate 18), status `Planned` on authoring, date, owner, authority order], plus a **§1
  "Determination"** subsection with the §1 locked-determination evidence above; (2) Objective
  [sourced from the parent spec §5 seed + ROADMAP framing]; (3) Scope [in/out/not-allowed —
  carry the seed's bounds and the parent + R1 "Not allowed" lists, and the §6 reveal/reaction
  non-promotion guardrail]; (4) Deliverables [concrete artifact tree: the four games' touched
  surfaces + register receipts under `MSC-8C-*` + a new characterization report
  `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` + `specs/README.md`
  flip]; (5) Work breakdown [bounded candidate AGENT-TASK items (profile `scaffold-refactor`) in
  dependency order: an admission/characterization wave, then per-game/per-helper audit-and-migrate
  waves (including dedicated C-07 no-leak and C-08 seat-private-export waves), then a consolidation
  + register + status-flip closeout — each item names exact files/symbols/affected hash &
  visibility surfaces/rollback scope, and each migration carries the ADR-0009 10-point protocol R1
  used in its §5.1]; (6) Exit criteria [row-for-row, mapped to the seed's admission/exit and to
  parent EC-28/EC-30; include a per-(game × helper) verdict-coverage criterion and an explicit
  no-leak-preserved criterion]; (7) Acceptance evidence [the command set — `cargo fmt`, focused
  crate tests incl. `game-test-support`, `cargo test --workspace --all-targets`,
  `replay-check`/`fixture-check` for the four games, `boundary-check.sh`,
  `cargo tree --workspace -e normal --invert game-test-support`, `check-doc-links.mjs`,
  `check-catalog-docs.mjs` as a regression guard — plus a golden/fixture/diff policy that names
  the authorized vs unauthorized byte changes, and register before/after]; (8) FOUNDATIONS &
  boundary alignment [principles engaged + stance — center the hidden-info / no-leak invariants];
  (9) Forbidden changes; (10) Documentation updates required [register receipts + this index flip;
  explicitly mark `apps/web/README.md` **not applicable** — no game/web surface]; (11) Sequencing
  [predecessor 8C-R1 `Done`; successors 8C-R3/R4 then Gate 18; admission rule]; (12) Assumptions
  [one-line-correctable, including the slug and the fixed-two-seat assumption].
- **Include a per-surface audit matrix** (game × helper C-01…C-08, with each cell a verdict:
  *migrate / not-applicable / exception / already-discharged-by-8C-pilot*) as the spine of the
  Scope/Work-breakdown sections — this is what makes R2 a bounded audit rather than an open sweep.
  Add the per-helper sub-surface tables R1 used (effect public/private constructors; seat
  parser/output; exact-count; action-tree v1 + adjacent state/effect/view/replay/export/
  diagnostic surfaces; **C-07 pairwise matrix by source-seat × viewer × surface**; C-08 profile
  classes including `seat-private-export-v1`). Fill the verdicts from the real code at `e06bdb0`.
- **Carry the `assumption:` lines** from §3 into the spec's §12 so the user can override them.
- Append the **locked / no-questions** posture in the spec preamble: it is an authored plan to be
  reassessed and decomposed, not executed code.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The determination is confirmed-and-documented with the §1 evidence (8C-R1 `Done`; 8C-R2 lowest
  non-`Done` row; atlas §10A empty; EC-28/EC-30), **not** re-opened, and the scope never drifts
  to 8C-R3/R4 or Gate 18.
- Exactly the four named games appear (`high_card_duel`, `secret_draft`, `poker_lite`,
  `masked_claims`); every (game × applicable helper) surface carries an explicit verdict; no
  unowned cleanup bucket; no fifth game.
- The hidden-information surfaces R1 deferred are treated as **applicable** here: C-07 pairwise
  no-leak geometry, `seat-private-export-v1` profiles, and private (`seat_private`) effect
  envelopes each have a real per-game verdict (migrate / N-A / exception / pilot-discharged).
- No already-shipped 8C / 8C-R1 work is re-proposed as missing; High Card Duel's pilot is treated
  as discharging only its named C-04 / C-07 surfaces, and R2 audits only its residual surfaces.
- **No shared helper decides reveal timing, commitment/reveal, reaction-window / pending-response,
  projection/redaction, betting/pledge/pot, or scoring/outcome policy** — all of it stays
  game-local, per the Non-Promotion List and atlas §10B.
- Every hash/byte/seat/visibility/RNG-touching item is framed as a named ADR-0009 per-surface
  migration with characterization + before/after + compatibility window + one-surface rollback;
  no blanket golden regeneration; one surface per diff. No hidden datum or test canary appears in
  any committed payload/trace/fixture/export.
- No new doctrine weakens an upstream foundation doc or silently amends an accepted ADR; any
  genuinely required doc change (incl. an ADR-0004 hidden-info-taxonomy gap) is surfaced as a
  flagged §13 ADR-trigger inside the spec, not enacted.
- The deliverable is the single Unit 8C-R2 spec, framed as an intermediate artifact for
  `/reassess-spec` → `/spec-to-tickets`, following the `specs/README.md` 12-section format and the
  R1 anatomy, and marks `apps/web/README.md` not applicable.
- Every external claim that shaped a recommendation is cited.
- The `e06bdb0` fetch-baseline commit contains every file named in the §2 read-in-full list.
