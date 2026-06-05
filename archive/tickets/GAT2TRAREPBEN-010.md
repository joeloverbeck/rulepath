# GAT2TRAREPBEN-010: Implement `tools/seed-reducer` v0

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — replaces the `tools/seed-reducer` no-op placeholder with an honest v0 reproducer/normalizer; documents the future reducer plan in TESTING doctrine.
**Deps**: GAT2TRAREPBEN-009, GAT2TRAREPBEN-003, GAT2TRAREPBEN-004

## Problem

`tools/seed-reducer` is a 16-line no-op. Gate 2 requires an honest v0 that turns a
failing seed/command stream into a normalized replay command or Trace Schema v1
reproducer, optionally does bounded prefix minimization when a failure predicate is
available, and explicitly says when true minimization is unavailable — no fake
delta-debugging, no fuzzing framework (spec §D7).

## Assumption Reassessment (2026-06-05)

1. `tools/seed-reducer/src/main.rs` is a 16-line no-op; the crate is in the workspace
   `Cargo.toml` `members` list. Its input — the machine-readable simulation failure
   report — is produced by GAT2TRAREPBEN-009; the Trace Schema v1 reproducer format is
   fixed by GAT2TRAREPBEN-001/003; the normalized replay command targets `replay-check`
   (GAT2TRAREPBEN-004).
2. Spec §D7 / §WB8 fix the CLI (`--game race_to_n --seed <n> --commands <stream>` and
   `--game race_to_n --failure-report <path>`) and the honesty requirements. The
   `simulate` failure-report dependency was added by reassessment M3.
3. Cross-crate boundary under audit: `seed-reducer` consumes the `simulate`
   failure-report schema (009) and emits either a `replay-check` command (004's input)
   or a Trace Schema v1 reproducer (003's format). All three contracts must line up.
4. FOUNDATIONS §11 (honest, deterministic tooling): restate that the tool must
   preserve the exact reproducer and state plainly when minimization is unavailable —
   claiming minimization when only normalization happened is forbidden (§D7;
   §Forbidden changes). For a fixed input it is deterministic; perfect-information game
   → the reproducer leaks nothing hidden.

## Architecture Check

1. A normalize-first, minimize-only-with-a-predicate design is the honest v0 the spec
   asks for — it avoids pretending to delta-debug without a real failure predicate.
2. No backwards-compatibility shims; the no-op placeholder is replaced, not aliased.
3. `engine-core` untouched; `seed-reducer` reaches game behavior through the existing
   tool/game contracts, adding no kernel noun.

## Verification Layers

1. Reproducer fidelity → deterministic replay-hash check: the emitted reproducer,
   fed to `replay-check`, reproduces the original failure.
2. Honest minimization status → manual review + CLI run: when no predicate exists the
   tool says so and preserves the exact reproducer; it never claims minimization.
3. Cross-tool contract → schema validation: the emitted Trace Schema v1 reproducer
   passes `fixture-check`, and the emitted replay command is accepted by `replay-check`.

## What to Change

### 1. CLI + ingestion (`tools/seed-reducer/src/main.rs`)

Implement both input modes (explicit seed/command stream; `--failure-report <path>`
consuming the GAT2TRAREPBEN-009 report). Emit a normalized `replay-check` command
and/or a Trace Schema v1 reproducer; replay the failure when enough context exists;
attempt bounded prefix minimization only when a clear failure predicate is available;
otherwise state that minimization is unavailable and preserve the exact reproducer.

### 2. `tools/seed-reducer/Cargo.toml`

Add the dependencies needed to parse the failure report and emit the reproducer.

### 3. `docs/TESTING-REPLAY-BENCHMARKING.md`

Document the seed-reduction v0 scope and the future reducer plan (spec §WB8 required
work).

## Files to Touch

- `tools/seed-reducer/src/main.rs` (modify) — replace no-op with v0 reproducer/normalizer
- `tools/seed-reducer/Cargo.toml` (modify) — add report-parse / reproducer-emit deps
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify) — record seed-reduction v0 + future plan

## Out of Scope

- A fuzzing framework, randomized shrinking, or real delta-debugging (§D7 non-goals).
- The `simulate` failure-report emission (GAT2TRAREPBEN-009).
- CI wiring (GAT2TRAREPBEN-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p seed-reducer -- --game race_to_n --failure-report <path>` — emits a normalized replay command and/or Trace Schema v1 reproducer.
2. The emitted reproducer, passed to `cargo run -p replay-check -- --game race_to_n --trace <reproducer>` (or as a replay command), reproduces the original failure.
3. When no failure predicate is available, the tool explicitly reports that minimization is unavailable and preserves the exact reproducer.

### Invariants

1. The tool never claims minimization when only normalization occurred (§D7; §11 honesty).
2. The reproducer round-trips through `fixture-check` / `replay-check` (cross-tool contract holds).

## Test Plan

### New/Modified Tests

1. `tools/seed-reducer/src/main.rs` (or `tools/seed-reducer/tests/`) — inject a `simulate` failure, reduce it, and assert the reproducer replays the failure via `replay-check`.

### Commands

1. `cargo run -p seed-reducer -- --game race_to_n --failure-report /tmp/fail.json`
2. `cargo test -p seed-reducer`
3. `cargo run -p seed-reducer -- --game race_to_n --seed <n> --commands <stream>` — explicit-stream mode smoke.
