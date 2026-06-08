# GAT9TOKBAZBRO-008: Level 0 random-legal + Level 1 TokenBazaarLevel1Bot + bot tests

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/token_bazaar/src/bots.rs` (new), `tests/bots.rs` (new), `src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-005

## Problem

Gate 9 requires a competent default browser bot, not merely random legal. This
ticket ships the Level 0 random-legal bot (baseline/simulation coverage) and the
default Level 1 `TokenBazaarLevel1Bot` — a deterministic heuristic that prefers
good public moves (fulfill the highest-point affordable contract, else collect
toward the best target, else a valuable exchange) with stable lexicographic
tie-breaks and a public-safe rationale. Both bots must route through the normal
legal-action API and use only public state.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/{rules,effects,actions}.rs` (GAT9TOKBAZBRO-004/005)
   provide the legal-action API + validation the bots call; `crates/ai-core`
   (`src/random_legal.rs`, verified present) provides the random-legal
   infrastructure the Level 0 bot reuses. The sibling
   `games/high_card_duel/src/bots.rs` + `tests/bots.rs` establish the house
   pattern (verified present) — but `high_card_duel` ships Level 0 only; Token
   Bazaar adds a Level 1 heuristic. `src/lib.rs` modified to add `mod bots;`.
2. The Level 1 policy is fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Level 1 bot requirements" (the five-tier priority + random-legal safety
   fallback; deterministic for a given public state; prefer lexicographic
   tie-breaks; rationale public-safe) and "Bot evidence must show…" (validates
   through normal command validation; deterministic; rationale present and
   exposes no candidate tables; ≥1 fixture fulfilling a contract; ≥1 fixture
   collecting toward an unaffordable target).
3. Cross-artifact boundary under audit: the bot↔legal-action-API contract. The
   bot must select only from the `legal_actions` tree and pass `validate_command`;
   it mutates no state directly. Its rationale is a public string, consumed by the
   AI docs (-017), the evidence pack (-017), and the browser (-014/-015).
4. FOUNDATIONS §8 + §11 (public bots; no-leak): no MCTS/ISMCTS/Monte Carlo/ML/RL
   and no hidden-state sampling (the game is public, so "allowed views" = the
   public view; the bot must still not fabricate omniscient/debug data). The bot
   is beatable, deterministic under declared inputs, and explainable. Restating
   the invariant before trusting the spec: the bot uses the same legal action API
   as humans and chooses through normal validation.
5. No-leak rationale firewall: the public rationale must not expose internal
   candidate tables, debug scores, hidden simulation, or omniscient information.
   This is the enforcement point; the no-leak suite (-009) and e2e (-016) assert
   the rationale leaks nothing — trivially safe here because all state is public,
   but the candidate/debug-table prohibition still binds.

## Architecture Check

1. Shipping Level 0 (reusing `ai-core::random_legal`) and Level 1 in one file with
   a shared selection harness keeps the bot surface cohesive and reviewable; the
   Level 1 heuristic is authored policy (allowed), not a search/learning class.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` untouched; no `game-stdlib` bot-policy primitive is created (the
   spec forbids a generic bot-policy helper). The heuristic stays game-local.

## Verification Layers

1. Bot legality (every selection validates) -> bot legality check: `tests/bots.rs`
   asserts each chosen action passes `validate_command` across multiple states/seeds.
2. Determinism (fixed state/seed → fixed choice) -> `tests/bots.rs` repeat-decision test.
3. Policy quality fixtures -> `tests/bots.rs` fixture asserting the bot fulfills a
   contract, and one asserting it collects toward an unaffordable target.
4. No-leak rationale -> `tests/bots.rs` asserts the rationale string contains no
   candidate/debug/internal field (full no-leak suite in -009).

## What to Change

### 1. `games/token_bazaar/src/bots.rs`

`TokenBazaarRandomBot` (Level 0, via `ai-core::random_legal`) and
`TokenBazaarLevel1Bot` (default): the five-tier priority with lexicographic
tie-breaks, the random-legal safety fallback (visible in tests, not reached in
normal fixtures), and a public-safe rationale per decision.

### 2. `games/token_bazaar/tests/bots.rs`

Legality across states/seeds, determinism, the two required quality fixtures, the
fallback-reachability test, and the rationale no-leak assertion.

### 3. `games/token_bazaar/src/lib.rs` (modify)

Add `mod bots;`; re-export both bots.

## Files to Touch

- `games/token_bazaar/src/bots.rs` (new)
- `games/token_bazaar/tests/bots.rs` (new)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- Any search/learning bot (MCTS/ISMCTS/Monte Carlo/ML/RL) — forbidden by the spec.
- A `game-stdlib` bot-policy primitive.
- Bot docs (`AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`) —
  GAT9TOKBAZBRO-017.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar --test bots` — every bot selection validates through
   normal command validation across multiple states/seeds.
2. `cargo test -p token_bazaar --test bots` — Level 1 decisions are deterministic
   for fixed state/seed.
3. `cargo test -p token_bazaar --test bots` — one fixture shows a contract
   fulfillment; one shows a collect toward an unaffordable visible contract.

### Invariants

1. Bots use only the legal-action API and the public view; no hidden-state
   sampling, no direct state mutation (§8/§11).
2. No MCTS/ISMCTS/Monte Carlo/ML/RL (§8).
3. The public rationale exposes no candidate table, debug score, or omniscient data.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/bots.rs` — legality, determinism, quality fixtures,
   fallback reachability, rationale no-leak.

### Commands

1. `cargo test -p token_bazaar --test bots`
2. `cargo test -p token_bazaar && bash scripts/boundary-check.sh`
3. Per-crate bot tests are the correct boundary; end-to-end bot turns in the
   browser are exercised by the e2e smoke (GAT9TOKBAZBRO-016).

## Outcome

Completed: 2026-06-08

What changed:

- Added `games/token_bazaar/src/bots.rs` with Level 0 seeded random-legal bot
  and deterministic Level 1 heuristic bot.
- Added `games/token_bazaar/tests/bots.rs` covering legality through normal
  validation, deterministic Level 1 choice, contract fulfillment, collect toward
  an unaffordable visible target, forced-pass fallback, and rationale no-leak.
- Added `ai-core` as a game-local dependency and updated `src/lib.rs` exports.

Deviations from original plan:

- None.

Verification results:

- `cargo test -p token_bazaar --test bots` passed with 7 tests.
- `cargo test -p token_bazaar` passed with 32 unit tests, 7 integration tests,
  and doc tests.
- `cargo build -p token_bazaar` passed.
- `bash scripts/boundary-check.sh` passed.
