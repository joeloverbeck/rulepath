# Unit 8C-R2 — C-11 follow-on: two-seat hidden/reaction scaffolding

| Field | Value |
|---|---|
| Spec ID | `8C-R2` |
| Artifact slug | `8c-r2-two-seat-hidden-reaction-scaffolding` |
| Roadmap stage | Public scaling phase — C-11 follow-on retrofit lane |
| Roadmap build gate | `8C-R2` (precedes `8C-R3`, `8C-R4`, and Gate 18) |
| Status | `Planned` |
| Date | 2026-06-23 |
| Owner | Rulepath maintainers; implementation delegated through bounded `AGENT-TASK` packets |
| Analysis baseline | Authored against `joeloverbeck/rulepath` at `e06bdb0`, then re-grounded against current `main` during `/reassess-spec`: every referenced symbol, path, and governance row below validated unchanged. `e06bdb0` is now an ancestor of `main`. |

> **Locked authored-plan posture.** This is an intermediate `new-spec` artifact, not executed code and not a claim that the repository already contains the planned result. Save it under `specs/` for `/reassess-spec`, then use `/spec-to-tickets` only after reassessment accepts the grounded matrix. The eventual target path is `specs/8c-r2-two-seat-hidden-reaction-scaffolding.md`; archive it only at closeout. Scope and determination are locked; reassessment may correct one-line implementation details but may not reopen the unit, add games, or absorb successor work.

This spec is subordinate to the foundation set indexed by
[`docs/README.md`](../docs/README.md). Where this spec and a higher-authority
document disagree, the higher-authority document wins. Authority order:
[`docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md) →
[`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) →
[`docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md) →
accepted ADRs and area contracts →
[`docs/ROADMAP.md`](../docs/ROADMAP.md) → this spec → tickets.

**Grounded correction to the research brief.** The brief states that High Card
Duel's Unit 8C pilot discharged both C-04 action-tree v1 and C-07 no-leak
geometry. The repository evidence supports only the C-07 part:
the active tracker says the pilot discharges only its named no-leak surface;
the parent seed says the same; `MSC-8C-004` names Race to N and Draughts Lite
as the C-04 pilots; and `MSC-8C-007` names High Card Duel's observer/seat-0/
seat-1 matrix. High Card's pilot does snapshot its action tree as a **C-07
leak surface**, but that is not C-04 canonical action-tree byte/hash adoption.
Therefore this spec assigns High Card C-04/C-05 `migrate` and C-07
`already-discharged-by-8C-pilot`. This correction does not reopen the locked
unit or game set.

## 1. Determination

### 1.1 Locked determination

The next-unit determination is confirmed, not re-decided:

1. [`specs/README.md`](../specs/README.md) records `8C-R1` as `Done`, completed
   2026-06-23. `8C-R2` is the lowest active-epoch row whose status is
   `Not started`; `8C-R3`, `8C-R4`, and Gate 18 follow it. The same file's
   workflow requires selecting the lowest non-`Done` unit, authoring its spec,
   and then running `/reassess-spec` → `/spec-to-tickets`.
2. [`docs/MECHANIC-ATLAS.md`](../docs/MECHANIC-ATLAS.md) §10A says
   `Current debt: None`, last reviewed at Gate 17 closeout. No open primitive
   promotion debt precedes this unit. This is a code-scaffolding retrofit, not
   a mechanic-ladder gate.
3. [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`](../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md)
   §5 seeds exactly `8C-R2` with `high_card_duel` (residual),
   `secret_draft`, `poker_lite`, and `masked_claims`; it requires explicit
   ADR-0004 evidence and adoption of geometry without moving reveal/reaction
   policy. Work item `8C-030` created the four C-11 rows. Parent EC-28 assigns
   every official game exactly once across those bounded rows, while EC-30
   keeps Gate 18 after their closure, accepted not-applicability, or accepted
   exception.
4. [`archive/specs/8c-r1-public-fixed-seat-scaffolding.md`](../archive/specs/8c-r1-public-fixed-seat-scaffolding.md)
   is the direct structural precedent: complete applicability matrix,
   sub-surface classifications, characterization first, one surface per diff,
   ADR-0009 migration packets, register receipts, and tracker closeout.

**Determination:** Unit `8C-R2` is the required current unit. Its bounded game
set is exactly:

- `high_card_duel` — residual audit;
- `secret_draft`;
- `poker_lite`;
- `masked_claims`.

No fifth game, `8C-R3`/`8C-R4` work, or Gate 18 work is admitted.

### 1.2 Evidence posture

Repository claims in this spec were authored against `joeloverbeck/rulepath`
at `e06bdb0` and re-grounded against current `main` during `/reassess-spec`:
every referenced symbol, path, and governance row below was confirmed against
the working tree at reassessment time. The original baseline is now an ancestor
of `main`, and the four-game C-01…C-10 surfaces validated unchanged across that
span — so the analysis holds for the current tree, not merely the authoring
commit.

## 2. Objective

Within the C-11 retrofit lane established by the completed Unit 8C parent and
the public-scaling sequence in
[`docs/ROADMAP.md`](../docs/ROADMAP.md), Unit 8C-R2 must turn the hidden-
information/reaction seed into a bounded, reversible implementation plan. It
must:

1. audit every official C-01…C-08 surface in the four locked games;
2. resolve every game/helper and helper sub-surface to exactly one verdict:
   **migrate**, **not-applicable**, **exception**, or
   **already-discharged-by-8C-pilot**;
3. adopt behavior-free constructors, seat grammar, structural count checks,
   canonical action-tree bytes, test geometry, and typed evidence-profile
   metadata while leaving every reveal, commitment, reaction, projection,
   redaction, pledge, pot, score, and outcome rule in its game;
4. treat private effects, viewer-scoped views, public exports, actual
   seat-private exports, bot surfaces, and no-leak matrices as first-class
   applicable surfaces rather than repeating R1's public-game N/A decisions;
5. characterize all byte-, hash-, seat-, visibility-, export-, fixture-, and
   RNG-consumption surfaces before changing them;
6. preserve existing bytes by default and permit any change only through one
   named ADR-0009 per-surface packet with classification, compatibility
   window, exact evidence, and isolated rollback;
7. migrate the real local unbiased-index implementations found in
   `high_card_duel`, `poker_lite`, and `masked_claims` under C-09, while
   recording `secret_draft` as not applicable; and
8. leave no unnamed “remaining cleanup” bucket.

The outcome is a characterized four-game retrofit, not a new game, mechanic,
UI feature, replay redesign, hidden-information taxonomy change, or generic
reaction framework.

## 3. Scope

### 3.1 In scope

- **Games:** exactly `high_card_duel`, `secret_draft`, `poker_lite`,
  `masked_claims`.
- **Primary audit:** C-01…C-08.
- **C-01:** public and actual seat-private effect-envelope constructors.
  The shipped API spelling is `EffectEnvelope::private_to`; the brief's
  `seat_private` wording denotes that same seat-private class.
- **C-02:** strict canonical `seat_<n>` Rust parsing and the already-landed
  import-only legacy adapter in `wasm-api`; legacy runtime-roster or historical
  trace spellings are classified explicitly rather than silently swept.
- **C-03:** exact-two-seat structural validation through `SeatCount`, with
  game-owned diagnostics and variant expectations unchanged.
- **C-04/C-05:** action-tree v1 bytes/hash as a parallel selected surface using
  `ActionTreeEncodingVersion::V1` and `StableBytesWriter`; adjacent state,
  effect, view, replay/export, and diagnostic byte surfaces are classified
  separately.
- **C-06:** `game-test-support` as a `[dev-dependencies]`-only edge.
- **C-07:** pairwise no-leak geometry over source seat × viewer × surface,
  retaining every game-specific reveal assertion.
- **C-08:** typed `replay-command-v1`, `setup-evidence-v1`,
  `public-export-v1`, applicable `seat-private-export-v1`, and explicit
  `domain-evidence-v1` N/A decisions.
- **Checkpoints:** C-09 unbiased bounded-index migration where code proves a
  real local surface; C-10 non-promotion affirmation.
- **Governance:** a characterization report, existing `MSC-8C-*` receipt
  updates, exact acceptance evidence, and the `specs/README.md` status flip.

### 3.2 Verdict vocabulary

| Verdict | Meaning |
|---|---|
| `migrate` | One named surface changes in one reviewable diff under §5.1. |
| `not-applicable` | No official surface of that class exists; the exact rationale and next trigger are recorded. |
| `exception` | A real legacy/local surface remains under a named owner, compatibility statement, rollback boundary, and next review trigger. |
| `already-discharged-by-8C-pilot` | Unit 8C already implemented and evidenced that exact surface; R2 verifies the receipt and does not rebuild it. |

