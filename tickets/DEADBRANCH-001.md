# DEADBRANCH-001: engine-core ActionTree dead-branch detection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `engine-core` (`crates/engine-core/src/action.rs`): new generic well-formedness API on `ActionTree`/`ActionNode`. No behavior change to existing types' fields or serialization.
**Deps**: none

## Problem

The `ActionTree` contract (engine-core) allows a non-leaf `ActionChoice` — one whose
`next` is `Some(ActionNode { .. })` — to carry an **empty** `choices` vector, or a
subtree that contains no reachable leaf. This "dead branch" is a malformed action
tree: there is no completable action path beneath the choice, yet the choice is
present and may be marked `ActionPreview::Available`.

Two consumers disagree on what a dead branch means, which is how the live bug in
Event Frontier escaped detection:

- The shared random-legal bot (`crates/ai-core/src/random_legal.rs` `collect_choice`,
  lines 47-54) recurses into the empty `next` node and pushes **zero** paths, so the
  bot silently never selects the branch.
- The web UI (`apps/web/src/components/ActionPathBuilder.tsx:39`,
  `canConfirm = Boolean(leaf && !leaf.next?.choices?.length)`) treats the empty-`next`
  choice as a confirmable leaf and submits the bare parent segment, which Rust then
  rejects as `malformed_action`.

There is currently no generic way to assert an action tree is free of dead
branches. This ticket adds that primitive in `engine-core` (which owns the `action
tree` / `action path` contract nouns per FOUNDATIONS §3) so that game rule tests and
the simulation sweep (DEADBRANCH-003) can enforce well-formedness uniformly. This is
pure detection with no behavior change; the Event Frontier fix is DEADBRANCH-002.

## Assumption Reassessment (2026-06-13)

1. `crates/engine-core/src/action.rs` defines `ActionTree { root: ActionNode, freshness_token }`,
   `ActionNode { choices: Vec<ActionChoice> }`, and `ActionChoice { segment, label,
   accessibility_label, metadata, tags, preview, next: Option<Box<ActionNode>> }`
   (verified: lines 12-65). A "leaf" is `next == None`; a non-leaf is `next == Some(_)`.
   There is no existing well-formedness method on these types.
2. `docs/FOUNDATIONS.md` §3 explicitly lists `action tree` and `action path` among the
   generic nouns `engine-core` MAY know (verified: lines 53-55). A dead-branch
   well-formedness rule is a generic property of the action-tree contract and
   introduces no mechanic/domain nouns, so it belongs in `engine-core`.
3. Cross-crate boundary under audit: the `ActionTree` schema shared by `games/*`
   (producers), `crates/ai-core/src/random_legal.rs` `legal_paths`/`collect_paths`
   (consumer), `crates/wasm-api` (serializer to the browser), and
   `apps/web/.../ActionPathBuilder.tsx` (consumer). This ticket only adds a read-only
   inspection method; it does not alter the schema's fields or wire format.
6. Schema extension classification: this adds **methods only** to `ActionTree`/
   `ActionNode`; it adds no fields and changes no serialized representation, so it is
   additive-only and breaks no consumer (`ai-core`, `wasm-api`, web).

## Architecture Check

1. The well-formedness rule is a property of the action-tree *contract*, not of any
   one game or of the bot. Placing it in `engine-core` next to the type keeps a single
   authoritative definition that game tests and tooling share, rather than each game
   or the simulate tool re-deriving "is this choice a dead end?" The alternative —
   detecting in `ai-core::legal_paths` — is wrong because `legal_paths` deliberately
   *flattens* the tree and would have to change its return contract to surface the
   divergence; detection belongs at the contract level.
2. No backwards-compatibility aliasing/shims: this is net-new API.
3. `engine-core` stays free of mechanic nouns — `dead_branch_paths` reasons only over
   generic `ActionNode`/`ActionChoice`/`segment` structure (§3). No `game-stdlib`
   change, so the mechanic-atlas earned-promotion rule (§4) is not engaged.

## Verification Layers

1. Dead-branch detection correctness (empty `next`, recursively-dead `next`, healthy
   leaf, healthy nested) -> unit tests in `crates/engine-core/src/action.rs`.
2. Generic-contract / no-mechanic-noun invariant (§3) -> codebase grep-proof that the
   new code references only `ActionNode`/`ActionChoice`/`ActionPath` symbols and
   `scripts/boundary-check.sh` still passes.
3. Additive-only schema invariant -> `cargo build --workspace` + `cargo test --workspace`
   confirm no `ai-core`/`wasm-api` consumer signature breaks.

## What to Change

### 1. Add dead-branch inspection to `engine-core`

In `crates/engine-core/src/action.rs`, add (public) methods:

- `ActionNode::dead_branch_paths(&self) -> Vec<Vec<String>>` — returns, in
  deterministic depth-first order, the segment path (accumulated `segment`s from the
  root of the walk) of every choice that has `next == Some(node)` but whose subtree
  yields **no** reachable leaf. A choice is a dead branch when its `next` node is empty
  **or** every child beneath it is itself a dead branch (recursive). Leaf choices
  (`next == None`) are never dead.
- `ActionTree::dead_branch_paths(&self) -> Vec<Vec<String>>` — delegates to
  `self.root.dead_branch_paths()`.
- `ActionTree::has_dead_branches(&self) -> bool` — convenience wrapper returning
  `!self.dead_branch_paths().is_empty()`.

Detection must be deterministic (depth-first, preserving `choices` order) so callers
may assert on stable output. Do not mutate or prune the tree — detection only.

## Files to Touch

- `crates/engine-core/src/action.rs` (modify — add methods + unit tests)

## Out of Scope

- Fixing the Event Frontier action tree (DEADBRANCH-002).
- Wiring the assertion into the simulation sweep (DEADBRANCH-003).
- Any change to `ai-core::legal_paths`, `wasm-api`, or the web UI.
- Pruning/normalizing trees at runtime — this ticket is detection only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core action::` — new unit tests for `dead_branch_paths`.
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace` (no consumer regressions) and `bash scripts/boundary-check.sh`.

### Invariants

1. A choice with `next == None` is never reported as a dead branch.
2. A choice with `next == Some(node)` is reported iff its subtree yields no leaf
   (empty node, or all descendants dead), and detection is deterministic.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/action.rs` (tests module) — `dead_branch_paths`:
   - flat all-leaf tree -> no dead branches;
   - non-leaf choice with non-empty leaf children -> no dead branches;
   - non-leaf choice with `next = Some(ActionNode { choices: vec![] })` -> reports that
     choice's segment path;
   - nested non-leaf whose every grandchild is itself an empty-`next` dead branch ->
     reports the top dead branch (recursive propagation);
   - ordering is deterministic across repeated calls.

### Commands

1. `cargo test -p engine-core action::`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. Narrow `-p engine-core` command isolates the new primitive; the workspace run
   proves no consumer of the shared `ActionTree` type regressed.
