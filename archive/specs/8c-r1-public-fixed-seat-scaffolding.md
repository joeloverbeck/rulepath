# Unit 8C-R1 — C-11 follow-on: public/fixed-seat scaffolding

| Field | Value |
|---|---|
| Spec ID | `8c-r1-public-fixed-seat-scaffolding` |
| Roadmap stage | Public scaling phase — C-11 follow-on retrofit lane |
| Roadmap build gate | 8C-R1 (precedes 8C-R2…R4 and Gate 18) |
| Status | `Done` |
| Date | 2026-06-23 |
| Owner | Rulepath maintainers; implementation delegated through bounded `AGENT-TASK` packets |

This spec is an implementation plan. It is subordinate to the foundation set in
[`../docs/README.md`](../docs/README.md) and MUST NOT redefine any foundation
contract. Where this spec and a foundation document disagree, the foundation
document wins. Authority order for this spec:
[`../docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md),
[`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
[`../docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md),
accepted ADRs and area contracts,
[`../docs/ROADMAP.md`](../docs/ROADMAP.md), then this spec, then tickets.

## 1. Determination

### 1.1 Locked determination

The next-unit determination is confirmed, not reopened:

1. The active-epoch tracker records Unit `8C` as `Done`; row `8C-R1` is the lowest row still `Not started`. Rows `8C-R2`, `8C-R3`, `8C-R4`, and Gate 18 follow it. Gate 18 is explicitly pending closure, accepted not-applicability, or accepted exception of all four C-11 waves. The same document requires choosing the lowest non-`Done` unit and then using `/reassess-spec` and `/spec-to-tickets` after the spec is accepted. See [`specs/README.md`](../specs/README.md).
2. The mechanic atlas open-promotion-debt register says `Current debt: None`, last reviewed at Gate 17 closeout. No primitive-promotion debt precedes this unit, and this code-scaffolding retrofit is not itself a mechanic-ladder gate. See [`docs/MECHANIC-ATLAS.md`](../docs/MECHANIC-ATLAS.md) §10A.
3. The completed parent spec seeds exactly this wave, creates the `8C-R1…R4` tracker rows in work item `8C-030`, and fixes sequencing through EC-28 and EC-30: every official game is covered exactly once by four bounded C-11 seeds, and Gate 18 remains after those waves are closed, declared not applicable, or explicitly excepted. See [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`](../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md) §5 and exit criteria EC-28/EC-30.

**Determination:** Unit `8C-R1` is the required next spec. The bounded game set is exactly `race_to_n`, `draughts_lite`, `three_marks`, `column_four`, `directional_flip`, and `token_bazaar`. This spec does not admit work from later C-11 waves or Gate 18.

## 2. Objective

Within the public-scaling sequence recorded by [`docs/ROADMAP.md`](../docs/ROADMAP.md) and operationalized by the parent C-11 seed, Unit 8C-R1 converts the first follow-on wave into a bounded, reviewable retrofit plan for six public, fixed-two-seat games. It must:

1. inventory every applicable C-01…C-05 and C-08 surface in the six games;
2. resolve every audited surface to exactly one accepted verdict: **migrate**, **not applicable**, **accepted exception with a named review trigger**, or **already discharged by the Unit 8C pilot**;
3. adopt only behavior-free shared scaffolding already accepted and shipped by Unit 8C;
4. preserve legality, setup semantics, state transitions, visibility, projection, effects, scoring, outcomes, bot choices, and static-data meaning;
5. perform every byte-, hash-, seat-ID-, replay-, export-, or visibility-bearing change as a named ADR-0009 per-surface migration with characterization, parallel/compatibility treatment, before-and-after evidence, and a one-surface rollback point; and
6. leave no unnamed “remaining cleanup” bucket.

The result is a complete first-wave adoption ledger and implementation packet set—not a new game, mechanic, UI feature, or architecture redesign. This sharp delta builds on the already-landed C-01…C-10 infrastructure, `game-test-support`, register entries `MSC-8C-001…010`, the six Unit 8C pilots, and the previously landed `game-stdlib::board_space` back-port. It does not re-propose those assets as missing.

## 3. Scope

### 3.1 In scope

The unit covers only these games and helper families:

- **Games:** `race_to_n`, `draughts_lite`, `three_marks`, `column_four`, `directional_flip`, `token_bazaar`.
- **Primary helper audit:** C-01 effect-envelope constructors; C-02 canonical seat grammar plus WASM import/output boundaries; C-03 seat-count validation and applicable ring-index plumbing; C-04 action-tree encoding/hash v1; C-05 `StableBytesWriter` v1 adoption for the selected action-tree surface; C-08 evidence-profile drivers.
- **Checkpoint-only audit:** C-06 test-support dependency discipline; C-07 no-leak geometry; C-09 unbiased RNG applicability; C-10 non-promotion compliance.
- **Governance:** per-surface characterization, ADR-0009 migration receipts, mechanical-scaffolding register updates, and the `specs/README.md` status transition.

### 3.2 Primary applicability and verdict matrix

Verdict meanings:

- **`migrate`** — one named surface changes in one reviewable diff under the protocol in §5.1.
- **`not-applicable`** — the game has no real surface of that class; rationale is recorded.
- **`exception`** — a real surface remains legacy/local under a named owner, compatibility statement, rollback boundary, and next review trigger.
- **`already-discharged-by-8C-pilot`** — Unit 8C already implemented and evidenced that exact named surface; R1 verifies the receipt but does not rebuild it.

| Game | C-01 envelope constructor | C-02 seat grammar/output | C-03 seat count/ring | C-04 action-tree v1 | C-05 writer v1 | C-08 profile drivers |
|---|---|---|---|---|---|---|
| `race_to_n` | `already-discharged-by-8C-pilot` | `migrate` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` |
| `draughts_lite` | `migrate` | `migrate` | `migrate` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `migrate` |
| `three_marks` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `column_four` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `directional_flip` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `token_bazaar` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |

The matrix verdict is the aggregate for the helper family. The sub-surface tables below are authoritative when a helper contains both a migration and an exception.

### 3.3 C-01 — effect-envelope constructor surfaces

The exact selected surface is each game's local `public_effect` constructor. The Race pilot already calls `EffectEnvelope::public`. The other five construct the same public envelope with a literal and may migrate without changing payload, ordering, or visibility.

| Game | Exact site | Verdict | Required semantic result |
|---|---|---|---|
| `race_to_n` | [`games/race_to_n/src/effects.rs`](../games/race_to_n/src/effects.rs) `public_effect` | `already-discharged-by-8C-pilot` | Verify the existing `EffectEnvelope::public(payload)` adoption; no code change. |
| `draughts_lite` | [`games/draughts_lite/src/effects.rs`](../games/draughts_lite/src/effects.rs) `public_effect` | `migrate` | Replace only the equivalent `VisibilityScope::Public` literal with `EffectEnvelope::public`; preserve effect order and payload. |
| `three_marks` | [`games/three_marks/src/effects.rs`](../games/three_marks/src/effects.rs) `public_effect` | `migrate` | Same bounded replacement. |
| `column_four` | [`games/column_four/src/effects.rs`](../games/column_four/src/effects.rs) `public_effect` | `migrate` | Same bounded replacement. |
| `directional_flip` | [`games/directional_flip/src/effects.rs`](../games/directional_flip/src/effects.rs) `public_effect` | `migrate` | Same bounded replacement. |
| `token_bazaar` | [`games/token_bazaar/src/effects.rs`](../games/token_bazaar/src/effects.rs) `public_effect` | `migrate` | Same bounded replacement. |

No private-envelope migration is applicable: these six games expose public effects for the audited paths. C-07/hidden-information work is not inferred from the existence of `EffectEnvelope`.

### 3.4 C-02 — canonical seat grammar and compatibility surfaces

Canonical Rust identity is `seat_<zero-based>`. Legacy hyphen and symbolic aliases remain import-only at the WASM boundary. TypeScript must not normalize or repair seat IDs.

| Game | Typed Rust parser | WASM import aliases | WASM replay-document output | Native `default_seats` and pre-existing non-WASM trace spellings |
|---|---|---|---|---|
| `race_to_n` | `already-discharged-by-8C-pilot`: `RaceSeat::parse` delegates to `SeatId::parse_canonical` | `already-discharged-by-8C-pilot`: `parse_seat_import` accepts bounded canonical, hyphen, and symbolic aliases | `migrate`: canonicalize the one game-specific exported document and its `wasm-exported.trace.json` | `exception`: retain legacy-readable native inputs and committed non-WASM traces; trigger = a separately admitted native replay/trace seat-surface migration |
| `draughts_lite` | `migrate`: `DraughtsLiteSeat::parse` | `already-discharged-by-8C-pilot` | `migrate` | `exception` with the same trigger |
| `three_marks` | `migrate`: `ThreeMarksSeat::parse` | `already-discharged-by-8C-pilot` | `migrate` | `exception` with the same trigger |
| `column_four` | `migrate`: `ColumnFourSeat::parse` | `already-discharged-by-8C-pilot` | `migrate` | `exception` with the same trigger |
| `directional_flip` | `migrate`: `DirectionalFlipSeat::parse` | `already-discharged-by-8C-pilot` | `migrate` | `exception` with the same trigger |
| `token_bazaar` | `migrate`: `TokenBazaarSeat::parse` | `already-discharged-by-8C-pilot`; `trace_token_seat` is already canonical | `migrate`: the roster/document still emits legacy hyphen spellings | `exception` with the same trigger |

C-02 output migration constraints:

- `crates/wasm-api/src/seats.rs::seats()` and `seats_for_count()` remain unchanged because they feed legacy setup/replay surfaces whose byte and hash consequences exceed a single output diff.
- Add or expose an **output-only canonical roster/formatter surface** in [`crates/wasm-api/src/seats.rs`](../crates/wasm-api/src/seats.rs) while retaining the import adapter.
- Flip one `*_replay_document_json` function at a time in `crates/wasm-api/src/games/{race,draughts,three,column,directional,token}.rs`.
- For each game, only that game's `tests/golden_traces/wasm-exported.trace.json` may change in the same diff. All other traces remain byte-identical.
- Canonicalize only the seat-ID-bearing fields (roster, command actors, outcome seats) of the exported document. Some exported documents already contain incidental non-seat tokens that happen to be `seat_<n>`-shaped (e.g. Directional Flip and Token Bazaar `wasm-exported.trace.json` already carry both hyphen and underscore spellings); 8C-R1-002 must pin which fields are seat IDs, and the migration MUST leave every non-seat-ID byte unchanged.
- Old canonical-underscore, legacy-hyphen, and bounded symbolic imports remain readable through the existing Rust WASM adapter during the compatibility window.

### 3.5 C-03 — seat-count and ring-index surfaces

| Game | Exact-count validation | Ring/index geometry | Verdict rationale |
|---|---|---|---|
| `race_to_n` | `already-discharged-by-8C-pilot`: `SeatCountRange::inclusive(...).validate(seats.len())` | `not-applicable` | Fixed two-seat turn changes use the typed game-local `other()` mapping; no raw modulo/ring drift exists. |
| `draughts_lite` | `migrate` in `setup_match` | `not-applicable` | Replace only the hand-written exact-count predicate; preserve the game's diagnostic mapping and typed `other()`. |
| `three_marks` | `migrate` in `setup_match` | `not-applicable` | Same. |
| `column_four` | `migrate` in `setup_match` | `not-applicable` | Same. |
| `directional_flip` | `migrate` in `setup_match` | `not-applicable` | Same. |
| `token_bazaar` | `migrate` in `setup_match`; add normal `game-stdlib` dependency | `not-applicable` | Same; no generic index conversion is introduced. |

A helper adoption must preserve `invalid_seat_count` code, public message, accepted count, ordering, and setup state. Replacing a clear fixed-two typed enum mapping with generic `usize` ring arithmetic is prohibited churn, not a C-03 success.

### 3.6 C-04/C-05 — action-tree v1 and adjacent stable-byte surfaces

The selected C-05 adoption surface is **only the action-tree v1 encoding** owned by `engine-core::ActionTree::stable_bytes(ActionTreeEncodingVersion::V1)`. It is not authority to migrate every stable-serialization surface in a game.

| Game | Parallel action-tree v1 surface | Existing legacy `action_tree_hash` | Verdict |
|---|---|---|---|
| `race_to_n` | Present: `action_tree_v1_bytes` / `action_tree_v1_hash` | Retained for compatibility | `already-discharged-by-8C-pilot`; compatibility exception remains explicit |
| `draughts_lite` | Present: `action_tree_v1_bytes` / `action_tree_v1_hash` | Retained for compatibility | `already-discharged-by-8C-pilot`; compatibility exception remains explicit |
| `three_marks` | Add parallel wrappers over `ActionTreeEncodingVersion::V1` | Retain byte-for-byte; do not rewrite `ReplayHashes.action_tree_hash` | `migrate` + legacy compatibility `exception` |
| `column_four` | Add parallel wrappers | Retain byte-for-byte | `migrate` + legacy compatibility `exception` |
| `directional_flip` | Add parallel wrappers, including metadata/tags/preview already present in the engine contract | Retain byte-for-byte | `migrate` + legacy compatibility `exception` |
| `token_bazaar` | Add parallel wrappers, including metadata ordering | Retain byte-for-byte | `migrate` + legacy compatibility `exception` |

The legacy action-tree hash remains the field asserted by existing trace schemas in R1. A future authority flip requires its own named surface migration; R1 does not silently make v1 authoritative inside legacy `expected_action_tree_hashes`.

Adjacent C-05 surfaces are explicitly classified rather than silently omitted:

| Stable-byte/hash surface | Games | Verdict in R1 | Owner and next review trigger |
|---|---|---|---|
| State snapshot bytes/hash | all six | `exception` | Game-local replay/serialization owner. Reopen only under a dedicated state-surface version migration or semantic schema change. |
| Effect bytes/hash | all six | `exception` | Game-local effect serialization. Reopen only under a dedicated effect-surface migration; C-01 constructor adoption must prove unchanged bytes. |
| Public-view bytes/hash | all six | `exception` | Game-local projection/serialization. Reopen only under a dedicated public-view schema migration. |
| Replay/export bytes/hash | all six, including Token Bazaar public export | `exception` | Native replay or Rust/WASM export owner. C-08 may validate profile metadata around the existing surface but does not change canonical bytes. Reopen under a named replay/export version migration. |
| Dedicated diagnostic hash/bytes | `race_to_n`, `draughts_lite`, `three_marks` | `exception` | Game-local diagnostics. Reopen under a diagnostic-surface migration. |
| Dedicated diagnostic hash/bytes | `column_four`, `directional_flip`, `token_bazaar` | `not-applicable` | No distinct dedicated diagnostic hash field was found in the inspected replay-support surface; diagnostic traces remain replay evidence, not a separate C-05 target. |

### 3.7 C-08 — evidence-profile driver matrix

R1 follows the Race pilot precedent: it constructs a parallel typed profile adapter in tests around existing legacy evidence and explicitly preserves original trace/fixture bytes unless another separately named migration in this spec owns those bytes. Filename suffixes do not infer profile class.

| Game | `replay-command-v1` | `setup-evidence-v1` | `public-export-v1` | `seat-private-export-v1` | `domain-evidence-v1` |
|---|---|---|---|---|---|
| `race_to_n` | `already-discharged-by-8C-pilot` | `not-applicable`: no setup fixture | `not-applicable`: no distinct public-export profile in this unit | `not-applicable`: perfect information/no seat-private export | `not-applicable`: no domain fixture |
| `draughts_lite` | `migrate` | `migrate`: standard setup fixture | `not-applicable` | `not-applicable` | `not-applicable` |
| `three_marks` | `migrate` | `not-applicable`: no setup fixture | `not-applicable` | `not-applicable` | `not-applicable` |
| `column_four` | `migrate` | `not-applicable`: its standard fixture is command/terminal/diagnostic/bot evidence, not setup evidence | `not-applicable` | `not-applicable` | `not-applicable` |
| `directional_flip` | `migrate` | `migrate`: standard setup fixture | `not-applicable` | `not-applicable` | `not-applicable` |
| `token_bazaar` | `migrate` | `migrate`: standard setup fixture | `migrate`: actual `PublicReplayExport` round trip and public-export fixture surface | `not-applicable`: observer and seat views are identical | `not-applicable` |

Rules for all C-08 tasks:

- `game-test-support` is a `[dev-dependencies]` edge only.
- `ReplayCommandV1Driver`, `SetupEvidenceV1Driver`, and `PublicExportV1Driver` validate metadata and delegate to game/tool-owned behavior; they do not replay, set up, project, export, or decide rules themselves.
- `replay-check` and `fixture-check` remain validator owners. Tool changes are allowed only if reassessment proves a missing thin registration/dispatch seam; game behavior may not move into a tool.
- The C-02 WASM-output task is the only default owner of the six `wasm-exported.trace.json` byte changes. C-08 tests must not opportunistically rewrite them.

### 3.8 C-06/C-07/C-09/C-10 checkpoints

| Checkpoint | Verdict | Required record |
|---|---|---|
| **C-06 dev-only test-support crate** | Infrastructure already shipped. `race_to_n` already uses it; the other five may add it only as a dev dependency for C-08. | `cargo tree` evidence proves no normal/build path from production, WASM, tools, or game libraries. |
| **C-07 pairwise no-leak geometry** | `not-applicable` for this six-game public/perfect-information wave. | Record that no hidden source datum or seat-private viewer pair exists. Preserve existing public visibility tests; do not delete them. |
| **C-09 unbiased RNG** | `not-applicable` for R1. | No characterized game-rule bounded-index sampling surface requires migration. Bot RNG and any existing RNG consumption remain unchanged. No sampler adoption, seed change, shuffle change, or new RNG vector. |
| **C-10 non-promotion affirmation** | Audit checkpoint only. | Confirm every change stays on the register's Non-Promotion side: no legality, setup policy, reveal, projection, scoring, outcome, bot, or diagnostic policy moves into shared code. |

### 3.9 Out of scope

- Any seventh game.
- Work assigned to `8C-R2`, `8C-R3`, `8C-R4`, or Gate 18.
- A new game, rules change, mechanic primitive, UI feature, bot strategy, benchmark policy, or catalog entry.
- Rebuilding C-01…C-10 infrastructure, the `game-test-support` crate, or already-landed Unit 8C pilots.
- Reworking the Gate 7.1 `board_space` back-port.
- Flipping all legacy action-tree hashes to v1 or replacing Trace Schema v1 wholesale.
- Migrating state, effect, view, replay/export, and diagnostic stable bytes as one sweep.
- General fixture-schema consolidation, filename normalization, or mass profile-field insertion.
- Changing `HashValue::from_stable_bytes` or adopting a general serializer as Rulepath hash authority.
- Browser/catalog work, including `apps/web/README.md`.
- Benchmark threshold changes. Runtime benchmarks are `not applicable` unless implementation reveals a measurable regression risk that must be separately admitted.

### 3.10 Not allowed

- Mechanic or game nouns in `engine-core`.
- Behavioral promotion through the mechanical-scaffolding lane.
- Shared helpers that decide legality, setup semantics, reveal timing, authorization, projection/redaction, scoring, outcomes, bot choices, or diagnostic prose.
- Silent changes to bytes, hashes, trace meaning, export meaning, seat IDs, visibility, serialization order, or RNG consumption.
- Blanket golden regeneration, `update snapshots`, or accepting new expected hashes merely because files were regenerated.
- Changing `TRACE-SCHEMA-v1` bytes without the exact ADR-0009 surface packet that owns the change.
- Making `game-test-support` a normal/build dependency.
- TypeScript seat normalization, rule enforcement, or data repair.
- YAML, a DSL, selectors, conditions, triggers, scripts, loops, formulas, or procedural mutation instructions in data.
- Deleting, weakening, ignoring, narrowing, or replacing specific tests merely to get green output.

## 4. Deliverables

### 4.1 Concrete artifact tree

The implementation defaults to the following bounded tree. Reassessment may correct a test-module or helper name, but it must not expand ownership or merge multiple migration surfaces into one diff.

```text
# Unit evidence and governance
reports/8c-r1-public-fixed-seat-scaffolding-characterization.md   # new

docs/MECHANICAL-SCAFFOLDING-REGISTER.md
specs/README.md
specs/8c-r1-public-fixed-seat-scaffolding.md                      # eventual accepted spec

# C-01 — five local constructor adoptions
games/draughts_lite/src/effects.rs
games/three_marks/src/effects.rs
games/column_four/src/effects.rs
games/directional_flip/src/effects.rs
games/token_bazaar/src/effects.rs

# C-02 — strict game parsers
games/draughts_lite/src/ids.rs
games/three_marks/src/ids.rs
games/column_four/src/ids.rs
games/directional_flip/src/ids.rs
games/token_bazaar/src/ids.rs

# C-02 — output-only canonical WASM replay documents
crates/wasm-api/src/seats.rs
crates/wasm-api/src/games/race.rs
crates/wasm-api/src/games/draughts.rs
crates/wasm-api/src/games/three.rs
crates/wasm-api/src/games/column.rs
crates/wasm-api/src/games/directional.rs
crates/wasm-api/src/games/token.rs
crates/wasm-api/src/tests.rs

games/race_to_n/tests/golden_traces/wasm-exported.trace.json
games/draughts_lite/tests/golden_traces/wasm-exported.trace.json
games/three_marks/tests/golden_traces/wasm-exported.trace.json
games/column_four/tests/golden_traces/wasm-exported.trace.json
games/directional_flip/tests/golden_traces/wasm-exported.trace.json
games/token_bazaar/tests/golden_traces/wasm-exported.trace.json

# C-03 — exact-count validation
games/draughts_lite/src/setup.rs
games/three_marks/src/setup.rs
games/column_four/src/setup.rs
games/directional_flip/src/setup.rs
games/token_bazaar/src/setup.rs
games/token_bazaar/Cargo.toml

# C-04/C-05 — parallel action-tree v1 wrappers/evidence
games/three_marks/src/replay_support.rs
games/three_marks/tests/serialization_tests.rs
games/column_four/src/replay_support.rs
games/column_four/tests/replay.rs
games/directional_flip/src/replay_support.rs
games/directional_flip/tests/replay.rs
games/token_bazaar/src/replay_support.rs
games/token_bazaar/tests/serialization.rs

# C-08 — dev-only profile-driver adoption
games/draughts_lite/Cargo.toml
games/draughts_lite/tests/replay.rs
games/three_marks/Cargo.toml
games/three_marks/tests/replay_tests.rs
games/column_four/Cargo.toml
games/column_four/tests/replay.rs
games/directional_flip/Cargo.toml
games/directional_flip/tests/replay.rs
games/token_bazaar/Cargo.toml
games/token_bazaar/tests/replay.rs
```

### 4.2 Required characterization report

`reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` must be append-only during the unit and contain:

- the final primary and sub-surface verdict matrices;
- exact legacy seat spellings by import, native replay, WASM output, and committed trace surface;
- per-game representative legacy action-tree bytes/hash and, where present or added, v1 bytes/hash;
- the six selected `wasm-exported.trace.json` before/after byte and hash receipts;
- proof that non-selected traces and fixtures are byte-identical;
- profile classification, validator owner, visibility class, byte authority, and source artifact for each C-08 adoption;
- C-06/C-07/C-09/C-10 applicability conclusions;
- every accepted exception's owner, risk, compatibility window, rollback, and next trigger; and
- the exact command transcript required by §7.

### 4.3 Required register receipts

Update [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`](../docs/MECHANICAL-SCAFFOLDING-REGISTER.md) under the relevant `MSC-8C-001…010` entries—or add bounded R1 adoption receipts using the register schema—to record each migration, not-applicability decision, and accepted exception. The receipt must identify the game, exact surface, decision state, evidence link, hash/visibility impact, rollback, and next review trigger.

