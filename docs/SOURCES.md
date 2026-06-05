# Rulepath Sources

Status: supporting bibliography for the Rulepath foundation document set.

Last reviewed: 2026-06-05.

These sources inform Rulepath's architectural laws, risk controls, and staged implementation doctrine. They are evidence and precedent, not source material to copy. Game-specific `docs/SOURCES.md` files must still be created for each implemented public game.

## Source-use rules

Rulepath source notes MUST follow these rules:

- summarize in original language;
- cite official documentation, primary papers, or reputable references when possible;
- avoid copying rulebook prose, card text, UI copy, board art, screenshots, icons, or trade dress;
- distinguish project policy from legal conclusions;
- document the date consulted for game-rule sources;
- document chosen variants and deviations from common variants;
- treat old or unmaintained tooling documentation as a warning sign, not as stable foundation.

## Comparable game systems and research precedents

### boardgame.io

- URL: <https://boardgame.io/>
- Documentation: <https://boardgame.io/documentation/>

Evidence used:

- boardgame.io presents itself as an open-source engine for turn-based games.
- Its public feature list includes state management, multiplayer, AI bots, phases, lobby support, plugins, React integration, logging, and time-travel/debugging support.
- Its model is pragmatic: game authors write game logic as ordinary code while the framework supplies surrounding turn-based infrastructure.

Rulepath lesson:

- A small web-first turn-based framework surface is a useful precedent.
- Logs, phases, plugins, React integration, and development tooling are worth studying.
- boardgame.io is not evidence that arbitrary tabletop support, generic bots, or a universal rule language are easy.
- Rulepath differs by making deterministic Rust rule enforcement, legal-action generation, visibility filtering, replay, and polished public presentation hard requirements.

### VASSAL

- URL: <https://vassalengine.org/>
- Designer Guide: <https://vassalengine.org/doc/3.7.14/designerguide/designerguide.pdf>
- User Guide: <https://obj.vassalengine.org/images/8/8c/Userguide.pdf>

Evidence used:

- VASSAL has a large tabletop module ecosystem and supports online board/wargame play.
- The VASSAL Designer Guide explicitly discourages broad rule enforcement and advises designers to target automation narrowly.
- VASSAL scales partly because it lets players enforce many rules themselves, as they would at a physical table.

Rulepath lesson:

- VASSAL is the anti-example for Rulepath's central bet.
- Rulepath chooses full rule enforcement; therefore it MUST pay the cost through typed rules, legal-action APIs, visibility-safe public views, tests, golden traces, replays, benchmarks, and game-specific implementation discipline.
- Rulepath MUST NOT import the VASSAL module mindset and then pretend full rules will emerge from data/assets.

### Ludii and ludemes

- URL: <https://ludii.games/>
- Paper: <https://arxiv.org/abs/1905.05013>

Evidence used:

- Ludii is a general game system that represents games as structured sets of ludemes.
- Ludii targets broad generality across board, card, dice, and mathematical games and includes AI/search tooling.
- The Ludii paper evaluates goals such as generality, extensibility, understandability, and efficiency.

Rulepath lesson:

- Serious general game systems require formal language/tooling discipline.
- Generality is possible, but it resembles a research program with a designed representation, validation, examples, performance work, and years of accumulated library design.
- Rulepath MUST NOT claim near-term arbitrary tabletop support.
- A future Rulepath DSL, if ever justified, must be treated as a typed, compiled/lowered, source-span-aware, linted, formatted, versioned, benchmarked language project.

### Regular Boardgames / RBG

- RBG overview: <https://arxiv.org/abs/1706.02462>
- Efficient Reasoning in Regular Boardgames: <https://arxiv.org/abs/2006.08295>

Evidence used:

- Regular Boardgames uses a formal language based on regular expressions and automata-like reasoning for finite deterministic perfect-information games.
- The RBG efficiency work focuses on compiler optimizations, fast move generation, and benchmark comparisons.

Rulepath lesson:

- Efficient general game descriptions require compiler and benchmark culture.
- Untyped nested data is the wrong foundation for procedural behavior.
- If Rulepath ever adds a language, it MUST have a lowering/compilation story, test corpus, benchmark suite, diagnostics, and anti-examples.

### Regular Games

- Paper: <https://arxiv.org/html/2511.10593v1>

Evidence used:

- Regular Games extends automata-based general game work toward a broader class of finite turn-based games, including imperfect-information concerns.
- The described ecosystem emphasizes languages, editor support, visualization, benchmark culture, and debugging tools.

Rulepath lesson:

- Generality without tooling is fantasy.
- The right v1 move is not to design a universal language; it is to implement games in typed Rust until repeated, painful, stable shapes justify abstraction.

### OpenSpiel

- Documentation: <https://openspiel.readthedocs.io/en/latest/intro.html>
- Repository: <https://github.com/google-deepmind/open_spiel>

Evidence used:

- OpenSpiel is a framework for reinforcement learning, search, planning, and game AI research.
- It supports many game classes, including imperfect information and simultaneous-move settings.
- Its core game API and many implementations are in C++, with Python exposure.