### 3.3 Primary applicability and verdict matrix

The aggregate matrix covers C-01…C-08. Sub-surface tables below are
authoritative when a helper family contains both a migration and an exception
or N/A.

| Game | C-01 effects | C-02 seats | C-03 count | C-04 tree v1 | C-05 writer v1 | C-06 dev support | C-07 no-leak | C-08 profiles |
|---|---|---|---|---|---|---|---|---|
| `high_card_duel` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `already-discharged-by-8C-pilot` | `already-discharged-by-8C-pilot` | `migrate` |
| `secret_draft` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `poker_lite` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `masked_claims` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |

### 3.4 C-01 — public and seat-private effect-envelope surfaces

Constructors change only envelope construction. Payload formation, effect
ordering, reveal state, recipient choice, and filtering remain game-owned.

| Game | Public constructor | Seat-private constructor | Verdict and required result |
|---|---|---|---|
| `high_card_duel` | `games/high_card_duel/src/effects.rs::public_effect` | `private_effect` | `migrate` both independently to `EffectEnvelope::public` and `EffectEnvelope::private_to`; preserve payloads, `VisibilityScope`, stable effect strings/hashes, ordering, owner IDs, and observer/opponent filtering. |
| `secret_draft` | `games/secret_draft/src/effects.rs::public_effect` | No emitted seat-private envelope | public `migrate`; private `not-applicable`. `OwnCommitAccepted` and pre-reveal diagnostics remain viewer-safe by omitting the committed item; do not invent a private effect or move commitment/reveal policy. |
| `poker_lite` | `games/poker_lite/src/effects.rs::public_effect` | `private_effect` | `migrate` both independently; preserve private setup card delivery and bot-choice visibility exactly. |
| `masked_claims` | `games/masked_claims/src/effects.rs::public_effect` | No emitted seat-private envelope | public `migrate`; private `not-applicable`. Claim-path redaction and reveal timing remain in game-owned payload/projection code. |

The private N/A rows are not permission to weaken no-leak evidence. They mean
only that the game does not emit a seat-private **effect envelope** at this
commit.

### 3.5 C-02 — canonical seat grammar and compatibility surfaces

Canonical Rust identity is `seat_<zero-based>`. Legacy hyphen and symbolic
aliases remain import-only in
[`crates/wasm-api/src/seats.rs`](../crates/wasm-api/src/seats.rs).
TypeScript never repairs or normalizes a seat ID.

| Game | Typed Rust parser | WASM import adapter | Current WASM/runtime roster or trace helper | Verdict |
|---|---|---|---|---|
| `high_card_duel` | `HighCardDuelSeat::parse` manually matches `seat_0/1` | `parse_high_card_seat` already delegates through the bounded alias adapter | `high_card_replay_to_cursor` uses legacy `seats()`; unused `trace_high_card_seat` emits `seat-0/1` | parser `migrate`; adapter `already-discharged-by-8C-pilot`; roster/trace helper `exception` owned by `wasm-api`, preserved through C-11 because changing runtime `SeatId` bytes would touch state/effect visibility and hashes; trigger = dedicated WASM runtime-seat migration |
| `secret_draft` | `SecretDraftSeat::parse` manual | `parse_secret_seat` already uses alias adapter | replay cursor uses legacy `seats()`; game replay commands already emit canonical `as_str()` | parser `migrate`; adapter already shipped; roster `exception` with the same compatibility/rollback trigger |
| `poker_lite` | `PokerLiteSeat::parse` manual | `parse_poker_seat` already uses alias adapter | replay cursor uses legacy `seats()`; game replay commands already emit canonical `as_str()` | parser `migrate`; adapter already shipped; roster `exception` with the same trigger |
| `masked_claims` | `MaskedClaimsSeat::parse` manual | `parse_masked_seat` already uses alias adapter | `masked_seats()` and `trace_masked_seat` already emit canonical underscore IDs | parser `migrate`; adapter/output `already-discharged-by-8C-pilot`/already canonical; no output flip |

Each parser migration delegates only canonical acceptance to
`SeatId::parse_canonical` and then maps indices 0/1 to the typed enum. It must
continue rejecting `seat-0`, symbolic aliases, ambiguous labels, out-of-range
IDs, leading-zero variants, Unicode lookalikes, and role names inside game
crates. Those aliases remain accepted only by the Rust WASM import adapter.

### 3.6 C-03 — exact-two-seat structural validation

All four games are fixed-two-seat at this commit. `high_card_duel` compares
against its variant's declared count, whose official variant is two; the other
three compare against `STANDARD_SEAT_COUNT`.

| Game | Exact site | Verdict | Required semantic result |
|---|---|---|---|
| `high_card_duel` | `games/high_card_duel/src/setup.rs::setup_match` | `migrate` | Validate nonzero structure with `SeatCount`, retain `options.variant.seat_count` as game-owned expected value, and preserve exact diagnostic code/message and setup/RNG behavior. |
| `secret_draft` | `games/secret_draft/src/setup.rs::setup_match` | `migrate` | Replace only the hand-written length predicate with structural `SeatCount` use; add normal `game-stdlib` dependency; preserve diagnostics and state bytes. |
| `poker_lite` | `games/poker_lite/src/setup.rs::setup_match` | `migrate` | Same bounded migration. |
| `masked_claims` | `games/masked_claims/src/setup.rs::setup_match` | `migrate` | Same bounded migration. |

`SeatCount::next_ring_index` is `not-applicable` for this wave. Existing typed
two-seat `other()` or phase-order rules remain game-local.

### 3.7 C-04/C-05 — action-tree v1 and adjacent stable-byte surfaces

The selected migration is the legal action tree only. Every game adds or
adopts a version-explicit parallel v1 byte/hash surface. No existing replay,
fixture, or legacy hash is silently reinterpreted.

| Game | Current surface | Verdict | Migration classification |
|---|---|---|---|
| `high_card_duel` | `legal_action_tree`; no game-owned v1 byte/hash wrapper. Unit 8C uses a debug snapshot only inside C-07. | `migrate` | `parallel-new-surface`: add explicit v1 bytes/hash evidence; do not relabel the C-07 snapshot as C-04. |
| `secret_draft` | local `replay_support::action_tree_hash` string encoder | `migrate` | retain legacy hash as `exception`; add parallel v1 bytes/hash; compare semantics and ordering. |
| `poker_lite` | local `replay_support::action_tree_hash` string encoder | `migrate` | same parallel migration and legacy compatibility. |
| `masked_claims` | compound claim tree and response tree; no game-owned canonical tree hash | `migrate` | `parallel-new-surface` covering both compound claim and flat response shapes without changing legality or pending-responder policy. |

Adjacent C-05 classifications apply to every game:

| Surface class | Verdict | Owner/compatibility/rollback/trigger |
|---|---|---|
| selected action-tree v1 bytes/hash | `migrate` | Game replay/evidence adapter calls `engine-core`; legacy tree authority remains readable; rollback removes only the new v1 adapter/tests. |
| internal state bytes/hash | `exception` | game-owned current stable summary; unchanged; trigger = dedicated state-hash migration |
| effect bytes/hash | `exception` | game-owned current effect stable strings; unchanged; trigger = dedicated effect-hash migration |
| public or seat-private view bytes/hash | `exception` | game-owned projection contract; unchanged; trigger = dedicated view migration with ADR-0004 proof |
| replay-command bytes/hash | `exception` for encoding, C-08 profile metadata only | current trace bytes remain authoritative; trigger = separately named replay migration |
| public/seat-private export bytes/hash | `exception` for encoding, C-08 profile metadata only | current export bytes remain authoritative; trigger = separately named export migration |
| diagnostic bytes/hash | `exception` or `not-applicable` by game | diagnostics remain local; trigger = dedicated diagnostic-surface migration |

### 3.8 C-07 — pairwise no-leak geometry

The shared harness owns only deterministic matrix enumeration and structured
failure reporting. Each game owns canaries, snapshots, phase setup,
authorization/reveal expectations, and containment. Existing focused tests are
retained; the generic matrix does not subsume them.

For every table below, run each row twice: once with source `S = seat_0` and
once with `S = seat_1`. `Owner` means viewer `S`; `opponent` means the other
seat. `A` = `MustBeAbsent`, `P` = `MustBePresent`, `N/A` =
`NotApplicable`. Phase-dependent public reveal is set up by game code, not by
the harness.

#### High Card Duel — receipt verification, not reconstruction

