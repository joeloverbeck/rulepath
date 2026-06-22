# PREGAT18REUDOC-009: ARCHITECTURE ownership matrix + MECHANIC-ATLAS behavioral/scaffolding split

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/ARCHITECTURE.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: 004, 006

## Problem

The scaffolding lane needs an ownership home in `ARCHITECTURE.md` (which lane owns kernel ergonomics vs `game-stdlib` vs a future dev-only test-support crate) and the mechanic atlas must explicitly keep its behavioral gate while pointing non-behavioral repetition at the new scaffolding register. Without these edits the lane's ownership and the atlas/register division stay implicit.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/ARCHITECTURE.md` has an ownership table (six layers: `engine-core`, `game-stdlib`, `ai-core`, `games/*`, `wasm-api`, `apps/web`, `tools/*`) but **no** ownership matrix distinguishing kernel-ergonomics / `game-stdlib` / test-support lanes and **no** `game-test-support` crate (confirmed via `/reassess-spec` this session). ADR 0008 (ticket 004) names `ARCHITECTURE` as amended, so this edit lands only after 0008 is `Accepted`. Hence `Deps: 004`.
2. Verified `docs/MECHANIC-ATLAS.md` §§4–8 exist as the behavioral gate (§4 first/second/third-use, §5 hard-gate options, §6 ledger fields, §7 extraction process, §8 review questions) with no scaffolding-register link today (spec D9 / WB6).
3. Cross-artifact boundary under audit: the atlas links the register (ticket 006, hence `Deps: 006`); the ARCHITECTURE matrix names the future dev-only `game-test-support` crate that ADR 0008 defines as an allowed home (crate creation is the Part C unit, not here).
4. FOUNDATIONS §4 motivates this: restating the invariant — the atlas keeps its behavioral gate (§§4–8) intact; the ownership matrix adds lanes without moving any behavior, and is meaning-preserving given accepted ADR 0008.
5. Touches the §4 hard gate: confirm the atlas behavioral gate is unchanged, the register link is additive, and no leak/nondeterminism path is introduced.

## Architecture Check

1. An ownership matrix (3 lanes + dev-dependency rule) plus an atlas that explicitly splits behavioral-vs-scaffolding cleanly assigns each reuse kind a home, instead of leaving scaffolding ownership ambiguous in the existing 6-layer table.
2. No backwards-compatibility shims; the `game-test-support` crate is *named*, not created.
3. `engine-core` stays noun-free (§3); the behavioral earning rule (§4) is preserved.

## Verification Layers

1. `ARCHITECTURE.md` carries the ownership matrix (kernel ergonomics / `game-stdlib` / test-support) + dev-dependency rule -> codebase grep-proof.
2. `MECHANIC-ATLAS.md` §§4–8 behavioral gate intact + a link to the scaffolding register -> grep + manual review.
3. ADR 0008 `Accepted` precondition -> grep (`^Status: Accepted` on `docs/adr/0008-*.md`).
4. Kernel boundary intact -> `bash scripts/boundary-check.sh`.
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. ARCHITECTURE ownership matrix

Add a matrix distinguishing kernel-ergonomics / `game-stdlib` / dev-only test-support ownership, with the dev-dependency rule (test-support is a dev-only dependency).

### 2. MECHANIC-ATLAS split

Keep §§4–8 the behavioral gate verbatim; add a pointer that non-behavioral repetition routes to `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.

## Files to Touch

- `docs/ARCHITECTURE.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- The `FOUNDATIONS.md` §4 lane + boundary four-lane edits (ticket 008).
- Creating the `game-test-support` crate (Part C successor unit).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "game-test-support|ownership matrix|kernel ergonomics" docs/ARCHITECTURE.md` returns the matrix.
2. `grep -niE "MECHANICAL-SCAFFOLDING-REGISTER|scaffolding register" docs/MECHANIC-ATLAS.md` returns the register link, and §§4–8 headings are unchanged.
3. `bash scripts/boundary-check.sh` and `node scripts/check-doc-links.mjs` pass.

### Invariants

1. The atlas behavioral gate (§§4–8) is preserved; only an additive register link is added.
2. No mechanic noun enters `engine-core`; `boundary-check.sh` stays green.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep, boundary-check, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "ownership matrix|game-test-support|scaffolding register" docs/ARCHITECTURE.md docs/MECHANIC-ATLAS.md`
2. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
3. The grep + boundary-check pair is the correct boundary; these are docs with no code surface beyond the kernel-noun guard.
