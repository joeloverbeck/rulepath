# PHA0NEXPHAFOU-004: ENGINE-GAME-DATA-BOUNDARY + MECHANIC-ATLAS N-seat/surface clarifications

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/ENGINE-GAME-DATA-BOUNDARY.md` + `docs/MECHANIC-ATLAS.md` edits only.
**Deps**: PHA0NEXPHAFOU-002

## Problem

Larger surfaces will pressure authors toward a generic "map/deck/hand/seat helper" in `engine-core`/`game-stdlib` or a map/scenario DSL. `ENGINE-GAME-DATA-BOUNDARY.md` forbids this but does not name N-seat / topology scale as a danger point, and `MECHANIC-ATLAS.md` does not arm the next-phase third-use gates the public ladder will trip quickly.

## Assumption Reassessment (2026-06-13)

1. `game-stdlib` is earned only via the atlas (FOUNDATIONS §4); `engine-core` is noun-free. The current largest surfaces (`games/frontier_control`, `games/event_frontier`) are still small — the implementer re-confirms their exact topology scale against those crates when writing the atlas interlock note, rather than hardcoding counts here.
2. Docs: `docs/ENGINE-GAME-DATA-BOUNDARY.md` (`engine-core` vocabulary, `game-stdlib` promotion, static-data law) and `docs/MECHANIC-ATLAS.md` (third-use rule, repeated-shape candidates, promotion-debt register). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target.
3. Cross-artifact boundary under audit: the boundary doc + the atlas; shared surface = the third-use / promotion discipline and the static-data-is-not-behavior law.
4. FOUNDATIONS principle restate: §4 (`game-stdlib` earned) and §5 (static data is typed content, not behavior). These edits are meaning-preserving clarifications.
5. Enforcement surface: §4 third-use hard gate. The "next-phase armed interlocks" register *arms* (does not pre-decide) the primitive-pressure ledger; it introduces no leakage or nondeterminism path and is enforced when each Gate 15+ game trips its third official use.

## Architecture Check

1. Naming the N-seat/topology danger point and arming the interlocks early is cleaner than discovering a third-use trip mid-gate with no ledger entry.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` stays noun-free; "large map is not a DSL license" — topology may be typed content, but conditions/triggers/formulas stay Rust (§5). No YAML/DSL is introduced.

## Verification Layers

1. Boundary clarifications present → manual review.
2. Atlas next-phase armed-interlock register present → codebase grep-proof.
3. No DSL/YAML or kernel change introduced → `bash scripts/boundary-check.sh` + FOUNDATIONS alignment check (§4/§5/§12).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/ENGINE-GAME-DATA-BOUNDARY.md`

Add examples: seat-range validators, graph topology, route networks, community-card evaluators, wall/deck shuffles, partnerships, side-pot allocators, and tile-meld validators all start game-local; after repeated use, `game-stdlib` may accept narrow typed helpers **only via the atlas**. Add a "large map is not a DSL license" warning: topology data may be typed content; conditions/triggers/formulas remain Rust.

### 2. `docs/MECHANIC-ATLAS.md`

Add a "next-phase armed interlocks" section: Texas Hold'Em trips deterministic shuffle / private-hand / community-card / showdown / accounting comparisons; Hearts/Oh Hell/Spades trip trick-taking and hidden-hand reuse; Star Halma/Pachisi trip graph/track topology; the original medium-map capstone trips graph/site/faction/asymmetric-victory hard gates. Require a ledger entry before each third official use.

## Files to Touch

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Proposing a DSL/YAML or any `engine-core` vocabulary change (each requires its own ADR).
- Promoting any helper into `game-stdlib` now — this ticket only arms the register; promotion decisions belong to each Gate 15+ ledger.
- Touching `engine-core`/`game-stdlib`/`games/*` code.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/ENGINE-GAME-DATA-BOUNDARY.md` names the N-seat/topology danger points and the "large map is not a DSL license" warning.
2. `docs/MECHANIC-ATLAS.md` has a "next-phase armed interlocks" section requiring a ledger entry before each third use.
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. The §4 third-use discipline is unchanged in meaning — the edit arms upcoming gates, it does not relax the hard gate.
2. No static-data behavior field (selector/trigger/formula) is introduced; no `engine-core` noun added.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. `grep -niE "armed interlock|large map is not a DSL" docs/MECHANIC-ATLAS.md docs/ENGINE-GAME-DATA-BOUNDARY.md`

## Outcome

Completed: 2026-06-13

Updated `docs/ENGINE-GAME-DATA-BOUNDARY.md` to name N-seat and larger-surface
danger points explicitly: seat-range validators, graph topology, route networks,
community-card evaluators, wall/deck shuffles, partnerships, side-pot
allocators, and tile-meld validators start game-local and may move to
`game-stdlib` only through the atlas. Added the "large map is not a DSL license"
warning that topology and surface budgets can be typed content while conditions,
triggers, formulas, selectors, legality, scoring, visibility, bot tactics, and
exception logic remain Rust-owned.

Added `docs/MECHANIC-ATLAS.md` §9A, `Next-phase armed interlocks`, arming future
ledger checks for River Ledger/Hold'Em, side pots, Hearts/Oh Hell/Spades, Rummy,
Star Halma/Pachisi, Four Winds Melds, and Commonwealth Frontier. The section
requires ledger decisions before third official uses but promotes no helper and
does not relax the hard gate.

Deviations from plan: none. No code, kernel vocabulary, static behavior data,
YAML, or DSL was introduced.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `grep -niE "armed interlock|large map is not a DSL" docs/MECHANIC-ATLAS.md docs/ENGINE-GAME-DATA-BOUNDARY.md`
  found the new atlas section and boundary warning.
- `rg -n "Seat-range validators|community-card evaluators|side-pot allocators|tile-meld validators|Next-phase armed interlocks|third official use" docs/ENGINE-GAME-DATA-BOUNDARY.md docs/MECHANIC-ATLAS.md`
  confirmed the requested examples and hard-gate wording.
