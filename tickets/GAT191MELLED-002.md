# GAT191MELLED-002: `advance_to_next_round` transition + per-round deal seed

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/meldfall_ledger` (`src/rules.rs`, `src/setup.rs`); tests `tests/rules.rs`
**Deps**: GAT191MELLED-001

## Problem

`ML-MATCH-006` requires that a non-terminal settled round deterministically deals
the next round — rotate dealer clockwise, clear round-only table state, deal a
fresh round, and start with the seat left of the new dealer. No code performs
this today: `rules.rs` only drives the round *into* `RoundSettled`
(`settle_round_stock_exhausted` rules.rs:358, `round.phase = TurnPhase::RoundSettled`
rules.rs:491) and the match dead-ends. This ticket implements the Rust-owned
transition operation and the deterministic per-round deal-seed derivation it needs.

## Assumption Reassessment (2026-06-26)

1. `advance_to_next_round` does not exist. The reusable pieces do: `deal_for_round`
   (`games/meldfall_ledger/src/setup.rs:101`, signature `(seed, dealer_index,
   seat_count, hand_size) -> Result<RoundDeal, Diagnostic>`), `next_clockwise_index`
   (`games/meldfall_ledger/src/ids.rs:43`, returns `Option<usize>`), and
   `settle_round` (`games/meldfall_ledger/src/scoring.rs:64`). The active-seat rule
   "left of dealer" already appears at `setup.rs:75`. Reuse all as-is.
2. The reset/preserve contract is spec §3.4. The round-only fields to reset are
   exactly `RoundState`'s `{active_seat_index, phase, stock, discard, tableau,
   pending_pickup, round_played_scores, seats, round_end}` (`state.rs:84-95`);
   `cumulative_scores` is preserved (it lives on `MatchState`, untouched by the
   re-deal). `RoundState::from_initial_setup` (state.rs:97-115) is the precedent
   for a freshly-dealt round's field values.
3. Cross-artifact boundary under audit: the transition mutates `MatchState`
   (rotates `dealer_index`, advances the GAT191MELLED-001 round counter, preserves
   `cumulative_scores`) and replaces its `RoundState` wholesale. The shared contract
   under audit is the §3.4 reset/preserve matrix — every row must be asserted.
4. FOUNDATIONS §2 behavior authority + §11 determinism restated: the transition,
   dealer rotation, and re-deal are Rust-owned and deterministic. No TypeScript,
   wall-clock, or browser RNG participates. The re-deal seed derives only from the
   base match seed (GAT191MELLED-001) and the round index.
5. §11 deterministic-RNG surface: the per-round seed derivation uses only the
   existing `engine_core` seed primitives (`Seed(u64)`, `SeededRng::from_seed`) —
   no seed-derivation helper exists in `engine-core` and none is added. Round 0
   keeps the existing `setup_match`/`deal_for_round(seed)` path byte-identically;
   the derivation governs rounds 1+ only, so existing round-0 deals/traces stay
   stable. The re-deal produces a fresh hidden stock + private hands; the no-leak
   property across the transition is exercised at the export layer in
   GAT191MELLED-004, and this op introduces no nondeterministic input into canonical
   forms.

## Architecture Check

1. A single Rust-owned `advance_to_next_round` that reuses `deal_for_round` and
   `next_clockwise_index` is cleaner than re-implementing dealing or rotation — it
   keeps one deal code path shared by round 0 (`setup_match`) and rounds 1+.
2. No backwards-compatibility shim: round 0 continues to use the existing setup
   path unchanged; the new op is additive and governs only subsequent rounds.
3. `engine-core` is untouched; the op reuses existing game-crate helpers; no
   `game-stdlib` promotion — multi-round cumulative scoring is the declared
   first-use local-only primitive (spec §1 Primitive stance), not a third use.

## Verification Layers

1. Reset/preserve matrix (every §3.4 row) -> one unit assertion per row in `tests/rules.rs`.
2. Dealer rotation across 2..6 seats + active seat left of the new dealer -> unit test.
3. Deterministic re-deal: same base seed ⇒ identical multi-round state/score sequence -> unit test running the sequence twice and comparing `stable_internal_summary`.
4. Card conservation: 52 cards accounted for every round -> unit test.
5. Meld-id policy (reset-per-round vs monotonic-per-match) -> explicit assertion test (spec §3.4 last row).

## What to Change

### 1. Per-round deal-seed derivation (`setup.rs`)

Add a pure function deriving the round deal seed from `(base_seed, round_index)`
using `Seed` / `SeededRng::from_seed` only. It MUST be replay-stable and
documented; round 0 does not call it (it keeps `setup_match`'s direct
`deal_for_round(seed)`).

### 2. `advance_to_next_round` (`rules.rs`)

When the just-settled round is non-terminal, in order: increment the round
counter; rotate `dealer_index` via `next_clockwise_index`; derive the round seed
(§1); deal via `deal_for_round`; build the new `RoundState` resetting every §3.4
round-only field and preserving `cumulative_scores`; set the active seat to the
seat left of the new dealer and `phase` to `Draw`.

### 3. Meld-id policy

Decide and document whether `MeldId` resets per round or stays monotonic per match
(pick the option that keeps trace hashes and the tableau view unambiguous), and
assert it in a test.

### 4. Transition unit tests (`tests/rules.rs`)

Author the §7.2 matrix listed in Verification Layers.

## Files to Touch

- `games/meldfall_ledger/src/rules.rs` (modify) — `advance_to_next_round`
- `games/meldfall_ledger/src/setup.rs` (modify) — per-round deal-seed derivation
- `games/meldfall_ledger/tests/rules.rs` (modify) — transition unit tests

## Out of Scope

- Apply-path wiring that *calls* `advance_to_next_round` in either host — GAT191MELLED-004.
- The `NextRoundDealt` effect and the `round_score_index` fix — GAT191MELLED-003.
- Web feedback (005); golden trace / docs / closeout (006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger rules` — the new transition unit tests pass.
2. `cargo test -p meldfall_ledger` — full crate green (existing single-round tests unaffected).
3. `cargo test --workspace` — no regression.

### Invariants

1. The transition is deterministic and Rust-owned; identical `(base seed, round
   index)` produce an identical re-deal (stock order, hands, discard).
2. `cumulative_scores` is preserved; every §3.4 round-only field is reset; dealer
   rotates clockwise; the active seat is left of the new dealer; `phase` is `Draw`.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — reset/preserve matrix (every §3.4 row),
   dealer rotation across 2..6 seats, active-seat-left-of-dealer, deterministic
   re-deal (same seed twice), 52-card conservation per round, and the meld-id
   policy assertion.

### Commands

1. `cargo test -p meldfall_ledger rules`
2. `cargo test --workspace`
3. The crate-scoped `rules` filter is the correct boundary for the transition
   logic; `cargo test --workspace` is the full regression guard.