| Source datum / phase / surface | Observer | Owner | Opponent |
|---|---:|---:|---:|
| unrevealed private card → projected view | A | P | A |
| unrevealed private card → legal action tree | A | A | A |
| unrevealed private card → diagnostic | A | A | A |
| private deal/own-commit effect → filtered effects | A | P | A |
| unrevealed private card → public replay export | A | A | A |
| unrevealed private card → bot input for acting seat | N/A | P | A |
| raw private ID → bot explanation/candidate rendering | A | A | A |
| cards after game-authorized reveal → public view/effect/export | P | P | P |

Verdict: `already-discharged-by-8C-pilot`, verified against
`MSC-8C-007` and the existing
`no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces` test.
R2 adds only residual profile/effect/seat/count/tree/RNG work.

#### Secret Draft — simultaneous commitment/reveal remains game-local

| Source datum / phase / surface | Observer | Owner | Opponent |
|---|---:|---:|---:|
| committed item before synchronized reveal → projected view | A | A | A |
| committed item before reveal → action tree/metadata | A | A | A |
| committed item before reveal → diagnostic | A | A | A |
| committed item before reveal → filtered effects | A | A | A |
| committed item before reveal → public export | A | A | A |
| committed item before reveal → seat-private export | A | A | A |
| committed item → internal command trace | N/A | N/A | N/A |
| item after game-authorized simultaneous reveal → public view/effect/export | P | P | P |
| raw pre-reveal choice → bot explanation/candidates | A | A | A |

The internal command trace is a distinct `internal-dev` authority and is tested
separately; it is not a viewer surface. Verdict: `migrate`.

#### Poker Lite — hand privacy, showdown, and yield policy remain game-local

| Source datum / phase / surface | Observer | Owner | Opponent |
|---|---:|---:|---:|
| private crest before showdown → projected view | A | P | A |
| private crest before showdown → action tree/metadata | A | A | A |
| private setup/choice effect → filtered effects | A | P | A |
| private crest before showdown → diagnostic | A | A | A |
| private crest before showdown → public export | A | A | A |
| private crest before showdown → seat-private export | A | P | A |
| own private crest → bot input | N/A | P | A |
| raw private ID before reveal → explanation/candidates | A | A | A |
| both crests after authorized showdown → public view/export | P | P | P |
| unshown losing crest after yield → public view/export | A | P or N/A per existing owner view | A |

Verdict: `migrate`.

#### Masked Claims — reaction window and claim redaction remain game-local

| Source datum / phase / surface | Observer | Owner | Opponent/responder |
|---|---:|---:|---:|
| unrevealed hand tile → projected view | A | P | A |
| claimed tile identity during pending response → public/opponent view | A | P or N/A per existing owner view | A |
| claimed tile identity → responder action tree | A | N/A | A |
| claimed tile identity → public effect/diagnostic | A | A | A |
| claimed tile identity → public replay export | A | A | A |
| unrevealed tile → bot input for owning seat | N/A | P | A |
| raw tile ID → bot rationale/candidates | A | A | A |
| accepted mask that rules keep secret → public/opponent surfaces after resolution | A | N/A | A |
| challenged tile after game-authorized reveal → public view/effect/export | P | P | P |

Verdict: `migrate`. The harness may not infer who may respond, when a claim is
pending, whether an accepted mask remains hidden, or when a challenge reveals.

### 3.9 C-08 — evidence-profile driver matrix

Profile drivers validate metadata and delegate behavior to the game or owning
validator. They do not parse commands, project views, authorize exports, or
interpret fixtures.

| Profile | `high_card_duel` | `secret_draft` | `poker_lite` | `masked_claims` |
|---|---|---|---|---|
| `replay-command-v1` | `migrate`; `internal-dev`; current internal trace bytes remain authority | `migrate`; `internal-dev` | `migrate`; `internal-dev` | `migrate`; `internal-dev`; use existing rule/replay construction, not a new omniscient export |
| `setup-evidence-v1` | `migrate`; fixture metadata only; exact private deal assertions remain internal-dev | `migrate`; fixture contains public setup parameters/empty commitments | `migrate`; fixture metadata/deck declaration, not an exported private hand | `migrate`; fixture metadata/mask ordering, not executable reveal policy |
| `public-export-v1` | `migrate`; observer-only `export_public_observer_replay` | `migrate`; observer invocation of `export_public_replay` | `migrate`; observer invocation of `export_public_replay` | `migrate`; observer-only `PublicReplayExport` path |
| `seat-private-export-v1` | `not-applicable`: seat-private **view** trace exists, but no official seat-private replay exporter | `migrate`: the game exporter accepts a seat `Viewer`; cover `seat_0` and `seat_1` and preserve pre-reveal redaction | `migrate`: same, preserving own-hand-only access and showdown/yield policy | `not-applicable`: constructor can carry a viewer label, but the official import/export path is observer-only and no game-owned seat-private timeline exporter exists |
| `domain-evidence-v1` | `not-applicable`: no distinct domain fixture beyond setup | `not-applicable` | `not-applicable` | `not-applicable` |

`canonical_byte_authority` is `none` when a driver asserts metadata and calls an
existing validator without claiming new canonical bytes. Where current
trace/export bytes are compared, the existing game/tool validator remains the
authority. No profile task may rewrite an artifact merely to add metadata.

### 3.10 C-06/C-09/C-10 checkpoint matrix

| Game | C-06 dev-only support | C-09 bounded index | C-10 non-promotion |
|---|---|---|---|
| `high_card_duel` | `already-discharged-by-8C-pilot`; dev dependency already present | `migrate`: `setup.rs::next_bounded_index_unbiased` is algorithmically the shipped v1 sampler | reaffirm: shuffle/deal/commit/reveal/outcome local |
| `secret_draft` | `migrate`: add `game-test-support` under `[dev-dependencies]` only | `not-applicable`: no RNG/bounded-index rule surface | reaffirm: simultaneous commitment/reveal and visible-pool resolution local |
| `poker_lite` | `migrate` | `migrate`: local rejection sampler in `setup.rs` | reaffirm: pledge rounds, showdown, shared-pool allocation, scoring local |
| `masked_claims` | `migrate` | `migrate`: local rejection sampler in `setup.rs` | reaffirm: reaction window, pending responder, redaction, challenge reveal, outcome local |

For every C-09 migration, pin returned indices, rejection draw counts, complete
shuffle/deal vectors, state/effect/view/export hashes, and relevant golden
traces before replacement. The expected ADR-0009 class is `unchanged`; any
observed divergence stops that game's migration.

### 3.11 Out of scope

- `8C-R3`, `8C-R4`, Gate 18, partnerships, or any new game.
- New public APIs beyond the smallest game adapter needed to call already
  accepted scaffolding.
- New hidden-information taxonomy, replay schema, action grammar, fixture
  schema, or UI contract.
- A generic commitment/reveal, reaction-window, pending-responder, betting,
  pledge, pot, showdown, scoring, or outcome framework.
- Browser/catalog/renderer work.
- Bot strategy changes.
- Performance-threshold changes.
- Fixture/data migration except a separately named, characterized artifact
  explicitly added during reassessment.

### 3.12 Not allowed

- Moving legality, setup policy, reveal timing, response authorization,
  projection, redaction, scoring, outcome, or bot choice into shared code.
- A helper that decides what is hidden, when it becomes visible, who may
  respond, or how a pledge/pot/claim resolves.
- Game nouns in `engine-core`.
- A normal/build dependency on `game-test-support`.
- TypeScript seat normalization or legality.
- Silent byte/hash/seat-ID/visibility/RNG-consumption changes.
- Blanket golden regeneration or “update snapshots”.
- Test deletion, weakening, replacement of a specific assertion with a generic
  one, or committed leak canaries.
- YAML, DSLs, selectors, triggers, formulas, or executable fixture behavior.

## 4. Deliverables

### 4.1 Concrete artifact tree

The reassessed implementation may touch only the following bounded families.

