# GAT6DIRFLI-002: Primitive-pressure comparison & ledger decision

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — documentation/governance: `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new), `docs/MECHANIC-ATLAS.md` (modify). Gates whether `game-stdlib` gains surface (GAT6DIRFLI-003).
**Deps**: 001

## Problem

`directional_flip` is the **third** official game to exert rectangular-coordinate / directional-scan pressure after `three_marks` and `column_four`. Per `docs/FOUNDATIONS.md` §4 and `docs/MECHANIC-ATLAS.md` §4–§5, the third use is a **hard gate**: the game MUST NOT proceed into deep rule implementation until a primitive-pressure ledger decides exactly one of reuse / promote-narrow-helper / defer-reject / escalate-to-ADR. This ticket performs that comparison and records the decision. It is the architectural center of Gate 6 (spec §10) and blocks the crate skeleton (GAT6DIRFLI-004).

## Assumption Reassessment (2026-06-07)

1. `crates/game-stdlib/src/lib.rs` is currently a placeholder (`placeholder_version()` only, no spatial helpers), so any promotion would be genuinely new surface, not an edit to an existing helper. Confirmed against the current crate.
2. `docs/MECHANIC-ATLAS.md` §10 already carries candidate rows pointing at this decision: "fixed 2D occupancy", "simple line/pattern detection", and "coordinate/targeted placement" each list `three_marks`, `column_four` as `repeated-shape candidate` with "Gate 6 `directional_flip` comparison" as the next gate, and a "directional scanning and grouped flips" row for `directional_flip`. This ticket updates those rows with the decision. The atlas §6 ledger-entry field list is the required schema for the ledger.
3. Cross-artifact boundary under audit: the ledger compares the spatial helper shapes actually used in `games/three_marks/src/` and `games/column_four/src/` (cell-id vocabulary, line/ray scanning, direction/offset helpers) against `directional_flip`'s planned eight-direction ray scan (spec §10.1). The comparison reads existing game source; the decision authors governance docs only.
4. FOUNDATIONS §4 "`game-stdlib` is earned" motivates this ticket: restate the principle before deciding — helpers enter `game-stdlib` only after implemented games prove a repeated shape AND the primitive-pressure process records the decision; no helper enters `engine-core` merely because multiple games use it. The default per atlas §5 is "keep local until repeated public pressure proves extraction."
5. This ticket touches the §4 third-use mechanic hard gate. The decision must confirm that any promoted helper would be behavior-free (no flip/capture/win/legality/forced-pass/bot/UI policy — spec §10.3), introduce no mechanic noun into `engine-core` (§3), and have no replay/hash or visibility impact beyond what a typed coordinate/ray utility implies. If the helper would affect replay/hash semantics, data policy, visibility, or kernel vocabulary, the decision MUST be escalate-to-ADR (atlas §5 option 4), not silent promotion.

## Architecture Check

1. Recording the decision in a ledger before implementation is the mechanism FOUNDATIONS §4 / atlas §4 prescribe; it prevents an agent from "cleaning up" duplicated coordinate logic into a speculative helper mid-implementation (atlas §9 anti-pattern).
2. No backwards-compatibility aliasing/shims; this is new governance documentation plus an atlas table update.
3. The ledger explicitly asserts `engine-core` stays mechanic-noun-free and that `game-stdlib` growth (if any) is the earned third-use outcome — exactly the §3/§4 confirmations the architecture check requires.

## Verification Layers

1. Hard-gate-resolved invariant -> FOUNDATIONS alignment check (§4): the ledger records exactly one decision (reuse / promote / defer-reject / ADR) with rationale, per atlas §6 field list.
2. Atlas consistency -> codebase grep-proof: the three candidate rows in `docs/MECHANIC-ATLAS.md` §10 and the `directional_flip` row are updated to reflect the decision; no candidate row still points to an unresolved "Gate 6 comparison".
3. Boundary safety of any promotion -> FOUNDATIONS alignment check (§3/§5): ledger confirms no `engine-core` vocabulary change and no behavior-in-data, and names the replay/hash/visibility impact (must be none, or ADR).

## What to Change

### 1. `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Author the ledger entry using the `docs/MECHANIC-ATLAS.md` §6 field list (Mechanic shape, Status, Games exerting pressure, Relevant files/docs, What is repeated, What differs, Why local duplication is now risky or acceptable, Decision, Why not engine-core, Why game-stdlib is/ is not appropriate, Data/Rust boundary impact, Replay/hash impact, Visibility impact, Bot impact, UI/effect impact, Tests required, Benchmarks required, Back-port plan, Examples, Anti-examples, Agent misuse risks, Review owner/date). Compare `three_marks` / `column_four` / `directional_flip` per spec §10.1 and record exactly one decision per §10.2. Follow the `templates/PRIMITIVE-PRESSURE-LEDGER.md` structure. (`games/directional_flip/docs/MECHANICS.md`, authored in GAT6DIRFLI-020, must link to this ledger.)

### 2. `docs/MECHANIC-ATLAS.md`

Update the §10 table: resolve the "fixed 2D occupancy", "simple line/pattern detection", and "coordinate/targeted placement" candidate rows and the "directional scanning and grouped flips" row to reflect the recorded decision (promoted-primitive / rejected-deferred-with-rationale / ADR-required), citing the ledger.

## Files to Touch

- `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- The actual `game-stdlib` helper extraction and back-port — that is GAT6DIRFLI-003, conditional on this ticket deciding *promote*.
- Any `directional_flip` rule code (GAT6DIRFLI-005).
- The full mechanic inventory `MECHANICS.md` (GAT6DIRFLI-020); only the primitive-pressure comparison feeding the decision is in scope here.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` — ledger exists with all atlas §6 fields populated.
2. `grep -iE 'reuse|promote|defer|reject|ADR' games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` — exactly one decision is recorded.
3. `node scripts/check-doc-links.mjs` — atlas/ledger cross-links resolve.

### Invariants

1. Exactly one of {reuse, promote, defer/reject, ADR} is decided, with rationale (FOUNDATIONS §4; atlas §5).
2. The decision asserts no `engine-core` mechanic-noun change and names replay/hash/visibility impact (none, or ADR) (FOUNDATIONS §3/§13).

## Test Plan

### New/Modified Tests

1. `None — governance/documentation ticket; verification is command-based (doc-link + presence) and FOUNDATIONS-alignment review.`

### Commands

1. `test -f games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md && grep -iE 'Decision:' games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`
2. `node scripts/check-doc-links.mjs`
3. A doc-link + presence check is the correct boundary because this ticket produces no executable surface; the earned-promotion code path (if any) is verified in GAT6DIRFLI-003.
