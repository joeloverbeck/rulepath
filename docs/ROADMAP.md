# Rulepath Roadmap

Status: staged mechanic ladder, build-gate order, and public product law.

This document merges two complementary views of the same progression:

- the **product mechanic ladder** — what each official game first proves and its public product role;
- the **build gate order** — the order in which infrastructure and games are built and the admit/build/exit conditions per gate.

The two numberings deliberately differ: the build gates insert non-game gates (skeleton, trace/replay hardening, WASM/web shell) that have no mechanic-ladder equivalent. The crosswalk below maps them. A stage or gate may be skipped only by ADR.

The ladder builds a polished public playable site while earning mechanic complexity through observable stages. It is not a disguised path toward private licensed games and not a promise of arbitrary tabletop support.

V1/v2 excludes hosted multiplayer, accounts, databases, matchmaking, DSL work, YAML, public MCTS/ISMCTS/Monte Carlo bots, ML/RL, and private monster-game work.

## Stage/gate crosswalk

| Ladder stage | Build gate(s) | Game / focus |
|---:|---|---|
| 0 | Gate 0 | repository skeleton |
| 1 | Gate 1, Gate 2, Gate 3 | `race_to_n` / Nim, then trace/replay/benchmark hardening and the WASM/web shell |
| 2 | Gate 4 | `three_marks` |
| 3 | Gate 5 | `column_four` |
| 4 | Gate 6 | `directional_flip` + earned extraction decision |
| 5 | Gate 7 | `draughts_lite` |
| 6 | Gate 8 | `high_card_duel` / `blackjack_lite` |
| 7 | Gate 9 | `token_bazaar` / `resource_race` |
| 8 | Gate 9 | `secret_draft` / commitment microgame |
| 9 | Gate 10 | `poker_lite` |
| 10 | Gate 10 | `plain_tricks` |
| 11 | Gate 11 | `masked_claims` |
| 12 | Gate 12 | `flood_watch` |
| 13 | Gate 13 | `frontier_control` |
| 14 | Gate 14 | `event_frontier` |
| Appendix | Gate P | private monster-game red-team |

## Per-stage gate (every stage and gate)

Every stage and gate must satisfy the [universal acceptance invariants](INVARIANTS.md) and, in addition, produce/verify the per-game deliverables:

- typed Rust rules;
- per-game docs and source/IP notes;
- `MECHANICS.md` and repo mechanic-atlas update;
- rule coverage matrix;
- unit/rule/golden/property/simulation/replay/serialization tests;
- CLI simulation;
- benchmark coverage;
- random legal bot;
- UI metadata;
- UI smoke tests once web-exposed;
- updated boundary review (Rust owns behavior, TypeScript owns no legality, `engine-core` noun-free, static data typed content/parameters only, replay deterministic, bots use legal APIs and allowed views, IP boundaries preserved).

A third repeated mechanic shape must resolve through the primitive-pressure ledger before proceeding.

Stop conditions for the whole repository are defined in `docs/FOUNDATIONS.md` §12.

---

# Part A — Product mechanic ladder

| Stage | Candidate | First proves | Product role |
|---:|---|---|---|
| 0 | repository skeleton | workspace, CI, docs, ADRs, WASM smoke | not gameplay |
| 1 | `race_to_n` / Nim | tiny deterministic kernel | scaffolding |
| 2 | `three_marks` | flat placement and simple pattern detection | first pleasant board smoke |
| 3 | `column_four` | gravity alignment, previews, public polish | first showcase target |
| 4 | `directional_flip` | directional scanning and grouped effects | richer animation proof |
| 5 | `draughts_lite` | movement, capture, forced continuation | action-tree proof |
| 6 | `high_card_duel` / `blackjack_lite` | chance, private views, filtered logs | hidden-info proof |
| 7 | `token_bazaar` / `resource_race` | resources, payments, score economy | original portfolio microgame |
| 8 | `secret_draft` / commitment microgame | simultaneous hidden choice, reveal, drafting | waiting/private-bot proof |
| 9 | `poker_lite` | betting, pots, public/private cards | imperfect-info policy proof |
| 10 | `plain_tricks` | follow-suit, tricks, round scoring | classic card-game depth |
| 11 | `masked_claims` | bluffing, claims, challenges, reactions | reaction-window proof |
| 12 | `flood_watch` | cooperative event pressure | shared outcome and automation |
| 13 | `frontier_control` | asymmetric area control | faction/action asymmetry |
| 14 | `event_frontier` | event-driven asymmetric scenario | highest public complexity |
| Appendix | private red-team | private stress test only | not architecture-driving |

