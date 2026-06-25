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
| `candidate` | A new or repeated behavior-free scaffolding shape is recorded, but no shared-helper decision has landed. A first-use candidate MUST name its owning game/site, behavior exclusions, and second-use or other next review trigger. |
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

## Forward Per-Game Maintenance Cadence

Every new official game completes two linked register checkpoints.

### Pre-implementation checkpoint

The game's reuse-first audit records:

| Field | Required content |
|---|---|
| Game and gate | Stable game id and roadmap/spec unit. |
| Audit evidence | Link to the filled `GAME-MECHANICS.md` audit and initialized `GAME-EVIDENCE.md` receipt. |
| Existing scaffolding reviewed | Matching MSC entry ids and accepted helpers in `engine-core`, `game-stdlib`, `game-test-support`, or `wasm-api`. |
| Planned disposition | Reuse, accepted exception, local-only, new candidate, rejected/rerouted, or not applicable with rationale. |
| Expected prior matches | Earlier official games/sites that may require characterization or migration. |
| Compatibility expectation | Hash, visibility, determinism, fixture/export, and ADR 0009 migration expectation. |

### Post-implementation checkpoint

Before official-game closeout:

1. update every reused entry whose migration evidence or next-review trigger
   changed;
2. add an entry for every newly invented behavior-free scaffolding shape;
3. record exact new and prior matching sites;
4. classify the prior-game migration set;
5. link the game evidence receipt and the machine audit record; and
6. name the follow-on tracker unit or accepted no-unit disposition.

A first-use entry is inventory, not extraction authority. It normally remains
`candidate`, `local-only`, or `rejected` until repeated evidence satisfies ADR
0008.

## Automatic Prior-Game Refactor Trigger

A bounded follow-on refactoring unit is required when a new game or newly
promoted helper leaves real characterization or migration work in earlier
official games. This includes:

- an exact semantic behavior-free shape now present in the new game and one or
  more earlier official games;
- a promoted helper whose migration set includes earlier games;
- a register entry that becomes `promotion-debt-open`; or
- a pre-third-copy decision whose accepted outcome requires consolidation.

The same closeout that records the migration set MUST add a named unit to
`specs/README.md`. The unit names the games, candidate/register entry, expected
hash and visibility impact, characterization evidence, rollback boundary, and
admission consequence.

A follow-on unit is not required only when the governing entry is explicitly
`local-only`, `deferred`, or `rejected` and records:

- why the sites are not semantically identical or why extraction is not worth
  the risk;
- the evidence supporting that decision;
- an owner; and
- a concrete next review trigger.

An unnamed TODO, issue reference without a tracker unit, or bare “review later”
does not satisfy this rule. Existing third-copy and promotion-debt blocking law
remains authoritative.

## Current Entries

The MSC-8C-001...010 entries and their R1-R4 receipts are the historical
baseline created by Unit 8C. Preserve them as shipped evidence.

