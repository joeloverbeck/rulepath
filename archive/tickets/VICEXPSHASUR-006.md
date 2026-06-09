# VICEXPSHASUR-006: Terminal-reason + hidden-info outcome rationale — `draughts_lite` + `high_card_duel`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite` (project the terminal reason into `TerminalView`) and `games/high_card_duel` (final-score rationale from revealed public history), per-game docs, and golden traces. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-003

## Problem

`draughts_lite` computes the decisive terminal reason (`OpponentNoPieces`/`OpponentNoLegalMove`) but only in effects — `TerminalView` exposes just `Win { winning_seat }`, so the UI cannot say *why* the win occurred. `high_card_duel` has the score and the revealed round history but `TerminalView` only distinguishes win/draw, with no final-score rationale — and as a hidden-information game it carries a no-leak obligation (the unrevealed deck tail must never appear). These two batch into one diff: each projects a rationale from data Rust already has, and `high_card_duel` anchors the hidden-info no-leak pattern. Source: `archive/specs/victory-explanation-shared-surface.md` §9.5, §9.6.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `draughts_lite` `TerminalWinReason { OpponentNoPieces, OpponentNoLegalMove }` (`games/draughts_lite/src/effects.rs:74-77`) carried by the terminal effect, but `TerminalView { NonTerminal, Win { winning_seat } }` (`visibility.rs:53-56`) does **not** carry the reason — it must be projected into the public terminal view. `high_card_duel` `RoundScored` effect (`effects.rs:33`); `PublicView { score, revealed_cards: Vec<RevealedRoundView>, … }` (`visibility.rs:13-35`); `TerminalView { NonTerminal, Win { winning_seat }, Draw }` (`visibility.rs:74-78`) — the final-score rationale is projected from the already-public revealed history.
2. Spec §9.5/§9.6. Rule IDs: `draughts_lite` uses `DL-` (no-pieces / no-legal-moves terminal IDs); `high_card_duel` uses `HCD-` (round scoring / round limit / final-score-draw IDs).
3. Cross-artifact boundary under audit: rationale via `TerminalView` (both); `draughts_lite` pulls the reason from the effect into the view (canonical end-state = the view rationale, so the panel does not read the effect log per spec §10.4); `high_card_duel` reuses the public `revealed_cards` history; both round-trip golden traces and feed `client.ts` (010).
4. FOUNDATIONS §2 restated: Rust owns terminal-reason detection and final-score computation; TypeScript renders. TS MUST NOT recompute the terminal reason or compare round cards.
5. §11 no-leak firewall (the load-bearing check here): `high_card_duel` is a hidden-information game — the rationale may use **only** the revealed round history; the unrevealed deck order/tail MUST NOT appear in the breakdown, the view, the effect log, or any trace export, even in the expanded breakdown. `draughts_lite` is fully public (piece counts public), so its no-leak is trivial. Both serialize deterministically (golden-trace migration).
6. Schema extension: `draughts_lite` adds the terminal reason to `TerminalView`; `high_card_duel` adds a final-score rationale to `TerminalView`. Consumers: golden traces (this ticket) + `client.ts` (010). Additive.

## Architecture Check

1. Projecting `draughts_lite`'s effect-known reason into the public view (D3) makes the view — not the effect log — the authoritative "why" source (spec §10.4); `high_card_duel` reuses the public revealed history rather than touching hidden state. Both are game-local rationales (D2), keeping the kernel noun-free.
2. No backwards-compatibility shims: additive projection; `TerminalWinReason`/`revealed_cards` reused, not aliased.
3. `engine-core` stays free of mechanic nouns (`piece`/`card`/`deck` stay in the games); `game-stdlib` unchanged.

## Verification Layers