No foundation document or ADR amendment is expected. If implementation proves one necessary, the task stops and records the §8.4 ADR trigger rather than editing doctrine as routine closeout.

## 5. Work breakdown

### 5.1 Protocol for every candidate `AGENT-TASK`

Every item below is a candidate `Task profile: scaffold-refactor` packet under [`templates/AGENT-TASK.md`](../templates/AGENT-TASK.md) and [`docs/AGENT-DISCIPLINE.md`](../docs/AGENT-DISCIPLINE.md). Each packet must include:

1. accepted authority (`ADR 0008`, `ADR 0009`, the relevant `MSC-8C-*` entry, this spec);
2. exact duplicate/adoption sites and exact symbols;
3. affected hash, trace, fixture, seat, visibility, and export surfaces, including explicit `not applicable` rationale;
4. a characterization test or report row **before** migration;
5. exactly one selected surface per reviewable diff;
6. ADR-0009 classification: `unchanged`, `parallel-new`, or `intentional-migration`;
7. compatibility window and reader behavior;
8. exact before/after evidence;
9. rollback scope no wider than the selected surface; and
10. the failing-test protocol: validate the test, locate fault in SUT or test, fix, add regression proof, and never weaken coverage.

A task whose only evidence is “tests pass” or a broad golden update is invalid.

