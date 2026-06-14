# GAT15RIVLEDTEX-013: Bots L0/L1/L2 and bot registry doc

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/bots.rs`, `tests/bots.rs`, `src/lib.rs`, `docs/AI.md`; bot golden trace
**Deps**: GAT15RIVLEDTEX-008, GAT15RIVLEDTEX-012

## Problem

River Ledger needs three bots — L0 legal-random, L1 conservative public/own-hole heuristic, L2 limited opponent-count-aware authored policy — that choose only from the Rust legal-action tree using only the bot seat's authorized projection plus public state, and produce viewer-safe explanations. No search/learning class is allowed.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/bots.rs` (with `PokerLiteLevel2Bot`) is the precedent for the L0/L1/L2 shape consuming the legal-action API; this ticket consumes the legal-action tree from 005 and the authorized projections from 008.
2. `specs/...-base.md` §4.1 (`bots.rs`), §5 G15-RL-008, and the L2 priority vector authored in 012 fix `RL-BOT-*`; `docs/AI.md` is the bot registry/status doc.
3. Cross-artifact boundary under audit: the `BOT-STRATEGY-EVIDENCE-PACK.md` (012) maps each bot input to an authorized `visibility.rs` (008) view field; bots mutate no state and route through the same legal API as humans; the 6-seat bot-vs-bot trace exercises the full game.
4. FOUNDATIONS §8 (public bots) motivates this ticket: bots are competent, explainable, fair, deterministic under declared inputs, and beatable; L1/L2 read no raw internal state, deck tail, burn cards, or opponent hole cards.
5. §11 no-leak + bot-legality enforcement surface under audit: bot explanations and candidate rankings carry no hidden fact; bots use only legal actions from the authorized view; the hard exclusions (MCTS/ISMCTS/Monte Carlo/ML/RL/rollout/hidden-state sampling/omniscient ranks/solver) are absent. Confirm bot inputs derive only from the bot seat's projection plus public state.

## Architecture Check

1. Routing every bot through the shared legal-action API and authorized projection keeps the no-leak firewall and bot legality one boundary, matching the L2 sibling pattern.
2. No backwards-compatibility aliasing/shims — new module + new test file.
3. `engine-core` stays noun-free (§3); bot policies are crate-local in `games/*` (ai-core infra reused as-is); no `game-stdlib` promotion (§4).

## Verification Layers

1. Each bot chooses only from the legal-action tree -> `cargo test -p river_ledger --test bots` legality tests.
2. Bot explanations/candidate rankings leak no hidden fact -> bot no-leak tests (§11).
3. L1/L2 read no deck tail/burn/opponent holes/raw internal state -> input-source assertion tests (§8).
4. Seeded 3/4/5/6-seat bot playouts are deterministic -> bot simulation tests + the `bot-vs-bot-full-game-6p` golden trace (replay-validated after 015).

## What to Change

### 1. `games/river_ledger/src/bots.rs`

L0 legal-random; L1 conservative heuristic (own-hole strength, public board texture, call price, live-opponent count, street, cap pressure); L2 limited opponent-count-aware policy encoding the 012 priority vector with deterministic tie-breaks.

### 2. Tests + registry doc + trace

Create `tests/bots.rs` (legality, no-leak, input-source, seeded determinism at 3/4/5/6); author `docs/AI.md` (registry/status, information boundary, non-goals); add `bot-vs-bot-full-game-6p` golden trace.

## Files to Touch

- `games/river_ledger/src/bots.rs` (new)
- `games/river_ledger/tests/bots.rs` (new)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/docs/AI.md` (new)
- `games/river_ledger/tests/golden_traces/bot-vs-bot-full-game-6p.trace.json` (new)

## Out of Scope

- Native simulation registration + benchmarks (GAT15RIVLEDTEX-014).
- WASM/web bot dispatch (016/017); TypeScript bot decisions (forbidden, §2).
- Any MCTS/ISMCTS/Monte Carlo/ML/RL/solver (forbidden, §8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test bots` — legality, no-leak explanations, input-source, seeded determinism at 3/4/5/6.
2. Bots never consult internal deck order, opponent hole cards, burn cards, or raw internal trace fields.
3. `cargo test -p river_ledger` passes overall.

### Invariants

1. Bots use the normal legal action API and authorized views only (§8/§11).
2. No search/learning class; explanations/candidate rankings are no-leak (§8/§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/bots.rs` (new) — legality, no-leak, input-source, determinism.
2. `games/river_ledger/tests/golden_traces/bot-vs-bot-full-game-6p.trace.json` (new) — full 6-seat bot game evidence.

### Commands

1. `cargo test -p river_ledger --test bots`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. The bots test is the correct boundary; seeded full-game simulation throughput is exercised in GAT15RIVLEDTEX-014.
