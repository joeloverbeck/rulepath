# DEADBRANCH-002: Event Frontier must not offer an operation with no legal target

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `games/event_frontier` (`src/actions.rs`): legal action tree construction. No `engine-core`/`game-stdlib` change.
**Deps**: DEADBRANCH-001

## Problem

In Event Frontier, the legal action tree offers an "Operation" / "Limited operation"
root choice based purely on the card phase, regardless of whether any operation has a
legal target. When a faction is allowed to choose an operation in the current phase
but **no** operation kind has a legal target (common for a *limited* operation, which
is bounded to one site), the tree still presents the operation root — but its child
node is empty.

Observed bug (reported from live play): Charter, at the second choice after
Freeholders chose Operation, was offered `Event`, `Limited operation`, `Pass`.
Selecting "Limited operation" showed only a `Confirm` button with no operation to
choose, and confirming produced `malformed_action` ("that Event Frontier action path
is malformed").

Root cause: `operation_root_choice` (`games/event_frontier/src/actions.rs:378-412`)
**unconditionally** sets the root choice's `next` to `operation_kind_choices(...)`.
`operation_kind_choices` (`:414-445`) drops each kind whose `operation_leaf_choices`
is empty (`if leaf_choices.is_empty() { return None }`). When all of a faction's kinds
have no legal target, it returns an empty `Vec`, so the operation root becomes a
non-leaf with an empty `next` node — a dead branch (see DEADBRANCH-001). The UI treats
the empty-`next` choice as a confirmable leaf and submits the bare `limited_operation`
segment, which `parse_action_path` (`:216-242`) rejects → `malformed_action`.

Per FOUNDATIONS §2, Rust owns legal action generation; per §7, normal public UI must
be legal-only (illegal/incompletable options must be absent or inert). The fix: omit
the operation root from the legal action tree when it has no descendable children.

## Assumption Reassessment (2026-06-13)

1. `games/event_frontier/src/actions.rs`: `legal_action_tree` (`:120-136`) maps each
   `MenuEntry` from `choosing_menu` through `menu_choice` (`:356-376`), which for
   `MenuEntry::Operation { limited }` calls `operation_root_choice` (`:378-412`).
   `operation_root_choice` sets `choice.next = Some(Box::new(ActionNode { choices:
   operation_kind_choices(state, faction, limited) }))`. `operation_kind_choices`
   (`:414-445`) returns an empty `Vec` when every kind's `operation_leaf_choices`
   (`:447-492`) is empty. Verified by reading the file in full.
2. `choosing_menu`/`second_choice_menu` (`:181-214`) derive the menu from `CardPhase`
   only; for `FirstChoice::Operation` the second-choice menu is `[Event,
   Operation{limited:true}, Pass]` (`:203-207`), matching the reported scenario.
3. Shared boundary under audit: the `ActionTree` action-path schema. `validate_command`
   (`:244-278`) parses `command.action_path.segments` via `parse_action_path`; a bare
   `["limited_operation"]` already correctly yields `malformed_action` (verified
   `:216-242`). This ticket changes only **tree construction**, not parsing or
   validation — the parser stays the fail-closed backstop.
4. FOUNDATIONS principle under audit: §2 (Rust owns legal action generation) and §7
   (public UI is legal-only; illegal moves absent/inert; "TypeScript MUST NOT invent
   legality"). A tree that offers an incompletable operation root violates §7's intent
   by surfacing a non-actionable option. The correct repair is in Rust tree
   construction, not the UI.
6. Schema extension classification: no schema change. The set of *possible* choice
   segments is unchanged; this only removes a choice instance from the tree when it has
   no reachable leaf. Additive/subtractive-only on a per-state basis; the wire format
   and `ActionChoice` shape are untouched.

## Architecture Check

1. Filtering the operation root at tree-construction time (in `legal_action_tree` /
   `menu_choice`) is cleaner than the alternatives: (a) changing the parser to accept a
   bare operation segment would invent a meaningless no-target operation and contradict
   the rules; (b) guarding in the UI would put legality in TypeScript (§2 violation).
   The menu-entry → choice mapping is the single place that already knows the subtree,
   so the guard is local and total.
2. No backwards-compatibility aliasing/shims: the dead root is removed, not aliased.
3. `engine-core` untouched (no mechanic nouns added); no `game-stdlib` change, so the
   mechanic-atlas earned-promotion rule (§4) is not engaged. The guard is game-local in
   `games/event_frontier`.

## Verification Layers

1. No-dead-branch invariant for Event Frontier trees -> rule test asserting
   `legal_action_tree(state, actor).dead_branch_paths().is_empty()` (helper from
   DEADBRANCH-001) for the reproduced no-legal-operation state.
2. Correct menu after the fix (operation root absent; Event + Pass still present) ->
   rule test inspecting `tree.root.choices` segments.
3. Fail-closed parser backstop unchanged -> test that a bare `["limited_operation"]`
   path still returns the `malformed_action` diagnostic via `validate_command`.
4. End-to-end determinism unaffected -> `cargo run -p simulate -- --game event_frontier
   --games 1000` and `cargo run -p replay-check -- --game event_frontier --all` still pass.

## What to Change

### 1. Omit an operation root choice with no descendable children

In `games/event_frontier/src/actions.rs`, change tree construction so a
`MenuEntry::Operation { limited }` produces a choice **only** when
`operation_kind_choices(state, faction, limited)` is non-empty. Concretely, build the
operation root in `legal_action_tree` (or `menu_choice`) and drop it when its `next`
node is empty (`choice.next.as_ref().is_some_and(|node| node.choices.is_empty())`), so
the resulting `tree.root.choices` never contains an operation root with an empty
subtree. Apply to both `limited` and non-limited operation roots (the same dead-branch
shape is possible for either). Event and Pass choices are unaffected.

Note: `validate_command`/`validate_menu_allows` (`:835-849`) still accept
`MenuEntry::Operation` as phase-legal; that is correct and unchanged — a faction with
no legal operation simply has no operation *choice* in its tree, while the parser
remains the fail-closed backstop for any malformed submission.

## Files to Touch

- `games/event_frontier/src/actions.rs` (modify — tree construction + tests)

## Out of Scope

- The generic detection primitive (DEADBRANCH-001) and the all-games sweep
  (DEADBRANCH-003).
- Any change to `parse_action_path`, `validate_command`, or the diagnostics.
- UI changes in `apps/web` (covered separately if pursued; this ticket fixes the Rust
  authority so the UI never receives the dead branch).
- Operation cost/edict logic.

## Acceptance Criteria

### Tests That Must Pass

1. New rule test (TDD: written failing first) reproducing the reported state — Charter
   in `CardPhase::AwaitingSecondChoice { first_choice: FirstChoice::Operation, .. }`
   with a board where no Charter operation (Survey/Fortify/Writ) has a legal target —
   asserts `legal_action_tree` offers `event` and `pass` but **no**
   `operation`/`limited_operation` root, and `tree.dead_branch_paths().is_empty()`.
2. Regression test: a bare `["limited_operation"]` action path still yields the
   `malformed_action` diagnostic from `validate_command`.
3. `cargo test -p event_frontier`, `cargo run -p simulate -- --game event_frontier
   --games 1000`, `cargo run -p replay-check -- --game event_frontier --all`,
   `cargo run -p rule-coverage -- --game event_frontier`,
   `cargo run -p fixture-check -- --game event_frontier`.

### Invariants

1. For every reachable Event Frontier state, `legal_action_tree(state, actor)` contains
   no dead branch (no operation root with an empty subtree).
2. A faction allowed an operation by phase but with no legal target is offered only the
   remaining legal menu entries (Event and/or Pass), never an incompletable operation.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/src/actions.rs` (tests) or `games/event_frontier/tests/` —
   `limited_operation_with_no_legal_target_is_not_offered` (TDD anchor; written first,
   must fail before the fix): builds the no-legal-operation second-choice state and
   asserts the operation root is absent and `dead_branch_paths` is empty.
2. Same suite — `bare_operation_path_still_rejected`: confirms the parser backstop is
   intact.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p simulate -- --game event_frontier --games 1000 && cargo run -p replay-check -- --game event_frontier --all && cargo run -p rule-coverage -- --game event_frontier && cargo run -p fixture-check -- --game event_frontier`
3. The per-game gate set is the correct verification boundary because the change is
   confined to Event Frontier's legal-action generation; the workspace build/clippy run
   in DEADBRANCH-001/003 covers cross-crate effects.