### 5.2 Wave A — admission and characterization

| Task | Dependencies | Exact targets | Required result; affected surfaces; rollback |
|---|---|---|---|
| **8C-R1-001 — Freeze determination and inventory** | None | `specs/README.md`; `docs/MECHANIC-ATLAS.md`; parent spec; new characterization report | Record the locked next-unit proof, exact six-game list, helper matrix, atlas debt `None`, and parent EC-28/EC-30 mapping. No code/hash/visibility change. Rollback: delete only the unaccepted report draft. |
| **8C-R1-002 — Characterize current surfaces** | 001 | All six `src/{effects,ids,setup,replay_support}.rs`; manifests/fixtures; committed golden traces; `crates/wasm-api/src/seats.rs`; six WASM game bridges | Pin current constructor shapes, parser spellings, exact-count diagnostics, legacy/v1 action-tree hashes, profile classes, C-09 RNG vectors (`not applicable` where absent), and byte digests of all candidate artifacts. No migration may begin until every matrix cell and exception row has an owner. Rollback: report-only. |

### 5.3 Wave B — C-01 public-envelope constructor adoptions

Each task is classified `unchanged`: effect payload, visibility, order, and resulting effect hash must match before/after. Golden files are preserved.

| Task | Dependency | Exact file/symbol | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-101 — Draughts C-01** | 002 | `games/draughts_lite/src/effects.rs::public_effect` | Replace the literal with `EffectEnvelope::public`. Run focused effect tests and all Draughts replay traces; effect/public-view hashes unchanged. Rollback: one function. |
| **8C-R1-102 — Three Marks C-01** | 002 | `games/three_marks/src/effects.rs::public_effect` | Replace only the public literal; effect bytes/hash and public visibility must be unchanged. Rollback: this function and its focused assertion. |
| **8C-R1-103 — Column Four C-01** | 002 | `games/column_four/src/effects.rs::public_effect` | Replace only the public literal; effect bytes/hash and public visibility must be unchanged. Rollback: this function and its focused assertion. |
| **8C-R1-104 — Directional Flip C-01** | 002 | `games/directional_flip/src/effects.rs::public_effect` | Replace only the public literal; effect bytes/hash and public visibility must be unchanged. Rollback: this function and its focused assertion. |
| **8C-R1-105 — Token Bazaar C-01** | 002 | `games/token_bazaar/src/effects.rs::public_effect` | Replace only the public literal; effect bytes/hash, public visibility, and public-export effect bytes must be unchanged. Rollback: this function and its focused assertion. |

