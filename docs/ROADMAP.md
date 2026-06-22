# Rulepath Roadmap

Status: prescriptive staged ladder and build-gate law. A stage or gate may be skipped or reordered only by accepted ADR.

The roadmap builds a polished public playable site while earning mechanic complexity through observable public games. It is not a disguised path toward private licensed games and not a promise of arbitrary tabletop support.

V1/v2 exclude hosted multiplayer, accounts, databases, matchmaking, chat, ranked play, DSL work, YAML, public MCTS/ISMCTS/Monte Carlo bots, ML/RL, and private monster-game work.

## 1. Stage/gate crosswalk

| Ladder stage | Build gate | Game / focus | Public role |
|---:|---|---|---|
| 0 | Gate 0 | Repository skeleton | No gameplay. |
| 1 | Gates 1-3 | `race_to_n` / Nim-like tiny game, trace/replay hardening, WASM shell | Plumbing proof. |
| 2 | Gate 4 | `three_marks` | First pleasant board smoke. |
| 3 | Gate 5 | `column_four` | First true showcase target. |
| 4 | Gate 6 | `directional_flip` | Directional scanning and grouped effects; extraction decision. |
| 5 | Gate 7 | `draughts_lite` | Compound action tree proof. |
| 5M | Gate 7.1 | `board_space` primitive conformance | Mandatory promotion-debt closure before the next mechanic-ladder gate. |
| 6 | Gate 8 | `high_card_duel` | Chance and hidden-information proof. |
| 7 | Gate 9 | `token_bazaar` / `resource_race` | Original resource/economy microgame. |
| 8 | Gate 9 | `secret_draft` | Simultaneous commitment/reveal proof. |
| 9 | Gate 10 | `poker_lite` | Imperfect-information accounting/bot proof. |
| 10 | Gate 10 | `plain_tricks` | Classic card-game depth. |
| 11 | Gate 11 | `masked_claims` | Bluffing/reaction-window proof. |
| 12 | Gate 12 | `flood_watch` | Cooperative event pressure. |
| 13 | Gate 13 | `frontier_control` | Asymmetric area-control proof. |
| 14 | Gate 14 | `event_frontier` | Highest public complexity before private red-team. |
| 15P | Phase 0 | Foundation realignment and next-phase admission | Documentation/law pass for the public scaling phase. |
| 15A | Infra A-D | N-seat setup, simulator, shell, and no-leak infrastructure | Public scaling interlocks before the next game gate. |
| 15 | Gate 15 | `river_ledger` / Texas Hold'Em rules family | First official N-seat hidden-information betting game. |
| 15.1 | Gate 15.1 | River Ledger all-in / side-pot extension | Multi-way allocation and side-pot rationale. |
| 16 | Gate 16 | Hearts | Fixed four-seat trick-taking. |
| 17 | Gate 17 | Oh Hell | Variable-N bidding and trick-taking. |
| 18 | Gate 18 | Spades | Partnerships, teams, and grouped outcomes. |
| 19 | Gate 19 | Five Hundred Rummy | Public meld tableau plus private hands. |
| 20 | Gate 20 | Star Halma / Chinese Checkers family | Larger topology and board-surface scaling. |
| 21 | Gate 21 | Pachisi-family race | Track topology, deterministic chance, capture/safety. |
| 22 | Gate 22 | Four Winds Melds | Scoped meld/reaction proof with concealed state. |
| 23 | Gate 23 | Commonwealth Frontier | Original medium-heavy public capstone. |
| Tail | Gate P | Private monster-game red-team | Last, isolated, optional, non-public, non-architectural. |

Every stage and gate must satisfy [OFFICIAL-GAME-CONTRACT.md](OFFICIAL-GAME-CONTRACT.md) for any official game and the universal invariants in [FOUNDATIONS.md](FOUNDATIONS.md).

