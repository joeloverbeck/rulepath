# Unit 8C-R4 — C-11 follow-on: N-seat/private/trick mechanical-scaffolding residual retrofit

| Field | Value |
|---|---|
| Spec ID | `8C-R4` |
| Artifact slug | `8c-r4-n-seat-private-trick-scaffolding` |
| Roadmap stage | Public scaling phase — C-11 follow-on retrofit lane |
| Roadmap build gate | `8C-R4` — final C-11 wave; precedes Gate 18 |
| Status | `Planned` |
| Date | 2026-06-24 |
| Owner | Rulepath maintainers; implementation delegated through bounded `AGENT-TASK` packets using profile `scaffold-refactor` |
| Analysis baseline | Grounded against the repository working tree at commit `0d01901c24b21c6d4620b1d5a6c19d2e84fb4e6a` (HEAD at reassessment). |

> **Locked authored plan.** This is an intermediate implementation-spec artifact,
> not executed code and not a ready-to-land final repository file. Save it under
> `specs/`, then run `/reassess-spec` followed by `/spec-to-tickets`. Reassessment
> may correct one-line implementation details but may not reopen the unit, add or
> remove a game, absorb Gate 18, or weaken an upstream authority. The supplied
> requirements are treated as final; this authored plan asks no implementation
> questions and proceeds with explicit assumptions and stop conditions.

This spec is subordinate to the foundation set indexed by
[`docs/README.md`][docs-readme]. Where this spec and a higher-authority document
conflict, the higher-authority document wins. Authority order:
[`docs/FOUNDATIONS.md`][foundations] →
[`docs/ARCHITECTURE.md`][architecture] →
[`docs/ENGINE-GAME-DATA-BOUNDARY.md`][boundary] → accepted ADRs and area
contracts → [`docs/ROADMAP.md`][roadmap] → this spec → tickets.

**Repository evidence basis.** Repository-state claims below were grounded
against the working tree at the commit named in the header. References to
another repository inside a file are ordinary file content, not provenance
contamination. External research is separated in §7.6 and is not used to assert
repository state.

**Grounded corrections to the research brief.** These corrections narrow the
residual matrix without reopening the locked unit or game set:

1. **Gate 15.1 did not land after the River 8C pilot.**
   [`archive/specs/gate-15-1-river-ledger-all-in-side-pots.md`][gate-15-1]
   predates Unit 8C, and
   [`archive/tickets/UNI8CMECSCA-021.md`][ticket-021] expressly names
   all-in/reopen/pot/allocation as existing game-owned policy during the River
   no-leak pilot. River still has R4 residual work, but the residual is defined
   by the pilot's **named surfaces**, not by a false chronology. The pilot
   generalized one N-seat source × viewer × surface matrix while retaining
   separate stack/pot/multipot lifecycle assertions; R4 audits only the
   remaining profile and selected scenario-adapter debt.
2. **Vow Tide was not a C-02 pilot.** `MSC-8C-002` and the Unit 8C parent name
   Race to N and River Ledger as the canonical-seat pilots. Ticket
   [`UNI8CMECSCA-025`][ticket-025] explicitly leaves Vow setup, seat/ring, and
   RNG work out of scope. At the target commit,
   `games/vow_tide/src/ids.rs::{VowTideSeat::parse, as_str,
   seat_id_for_index}` and `crates/wasm-api/src/games/vow.rs::parse_vow_seat`
   remain manual. Vow C-02 is therefore `migrate`, not pilot credit.
3. **Vow Tide has private hands but no seat-private effect-envelope class.**
   Its effects are public game events, and
   `visibility::filter_effects_for_viewer` returns the public payloads. The
   actual C-01 duplicate is the public literal in
   `crates/wasm-api/src/games/vow.rs::vow_apply_command`. Thus Vow's public
   C-01 surface is `migrate`; private C-01 is `not-applicable`. Its C-07 and
   seat-private C-08 obligations remain fully applicable because its projected
   hands and viewer exports are genuinely private.
4. **Briar and Vow do not contain local unbiased rejection samplers.** Their
   setup shuffles call legacy `DeterministicRng::next_index`, which has modulo
   consumption semantics. River alone already migrated an algorithm-equivalent
   rejection sampler under [`UNI8CMECSCA-017`][ticket-017]. Therefore Briar and
   Vow C-09 are `not-applicable` to an in-wave helper substitution. Any move to
   `next_index_unbiased_v1` is a real RNG algorithm/version migration governed
   by ADR 0009 and must be separately admitted; it cannot be hidden in R4.

## 1. Determination

### 1.1 Locked determination

The next-unit determination is confirmed and documented, not re-decided:

1. [`specs/README.md`][spec-index] records `8C-R3` as `Done`, completed
   2026-06-24. `8C-R4` is the lowest active-epoch row whose status is not
   `Done`; it is `Not started` and names exactly `river_ledger` residual audit,
   `briar_circuit`, and `vow_tide`.
2. The Gate 18 row sits immediately below R4 and gates admission on several
   conditions — among them closure, explicit not-applicability, or accepted
   exception for all `8C-R1…8C-R4` waves, alongside the 8M realignment being
   `Done`, accepted ADRs 0008/0009, fixed trace profiles and canonical seat
   grammar, and the partnership/trick-taking atlas interlock. R4 is therefore
   the last C-11 row whose closeout clears only that specific C-11 item; the
   other named conditions persist. Clearing an interlock item is not permission
   to author or begin Gate 18 in this unit.
3. [`docs/MECHANIC-ATLAS.md`][atlas] §10A records `Current debt: _None_`.
   No primitive-promotion debt precedes R4, and R4 is a mechanical-scaffolding
   retrofit rather than a mechanic-ladder gate.
4. [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`][parent]
   §5 seeds the exact R4 game set and audit emphasis: 3–7 seat matrices,
   private hands, pass/bid/deal order, public and seat-private export, and RNG
   sampler divergence. Parent work item `8C-030` created all four C-11 rows.
   Parent EC-28 requires bounded waves covering every official game exactly
   once, with pilot games limited to residual audits; EC-30 keeps Gate 18 after
   all waves close or receive explicit accepted dispositions.
5. The direct structural precedents—
   [`8C-R1`][r1-spec], [`8C-R2`][r2-spec], and [`8C-R3`][r3-spec]—fix the
   execution shape: characterization before migration, a complete primary
   matrix, per-helper sub-surface dispositions, one selected surface per diff,
   ADR-0009 migration packets, register receipts, command evidence, and a final
   tracker flip. R2's High Card Duel treatment is the closest residual-audit
   precedent; R3 is the useful inverse because it had no pilot-credit cells.
6. [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`][register] and tickets
   `UNI8CMECSCA-017`, `-021`, `-024`, `-025`, and `-026` prove that the three
   R4 games are not blank adopters. The correct task is to preserve the named
   pilot discharge and close only the residual cells.

**Determination:** Unit `8C-R4` is the required current unit. Its bounded game
set is exactly:

- `river_ledger` — 3–6 seats; residual audit over a private-hand,
  finite-stack, all-in, side-pot Hold'Em-family implementation;
- `briar_circuit` — fixed four seats; private hands, simultaneous pass
  commitments/exchange, trick play, and moon scoring;
- `vow_tide` — 3–7 seats; private hands, variable hand schedules, dealer
  rotation, bidding/contract order, turn-up trump, and trick play.

No fourth game, Gate 18 partnership work, new mechanic, UI feature, bot strategy,
or new shared-helper contract is admitted.

### 1.2 Determination outcome and interlock effect

R4 closes successfully only when every listed helper and sub-surface has exactly
one disposition—`migrate`, `already-discharged-by-8C-pilot`,
`not-applicable`, or `exception`—with evidence and rollback. When that closeout
is recorded and the tracker row flips to `Done`, the C-11 portion of Gate 18's
admission condition becomes satisfied. The successor still begins only through
the normal tracker workflow; this spec neither writes nor executes Gate 18.

## 2. Objective

Within the C-11 retrofit lane established by the completed Unit 8C parent and
the public-scaling sequence in [`docs/ROADMAP.md`][roadmap], Unit 8C-R4 must
turn the final seed into a bounded, reversible residual implementation plan. It
must:

1. inventory the exact pilot-discharge boundary for each of the three games and
   prevent already-proven surfaces from being re-proposed as missing;
2. resolve every C-01…C-09 aggregate cell and every listed sub-surface to one
   explicit verdict, with no unowned cleanup bucket;
3. adopt only already-accepted behavior-free scaffolding: generic effect
   envelopes, canonical seat grammar, structural seat counts/ring indices,
   action-tree v1 bytes/hash, dev-only test support, pairwise no-leak geometry,
   evidence-profile metadata, and the already-shipped unbiased sampler where
   algorithm equivalence exists;
4. preserve all legacy byte/hash/seat/visibility/RNG authority unless a named
   per-surface ADR-0009 packet explicitly authorizes a parallel-new surface or
   separately accepted authority flip;
5. prove the widest seat and viewer products in the current corpus: River 3–6,
   Vow 3–7, and Briar fixed-4, including observer plus every declared seat;
6. close River's residual action-tree, evidence-profile, and selected
   stack/pot/multipot no-leak adapter debt without moving any betting, all-in,
   side-pot, evaluator, showdown, or allocation policy;
7. close Briar and Vow's residual C-01…C-08 surfaces while keeping pass, deal,
   dealer, bid, contract, trick, partnership, projection, and scoring policy
   game-owned;
8. record C-09 divergence honestly: River remains pilot-discharged; Briar and
   Vow remain on their characterized legacy sampler until a separate algorithm
   migration is accepted; and
9. add R4 receipts under `MSC-8C-001…010`, add an R4 closeout block, and flip
   only the `8C-R4` tracker row after all exit evidence passes.

R4 adds no shared helper and changes no accepted helper contract.

## 3. Scope

### 3.1 In scope

- Exactly the three games named in §1.
- A residual applicability audit of C-01…C-09 and a C-10 non-promotion
  reaffirmation.
- Explicit pilot-credit verification against register receipts and archived
  Unit 8C tickets.
- One-surface-per-diff migration to existing helper APIs where code proves
  applicability.
- Public and seat-private no-leak matrices appropriate to each game's actual
  hidden data and reveal rules.
- Public, seat-private, replay-command, setup, and domain profile classification
  using existing driver APIs; profile metadata remains test/evidence
  scaffolding rather than behavior.
- Parallel action-tree v1 bytes/hash with legacy authority preserved.
- Characterization of Briar/Vow RNG consumption and an explicit non-migration
  receipt unless a separately accepted ADR-0009 algorithm packet exists before
  implementation.
- Register receipts, characterization report, command evidence, and the final
  tracker status flip.

### 3.2 Verdict vocabulary

| Verdict | Meaning in R4 |
|---|---|
| `migrate` | A real target-commit surface is eligible for one bounded adoption of an already-shipped helper. The task owns one surface, characterization, compatibility, evidence, and rollback. |
| `already-discharged-by-8C-pilot` | Unit 8C already implemented and evidenced that exact named surface. R4 verifies the receipt and does not rebuild it. |
| `not-applicable` | The helper has no matching surface, or applying it would require a different behavior/algorithm migration outside R4. The rationale and next review trigger are recorded. |
| `exception` | A matching surface exists but remains on its current owner/encoding for compatibility or ownership reasons. The receipt names owner, compatibility window, rollback, and next review trigger. |

A migration task that cannot meet its characterized equality bar does not
quietly weaken the bar. It stops and resolves to a named exception or a separate
ADR-triggered migration.

### 3.3 Primary applicability and verdict matrix

| Game | C-01 envelopes | C-02 seats | C-03 count/ring | C-04 action tree | C-05 stable bytes | C-06 dev-only support | C-07 no-leak | C-08 profiles | C-09 bounded index |
|---|---|---|---|---|---|---|---|---|---|
| `river_ledger` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `migrate` | `migrate` | `already-discharged-by-8C-pilot` | `migrate` — residual stack/pot/multipot scenario adapters only; base matrix retains pilot credit | `migrate` — residual profiles | `already-discharged-by-8C-pilot` |
| `briar_circuit` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `already-discharged-by-8C-pilot` | `migrate` | `migrate` — domain profile retains pilot credit | `not-applicable` — current algorithm is legacy modulo; separate versioned migration required |
| `vow_tide` | `migrate` — public only; private constructor N/A | `migrate` | `migrate` | `migrate` | `migrate` | `already-discharged-by-8C-pilot` | `migrate` | `migrate` — public/seat-private profiles retain pilot credit | `not-applicable` — current algorithm is legacy modulo; separate versioned migration required |