### 5.4 Wave C — C-02 strict canonical parsers

Each parser task is `unchanged` for accepted canonical input. It must add direct rejection tests for malformed or out-of-range spellings at the game parser while proving the WASM import adapter still accepts bounded legacy aliases. No trace or golden file changes are allowed in this wave.

| Task | Dependency | Exact file/symbol | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-201 — Draughts parser** | 002 | `games/draughts_lite/src/ids.rs::DraughtsLiteSeat::parse` | Delegate canonical parsing to `SeatId::parse_canonical` plus bounded enum mapping. Preserve `as_str()`. Hash/visibility unchanged. Rollback: parser and its unit tests. |
| **8C-R1-202 — Three Marks parser** | 002 | `games/three_marks/src/ids.rs::ThreeMarksSeat::parse` | Canonical accepted values and all downstream hashes/visibility remain unchanged; malformed labels reject locally while WASM aliases remain import-only. Rollback: parser plus focused parser tests. |
| **8C-R1-203 — Column Four parser** | 002 | `games/column_four/src/ids.rs::ColumnFourSeat::parse` | Canonical accepted values and all downstream hashes/visibility remain unchanged; malformed labels reject locally while WASM aliases remain import-only. Rollback: parser plus focused parser tests. |
| **8C-R1-204 — Directional Flip parser** | 002 | `games/directional_flip/src/ids.rs::DirectionalFlipSeat::parse` | Canonical accepted values and all downstream hashes/visibility remain unchanged; malformed labels reject locally while WASM aliases remain import-only. Rollback: parser plus focused parser tests. |
| **8C-R1-205 — Token Bazaar parser** | 002 | `games/token_bazaar/src/ids.rs::TokenBazaarSeat::parse` | Canonical accepted values and all downstream hashes/public-export visibility remain unchanged; malformed labels reject locally while WASM aliases remain import-only. Rollback: parser plus focused parser tests. |

### 5.5 Wave D — C-02 output-only canonical WASM migration

This wave owns the only default golden-byte changes in R1. The classification is `parallel-new` for the output formatter helper and `intentional-migration` for each game document. The compatibility window keeps old imports readable indefinitely unless a later accepted spec narrows it.

