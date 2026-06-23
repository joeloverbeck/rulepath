# UNI8CR2TWOSEA-024: High Card Duel — C-07 no-leak pilot receipt verification

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/tests/visibility.rs`, `games/high_card_duel/tests/bots.rs`; residual no-leak verification, no pilot reconstruction
**Deps**: 001

## Problem

Spec §3.8 / task `8C-R2-510`: High Card Duel's C-07 pairwise no-leak geometry is `already-discharged-by-8C-pilot`. R2 **verifies** the pilot receipt (`MSC-8C-007` + the existing `no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces` test) and adds only residual profile/effect/seat/count/tree/RNG checks — it does not rebuild the pilot matrix.

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/tests/visibility.rs` contains `no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces` (confirmed ~line 264); `tests/bots.rs` exists.
2. Spec §3.8/§9: verify, do not rebuild; the C-04 surface is independently migrated (`-017`); retain all reveal-specific tests.
3. Cross-crate boundary under audit: `game-test-support::no_leak` (`crates/game-test-support/src/no_leak.rs`, `ExposureExpectation`) — the harness enumerates caller-supplied cases only; HCD already has the dev-dependency.
4. §11 no-leak firewall: the unrevealed private card stays absent for observer/opponent across view/action/diagnostic/effect/public-export/bot surfaces and present only for the owner/acting seat; canaries are constructed in memory and never committed (R2-EC-20).

## Architecture Check

1. Verifying the receipt rather than re-deriving the matrix keeps the pilot authoritative and avoids duplicated geometry — the spec's residual-only contract.
2. No backwards-compat alias; existing reveal-specific tests are retained, not replaced by a generic matrix.
3. `engine-core` untouched; the harness is dev-only `game-test-support`.

## Verification Layers

1. Pilot receipt holds (observer/seat0/seat1 × view/action/diagnostic/effect/public-export/bot input) -> no-leak visibility test (`tests/visibility.rs`) + bot legality check (`tests/bots.rs`).
2. Residual seat/count/tree/RNG surfaces leak-free -> no-leak visibility test + deterministic replay-hash check (`replay-check --game high_card_duel --all`).
3. Canaries never committed -> codebase grep-proof (no canary string in any trace/fixture/snapshot/test ID).

## What to Change

### 1. Verify the pilot receipt

Assert the existing pilot matrix and `MSC-8C-007` coverage still hold; add residual checks for the profile/effect/seat/count/tree/RNG surfaces this wave introduces.

## Files to Touch

- `games/high_card_duel/tests/visibility.rs` (modify)
- `games/high_card_duel/tests/bots.rs` (modify)

## Out of Scope

- Rebuilding or replacing the C-07 pilot matrix; the C-04 v1 surface (`-017`).
- Any committed canary; any golden-trace or fixture change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including the retained pilot/reveal-specific tests and the residual checks.
2. `cargo run -p replay-check -- --game high_card_duel --all` — no byte change.

### Invariants

1. The unrevealed private card never reaches observer/opponent surfaces.
2. No canary appears in any committed trace, fixture, snapshot, log, or test ID.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs` — residual profile/effect/seat/count/tree/RNG no-leak assertions atop the retained pilot test.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`

## Outcome

Completed on 2026-06-23.

Verified the existing C-07 pilot receipt by retaining
`no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces`, then added
a residual `visibility.rs` check for the post-lead-commit pre-reveal state.
The new check covers public count/profile metadata, owner/opponent projected
views, filtered effects, reply legal action tree debug/v1 bytes, reply bot
input, and deterministic reply bot decision output. It uses real in-memory
cards from seeded setup, adds no committed canary, and does not rebuild the
pilot matrix or change any golden trace/fixture bytes. `tests/bots.rs` already
covered the relevant bot input/decision boundary, so no edit there was needed.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p high_card_duel`
3. `cargo run -p replay-check -- --game high_card_duel --all`
