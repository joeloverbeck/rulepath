# AGENT DISCIPLINE

Status: coding-agent operating law.

Claude Code, Codex, and similar tools are expected to help implement the first games. They are accelerators, not unattended architects.

## 1. Agent role

Agents MAY:

- implement bounded game modules;
- add tests;
- add golden traces;
- write simulations;
- write benchmarks;
- refactor narrow modules;
- port bounded code from TypeScript to Rust;
- generate docs from existing code;
- build UI components against stable contracts;
- improve diagnostics;
- analyze failures.

Agents MUST NOT:

- invent major architecture without ADR;
- generalize the engine from one game;
- add game nouns to `engine-core`;
- add YAML behavior;
- create a DSL without ADR;
- implement private licensed content in public files;
- rewrite tests blindly;
- optimize without benchmarks;
- produce sprawling changes without a bounded target.

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
  What IP boundaries apply?

Tests:
  What unit/rule/golden/property/simulation/UI tests are required?

Benchmarks:
  What benchmark must exist or not regress?

Docs:
  What rules/source/coverage/UI/AI docs must be added or updated?

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
- “make it fast” without benchmark targets.

These tasks create sludge.

## 4. Good tasks

Use bounded tasks like:

```text
Implement Stage 1 four-in-a-row in games/four_in_a_row.
Use typed Rust only. Do not modify engine-core except to fix documented bugs.
Add unit tests for line detection, golden traces for two games, random bot legality tests,
CLI simulation support, and a benchmark for legal action generation.
Update RULES.md and RULE-COVERAGE.md in the game module.
Output complete files or coherent complete sections, not diffs.
```

```text
Add viewer-filtered effect log tests for the Stage 4 card smoke game.
Do not alter rule behavior unless a test proves the current behavior is wrong.
First determine whether failing tests are valid, then whether the issue is SUT or test suite.
Add regression coverage for any bug found.
```

```text
Refactor repeated grid-coordinate helpers from tic_tac_toe and four_in_a_row into game-stdlib.
Do not add grid concepts to engine-core.
Back-port both games to the helper and prove all existing traces still pass.
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
- Does it require ADR?

Default answer: do not change `engine-core`.

## 7. Data-format protocol

Agents MUST use the allowed v1 static formats unless instructed otherwise:

- TOML for manifests/config;
- JSON for traces/interchange;
- RON for Rust-shaped static fixtures;
- CSV for tables.

Agents MUST NOT add YAML without ADR.

Agents MUST NOT turn static data into behavior.

## 8. IP protocol

Agents MUST:

- write original rules summaries;
- cite/link sources where appropriate;
- avoid copied rulebook prose;
- avoid proprietary card text/assets;
- use neutral names for trademark-risk games;
- keep private licensed experiments out of public files.

If an agent is unsure whether content is safe, it MUST omit the content and leave a TODO for human review.

## 9. Output protocol

For code corrections, agents SHOULD output complete files or coherent complete sections.

Do not output diffs as the primary artifact.

For large work, agents SHOULD report:

- files changed;
- tests added;
- benchmarks added;
- docs added;
- boundary decisions;
- unresolved questions;
- commands to run.

## 10. Review checklist

Before accepting agent output, verify:

- `engine-core` has no game nouns;
- rule behavior is Rust, not untyped data;
- tests cover the change;
- golden traces updated only when rule change is intentional;
- replay remains deterministic;
- hidden information is safe;
- bots use legal action API;
- benchmarks exist for hot paths;
- public files contain no licensed data;
- docs match implementation;
- ADR exists for major decisions.

## 11. Why this is strict

Public developer guidance from major board-game platforms recognizes AI tools as useful for bounded tasks such as boilerplate, tests, refactoring, trace analysis, and UI code, while warning that they do not reliably implement meaningful complete games unattended. This project should exploit agent speed without surrendering architecture.

## Source notes

See `SOURCES.md`, especially Board Game Arena AI-development guidance, Board Game Arena bot guidance, the testing doctrine, and ADR doctrine.
