# 8CR1PUBFIXSEA-007: Draughts Lite C-02 strict canonical seat parser

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/draughts_lite` (`src/ids.rs`); accepted canonical input unchanged, no trace/golden change
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/draughts_lite/src/ids.rs::DraughtsLiteSeat::parse` hand-rolls canonical seat parsing (`match value { "seat_0" => …, "seat_1" => …, _ => None }`) instead of delegating to the kernel canonical parser `SeatId::parse_canonical` (MSC-8C-002). Adopt strict canonical delegation plus a bounded enum mapping, preserving accepted values and `as_str()`, and add direct rejection tests for malformed/out-of-range spellings. The WASM import adapter (`crates/wasm-api/src/seats.rs::parse_seat_import`) remains the only place bounded legacy hyphen/symbolic aliases are accepted.

## Assumption Reassessment (2026-06-23)

1. `DraughtsLiteSeat::parse` currently matches the canonical underscore strings directly and `DraughtsLiteSeat::as_str()` returns them; `SeatId::parse_canonical` exists in `crates/engine-core/src/lib.rs`. Confirmed during reassessment.
2. Spec §3.4 and §5.4 classify this as `migrate` / ADR-0009 `unchanged` for accepted input; register entry MSC-8C-002 owns the seat grammar. `parse_seat_import` (wasm-api) keeps accepting bounded hyphen/symbolic aliases — import-only.
3. Cross-artifact: canonical seat identity is an `engine-core` contract (`SeatId`); the game maps it to its typed `DraughtsLiteSeat` enum. No TypeScript normalization is involved (§2).
4. §2 behavior-authority motivates this ticket: Rust owns canonical seat identity; TypeScript MUST NOT normalize or repair seat IDs. Strictness moves into the typed Rust parser.
5. Enforcement surface = the seat-ID grammar and all downstream hashes/visibility; delegating to `parse_canonical` keeps accepted values byte-identical (no hash/visibility drift) and adds rejection of malformed labels without leaking hidden information.

## Architecture Check

1. Delegating to the single kernel parser removes a hand-maintained string match from the game and makes "what is a canonical seat" one owned definition.
2. No backwards-compatibility aliasing is added to the game parser; legacy aliases stay import-only at the WASM boundary (no shim in the game).
3. `engine-core` stays noun-free (§3); `SeatId` is an allowed kernel noun; no `game-stdlib` change (§4).

## Verification Layers

1. Canonical `seat_0`/`seat_1` round-trip unchanged -> unit test + codebase grep-proof (`SeatId::parse_canonical` called; `as_str()` preserved).
2. Malformed/out-of-range/leading-zero/Unicode-digit labels reject locally -> new direct rejection unit tests.
3. WASM import adapter still accepts bounded hyphen/symbolic aliases -> existing `parse_seat_import` test stays green.

## What to Change

### 1. Delegate canonical parsing

Rewrite `DraughtsLiteSeat::parse` to call `SeatId::parse_canonical` and map the bounded result to the typed `Seat0`/`Seat1` enum; preserve `as_str()` output exactly.

### 2. Add rejection tests

Add direct parser tests asserting rejection of leading-zero, Unicode-digit, hyphen, symbolic, and out-of-range spellings at the game parser, and that `parse_seat_import` still accepts the bounded aliases.

## Files to Touch

- `games/draughts_lite/src/ids.rs` (modify)

## Out of Scope

- Any trace or golden-file change (none allowed in this wave).
- Making legacy hyphen/symbolic aliases valid in the game-local parser.
- The WASM-output canonicalization (owned by the C-02 output tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` is green, including new malformed/out-of-range rejection tests.
2. `cargo test -p wasm-api` is green — `parse_seat_import` still accepts bounded hyphen/symbolic aliases.
3. `cargo run -p replay-check -- --game draughts_lite --all` passes with all downstream hashes/visibility unchanged.

### Invariants

1. Accepted canonical values and `as_str()` output are byte-identical.
2. The game-local parser rejects every non-canonical spelling; legacy aliases remain import-only at the WASM boundary.

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/src/ids.rs` parser unit tests — canonical round-trip plus leading-zero/Unicode-digit/hyphen/symbolic/out-of-range rejection.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo test -p wasm-api`
3. The game-parser unit tests plus the wasm-api import test are the correct boundary: strictness is game-local, alias acceptance is WASM-boundary-local.
