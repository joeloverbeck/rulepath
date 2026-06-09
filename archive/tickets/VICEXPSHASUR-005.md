# VICEXPSHASUR-005: Target/score outcome rationale — `race_to_n` + `directional_flip`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `games/race_to_n` (`visibility.rs` rationale on the public terminal view) and `games/directional_flip` (`visibility.rs` rationale projecting the terminal trigger), per-game docs, and golden traces. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-003

## Problem

`race_to_n` exposes only `winner` on its `PublicView` with no terminal rationale; `directional_flip` already carries `final_score` in `TerminalView` but its terminal trigger (full board / double forced pass / no continuation) lives only in effects, so the web UI cannot explain *why* the match ended without reading the effect log. Both are "score/target reached the threshold" shapes and batch into one reviewable diff following the pilot (003). Source: `specs/victory-explanation-shared-surface.md` §9.1, §9.4.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `race_to_n` `PublicView { counter, target, max_add, winner, legal_additions }` (`games/race_to_n/src/visibility.rs:10-20`), `GameEnded` effect (`effects.rs`), **no** terminal rationale object. `directional_flip` `TerminalView { Win { winning_seat, final_score: ScoreView }, Draw { final_score: ScoreView }, NonTerminal }` (`games/directional_flip/src/visibility.rs:89-98`); `TerminalReason { BoardFull, NoContinuation, DoubleForcedPass }` (`effects.rs:62-66`) is carried by `GameEnded` but **not** by `TerminalView`. So `race_to_n` gains a new rationale on its public terminal view; `directional_flip` reuses `final_score` but must additionally project the terminal trigger (currently effect-only) into the rationale.
2. Spec §9.1/§9.4. Rule IDs: `race_to_n` uses `R-` (`R-SCORE-*`, `R-END-*` in `games/race_to_n/docs/RULES.md`); `directional_flip` uses `DF-`. Cite exact-target / final-score-comparison / terminal-trigger IDs.
3. Cross-artifact boundary under audit: rationale via `PublicView` (`race_to_n`) and `TerminalView` (`directional_flip`); round-trips `games/<g>/tests/golden_traces`; consumed by `apps/web/src/wasm/client.ts` (010).
4. FOUNDATIONS §2 restated: Rust owns exact-target detection and final-score comparison; TypeScript renders. TS MUST NOT compare scores or counters to decide the winner.
5. §11 determinism: both games are fully public (no hidden information), so no-leak is trivial; rationale fields serialize deterministically. The `directional_flip` terminal trigger projected into the view MUST equal the value `GameEnded` already carries — one canonical fact, now reachable through the view (so the panel reads the view, not the effect log, per spec §10.4), not a recomputed second source.
6. Schema extension: `race_to_n` adds a rationale to `PublicView`; `directional_flip` adds `terminal_trigger`/rationale to `TerminalView`. Consumers: golden traces (this ticket) + `client.ts` (010). Additive.

## Architecture Check

1. Projecting the existing facts (counter/target for `race_to_n`, `final_score`+trigger for `directional_flip`) as a game-local rationale (D2/D3) keeps the decisive cause as deterministic view data and makes the public view — not the effect log — the authoritative "why" source for the UI (spec §10.4).
2. No backwards-compatibility shims: additive projection; `final_score`/`TerminalReason` are reused, not aliased.
3. `engine-core` stays free of mechanic nouns (`counter`/`flip`/`disc` stay in the games); `game-stdlib` unchanged.

## Verification Layers

1. `race_to_n` exact-target cause → grep-proof `visibility.rs` carries the rationale; test asserts the win rationale names `counter_before`/`addition`/`counter_after`/`target` reaching the target exactly (`race_to_n.exact_target_reached`).
2. `directional_flip` trigger + score → test: the win/draw rationale names the final score per seat AND the terminal trigger (board full / double forced pass), and the trigger equals `GameEnded`'s `TerminalReason`.
3. Determinism → golden trace / replay-hash check for both games.
4. §2 behavior authority → manual/grep review: no TS score/counter comparison introduced.

## What to Change

### 1. `race_to_n` rationale (`games/race_to_n/src/visibility.rs`)

Project a terminal rationale reachable through the public view: result `Win { winning_seat }`; decisive cause = winning seat advanced the counter to the target exactly; breakdown `counter_before`, `addition`, `counter_after`, `target`, `max_add`; `decisive_rule_ids` = `R-` exact-target/terminal IDs; template key `race_to_n.exact_target_reached`. Add the `UI.md` outcome section per the 001 template; ensure `RULES.md` IDs.

### 2. `directional_flip` rationale (`games/directional_flip/src/visibility.rs`)

Project a rationale on `TerminalView`: result win/draw; decisive cause = final-score comparison after the terminal trigger; `terminal_trigger` = the `TerminalReason` value (`BoardFull`/`NoContinuation`/`DoubleForcedPass`); breakdown = final disc counts by seat + terminal reason; `decisive_rule_ids` = `DF-` final-score/trigger IDs; template keys `directional_flip.final_score_win`, `directional_flip.final_score_draw`. Add the `UI.md` outcome section; ensure `RULES.md` IDs.

### 3. Tests + golden traces

Add rationale unit tests for both games (exact-target win; full-board and double-pass terminal triggers + score); intentionally regenerate the affected golden traces.

## Files to Touch

- `games/race_to_n/src/visibility.rs` (modify)
- `games/race_to_n/docs/UI.md` (modify)
- `games/race_to_n/docs/RULES.md` (modify)
- `games/race_to_n/tests/golden_traces/` (modify — intentional regeneration)
- `games/directional_flip/src/visibility.rs` (modify)
- `games/directional_flip/docs/UI.md` (modify)
- `games/directional_flip/docs/RULES.md` (modify)
- `games/directional_flip/tests/golden_traces/` (modify — intentional regeneration)

## Out of Scope

- Any other game's rationale (003, 004, 006–008).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing TS score/counter-comparison logic or an `engine-core`/`game-stdlib` outcome type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n` and `cargo test -p directional_flip` pass, including new rationale tests.
2. The `race_to_n` win rationale names the exact-target reach; the `directional_flip` rationale names the final score per seat AND the terminal trigger.
3. `cargo run -p replay-check -- --game race_to_n --all` and `cargo run -p replay-check -- --game directional_flip --all` pass.

### Invariants

1. The decisive cause is Rust-projected view data; no downstream code compares scores/counters (FOUNDATIONS §2).
2. The `directional_flip` terminal trigger in the view equals the `GameEnded` `TerminalReason` (one canonical fact, no divergent second source); regenerated traces are replay-stable (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/` + `games/directional_flip/` rationale unit tests (exact-target; full-board / double-pass trigger + score).
2. `games/race_to_n/tests/golden_traces/` + `games/directional_flip/tests/golden_traces/` — intentionally regenerated traces.

### Commands

1. `cargo test -p race_to_n && cargo test -p directional_flip`
2. `cargo run -p replay-check -- --game race_to_n --all && cargo run -p replay-check -- --game directional_flip --all`
3. `cargo run -p fixture-check -- --game race_to_n && cargo run -p fixture-check -- --game directional_flip`