Rulepath lesson:

- Serious AI/search infrastructure benefits from fast procedural game cores.
- OpenSpiel is not a public consumer web-app architecture.
- Rulepath should keep rule transitions and AI hot loops native and deterministic, then expose a polished web UI separately.
- ML/RL remains outside Rulepath v1/v2; OpenSpiel is evidence for clean game-core/AI infrastructure, not a reason to chase research AI before public play works.

### Board Game Arena bot and AI-development guidance

- Bots and Artificial Intelligence: <https://en.doc.boardgamearena.com/Bots_and_Artificial_Intelligence>
- Zombie Mode: <https://en.doc.boardgamearena.com/Zombie_Mode>
- Using AI for BGA game development: <https://en.doc.boardgamearena.com/Using_AI_for_BGA_game_development>

Evidence used:

- BGA documentation describes bots as custom game-specific work rather than general framework support.
- Zombie Mode distinguishes random, greedy, and more tailored automated behavior for replacing absent players.
- BGA's AI-development guidance treats coding agents as useful for bounded support work such as boilerplate, tests, trace analysis, structured rules, refactoring, and UI/CSS, while warning that they cannot reliably implement complete meaningful games unattended.

Rulepath lesson:

- Random legal bots and rule-informed baselines are valid early milestones.
- Polished bots should be game-specific policy modules with explicit information access and explanations.
- Coding agents must be constrained by bounded tasks, tests, non-goals, kernel-change protocols, and complete-file/coherent-section outputs.

## Web UI and rendering references

### React

- URL: <https://react.dev/>

Evidence used:

- React is a component-oriented UI library for web and native interfaces.

Rulepath lesson:

- React/TypeScript is appropriate for the app shell, layout, menus, panels, settings, replay controls, accessibility wrappers, and WASM integration.
- React MUST NOT own game legality.

### SVG

- URL: <https://developer.mozilla.org/en-US/docs/Web/SVG>

Evidence used:

- SVG is a text-based open web standard for two-dimensional vector graphics.
- SVG is scriptable through the DOM/JavaScript and scalable without quality loss.

Rulepath lesson:

- React + SVG is the v1 default board/card renderer for early ladder games with modest object counts.
- SVG supports clean abstract visuals, inspectability, debug overlays, responsive scaling, and accessibility hooks.

### Canvas API

- URL: <https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API>

Evidence used:

- Canvas supports JavaScript-driven drawing for animation, game graphics, data visualization, and other real-time rendering uses.

Rulepath lesson:

- Canvas MAY supplement or replace SVG when measured object counts, animation load, or DOM overhead justify it.
- Canvas is not a v1 default merely because it feels more game-like.

### PixiJS

- URL: <https://pixijs.com/8.x/guides/getting-started/intro>

Evidence used:

- PixiJS provides WebGL/WebGPU 2D rendering, scene-graph-style rendering capabilities, interaction support, and performance-oriented features.

Rulepath lesson:

- PixiJS is a legitimate later renderer option for heavier animation or many visual objects.
- It should not enter before the renderer boundary exists and SVG/Canvas pressure has been measured.

## Rust and WebAssembly references

### MDN Rust to WebAssembly

- URL: <https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm>

Evidence used:

- Rust can be compiled to WebAssembly and used inside an existing web application.
- wasm-pack and wasm-bindgen are documented as part of the Rust-to-WASM workflow.

Rulepath lesson:

- Rust-first rules and a consumer web app are compatible.
- Rulepath should expose a small batched WASM API rather than crossing the JS/WASM boundary in rule hot loops.

### wasm-bindgen

- URL: <https://wasm-bindgen.github.io/wasm-bindgen/>

Evidence used:

- wasm-bindgen provides Rust and JavaScript interop for WebAssembly modules.

Rulepath lesson:

- `wasm-api` can expose TypeScript-friendly bindings while keeping rules, legal actions, previews, replay, public/private views, semantic effects, and bot decisions in Rust.

### wasm-pack

- Current documentation: <https://wasm-bindgen.github.io/wasm-pack/>
- Historical documentation: <https://rustwasm.github.io/docs/wasm-pack/introduction.html>

Evidence used:

- The older `rustwasm.github.io` wasm-pack documentation now points to the current wasm-bindgen-hosted documentation.
- wasm-pack remains the relevant packaging/build tool for Rust-generated WebAssembly packages in JavaScript workflows.

Rulepath lesson:

- The foundation docs should prefer current official documentation and treat moved docs as maintenance signals.

### Rust WebAssembly page

- URL: <https://www.rust-lang.org/what/wasm/>

Evidence used:

- Rust positions WebAssembly as useful for augmenting JavaScript with processing-heavy or lower-level components.

Rulepath lesson:

- Rust should own deterministic rules, simulations, replay, and AI hot loops; TypeScript should own browser presentation and orchestration.

## Rust structured-data formats

### serde_yaml

- URL: <https://docs.rs/serde_yaml/latest/serde_yaml/>