> Implementation progress and the per-gate spec for each gate are tracked in [`../specs/README.md`](../specs/README.md). The ladder above is law; that index is the mutable progress tracker. This document is not edited to record which gates are done.

Pre-Gate-18 debt interlock: before opening Gate 18 implementation work, the
current gate spec must confirm that mechanical scaffolding debt and trace debt
from Gates 15-17 are either closed, explicitly not applicable, or deferred by an
accepted authority. Mechanical scaffolding decisions are governed by
[ADR 0008](adr/0008-mechanical-scaffolding-governance.md) and
[MECHANICAL-SCAFFOLDING-REGISTER.md](MECHANICAL-SCAFFOLDING-REGISTER.md).
Replay, fixture, export, and hash-surface debt is governed by
[ADR 0009](adr/0009-replay-fixture-hash-taxonomy.md),
[TESTING-REPLAY-BENCHMARKING.md](TESTING-REPLAY-BENCHMARKING.md),
[TRACE-SCHEMA-v1.md](TRACE-SCHEMA-v1.md), and
[EVIDENCE-FIXTURE-CONTRACT.md](EVIDENCE-FIXTURE-CONTRACT.md).

## 2. Per-stage requirements

Each official game stage MUST produce or verify:

- typed Rust rules;
- source notes and original rules summary;
- rule coverage matrix;
- mechanic inventory;
- repo mechanic atlas update;
- unit/rule/golden/property/simulation/replay/serialization tests;
- CLI simulation;
- benchmark coverage;
- random legal bot;
- UI metadata;
- replay support;
- UI smoke tests once web-exposed;
- boundary review: Rust behavior authority, TypeScript no legality, `engine-core` noun-free, static data not behavior, replay deterministic, bots fair, hidden information safe, IP clean.

A third repeated mechanic shape MUST resolve through the primitive-pressure ledger before proceeding.

When that resolution promotes a `game-stdlib` primitive, all previous official games using the promoted shape MUST be migrated, audited not applicable, or explicitly excepted. Open promotion debt is a maintenance interlock: it is specced and closed before the next new mechanic-ladder gate unless an accepted ADR says otherwise.

Every new mechanic-ladder gate must include a per-gate debt review before
implementation starts: mechanic-atlas pressure, mechanical scaffolding debt,
trace debt, fixture-profile debt, seat/viewer grammar debt, replay/hash
migration debt, and evidence-receipt blockers. The review may be short, but it
must name the governing ledger, register, ADR, receipt, or `not applicable`
rationale. This debt review does not reorder the product mechanic ladder and is
not implementation progress tracking.

Cross-game public UI infrastructure specs, such as the shared How to Play /
Rules surface, may be tracked outside the mechanic gate ladder. They do not
introduce a new mechanic gate unless explicitly accepted as a roadmap item.

## 3. Product mechanic ladder