## Stage 0: repository skeleton

Purpose: create the repository law, workspace, CI, and empty contracts without pretending Rulepath already has a real game.

Mechanics proved: none; only build shape, ADR placement, static web shell, placeholder WASM loading, and noun-free `engine-core` discipline.

Public role: no public gameplay.

Required docs/tests/traces/benchmarks: foundation docs, ADR folder, format/test/build smoke, no game traces.

Mechanic atlas pressure: initialize atlas process; no primitives.

Exit gate: docs copied, CI smoke passes, web shell loads placeholder WASM, `engine-core` contains only generic contracts.

Not allowed: real mechanics in `engine-core`, YAML behavior, hosted services, private-game names.

## Stage 1: tiny game — `race_to_n` or Nim

Purpose: prove setup, turn order, legal actions, validation, command application, terminal detection, effects, replay, bot simulation, and WASM path while the game is too small to hide architecture mistakes.

Mechanics proved: tiny turn-taking, simple numeric state, flat actions, terminal outcome, stale/invalid diagnostics.

Public role: scaffolding; not the first impressive milestone.

Required docs/tests/traces/benchmarks: full per-game docs, normal-win trace, invalid/stale diagnostic trace, random-bot simulations, replay hash tests, legal-action/apply benchmarks.

Mechanic atlas pressure: local-only; no counter/resource primitive yet.

Exit gate: human vs random bot works in CLI and web, 100,000 native random games complete without crash, replay reproduces hashes, static data contains no behavior.

Not allowed: generalized piles, decks, boards, tracks, resources, multiplayer, polished renderer detour.

## Stage 2: flat placement — `three_marks`

Purpose: prove fixed spatial positions, occupancy, direct legal target highlighting, simple pattern detection, draw states, and a clean SVG board.

Mechanics proved: placement, occupancy, row/column/diagonal detection, draw, direct legal controls, keyboard-accessible actions where practical.

Public role: first pleasant board UI smoke; still not the showcase.

Required docs/tests/traces/benchmarks: rule tests for occupied cells and wins, property tests for pattern detection, draw traces, Level 1 bot tests, UI smoke.

Mechanic atlas pressure: record fixed coordinate occupancy and simple pattern detection as local-only; do not extract yet.

Exit gate: occupied cells are never legal, win/draw detection covered, random and rule-informed bots exist, UI is pleasant and neutral.

Not allowed: grid primitive in `engine-core`, speculative `game-stdlib` extraction from one game.

## Stage 3: gravity alignment — `column_four`

Purpose: make the first “Rulepath is real” public milestone.

Mechanics proved: gravity-constrained placement, legal columns, hover/drop previews, line detection under gravity, effect-log-driven drop animation, win-line effects, baseline policy bot.

Public role: first showcase-quality game and portfolio proof.

Required docs/tests/traces/benchmarks: line detection tests, golden traces for horizontal/vertical/diagonal wins and draw, bot legality/determinism/explanation tests, playout throughput, bot latency, UI smoke, replay viewer smoke.

Mechanic atlas pressure: compare coordinate occupancy and line detection from `three_marks` and `column_four`; likely repeated-shape candidate but extraction should normally wait for Stage 4.

Exit gate: public page feels polished, legal columns only are clickable, previews are safe, animations come from semantic effects, Level 1 and preferably Level 2 bot explains choices.

Not allowed: debug-first public screen, TypeScript legality, early Canvas/PixiJS without evidence, `engine-core` grid nouns.

## Stage 4: directional multi-piece effects — `directional_flip`

Purpose: prove directional scanning, bracketed changes, grouped effects, multi-piece animation, greedy bot policy, and real grid/pattern extraction pressure.

Mechanics proved: directional scans, bracketed flips/captures, pass/no-move if included, grouped effect batches, richer replay animation.

