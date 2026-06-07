# GAT6DIRFLI-011: Bots — Level 0 random-legal & Level 2-lite authored policy

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/directional_flip/src/bots.rs` (Level 0 random-legal bot, Level 2-lite authored one-ply policy bot).
**Deps**: 005, 008, 010

## Problem

`directional_flip` needs a Level 0 random-legal bot and a Level 2-lite authored policy bot (FOUNDATIONS §8, spec §11). Both must use the same legal action API as humans, mutate no state directly, use no hidden information, be deterministic under seed/tie-break, and emit viewer-safe rationale. The Level 2-lite policy is a bounded one-ply evaluation over visible features (corners, X/C danger, mobility, stability, frontier, phase-aware disc count) — no search/ML/LLM. Realizes `DF-BOT-001`/`002`.

## Assumption Reassessment (2026-06-07)

1. `crates/ai-core/src/random_legal.rs` provides `RandomLegalBot` (used by `games/column_four/src/bots.rs` via `use ai_core::RandomLegalBot;` — confirmed in the column_four bots module, which also defines `RANDOM_POLICY_ID` and `LEVEL2_POLICY_ID` constants and a Level 2 tactical bot). `directional_flip`'s Level 0 bot consumes the same `ai-core` pattern. The action tree (GAT6DIRFLI-006) and effects (008, the `BotChoseAction` carrier) exist.
2. Spec §11.1 (required bots), §11.2 (allowed/forbidden boundary), §11.3 (lexicographic policy outline), and the strategy docs `games/directional_flip/docs/COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md` (GAT6DIRFLI-010) are authoritative. Rule ids `DF-BOT-001`/`002`.
3. Cross-crate boundary under audit: `games/directional_flip` ↔ `ai-core` (random-legal selection, seed contract) and ↔ the game's own legal action tree (006). The Level 2-lite bot must consume legal placements + forced pass through the normal command path and read only the public view (007) — never internal state.
4. FOUNDATIONS §8 motivates this ticket: restate before coding — public bots must be competent, explainable, fair, deterministic under declared inputs, and beatable; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL; bots use the normal legal action API and allowed views only.
5. This ticket touches the §11 bot-legality and no-leak invariants: confirm the bot routes every choice through the legal action API (no direct mutation), uses no hidden information unavailable to its seat, is deterministic by seed/tie-break, and that its rationale (carried by `BotChoseAction`, 008) is viewer-safe (no candidate-ranking or hidden-state leak — §11 no-leak firewall). Level 2-lite inspects only the immediate successor position's visible features (one ply), never opponent-reply search.

## Architecture Check

1. Reusing `ai-core`'s `RandomLegalBot` for Level 0 and building Level 2-lite as a lexicographic one-ply scorer over the public view keeps both bots inside the §8 boundary and makes the policy auditable against the GAT6DIRFLI-010 docs.
2. No backwards-compatibility shims; new bot module.
3. `engine-core` stays noun-free; no bot policy or hidden-information access is smuggled through shared helpers (atlas §9). The bot reads the public view only.

## Verification Layers

1. Bot legality -> bot legality check (`DF-BOT-001`/`002`): both bots select only actions present in the Rust action tree, validated through the normal command path, across many seeds/states.
2. Determinism -> rule test: fixed seed → identical choice; Level 2-lite tie-break is deterministic.
3. One-ply / no-search boundary -> FOUNDATIONS alignment check (§8) + manual review against spec §11.2: no minimax/alpha-beta/recursive/MCTS/ML/LLM; evaluation is bounded to the immediate successor.
4. Rationale no-leak -> no-leak visibility test: `BotChoseAction` rationale is viewer-safe; no candidate-ranking/hidden-state leak (FOUNDATIONS §11).

## What to Change

### 1. Level 0 random-legal bot

In `bots.rs`, wrap `ai_core::RandomLegalBot` over the `directional_flip` action tree; deterministic by seed; emits a `BotChoseAction` with a safe rationale and a stable policy id.

### 2. Level 2-lite authored policy

Implement the lexicographic one-ply policy of spec §11.3 over visible features only (forced-pass handling, immediate terminal preference, corners, X/C avoidance, opponent/own mobility, stable extensions, frontier caution, phase-aware disc-count tie-break, deterministic seeded final tie-break). Emit a safe rationale matching the documented policy.

## Files to Touch

- `games/directional_flip/src/bots.rs` (new)
- `games/directional_flip/src/lib.rs` (modify — export the bots module)

## Out of Scope

- `AI.md` documentation (GAT6DIRFLI-020) and the strategy docs (010, already landed as a Dep).
- Bot-action golden trace (GAT6DIRFLI-013) and bot benchmarks (014).
- Any TypeScript bot logic (forbidden; FOUNDATIONS §8, spec §11.2/§12.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — bot legality + determinism tests pass for both levels.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Both bots act only through the legal action API, use no hidden information, and are deterministic under seed (FOUNDATIONS §8, §11; `DF-BOT-001`/`002`).
2. Level 2-lite performs no opponent-reply search and no forbidden ML/LLM evaluation (FOUNDATIONS §8, spec §11.2).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/bots.rs` — Level 0 validates for many seeds/states; Level 2-lite is deterministic, legal, emits safe rationale, and follows the documented lexicographic order (expanded in GAT6DIRFLI-012).

### Commands

1. `cargo test -p directional_flip bots`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct; end-to-end bot-vs-random throughput is exercised by the simulation/benchmarks in GAT6DIRFLI-014/016.

## Outcome

Implemented `games/directional_flip/src/bots.rs` with a seeded Level 0 random-legal bot via `ai-core::RandomLegalBot` and a deterministic Level 2-lite authored policy. The Level 2-lite policy consumes Rust legal action paths, validates through the normal command path for successor evaluation, ranks bounded one-ply visible features, emits viewer-safe `BotChoseAction` rationale, and excludes search/playouts/ML/RL/LLM. Updated the strategy docs to include the implemented favorable-terminal and stable edge/corner extension priority slots.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p directional_flip bots`
3. `cargo test -p directional_flip`
4. `bash scripts/boundary-check.sh`
5. `node scripts/check-doc-links.mjs`