| Stage | Candidate | First proves | Product role | Primitive pressure |
|---:|---|---|---|---|
| 0 | skeleton | workspace, CI, docs, ADR placement, WASM smoke | foundation only | initialize process |
| 1 | `race_to_n` / Nim | setup, turn order, flat legal actions, validation, commands, effects, replay, random bot, WASM path | plumbing proof | local-only |
| 2 | `three_marks` | placement, fixed positions, occupancy, simple pattern win/draw, SVG board | first pleasant board UI | record local spatial/pattern shapes |
| 3 | `column_four` | gravity placement, legal columns, previews, line detection, effect-driven drop/win animation | first showcase | compare fixed coordinate/line pressure |
| 4 | `directional_flip` | directional scans, bracketed grouped changes, pass/no-move if scoped, multi-piece effects | richer abstract board game | third-use coordinate/scan decision |
| 5 | `draughts_lite` | movement, capture, mandatory capture, forced continuation, action trees | serious compound-action proof | movement/capture inventory |
| 6 | `high_card_duel` | deterministic shuffle, private views, filtered logs/effects, no-leak serialization | hidden-information safety proof | card/zone local until repeated; `blackjack_lite` deferred by [ADR 0006](adr/0006-blackjack-lite-roadmap-placement.md) |
| 7 | `token_bazaar` / `resource_race` | resources, payments, score economy, cleanup phases, valuation bot | original portfolio microgame | resource/accounting candidate later |
| 8 | `secret_draft` | commitments, reveal, waiting states, simultaneous resolution, drafting | private-view waiting UX | commitment/reveal candidate later |
| 9 | `poker_lite` | betting, pots, public/private cards, simple showdown, imperfect-info policy | accounting + imperfect-info bot proof | cards/resources pressure increases |
| 10 | `plain_tricks` | lead/follow constraints, tricks, round scoring, deal rotation | classic card-game depth | card/trick helpers candidate |
| 11 | `masked_claims` | claims, challenges, pending responses, conditional resolution | reaction-window proof | reaction helper requires pressure/ADR |
| 12 | `flood_watch` | shared outcome, event pressure, role powers, automation, multi-action budgets | cooperative original game | event/role local |
| 13 | `frontier_control` | graph maps, control, asymmetry, faction-specific actions/scoring | asymmetric strategy showcase | graph/control candidates only after pressure |
| 14 | `event_frontier` | event decks, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees | highest public complexity | every repeated shape resolved |
| 15 | `river_ledger` / Texas Hold'Em rules family | 3-6 seats, 52-card deck, multi-street betting, showdown explanation, split winners | first public N-seat hidden-information betting game | card/deck/private-hand/outcome/accounting pressure |
| 15.1 | River Ledger all-in / side pots | partial eligibility, nested pots, all-in contribution caps, multi-way allocation explanation | scoped allocation extension | public resource accounting and edge-case golden-trace pressure |
| 16 | Hearts | fixed 4-seat trick-taking, pass direction, follow-suit, negative scoring | classic fixed N-seat trick-taking | trick-taking and N-seat private-hand pressure |
| 17 | Oh Hell | 3-7 seats, dealer rotation, bids/contracts, changing hand size | variable-N trick-taking/bidding proof | trick-taking promotion decision pressure |
| 18 | Spades | partnership pairs, team scoring, contract evaluation, grouped UI | team/partnership proof | team outcome and partnership visibility pressure |
| 19 | Five Hundred Rummy | draw/discard piles, public meld tableau, private hands, multi-round target | larger card-zone/action-surface proof | meld/tableau pressure |
| 20 | Star Halma / Chinese Checkers family | 121-space star board, long jump chains, multi-seat spatial race | larger board/topology proof | topology/path/jump helper hard gate |
| 21 | Pachisi-family race | track topology, dice/chance, safe/capture spaces, multiple pawns per seat | public track-race proof | track topology and deterministic chance pressure |
| 22 | Four Winds Melds | draw/discard rhythm, exposed/concealed sets, discard-claim priority | scoped meld/reaction proof | reaction-window and wall/concealed-set pressure |
| 23 | Commonwealth Frontier | 3-4 asymmetric factions, 24-36 sites, event queues, public tracks | medium-heavy public capstone | graph/site/faction/resource/event/outcome pressure resolved before Gate P |

## 4. Gate 0: skeleton

Admit: no implementation exists.

Build shape:

- Rust workspace;
- placeholder `engine-core`, `game-stdlib`, `ai-core`, `wasm-api`;
- React/TypeScript web shell;
- tool placeholders;
- docs and ADR folder;
- CI smoke.

Exit:

- workspace smoke tests run;
- web shell builds;
- placeholder WASM loads;
- foundation docs are present;
- `engine-core` contains only generic contracts.

Not allowed:

- real mechanics in `engine-core`;
- YAML behavior;
- DSL work;
- hosted services;
- private-game names.

## 5. Gates 1-3: tiny game, trace hardening, static web shell

