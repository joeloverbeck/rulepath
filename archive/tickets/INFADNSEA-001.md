# INFADNSEA-001: Infra A — N-seat-aware bridge seat acceptance + setup diagnostics

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (generalize the bridge seat factory + setup path); no `engine-core`, `game-stdlib`, or `games/*` behavior change
**Deps**: None

## Problem

The WASM bridge hardcodes a two-seat slice — `seats()` returns `vec![SeatId("seat-0"), SeatId("seat-1")]` (`crates/wasm-api/src/lib.rs:4095-4097`) — so the browser has no path to request any other seat count, and the first official N-seat game (Gate 15, River Ledger) could not instantiate >2 seats. Each game already validates seat count in its own `setup.rs` and rejects a wrong count with a `Diagnostic` (`games/race_to_n/src/setup.rs:27-30`, identical pattern in every game), so the missing piece is purely the bridge: request a count, build the N-element seat slice, call the game's `*_setup_match`, and surface the game's *existing* rejection diagnostic. Rust stays the seat-count authority; no game gains or loses behavior.

## Assumption Reassessment (2026-06-14)

1. `seats()` at `crates/wasm-api/src/lib.rs:4095-4097` returns the hardcoded two-element slice; per-game setup is dispatched through `*_setup_match(Seed, &seats, &SetupOptions)` (`crates/wasm-api/src/lib.rs:569-670`).
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` §2 (current state) + WB1, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §2` (seat-range declaration; Rust-owned wrong-seat-count diagnostics; the browser presents validation but must not decide which counts are legal).
3. Shared boundary under audit: the `wasm-api` → game `setup_match` call plus the `engine-core` `SeatId` / `Diagnostic` contract. `seat id` is an explicitly §3-permitted kernel noun, so no new noun enters `engine-core`.
4. FOUNDATIONS §2 (behavior authority): seat-count acceptance, validation, and diagnostics stay in Rust; TypeScript presents only. This ticket keeps the decision in Rust — the bridge builds the slice and returns the game's diagnostic; it adds no TS-side legality.
5. §11 fail-closed validation: each game's `setup.rs` already rejects a wrong count with a `Diagnostic` (`games/race_to_n/src/setup.rs:27-30`). This ticket *surfaces* that existing fail-closed path through the bridge rather than adding new validation, so no acceptance invariant is weakened and no hidden information is introduced (seat IDs are public).

## Architecture Check

1. Generalizing the existing `seats()` factory to take a requested count — and reusing each game's existing setup diagnostic — is cleaner than adding a parallel seat-count validator in the bridge: the single source of seat-count truth stays in each game's `setup.rs`.
2. No backwards-compatibility shim: `seats()` is replaced by a count-parameterized builder; existing two-seat call sites request the default count of 2.
3. `engine-core` stays free of mechanic nouns (only the already-permitted `seat id`/`Diagnostic` contracts are used); no `game-stdlib` change.

## Verification Layers

1. Bridge accepts a valid count and rejects an unsupported one -> unit test on the generalized setup entry asserting the game's `Diagnostic` is returned for a wrong count (no panic, no TS-side check).
2. Existing two-seat games unchanged -> `cargo test --workspace` plus a default-count regression (the default path still yields a 2-seat match for every game).
3. Kernel boundary preserved -> `bash scripts/boundary-check.sh` grep-proof (no mechanic noun enters `engine-core`).
4. §2 behavior authority -> FOUNDATIONS alignment check: seat-count acceptance is Rust-side; the bridge returns Rust diagnostics for TS to display.

## What to Change

### 1. Generalize the bridge seat factory

Replace the hardcoded `seats()` with a builder taking a requested seat count and returning the N-element `Vec<SeatId>` with stable `seat-0 .. seat-(n-1)` labels. Default callers request 2 so existing behavior is byte-identical.

### 2. Route setup through the game's existing diagnostic

The setup entry point passes the N-element slice to `*_setup_match`; when a game's `setup.rs` returns a wrong-count `Diagnostic`, surface it through the existing bridge diagnostic/result channel rather than asserting two seats in the bridge itself.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)

## Out of Scope

- Per-game seat-range catalog metadata and `client.ts` types (INFADNSEA-002).
- Any game declaring a supported range >2 — existing games keep rejecting ≠2 via their own `setup.rs`; the first >2 range arrives at Gate 15.
- Trace-schema, WASM-API-schema, or hash migration (spec §3.3 Not allowed).

## Acceptance Criteria

### Tests That Must Pass

1. New `wasm-api` unit test: setup with a valid count succeeds; setup with an unsupported count returns the game's Rust `Diagnostic` (no panic, no TypeScript-side legality).
2. `cargo test --workspace` — existing two-seat behavior unchanged.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Seat-count acceptance and rejection are decided in Rust; the bridge never hardcodes a fixed seat-count assertion that bypasses the game's diagnostic.
2. The seat slice uses stable, deterministic `seat-<i>` labels (setup/replay determinism).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`#[cfg(test)]`) — count-parameterized setup acceptance/rejection test.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test --workspace && bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-14

- Added count-aware Rust bridge match creation functions and additive raw WASM exports: `new_match_with_seat_count` and `new_match_with_variant_and_seat_count`.
- Routed bridge setup through count-parameterized seat builders while preserving existing two-seat defaults and the existing per-game seat label conventions.
- Added bridge tests for deterministic seat labels and for surfacing the current game's `invalid_seat_count` diagnostic when a requested count is unsupported.
- Deviations: existing default `new_match` / `new_match_for_variant` behavior remains two-seat for compatibility; per-game seat-range catalog metadata and web presentation remain owned by INFADNSEA-002/003.
- Verification: `cargo test -p wasm-api`; `cargo fmt --all --check`; `bash scripts/boundary-check.sh`; `cargo test --workspace`.
