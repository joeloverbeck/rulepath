# Rulepath Sources

Status: supporting bibliography for the Rulepath foundation document set.

Last reviewed: 2026-06-05.

These sources inform Rulepath's architecture, authoring discipline, bot policy, UI policy, data-format policy, replay model, and IP caution. They are evidence and precedent, not text to copy. Game-specific `docs/SOURCES.md` files must still document the exact rules sources, variants, naming rationale, and asset status for each implemented game.

## Source-use rules

Rulepath source notes MUST:

- summarize in original language;
- prefer official documentation, primary papers, and reputable references;
- distinguish evidence from Rulepath policy;
- avoid copying rulebook prose, card text, UI copy, board art, screenshots, icons, fonts, or trade dress;
- record date consulted;
- record variant decisions and deviations for each implemented game;
- mark uncertain or unverified sources instead of inventing support.

## Comparable game systems

### boardgame.io

- URL: <https://boardgame.io/>
- Documentation: <https://boardgame.io/documentation/>
- Date consulted: 2026-06-05
- Evidence used: boardgame.io presents itself as an open-source engine for turn-based games. Its public materials emphasize state management, moves, phases, logs, multiplayer, lobby support, AI hooks, plugins, React integration, and time-travel/debugging.
- Rulepath lesson: a pragmatic web turn-based framework can be useful without being an arbitrary tabletop language. Logs, phases, React integration, and dev tooling are worth studying. Rulepath differs by making deterministic Rust legality, replay, visibility safety, and polished public presentation mandatory.

### VASSAL

- URL: <https://vassalengine.org/>
- Designer Guide: <https://vassalengine.org/doc/3.7.14/designerguide/designerguide.pdf>
- Date consulted: 2026-06-05
- Evidence used: VASSAL supports a large ecosystem of tabletop modules and online play. Its designer guidance warns module authors not to attempt broad rule enforcement and treats rule automation as targeted assistance rather than full authority.
- Rulepath lesson: VASSAL is a strong precedent for broad module ecosystems, but it is also the anti-example for Rulepath's core bet. Rulepath chooses full legal action generation and rule enforcement, so it must pay with typed game modules, tests, traces, replay, visibility safety, and benchmarks.

## General game description research

### Ludii and ludemes

- URL: <https://ludii.games/>
- Paper: <https://arxiv.org/abs/1905.05013>
- Date consulted: 2026-06-05
- Evidence used: Ludii represents games using structured ludemes and targets broad board/card/dice/game generality with AI support and tooling.
- Rulepath lesson: serious generality requires a formal representation, validation, examples, tooling, performance work, and long-term research discipline. Rulepath must not imply near-term arbitrary tabletop support.

### Regular Boardgames / RBG

- Overview paper: <https://arxiv.org/abs/1706.02462>
- Efficient Reasoning paper: <https://arxiv.org/abs/2006.08295>
- Date consulted: 2026-06-05
- Evidence used: RBG uses a formal language based on regular expressions and automata-style reasoning for finite deterministic games; the efficiency work emphasizes compiler optimization, move generation, and benchmarks.
- Rulepath lesson: efficient general game languages require compiler and benchmark culture. Untyped nested content files are the wrong place to hide rule behavior.

### Regular Games

- Paper: <https://arxiv.org/html/2511.10593v1>
- Date consulted: 2026-06-05
- Evidence used: Regular Games extends automata-based game-description work and describes an ecosystem involving languages, editor support, visualization, debugging, and benchmarks.
- Rulepath lesson: generality without tooling is fantasy. Rulepath should implement typed Rust modules until repeated, painful, stable shapes justify a language or helper extraction.

## AI/search frameworks and bot guidance

### OpenSpiel

- Documentation: <https://openspiel.readthedocs.io/en/latest/intro.html>
- Repository: <https://github.com/google-deepmind/open_spiel>
- Date consulted: 2026-06-05
- Evidence used: OpenSpiel is a framework for reinforcement learning, search, planning, and game AI research. It supports many game classes, including imperfect-information and simultaneous-move settings, with fast procedural game cores and Python exposure.
- Rulepath lesson: serious AI infrastructure benefits from fast native game cores. OpenSpiel is not a consumer UI model and is not a reason for Rulepath to chase ML/RL or MCTS before public play, fairness, replay, and bot explanations work.

