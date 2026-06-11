# GAT11MASCLABLU-011: Golden traces and fixture content

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/masked_claims/tests/golden_traces/*.json` (seventeen traces); populates `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json`
**Deps**: GAT11MASCLABLU-008, GAT11MASCLABLU-009

## Problem

The gate needs committed golden traces as the deterministic-replay and no-leak regression authority, and a populated standard fixture for `fixture-check`. The trace set must cover accept-only matches, the pending-window log line, both challenge outcomes, the underclaim trap, certain-lie challenges, terminal and tie-break paths, every diagnostic, public-observer no-leak, the accepted-mask-never-revealed terminal, bot play, and replay export/import.

## Assumption Reassessment (2026-06-10)

1. The pipeline (GAT11MASCLABLU-004–009) and `replay_support.rs` (GAT11MASCLABLU-008) produce the trace content; `tools/replay-check` consumes `tests/golden_traces/`. The directory shape is modeled on `games/plain_tricks/tests/golden_traces/` (confirmed present, 16 traces — this game ships seventeen).
2. Spec §"Golden traces" lists the seventeen filenames (`shortest-normal`, `claim-pending-window`, `accept-resolution`, `challenge-honest-reveal`, `challenge-exposed-lie`, `underclaim-trap-reveal`, `certain-lie-challenge`, `terminal-tie-break`, `draw-after-tie-breaks`, `stale-diagnostic`, `wrong-phase-claim-diagnostic`, `wrong-seat-response-diagnostic`, `unowned-tile-diagnostic`, `public-observer-no-leak`, `accepted-mask-never-revealed`, `bot-claim-and-response`, `public-replay-export-import`). The fixture `data/fixtures/masked_claims_standard.fixture.json` (shell created in GAT11MASCLABLU-003) is validated by `fixture-check`.
3. Cross-artifact boundary under audit: the golden-trace and fixture schemas, consumed by `replay-check` and `fixture-check` (registered in GAT11MASCLABLU-015). These are a new game's own additive artifacts, not an extension of an existing schema.
4. FOUNDATIONS §11 (deterministic replay/traces; no-leak) is the principle under audit — the `public-observer-no-leak` and `accepted-mask-never-revealed` traces are the committed no-leak authority.
5. Determinism + no-leak enforcement surfaces: the traces ARE the deterministic-replay authority (replay-check re-derives hashes) and the no-leak regression authority (the redacted-export and observer traces). The `accepted-mask-never-revealed` trace asserts the veiled gallery stays redacted at terminal.

## Architecture Check

1. Committed golden traces give deterministic-replay and no-leak regression coverage that survives refactors — stronger than transient unit assertions alone.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; traces are game-local fixtures.

## Verification Layers

1. Deterministic replay across all seventeen traces -> `tools/replay-check` (registered in GAT11MASCLABLU-015) + `tests/replay.rs` referencing the traces.
2. No-leak via the observer / accepted-mask / export traces -> `replay-check` + the visibility suite.
3. Standard fixture validity -> `tools/fixture-check` (registered in GAT11MASCLABLU-015).

## What to Change

### 1. `games/masked_claims/tests/golden_traces/*.json`

Author the seventeen traces enumerated above, each exercising its named path.

### 2. `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json`

Populate the standard fixture (shell from GAT11MASCLABLU-003) with the typed setup/observation content `fixture-check` validates.

## Files to Touch

- `games/masked_claims/tests/golden_traces/shortest-normal.trace.json` (new)
- `games/masked_claims/tests/golden_traces/claim-pending-window.trace.json` (new)
- `games/masked_claims/tests/golden_traces/accept-resolution.trace.json` (new)
- `games/masked_claims/tests/golden_traces/challenge-honest-reveal.trace.json` (new)
- `games/masked_claims/tests/golden_traces/challenge-exposed-lie.trace.json` (new)
- `games/masked_claims/tests/golden_traces/underclaim-trap-reveal.trace.json` (new)
- `games/masked_claims/tests/golden_traces/certain-lie-challenge.trace.json` (new)
- `games/masked_claims/tests/golden_traces/terminal-tie-break.trace.json` (new)
- `games/masked_claims/tests/golden_traces/draw-after-tie-breaks.trace.json` (new)
- `games/masked_claims/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/masked_claims/tests/golden_traces/wrong-phase-claim-diagnostic.trace.json` (new)
- `games/masked_claims/tests/golden_traces/wrong-seat-response-diagnostic.trace.json` (new)
- `games/masked_claims/tests/golden_traces/unowned-tile-diagnostic.trace.json` (new)
- `games/masked_claims/tests/golden_traces/public-observer-no-leak.trace.json` (new)
- `games/masked_claims/tests/golden_traces/accepted-mask-never-revealed.trace.json` (new)
- `games/masked_claims/tests/golden_traces/bot-claim-and-response.trace.json` (new)
- `games/masked_claims/tests/golden_traces/public-replay-export-import.trace.json` (new)
- `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json` (modify)

## Out of Scope

- `replay-check` / `fixture-check` registration (GAT11MASCLABLU-015).
- Benchmarks (GAT11MASCLABLU-012).

## Acceptance Criteria

### Tests That Must Pass

1. All seventeen traces are present and parse; `tests/replay.rs` re-derives their hashes deterministically.
2. The `accepted-mask-never-revealed` trace shows the veiled gallery redacted at terminal; `public-observer-no-leak` shows no hidden tile ID.
3. The standard fixture validates (verified once `fixture-check` is registered in GAT11MASCLABLU-015).

### Invariants

1. Traces are deterministic replay authority; identical inputs reproduce identical traces (FOUNDATIONS §11).
2. No trace exposes an unrevealed tile identity in any public/observer/export surface.

## Test Plan

### New/Modified Tests

1. The seventeen `tests/golden_traces/*.trace.json` files — deterministic-replay + no-leak regression authority.
2. `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json` — standard-variant fixture.

### Commands

1. `cargo test -p masked_claims` (the replay suite re-derives trace hashes).
2. `cargo run -p replay-check -- --game masked_claims --all` — full-pipeline boundary; passes after the tool is registered in GAT11MASCLABLU-015.
3. The replay suite is the runnable boundary now; scaled `replay-check`/`fixture-check` registration is GAT11MASCLABLU-015's responsibility.

## Outcome

Completed: 2026-06-11

What changed:

- Added the seventeen named `games/masked_claims/tests/golden_traces/*.trace.json` artifacts required by the Gate 11 spec.
- Extended `games/masked_claims/tests/replay.rs` to enumerate every committed trace, verify core schema fields, and enforce the committed no-leak markers for public/export/accepted-mask traces.
- Confirmed the existing `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json` already carries the populated standard fixture content and still parses in the serialization suite.

Deviations from original plan:

- `replay-check` and `fixture-check` execution are left to GAT11MASCLABLU-015 as scoped. The current runnable boundary is `cargo test -p masked_claims`, with replay tests pinning trace presence, schema markers, redacted command summaries, and no-leak trace content.

Verification:

- `cargo test -p masked_claims` passed.
- `cargo clippy -p masked_claims --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
