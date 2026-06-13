# Rulepath Foundations

Status: repository constitution. Supersede only by accepted ADR.

Rulepath is a Rust-first, public playable, portfolio-quality web app for card and board games. It must make a visitor think: **this is a polished playable game site, and the architecture behind it is serious**.

Rulepath is also a path toward later private stress tests and long-term engine research. Those are subordinate goals. When public product quality, deterministic correctness, clean boundaries, private monster-game pressure, hosted multiplayer, and research ambition compete, the polished public playable product wins.

## 1. Priority order

1. Polished public playable site.
2. Correct deterministic rules.
3. Clean engine/game/data boundary.
4. Future multiplayer readiness through command logs, public/private views, replay, serialization, and deterministic Rust rules.
5. Later private stress tests.
6. Long-term engine research.

Rulepath MUST NOT sacrifice visible product quality to speculative engine generality, early hosted services, private licensed game pressure, or research-AI ambition.

Rulepath MUST NOT pretend to be an arbitrary tabletop engine. Complexity is earned through the public staged ladder in [ROADMAP.md](ROADMAP.md).

## 2. Behavior authority

Rust MUST own:

- setup;
- legal action generation;
- validation;
- state transitions;
- scoring and terminal detection;
- deterministic randomness;
- semantic effects;
- public/private view projection;
- replay and hash behavior;
- serialization contracts;
- bot decisions.

TypeScript/React MUST own presentation only:

- app shell;
- game picker and setup UI;
- layout;
- renderer integration;
- settings and panels;
- accessibility wrappers;
- local safe replay import/export UI;
- development inspectors.

TypeScript MUST NOT decide legality. Static files MUST NOT define rule behavior. Browser payloads MUST already be safe for the receiving viewer.

These authority rules apply to every supported seat count. A game may declare
one seat, two seats, or many seats, but Rust still owns setup validation, turn
order, active or pending actors, public/private projections, terminal
breakdowns, and replay evidence for that declared range. The N-seat details are
codified in [MULTI-SEAT-AND-SURFACE-CONTRACT.md](MULTI-SEAT-AND-SURFACE-CONTRACT.md).

## 3. `engine-core` is a contract kernel

`engine-core` exists for generic infrastructure contracts. It MAY know these nouns:

`game id`, `match id`, `seat id`, `player id`, `actor`, `viewer`, `rules version`, `manifest/data version`, `schema version`, `seed`, deterministic RNG contract, `action tree`, `action path`, `command envelope`, `diagnostic`, `effect envelope`, `public/private view contract`, `visibility scope`, `replay`, `checkpoint`, `hash`, and serialization boundary.

`engine-core` MUST NOT know mechanic/domain nouns such as:

`board`, `grid`, `card`, `deck`, `pile`, `hand`, `suit`, `faction`, `scenario`, `trick`, `pot`, `resource`, `role`, `combat`, `territory`, `movement`, `adjacency`, `line`, `capture`, `flip`, `promotion`, `auction`, `betting`, `drafting`, or similar game-mechanic vocabulary.

Decision rule: `engine-core` may know that an opaque game-defined payload exists; it MUST NOT know what that payload means mechanically. Typed mechanic nouns belong in `games/*` first and only in `game-stdlib` after earned pressure.

A game-specific type inside a game module is correct. A game-specific type inside `engine-core` is a boundary failure.

N-seat pressure does not relax this boundary. Turn-order policies, tables,
teams, pots, graphs, decks, walls, factions, partnerships, hand evaluators, and
seat-role semantics are game-local first, and may enter `game-stdlib` only after
the mechanic atlas process earns a narrow helper. They do not belong in
`engine-core`.

## 4. `game-stdlib` is earned, not speculative

`game-stdlib` is not a second kernel and not a universal tabletop library.

Reusable helpers MAY enter `game-stdlib` only after implemented official games prove a repeated mechanic shape and the primitive-pressure process in [MECHANIC-ATLAS.md](MECHANIC-ATLAS.md) records the decision.

The normal rule:

- first use: implement locally;
- second similar use: implement locally, compare, and update mechanic inventories/atlas notes;
- third official use: hard gate. The game MUST NOT proceed until the primitive-pressure ledger decides reuse, narrow promotion, explicit deferral/rejection with rationale, or ADR escalation.

Promotion is not only extraction. When a helper is promoted to `game-stdlib`, every earlier official game identified by the atlas as using the promoted mechanic shape MUST either migrate to that helper or carry an explicit accepted exception. Same-gate deferral is allowed only when it is recorded as promotion debt with named games, named primitive, evidence, risk, and the closure gate. Unless an accepted exception says otherwise, the next implementation spec before further mechanic-ladder advancement MUST close open promotion debt.