### Board Game Arena: Bots and Artificial Intelligence

- URL: <https://en.doc.boardgamearena.com/Bots_and_Artificial_Intelligence>
- Date consulted: 2026-06-05
- Evidence used: BGA documentation describes bots as game-specific work and warns that bot support is custom rather than generic framework magic.
- Rulepath lesson: Rulepath should provide bot infrastructure, random legal bots, and policy composition helpers, but strategy belongs in game-specific modules.

### Board Game Arena: Zombie Mode

- URL: <https://en.doc.boardgamearena.com/Zombie_Mode>
- Date consulted: 2026-06-05
- Evidence used: BGA distinguishes random, greedy, and smarter game-specific replacement behavior.
- Rulepath lesson: a bot ladder from random legal choice to rule-informed and authored policy bots is practical and product-oriented.

### Board Game Arena: Using AI for BGA game development

- URL: <https://en.doc.boardgamearena.com/Using_AI_for_BGA_game_development>
- Date consulted: 2026-06-05
- Evidence used: BGA's guidance treats coding agents as useful for bounded support work such as boilerplate, tests, structured rules, refactoring, trace analysis, UI/CSS, and documentation, while warning that they do not reliably implement complete meaningful games unattended.
- Rulepath lesson: agents need bounded tasks, test protocols, forbidden changes, source notes, and complete-file/coherent-section outputs.

## Web UI and rendering

### React

- URL: <https://react.dev/>
- Date consulted: 2026-06-05
- Evidence used: React is a component-oriented UI library for web interfaces.
- Rulepath lesson: React is appropriate for app shell, game picker, panels, settings, replay controls, accessibility wrappers, and WASM orchestration. React must not own rule legality.

### SVG

- URL: <https://developer.mozilla.org/en-US/docs/Web/SVG>
- Date consulted: 2026-06-05
- Evidence used: SVG is a scalable, scriptable, DOM-integrated web standard for two-dimensional vector graphics.
- Rulepath lesson: React + SVG is the v1 default for early board/card renderers because object counts are modest, visuals scale cleanly, and accessibility/debug overlays are easier.

### Canvas API

- URL: <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API>
- Date consulted: 2026-06-05
- Evidence used: Canvas supports script-driven drawing for animation, games, graphics, and visualization.
- Rulepath lesson: Canvas is a later option when profiling shows SVG DOM/object pressure or custom drawing needs. It is not the default merely because it feels more game-like.

### PixiJS

- URL: <https://pixijs.com/8.x/guides/getting-started/intro>
- Date consulted: 2026-06-05
- Evidence used: PixiJS provides WebGL/WebGPU-oriented 2D rendering and scene-graph-style interaction/performance tools.
- Rulepath lesson: PixiJS is a legitimate later renderer for heavier animation or many objects, but its complexity should be earned by profiling or ADR after the renderer boundary exists.

## Rust and WebAssembly

### MDN Rust to WebAssembly

- URL: <https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm>
- Date consulted: 2026-06-05
- Evidence used: Rust can compile to WebAssembly and integrate into existing web applications.
- Rulepath lesson: Rust-first rules and a polished web app are compatible. Rulepath should expose a batched WASM boundary, not cross the JS/WASM boundary for every rule step.

### wasm-bindgen

- URL: <https://wasm-bindgen.github.io/wasm-bindgen/>
- Date consulted: 2026-06-05
- Evidence used: wasm-bindgen provides Rust/JavaScript interoperability for WebAssembly modules.
- Rulepath lesson: `wasm-api` can provide TypeScript-friendly bindings while keeping legality, previews, replay, public/private views, effects, and bots in Rust.

### wasm-pack

