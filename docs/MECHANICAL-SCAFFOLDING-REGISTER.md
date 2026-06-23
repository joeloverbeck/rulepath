# Mechanical Scaffolding Register

Status: governed by accepted
[ADR 0008](adr/0008-mechanical-scaffolding-governance.md).

This register records decisions for Rulepath mechanical scaffolding:
behavior-free typed infrastructure that supports generic contracts without
deciding game rules. It is parallel to, and does not replace, the behavioral
mechanic atlas and primitive-pressure ledger.

Mechanical scaffolding may only cover repeated plumbing around already-lawful
generic contracts: effect envelopes, seat IDs, actor/viewer IDs, action trees,
command envelopes, visibility scopes, replay/hash bytes, serialization
boundaries, benchmark/evidence records, and dev-only evidence harnesses.

It must not encode legality, scoring, reveal policy, turn policy, strategy,
hidden-state semantics, renderer policy, game-local state meaning, or private
licensed content.

## Entry Schema

Each register entry must include these fields.

| Field | Required content |
|---|---|
| Entry id | Stable id, date, status, and owner. |
| Candidate | Short name of the proposed scaffolding shape. |
| semantic.risk | `low`, `medium`, or `high`, with rationale for why the shape is behavior-free or why it is rejected. |
| Proposed home | `engine-core`, `game-stdlib`, `game-test-support`, `wasm-api`, local-only, or rejected. |
| Production-vs-test home | Whether production crates may depend on it, or whether it is dev/test-only. |
| Exact duplicate sites | File paths, symbols, and games/tools that currently repeat the shape. |
| Behavior exclusions | What game mechanics, policies, and hidden-state meanings the candidate explicitly does not own. |
| Affected hashes | State, effect, action-tree, public-view, seat-private-view, export, domain, or none. |
| Visibility impact | Public, viewer-scoped, seat-private, internal-dev, private-source, or none. |
| Determinism impact | Ordering, serialization, RNG, stable bytes, or none. |
| Migration set | Every official game, crate, tool, or doc that must migrate, or `none`. |
| Acceptance evidence | Tests, examples, no-leak checks, replay/hash checks, benchmarks, and docs required before adoption. |
| Rejection rationale | Required when the decision is local-only, deferred, or rejected. |
| Next review trigger | Second-use review, pre-third-copy hard decision, named gate, or no further review. |

## Decision States

| State | Meaning |
|---|---|
| `candidate` | Repetition is observed, but no reuse decision has landed. |
| `local-only` | Keep all known sites local with rationale. |
| `promoted` | A narrow behavior-free helper is adopted in the named home and all migration obligations are closed. |
| `promotion-debt-open` | A helper is adopted, but one or more matching sites still require migration or accepted exception. |
| `deferred` | Revisit at a named trigger; no helper exists yet. |
| `rejected` | The shape is not scaffolding or is not worth extracting. |

## Non-Promotion List

These shapes stay behavioral. They are not mechanical scaffolding merely because
multiple games use similar words or data paths.

| Shape | Register stance |
|---|---|
| Deal schedule, shuffle/deal policy, redeal policy | stays behavioral; game-local unless the mechanic atlas separately promotes a narrow helper. |
| Reveal timing, hidden commitment reveal, staged public reveal | stays behavioral; visibility and effect policy remain game-owned. |
| Projection and redaction policy for game state | stays behavioral; scaffolding may carry generic visibility scopes only, not decide what facts are visible. |
| Betting, bidding, contribution, raise, call, or fold policy | stays behavioral; economic and action legality policy remain game-owned. |
| Pot construction, side-pot allocation, remainder order | stays behavioral; allocation semantics stay game-owned. |
| Trick lifecycle, led-suit policy, trump policy, winner-leads policy | stays behavioral except for helpers already promoted by the mechanic atlas with explicit scope. |
| Teams, partnerships, alliances, shared victory, teammate visibility | stays behavioral; seat identity scaffolding must not encode team policy. |
| Graph, topology, adjacency, movement, reachability, connectivity | stays behavioral unless the mechanic atlas records a narrow promoted primitive. |
| Resource accounting, market costs, shared ledgers, scoring ledgers | stays behavioral; accounting semantics stay game-owned. |
| Reaction windows, interrupts, pending responder policy | stays behavioral; response legality and resolution stay game-owned. |
| Scoring, terminal outcome, ranking, tiebreakers, victory rationale | stays behavioral; scaffolding may transport typed evidence only. |

If a proposed entry touches one of these shapes, the default decision is
`rejected` for this register and rerouted to the mechanic atlas or a separate
ADR. A future exception must cite accepted authority and explain why the helper
is behavior-free despite the listed risk.

## Current Entries

The Unit 8C code-extraction series adds candidate entries before any helper
implementation. Entries remain `candidate` until their owning implementation
ticket proves the acceptance evidence and updates this register. The behavioral
policy bundle entry starts as `rejected / local-only` because it is not
mechanical scaffolding.

### MSC-8C-001 - Generic effect-envelope constructors

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-01.
- Candidate: inherent constructors for `EffectEnvelope<T>` public and
  seat-private envelopes.
- semantic.risk: `low`; the helper sets only the existing generic
  `VisibilityScope` plus caller-supplied payload and does not inspect payload
  meaning.