| Task | Dependencies | Exact files/symbols | Affected surface, evidence, rollback |
|---|---|---|---|
| **8C-R1-210 — Canonical output helper** | 002 | `crates/wasm-api/src/seats.rs`: add an output-only canonical seat/roster helper (default symbols `canonical_trace_seat_id` / `canonical_seats_for_count`); characterize existing `trace_*_seat` functions read-only; local tests | Add a parallel output formatter based on canonical Rust IDs. Do not change any existing `trace_*_seat` return value, `parse_seat_import`, `seats`, or `seats_for_count` in this task. No game document flips and no golden changes. Rollback: new helper/tests only. |
| **8C-R1-211 — Race WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_race_seat`; `crates/wasm-api/src/games/race.rs::race_replay_document_json`; `crates/wasm-api/src/tests.rs`; `games/race_to_n/tests/golden_traces/wasm-exported.trace.json` | Canonicalize roster, command actors, and outcome seats emitted by this document. Characterize complete document bytes before/after; expected gameplay hashes remain semantically valid. Update only this golden file with migration note. Rollback: one bridge function + one trace. |
| **8C-R1-212 — Draughts WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_draughts_seat`; `crates/wasm-api/src/games/draughts.rs::draughts_replay_document_json`; shared tests; Draughts `wasm-exported.trace.json` | Intentional seat/output-byte migration only; public visibility and gameplay hashes remain valid, old hyphen documents still import. Rollback: this trace helper, bridge function, focused tests, and one golden file. |
| **8C-R1-213 — Three Marks WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_three_seat`; `crates/wasm-api/src/games/three.rs::three_replay_document_json`; shared tests; Three Marks `wasm-exported.trace.json` | Intentional seat/output-byte migration only; public visibility and gameplay hashes remain valid, old hyphen documents still import. Rollback: this trace helper, bridge function, focused tests, and one golden file. |
| **8C-R1-214 — Column Four WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_column_seat`; `crates/wasm-api/src/games/column.rs::column_replay_document_json`; shared tests; Column Four `wasm-exported.trace.json` | Intentional seat/output-byte migration only; public visibility and gameplay hashes remain valid, old hyphen documents still import. Rollback: this trace helper, bridge function, focused tests, and one golden file. |
| **8C-R1-215 — Directional Flip WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_directional_seat`; `crates/wasm-api/src/games/directional.rs::directional_replay_document_json`; shared tests; Directional Flip `wasm-exported.trace.json` | Intentional seat/output-byte migration only; public visibility and gameplay hashes remain valid, old hyphen documents still import. Rollback: this trace helper, bridge function, focused tests, and one golden file. |
| **8C-R1-216 — Token Bazaar WASM output** | 210 | `crates/wasm-api/src/seats.rs::trace_token_seat` (verify already canonical); `crates/wasm-api/src/games/token.rs::token_replay_document_json`; shared tests; Token Bazaar `wasm-exported.trace.json` | Intentional roster/output-byte migration only; preserve public visibility, `PublicReplayExport` semantics, and expected public-export/gameplay hash authority. Rollback: bridge function, focused tests, and one golden file; the already-canonical trace helper should remain unchanged. |

A review must reject a diff that also changes `default_seats`, non-WASM traces, game state bytes, or another game's WASM document.

### 5.6 Wave E — C-03 exact-count validation

Each task is classified `unchanged`. The shared helper may decide only count/range validity; the game continues to own setup options, diagnostics, ordering, and state construction.

| Task | Dependency | Exact file/symbol | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-301 — Draughts exact count** | 002 | `games/draughts_lite/src/setup.rs::setup_match` | Replace only `seats.len() != options.variant.seat_count as usize` with `SeatCountRange::inclusive(...).validate`. Preserve `invalid_seat_count` diagnostic bytes. Rollback: one validation block. |
| **8C-R1-302 — Three Marks exact count** | 002 | `games/three_marks/src/setup.rs::setup_match` | Adopt exact-count validation; setup state, `invalid_seat_count` bytes, all replay hashes, and public visibility remain unchanged. Rollback: one validation block plus focused tests. |
| **8C-R1-303 — Column Four exact count** | 002 | `games/column_four/src/setup.rs::setup_match` | Adopt exact-count validation; setup state, `invalid_seat_count` bytes, all replay hashes, and public visibility remain unchanged. Rollback: one validation block plus focused tests. |
| **8C-R1-304 — Directional Flip exact count** | 002 | `games/directional_flip/src/setup.rs::setup_match` | Adopt exact-count validation; setup state, `invalid_seat_count` bytes, all replay hashes, and public visibility remain unchanged. Rollback: one validation block plus focused tests. |
| **8C-R1-305 — Token Bazaar exact count** | 002 | `games/token_bazaar/Cargo.toml`; `games/token_bazaar/src/setup.rs::setup_match` | Add normal `game-stdlib` dependency and adopt exact-count validation. Prove no additional production dependency and no behavior/hash change. Rollback: dependency line + validation block. |

### 5.7 Wave F — C-04/C-05 parallel action-tree v1 surfaces

Each task is `parallel-new`. Existing `action_tree_hash`, `ReplayHashes.action_tree_hash`, committed trace expectations, and legacy bytes stay untouched. The new v1 surface must directly delegate to `ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)` and receive a focused pinned receipt. No manual reimplementation of the RPSB framing is allowed in a game.

| Task | Dependency | Exact files/symbols | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-401 — Three Marks action-tree v1** | 002 | `games/three_marks/src/replay_support.rs::action_tree_hash`; add `action_tree_v1_bytes` / `action_tree_v1_hash`; `tests/serialization_tests.rs` | Pin representative legacy hash, new v1 bytes/hash, order, and inequality where expected. All existing traces unchanged. Rollback: new wrappers/test only. |
| **8C-R1-402 — Column Four action-tree v1** | 002 | `games/column_four/src/replay_support.rs::action_tree_hash`; add `action_tree_v1_bytes` / `action_tree_v1_hash`; `tests/replay.rs` or reassessed serialization owner | Add only the parallel engine-v1 byte/hash surface; pin order/framing while preserving the legacy action-tree hash, all committed trace bytes, and public visibility. Rollback: new wrappers and one focused test. |
| **8C-R1-403 — Directional Flip action-tree v1** | 002 | `games/directional_flip/src/replay_support.rs::action_tree_hash`; add v1 wrappers; `tests/replay.rs` | Prove segment, label, accessibility label, metadata order, tag order, preview, child structure, and freshness framing are covered by engine v1. Legacy trace hash unchanged. |
| **8C-R1-404 — Token Bazaar action-tree v1** | 002 | `games/token_bazaar/src/replay_support.rs::action_tree_hash`; add v1 wrappers; `tests/serialization.rs` | Prove action segment and metadata order in v1; preserve legacy and public-export hashes. |

Race and Draughts receive report/register verification only for this helper family because their named Unit 8C pilot surfaces are already discharged.

### 5.8 Wave G — C-08 replay-command profile drivers

These tasks use the shipped Race pattern: parse existing legacy trace evidence, construct a typed `ProfileArtifact`, validate with `ReplayCommandV1Driver`, and delegate replay to the existing game test. They do not insert `profile_id` fields into the committed trace by default.

| Task | Dependency | Exact files/symbols | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-501 — Draughts replay profile** | 002 | `games/draughts_lite/Cargo.toml` `[dev-dependencies]`; `games/draughts_lite/tests/replay.rs`; representative committed command trace | Add dev-only `game-test-support`; validate `replay-command-v1` metadata with owner `replay-check`, then execute existing replay/hash assertions. Trace bytes unchanged. Rollback: dev edge + one test. |
| **8C-R1-502 — Three Marks replay profile** | 002 | `games/three_marks/Cargo.toml`; `games/three_marks/tests/replay_tests.rs`; representative committed command trace | Add only a dev dependency and parallel internal-dev profile adapter; trace bytes, state/effect/action-tree/view/diagnostic hashes, and public visibility remain unchanged. Rollback: dev edge plus one focused test. |
| **8C-R1-503 — Column Four replay profile** | 002 | `games/column_four/Cargo.toml`; `games/column_four/tests/replay.rs`; representative committed command trace | Add only a dev dependency and parallel internal-dev profile adapter; trace bytes and all declared replay/public-view hashes remain unchanged. Rollback: dev edge plus one focused test. |
| **8C-R1-504 — Directional Flip replay profile** | 002 | `games/directional_flip/Cargo.toml`; `games/directional_flip/tests/replay.rs`; representative committed command trace | Add only a dev dependency and parallel internal-dev profile adapter; trace bytes, preview-bearing action-tree evidence, hashes, and public visibility remain unchanged. Rollback: dev edge plus one focused test. |
| **8C-R1-505 — Token Bazaar replay profile** | 002 | `games/token_bazaar/Cargo.toml`; `games/token_bazaar/tests/replay.rs`; representative committed command trace | Add only a dev dependency and parallel internal-dev replay profile adapter, kept separate from the public-export profile. Trace/export bytes, declared hashes, and public visibility remain unchanged. Rollback: dev edge plus one focused replay-profile test. |

### 5.9 Wave H — C-08 setup and public-export profile drivers

| Task | Dependencies | Exact files/symbols | Acceptance and rollback |
|---|---|---|---|
| **8C-R1-511 — Draughts setup profile** | 501 | `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` as read-only evidence; `games/draughts_lite/tests/replay.rs` or reassessed fixture-test owner | Build `SetupEvidenceV1Driver` metadata with validator owner `fixture-check`, canonical-byte authority `none`, canonical byte claim `false`, canonical seat grammar, setup options, and expected setup. Delegate to game setup assertions. Fixture bytes unchanged. |
| **8C-R1-512 — Directional Flip setup profile** | 504 | `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` read-only; focused setup/profile test | Validate a public setup profile with byte authority `none`, then delegate to current setup assertions. Fixture bytes, setup hashes/state, RNG use, and visibility remain unchanged. Rollback: one profile test/adapter. |
| **8C-R1-513 — Token Bazaar setup profile** | 505 | `games/token_bazaar/data/fixtures/token_bazaar_standard.fixture.json` read-only; focused setup/profile test | Validate a public setup profile with byte authority `none`, then delegate to current setup assertions. Fixture bytes, setup state/hashes, RNG use, and visibility remain unchanged. Rollback: one profile test/adapter. |
| **8C-R1-520 — Token Bazaar public-export profile** | 505, 216 | `games/token_bazaar/src/replay_support.rs::{PublicReplayExport,export_public_replay,import_public_export}`; `games/token_bazaar/tests/replay.rs`; public-export fixture surface | Validate `PublicExportV1Driver` with public visibility and Rust/WASM export byte authority, then delegate to the existing export/import round trip and hidden-absence assertions. C-02 owns seat spelling bytes; this task must not introduce a second export format change. Rollback: one profile test/adapter only. |

`three_marks` and `race_to_n` have no setup fixture; `column_four`'s current standard fixture is not reclassified as setup evidence; no domain or seat-private driver is added in R1.

### 5.10 Wave I — consolidation, register, and status closeout

| Task | Dependencies | Exact targets | Required result; rollback |
|---|---|---|---|
| **8C-R1-601 — Complete checkpoints and exception register** | All migration tasks | Characterization report; relevant `MSC-8C-*` entries | Record C-06 dependency proof, C-07 not-applicability, C-09 no-consumption-change result, C-10 non-promotion result, C-02 native-seat exceptions, C-04 legacy-hash compatibility, and C-05 adjacent-surface exceptions. No code migration. Rollback: register/report rows only. |
| **8C-R1-602 — Consolidated verification** | 601 | Whole workspace; six games; WASM API; tools; scripts | Run §7 in full, compare the candidate diff against the admission byte inventory, prove only six authorized WASM trace files changed, and attach the command transcript. Any unexplained hash/byte/visibility change blocks closeout. |
| **8C-R1-603 — Documentation and tracker closeout** | 602 | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`; characterization report; accepted spec; `specs/README.md` | After every exit criterion passes, set the `8C-R1` tracker row to `Done` with a concise closeout note. Do not edit R2/R3/R4 or Gate 18 status. Rollback: documentation/status-only commit. |

## 6. Exit criteria

