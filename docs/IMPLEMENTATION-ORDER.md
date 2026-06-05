# Rulepath Implementation Order

Status: gate order. This is not a ticket plan.

Rulepath allows complexity through gates. At every gate, docs, tests, traces, simulations, benchmarks, mechanic atlas updates, and boundary review are required.

V1/v2 excludes hosted multiplayer, accounts, databases, matchmaking, DSL work, YAML, public MCTS/ISMCTS/Monte Carlo bots, ML/RL, and private monster-game work.

## Cross-gate requirements

Every gate must verify:

- Rust owns behavior;
- TypeScript does not decide legality;
- static data remains typed content/parameters only;
- `engine-core` has no game nouns;
- `game-stdlib` changes have mechanic atlas/ledger support;
- a third repeated mechanic shape is resolved before proceeding;
- rule docs, source notes, mechanic inventory, rule coverage, traces, simulations, benchmarks, and boundary review are updated;
- replay remains deterministic;
- bots use legal action APIs and allowed views;
- public IP boundaries are preserved.

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

## Stop conditions

Stop when `engine-core` gains game nouns, static data becomes procedural, TypeScript starts deciding legality, bots cheat, replay stops being deterministic, performance is unknown, public UI is debug-first, public builds contain licensed data, or private work starts driving public architecture.
