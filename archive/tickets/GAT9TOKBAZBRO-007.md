# GAT9TOKBAZBRO-007: Replay support — full trace + public export/import

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/src/replay_support.rs` (new), `src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-006

## Problem

Even though Token Bazaar is fully public, it must satisfy the same determinism
discipline as prior games: a seed + variant + command stream must reproduce the
final state, effects, action-tree hashes, public-view hashes, outcome, and
terminal state, and a public replay must export/import losslessly. This ticket
implements the replay support surface that the replay-check tool, golden traces,
and WASM export/import consume.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/{state,rules,effects,visibility}.rs` (GAT9TOKBAZBRO-003/
   004/005/006) provide the setup, validated commands, effects, and public view
   this replay layer threads. The sibling `games/high_card_duel/src/replay_support.rs`
   establishes the house pattern (verified present) — but `high_card_duel` carries a
   hidden-info public/viewer-scoped export split (ADR 0004); Token Bazaar is fully
   public, so there is a single public export path with no redaction split.
   `src/lib.rs` modified to add `mod replay_support;`.
2. The replay requirements are fixed by `specs/gate-9-token-bazaar-browser-proof.md`
   → "Replay, export, and no-leak requirements" (reproduce final state/effects/
   action-tree hashes/public-view hashes/outcome/terminal; public export/import via
   WASM; invalid commands produce stable diagnostics without mutation; stale tokens
   reject without mutation).
3. Cross-artifact boundary under audit: the replay + checkpoint + hash contract
   from `docs/TESTING-REPLAY-BENCHMARKING.md` and `engine-core`'s replay surface.
   This must conform to the engine replay/hash contract; it does not change
   replay/hash *semantics* (no §13 ADR trigger — spec confirms).
4. FOUNDATIONS §11 (deterministic replay/hash/serialization): identical seed +
   variant + command stream → identical state, effects, action-tree hash, and
   public-view hash. No wall-clock or nondeterministic input enters canonical
   forms.
5. Replay/hash determinism + no-leak surface: the public export must contain only
   public-safe content. Since the game is fully public there is no hidden field to
   strip, but the export must not embed bot rationale/candidate/debug data; the
   no-leak negative test (-009) and the export golden trace (-010) assert this.
   Confirm the single public export path introduces no nondeterminism the engine
   replay contract would have to undo.
6. Replay schema: this ticket produces the command-stream + checkpoint + hash
   artifacts consumed by `replay-check` (-012), golden traces (-010), and WASM
   export/import (-013). Additive for this game; the hashed surfaces (action-tree,
   public view, diagnostic) are enumerated for the trace ticket.

## Architecture Check

1. A single public replay path (no viewer-scoped split) is the correct minimal
   design for a fully-public game — adding `high_card_duel`'s redaction split here
   would be unused complexity. The seam is still the normal engine replay contract,
   so future hidden-state games are not blocked.
2. No backwards-compatibility aliasing/shims — new file.
3. `engine-core` untouched; replay support uses the kernel's generic replay/hash
   contract over an opaque game payload. No `game-stdlib` helper added.

## Verification Layers

1. Deterministic reproduction -> deterministic replay-hash check (full golden
   traces + `replay-check --all` in -010/-012); targeted replay round-trip unit
   test here.
2. Public export/import lossless -> export/import round-trip unit test (and the
   `wasm-exported` / public-export golden trace in -010).
3. Invalid + stale commands reject without mutation under replay -> unit tests
   reusing the -004 diagnostics.
4. No-leak: export carries no bot/debug/candidate field -> no-leak assertion
   (full suite in -009).

## What to Change

### 1. `games/token_bazaar/src/replay_support.rs`

The replay surface: drive a command stream from setup, expose deterministic
action-tree + public-view + diagnostic hashes, and a single public export/import
of the command stream + checkpoint that reproduces the final public state and
outcome. Reuse the -004 validation so invalid/stale commands reject without
mutation during replay.

### 2. `games/token_bazaar/src/lib.rs` (modify)

Add `mod replay_support;`; re-export the replay surface.

## Files to Touch

- `games/token_bazaar/src/replay_support.rs` (new)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- The `replay-check` tool registration (GAT9TOKBAZBRO-012) and WASM export wiring
  (GAT9TOKBAZBRO-013).
- The golden-trace files + replay test (GAT9TOKBAZBRO-010) — only a targeted
  round-trip unit test here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — seed+variant+command-stream reproduces final
   state, effects, action-tree hash, public-view hash, outcome, terminal.
2. `cargo test -p token_bazaar` — public export → import → identical public state.
3. `cargo test -p token_bazaar` — invalid and stale commands reject without
   mutation during replay.

### Invariants

1. Replay is byte-deterministic: identical inputs + versions → identical hashes.
2. No nondeterministic input (wall-clock, hash-map order) enters any canonical
   replay form.
3. The public export contains no bot/debug/candidate/internal field.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/replay_support.rs` (unit) — replay reproduction + export/import round-trip.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo build -p token_bazaar && bash scripts/boundary-check.sh`
3. Full `replay-check -- --game token_bazaar --all` runs in GAT9TOKBAZBRO-012 once
   the tool arm + traces exist; the per-crate round-trip is the correct boundary here.

## Outcome

Completed: 2026-06-08

What changed:

- Added `games/token_bazaar/src/replay_support.rs` with deterministic command
  replay from setup, state/effect/action-tree/public-view/replay hash helpers,
  and replay step projections.
- Added public replay export/import structures with stable JSON bytes for the
  fully public Token Bazaar replay timeline.
- Updated `src/lib.rs` exports for replay helpers and replay data structures.

Deviations from original plan:

- None.

Verification results:

- `cargo test -p token_bazaar` passed with 32 tests.
- `cargo build -p token_bazaar` passed.
- `bash scripts/boundary-check.sh` passed.