```text
reports/
└── 8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md

games/high_card_duel/
├── Cargo.toml                                  # audit only unless dependency proof requires correction
├── src/effects.rs                              # C-01 public/private
├── src/ids.rs                                  # C-02 parser
├── src/setup.rs                                # C-03 and C-09
├── src/replay_support.rs                       # C-04/C-05 and C-08
├── src/visibility.rs                           # behavior read-only by default
├── tests/replay.rs                             # C-04/C-05/C-08 evidence
├── tests/visibility.rs                         # C-07 receipt verification; retain specific tests
├── tests/bots.rs                               # no-leak bot surfaces
├── tests/serialization.rs                      # byte/readability evidence
├── tests/golden_traces/*.trace.json            # read-only by default
└── data/fixtures/high_card_duel_standard.fixture.json  # read-only by default

games/secret_draft/
├── Cargo.toml                                  # game-stdlib normal; game-test-support dev-only
├── src/effects.rs
├── src/ids.rs
├── src/setup.rs
├── src/replay_support.rs
├── src/visibility.rs                           # policy read-only
├── tests/{replay,visibility,bots,serialization}.rs
├── tests/golden_traces/*.trace.json            # read-only by default
└── data/fixtures/secret_draft_standard.fixture.json

games/poker_lite/
├── Cargo.toml
├── src/effects.rs
├── src/ids.rs
├── src/setup.rs
├── src/replay_support.rs
├── src/visibility.rs                           # policy read-only
├── tests/{replay,visibility,bots,serialization}.rs
├── tests/golden_traces/*.trace.json            # read-only by default
└── data/fixtures/poker_lite_standard.fixture.json

games/masked_claims/
├── Cargo.toml
├── src/effects.rs
├── src/ids.rs
├── src/setup.rs
├── src/replay_support.rs
├── src/visibility.rs                           # policy read-only
├── tests/{replay,visibility,bots,serialization}.rs
├── tests/golden_traces/*.trace.json            # read-only by default
└── data/fixtures/masked_claims_standard.fixture.json

crates/wasm-api/src/
├── seats.rs                                    # import adapter/legacy roster receipt; no default output flip
└── games/{high_card,secret,poker,masked}.rs    # compatibility evidence only by default

tools/
├── replay-check/src/main.rs                    # thin dispatch only if a named profile lacks it
└── fixture-check/src/main.rs                   # thin dispatch only if a named profile lacks it

docs/
└── MECHANICAL-SCAFFOLDING-REGISTER.md          # R2 receipts under MSC-8C-001…010

specs/
└── README.md                                   # R2 status only
```

Shared homes in `engine-core`, `game-stdlib`, and `game-test-support` are built
on, not rebuilt. A defect in a shared helper is not silently fixed inside a
game ticket; it invokes the failing-test protocol and a separately admitted
shared-owner task.

### 4.2 Characterization report

`reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` must
exist before the first production migration and contain:

1. exact commit and repository baseline;
2. a complete game × helper × sub-surface verdict table;
3. exact owner path and symbol for each surface;
4. existing trace/fixture filenames and stable hashes;
5. current seat spellings and import routes;
6. current action-tree legacy hash, if any, plus v1 candidate vectors;
7. state/effect/view/replay/export/diagnostic hash classifications;
8. C-01 public/private envelope payload, order, visibility, and stable-string
   baselines;
9. C-03 accepted/rejected seat counts and exact diagnostics;
10. C-07 source × viewer × surface matrices, phase setup, canary construction,
    expected presence/absence, and proof that canaries are never committed;
11. C-08 profile metadata, visibility class, validator owner,
    canonical-byte-authority claim, and N/A rationale;
12. C-09 fixed RNG words, rejection counts, indices, shuffled/dealt vectors,
    and downstream hashes;
13. accepted-exception rows with owner, compatibility window, rollback, and
    next trigger;
14. before/after evidence and ADR-0009 class for every migration; and
15. a final changed-artifact inventory proving that no unauthorized golden or
    fixture changed.

### 4.3 Register receipts

Append R2 receipt tables beneath existing entries, without creating competing
C-01…C-10 entries:

- `MSC-8C-001`: six constructor migrations plus two private-constructor N/As;
- `MSC-8C-002`: four parser migrations, landed import adapters, canonical
  outputs, and legacy roster exceptions;
- `MSC-8C-003`: four exact-two-seat migrations and ring N/A;
- `MSC-8C-004`/`005`: four action-tree v1 adoptions and adjacent exceptions;
- `MSC-8C-006`: High Card receipt plus three dev-only additions and reverse-dep proof;
- `MSC-8C-007`: High Card pilot verification plus three new pairwise matrices;
- `MSC-8C-008`: every profile verdict, including seat-private and domain N/As;
- `MSC-8C-009`: three RNG migrations plus Secret Draft N/A;
- `MSC-8C-010`: four non-promotion affirmations.

Every row names decision state, evidence, byte/hash/visibility impact, rollback,
and next review trigger.

## 5. Work breakdown

Every item below is a candidate `AGENT-TASK` with task profile
`scaffold-refactor`. Ticket decomposition may rename IDs or split a row when a
single selected surface still cannot fit one reviewable diff. It may not merge
games or surfaces.

### 5.1 Mandatory ten-point protocol for every migration task

Each task must state:

1. accepted authority (`MSC-8C-*`, ADR 0008, ADR 0009, and ADR 0004 where hidden information is involved);
2. exact owner paths and symbols;
3. every affected hash, trace, fixture, seat spelling, visibility, export, and RNG-consumption surface, including explicit N/A rows;
4. characterization tests and receipts captured before modification;
5. one selected surface per reviewable diff;
6. ADR-0009 classification: `unchanged`, `parallel-new-surface`, or `intentional-migration`;
7. compatibility window and legacy reader/authority;
8. exact before/after commands and artifact/hash comparison;
9. rollback that removes only the selected surface; and
10. the `AGENT-DISCIPLINE.md` failing-test protocol: validate the failing test, identify SUT versus test fault, fix the correct owner, and never weaken/delete tests merely to get green.

### 5.2 Wave A — admission and characterization

| Task | Exact target | Affected surfaces | Required evidence and rollback |
|---|---|---|---|
| `8C-R2-001` | `specs/README.md`, parent/R1 specs, atlas, register, ADRs; create characterization report shell | determination, authority, complete inventory | Freeze the four-game set and all verdict slots. Record the High Card C-04/C-07 correction. Evidence-only rollback removes only the unaccepted report draft. |
| `8C-R2-002` | all four `Cargo.toml`, `src/{effects,ids,setup,replay_support,visibility}.rs`, `tests/{replay,visibility,bots,serialization}.rs`, all named golden traces and fixtures | every byte/hash/seat/visibility/profile/RNG surface | Populate full before-state digests, matrices, and exception rows. No production change. Any uncharacterized surface blocks its later task. |

### 5.3 Wave B — C-01 envelope constructors

| Task | Exact files/symbols | Hash/visibility surface | One-surface rollback |
|---|---|---|---|
| `8C-R2-101` | `games/high_card_duel/src/effects.rs::public_effect` and focused tests | public effect stable strings/hash/order; observer and both seats | Restore only local public literal constructor. |
| `8C-R2-102` | `games/high_card_duel/src/effects.rs::private_effect` | private owner `SeatId`, filtered effects, private diagnostics/deal/commit, effect hash | Restore only local private literal constructor. |
| `8C-R2-103` | `games/secret_draft/src/effects.rs::public_effect` | public redacted payloads, effect hash/export; no private constructor added | Restore only local public constructor. |
| `8C-R2-104` | `games/poker_lite/src/effects.rs::public_effect` | public effects and reveal-safe stable strings | Restore only public constructor. |
| `8C-R2-105` | `games/poker_lite/src/effects.rs::private_effect` | private setup/bot effects, owner filtering, effect/export hashes | Restore only private constructor. |
| `8C-R2-106` | `games/masked_claims/src/effects.rs::public_effect` | claim/reaction public effects and redacted payloads | Restore only public constructor. |

Each task must prove serialized/stable effect output unchanged. Secret Draft
and Masked Claims private-constructor N/A decisions land as report/register
receipts, not synthetic code.

### 5.4 Wave C — C-02 parser adoption and compatibility audit

| Task | Exact files/symbols | Affected surface | Rollback |
|---|---|---|---|
| `8C-R2-201` | `games/high_card_duel/src/ids.rs::HighCardDuelSeat::parse` | canonical accept/reject set only | Restore manual canonical match. |
| `8C-R2-202` | `games/secret_draft/src/ids.rs::SecretDraftSeat::parse` | same | Restore manual match. |
| `8C-R2-203` | `games/poker_lite/src/ids.rs::PokerLiteSeat::parse` | same | Restore manual match. |
| `8C-R2-204` | `games/masked_claims/src/ids.rs::MaskedClaimsSeat::parse` | same | Restore manual match. |
| `8C-R2-205` | `crates/wasm-api/src/seats.rs`; `games/{high_card,secret,poker,masked}.rs`; WASM seat tests/snapshots | verify alias import, canonical Masked output, and legacy HCD/Secret/Poker roster exception; no output flip by default | Evidence-only. If reassessment admits a runtime-roster migration, split it into a separate game-specific task with state/effect/view/hash compatibility; never hide it here. |

Parser tasks must add canonical, out-of-range, leading-zero, alias,
lookalike, and role-label tests. No trace/golden change is authorized.

### 5.5 Wave D — C-03 exact-two-seat structure

