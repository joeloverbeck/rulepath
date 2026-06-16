# RIVLEDSHOWUX-008: V2 reveal-scoped projection + bridge + TS types

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/visibility.rs`, `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`, `games/river_ledger/tests/{visibility,replay}.rs`
**Deps**: RIVLEDSHOWUX-007

## Problem

The V2 payload (RIVLEDSHOWUX-007) must reach the browser viewer-safe: projected per-viewer through `visibility.rs`, serialized through the WASM bridge, and typed in `client.ts` — with no-leak coverage proving folded/unrevealed seats carry no standings or card-usage for any viewer.

## Assumption Reassessment (2026-06-16)

1. Verified: `visibility.rs` holds the per-viewer projection path; `crates/wasm-api/src/lib.rs` exposes the terminal JSON; `apps/web/src/wasm/client.ts:801` defines `RiverLedgerPublicView`. V2 fields are produced by RIVLEDSHOWUX-007 (hence `Deps`).
2. Verified against spec §6 D6 + §8 WB8; `RULES.md` `RL-VIS-SHOWDOWN-001`, `RL-UI-NOLEAK-001`, `RL-REPLAY-EXPORT-001`.
3. Shared boundary under audit: the `crates/wasm-api` JSON seam + the viewer-hash / replay-export surfaces — the projection must preserve pairwise seat-private redaction across 3–6 seats.
4. FOUNDATIONS §11 (per-viewer projection; hidden information does not leak to payloads, DOM, replay exports) motivates this ticket.
5. No-leak firewall / determinism: each seat viewer and the public observer see standings/card-usage only for authorized reveals; folded/unrevealed seats carry only `folded_rows`; viewer-hash and replay-export coverage proves no private leak (§11; `RL-REPLAY-EXPORT-001`).
6. Schema extension: the bridge terminal JSON + `RiverLedgerPublicView` TS type gain the V2 fields; consumer is the renderer (RIVLEDSHOWUX-009); additive-only.

## Architecture Check

1. Routing V2 through the existing `visibility.rs` → `wasm-api` → `client.ts` seam (the documented bridge contract) keeps TS presentation-only and the projection Rust-owned.
2. No shims; the TS type mirrors the Rust shape, adding no logic.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Pairwise no-leak across 3–6 seats -> `games/river_ledger/tests/visibility.rs` (per-seat + observer projection of V2).
2. Replay export carries no unauthorized standings/usage -> `games/river_ledger/tests/replay.rs` + `cargo run -p replay-check -- --game river_ledger --all`.
3. Bridge JSON ↔ TS type parity -> `npm --prefix apps/web run build` + `npm --prefix apps/web run smoke:wasm`.

## What to Change

### 1. `games/river_ledger/src/visibility.rs`

Project V2 per-viewer: standings/card-usage only for authorized revealed seats; folded/unrevealed → `folded_rows` redaction; observer sees only public-authorized reveals.

### 2. `crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`

Expose the V2 terminal JSON; add the matching `RiverLedgerPublicView` V2 fields to the TS type.

## Files to Touch

- `games/river_ledger/src/visibility.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)
- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- The V2 payload construction (RIVLEDSHOWUX-007).
- The V2 renderer (RIVLEDSHOWUX-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` + `cargo test -p wasm-api` — V2 projects viewer-safe; folded/unrevealed seats carry no standings/usage for any viewer.
2. `cargo run -p replay-check -- --game river_ledger --all` — replay export leak-free and deterministic.
3. `npm --prefix apps/web run build` + `npm --prefix apps/web run smoke:wasm` — bridge JSON ↔ TS type parity.

### Invariants

1. Pairwise seat-private redaction preserved across 3–6 seats (§11 no-leak firewall).
2. TS type mirrors the Rust shape and adds no logic (§2).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — per-seat + observer V2 projection no-leak.
2. `games/river_ledger/tests/replay.rs` — V2 replay-export redaction.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. `npm --prefix apps/web run smoke:wasm`
