# Rulepath Agent Discipline

Status: operational law for coding agents and AI-assisted documentation work.

Claude Code, Codex, and similar agents are allowed accelerators. They are not unattended architects. Rulepath work given to an agent MUST be bounded, testable, source-grounded, and explicit about forbidden changes.

## 1. Agent role

Agents MAY:

- implement bounded game-module behavior in typed Rust;
- add tests, traces, simulations, and benchmarks;
- write or update per-game documentation from verified rules and implemented behavior;
- build UI components against stable Rust/WASM contracts;
- improve diagnostics, replay tooling, or benchmark reports;
- refactor narrow code after the mechanic atlas and primitive-pressure process permit it.

Agents MUST NOT:

- invent major architecture without ADR;
- generalize from one game;
- add game nouns to `engine-core`;
- make static data procedural;
- add YAML by convenience;
- create a DSL without ADR;
- put legality in TypeScript;
- let bots inspect hidden information unavailable to their seat;
- add private licensed content to public files;
- delete or weaken tests just to get green output;
- optimize without benchmark evidence;
- produce sprawling unreviewable changes.

## 2. Required task structure

Every agent task SHOULD state:

- context: which foundation documents, ADRs, game docs, and source notes apply;
- target: exact crate, game, doc, or template being changed;
- stage: ladder gate and mechanics being proved;
- goal: observable behavior or document outcome;
- non-goals: areas that must not be touched;
- forbidden changes: kernel, data-format, DSL, UI, bot, IP, and primitive boundaries;
- sources/docs: required rules, source notes, coverage rows, templates, and ADRs;
- tests: required unit, rule, golden, property, simulation, replay, visibility, bot, and UI smoke tests;
- benchmarks: required baseline or non-regression evidence;
- docs: required updates to `RULES.md`, `SOURCES.md`, `RULE-COVERAGE.md`, `AI.md`, `UI.md`, `BENCHMARKS.md`, and mechanic inventories;
- output format: complete files or coherent complete sections, not diffs.

Use `templates/AGENT-TASK.md` for repeatable prompts.

## 3. Good and bad tasks

Good tasks are narrow:

> Implement Stage 3 `column_four` legal action generation, validation, semantic effects, golden traces, Level 1 bot legality tests, and per-game docs. Do not modify `engine-core`. Update the mechanic inventory and primitive-pressure notes. Output complete files or coherent sections, not diffs.

> Add viewer-filtered effect-log tests for `high_card_duel`. First decide whether failing tests are still valid, then whether the issue is the SUT or tests. Add regression coverage for any bug found.

> Compare `three_marks`, `column_four`, and `directional_flip` mechanic inventories. Prepare a primitive-pressure ledger entry for coordinate and line/direction helpers. Do not extract until the ledger says reuse, promote, or defer.

Bad tasks are vague or architecture-seeking:

- “make the engine support any board game”;
- “generalize this”;
- “add YAML support for convenience”;
- “finish the bot”;
- “make the UI feel good”;
- “fix all tests” without validating test intent;
- “add MCTS”;
- “implement multiplayer”;
- “clean up the architecture”;
- “port a private licensed game into the public repo.”

## 4. Failing-test protocol

Agents and humans must follow the [failing-test protocol](INVARIANTS.md#1-failing-test-protocol): tests must not be deleted, weakened, renamed away, or rewritten merely to get green output.

## 5. Kernel-change protocol

Any `engine-core` change must answer the [kernel-change protocol](INVARIANTS.md#2-kernel-change-protocol). Default answer: do not change `engine-core`.

## 6. Data-format protocol

Agents MUST use approved v1 data formats only:

- TOML for manifests and simple configuration;
- JSON for traces, replay interchange, fixtures, and machine reports;
- RON for Rust-shaped fixtures and enum-heavy typed content;
- CSV for tables;
- Postcard or equivalent compact Serde formats only for approved non-hand-authored internal artifacts.

Agents MUST reject unknown fields by default and flag behavior-looking fields such as `when`, `if`, `then`, `else`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, and `ai_condition`.

Agents MUST NOT use the phrase “data-driven rules” approvingly. In Rulepath, the approved boundary is data-driven content and parameters, with typed Rust behavior.

## 7. DSL protocol

Agents MUST NOT create a DSL.

A future DSL requires ADR and MUST address typed semantics, lowering/compilation, source spans, formatting, linting, versioning, tests, benchmarks, examples, anti-examples, determinism, replay/hash implications, visibility, hidden defaults, and migration.

A DSL MUST NOT be introduced to rescue one complex game or private experiment.

## 8. Primitive-pressure protocol

Agents MUST update per-game mechanic inventories and the repo-level mechanic atlas when mechanics repeat.

Hard gate: a third official game with the same mechanic shape MUST NOT proceed until the primitive-pressure ledger says one of:

- reuse an existing primitive;
- promote a narrow typed helper;
- explicitly defer with rationale;
- require ADR.

Agents MUST NOT “clean up” repeated mechanics into `engine-core`. Earned helpers belong in `game-stdlib`, and only after the atlas process permits them.

## 9. Bot protocol

Agents implementing bots MUST:

- use the same legal action API as humans;
- choose legal action paths through normal validation;
- use only the allowed view for that seat;
- implement deterministic tie-breaking;
- avoid omniscient state, hidden-state shortcuts, and weight soup;
- add legality tests over many seeds;
- add determinism tests, explanation examples, and latency benchmarks;
- add no-leak tests for hidden-information games;
- provide or update a strategy evidence pack before Level 2 authored policy bots.

Diagnostic full-state tools are allowed locally. They are not public bots.

## 10. UI protocol

Agents implementing UI MUST:

- build controls from Rust legal action trees;
- request Rust previews for partial/compound actions;
- submit action paths with freshness markers;
- animate from viewer-filtered semantic effects;
- settle visual state to the latest public view;
- keep debug panels behind a dev toggle;
- protect hidden information in DOM, logs, local storage, payloads, test IDs, and replay exports;
- support reduced motion when animations are added;
- avoid proprietary mimicry and debug-console-first layouts.

TypeScript MUST NOT decide legality.

## 11. IP protocol

Agents MUST:

- write original rules summaries;
- cite consulted rules sources;
- avoid copied rulebook prose, proprietary card text, proprietary assets, screenshots, scans, fonts without verified redistribution rights, and trade dress;
- use neutral names when commercial trademark or presentation risk exists;
- keep private licensed experiments out of public files, public CI, public docs, public traces, and public bundles;
- leave a human/legal review note when unsure.

If content ships to an unauthorized browser, it has shipped.

## 12. Output protocol

Agents SHOULD output complete files or coherent complete sections. Diffs are not the primary deliverable.

For substantial work, report:

- files changed;
- tests added or updated;
- traces added or intentionally changed;
- simulations and benchmarks run;
- docs updated;
- boundary decisions;
- unresolved questions;
- commands a human should run.

## 13. Review checklist

Before accepting agent work, verify the [universal acceptance invariants](INVARIANTS.md#3-universal-acceptance-invariants). They cover Rust behavior ownership, noun-free `engine-core`, earned `game-stdlib`, no behavior in untyped data, TypeScript with no legality, effect-driven animation, deterministic replay, hidden-information safety, fair bots, test/trace/simulation/benchmark coverage, no licensed/private content, documentation matching implementation, ADRs for major decisions, and bounded reviewable output.
