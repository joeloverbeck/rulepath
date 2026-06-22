# ADR: <title>

Status: Proposed

Date: YYYY-MM-DD

Decision owner: <name or role>

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/AI-BOTS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`

Required scaling / supersession fields:

- Affected foundation sections: <sections or none>
- Superseded decision, if any: <ADR/section or none>
- Hidden-information no-leak compatibility: <public/private/viewer impact>
- No-DSL/no-YAML compatibility: <data-language impact>
- Evidence classification: <doctrine/code/trace/fixture/benchmark/UI/IP/none>
- Compatibility window: <none, permanent, or start/end conditions>
- Accepted exceptions: <exception IDs or none>
- Effective only after named foundation updates land: <yes/no + named docs>
- Proposed-ADR review trigger / expiry: <event/date before re-review or none>
- Rollback / contamination risk: <how to avoid kernel, data, private-content, or UI contamination>

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| <surface or none> | <change or none> | <owner> | <window> | <proof> |

## Context

What pressure exists?

- Which implemented official games are affected?
- Which tests, traces, simulations, benchmarks, or UI constraints exposed the issue?
- What current rule or architecture is insufficient?
- Why is an ADR required instead of a local game-module change?
- Does this support public playable Rulepath before research/private stress tests?

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

- Does the decision affect RNG, iteration order, clocks, floating point, parallelism, serialization order, replay, or hashes?
- What tests prove determinism?

## Replay/hash impact

- Does this change command streams, state hashes, effect hashes, action-tree hashes, public/private view hashes, trace format, or migration rules?
- Are existing golden traces preserved, migrated, or intentionally updated?
- What migration note is required?

## Visibility impact

- Does this expose, filter, store, log, serialize, preview, animate, explain, or rank private information?
- How are public/private views and hidden-information tests affected?
- Could browser payloads, DOM, local storage, logs, bot explanations, candidate rankings, or replay exports leak hidden information?

## Data/Rust boundary impact

- Does this introduce a new static data field, hand-authored format, expression, selector, behavior ID, variant, or schema?
- Does behavior remain in typed Rust?
- Are unknown fields rejected?
- Are behavior-looking fields blocked?
- Does the decision create DSL pressure?

## `engine-core` contamination risk

- Does the change add game nouns, mechanic nouns, strategy, renderer concerns, networking, storage, or private content to `engine-core`?
- Why can this not live in `games/*`?
- Why can this not live in `game-stdlib` after earned pressure?
- What already implemented official games require it?

## `game-stdlib` / primitive-pressure impact

- Which games exert pressure?
- Is this first, second, or third use?
- Is there a primitive-pressure ledger entry?
- Are examples, anti-examples, tests, benchmarks, and back-ports required?

## UI impact

- Does this change action trees, previews, effect-log animation, renderer boundaries, accessibility, reduced motion, responsive behavior, or dev inspector behavior?
- Does TypeScript remain presentation-only?

## Bot impact

- Does this change bot views, legal action APIs, candidate ranking, explanation output, hidden-information safety, benchmarks, or public bot levels?
- Does any Level 2 bot require a strategy evidence pack update?
- Does it introduce any v1/v2-excluded AI technique?

## IP impact

- Does this affect public naming, rules prose, assets, fonts, fixtures, traces, private content, or browser-shipped bundles?
- Is human/legal review required?

## Benchmark impact

- Which native benchmarks must be added or updated?
- Which public latency budgets are affected?
- Is a WASM/browser smoke benchmark required?

## Migration notes

Existing docs to update:

- <docs>

Existing games to back-port:

- <games>

Existing traces to preserve or update:

- <traces>

Existing data/schema versions to bump:

- <versions>

Existing public UI behavior to migrate:

- <behavior>

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
- affected foundation docs and per-game docs are updated.
