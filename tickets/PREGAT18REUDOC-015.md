# PREGAT18REUDOC-015: Centralize IP evidence receipt + add external SOURCES with lessons/non-adoptions

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/IP-POLICY.md`, `docs/SOURCES.md`)
**Deps**: 012

## Problem

IP evidence is scattered, and the external prior-art comparables the change plan consulted carry no recorded Rulepath-specific lesson or explicit non-adoption — risking that comparable frameworks' differing authority/AI assumptions dilute Rulepath doctrine. This ticket centralizes the IP evidence receipt + source IDs and records the 8 external sources with their lessons and explicit non-adoptions.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/IP-POLICY.md` and `docs/SOURCES.md` exist; spec D11. The IP evidence receipt centralizes into the `GAME-EVIDENCE.md` receipt (ticket 012); hence `Deps: 012`.
2. Verified against spec D11 and the report's risk register (§6.1 "External prior art dilutes Rulepath doctrine → `SOURCES.md` records Rulepath-specific lesson and explicit non-adoption"): add the 8 external sources with per-source lesson + non-adoption.
3. Cross-artifact boundary under audit: `IP-POLICY.md` references the IP receipt field in `GAME-EVIDENCE.md` (ticket 012); the receipt must exist before the pointer (`check-doc-links`).
4. FOUNDATIONS §10 (IP conservatism) motivates this: restating the principle — public games use original/public-domain/permissioned IP; the SOURCES non-adoptions record *why* external prior art is consulted as evidence but not copied into Rulepath doctrine.

## Architecture Check

1. A centralized IP evidence receipt (pointing at `GAME-EVIDENCE.md`) + explicit source non-adoptions is cleaner than scattered IP notes and protects doctrine from external-framework drift.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `IP-POLICY.md` centralizes the IP evidence receipt + source IDs (pointing at `GAME-EVIDENCE.md`) -> codebase grep-proof.
2. `SOURCES.md` adds the 8 external sources, each with a Rulepath-specific lesson + explicit non-adoption -> grep + count.
3. `templates/GAME-EVIDENCE.md` exists (Deps 012) -> `test -f`.
4. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. IP-POLICY centralization

Centralize the IP evidence receipt + source IDs in `docs/IP-POLICY.md`, pointing at the `GAME-EVIDENCE.md` IP-receipt field.

### 2. SOURCES external prior art

Add the 8 external sources to `docs/SOURCES.md`, each with its Rulepath-specific lesson and explicit non-adoption.

## Files to Touch

- `docs/IP-POLICY.md` (modify)
- `docs/SOURCES.md` (modify)

## Out of Scope

- The other D11 docs (official-contract 013, AI/UI 014, AGENT-DISCIPLINE/archival 016).
- Authoring the `GAME-EVIDENCE.md` receipt (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "GAME-EVIDENCE|IP receipt|source ID" docs/IP-POLICY.md` returns the centralized receipt pointer.
2. `grep -ciE "non-adoption|not adopted" docs/SOURCES.md` reflects the 8 external sources' non-adoptions (count consistent with the added sources).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No external prior art is copied into public Rulepath doctrine (§10); each is consulted-and-non-adopted with a recorded reason.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (receipt grep, source-count grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "GAME-EVIDENCE|non-adoption|lesson" docs/IP-POLICY.md docs/SOURCES.md`
2. `node scripts/check-doc-links.mjs`
3. The receipt + source-count grep is the correct boundary; docs-only with no code surface.
