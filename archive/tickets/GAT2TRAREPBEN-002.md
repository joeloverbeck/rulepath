# GAT2TRAREPBEN-002: Extract `race_to_n` replay/hash support module

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/race_to_n/src/replay_support.rs` game-local module; `games/race_to_n/src/lib.rs` exports it; `games/race_to_n/tests/replay_tests.rs` consumes it. No `engine-core` / `game-stdlib` change.
**Deps**: GAT2TRAREPBEN-001

## Problem

Canonical `race_to_n` replay/hash evaluation currently lives only in test-only
helper code inside `tests/replay_tests.rs`. The Gate 2 tools (`replay-check`,
`trace-viewer`, `seed-reducer`) must replay through the *same* Rust path that the
tests use, not a re-implementation. This ticket extracts the canonical replay/hash
evaluation into a reusable game-local module so tests and tools share one
authority (spec §D2).

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/tests/replay_tests.rs` holds the test-only helpers
   `replay_commands`, `replay_bot_action`, `replay_invalid`, `hashes_for_state`,
   `effect_hash`, `action_tree_hash`, `parse_fixture`, `assert_fixture`. No
   `games/race_to_n/src/replay_support.rs` exists (verified absent). `engine-core`
   already exposes the generic contracts these use (`ReplayRecord`, `HashValue`,
   `ActionTree`, `CommandEnvelope`, `EffectEnvelope`, `Checkpoint`) in
   `crates/engine-core/src/replay.rs` and `action.rs`.
2. Spec §D2 / §WB2 prescribe `games/race_to_n/src/replay_support.rs` (or equivalent
   game-local module), forbid putting game-specific trace logic in `engine-core`,
   forbid `game-stdlib` promotion, and prefer game-local functions over a generic
   trait.
3. Cross-crate boundary under audit: the extracted module's public API is the
   shared contract that `replay-check` (004), `trace-viewer` (012), and
   `seed-reducer` (010) call. The contract surface is the hash-set
   {state, effect, action-tree, public-view, diagnostic} + outcome/terminal —
   it must align with the canonical Trace Schema v1 surfaces fixed in
   GAT2TRAREPBEN-001.
4. FOUNDATIONS §2 (Rust owns replay/hash behavior) motivates this: restate that
   this is a *move* of existing Rust logic into a reusable location, not a transfer
   of authority and not a hash-semantics change.
5. §11 determinism enforcement surface: this module IS the deterministic replay/hash
   path. Confirm the extraction preserves byte-identical hashes — same stable
   serialization order, no incidental map/set iteration in hashing, no wall-clock
   or OS entropy. Because semantics are unchanged, no §13 ADR trigger fires; the
   existing golden traces are the regression guard.

## Architecture Check

1. A game-local module (not a generic trait, not `engine-core`) is exactly spec
   §D2's preferred shape; a narrow contract-only trait is deferred until duplication
   across a second game proves it earned (and only via the kernel-change protocol).
2. No backwards-compatibility shims — the test helpers move; they are not aliased.
3. `engine-core` stays free of `race_to_n` nouns (counter/target/seat/race); the
   extraction lands wholly in `games/race_to_n`. `game-stdlib` untouched.

## Verification Layers

1. Hashes unchanged after extraction → deterministic replay-hash check: existing
   golden traces reproduce identical state/effect/action-tree/public-view hashes.
2. Tests and tools call one path → codebase grep-proof: `replay_tests.rs` imports
   `replay_support`; later `replay-check` imports the same symbols.
3. No kernel leakage → grep-proof: `crates/engine-core` gains no `race_to_n`
   mechanic noun (boundary-check stays green).

## What to Change

### 1. New `games/race_to_n/src/replay_support.rs`

Move the canonical replay evaluation into public game-local functions that, given
seed/options/command-stream, produce the {state, effect, action-tree, public-view,
diagnostic} hashes plus outcome/terminal status. Keep bot-trace verification
game-local.

### 2. `games/race_to_n/src/lib.rs`

Declare the module and export the narrow replay-support API.

### 3. `games/race_to_n/tests/replay_tests.rs`

Replace the inline helpers with calls into `replay_support`. Retain `parse_fixture`
for the legacy key-value format only until GAT2TRAREPBEN-003 migrates traces to
JSON (mark it temporary).

## Files to Touch

- `games/race_to_n/src/replay_support.rs` (new)
- `games/race_to_n/src/lib.rs` (modify) — declare + export the module
- `games/race_to_n/tests/replay_tests.rs` (modify) — call shared support

## Out of Scope

- Trace JSON migration (GAT2TRAREPBEN-003) — `parse_fixture` stays as a temporary legacy path here.
- `replay-check` tool implementation (GAT2TRAREPBEN-004).
- Any generic replay trait or `engine-core`/`game-stdlib` change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n --test replay_tests` — passes; hashes identical to pre-extraction.
2. `cargo test --workspace` — passes; no regression elsewhere.
3. `bash scripts/boundary-check.sh` — passes; `engine-core` stays noun-free.

### Invariants

1. Identical inputs+versions reproduce identical hashes through `replay_support` (FOUNDATIONS §2/§11).
2. `replay_support` is game-local; no `race_to_n` noun enters `engine-core` (§3).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/replay_tests.rs` — rewired to call `replay_support`; existing assertions unchanged.
2. `games/race_to_n/src/replay_support.rs` — optional in-module unit tests for the public evaluation fns.

### Commands

1. `cargo test -p race_to_n --test replay_tests`
2. `cargo test --workspace`
3. `bash scripts/boundary-check.sh` — narrower boundary proof that extraction did not leak nouns into the kernel.
