# GAT17VOWTIDOHHEL-012: L0 random-legal and bounded L1 rule-informed bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/vow_tide/src/bots.rs`; new `games/vow_tide/tests/bots.rs` + bot golden traces
**Deps**: 008, 009, 010

## Problem

Ship the mandatory L0 random-legal bot and a bounded L1 rule-informed bidding/play policy for every seat count. Both use only authorized seat views, public history, and Rust legal leaves, choose through normal validation, mutate no state, and never read hidden information. No search/sampling/learning.

## Assumption Reassessment (2026-06-21)

1. `crates/ai-core` exposes `RandomLegalBot` (the L0 base used by `games/plain_tricks/src/bots.rs`); Vow Tide's `bots.rs` is new and consumes the 010 seat-private view + the 007/008 legal leaves. `games/briar_circuit/src/bots.rs` (L0 + L1 `l1_priority`) is the closest structural precedent.
2. Spec Appendix C fixes L0 (uniform over legal leaves, declared bot RNG, viewer-safe explanation) and the bounded L1 (own-hand control estimate for bidding; `needed = bid - tricks_taken` contract-relative play; "currently winning" via the same pure comparator). L1 weights live in `AI.md` (014), not data.
3. Cross-artifact boundary under audit: the bot input schema is exactly the §C.2.1 authorized fields; bot explanations/candidate rankings are viewer-safe outputs consumed by traces (011), simulate (013), and dev panels (017/018).
4. FOUNDATIONS §8/§11 under audit: bots use the normal legal action API + allowed views only; public v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL; no hidden-world sampling or determinization.
5. §11 no-leak enforcement surface: the L1 may read only its own hand + public facts + own legal leaves; it must not inspect other hands/stock present in native state, and explanations mention only viewer-authorized facts (no "seat 4 has no clubs", no "ace in the stock").

## Architecture Check

1. Building L1 as a small documented authored policy over authorized inputs (no sampling) keeps it explainable, deterministic, and inside the no-leak law — distinct from any future L2 evidence pack.
2. No shims; new bots module.
3. `engine-core` untouched; no `game-stdlib` change; bots route through the same legal action API as humans.

## Verification Layers

1. L0/L1 request the normal legal tree, choose a leaf, validate normally, are deterministic under declared seed/view → `cargo test -p vow_tide --test bots`.
2. Bots never receive raw state/other hands/stock/unauthorized candidates → bot-input audit test + no-leak canary in explanations.
3. Many-seed legality/no-leak across N=3..7 → bots test + (simulation in 013).
4. No forbidden search/sampling/learning → manual review + grep for absence of rollout/determinization.

## What to Change

### 1. L0 random-legal

`bots.rs`: request legal leaves for the bot's authorized seat viewer, sample uniformly with declared bot RNG, submit through validation, return a viewer-safe explanation; empty legal set for an active non-terminal bot is a test failure.

### 2. Bounded L1 policy

Bidding: deterministic own-hand control estimate clamped to `0..=H`, nearest legal bid, hook-adjusted, tie-broken by stable numeric order. Play: `needed`-relative posture (secure lowest currently-winning when `needed>0`; shed/avoid when `needed==0`), "currently winning" computed against the public trick via the promoted comparator. Viewer-safe explanations only.

## Files to Touch

- `games/vow_tide/src/bots.rs` (new)
- `games/vow_tide/tests/bots.rs` (new)
- `games/vow_tide/tests/golden_traces/l0-bid-and-play.trace.json` (new)
- `games/vow_tide/tests/golden_traces/l1-contract-relative-bid-and-play.trace.json` (new)

## Out of Scope

- Simulations by seat count + simulate registration (013); `AI.md`/`COMPETENT-PLAYER`/`EVIDENCE-PACK` docs (014).
- Any L2 policy, search, sampling, ML/RL, or hidden-state access.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test bots` — L0/L1 legality, determinism, viewer-safe explanations.
2. `cargo test -p vow_tide` — bot traces deterministic.
3. `cargo clippy -p vow_tide --all-targets -- -D warnings`.

### Invariants

1. Bots submit only Rust legal leaves from authorized information; explanations leak no hidden fact.
2. No MCTS/ISMCTS/Monte Carlo/ML/RL/determinization is present.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/bots.rs` — authorized-input audit, many-seed legality/no-leak, hook-adjustment + contract-relative play scenarios.
2. `games/vow_tide/tests/golden_traces/l{0,1}-*.trace.json`.

### Commands

1. `cargo test -p vow_tide --test bots`
2. `cargo test -p vow_tide`
3. Narrower command rationale: the bots suite is the legality/no-leak boundary; many-seed completion across N=3..7 is exercised by the simulator (013).
