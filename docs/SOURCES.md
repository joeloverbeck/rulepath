# SOURCES

Status: supporting bibliography for the foundation document set.

These sources informed the repository law, architecture direction, and risk controls. They are not incorporated by quotation; the project documents use them as evidence and precedent.

## Comparable game systems

### boardgame.io

- URL: https://boardgame.io/
- Official documentation: https://boardgame.io/documentation/
- Relevance: boardgame.io demonstrates a pragmatic web-first turn-based-game framework that provides state management, multiplayer support, logs, plugins, lobby support, and React bindings while letting game authors write ordinary game logic.
- Project lesson: useful precedent for a small, pragmatic API surface; not evidence that a universal formal tabletop language or generic bot system is easy.

### VASSAL

- URL: https://vassalengine.org/
- User guide: https://obj.vassalengine.org/images/8/8c/Userguide.pdf
- Designer guide: https://vassalengine.org/doc/3.7.14/designerguide/designerguide.pdf
- Relevance: VASSAL has scaled as a tabletop module system partly because it does not generally enforce rules or provide computer opponents. Its designer documentation explicitly warns against trying to enforce rules broadly.
- Project lesson: this project is choosing the harder path by enforcing rules; therefore it MUST pay the cost in tests, legal-action APIs, replay, performance, and game-specific implementations.

### Ludii

- URL: https://ludii.games/
- Overview paper: https://arxiv.org/abs/1905.05013
- Relevance: Ludii is a general game system based on ludemes, supporting many board, card, dice, and mathematical games, plus AI/search tooling and a large game database.
- Project lesson: serious generality is possible, but it resembles a research program with formal language design, validation, tooling, and years of accumulated structure. This project MUST NOT pretend to reach that level in v1.

### OpenSpiel

- Documentation: https://openspiel.readthedocs.io/en/latest/intro.html
- Repository: https://github.com/google-deepmind/open_spiel
- Relevance: OpenSpiel is a framework for reinforcement learning, search, planning, and game AI research. Its core API and many games are implemented in C++ with Python exposure.
- Project lesson: AI/search frameworks keep the computational game core separate from friendly presentation and care about fast procedural state transitions. This supports a Rust-first engine and native benchmark doctrine.

### Regular Boardgames / Regular Games

- Regular Boardgames overview: https://ojs.aaai.org/index.php/AAAI/article/view/8424
- Efficient Reasoning in Regular Boardgames: https://arxiv.org/abs/2006.07953
- Regular Games: https://arxiv.org/html/2511.10593v1
- Relevance: these systems emphasize compiled/formal game representations, automata, optimization, and playout benchmarks.
- Project lesson: if a future DSL exists, it MUST be typed, compiled/lowered, tested, benchmarked, and designed as a real language. Untyped data-driven behavior is the wrong center of gravity.

### Board Game Arena bots and AI-development guidance

- Bots and Artificial Intelligence: https://en.doc.boardgamearena.com/Bots_and_Artificial_Intelligence
- Zombie Mode: https://en.doc.boardgamearena.com/Zombie_Mode
- Using AI for BGA game development: https://en.doc.boardgamearena.com/Using_AI_for_BGA_game_development
- Relevance: BGA documentation states that bot support is custom, not generally framework-provided; its zombie mode distinguishes random, greedy, and smarter automated replacement behavior; its AI-development guidance warns that AI tools are useful for bounded tasks but not reliable unattended implementers.
- Project lesson: game-specific bots are normal; random and greedy baselines are valid early milestones; coding agents must be constrained by documentation, tests, and narrow task boundaries.

## Rust and WebAssembly

### MDN Rust to WebAssembly guide

- URL: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm
- Relevance: documents compiling Rust to WebAssembly and using the result inside an existing web application.
- Project lesson: Rust-first does not block a public web app.

### wasm-bindgen

- URL: https://wasm-bindgen.github.io/wasm-bindgen/
- Relevance: Rust library and CLI for high-level interaction between Wasm modules and JavaScript, including TypeScript bindings.
- Project lesson: `wasm-api` SHOULD expose a small batched browser-facing API rather than making TypeScript own game rules.

### wasm-pack

- URL: https://rustwasm.github.io/docs/wasm-pack/introduction.html
- Relevance: tooling for building Rust-generated WebAssembly packages for JavaScript workflows.
- Project lesson: static-site deployment with Rust/WASM plus a TypeScript shell is practical.

### Rust language WebAssembly page

- URL: https://www.rust-lang.org/what/wasm/
- Relevance: Rust positions WebAssembly as a way to augment JavaScript for processing-heavy or low-level tasks.
- Project lesson: engine hot loops belong in Rust; JavaScript/TypeScript remains appropriate for UI and browser integration.

## Rust structured-data formats

### serde_yaml

- URL: https://docs.rs/serde_yaml/latest/serde_yaml/
- Relevance: the crate page marks `serde_yaml` 0.9.34+deprecated and says the project is no longer maintained.
- Project lesson: YAML MUST NOT be the default v1 format; any YAML use REQUIRES ADR and a strong reason.

