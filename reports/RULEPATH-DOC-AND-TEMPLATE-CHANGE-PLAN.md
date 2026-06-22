# Rulepath Documentation and Template Change Plan

**Advisory deliverable:** new change-plan document; it does not replace repository law.  
**Target repository:** `joeloverbeck/rulepath`  
**Target commit:** `db0c50b95f84df12b349710033c77db2bf7326b3` (`db0c50b`)  
**Freshness claim:** user-supplied target commit only; this plan does **not** independently verify that the commit is current `main`.  
**Repository-evidence rule:** claims about repository state below use only manifest-listed files acquired by exact-commit URL. External material is used only to pressure-test recommendations.  
**Retrofit assumption adopted:** existing-game migrations are named and roughly scoped, not executed or fully specified here.

---

## 1. Executive summary

Rulepath's behavioral-reuse doctrine is basically right. The **third-use hard gate should remain intact for behavioral mechanics**. The 17 shipped games show that raw repetition is a poor proxy for shared behavior: deterministic shuffle, private hands, staged reveal, public accounting, turn budgets, graph topology, and trick lifecycle recur, yet their legality, reveal timing, diagnostics, effect vocabulary, replay/export policy, and scoring consequences differ materially. The mechanic atlas correctly promoted only narrow, pure cores such as `board_space` and the two `trick_taking` index helpers while leaving policy local.

The doctrine is nevertheless missing a second category. Rulepath now has enough evidence for a distinct **mechanical scaffolding** lane: typed, behavior-free infrastructure that carries no game rule but is repeatedly rebuilt around the generic kernel. Examples include `EffectEnvelope` constructors, canonical seat-ID formatting/parsing, seat-count validation, canonical action-tree/stable-byte framing, replay-test drivers, and pairwise no-leak test matrices. Treating these as “mechanics” forces the atlas to compare unlike things and delays low-risk consolidation; promoting them as if they were game behavior would be equally wrong. The correct move is an accepted ADR that defines this lane, its allowed homes, its proof burden, and its migration rules.

The highest-leverage plan is therefore:

1. **Repair authority and evidence ownership first.** Put `TRACE-SCHEMA-v1.md` into the authority index, make ADR status explicit, and replace duplicated template narratives with one canonical per-game evidence receipt.
2. **Author two doctrine ADRs before code extraction.** ADR 0008 should govern mechanical scaffolding; ADR 0009 should separate replay-command envelopes, internal evidence fixtures, viewer-scoped exports, and hash versions.
3. **Split reuse governance by semantic risk.** Keep “third official use blocks progress” for behavioral mechanics. For policy-free scaffolding, require a comparison at the second exact duplication and a hard decision at the third, permitting second-use promotion only when the API is noun-free or properly game-layer typed, behavior-neutral, deterministic, leak-safe, and migration-complete.
4. **Build the smallest shared surfaces.** Add generic `EffectEnvelope` constructors; canonical seat identifiers and seat-count validation; a versioned stable-byte writer and action-tree encoding; and a test-only `game-test-support` crate. Do **not** promote generic deal, hand, reveal, view-projection, pot, partnership, or shuffle policy.
5. **Make templates conditional and referential.** The current universal “complete everything in full” rule is contradicted by the shipped corpus: foundational smoke games legitimately lack later strategy/release/pressure artifacts, while mature games often collapse large templates into short evidence receipts. Introduce completion profiles, keep explicit not-applicable decisions, and stop copying the same no-leak, IP, benchmark, and strategy declarations across five documents.
6. **Resolve the trace-schema mismatch before Gate 18.** The file called the canonical Trace Schema v1 describes a strict Gate-2 command envelope, while shipped setup fixtures use materially different roots. The repository should name those as separate artifact profiles rather than silently treating all `*.trace.json` files as one schema.

The plan preserves every `FOUNDATIONS.md` §11 invariant and §12 stop condition. It introduces no YAML, no DSL, no TypeScript rule authority, no hidden-information shortcut, and no noun-bearing mechanic in `engine-core`. Any hash, visibility, or authority change is paired with an ADR and an explicit migration gate.
## 2. Method & evidence base

### 2.1 Provenance and evidence lanes

Acquisition used the uploaded manifest only as a path inventory. Each repository path was constructed mechanically from the exact raw base URL and the full target SHA. No clone, GitHub code search, repository snippet, default-branch lookup, branch-name fetch, or repository-scoped connector metadata was used. Foreign repository names inside validly fetched files were treated as content, never as transport contamination.

The analysis kept three lanes separate:

- **Target-repository evidence:** exact-commit files listed in Appendix A.
- **User-supplied controls:** the research brief and manifest, used for scope and inventory rather than as substitutes for repository content.
- **External research:** official or primary sources used to pressure-test reuse, documentation, testing, replay, and ADR practices; never used to assert what exists in Rulepath.

### 2.2 Repository corpus inspected

The complete foundation set, all ADRs, the ADR template, every existing template, the live spec index, the Phase-0 realignment report, and the public scaling-ladder report were read. Filled game documentation was sampled across **all 17 shipped games**. Source modules were inspected deeply for nine representative games: `race_to_n`, `column_four`, `draughts_lite`, `high_card_duel`, `poker_lite`, `plain_tricks`, `briar_circuit`, `river_ledger`, and `vow_tide`. Those samples span tiny perfect-information play, board topology, compound action trees, chance, hidden information, two-seat and N-seat play, trick taking, betting/showdown, and variable seat counts.

The shared-code seams inspected directly were `engine-core`, all of `game-stdlib`, and the seat/action/replay adapters in `wasm-api`. Golden traces were compared across the canonical Gate-2 shape and later setup/export fixtures. The final exact-URL ledger contains **350 manifest-backed repository files**.

### 2.3 How duplication and friction were verified

No promotion recommendation is based on file-name counts alone. For each proposed extraction, the live symbols and byte-shaping behavior were compared:

- identical public/private `EffectEnvelope` construction in multiple `effects.rs` modules;
- repeated `from_index` / `index` / `as_str` / `parse` / cyclic-next seat code across fixed and variable seat models;
- differing local `action_tree_hash` implementations and extensive hand-built stable serialization in `replay_support.rs`;
- repeated viewer/actor/command builders and pairwise leak assertions in visibility tests;
- different shuffle algorithms and reveal semantics, which is why generic shuffle/deal promotion is rejected;
- actual filled template documents, including long admission records and compact release/AI receipts, rather than assuming that template size predicts useful evidence.

### 2.4 Template adoption evidence

The manifest shows the following shipped adoption, which matters because `templates/README.md` currently says every official game completes every relevant template “in full” and rejects a lighter mode:

| Artifact | Games containing it | Interpretation |
|---|---:|---|
| `SOURCES`, `RULES`, `RULE-COVERAGE`, `MECHANICS`, `GAME-IMPLEMENTATION-ADMISSION`, `HOW-TO-PLAY`, `AI`, `UI`, `BENCHMARKS` | 17 / 17 | Stable core set. |
| `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK` | 15 / 17 | Legitimately absent from the two early foundation-smoke games; should be conditional on authored strategy level. |
| `PRIMITIVE-PRESSURE-LEDGER` | 11 / 17 | Correctly event-driven by second/third-use pressure, not universally applicable. |
| `PUBLIC-RELEASE-CHECKLIST` | 15 / 17 | Missing from early smoke games; later filled copies often behave as short sign-off receipts rather than full narratives. |

### 2.5 External sources consulted

