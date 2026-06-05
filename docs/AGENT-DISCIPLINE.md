# Rulepath Agent Discipline

Status: coding-agent operating law.

Claude Code, Codex, and similar tools are expected to help implement Rulepath. They are accelerators, not unattended architects.

## 1. Agent role

Agents MAY:

- implement bounded game modules;
- add tests;
- add golden traces;
- write simulations;
- write benchmarks;
- refactor narrow modules;
- port bounded code from TypeScript to Rust when behavior belongs in Rust;
- generate docs from existing code;
- build UI components against stable contracts;
- improve diagnostics;
- analyze failures;
- update rule coverage matrices.

Agents MUST NOT:

- invent major architecture without ADR;
- generalize the engine from one game;
- add game nouns to `engine-core`;
- add YAML behavior;
- create a DSL without ADR;
- implement private licensed content in public files;
- rewrite tests blindly;
- optimize without benchmarks;
- produce sprawling changes without a bounded target;
- make bots omniscient;
- put rule legality in TypeScript;
- make public docs or names look like a private licensed-game project.

## 2. Required task structure

Every agent task SHOULD include:

```text
Context:
  What project law applies?
  What source documents should be followed?

Target:
  Which crate/module/game is being changed?

Stage:
  Which game-ladder stage is this?

Mechanics:
  Which mechanics are being tested?

Goal:
  What behavior should exist when complete?

Non-goals:
  What must not be touched?

Forbidden changes:
  What kernel changes are forbidden?
  What data formats are forbidden?
  What DSL/YAML/IP/UI/bot boundaries apply?

Tests:
  What unit/rule/golden/property/simulation/visibility/UI tests are required?

Benchmarks:
  What benchmark must exist or not regress?

Docs:
  What RULES/SOURCES/RULE-COVERAGE/UI/AI/BENCHMARKS docs must be added or updated?

Output:
  Provide complete files or coherent complete sections, not diffs.
```

## 3. Bad tasks

Do not give agents tasks like:

- “make the engine support any board game”;
- “generalize this”;
- “make the UI feel good”;
- “finish the bot”;
- “support this huge game”;
- “add YAML support for this special case”;
- “fix all failing tests” without validating the tests;
- “clean up the architecture”;
- “make it fast” without benchmark targets;
- “add MCTS”;
- “implement multiplayer”;
- “add a private game module to public repo”.

These tasks create sludge.

## 4. Good tasks

Use bounded tasks like:

```text
Implement Stage 3 `column_four` in games/column_four.
Use typed Rust only. Do not modify engine-core except to fix documented generic bugs.
Add unit tests for line detection, golden traces for two games, random bot legality tests,
CLI simulation support, and benchmarks for legal action generation and bot choice.
Update RULES.md, SOURCES.md, RULE-COVERAGE.md, UI.md, AI.md, and BENCHMARKS.md.
Output complete files or coherent complete sections, not diffs.
```

```text
Add viewer-filtered effect log tests for Stage 6 `high_card_duel`.
Do not alter rule behavior unless a valid test proves current behavior is wrong.
First determine whether failing tests are valid, then whether the issue is SUT or test suite.
Add regression coverage for any bug found.
```

```text
Refactor repeated coordinate/line helpers from `three_marks`, `column_four`, and `directional_flip`
into game-stdlib. Do not add grid concepts to engine-core. Back-port all three games and prove
existing traces still pass or document intentional trace changes.
```

## 5. Failing-test protocol

When tests fail, agents MUST follow this protocol:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report what changed.

Agents MUST NOT delete, weaken, or rewrite tests merely to get green output.

## 6. Kernel-change protocol

Any change to `engine-core` MUST answer:

- Which implemented games need it?
- Why can this not live in a game module?
- Why can this not live in `game-stdlib`?
- Does it introduce any game-specific noun?
- Does it preserve determinism?
- Does it preserve visibility boundaries?
- Does it affect replay hashes or compatibility?
- Does it require ADR?

Default answer: do not change `engine-core`.

## 7. Data-format protocol

Agents MUST use the allowed v1 static formats unless instructed otherwise:

- TOML for manifests/config;
- JSON for traces/interchange;
- RON for Rust-shaped static fixtures;
- CSV for tables;
- Postcard/binary Serde only for non-hand-authored internal artifacts when approved.

Agents MUST NOT add YAML without ADR.

Agents MUST NOT turn static data into behavior.

Suspicious fields include:

```text
when
if
then
else
selector
condition
trigger
script
loop
foreach
priority_expression
ai_condition
```

## 8. DSL protocol

Agents MUST NOT create a DSL.

A future DSL requires ADR and must be typed, compiled/lowered, source-span-aware, deterministic, formatted, linted, versioned, tested, benchmarked, documented with examples/anti-examples, and unable to silently depend on hidden defaults.

A DSL MUST NOT be introduced merely to make one monster game possible.

## 9. Bot protocol

Agents implementing bots MUST:

- use normal legal action API;
- use allowed bot views only;
- avoid omniscient state;
- implement deterministic tie-breaking;
- add legality tests over many seeds;
- add latency benchmarks;
- document strategy and information access;
- produce explanation examples for non-random bots;
- avoid weight soup.

Agents MUST NOT present a cheating diagnostic tool as a bot.

## 10. UI protocol

Agents implementing UI MUST:

- map controls from Rust legal action trees;
- avoid TypeScript rule legality;
- drive animations from semantic effects;
- keep debug panels behind a dev toggle;
- protect hidden information in DOM, local storage, logs, and payloads;
- support reduced motion where animation is added;
- use original assets and neutral presentation.

## 11. IP protocol

Agents MUST:

- write original rules summaries;
- cite/link sources where appropriate;
- avoid copied rulebook prose;
- avoid proprietary card text/assets;
- use neutral names for trademark-risk games;
- keep private licensed experiments out of public files;
- leave a TODO for human review when unsure.

If an agent is unsure whether content is safe, it MUST omit the content and ask for human review.

## 12. Output protocol

For code corrections, agents SHOULD output complete files or coherent complete sections. Do not output diffs as the primary artifact.

For documentation corrections, agents SHOULD output whole Markdown files when practical.

For large work, agents SHOULD report:

- files changed;
- tests added;
- benchmarks added;
- docs added;
- boundary decisions;
- unresolved questions;
- commands to run.

## 13. Review checklist

Before accepting agent output, verify:

- `engine-core` has no game nouns;
- rule behavior is Rust, not untyped data;
- TypeScript does not decide legality;
- tests cover the change;
- golden traces updated only when rule change is intentional;
- replay remains deterministic;
- hidden information is safe;
- bots use legal action API and allowed views;
- benchmarks exist for hot paths;
- public files contain no licensed data;
- docs match implementation;
- ADR exists for major decisions;
- output is bounded and coherent.

## 14. Why strictness is required

Major board-game platform guidance treats AI tools as useful for bounded tasks such as boilerplate, tests, refactoring, trace analysis, and UI code, while warning that they do not reliably implement meaningful complete games unattended. Rulepath should exploit agent speed without surrendering architecture.

## Source notes

See `SOURCES.md`, especially Board Game Arena AI-development guidance, Board Game Arena bot guidance, testing doctrine, data-format sources, and ADR doctrine.
