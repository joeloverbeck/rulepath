# GAT101PLATRI-007: Rules transition engine

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/plain_tricks/src/rules.rs`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-006

## Problem

The game needs the Rust-owned transition engine: applying a card play, resolving the trick (highest led-suit rank wins; off-suit never wins), trick-winner-leads turn order, round close and scoring (1 point/trick), deal rotation into round 2, terminal outcome (TrickWin / Split on 6–6), and freshness-token increments. This is the behavioral core proving state-dependent trick-taking rules without engine-core nouns.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{state,actions}.rs` exist (GAT101PLATRI-005/006) supplying the validated command, current trick, hands, and led suit. Mirror `games/poker_lite/src/rules.rs` for the transition/terminal shape.
2. Spec §5 item 6 and appendix A4 fix resolution: leader sets the led suit; follower plays under follow-suit; higher led-suit rank wins; off-suit play means the leader wins; no within-trick tie (single-copy deck); after trick 6 emit round score; after round 2 resolve terminal (higher total wins; 6–6 is Split). Trick winner leads next.
3. Shared boundary under audit: the freshness-token / terminal-outcome contract from `engine-core` (consumed) and the internal state mutation surface from GAT101PLATRI-005. Effects are emitted in GAT101PLATRI-008; this ticket owns the state machine.
4. FOUNDATIONS §2 (Rust owns state transitions, scoring, terminal detection) is under audit — none of this may move to TypeScript.
5. Enforcement surface: deterministic replay/hash (§11/§13). Each apply must be a pure function of (state, validated command) with a deterministic freshness increment, so identical command streams reproduce identical state/hashes (golden traces land in GAT101PLATRI-011). No hidden-information path is introduced — transitions mutate internal state only; projection is separate.

## Architecture Check

1. A pure (state, command) → state transition engine (vs. interleaving effects/views) keeps behavior authority crisp and replay deterministic; effects/views consume the resulting state.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; trick/lead/scoring nouns are `plain_tricks`-local. No `game-stdlib` change.

## Verification Layers

1. Trick resolution (highest led-suit rank; off-suit never wins; no tie) -> rule unit tests + golden traces `off-suit-never-wins`, `follow-suit-forced`.
2. Trick-winner-leads, round close/scoring, deal rotation, terminal/Split -> rule unit tests + golden traces `trick-winner-leads-next`, `round-close-deal-rotation`, `terminal-most-points-win`, `tie-split`.
3. Determinism (apply is pure; freshness increments deterministic) -> deterministic replay-hash check (GAT101PLATRI-011).
4. Bounded termination (exactly 24 plays; totals sum to 12) -> property tests (GAT101PLATRI-010).

## What to Change

### 1. `games/plain_tricks/src/rules.rs`

Implement `apply_action`: card-play transition, led-suit assignment, trick resolution (A4), trick-winner-leads, round close + per-seat scoring, deal rotation into round 2 (from the continuing RNG stream via setup), terminal resolution (TrickWin{winner,points} / Split{each}), and freshness-token increments. The terminal rationale records per-round and total trick counts (consumed by the outcome-explanation surface).

## Files to Touch

- `games/plain_tricks/src/rules.rs` (new)

## Out of Scope

- Semantic effects emission (GAT101PLATRI-008).
- View projection / no-leak tests (GAT101PLATRI-009).
- Property tests (GAT101PLATRI-010) and golden traces (GAT101PLATRI-011) — this ticket provides the engine they exercise.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks --test rules`: trick resolution, off-suit-never-wins, trick-winner-leads, round close/scoring, deal rotation, terminal win, Split.
2. A full match driven by validated commands terminates in exactly 24 plays with per-seat trick totals summing to 12.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Off-suit cards never win a trick; the trick winner is always one of the two played seats (FOUNDATIONS §2; spec property test).
2. `apply_action` is a deterministic pure function of (state, validated command) (FOUNDATIONS §2/§11).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/rules.rs` — trick resolution + round close + rotation + terminal, citing `RULES.md` rule IDs.
2. Within-crate match driver test — terminates at 24 plays; totals sum to 12.

### Commands

1. `cargo test -p plain_tricks --test rules`
2. `cargo test -p plain_tricks && bash scripts/boundary-check.sh`
3. Per-crate rule scope is correct; deterministic cross-tool replay is proven in GAT101PLATRI-011.

## Outcome

Completed: 2026-06-09

What changed:

- Added `games/plain_tricks/src/rules.rs` with Rust-owned `apply_action`, trick resolution, trick-winner-led turn order, round close, second-round deal rotation, terminal win, and Split resolution.
- Added `trick_winner` where the highest led-suit rank wins and off-suit cards never win.
- Stored the continuing internal `SeededRng` in `PlainTricksState` so round 2 is dealt from the same deterministic RNG stream after round 1 closes.
- Updated setup/state/action test construction for the internal RNG state and exported rules helpers from `lib.rs`.

Deviations from original plan:

- Tests are currently crate unit tests rather than `games/plain_tricks/tests/rules.rs`; this keeps the pre-effects/pre-replay surface compact. Integration/golden trace coverage remains scheduled for later tickets.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p plain_tricks` passed: 28 tests passed.
- `bash scripts/boundary-check.sh` passed.
- A full match driver test terminated in exactly 24 validated plays, with total trick points summing to 12.
