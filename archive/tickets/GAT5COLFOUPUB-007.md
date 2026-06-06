# GAT5COLFOUPUB-007: Column Four bot strategy docs (Level 2 policy)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new docs `games/column_four/docs/COMPETENT-PLAYER.md`, `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (no code surfaces)
**Deps**: 001

## Problem

Spec §12 requires the Level 2 authored policy to be **documented before coding** in `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`. `column_four` is the first official game to reach Level 2 (spec A-3; `three_marks` stopped at Level 1, so only the `templates/*` versions exist). The bounded priority vector, tie-breaking, explanation format, and explicit exclusion of search/solver/ML must be authored first so the bot implementation (008) has an authoritative, FOUNDATIONS-clean policy spec.

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/docs/` contains no `COMPETENT-PLAYER.md` or `BOT-STRATEGY-EVIDENCE-PACK.md` (verified — it ships only RULES/MECHANICS/RULE-COVERAGE/UI/AI/BENCHMARKS/SOURCES/ADMISSION). The `templates/COMPETENT-PLAYER.md` and `templates/BOT-STRATEGY-EVIDENCE-PACK.md` exist and are the only starting point. This ticket fills them for `column_four`.
2. Spec §12.2 (Level 2 policy), §12.4 (allowed bounded evaluation), and §12.5 (bot explanations) define content: a bounded priority vector (win / block / safe / extend-threat / multi-threat / center-preference / deterministic-or-seeded tie-break), bounded one-ply successor evaluation only, viewer-safe rationale examples, and explicit exclusion of minimax/negamax/alpha-beta/MCTS/ISMCTS/Monte-Carlo/tablebase/ML/RL/LLM. Rules identity comes from GAT5COLFOUPUB-001.
3. Cross-artifact boundary under audit: the doc-governed bot contract in `docs/AI-BOTS.md` (Level definitions; Level 2 = authored policy) and `docs/OFFICIAL-GAME-CONTRACT.md` (bot evidence). This ticket produces policy prose conforming to them; the executable policy lands in 008.
4. FOUNDATIONS §8 (public bots are product opponents, not research AI) motivates this ticket. Restating before trusting the spec: bots MUST be competent, explainable, deterministic under declared inputs, beatable, and route through the legal action API; public v1/v2 MUST NOT use MCTS/ISMCTS/Monte-Carlo/ML/RL — any such future use needs an ADR. The doc must rule these out explicitly.

## Architecture Check

1. Authoring the policy before the bot code prevents the implementation from drifting into ad-hoc search and gives 008 an auditable priority vector — cleaner than backfilling the evidence pack after coding (rejected by spec §12.2).
2. No backwards-compatibility aliasing/shims — new docs.
3. No code surfaces; `engine-core` and `game-stdlib` untouched.

## Verification Layers

1. Policy-completeness invariant -> manual review against spec §12.2 priority vector and §12.4 bounded-evaluation limits.
2. Search-exclusion invariant -> FOUNDATIONS alignment check (§8): the docs explicitly exclude minimax/negamax/alpha-beta/MCTS/ISMCTS/Monte-Carlo/tablebase/ML/RL/LLM.
3. Explanation-safety invariant -> manual review against spec §12.5: rationale examples are viewer-safe public prose; bad examples (score arrays, candidate rankings, search-tree dumps) are named as forbidden.
4. Single-doc-pair ticket: layers map to the two surfaces (policy description vs. evidence/exclusion pack); no code-proof surface applies.

## What to Change

### 1. `games/column_four/docs/COMPETENT-PLAYER.md`

Original prose (from `templates/COMPETENT-PLAYER.md`): tactical principles of competent vertical four-in-a-row play in original words, translated into the bounded priority vector, tie-breaking rule (deterministic or seeded), and the viewer-safe explanation vocabulary.

### 2. `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

From `templates/BOT-STRATEGY-EVIDENCE-PACK.md`: the policy priorities with rationale, the bounded one-ply evaluation justification, explanation examples (good vs. bad), determinism-under-seed statement, and the explicit search/solver/ML/LLM exclusion list with the FOUNDATIONS §8 citation.

## Files to Touch

- `games/column_four/docs/COMPETENT-PLAYER.md` (new)
- `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- The bot implementation `games/column_four/src/bots.rs` (GAT5COLFOUPUB-008).
- `AI.md` (lands in GAT5COLFOUPUB-017, citing the implemented bot registry).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/column_four/docs/COMPETENT-PLAYER.md && test -f games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` — both files exist.
2. `grep -niE "minimax|negamax|alpha-beta|mcts|ismcts|monte carlo|tablebase|ml/rl|llm" games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` — the exclusion list is present.
3. Manual review: the priority vector and tie-break match spec §12.2; explanation examples are viewer-safe.

### Invariants

1. The documented policy is bounded (priority vector + one-ply evaluation), deterministic under declared inputs, and search/solver/ML/LLM-free.
2. Explanation guidance forbids exposing score arrays, candidate rankings, or search internals.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep/test -f) plus the manual policy/IP review named in Assumption Reassessment.`

### Commands

1. `test -f games/column_four/docs/COMPETENT-PLAYER.md && test -f games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
2. `grep -niE "priority|win|block|center|tie-break|seed" games/column_four/docs/COMPETENT-PLAYER.md`
3. A grep/manual boundary is correct: no compiled surface exists yet; bot legality is exercised in 008/009.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/column_four/docs/COMPETENT-PLAYER.md` with original strategy analysis, rules cross-checks, immediate tactics, threat/block descriptions, center preference, risk posture, visible signals, no-hidden-info boundary, examples, anti-examples, and implied bot tests.
- Added `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` defining the `column_four_tactical_v1` Level 2 authored policy input view, legal action API, candidate extraction plan, phase model, lexicographic priority vector, bounded tie-breaks, deterministic seeded tie-break, explanation contract, exclusions, known weaknesses, tests, and benchmark budget.

Deviations from original plan:

- None. This ticket remained documentation-only; executable bot implementation and legality/determinism tests remain in GAT5COLFOUPUB-008/009.

Verification results:

- Passed: `test -f games/column_four/docs/COMPETENT-PLAYER.md && test -f games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- Passed: `grep -niE "minimax|negamax|alpha-beta|mcts|ismcts|monte carlo|tablebase|ml/rl|llm" games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- Passed: `grep -niE "priority|win|block|center|tie-break|seed" games/column_four/docs/COMPETENT-PLAYER.md`
- Passed: `node scripts/check-doc-links.mjs`
- Manual policy review: priority vector and tie-breaks match the Gate 5 spec; explanations are viewer-safe and explicitly forbid score arrays, candidate rankings, and search internals.
