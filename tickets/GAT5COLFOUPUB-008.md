# GAT5COLFOUPUB-008: Column Four bots — Level 0 random-legal & Level 2 tactical

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/column_four/src/bots.rs`
**Deps**: 003, 005, 007

## Problem

Gate 5 requires two bots: a Level 0 random-legal bot and a Level 2 authored tactical policy bot suitable for public demonstration (spec §12, ROADMAP "baseline and preferably Level 2"). Both must choose only from the Rust legal action tree, be deterministic under seed/state, emit viewer-safe rationale, and never use search/solver/ML.

## Assumption Reassessment (2026-06-06)

1. `crates/ai-core/src/random_legal.rs` exports the generic `RandomLegalBot` (`new(seed)` + `select_action(&ActionTree)`), reused by `ThreeMarksRandomBot` in `games/three_marks/src/bots.rs` (verified both). The Level 0 `column_four` bot reuses `RandomLegalBot`; the Level 2 bot mirrors the `ThreeMarksLevel1Bot` priority-policy shape but encodes the richer vector.
2. Spec §12.1 (Level 0), §12.2 (Level 2 priority vector), §12.4 (bounded one-ply evaluation), §12.5 (rationale) define behavior. The authored policy spec is GAT5COLFOUPUB-007's `COMPETENT-PLAYER.md`/`BOT-STRATEGY-EVIDENCE-PACK.md`; legal-action generation and successor states come from GAT5COLFOUPUB-003; the bot-chose-action effect/rationale payload is GAT5COLFOUPUB-005.
3. Cross-artifact boundary under audit: the `engine-core` legal-action-tree / `ActionPath` / `BotDecision` contract and the no-leak firewall. Bots produce an `ActionPath` through the normal legal API and a viewer-safe rationale; no candidate ranking or score array escapes.
4. FOUNDATIONS §8 (public bots) motivates this ticket: bots use the same legal action API as humans, mutate no state directly, use no hidden state, are deterministic under declared inputs and beatable; public v1/v2 exclude MCTS/ISMCTS/Monte-Carlo/ML/RL.
5. No-leak firewall (§11) and the §8 search-exclusion are the enforcement surfaces under audit: the Level 2 bounded one-ply evaluation MUST NOT become recursive search, and its rationale MUST carry only public facts (confirmed against the §12.5 good/bad examples) — no scores, candidate lists, or search trees reach the effect/trace.

## Architecture Check

1. Reusing `ai-core::RandomLegalBot` for Level 0 and an authored priority-vector policy for Level 2 keeps both bots on the legal-action API with zero new search infrastructure — cleaner and the only FOUNDATIONS §8-compliant design. Alternatives (minimax/MCTS) are forbidden (spec §6 non-goals, FOUNDATIONS §8/§13).
2. No backwards-compatibility aliasing/shims — new module; Level 0 delegates to the existing generic bot rather than copying it.
3. `engine-core` stays free of mechanic nouns (tactical evaluation is game-local in `bots.rs`); `game-stdlib` untouched — the tactical-threat-evaluation duplication vs. `three_marks` is recorded as pressure in GAT5COLFOUPUB-018, not extracted (FOUNDATIONS §4).

## Verification Layers

1. Legal-only invariant -> bot legality check: both bots choose only columns present in the Rust legal action tree; full/terminal states yield no illegal action.
2. Determinism invariant -> unit test: same state+seed yields the same Level 0 choice; the Level 2 policy is deterministic (priority order + declared tie-break).
3. Tactical-correctness invariant -> unit tests: Level 2 takes an immediate win, blocks an immediate loss, avoids handing an immediate win, and prefers center when no urgent tactic exists.
4. No-leak / no-search invariant -> no-leak visibility test + FOUNDATIONS alignment check (§8): rationale is viewer-safe prose; the implementation contains no recursive/minimax/MCTS search.
5. Validation-path invariant -> bot legality check: bot actions validate through the same Rust command path as human actions (GAT5COLFOUPUB-003).

## What to Change

### 1. `games/column_four/src/bots.rs`

Implement `ColumnFourRandomBot` (Level 0) delegating to `ai_core::RandomLegalBot` over the Rust legal action tree, and `ColumnFourLevel2Bot` (authored policy) applying the GAT5COLFOUPUB-007 priority vector via bounded one-ply successor evaluation (win-now / block / safe / extend / multi-threat / center / deterministic-or-seeded tie-break). Each returns a `BotDecision`/`ActionPath` plus a viewer-safe rationale string consumed by the bot-chose-action effect (005).

## Files to Touch

- `games/column_four/src/bots.rs` (new)

## Out of Scope

- The comprehensive cross-cutting test suite (GAT5COLFOUPUB-009 adds broader bot/property tests); this ticket carries the focused bot unit tests proving its own invariants.
- `AI.md` registry doc (GAT5COLFOUPUB-017) and bot golden traces (GAT5COLFOUPUB-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four bots` — legal-only, determinism, tactical-correctness, and no-leak rationale tests pass.
2. `cargo test -p column_four` — no regression to rules/view/effects.
3. `grep -niE "minimax|negamax|alpha_beta|mcts|ismcts|monte_carlo|tablebase" games/column_four/src/bots.rs` returns nothing — no search class present.

### Invariants

1. Both bots act only through the legal action tree, mutate no state directly, and use no hidden information.
2. Level 2 is deterministic under declared inputs and emits only viewer-safe rationale.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/bots.rs` (unit tests) — Level 0 legal+seeded determinism; Level 2 immediate win, immediate block, avoid concession, center preference, tie-break determinism, viewer-safe rationale.

### Commands

1. `cargo test -p column_four bots`
2. `cargo test -p column_four`
3. `cargo clippy -p column_four --all-targets -- -D warnings`
