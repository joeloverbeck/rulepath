# GAT15RIVLEDTEX-004: Setup — 3–6-seat validation, deterministic shuffle/deal, blinds and button

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/setup.rs`, `tests/rules.rs`, `src/lib.rs`; setup fixtures + setup golden traces
**Deps**: GAT15RIVLEDTEX-003

## Problem

River Ledger needs deterministic setup for exactly 3–6 seats: a standard 52-card deck, deterministic shuffle from engine RNG, button/SB/BB assignment from stable seat order, forced blind contributions, two private hole cards per seat, reserved community deck state, and the initial preflop active seat — with deterministic diagnostics rejecting out-of-range seat counts.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/setup.rs` is the precedent for deterministic deal from `Seed` + the `tests/rules.rs` integration-test layout; this ticket creates `games/river_ledger/tests/rules.rs` (extended by 005/007).
2. `specs/...-base.md` §4.1 (`setup.rs`), §6 exit row 1 (accept/reject seat range deterministically), and §5 G15-RL-003 fix the rule coverage `RL-SETUP-*`, `RL-DEAL-*`, `RL-BET-BLINDS-*`, `RL-VIS-PRIVATE-HOLE-*`.
3. Cross-artifact boundary under audit: setup consumes the `state.rs`/`cards.rs`/`ids.rs` types from 003 and produces the initial state record that visibility (008) projects; the setup fixtures land in `data/fixtures/` (shared additive dir).
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: shuffle, deal, blind/button assignment, seat-range validation, and the initial active seat are Rust-owned; TypeScript never derives them.
5. Determinism + no-leak (§11) under audit: identical `(seed, seats, setup)` serializes identically; different seeds vary the shuffle; the public observer projection must expose no hole cards, deck order, deck tail, or burn placeholders (full no-leak proof in 009/010). Seat-count rejection is fail-closed with a stable diagnostic.

## Architecture Check

1. A pure `setup(seed, seats, setup) -> state | diagnostic` keeps deal determinism and golden traces reproducible and the no-leak firewall a projection concern, matching the sibling setup pattern.
2. No backwards-compatibility aliasing/shims — new module extending in-batch stubs.
3. `engine-core` stays noun-free (§3); deck/deal/blind/button are crate-local; no `game-stdlib` promotion (§4).

## Verification Layers

1. Seat-count accept (3–6) / reject (0/1/2/7+) with stable diagnostics -> `cargo test -p river_ledger --test rules` seat-validation tests.
2. Deterministic deal (same seed+seats → identical state; different seed → different shuffle) -> determinism unit tests.
3. Public observer cannot see hole/deck/burn at setup -> projection assertion (full pairwise proof deferred to GAT15RIVLEDTEX-009).
4. Setup golden traces for 3/4/5/6 + invalid-seat-count -> `cargo run -p replay-check -- --game river_ledger` (cross-cutting validation wired in GAT15RIVLEDTEX-015; behavior is tested here by the rule tests).

## What to Change

### 1. `games/river_ledger/src/setup.rs`

Seat-range validation (accept 3–6, reject else with deterministic diagnostic), deterministic shuffle/deal from engine RNG, button/SB/BB from stable `SeatId` order, forced small/big blind contributions, two hole cards per seat, reserved community deck state, initial preflop active seat, fixture setup options.

### 2. Tests + fixtures + traces

Create `tests/rules.rs` with seat-validation, deterministic-deal, and different-seed-variance tests; add `data/fixtures/river_ledger_{3,4,5,6}p_standard.fixture.json`; add `tests/golden_traces/setup-{3,4,5,6}p.trace.json` and `invalid-seat-count.trace.json`.

## Files to Touch

- `games/river_ledger/src/setup.rs` (new)
- `games/river_ledger/tests/rules.rs` (new)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json` (new)
- `games/river_ledger/data/fixtures/river_ledger_4p_standard.fixture.json` (new)
- `games/river_ledger/data/fixtures/river_ledger_5p_standard.fixture.json` (new)
- `games/river_ledger/data/fixtures/river_ledger_6p_standard.fixture.json` (new)
- `games/river_ledger/tests/golden_traces/setup-3p.trace.json` (new)
- `games/river_ledger/tests/golden_traces/setup-4p.trace.json` (new)
- `games/river_ledger/tests/golden_traces/setup-5p.trace.json` (new)
- `games/river_ledger/tests/golden_traces/setup-6p.trace.json` (new)
- `games/river_ledger/tests/golden_traces/invalid-seat-count.trace.json` (new)

## Out of Scope

- Betting/street resolution beyond initial blind state (GAT15RIVLEDTEX-005).
- Any all-in/side-pot state (deferred to Gate 15.1).
- Visibility projection machinery and pairwise no-leak proof (008/009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test rules` — accepted (3–6) and rejected (0/1/2/7) seat counts; deterministic deal; shuffle variance.
2. Same `(seed, seats, setup)` produces byte-identical serialized setup state.
3. `bash scripts/boundary-check.sh` — no mechanic noun reaches `engine-core`.

### Invariants

1. Setup/shuffle/deal/blinds/button are Rust-owned and deterministic (§2/§11).
2. The public observer projection at setup exposes no hidden card/deck facts (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` (new) — seat validation, deterministic deal, shuffle variance, initial active seat.
2. `games/river_ledger/tests/golden_traces/setup-{3,4,5,6}p.trace.json` + `invalid-seat-count.trace.json` (new) — setup determinism evidence.

### Commands

1. `cargo test -p river_ledger --test rules`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. Golden-trace replay validation runs via `cargo run -p replay-check -- --game river_ledger` once the tool is registered (GAT15RIVLEDTEX-015); behavior here is proven by the rule tests.

## Outcome

Completed: 2026-06-14

Implemented deterministic River Ledger setup:

- Added `games/river_ledger/src/setup.rs` with 3-6 seat validation, deterministic
  seeded shuffle, two-card private deal, reserved five-card community deck,
  deck-tail retention, button/SB/BB assignment, forced blind contributions, and
  initial preflop active seat.
- Extended `RiverLedgerState` with internal private-hand/community/deck-tail
  storage, stable internal serialization text for determinism checks, and a
  public setup summary that exposes counts but no hidden card identities.
- Added `games/river_ledger/tests/rules.rs` covering seat-count acceptance and
  rejection, same-seed determinism, different-seed shuffle variance, initial
  ledger/roles/active seat, unique 52-card deal accounting, and setup public
  no-leak summary.
- Added setup fixtures for 3, 4, 5, and 6 seats plus setup/invalid-seat-count
  golden trace placeholders for later replay-check registration.

Deviations: replay-check validation for the new golden traces is intentionally
deferred until `river_ledger` is registered with replay-check in
GAT15RIVLEDTEX-015, matching the ticket note. No betting/street resolution past
initial blind state, visibility module, effects, replay export, WASM, or web
registration was implemented.

Verification:

- `cargo test -p river_ledger --test rules` passed (6 tests).
- `cargo test -p river_ledger` passed (7 unit tests, 6 integration tests).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `cargo fmt --all --check` passed.

Unrelated worktree changes left untouched: `.claude/skills/spec-to-tickets/SKILL.md`.
