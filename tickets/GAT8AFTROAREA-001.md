# GAT8AFTROAREA-001: Realign root README.md Gate 8 status

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — root `README.md` prose only.
**Deps**: None

## Problem

The root `README.md` tells a new contributor that Gate 8 is still planned and that a Blackjack checkpoint blocks Gate 9. Both are false in the current worktree: Gate 8 `high_card_duel` is complete (registered in `crates/wasm-api`, all five game tools, and the `smoke:e2e` browser suite), and `blackjack_lite` is deferred by ADR 0006 with no interlock before Gate 9. The README's per-game command-example list is also stale — it ends at `draughts_lite` and omits `high_card_duel`. This ticket makes the README's Status section and per-game list truthful (spec D1 / WB1).

## Assumption Reassessment (2026-06-08)

1. `README.md` currently reads `**Gates 0-7.1 complete; Gate 8 planned**` (`README.md:17`) and describes a `post-Gate-8 blackjack_lite continuation checkpoint before Gate 9 admission` (`README.md:22-24`); the per-game checks list names `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite` and omits `high_card_duel` (`README.md:84-86`). All three are stale and in scope.
2. The corrected status is sourced from `specs/README.md` (Gate 8 = `high_card_duel` `Done`, rows 36-37; 6C checkpoint closed by ADR 0006, row 38) and `docs/adr/0006-blackjack-lite-roadmap-placement.md` (Accepted 2026-06-08). `high_card_duel` completeness was confirmed: `grep high_card_duel crates/wasm-api/src/lib.rs` (registered), all five `tools/*/src/main.rs`, and `apps/web/e2e/high-card-duel.smoke.mjs` present.
3. Cross-artifact boundary under audit: the README is a human-orientation pointer that must agree with the canonical index (`specs/README.md`) and ladder law (`docs/ROADMAP.md` §10). This ticket changes only `README.md`; it does not edit the index or ROADMAP (those already read correctly).

## Architecture Check

1. Editing only the README's Status paragraph and per-game list — not restating every prior gate — keeps the doc short and avoids turning the README into a progress diary (spec §Documentation-update-rules). Cleaner than duplicating the `specs/README.md` table here.
2. No backwards-compatibility shims; prose-only change.
3. `engine-core` untouched; no `game-stdlib` change; no mechanic noun introduced.

## Verification Layers

1. README no longer claims Gate 8 planned / Blackjack-blocks-Gate-9 -> codebase grep-proof (`grep -niE "gate 8 .*plan|blackjack" README.md` returns no stale claim).
2. README per-game list includes `high_card_duel` -> codebase grep-proof (`grep -n high_card_duel README.md`).
3. Doc links still resolve after the edit -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Status section

Rewrite `README.md:15-26` so it states Gates 0–8 are complete (High Card Duel is the accepted Gate 8 chance / hidden-information proof), that `blackjack_lite` is deferred by ADR 0006 and does not block Gate 9, and that Gate 9 (`token_bazaar`) is the next implementation target. Keep the existing pointers to `specs/README.md` and `docs/ROADMAP.md`. Use short status prose.

### 2. Per-game command list

Add `high_card_duel` to the per-game checks list (`README.md:84-86`) so all six official games are named.

## Files to Touch

- `README.md` (modify)

## Out of Scope

- Editing `specs/README.md`, `docs/ROADMAP.md`, or `docs/MECHANIC-ATLAS.md` — already reconciled (covered by GAT8AFTROAREA-006 for the index maintenance row only).
- `progress.md` and `apps/web/README.md` (GAT8AFTROAREA-002 / -003).
- Any gameplay or CI change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "gate 8.*planned|blackjack.*before gate 9|checkpoint before gate 9" README.md` returns no match.
2. `grep -n "high_card_duel" README.md` shows it in the per-game list.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. `README.md` Status reflects Gate 8 complete and Gate 9 as next target, consistent with `specs/README.md` and `docs/ROADMAP.md` §10.
2. README remains short status prose, not a per-gate progress log.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -niE "gate 8|blackjack|high_card_duel|token_bazaar" README.md` — targeted post-edit truthfulness proof.
2. `node scripts/check-doc-links.mjs` — full doc-link integrity pass.
3. A narrower command is correct here because the change is prose-only; no Rust/test pipeline is affected.
