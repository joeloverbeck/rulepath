# RIVLEDSHO-001: Rust-authored showdown explanation builder + additive terminal view fields

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/showdown.rs` (+ optional `games/river_ledger/src/display.rs`), `games/river_ledger/src/state.rs`, golden traces, serialization tests
**Deps**: None

## Problem

At showdown River Ledger emits only machine facts: `ShowdownReveal` carries a raw `category` enum string (`"one_pair"`), a `tie_break_vector`, and `best_five` (`games/river_ledger/src/state.rs:93`), and `ShowdownSeatExplanation.summary` is a developer string (`"… tie_break={:?}; allocated=…"`, `showdown.rs:113-139`). The decisive "Pair of Queens beats Pair of Eights" comparison is never authored. This ticket builds a deterministic, Rust-authored human-readable explanation layer so the UI never has to decode rank integers or infer winners (spec WB1 / D1).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `ShowdownReveal` (`state.rs:93`) and `ShowdownSeatExplanation` (`state.rs:102`) carry category/tie-break/best-five/summary; `showdown.rs:142-151` builds `ShowdownReveal` from `evaluator::best_five_from_seven` (`category`, `tie_break_vector`, `used_cards`). The evaluator stays the source of category/tie-break/best-five facts; this ticket only consumes its output.
2. Verified against specs/docs: spec §6 D1 + §8 WB1; `games/river_ledger/docs/RULES.md` `RL-UI-SHOWDOWN-001` (Rust-authored outcome), `RL-EVAL-TIEBREAK-001` (category then rank vector), `RL-EVAL-USED-001` (used-card explanation).
3. Cross-artifact boundary under audit: the terminal projection (`TerminalOutcome::Showdown` in `state.rs`) and the showdown builder (`showdown.rs`). New fields are **additive** to the existing terminal shape — existing `category_key`/`tie_break_vector`/`best_five` are retained for the debug/details tier.
4. FOUNDATIONS §2 behavior authority motivates this ticket: hand names, category labels, and the decisive comparison are Rust-authored view data, never synthesized in TypeScript by re-interpreting the tie-break vector.
5. §11 determinism + no-leak firewall is the enforcement surface: the builder runs only over already-authorized showdown results (reveal-scoping is RIVLEDSHO-002), new serialized fields are deterministic, and the change rides the ordinary golden-trace/serialization migration path; reveal-scoping that withholds folded-seat explanations is deferred to RIVLEDSHO-002.
6. Extends the terminal-projection schema (additive-only: new optional/defaulted fields); consumers are the WASM bridge (RIVLEDSHO-003) and golden-trace/serialization fixtures, updated here.

## Architecture Check

1. A deterministic explanation builder co-located in `games/river_ledger` (an internal `display.rs` section or a `showdown.rs` helper) keeps label logic in the game crate, not `engine-core` and not TypeScript — the only boundary-correct home (§2/§3).
2. No backwards-compatibility aliasing/shims; raw fields are retained additively, not replaced.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — game-local view types only.

## Verification Layers

1. Worked example (`4C 3D QH 6H 8H`, Seat 4 `QC QH 10S 8H 6H` vs Seat 0 `QH 10C 8D 8H 6H`) yields headline "Seat 4 wins with Pair of Queens." + decisive "Pair of Queens beats Pair of Eights." -> `games/river_ledger/tests/rules.rs` unit test.
2. Split and foldout paths produce correct `result_label`/`hand_name`/`comparison_note` -> `rules.rs` showdown/split/foldout tests.
3. New serialized fields are deterministic and additive (existing consumers unbroken) -> `games/river_ledger/tests/serialization.rs` + `cargo run -p replay-check -- --game river_ledger` + `cargo run -p fixture-check -- --game river_ledger`.

## What to Change

### 1. Explanation builder

Add a deterministic builder (in `showdown.rs` or a new `display.rs`) that derives, from already-evaluated showdown results: `headline`, `decisive_comparison`, `comparison_basis`; and per revealed seat `result_label`, `hand_name`, `rank_explanation`, `comparison_note`, `best_five_accessibility_label`. Hand names cover every category (high card → straight flush).

### 2. Additive terminal view fields

Add these as additive fields on the terminal projection / `ShowdownReveal`-adjacent view model in `state.rs`, retaining raw `category_key`/`tie_break_vector`/`best_five` for the details tier.

### 3. Tests + fixtures

Unit tests for the worked example, split, foldout, and per-category hand names; update golden traces and serialization fixtures where the terminal view JSON gains fields.

## Files to Touch

- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/src/display.rs` (new; optional — fold into `showdown.rs` if leaner)
- `games/river_ledger/src/state.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)
- `games/river_ledger/tests/golden_traces/*.trace.json` (modify — showdown/split/foldout traces where terminal JSON gains fields)

## Out of Scope

- Reveal-scoped projection + folded-seat withholding (RIVLEDSHO-002).
- WASM bridge and TypeScript types (RIVLEDSHO-003).
- Any panel rendering (RIVLEDSHO-004).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test rules` — worked-example headline/decisive/basis, per-category hand names, split, foldout.
2. `cargo test -p river_ledger` — full crate green, including serialization fixtures.
3. `cargo run -p replay-check -- --game river_ledger` and `cargo run -p fixture-check -- --game river_ledger` — additive change leaves replay/fixtures green.

### Invariants

1. Explanation strings are Rust-authored and deterministic for identical inputs+versions (§2/§11).
2. New fields are additive-only; raw `category_key`/`tie_break_vector`/`best_five` are retained (no breaking reshape).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — explanation-builder unit tests (worked example, every category, split, foldout).
2. `games/river_ledger/tests/serialization.rs` — terminal view JSON with the additive fields.
3. `games/river_ledger/tests/golden_traces/*.trace.json` — showdown/split/foldout trace updates.

### Commands

1. `cargo test -p river_ledger --test rules`
2. `cargo test -p river_ledger`
3. `cargo run -p replay-check -- --game river_ledger` — confirms the additive serialization change does not break deterministic replay.
