# GAT2TRAREPBEN-001: Trace Schema v1 + benchmark/Stage-1 doctrine docs

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `docs/TRACE-SCHEMA-v1.md` canonical schema doctrine; modifies `docs/TESTING-REPLAY-BENCHMARKING.md` (doctrine only, no code).
**Deps**: None

## Problem

Every Gate 2 trace and trace-consuming tool must validate against one canonical
Trace Schema v1 definition. Today the schema decision lives only in the spec
(§D1), and `docs/TESTING-REPLAY-BENCHMARKING.md §3` carries a soft, drifting
golden-trace field list (`action_stream`, `expected_legal_action_hashes`,
`expected_public_view_hashes for selected viewers`) that does not match the
Trace Schema v1 field names. This ticket fixes the schema-of-record and the
benchmark-hard-fail / Stage-1 doctrine so downstream tickets (003 migration,
005 fixture-check, 006 benchmark output) build against a single authority.

## Assumption Reassessment (2026-06-05)

1. No `docs/TRACE-SCHEMA-v1.md` exists yet (verified absent). The six Gate 2 tool
   crates are 16-line no-op placeholders. `docs/TESTING-REPLAY-BENCHMARKING.md §3`
   lists the soft golden-trace field set; §14–§16 carry benchmark doctrine and the
   §15 Stage-1 `500,000+ games/sec` provisional budget ("to be replaced by measured
   baselines").
2. Spec §D1 defines the Trace Schema v1 root + command-stream fields and the
   forbidden behavior-key list; §WB1 records JSON as the decision. The `/reassess-spec`
   session pinned `rules_version` canonical form to the string `race_to_n-rules-v1`
   (M1) and flagged the §3 field-name drift for reconciliation here (M4). The gate
   reconfirmation and the in-spec decision recording already landed during that
   session; this ticket writes the externalized doctrine doc.
3. Cross-artifact boundary under audit: this doc is the schema-of-record that
   `fixture-check` (005), `replay-check` (004), and `trace-viewer` (012) all validate
   against, and that the migrated traces (003) instantiate. It must not create a
   second authority competing with TESTING §3 — TESTING §3 is reconciled to point at
   the canonical names.
4. FOUNDATIONS §5 (static data is typed content, not behavior) motivates the schema:
   restate that traces are typed evidence, never rule behavior. The schema MUST
   enumerate and forbid behavior-looking keys (`when`, `if`, `then`, `selector`,
   `condition`, `trigger`, `script`, `loop`, `foreach`, `rule`, `requires`, `valid_if`,
   `on_play`, `on_reveal`, …) per §5/§12 (no DSL).
5. §11 enforcement substrate: this doc DEFINES the surface that `fixture-check`
   (GAT2TRAREPBEN-005) later enforces fail-closed. Confirm the schema mandates
   unknown-field rejection by default, deterministic hash fields (state/effect/
   action-tree/public-view), and an explicit hidden-information / stochastic
   not-applicable rationale — so the data model introduces no leakage or
   nondeterminism path the deferred validator would have to undo. No validator
   exists yet; enforcement lands in 005.

## Architecture Check

1. A single canonical `docs/TRACE-SCHEMA-v1.md` linked from TESTING (rather than
   leaving the schema scattered across the spec and a soft TESTING §3 list) gives
   one authority and kills the field-name drift (reassess M4). TESTING §3 reduces
   to a pointer plus reconciled names.
2. No backwards-compatibility shims; the legacy key-value `.trace` format is
   documented as a temporary migration input only (owned by 003), not a parallel
   schema authority.
3. `engine-core` is untouched — this is doctrine prose; no mechanic noun enters
   the kernel; `game-stdlib` untouched.

## Verification Layers

1. Schema completeness → manual review: every spec §D1 root + command-stream field
   appears in `TRACE-SCHEMA-v1.md` with required/conditional marking.
2. Behavior-key prohibition present → FOUNDATIONS alignment check (§5/§12): the
   forbidden-key list is enumerated and the "evidence not behavior" rule is stated.
3. Single field-name authority → codebase grep-proof: TESTING §3 no longer uses
   `action_stream` / `expected_legal_action_hashes`; it uses `commands` /
   `expected_action_tree_hashes` or points to the canonical doc.
4. Cross-artifact doc ticket — layers mapped above rather than a single review;
   doc-link integrity proven by `check-doc-links.mjs`.

## What to Change

### 1. New `docs/TRACE-SCHEMA-v1.md`

Author the canonical Trace Schema v1: root fields (`schema_version`, `trace_id`,
`fixture_kind`, `purpose`, `note`, `migration_update_note`, `game_id`,
`rules_version` — canonical string form `race_to_n-rules-v1` —, `engine_version`,
`data_version`, `seed`, `variant`, `options`, `seats`, `commands`, `checkpoints`,
`expected_state_hashes`, `expected_effect_hashes`, `expected_action_tree_hashes`,
`expected_public_view_hashes`, `expected_private_view_hashes` [conditional],
`expected_diagnostics` [conditional], `expected_outcome`, `expected_terminal_state`,
`not_applicable`), the command-stream record fields, the forbidden behavior-key
list, the unknown-field-rejection rule, and the hidden-info/stochastic
not-applicable rationale requirement.

### 2. `docs/TESTING-REPLAY-BENCHMARKING.md`

Link the canonical schema doc from §3 and reconcile the §3 field names to the
canonical set (`action_stream` → `commands`, `expected_legal_action_hashes` →
`expected_action_tree_hashes`). Confirm §14–§16 state the Gate 2 benchmark
hard-fail doctrine and that the §15 Stage-1 budget is binding until formally
recalibrated (the resolution path is owned by 008).

## Files to Touch

- `docs/TRACE-SCHEMA-v1.md` (new)
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify) — link canonical doc; reconcile §3 field names; confirm benchmark hard-fail doctrine

## Out of Scope

- Migrating actual trace files to JSON (GAT2TRAREPBEN-003).
- Implementing `fixture-check` enforcement of this schema (GAT2TRAREPBEN-005).
- Benchmark JSON emission / `thresholds.json` (GAT2TRAREPBEN-006).
- The Stage-1 budget decision record in `BENCHMARKS.md` (GAT2TRAREPBEN-008 / -014).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes; `TRACE-SCHEMA-v1.md` is linked from TESTING with no broken links.
2. `grep -i "selector\|trigger\|on_play" docs/TRACE-SCHEMA-v1.md` — the forbidden-behavior-key list is present.
3. `grep -n "action_stream\|expected_legal_action_hashes" docs/TESTING-REPLAY-BENCHMARKING.md` — returns no live field-name usage (reconciled or pointer only).

### Invariants

1. Trace Schema v1 is defined in exactly one authoritative doc; TESTING references it rather than redefining it.
2. The schema forbids behavior-looking keys and records `rules_version` as the canonical string `race_to_n-rules-v1` (§5/§11; reassess M1).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n "commands\|expected_action_tree_hashes" docs/TESTING-REPLAY-BENCHMARKING.md`
3. A narrower command set is correct here: this ticket ships only doctrine prose; behavior is proven when 003/005 build against the doc.
