# Rulepath Sources

Status: researched bibliography for the foundation set.

Last reviewed: 2026-06-28.

These sources inform Rulepath architecture, authoring discipline, bot policy, UI policy, data-format policy, replay model, and IP caution. They are precedents and warnings, not text to copy. Game-specific source notes must still document exact rules sources, chosen variants, naming rationale, and asset status.

## Game-specific source notes

| Game | Source note | Status |
|---|---|---|
| `race_to_n` | `games/race_to_n/docs/SOURCES.md` | completed for Gate 1 |
| `three_marks` | `games/three_marks/docs/SOURCES.md` | completed for Gate 4; covers original Rulepath naming, public-domain/classic-family context, variant choice, and asset posture |
| `high_card_duel` | `games/high_card_duel/docs/SOURCES.md` | completed for Gate 8; covers original neutral card-game naming, War/Blackjack exclusion, variant choice, hidden-information posture, and public asset posture |
| `briar_circuit` | `games/briar_circuit/docs/SOURCES.md` | opened for Gate 16; covers original Briar Circuit naming, classic Hearts rules-family references, selected variant choices, pass/visibility privacy, and asset/IP review posture |
| `vow_tide` | `games/vow_tide/docs/SOURCES.md` | completed for Gate 17; covers original Vow Tide naming, classic Oh Hell rules-family references, 3-7-seat variant choices, bid/hook/scoring deviations, hidden-stock privacy, helper conformance, and asset/IP review posture |
| `blackglass_pact` | `games/blackglass_pact/docs/SOURCES.md` | completed for Gate 18; covers original Blackglass Pact naming, classic partnership Spades rules-family references, fixed-four variant pinning, nil/blind-nil/bag deviations, public/private partnership signals, helper conformance, and human IP/public-release review posture |
| `meldfall_ledger` | `games/meldfall_ledger/docs/SOURCES.md` | completed for Gate 19; covers original Meldfall Ledger naming, Five Hundred Rummy / Rummy 500 rules-family references, one-deck 2-6 seat variant pinning, strict discard-pickup commitment, public meld tableau, cumulative scoring, hidden-stock/privacy posture, and human IP/public-release review posture |
| `starbridge_crossing` | `games/starbridge_crossing/docs/SOURCES.md` | completed for Gate 20 source/IP intake; covers original Starbridge Crossing naming, Star Halma / Chinese Checkers rules-family references, 121-space star topology, `{2,3,4,6}` seat support, 10-peg homes, stop-anywhere hop chains, all-public visibility, and human IP/public-release review posture |

### Blackglass Pact source-use lessons

Gate 18 records Blackglass Pact's detailed source bibliography in
`games/blackglass_pact/docs/SOURCES.md`. The repo-level lesson is that
partnership trick-taking sources disagree on mechanically significant edges, so
Rulepath pins the variant in Rust-owned game docs before implementation:
failed-nil bag attribution follows the selected Pagat/Trickster-style rule,
blind nil is a pre-deal individual commitment that skips the later ordinary bid,
and partnership identity is public while partner hands remain private. Source
notes are evidence for variant choices, not permission to copy rules prose,
table presentation, card art, or trade dress. Human IP/public-release review
remains required before external release.

### Meldfall Ledger source-use lessons

Gate 19 records Meldfall Ledger's detailed source bibliography in
`games/meldfall_ledger/docs/SOURCES.md`. The repo-level lesson is that
Rummy 500-family sources agree on the broad draw/meld/lay-off/scoring shape but
vary on implementation-relevant house rules. Rulepath pins one original public
variant before implementation: one standard 52-card deck for 2-6 seats, strict
immediate-use commitment for any discard-pile pickup including the top discard,
ace-low or ace-high runs with no around-the-corner wrap, no opening minimum,
no floating, no discard reshuffle, no jokers/wilds, no partnerships, and no
tabled-meld rearrangement. Source notes are evidence for variant choices, not
permission to copy rules prose, app layouts, card art, screenshots, icons, or
trade dress. Human IP/public-release review remains required before external
release.

## Source-use rules

Rulepath source notes MUST:

- summarize in original language;
- prefer official documentation, primary papers, and reputable references;
- distinguish evidence from Rulepath policy;
- avoid copying rulebook prose, card text, UI copy, board art, screenshots, icons, fonts, or trade dress;
- record date consulted;
- record variant decisions and deviations for each implemented game;
- mark uncertainty instead of inventing support.

## Doctrine comparables and adoption boundaries

These source IDs came from the doc/template change-plan research. They shape
Rulepath doctrine only as comparables and warnings; they are not authority over
Rulepath's foundation docs, accepted ADRs, templates, or game-specific source
notes.

