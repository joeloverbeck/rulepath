# Unit 8C — Mechanical-scaffolding code extraction (Part C)

Unit 8C is complete: `specs/README.md` selects the lowest unit not marked
`Done`, row `8M` is `Done`, and 8C now closes the pre-Gate-18
mechanical-scaffolding unit while seeding 8C-R1...8C-R4 before Gate 18. Its
dependencies were satisfied — ADR 0008 (mechanical-scaffolding
governance) and ADR 0009 (replay/fixture/hash taxonomy) are both `Accepted`,
ADR 0004 (hidden-info replay/export) governs the visibility surfaces, and
`docs/MECHANIC-ATLAS.md` §10A records `Current debt: _None_`.

Scope delta against the source report
(`reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`, Part C / Batches 3–4):

- **Already shipped in 8M:** report Parts A, B, and D; ADRs 0008/0009; the
  authority-map and foundation/area-doc amendments;
  `MECHANICAL-SCAFFOLDING-REGISTER.md`; `EVIDENCE-FIXTURE-CONTRACT.md`;
  `GAME-EVIDENCE.md`; and template realignment.
- **This unit:** report Part C / Batches 3–4 as code — C-01 through C-10, the
  dev-only `game-test-support` crate, the harnesses, and the bounded pilots.
- **After this unit:** C-11's remaining game retrofits as bounded follow-on
  units (8C-R1…R4), then Gate 18. A pilot made here does not count as a
  whole-game retrofit unless its ticket explicitly audits every applicable
  helper and surface.

No foundation/ADR contradiction was found: the work proceeds entirely within the
accepted mechanical-scaffolding, fixture-profile, replay/hash, and
hidden-information laws.

## 1. Header

| Field | Value |
|---|---|
| **Spec ID** | `8C` |
| **Title** | Mechanical-scaffolding code extraction (Part C) |
| **Stage** | Public scaling phase; pre-Gate-18 code unit |
| **Gate / unit** | Unit 8C; non-game infrastructure and pilot-adoption unit |
| **Status on authoring** | `Planned` |
| Status | `Done` |
| **Date** | 2026-06-22 |
| **Owner** | Rulepath maintainers |
| **Source scope** | `reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` Part C, C-01…C-11, Batches 3–4, narrowed by completed unit 8M. |

**Authority order for implementation and review:**

1. `docs/FOUNDATIONS.md`.
2. `docs/ARCHITECTURE.md` and `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
3. Accepted ADR 0008, ADR 0009, and ADR 0004 for the sections they govern.
4. Area law: the scaffolding register, mechanic atlas, testing/replay law, evidence-fixture contract, trace schema, WASM boundary, agent discipline, and roadmap.
5. This spec.
6. AGENT-TASK tickets produced from this spec.

Where a lower layer appears to conflict with a higher layer, the lower-layer plan is wrong and must stop rather than design around the conflict.

## 2. Objective

Create the smallest complete shared mechanical-scaffolding layer that closes the known pre-Gate-18 code debt without moving game behavior into shared code.

The unit will:

1. Add noun-free kernel ergonomics for effect-envelope construction, canonical seat identifiers, versioned stable bytes, canonical action-tree encoding/hashing, and explicitly versioned bounded-index RNG sampling.
2. Add narrow game-layer seat-count and ring-index plumbing to `game-stdlib` without introducing role, team, deal, pass, turn-policy, or diagnostic semantics.
3. Create a dev/test-only `crates/game-test-support` crate whose consumers use it only through `[dev-dependencies]` and whose helpers own proof geometry, never legality, projection, redaction, or behavior.
4. Provide pairwise no-leak and evidence-profile drivers that preserve or strengthen ADR 0004 and validate the five ADR-0009 profiles independently.
5. Establish one register decision per candidate **before** extraction, including affected hashes, visibility, migration set, and acceptance evidence.
6. Prove each helper against a small representative pilot set spanning simple/compound action trees, public/private effects, fixed/variable seat counts, two-seat/N-seat hidden information, command replay, viewer-scoped exports, setup evidence, domain evidence, and an already-equivalent unbiased RNG implementation.
7. Preserve every legacy byte/hash surface unless a named pilot surface receives a versioned, characterized, per-surface migration with a compatibility window and rollback point.
8. Seed, but not execute, bounded C-11 follow-on units for all remaining games before Gate 18.

The objective is **not** a general rules engine, shared card/deck framework, new game, schema rewrite, or repository-wide cleanup. It is behavior-free plumbing extraction with proof.

## 3. Scope

### 3.1 In scope

| Report ID | In-scope obligation |
|---|---|
| **C-01** | Generic `EffectEnvelope<T>` constructors for public and seat-private envelopes. |
| **C-02** | Strict canonical `seat_<zero-based>` parse/format helpers in Rust plus an import-only legacy-alias adapter at the WASM boundary. |
| **C-03** | Typed nonzero seat counts, exact/range validation, and checked ring-index arithmetic. Macro-generated game seat enums remain deferred. |
| **C-04** | A version-named canonical action-tree byte encoding and `stable_hash_vN`, covering the exact action-tree contract present in the current `engine-core` action contract (`crates/engine-core/src/action.rs`). |
| **C-05** | A small explicit, versioned stable-byte writer with domain separation, tags, type framing, lengths, and deterministic sequence rules; no derive/reflection magic. |
| **C-06** | New workspace member `crates/game-test-support`, usable by game tests only as a development dependency. |
| **C-07** | Pairwise source-seat × viewer × surface no-leak assertion geometry with game-owned canaries, snapshots, authorization, and reveal timing. |
| **C-08** | Separate profile drivers for `replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, and `domain-evidence-v1`; thin tool integration without relocating game behavior. |
| **C-09** | Document the unchanged legacy modulo sampler; add an explicit unbiased rejection-sampling `v1` method; characterize RNG consumption; keep shuffle/deal policy local. |
| **C-10** | Affirm the existing Non-Promotion List with a rejected/local-only register decision and retain the atlas↔register boundary. |
| **Pilots** | Bounded adoptions in Race to N, Draughts Lite, High Card Duel, River Ledger, Vow Tide, and one Briar Circuit domain-fixture driver. |
| **Forward seeds** | Create bounded C-11 follow-on tracker/spec seeds for remaining game audits and retrofits. |

### 3.2 Out of scope

- Full C-11 migration across all remaining games.
- Gate 18 / Spades, partnership rules, team scoring, partnership UI grouping, or partnership-aware seat types.
- Re-authoring ADR 0008, ADR 0009, ADR 0004, the scaffolding register schema, the evidence-fixture contract, `GAME-EVIDENCE.md`, or the 8M template changes.
- Report Parts A, B, or D except truthful status/entry updates required by code closeout.
- A macro or derive that generates game seat enums.
- A generic shuffle, deal, hand, deck, reveal, betting, pot, trick, scoring, team, graph, reaction, or accounting framework.
- A new serialization dependency adopted as the repository hash authority. External formats inform the framing design; they do not replace the explicit Rulepath byte contract.
- A new hash function. `HashValue::from_stable_bytes` remains the hash algorithm unless a future accepted ADR says otherwise.
- General fixture-schema consolidation, fixture renaming, or mass insertion of profile metadata.
- Browser-visible features or catalog changes.
- Benchmark threshold recalibration. This unit must not alter gameplay performance semantics; compile/test coverage is required, while new runtime benchmarks are `not applicable` unless a helper proves measurable risk during implementation.

### 3.3 Not allowed

- Mechanic nouns or policy in `engine-core`.
- Behavioral promotion through the scaffolding lane.
- Silent byte, hash, trace, RNG-consumption, seat-ID, visibility, or export changes.
- Blanket golden regeneration, “update snapshots” sweeps, or accepting new hashes merely because the tests were regenerated.
- A shared helper that decides legality, setup semantics, reveal timing, viewer authorization, projection/redaction, scoring, outcome, bot choice, or diagnostic prose.
- `game-test-support` as a normal/build dependency of production, WASM, browser, or game library targets.
- Test canaries in public, viewer-scoped, or seat-private artifacts.
- TypeScript seat normalization or legality decisions.
- YAML, procedural data, selectors, triggers, formulas, or a DSL.
- Test deletion, weakening, narrowing, ignored failures, or replacement of a specific game assertion by a less specific generic assertion.
- Changes to `TRACE-SCHEMA-v1` bytes without an explicit ADR-0009 migration packet.

## 4. Deliverables

### 4.1 Concrete artifact tree

The exact filenames below are the required default; a maintainer may correct a module name before implementation if ownership and API semantics remain unchanged.

```text
Cargo.toml
crates/engine-core/src/lib.rs
crates/engine-core/src/action.rs
crates/engine-core/src/replay.rs
crates/engine-core/src/rng.rs
crates/game-stdlib/src/lib.rs
crates/game-stdlib/src/seat.rs                         # new
crates/game-test-support/Cargo.toml                   # new
crates/game-test-support/src/lib.rs                   # new
crates/game-test-support/src/no_leak.rs               # new
crates/game-test-support/src/profiles.rs              # new
crates/wasm-api/src/seats.rs
scripts/boundary-check.sh
reports/8c-mechanical-scaffolding-characterization.md  # new

# Pilot call sites and evidence
 games/race_to_n/src/{effects.rs,ids.rs,replay_support.rs,setup.rs}
 games/race_to_n/tests/{replay_tests.rs,serialization_tests.rs}
 games/race_to_n/tests/golden_traces/shortest-normal.trace.json
 games/draughts_lite/src/replay_support.rs
 games/draughts_lite/tests/golden_traces/multi-jump.trace.json
 games/high_card_duel/tests/{visibility.rs,replay.rs}
 games/high_card_duel/tests/golden_traces/{public-replay-export-import.trace.json,seat-private-view.trace.json}
 games/river_ledger/src/{effects.rs,ids.rs,replay_support.rs,setup.rs}
 games/river_ledger/tests/{visibility.rs,replay.rs}
 games/river_ledger/data/{manifest.toml,fixtures/river_ledger_3p_standard.fixture.json}
 games/river_ledger/tests/golden_traces/{public-replay-export-import.trace.json,seat-private-view.trace.json}
 games/vow_tide/data/{manifest.toml,fixtures/vow_tide_3p_standard.fixture.json}
 games/vow_tide/tests/golden_traces/{public-replay-export-import.trace.json,seat-private-replay-export-import-all-viewers.trace.json}
 games/briar_circuit/data/{manifest.toml,fixtures/briar_circuit_moon.fixture.json,fixtures/briar_circuit_first_trick_exception.fixture.json}

# Governance and closeout
 docs/MECHANICAL-SCAFFOLDING-REGISTER.md
 specs/README.md
 specs/unit-8c-mechanical-scaffolding-code-extraction.md
```

