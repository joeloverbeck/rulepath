# 8CR3PUBCOOASY-514: C-07 Event Frontier hidden-deeper-deck no-leak matrix

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/event_frontier/tests/{visibility,bots,replay}.rs`
**Deps**: 8CR3PUBCOOASY-504

## Problem

`event_frontier` has no per-seat private holding, but its current and next cards
are public by game rule while the deeper deck order stays hidden. C-07 adopts
the shipped `assert_pairwise_no_leak` geometry to assert that cards deeper than
the public current/next window are absent for the public observer and both seats
across every surface, becoming public only when the game authorizes. Per-seat
private surfaces are N/A (recorded by 802). Source tokens are caller-owned
deck-position sources.

## Assumption Reassessment (2026-06-24)

1. Shipped `assert_pairwise_no_leak` at `crates/game-test-support/src/no_leak.rs:102`.
   Event test files `tests/{visibility,bots,replay}.rs` exist; the dev-dep edge
   is added by 504.
2. Spec §3.8 (Event hidden-tail/public-observer matrix) + §5.8 task `8C-R3-514`
   scope hidden deeper-deck positions × observer/seat0/seat1 × view/tree/
   diagnostic/effect/public-export/bot surfaces, with current/next/resolved
   public transitions. C-07 aggregate verdict is `migrate`; per-seat private
   rows are N/A.
3. Cross-artifact boundary under audit: generic no-leak geometry vs game-owned
   probes/expectations — the harness enumerates only; event/edict reveal and
   resolution policy stay local.
4. FOUNDATIONS §11 no-leak firewall motivates this ticket: still-hidden deeper
   deck cards must not reach any viewer or surface before the game's authorized
   reveal.
5. Enforcement surface: the hidden-tail matrix in `tests/visibility.rs` (+ bot
   no-leak in `tests/bots.rs`, export no-leak in `tests/replay.rs`); zero
   structured leak failures; existing specific tests intact; canary in-memory
   only.

## Architecture Check

1. Adopting the shipped geometry adds systematic public-observer coverage while
   keeping current/next/event-specific tests; cleaner than hand-rolled negatives.
2. No backwards-compatibility alias — new coverage, existing tests retained.
3. `engine-core` untouched; `game-test-support` provides geometry only.

## Verification Layers

1. Hidden-tail no-leak -> `tests/visibility.rs`: cards deeper than the current/
   next window absent for observer/seat0/seat1 across view/tree/diagnostic/effect.
2. Export/bot no-leak -> `tests/replay.rs` (public export omits deeper deck) +
   `tests/bots.rs` (bot input/explanation carries no hidden deeper card).
3. Reveal correctness -> current/next/resolved cards appear exactly when game
   policy authorizes; canary in-memory only (grep-proof no canary committed).

## What to Change

### 1. Add the hidden-tail no-leak matrix

In `tests/visibility.rs`, drive `assert_pairwise_no_leak` over caller-owned
deck-position source tokens × observer/seat0/seat1 × the surface set from spec
§3.8, with caller-supplied expectations. Add export/bot coverage in
`tests/replay.rs` / `tests/bots.rs`. Retain existing specific tests.

## Files to Touch

- `games/event_frontier/tests/visibility.rs` (modify)
- `games/event_frontier/tests/bots.rs` (modify)
- `games/event_frontier/tests/replay.rs` (modify)

## Out of Scope

- Any per-seat private timeline/surface (N/A — recorded by 802).
- Any production code change; replacing existing event/edict tests.
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (new matrix + retained specific tests).
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.
3. Grep-proof: no canary string in `games/event_frontier/tests/golden_traces/` or exports.

### Invariants

1. No still-hidden deeper-deck card reaches any viewer/surface before an
   authorized reveal.
2. Existing specific tests remain and pass; no committed canary exists.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/visibility.rs` — hidden deeper-deck pairwise matrix.
2. `games/event_frontier/tests/bots.rs` — bot no-leak coverage.
3. `games/event_frontier/tests/replay.rs` — public-export no-leak coverage.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game test + replay-check is the correct boundary: the matrix is
   game-owned and test-side.

## Outcome

Completed: 2026-06-24

- Added Event Frontier hidden-tail pairwise no-leak coverage in
  `games/event_frontier/tests/visibility.rs`,
  `games/event_frontier/tests/bots.rs`, and
  `games/event_frontier/tests/replay.rs`.
- The matrix covers observer, seat0, and seat1 across public views, action
  trees, diagnostics, effects, Level1 bot input/decision/rationale, and
  viewer-scoped public replay exports. Current/next public cards are excluded
  from the hidden-tail probes so the assertions target only still-hidden deeper
  deck cards.
- No production code changed; `engine-core` remains untouched and
  `game-test-support` supplies enumeration only.
- Verification:
  - `cargo test -p event_frontier`
  - `cargo run -p replay-check -- --game event_frontier --all`
  - `rg -n "R3_EVENT_NOLEAK_CANARY" games/event_frontier/tests/golden_traces`
    returned no matches.
