# infra-a-d-n-seat-public-infrastructure — N-seat setup/catalog, simulator summaries, multi-seat shell, and N-player no-leak harness

- **Filename:** `specs/infra-a-d-n-seat-public-infrastructure.md`
- **Spec ID:** `infra-a-d-n-seat-public-infrastructure`
- **Roadmap stage:** Public scaling phase — Infra A–D (non-gate infrastructure interlocks)
- **Roadmap build gate:** `docs/ROADMAP.md` §15 "Infra A–D: N-seat public infrastructure interlocks" (summary-table row `15A`). Not a mechanic-ladder game gate.
- **Status:** Done
- **Date:** 2026-06-14
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs (`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs (esp. ADR 0004) → this spec.
- **Subordination:** Subordinate to the foundation docs, area docs, accepted ADRs, and the multi-seat contract. This spec plans infrastructure capability; it does not amend or weaken upstream law and authorizes no trace-schema, hash, WASM-API-schema, or visibility-contract migration.

---

## 1. Objective

Close the four cross-cutting public-infrastructure assumptions named in
`docs/ROADMAP.md` §15 "Infra A–D" before the first official N-seat game gate
(Gate 15, River Ledger). The platform is two-seat-bound today in four concrete
places; this phase generalizes each to be seat-count-agnostic while preserving
all existing two-seat behavior, so Gate 15 inherits ready plumbing rather than
discovering it mid-game.

This is **one combined spec** for the single ROADMAP ladder unit `15A`, which
carries one shared Exit list. The four units remain individually reviewable at
the **ticket** layer (the work breakdown in §5 is per-unit, and
`/spec-to-tickets` emits one ticket per reviewable diff). The units are:

- **Infra A — N-seat setup and catalog metadata.** Rust owns seat-count
  acceptance, setup validation, and per-game variant/seat-range metadata;
  TypeScript only presents it.
- **Infra B — N-seat simulator summaries.** `tools/simulate` summary output uses
  deterministic seat-keyed maps instead of fixed two-seat scalar counters.
- **Infra C — Multi-seat shell frame.** Seat rails, active/pending seats,
  observer mode, and viewer selection render Rust/WASM-projected state only.
- **Infra D — N-player no-leak test harness.** Pairwise private-datum × viewer ×
  surface assertions over browser payloads, DOM, storage, logs, bot
  explanations, candidate rankings, and replay exports, conforming to ADR 0004.

Grounding law: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §2 (seat-range
declaration) → Infra A; §13 (simulator summaries) → Infra B; §4/§5/§7
(turn-order, viewer matrix, public observer) → Infra C; §6 (pairwise no-leak
matrix) → Infra D. ADR 0004 is the replay/export authority for Infra D.

---

## 2. Current state and motivating evidence

Verified against the working tree (2026-06-14):

| Unit | Current two-seat binding | Evidence |
|---|---|---|
| A | `seats()` returns a hardcoded two-element slice; the browser cannot choose a count | `crates/wasm-api/src/lib.rs:4095-4097` (`vec![SeatId("seat-0"), SeatId("seat-1")]`) |
| A | Catalog metadata hardcodes `"viewer_modes":["observer","seat_0","seat_1"]` per game; no seat-range / supported-count metadata | `crates/wasm-api/src/lib.rs:461-520` |
| A | Setup goes through per-game `*_setup_match(Seed, &seats, &SetupOptions)`; `Game::setup` already receives a seat slice (contract §2) | `crates/wasm-api/src/lib.rs:569-670` |
| A | Each game already validates seat count in its own setup and rejects a wrong count with a `Diagnostic`; a `variant.seat_count` field already exists (so per-game rejection is not new work — Infra A wires/projects it) | `games/race_to_n/src/setup.rs:27-30` ("requires exactly two seats"); identical pattern in all current games |
| B | Summary structs and per-game aggregation use `seat_0_wins`/`seat_1_wins` / `seat_0_n`/`seat_1_n` scalars | `tools/simulate/src/main.rs:87-88, 267-268, 295-383` |
| C | Web shell has `AppShell`, `shellReducer`, `ReplayViewer`, `ModeControls`; no shared seat-rail / active-pending / viewer-selector frame; each board hand-rolls seat display | `apps/web/src/components/AppShell.tsx`, `apps/web/src/state/shellReducer.ts` |
| D | No-leak proof is per-game `wasm-api` bridge tests + per-game `tests/replay.rs`/`tests/serialization.rs`/`src/visibility.rs`, all viewer-keyed to `observer/seat_0/seat_1`; no central N-player pairwise harness | `crates/wasm-api/src/lib.rs` bridge tests; `games/*/tests/replay.rs`, `games/*/src/visibility.rs` |

Every current official game is two-seat. There is therefore no >2-seat official
game to exercise the generalized plumbing end-to-end before Gate 15. This spec's
verification strategy (below) proves (a) byte-identical behavior for the existing
two-seat games and (b) N>2 correctness via seat-count-agnostic unit/property
tests with synthetic seat fixtures; the first full end-to-end N-seat proof lands
at Gate 15 — which is precisely why Infra A–D is its prerequisite.

---

## 3. Scope

### 3.1 In scope

- **A:** Rust-owned seat-count acceptance + setup-diagnostic surface; per-game
  seat-range / supported-set / seat-label / default-seat metadata projected
  through the catalog payload; WASM-API and `client.ts` type additions; web
  setup/catalog presentation of Rust-supplied seat metadata.
- **B:** `tools/simulate` summary generalization to deterministic seat-keyed maps
  (ordered seat IDs; win/loss/draw/tie/split counts keyed by seat ID; terminal
  reason counts; failure sample seeds), with stable machine-readable key order.
- **C:** A shared multi-seat shell frame in `apps/web` (seat rail, active/pending
  indication, observer mode, viewer selector) rendering only Rust/WASM-projected
  active/pending/turn-order/viewer state.
- **D:** A reusable N-player no-leak harness covering the ADR-0004 / contract-§6
  surface set, parameterized by seat count and viewer, runnable in Rust and in
  the web smoke layer.
- Lift-ready amendments only where a unit's contract field needs an acceptance
  clause (see §10); the doc edits land at closeout, not before.

### 3.2 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| The Gate 15 game (River Ledger) itself, or any new game | Out | Infra A–D is the prerequisite; the game is the successor unit. |
| Trace-schema, trace-version, hash-semantic, or replay-compatibility migration | Forbidden here | FOUNDATIONS §13 / contract §12 ADR trigger. Infra uses existing replay concepts consistently (stable `seats` array order). |
| WASM exported-API **schema** migration beyond additive viewer-safe fields | Out | Schema changes route through their own review; this phase adds seat metadata fields, not new API contracts. |
| `engine-core` mechanic nouns (`team`/`table`/`pot`/`role`/`faction`), or `game-stdlib` seat-model promotion | Forbidden | Kernel stays noun-free (FOUNDATIONS §3); `seat id`/`actor`/`viewer` are already §3-permitted, but seat *models* and the mechanic nouns above start in `games/*`; promotion is atlas-only. |
| Roles / teams / partnerships modeling | Deferred | Contract §3 keeps these game-local; first exercised at a teams gate (Gate 18 Spades), not by infrastructure. |
| Force-converting existing two-seat games to a different seat model | Rejected | Infra is seat-count-agnostic plumbing; existing games stay byte-identical. |

### 3.3 Not allowed (carried from ROADMAP §15 "Not allowed across the public scaling phase" + Infra exit)

- No TypeScript legality, turn-order, or active-seat **inference** from seat
  index, DOM, local setup mode, or rendered labels (contract §4).
- No hidden-information leak across any public/browser/replay/bot surface
  (contract §6, ADR 0004).
- No kernel noun growth; no YAML/DSL; no static-data-as-behavior.
- No private licensed content or public/private content-policy relaxation.
- No trace/schema/hash migration; no bot-policy-law change; no MCTS/ISMCTS/
  Monte Carlo/ML/RL.

---

## 4. Deliverables

```text
crates/wasm-api/src/lib.rs            (A: seat-count acceptance + setup diagnostics;
                                        seat-range/supported-set/label catalog metadata;
                                        D: harness hooks where bridge-level)
apps/web/src/wasm/client.ts           (A: seat metadata + setup-diagnostic types;
                                        C: active/pending/viewer projection types)
apps/web/src/components/MatchSetup.tsx (A: present Rust seat-range + validation messages)
apps/web/src/components/GamePicker.tsx (A: surface supported seat counts from catalog)
tools/simulate/src/main.rs            (B: seat-keyed deterministic summary maps,
                                        per-game aggregation generalized)
apps/web/src/components/<SeatFrame>.tsx (C: shared multi-seat shell frame — name indicative)
apps/web/src/state/shellReducer.ts    (C: viewer/active-seat selection state, presentation-only)
crates/wasm-api/tests/ or a shared test module (D: N-player pairwise no-leak harness)
games/*/tests/ + apps/web/e2e/        (D: adopt harness; A/B/C regression coverage)
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md, docs/TESTING-REPLAY-BENCHMARKING.md,
  docs/UI-INTERACTION.md             (closeout: acceptance-clause amendments only, §10)
apps/web/README.md                    (C: Shell Surface section — seat frame)
specs/README.md                       (index row maintained / flipped)
```

Per-unit Rust/tool changes ride the ordinary verification set: unit/rule tests,
`fixture-check`, `replay-check`, `rule-coverage`, serialization tests,
`boundary-check.sh`, `simulate` smoke; web changes ride `smoke:wasm`/`smoke:ui`/
`smoke:effects`.

---

## 5. Work breakdown (candidate AGENT-TASK decomposition)

Dependency order; each item is one reviewable diff. Unit order A → B → C → D
matches the index interlocks (B pending A; C pending B; D pending C + Phase 0
no-leak taxonomy).

| # | Unit | Item | Depends on |
|---|---|---|---|
| WB1 | A | Rust seat-count acceptance + setup diagnostics: generalize the bridge `seats()` path to request a count and build the N-element slice, then surface each game's *existing* `setup.rs` seat-count rejection `Diagnostic` (e.g. `race_to_n/src/setup.rs:27-30`) rather than inventing new validation; existing two-seat games unchanged in behavior | — |
| WB2 | A | Extend the existing per-game `variant.seat_count` to a seat-range/supported-set/default + seat-label metadata, and project it into the catalog payload (currently un-projected); `client.ts` types; no behavior fields | WB1 |
| WB3 | A | Web setup/catalog presentation: `MatchSetup`/`GamePicker` present Rust seat range + validation messages; TS infers no legality | WB2 |
| WB4 | B | `tools/simulate` summary generalization: deterministic seat-keyed maps (ordered seat IDs; win/loss/draw/tie/split keyed by seat ID; terminal-reason counts; failure seeds); stable key ordering; existing two-seat output covered by a parity/regression check | WB1 |
| WB5 | C | Shared multi-seat shell frame component (seat rail, active/pending, observer mode, viewer selector) rendering Rust/WASM-projected active/pending/turn-order/viewer state only; `shellReducer` viewer-selection state is presentation-only | WB2, WB3 |
| WB6 | C | Adopt the shell frame across existing boards (or record a board-native exception per board); replay viewer + observer mode use the frame | WB5 |
| WB7 | D | N-player pairwise no-leak harness: parameterized by seat count × viewer × surface (payloads, action trees, previews, effects, bot explanations, candidate rankings, replay exports, DOM/test-id/storage/logs); conforms to ADR 0004; synthetic N-seat fixtures where no official >2-seat game exists | WB1, Phase 0 no-leak taxonomy |
| WB8 | D | Adopt the harness for the existing hidden-information games (regression at 2 seats) and wire it into the web no-leak smoke layer; name supported seat counts + max-surface fixtures in the evidence | WB5, WB7 |
| WB9 | all | Closeout: acceptance-clause amendments (§10), `apps/web/README.md` Shell Surface update, `node scripts/check-doc-links.mjs` + `node scripts/check-catalog-docs.mjs` + `boundary-check.sh`, and flip this spec's index row to `Done` with evidence | WB1–WB8 |

---

## 6. Exit criteria

Mapped row-for-row to `docs/ROADMAP.md` §15 "Infra A–D" Exit list, then per-unit:

1. **(ROADMAP) Specs complete with evidence** — Infra A/B/C/D are implemented and
   verified; this spec's index row is `Done` with the evidence below.
2. **(ROADMAP) No TS legality/turn-order inference** — every active/pending/legal
   surface in the new shell frame and setup UI renders Rust/WASM-projected state;
   a no-inference review is recorded (contract §4, §9).
3. **(ROADMAP) Hidden information viewer-safe across every public/browser/replay/
   bot surface** — the Infra D harness passes for every authorized viewer and
   public observer over the full contract-§6 surface set, conforming to ADR 0004.
4. **(ROADMAP) Benchmark/smoke evidence names supported seat counts + max-surface
   fixtures** — Infra D and `simulate` evidence enumerate the seat counts and the
   largest fixtures covered.
5. **(A)** A game can declare and Rust can accept/reject a seat-count range
   deterministically with Rust-owned diagnostics; the catalog payload carries
   seat-range/supported-set/default/label metadata; the browser presents it
   without deciding legality. Existing two-seat games behave identically.
6. **(B)** `simulate` summaries use deterministic seat-keyed maps with stable key
   ordering for ≥1 synthetic N>2 configuration and remain byte-stable (parity
   check) for the existing two-seat games.
7. **(C)** A shared multi-seat shell frame presents seat order, active/pending
   seats, observer mode, and viewer selection from projected state; adopted by
   existing boards or with a recorded board-native exception each.
8. **(D)** The pairwise no-leak harness is reusable, seat-count-parameterized, and
   green for the existing hidden-information games at 2 seats plus synthetic N>2
   fixtures; wired into Rust tests and the web no-leak smoke layer.
9. Gate 0 hygiene (`cargo fmt`/`clippy`/`build`/`test`), `boundary-check.sh`,
   `check-doc-links.mjs`, and `check-catalog-docs.mjs` all pass.

---

## 7. Acceptance evidence

- **Rust:** unit tests for seat-count acceptance/rejection diagnostics (A), seat
  metadata projection (A), seat-keyed summary aggregation determinism (B), and
  the pairwise no-leak harness (D); serialization/golden-trace fixtures unchanged
  for existing games (no schema migration); `cargo fmt/clippy/build/test` clean.
- **Tools:** `simulate` smoke for the existing games (parity) + a synthetic N>2
  run showing seat-keyed output; `fixture-check`, `replay-check`, `rule-coverage`
  for touched games unchanged; `boundary-check.sh` clean (no kernel contamination).
- **Web:** `smoke:wasm`, `smoke:ui`, `smoke:effects` green; new smoke for the seat
  frame (seat rail, active/pending, observer, viewer switch) and the no-leak
  sweep; setup UI presents Rust seat-range validation.
- **No-leak:** Infra D evidence enumerates, per game, the viewer × source-seat ×
  surface assertions that passed, the seat counts covered, and the max-surface
  fixtures used; conforms to ADR 0004 export taxonomy.
- Game-level acceptance is **not applicable** (no game ships in this unit).

---

## 8. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| §2 Rust owns behavior | aligns | Seat-count acceptance/diagnostics (A), summary aggregation (B), active/pending/viewer projection (C), and no-leak derivation (D) stay in Rust/tools; TS presents only. |
| §3 `engine-core` noun-free | aligns | `seat id`/`actor`/`viewer` are already §3-permitted kernel nouns; the forbidden mechanic nouns (`team`/`table`/`pot`/`role`/`faction`/`partnership`) and game-specific seat-metadata types stay in `games/*`/`wasm-api`/`apps/web`. No `game-stdlib` promotion. |
| §5 static data is not behavior | aligns | Seat-range/label/supported-set metadata is typed content; no conditions/selectors/formulas; no YAML/DSL. |
| §11 no hidden-info leak; deterministic serialization/order | aligns | Infra D enforces pairwise no-leak (contract §6); B uses deterministic seat-keyed ordering; no nondeterminism introduced. |
| §11 replay/hash/serialization stay deterministic or explicitly migrated | aligns | No trace/hash migration; existing replay concepts (stable `seats` array order) used consistently (contract §12). |
| §12 stop conditions: TS legality, static-data behavior, hidden-info leak, kernel noun growth | clears | Each is an explicit forbidden change (§9) and a per-ticket stop condition. |
| §13 ADR triggers: visibility contract, replay/hash semantics, WASM-API schema | clears | None tripped — additive viewer-safe metadata only; ADR 0004 already authorizes the export taxonomy Infra D enforces. If any unit is found to require a schema/hash/visibility migration, it stops and routes to an ADR first. |

---

## 9. Forbidden changes

- No `engine-core` edits; no `game-stdlib` additions or helper promotion outside
  the atlas process.
- No TypeScript-computed legality, turn order, or active/pending inference.
- No trace-schema/version/hash/replay-compatibility migration; no
  serialization-order change to existing games.
- No WASM exported-API schema redefinition beyond additive viewer-safe seat
  metadata fields.
- No behavior fields in seat/variant metadata; no YAML; no DSL.
- No roles/teams/partnership modeling (game-local, deferred to a teams gate).
- No change to bot policy, scoring, legality, or effects.
- No weakening or deletion of existing tests (AGENT-DISCIPLINE §4).

---

## 10. Documentation updates required

- `specs/README.md`: collapse active-epoch rows 1–4 (Infra A/B/C/D) into a single
  `15A — Infra A–D` row linking this spec, status `Planned` at authoring →
  `Done` at WB9 with evidence (the row collapse was done in this spec's authoring; see §12 A1–A2).
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §13: note that the simulator-summary
  generalization is delivered (the doc currently defers it to "the later Infra B
  spec"); acceptance-clause edit at WB9 only.
- `docs/TESTING-REPLAY-BENCHMARKING.md`: name the reusable N-player no-leak
  harness as an available verification surface (acceptance clause, WB9).
- `docs/UI-INTERACTION.md`: acceptance clause for the shared multi-seat shell
  frame (seat rail / active-pending / observer / viewer selector), WB9 only.
- `apps/web/README.md`: Shell Surface section gains the seat frame (WB6/WB9).
- No `docs/ROADMAP.md` edit for progress — the index tracker records progress
  (per `specs/README.md` workflow).

---

## 11. Sequencing

- **Predecessor:** Phase 0 (foundation realignment) — `Done`. It produced
  `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, ADR 0007, and the N-seat doc/template
  fields these units implement, and explicitly deferred the four code interlocks
  to "the later Infra A/B/C/D spec(s)" (Phase 0 spec Out-of-scope table).
- **Successor:** Gate 15 (River Ledger / Texas Hold'Em base) — the first official
  N-seat game, which depends on all four interlocks (index Order 5).
- **Admission rule:** Infra A–D is admitted by accepted ADR 0007 and recorded in
  ROADMAP §15. No open mechanic-atlas promotion debt blocks it (atlas §10A is
  empty; the promotion-debt interlock gates the next mechanic-ladder **game**
  gate, not infrastructure units). Internal order is A → B → C → D.

---

## 12. Assumptions (one-line-correctable)

1. **(A1) One combined spec** — assuming a single `15A` spec covering A–D with a
   per-unit work breakdown (user-confirmed); revertible to 2 or 4 specs by
   splitting §5 along unit boundaries.
2. **(A2) Index collapse** — assuming active-epoch rows 1–4 collapse into one
   `15A` row; keep four rows instead if per-unit tracking is preferred.
3. **(A3) N>2 verification via synthetic fixtures** — assuming Infra A–D prove
   N>2 with seat-count-agnostic unit/property tests + synthetic seat fixtures
   (no official >2-seat game exists pre-Gate-15) and full end-to-end N-seat proof
   lands at Gate 15; flag if a throwaway N-seat test game is wanted instead.
4. **(A4) Additive-only metadata/API** — assuming seat metadata is additive
   viewer-safe fields with no WASM-schema/trace/hash migration; any unit that
   needs a migration stops and routes to an ADR first (§8 §13 row).
5. **(A5) race_to_n seat agnosticism** — confirmed: every game's `setup_match`
   already takes a `&[SeatId]` slice and rejects a wrong count via its own
   `setup.rs` `Diagnostic`, so existing games stay byte-identical at two seats
   and are not converted to a new seat model; revisit only if a game's setup
   path is found to resist a count parameter.
6. **(A6) Component naming** — `<SeatFrame>` and harness module names are
   indicative, not binding.
7. **(A7) No external research** — none commissioned; the multi-seat contract +
   ADR 0004 fully ground the infrastructure. Commission `research-brief` only if
   multi-seat UX patterns are wanted for Infra C presentation (deferred to Gate
   15's game-specific UI otherwise).

---

## Outcome

Completed on 2026-06-14.

Infra A-D shipped as one combined 15A infrastructure unit:

- Infra A: Rust/WASM seat-count bridge operations, deterministic seat builders,
  catalog-projected `min_seats`, `max_seats`, `default_seats`,
  `supported_seats`, `seat_labels`, and `viewer_modes`, plus setup/catalog UI
  presentation.
- Infra B: `tools/simulate` now reports deterministic `seat_order` and
  seat-keyed `*_by_seat` maps instead of fixed `seat_0`/`seat_1` scalar summary
  counters.
- Infra C: shared web `SeatFrame` presents catalog seat labels, active/pending
  seat state, observer mode, and viewer selection across live play and replay.
- Infra D: reusable pairwise no-leak harness covers source-seat private tokens
  across bridge surfaces, with synthetic 4-seat max-surface coverage and web
  `SeatFrame` no-leak smoke coverage.

Closeout amendments landed in `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`,
`docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/UI-INTERACTION.md`, and
`apps/web/README.md`. They are acceptance-clause updates only; no FOUNDATIONS
principle, replay/hash semantics, visibility contract, or ADR policy changed.

Exit evidence:

- `cargo test -p wasm-api`
- `cargo test --workspace`
- `cargo run -p simulate -- --game race_to_n --games 1000`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `bash scripts/boundary-check.sh`

Deviations:

- The reusable pairwise no-leak harness applies to current hidden-information
  bridge games with source-seat private tokens. Veiled Draft, Flood Watch, and
  Event Frontier retain explicit no-leak assertions for shared commitments or
  hidden deck/order surfaces, where source-seat private-token pairing is not the
  right proof shape.