The aggregate matrix is a spine, not a substitute for the sub-surface tables
below. An aggregate `migrate` cell may contain pilot-discharged or N/A rows that
must remain explicit.

#### 3.3.1 Explicit verdict columns

This closeout view restores `already-discharged-by-8C-pilot` as a first-class
column rather than hiding pilot credit inside prose.

| Game | `migrate` | `already-discharged-by-8C-pilot` | `not-applicable` | `exception` |
|---|---|---|---|---|
| `river_ledger` | C-04; selected C-05 action-tree v1; residual C-07 stack/runout/multipot adapters; residual C-08 replay/public/seat-private/domain profiles | C-01 public/private; C-02; C-03 range/ring; C-06; ticket-021 C-07 base matrix; C-08 setup; C-09 | further C-03 resource-vector extraction; non-seat C-02 IDs | adjacent legacy C-05 state/effect/view/replay/export/diagnostic authorities |
| `briar_circuit` | C-01 public/private; C-02; C-03 exact-count structure; C-04; selected C-05 action-tree v1; C-07; residual C-08 replay/setup/public/seat-private | C-06; C-08 domain | C-09 in-wave substitution; non-seat C-02 IDs | pass/dealer topology under C-03; adjacent legacy C-05 authorities |
| `vow_tide` | C-01 public; C-02; C-03 range/ring structure; C-04; selected C-05 action-tree v1; C-07; residual C-08 replay/setup/domain | C-06; C-08 public and seat-private export | C-01 private envelopes; C-09 in-wave substitution; non-seat C-02 IDs; C-03 behavioral diversity | hand schedule/deal capacity under C-03; adjacent legacy C-05 authorities |

Every item in this compact table is expanded below with exact seams, authority,
compatibility, rollback, and next-review rules.

### 3.4 C-01 — public and seat-private effect-envelope constructors

| Game / sub-surface | Target-commit seam | Verdict | Compatibility and rollback |
|---|---|---|---|
| River public envelopes | `games/river_ledger/src/effects.rs::public_effect` calls `EffectEnvelope::public` | `already-discharged-by-8C-pilot` (`MSC-8C-001`; Unit 8C C-01 pilot) | Verify effect order, payload, scope, and hashes; no R4 code change. |
| River private envelopes | `games/river_ledger/src/effects.rs::private_effect` calls `EffectEnvelope::private_to` | `already-discharged-by-8C-pilot` | Verify owner-only scope and no-leak; no R4 code change. All-in/side-pot effects use the same already-migrated constructor path. |
| Briar public envelopes | `games/briar_circuit/src/visibility.rs::effect_envelopes` constructs a public literal | `migrate` | Replace only the public literal with `EffectEnvelope::public`; payload/order/hash and visibility must be identical. Rollback restores the literal. |
| Briar private envelopes | `games/briar_circuit/src/visibility.rs::private_effect` constructs `PrivateToSeat` manually for pass selection/exchange | `migrate` | Replace only owner-private construction with `EffectEnvelope::private_to`; pass authorization/reveal stays local. Rollback restores the literal helper. |
| Vow public envelopes | `crates/wasm-api/src/games/vow.rs::vow_apply_command` maps public payloads through a literal envelope | `migrate` | Use `EffectEnvelope::public`; Vow effect ordering and rendered/logged bytes remain unchanged. Rollback restores the literal map. |
| Vow private envelopes | No private effect variant or private effect envelope exists; private information is in views/exports, not effects | `not-applicable` | Do not invent a private effect class. Trigger: first game-owned Vow effect that is intentionally seat-private. |

The helper never decides reveal timing, effect semantics, viewer authorization,
animation, logging, or redaction.

### 3.5 C-02 — canonical seat grammar and compatibility surfaces

C-02 applies only to seat identity. Card, trick, contract, pot, hand, and other
domain IDs are outside this helper.

| Game / sub-surface | Target-commit seam | Verdict | Required proof |
|---|---|---|---|
| River game parser/formatter/roster | `games/river_ledger/src/ids.rs::{RiverLedgerSeat::parse, as_str, seat_id_for_index}` already delegate to canonical `SeatId` helpers | `already-discharged-by-8C-pilot` (`MSC-8C-002`; [`UNI8CMECSCA-009`][ticket-009]) | Canonical round-trip and 3–6 bounds remain identical. |
| River import aliases | shared WASM import adapter already accepts bounded legacy aliases | `already-discharged-by-8C-pilot` | Historical inputs remain readable; no output-byte migration. |
| Briar game parser | `games/briar_circuit/src/ids.rs::BriarCircuitSeat::parse` manually matches `seat_0…seat_3` | `migrate` | Strict canonical parse delegates to `SeatId::parse_canonical`; same acceptance/rejection and enum mapping. |
| Briar formatter/roster | `as_str`, `seat_id_for_index`, `canonical_seat_ids` manually format strings | `migrate` | Use canonical formatting/index helpers without changing emitted strings or roster order. |
| Briar import aliases | `crates/wasm-api/src/games/briar.rs::parse_briar_seat` manually accepts `seat-0…seat-3` | `migrate` | Route aliases through `crates/wasm-api/src/seats.rs`; aliases remain import-only. Rollback restores the local adapter. |
| Vow game parser | `games/vow_tide/src/ids.rs::VowTideSeat::parse` manually matches `seat_0…seat_6` | `migrate` | Strict canonical parse, 0–6 bounds, and enum mapping remain identical. |
| Vow formatter/roster | `as_str`, `seat_id_for_index`, `canonical_seat_ids` manually format strings | `migrate` | Preserve canonical underscore outputs and declared 3–7 roster order. |
| Vow import aliases | `crates/wasm-api/src/games/vow.rs::parse_vow_seat` manually accepts hyphen aliases | `migrate` | Route through the bounded shared import adapter; no TypeScript normalization and no canonical-output flip. |
| Legacy trace/export spellings | Existing legacy documents selected in characterization | `exception` | Read compatibility remains through C-11 closeout. Trigger: separately named ADR-0009 seat-string authority migration. |
| Non-seat IDs | Briar/Vow card IDs, River card/pot/showdown identifiers, trick/contract labels | `not-applicable` | C-02 is seat grammar only. |

### 3.6 C-03 — structural seat count/range/ring without game policy

| Game / structural surface | Target-commit seam | Verdict | Boundary |
|---|---|---|---|
| River 3–6 admission | `setup.rs::river_seat_count` uses `SeatCountRange::inclusive(3, 6)` | `already-discharged-by-8C-pilot` (`MSC-8C-003`; [`UNI8CMECSCA-011`][ticket-011]) | Existing diagnostics remain game-local. |
| River checked index/ring | `SeatCount::checked_index` and `next_ring_seat` use shared structural helpers | `already-discharged-by-8C-pilot` | Button/blind/actor policy remains River-owned. |
| River starting-stack vector cardinality | Gate 15.1 setup requires stack-vector length to equal selected seats | `not-applicable` to further C-03 extraction | This is River setup/resource policy already expressed locally. Do not add a generic resource-vector helper. |
| Briar exact-four admission | `setup.rs::setup_match` compares `seats.len()` directly to four | `migrate` | Use `SeatCountRange::inclusive(4,4)` or an equivalent existing exact-count path, mapping the same `BC_UNSUPPORTED_SEAT_COUNT` diagnostic locally. |
| Briar fixed roster indexing | array conversion at setup | `migrate` only for checked structural indices needed by setup | No pass direction, partnership shape, dealer, or trick policy enters the helper. |
| Briar `next_clockwise` and pass targets | `ids.rs::{next_clockwise, pass_left_target, pass_right_target, pass_across_target}` | `exception` | These methods encode game-local dealer/pass topology. They remain local. Trigger: a separately reviewed behavior-free ring call that can preserve all pass/deal semantics. |
| Vow 3–7 admission | `ids.rs::supported_seat_count`; repeated checks in `setup.rs` and WASM creation | `migrate` | Centralize only the structural range through `SeatCountRange`; preserve `VT_INVALID_SEAT_COUNT` text locally. |
| Vow ring/index arithmetic | `VowTideSeat::next_clockwise(seat_count)` and `setup.rs::deal_order_after` | `migrate` | Use checked `SeatCount::next_ring_index` beneath game-owned dealer/deal/bid ordering. The helper does not select a dealer, first bidder, contract order, or leader. |
| Vow hand schedule and deal capacity | `max_hand_size_for_seats`, `hand_schedule_for_seats`, `deal_hand` capacity checks | `exception` | These are game rule/setup policy, not generic seat geometry. |
| Vow dealer/bid/contract diversity | state/action/rules logic | `not-applicable` to C-03 | C-03 validates structure only. |

### 3.7 C-04/C-05 — action-tree v1 and adjacent byte/hash surfaces

The selected C-05 migration is the **parallel** action-tree v1 byte/hash
surface only. Existing state/effect/view/replay/export/diagnostic authorities do
not flip in R4.

| Game | Existing action surface | C-04/C-05 verdict | Migration shape |
|---|---|---|---|
| River | `actions::legal_action_tree`; legacy `replay_support::action_tree_hash` hashes Debug text | `migrate` | Add explicit `ActionTreeEncodingVersion::V1` bytes/hash beside the legacy hash. Characterize fold/check/call/bet/raise plus full/short all-in metadata and side-pot-relevant states. Legacy hash remains authoritative/readable. |
| Briar | Browser tree is hand-built in `crates/wasm-api/src/games/briar.rs::briar_action_tree_json` from `legal_bot_actions`; game replay `action_hash` hashes projected previews | `migrate` | First add a game-owned typed `ActionTree` adapter over existing legal pass/play actions and prove exact path/order/label parity with the current browser tree. Then add parallel v1 bytes/hash. Do not move legality into WASM or replace the legacy browser JSON in the same diff. |
| Vow | `actions::legal_action_tree` has compound bid and play branches; replay hashes Debug text | `migrate` | Add parallel v1 bytes/hash covering bidding, dealer-hook exclusion, play/follow-suit, empty/wrong-actor trees, and 3–7 seats. Legacy hashes stay unchanged. |

Adjacent C-05 classifications apply to all three games:

| Surface class | Verdict | Owner, compatibility, rollback, trigger |
|---|---|---|
| Selected action-tree v1 bytes/hash | `migrate` | Game-owned adapter calls the existing `engine-core` v1 encoder. Rollback removes only the new parallel adapter/tests. |
| Legacy action-tree/debug hash | `exception` | Existing replay/evidence owner remains authoritative through C-11. Trigger: separately named hash-authority flip. |
| Internal state bytes/hash | `exception` | Game-owned stable summary/hash remains unchanged. Trigger: dedicated state migration. |
| Effect bytes/hash | `exception` | Game-owned effect ordering/rendering remains unchanged. Trigger: dedicated effect migration. |
| Public and seat-private view bytes/hash | `exception` | Projection/redaction contract remains game-owned under ADR 0004. Trigger: dedicated view migration with no-leak proof. |
| Replay-command bytes/hash | `exception` for encoding; C-08 metadata only | Existing trace bytes remain authority. Trigger: named replay migration. |
| Public/seat-private export bytes/hash | `exception` for encoding; C-08 metadata only | Existing export bytes remain authority. Trigger: named export migration. |
| Diagnostic bytes/hash | `exception` or `not-applicable` by existing game surface | Diagnostics remain game-local. Trigger: dedicated diagnostic migration. |

No task may use “the new hash is green” as permission to replace an existing
hash. The expected ADR-0009 class is `parallel-new-surface` with unchanged
legacy authority.

### 3.8 C-07 — pairwise no-leak geometry

The shared harness owns only deterministic enumeration and structured failure
reporting. Each game owns hidden-data construction, phase setup, viewer
authorization, reveal timing, snapshots, and containment. Existing focused
visibility tests remain; the generic matrix does not subsume them.

#### River Ledger — preserve pilot credit, close only residual scenario adapters

Pilot-discharge rows from `UNI8CMECSCA-021` remain
`already-discharged-by-8C-pilot`: counts 3–6; every source seat; observer plus
every seat viewer; view projection; setup effects; action tree; diagnostics;
viewer-scoped replay export; folded-showdown projection; bot input; and bot
explanation. R4 must not rebuild those rows.

Residual R4 rows:

| Source datum / state / surface | Observer | Owning seat | Other seat | Verdict |
|---|---:|---:|---:|---|
| Hole-card canary during short-call/full-call/open-bet/raise all-in lifecycle → view/action/diagnostic/effect | absent | present only in authorized private view/effect | absent | `migrate`: wrap selected existing stack lifecycle states in shared geometry |
| Non-winning or folded hole cards during multipot showdown → public and seat-private export | absent unless River's existing showdown policy authorizes reveal | same policy, never broader | absent unless existing policy authorizes reveal | `migrate`: selected multipot export matrix; River owns reveal policy |
| Hidden future board/deck tail during automatic all-in runout → view/effect/export/bot explanation | absent | absent | absent | `migrate` |
| Pot layers, contribution totals, all-in status, uncalled returns, allocation explanation | present as public accounting where current projection permits | present | present | `migrate` as absence-of-card proof, not as a secrecy rule for public accounting |
| Existing base matrix rows named by ticket 021 | as already proven | as already proven | as already proven | `already-discharged-by-8C-pilot` |

Required selected scenarios include at least: short blind all-in, short call,
short raise, cumulative reopen, all-all-in runout, uncalled return, three-way
main plus two side pots, different winners across pots, and per-pot remainder
order. These are **test states**, not behavior moved into the harness.

#### Briar Circuit — fixed-four private hand and pass geometry

For each source seat `S ∈ seat_0…seat_3`, run observer plus all four seat viewers.

| Source datum / phase / surface | Observer | Owner `S` | Other seat | Verdict |
|---|---:|---:|---:|---|
| Unplayed hand card → projected view | absent | present | absent | `migrate` |
| Selected pass card before confirm → pass view/action tree/preview | absent | present where current owner view allows | absent | `migrate` |
| Sent/received pass cards during atomic exchange → filtered effects/export | absent | present only to the affected seat under existing policy | absent | `migrate` |
| Other hands and pass provenance → diagnostic, public export, bot explanation/candidates | absent | absent unless current owner contract explicitly exposes own datum | absent | `migrate` |
| Played/captured cards after game-authorized public play | present | present | present | `migrate`; reveal timing remains Briar-owned |
| Moon/scoring totals without private card identities | public per existing projection | public | public | `migrate` as no-private-card proof; scoring stays local |

#### Vow Tide — 3–7 private hands, hidden stock, bidding and trick geometry

For each supported count 3–7 and each source seat `S`, run observer plus every
declared seat viewer.

| Source datum / phase / surface | Observer | Owner `S` | Other seat | Verdict |
|---|---:|---:|---:|---|
| Private hand card → view/action tree/diagnostic | absent | present only in own projected hand or authorized bot input | absent | `migrate` |
| Hidden stock card → all viewer-scoped surfaces | absent | absent | absent | `migrate` |
| Private hand card → public export | absent | absent | absent | `migrate` |
| Private hand card → seat-private export | absent for observer | present only for owner export | absent from other seat exports | `migrate`; driver receipt for export class itself remains pilot-discharged |
| Bid values, dealer, hand size, trump indicator, played cards and captured tricks | public as current rules/projector permit | public | public | `migrate` as no-extra-private-data proof |
| Raw hand/stock IDs → bot explanations/candidate rendering | absent except own legal input where allowed | no raw disallowed IDs in explanation | absent | `migrate` |

All canaries are generated in memory. No canary token may appear in a committed
trace, fixture, export, snapshot, log, or test identifier.

### 3.9 C-08 — evidence-profile driver matrix

Drivers validate profile metadata, then delegate behavior to the game or owning
validator. They do not parse commands, set up games, project views, authorize
exports, interpret side pots, decide tricks, or execute scoring from data.

| Profile | `river_ledger` | `briar_circuit` | `vow_tide` |
|---|---|---|---|
| `replay-command-v1` | `migrate`; internal-dev command traces, including selected Gate 15.1 traces; legacy bytes stay authority | `migrate`; pass/play traces | `migrate`; bid/play traces across representative 3- and 7-seat cases |
| `setup-evidence-v1` | `already-discharged-by-8C-pilot` ([`UNI8CMECSCA-024`][ticket-024], `river_ledger_3p_standard.fixture.json`) | `migrate`; standard fixed-four deterministic deal metadata | `migrate`; representative 3- and 7-seat schedule/deal metadata; hand schedule remains code |
| `public-export-v1` | `migrate`; observer export, including a multipot terminal path | `migrate`; `ViewerExportClass::Public` and existing public replay path | `already-discharged-by-8C-pilot` (`UNI8CMECSCA-025`) |
| `seat-private-export-v1` | `migrate`; every viewer for counts 3–6, including multipot terminal path | `migrate`; `ViewerExportClass::SeatPrivate` for all four seats | `already-discharged-by-8C-pilot` (`UNI8CMECSCA-025`, all declared viewers) |
| `domain-evidence-v1` | `migrate`; virtual metadata around selected game-owned all-in/side-pot allocation evidence, with `canonical_byte_authority = none` unless the existing validator owns bytes | `already-discharged-by-8C-pilot` ([`UNI8CMECSCA-026`][ticket-026], moon plus first-trick negative boundary) | `migrate`; hook/dealer-bid constraint and terminal-tie fixtures delegate legality/scoring to Vow Rust |

Minimum R4 domain selections:

- River: `three-way-main-two-side-pots`, `uncalled-return`, and
  `per-pot-remainder-button-order` evidence, with game-owned allocator and
  explanation assertions;
- Vow: `vow_tide_hook.fixture.json` and
  `vow_tide_terminal_tie.fixture.json`, with game-owned bid legality and
  competition-ranking assertions;
- Briar: verify, but do not reconstruct, the moon and first-trick pilot receipt.

A physical artifact may support more than one profile only when each profile
has an independent virtual metadata adapter and clearly distinct validator
owner/field set. No task rewrites an artifact merely to insert profile keys.

### 3.10 C-06, C-09, and C-10 checkpoint matrix

| Game | C-06 dev-only support | C-09 bounded index | C-10 non-promotion reaffirmation |
|---|---|---|---|
| `river_ledger` | `already-discharged-by-8C-pilot`; `game-test-support` is already a `[dev-dependencies]` edge from ticket 021 | `already-discharged-by-8C-pilot`; `setup.rs::shuffle_deck` calls `next_index_unbiased_v1`, with `UNI8CMECSCA-017` consumption parity | betting/fold/check/call/bet/raise legality, all-in/reopen, stacks/contributions, pot/side-pot construction and allocation, uncalled returns, evaluator/showdown, remainder order, reveal/projection, scoring and bots remain local |
| `briar_circuit` | `already-discharged-by-8C-pilot`; added by [`UNI8CMECSCA-026`][ticket-026] | `not-applicable` to R4 substitution; setup uses legacy modulo `next_index`; characterize and record separate ADR-0009 trigger | deal/pass direction/exchange, follow suit, first-trick exception, hearts-broken, trick winner/next leader, moon/scoring, projection and bots remain local |
| `vow_tide` | `already-discharged-by-8C-pilot`; added by `UNI8CMECSCA-025` | `not-applicable` to R4 substitution; setup uses legacy modulo `next_index`; characterize and record separate ADR-0009 trigger | dealer rotation, hand schedule, deal/stock/trump, bid/contract/hook/last-bidder, follow suit, trick winner/next leader, exact-bid scoring, projection and bots remain local |

`cargo tree --workspace -e normal --invert game-test-support` and the normal +
build variant are mandatory checkpoint evidence even though no C-06 dependency
is newly added.

The already-promoted `game-stdlib::trick_taking` helper remains an atlas-owned
behavioral primitive with its accepted narrow scope. R4 uses it as existing game
code does; R4 does not rename it as scaffolding, broaden it, or reopen its
promotion decision.

### 3.11 Out of scope

- Any game beyond the three named in §1.
- Gate 18 authoring, implementation, partnership abstraction, team grouping, or
  Spades rules.
- New shared-helper contracts or speculative “future-proof” options.
- Rebuilding any Unit 8C, R1, R2, or R3 helper, test crate, pilot, receipt, or
  migration.
- A generic poker, betting, all-in, pot, side-pot, evaluator, showdown, trick,
  bidding, contract, deal, pass, partnership, or scoring framework.
- An RNG algorithm change for Briar or Vow. Such work requires a separately
  versioned ADR-0009 packet and is not silently admitted by R4.
- Replacing legacy state/effect/view/replay/export/diagnostic hashes with action
  tree v1.
- General trace-schema or fixture-schema consolidation.
- Browser/catalog/renderer feature work, including `apps/web/README.md`.
- Bot strategy changes, benchmark-threshold changes, or game balance changes.
- Foundation-doc or ADR editing by default.

### 3.12 Not allowed

- Mechanic or game nouns added to `engine-core`.
- Behavioral promotion through the mechanical-scaffolding lane.
- Shared code deciding legality, setup semantics, dealer, bid, contract,
  follow-suit, trump, trick winner, next leader, pass target, reveal timing,
  authorization, projection/redaction, betting, all-in/reopen, pot/side-pot
  construction or allocation, remainder order, partnership, scoring, outcome,
  bot choice, or diagnostic prose.
- Silent change to bytes, hashes, trace meaning, export meaning, seat IDs,
  visibility, serialization order, RNG algorithm, RNG draw count, or shuffle/
  deal order.
- Blanket golden regeneration, snapshot sweeps, or accepting changed expected
  hashes merely because generated files changed.
- Changing a trace/fixture/export and its expected hash in the same diff without
  independent pre-change evidence.
- Test canaries in committed artifacts.
- `game-test-support` as a normal or build dependency.
- TypeScript seat repair, legality, projection, or normalization.
- YAML or a DSL; selectors, conditions, triggers, scripts, loops, formulas, or
  procedural rules in fixture/domain data.
- Deleting, weakening, ignoring, narrowing, or replacing a specific game test
  merely to make generic scaffolding pass.

## 4. Deliverables

### 4.1 Concrete artifact tree

The implementation wave may touch only the bounded artifacts below. A task may
remove a listed path from its migration set after characterization resolves the
surface to `not-applicable` or `exception`; it may not add a fourth game or an
unlisted behavior owner without `/reassess-spec` review.