For every new official game after the forward-governance interlock becomes
effective, register maintenance is part of the normal game lifecycle rather
than an optional reaction to a later ticket. A new behavior-free shape receives
a register entry before game closeout even when it remains a first-use
`candidate` or `local-only` shape. A game that introduces no new scaffolding
records that result in its game evidence and CI audit receipt; it does not need a
no-op candidate entry.

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | public effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-002.md`; `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` | public payload/order/hash and visibility unchanged | restore local literal constructor | C-11 game audit or effect-surface migration |
| `high_card_duel` | seat-private effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-003.md`; characterization report | owner scope, filtered payload, effect order/hash, and no-leak visibility unchanged | restore local literal constructor | C-11 private-effect migration |
| `secret_draft` | public effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-004.md`; characterization report | public commit/reveal effect payloads and hashes unchanged | restore local literal constructor | C-11 effect-surface migration |
| `secret_draft` | seat-private effect constructor | not applicable | characterization report C-01 private verdict | no local private-effect constructor exists; no visibility/hash change | no R2 change to roll back | first Secret private-effect surface |
| `poker_lite` | public effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-005.md`; characterization report | public pledge/showdown/yield effect bytes and visibility unchanged | restore local literal constructor | C-11 effect-surface migration |
| `poker_lite` | seat-private effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-006.md`; characterization report | owner scope, private deal payloads, and no-leak filtering unchanged | restore local literal constructor | C-11 private-effect migration |
| `masked_claims` | public effect envelope constructor | migrated | `archive/tickets/UNI8CR2TWOSEA-007.md`; characterization report | claim/window/reveal/terminal public effect bytes and visibility unchanged | restore local literal constructor | C-11 effect-surface migration |
| `masked_claims` | seat-private effect constructor | not applicable | characterization report C-01 private verdict | no local private-effect constructor exists; no visibility/hash change | no R2 change to roll back | first Masked private-effect surface |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | public and seat-private effect-envelope constructors | migrated | `archive/tickets/8CR3PUBCOOASY-101.md`; `archive/tickets/8CR3PUBCOOASY-102.md`; characterization report | public and owner-private payloads, order, scopes, hashes, and no-leak behavior unchanged | restore local literal constructors | C-11 effect-surface migration |
| `flood_watch` | public effect-envelope constructor | migrated | `archive/tickets/8CR3PUBCOOASY-103.md`; characterization report | public cooperative effect payloads/order/hash unchanged; no private effect class exists | restore local literal constructor | C-11 effect-surface migration |
| `frontier_control` | public effect-envelope constructor | migrated | `archive/tickets/8CR3PUBCOOASY-104.md`; characterization report | fully public graph/clash/scoring effects unchanged | restore local literal constructor | C-11 effect-surface migration |
| `event_frontier` | public effect-envelope constructor | migrated | `archive/tickets/8CR3PUBCOOASY-105.md`; characterization report | event/edict/operation/reckoning effect payloads and visibility unchanged | restore local literal constructor | C-11 effect-surface migration |
| R3 non-Plain games | private effect constructor | not applicable | characterization report C-01 private N/A rows | no local private-effect constructor exists; no synthetic private class added | no R3 change to roll back | first real private-effect constructor in those games |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | public and seat-private effect-envelope constructors | already discharged by Unit 8C pilot | Unit 8C C-01 pilot receipts above; characterization report | no R4 byte/hash/visibility change | N/A | C-11 game audit if River effect surfaces expand |
| `briar_circuit` | public effect-envelope constructor | migrated | `archive/tickets/8CR4NSEAPRITRI-002.md`; characterization report | public effect payloads, ordering, scopes, and hashes unchanged | restore local public literal | C-11 effect-surface migration |
| `briar_circuit` | seat-private effect-envelope constructor | migrated | `archive/tickets/8CR4NSEAPRITRI-003.md`; characterization report | owner scopes, pass-reveal payloads, visibility, and hashes unchanged | restore local private literal | C-11 private-effect migration |
| `vow_tide` | public effect-envelope constructor | migrated | `archive/tickets/8CR4NSEAPRITRI-004.md`; characterization report | public Vow WASM effect envelope output unchanged; no private effect class added | restore local public literal map | C-11 effect-surface migration |
| `vow_tide` | private effect-envelope constructor | not applicable | characterization report C-01 private N/A row | Vow private facts remain in views/exports, not effects; no visibility/hash change | no R4 code rollback | first intentional Vow private effect class |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | typed strict canonical parser | migrated | `archive/tickets/UNI8CR2TWOSEA-008.md`; characterization report | parser accepts canonical seats and rejects malformed/out-of-range labels; state/effect/hash surfaces unchanged | restore local parser branch | C-11 native seat-surface migration |
| `secret_draft` | typed strict canonical parser | migrated | `archive/tickets/UNI8CR2TWOSEA-009.md`; characterization report | parser accepts canonical seats and rejects malformed/out-of-range labels; state/effect/hash surfaces unchanged | restore local parser branch | C-11 native seat-surface migration |
| `poker_lite` | typed strict canonical parser | migrated | `archive/tickets/UNI8CR2TWOSEA-010.md`; characterization report | parser accepts canonical seats and rejects malformed/out-of-range labels; state/effect/hash surfaces unchanged | restore local parser branch | C-11 native seat-surface migration |
| `masked_claims` | typed strict canonical parser | migrated | `archive/tickets/UNI8CR2TWOSEA-011.md`; characterization report | parser accepts canonical seats and rejects malformed/out-of-range labels; state/effect/hash surfaces unchanged | restore local parser branch | C-11 native seat-surface migration |
| R2 WASM/import boundary | legacy roster aliases for HCD, Secret, and Poker | accepted exception | `archive/tickets/UNI8CR2TWOSEA-012.md`; characterization report | aliases remain import-only/read-compatibility; no TypeScript normalization or output flip | remove adapter exception only under named seat-string migration | named ADR-0009 seat-string migration |
| R2 WASM/import boundary | Masked Claims legacy roster aliases | not applicable | `archive/tickets/UNI8CR2TWOSEA-012.md`; characterization report | no legacy roster exception required; no visibility/hash change | no R2 change to roll back | first Masked legacy import need |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | typed strict canonical parser | migrated | `archive/tickets/8CR3PUBCOOASY-201.md`; characterization report | canonical seats accepted and malformed/out-of-range labels rejected; trace/export visibility unchanged | restore local parser branch | C-11 native seat-surface migration |
| R3 WASM/import boundary | legacy import aliases for Plain, Flood, Frontier, and Event | accepted exception | `archive/tickets/8CR3PUBCOOASY-202.md`; characterization report | aliases remain import-only/read-compatibility; no TypeScript normalization or output authority change | remove adapter exception only under named seat-string migration | named ADR-0009 seat-string migration |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | game parser/formatter/roster | already discharged by Unit 8C pilot | `archive/tickets/UNI8CMECSCA-009.md`; characterization report | no R4 seat byte/hash/visibility change | N/A | named native replay/trace or seat-surface migration |
| `briar_circuit` | typed strict canonical parser | migrated | `archive/tickets/8CR4NSEAPRITRI-005.md`; characterization report | canonical seats accepted and malformed/out-of-range labels rejected; replay/export visibility unchanged | restore local parser branch | C-11 native seat-surface migration |
| `briar_circuit` | canonical roster/formatter | migrated | `archive/tickets/8CR4NSEAPRITRI-006.md`; characterization report | output-only helper formats caller-owned seats; no visibility authority | restore local formatting | next WASM/export seat-surface migration |
| `briar_circuit` | WASM import legacy aliases | migrated | `archive/tickets/8CR4NSEAPRITRI-007.md`; characterization report | aliases remain import-only/read-compatibility; no TypeScript normalization | restore local import branch | named ADR-0009 seat-string migration |
| `vow_tide` | typed strict canonical parser | migrated | `archive/tickets/8CR4NSEAPRITRI-008.md`; characterization report | canonical seats accepted and malformed/out-of-range labels rejected; replay/export visibility unchanged | restore local parser branch | C-11 native seat-surface migration |
| `vow_tide` | canonical roster/formatter | migrated | `archive/tickets/8CR4NSEAPRITRI-009.md`; characterization report | output-only helper formats caller-owned seats for 3-7 seats; no visibility authority | restore local formatting | next WASM/export seat-surface migration |
| `vow_tide` | WASM import legacy aliases | migrated | `archive/tickets/8CR4NSEAPRITRI-010.md`; characterization report | aliases remain import-only/read-compatibility; no TypeScript normalization | restore local import branch | named ADR-0009 seat-string migration |
| R4 wave | non-seat IDs | not applicable | characterization report N/A ledger | card, trick, pot, contract, and hand identifiers remain game-owned | no R4 code rollback | first proposal to route non-seat IDs through a seat helper |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | exact two-seat validation | migrated | `archive/tickets/UNI8CR2TWOSEA-013.md`; characterization report | exact diagnostic, setup state, private hands, replay, and visibility unchanged | restore local exact-count predicate | C-11 setup-surface migration |
| `secret_draft` | exact two-seat validation plus normal `game-stdlib` dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-014.md`; characterization report | exact diagnostic, setup draft pool, commitments, replay, and visibility unchanged | restore local predicate and remove normal dependency | C-11 setup-surface migration |
| `poker_lite` | exact two-seat validation plus normal `game-stdlib` dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-015.md`; characterization report | exact diagnostic, shuffle/deal state, private hands, replay, and visibility unchanged | restore local predicate and remove normal dependency | C-11 setup-surface migration |
| `masked_claims` | exact two-seat validation plus normal `game-stdlib` dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-016.md`; characterization report | exact diagnostic, hands/reserve state, replay, and visibility unchanged | restore local predicate and remove normal dependency | C-11 setup-surface migration |
| R2 four-game wave | ring/index geometry | not applicable | characterization report C-03 matrix | fixed two-seat `other()` and phase-order rules remain game-local; no visibility/hash change | no R2 change to roll back | future game with ring geometry |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | exact roster count | migrated | `archive/tickets/8CR3PUBCOOASY-301.md`; characterization report | exact diagnostics, setup, hands/tail, replay, and visibility unchanged | restore local count predicate | C-11 setup-surface migration |
| `plain_tricks` | variant seat-count predicate | not applicable | `archive/tickets/8CR3PUBCOOASY-302.md`; characterization report | current setup path has no second variant predicate; no new rule introduced | no R3 change to roll back | first real Plain variant seat-count predicate |
| `flood_watch` | roster, variant seat-count, and role-order counts | migrated | `archive/tickets/8CR3PUBCOOASY-303.md`; `archive/tickets/8CR3PUBCOOASY-304.md`; characterization report | diagnostics, role order, setup, replay, fixture, and visibility unchanged | restore local predicates | C-11 setup-surface migration |
| `frontier_control` | roster and variant seat-count predicates | migrated | `archive/tickets/8CR3PUBCOOASY-305.md`; `archive/tickets/8CR3PUBCOOASY-306.md`; characterization report | diagnostics, graph/faction setup, replay, fixture, and visibility unchanged | restore local predicates | C-11 setup-surface migration |
| `event_frontier` | roster, variant seat-count, and faction-order predicates | migrated / exception | `archive/tickets/8CR3PUBCOOASY-307.md`; `archive/tickets/8CR3PUBCOOASY-308.md`; characterization report | count diagnostics unchanged; Charter/Freeholders order remains game-owned | restore local predicates | C-11 setup-surface migration or faction-order review |
| R3 wave | range/ring helpers and unowned faction/role policy | not applicable / exception | characterization report C-03 matrix | no game admits range or ring migration; rotation/order policy stays local | no R3 change to roll back | first true range/ring helper use or explicit faction/role order migration |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | 3-6 range and ring index | already discharged by Unit 8C pilot | `archive/tickets/UNI8CMECSCA-011.md`; characterization report | no R4 setup/replay/visibility change | N/A | C-11 setup-surface migration if River topology expands |
| `river_ledger` | stack-vector cardinality | not applicable | characterization report C-03 N/A row | resource vector cardinality remains River setup policy | no R4 code rollback | first behavior-free cardinality helper proposal |
| `briar_circuit` | exact-four setup admission | migrated | `archive/tickets/8CR4NSEAPRITRI-011.md`; characterization report | setup diagnostic, replay, fixture, and visibility unchanged | restore local length comparison | C-11 setup-surface migration |
| `briar_circuit` | pass/dealer topology | accepted exception | characterization report C-03 exception row | ring arithmetic call preserves Briar pass/deal semantics; no helper owns topology | restore local topology arithmetic if parity fails | named pass/dealer topology migration |
| `vow_tide` | 3-7 range admission | migrated | `archive/tickets/8CR4NSEAPRITRI-012.md`; characterization report | setup diagnostics, schedule, replay, fixture, and visibility unchanged | restore local range predicate | C-11 setup-surface migration |
| `vow_tide` | checked ring step | migrated | `archive/tickets/8CR4NSEAPRITRI-013.md`; characterization report | dealer rotation and deal order unchanged; helper supplies checked arithmetic only | restore local modulo step | C-11 ring/topology migration |
| `vow_tide` | hand schedule/deal capacity | accepted exception | characterization report C-03 exception row | schedule/deal capacity remains Vow game policy | no R4 code rollback | separately reviewed behavior-free schedule/deal proposal |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | action-tree v1 wrappers/hash for commit trees | migrated | `archive/tickets/UNI8CR2TWOSEA-017.md`; characterization report | v1 bytes/hash added as parallel surface; legal choices, legacy checks, hidden commits, and visibility unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `secret_draft` | action-tree v1 wrappers/hash for commit trees | migrated with legacy exception | `archive/tickets/UNI8CR2TWOSEA-018.md`; characterization report | v1 bytes/hash added as parallel surface; legacy `action_tree_hash` remains authoritative | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `poker_lite` | action-tree v1 wrappers/hash for pledge phases | migrated with legacy exception | `archive/tickets/UNI8CR2TWOSEA-019.md`; characterization report | v1 bytes/hash added as parallel surface; legacy `action_tree_hash` remains authoritative | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `masked_claims` | action-tree v1 wrappers/hash for claim/response trees | migrated | `archive/tickets/UNI8CR2TWOSEA-020.md`; characterization report | v1 bytes/hash added as parallel surface; pending response choices and hidden tile facts unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR3PUBCOOASY-401.md`; characterization report | v1 bytes/hash added as parallel surface; follow-suit legality and legacy checks unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `flood_watch` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR3PUBCOOASY-402.md`; characterization report | v1 bytes/hash added as parallel surface; budget/forecast legality and hidden deck unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `frontier_control` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR3PUBCOOASY-403.md`; characterization report | v1 bytes/hash added as parallel surface; graph legality and public visibility unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `event_frontier` | action-tree v1 wrappers/hash | migrated | `archive/tickets/8CR3PUBCOOASY-404.md`; characterization report | v1 bytes/hash added as parallel surface; event/operation legality and hidden deeper deck unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | action-tree v1 adapter, base and richer all-in vectors | migrated | `archive/tickets/8CR4NSEAPRITRI-014.md`; `archive/tickets/8CR4NSEAPRITRI-015.md`; characterization report | v1 bytes/hash added as parallel surface; River command legality, all-in metadata, and legacy hash authority unchanged | remove v1 wrappers/tests | future authority flip with ADR-0009 packet |
| `briar_circuit` | typed action-tree adapter and v1 vectors | migrated | `archive/tickets/8CR4NSEAPRITRI-016.md`; `archive/tickets/8CR4NSEAPRITRI-017.md`; characterization report | v1 bytes/hash added as parallel surface; browser parity, pass/play legality, and legacy hashes unchanged | remove typed/v1 adapter tests | future authority flip with ADR-0009 packet |
| `vow_tide` | action-tree v1 vectors for bid/play and 3-7 seats | migrated | `archive/tickets/8CR4NSEAPRITRI-018.md`; characterization report | v1 bytes/hash added as parallel surface; bid/play legality and legacy Debug-derived hashes unchanged | remove v1 wrapper/tests | future authority flip with ADR-0009 packet |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | action-tree v1 writer use | migrated | `archive/tickets/UNI8CR2TWOSEA-017.md`; characterization report | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `secret_draft` | action-tree v1 writer use | migrated | `archive/tickets/UNI8CR2TWOSEA-018.md`; characterization report | v1 action-tree bytes only; legacy action hash and adjacent surfaces unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `poker_lite` | action-tree v1 writer use | migrated | `archive/tickets/UNI8CR2TWOSEA-019.md`; characterization report | v1 action-tree bytes only; legacy action hash and adjacent surfaces unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| `masked_claims` | action-tree v1 writer use | migrated | `archive/tickets/UNI8CR2TWOSEA-020.md`; characterization report | v1 action-tree bytes only; state/effect/view/replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| R2 four-game wave | state/effect/public-view/replay/export/diagnostic stable-byte authority | accepted exception or not applicable by surface | characterization report C-05 exception matrix | no broad stable-byte authority flip; no golden/fixture regeneration | no R2 change to roll back outside named v1 action-tree wrappers | dedicated stable-byte migration with ADR-0009 packet |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| R3 action-tree v1 surfaces | StableBytesWriter use via `action_tree_v1_bytes` | migrated | `archive/tickets/8CR3PUBCOOASY-401.md`...`archive/tickets/8CR3PUBCOOASY-404.md`; characterization report | v1 parallel bytes/hashes added; legacy hash authorities and adjacent replay/export bytes unchanged | remove v1 wrappers/tests | future named stable-byte surface migration |
| R3 adjacent legacy surfaces | replay/export/fixture/domain bytes outside v1 action-tree | accepted exception | characterization report C-05 matrix | existing byte authorities remain game-owned and unchanged | no code rollback; exception-only receipt | named ADR-0009 stable-byte migration |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| R4 action-tree v1 surfaces | StableBytesWriter use via `action_tree_v1_encoding` / typed action-tree adapters | migrated | `archive/tickets/8CR4NSEAPRITRI-014.md`...`archive/tickets/8CR4NSEAPRITRI-018.md`; characterization report | v1 parallel bytes/hashes added only; no legacy action-tree/debug authority changed | remove v1 wrappers/tests | future named stable-byte surface migration |
| R4 adjacent state/effect/view/replay/export/diagnostic surfaces | stable-byte authority outside selected action-tree v1 | accepted exception / not applicable by surface | characterization report C-05 exception ledger; `archive/tickets/8CR4NSEAPRITRI-036.md` | state, effect, public/seat-private view, replay command, public/seat-private export, fixture, and diagnostic bytes remain game-owned and unchanged unless a ticket already named a parallel profile adapter | no code rollback; exception-only receipt | dedicated ADR-0009 stable-byte migration per surface |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | existing `game-test-support` dev dependency | retained pilot dependency | `archive/tickets/UNI8CR2TWOSEA-024.md`; characterization report | dev/test-only; no runtime hash/visibility impact | remove dev dependency and profile/no-leak tests if unused | next no-leak/profile adoption |
| `secret_draft` | `game-test-support` dev dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-021.md`; characterization report | dev/test-only; no runtime hash/visibility impact | remove dev dependency and harness/profile tests | next no-leak/profile adoption |
| `poker_lite` | `game-test-support` dev dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-022.md`; characterization report | dev/test-only; no runtime hash/visibility impact | remove dev dependency and harness/profile tests | next no-leak/profile adoption |
| `masked_claims` | `game-test-support` dev dependency | migrated | `archive/tickets/UNI8CR2TWOSEA-023.md`; characterization report | dev/test-only; no runtime hash/visibility impact | remove dev dependency and harness/profile tests | next no-leak/profile adoption |
| workspace | production/build reverse dependency edge | accepted proof: no production/build path | characterization report C-06 closeout; `bash scripts/boundary-check.sh`; `cargo tree --workspace -e normal --invert game-test-support` | no runtime edge; no WASM/tool production dependency | remove offending normal/build dependency if detected | every profile/no-leak adoption and final closeout |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | dev-only `game-test-support` edge | migrated | `archive/tickets/8CR3PUBCOOASY-501.md`; characterization report | dev/test-only dependency; production dependency graph unchanged | remove dev-dependency and tests using it | next dev-only profile/no-leak surface |
| `flood_watch` | dev-only `game-test-support` edge | migrated | `archive/tickets/8CR3PUBCOOASY-502.md`; characterization report | dev/test-only dependency; production dependency graph unchanged | remove dev-dependency and tests using it | next dev-only profile/no-leak surface |
| `frontier_control` | dev-only `game-test-support` edge | migrated | `archive/tickets/8CR3PUBCOOASY-503.md`; characterization report | dev/test-only dependency; production dependency graph unchanged | remove dev-dependency and tests using it | next dev-only profile/no-leak surface |
| `event_frontier` | dev-only `game-test-support` edge | migrated | `archive/tickets/8CR3PUBCOOASY-504.md`; characterization report | dev/test-only dependency; production dependency graph unchanged | remove dev-dependency and tests using it | next dev-only profile/no-leak surface |
| Workspace | production inverse-edge proof | migrated | characterization report; `cargo tree --workspace -e normal --invert game-test-support` | `game-test-support` remains absent from normal dependency graph | remove offending normal dependency | every dependency edit touching game-test-support |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | existing `game-test-support` dev-only edge | retained pilot dependency | `archive/tickets/UNI8CMECSCA-021.md`; characterization report | dev/test-only dependency; production/build dependency graph unchanged | remove dev dependency and tests if no longer used | next dev-only profile/no-leak surface |
| `briar_circuit` | existing `game-test-support` dev-only edge | retained pilot dependency | `archive/tickets/UNI8CMECSCA-026.md`; characterization report | dev/test-only dependency; production/build dependency graph unchanged | remove dev dependency and tests if no longer used | next dev-only profile/no-leak surface |
| `vow_tide` | existing `game-test-support` dev-only edge | retained pilot dependency | `archive/tickets/UNI8CMECSCA-025.md`; characterization report | dev/test-only dependency; production/build dependency graph unchanged | remove dev dependency and tests if no longer used | next dev-only profile/no-leak surface |
| Workspace | production/build reverse dependency proof | accepted proof: no production/build path | `archive/tickets/8CR4NSEAPRITRI-036.md`; characterization report | inverse tree checks and boundary check confirm no runtime or build edge to `game-test-support` | remove offending normal/build dependency if detected | every profile/no-leak adoption and final closeout |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | residual no-leak pilot verification | verified retained pilot plus residual matrix | `archive/tickets/UNI8CR2TWOSEA-024.md`; characterization report | hidden lead commitment remains absent from observer/opponent surfaces and present only on authorized owner/internal surfaces | remove residual test only if pilot evidence supersedes it | next HCD hidden-info surface |
| `secret_draft` | pairwise source-seat x viewer no-leak matrix | migrated | `archive/tickets/UNI8CR2TWOSEA-025.md`; characterization report | private draft choices and seeds remain absent from unauthorized views/effects/replay/bot surfaces | remove matrix test and dev dependency | next Secret hidden-info/export surface |
| `poker_lite` | pairwise source-seat x viewer no-leak matrix | migrated | `archive/tickets/UNI8CR2TWOSEA-026.md`; characterization report | own/opponent/public hand access, showdown reveal, yield non-reveal, effects, exports, and bots stay scoped | remove matrix test and dev dependency | next Poker hidden-info/export surface |
| `masked_claims` | pairwise source-seat x viewer no-leak matrix | migrated | `archive/tickets/UNI8CR2TWOSEA-027.md`; characterization report | pending claim, responder tree, accepted-secret state, challenge reveal, export, and bot surfaces stay scoped | remove matrix test and dev dependency | next Masked hidden-info/export surface |
| R2 four-game wave | committed canary artifacts | accepted proof: no committed canary | characterization report C-07 closeout | canaries remain in-memory test data only; no trace, fixture, export, DOM, storage, log, or snapshot leak | remove offending artifact if ever introduced | every hidden-info proof expansion |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | private hand/tail no-leak matrix and seat-private export pairwise proof | migrated | `archive/tickets/8CR3PUBCOOASY-511.md`; `archive/tickets/8CR3PUBCOOASY-641.md`; characterization report | own-hand visibility preserved; opponent hand and tail absent from unauthorized surfaces | remove wrapper/no-leak tests only | next private-holding visibility surface |
| `flood_watch` | hidden future-deck no-leak matrix | migrated | `archive/tickets/8CR3PUBCOOASY-512.md`; characterization report | forecast/draw visibility unchanged; future deck absent from public/export/bot surfaces | remove no-leak tests only | next hidden future-deck surface |
| `frontier_control` | C-07 N/A equality receipt | not applicable | `archive/tickets/8CR3PUBCOOASY-513.md`; characterization report | no hidden source; observer and seat projections remain equivalent | remove N/A receipt test only | first real hidden/private source |
| `event_frontier` | hidden deeper-deck no-leak matrix | migrated | `archive/tickets/8CR3PUBCOOASY-514.md`; characterization report | current/next/history visibility unchanged; deeper deck absent from public/export/bot surfaces | remove no-leak tests only | next hidden deeper-deck surface |
| R3 wave | canary hygiene | migrated | characterization report C-07 canary proof | in-memory canaries only; no persistent artificial secret tokens | remove offending persistent canary | every no-leak test addition |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | residual all-in lifecycle and export no-leak matrices | migrated | `archive/tickets/8CR4NSEAPRITRI-019.md`; `archive/tickets/8CR4NSEAPRITRI-020.md`; characterization report | stack/pot/multipot lifecycle and export surfaces keep hidden cards absent from unauthorized viewers | remove residual matrix tests only | next River hidden-info/accounting surface |
| `briar_circuit` | pass and play no-leak matrices | migrated | `archive/tickets/8CR4NSEAPRITRI-021.md`; `archive/tickets/8CR4NSEAPRITRI-022.md`; characterization report | private hands/pass selections/trick reveal timing remain scoped; no persistent canary artifacts | remove matrix tests only | next Briar private/trick visibility surface |
| `vow_tide` | hand/stock and bid/trick no-leak matrices | migrated | `archive/tickets/8CR4NSEAPRITRI-023.md`; `archive/tickets/8CR4NSEAPRITRI-024.md`; characterization report | private hands, hidden stock, bids/tricks, exports, and bots remain scoped across 3-7 seats | remove matrix tests only | next Vow private/trick visibility surface |
| R4 wave | canary hygiene | accepted proof | characterization report C-07 checkpoint | in-memory canaries only; no trace, fixture, export, DOM, storage, log, snapshot, or test identifier leak | remove offending persistent canary | every no-leak test addition |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | `replay-command-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-028.md`; characterization report | internal-dev profile evidence only; command trace validator remains game-owned | remove profile wrapper test | next replay profile adoption |
| `secret_draft` | `replay-command-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-029.md`; characterization report | internal-dev profile evidence only; no public leak or canonical byte claim | remove profile wrapper test | next replay profile adoption |
| `poker_lite` | `replay-command-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-030.md`; characterization report | internal-dev profile evidence only; no public leak or canonical byte claim | remove profile wrapper test | next replay profile adoption |
| `masked_claims` | `replay-command-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-031.md`; characterization report | internal-dev profile evidence only; wraps existing rule replay evidence | remove profile wrapper test | next replay profile adoption |
| `high_card_duel` | `setup-evidence-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-032.md`; characterization report | setup fixture read-only; no private deal or RNG seed leak | remove setup profile test | next setup-evidence profile adoption |
| `secret_draft` | `setup-evidence-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-033.md`; characterization report | setup fixture read-only; visible pool and commitment placeholder verified | remove setup profile test | next setup-evidence profile adoption |
| `poker_lite` | `setup-evidence-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-034.md`; characterization report | setup fixture read-only; private cards remain setup-hidden | remove setup profile test | next setup-evidence profile adoption |
| `masked_claims` | `setup-evidence-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-035.md`; characterization report | setup fixture read-only; hand/reserve internals remain hidden | remove setup profile test | next setup-evidence profile adoption |
| `high_card_duel` | `public-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-036.md`; characterization report | observer export hash checked; no hidden commitment or seed leak | remove public-export profile test | next public-export profile adoption |
| `secret_draft` | `public-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-037.md`; characterization report | observer export redacts pre-reveal path/item and seed material | remove public-export profile test | next public-export profile adoption |
| `poker_lite` | `public-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-038.md`; characterization report | observer export preserves showdown/yield redaction policy | remove public-export profile test | next public-export profile adoption |
| `masked_claims` | `public-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-039.md`; characterization report | observer export keeps claim tile redaction; replay-check currently accepts Masked traces via not-applicable baseline | remove public-export profile test | next public-export profile adoption |
| `high_card_duel` | `seat-private-export-v1` profile | not applicable | characterization report C-08 matrix | no seat-private export profile admitted for R2; no visibility/hash change | no R2 change to roll back | first HCD seat-private export profile need |
| `secret_draft` | `seat-private-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-040.md`; characterization report | viewer labels explicit; pre-reveal item/path and seed remain absent even for owner export | remove seat-private profile test | next seat-private profile adoption |
| `poker_lite` | `seat-private-export-v1` profile | migrated | `archive/tickets/UNI8CR2TWOSEA-041.md`; characterization report | own crest present, opponent crest and seed absent; showdown/yield policy unchanged | remove seat-private profile test | next seat-private profile adoption |
| `masked_claims` | `seat-private-export-v1` profile | not applicable | characterization report C-08 matrix | no seat-private export profile admitted for R2; no visibility/hash change | no R2 change to roll back | first Masked seat-private export profile need |
| R2 four-game wave | `domain-evidence-v1` profile | not applicable | characterization report C-08 matrix | no domain evidence fixture admitted; no visibility/hash change | no R2 change to roll back | future domain-evidence surface |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | replay/setup/domain/public-export/seat-private profiles | migrated | `archive/tickets/8CR3PUBCOOASY-601.md`; `archive/tickets/8CR3PUBCOOASY-611.md`; `archive/tickets/8CR3PUBCOOASY-621.md`; `archive/tickets/8CR3PUBCOOASY-631.md`; `archive/tickets/8CR3PUBCOOASY-641.md`; characterization report | wrappers are dev-only; exporter/replay/setup/domain authorities and no-leak behavior unchanged | remove profile wrapper tests | next profile adoption |
| `flood_watch` | replay/setup/domain/public-export profiles; seat-private N/A | migrated / not applicable | `archive/tickets/8CR3PUBCOOASY-602.md`; `archive/tickets/8CR3PUBCOOASY-612.md`; `archive/tickets/8CR3PUBCOOASY-622.md`; `archive/tickets/8CR3PUBCOOASY-632.md`; characterization report | wrappers are dev-only; no official seat-private timeline exists | remove profile wrappers; no N/A code rollback | next profile adoption or real seat-private export |
| `frontier_control` | replay/setup/domain/public-export profiles; seat-private N/A | migrated / not applicable | `archive/tickets/8CR3PUBCOOASY-603.md`; `archive/tickets/8CR3PUBCOOASY-613.md`; `archive/tickets/8CR3PUBCOOASY-623.md`; `archive/tickets/8CR3PUBCOOASY-633.md`; characterization report | wrappers are dev-only; fully public exporter unchanged; no private timeline exists | remove profile wrappers; no N/A code rollback | next profile adoption or real private timeline |
| `event_frontier` | replay/setup/domain/public-export profiles; seat-private N/A | migrated / not applicable | `archive/tickets/8CR3PUBCOOASY-604.md`; `archive/tickets/8CR3PUBCOOASY-614.md`; `archive/tickets/8CR3PUBCOOASY-624.md`; `archive/tickets/8CR3PUBCOOASY-634.md`; characterization report | wrappers are dev-only; hidden deeper deck remains absent from public export; no per-seat private timeline exists | remove profile wrappers; no N/A code rollback | next profile adoption or real seat-private export |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | replay/public-export/seat-private/domain profiles; setup pilot credit | migrated / already discharged | `archive/tickets/8CR4NSEAPRITRI-025.md`; `archive/tickets/8CR4NSEAPRITRI-026.md`; `archive/tickets/8CR4NSEAPRITRI-027.md`; `archive/tickets/8CR4NSEAPRITRI-028.md`; `archive/tickets/UNI8CMECSCA-024.md`; characterization report | wrappers are dev-only; command/export/domain behavior delegates to River; golden/fixture bytes unchanged | remove profile wrapper tests only | next River profile adoption |
| `briar_circuit` | replay/setup/public-export/seat-private profiles; domain pilot credit | migrated / already discharged | `archive/tickets/8CR4NSEAPRITRI-029.md`; `archive/tickets/8CR4NSEAPRITRI-030.md`; `archive/tickets/8CR4NSEAPRITRI-031.md`; `archive/tickets/8CR4NSEAPRITRI-032.md`; `archive/tickets/UNI8CMECSCA-026.md`; characterization report | wrappers are dev-only; setup/export/replay behavior delegates to Briar; golden/fixture bytes unchanged | remove profile wrapper tests only | next Briar profile adoption |
| `vow_tide` | replay/setup/domain profiles; public/seat-private export pilot credit | migrated / already discharged | `archive/tickets/8CR4NSEAPRITRI-033.md`; `archive/tickets/8CR4NSEAPRITRI-034.md`; `archive/tickets/8CR4NSEAPRITRI-035.md`; `archive/tickets/UNI8CMECSCA-025.md`; characterization report | wrappers are dev-only; bid/play/setup/domain behavior delegates to Vow; golden/fixture bytes unchanged | remove profile wrapper tests only | next Vow profile adoption |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | setup shuffle bounded-index sampler | migrated | `archive/tickets/UNI8CR2TWOSEA-042.md`; characterization report | fixed words, rejection counts, deck/deal vectors, replay, export, and visibility unchanged | restore local helper and call | future explicit RNG migration |
| `secret_draft` | setup shuffle bounded-index sampler | not applicable | characterization report C-09 matrix | no local unbiased bounded-index helper in R2 scope; no RNG/hash/visibility change | no R2 change to roll back | first Secret bounded-index helper migration |
| `poker_lite` | setup shuffle bounded-index sampler | migrated | `archive/tickets/UNI8CR2TWOSEA-043.md`; characterization report | fixed words, rejection counts, private-hand deal vectors, showdown/yield traces, and visibility unchanged | restore local helper and call | future explicit RNG migration |
| `masked_claims` | setup shuffle bounded-index sampler | migrated | `archive/tickets/UNI8CR2TWOSEA-044.md`; characterization report | fixed words, rejection counts, hands/reserve vectors, pending-claim/export redaction, and visibility unchanged | restore local helper and call | future explicit RNG migration |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | setup shuffle bounded-index sampler | migrated | `archive/tickets/8CR3PUBCOOASY-701.md`; characterization report | fixed words, rejection counts, deck/deal order, hands/tail, replay, export, fixture, and visibility unchanged | restore local helper and call | future explicit RNG migration |
| `flood_watch` | event-deck shuffle bounded-index sampler | migrated | `archive/tickets/8CR3PUBCOOASY-702.md`; characterization report | fixed words, rejection counts, event-deck/forecast order, replay, export, fixture, and visibility unchanged | restore local helper and call | future explicit RNG migration |
| `event_frontier` | epoch shuffle and non-Reckoning swap bounded-index sampler | migrated | `archive/tickets/8CR3PUBCOOASY-703.md`; characterization report | fixed words, rejection counts, per-epoch order, current/next/deeper-tail privacy, replay, export, fixture, and visibility unchanged | restore local helper and setup call sites | future explicit RNG migration |
| `frontier_control` | bounded-index sampler | not applicable | characterization report C-09 matrix | setup is RNG-free; no RNG/hash/visibility change | no R3 change to roll back | first Frontier RNG-consuming setup surface |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | setup shuffle bounded-index sampler | already discharged by Unit 8C pilot | `archive/tickets/UNI8CMECSCA-017.md`; characterization report | River continues to use `next_index_unbiased_v1`; setup/replay/fixture/no-leak evidence remains pilot-owned | N/A | future explicit RNG migration |
| `briar_circuit` | setup shuffle bounded-index sampler | not applicable to in-wave substitution | characterization report C-09 checkpoint; `archive/tickets/8CR4NSEAPRITRI-036.md` | legacy modulo `next_index(index + 1)` semantics, draw count, deck/deal bytes, replay, fixture, and visibility unchanged | no R4 code rollback | separately accepted ADR-0009 RNG algorithm migration |
| `vow_tide` | setup shuffle bounded-index sampler | not applicable to in-wave substitution | characterization report C-09 checkpoint; `archive/tickets/8CR4NSEAPRITRI-036.md` | legacy modulo `next_index(index + 1)` semantics, draw count, deck/deal bytes, replay, fixture, and visibility unchanged | no R4 code rollback | separately accepted ADR-0009 RNG algorithm migration |

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

R2 two-seat hidden/reaction receipts, 2026-06-23:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `high_card_duel` | deal/reveal/projection/reaction/scoring/outcome/bot policy bundle | rejected / local-only reaffirmed | `archive/tickets/UNI8CR2TWOSEA-045.md`; characterization report C-10 consolidation | no behavior, legality, reveal timing, projection, scoring, outcome, bot, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or proposed HCD behavioral extraction |
| `secret_draft` | draft/reveal/projection/reaction/outcome/bot policy bundle | rejected / local-only reaffirmed | `archive/tickets/UNI8CR2TWOSEA-045.md`; characterization report C-10 consolidation | no behavior, legality, reveal timing, projection, outcome, bot, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or proposed Secret behavioral extraction |
| `poker_lite` | pledge/pot/showdown/yield/projection/scoring/outcome/bot policy bundle | rejected / local-only reaffirmed | `archive/tickets/UNI8CR2TWOSEA-045.md`; characterization report C-10 consolidation | no pledge, pot, showdown, yield, projection, scoring, outcome, bot, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or proposed Poker behavioral extraction |
| `masked_claims` | claim/reaction/reveal/projection/scoring/outcome/bot policy bundle | rejected / local-only reaffirmed | `archive/tickets/UNI8CR2TWOSEA-045.md`; characterization report C-10 consolidation | no claim, reaction-window, reveal, projection, scoring, outcome, bot, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or proposed Masked behavioral extraction |

R3 public cooperative/asymmetric trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `plain_tricks` | deal/shuffle, trick lifecycle, follow-suit, scoring/outcome, projection, bot, and diagnostic policy | rejected / local-only reaffirmed | characterization report C-10 matrix | no behavior, legality, setup policy, projection, scoring, outcome, bot, diagnostic policy, TypeScript authority, YAML, DSL, or static behavior moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Plain behavioral extraction proposal |
| `flood_watch` | forecast/draw/reprieve, levee/flood/budget, shared outcome, projection, bot, and diagnostic policy | rejected / local-only reaffirmed | characterization report C-10 matrix | no event/flood/resource policy or hidden-deck visibility policy moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Flood behavioral extraction proposal |
| `frontier_control` | factions, graph/topology, adjacency/movement, clash, caps, connectivity, scoring, projection, bot, and diagnostic policy | rejected / local-only reaffirmed | characterization report C-10 matrix | no graph, legality, connectivity, scoring, or faction policy moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Frontier behavioral extraction proposal |
| `event_frontier` | factions, graph/trails, events/edicts, eligibility, funding/pass/Reckoning income, caps, scoring, projection, bot, and diagnostic policy | rejected / local-only reaffirmed | characterization report C-10 matrix | no event, resource, graph, eligibility, scoring, or hidden-deck policy moved to shared code | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Event behavioral extraction proposal |

R4 N-seat/private/trick receipts, 2026-06-24:

| Game/scope | Surface | Decision state | Evidence link | Hash/visibility impact | Rollback | Next review trigger |
|---|---|---|---|---|---|---|
| `river_ledger` | betting/all-in/reopen, stacks/contributions, pots/side pots, allocation, uncalled returns, showdown/evaluator, projection, scoring, and bots | rejected / local-only reaffirmed | characterization report C-10 checkpoint; `archive/tickets/8CR4NSEAPRITRI-036.md` | no behavior, legality, accounting, evaluator, projection, scoring, outcome, bot, YAML, DSL, or static behavior moved to shared scaffolding | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or River behavioral extraction proposal |
| `briar_circuit` | deal/pass/exchange, follow suit, first-trick exception, hearts-broken, trick winner/leader, moon/scoring, projection, and bots | rejected / local-only reaffirmed | characterization report C-10 checkpoint; `archive/tickets/8CR4NSEAPRITRI-036.md` | no trick, pass, deal, scoring, projection, bot, YAML, DSL, or static behavior moved to shared scaffolding; existing `game-stdlib::trick_taking` helper not broadened | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Briar behavioral extraction proposal |
| `vow_tide` | dealer rotation, hand schedule, deal/stock/trump, bidding/contract/hook, follow suit, trick winner/leader, exact-bid scoring, projection, and bots | rejected / local-only reaffirmed | characterization report C-10 checkpoint; `archive/tickets/8CR4NSEAPRITRI-036.md` | no schedule, deal, bidding, trick, scoring, projection, bot, YAML, DSL, or static behavior moved to shared scaffolding; existing `game-stdlib::trick_taking` helper not broadened | revert offending promotion and route through mechanic atlas/ADR | next mechanic-ladder gate or Vow behavioral extraction proposal |

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

### Unit 8C-R1 closeout evidence - 2026-06-23

- Final state: R1 public fixed-seat receipts are recorded under
  `MSC-8C-001`...`MSC-8C-010` above. Migrated surfaces, not-applicable
  decisions, accepted exceptions, rollback notes, and next-review triggers are
  closed for `race_to_n`, `draughts_lite`, `three_marks`, `column_four`,
  `directional_flip`, and `token_bazaar`.
- Final validators: the full §7.1 command set in
  `archive/specs/8c-r1-public-fixed-seat-scaffolding.md` passed, including focused
  crates, `cargo test --workspace --all-targets`, six `replay-check --all`
  runs, six `fixture-check` runs, `bash scripts/boundary-check.sh`,
  `cargo tree --workspace -e normal --invert game-test-support`,
  `node scripts/check-doc-links.mjs`, and
  `node scripts/check-catalog-docs.mjs`.
- Scope result: the golden/fixture diff audit from the pre-series base
  `d8061fcf8e974a25fdad15a8bf044891476265b2` reported exactly the six
  authorized `wasm-exported.trace.json` files and no fixture changes.
  Successor rows `8C-R2`, `8C-R3`, `8C-R4`, and Gate 18 remain unimplemented.

### Unit 8C-R3 closeout evidence - 2026-06-24

- Final state: R3 public cooperative/asymmetric trick receipts are recorded
  under `MSC-8C-001`...`MSC-8C-010` above for `plain_tricks`, `flood_watch`,
  `frontier_control`, and `event_frontier`. Migrated surfaces,
  not-applicable decisions, accepted exceptions, rollback notes, and
  next-review triggers are closed for the R3 wave.
- C-06 checkpoint: all four official R3 games keep `game-test-support` as a
  dev-only dependency; production dependency inversion remains clean.
- C-09 checkpoint: Plain/Flood/Event migrated local unbiased samplers to
  `DeterministicRng::next_index_unbiased_v1` with replay/fixture identity;
  Frontier Control is explicitly not applicable because setup is RNG-free.
- C-10 checkpoint: every behavior-bearing extraction remains rejected /
  local-only. `docs/MECHANIC-ATLAS.md` section 10A still reports
  `Current debt: _None_`.
- Final validators for the register receipt ticket: `node
  scripts/check-doc-links.mjs`, grep review for `MSC-8C-001`...`MSC-8C-010`,
  and `git diff --check`.

### Unit 8C-R4 receipt checkpoint evidence - 2026-06-24

- Receipt state: R4 N-seat/private/trick receipts are recorded under
  `MSC-8C-001`...`MSC-8C-010` above for `river_ledger`, `briar_circuit`, and
  `vow_tide`. Migrated surfaces, pilot-credit rows, not-applicable decisions,
  accepted exceptions, rollback notes, and next-review triggers are present for
  the register receipt checkpoint.
- C-06 checkpoint: all three R4 games keep `game-test-support` as a dev-only
  dependency; normal and normal/build inverse dependency checks are required
  evidence for `8CR4NSEAPRITRI-036`.
- C-09 checkpoint: River keeps the Unit 8C unbiased-sampler pilot receipt.
  Briar and Vow keep legacy modulo `next_index(index + 1)` semantics and close
  R4 as not applicable to in-wave substitution; a separate ADR-0009 RNG
  migration is the next-review trigger.
- C-10 checkpoint: River, Briar, and Vow behavior bundles remain rejected /
  local-only. No `MSC-8C-*` helper contract is broadened, no new helper entry
  is created, and the existing `game-stdlib::trick_taking` helper is not
  reclassified by this register.
- Final tracker flip, final command evidence, and the Unit 8C-R4 final closeout
  block remain owned by `8CR4NSEAPRITRI-037`.

### Unit 8C-R4 closeout evidence - 2026-06-24

- Final state: R4 N-seat/private/trick receipts are recorded under
  `MSC-8C-001`...`MSC-8C-010` above for `river_ledger`, `briar_circuit`, and
  `vow_tide`. Migrated surfaces, pilot-credit rows, not-applicable decisions,
  accepted exceptions, rollback notes, and next-review triggers are closed for
  the R4 wave.
- C-06 checkpoint: `game-test-support` remains dev-only; both inverse
  dependency checks reported only `game-test-support` itself, and
  `bash scripts/boundary-check.sh` passed.
- C-09 checkpoint: River remains pilot-discharged through the unbiased sampler
  receipt. Briar and Vow retain legacy modulo sampler semantics and are closed
  as not applicable to in-wave substitution; a separately accepted ADR-0009 RNG
  algorithm migration remains the next-review trigger.
- C-10 checkpoint: every behavior-bearing extraction remains rejected /
  local-only. No `MSC-8C-*` helper was broadened, no new helper entry was
  created, and `game-stdlib::trick_taking` was not reclassified as mechanical
  scaffolding.
- Artifact-diff result: existing golden traces, data fixtures, committed export
  artifacts, legacy hashes, visibility authority, seat authority, and RNG
  output authorities remain unchanged. R4 additions are migrated helper calls,
  parallel action-tree v1 vectors, test-only profile/no-leak adapters, and
  documentation/tracker receipts. Unauthorized artifact changes: zero.
- Final validators: `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, focused crate/game/tool tests,
  `cargo test --workspace`, River/Briar/Vow `replay-check`, `fixture-check`,
  and `rule-coverage`, `bash scripts/boundary-check.sh`, both inverse
  `cargo tree` checks, `node scripts/check-doc-links.mjs`, and
  `node scripts/check-catalog-docs.mjs` all passed for `8CR4NSEAPRITRI-037`.
