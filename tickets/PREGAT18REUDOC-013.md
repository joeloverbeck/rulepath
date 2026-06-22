# PREGAT18REUDOC-013: Point OFFICIAL-GAME-CONTRACT at GAME-EVIDENCE + completion profiles

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs-only (`docs/OFFICIAL-GAME-CONTRACT.md`)
**Deps**: 012

## Problem

The official-game contract enumerates the per-game deliverable set but doesn't reference the new canonical `GAME-EVIDENCE.md` receipt or the completion-profile system. It should point at the receipt and define completion profiles so a game's conformance is read from one place.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/OFFICIAL-GAME-CONTRACT.md` exists as the per-game deliverable contract (referenced from `docs/FOUNDATIONS.md` §6 and `specs/README.md`); spec D11.
2. Verified `templates/GAME-EVIDENCE.md` is authored by ticket 012; this ticket references it, so `Deps: 012` (create-then-reference on the new receipt template).
3. Cross-artifact boundary under audit: the contract references the receipt + completion profiles defined in ticket 012; the receipt must exist before the contract can point at it (`check-doc-links`).
4. FOUNDATIONS §6 (official games are evidence-heavy) motivates this: restating the principle — pointing the contract at the canonical receipt and defining completion profiles records what "done" means per profile **without weakening** any per-game evidence requirement.

## Architecture Check

1. Pointing the contract at one canonical receipt is cleaner than re-listing evidence requirements inline; completion profiles make partial-completion states explicit without lowering the bar.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `OFFICIAL-GAME-CONTRACT.md` references `GAME-EVIDENCE.md` and defines completion profiles -> codebase grep-proof.
2. `templates/GAME-EVIDENCE.md` exists (Deps 012) -> `test -f`.
3. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Point at the receipt

Add a reference from `OFFICIAL-GAME-CONTRACT.md` to `templates/GAME-EVIDENCE.md` as the canonical per-game conformance receipt.

### 2. Define completion profiles

Define the completion profiles in contract terms, keeping every existing evidence requirement intact (profiles describe applicability, not waivers).

## Files to Touch

- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)

## Out of Scope

- The other D11 docs (AI/UI in 014, IP/SOURCES in 015, AGENT-DISCIPLINE/archival in 016).
- Authoring the `GAME-EVIDENCE.md` receipt itself (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "GAME-EVIDENCE|completion profile" docs/OFFICIAL-GAME-CONTRACT.md` returns the pointer + profile definition.
2. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No per-game evidence requirement is weakened; profiles describe applicability only.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (pointer grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "GAME-EVIDENCE|completion profile" docs/OFFICIAL-GAME-CONTRACT.md`
2. `node scripts/check-doc-links.mjs`
3. The pointer grep + link check is the correct boundary; docs-only with no code surface.
