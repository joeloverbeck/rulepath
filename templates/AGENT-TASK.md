# Agent Task

## Context

Foundation documents to follow (see `docs/README.md` for the full index):

- `docs/FOUNDATIONS.md`
- `docs/INVARIANTS.md`
- `docs/ARCHITECTURE.md`
- `docs/DATA-RUST-BOUNDARY.md`
- `docs/AUTHORING-MODEL.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/ROADMAP.md`
- `docs/UI-INTERACTION.md`
- `docs/AI-BOTS.md`
- `docs/TESTING-AND-BENCHMARKING.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`

Other required docs/source notes:

- <paths>

## Target

Exact target crate, module, game, doc, or template:

- <target>

## Stage

Ladder gate/stage:

- <stage>

## Mechanics

Mechanics being proved or modified:

- <mechanic>

Mechanic atlas / primitive-pressure status:

- <local-only / repeated-shape candidate / extraction required / promoted / deferred / ADR-required>

## Goal

When complete, the following must be true:

- <observable result>

## Non-goals

Do not do these things:

- <non-goal>

## Forbidden changes

- Do not add game nouns to `engine-core`.
- Do not move behavior into static data.
- Do not add YAML without ADR.
- Do not create a DSL.
- Do not let TypeScript decide legality.
- Do not let bots access hidden information unavailable to their seat.
- Do not add private licensed content to public files.
- Do not update golden traces without explaining intentional behavior/format change.
- Do not optimize without benchmark evidence.
- <task-specific forbidden changes>

## Sources and docs

Rules/source docs to consult or update:

- <path/source>

Per-game docs to update:

- `RULES.md`
- `SOURCES.md`
- `RULE-COVERAGE.md`
- `MECHANICS.md`
- `AI.md`
- `UI.md`
- `BENCHMARKS.md`

## Tests

Required tests:

- unit tests:
- rule tests:
- golden traces:
- property/invariant tests:
- simulation/fuzz tests:
- replay tests:
- serialization tests:
- visibility/no-leak tests:
- AI legal-action tests:
- UI smoke tests:

Failing-test protocol:

1. determine whether failing tests are still valid;
2. determine whether issue is in SUT or test suite;
3. fix issue;
4. add/update regression coverage;
5. report changes.

## Benchmarks

Required benchmark evidence:

- native benchmark(s):
- WASM/browser smoke benchmark(s), if needed:
- expected budget/threshold:

## Documentation

Documentation updates required:

- <docs>

## Output format

Provide complete files or coherent complete sections, not diffs.

Report:

- files changed;
- tests added/updated;
- traces added/updated;
- simulations run;
- benchmarks run;
- docs updated;
- boundary decisions;
- unresolved questions;
- commands for human verification.

## Review checklist

- Rust owns behavior.
- TypeScript does not decide legality.
- Static data remains content/parameters/metadata only.
- `engine-core` remains noun-free.
- `game-stdlib` extraction is earned or deferred by ledger.
- Replay and hashes remain deterministic.
- Hidden information is safe.
- Bots use allowed views and legal action APIs.
- Tests, traces, simulations, and benchmarks cover the work.
- Public files are IP-safe.
- Output is bounded and reviewable.
