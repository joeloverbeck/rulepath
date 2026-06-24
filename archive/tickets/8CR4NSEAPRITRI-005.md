# 8CR4NSEAPRITRI-005: Briar Circuit C-02 canonical seat parser adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/briar_circuit` (`src/ids.rs`); accepted/rejected seat inputs and enum mapping unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

`games/briar_circuit/src/ids.rs::BriarCircuitSeat::parse` hand-matches `seat_0…seat_3` instead of delegating strict canonical parsing to `SeatId::parse_canonical` (MSC-8C-002). Adopt the canonical helper plus a bounded enum conversion with identical acceptance, rejection, and enum mapping (spec §3.5 Briar parser, §5.4). C-02 applies to seat identity only — card/trick/contract IDs are out of scope.

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/ids.rs::BriarCircuitSeat::parse` currently matches `seat_0`…`seat_3` literally and returns `None` otherwise; `SeatId::parse_canonical` exists in `crates/engine-core/src/lib.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.5 classifies this as `migrate`; register MSC-8C-002 and `archive/tickets/UNI8CMECSCA-009.md` own the canonical seat grammar; River's parser already delegates (pilot credit).
3. Cross-artifact: the canonical seat-string contract lives in `engine-core::SeatId`; the formatter/roster surface is migrated separately in `-006`, the WASM import aliases in `-007`. Baseline accepted/rejected strings come from `-001`.
4. §11 acceptance invariant motivates this ticket: the accepted set stays exactly `seat_0…seat_3`, and malformed/out-of-range strings remain rejected — no widening or narrowing of seat acceptance.
5. Enforcement surface = seat-string parse acceptance + enum mapping in the Briar ID tests; delegation changes no accepted/rejected string and no canonical output, so no hidden-info or determinism path is affected.

## Architecture Check

1. Delegating strict canonical parse to the kernel helper removes a hand-rolled per-seat match and routes acceptance through the single owned grammar.
2. No backwards-compatibility aliasing or shim is introduced; legacy hyphen aliases stay an import-only concern owned by `-007`, not added here.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Accepted/rejected seat set unchanged -> table-driven canonical/malformed/out-of-range parse test in the Briar ID tests.
2. Enum mapping (`seat_N` -> `BriarCircuitSeat::SeatN`) preserved -> codebase grep-proof + focused unit assertion.
3. Canonical helper adopted -> codebase grep-proof (`SeatId::parse_canonical` present in `parse`; the manual `seat_0…seat_3` match gone).

## What to Change

### 1. Delegate strict parse to `SeatId::parse_canonical`

In `BriarCircuitSeat::parse`, parse the input via `SeatId::parse_canonical`, then map the bounded index to the `BriarCircuitSeat` enum, rejecting out-of-range exactly as today. Preserve the `Option` return contract.

## Files to Touch

- `games/briar_circuit/src/ids.rs` (modify)

## Out of Scope

- The formatter/roster migration (`-006`) and WASM import-alias adapter (`-007`).
- Applying C-02 to card, trick, contract, or other non-seat IDs.
- Any canonical output-string change or new alias acceptance.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including a table-driven parse test (canonical accepted, malformed/out-of-range rejected).
2. `cargo run -p replay-check -- --game briar_circuit --all` passes (seat round-trips unchanged).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The accepted seat set is exactly `seat_0…seat_3`; rejection of malformed/out-of-range inputs is unchanged.
2. No new public symbol or shim is introduced; enum mapping is identical.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/src/ids.rs` test module (or `tests/`) — add/strengthen a table-driven canonical/malformed/out-of-range parse assertion.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game test is the correct boundary: seat parsing is game-local ID grammar over a kernel helper.

## Outcome

Completed: 2026-06-24

What changed:
- `BriarCircuitSeat::parse` now delegates strict seat-string parsing to `SeatId::parse_canonical`, extracts the canonical zero-based index, and maps the bounded index through the existing enum conversion.
- Added a focused table-driven test covering canonical `seat_0` through `seat_3` acceptance plus malformed and out-of-range rejection.

Deviations:
- None. Formatter/roster adoption and WASM alias adapter work remain owned by `8CR4NSEAPRITRI-006` and `8CR4NSEAPRITRI-007`.

Verification:
- `cargo fmt --all --check`
- `cargo test -p briar_circuit`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `bash scripts/boundary-check.sh`
