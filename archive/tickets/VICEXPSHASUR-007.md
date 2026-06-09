# VICEXPSHASUR-007: Tiebreak-ladder outcome rationale — `token_bazaar`

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar` (`visibility.rs` ladder + decisive-rung rationale on `TerminalView`), per-game docs, and golden traces / terminal fixtures. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-003

## Problem

`token_bazaar` resolves ties through a three-rung ladder (higher score → more fulfilled contracts → higher total remaining inventory → draw), and the facts live in the terminal effect, but `TerminalView` exposes only winner/draw — so the UI cannot say *which rung* decided the match. This is the first multi-rung-ladder retrofit; it gets its own reviewable diff (the per-rung fixtures alone are substantial) following the pilot (003). Source: `archive/specs/victory-explanation-shared-surface.md` §9.7.

## Assumption Reassessment (2026-06-09)

1. Verified current code: the tiebreak ladder is computed in `games/token_bazaar/src/rules.rs:127-163` (higher score lines 128-132 → `fulfilled_counts()` lines 139-149 → `inventory_totals()` lines 151-162 → draw); the terminal effect carries outcome/scores/fulfilled/inventory, but `TerminalView { NonTerminal, Win { winning_seat }, Draw }` (`visibility.rs:78-82`) carries none of it — the ladder + decisive rung must be projected into the public view.
2. Spec §9.7. Rule IDs verified present in `games/token_bazaar/docs/RULES.md`: `TB-SCORE-001` (score is the first tiebreak, L193), `TB-SCORE-004` (fulfilled count, L196), `TB-SCORE-005` (total remaining inventory tiebreak, L197), `TB-END-001` (turn-cap, L203), `TB-END-002` (market exhaustion, L204), `TB-END-003` (tiebreak order, L205). The rationale cites the rung IDs and the terminal-trigger IDs.
3. Cross-artifact boundary under audit: rationale via `TerminalView` (`visibility.rs`); the ladder values are already computed by `rules.rs`/effects; round-trips `games/token_bazaar/tests/golden_traces`; consumed by `apps/web/src/wasm/client.ts` (010).
4. FOUNDATIONS §2 restated: Rust owns the tiebreak ladder and selects the decisive rung; TypeScript renders the marked ladder. TS MUST NOT select the decisive rung (no `resolveTiebreak` equivalent).
5. §11 determinism: `token_bazaar` is fully public, so no-leak is trivial; the rationale serializes deterministically, and the decisive-rung marker equals `rules.rs`'s ladder resolution (one canonical fact, no recomputation).
6. Schema extension: the rationale (ordered ladder + decisive-rung marker + terminal trigger) is added to `TerminalView`. Consumers: golden traces / terminal fixtures (this ticket) + `client.ts` (010). Additive.

## Architecture Check

1. Projecting the full ordered ladder with the decisive rung marked (spec §9.7) — rather than just the winner — lets the UI explain "won on the second tiebreaker" without any TS ladder logic; the marker reuses the rung `rules.rs` already selected (D2/D3).
2. No backwards-compatibility shims: additive projection; `fulfilled_counts()`/`inventory_totals()` reused, not aliased.
3. `engine-core` stays free of mechanic nouns (`contract`/`inventory`/`market` stay in the game); `game-stdlib` unchanged.

## Verification Layers

1. Ladder + decisive rung projected → grep-proof `visibility.rs` carries the rationale; unit tests assert the decisive rung is marked correctly for each ladder outcome.
2. Per-rung coverage → deterministic terminal fixtures for score win, fulfilled-count tiebreak, inventory-total tiebreak, and all-tied draw; each fixture's rationale names the correct decisive rung and the per-seat score/fulfilled/inventory standing.
3. Terminal trigger → test: the rationale names turn-cap (`TB-END-001`) vs market-exhaustion (`TB-END-002`) as applicable.
4. Determinism → golden trace / replay-hash check (`docs/TESTING-REPLAY-BENCHMARKING.md`).
5. §2 behavior authority → manual/grep review: no TS ladder selection introduced.

## What to Change

### 1. `token_bazaar` rationale (`games/token_bazaar/src/visibility.rs`)

Project a rationale on `TerminalView`: result win/draw; `terminal_trigger` = turn cap or market/contract exhaustion; decisive cause = the rung that decided (score / fulfilled-count / inventory-total / all-tied draw); final standing = score, fulfilled count, inventory total per seat; breakdown = the ordered ladder with the decisive rung marked; `decisive_rule_ids` = `TB-SCORE-001/004/005`, `TB-END-001/002/003` as applicable; template keys `token_bazaar.score_win`, `token_bazaar.fulfilled_tiebreak_win`, `token_bazaar.inventory_tiebreak_win`, `token_bazaar.all_tied_draw`. Reuse the rung `rules.rs` already selects. Add the `UI.md` outcome section per the 001 template.

### 2. Tests + fixtures + golden traces

Add deterministic terminal fixtures for the four outcomes (score win, fulfilled-count tiebreak, inventory-total tiebreak, all-tied draw) and rationale unit tests asserting the decisive rung; intentionally regenerate the affected `games/token_bazaar/tests/golden_traces`.

## Files to Touch

- `games/token_bazaar/src/visibility.rs` (modify)
- `games/token_bazaar/docs/UI.md` (modify)
- `games/token_bazaar/docs/RULES.md` (modify — only if a cited terminal/tiebreak rule needs a stable ID it lacks; the named IDs already exist)
- `games/token_bazaar/tests/golden_traces/` (modify — intentional regeneration + new per-rung terminal fixtures)

## Out of Scope

- Any other game's rationale (003–006, 008).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing TS tiebreak-ladder logic or an `engine-core`/`game-stdlib` outcome/scoring type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` passes, including new tests covering score win, fulfilled-count tiebreak, inventory-total tiebreak, and all-tied draw.
2. Each per-rung fixture's rationale names the correct decisive rung and the per-seat score/fulfilled/inventory standing; the marker equals `rules.rs`'s ladder resolution.
3. `cargo run -p replay-check -- --game token_bazaar --all` passes.

### Invariants

1. The decisive rung is Rust-selected view data; no downstream code selects it (FOUNDATIONS §2).
2. The decisive-rung marker equals the `rules.rs` ladder resolution (one canonical fact); regenerated traces are replay-stable (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/` rationale unit tests (the four ladder outcomes).
2. `games/token_bazaar/tests/golden_traces/` — intentionally regenerated traces + new per-rung terminal fixtures.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo run -p replay-check -- --game token_bazaar --all`
3. `cargo run -p fixture-check -- --game token_bazaar`

## Outcome

Completed 2026-06-09 in commit `0d3633b`.

Implemented the Rust-owned `token_bazaar` tiebreak-ladder rationale, including
decisive rung projection, terminal-trigger/final-standing breakdowns,
per-game documentation, tests, and intentional golden trace updates.

Acceptance evidence was re-proven by the final capstone:

- `cargo test --workspace` passed.
- `cargo run -p replay-check -- --game token_bazaar --all` passed.
- `node scripts/check-outcome-explanations.mjs` passed.
