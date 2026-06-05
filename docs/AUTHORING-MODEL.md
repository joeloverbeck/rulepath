# Rulepath Authoring Model

Status: authoring law for game modules, docs, static data, and future language pressure.

The first real authors may be humans assisted by Claude Code, Codex, or similar agents. Therefore the authoring model MUST be typed, explicit, testable, and hostile to accidental mini-languages.

## 1. V1 behavior model

In v1, game behavior MUST be written in typed Rust game modules.

Allowed behavior locations:

- `games/<game>/src/setup.rs`;
- `games/<game>/src/actions.rs`;
- `games/<game>/src/rules.rs`;
- `games/<game>/src/visibility.rs`;
- `games/<game>/src/effects.rs`;
- `games/<game>/src/bots.rs`;
- narrow helpers in `game-stdlib` after extraction is justified.

Forbidden behavior locations:

- TOML;
- JSON;
- RON;
- CSV;
- YAML;
- UI metadata;
- localization strings;
- explanation templates;
- TypeScript renderer code;
- `engine-core`.

## 2. Game module authoring contract

Every game module SHOULD define a typed model for:

- game state;
- seat/player mapping;
- phases;
- action payloads or action-path decoding;
- validated commands;
- effect payloads;
- visibility projection;
- terminal outcome and scoring;
- variant enums;
- bot policy hooks;
- UI metadata hooks.

Rules SHOULD read like rules. Avoid generic factories that make simple rules mysterious.

Verbose local game code is acceptable. A contaminated kernel is not.

## 3. Required docs inside every game

Every game SHOULD contain:

```text
games/<game>/docs/
  RULES.md
  SOURCES.md
  RULE-COVERAGE.md
  AI.md
  UI.md
  BENCHMARKS.md
```

### `RULES.md`

MUST include:

- game purpose and chosen variant;
- components/state vocabulary;
- setup;
- turn sequence;
- legal actions;
- forced actions;
- scoring;
- terminal conditions;
- visibility rules;
- bot-relevant strategic notes;
- known ambiguities and chosen resolutions.

Rules text MUST be written in original language. Do not copy rulebook prose.

### `SOURCES.md`

MUST include:

- sources consulted;
- date consulted;
- variant notes;
- deviations from common variants;
- proof notes that public rules prose and assets are original;
- asset authorship/licensing notes.

### `RULE-COVERAGE.md`

MUST map rule sections to implementation modules, unit tests, rule tests, golden traces, property tests, and known gaps.

Suggested table:

| Rule section | Summary | Implementation | Unit tests | Rule tests | Golden traces | Notes |
|---|---|---|---|---|---|---|

Every omitted rule MUST be marked as not applicable, intentionally deferred, unsupported, or open question.

### `AI.md`

MUST document every non-random bot:

- strategy level;
- information access;
- decision order;
- tie-break rules;
- style profiles if any;
- known weaknesses;
- latency benchmark notes;
- explanation examples;
- no-leak tests for hidden-information games.

### `UI.md`

MUST document:

- renderer assumptions;
- legal action mapping;
- progressive construction if any;
- effect-to-animation mapping;
- accessibility labels;
- reduced-motion behavior;
- debug-mode payloads;
- hidden-information safeguards.

### `BENCHMARKS.md`

MUST document:

- hardware;
- Rust version;
- build profile;
- engine version;
- game rules version;
- benchmark command;
- baseline numbers;
- regression threshold;
- known bottlenecks.

## 4. Static data authoring

Static data is allowed for content and parameters.

Use:

- TOML for manifests and simple configuration;
- JSON for traces, replay interchange, browser fixtures, reports;
- RON for Rust-shaped fixtures and enum-heavy static content;
- CSV for tables;
- Postcard or another compact binary Serde format only for non-hand-authored internal snapshots/caches.

Do not use YAML by default.

Static files MUST be schema-validated or deserialized into strongly typed Rust structures.

Static files MUST NOT contain behavior.

See `DATA-RUST-BOUNDARY.md` for binding law.

## 5. Source-note authoring

Public classic implementations require careful source notes.

A source note SHOULD include:

```text
Source: <name and URL>
Consulted: YYYY-MM-DD
Used for: rule verification only
Copied prose: none
Variant choice: <chosen variant>
Deviations: <project-specific changes>
Assets: original/project-owned/licensed separately
```

Source notes are not a permission slip to copy prose, art, icons, card text, or trade dress.

## 6. Rule coverage authoring

Rule coverage is part of authoring, not QA polish after the fact.