Evidence used:

- The `serde_yaml` crate page says the project is no longer maintained.

Rulepath lesson:

- YAML MUST NOT be the default v1 format.
- This maintenance status strengthens the ban, but the deeper reason is behavioral: YAML previously became an accidental untyped programming language.

### TOML

- URL: <https://docs.rs/toml/latest/toml/>

Evidence used:

- TOML is readable configuration-style data and has mature Serde support in Rust.

Rulepath lesson:

- TOML is appropriate for manifests, simple variants, metadata, and build-time configuration.

### JSON / serde_json

- URL: <https://docs.rs/serde_json/latest/serde_json/>

Evidence used:

- JSON is a ubiquitous text format with strong Serde support and web interoperability.

Rulepath lesson:

- JSON is appropriate for browser-facing payloads, golden traces, replay summaries, and machine-readable reports.

### RON

- URL: <https://docs.rs/ron/latest/ron/>

Evidence used:

- RON is Rusty Object Notation, designed for readable Serde-shaped Rust data including structs and enums.

Rulepath lesson:

- RON is appropriate for complex Rust-shaped fixtures and static content where TOML/JSON become awkward.
- RON does not make behavior-in-data acceptable.

### CSV

- URL: <https://docs.rs/csv/latest/csv/>

Evidence used:

- The Rust `csv` crate provides mature CSV reading/writing and Serde integration.

Rulepath lesson:

- CSV is appropriate for tabular content: card lists, scoring tables, rule coverage exports, balance tables, and benchmark reports.

### Postcard

- URL: <https://docs.rs/postcard/latest/postcard/>

Evidence used:

- Postcard is a compact Serde format with a stable wire format.

Rulepath lesson:

- Postcard MAY be used for compact snapshots, caches, benchmark fixtures, or replay artifacts.
- It MUST NOT be used for hand-authored rules.

## Multiplayer, replay, and command-log references

### Gaffer on Games: Deterministic Lockstep

- URL: <https://gafferongames.com/post/deterministic_lockstep/>

Evidence used:

- Deterministic lockstep sends inputs rather than full state and requires identical results from identical initial conditions and inputs.
- The article warns about determinism hazards such as floating-point differences.

Rulepath lesson:

- Deterministic command logs, seed discipline, hashes, and replay are foundational even before online multiplayer.
- This does not mean Rulepath should choose peer-to-peer lockstep for hidden-information games.

### Gabriel Gambetta: client-server game architecture

- URL: <https://www.gabrielgambetta.com/lag-compensation.html>

Evidence used:

- Authoritative server architecture treats clients as input sources, while the server owns the authoritative state and sends resulting updates/snapshots.

Rulepath lesson:

- Future hosted multiplayer MUST use an authoritative Rust server.
- Browser clients MUST NOT own authoritative state.

### Game Programming Patterns: Command

- URL: <https://gameprogrammingpatterns.com/command.html>

Evidence used:

- Command streams can decouple command producers from command consumers and support replay by executing recorded commands through the normal simulation.

Rulepath lesson:

- Commands/action paths should be first-class artifacts for replay, debugging, simulation, and future network validation.

## Copyright, rules, and IP references

### U.S. Copyright Office: Games

- URL: <https://www.copyright.gov/register/tx-games.html>

Evidence used:

- The Copyright Office distinguishes unprotected game ideas, names/titles, and methods of play from protectable expressive material such as rule prose and artwork.

Rulepath lesson:

- Implementing neutral mechanics is different from copying rulebook prose, assets, card text, board art, icons, screenshots, or trade dress.
- Public games need original rules summaries, original presentation, and source notes.

### USPTO Copyright Basics

- URL: <https://www.uspto.gov/ip-policy/copyright-policy/copyright-basics>

Evidence used:

- Copyright protects expression, not ideas, procedures, processes, systems, methods of operation, concepts, or principles.

Rulepath lesson:

- Rulepath may implement mechanics carefully, but public files MUST avoid copying expressive materials.

### Baker v. Selden

- URL: <https://supreme.justia.com/cases/federal/us/101/99/>

Evidence used:

- The decision distinguishes copyright in an explanatory work from exclusive rights to the system or method described.

Rulepath lesson:

- This is useful background for idea/expression separation, but Rulepath policy remains deliberately conservative: avoid public IP risk unless permission is clear.

## Public rules references for classic games

These sources are examples for verifying public/classic rules. They do not grant permission to copy prose.

- Bicycle Cards: <https://bicyclecards.com/how-to-play>
- Pagat card-game rules: <https://www.pagat.com/>
- Encyclopaedia Britannica game entries: <https://www.britannica.com/>
- World Draughts Federation: <https://www.fmjd.org/>
- American Contract Bridge League: <https://www.acbl.org/>
- U.S. Chess Federation rules information: <https://new.uschess.org/>

Rulepath lesson:

- Each implemented classic game MUST document the exact variant and rules sources consulted.
- Public docs MUST rewrite rules in original language.
- Public assets and presentation MUST be original and neutral.