`tools/replay-check` and `tools/fixture-check` may receive thin registration/profile-dispatch changes if the pilots require them. They must remain validator owners and must not acquire game behavior. `apps/web/README.md` is not a closeout surface for this non-game unit.

### 4.2 Required landing-home decisions

The brief delegates the exact homes. The following are the **required defaults** because they apply the architecture ownership matrix and “narrowest lawful owner wins.”

| Candidate | Options considered | Required default | Reason / rejection boundary |
|---|---|---|---|
| **C-01 envelope constructors** | `engine-core`; `game-stdlib`; remain game-local | `engine-core`, inherent methods on `EffectEnvelope<T>` | The type and visibility vocabulary already belong to the kernel. Constructors preserve fields and add no game noun or policy. `game-stdlib` would be an unnecessary wrapper. |
| **C-02 canonical seat grammar** | All in `engine-core`; all in `wasm-api`; `game-stdlib`; local parsers | Strict canonical constructor/parser in `engine-core`; legacy import aliases only in `wasm-api` | Canonical seat identity is kernel vocabulary; transport aliases are boundary compatibility. A global permissive `FromStr` is rejected because it would silently legitimize legacy/role labels everywhere. |
| **C-03 seat counts/ring index** | `engine-core`; `game-stdlib::seat`; local | `game-stdlib::seat` | Nonzero counts and checked ring geometry are reusable game-layer plumbing, but exact/range admission and diagnostic mapping remain game-owned. Keeping them out of the kernel protects noun-free minimalism. |
| **C-04 action-tree encoding/hash** | `engine-core`; `game-stdlib`; per-game | `engine-core::action` API using the kernel writer/hash types | `ActionTree` is a kernel contract. Per-game encoders are already drifting. The encoding covers only contract fields and order, not legal-choice meaning. |
| **C-05 stable-byte writer** | `engine-core::replay`; separate utility crate; third-party serializer | `engine-core::replay` or a noun-free sibling re-exported there | Stable replay/hash bytes are kernel infrastructure. A separate crate adds dependency surface; third-party general serializers do not provide this repository’s explicit long-lived byte contract by default. |
| **C-06 test-support crate** | Game-local modules; `game-stdlib`; `engine-core`; new crate | New `crates/game-test-support` | The helpers are test/evidence orchestration and must be absent from production dependency graphs. They do not belong in runtime crates. |
| **C-07 no-leak geometry** | `game-test-support`; runtime visibility module; game-local | `game-test-support::no_leak` | Only the Cartesian-product assertion geometry is common. Runtime projection and authorization remain in each game. |
| **C-08 profile drivers** | `game-test-support`; CLI tools; game-local | Generic drivers in `game-test-support`; canonical validator/registration adapters remain in tools and games | Tests need reusable orchestration, but validator ownership and game-specific setup/commands/exports cannot move. The test crate does not become a normal dependency of CLI tools by default. |
| **C-09 bounded-index sampling** | Change `next_index`; add explicit method in `engine-core`; local only | Keep `next_index` byte-identical and document it as legacy modulo semantics; add `next_index_unbiased_v1` in `engine-core` | The random-word-to-index mapping is noun-free. Silently changing the existing method would change RNG consumption and deals. Shuffle/deal policy remains local. |
| **C-10 non-promotion boundary** | Atlas rewrite; register-only entry; no change | Register a rejected/local-only decision that points to the existing list; do not rewrite the list | 8M already published the doctrine. 8C records implementation compliance and a future review trigger rather than pretending the document is missing. |

### 4.3 Required API defaults

#### C-01 — `EffectEnvelope<T>`

```rust
impl<T> EffectEnvelope<T> {
    pub fn public(payload: T) -> Self;
    pub fn private_to(seat_id: SeatId, payload: T) -> Self;
}
```

The constructors must set only `visibility` and `payload`; no filtering, reveal, serialization, or conversion behavior is permitted. `private_to` accepts an already typed `SeatId`, not `impl Into<String>`.

#### C-02 — canonical seat IDs and aliases

Kernel surface, exact naming one-line-correctable:

```rust
impl SeatId {
    pub fn from_zero_based_index(index: u32) -> Self;
    pub fn parse_canonical(input: &str) -> Result<Self, CanonicalSeatIdError>;
    pub fn canonical_zero_based_index(&self) -> Result<u32, CanonicalSeatIdError>;
}
```

Canonical grammar is exactly `seat_<unsigned-zero-based-decimal>`. The strict parser rejects whitespace, signs, empty suffixes, non-digits, overflow, and non-canonical leading-zero spellings except `seat_0`. Formatting always emits the canonical form. Do not make permissive legacy parsing the global `SeatId` meaning.

The WASM/import adapter accepts:

- canonical `seat_<n>`;
- bounded legacy `seat-<n>`;
- symbolic legacy values such as `seat-a` **only** through an explicit, caller-provided seat-order alias table.

Every successful import normalizes immediately to typed canonical identity. Every output path **that adopts the canonical formatter** emits canonical underscore form. Unknown or ambiguous values reject; TypeScript does not repair them.

Scope note on existing output surfaces: at the current commit the game layer is already underscore-canonical (e.g. `games/race_to_n/src/ids.rs` formats `seat_<n>`), but several `wasm-api` per-game *trace* adapters still emit legacy hyphen (`crates/wasm-api/src/seats.rs`, e.g. `trace_race_seat` → `seat-0`), and the committed `race_to_n` / `draughts_lite` / `high_card_duel` golden traces are hyphen. "Every output path emits canonical underscore form" governs the formatter's contract and newly-written or migrated evidence; it does **not** authorize silently routing an existing hyphen-emitting trace adapter through the canonical formatter. Flipping such a surface is a hash-bearing change and follows the named ADR-0009 per-surface migration protocol (EC-06, EC-11, work item 8C-009), never a silent rewrite.

#### C-03 — count/ring plumbing

Required semantic surface:

```rust
pub struct SeatCount(/* nonzero usize */);
pub struct SeatCountRange { /* inclusive min/max */ }

impl SeatCount {
    pub fn new(actual: usize) -> Result<Self, SeatCountError>;
    pub fn get(self) -> usize;
    pub fn checked_index(self, index: usize) -> Result<usize, SeatIndexError>;
    pub fn next_ring_index(self, current: usize) -> Result<usize, SeatIndexError>;
}

impl SeatCountRange {
    pub fn inclusive(min: usize, max: usize) -> Result<Self, SeatCountRangeError>;
    pub fn validate(self, actual: usize) -> Result<SeatCount, SeatCountError>;
}
```

The shared layer returns typed structural errors only. Each game maps them to its existing setup/action diagnostics. No pass direction, dealer rule, bidding order, partnership, active-seat policy, or generated enum enters the module.

#### C-05 — `StableBytesWriter` v1

Required properties:

- explicit domain and surface version;
- deterministic integer endianness;
- field tags and type tags;
- length-delimited strings, bytes, nested records, and sequence elements;
- explicit option and enum discriminants;
- caller-supplied stable order for records and collections;
- rejection of duplicate/non-increasing record field tags;
- no unordered-map helper, floating-point helper, reflection, derive, or implicit schema discovery in v1;
- raw UTF-8 bytes with no hidden normalization.

Required default framing, exact constants one-line-correctable before the first fixture migration:

```text
magic                 = 4 bytes: "RPSB"
writer_version        = u16 little-endian, value 1
domain_length         = u32 little-endian
domain                = domain_length raw bytes
surface_version       = u32 little-endian
record fields         = ascending field tag order
field                  = tag:u32 | type:u8 | payload_length:u32 | payload
sequence payload      = count:u32 | repeated(element_length:u32 | element_bytes)
option payload        = discriminant:u8 (0/1) | optional framed value
bool payload          = one byte 0 or 1
```

`HashValue::from_stable_bytes` remains the hash function. The new writer defines bytes; it does not redefine the hash algorithm.

#### C-04 — action-tree encoding v1

`ActionTreeEncodingVersion::V1` must encode the exact current action-tree contract (`crates/engine-core/src/action.rs`: `ActionTree`, `ActionNode`, `ActionChoice`, `ActionMetadata`, `ActionPreview`), recursively and in existing vector order:

- encoding/domain version;
- freshness token;
- ordered root choices and child choices;
- choice segment;
- label;
- accessibility label;
- metadata entries in their existing vector order, with key and value framed independently;
- tags in their existing vector order;
- current `ActionPreview` discriminant (`Unavailable` or `Available`); the enum is payload-free (`crates/engine-core/src/action.rs`);
- explicit `next = none/some`, with recursively framed child node.

The current action contract does **not** contain the source report’s hypothetical disabled-state/reason fields. V1 must not invent them. If a later contract adds fields, it requires a later encoding version or an explicit rule that the field is outside the hash surface.

Expose only version-explicit persistence methods, for example:

```rust
impl ActionTree {
    pub fn stable_bytes(&self, version: ActionTreeEncodingVersion) -> Vec<u8>;
    pub fn stable_hash(&self, version: ActionTreeEncodingVersion) -> HashValue;
}
```

No unversioned “current” persisted hash is allowed.

#### C-09 — bounded-index RNG

Keep the existing method’s implementation and consumption exactly unchanged:

```rust
pub fn next_index(&mut self, upper_bound: usize) -> Option<usize>; // legacy modulo semantics
```

Add an explicitly named unbiased method whose accepted-zone/rejection behavior matches the already repeated River Ledger / Plain Tricks / Poker Lite implementation:

```rust
pub fn next_index_unbiased_v1(&mut self, upper_bound: usize) -> Option<usize>;
```

The method returns `None` for zero, computes the largest accepted prefix of the `u64` range divisible by the bound using `u128`, redraws rejected words, and returns the remainder. Tests must pin both returned indices and random-word consumption for selected seeds/bounds. No game that currently calls modulo `next_index` migrates in 8C unless its exact output/consumption is separately characterized and version-migrated.

### 4.4 Register entries required before implementation

The first implementation ticket creates complete entries, not placeholder rows:

