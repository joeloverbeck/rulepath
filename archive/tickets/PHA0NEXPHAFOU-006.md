# PHA0NEXPHAFOU-006: AI-BOTS — N-player imperfect-information bot subsection

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/AI-BOTS.md` edit only; no `ai-core`/`games/*` code.
**Deps**: PHA0NEXPHAFOU-002

## Problem

`AI-BOTS.md` does not differentiate multi-opponent hidden information from heads-up hidden information. The Hold'Em rung and later trick/meld games create pressure to reach for Monte Carlo rollouts or opponent-hand sampling — exactly the search classes the public bot law forbids. The doctrine needs an explicit N-player imperfect-information section before the first 3+ seat hidden-info game is authored.

## Assumption Reassessment (2026-06-13)

1. Public bots use the same legal action API and allowed views only, mutate no state, and never use hidden information unavailable to their seat (FOUNDATIONS §8/§11); bot infrastructure lives in `crates/ai-core`. This ticket changes doctrine only — no bot code.
2. Docs: `docs/AI-BOTS.md` (bot levels, hidden-information safety, Level 2 evidence) and `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (viewer-scoped export). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target; the bot templates are edited separately in PHA0NEXPHAFOU-012.
3. Cross-artifact boundary under audit: the AI-BOTS doctrine + the no-leak boundary; shared surface = the set of legal inputs a bot may read for its authorized seat.
4. FOUNDATIONS principle restate: §8 (public bots exclude MCTS/ISMCTS/Monte Carlo/ML/RL; future use needs an ADR) and §11 (bots use allowed views only; explanations/candidate rankings must not leak). Edits are meaning-preserving clarifications.
5. Enforcement surface: §11 no-leak firewall (bot explanation, candidate rankings) and §8 forbidden search classes. The subsection clarifies both; it introduces no leakage path and is enforced by bot-legality and no-leak tests at each Gate 15+ hidden-info game.

## Architecture Check

1. Differentiating N-player imperfect-information bots up front prevents Monte Carlo / hidden-world-sampling creep more reliably than catching it per gate.
2. No backwards-compatibility aliasing/shims introduced.
3. No new bot search class is introduced (that would require an ADR per §8/§13); `engine-core` is untouched and stays noun-free.

## Verification Layers

1. N-player imperfect-info subsection present → manual review.
2. Forbidden-search-class assertion present → codebase grep-proof (`MCTS`/`Monte Carlo`/`ISMCTS`/`ML`/`RL`).
3. Per-viewer explanation redaction reads as a no-leak clarification → FOUNDATIONS alignment check (§8/§11).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/AI-BOTS.md`

Add an "N-player imperfect-information bots" subsection. **Legal sources**: the bot's own private view, public state, history visible to that bot, and legal inference. **Forbidden**: other seats' hidden cards, the deck tail, unredacted replay, DOM/state peeking, and MCTS/ISMCTS/Monte Carlo/ML/RL unless a later ADR permits it. **Required multi-opponent policy notes**: target opponent set, risk model, deterministic tie-breaks, and explanation redaction per viewer. State that multi-opponent "belief" is rule-of-thumb categories and public pot odds, not sampled hidden worlds.

## Files to Touch

- `docs/AI-BOTS.md` (modify)

## Out of Scope

- Editing `crates/ai-core` or any `games/*` bot code.
- Introducing a new bot search class (requires an ADR).
- Editing the bot templates (`GAME-AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`) — PHA0NEXPHAFOU-012 owns those.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/AI-BOTS.md` has an "N-player imperfect-information bots" subsection with the legal-source list, the forbidden-class assertion, and the required multi-opponent policy notes.
2. The forbidden-class assertion explicitly names MCTS/ISMCTS/Monte Carlo/ML/RL.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §8 forbidden-search-class rule is unchanged in meaning — the subsection reaffirms it for the N-player case.
2. Bot inputs stay within the authorized seat view; the no-leak firewall (§11) is not weakened.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "N-player|MCTS|Monte Carlo|per viewer" docs/AI-BOTS.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Added `docs/AI-BOTS.md` §4A, `N-player imperfect-information bots`. The new
subsection lists legal bot sources for 3+ seat hidden-info games, bans other
seats' hidden data, deck/wall tails, unredacted replays, DOM/dev/full-state
peeking, hidden-state-derived rankings, sampled hidden worlds copied from actual
hidden state, and the public v1/v2 excluded bot classes (MCTS, ISMCTS, Monte
Carlo, ML, RL, runtime LLM move selection). It requires Level 1+ multi-opponent
policy notes for target opponent set, risk model, visible inference facts,
deterministic tie-breaks, and explanation redaction per viewer.

Deviations from plan: none. This was documentation-only; no `ai-core`, game,
kernel, schema, or bot implementation changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `grep -niE "N-player|MCTS|Monte Carlo|per viewer" docs/AI-BOTS.md` confirmed
  the subsection and forbidden-class assertion.
- `rg -n "own authorized private view|unredacted replay|sampled hidden worlds|target opponent set|risk model|deterministic tie-breaks" docs/AI-BOTS.md`
  confirmed the legal-source, forbidden-source, and policy-note details.
