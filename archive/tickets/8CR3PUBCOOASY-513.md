# 8CR3PUBCOOASY-513: C-07 Frontier Control no-leak N/A + equality receipt

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (no-leak test geometry) — `games/frontier_control/tests/visibility.rs`
**Deps**: 8CR3PUBCOOASY-503

## Problem

`frontier_control` is fully public: its visibility implementation returns the
same projection for observer, seat 0, and seat 1, and setup has no randomness or
hidden holdings. C-07 is therefore `not-applicable` for a pairwise hidden-fact
matrix. This ticket retains the focused observer/seat equality and public
effect/export tests (it does NOT replace them with a vacuous generic matrix) and
records the N/A characterization. No artificial secret canary is inserted.

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/tests/visibility.rs` exists; the visibility
   implementation (`src/visibility.rs`) returns equal projections for all three
   viewers (confirmed: setup is RNG-free, no `next_bounded_index_unbiased`).
   The dev-dep edge is added by 503.
2. Spec §3.8 (Frontier fully public) sub-surface verdicts: pairwise hidden-fact
   matrix `not-applicable`; observer/seat equality `exception` (retain focused
   tests); seat-private export `not-applicable`. §5.8 task `8C-R3-513` scopes the
   N/A characterization + equality retention. The N/A receipts are recorded by
   802.
3. Cross-artifact boundary under audit: there is no hidden source, private
   holding, viewer redaction, or hidden export class to enumerate — the harness
   has nothing to redact.
4. FOUNDATIONS §11 no-leak firewall still motivates the audit: the ticket proves
   the absence of any hidden source rather than inventing one (Forbidden change
   #7 bans treating fully-public state as hidden-information by inserting a
   secret).
5. Enforcement surface: the retained observer=seat0=seat1 projection and public
   effect/export equality tests in `tests/visibility.rs`; no canary, no fake
   secret, byte-identical to the 001 baseline.

## Architecture Check

1. Recording the N/A and retaining specific equality tests is correct for a
   fully-public game; a generic pairwise matrix here would be vacuous and could
   mask a regression.
2. No backwards-compatibility alias — existing equality tests retained, not
   replaced.
3. `engine-core` untouched; no `game-stdlib`/`game-test-support` geometry forced
   onto a game with nothing to redact.

## Verification Layers

1. Projection equality -> `tests/visibility.rs`: observer, seat 0, seat 1
   projections and public effect streams are equal (retained, specific).
2. No hidden source -> characterization proof (recorded for 802) that setup is
   RNG-free with no private holding/effect/redaction class.
3. No fake canary -> grep-proof no artificial secret in tests or committed
   artifacts.

## What to Change

### 1. Retain equality tests + record N/A

In `games/frontier_control/tests/visibility.rs`, keep (and, if helpful, clarify)
the observer=seat0=seat1 projection and public effect/export equality tests.
Record the C-07 N/A characterization (no hidden source / no seat-private
timeline) as evidence for the register ticket 802. Do not add a generic matrix
or any artificial secret.

## Files to Touch

- `games/frontier_control/tests/visibility.rs` (modify)

## Out of Scope

- Adding a pairwise hidden-fact matrix or a seat-private export (both N/A).
- Inserting any artificial secret/canary to force a hidden-info test.
- Any production code change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (retained equality tests pass).
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.
3. Grep-proof: no artificial secret/canary in frontier_control tests or traces.

### Invariants

1. Observer/seat projections remain equal and specifically tested.
2. No artificial secret is introduced; the N/A is characterized, not faked.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/visibility.rs` — retained/clarified
   observer=seat0=seat1 equality coverage (no new generic matrix).

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game test + replay-check is the correct boundary: the equality tests
   are game-owned; the N/A is evidence-only.

## Outcome

Completed: 2026-06-24

- Added an explicit C-07 N/A receipt test in
  `games/frontier_control/tests/visibility.rs` that verifies repeated setup
  equality, observer/seat projection equality, and identical public-effect
  filtering for all viewers.
- Did not add a pairwise hidden-fact matrix, seat-private export, or artificial
  secret/canary; Frontier Control remains fully public.
- Verified `cargo test -p frontier_control`,
  `cargo run -p replay-check -- --game frontier_control --all`, and
  `rg -n "CANARY|NOLEAK_CANARY|ARTIFICIAL_SECRET" games/frontier_control/tests games/frontier_control/src`
  with no matches.
