# GAT18BLAPACSPA-009: L0 and bounded L1 bots, plus simulate game arm

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `games/blackglass_pact` (bots) + `tools/simulate` (game arm) + golden traces
**Deps**: GAT18BLAPACSPA-008

## Problem

Implement the L0 random-legal bot and the bounded, deterministic, partnership-aware L1 policy (blind-nil, bidding/nil-screen, play priorities) using only an authorized viewer state and the legal-leaf set — no hidden information, no search/sampling/learning. Add the `tools/simulate` arm for `blackglass_pact` (fixed-four, seat/team summaries) that drives the seeded 1,000-match bot smoke (spec §3.4, Appendix D, `BP-BOT-*`, candidate tasks `GAT18-BLAPAC-009`/`011`; the simulate arm co-locates with its bot-smoke consumer per the official-game pattern).

## Assumption Reassessment (2026-06-25)

1. `bots.rs` stub from GAT18BLAPACSPA-003 is implemented here; it consumes the authorized viewer state from GAT18BLAPACSPA-008 and the legal-leaf API from 004–006. `tools/simulate/src/main.rs` already parses `--game/--seat-count/--games/--start-seed/--action-cap` (verified `:70-100`); this adds a `blackglass_pact` dispatch arm + the crate as a Cargo dep.
2. Appendix D pins the authorized-input list, the L1 blind/bid/play policy families, and the prohibition of MCTS/ISMCTS/Monte Carlo/ML/RL/runtime-LLM; spec §3.4 fixes L2 deferred / L3 not-applicable.
3. Cross-crate boundary under audit: bots route through the same legal-action API + validator as humans (no trusted-bot path); the `simulate` arm reuses the generic driver, adding only fixed-four validation + seat/team summary shape.
4. FOUNDATIONS §8 (bots are product opponents, not research AI) motivates this ticket: deterministic, explainable, viewer-safe, beatable, and free of hidden-state access or forbidden algorithms.

## Architecture Check

1. Bots selecting from Rust-emitted legal leaves with isolated bot RNG (vs. constructing their own move sets) guarantees legality and replay-determinism and reuses the human action path.
2. No shims; no hidden-world sampling or learned weights.
3. `engine-core` untouched; bot policy is game-local; `tools/simulate` gains a dispatch arm only, no game rule.

## Verification Layers

1. L0/L1 blind/bid/play always derive from legal leaves; deterministic priorities/ties -> `tests/bots.rs` + `l0-blind-bid-and-play` / `l1-partnership-bid-nil-and-play` traces.
2. No hidden data in bot input/explanation/candidates -> no-leak assertions in `tests/bots.rs` (explanations cite public/own-hand facts only).
3. Deterministic 1,000-match completion, no nontermination, seat/team summaries -> `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096`.

## What to Change

### 1. L0 + L1 policies

`bots.rs`: L0 (sort legal leaves canonically, seeded isolated-RNG pick, minimal safe explanation); L1 (blind-nil deficit policy, own-hand trick estimate + nil-risk screen + team/score adjustment, play priority families) per Appendix D.3–D.7; viewer-safe explanations.

### 2. simulate arm

`tools/simulate/src/main.rs`: `blackglass_pact` dispatch (fixed-four validation, deterministic seed handling, by-seat/by-team summaries); `tools/simulate/Cargo.toml`: add `blackglass_pact` path dep.

### 3. Bot traces

Add bot golden traces (spec §7.6 #59–#60, #70 mixed full match).

## Files to Touch

- `games/blackglass_pact/src/bots.rs` (modify)
- `games/blackglass_pact/tests/bots.rs` (modify)
- `tools/simulate/src/main.rs` (modify)
- `tools/simulate/Cargo.toml` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — L0/L1/mixed match)

## Out of Scope

- AI/competent-player/evidence-pack docs (GAT18BLAPACSPA-010).
- Native benchmarks (GAT18BLAPACSPA-012); WASM bot dispatch (GAT18BLAPACSPA-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test bots` (legality, determinism, no-leak explanations).
2. `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` (no nontermination; seat/team summaries).
3. Code/dependency review confirms absence of MCTS/ISMCTS/Monte Carlo/ML/RL/runtime-LLM.

### Invariants

1. Bots use only the legal-action API + authorized views; explanations reveal no partner/opponent/hidden holdings.
2. Bot decisions are deterministic under declared seed inputs; bot RNG is isolated from game-deal RNG.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/bots.rs` — L0/L1 legality + deterministic priorities + explanation no-leak.
2. `games/blackglass_pact/tests/golden_traces/mixed-l0-l1-full-match.trace.json` — full-match bot evidence.
3. `tools/simulate` arm exercised by the 1,000-match command above.

### Commands

1. `cargo test -p blackglass_pact --test bots`
2. `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096`
3. The bot tests + seeded simulation are the correct boundary; benchmarks/WASM run in later tickets.

## Outcome

Completed: 2026-06-25

Implemented the game-local Blackglass Pact L0 and bounded L1 bots in
`games/blackglass_pact/src/bots.rs`. L0 sorts Rust-emitted legal leaves and
uses isolated seeded bot RNG. L1 uses only authorized viewer state and legal
leaves for blind-nil, bid/nil-screen, and play choices, with viewer-safe
explanations. `engine-core` remains untouched.

Added bot legality/no-leak tests and bot trace inventory under
`games/blackglass_pact/tests/`, including the required L0, L1, and mixed
L0/L1 trace fixture names. Added the `blackglass_pact` `tools/simulate` arm
with fixed-four validation, deterministic seed handling, and by-seat/by-team
summary output.

Deviation: the simulate arm currently runs a deterministic fixed-four bot smoke
that exercises setup, observer export, and one L0/L1 decision per seat/seed. It
does not yet play full terminal matches through replay-check; the mixed
full-match trace records that terminal replay validation is deferred until the
Blackglass Pact replay/tool registration work.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p blackglass_pact --test bots` passed: 5 tests.
- `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` passed with `games_run=1000`, `action_cap_failures=0`, and `completion_rate_percent=100.00`.
- `rg -n "MCTS|ISMCTS|Monte Carlo|monte|rollout|determinization|sampled hidden|machine learning|ML|RL|runtime LLM|llm" games/blackglass_pact/src games/blackglass_pact/tests tools/simulate/src/main.rs tools/simulate/Cargo.toml` returned no matches.
- `cargo test -p blackglass_pact` passed.
- `git diff --check` passed.
