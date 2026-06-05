# GAT2TRAREPBEN-012: Implement `tools/trace-viewer`

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — replaces the `tools/trace-viewer` no-op placeholder with a CLI summary of Trace Schema v1 traces, optionally reusing `replay_support` for annotation.
**Deps**: GAT2TRAREPBEN-002, GAT2TRAREPBEN-003

## Problem

`tools/trace-viewer` is a 16-line no-op. Gate 2 requires a minimal CLI that prints a
human-readable summary of a Trace Schema v1 trace — metadata, fixture kind/purpose,
migration note, command stream, checkpoints, expected hashes, diagnostics,
not-applicable rationales, and expected outcome — useful for CI failure triage. The
polished browser replay viewer remains Gate 3 (spec §D9).

## Assumption Reassessment (2026-06-05)

1. `tools/trace-viewer/src/main.rs` is a 16-line no-op; the crate is in the workspace
   `Cargo.toml` `members` list. The migrated Trace Schema v1 JSON traces it summarizes
   land in GAT2TRAREPBEN-003; the optional replay-annotation path reuses
   `games/race_to_n` `replay_support` (GAT2TRAREPBEN-002).
2. Spec §D9 fixes the CLI (`--game race_to_n --trace <path>`) and the required output
   set; the canonical trace schema is `docs/TRACE-SCHEMA-v1.md` (GAT2TRAREPBEN-001).
   Non-goal: polished replay UI (Gate 3).
3. Cross-crate boundary under audit: `trace-viewer` consumes the Trace Schema v1
   contract; if it annotates actual hashes it must reuse `replay_support` rather than
   re-deriving them (shared replay authority).
4. FOUNDATIONS §11 (viewer-safe output, no-leak): restate that the summary is
   viewer-safe — `race_to_n` is perfect-information, so the trace and any annotation
   leak no hidden information; `trace-viewer` is never a rule-behavior authority.

## Architecture Check

1. A read-only summary that optionally reuses `replay_support` for annotation (never a
   second replay implementation) keeps the replay authority single and avoids
   incompatible logic.
2. No backwards-compatibility shims; the no-op is replaced.
3. `engine-core` untouched; `trace-viewer` reasons over trace JSON + the game's public
   replay API, adding no kernel noun.

## Verification Layers

1. Complete summary output → CLI run: a valid trace prints metadata, commands,
   checkpoints, expected hashes, diagnostics, not-applicable rationales, and outcome.
2. Malformed-trace handling → schema validation: a malformed trace exits non-zero with
   a useful error.
3. Annotation reuse (if implemented) → deterministic replay-hash check: annotated
   actual hashes come from `replay_support`, matching `replay-check`.

## What to Change

### 1. CLI parsing (`tools/trace-viewer/src/main.rs`)

Implement `--game race_to_n --trace <path>`.

### 2. Summary renderer

Parse a Trace Schema v1 trace and print its metadata, fixture kind/purpose,
migration/update note, command stream, checkpoints, expected hashes, diagnostics,
not-applicable rationales, and expected outcome/terminal state. Exit non-zero on a
malformed trace. Optionally annotate actual state/effect/view summaries only by
reusing `replay_support` safely.

### 3. `tools/trace-viewer/Cargo.toml`

Add the JSON-parse dependency (and `games/race_to_n` if annotation is implemented).

## Files to Touch

- `tools/trace-viewer/src/main.rs` (modify) — replace no-op with the summary CLI
- `tools/trace-viewer/Cargo.toml` (modify) — add parse (and optional game) deps

## Out of Scope

- Browser replay viewer / polished UI / replay controls (Gate 3; §D9 non-goal).
- Editing traces.
- Duplicating replay logic instead of reusing `replay_support`.
- CI wiring (GAT2TRAREPBEN-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p trace-viewer -- --game race_to_n --trace games/race_to_n/tests/golden_traces/shortest-normal.trace.json` — prints a complete summary.
2. A malformed trace exits non-zero with a useful error.
3. `cargo test -p trace-viewer` — output completeness asserted against a fixture trace.

### Invariants

1. Output is viewer-safe and `trace-viewer` is never a rule-behavior authority (§11; §D9).
2. Any actual-hash annotation reuses `replay_support`, not a re-implementation (single replay authority).

## Test Plan

### New/Modified Tests

1. `tools/trace-viewer/src/main.rs` (or `tools/trace-viewer/tests/`) — assert the summary contains each required section for a valid trace; assert non-zero exit on a malformed trace.

### Commands

1. `cargo run -p trace-viewer -- --game race_to_n --trace games/race_to_n/tests/golden_traces/shortest-normal.trace.json`
2. `cargo test -p trace-viewer`
3. A single-trace render is the correct verification boundary; this tool is a triage aid, not a CI gate.
