# Unit 8C-R4 Characterization Report

Status: baseline for `8CR4NSEAPRITRI-001`; migration receipts and final
closeout are owned by later tickets in the series.

## Evidence Basis

- Repository: `/home/joeloverbeck/projects/rulepath`.
- Baseline commit: `9c5b4c8730fc917af88aefdfae7e641c258e94d5`.
- Reference spec:
  `specs/8c-r4-n-seat-private-trick-scaffolding-intermediate-spec.md`.
- Active ticket: `tickets/8CR4NSEAPRITRI-001.md`.
- Foundation authority: `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`,
  `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`,
  `docs/TRACE-SCHEMA-v1.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`,
  `docs/EVIDENCE-FIXTURE-CONTRACT.md`, ADR 0004, ADR 0008, and ADR 0009.

This report records current behavior only. It does not change code, schemas,
golden traces, fixtures, canonical bytes, hashes, seat IDs, visibility policy,
or RNG algorithms.

## Locked Determination

`8C-R4` is the lowest active public-scaling row not yet completed in
`specs/README.md`. `8C-R3` is `Done`; `8C-R4` is the required final C-11
follow-on wave before Gate 18. `docs/MECHANIC-ATLAS.md` section 10A records
`Current debt: _None_`, so no open behavioral primitive debt blocks this
mechanical-scaffolding retrofit.

The bounded game set is exactly:

| Game | Seat range | R4 stance |
|---|---:|---|
| `river_ledger` | 3-6 | residual audit only over named pilot leftovers |
| `briar_circuit` | fixed 4 | C-01 through C-08 residual migration/receipt wave |
| `vow_tide` | 3-7 | C-01 through C-08 residual migration/receipt wave |

R4 closeout clears only the C-11 portion of Gate 18's admission interlock.
Gate 18 authoring, partnership behavior, team grouping, Spades rules, and any
new helper contract remain out of scope.

Grounded corrections carried forward from the spec:

- River's Gate 15.1 all-in/side-pot work predates Unit 8C; R4 audits only the
  residual surfaces named by the River pilot receipts.
- Vow Tide was not a C-02 pilot; its game and WASM seat grammar remain residual
  R4 work.
- Vow Tide has private hands but no private effect-envelope class; Vow C-01
  private is `not-applicable`.
- Briar and Vow use legacy modulo `DeterministicRng::next_index`; in-wave C-09
  substitution is `not-applicable` without a separate ADR-0009 RNG migration.

## Pilot Receipt Inventory

| Game | Helper/profile cell | Verdict | Register entry | Receipt |
|---|---|---|---|---|
| `river_ledger` | C-01 public/private effect envelopes | already-discharged-by-8C-pilot | `MSC-8C-001` | Unit 8C C-01 pilot; current seam `games/river_ledger/src/effects.rs::{public_effect,private_effect}` calls `EffectEnvelope::{public,private_to}` |
| `river_ledger` | C-02 canonical seats | already-discharged-by-8C-pilot | `MSC-8C-002` | `archive/tickets/UNI8CMECSCA-009.md` |
| `river_ledger` | C-03 3-6 range and ring index | already-discharged-by-8C-pilot | `MSC-8C-003` | `archive/tickets/UNI8CMECSCA-011.md` |
| `river_ledger` | C-06 dev-only support | already-discharged-by-8C-pilot | `MSC-8C-006` | `archive/tickets/UNI8CMECSCA-021.md` |
| `river_ledger` | C-07 base N-seat no-leak matrix | already-discharged-by-8C-pilot | `MSC-8C-007` | `archive/tickets/UNI8CMECSCA-021.md` |
| `river_ledger` | C-08 setup-evidence profile | already-discharged-by-8C-pilot | `MSC-8C-008` | `archive/tickets/UNI8CMECSCA-024.md` |
| `river_ledger` | C-09 unbiased bounded index | already-discharged-by-8C-pilot | `MSC-8C-009` | `archive/tickets/UNI8CMECSCA-017.md` |
| `briar_circuit` | C-06 dev-only support | already-discharged-by-8C-pilot | `MSC-8C-006` | `archive/tickets/UNI8CMECSCA-026.md` |
| `briar_circuit` | C-08 domain-evidence profile | already-discharged-by-8C-pilot | `MSC-8C-008` | `archive/tickets/UNI8CMECSCA-026.md` |
| `vow_tide` | C-06 dev-only support | already-discharged-by-8C-pilot | `MSC-8C-006` | `archive/tickets/UNI8CMECSCA-025.md` |
| `vow_tide` | C-08 public/seat-private export profiles | already-discharged-by-8C-pilot | `MSC-8C-008` | `archive/tickets/UNI8CMECSCA-025.md` |

