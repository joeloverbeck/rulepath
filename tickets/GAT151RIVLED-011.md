# GAT151RIVLED-011: Replay, hash, and serialization v2 migration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`replay_support.rs`, `lib.rs`, `ids.rs`), `data/manifest.toml`, tests; intentional per-game v2 drift
**Deps**: GAT151RIVLED-010

## Problem

All behavior-critical stack, reopen, ordered-pot, return, and award state must participate in deterministic serialization and hashing, which requires advancing River Ledger to a per-game rules/data **v2** baseline. Existing v1 replays must be handled deterministically (reject with a stable diagnostic, or pass through an explicit tested converter) — never silently reinterpreted under v2 stack rules. The global Trace Schema v1 stays unchanged; this is intentional per-game drift, not a global migration.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/ids.rs::RULES_VERSION_LABEL = "river-ledger-rules-v1"`; `data/manifest.toml` `rules_version = 1`, `data_version = 1`; `replay_support.rs` provides viewer-scoped export over the engine-core recorder. The new stack/reopen/pot/return/award state from GAT151RIVLED-004–010 is not yet hashed/serialized.
2. Docs: spec §7.6 — global `docs/TRACE-SCHEMA-v1.md` unchanged; River Ledger advances to v2; all outcome-affecting state participates in deterministic serialization/hash; v1 imports either fail with a deterministic version-mismatch diagnostic or pass an explicit converter; ADR 0004 viewer-scoped export preserved (no terminal auto-reveal). This ticket realizes the version plan authored in GAT151RIVLED-001.
3. Cross-artifact boundary under audit: the serialization-boundary/hash contract (`engine-core` hash + `docs/ENGINE-GAME-DATA-BOUNDARY.md`) ↔ the resolved-result shape from GAT151RIVLED-009 and projections from -010 — all must serialize in stable order.
4. (§11/§13 determinism + ADR boundary) Restate: the change to River Ledger's replay/hash *content* is intentional per-game v2 drift; it does NOT change the global hash algorithm, replay envelope, or Trace Schema v1 (which would trip a §13 ADR). Confirm identical inputs+v2 produce identical hashes and that no nondeterministic input enters canonical forms.
5. (rename/version blast radius) Bumping `RULES_VERSION_LABEL` + `manifest.toml` versions starts the expected CI red window for `check-player-rules` / `rule-coverage` / `replay-check` / golden traces, resolved by GAT151RIVLED-017 (traces) and -019 (docs). No other crate hardcodes `river-ledger-rules-v1`.

## Architecture Check

1. Hashing all outcome-affecting state behind a single per-game version bump makes the v2 drift explicit, reviewable, and replay-safe, versus silently changing hashes under the v1 label.
2. No backwards-compatibility shims; v1 imports are explicitly rejected or converted, never reinterpreted.
3. Version/hash vocabulary stays game-local; the global Trace Schema v1 and engine-core hash contract are unchanged (§3/§13).

## Verification Layers

1. Replay equivalence + stable viewer hashes -> deterministic replay-hash check (`docs/TESTING-REPLAY-BENCHMARKING.md`).
2. Stable serialization/iteration order of pots/returns/awards -> serialization tests independent of container iteration.
3. v1 import rejected/converted deterministically -> version-mismatch unit test.
4. No global Trace Schema v1 change -> grep-proof on `docs/TRACE-SCHEMA-v1.md` (unchanged) + boundary check.

## What to Change

### 1. v2 versioning + hashed state

Bump `RULES_VERSION_LABEL` to v2 and `manifest.toml` `rules_version`/`data_version`; include stack/reopen/pot/return/award state in stable summaries, hashes, serialization, and viewer-scoped export.

### 2. v1 replay-import policy

Reject v1 replays with a stable deterministic version-mismatch diagnostic (default), or supply an explicit tested converter with exact historical semantics; record the intentional v2 drift in a migration note.

## Files to Touch

- `games/river_ledger/src/replay_support.rs` (modify)
- `games/river_ledger/src/lib.rs` (modify)
- `games/river_ledger/src/ids.rs` (modify)
- `games/river_ledger/data/manifest.toml` (modify)
- `games/river_ledger/tests/replay.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)

## Out of Scope

- Authoring/regenerating golden traces (GAT151RIVLED-017).
- RULES/HOW-TO-PLAY/RULE-COVERAGE doc version reconciliation (GAT151RIVLED-019).
- Any global Trace Schema / hash-algorithm change (would require an ADR; not in scope).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — replay equivalence, stable serialization order, viewer hash, and v1-import version-mismatch behavior.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free; no global contract change.
3. `cargo run -p replay-check -- --game river_ledger --all` — re-recorded v2 replays are internally consistent (golden-trace reconciliation deferred to GAT151RIVLED-017).

### Invariants

1. Identical inputs + v2 produce identical hashes and serialization order; no nondeterministic input enters canonical forms.
2. `docs/TRACE-SCHEMA-v1.md` and the global hash algorithm are unchanged; v1 imports are never silently reinterpreted.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — v2 replay equivalence and v1-import rejection/conversion.
2. `games/river_ledger/tests/serialization.rs` — stable ordering of stack/reopen/pot/return/award state.

### Commands

1. `cargo test -p river_ledger`
2. `bash scripts/boundary-check.sh`
3. `cargo run -p replay-check -- --game river_ledger --all` — confirms v2 internal consistency; full golden-trace hashes are owned by GAT151RIVLED-017.