### Gate 1: `race_to_n` / Nim-like tiny game

Purpose: prove setup, legal actions, validation, command application, terminal detection, effects, replay, bot simulation, and WASM without hiding architecture mistakes.

Exit:

- human vs random bot works in CLI and web;
- 100,000 native random games complete without crash;
- replay reproduces hashes;
- invalid/stale diagnostics are tested;
- per-game docs and mechanic inventory exist.

Not allowed: generalized piles, decks, boards, tracks, resources, multiplayer, polished-renderer detour.

### Gate 2: trace, replay, and benchmark hardening

Build:

- trace serialization;
- replay checker;
- stable hashes;
- benchmark harness;
- failure seed/command output;
- seed-reduction plan;
- fixture validation.

Exit: failing simulations can be replayed from seed and command log; golden traces fail loudly on drift; benchmark reports include version, command, environment, and thresholds.

### Gate 3: WASM/static web shell

Build:

- batched WASM API;
- game picker;
- match setup;
- public view store;
- action tree store;
- effect queue;
- replay controls;
- dev toggle;
- safe local replay import/export.

Exit: static site plays the tiny game with no backend; human vs bot, hotseat where applicable, bot-vs-bot replay, and replay viewer work; no legality exists in TypeScript.

## 6. Gate 4: `three_marks`

Purpose: prove fixed spatial positions, occupancy, direct legal target highlighting, simple pattern detection, draw states, and a clean SVG board.

Exit:

- occupied positions are never legal;
- win/draw detection is covered;
- random and Level 1 bots exist;
- UI is pleasant and accessible where practical;
- spatial/pattern mechanics are recorded but not extracted.

Not allowed: grid primitive in `engine-core`; speculative `game-stdlib` extraction from one game.

## 7. Gate 5: `column_four` public polish

Purpose: make the first “Rulepath is real” public milestone.

Status: completed on 2026-06-06. The mutable progress index is
[`../specs/README.md`](../specs/README.md); this note records the accepted Gate 5
evidence without changing the ladder.

Proves:

- gravity-constrained placement;
- legal columns;
- hover/drop previews;
- line detection under gravity;
- effect-log-driven drop animation;
- win-line effects;
- baseline and preferably Level 2 policy bot.

Exit:

- public page feels polished;
- legal columns only are clickable;
- previews are Rust-safe;
- animations come from semantic effects;
- bot explanations are available for non-random bot;
- replay viewer smoke passes;
- benchmark and UI smoke coverage exists;
- mechanic atlas records repeated coordinate/line pressure.

Completion evidence:

- `cargo test --workspace`
- `cargo run -p simulate -- --game column_four --games 1000`
- `cargo run -p replay-check -- --game column_four --all`
- `cargo run -p fixture-check -- --game column_four`
- `cargo run -p rule-coverage -- --game column_four`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:e2e`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`

Not allowed: debug-first public screen, TypeScript legality, early Canvas/PixiJS without evidence, `engine-core` grid nouns.

## 8. Gate 6: `directional_flip` and extraction decision

Purpose: prove directional scanning, bracketed changes, grouped effects, multi-piece animation, greedy/policy bot, and real grid/pattern extraction pressure.

Decision required: compare `three_marks`, `column_four`, and `directional_flip`. The primitive-pressure ledger MUST decide whether coordinate/line/directional helpers are reused, promoted to a narrow `game-stdlib` helper, deferred/rejected, or escalated to ADR.

Exit:

- all flips/changes are represented in effects or visible child details;
- replay reconstructs consequences;
- extraction decision is documented;
- any helper is typed, narrow, tested, documented, back-ported, benchmarked, and not in `engine-core`.

Not allowed: untyped directional selectors in data; grid concepts in `engine-core`.

## 9. Gate 7: `draughts_lite` action trees

Purpose: introduce origin/destination movement, capture, forced moves, multi-step continuations, promotion if scoped, and progressive action construction.