No helper may enter `engine-core` merely because multiple games use it.

## 5. Static data is typed content, not behavior

Rulepath is data-driven for typed content, parameters, presentation metadata, fixtures, traces, and variant selection. Rulepath is not data-driven for rule behavior in v1/v2.

Allowed static data includes manifests, display metadata, typed variants, original component IDs, setup constants, scoring tables, fixtures, golden traces, benchmark fixtures, source notes, UI metadata, and explanation templates keyed to Rust effects/actions.

Static data MUST NOT include selectors, rule branches, loops, triggers, conditional effects, mandatory-action rules, tactical AI conditions, procedural mutation instructions, exception logic, hidden defaults, arbitrary expressions, or nested untyped objects interpreted as behavior.

No YAML by default. No DSL at project start.

A future DSL requires ADR and repeated painful evidence from typed Rust implementations. The ADR MUST address typed semantics, lowering/compilation, source spans, formatting, linting, versioning, tests, benchmarks, examples, anti-examples, determinism, replay/hash implications, visibility, migration, and agent safety.

## 6. Official games are evidence-heavy

Every official game MUST be fully supported:

- typed Rust rules;
- original Rulepath rules summary;
- source notes;
- mechanic inventory;
- rule coverage;
- unit/rule/golden/property/simulation/replay/serialization tests;
- CLI simulation;
- benchmarks;
- deterministic replay support;
- random legal bot;
- UI metadata;
- UI smoke tests once web-exposed.

A game is not done merely because it appears playable in the browser. Browser playability without rule coverage, traces, replay, visibility tests, bot legality, benchmarks, and docs is a demo shell, not an official game.

The full contract lives in [OFFICIAL-GAME-CONTRACT.md](OFFICIAL-GAME-CONTRACT.md).

## 7. Public UI is central product work

Normal public UI MUST be legal-only: illegal moves are absent, inert, or visually unavailable. Learning/debug mode MAY show Rust-supplied viewer-safe disabled reasons. TypeScript MUST NOT invent legality.

Rust/WASM supplies legal action trees, previews, diagnostics, semantic effects, bot decisions, replay, and viewer-safe views. TypeScript/React presents them.

Animation MUST be driven by semantic effects emitted by Rust. Renderer state diffs MAY be used as diagnostics, not normal authoritative causality. After animation, the renderer MUST settle to the latest viewer-safe public view.

Public visuals SHOULD feel like a cozy premium board-game table: warm, tactile, polished, original, readable, restrained, and inviting. Avoid casino vibes, SaaS-dashboard coldness, debug-console dominance, aggressive skeuomorphism, proprietary mimicry, and trade-dress imitation.

React + SVG is the v1 default. Canvas or PixiJS require profiling evidence or ADR.

## 8. Public bots are product opponents, not research AI

Every official game needs a Level 0 random legal bot. Serious public demos need Level 1 rule-informed bots. Polished public games should have Level 2 authored policy bots.

Public bots MUST be competent, explainable, fair, human-plausible, deterministic under declared inputs, and beatable. They MUST use the same legal action API as humans, choose through normal validation, mutate no state directly, and never use hidden information unavailable to their seat.

Public v1/v2 MUST NOT use MCTS, ISMCTS, Monte Carlo bots, ML, or RL. Future use requires ADR.

Bot personalities MAY vary policy order, risk posture, candidate preferences, bounded tie-breakers, allowed search limits, and explanation tone. Personality MUST NOT mean cheating, arbitrary random blunders, hidden-state access, or giant weight soup.

## 9. Local-first v1/v2

V1/v2 are static/local-first:

- human vs bot;
- hotseat;
- bot vs bot replay;
- replay viewer;
- local replay import/export.

V1/v2 MUST NOT include accounts, database, hosted multiplayer, matchmaking, chat, ranked play, or server persistence.

Rulepath MUST preserve a path to future hosted multiplayer through command logs, deterministic replay, serialization, public/private views, and an eventual authoritative Rust server running the same rule code natively. Browser clients may preview locally, but they MUST NOT own authoritative state.

## 10. IP conservatism

Public games MUST be public-domain/classic, original, or permissioned. Public files MUST use original rules prose and original, project-owned, generated-reviewed, or compatibly licensed assets.

Public Rulepath MUST NOT copy rulebook prose, proprietary card text, board art, icons, screenshots, scans, fonts without verified redistribution rights, trade dress, licensed data, or private module material.

