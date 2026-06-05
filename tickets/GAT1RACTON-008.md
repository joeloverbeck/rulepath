# GAT1RACTON-008: Replay/hash, golden traces, serialization + property tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/race_to_n` gains replay reproduction tests, the required golden-trace set, and the deterministic-hash test surface (no new production logic beyond test/trace fixtures).
**Deps**: GAT1RACTON-006, GAT1RACTON-007

## Problem

Replay is first-class from the first tiny game (ARCHITECTURE §8). This ticket
proves that seed + options + command stream reproduce identical state, effect,
action-tree, and public-view hashes (TESTING §4; spec §5 exit criterion "replay
reproduces hashes"), and produces the required golden-trace set: shortest-normal,
terminal, bot-action, and invalid/stale diagnostic traces (OFFICIAL-GAME-CONTRACT
§11; spec §6).

## Assumption Reassessment (2026-06-05)

1. The hash + replay/command-stream + serialization contracts exist in
   `engine-core` (GAT1RACTON-003); `race_to_n` has rules/effects (005),
   projection/serialization (006), and a wired bot (007). All inputs to replay
   exist. No `tests/golden_traces/` or `tests/replay_tests.rs` exists yet.
2. The trace set required is OFFICIAL-GAME-CONTRACT §11 / TESTING §4: shortest
   normal, terminal, bot-action, invalid/stale diagnostic. `race_to_n` is
   perfect-information and has no random *game* events (RNG is bot-only), so the
   redacted-hidden-info trace and the stochastic-game trace are **not-applicable**
   (record the rationale); the bot-action trace exercises the bot RNG path.
3. Cross-artifact boundary under audit: golden traces are executable historical
   evidence (TESTING §3) carrying expected state/effect/action-tree/public-view
   hashes; they bind the serialized forms from GAT1RACTON-006 and the effect order
   from GAT1RACTON-005. A trace fixture lives under `games/race_to_n/tests/golden_traces/`
   and `games/race_to_n/data/fixtures/`.
4. FOUNDATIONS §11 (replay, hashes, serialization order, RNG, traces remain
   deterministic) motivates this ticket. This is verification, not a replay/hash
   *semantics* change, so no §13 ADR trigger — it establishes the baseline golden
   traces.
5. Deterministic replay/hash enforcement surface: this IS the §11 determinism
   enforcement surface. Confirm: identical game id + rules/data version + seed +
   seats + options + command stream reproduce identical state/effect/
   action-tree/public-view hashes and identical outcome/terminal state
   (ARCHITECTURE §8; TESTING §4); golden-trace drift fails loudly with a required
   note (TESTING §3). No hidden information in traces (perfect-info game; traces
   are public-safe — IP/no-leak note recorded).
6. Schema/contract: golden traces consume the replay-record + view/effect/hash
   JSON shapes (GAT1RACTON-003/006). Additive test fixtures; no production schema
   changed. Property tests assert the TESTING §6 invariants.

## Architecture Check

1. Proving replay/hash in-crate at Gate 1 (basic replay test + ≥1 golden trace)
   is exactly the spec's Assumption 3 scope; the standalone tool hardening
   (`replay-check`, `seed-reducer`) is deferred to Gate 2. Re-enumerating expected
   hashes from the fixture at test start (not hardcoding) prevents stale counts.
2. No backwards-compatibility shims.
3. `engine-core` untouched; all trace/test material lives in `games/race_to_n`.
   `game-stdlib` untouched.

## Verification Layers

1. Replay reproduces hashes -> deterministic replay-hash check (seed + options +
   command stream → identical state/effect/action-tree/public-view hashes +
   outcome; TESTING §4).
2. Golden-trace drift loudness -> golden trace check (a behavior change fails the
   trace test; update requires a note — TESTING §3).
3. Serialization round-trip under replay -> serialization test (replay JSON
   round-trips and reproduces; extends GAT1RACTON-006's coverage).
4. State/action invariants -> property/invariant test (TESTING §6: no-panic,
   no-invalid-state, conservation, terminal-no-actions).

## What to Change

### 1. Replay tests (`tests/replay_tests.rs`)

Drive setup → command stream → assert reproduced state/effect/action-tree/
public-view hashes + outcome match a second run from the same inputs.

### 2. Golden traces (`tests/golden_traces/` + `data/fixtures/`)

Author and check the four traces: shortest-normal, terminal, bot-action,
invalid/stale diagnostic — each with expected hashes and a note explaining why it
exists (TESTING §3). Record stochastic/redacted traces as not-applicable.

### 3. Property/invariant tests (extend `tests/property_tests.rs`)

Assert TESTING §6 invariants over many states: replay hashes deterministic,
serialization round-trips preserve state, action trees use stable IDs.

## Files to Touch

- `games/race_to_n/tests/replay_tests.rs` (new)
- `games/race_to_n/tests/golden_traces/` (new — trace fixtures + harness)
- `games/race_to_n/data/fixtures/` (modify) — add replay/trace fixtures
- `games/race_to_n/tests/property_tests.rs` (modify) — add replay/serialization invariants

## Out of Scope

- Standalone `replay-check` / `seed-reducer` / `trace-viewer` hardening (Gate 2;
  spec §2 out-of-scope).
- The 100k-seed simulation harness (GAT1RACTON-009) — distinct from these
  deterministic replay fixtures.
- CI wiring of these tests (GAT1RACTON-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n --test replay_tests` — replay reproduces identical state/effect/action-tree/public-view hashes + outcome.
2. `cargo test -p race_to_n` — golden-trace tests pass; property/invariant tests pass.
3. A deliberate behavior tweak makes a golden-trace test fail (drift is loud) — verified manually, then reverted.

### Invariants

1. Identical inputs+versions reproduce identical hashes and outcome (ARCHITECTURE §8; FOUNDATIONS §11).
2. Golden-trace drift fails the test and requires an explanatory note (TESTING §3).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/replay_tests.rs` — seed+options+command-stream hash reproduction.
2. `games/race_to_n/tests/golden_traces/*` — shortest-normal, terminal, bot-action, invalid/stale diagnostic traces with expected hashes + notes.
3. `games/race_to_n/tests/property_tests.rs` — replay determinism + serialization round-trip invariants.

### Commands

1. `cargo test -p race_to_n --test replay_tests`
2. `cargo test --workspace`
3. `ls games/race_to_n/tests/golden_traces` — confirm the four required traces exist.