| Entry ID | Candidate | Initial decision | Required home |
|---|---|---|---|
| `MSC-8C-001` | Generic effect-envelope constructors | `accepted` after review | `engine-core` |
| `MSC-8C-002` | Canonical seat-ID grammar plus import aliases | `accepted` after review | `engine-core` + `wasm-api` adapter |
| `MSC-8C-003` | Seat-count validation and ring-index arithmetic | `accepted` after review | `game-stdlib::seat` |
| `MSC-8C-004` | Action-tree encoding/hash v1 | `accepted` after characterization | `engine-core` |
| `MSC-8C-005` | Stable-byte writer v1 | `accepted` after byte-contract review | `engine-core` |
| `MSC-8C-006` | Dev-only game test-support crate | `accepted` after dependency review | `game-test-support` |
| `MSC-8C-007` | Pairwise no-leak assertion geometry | `accepted` after ADR-0004 review | `game-test-support` |
| `MSC-8C-008` | Evidence-profile drivers | `accepted` after ADR-0009 review | `game-test-support` + thin validator adapters |
| `MSC-8C-009` | Versioned bounded-index sampling | `accepted` only for the sampler primitive | `engine-core` |
| `MSC-8C-010` | Behavioral-policy bundle on the Non-Promotion List | `rejected / local-only` | game crates / behavioral atlas only |

Each row must expand to every field in the register Entry Schema: owner, semantic risk, exact duplicate sites, explicit exclusions, affected hashes, visibility impact, determinism impact, migration set, acceptance evidence, rejection rationale where applicable, and next review trigger.

### 4.5 Pilot-adoption set

| Pilot | What it proves | Explicit limit |
|---|---|---|
| **Race to N** | Public C-01 envelope; canonical-seat adapter and exact-two count path; flat C-04/C-05 action tree; `replay-command-v1` driver. | Does not authorize mass conversion of legacy hyphenated fixture bytes. |
| **Draughts Lite** | Recursive compound-path action-tree encoding and delimiter-collision resistance. | No movement/capture/promotion behavior enters the encoder. |
| **High Card Duel** | Two-seat hidden-information C-07 geometry across view/action/effect/export surfaces. | Game still owns private-card projection, reveal, and export policy. |
| **River Ledger** | Public/private envelope constructors; canonical variable seats; 3–6 ring geometry; N-seat no-leak matrix; `setup-evidence-v1` driver; byte-identical C-09 unbiased sampler extraction. | Betting, all-in/reopen, pot, evaluator, showdown, and allocation remain local. |
| **Vow Tide (profile-only)** | Real `public-export-v1` and `seat-private-export-v1` drivers, including all declared seat viewers. | This does not migrate Vow’s setup, no-leak matrix, seat/ring helpers, modulo RNG caller, bidding, deal, or trick behavior. |
| **Briar Circuit (fixture-only)** | A real `domain-evidence-v1` driver against the scoring fixture, including explicit visibility/validator/canonical-byte metadata. | This is not a full Briar C-11 retrofit and does not promote scoring or trick lifecycle. |

This six-game set is the minimum grounded set that exercises all distinct risk/profile shapes without turning 8C into the forbidden 17-game sweep. Vow and Briar adoptions are deliberately fixture/profile-only.

## 5. Work breakdown

Every work item below is a candidate `templates/AGENT-TASK.md` packet and must name exact files, symbols, tests, affected hash/visibility surfaces, and rollback scope. A ticket must follow `docs/AGENT-DISCIPLINE.md`: first determine whether any failing test remains valid, then whether the fault is in the SUT or test, then fix the correct layer. No ticket may “green” the suite by weakening evidence.

### Wave 0 — admission, register, and characterization

| Work ID | Report IDs | Candidate AGENT-TASK | Dependencies | Review boundary and required evidence |
|---|---|---|---|---|
| **8C-001** | all | Finalize this spec (`specs/unit-8c-mechanical-scaffolding-code-extraction.md`) and flip the `specs/README.md` row 8C from `Not started`/stale `Blocked` to `Planned` with the new spec link; leave implementation untouched. | 8M `Done`; ADRs accepted | Spec passes authority review and preserves the exact in/out boundary. Rollback is the single spec/index diff. |
| **8C-002** | C-01…C-10 | Add full `MSC-8C-001`…`MSC-8C-010` entries before any helper implementation. Use `candidate` until each owning ticket proves acceptance; C-10 starts and remains rejected/local-only. | 8C-001 | Register schema complete for every entry; no placeholder “TBD” for affected hashes, visibility, migration set, or evidence. Rollback is register-only. |
| **8C-003** | C-02, C-04, C-05, C-08, C-09 | Produce `reports/8c-mechanical-scaffolding-characterization.md` for every pilot surface. Record current inputs, bytes, hashes, fixture profile/classification, seat spellings, RNG outputs and draw counts, and owning validators. Do not change production code or fixtures. | 8C-002 | Packet includes Race flat tree, Draughts compound tree, River setup/public/seat-private evidence, Vow public/seat-private export artifacts, High Card public/seat-private artifacts, Briar domain fixtures, and River RNG vectors. Existing tests pass unchanged. Rollback is characterization files/tests only. |
| **8C-004** | C-04, C-05, C-08 | Add explicit collision/ambiguity characterization tests around the current local encoders: delimiters inside strings, empty vs absent values, nested choice boundaries, metadata/tag order, and fixture metadata absence. Tests should expose ambiguity without changing expected legacy output. | 8C-003 | Demonstrates why a framed versioned surface is needed; does not declare a legacy hash “wrong” or mutate any golden. |

#### Characterization packet schema

For every byte/hash/visibility/RNG surface, 8C-003 records:

```text
surface_id
owning game/tool and exact path/symbol
artifact profile and profile version, or legacy/unclassified
visibility class
rules/data/seat/hash versions
canonical-byte authority, or none
exact setup/options/seed/viewer/input vector
legacy bytes in hex or escaped byte form
legacy hash value(s)
legacy seat spellings and alias route
legacy RNG output sequence and number of next_u64 calls, where applicable
new surface/version proposed
expected classification: unchanged | parallel-new-surface | intentional-migration
migration_update_note (non-empty for intentional migration)
compatibility window
rollback commit/ticket boundary
validator commands
```

The packet is acceptance evidence, not a second source of behavior. Test code must derive or compare values through the real Rust SUT.

### Wave 1 — lowest-risk ergonomics (Batch 3)

| Work ID | Report IDs | Candidate AGENT-TASK | Dependencies | Review boundary and required evidence |
|---|---|---|---|---|
| **8C-005** | C-01 | Add `EffectEnvelope::public` and `EffectEnvelope::private_to` plus focused `engine-core` unit tests. Do not migrate a game yet. | 8C-002 | Constructor equality against current struct literals, payload move semantics, and exact scopes. No serialization/hash delta. |
| **8C-006** | C-01 | Pilot C-01 in Race to N and River Ledger only; replace matching literals/functions without changing payload construction or ordering. | 8C-005, 8C-003 | Race public effects and River public/private effects are byte/hash/visibility identical. Existing game visibility/replay suites pass. Rollback is two game call-site diffs. |
| **8C-007** | C-02 | Add strict canonical seat-ID constructor/parser/index extraction in `engine-core`, typed errors, exhaustive round-trip and rejection tests, and rustdoc defining the grammar version. | 8C-002 | `seat_0`, boundary indices, overflow, signs, whitespace, empty suffix, leading zeros, and non-ASCII digit cases pinned. No permissive `FromStr` for all `SeatId` values. |
| **8C-008** | C-02 | Replace per-game seat parsing branches in `wasm-api/src/seats.rs` with one import adapter: canonical underscore, legacy hyphen, explicit symbolic alias table. Preserve existing error classes/messages unless a named compatibility test approves a change. | 8C-007, 8C-003 | Every supported game import case has table tests; the import adapter and any newly-written/migrated output emit only underscore canonical IDs. Existing hyphen-emitting trace adapters are not silently flipped — any such change is a named ADR-0009 migration per 8C-009. Unknown/ambiguous labels reject. No TypeScript normalization added. |
| **8C-009** | C-02 | Pilot canonical-seat call sites in Race to N and River Ledger. Keep legacy trace inputs readable; do not rewrite their historical actor/viewer strings merely to adopt canonical output. | 8C-008 | Existing legacy hyphen fixtures import; canonical output round-trips; River variable-seat IDs remain identical; any hash-bearing string change is a separate named migration, not hidden in this ticket. |
| **8C-010** | C-03 | Add `game-stdlib::seat` with `SeatCount`, inclusive constraints, checked index, and next-ring-index tests. Keep errors structural and noun-free. | 8C-002 | Property tests cover all valid counts in a bounded range, wraparound, zero rejection, invalid current index, min/max inversion, and overflow-safe construction. |
| **8C-011** | C-03 | Pilot exact-two validation in Race to N and 3–6 validation/ring arithmetic in River Ledger. Map shared errors to the games’ existing diagnostic text locally. | 8C-010, 8C-003 | Setup acceptance/rejection, diagnostics, canonical seats, active/dealer progression, replay hashes, and serialization stay unchanged. No River betting/button policy moves. |

Wave 1 may be reviewed and merged independently of the hash-surface work, but all helper entries must already exist. A failure in a pilot does not justify widening the helper; it defaults to reverting that pilot or narrowing the API.

### Wave 2 — versioned bytes, action trees, and RNG

| Work ID | Report IDs | Candidate AGENT-TASK | Dependencies | Review boundary and required evidence |
|---|---|---|---|---|
| **8C-012** | C-05 | Implement `StableBytesWriter` v1 and its byte-contract tests in `engine-core`. No game migration. | 8C-003, 8C-004 | Golden byte vectors for every primitive/framing operation; field-tag ordering errors; nested/sequence/option boundaries; delimiter-collision negatives; cross-run determinism. Existing `StableSerialize`/hash behavior untouched. |
| **8C-013** | C-04, C-05 | Implement `ActionTreeEncodingVersion::V1` over `StableBytesWriter`; add kernel tests for empty, flat, multi-choice, metadata/tag, preview, and recursive trees. | 8C-012 | Exact field coverage, vector order preservation, child framing, freshness inclusion, and explicit version/domain. No hypothetical fields added. |
| **8C-014** | C-04, C-05 | Add the Race flat-tree pilot as a **parallel named surface** first. Compare legacy local action-tree hash and new v1 hash from the same tree. Migrate a single fixture/checkpoint only if its packet names the new hash-surface version and compatibility behavior. | 8C-013, 8C-003 | Legacy trace remains readable and passing. New v1 bytes/hash are pinned. If a fixture changes, before/after bytes, old/new hash, update note, validator result, and rollback point are committed together. |
| **8C-015** | C-04, C-05 | Add the Draughts Lite compound-tree pilot; prove nested path and ambiguous-string cases that the Race pilot cannot exercise. | 8C-014 | Existing multi-jump behavior/path legality unchanged; legacy hash/checkpoints still pass or receive one explicit per-surface migration; recursive v1 vector pinned. No movement/capture semantics in shared code. |
| **8C-016** | C-09 | Document current `next_index` as legacy modulo semantics and add `next_index_unbiased_v1`; add deterministic vectors that pin outputs and consumed random words, including rejection cases. | 8C-003 | Existing `next_index` vectors unchanged. Unbiased method matches the characterized local algorithm. Zero/power-of-two/non-power-of-two/large-bound cases covered. |
| **8C-017** | C-09 | Replace River Ledger’s local unbiased helper with `next_index_unbiased_v1`; no other game adopts it in 8C. | 8C-016 | Byte-for-byte identical deal/setup/replay/visibility outputs for all selected River seeds and supported counts; equal `next_u64` consumption demonstrated. Local shuffle/deal order remains in River. |

