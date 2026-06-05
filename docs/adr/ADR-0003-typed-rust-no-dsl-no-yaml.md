# ADR-0003: Typed Rust behavior, no startup DSL, no default YAML

## Status

Accepted.

## Context

Rulepath is likely to be implemented partly by coding agents. Untyped behavior in static files is hard to review, hard to test, hard to benchmark, and easy to mutate into an accidental language. General game systems and formal game-description research show that true language-based generality requires serious tooling, not ad hoc nested objects.

## Decision

In v1 and v2, rule behavior MUST be implemented in typed Rust game modules.

Rule behavior includes:

- setup logic;
- legal action generation;
- validation;
- action application;
- triggers and reactions;
- mandatory action rules;
- conditional effects;
- terminal detection;
- scoring;
- visibility projection;
- tactical bot policy.

Rulepath starts with no custom DSL.

A future DSL MAY be proposed only after multiple implemented Rust game modules reveal repeated, painful, stable behavior shapes that typed Rust plus `game-stdlib` cannot maintain cleanly.

A future DSL MUST be typed, compiled/lowered, source-span-aware, deterministic, formatted, linted, versioned, tested, benchmarked, documented with examples and anti-examples, and unable to silently depend on hidden defaults.

YAML MUST NOT be used by default. Any YAML use requires ADR.

## Consequences

Positive:

- rule behavior remains visible to Rust types, tests, benchmarks, and code review;
- coding-agent output is easier to constrain;
- hidden procedural logic is harder to smuggle into content files;
- future DSL pressure can be measured from real games.

Negative:

- non-programmer authoring is not a v1 goal;
- some game-specific Rust may be verbose;
- data files cannot define arbitrary card/effect behavior.

Migration consequences:

- DSL experiments must start as separate research artifacts, not public foundation dependencies;
- existing static files that acquire selectors, loops, branches, triggers, or exception logic must be migrated back into typed Rust behavior.

## Alternatives considered

### YAML behavior files

Rejected. They encourage accidental mini-languages; current Rust YAML ecosystem maintenance also makes YAML a poor default.

### JSON/RON/TOML behavior files

Rejected for the same behavioral reason. Better syntax does not solve the untyped-language problem.

### Launch with a formal DSL

Rejected. The project has not yet implemented enough games to know the stable shapes worth language support.
