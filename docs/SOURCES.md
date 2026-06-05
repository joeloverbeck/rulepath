# Rulepath Sources

Status: researched bibliography for the foundation set.

Last reviewed: 2026-06-05.

These sources inform Rulepath architecture, authoring discipline, bot policy, UI policy, data-format policy, replay model, and IP caution. They are precedents and warnings, not text to copy. Game-specific source notes must still document exact rules sources, chosen variants, naming rationale, and asset status.

## Source-use rules

Rulepath source notes MUST:

- summarize in original language;
- prefer official documentation, primary papers, and reputable references;
- distinguish evidence from Rulepath policy;
- avoid copying rulebook prose, card text, UI copy, board art, screenshots, icons, fonts, or trade dress;
- record date consulted;
- record variant decisions and deviations for each implemented game;
- mark uncertainty instead of inventing support.

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