#### ADR-0009 migration protocol for 8C-014, 8C-015, and any C-08 artifact edit

Each migrated surface is one reviewable diff and follows this order:

1. Name the legacy surface and its current owner.
2. Commit characterization tests before the implementation changes its bytes.
3. Add the new writer/encoding and compute old/new values in parallel.
4. Classify the outcome as:
   - **unchanged** — identical bytes/hash and no fixture edit;
   - **parallel-new-surface** — legacy remains authoritative for old artifacts while v1 is added for new/opted-in evidence;
   - **intentional-migration** — one named artifact/surface changes with non-empty note and version anchor.
5. Keep a read-only compatibility route for the legacy surface through the completion of the C-11 retrofit waves unless an accepted later spec shortens the window with evidence.
6. Run the surface owner, `replay-check`, `fixture-check`, no-leak tests, and workspace tests.
7. Merge only with a rollback point that can remove the one migration without reverting unrelated scaffolding.

A ticket that proposes “regenerate all expected hashes” fails review by definition.

### Wave 3 — dev-only test-support infrastructure

| Work ID | Report IDs | Candidate AGENT-TASK | Dependencies | Review boundary and required evidence |
|---|---|---|---|---|
| **8C-018** | C-06 | Create `crates/game-test-support`, add it to the workspace, define public module boundaries, and extend the boundary check to reject normal/build dependency edges from production/workspace targets. | 8C-012, 8C-013 | Crate compiles; no game behavior; no dependency on game crates; game consumers can add it only under `[dev-dependencies]`; `cargo tree`/script evidence proves no normal reverse edge. |
| **8C-019** | C-07 | Implement generic no-leak matrix geometry and structured failure types in `game-test-support::no_leak`. Use closures/typed adapters for viewers, surfaces, snapshots, probes, and expectations. | 8C-018 | Unit tests include authorized, unauthorized, public-after-reveal, ignored/not-applicable, missing-canary, false-positive-resistant probe, and diagnostic rendering cases. Harness never constructs a view or decides authorization. |
| **8C-020** | C-07 | Pilot the no-leak harness in High Card Duel’s two-seat tests while retaining game-specific assertions that prove reveal semantics. | 8C-019, 8C-003 | Public observer and both seat viewers cover view, action tree, diagnostics, effects, replay/export, and any bot explanation/candidate surface present. Existing specific tests are retained or made strictly stronger. |
| **8C-021** | C-07 | Pilot the no-leak harness in River Ledger across every supported seat count 3–6 and every source-seat × viewer pair. | 8C-020 | Complete matrices pass for observer plus all seats; public/private effects, action/preview/diagnostic, view/export, showdown transitions, and bot explanation/candidates remain leak-safe. Betting/pot/showdown rules stay game-owned. |
| **8C-022** | C-08 | Implement five distinct profile-driver types/modules in `game-test-support::profiles`, plus common metadata validation primitives. Do not implement one permissive fixture union. | 8C-018, 8C-003 | Driver unit tests reject wrong profile ID/version, missing visibility, mismatched validator owner, illegal canonical-byte claims, absent migration notes, and profile-specific field misuse. |
| **8C-023** | C-08 | Adopt `replay-command-v1` in Race to N using its existing replay support and shortest-normal fixture. | 8C-022, 8C-014 | Commands/checkpoints/hashes replay through game-owned functions; legacy fixture stays readable; any metadata insertion is a named profile migration. |
| **8C-024** | C-08 | Adopt `setup-evidence-v1` in River Ledger using the standard 3-seat setup fixture; characterize any metadata edit separately from command replay or export surfaces. | 8C-022, 8C-021, 8C-017 | Setup evidence remains non-command input, canonical-byte authority stays `none` unless explicitly defined, and River setup semantics/3–6 behavior remain game-owned. |
| **8C-025** | C-08 | Adopt `public-export-v1` and `seat-private-export-v1` in Vow Tide using the existing public round-trip and all-viewers seat-private round-trip fixtures. | 8C-022 | Public export remains observer-safe; every declared seat viewer is explicitly labeled; import does not restore omniscient state; Vow bidding/deal/trick/no-leak policy remains local. This is a profile-driver pilot, not a full Vow retrofit. |
| **8C-026** | C-08 | Adopt `domain-evidence-v1` for `briar_circuit_moon.fixture.json`; use `briar_circuit_first_trick_exception.fixture.json` as a negative boundary check that the driver validates evidence shape but does not execute scoring/legality from data. | 8C-022 | Domain fixture declares visibility, validator owner, rules/data/domain version, canonical-byte authority (`none` unless explicitly defined), and migration note. Scoring and first-trick legality remain Rust code. This ticket does not retrofit Briar replay/visibility suites. |
| **8C-027** | C-08 | Add thin profile registration/dispatch to `fixture-check` and `replay-check` only where the pilots require it. Keep tools independent of `game-test-support` by default and invoke game-owned validators. | 8C-023, 8C-024, 8C-025, 8C-026 | CLI commands validate the pilot profiles; unknown profile/fields reject; no behavior-looking fixture key becomes executable; no production dependency edge appears. |

#### C-07 harness contract

The harness must accept game-owned values equivalent to:

```rust
pub enum ExposureExpectation {
    MustBeAbsent,
    MustBePresent,
    NotApplicable,
}

pub struct LeakProbe<SourceSeat, CanaryId, Canary> {
    pub source_seat: SourceSeat,
    pub canary_id: CanaryId,
    pub value: Canary,
}

pub fn assert_pairwise_no_leak<Viewer, Surface, Snapshot, ...>(
    viewers: impl IntoIterator<Item = Viewer>,
    surfaces: impl IntoIterator<Item = Surface>,
    probes: impl IntoIterator<Item = LeakProbe<...>>,
    snapshot: impl Fn(&Viewer, &Surface) -> Snapshot,
    expectation: impl Fn(&SourceSeat, &Viewer, &Surface, &CanaryId)
        -> ExposureExpectation,
    contains: impl Fn(&Snapshot, &Canary) -> bool,
) -> Result<(), PairwiseLeakFailure<...>>;
```

Exact generic shape is correctable, but ownership is not: the game supplies snapshots, probe values, reveal/authorization expectations, and the containment function. The helper only enumerates cases, compares expected exposure, aggregates failures, and reports source seat, viewer, surface, canary ID, and expectation. It must support typed viewers including a public observer without converting authorization to string heuristics.

Canaries must be generated in native test code, uniquely scoped to the source seat, and demonstrably absent from committed public/seat-private fixtures. A helper that injects private canaries into serializable state is rejected.

#### C-08 profile-driver contract

The five drivers share metadata checks but have separate semantic adapters:

| Driver | Shared work it may own | Work it must delegate |
|---|---|---|
| `ReplayCommandV1Driver` | Profile/version validation, deterministic command loop, checkpoint comparison, named hash-surface dispatch | Setup, command parsing/application, legality, state/action/effect/view hashing |
| `PublicExportV1Driver` | Public profile/visibility checks, export/import round-trip sequencing, absence of private-only metadata | Projection/redaction, public-state construction, import semantics |
| `SeatPrivateExportV1Driver` | Viewer ID/version checks, seat-scoped round-trip sequencing, pairwise invocation | Authorization, reveal timing, seat-private projection |
| `SetupEvidenceV1Driver` | Manifest/profile/version/seat-grammar shape, deterministic invocation comparison | Setup legality, options meaning, variant semantics |
| `DomainEvidenceV1Driver` | Domain profile/version/visibility/validator declarations, typed input handoff | Evaluator, allocator, topology, scoring, legality, no-leak policy, or any domain rule |

No driver reads selectors, formulas, triggers, or procedural steps from fixture data. Unknown fields reject according to the owning validator’s strictness rule. `canonical_byte_authority = none` means the driver validates structure/semantics without inventing a stable-byte claim.

### Wave 4 — pilot consolidation, C-10 affirmation, and forward seeds

| Work ID | Report IDs | Candidate AGENT-TASK | Dependencies | Review boundary and required evidence |
|---|---|---|---|---|
| **8C-028** | C-01…C-09 | Run a pilot-consolidation audit: every accepted helper has at least one real game caller, every pilot still retains its game-specific behavior assertions, and no helper accumulated policy flags to satisfy a pilot. | 8C-006…8C-027 | Call-site inventory, API review, no unused framework, no “future-proof” options, no unregistered helper. If a helper has no convincing caller, remove it rather than keep speculative API. |
| **8C-029** | C-10 | Finalize `MSC-8C-010` as rejected/local-only and review every accepted entry against the Non-Promotion List. Record explicit exclusions for deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome. | 8C-028 | Register and atlas seam remains intact; no §10A behavioral promotion debt created. No rewrite of already-shipped doctrine. |
| **8C-030** | C-11 | Seed four bounded follow-on units in `specs/README.md` (or the accepted tracker naming convention) with exact game sets, applicable-helper audit, hashes/visibility, rollback, and admission conditions. Do not write or execute the game retrofits here. | 8C-028, 8C-029 | Every official game appears exactly once in a follow-on audit; pilot games are explicitly residual-only; Gate 18 remains after closure/accepted exceptions. |
| **8C-031** | all | Full closeout: run evidence commands; update register statuses/evidence; flip 8C to `Done`; archive/record outcome per repository workflow; leave follow-on rows `Not started`. | 8C-030 | Every exit criterion below has linked evidence. No open scaffold/hash/trace/fixture/seat debt attributable to 8C. |