- Current documentation: <https://wasm-bindgen.github.io/wasm-pack/>
- Historical documentation: <https://rustwasm.github.io/docs/wasm-pack/introduction.html>
- Date consulted: 2026-06-05
- Evidence used: wasm-pack remains relevant to packaging Rust-generated WebAssembly for JavaScript workflows, and older docs point to newer wasm-bindgen-hosted documentation.
- Rulepath lesson: prefer current official docs and treat moved documentation as a maintenance signal.

### Rust WebAssembly page

- URL: <https://www.rust-lang.org/what/wasm/>
- Date consulted: 2026-06-05
- Evidence used: Rust positions WebAssembly as useful for augmenting JavaScript with performance-sensitive components.
- Rulepath lesson: deterministic rules, simulation, replay, serialization, and AI hot loops belong in Rust; TypeScript should handle presentation.

## Structured data formats

### serde_yaml

- URL: <https://docs.rs/serde_yaml/latest/serde_yaml/>
- Date consulted: 2026-06-05
- Evidence used: the crate page states that the project is no longer maintained.
- Rulepath lesson: YAML is banned by default. Maintenance status reinforces the ban, but the stronger architectural reason is that YAML too easily becomes an accidental untyped programming language.

### TOML

- URL: <https://docs.rs/toml/latest/toml/>
- Date consulted: 2026-06-05
- Evidence used: TOML has mature Rust/Serde support and fits readable configuration-style data.
- Rulepath lesson: use TOML for manifests, simple options, metadata, and narrow typed variant selection.

### JSON / serde_json

- URL: <https://docs.rs/serde_json/latest/serde_json/>
- Date consulted: 2026-06-05
- Evidence used: JSON is ubiquitous, web-friendly, and supported by Serde.
- Rulepath lesson: use JSON for browser payload fixtures, golden traces, replay summaries, and machine-readable reports, not hand-authored behavior.

### RON

- URL: <https://docs.rs/ron/latest/ron/>
- Date consulted: 2026-06-05
- Evidence used: RON is Rusty Object Notation, designed for readable Serde-shaped Rust data including structs and enums.
- Rulepath lesson: use RON for Rust-shaped fixtures and complex typed static content where TOML/JSON become awkward. RON does not make behavior-in-data acceptable.

### CSV

- URL: <https://docs.rs/csv/latest/csv/>
- Date consulted: 2026-06-05
- Evidence used: the Rust `csv` crate provides CSV reading/writing with Serde integration.
- Rulepath lesson: use CSV for tabular card lists, scoring tables, rule coverage exports, balance tables, and benchmark reports.

### Postcard

- URL: <https://docs.rs/postcard/latest/postcard/>
- Date consulted: 2026-06-05
- Evidence used: Postcard is a compact Serde format with a stable wire format.
- Rulepath lesson: Postcard may be used for compact internal snapshots, caches, benchmark fixtures, or non-hand-authored replay artifacts. It must not be hand-authored rules.

## Replay, command logs, and future multiplayer

### Gaffer on Games: Deterministic Lockstep

- URL: <https://gafferongames.com/post/deterministic_lockstep/>
- Date consulted: 2026-06-05
- Evidence used: deterministic lockstep sends inputs and requires identical results from identical initial state and inputs; it also highlights determinism hazards such as floating-point differences.
- Rulepath lesson: deterministic command logs, seed discipline, stable hashing, and replay are foundational even before networking. This does not commit Rulepath to peer-to-peer lockstep for hidden-information games.

### Gabriel Gambetta: client-server architecture

- URL: <https://www.gabrielgambetta.com/lag-compensation.html>
- Date consulted: 2026-06-05
- Evidence used: authoritative server architecture treats clients as input sources while the server owns authoritative state and sends resulting updates/snapshots.
- Rulepath lesson: future hosted multiplayer must use an authoritative Rust server. Browser clients may preview locally, but they must not own authoritative state.

### Game Programming Patterns: Command

- URL: <https://gameprogrammingpatterns.com/command.html>
- Date consulted: 2026-06-05
- Evidence used: command objects decouple producers and consumers and can support replay by executing recorded commands through the normal simulation path.
- Rulepath lesson: action paths, commands, command streams, replay, and trace hashes should be first-class architecture artifacts.

