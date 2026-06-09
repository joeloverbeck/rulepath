# GAT10POKLITBET-013: Native benchmarks, thresholds, BENCHMARKS.md, and gate-2 CI

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/poker_lite/benches/poker_lite.rs`, `games/poker_lite/benches/thresholds.json`, `games/poker_lite/docs/BENCHMARKS.md`, `games/poker_lite/Cargo.toml` (bench target), `.github/workflows/gate-2-benchmarks.yml`. No kernel change.
**Deps**: GAT10POKLITBET-010

## Problem

Official games require native benchmarks with calibrated thresholds and a `BENCHMARKS.md` doc, plus CI wiring. `poker_lite` needs a benchmark suite covering setup/playout/projection/export throughput, provisional thresholds, and gate-2 registration — without claiming the (still-Proposed) ADR 0005 variance-aware floor policy as accepted.

## Assumption Reassessment (2026-06-08)

1. The benchmark shape matches `games/secret_draft/benches/secret_draft.rs` + `games/secret_draft/benches/thresholds.json` (verified present this session). The bench operations enumerated in spec §F (setup+shuffle+deal, legal-action generation, apply-action, observer/seat projection, public export/import, full random-legal + Level 2 simulation) mirror the sibling suite.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 benches, §6 "Native benchmarks exist" — provisional floor ≥2,000 completed hands/sec, §F, §7 Benchmarks) fixes the suite and the provisional floor. The bot simulation bench needs the Level 2 bot (GAT10POKLITBET-010).
3. Cross-artifact boundary under audit: `gate-2-benchmarks.yml` registers all 8 games via explicit per-game steps (bench-smoke + bench-gate) — `poker_lite` adds steps there. `tools/bench-report` registers only games with threshold files (5 of 8; `token_bazaar`/`secret_draft` absent), so a `bench-report` arm is **optional** per the resolved tool-scope set — add only if a threshold/report need arises, matching the sibling precedent. `BENCHMARKS.md` is one of the docs `tools/rule-coverage` reads, so it complements GAT10POKLITBET-012's rule-coverage registration (both should land for a fully-green rule-coverage; they share no file).
4. FOUNDATIONS §6 (benchmarks mandatory) plus the accepted benchmark ADRs motivate this ticket. Restated: ADR 0002 (`docs/adr/0002-ci-benchmark-gating-lanes.md`, Accepted) and ADR 0003 (`…0003-ci-calibrated-benchmark-thresholds.md`, Accepted) govern the lanes/thresholds; **ADR 0005** (`…0005-variance-aware-ci-benchmark-floors.md`) is **Proposed** (verified this session) — this ticket must NOT report ADR 0005 as accepted even if it follows variance-aware practice (spec §9 forbidden change).

## Architecture Check

1. Provisional thresholds + a named calibration follow-up under the accepted ADR 0002/0003 lanes keep CI honest (no flaky hard-gate claim) while the game is small and fast. Matches the sibling benchmark approach.
2. No backwards-compatibility aliasing/shims — new bench target + additive CI steps.
3. `engine-core` untouched (§3); no `game-stdlib` promotion (§4); benches depend on the game crate.

## Verification Layers

1. Benchmarks run -> `cargo bench -p poker_lite` (smoke filter).
2. Threshold presence + lane config -> `benches/thresholds.json` review against ADR 0002/0003 lanes.
3. CI wiring (gate-2 bench-smoke + bench-gate steps for poker_lite) -> codebase grep-proof in `gate-2-benchmarks.yml`.
4. ADR-status fidelity (no ADR 0005 "accepted" claim) -> grep-proof over `BENCHMARKS.md` + `thresholds.json` for any "0005 accepted" wording (must be absent).

## What to Change

### 1. `games/poker_lite/benches/poker_lite.rs` + `benches/thresholds.json` + `games/poker_lite/Cargo.toml`

Author the bench suite per §F; add provisional thresholds with a calibration-follow-up note under ADR 0002/0003; register the `[[bench]]` target in the crate `Cargo.toml`.

### 2. `games/poker_lite/docs/BENCHMARKS.md`

Instantiate from `templates/GAME-BENCHMARKS.md`. Document operations, the provisional ≥2,000 hands/sec floor, and the calibration follow-up; state ADR 0005 is Proposed, not accepted.

### 3. `.github/workflows/gate-2-benchmarks.yml` (modify)

Add the `poker_lite` bench-smoke and bench-gate steps mirroring an existing per-game block.

## Files to Touch

- `games/poker_lite/benches/poker_lite.rs` (new)
- `games/poker_lite/benches/thresholds.json` (new)
- `games/poker_lite/docs/BENCHMARKS.md` (new)
- `games/poker_lite/Cargo.toml` (modify — add bench target)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Tool registration for the four all-games tools (GAT10POKLITBET-012).
- `bench-report` registration unless a concrete need arises (optional per resolved tool-scope set).
- Accepting / citing ADR 0005 as accepted (spec §9 forbidden).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p poker_lite` runs the suite (smoke) without error.
2. `gate-2-benchmarks.yml` contains poker_lite bench-smoke + bench-gate steps.
3. `BENCHMARKS.md` and `thresholds.json` contain no claim that ADR 0005 is accepted.

### Invariants

1. Benchmark thresholds follow accepted ADR 0002/0003 lanes; variance-aware floors are not claimed as accepted policy (§13, spec §9).
2. Benchmarks are deterministic-input and report calibration/variance notes (no flaky hard-gate claim).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/benches/poker_lite.rs` — setup/playout/projection/export/bot-sim benchmarks.
2. `games/poker_lite/benches/thresholds.json` — provisional calibrated thresholds.

### Commands

1. `cargo bench -p poker_lite` (use the crate's bench filters for a smoke run)
2. `grep -L "0005.*accepted" games/poker_lite/docs/BENCHMARKS.md` — confirms no accepted-ADR-0005 claim.
3. `cargo run -p rule-coverage -- --game poker_lite` — confirms BENCHMARKS.md satisfies the rule-coverage doc set alongside GAT10POKLITBET-012.

## Outcome

Completed on 2026-06-09.

- Added the native `poker_lite` benchmark target with deterministic setup,
  legal-action generation, validation, apply, observer projection, public
  export/import, terminal hashing, Level 2 bot decision, and full Level 2
  playout surfaces.
- Added provisional threshold metadata and `BENCHMARKS.md` without claiming ADR
  0005 is accepted.
- Registered `poker_lite` in Gate 2 bench-smoke and bench-gate workflow steps.
- Updated `rule-coverage` to read `games/poker_lite/docs/BENCHMARKS.md` instead
  of the temporary rule-coverage placeholder.

Verification:

- `cargo fmt --all --check`
- `cargo bench -p poker_lite`
- `cargo bench -p poker_lite -- legal_actions`
- `cargo run -p bench-report -- --input /tmp/poker_lite-benchmark-report.txt --thresholds games/poker_lite/benches/thresholds.json`
- `cargo run -p rule-coverage -- --game poker_lite`
- `grep -Ri "0005.*accepted" games/poker_lite/docs/BENCHMARKS.md games/poker_lite/benches/thresholds.json` produced no matches.