- Proposed home: `engine-core`.
- Production-vs-test home: production kernel ergonomics; normal production
  dependencies may use it.
- Exact duplicate sites: `games/race_to_n/src/effects.rs::public_effect`;
  `games/river_ledger/src/effects.rs` public/private envelope construction;
  repeated `EffectEnvelope { visibility, payload }` literals in game effect
  modules and tests.
- Behavior exclusions: no reveal policy, effect meaning, animation policy,
  redaction, viewer authorization, scoring, legality, diagnostics, or game
  state mutation.
- Affected hashes: effect hash surfaces may be compared during pilots; expected
  result for this entry is unchanged bytes and hashes.
- Visibility impact: public and seat-private scopes only as already supplied by
  callers; no new authorization path.
- Determinism impact: none beyond preserving existing ordered effect bytes.
- Migration set: initial pilots in Race to N and River Ledger only; remaining
  games stay local until later C-11 audits.
- Acceptance evidence: UNI8CMECSCA-005 constructor-vs-literal equality and
  move-semantics tests; UNI8CMECSCA-006 Race/River replay checks, River
  visibility checks, and workspace tests with no effect-order/hash delta.
- Rejection rationale: not applicable; accepted with behavior exclusions above.
- Next review trigger: C-11 game audits if additional games need effect-envelope
  migration.

R1 public fixed-seat receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | public effect envelope constructor | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | no R1 byte/hash/visibility change | N/A | C-11 game audit if effect surfaces expand |
| `draughts_lite` | public effect envelope constructor | migrated | `archive/tickets/8CR1PUBFIXSEA-002.md` | effect order, payload, visibility, and hashes unchanged | restore local literal constructor | C-11 game audit or effect-surface migration |
| `three_marks` | public effect envelope constructor | migrated | `archive/tickets/8CR1PUBFIXSEA-003.md` | effect order, payload, visibility, and hashes unchanged | restore local literal constructor | C-11 game audit or effect-surface migration |
| `column_four` | public effect envelope constructor | migrated | `archive/tickets/8CR1PUBFIXSEA-004.md` | effect order, payload, visibility, and hashes unchanged | restore local literal constructor | C-11 game audit or effect-surface migration |
| `directional_flip` | public effect envelope constructor | migrated | `archive/tickets/8CR1PUBFIXSEA-005.md` | effect order, payload, visibility, and hashes unchanged | restore local literal constructor | C-11 game audit or effect-surface migration |
| `token_bazaar` | public effect envelope constructor | migrated | `archive/tickets/8CR1PUBFIXSEA-006.md` | effect order, payload, visibility, hashes, and public export effect bytes unchanged | restore local literal constructor | C-11 game audit or effect/export-surface migration |

### MSC-8C-002 - Canonical seat-ID grammar plus import aliases

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-02.
- Candidate: strict canonical `seat_<zero-based>` parse/format/index helpers
  plus an explicit WASM/import adapter for bounded legacy aliases.
- semantic.risk: `medium`; seat identity is kernel vocabulary, but permissive
  parsing could become policy if aliases leak outside the import boundary.
- Proposed home: `engine-core` for strict canonical grammar; `wasm-api` for the
  import-only legacy adapter.
- Production-vs-test home: production contract helper and browser-boundary
  adapter; no TypeScript normalization.
- Exact duplicate sites: `crates/wasm-api/src/seats.rs` per-game trace seat
  adapters; `tools/replay-check/src/main.rs` `parse_*_trace_seat` helpers;
  game canonical seat helpers including `games/river_ledger/src/ids.rs` and
  `games/vow_tide/src/ids.rs`; legacy hyphen fixtures under
  `games/race_to_n/tests/golden_traces/`,
  `games/draughts_lite/tests/golden_traces/`, and
  `games/high_card_duel/tests/golden_traces/`.
- Behavior exclusions: no roles, teams, dealer order, turn order, actor
  authorization, setup admission, diagnostic prose, or display labels.
- Affected hashes: seat-string fixture/export/hash surfaces only when a later
  ticket names a per-surface ADR-0009 migration; default read compatibility is
  unchanged.
- Visibility impact: none for visibility policy; seat-private labels remain
  viewer-scoped only when game/WASM code already authorizes them.
- Determinism impact: stable formatting and rejection behavior; no RNG impact.
- Migration set: canonical helper tests, `crates/wasm-api/src/seats.rs`, Race
  to N and River Ledger pilots; historical hyphen traces remain readable until
  named migrations.
- Acceptance evidence: UNI8CMECSCA-007 kernel round-trip/rejection tables;
  UNI8CMECSCA-008 `wasm-api` alias-import and rejection tests; UNI8CMECSCA-009
  Race/River canonical-seat pilot tests, replay checks, workspace tests, and
  no-golden-diff proof.
- Rejection rationale: not applicable; accepted with import-boundary and
  no-silent-migration exclusions above.
- Next review trigger: named ADR-0009 seat-string migrations or C-11 game
  audits if additional games adopt canonical helpers.

