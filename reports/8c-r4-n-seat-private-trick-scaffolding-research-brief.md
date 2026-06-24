# Research brief — Unit 8C-R4 (C-11 follow-on: N-seat/private/trick scaffolding) next spec

> **You are Session 2: a locked deep-research-and-author session.** Produce the deliverable
> directly as a downloadable markdown document. **Do not interview, do not ask clarifying
> questions** — the requirements below are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-24_0d01901.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `0d01901` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `0d01901`, not the cited string. (In
particular, the R1/R2/R3 characterization reports cite their *own* pre-series base commits for
diff audits — those are correct for their waves but are **not** your baseline.)

**The determination is already made (do not re-open it).** This brief does not ask you to
decide *what* to build next — that decision is locked. The repository workflow
(`specs/README.md` → "Workflow": *pick the lowest non-`Done` unit from the active-epoch
tracker*) selects the next unit deterministically, and that unit is **Unit 8C-R4 — "C-11
follow-on — N-seat/private/trick support."** Your job is to **confirm-and-document** that
determination with cited evidence and then **author the bounded implementation spec for Unit
8C-R4**. The evidence that fixes the determination, all verifiable in the tree at `0d01901`:

- `specs/README.md` active-epoch tracker: row `8C-R3` is now **`Done`** (completed 2026-06-24),
  and **`8C-R4` is the lowest row whose status is `Not started`** (a `(seed; unwritten)` row).
  Gate 18 (Spades) sits below it and is explicitly *"Pending … **8C-R1…8C-R4** closed /
  explicitly not applicable / accepted-excepted."*
