# GAT201STACROHOP-001: Forbid hop-chain return to the moving peg's origin space

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `games/starbridge_crossing` (`src/rules.rs` legality; `tests/rules.rs`, `tests/property.rs`)
**Deps**: None

## Problem

A Starbridge Crossing hop chain can land back on the moving peg's own origin
space (its space at turn start) — e.g. `origin → A → origin`. The jumped peg is
not moved (`SC-MOVE-003`), so a committed origin-return chain leaves board
occupancy identical before and after while the ply still advances: a net-zero
**no-op turn**, i.e. a back-door voluntary pass that contradicts `SC-TURN-002`
("a turn is exactly one move") and `SC-MOVE-009` (which must not create a
strategic pass option). The defect: `legal_jump_landings`
(`games/starbridge_crossing/src/rules.rs:81`) treats the vacated origin as an
empty, legal landing (`occupancy_during_chain` returns `None` for the origin at
`rules.rs:343-344`), so the origin is never excluded from the chain.

## Assumption Reassessment (2026-06-27)

1. `legal_jump_landings` (`src/rules.rs:81`) computes `origin` from `state.pegs`
   (`rules.rs:87-91`); state is **not** mutated mid-chain during enumeration or
   validation, so `origin` is stable across the whole chain. Both the
   action-tree enumeration (`actions.rs::jump_landing_choices`, `actions.rs:289`
   → `legal_jump_landings`, `actions.rs:295`) and validation
   (`validate_jump_command`, `rules.rs:121`, which calls `legal_jump_landings`
   at `rules.rs:148`) funnel through this single function, so one internal guard
   `if landing == origin { continue; }` (the function already has `origin` in
   scope) closes every chain-construction path. A first hop from `origin` lands
   two spaces away and can never equal `origin`, so the `&[]` blocked-detection
   calls (`actions.rs:46`, `rules.rs:78`) are unaffected.
2. Spec `specs/gate-20-1-starbridge-crossing-hop-chain-origin-return-prohibition.md`
   §4 D1 and §5 STACROSORIG-002 require exactly this single-point guard (the
   "seed the visited set" and `actions.rs::add_jump_choices` alternatives were
   removed during the in-session `/reassess-spec` pass). `occupancy_during_chain`
   (`rules.rs:334`) semantics for genuine mid-chain spaces are preserved — the
   guard short-circuits before the origin reaches the occupancy check.
3. Cross-artifact boundary under audit: the enumerated **action tree** (the
   `JumpLanding` set `legal_jump_landings` returns) is the shared surface
   consumed by both `actions.rs` (tree construction) and `rules.rs` (validation).
   Narrowing it at the single producer keeps the tree and the validator in lockstep
   by construction — no second site can drift.
4. FOUNDATIONS §2 (Rust owns legal-action generation and validation) and §11
   (a committed turn must change board occupancy unless Rust issues
   `pass_blocked`) motivate this fix. Returning the peg to its exact origin is the
   **sole** no-op vector: jumped pegs are not moved (`SC-MOVE-003`); a step always
   moves to an empty adjacent space (never a no-op); and every non-origin hop
   landing net-displaces the peg.
5. Deterministic replay/hash surface (§11/§13): removing the origin-return node
   changes the action tree and its dependent hashes / golden traces / replay
   fixtures. This ticket does **not** regenerate those artifacts — the governed,
   per-artifact ADR-0009 migration is owned by `GAT201STACROHOP-003`. The change
   is legality-only and touches no view projection, so it introduces no
   hidden-information path the no-leak firewall would have to undo.

## Architecture Check

1. A single internal `landing == origin` guard in `legal_jump_landings` is
   cleaner than seeding a forbidden set at all four call sites (including the
   `&[]` blocked-detection calls, which must stay unaffected): one chokepoint,
   `origin` already computed, no caller churn.
2. No backwards-compatibility shims or alias paths introduced; the guard is a
   direct narrowing of the existing enumeration.
3. `engine-core` is untouched — the fix is entirely inside
   `games/starbridge_crossing`; no mechanic noun enters the kernel and no
   `game-stdlib` change is made (§3/§4).

## Verification Layers