| Task | Exact files/symbols | Affected surface | Rollback |
|---|---|---|---|
| `8C-R2-301` | `games/high_card_duel/src/setup.rs::setup_match`; existing `game-stdlib` dependency | accepted/rejected counts, variant expected count, diagnostics, setup/RNG/state hashes | Restore local predicate only. |
| `8C-R2-302` | `games/secret_draft/Cargo.toml`; `src/setup.rs::setup_match` | same, plus normal dependency edge | Restore predicate and remove only added normal dependency. |
| `8C-R2-303` | `games/poker_lite/Cargo.toml`; `src/setup.rs::setup_match` | same | Same isolated rollback. |
| `8C-R2-304` | `games/masked_claims/Cargo.toml`; `src/setup.rs::setup_match` | same | Same isolated rollback. |

Diagnostics and game-owned expected-count policy must remain byte-identical.
No ring helper adoption is permitted.

### 5.6 Wave E — C-04/C-05 parallel action-tree v1

| Task | Exact files/symbols | Selected and adjacent surfaces | Rollback |
|---|---|---|---|
| `8C-R2-401` | `games/high_card_duel/src/replay_support.rs`; `legal_action_tree`; replay/serialization tests | add parallel v1 bytes/hash for representative commit states; preserve C-07 debug snapshots and all state/effect/export hashes | Remove only new v1 adapter/tests. |
| `8C-R2-402` | `games/secret_draft/src/replay_support.rs::action_tree_hash`; replay tests | preserve legacy hash; add v1 bytes/hash for first-commit and pending-second-commit trees; no reveal-policy change | Remove parallel adapter/tests, legacy remains. |
| `8C-R2-403` | `games/poker_lite/src/replay_support.rs::action_tree_hash`; replay tests | preserve legacy hash; add v1 vectors across pledge phases; no pledge legality change | Same. |
| `8C-R2-404` | `games/masked_claims/src/replay_support.rs`, `src/actions.rs::legal_action_tree`; replay/rule tests | v1 vectors for compound claim and pending-response trees; response authorization remains game-local | Remove only v1 adapter/tests. |

All four are `parallel-new-surface` unless reassessment produces exact evidence
for a different classification. No existing golden trace changes.

### 5.7 Wave F — C-06 dev-only dependency discipline

| Task | Exact file | Surface | Rollback |
|---|---|---|---|
| `8C-R2-501` | `games/secret_draft/Cargo.toml` | add `game-test-support` under `[dev-dependencies]` only | Remove dev dependency and dependent tests. |
| `8C-R2-502` | `games/poker_lite/Cargo.toml` | same | Same. |
| `8C-R2-503` | `games/masked_claims/Cargo.toml` | same | Same. |

High Card is receipt-only. Every task runs the normal-edge inverse tree check
before and after. No tool, game library, WASM, or production target may gain a
normal/build edge.

### 5.8 Wave G — dedicated C-07 no-leak work

| Task | Exact files/symbols | Matrix/surfaces | Rollback |
|---|---|---|---|
| `8C-R2-510` | `games/high_card_duel/tests/visibility.rs`; `tests/bots.rs`; `MSC-8C-007` receipt | verify observer/seat0/seat1 × view/action/diagnostic/effect/public export/bot input, plus retained reveal-specific tests | Evidence-only; no pilot reconstruction. |
| `8C-R2-511` | `games/secret_draft/tests/visibility.rs`, `tests/bots.rs`, `tests/replay.rs`; `game_test_support::no_leak` | both source seats × observer/owner/opponent × view/action/diagnostic/effect/public export/seat-private export/bot surfaces, pre- and post-synchronized reveal | Remove only generic matrix test; all specific tests remain. |
| `8C-R2-512` | equivalent Poker files | private hand, center reveal, showdown, yield, public/seat-private export, bot surfaces | Same. |
| `8C-R2-513` | equivalent Masked files | hand, pending claim, accepted-secret resolution, challenged reveal, responder action tree, public export, bot surfaces | Same. |

Canaries are constructed in memory and never written to any trace, fixture,
snapshot, log, test ID, or browser artifact.

### 5.9 Wave H — C-08 replay-command profile drivers

| Task | Exact files/symbols | Profile/bytes | Rollback |
|---|---|---|---|
| `8C-R2-601` | HCD `Cargo.toml`, `tests/replay.rs`, `src/replay_support.rs`; `ReplayCommandV1Driver` | `internal-dev`; current internal trace validator/bytes | Remove driver test only. |
| `8C-R2-602` | Secret equivalent | same; private command authority remains internal only | Same. |
| `8C-R2-603` | Poker equivalent | same | Same. |
| `8C-R2-604` | Masked `tests/replay.rs`/rule replay builder and `src/replay_support.rs` | profile existing command/replay evidence without inventing an omniscient export | Same. |

A missing thin `replay-check` dispatch is split into a separate profile/game
task during reassessment. Game behavior never moves into the tool.

### 5.10 Wave I — C-08 setup-evidence profile drivers

| Task | Exact files | Profile/visibility | Rollback |
|---|---|---|---|
| `8C-R2-611` | HCD fixture, setup tests, `SetupEvidenceV1Driver` | public metadata plus internal-dev private-deal assertions; no fixture rewrite | Remove profile test. |
| `8C-R2-612` | Secret fixture/setup tests | setup parameters/pool metadata; commitments empty; no reveal behavior in data | Same. |
| `8C-R2-613` | Poker fixture/setup tests | setup/deck metadata; private dealt cards remain internal test evidence | Same. |
| `8C-R2-614` | Masked fixture/setup tests | mask ordering/status metadata; no selectors or reaction policy | Same. |

A missing thin `fixture-check` dispatch is separately bounded. All fixture
bytes are read-only by default.

### 5.11 Wave J — C-08 public-export profile drivers

| Task | Exact files/symbols | Profile/visibility/hash | Rollback |
|---|---|---|---|
| `8C-R2-621` | HCD `export_public_observer_replay`, replay/visibility tests | `public-export-v1`, observer only; current export bytes/hash | Remove driver test. |
| `8C-R2-622` | Secret `export_public_replay` with observer | public profile, pre-reveal path/seed redaction | Same. |
| `8C-R2-623` | Poker `export_public_replay` with observer | public profile, showdown/yield policy preserved | Same. |
| `8C-R2-624` | Masked `PublicReplayExport`, observer import path | public profile, claim tile redaction | Same. |

### 5.12 Wave K — C-08 seat-private export drivers

| Task | Exact files/symbols | Profile/visibility/hash | Rollback |
|---|---|---|---|
| `8C-R2-631` | Secret `export_public_replay` invoked with `Viewer(seat_0)` and `Viewer(seat_1)`; replay/visibility tests | `seat-private-export-v1`; pre-reveal choice remains absent even for owner; viewer label explicit | Remove seat-private profile tests only. |
| `8C-R2-632` | Poker equivalent | own private crest present, opponent absent; showdown/yield phase rules unchanged | Same. |

High Card and Masked Claims N/A rows land in the report/register. Do not create
a new seat-private exporter merely to make a profile apply.

### 5.13 Wave L — C-09 local unbiased-index replacement

| Task | Exact files/symbols | RNG/hash/visibility surface | Rollback |
|---|---|---|---|
| `8C-R2-701` | `games/high_card_duel/src/setup.rs::{shuffle_deck,next_bounded_index_unbiased}` | fixed words, rejection counts, full shuffled deck/deal, private effects/views, state/replay/export hashes | Restore local helper and call only. |
| `8C-R2-702` | `games/poker_lite/src/setup.rs::{shuffle_deck,next_bounded_index_unbiased}` | same, including private hands and showdown/yield traces | Same. |
| `8C-R2-703` | `games/masked_claims/src/setup.rs::{shuffle_masks,next_bounded_index_unbiased}` | same, including hands/reserve, pending claim and export redaction | Same. |

Replace only the local call with
`DeterministicRng::next_index_unbiased_v1`. Do not alter loop bounds, shuffle
order, deal order, seed handling, or game policy. `secret_draft` is explicit
N/A.

### 5.14 Wave M — consolidation, register, and status closeout

| Task | Exact targets | Required result | Rollback |
|---|---|---|---|
| `8C-R2-801` | characterization report and all touched tests/artifacts | reconcile every matrix cell, hash/visibility surface, exception, N/A, and before/after result; no unowned cleanup | Revert report consolidation only. |
| `8C-R2-802` | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | append complete R2 receipt tables under existing `MSC-8C-001…010`; C-10 explicitly rejects behavioral promotion | Revert only R2 receipt rows. |
| `8C-R2-803` | full §7 command set and changed-file audit | all focused and workspace evidence green; zero unauthorized golden/fixture diffs; classify any failure before fixing | Revert offending selected task, not unrelated work. |
| `8C-R2-804` | `specs/README.md` | after all exit criteria, flip only `8C-R2` to `Done`, record date/outcome; leave `8C-R3`, `8C-R4`, Gate 18 untouched | Restore only R2 tracker row. |