### TOML

- URL: https://docs.rs/toml/latest/toml/
- Relevance: TOML is a Serde-compatible readable configuration format and is used by Cargo.
- Project lesson: TOML is suitable for manifests, simple game metadata, variants, and build-time config.

### JSON / serde_json

- URL: https://docs.rs/serde_json/latest/serde_json/
- Relevance: JSON has broad web interoperability and Serde support.
- Project lesson: JSON is suitable for browser-facing API payloads, golden traces, fixtures, and machine-readable reports.

### RON

- URL: https://docs.rs/ron/latest/ron/
- Relevance: RON is a Rusty Object Notation format designed for readable Serde data structures.
- Project lesson: RON is suitable for complex Rust-shaped static data when JSON/TOML become awkward but behavior is still not being encoded.

### CSV

- URL: https://docs.rs/csv/latest/csv/
- Relevance: CSV is a practical format for tabular data with mature Rust support.
- Project lesson: CSV is suitable for card lists, test tables, scoring tables, and content matrices.

### Postcard

- URL: https://docs.rs/postcard/latest/postcard/
- Relevance: Postcard is a compact no-std Serde format with a stable wire format.
- Project lesson: compact binary snapshots MAY be considered for local cache or replay artifacts, but not for hand-authored rules.

## Multiplayer, replay, and command logs

### Gaffer on Games: Deterministic Lockstep

- URL: https://gafferongames.com/post/deterministic_lockstep/
- Relevance: explains sending inputs instead of state and defines determinism as identical results from identical initial conditions and inputs.
- Project lesson: command logs and deterministic replay are foundation features. Deterministic lockstep is a useful model, but future hosted multiplayer still needs hidden-information and trust boundaries.

### Gabriel Gambetta: client-server game architecture / lag compensation

- URL: https://www.gabrielgambetta.com/lag-compensation.html
- Relevance: summarizes authoritative server architecture: clients send inputs; server processes and sends snapshots/updates.
- Project lesson: future online multiplayer MUST use an authoritative Rust server. Browsers must not own authoritative state.

### Game Programming Patterns: Command

- URL: https://gameprogrammingpatterns.com/command.html
- Relevance: describes replay through recorded command execution rather than state snapshots.
- Project lesson: command logs support replay/debugging and MUST be first-class.

## Copyright, game rules, and IP

### U.S. Copyright Office: Games

- URL: https://www.copyright.gov/register/tx-games.html
- Relevance: states that the idea for a game, name/title, and methods for playing are not protected by copyright, while rule text and gameboard art may be protected if expressive enough.
- Project lesson: public implementations MUST use original rule summaries, original assets, and safe naming/presentation.

### USPTO Copyright Basics

- URL: https://www.uspto.gov/ip-policy/copyright-policy/copyright-basics
- Relevance: explains the idea-expression dichotomy: copyright protects expression, not ideas, procedures, systems, or methods of operation.
- Project lesson: implementing mechanics is not the same as copying rulebook prose, art, card text, or proprietary presentation.

### Baker v. Selden

- URL: https://supreme.justia.com/cases/federal/us/101/99/
- Relevance: foundational U.S. case often cited for the distinction between copyright in a descriptive work and exclusive rights in the system it describes.
- Project lesson: useful legal background, but project policy remains stricter than the bare minimum: do not create public IP risk.

## Public rules sources and classic games

These sources are examples for rule-source notes. They do not grant permission to copy prose; use them to verify rules and write original summaries.

- Bicycle Cards: https://bicyclecards.com/how-to-play
- Pagat card-game rules: https://www.pagat.com/
- Britannica game entries: https://www.britannica.com/
- World Draughts Federation: https://www.fmjd.org/
- American Contract Bridge League: https://www.acbl.org/
- U.S. Chess Federation rules information: https://new.uschess.org/

## UI and rendering

### React

- URL: https://react.dev/
- Relevance: React builds user interfaces from components and is appropriate for application shell, panels, logs, settings, and stateful UI composition.
- Project lesson: React SHOULD own app structure and inspector panels, not rule legality.

### MDN SVG

- URL: https://developer.mozilla.org/en-US/docs/Web/SVG
- Relevance: SVG is a text-based open web standard for two-dimensional vector graphics, scriptable with DOM/JavaScript and scalable without loss of quality.
- Project lesson: SVG is the recommended v1 board renderer for low-to-moderate piece counts, clean abstract presentation, accessibility, and debuggability.

### MDN Canvas API

- URL: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
- Relevance: Canvas provides JavaScript drawing for animation, game graphics, visualization, and real-time processing.
- Project lesson: Canvas MAY replace or supplement SVG when measured board complexity or animation load requires it.

### PixiJS

- URL: https://pixijs.com/8.x/guides/getting-started/intro
- Relevance: PixiJS is a WebGL/WebGPU 2D renderer with scene graph, interaction support, and high-performance rendering.
- Project lesson: PixiJS SHOULD be a later upgrade path, not the first dependency added before SVG/Canvas have proven insufficient.
