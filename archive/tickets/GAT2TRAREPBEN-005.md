# GAT2TRAREPBEN-005: Implement `tools/fixture-check`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — replaces the `tools/fixture-check` no-op placeholder with a real fail-closed fixture/static-data validator for Trace Schema v1 traces, `manifest.toml`, and `variants.toml`.
**Deps**: GAT2TRAREPBEN-001, GAT2TRAREPBEN-003

## Problem

`tools/fixture-check` is a 16-line no-op. Gate 2 needs a real fail-closed validator
that enforces Trace Schema v1 strictness, rejects behavior-looking and unknown
fields recursively, catches duplicate IDs and missing notes, and checks
manifest/variants strictness — so static data and traces cannot smuggle behavior or
drift silently (spec §D4; FOUNDATIONS §5/§11).

## Assumption Reassessment (2026-06-05)

1. `tools/fixture-check/src/main.rs` is a 16-line no-op; the crate is in the
   workspace `Cargo.toml` `members` list. `games/race_to_n/data/manifest.toml`
   (`game_id`, `rules_version = 1`, `data_version`, `schema_version`,
   `counter_name`, `seat_name`) and `data/variants.toml` (`variant_id = "race_to_21"`,
   `target`, `max_add`, `seat_count`, …) exist as typed TOML. Migrated Trace Schema
   v1 JSON traces land in GAT2TRAREPBEN-003.
2. Spec §D4 lists the required checks; canonical schema = `docs/TRACE-SCHEMA-v1.md`
   (GAT2TRAREPBEN-001). The `/reassess-spec` session pinned the `rules_version`
   consistency anchor to the canonical string `race_to_n-rules-v1` (M1) — note the
   manifest currently records the integer `1`, which this checker must flag (or
   accept via the documented int↔string mapping) per the M1 disposition.
3. Cross-artifact boundary under audit: `fixture-check` is the enforcement surface
   for the Trace Schema v1 contract (001) and the static-data discipline of
   `docs/ENGINE-GAME-DATA-BOUNDARY.md`. It validates files; it never executes replay.
4. FOUNDATIONS §5 (static data is not behavior) and §11 (unknown fields rejected;
   behavior-looking fields blocked): restate that the validator must be deterministic,
   blocking, and fail-closed — reject unknown fields by default and refuse
   behavior-looking keys recursively (`when`/`if`/`then`/`selector`/`trigger`/…).
5. §11 fail-closed enforcement: this is the validator the schema doc (001) was the
   substrate for. Confirm it distinguishes blockers from warnings, names what failing
   means (non-zero exit), rejects YAML under fixture/report/trace paths, and that its
   diagnostics leak no hidden information (perfect-info game).

## Architecture Check

1. A recursive structural validator (rejecting unknown + behavior-looking keys at
   any depth) is stronger than a flat allow-list — behavior can hide in nested
   objects, which §5 forbids.
2. No backwards-compatibility shims; legacy `.trace` files are accepted only if
   explicitly marked as retained migration fixtures with verified migration status.
3. `engine-core` untouched; `fixture-check` reasons over data files, adding no
   kernel noun.

## Verification Layers

1. Unknown/behavior-looking-field rejection → schema/serialization validation: a
   fixture with an unknown field and one with a behavior-looking key each fail.
2. Duplicate-ID / empty-note / missing-migration-note rejection → schema validation:
   negative fixtures fail with path+field context.
3. Manifest/variants strictness + no-YAML → static-data validation: a YAML file
   under a fixture/trace/report path fails; `rules_version` consistency is checked
   against the canonical anchor.
4. CLI fail-closed behavior → CLI run: valid fixtures pass, malformed exit non-zero.

## What to Change

### 1. CLI parsing (`tools/fixture-check/src/main.rs`)

Implement `--game race_to_n` and `--game race_to_n --trace <path>`.

### 2. Validation engine

Enforce Trace Schema v1 required fields + unknown-field rejection; recursively
reject behavior-looking keys; reject duplicate trace IDs, empty `note`, and missing
`migration_update_note` where required; check game/rules/data/engine version fields
present and consistent (rules_version anchored on the canonical `race_to_n-rules-v1`
string, per reassess M1); check `manifest.toml` / `variants.toml` strictness where
practical; reject YAML under fixture/report/trace paths. Report every failure with
path + field context; exit non-zero on any failure.

### 3. `tools/fixture-check/Cargo.toml`

Add the dependencies needed to parse JSON traces + TOML manifest/variants.

## Files to Touch

- `tools/fixture-check/src/main.rs` (modify) — replace no-op with real validator
- `tools/fixture-check/Cargo.toml` (modify) — add JSON/TOML parse deps

## Out of Scope

- Executing replay behavior (GAT2TRAREPBEN-004 owns replay).
- Migrating the traces (GAT2TRAREPBEN-003).
- Replacing Rust static-data parser tests (`R-VAR-002` coverage stays).
- CI wiring (GAT2TRAREPBEN-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p fixture-check -- --game race_to_n` — passes on the valid migrated fixtures/manifest/variants.
2. Fixtures with (a) an unknown field, (b) a behavior-looking key, (c) a duplicate ID, (d) an empty note, (e) a missing migration note each exit non-zero with path+field context.
3. `cargo test -p fixture-check` — negative-fixture tests prove each rejection.

### Invariants

1. Validation is deterministic, blocking, and fail-closed; unknown + behavior-looking fields are rejected by default (§5/§11).
2. No YAML appears under fixture/trace/report paths (§5/§11).

## Test Plan

### New/Modified Tests

1. `tools/fixture-check/src/main.rs` (or `tools/fixture-check/tests/`) — negative fixtures for unknown field, behavior key, duplicate ID, empty note, missing migration note, YAML rejection.
2. Reuse migrated traces + `manifest.toml`/`variants.toml` as the valid-pass fixtures.

### Commands

1. `cargo run -p fixture-check -- --game race_to_n`
2. `cargo test -p fixture-check`
3. `cargo run -p fixture-check -- --game race_to_n --trace games/race_to_n/tests/golden_traces/shortest-normal.trace.json` — single-trace strictness smoke.
