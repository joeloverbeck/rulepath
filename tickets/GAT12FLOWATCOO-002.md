# GAT12FLOWATCOO-002: Primitive-pressure ledger and mechanic-atlas reviews

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new); `docs/MECHANIC-ATLAS.md` §10B/§10A (modify). No code surface.
**Deps**: None

## Problem

FOUNDATIONS §4 makes mechanic-pressure review a precondition, not an afterthought: before `flood_watch` writes any shuffle, automation, or visibility code, the gate must record (a) that it is **not** a second reaction-capable use of the `masked_claims` reaction-window primitive, (b) that it is **not** a fifth use of the deterministic-shuffle / private-holdings / redacted-reveal triple (it has no per-seat private holdings), and (c) first-use records for the four genuinely new shapes — shared-outcome cooperative terminal, event-deck environment automation, role-modified action effects, and multi-action turn budgets. Spec Work-breakdown item 1 marks this **"Blocks all implementation tasks"**: if either review contradicts its expected outcome, the gate stops and the relevant atlas trigger fires before code is written.

## Assumption Reassessment (2026-06-11)

1. `docs/MECHANIC-ATLAS.md` currently reads (verified): §10A promotion-debt register is `_None_` (line 204); §10B carries the `reaction window/pending response` row (`masked_claims`, reopen "when a second reaction-capable official game appears; hard-gate before a third similar use") and the `deterministic shuffle / private hand / staged reveal` row (`high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`; reopen "before a fifth official game repeats deterministic shuffle plus private holdings plus redacted reveal/export"). The exemplar ledger `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` exists and is the structural model; `templates/PRIMITIVE-PRESSURE-LEDGER.md` exists.
2. The spec (§Scope "Pre-implementation atlas work", §Deliverables "Atlas / ledger", Work-breakdown item 1, §Documentation-updates `docs/MECHANIC-ATLAS.md`, Assumption A8) requires exactly: reaction-window second-use review (expected: not reaction-capable; trigger stays armed), deterministic-shuffle review (expected: not a fifth use — no private holdings), four new first-use §10B rows with second-use revisit triggers naming Gate 13/14 candidates (`frontier_control` faction powers / asymmetric victory; `event_frontier` event decks / periodic automation), and confirmation §10A stays `_None_`.
3. Cross-artifact boundary under audit: the atlas `§10B`/`§10A` registers are the repo-wide mechanic-pressure contract that Gate 13's author will read. The new rows' second-use revisit triggers must name the correct successor candidates so the next gate's ledger review binds to them. This ticket authors atlas state every later gate depends on; it must not weaken the existing reopen triggers.
4. FOUNDATIONS §4 (`game-stdlib` is earned) and the §11 invariant "third-use mechanic pressure is resolved before proceeding" motivate this ticket. The intended principle: first use → local; the four new shapes are first official uses recorded `local-only`, and neither the reaction-window second-use trigger nor the shuffle fifth-use reopen fires — confirmed before, not after, implementation.
5. Enforcement surface: this is the §4 hard-gate review surface itself. The reviews must confirm no `game-stdlib` promotion and no `engine-core` noun is authorized, and (per the shuffle row's "or earlier if" clause) record any new shuffle-helper / no-leak-helper evidence. If either expected outcome is contradicted at implementation, stop: the reaction-window hard-gate or the shuffle-row reopen fires.

## Architecture Check

1. Recording the pressure decision before the crate skeleton is the FOUNDATIONS §4 contract: a later "we should have extracted a helper" cannot be retrofitted cleanly, and an unreviewed third/fifth use is architectural debt. Authoring it as a doc + atlas update (no code) keeps it a reviewable, auditable diff.
2. No backwards-compatibility aliasing/shims; net-new ledger plus additive atlas rows.
3. Confirms `engine-core` stays free of `event`/`deck`/`role`/`scenario`/`district`/`flood`/`levee`/`budget` nouns and that `game-stdlib` gains nothing — the reviews' explicit output.

## Verification Layers

1. Reaction-window not-second-use review recorded -> FOUNDATIONS §4 alignment check (ledger prose against `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md`) + grep-proof the §10B reaction-window row's reopen trigger is unchanged.
2. Deterministic-shuffle not-fifth-use review recorded -> FOUNDATIONS §4 alignment check (no per-seat private holdings in `flood_watch`) + grep-proof the §10B shuffle row's "fifth official game" reopen clause is preserved.
3. Four first-use rows added with second-use revisit triggers -> grep-proof the new §10B rows exist and name the Gate 13/14 successor candidates.
4. §10A stays `_None_` -> grep-proof (`grep -A2 "## 10A" docs/MECHANIC-ATLAS.md` still shows `_None_`).

## What to Change

### 1. `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Instantiate from `templates/PRIMITIVE-PRESSURE-LEDGER.md`. Record: (a) the reaction-window second-use review against `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` (expected: not reaction-capable — the environment phase is automation, not a response window; no seat responds to another seat's pending action; the second-use trigger stays armed); (b) the deterministic-shuffle row review (expected: not a fifth use — deterministic shuffle and staged reveal but no per-seat private holdings; record any shuffle-helper / no-leak-helper evidence the row's "or earlier if" clause asks for); (c) first-use records for shared-outcome cooperative terminal, event-deck environment automation, role-modified action effects, and multi-action turn budgets — each `local-only`, each with a second-use revisit trigger naming the Gate 13/14 candidate. State explicitly that no `game-stdlib` promotion and no `engine-core` noun is authorized.

### 2. `docs/MECHANIC-ATLAS.md`

Update §10B: append the review outcomes to the `reaction window/pending response` and `deterministic shuffle / private hand / staged reveal` rows (do not weaken their reopen triggers), and add four new `local-only` first-use rows (`shared-outcome cooperative terminal`, `event-deck environment automation`, `role-modified action effects`, `multi-action turn budgets`) with second-use revisit triggers naming `frontier_control` (faction powers / asymmetric victory) and `event_frontier` (event decks / periodic automation). Confirm §10A still reads `_None_` (no edit unless debt opened — it must not be).

## Files to Touch

- `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any code, crate skeleton, or static data (GAT12FLOWATCOO-003+).
- Promoting anything into `game-stdlib` — this ticket's outcome is that nothing is promoted.
- The final §10A confirmation at gate close (GAT12FLOWATCOO-019 re-verifies no debt was opened).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the new ledger present.
2. `grep -c "local-only" docs/MECHANIC-ATLAS.md` increases by four (the new first-use rows).
3. `grep -A2 "## 10A" docs/MECHANIC-ATLAS.md` still shows `_None_` (no promotion debt opened).

### Invariants

1. Neither the reaction-window second-use reopen nor the deterministic-shuffle fifth-use reopen trigger is weakened or removed.
2. No `game-stdlib` promotion and no `engine-core` mechanic noun is authorized by the reviews.

## Test Plan

### New/Modified Tests

1. `None — documentation/atlas ledger ticket; verification is command-based (doc-link integrity + atlas grep-proofs). The FOUNDATIONS §4 review is the deliverable.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n "local-only" docs/MECHANIC-ATLAS.md && grep -A2 "## 10A" docs/MECHANIC-ATLAS.md`
3. A code/test pipeline is not the verification boundary: this ticket precedes all implementation by design (Work-breakdown item 1 blocks it). The boundary is atlas/ledger grep-proofs that the reviews landed with triggers intact.