## 6. Exit criteria

The unit is `Done` only when every row passes. Silence is never a verdict.

| Exit ID | Obligation | Pass condition / parent mapping |
|---|---|---|
| `R2-EC-01` | Locked determination | `8C-R1` is confirmed `Done`, `8C-R2` remains the selected lowest non-`Done` unit, and no successor work entered scope. |
| `R2-EC-02` | Exact game ownership | Exactly the four locked games appear, matching the parent seed's R2 slice of EC-28. |
| `R2-EC-03` | Complete verdict coverage | Every game × C-01…C-08 cell and every listed sub-surface has one accepted verdict, evidence owner, rollback, and next trigger. |
| `R2-EC-04` | Correct ownership | Kernel mechanics remain generic; seat structure lives in `game-stdlib`; test geometry/profiles in `game-test-support`; aliases in `wasm-api`; behavior in games. |
| `R2-EC-05` | C-01 public neutrality | Four public constructor migrations preserve payload, order, bytes/hash, and visibility. |
| `R2-EC-06` | C-01 private neutrality | High Card and Poker private constructor migrations preserve owner IDs/filtering; Secret and Masked private N/As are evidenced and do not weaken no-leak proof. |
| `R2-EC-07` | C-02 grammar | Four game parsers use canonical Rust authority; aliases remain import-only; malformed/ambiguous/out-of-range labels are rejected. |
| `R2-EC-08` | C-02 compatibility | Legacy HCD/Secret/Poker runtime roster spellings remain an explicit exception; no silent state/effect/hash migration occurs. |
| `R2-EC-09` | C-03 structure | All four exact-two-seat checks use structural `SeatCount` while exact diagnostics, variant expectations, setup state, and behavior remain unchanged. |
| `R2-EC-10` | C-04 tree coverage | V1 vectors cover HCD commit, Secret pending commit, Poker pledge phases, and Masked compound claim/pending response trees without changing legal choices. |
| `R2-EC-11` | C-05 byte discipline | Selected action-tree v1 framing is explicit/versioned; all adjacent state/effect/view/replay/export/diagnostic surfaces are unchanged or accepted-excepted. |
| `R2-EC-12` | Hash migration discipline | Every affected surface is `unchanged`, `parallel-new-surface`, or separately authorized intentional migration; no broad regeneration occurs. |
| `R2-EC-13` | Legacy readability | Existing traces, fixtures, local Secret/Poker action hashes, and import paths remain valid during their stated compatibility windows. |
| `R2-EC-14` | Dev-only boundary | Secret, Poker, and Masked list `game-test-support` only as dev dependency; inverse normal-edge proof and boundary script pass. |
| `R2-EC-15` | Geometry-only harness | No-leak harness only enumerates caller-supplied cases and cannot project, redact, authorize, reveal, or execute rules. |
| `R2-EC-16` | High Card residual rule | Existing C-07 pilot receipt is verified, not rebuilt; C-04 is independently migrated as corrected in the preamble. |
| `R2-EC-17` | Secret Draft no-leak | Both sources × three viewers × all applicable surfaces pass before and after synchronized reveal; private command trace remains internal-dev. |
| `R2-EC-18` | Poker Lite no-leak | Own/opponent/public hand access, showdown reveal, yield non-reveal, effects, exports, and bot surfaces pass. |
| `R2-EC-19` | Masked Claims no-leak | Pending claim, responder tree, accepted-secret state, challenge reveal, export, and bot surfaces pass without changing reaction policy. |
| `R2-EC-20` | Canary hygiene | No canary appears in committed trace, fixture, export, log, snapshot, DOM/test ID, storage, or accessibility artifact. |
| `R2-EC-21` | C-08 separation | Distinct typed profile drivers reject wrong IDs, owners, visibility classes, and fields; no permissive union is introduced. |
| `R2-EC-22` | C-08 real callers | All four replay/setup/public profiles invoke real game evidence; Secret/Poker seat-private profiles invoke both viewers; HCD/Masked seat-private and all domain N/As are explicit. |
| `R2-EC-23` | ADR-0004 preservation | Public exports omit all disallowed private facts; seat-private exports are explicitly viewer-labelled and never reconstruct omniscient state. |
| `R2-EC-24` | Fixture/data boundary | Fixtures remain typed data/evidence only; no selectors, triggers, conditions, formulas, or procedural reveal/reaction behavior enters data. |
| `R2-EC-25` | Tool ownership | `replay-check`/`fixture-check` remain validator owners with thin dispatch only; no game behavior moves into tools. |
| `R2-EC-26` | C-09 identity | HCD, Poker, and Masked shared sampler adoptions preserve outputs, rejection draw counts, shuffle/deal vectors, and every downstream hash/visibility surface; Secret is evidenced N/A. |
| `R2-EC-27` | C-10 / noun-free boundary | Reveal/reaction/projection/pledge/pot/scoring/outcome remain local; `engine-core` remains noun-free; boundary checks pass. |
| `R2-EC-28` | Full test health | Focused and workspace suites pass without deleted, ignored, or weakened tests and without benchmark-threshold drift. |
| `R2-EC-29` | C-11 ownership truth | R2 is closed as the second bounded parent EC-28 wave; R3/R4 remain separately owned and unimplemented. |
| `R2-EC-30` | Gate 18 sequencing | Consistent with parent EC-30, Gate 18 remains blocked behind R3/R4 closure/N-A/accepted exception; only the R2 row changes status. |

## 7. Acceptance evidence

### 7.1 Required command set

Run from repository root. Record command, exit status, relevant output summary,
and changed-artifact inventory in the characterization report.

```bash
cargo fmt --all -- --check

cargo test -p engine-core
cargo test -p game-stdlib
cargo test -p game-test-support
cargo test -p wasm-api
cargo test -p high_card_duel
cargo test -p secret_draft
cargo test -p poker_lite
cargo test -p masked_claims

cargo test -p replay-check
cargo test -p fixture-check
cargo test --workspace --all-targets

cargo run -p replay-check -- --game high_card_duel --all
cargo run -p replay-check -- --game secret_draft --all
cargo run -p replay-check -- --game poker_lite --all
cargo run -p replay-check -- --game masked_claims --all

cargo run -p fixture-check -- --game high_card_duel
cargo run -p fixture-check -- --game secret_draft
cargo run -p fixture-check -- --game poker_lite
cargo run -p fixture-check -- --game masked_claims

bash scripts/boundary-check.sh
cargo tree --workspace -e normal --invert game-test-support

node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
```

`check-catalog-docs.mjs` is a regression guard only; R2 adds no game or web
surface.

### 7.2 Focused evidence by migration class

| Class | Exact evidence |
|---|---|
| C-01 public/private | Before/after envelope debug/stable strings, effect hashes, order, filtered payloads for observer/seat0/seat1, and no private recipient drift. |
| C-02 parser | Canonical acceptance and strict rejection vectors; WASM alias acceptance remains; no TypeScript normalization; no existing trace/golden diff. |
| C-03 count | Accepted count 2; rejected 0/1/3 where callable; exact diagnostic code/message; setup/state/replay/view equality. |
| C-04/C-05 | Exact v1 bytes and hashes for named trees; legacy Secret/Poker hash equality; no legal-choice/metadata/preview change. |
| C-06 | Cargo manifests plus inverse normal-edge output before/after. |
| C-07 | Matrix dimensions, expected cases, zero structured failures, retained focused reveal tests, and in-memory-only canary proof. |
| C-08 | Valid metadata, wrong-profile/owner/visibility/field rejection, real validator invocation, existing artifact byte/hash equality, explicit N/As. |
| C-09 | Fixed-word and rejection vectors, draw counts, shuffled/dealt sequences, state/effect/view/replay/export equality. |
| C-10 | Register/atlas review proving behavior stays game-owned. |

### 7.3 Characterization anchors

At admission, re-pin at minimum:

- High Card: `hidden-info-public-observer`, `seat-private-view`,
  `public-replay-export-import`, invalid-private-card redaction, effect stable
  strings, setup shuffle/deal, and the existing C-07 matrix.
- Secret Draft: first-commit pending, seat-private no-prereveal choice,
  simultaneous-reveal batch, public-observer no-leak,
  public-replay-export-import, and committed command trace authority.
- Poker Lite: deal-private no-leak, seat-private view, public observer,
  public replay export/import, showdown reveal, yield no-showdown, private
  diagnostic, and setup shuffle/deal.
- Masked Claims: claim-pending window, public-observer no-leak, accepted mask
  never revealed, challenge reveal paths, public replay export/import, wrong
  responder diagnostics, and setup shuffle/deal.

