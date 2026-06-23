# 8CR1PUBFIXSEA-008: Three Marks C-02 strict canonical seat parser

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/three_marks` (`src/ids.rs`); accepted canonical input unchanged, no trace/golden change
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/three_marks/src/ids.rs::ThreeMarksSeat::parse` hand-rolls canonical seat parsing (`match value { "seat_0" => …, "seat_1" => …, _ => None }`) instead of delegating to `SeatId::parse_canonical` (MSC-8C-002). Adopt strict canonical delegation plus a bounded enum mapping, preserving accepted values and `as_str()`, and add direct rejection tests. The WASM import adapter (`parse_seat_import`) remains the only place bounded legacy aliases are accepted.

## Assumption Reassessment (2026-06-23)

1. `ThreeMarksSeat::parse` currently matches the canonical underscore strings directly and `ThreeMarksSeat::as_str()` returns them; `SeatId::parse_canonical` exists in `crates/engine-core/src/lib.rs`. Confirmed during reassessment.
2. Spec §3.4 and §5.4 classify this as `migrate` / ADR-0009 `unchanged` for accepted input; MSC-8C-002 owns the seat grammar; `parse_seat_import` keeps accepting bounded aliases import-only.
3. Cross-artifact: canonical seat identity is an `engine-core` contract (`SeatId`); the game maps it to typed `ThreeMarksSeat`. No TypeScript normalization (§2).
4. §2 behavior-authority motivates this ticket: Rust owns canonical seat identity; TypeScript MUST NOT normalize seat IDs.
5. Enforcement surface = the seat-ID grammar and downstream hashes/visibility; delegation keeps accepted values byte-identical and rejects malformed labels without leaking hidden information.

## Architecture Check

1. Delegating to the single kernel parser removes a hand-maintained string match and centralizes the canonical-seat definition.
2. No backwards-compatibility aliasing is added to the game parser; legacy aliases stay import-only at the WASM boundary.
3. `engine-core` stays noun-free (§3); `SeatId` is an allowed kernel noun; no `game-stdlib` change (§4).

## Verification Layers

1. Canonical `seat_0`/`seat_1` round-trip unchanged -> unit test + grep-proof (`SeatId::parse_canonical` called; `as_str()` preserved).
2. Malformed/out-of-range/leading-zero/Unicode-digit labels reject locally -> new direct rejection unit tests.
3. WASM import adapter still accepts bounded aliases -> existing `parse_seat_import` test stays green.

## What to Change

### 1. Delegate canonical parsing

Rewrite `ThreeMarksSeat::parse` to call `SeatId::parse_canonical` and map the bounded result to the typed enum; preserve `as_str()` exactly.

### 2. Add rejection tests

Add direct parser tests asserting rejection of leading-zero, Unicode-digit, hyphen, symbolic, and out-of-range spellings, and that `parse_seat_import` still accepts the bounded aliases.

## Files to Touch

- `games/three_marks/src/ids.rs` (modify)

## Out of Scope

- Any trace or golden-file change.
- Making legacy hyphen/symbolic aliases valid in the game-local parser.
- The WASM-output canonicalization (owned by the C-02 output tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks` is green, including new malformed/out-of-range rejection tests.
2. `cargo test -p wasm-api` is green — `parse_seat_import` still accepts bounded aliases.
3. `cargo run -p replay-check -- --game three_marks --all` passes with all downstream hashes/visibility unchanged.

### Invariants

1. Accepted canonical values and `as_str()` output are byte-identical.
2. The game-local parser rejects every non-canonical spelling; legacy aliases remain import-only at the WASM boundary.

## Test Plan

### New/Modified Tests

1. `games/three_marks/src/ids.rs` parser unit tests — canonical round-trip plus leading-zero/Unicode-digit/hyphen/symbolic/out-of-range rejection.

### Commands

1. `cargo test -p three_marks`
2. `cargo test -p wasm-api`
3. The game-parser unit tests plus the wasm-api import test are the correct boundary.