| Area | Expected artifacts | Required result |
|---|---|---|
| Wave characterization | `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` | Working-tree baseline; pilot-discharge map; aggregate and sub-surface verdicts; pre-change bytes/hashes/seat spellings/RNG vectors; visibility products; accepted exception/N-A ledger; final diff inventory; command results. |
| River action-tree v1 | `games/river_ledger/src/replay_support.rs`; focused additions in `games/river_ledger/tests/{rules,serialization}.rs` and/or a narrowly added action-tree test module | Parallel `ActionTreeEncodingVersion::V1` bytes/hash over the already game-owned `actions::legal_action_tree`; legacy `action_tree_hash` authority unchanged. |
| River residual no-leak | `games/river_ledger/tests/visibility.rs`; selected existing files under `games/river_ledger/tests/golden_traces/` as **read-only characterization inputs** | Shared no-leak geometry over selected stack/all-in/runout/multipot states while preserving ticket-021 pilot credit and all game-owned reveal/accounting rules. |
| River residual profiles | `games/river_ledger/tests/{replay,visibility,serialization}.rs` or narrowly scoped new test modules; virtual profile adapters around selected existing traces/fixtures | `replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, and selected `domain-evidence-v1` coverage without rewriting existing artifact bytes merely to add metadata. |
| Briar C-01 | `games/briar_circuit/src/visibility.rs`; focused visibility/effect tests | Existing public/private envelope literals delegate to `EffectEnvelope::{public,private_to}` with byte/order/scope equality. |
| Briar C-02/C-03 | `games/briar_circuit/src/{ids,setup}.rs`; `crates/wasm-api/src/{seats.rs,games/briar.rs}`; focused crate/WASM tests | Canonical Rust seat grammar, import-only aliases, and exact-four structural validation use existing helpers; pass/dealer topology remains local. |
| Briar C-04/C-05 | `games/briar_circuit/src/{actions,replay_support}.rs`; `crates/wasm-api/src/games/briar.rs`; focused action/replay/serialization tests | A game-owned typed action-tree adapter is parity-checked against the current browser choices, then receives a parallel v1 byte/hash surface. Current browser JSON and replay hashes remain authoritative. |
| Briar C-07/C-08 | `games/briar_circuit/tests/{visibility,replay,serialization}.rs`; virtual adapters around existing golden traces and `briar_circuit_standard.fixture.json` | Fixed-four source-seat × viewer matrices plus residual replay/setup/public/seat-private profile coverage. The moon/first-trick domain pilot is verified, not rebuilt. |
| Vow C-01 | `crates/wasm-api/src/games/vow.rs`; focused WASM/effect tests | `vow_apply_command` uses `EffectEnvelope::public`; effect payload/order/log bytes remain unchanged. No private effect class is invented. |
| Vow C-02/C-03 | `games/vow_tide/src/{ids,setup}.rs`; `crates/wasm-api/src/{seats.rs,games/vow.rs}`; focused game/WASM tests | Canonical seat grammar, import aliases, 3–7 structural range, and checked ring index use existing helpers. Hand schedule, dealer, deal, bid, contract, and leader policy remain local. |
| Vow C-04/C-05 | `games/vow_tide/src/replay_support.rs`; focused action/replay/serialization tests | Parallel v1 bytes/hash over `actions::legal_action_tree`; legacy Debug-derived hashes and all view/export bytes remain unchanged. |
| Vow C-07/C-08 | `games/vow_tide/tests/{visibility,replay,serialization}.rs`; virtual adapters around existing 3p/7p traces and selected fixtures | Full 3–7 source-seat × viewer matrices plus residual replay/setup/domain profiles. Existing public/seat-private export driver receipts remain pilot credit. |
| C-06/C-09/C-10 checkpoints | the characterization report; focused tests only when needed to pin current behavior | Dev-only dependency direction proven; River unbiased sampler receipt verified; Briar/Vow modulo consumption pinned and left unchanged; all non-promotion bundles reaffirmed. |
| Governance receipts | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | R4 receipt rows under existing `MSC-8C-001…010` entries plus a Unit 8C-R4 closeout block. No new helper entry. |
| Tracker closeout | `specs/README.md` | Flip only `8C-R4` to `Done`, link final evidence, and state that the final C-11 admission interlock for Gate 18 is cleared. Do not alter Gate 18's scope or status. |
| Intermediate/final spec lifecycle | this artifact, later reassessed into `specs/8c-r4-n-seat-private-trick-scaffolding.md`, and archived only at closeout | `/reassess-spec` validates paths/symbols/verdicts; `/spec-to-tickets` creates bounded packets; implementation does not skip those steps. |

Existing helpers in `crates/engine-core`, `crates/game-stdlib`, and
`crates/game-test-support` are dependencies, not new deliverables. A contract
change in one of those crates is a stop condition for this spec unless the
change is a test-only regression assertion that does not broaden the helper.

### 4.2 Characterization-report minimum schema

The new report must contain, at minimum:

1. repository, the working-tree commit the characterization was grounded
   against, and the evidence basis;
2. the locked determination and exact three-game scope;
3. a pilot-receipt table naming the exact `MSC-8C-*` entry and
   `UNI8CMECSCA-*` ticket for every pilot-credit cell;
4. the §3.3 aggregate matrix and every §3.4–§3.10 sub-surface disposition as
   actually resolved immediately before implementation;
5. per selected surface: owner, file, symbol, current serializer/hasher,
   visibility class, artifact/profile IDs, legacy authority, and rollback unit;
6. exact pre-change byte/hash values for every touched hash-bearing surface;
7. exact accepted/rejected canonical and legacy seat strings for Briar and Vow;
8. River 3–6, Vow 3–7, and Briar fixed-four seat/viewer enumeration counts and
   test names;
9. River `next_index_unbiased_v1` parity receipt and Briar/Vow legacy
   `next_index` draw/output vectors, without changing those algorithms;
10. a golden/fixture/export diff inventory classifying each changed path as
    `unchanged`, `parallel-new`, or `intentional-migration` under ADR 0009;
11. every accepted N/A/exception with owner, compatibility statement, rollback,
    and next review trigger; and
12. final commands, register receipt links, and tracker closeout evidence.

### 4.3 Explicit non-deliverables

R4 delivers no new game, rules text, card art, web component, catalog entry,
bot policy, benchmark target, static-data behavior, shared trick policy, shared
betting/pot policy, shared partnership model, foundation amendment, or accepted
ADR. Existing golden traces and fixtures are evidence inputs, not a bulk
rewrite target.

## 5. Work breakdown

### 5.1 Packet contract and dependency law

Every row below is a candidate [`AGENT-TASK.md`][agent-task] packet with task
profile `scaffold-refactor`. `/spec-to-tickets` may renumber IDs or split a row
further, but it may not merge independent byte-, visibility-, seat-, or
RNG-bearing surfaces into one diff.

Every packet must fill the Scaffold-Refactor Profile fields with:

- accepted authority: ADR 0008, ADR 0009 when bytes/hashes/profiles are touched,
  ADR 0004 when viewer-scoped data is touched, and the relevant existing
  `MSC-8C-*` entry;
- exact duplicate sites and symbols;
- affected legacy hash, trace, fixture, export, and visibility authorities;
- characterization tests that run **before** the implementation edit;
- the reference helper/pilot and semantic-equality comparison;
- one selected migration surface;
- a rollback that removes or restores only that selected surface; and
- the task-specific forbidden behavioral changes.

The failing-test protocol in [`docs/AGENT-DISCIPLINE.md`][agent-discipline]
applies: first decide whether the failing test remains valid, then whether the
fault is in the system under test or the test, then fix the fault without
weakening valid coverage.

### 5.2 Wave A — admission, provenance, and characterization

| Candidate task | Exact files/symbols | Bounded result and affected authorities | Evidence and rollback |
|---|---|---|---|
| `8CR4NSEAT-001` — lock determination and report shell | `specs/README.md`; `docs/MECHANIC-ATLAS.md` §10A; parent/R1/R2/R3 specs; `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` | Create the report shell with the working-tree evidence basis, §1 determination, three-game boundary, Gate-18 interlock statement, and grounded corrections. No source edit. | Evidence: cited spec/index/register rows. Rollback: delete only the uncommitted report shell. |
| `8CR4NSEAT-002` — pilot-credit versus residual inventory | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`; `archive/tickets/UNI8CMECSCA-{009,011,017,021,024,025,026}.md`; all files named in §3 | Pin every `already-discharged-by-8C-pilot` cell to its exact receipt and every residual to an actual target-commit seam. Correct Vow C-02, River chronology, Vow private C-01, and Briar/Vow C-09 in the report. | Evidence: report matrix with no unresolved cell. Rollback: report-only. Stop if any required path/receipt cannot be acquired. |
| `8CR4NSEAT-003` — baseline bytes, IDs, visibility, profiles, RNG | the three games' `src/{actions,effects,ids,replay_support,setup,state,visibility}.rs`; `tests/{replay,serialization,visibility}.rs`; selected traces/fixtures; WASM Briar/Vow seams | Record current hashes/bytes, accepted/rejected seat inputs, viewer/export classes, profile ownership, and RNG draw/output vectors. Do not modify goldens or algorithms. | Evidence: reproducible focused tests/commands in report. Rollback: characterization tests/report only. Any unstable baseline blocks the dependent migration. |

No implementation wave starts until tasks 001–003 are reviewed together and
all §3 cells have an owner and verdict.

### 5.3 Wave B — C-01 envelope migrations

| Candidate task | Exact files/symbols | One selected surface | Affected authority, proof, and rollback |
|---|---|---|---|
| `8CR4NSEAT-101` — Briar public envelope | `games/briar_circuit/src/visibility.rs::effect_envelopes`; focused tests in `games/briar_circuit/tests/visibility.rs` | Replace only the public `EffectEnvelope` literal arm with `EffectEnvelope::public`. | Pin payload sequence, Debug/stable hash consumers, `VisibilityScope::Public`, filtered results, and WASM logged JSON. Rollback restores the public literal only. |
| `8CR4NSEAT-102` — Briar owner-private envelope | `games/briar_circuit/src/visibility.rs::private_effect`; pass-selection/exchange visibility tests | Replace only the `PrivateToSeat` literal with `EffectEnvelope::private_to`. | Pin owner seat, payload/card order, non-owner absence, observer absence, effect hash, and pass reveal timing. Rollback restores `private_effect`; pass behavior is untouched. |
| `8CR4NSEAT-103` — Vow public envelope | `crates/wasm-api/src/games/vow.rs::vow_apply_command`; `crates/wasm-api` focused tests | Replace only the literal public envelope map with `EffectEnvelope::public`. | Pin effect count/order/payload, logged JSON, replay-step output, and visibility. Rollback restores the literal map. No private Vow effect task exists. |

River C-01 receives a verification receipt in task 002/901, not an
implementation packet.

### 5.4 Wave C — C-02 canonical seats and import-only aliases

| Candidate task | Exact files/symbols | One selected surface | Affected authority, proof, and rollback |
|---|---|---|---|
| `8CR4NSEAT-201` — Briar game parser | `games/briar_circuit/src/ids.rs::BriarCircuitSeat::parse` | Delegate strict canonical input to `SeatId::parse_canonical` plus bounded enum conversion. | Accepted set remains `seat_0…seat_3`; malformed/out-of-range strings remain rejected. Rollback restores the match. |
| `8CR4NSEAT-202` — Briar formatter/roster | `BriarCircuitSeat::as_str`; `seat_id_for_index`; `canonical_seat_ids` | Delegate canonical output/index construction to existing `SeatId` helpers without changing public strings/order. | Pin source/API/WASM/trace outputs. Rollback restores local formatting. This task does not touch pass targets. |
| `8CR4NSEAT-203` — Briar WASM alias adapter | `crates/wasm-api/src/seats.rs::{parse_seat_import,parse_seat_enum}` and `crates/wasm-api/src/games/briar.rs::parse_briar_seat` | Add a bounded Briar adapter and replace the local hyphen-alias match. | Canonical and `seat-0…seat-3` imports remain accepted; outputs remain underscore canonical. Rollback removes only the Briar adapter and restores local parsing. |
| `8CR4NSEAT-211` — Vow game parser | `games/vow_tide/src/ids.rs::VowTideSeat::parse` | Delegate strict canonical input to `SeatId::parse_canonical` plus bounded enum conversion. | Accepted set remains `seat_0…seat_6`; malformed/out-of-range strings remain rejected. Rollback restores the match. |
| `8CR4NSEAT-212` — Vow formatter/roster | `VowTideSeat::as_str`; `seat_id_for_index`; `canonical_seat_ids` | Delegate canonical output/index construction without changing strings or selected roster length/order. | Pin 3–7 rosters, traces, exports, and WASM output. Rollback restores local formatting. |
| `8CR4NSEAT-213` — Vow WASM alias adapter | `crates/wasm-api/src/seats.rs::{parse_seat_import,parse_seat_enum}` and `crates/wasm-api/src/games/vow.rs::parse_vow_seat` | Add a bounded seven-seat Vow adapter and remove the local `seat-{n}` search. | Canonical plus bounded hyphen aliases remain import-compatible; no new symbolic alias is inferred. Rollback removes only the Vow adapter and restores local parsing. |

The game parser tasks precede their corresponding WASM adapter tasks. A legacy
trace/export spelling remains an accepted exception unless a distinct ADR-0009
packet admits an output migration.

### 5.5 Wave D — C-03 structural seat validation and ring index

| Candidate task | Exact files/symbols | One selected surface | Boundary proof and rollback |
|---|---|---|---|
| `8CR4NSEAT-301` — Briar exact-four count | `games/briar_circuit/src/setup.rs::{setup_match,invalid_seat_count_diagnostic}`; `crates/wasm-api/src/games/briar.rs::create_briar_circuit_match` | Use `SeatCountRange::inclusive(4,4).validate` beneath the existing game diagnostic. | Pin valid setup/deal and invalid above/below diagnostic bytes. Rollback restores the direct comparison. No pass/dealer rule moves. |
| `8CR4NSEAT-302` — Vow 3–7 range | `games/vow_tide/src/ids.rs::supported_seat_count`; `setup.rs::{setup_match,invalid_seat_count_diagnostic}`; `crates/wasm-api/src/games/vow.rs::create_vow_tide_match` | Centralize only inclusive range validation through `SeatCountRange`. | Pin valid 3–7 and invalid 0–2/8+ diagnostics. Rollback restores local range checks. Hand schedule is untouched. |
| `8CR4NSEAT-303` — Vow checked ring index | `games/vow_tide/src/ids.rs::VowTideSeat::next_clockwise`; `setup.rs::deal_order_after`; focused setup/rule tests | Use `SeatCount::next_ring_index` for one clockwise structural step under existing dealer/deal/bid policy. | Pin all counts 3–7, every current index, wrap order, dealer/first-bidder traces, and unchanged RNG consumption. Rollback restores local modulo. |

