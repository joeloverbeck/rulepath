# Phase 0 — Foundation Realignment & Next-Phase Admission

| Field | Value |
|---|---|
| Spec ID | `phase-0-next-phase-foundation-realignment` |
| Roadmap stage | Public scaling phase — Order 0 (pre-Gate-15 foundation pass) |
| Roadmap build gate | Phase 0 — **non-feature** docs/templates/admission realignment |
| Status | Planned |
| Date | 2026-06-13 |
| Owner | Rulepath maintainers |
| Primary targets | `docs/adr/0007-*.md` (new), `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (new), the foundation/area doc set, `templates/**`, `docs/ROADMAP.md` |
| Browser implementation | Not applicable; documentation/law realignment pass only |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ROADMAP.md` → accepted `docs/adr/0007-*.md` → `docs/AGENT-DISCIPLINE.md` → this spec |

Where this spec and a foundation document disagree, the foundation document wins.
This spec is seeded from two advisory research reports —
[`../reports/foundation-doc-realignment.md`](../reports/foundation-doc-realignment.md)
and
[`../reports/public-game-ladder-and-implementation-order.md`](../reports/public-game-ladder-and-implementation-order.md)
— authored against user-supplied target commit `e3b1729`. This spec does not
verify that commit is the current `main`; AGENT-TASK decomposition re-reads each
target file against live `main` before editing.

> Reader orientation: this spec carries the canonical Rulepath section set as a
> **docs/law realignment** pass, not a gameplay gate. It admits the public
> scaling phase (3+ seats, larger surfaces) into the authority docs and templates
> so that every later Gate 15+ spec can be grounded without re-litigating N-seat
> obligations. It writes no game code.

## Objective

Realign the foundation docs, area docs, templates, and ROADMAP so the public
scaling phase can be executed without weakening the constitution, and record the
roadmap admission in an accepted ADR.

The reports' shared verdict, confirmed against the codebase: the next phase can
be admitted **without** moving nouns into `engine-core`, introducing YAML/DSL, or
relaxing no-leak or deterministic replay. The kernel is already seat-generic
(`Game::setup(seats: &[SeatId])`, optional-seat `Viewer`,
`VisibilityScope::PrivateToSeat`, a `seats` array in replay). The two-seat
ceiling lives in **game crates, tooling (`tools/simulate`), and web-shell
presentation** — not the kernel. The missing piece is documentation: an explicit
N-seat / larger-surface contract underneath the existing law, plus clarifications
threaded through the foundation/area docs and templates, and a roadmap that
records the new public phase with Gate P moved to the tail.

## Scope

### In scope

- **ADR 0007** (new): admit a public scaling phase after Gate 14 and move Gate P
  to the tail. This is the hard prerequisite — `docs/ROADMAP.md`'s header law
  forbids gate reorder/addition without an accepted ADR (a FOUNDATIONS §13-class
  architecture decision).
- **`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`** (new area doc): state the N-seat
  and larger-surface obligations once, governed by `FOUNDATIONS.md`,
  `ARCHITECTURE.md`, and `ENGINE-GAME-DATA-BOUNDARY.md`. **Codification only** —
  it restates existing principles for N-seat/large-surface contexts; it does not
  change trace-schema semantics, browser authority, or kernel boundaries.
- **Foundation/area doc clarifications** (meaning-preserving): `docs/README.md`
  map; `FOUNDATIONS.md`; `ARCHITECTURE.md`; `ENGINE-GAME-DATA-BOUNDARY.md`;
  `OFFICIAL-GAME-CONTRACT.md`; `MECHANIC-ATLAS.md`; `AI-BOTS.md`;
  `UI-INTERACTION.md`; `TESTING-REPLAY-BENCHMARKING.md`; `TRACE-SCHEMA-v1.md`
  (semantics note, **no** field/version change); `WASM-CLIENT-BOUNDARY.md`
  (conceptual refresh, no exported-API schema change); `IP-POLICY.md`;
  `SOURCES.md`; `AGENT-DISCIPLINE.md`; `archival-workflow.md` (add an index
  rollover section); explanatory notes on ADRs `0004` and `0006`.
- **Resolve the ADR 0005 status ambiguity** — either accept it through the normal
  ADR process and update status/index references, or stop citing it as accepted.
  New-phase specs must not rely on a `Proposed` ADR as law.
- **Template realignment** across `templates/**` — add seat-range, turn-order,
  view-matrix, pairwise no-leak, outcome-matrix, and surface-scale fields per the
  realignment report's per-template matrix.
- **`docs/ROADMAP.md`** (after ADR 0007 is accepted): add the public scaling
  phase after Gate 14; restate Gate P as last, private, optional, non-architectural.
- **Reconcile this index** (`specs/README.md`): flip the Phase 0 row through to
  `Done`, and update the Gate 15+ rows' interlock notes to reflect ADR 0007 once
  accepted and ROADMAP once edited.

### Out of scope (later units, not this pass)

| Surface | Why deferred | Where it lands |
|---|---|---|
| `tools/simulate` `seat_0_wins`/`seat_1_wins` generalization | Code change | Infra B spec |
| N-seat setup/catalog metadata, setup UX | Code change | Infra A spec |
| Multi-seat web-shell frame (seat rail, turn-order, viewer selector) | Code change | Infra C spec |
| N-player no-leak test harness | Code change | Infra D spec |
| Any game crate (River Ledger / Hold'Em, Hearts, …) | Gameplay | Gate 15+ specs |
| `templates/MULTI-SEAT-VIEW-MATRIX.md` (optional new template) | Optional; report marks it foldable into existing templates | A later template pass if pressure warrants |

### Not allowed

This realignment pass MUST NOT:

- add any mechanic noun (`board`, `card`, `deck`, `seat`-typed enums, `pot`,
  `pawn`, `meld`, `faction`, …) to `engine-core`, or promote any helper into
  `game-stdlib` outside the `MECHANIC-ATLAS.md` third-use discipline;
- introduce YAML, a DSL, selectors, conditions, triggers, or formulas as data;
- change `TRACE-SCHEMA-v1.md` fields or version, change the WASM exported-API
  schema incompatibly, or alter any replay/hash/RNG/serialization contract — any
  such change requires its own ADR and is out of this pass;
- weaken or supersede a FOUNDATIONS principle or an accepted ADR by editorial
  edit; a genuine principle change requires its own superseding ADR first;
- weaken, delete, or relax no-leak, determinism, or the no-public-MCTS/ML/RL bot law;
- write game code, tooling, CI, traces, fixtures, or benchmarks;
- edit archived specs or rewrite `docs/ROADMAP.md` as a progress diary.

## Deliverables

| # | Artifact | Required change |
|---:|---|---|
| D1 | `docs/adr/0007-<slug>.md` (new) | Accepted ADR admitting the public scaling phase after Gate 14 and moving Gate P to the tail; built from `docs/adr/ADR-TEMPLATE.md`. ID `0007` (next after `0006`). |
| D2 | `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (new) | N-seat/larger-surface contract: seat-range declaration; stable seat IDs + optional roles/teams; turn-order model; viewer matrix; pairwise hidden-info no-leak matrix; public-observer rules; topology/object-count and action-fanout budgets; per-seat final-breakdown requirement; trace/view-hash expectations; simulator-summary format for arbitrary winners/splits/teams. |
| D3 | `docs/README.md` | Add D2 to the document map under the foundation/area-doc layer, before `ROADMAP.md`; mark it subordinate to the constitution, architecture, data boundary, hidden-info ADRs, and bot law. |
| D4 | Foundation docs (`FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `OFFICIAL-GAME-CONTRACT.md`) | Meaning-preserving N-seat clarifications per the report's per-document matrix (invariants apply to any positive seat count; pairwise seat-private redaction; N-seat nouns are game-local/atlas-only; per-seat/per-team outcome + showdown rationale). |
| D5 | Bot/UI/testing/trace/WASM/atlas docs (`AI-BOTS.md`, `UI-INTERACTION.md`, `TESTING-REPLAY-BENCHMARKING.md`, `TRACE-SCHEMA-v1.md`, `WASM-CLIENT-BOUNDARY.md`, `MECHANIC-ATLAS.md`) | N-player imperfect-info bot subsection; multi-seat layout + showdown rendering; N-seat no-leak taxonomy + benchmark-by-seat-count; N-seat trace semantics (no schema change); multi-seat WASM operations (conceptual); next-phase armed-interlock register. |
| D6 | IP / sources / discipline / archival (`IP-POLICY.md`, `SOURCES.md`, `AGENT-DISCIPLINE.md`, `archival-workflow.md`) | Hold'Em casino-trade-dress note; next-phase scaling sources; canonical bounded N-seat task examples; spec-index rollover section. |
| D7 | ADR notes / status | Explanatory cross-reference notes on ADR `0004` (pairwise export) and `0006` (placement precedent); **resolve ADR `0005` status** (accept-and-update or stop-citing-as-accepted); optional `ADR-TEMPLATE.md` fields for scaling ADRs. |
| D8 | `templates/**` | Seat-range / turn-order / view-matrix / pairwise-no-leak / outcome-matrix / surface-scale fields across the templates named in the report's per-template matrix; `templates/README.md` adoption note. |
| D9 | `docs/ROADMAP.md` | After D1 is accepted: public scaling phase after Gate 14; Gate P restated as last/private/optional/non-architectural. |
| D10 | `specs/README.md` | Flip Phase 0 row to `Done` on exit; refresh Gate 15+ interlock notes once ADR 0007 + ROADMAP land. |

## Work breakdown

Each item is a candidate AGENT-TASK; `/spec-to-tickets` splits these into one
ticket per reviewable diff. Dependency order is load-bearing — ADR 0007 first.

| # | Candidate task | Depends on | Notes |
|---:|---|---|---|
| 1 | Author + accept **ADR 0007** (public scaling phase + Gate P tail) | — | Use the report's proposed outline. Must be **accepted** before any ROADMAP edit (WB10). |
| 2 | Write **`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`** | 1 | Codification only; cite FOUNDATIONS/ARCHITECTURE/DATA-BOUNDARY as governing. |
| 3 | Update `docs/README.md` document map | 2 | Place D2 before ROADMAP; mark subordination. |
| 4 | FOUNDATIONS + ARCHITECTURE N-seat clarifications | 2 | Meaning-preserving; assert the semantic-preservation guarantee in the diff. |
| 5 | ENGINE-GAME-DATA-BOUNDARY + MECHANIC-ATLAS clarifications | 2 | Name N-seat/topology danger points; arm next-phase third-use interlocks; "large map is not a DSL license." |
| 6 | OFFICIAL-GAME-CONTRACT + UI-INTERACTION N-seat/showdown rows | 2 | Per-seat/per-team breakdown; pairwise no-leak proof; multi-seat layout + showdown rendering. |
| 7 | AI-BOTS N-player imperfect-info subsection | 2 | Legal sources only; forbid MCTS/ISMCTS/Monte Carlo/ML/RL; per-viewer explanation redaction. |
| 8 | TESTING + TRACE-SCHEMA semantics (no field/version change) | 2 | N-seat no-leak taxonomy; benchmark-by-seat-count; stricter `seats`-array semantics with **no** migration. |
| 9 | WASM-CLIENT-BOUNDARY conceptual refresh + IP/SOURCES/AGENT-DISCIPLINE/archival notes + ADR 0004/0006 notes + ADR 0005 status resolution | 2 | Conceptual only (no exported-API schema change); resolve ADR 0005 ambiguity. |
| 10 | `templates/**` realignment + `templates/README.md` adoption note | 4,5,6,7,8 | Seat/surface/no-leak/outcome fields per the per-template matrix. |
| 11 | `docs/ROADMAP.md` public scaling phase + Gate P tail | 1,10 | Only after ADR 0007 accepted. |
| 12 | Reconcile `specs/README.md` + run doc-link/boundary/catalog checks | 11 | Flip Phase 0 row; refresh interlock notes; validate. |

## Exit criteria

| Criterion | Evidence required |
|---|---|
| Roadmap admission recorded | `docs/adr/0007-*.md` exists with `Status: Accepted`; ROADMAP records the public scaling phase and Gate P as the tail. |
| Multi-seat contract exists and is indexed | `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` exists and is listed in `docs/README.md`'s document map before ROADMAP. |
| Clarifications are meaning-preserving | Foundation-doc diffs add N-seat clarifications without changing any principle's meaning; no FOUNDATIONS principle or accepted ADR is superseded by editorial edit. |
| No schema/contract drift | `TRACE-SCHEMA-v1.md` fields/version unchanged; no replay/hash/RNG/serialization contract changed; WASM exported-API schema unchanged. |
| Boundary preserved | `engine-core` gains no mechanic noun; `bash scripts/boundary-check.sh` passes. |
| Templates carry N-seat fields | Each template named in the report's matrix has the seat/surface/no-leak/outcome additions. |
| ADR 0005 ambiguity resolved | ADR 0005 is either accepted (status/index updated) or no longer cited as accepted anywhere. |
| Index + links truthful | `specs/README.md` Phase 0 row is `Done`; `node scripts/check-doc-links.mjs` passes. |

## Acceptance evidence

Game-level evidence (rules/traces/benchmarks/bot-legality/UI smoke) is **not
applicable** — this gate has no game. Minimum acceptance transcript:

```bash
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
bash scripts/boundary-check.sh
```

Recommended review checks:

```bash
git diff -- docs/ templates/ specs/README.md
grep -rnE "engine-core" docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md   # confirm no noun smuggling into the kernel
```

The diff must be documentation/law only — no Rust, WASM, tooling, CI, trace,
fixture, or benchmark files. If any check fails, follow the failing-test
protocol: determine whether the check is valid, then whether the issue is in the
edited docs or the check, then fix without weakening the check.

## FOUNDATIONS & boundary alignment

The work this spec governs is product-behavior-adjacent (it sets law for N-seat
behavior), so the alignment below covers the *doctrine being written*, not code.

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Docs reaffirm Rust/WASM owns legality; TS displays N-seat turn order but never infers it. |
| §3 `engine-core` noun-free kernel | Aligned (load-bearing) | Contract explicitly keeps N-seat/poker/map/topology nouns game-local or `game-stdlib`-via-atlas; `boundary-check.sh` gates it. |
| §4 `game-stdlib` earned | Aligned | Atlas third-use discipline is reaffirmed and armed for the next phase; no premature promotion. |
| §5 Static data is not behavior | Aligned | "Large map is not a DSL license" — topology may be typed content; conditions/triggers/formulas stay Rust. No YAML/DSL. |
| §11 Universal invariants | Aligned | Invariants restated to apply to **any positive seat count**, incl. pairwise seat-private redaction; replay/hash/RNG/serialization contracts unchanged. |
| §12 Stop conditions | Clear | Stop if any edit would change a trace field/version, the WASM schema, a kernel boundary, or a principle's meaning — those route to their own ADR, not this pass. A 3+ game that cannot prove viewer-safe public + per-seat projections does not ship. |
| §13 ADR triggers | Engaged → ADR 0007 | Roadmap reorder/addition requires an accepted ADR per `ROADMAP.md`'s header law; ADR 0007 — a §13-class architecture decision — satisfies it (WB1) before any ROADMAP edit. No other §13 trigger is tripped (no schema/visibility/bot-class change). |

Hidden-information no-leak (ADR 0004) is **strengthened, not weakened**: the new
taxonomy adds pairwise (A≠B) negative assertions and per-viewer export proofs.

## Forbidden changes

Do not:

- add mechanic nouns to `engine-core` or promote helpers outside atlas discipline;
- introduce YAML/DSL/selectors/conditions/triggers/formulas;
- change `TRACE-SCHEMA-v1.md` fields/version, the WASM exported-API schema, or any
  replay/hash/RNG/serialization/determinism contract (each needs its own ADR);
- supersede or weaken a FOUNDATIONS principle or accepted ADR by editorial edit;
- weaken no-leak, determinism, or the no-public-MCTS/ML/RL bot law;
- write or edit Rust crates, WASM, tools, CI, traces, fixtures, benchmarks, or
  game code;
- edit archived specs; rewrite ROADMAP as a progress diary;
- delete or weaken tests/checks to make validation pass.

## Documentation updates required

- New: `docs/adr/0007-*.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`.
- Edited (clarifications/notes per the per-document matrix): `docs/README.md`,
  `FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`,
  `OFFICIAL-GAME-CONTRACT.md`, `MECHANIC-ATLAS.md`, `AI-BOTS.md`,
  `UI-INTERACTION.md`, `TESTING-REPLAY-BENCHMARKING.md`, `TRACE-SCHEMA-v1.md`,
  `WASM-CLIENT-BOUNDARY.md`, `IP-POLICY.md`, `SOURCES.md`, `AGENT-DISCIPLINE.md`,
  `archival-workflow.md`, ADR `0004`/`0005`/`0006` (notes/status), `ADR-TEMPLATE.md`.
- Edited: `templates/**` per the per-template matrix + `templates/README.md`.
- Edited: `docs/ROADMAP.md` (after ADR 0007 accepted); `specs/README.md` (Phase 0
  row → `Done`; refresh Gate 15+ interlock notes).

## Sequencing

- **Predecessor:** Gate 14 (`event_frontier`) is Done; the public mechanic ladder
  through Gate 14 is complete.
- **This pass:** docs/templates/ROADMAP realignment + ADR 0007 admission. No game.
- **Successor:** Infra A–D (code interlocks) and then Gate 15 (River Ledger /
  Texas Hold'Em). None may be authored as a grounded spec until ADR 0007 is
  accepted and ROADMAP records the phase (Infra A–D additionally depend on the
  Phase 0 multi-seat contract and no-leak taxonomy doctrine).
- **Admission rule:** ADR 0007 must be `Accepted` before WB11 edits ROADMAP; the
  semantic-preservation guarantee must hold for every foundation-doc edit, else
  that edit routes to its own superseding ADR.

## Assumptions

- A1: The foundation-doc edits are meaning-preserving clarifications of existing
  principles (the governance-doc editorial gate), not principle changes —
  assuming any edit that would change a principle's meaning is split out into its
  own superseding ADR rather than landed here.
- A2: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` only codifies existing principles
  and needs no ADR of its own — assuming it changes no trace-schema semantics,
  browser authority, or kernel boundary.
- A3: No `TRACE-SCHEMA-v1.md` field/version change is needed for N seats —
  assuming the existing `seats` array + viewer hashes suffice (per the report).
- A4: ADR 0007 is the only ADR this pass authors; the optional
  `templates/MULTI-SEAT-VIEW-MATRIX.md` is deferred unless the template pass shows
  the fields don't fold into existing templates.
- A5: Original game/rules design (Hold'Em scope, evaluator, the capstone) is **not**
  decided here — it belongs to the Gate 15+ specs, where the
  research-or-skip decision (`research-brief` / `deep-research`) is made explicit.
- A6: The two source reports' commit baseline (`e3b1729`) is close enough to live
  `main` that per-file re-reads at decomposition time will reconcile any drift.
