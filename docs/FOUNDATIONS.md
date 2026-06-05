# Rulepath Foundations

Status: repository constitution. Supersede only by accepted ADR.

Rulepath is a public playable, portfolio-quality web app for card and board games. It is also a disciplined path toward eventually stress-testing private complex tabletop implementations and, later, a long-term engine research project. When those goals compete, the public playable product wins.

Rulepath must make a visitor think: “This is a polished playable game site, and the architecture behind it is serious.” Rulepath must not claim near-term arbitrary tabletop support. Complexity is earned through the public game ladder.

## 1. Priority order

1. Polished public playable site.
2. Correct deterministic rules.
3. Clean kernel boundary.
4. Future multiplayer readiness through commands, views, replay, and serialization.
5. Long-term engine research.

The public app wins tradeoffs. Do not sacrifice visible play quality to speculative generality, private stress tests, early hosted services, or research-AI ambition.

## 2. Rust owns behavior

Rust must own setup, legal action generation, validation, state transitions, scoring, terminal detection, deterministic randomness, semantic effects, replay/hash behavior, public/private view projection, serialization contracts, and bot choices.

TypeScript owns presentation: browser shell, routing if used, layout, menus, renderer integration, settings, panels, accessibility wrappers, local safe replay import/export, and development inspectors.

TypeScript must not decide legality. Static files must not define behavior.

## 3. Engine-core is a contract kernel

`engine-core` is generic infrastructure only. It may define identities, versions, seeds, viewer/actor contracts, action trees, action paths, commands, diagnostics, semantic effect contracts, replay/checkpoint/hash contracts, visibility contracts, serialization boundaries, and generic errors.

`engine-core` must remain small and noun-free. It must not contain board, grid, card, deck, pile, hand, suit, faction, scenario, trick, pot, resource, role, combat, territory, movement, adjacency, line, capture, flip, promotion, or similar game nouns.

Game-specific behavior belongs in `games/*`. Game-specific types inside a game module are correct. Game-specific types inside `engine-core` are a boundary failure.

## 4. Game-stdlib is earned

`game-stdlib` is not a speculative second kernel. Reusable mechanics enter it only after implemented games exert pressure and the mechanic atlas records the decision.

A third official game with the same mechanic shape must not reimplement that shape without the primitive-pressure ledger saying one of: reuse, promote, or explicitly defer with rationale.

## 5. Static data is typed content, not behavior

Rulepath is data-driven for game content, parameters, presentation metadata, fixtures, traces, and typed variant selection. Rulepath is not data-driven for rule behavior in v1/v2.

Do not describe v1/v2 as “data-driven rules” except when warning against that idea.

Static files may contain manifests, labels, icon IDs, theme tokens, public/original component IDs, setup constants, scoring tables, typed variants, localization strings, explanation templates, UI metadata, golden traces, benchmark fixtures, and source notes. They must deserialize into strict typed structures or strict browser-facing schemas.

Static files must not contain selectors, rule branches, loops, triggers, conditional effects, tactical AI conditions, exception logic, mandatory-action rules, hidden defaults, arbitrary expressions, or nested untyped objects interpreted as behavior.

No YAML by default. No DSL at project start.

A future DSL requires ADR, repeated Rust pain, typed/lowered semantics, source spans, formatting, linting, versioning, tests, benchmarks, examples, anti-examples, and explicit replay/hash implications.

## 6. Legal-only UI

Normal public UI must allow only legal moves. Rust/WASM supplies legal action trees, safe previews, diagnostics, semantic effects, and viewer-safe public views.

Animation is effect-log-driven. The UI must not infer authoritative causality from state diffs except as diagnostics. After animation, the renderer must settle to the latest viewer-safe public view.

Hidden information must not leak through browser payloads, DOM attributes, debug panels, local storage, serialized public views, previews, logs, bot explanations, or replay exports.

V1 uses React + SVG by default. Canvas or PixiJS may be introduced only after profiling evidence or ADR.

Public visual direction is a cozy board-game table: warm, tactile, inviting, original, polished. Avoid proprietary mimicry, casino vibes, SaaS dashboard coldness, debug-console dominance, and skeuomorphic clutter.

## 7. Fair explainable bots

Every official game must have a Level 0 random legal bot. Serious public games should have Level 1 rule-informed bots. Polished public games should have Level 2 authored policy bots.

Bots must use the same legal action API as humans, choose through normal validation, mutate no state directly, and never use hidden information unavailable to their seat. Public bots must be deterministic under seed, rules version, view, and decision limits.

Level 2 bots require a strategy evidence pack before coding. Avoid weight soup. Prefer candidate extraction, ordered tactical priorities, phase-aware policy nodes, lexicographic ranking, small bounded tie-breakers, deterministic seeded tie-break, and human-readable explanations.

MCTS, ISMCTS, Monte Carlo-style bots, ML, and RL are not public v1/v2 features. Any future use requires ADR.

## 8. Full support for every official game

Smoke games may be visually modest, but they are not internally disposable. Every officially encoded game receives typed rules, docs, source notes, mechanics inventory, rule coverage, unit/rule/golden/property/simulation/replay/serialization tests, CLI simulation, benchmark coverage, UI metadata, replay support, a random legal bot, and UI smoke tests once web-exposed.

A game is not implemented merely because it appears to work in the browser.

## 9. Local-first v1/v2

Initial Rulepath is a static local-first app: human vs bot, local hotseat, bot vs bot replay, replay viewer, and local replay import/export.

No accounts, database, hosted multiplayer, matchmaking, chat, ranked play, or server persistence belong in v1/v2.

Future hosted multiplayer requires an authoritative Rust server. Browser clients must not own authoritative state.

## 10. IP conservatism

Public games must be public-domain/classic, original, or permissioned. Common names such as Tic-Tac-Toe or Checkers may be used when safe; neutral IDs and names are required where commercial trademark or trade-dress risk exists.

Public files must use original rules prose and original or properly licensed assets. Do not copy rulebook prose, proprietary card text, board art, icons, screenshots, fonts without verified redistribution rights, trade dress, licensed data, or private modules.

Private licensed stress tests are late, isolated, optional, and never architecture-driving. If code or data ships to an unauthorized browser, it has shipped.

## 11. Agent discipline

Claude Code, Codex, and similar agents are accelerators, not unattended architects. Agent tasks must be bounded, test-driven, explicit about non-goals, and forbidden from kernel contamination, behavior-in-data, TypeScript legality, cheating bots, IP leaks, and blind test rewriting.

When tests fail, the required protocol is:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report changes.

Output complete files or coherent complete sections, not diffs.

## 12. Stop conditions

Stop and reassess when any of these happens:

- `engine-core` gains game nouns;
- static files start acting procedural;
- the phrase “data-driven rules” appears as an approval rather than a warning;
- TypeScript decides legality;
- animations depend on guessed state diffs;
- bots bypass legal action APIs or use unauthorized hidden state;
- official games lack docs, traces, simulations, benchmarks, or rule coverage;
- a third repeated mechanic proceeds without atlas/ledger decision;
- public UI becomes debug-first;
- YAML or DSL work appears without ADR;
- public builds contain private licensed content;
- private monster-game work starts shaping public architecture.
