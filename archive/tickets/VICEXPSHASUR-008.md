# VICEXPSHASUR-008: Tiebreak-ladder + hidden-info outcome rationale — `secret_draft`

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft` (`visibility.rs` five-rung ladder rationale on `TerminalView` from the existing `TieBreakSummary`), per-game docs, and golden traces. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-003

## Problem

`secret_draft` resolves ties through a five-rung ladder (total score → complete sets → highest single drafted value → distinct represented threads → fewer priority-won contested items → draw), and Rust already computes a `TieBreakSummary`, but `TerminalView` exposes only winner/draw — so the UI cannot say which rung decided the draft. As a commitment/reveal hidden-information game it also carries a no-leak obligation: nothing pre-reveal may surface early. This is the deepest ladder and the second hidden-info game, so it gets its own reviewable diff following the pilot (003). Source: `specs/victory-explanation-shared-surface.md` §9.8.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `TieBreakSummary { scores: [u32;2], complete_sets: [u8;2], highest_single_values: [u8;2], distinct_threads: [u8;2], priority_conflict_wins: [u8;2] }` (`games/secret_draft/src/effects.rs:70-76`); the `Terminal` effect carries `outcome`/`final_scores`/`tie_break_summary` (`effects.rs:46-50`); the ladder is `determine_terminal_outcome_from_summary` (`rules.rs:327-351`: total score → complete sets → highest single → distinct threads → fewer priority-conflict-wins → draw); `TerminalView { NonTerminal, Win { winning_seat }, Draw }` (`visibility.rs:75-79`) carries none of it — the five-rung ladder + decisive rung must be projected into the public terminal view from the already-computed `TieBreakSummary`.
2. Spec §9.8. Rule IDs use the `SD-` prefix; the rationale cites the scoring-component IDs, the reveal-completion (sixth reveal / draft completion) ID, and each tie-break rung ID.
3. Cross-artifact boundary under audit: rationale via `TerminalView` (`visibility.rs`); ladder values from `TieBreakSummary` (already computed in `rules.rs`/effects); round-trips `games/secret_draft/tests/golden_traces`; consumed by `apps/web/src/wasm/client.ts` (010).
4. FOUNDATIONS §2 restated: Rust owns the tiebreak ladder and selects the decisive rung; TypeScript renders the marked ladder. TS MUST NOT compute any tie-break rung.
5. §11 no-leak firewall (load-bearing): `secret_draft` hides commitments pre-reveal. The rationale may use drafted/revealed facts **only after all six reveals complete** (terminal). Pre-reveal choices, commitments, and unavailable hidden values MUST NOT appear early or in any view, breakdown, effect log, replay export, DOM, or test ID. The rationale serializes deterministically (golden-trace migration); the five-rung ladder + decisive rung equal Rust's `determine_terminal_outcome_from_summary` resolution.
6. Schema extension: the rationale (five-rung ladder + decisive-rung marker + reveal-completion trigger) is added to `TerminalView`, sourced from `TieBreakSummary`. Consumers: golden traces (this ticket) + `client.ts` (010). Additive.

## Architecture Check

1. Projecting the existing `TieBreakSummary` as an ordered five-rung ladder with the decisive rung marked (spec §9.8) lets the UI explain "won on the third tiebreaker" with zero TS ladder logic; the marker reuses Rust's resolution (D2/D3).
2. No backwards-compatibility shims: additive projection; `TieBreakSummary`/`determine_terminal_outcome_from_summary` reused, not aliased.
3. `engine-core` stays free of mechanic nouns (`draft`/`thread`/`set` stay in the game); `game-stdlib` unchanged.

## Verification Layers

1. Five-rung ladder + decisive rung projected → grep-proof `visibility.rs` carries the rationale; per-rung unit tests assert the decisive rung is marked correctly (total-score win, complete-sets, highest-single, distinct-threads, fewer-priority-conflict-wins, all-tied draw).
2. Final standing → test: the rationale names total score, complete sets, highest single value, distinct threads, and priority-conflict wins per seat, plus the drafted/revealed item list public at terminal.
3. No-leak → no-leak visibility test: the public-observer payload contains no pre-reveal commitment/choice/hidden value in the view, the expanded breakdown, or trace export.
4. Determinism → golden trace / replay-hash check (`docs/TESTING-REPLAY-BENCHMARKING.md`).
5. §2 behavior authority → manual/grep review: no TS ladder/tiebreak computation introduced.

## What to Change

### 1. `secret_draft` rationale (`games/secret_draft/src/visibility.rs`)

Project a rationale on `TerminalView`: result win/draw; `terminal_trigger` = sixth reveal / draft completion; decisive cause = total score or the first tie-break rung that differs (all-tied → draw); final standing = total score, complete sets, highest single value, distinct threads, priority-conflict wins per seat; breakdown = the ordered five-rung ladder with the decisive rung marked + the drafted/revealed item list public at terminal; `decisive_rule_ids` = `SD-` scoring/reveal-completion/tie-break IDs; template keys `secret_draft.score_win`, `secret_draft.complete_sets_tiebreak`, `secret_draft.highest_single_tiebreak`, `secret_draft.distinct_threads_tiebreak`, `secret_draft.fewer_priority_conflict_wins_tiebreak`, `secret_draft.all_tied_draw`. Source the rungs from `TieBreakSummary`; reuse the rung `rules.rs` selects. Add the `UI.md` outcome section per the 001 template, including the "no pre-reveal commitment appears early" redaction rule.

### 2. Tests + golden traces

Add per-rung rationale unit tests and a public-observer no-leak test; intentionally regenerate the affected `games/secret_draft/tests/golden_traces` (including the existing terminal tie-break and public-observer no-leak traces, now asserting the new view fields).

## Files to Touch

- `games/secret_draft/src/visibility.rs` (modify)
- `games/secret_draft/docs/UI.md` (modify)
- `games/secret_draft/docs/RULES.md` (modify — only if a cited scoring/reveal/tie-break rule needs a stable ID it lacks)
- `games/secret_draft/tests/golden_traces/` (modify — intentional regeneration)

## Out of Scope

- Any other game's rationale (003–007).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing TS tiebreak-ladder logic or an `engine-core`/`game-stdlib` outcome/scoring type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` passes, including per-rung rationale tests and the public-observer no-leak test.
2. Each per-rung test marks the correct decisive rung (equal to `determine_terminal_outcome_from_summary`); no pre-reveal commitment/hidden value appears in any payload, breakdown, or trace export.
3. `cargo run -p replay-check -- --game secret_draft --all` passes.

