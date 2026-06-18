# RIVLEDSHOSEA-003: Single canonical resolved-showdown assembly with invariant checks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/showdown.rs` (and `src/state.rs` only if an internal carrier must change)
**Deps**: RIVLEDSHOSEA-001, RIVLEDSHOSEA-002

## Problem

After RIVLEDSHOSEA-001/002 fix order and label semantics, `resolve_showdown()` still derives downstream fields from partly-transformed structures: `headline`/`decisive_comparison`/`comparison_basis` from the canonical winner vector, but `TerminalOutcome::Showdown.winners`/`allocations` and `explain_showdown`/`showdown_presentation_v2` from `&allocation`. Multiple builders each choose a winner source independently, which is how the contradiction arose. This ticket consolidates derivation into one internal resolved-showdown value and adds construction-time invariant assertions so the surfaces can never disagree again (spec §7.2 / D2).

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/src/showdown.rs::resolve_showdown(state) -> TerminalOutcome` currently composes `winning_seats`, `allocate_single_pot`, `explain_showdown`, `showdown_headline`, `decisive_comparison`, `comparison_basis`, and `showdown_presentation_v2` separately and assembles `TerminalOutcome::Showdown { .. }`. `primary_winner(evaluations, winners) -> Option<&SeatEvaluation>` reads the first element of the winners slice it receives. Confirmed.
2. `src/state.rs` defines `TerminalOutcome::Showdown { winners, pot_total, allocations, headline, decisive_comparison, comparison_basis, explanations, presentation_v2 }`, `RiverLedgerShowdownPresentationV2 { result_banner, .. }`, and `ShowdownResultBanner { headline, subheadline, accessibility_label }`. No `ResolvedShowdown` type exists yet — it is a new private assembly type (spec D2 names it as recommended, not pre-existing). Confirmed.
3. Shared boundary under audit: the single internal resolved-showdown value and every field derived from it (terminal outcome, V2 banner, decisive reason, standings, explanations, serialized winners/allocations). End state: all downstream fields read from that one value; no builder recomputes winner meaning.
4. FOUNDATIONS §11 (deterministic, conservation-respecting, non-leaking outcomes): the assembly must keep the serialized winner set canonical and unique, allocations conserved, and reveal authorization unchanged. Restated before trusting the spec.
5. Determinism/no-leak surface: this is a pure internal restructuring of how `TerminalOutcome::Showdown` is built; serialized field *values* match RIVLEDSHOSEA-001/002 semantics, and showdown reveal authorization (`RL-VIS-SHOWDOWN-001`) is untouched — no folded/non-revealed private card enters any field. The added assertions are debug/construction-time guards, not new serialized data.
6. `RiverLedgerShowdownPresentationV2` / `TerminalOutcome::Showdown` serialized shapes are not extended (no new public field); only the internal derivation path changes. If a private carrier (`ResolvedShowdown`) is introduced it stays module-internal and unserialized.

## Architecture Check

1. One assembled value with downstream fields derived from it removes the "which winner source did this builder use" failure mode structurally; assertions turn a silent divergence into a construction-time panic in tests. Cleaner than auditing each builder forever.
2. No shim: `primary_winner()` is removed or narrowed for ties (a split headline names all canonical winners); no alias keeps the old per-builder paths alive.
3. `engine-core` untouched; the assembly type, if added, is a private `games/river_ledger` type — no kernel noun, no `game-stdlib` promotion.

## Verification Layers

1. Single winner source -> code review + `tests/rules.rs` asserting `TerminalOutcome.winners`, V2 banner winner flags, decisive reason, standings, and awarded shares agree for unique, even-split, and remainder-split outcomes.
2. Construction invariants hold -> `tests/rules.rs` cases exercising the assertions: canonical winners nonempty/unique; each winner exactly one positive allocation; no non-winner positive; allocations sum to `pot_total`; `remainder_order` is exactly the winner set; V2 standings flags match canonical winners; single-winner banner identity matches the sole `TerminalOutcome.winners` ID; split banner names every winner and no loser.
3. No-leak preserved -> `tests/visibility.rs` confirming folded/non-revealed private cards remain absent from the assembled outcome (re-run against the seed-`10018`/seed-`31` states).

## What to Change

### 1. `showdown.rs` — consolidate assembly

Construct one internal resolved value (recommended `ResolvedShowdown`) holding ordered evaluations, canonical semantic winner IDs, per-winner allocation amounts, explicit remainder order/recipients, canonical Rust-authored winner labels, decisive comparison + basis, explanations, and the V2 presentation. Build `TerminalOutcome::Showdown`, the banner, decisive reason, standings, explanations, and allocations from that value.

### 2. `showdown.rs` — narrow `primary_winner` and add assertions

Remove or narrow `primary_winner()` so a tied result never presents one seat as the sole/primary winner; a split headline names all canonical winners and a shared equal-hand description does not imply a single true winner. Add the construction-time/debug assertions enumerated in Verification Layers.

## Files to Touch

- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/src/state.rs` (modify; only if the internal carrier requires it)
- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)

## Out of Scope

- Pot/label order semantics (RIVLEDSHOSEA-001/002 — depended on, not re-done).
- Golden-trace/replay/serialization reconciliation (RIVLEDSHOSEA-004).
- Any WASM/TypeScript change.
- Adding a new serialized public field or a V3 presentation.

## Acceptance Criteria

### Tests That Must Pass

1. Agreement test: `TerminalOutcome.winners`, V2 banner, decisive reason, standings, accessibility announcement, and awarded shares name the same winner identity for unique, even-split, and remainder-split outcomes.
2. Invariant tests exercising every construction-time assertion (Verification Layers item 2).
3. `cargo test -p river_ledger` green; `tests/visibility.rs` no-leak cases pass.

### Invariants

1. Every showdown surface derives its winner set from one internal resolved value; no builder recomputes winner meaning.
2. No tied result is narrated, listed, or labeled as having a single/primary winner.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — cross-surface agreement + invariant-assertion cases (seeds `10018`, `31`, plus an even-split seed).
2. `games/river_ledger/tests/visibility.rs` — folded/non-revealed cards absent from the assembled outcome.

### Commands

1. `cargo test -p river_ledger`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace`
3. Golden-trace/replay verification is RIVLEDSHOSEA-004's boundary (it regenerates the traces these outcomes feed), so it is not gated here.