A new rule SHOULD be added in this order:

1. Write or update the rule summary.
2. Add source/variant notes if needed.
3. Add rule coverage row.
4. Add focused unit/rule tests.
5. Add or update golden traces.
6. Add invariants/simulation coverage.
7. Implement typed Rust behavior.
8. Update UI metadata/previews/effects.
9. Update bot policy if relevant.
10. Benchmark if the rule affects hot paths.

## 7. Reusable primitive extraction

A reusable primitive SHOULD enter `game-stdlib` only when repeated pressure exists.

Extraction checklist:

- two implemented games have the same shape, or an ADR exists;
- the abstraction has a small name and clear limits;
- no game-specific noun enters the helper;
- both original games are back-ported;
- existing traces still pass or intentional changes are documented;
- benchmarks do not regress unexpectedly;
- docs explain examples and anti-examples.

Examples of likely earned primitives:

- after `three_marks`, `column_four`, and `directional_flip`: coordinates and line/direction scanning helpers;
- after card smoke and trick-taking: typed zone/deck helpers;
- after resources and betting: counters/payment/accounting helpers;
- after simultaneous choice and bluffing: commitments/reveal helpers;
- after reaction-window pressure in multiple games: pending-response helpers.

## 8. Future DSL policy

No DSL at project start.

A DSL MAY be proposed only when multiple Rust game modules show repeated, painful, stable behavior shapes that cannot be maintained cleanly in typed Rust plus `game-stdlib`.

A DSL proposal MUST include:

- problem cases from implemented games;
- rejected Rust/helper alternatives;
- grammar or typed schema;
- static typing model;
- deterministic lowering/compilation;
- source span diagnostics;
- formatter plan;
- linter plan;
- versioning and migration plan;
- test harness;
- benchmark harness;
- replay/hash implications;
- examples and anti-examples;
- hidden-default prevention;
- agent safety plan;
- IP/public-private data plan.

A DSL MUST NOT be introduced to make one monster game possible.

## 9. Game-specific content and public naming

Public games SHOULD use neutral names when a classic game's commercial name creates trademark or trade-dress risk.

Recommended pattern:

| Mechanic | Safer public ID |
|---|---|
| take-away counter game | `race_to_n` or `nim_lite` |
| Tic-Tac-Toe-like placement | `three_marks` |
| Four-in-a-Row-like gravity alignment | `column_four` |
| Reversi-style flipping | `directional_flip` |
| Checkers/draughts-style movement | `draughts_lite` |
| War-like high-card comparison | `high_card_duel` |
| resource economy microgame | `token_bazaar` |
| poker subset | `poker_lite` |
| hidden-claim bluffing | original name only |

## 10. Private licensed modules

Private licensed experiments MUST live outside the public repository and public builds.

They MAY exist only if:

- public CI does not require them;
- public docs do not leak proprietary names, text, assets, or scenarios;
- public WASM does not bundle them;
- local/private builds load them from private sources;
- the public app cannot fetch or reveal them;
- they do not force game nouns into `engine-core`.

Do not hide private licensed content in a public build behind credentials. If it ships to an unauthorized browser, it has shipped.

## 11. Authoring task template

Every game-authoring task SHOULD specify:

```text
Context:
  Which foundation documents apply?
Target:
  Which game/module is being changed?
Ladder stage:
  Which stage and mechanics are being proven?
Goal:
  What behavior should exist when complete?
Rules:
  Which RULES.md sections and sources apply?
Non-goals:
  What must not be touched?
Forbidden changes:
  Which kernel/data/DSL/IP boundaries are forbidden?
Data:
  Which static files and schemas may be used?
Tests:
  Which unit/rule/golden/property/simulation/visibility/UI tests are required?
Benchmarks:
  Which benchmark must exist or not regress?
Docs:
  Which docs must be updated?
Output:
  Complete files or coherent complete sections, not diffs.
```

## 12. Authoring anti-patterns

MUST NOT:

- make a generic engine feature from one game;
- encode rules in data;
- encode rules in UI;
- add hidden default behavior;
- add YAML to solve a local convenience issue;
- introduce a DSL before Rust pressure exists;
- copy rulebook prose;
- make public docs feel like a private licensed adaptation plan;
- let agents rewrite tests without validating test intent;
- ship a game without rule coverage and traces.

## Source notes

See `SOURCES.md`, especially VASSAL, Ludii, Regular Boardgames, Regular Games, data-format sources, Board Game Arena AI-development guidance, and copyright/IP sources.