### Invariants

1. The decisive rung is Rust-selected view data; no downstream code computes the ladder (FOUNDATIONS §2).
2. The rationale exposes only post-reveal drafted/revealed facts; no pre-reveal commitment leaks through view, breakdown, effect log, replay export, or test ID (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/` per-rung rationale tests + public-observer no-leak test.
2. `games/secret_draft/tests/golden_traces/` — intentionally regenerated terminal tie-break + public-observer traces.

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`
3. `cargo run -p fixture-check -- --game secret_draft`

## Outcome

Implemented the `secret_draft` terminal rationale on Rust `TerminalView`.
Terminal wins and draws now carry `OutcomeRationaleView` with result kind,
sixth-reveal terminal trigger, decisive cause/template key, decisive rule IDs,
public per-player final standing, and the ordered ladder for score, complete
sets, highest single value, distinct threads, fewer priority-conflict wins, and
all-tied draw.

The rationale is terminal-only and sourced from Rust `TieBreakSummary` via
`terminal_tie_break_summary`; TypeScript was not changed and does not compute
any rung. Public standing includes only terminal public drafted items and public
tie-break facts. Visibility tests cover every decisive rung plus all-tied draw
and assert the pre-reveal terminal surface does not create a hidden rationale.

Updated `games/secret_draft/docs/UI.md` and `games/secret_draft/docs/RULES.md`
with the outcome/victory explanation contract, rule IDs, no-leak constraints,
and web smoke responsibility. Updated the two affected terminal golden traces
(`terminal-tie-break` and `draw-after-tie-breaks`) for the intentional terminal
view/public-export hash drift.

Verification run:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p secret_draft` — passed.
3. `cargo run -p replay-check -- --game secret_draft --all` — passed.
4. `cargo run -p fixture-check -- --game secret_draft` — passed.
5. `node scripts/check-doc-links.mjs` — passed.
6. `git diff --check` — passed.
7. `env RULEPATH_OUTCOME_GAME_IDS=secret_draft node scripts/check-outcome-explanations.mjs` — expected failure only for out-of-scope web follow-up surfaces: `apps/web/src/wasm/client.ts` lacks the outcome rationale mirror, and `apps/web/src/components/outcomeExplanationTemplates.ts` is not present yet.