Prefer neutral IDs/names where commercial trademark or trade-dress risk exists.

Private licensed/commercial-game stress tests are late, isolated, optional, non-public, and forbidden from shaping `engine-core`. If code or data ships to an unauthorized browser, it has shipped.

## 11. Universal acceptance invariants

Every substantial change, official game, and ADR MUST satisfy these invariants unless an accepted ADR explicitly changes them and updates this section:

These invariants apply to any positive seat count declared by a game. For
multi-seat and hidden-information games, viewer safety includes pairwise
seat-private redaction: facts private to seat A must not reach seat B, the public
observer, DOM/storage/logs, bot explanations, replay exports, traces, or tests
unless Rust has made those facts public or otherwise authorized for that viewer.

- Rust remains behavior authority.
- TypeScript does not decide legality.
- `engine-core` contains generic contracts only and remains free of mechanic nouns.
- `game-stdlib` changes are earned through the mechanic atlas.
- Promoted `game-stdlib` primitives are adopted by all matching official games, or each non-adoption has an explicit accepted exception in the atlas.
- Open promotion debt is closed before the next mechanic-ladder gate unless an accepted exception or ADR says otherwise.
- Third-use mechanic pressure is resolved before proceeding.
- Static data is typed content/parameters/metadata/fixtures/traces only.
- Unknown fields in hand-authored data are rejected by default.
- Behavior-looking fields are blocked or escalated.
- No YAML or DSL appears without ADR.
- Replay, hashes, serialization order, RNG, and traces remain deterministic or are explicitly migrated.
- Public/private views are viewer-safe.
- Hidden information does not leak through browser payloads, DOM, local storage, logs, previews, diagnostics, effect logs, bot explanations, candidate rankings, UI test IDs, or replay exports.
- Bots use the normal legal action API and allowed views only.
- Public v1/v2 exclude MCTS, ISMCTS, Monte Carlo bots, ML, and RL.
- Semantic effects drive animation; renderer diffs are diagnostics only.
- Public UI is play-first, polished, responsive, accessible, and not debug-dominated.
- V1/v2 remain static/local-first.
- Private licensed experiments remain isolated and non-architectural.
- Tests, traces, simulations, benchmarks, docs, and source notes cover the change.
- Agent output is bounded, reviewable, and delivered as complete files or coherent complete sections, not diffs.

## 12. Stop conditions

Stop and reassess before continuing when any of these happens:

- `engine-core` gains game/mechanic nouns;
- static files start acting procedural;
- anyone approves “data-driven rules” for v1/v2;
- YAML appears without ADR;
- DSL work appears without ADR;
- TypeScript decides legality;
- UI normal-mode illegal moves become clickable because “validation catches it later”;
- animation depends on guessed state diffs instead of Rust effects;
- hidden information reaches browser payloads, DOM, local storage, logs, previews, bot explanations, candidate rankings, or replay exports;
- a bot bypasses legal action APIs or uses unauthorized hidden state;
- a third repeated mechanic proceeds without ledger decision;
- a promoted primitive leaves matching prior official games un-migrated without an explicit exception or recorded promotion-debt closure gate;
- a new mechanic-ladder gate proceeds while promotion debt is still open;
- a 3+ seat game cannot prove viewer-safe public and per-seat projections;
- official games lack docs, traces, simulations, benchmarks, rule coverage, replay, or serialization tests;
- public UI becomes debug-first;
- private licensed content enters public files, public CI, public docs, public traces, public bundles, or public WASM/JS;
- private monster-game work starts shaping public architecture;
- agents are asked to “generalize,” “clean up the engine,” or “fix all tests” without bounded scope and forbidden changes.

Stopping is not failure. Continuing through a stop condition is architectural debt.

## 13. ADR triggers

ADR is required for architecture-changing decisions, including:

- changing the priority order;
- changing v1/v2 local-first scope;
- adding hosted multiplayer, accounts, server persistence, chat, matchmaking, or ranked play;
- changing `engine-core` vocabulary or responsibilities;
- promoting mechanics outside the normal primitive-pressure path;
- introducing YAML or a new hand-authored data format;
- introducing selectors, expressions, rule-like data, or DSL work;
- changing replay/hash semantics;
- changing public/private visibility contracts;
- introducing public MCTS, ISMCTS, Monte Carlo bots, ML, or RL;
- replacing React + SVG as default renderer without profiling evidence;
- allowing private licensed experiments to influence public architecture.