## Aggregate Verdict Matrix

| Game | C-01 envelopes | C-02 seats | C-03 count/ring | C-04 action tree | C-05 stable bytes | C-06 dev-only support | C-07 no-leak | C-08 profiles | C-09 bounded index |
|---|---|---|---|---|---|---|---|---|---|
| `river_ledger` | already-discharged-by-8C-pilot | already-discharged-by-8C-pilot | already-discharged-by-8C-pilot | migrate | migrate, action-tree v1 only | already-discharged-by-8C-pilot | migrate, residual stack/pot/multipot adapters only | migrate, residual replay/public/seat-private/domain profiles | already-discharged-by-8C-pilot |
| `briar_circuit` | migrate public/private | migrate | migrate structural exact-four only | migrate | migrate, action-tree v1 only | already-discharged-by-8C-pilot | migrate | migrate, excluding domain pilot credit | not-applicable |
| `vow_tide` | migrate public only; private N/A | migrate | migrate structural 3-7 and checked ring step | migrate | migrate, action-tree v1 only | already-discharged-by-8C-pilot | migrate | migrate, excluding public/seat-private export pilot credit | not-applicable |

## Sub-Surface Dispositions

### C-01 Effect Envelopes

| Game / sub-surface | Owner and seam | Current constructor/hash authority | Verdict | Rollback unit |
|---|---|---|---|---|
| River public | `games/river_ledger/src/effects.rs::public_effect` | `EffectEnvelope::public`; effect bytes/hashes remain River replay-support authority | already-discharged-by-8C-pilot | none |
| River private | `games/river_ledger/src/effects.rs::private_effect` | `EffectEnvelope::private_to`; owner scope supplied by River | already-discharged-by-8C-pilot | none |
| Briar public | `games/briar_circuit/src/visibility.rs::effect_envelopes` | literal `EffectEnvelope { visibility: Public, payload }`; legacy effect hash/scope authority | migrate | restore public literal only |
| Briar private | `games/briar_circuit/src/visibility.rs::private_effect` | literal `PrivateToSeat(SeatId(seat.as_str()))`; pass reveal policy stays Briar-owned | migrate | restore `private_effect` literal only |
| Vow public | `crates/wasm-api/src/games/vow.rs::vow_apply_command` | literal public envelope map over Vow effects | migrate | restore literal map only |
| Vow private | no private effect-envelope class exists | private hands live in views/exports, not effects | not-applicable | none |

### C-02 Seat Grammar

