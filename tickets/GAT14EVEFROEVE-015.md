# GAT14EVEFROEVE-015: Native tools, RULE-COVERAGE.md, boundary-check, and gate-1 CI

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs` (modify); `scripts/boundary-check.sh` (modify); `games/event_frontier/docs/RULE-COVERAGE.md` (new); `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: GAT14EVEFROEVE-001, GAT14EVEFROEVE-011, GAT14EVEFROEVE-014

## Problem

`event_frontier` must be registered across the native tool suite and the gate-1 CI lanes. `simulate` must report per-faction win counts, **victory-type frequencies** (Charter instant / Freeholder instant / final fallback), average card count, average Reckoning scores, and pass rates — the "useful metrics"/"demo-coherent" evidence base. `replay-check`, `fixture-check`, and `rule-coverage` validate the traces/fixtures/`RULE-COVERAGE.md`; `bench-report` reads the thresholds. This ticket also **evaluates** extending `scripts/boundary-check.sh`'s `mechanic_pattern` with `initiative`/`eligibility` (word-boundary false-positive check first; `event` stays out as too generic), and adds the gate-1 native CI steps.

## Assumption Reassessment (2026-06-12)

1. The tool dispatch sites and boundary pattern are current: verified each of `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs` registers a game via a per-game match/if dispatch (e.g. `frontier_control` arms), and that `scripts/boundary-check.sh` line ~4 defines `mechanic_pattern='board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting|role|scenario|faction|territory'` (covers `card`/`deck`/`scenario`/`faction`, not `initiative`/`eligibility`/`event`).
2. The validated artifacts exist: verified ticket 001 authored `RULES.md` (rule IDs `rule-coverage` maps), ticket 011 authored the golden traces (`replay-check`) and scenario fixtures via ticket 004 (`fixture-check`), and ticket 012 authored `thresholds.json` (`bench-report`) and `BENCHMARKS.md`. `RULE-COVERAGE.md` is authored here and read by `rule-coverage` with `RULES.md` + `BENCHMARKS.md`.
3. Cross-artifact boundary under audit: a fully-green `rule-coverage --game event_frontier` depends on `RULES.md` (ticket 001), `BENCHMARKS.md` (ticket 012), and this ticket's `RULE-COVERAGE.md` all present — flagged partial-green window. `seed-reducer`/`trace-viewer` carry a hardcoded `race_to_n`/`directional_flip` allowlist (verified), so `event_frontier` needs no registration there and is correctly excluded.
4. FOUNDATIONS §3 (`engine-core` kernel boundary; the mechanical boundary check) and §11 (no silent rule-coverage gaps) motivate this ticket. Restated before trusting the spec: the `initiative`/`eligibility` pattern extension must stay green on the whole existing tree (word-boundary false-positive check) before adoption; `event` is excluded as too generic to pattern-match.
5. Rename/extension blast radius: extending `mechanic_pattern` is a repo-wide guard change — confirm `grep -rIE "initiative|eligibility" crates/engine-core/src` returns no in-kernel matches before adding the tokens, so the extension does not red-flag existing `engine-core` code. No replay/hash semantics change.

## Architecture Check

1. Registering per-tool match arms (mirroring `frontier_control`) is the established dispatch pattern; evaluating the boundary-pattern extension with a false-positive check before adoption is cleaner than blindly adding tokens that could red-flag legitimate kernel code.
2. No backwards-compatibility aliasing/shims — additive tool arms, additive CI steps, guarded pattern extension.
3. Confirms `engine-core` stays free of mechanic nouns (the boundary check is the enforcement); no `game-stdlib` promotion.

## Verification Layers

1. Tool registration -> `cargo run -p simulate -- --game event_frontier --games 1000` (per-faction win counts + victory-type frequencies + pass rates), `replay-check --game event_frontier --all`, `fixture-check --game event_frontier`, `rule-coverage --game event_frontier` all pass.
2. Boundary check -> `bash scripts/boundary-check.sh` passes on the whole tree, including any `initiative`/`eligibility` extension.
3. Rule coverage -> `rule-coverage --game event_frontier` reports no silent gaps (consuming `RULES.md` + `RULE-COVERAGE.md` + `BENCHMARKS.md`).
4. CI lane -> `gate-1-game-smoke.yml` adds the native simulate/replay/fixture/rule-coverage steps for `event_frontier`.

## What to Change

### 1. Native tool registration

Add `event_frontier` match arms to `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs`. `simulate` reports per-faction win counts, victory-type frequencies, average card count, average Reckoning scores, and pass rates.

### 2. RULE-COVERAGE.md + boundary check

Author `games/event_frontier/docs/RULE-COVERAGE.md` (from `templates/GAME-RULE-COVERAGE.md`) mapping every rule ID to tests/traces. Run the `grep -rIE "initiative|eligibility" crates/engine-core/src` false-positive check; if clean, extend `mechanic_pattern` with `initiative` and `eligibility` (keep `event` out); confirm `boundary-check.sh` stays green.

### 3. gate-1 CI

Add the `event_frontier` native steps (simulate/replay/fixture/rule-coverage, web build/e2e registration) to `.github/workflows/gate-1-game-smoke.yml`.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `scripts/boundary-check.sh` (modify) — evaluate + (if clean) add `initiative`/`eligibility`
- `games/event_frontier/docs/RULE-COVERAGE.md` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- The browser E2E smoke + catalog README reconciliation (ticket 018) — also touches `gate-1-game-smoke.yml` (shared-file merge, no inter-dep).
- `seed-reducer`/`trace-viewer` registration — not needed (hardcoded 2-game allowlist; `event_frontier` correctly excluded).
- Authoring traces/fixtures/thresholds (tickets 011/012) — this ticket validates them.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game event_frontier --games 1000` finishes with no illegal bot action or invariant failure and reports the victory-type frequencies.
2. `cargo run -p replay-check -- --game event_frontier --all`, `fixture-check --game event_frontier`, and `rule-coverage --game event_frontier` all pass.
3. `bash scripts/boundary-check.sh` passes on the whole tree including any pattern extension.

### Invariants

1. `rule-coverage --game event_frontier` reports no silent gaps.
2. The boundary-pattern extension stays green on the existing tree; `engine-core` holds no mechanic noun.

## Test Plan

### New/Modified Tests

1. `tools/*/src/main.rs` — `event_frontier` registration arms.
2. `games/event_frontier/docs/RULE-COVERAGE.md` — rule-ID-to-test mapping.

### Commands

1. `cargo run -p simulate -- --game event_frontier --games 1000 && cargo run -p replay-check -- --game event_frontier --all`
2. `cargo run -p fixture-check -- --game event_frontier && cargo run -p rule-coverage -- --game event_frontier && bash scripts/boundary-check.sh`
3. The tool CLIs plus the boundary check are the correct boundary — they validate the gate's native evidence end to end before the browser layer.