- `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register: **`Current debt: _None_.`** No
  primitive-promotion debt is open (last reviewed at Gate 17 Vow Tide closeout; the
  `game-stdlib::trick_taking` promotion conformed Plain Tricks/Briar Circuit in-gate and Vow
  Tide uses the helper). So nothing must close ahead of 8C-R4. (8C-R4 is a *code-scaffolding
  retrofit*, not a mechanic-ladder gate, so the atlas "close debt before the next
  mechanic-ladder gate" interlock does not gate it either.)
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` (the parent spec): §5
  "Required C-11 forward-wave seeds" defines 8C-R4's exact game set and audit shape; work item
  **8C-030** created the four `8C-R1…R4` seed rows in `specs/README.md`; exit criteria **EC-28**
  ("Four bounded C-11 seeds cover every official game exactly once … pilot games limited to
  residual audits … They remain unimplemented") and **EC-30** ("Gate 18 remains after the
  seeded C-11 waves are closed, not applicable, or explicitly excepted") fix the sequencing.
- The three preceding waves — `archive/specs/8c-r1-public-fixed-seat-scaffolding.md`,
  `archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md`, and
  `archive/specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md` — are the **direct
  structural precedents**. 8C-R4 is the *fourth and final* wave in the same lane and shape,
  applied to the N-seat / private-hand / trick-taking game set.

So this is a **new-spec** task whose subject is a behavior-free **mechanical-scaffolding
residual retrofit/audit unit**, not a new game — the last of four C-11 follow-on waves, the one
that finally clears the Gate 18 admission interlock.

**The defining delta from R1/R2/R3 — read this carefully. 8C-R4 is the residual /
pilot-discharged wave; it is the mirror image of R3.** R3 covered four games **none of which was
an 8C pilot** (so it had *zero* `already-discharged-by-8C-pilot` cells and every surface was a
fresh audit). **8C-R4 is the opposite: all three of its games — `river_ledger`, `vow_tide`,
`briar_circuit` — were 8C pilots (and the C-08 fixture pilots were Race / River / Vow / Briar),
and all three already carry a `game-test-support` dev-dependency** (verify in each
`games/<g>/Cargo.toml` at `0d01901` — present in all three). Therefore the
`already-discharged-by-8C-pilot` verdict is **central** to this wave, and your audit is largely a
**residual closeout**: for each (game × helper) surface you must distinguish what the 8C pilot
*already named and discharged* (do **not** re-propose it as missing) from the **residual
surfaces the pilot did not name**, which are this wave's real work. The seed's own admission text
is decisive: *"The 8C River/Vow/Briar pilots discharge only their named surfaces."* The wave's
four distinguishing properties, verified at `0d01901`:

1. **The three games carry heavy named 8C-pilot discharge already in the register** — build on
   it, do not rebuild. From `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and the
   `UNI8CMECSCA-005…030` tickets:
   - **River Ledger** was the pilot for **C-01** public/private effect envelopes (MSC-8C-001
     duplicate sites), **C-02** canonical seat helpers (`games/river_ledger/src/ids.rs`),
     **C-03** the 3-6 seat validation + ring/index arithmetic (MSC-8C-003), **C-07** the pairwise
     no-leak harness piloted across **counts 3-6** for observer + every seat over view / effect /
     action / diagnostic / export / showdown / bot-input / bot-explanation surfaces
     (`UNI8CMECSCA-021`), **C-08** `setup-evidence-v1` (`UNI8CMECSCA-024`,
     `river_ledger_3p_standard.fixture.json`), and **C-09** the unbiased bounded-index sampler —
     River's *local* unbiased helper was replaced by `next_index_unbiased_v1` (`UNI8CMECSCA-017`).
   - **Vow Tide** was the pilot for **C-02** canonical seats (`games/vow_tide/src/ids.rs`),
     **C-06** the `game-test-support` dev-dependency, and **C-08** `public-export-v1` **and**
     `seat-private-export-v1` (`UNI8CMECSCA-025`, the public + all-viewer seat-private export
     fixtures).
   - **Briar Circuit** was the pilot for **C-06** the dev-dependency and **C-08**
     `domain-evidence-v1` (`UNI8CMECSCA-026`, the moon-scoring + first-trick-exception fixtures).
2. **River Ledger's residual is genuinely larger than a pilot leftover because Gate 15.1 added
   all-in / side-pot surfaces *after* the 8C pilot.** The `river_ledger` 8C pilot characterized
   the base Hold'Em surfaces; `archive/specs/gate-15-1-river-ledger-all-in-side-pots.md` then
   added finite stacks, all-in qualification, layered side pots, uncalled returns, and
   viewer-safe allocation explanations with their own v2 replay/no-leak/bench evidence. Those
   side-pot/all-in action-tree, state, export, and no-leak surfaces are **residual** to the 8C
   pilot and are a primary R4 audit target — classify each from the real code (migrate /
   already-discharged / N-A / exception), never assume the pilot covered them.
3. **C-07 seat-private no-leak and C-08 `seat-private-export-v1` are uniformly applicable, not
   mixed.** Unlike R3 (where only `plain_tricks` had real private hands and the three public/co-op
   games were public-observer-only), **all three R4 games have genuine per-seat private hands**:
   River's two hole cards per seat, Vow Tide's and Briar Circuit's full private hands. So ADR 0004
   (hidden-information replay/export taxonomy) is **uniformly central** across the wave, and the
   seat-private no-leak / seat-private-export surfaces are *applicable* for every game (the
   verdict is then `already-discharged-by-8C-pilot` where the 8C pilot named it — e.g. River's
   `UNI8CMECSCA-021` no-leak pilot and Vow's `seat-private-export-v1` pilot — or `migrate` for the
   residual). You still assign each cell from the code; do not silently omit.
4. **The seed centers the widest seat matrices and "RNG-sampler divergence."** The seed's helper
   focus is *"3–7 seat matrices, private hands, pass/bid/deal order, public/seat-private export,
   RNG-sampler divergence … Full seat/profile/no-leak audit; any RNG algorithm migration is
   separately versioned."* Concretely: River 3-6 seats, Vow Tide 3-7 seats (the widest in the
   corpus, with dealer rotation + changing hand size + bidding/contract order), Briar Circuit
   fixed-4. C-03 must respect that diversity while keeping dealer/bid/contract/partnership policy
   **game-local**. For C-09, audit whether Vow Tide and Briar Circuit still hold *local* unbiased
   bounded-index samplers not yet migrated to `next_index_unbiased_v1` (River's was migrated in
   the 8C pilot) — adopt the shared sampler only on a real characterized game-rule bounded-index
   surface with byte-identical consumption proof, and treat any actual RNG *algorithm* change as a
   separately versioned ADR-0009 migration, never a silent re-seed.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground truth.
The user named **`docs/**` (all files and folders), `templates/**`, and `specs/README.md`** as
the mandatory floor; read the entire `docs/**` and `templates/**` trees. Each line below states
why a file is load-bearing *for the 8C-R4 spec*; files in the named floor that only bear
incidentally are covered by one directory-level reason per tier.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect; the ADR status index (0004/0008/0009 all `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants (esp. the determinism and hidden-information / no-leak invariants — load-bearing because all three R4 games have private hands), §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — boundary & scaffolding law (the crux for this unit)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, the Rust/WASM boundary, and the action/view/effect/replay/determinism model; the "narrowest lawful owner wins" ownership matrix the retrofit must honor.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `game-test-support` / `games/*` / static-data boundary; the retrofit may adopt shared helpers but must never move game behavior (betting/pot/side-pot, trick/led-suit/trump, bid/contract, deal/shuffle, partnership, scoring, projection policy) across this line or add nouns to `engine-core`.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law; **§10** initial atlas table, **§10A** empty open-promotion-debt register (proves nothing precedes this unit — the `game-stdlib::trick_taking` promotion conformed Plain/Briar in-gate and Vow uses it; **no open debt**), and **§10B** deferred/candidate register (records the `deterministic shuffle / private hand / staged reveal`, `public resource accounting / shared ledgers`, and trick-taking rows that classify the R4 games' behavioral surfaces). Those rows are the behavioral boundary the scaffolding lane must **not** breach — note especially that `game-stdlib::trick_taking` is an *already-promoted behavioral atlas helper with explicit scope* (Vow/Briar use it), so R4 must not re-treat trick policy as scaffolding.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — **the governing register for this unit; read it in full** (it is long — ~850 lines). The Entry Schema, Decision States (incl. `promotion-debt-open`), the **Non-Promotion List** (deal/shuffle, reveal timing, projection/redaction, **betting/bidding/contribution/raise/call/fold**, **pot construction / side-pot allocation / remainder order**, trick lifecycle/led-suit/trump/winner-leads, **teams/partnerships**, graph/topology, **resource accounting / shared ledgers / scoring ledgers**, reaction windows, scoring/outcome — *all the policy this wave's games embody and must keep game-local*), and the landed `MSC-8C-001…010` entries with their **8C pilot receipts** *and* the `R1 / R2 / R3` receipt tables. The 8C pilot receipts for River/Vow/Briar are exactly the "already-discharged" baseline you must read off (e.g. `UNI8CMECSCA-017/021/024/025/026`). Every R4 migration adds/updates a register entry here (R4 receipts go under the same `MSC-8C-001…010` entries the way R1/R2/R3's did).
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted governance ADR for the scaffolding lane; defines what may be extracted and the register-first discipline.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2; **every hash-bearing surface flip in R4 follows its per-surface migration protocol** (characterize → parallel surface → classify unchanged/parallel-new/intentional-migration → compatibility window → rollback). River's side-pot/all-in surfaces and any C-09 sampler audit are governed here.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the hidden-information replay/export visibility taxonomy; **uniformly central this wave** — all three games have real private hands and seat-private export surfaces. Private data (hole cards, private hands) must remain absent from every viewer-scoped or public surface; the seat-private no-leak / `seat-private-export-v1` audits prove it.

**Tier 3 — evidence, determinism, hidden-info & multi-seat law**
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden-trace obligations, deterministic replay/hash discipline, **no-leak tests**, benchmarks, CI; the acceptance-evidence backbone of the spec.
- `docs/TRACE-SCHEMA-v1.md` — trace/replay-fixture schema law; load-bearing because C-04/C-05/C-08 audits touch trace bytes (these games carry rich N-seat betting/side-pot, bidding, and trick action-tree / replay-export traces) and any change is an ADR-0009 migration packet, never a silent edit.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the evidence-fixture profile contract (ADR 0009); separates command traces, **setup/domain fixtures**, and **viewer-scoped (public + seat-private) exports** — central this wave because all five C-08 profile classes (`replay-command-v1`, `setup-evidence-v1`, `domain-evidence-v1`, `public-export-v1`, `seat-private-export-v1`) are in play.
- `docs/AI-BOTS.md` — bot policy and **bot-input / bot-explanation no-leak law**; load-bearing because each game ships bot traces and the C-07 audit asserts over bot-input/bot-explanation surfaces (River's no-leak pilot already covered bot-input/bot-explanation across seats); no bot may read disallowed private state.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat/surface obligations and the canonical seat-declaration/viewer rules; **directly load-bearing** — R4 carries the **widest** seat matrices in the corpus (River 3-6, Vow 3-7, Briar fixed-4) with dealer rotation, changing hand size, and bidding order, which the canonical-seat-grammar (C-02), seat-count/variant (C-03), and viewer-scoped-surface (C-07/C-08) audits must respect without moving dealer/bid/partnership policy into shared code.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, the failing-test protocol the spec's work breakdown and "one surface per diff" rollback rule must encode.

**Tier 4 — the named floor remainder + planning artifacts**
- Remaining `docs/**` — `OFFICIAL-GAME-CONTRACT.md`, `UI-INTERACTION.md`, `IP-POLICY.md`, `SOURCES.md`, `WASM-CLIENT-BOUNDARY.md`, `ROADMAP.md`, `archival-workflow.md`, `adr/0001…0003/0005/0006/0007`, `adr/ADR-TEMPLATE.md` — part of the mandatory `docs/**` floor; mostly background for this behavior-free unit (no new game, no UI, no bot-strategy change). Read for completeness; `WASM-CLIENT-BOUNDARY.md` has direct bearing because C-02's legacy seat-alias adapter lives at the `wasm-api` boundary, `ROADMAP.md` fixes the public-scaling sequence and the pre-Gate-18 debt interlock, and `IP-POLICY.md` matters only as a regression guard (these are original games, not licensed content).
- All of `templates/**` — `AGENT-TASK.md` is the packet shape the spec's work breakdown decomposes into (each work item is a candidate AGENT-TASK; profile `scaffold-refactor`); the `GAME-*` and evidence/admission templates (esp. `GAME-EVIDENCE.md`, `GAME-RULE-COVERAGE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`) show the evidence vocabulary the spec's acceptance section should reference. Part of the mandatory floor.
- `specs/README.md` — the living tracker (proves 8C-R4 is the lowest non-`Done` unit and that Gate 18 is hard-gated behind it), the 12-section **Spec format** your deliverable must follow, and the **Workflow** that fixes the determination.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` — **the parent spec; read it closely.** §5 "Required C-11 forward-wave seeds" (8C-R4's exact game set + admission/exit shape: *full seat/profile/no-leak audit; RNG algorithm migration separately versioned; the 8C River/Vow/Briar pilots discharge only their named surfaces*), the §4.3 API defaults for C-01…C-05, the §5 ADR-0009 migration protocol, the pilot-discharge rules, and EC-28/EC-30.
- `archive/specs/8c-r1-public-fixed-seat-scaffolding.md`, `archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md`, and `archive/specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md` — **the three direct structural precedents.** Your deliverable should mirror their anatomy: the §1 "Determination" subsection, the §3.2 primary applicability and verdict matrix (game × helper, each cell `migrate` / `already-discharged-by-8C-pilot` / `not-applicable` / `exception`), the per-helper sub-surface tables, the §5 wave decomposition (admission/characterization → per-helper waves → consolidation/register/status-flip), the §6 row-for-row exit criteria, and the §7 acceptance-evidence command set and golden/fixture/diff policy. **Re-use this structure; change the game set, and — crucially — restore the `already-discharged-by-8C-pilot` column (R1 used it for Race/Draughts, R2 used the "residual audit" framing for High Card Duel; R3 dropped it because no R3 game was a pilot, but R4 needs it for all three games).** **R2's High Card Duel residual-audit framing is the closest single precedent for River Ledger's residual audit** — study how R2 distinguished the pilot's named no-leak discharge from the residual profile/helper audit it still owed. Note R1/R2/R3's discipline of *classifying* adjacent C-05 surfaces rather than silently omitting them, and the `Golden, fixture, and diff policy` (authorized vs unauthorized byte changes). Note also the "Grounded correction to the research brief" mechanism — if your own grounded read contradicts any claim in *this* brief, apply the same correction-in-spec discipline (correct it, cite the evidence, do not reopen the locked unit/game set).
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md`, `archive/specs/gate-15-1-river-ledger-all-in-side-pots.md`, `archive/specs/gate-16-briar-circuit-trick-taking.md`, `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` — the four owning game specs; read them to ground which surfaces each game actually has (River's base vs Gate-15.1 side-pot/all-in surfaces; Briar's fixed-4 trick-taking + moon; Vow's 3-7 variable-N bidding + trick-taking), so your residual-vs-discharged classification is grounded in the real game scope, not assumed.
- `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — the source report for Part C / C-01…C-11; the origin of the C-11 retrofit obligation R4 discharges (and the final one before Gate 18).
- `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md`, `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md`, and `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md` — the **precedent format** for the per-surface characterization R4 requires before any migration (bytes/hashes/seat-spellings/RNG-vectors/visibility classes, plus the accepted-exception and not-applicable tables). R2's report is the model for a residual audit; R1's accepted-exception tables model the WASM-seat and stable-byte exception receipts.

**Code seams to inspect directly (read in the repo; *inspect, not read-fully*, and not pasted here):**
- For **each of the three R4 games** (`games/river_ledger`, `games/briar_circuit`, `games/vow_tide`): `src/effects.rs` (C-01 public **and** seat-private envelope constructors — all three have real private surfaces), `src/ids.rs` (C-02 seat-ID parse/format — note River/Vow already piloted canonical seat helpers; Briar's may be residual; these files also define non-seat IDs — card/trick/contract IDs — which are **not** C-02 surfaces), `src/setup.rs` (C-03 — River's 3-6 validation + ring/index was the 8C pilot; audit Vow's 3-7 range + dealer/bid order and Briar's fixed-4; dealer/bid/contract/partnership policy stays game-local), `src/visibility.rs` + `tests/visibility.rs` (C-07 seat-private no-leak geometry — River's was the 8C pilot across 3-6; audit Vow/Briar residual; projection/redaction policy stays game-local), `src/state.rs` / `src/replay_support.rs` + `tests/golden_traces/*.trace.json` (C-04/C-05 action-tree & stable-byte/hash surfaces — incl. River's all-in/side-pot Gate-15.1 traces; C-08 replay-command / public-export / seat-private-export / setup / domain profiles), `data/fixtures/*` (C-08 setup/domain fixtures — River `*_3p_standard` etc., Vow public + seat-private export fixtures, Briar moon + first-trick-exception). These reveal which helper applies to which surface, and **which were already discharged by the 8C pilot vs which are residual**. Confirm each game's `Cargo.toml` already has the `game-test-support` dev-dependency (it does at `0d01901`) — so C-06 is `already-discharged-by-8C-pilot`, not a fresh adoption.
- Shared landing homes already in place (built on, not rebuilt): `crates/engine-core/src/{action.rs,replay.rs,rng.rs}` (C-01/C-04/C-05/C-09 homes — incl. `next_index_unbiased_v1`), `crates/game-stdlib/src/seat.rs` (C-03 home), `crates/game-test-support/src/{no_leak.rs,profiles.rs}` (C-07/C-08 homes — `profiles.rs` holds the five drivers incl. `PublicExportV1Driver` / `SeatPrivateExportV1Driver` / `DomainEvidenceV1Driver` central to this wave), `crates/wasm-api/src/seats.rs` (C-02 legacy-alias adapter + per-game trace seat emission). Also `crates/game-stdlib/src/` trick-taking helper (already-promoted *behavioral* atlas helper Vow/Briar use — **not** scaffolding; do not re-treat as such).
- Tooling & guards: `tools/replay-check`, `tools/fixture-check` (validator owners; thin profile dispatch only — game behavior must not move into a tool), `scripts/boundary-check.sh` (noun-free kernel + dev-only `game-test-support` reverse-dep gate), `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs` (note: R4 adds no game, so the catalog check is a regression guard, not a closeout surface).

---

## 3. Settled intentions (these make this brief locked — do not re-decide them)

1. **Target locked: Unit 8C-R4**, "C-11 follow-on — N-seat/private/trick support." Confirm-and-document
   the determination with the §1 evidence; **do not** re-open "what's next," and **do not** expand
   into Gate 18 or re-decide ladder order. This is plumbing/residual-audit work, not a new game.
   8C-R4 is the *final* C-11 wave; closing it (with each surface migrated / already-discharged /
   not-applicable / accepted-excepted) clears the last item in the Gate 18 admission interlock —
   but **authoring or starting Gate 18 is out of scope.**
2. **Bounded game set — exactly three, none added, none dropped:** `river_ledger` (residual audit),
   `briar_circuit`, `vow_tide`. (Verify against the parent spec §5 seed and the manifest.)
3. **R4 is the residual / pilot-discharged wave (the defining delta; the mirror of R3).** All three
   games were 8C pilots **and** C-08 fixture pilots, and all three already carry the
   `game-test-support` dev-dependency. Therefore the verdict vocabulary **includes
   `already-discharged-by-8C-pilot`** as a first-class column (central here; deliberately absent in
   R3), and the audit's core job is to **distinguish each pilot's already-named-and-discharged
   surfaces from the residual surfaces it did not name** — the residual is R4's real work. Read the
   8C pilot receipts in the register (`UNI8CMECSCA-017/021/024/025/026` and the MSC-8C-001…010 pilot
   tables) and **do not re-propose an already-discharged surface as missing.**
4. **Scope = residual applicability audit + per-surface migration of helpers C-01…C-09**, with the
   seed's stated emphasis on **the widest seat matrices (3–7), private hands, pass/bid/deal order,
   public/seat-private export, and RNG-sampler divergence**:
   - **C-01** effect-envelope constructors — public **and** `seat_private` for all three (all have
     real private effect surfaces); River's were the 8C pilot, so classify River's as
     already-discharged unless a residual (e.g. side-pot) private effect surface exists; audit
     Vow/Briar.
   - **C-02** canonical `seat_<n>` grammar + WASM import-alias adapter (seat IDs only — *not* the
     games' card/trick/contract IDs). River and Vow piloted canonical seat helpers; audit Briar and
     any residual; legacy import aliases stay import-only at the `wasm-api` boundary.
   - **C-03** `SeatCount`/range validation **plus the variant/dealer/bid/contract diversity** these
     games carry (River 3-6 + ring/index was the 8C pilot; Vow 3-7 with dealer rotation + changing
     hand size + bidding/last-bidder order; Briar fixed-4). The helper covers behavior-free
     count/range/ring structure only; dealer/bid/contract/partnership assignment policy stays
     game-local.
   - **C-04** action-tree encoding/hash v1; **C-05** `StableBytesWriter` v1 (action-tree surface),
     across these games' richer action trees (multi-street betting incl. **all-in/side-pot** from
     Gate 15.1, bidding/contract selection, trick play). v1 hashes are added as **parallel** surfaces;
     legacy hash authority stays unchanged unless a named ADR-0009 authority flip is justified.
   - **C-06** dev-only `game-test-support` dependency — **already-discharged-by-8C-pilot** for all
     three (each already has the dev-dep); the C-06 obligation is a *checkpoint* (prove the
     production/build reverse-dep edge stays absent), not a fresh adoption.
   - **C-07** pairwise no-leak assertion geometry — **applicable for all three** (real per-seat
     private hands). River's was the 8C pilot across counts 3-6 (already-discharged for the named
     surfaces); audit Vow/Briar and any River residual (e.g. side-pot/all-in no-leak surfaces).
   - **C-08** evidence-profile drivers — `replay-command-v1`, `setup-evidence-v1`,
     `public-export-v1`, **`seat-private-export-v1`** (applicable for all three — real private
     hands), and `domain-evidence-v1` where a game ships domain fixtures (Briar's moon/first-trick
     fixtures were the 8C pilot). Classify River's `setup-evidence-v1`, Vow's `public-export-v1` +
     `seat-private-export-v1`, and Briar's `domain-evidence-v1` as already-discharged where the 8C
     pilot named them; migrate the residual profile surfaces (e.g. River public/seat-private export,
     River domain side-pot evidence, Vow/Briar replay-command/setup) per the real code.
   - **C-09** unbiased bounded-index RNG — River's local sampler was migrated to
     `next_index_unbiased_v1` in the 8C pilot (already-discharged); **audit whether Vow Tide and
     Briar Circuit still hold local unbiased bounded-index samplers** in their deal/shuffle setup
     and migrate **only** on byte-identical consumption proof. Any actual RNG *algorithm* change is a
     **separately versioned** ADR-0009 migration, never a silent re-seed; default to `not applicable`
     with rationale where the game's deal/shuffle is already on the shared sampler or has no local
     bounded-index helper.
   - **C-10** non-promotion affirmation carries forward as the closing audit checkpoint
     (rejected / local-only reaffirmed for every game's behavioral bundle).
5. **Verdict resolution is Session 2's grounded audit, not pre-locked.** Per (game × applicable
   helper) the audit resolves to **exactly one** of: *migrate one surface per reviewable diff*,
   *already-discharged-by-8C-pilot (name the pilot surface/ticket)*, *accepted not-applicable with
   rationale*, or *accepted exception with named owner + compatibility + rollback + next review
   trigger*. **You determine each cell's verdict from the actual code at `0d01901`** — do not guess
   applicability, and do not pre-assume a verdict the code may contradict (in particular, do not
   assume the 8C pilot covered a Gate-15.1 side-pot surface it predates). Use explicit `not
   applicable` rows over silent omissions (`specs/README.md` spec-format rule). Every official surface
   in the three games is assigned a verdict; there is no unowned "remaining cleanup" bucket. (This
   mirrors how R1/R2/R3 built their matrices: the spec fixes the matrix *shape* and the per-surface
   protocol; the executing tasks pin the bytes.)
6. **Do not re-recommend already-shipped 8C / 8C-R1 / 8C-R2 / 8C-R3 work as if missing (the
   sharp-delta rule).** The following landed and must be *built on*, not rebuilt: the C-01…C-10
   infrastructure in `engine-core`/`game-stdlib`; the dev-only `crates/game-test-support` crate (incl.
   the `no_leak` and `profiles` modules with all five drivers); register entries `MSC-8C-001…010`
   **with their 8C-pilot, R1, R2, and R3 receipt tables**; and the three R4 games' **own** 8C-pilot
   discharge (River C-01/C-02/C-03/C-07/C-08-setup/C-09; Vow C-02/C-06/C-08-public+seat-private; Briar
   C-06/C-08-domain). R4 audits the **residual** of these three games; it adds no new helper and
   changes no shipped helper's contract.
7. **Betting/pot/side-pot, trick, bid/contract, deal/shuffle, partnership, and scoring policy stay
   behavioral — this is the central guardrail of the wave.** The Non-Promotion List (register) and
   atlas §10/§10B explicitly keep game-local: `river_ledger` betting/raise/call/fold legality, pot &
   **side-pot construction / allocation / remainder order**, all-in qualification, showdown ranking,
   and contribution accounting; `vow_tide` dealer rotation, bid/contract legality and last-bidder
   constraint, follow-suit, trick lifecycle, and exact-bid scoring; `briar_circuit` follow-suit, trick
   lifecycle, moon/shoot scoring, and pass policy. (Note the **already-promoted behavioral**
   `game-stdlib::trick_taking` atlas helper Vow/Briar use — it is an atlas promotion with explicit
   scope, **not** mechanical scaffolding; R4 must not re-route trick policy through the scaffolding
   lane.) The audit **adopts scaffolding (envelopes, seat IDs, counts/ring, action-tree bytes, no-leak
   *test geometry*, profile *metadata*) without moving any betting/pot/trick/bid/deal/partnership/
   scoring/projection policy into shared code.** A shared helper that decides *legality*, *who
   leads/wins a trick*, *how a pot or side-pot is built/allocated*, *bid/contract validity*, *deal or
   shuffle policy*, *partnership grouping*, or *how an outcome scores* is out of bounds and must be
   `rejected`.
8. **Hard constraints carried into the spec** (from FOUNDATIONS, the boundary doc, ADR 0008/0009/0004,
   the register Non-Promotion List, and the parent + R1/R2/R3 "Not allowed" lists): no behavioral
   promotion through the scaffolding lane; no shared helper that decides legality / setup / dealer /
   bid / contract / trick / pot / side-pot / partnership / scoring / projection / redaction /
   authorization / bot-choice; **no silent byte/hash/seat-ID/visibility/RNG-consumption change** —
   every hash-bearing or visibility-bearing flip is a named ADR-0009 per-surface migration with
   characterization, before/after evidence, compatibility window, and one-surface rollback; **no
   blanket golden regeneration or "update snapshots" sweep**; **no hidden-information leak** (hole
   cards, private hands) into payloads, DOM, storage, logs, effect logs, bot explanations, candidate
   rankings, traces, fixtures, or replay exports, and **no test canary in a committed artifact**;
   `engine-core` stays noun-free; `game-test-support` stays a `[dev-dependencies]`-only edge; one
   surface per reviewable diff; no test deletion/weakening (AGENT-DISCIPLINE failing-test protocol); no
   YAML/DSL/selectors/triggers/formulas in data (domain fixtures stay typed metadata).
9. **Documentation amendments are routine and spec-driven only:** new/updated R4 receipts in
   `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` per migrated/already-discharged/excepted/N-A surface
   (under the existing `MSC-8C-001…010` entries, as R1/R2/R3 did), a register **Unit 8C-R4 closeout
   evidence** block (mirroring the R1/R3 closeout blocks), and the `specs/README.md` status flip
   (8C-R4 → `Done` with evidence; note that this is the row that clears the last Gate 18 admission
   interlock). **No foundation-doc or ADR amendment is expected.** If you find a foundation/ADR change
   is genuinely required (e.g. an N-seat export-taxonomy, side-pot-allocation-evidence, or
   RNG-algorithm-versioning gap in an accepted ADR), do **not** design against doctrine — surface it
   inside the spec as a flagged **FOUNDATIONS §13 ADR-trigger / blocking note** for maintainers, per
   the user's "if amendments to docs/** would be necessary, the spec should indicate it."
10. **Deliverable framing — intermediate spec artifact (not a final drop-in file).** See §7.
11. `assumption:` the unit slug/label defaults to `8c-r4-n-seat-private-trick-scaffolding`; the exact
    label is one-line-correctable downstream at `/reassess-spec` time. (`8C-R4` is the fixed unit ID
    from the tracker.)
12. `assumption:` River Ledger admits 3-6 seats, Vow Tide 3-7 seats (with dealer rotation + changing
    hand size), and Briar Circuit fixed-4 (per the owning game specs at `0d01901`). If a deeper read
    shows a different range or a residual variant predicate, treat C-03 accordingly and note the
    correction in the spec — but do not expand scope.

---

## 4. The task

Author the bounded **implementation spec for Unit 8C-R4** (`new-spec`, subject = behavior-free
mechanical-scaffolding **residual** retrofit/audit, N-seat/private/trick wave — the final C-11
wave). The spec turns the 8C-R4 seed row into a concrete, reviewable plan: it confirms-and-documents
the determination with cited evidence (incl. that this row clears the last Gate 18 admission
interlock); states the objective and the locked three-game scope; lays out the **residual**
applicability audit of helpers C-01…C-09 per game (with a per-surface verdict matrix whose cells you
fill from the real code — **including the `already-discharged-by-8C-pilot` column for the three
games' named 8C-pilot surfaces, the residual River Gate-15.1 all-in/side-pot surfaces, the
uniformly-applicable C-07 seat-private no-leak and C-08 `seat-private-export-v1` surfaces, and the
C-09 RNG-sampler-divergence audit for Vow/Briar**); decomposes the work into bounded candidate
AGENT-TASK items each naming exact files/symbols/affected-hash-and-visibility surfaces/rollback scope
and obeying the ADR-0009 one-surface-per-diff migration protocol; and maps exit criteria and
acceptance evidence (commands, focused tests, register entries) to the seed's admission/exit and to
the parent spec's EC-28/EC-30 sequencing. The spec must keep all betting/pot/side-pot/trick/bid/
deal/partnership/scoring/reveal policy game-local and be aligned with `docs/**`; where a doc
genuinely needs amending, fold that into the spec as a flagged ADR-trigger rather than editing
doctrine.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — in particular, read the
actual current surfaces of the three R4 games to ground every per-surface verdict (do not guess
applicability or pilot-discharge; verify it against `src/*.rs` incl.
`setup.rs`/`effects.rs`/`visibility.rs`/`replay_support.rs`, `data/fixtures/*`,
`tests/golden_traces/*`, and `tests/visibility.rs`, and against the register's 8C-pilot receipts and
the `UNI8CMECSCA-*` tickets). Confirm which surfaces each game's 8C pilot already named and
discharged versus which are residual (especially River's Gate-15.1 all-in/side-pot surfaces, added
after the base 8C pilot), and confirm all three already carry the `game-test-support` dev-dependency
(so C-06 is a checkpoint, not a fresh adoption). Cross-check Vow Tide and Briar Circuit for any local
unbiased bounded-index sampler still unmigrated (the C-09 "RNG-sampler divergence" angle).

Research online only as deeply as it sharpens the deliverable — and calibrate to the target: this is
a behavior-free plumbing retrofit governed by already-accepted ADRs, so external prior art is a
sharpening aid, **not the crux**. Useful angles if you pursue them: characterization-testing
discipline for legacy byte/hash surfaces (Feathers-style "characterization tests"),
canonical/versioned serialization framing, dev-only test-scaffolding patterns, table-driven
domain-fixture/golden-master evidence patterns, and information-flow / non-interference no-leak
testing geometry as background for the uniformly-applicable seat-private C-07 audits. Cite any
external source that shapes a recommendation. Do not let online research expand the locked scope.

---

## 6. Doctrine & constraints (honor; trimmed to what this unit engages)

- `docs/FOUNDATIONS.md` is the constitution — every decision must satisfy its §11 universal
  invariants (determinism, and the hidden-information / no-leak invariants for all three games'
  private hands and seat-private exports) and clear its §12 stop conditions; a genuine divergence
  requires an accepted ADR superseding the affected principle first (never design against it
  silently). If you hit a §13 ADR trigger, flag it in the spec.
- Authority order: foundation docs govern area docs govern specs govern tickets; if a lower layer
  conflicts with a higher one, the lower layer is wrong and must stop, not design around it.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `hand`, `trick`, `pot`,
  `bid`, `partner`, `seat`-policy, etc.; typed mechanic nouns belong in `games/*`, shared helpers in
  `game-stdlib`/`game-test-support` only via the register and only when behavior-free.
- **TypeScript never decides legality** and never normalizes/repairs seat IDs — C-02 canonical
  parse/format authority is Rust only; legacy aliases are import-only at the `wasm-api` boundary.
- **No YAML and no DSL without an accepted ADR.** Profile/fixture/domain data stays typed
  content/parameters/metadata — never selectors, conditions, triggers, or formulas (betting/pot/
  side-pot, trick, bid, and scoring semantics stay in Rust, not in fixtures).
- **Determinism:** replay, hashes, RNG consumption, serialization order, and traces stay
  deterministic; any change is an explicit, characterized, versioned ADR-0009 migration (this
  includes any Vow/Briar C-09 sampler migration — byte-identical consumption proof or a separately
  versioned algorithm migration).
- **No hidden-information leaks** (hole cards, private hands) into payloads, DOM, storage, logs,
  effect logs, bot explanations, candidate rankings, or replay/export exports; test canaries never
  appear in committed artifacts. The uniformly-applicable seat-private no-leak audits prove it for all
  three games.
- **No behavioral promotion** through the scaffolding lane (the parent spec's "Not allowed" list and
  the register Non-Promotion List): betting/raise/call/fold/pot/side-pot policy, deal/shuffle policy,
  reveal timing, trick lifecycle / led-suit / trump / winner-leads, bid/contract policy,
  team/partnership policy, and scoring/outcome stay game-owned. The already-promoted *behavioral*
  `game-stdlib::trick_taking` atlas helper is not re-opened here.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (AGENT-DISCIPLINE §4); a generic assertion never replaces a specific game assertion (existing
  per-game visibility / no-leak / pot/side-pot / scoring / trick tests are preserved, not subsumed).

---

## 7. Deliverable specification

Produce **one downloadable markdown document**: the Unit 8C-R4 implementation spec.

- **Shape: intermediate spec artifact — NOT a ready-to-drop final repo file.** The user saves your
  output and runs the repository's new-spec pipeline (`/reassess-spec` → `/spec-to-tickets`) to
  revalidate it against the codebase and decompose it into `tickets/`. The eventual target path is
  `specs/8c-r4-n-seat-private-trick-scaffolding.md` (label one-line-correctable; saved under `specs/`
  for the reassess step, archived to `archive/specs/` only at closeout). Your deliverable **does not
  land there directly** and must not be presented as already-final or as skipping the
  reassess/decompose step.
- **Structure: follow the `specs/README.md` 12-section spec format** exactly, mirroring the R1/R2/R3
  anatomy — (1) Header [Spec ID `8C-R4`, stage = "Public scaling phase — C-11 follow-on retrofit
  lane", build gate `8C-R4` (the final C-11 wave; precedes Gate 18), status `Planned` on authoring,
  date, owner, authority order], plus a **§1 "Determination"** subsection with the §1
  locked-determination evidence above (incl. that 8C-R4 is the last C-11 wave clearing the Gate 18
  interlock); (2) Objective [sourced from the parent spec §5 seed + ROADMAP framing]; (3) Scope
  [in/out/not-allowed — carry the seed's bounds and the parent + R1/R2/R3 "Not allowed" lists, and the
  §7-settled betting/pot/side-pot/trick/bid/deal/partnership/scoring non-promotion guardrail; out of
  scope: Gate 18 authoring]; (4) Deliverables [concrete artifact tree: the three games' touched
  residual surfaces + register receipts under `MSC-8C-*` + a register Unit 8C-R4 closeout block + a new
  characterization report `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` +
  `specs/README.md` flip]; (5) Work breakdown [bounded candidate AGENT-TASK items (profile
  `scaffold-refactor`) in dependency order: an admission/characterization wave (read off each game's
  8C-pilot discharge vs residual), then per-game/per-helper audit-and-migrate waves (including
  dedicated River Gate-15.1 side-pot/all-in residual surfaces, C-07 seat-private no-leak for all three,
  C-08 `seat-private-export-v1`/`public-export-v1`/`domain-evidence-v1` residual surfaces, and the
  C-09 Vow/Briar sampler audit), then a consolidation + register + status-flip closeout — each item
  names exact files/symbols/affected hash & visibility surfaces/rollback scope, and each migration
  carries the ADR-0009 per-surface protocol R1/R2/R3 used]; (6) Exit criteria [row-for-row, mapped to
  the seed's admission/exit and to parent EC-28/EC-30; include a per-(game × helper) verdict-coverage
  criterion, an explicit seat-private-no-leak-preserved criterion, a behavioral-non-promotion
  criterion, and a criterion that this row's closure clears the last Gate 18 C-11 interlock]; (7)
  Acceptance evidence [the command set — `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, focused crate tests incl.
  `game-test-support`, `cargo test --workspace`, `replay-check`/`fixture-check`/`rule-coverage` for the
  three games, `boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`,
  `check-doc-links.mjs`, `check-catalog-docs.mjs` as a regression guard — plus a golden/fixture/diff
  policy that names the authorized vs unauthorized byte changes (default expectation: parallel-new v1
  surfaces only, no legacy authority flip unless a named ADR-0009 packet justifies it), and register
  before/after]; (8) FOUNDATIONS & boundary alignment [principles engaged + stance — center
  determinism + the behavioral non-promotion of betting/pot/side-pot/trick/bid/scoring policy, plus the
  uniformly-applicable private-hand no-leak invariant]; (9) Forbidden changes; (10) Documentation
  updates required [register receipts + closeout block + this index flip; explicitly mark
  `apps/web/README.md` **not applicable** — no game/web surface]; (11) Sequencing [predecessor 8C-R3
  `Done`; successor Gate 18; admission rule — this is the final C-11 wave, so its closure is the
  Gate-18-admitting event]; (12) Assumptions [one-line-correctable, including the slug and the
  per-game seat-range/variant assumption].
- **Include a per-surface audit matrix** (game × helper C-01…C-09, with each cell a verdict:
  *migrate / already-discharged-by-8C-pilot / not-applicable / exception*) as the spine of the
  Scope/Work-breakdown sections — this is what makes R4 a bounded **residual** audit rather than an
  open sweep. **Restore the `already-discharged-by-8C-pilot` column** R1/R2 used (R3 dropped it; R4
  needs it for all three games). Add the per-helper sub-surface tables R1/R2/R3 used (effect
  public/private constructors; seat parser/output; exact-count/range **plus dealer/bid/contract
  diversity**; action-tree v1 + adjacent state/effect/view/replay/export/diagnostic surfaces incl.
  River side-pot/all-in; C-07 pairwise matrix by source-seat × viewer × surface for all three; C-08
  profile classes including **`seat-private-export-v1`** and `domain-evidence-v1`; C-09 sampler
  audit). Fill the verdicts from the real code at `0d01901`.
- **Carry the `assumption:` lines** from §3 into the spec's §12 so the user can override them.
- Append the **locked / no-questions** posture in the spec preamble: it is an authored plan to be
  reassessed and decomposed, not executed code.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The determination is confirmed-and-documented with the §1 evidence (8C-R3 `Done`; 8C-R4 lowest
  non-`Done` row; atlas §10A empty; parent EC-28/EC-30; Gate 18 hard-gated behind R1…R4 and cleared by
  this row), **not** re-opened, and the scope never drifts to Gate 18 authoring.
- Exactly the three named games appear (`river_ledger`, `briar_circuit`, `vow_tide`); every
  (game × applicable helper) surface carries an explicit verdict; no unowned cleanup bucket; no
  fourth game.
- **The `already-discharged-by-8C-pilot` column is present and used** for the three games' named
  8C-pilot surfaces (River C-01/C-02/C-03/C-07/C-08-setup/C-09; Vow C-02/C-06/C-08-public+seat-private;
  Briar C-06/C-08-domain), citing the `UNI8CMECSCA-*` tickets / MSC-8C pilot receipts; C-06 is a
  checkpoint (dev-dep already present), not a fresh adoption; the verdict vocabulary is migrate /
  already-discharged-by-8C-pilot / not-applicable / exception.
- The residual surfaces are each classified from real code, not auto-defaulted: River's Gate-15.1
  all-in/side-pot action-tree/state/export/no-leak surfaces; C-07 seat-private no-leak applicable for
  all three (private hands); C-08 `seat-private-export-v1` applicable for all three; the C-09 Vow/Briar
  local-sampler audit (migrate only on byte-identical consumption proof, else N/A or separately
  versioned algorithm migration).
- No already-shipped 8C / 8C-R1 / 8C-R2 / 8C-R3 work — including each R4 game's own 8C-pilot discharge
  — is re-proposed as missing; R4 audits the residual and adds/changes no helper contract.
- **No shared helper decides betting/raise/call/fold, pot/side-pot construction/allocation, deal/
  shuffle, reveal timing, trick lifecycle / led-suit / trump / winner-leads, bid/contract, team/
  partnership, or scoring/outcome policy** — all of it stays game-local, per the Non-Promotion List
  and atlas §10/§10B; the behavioral `game-stdlib::trick_taking` atlas helper is not re-routed through
  the scaffolding lane.
- Every hash/byte/seat/visibility/RNG-touching item is framed as a named ADR-0009 per-surface
  migration with characterization + before/after + compatibility window + one-surface rollback; no
  blanket golden regeneration; one surface per diff. No hidden datum (hole card / private hand) or test
  canary appears in any committed payload/trace/fixture/export.
- No new doctrine weakens an upstream foundation doc or silently amends an accepted ADR; any genuinely
  required doc change is surfaced as a flagged §13 ADR-trigger inside the spec, not enacted.
- The deliverable is the single Unit 8C-R4 spec, framed as an intermediate artifact for
  `/reassess-spec` → `/spec-to-tickets`, following the `specs/README.md` 12-section format and the
  R1/R2/R3 anatomy, and marks `apps/web/README.md` not applicable.
- Every external claim that shaped a recommendation is cited.
- The `0d01901` fetch-baseline commit contains every file named in the §2 read-in-full list.
