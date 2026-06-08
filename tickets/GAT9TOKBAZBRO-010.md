# GAT9TOKBAZBRO-010: Golden traces + replay test

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/tests/golden_traces/*.trace.json` (new, 12 traces), `tests/replay.rs` (new)
**Deps**: GAT9TOKBAZBRO-007, GAT9TOKBAZBRO-009

## Problem

Deterministic replay must be pinned by golden traces covering normal play,
terminal, contract refill, exchange, market exhaustion, each invalid diagnostic,
stale-command rejection, a bot action, and WASM export. This ticket authors the
twelve golden traces named by the spec and the replay test that re-derives each
trace's hashes from the command stream, proving byte-deterministic reproduction.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/replay_support.rs` (GAT9TOKBAZBRO-007) provides the
   deterministic hashes + export/import this test exercises; the standard fixture
   and the diagnostics exist from -009/-004. The sibling
   `games/high_card_duel/tests/golden_traces/*.trace.json` + `tests/replay.rs`
   establish the house pattern (verified: 10 trace files + `replay.rs` present).
2. The exact trace set is fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Expected golden traces": `shortest-normal`, `terminal-turn-cap`,
   `contract-fulfill-refill`, `market-exhaustion`, `exchange`,
   `supply-exhaustion-diagnostic`, `insufficient-resources-diagnostic`,
   `empty-slot-diagnostic`, `stale-diagnostic`, `non-active-seat-diagnostic`,
   `bot-action`, `wasm-exported`.
3. Cross-artifact boundary under audit: the trace-schema contract from
   `docs/TRACE-SCHEMA-v1.md` and the replay/hash surface. Each trace conforms to
   the v1 trace schema; the replay test re-derives hashes rather than trusting
   stored values. These traces are also consumed by `replay-check --all` (-012).
4. FOUNDATIONS §11 (deterministic replay/hash/traces): each trace's stored
   action-tree, public-view, and diagnostic hashes must re-derive identically from
   the command stream; re-enumerate expected counts from the fixture at test start
   rather than hardcoding.
5. Replay/hash + no-leak surface: the `wasm-exported` trace proves the public
   export round-trips, and the trace files (a replay-export surface) must carry no
   bot/debug/candidate/internal field — the no-leak firewall extends to exported
   traces. Confirm every diagnostic trace shows rejection-without-mutation.

## Architecture Check

1. Authoring all twelve traces in one ticket (they share the fixture and the
   replay harness) keeps the deterministic-replay proof atomic and reviewable;
   re-deriving hashes (not asserting stored constants) is the robust design.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` untouched; traces are game-local test data over the generic
   trace schema. No `game-stdlib` helper added.

## Verification Layers

1. Deterministic reproduction -> deterministic replay-hash check (`tests/replay.rs`
   re-derives each trace's hashes from its command stream).
2. Trace-schema conformance -> schema validation against `docs/TRACE-SCHEMA-v1.md`.
3. Each invalid case rejects without mutation -> the five diagnostic traces +
   replay test assertions.
4. Public export round-trip + no-leak -> `wasm-exported` trace + assertion that no
   trace carries an internal-only field.

## What to Change

### 1. `games/token_bazaar/tests/golden_traces/*.trace.json`

The twelve traces listed above, each a command stream + expected hashes/outcome
conforming to `docs/TRACE-SCHEMA-v1.md`.

### 2. `games/token_bazaar/tests/replay.rs`

Load each trace, replay its command stream from the standard fixture, and assert
the re-derived final state, effects, action-tree hash, public-view hash, outcome,
terminal state, and (for diagnostic traces) rejection-without-mutation match.

## Files to Touch

- `games/token_bazaar/tests/golden_traces/shortest-normal.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/terminal-turn-cap.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/contract-fulfill-refill.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/market-exhaustion.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/exchange.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/supply-exhaustion-diagnostic.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/insufficient-resources-diagnostic.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/empty-slot-diagnostic.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/non-active-seat-diagnostic.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/bot-action.trace.json` (new)
- `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` (new)
- `games/token_bazaar/tests/replay.rs` (new)

## Out of Scope

- The `replay-check` tool arm that runs `--all` (GAT9TOKBAZBRO-012).
- WASM export wiring (GAT9TOKBAZBRO-013) — the `wasm-exported` trace pins the
  public-export shape the WASM ticket must reproduce.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar --test replay` — every trace's hashes re-derive
   identically from its command stream.
2. `cargo test -p token_bazaar --test replay` — each diagnostic trace shows
   rejection without mutation.
3. `cargo test -p token_bazaar` — full crate suite remains green with traces added.

### Invariants

1. Replay is byte-deterministic across all twelve traces.
2. Stored hashes equal re-derived hashes (no drift); counts re-enumerated from the
   fixture, not hardcoded.
3. No trace file carries a bot/debug/candidate/internal field.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/replay.rs` — re-derive + assert all twelve traces.

### Commands

1. `cargo test -p token_bazaar --test replay`
2. `cargo test -p token_bazaar`
3. The standalone `replay-check -- --game token_bazaar --all` pipeline run is
   verified in GAT9TOKBAZBRO-012 once the tool arm exists.
