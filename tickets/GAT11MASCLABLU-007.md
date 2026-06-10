# GAT11MASCLABLU-007: Conditional resolution, scoring, terminal, and tie-breaks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends `games/masked_claims/src/{rules.rs,effects.rs}`, `src/lib.rs`
**Deps**: GAT11MASCLABLU-006

## Problem

Resolution is conditional on the response choice and, only when challenged, on hidden information that reveals at exactly that moment. Accept scores the declared grade and moves the tile face-down to the veiled gallery without ever revealing it; challenge reveals the tile and applies graded honest/exposed scoring. The game ends after turn 8 with a deterministic public tie-break ladder. Accepted masks never reveal — including at terminal.

## Assumption Reassessment (2026-06-10)

1. `src/rules.rs` window logic and `src/effects.rs` from GAT11MASCLABLU-006 provide the apply-response point. Resolution follows spec §"Conditional resolution" and §"Terminal and tie-breaks".
2. Spec §"Semantic effect model" effects: `ClaimAccepted { turn, declared_grade, score_delta }`, `ChallengeDeclared { turn, responder }`, `MaskRevealed { turn, tile_id, actual_grade }` (the first and only public appearance of a pedestal tile's identity), `ChallengeResolved { turn, outcome, awards }`, `ScoreChanged`, `TurnAdvanced`, `Terminal`. Scoring constants per Assumption A4: accept = declared grade; honest challenge = actual + 2; exposed lie = claimant 0, challenger = declared − actual. Tie-break ladder: score → fewer exposed lies → more successful challenges → fewer challenges declared → Draw.
3. Cross-artifact boundary under audit: the effect-envelope visibility contract. `MaskRevealed` is the single point a tile ID becomes public, and only on challenge; the `Terminal` effect carries the winner/draw, final scores, and tie-break summary but no veiled-gallery, hand, or reserve identities.
4. FOUNDATIONS §2 (scoring, terminal detection, tie-breaks, and reveal timing are Rust-owned) and §11 (accepted masks never reveal, for the lifetime of the match — Assumption A5) are the principles under audit.
5. §11 no-leak firewall enforcement surface: reveal timing. Confirm a tile ID appears ONLY in `MaskRevealed` on a challenge; the accept path, the veiled gallery, the unplayed hand tile, the reserve, and the terminal view never reveal an identity.

## Architecture Check

1. Graded conditional resolution kept local, with reveal expressed as a semantic effect (`MaskRevealed`) rather than a view diff, keeps animation causality in Rust (§7) and the no-leak boundary airtight.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; resolution/scoring/tie-break logic is game-local typed Rust — no `game-stdlib` challenge-resolver or scoring helper.

## Verification Layers

1. Accept / honest-challenge / exposed-lie / underclaim resolution + scoring -> rule tests (full suite in GAT11MASCLABLU-010).
2. Terminal after turn 8 + the five-step tie-break ladder -> rule tests + golden traces `terminal-tie-break`, `draw-after-tie-breaks` (GAT11MASCLABLU-011).
3. Accepted mask never revealed (including at terminal) -> no-leak visibility test + golden trace `accepted-mask-never-revealed`.
4. Reveal only on challenge, via `MaskRevealed` -> no-leak test + reveal-ordering assertion.

## What to Change

### 1. `src/rules.rs` (resolution + terminal)

Accept: score declared grade, move pedestal tile face-down to the claimant's veiled gallery, never reveal. Challenge: emit `MaskRevealed`, grade honest (actual ≥ declared → actual + 2; underclaim pays full actual) vs exposed (actual < declared → claimant 0, challenger declared − actual), move the revealed tile to the appropriate exposed row. Turn advance with claimant alternation; terminal after turn 8; the deterministic tie-break ladder.

### 2. `src/effects.rs` (resolution effects)

`ClaimAccepted`, `ChallengeDeclared`, `MaskRevealed`, `ChallengeResolved`, `ScoreChanged`, `TurnAdvanced`, `Terminal` with the payload visibility rules above — `Terminal` reveals no veiled gallery, hand, or reserve.

## Files to Touch

- `games/masked_claims/src/rules.rs` (modify)
- `games/masked_claims/src/effects.rs` (modify)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Public/seat view projection, replay/export surfaces (GAT11MASCLABLU-008).
- Bots (GAT11MASCLABLU-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` resolution, scoring, terminal, and tie-break tests pass.
2. Accept never emits a tile ID; the veiled gallery is never revealed, including at terminal.
3. A challenged tile's identity appears first and only in `MaskRevealed`.

### Invariants

1. Scoring, terminal detection, reveal timing, and tie-breaks are deterministic Rust (FOUNDATIONS §2).
2. Accepted masks, unplayed hand tiles, and the reserve are never revealed for the match lifetime (FOUNDATIONS §11, Assumption A5).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/rules.rs` `#[cfg(test)]` — accept/honest/exposed/underclaim resolution, terminal, full tie-break ladder.
2. `games/masked_claims/src/effects.rs` `#[cfg(test)]` — `MaskRevealed` is challenge-only; `Terminal` carries no hidden identities.

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. Unit-level boundary; golden-trace resolution proofs and the export no-leak sweep are exercised in GAT11MASCLABLU-008/010/011.
