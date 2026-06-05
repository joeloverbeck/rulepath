# FOUNDATIONS

Repository constitution. Feed into every spec, ticket, and agent prompt.
Goal: a public playable card/board-game website with excellent UI, clear animation, original visuals, deterministic rules, competent bots, replay/debugging, and portfolio architecture. Complexity MUST be earned through a mechanic ladder. Do not claim near-term support for arbitrary tabletop games.

## Law

- Rust MUST own rules, legal move generation, transitions, replay, simulation, serialization contracts, and AI hot loops. TypeScript MUST be only browser shell, UI integration, and presentation.
- The UI MUST NOT decide legality. It asks Rust what is visible, legal, and changed.
- `engine-core` MUST stay tiny: identities, transitions, action trees/paths, effects, seeds, replay, visibility/public-view contracts, serialization, diagnostics, and versioning.
- `engine-core` MUST NOT contain game nouns, game rules, UI concepts, bot strategy, licensed data, card names, factions, or scenario concepts.
- Game nouns and game rules belong in `games/*`. Reusable mechanics belong in `game-stdlib`, not automatically in `engine-core`.
- Generic primitives MUST NOT be speculative. Reusable primitives SHOULD require two implemented games or an ADR.
- Rule behavior MUST be typed Rust in v1. Static content MAY be data-driven; behavior MUST NOT hide in untyped data.
- YAML MUST NOT be default; it REQUIRES ADR and a strong reason. Static data MUST NOT contain selectors, loops, rule branches, tactical AI, or exception logic.
- No custom DSL at the start. A future DSL MUST be typed, validated, source-span-aware, tested, formatted, versioned, benchmarked, and compiled/lowered.
- Legal actions MUST support action trees/progressive construction when moves are compound.
- The UI MUST consume semantic effect logs and MUST NOT infer causality from state diffs.
- Every committed action MUST be replayable from seed and command stream.
- Hidden information MUST NOT leak through state, logs, previews, serialization, UI metadata, or bots.
- Bots MUST consume the same legal action API as humans and MUST choose only legal actions.
- Every game MUST first have a random legal bot. Next SHOULD come rule-informed baselines and authored heuristic/policy bots. Shallow search MAY be added where appropriate. MCTS/ISMCTS only after speed/stability is proven. ML/RL MUST NOT be used in v1/v2 unless ADR-approved.
- Bot policy SHOULD avoid weight soup. Prefer ordered priorities, decision trees, lexicographic priorities, phase-aware strategies, explainability, and game-specific policy modules.
- The first public app SHOULD be static: no accounts, no database, no hosted multiplayer. Initial modes SHOULD be human vs bot, local hotseat, bot vs bot replay, replay viewer, and game picker.
- Public UI SHOULD expose clear legal actions, effect logs, replay controls, seed display, action/effect inspectors, and performance timings.
- Future hosted multiplayer MUST use an authoritative native Rust server. Browser clients MUST NOT own authoritative state. Bots do not require multiplayer infrastructure.
- Public games MUST be public-domain/classic, original, or permissioned. Rules docs MUST be original summaries with source notes. Public builds MUST use original assets and neutral presentation. Trademark-risk games SHOULD use neutral names.
- Licensed games/data/modules, proprietary card text, and proprietary assets MUST NOT enter public repos/builds. Private licensed experiments MAY exist only in private repos/submodules/local folders and MUST NOT be public-CI dependencies or shipped to unauthorized browsers.
- Every game MUST have unit/rule tests, golden traces, simulation/fuzz coverage, deterministic replay, serialization coverage, rule coverage notes, UI metadata, AI legal-action tests, CLI simulation, and benchmarks. Hidden-info games MUST add no-leak tests. Public-web games MUST add UI smoke tests.
- Every bug MUST get regression coverage. When tests fail: validate tests, identify SUT vs test-suite fault, fix it, add/update regression coverage, and report changes. Never blindly rewrite tests to pass.
- Major architecture changes REQUIRE ADRs.
- AI coding agents are accelerators, not unattended architects. Agent tasks MUST be bounded, measurable, test-driven, explicit about non-goals and forbidden kernel changes, and should request complete files or coherent complete sections, not diffs.