| ID | Exit criterion | Seed/parent mapping |
|---|---|---|
| **EC-R1-01** | The determination evidence records `8C-R1` as the lowest non-`Done` unit, atlas debt `None`, and the parent EC-28/EC-30 sequencing. | Tracker workflow; parent §5, EC-28, EC-30 |
| **EC-R1-02** | The unit contains exactly the six locked games and no seventh game. | 8C-R1 seed game set |
| **EC-R1-03** | Every primary game × helper cell has one final verdict, and every helper sub-surface is migrated, explicitly not applicable, explicitly excepted, or pilot-discharged. | Seed admission/exit; explicit-N/A spec rule |
| **EC-R1-04** | Race and Draughts pilot receipts are treated as named-surface discharges only; no shipped 8C or Gate 7.1 work is reimplemented. | Parent pilot-discharge rule; sharp-delta constraint |
| **EC-R1-05** | The five C-01 literal constructors use `EffectEnvelope::public` with byte-identical effects and unchanged visibility. | C-01; ADR 0008/0009 |
| **EC-R1-06** | Five game-local seat parsers delegate to strict canonical Rust parsing; the WASM import adapter still accepts bounded legacy aliases and rejects malformed/out-of-range labels. | C-02; WASM boundary law |
| **EC-R1-07** | Six WASM replay-document outputs emit canonical `seat_<n>` identities under six independent migration receipts; only the six named WASM golden traces change. | C-02; ADR 0009 intentional migration |
| **EC-R1-08** | Native `default_seats` and non-WASM legacy trace spellings are either unchanged or covered by accepted exceptions with owner, compatibility, rollback, and trigger. | Seed exception route |
| **EC-R1-09** | Five setup functions use C-03 exact-count validation while preserving diagnostics and setup semantics; ring-index adoption is explicitly not applicable. | C-03 |
| **EC-R1-10** | Three Marks, Column Four, Directional Flip, and Token Bazaar expose parallel action-tree v1 byte/hash wrappers with pinned receipts; Race and Draughts retain their shipped pilot implementations. | C-04/C-05 |
| **EC-R1-11** | Every existing legacy action-tree hash remains byte-identical and authoritative for current legacy trace fields; no broad golden update occurs. | ADR 0009 compatibility window |
| **EC-R1-12** | All adjacent C-05 state/effect/view/replay/export/diagnostic surfaces have explicit exception or N/A rows and named future triggers. | No silent omissions; one surface per diff |
| **EC-R1-13** | Replay-command profile drivers cover Draughts, Three Marks, Column Four, Directional Flip, and Token Bazaar; setup profiles cover only Draughts, Directional Flip, and Token Bazaar; public-export profile covers only Token Bazaar. | C-08 actual applicability |
| **EC-R1-14** | `game-test-support` appears only in dev dependency graphs; tools remain validator owners and no game behavior moves into tests or tools. | C-06/C-08 boundary |
| **EC-R1-15** | C-07 and C-09 are recorded not applicable with evidence; existing visibility tests and RNG consumption remain unchanged. | C-07/C-09 checkpoints |
| **EC-R1-16** | The C-10 audit finds no behavioral promotion, game noun in `engine-core`, data behavior, TypeScript authority, or foundation divergence. | C-10; ADR 0008; boundary law |
| **EC-R1-17** | Every byte/hash/seat/output change has characterization, ADR-0009 classification, compatibility window, before/after receipt, and one-surface rollback. | ADR 0009; seed rollback rule |
| **EC-R1-18** | The mechanical-scaffolding register and characterization report contain complete adoption/exception receipts and no unresolved “remaining cleanup” row. | Register governance; EC-28 coverage |
| **EC-R1-19** | All commands and focused evidence in §7 pass without test deletion or weakening. | Testing/agent discipline |
| **EC-R1-20** | `apps/web/README.md` is explicitly recorded `not applicable`; catalog checks pass only as regression guards. | Non-game unit documentation rule |
| **EC-R1-21** | `specs/README.md` flips only `8C-R1` to `Done` after all criteria pass; R2/R3/R4 and Gate 18 remain sequenced after it. | Parent EC-30 |

## 7. Acceptance evidence

### 7.1 Required command set

Run from repository root and retain the complete transcript in the characterization report:

```bash
cargo fmt --all -- --check

cargo test -p engine-core
cargo test -p game-stdlib
cargo test -p game-test-support
cargo test -p wasm-api

cargo test -p race_to_n
cargo test -p draughts_lite
cargo test -p three_marks
cargo test -p column_four
cargo test -p directional_flip
cargo test -p token_bazaar

cargo test --workspace --all-targets

cargo run -p replay-check -- --game race_to_n --all
cargo run -p replay-check -- --game draughts_lite --all
cargo run -p replay-check -- --game three_marks --all
cargo run -p replay-check -- --game column_four --all
cargo run -p replay-check -- --game directional_flip --all
cargo run -p replay-check -- --game token_bazaar --all

cargo run -p fixture-check -- --game race_to_n
cargo run -p fixture-check -- --game draughts_lite
cargo run -p fixture-check -- --game three_marks
cargo run -p fixture-check -- --game column_four
cargo run -p fixture-check -- --game directional_flip
cargo run -p fixture-check -- --game token_bazaar

bash scripts/boundary-check.sh
cargo tree --workspace -e normal --invert game-test-support
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
```

For games without a setup fixture, `fixture-check` must produce an explicit accepted no-fixture/not-applicable result or the report must record the tool's existing behavior and the focused test that owns the N/A. Silence is not evidence.

### 7.2 Focused evidence by migration class

| Surface | Required evidence |
|---|---|
| C-01 constructor | Before/after effect envelope equality; focused public visibility assertion; replay-check proves effect hashes unchanged. |
| C-02 parser | Canonical `seat_0`/`seat_1` round trips; leading-zero, Unicode-digit, malformed, and out-of-range rejection; WASM import adapter still accepts bounded hyphen/symbolic aliases. |
| C-02 WASM output | Byte digest of one document before/after; exact changed seat-bearing fields; old exported document still imports; new exported document replays; only the named `wasm-exported.trace.json` changes. |
| C-03 validation | Valid exact-two setup equality; wrong-count diagnostic equality including code/message; no ordering/state/hash drift. |
| C-04/C-05 v1 | Legacy bytes/hash pinned; v1 bytes/hash pinned; v1 delegates to engine API; order/framing tests; existing trace hashes unchanged. |
| C-08 replay profile | Driver validates class/version/visibility/owner/byte authority/migration note before delegating to actual replay/hash assertions; source trace bytes unchanged. |
| C-08 setup profile | Driver validates metadata with byte authority `none`, then delegates to actual setup validation; fixture bytes unchanged. |
| C-08 public export | Public-only visibility; Rust/WASM byte authority; export/import timeline round trip; hidden-absence rationale; no seat-private claim. |
| C-06 | `cargo tree` shows dev-only edges and no production path. |
| C-07 | Existing public/visibility tests pass; explicit no hidden source datum/viewer pair rationale. |
| C-09 | Characterization report states no selected game-rule bounded-index sampler and shows unchanged RNG-related vectors/hashes. |
| C-10 | Boundary script and register review prove no behavioral promotion. |

### 7.3 Characterization anchors to re-verify at admission

These values are not permission to regenerate expectations; they are sentinels recorded from the current codebase and must be re-read by the executing task before use.

| Game / representative evidence | Legacy action-tree hash sentinel |
|---|---:|
| `race_to_n` `shortest-normal.trace.json` | `8451402319224114161` |
| `draughts_lite` `shortest-quiet.trace.json` | `8126650368011512904` |
| `three_marks` `shortest-normal.trace.json` | `14695981039346656037` |
| `column_four` `shortest-normal-win.trace.json` | `14695981039346656037` |
| `directional_flip` `opening-legal-move.trace.json` | `16457061400249558986` |
| `token_bazaar` `shortest-normal.trace.json` | `6002416109879922099` |

The existing Unit 8C characterization also pins Race's legacy flat surface and the Race/Draughts parallel-v1 pilot evidence. R1 cites those receipts rather than recreating the pilot.

### 7.4 Golden, fixture, and diff policy

- **Authorized default golden changes:** exactly six `games/<game>/tests/golden_traces/wasm-exported.trace.json` files, one per C-02 output task.
- **Unauthorized by default:** all other golden traces, setup fixtures, data manifests, state/effect/view/replay expectations, and benchmark thresholds.
- C-04/C-05 tasks add focused v1 tests and report rows; they do not update legacy trace hashes.
- C-08 tasks wrap existing evidence with typed profile adapters; they do not mass-insert profile fields.
- Any additional changed evidence file is a stop condition until its exact surface is added to the matrix, separately characterized, and admitted under ADR 0009.

### 7.5 Reviews and not-applicable evidence