Exit:

- action trees work in CLI and web;
- forced continuations replay correctly;
- UI guides path construction clearly;
- baseline bot follows forced rules;
- legal tree and bot benchmarks exist.

Not allowed: full chess exception load, generic movement in `engine-core`, search without benchmarks.


## 9A. Gate 7.1: board-space primitive back-port and promotion-debt closure

Purpose: close the promotion debt created when Gate 7 promoted `game-stdlib::board_space` but intentionally did not force-retrofit earlier board games. This is a maintenance/interlock gate, not a new public game and not a new mechanic-ladder stage.

Scope:

- Audit every official game against the promoted board-space primitive.
- Retrofit `three_marks`, `column_four`, and `directional_flip` to use `game-stdlib::board_space` for the behavior-free coordinate/rectangular-board subset where it applies.
- Record `race_to_n` as not applicable if the audit confirms it has no board-space mechanic.
- Keep `draughts_lite` as the exemplar/regression target for board-space use.
- Preserve public behavior, action path strings, action ordering, diagnostics, semantic effects, stable summaries, replay hashes, golden traces, UI anchors, bot legality, visibility, benchmarks, and WASM surfaces unless an existing bug is proven and the spec explicitly authorizes a migration.
- Update the mechanic atlas and affected per-game mechanic/ledger docs so future spec authors can discover the conformance rule without chat context.

Exit:

- `three_marks`, `column_four`, and `directional_flip` depend on `game-stdlib` and no longer maintain local coordinate/cell/bounds/indexing behavior that duplicates `board_space` within the promoted scope.
- Game-local semantics remain local: Three Marks lines, Column Four column actions/gravity/four-in-a-row scans, Directional Flip ray bracketing/flips/forced pass, and Draughts Lite movement/capture/promotion/forced continuation are not promoted.
- All existing native tests, replay/golden-trace checks, visibility tests, serialization tests, benchmarks, and web smoke tests for affected games pass with stable behavior by default.
- `race_to_n` is audited and documented as not applicable, or the gate stops with evidence if the audit contradicts that assumption.
- The atlas open promotion-debt register has no open `board_space` debt.

Not allowed: new `engine-core` board/grid/cell vocabulary; broad generic board-game engines; generic occupancy boards; generic line-win, gravity, directional-flip, ray-capture, movement, capture, promotion, or bot-strategy helpers; TypeScript legality; trace/hash migration by accident; implementation-ticket decomposition inside the spec.

## 10. Gate 8: cards, chance, hidden information

Purpose: prove deterministic shuffle, private views, viewer-filtered logs/effects, no-leak serialization, and bots acting from allowed private views.

Required candidate: `high_card_duel`.

`blackjack_lite` is not a Gate 8 or Gate 8.1 implementation target. It is a deferred comparison case under [ADR 0006](adr/0006-blackjack-lite-roadmap-placement.md); reconsider it only after Gate 9 resource/accounting and simultaneous-choice pressure has landed, and only with a scoped non-casino naming/IP plan. If a draw/stand threshold proof is needed earlier, propose an original non-casino microgame by spec/ADR instead of treating Blackjack as mandatory.

Exit:

- hidden identities never leak through views, logs, previews, serialization, bot views, UI payloads, DOM fixtures, local storage, or replay exports;
- seed replay reproduces draws;
- bots simulate many games legally;
- hidden-info docs and tests are complete.

Not allowed: sending hidden state to browser, omniscient bots, proprietary card text.

## 11. Gate 9: resources, simultaneous choice, drafting

Purpose: prove resource effects, payments, gains, scoring economy, waiting states, reveal phases, and private-view bot choices.

Build candidates:

- `token_bazaar` / `resource_race` for original resource economy;
- `secret_draft` for simultaneous commitment and drafting.

Exit:

- resource accounting is effect-visible;
- costs/previews come from Rust;
- simultaneous choices remain hidden until reveal;
- UI shows pending seats without leaking choices;
- bots use allowed views;
- invariant/no-leak tests and benchmarks pass.

Not allowed: static data formulas for payments; hidden choices in DOM/local storage; actual hidden-state sampling by bots.

## 12. Gate 10: betting and tricks

Purpose: add betting and trick-taking after hidden info, resources, action trees, and card zones are proven.

Build candidates:

- `poker_lite` with scoped betting/showdown;
- `plain_tricks` with scoped lead/follow/trick scoring.

Variant scope MUST be written before coding.

Exit:

- betting/trick rules are correct for chosen variants;
- pot/accounting and follow-suit tests cover edge cases;
- bots finish games without hidden-state cheating;
- no public MCTS/ISMCTS is used;
- UI remains understandable;
- native benchmarks exist.

Not allowed: real-money/casino features, unbounded variants, hidden-state cheating, ML/RL, copied rules prose.

## 13. Gate 11: bluffing and reactions

Purpose: prove claims, challenges, pending responses, reaction windows, conditional resolution, cancellation/replacement if scoped, and no-leak logs.

Exit:

- logs explain who may respond and why;
- bots respond legally;
- hidden claims do not leak;
- reaction UI smoke tests pass;
- any reaction-window abstraction has repeated pressure or ADR before promotion.

Not allowed: trademark-forward hidden-role names, proprietary role/card text, generic reaction window in `engine-core`.

## 14. Gates 12-14: higher public complexity

### Gate 12: `flood_watch`

Purpose: original cooperative event-pressure game.

Proves shared win/loss, event deck pressure, role powers, environment automation, multi-action budgets, scenario setup, and cooperative bot baseline.

Exit: automation is deterministic and effect-log-driven; shared outcome is tested; role powers stay game-local; UI explains event pressure clearly.

### Gate 13: `frontier_control`

Purpose: original graph-map/area-control microgame.

Proves graph topology, control, asymmetry, faction-specific legal actions/scoring, per-faction UI, and per-faction bots.

Exit: no faction nouns enter `engine-core`; each faction has random and baseline bot; simulations produce useful metrics; effect logs stay readable.

### Gate 14: `event_frontier`

Purpose: highest public complexity before any private red-team work.

Proves event decks with exceptions, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees, scripted policy bots, scenario setup, and long-game replay/debug tools.

Exit: public Rulepath stands without private experiments; action trees remain usable; scripted bots are demo-coherent; every repeated mechanic has ledger resolution.

Not allowed: private licensed content, DSL by stealth, architecture claims beyond proven games.

## 15. Public scaling phase

Admit: Gate 14 is complete, ADR 0007 is accepted, and the Phase 0 foundation
realignment has recorded the N-seat/larger-surface contract in the foundation
document set and templates.

Purpose: prove 3+ official seats, larger public surfaces, N-player hidden
information safety, multi-seat UI presentation, larger benchmark envelopes, and
larger outcome explanations through public, IP-safe games before any private
monster-game red-team work can influence architecture.

This phase is public-first and requirements-first. The living status tracker is
[`../specs/README.md`](../specs/README.md); this roadmap records the ladder law,
not per-spec progress.

### Phase 0: foundation realignment and next-phase admission

Purpose: complete the documentation and template realignment needed before
Gate 15+ specs can be grounded.

Exit:

- ADR 0007 is accepted and records the public scaling phase plus Gate P tail
  placement;
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` exists and is indexed;
- foundation, area, template, ADR-note, source/IP, discipline, and archival docs
  carry N-seat/larger-surface clarifications without changing kernel boundaries,
  trace schema fields, WASM exported API, replay/hash contracts, bot policy law,
  or no-leak law;
- `docs/ROADMAP.md` records this public scaling phase and moves Gate P to the
  tail;
- `specs/README.md` records Phase 0 completion and refreshes Gate 15+
  interlock notes.

Not allowed: Rust/WASM/tool/game code, trace/schema migration, YAML/DSL work,
kernel noun growth, or public/private content-policy relaxation.

### Infra A-D: N-seat public infrastructure interlocks

Purpose: close cross-cutting public infrastructure assumptions before the first
N-seat game gate.

Required units:

- Infra A: N-seat setup and catalog metadata; Rust owns seat-count acceptance,
  setup validation, and variant metadata, while TypeScript only presents it.
- Infra B: N-seat simulator summaries; summary output uses deterministic
  indexed maps instead of two-seat-only counters.
- Infra C: multi-seat shell frame; seat rails, active/pending seats, observer
  mode, and viewer selection render Rust/WASM-projected state only.
- Infra D: N-player no-leak test harness; pairwise private-datum by viewer by
  surface assertions cover browser payloads, DOM, storage, logs, bot
  explanations, candidate rankings, and replay exports where applicable.

Exit:

- N-seat setup/catalog, simulator, shell, and no-leak harness specs are complete
  with evidence;
- no TypeScript legality or turn-order inference is introduced;
- hidden information remains viewer-safe across every public/browser/replay/bot
  surface;
- benchmark and smoke evidence names the supported seat counts and max-surface
  fixtures it covers.

### Gate 15: River Ledger / Texas Hold'Em rules family

Purpose: first official N-seat hidden-information betting game.

Scope: 3-6 seats, standard 52-card deck, deterministic shuffle, two private
hole cards per seat, community cards, multi-street fixed-limit capped-raise
betting, showdown explanation, split winners, and original neutral Rulepath
presentation.

Exit:

- setup accepts and rejects the documented seat range deterministically;
- public and private views prove N-player no-leak, including public observer
  and replay export surfaces;
- betting state, legal actions, contribution accounting, terminal conditions,
  showdown rationale, and split results are covered by rules, tests, traces,
  simulations, replay/hash checks, serialization checks, and benchmarks;
- bots use only legal action APIs and authorized seat views;
- UI shows seat order, active/pending seats, safe previews, showdown/final
  breakdown, and viewer-safe explanations without casino trade dress.

Not allowed: real-money/casino features, tournament/product mimicry, hidden
card/deck leakage, omniscient bots, public MCTS/ISMCTS/Monte Carlo/ML/RL, or
side-pot/all-in scope unless explicitly admitted by Gate 15.1.

### Gate 15.1: River Ledger all-in and side-pot extension

Purpose: prove multi-way partial eligibility and allocation accounting after
the base Hold'Em rules are stable.

Exit:

- all-in contribution caps, side-pot construction, eligibility, remainders,
  split winners, and terminal explanations are covered by named rules, tests,
  golden traces, replay/hash checks, no-leak checks, and benchmarks;
- the outcome surface explains every public allocation without revealing
  private cards that remain hidden to a viewer;
- accounting stays in typed Rust and does not become static-data behavior.

### Gate 16: Hearts

Purpose: prove fixed four-seat trick-taking with full private hands.

Exit:

- passing, lead/follow obligations, trick capture, round scoring, match
  accumulation, and shoot-the-moon or scoped equivalent are covered for the
  chosen variant;
- four-seat private-hand no-leak holds across views, UI, replay exports, logs,
  storage, and bot explanations;
- trick-taking helper pressure is recorded and resolved or deferred through the
  mechanic atlas before later gates depend on it.

### Gate 17: Oh Hell

Purpose: generalize trick-taking from fixed four seats to variable N with
contract/bid pressure.

Exit:

- official seat range, dealer rotation, changing hand size, bidding order,
  last-bidder constraint, trick play, scoring, and terminal standings are
  covered;
- simulations and benchmarks record results by seat count;
- trick-taking and bidding helper decisions are resolved through the primitive
  pressure process.

### Gate 18: Spades

Purpose: prove teams, partnerships, grouped UI, and team outcome explanations.

Exit:

- team assignment, partnership visibility, bidding/contracts, trick scoring,
  nil-style risk if scoped, bag/penalty rules if scoped, and terminal outcomes
  are covered;
- UI and outcome surfaces clearly distinguish per-seat and per-team facts;
- no hidden or teammate-only information leaks to unauthorized viewers.

### Gate 19: Five Hundred Rummy

Purpose: prove public meld tableau plus private hands and larger card-action
surfaces outside trick-taking.

Exit:

- draw/discard, public melds, laying off if scoped, private hands, scoring,
  round/match flow, and terminal results are covered;
- action affordances remain usable with larger hand/tableau surfaces;
- meld/tableau primitive pressure is recorded and resolved or deferred.

### Gate 20: Star Halma / Chinese Checkers family

Purpose: prove a larger public board surface and topology/path pressure without
hidden cards.

Exit:

- official seat variants, 121-space topology, move/jump chains, blocked-path
  behavior, win conditions, replay, serialization, and benchmarks are covered;
- renderer performance and accessibility are proven for the largest official
  board fixture;
- topology/path helper pressure is resolved before the next topology-dependent
  gate.

### Gate 21: Pachisi-family race

Purpose: prove track topology, deterministic chance, capture/safety spaces, and
multiple pieces per seat.

Exit:

- source/IP notes choose a stable public-domain rules source and original
  presentation;
- dice/chance, track movement, capture, safety, entry/home rules, partnerships
  if scoped, no-leak if any private state exists, and outcome explanations are
  covered;
- track topology and deterministic chance pressure are recorded.

### Gate 22: Four Winds Melds

Purpose: prove scoped Mahjong-family draw/discard rhythm, concealed state, and
multi-opponent reaction priority without importing a sprawling clone.

Exit:

- wall/deck draw, discard, exposed and concealed sets, claim priority,
  reaction windows, scoring, and terminal explanations are covered for a small
  original scoped variant;
- wall/concealed information does not leak through public/browser/replay/bot
  surfaces;
- reaction-window hard-gate and meld/zone pressure are resolved or explicitly
  deferred.

### Gate 23: Commonwealth Frontier

Purpose: prove the medium-heavy public ambition ceiling with an original
asymmetric game.

Scope: 3-4 asymmetric factions, 24-36 sites, regional adjacency, several public
tracks, original events, public resource accounting, event queues, periodic
scoring, faction-specific actions, and explainable outcomes.

Exit:

- graph topology, site control, faction asymmetry, resource accounting, event
  timing, periodic scoring, large view payloads, replay, serialization,
  benchmarks, UI, bots, and outcome explanations are all covered;
- all armed mechanic-atlas interlocks from Gate 15+ are resolved, promoted,
  deferred, rejected, or escalated to ADR before Gate P;
- public Rulepath can stand on public, IP-safe evidence without private
  monster-game assumptions.

Not allowed across the public scaling phase: private licensed content, copied
rules prose or trade dress, YAML/DSL behavior, TypeScript legality, public
MCTS/ISMCTS/Monte Carlo/ML/RL bots, kernel noun growth, hidden-state leakage,
or private work shaping public architecture.

## 16. Gate P: private monster-game red-team

Admit: Gate 23 is complete, all public scaling-phase interlocks are resolved,
and public Rulepath is coherent without private work.

Purpose: stress-test architecture without driving public architecture.

Rules:

- last tail item only;
- optional and non-architectural;
- private repo/submodule/local-only folder only;
- no public build;
- no public CI dependency;
- no public docs naming licensed games;
- no public assets, traces, screenshots, card text, scenarios, IDs, or module names;
- strict kernel-contamination review;
- public atlas may record generic pressure only without private names or proprietary details.

Exit: no kernel contamination; missing abstractions are documented without
private names; performance is measurable; public Rulepath can abandon the
experiment without damage.