## Copyright, trademark, and IP

### U.S. Copyright Office: Games

- URL: <https://www.copyright.gov/register/tx-games.html>
- Date consulted: 2026-06-05
- Evidence used: the Copyright Office distinguishes unprotected game ideas, names, titles, methods, and systems from protectable expressive material such as rule prose and artwork.
- Rulepath lesson: neutral mechanics can often be implemented, but public files must not copy prose, card text, art, screenshots, icons, or trade dress.

### USPTO: Copyright Basics

- URL: <https://www.uspto.gov/ip-policy/copyright-policy/copyright-basics>
- Date consulted: 2026-06-05
- Evidence used: copyright protects expression, not ideas, procedures, processes, systems, methods of operation, concepts, principles, or discoveries.
- Rulepath lesson: implement mechanics carefully; write original prose and use original/compatible assets.

### Baker v. Selden

- URL: <https://supreme.justia.com/cases/federal/us/101/99/>
- Date consulted: 2026-06-05
- Evidence used: the decision distinguishes copyright in an explanatory work from exclusive rights to the system or method described.
- Rulepath lesson: this supports the idea/expression distinction, but Rulepath remains deliberately conservative because the goal is a clean public portfolio.

### USPTO: Trademark basics

- URL: <https://www.uspto.gov/trademarks/basics/what-trademark>
- Date consulted: 2026-06-05
- Evidence used: trademarks identify the source of goods or services and distinguish them from others.
- Rulepath lesson: neutral public IDs reduce avoidable source-confusion risk for commercial or living brands.

### Copyright Office FAQ: names and titles

- URL: <https://www.copyright.gov/help/faq/faq-protect.html>
- Date consulted: 2026-06-05
- Evidence used: copyright does not protect names, titles, slogans, or short phrases, but trademark may apply.
- Rulepath lesson: common names may be usable when safe, but commercial names and presentation still need caution.

### Trade dress overview

- URL: <https://www.finnegan.com/en/insights/articles/trade-dress-101.html>
- Date consulted: 2026-06-05
- Evidence used: trade dress can protect the total image or overall commercial impression of a product where source-identifying.
- Rulepath lesson: avoid proprietary component styling, visual layout, colors-as-identity, screenshots, and trade-dress mimicry.

## Public rules sources for classic games

These are examples for rule verification. They do not grant permission to copy prose.

### Bicycle Cards how-to-play library

- URL: <https://bicyclecards.com/how-to-play>
- Date consulted: 2026-06-05
- Evidence used: public rules summaries for common card games.
- Rulepath lesson: useful for checking variants and public card-game rules; Rulepath must still write original summaries.

### Pagat

- URL: <https://www.pagat.com/>
- Date consulted: 2026-06-05
- Evidence used: extensive card-game rules and variant notes, including trick-taking families.
- Rulepath lesson: useful for variant research; do not copy text.

### World Draughts Federation / FMJD

- URL: <https://www.fmjd.org/>
- Date consulted: 2026-06-05
- Evidence used: official draughts rules and federation materials.
- Rulepath lesson: use for variant verification when implementing `draughts_lite`-style games, then document simplified deviations.

### Encyclopaedia Britannica game entries

- URL: <https://www.britannica.com/>
- Date consulted: 2026-06-05
- Evidence used: general-reference descriptions of classic games such as tic-tac-toe and related history.
- Rulepath lesson: useful secondary context; still prefer original Rulepath prose and exact variant docs.

### U.S. Chess Federation rules information

- URL: <https://new.uschess.org/>
- Date consulted: 2026-06-05
- Evidence used: official chess-rule references and rulebook information.
- Rulepath lesson: useful if chesslike movement is ever studied; full chess is not an early Rulepath target.

### American Contract Bridge League

- URL: <https://www.acbl.org/>
- Date consulted: 2026-06-05
- Evidence used: official bridge organization resources.
- Rulepath lesson: useful for future trick-taking/partnership reference, not as prose to copy.
