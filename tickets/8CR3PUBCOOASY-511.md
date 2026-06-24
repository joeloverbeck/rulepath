# 8CR3PUBCOOASY-511: C-07 Plain Tricks pairwise no-leak matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/plain_tricks/tests/{visibility,bots,replay}.rs`
**Deps**: 8CR3PUBCOOASY-501

## Problem

`plain_tricks` has real per-seat private hands and is the only R3 game needing a
full seat-private no-leak matrix. C-07 adopts the shipped
`game_test_support::no_leak::assert_pairwise_no_leak` geometry to enumerate
caller-supplied facts/viewers/surfaces and assert expected presence/absence,
augmenting (never replacing) the existing specific deal-privacy / follow-suit /
export tests. The harness owns enumeration only; the game owns every probe,
viewer choice, and expectation.

## Assumption Reassessment (2026-06-24)

1. Shipped `assert_pairwise_no_leak` is at
   `crates/game-test-support/src/no_leak.rs:102`. Plain test files
   `tests/{visibility,bots,replay}.rs` exist; the dev-dep edge is added by 501.
2. Spec §3.8 (Plain full seat-private matrix) + §5.8 task `8C-R3-511` scope the
   matrix across both source seats × observer/owner/opponent × view/tree/
   diagnostic/effect/public-export/seat-private-export/bot surfaces, pre/post
   play and terminal tail. ADR 0004 applies to every viewer-scoped export step.
3. Cross-artifact boundary under audit: the generic no-leak geometry vs the
   game-owned probes/expectations — the harness must not project state, choose
   viewers, infer hidden facts, decide reveal timing, filter effects, execute
   actions, or create export policy.
4. FOUNDATIONS §11 no-leak firewall motivates this ticket: facts private to a
   seat must not reach the other seat, the public observer, DOM/logs, bot
   explanations, candidate rankings, or replay exports unless the game
   authorizes the reveal.
5. Enforcement surface: the pairwise matrix in `tests/visibility.rs` (+ bot
   no-leak in `tests/bots.rs`, export no-leak in `tests/replay.rs`); zero
   structured leak failures, and the existing specific tests remain intact.
   Canaries are in-memory only — never written to any committed artifact.

## Architecture Check

1. Adopting the shipped geometry augments coverage with systematic pairwise
   enumeration while keeping every game-specific test; cleaner than hand-rolling
   per-surface negative tests.
2. No backwards-compatibility alias — new test coverage, existing tests retained.
3. `engine-core` untouched; `game-test-support` provides geometry only — no
   game fact or reveal policy enters the shared crate.

## Verification Layers

1. Seat-private no-leak -> `tests/visibility.rs` pairwise matrix: hand/legal-tree
   facts present only for the owner, absent for opponent/observer pre-reveal.
2. Bot no-leak -> `tests/bots.rs`: bot input/explanation/candidate rendering
   carries no non-owned private card.
3. Export no-leak -> `tests/replay.rs`: public export omits hidden hand/tail;
   seat-private export shows only the selected viewer's facts; canary in-memory
   only (grep-proof no canary string in committed traces/exports).

## What to Change

### 1. Add the pairwise no-leak matrix

In `tests/visibility.rs`, drive `assert_pairwise_no_leak` over both source seats
× observer/owner/opponent × the surface set from spec §3.8, with caller-supplied
source facts and expectations. Add the bot-surface and export-surface coverage
in `tests/bots.rs` and `tests/replay.rs`. Retain all existing specific tests.

## Files to Touch

- `games/plain_tricks/tests/visibility.rs` (modify)
- `games/plain_tricks/tests/bots.rs` (modify)
- `games/plain_tricks/tests/replay.rs` (modify)

## Out of Scope

- Any production code change (geometry is test-side; the game's probes are
  supplied by the test, not new production surfaces).
- Replacing any existing specific deal/follow-suit/export/bot test with the
  generic matrix.
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (new matrix + retained specific tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.
3. Grep-proof: no canary string in `games/plain_tricks/tests/golden_traces/` or exports.

### Invariants

1. No seat-private fact reaches opponent/observer/public/bot/export before an
   authorized reveal.
2. Existing specific tests remain and pass; no committed canary exists.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/visibility.rs` — pairwise seat-private matrix via
   `assert_pairwise_no_leak`.
2. `games/plain_tricks/tests/bots.rs` — bot-input/explanation no-leak coverage.
3. `games/plain_tricks/tests/replay.rs` — public + seat-private export no-leak
   coverage with in-memory-only canary.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: the matrix is
   game-owned and test-side; byte neutrality is asserted by replay-check.
