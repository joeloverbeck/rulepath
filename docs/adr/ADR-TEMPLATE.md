# ADR: <title>

Status: Proposed | Accepted | Superseded | Rejected

Date: YYYY-MM-DD

Decision owner: <name or role>

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/INVARIANTS.md`
- `docs/ARCHITECTURE.md`
- `docs/DATA-RUST-BOUNDARY.md`
- `docs/MECHANIC-ATLAS.md`
- <other docs>

## Context

What pressure exists?

- Which implemented games are affected?
- Which tests, traces, simulations, benchmarks, or UI constraints exposed the issue?
- What current rule or architecture is insufficient?
- Why is an ADR required instead of a local game-module change?

## Decision

State the decision clearly.

Use MUST / SHOULD / MAY intentionally.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| <alternative> | <reason> | <reason> |

## Consequences

Positive consequences:

- <consequence>

Negative or risky consequences:

- <consequence>

Operational requirements:

- <tests, docs, migrations, benchmarks>

## Determinism impact

- Does the decision affect RNG, order of iteration, clocks, floating point, parallelism, serialization order, or replay?
- What tests prove determinism?

## Replay/hash impact

- Does this change command streams, state hashes, effect hashes, public view hashes, or trace format?
- Are existing golden traces preserved, migrated, or intentionally updated?
- What migration note is required?

## Visibility impact

- Does this expose, filter, store, log, serialize, preview, or animate private information?
- How are public/private views and hidden-information tests affected?

## Data/Rust boundary impact

- Does this introduce a new static data field, hand-authored format, expression, selector, behavior ID, variant, or schema?
- Does behavior remain in typed Rust?
- Are unknown fields rejected?
- Are behavior-looking fields blocked?

## `engine-core` contamination risk

- Does the change add game nouns, mechanic nouns, strategy, renderer concerns, networking, storage, or private content to `engine-core`?
- Why can this not live in `games/*`?
- Why can this not live in `game-stdlib` after earned pressure?

## UI impact

- Does this change action trees, previews, effect-log animation, renderer boundaries, accessibility, reduced motion, or dev inspector behavior?
- Does TypeScript remain presentation-only?

## Bot impact

- Does this change bot views, legal action APIs, candidate ranking, explanation output, benchmarks, or no-leak tests?
- Does any Level 2 bot require a strategy evidence pack update?

## IP impact

- Does this affect public naming, rules prose, assets, fonts, fixtures, traces, private content, or browser-shipped bundles?
- Is human/legal review required?

## Benchmark impact

- Which native benchmarks must be added or updated?
- Which public latency budgets are affected?
- Is a WASM/browser smoke benchmark required?

## Migration notes

- Existing docs to update:
- Existing games to back-port:
- Existing traces to preserve or update:
- Existing data/schema versions to bump:
- Existing public UI behavior to migrate:

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research;
- Rust remains behavior authority;
- TypeScript does not decide legality;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters, not behavior;
- replay determinism is preserved or migration is explicit;
- visibility boundaries remain safe;
- bots remain fair and explainable;
- benchmarks exist for hot paths;
- IP/public-private boundaries are preserved;
- templates and affected docs are updated.
