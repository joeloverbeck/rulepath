# GAT91SECDRACOM-004: secret_draft legal action tree + validation + viewer-safe diagnostics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft/src/actions.rs` (and validation helpers); no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-003

## Problem

Both seats must be able to choose from the shared visible pool until each has committed, with Rust owning all legality, preview metadata, and diagnostics. The action tree must be actor-specific (an already-committed seat gets no choices and safe pending metadata), and diagnostics must be viewer-safe (never "opponent already chose that item"). This is the legal-action API humans and bots both call.

## Assumption Reassessment (2026-06-08)

1. The state types from GAT91SECDRACOM-003 (`visible_pool: Vec<DraftItemId>`, `commitments: [Option<DraftItemId>; 2]`, `phase`, `freshness_token`) are the inputs. `games/high_card_duel/src/actions.rs` is the behavioral precedent for a commit-style legal tree; `games/token_bazaar/src/actions.rs` is the structural precedent.
2. Spec §"Legal action tree" + §Validation define the contract: for an actor mapped to a seat — terminal → empty tree; already-committed this round → empty tree with safe pending/waiting metadata; otherwise → flat choices for every item currently in `visible_pool`. Validation rejects: stale freshness token, actor not seated, terminal phase, actor already committed, malformed segment, item not currently visible, extra action-path segments. Returns `ValidatedAction { actor, item }`.
3. Cross-artifact boundary under audit: the action-tree / command-envelope contract (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`) and the diagnostic contract. Choice metadata may include public item ID/value/thread/label/current public score preview and a safe "another seat may already be pending" warning; it MUST NOT include opponent hidden choices, hidden predictions, hidden bot candidates, or TS-computed consequences.
4. §2 behavior authority is the motivating principle: restate before trusting spec — Rust owns legal action generation, validation, and all preview metadata; TypeScript decides no legality. A clickable-illegal-move path would be a §12 stop condition.
5. §11 no-leak firewall substrate: diagnostics and previews are a leak surface. "item is unavailable for this commitment" is the required wording, never "opponent already chose that item." This ticket establishes viewer-safe diagnostics; the negative no-leak tests land in GAT91SECDRACOM-009.

## Architecture Check

1. An actor-specific flat tree (choices only for uncommitted seats; pending metadata otherwise) is cleaner than a single shared tree filtered in the UI — it keeps legality decisions in Rust and makes "both seats eligible until committed" structural.
2. No backwards-compatibility aliasing/shims — fills GAT91SECDRACOM-002 stubs.
3. `engine-core` stays noun-free; action-path/command-envelope generics are reused, draft nouns stay game-local. No `game-stdlib` helper.

## Verification Layers

1. Legal-tree correctness -> unit tests: uncommitted seat gets full visible-pool choices; committed seat gets empty tree + pending metadata; terminal gets empty tree.
2. Validation fail-closed -> unit tests for each rejection (stale token, wrong actor, terminal, already-committed, malformed/extra segment, item not in pool).
3. Diagnostic no-leak -> grep/manual review that unavailable-item diagnostic text carries no opponent-choice information (negative test in GAT91SECDRACOM-009).
4. Schema conformance -> action-tree/command-envelope shape validated against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.

## What to Change

### 1. `src/actions.rs` — legal action tree

Build the actor-specific tree per the spec: empty (terminal), empty + pending metadata (already committed), flat per-visible-pool-item choices (uncommitted). Attach viewer-safe preview metadata (item ID/value/thread/label, public score preview, safe pending warning).

### 2. `src/actions.rs` — validation

Implement validation rejecting all listed cases and returning `ValidatedAction { actor, item }`. Diagnostics are game-local and viewer-safe; use "item is unavailable for this commitment" framing.

## Files to Touch

- `games/secret_draft/src/actions.rs` (modify)

## Out of Scope

- Applying the validated action / emitting effects / reveal resolution (GAT91SECDRACOM-005).
- View projection (GAT91SECDRACOM-006) and the full no-leak negative test suite (GAT91SECDRACOM-009).
- Bot use of the action tree (GAT91SECDRACOM-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` — legal-tree tests (uncommitted/committed/terminal) pass.
2. Validation tests for every rejection case pass (fail-closed, blocking).
3. Diagnostic text for an unavailable item contains no opponent-choice information.

### Invariants

1. Rust owns all legality and preview metadata; no TS legality path (§2).
2. Validation is deterministic, fail-closed, and blocking; diagnostics are viewer-safe (§11).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/actions.rs` inline unit tests — legal-tree per phase/commit-state; per-case validation rejections; diagnostic safety.

### Commands

1. `cargo test -p secret_draft actions`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. A targeted `actions` test filter is the correct boundary; end-to-end legality is exercised later via simulation (GAT91SECDRACOM-012 registration) and golden traces (GAT91SECDRACOM-010).
