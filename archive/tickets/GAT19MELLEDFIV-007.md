# GAT19MELLEDFIV-007: Public meld tableau model and score-credit ownership (first-use primitive)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{state,rules}.rs`; tableau golden traces; first-use ledger entry ML-PP-002
**Deps**: GAT19MELLEDFIV-006

## Problem

When a seat tables a legal meld it becomes public. Meldfall Ledger needs the public meld tableau: melds grouped by originating meld with stable ids, an `origin_seat` (who opened the group) tracked separately from per-card `played_by` score-credit ownership, and a public projection of tableau cards. This is the first official use of a public meld tableau / zone model (`ML-PP-002`, `local-only`).

## Assumption Reassessment (2026-06-25)

1. No existing game has a public meld tableau (confirmed during reassessment). `MeldGroup`/`TableCard` shapes (`origin_seat`, `played_by`, `play_turn`) were stubbed in `state.rs` (GAT19MELLEDFIV-005); meld legality exists from GAT19MELLEDFIV-006.
2. Spec §3.1 (Laying off / Visibility rows), Appendix B.1 (`MeldGroup`/`TableCard`), Appendix B.3 (score-credit model), and Appendix D (`ML-PP-002`) define the model.
3. Cross-artifact: the public-view contract (`docs/ENGINE-GAME-DATA-BOUNDARY.md`) is the boundary — tableau cards are public state once tabled and must project identically to every viewer.
4. FOUNDATIONS §4: public tableau is rummy-specific state, first official use, `local-only`; no helper promotion (`ML-PP-002`).
5. FOUNDATIONS §11 no-leak: tabling moves cards from a private hand to public state — the projection must expose tabled cards to all viewers while the move itself must not reveal the rest of the acting seat's hand. The redaction harness is GAT19MELLEDFIV-012/013; this ticket keeps tableau state public-by-construction with no residual hand leak.

## Architecture Check

1. Separating `origin_seat` from per-card `played_by` lets lay-offs (GAT19MELLEDFIV-008) credit the playing seat without rewriting meld ownership — a clean extension point.
2. No backwards-compatibility shims.
3. `engine-core` untouched; tableau nouns are crate-local; `ML-PP-002` is a first-use local-only record, not a promotion.

## Verification Layers

1. Tabled melds are public with stable ids and correct ownership -> `cargo test -p meldfall_ledger` tableau tests.
2. Public projection identical across viewers -> public-view projection test (full no-leak matrix in GAT19MELLEDFIV-013).
3. First-use primitive recorded, no promotion -> FOUNDATIONS §4 alignment check.

## What to Change

### 1. `state.rs` — tableau

Public meld tableau grouping by `MeldGroup` with stable `MeldId`s, `origin_seat`, and per-card `played_by`/`play_turn`; round-played score accumulation hook.

### 2. `rules.rs` — tabling

Tabling a validated meld into the public tableau, asserting legality at table time and assigning score-credit to the playing seat.

### 3. Public projection + tableau traces + ledger entry

Public tableau projection (all viewers see tabled cards + score-credit owner). Golden trace for tabling and public visibility. Record `ML-PP-002` (public meld tableau / zone model, first use, `local-only`).

## Files to Touch

- `games/meldfall_ledger/src/state.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/rules.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/rules.rs` (modify — tableau tests)
- `games/meldfall_ledger/tests/golden_traces/public-tableau-tabling.trace.json` (new)

## Out of Scope

- Lay-off onto existing melds (GAT19MELLEDFIV-008) and discard-pickup (GAT19MELLEDFIV-009).
- The full pairwise no-leak matrix and the atlas/ledger doc reconciliation (GAT19MELLEDFIV-013/022).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` tableau tests: tabled meld gets a stable id, `origin_seat`, and per-card `played_by`.
2. Public projection shows tabled cards + score-credit owner to every viewer identically.
3. `cargo test --workspace` passes.

### Invariants

1. Tableau cards are public once tabled; no residual hand leak on the tabling move (FOUNDATIONS §11).
2. `origin_seat` and per-card `played_by` are independently tracked (FOUNDATIONS §4 first-use local-only).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — tableau grouping, stable ids, ownership separation.
2. `games/meldfall_ledger/tests/golden_traces/public-tableau-tabling.trace.json` — tabling + public visibility.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. Full multi-viewer no-leak assertions are the boundary of GAT19MELLEDFIV-013; this ticket verifies public-by-construction projection.

## Outcome

Completed: 2026-06-26

- Added stable public meld id allocation on `MeldTableau` and a `table_new_meld` rule helper that validates ownership/legality, moves selected cards from the acting hand to public tableau, assigns `origin_seat`, and records per-card `played_by`/`score_credit_owner`.
- Added the round-played score accumulation hook for tabled meld cards while keeping origin ownership separate from score-credit ownership for later lay-off support.
- Added public tableau projection types in `visibility.rs`; public observer and all seat viewers receive the same tabled card identities, origin seat, played-by owner, score-credit owner, and play turn.
- Added tableau tests proving stable ids, hand-safe tabling, score-credit ownership, and identical viewer projections that do not include the acting seat's remaining hand card.
- Added `public-tableau-tabling.trace.json` with `ML-PP-002` recorded as first official use, `local-only`.

Verification:

- `cargo fmt --all --check`
- `cargo test -p meldfall_ledger`
- `cargo test --workspace`
