# GAT17VOWTIDOHHEL-003: Back-port `plain_tricks` to the promoted helper

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `games/plain_tricks/src/{rules,actions}.rs`, `games/plain_tricks/Cargo.toml`, `games/plain_tricks/docs/{PRIMITIVE-PRESSURE-LEDGER,MECHANICS}.md`
**Deps**: 002

## Problem

FOUNDATIONS §4/§11 require that a promoted `game-stdlib` primitive be adopted by every matching prior official game in-gate, with no promotion debt. `plain_tricks` is the first close trick-taking use. This ticket migrates its local "matching led suit or all" selection and pure winner-index computation to `game-stdlib::trick_taking` (trumpless), preserving all observable behavior.

## Assumption Reassessment (2026-06-21)

1. `games/plain_tricks/src/actions.rs` computes follow-suit legality in `legal_cards()` (filter led suit else all, ~`:59`/`:75`) and `must_follow_suit()` (`:198`); `games/plain_tricks/src/rules.rs` computes the winner in `trick_winner(leader_play, follower_play)` (`:64`, no trump). These are the exact local sites to replace.
2. `games/plain_tricks/Cargo.toml` currently depends only on `ai-core`, `engine-core` (no `game-stdlib`) — the dependency must be added.
3. Cross-crate boundary under audit: the helper's stable-index output must map back to the game's `TrickCardId`/seat order without reordering action leaves or effect output; golden traces and state/action-tree/effect hashes are the preservation contract.
4. FOUNDATIONS §11 (promoted-primitive adoption + deterministic replay/hash) is the principle under audit: this is a behavior-preserving conformance change, not a rules change — change rationale is the §4 in-gate adoption mandate.
5. Replay/hash enforcement surface: pre-change command/action/effect/view/hash/trace baselines must be captured and compared byte-for-byte; any observable change halts for reassessment (no trace refresh as a shortcut). No hidden-info path changes — the helper sees only projected suit/rank keys.

## Architecture Check

1. Routing the two pure computations through the shared helper removes a duplicate implementation while keeping all local types, diagnostics, legal-action construction, phase checks, effects, and scoring untouched — the cleanest possible conformance.
2. No shims/aliases; the local helper functions are replaced, not wrapped.
3. `engine-core` untouched; the adopted `game-stdlib` helper is the atlas-earned promotion from 002.

## Verification Layers

1. Follow-suit + winner behavior unchanged → `cargo test -p plain_tricks` (existing rules/property suites).
2. Deterministic replay/hash preserved → `cargo run -p replay-check -- --game plain_tricks --all` (byte-identical traces/hashes).
3. Serialization stable → `cargo test -p plain_tricks --test serialization`.
4. Adoption recorded → grep `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` for the helper-conformance receipt.

## What to Change

### 1. Adopt the helper in `actions.rs` / `rules.rs`

Add `game-stdlib` to `Cargo.toml`. Replace the local led-suit filter with `follow_suit_indices` (projecting `TrickCardId::suit`) and the local `trick_winner` rank comparison with `winning_play_index` (`trump = None`), mapping returned indices back to the local seat/card. Keep all diagnostics, ordering, effects, and scoring.

### 2. Ledger + mechanics receipt

Record the helper-conformance receipt and preserved-evidence note in `PRIMITIVE-PRESSURE-LEDGER.md` and `MECHANICS.md`.

## Files to Touch

- `games/plain_tricks/src/actions.rs` (modify)
- `games/plain_tricks/src/rules.rs` (modify)
- `games/plain_tricks/Cargo.toml` (modify)
- `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)
- `games/plain_tricks/docs/MECHANICS.md` (modify)

## Out of Scope

- Any change to Plain Tricks rules, action order, diagnostics, effects, visibility, bots, or UI beyond the behavior-preserving swap.
- `briar_circuit` back-port (004); Vow Tide code.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` — all pre-existing tests unchanged-green.
2. `cargo run -p replay-check -- --game plain_tricks --all` — traces/hashes byte-identical.
3. `cargo bench -p plain_tricks` — no material regression vs the 002 baseline.

### Invariants

1. No observable behavior, hash, trace, action order, or effect output changes.
2. No promotion debt opened; the ledger records adoption.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/rules.rs` / `property.rs` — existing tests must pass unmodified; add a regression note only if a latent defect is proved (then follow the failing-test protocol).

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. Narrower command rationale: replay-check `--all` is the determinism boundary that proves the swap changed nothing; full-workspace test is deferred to the capstone.
