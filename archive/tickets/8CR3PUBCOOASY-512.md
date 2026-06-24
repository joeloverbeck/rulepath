# 8CR3PUBCOOASY-512: C-07 Flood Watch hidden-future-deck no-leak matrix

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/flood_watch/tests/{visibility,bots,replay}.rs`
**Deps**: 8CR3PUBCOOASY-502

## Problem

`flood_watch` has no per-seat private holding, but it has a public forecast with
a hidden future event-deck tail. C-07 adopts the shipped
`assert_pairwise_no_leak` geometry to assert that event cards deeper than the
public forecast are absent for the public observer and both seats across every
surface, becoming public only when the game authorizes a reveal. Per-seat
private surfaces are N/A (recorded by 802). Source tokens are caller-owned (e.g.
`EventDeckIndex(i)`), not pretend-seat-owned.

## Assumption Reassessment (2026-06-24)

1. Shipped `assert_pairwise_no_leak` at `crates/game-test-support/src/no_leak.rs:102`.
   Flood test files `tests/{visibility,bots,replay}.rs` exist; the dev-dep edge
   is added by 502.
2. Spec §3.8 (Flood hidden-future/public-observer matrix) + §5.8 task
   `8C-R3-512` scope hidden event-deck positions × observer/seat0/seat1 ×
   view/tree/diagnostic/effect/public-export/bot surfaces, with forecast/drawn
   public transitions. C-07 aggregate verdict is `migrate`; per-seat private
   rows are N/A.
3. Cross-artifact boundary under audit: generic no-leak geometry vs game-owned
   probes/expectations — the harness enumerates only; cooperative role identity
   and event/forecast rules stay local.
4. FOUNDATIONS §11 no-leak firewall motivates this ticket: undrawn future-deck
   cards must not reach any viewer or surface before the game's authorized
   reveal.
5. Enforcement surface: the hidden-future matrix in `tests/visibility.rs` (+ bot
   no-leak in `tests/bots.rs`, export no-leak in `tests/replay.rs`); zero
   structured leak failures; existing specific tests intact; canary in-memory
   only.

## Architecture Check

1. Adopting the shipped geometry adds systematic public-observer coverage while
   keeping forecast/event-specific tests; cleaner than hand-rolled negatives.
2. No backwards-compatibility alias — new coverage, existing tests retained.
3. `engine-core` untouched; `game-test-support` provides geometry only.

## Verification Layers

1. Hidden-future no-leak -> `tests/visibility.rs`: cards deeper than the public
   forecast absent for observer/seat0/seat1 across view/tree/diagnostic/effect.
2. Export/bot no-leak -> `tests/replay.rs` (public export omits future deck) +
   `tests/bots.rs` (bot input/explanation carries no undrawn card).
3. Reveal correctness -> drawn/resolved cards become public only after the game
   authorizes; canary in-memory only (grep-proof no canary in committed
   artifacts).

## What to Change

### 1. Add the hidden-future no-leak matrix

In `tests/visibility.rs`, drive `assert_pairwise_no_leak` over hidden
`EventDeckIndex`-style source tokens × observer/seat0/seat1 × the surface set
from spec §3.8, with caller-supplied expectations identical across viewers
unless the game authorizes a public reveal. Add export/bot coverage in
`tests/replay.rs` / `tests/bots.rs`. Retain existing specific tests.

## Files to Touch

- `games/flood_watch/tests/visibility.rs` (modify)
- `games/flood_watch/tests/bots.rs` (modify)
- `games/flood_watch/tests/replay.rs` (modify)

## Out of Scope

- Any per-seat private timeline/surface (N/A — recorded by 802).
- Any production code change; replacing existing forecast/event tests.
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (new matrix + retained specific tests).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. Grep-proof: no canary string in `games/flood_watch/tests/golden_traces/` or exports.

### Invariants

1. No undrawn future-deck card reaches any viewer/surface before an authorized
   reveal.
2. Existing specific tests remain and pass; no committed canary exists.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/visibility.rs` — hidden-future-deck pairwise matrix.
2. `games/flood_watch/tests/bots.rs` — bot no-leak coverage.
3. `games/flood_watch/tests/replay.rs` — public-export no-leak coverage.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: the matrix is
   game-owned and test-side.

## Outcome

Completed: 2026-06-24

- Added a Flood-owned hidden-future deck no-leak matrix in
  `games/flood_watch/tests/visibility.rs` over observer/seat viewers and public
  view/action-tree/diagnostic/effect surfaces.
- Added `games/flood_watch/tests/bots.rs` Level1 bot input/decision/rationale
  coverage for hidden future deck identities.
- Added `games/flood_watch/tests/replay.rs` public export coverage for
  observer and seat-scoped exports, plus an in-memory canary check.
- Verified `cargo test -p flood_watch`,
  `cargo run -p replay-check -- --game flood_watch --all`, and
  `rg -n "R3_FLOOD_NOLEAK_CANARY" games/flood_watch/tests/golden_traces reports archive specs tickets docs`
  with no matches.
