# AUTHORING MODEL

Status: authoring law for game implementation and static data.

The first real game authors may be Claude Code, Codex, or similar agents. Therefore the authoring model MUST be typed, testable, explicit, and hostile to accidental mini-languages.

## 1. v1 rule behavior

In v1, rule behavior MUST be written in typed Rust game modules.

Allowed:

- Rust functions for setup, legal actions, validation, state transitions, scoring, visibility, and terminal detection;
- Rust enums and structs for game-specific nouns;
- typed helper functions from `game-stdlib`;
- game-specific bot policies in Rust;
- tests and golden traces as first-class authoring artifacts.

Forbidden in v1:

- no-code authoring;
- custom DSL;
- behavior hidden in YAML, JSON, TOML, RON, or CSV;
- selectors encoded as strings in static data;
- loops or branches in static content files;
- tactical AI conditions in static data;
- exception logic in static data;
- rule semantics that require readers to interpret nested untyped objects.

## 2. Static content data

Static data is content, not behavior.

Appropriate static data:

- game manifest;
- variant definitions;
- display names;
- icon ids;
- color/theme ids;
- board labels;
- coordinate labels;
- initial setup constants;
- card ids and non-proprietary original card text for original/public games;
- deck composition;
- scoring tables;
- localization strings;
- explanation template text;
- UI metadata;
- golden trace fixtures.

Static data MUST be schema-validated or deserialized into strongly typed Rust structures.

Static data MUST NOT control rule flow except through typed, documented, narrow parameters consumed by Rust behavior.

## 3. Recommended formats

### TOML

Use TOML for:

- game manifests;
- simple variants;
- build-time configuration;
- metadata humans will edit by hand.

Reason: TOML has mature Rust/Serde support and is readable for configuration-style data.

### JSON

Use JSON for:

- browser-facing payload fixtures;
- golden traces;
- replay summaries;
- machine-readable reports;
- interoperability with TypeScript tools.

Reason: JSON is the common web interchange format and has mature Serde support.

### RON

Use RON for:

- complex Rust-shaped static content;
- enum-heavy fixtures;
- nested setup data that becomes ugly in JSON/TOML;
- test fixtures written by Rust developers.

Reason: RON maps naturally to Serde data structures while remaining human-readable.

### CSV

Use CSV for:

- card lists;
- scoring tables;
- balance tables;
- tabular rule coverage matrices;
- benchmark result exports.

Reason: tables should be edited as tables.

### Postcard or another compact binary Serde format

MAY be used for:

- internal cache files;
- compact local snapshots;
- benchmark fixtures;
- non-hand-authored binary replay artifacts.

It MUST NOT be used for hand-authored game rules.

### YAML

YAML MUST NOT be used by default in v1.

A YAML proposal REQUIRES ADR and MUST answer:

- why TOML, JSON, RON, or CSV are insufficient;
- which maintained Rust parser is used;
- how schemas and diagnostics work;
- how behavior is prevented from leaking into YAML;
- how round-tripping, formatting, and CI validation work.

The prior failure mode was not merely a syntax preference. YAML became an accidental untyped programming language. The current `serde_yaml` crate is also marked deprecated/unmaintained, which strengthens the default ban.

## 4. Rust game behavior

Each game module SHOULD define a typed model:

```text
State
Player/Seat mapping
Phase
Action or ActionPath payloads
Command validation
Effect emission
Visibility projection
Outcome/scoring
Bot policy hooks
UI metadata hooks
```

Rules SHOULD be readable as rules. Do not hide them behind generic factories merely to look clever.

A game module MAY be verbose. Verbosity in a game module is cheaper than a contaminated kernel.

## 5. Reusable primitives

Reusable primitives SHOULD be extracted only when repeated pressure exists.

Extraction rule:

1. Implement the first game locally.
2. Implement the second game honestly.
3. Identify the repeated shape.
4. Extract a tested helper into `game-stdlib`.
5. Back-port the helper to the earlier games.
6. Document the abstraction and its limits.

A primitive that exists because of one hypothetical future game REQUIRES ADR.

## 6. Optional future DSL

A DSL is not allowed at project start.

A DSL MAY be proposed only after multiple Rust game modules show repeated, painful, stable behavior shapes that are too verbose to maintain directly.

A future DSL MUST be:

- typed;
- validated before runtime;
- source-span-aware;
- deterministic;
- formatted;
- linted;
- versioned;
- testable in isolation;
- benchmarked;
- compiled/lowered into a compact internal model;
- documented with examples and anti-examples;
- unable to silently depend on hidden defaults;
- able to report diagnostics that a human or agent can fix.

A DSL MUST NOT be introduced to make one monster game possible.

## 7. Explanation templates

Explanation templates are presentation metadata, not source-of-truth rules.

Good:

```text
move_piece = "Move {piece} from {from} to {to}."
draw_cards = "Draw {count} card(s) from {deck}."
change_score = "{player} gains {amount} point(s)."
```

Bad:

```text
when phase == "event" and faction == "blue" and selector == "eligible_adjacent" then...
```

Templates MUST be keyed to semantic actions/effects emitted by Rust. The engine MUST NOT attempt to explain arbitrary code magically.

## 8. UI metadata

UI metadata MAY include:

- display labels;
- short help text;
- icon ids;
- layout hints;
- coordinate labels;
- piece shapes;
- theme tokens;
- action grouping tags;
- accessibility labels.

UI metadata MUST NOT include rule legality or hidden behavior.

## 9. Private licensed modules

Private licensed experiments MUST live outside the public repository and public builds.

They MAY use private static data, private modules, and private assets only if:

- public CI does not require them;
- public docs do not leak proprietary text/assets;
- public WASM does not bundle them;
- local/private builds load them from private sources;
- the public app cannot fetch or reveal them.

Do not hide private licensed content in a public build behind credentials. If the data or module ships to unauthorized browsers, it has shipped.

## 10. Agent-safe authoring checklist

Every game-authoring task MUST specify:

- target game and ladder stage;
- mechanics being tested;
- target module(s);
- exact rules source notes;
- non-goals;
- forbidden kernel changes;
- static data format;
- required tests;
- required golden traces;
- required simulation/fuzz coverage;
- required benchmarks;
- required docs;
- output format: complete files or coherent complete sections, not diffs.

## Source notes

See `SOURCES.md`, especially `serde_yaml`, TOML, JSON, RON, CSV, Postcard, Regular Boardgames / Regular Games, and Board Game Arena AI-development guidance.
