# PHA0NEXPHAFOU-003: FOUNDATIONS + ARCHITECTURE N-seat clarifications

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/FOUNDATIONS.md` + `docs/ARCHITECTURE.md` edits only.
**Deps**: PHA0NEXPHAFOU-002

## Problem

FOUNDATIONS (§2 behavior authority, §3 noun-free kernel, §8 bots, §11 invariants, §12 stop conditions) and ARCHITECTURE's runtime/WASM/replay model do not make the N-seat implication explicit. A future author could read the "seat" examples as two-player and smuggle multi-seat helpers (turn order, teams, pots, evaluators) into `engine-core`, or ship a 3+ game without proving per-seat view safety.

## Assumption Reassessment (2026-06-13)

1. FOUNDATIONS §3 already enumerates allowed kernel nouns (`seat id`, `actor`, `viewer`, `visibility scope`) and forbidden mechanic nouns (`board`, `card`, `deck`, `pot`, `faction`, …); `crates/engine-core/src/game.rs:16` `setup(seats: &[SeatId])` confirms the kernel is already multi-seat. The clarifications MUST NOT move any noun into the kernel.
2. Docs: `docs/FOUNDATIONS.md` §§2,3,8,11,12 (headers verified) and `docs/ARCHITECTURE.md` runtime/WASM/replay model. `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target.
3. Cross-artifact boundary under audit: the FOUNDATIONS principle set + the ARCHITECTURE match model; shared surface = the seat model and viewer/visibility contract codified in PHA0NEXPHAFOU-002.
4. FOUNDATIONS principle restate: §11 changes "unless an accepted ADR explicitly changes them and updates this section" — these edits are **meaning-preserving clarifications**, not principle changes, so no ADR beyond 0007 is required. §3 noun-free kernel is reinforced, not relaxed.
5. Enforcement surface: §11 invariant set + §12 stop conditions. The added stop condition ("a 3+ game that cannot prove viewer-safe public and per-seat projections does not ship") clarifies the existing no-leak/viewer-safe invariants; it introduces no leakage or nondeterminism path and is enforced by Gate 15+ no-leak proofs and the Infra D harness.

## Architecture Check

1. Making the N-seat implication explicit in the constitution prevents kernel-noun smuggling more reliably than per-spec vigilance.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` stays noun-free: the clarification reinforces §3, naming N-seat turn order/tables/teams/pots/graphs/decks/walls/factions/partnerships/evaluators as game-local or `game-stdlib`-via-atlas only.

## Verification Layers

1. Clarifications present and meaning-preserving → manual review + grep-proof that every §1–§13 heading survives unrenumbered.
2. New §12 stop condition reads as a clarification → FOUNDATIONS alignment check (§11/§12).
3. Cross-references to the contract doc resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/FOUNDATIONS.md`

Add meaning-preserving notes: §11 invariants apply to **any positive seat count** declared by a game, including pairwise seat-private redaction; §3 clarifies that N-seat turn order, tables, teams, pots, graphs, decks, walls, factions, partnerships, and hand evaluators are game-local or `game-stdlib` (atlas) only; add the §12 stop condition that a 3+ game which cannot prove viewer-safe public and per-seat projections does not ship. Cross-reference `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`.

### 2. `docs/ARCHITECTURE.md`

Add a "Multi-seat match model" subsection: game crates declare min/max seats and seat-role metadata; Rust owns active-seat / active-set and turn order; every view projection accepts a public observer or an authorized seat; replay records ordered seat assignments; UI may display but not infer turn order; all-active / simultaneous / reaction windows are game-local action-tree phases, not kernel concepts.

## Files to Touch

- `docs/FOUNDATIONS.md` (modify)
- `docs/ARCHITECTURE.md` (modify)

## Out of Scope

- Changing the meaning of any FOUNDATIONS principle (a genuine change routes to its own superseding ADR).
- Adding any kernel noun; touching `engine-core` code.
- Trace-schema or WASM exported-API schema changes.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/FOUNDATIONS.md` §3/§11/§12 carry the N-seat clarifications and the new stop condition; no principle is renumbered or deleted.
2. `docs/ARCHITECTURE.md` contains a "Multi-seat match model" subsection.
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. Every FOUNDATIONS §1–§13 heading is still present and unchanged in meaning (`grep -E "^## " docs/FOUNDATIONS.md` shows all thirteen).
2. `engine-core` is untouched and stays noun-free.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. `grep -nE "^## " docs/FOUNDATIONS.md` (confirm all thirteen sections intact)