Public role: richer abstract board game after the showcase.

Required docs/tests/traces/benchmarks: legal move generation tests, directional scan tests, grouped effect traces, replay animation smoke, greedy bot tests, playout benchmarks.

Mechanic atlas pressure: after Stages 2–4, fixed coordinate and line/directional scanning helpers may be promoted to `game-stdlib` if the ledger supports it. A third-use decision is mandatory.

Exit gate: all flips are represented in effects or visible child details, replay reconstructs consequences, extraction decision documented.

Not allowed: untyped directional selectors in data, grid concepts in `engine-core`.

## Stage 5: movement and forced continuation — `draughts_lite`

Purpose: introduce origin/destination movement, capture, forced moves, multi-step continuations, promotion if scoped, and progressive action construction.

Mechanics proved: movement paths, capture, mandatory capture, forced continuation, multi-step action trees, stale diagnostics, optional shallow search under budgets.

Public role: first serious compound-action UI proof.

Required docs/tests/traces/benchmarks: forced-capture tests, continuation traces, action-tree hash tests, progressive UI smoke, bot legality tests, legal tree and bot benchmarks.

Mechanic atlas pressure: movement/capture/placement inventory; no generic movement framework unless repeated pressure or ADR.

Exit gate: action trees work in CLI/web, forced continuations replay correctly, UI guides construction clearly, baseline bot follows forced rules.

Not allowed: full chess exception load, generic movement in `engine-core`, search without benchmarks.

## Stage 6: chance and hidden information — `high_card_duel` / `blackjack_lite`

Purpose: prove deterministic shuffle, private views, viewer-filtered logs/effects, no-leak serialization, and bots acting from allowed private views.

Mechanics proved: deck/list order, deterministic random sample, draw/reveal, public/private views, hidden identity redaction, private hand UI.

Public role: first hidden-information safety proof.

Required docs/tests/traces/benchmarks: shuffle replay traces, no-leak tests for views/logs/previews/serialization/bots/UI payloads, bot-view tests, hand/hide UI smoke, playout benchmarks.

Mechanic atlas pressure: deck/zone helpers remain local until a second card game proves repeated shape.

Exit gate: unauthorized viewers cannot receive hidden identities anywhere; seed replay reproduces draws; bots simulate many games legally.

Not allowed: sending hidden state to browser, omniscient bots, proprietary card text.

## Stage 7: resources and economy — `token_bazaar` / `resource_race`

Purpose: introduce explicit resource effects, payments, gains, scoring economy, cleanup phases, and valuation bots.

Mechanics proved: counters, resources, costs, gains, score tracks, purchase/take/pass choices, conservation invariants.

Public role: original microgame portfolio piece.

Required docs/tests/traces/benchmarks: accounting tests, invariant simulations, cost preview traces, valuation bot tests, legal action/apply benchmarks.

Mechanic atlas pressure: resource/accounting repeated-shape candidate only after a second economy/betting game.

Exit gate: all accounting changes are explicit effects, costs are previewed by Rust, bots explain visible value choices.

Not allowed: static data formulas for payments, casino-style presentation.

## Stage 8: commitment and drafting — `secret_draft`

Purpose: prove simultaneous hidden choice, waiting states, reveal phases, hand passing/drafting, ordered resolution, and private bot views.

Mechanics proved: commitments, reveal, hidden pending choices, simultaneous resolution, hand/list passing.

Public role: waiting-state and private-view UX proof.

Required docs/tests/traces/benchmarks: commitment no-leak tests, reveal traces, waiting UI smoke, bot-view tests, replay redaction tests.

Mechanic atlas pressure: commitment/reveal becomes candidate; extract only after repeated pressure.

Exit gate: commitments remain hidden until reveal, UI shows who is pending without leaking choices, bots act only from allowed views.

Not allowed: hidden choices in DOM/local storage, actual hidden-state sampling by bots.

## Stage 9: betting and showdown — `poker_lite`

Purpose: add betting after chance, hidden views, resources, and action trees are proven.

Mechanics proved: fold/check/call/bet/raise, pots, public/private cards, simple showdown, hand evaluator, imperfect-information authored bot.