| Game / sub-surface | Owner and seam | Current accepted canonical strings | Legacy/import strings | Verdict |
|---|---|---|---|---|
| River parser/formatter | `games/river_ledger/src/ids.rs::{RiverLedgerSeat::parse,as_str,seat_id_for_index}` | `seat_0` through `seat_5` depending on count | shared import compatibility from prior pilot | already-discharged-by-8C-pilot |
| Briar parser | `games/briar_circuit/src/ids.rs::BriarCircuitSeat::parse` | `seat_0` through `seat_3` | none in game parser | migrate |
| Briar formatter/roster | `games/briar_circuit/src/ids.rs::{as_str,seat_id_for_index,canonical_seat_ids}` | `seat_0` through `seat_3` | N/A output-only | migrate |
| Briar WASM import | `crates/wasm-api/src/games/briar.rs::parse_briar_seat` | `seat_0` through `seat_3` | `seat-0` through `seat-3` | migrate |
| Vow parser | `games/vow_tide/src/ids.rs::VowTideSeat::parse` | `seat_0` through `seat_6` | none in game parser | migrate |
| Vow formatter/roster | `games/vow_tide/src/ids.rs::{as_str,seat_id_for_index,canonical_seat_ids}` | selected prefix of `seat_0` through `seat_6` for counts 3-7 | N/A output-only | migrate |
| Vow WASM import | `crates/wasm-api/src/games/vow.rs::parse_vow_seat` | `seat_0` through `seat_6` | `seat-0` through `seat-6` | migrate |
| Non-seat IDs | cards, tricks, pots, contracts, hand IDs | N/A | N/A | not-applicable |

Rejected examples for Briar/Vow strict game parsers: `seat-0`, `seat_a`,
`seat_4` for Briar, `seat_7` for Vow, malformed strings, and empty strings.
WASM import accepts only the bounded hyphen legacy forms named above.

### C-03 Structural Seat Count/Ring

| Game / sub-surface | Owner and seam | Current diagnostic/hash authority | Verdict |
|---|---|---|---|
| River 3-6 admission | `games/river_ledger/src/setup.rs::river_seat_count` | `SeatCountRange::inclusive(3, 6)`; River diagnostic remains local | already-discharged-by-8C-pilot |
| River ring index | `games/river_ledger/src/setup.rs::next_ring_seat` | `SeatCount::next_ring_index`; button/blind policy remains River-owned | already-discharged-by-8C-pilot |
| River stack vector cardinality | River setup/resource policy | no generic resource-vector helper | not-applicable |
| Briar exact-four setup | `games/briar_circuit/src/setup.rs::setup_match` | local length comparison; diagnostic code `BC_UNSUPPORTED_SEAT_COUNT` | migrate |
| Briar pass/dealer topology | `games/briar_circuit/src/ids.rs::{next_clockwise,pass_left_target,pass_right_target,pass_across_target}` | pass/dealer behavior | exception |
| Vow 3-7 admission | `games/vow_tide/src/ids.rs::supported_seat_count`; setup and WASM creation checks | local range checks; diagnostic code `VT_INVALID_SEAT_COUNT` | migrate |
| Vow ring step | `games/vow_tide/src/ids.rs::next_clockwise`; `games/vow_tide/src/setup.rs::deal_order_after` | local modulo under game-owned deal/bid policy | migrate |
| Vow schedule/deal capacity | `hand_schedule_for_seats`, `max_hand_size_for_seats`, `deal_hand` capacity | Vow game policy | exception |

### C-04/C-05 Action Tree And Adjacent Hashes

| Game | Current action surface | Current hash/byte authority | Verdict |
|---|---|---|---|
| River | `games/river_ledger/src/actions.rs::legal_action_tree` | `games/river_ledger/src/replay_support.rs::action_tree_hash` hashes Debug text; selected export hash sentinel `7443748736294317283` in `games/river_ledger/tests/replay.rs` | migrate parallel v1 only |
| Briar | WASM builds browser JSON in `crates/wasm-api/src/games/briar.rs::briar_action_tree_json`; game replay hashes projected previews | `games/briar_circuit/src/replay_support.rs::action_hash`; sentinel `expected_public_action_hash=675868731199239589` in bot-match traces | migrate typed parity adapter, then parallel v1 only |
| Vow | `games/vow_tide/src/actions.rs::legal_action_tree` | `games/vow_tide/src/replay_support.rs::snapshot` hashes Debug legal tree text; export/hash sentinels in `games/vow_tide/tests/replay.rs`: public trace hash `9606057229737834804`, seat-private trace hash `17095095643214376875`, observer export hash `14136592432406028852`, seat_0 export hash `12688236753872554050` | migrate parallel v1 only |

