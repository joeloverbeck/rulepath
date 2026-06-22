# UNI8CMECSCA-019: Implement generic no-leak matrix geometry in `game-test-support::no_leak`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/game-test-support/src/no_leak.rs` (new), `crates/game-test-support/src/lib.rs`
**Deps**: UNI8CMECSCA-018

## Problem

The pairwise no-leak assertion geometry (source-seat × viewer × surface) is common across hidden-information games, but projection, reveal timing, and authorization are game-owned. This ticket implements the generic Cartesian-product assertion geometry and structured failure types in `game-test-support::no_leak`, using closures/typed adapters for viewers, surfaces, snapshots, probes, and expectations (C-07). The harness only enumerates cases, compares expected exposure, aggregates failures, and reports source seat/viewer/surface/canary/expectation — it never constructs a view, redacts, infers reveal timing, or authorizes facts.

## Assumption Reassessment (2026-06-22)

1. `crates/game-test-support` exists with a `no_leak` module stub after UNI8CMECSCA-018; no leak-matrix geometry exists yet. ADR 0004 (`docs/adr/0004-hidden-info-replay-export-taxonomy.md`, `Accepted`) governs the visibility/export taxonomy this geometry asserts against.
2. Spec §5 "C-07 harness contract" fixes the shape: `enum ExposureExpectation { MustBeAbsent, MustBePresent, NotApplicable }`; `struct LeakProbe<SourceSeat, CanaryId, Canary>`; `fn assert_pairwise_no_leak<...>(viewers, surfaces, probes, snapshot, expectation, contains) -> Result<(), PairwiseLeakFailure<...>>`. Exact generics are one-line-correctable; ownership is not — the game supplies snapshots, probe values, reveal/authorization expectations, and the containment function. Register entry `MSC-8C-007` homes this in `game-test-support`.
3. Cross-artifact boundary under audit: the no-leak assertion geometry vs. the runtime visibility projection (which stays in each game). The harness must support a typed public observer without string-heuristic authorization.
4. FOUNDATIONS §11 no-leak firewall: this is the enforcement helper for "facts private to seat A must not reach seat B / public / DOM / logs / exports." It must preserve or strengthen ADR 0004 (EC-15/EC-21) and never weaken a game-specific assertion.
5. No-leak/visibility surface under audit (§11/EC-15/EC-18): the harness enumerates and compares; it does not project, authorize, reveal, or redact. Canaries are generated in native test code, uniquely scoped to the source seat, and demonstrably absent from committed public/seat-private fixtures — a helper that injects private canaries into serializable state is rejected.

## Architecture Check

1. Only the Cartesian-product assertion geometry is common; keeping projection/authorization in each game via closures is the boundary that prevents the harness from becoming a runtime game framework.
2. No backwards-compatibility shim — a new module; nothing aliased.
3. `engine-core`/`game-stdlib` untouched; the harness is dev-only and decides no game policy.

## Verification Layers

1. Authorized, unauthorized, public-after-reveal, ignored/not-applicable, missing-canary, false-positive-resistant probe, and diagnostic-rendering cases pass → `game-test-support` unit tests.
2. Harness constructs no view and decides no authorization → grep-proof on `no_leak.rs` (no projection/redaction call; expectations come from the supplied closure).
3. Structured failure reports source seat/viewer/surface/canary/expectation → failure-rendering test.
4. ADR-0004 alignment → FOUNDATIONS/ADR-0004 alignment check in the ticket review.

## What to Change

### 1. `crates/game-test-support/src/no_leak.rs` (new)

Implement `ExposureExpectation`, `LeakProbe`, `PairwiseLeakFailure`, and `assert_pairwise_no_leak` with closure-supplied `snapshot` / `expectation` / `contains`. Support a typed public observer. The helper aggregates failures and reports; it performs no projection/authorization/reveal/redaction.

### 2. `crates/game-test-support/src/lib.rs`

Wire `pub mod no_leak;` (replace the stub).

### 3. Unit tests

Authorized / unauthorized / public-after-reveal / not-applicable / missing-canary / false-positive-resistant / diagnostic-rendering.

## Files to Touch

- `crates/game-test-support/src/no_leak.rs` (new)
- `crates/game-test-support/src/lib.rs` (modify)

## Out of Scope

- Piloting in any game (High Card UNI8CMECSCA-020, River UNI8CMECSCA-021).
- Any projection, authorization, reveal, or redaction logic in the harness.
- Injecting canaries into serializable/committed state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-test-support` passes, covering all seven case classes above.
2. A test proves the harness flags a missing canary and resists a false-positive probe.
3. `cargo build --workspace` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. The harness never constructs a view, authorizes a fact, infers reveal timing, or redacts output.
2. Canaries are test-only and never serialized into committed public/seat-private artifacts.

## Test Plan

### New/Modified Tests

1. `crates/game-test-support/src/no_leak.rs` (inline `#[cfg(test)]`) — the seven case classes + diagnostic rendering.

### Commands

1. `cargo test -p game-test-support`
2. `bash scripts/boundary-check.sh`
3. The `game-test-support` unit suite is the correct boundary — game pilots exercise it in UNI8CMECSCA-020/021.