Public role: imperfect-information bot and accounting proof.

Required docs/tests/traces/benchmarks: written variant scope before coding, pot edge-case tests, hand evaluator tests, no-leak bot candidate ranking tests, betting flow traces, latency benchmarks.

Mechanic atlas pressure: resource/accounting and card/zone pressure increases; no public MCTS/ISMCTS.

Exit gate: betting flow is correct for scoped variant, bots never inspect opponent hidden cards, dev ranking is viewer-safe.

Not allowed: real-money/casino features, unbounded variants, hidden-state cheating, ML/RL.

## Stage 10: trick-taking — `plain_tricks`

Purpose: prove lead/follow constraints, trick resolution, deal rotation, trump/no-trump variants if scoped, partnerships if selected, and round scoring.

Mechanics proved: follow-suit obligations, trick winner, round scoring, deal rotation, variant pressure.

Public role: classic card-game depth.

Required docs/tests/traces/benchmarks: follow-suit tests across many hands, trick-resolution traces, hidden-info no-leak tests, baseline bot tests, round scoring benchmarks.

Mechanic atlas pressure: card zones, follow constraints, and trick scoring become repeated candidates.

Exit gate: illegal plays are not clickable, rule coverage maps every variant rule, hidden information remains safe.

Not allowed: copied rule prose, full bridge-scale complexity.

## Stage 11: bluffing and reactions — `masked_claims`

Purpose: prove claims, challenges, pending responses, reaction windows, conditional resolution, cancellation/replacement if scoped, and no-leak logs.

Mechanics proved: hidden claims, challenges, response windows, pending responses, conditional effects, bot responses.

Public role: reaction-window proof.

Required docs/tests/traces/benchmarks: pending-response traces, no-leak claim/challenge tests, bot response legality, reaction UI smoke, effect log readability tests.

Mechanic atlas pressure: reaction/window helper requires repeated pressure or ADR; third-use hard gate applies.

Exit gate: logs explain who may respond and why, hidden claims do not leak, bots respond legally.

Not allowed: trademark-forward hidden-role names, proprietary role/card text, generic reaction window in `engine-core`.

## Stage 12: cooperative pressure — `flood_watch`

Purpose: prove shared win/loss, event pressure, role powers, enemy/environment automation, multi-action turn budgets, and scenario setup.

Mechanics proved: cooperative outcome, event deck, role powers, automation, multi-action budget, scenario setup.

Public role: original cooperative portfolio piece.

Required docs/tests/traces/benchmarks: automation replay traces, shared outcome tests, role-power coverage, cooperative bot simulations, event/action tree benchmarks.

Mechanic atlas pressure: event automation and role powers remain game-local unless repeated.

Exit gate: automation is deterministic and effect-log-driven, shared win/loss tested, UI explains event pressure.

Not allowed: mimicking proprietary cooperative games, role powers in `engine-core`.

## Stage 13: asymmetric area control — `frontier_control`

Purpose: prove graph maps, area control, faction-specific actions/scoring, per-faction UI, and per-faction bots.

Mechanics proved: graph topology, control, asymmetry, faction-specific legal actions, faction-specific scoring.

Public role: asymmetric strategy showcase.

Required docs/tests/traces/benchmarks: faction rule coverage, per-faction bot tests, graph/control traces, simulation metrics, UI affordance smoke.

Mechanic atlas pressure: graph/control helpers may become candidates; faction nouns stay local.

Exit gate: asymmetry is readable, each faction has random and baseline bot, simulations produce useful metrics.

Not allowed: faction nouns in `engine-core`, private-game-driven maps or names.

## Stage 14: event-driven asymmetric scenario — `event_frontier`

Purpose: prove highest public complexity before any private red-team work.

Mechanics proved: event decks with exceptions, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees, scripted policy bots, scenario setup.

Public role: public proof that Rulepath can handle serious complexity without claiming arbitrary tabletop support.

Required docs/tests/traces/benchmarks: robust rule coverage, long-game traces, scripted bot docs, large action-tree benchmarks, replay/debug tools for long games.

Mechanic atlas pressure: every repeated mechanic must have ledger resolution. Any new language pressure is recorded, not acted on without ADR.

