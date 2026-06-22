# UNI8CMECSCA-008: One `wasm-api` seat import adapter (canonical + legacy aliases)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/seats.rs`
**Deps**: UNI8CMECSCA-007, UNI8CMECSCA-003

## Problem

`crates/wasm-api/src/seats.rs` carries per-game seat parsing branches, each re-implementing canonical/legacy acceptance. This ticket replaces those import branches with one adapter that accepts canonical underscore `seat_<n>`, bounded legacy hyphen `seat-<n>`, and symbolic legacy values (`seat-a`) only through an explicit caller-provided seat-order alias table — normalizing every successful import immediately to typed canonical `SeatId` via the UNI8CMECSCA-007 kernel parser. Unknown/ambiguous labels reject; TypeScript does not repair them.

## Assumption Reassessment (2026-06-22)

1. `crates/wasm-api/src/seats.rs` currently has ~14 per-game `parse_*`/`parse_replay_*`/`trace_*` functions; some already dual-accept (`"seat-0" | "seat_0"`), and several `trace_*` adapters emit legacy hyphen (e.g. `trace_race_seat` → `"seat-0"`). The kernel `SeatId::parse_canonical` exists after UNI8CMECSCA-007.
2. Spec §4.3 C-02 and §5 8C-008 fix the adapter: canonical underscore, bounded legacy hyphen, explicit symbolic alias table; preserve existing error classes/messages unless a named compatibility test approves a change. The UNI8CMECSCA-003 packet records each game's current import cases and seat spellings.
3. Cross-artifact boundary under audit: the WASM seat transport contract (`docs/WASM-CLIENT-BOUNDARY.md`) and the per-game seat enums the adapter feeds. The import side is unified; **output** is handled per the M1 scope note below.
4. FOUNDATIONS §2: seat normalization stays in Rust; no TypeScript normalization is added. Unknown/ambiguous values reject (fail-closed, §11).
5. No-leak/determinism (§11): the adapter normalizes public seat identity only; it adds no hidden state. **M1 (spec §4.3 scope note):** "output paths emit only underscore canonical IDs" governs the import adapter and newly-written/migrated output — it does **not** silently route an existing hyphen-emitting `trace_*` adapter through the canonical formatter. Flipping such a surface (and the committed `race_to_n`/`draughts_lite`/`high_card_duel` hyphen goldens it feeds) is a hash-bearing change reserved for the named ADR-0009 migration in UNI8CMECSCA-009, not this ticket.

## Architecture Check

1. One import adapter with an explicit alias table removes the duplicated per-game branches and routes all canonicalization through the single kernel parser.
2. No backwards-compatibility shim beyond the bounded, explicit legacy-alias table the spec sanctions; symbolic aliases require a caller-provided order table, not a guess.
3. `engine-core` untouched; the adapter is transport-boundary code in `wasm-api`, the lawful home for legacy compatibility.

## Verification Layers

1. Every supported game import case (canonical, bounded hyphen, symbolic-via-table) parses to typed canonical `SeatId` → `wasm-api` table tests.
2. Unknown/ambiguous labels reject with the preserved error class/message → rejection table test.
3. No existing golden trace byte changes (no silent output flip) → `cargo run -p replay-check -- --game race_to_n --all`, `--game river_ledger --all`.
4. No TypeScript normalization added → grep-proof that seat parsing stays in `wasm-api`/`engine-core` (none added under `apps/web`).

## What to Change

### 1. `crates/wasm-api/src/seats.rs` — unified import adapter

Replace the per-game import branches with one adapter: canonical underscore, bounded legacy hyphen, explicit symbolic alias table; normalize to typed canonical `SeatId` on success; reject unknown/ambiguous. Preserve current error classes/messages. Leave existing `trace_*` output adapters as-is (their migration is UNI8CMECSCA-009-gated).

### 2. Tests

Per-game import table tests (canonical/hyphen/symbolic) and a rejection table.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify)

## Out of Scope

- Flipping any `trace_*` output adapter from hyphen to underscore (named migration in UNI8CMECSCA-009).
- Changing any golden trace or fixture bytes.
- Adopting the adapter at game call sites beyond the existing wiring (UNI8CMECSCA-009).
- TypeScript-side seat handling.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` passes, including the per-game import + rejection tables.
2. `cargo run -p replay-check -- --game race_to_n --all` and `--game river_ledger --all` pass with unchanged hashes (no output flip).
3. `cargo test --workspace` passes.

### Invariants

1. Every output path the adapter newly writes emits underscore canonical; existing hyphen `trace_*` adapters are unchanged.
2. Unknown/ambiguous labels reject; no TypeScript normalization exists.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/seats.rs` (or `src/tests.rs`) — per-game import table + rejection table.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game race_to_n --all`
3. `wasm-api` tests plus `replay-check` are the correct boundary — the import unification must not perturb any committed trace byte.
