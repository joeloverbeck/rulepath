# GAT11MASCLABLU-009: Level 0 and Level 1 bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/src/bots.rs`, `src/lib.rs`
**Deps**: GAT11MASCLABLU-007, GAT11MASCLABLU-008

## Problem

Both bot roles — claimant and responder — must act through the same legal action API as humans, using only the bot seat's allowed view, deterministically under declared inputs. Level 0 picks uniformly from the legal tree; Level 1 adds a deterministic parameterized claim policy (honest/underclaim/bluff) and a response policy (certain-lie detection by public counting plus a calibrated challenge threshold), with viewer-safe explanations.

## Assumption Reassessment (2026-06-10)

1. The legal trees and validation from GAT11MASCLABLU-005–007 and the seat-view projection from GAT11MASCLABLU-008 are the bot's only inputs. The Level 0 shape model is `RandomBot` (confirmed in `games/plain_tricks/src/bots.rs`); `ai-core` provides the bot RNG/infra.
2. Spec §"Bot policy": Level 0 `MaskedClaimsRandomBot` selects uniformly from the legal tree in both phases; Level 1 `MaskedClaimsLevel1Bot` is deterministic under (seat view + bot seed) with a claim policy (honest default preferring the highest-grade held tile; deterministic parameterized bluff rate near the equilibrium-informed one-third; counting guard against certain-lie claims) and a response policy (certain-lie detection by public counting; calibrated threshold challenge; otherwise accept).
3. Cross-artifact boundary under audit: the legal-action API and the allowed-view boundary — the bot must route every decision through the same action tree and validation as a human command and read only its own seat view.
4. FOUNDATIONS §8 (public bots are competent, explainable, fair, deterministic under declared inputs, beatable, use the normal legal action API, mutate no state, and use no hidden information; no MCTS/ISMCTS/Monte Carlo/ML/RL) and §11 are the principles under audit.
5. §11 no-leak firewall enforcement surface: bot explanations and candidate rankings. Confirm no rationale references the pedestal tile's actual identity, opponent hand, or reserve, and that a decision and its rationale are unchanged when the hidden pedestal tile differs but the bot's allowed view is identical.

## Architecture Check

1. A deterministic parameterized policy (priced bluff rate, public-counting challenge) is explainable and beatable — the FOUNDATIONS §8 product-opponent posture — and avoids the forbidden search/learning classes by construction.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; no `game-stdlib` bluff-policy or challenge-resolver helper; the bot reads no hidden state.

## Verification Layers

1. Legal-only selection in both phases -> bot legality check (bot test + simulation in GAT11MASCLABLU-015).
2. Determinism under declared inputs -> bot test: same seat view + bot seed → same decision.
3. No hidden-state dependence -> bot test: vary the hidden pedestal tile with an identical allowed view → identical decision and rationale.
4. Viewer-safe explanations -> no-leak test over bot rationale and candidate rankings.

## What to Change

### 1. `src/bots.rs`

`MaskedClaimsRandomBot` (Level 0): uniform selection from the legal tree in both phases via deterministic bot RNG. `MaskedClaimsLevel1Bot` (Level 1): claim policy (honest default, parameterized bluff/underclaim selection, counting guard, stable tile-ID tie-break) and response policy (certain-lie detection by public counting, threshold challenge biased by declared-grade value, otherwise accept), each emitting viewer-safe explanations derived from public/own-seat state only.

## Files to Touch

- `games/masked_claims/src/bots.rs` (new)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Bot-strategy docs `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` (GAT11MASCLABLU-013).
- The `tests/bots.rs` integration suite and simulation registration (GAT11MASCLABLU-010/015).
- Any Level 2 authored-policy bot (out of gate scope per Assumption A9).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` bot tests pass: Level 0 and Level 1 select only legal action paths in both phases.
2. Level 1 decisions are deterministic under (seat view + bot seed) and unchanged when the hidden pedestal tile differs but the allowed view is identical.
3. No bot explanation references a hidden tile identity, opponent hand, or reserve.

### Invariants

1. Bots use the normal legal action API and allowed views only; no MCTS/ISMCTS/Monte Carlo/ML/RL (FOUNDATIONS §8).
2. Bot rationales and candidate rankings are viewer-safe (FOUNDATIONS §11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/bots.rs` `#[cfg(test)]` — legality, determinism, hidden-state independence (the `tests/bots.rs` end-to-end suite is added in GAT11MASCLABLU-010).

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. Unit-level boundary; many-game legality at scale is proven by `simulate` in GAT11MASCLABLU-015.
