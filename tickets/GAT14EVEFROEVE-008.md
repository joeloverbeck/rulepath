# GAT14EVEFROEVE-008: Reckoning pipeline, asymmetric victory, and terminal

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/src/{rules,effects,state}.rs` (ordered Reckoning pipeline, asymmetric instant victory, terminal detection)
**Deps**: GAT14EVEFROEVE-007

## Problem

A Reckoning always resolves and runs an ordered deterministic pipeline: **victory check → site-majority scoring → income → reset** (edict expiry + eligibility restoration). Victory is asymmetric — the Charter wins instantly with majority presence at ≥4 of 6 sites, the Freeholders with ≥8 total caches, both-met resolves to the Freeholders — checked **before any reset**. After the third Reckoning with no instant victory, the higher cumulative score wins, tie to the Freeholders (the deliberate inverse of Gate 13's incumbent tiebreak). This is the asymmetric-victory proof Gate 13 deferred, flowing through the same terminal/outcome-explanation contracts symmetric games used.

## Assumption Reassessment (2026-06-12)

1. The state, edict list, and effects this pipeline drives exist: verified ticket 004's `scores`/`terminal_outcome`/`epoch`/Reckoning markers, ticket 007's `active_edicts` list and `EdictExpired` expiry effect, and the per-site component counts from ticket 004. Eligibility restoration writes the `eligibility` markers ticket 005 reads.
2. The pipeline and constants are specified: verified the spec's "Reckoning" — victory check (Charter ≥4-site majority; Freeholder ≥8 caches; both → Freeholders), site scoring (1 point to strictly-greater presence; agents+depot for Charter, settlers for Freeholders; caches don't count for presence; ties award no one), income (+2 cap-respecting), reset (edict expiry + eligibility restore); final fallback by cumulative score with the Freeholder tiebreak (stable rule IDs).
3. Cross-crate boundary under audit: `ReckoningResolved` carries the full per-site breakdown and `Terminal` carries winner + victory type + decisive cause through the same generic terminal/effect contracts every symmetric game used; the asymmetric victory must not fork those contracts. Pipeline order (victory before reset) is load-bearing and must be deterministic.
4. FOUNDATIONS §11 (deterministic scoring/terminal) and §2 (scoring + terminal detection owned by Rust) motivate this ticket. Restated before trusting the spec: victory evaluation, scoring, and terminal cause are Rust-only; the both-met and tie rules are stable, deterministic, and replayable.
5. Determinism surface (§11): the pipeline is canonical replay/hash input. Confirm victory-check-before-reset ordering is fixed, the both-met and final-tiebreak rules are deterministic with stable rule IDs, and edict expiry is a deterministic list clear. No replay/hash *semantics* change — this reuses the terminal contract.

## Architecture Check

1. An ordered pipeline inside the Reckoning card's resolution (victory → scoring → income → reset) is cleaner than scattered checks: it makes "victory is checked before reset" a structural guarantee, not a convention.
2. No backwards-compatibility aliasing/shims — fills the rules/effects stubs; reuses the terminal contract.
3. `engine-core` stays noun-free (Reckoning/victory nouns are local); no `game-stdlib` `VictoryCondition`/`ScoringRound` promotion (forbidden by the spec; ledger authorized none).

## Verification Layers

1. Pipeline order (§11) -> a rule test asserting victory is evaluated before scoring before income before reset, and that edicts expire only at reset.
2. Asymmetric victory -> rule tests for Charter instant (≥4-site majority), Freeholder instant (≥8 caches), the both-met → Freeholders rule, and terminal behavior after victory.
3. Final fallback -> a rule test for the cumulative-score comparison and the Freeholder tiebreak when neither instant condition fires by the third Reckoning.
4. Determinism -> a replay test that the Reckoning breakdown, scores, and terminal outcome reproduce under seed + scenario + command stream.

## What to Change

### 1. Reckoning pipeline (`src/rules.rs`)

On a Reckoning card resolving: run victory check → site-majority scoring → income (+2, cap-respecting) → reset (expire all edicts with expiry effects, restore eligibility for all). Track Reckoning count/epoch.

### 2. Asymmetric victory and terminal (`src/rules.rs`, `src/state.rs`)

Evaluate Charter ≥4-site majority and Freeholder ≥8-cache instant conditions with the both-met → Freeholders rule (stable rule ID); after the third Reckoning with neither met, decide by cumulative score with the Freeholder tiebreak (stable rule ID). Write `terminal_outcome` (winner, victory type, totals, decisive cause).

### 3. Effects (`src/effects.rs`)

Emit `ReckoningResolved { round, victory_check, site_breakdown, income, expired_edicts }` and `Terminal { winner, victory_type, totals, summary }`.

## Files to Touch

- `games/event_frontier/src/rules.rs` (modify; created by 003)
- `games/event_frontier/src/effects.rs` (modify; created by 003)
- `games/event_frontier/src/state.rs` (modify; created by 003/004)

## Out of Scope

- Visibility projection / replay export of the breakdown (ticket 009).
- Outcome-explanation templates and the browser breakdown panel (tickets 016/017).
- Bot use of victory distance (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes the pipeline-order test (victory before scoring before income before reset).
2. Both instant victories, the both-met rule, the final cumulative-score fallback, and the Freeholder tiebreak tests pass.
3. The replay test reproduces the Reckoning breakdown, scores, and terminal outcome.

### Invariants

1. Victory is always checked before any reset; edicts expire only at Reckoning reset.
2. Winner, victory type, and decisive cause are Rust-computed and deterministic with stable rule IDs.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/rules.rs` — pipeline order, both victories, both-met, fallback, tiebreak, post-terminal behavior.
2. `games/event_frontier/tests/replay.rs` — Reckoning-breakdown + terminal reproduction.

### Commands

1. `cargo test -p event_frontier --test rules`
2. `cargo test -p event_frontier`
3. The per-crate rule/replay tests are the correct boundary — the full game now resolves end to end natively; visibility and tools layers land in later tickets.