### Required C-11 forward-wave seeds

The following is the required default decomposition. Exact unit labels are one-line-correctable, but the bounded sets and evidence rules are not.

| Seed | Games | Primary risks | Admission/exit shape |
|---|---|---|---|
| **8C-R1 — public/fixed-seat scaffolding** | `race_to_n` (residual audit), `draughts_lite` (residual audit), `three_marks`, `column_four`, `directional_flip`, `token_bazaar` | Public envelopes, canonical seats, fixed counts, flat/compound trees, legacy local hashes | Audit C-01…C-05/C-08 applicability; migrate one surface per diff or record accepted not-applicable/exception. 8C’s Race/Draughts pilots discharge only their named surfaces. |
| **8C-R2 — two-seat hidden/reaction scaffolding** | `high_card_duel` (residual audit), `secret_draft`, `poker_lite`, `masked_claims` | Private effects/views, commitment/reaction timing, replay/export, no-leak | Adopt geometry without moving reveal/reaction policy; explicit ADR-0004 evidence. High Card’s 8C no-leak pilot does not discharge its full profile/helper audit. |
| **8C-R3 — public/co-op/asymmetric/trick support** | `plain_tricks`, `flood_watch`, `frontier_control`, `event_frontier` | Setup/profile diversity, trick/co-op/asymmetric domain fixtures, action-tree/effect hashing | Preserve behavioral atlas decisions; no trick, graph, event, budget, or scoring promotion. |
| **8C-R4 — N-seat/private/trick support** | `river_ledger` (residual audit), `briar_circuit`, `vow_tide` | 3–7 seat matrices, private hands, pass/bid/deal order, public/seat-private export, RNG-sampler divergence | Full seat/profile/no-leak audit; any RNG algorithm migration is separately versioned. The 8C River/Vow/Briar pilots discharge only their named surfaces. |

The table assigns every one of the 17 official game crates exactly once to a bounded follow-on audit. Pilot labels mean “audit only the unapplied helpers/surfaces”; they do not authorize redoing already-proven migrations. There is no unowned “remaining cleanup” bucket.

## 6. Exit criteria

The unit is `Done` only when every row below is satisfied. “Not applicable” requires an explicit rationale in the spec outcome; silence is not acceptance.

| Exit ID | Obligation | Pass condition |
|---|---|---|
| **EC-01** | Locked scope | Implementation contains C-01…C-10 infrastructure and only the named pilots; it does not execute the broad C-11 retrofit or Gate 18. Parts A/B/D are not re-authored. |
| **EC-02** | Register-first governance | `MSC-8C-001`…`MSC-8C-010` exist with every Entry Schema field. Each implemented helper is `accepted` with evidence; C-10 is explicitly rejected/local-only. No unregistered shared helper landed. |
| **EC-03** | Correct ownership | Kernel ergonomics are in `engine-core`, seat geometry in `game-stdlib`, test proof geometry in `game-test-support`, and legacy transport aliases in `wasm-api`. No ownership-matrix exception remains implicit. |
| **EC-04** | C-01 behavior neutrality | Race and River effect envelopes preserve scope, payload, order, serialized bytes, hashes, and visibility. Public/private filtering remains unchanged. |
| **EC-05** | C-02 grammar | Rust is the sole canonical parse/format authority for `seat_<zero-based>`. Legacy aliases are import-only, explicit, tested, and never emitted. Unknown/ambiguous/role labels are not guessed. |
| **EC-06** | C-02 compatibility | Every legacy pilot fixture remains readable during the compatibility window. Any intentional seat-byte change has a named version, before/after evidence, non-empty note, and rollback point. |
| **EC-07** | C-03 structural-only API | Fixed/ranged seat validation and ring arithmetic are shared; diagnostics and game order policy remain local. Race exact-two and River 3–6 behavior, setup errors, and progression are unchanged. |
| **EC-08** | C-05 byte contract | `StableBytesWriter` v1 has published in-code framing, exact golden vectors, domain/version separation, collision negatives, deterministic ordering, and no derive/reflection/unordered-map/floating-point escape hatch. |
| **EC-09** | C-04 tree coverage | Action-tree v1 covers every current contract field and recursive child in deterministic order. Race flat and Draughts compound vectors pass; no nonexistent field was invented. |
| **EC-10** | Hash migration discipline | Every affected surface is classified as unchanged, parallel-new-surface, or intentional migration. Existing hashes are unchanged **or** the exact per-surface migration note, version, compatibility window, validators, and rollback are present. No blanket golden regeneration occurred. |
| **EC-11** | Legacy readability | All pre-8C pilot traces/fixtures selected in characterization still validate through their legacy authority. A new v1 surface does not retroactively reinterpret old bytes. |
| **EC-12** | C-09 legacy stability | `DeterministicRng::next_index` returns the same values and consumes the same words as before. Its modulo semantics are documented rather than silently “fixed.” |
| **EC-13** | C-09 unbiased sampler | `next_index_unbiased_v1` is statistically unbiased by construction, deterministic for a fixed stream, and pinned for rejection consumption. River’s extracted caller is byte/trace identical to its former local implementation. No shuffle/deal policy moved. |
| **EC-14** | Dev-only dependency boundary | `game-test-support` has no normal/build reverse dependency from production, WASM, browser, tool, or game library targets. Game consumers list it only under `[dev-dependencies]`. The boundary is enforced by a script/test, not convention alone. |
| **EC-15** | C-07 geometry-only harness | The harness enumerates source × viewer × surface and reports structured failures but does not create projections, infer reveal timing, authorize facts, redact output, or execute game rules. |
| **EC-16** | Two-seat no-leak pilot | High Card Duel passes the full declared matrix for observer and both seats across every applicable surface. Existing reveal-specific assertions are preserved or strengthened. |
| **EC-17** | N-seat no-leak pilot | River Ledger passes all supported counts 3, 4, 5, and 6, all source seats, observer plus every seat viewer, and all applicable surfaces. No private fact leaks through view, action tree, preview, diagnostic, effect, export, replay, bot explanation/candidates, logs, or test IDs. |
| **EC-18** | Canary hygiene | No test canary appears in committed public, viewer-scoped, seat-private, trace, fixture, browser, DOM, storage, log, or accessibility artifact. |
| **EC-19** | C-08 profile separation | All five ADR-0009 profile drivers exist as distinct typed paths and reject misuse. No permissive union silently accepts fields from another profile. |
| **EC-20** | C-08 real callers | Race drives `replay-command-v1`; River drives setup evidence; Vow drives public and seat-private export; Briar drives domain evidence. Each invokes game-owned behavior and the owning validator. |
| **EC-21** | ADR-0004 preservation | Public export remains observer-safe; seat-private export is explicitly viewer-labeled; import reconstructs a viewer-scoped observation timeline rather than omniscient state. Hidden bot candidates/explanations remain protected. |
| **EC-22** | Fixture/data boundary | Profile drivers treat artifacts as input/expected evidence only. No selectors, triggers, formulas, executable conditions, or procedural behavior enter fixture data. |
| **EC-23** | Tool ownership | `replay-check` and `fixture-check` validate all pilot artifacts, reject unknown/mismatched profile metadata, and retain canonical ownership. Thin adapters do not relocate game logic. |
| **EC-24** | Noun-free kernel | `scripts/boundary-check.sh` passes; new `engine-core` identifiers and docs contain no forbidden game-mechanic nouns or policy types. |
| **EC-25** | Full test health | `cargo test --workspace --all-targets` and focused pilot suites pass without ignored/deleted/weakened tests. Any pre-existing failure is classified under the failing-test protocol. |
| **EC-26** | No performance gate drift | Existing benchmark sources/thresholds are untouched unless a characterized helper regression requires a separately reviewed correction. No threshold is relaxed to close 8C. |
| **EC-27** | C-10 boundary | The register and atlas still distinguish behavior-free scaffolding from behavioral mechanics. Deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome remain local or atlas-governed. |
| **EC-28** | Follow-on ownership | Four bounded C-11 seeds cover every official game exactly once, with pilot games limited to residual audits, with explicit hash/visibility/rollback obligations. They remain unimplemented. |
| **EC-29** | Documentation truth | Register evidence and `specs/README.md` status/sequencing are current. No foundation, ADR, architecture, or roadmap text was edited merely to record progress. `apps/web/README.md` is explicitly not applicable. |
| **EC-30** | Gate 18 admission | 8C’s own scaffold/hash/profile/seat obligations are closed, and the tracker states that Gate 18 remains after the seeded C-11 waves are closed, not applicable, or explicitly excepted under accepted law. |

## 7. Acceptance evidence

### 7.1 Required command evidence

Run from the repository root. Record command, exit status, and the relevant output summary in the spec outcome or canonical evidence receipt.

```bash
cargo fmt --all -- --check
cargo test -p engine-core
cargo test -p game-stdlib
cargo test -p game-test-support
cargo test -p wasm-api
cargo test -p race_to_n
cargo test -p draughts_lite
cargo test -p high_card_duel
cargo test -p river_ledger
cargo test -p vow_tide
cargo test -p briar_circuit
cargo test --workspace --all-targets

cargo run -p replay-check -- --game race_to_n --all
cargo run -p replay-check -- --game draughts_lite --all
cargo run -p replay-check -- --game high_card_duel --all
cargo run -p replay-check -- --game river_ledger --all
cargo run -p replay-check -- --game vow_tide --all
cargo run -p replay-check -- --game briar_circuit --all

cargo run -p fixture-check -- --game race_to_n
cargo run -p fixture-check -- --game river_ledger
cargo run -p fixture-check -- --game vow_tide
cargo run -p fixture-check -- --game briar_circuit

bash scripts/boundary-check.sh
cargo tree --workspace -e normal --invert game-test-support
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
```

The `cargo tree` result must show no normal reverse consumer of `game-test-support`; a dedicated boundary-script assertion is the durable pass/fail authority. Development edges from the named game test targets are expected and should be recorded separately with `cargo tree -e dev` when useful.

`check-catalog-docs.mjs` is a regression guard only. This unit adds no game and has no web catalog closeout.

### 7.2 Required focused test evidence

