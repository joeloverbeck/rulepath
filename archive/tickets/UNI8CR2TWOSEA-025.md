# UNI8CR2TWOSEA-025: Secret Draft — C-07 pairwise no-leak geometry

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/tests/{visibility,bots,replay}.rs`; adopts `game_test_support::no_leak` pairwise matrix
**Deps**: 021

## Problem

Spec §3.8 / task `8C-R2-511`: add the pairwise no-leak matrix for Secret Draft — both source seats × observer/owner/opponent × view/action/diagnostic/effect/public-export/seat-private-export/bot surfaces, before and after the synchronized reveal. Simultaneous commitment/reveal remains game-local; the internal command trace is a distinct `internal-dev` authority tested separately. Existing focused reveal tests are retained.

## Assumption Reassessment (2026-06-23)

1. `games/secret_draft/tests/{visibility,bots,replay}.rs` exist; the committed item is absent pre-reveal across all viewer surfaces (spec §3.8 matrix).
2. Spec §3.8/§9: the harness may not decide when choices reveal or how the visible pool changes; retain focused reveal tests; do not subsume them with the generic matrix.
3. Cross-crate boundary under audit: `game-test-support::no_leak` (`ExposureExpectation{MustBeAbsent,MustBePresent,NotApplicable}`) — the harness only enumerates caller-supplied cases; needs the `-021` dev-dependency.
4. §11 no-leak firewall: the committed item stays absent for observer/owner/opponent on every viewer surface pre-reveal and present only after the game-authorized simultaneous reveal; the internal command trace is `internal-dev`, not a viewer surface; canaries are in-memory only (R2-EC-20).

## Architecture Check

1. A caller-driven pairwise matrix proves redaction for both source seats without moving reveal policy into shared code — the geometry-only harness contract.
2. No backwards-compat alias; the generic matrix is added alongside, not over, the focused reveal tests.
3. `engine-core` untouched; the harness is dev-only `game-test-support`.

## Verification Layers

1. Both seats × 3 viewers × all surfaces, pre/post reveal -> no-leak visibility test (`tests/visibility.rs`, `tests/replay.rs`).
2. Bot input/explanation leak-free -> bot legality check (`tests/bots.rs`).
3. Internal command trace remains internal-dev (not a viewer surface) -> codebase grep-proof + `replay-check --game secret_draft --all`.

## What to Change

### 1. Add the pairwise no-leak matrix

Using `game_test_support::no_leak`, enumerate source seat × viewer × surface cases for Secret Draft, pre- and post-synchronized-reveal, with in-memory canaries; retain the existing focused reveal tests.

## Files to Touch

- `games/secret_draft/tests/visibility.rs` (modify)
- `games/secret_draft/tests/bots.rs` (modify)
- `games/secret_draft/tests/replay.rs` (modify)

## Out of Scope

- Moving commitment/reveal or visible-pool policy into shared code.
- Any committed canary; any golden-trace or fixture change; the C-08 profile drivers.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, including the new matrix and the retained focused reveal tests.
2. `cargo run -p replay-check -- --game secret_draft --all` — no byte change.

### Invariants

1. The committed item is absent for all three viewers pre-reveal and present only post-authorized-reveal.
2. No canary appears in any committed trace, fixture, snapshot, log, or test ID.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/visibility.rs` — pairwise source × viewer × surface no-leak matrix (pre/post reveal).

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`

## Outcome

Completed on 2026-06-23.

Added a `game_test_support::no_leak` pairwise matrix in
`games/secret_draft/tests/visibility.rs`. The matrix covers both source seats
across observer, seat 0, and seat 1 viewers for pre-reveal commitment/private
view fields, action metadata, diagnostics, effects, public export,
seat-private export, and bot rationale, plus post-synchronized-reveal view,
effect, public export, and seat-private export surfaces. The pre-reveal
snapshots intentionally check commitment relation surfaces rather than raw
visible-pool/action-choice item ids, because Secret Draft keeps the visible
pool public until the synchronized reveal. Existing focused bot and replay
tests stayed green, so no redundant edits were needed there.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p secret_draft`
3. `cargo run -p replay-check -- --game secret_draft --all`
