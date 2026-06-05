# Rulepath Agent Discipline

Status: operational law for coding agents and AI-assisted documentation work.

Claude Code, Codex, and similar agents are accelerators. They are not unattended architects. Agent work MUST be bounded, testable, source-grounded, explicit about forbidden changes, and delivered as complete files or coherent complete sections, not diffs.

## 1. Agent role

Agents MAY:

- implement bounded game-module behavior in typed Rust;
- add tests, traces, simulations, and benchmarks;
- write or update per-game documentation from verified rules and implemented behavior;
- build UI components against stable Rust/WASM contracts;
- improve diagnostics, replay tooling, benchmark reports, or fixture validation;
- refactor narrow code after the mechanic atlas permits it.

Agents MUST NOT:

- invent major architecture without ADR;
- generalize from one game;
- add game/mechanic nouns to `engine-core`;
- make static data procedural;
- add YAML by convenience;
- create a DSL;
- put legality in TypeScript;
- let bots inspect hidden information unavailable to their seat;
- add private licensed content to public files;
- delete or weaken tests to get green output;
- optimize without benchmark evidence;
- produce sprawling unreviewable changes.

## 2. Required task packet

Every agent task SHOULD state:

```text
Context: relevant foundation docs, ADRs, game docs, source notes
Target: exact crate/game/doc/tool being changed
Stage/gate: roadmap stage and mechanics being proved
Goal: observable behavior or document outcome
Non-goals: areas not to touch
Forbidden changes: kernel, data format, DSL, UI, bot, IP, primitive boundaries
Sources/docs: rule sources, source notes, coverage rows, ADRs
Tests: required unit/rule/golden/property/simulation/replay/visibility/bot/UI tests
Benchmarks: baseline or non-regression evidence
Docs: required updates
Output format: complete files or coherent complete sections, not diffs
Handoff: commands run, results, unresolved questions
```

A task without forbidden changes invites architecture drift.

## 3. Good and bad tasks

Good tasks are bounded:

```text
Implement Stage 3 `column_four` legal action generation, validation, semantic effects, golden traces, Level 1 bot legality tests, and per-game docs. Do not modify `engine-core`. Update the mechanic inventory and atlas notes. Output complete files or coherent sections, not diffs.
```

```text
Add viewer-filtered effect-log tests for `high_card_duel`. First decide whether failing tests are still valid, then whether the issue is the SUT or tests. Add regression coverage for any bug found.
```

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

When tests fail, agents and humans MUST:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or in the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report changes.

Tests MUST NOT be deleted, weakened, renamed away, or rewritten merely to get green output.

## 5. Kernel-change protocol

Any `engine-core` change MUST answer:

1. Which already implemented official games require this?
2. Why can the change not live inside `games/*`?
3. Why can the change not live inside `game-stdlib` after earned pressure?
4. Does it introduce any game noun, mechanic noun, strategy, renderer concern, network concern, or storage concern?
5. Does it preserve deterministic replay, visibility boundaries, serialization compatibility, and hashes?
6. Does it require ADR?

Default answer: do not change `engine-core`.

## 6. Data protocol

Agents MUST use approved v1/v2 formats only:

- TOML for manifests and simple configuration;
- JSON for traces, replay interchange, fixtures, and machine reports;
- RON for Rust-shaped fixtures and enum-heavy typed content;
- CSV for tables;
- Postcard or equivalent compact Serde formats only for approved non-hand-authored internal artifacts.

Agents MUST reject unknown fields by default and flag behavior-looking fields such as `when`, `if`, `then`, `else`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, `ai_condition`, `effect_script`, `rule`, and `requires`.

Agents MUST NOT use “data-driven rules” approvingly.

## 7. DSL protocol

Agents MUST NOT create a DSL.

A future DSL requires ADR and must address typed semantics, lowering/compilation, source spans, formatting, linting, versioning, tests, benchmarks, examples, anti-examples, determinism, replay/hash implications, visibility, hidden defaults, agent safety, and migration.

A DSL MUST NOT be introduced to rescue one complex game or private experiment.

## 8. Primitive-pressure protocol

Agents MUST update per-game mechanic inventories and the repo-level atlas when mechanics repeat.

Hard gate: a third official game with the same mechanic shape MUST NOT proceed until the primitive-pressure ledger says one of:

- reuse an existing primitive;
- promote a narrow typed helper;
- explicitly defer/reject with rationale;
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
- provide/update a strategy evidence pack before Level 2 authored policy bots.

Public v1/v2 agents MUST NOT add MCTS, ISMCTS, Monte Carlo bots, ML, or RL.

## 10. UI protocol

Agents implementing UI MUST:

- build controls from Rust legal action trees;
- request Rust previews for partial/compound actions;
- submit action paths with freshness tokens;
- animate from viewer-filtered semantic effects;
- settle visual state to the latest public view;
- keep debug panels behind a dev toggle and viewer-safe data source;
- protect hidden information in DOM, logs, local storage, payloads, test IDs, and replay exports;
- support reduced motion when animations are added;
- avoid proprietary mimicry and debug-console-first layouts.

TypeScript MUST NOT decide legality.

## 11. IP protocol

Agents MUST:

- write original rules summaries;
- cite consulted rules sources in source notes;
- avoid copied rulebook prose, proprietary card text, proprietary assets, screenshots, scans, font files without verified redistribution rights, and trade dress;
- use neutral names when commercial trademark or presentation risk exists;
- keep private licensed experiments out of public files, public CI, public docs, public traces, public bundles, and public WASM/JS;
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

## 13. Review check

Before accepting agent work, verify:

- the task stayed inside scope;
- forbidden changes were not made;
- failing-test protocol was followed;
- Rust remains behavior authority;
- TypeScript remains presentation-only;
- `engine-core` remains noun-free;
- static data remains typed content/parameters only;
- replay/hash determinism is preserved;
- hidden information is safe;
- bots are fair and explainable;
- tests/traces/simulations/benchmarks/docs were updated;
- IP boundaries are preserved;
- output is reviewable without reconstructing patches.
