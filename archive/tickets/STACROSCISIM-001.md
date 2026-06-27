# STACROSCISIM-001: Per-game CI simulation game-count override; cap starbridge_crossing's Gate 1 lane

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — CI config only (`ci/games.json`, `.github/workflows/gate-1-game-smoke.yml`, `scripts/check-ci-games.mjs`). No `engine-core`/`game-stdlib`/`games/*` behavior, schema, trace, or doc-contract change.
**Deps**: None. (Sibling perf follow-up STACROSCISIM-002 is independent and not required to unblock CI.)

## Problem

The Gate 1 `starbridge_crossing` lane does not fail — it runs for ~37 minutes and reads as a hang. The `Quick simulation` step hardcodes `--games 1000` for every game (`.github/workflows/gate-1-game-smoke.yml:99`), but `starbridge_crossing` averages ~2000 actions/game versus ~60 for every other game (its `max_plies: 2000` turn limit at `games/starbridge_crossing/src/variants.rs:101` is reached almost every game under the L0 bot in a 6-seat star-Halma match). Measured locally: 0.45 games/sec debug → ~37 min for 1000 games. CI builds the tools in debug (`gate-1-game-smoke.yml:47`, no `--release`) on slower 2-core runners, so the wall-clock is at least this bad. Every other lane finishes in well under a minute; the worst current lane (`meldfall_ledger`, 1000 games) is 4m16s.

The 2000-ply turn limit is deliberate, spec'd game design (`archive/specs/gate-20-starbridge-crossing-star-halma.md:91` — "Default 2000 plies for public simulations and benchmarks") and is NOT the lever; lowering it would be a product-behavior change requiring an ADR. The correct surface is the CI simulation budget, which currently has no per-game override for the games count even though it already supports per-game `sim_flags`.

## Assumption Reassessment (2026-06-27)

1. The Gate 1 simulation step is `./ci-bin/simulate --game ${{ matrix.id }} --games 1000 ${{ matrix.sim_flags }}` (`.github/workflows/gate-1-game-smoke.yml:99`). `--games` is hardcoded; only `--seat-count`/`--action-cap` are passed per-game via `sim_flags` (`ci/games.json`). `simulate` already accepts `--games` (`tools/simulate/src/main.rs:214`); `DEFAULT_GAMES = 1000` (`main.rs:86`).
2. The CI matrix is the raw `ci/games.json` array, emitted verbatim by `scripts/check-ci-games.mjs --emit` (lines 78–84), so each entry's fields become `matrix.<field>`. The validator currently enforces exactly `id` (string), `sim_flags` (string), `e2e` (string) per entry (lines 37–54) and a set-equality drift check against `games/`; it rejects nothing extra but validates nothing extra either.
3. Cross-artifact boundary under audit: the `ci/games.json` schema ⇄ `scripts/check-ci-games.mjs` validator ⇄ `gate-1-game-smoke.yml` matrix consumer. Adding an optional `games` field touches all three; the game crates and `simulate` are untouched.
4. No FOUNDATIONS product-behavior principle is engaged — this changes test breadth on one CI lane, not legality, determinism, or views. It aligns with the gate-20 spec's directive (`archive/specs/gate-20-starbridge-crossing-star-halma.md:363`) that implementation "must measure native baselines, commit variance-aware CI floors"; no fixed "1000 games" doctrine exists in `docs/TESTING-REPLAY-BENCHMARKING.md` or `docs/FOUNDATIONS.md`.
8. Adjacent contradiction classified: the slow per-action cost from recursive jump-chain enumeration (`games/starbridge_crossing/src/actions.rs:288–308` clones `visited` per hop) is a *separate* performance concern, split into follow-up STACROSCISIM-002 — not required here.

## Architecture Check

1. A per-game `games` override mirrors the existing per-game `sim_flags` precedent — same data file, same emit path, smallest possible blast radius. Alternatives rejected: (a) lowering the game's `max_plies` mutates spec'd game behavior (ADR-gated); (b) building the tools in `--release` is a global change with a slower build job that leaves `starbridge_crossing` the slowest lane (the ~30× length factor remains); (c) lowering `--action-cap` below 2000 would force `capped` non-terminal games and corrupt the smoke's pass semantics.
2. No backwards-compatibility shim: the field is additive and optional. Entries without `games` fall back to `1000` via the workflow expression default, so the other 20 rows are unchanged.
3. `engine-core` is untouched (no mechanic nouns introduced); `game-stdlib` is untouched. This ticket edits CI config and one Node validator only.

## Verification Layers

