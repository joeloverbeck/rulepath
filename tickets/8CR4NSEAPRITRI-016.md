# 8CR4NSEAPRITRI-016: Briar Circuit C-04 typed action-tree parity adapter

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit` (`src/actions.rs`); game-owned typed adapter, browser JSON authority unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Briar's browser action tree is hand-built in `crates/wasm-api/src/games/briar.rs::briar_action_tree_json` from `bots::legal_bot_actions`; there is no game-owned typed `ActionTree` over Briar's legal pass/play actions. Add a game-owned `legal_action_tree` adapter in `games/briar_circuit/src/actions.rs` and prove **exact path/order/label parity** with the current browser choices, keeping the current browser JSON the rendered authority and legality in Briar Rust (spec §3.7 Briar, §5.6). The parallel v1 bytes/hash are a separate diff (`-017`).

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/actions.rs` exists but has no `legal_action_tree`; `bots::legal_bot_actions` exists at `src/bots.rs`; the browser tree is built in `crates/wasm-api/src/games/briar.rs::{briar_action_tree_json, briar_action_choice_json, briar_action_next_json}`. `engine_core::ActionTree` exists. Confirmed during `/reassess-spec`.
2. Spec §3.7 classifies the typed adapter + parity as `migrate`; this diff adds only the typed adapter and parity test — it does not move legality into WASM or replace the browser JSON.
3. Cross-artifact: legality stays in Briar Rust; the browser JSON remains the rendered authority. The current browser tree path/order/label baseline comes from `-001`.
4. §2 behavior-authority motivates this ticket: the typed adapter derives from the same legal pass/play actions the bots use; no legality moves to TypeScript and the browser output stays authoritative until a separate decision.
5. Enforcement surface = path/order/label parity between the typed adapter and the current browser tree; the adapter adds no rendered surface and changes no WASM byte.

## Architecture Check

1. A game-owned typed `ActionTree` over existing legal actions is cleaner than leaving the only tree representation hand-built in WASM — it gives Briar a Rust-owned tree to later hash, with parity proven first.
2. No backwards-compatibility shim is introduced; the browser JSON is unchanged. Legality remains in Briar Rust (no move to WASM in this diff).
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Exact path/order/label parity vs current browser tree -> focused parity test comparing the typed adapter to `briar_action_tree_json` over pass select/unselect/confirm and legal play paths.
2. Legality unchanged / browser JSON authoritative -> schema/serialization validation (browser JSON byte-identical) + `replay-check --game briar_circuit --all`.
3. Adapter present, legality not moved -> codebase grep-proof (`legal_action_tree` in `actions.rs`; WASM still owns the rendered tree).

## What to Change

### 1. Add a game-owned `legal_action_tree` adapter

In `games/briar_circuit/src/actions.rs`, add a `legal_action_tree` adapter that builds a typed `engine_core::ActionTree` from the existing legal pass/play actions (the same source `legal_bot_actions` uses). Add a parity test proving exact path/order/label equality with the current `briar_action_tree_json` choices.

## Files to Touch

- `games/briar_circuit/src/actions.rs` (modify)

## Out of Scope

- Adding the parallel v1 bytes/hash surface (`-017`).
- Replacing the browser JSON, moving legality into WASM, or changing any rendered output.
- Pass-target/reveal/trick policy (game-local).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the typed-adapter-vs-browser parity test (pass select/unselect/confirm + legal play).
2. `cargo test -p wasm-api` and `cargo run -p replay-check -- --game briar_circuit --all` pass with browser JSON byte-identical to baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The typed adapter's path/order/labels exactly match the current browser tree; the browser JSON is unchanged.
2. Legality stays in Briar Rust; no legality moves to TypeScript/WASM.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/` — a parity test asserting the typed `legal_action_tree` equals the current browser choices over the enumerated paths.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo test -p wasm-api`
3. The per-game parity test is the correct boundary: it proves Rust-owned/typed parity before any hash surface is added in `-017`.
