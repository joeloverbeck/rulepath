# GAT9TOKBAZBRO-011: Benchmarks + thresholds + BENCHMARKS.md + gate-2 CI

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/benches/token_bazaar.rs` (new), `benches/thresholds.json` (new), `docs/BENCHMARKS.md` (new), `.github/workflows/gate-2-benchmarks.yml` (modify)
**Deps**: GAT9TOKBAZBRO-005, GAT9TOKBAZBRO-008

## Problem

Official games carry benchmarks with documented thresholds and a gate-2 CI lane.
This ticket adds the `token_bazaar` benchmark suite over the spec's named
operations, a `thresholds.json` of smoke floors, the `BENCHMARKS.md` doc
explaining the operations and that the floors are smoke floors pending
calibration, and the gate-2 workflow steps that run the bench and check it
against thresholds — without claiming performance the way `high_card_duel` does.

## Assumption Reassessment (2026-06-08)

1. The benchable surface exists: setup/legal-actions/validate-apply/view/effects
   (GAT9TOKBAZBRO-003/004/005/006), replay (-007), and the Level 1 bot decision
   (-008). The sibling `games/high_card_duel/benches/high_card_duel.rs` +
   `benches/thresholds.json` + `docs/BENCHMARKS.md` establish the house pattern
   (verified present), and `.github/workflows/gate-2-benchmarks.yml` already has a
   `high_card_duel` bench smoke + threshold-check step (verified at lines 41-83)
   this ticket mirrors for `token_bazaar`.
2. The benchmark operations are fixed by `specs/gate-9-token-bazaar-browser-proof.md`
   → "Fixtures, properties, rule coverage, and benchmarks" (setup; legal action
   tree; validate/apply collect; validate/apply exchange; validate/apply
   fulfill/refill; public view projection; effect serialization/filtering; replay
   command stream; random legal playout; Level 1 bot decision; WASM operation
   smoke) and "Start with smoke thresholds … BENCHMARKS.md must explain that they
   are smoke floors and name the follow-up calibration expectation."
3. Cross-artifact boundary under audit: three surfaces in lockstep — the Criterion
   bench, the `thresholds.json` consumed by `tools/bench-report`, and the gate-2
   workflow. `bench-report` is invoked generically with `--input` + `--thresholds`
   (verified: gate-2 passes `games/<game>/benches/thresholds.json`), so no
   `bench-report` code arm is needed — only the new thresholds file + workflow
   step. `BENCHMARKS.md` is additionally consumed by `tools/rule-coverage` by path,
   so it co-lands here (tool-validated doc) and is referenced by the -012
   registration.

## Architecture Check

1. Co-locating the bench, thresholds, doc, and CI step in one ticket keeps the
   benchmark lane self-consistent (a threshold file with no bench, or a CI step
   with no doc, would be incoherent); smoke floors with an explicit calibration
   note avoid over-claiming performance.
2. No backwards-compatibility aliasing/shims — new bench/threshold/doc; the gate-2
   workflow gains an additive step.
3. `engine-core` untouched; benches exercise the game crate only. No `game-stdlib`
   helper introduced.

## Verification Layers

1. Bench runs + threshold check passes -> benchmark check: `cargo bench -p token_bazaar`
   then `cargo run -p bench-report -- --input <report> --thresholds games/token_bazaar/benches/thresholds.json`.
2. Smoke-floor semantics documented -> manual review of `BENCHMARKS.md` (names the
   floors as smoke + the calibration follow-up).
3. gate-2 workflow step is valid YAML and references real paths -> CI dry-read +
   `cargo bench -p token_bazaar -- legal_actions` smoke.

## What to Change

### 1. `games/token_bazaar/benches/token_bazaar.rs`

Criterion benches for the spec's operations (setup, legal_actions, validate/apply
collect/exchange/fulfill+refill, view projection, effect serialization, replay
stream, random playout, Level 1 bot decision, WASM-op smoke).

### 2. `games/token_bazaar/benches/thresholds.json`

Smoke floors per operation, shape matching `high_card_duel/benches/thresholds.json`.

### 3. `games/token_bazaar/docs/BENCHMARKS.md`

Document the operations and state the thresholds are smoke floors with a named
calibration follow-up; no measured-performance claims without baselines.

### 4. `.github/workflows/gate-2-benchmarks.yml` (modify)

Add a `token_bazaar` bench smoke step + a full-bench + `bench-report` threshold
check step, mirroring the existing `high_card_duel` steps.

## Files to Touch

- `games/token_bazaar/benches/token_bazaar.rs` (new)
- `games/token_bazaar/benches/thresholds.json` (new)
- `games/token_bazaar/docs/BENCHMARKS.md` (new)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- `tools/bench-report` code changes (it is generic; no game arm needed).
- `RULE-COVERAGE.md` / rule-coverage registration (GAT9TOKBAZBRO-012) — though
  `BENCHMARKS.md` authored here is what rule-coverage later validates.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p token_bazaar -- legal_actions` — bench smoke runs.
2. `cargo bench -p token_bazaar | tee /tmp/r.txt && cargo run -p bench-report -- --input /tmp/r.txt --thresholds games/token_bazaar/benches/thresholds.json` — threshold check passes.
3. gate-2 workflow parses and references existing paths.

### Invariants

1. Thresholds are smoke floors with documented calibration intent — no
   unmeasured performance claim.
2. `bench-report` is invoked generically (no game-specific code arm).

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/benches/token_bazaar.rs` — Criterion bench group.
2. `games/token_bazaar/benches/thresholds.json` — floor values consumed by bench-report.

### Commands

1. `cargo bench -p token_bazaar -- legal_actions`
2. `cargo bench -p token_bazaar | tee /tmp/token_bazaar-bench.txt && cargo run -p bench-report -- --input /tmp/token_bazaar-bench.txt --thresholds games/token_bazaar/benches/thresholds.json`
3. The bench + bench-report pair is the correct boundary; CI wiring is the same
   pair under the gate-2 workflow.