River C-03 and Briar's pass-target methods receive explicit receipts only. Vow
hand schedule and deal capacity remain game-owned exceptions.

### 5.6 Wave E — C-04/C-05 parallel action-tree v1

| Candidate task | Exact files/symbols | One selected surface | Hash/byte authority, proof, and rollback |
|---|---|---|---|
| `8CR4NSEAT-401` — River v1 adapter | `games/river_ledger/src/actions.rs::legal_action_tree`; `games/river_ledger/src/replay_support.rs::{action_tree_hash}` plus a new clearly named parallel-v1 function | Add the River adapter that invokes `ActionTree::{stable_bytes,stable_hash}(ActionTreeEncodingVersion::V1)`. | Characterize fold/check/call/bet/raise ordering and metadata first. Legacy Debug hash is unchanged. Rollback deletes only the new function/tests. |
| `8CR4NSEAT-402` — River all-in/side-pot action-tree vectors | focused River rules/serialization tests; existing traces `short-*-all-in`, `cumulative-reopen`, `all-all-in-runout`, and `three-way-main-two-side-pots` as read-only inputs | Add parallel-v1 expected vectors for the selected richer states. | No betting/all-in/pot logic changes and no golden rewrite. Rollback removes only these v1 vectors. |
| `8CR4NSEAT-411` — Briar typed-tree parity adapter | `games/briar_circuit/src/actions.rs`; `games/briar_circuit/src/bots.rs::legal_bot_actions`; `crates/wasm-api/src/games/briar.rs::{briar_action_tree_json,briar_action_choice_json,briar_action_next_json}` | Add a game-owned `legal_action_tree` adapter over existing legal pass/play actions and prove exact path/order/label parity with current browser choices. | Current browser JSON remains the rendered authority. Rollback removes the typed adapter/parity test only. Legality remains in Briar Rust. |
| `8CR4NSEAT-412` — Briar v1 bytes/hash | `games/briar_circuit/src/replay_support.rs::{action_hash,replay_hash_snapshot}` plus the adapter from 411 | Add a parallel action-tree v1 bytes/hash surface for observer and seat viewers. | Existing preview Debug hashes and browser JSON remain unchanged. Pin pass select/unselect/confirm and legal play paths. Rollback removes only the new v1 fields/functions/tests; do not change the existing snapshot schema in place if compatibility would break. |
| `8CR4NSEAT-421` — Vow v1 bytes/hash | `games/vow_tide/src/actions.rs::legal_action_tree`; `games/vow_tide/src/replay_support.rs::snapshot` and a new parallel-v1 function | Add parallel v1 bytes/hash for bid and play trees across representative 3–7 states. | Pin dealer-hook exclusion, bid ordering, card ordering, empty/wrong-actor trees, and legacy Debug hash values. Rollback removes only parallel code/tests. |
| `8CR4NSEAT-431` — adjacent C-05 exception receipts | characterization report; `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Record state/effect/view/replay/export/diagnostic encodings as explicit exceptions or N/A, including owner, legacy authority, and review trigger. | Evidence-only; no source byte changes. Rollback is the receipt edit. |

If the existing v1 API cannot express a game tree without changing its
contract, the game task stops. R4 does not broaden `engine-core`; it records an
exception or raises a §8.4 ADR trigger.

### 5.7 Wave F — C-06 dependency checkpoint

| Candidate task | Exact files/symbols | Bounded result | Evidence and rollback |
|---|---|---|---|
| `8CR4NSEAT-501` — dev-only dependency proof | `games/{river_ledger,briar_circuit,vow_tide}/Cargo.toml`; workspace dependency graph; `scripts/boundary-check.sh` | Verify all three already declare `game-test-support` only under `[dev-dependencies]`, no production/build reverse edge exists, and no R4 source imports it outside tests/benches. | `cargo tree` normal and normal+build inverse queries plus boundary check. No dependency edit is expected. Any improper edge is removed in a separately reviewable correction; rollback restores only that manifest edge. |

### 5.8 Wave G — C-07 private-information matrices

| Candidate task | Exact files/symbols | One selected matrix surface | Visibility authority, evidence, and rollback |
|---|---|---|---|
| `8CR4NSEAT-601` — River selected stack lifecycle matrix | `games/river_ledger/tests/visibility.rs::{visibility_stack_pot_pairwise_matrix_hides_private_data_across_lifecycle_states,no_leak_snapshot,no_leak_expectation}` and existing setup helpers | Wrap selected short-blind/short-call/open-bet/raise/cumulative-reopen states in shared pairwise geometry. | Existing River projectors/effects/diagnostics are authority. Pin source seat × observer/all seats for counts 3–6. Rollback removes only new probes/adapters. |
| `8CR4NSEAT-602` — River runout/multipot export matrix | River `tests/visibility.rs`, `tests/replay.rs` or narrowly added test module; `replay_support::{export_public_replay,import_public_export}`; selected multipot traces | Add hidden-future-card and folded/non-winning-hole-card absence checks over automatic runout and multipot public/seat-private exports. | River reveal/allocation policy is unchanged; public pot accounting is not treated as secret. Rollback removes only selected matrix rows. |
| `8CR4NSEAT-611` — Briar pass-phase matrix | `games/briar_circuit/tests/visibility.rs`; `visibility::{project_pass_view,project_action_previews,effect_envelopes,filter_effects_for_viewer}` | Enumerate all four source seats × observer/all four viewers for pass selection, commitment, and exchange surfaces. | Pin owner-only cards and non-owner absence. Pass target/reveal/atomic exchange remains local. Rollback removes matrix adapters only. |
| `8CR4NSEAT-612` — Briar play/export/bot matrix | Briar visibility/replay/bot tests; `visibility::project_view`; `replay_support::export_viewer_timeline`; bot explanation/candidate surfaces | Cover private hands before play and public cards after authorized play across view, export, diagnostics, bot input/explanation, and action choices. | Existing focused rule/no-leak tests remain. Rollback removes matrix adapters only. |
| `8CR4NSEAT-621` — Vow hand/stock matrix | `games/vow_tide/tests/visibility.rs::{exhaustive_seat_pair_no_leak_for_three_to_seven_players,install_canary_hands_and_stock}` | Convert/extend to shared geometry for every count 3–7, source seat, observer, and declared seat over view/action/diagnostic/effect. | Pin own-hand-only and stock-never-visible rules. Rollback removes shared adapter while retaining existing exhaustive assertions. |
| `8CR4NSEAT-622` — Vow bid/trick/export/bot matrix | Vow visibility/replay/bot tests; `replay_support::export_for_viewer`; action-tree and explanation seams | Extend matrix across bidding/trick phases, public and seat-private export, bot input/explanation, and candidate rendering. | Public bids/trump/plays remain public; private hand/stock IDs remain absent. Rollback removes only new matrix rows. |

No matrix task may delete a game-specific assertion after the shared harness is
added. The task report records the Cartesian-product size actually executed.

### 5.9 Wave H — C-08 residual evidence profiles

Each row adds a virtual profile adapter around an existing artifact or
game-owned validator. It does not insert procedural metadata into the artifact.

| Candidate task | Exact files/artifacts | One selected profile | Authority, evidence, and rollback |
|---|---|---|---|
| `8CR4NSEAT-701` — River replay-command | River `tests/replay.rs`; representative base and all-in golden traces; `ReplayCommandV1Driver` | Validate selected internal-dev command traces under `replay-command-v1`. | Trace bytes/schema remain unchanged. Rollback removes profile metadata/driver test. |
| `8CR4NSEAT-702` — River public export | River replay tests; `replay_support::export_public_replay`; `public-replay-export-import.trace.json` and selected multipot terminal export | Add `public-export-v1` observer round-trip and no-leak evidence. | Existing export JSON/hash authority remains. Rollback removes driver adapter. |
| `8CR4NSEAT-703` — River seat-private export | River replay/visibility tests; every viewer at 3–6; selected multipot state | Add `seat-private-export-v1` driver coverage over all declared viewers. | Private-to-owner and non-owner absence are pinned; no export encoding flip. Rollback removes driver adapter. |
| `8CR4NSEAT-704` — River side-pot domain evidence | River fixture/replay/rule tests; selected `three-way-main-two-side-pots`, `uncalled-return`, and remainder-order evidence; `DomainEvidenceV1Driver` | Add virtual domain metadata delegating allocation/explanation validation to River code. | `canonical_byte_authority = none` unless an existing validator owns exact bytes. Rollback removes metadata adapter only. |
| `8CR4NSEAT-711` — Briar replay-command | Briar replay tests and representative pass/play golden traces; `ReplayCommandV1Driver` | Validate selected internal command traces. | No trace rewrite. Rollback removes profile adapter. |
| `8CR4NSEAT-712` — Briar setup evidence | `games/briar_circuit/data/fixtures/briar_circuit_standard.fixture.json`; setup/serialization tests; `SetupEvidenceV1Driver` | Validate fixed-four deterministic setup metadata through Briar setup owner. | Deal/shuffle policy remains code. Rollback removes adapter. |
| `8CR4NSEAT-713` — Briar public export | `replay_support::{export_viewer_timeline,import_viewer_timeline}` with `ViewerExportClass::Public`; replay tests | Add `public-export-v1` round-trip/no-leak profile. | Existing export bytes/class remain authority. Rollback removes adapter. |
| `8CR4NSEAT-714` — Briar seat-private export | same functions with `ViewerExportClass::SeatPrivate`; all four viewers | Add `seat-private-export-v1` profile over all seats. | Owner/non-owner hand and pass data boundaries pinned. Rollback removes adapter. |
| `8CR4NSEAT-721` — Vow replay-command | Vow replay tests; representative bid/play traces at 3 and 7 seats; `ReplayCommandV1Driver` | Validate internal command traces. | Existing trace bytes/hash remain. Rollback removes adapter. |
| `8CR4NSEAT-722` — Vow setup evidence | `vow_tide_3p_standard.fixture.json`, `vow_tide_7p_standard.fixture.json`; setup tests; `SetupEvidenceV1Driver` | Validate seat count, schedule label, and deterministic deal evidence while delegating schedule/deal rules to Vow. | No static-data formula. Rollback removes adapter. |
| `8CR4NSEAT-723` — Vow domain evidence | `vow_tide_hook.fixture.json`, `vow_tide_terminal_tie.fixture.json`; rule/scoring tests; `DomainEvidenceV1Driver` | Validate hook negative boundary and terminal competition ranking through Vow Rust. | Bidding/scoring stays local; fixture bytes remain unchanged unless a separate ADR-0009 packet says otherwise. Rollback removes adapter. |

River setup, Vow public/seat-private export, and Briar domain-evidence profiles
receive pilot-verification receipts only.

### 5.10 Wave I — C-09 sampler-divergence disposition

| Candidate task | Exact files/symbols | Bounded result | RNG authority, evidence, and rollback |
|---|---|---|---|
| `8CR4NSEAT-801` — Briar legacy sampler receipt | `games/briar_circuit/src/setup.rs::shuffle_deck`; `DeterministicRng::next_index`; setup/serialization tests | Pin representative raw RNG draws, chosen indices, draw count, deck permutation, and deal bytes; record C-09 `not-applicable` to in-wave substitution. | No code/algorithm edit. Next review trigger: separately accepted ADR-0009 algorithm version. Rollback is report/register-only. |
| `8CR4NSEAT-802` — Vow legacy sampler receipt | `games/vow_tide/src/setup.rs::shuffle_deck`; `DeterministicRng::next_index`; 3- and 7-seat setup tests | Pin representative draw/index/deck/deal vectors and record the same N/A. | No code/algorithm edit. Rollback is report/register-only. |

A surprising byte-identical result from a small sample is not sufficient to
convert either task into a migration. The current algorithms are structurally
different; an algorithm change requires its own admitted compatibility and
versioning plan.

### 5.11 Wave J — consolidation, register, verification, and status flip

| Candidate task | Exact files/symbols | Bounded result | Evidence and rollback |
|---|---|---|---|
| `8CR4NSEAT-901` — consolidate verdicts and C-10 receipts | characterization report; `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` under `MSC-8C-001…010` | Add one R4 receipt for every aggregate/sub-surface disposition, including pilot credit, N/A, exceptions, migration commits/tests, and all three C-10 local bundles. | Review against §3 and task outputs; no unowned row. Rollback reverts only R4 receipt additions. |
| `8CR4NSEAT-902` — full command and artifact-diff audit | entire workspace; exact command set in §7.1; `git diff --name-only`/byte-digest evidence generated by maintainers at execution time | Run focused then workspace verification; classify every changed golden/fixture/export/trace/hash path; prove dependency/boundary/doc guards. | Any unauthorized artifact change blocks closeout and is reverted at its owning task. No blanket regeneration. |
| `8CR4NSEAT-903` — R4 closeout and tracker flip | report; register Unit 8C-R4 closeout block; `specs/README.md` R4 row | Record final evidence, flip only R4 to `Done`, and state that the final C-11 Gate-18 admission interlock is cleared. | Requires every §6 row. Rollback restores the tracker row and closeout block; Gate 18 remains untouched. |

### 5.12 Required internal order

The dependency order is:

```text
001 → 002 → 003
          ↓
