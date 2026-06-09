# GAT101PLATRI-002: Third-use primitive-pressure ledger decision and mechanic-atlas update

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`; modifies `docs/MECHANIC-ATLAS.md` (§10/§10B, and §10A only if promotion debt is created). No Rust code.
**Deps**: GAT101PLATRI-001

## Problem

`plain_tricks` is the **third** official use of the deterministic shuffle / private hand / staged-reveal mechanic shape (`high_card_duel`, `poker_lite`, then this game). FOUNDATIONS §4 makes the third use a **hard gate**: the game MUST NOT proceed past crate-skeleton work until the primitive-pressure ledger records exactly one decision — reuse / promote narrow typed helper / explicit defer-reject / ADR escalation. `docs/MECHANIC-ATLAS.md` §10B explicitly names `plain_tricks` as this third use requiring a ledger decision before extraction. This ticket makes and records that decision and is a §12 stop-condition guard for the whole gate.

## Assumption Reassessment (2026-06-09)

1. `docs/MECHANIC-ATLAS.md` §10B records the deterministic shuffle / private hand / staged reveal row with `high_card_duel` + `poker_lite` and the note "Third card/private-hand use, likely `plain_tricks` if it repeats the shape, must record a ledger decision before extraction"; §10A is currently empty (no open promotion debt). `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` exists as the second-use comparison to cross-reference (not duplicate).
2. Spec §2 (Third-use hard gate), §5 item 2, §6 (third-use hard-gate exit criteria), and §8 (§4 row) bind this ticket; spec appendix A6 supplies provisional, non-binding framing leaning defer-reject or at most a narrow behavior-free seeded-shuffle helper. The implementation-time ledger comparison is authoritative.
3. Shared boundary under audit: the `game-stdlib` promotion boundary (FOUNDATIONS §4) and the atlas ledger contract. Comparison spans `games/high_card_duel/src/setup.rs`, `games/poker_lite/src/setup.rs`, and the planned `games/plain_tricks/src/setup.rs` shuffle/deal shape.
4. FOUNDATIONS §4 third-use rule and §12 stop condition "a third repeated mechanic proceeds without ledger decision" are the principles under audit; restated here before trusting the spec narrative — the decision is blocking, not advisory.
5. Enforcement surface: this ticket IS the §4 third-use hard-gate decision. It introduces no hidden-information leak and no replay/hash change by itself; a *promote* outcome would touch deterministic shuffle code in two existing games and is gated to GAT101PLATRI-003 with trace/hash-preservation evidence. The decision must also record whether `plain_tricks` trick-count round scoring is a third use of the public resource-accounting shape (`token_bazaar`/`poker_lite`) or distinct outcome scoring; the spec's stance is distinct (run the same four-option decision only if the comparison refutes it).

## Architecture Check

1. Recording the decision before any shuffle/hand/deal code (vs. extracting speculatively or deciding post-hoc) is exactly the FOUNDATIONS §4 hard gate; it prevents an unearned `game-stdlib` promotion and prevents proceeding through a §12 stop condition.
2. No backwards-compatibility aliasing/shims; this is a decision record plus an atlas update.
3. Confirms `engine-core` stays noun-free (no kernel change) and that any `game-stdlib` change is earned via the atlas — the decision explicitly chooses among reuse/promote/defer-reject/ADR.

## Verification Layers

1. Exactly-one recorded decision, made before rules implementation -> manual review against spec §6 third-use exit criteria + grep that `PRIMITIVE-PRESSURE-LEDGER.md` states one of the four options.
2. Atlas consistency (§10B row updated from "pending" to the decision; §10A only if debt) -> codebase grep-proof on `docs/MECHANIC-ATLAS.md`.
3. Accounting-shape stance recorded (trick scoring vs resource accounting) -> manual review of the ledger entry.
4. FOUNDATIONS §4 / §12 alignment -> FOUNDATIONS alignment check.

## What to Change

### 1. `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Author the third-use ledger entry: compare `high_card_duel`, `poker_lite`, and planned `plain_tricks` shuffle/private-hand/viewer-filtered-deal shapes using the spec §6 / appendix A6 ledger fields (holding size, reveal model, legality coupling, terminal reveal). Record exactly one decision: reuse / promote narrow typed helper / explicit defer-reject with rationale / ADR escalation. Cross-reference (do not duplicate) `docs/MECHANIC-ATLAS.md` and `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`. Record the accounting-shape stance (trick-count scoring is outcome scoring, not resource accounting — or, if refuted, run the same four-option decision). Name the next review trigger.

### 2. `docs/MECHANIC-ATLAS.md`

Update the §10B deterministic-shuffle row from "third use pending" to the recorded decision; record the trick-scoring-vs-accounting stance on the resource-accounting row. If — and only if — the decision is *promote* and back-port is deferred, add a §10A promotion-debt row (named games, primitive, evidence, risk, closure gate) that GAT101PLATRI-003 or the capstone closes. (First-use rows for trick-specific shapes are added later in GAT101PLATRI-020.)

## Files to Touch

- `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Writing any shuffle/hand/deal Rust code (GAT101PLATRI-005) — forbidden until this decision is recorded.
- The conditional helper extraction + back-port itself (GAT101PLATRI-003), which is worked only if the decision is *promote*.
- Atlas first-use rows for follow-suit / trick resolution / trick-winner-leads / deal rotation (GAT101PLATRI-020).

## Acceptance Criteria

### Tests That Must Pass

1. `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` records exactly one of the four decisions, dated before rules implementation.
2. `grep -nE "deterministic shuffle / private hand" docs/MECHANIC-ATLAS.md` shows the §10B row updated to the recorded decision (no longer "pending").
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The third-use hard gate resolves before any `plain_tricks` shuffle/hand code is written (FOUNDATIONS §4/§12).
2. No silent `game-stdlib` promotion: any promotion is recorded with atlas adoption/debt for all matching prior official games (FOUNDATIONS §4/§11).

## Test Plan

### New/Modified Tests

1. `None — documentation/decision ticket; verification is command-based and named in Assumption Reassessment.`

### Commands

1. `grep -nE "plain_tricks|third use|pending" docs/MECHANIC-ATLAS.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command set is correct: this ticket produces a decision record and an atlas edit; the downstream code consequence (local shuffle vs promoted helper) is verified in GAT101PLATRI-005 / GAT101PLATRI-003.
