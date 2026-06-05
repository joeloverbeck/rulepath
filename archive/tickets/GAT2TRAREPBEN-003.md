# GAT2TRAREPBEN-003: Migrate traces to Trace Schema v1 + fixture catalog

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — converts `race_to_n` golden traces from legacy key-value to Trace Schema v1 JSON, resolves the orphaned `data/fixtures` trace set, and rewires the replay-test harness to load JSON.
**Deps**: GAT2TRAREPBEN-001, GAT2TRAREPBEN-002

## Problem

`race_to_n` golden traces are legacy key-value `.trace` files (e.g.
`id=…`, `seed=…`, `expected_state_hash=…`). Spec §D1 requires Trace Schema v1
JSON as the durable evidence format. Additionally, `tests/golden_traces/` (5
files, the only set consumed by code) is shadowed by an orphaned parallel set
`data/fixtures/replay-*.trace` (4 files) with no source consumer — a duplication
that must be resolved during migration (reassess M2).

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/tests/golden_traces/` holds 5 legacy traces (`shortest-normal`,
   `terminal`, `bot-action`, `invalid-stale-diagnostic`, `not-applicable`), each
   `include_str!`d by `tests/replay_tests.rs`. `games/race_to_n/data/fixtures/`
   holds `replay-shortest-normal/terminal/bot-action/invalid-stale-diagnostic.trace`
   — verified to have **zero** Rust consumers (orphaned at GAT1RACTON-008, which
   created the dir but wired tests to `golden_traces/`).
2. Spec §D1 defines Trace Schema v1; §WB3 requires migration, a fixture catalog
   (IDs/purposes), migration/update notes, and explicit hidden-info + stochastic
   not-applicable coverage. Canonical schema = `docs/TRACE-SCHEMA-v1.md`
   (GAT2TRAREPBEN-001); canonical `rules_version` = string `race_to_n-rules-v1`
   (reassess M1); orphan disposition added by reassess M2.
3. Cross-artifact boundary under audit: traces bind the serialized hash surfaces
   computed by `replay_support` (GAT2TRAREPBEN-002) to the canonical schema doc.
   The surviving canonical trace location is the one `replay-check` (004),
   `fixture-check` (005), and `trace-viewer` (012) all target.
4. FOUNDATIONS §5 (static data is not behavior): restate that the migrated JSON is
   typed evidence — no behavior-looking keys, no rule execution from trace data.
5. §11 determinism / no-leak: confirm each migrated trace's hashes equal the
   current legacy hashes (no silent hash change — any intentional change carries a
   `migration_update_note`). `race_to_n` is perfect-information → record the
   hidden-info and stochastic-game-event surfaces as `not_applicable` with rationale;
   only public-view hashes are required.
6. Schema/contract extension: the golden-trace fixture format changes from
   key-value to Trace Schema v1 JSON. Consumer = `tests/replay_tests.rs` today
   (plus future 004/005/012). This is a format migration, not additive — it is
   gated by the migration policy and the legacy parser is marked temporary.

## Architecture Check

1. Migrating to JSON **and** consolidating to one canonical trace location (folding
   or removing the orphaned `data/fixtures` set) prevents two competing trace sets
   drifting apart — cleaner than migrating both into parallel JSON copies.
2. No backwards-compatibility shims: the legacy key-value parser survives only as an
   explicitly-marked temporary migration import, removed/quarantined after migration.
3. `engine-core` untouched; all trace material is in `games/race_to_n`.

## Verification Layers

1. Migrated hashes == legacy hashes → deterministic replay-hash check (each JSON
   trace reproduces the same state/effect/action-tree/public-view hash).
2. Coverage preserved or expanded → golden-trace check: normal, terminal,
   bot-action, invalid/stale-diagnostic, and the two not-applicable rationale traces
   are all present.
3. Orphan resolved → codebase grep-proof: no unconsumed `.trace` file remains
   (or its retention is documented as a marked migration fixture).
4. No behavior keys → schema/fixture validation: forbidden-key scan over the JSON.

## What to Change

### 1. Convert golden traces to Trace Schema v1 JSON

Author `tests/golden_traces/*.trace.json` for each of the five traces, each
carrying `schema_version`, `trace_id`, `fixture_kind`, `purpose`, `note`,
`migration_update_note`, `game_id`, `rules_version` (`race_to_n-rules-v1`),
`engine_version`, `data_version`, `seed`, `variant`, `options`, `seats`,
`commands`, `checkpoints`, expected hash surfaces, `expected_outcome`,
`expected_terminal_state`, and the `not_applicable` hidden-info / stochastic
rationale.

### 2. Resolve the orphaned `data/fixtures` trace set

Consolidate the 4 `data/fixtures/replay-*.trace` files into the canonical
`golden_traces` location (as JSON) or remove them, and document the decision so a
single trace set is authoritative.

### 3. Rewire `tests/replay_tests.rs`

Load the JSON traces through `replay_support` (GAT2TRAREPBEN-002). Keep the legacy
key-value parser only as a marked-temporary migration path.

## Files to Touch

- `games/race_to_n/tests/golden_traces/*.trace.json` (new) — migrated Trace Schema v1 traces
- `games/race_to_n/tests/golden_traces/*.trace` (modify) — remove/quarantine legacy key-value files
- `games/race_to_n/data/fixtures/` (modify) — resolve the orphaned `replay-*.trace` set
- `games/race_to_n/tests/replay_tests.rs` (modify) — load JSON via `replay_support`

## Out of Scope

- `replay-check` tool (GAT2TRAREPBEN-004) and `fixture-check` tool (GAT2TRAREPBEN-005).
- RULE-COVERAGE.md evidence-row updates (GAT2TRAREPBEN-014 owns the doc rows).
- Adding new game behavior or new variants.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n` — golden-trace + replay tests pass against the JSON traces.
2. A deliberately corrupted expected hash in one `.trace.json` makes the trace test fail loudly (verified manually, then reverted).
3. `ls games/race_to_n/tests/golden_traces/*.trace.json` — the migrated Trace Schema v1 set exists; no orphaned unconsumed trace remains.

### Invariants

1. Migrated traces reproduce identical hashes to the legacy ones unless a `migration_update_note` explains the change (TESTING §3; reassess M1).
2. Exactly one canonical trace set is consumed by code (orphan resolved; reassess M2).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/golden_traces/*.trace.json` — five migrated Trace Schema v1 traces with notes + not-applicable rationales.
2. `games/race_to_n/tests/replay_tests.rs` — loads JSON traces through `replay_support`; legacy parser marked temporary.

### Commands

1. `cargo test -p race_to_n`
2. `cargo test --workspace`
3. `ls games/race_to_n/tests/golden_traces` — confirm the JSON set is present and the legacy/orphan duplication is resolved.