- Tracker/interlock result: `specs/README.md` flips only `8C-R4` to `Done`.
  All four C-11 follow-on waves are now closed or explicitly disposed, clearing
  the final C-11 Gate 18 admission interlock. Gate 18 remains unstarted and
  unauthored.

### Gate 18 forward-v1 Blackglass Pact receipt - 2026-06-25

- Receipt state: Blackglass Pact is the first official game admitted after the
  forward-v1 interlock. It reviewed `MSC-8C-001`...`MSC-8C-010` in
  [../games/blackglass_pact/docs/MECHANICS.md](../games/blackglass_pact/docs/MECHANICS.md)
  and records the machine receipt in `ci/scaffolding-audits.json` with
  `coverage: "forward-v1"`.
- C-01 checkpoint: effect envelopes are reused as existing scaffolding only;
  blind, deal, bid, play, score, bag, and terminal effect meanings stay
  `blackglass_pact` behavior.
- C-02 checkpoint: canonical seat grammar fits `seat_0`...`seat_3`; team IDs
  remain game-local and do not broaden seat identity.
- C-03 checkpoint: exact-four setup and clockwise arithmetic reuse structural
  count/ring helpers where applicable; dealer, blind order, bid order,
  partnership, and winner-leads policy stay game-local.
- C-04/C-05 checkpoint: action-tree and stable-byte helpers remain framing or
  evidence surfaces only; legal leaves, accepted actions, replay authority, and
  hashes are not migrated by this gate.
