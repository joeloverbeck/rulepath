# GAT8AFTROAREA-002: Realign progress.md and add candidate-placement note

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — `progress.md` prose only.
**Deps**: None

## Problem

`progress.md` states Gate 8 is planned with a mandatory `blackjack_lite` continuation checkpoint before Gate 9 (`progress.md:1-4`), and has no completion entry for Gate 8 even though it carries dated entries for Gates 7.1, 7, 6, 5, and 3. A new contributor reading it would believe Gate 8 is unbuilt and Blackjack-blocked. This ticket fixes the header, adds a Gate 8 `high_card_duel` completion entry in the existing per-gate style, and adds the candidate-placement note that routes Gate 9+ candidates (spec D2 + D4 / WB2 + WB4). D4 is merged here because the spec routes the note to the "progress/specs index area" and `progress.md` is the natural human-status home — keeping it in one file with the Gate 8 entry is one reviewable diff.

## Assumption Reassessment (2026-06-08)

1. `progress.md:1-4` reads `Gate 8 is planned as the next chance / hidden-information proof (high_card_duel), with a mandatory blackjack_lite continuation checkpoint before Gate 9 admission`; the only `high_card_duel` mention is this stale header (no `## Gate 8` completion heading exists). Existing per-gate entries (`## Gate 7.1 …`, `## Gate 7 …`, `## Gate 6 …`) establish the entry style to mirror.
2. Corrected status and routing are sourced from `specs/README.md` (rows 36-38), `docs/ROADMAP.md` §10-§11 (Gate 8 = `high_card_duel`; Gate 9 = `token_bazaar` / `resource_race` and `secret_draft`), and `docs/adr/0006-blackjack-lite-roadmap-placement.md`. The candidate-placement table content is given verbatim in the spec's §Candidate-placement note.
3. Cross-artifact boundary under audit: `progress.md` is the human status doc that must agree with the canonical index (`specs/README.md`) without duplicating the ladder (`docs/ROADMAP.md`). This ticket edits only `progress.md`; the `specs/README.md` maintenance row is GAT8AFTROAREA-006, and ROADMAP §3 is NOT edited (the spec confines the alias clarification to the progress/index area unless it is genuinely placement law).

## Architecture Check

1. Adding a dated Gate 8 entry that mirrors the existing per-gate format keeps `progress.md` internally consistent and makes the completion auditable, cleaner than only editing the header line. The candidate-placement table lives here (not in ROADMAP) so ROADMAP stays ladder law, not a progress log (spec §Documentation-update-rules).
2. No backwards-compatibility shims; prose-only change.
3. `engine-core` untouched; no `game-stdlib` change; the `blackjack_lite` row restates the accepted ADR 0006 deferral and does not authorize any promotion.

## Verification Layers

1. Header no longer claims Gate 8 planned / mandatory Blackjack checkpoint -> codebase grep-proof (`grep -niE "gate 8 is planned|mandatory blackjack" progress.md` returns no match).
2. A Gate 8 `high_card_duel` completion entry exists -> codebase grep-proof (`grep -n "Gate 8" progress.md` shows a completion heading).
3. Candidate-placement note names all six candidates -> codebase grep-proof (`grep -niE "token_bazaar|resource_race|secret_draft|blackjack_lite|poker_lite|plain_tricks" progress.md`).

## What to Change

### 1. Header realignment

Rewrite `progress.md:1-4` so the current status states Gates 0–8 complete in the worktree, High Card Duel is the accepted Gate 8 proof, `blackjack_lite` is deferred by ADR 0006 (not a Gate 9 blocker), and Gate 9 (`token_bazaar`) is the next implementation target. Keep `specs/README.md` named as the mutable source of truth for gate progress.

### 2. Gate 8 completion entry

Add a `## Gate 8 High Card Duel` entry in the existing dated per-gate style: completion context, the proved surfaces (deterministic setup shuffle, private views, viewer-filtered effects/logs, public replay/export redaction, bot view discipline, browser no-leak smoke, benchmark smoke floors), and boundary notes (card/zone semantics stayed game-local; no `engine-core` / `game-stdlib` promotion).

### 3. Candidate-placement note

Add the candidate-placement table from the spec's §Candidate-placement note, placing `token_bazaar` (primary Gate 9 target), `resource_race` (alias/alternate only — not a separate build target), `secret_draft` (later proof), `blackjack_lite` (deferred under ADR 0006), `poker_lite` / `plain_tricks` (Gate 10+), and the private monster-game red-team (not part of this pass).

## Files to Touch

- `progress.md` (modify)

## Out of Scope

- Editing `docs/ROADMAP.md` §3 candidate ladder — placement law unless the alias clarification genuinely requires it; default home is `progress.md`.
- `specs/README.md` maintenance row + `Done` flip (GAT8AFTROAREA-006).
- `README.md` / `apps/web/README.md` (GAT8AFTROAREA-001 / -003).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "gate 8 is planned|mandatory blackjack" progress.md` returns no match.
2. `grep -n "Gate 8" progress.md` shows a completion entry (not just the header reference).
3. `grep -ciE "token_bazaar|resource_race|secret_draft|blackjack_lite|poker_lite|plain_tricks" progress.md` is ≥ 6, and `node scripts/check-doc-links.mjs` passes.

### Invariants

1. `progress.md` states Gate 8 complete and names Gate 9 as next, consistent with `specs/README.md`.
2. The candidate-placement note marks `resource_race` as alias/alternate, not a second parallel build target.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -niE "gate 8|blackjack|token_bazaar|resource_race|secret_draft" progress.md` — targeted truthfulness + routing proof.
2. `node scripts/check-doc-links.mjs` — full doc-link integrity pass.
3. A narrower grep command is the correct boundary because the change is prose-only; no Rust/test pipeline is affected.