The report must name exact filenames and current hashes from the repository,
not copy values from this planning artifact.

### 7.4 Golden, fixture, and diff policy

**Default authorized changes to existing golden traces or fixtures: none.**

Authorized additions are focused Rust tests, profile metadata adapters, and the
characterization/register receipts. The planned constructor, parser,
seat-count, parallel action-tree, no-leak, profile, and C-09 migrations are
expected to preserve every existing committed artifact byte.

An existing trace/fixture/export byte may change only if `/reassess-spec`
adds a separately named row before implementation that states:

1. exact artifact and owner;
2. why unchanged/parallel treatment is impossible;
3. ADR-0009 intentional-migration classification;
4. old and new versions/hashes;
5. non-empty migration note;
6. compatibility reader/window;
7. ADR-0004 visibility proof when applicable;
8. exact validator commands; and
9. one-artifact rollback.

“Regenerate all”, “update snapshots”, and opportunistic reformatting are
forbidden. An unexpected diff is a stop condition, not an invitation to bless
new bytes.

### 7.5 Register before/after and reviews

Before the first migration, capture the existing `MSC-8C-001…010` entries and
R1 receipt tables. At closeout, show only appended R2 receipts and prove no
existing decision was weakened. Required human review areas:

- hidden-information taxonomy and every seat-private/public export row;
- High Card C-04/C-07 correction;
- legacy WASM roster exceptions;
- all C-09 draw-count identity evidence;
- all N/A and exception rationales;
- changed-file inventory and zero unauthorized golden/fixture diffs.

### 7.6 External-research sharpening

External sources do not establish repository state; they only reinforce the
accepted local doctrine:

- Cargo documents that dev-dependencies are used for tests/examples/benchmarks,
  not ordinary package builds, supporting the C-06 manifest plus inverse-tree
  enforcement rather than convention alone.[^ext-cargo-dev]
- RFC 8785 notes that repeatable hashing requires an invariant
  representation, reinforcing explicit versioned action-tree bytes rather
  than “whatever current serialization emits”.[^ext-rfc8785]
- Protocol Buffers' own documentation warns that deterministic serialization
  is not necessarily canonical, reinforcing Rulepath's separate canonical
  byte authority, compatibility, and version policy.[^ext-protobuf]

These sources do not widen scope or authorize a new serialization format.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Principles engaged

| Authority | R2 stance |
|---|---|
| `FOUNDATIONS.md` product priority | Correctness, deterministic replay, and no-leak proof outrank deduplication. A helper adoption is rejected if identity cannot be proved. |
| `FOUNDATIONS.md` §11 hidden information | Every private datum is tested across public, owner, opponent, replay/export, effect, diagnostic, bot-input, and explanation surfaces. |
| `FOUNDATIONS.md` §11 determinism | Byte order, hashes, RNG draw counts, trace/export bytes, and seat spellings are explicit evidence surfaces. |
| `FOUNDATIONS.md` §12 stop conditions | Any leak, unexpected artifact diff, noun contamination, hidden normal dependency, or behavior movement stops the affected task. |
| `FOUNDATIONS.md` §13 | A missing hidden-info class, schema change, cross-owner behavior, or incompatible replay/hash change is a blocking ADR trigger. |
| `ARCHITECTURE.md` | Rust remains sole behavior authority; narrowest lawful owner wins; TS presents only. |
| `ENGINE-GAME-DATA-BOUNDARY.md` | Shared code carries mechanics/geometry/metadata only; games retain rules and projection. |
| ADR 0004 | Internal-full, public observer, and explicit seat-private evidence remain distinct; public artifacts never gain private facts. |
| ADR 0008/register | Register-first, behavior-free extraction only. |
| ADR 0009 | Per-surface characterization, classification, compatibility, evidence, and rollback. |
| Multi-seat/surface contract | Fixed-two-seat does not waive viewer-pair or surface enumeration. |
| Agent discipline | Bounded tasks, one surface per diff, valid-test/SUT diagnosis, no weakening. |

### 8.2 Ownership decisions

- `engine-core`: reuse generic envelope, stable writer/action-tree, replay/hash,
  and unbiased-index APIs; add no game noun or policy.
- `game-stdlib`: structural `SeatCount` only; no turn/reaction order policy.
- `game-test-support`: dev-only pairwise enumeration and profile metadata.
- `games/*`: all setup expectations, private payload creation, projection,
  reveal timing, reaction authorization, pledge/pot logic, scoring/outcome,
  bot inputs, and evidence adapters.
- `wasm-api`: import-only legacy seat aliases and transport compatibility.
- tools: validation/dispatch only.
- static data: typed parameters, metadata, fixtures, and expected evidence only.

### 8.3 Hidden-information and non-promotion stance

The defining safety rule is stronger than “tests pass”: no scaffolding helper
may decide a hidden-information fact. Specifically:

- Secret Draft alone decides when simultaneous choices reveal and how the
  visible pool changes.
- Poker Lite alone decides pledge rounds, showdown visibility, yield
  non-reveal, and shared-pool allocation.
- Masked Claims alone decides pending response, eligible responder, claim-path
  redaction, accepted-mask secrecy, challenge reveal, and outcome.
- High Card Duel alone decides deal, commit, reveal, refill, and scoring.
- Each game alone supplies C-07 expectations and C-08 export construction.

### 8.4 Blocking ADR-trigger note

No foundation or ADR amendment is expected. Stop the affected implementation
and flag a maintainer decision under `FOUNDATIONS.md` §13 if reassessment finds
any of the following:

- ADR 0004 cannot classify an actual official export without ambiguity;
- a required migration changes public/seat-private/internal authority;
- a canonical byte change cannot preserve a compatibility reader/window;
- a proposed helper must infer reveal/reaction/projection policy;
- a shared owner needs a game noun or semantic rule;
- a profile requires executable fixture behavior; or
- a real output-seat migration cannot be isolated from hidden visibility/hash
  state.

The spec must not design around such a gap or silently amend doctrine.

## 9. Forbidden changes

- Do not add, remove, or substitute a game.
- Do not modify `8C-R3`, `8C-R4`, or Gate 18 status/scope.
- Do not reimplement landed C-01…C-10 infrastructure or High Card's C-07 pilot.
- Do not claim High Card C-04 was already discharged.
- Do not move commitment/reveal, reaction, pending-response, pledge/pot,
  projection/redaction, scoring, or outcome policy into shared code.
- Do not add a private effect/export merely to make a profile applicable.
- Do not change the WASM runtime roster in a parser task.
- Do not reinterpret legacy traces with v1 bytes.
- Do not change RNG algorithm, draw count, loop order, shuffle/deal order, or
  seed meaning.
- Do not permit hidden data in payloads, DOM, storage, logs, effects, bot
  explanations, rankings, traces, fixtures, or public exports.
- Do not commit test canaries.
- Do not make `game-test-support` a production, normal, build, WASM, browser,
  or tool dependency.
- Do not put nouns or rule policy in `engine-core`.
- Do not use YAML or add a DSL.
- Do not let TypeScript decide legality or normalize seats.
- Do not delete, ignore, broaden away, or weaken tests.
- Do not mass-update goldens/fixtures.
- Do not relax benchmarks or CI thresholds to close the unit.

## 10. Documentation updates required

| Document | Required? | Required change |
|---|---:|---|
| `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` | yes | Create complete before/after verdict, bytes/hash, seat, visibility, profile, RNG, exception, and diff evidence. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes | Append R2 receipts under existing `MSC-8C-001…010`; do not create a rival register or alter accepted law. |
| `specs/README.md` | yes | `Not started` → `Planned` when the accepted spec lands; later `Planned/In progress` → `Done` only after all exit criteria. Change only R2. |
| Rust comments/tests | as needed | Short version/compatibility comments where parallel authority would otherwise be ambiguous. |
| `apps/web/README.md` | **not applicable** | No game, catalog, renderer, smoke layer, or web-shell surface is added or changed. |
| Game rules/mechanics/UI/AI docs | **not applicable** | Behavior, mechanics, presentation, and bot strategy are unchanged. |
| `docs/FOUNDATIONS.md`, architecture/boundary docs, ADRs | no by default | Any genuine gap is the blocking §8.4 ADR trigger, not routine cleanup. |
| `docs/MECHANIC-ATLAS.md` | no | No mechanic promotion or debt decision changes; C-10 only reaffirms local ownership. |
| `docs/ROADMAP.md` | no | The active tracker and parent already own sequencing. |

## 11. Sequencing

### 11.1 External sequence

- **Predecessor:** `8C-R1` is `Done`; its direct structural precedent and
  landed C-01…C-10 infrastructure are prerequisites.
- **Current:** `8C-R2` only.
- **Successors:** `8C-R3`, then `8C-R4`, then Gate 18 according to the active
  tracker.
