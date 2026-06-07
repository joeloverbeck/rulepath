# GAT72GAT8HIG-012: Golden traces + fixture + replay test

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/tests/golden_traces/*.trace.json`, `games/high_card_duel/data/fixtures/high_card_duel_standard.fixture.json`, `games/high_card_duel/tests/replay.rs`
**Deps**: GAT72GAT8HIG-009, GAT72GAT8HIG-010, GAT72GAT8HIG-011

## Problem

Gate 8 needs the deterministic golden-trace set and the setup fixture that pin
the game's behavior (normal game, tie, diagnostics, bot action, hidden-info
observer, seat-private view, public replay export/import, terminal) for
`replay-check` and `fixture-check` to validate.

## Assumption Reassessment (2026-06-07)

1. Verified the trace/fixture conventions: sibling `games/draughts_lite/tests/
   golden_traces/*.trace.json` (incl. `wasm-exported.trace.json`,
   `bot-action.trace.json`, `stale-diagnostic.trace.json`) and
   `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json`;
   `tools/replay-check` reads a per-game `trace_dir`.
2. Verified against the spec: §4.2.9 fixes the ten required traces
   (`shortest-normal`, `tie-round`, `invalid-wrong-seat-diagnostic`,
   `invalid-private-card-redacted`, `stale-diagnostic`, `bot-action`,
   `hidden-info-public-observer`, `seat-private-view`,
   `public-replay-export-import`, `terminal`) and the standard fixture
   (`high_card_duel_standard.fixture.json`).
3. Cross-artifact boundary under audit: the golden-trace + fixture schema
   (`docs/TRACE-SCHEMA-v1.md`) and the internal-vs-public replay distinction from
   009 — the public-export trace must use the hidden-info-safe class, not the
   internal command stream.
4. FOUNDATIONS principle under audit (§11 determinism + no-leak): traces replay
   to identical hashes; the `hidden-info-public-observer` and
   `public-replay-export-import` traces must demonstrate no hidden-identity leak.
5. Enforcement surface named: deterministic replay-hash (§11) and the no-leak
   firewall. Confirm the observer/public-export traces contain no `hcd:r..`
   private identity and the terminal trace does not auto-reveal the deck tail.

## Architecture Check

1. Pinning behavior in golden traces from seeded setups (re-deriving expected
   counts at test start, not hardcoding) is the standard deterministic-evidence
   approach and catches regressions in reveal/score/refill ordering.
2. No backwards-compatibility shims — new traces/fixture.
3. Tests/data only; no `engine-core`/`game-stdlib` change.

## Verification Layers

1. Trace determinism -> deterministic replay-hash check: each trace replays to a stable hash via the game's `replay_support`.
2. Observer/public-export no-leak -> no-leak visibility test: the hidden-info traces carry no private identities.
3. Fixture validity -> schema/serialization validation: the standard fixture parses and matches the setup contract.
4. Trace completeness -> codebase grep-proof: all ten spec §4.2.9 filenames exist under `tests/golden_traces/`.

## What to Change

### 1. Golden traces

Author the ten `*.trace.json` files per spec §4.2.9, each tied to a documented
seed; the `public-replay-export-import` trace exercises the public-export class
from 009.

### 2. Fixture

`data/fixtures/high_card_duel_standard.fixture.json` — standard-variant setup
fixture.

### 3. `tests/replay.rs` (extend)

Assert each golden trace replays to a stable revealed sequence/hash and the
public-export traces are no-leak.

## Files to Touch

- `games/high_card_duel/tests/golden_traces/shortest-normal.trace.json` (new) … `terminal.trace.json` (new) — all ten per §4.2.9
- `games/high_card_duel/data/fixtures/high_card_duel_standard.fixture.json` (new)
- `games/high_card_duel/tests/replay.rs` (modify — extend from 009)

## Out of Scope

- `tools/replay-check`/`fixture-check` registration (GAT72GAT8HIG-013).
- WASM-exported trace via the browser boundary (folded into 016/019 if needed).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel --test replay` — all golden traces replay deterministically.
2. `ls games/high_card_duel/tests/golden_traces/ | wc -l` — the ten required traces present.

### Invariants

1. Traces are deterministic (§11) and the hidden-info traces are no-leak.
2. The standard fixture matches the setup contract.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/golden_traces/*.trace.json` — the ten pinned traces.
2. `games/high_card_duel/data/fixtures/high_card_duel_standard.fixture.json` — setup fixture.

### Commands

1. `cargo test -p high_card_duel --test replay`
2. `cargo test -p high_card_duel`
3. Native replay tests are the correct boundary here; `replay-check`/`fixture-check` CLI confirmation lands with tool registration (013).