1. `validate_jump_command` rejects any chain landing on `origin` -> new
   `tests/rules.rs` unit test asserting the diagnostic error.
2. The enumerated action tree never offers an origin-return landing -> new
   `tests/rules.rs` assertion over `legal_jump_landings` / the jump action node.
3. No committed non-`pass_blocked` turn leaves occupancy unchanged ->
   `tests/property.rs` property assertion (A2).
4. Legitimate multi-hop behaviour (direction-change, stop-midway,
   `SC-MOVE-003..006`) is unchanged -> existing `tests/rules.rs` cases stay green.
5. Deterministic replay/hash migration is governed, not silent -> golden-trace /
   replay / bench regeneration deferred to `GAT201STACROHOP-003` (FOUNDATIONS
   §11/§13, ADR 0009).

## What to Change

### 1. Guard the origin landing in `legal_jump_landings`

In `games/starbridge_crossing/src/rules.rs`, inside `legal_jump_landings`, add an
explicit `if landing == origin { continue; }` guard in the per-direction loop
(before the landing is pushed), so the moving peg's origin space is never an
offered landing at any chain depth. Leave `occupancy_during_chain` and its
non-origin semantics unchanged.

### 2. Failing-first regression test

In `games/starbridge_crossing/tests/rules.rs`, add
`hop_chain_cannot_return_to_origin_space`: construct an `origin → A → origin`
chain and assert (a) `validate_jump_command` returns the invalid-jump diagnostic,
and (b) the origin is absent from the enumerated jump landings / action tree.
Confirm it fails on the pre-guard code, then passes with the guard.

### 3. No-op-turn property

In `games/starbridge_crossing/tests/property.rs`, add a property asserting that
every committed turn that is not a Rust-issued `pass_blocked` changes board
occupancy (A2's invariant).

## Files to Touch

- `games/starbridge_crossing/src/rules.rs` (modify)
- `games/starbridge_crossing/tests/rules.rs` (modify)
- `games/starbridge_crossing/tests/property.rs` (modify)

## Out of Scope

- Regenerating golden traces / replay fixtures / benchmark baselines, and
  refreshing `GAME-EVIDENCE.md` — owned by `GAT201STACROHOP-003`. Full
  `cargo test -p starbridge_crossing`, `replay-check --all`, and `fixture-check`
  may be **red** until `-003` lands (golden-trace assertions in
  `tests/serialization.rs` / `tests/bots.rs` go stale the moment the guard lands).
- `RULES.md` / `RULE-COVERAGE.md` edits (the new `SC-MOVE-010` rule) — owned by
  `GAT201STACROHOP-002`.
- Any new pass option, variant, or piece/seat count; any change to
  `occupancy_during_chain` semantics for non-origin spaces; any TypeScript
  legality; any other movement/finish/terminal/visibility/bot/UI behaviour.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test rules` — including the new
   `hop_chain_cannot_return_to_origin_space`.
2. `cargo test -p starbridge_crossing --test property` — including the new
   no-op-turn property.
3. `cargo fmt --all --check` and `cargo clippy -p starbridge_crossing
   --all-targets -- -D warnings` pass.

### Invariants

1. `legal_jump_landings` never returns a landing equal to the moving peg's
   `origin`, at any chain depth.
2. `occupancy_during_chain` behaviour for non-origin spaces is byte-for-byte
   unchanged (genuine mid-chain occupancy still blocks/permits as before).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/rules.rs` — `hop_chain_cannot_return_to_origin_space`:
   reproduces `origin → A → origin`, asserts `validate_jump_command` rejection
   and action-tree absence; run pre-guard to confirm RED, then green.
2. `games/starbridge_crossing/tests/property.rs` — committed non-`pass_blocked`
   turns change occupancy.

### Commands

1. `cargo test -p starbridge_crossing --test rules --test property`
2. `cargo clippy -p starbridge_crossing --all-targets -- -D warnings`
3. Full `cargo test -p starbridge_crossing` / `cargo run -p replay-check --
   --game starbridge_crossing --all` are intentionally **not** the gate here —
   their golden traces regenerate in `GAT201STACROHOP-003`, so the scoped
   `--test rules --test property` run is the correct verification boundary for
   this diff.