R1 public fixed-seat receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `draughts_lite` | typed strict canonical parser | migrated | `archive/tickets/8CR1PUBFIXSEA-007.md` | no hash/visibility change; non-canonical imports still rejected outside WASM adapter | restore local parser branch | C-11 game audit or native seat-surface migration |
| `three_marks` | typed strict canonical parser | migrated | `archive/tickets/8CR1PUBFIXSEA-008.md` | no hash/visibility change; non-canonical imports still rejected outside WASM adapter | restore local parser branch | C-11 game audit or native seat-surface migration |
| `column_four` | typed strict canonical parser | migrated | `archive/tickets/8CR1PUBFIXSEA-009.md` | no hash/visibility change; non-canonical imports still rejected outside WASM adapter | restore local parser branch | C-11 game audit or native seat-surface migration |
| `directional_flip` | typed strict canonical parser | migrated | `archive/tickets/8CR1PUBFIXSEA-010.md` | no hash/visibility change; non-canonical imports still rejected outside WASM adapter | restore local parser branch | C-11 game audit or native seat-surface migration |
| `token_bazaar` | typed strict canonical parser | migrated | `archive/tickets/8CR1PUBFIXSEA-011.md` | no hash/visibility change; non-canonical imports still rejected outside WASM adapter | restore local parser branch | C-11 game audit or native seat-surface migration |
| R1 wave | output-only canonical roster helper | migrated | `archive/tickets/8CR1PUBFIXSEA-012.md` | helper only formats caller-supplied seat ids; no visibility authority | restore local formatting at output sites | next WASM/export seat-surface migration |
| `race_to_n` | selected WASM trace roster/actor output | migrated | `archive/tickets/8CR1PUBFIXSEA-013.md`; `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` | selected WASM export seat bytes changed from legacy hyphen to canonical underscore; winner unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| `draughts_lite` | selected WASM trace roster/actor output | migrated | `archive/tickets/8CR1PUBFIXSEA-014.md`; report after-receipt | selected WASM export seat bytes changed from legacy hyphen to canonical underscore; winner unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| `three_marks` | selected WASM trace roster/actor output | migrated | `archive/tickets/8CR1PUBFIXSEA-015.md`; report after-receipt | selected WASM export seat bytes changed from legacy hyphen to canonical underscore; winner unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| `column_four` | selected WASM trace roster output | migrated | `archive/tickets/8CR1PUBFIXSEA-016.md`; report after-receipt | selected WASM export roster bytes changed to canonical underscore; actor/winner unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| `directional_flip` | selected WASM trace roster output | migrated | `archive/tickets/8CR1PUBFIXSEA-017.md`; report after-receipt | selected WASM export roster bytes changed to canonical underscore only; actor/action/winner unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| `token_bazaar` | selected WASM trace roster output | migrated | `archive/tickets/8CR1PUBFIXSEA-018.md`; report after-receipt | selected WASM export roster bytes changed to canonical underscore only; actor/winner/public-export hash authority unchanged; visibility unchanged | restore selected golden trace and WASM output spelling | named native replay/trace or WASM seat-surface migration |
| R1 wave | native `default_seats` and non-WASM legacy trace seats | accepted exception | characterization report accepted-exception table | mixed spelling remains outside selected WASM exports; no visibility change | no R1 change to roll back | separately admitted native replay/trace seat-surface migration |

### MSC-8C-003 - Seat-count validation and ring-index arithmetic

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-03.
- Candidate: `game-stdlib::seat` structural `SeatCount`,
  `SeatCountRange`, checked index, and next-ring-index arithmetic.
- semantic.risk: `medium`; count/ring arithmetic is behavior-free only while
  games retain setup diagnostics, active-seat policy, dealer policy, roles, and
  teams.
- Proposed home: `game-stdlib`.
- Production-vs-test home: optional production game-layer dependency for games
  that adopt the structural helper.
- Exact duplicate sites: exact-two setup validation in
  `games/race_to_n/src/setup.rs`; 3-6 setup validation and stable seat order in
  `games/river_ledger/src/setup.rs`; simulator/tool seat-count checks in
  `tools/simulate/src/main.rs`.
- Behavior exclusions: no pass direction, dealer/blind/button policy,
  partnership/team grouping, bidding order, active-seat transitions, setup
  diagnostic text, or generated seat enums.
- Affected hashes: setup, state, replay, public-view, and seat-private-view
  hashes for Race/River pilots must remain unchanged.
- Visibility impact: none; helper does not project or redact seat facts.
- Determinism impact: deterministic checked arithmetic and wraparound only.
- Migration set: Race to N exact-two pilot and River Ledger 3-6 validation/ring
  pilot; other games deferred to C-11 audits.
- Acceptance evidence: UNI8CMECSCA-010 structural-error unit/property tests;
  UNI8CMECSCA-011 Race/River setup acceptance and rejection tests,
  diagnostic-preservation checks, replay/fixture comparisons, workspace tests,
  and policy-term grep proof for `game-stdlib::seat`.
- Rejection rationale: not applicable; accepted with behavior exclusions above.
- Next review trigger: C-11 audits if additional games adopt seat-count or ring
  helpers.