1. Schema/validator conformance (optional `games` is a positive integer when present) -> `node scripts/check-ci-games.mjs` exits 0 on the edited manifest and exits non-zero on a planted `"games": 0` / `"games": "x"`.
2. Matrix-emit conformance (the new field reaches the matrix) -> `node scripts/check-ci-games.mjs --emit` prints a manifest whose `starbridge_crossing` entry carries `games`.
3. CI wall-clock floor (the lane lands under budget) -> the Gate 1 `starbridge_crossing` lane's `Quick simulation` step completes in ≤ ~4 min (at or below the current worst lane, `meldfall_ledger` 4m16s), confirmed on the PR run after measuring.
4. Smoke still meaningful -> the reduced count still drives the game end-to-end across distinct seeds without panic/drift (the simulation step exits 0).

## What to Change

### 1. `ci/games.json` — add the optional `games` override to the starbridge row

Add `"games": 50` (a starting value — see budget note) to the `starbridge_crossing` entry, alongside its existing `sim_flags`. Leave all other rows untouched (they inherit the 1000 default).

### 2. `.github/workflows/gate-1-game-smoke.yml` — consume the override with a default

Change line 99 from `--games 1000` to `--games ${{ matrix.games || 1000 }}`. Rows without a `games` field yield an empty `matrix.games`, so the `|| 1000` fallback preserves today's behavior for every other lane.

### 3. `scripts/check-ci-games.mjs` — validate the optional field

In the per-entry shape loop (around lines 46–53), add: if `entry.games` is present, require it to be an integer `> 0`, else push an error (`"<id>" must have a positive integer "games" when present`). Do not require it (absence is valid).

### Budget note (measure, don't guess)

CI runs debug binaries on 2-core runners, slower than a dev box, so `50` is a *starting* value, not a proven floor. The implementer must read the PR's Gate 1 `starbridge_crossing` lane timing and tune `games` (expected window ~25–100) so the `Quick simulation` step lands at or below ~4 min. Record the chosen value's measured lane time in the PR. If STACROSCISIM-002 later lands and speeds per-action cost, the count may be raised in a separate change.

## Files to Touch

- `ci/games.json` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `scripts/check-ci-games.mjs` (modify)

## Out of Scope

- Lowering `starbridge_crossing`'s `max_plies` / turn limit, or any game-rule/terminal-condition change (spec'd behavior; ADR-gated).
- Building the Gate 1 tools in `--release` (separate global decision; rejected in Architecture Check).
- The jump-chain enumeration perf optimization (STACROSCISIM-002).
- Changing the games count for any lane other than `starbridge_crossing`.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-ci-games.mjs` exits 0 against the edited `ci/games.json`.
2. A temporary planted invalid value (`"games": 0` or `"games": "x"`) makes `node scripts/check-ci-games.mjs` exit non-zero with the new message (revert the plant after checking).
3. `node scripts/check-ci-games.mjs --emit` emits a matrix whose `starbridge_crossing` row carries the `games` field; all other rows are byte-for-byte unchanged.
4. On the PR run, the Gate 1 `starbridge_crossing` lane completes (no ~37-min runaway) with its `Quick simulation` step at ≤ ~4 min.

### Invariants

1. Every `ci/games.json` row still passes the existing `id`/`sim_flags`/`e2e` + set-equality drift checks; the `games` field is additive and optional.
2. Lanes without a `games` field still run exactly 1000 games (workflow default preserved).

## Test Plan

### New/Modified Tests

1. `None — CI-config + validator ticket; verification is command-based (check-ci-games.mjs) plus the measured PR lane timing named in Acceptance Criteria.`

### Commands

1. `node scripts/check-ci-games.mjs`
2. `node scripts/check-ci-games.mjs --emit`
3. The full simulation/replay/fixture/coverage pipeline is exercised per-lane by Gate 1 itself; the narrower validator commands above are the correct local boundary because the games-count budget can only be confirmed against real CI runner timing.

## Outcome

Completed: 2026-06-27

What changed:

- Added an optional `games` override to the `starbridge_crossing` row in `ci/games.json`, set to `50`.
- Updated the Gate 1 `Quick simulation` workflow command to use `matrix.games || 1000`, preserving the 1000-game default for every row without an override.
- Extended `scripts/check-ci-games.mjs` to reject non-positive or non-integer `games` values when the optional field is present.

Deviations from plan:

- No code or game-rule files were changed for this ticket. The pre-existing `games/starbridge_crossing/src/rules.rs` worktree diff was left untouched because this ticket's scope is CI config only.
- The PR-run wall-clock criterion remains maintainer/CI evidence: local validation proves the matrix shape and fail-closed schema, but the GitHub-hosted `starbridge_crossing` lane timing can only be confirmed after the PR workflow runs.

Verification:

- `node scripts/check-ci-games.mjs` passed with `ci/games.json OK -- 20 games in sync with games/.`
- `node scripts/check-ci-games.mjs --emit` emitted the matrix with `starbridge_crossing` carrying `"games":50`; the other rows remained without a `games` field and therefore continue to use the workflow default.
- Temporarily planting `"games": 0` for `starbridge_crossing` made `node scripts/check-ci-games.mjs` fail with `"starbridge_crossing" must have a positive integer "games" when present`; the planted invalid value was reverted to `50`.