| Source ID | Source | Rulepath-specific lesson | Explicit boundary |
|---|---|---|---|
| `EXT-1` | [Diataxis](https://diataxis.fr/) and [four-mode primer](https://diataxis.fr/start-here/) | Separate reference law, how-to workflow, explanation, and evidence receipts instead of making every template repeat all modes. | Non-adoption: Rulepath does not adopt Diataxis as a mandatory doc taxonomy or rename foundation docs to match it. |
| `EXT-2` | [The Practical Test Pyramid](https://martinfowler.com/articles/practical-test-pyramid.html) | Test layering and DRY/DAMP tradeoffs support extracting repetition only when it obscures correctness, while retaining use-before-reuse discipline. | Non-adoption: Rulepath does not replace its replay, simulation, no-leak, benchmark, browser-smoke, or per-game evidence taxonomy with a generic pyramid. |
| `EXT-3` | [OpenSpiel introduction](https://openspiel.readthedocs.io/en/latest/intro.html) and [core API reference](https://openspiel.readthedocs.io/en/latest/api_reference.html) | A generic game kernel can coexist with game-specific state, and observations/information states are distinct viewer contracts. | Non-adoption: Rulepath does not adopt OpenSpiel's architecture, search/RL agenda, or public bot policy. |
| `EXT-4` | [RFC 8785 JSON Canonicalization Scheme](https://www.rfc-editor.org/info/rfc8785/) | Hashing requires an invariant representation, explicit canonicalization, and versioning. | Non-adoption: Rulepath does not switch replay/hash surfaces to JSON canonicalization without an accepted migration. |
| `EXT-5` | [Rust API Guidelines checklist](https://rust-lang.github.io/api-guidelines/checklist.html) | Validating newtypes, inherent constructors, and type-significant arguments support narrow typed scaffolding APIs. | Non-adoption: Rulepath does not treat generic Rust API style as permission to move game behavior or mechanic nouns into `engine-core`. |
| `EXT-6` | [Proptest state-machine testing](https://proptest-rs.github.io/proptest/proptest/state-machine.html) | Shared test support can generate transitions and compare invariants while leaving game-specific legality and reference models in each game. | Non-adoption: Rulepath does not replace named rule tests, golden traces, replay/hash checks, simulations, or no-leak matrices with property testing alone. |
| `EXT-7` | [Michael Nygard, Documenting Architecture Decisions](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) | Explicit ADR status and supersession keep small consequential decisions reviewable. | Non-adoption: Rulepath does not make blog-style ADR prose itself binding; accepted repo ADRs and foundation docs remain the authority. |
| `EXT-8` | [boardgame.io Game API](https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/api/Game.md) | Generic setup validation and per-player state projection are useful comparative patterns. | Non-adoption: Rulepath does not adopt JavaScript rule authority, boardgame.io plugin/state architecture, or weaker replay/export visibility taxonomy. |

## Private-lane prior art and non-adoption notes

These notes record the private-lane readiness prior art cited by the change
plan. They are provenance and boundary notes only. Rulepath adopts the lesson,
not the external system, workflow, license posture, hosting model, or source
expression.

| Source | Rulepath lesson | Non-adoption rationale |
|---|---|---|
| Rally the Troops / GMT-style hosted modules | Hosted modules can support complex licensed tabletop games when rights, hosting, account, and module boundaries are explicit. | Rulepath does not adopt a hosted private module marketplace, licensed title presentation, account/server runtime, or public artifact that names private games. Private licensed work stays in isolated private repositories/builds. |
| VASSAL Engine | A mature module ecosystem shows that private/local tabletop modules can carry complex maps, decks, scenarios, and player-driven workflows. | Rulepath does not adopt VASSAL's broad module-authoring model or player-policed rules posture. Rulepath keeps Rust-owned legality, replay, hashes, visibility, and no-leak tests. |
| boardgame.io | Public/private player views, phases, and web presentation are useful comparative shapes for catalog and renderer seams. | Rulepath does not adopt JavaScript rule authority, boardgame.io plugins, multiplayer/lobby assumptions, or a weaker replay/hash/export boundary. Rust/WASM remains behavior authority. |
| OpenSpiel | Generic game APIs and observation/information-state separation are useful reminders for imperfect-information boundaries. | Rulepath does not adopt OpenSpiel's research stack, search/RL agenda, game abstraction layer, or public bot policy. Public v1/v2 still exclude MCTS, ISMCTS, Monte Carlo, ML, and RL bots. |
| GitHub reusable workflows and checkout | Private CI can call public reusable checks against a pinned public commit without public CI knowing private game names. | Rulepath does not adopt a public workflow that fetches, names, or depends on the private repository. Public CI remains private-blind; private CI owns private overlay/drift proof. |
| Cargo workspaces, git dependencies, and registries | Rust tooling can compose code across repositories, but dependency metadata is itself a leak surface. | Rulepath does not adopt public optional dependencies, public features, public workspace members, or public registry packages that name private games. The default is a separate private repository pinned to public Rulepath. |

## Comparable web and tabletop systems

### boardgame.io

- URL: https://boardgame.io/
- Documentation: https://boardgame.io/documentation/
- Date consulted: 2026-06-05
- Evidence used: boardgame.io is a pragmatic JavaScript framework for turn-based games with concepts such as moves, phases, logs/time travel, multiplayer/lobby support, React integration, plugins, and AI hooks.
- Rulepath lesson: a useful game framework can be product-oriented and web-native without being an arbitrary tabletop language. Rulepath should learn from logs, phases, React integration, and dev tooling, but diverges by making deterministic Rust legality, public/private visibility, replay hashes, and polished public support mandatory.

### VASSAL Engine

- URL: https://vassalengine.org/
- Designer Guide: https://vassalengine.org/doc/3.7.14/designerguide/designerguide.pdf
- Reference Manual: https://vassalengine.org/doc/latest/ReferenceManual/GameModule.html
- Date consulted: 2026-06-05
- Evidence used: VASSAL supports broad module authoring for board/card/tabletop games and explicitly treats broad rules enforcement as undesirable or costly in many modules. It favors player-driven tabletop-style play with targeted automation.
- Rulepath lesson: VASSAL is a strong precedent for broad module ecosystems and a strong anti-example for Rulepath's core bet. Rulepath chooses full legal action generation and rule enforcement, so it must pay with typed game modules, tests, traces, replay, visibility safety, and benchmarks. Rulepath should not pretend to get VASSAL-level breadth with stricter legality for free.

### Tabletop Playground / boardgame.io integration

- URL: https://tabletopplayground.com/knowledge-base/using-boardgame-io/
- Date consulted: 2026-06-05
- Evidence used: Tabletop Playground can use boardgame.io as a logic layer while the tabletop environment supplies visualization/networking.
- Rulepath lesson: separating rules from presentation is a practical pattern. Rulepath pushes that split harder: Rust owns rules and TypeScript owns presentation.

## Game-description and general game-playing research

### Ludii and ludemes

- URL: https://ludii.games/
- Paper: https://arxiv.org/abs/1905.05013
- Universality paper: https://arxiv.org/abs/2205.00451
- Logic guide: https://arxiv.org/abs/2101.02120
- Date consulted: 2026-06-05
- Evidence used: Ludii represents games through structured ludemes and targets broad game-description generality with AI/tooling support. Its research emphasizes expressiveness, efficiency, extensibility, and understandable descriptions.
- Rulepath lesson: serious generality requires a formal representation, validation, examples, tooling, performance work, and long-term research discipline. Rulepath should remain typed-Rust-first until repeated public games prove that a language is worth the cost.

### Regular Boardgames / RBG

- Overview paper: https://arxiv.org/abs/1706.02462
- Efficient reasoning paper: https://arxiv.org/abs/2006.08295
- Date consulted: 2026-06-05
- Evidence used: RBG is a formal language for finite deterministic perfect-information games, using automata/regular-expression ideas and compiler optimizations for efficient reasoning.
- Rulepath lesson: efficient rule languages need compiler culture and benchmark culture. Untyped content files are the wrong place to hide rule behavior. Any future Rulepath DSL must be typed, lowered, benchmarked, versioned, and replay-aware.

### Regular Games

- Paper: https://arxiv.org/html/2511.10593v1
- Date consulted: 2026-06-05
- Evidence used: Regular Games extends automata-based game-description work and frames general game descriptions as a language/tooling ecosystem, not a few config fields.
- Rulepath lesson: generality without tooling is fantasy. Rulepath should not invent a mini-language casually because one implementation feels repetitive.

## AI, bots, and simulations

### OpenSpiel

- Documentation: https://openspiel.readthedocs.io/en/latest/intro.html
- Repository: https://github.com/google-deepmind/open_spiel
- Paper: https://arxiv.org/abs/1908.09453
- Date consulted: 2026-06-05
- Evidence used: OpenSpiel is a research framework for reinforcement learning, search, planning, and game-theoretic algorithms across many game classes, including imperfect-information and simultaneous-move games.
- Rulepath lesson: fast native game cores are valuable for AI experiments, but OpenSpiel is not a consumer UI model. Rulepath should not chase ML/RL or MCTS before public play, fairness, replay, no-leak boundaries, and explanations work.

### Board Game Arena: Bots and Artificial Intelligence

- URL: https://en.boardgamearena.com/doc/Bots_and_Artificial_Intelligence
- Date consulted: 2026-06-05
- Evidence used: BGA guidance frames bot support as game-specific and often closer to custom solo/automa behavior than a universal AI solution.
- Rulepath lesson: provide bot infrastructure and random legal bots, but keep real strategy in game-specific modules.

### Board Game Arena: Zombie Mode

- URL: https://en.boardgamearena.com/doc/Zombie_Mode
- Date consulted: 2026-06-05
- Evidence used: BGA uses automated replacement behavior for players who leave, with practical categories ranging from random to smarter game-specific behavior.
- Rulepath lesson: a bot ladder from random legal to rule-informed to authored policy is practical and product-oriented.

### Board Game Arena: Using AI for BGA game development

- URL: https://en.boardgamearena.com/doc/Using_AI_for_BGA_game_development
- Date consulted: 2026-06-05
- Evidence used: BGA's AI-development guidance treats coding agents as useful for bounded tasks, support work, docs, tests, and structured assistance, not complete unattended game implementation.
- Rulepath lesson: agents need bounded tasks, source grounding, forbidden changes, failing-test protocol, and complete-file/coherent-section outputs.

### Practical board-game bot architecture notes

- URL: https://xebia.com/blog/writing-board-game-ai-bots-the-good-the-bad-and-the-ugly/
- Date consulted: 2026-06-05
- Evidence used: practical bot writeups emphasize separating game state, moves, agents, and an arena/referee that prevents cheating.
- Rulepath lesson: public bots should be supervised by the same legal validation path as humans. The bot should choose; the engine should validate and apply.

## Blackjack placement audit references

Date consulted: 2026-06-08.

These sources support [ADR 0006](adr/0006-blackjack-lite-roadmap-placement.md). They are research references, not permission to copy rules prose, casino presentation, source examples, screenshots, cards, table layouts, or trade dress.

| Source | URL | Evidence used | Rulepath lesson |
|---|---|---|---|
| Pagat Blackjack | https://www.pagat.com/banking/blackjack.html | Standard Blackjack includes betting circles, dealer up/down-card deal protocols, insurance, dealer blackjack check, player action choices, settlement, and casino-rule variation. | Blackjack is not just hidden draw/stand; it is a bundle of dealer automation, private dealer state, accounting, and optional actions. |
| Wizard of Odds Blackjack basics | https://wizardofodds.com/games/blackjack/basics/ | Summarizes beating the dealer, bust, card valuation, natural blackjack, dealer hole card, insurance, peek, push, and settlement. | Even a stripped implementation must decide which natural, hole-card, ace-valuation, and settlement rules are in scope. |
| Wizard of Odds Blackjack rule variations | https://wizardofodds.com/games/blackjack/rule-variations/ | Shows rule variants such as soft 17, surrender, double/split options, no-hole-card impacts, and payout changes materially affect expected return. | Variant choice is mechanical behavior, not cosmetic data. Do not admit Blackjack without a tight Rust-owned variant contract. |
| Encyclopaedia Britannica Blackjack | https://www.britannica.com/topic/blackjack-card-game | Identifies Blackjack as a casino/gambling card game and notes natural, split, double-down, dealer-vs-player framing, and variable house rules. | Public naming/UI should avoid casino vibes where a neutral original mechanic can do the job. |
| Gymnasium Blackjack environment | https://gymnasium.farama.org/environments/toy_text/blackjack/ | Presents Blackjack as a stochastic decision problem with hit/stick actions, dealer reveal/draw policy, usable ace observation, rewards, and Sutton & Barto provenance. | Blackjack remains computationally non-trivial even when stripped to hit/stick; Rulepath should not use RL/ML for public bots, but the source confirms mechanical weight. |
| RLCard | https://rlcard.org/ | A reinforcement-learning toolkit for card games and imperfect-information games. | Blackjack-like card games belong in a higher-complexity family; Rulepath should keep public bots Rust-owned and non-ML unless an ADR changes policy. |
| OpenSpiel concepts | https://openspiel.readthedocs.io/en/latest/concepts.html | Represents trajectories as game trees and models chance as an explicit player/chance node. | Rulepath can learn from explicit chance/hidden-info modeling without adopting a general-game framework or search/RL product direction. |
| Ludii universality paper | https://arxiv.org/abs/2205.00451 | Discusses finite non-deterministic and imperfect-information games in a game-description-language context. | Generality is expensive and should not be smuggled in via Blackjack pressure. |
| GOPS / Game of Pure Strategy reference | https://coppercod.games/game/gops/ | Describes simultaneous closed bids for a face-up prize with no luck. | Simultaneous commitment/bid pressure can be separated from dealer/accounting/casino pressure. |
| Bicycle Thirty-One | https://bicyclecards.com/how-to-play/thirty-one | A draw/count threshold family with card values and closest-to-threshold scoring, while still tagged casino. | If draw/stand threshold pressure is desired, use an original non-casino design rather than public Blackjack branding. |

## Web UI, rendering, and accessibility

### React

- URL: https://react.dev/
- Learn docs: https://react.dev/learn
- Date consulted: 2026-06-05
- Evidence used: React is a component model for building web user interfaces from reusable pieces.
- Rulepath lesson: React is appropriate for the app shell, panels, setup flow, replay UI, accessibility wrappers, and renderer orchestration. React must not own rules or legality.

### SVG

- URL: https://developer.mozilla.org/en-US/docs/Web/SVG
- SVG title/accessibility reference: https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title
- Date consulted: 2026-06-05
- Evidence used: SVG is a DOM-integrated, scalable vector standard for two-dimensional graphics and can carry accessible names/descriptions.
- Rulepath lesson: React + SVG is the right v1 default for modest board/card object counts, clean scaling, accessible labels, and inspectable debug overlays.

### Canvas API

- URL: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
- Date consulted: 2026-06-05
- Evidence used: Canvas supports script-driven drawing for animation, game graphics, visualization, and real-time rendering.
- Rulepath lesson: Canvas is a later option when profiling shows SVG object/animation pressure. It is not the default just because it sounds more game-like.

### PixiJS

- URL: https://pixijs.com/8.x/guides/getting-started/intro
- Date consulted: 2026-06-05
- Evidence used: PixiJS is a high-performance 2D rendering engine built around WebGL and optionally WebGPU.
- Rulepath lesson: PixiJS is a legitimate later renderer for heavier scenes, but it increases complexity and should be earned by profiling or ADR.

### Reduced motion

- URL: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@media/prefers-reduced-motion
- W3C technique: https://www.w3.org/WAI/WCAG22/Techniques/css/C39
- Date consulted: 2026-06-05
- Evidence used: browsers can detect a user's preference to reduce non-essential motion, and WCAG techniques describe using that preference to reduce problematic animation.
- Rulepath lesson: effect-driven animation must include a reduced-motion path from the start of animation work.

### Keyboard and ARIA practices

- WAI APG keyboard interface: https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/
- WAI APG grid pattern: https://www.w3.org/WAI/ARIA/apg/patterns/grid/
- MDN ARIA overview: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA
- Date consulted: 2026-06-05
- Evidence used: custom widgets require explicit keyboard support and accessible names/roles; ARIA helps only when used correctly.
- Rulepath lesson: Rust action trees can give the UI a strong accessibility backbone. Do not rely on pointer-only SVG interaction.

## Rust and WebAssembly

### MDN Rust to WebAssembly

- URL: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm
- Date consulted: 2026-06-05
- Evidence used: Rust can compile to WebAssembly and integrate into existing JavaScript frontends.
- Rulepath lesson: Rust-first rules and a polished TypeScript/React web app are compatible. Use Rust for behavior, not for the entire public UI by default.

### wasm-bindgen

- URL: https://wasm-bindgen.github.io/wasm-bindgen/
- Date consulted: 2026-06-05
- Evidence used: wasm-bindgen provides high-level Rust/JavaScript interop for WebAssembly modules.
- Rulepath lesson: a `wasm-api` crate can expose TypeScript-friendly calls while keeping legality, views, previews, replay, effects, and bots in Rust.

### wasm-pack

- URL: https://rustwasm.github.io/docs/wasm-pack/introduction.html
- Current ecosystem docs: https://wasm-bindgen.github.io/wasm-pack/
- Date consulted: 2026-06-05
- Evidence used: wasm-pack packages Rust-generated WebAssembly for JavaScript workflows and npm-style integration.
- Rulepath lesson: package the Rust rule boundary cleanly and treat moved/updated documentation as a maintenance signal.

### Rust WebAssembly page

- URL: https://www.rust-lang.org/what/wasm/
- Date consulted: 2026-06-05
- Evidence used: Rust/Wasm is positioned as a way to augment JavaScript with performance-sensitive or low-level tasks.
- Rulepath lesson: deterministic rules, simulation, replay, serialization, and bot hot loops belong in Rust; TypeScript should present results.

### WebAssembly project

- URL: https://webassembly.org/
- Date consulted: 2026-06-05
- Evidence used: WebAssembly is a portable compilation target for web and server environments.
- Rulepath lesson: compiling the same Rust rule code to browser WASM and native server/tool binaries preserves a path from local-first play to future authoritative server execution.

## Data formats and schema discipline

### Serde

- URL: https://serde.rs/
- Date consulted: 2026-06-05
- Evidence used: Serde separates Rust data structures from supported serialization formats.
- Rulepath lesson: typed schema deserialization is the right boundary. The format is not the rules engine.

### serde_yaml

- URL: https://docs.rs/serde_yaml/latest/serde_yaml/
- Date consulted: 2026-06-05
- Evidence used: the crate page states the project is no longer maintained.
- Rulepath lesson: YAML is banned by default. The maintenance status strengthens the ban; the deeper reason is that YAML invites accidental untyped behavior languages.

### TOML

- Spec: https://toml.io/en/
- Rust crate: https://docs.rs/toml/latest/toml/
- Date consulted: 2026-06-05
- Evidence used: TOML is intended as a readable configuration format with Rust/Serde ecosystem support.
- Rulepath lesson: use TOML for manifests, simple options, metadata, and narrow typed variant selection.

### JSON / serde_json

- RFC: https://datatracker.ietf.org/doc/html/rfc8259
- Rust crate: https://docs.rs/serde_json/latest/serde_json/
- Date consulted: 2026-06-05
- Evidence used: JSON is a lightweight interchange format and has strong Serde support.
- Rulepath lesson: use JSON for browser payload fixtures, golden traces, replay summaries, and machine-readable reports. Do not use JSON as hand-authored behavior.

### RON

- URL: https://docs.rs/ron/latest/ron/
- Date consulted: 2026-06-05
- Evidence used: RON is Rusty Object Notation and supports Serde-shaped Rust data including structs and enums.
- Rulepath lesson: use RON for Rust-shaped fixtures and enum-heavy typed content. RON does not make behavior-in-data acceptable.

### CSV

- URL: https://docs.rs/csv/latest/csv/
- Date consulted: 2026-06-05
- Evidence used: Rust's `csv` crate supports fast reading/writing and Serde integration.
- Rulepath lesson: use CSV for tabular card/component lists, scoring tables, coverage exports, balance tables, and benchmark reports.

### Postcard

- URL: https://docs.rs/postcard/latest/postcard/
- Date consulted: 2026-06-05
- Evidence used: Postcard is a compact Serde format with a documented stable wire format as of 1.0.
- Rulepath lesson: Postcard may be useful for compact internal snapshots, caches, or benchmark artifacts. It is not the default public replay interchange and must not be hand-authored rules.

## Next-phase scaling sources

Date consulted: 2026-06-13.

These sources support public scaling-phase planning. They support rule facts,
multi-seat concepts, and implementation comparisons only. Rulepath prose,
assets, names, UI, tests, traces, and bot explanations remain original and
project-owned or compatibly licensed.

| Source | URL | Evidence used | Rulepath lesson |
|---|---|---|---|
| Pagat Texas Hold'em | https://www.pagat.com/poker/variants/texasholdem.html | Public rules reference for shared community cards, betting rounds, showdown, and player counts. | Useful facts for a future Hold'Em-family spec; do not copy prose or casino presentation. |
| Pagat poker hand ranking | https://www.pagat.com/poker/rules/ranking.html | Hand-ranking order and comparison concepts. | Evaluation facts belong in Rust, with original explanation copy and no copied tables. |
| Pagat Hearts | https://www.pagat.com/reverse/hearts.html | Four-seat trick-taking, passing, scoring, and shoot-the-moon family facts. | Useful for Gate 16 variant research; source facts do not replace Rulepath rules prose. |
| Pagat Oh Hell | https://www.pagat.com/exact/ohhell.html | Variable-player trick-taking with bidding and changing hand sizes. | Useful for variable-N trick/bid pressure; final variant must be scoped in a game source note. |
| Pagat Spades | https://www.pagat.com/auctionwhist/spades.html | Partnership trick-taking and contract scoring family facts. | Useful for team/partnership pressure; Rulepath must choose and document a public-safe variant. |
| Pagat 500 Rum | https://www.pagat.com/rummy/500rum.html | Rummy 500 family draw/discard/meld/scoring facts. | Candidate source for a future rummy-family gate; final source selection remains TBD by that spec. |
| Chinese Checkers / Star Halma reference | https://www.pagat.com/race/chinese_checkers.html | Star board, jump movement, and multi-seat race-game facts. | Useful for large-surface topology pressure; Rulepath presentation and rules prose remain original. |
| OpenSpiel concepts | https://openspiel.readthedocs.io/en/latest/concepts.html | Multi-player game modeling, observations, imperfect information, and chance concepts. | Architecture comparison only; Rulepath does not adopt RL/search as public bot policy. |
| OpenSpiel paper | https://arxiv.org/abs/1908.09453 | Research framework for many game classes and algorithms. | Confirms the breadth of N-player/imperfect-information research without changing Rulepath's public bot law. |
| boardgame.io | https://boardgame.io/ | Web game framework with turns, phases, logs, and multiplayer concepts. | Useful comparison for presentation/tooling concepts; Rulepath keeps Rust as behavior authority. |
| Pachisi-family source | TBD by future Gate 21 spec | Candidate public-domain race-game source still needs final selection. | Do not write the game spec until the source note chooses stable public rules facts. |
| Mahjong-family source | TBD by future Gate 22 spec | Candidate scoped meld/reaction family source still needs final selection. | Future spec should avoid a sprawling clone and document an original scoped variant. |

### Texas Hold'Em rules family / River Ledger

Date consulted: 2026-06-14.

These sources support Gate 15 River Ledger planning and implementation. They
verify public rules-family facts only: private hole cards, community board
structure, street order, betting-round vocabulary, showdown, split/tie concepts,
and standard poker hand-ranking categories. They do not authorize copied rules
prose, hand-ranking tables, examples, card art, screenshots, scans, fonts,
icons, casino layout, tournament branding, product names, or trade dress.

Rulepath's public product identity is **River Ledger** (`river_ledger`). Its
rules prose, player help, UI copy, asset posture, bot explanations, tests, and
trace fixtures must remain original and project-owned or compatibly licensed.
Public presentation uses abstract contribution units and must avoid real-money,
rake, payout, tournament, and casino-product framing.

| Source | URL | Evidence used | Rulepath lesson |
|---|---|---|---|
| Pagat Texas Hold'em | https://www.pagat.com/poker/variants/texasholdem.html | Rules-family reference for hole cards, community cards, streets, betting rounds, showdown, and table-size context. | River Ledger may implement the neutral rules-family mechanics, but must write original prose and avoid casino presentation. |
| Pagat poker hand ranking | https://www.pagat.com/poker/rules/ranking.html | Hand category order and comparison concepts, including straight/flush/full-house style ranking. | Evaluator and showdown explanation belong in Rust; copied ranking tables or examples do not belong in docs or UI. |
| Fournier Texas Hold'em explainer | https://www.nhfournier.es/en/como-jugar/texas-holdem-poker/ | Secondary check on shared board, street, and showdown family shape. | Use only as corroborating context, not as wording, visual, or product-presentation source. |
| OpenSpiel concepts | https://openspiel.readthedocs.io/en/latest/concepts.html | Prior-art vocabulary for imperfect information, observations, players, and game trajectories. | Useful comparison for N-seat hidden-information modeling; Rulepath does not adopt OpenSpiel's architecture or public search/RL bots. |
| OpenSpiel paper | https://arxiv.org/abs/1908.09453 | Research context for imperfect-information and multi-player games. | Confirms the complexity of this problem space without changing Rulepath's public bot law. |
| boardgame.io | https://boardgame.io/ | Web-game framework comparison for phases, turns, logs, and player views. | Presentation/tooling precedent only; Rulepath keeps Rust as behavior authority. |

## Replay, deterministic logs, and future multiplayer

### Gaffer on Games: Deterministic Lockstep

- URL: https://gafferongames.com/post/deterministic_lockstep/
- Date consulted: 2026-06-05
- Evidence used: deterministic lockstep sends inputs and relies on identical deterministic simulation, while highlighting how fragile determinism can be.
- Rulepath lesson: command logs, deterministic RNG, stable hashes, and replay discipline matter from the first game even before networking. This does not commit Rulepath to peer-to-peer lockstep.

### Gaffer on Games: Floating Point Determinism

- URL: https://gafferongames.com/post/floating_point_determinism/
- Date consulted: 2026-06-05
- Evidence used: floating point determinism depends on strict environmental constraints and is a known source of desync concern.
- Rulepath lesson: avoid floating point in rule decisions unless an ADR defines exact constraints.

### Gabriel Gambetta: client-server game architecture

- URL: https://www.gabrielgambetta.com/client-server-game-architecture.html
- Client-side prediction: https://www.gabrielgambetta.com/client-side-prediction-server-reconciliation.html
- Date consulted: 2026-06-05
- Evidence used: authoritative server architecture treats clients as input sources and the server as owner of authoritative state, with prediction/reconciliation for responsiveness.
- Rulepath lesson: future hosted multiplayer should use an authoritative Rust server. Browser clients may preview locally; they must not own authoritative state.

### Game Programming Patterns: Command

- URL: https://gameprogrammingpatterns.com/command.html
- Date consulted: 2026-06-05
- Evidence used: command objects can decouple input from execution and support replay by executing recorded commands through the normal simulation path.
- Rulepath lesson: action paths, command envelopes, command streams, replay, and trace hashes should be architecture primitives.

## Copyright, trademarks, trade dress, assets, and fonts

### U.S. Copyright Office: Games

- URL: https://www.copyright.gov/register/tx-games.html
- Date consulted: 2026-06-05
- Evidence used: copyrightable parts of a game may include literary and pictorial expression.
- Rulepath lesson: public files must use original or permissioned rules prose and assets even when implementing neutral mechanics.

### U.S. Copyright Office: what copyright protects

- URL: https://www.copyright.gov/what-is-copyright/
- FAQ: https://www.copyright.gov/help/faq/faq-protect.html
- Date consulted: 2026-06-05
- Evidence used: copyright protects expression, not ideas, procedures, methods, systems, concepts, principles, or facts.
- Rulepath lesson: mechanics and systems are not the same as expressive rulebook text, art, logos, component presentation, or card text. Implement mechanics carefully and write original prose.

### USPTO copyright basics

- URL: https://www.uspto.gov/ip-policy/copyright-policy/copyright-basics
- Date consulted: 2026-06-05
- Evidence used: copyright protects original works fixed in a tangible medium and includes literary, artistic, software, and game-related expression.
- Rulepath lesson: Rulepath source code, original rules summaries, and original assets are protectable project work; copied expression remains risky.

### Baker v. Selden

- URL: https://supreme.justia.com/cases/federal/us/101/99/
- Date consulted: 2026-06-05
- Evidence used: the decision distinguishes copyright in explanatory expression from rights in a functional system or method.
- Rulepath lesson: the idea/expression distinction supports neutral implementations, but Rulepath remains conservative because the public portfolio must be clean.

### USPTO trademark basics

- URL: https://www.uspto.gov/trademarks/basics
- Date consulted: 2026-06-05
- Evidence used: trademarks identify source and distinguish goods/services.
- Rulepath lesson: neutral public IDs reduce avoidable source-confusion risk for commercial or living brands.

### Trade dress overview

- URL: https://www.nycbar.org/reports/21st-century-trademark-basics/
- Date consulted: 2026-06-05
- Evidence used: trade dress concerns the visual impression or appearance of goods and services when it functions as source identity.
- Rulepath lesson: avoid distinctive commercial presentation, component styling, color-as-identity, packaging-like layouts, screenshots, and product mimicry.

### U.S. Copyright Office AI reports

- URL: https://www.copyright.gov/ai/
- Date consulted: 2026-06-05
- Evidence used: the Copyright Office has issued AI-related reports, including one on copyrightability of generative-AI outputs.
- Rulepath lesson: AI-generated assets require human review and replaceability. Do not treat generated output as automatically safe, original, or licensable.

### Font licensing references

- Google Fonts licensing glossary: https://fonts.google.com/knowledge/glossary/licensing
- Open Font License FAQ: https://openfontlicense.org/ofl-faq/
- Date consulted: 2026-06-05
- Evidence used: fonts are licensed assets; even free fonts have license terms, and open font licenses define redistribution/modification conditions.
- Rulepath lesson: never ship font files without verified redistribution rights and preserved license notes. Prefer system fonts or clearly open-licensed fonts.

## Public rules reference examples for future game work

These sources can help verify variants. They do not grant permission to copy prose.

### Bicycle Cards how-to-play library

- URL: https://bicyclecards.com/how-to-play
- Date consulted: 2026-06-05
- Rulepath lesson: useful for common card-game variant checks; Rulepath must still write original summaries.

### Pagat

- URL: https://www.pagat.com/
- Trick-taking overview: https://www.pagat.com/class/trick.html
- Date consulted: 2026-06-05
- Rulepath lesson: useful for card-game variant research, especially trick-taking. Do not copy text.

### World Draughts Federation / FMJD

- URL: https://www.fmjd.org/
- Date consulted: 2026-06-05
- Rulepath lesson: useful for draughts/checkers variant verification; `draughts_lite` must document simplified deviations.

### Encyclopaedia Britannica

- URL: https://www.britannica.com/
- Ticktacktoe entry: https://www.britannica.com/topic/ticktacktoe
- Checkers summary: https://www.britannica.com/summary/checkers
- Date consulted: 2026-06-05
- Rulepath lesson: useful secondary context for classic games, not a source to copy.

### Berkeley GamesCrafters

- URL: https://gamescrafters.berkeley.edu/games.php?game=tictactoe
- Date consulted: 2026-06-05
- Rulepath lesson: useful for solved/simple-game context and tiny-game reasoning; Rulepath should still maintain its own tests and rules summary.