101–103, 201–213, 301–303
          ↓
401 → 402
411 → 412
421
          ↓
501
          ↓
601–622
          ↓
701–723
          ↓
801–802
          ↓
431 → 901 → 902 → 903
```

Independent game/surface packets within one row may run in parallel only after
003 has pinned their shared baseline. A later task may consume an earlier
parallel surface; it may not combine its rollback with that earlier surface.

## 6. Exit criteria

R4 is complete only when every row below is satisfied and linked from the
characterization report. `Accepted` means maintainer-reviewed evidence, not
merely an agent assertion.

| ID | Exit criterion | Seed/parent mapping |
|---|---|---|
| `EC-R4-01` | The report confirms `8C-R3` is `Done`, `8C-R4` was the lowest non-`Done` row, §10A promotion debt was empty, and the exact three-game unit was not reopened. | Tracker workflow; parent 8C-030. |
| `EC-R4-02` | Baseline provenance is recorded: the working-tree commit SHA the characterization was grounded against, with every cited path and symbol re-resolved at that checkout. | Working-tree baseline. |
| `EC-R4-03` | Every C-01…C-09 aggregate cell and every listed sub-surface has exactly one reviewed verdict: migrate, pilot-discharged, N/A, or exception. No “later cleanup” bucket remains. | R4 seed admission/exit; parent EC-28. |
| `EC-R4-04` | Pilot credit is pinned without reconstruction: River C-01/C-02/C-03/C-06/C-07-base/C-08-setup/C-09; Vow C-06/C-08-public/C-08-seat-private; Briar C-06/C-08-domain. Grounded corrections are retained. | Parent pilot-discharge rule; register/tickets. |
| `EC-R4-05` | All three Cargo manifests retain `game-test-support` only as a dev dependency; normal/build inverse dependency checks are empty. | C-06. |
| `EC-R4-06` | Briar public and private envelope migrations are semantically and byte/order/scope identical; River envelope receipt remains intact; Vow public envelope migrates and private envelope is explicitly N/A. | C-01. |
| `EC-R4-07` | Briar and Vow game-level canonical seat parse/format/roster paths use existing canonical helpers with identical output; non-seat IDs are explicitly excluded. | C-02. |
| `EC-R4-08` | Briar/Vow legacy input aliases are handled only at the WASM import boundary; canonical output remains Rust-authored and unchanged; malformed/out-of-range inputs remain rejected. | C-02; WASM boundary. |
| `EC-R4-09` | Briar exact-four and Vow 3–7 structural validation/ring steps use existing C-03 helpers where characterized; River receipt remains intact; dealer/pass/bid/contract/hand-schedule policy remains local. | C-03. |
| `EC-R4-10` | River, Briar, and Vow expose reviewed parallel action-tree v1 bytes/hash for the selected surfaces, including River all-in metadata, Briar pass/play parity, and Vow bid/play diversity. | C-04/C-05. |
| `EC-R4-11` | Existing action-tree/debug, state, effect, view, replay, export, and diagnostic authorities remain byte-identical unless a separately named accepted ADR-0009 packet exists; every adjacent surface has an explicit exception/N-A receipt. | ADR 0009; C-05. |
| `EC-R4-12` | River's ticket-021 base no-leak matrix is preserved, and selected stack/all-in/runout/multipot residual states are covered without changing reveal or allocation policy. | C-07; River residual. |
| `EC-R4-13` | Briar executes fixed-four source-seat × observer/all-seat checks over hands, pass selection/exchange, play, exports, diagnostics, bot input, and explanations. | C-07. |
| `EC-R4-14` | Vow executes source-seat × observer/all-declared-seat checks for every count 3–7 over hands, stock, bid/trick phases, exports, diagnostics, bot input, and explanations. | C-07; widest matrix. |
| `EC-R4-15` | No hole card, private hand, hidden stock/deck tail, private pass provenance, or disallowed raw ID appears on an unauthorized surface; no test canary appears in a committed artifact. | FOUNDATIONS hidden-info invariant; ADR 0004. |
| `EC-R4-16` | C-08 residual profiles close exactly as §3.9: River replay/public/seat-private/domain; Briar replay/setup/public/seat-private; Vow replay/setup/domain. Named pilot profiles remain credit rather than rework. | C-08; parent EC-28. |
| `EC-R4-17` | Profile drivers remain thin: metadata validation and dispatch only. All setup, replay, export, side-pot, pass, bid, trick, and scoring behavior remains in the owning game/validator. | Evidence Fixture Contract; boundary. |
| `EC-R4-18` | River's shared unbiased sampler pilot receipt and consumption parity remain intact. | C-09 River. |
| `EC-R4-19` | Briar and Vow legacy modulo draw/index/deck/deal vectors are pinned and unchanged; their C-09 cells close as reviewed N/A with a separately versioned ADR-0009 next-review trigger. | C-09 divergence. |
| `EC-R4-20` | C-10 reaffirms the complete local-only behavior bundles for all three games, and the already-promoted `game-stdlib::trick_taking` helper is neither broadened nor reclassified as scaffolding. | Register Non-Promotion List; atlas §10/§10B. |
| `EC-R4-21` | No shared helper decides betting/all-in/pot/side-pot, deal/shuffle, pass, dealer, bid/contract, trick lifecycle/winner/leader, partnership, projection/redaction, scoring, outcome, bot choice, or diagnostic prose. | FOUNDATIONS/boundary/ADR 0008. |
| `EC-R4-22` | Every touched byte/hash/seat/visibility/profile surface has pre-change evidence, after evidence, compatibility statement, and one-surface rollback; no blanket golden/snapshot regeneration occurred. | ADR 0009; R1/R2/R3 precedent. |
| `EC-R4-23` | All focused and workspace commands in §7.1 pass, or a failure is resolved through the required valid-test/SUT-or-test/fix protocol with evidence. | AGENT-DISCIPLINE. |
| `EC-R4-24` | The final diff inventory reports every golden trace, fixture, replay export, and expected hash as unchanged, parallel-new, or separately authorized intentional migration; unauthorized changes are zero. | Golden/fixture/diff policy. |
| `EC-R4-25` | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` carries complete R4 receipts under `MSC-8C-001…010` and a Unit 8C-R4 closeout block; no new helper contract is claimed. | Register-first governance. |
| `EC-R4-26` | No foundation doc or accepted ADR is edited by default. Any genuine doctrine gap is a flagged blocking §13 ADR trigger, not a silent local workaround. | FOUNDATIONS §§12–13. |
| `EC-R4-27` | `apps/web/README.md` is explicitly recorded as not applicable; catalog checks pass as regression guards and no web/catalog feature is changed. | Spec-format §10; no new game/web surface. |
| `EC-R4-28` | The characterization report contains final commands, matrices, receipts, diff inventory, and rollback map; the reassessed spec and generated tickets are linked. | Deliverable completeness. |
| `EC-R4-29` | The `8C-R4` tracker row alone flips to `Done` after all prior criteria pass. | Tracker workflow. |
| `EC-R4-30` | The closeout states that all four C-11 waves are now closed/explicitly disposed, satisfying parent EC-28 and EC-30 and clearing the last C-11 Gate-18 admission interlock, while Gate 18 remains unstarted and unauthored. | Parent EC-28/EC-30; Gate 18 interlock. |


## 7. Acceptance evidence

### 7.1 Required command set

Run focused checks after each packet, then run this complete set at consolidation.
Commands execute from the repository root at the implementation commit derived
from the target baseline.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings

cargo test -p engine-core
cargo test -p game-stdlib
cargo test -p game-test-support
cargo test -p wasm-api

cargo test -p river_ledger
cargo test -p briar_circuit
cargo test -p vow_tide

cargo test -p replay-check
cargo test -p fixture-check
cargo test -p rule-coverage
cargo test --workspace

cargo run -p replay-check -- --game river_ledger --all
cargo run -p replay-check -- --game briar_circuit --all
cargo run -p replay-check -- --game vow_tide --all

cargo run -p fixture-check -- --game river_ledger
cargo run -p fixture-check -- --game briar_circuit
cargo run -p fixture-check -- --game vow_tide

cargo run -p rule-coverage -- --game river_ledger
cargo run -p rule-coverage -- --game briar_circuit
cargo run -p rule-coverage -- --game vow_tide

bash scripts/boundary-check.sh
cargo tree --workspace -e normal --invert game-test-support
cargo tree --workspace -e normal,build --invert game-test-support