Exit gate: public portfolio demo stands without private experiments, action trees remain usable, effect logs remain readable, kernel remains clean.

Not allowed: private licensed content, DSL by stealth, architecture claims beyond proven games.

## Appendix: private monster-game red-team

Private monster-game experiments may happen only after the public ladder stands on its own. They are private, optional, isolated, and late.

Purpose: stress-test architecture without driving public architecture.

Mechanics proved: only whatever the private experiment explicitly records; not public claims.

Required docs/tests/traces/benchmarks: private rule coverage, private source/IP review, explicit kernel-contamination review, performance notes.

Mechanic atlas pressure: public atlas may record generic pressure without private names or proprietary details.

Exit gate: public Rulepath can stop the experiment without damage.

Not allowed: public build leakage, public CI dependency, public docs naming licensed games, kernel nouns, full private vertical slice before public ladder.

---

# Part B — Build gate order

This is gate order, not a ticket plan. Rulepath allows complexity through gates. At every gate, docs, tests, traces, simulations, benchmarks, mechanic atlas updates, and boundary review are required (see the per-stage gate above and the [universal acceptance invariants](INVARIANTS.md)).

## Gate 0: skeleton

Admit: no implementation exists.

Build shape: Rust workspace, empty or placeholder `engine-core`, empty or placeholder `game-stdlib`, `ai-core` trait shell, `wasm-api` shell, React/TypeScript web shell, tools placeholders, docs, templates, ADR folder, CI smoke.

Required ADRs before serious build: Rust core + WASM shell; engine/data/game boundary; typed Rust behavior/no DSL/no YAML; action tree/effect-log UI; bot fairness; static local-first app; public ladder/IP isolation.

Exit: `cargo test` or equivalent workspace smoke runs, web shell builds, placeholder WASM loads, docs are present, `engine-core` is noun-free.

## Gate 1: tiny game

Admit: skeleton passes.

Build: `race_to_n` or Nim with setup, state, legal actions, validation, commands, effects, terminal outcome, deterministic seed handling, replay, random legal bot, CLI simulation, golden traces, benchmark, minimal web display.

Exit: human vs random bot works; 100,000 native random games complete; replay reproduces state/effect/action-tree hashes; browser displays legal actions and effect log; per-game docs and mechanic inventory exist.

## Gate 2: trace, replay, and benchmark hardening

Admit: tiny game exists and exposes drift risks.

Build: trace serialization, replay checker, stable hashes, benchmark harness, failure seed/command output, seed-reduction plan, fixture validation.

Exit: failing simulations can be replayed from seed and command log; golden traces fail loudly on rule drift; benchmark report records versions, command, hardware, and thresholds.

## Gate 3: WASM/static web shell

Admit: native replay and benchmarks are solid enough.

Build: batched WASM API, game picker, match setup, public view store, action tree store, effect queue, replay controls, dev toggle, safe local replay import/export.

Exit: static site plays the tiny game with no backend; human vs bot, hotseat when applicable, bot vs bot replay, and replay viewer work; dev toggle shows safe diagnostics; no rule legality exists in TypeScript.

## Gate 4: `three_marks`

Admit: web shell can play tiny game.

Build: neutral Tic-Tac-Toe-like game with local spatial model, occupancy, simple win/draw detection, SVG board, direct legal target highlighting, random legal bot, Level 1 baseline bot, docs/tests/traces/benchmarks.

Exit: occupied positions are never legal; win/draw detection is covered; bot wins/blocks immediate threats; UI is pleasant and accessible where practical; no spatial primitive enters `engine-core`.

## Gate 5: `column_four` public polish

Admit: `three_marks` is correct and pleasant.

Build: gravity-constrained alignment game with legal column interaction, hover/drop preview, semantic drop/win effects, polished responsive SVG renderer, Level 1 and preferably Level 2 bot, replay viewer polish, UI smoke tests.

Exit: public page looks like a polished playable game; legal columns only are clickable; animations are effect-log-driven; bot explanations are available; benchmarks and UI smoke tests pass; mechanic atlas records repeated coordinate/line pressure.

## Gate 6: `directional_flip` and earned extraction decision

Admit: `column_four` is showcase-quality.

