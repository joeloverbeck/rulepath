# GAT91SECDRACOM-007: secret_draft replay support — full trace + viewer-scoped public export/import

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft/src/replay_support.rs`; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-006

## Problem

The game needs deterministic replay: an internal full trace (native test authority) and a viewer-scoped public export/import that, before reveal, contains only pending booleans/effects with item IDs redacted. Under ADR 0004, browser export defaults to the viewer-scoped observation timeline and must not let pre-reveal commitments, raw private action paths, or seed material reconstructing hidden choices escape.

## Assumption Reassessment (2026-06-08)

1. `games/high_card_duel/src/replay_support.rs` is the direct behavioral precedent (it implements the viewer-scoped export split for a hidden-info card game); `games/token_bazaar/src/replay_support.rs` is the structural precedent. Verified both crates carry `replay_support.rs`.
2. Views + effect filtering from GAT91SECDRACOM-006 and the effect set from GAT91SECDRACOM-005 are inputs. Spec §"Internal full trace vs browser export" + assumptions A7 (internal traces may carry raw private action paths; browser exports must not) define the contract.
3. Cross-artifact boundary under audit: the replay/checkpoint/hash and serialization-boundary contracts (`docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`) and ADR 0004 (`docs/adr/0004-hidden-info-replay-export-taxonomy.md`, confirmed present). Public export/import replays an observation timeline, not omniscient internal state.
4. §11 determinism + no-leak are the motivating invariants: restate before trusting spec — same seed + seats + options + command stream reproduces state/effect/action-tree/view hashes and reveal ordering (this is the GAT91SECDRACOM-010 replay-test contract). Before `ChoicesRevealed`, the public export contains only pending booleans/effects with item IDs redacted; after reveal, item IDs are public and may appear.
5. This does not change replay/hash *semantics* repo-wide (no §13 ADR trigger) — it implements a new game's replay surface within the existing deterministic contract and ADR 0004's already-accepted export taxonomy.

## Architecture Check

1. Deriving the public export from the viewer-scoped observation timeline (GAT91SECDRACOM-006's filtered effects/views) rather than re-redacting the internal trace is cleaner: the firewall is reused, not re-implemented, so export cannot drift from view redaction.
2. No backwards-compatibility aliasing/shims — fills GAT91SECDRACOM-002 stubs.
3. `engine-core` stays noun-free (generic replay/checkpoint/hash contracts reused); draft nouns game-local. No `game-stdlib` helper.

## Verification Layers

1. Deterministic replay -> replay unit test: same inputs reproduce state/effect/view/action-tree hashes and reveal ordering (full golden-trace coverage in GAT91SECDRACOM-010).
2. Export no-leak -> no-leak test: pre-reveal public export contains pending booleans only, no committed item ID, no raw private action path, no reconstructing seed material.
3. Export/import round-trip -> test: public export imports to the same observation timeline.
4. Serialization order -> stable-summary serialization test.

## What to Change

### 1. `src/replay_support.rs`

Implement: internal full trace (command stream, private action paths, hidden commitments, state hashes, seed evidence) for native test authority; viewer-scoped public export defaulting to the observation timeline with pre-reveal item IDs redacted; public import replaying that observation timeline; stable summaries and the action/effect/view hash surfaces consumed by GAT91SECDRACOM-010.

## Files to Touch

- `games/secret_draft/src/replay_support.rs` (modify)

## Out of Scope

- Golden trace files and the `tests/replay.rs` harness (GAT91SECDRACOM-010).
- WASM export wiring (GAT91SECDRACOM-013) — though it calls this surface.
- Any change to repo-wide replay/hash semantics (would require a §13 ADR; not in scope).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft replay` — deterministic-replay and export/import round-trip tests pass.
2. Pre-reveal public-export no-leak test passes (no item ID / private path / seed material).
3. Internal full trace reproduces state and effect hashes deterministically.

### Invariants

1. Same seed + seats + options + commands → identical hashes and reveal ordering (§11/§2 determinism).
2. Browser/public export carries no pre-reveal hidden choice or reconstructing material (§11 no-leak, ADR 0004).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/replay_support.rs` inline unit tests — deterministic replay, export/import round-trip, pre-reveal export redaction.

### Commands

1. `cargo test -p secret_draft replay`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Targeted `replay` filter is the correct boundary; cross-game replay validation via `replay-check --all` lands after tool registration (GAT91SECDRACOM-012) and golden traces (GAT91SECDRACOM-010).
