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
| 6 | Gate 8 | `high_card_duel` / `blackjack_lite` | Chance and hidden-information proof. |
| 7 | Gate 9 | `token_bazaar` / `resource_race` | Original resource/economy microgame. |
| 8 | Gate 9 | `secret_draft` | Simultaneous commitment/reveal proof. |
| 9 | Gate 10 | `poker_lite` | Imperfect-information accounting/bot proof. |
| 10 | Gate 10 | `plain_tricks` | Classic card-game depth. |
| 11 | Gate 11 | `masked_claims` | Bluffing/reaction-window proof. |
| 12 | Gate 12 | `flood_watch` | Cooperative event pressure. |
| 13 | Gate 13 | `frontier_control` | Asymmetric area-control proof. |
| 14 | Gate 14 | `event_frontier` | Highest public complexity before private red-team. |
| Appendix | Gate P | Private monster-game red-team | Late, isolated, optional, non-public. |

Every stage and gate must satisfy [OFFICIAL-GAME-CONTRACT.md](OFFICIAL-GAME-CONTRACT.md) for any official game and the universal invariants in [FOUNDATIONS.md](FOUNDATIONS.md).

> Implementation progress and the per-gate spec for each gate are tracked in [`../specs/README.md`](../specs/README.md). The ladder above is law; that index is the mutable progress tracker. This document is not edited to record which gates are done.

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

## 3. Product mechanic ladder

| Stage | Candidate | First proves | Product role | Primitive pressure |
|---:|---|---|---|---|
| 0 | skeleton | workspace, CI, docs, ADR placement, WASM smoke | foundation only | initialize process |
| 1 | `race_to_n` / Nim | setup, turn order, flat legal actions, validation, commands, effects, replay, random bot, WASM path | plumbing proof | local-only |
| 2 | `three_marks` | placement, fixed positions, occupancy, simple pattern win/draw, SVG board | first pleasant board UI | record local spatial/pattern shapes |
| 3 | `column_four` | gravity placement, legal columns, previews, line detection, effect-driven drop/win animation | first showcase | compare fixed coordinate/line pressure |
| 4 | `directional_flip` | directional scans, bracketed grouped changes, pass/no-move if scoped, multi-piece effects | richer abstract board game | third-use coordinate/scan decision |
| 5 | `draughts_lite` | movement, capture, mandatory capture, forced continuation, action trees | serious compound-action proof | movement/capture inventory |
| 6 | `high_card_duel` / `blackjack_lite` | deterministic shuffle, private views, filtered logs/effects, no-leak serialization | hidden-information safety proof | card/zone local until repeated |
| 7 | `token_bazaar` / `resource_race` | resources, payments, score economy, cleanup phases, valuation bot | original portfolio microgame | resource/accounting candidate later |
| 8 | `secret_draft` | commitments, reveal, waiting states, simultaneous resolution, drafting | private-view waiting UX | commitment/reveal candidate later |
| 9 | `poker_lite` | betting, pots, public/private cards, simple showdown, imperfect-info policy | accounting + imperfect-info bot proof | cards/resources pressure increases |
| 10 | `plain_tricks` | lead/follow constraints, tricks, round scoring, deal rotation | classic card-game depth | card/trick helpers candidate |
| 11 | `masked_claims` | claims, challenges, pending responses, conditional resolution | reaction-window proof | reaction helper requires pressure/ADR |
| 12 | `flood_watch` | shared outcome, event pressure, role powers, automation, multi-action budgets | cooperative original game | event/role local |
| 13 | `frontier_control` | graph maps, control, asymmetry, faction-specific actions/scoring | asymmetric strategy showcase | graph/control candidates only after pressure |
| 14 | `event_frontier` | event decks, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees | highest public complexity | every repeated shape resolved |

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

## 10. Gate 8: cards, chance, hidden information

Purpose: prove deterministic shuffle, private views, viewer-filtered logs/effects, no-leak serialization, and bots acting from allowed private views.

Recommended first candidate: `high_card_duel`. Add `blackjack_lite` only if it adds useful pressure without derailing polish.

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

## 15. Gate P: private monster-game red-team

Admit: Gate 14 is complete and public Rulepath is coherent without private work.

Purpose: stress-test architecture without driving public architecture.

Rules:

- private repo/submodule/local-only folder only;
- no public build;
- no public CI dependency;
- no public docs naming licensed games;
- no public assets, traces, screenshots, card text, scenarios, IDs, or module names;
- strict kernel-contamination review;
- public atlas may record generic pressure only without private names or proprietary details.

Exit: no kernel contamination; missing abstractions are documented without private names; performance is measurable; public Rulepath can abandon the experiment without damage.