Adjacent C-05 surfaces remain exceptions unless a later ticket names a separate
authority flip: legacy action-tree/debug hash, state bytes/hash, effect
bytes/hash, public/seat-private view bytes/hash, replay-command bytes/hash,
public/seat-private export bytes/hash, and diagnostic bytes/hash.

### C-07 Pairwise No-Leak Geometry

| Game | Enumeration | Current proof surface | R4 residual |
|---|---:|---|---|
| River | counts 3,4,5,6; each source seat; observer plus every seat viewer | `games/river_ledger/tests/visibility.rs::no_leak_harness_covers_full_n_seat_river_matrix` via `assert_pairwise_no_leak`; receipt `UNI8CMECSCA-021` | selected all-in stack lifecycle, automatic runout, multipot export, and side-pot accounting absence-of-card adapters |
| Briar | 4 source seats x 5 viewers | existing focused visibility/replay traces; R4 will add shared matrix | private hand, pass selection/exchange, public play reveal timing, bot/export no-leak |
| Vow | counts 3-7; each source seat; observer plus every declared seat viewer | existing `games/vow_tide/tests/visibility.rs::exhaustive_seat_pair_no_leak_for_three_to_seven_players`; export profile pilot for all viewers | private hand, hidden stock, bid/trick/export/bot matrix coverage through shared geometry |

Canary strings are test-generated only. No R4 canary may be committed to a
trace, fixture, export, snapshot, log, or test identifier.

### C-08 Evidence Profiles

| Profile | River | Briar | Vow |
|---|---|---|---|
| `replay-command-v1` | migrate; command traces including selected Gate 15.1 paths | migrate; pass/play traces | migrate; bid/play traces, representative 3p and 7p |
| `setup-evidence-v1` | already-discharged-by-8C-pilot via `UNI8CMECSCA-024` | migrate; fixed-four deterministic setup metadata | migrate; 3p/7p schedule/deal metadata |
| `public-export-v1` | migrate; observer export including multipot terminal path | migrate; current public replay/export path | already-discharged-by-8C-pilot via `UNI8CMECSCA-025` |
| `seat-private-export-v1` | migrate; viewers for 3-6 including multipot path | migrate; all four seats | already-discharged-by-8C-pilot via `UNI8CMECSCA-025` |
| `domain-evidence-v1` | migrate; all-in/side-pot allocation evidence, byte authority `none` unless existing validator owns bytes | already-discharged-by-8C-pilot via `UNI8CMECSCA-026` | migrate; hook/dealer-bid and terminal-tie evidence |

Profile drivers validate metadata and delegate behavior to game-owned code.
They do not parse commands, set up games, project views, authorize exports,
interpret side pots, decide tricks, or score outcomes from data.

### C-06/C-09/C-10 Checkpoints

| Game | C-06 | C-09 | C-10 non-promotion bundle |
|---|---|---|---|
| River | dev-only `game-test-support` receipt in `UNI8CMECSCA-021` | `next_index_unbiased_v1` receipt in `UNI8CMECSCA-017`; current seam `games/river_ledger/src/setup.rs::shuffle_deck` | betting, all-in/reopen, stacks/contributions, pots, side pots, allocation, uncalled returns, showdown/evaluator, projection, scoring, and bots stay River-owned |
| Briar | dev-only `game-test-support` receipt in `UNI8CMECSCA-026` | not-applicable; current setup seam `games/briar_circuit/src/setup.rs::shuffle_deck` uses legacy `next_index(index + 1)` | deal/pass/exchange, follow suit, first-trick exception, hearts-broken, trick winner/leader, moon/scoring, projection, and bots stay Briar-owned |
| Vow | dev-only `game-test-support` receipt in `UNI8CMECSCA-025` | not-applicable; current setup seam `games/vow_tide/src/setup.rs::shuffle_deck` uses legacy `next_index(index + 1)` | dealer rotation, hand schedule, deal/stock/trump, bidding/contract/hook, follow suit, trick winner/leader, exact-bid scoring, projection, and bots stay Vow-owned |