- **Gate 18:** remains blocked under parent EC-30 until the successor C-11
  waves are closed, explicitly N/A, or accepted-excepted.

### 11.2 Internal dependency order

1. Wave A freezes all surfaces, bytes, hashes, visibility matrices, and RNG
   vectors.
2. C-01, parser-only C-02, and C-03 tasks may proceed independently after
   their game's characterization.
3. C-04/C-05 tasks remain parallel-new and independent of profile work.
4. C-06 dev dependencies land before C-07/C-08 tests in Secret, Poker, and
   Masked.
5. C-07 tasks land before final public/seat-private profile closeout so the
   same viewer expectations are reused, not redefined.
6. Replay-command and setup profiles may proceed independently per game.
7. Public-export profiles precede seat-private profiles in Secret/Poker.
8. C-09 tasks require all before-state RNG and downstream hash evidence and
   must not share diffs with C-01/C-03/C-08.
9. Consolidation and register receipts follow every migration/N-A/exception.
10. Full acceptance runs before the R2 status flip; status is last.

### 11.3 Admission rule for a diff

A diff is admitted only when it names one matrix surface, points to its
before-state receipt, changes no unrelated surface, preserves or explicitly
versions compatibility, and can be reverted without reverting another
migration. “All four games”, “cleanup”, “normalize seats”, or “update
snapshots” diffs are inadmissible.

## 12. Assumptions

### 12.1 One-line-correctable assumptions

- `assumption:` the unit slug/label defaults to
  `8c-r2-two-seat-hidden-reaction-scaffolding`; fixed unit ID is `8C-R2`.
- `assumption:` the eventual accepted path is
  `specs/8c-r2-two-seat-hidden-reaction-scaffolding.md`; this intermediate file
  is the pre-decomposition artifact for `/reassess-spec` → `/spec-to-tickets`.
- `assumption:` the owner remains “Rulepath maintainers” until reassessment
  assigns named ticket owners.
- `assumption:` all four games are fixed-two-seat; deeper evidence changing
  one to a range corrects only that game's C-03 row without expanding scope.
- `assumption:` task IDs are planning identifiers and may be renamed or split,
  never merged across selected surfaces.
- `assumption:` the shipped seat-private envelope API is
  `EffectEnvelope::private_to`; exact local wrapper names are one-line
  correctable.
- `assumption:` HCD/Secret/Poker WASM runtime rosters remain accepted legacy
  exceptions in R2; a contrary decision requires a separately characterized
  output/state/visibility migration.
- `assumption:` existing traces and fixtures are read-only by default; profile
  drivers wrap existing validators and artifacts.
- `assumption:` Secret and Poker's game-level viewer exporters are sufficient
  real callers for `seat-private-export-v1`; browser import remains
  observer-only and is not widened.
- `assumption:` HCD and Masked Claims have no official seat-private replay
  export at this commit; their private-view evidence does not count as an
  export.
- `assumption:` `domain-evidence-v1` is N/A for all four because the only data
  evidence is setup-shaped.
- `assumption:` C-04/C-05 add a parallel action-tree v1 surface; Secret/Poker
  legacy local hashes stay authoritative through the compatibility window.
- `assumption:` the three local C-09 algorithms are semantically identical to
  `next_index_unbiased_v1`; any fixed-vector/draw-count divergence blocks that
  game rather than authorizing a behavior change.
- `assumption:` no foundation/ADR amendment is required; any discovered need
  invokes §8.4.
- `assumption:` no thin tool dispatch is missing; if one is missing,
  reassessment creates one separate profile/game dispatch task rather than
  burying it in a game migration.

### 12.2 Reassessment corrections that do not reopen scope

`/reassess-spec` may correct a helper spelling, test-module owner, exact trace
list/hash, validator dispatch status, register row identifier, or whether one
existing exporter qualifies as an official profile caller. It may not add a
game, introduce a new exporter to force applicability, move behavior, merge
surfaces, authorize a broad byte migration, or absorb successor work.

### 12.3 Repository evidence basis

The principal repository authorities and seams are:

- authority/workflow:
  [`docs/README.md`](../docs/README.md),
  [`docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md),
  [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
  [`docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md),
  [`docs/ROADMAP.md`](../docs/ROADMAP.md),
  [`specs/README.md`](../specs/README.md);
- scaffolding/hidden-info/migration:
  [`docs/MECHANIC-ATLAS.md`](../docs/MECHANIC-ATLAS.md),
  [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`](../docs/MECHANICAL-SCAFFOLDING-REGISTER.md),
  [`docs/adr/0004-hidden-info-replay-export-taxonomy.md`](../docs/adr/0004-hidden-info-replay-export-taxonomy.md),
  [`docs/adr/0008-mechanical-scaffolding-governance.md`](../docs/adr/0008-mechanical-scaffolding-governance.md),
  [`docs/adr/0009-replay-fixture-hash-taxonomy.md`](../docs/adr/0009-replay-fixture-hash-taxonomy.md);
- evidence:
  [`docs/TESTING-REPLAY-BENCHMARKING.md`](../docs/TESTING-REPLAY-BENCHMARKING.md),
  [`docs/TRACE-SCHEMA-v1.md`](../docs/TRACE-SCHEMA-v1.md),
  [`docs/EVIDENCE-FIXTURE-CONTRACT.md`](../docs/EVIDENCE-FIXTURE-CONTRACT.md),
  [`docs/AI-BOTS.md`](../docs/AI-BOTS.md),
  [`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`](../docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md),
  [`docs/AGENT-DISCIPLINE.md`](../docs/AGENT-DISCIPLINE.md);
- parent/precedent:
  [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`](../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md),
  [`archive/specs/8c-r1-public-fixed-seat-scaffolding.md`](../archive/specs/8c-r1-public-fixed-seat-scaffolding.md),
  [`reports/8c-mechanical-scaffolding-characterization.md`](../reports/8c-mechanical-scaffolding-characterization.md),
  [`reports/8c-r1-public-fixed-seat-scaffolding-characterization.md`](../reports/8c-r1-public-fixed-seat-scaffolding-characterization.md);
- shared homes:
  [`crates/engine-core/src/action.rs`](../crates/engine-core/src/action.rs),
  [`crates/engine-core/src/replay.rs`](../crates/engine-core/src/replay.rs),
  [`crates/engine-core/src/rng.rs`](../crates/engine-core/src/rng.rs),
  [`crates/game-stdlib/src/seat.rs`](../crates/game-stdlib/src/seat.rs),
  [`crates/game-test-support/src/no_leak.rs`](../crates/game-test-support/src/no_leak.rs),
  [`crates/game-test-support/src/profiles.rs`](../crates/game-test-support/src/profiles.rs),
  [`crates/wasm-api/src/seats.rs`](../crates/wasm-api/src/seats.rs);
- validators/guards:
  [`tools/replay-check/src/main.rs`](../tools/replay-check/src/main.rs),
  [`tools/fixture-check/src/main.rs`](../tools/fixture-check/src/main.rs),
  [`scripts/boundary-check.sh`](../scripts/boundary-check.sh),
  [`scripts/check-doc-links.mjs`](../scripts/check-doc-links.mjs),
  [`scripts/check-catalog-docs.mjs`](../scripts/check-catalog-docs.mjs).

### 12.4 Final self-check before acceptance

The accepted form must answer **yes** to every question:

- Is R2 confirmed rather than re-decided?
- Are exactly four games present?
- Does every C-01…C-08 cell and sub-surface have a verdict?
- Are private envelopes, C-07, and actual seat-private exports treated as live?
- Is High Card C-07 pilot-discharge preserved while C-04 is correctly left to
  migrate?
- Are all reveal/reaction/projection/pledge/pot/scoring/outcome decisions local?
- Is every byte/hash/seat/visibility/RNG surface characterized and reversible?
- Are existing goldens/fixtures unchanged by default?
- Are HCD/Poker/Masked C-09 migrations explicit and Secret N/A?
- Is `game-test-support` dev-only?
- Are specific tests retained alongside generic matrices?
- Is `apps/web/README.md` explicitly not applicable?
- Is the artifact framed for reassessment and ticket decomposition?
- Do R3/R4 and Gate 18 remain untouched and sequenced after R2?

[^ext-cargo-dev]: Rust Project, *The Cargo Book — Development dependencies*, https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies
[^ext-rfc8785]: RFC Editor, *RFC 8785 — JSON Canonicalization Scheme*, https://www.rfc-editor.org/rfc/rfc8785.html
[^ext-protobuf]: Protocol Buffers Documentation, *Proto Serialization Is Not Canonical*, https://protobuf.dev/programming-guides/serialization-not-canonical/
