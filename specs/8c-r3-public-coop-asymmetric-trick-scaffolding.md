# Unit 8C-R3 — C-11 follow-on: public/co-op/asymmetric/trick scaffolding

| Field | Value |
|---|---|
| Spec ID | `8C-R3` |
| Artifact slug | `8c-r3-public-coop-asymmetric-trick-scaffolding` |
| Roadmap stage | Public scaling phase — C-11 follow-on retrofit lane |
| Roadmap build gate | `8C-R3` (precedes `8C-R4` and Gate 18) |
| Status | `Planned` |
| Date | 2026-06-24 |
| Owner | Rulepath maintainers; implementation delegated through bounded `AGENT-TASK` packets using profile `scaffold-refactor` |
| Analysis baseline | Grounded against the repository working tree at commit `be1af6f` (HEAD at authoring). |

> **Locked scope.** The unit (`8C-R3`) and its four-game set — `plain_tricks`,
> `flood_watch`, `frontier_control`, and `event_frontier` — are fixed.
> Reassessment and ticket decomposition may correct one-line implementation
> details but may not reopen the determination, add or drop a game, absorb
> `8C-R4`, or begin Gate 18.

This spec is subordinate to the foundation set indexed by
[`docs/README.md`][docs-readme]. Where this spec and a higher-authority document
disagree, the higher-authority document wins. Authority order:
[`docs/FOUNDATIONS.md`][foundations] →
[`docs/ARCHITECTURE.md`][architecture] →
[`docs/ENGINE-GAME-DATA-BOUNDARY.md`][boundary] → accepted ADRs and area
contracts → [`docs/ROADMAP.md`][roadmap] → this spec → tickets.

**Grounded correction to the research brief.** The brief correctly leaves C-09
verdicts to the code but suggests a default of `not applicable`. The code
contains a real local rejection-sampling bounded-index helper in three of
the four games:
`plain_tricks::setup::{shuffle_deck, next_bounded_index_unbiased}`,
`flood_watch::setup::{shuffle_event_deck, next_bounded_index_unbiased}`, and
`event_frontier::setup::{build_seeded_deck, shuffle_epoch,
next_bounded_index_unbiased}`. Those three are therefore C-09 `migrate`
candidates, subject to exact RNG-word, rejection-count, shuffle/deal/deck, and
downstream hash equality. `frontier_control` remains `not-applicable` because
its setup is explicitly RNG-free. This correction narrows the grounded matrix;
it does not reopen the locked unit or game set. See [Plain Tricks setup][pt-setup],
[Flood Watch setup][fw-setup], [Frontier Control setup][fc-setup], and
[Event Frontier setup][ef-setup].

A second grounding nuance affects C-02 without changing scope: only
`plain_tricks` owns a game-local typed seat parser to replace. The other three
games use generic `SeatId` values and already cross the import boundary through
the shared `wasm-api` alias adapter. Their C-02 rows are accepted exceptions
with fresh conformance receipts, not synthetic game-local parser work and not
pilot credit. Domain IDs such as `TrickCardId`, `DistrictId`, `SiteId`,
`EventKind`, and `FactionId` are outside C-02.

## 1. Determination

### 1.1 Locked determination

The next-unit determination is confirmed and documented, not re-decided:

1. [`specs/README.md`][spec-index] records `8C-R2` as `Done`, completed
   2026-06-23. `8C-R3` is the lowest active-epoch row whose status is not
   `Done`; its row names exactly `plain_tricks`, `flood_watch`,
   `frontier_control`, and `event_frontier`. `8C-R4` follows it, and Gate 18 is
   explicitly blocked behind closure, accepted not-applicability, or accepted
   exception for all `8C-R1…8C-R4` waves.
2. [`docs/MECHANIC-ATLAS.md`][atlas] §10A records `_None_` and “No open
   promotion debt remains.” No primitive-promotion debt must close first. This
   unit is a code-scaffolding retrofit, not the next mechanic-ladder gate.
3. [`archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md`][parent]
   §5 seeds the exact R3 game set and the audit emphasis: setup/profile
   diversity, action-tree/effect hashing, and fixture/profile surfaces while
   preserving behavioral atlas decisions. Parent work item `8C-030` created
   the four forward rows. Parent EC-28 assigns every official game exactly once
   across the bounded C-11 seeds; EC-30 keeps Gate 18 after those waves close,
   become explicitly not applicable, or receive accepted exceptions.
4. [`archive/specs/8c-r1-public-fixed-seat-scaffolding.md`][r1-spec] and
   [`archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md`][r2-spec]
   are the two direct structural precedents: characterization first, complete
   applicability and sub-surface tables, one selected surface per diff,
   ADR-0009 migration packets, register receipts, full command evidence, and a
   final tracker flip.
5. The parent pilot lists and the four Cargo manifests show that
   none of the R3 games was an 8C or C-08 pilot and none currently depends on
   `game-test-support`. R3 therefore has no pilot-credit verdict. Every
   applicable surface is a fresh audit; every non-applicable or retained
   surface receives its own R3 rationale and receipt. See [Plain Tricks Cargo][pt-cargo],
   [Flood Watch Cargo][fw-cargo], [Frontier Control Cargo][fc-cargo], and
   [Event Frontier Cargo][ef-cargo].

**Determination:** Unit `8C-R3` is the required current unit. Its bounded game
set is exactly:

- `plain_tricks` — two-seat trick-taking with real private hands;
- `flood_watch` — two-seat cooperative public-role play with a public forecast
  and hidden future event-deck tail;
- `frontier_control` — two-seat asymmetric, fully public graph play;
- `event_frontier` — two-seat asymmetric event play with public current/next
  cards and a hidden deeper deck tail.

No fifth game, `8C-R4` work, Gate 18 partnership work, new game, UI feature, or
new helper contract is admitted.

## 2. Objective

Within the C-11 retrofit lane established by the completed Unit 8C parent and
the public-scaling sequence in [`docs/ROADMAP.md`][roadmap], Unit 8C-R3 must
turn the public/cooperative/asymmetric/trick seed into a bounded, reversible
implementation plan. It must:

1. audit every official C-01…C-08 surface in the four locked games and resolve
   every aggregate cell and listed sub-surface to exactly one verdict:
   **migrate**, **not-applicable**, or **exception**;
2. adopt existing behavior-free envelope constructors, canonical seat grammar,
   structural seat-count helpers, canonical action-tree v1 bytes/hash,
   dev-only test support, no-leak enumeration geometry, and typed evidence
   profile metadata without adding or changing a shared helper contract;
3. center C-03 on the actual setup diversity: plain exact roster count,
   cooperative `role_order` cardinality, asymmetric faction/variant checks,
   and exact game-owned diagnostics—never a false uniform “two seats only”
   abstraction;
4. center C-08 on real `setup-evidence-v1` and `domain-evidence-v1` fixture
   callers for standard and variant setup, connectivity/round scoring,
   event/edict resolution, levee/inundation pressure, and budget/resource
   accounting while keeping all semantic validation in the games;
5. apply ADR 0004 directly to `plain_tricks` private hands and viewer-scoped
   exports, and as a public-observer regression guard for Flood Watch and Event
   Frontier; record Frontier Control's lack of a hidden source as explicit C-07
   not-applicability rather than inventing a canary;
6. add action-tree v1 as a parallel, versioned selected surface for each game's
   richer legal tree while preserving every existing local hash, trace, state,
   effect, view, diagnostic, replay, and export byte unless a separately named
   ADR-0009 intentional migration is admitted;
7. replace the three proven local unbiased-index implementations with
   `DeterministicRng::next_index_unbiased_v1` only after exact identity is
   characterized; preserve seed handling, RNG consumption, rejection counts,
   shuffle/deal order, and game policy;
8. preserve every trick, graph/topology, adjacency/movement/connectivity,
   event/edict, budget/resource, role/faction/team, reveal/projection/redaction,
   legality, scoring, and outcome decision in its current game owner; and
9. leave no unnamed “remaining cleanup” bucket.

The outcome is a characterized four-game plumbing retrofit, not a new game,
mechanic promotion, trick framework, graph library, event engine, budget
engine, scoring framework, replay redesign, fixture DSL, or browser feature.

## 3. Scope

### 3.1 In scope

- **Games:** exactly `plain_tricks`, `flood_watch`, `frontier_control`, and
  `event_frontier`.
- **Primary audit:** C-01…C-08, with every aggregate cell and every sub-surface
  below explicitly assigned.
- **C-01:** `EffectEnvelope::public` for all four and
  `EffectEnvelope::private_to` only for Plain Tricks' real private deal
  effects. Payload construction, effect order, recipient choice, reveal
  timing, and filtering remain game-owned.
- **C-02:** `SeatId::parse_canonical` for Plain Tricks' typed parser and fresh
  conformance evidence for the shared `wasm-api` import-only legacy adapter and
  canonical output paths. Non-seat IDs are explicitly excluded.
- **C-03:** `SeatCount::new` as behavior-free structural count evidence for
  roster and applicable variant/cardinality checks. `SeatCountRange`, ring
  rotation, role identity/order, faction identity/order, variant policy,
  setup composition, and exact diagnostics remain local.
- **C-04/C-05:** `ActionTreeEncodingVersion::V1`,
  `ActionTree::stable_bytes`, `ActionTree::stable_hash`, and the shipped
  `StableBytesWriter` framing as one parallel selected action-tree surface.
  Existing local action-tree hashes and every adjacent byte/hash surface are
  classified separately.
- **C-06:** a fresh `game-test-support` `[dev-dependencies]` edge for all four
  games, with no production/build reverse dependency.
- **C-07:** generic pairwise assertion geometry over caller-supplied facts,
  viewers, surfaces, and expectations: full seat-private coverage for Plain
  Tricks; hidden-future-deck/public-observer coverage for Flood Watch and Event
  Frontier; explicit N/A plus equality regression for Frontier Control.
- **C-08:** `replay-command-v1`, `setup-evidence-v1`,
  `domain-evidence-v1`, `public-export-v1`, and only the real
  `seat-private-export-v1` surface in Plain Tricks. Drivers validate profile
  metadata and delegate all semantics to existing game/tool adapters.
- **C-09 checkpoint:** three proven unbiased-index migrations; one explicit
  N/A.
- **C-10 checkpoint:** non-promotion affirmation and rejection of every
  behavior-bearing extraction proposal.
- **Governance:** one characterization report, append-only R3 receipts under
  existing `MSC-8C-001…010`, complete acceptance evidence, and the final
  `specs/README.md` status flip.

### 3.2 Verdict vocabulary

| Verdict | Meaning |
|---|---|
| `migrate` | One named selected surface changes in one reviewable diff under §5.1. The task has exact before/after evidence and isolated rollback. |
| `not-applicable` | No official surface of that class exists. The exact rationale, evidence owner, and next review trigger are recorded; no synthetic code or artifact is created to force applicability. |
| `exception` | A real lawful surface remains under its named current owner without migration. The receipt states compatibility, rollback or evidence-only reversal, and the next review trigger. |

There is no pilot-credit verdict in this unit. “Already shared”, “already
canonical”, or “covered by a prior game” does not discharge an R3 row; it is
classified as a fresh conformance exception or N/A with R3 evidence.

### 3.3 Primary applicability and verdict matrix

The aggregate matrix covers C-01…C-08. The sub-surface tables that follow are
authoritative where one helper family contains more than one verdict.

