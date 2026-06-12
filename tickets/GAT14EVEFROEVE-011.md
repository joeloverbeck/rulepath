# GAT14EVEFROEVE-011: Native test suite and golden traces

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier/tests/{rules,property,replay,serialization,visibility,bots}.rs` (full coverage matrix); `games/event_frontier/tests/golden_traces/*.json` (eighteen golden traces)
**Deps**: GAT14EVEFROEVE-009, GAT14EVEFROEVE-010

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` §6/FOUNDATIONS §6 require evidence-heavy coverage before a game is admitted. The rule/visibility/bot tickets each added their targeted tests; this ticket completes the full coverage matrix (rules, diagnostics, property, replay, serialization, visibility, bots) and authors the **eighteen golden traces** the spec enumerates, each carrying Trace Schema v1 §5 stochastic (setup shuffle) and hidden-info (undrawn deck order) markers, checked by `replay-check`. It follows the failing-test protocol: tests prove behavior; code is fixed, tests are never weakened to get green.

## Assumption Reassessment (2026-06-12)

1. The behavior under test exists end to end: verified tickets 004–010 implement setup, eligibility/initiative, operations, events/edicts, Reckoning/victory/terminal, visibility/replay, and bots, with their targeted tests added to `tests/{rules,property,replay,serialization,visibility,bots}.rs` (created progressively by those tickets). This ticket fills gaps to full coverage and adds the golden traces.
2. The trace set and markers are specified: verified the spec's "Golden traces" — the eighteen named `*.trace.json` files (instant wins, fallback, event/op/limited-op, pass/double-pass, no-eligible discard, edict activation/expiry + block diagnostic, Reckoning breakdown, Reckoning-never-first, two scenario setups, ineligible diagnostic, bot-vs-bot, replay-export-import-no-deck-leak) and the Trace Schema v1 §5 markers (`docs/TESTING-REPLAY-BENCHMARKING.md`).
3. Cross-artifact boundary under audit: golden traces are the replay-determinism contract `tools/replay-check` (ticket 015) validates; their hashes (state/effect/action-tree/view) must be stable and reproducible from seed + scenario + command stream. The no-leak traces prove undrawn order never appears in an export.
4. FOUNDATIONS §11 (deterministic replay/hashes/traces; no hidden-info leak; evidence coverage) motivates this ticket. Restated before trusting the spec: the visibility tests use serialize-and-search (§8) to prove no payload/export contains undrawn order; replay tests prove byte-identical reproduction.
5. No-leak + determinism enforcement surface (§11): this ticket is where the firewall and determinism are proven, not merely structured. Confirm serialize-and-search covers projections, action trees, previews, diagnostics, effects, bot explanations, and replay exports; confirm every golden trace reproduces identical hashes. No replay/hash semantics change — coverage of the existing contract.

## Architecture Check

1. Authoring the full matrix + golden traces as one comprehensive ticket (after the surfaces exist) is cleaner than scattering trace authorship: it lets the trace set be enumerated from one fixture and keeps `replay-check` registration (ticket 015) pointing at a complete trace dir.
2. No backwards-compatibility aliasing/shims — additive tests and traces.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; tests never weakened to pass (failing-test protocol).

## Verification Layers

1. Rule + diagnostic coverage -> `tests/rules.rs` covers every eligibility-table cell, op legality/bounds/costs, each event effect, each edict, the Reckoning order, both victories, fallback/tiebreak; diagnostics are viewer-safe.
2. Property invariants -> `tests/property.rs` asserts flow-never-stalls, eligibility consistency, non-negative capped resources, edict expiry at Reckoning, Reckonings once per epoch, victory-before-reset, component-count provenance, replay determinism, no panics.
3. Replay/serialization determinism -> `tests/replay.rs` + `tests/serialization.rs` reproduce hashes and round-trip stable summaries with unknown/behavior-field rejection.
4. Visibility no-leak + bot legality -> `tests/visibility.rs` serialize-and-search; `tests/bots.rs` legality/determinism; golden traces under `golden_traces/` carry the §5 markers.

## What to Change

### 1. Complete the test matrix

Fill `tests/{rules,property,replay,serialization,visibility,bots}.rs` to the full coverage listed under the spec's "Native rules, replay, visibility, and bot evidence" and "Diagnostics tests" — every eligibility cell, op/event/edict/Reckoning/victory rule, every diagnostic, the property invariants, replay/serialization determinism, visibility no-leak, and bot evidence.

### 2. Author the eighteen golden traces

Create `tests/golden_traces/` with the eighteen named traces (per the spec's "Golden traces" list), each carrying Trace Schema v1 §5 stochastic (setup shuffle) + hidden-info (undrawn order) markers, per-seat surfaces not applicable.

## Files to Touch

- `games/event_frontier/tests/rules.rs` (modify; created by 004)
- `games/event_frontier/tests/property.rs` (modify; created by 005)
- `games/event_frontier/tests/replay.rs` (modify; created by 004)
- `games/event_frontier/tests/serialization.rs` (modify; created by 003/004)
- `games/event_frontier/tests/visibility.rs` (modify; created by 009)
- `games/event_frontier/tests/bots.rs` (modify; created by 010)
- `games/event_frontier/tests/golden_traces/` — eighteen `*.trace.json` (new)

## Out of Scope

- Tool registration (`replay-check`/`fixture-check`/`rule-coverage`) — ticket 015; this ticket authors the traces those tools validate.
- Benchmarks (ticket 012) and the simulation balance run (ticket 013/015).
- Weakening or deleting any test to get green — follow the failing-test protocol; fix the code.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes the full suite with no silent gaps.
2. The eighteen golden traces exist and reproduce identical hashes on replay.
3. Serialize-and-search visibility tests confirm no payload/export contains undrawn deck order.

### Invariants

1. Every rule, diagnostic, and invariant the spec enumerates is covered by a test; no test is weakened to pass.
2. Every golden trace is deterministically reproducible and carries the §5 stochastic + hidden-info markers.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/*.rs` — the full coverage matrix (rules/property/replay/serialization/visibility/bots).
2. `games/event_frontier/tests/golden_traces/*.trace.json` — the eighteen enumerated traces.

### Commands

1. `cargo test -p event_frontier`
2. `cargo test --workspace`
3. The per-crate suite plus a workspace test run is the correct boundary; `replay-check`/`rule-coverage` CLI validation lands with tool registration (ticket 015).
