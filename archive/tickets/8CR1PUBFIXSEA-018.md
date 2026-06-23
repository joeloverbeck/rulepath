# 8CR1PUBFIXSEA-018: Token Bazaar WASM output canonical seat migration

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `wasm-api` (`src/games/token.rs`, `src/tests.rs`) + one golden trace
**Deps**: 8CR1PUBFIXSEA-012

## Problem

`crates/wasm-api/src/games/token.rs::token_replay_document_json` still emits legacy hyphen seat spellings in its roster even though `trace_token_seat` is already canonical. Migrate this one document's roster/actor/outcome seat IDs to canonical `seat_<n>` — an ADR-0009 `intentional-migration` that changes exactly one golden trace (`games/token_bazaar/tests/golden_traces/wasm-exported.trace.json`) while preserving `PublicReplayExport` semantics and public-export hash authority. The document already carries incidental non-seat `seat_<n>`-shaped tokens; only seat-ID fields may change. Old hyphen documents stay importable.

## Assumption Reassessment (2026-06-23)

1. `token_replay_document_json` (`crates/wasm-api/src/games/token.rs`) still emits legacy hyphen roster spellings; `trace_token_seat` (`crates/wasm-api/src/seats.rs`) is already canonical and must remain unchanged. The Token Bazaar `wasm-exported.trace.json` carries both hyphen seat IDs and incidental `seat_<n>` tokens — confirmed during reassessment; `-001` pins which fields are seat IDs.
2. Spec §3.4 and §5.5 (task `8C-R1-216`) classify this as `intentional-migration`; only this game's `wasm-exported.trace.json` may change, and `trace_token_seat` stays unchanged. MSC-8C-002 owns the seat grammar; ADR-0009 governs the receipt.
3. Cross-artifact: consumer `games/token.rs` document → `tests.rs` assertion → the one golden trace, with the `PublicReplayExport` projection (`games/token_bazaar/src/replay_support.rs`) downstream. Seat-grammar contract is `engine-core::SeatId`.
4. §11 determinism/no-leak motivates this ticket: only public seat-ID spellings change; `PublicReplayExport` semantics, public-export hash authority, and incidental tokens stay byte-identical.
5. Enforcement surface = the WASM exported document bytes, its golden trace, and the public-export hash authority; the migration changes only seat-ID-bearing fields pinned in `-001`, leaks no hidden information, and keeps old hyphen documents importable.

## Architecture Check

1. Flipping one document while leaving the already-canonical `trace_token_seat` untouched is the minimal reversible change, versus a six-document flag-day rewrite (forbidden).
2. No backwards-compatibility reader is removed; `parse_seat_import` still accepts hyphen/symbolic aliases.
3. `engine-core` stays noun-free (§3); change lives in `wasm-api`; no `game-stdlib` change (§4).

## Verification Layers

1. Document roster/actor/outcome emit canonical `seat_<n>`; `PublicReplayExport` bytes preserved -> golden trace / deterministic replay check (`replay-check --game token_bazaar --all`) + byte digest before/after.
2. Old hyphen document still imports; new document replays -> `wasm-api` round-trip test in `tests.rs`.
3. Only this golden file changed -> `git diff --name-only` shows exactly the Token Bazaar WASM trace among trace files.

## What to Change

### 1. Canonicalize only the seat-ID fields in the document

In `token_replay_document_json`, emit canonical seat IDs for roster, command actors, and outcome seats. Leave `trace_token_seat` (already canonical) and incidental non-seat tokens unchanged. Preserve `PublicReplayExport` semantics and hash authority.

### 2. Update the assertion and golden trace

Update the `crates/wasm-api/src/tests.rs` expectation and regenerate ONLY `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json`, recording the before/after byte+hash receipt.

## Files to Touch

- `crates/wasm-api/src/games/token.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)
- `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` (modify)

## Out of Scope

- `trace_token_seat` (already canonical — must not change).
- `default_seats`, non-WASM traces, game state bytes, or any other game's WASM document.
- Any second export-format change (the public-export profile is owned by `-036`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green with the updated `tests.rs` expectation.
2. `cargo run -p replay-check -- --game token_bazaar --all` passes; the new document replays, public visibility, `PublicReplayExport` semantics, and gameplay/public-export hash authority remain valid.
3. `git diff --name-only` shows no other `wasm-exported.trace.json` changed.

### Invariants

1. Exactly one golden trace (`token_bazaar` WASM export) changes; all other traces are byte-identical.
2. `trace_token_seat`, incidental non-seat tokens, and `PublicReplayExport` bytes are unchanged; old hyphen documents remain importable.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` — canonical roster seat spellings, export bytes unchanged (migration receipt recorded).
2. `crates/wasm-api/src/tests.rs` — updated expected document + old-import / new-replay assertions.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game token_bazaar --all`
3. The per-game replay-check plus the wasm-api round-trip test is the correct boundary.

## Outcome

Completed on 2026-06-23.

- Token Bazaar WASM replay export now emits canonical `seat_0` / `seat_1` roster IDs and normalizes command actor IDs while leaving the already-canonical `trace_token_seat` helper unchanged.
- `crates/wasm-api/src/tests.rs` now asserts canonical Token Bazaar export output, verifies no legacy `seat-0` remains in the new export, and verifies legacy hyphen-spelled Token Bazaar export documents remain importable.
- Updated only `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` among `wasm-exported.trace.json` golden files, and recorded the before/after SHA-256 receipt in `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md`.
- The selected golden diff changed only the two roster `seat_id` fields; existing command actors, incidental non-seat `seat_<n>` tokens, and expected public-export hash authority were unchanged.
- Regenerated `crates/wasm-api/tests/snapshots/api_surface.tsv` after the intended Token Bazaar `export_replay` snapshot drift.

Verification:

- `cargo fmt --all -- --check`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game token_bazaar --all`
- `git diff --name-only -- games/*/tests/golden_traces/wasm-exported.trace.json` -> `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json`
- `git diff -- games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` showed only roster `seat_id` changes.

Deviation:

- Initial `cargo test -p wasm-api` failed only in `tests/api_surface.rs::public_api_surface_matches_snapshot` because the public API snapshot still expected Token Bazaar roster `seat-0` / `seat-1`. The intended snapshot was regenerated with `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface`, then `cargo test -p wasm-api` passed.