| Game | C-01 effects | C-02 seats | C-03 setup counts | C-04 tree v1 | C-05 writer v1 | C-06 dev support | C-07 no-leak | C-08 profiles |
|---|---|---|---|---|---|---|---|---|
| `plain_tricks` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `flood_watch` | `migrate` | `exception` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |
| `frontier_control` | `migrate` | `exception` | `migrate` | `migrate` | `migrate` | `migrate` | `not-applicable` | `migrate` |
| `event_frontier` | `migrate` | `exception` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` | `migrate` |

No cell authorizes behavior movement. A `migrate` verdict means only the
selected scaffolding surface named below may change.

### 3.4 C-01 — public and seat-private effect-envelope constructors

Constructors change only envelope assembly. Payload formation, semantic effect
variants, ordering, visibility decisions, recipient identity, reveal timing,
and filters remain in each game.

| Game | Public constructor | Seat-private constructor | Grounded verdict and retained owner |
|---|---|---|---|
| `plain_tricks` | `src/effects.rs::public_effect` → `EffectEnvelope::public` | `src/effects.rs::private_effect` → `EffectEnvelope::private_to` | `migrate` both as separate diffs. `hand_dealt_effect` continues to decide the already-typed owner and cards; setup/effect order remains local. |
| `flood_watch` | `src/effects.rs::public_effect` → shared constructor | No private envelope constructor or seat-private effect class | public `migrate`; private `not-applicable`. Forecast/event visibility remains local. |
| `frontier_control` | `src/effects.rs::public_effect` → shared constructor | No private envelope constructor or hidden effect class | public `migrate`; private `not-applicable`. All graph/clash/scoring effects remain public and local. |
| `event_frontier` | `src/effects.rs::public_effect` → shared constructor | No private envelope constructor or seat-private effect class | public `migrate`; private `not-applicable`. Current/next reveal and hidden-tail policy remain local. |

Required equality evidence per migration: envelope visibility, payload equality,
effect order, debug/stable rendering, declared effect hash, replay checkpoints,
viewer filtering, and export bytes. A constructor change must not regenerate a
golden trace.

### 3.5 C-02 — canonical seat grammar and compatibility surfaces

C-02 applies only to seat IDs. `TrickCardId`, `TrickSuit`, `TrickRank`,
`DistrictId`, `EventKind`, `SiteId`, and `FactionId` retain their game-owned
parsers and spellings.

| Game / sub-surface | Verdict | Exact owner and requirement |
|---|---|---|
| Plain Tricks typed parser | `migrate` | [`games/plain_tricks/src/ids.rs::PlainTricksSeat::parse`][pt-ids] delegates strict grammar to `SeatId::parse_canonical`, extracts index `0` or `1`, then maps through `PlainTricksSeat::from_index`. Preserve exact accepted/rejected language and `as_str()` output. |
| Plain Tricks WASM import aliases | `exception` | [`crates/wasm-api/src/seats.rs::{parse_plain_seat, parse_seat_import}`][wasm-seats] remains import-only compatibility; canonical `seat_<n>` output remains authoritative. No output flip. |
| Flood Watch seat parsing/output | `exception` | No game-local seat enum/parser exists. `parse_flood_seat` already returns canonical generic `SeatId` through the shared import adapter; `flood_seats` emits canonical IDs. Fresh tests/receipt only. |
| Frontier Control seat parsing/output | `exception` | Same, via `parse_frontier_seat` and `frontier_seats`. Faction IDs are not seat aliases. |
| Event Frontier seat parsing/output | `exception` | Same, via `parse_event_frontier_seat` and `event_frontier_seats`. Faction IDs are not seat aliases. |

Every C-02 receipt must include canonical acceptance, missing prefix, empty
index, leading zero, sign, whitespace, non-ASCII digit, overflow, out-of-game
index, and allowed legacy import alias cases. TypeScript must not normalize or
repair a seat ID. Existing traces and output spellings are unchanged by
default.

For the three shared-boundary exceptions, the compatibility window retains the
current import aliases and canonical output contract; rollback removes only R3
conformance tests/receipt text; the next review trigger is a new game-local
typed seat parser, an output-seat schema change, or a separately admitted
canonical-seat migration.

### 3.6 C-03 — exact-count, variant, role, and faction structure

`SeatCount::new` proves non-zero structural count and provides a typed count.
The game still owns the exact expected count, diagnostics, variant agreement,
role/faction identities, order, setup composition, and all behavioral policy.
No game admits a true seat range at the target commit.

| Game / predicate | Verdict | Selected structural change | Explicit retained behavior |
|---|---|---|---|
| Plain Tricks `seats.len()` | `migrate` | Construct `SeatCount`, compare `.get()` to `STANDARD_SEAT_COUNT`. | Existing exact diagnostic, two-seat policy, deal/leader rotation, and setup state. |
| Plain Tricks `variant.seat_count` | `not-applicable` | The current setup path does not enforce this second predicate; R3 must not add a new acceptance/rejection rule while adopting scaffolding. | Variant semantics remain unchanged; reassessment may only correct this row if the exact target code differs. |
| Flood Watch `seats.len()` | `migrate` | Structural count wrapper. | Exact two-seat cooperative policy and diagnostics. |
| Flood Watch `variant.seat_count` | `migrate` | Separate count-only diff. | Variant selection and all setup semantics. |
| Flood Watch `variant.role_order.len()` | `migrate` | Separate count-only diff. | Role identities, role ordering, assignment, powers, and cooperative policy remain local. |
| Frontier Control `seats.len()` | `migrate` | Structural count wrapper. | Exact two-seat asymmetric policy and diagnostics. |
| Frontier Control `variant.seat_count` | `migrate` | Separate count-only diff. | Variant policy. |
| Frontier Control faction sequence | `exception` | No shared helper decides or validates `Garrison`/`Prospectors` identity or order. | Exact faction identities/order, starting sites/units, graph, action budgets, and scoring stay local. |
| Event Frontier `seats.len()` | `migrate` | Structural count wrapper. | Exact two-seat asymmetric policy and diagnostics. |
| Event Frontier `variant.seat_count` | `migrate` | Separate count-only diff. | Variant policy. |
| Event Frontier faction sequence | `exception` | No shared helper decides or validates `Charter`/`Freeholders` identity or order. | Exact faction identities/order, event epochs, resources, graph, eligibility, and scoring stay local. |
| All four `SeatCountRange` | `not-applicable` | No target game admits a range at this commit. | Next trigger: a grounded setup path that genuinely accepts more than one count. |
| All four ring helpers | `not-applicable` | No C-03 task replaces leader, turn, role, faction, or round rotation. | Every rotation/order rule stays game-owned. |

Flood Watch and Frontier Control may require a new normal `game-stdlib`
dependency for the selected structural helper; Plain Tricks and Event Frontier
already have one. Dependency changes must remain noun-free and must not pull
behavior into `game-stdlib`.

### 3.7 C-04/C-05 — action-tree v1 and adjacent byte/hash surfaces

All four games contain an ad hoc local action-tree hash surface. R3 adds the
shipped canonical action-tree v1 bytes/hash in parallel; it does not silently
replace the local hash or make all game bytes canonical.

| Game | Existing local surface | Selected migration | Required representative trees |
|---|---|---|---|
| `plain_tricks` | `src/replay_support.rs::action_tree_hash` | Add `tree.stable_bytes(V1)` and `tree.stable_hash(V1)` as a named parallel surface; retain local hash as an exception. | Opening trick, forced follow-suit, void/free discard, final play, terminal empty tree. |
| `flood_watch` | `src/visibility.rs::action_tree_hash` | Same parallel v1 surface; retain local debug-derived hash. | Bail, place levee, role power, early end, budget exhausted/automatic environment, terminal. |
| `frontier_control` | `src/visibility.rs::action_tree_hash` | Same; retain local hash. | Muster/reinforce, move, clash branch, stake/dismantle, early end, terminal. |
| `event_frontier` | `src/visibility.rs::action_tree_hash` | Same; retain local hash. | Full/limited operation choice, multi-site branch, event choice, pass, edict-blocked state, Reckoning/terminal. |

The selected action-tree v1 surface includes its exact versioned bytes and the
hash computed from those bytes. This is one coherent C-04/C-05 migration
surface per game. Classification is expected to be `parallel-new-surface`.
The following adjacent surfaces are explicit `exception` rows and must remain
unchanged unless a separately admitted ADR-0009 packet names one of them:

| Adjacent surface | Verdict | Compatibility owner |
|---|---|---|
| Existing local action-tree hash | `exception` | Current game module and all existing trace/checkpoint consumers. |
| State bytes/hash | `exception` | Current game replay/serialization implementation. |
| Effect bytes/hash | `exception` | Current effect log/replay implementation. |
| Public-view bytes/hash | `exception` | Current game projection/visibility implementation. |
| Seat-private-view bytes/hash | Plain Tricks `exception`; other games `not-applicable` | Plain Tricks current viewer projection. |
| Replay command/trace bytes | `exception` | Existing native replay/trace validators. |
| Public export bytes/hash | `exception` | Existing Rust/WASM exporter for each game. |
| Seat-private export bytes/hash | Plain Tricks `exception`; others `not-applicable` | Plain Tricks viewer-scoped exporter. |
| Diagnostics | `exception` | Existing game rules/setup owner. |

No legal choice, segment, label, accessibility label, metadata, tag, preview,
freshness token, or branch order may change as a side effect of adding the
parallel v1 surface.

### 3.8 C-07 — no-leak geometry by actual visibility shape

The shared harness may enumerate caller-supplied cases and compare expected
presence/absence. It may not project state, choose viewers, infer hidden facts,
decide reveal timing, filter effects, execute actions, or create export policy.
Every game owns its probes and expectations.

#### Plain Tricks — full seat-private matrix

Run the matrix twice, once with source seat `S = seat_0` and once with
`S = seat_1`; `O` is the other seat. Choose characterized states where `S` is
the active seat when legal-tree presence is asserted.

| Source fact | Surface | Public observer | Viewer `S` | Viewer `O` |
|---|---|---:|---:|---:|
| Unplayed card in `S` hand | projected view | absent | present | absent |
| Currently legal unplayed card in `S` hand | legal action tree | absent | present | absent |
| Unplayed card in `S` hand | public effect stream | absent | absent | absent |
| `HandDealt` payload for `S` | filtered effect stream | absent | present | absent |
| Unplayed/private card | invalid-action diagnostic | absent | absent | absent |
| Unplayed/private card | `public-export-v1` timeline | absent | absent | absent |
| Unplayed/private card | `seat-private-export-v1` timeline for the selected viewer | absent | present | absent |
| Own hand and legal choices | bot input for `S` | not-applicable | present | absent |
| Non-selected private card/candidate | bot explanation and candidate rendering | absent | absent | absent |
| Hidden tail card before authorization | view/tree/effect/diagnostic/export/bot surfaces | absent | absent | absent |
| Card after game-authorized play/reveal | public view/effect/export | present | present | present |
| Any still-hidden tail card at terminal | every public/viewer-scoped surface | absent | absent | absent |

The matrix augments, never replaces, game-specific tests for deal privacy,
follow suit, off-suit resolution, trick-winner lead, public observer export,
seat-private view, bot behavior, and diagnostics. ADR 0004 applies to every
viewer-scoped export and timeline step.

#### Flood Watch — public forecast with hidden future deck

There is no per-seat private holding. Use caller-owned source tokens such as
`EventDeckIndex(i)` rather than pretending the source belongs to a seat.
Viewers are public observer, seat 0, and seat 1; expected results are identical
for all three unless the game itself authorizes a public reveal.

| Source fact | Surface | Observer | Seat 0 | Seat 1 |
|---|---|---:|---:|---:|
| Event card deeper than the public forecast | projected view/action tree/diagnostic/public effects | absent | absent | absent |
| Same hidden future card | public replay export | absent | absent | absent |
| Same hidden future card | bot input/explanation/candidate rendering | absent | absent | absent |
| Current public forecast | public view and export | present | present | present |
| Card drawn/resolved by the environment | public effect/view/export after reveal | present | present | present |
| Any still-undrawn future card | terminal/public summary | absent | absent | absent |
| Per-seat hand/choice/export | seat-private surfaces | `not-applicable` | `not-applicable` | `not-applicable` |

C-07 aggregate verdict is `migrate` for the hidden-future/public-observer
matrix. Cooperative role identity and event/forecast rules remain local.

#### Frontier Control — fully public, no hidden source

The visibility implementation returns the same public projection
for observer, seat 0, and seat 1; setup contains no randomness or hidden
holdings. There is no meaningful secret canary or source-owner matrix.

| Sub-surface | Verdict | Required evidence |
|---|---|---|
| Pairwise hidden-fact matrix | `not-applicable` | Characterization proves no hidden source, private holding, private effect, viewer redaction, or hidden export class. |
| Observer/seat equality | `exception` | Retain focused tests showing all three projections and public effect streams are equal. Do not replace them with a vacuous generic matrix. |
| Seat-private export | `not-applicable` | No official seat-private timeline exists; do not create one. |

#### Event Frontier — public current/next card with hidden deeper deck

There is no per-seat private holding. The current and next card are public by
game rule; deeper deck order remains hidden. Use caller-owned deck-position
sources and the same observer/seat 0/seat 1 viewer set.

| Source fact | Surface | Observer | Seat 0 | Seat 1 |
|---|---|---:|---:|---:|
| Card deeper than current/next public window | projected view/action tree/diagnostic/public effects | absent | absent | absent |
| Same hidden deeper card | public replay export | absent | absent | absent |
| Same hidden deeper card | bot input/explanation/candidate rendering | absent | absent | absent |
| Current event/edict card | public view/effect/export | present | present | present |
| Next public card | public view/export | present | present | present |
| Resolved/discarded card after authorization | public history/effect/export | present | present | present |
| Any still-hidden deeper tail card at terminal | every public surface | absent | absent | absent |
| Per-seat hand/choice/export | seat-private surfaces | `not-applicable` | `not-applicable` | `not-applicable` |

C-07 aggregate verdict is `migrate` for the hidden-tail/public-observer matrix.
Event/edict reveal and resolution policy remains local.

**Canary rule:** test canaries exist only in memory. A canary must never be
written to a committed trace, fixture, export, snapshot, log, test ID, DOM,
storage, accessibility artifact, or screenshot.

### 3.9 C-08 — evidence-profile driver matrix

Profile drivers validate profile metadata first and delegate all behavior to a
caller-supplied game or tool adapter. They must never interpret trick rules,
connectivity, event/edict semantics, budgets, resources, roles, factions, or
scoring. Existing fixtures and traces are read-only by default.

| Profile | `plain_tricks` | `flood_watch` | `frontier_control` | `event_frontier` |
|---|---|---|---|---|
| `replay-command-v1` | `migrate`; default `internal-dev`; current native trace/replay validator owns commands/checkpoints/hashes | `migrate`; default `internal-dev`; same | `migrate`; may be `public` only if characterization proves no seed/hidden input; otherwise `internal-dev` | `migrate`; `internal-dev` because native evidence may contain hidden deck order |
| `setup-evidence-v1` | `migrate`; standard fixture; `internal-dev` where deal/tail facts are asserted | `migrate`; standard + deluge; default `public` unless test-only deck facts require `internal-dev` | `migrate`; standard + highlands; default `public` | `migrate`; standard + hard-winter + land-rush; `internal-dev` where hidden deck order is asserted |
| `domain-evidence-v1` | `migrate`; deck partition/trick-round domain assertions remain game-owned | `migrate`; levee absorption, inundation, forecast/event pressure, role and budget evidence | `migrate`; graph/topology inputs, adjacency, clash, connectivity, round scoring | `migrate`; graph/event/edict/funding/resource/Reckoning evidence |
| `public-export-v1` | `migrate`; public observer timeline, no private hand/tail | `migrate`; public forecast/resolved events only, no future deck | `migrate`; fully public observation timeline | `migrate`; current/next/history only, no deeper deck |
| `seat-private-export-v1` | `migrate`; run for both labelled viewers with pairwise no-leak | `not-applicable`; no per-seat private timeline | `not-applicable`; no private timeline | `not-applicable`; no per-seat private timeline |

The central setup/domain fixture inventory is:

| Game | Setup/domain evidence inputs | Semantic owner retained |
|---|---|---|
| `plain_tricks` | `data/fixtures/plain_tricks_standard.fixture.json` | Deck partition, deal shape, hand/tail status, trick/round expectations. |
| `flood_watch` | `flood_watch_standard.fixture.json`, `flood_watch_deluge.fixture.json` | Role order, starting flood/levee/budget state, forecast/event pressure, levee absorption, inundation/loss and scoring. |
| `frontier_control` | `frontier_control_standard.fixture.json`, `frontier_control_highlands.fixture.json` | Named sites/edges, starting units, adjacency/movement inputs, connectivity and round/final scoring. |
| `event_frontier` | `event_frontier_standard.fixture.json`, `event_frontier_hard_winter.fixture.json`, `event_frontier_land_rush.fixture.json` | Named sites/trails, deck/epoch setup, event/edict resolution, eligibility, funding/income/resource caps and Reckoning/final scoring. |

A single physical fixture may feed separate setup and domain adapters only when
each adapter declares its selected fields, profile metadata, visibility,
validator owner, and N/A surfaces. The driver validates shape; the game-owned
adapter validates meaning. No fixture gains selectors, triggers, conditions,
formulas, scripts, loops, executable mutations, or hidden defaults.

The three seat-private profile N/As remain valid until a separately admitted,
viewer-labelled official export exists. A private view, internal trace, or
hidden deck is not by itself a seat-private export.

### 3.10 C-06/C-09/C-10 checkpoint matrix

| Game | C-06 dev-only support | C-09 bounded index | C-10 non-promotion |
|---|---|---|---|
| `plain_tricks` | `migrate`; add `game-test-support` under `[dev-dependencies]` only | `migrate`; replace local rejection sampler after exact vector/draw-count proof | `rejected / local-only`: deal, follow suit, trick lifecycle, winner/leader, scoring, projection/redaction |
| `flood_watch` | `migrate` | `migrate`; preserve event-deck order, forecast, draws, seed and rejection counts | `rejected / local-only`: roles, role powers/order, forecast/event pressure, levees, inundation, budget and scoring |
| `frontier_control` | `migrate` | `not-applicable`; no game randomness or bounded-index surface | `rejected / local-only`: factions, graph/topology, adjacency/movement, clash, muster/reinforce caps, connectivity and scoring |
| `event_frontier` | `migrate` | `migrate`; preserve epoch shuffle/deck order, current/next window, seed and rejection counts | `rejected / local-only`: factions, graph/trails, event/edict resolution, eligibility, funding/pass/Reckoning income, caps and scoring |

C-09 replaces only the local bounded-index call with
`DeterministicRng::next_index_unbiased_v1`. It does not centralize shuffle,
deal, deck construction, epoch partitioning, forecast, current/next reveal, or
any game rule. Expected ADR-0009 classification is `unchanged`; a single
vector, draw-count, visibility, or hash mismatch blocks that game's task.

### 3.11 Out of scope

- Any fifth game, `8C-R4`, River Ledger/Briar Circuit/Vow Tide audit, or Gate 18
  partnership work.
- A new game, rules change, content change, balance change, bot-strategy change,
  UI redesign, web catalog change, animation, accessibility feature, or browser
  smoke expansion.
- A new shared helper or a change to C-01…C-10 contracts already shipped.
- Promotion of trick-taking, graph/topology, connectivity, event/edict,
  budget/resource, team/role/faction, projection/redaction, scoring, or outcome
  policy.
- Replacing game-specific tests with a generic assertion.
- Reclassifying internal traces as public exports or creating new exporters to
  force a profile to apply.
- A blanket migration of existing local hashes, traces, fixtures, snapshots,
  or seat spellings.
- Foundation, ADR, roadmap, or evidence-taxonomy redesign. A genuine doctrine
  gap becomes the blocking §8.4 note, not implementation scope.
- Any change to `apps/web/README.md`; this unit has no game admission or web
  surface.

### 3.12 Not allowed

- No shared helper may decide legality, setup composition, seat assignment,
  role/faction/team identity or order, trick lead/follow/trump/winner/next
  leader, graph adjacency/movement/connectivity, event/edict resolution,
  budget/resource accrual/caps, scoring, outcome, reveal, projection,
  redaction, authorization, or bot choice.
- No game noun enters `engine-core`; no mechanic-specific noun or policy enters
  a generic helper merely because four games expose a similarly shaped test.
- No silent byte, hash, field order, seat-ID, visibility, export, RNG draw, or
  serialization change.
- No broad “normalize”, “clean up”, “deduplicate”, “update snapshots”, or
  “regenerate goldens” diff.
- No YAML, DSL, selectors, conditions, triggers, formulas, scripts, or
  procedural behavior in fixtures or static data.
- No hidden datum in public payloads, DOM, storage, logs, effects, diagnostics,
  bot explanations, candidate rankings, traces, fixtures, or exports.
- No committed test canary.
- No `game-test-support` normal/build dependency from a game, tool, WASM, or
  production target.
- No TypeScript legality or seat normalization.
- No deleted, ignored, weakened, broadened-to-vacuity, or rewritten test merely
  to obtain green output.
- No implementation diff spanning multiple selected surfaces or multiple games.

## 4. Deliverables

### 4.1 Concrete artifact tree

The accepted implementation is expected to produce the following bounded
artifact set. “Evidence input” means read and hashed/validated but unchanged by
default.

```text
reports/
  8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md   # new