| Evidence class | Status |
|---|---|
| Rule/game behavior review | Required: prove unchanged; no new rule coverage rows expected. |
| Hidden-information pairwise matrix | `not applicable`: all six are public/perfect-information in this wave. |
| Browser UI smoke | `not applicable`: no web behavior or rendering change. |
| Benchmarks | `not applicable` by default; compile/test and replay evidence are primary. |
| Bot strategy evidence | `not applicable`: no bot policy changes; existing legal-action tests remain regression evidence. |
| IP review | `not applicable`: no prose, art, licensed content, or public assets added. |
| Catalog documentation | Regression guard only; no catalog surface changes. |

### 7.6 External-research sharpening

External sources do not establish repository state. They sharpen only the migration discipline already required by repository law:

- RFC 8949's deterministic-encoding discussion reinforces the use of explicit framing and versioned deterministic bytes rather than an implicit general serializer.[^ext-rfc8949]
- Protocol Buffers documents that ordinary serialization is not canonical, supporting the decision not to treat a generic serializer as a long-lived hash authority.[^ext-protobuf]
- Git's hash-function transition design demonstrates a staged compatibility approach with parallel identities/readers rather than a flag-day rewrite; R1 applies that principle only as an analogy to ADR-0009 compatibility windows.[^ext-git-transition]
- Cargo's dependency and `cargo tree` documentation supports the dev-only `game-test-support` dependency proof.[^ext-cargo-deps][^ext-cargo-tree]

These sources do not authorize scope expansion or override Rulepath's accepted ADRs.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Principles engaged

| Authority / principle | R1 stance |
|---|---|
| `FOUNDATIONS.md` product priority and universal invariants | Preserve deterministic, replayable, testable Rust authority; make no behavior change for convenience. |
| `FOUNDATIONS.md` §12 stop conditions | Stop on unexplained determinism drift, hidden/public leakage, boundary inversion, data behavior, broad golden churn, or an unaccepted architectural decision. |
| `FOUNDATIONS.md` §13 ADR triggers | A new cross-layer authority, serializer/hash authority, visibility taxonomy change, trace-schema meaning change, production test-support dependency, or behavior promotion blocks implementation pending an accepted ADR. |
| `ARCHITECTURE.md` narrowest lawful owner | Constructors/action-tree bytes remain kernel-generic; seat-count plumbing remains `game-stdlib`; game behavior remains in each game; transport aliases/output remain `wasm-api`; profile orchestration remains dev-only. |
| `ENGINE-GAME-DATA-BOUNDARY.md` | No game noun or policy enters `engine-core`; static data remains content/evidence only. |
| ADR 0008 / scaffolding register | Register-first, behavior-free extraction/adoption only; every migration or exception receives a receipt. |
| ADR 0009 | Every changed byte/hash/seat/output surface is separately characterized, classified, versioned/parallel where needed, kept compatible, and independently reversible. |
| ADR 0004 | Public export remains observer-safe. No private data exists in this wave, but public status does not waive export-surface review. |
| Trace/evidence contracts | Trace Schema v1 remains valid legacy command evidence; C-08 profile wrappers are parallel classification, not silent format replacement. |
| Multi-seat/surface contract | All six declare fixed two seats; canonical identity and viewer declarations remain Rust-owned and deterministic. |
| Agent discipline | Bounded tasks, one surface per diff, no test weakening, and full failing-test triage. |

### 8.2 Ownership decisions

| Concern | Lawful owner in R1 | Rejected owner |
|---|---|---|
| Public envelope constructor | Existing `engine-core::EffectEnvelope` inherent API | Game-specific shared wrapper or static data |
| Canonical seat identity | `engine-core::SeatId` strict parser/formatter | TypeScript or permissive game-global parser |
| Legacy seat aliases | Rust `wasm-api` import adapter only | Engine/game parser or browser normalization |
| Output seat spelling | Rust `wasm-api` document serializer | TypeScript post-processing |
| Exact seat-count geometry | `game-stdlib::seat`; game maps diagnostics | `engine-core` game noun/policy or data formula |
| Action-tree v1 bytes/hash | `engine-core::action` using kernel stable writer | Per-game handwritten v1 framing |
| Legacy game serialization | Game-local until a named future migration | R1 mass sweep |
| Profile metadata orchestration | `game-test-support` dev-only; tools/games own validation behavior | Production runtime or behavior-bearing test driver |

### 8.3 Determinism, visibility, and data stance

- Deterministic iteration, ordering, freshness tokens, replay commands, hashes, and RNG consumption are preserved except for the six explicitly named output-seat byte migrations.
- Those six output changes remain public and observer-safe; no new data becomes visible.
- Existing old documents remain importable through the Rust boundary adapter.
- Fixtures and profile adapters are evidence, not procedural configuration; they may not gain selectors, triggers, formulas, or hidden defaults.
- C-09 does not authorize any sampler adoption because no selected characterized rules surface demands it.

### 8.4 Blocking ADR-trigger note

No foundation or ADR amendment is expected. During implementation, immediately block the affected task and add a maintainer note to the characterization report if any of these proves necessary:

- changing the meaning or field set of Trace Schema v1 rather than adding a parallel adapter;
- changing `HashValue` or adopting a new canonical serializer;
- making v1 action-tree hash authoritative across existing traces;
- narrowing removal of legacy seat aliases;
- adding a production dependency on `game-test-support`;
- moving setup, projection, visibility, scoring, outcome, bot, or diagnostic policy into shared code; or
- changing public/export visibility taxonomy.

The note must name the affected `FOUNDATIONS.md` §13 principle and the exact ADR sections to supersede. The implementation must not design around doctrine.

## 9. Forbidden changes

In addition to §3.10, the implementation must not:

- change any game's legal action tree, action ordering, labels, accessibility labels, metadata ordering, tags, preview semantics, freshness semantics, or dead-branch behavior;
- replace game-local `other()` methods with generic index arithmetic merely to claim C-03 adoption;
- change `crates/wasm-api/src/seats.rs::seats()` or `seats_for_count()` in the same diff as an output migration;
- make legacy hyphen or symbolic aliases valid in game-local parsers;
- emit legacy aliases from a newly canonical output path;
- delete compatibility readers after canonical output lands;
- update more than one game's WASM document/golden in one migration diff;
- replace `ReplayHashes.action_tree_hash` with v1 in R1;
- manually duplicate `StableBytesWriter` action-tree framing in a game;
- move replay/setup/export behavior into `game-test-support`, `replay-check`, or `fixture-check`;
- infer a profile from `.trace.json` or `.fixture.json` suffix;
- claim `setup-evidence-v1` for a command fixture merely because it contains setup fields;
- add `seat-private-export-v1` or C-07 pairwise geometry to these public games without a real hidden surface;
- modify bots, variants, scoring, effects content, public view content, game rules docs, or browser components;
- alter benchmark thresholds or optimize unrelated code;
- change foundation docs, accepted ADRs, the mechanic atlas, or the evidence contract as routine implementation work; or
- flip `8C-R1` to `Done` before every acceptance command, register receipt, exception row, and byte-diff audit passes.

## 10. Documentation updates required

| Document/artifact | Required? | Exact update |
|---|---:|---|
| `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` | yes | New append-only admission, migration, exception, byte/hash, and command-evidence packet described in §4.2. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes | Add/update per-game adoption receipts and explicit N/A/exception decisions under the relevant C-01…C-10 entries. |
| `specs/8c-r1-public-fixed-seat-scaffolding.md` | yes, after reassessment/acceptance | The accepted form of this working artifact. Do not present the current report artifact as already landed. |
| `specs/README.md` | yes | `Not started` → `Planned` when the accepted spec lands; `Planned`/`In progress` → `Done` only after closeout. Do not alter successor statuses. |
| Rust API/test comments | as needed | Short comments only where a compatibility window or parallel surface would otherwise be ambiguous. |
| `apps/web/README.md` | **not applicable** | No game, catalog, renderer, smoke-layer, or web-shell surface is added or changed. |
| Game rules/mechanics/UI/AI docs | **not applicable** | Behavior, mechanics, UI, and bots are unchanged. |
| `docs/FOUNDATIONS.md`, architecture/boundary docs, ADRs, trace/evidence contracts | no by default | A required change is a blocking §8.4 ADR trigger, not routine documentation cleanup. |
| `docs/MECHANIC-ATLAS.md` | no | No mechanic promotion debt or mechanic-ladder decision changes. |
| `docs/ROADMAP.md` | no | Tracker/spec sequencing already owns the unit; no roadmap redecision. |

`node scripts/check-catalog-docs.mjs` remains required only as a regression guard proving the non-game unit did not disturb catalog truth.

