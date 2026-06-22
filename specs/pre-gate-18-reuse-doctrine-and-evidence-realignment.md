# Pre-Gate-18 — Reuse-Doctrine & Evidence Realignment

**Status**: Planned

| Field | Value |
|---|---|
| Spec ID | `pre-gate-18-reuse-doctrine-and-evidence-realignment` |
| Roadmap stage | Public scaling phase — pre-Gate-18 foundation pass (non-feature) |
| Roadmap build gate | Pre-Gate-18 — **non-feature** docs / doctrine / template realignment |
| Status | Planned |
| Date | 2026-06-22 |
| Owner | Rulepath maintainers |
| Primary targets | `docs/adr/0008-*.md` (new), `docs/adr/0009-*.md` (new), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (new), `docs/EVIDENCE-FIXTURE-CONTRACT.md` (new), `templates/GAME-EVIDENCE.md` (new), the foundation/area doc set, `templates/**`, `specs/README.md` |
| Browser implementation | Not applicable; documentation / law / template realignment pass only |
| Authority order | `docs/FOUNDATIONS.md` → `docs/README.md` → accepted `docs/adr/0008-*.md` + `docs/adr/0009-*.md` → `docs/MECHANIC-ATLAS.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/AGENT-DISCIPLINE.md` → this spec |

Where this spec and a foundation document disagree, the foundation document
wins. This spec is seeded from one advisory research report —
[`../reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`](../reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md)
(and its commissioning brief
[`../reports/doc-and-template-overhaul-from-game-evidence-research-brief.md`](../reports/doc-and-template-overhaul-from-game-evidence-research-brief.md)) —
authored against target commit `db0c50b`, which is the current `HEAD` at
authoring time. This spec does not assume the report's per-file claims are
permanent; AGENT-TASK decomposition (`/reassess-spec`) re-reads each target file
against live `main` before editing, and the report's IDs are carried only as
traceability anchors.

> Reader orientation: this spec carries the canonical Rulepath section set as a
> **docs/law/template realignment** pass, not a gameplay gate. It is a delta on
> the shipped Phase 0 realignment, grounded this time in the implementation
> experience of **17 shipped games** (Gates 0–17 `Done`). It adds the missing
> **mechanical-scaffolding** reuse lane (via an accepted ADR), repairs trace /
> hash / fixture authority (via an accepted ADR), and consolidates duplicated
> per-game evidence into one canonical receipt — so every later Gate 18+ spec is
> easier to ground and the production code-extraction that the new doctrine
> authorizes can land safely in a **separate, ADR-gated successor unit**. It
> writes **no game, kernel, tooling, CI, trace, fixture, or benchmark code.**

## Objective

Realign the foundation docs, area docs, ADR set, and templates so that:

1. the constitution gains a lawful, narrow **mechanical-scaffolding** reuse lane
   (typed, behavior-free infrastructure that the mechanic-atlas third-use gate
   was never designed to govern), **without lowering the behavioral hard gate**;
2. replay / hash / fixture / export authority is made explicit and versioned, so
   heterogeneous `*.trace.json` artifacts stop masquerading as one schema; and
3. duplicated per-game evidence (no-leak matrices, IP receipts, benchmark and
   strategy declarations copied across five documents) collapses into one
   canonical `GAME-EVIDENCE.md` receipt with explicit completion profiles.

The report's central thesis, confirmed against the codebase: Rulepath's
*behavioral*-reuse doctrine is basically right and the third-use hard gate should
remain intact for behavioral mechanics. The new pressure the 17-game corpus
exposes is (a) a missing governance lane for behavior-free plumbing repeatedly
rebuilt around the generic kernel, and (b) operational duplication and
authority gaps in the docs/templates — **not** that Phase 0 failed.

Because adding a reuse lane amends `FOUNDATIONS.md` §4/§11 and the
`ENGINE-GAME-DATA-BOUNDARY.md` boundary, and because the trace/hash/export
taxonomy changes are §13 ADR triggers, the **superseding ADRs are authored and
accepted first**, and the foundation-doc edits they gate land only after. No
production code, no kernel change, and no hash/trace migration happens in this
pass — those are the next, ADR-gated unit.

## Scope

### In scope (report Batches 0–2)

- **ADR 0008** (new, accepted): *Mechanical Scaffolding Governance* (report D-01,
  D-02). Defines the mechanical-scaffolding category and its exclusions; allowed
  homes (`engine-core` contract ergonomics, `game-stdlib`, a future dev-only
  `game-test-support`, `wasm-api` adapters); evidence fields; a category-specific
  decision rule (review at second exact duplication, hard decision before a third
  copy, second-use promotion only when semantic identity is proven, the API is
  noun-free/properly game-layer typed, behavior-neutral, deterministic, leak-safe,
  and migration-complete); interlock with the mechanic atlas; and the exact
  foundation sections it amends. The behavioral third-use wording stays
  word-for-word effective.