| ID | Source | Rulepath-specific lesson used |
|---|---|---|
| EXT-1 | [Diátaxis](https://diataxis.fr/) and [its four-mode primer](https://diataxis.fr/start-here/) | Separate reference law, how-to workflow, explanation, and evidence receipts instead of making every template repeat all four modes. |
| EXT-2 | [The Practical Test Pyramid](https://martinfowler.com/articles/practical-test-pyramid.html) | “Use before reuse” and the Rule of Three support retaining the behavioral gate; the DRY/DAMP balance also supports extracting only repetition that obscures correctness. |
| EXT-3 | [OpenSpiel introduction](https://openspiel.readthedocs.io/en/latest/intro.html) and [core API reference](https://openspiel.readthedocs.io/en/latest/api_reference.html) | A generic game kernel can coexist with game-specific state; observations and information states remain distinct viewer contracts. Rulepath should preserve its stronger typed Rust and export-safety constraints. |
| EXT-4 | [RFC 8785, JSON Canonicalization Scheme](https://www.rfc-editor.org/info/rfc8785/) | Hashing requires an invariant representation. This supports explicit canonicalization/versioning, not an unreviewed switch to JSON hashing. |
| EXT-5 | [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html) | Inherent constructors, validating newtypes, and type-significant arguments support small typed scaffolding APIs such as envelope constructors and seat-count types. |
| EXT-6 | [Proptest state-machine testing](https://proptest-rs.github.io/proptest/proptest/state-machine.html) | Shared test support should generate transitions and compare invariants while leaving the game-specific reference model and legality in each game. |
| EXT-7 | [Michael Nygard, “Documenting Architecture Decisions”](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) | ADR status and supersession must remain explicit; small decisions with visible consequences are preferable to silently editing foundation law. |
| EXT-8 | [boardgame.io Game API](https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/api/Game.md) | Generic setup validation and per-player state projection are useful comparative patterns, but Rulepath should keep legality and projection in Rust and retain ADR 0004’s stronger replay/export taxonomy. |

External sources shape rationale only. They are not substitutes for Rulepath files.
## 3. Implemented-baseline acknowledgment

This plan is a delta on the shipped Phase-0 realignment, not a cold-start redesign. The archived report itself records an earlier commit of record (`e3b1729…`); that string is ordinary content inside the report. The report was fetched from the target repository at `db0c50b`, and this plan uses `db0c50b` for all repository-state claims.

The following Phase-0 work is already present and is **not** re-recommended as missing:

| Shipped baseline | Exact-commit confirmation | Treatment in this plan |
|---|---|---|
| A dedicated N-seat/larger-surface contract | [`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md) | Retain; add conformance IDs and referential evidence, not a replacement contract. |
| N-seat fields in templates | [`templates/GAME-RULES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULES.md), [`templates/GAME-RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULE-COVERAGE.md), [`templates/GAME-UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-UI.md), [`templates/GAME-BENCHMARKS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-BENCHMARKS.md) | Consolidate duplicated declarations; do not remove the obligations. |
| A public scaling phase and Gates 15–23 plus Gate P tail | [`docs/ROADMAP.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ROADMAP.md), [`specs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/specs/README.md) | Add execution interlocks for trace/scaffolding debt; do not redesign the ladder. |
| Accepted ADR 0007 governing the next public phase and Gate P | [`docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md) | No decision change. Preserve its public-first framing. |
| Stronger pairwise no-leak and per-seat outcome obligations | [`docs/FOUNDATIONS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/FOUNDATIONS.md), [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md), [`templates/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/PUBLIC-RELEASE-CHECKLIST.md) | Convert repeated evidence fields into shared conformance evidence; never weaken the checks. |

The new problem exposed by the 17-game corpus is not that Phase 0 failed. It is that the resulting obligations are copied into too many artifacts, while generic mechanical repetition still has no governance lane of its own.
## 4. The change-plan

Each entry is independently ticketable. “Mechanical scaffolding” below means typed infrastructure that carries no game rule, scoring policy, reveal policy, legality policy, or game noun. “Behavioral mechanic” keeps the existing atlas meaning and remains subject to the current third-use hard gate until an accepted ADR says otherwise.

### Part A — Foundation docs

#### Part A coverage map

| File | Disposition |
|---|---|
| `docs/README.md` | A-01 |
| `docs/FOUNDATIONS.md` | A-02 |
| `docs/ARCHITECTURE.md` | A-03 |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | A-04 |
| `docs/OFFICIAL-GAME-CONTRACT.md` | A-05 |
| `docs/MECHANIC-ATLAS.md` | A-06 |
| `docs/AI-BOTS.md` | A-07 |
| `docs/UI-INTERACTION.md` | A-08 |
| `docs/TESTING-REPLAY-BENCHMARKING.md` | A-09 |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | A-10 |
| `docs/TRACE-SCHEMA-v1.md` | A-11 |
| `docs/WASM-CLIENT-BOUNDARY.md` | A-12 |
| `docs/ROADMAP.md` | A-13 |
| `docs/IP-POLICY.md` | A-14 |
| `docs/AGENT-DISCIPLINE.md` | A-15 |
| `docs/SOURCES.md` | A-16 |
| `docs/archival-workflow.md` | A-17 |
| `docs/adr/ADR-TEMPLATE.md` | A-18 |
| ADR 0005 | A-19 |
| ADRs 0001–0004, 0006, 0007 | No decision change. Keep accepted doctrine; add cross-references only where the entries above require them. ADR 0004 remains the binding visibility/export constraint; ADR 0007 remains the scaling-ladder constraint. |
### A-01 — Make the authority index complete and status-aware

**ID:** A-01

**Target file(s):** [`docs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/README.md); cross-references to [`docs/TRACE-SCHEMA-v1.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TRACE-SCHEMA-v1.md) and `docs/adr/*`

**Type:** correct · clarify

**Evidence:** [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md) calls `TRACE-SCHEMA-v1.md` the canonical field authority, but [`docs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/README.md) omits that file from its numbered authority table. [`docs/adr/0005-variance-aware-ci-benchmark-floors.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0005-variance-aware-ci-benchmark-floors.md) is `Proposed`, while the hierarchy says only accepted ADRs supersede foundation law. Shipped setup fixtures such as [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json) and [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json) make the missing trace-authority placement consequential.

**Proposed change:** Add `TRACE-SCHEMA-v1.md` to the ordered authority map at the testing/replay layer, explicitly subordinate to `FOUNDATIONS`, `ARCHITECTURE`, ADR 0004, and the testing doc. Add a compact ADR status table or generated status index. State that Proposed ADRs are informative only and that supersession is effective only after an Accepted ADR names affected sections and the downstream documents are updated.

**Rationale:** Future specs need one unambiguous answer to “which schema governs this artifact?” Explicit status prevents a proposed benchmark rule or future draft ADR from being mistaken for law.

**Doctrine check:** `FOUNDATIONS` §13 already requires accepted ADRs for replay/hash or visibility changes. This edit clarifies hierarchy; it changes no invariant and no accepted decision.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Update doc-link checking to verify that every normative file named by another foundation document appears in the authority map or is explicitly classified as subordinate reference material.

### A-02 — Add a constitutional definition of mechanical scaffolding—only after ADR 0008

**ID:** A-02

**Target file(s):** [`docs/FOUNDATIONS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/FOUNDATIONS.md)

**Type:** add · clarify

**Evidence:** Repeated policy-free shapes exist in [`games/race_to_n/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/effects.rs) `public_effect`, [`games/column_four/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/effects.rs) `public_effect`, and [`games/river_ledger/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/effects.rs) `public_effect`/`private_effect`; seat framing repeats in [`games/race_to_n/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/ids.rs) `RaceSeat`, [`games/plain_tricks/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ids.rs) `PlainTricksSeat`, [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs) `VowTideSeat`, and [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs) `RiverLedgerSeat`. These repetitions carry less semantic risk than the behavioral shapes intentionally kept local in the pressure ledgers.

**Proposed change:** After accepting ADR 0008, add a short definition in §4: mechanical scaffolding is typed, behavior-neutral infrastructure whose output is determined solely by generic contract inputs and whose extraction cannot alter legality, scoring, reveal timing, viewer authorization, action vocabulary, or game-specific effects. Add an invariant that scaffolding promotion follows its own register, requires migration or explicit exceptions, and cannot be used to bypass the mechanic atlas. Keep the existing behavioral third-use wording intact.

**Rationale:** The constitution currently has only two apparent choices—generic kernel or earned behavioral primitive. A named middle category prevents plumbing from being forced through mechanic language while preserving the anti-speculation posture.

**Doctrine check:** This touches §4 and §11 and therefore must follow an accepted ADR. It preserves noun-free `engine-core`, deterministic replay, no-leak rules, and the stop condition against unresolved behavioral third use.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** No game refactor in the constitutional ticket. Later code tickets cite ADR 0008 and migrate only the specific approved seams.

### A-03 — Publish an ownership matrix for kernel ergonomics, game scaffolding, and test scaffolding

**ID:** A-03

**Target file(s):** [`docs/ARCHITECTURE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ARCHITECTURE.md)

**Type:** add · clarify

**Evidence:** [`crates/engine-core/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs) already owns generic `SeatId`, `Actor`, `Viewer`, `VisibilityScope`, and `EffectEnvelope`; [`crates/engine-core/src/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/replay.rs) owns `StableSerialize`, `EffectLog`, and replay contracts. Yet each game rebuilds constructors and replay helpers, while leak matrices repeat in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs) and [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs). [`crates/game-stdlib/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/lib.rs) exposes only the earned `board_space` and `trick_taking` modules.

**Proposed change:** Extend the ownership table with three explicit lanes: (1) `engine-core` contract ergonomics—noun-free constructors, canonical generic identifiers, stable framing; (2) `game-stdlib` typed game-layer scaffolding or earned mechanics; (3) a new `crates/game-test-support` for test-only generic harnesses. State that `wasm-api` owns external adapter compatibility, and that web presentation helpers do not belong in `game-stdlib` merely because many games render them.

**Rationale:** This gives authors a deterministic placement decision before they write a third local copy. It also prevents test helpers, browser adapters, and behavioral mechanics from being conflated.

**Doctrine check:** `engine-core` remains noun-free; game nouns remain game-local or in `game-stdlib` after governance. Test support has no production authority and cannot decide legality or visibility.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Add dependency-direction checks: games may depend on `game-test-support` only as a dev-dependency; production crates must not depend on it.

### A-04 — Replace the single promotion boundary with four explicit reuse lanes

**ID:** A-04

**Target file(s):** [`docs/ENGINE-GAME-DATA-BOUNDARY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ENGINE-GAME-DATA-BOUNDARY.md)

**Type:** split · clarify

**Evidence:** The current §13 routes reuse through `game-stdlib`, but live code shows four different pressures: generic envelopes in [`games/plain_tricks/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/effects.rs); generic seat framing in [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs); earned behavioral helpers in [`crates/game-stdlib/src/trick_taking.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/trick_taking.rs); and typed content in game data. Meanwhile [`games/river_ledger/src/pot.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/pot.rs) and [`games/river_ledger/src/showdown.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/showdown.rs) remain richly game-specific despite superficial repetition with poker-like games.

**Proposed change:** Split the boundary section into: **kernel contracts/ergonomics**, **mechanical scaffolding**, **behavioral mechanics**, and **typed static content**. For each lane, list allowed homes, required evidence, examples, anti-examples, migration obligations, and ADR triggers. Explicitly classify view authorization, reveal policy, betting/pot allocation, partnerships, and scoring as behavior—not scaffolding. Add a rule that the narrowest layer wins: a helper stays game-local unless its semantics are independent of the game.

**Rationale:** The current binary model makes low-risk plumbing wait behind behavioral doctrine and creates pressure to over-promote. Four lanes make ownership legible without permitting speculative abstraction.

**Doctrine check:** No YAML/DSL; static data remains non-behavioral. `engine-core` gains only generic nouns already in its contract vocabulary. Any visibility/hash semantic change still triggers §13 ADR review.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Update the boundary-check script with dependency rules once `game-test-support` exists; do not add lexical bans that would reject ordinary foreign-repository or historical text inside files.

### A-05 — Make the official-game contract point to one canonical evidence receipt

**ID:** A-05

**Target file(s):** [`docs/OFFICIAL-GAME-CONTRACT.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/OFFICIAL-GAME-CONTRACT.md)

**Type:** add · merge · clarify

**Evidence:** The same readiness facts are repeated across the filled [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md), [`games/river_ledger/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULE-COVERAGE.md), [`games/river_ledger/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/UI.md), and [`games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md). River Ledger’s admission record accretes base and side-pot phases, while its release checklist is already functioning as a compact evidence receipt. The manifest also shows strategy/pressure/release artifacts are not uniformly applicable to all 17 games.

**Proposed change:** Define a mandatory `games/<game>/docs/GAME-EVIDENCE.md` as the canonical conformance index: supported seats/variants, rules version, trace profile/version, viewer matrix, benchmark workload IDs, bot level, source/IP receipt, mechanic/scaffolding decisions, and links to named tests/artifacts. Other documents own prose in their domain and link to this receipt instead of repeating status tables. Define completion profiles: `foundation-smoke`, `public-standard`, `hidden-information`, `N-seat`, `authored-Level-2`, and `promotion-candidate`.

**Rationale:** One evidence index makes completion machine-checkable and makes later gate deltas additive instead of growing every document. Profiles preserve explicit applicability without weakening any official-game proof.

**Doctrine check:** The contract remains requirements-first and Rust-authoritative. A profile may mark obligations not applicable only with a reason; it may not waive §11 invariants or §12 stop conditions.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Pilot the receipt on Gate 18 and retrofit River Ledger plus one small game. After validation, migrate the remaining games in bounded batches.

### A-06 — Keep the behavioral hard gate; move scaffolding pressure into a separate register

**ID:** A-06

**Target file(s):** [`docs/MECHANIC-ATLAS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MECHANIC-ATLAS.md)

**Type:** clarify · split

**Evidence:** The atlas’s narrow promotions are validated by [`crates/game-stdlib/src/board_space.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/board_space.rs) and [`crates/game-stdlib/src/trick_taking.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/trick_taking.rs). The per-game ledgers—especially [`games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md), [`games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md), [`games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md), and [`games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md)—explicitly reject broad shuffle/deal/reveal/trick-lifecycle promotion because policy differs. By contrast, envelope constructors and stable framing are not mechanics at all.

**Proposed change:** Retain §§4–8 for behavioral mechanics with the third-use block unchanged. Remove behavior-free plumbing examples from the mechanic table over time and link a new `MECHANICAL-SCAFFOLDING-REGISTER.md` governed by ADR 0008. Require each register entry to classify semantic risk, production versus test-only home, affected hashes, visibility impact, exact duplicate sites, migration set, and rejection rationale. Preserve the existing promotion-debt interlock for both registers.

**Rationale:** The atlas is doing valuable restraint work. Separating categories improves signal without lowering the behavioral bar.

**Doctrine check:** Complies with §11 “game-stdlib changes are earned” and “promotion debt closes before the next gate.” ADR 0008 must explicitly name affected atlas sections.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Reclassify existing atlas rows only when clearly non-behavioral; do not rewrite historical decisions or count old mentions as new use sites.

### A-07 — Give each AI document one owner and one purpose

**ID:** A-07

**Target file(s):** [`docs/AI-BOTS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/AI-BOTS.md)

**Type:** clarify · merge

**Evidence:** Filled artifacts repeat the same strategy, information-access, exclusions, explanations, and tests. Examples include [`games/river_ledger/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/COMPETENT-PLAYER.md), [`games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md), and the very compact [`games/river_ledger/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/AI.md); the same three-way stack exists in [`games/vow_tide/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/COMPETENT-PLAYER.md), [`games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md), and [`games/vow_tide/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/AI.md).

**Proposed change:** `COMPETENT-PLAYER.md` should own human strategy claims and examples. `BOT-STRATEGY-EVIDENCE-PACK.md` should own the exact allowed input view, candidate extraction, deterministic ranking/tie-breaks, explanations, no-leak proofs, and Level-2 evidence. `AI.md` should become a shipped bot registry: levels present, policy IDs/versions, defaults, benchmark IDs, and links. Remove duplicated long-form strategy prose from `AI.md` and duplicated generic exclusions from per-game files by reference to `AI-BOTS.md`.

**Rationale:** Future games can author strategy once and translate it into bot evidence without maintaining three divergent narratives.

**Doctrine check:** Bots still use legal actions and allowed views only; all public Monte Carlo/MCTS/ML/RL exclusions remain. No hidden state is made available by the consolidation.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit the three trick-taking games together so Gate 18 can consume one consistent partnership-aware strategy/evidence pattern.

### A-08 — Replace the presentation-helper count trigger with semantic scaffolding review

**ID:** A-08

**Target file(s):** [`docs/UI-INTERACTION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/UI-INTERACTION.md)

**Type:** correct · clarify

**Evidence:** §10A defers shared presentation helpers until a “third structural divergence” or more than 20 games, but shipped UI contracts already repeat effect pacing, seat framing, and outcome wiring across [`games/river_ledger/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/UI.md), [`games/briar_circuit/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/UI.md), and [`games/vow_tide/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/UI.md). The corresponding Rust effects remain game-specific, as seen in [`games/column_four/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/effects.rs) and [`games/river_ledger/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/effects.rs).

**Proposed change:** Delete the arbitrary “>20 games” trigger. Route behavior-free Rust/WASM presentation scaffolding through the mechanical-scaffolding register, and route browser-only visual composition through an explicit web shared-surface decision record. Require structural identity, accessibility parity, effect-driven behavior, and no rule inference. Keep semantic effect payloads and Rust-generated action/preview metadata game-owned unless a separate earned helper exists.

**Rationale:** Counts do not measure semantic sameness. Earlier review can remove adapter boilerplate, while the decision record prevents a shared component from becoming a covert rules engine.

**Doctrine check:** TypeScript remains presentation-only; semantic effects remain Rust authority. No view projection or visibility policy is moved into the browser.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Evaluate seat-frame and outcome-explanation adapters before Gate 18; do not generalize game board renderers.

### A-09 — Define shared test-support law and explicit fixture profiles

**ID:** A-09

**Target file(s):** [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md)

**Type:** add · clarify

**Evidence:** Visibility tests independently rebuild viewer matrices and leak scans in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs), [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs), [`games/river_ledger/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/visibility.rs), [`games/briar_circuit/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/visibility.rs), and [`games/vow_tide/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/visibility.rs). Trace roots differ between [`games/race_to_n/tests/golden_traces/shortest-normal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/shortest-normal.trace.json), [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json), and [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json).

**Proposed change:** Add a test-support section defining a dev-only harness boundary, required game-supplied closures/canaries, and a prohibition on test helpers implementing rules. Add named fixture profiles: `replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, and `domain-evidence-v1`; each profile must identify its validator and whether it carries hashes. Add a hash migration protocol: version bump, old/new compatibility window, fixture regeneration command, human-readable migration note, and no-leak re-run.

**Rationale:** Reusable harnesses improve proof coverage while explicit profiles stop custom evidence fixtures from masquerading as canonical replay records.

**Doctrine check:** Tests are strengthened, not weakened. ADR 0004 controls export visibility; replay/hash semantic changes require ADR 0009. Property tests remain game-specific where they model rules.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Create `game-test-support` only after this contract and ADR 0009 are accepted; pilot against one perfect-information and one hidden-information game.

### A-10 — Turn the multi-seat contract into reusable conformance IDs, not more copied prose

**ID:** A-10

**Target file(s):** [`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md)

**Type:** clarify · add

**Evidence:** The Phase-0 contract is exercised heavily by [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json), [`games/river_ledger/tests/golden_traces/setup-6p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-6p.trace.json), [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json), and pairwise no-leak traces such as [`games/vow_tide/tests/golden_traces/seat-private-pairwise-no-leak-7p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/seat-private-pairwise-no-leak-7p.trace.json) and [`games/briar_circuit/tests/golden_traces/seat-private-pairwise-no-leak.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/golden_traces/seat-private-pairwise-no-leak.trace.json). The same obligations are restated in coverage, UI, benchmark, admission, and release templates.

**Proposed change:** Assign stable conformance IDs to seat-range validation, canonical ordering, pairwise viewer safety, public observer behavior, turn/pending-set representation, topology/object budgets, per-seat outcomes, and supported-seat benchmark fixtures. Define one evidence matrix format in `GAME-EVIDENCE.md`; downstream docs cite conformance IDs and add game-specific deltas only.

**Rationale:** Stable IDs make N-seat proof machine-checkable and reduce wording drift while preserving every Phase-0 obligation.

**Doctrine check:** No change to ADR 0007 or the pairwise redaction invariant. The contract remains subordinate to `FOUNDATIONS`, architecture, and ADR 0004.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit River Ledger, Briar Circuit, and Vow Tide first; then use the same IDs in the Gate 18 partnership evidence matrix.

### A-11 — Split canonical replay records from game-specific evidence fixtures

**ID:** A-11

**Target file(s):** [`docs/TRACE-SCHEMA-v1.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TRACE-SCHEMA-v1.md)

**Type:** split · correct

**Evidence:** [`docs/TRACE-SCHEMA-v1.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TRACE-SCHEMA-v1.md) labels one strict root as canonical. [`games/race_to_n/tests/golden_traces/shortest-normal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/shortest-normal.trace.json) and [`games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json) substantially follow that envelope. [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json) uses a reduced setup root and an outdated `river-ledger-rules-v1` label relative to live `RULES_VERSION_LABEL` v2; [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json) has no schema version and includes private setup evidence.

**Proposed change:** Narrow Trace Schema v1 to the command/replay contract it actually defines, or supersede it with a versioned `REPLAY-RECORD-SCHEMA-v2.md`. Add a separate `EVIDENCE-FIXTURE-CONTRACT.md` for setup/domain fixtures, with profile IDs, validator ownership, visibility classification, version anchors, and allowed private test-only data. State that filename suffix does not determine schema. Add a migration inventory for every nonconforming fixture and fix stale rules-version anchors.

**Rationale:** The current label creates false confidence: strict tooling cannot validate heterogeneous roots against one contract. Separate profiles preserve useful bespoke fixtures without weakening replay interoperability.

**Doctrine check:** Changing schema/hash semantics requires ADR 0009. Private evidence remains test-only and must never become a public export. ADR 0004 governs viewer-scoped export profiles.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Migrate or explicitly classify River Ledger and Vow Tide setup fixtures; add validators before authoring Gate 18 traces.

### A-12 — Specify one canonical external seat grammar and compatibility policy

**ID:** A-12

**Target file(s):** [`docs/WASM-CLIENT-BOUNDARY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/WASM-CLIENT-BOUNDARY.md); adapter in [`crates/wasm-api/src/seats.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/seats.rs)

**Type:** add · clarify

**Evidence:** Internal game IDs predominantly use `seat_0`, as in [`games/plain_tricks/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ids.rs) and [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs); early replay helpers use `seat-0`, as in [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs). [`crates/wasm-api/src/seats.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/seats.rs) contains per-game parsing/trace-label branches and compatibility aliases.

**Proposed change:** Declare the public canonical grammar (`seat_<zero-based index>` is the least disruptive choice), maximum accepted numeric form, rejection rules for leading zeros and out-of-range indices, and a time-bounded alias policy for legacy hyphenated IDs. Require Rust-owned parse/format helpers and round-trip tests across setup, commands, views, effects, traces, replay import/export, and UI labels. Never silently rewrite an unknown identifier.

**Rationale:** Gate 18 adds partnerships and more seats; inconsistent string framing becomes a replay and viewer-routing hazard. One grammar removes per-game adapter branches without inventing team semantics.

**Doctrine check:** `SeatId` is already generic kernel vocabulary. Versioned alias handling preserves determinism and avoids visibility misrouting. Partnerships remain separate game-level roles.

**Priority:** High

**Benefiting gates:** 18

**Follow-on:** Migrate `race_to_n` and `column_four` replay helpers first; retain legacy import aliases only for fixtures explicitly covered by compatibility tests.

### A-13 — Add trace/scaffolding debt interlocks to the existing roadmap

**ID:** A-13

**Target file(s):** [`docs/ROADMAP.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ROADMAP.md) and [`specs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/specs/README.md)

**Type:** clarify · add

**Evidence:** [`specs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/specs/README.md) marks Gates 0–17 done and Gates 18–23 plus P as unwritten seeds. [`docs/MECHANIC-ATLAS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MECHANIC-ATLAS.md) already arms Gate 18 partnership/trick-taking interlocks. The trace mismatch and repeated seat/replay scaffolding are visible in the shipped Gate 15–17 files, including [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs) and [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs).

**Proposed change:** Keep the ladder unchanged. Add pre-Gate-18 exit criteria: ADR 0008 and ADR 0009 resolved; trace profiles named; canonical seat grammar decided; any approved scaffold promotion debt closed or explicitly excepted. For every later gate, require both mechanic-atlas debt and mechanical-scaffolding debt to be reviewed before the next mechanic-ladder row begins.

**Rationale:** The repository already uses interlocks successfully. Naming the new debt classes prevents foundational cleanup from being postponed until partnership/reaction complexity compounds it.

**Doctrine check:** Preserves ADR 0007 and the existing roadmap order. It does not let infrastructure work displace public playable games; it is a bounded prerequisite for them.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Express each interlock as a spec-index checkbox with links to accepted ADRs/register entries, not as a vague “cleanup” gate.

### A-14 — Keep IP law; centralize its evidence receipt

**ID:** A-14

**Target file(s):** [`docs/IP-POLICY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/IP-POLICY.md)

**Type:** clarify

**Evidence:** Every game has a source document, for example [`games/high_card_duel/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/SOURCES.md), [`games/river_ledger/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/SOURCES.md), and [`games/vow_tide/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/SOURCES.md); source/IP status is then repeated in admission and release templates. The same public/private content questions appear in [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md) and [`games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md).

**Proposed change:** Do not change the substantive policy. Add one sentence naming `GAME-SOURCES.md` as the prose/source authority and `GAME-EVIDENCE.md` as the release-status receipt. Require stable source IDs so rule rows, generated assets, and human/legal review triggers can reference a source without copying its bibliographic text.

**Rationale:** This reduces divergence while keeping source provenance and release blocking visible.

**Doctrine check:** All public/private, licensing, prose, asset, font, and bundle constraints remain intact. No external source becomes behavior authority.

**Priority:** Low

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit source IDs opportunistically when a game’s rules or assets change; no mass rewrite is required before Gate 18.

### A-15 — Add a bounded protocol for scaffolding and hash-sensitive refactors

**ID:** A-15

**Target file(s):** [`docs/AGENT-DISCIPLINE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/AGENT-DISCIPLINE.md)

**Type:** add · clarify

**Evidence:** A superficially small extraction can alter evidence: [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs) hash only root action segments, while [`games/draughts_lite/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/replay_support.rs) uses a recursive shape. [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs) also hand-builds strict JSON. Visibility suites such as [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs) protect multiple leak surfaces.

**Proposed change:** Add a “shared-scaffolding task” protocol: validate whether failing tests are still valid; classify SUT versus test failure; inventory exact adopters and hashes; freeze public behavior; add characterization tests; implement the narrow helper; migrate one reference game; compare traces/hashes/no-leak matrices; then migrate remaining matching games or record an accepted exception. Ban “update all goldens” as an acceptance criterion.

**Rationale:** The repository’s agent law is strongest when tasks are bounded. Shared refactors need a specific protocol because broad green-test goals can conceal changed serialization or visibility semantics.

**Doctrine check:** Directly reinforces §11’s bounded-output, determinism, test, and no-leak invariants and the §12 stop condition against unbounded “generalize the engine” work.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Add an `AGENT-TASK` scaffold-refactor profile after the foundation wording lands.

### A-16 — Add the external prior art and explicit Rulepath lessons

**ID:** A-16

**Target file(s):** [`docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/SOURCES.md)

**Type:** add

**Evidence:** The shipped games expose four concrete pressures: duplicated documentation modes in [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md); repeated test matrices in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs); stable-hash divergence in [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/draughts_lite/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/replay_support.rs); and viewer-specific projection obligations in [`games/vow_tide/src/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/visibility.rs).

**Proposed change:** Add the eight external sources listed in §2.5 with short Rulepath-specific lessons and explicit non-adoptions: Diátaxis informs document purpose, not authority; OpenSpiel informs kernel/game separation, not AI scope; RFC 8785 motivates canonicalization, not necessarily JSON; boardgame.io is comparative only and does not justify TypeScript authority; Proptest supports harness design, not generic rule models.

**Rationale:** The bibliography should record why a source mattered and where Rulepath deliberately differs, preventing cargo-cult adoption.

**Doctrine check:** No external architecture overrides foundation law. All research remains advisory unless incorporated through the authority chain.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Add source IDs so future ADRs can cite these lessons without duplicating summaries.

### A-17 — Make archival closeout record the authority state it leaves behind

**ID:** A-17

**Target file(s):** [`docs/archival-workflow.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/archival-workflow.md)

**Type:** add · clarify

**Evidence:** The archived Phase-0 report [`archive/reports/foundation-doc-realignment.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/archive/reports/foundation-doc-realignment.md) names a different historical commit while the shipped target tree now contains its results. Long-lived admission records such as [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md) also accumulate multiple phases. Without a closeout receipt, later readers must reconstruct which recommendations actually shipped.

**Proposed change:** Require every archived spec/report series to leave a closeout block: implementation commit, accepted/rejected recommendations, affected foundation sections, ADR status/supersession, open mechanic/scaffolding debt, trace/hash migrations, catalog/doc checks, and links from the live index. State that historical commit strings remain content and are not current-repository provenance.

**Rationale:** A closeout receipt turns archives into reliable history instead of a second, ambiguous source of current law.

**Doctrine check:** The live authority chain remains controlling. Archives never supersede foundation docs or accepted ADRs.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Backfill Phase 0 and Gates 15–17 only when those archives are next touched; require the receipt for all new archive moves.

### A-18 — Strengthen the ADR template around affected sections and migration evidence

**ID:** A-18

**Target file(s):** [`docs/adr/ADR-TEMPLATE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/ADR-TEMPLATE.md)

**Type:** correct · add

**Evidence:** ADR 0004 governs hidden-info replay/export, ADR 0007 governs the public scaling phase, and [`docs/adr/0005-variance-aware-ci-benchmark-floors.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0005-variance-aware-ci-benchmark-floors.md) remains Proposed. The upcoming scaffolding and trace decisions affect live implementations such as [`games/river_ledger/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/replay_support.rs), [`games/vow_tide/src/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/visibility.rs), and multiple golden traces.

**Proposed change:** Make `Status` a single selected value, not a pipe-delimited placeholder. Require: affected foundation sections; superseded decision/section; evidence classification (repository versus external); adopter/migration matrix; compatibility window; trace/hash/view impact; rollback criteria; accepted exceptions; and exact document updates required on acceptance. Add “decision effective only after named foundation updates land” when the ADR changes repository law.

**Rationale:** The template is already thorough, but these fields make supersession and migration mechanically reviewable and prevent a nominally accepted ADR from leaving contradictory docs behind.

**Doctrine check:** Aligns with `docs/README.md` and Nygard-style explicit status/supersession. Does not change any accepted decision.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Use the revised template for ADRs 0008 and 0009; do not rewrite historical ADRs except to add status/supersession links where needed.

### A-19 — Resolve ADR 0005’s long-lived Proposed status

**ID:** A-19

**Target file(s):** [`docs/adr/0005-variance-aware-ci-benchmark-floors.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0005-variance-aware-ci-benchmark-floors.md); references in [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md)

**Type:** ADR · clarify

**Evidence:** [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md) explicitly says ADR 0005 is Proposed and non-binding, while every shipped game has benchmark artifacts such as [`games/river_ledger/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/thresholds.json), [`games/briar_circuit/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/benches/thresholds.json), and [`games/vow_tide/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/benches/thresholds.json). Leaving the status open makes it unclear which calibration rule agents must apply.

**Proposed change:** Run a bounded decision review using the shipped benchmark data: accept ADR 0005 with any necessary wording changes, reject it, or mark it superseded/withdrawn. Update all references to state the outcome and remove any prose that sounds binding if it remains merely proposed.

**Rationale:** A proposed ADR can be useful research, but indefinite limbo is poor operational law—especially when templates ask for exact benchmark thresholds.

**Doctrine check:** No benchmark floor may weaken correctness or no-leak tests. Any accepted change must preserve ADRs 0002 and 0003 or explicitly supersede named parts.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** No game benchmark rewrite until the decision is made; then migrate thresholds through normal per-game benchmark tickets.

### Part B — Templates

#### Part B coverage map

Every existing template receives an explicit recommendation below. B-02 adds one new template; no existing file is silently deleted.
### B-01 — Replace universal full completion with explicit completion profiles

**ID:** B-01

**Target file(s):** [`templates/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/README.md)

**Type:** correct · clarify

**Evidence:** The current “Universal completion rule” rejects a lighter mode, yet the shipped set lacks `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK`, and `PUBLIC-RELEASE-CHECKLIST` for [`games/race_to_n/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/AI.md)’s game and [`games/three_marks/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/three_marks/docs/AI.md)’s game, and only 11 games have a pressure ledger. Later games such as [`games/river_ledger/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/AI.md) legitimately fill some large templates as compact registries.

**Proposed change:** Define a core profile plus conditional overlays: foundation-smoke, public-standard, hidden-information, N-seat, authored-Level-2, and primitive/scaffolding pressure. Every applicable artifact remains required; every non-applicable overlay must be explicitly recorded in `GAME-EVIDENCE.md`. Add lifecycle stages—research, admission, implementation, evidence closeout, release—and say which templates are authoritative versus receipts.

**Rationale:** Profiles describe the actual shipped lifecycle and prevent authors from manufacturing boilerplate merely to satisfy a universal form.

**Doctrine check:** No obligation disappears. Profile selection cannot waive foundation invariants, and explicit not-applicable reasons remain mandatory.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Use Gate 18 as the first profile-driven implementation; validate with one smoke-game retrofit before repository-wide migration.

### B-02 — Add `GAME-EVIDENCE.md` as the canonical conformance index

**ID:** B-02

**Target file(s):** `templates/GAME-EVIDENCE.md` (new)

**Type:** add

**Evidence:** Evidence is scattered across [`games/river_ledger/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULE-COVERAGE.md), [`games/river_ledger/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/UI.md), [`games/river_ledger/docs/BENCHMARKS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BENCHMARKS.md), [`games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md), and equivalent Vow Tide/Briar files. The compact River Ledger release document already demonstrates the value of link-first evidence.

**Proposed change:** Create a short, machine-friendly Markdown template with stable fields: game/rules/data/trace versions; completion profile; supported seats/variants; source/IP receipt; rule-coverage summary; named trace profiles; public and per-seat viewer matrix; replay/hash compatibility; benchmark workload IDs; bot levels/policy IDs; mechanic and scaffolding register decisions; release state; blockers; and exact artifact links. The file must contain status and links, not duplicate domain prose.

**Rationale:** One receipt gives agents, reviewers, and CI a definitive map of proof while preserving specialized documents as the owners of detail.

**Doctrine check:** The receipt is not behavior authority. Rust, rules docs, foundation law, and accepted ADRs remain upstream. Unknown or missing required fields should fail the catalog-doc check.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Add checker support and pilot on Gate 18, River Ledger, and Race to N.

### B-03 — Turn `GAME-SOURCES` into the sole source/IP narrative authority

**ID:** B-03

**Target file(s):** [`templates/GAME-SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-SOURCES.md)

**Type:** clarify · remove

**Evidence:** Source, variant, naming, asset, font, private-content, and release-blocking declarations are repeated in [`games/river_ledger/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/SOURCES.md), its admission record, and its release checklist; similar duplication exists in [`games/briar_circuit/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/SOURCES.md) and [`games/vow_tide/docs/SOURCES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/SOURCES.md).

**Proposed change:** Keep the detailed consulted-source, ambiguity, naming, asset, font, and legal-review sections here. Add stable source IDs and a “verification only; copied content: none/explicitly licensed” field. Remove the final generic release checklist in favor of a link from `GAME-EVIDENCE.md`; other templates should reference source IDs and blockers rather than restating provenance.

**Rationale:** This preserves careful IP research while eliminating three copies of the same status.

**Doctrine check:** Complies fully with `IP-POLICY.md`; no copied prose or asset becomes acceptable through the new field.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit source IDs when rules are next revised; leave historical prose intact until then.

### B-04 — Keep `GAME-RULES` authoritative and remove non-authoritative strategy duplication

**ID:** B-04

**Target file(s):** [`templates/GAME-RULES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULES.md)

**Type:** clarify · remove · add

**Evidence:** The template includes “Bot-relevant non-authoritative strategy notes,” while authored strategy already lives in [`games/vow_tide/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/COMPETENT-PLAYER.md) and bot translation in [`games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md). Seat, visibility, and replay semantics in [`games/river_ledger/docs/RULES.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULES.md) are more important cross-system contracts.

**Proposed change:** Remove the strategy-notes section or reduce it to links to rule IDs that have strategic consequences. Add explicit fields for canonical seat-ID grammar, supported seat counts, viewer classes, simultaneous/reaction ownership, random-event sequence, reveal timing, replay/trace version, and terminal disclosure policy. Keep rule IDs and formal game prose as the authority.

**Rationale:** Rules become more useful to implementers and less likely to drift from strategy docs.

**Doctrine check:** Rust still implements the rules. The template cannot authorize a viewer or reveal beyond ADR 0004 and the game’s typed projection.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Apply first to Gate 18; backfill only games whose rules docs are changed for other reasons.

### B-05 — Make coverage rows point to stable evidence IDs and fixture profiles

**ID:** B-05

**Target file(s):** [`templates/GAME-RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULE-COVERAGE.md)

**Type:** clarify · add · remove

**Evidence:** Coverage documents such as [`games/river_ledger/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULE-COVERAGE.md), [`games/briar_circuit/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/RULE-COVERAGE.md), and [`games/vow_tide/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/RULE-COVERAGE.md) repeat trace catalogs, no-leak matrices, terminal variants, simulations, and benchmark links that also appear in release/UI/benchmark files. Later setup fixtures do not all use the same schema root.

**Proposed change:** Add stable evidence IDs (`RULE-*`, `TRACE-*`, `LEAK-*`, `BENCH-*`) and a required fixture-profile column. Keep the rule-ID-to-proof matrix authoritative. Replace copied pairwise matrices and benchmark prose with links to `GAME-EVIDENCE.md` rows. Require every “not applicable” row to name the rule or surface and reason.

**Rationale:** Stable IDs make coverage machine-checkable and allow other documents to reference proof without copying it.

**Doctrine check:** Coverage cannot be downgraded by linking; the linked artifact must exist and match the declared profile/version.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Update `tools/rule-coverage` and catalog checks to validate evidence IDs and links.

### B-06 — Classify every repeated shape before asking for promotion

**ID:** B-06

**Target file(s):** [`templates/GAME-MECHANICS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-MECHANICS.md)

**Type:** add · clarify

**Evidence:** The current template treats repeated shapes as mechanic pressure. Yet [`games/race_to_n/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/effects.rs) and [`games/river_ledger/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/effects.rs) share envelope framing without sharing gameplay, while [`games/plain_tricks/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/rules.rs), [`games/briar_circuit/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/rules.rs), and [`games/vow_tide/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/rules.rs) share only narrow trick-taking cores now promoted in `game-stdlib`.

**Proposed change:** Add a mandatory classification column: behavioral mechanic, production mechanical scaffolding, test scaffolding, presentation scaffolding, or superficial similarity. Record the governing register/ADR and whether the shape changes legality, scoring, visibility, effects, replay bytes, or game vocabulary. Keep the existing second/third-use warnings only for behavioral mechanics; link scaffolding decisions to the new register.

**Rationale:** Authors will stop filing plumbing as a mechanic and stop using repetition counts as proof of semantic identity.

**Doctrine check:** The classification cannot itself authorize reuse. Behavioral promotions still require the atlas; scaffolding requires ADR 0008 governance.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Reclassify only active pressure rows; historical game mechanics inventories need not be rewritten wholesale.

### B-07 — Slim implementation admission into a pre-build decision and delta record

**ID:** B-07

**Target file(s):** [`templates/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-IMPLEMENTATION-ADMISSION.md)

**Type:** split · remove · clarify

**Evidence:** [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md) has grown into a combined Gate 15 and Gate 15.1 narrative with extensive post-implementation evidence, while [`games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md) repeats many template checks already proven elsewhere.

**Proposed change:** Limit the admission document to pre-implementation facts: source/rule readiness, supported scope, novel mechanics, pressure decisions, boundary risks, required bot/UI/benchmark profile, blockers, and an admit/defer/reject decision. Add a short “delta admission” section for a later expansion. Move post-build proof to `GAME-EVIDENCE.md` and release sign-off.

**Rationale:** Admission should answer “may work begin?” rather than become a second implementation report. Delta sections prevent expansions from accreting another full copy.

**Doctrine check:** The requirements-first workflow remains. No game is admitted with unresolved third-use pressure, hidden-info risk, or scope ambiguity.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Do not rewrite old admissions immediately; use the new form for Gate 18 and any new expansion spec.

### B-08 — Make agent tasks reference law and evidence instead of copying it

**ID:** B-08

**Target file(s):** [`templates/AGENT-TASK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/AGENT-TASK.md)

**Type:** remove · add · clarify

**Evidence:** The template repeats long Rust-authority, static-data, hidden-info, IP, test, documentation, and final-checklist tables. Game tickets then risk copying those declarations while actual proof lives in files such as [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs) and [`games/river_ledger/docs/RULE-COVERAGE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULE-COVERAGE.md).

**Proposed change:** Keep task-specific scope, target, non-goals, forbidden changes, acceptance evidence, failing-test protocol, and output format. Replace generic law tables with exact foundation/ADR links plus a “task-specific deltas and risks” table. Add fields for scaffold-register entry, hash/trace compatibility class, affected evidence IDs, and required characterization tests.

**Rationale:** Shorter tasks are easier to review and less likely to drift from upstream law, while still requiring concrete proof.

**Doctrine check:** The task cannot override foundation docs. Failing tests must still be validated before changing the SUT or suite, and no test may be weakened to make a refactor pass.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Update ticket-generation skills to populate references and deltas, not paste whole policy sections.

### B-09 — Keep `GAME-HOW-TO-PLAY` player-facing and deliberately non-exhaustive

**ID:** B-09

**Target file(s):** [`templates/GAME-HOW-TO-PLAY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-HOW-TO-PLAY.md)

**Type:** clarify

**Evidence:** Player-facing documents such as [`games/race_to_n/docs/HOW-TO-PLAY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/HOW-TO-PLAY.md), [`games/river_ledger/docs/HOW-TO-PLAY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/HOW-TO-PLAY.md), and [`games/vow_tide/docs/HOW-TO-PLAY.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/HOW-TO-PLAY.md) serve a different audience from formal `RULES.md`. Their useful brevity contrasts with the evidence-heavy engineering templates.

**Proposed change:** Keep the template separate. Add a required link to formal rules and a single version/source receipt; remove any maintainer checklist that belongs in `GAME-EVIDENCE.md`. Add a small “what the interface will show” field for hidden-info/reaction games, but forbid implementation or bot strategy detail.

**Rationale:** The player document should remain a how-to guide, not become another reference contract—consistent with the actual successful filled copies and Diátaxis separation.

**Doctrine check:** It is never behavior authority and cannot promise an action or reveal that Rust does not expose.

**Priority:** Low

**Benefiting gates:** 18–23+

**Follow-on:** No mass retrofit. Apply when player rules are next edited or generated for a new game.

### B-10 — Make `COMPETENT-PLAYER` conditional and authoritative for strategy claims

**ID:** B-10

**Target file(s):** [`templates/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/COMPETENT-PLAYER.md)

**Type:** clarify · remove

**Evidence:** The document is absent for the two earliest smoke games but present for authored-bot games. In [`games/river_ledger/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/COMPETENT-PLAYER.md) and [`games/vow_tide/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/COMPETENT-PLAYER.md), strategy claims overlap with the bot pack’s phase, threat, and information-boundary sections.

**Proposed change:** Require this template only when a game needs a Level-2 strategy evidence base or public strategy guidance. Keep human tactics, threats, mistakes, visible inference, examples, anti-examples, and Rule-ID links. Remove candidate-vector, exact bot-input, deterministic tie-break, and test-plan detail; those belong in the bot pack.

**Rationale:** The split makes human strategy readable and makes translation into a bot auditable rather than duplicated.

**Doctrine check:** Human strategy may discuss only legal observable information. It cannot authorize peeking, sampling actual hidden state, or excluded AI techniques.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit Plain Tricks, Briar Circuit, and Vow Tide as one strategy-doc family before Gate 18.

### B-11 — Make the bot evidence pack the sole Level-2 implementation proof

**ID:** B-11

**Target file(s):** [`templates/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/BOT-STRATEGY-EVIDENCE-PACK.md)

**Type:** clarify · remove · add

**Evidence:** The template repeats strategy content already present in `COMPETENT-PLAYER`, while filled packs such as [`games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md) and [`games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md) are most valuable where they specify exact input views, candidate order, no-leak boundaries, explanations, and tests.

**Proposed change:** Require a source link to `COMPETENT-PLAYER` and record only the translation: input schema/viewer, legal-action API, candidate extraction, lexicographic priorities, bounded scores, deterministic tie-break, explanation redaction, known failure modes, tests, and benchmark IDs. Replace generic v1/v2 exclusions with a link to `AI-BOTS.md` plus a per-game compliance statement.

**Rationale:** This turns the pack into a reproducible implementation contract rather than a second strategy essay.

**Doctrine check:** Retains all hidden-information and bot-level restrictions. Dev rankings remain dev-only and viewer-safe.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Use the revised pack for Gate 18 partnership targeting and teammate-information limits.

### B-12 — Collapse `GAME-AI` into a shipped bot registry

**ID:** B-12

**Target file(s):** [`templates/GAME-AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-AI.md)

**Type:** remove · merge · clarify

**Evidence:** The 203-line template asks for full detail at Levels 0–3, information access, decision order, explanations, tests, benchmarks, and simulations. Actual files such as [`games/river_ledger/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/AI.md) and [`games/vow_tide/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/AI.md) are compact because the substantive evidence already lives elsewhere.

**Proposed change:** Replace the long template with a registry table: implemented levels, policy IDs/versions, public default, allowed viewer, deterministic seed/tie-break source, strategy-pack link, tests, benchmark workload ID, explanation surface, and known limitations. Keep explicit `not implemented` rows for Levels 2/3 where relevant.

**Rationale:** The template will match actual use and stop inviting divergence from bot evidence packs.

**Doctrine check:** No bot capability or information access is expanded. The registry reports shipped behavior; it does not define it.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit all games mechanically after the template lands; this is a documentation-only batch with no bot code change.

### B-13 — Make `GAME-UI` a game-specific delta contract

**ID:** B-13

**Target file(s):** [`templates/GAME-UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-UI.md)

**Type:** split · remove · clarify

**Evidence:** The 350-line template repeats global accessibility, responsive, legal-only, hidden-info, replay, and outcome requirements. Filled UI documents such as [`games/column_four/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/UI.md), [`games/river_ledger/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/UI.md), and [`games/vow_tide/docs/UI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/UI.md) mainly need to describe their board/table layout, action mapping, effect mappings, and unique reveal/outcome surfaces.

**Proposed change:** Keep game-specific product target, renderer/surface budget, seat layout, action-path mapping, progressive construction, previews, effect-to-animation mapping, outcome/showdown variants, game-specific accessibility labels, and smoke tests. Replace global doctrine checklists with conformance IDs and evidence links. Mark sections conditional by profile instead of requiring pages of `not applicable` rows.

**Rationale:** Authors can focus on the UI decisions that differ while shared safety remains centrally enforceable.

**Doctrine check:** TypeScript remains presentation-only; all legal actions, previews, visibility, effects, and outcomes remain Rust-owned.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Pilot on Gate 18 and compare document size/readability with River Ledger before migrating other games.

### B-14 — Give benchmarks stable workload identities and compatibility anchors

**ID:** B-14

**Target file(s):** [`templates/GAME-BENCHMARKS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-BENCHMARKS.md)

**Type:** add · clarify

**Evidence:** Games ship per-crate benchmarks and threshold files, for example [`games/river_ledger/benches/river_ledger.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/river_ledger.rs) / [`games/river_ledger/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/thresholds.json) and the analogous Briar/Vow files. Trace/hash or seat-count changes can silently change what a benchmark measures.

**Proposed change:** Add required workload IDs and versions, fixture/profile ID, seat count, topology/object count, rule/data version, hash/trace compatibility note, sample-count policy, and threshold source. Reference ADR 0005’s final status. Move generic doctrine and release blockers to foundation/evidence receipts.

**Rationale:** Stable identities make before/after comparisons meaningful and let shared-scaffolding refactors prove they did not alter workloads.

**Doctrine check:** Performance gates never replace correctness, replay, or no-leak proofs. Native-first doctrine and accepted benchmark ADRs remain controlling.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Assign workload IDs when benchmarks are next recalibrated; require them immediately for Gate 18 onward.

### B-15 — Keep the primitive-pressure ledger behavioral; link a new scaffold record

**ID:** B-15

**Target file(s):** [`templates/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/PRIMITIVE-PRESSURE-LEDGER.md)

**Type:** clarify · split

**Evidence:** Filled ledgers such as [`games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md), [`games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md), and [`games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md) carefully compare behavior, visibility, effects, and replay. Envelope/seat/test boilerplate does not need the same mechanic-specific API-sketch fields.

**Proposed change:** Rename the template’s scope explicitly to behavioral mechanics. Add a first field that rejects or redirects non-behavioral repetition to `MECHANICAL-SCAFFOLDING-REGISTER`. Keep similarities/differences, behavior semantics, examples, anti-examples, migration, tests, and third-use decision. Remove generic plumbing examples from instructions.

**Rationale:** The ledger remains rigorous where rigor matters and no longer becomes a catch-all duplication report.

**Doctrine check:** The behavioral third-use gate is unchanged. Redirecting a shape does not authorize promotion; it starts the other governed decision process.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Update active Gate 18 pressure entries only; preserve historical ledger text as evidence of prior decisions.

### B-16 — Turn the public release checklist into final sign-off over linked evidence

**ID:** B-16

**Target file(s):** [`templates/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/PUBLIC-RELEASE-CHECKLIST.md)

**Type:** remove · merge · clarify

**Evidence:** The template repeats IP, no-leak, replay, UI, legal-only, accessibility, bot, dev-panel, and test matrices. Filled release files such as [`games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md), [`games/briar_circuit/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/PUBLIC-RELEASE-CHECKLIST.md), and [`games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md) already compress these into pass/fail rows with evidence links.

**Proposed change:** Keep shipment rule, exact build/version, completion profile, blocker list, human review, and final release decision. Replace repeated domain matrices with required evidence-ID references and automated-check results from `GAME-EVIDENCE.md`. Retain a small set of non-delegable human checks: play-first quality, IP/trade-dress review, hidden-info surface spot check, accessibility experience, and public-build dev-panel inspection.

**Rationale:** Release sign-off should verify completed evidence, not restate every requirement. Human judgment remains visible where automation cannot replace it.

**Doctrine check:** No test or proof is removed; missing linked evidence is an automatic failure. ADR 0004 and all release stop conditions remain binding.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Generate initial rows from `GAME-EVIDENCE.md`, but require a human-authored final decision and blocker rationale.

### Part C — Code-reuse moves

The code moves below are recommendations, not implementations. Each is authorized only after the governing documents/ADRs land. They intentionally distinguish generic contract ergonomics, behavior-free game-layer scaffolding, test-only scaffolding, and behavioral mechanics.
### C-01 — Add generic `EffectEnvelope` constructors

**ID:** C-01

**Target file(s):** [`crates/engine-core/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs) and call sites in `games/*/src/effects.rs`

**Type:** new-scaffolding

**Evidence:** Exact struct literals repeat in [`games/race_to_n/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/effects.rs) `public_effect`, [`games/column_four/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/effects.rs) `public_effect`, [`games/plain_tricks/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/effects.rs) `public_effect`/`private_effect`, and [`games/river_ledger/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/effects.rs) `public_effect`/`private_effect`. [`crates/engine-core/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs) already defines generic `EffectEnvelope<T>` and `VisibilityScope` but no constructors.

**Proposed change:** Add inherent generic constructors such as `EffectEnvelope::public(payload)` and `EffectEnvelope::private_to(seat_id, payload)`. Keep fields readable if existing code depends on them, but make constructors the documented path. Add unit tests for scope and payload preservation; do not add game-specific visibility classes.

**Rationale:** This is exact, behavior-free repetition. Constructors reduce error-prone scope literals and align with Rust API guidance without adding game nouns.

**Doctrine check:** Noun-free kernel; no legality, reveal policy, or filtering behavior changes. `EffectLog::since` remains the single generic viewer filter. No hash changes should occur if serialized bytes are unchanged.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Retrofit all matching games in one bounded mechanical ticket; preserve traces byte-for-byte and run every visibility suite.

### C-02 — Introduce canonical seat-ID parse/format helpers with versioned aliases

**ID:** C-02

**Target file(s):** [`crates/engine-core/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs), [`crates/wasm-api/src/seats.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/seats.rs), game `ids.rs`/replay helpers

**Type:** new-scaffolding

**Evidence:** `seat_0` formatting/parsing repeats in [`games/plain_tricks/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ids.rs), [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs), and [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs); `seat-0` is emitted in [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs). [`crates/wasm-api/src/seats.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/seats.rs) compensates with per-game parsing/trace-label logic.

**Proposed change:** Add a generic canonical constructor/parser on `SeatId` or a small `CanonicalSeatId` newtype: zero-based index, underscore form, strict round-trip, bounded numeric parse. Keep legacy hyphen aliases only in the import/adapter layer with explicit version tests. Do not collapse game-local role/team labels into the canonical seat identifier.

**Rationale:** A shared grammar removes duplicated string code and prevents actor/viewer mismatches across setup, replay, and browser APIs.

**Doctrine check:** Seat is already generic engine vocabulary. Alias migration must be versioned because IDs appear in hashes/traces. Viewer authorization compares canonical typed IDs, not untrusted strings.

**Priority:** High

**Benefiting gates:** 18

**Follow-on:** Migrate early hyphenated helpers first; retain golden fixtures through explicit compatibility import tests or an ADR-approved trace migration.

### C-03 — Add typed seat-count validation and ring-index arithmetic; defer enum generation

**ID:** C-03

**Target file(s):** [`crates/engine-core/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs) or a narrowly governed `game-stdlib::seat` module; game `setup.rs` and `ids.rs`

**Type:** new-scaffolding

**Evidence:** Fixed-count validation and seat cycling recur in [`games/race_to_n/src/setup.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/setup.rs), [`games/column_four/src/setup.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/setup.rs), [`games/plain_tricks/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ids.rs) `other`, [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs) `supported_seat_count`/`next_clockwise`, and [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs) `seats_for_count`/`next_in_count`.

**Proposed change:** Provide validating types such as `SeatCount::exact(n)` / `SeatCountRange::new(min,max)` and a generic `next_index(current,count)` that rejects invalid inputs. Do **not** initially add a macro that generates game seat enums; Gate 18 partnerships may reveal role/team distinctions that a macro would obscure. Revisit enum generation only after one partnership game uses the canonical helpers.

**Rationale:** Validation/ring arithmetic is mechanical; game role semantics are not. This extracts the safe core while preserving readable local types.

**Doctrine check:** No game mechanic or team concept enters `engine-core`. Setup diagnostics remain game-owned unless exact diagnostic wording is deliberately standardized and migrated.

**Priority:** High

**Benefiting gates:** 18–21

**Follow-on:** Retrofit River Ledger and Vow Tide first, then one fixed two-seat game. Compare setup diagnostics and traces before broader adoption.

### C-04 — Define a versioned canonical action-tree encoding and hash

**ID:** C-04

**Target file(s):** [`crates/engine-core/src/action.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/action.rs) and [`crates/engine-core/src/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/replay.rs)

**Type:** new-scaffolding · ADR

**Evidence:** [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs) hash only root choice segments, while [`games/draughts_lite/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/replay_support.rs) recursively represents compound action structure. Gate 18 reaction/partnership choices will make shallow hashes less meaningful.

**Proposed change:** After ADR 0009, add `ActionTreeEncodingVersion` and a canonical, length-delimited traversal covering freshness token, ordered choices, segment, disabled state/reason if contractually included, metadata fields selected by the ADR, and child structure. Expose `stable_bytes_vN()` / `stable_hash_vN()`. Do not silently change existing `expected_action_tree_hashes`; support legacy v1 fixtures or migrate them with notes.

**Rationale:** One encoding makes replay comparisons meaningful across simple and compound trees and removes local hash drift.

**Doctrine check:** Replay/hash semantics change, so §13 requires an ADR. Ordering must be deterministic; disabled reasons must be viewer-safe; TypeScript does not participate in hashing.

**Priority:** High

**Benefiting gates:** 18, 22, 23

**Follow-on:** Pilot on Race to N and Draughts Lite to prove flat/compound compatibility, then migrate games by trace profile.

### C-05 — Add a canonical stable-byte writer, not automatic magic hashing

**ID:** C-05

**Target file(s):** [`crates/engine-core/src/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/replay.rs)

**Type:** new-scaffolding · ADR

**Evidence:** `StableSerialize` only returns bytes. Local code therefore concatenates strings or hand-builds JSON, notably [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) `effect_stable_string`/`action_tree_hash` and [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs) `ColumnFourReplayJson`, `escape_json`, and `StrictJsonObject`. Similar stable-summary code exists across representative games.

**Proposed change:** Add a small versioned `StableBytesWriter` with explicit methods for tagged fields, unsigned integers, booleans, byte strings, UTF-8 strings, option tags, and length-delimited sequences. Require callers to choose field order and version. Do not provide blanket reflection/derive hashing in the first iteration. Document that canonical JSON is an optional wire representation, not the hash authority unless an ADR adopts it.

**Rationale:** Explicit framing prevents separator collisions and inconsistent escaping while keeping the byte contract reviewable in Rust.

**Doctrine check:** Hash semantics require ADR 0009 and migration evidence. The writer is noun-free and deterministic; it does not serialize unordered maps without caller-supplied stable ordering.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Characterize current hashes, add writer tests including ambiguous-string cases, then migrate one surface at a time rather than all state/effect/view hashes together.

### C-06 — Create a dev-only `game-test-support` crate

**ID:** C-06

**Target file(s):** `crates/game-test-support` (new) plus workspace/dependency docs

**Type:** new-scaffolding

**Evidence:** Test setup repeats actor/viewer/command builders, deterministic replay loops, leak canaries, and assertions in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs), [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs), [`games/river_ledger/tests/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/replay.rs), [`games/briar_circuit/tests/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/replay.rs), and [`games/vow_tide/tests/replay.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/replay.rs).

**Proposed change:** Create a dev-only crate containing generic harness combinators, not a game framework: viewer-matrix iteration, source-seat × viewer × surface cases, stable canary scanning, replay round-trip orchestration, trace-profile assertion helpers, action-tree path enumeration/dead-branch checks, and supported-seat setup matrices. Every game supplies typed closures for setup, legal actions, projection, export, and canary extraction.

**Rationale:** The harness removes repetitive test plumbing and makes strong proofs cheap enough to apply uniformly to Gates 18–23.

**Doctrine check:** The crate cannot be a production dependency and cannot implement legality, projection, redaction, or game state. Tests must still verify the SUT’s Rust behavior; helpers may not redact failures away.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Pilot with Race to N and High Card Duel; then River Ledger for N-seat/private complexity. Add dependency checks before broad rollout.

### C-07 — Standardize pairwise no-leak assertions without standardizing projection policy

**ID:** C-07

**Target file(s):** `crates/game-test-support` viewer-matrix module; game visibility tests

**Type:** new-scaffolding

**Evidence:** Hidden-information suites independently assert observer and seat safety in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs), [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs), [`games/river_ledger/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/visibility.rs), [`games/briar_circuit/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/visibility.rs), and [`games/vow_tide/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/visibility.rs). Their game-specific private facts differ, but the matrix shape repeats.

**Proposed change:** Provide a harness that takes: supported viewers; named surfaces (view, action tree, preview, diagnostics, effects, export, explanation); game-supplied private canaries per source seat; and an authorization predicate. Assert absence/presence across the full matrix and emit a structured failure showing source seat, viewer, surface, and canary ID. Never generate the projected payload itself.

**Rationale:** This extracts the proof geometry, not the visibility policy. It makes pairwise N-seat testing practical and failures diagnosable.

**Doctrine check:** ADR 0004 remains authoritative. The game supplies authorization; the helper cannot infer when a fact becomes public. Canary values must be safe test data and must not enter public fixtures.

**Priority:** High

**Benefiting gates:** 18, 22, 23

**Follow-on:** Retrofit Vow Tide’s seven-seat matrix and River Ledger’s public/private exports after the two-game pilot.

### C-08 — Standardize replay-fixture drivers by profile

**ID:** C-08

**Target file(s):** `crates/game-test-support` replay module and trace validators

**Type:** new-scaffolding · ADR

**Evidence:** Command replay plumbing repeats in [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs), [`games/column_four/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs), and [`games/river_ledger/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/replay_support.rs); fixture roots differ across [`games/race_to_n/tests/golden_traces/shortest-normal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/shortest-normal.trace.json), [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json), and [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json).

**Proposed change:** After ADR 0009, create profile-specific test drivers: command replay with checkpoints/hashes, setup evidence, public export round-trip, and seat-private export round-trip. The driver validates common metadata and invokes game-supplied setup/command/projection functions. Keep domain-specific expected fields in game-owned typed fixture structs.

**Rationale:** Common orchestration and version checks become reliable without flattening all evidence into one schema.

**Doctrine check:** No driver may execute behavior described by fixture data; fixtures contain inputs/expected evidence only. Unknown fields reject by default, and private profiles never become public exports.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Register existing fixtures by profile before migrating their roots; support read-only legacy adapters during a bounded compatibility window.

### C-09 — Do not centralize shuffle yet; first standardize bounded-index RNG semantics

**ID:** C-09

**Target file(s):** [`crates/engine-core/src/rng.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/rng.rs); local setup/shuffle modules

**Type:** clarify · new-scaffolding

**Evidence:** Local shuffle code is not byte-equivalent: River Ledger, Plain Tricks, and Poker Lite use explicit unbiased rejection-sampling helpers in their setup paths, while Briar Circuit and Vow Tide use kernel index selection patterns. Their deal, tail, trump, private-hand, and reveal policies also differ, as documented in [`games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md) and [`games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md).

**Proposed change:** Keep game-local shuffle/deal order for now. Review `DeterministicRng::next_index` for a single documented unbiased bounded-sampling contract and version it if current behavior differs. Only after byte-for-byte comparison should a noun-free in-place permutation helper be considered. Any adoption must preserve each game’s exact draw/deal sequence or explicitly migrate traces via ADR 0009.

**Rationale:** This addresses the genuinely generic randomness primitive without pretending that deck composition, dealing, or reveal is shared behavior.

**Doctrine check:** Determinism is a §11 invariant. No RNG algorithm or consumption order changes without an explicit replay/hash migration. No card/deck nouns enter the kernel.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Produce a comparison report and characterization vectors; no game retrofit is authorized by this recommendation alone.

### C-10 — Publish an explicit non-promotion list for superficially repeated behavior

**ID:** C-10

**Target file(s):** [`docs/MECHANIC-ATLAS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MECHANIC-ATLAS.md) and the new scaffolding register

**Type:** clarify

**Evidence:** The live ledgers show meaningful divergence: [`games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md) keeps deal/reveal/trick lifecycle local; [`games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md) keeps pots, evaluator, hidden hands, and accounting local; [`games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md) and [`games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) keep graph/control/asymmetry behavior local.

**Proposed change:** Record that the following are **not** mechanical scaffolding: deal order, private-hand ownership/reveal, viewer projection/redaction policy, betting/reopen/all-in logic, pot allocation, trick lifecycle/winner-leads, team/partnership semantics, graph traversal/control, resource accounting, reaction priority, scoring, and outcome rationale. They may still earn narrow behavioral primitives through the atlas, but raw repetition cannot route them through the scaffold lane.

**Rationale:** A non-promotion list protects the new category from becoming a loophole and communicates rejected ideas to future agents.

**Doctrine check:** Directly preserves noun-free kernel, hidden-info safety, the behavioral hard gate, and no-DSL law.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Review the list after each new mechanic gate; additions require evidence, and removals require an atlas/ADR decision.

### C-11 — Retrofit in representative slices, not a 17-game sweep

**ID:** C-11

**Target file(s):** follow-on specs/tickets across selected games

**Type:** clarify

**Evidence:** The corpus contains distinct risk tiers: tiny [`games/race_to_n/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/lib.rs), compound-board [`games/draughts_lite/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/lib.rs), two-seat hidden-info [`games/high_card_duel/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/lib.rs), and N-seat hidden-info [`games/river_ledger/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/lib.rs) / [`games/vow_tide/src/lib.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/lib.rs). A single migration would make regressions hard to localize.

**Proposed change:** Define four retrofit waves: (1) Race to N + Column Four for envelope/seat/stable framing; (2) Draughts Lite for compound action trees; (3) High Card Duel + Poker Lite for hidden-info harnesses; (4) River Ledger + Vow Tide + Briar Circuit for N-seat/private export. Only after each wave preserves behavior and closes exceptions should remaining matching games adopt the helper.

**Rationale:** Representative slices expose flat/compound, public/private, and fixed/variable-seat risks before broad churn.

**Doctrine check:** Follows agent-discipline bounded-task law and the atlas requirement that matching games migrate or receive explicit exceptions. No golden trace changes are accepted merely because a shared helper landed.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Create separate specs per wave with exact hashes, traces, tests, and rollback points.

### Part D — Doctrine changes
### D-01 — Author ADR 0008: Mechanical Scaffolding Governance

**ID:** D-01

**Target file(s):** `docs/adr/0008-mechanical-scaffolding-governance.md` (new) plus named foundation updates

**Type:** ADR

**Evidence:** The 17-game corpus contains exact behavior-free duplication in [`games/race_to_n/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/effects.rs), [`games/column_four/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/effects.rs), [`games/plain_tricks/src/effects.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/effects.rs), seat framing in [`games/vow_tide/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs) and [`games/river_ledger/src/ids.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs), and test geometry in [`games/high_card_duel/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs) and [`games/poker_lite/tests/visibility.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs). The existing atlas was designed for behavioral mechanics and has correctly rejected broad policy promotion.

**Proposed change:** The ADR should decide: definition and exclusions; allowed homes (`engine-core` ergonomics, `game-stdlib`, `game-test-support`, `wasm-api` adapters); evidence fields; second-use review; third-use hard decision; conditions for second-use promotion; migration/exception debt; hash/visibility review; and interlock with the mechanic atlas. It must explicitly name affected sections in `FOUNDATIONS` §4/§11/§12, `ENGINE-GAME-DATA-BOUNDARY` §13, `MECHANIC-ATLAS` §§4–8/10, `ARCHITECTURE`, and `UI-INTERACTION` §10A.

**Rationale:** This is the minimum lawful way to add the missing category. Editing lower docs alone would silently change the meaning of the constitutional third-use rule.

**Doctrine check:** The ADR must state that scaffolding never includes legality, scoring, reveal, viewer authorization, game effect semantics, team roles, or data-driven behavior. It cannot weaken tests or migration debt.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Accept before C-01 through C-08. Rejected or deferred ADR means those extractions remain local except ordinary generic constructors already clearly within kernel ergonomics.

### D-02 — Recalibrate the gate by category, not by lowering the behavioral bar

**ID:** D-02

**Target file(s):** [`docs/FOUNDATIONS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/FOUNDATIONS.md), [`docs/MECHANIC-ATLAS.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MECHANIC-ATLAS.md), new scaffold register

**Type:** clarify · ADR

**Evidence:** Behavioral third-use discipline produced the narrow `trick_taking` promotion after comparing [`games/plain_tricks/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/rules.rs), [`games/briar_circuit/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/rules.rs), and [`games/vow_tide/src/rules.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/rules.rs); the same discipline correctly deferred shuffle/deal and public accounting in the corresponding ledgers. Mechanical copies such as envelope constructors were already exact by the second game.

**Proposed change:** Decision rule: **behavioral mechanics** keep first-use local, second-use comparison, third-use hard gate. **Mechanical scaffolding** gets mandatory review at the second exact duplication and a hard decision before a third copy. It may promote at second use only if semantic identity is proven, the API is narrow/typed, it carries no policy, all adopters migrate in the same change, and hashes/visibility are unchanged or explicitly versioned. Superficial similarity is recorded and dismissed.

**Rationale:** This is a calibration, not relaxation. It accelerates safe boilerplate extraction while keeping gameplay abstraction conservative.

**Doctrine check:** Must be part of ADR 0008 or a tightly coupled accepted ADR. Behavioral stop conditions stay word-for-word effective.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Measure success by fewer local plumbing copies and unchanged behavioral promotion rates—not by the number of new shared modules.

### D-03 — Author ADR 0009: Replay, evidence-fixture, export, and hash taxonomy v2

**ID:** D-03

**Target file(s):** `docs/adr/0009-replay-fixture-hash-taxonomy.md` (new) plus trace/testing/ADR 0004 cross-references

**Type:** ADR

**Evidence:** Canonical-looking command traces [`games/race_to_n/tests/golden_traces/shortest-normal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/shortest-normal.trace.json) coexist with reduced setup evidence [`games/river_ledger/tests/golden_traces/setup-3p.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json) and private domain evidence [`games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json). Local action-tree hashes differ between [`games/race_to_n/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs) and [`games/draughts_lite/src/replay_support.rs`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/replay_support.rs).

**Proposed change:** The ADR should define artifact classes, visibility classes, validators, version identifiers, canonical byte authority, hash-surface versions, compatibility windows, migration tooling, and the relationship to ADR 0004. Decide whether Trace Schema v1 remains a legacy command schema or is superseded. State that internal private fixtures are never public replay exports and that filename conventions are non-authoritative.

**Rationale:** The repository needs a lawful migration path before canonical action-tree and stable-byte helpers can land.

**Doctrine check:** Replay/hash and visibility contracts are explicit §13 ADR triggers. ADR 0009 must not supersede ADR 0004’s no-leak taxonomy except by naming exact sections and preserving or strengthening them.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Inventory and classify all current golden traces; migrate in profile batches with generated reports and human-reviewed notes.

### D-04 — Adopt a deliberate add/merge/remove plan for docs and templates

**ID:** D-04

**Target file(s):** foundation/template set as enumerated above

**Type:** add · merge · remove · clarify

**Evidence:** The shipped games demonstrate both over-duplication and legitimate specialization: [`games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md) is expansive, [`games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md) is receipt-like, [`games/river_ledger/docs/AI.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/AI.md) is registry-like, and [`games/river_ledger/docs/COMPETENT-PLAYER.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/COMPETENT-PLAYER.md) / [`games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md) remain meaningfully distinct.

**Proposed change:** Add `GAME-EVIDENCE.md` and `MECHANICAL-SCAFFOLDING-REGISTER.md`; split trace/evidence-fixture authority; slim but retain `GAME-SOURCES`, `GAME-RULES`, `RULE-COVERAGE`, `HOW-TO-PLAY`, `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK`, `GAME-UI`, and `GAME-BENCHMARKS`; convert `GAME-AI`, admission, and release checklist to receipts/registries; keep the behavioral pressure ledger separate. Do not merge all foundation docs or collapse competent-player strategy into bot implementation evidence.

**Rationale:** This removes duplicated ownership while retaining audience and authority distinctions that the shipped games actually use.

**Doctrine check:** All removals are field-level or superseded-template migrations, never silent deletion of required proof. Cross-references and checkers must land in the same batch.

**Priority:** High

**Benefiting gates:** 18–23+

**Follow-on:** Publish a migration map and deprecation window; old game docs remain valid until their evidence receipt exists and link checks pass.

### D-05 — Treat Proposed ADR status as an operational state with an expiry/review trigger

**ID:** D-05

**Target file(s):** [`docs/README.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/README.md), [`docs/adr/ADR-TEMPLATE.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/ADR-TEMPLATE.md), [`docs/adr/0005-variance-aware-ci-benchmark-floors.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0005-variance-aware-ci-benchmark-floors.md)

**Type:** clarify · ADR

**Evidence:** ADR 0005 remains Proposed while benchmark evidence is still maintained through game-specific calibrated artifacts, including [`games/river_ledger/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/thresholds.json), [`games/vow_tide/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/benches/thresholds.json), [`games/briar_circuit/benches/thresholds.json`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/benches/thresholds.json), and their filled `BENCHMARKS.md` records. Accepted ADRs 0001–0004, 0006, and 0007 are clearly binding. The distinction is already noted in [`docs/TESTING-REPLAY-BENCHMARKING.md`](https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md) but not centrally indexed.

**Proposed change:** Add a required review trigger/date or next-gate condition for Proposed ADRs and a status index in `docs/README.md`. A Proposed ADR may guide research but must be cited as non-binding. At the trigger, it must be accepted, rejected, superseded, or explicitly renewed with new evidence.

**Rationale:** This prevents draft doctrine from becoming de facto law through repeated references while preserving room for evidence gathering.

**Doctrine check:** Only Accepted ADRs supersede foundation law. Status changes require the normal review process and document updates.

**Priority:** Medium

**Benefiting gates:** 18–23+

**Follow-on:** Apply immediately to ADR 0005 and to ADRs 0008/0009 while they are under review.

## 5. Prioritized execution order

The sequence below honors the repository interlock: atlas promotion debt and the new scaffolding/trace debt must close before the next mechanic-ladder gate proceeds.

### Batch 0 — Authority hygiene and decision readiness

**Scope:** A-01, A-18, A-19, D-05.  
**Output:** complete authority index; central ADR status table; revised ADR template; explicit disposition process for ADR 0005.  
**Exit criteria:** every normative document and ADR has an unambiguous status; Proposed material is not treated as binding.

### Batch 1 — Doctrine decisions before implementation

**Scope:** D-01, D-02, D-03, A-02, A-04, A-06, A-09, A-11, A-12, A-13.  
**Output:** accepted ADR 0008 and ADR 0009; mechanical-scaffolding register; trace/evidence artifact taxonomy; canonical seat grammar; updated roadmap interlocks.  
**Exit criteria:** affected foundation sections are updated in the same series; no contradiction remains between accepted ADRs and foundation text; Gate 18 is blocked until the series closes.

### Batch 2 — Evidence ownership and template spine

**Scope:** A-05, A-07, A-10, A-14, A-16; B-01 through B-16; D-04.  
**Output:** completion profiles; `GAME-EVIDENCE.md`; slimmed admission/AI/release artifacts; stable evidence IDs; conditional strategy/pressure templates.  
**Pilot set:** Gate 18 draft, River Ledger, and Race to N.  
**Exit criteria:** catalog/doc tooling verifies the new evidence receipt; no required proof exists only in a removed field; old links either migrate or resolve through documented compatibility aliases.

### Batch 3 — Lowest-risk production scaffolding

**Scope:** C-01, C-02, C-03.  
**Order:** envelope constructors → seat grammar/import aliases → seat-count/ring helpers.  
**Pilot set:** Race to N and Column Four, then River Ledger and Vow Tide.  
**Exit criteria:** identical public behavior; unchanged hashes or approved migration notes; all visibility/replay/serialization tests pass; matching adopters migrate or have accepted exceptions.

### Batch 4 — Versioned serialization and test infrastructure

**Scope:** C-04 through C-08, A-15.  
**Order:** characterize current bytes/hashes → add stable writer/version types → add action-tree encoding → create `game-test-support` → add viewer/replay profile harnesses.  
**Pilot waves:**

1. Race to N + Column Four (flat/public);
2. Draughts Lite (compound action tree);
3. High Card Duel + Poker Lite (two-seat hidden information);
4. River Ledger + Vow Tide + Briar Circuit (N-seat/private export).

**Exit criteria:** each wave has before/after evidence, no blanket golden regeneration, and a rollback point. Legacy fixture readers exist for any declared compatibility window.

### Batch 5 — Gate 18 admission and implementation

Gate 18 may begin only when:

- the partnership/trick-taking atlas interlock is resolved;
- ADRs 0008 and 0009 are accepted or explicitly rejected with local-code consequences;
- trace profiles and seat identifiers are fixed;
- approved scaffold debt from Batches 3–4 is closed or excepted;
- the Gate 18 admission and evidence receipt use the new profile system.

Gate 18 must keep partnerships, teams, teammate visibility, partnership scoring, and winner reasoning game-local. Existing trick-taking index helpers may be reused where exact semantics match.

### Batch 6 — Gates 19–23 and ongoing governance

Before each later gate:

- review both the mechanic atlas and scaffolding register;
- close third-use behavioral pressure before implementation proceeds;
- review second exact scaffolding duplication and block a third copy without decision;
- add new fixture profiles only through ADR 0009’s extension rules;
- migrate prior matching adopters in the same gate or record a bounded accepted exception.

Suggested gate emphasis:

| Gate | Reuse/evidence emphasis |
|---|---|
| 19 — Five Hundred Rummy | Keep meld/tableau/scoring local; test whether zone/index or evidence harness scaffolding repeats. |
| 20 — Halma | Reuse `board_space` where exact; do not pre-promote graph/path or jump-chain behavior. |
| 21 — Pachisi | Compare track/ring index scaffolding separately from capture/safe-square/race behavior. |
| 22 — Four Winds | Stress compound/reaction action-tree encoding and viewer-safe pending windows; keep reaction priority behavioral. |
| 23 — capstone | Close all open atlas/scaffolding debt and exercise every trace/export profile before declaring phase completion. |
## 6. Risks & explicitly-rejected ideas

### 6.1 Risk register

| Risk | Why it matters | Control |
|---|---|---|
| Stable-byte or action-tree helper changes existing hashes | Golden traces and replay compatibility can appear “fixed” by regeneration while semantics drift. | ADR 0009, versioned encodings, characterization tests, legacy reader/migration notes, no blanket golden updates. |
| Canonical seat grammar misroutes viewers | A string normalization bug can become a hidden-information leak. | Strict Rust parser, typed canonical IDs, import-only aliases, pairwise viewer tests, reject unknown forms. |
| Test harness creates false confidence | A helper that projects or redacts data itself could test its own assumptions rather than the game. | Dev-only dependency; game supplies projection and canaries; helper only iterates/asserts; direct game tests retained. |
| “Mechanical scaffolding” becomes an abstraction loophole | Teams, reveal, betting, or scoring could be relabeled as plumbing. | ADR exclusions, explicit non-promotion list, semantic-risk field, accepted review before promotion. |
| Completion profiles become waivers | Authors may select a lighter profile to skip required evidence. | Profile is derived from shipped capabilities; explicit applicability; foundation invariants always apply; checker rejects unexplained omissions. |
| Template slimming breaks old links/tooling | Specs and scripts may reference removed headings. | Migration map, compatibility anchors, same-batch checker updates, bounded deprecation period. |
| Mega-retrofit obscures regressions | 17-game simultaneous changes make hash/leak failures hard to localize. | Representative waves with rollback points and exact adopter lists. |
| External prior art dilutes Rulepath doctrine | Comparable frameworks have different authority and AI assumptions. | `SOURCES.md` records Rulepath-specific lesson and explicit non-adoption; foundation law remains controlling. |

### 6.2 Explicitly rejected ideas

| Rejected idea | Repository evidence | Why rejected |
|---|---|---|
| Promote a generic deck/deal/private-hand/reveal helper now | `plain_tricks`, `briar_circuit`, `vow_tide`, `poker_lite`, `high_card_duel`, and `river_ledger` differ in deal order, tail/community/trump handling, reveal policy, and export safety; their pressure ledgers record those differences. | Repetition is behavioral, not scaffolding. A broad helper would encode policy or force awkward callbacks that hide policy. |
| Centralize shuffle without preserving RNG consumption | Local implementations use different bounded-sampling patterns and produce replay-relevant byte sequences. | Determinism would change. First standardize/document bounded-index semantics and characterize vectors. |
| Add a generic view projector or redaction engine | Visibility modules and ADR 0004 make authorization game-specific; effects, diagnostics, explanations, and terminal reveals differ. | Leak-safe construction cannot be delegated to generic field filtering without a rule model or DSL. Share test geometry, not projection behavior. |
| Put teams/partnerships in `engine-core` before Gate 18 | Existing seats are generic; no shipped partnership game establishes teammate visibility, scoring, or active-set semantics. | It would add speculative game nouns/policy to the kernel. Gate 18 must prove local semantics first. |
| Promote generic pots, public resources, or accounting | River Ledger side pots/remainder order and Token Bazaar/resource games have different conservation, ownership, tie, and effect semantics. | Numeric resemblance is not shared behavior. Narrow arithmetic helpers may emerge later only with exact evidence. |
| Promote a universal trick lifecycle | Only follow-suit index selection and winner comparison proved identical; dealer rotation, winner-leads, first-trick restrictions, scoring, bids, hearts-breaking, and partnerships differ. | The current narrow `trick_taking` module is correctly calibrated. |
| Generalize graph control/pathfinding from two area games | Frontier Control and Event Frontier already diverge on connectivity scoring, majority/control, contests, and faction asymmetry. | Atlas correctly defers until a true third close use; Halma/Pachisi topology should start local and compare. |
| Introduce YAML, selectors, triggers, or a rules DSL to reduce boilerplate | No shipped game demonstrates a typed-Rust limitation that requires it. | Directly violates foundation law without ADR and would move behavior into data. |
| Use automatic reflection/derive as stable-hash authority | Current hashes depend on deliberate ordering and surface selection. | Field additions/reordering could silently change bytes; explicit versioned writers are safer. |
| Adopt RFC 8785 wholesale as the hash format | JCS proves why canonicalization matters but has JSON/I-JSON/number semantics and cross-language concerns not yet evaluated for Rulepath. | Use it as pressure-test evidence; choose Rulepath’s byte contract through ADR 0009. |
| Move legality or convenience validation into TypeScript | The web layer could appear simpler, especially for shared controls. | Violates the core authority invariant and creates divergence/leak risk. Rust/WASM remains the only legality authority. |
| Merge all foundation documents into one mega-document | The current layered authority separates constitution, architecture, boundary, area law, roadmap, and process. | The problem is duplicated operational evidence, not too many distinct authorities. Improve indexing and ownership instead. |
| Merge `COMPETENT-PLAYER` and `BOT-STRATEGY-EVIDENCE-PACK` completely | Human strategy and deterministic implementation evidence have different audiences and proof burdens. | Keep them separate but remove duplicated sections and link them explicitly. |
| Delete release/admission records entirely | Actual filled copies show value as decisions and sign-offs. | Slim them into receipts; do not remove the lifecycle checkpoints. |
| Retrofit all 17 games in one change | The games span flat/compound, perfect/hidden, two/N-seat, and multiple trace generations. | Bounded waves are safer, reviewable, and consistent with agent discipline. |
---

## Appendix A — Final exact-commit acquisition ledger

```text
Requested repository: joeloverbeck/rulepath
Target commit: db0c50b95f84df12b349710033c77db2bf7326b3
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.open exact-raw-URL preflight/verification and container.download exact full raw URLs
Requested file count: 350
Successfully verified file count: 350
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

Fetched repository files:
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/archive/reports/foundation-doc-realignment.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/archive/reports/public-game-ladder-and-implementation-order.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/action.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/game.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/engine-core/src/rng.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/board_space.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/game-stdlib/src/trick_taking.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/action_path.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/action_tree.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/actors.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/json.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/seats.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/crates/wasm-api/src/store.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/AGENT-DISCIPLINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/AI-BOTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ARCHITECTURE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/FOUNDATIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/IP-POLICY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MECHANIC-ATLAS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/OFFICIAL-GAME-CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/ROADMAP.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TESTING-REPLAY-BENCHMARKING.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/TRACE-SCHEMA-v1.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/UI-INTERACTION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/WASM-CLIENT-BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0001-stage-1-random-playout-budget.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0002-ci-benchmark-gating-lanes.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0003-ci-calibrated-benchmark-thresholds.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0004-hidden-info-replay-export-taxonomy.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0005-variance-aware-ci-benchmark-floors.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0006-blackjack-lite-roadmap-placement.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/adr/ADR-TEMPLATE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/docs/archival-workflow.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/benches/thresholds.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/cards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/scoring.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/golden_traces/seat-private-pairwise-no-leak.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/briar_circuit/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/column_four/tests/golden_traces/wasm-exported.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/directional_flip/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/directional_flip/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/directional_flip/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/draughts_lite/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/event_frontier/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/flood_watch/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/flood_watch/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/flood_watch/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/flood_watch/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/frontier_control/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/golden_traces/shortest-normal.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/high_card_duel/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/masked_claims/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/masked_claims/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/masked_claims/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/masked_claims/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/plain_tricks/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/golden_traces/deal-private-no-leak.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/golden_traces/public-replay-export-import.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/golden_traces/wasm-exported.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/poker_lite/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/shortest-normal.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/race_to_n/tests/golden_traces/wasm-exported.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/river_ledger.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/benches/thresholds.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/betting.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/cards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/evaluator.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/pot.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/showdown.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/deal-private-no-leak.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/public-replay-export-import.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-3p.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/golden_traces/setup-6p.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/river_ledger/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/secret_draft/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/secret_draft/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/secret_draft/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/secret_draft/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/three_marks/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/three_marks/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/three_marks/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/three_marks/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/token_bazaar/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/token_bazaar/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/token_bazaar/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/token_bazaar/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/benches/thresholds.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/cards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/scoring.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/public-replay-export-import.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/seat-private-pairwise-no-leak-7p.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/games/vow_tide/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/specs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/AGENT-TASK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-HOW-TO-PLAY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/GAME-UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/templates/README.md`
## Appendix B — Existing ADR disposition

| ADR | Status at target commit | Change-plan treatment |
|---|---|---|
| 0001 — Stage 1 random playout budget | Accepted | No decision change. Bot/replay scaffolding must preserve its bounded public AI assumptions. |
| 0002 — CI benchmark gating lanes | Accepted | No decision change. ADR 0005 review and workload IDs must remain compatible. |
| 0003 — Calibrated benchmark thresholds | Accepted | No decision change. Threshold migrations remain explicit. |
| 0004 — Hidden-info replay/export taxonomy | Accepted | No decision change. It governs all view/export/harness recommendations and constrains ADR 0009. |
| 0005 — Variance-aware CI benchmark floors | Proposed | Resolve through A-19/D-05; non-binding until Accepted. |
| 0006 — Blackjack Lite roadmap placement | Accepted | No decision change. The plan does not reorder the ladder or revive the game as an architecture driver. |
| 0007 — Next public scaling phase and Gate P tail | Accepted | No decision change. Batches and Gate 18–23 sequencing preserve it. |

## Appendix C — Self-check

- Every recommendation names exact game files, symbols, or filled-template evidence from the target commit.
- Duplication-driven promotions were compared against live implementations; behaviorally divergent shapes are explicitly rejected or kept local.
- The behavioral third-use gate remains intact; only a separately defined, ADR-governed scaffolding category is proposed.
- No recommendation introduces YAML, a DSL, data-driven rules, TypeScript legality, public Monte Carlo/MCTS/ML/RL, noun-bearing mechanics in `engine-core`, or weaker tests.
- ADR 0004 remains controlling for hidden-information replay/export; replay/hash changes require ADR 0009.
- Phase-0 multi-seat, N-seat template, roadmap, and ADR 0007 work is acknowledged as shipped and is not re-proposed.
- All foundation documents, all seven ADRs, the ADR template, `archival-workflow.md`, and all 15 existing templates have an explicit disposition.
- The deliverable is advisory and names—without executing—existing-game retrofit opportunities.