| Evidence ID | Required proof |
|---|---|
| **EV-REG** | Register entries before/after, with reviewer and final state. |
| **EV-EFF** | Constructor-vs-literal equality tests plus Race/River replay and visibility results. |
| **EV-SEAT** | Canonical parse/format round-trip table, strict rejection table, alias-import table, output-canonicalization table, and legacy fixture-read results. |
| **EV-COUNT** | Seat-count/range/ring property tests plus Race and River setup/diagnostic/replay comparisons. |
| **EV-BYTES** | Stable-writer specification vectors in escaped/hex form, including nested and ambiguity cases. |
| **EV-TREE-FLAT** | Race legacy and v1 action-tree bytes/hashes for the same flat tree. |
| **EV-TREE-COMPOUND** | Draughts legacy and v1 bytes/hashes for the same multi-jump compound tree. |
| **EV-HASH-MIG** | One receipt per intentionally changed artifact/surface: old/new bytes and hashes, profile/version, update note, compatibility window, validators, rollback. |
| **EV-RNG-LEGACY** | Existing `next_index` seed/bound/output/consumption vectors before and after. |
| **EV-RNG-V1** | Unbiased rejection vectors, including at least one rejected draw, and River local-vs-shared equivalence. |
| **EV-DEPS** | Production dependency graph proving no normal/build edge to `game-test-support`. |
| **EV-NOLEAK-2** | High Card Duel source × viewer × surface case count and zero failures. |
| **EV-NOLEAK-N** | River case counts for 3/4/5/6 seats and zero failures, including observer. |
| **EV-PROFILES** | Positive and negative conformance table for all five profile drivers and each pilot artifact. |
| **EV-TOOLS** | `replay-check`/`fixture-check` pilot output and strict rejection tests. |
| **EV-FULL** | Workspace, boundary, links, and catalog guard results. |
| **EV-FORWARD** | Tracker rows for all C-11 waves with game coverage and Gate 18 interlock. |

### 7.3 Hash/fixture evidence rules

- Before/after values must be produced by tests or validators, not copied from an editor or hand-calculated without a check.
- A fixture whose canonical-byte authority is `none` still needs semantic/profile characterization if its text changes, but it must not acquire an invented stable hash merely for uniformity.
- Old and new hash surfaces must have distinct names/versions in receipts. “Action tree hash” without a surface version is insufficient once v1 exists.
- Read compatibility and write authority are separate. A legacy parser may remain read-only while new evidence writes canonical v1.
- A compatibility window ends only in a named C-11 closeout or later accepted spec. It cannot disappear in an unrelated cleanup.
- Hash and fixture failures are investigated as SUT-vs-test questions. A valid golden that catches a semantic drift is not “stale” merely because a shared helper changed.

### 7.4 Benchmarks and browser evidence

- **Game benchmark thresholds:** `not applicable` as an intended deliverable. No behavioral algorithm is being optimized or altered. Existing benches must continue to compile under `--all-targets`; threshold files remain unchanged.
- **Browser/e2e:** `not applicable` as a positive feature surface. The WASM seat adapter and all catalog guards must remain green. No browser snapshot is an authority for seat parsing, legality, visibility, or hashing.
- **Manual review:** required for API noun-freedom, register completeness, byte-contract readability, C-10 exclusions, and proof that pilot-specific policy did not leak into shared APIs.

### 7.5 External prior art informing the design

External sources inform design technique only; they do not establish target-repository state.

1. **Canonical bytes must be constructed, not assumed.** RFC 8949’s deterministic CBOR rules require definite lengths and deterministic ordering; Borsh likewise makes endianness, length prefixes, field order, and unordered-container ordering explicit.[^rfc8949][^borsh] Rulepath should adopt those principles in its small domain-specific writer, not adopt either format wholesale.
2. **“Deterministic serializer” is not the same as a canonical long-lived hash surface.** Protocol Buffers explicitly warns that deterministic serialization is not canonical and that serialized hashes can be fragile across schema, implementation, build, and library changes.[^protobuf-canonical] This supports an explicit Rulepath byte version/domain, reviewed field set, and golden vectors rather than hashing a general serializer’s output.
3. **Compatibility should be staged and named.** Git’s SHA-256 transition design separates storage format, mappings, input/output modes, and non-goals rather than silently changing all object names at once.[^git-transition] The direct lesson is dual-read/parallel-surface migration with a bounded transition mode and no unrelated format “fixes.”
4. **A Rust test-support crate can remain outside production builds when consumers use development dependencies.** Cargo documents that dev-dependencies are used for tests/examples/benchmarks, not normal package builds, and are not propagated to downstream packages; `cargo tree -e normal` can inspect the relevant edge class.[^cargo-dev][^cargo-tree] Rulepath still needs a repository guard because manifest intent can drift.
5. **String parse/format round-trips require an explicit contract.** Rust’s `FromStr` documentation warns that `Display` and parsing are not automatically lossless or mutually compatible.[^rust-fromstr] Rulepath therefore documents one strict machine grammar and isolates legacy aliases at import rather than making permissiveness global.
6. **Modulo reduction is generally biased; rejection sampling is a real semantic choice.** Lemire’s bounded-integer analysis describes the need to map fixed-width random words into `[0, s)` without bias and reviews rejection methods.[^lemire] Because rejection can consume additional random words, Rulepath must version and characterize it rather than replace the legacy modulo method in place.

**Deliberate non-adoptions:** no CBOR/Borsh/Protobuf/bincode dependency becomes the canonical hash authority in 8C; no Git-style global object-ID migration is attempted; no generic random distribution/shuffle framework is introduced. The external lesson is explicit formats, versioned migration, dependency isolation, and consumption-aware testing—not architectural expansion.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Governing alignment table

| Authority | Principle engaged | 8C stance | Proof / stop boundary |
|---|---|---|---|
| `FOUNDATIONS.md` §2 | Rust is behavior authority | All canonical parsing, hashing, RNG mapping, projection inputs, replay, and test-driving behavior remain Rust. | Stop if TypeScript is asked to normalize seat IDs, decide visibility, or calculate a hash. |
| `FOUNDATIONS.md` §3 | `engine-core` is a generic contract kernel | Only existing kernel nouns receive ergonomics: seat ID, effect envelope, action tree, replay/hash/serialization, deterministic RNG. | `boundary-check.sh`; reject any board/card/deck/hand/pot/trick/team/etc. identifier or policy. |
| `FOUNDATIONS.md` §4 | `game-stdlib` is earned; mechanical scaffolding has a separate lane | Only count/ring geometry lands in `game-stdlib`; every candidate is registered first. Behavioral third-use law remains untouched. | Register entries and atlas review; stop if a helper needs mechanic flags. |
| `FOUNDATIONS.md` §5 | Static data is typed content, not behavior | Profile drivers validate inputs/expected evidence and reject behavior-looking keys. | Stop if a fixture describes executable selectors, conditions, formulas, triggers, or steps. |
| `FOUNDATIONS.md` §11 | Determinism, no leak, registered scaffolding, typed data, bounded work | Versioned byte surfaces, characterization-first migration, full pairwise matrices, dev-only harnesses, bounded pilots. | EC-02, EC-10…EC-25. |
| `FOUNDATIONS.md` §12 | Stop conditions | The work breakdown contains explicit stop/revert points for nouns, leaks, procedural data, unbounded cleanup, and invalid goldens. | Any stop condition halts the affected ticket; it does not justify a silent workaround. |
| `FOUNDATIONS.md` §13 | ADR triggers | ADR 0008 already authorizes the scaffolding category; ADR 0009 authorizes profile/hash migration; ADR 0004 governs hidden export. | A change outside those named decisions—new kernel responsibility, new visibility meaning, new hand-authored format, new hash algorithm—requires a new accepted ADR first. |
| `ARCHITECTURE.md` §3A | Reuse Ownership Matrix | Kernel ergonomics / game-layer plumbing / dev-only proof / browser adapter are separated exactly by owner. | Home-decision table in §4.2 and dependency graph evidence. |
| `ARCHITECTURE.md` §§5–9 | Action, view, effects, replay, determinism | C-04/C-05 encode the existing action contract; C-01 preserves effect meaning; C-07/C-08 invoke existing projections/replay. | No helper may reinterpret choice meaning, effect meaning, visibility, or ordering. |
| `ENGINE-GAME-DATA-BOUNDARY.md` §2A | Four reuse lanes; narrowest layer wins | Each candidate is classified before code. Behavioral mechanics and typed content remain separate from scaffolding. | Register classification and C-10 rejection entry. |
| `ENGINE-GAME-DATA-BOUNDARY.md` §3 | Generic vs mechanic vocabulary | API names remain noun-free at the kernel. | Boundary script plus reviewer checklist. |
| `ENGINE-GAME-DATA-BOUNDARY.md` §§5–6 | Allowed/forbidden data | Profile metadata is declarative; domain semantics stay in Rust. | `fixture-check` strictness and negative driver tests. |
| ADR 0008 | Mechanical-scaffolding governance | Second/third-copy pressure is resolved through complete register entries, correct homes, behavior neutrality, deterministic/leak-safe APIs, and complete pilot migration. | `MSC-8C-001`…`010`; no speculative API survives without callers. |
| ADR 0009 | Fixture/hash taxonomy | Every artifact has a profile, visibility class, validator owner, version anchors, canonical-byte authority, and migration note when changed. | C-08 drivers and per-surface receipts. |
| ADR 0004 | Hidden replay/export taxonomy | Public remains observer-safe; seat-private is viewer-scoped; import is an observation timeline, not omniscient state. | High Card/River no-leak and export round-trip evidence. |
| `MECHANIC-ATLAS.md` | Behavioral third-use gate and promotion debt | No behavioral primitive is promoted; §10A remains empty. | C-10 review; stop if a pilot needs deal/reveal/trick/team/etc. policy. |
| `TESTING-REPLAY-BENCHMARKING.md` | Shared test-support and hash migration law | Test geometry is shared, game assertions remain; each hash migration is named and isolated. | Characterization packet, compatibility window, validators, no bulk regen. |
| `TRACE-SCHEMA-v1.md` | Existing command/replay bytes | V1 fields are not silently changed. New profile/hash metadata is versioned outside or through an explicitly governed artifact migration. | Legacy traces remain readable; any trace root change is named. |
| `WASM-CLIENT-BOUNDARY.md` | Canonical seat grammar and aliases | Canonical underscore output; aliases accepted only at import with bounded mapping. | C-02 tests; no browser repair. |
| `AGENT-DISCIPLINE.md` | Bounded tasks and failing-test/scaffold protocols | One ticket per reviewable diff; characterize, migrate a pilot, compare, then proceed. | Work IDs 8C-001…031 and rollback boundaries. |

