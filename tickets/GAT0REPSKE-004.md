# GAT0REPSKE-004: CI smoke pipeline and boundary gates

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — CI configuration only; no crate or source behavior change.
**Deps**: GAT0REPSKE-001, GAT0REPSKE-002, GAT0REPSKE-003

## Problem

Gate 0's exit criteria require workspace smoke tests, the web build, a WASM build smoke, and a noun-free `engine-core` with correct dependency direction to be continuously verifiable. A CI pipeline turns these into re-runnable gates that fail loudly. In particular, it codifies the boundary review (noun-free + dependency direction) as automated, blocking checks rather than a one-time manual sign-off, so the kernel boundary cannot drift as later gates add code.

## Assumption Reassessment (2026-06-05)

1. `.github/` is absent (greenfield, verified `test -e` 2026-06-05); the workspace (GAT0REPSKE-001), `wasm-api`/`apps/web` (GAT0REPSKE-002), and `tools/*` (GAT0REPSKE-003) exist by declared `Deps`. CI provider is GitHub Actions per spec assumption A-3 — no foundation doc pins a provider, so the config location is swappable.
2. Spec WB4 + §5 exit criteria + §6 acceptance evidence; `docs/TESTING-REPLAY-BENCHMARKING.md` §17 CI expectations (formatting, linting, unit/rule tests, WASM build smoke, docs link checks where practical).
3. Cross-artifact boundary under audit: CI consumes the whole workspace, the `apps/web` build, and the `engine-core` boundary contract (noun-free + dependency direction established in 001).
4. §11 universal acceptance invariants: CI must be deterministic, fail-closed, and blocking; builds/hashes fail loudly on unplanned drift.
5. Fail-closed boundary gate (§3/§11): the mechanic-noun grep and the `cargo tree -p engine-core` dependency-direction check are **blocking** CI gates — a violation fails the build; it is not a warning. This is the re-runnable enforcement surface for the spec's boundary-review exit criterion.

## Architecture Check

1. Encoding the boundary review as CI gates (grep + `cargo tree`) is more robust than a one-time manual sign-off — it prevents kernel-boundary drift on every later commit.
2. No backwards-compatibility shims.
3. CI enforces `engine-core` noun-free (§3) and introduces no `game-stdlib` change.

## Verification Layers

1. format/lint/build/test gate -> simulation/CLI run (CI runs `cargo fmt --check`, clippy, `cargo build`, `cargo test`).
2. WASM build + web build smoke -> simulation/CLI run (CI builds the `wasm-api` artifact and the `apps/web` shell).
3. Boundary gate (noun-free + dependency direction) -> codebase grep-proof + schema/serialization validation (CI grep + `cargo tree -p engine-core`).
4. Docs link check -> manual review / CLI (link checker over `docs/` + `specs/`, where practical per TESTING §17).

## What to Change

### 1. CI workflow

`.github/workflows/ci.yml` with jobs: format check, lint/clippy, `cargo build`, `cargo test`, WASM build smoke, `apps/web` build, the boundary gate (noun-free grep + `cargo tree -p engine-core`), and a docs/anchor link check over `docs/`+`specs/` (where practical; mark deferred otherwise).

### 2. Boundary gate script (optional)

If the boundary gate is scripted rather than inlined, a small `scripts/boundary-check.sh` runnable locally and from CI.

## Files to Touch

- `.github/workflows/ci.yml` (new)
- `scripts/boundary-check.sh` (new) — optional, if the boundary gate is scripted

## Out of Scope

- Full fuzzing and expensive benchmarks (nightly/manual; Gate 2+).
- Golden-trace, replay, visibility/no-leak, bot, and UI-smoke jobs (no game exists yet — Gate 1+).
- Hosted services, deploy pipelines (spec §2 not-allowed).

## Acceptance Criteria

### Tests That Must Pass

1. The CI pipeline runs and passes on a clean checkout (format, lint, build, test, WASM smoke, web build).
2. The boundary gate fails the build when a mechanic noun or a forbidden `engine-core` dependency is introduced (negative check).
3. The docs link check runs, or is explicitly marked deferred in the workflow.

### Invariants

1. CI checks are blocking (fail-closed), not advisory (§11).
2. The CI build is deterministic — no wall-clock- or network-dependent gating logic (ARCHITECTURE.md §9).

## Test Plan

### New/Modified Tests

1. `.github/workflows/ci.yml` — the pipeline itself; verified by a CI run on a branch.
2. `scripts/boundary-check.sh` (if added) — the boundary gate, runnable and assertable locally.

### Commands

1. `grep -rniE "board|card|deck|grid|suit|resource|capture" crates/engine-core/src && echo FAIL || echo OK; cargo tree -p engine-core` — boundary gate run locally.
2. `cargo fmt --check && cargo clippy && cargo test` plus the WASM and `apps/web` build commands — local mirror of the full pipeline.
3. Push to a branch (or run via `act`) to exercise the workflow end-to-end — the workflow can only be fully validated in the CI runtime.