- C-06 checkpoint: any `game-test-support` use remains dev/test-only; no
  production edge is introduced.
- C-07/C-08 checkpoint: pairwise no-leak and evidence-profile geometry is
  reused as test scaffolding; Blackglass supplies the hidden datum taxonomy,
  observer/seat expectations, and replay/export policy.
- C-09 checkpoint: bounded-index sampling creates no migration authority for
  Blackglass shuffle/deal bytes; any RNG migration would require separate
  authority.
- C-10 checkpoint: every behavior-bearing shape remains rejected/local-only:
  partnerships, team scoring, nil/blind nil, bags, contracts, broken-spades
  policy, bots, visibility, outcome, and UI behavior do not enter mechanical
  scaffolding.
- Prior-site disposition: Vow Tide is the numeric trick-contract comparison
  point and earlier trick games are helper-conformance comparison points. No
  prior-game migration or follow-on unit is required; the accepted no-unit
  disposition is `MSC-8C-010`.
- Artifact-diff result: no new `MSC-*` register entry is required, no existing
  helper contract is broadened, no `engine-core` noun is added, and no
  `game-stdlib` helper promotion debt is created.

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
- a new game's pre-implementation audit and post-implementation closeout are both linked;
- every new first-use scaffolding shape is registered without implying premature promotion;
- prior matching official-game sites are complete;
- a required follow-on unit exists in `specs/README.md`, or the accepted no-unit disposition carries rationale, owner, evidence, and next review trigger;
- the CI audit receipt agrees with the human evidence and register state.
