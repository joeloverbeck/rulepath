# GAT1RACTON-007: Level 0 random legal bot (ai-core) + race_to_n wiring + bot tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/ai-core` gains a generic Level 0 random legal bot over the deterministic RNG contract + Rust-supplied legal paths; `games/race_to_n` wires it; bot legality + determinism tests.
**Deps**: GAT1RACTON-002, GAT1RACTON-005

## Problem

Every official game needs a Level 0 random legal bot (FOUNDATIONS §8;
OFFICIAL-GAME-CONTRACT §9). It belongs in `ai-core` (ARCHITECTURE §3: "random
legal bot" is an `ai-core` responsibility) as a **generic** helper that picks
uniformly among Rust-supplied legal action paths via the deterministic RNG
contract; `race_to_n` only wires it. The bot must use the normal legal action API
and be deterministic under fixed seed/view/limits (FOUNDATIONS §8/§11).

## Assumption Reassessment (2026-06-05)

1. `crates/ai-core/src/lib.rs` currently defines only `pub trait Bot { fn
   select_action(&self, viewer: &Viewer) -> Result<ActionPath, Diagnostic>; }`
   (verified). The generic random legal bot does not exist yet — additive. The
   deterministic RNG contract (GAT1RACTON-002) and `race_to_n` legal action
   generation (GAT1RACTON-005) are in place.
2. ARCHITECTURE §3 ownership table assigns "random legal bot, deterministic bot
   RNG helpers" to `ai-core`; AI-BOTS / AGENT-DISCIPLINE §9 require bots to use
   the same legal action API, choose through normal validation, mutate no state,
   use only the allowed view, and tie-break deterministically.
3. Cross-crate boundary under audit: the generic bot depends only on `engine-core`
   contracts (action tree, RNG, viewer) — never on `games/race_to_n` (so it stays
   game-agnostic). The wiring lives in `games/race_to_n/src/bots.rs` and feeds the
   game's legal action tree to the generic bot.
4. FOUNDATIONS §8 (bots are product opponents: fair, deterministic, beatable, use
   the legal action API, no hidden state) and §11 (bots use the normal legal
   action API and allowed views only; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL)
   motivate this ticket. This is a uniform random pick — no search, no learning.
5. Bot-legality enforcement surface: the bot tests are the §11 bot-legality
   enforcement surface. Confirm: the bot selects only paths present in the
   Rust-supplied legal action tree, routed through normal validation; it reads
   only the allowed view (perfect-information here, so no hidden-state risk, but
   the bot still takes a `Viewer`/view, not internal state); fixed seed/view/limits
   → identical choice (deterministic tie-break). No leakage path (the bot consumes
   the public view only).
6. Schema/contract: extends `ai-core` with a new concrete bot type implementing
   the existing `Bot` trait (additive). No existing consumer of `Bot` to break
   (greenfield). `race_to_n` is the first consumer via its bot registry/wiring.

## Architecture Check

1. A generic random-legal bot in `ai-core` (parameterized over the legal action
   tree + RNG contract) is reused by every future game — the correct home per
   ARCHITECTURE §3. Putting game-specific bot logic in `ai-core` would be a
   boundary error; putting the generic random bot in `games/*` would force every
   game to reimplement it. The wiring (game-specific) stays in `games/race_to_n`.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `ai-core` gains only a generic bot (no game noun, no
   strategy soup). `game-stdlib` untouched (§4).

## Verification Layers

1. Bot legality -> bot legality check (over many seeds/states, the bot's chosen
   path is in the legal action tree and passes normal validation — TESTING §10).
2. Determinism -> deterministic replay-hash check (fixed seed/view/limits →
   identical chosen path; deterministic tie-break — TESTING §10).
3. No hidden-state access -> FOUNDATIONS alignment check (§8/§11: bot consumes the
   allowed view, not internal state; mutates nothing).
4. Cross-crate: generic bot in `ai-core` depends only on `engine-core` (build
   proof: no `race_to_n` dependency in `ai-core/Cargo.toml`).

## What to Change

### 1. Generic random legal bot (`crates/ai-core`)

Add a generic random-legal bot that, given a legal action tree and a deterministic
RNG, picks a legal action path uniformly with deterministic tie-breaking. It
operates through the `Bot` trait / legal action API; it never mutates state.

### 2. race_to_n wiring (`games/race_to_n/src/bots.rs`)

Wire the generic bot to `race_to_n`'s legal action generation so a bot seat plays
to terminal. Author `games/race_to_n/docs/AI.md` (draft — Level 0 bot notes;
finalized in GAT1RACTON-014).

### 3. Bot tests

`games/race_to_n/tests/bot_tests.rs`: legality over many seeds, determinism for
fixed seed/view/limits, no direct state mutation.

## Files to Touch

- `crates/ai-core/src/lib.rs` (modify) — add the generic random legal bot
- `crates/ai-core/src/random_legal.rs` (new, optional — module split)
- `games/race_to_n/src/bots.rs` (new)
- `games/race_to_n/src/lib.rs` (modify) — register/expose the bot
- `games/race_to_n/tests/bot_tests.rs` (new)
- `games/race_to_n/docs/AI.md` (new — draft)

## Out of Scope

- Level 1+ bots (forbidden at Gate 1; spec §2).
- Simulation harness + benchmarks (GAT1RACTON-009/010) — they consume this bot.
- Replay/golden traces (GAT1RACTON-008) — the bot-action trace lives there.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p ai-core` — generic random-legal bot unit tests (uniform pick, deterministic tie-break) pass.
2. `cargo test -p race_to_n` — `bot_tests.rs`: bot legality over many seeds + determinism for fixed seed/view/limits pass.
3. `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Invariants

1. The bot chooses only legal action paths through normal validation; it mutates no state and reads only the allowed view (FOUNDATIONS §8/§11).
2. Fixed seed/view/limits → identical bot choice (deterministic; TESTING §10).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/bot_tests.rs` — legality (many seeds), determinism (fixed inputs), no-mutation.
2. `crates/ai-core/src/lib.rs` (or `random_legal.rs`) `#[cfg(test)]` — uniform selection + deterministic tie-break on a synthetic action tree.

### Commands

1. `cargo test -p ai-core -p race_to_n`
2. `cargo test --workspace`
3. `grep -n 'race_to_n' crates/ai-core/Cargo.toml` — expect zero hits (ai-core stays game-agnostic).