1. `draughts_lite` reason projected → grep-proof `visibility.rs` carries the reason on `TerminalView`; test asserts the win rationale names `opponent_no_pieces` / `opponent_no_legal_move` with remaining piece counts and the losing seat's zero legal-move count, and that the value equals the effect's `TerminalWinReason`.
2. `high_card_duel` final-score cause → test: the win/draw rationale names the final score per seat and the per-round comparisons drawn from `revealed_cards`.
3. `high_card_duel` no-leak → no-leak visibility test: the public observer payload contains only revealed round history; no unrevealed deck card/order appears in the view, breakdown, or trace export.
4. Determinism → golden trace / replay-hash check for both games (`docs/TESTING-REPLAY-BENCHMARKING.md`).
5. §2 behavior authority → manual/grep review: no TS terminal-reason or card comparison introduced.

## What to Change

### 1. `draughts_lite` rationale (`games/draughts_lite/src/visibility.rs`)

Project a rationale on `TerminalView`: result win; decisive cause = opponent had no pieces / opponent had no legal move (the `TerminalWinReason` value); breakdown = remaining piece counts by seat, king/regular counts where already public, the to-act side's legal-move count (zero) at terminal; `decisive_rule_ids` = `DL-` no-pieces/no-legal-moves IDs; template keys `draughts_lite.opponent_no_pieces`, `draughts_lite.opponent_no_legal_move`. Add the `UI.md` outcome section per the 001 template; ensure `RULES.md` IDs.

### 2. `high_card_duel` rationale (`games/high_card_duel/src/visibility.rs`)

Project a rationale on `TerminalView` from public history: result win/draw; decisive cause = final score after the round limit; breakdown = each revealed round (ranks, round winner/tie, point delta, cumulative/final score); tiebreaker = none (equal final score is a draw); `decisive_rule_ids` = `HCD-` round-scoring/round-limit/final-score-draw IDs; template keys `high_card_duel.final_score_win`, `high_card_duel.final_score_draw`. Add the `UI.md` outcome section including the explicit "never expose unrevealed deck order/tail" redaction rule; ensure `RULES.md` IDs.

### 3. Tests + golden traces

Add rationale unit tests (both terminal reasons for `draughts_lite`; final-score win + tie/draw for `high_card_duel`) and a `high_card_duel` no-leak test; intentionally regenerate the affected golden traces (including the `draughts_lite` `terminal-no-pieces` / `terminal-no-legal-moves` traces and the `high_card_duel` hidden-info public-observer trace).

## Files to Touch

- `games/draughts_lite/src/visibility.rs` (modify)
- `games/draughts_lite/docs/UI.md` (modify)
- `games/draughts_lite/docs/RULES.md` (modify)
- `games/draughts_lite/tests/golden_traces/` (modify — intentional regeneration)
- `games/high_card_duel/src/visibility.rs` (modify)
- `games/high_card_duel/docs/UI.md` (modify)
- `games/high_card_duel/docs/RULES.md` (modify)
- `games/high_card_duel/tests/golden_traces/` (modify — intentional regeneration)

## Out of Scope

- Any other game's rationale (003–005, 007–008).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing TS terminal-reason / card-comparison logic or an `engine-core`/`game-stdlib` outcome type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` and `cargo test -p high_card_duel` pass, including new rationale tests and the `high_card_duel` no-leak test.
2. The `draughts_lite` win rationale names the terminal reason (equal to the effect's `TerminalWinReason`); the `high_card_duel` rationale exposes only revealed round history (no unrevealed deck card in any payload or trace export).
3. `cargo run -p replay-check -- --game draughts_lite --all` and `cargo run -p replay-check -- --game high_card_duel --all` pass.

### Invariants

1. The decisive cause is Rust-projected view data; `draughts_lite`'s view reason equals the effect's `TerminalWinReason` (one canonical fact); no downstream code recomputes it (FOUNDATIONS §2).
2. `high_card_duel` leaks no unrevealed deck order/tail through view, breakdown, effect log, or replay export (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/` rationale tests (both terminal reasons) + regenerated `terminal-no-pieces` / `terminal-no-legal-moves` traces.
2. `games/high_card_duel/` rationale + no-leak test + regenerated hidden-info public-observer trace.

### Commands

1. `cargo test -p draughts_lite && cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game draughts_lite --all && cargo run -p replay-check -- --game high_card_duel --all`
3. `cargo run -p fixture-check -- --game draughts_lite && cargo run -p fixture-check -- --game high_card_duel`