Build: directional flipping game with directional scanning tests, grouped multi-piece effects, replay animation, greedy bot, trace/replay/benchmark coverage.

Decision: compare `three_marks`, `column_four`, and `directional_flip`. Reuse, promote, or defer coordinate/line/directional helpers through the primitive-pressure ledger.

Exit: flips replay and animate correctly; any extracted helper is narrow, typed, tested, documented, back-ported, benchmarked, and not in `engine-core`.

## Gate 7: `draughts_lite` action trees

Admit: grid/directional pressure resolved.

Build: movement/capture/mandatory-continuation game with action trees, progressive UI, forced rules, stale diagnostics, action tree inspector, random and rule-informed bot. Shallow deterministic search only if benchmarks allow.

Exit: forced multi-step actions replay correctly; UI guides path construction and confirmation; action tree inspector is usable; bot legality and latency tests pass.

## Gate 8: cards, chance, hidden information

Admit: compound actions and replay are stable.

Build: `high_card_duel`; add `blackjack_lite` only if it adds useful pressure. Implement deterministic shuffle, private/public views, draw/reveal effects, no-leak tests, serialization tests, bot-view tests, private hand UI.

Exit: hidden identities never leak through views, logs, previews, serialization, bot views, UI payloads, DOM fixtures, local storage, or replay exports; seed replay reproduces random samples.

## Gate 9: resources, simultaneous choice, drafting

Admit: hidden information is safe.

Build: original resource economy and commitment/drafting microgames. Add explicit resource effects, waiting states, reveal phases, hand/list passing, valuation bots, invariant tests.

Exit: resource accounting is effect-visible; simultaneous choices remain hidden until reveal; bots use allowed views; invariant/no-leak tests and benchmarks pass; atlas updated.

## Gate 10: betting and tricks

Admit: resources, hidden info, action trees, and card zones are proven.

Build: `poker_lite` and one scoped trick-taking game when ready. Variant scope must be written before coding. Implement pot/accounting tests, hand evaluator tests, follow-suit tests, hidden-info tests, bot legality/latency benchmarks.

Exit: betting/trick rules are correct for chosen variants; bots finish games without hidden-state cheating; UI remains understandable; native benchmarks exist.

## Gate 11: bluffing and reactions

Admit: card games and hidden-info bots are stable.

Build: original bluffing/reaction microgames with claims, challenges, pending responses, conditional resolution, cancellation/replacement if scoped, no-leak tests, baseline policy bot.

Exit: logs explain who may respond and why; bots respond legally; hidden claims do not leak; reaction-window abstraction has repeated pressure or ADR before promotion.

## Gate 12: cooperative pressure

Admit: reactions and event-like resolution are understood enough.

Build: original cooperative event-pressure game with shared outcome, event deck, role powers, environment automation, multi-action budgets, scenario setup, cooperative bot baseline, replayable automation effects.

Exit: role powers live in game module; automation is deterministic and effect-log-driven; shared win/loss tested; public UI explains pressure clearly.

## Gate 13: asymmetric area control

Admit: cooperative event pressure and simulation tooling are stable.

Build: original graph-map/area-control microgame with faction-specific actions/scoring/UI affordances, per-faction bots, simulation metrics.

Exit: no faction nouns enter `engine-core`; each faction has random and baseline bot; effect logs stay readable; simulations produce useful metrics.

## Gate 14: event-driven asymmetric scenario

Admit: area-control asymmetry works publicly.

Build: original public scenario/event-driven asymmetric game with event decks, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees, scripted policy bots, robust rule coverage, long-game replay/debug tools.

Exit: action tree UI remains usable; scripted bots are demo-coherent; debug/replay tools diagnose long games; public Rulepath stands without private experiments; every repeated mechanic has ledger resolution.

## Gate P: private monster-game red-team

Admit: Gate 14 is complete and public Rulepath is coherent without private work.

Build: optional private vertical slice in private repo/submodule/local-only folder. No public build, public CI dependency, public docs naming, public assets, or full bot at first. Strict kernel-contamination review.

Exit: no kernel contamination; missing abstractions are documented without private names; performance is measurable; public Rulepath can abandon the private experiment without damage.