## 11. Sequencing

### 11.1 External sequence

- **Predecessor:** Unit 8C is `Done`; its infrastructure and pilots are hard prerequisites and must be reused.
- **Current unit:** `8C-R1` only.
- **Successors:** `8C-R2`, then `8C-R3`, then `8C-R4`, then Gate 18 according to the active tracker. This spec neither plans nor executes successor scope.
- **Gate 18 admission:** remains blocked until all C-11 waves are closed, explicitly not applicable, or accepted-excepted, consistent with parent EC-30.

### 11.2 Internal dependency order

1. Wave A freezes the complete matrix and byte/hash baseline.
2. C-01, parser-only C-02, and C-03 tasks may proceed independently after characterization.
3. C-02 output helper lands before any one-game output flip; each game flip is independent and reversible.
4. C-04/C-05 parallel-v1 tasks remain independent of C-02 output migrations and must not share golden changes.
5. C-08 replay drivers may proceed after characterization; setup/public-export drivers depend on their game's replay-profile task. Token public-export profiling also depends on its C-02 output migration so the final public byte surface is characterized once.
6. Consolidation begins only after every migration task and every N/A/exception decision is complete.
7. Status closeout is last.

### 11.3 Admission rule for an implementation diff

A diff is admitted only when it names one matrix surface, has a pre-migration receipt, changes no unrelated surface, keeps a compatibility reader when required, and can be reverted without reverting another migration. Bundled “all six games” or “update snapshots” diffs are inadmissible even when green.

## 12. Assumptions

### 12.1 One-line-correctable assumptions

- `assumption:` the unit slug/label is `8c-r1-public-fixed-seat-scaffolding`; the fixed unit ID is `8C-R1`.
- `assumption:` this spec lives at the accepted path `specs/8c-r1-public-fixed-seat-scaffolding.md` (renamed from its intermediate working-artifact name during reassessment); the `specs/README.md` index row flip is a separate closeout action per §10.
- `assumption:` the owner field remains “Rulepath maintainers” until reassessment assigns a named owner.
- `assumption:` candidate task IDs `8C-R1-001…603` are planning identifiers and may be renamed during ticket decomposition without combining surfaces.
- `assumption:` register receipt IDs/names are assigned during reassessment according to the current register convention; the receipt content and one-surface granularity are not optional.
- `assumption:` the C-02 native `default_seats` and non-WASM legacy trace spelling surfaces remain accepted exceptions in R1; only the six WASM exported-document surfaces migrate.
- `assumption:` the output-only canonical roster helper name is one-line-correctable, but it must leave legacy setup builders unchanged.
- `assumption:` C-08 uses parallel test adapters around existing evidence, following the Race pilot; fixtures/traces remain read-only unless a separately named task is added during reassessment with its own ADR-0009 packet.
- `assumption:` `game-test-support` remains dev-only and no tool needs more than thin profile dispatch; if reassessment finds an absent dispatch registration, it becomes a separately bounded task.
- `assumption:` the selected C-05 migration is action-tree v1 only; state, effect, view, replay/export, and diagnostic stable-byte surfaces remain accepted exceptions with the triggers in §3.6.
- `assumption:` no selected game has a rules RNG surface requiring `next_index_unbiased_v1`; a contrary characterization result blocks the affected game for a separate RNG migration packet rather than silently expanding R1.
- `assumption:` no foundation or ADR amendment is required; any discovered need invokes §8.4.

### 12.2 Reassessment questions that do not reopen scope

`/reassess-spec` may correct only implementation-detail facts such as a test module owner, helper symbol spelling, register receipt identifier, or whether a thin validator registration already exists. It must not add games, helpers, behavior changes, successor work, a mass trace migration, or a new architecture decision.

### 12.3 Repository evidence basis

The key repository authorities and code seams this spec relies on are linked below (repo-relative paths):

- Authority and workflow: [`docs/README.md`](../docs/README.md); [`docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md); [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md); [`docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md); [`docs/ROADMAP.md`](../docs/ROADMAP.md); [`specs/README.md`](../specs/README.md).
- Scaffolding and migration law: [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`](../docs/MECHANICAL-SCAFFOLDING-REGISTER.md); [`docs/adr/0008-mechanical-scaffolding-governance.md`](../docs/adr/0008-mechanical-scaffolding-governance.md); [`docs/adr/0009-replay-fixture-hash-taxonomy.md`](../docs/adr/0009-replay-fixture-hash-taxonomy.md); [`docs/adr/0004-hidden-info-replay-export-taxonomy.md`](../docs/adr/0004-hidden-info-replay-export-taxonomy.md).
- Evidence law: [`docs/TESTING-REPLAY-BENCHMARKING.md`](../docs/TESTING-REPLAY-BENCHMARKING.md); [`docs/TRACE-SCHEMA-v1.md`](../docs/TRACE-SCHEMA-v1.md); [`docs/EVIDENCE-FIXTURE-CONTRACT.md`](../docs/EVIDENCE-FIXTURE-CONTRACT.md); [`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`](../docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md); [`docs/AGENT-DISCIPLINE.md`](../docs/AGENT-DISCIPLINE.md).
- Parent/source evidence: [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`](../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md); [`reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`](../reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md); [`reports/8c-mechanical-scaffolding-characterization.md`](../reports/8c-mechanical-scaffolding-characterization.md).
- Shared landed homes: [`crates/engine-core/src/action.rs`](../crates/engine-core/src/action.rs); [`crates/engine-core/src/replay.rs`](../crates/engine-core/src/replay.rs); [`crates/engine-core/src/rng.rs`](../crates/engine-core/src/rng.rs); [`crates/game-stdlib/src/seat.rs`](../crates/game-stdlib/src/seat.rs); [`crates/game-test-support/src/profiles.rs`](../crates/game-test-support/src/profiles.rs); [`crates/game-test-support/src/no_leak.rs`](../crates/game-test-support/src/no_leak.rs); [`crates/wasm-api/src/seats.rs`](../crates/wasm-api/src/seats.rs).
- Validator/guard owners: [`tools/replay-check/src/main.rs`](../tools/replay-check/src/main.rs); [`tools/fixture-check/src/main.rs`](../tools/fixture-check/src/main.rs); [`scripts/boundary-check.sh`](../scripts/boundary-check.sh); [`scripts/check-doc-links.mjs`](../scripts/check-doc-links.mjs); [`scripts/check-catalog-docs.mjs`](../scripts/check-catalog-docs.mjs).

### 12.4 Final self-check before acceptance

The accepted form must answer **yes** to every item:

- Is `8C-R1` confirmed rather than re-decided?
- Are exactly six games present in the primary matrix?
- Does every helper sub-surface have a verdict and owner?
- Are Race/Draughts pilot surfaces treated as discharged rather than rebuilt?
- Is every changed byte/hash/seat/output surface independently characterized and reversible?
- Are only six WASM exported traces authorized to change by default?
- Are legacy action-tree hashes preserved while four parallel v1 surfaces are added?
- Are C-08 drivers parallel test adapters rather than a blanket fixture rewrite?
- Are C-06/C-07/C-09/C-10 explicit?
- Is `engine-core` noun-free and game behavior local?
- Are tests preserved and the failing-test protocol mandatory?
- Is `apps/web/README.md` explicitly `not applicable`?
- Is the artifact framed for `/reassess-spec` → `/spec-to-tickets` rather than execution?
- Do EC-28/EC-30 and successor sequencing remain intact?

## Outcome

Completed on 2026-06-23.

All `8C-R1` tickets are archived. The full §7.1 command set passed, the
golden/fixture diff audit found exactly the six authorized
`wasm-exported.trace.json` files changed, and the C-06/C-07/C-09/C-10
checkpoint conclusions are recorded in the characterization report and
mechanical-scaffolding register. No successor `8C-R2`/`8C-R3`/`8C-R4` or Gate
18 status was changed.

[^ext-rfc8949]: IETF, *RFC 8949 — Concise Binary Object Representation (CBOR)*, deterministic encoding requirements: https://www.rfc-editor.org/rfc/rfc8949.html
[^ext-protobuf]: Protocol Buffers documentation, *Proto Serialization Is Not Canonical*: https://protobuf.dev/programming-guides/serialization-not-canonical/
[^ext-git-transition]: Git documentation, *Hash Function Transition*: https://git-scm.com/docs/hash-function-transition/
[^ext-cargo-deps]: Cargo Reference, *Specifying Dependencies*: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
[^ext-cargo-tree]: Cargo Book, *cargo tree*: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