### 8.2 Why each move is behavior-neutral

- **C-01** replaces equivalent envelope literals. It does not decide who may see an effect; `VisibilityScope` and the game payload already decide that.
- **C-02** defines machine identity syntax. It does not define roles, teams, dealer order, actor authorization, or display labels.
- **C-03** validates structural cardinality and modular index geometry. It does not define whose turn follows, except where a game explicitly calls the generic arithmetic as part of its own policy.
- **C-04/C-05** encode an existing tree/value contract. They do not choose legal actions, labels, metadata, preview content, or child branches.
- **C-06/C-07/C-08** execute test orchestration around game-owned functions. They do not become a runtime game framework.
- **C-09** maps an already-produced random word to an index under an explicitly named algorithm. It does not choose a collection, mutation order, dealing schedule, or reveal policy.
- **C-10** rejects policy-shaped reuse through this lane.

### 8.3 Stop-and-reassess triggers specific to this unit

Stop the affected ticket immediately if:

1. A helper requires an option like `is_trick_game`, `private_hand`, `all_in`, `team_count`, `pass_direction`, `reveal_on`, or any equivalent mechanic policy flag.
2. A strict canonical seat parser cannot coexist with a game’s semantic seat/role label without loss; leave that label game-local and route it through an explicit alias map.
3. The v1 byte writer cannot encode the action contract without depending on reflection, unstable map order, implicit struct layout, or platform size.
4. A new action-tree hash would overwrite an existing expected hash without a named parallel/migration surface.
5. An RNG migration changes a pilot’s `next_u64` consumption or downstream setup bytes unexpectedly.
6. The no-leak harness needs to know when a card, bid, commitment, candidate, or explanation becomes public.
7. A profile driver needs to execute behavior from fixture keys.
8. `game-test-support` appears in a normal/build dependency tree.
9. A valid game-specific test must be deleted or made less specific to use a generic helper.
10. A required target-repository claim depends on an unfetched/unverified file or an uncharacterized surface.

The default response is to narrow or reject the helper, revert the pilot, and update the register rationale. It is not to widen shared code.

## 9. Forbidden changes

The following are hard prohibitions for every 8C ticket:

1. **No new game/mechanic vocabulary in `engine-core`:** no board, grid, card, deck, hand, deal, bid, trump, trick, pass, faction, team, partnership, pot, bet, all-in, evaluator, score, winner, graph, site, territory, resource, reaction, or equivalent noun/policy.
2. **No behavioral promotion:** legality, setup policy, reveal timing, projection/redaction, effect meaning, scoring, outcome, bot policy, and diagnostics stay with games.
3. **No macro-generated seat enums:** Gate 18’s partnership evidence must arrive before reconsideration.
4. **No permissive global seat parser:** import aliases do not become the canonical `SeatId` grammar; role/team labels are not silently coerced.
5. **No canonical hyphen output:** `seat-<n>` is read compatibility only.
6. **No silent trace/fixture rewrite:** old actor/viewer strings, roots, hashes, or metadata are not mass-normalized.
7. **No unversioned persisted action-tree hash:** every new canonical tree hash names its encoding version.
8. **No general serializer as hash authority:** no derive/reflection “stable hash,” no implicit `serde`/JSON/bincode/Borsh/CBOR bytes, no struct-memory hashing.
9. **No unordered-map hashing:** callers must supply an explicit stable order.
10. **No change to `HashValue::from_stable_bytes`:** a hash-algorithm transition is a different ADR/unit.
11. **No mutation of legacy `next_index`:** changing modulo to rejection sampling in place is forbidden.
12. **No shared shuffle/deal helper:** even a noun-free permutation helper is deferred until all candidate games’ bytes/consumption are compared in a future decision.
13. **No test-support production edge:** not from game libraries, `engine-core`, `game-stdlib`, `wasm-api`, browser code, or normal CLI builds.
14. **No projection in the harness:** it cannot redact, authorize, reveal, construct views, or infer public facts.
15. **No leak canary in committed artifacts:** canaries remain test-only and non-serializable to public/viewer fixtures.
16. **No fixture DSL:** profile drivers do not interpret selectors, formulas, expressions, triggers, conditions, or procedures.
17. **No weakened evidence:** no deleted cases, ignored tests, reduced seat counts, removed observer, relaxed unknown-field checks, or replaced game assertion with only a generic smoke test.
18. **No blanket golden/hash regeneration:** each changed artifact is separately justified and reviewable.
19. **No unrelated cleanup:** no formatting sweep, naming campaign, module reorganization, dependency upgrades, or “fix all duplicated code” effort.
20. **No Gate 18 implementation:** no Spades rules, partnerships, team scoring, or UI grouping.
21. **No full C-11 sweep:** only the named pilots and forward seeds are in 8C.
22. **No progress edits to `docs/ROADMAP.md`:** the living tracker is `specs/README.md`.
23. **No `apps/web/README.md` content edit for closeout:** there is no new web-exposed game or renderer.

## 10. Documentation updates required

### Required documentation amendments

The code unit requires the following repository documentation changes and no others by default.

#### A. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — required

Before implementation, add the ten full entries listed in §4.4. During closeout:

- change implemented entries from `candidate` to the final accepted state only after their evidence passes;
- attach exact pilot paths, hashes/visibility impact, migration receipts, and commands;
- retain `MSC-8C-010` as rejected/local-only with the next mechanic-gate review trigger;
- record any candidate narrowed or rejected during pilots honestly rather than editing the schema to make it fit.

This is the principal doctrine-facing amendment. It is an expected use of the register shipped by 8M, not a re-authoring of that document.

#### B. `specs/README.md` — required

At spec authoring:

- replace row 8C’s `_(seed; unwritten)_` with the new spec link;
- change status `Not started` → `Planned`;
- remove the stale `Blocked` assertion because ADRs 0008/0009 are accepted and 8M is done;
- describe 8C accurately as shared infrastructure plus bounded pilots, with remaining C-11 retrofits seeded forward.

Before Gate 18 admission:

- insert the four bounded C-11 follow-on rows after 8C and before Gate 18, or use an equivalent accepted decomposition that preserves exact game ownership;
- update Gate 18’s interlock to require those rows closed, explicitly not applicable, or accepted-excepted;
- keep Gate 18 `Not started` until those conditions and the partnership/trick atlas interlock are satisfied.

At 8C closeout:

- change 8C `In progress` → `Done` only after all §6 criteria pass;
- include a concise outcome/evidence link and leave follow-on rows `Not started`.

#### C. `specs/unit-8c-mechanical-scaffolding-code-extraction.md` — required

This file is the canonical 12-section spec. During implementation, update status and outcome according to the repository workflow. The outcome must link the characterization/migration evidence, register entries, and C-11 seeds.

#### D. Code/API rustdoc — required

Document in the owning modules:

- canonical seat grammar and alias boundary;
- stable-byte v1 framing and field-order rules;
- action-tree encoding v1 field coverage;
- legacy modulo and unbiased-v1 RNG semantics, including consumption consequences;
- `game-test-support`’s dev-only/no-behavior contract;
- no-leak/profile driver ownership boundaries.

These are API contracts, not foundation amendments.

#### E. `docs/ROADMAP.md` — no amendment required

The existing roadmap already requires pre-gate scaffold/trace/fixture/seat/hash debt closure. `specs/README.md` explicitly says it, not the roadmap, records progress. Do not edit the roadmap merely to mark 8C or its waves complete.

Amend `docs/ROADMAP.md` only if implementation uncovers a genuine normative contradiction in the admission rule. Such a contradiction must name the exact section and be resolved under authority/ADR rules before proceeding; none was found during this design.

#### F. Foundation, architecture, boundary, ADRs, fixture contract, trace schema, atlas — no amendment required

No amendment is currently needed to:

- `docs/FOUNDATIONS.md`;
- `docs/ARCHITECTURE.md`;
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`;
- ADR 0004, 0008, or 0009;
- `docs/EVIDENCE-FIXTURE-CONTRACT.md`;
- `docs/TRACE-SCHEMA-v1.md`;
- `docs/MECHANIC-ATLAS.md`.

They already authorize and constrain the work. C-10 is satisfied by a register decision affirming the list already shipped by 8M, not by duplicating or moving that list.

#### G. Web documentation — explicitly not applicable

`apps/web/README.md` is **not applicable** as a closeout surface because 8C exposes no new game, catalog entry, renderer, or smoke layer. `node scripts/check-catalog-docs.mjs` still runs as a regression guard.

**Documentation-amendment conclusion:** beyond register entries, the new spec/API docs, and `specs/README.md` status/sequencing changes, no `docs/**` normative amendment is required. Any implementation pressure to change accepted doctrine is a stop condition, not permission for a silent design deviation.

## 11. Sequencing

### 11.1 Predecessor

**Unit 8M — reuse-doctrine and evidence realignment: `Done`.** It supplied the accepted governance, register, fixture profiles, evidence receipt, and template/document baseline. 8C must consume that baseline and must not repeat it.

### 11.2 Internal dependency order

```text
8C spec/index admission
    ↓
register entries
    ↓
characterization packets and legacy ambiguity tests
    ├── C-01 / C-02 / C-03 low-risk APIs and pilots
    └── C-05 stable-byte writer
            ↓
        C-04 action-tree encoding
            ↓
        Race flat + Draughts compound byte/hash pilots
            ↓
        C-09 versioned sampler + River equivalence pilot
            ↓
        C-06 game-test-support dependency boundary
            ↓
        C-07 no-leak geometry
            ↓
        High Card + River no-leak pilots
            ↓
        C-08 profile drivers
            ↓
        Race / River / Vow / Briar profile pilots and tool adapters
            ↓
        C-10 final boundary audit
            ↓
        C-11 follow-on seeds
            ↓
        8C closeout
```

Low-risk Wave 1 changes may proceed in parallel after register/characterization, but no byte/profile pilot may bypass the stable-writer and migration gates.

### 11.3 Successors

1. **8C-R1 — flat/public scaffolding retrofits.**
2. **8C-R2 — two-seat hidden/reaction retrofits.**
3. **8C-R3 — public/co-op/asymmetric/trick support retrofits.**
4. **8C-R4 — N-seat/private/trick support retrofits.**
5. **Gate 18 — Spades/partnerships.**

A successor wave may split further if one migration cannot remain a reviewable diff. It may not merge waves into a repository-wide sweep.

### 11.4 Gate 18 admission rule

Gate 18 is admitted only when:

- 8C is `Done` with every helper registered and every pilot evidence packet accepted;
- every C-11 follow-on row is `Done`, explicitly not applicable, or has an accepted exception naming game, helper, risk, evidence, and next review trigger;
- no open mechanical-scaffolding, trace/profile, fixture, seat-grammar, hash, or §10A behavioral promotion debt remains;
- canonical seat grammar is fixed without collapsing partnership/team semantics into seat identity;
- the partnership/trick-taking atlas interlock is resolved in Gate 18’s own spec;
- teams/partnership scoring and UI grouping remain game-local unless separately authorized.

8C closure is necessary but, under this design’s locked sequence, not by itself sufficient to start Gate 18; the bounded C-11 waves close the migration set that 8C deliberately does not execute.

## 12. Assumptions

Each assumption is one-line-correctable by the maintainer before its first dependent ticket. An override must still satisfy the authority order, register decision rule, and migration evidence.

| ID | Assumption |
|---|---|
| **A-01 — bounded delegation: pilots** | Required default pilot set is Race to N, Draughts Lite, High Card Duel, River Ledger, a profile-only Vow Tide adoption, plus a fixture-only Briar Circuit domain-profile pilot. A replacement must cover every risk/profile shape with no larger scope. |
| **A-02 — bounded delegation: homes** | Required defaults are the landing homes in §4.2. A different home must show why it is narrower and lawful under the architecture ownership matrix. |
| **A-03 — API naming** | Exact method/type/module names in §4.3 may be corrected for established repository naming, but permissiveness, ownership, and version explicitness may not change. |
| **A-04 — byte framing** | The proposed `RPSB` v1 framing is the required design default. Constants/tag widths may change during the pre-adoption byte-contract review; after a pilot artifact adopts v1, changes require v2 or an explicit migration. |
| **A-05 — hash algorithm** | Existing `HashValue::from_stable_bytes` remains the algorithm; 8C versions input bytes/surfaces, not the hash primitive. |
| **A-06 — compatibility window** | Legacy pilot artifacts remain readable through completion of all C-11 waves. A shorter window requires an accepted spec with inventory/evidence. |
| **A-07 — tool dependencies** | `replay-check` and `fixture-check` do not normally depend on `game-test-support`; thin duplicated dispatch is preferable to contaminating the production/normal graph. A tool-only normal edge may be proposed only if the boundary law explicitly permits and guards it. |
| **A-08 — domain pilot** | `briar_circuit_moon.fixture.json` is the default real `domain-evidence-v1` pilot because it is explicitly scoring evidence; `briar_circuit_first_trick_exception.fixture.json` supplies a useful negative policy boundary. Neither promotes scoring/legality. |
| **A-09 — RNG pilot** | River Ledger is the only 8C RNG adoption because its local rejection sampler is semantically and byte-consumption equivalent to the proposed shared method. Briar/Vow modulo callers remain for a future explicit migration. |
| **A-10 — profile write authority** | New or migrated evidence writes canonical profile metadata; legacy artifacts may be read through bounded adapters. Read compatibility does not make legacy spellings canonical output. |
| **A-11 — no new ADR** | Accepted ADRs 0008/0009/0004 fully authorize the scoped work. A new ADR is required only if implementation changes kernel responsibility, hash algorithm, visibility meaning, hand-authored format, or other §13 trigger beyond those decisions. |
| **A-12 — documentation** | No normative `docs/**` amendment is needed beyond using the register; the tracker/spec/API documentation changes in §10 are sufficient unless implementation uncovers a named contradiction. |

## Outcome

Completed: 2026-06-22

Unit 8C is `Done`. C-01...C-09 landed as accepted mechanical-scaffolding entries,
C-10 remains `rejected / local-only`, and C-11 is seeded forward as four bounded
unimplemented rows. No Gate 18 game work was executed.

Closeout evidence:

| Evidence | Result |
|---|---|
| Register | [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`](../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md) records `MSC-8C-001`...`MSC-8C-009` as accepted and `MSC-8C-010` as rejected/local-only, with final closeout evidence. |
| Characterization | [`reports/8c-mechanical-scaffolding-characterization.md`](../../reports/8c-mechanical-scaffolding-characterization.md) plus archived tickets `UNI8CMECSCA-003`...`UNI8CMECSCA-004` preserve the legacy byte/hash ambiguity and pre-migration evidence boundary. |
| Implementation and pilots | Archived tickets `UNI8CMECSCA-005`...`UNI8CMECSCA-027` record the helper implementations, pilots, migration receipts, replay/fixture evidence, and dependency checks. |
| Consolidation and boundary | Archived tickets `UNI8CMECSCA-028`...`UNI8CMECSCA-030` record caller inventory, C-10 rejected/local-only closeout, and C-11 follow-on ownership. |
| Forward rows | [`specs/README.md`](../../specs/README.md) includes 8C-R1...8C-R4 as `Not started`, covers every official game exactly once, and keeps Gate 18 blocked on their closure / not-applicable / accepted-excepted outcomes. |

Exit-criteria mapping:

| Exit criteria | Evidence |
|---|---|
| EC-01...EC-03 | Locked scope, register-first governance, and ownership are covered by the register, archived tickets `UNI8CMECSCA-001`...`UNI8CMECSCA-004`, `bash scripts/boundary-check.sh`, and `cargo tree --workspace -e normal --invert game-test-support`. |
| EC-04...EC-13 | C-01/C-02/C-03/C-04/C-05/C-09 behavior, compatibility, byte/hash, legacy-read, and RNG evidence is covered by archived tickets `UNI8CMECSCA-005`...`UNI8CMECSCA-017`, focused shared/game crate tests, and pilot `replay-check` / `fixture-check` runs. |
| EC-14...EC-18 | Dev-only dependency and no-leak geometry are covered by archived tickets `UNI8CMECSCA-018`...`UNI8CMECSCA-021`, `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`, and High Card/River focused tests. |
| EC-19...EC-23 | Profile separation, real callers, ADR-0004 preservation, fixture/data boundary, and tool ownership are covered by archived tickets `UNI8CMECSCA-022`...`UNI8CMECSCA-027`, tool strict-rejection tests, and all pilot `replay-check` / `fixture-check` runs. |
| EC-24...EC-27 | Noun-free kernel, full test health, benchmark-threshold non-drift, and C-10 boundary are covered by `bash scripts/boundary-check.sh`, `cargo test --workspace --all-targets`, the register, and archived tickets `UNI8CMECSCA-028`...`UNI8CMECSCA-029`. |
| EC-28...EC-30 | Follow-on ownership, documentation truth, and Gate 18 admission are covered by archived ticket `UNI8CMECSCA-030`, [`specs/README.md`](../../specs/README.md), `node scripts/check-doc-links.mjs`, and `node scripts/check-catalog-docs.mjs`. |

Final verification commands all exited successfully:

1. `cargo fmt --all -- --check`
2. `cargo test -p engine-core`
3. `cargo test -p game-stdlib`
4. `cargo test -p game-test-support`
5. `cargo test -p wasm-api`
6. `cargo test -p race_to_n`
7. `cargo test -p draughts_lite`
8. `cargo test -p high_card_duel`
9. `cargo test -p river_ledger`
10. `cargo test -p vow_tide`
11. `cargo test -p briar_circuit`
12. `cargo test --workspace --all-targets`
13. `cargo run -p replay-check -- --game race_to_n --all`
14. `cargo run -p replay-check -- --game draughts_lite --all`
15. `cargo run -p replay-check -- --game high_card_duel --all`
16. `cargo run -p replay-check -- --game river_ledger --all`
17. `cargo run -p replay-check -- --game vow_tide --all`
18. `cargo run -p replay-check -- --game briar_circuit --all`
19. `cargo run -p fixture-check -- --game race_to_n`
20. `cargo run -p fixture-check -- --game river_ledger`
21. `cargo run -p fixture-check -- --game vow_tide`
22. `cargo run -p fixture-check -- --game briar_circuit`
23. `bash scripts/boundary-check.sh`
24. `cargo tree --workspace -e normal --invert game-test-support`
25. `node scripts/check-doc-links.mjs`
26. `node scripts/check-catalog-docs.mjs`

`cargo test --workspace --all-targets` executed benchmark binaries as test
targets and exited successfully. Some benchmark JSON rows printed local
threshold `pass: false` values for pre-existing benchmark floors; no benchmark
source or threshold was edited by 8C closeout, and benchmark threshold
recalibration remains out of scope.

`apps/web/README.md`, foundation docs, ADRs, architecture docs, roadmap docs,
mechanic atlas, code, tests, fixtures, hashes, WASM surfaces, and runtime
behavior were intentionally unchanged during this capstone.

[^rfc8949]: RFC 8949, *Concise Binary Object Representation (CBOR)*, especially §4.2 deterministic encoding requirements: definite lengths and deterministic map-key order. <https://www.rfc-editor.org/rfc/rfc8949>
[^borsh]: Borsh specification: canonical/deterministic bytes, little-endian integers, length-prefixed dynamic containers, deterministic ordering for unordered containers, and declared struct/enum order. <https://borsh.io/>
[^protobuf-canonical]: Protocol Buffers documentation, *Proto Serialization Is Not Canonical*: deterministic serialization is not canonical, and hashes of serialized messages are fragile across changes. <https://protobuf.dev/programming-guides/serialization-not-canonical/>
[^git-transition]: Git documentation, *hash-function-transition*: distinct repository format, bidirectional mapping, staged input/output modes, and explicit non-goals during SHA-1→SHA-256 transition. <https://git-scm.com/docs/hash-function-transition>
[^cargo-dev]: Cargo Book, *Development dependencies*: dev-dependencies are used for tests/examples/benchmarks, not normal package builds, and are not propagated to dependent packages. <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies>
[^cargo-tree]: Cargo Book, *cargo tree*: `--edges normal` selects normal dependency edges and `--invert` exposes reverse dependencies. <https://doc.rust-lang.org/cargo/commands/cargo-tree.html>
[^rust-fromstr]: Rust standard library documentation, `FromStr` “Input format and round-tripping”: `Display`/parse compatibility and losslessness must be explicitly designed and documented. <https://doc.rust-lang.org/std/str/trait.FromStr.html#input-format-and-round-tripping>
[^lemire]: Daniel Lemire, *Fast Random Integer Generation in an Interval*, ACM TOMACS / arXiv 1805.10941: fixed-width random words need an unbiased interval mapping; rejection methods avoid modulo bias but can consume additional words. <https://arxiv.org/abs/1805.10941>
