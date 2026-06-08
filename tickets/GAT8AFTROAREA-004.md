# GAT8AFTROAREA-004: Wire high_card_duel native smoke into Gate 1 CI

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `.github/workflows/gate-1-game-smoke.yml` CI steps only; invokes existing tools against an already-registered game.
**Deps**: None

## Problem

`high_card_duel` is an accepted Gate 8 official game (registered in `crates/wasm-api`, all five game tools, and the `smoke:e2e` browser suite), but `.github/workflows/gate-1-game-smoke.yml` runs no native smoke for it: the workflow has simulate / replay-check / fixture-check / rule-coverage steps for `race_to_n`, `three_marks`, `column_four`, `directional_flip`, and `draughts_lite` — all five — but zero for `high_card_duel`. Separately, the Browser E2E step name (line 105) lists "column_four, directional_flip, and draughts_lite" while its body (line 113) already runs `high-card-duel.smoke.mjs`. This leaves the accepted Gate 8 game's deterministic-replay / fixture / rule-coverage evidence unexercised in CI and the step name misleading. This ticket closes the confirmed gap (spec D5 / WB5).

## Assumption Reassessment (2026-06-08)

1. `grep -c high_card_duel .github/workflows/gate-1-game-smoke.yml` returns `0` — confirmed gap. The five existing games each have four native steps (`simulate`/`replay-check`/`fixture-check`/`rule-coverage`, `gate-1-game-smoke.yml:30-88`); the E2E step body (`:106-113`) already runs `high-card-duel.smoke.mjs` while its name (`:105`) omits the game.
2. Tool support is confirmed: `high_card_duel` resolves in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` (all `grep high_card_duel tools/*/src/main.rs`), and `games/high_card_duel/benches/` exists with `thresholds.json`. So the four native commands and the bench run against a real registered game — no "tool does not support the game" defer is warranted.
3. Cross-artifact boundary under audit: the CI workflow is the documented validation path that must actually cover accepted-game evidence; this ticket adds steps that invoke existing tools (it changes no tool, game, or trace). The step pattern mirrors the existing five games' steps exactly.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §11 (replay/hash/serialization remain deterministic; evidence coverage) motivate this: the game already carries the coverage; CI must exercise it so drift fails loudly. Restating the principle confirms the fix is "run the existing checks in CI," not new behavior.
5. The replay-check (deterministic replay-hash) and fixture-check (serialization/no-leak fixture) surfaces are FOUNDATIONS §11 enforcement surfaces. This ticket only *invokes* them in CI against `high_card_duel`; it introduces no leak or nondeterminism path — it makes the existing deterministic/no-leak checks block on `high_card_duel` drift. The golden-trace/visibility coverage itself lives in the game crate (`games/high_card_duel/tests/`), unchanged here.

## Architecture Check

1. Adding the four native steps in the same pattern as the five existing games (and fixing the E2E step name) keeps CI's per-game coverage uniform and makes Gate 8 evidence fail loudly on drift — cleaner than a silent omission or an unexplained defer.
2. No backwards-compatibility shims; the workflow gains steps that call existing tools.
3. `engine-core` untouched; no `game-stdlib` change; no mechanic noun introduced. CI invokes Rust-owned tools (§2 behavior authority preserved).

## Verification Layers

1. `high_card_duel` has simulate/replay-check/fixture-check/rule-coverage steps in CI -> codebase grep-proof (`grep -c high_card_duel .github/workflows/gate-1-game-smoke.yml` ≥ 4).
2. The four native commands run green -> simulation/CLI run + golden trace / deterministic replay-hash check (`cargo run -p simulate -- --game high_card_duel --games 1000`; `cargo run -p replay-check -- --game high_card_duel --all`; `cargo run -p fixture-check -- --game high_card_duel`; `cargo run -p rule-coverage -- --game high_card_duel`).
3. E2E step name matches its body -> manual review (the step name includes `high_card_duel`).
4. Workflow YAML stays valid -> manual review / CI parse on push.

## What to Change

### 1. Native smoke steps for high_card_duel

Add four steps to `.github/workflows/gate-1-game-smoke.yml` mirroring the existing five-game pattern: `high_card_duel quick simulation` (`simulate -- --game high_card_duel --games 1000`), replay drift check (`replay-check -- --game high_card_duel --all`), fixture validation (`fixture-check -- --game high_card_duel`), and rule coverage drift check (`rule-coverage -- --game high_card_duel`). Place each alongside its sibling-game group.

### 2. E2E step name

Correct the Browser E2E step name (`:105`) so it names `high_card_duel` (its body already runs the smoke), keeping the listed-game phrasing consistent with the commands run.

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Adding a benchmark lane (benchmarks live in `.github/workflows/gate-2-benchmarks.yml`; spec lists the bench command as optional acceptance evidence, not a Gate 1 CI step).
- `token_bazaar` CI steps — a future Gate 9 concern (`specs/gate-9-token-bazaar-browser-proof.md` will add them to the same workflow; the two games' steps coexist).
- Any tool, game, trace, or fixture change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c "high_card_duel" .github/workflows/gate-1-game-smoke.yml` returns ≥ 4 (the four native steps), and the E2E step name contains `high_card_duel`.
2. `cargo run -p simulate -- --game high_card_duel --games 1000`, `cargo run -p replay-check -- --game high_card_duel --all`, `cargo run -p fixture-check -- --game high_card_duel`, and `cargo run -p rule-coverage -- --game high_card_duel` all exit 0.
3. `node scripts/check-doc-links.mjs` passes (no doc links broken) and the workflow parses on push.

### Invariants

1. `high_card_duel` native evidence (simulate/replay/fixture/rule-coverage) is exercised in Gate 1 CI on every PR/push — no accepted game omitted silently.
2. The CI steps invoke Rust-owned tools only; no behavior moves out of Rust (§2), and the existing determinism/no-leak checks (§11) now block on `high_card_duel` drift.

## Test Plan

### New/Modified Tests

1. `None — CI-wiring ticket; the verification is the four existing tool commands run against high_card_duel plus the workflow grep-proof. No game test/trace changes.`

### Commands

1. `grep -nE "high_card_duel" .github/workflows/gate-1-game-smoke.yml` — post-edit step-presence + step-name proof.
2. `cargo run -p simulate -- --game high_card_duel --games 1000 && cargo run -p replay-check -- --game high_card_duel --all && cargo run -p fixture-check -- --game high_card_duel && cargo run -p rule-coverage -- --game high_card_duel` — the exact commands the new CI steps run.
3. These four CLI commands are the correct verification boundary because the ticket's deliverable is wiring them into CI; running them locally proves the steps will pass.