R1 public fixed-seat receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | exact two-seat validation | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | no R1 byte/hash/visibility change | N/A | C-11 game audit if setup surfaces expand |
| `draughts_lite` | exact two-seat validation | migrated | `archive/tickets/8CR1PUBFIXSEA-019.md` | setup diagnostics, state, replay, and visibility unchanged | restore local exact-count predicate | C-11 game audit or setup-surface migration |
| `three_marks` | exact two-seat validation | migrated | `archive/tickets/8CR1PUBFIXSEA-020.md` | setup diagnostics, state, replay, and visibility unchanged | restore local exact-count predicate | C-11 game audit or setup-surface migration |
| `column_four` | exact two-seat validation | migrated | `archive/tickets/8CR1PUBFIXSEA-021.md` | setup diagnostics, state, replay, and visibility unchanged | restore local exact-count predicate | C-11 game audit or setup-surface migration |
| `directional_flip` | exact two-seat validation | migrated | `archive/tickets/8CR1PUBFIXSEA-022.md` | setup diagnostics, state, replay, and visibility unchanged | restore local exact-count predicate | C-11 game audit or setup-surface migration |
| `token_bazaar` | exact two-seat validation plus normal `game-stdlib` dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-023.md` | setup diagnostics, state, replay, and visibility unchanged | restore local exact-count predicate and remove normal dependency | C-11 game audit or setup-surface migration |
| R1 wave | ring/index geometry | not applicable | characterization report C-03 matrix | typed two-seat `other()` mappings remain game-local; no visibility/hash change | no R1 change to roll back | future ring/index helper adoption in a game with ring geometry |

### MSC-8C-004 - Action-tree encoding/hash v1

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-04.
- Candidate: versioned canonical encoding and stable hash for the current
  `ActionTree`, `ActionNode`, `ActionChoice`, `ActionMetadata`, and
  `ActionPreview` contract.
- semantic.risk: `medium`; the encoder is behavior-free only if it encodes the
  existing contract fields in explicit order and does not add legal-choice
  semantics.
- Proposed home: `engine-core`.
- Production-vs-test home: production kernel replay/hash helper with
  version-explicit callers.
- Exact duplicate sites: local action-tree hash encoders in
  `games/race_to_n/src/replay_support.rs`,
  `games/draughts_lite/src/replay_support.rs`,
  `games/high_card_duel/src/replay_support.rs`, and tool-side replay hashes in
  `tools/replay-check/src/main.rs`.
- Behavior exclusions: no legality generation, disabled-state invention,
  choice meaning, preview behavior, UI renderer policy, diagnostics, or game
  action parsing.
- Affected hashes: action-tree hash surfaces only; Race and Draughts pilots use
  parallel-new-surface or one named ADR-0009 migration.
- Visibility impact: viewer-scoped only through already viewer-safe action
  trees supplied by games.
- Determinism impact: stable bytes, field order, vector order, recursive child
  framing, and versioned domain separation.
- Migration set: Race flat-tree and Draughts compound-tree pilots; legacy
  surfaces keep read compatibility through C-11 unless later accepted specs
  close it.
- Acceptance evidence: collision/ambiguity characterization from
  UNI8CMECSCA-004, `StableBytesWriter` byte-contract evidence from
  UNI8CMECSCA-012, and UNI8CMECSCA-013 kernel V1 encoding vectors/hash
  receipts for empty, flat, metadata/tag, preview, and recursive action trees.
- Rejection rationale: not applicable.
- Next review trigger: Race/Draughts old-vs-v1 hash receipts and
  replay-check/fixture-check evidence from UNI8CMECSCA-014/015.

R1 public fixed-seat receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | action-tree v1 wrappers/hash | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | legacy hash authority unchanged; v1 parallel surface only | N/A | future authority flip with ADR-0009 packet |
| `draughts_lite` | action-tree v1 wrappers/hash | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | legacy hash authority unchanged; v1 parallel surface only | N/A | future authority flip with ADR-0009 packet |
| `three_marks` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR1PUBFIXSEA-024.md` | legacy `action_tree_hash` unchanged and authoritative; v1 hash is parallel viewer-safe evidence | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `column_four` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR1PUBFIXSEA-025.md` | legacy `action_tree_hash` unchanged and authoritative; v1 hash is parallel viewer-safe evidence | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `directional_flip` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR1PUBFIXSEA-026.md` | legacy `action_tree_hash` unchanged and authoritative; v1 hash is parallel viewer-safe evidence | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `token_bazaar` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR1PUBFIXSEA-027.md` | legacy `action_tree_hash` unchanged and authoritative; v1 hash is parallel viewer-safe evidence | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| R1 wave | legacy Trace Schema v1 action-tree hash | accepted exception | characterization report accepted-exception table | compatibility preserved by keeping legacy hash authority unchanged | remove no code; exception-only receipt | future authority flip with ADR-0009 packet |

### MSC-8C-005 - Stable-byte writer v1

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-05.
- Candidate: explicit `StableBytesWriter` v1 with magic, domain, surface
  version, type tags, field tags, lengths, deterministic integer endianness,
  sequence framing, option framing, and duplicate/non-increasing field-tag
  rejection.
- semantic.risk: `medium`; byte writers affect replay/hash stability and must
  stay explicit rather than deriving behavior from reflection or serializers.
- Proposed home: `engine-core`.
- Production-vs-test home: production replay/hash infrastructure; usable by
  kernel and games for named stable surfaces.
- Exact duplicate sites: ad hoc `HashValue::from_stable_bytes` string/byte
  assembly in `games/*/src/replay_support.rs`, especially Race to N and
  Draughts Lite pilots, plus replay/tool hash assembly in
  `tools/replay-check/src/main.rs`.
- Behavior exclusions: no hash algorithm change, no schema discovery, no
  unordered-map hashing, no floating point, no JSON/CBOR/Borsh/bincode
  authority, no action legality or effect meaning.
- Affected hashes: none by itself; hash surfaces change only when a later
  versioned writer caller names a surface and migration classification.
- Visibility impact: none; writer only frames caller-supplied viewer-safe or
  internal-dev bytes.
- Determinism impact: stable bytes and serialization order.
- Migration set: kernel writer tests first; Race/Draughts action-tree pilots
  consume it later.
- Acceptance evidence: golden byte vectors, nested/sequence/option vectors,
  delimiter-collision negatives, field-order rejection tests, and cross-run
  determinism checks.
- Acceptance evidence: UNI8CMECSCA-012 implemented the writer in `engine-core`
  with golden byte vectors for header, primitive fields, nested records,
  sequences, options, enum discriminants, delimiter-collision resistance,
  duplicate/non-increasing tag rejection, and repeated-input determinism.
- Rejection rationale: not applicable.
- Next review trigger: first game/hash-surface migration evidence in
  UNI8CMECSCA-013 through UNI8CMECSCA-015.

R1 public fixed-seat receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | action-tree v1 writer use | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | v1 action-tree bytes only; adjacent surfaces unchanged | N/A | future named stable-byte surface migration |
| `draughts_lite` | action-tree v1 writer use | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | v1 action-tree bytes only; adjacent surfaces unchanged | N/A | future named stable-byte surface migration |
| `three_marks` | action-tree v1 writer use | migrated | `archive/tickets/8CR1PUBFIXSEA-024.md` | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `column_four` | action-tree v1 writer use | migrated | `archive/tickets/8CR1PUBFIXSEA-025.md` | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `directional_flip` | action-tree v1 writer use | migrated | `archive/tickets/8CR1PUBFIXSEA-026.md` | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `token_bazaar` | action-tree v1 writer use | migrated | `archive/tickets/8CR1PUBFIXSEA-027.md` | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| R1 wave | state snapshot bytes/hash | accepted exception | characterization report C-05 exception table | no R1 state byte/hash change | no R1 change to roll back | dedicated state-surface migration |
| R1 wave | effect bytes/hash | accepted exception | characterization report C-05 exception table | no R1 effect byte/hash change; C-01 proved constructor parity | restore per-game C-01 constructor if parity failed | dedicated effect-surface migration |
| R1 wave | public-view bytes/hash | accepted exception | characterization report C-05 exception table | no R1 public-view byte/hash or visibility change | no R1 change to roll back | dedicated public-view migration |
| R1 wave | replay/export bytes/hash outside selected WASM seat output | accepted exception | characterization report C-05 exception table | selected WASM seat bytes changed only under C-02 receipts; other replay/export bytes unchanged | restore selected WASM traces/output if needed | dedicated replay/export migration |
| R1 wave | dedicated diagnostic bytes/hash | accepted exception or not applicable by game | characterization report C-05 exception table | no diagnostic byte/hash or visibility change | no R1 change to roll back | dedicated diagnostic-surface migration |

### MSC-8C-006 - Dev-only game test-support crate

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-06.
- Candidate: new `crates/game-test-support` crate for test/evidence
  orchestration only.
- semantic.risk: `medium`; the crate is lawful only if production crates,
  WASM, browser surfaces, and normal CLI builds do not depend on it.
- Proposed home: `game-test-support`.
- Production-vs-test home: dev/test-only; consumers may use it through
  `[dev-dependencies]` only.
- Exact duplicate sites: game-local visibility/no-leak test geometry in
  `games/high_card_duel/tests/visibility.rs`,
  `games/river_ledger/tests/visibility.rs`, `games/vow_tide/tests/visibility.rs`,
  and replay/profile fixture assertions across the six 8C pilots.
- Behavior exclusions: no setup legality, rule legality, projection/redaction,
  scoring, effect meaning, bot choices, game strategy, or runtime framework.
- Affected hashes: none directly; drivers may compare hashes supplied by games.
- Visibility impact: internal-dev test harness only.
- Determinism impact: deterministic test enumeration and reporting only.
- Migration set: workspace manifest, `crates/game-test-support`, boundary
  script guard, and pilot dev-dependencies added by later tickets.
- Acceptance evidence: UNI8CMECSCA-018 created the workspace crate, declared
  `no_leak` and `profiles` module boundaries, compiled
  `game-test-support`, passed `cargo tree --workspace -e normal --invert
  game-test-support` with only the root package, and extended
  `scripts/boundary-check.sh` to reject normal/build reverse dependency edges.
- Rejection rationale: not applicable.
- Next review trigger: UNI8CMECSCA-019 no-leak implementation and
  UNI8CMECSCA-022 profile-driver implementation.

R1 public fixed-seat receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | `game-test-support` dev dependency | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile test | next profile-driver adoption |
| `draughts_lite` | `game-test-support` dev dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-028.md`; `archive/tickets/8CR1PUBFIXSEA-033.md` | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile tests | next profile-driver adoption |
| `three_marks` | `game-test-support` dev dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-029.md` | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile test | next profile-driver adoption |
| `column_four` | `game-test-support` dev dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-030.md` | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile test | next profile-driver adoption |
| `directional_flip` | `game-test-support` dev dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-031.md`; `archive/tickets/8CR1PUBFIXSEA-034.md` | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile tests | next profile-driver adoption |
| `token_bazaar` | `game-test-support` dev dependency | migrated | `archive/tickets/8CR1PUBFIXSEA-032.md`; `archive/tickets/8CR1PUBFIXSEA-035.md`; `archive/tickets/8CR1PUBFIXSEA-036.md` | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile tests | next profile-driver adoption |
| workspace | production/build reverse dependency edge | accepted proof: no production/build path | characterization report C-06 closeout; `cargo tree --workspace -e normal --invert game-test-support` | output shows only `game-test-support`; no runtime edge | boundary failure would require removing offending dependency | every profile-driver adoption and final closeout |

### MSC-8C-007 - Pairwise no-leak assertion geometry

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-07.
- Candidate: generic source-seat x viewer x surface x canary matrix assertion
  geometry with structured failure reporting.
- semantic.risk: `medium`; the harness is behavior-free only if games provide
  snapshots, canaries, authorization/reveal expectations, and containment
  checks.
- Proposed home: `game-test-support`.
- Production-vs-test home: dev/test-only.
- Exact duplicate sites: pairwise hidden-information tests in
  `games/high_card_duel/tests/visibility.rs`,
  `games/river_ledger/tests/visibility.rs`, and existing N-seat no-leak traces
  such as River Ledger and Vow Tide public/seat-private no-leak fixtures.
- Behavior exclusions: no projection, redaction, reveal timing, authorization,
  private-card meaning, bid/commitment meaning, bot explanation policy, or
  browser export construction.
- Affected hashes: no direct hash authority; may compare view/export/effect
  surfaces supplied by games.
- Visibility impact: internal-dev no-leak proof for public, viewer-scoped, and
  seat-private surfaces.
- Determinism impact: deterministic matrix enumeration and stable failure text.
- Migration set: High Card Duel two-seat pilot, then River Ledger 3-6-seat
  pilot; other hidden-information games deferred to C-11.
- Acceptance evidence: UNI8CMECSCA-019 implemented the generic matrix
  enumeration and structured failure types with unit tests for authorized,
  unauthorized, revealed, not-applicable, missing-canary,
  false-positive-resistant containment, and diagnostic-rendering cases.
  UNI8CMECSCA-020 piloted the harness in High Card Duel across observer, seat
  0, and seat 1 for view, action tree, diagnostic, effect, replay export, and
  bot-input surfaces while retaining reveal-specific assertions.
  UNI8CMECSCA-021 piloted the harness in River Ledger across counts 3-6 for
  observer plus every seat viewer over view, effect, action, diagnostic,
  export, showdown, bot-input, and bot-explanation surfaces while leaving
  betting/pot/showdown policy game-owned.
- Rejection rationale: not applicable.
- Next review trigger: broader hidden-information game migration under C-11.

R1 public fixed-seat receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| R1 six-game wave | pairwise source-seat x viewer no-leak matrix | not applicable | characterization report C-07 closeout | games are public/perfect-information for this wave; no hidden source datum or unauthorized seat-private viewer pair introduced | no C-07 artifact added | future hidden-info/private-view surface |
| R1 six-game wave | public visibility regressions | retained existing evidence | per-game migration tickets and existing visibility/replay suites | visibility surfaces unchanged except canonical public seat strings in selected WASM exports | restore offending migration if visibility drift appears | future hidden-info/private-view surface |

### MSC-8C-008 - Evidence-profile drivers

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-08.
- Candidate: separate drivers for `replay-command-v1`, `public-export-v1`,
  `seat-private-export-v1`, `setup-evidence-v1`, and `domain-evidence-v1` with
  common metadata checks.
- semantic.risk: `medium`; drivers are lawful only when they validate profile
  shape and delegate setup, commands, projection, import/export semantics, and
  domain rules to games/tools.
- Proposed home: `game-test-support` plus thin validator adapters in
  `replay-check` and `fixture-check` where pilots require them.
- Production-vs-test home: drivers are dev/test-only; CLI tools remain validator
  owners and do not depend on game-test-support by default.
- Exact duplicate sites: profile-shaped evidence in
  `games/race_to_n/tests/golden_traces/shortest-normal.trace.json`,
  `games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json`,
  `games/vow_tide/tests/golden_traces/public-replay-export-import.trace.json`,
  `games/vow_tide/tests/golden_traces/seat-private-replay-export-import-all-viewers.trace.json`,
  `games/briar_circuit/data/fixtures/briar_circuit_moon.fixture.json`, and
  `games/briar_circuit/data/fixtures/briar_circuit_first_trick_exception.fixture.json`.
- Behavior exclusions: no selectors, formulas, triggers, procedural steps,
  setup legality, command parsing/application, projection/redaction, export
  authorization, scoring, evaluator, topology, or domain rules.
- Affected hashes: replay-command, public-export, seat-private-export, setup,
  and domain surfaces only when a pilot names canonical-byte authority or a
  migration note; otherwise canonical-byte authority may be `none`.
- Visibility impact: profile-specific public, viewer-scoped, seat-private, and
  internal-dev classifications.
- Determinism impact: deterministic driver sequencing and metadata validation;
  hash/canonical-byte determinism delegated to the named validator owner.
- Migration set: Race replay-command, River setup-evidence, Vow public and
  seat-private export, Briar domain evidence, plus thin `fixture-check` and
  `replay-check` dispatch if required.
- Acceptance evidence: UNI8CMECSCA-022 implemented five distinct
  `game-test-support::profiles` driver types, shared metadata validation,
  adapter handoff after metadata acceptance only, positive and negative driver
  tests for all five profiles, strict unknown/wrong-profile rejection,
  canonical-byte-authority `none` handling, and no production dependency edge.
  UNI8CMECSCA-023 adopted `ReplayCommandV1Driver` in Race to N against the
  legacy `shortest-normal` fixture without changing fixture bytes.
  UNI8CMECSCA-024 adopted `SetupEvidenceV1Driver` in River Ledger against the
  legacy 3-seat setup fixture with canonical-byte authority `none` and no
  fixture byte changes.
  UNI8CMECSCA-025 adopted `PublicExportV1Driver` and
  `SeatPrivateExportV1Driver` in Vow Tide against the legacy public and
  all-viewer seat-private export fixtures without changing fixture bytes.
  UNI8CMECSCA-026 adopted `DomainEvidenceV1Driver` in Briar Circuit against
  the legacy moon scoring and first-trick exception fixtures, with
  canonical-byte authority `none`, no fixture byte changes, and scoring/legality
  delegated to Briar Rust code.
  UNI8CMECSCA-027 added thin local profile dispatch to `replay-check` and
  `fixture-check` for the Race, Vow, River, and Briar pilots, including strict
  unknown-profile/cross-profile-field rejection and no `game-test-support` tool
  dependency.
- Rejection rationale: not applicable.
- Next review trigger: future C-08 pilot/output expansion under C-11.

R1 public fixed-seat receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `race_to_n` | `replay-command-v1` profile | already discharged by Unit 8C pilot | Unit 8C pilot receipts above | internal-dev evidence only; fixture bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `draughts_lite` | `replay-command-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-028.md` | internal-dev evidence only; golden trace bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `three_marks` | `replay-command-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-029.md` | internal-dev evidence only; golden trace bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `column_four` | `replay-command-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-030.md` | internal-dev evidence only; golden trace bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `directional_flip` | `replay-command-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-031.md` | internal-dev evidence only; golden trace bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `token_bazaar` | `replay-command-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-032.md` | internal-dev evidence only; golden trace bytes unchanged | remove profile wrapper test | next replay profile adoption |
| `draughts_lite` | `setup-evidence-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-033.md` | internal-dev evidence only; setup fixture bytes unchanged; canonical-byte authority `none` | remove setup profile test | next setup-evidence profile adoption |
| `directional_flip` | `setup-evidence-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-034.md` | internal-dev evidence only; setup fixture bytes unchanged; canonical-byte authority `none` | remove setup profile test | next setup-evidence profile adoption |
| `token_bazaar` | `setup-evidence-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-035.md` | internal-dev evidence only; setup fixture bytes unchanged; canonical-byte authority `none` | remove setup profile test | next setup-evidence profile adoption |
| `token_bazaar` | `public-export-v1` profile | migrated | `archive/tickets/8CR1PUBFIXSEA-036.md` | public export shape/hash authority checked; no private/internal/debug candidate fields admitted | remove public-export profile test | next public-export profile adoption |
| R1 six-game wave | `seat-private-export-v1` profile | not applicable | characterization report C-08 matrix | observer and seat views are equivalent or no seat-private export in this wave; no visibility/hash change | no artifact added | future private-view/export surface |
| R1 six-game wave | `domain-evidence-v1` profile | not applicable | characterization report C-08 matrix | no domain evidence fixture admitted in this wave; no visibility/hash change | no artifact added | future domain-evidence surface |

### MSC-8C-009 - Versioned bounded-index sampling

- Entry id: 2026-06-22, status `accepted`, owner Unit 8C / C-09.
- Candidate: document legacy modulo `next_index` semantics and add
  `next_index_unbiased_v1` for explicit rejection-sampling bounded indices.
- semantic.risk: `medium`; random-word mapping is kernel vocabulary, but
  changing existing consumption or migrating shuffle/deal policy would be
  behavior.
- Proposed home: `engine-core`.
- Production-vs-test home: production deterministic RNG contract helper.
- Exact duplicate sites: `crates/engine-core/src/rng.rs::next_index`; local
  unbiased bounded-index implementations in River Ledger setup/shuffle code and
  related repeated game-local algorithms characterized by 8C.
- Behavior exclusions: no shuffle helper, deal schedule, collection choice,
  mutation order, reveal policy, setup semantics, or automatic migration of
  existing modulo callers.
- Affected hashes: RNG-consuming setup, state, replay, public-view, and
  seat-private-view hashes for River only when the pilot proves byte-identical
  consumption; no legacy `next_index` hash change.
- Visibility impact: none by itself; downstream game surfaces remain
  game-owned.
- Determinism impact: RNG word consumption and rejection behavior are explicit
  and versioned.
- Migration set: engine-core vectors first, River Ledger local unbiased helper
  replacement only; all other games remain unchanged in 8C.
- Acceptance evidence: UNI8CMECSCA-016 documented legacy modulo consumption,
  preserved legacy `next_index` vectors, added unbiased v1 returned-index and
  consumed-word vectors including rejection cases, and matched the existing
  local rejection-sampling algorithm in engine-core tests; UNI8CMECSCA-017
  replaced River Ledger's local helper with the shared method, pinned
  local-vs-shared returned-index/draw-count equivalence, and passed River
  replay/fixture/visibility plus workspace checks with no setup artifact drift.
- Rejection rationale: not applicable.
- Next review trigger: future explicit RNG migrations outside Unit 8C, if any.

R1 public fixed-seat receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| R1 six-game wave | bounded-index RNG sampling | not applicable | characterization report C-09 closeout | no RNG helper/sampler migration; bot RNG, setup RNG, replay hashes, and vectors unchanged | no R1 change to roll back | separately admitted RNG migration packet |

### MSC-8C-010 - Behavioral-policy bundle on the Non-Promotion List - rejected / local-only

- Entry id: 2026-06-22, status `rejected / local-only`, owner Unit 8C / C-10.
- Candidate: deal/reveal/projection/betting/pot/trick/team/graph/accounting/
  reaction/scoring/outcome policy bundle listed in this register's
  Non-Promotion List.
- semantic.risk: `high`; these shapes decide or encode game behavior and hidden
  information policy.
- Proposed home: `rejected`; behavior remains in game crates or, for repeated
  behavioral mechanics, the mechanic atlas.
- Production-vs-test home: local-only game code; no shared scaffolding crate.
- Exact duplicate sites: behavioral ledgers and game docs including
  `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`,
  `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`,
  `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md`, and
  `docs/MECHANIC-ATLAS.md`.
- Behavior exclusions: the entry excludes deal, reveal, projection, betting,
  pot, trick, team, graph, accounting, reaction, scoring, and outcome policy
  from the mechanical-scaffolding lane rather than defining helper scope.
- Affected hashes: none from this register decision; any future behavioral
  migration would need its own atlas/ADR evidence and named hash migration.
- Visibility impact: none from this register decision; game-owned visibility
  policy remains unchanged.
- Determinism impact: none from this register decision.
- Migration set: none.
- Acceptance evidence: register review against the Non-Promotion List completed
  in UNI8CMECSCA-029,
  `docs/MECHANIC-ATLAS.md` section 10A still showing `Current debt: _None_`,
  and the accepted MSC-8C entries retaining explicit behavior exclusions.
- Rejection rationale: the bundle includes behavior-bearing policy and hidden
  information semantics, so it is not mechanical scaffolding under ADR 0008.
- Next review trigger: the next mechanic-ladder gate if a later game repeats
  one of these behavioral shapes.

R1 public fixed-seat receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| R1 six-game wave | deal/reveal/projection/scoring/outcome/bot/diagnostic policy bundle | rejected / local-only reaffirmed | characterization report C-10 closeout; `bash scripts/boundary-check.sh` | no behavior, legality, setup policy, projection, scoring, outcome, bot, diagnostic policy, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or any proposed behavioral extraction |

### Unit 8C closeout evidence - 2026-06-22

- Final state: `MSC-8C-001`...`MSC-8C-009` are `accepted`; `MSC-8C-010` is
  `rejected / local-only`.
- Pilot receipts: Race to N, Draughts Lite, High Card Duel, River Ledger, Vow
  Tide, and Briar Circuit evidence is recorded in the owning entries above and
  in the archived UNI8CMECSCA-005...UNI8CMECSCA-030 tickets.
- Final validators: `cargo fmt --all -- --check`, focused shared/pilot crate
  tests, `cargo test --workspace --all-targets`, pilot `replay-check` and
  `fixture-check` runs, `bash scripts/boundary-check.sh`,
  `cargo tree --workspace -e normal --invert game-test-support`,
  `node scripts/check-doc-links.mjs`, and
  `node scripts/check-catalog-docs.mjs` all completed successfully for
  UNI8CMECSCA-031.
- Scope result: no production-code, test, byte, fixture, hash, WASM, atlas, or
  roadmap change was needed for closeout; the four 8C-R follow-on rows remain
  unimplemented and `Not started`.

## Review Checklist

Before accepting a register entry, verify:

- the candidate is behavior-free;
- the API uses allowed generic vocabulary or correctly game-layer typed inputs;
- behavior exclusions name the mechanics the helper does not own;
- affected hashes and visibility impact are explicit;
- hidden information cannot leak through payloads, DOM, logs, bot explanations,
  candidate rankings, replay exports, traces, fixtures, or tests;
- deterministic ordering and stable bytes are proven where relevant;
- migration set is complete, explicitly deferred, or rejected with rationale;
- `engine-core` remains free of mechanic nouns;
- `game-stdlib` remains earned and narrow;
- no YAML, DSL, selector, condition, trigger, formula, or rule behavior enters
  static data.