node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
```

The two inverse `cargo tree` commands are expected to report no production or
build reverse dependency on `game-test-support`. Cargo's own dependency model
states that development dependencies are not used for normal package builds and
are not propagated to dependent packages; R4 nevertheless proves the concrete
workspace graph rather than relying on the label alone.[^cargo-dev-deps]

At final review, maintainers also capture:

```bash
git diff --name-only <r4-baseline>...HEAD
git diff --stat <r4-baseline>...HEAD
git diff --check <r4-baseline>...HEAD
```

`<r4-baseline>` is the implementation branch's recorded pre-wave commit, not a
branch name substituted for the analysis target. The characterization report
must map every changed path to its owning packet and approved surface.

### 7.2 Focused evidence by helper

| Helper | Focused evidence required before workspace closeout |
|---|---|
| C-01 | Constructor equality tests pin payloads, sequence, visibility scope, filtered viewer results, effect hashes, and WASM/log serialization. |
| C-02 | Table-driven canonical/alias/malformed/out-of-range tests; canonical game/WASM/trace/export outputs pinned separately from import compatibility. |
| C-03 | Valid/invalid count tables; every ring index and wrap for Vow counts 3–7; unchanged dealer/deal/bid traces; Briar exact-four diagnostic equality. |
| C-04/C-05 | Pre-change legacy hash vectors plus post-change parallel v1 vectors; path/order/label/metadata parity; explicit proof that no legacy authority field or byte changed. |
| C-06 | Three manifest reviews, source-import search, two inverse dependency-tree commands, and boundary check. |
| C-07 | Actual Cartesian products: River counts 3–6, Briar four, Vow 3–7; every source private datum, observer and every declared seat, and each named surface. Structured failure output must identify game/count/source/viewer/surface/canary. |
| C-08 | Driver accepts correct metadata, rejects wrong profile/version/field set, delegates to the owning validator, and leaves physical artifact bytes unchanged unless separately authorized. |
| C-09 | River receipt equality; Briar/Vow raw-draw/index/draw-count/permutation/deal vectors; no substitution. |
| C-10 | Source and dependency review proving no behavior owner moved and no behavioral helper was added/broadened. |

### 7.3 Golden, fixture, export, and hash policy

The default expected artifact result for R4 is:

```text
existing golden trace bytes: unchanged
existing fixture bytes: unchanged
existing public/seat-private export bytes: unchanged
existing legacy hashes: unchanged
new action-tree-v1 vectors: parallel-new
new profile adapters/metadata in tests: parallel-new
new no-leak matrix adapters in tests: parallel-new
register/report/tracker receipts: documentation-only
```

#### Authorized changes without a new ADR-0009 authority flip

- new game-owned adapter functions that call an existing accepted helper;
- new parallel action-tree v1 bytes/hash vectors;
- test-only virtual profile metadata and driver adapters;
- test-only no-leak probes, viewer enumerators, and structured contexts;
- canonical-helper delegation that is proven byte-for-byte/string-for-string
  equivalent at the existing output boundary;
- register, report, reassessed-spec, ticket, and tracker evidence; and
- a narrowly necessary test expectation correction only when the pre-change
  characterization proves the test itself was invalid and the failing-test
  protocol is documented.

#### Unauthorized by default

- any changed existing golden trace, fixture, replay export, viewer export,
  snapshot, legacy expected hash, canonical output seat string, viewer class,
  visibility decision, RNG output, RNG draw count, deck permutation, or deal;
- regenerating a directory or accepting all new snapshots;
- deleting old vectors when adding v1 vectors;
- changing an artifact and its expected digest in the same packet without an
  independently captured before value;
- treating a profile-driver migration as permission to rewrite its underlying
  artifact; or
- using a new v1 hash to replace the old authority.

An actual authorized legacy-byte change requires a separately named ADR-0009
migration packet with: surface identity; old/new schema and authority; before
bytes/hashes; new parallel reader/writer where required; unchanged/parallel-new/
intentional-migration classification; compatibility window; one-surface
rollback; and explicit maintainer acceptance. Without that packet, the task
reverts the byte change.

Canonicalization literature reinforces why a hash-bearing representation must
be explicitly versioned and invariant rather than implicitly dependent on an
ordinary serializer.[^rfc-8785] Rulepath's accepted mechanism remains its own
`StableBytesWriter`/action-tree v1 contract; R4 does **not** adopt RFC 8785 or
change serialization doctrine.

### 7.4 Hidden-information and non-interference evidence

For each C-07 packet, the report records:

- source private datum class and source seat;
- supported seat count and exact roster;
- viewer class: observer, owning seat, and every non-owner seat;
- surface: view, action tree/preview, diagnostic, effect/effect log, replay,
  public export, seat-private export, bot input, bot explanation, candidate
  rendering, and any game-specific inspector actually present;
- expected presence/absence and the game rule that owns any authorized reveal;
- generated in-memory canary identity and proof that it was not committed; and
- failure context sufficient to reproduce one cell.

The classic non-interference formulation treats information flow in terms of
what one user can observe after another user's actions; that is useful
background for the source-seat × viewer × surface product used here.[^goguen]
The binding authority, however, is Rulepath's FOUNDATIONS, ADR 0004,
Multi-Seat contract, and current game projection/reveal behavior—not the
external paper.

### 7.5 Register before/after evidence

Before the first source migration, the characterization report snapshots the
relevant existing `MSC-8C-001…010` text and pilot/R1/R2/R3 receipt tables. After
implementation, the register must show:

1. an R4 row for every migrated surface, naming game, exact sites, helper,
   characterization evidence, equality/ADR class, tests, and rollback;
2. an R4 row for every pilot-credit surface, naming the original ticket rather
   than pretending R4 reimplemented it;
3. an R4 row for every N/A/exception, naming owner, rationale, compatibility,
   rollback, and next review trigger;
4. C-06 reverse-edge proof, C-09 divergence receipts, and C-10 rejected/local
   behavior bundles; and
5. one Unit 8C-R4 closeout block with command evidence, artifact-diff result,
   exact three-game coverage, and Gate-18 interlock consequence.

The before/after review must confirm no `MSC-8C-*` helper contract was silently
broadened and no new register entry was invented to absorb game behavior.

### 7.6 External research lane

External material is non-authoritative and was used only to sharpen three
implementation disciplines:

- official Cargo documentation supports the meaning of dev-only dependencies,
  while the workspace graph remains the actual acceptance evidence;[^cargo-dev-deps]
- RFC 8785 illustrates the need for a stable, named canonical representation
  when bytes are hashed, while Rulepath continues to use its accepted native
  stable-byte taxonomy;[^rfc-8785] and
- Goguen and Meseguer provide primary-source background for non-interference,
  while the repository's viewer taxonomy defines the concrete no-leak tests.[^goguen]

No external source establishes what exists in `joeloverbeck/rulepath`, fills a
failed repository fetch, changes a verdict, or expands R4's locked scope.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Principles engaged

| Authority | Engaged rule | R4 stance |
|---|---|---|
| [`FOUNDATIONS.md`][foundations] | Rust owns behavior; determinism; hidden-information safety; tests/replay are product contracts; stop rather than silently diverge | All migrations are Rust-owned, characterized, reversible, and no-leak. Legacy byte/RNG authority is preserved unless separately versioned. |
| [`ARCHITECTURE.md`][architecture] | Narrowest lawful owner wins; engine/game/WASM layering; action/view/effect/replay separation | Generic contracts remain in `engine-core`; structural helpers remain in stdlib; test geometry/profile drivers remain dev-only; game policy stays in each game. |
| [`ENGINE-GAME-DATA-BOUNDARY.md`][boundary] | Noun-free kernel; static data is typed content/metadata, not behavior; tools are thin | No card/trick/pot/bid/hand behavior enters engine-core, fixtures, profiles, or tools. |
| [`ADR 0008`][adr-0008] and the register | Register-first, behavior-free mechanical-scaffolding extraction | R4 adopts only C-01…C-09 contracts already accepted and records every disposition under existing entries. |
| [`ADR 0009`][adr-0009] | Per-surface replay/fixture/export/hash migration taxonomy | Action-tree v1 is parallel-new; all other legacy authorities remain. Any real authority flip is separately named and reversible. |
| [`ADR 0004`][adr-0004] | Public versus seat-private replay/export visibility | Every R4 game receives explicit public and seat-private classification; private hands never enter public/other-seat surfaces. |
| [`MULTI-SEAT-AND-SURFACE-CONTRACT.md`][multi-seat] | Declared seats, canonical seat identity, observer/seat viewers, N-seat surface products | River 3–6, Vow 3–7, Briar four are enumerated exactly; dealer/pass/bid policy is not inferred from generic seat order. |
| [`TESTING-REPLAY-BENCHMARKING.md`][testing] and [`TRACE-SCHEMA-v1.md`][trace-schema] | Golden/replay determinism, no-leak tests, stable trace meaning | Existing traces are read-only by default; new v1 vectors and profile adapters are additive. |
| [`EVIDENCE-FIXTURE-CONTRACT.md`][fixture-contract] | Command/setup/domain/export profiles have distinct owners | C-08 uses separate virtual adapters and delegates semantics to game/validator owners. |
| [`AI-BOTS.md`][ai-bots] | Bots receive only legal viewer-authorized input; explanations cannot leak | C-07 includes bot input/explanation/candidate surfaces; no strategy change is admitted. |
| [`AGENT-DISCIPLINE.md`][agent-discipline] | Bounded packets, characterization, valid-test protocol, one-surface rollback | §5 is directly decomposable into `scaffold-refactor` packets and forbids test weakening. |
| [`MECHANIC-ATLAS.md`][atlas] | Primitive pressure and current promotion debt | Debt is empty. Existing trick-taking promotion remains atlas-owned and out of R4's extraction lane. |

### 8.2 Ownership matrix

| Concern | Lawful owner in R4 | Explicitly not owned by |
|---|---|---|
| Generic effect envelope type/construction | `engine-core`; game/WASM calls it | profile driver, TypeScript, static data |
| Canonical seat grammar | `engine-core::SeatId`; typed game enum conversion; WASM import compatibility | TypeScript repair, fixture aliases, browser display copy |
| Count/ring structure | `game-stdlib::seat` beneath game-owned setup/order | engine-core game policy, profile metadata |
| Betting/all-in/reopen/pots/side pots/showdown | `games/river_ledger` | engine-core, stdlib scaffolding, tools, fixtures |
| Pass/trick/moon policy | `games/briar_circuit`, with existing accepted trick helper only where already used | C-07 harness, C-08 driver, WASM |
| Dealer/hand schedule/bid/contract/trick/scoring | `games/vow_tide`, with existing accepted trick helper only where already used | seat helper, profile metadata, WASM |
| Projection/redaction/reveal authorization | each game's `visibility`/replay owner under ADR 0004 | no-leak harness, UI, effect constructor |
| Action-tree v1 encoding/hash | existing `engine-core` contract called by a game adapter | WASM legality or a new serializer |
| Pairwise no-leak enumeration/reporting | dev-only `game-test-support` | production game behavior |
| Profile metadata validation/dispatch | dev-only `game-test-support`, with tools thin | game rules encoded as data |
| Trace/fixture/export interpretation | game or existing validator/tool owner named by profile | profile metadata itself |
| RNG algorithm and consumption | `engine-core` API selected explicitly by game setup; versioned under ADR 0009 | an incidental scaffolding refactor |

### 8.3 FOUNDATIONS §12 stop conditions

The owning task stops before source migration or closeout when any of these is
true:

- a selected helper cannot preserve characterized behavior, bytes, strings,
  visibility, or RNG consumption at its stated equality bar;
- the task would need to add a card/trick/pot/bid/hand/deal noun or rule to
  `engine-core`;
- a profile or fixture would need selectors, conditions, formulas, or executable
  behavior to validate the case;
- a shared helper would decide legality, reveal, authorization, projection,
  dealer/pass/bid/trick/pot/side-pot/scoring policy, or diagnostic wording;
- a required repository file/receipt cannot be located at the working-tree
  baseline;
- the migration would change a legacy authority without an accepted ADR-0009
  packet;
- a hidden datum appears on an unauthorized viewer surface or a canary would
  need to be committed;
- `game-test-support` appears in a normal/build dependency path;
- an executing packet proposes broad goldens/snapshots regeneration or test
  weakening; or
- a foundation/accepted ADR conflict is discovered.

A stopped surface is not silently omitted. It becomes a blocking note,
reviewed exception, or separately admitted ADR task before the relevant R4 exit
criterion can pass.

### 8.4 FOUNDATIONS §13 ADR triggers

No new ADR is expected. The following findings would be genuine triggers rather
than reasons to improvise locally:

1. the accepted action-tree v1 contract cannot represent an existing River,
   Briar, or Vow action tree without changing its public contract;
2. ADR 0004 cannot classify an actual N-seat seat-private export or multipot
   viewer explanation without ambiguity;
3. ADR 0009 lacks a lawful compatibility/versioning path for a required legacy
   authority change;
4. Briar or Vow must change from modulo sampling to unbiased sampling to satisfy
   a product requirement;
5. River side-pot domain evidence requires a new canonical byte authority rather
   than virtual metadata over the game-owned validator;
6. preserving a required historical seat spelling would force canonical output
   and import aliases to share authority; or
7. any proposed helper must decide behavior on the register Non-Promotion List.

The spec/report must flag the trigger, affected surface, blocking criterion, and
required maintainer decision. R4 does not author or accept the ADR itself.

## 9. Forbidden changes

In addition to §3.12, implementation packets must not:

1. change the locked game set or absorb Gate 18;
2. introduce a new helper, broaden C-01…C-09, or modify an accepted helper's
   semantic contract to fit one game;
3. treat `game-stdlib::trick_taking` as mechanical scaffolding or reopen its
   accepted promotion;
4. move River betting, all-in, reopen, contribution, pot/side-pot, uncalled
   return, evaluator, showdown, remainder, reveal, or explanation policy;
5. move Briar deal, pass target/exchange, follow-suit, first-trick, hearts-broken,
   trick-winner/leader, moon, match scoring, projection, or bot policy;
6. move Vow deal, hand schedule, dealer, bid/contract/hook/last-bidder,
   follow-suit, trump, trick-winner/leader, exact-bid scoring, projection, or bot
   policy;
7. invent a Vow private effect merely because Vow has private views;
8. apply C-02 to card, trick, contract, pot, hand, or other non-seat IDs;
9. make WASM or TypeScript repair canonical seat outputs or decide legality;
10. make the no-leak harness decide what is secret or when it becomes public;
11. encode setup, allocation, trick, bid, scoring, or projection rules in profile
    metadata, fixtures, TOML, JSON, YAML, or a new DSL;
12. replace an old hash with action-tree v1, or remove legacy readers/vectors;
13. change Briar/Vow RNG algorithm, draw count, shuffle order, or deal order;
14. change any existing golden/fixture/export byte without a separately accepted
    per-surface migration packet;
15. write a canary into a trace, fixture, export, snapshot, log, identifier, or
    committed source datum;
16. add `game-test-support` to production/build dependencies or expose it through
    a production feature;
17. delete, ignore, loosen, or collapse a focused game test into a generic test;
18. combine unrelated games or surface classes in one rollback unit;
19. alter benchmark thresholds, bot strength, UI, catalog, art, rules prose, or
    IP posture; or
20. amend FOUNDATIONS, an accepted ADR, ROADMAP, or foundation contracts merely
    to make R4 convenient.

## 10. Documentation updates required

| Document/artifact | Required? | Closeout change |
|---|---:|---|
| `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` | yes | Create the full report described in §4.2 and record the working-tree evidence basis. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes | Add R4 receipts under existing `MSC-8C-001…010` entries and a Unit 8C-R4 closeout evidence block. Preserve all pilot/R1/R2/R3 history. |
| `specs/README.md` | yes, at final closeout only | Flip `8C-R4` to `Done`, link the accepted spec/report, state completion evidence, and note that the final C-11 Gate-18 admission interlock is cleared. Do not start or rewrite Gate 18. |
| `specs/8c-r4-n-seat-private-trick-scaffolding.md` | yes, after `/reassess-spec` | Save the accepted/reassessed version under the one-line-correctable slug; archive only after closeout under the normal archival workflow. |
| Generated `tickets/*.md` | yes, after `/spec-to-tickets` | Bounded packets corresponding to §5, each using profile `scaffold-refactor` and referencing rather than copying law. |
| `apps/web/README.md` | **not applicable** | R4 adds no game, renderer, catalog entry, setup mode, or browser smoke layer. `check-catalog-docs.mjs` is a regression guard only. |
| Game `docs/**` | not expected | No rule, mechanic, bot, UI, benchmark, source, admission, or release contract changes. Update only if a characterization finds a factual receipt link that must point to new R4 evidence; do not rewrite behavior prose. |
| `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`, boundary/area docs, ADRs | no by default | A genuine gap is a flagged §8.4 ADR-trigger/blocking note. R4 does not silently amend doctrine. |
| `docs/ROADMAP.md` | no | Progress belongs in `specs/README.md`; R4 does not alter roadmap law. |
| `docs/MECHANIC-ATLAS.md` | no expected change | Current debt remains none; existing trick-taking promotion and deferred rows remain authoritative. |
| Templates | no expected change | They are authority/input for packet and evidence shape, not R4 deliverables. |

Documentation changes are subordinate to passing source/evidence work; the
tracker must not be flipped early to manufacture admission.


## 11. Sequencing

### 11.1 Roadmap position

| Position | Unit | Status/relationship |
|---|---|---|
| Predecessor | `8C-R3` — public/co-op/asymmetric/trick scaffolding | `Done` on 2026-06-24; its closeout is the direct admission predecessor. |
| Current | `8C-R4` — N-seat/private/trick residual scaffolding | This planned intermediate artifact; final and fourth C-11 wave. |
| Successor | Gate 18 — Spades/partnerships | Remains `Not started`. It is admitted through the normal tracker workflow only after R4 closes and all other named Gate-18 conditions remain satisfied. |

### 11.2 Admission rule

R4 implementation may begin only after:

1. this intermediate artifact is saved for review;
2. `/reassess-spec` verifies the current implementation checkout against the
   working-tree characterization and resolves any one-line-correctable symbol or
   task boundary;
3. `/spec-to-tickets` emits bounded `scaffold-refactor` packets;
4. tasks 001–003 pin the execution baseline and complete matrix; and
5. no new primitive-promotion debt or accepted-ADR conflict has appeared.

A reassessment may split a packet or correct a path/symbol. It may not reopen the
locked unit/game set, convert an N/A algorithm migration into silent work, or
move Gate 18 into R4.

### 11.3 Execution order and closeout

- Characterization precedes every migration.
- Canonical game seats precede their WASM import adapters.
- Structural count/ring changes precede profiles that enumerate those seats.
- A game-owned action-tree adapter precedes its v1 encoding/hash.
- Viewer geometry precedes the corresponding export profile closeout.
- C-09 remains evidence-only unless a separately accepted migration is admitted
  outside this spec.
- Register receipts trail the owning source/evidence packet; the tracker flip
  trails the full command/diff audit.

When `8C-R4` reaches `Done`, all four C-11 follow-on waves have a closed or
explicitly accepted disposition. That event clears the last **C-11** item in
Gate 18's admission interlock. It does not implement Gate 18, resolve every
future partnership question, or authorize work outside the successor's own
specification process.

## 12. Assumptions

### 12.1 One-line-correctable assumptions

- `assumption:` the unit slug remains
  `8c-r4-n-seat-private-trick-scaffolding`; the fixed unit ID is `8C-R4`.
- `assumption:` the accepted final spec is saved as
  `specs/8c-r4-n-seat-private-trick-scaffolding.md` before decomposition and is
  archived only at closeout.
- `assumption:` River Ledger supports 3–6 seats, Vow Tide supports 3–7 seats,
  and Briar Circuit supports exactly four at the implementation baseline. A
  deeper current-code read may correct a residual variant predicate but may not
  add a game or expand the locked unit.
- `assumption:` candidate task IDs beginning `8CR4NSEAT-` are provisional and
  may be mechanically renamed by `/spec-to-tickets` without changing task
  boundaries.
- `assumption:` the existing engine-core action-tree v1 API is sufficient for
  the selected game-owned adapters without a helper-contract change; otherwise
  §8.4 trigger 1 blocks that surface.
- `assumption:` all existing golden traces, fixtures, viewer exports, canonical
  output seat strings, and legacy hashes can remain unchanged; any proven
  exception requires its own ADR-0009 packet.
- `assumption:` River's selected side-pot domain evidence can use virtual
  `domain-evidence-v1` metadata over existing game-owned validators without
  declaring a new canonical byte authority.
- `assumption:` Briar and Vow continue to use legacy modulo `next_index` at the
  execution baseline; R4 characterizes and records N/A rather than changing the
  algorithm.
- `assumption:` no foundation-doc or ADR amendment is required. A genuine gap
  is reported as a blocking FOUNDATIONS §13 trigger.
- `assumption:` no `apps/web` source or documentation change is needed; web and
  catalog commands remain regression checks only.
- `assumption:` Gate 18 remains unstarted and out of scope even after R4's
  closeout clears the final C-11 interlock.

### 12.2 `/reassess-spec` checklist

Before accepting the repo-local spec, reassessment must:

1. verify the working-tree baseline and record it as research provenance;
2. re-resolve every listed path/symbol against the implementation checkout and
   correct only mechanical drift;
3. retain the grounded corrections in the preamble unless stronger exact-file
   evidence changes a sub-surface verdict;
4. verify every pilot-credit ticket/receipt and prevent duplicate work;
5. re-run the C-09 code inspection for Briar/Vow before retaining N/A;
6. verify River ticket-021 coverage before selecting only residual no-leak
   scenarios;
7. split any candidate packet that would touch two legacy authorities or two
   rollback surfaces;
8. ensure each ticket names exact pre-change hashes/bytes/visibility classes or
   an explicit N/A;
9. verify no proposed source edit broadens a shared helper or moves behavior;
10. retain all §6 exit criteria and §7 command/diff policy; and
11. keep the determination, exact three-game set, and Gate-18 exclusion locked.

### 12.3 Authority and evidence basis read for authoring

The repository evidence basis read for this artifact included:

- **all of `docs/**`:** the authority index, FOUNDATIONS, architecture and
  engine/game/data boundary, mechanic atlas, mechanical-scaffolding register,
  multi-seat/surface contract, evidence/trace/testing contracts, AI/UI/WASM/IP/
  source/official-game/roadmap/agent/archival documents, ADRs 0001–0009, and the
  ADR template;
- **all of `templates/**`:** packet, evidence, admission, rules/coverage,
  mechanics, AI/bot, UI, benchmark, source, primitive-pressure, release, and
  template-index documents;
- **planning law and precedents:** `specs/README.md`, the Unit 8C parent, R1/R2/
  R3 specs and characterization reports, the four owning game specs, and the
  Part-C source report;
- **pilot receipts:** the relevant `UNI8CMECSCA-*` tickets, especially 017,
  021, 024, 025, and 026, plus the canonical seat/count pilot tickets;
- **the three game implementations:** Cargo manifests; source seams for actions,
  effects, IDs, setup, state, replay, visibility, rules/scoring/bots where
  necessary; tests; all relevant golden-trace names; and all data fixtures;
- **shared landing homes:** engine action/replay/RNG contracts, stdlib seat and
  trick-taking helpers, dev-only no-leak/profile drivers, and WASM seat/game
  adapters; and
- **tools/guards:** replay-check, fixture-check, rule-coverage, boundary check,
  documentation links, and catalog-doc regression checks.

The research brief supplied scope and authority-order instructions, not
repository file contents.

### 12.4 Author self-check

- [x] The determination is confirmed, not reopened; R3 is Done, R4 is the lowest
      open row, atlas debt is empty, parent EC-28/EC-30 govern, and Gate 18 stays
      out of scope.
- [x] Exactly `river_ledger`, `briar_circuit`, and `vow_tide` are implementation
      subjects.
- [x] The verdict vocabulary includes and uses
      `already-discharged-by-8C-pilot`; every C-01…C-09 cell and listed
      sub-surface has an owner and disposition.
- [x] River's pilot credit and residual stack/all-in/runout/multipot work are
      separated without relying on the corrected chronology.
- [x] C-07 and `seat-private-export-v1` are applicable to all three games at the
      private-view/export layer, while Vow private **effect envelopes** are
      correctly N/A.
- [x] Vow C-02 is migrated rather than falsely credited to the pilot.
- [x] Briar/Vow C-09 records current modulo algorithms and forbids silent
      substitution.
- [x] C-06 is a dependency checkpoint, not fresh adoption.
- [x] Betting/pot/side-pot/pass/trick/bid/deal/partnership/scoring/reveal policy
      remains game-local; the existing trick helper is not reopened.
- [x] Every byte/hash/seat/visibility/RNG task is characterized, versioned where
      required, and one-surface reversible; no blanket golden policy exists.
- [x] The 12-section format, bounded candidate packets, row-mapped exit criteria,
      commands, register updates, tracker flip, and `apps/web/README.md` N/A are
      present.
- [x] This remains an intermediate artifact for `/reassess-spec` then
      `/spec-to-tickets`, not a claim that code has shipped.

[docs-readme]: ../docs/README.md
[foundations]: ../docs/FOUNDATIONS.md
[architecture]: ../docs/ARCHITECTURE.md
[boundary]: ../docs/ENGINE-GAME-DATA-BOUNDARY.md
[roadmap]: ../docs/ROADMAP.md
[atlas]: ../docs/MECHANIC-ATLAS.md
[register]: ../docs/MECHANICAL-SCAFFOLDING-REGISTER.md
[multi-seat]: ../docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md
[testing]: ../docs/TESTING-REPLAY-BENCHMARKING.md
[trace-schema]: ../docs/TRACE-SCHEMA-v1.md
[fixture-contract]: ../docs/EVIDENCE-FIXTURE-CONTRACT.md
[ai-bots]: ../docs/AI-BOTS.md
[agent-discipline]: ../docs/AGENT-DISCIPLINE.md
[agent-task]: ../templates/AGENT-TASK.md
[adr-0004]: ../docs/adr/0004-hidden-info-replay-export-taxonomy.md
[adr-0008]: ../docs/adr/0008-mechanical-scaffolding-governance.md
[adr-0009]: ../docs/adr/0009-replay-fixture-hash-taxonomy.md
[spec-index]: README.md
[parent]: ../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md
[r1-spec]: ../archive/specs/8c-r1-public-fixed-seat-scaffolding.md
[r2-spec]: ../archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md
[r3-spec]: ../archive/specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md
[gate-15-1]: ../archive/specs/gate-15-1-river-ledger-all-in-side-pots.md
[ticket-009]: ../archive/tickets/UNI8CMECSCA-009.md
[ticket-011]: ../archive/tickets/UNI8CMECSCA-011.md
[ticket-017]: ../archive/tickets/UNI8CMECSCA-017.md
[ticket-021]: ../archive/tickets/UNI8CMECSCA-021.md
[ticket-024]: ../archive/tickets/UNI8CMECSCA-024.md
[ticket-025]: ../archive/tickets/UNI8CMECSCA-025.md
[ticket-026]: ../archive/tickets/UNI8CMECSCA-026.md

[^cargo-dev-deps]: Cargo Reference, “Development dependencies,” <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies>.
[^rfc-8785]: RFC 8785, *JSON Canonicalization Scheme (JCS)*, <https://www.rfc-editor.org/rfc/rfc8785.html>.
[^goguen]: Joseph A. Goguen and José Meseguer, “Security Policies and Security Models,” 1982 IEEE Symposium on Security and Privacy, <https://doi.org/10.1109/SP.1982.10014>.