games/plain_tricks/
  Cargo.toml                                                           # dev-only support
  src/effects.rs                                                       # C-01 public/private constructors
  src/ids.rs                                                           # C-02 typed seat parser
  src/setup.rs                                                         # C-03 roster + C-09 sampler
  src/replay_support.rs                                                # C-04/C-05 parallel tree v1; profile adapters if test-only placement is impossible
  tests/replay.rs                                                      # C-04/C-05/C-08 evidence
  tests/serialization.rs                                               # byte/hash compatibility evidence
  tests/visibility.rs                                                  # C-07 full pairwise matrix; C-08 exports
  tests/bots.rs                                                        # bot-input/explanation no-leak
  data/fixtures/plain_tricks_standard.fixture.json                     # evidence input; unchanged by default
  tests/golden_traces/*.trace.json                                     # evidence inputs; unchanged by default

games/flood_watch/
  Cargo.toml                                                           # game-stdlib if needed + dev-only support
  src/effects.rs                                                       # C-01 public constructor
  src/setup.rs                                                         # C-03 predicates + C-09 sampler
  src/visibility.rs                                                    # C-04/C-05 parallel tree v1 only if this remains the narrowest owner
  src/replay_support.rs                                                # C-08 adapter seam if already authoritative
  tests/replay.rs
  tests/serialization.rs
  tests/visibility.rs                                                  # C-07 hidden-future/public-observer matrix
  tests/bots.rs
  data/fixtures/flood_watch_standard.fixture.json                   # evidence input
  data/fixtures/flood_watch_deluge.fixture.json                    # evidence input
  tests/rules.rs                                                        # game-owned domain assertions
  tests/golden_traces/*.trace.json                                     # evidence inputs

games/frontier_control/
  Cargo.toml                                                           # game-stdlib if needed + dev-only support
  src/effects.rs                                                       # C-01 public constructor
  src/setup.rs                                                         # C-03 predicates
  src/visibility.rs                                                    # C-04/C-05 parallel tree v1 + retained equality tests
  src/replay_support.rs                                                # C-08 adapter seam if already authoritative
  tests/replay.rs
  tests/serialization.rs
  tests/visibility.rs                                                  # C-07 N/A/equality receipt
  tests/bots.rs
  data/fixtures/frontier_control_standard.fixture.json              # evidence input
  data/fixtures/frontier_control_highlands.fixture.json             # evidence input
  tests/rules.rs                                                        # game-owned graph/scoring assertions
  tests/golden_traces/*.trace.json                                     # evidence inputs

games/event_frontier/
  Cargo.toml                                                           # dev-only support
  src/effects.rs                                                       # C-01 public constructor
  src/setup.rs                                                         # C-03 predicates + C-09 sampler
  src/visibility.rs                                                    # C-04/C-05 parallel tree v1
  src/replay_support.rs                                                # C-08 adapter seam if already authoritative
  tests/replay.rs
  tests/serialization.rs
  tests/visibility.rs                                                  # C-07 hidden-tail/public-observer matrix
  tests/bots.rs
  data/fixtures/event_frontier_standard.fixture.json                # evidence input
  data/fixtures/event_frontier_hard_winter.fixture.json            # evidence input
  data/fixtures/event_frontier_land_rush.fixture.json              # evidence input
  tests/rules.rs                                                        # game-owned event/resource/scoring assertions
  tests/golden_traces/*.trace.json                                     # evidence inputs

crates/wasm-api/src/seats.rs                                           # C-02 conformance tests only; no output change expected

docs/MECHANICAL-SCAFFOLDING-REGISTER.md                               # append R3 receipts under MSC-8C-001…010
specs/README.md                                                        # final R3-only status flip
```

No change is planned to `engine-core`, `game-stdlib`, `game-test-support`, the
accepted ADRs, foundation docs, `tools/*`, or `apps/web/*`. Those are existing
owners and regression surfaces. If implementation discovers that a shared
contract itself must change, the affected task stops under §8.4 rather than
quietly expanding this tree.

### 4.2 Characterization report

Create
`reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`
before the first production migration. It is append-only during execution and
must contain:

1. the repository/commit statement;
2. the locked determination and four-game inventory;
3. proof that none of the four Cargo manifests contains `game-test-support`
   before R3 and none of the games is named as an 8C/C-08 pilot;
4. a file/symbol inventory for C-01…C-10, including every explicit N/A and
   exception;
5. before-state digests for every named existing golden trace, fixture, public
   export, seat-private export, local action-tree hash vector, and relevant
   replay/checkpoint hash;
6. canonical seat acceptance/rejection and WASM alias/output vectors;
7. C-03 accepted/rejected count vectors, exact diagnostics, variant seat-count
   evidence, Flood Watch role-order cardinality, and retained faction-order
   policy;
8. per-game C-04/C-05 representative action trees with exact existing local
   hash and proposed v1 byte/hash vectors;
9. the complete Plain Tricks source-seat × viewer × surface matrix; Flood Watch
   and Event Frontier hidden-deck × viewer × surface matrices; Frontier Control
   N/A/equality table;
10. C-08 metadata, visibility, owner, selected fields, real caller, validator,
    canonical-byte authority, migration note, and N/A rows for every profile;
11. C-09 fixed-word inputs, rejection paths, RNG draw counts, complete shuffled
    decks, dealt/forecast/current-next state, and downstream hash/visibility
    comparisons;
12. the authorized changed-file inventory and explicit proof of zero
    unauthorized golden/fixture/export changes;
13. command, exit status, relevant output, and failure classification for every
    §7.1 command; and
14. before/after register receipts and the final tracker change.

The report records values measured from the executing repository. This planning
artifact deliberately does not freeze hash literals that `/reassess-spec` must
re-pin.

### 4.3 Register receipts

Append R3 receipt tables under the existing entries; do not create parallel
register entries for the same shipped helper.

| Register entry | Required R3 receipt content |
|---|---|
| `MSC-8C-001` | Five C-01 migrations: Plain public/private and the three public-only constructors; three private N/A rows. |
| `MSC-8C-002` | Plain typed-parser migration; shared WASM import/output conformance exceptions for all four; explicit non-seat-ID exclusion. |
| `MSC-8C-003` | Eight selected count predicates, Plain variant N/A, faction/role identity-order exceptions, range/ring N/As. |
| `MSC-8C-004` | Four action-tree v1 byte/hash parallel-surface receipts and representative tree vectors. |
| `MSC-8C-005` | `StableBytesWriter` v1 use only through the action-tree surface; all adjacent stable-byte/hash exceptions. |
| `MSC-8C-006` | Four fresh dev-only dependency receipts plus inverse normal-edge proof. |
| `MSC-8C-007` | Plain full private matrix, Flood hidden-future matrix, Frontier N/A/equality receipt, Event hidden-tail matrix, canary hygiene. |
| `MSC-8C-008` | Twenty profile decisions: four replay, four setup, four domain, four public export, one seat-private migration, three seat-private N/As. |
| `MSC-8C-009` | Plain/Flood/Event sampler migrations with output and draw-count identity; Frontier N/A. |
| `MSC-8C-010` | Per-game rejected/local-only lists for trick, role, faction, graph, event, budget/resource, projection/redaction, scoring and outcome behavior. |

Every exception receipt names owner, compatibility statement, rollback or
reversal scope, and next review trigger. Every N/A receipt names the missing
official surface and the concrete condition that would reopen review.

## 5. Work breakdown

Every row below is a candidate `AGENT-TASK` using task profile
`scaffold-refactor`. Ticket decomposition may rename IDs or split a row when a
selected surface still cannot fit one reviewable diff. It may not merge games,
merge profile classes, merge public/private visibility classes, or turn an
N/A/exception into speculative implementation.

### 5.1 Mandatory ten-point protocol for every migration task

Each task must state:

1. accepted authority: the applicable `MSC-8C-*` entry, ADR 0008, ADR 0009,
   and ADR 0004 whenever visibility/export is involved;
2. exact owner paths and existing/proposed symbols;
3. every affected hash, trace, fixture, seat spelling, visibility, export,
   diagnostic, bot, and RNG-consumption surface, including explicit N/A rows;
4. characterization tests, artifact digests, and register/report receipt
   captured before modification;
5. one selected surface in one reviewable diff;
6. ADR-0009 classification: `unchanged`, `parallel-new-surface`, or
   `intentional-migration`—the last requires separate admission before code;
7. compatibility window and retained legacy reader/authority;
8. exact before/after commands and byte/hash/visibility/RNG comparison;
9. rollback that removes only the selected surface and restores no unrelated
   file; and
10. the `AGENT-DISCIPLINE.md` failing-test protocol: verify the failing test is
    still valid, decide whether the fault belongs to the SUT or test, fix the
    correct owner, and never delete or weaken a valid test to get green.

### 5.2 Wave A — admission and characterization

| Task | Exact target | Affected surfaces | Required evidence and rollback |
|---|---|---|---|
| `8C-R3-001` | `specs/README.md`, parent/R1/R2 specs, atlas, register, accepted ADRs; create characterization report shell | determination, authority, provenance, exact game inventory, all verdict slots | Freeze the four games and the matrix; record the C-09 correction and historical-baseline divergence. Evidence-only rollback removes only an unaccepted report draft. |
| `8C-R3-002` | Four `Cargo.toml`; selected `src/{actions,bots,effects,ids,rules,setup,state,replay_support,visibility}.rs`; relevant `tests/{bots,property,replay,rules,serialization,visibility}.rs` that exist; all named fixtures/traces | every C-01…C-10 byte/hash/seat/count/visibility/profile/RNG surface | Populate before-state digests, matrices, C-03 predicates, C-07 expectations, C-08 callers, C-09 vectors, exceptions and N/As. No production change. Any uncharacterized required surface blocks its later task. |

### 5.3 Wave B — C-01 envelope constructors

| Task | Exact files/symbols | Affected hash/visibility surface | One-surface rollback |
|---|---|---|---|
| `8C-R3-101` | `games/plain_tricks/src/effects.rs::public_effect` | public effect visibility/payload/order, effect hash, replay checkpoints, public export | Restore only the local public literal constructor. |
| `8C-R3-102` | `games/plain_tricks/src/effects.rs::private_effect`; callers including `hand_dealt_effect` | private owner `SeatId`, filtered deal effects, seat-private view/export, effect hash, no-leak matrix | Restore only the local private literal constructor; do not alter payload or caller ownership. |
| `8C-R3-103` | `games/flood_watch/src/effects.rs::public_effect` | public forecast/event/levee/terminal effects and every current effect hash/export | Restore only the local public literal constructor. |
| `8C-R3-104` | `games/frontier_control/src/effects.rs::public_effect` | public graph/clash/round/terminal effects and hashes | Same isolated rollback. |
| `8C-R3-105` | `games/event_frontier/src/effects.rs::public_effect` | public event/edict/resource/Reckoning effects and hashes | Same isolated rollback. |

Private-constructor N/A rows for Flood Watch, Frontier Control, and Event
Frontier land in the report/register; no private helper or effect variant is
added.

### 5.4 Wave C — C-02 parser adoption and boundary conformance

| Task | Exact files/symbols | Affected surface | Rollback |
|---|---|---|---|
| `8C-R3-201` | `games/plain_tricks/src/ids.rs::PlainTricksSeat::{parse,from_index,as_str}`; unit tests | canonical accept/reject set and typed mapping only; no card-ID parsing | Restore the manual two-string parser. Existing output and traces remain unchanged. |
| `8C-R3-202` | `crates/wasm-api/src/seats.rs::{parse_seat_import,parse_plain_seat,parse_flood_seat,parse_frontier_seat,parse_event_frontier_seat,plain_seats,flood_seats,frontier_seats,event_frontier_seats}` plus existing seat tests/snapshots | verify import aliases, canonical Rust output, out-of-game index rejection, and absence of TS normalization | Evidence/test-only rollback. If a real output defect is found, stop and split one game-specific ADR-0009 seat-output migration; do not hide it here. |

### 5.5 Wave D — C-03 setup/profile diversity

Each predicate is a separate selected surface. Exact game diagnostics and
behavior must remain byte-identical.

| Task | Exact files/symbols | Affected setup/hash surface | Rollback |
|---|---|---|---|
| `8C-R3-301` | `games/plain_tricks/src/setup.rs::setup_match`, `seats.len()` predicate; existing `game-stdlib` edge | accepted/rejected count, diagnostic, setup/deal/RNG/state/effect/replay/view hashes | Restore only the local roster-length predicate. |
| `8C-R3-302` | `games/flood_watch/Cargo.toml`; `src/setup.rs::setup_match`, roster predicate | normal dependency if required; count/diagnostic; setup/deck/state hashes | Restore predicate and remove only the added `game-stdlib` edge if no other R3 task needs it. |
| `8C-R3-303` | `games/flood_watch/src/setup.rs::setup_match`, `variant.seat_count` predicate | variant acceptance/diagnostic and setup equality | Restore only that comparison. |
| `8C-R3-304` | same function, `variant.role_order.len()` predicate | role-order cardinality acceptance/diagnostic; role identities/order/powers unchanged | Restore only cardinality comparison. |
| `8C-R3-305` | `games/frontier_control/Cargo.toml`; `src/setup.rs::setup_match`, roster predicate | dependency if required; setup/state/diagnostic equality | Restore predicate and isolated dependency edge. |
| `8C-R3-306` | `games/frontier_control/src/setup.rs::setup_match`, `variant.seat_count` predicate | variant acceptance/diagnostic; faction and graph setup unchanged | Restore only that comparison. |
| `8C-R3-307` | `games/event_frontier/src/setup.rs::setup_match`, roster predicate | accepted/rejected count, diagnostic, deck/state/replay/view equality | Restore only roster comparison. |
| `8C-R3-308` | same function, `variant.seat_count` predicate | variant acceptance/diagnostic; faction/event/resource setup unchanged | Restore only that comparison. |

Plain variant enforcement, range/ring helpers, Flood role identity/order, and
Frontier/Event faction identity/order land as evidence receipts, not code.

### 5.6 Wave E — C-04/C-05 parallel action-tree v1

| Task | Exact files/symbols | Selected and adjacent surfaces | Rollback |
|---|---|---|---|
| `8C-R3-401` | `games/plain_tricks/src/replay_support.rs::action_tree_hash`; legal-tree caller; replay/serialization tests | add v1 bytes/hash for named trick trees; retain local hash and all state/effect/view/replay/export/diagnostic bytes | Remove only the new v1 adapter/vectors/tests. |
| `8C-R3-402` | `games/flood_watch/src/visibility.rs::action_tree_hash`; legal-tree caller; replay/serialization tests | add v1 bytes/hash for levee/bail/role/end/environment trees; retain local debug hash and all domain behavior | Same. |
| `8C-R3-403` | `games/frontier_control/src/visibility.rs::action_tree_hash`; legal-tree caller; replay/serialization tests | add v1 bytes/hash for muster/move/clash/stake/end trees; graph legality unchanged | Same. |
| `8C-R3-404` | `games/event_frontier/src/visibility.rs::action_tree_hash`; legal-tree caller; replay/serialization tests | add v1 bytes/hash for operation/event/edict/pass/Reckoning trees; event legality unchanged | Same. |

All four are expected `parallel-new-surface`. Any proposal to replace an
existing local hash is a new selected surface and must be separately admitted.

### 5.7 Wave F — C-06 dev-only dependency discipline

| Task | Exact file | Surface | Rollback |
|---|---|---|---|
| `8C-R3-501` | `games/plain_tricks/Cargo.toml` | add `game-test-support` under `[dev-dependencies]` only | Remove dependency and only tests that directly require it. |
| `8C-R3-502` | `games/flood_watch/Cargo.toml` | same | Same. |
| `8C-R3-503` | `games/frontier_control/Cargo.toml` | same | Same. |
| `8C-R3-504` | `games/event_frontier/Cargo.toml` | same | Same. |

Every task captures `cargo tree --workspace -e normal --invert game-test-support`
before and after. A normal/build edge is a stop condition.

### 5.8 Wave G — dedicated C-07 no-leak work

| Task | Exact files/symbols | Matrix/surfaces | Rollback |
|---|---|---|---|
| `8C-R3-511` | `games/plain_tricks/tests/visibility.rs`, `tests/bots.rs`, `tests/replay.rs`; `game_test_support::no_leak::assert_pairwise_no_leak` | both source seats × observer/owner/opponent × view/tree/diagnostic/effect/public export/seat-private export/bot surfaces; pre/post play and terminal tail | Remove only generic matrix tests/adapters; retain every specific trick/deal/export test. |
| `8C-R3-512` | `games/flood_watch/tests/visibility.rs`, `tests/bots.rs`, `tests/replay.rs` | hidden event-deck positions × observer/seat0/seat1 × view/tree/diagnostic/effect/public export/bot; forecast/drawn public transitions | Same; role/event/forecast policy remains in game probes. |
| `8C-R3-513` | `games/frontier_control/tests/visibility.rs` and relevant replay/bot tests | characterize N/A; retain observer=seat0=seat1 projection and public effect/export equality | Evidence/test-only rollback; do not add a fake secret canary. |
| `8C-R3-514` | `games/event_frontier/tests/visibility.rs`, `tests/bots.rs`, `tests/replay.rs` | hidden deeper-deck positions × three viewers × all public surfaces; current/next/resolved visibility transitions | Remove only generic matrix tests/adapters; retain specific event/edict/export tests. |

Canaries are in-memory-only and absent from committed artifacts by construction.

### 5.9 Wave H — C-08 replay-command profile drivers

| Task | Exact files/symbols | Profile/bytes | Rollback |
|---|---|---|---|
| `8C-R3-601` | Plain `tests/replay.rs`, `src/replay_support.rs`; `ReplayCommandV1Driver` | current native commands/checkpoints/hashes; expected `internal-dev`; no trace rewrite | Remove profile metadata/adapter test only. |
| `8C-R3-602` | Flood equivalent | current command authority, event-deck/setup checkpoints and hashes | Same. |
| `8C-R3-603` | Frontier equivalent | current command authority and fully public state; visibility class pinned by characterization | Same. |
| `8C-R3-604` | Event equivalent | current native command authority including hidden deck order; `internal-dev` | Same. |

If a thin `replay-check` dispatch is genuinely absent, reassessment creates a
separate game/profile dispatch task. The tool validates; it never executes or
owns game semantics.

### 5.10 Wave I — C-08 setup-evidence profile drivers

| Task | Exact files/symbols | Setup evidence | Rollback |
|---|---|---|---|
| `8C-R3-611` | Plain standard fixture; setup tests; `SetupEvidenceV1Driver` | seats/options/variant/deck partition/deal expectations; private test facts stay `internal-dev` | Remove profile adapter/test; fixture unchanged. |
| `8C-R3-612` | Flood standard + deluge fixtures; setup tests | seats/role-order cardinality/scenario/start-state evidence; no role behavior in driver | Same. |
| `8C-R3-613` | Frontier standard + highlands fixtures; setup tests | seats/factions/start sites/units/graph-shape setup evidence; no movement/scoring policy | Same. |
| `8C-R3-614` | Event standard + hard-winter + land-rush fixtures; setup tests | seats/factions/epoch/deck/start resources/sites; hidden deck facts test-only | Same. |

A missing thin `fixture-check` dispatch is separately bounded. Existing fixture
bytes are read-only by default.

### 5.11 Wave J — C-08 domain-evidence profile drivers

These are the center of the R3 fixture work. Each task selects domain fields,
validates profile metadata, then invokes an existing or narrowly test-local
game-owned semantic adapter.

| Task | Exact files/symbols | Domain surface | Rollback |
|---|---|---|---|
| `8C-R3-621` | Plain standard fixture; rule/replay tests; `DomainEvidenceV1Driver` | deck partition, hand/tail counts, trick/round invariants and expected outcomes; no trick algorithm in driver | Remove domain adapter/test; fixture and rules unchanged. |
| `8C-R3-622` | Flood standard/deluge fixtures; rule/replay tests | levee absorption, flood rise/inundation, forecast/event pressure, role/start budget and terminal evidence | Same; all calculations stay in Flood Watch. |
| `8C-R3-623` | Frontier standard/highlands fixtures; rule/replay tests | site/edge inputs, adjacency examples, clash/start composition, supply/connectivity, round/final scoring evidence | Same; no graph or scoring helper created. |
| `8C-R3-624` | Event standard/hard-winter/land-rush fixtures; rule/replay tests | site/trail inputs, event/edict cases, eligibility, operation funding/pass/Reckoning income, caps and scoring evidence | Same; no event/budget/scoring DSL or helper created. |

### 5.12 Wave K — C-08 public-export profile drivers

| Task | Exact files/symbols | Profile/visibility/hash | Rollback |
|---|---|---|---|
| `8C-R3-631` | Plain viewer-scoped exporter in `src/replay_support.rs`; `public-replay-export-import` and visibility tests; `PublicExportV1Driver` | observer timeline, hidden hand/tail absence, import round-trip, current export bytes/hash | Remove driver test only. |
| `8C-R3-632` | Flood exporter; `public-replay-export-import` and `public-observer-no-leak` traces/tests | public forecast/resolved events, hidden future-deck absence | Same. |
| `8C-R3-633` | Frontier exporter; `replay-export-import` trace/tests | fully public timeline and import round-trip | Same. |
| `8C-R3-634` | Event exporter; `replay-export-import-no-deck-leak` trace/tests | current/next/resolved history, deeper-deck absence | Same. |

### 5.13 Wave L — C-08 seat-private export profile

| Task | Exact files/symbols | Profile/visibility/hash | Rollback |
|---|---|---|---|
| `8C-R3-641` | Plain viewer-scoped exporter invoked with `Viewer(seat_0)` and `Viewer(seat_1)`; replay/visibility tests; `SeatPrivateExportV1Driver` | labelled viewer seat, own hand at each step, opponent/tail absence, pairwise no-leak, import round-trip if currently supported | Remove seat-private profile tests/adapters only; exporter and policy unchanged. |
| `8C-R3-642` | Flood, Frontier, Event report/register rows | three explicit N/As: no official per-seat private timeline | Evidence-only rollback removes only unaccepted receipt text. Do not create an exporter. |

### 5.14 Wave M — C-09 local unbiased-index replacement

| Task | Exact files/symbols | RNG/hash/visibility surface | Rollback |
|---|---|---|---|
| `8C-R3-701` | `games/plain_tricks/src/setup.rs::{shuffle_deck,next_bounded_index_unbiased}` | fixed RNG words, rejection draws, full deck order, hands/tail, private deal effects/views, state/replay/export hashes | Restore the local helper and its single call path. |
| `8C-R3-702` | `games/flood_watch/src/setup.rs::{shuffle_event_deck,next_bounded_index_unbiased}` | words/rejections, event-deck order, forecast, draw sequence, public view/effects/replay/export hashes | Same. |
| `8C-R3-703` | `games/event_frontier/src/setup.rs::{build_seeded_deck,shuffle_epoch,next_bounded_index_unbiased}` | words/rejections, per-epoch order, current/next/deeper tail, event visibility, replay/export hashes | Same. |

Replace only the local bounded-index call with
`DeterministicRng::next_index_unbiased_v1`. `frontier_control` receives an
explicit N/A receipt. Any identity failure restores the local helper; it does
not authorize a new RNG algorithm.

### 5.15 Wave N — consolidation, register, acceptance, and status closeout

| Task | Exact targets | Required result | Rollback |
|---|---|---|---|
| `8C-R3-801` | characterization report and all touched tests/artifacts | reconcile every matrix cell, sub-surface, hash/visibility/RNG result, exception and N/A; no unowned cleanup | Revert report consolidation only. |
| `8C-R3-802` | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | append complete R3 receipt tables under `MSC-8C-001…010`; preserve R1/R2 receipts; C-10 remains rejected/local-only | Revert only R3 receipt rows. |
| `8C-R3-803` | full §7 command set and changed-file audit | all focused/workspace evidence green; zero unauthorized golden/fixture/export diffs; every failure classified before fixing | Revert the offending selected task, not unrelated work. |
| `8C-R3-804` | `specs/README.md` | after every exit row passes, flip only `8C-R3` to `Done`, add completion evidence/date, leave `8C-R4` and Gate 18 untouched | Restore only the R3 tracker row. |

## 6. Exit criteria

The unit is `Done` only when every row passes. Silence, an unreviewed artifact
diff, or “already works” is never a verdict.

| Exit ID | Obligation | Pass condition / parent mapping |
|---|---|---|
| `R3-EC-01` | Locked determination | `8C-R2` is confirmed `Done`, `8C-R3` remains the selected lowest non-`Done` unit at admission, and no `8C-R4` or Gate 18 work enters scope. |
| `R3-EC-02` | Exact game ownership | Exactly the four locked games appear, matching the parent seed's R3 slice of EC-28; no fifth game or dropped game. |
| `R3-EC-03` | Fresh-audit rule | No pilot-credit verdict exists; the four Cargo manifests' pre-state and parent pilot lists are recorded, and every applicable surface has fresh R3 evidence. |
| `R3-EC-04` | Complete verdict coverage | Every game × C-01…C-08 aggregate cell and every listed sub-surface has one accepted verdict, owner, evidence, rollback/reversal, and next trigger. |
| `R3-EC-05` | Correct shared ownership | `engine-core` remains generic/noun-free; `game-stdlib` owns count structure only; `game-test-support` owns test geometry/profile metadata only; `wasm-api` owns import aliases; behavior remains in games. |
| `R3-EC-06` | C-01 public neutrality | Four public constructor migrations preserve payload, visibility, order, effect bytes/hash, replay checkpoints and exports. |
| `R3-EC-07` | C-01 private neutrality | Plain private constructor migration preserves owner IDs, filtered deal effects and every no-leak surface; the other three private N/As are evidenced and create no synthetic private class. |
| `R3-EC-08` | C-02 typed grammar | Plain's typed parser delegates strict grammar to canonical Rust authority and maps only indices 0/1; all malformed and out-of-game values reject as characterized. |
| `R3-EC-09` | C-02 boundary compatibility | Shared import aliases remain import-only; all four canonical output paths are unchanged; Flood/Frontier/Event no-parser exceptions name `wasm-api` and exclude domain IDs. |
| `R3-EC-10` | C-03 roster structure | Four roster predicates use `SeatCount` structure while exact diagnostics, setup acceptance, state, replay and behavior remain unchanged. |
| `R3-EC-11` | C-03 profile diversity | Flood variant count and role-order cardinality plus Frontier/Event variant counts use separate structural migrations; role/faction identity/order and variant policy remain game-owned. |
| `R3-EC-12` | C-03 no false generalization | Plain variant enforcement, all ranges, all ring/rotation helpers and faction/role policy are explicitly N/A/excepted; no new setup rule is introduced. |
| `R3-EC-13` | C-04 representative coverage | V1 vectors cover the named trick, cooperative action, graph branch, and event/edict trees without legal-choice, metadata, preview, label or branch-order drift. |
| `R3-EC-14` | C-05 byte discipline | The selected action-tree v1 framing is explicit/versioned; the shipped writer is used only through that surface; adjacent state/effect/view/replay/export/diagnostic bytes are unchanged or accepted exceptions. |
| `R3-EC-15` | Hash migration discipline | Every touched surface is classified `unchanged`, `parallel-new-surface`, or separately admitted `intentional-migration`; no broad regeneration or silent local-hash replacement occurs. |
| `R3-EC-16` | Legacy readability | Existing traces, fixtures, local action-tree hashes, WASM imports and existing exports remain valid for their compatibility windows. |
| `R3-EC-17` | Dev-only boundary | All four games list `game-test-support` only under `[dev-dependencies]`; inverse normal-edge proof and boundary checks pass. |
| `R3-EC-18` | Geometry-only harness | The no-leak helper enumerates only caller-supplied facts/viewers/surfaces and cannot project, redact, reveal, authorize, execute, score or choose. |
| `R3-EC-19` | Plain Tricks no-leak | Both source seats × observer/owner/opponent × every applicable view/tree/diagnostic/effect/public export/seat-private export/bot surface pass before and after reveal; hidden tail remains absent. |
| `R3-EC-20` | Flood Watch public-observer guard | Hidden future deck facts are absent for observer and both seats across view/tree/diagnostic/effect/export/bot surfaces; forecast/drawn cards become public only through existing game rules. |
| `R3-EC-21` | Frontier Control C-07 N/A | Characterization proves no hidden source or viewer-dependent projection; existing observer/seat equality and public-effect/export tests remain intact and specific. |
| `R3-EC-22` | Event Frontier public-observer guard | Deeper deck facts remain absent across every public surface; current/next/resolved cards appear exactly when existing game policy authorizes them. |
| `R3-EC-23` | Canary hygiene | No canary appears in any committed trace, fixture, export, log, snapshot, test ID, DOM, storage, accessibility artifact or screenshot. |
| `R3-EC-24` | C-08 profile separation | All five drivers reject wrong profile/version/owner/visibility/fields and delegate only after metadata succeeds; no permissive union profile is introduced. |
| `R3-EC-25` | C-08 real callers | Four replay, setup, domain and public-export profiles invoke real game evidence; Plain invokes both seat-private viewers; the three seat-private N/As are explicit. |
| `R3-EC-26` | Setup/domain center | Every standard and named variant fixture is assigned setup/domain fields and a game-owned semantic validator; no behavior migrates into a profile driver or tool. |
| `R3-EC-27` | ADR 0004 preservation | Plain public and seat-private exports remain correctly viewer-scoped; Flood/Event public exports omit hidden deck tails; Frontier remains public; no omniscient state is reconstructed. |
| `R3-EC-28` | Fixture/data boundary | Fixtures remain typed evidence/parameters only; no selector, trigger, condition, formula, script, loop, procedural mutation or rule behavior enters data. |
| `R3-EC-29` | Tool ownership | `replay-check`, `fixture-check`, and `rule-coverage` remain validator/dispatch owners only; no game behavior moves into tools. |
| `R3-EC-30` | C-09 identity | Plain/Flood/Event shared sampler adoptions preserve RNG outputs, rejection draw counts, full deck sequences and all downstream state/effect/view/replay/export hashes; Frontier is evidenced N/A. |
| `R3-EC-31` | C-10 behavioral non-promotion | Every trick, role/faction, graph/connectivity, event/edict, budget/resource, projection/redaction, scoring and outcome decision remains in its game; register receipts explicitly reject promotion. |
| `R3-EC-32` | Full test health | Focused/tool/workspace suites pass without deleted, ignored or weakened tests and without unrelated benchmark, catalog or UI drift. |
| `R3-EC-33` | C-11 ownership truth | R3 closes as the third bounded parent EC-28 wave; R4 remains separately owned and unimplemented. |
| `R3-EC-34` | Gate 18 sequencing | Consistent with parent EC-30, Gate 18 remains blocked behind R4 and all stated interlocks; only the R3 row changes status. |

## 7. Acceptance evidence

### 7.1 Required command set

Run from repository root. Record command, exit status, relevant output summary,
and changed-artifact inventory in the characterization report.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings

cargo test -p engine-core
cargo test -p game-stdlib
cargo test -p game-test-support
cargo test -p wasm-api
cargo test -p plain_tricks
cargo test -p flood_watch
cargo test -p frontier_control
cargo test -p event_frontier

cargo test -p replay-check
cargo test -p fixture-check
cargo test -p rule-coverage
cargo test --workspace

cargo run -p replay-check -- --game plain_tricks --all
cargo run -p replay-check -- --game flood_watch --all
cargo run -p replay-check -- --game frontier_control --all
cargo run -p replay-check -- --game event_frontier --all

cargo run -p fixture-check -- --game plain_tricks
cargo run -p fixture-check -- --game flood_watch
cargo run -p fixture-check -- --game frontier_control
cargo run -p fixture-check -- --game event_frontier

cargo run -p rule-coverage -- --game plain_tricks
cargo run -p rule-coverage -- --game flood_watch
cargo run -p rule-coverage -- --game frontier_control
cargo run -p rule-coverage -- --game event_frontier

bash scripts/boundary-check.sh
cargo tree --workspace -e normal --invert game-test-support

node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
```

`check-catalog-docs.mjs` is a regression guard only. R3 adds no game, public
rules entry, web renderer, or smoke layer. No browser screenshot or manual UI
acceptance is required.

### 7.2 Focused evidence by migration class

| Class | Exact evidence |
|---|---|
| C-01 public/private | Before/after envelope equality, debug/stable rendering, effect order/hash, owner IDs, filtered payloads for observer/seat0/seat1, replay checkpoints and export equality. |
| C-02 parser/boundary | Canonical and malformed vectors; out-of-game index rejection; import alias acceptance; canonical output snapshots; no TypeScript normalization; no trace/golden diff. |
| C-03 setup counts | Accepted count 2; rejected 0/1/3 where callable; exact diagnostic code/message; variant count and role-order cardinality; setup/state/RNG/effect/replay/view equality; faction/role policy exception proof. |
| C-04/C-05 | Exact v1 bytes/hashes for named trees; local hash equality; no legal-choice/segment/label/metadata/tag/preview/freshness/branch-order drift; no adjacent hash change. |
| C-06 | Four Cargo manifests plus inverse normal-edge output before/after. |
| C-07 | Matrix dimensions, source facts, viewers, surfaces, expected cases, zero structured failures, retained focused tests, and in-memory-only canary proof; Frontier N/A/equality evidence. |
| C-08 | Valid metadata; wrong-profile/version/owner/visibility/field rejection; real game/tool adapter invocation; existing artifact byte/hash equality; selected fixture fields; explicit seat-private N/As. |
| C-09 | Fixed RNG words including rejection cases, draw counts, full deck/order outputs, hands/forecast/current-next state, and every downstream hash/visibility surface. |
| C-10 | Atlas/register review and changed-code audit proving all named behavior remains game-owned. |

### 7.3 Characterization anchors

At admission, re-pin at minimum the following exact artifacts and relevant Rust
tests/symbols. The report must record current digests measured from the
executing tree, not copy values from this plan.

**Plain Tricks**

- `tests/golden_traces/deal-private-no-leak.trace.json`
- `tests/golden_traces/no-leak-public-observer.trace.json`
- `tests/golden_traces/seat-private-view.trace.json`
- `tests/golden_traces/public-replay-export-import.trace.json`
- `tests/golden_traces/follow-suit-forced.trace.json`
- `tests/golden_traces/invalid-must-follow-diagnostic.trace.json`
- `tests/golden_traces/off-suit-never-wins.trace.json`
- `tests/golden_traces/round-close-deal-rotation.trace.json`
- `tests/golden_traces/trick-winner-leads-next.trace.json`
- `tests/golden_traces/void-free-discard.trace.json`
- `tests/golden_traces/terminal-most-points-win.trace.json`
- `tests/golden_traces/bot-action.trace.json`
- standard fixture, setup shuffle/deal vectors, effect strings, typed seat
  parser vectors, local action-tree hashes, viewer exporter and bot input.

**Flood Watch**

- `tests/golden_traces/public-observer-no-leak.trace.json`
- `tests/golden_traces/public-replay-export-import.trace.json`
- `tests/golden_traces/forecast-public-reveal.trace.json`
- `tests/golden_traces/scenario-deluge-setup.trace.json`
- `tests/golden_traces/budget-exhaustion-auto-environment.trace.json`
- `tests/golden_traces/levee-absorption.trace.json`
- `tests/golden_traces/loss-by-inundation.trace.json`
- `tests/golden_traces/role-power-levee-warden.trace.json`
- `tests/golden_traces/role-power-pumpwright.trace.json`
- `tests/golden_traces/bot-coop-full-game.trace.json`
- standard/deluge fixtures, exact role-order/count diagnostics, event-deck
  shuffle/forecast vectors, local action-tree hashes and public exporter.

**Frontier Control**

- `tests/golden_traces/replay-export-import.trace.json`
- `tests/golden_traces/highlands-setup.trace.json`
- `tests/golden_traces/round-scoring-breakdown.trace.json`
- `tests/golden_traces/supply-cut-scores-zero.trace.json`
- `tests/golden_traces/muster-and-reinforce-caps.trace.json`
- `tests/golden_traces/non-adjacent-move-diagnostic.trace.json`
- `tests/golden_traces/clash-crew-into-guards.trace.json`
- `tests/golden_traces/clash-guard-into-crews.trace.json`
- `tests/golden_traces/bot-vs-bot-full-game.trace.json`
- standard/highlands fixtures, exact faction/setup validation, public view
  equality, local action-tree hashes and fully public exporter.

**Event Frontier**

- `tests/golden_traces/replay-export-import-no-deck-leak.trace.json`
- `tests/golden_traces/hard-winter-setup.trace.json`
- `tests/golden_traces/land-rush-setup.trace.json`
- `tests/golden_traces/event-choice-resolves-card.trace.json`
- `tests/golden_traces/edict-activation-and-expiry.trace.json`
- `tests/golden_traces/edict-blocks-action-diagnostic.trace.json`
- `tests/golden_traces/op-full-multi-site.trace.json`
- `tests/golden_traces/reckoning-scoring-breakdown.trace.json`
- `tests/golden_traces/bot-vs-bot-full-game.trace.json`
- standard/hard-winter/land-rush fixtures, exact faction/setup validation,
  epoch shuffle/current-next/deeper-tail vectors, local action-tree hashes,
  public exporter and bot surfaces.

### 7.4 Golden, fixture, export, and diff policy

**Default authorized changes to existing golden traces, fixtures, snapshots, or
export bytes: none.**

Authorized additions are focused Rust tests, test-local profile adapters,
action-tree v1 vectors, the characterization report, register receipts and the
R3 tracker closeout. Planned constructor, parser, count, parallel tree,
no-leak, profile and sampler migrations are expected to preserve every existing
committed evidence byte.

An existing trace, fixture, export, snapshot, seat spelling or declared hash may
change only if `/reassess-spec` adds a separately named row before
implementation that states:

1. exact artifact, selected surface and owner;
2. why unchanged or parallel treatment is impossible;
3. ADR-0009 `intentional-migration` classification;
4. old and new profile/schema/hash-surface versions and exact digests;
5. non-empty migration/update note;
6. compatibility reader and time-bounded window;
7. ADR-0004 viewer/visibility proof where applicable;
8. exact validator and no-leak commands; and
9. one-artifact rollback.

“Regenerate all”, “update snapshots”, accepting an entire directory diff, and
opportunistic reformatting are forbidden. An unexpected diff is a stop
condition, not an invitation to bless new bytes.

### 7.5 Register before/after and required reviews

Before the first migration, capture `MSC-8C-001…010` including the landed R1
and R2 receipt tables. At closeout, show only appended R3 receipts and prove no
existing decision or non-promotion boundary was weakened.

Required human review areas:

- the complete C-03 variant/role/faction split and every retained game policy;
- Plain Tricks' private effects, pairwise matrix and both export classes;
- Flood Watch/Event Frontier hidden-future-deck expectations and reveal points;
- Frontier Control C-07 N/A rather than a vacuous secret test;
- all setup/domain field selections and semantic owners;
- all three C-09 rejection-draw and full-deck identity proofs;
- every N/A and exception rationale;
- changed-file inventory and zero unauthorized golden/fixture/export diffs;
- R3-only register and tracker changes.

### 7.6 External-research sharpening

External sources do not establish repository state and do not widen scope.
They reinforce the already-accepted local doctrine only:

- Cargo documents development dependencies as dependencies used for tests,
  examples and benchmarks rather than ordinary package builds, supporting the
  C-06 manifest rule plus inverse-tree enforcement rather than convention
  alone.[^ext-cargo-dev]
- RFC 8785 explains why repeatable hashing requires an invariant canonical
  representation, reinforcing explicit versioned action-tree bytes instead of
  hashing incidental serializer output.[^ext-rfc8785]
- Protocol Buffers' own documentation warns that deterministic serialization
  is not necessarily canonical or stable across implementations/versions,
  reinforcing Rulepath's distinct canonical-byte authority, surface version,
  compatibility and rollback rules.[^ext-protobuf]

These sources authorize no new serialization format, fixture schema, or shared
behavior.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Principles engaged

| Authority | R3 stance |
|---|---|
| `FOUNDATIONS.md` product priority | Correctness, deterministic replay and no-leak proof outrank deduplication. A helper adoption is rejected if behavioral or byte identity cannot be proved. |
| `FOUNDATIONS.md` §11 determinism | Replay, hashes, RNG consumption, serialization order, seat spellings, traces, fixtures and exports are named evidence surfaces. |
| `FOUNDATIONS.md` §11 hidden information | Plain private hands and Flood/Event hidden deck tails are tested across every relevant viewer and surface; public artifacts never gain unauthorized facts. |
| `FOUNDATIONS.md` §12 stop conditions | Any leak, unexpected artifact diff, RNG drift, noun contamination, hidden normal dependency, weakened test or behavior movement stops the affected task. |
| `FOUNDATIONS.md` §13 | A new evidence class, incompatible canonical byte change, visibility ambiguity or cross-owner behavior requirement is a blocking ADR trigger. |
| `ARCHITECTURE.md` | Rust remains sole behavior authority; the narrowest lawful owner wins; TypeScript presents only. |
| `ENGINE-GAME-DATA-BOUNDARY.md` | Shared code carries generic envelopes/counts/bytes/test geometry/metadata only; games retain every rule and semantic fixture check. |
| `MECHANIC-ATLAS.md` | Empty promotion debt permits the retrofit; §10/§10B keep trick, graph, public ledger, event and hidden-deck decisions local/deferred as recorded. |
| ADR 0004 | Internal-full evidence, public observer export and labelled seat-private export stay distinct. |
| ADR 0008/register | Register-first, behavior-free extraction only; C-10 rejects promotion. |
| ADR 0009/evidence contract | Per-surface characterization, explicit profile/visibility/owner, migration classification, compatibility and rollback. |
| Multi-seat/surface contract | Fixed count does not collapse viewer or surface enumeration; role/faction identity is not generic seat structure. |
| AI/bot contract | Bots receive only viewer-authorized information; explanations/candidate rankings cannot leak hidden state. |
| Agent discipline | Bounded tasks, one surface per diff, valid-test/SUT diagnosis and no weakening. |

### 8.2 Ownership decisions

- **`engine-core`:** reuse `EffectEnvelope`, canonical `SeatId`, action-tree v1,
  stable writer/replay hash types and unbiased-index APIs. Add no game noun,
  policy, or new helper contract.
- **`game-stdlib`:** reuse `SeatCount` as structural cardinality only. It does
  not know exact game counts, roles, factions, leaders, turn order or setup
  composition.
- **`game-test-support`:** dev-only pairwise enumeration and profile metadata
  validation. It does not know what a hidden fact means or how a fixture
  scores.
- **`games/plain_tricks`:** owns deck/deal/tail, legal cards/follow suit, trick
  winner/next leader, round/terminal scoring, private projection/effects,
  exports and bot inputs.
- **`games/flood_watch`:** owns roles/order/powers, event deck/forecast,
  environment pressure, levees/flood/inundation, budget and outcome.
- **`games/frontier_control`:** owns factions, sites/edges, adjacency/movement,
  clash, muster/reinforce, supply/connectivity and scoring.
- **`games/event_frontier`:** owns factions, sites/trails, epoch deck,
  current/next reveal, event/edict resolution, eligibility, operations,
  funding/income/caps, Reckoning and outcome.
- **`wasm-api`:** owns import-only legacy seat aliases and transport
  compatibility; it does not repair game state or decide legality.
- **Tools:** validate and dispatch only.
- **Static data:** typed content, setup parameters and evidence inputs only.

### 8.3 Behavioral non-promotion and visibility stance

The unit's defining safety rule is stronger than “all tests pass”: no selected
helper may decide a game fact.

- Plain Tricks alone decides deal shape, follow-suit legality, off-suit winner,
  trick lifecycle, winner-leads-next, trick-count scoring and when a card is
  public.
- Flood Watch alone decides role order/powers, forecast and event pressure,
  levee absorption, inundation/loss, budget use and cooperative outcome.
- Frontier Control alone decides site/edge topology, legal adjacency/movement,
  clash, unit caps, supply/connectivity and round/final scoring.
- Event Frontier alone decides trails, event/edict resolution, eligibility,
  operation funding, pass/Reckoning income, resource caps and scoring.
- Each game alone supplies C-07 source facts, probes, reveal expectations and
  C-08 semantic adapters.

A proposed helper that answers any of those questions is rejected, even if its
call signature appears reusable.

### 8.4 Blocking ADR-trigger note

No foundation, area-contract or ADR amendment is expected. Stop the affected
implementation and flag a maintainer decision under `FOUNDATIONS.md` §13 if
reassessment finds any of the following:

- ADR 0004 cannot classify an actual Plain/Flood/Event export without
  ambiguity;
- a required migration changes public, viewer-scoped, seat-private or
  internal-dev authority;
- a canonical byte/hash change cannot preserve a compatibility reader/window;
- setup/domain evidence requires executable fixture behavior or a new DSL;
- a profile driver or tool would need to infer trick, graph, event, budget,
  role/faction, scoring or reveal policy;
- a shared owner needs a game noun or semantic rule;
- C-03 cannot preserve the current accepted/rejected setup set and diagnostics;
- C-09 shared/local samplers differ in output or RNG consumption; or
- an output-seat migration cannot be isolated from state, visibility or hash
  semantics.

The spec must not design around such a gap, silently amend doctrine, or convert
it into a broad implementation ticket.

## 9. Forbidden changes

The following are independently disqualifying, even when the full test suite
happens to pass:

1. adding `board`, `card`, `deck`, `hand`, `trick`, `role`, `faction`, `site`,
   `edge`, `graph`, `levee`, `event`, `edict`, `budget`, `resource`, `score`, or
   equivalent game/domain nouns to `engine-core`;
2. adding or changing a shared helper contract instead of adopting the shipped
   C-01…C-10 surfaces;
3. moving deal/shuffle construction, reveal timing, follow-suit, trick winner,
   next leader, role/faction assignment, graph adjacency/connectivity,
   event/edict resolution, budget/resource accounting, scoring or outcome out
   of its game;
4. making `game-test-support` a normal or build dependency, or importing it
   into production library code;
5. allowing a profile driver, fixture validator, replay tool or generic harness
   to call game transitions in order to decide expected semantics;
6. creating a private effect, private view or seat-private exporter for Flood
   Watch, Frontier Control or Event Frontier merely to make a helper apply;
7. treating Frontier Control's fully public state as a hidden-information test
   by inserting an artificial secret;
8. broadening Plain Tricks' owner view/export, or exposing a hidden hand/tail
   card to observer/opponent/public output;
9. exposing Flood Watch's future event deck or Event Frontier's deeper deck in
   view, effect, diagnostic, bot or export surfaces before the game authorizes
   public reveal;
10. changing current/next event visibility, forecast visibility, effect
    filtering or export redaction as an incidental scaffolding edit;
11. accepting legacy seat aliases in game/kernel canonical parsers or adding
    seat normalization to TypeScript;
12. applying C-02 to card, suit, rank, district, event, site or faction IDs;
13. using `SeatCountRange` or ring helpers when no true range/ring structural
    surface exists, or adding a new Plain variant-count rejection rule;
14. replacing an existing local action-tree hash, state hash, effect hash, view
    hash, replay hash or export hash without a separately admitted migration;
15. modifying action-tree choices, order, labels, accessibility labels,
    metadata, tags, preview or freshness while adding v1 evidence;
16. changing RNG seeds, word consumption, rejection behavior, loop bounds,
    shuffle order, deal order, epoch partition, forecast or current/next window;
17. editing existing golden traces, fixtures, snapshots or exports without the
    exact §7.4 migration packet;
18. running or accepting blanket regeneration, snapshot update or formatting
    across unrelated files;
19. placing conditions, selectors, triggers, formulas, scripts, loops,
    behavior IDs, procedural mutations or hidden defaults in static data;
20. writing no-leak canaries into committed artifacts or browser-accessible
    surfaces;
21. deleting, ignoring, loosening or replacing a specific game assertion with
    a weaker generic assertion;
22. combining multiple games or selected surfaces into one implementation diff;
23. editing foundation docs or accepted ADRs to rationalize an implementation
    that current doctrine rejects;
24. changing `docs/ROADMAP.md`, game admission docs, public rules, web catalog,
    renderers or smoke lists for this non-game retrofit; or
25. marking R3 `Done` before all §6 rows and §7 evidence pass.

## 10. Documentation updates required

| Document / artifact | Required update | Applicability |
|---|---|---|
| `reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md` | Create and maintain the complete before/after evidence ledger described in §4.2. | Required. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Append R3 receipts under existing `MSC-8C-001…010`; preserve R1/R2 rows and all non-promotion decisions. | Required at closeout. |
| `specs/README.md` | After all exit criteria pass, change only the `8C-R3` row to `Done`, add completion date/evidence, and leave `8C-R4`/Gate 18 pending. | Required at closeout. |
| This spec | Reassess, accept, save under `specs/`, decompose into tickets, then archive only after completion. | Required workflow artifact. |
| `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`, boundary/area docs | No planned amendment. A discovered gap is a blocking §8.4 ADR-trigger, not an opportunistic edit. | Not applicable by default. |
| ADR 0004/0008/0009 or a new ADR | No planned amendment. | Not applicable unless §8.4 fires and maintainers resolve it before implementation. |
| `docs/ROADMAP.md` | Progress is tracked in `specs/README.md`, not ROADMAP. | Not applicable. |
| Per-game `RULES`, `MECHANICS`, `AI`, `UI`, `SOURCES`, release/admission docs | No behavior, evidence obligation or public feature changes. | Not applicable. |
| `templates/**` | Existing templates govern tickets/evidence; R3 adds no template contract. | Not applicable. |
| `apps/web/README.md` | No game/web surface, catalog entry, renderer or smoke layer is added. | **Not applicable.** |

Documentation edits are receipts of accepted implementation, never authority to
retroactively legalize an out-of-bounds code change.

## 11. Sequencing

### 11.1 External sequence

1. **Predecessor:** `8C-R2` is `Done` at the target commit.
2. **Current:** author, reassess, decompose, execute and close `8C-R3` only.
3. **Next:** `8C-R4` remains the next seed after R3 closes.
4. **Then:** Gate 18 remains blocked until R1…R4 and all stated atlas/ADR/profile
   interlocks are closed, explicitly not applicable or accepted-excepted.

The R3 spec may document successor sequencing but may not create successor
implementation work.

### 11.2 Internal dependency order

The required order is:

1. provenance/determination and complete characterization (`001–002`);
2. C-01 envelope constructors (`101–105`);
3. C-02 typed parser and shared boundary conformance (`201–202`);
4. C-03 count predicates, one predicate per diff (`301–308`);
5. C-04/C-05 parallel action-tree v1 (`401–404`);
6. C-06 dev-only dependencies (`501–504`) before C-07/C-08 tests import the
   support crate;
7. C-07 visibility work (`511–514`);
8. C-08 replay, setup, domain, public-export and seat-private-profile waves
   (`601–642`), with domain evidence kept separate from setup evidence;
9. C-09 sampler replacements (`701–703`) only after setup/RNG baselines are
   frozen; and
10. consolidation, register, full acceptance and tracker closeout (`801–804`).

A game may proceed independently once its prerequisites and characterization
are complete, but no task may skip the profile/visibility/hash/RNG evidence
that guards its selected surface.

### 11.3 Admission rule for a diff

A diff is admitted only when one sentence can name exactly one selected surface,
its current owner, its target existing helper, its expected classification, its
complete evidence set and its isolated rollback. “All four games”, “scaffolding
cleanup”, “normalize profiles”, “fix fixtures”, “convert hashes” or “update
snapshots” is not an admissible diff description.

A task that discovers a second required migration stops, records the finding,
and returns it to `/reassess-spec` or a separately bounded ticket. It does not
expand itself.

### 11.4 Closeout order

The status flip is last. The characterization report and register must already
show complete accepted results; all §7 commands must have been rerun on the
final tree; the changed-file inventory must show no unauthorized artifact
diff; and human review must accept every N/A/exception and C-10 rejection. Only
then may `specs/README.md` change R3 to `Done`.

## 12. Assumptions

### 12.1 One-line-correctable assumptions

- `assumption:` the unit slug/label defaults to
  `8c-r3-public-coop-asymmetric-trick-scaffolding`; the fixed unit ID is
  `8C-R3`.
- `assumption:` the accepted path is
  `specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md`; this document is the
  pre-decomposition artifact for `/reassess-spec` → `/spec-to-tickets`.
- `assumption:` the owner remains “Rulepath maintainers” until reassessment
  assigns named ticket owners.
- `assumption:` task IDs are planning identifiers and may be renamed or split,
  never merged across games, profiles, visibility classes or selected surfaces.
- `assumption:` each R3 game enforces a fixed `STANDARD_SEAT_COUNT` at the bare
  roster predicate; `flood_watch`, `frontier_control`, and `event_frontier`
  add variant/role/faction validation on top. If a grounded reassessment finds
  a true seat range, correct only that game's C-03 row and note the correction;
  do not expand scope.
- `assumption:` Plain Tricks' current setup does not separately reject a
  mismatched `variant.seat_count`; adding such a rule remains out of scope
  unless exact target code disproves this characterization.
- `assumption:` the shipped seat-private envelope API is
  `EffectEnvelope::private_to`; exact local wrapper names are one-line
  correctable.
- `assumption:` Flood Watch, Frontier Control and Event Frontier have no
  game-local typed seat parser; their C-02 work is shared-boundary conformance,
  not new seat types.
- `assumption:` existing traces, fixtures, snapshots and exports are read-only
  by default; C-08 profile adapters wrap current validators/artifacts rather
  than rewriting them.
- `assumption:` one physical setup fixture may feed separate setup/domain
  adapters without becoming a union schema or procedural rules file.
- `assumption:` Plain Tricks' existing viewer-scoped exporter is a real caller
  for both public and labelled seat-private profiles; Flood/Frontier/Event have
  no official seat-private export.
- `assumption:` C-04/C-05 add a parallel action-tree v1 surface while all four
  local action-tree hashes remain authoritative during the compatibility
  window.
- `assumption:` the three local C-09 algorithms are semantically and
  consumption-identical to `next_index_unbiased_v1`; any vector/draw-count
  divergence blocks that game and restores the local helper.
- `assumption:` no foundation/ADR amendment is required; any discovered need
  invokes §8.4.
- `assumption:` existing replay/fixture/rule-coverage dispatch supports the four
  games. If one thin dispatch is missing, reassessment creates a separate
  profile/game dispatch task rather than burying it in a game migration.
- `assumption:` the analysis baseline commit `be1af6f` is the repository HEAD at
  authoring (verified during reassessment).

### 12.2 Reassessment corrections that do not reopen scope

`/reassess-spec` may correct a helper spelling, exact function/test-module
owner, test name, trace list/digest, profile visibility value, validator
owner/dispatch status, register row identifier, normal dependency pre-state,
or whether an existing exporter supports import round-trip for a selected
viewer. It may split a candidate task further to preserve one-surface-per-diff.

It may not add a game; drop a game; create a new private/export surface to
force applicability; promote behavior; merge setup and domain profiles; merge
public and seat-private evidence; replace a local hash; authorize a broad byte
migration; weaken a no-leak rule; absorb `8C-R4`; or begin Gate 18.

A grounded contradiction is recorded as a correction in the reassessed spec
and its affected matrix row. The locked unit and four-game set remain fixed.

### 12.3 Repository evidence basis

The principal repository authorities and seams are:

- **Authority/workflow:** [`docs/README.md`][docs-readme],
  [`docs/FOUNDATIONS.md`][foundations], [`docs/ARCHITECTURE.md`][architecture],
  [`docs/ENGINE-GAME-DATA-BOUNDARY.md`][boundary],
  [`docs/ROADMAP.md`][roadmap], [`specs/README.md`][spec-index].
- **Scaffolding/mechanics/migration:** [`docs/MECHANIC-ATLAS.md`][atlas],
  [`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`][register],
  [ADR 0004][adr-0004], [ADR 0008][adr-0008], [ADR 0009][adr-0009].
- **Evidence/seat/agent law:** [testing/replay/benchmarking][testing],
  [Trace Schema v1][trace-schema], [Evidence Fixture Contract][fixture-contract],
  [AI/Bots][ai-bots], [Multi-Seat and Surface Contract][multi-seat],
  [Agent Discipline][agent-discipline].
- **Parent and direct precedents:** [Unit 8C parent][parent], [R1 spec][r1-spec],
  [R2 spec][r2-spec], [source change-plan report][change-plan],
  [R1 characterization][r1-char], [R2 characterization][r2-char].
- **Shared homes:** [engine envelope/seat APIs][engine-lib],
  [action-tree v1][engine-action], [stable writer/replay][engine-replay],
  [unbiased RNG][engine-rng], [seat structure][stdlib-seat],
  [no-leak geometry][test-no-leak], [profile drivers][test-profiles],
  [WASM seats][wasm-seats].
- **Validators/guards:** [replay-check][replay-check],
  [fixture-check][fixture-check], [rule-coverage][rule-coverage],
  [boundary-check][boundary-check], [doc links][doc-links],
  [catalog docs][catalog-docs].
- **Plain Tricks:** [Cargo][pt-cargo], [effects][pt-effects], [IDs][pt-ids],
  [setup][pt-setup], [replay support][pt-replay], [visibility][pt-visibility],
  [visibility tests][pt-vis-tests], [standard fixture][pt-fixture].
- **Flood Watch:** [Cargo][fw-cargo], [effects][fw-effects], [IDs][fw-ids],
  [setup][fw-setup], [replay support][fw-replay], [visibility][fw-visibility],
  [visibility tests][fw-vis-tests], [standard fixture][fw-fixture-standard],
  [deluge fixture][fw-fixture-deluge].
- **Frontier Control:** [Cargo][fc-cargo], [effects][fc-effects], [IDs][fc-ids],
  [setup][fc-setup], [replay support][fc-replay], [visibility][fc-visibility],
  [visibility tests][fc-vis-tests], [standard fixture][fc-fixture-standard],
  [highlands fixture][fc-fixture-highlands].
- **Event Frontier:** [Cargo][ef-cargo], [effects][ef-effects], [IDs][ef-ids],
  [setup][ef-setup], [replay support][ef-replay], [visibility][ef-visibility],
  [visibility tests][ef-vis-tests], [standard fixture][ef-fixture-standard],
  [hard-winter fixture][ef-fixture-hard-winter],
  [land-rush fixture][ef-fixture-land-rush].

The complete mandatory floor was also reviewed:

- all of `docs/**`, including `OFFICIAL-GAME-CONTRACT.md`,
  `UI-INTERACTION.md`, `IP-POLICY.md`, `SOURCES.md`,
  `WASM-CLIENT-BOUNDARY.md`, `archival-workflow.md`, ADRs 0001–0007, and
  `ADR-TEMPLATE.md` in addition to the load-bearing files above; and
- all of `templates/**`: `AGENT-TASK.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`,
  `COMPETENT-PLAYER.md`, `GAME-AI.md`, `GAME-BENCHMARKS.md`,
  `GAME-EVIDENCE.md`, `GAME-HOW-TO-PLAY.md`,
  `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-MECHANICS.md`,
  `GAME-RULE-COVERAGE.md`, `GAME-RULES.md`, `GAME-SOURCES.md`, `GAME-UI.md`,
  `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, and
  `README.md`.

### 12.4 Final self-check before acceptance

The accepted form must answer **yes** to every question:

- Is R3 confirmed rather than re-decided, with R2 `Done`, atlas debt empty, and
  EC-28/EC-30 sequencing documented?
- Are exactly `plain_tricks`, `flood_watch`, `frontier_control`, and
  `event_frontier` present, with no fifth game?
- Does every C-01…C-08 cell and every sub-surface have one permitted verdict?
- Is every applicable surface a fresh R3 audit, with no pilot-credit column or
  residual framing?
- Does C-03 preserve setup/profile diversity instead of collapsing roles,
  factions and variants into a uniform predicate?
- Are `setup-evidence-v1` and `domain-evidence-v1` central, separately owned,
  and backed by every standard/variant fixture?
- Is Plain Tricks' full seat-private C-07 matrix present?
- Are Flood Watch and Event Frontier public-observer/hidden-deck matrices
  present, with seat-private rows explicit N/A?
- Is Frontier Control C-07 explicitly N/A while public-view equality tests are
  retained?
- Do all four C-04/C-05 rows add parallel action-tree v1 without replacing a
  local hash or changing legal trees?
- Are all four C-06 edges dev-only?
- Are Plain/Flood/Event C-09 migrations explicit and Frontier N/A, with exact
  draw-count identity required?
- Does every trick, graph, event, budget/resource, role/faction, projection,
  scoring and outcome rule remain game-owned?
- Is every byte/hash/seat/visibility/RNG-touching task characterized,
  classified, compatible and independently reversible?
- Are existing goldens, fixtures, snapshots and exports unchanged by default,
  with no blanket regeneration?
- Are specific per-game tests preserved alongside generic geometry/profile
  tests?
- Is `engine-core` noun-free and `game-test-support` absent from normal/build
  dependency graphs?
- Is `apps/web/README.md` explicitly not applicable?
- Do `8C-R4` and Gate 18 remain untouched and sequenced after R3?

[^ext-cargo-dev]: Rust Project, *The Cargo Book — Development dependencies*, https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies
[^ext-rfc8785]: RFC Editor, *RFC 8785 — JSON Canonicalization Scheme*, https://www.rfc-editor.org/rfc/rfc8785.html
[^ext-protobuf]: Protocol Buffers Documentation, *Proto Serialization Is Not Canonical*, https://protobuf.dev/programming-guides/serialization-not-canonical/

[adr-0004]: ../docs/adr/0004-hidden-info-replay-export-taxonomy.md
[adr-0008]: ../docs/adr/0008-mechanical-scaffolding-governance.md
[adr-0009]: ../docs/adr/0009-replay-fixture-hash-taxonomy.md
[agent-discipline]: ../docs/AGENT-DISCIPLINE.md
[ai-bots]: ../docs/AI-BOTS.md
[architecture]: ../docs/ARCHITECTURE.md
[atlas]: ../docs/MECHANIC-ATLAS.md
[boundary]: ../docs/ENGINE-GAME-DATA-BOUNDARY.md
[boundary-check]: ../scripts/boundary-check.sh
[catalog-docs]: ../scripts/check-catalog-docs.mjs
[change-plan]: ../reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md
[doc-links]: ../scripts/check-doc-links.mjs
[docs-readme]: ../docs/README.md
[ef-cargo]: ../games/event_frontier/Cargo.toml
[ef-effects]: ../games/event_frontier/src/effects.rs
[ef-fixture-hard-winter]: ../games/event_frontier/data/fixtures/event_frontier_hard_winter.fixture.json
[ef-fixture-land-rush]: ../games/event_frontier/data/fixtures/event_frontier_land_rush.fixture.json
[ef-fixture-standard]: ../games/event_frontier/data/fixtures/event_frontier_standard.fixture.json
[ef-ids]: ../games/event_frontier/src/ids.rs
[ef-replay]: ../games/event_frontier/src/replay_support.rs
[ef-setup]: ../games/event_frontier/src/setup.rs
[ef-vis-tests]: ../games/event_frontier/tests/visibility.rs
[ef-visibility]: ../games/event_frontier/src/visibility.rs
[engine-action]: ../crates/engine-core/src/action.rs
[engine-lib]: ../crates/engine-core/src/lib.rs
[engine-replay]: ../crates/engine-core/src/replay.rs
[engine-rng]: ../crates/engine-core/src/rng.rs
[fc-cargo]: ../games/frontier_control/Cargo.toml
[fc-effects]: ../games/frontier_control/src/effects.rs
[fc-fixture-highlands]: ../games/frontier_control/data/fixtures/frontier_control_highlands.fixture.json
[fc-fixture-standard]: ../games/frontier_control/data/fixtures/frontier_control_standard.fixture.json
[fc-ids]: ../games/frontier_control/src/ids.rs
[fc-replay]: ../games/frontier_control/src/replay_support.rs
[fc-setup]: ../games/frontier_control/src/setup.rs
[fc-vis-tests]: ../games/frontier_control/tests/visibility.rs
[fc-visibility]: ../games/frontier_control/src/visibility.rs
[fixture-check]: ../tools/fixture-check/src/main.rs
[fixture-contract]: ../docs/EVIDENCE-FIXTURE-CONTRACT.md
[foundations]: ../docs/FOUNDATIONS.md
[fw-cargo]: ../games/flood_watch/Cargo.toml
[fw-effects]: ../games/flood_watch/src/effects.rs
[fw-fixture-deluge]: ../games/flood_watch/data/fixtures/flood_watch_deluge.fixture.json
[fw-fixture-standard]: ../games/flood_watch/data/fixtures/flood_watch_standard.fixture.json
[fw-ids]: ../games/flood_watch/src/ids.rs
[fw-replay]: ../games/flood_watch/src/replay_support.rs
[fw-setup]: ../games/flood_watch/src/setup.rs
[fw-vis-tests]: ../games/flood_watch/tests/visibility.rs
[fw-visibility]: ../games/flood_watch/src/visibility.rs
[multi-seat]: ../docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md
[parent]: ../archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md
[pt-cargo]: ../games/plain_tricks/Cargo.toml
[pt-effects]: ../games/plain_tricks/src/effects.rs
[pt-fixture]: ../games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json
[pt-ids]: ../games/plain_tricks/src/ids.rs
[pt-replay]: ../games/plain_tricks/src/replay_support.rs
[pt-setup]: ../games/plain_tricks/src/setup.rs
[pt-vis-tests]: ../games/plain_tricks/tests/visibility.rs
[pt-visibility]: ../games/plain_tricks/src/visibility.rs
[r1-char]: ../reports/8c-r1-public-fixed-seat-scaffolding-characterization.md
[r1-spec]: ../archive/specs/8c-r1-public-fixed-seat-scaffolding.md
[r2-char]: ../reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md
[r2-spec]: ../archive/specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md
[register]: ../docs/MECHANICAL-SCAFFOLDING-REGISTER.md
[replay-check]: ../tools/replay-check/src/main.rs
[roadmap]: ../docs/ROADMAP.md
[rule-coverage]: ../tools/rule-coverage/src/main.rs
[spec-index]: ../specs/README.md
[stdlib-seat]: ../crates/game-stdlib/src/seat.rs
[test-no-leak]: ../crates/game-test-support/src/no_leak.rs
[test-profiles]: ../crates/game-test-support/src/profiles.rs
[testing]: ../docs/TESTING-REPLAY-BENCHMARKING.md
[trace-schema]: ../docs/TRACE-SCHEMA-v1.md
[wasm-seats]: ../crates/wasm-api/src/seats.rs