## Seat And Viewer Enumeration Counts

| Game | Seat counts | Matrix viewers | Source x viewer products |
|---|---|---|---|
| River | 3, 4, 5, 6 | observer plus all seats | 3x4=12; 4x5=20; 5x6=30; 6x7=42; total 104 |
| Briar | 4 | observer plus 4 seats | 4x5=20 |
| Vow | 3, 4, 5, 6, 7 | observer plus all seats | 3x4=12; 4x5=20; 5x6=30; 6x7=42; 7x8=56; total 160 |

## RNG Baseline

| Game | Current setup sampler | R4 verdict | Baseline statement |
|---|---|---|---|
| River | `DeterministicRng::next_index_unbiased_v1(index + 1)` | already-discharged-by-8C-pilot | `UNI8CMECSCA-017` pinned equivalence against the removed local unbiased helper, including rejected-draw behavior and `next_u64` consumption. |
| Briar | `DeterministicRng::next_index(index + 1)` | not-applicable to in-wave substitution | legacy modulo semantics stay current; any unbiased migration needs a separate ADR-0009 packet. |
| Vow | `DeterministicRng::next_index(index + 1)` | not-applicable to in-wave substitution | legacy modulo semantics stay current; any unbiased migration needs a separate ADR-0009 packet. |

## N/A And Exception Ledger

| Surface | Owner | Disposition | Compatibility and next review trigger |
|---|---|---|---|
| Vow private effect envelopes | Vow view/export code | not-applicable | first intentional Vow private effect class |
| Non-seat IDs | owning game modules | not-applicable | first proposal to route card/trick/pot/contract IDs through a seat helper is rejected by default |
| River stack-vector cardinality | River setup | not-applicable | first behavior-free cardinality helper proposal with non-resource semantics |
| Briar pass/dealer topology | Briar IDs/rules/setup | exception | separately reviewed behavior-free ring call that preserves pass/deal semantics |
| Vow hand schedule/deal capacity | Vow IDs/setup/rules | exception | separately reviewed migration proving schedule/deal policy stays local |
| Briar/Vow C-09 unbiased substitution | game setup | not-applicable | separately accepted ADR-0009 RNG algorithm migration |
| Legacy action/debug hashes | game replay support | exception | separately named hash-authority flip |
| State/effect/view/replay/export/diagnostic encodings | owning game modules and replay/export code | exception or N/A by existing surface | dedicated per-surface migration with no-leak/hash evidence |

## Golden, Fixture, And Export Diff Inventory

Ticket 001 changes no artifact bytes. Starting classification:

| Path class | Current classification |
|---|---|
| `games/river_ledger/tests/golden_traces/*` | unchanged |
| `games/briar_circuit/tests/golden_traces/*` | unchanged |
| `games/vow_tide/tests/golden_traces/*` | unchanged |
| `games/river_ledger/data/fixtures/*` | unchanged |
| `games/briar_circuit/data/fixtures/*` | unchanged |
| `games/vow_tide/data/fixtures/*` | unchanged |
| WASM export fixtures generated by tests | unchanged |

Future tickets must classify any changed artifact as `parallel-new` or
`intentional-migration` under ADR 0009. This report does not authorize blanket
golden regeneration.

## Baseline Verification

Commands run for ticket 001:

- `cargo run -p replay-check -- --game river_ledger --all` - passed; all River
  golden traces accepted, including all-in, multipot, setup 3p-6p, and
  viewer-export traces.
- `cargo run -p replay-check -- --game briar_circuit --all` - passed; all Briar
  golden traces accepted.
- `cargo run -p replay-check -- --game vow_tide --all` - passed; all Vow golden
  traces accepted.
- `cargo test --workspace` - passed; workspace tests and doc-tests completed
  successfully.

## Rollback Map

Ticket 001 rollback is report-only: remove this file before archival if the
baseline is rejected. Later migration tickets must provide one-surface rollback
steps in their own ticket outcomes and, for tickets 036/037, consolidate the
after-receipts here and in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
