# UNI8CMECSCA-018: Create dev-only `crates/game-test-support` + boundary guard

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/game-test-support/{Cargo.toml,src/lib.rs}` (new), `Cargo.toml`, `scripts/boundary-check.sh`
**Deps**: UNI8CMECSCA-012, UNI8CMECSCA-013

## Problem

The no-leak and profile-driver helpers (C-07/C-08) are test/evidence orchestration and must be absent from every production dependency graph. This ticket creates the new workspace member `crates/game-test-support`, defines its public module boundaries (`no_leak`, `profiles` land in later tickets), and extends `scripts/boundary-check.sh` to reject any normal/build dependency edge from production/workspace targets to it — so game consumers can use it only under `[dev-dependencies]` (C-06). The boundary is enforced by a script/test, not convention.

## Assumption Reassessment (2026-06-22)

1. The workspace `Cargo.toml` lists ~28 members (crates + games + tools); `crates/game-test-support` does not exist (confirmed by `ls`/grep at the reassessed commit). `scripts/boundary-check.sh` exists and currently guards `engine-core` noun-freedom.
2. Spec §4.1/§4.2/§5 8C-018 fix the deliverable: new crate usable only as a development dependency; public module boundaries declared; boundary check extended to reject normal/build reverse edges; `cargo tree --workspace -e normal --invert game-test-support` shows no normal reverse consumer.
3. Cross-artifact boundary under audit: the workspace dependency graph (`Cargo.toml`) and the boundary guard (`scripts/boundary-check.sh`). Register entry `MSC-8C-006` homes this as a new crate. The crate may depend on `engine-core` (for kernel types incl. `StableBytesWriter`/`ActionTree` from UNI8CMECSCA-012/013) but must not depend on any `games/*` crate.
4. FOUNDATIONS §3/§11: the crate is dev/test infrastructure with no game behavior and no production reverse edge; the boundary guard makes the dev-only contract fail-closed (§14 EC-14) rather than relying on manifest intent.
5. Determinism/no-leak substrate (§11): this ticket builds the *input* surface that the no-leak harness (UNI8CMECSCA-019) and profile drivers (UNI8CMECSCA-022) consume — it introduces no projection, authorization, or leakage path itself; the no-leak firewall is enforced by those later tickets and the guard prevents this crate from entering production.

## Architecture Check

1. A separate dev-only crate (vs. game-local modules or a runtime crate) is the only home that keeps the helpers out of production/WASM/browser dependency graphs while remaining shared.
2. No backwards-compatibility shim — a new crate; nothing aliased.
3. `engine-core`/`game-stdlib` untouched in behavior; the new crate is test infrastructure, and the guard proves it never enters a normal build.

## Verification Layers

1. Crate compiles and is a workspace member → `cargo build -p game-test-support`.
2. No normal/build reverse edge from production/workspace targets → `cargo tree --workspace -e normal --invert game-test-support` (empty) + the extended `scripts/boundary-check.sh` assertion.
3. Public module boundaries declared (no game behavior, no `games/*` dependency) → grep-proof on `crates/game-test-support/Cargo.toml`/`src/lib.rs`.

## What to Change

### 1. `crates/game-test-support/{Cargo.toml,src/lib.rs}` (new)

Create the crate with `engine-core` (and `game-stdlib` if needed) as dependencies, declaring the public module surface (`pub mod no_leak;` / `pub mod profiles;` stubbed for later tickets). No `games/*` dependency.

### 2. `Cargo.toml` (workspace)

Add `crates/game-test-support` to `members`.

### 3. `scripts/boundary-check.sh`

Add an assertion that `cargo tree -e normal --invert game-test-support` (or an equivalent edge query) reports no production/workspace reverse consumer; fail closed on any match.

## Files to Touch

- `crates/game-test-support/Cargo.toml` (new)
- `crates/game-test-support/src/lib.rs` (new)
- `Cargo.toml` (modify)
- `scripts/boundary-check.sh` (modify)

## Out of Scope

- Implementing `no_leak` (UNI8CMECSCA-019) or `profiles` (UNI8CMECSCA-022) bodies.
- Adding the crate to any game's `[dev-dependencies]` (the pilot tickets do that).
- Any `games/*` or production dependency on this crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p game-test-support` and `cargo build --workspace` pass.
2. `cargo tree --workspace -e normal --invert game-test-support` shows no normal reverse consumer.
3. `bash scripts/boundary-check.sh` passes and now fails closed on a normal reverse edge to `game-test-support`.

### Invariants

1. `game-test-support` has no `games/*` or production normal/build dependent.
2. The dev-only boundary is enforced by `scripts/boundary-check.sh`, not convention.

## Test Plan

### New/Modified Tests

1. `scripts/boundary-check.sh` — new assertion rejecting a normal reverse edge to `game-test-support` (the durable pass/fail authority).

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `bash scripts/boundary-check.sh && cargo build --workspace`
3. `cargo tree` + the boundary script are the correct boundary — the dev-only contract is a dependency-graph property, not a unit-test property.

## Outcome

Completed: 2026-06-22

What changed:
- Added workspace member `crates/game-test-support`.
- Created the dev/test-only crate with a normal dependency only on
  `engine-core`.
- Declared public module boundaries for later work via `pub mod no_leak` and
  `pub mod profiles`, with stub module files.
- Extended `scripts/boundary-check.sh` to run
  `cargo tree --workspace -e normal,build --invert game-test-support` and fail
  if any reverse dependency edge appears after the root package.
- Flipped `MSC-8C-006` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from
  `candidate` to `accepted`.

Deviations:
- Added `crates/game-test-support/src/no_leak.rs` and
  `crates/game-test-support/src/profiles.rs` in addition to `src/lib.rs` so the
  declared public modules compile as real module boundaries for later tickets.
- `Cargo.lock` gained the new workspace package entry when the crate was built.
- No game, tool, production crate, or dev-dependency consumer was added.

Verification:
- `cargo build -p game-test-support`
- `cargo tree --workspace -e normal --invert game-test-support`
- `cargo tree --workspace -e normal,build --invert game-test-support`
- `bash scripts/boundary-check.sh`
- `cargo build --workspace`
- `cargo fmt --all --check`
- `rg -n "games/|path = \"\\.\\./\\.\\./games|path = \"\\.\\./games|engine-core|pub mod" crates/game-test-support`
