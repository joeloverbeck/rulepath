# GAT19MELLEDFIV-006: Meld validation — sets and runs (first-use primitive)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{rules,cards}.rs`; meld golden traces; first-use ledger entry ML-PP-001
**Deps**: GAT19MELLEDFIV-005

## Problem

Meldfall Ledger needs Rust-owned meld legality: sets (3–4 same-rank cards) and runs (3+ consecutive same-suit cards), with aces low (`A-2-3`) or high (`Q-K-A`) but never around-the-corner (`K-A-2`, `Q-K-A-2`). This is the first official use of meld validation in Rulepath, recorded as a `local-only` primitive-pressure entry (`ML-PP-001`); no `game-stdlib` helper is created.

## Assumption Reassessment (2026-06-25)

1. No existing `games/*` crate has meld validation (confirmed during reassessment: `game-stdlib` holds only `seat`, `trick_taking`, `board_space`; no rummy/meld helper). `rules.rs` and `cards.rs` exist from GAT19MELLEDFIV-005/004; ace-run helpers live in `cards.rs`.
2. Spec §3.1 (Melds row), Appendix A.2 (meld kinds + ace-in-runs rows), and Appendix D (`ML-PP-001`) define the legality and the ledger entry.
3. Cross-artifact: `MeldGroup`/`MeldKind` shapes in `state.rs` (GAT19MELLEDFIV-005) are the boundary this legality writes against; meld legality is pure (no hidden-state read).
4. FOUNDATIONS §4 `game-stdlib` is earned: first official use of meld validation stays local in `rules.rs`; the mechanic atlas says start local. This ticket records `ML-PP-001` (`local-only`) — it must NOT promote a helper; the third-use hard gate has not fired.
5. FOUNDATIONS §2 behavior authority: meld legality is Rust-owned and pure-deterministic; no TypeScript validates melds.

## Architecture Check

1. Implementing set/run legality in `rules.rs` against the `MeldGroup` shape keeps validation local, pure, and testable; ace-run helpers in `cards.rs` keep ordering logic with the card model.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `game-stdlib` gains nothing — `ML-PP-001` is the first-use local-only record, not a promotion.

## Verification Layers

1. Valid sets/runs accepted, invalid rejected (too small, mixed rank, mixed suit, gap) -> `cargo test -p meldfall_ledger` meld unit/property tests.
2. Ace low/high but no wrap -> golden trace `meld-run-valid-ace-low-high-no-wrap.trace.json` (A-2-3 and Q-K-A accepted; K-A-2 / Q-K-A-2 rejected).
3. First-use primitive recorded, no helper promoted -> FOUNDATIONS §4 alignment check + grep that no `game-stdlib` meld symbol exists.

## What to Change

### 1. `rules.rs` — meld legality

Set legality (3–4 same rank, single deck so no duplicate-rank multi-deck sets), run legality (3+ consecutive same suit), ace low/high/no-wrap, and viewer-safe diagnostics for invalid melds.

### 2. `cards.rs` — ace-run helpers

Rank-ordering helpers supporting ace-low and ace-high runs without around-the-corner wrap.

### 3. Meld golden traces + ledger entry

`meld-set-valid-and-invalid.trace.json`, `meld-run-valid-ace-low-high-no-wrap.trace.json`; record `ML-PP-001` (meld validation, first official use, `local-only`) — the ledger doc itself is reconciled in GAT19MELLEDFIV-022.

## Files to Touch

- `games/meldfall_ledger/src/rules.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/cards.rs` (modify; created by GAT19MELLEDFIV-004)
- `games/meldfall_ledger/tests/rules.rs` (modify — meld cases)
- `games/meldfall_ledger/tests/property.rs` (modify; created by GAT19MELLEDFIV-003 — meld legality properties)
- `games/meldfall_ledger/tests/golden_traces/meld-set-valid-and-invalid.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/meld-run-valid-ace-low-high-no-wrap.trace.json` (new)

## Out of Scope

- Public tableau placement (GAT19MELLEDFIV-007), lay-off (GAT19MELLEDFIV-008), discard-pickup (GAT19MELLEDFIV-009).
- The atlas/ledger doc reconciliation (GAT19MELLEDFIV-022) — this ticket only emits the `ML-PP-001` test/record.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` meld tests: valid sets/runs accepted; too-small, mixed-rank, mixed-suit, gapped, and wrap (`K-A-2`, `Q-K-A-2`) rejected.
2. Property test: a generated legal meld always validates; ownership/conservation holds.
3. `cargo test --workspace` passes.

### Invariants

1. Meld legality is Rust-owned and pure (FOUNDATIONS §2); no TypeScript meld validation.
2. First-use stays local — no `game-stdlib` rummy/meld symbol exists (FOUNDATIONS §4).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — set/run legality + ace-run edge cases.
2. `games/meldfall_ledger/tests/property.rs` — generated-meld legality property.
3. `games/meldfall_ledger/tests/golden_traces/meld-*.trace.json` — meld accept/reject traces.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. `grep -rn "meld" crates/game-stdlib/src` must return no meld helper (first-use-stays-local proof).