- **ADR 0009** (new, accepted): *Replay, evidence-fixture, export, and hash
  taxonomy v2* (report D-03). Defines artifact classes, visibility classes,
  validators, version identifiers, canonical-byte authority, hash-surface
  versions, compatibility windows, the relationship to ADR 0004, and whether
  Trace Schema v1 stays a legacy command schema or is superseded. **Decision
  only** — no fixture/hash bytes change in this pass.
- **Authority hygiene** (report A-01, A-18, A-19, D-05): add `TRACE-SCHEMA-v1.md`
  (and the new register/contract docs) to `docs/README.md`'s authority map;
  strengthen `ADR-TEMPLATE.md` — it already carries `Affected foundation
  sections`, `Superseded decision`, `Rollback / contamination risk`, and split
  determinism/replay-hash/visibility-impact sections (today under an *Optional*
  heading), so this is **promote the relevant existing fields from optional to
  required** plus **add only the genuinely-absent fields** (compatibility window,
  evidence-classification, accepted-exceptions, an "effective only after named
  foundation updates land" field, and a Proposed-ADR review-trigger/expiry field),
  and upgrade the prose `Migration plan` to an adopter/migration matrix; a
  Proposed-ADR status policy
  with a review trigger and central status index; and a bounded
  accept/reject/supersede disposition of the still-`Proposed` ADR 0005.
- **Foundation/area-doc amendments gated on the ADRs** (report A-02, A-03, A-04,
  A-06): a `FOUNDATIONS.md` §4 mechanical-scaffolding definition (after ADR 0008);
  an `ARCHITECTURE.md` ownership matrix (kernel ergonomics / `game-stdlib` /
  test-support); four explicit reuse lanes in `ENGINE-GAME-DATA-BOUNDARY.md`; and
  a `MECHANIC-ATLAS.md` split that keeps the behavioral gate and links the new
  scaffolding register.
- **Trace/test/seat/roadmap doc edits** (report A-09, A-11, A-12, A-13): shared
  test-support law + named fixture profiles + a hash-migration protocol in
  `TESTING-REPLAY-BENCHMARKING.md`; the `TRACE-SCHEMA-v1.md` narrowing + new
  `EVIDENCE-FIXTURE-CONTRACT.md`; a canonical external seat grammar
  (`seat_<zero-based>` as the going-forward form — `SeatId` is today an opaque
  string, and the corpus already diverges across `seat-<n>` hyphen, `seat_<n>`
  underscore, and the `seat-a` letter form in engine-core's own doctest) + a
  compatibility/alias policy that enumerates those existing forms (migration
  deferred), in `WASM-CLIENT-BOUNDARY.md`, which today defines no seat grammar;
  and pre-Gate-18 + per-gate scaffolding/trace debt interlocks in
  `ROADMAP.md`/`specs/README.md` (the `specs/README.md` Part C seed and Gate 18
  interlock already exist — see D13 — so the `ROADMAP.md` interlocks are the new
  work there).
- **New register / contract / receipt docs** (report A-06, A-11, B-02):
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (governed by ADR 0008),
  `docs/EVIDENCE-FIXTURE-CONTRACT.md` (governed by ADR 0009), and the new
  `templates/GAME-EVIDENCE.md` canonical conformance receipt.
- **Template realignment** (report B-01, B-03…B-16, A-05, A-07, A-08, A-10, A-14,
  A-16, A-17, A-15, D-04): completion profiles in `templates/README.md`; slim the
  source/rules/coverage/UI/benchmark/strategy templates to their domain authority
  with stable evidence IDs and links into `GAME-EVIDENCE.md`; convert `GAME-AI`,
  admission, and release checklist into receipts/registries; keep the
  primitive-pressure ledger behavioral and redirect non-behavioral repetition to
  the scaffolding register; and reference law/evidence from `AGENT-TASK.md`
  instead of copying it (incl. a scaffold-refactor protocol from A-15).

### Out of scope (the next units, not this pass)

| Surface | Why deferred | Where it lands |
|---|---|---|
| All production code-reuse moves (report Part C: C-01 `EffectEnvelope` constructors, C-02 seat-ID helpers, C-03 seat-count/ring helpers, C-04 action-tree encoding, C-05 stable-byte writer, C-09 RNG bounded-index) | Kernel/game code + ADR-gated hash/visibility migration | Seeded forward unit — **Mechanical-scaffolding code extraction (Part C)**, blocked on accepted ADRs 0008/0009 |
| `crates/game-test-support` (new dev-only crate) + shared test harnesses (report C-06, C-07, C-08) | New crate + test code | Same forward code unit (report Batch 4) |
| Per-game retrofits to adopt new scaffolding / `GAME-EVIDENCE.md` / evidence IDs (report C-11 retrofit waves; B-* "Follow-on") | Game code + doc migration | Bounded retrofit specs per wave, after the code unit |
| Checker/tooling support for `GAME-EVIDENCE.md`, evidence IDs, and authority-map verification (report A-01, B-02, B-05 follow-ons) | Tooling code (`scripts/*.mjs`, `tools/rule-coverage`) | Forward code unit / a small tooling ticket; this pass *specifies* the machine-checkable fields, it does not implement the checker |
| Migrating / regenerating any `*.trace.json` fixture, rules-version anchor, or golden hash to the new profiles | Data/hash migration under ADR 0009 | Forward code unit (report C-08, Batch 4) |
| Gate 18 (Spades / partnerships) admission and implementation | Gameplay | Existing Gate 18 seed row (report Batch 5) |
| Reclassifying historical atlas/ledger rows or rewriting old game docs | Mass retrofit | Opportunistic, when those files are next touched (report A-06, A-17, B-* "Follow-on") |

### Not allowed

This realignment pass MUST NOT:

- add any mechanic noun (`board`, `card`, `deck`, `hand`, `pot`, `trick`,
  `faction`, `partnership`, …) to `engine-core`, or promote any helper into
  `game-stdlib` outside the `MECHANIC-ATLAS.md` third-use discipline;
- **implement** any mechanical-scaffolding extraction, new crate, or shared
  harness — ADR 0008 *authorizes* the lane; the code lands in the successor unit;
- introduce YAML, a DSL, selectors, conditions, triggers, or formulas as data;
- change any `*.trace.json` fixture, `TRACE-SCHEMA-v1.md` *field bytes*, hash,
  RNG, or serialization output — ADR 0009 *decides* the taxonomy; migration is the
  successor unit;
- weaken, delete, or relax the behavioral third-use hard gate, no-leak,
  determinism, replay/hash proofs, or the no-public-MCTS/ML/RL bot law;
- weaken or supersede a FOUNDATIONS principle or an accepted ADR by **editorial
  edit** — every meaning change to §4/§11/§12 or to ADR 0004's taxonomy routes
  through ADR 0008/0009's named-section supersession, never an unannounced doc
  edit;
- silently delete a template field that carries required proof — every removal is
  a field-level migration into `GAME-EVIDENCE.md` (or another named owner) with
  the cross-reference landing in the same change;
- write game code, tooling, CI, traces, fixtures, or benchmarks; edit archived
  specs; or rewrite `docs/ROADMAP.md` as a progress diary.

## Deliverables

| # | Artifact | Required change | Report IDs |
|---:|---|---|---|
| D1 | `docs/adr/0008-mechanical-scaffolding-governance.md` (new) | Accepted ADR defining the mechanical-scaffolding lane, exclusions, allowed homes, evidence fields, the second-use-review / pre-third-copy hard-decision rule, atlas interlock, and exact amended sections (`FOUNDATIONS` §4/§11/§12, `ENGINE-GAME-DATA-BOUNDARY` §13, `MECHANIC-ATLAS` §§4–8, `ARCHITECTURE`, `UI-INTERACTION` §10A). Built from the revised `ADR-TEMPLATE.md`. ID `0008` (next after `0007`). | D-01, D-02 |
| D2 | `docs/adr/0009-replay-fixture-hash-taxonomy.md` (new) | Accepted ADR defining artifact/visibility classes, validators, version identifiers, canonical-byte authority, hash-surface versions, compatibility windows, the relationship to ADR 0004, and the disposition of Trace Schema v1. Decision only; no bytes change. ID `0009`. | D-03 |
| D3 | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (new) | The scaffolding decision register governed by ADR 0008: per-entry semantic-risk classification, production-vs-test home, affected hashes, visibility impact, exact duplicate sites, migration set, and rejection rationale; carries the explicit non-promotion list (deal/reveal/projection/betting/pot/trick-lifecycle/teams/graph/accounting/reaction/scoring stay behavioral). | A-06, C-10, D-04 |
| D4 | `docs/EVIDENCE-FIXTURE-CONTRACT.md` (new) | Setup/domain evidence-fixture contract governed by ADR 0009: named profiles (`replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, `domain-evidence-v1`), validator ownership, visibility classification, version anchors, allowed private test-only data; states filename suffix is non-authoritative. | A-09, A-11, D-03 |
| D5 | `templates/GAME-EVIDENCE.md` (new) | Canonical machine-friendly conformance receipt: game/rules/data/trace versions; completion profile; supported seats/variants; source/IP receipt; rule-coverage summary; named trace profiles; public + per-seat viewer matrix; replay/hash compatibility; benchmark workload IDs; bot levels/policy IDs; mechanic + scaffolding register decisions; release state; blockers; exact artifact links. Status + links only — no duplicated domain prose. | B-02, A-05 |
| D6 | `docs/README.md` | Add `TRACE-SCHEMA-v1.md`, D3, and D4 to the ordered authority map at their correct layers (subordinate to FOUNDATIONS/ARCHITECTURE/ADR 0004); add a central ADR status index; state Proposed ADRs are informative only. | A-01, D-05 |
| D7 | `docs/adr/ADR-TEMPLATE.md` | **Reframed after reassessment** — several requested fields already exist (`Affected foundation sections` L25, `Superseded decision` L26, `Rollback / contamination risk` L30, split `Determinism`/`Replay-hash`/`Visibility` impact sections L68–83, prose `Migration plan` L29), today under an *Optional* heading. So: (a) **promote** the relevant existing fields from optional to required; (b) **add only the absent** fields — evidence-classification, compatibility window, accepted-exceptions, an "effective only after named foundation updates land" field, and a Proposed-ADR review-trigger/expiry field; (c) **upgrade** the prose `Migration plan` to an adopter/migration matrix; (d) keep a single selected `Status` value. Do **not** re-introduce already-present fields. | A-18, D-05 |
| D8 | ADR 0005 disposition | A bounded decision review of `docs/adr/0005-variance-aware-ci-benchmark-floors.md` using shipped benchmark data: accept (with wording fixes), reject, or supersede/withdraw; update all references (esp. `TESTING-REPLAY-BENCHMARKING.md`) to state the outcome and remove any binding-sounding prose if it stays non-accepted. **Delta on Phase 0** (which already made it non-binding); this pass makes the actual disposition + adds the D-05 status policy. | A-19, D-05 |
| D9 | Foundation/boundary/atlas docs (`FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `MECHANIC-ATLAS.md`) | After ADR 0008 accepted: §4 mechanical-scaffolding definition + scaffolding-register invariant; ARCHITECTURE ownership matrix (3 lanes + dev-dependency rule); four explicit reuse lanes (kernel ergonomics / scaffolding / behavioral mechanics / typed content) with narrowest-layer-wins; atlas keeps §§4–8 behavioral gate and links the register. | A-02, A-03, A-04, A-06 |
| D10 | Testing / trace / seat / roadmap docs (`TESTING-REPLAY-BENCHMARKING.md`, `TRACE-SCHEMA-v1.md`, `WASM-CLIENT-BOUNDARY.md`, `ROADMAP.md`) | After ADR 0009 accepted: test-support law + fixture profiles + hash-migration protocol; narrow Trace Schema v1 to the command/replay contract (or mark superseded) + point at D4; canonical seat grammar (`seat_<zero-based>` as the going-forward form) + a bounded alias policy that enumerates the divergent in-corpus forms (`seat-<n>`, `seat_<n>`, `seat-a`); migration deferred; pre-Gate-18 and per-gate scaffolding/trace debt interlocks. | A-09, A-11, A-12, A-13 |
| D11 | AI / UI / official-contract / IP / sources / discipline / archival docs (`AI-BOTS.md`, `UI-INTERACTION.md`, `OFFICIAL-GAME-CONTRACT.md`, `IP-POLICY.md`, `SOURCES.md`, `AGENT-DISCIPLINE.md`, `archival-workflow.md`) | One owner/purpose per AI doc; replace the UI count trigger (UI-INTERACTION.md §10A, line 227: presentation-helper promotion deferred until "a third structural divergence ... or an **official-game count above 20**") with a semantic scaffolding review; point the official-game contract at `GAME-EVIDENCE.md` + define completion profiles; centralize the IP evidence receipt + source IDs; add the 8 external sources with Rulepath-specific lessons/non-adoptions; add the bounded scaffolding/hash-refactor protocol; add an archival closeout-receipt requirement. | A-05, A-07, A-08, A-14, A-16, A-17, A-15 |
| D12 | `templates/**` realignment | `templates/README.md` completion profiles + lifecycle; slim `GAME-SOURCES`, `GAME-RULES`, `GAME-RULE-COVERAGE`, `GAME-MECHANICS`, `GAME-HOW-TO-PLAY`, `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK`, `GAME-UI`, `GAME-BENCHMARKS`, `PRIMITIVE-PRESSURE-LEDGER` to domain authority + evidence IDs + links; convert `GAME-AI`, `GAME-IMPLEMENTATION-ADMISSION`, `PUBLIC-RELEASE-CHECKLIST` to receipts/registries; reference-not-copy law in `AGENT-TASK.md` + scaffold-refactor profile. No silent deletion of required proof. Decomposition (`/spec-to-tickets`, one ticket per reviewable diff = one per template) maps each report `B-NN` to its template by re-reading the report §4 change-plan; record that `B-ID → template` mapping in the per-template tickets so each diff traces to its source claim without re-reading the full report. | B-01, B-03…B-16, D-04 |
| D13 | `specs/README.md` | **Reframed after reassessment** — the **Mechanical-scaffolding code extraction (Part C)** forward row already exists (`8C`, line 98, `Not started`, **Blocked** on ADRs 0008/0009 + the 8M realignment), and the Gate 18 interlock already exists (row `9`, line 99, requiring 8M `Done` + 8C debt closed + ADRs 0008/0009 accepted + fixed trace profiles/seat grammar + partnership atlas interlock). So D13 is: **confirm/reconcile** those already-present rows (do **not** duplicate them — refine wording only if reassessment surfaced drift), and the only `specs/README.md` mutation on exit is **flipping this realignment row (`8M`) to `Done`**. New per-gate/pre-Gate-18 scaffolding/trace debt interlocks land in `ROADMAP.md` (D10/WB11), not here. | A-13, Batch 5 |

## Work breakdown

Each item is a candidate AGENT-TASK; `/reassess-spec` then `/spec-to-tickets`
split these into one ticket per reviewable diff. **Dependency order is
load-bearing — the ADRs are authored and accepted before the foundation edits
they gate.** Report IDs are traceability anchors; the reassess step verifies each
against live `main`.

| # | Candidate task | Depends on | Report IDs | Notes |
|---:|---|---|---|---|
| WB1 | Authority hygiene: complete the `docs/README.md` authority map + ADR status index; strengthen `ADR-TEMPLATE.md`; add the Proposed-ADR status policy | — | A-01, A-18, D-05 | Docs only; no ADR doctrine yet. Provides the template the new ADRs use. |
| WB2 | Dispose of ADR 0005 (accept/reject/supersede) using shipped benchmark data; reconcile `TESTING-REPLAY-BENCHMARKING.md` references | WB1 | A-19, D-05 | Delta on Phase 0's "made non-binding". Bounded decision review, no benchmark rewrite. |
| WB3 | Author + accept **ADR 0008** (mechanical-scaffolding governance + category decision rule) | WB1 | D-01, D-02 | Must be **Accepted** before WB6. Names the exact amended sections. Behavioral gate unchanged. |
| WB4 | Author + accept **ADR 0009** (replay/fixture/hash taxonomy v2) | WB1 | D-03 | Must be **Accepted** before WB7. Decision only; no bytes change. Must not weaken ADR 0004 except by named-section supersession. |
| WB5 | Write `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (incl. non-promotion list) and `docs/EVIDENCE-FIXTURE-CONTRACT.md`; index both in `docs/README.md` | WB3, WB4 | A-06, A-11, C-10, D-04 | Register governed by ADR 0008; fixture contract by ADR 0009. |
| WB6 | FOUNDATIONS §4 + ARCHITECTURE ownership matrix + ENGINE-GAME-DATA-BOUNDARY four-lane split + MECHANIC-ATLAS behavioral/scaffolding split | WB3, WB5 | A-02, A-03, A-04, A-06 | Gated on ADR 0008 accepted. Assert the semantic-preservation guarantee; route any genuine §11 meaning change through the ADR. |
| WB7 | TESTING test-support law + fixture profiles + hash-migration protocol; TRACE-SCHEMA-v1 narrowing/supersession note; WASM canonical seat grammar (`seat_<zero-based>` going-forward; `SeatId` is opaque today) + alias policy enumerating the existing `seat-<n>`/`seat_<n>`/`seat-a` forms (migration deferred) | WB4, WB5 | A-09, A-11, A-12 | Gated on ADR 0009 accepted. **No** field/version byte change to any fixture in this pass. |
| WB8 | `templates/GAME-EVIDENCE.md` (new) + completion profiles in `templates/README.md` | WB1 | B-01, B-02, A-05 | Defines the receipt + profile system the slimming depends on. Specify machine-checkable fields; checker impl deferred. |
| WB9 | Official-game-contract + AI + UI + IP + SOURCES + AGENT-DISCIPLINE + archival doc edits | WB3, WB8 | A-05, A-07, A-08, A-14, A-16, A-17, A-15 | Point contract/IP at `GAME-EVIDENCE.md`; one-owner AI docs; semantic UI scaffolding review; scaffold-refactor protocol; closeout receipt. |
| WB10 | Template slimming + receipts/registries across `templates/**` | WB5, WB8 | B-03…B-16, D-04 | Every removed field migrates to a named owner; no silent proof deletion. Behavioral pressure ledger stays behavioral and redirects to the register. |
| WB11 | ROADMAP + `specs/README.md` interlocks: add pre-Gate-18 exit criteria + per-gate atlas + scaffolding-debt review to `ROADMAP.md` (new); confirm/reconcile the already-present `specs/README.md` Part C (`8C`) seed and Gate 18 interlock (do not duplicate) | WB3, WB4, WB6, WB10 | A-13, Batch 5 | Keep the ladder unchanged; add debt interlocks only. The new debt interlocks are the `ROADMAP.md` work; the `8C` seed + Gate 18 interlock already exist in the index. Flip the `8M` row to `Done`; run the doc-link/catalog/boundary checks. |

## Exit criteria

| Criterion | Evidence required |
|---|---|
| Scaffolding lane is lawful | `docs/adr/0008-*.md` exists with `Status: Accepted`, names every amended section, and keeps the behavioral third-use wording effective; `FOUNDATIONS.md` §4 and `ENGINE-GAME-DATA-BOUNDARY.md` carry the new lane only after acceptance. |
| Replay/hash/fixture taxonomy decided | `docs/adr/0009-*.md` exists with `Status: Accepted`; `docs/EVIDENCE-FIXTURE-CONTRACT.md` exists with named profiles + validators; ADR 0004's no-leak taxonomy is preserved or strengthened, never silently amended. |
| No bytes/contract drift in this pass | No `*.trace.json`, hash, RNG, serialization, or benchmark output changed; `TRACE-SCHEMA-v1.md` field bytes unchanged (its prose is narrowed/annotated only); the WASM exported-API schema is unchanged (the seat grammar is documented, migration deferred). |
| Authority map + ADR status complete | `docs/README.md` lists `TRACE-SCHEMA-v1.md`, the register, and the fixture contract at correct layers and carries an ADR status index; ADR 0005 is accepted/rejected/superseded and no longer cited as binding if non-accepted. |
| Boundary preserved | `engine-core` gains no mechanic noun; no helper is promoted to `game-stdlib`; `bash scripts/boundary-check.sh` passes. |
| Evidence receipt + profiles exist | `templates/GAME-EVIDENCE.md` exists; `templates/README.md` defines completion profiles with explicit not-applicable reasons; the official-game contract points at the receipt. |
| Template slimming loses no proof | Every field removed from a template is migrated to a named owner (`GAME-EVIDENCE.md` or another template) with the cross-reference present; no §11 invariant or §12 stop condition is waived by a profile. |
| Index + links truthful | `specs/README.md` has the `8M` row `Done`, the Part C code unit (`8C`) present and blocked, and the Gate 18 interlock present (both pre-existing, reconciled not duplicated); `node scripts/check-doc-links.mjs` passes. |

## Acceptance evidence

Game-level evidence (rules/traces/benchmarks/bot-legality/UI smoke) is **not
applicable** — this pass has no game and writes no code. Minimum acceptance
transcript:

```bash
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
bash scripts/boundary-check.sh
```

Recommended review checks:

```bash
git diff -- docs/ templates/ specs/README.md
# Confirm no kernel/code/fixture drift slipped into a docs pass:
git status --porcelain -- crates/ games/ tools/ apps/ scripts/ '**/*.trace.json'
# Confirm the new lane added no mechanic noun to the kernel:
grep -rnE "board|card|deck|hand|pot|trick|faction|partnership" docs/MECHANICAL-SCAFFOLDING-REGISTER.md
```

The diff must be documentation / law / template only — no Rust, WASM, tooling,
CI, trace, fixture, or benchmark files. If any check fails, follow the
failing-test protocol (`AGENT-DISCIPLINE` §4): determine whether the check is
valid, then whether the issue is in the edited docs or the check, then fix
without weakening the check.

## FOUNDATIONS & boundary alignment

This pass writes **doctrine** (it sets law for a new reuse lane and the
trace/hash taxonomy), so the alignment below covers the *doctrine being written*,
not code. The full read of `docs/FOUNDATIONS.md` confirms each cited section.

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Scaffolding is behavior-free by definition; TS gains no legality. The seat-grammar and test-support docs reaffirm Rust owns parse/format, projection, and legality. |
| §3 `engine-core` noun-free kernel | Aligned (load-bearing) | The new lane targets nouns **already** in §3's allowed vocabulary (`effect envelope`, `seat id`, `action tree`, `visibility scope`); it explicitly excludes mechanic nouns. `boundary-check.sh` gates it. |
| §4 `game-stdlib` earned | Engaged → ADR 0008 | Adding a reuse category *beside* the third-use behavioral gate amends §4; FOUNDATIONS L3 ("supersede only by accepted ADR") + §13 require ADR 0008 first. The behavioral hard gate is preserved word-for-word. |
| §5 Static data is not behavior | Aligned | No YAML/DSL/selectors; the scaffolding lane stays Rust-typed and behavior-free; `GAME-EVIDENCE.md` carries status/links, not rule data. |
| §11 Universal invariants | Engaged → ADR 0008/0009 | The §11 "game-stdlib changes are earned" and determinism/no-leak invariants are amended only by named-section supersession in the ADRs; profiles never waive an invariant; ADR 0004's leak taxonomy is preserved/strengthened. |
| §12 Stop conditions | Clear | Stop if any edit would change a trace field/hash, weaken the behavioral gate, relabel behavior (teams/reveal/betting/scoring) as plumbing, or move behavior into data — those route to the ADRs or stay local (non-promotion list). No code "generalize the engine" work happens here. |
| §13 ADR triggers | Engaged → ADR 0008 + 0009 | "Promoting mechanics outside the normal primitive-pressure path" and "changing engine-core vocabulary/responsibilities" → ADR 0008; "changing replay/hash semantics" and "changing public/private visibility contracts" → ADR 0009. Both are authored + accepted before the foundation edits they gate. |

ADR 0004 (hidden-info replay/export) is **preserved or strengthened**: ADR 0009
may only touch it by naming exact sections, and the new shared test-support and
fixture profiles add proof geometry without moving projection/redaction policy
out of the games.

## Forbidden changes

Do not:

- add mechanic nouns to `engine-core` or promote helpers outside atlas discipline;
- **implement** any scaffolding extraction, new crate, harness, or per-game
  retrofit — this pass authorizes the lane; code lands in the successor unit;
- change any `*.trace.json` fixture, `TRACE-SCHEMA-v1.md` field bytes, hash, RNG,
  serialization, or benchmark output (each needs ADR 0009 + the migration unit);
- introduce YAML/DSL/selectors/conditions/triggers/formulas;
- supersede or weaken a FOUNDATIONS principle, the behavioral third-use gate, or
  an accepted ADR by editorial edit; meaning changes route through ADR 0008/0009;
- delete a template field that carries required proof without migrating it to a
  named owner in the same change;
- weaken no-leak, determinism, replay/hash proofs, or the no-public-MCTS/ML/RL
  bot law;
- write or edit Rust crates, WASM, tools, CI, traces, fixtures, benchmarks,
  checkers, or game code;
- edit archived specs; rewrite ROADMAP as a progress diary;
- delete or weaken tests/checks to make validation pass.

## Documentation updates required

- New: `docs/adr/0008-mechanical-scaffolding-governance.md`,
  `docs/adr/0009-replay-fixture-hash-taxonomy.md`,
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`,
  `docs/EVIDENCE-FIXTURE-CONTRACT.md`, `templates/GAME-EVIDENCE.md`.
- Edited (per the per-document recommendations): `docs/README.md`,
  `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`,
  `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/OFFICIAL-GAME-CONTRACT.md`,
  `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`,
  `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/TRACE-SCHEMA-v1.md`,
  `docs/WASM-CLIENT-BOUNDARY.md`, `docs/ROADMAP.md`, `docs/IP-POLICY.md`,
  `docs/SOURCES.md`, `docs/AGENT-DISCIPLINE.md`, `docs/archival-workflow.md`,
  `docs/adr/ADR-TEMPLATE.md`, `docs/adr/0005-variance-aware-ci-benchmark-floors.md`
  (status disposition).
- Edited: `templates/**` per the per-template recommendations +
  `templates/README.md` completion profiles/lifecycle.
- Edited: `specs/README.md` — flip the `8M` row to `Done`; confirm/reconcile the
  already-present Part C (`8C`) seed and Gate 18 interlock (do not duplicate them).
- This pass has **no** web-exposed game gate, so the web-shell catalog README
  (`apps/web/README.md`) is **not applicable** as a closeout surface
  (`check-catalog-docs.mjs` still runs as a guard).

## Sequencing

- **Predecessor:** Gate 17 (`vow_tide`) is `Done`; the public mechanic ladder
  through Gate 17 is complete with no open atlas promotion debt (per
  `specs/README.md`). This pass is a delta on the shipped Phase 0 realignment.
- **This pass:** docs / doctrine (ADR 0008 + 0009) / templates realignment. No
  game, no code, no fixture/hash migration.
- **Successor:** the seeded **Mechanical-scaffolding code extraction (Part C)**
  unit — blocked until ADRs 0008/0009 are `Accepted` and this pass is `Done` —
  then Gate 18 (Spades / partnerships).
- **Admission rule:** Gate 18 may begin only when this realignment is `Done`, the
  Part C scaffolding debt is closed or carries an accepted exception, ADRs 0008
  and 0009 are accepted (or explicitly rejected with local-code consequences),
  trace profiles and the canonical seat grammar are fixed, and the
  partnership/trick-taking atlas interlock is resolved. Gate 18 keeps teams,
  teammate visibility, partnership scoring, and winner reasoning game-local.

## Assumptions

- A1 (one-line-correctable): The single authored unit is the docs/doctrine/
  template realignment (report Batches 0–2); the production code-reuse moves
  (Part C / Batches 3–4) are seeded as a separate, ADR-blocked successor unit —
  assuming the Phase-0 → Infra-A–D precedent (docs spec separate from code specs)
  is the desired slicing. *(Confirmed with the user at authoring time.)*
- A2: ADR 0008 and ADR 0009 are the only ADRs this pass authors, and authoring
  them here (with maintainer acceptance) is in scope — assuming the Phase-0
  precedent where the realignment spec authored ADR 0007 and a maintainer flipped
  it to `Accepted`.
- A3: The foundation-doc edits gated on the ADRs are meaning-preserving *given the
  accepted ADRs*; any edit that would change a §11 invariant's meaning beyond what
  the ADR names is split out rather than landed here.
- A4: No `TRACE-SCHEMA-v1.md` field/version byte change and no fixture/hash
  migration is needed in this pass — assuming ADR 0009 can decide the taxonomy and
  defer all byte migration to the code unit (per the report).
- A5: Checker/tooling support for `GAME-EVIDENCE.md`, evidence IDs, and
  authority-map verification is deferred to the code unit / a tooling ticket; this
  pass specifies the machine-checkable fields but implements no `scripts/*.mjs` or
  `tools/*` change — assuming a docs-only diff keeps the pass reviewable and
  matches Phase 0.
- A6: ADR 0005's disposition is a *delta* on Phase 0 (which made it non-binding):
  this pass makes the accept/reject/supersede decision and adds the D-05
  Proposed-ADR status policy — assuming the residual is the decision + policy, not
  re-litigating the non-binding status Phase 0 already shipped.
- A7: The report's per-file evidence (commit `db0c50b`, current `HEAD`) is
  re-verified per target file at `/reassess-spec` time; report IDs are
  traceability anchors, not a substitute for the live re-read.
- A8: Template field removals are field-level migrations into a named owner, never
  silent deletions — assuming any field that cannot find an owner is retained
  rather than dropped.
- A9 (validated current state, from `/reassess-spec` against live `main`): the
  edit-target premises were resolved so decomposition need not re-investigate.
  **Confirmed absent / edit needed:** `FOUNDATIONS.md` §4 scaffolding lane;
  `ARCHITECTURE.md` ownership matrix + `game-test-support` layer;
  `ENGINE-GAME-DATA-BOUNDARY.md` four-lane split; `WASM-CLIENT-BOUNDARY.md` seat
  grammar (none defined today); `docs/README.md` authority-map entries for
  `TRACE-SCHEMA-v1.md` + a central ADR status index + a "Proposed ADRs are
  informative only" statement; `TESTING-REPLAY-BENCHMARKING.md` test-support law /
  fixture profiles / hash-migration protocol; `ROADMAP.md` pre-Gate-18 + per-gate
  scaffolding/trace debt interlocks. **Confirmed present / target exists:**
  `MECHANIC-ATLAS.md` §§4–8 (behavioral gate), `ENGINE-GAME-DATA-BOUNDARY.md` §13,
  `UI-INTERACTION.md` §10A + the "official-game count above 20" trigger (line 227),
  ADR 0004 (`Accepted`, governs the no-leak taxonomy ADR 0009 must preserve),
  ADR 0005 (`Proposed`; binding MUST/MUST-NOT prose still present despite the
  Phase-0 non-binding note — so D8's "remove binding-sounding prose if non-accepted"
  is live), `TRACE-SCHEMA-v1.md` (self-describes as "canonical trace **and replay
  fixture** schema" — broader than the command/replay contract, no superseded
  marker yet), and several `ADR-TEMPLATE.md` fields (see D7). **Already done /
  do not recreate:** the `specs/README.md` Part C (`8C`) seed and Gate 18 interlock
  (see D13).
