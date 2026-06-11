# GAT11MASCLABLU-005: Claim-phase action tree and validation

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/src/{actions.rs,rules.rs}`, extends `src/lib.rs`
**Deps**: GAT11MASCLABLU-004

## Problem

The claimant needs a Rust-owned legal action tree (one choice per held tile × declared grade) and fail-closed validation, with the claim path carrying the tile ID internally while every public surface shows only the declared grade. The responder (and any non-claimant) gets an empty waiting tree. TypeScript must never compute this legality.

## Assumption Reassessment (2026-06-10)

1. `src/state.rs`/`src/setup.rs` from GAT11MASCLABLU-004 provide `Phase::Claim`, the claimant, and hands. The action-tree / `ActionPath` model is the generic `engine_core` contract; `games/plain_tricks/src/actions.rs` is the local shape model.
2. Spec §"Legal action tree" (claim phase): `claim/<tile-id>/<grade>` for every held tile and grade `1..=5`; public summary `claim/grade-<g>`; claimant-only tree; responder/non-claimant flat empty tree with safe waiting metadata. Spec §Validation enumerates the rejects.
3. Cross-artifact boundary under audit: the action-tree and command-envelope contract (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`). The internal `claim/<tile-id>/<grade>` path is private; the public command summary redacts the tile ID to the declared grade.
4. FOUNDATIONS §2 (legality is Rust-owned; TypeScript presentation-only) is the principle under audit — the legal tree, previews, and diagnostics all originate in Rust.
5. §11 no-leak firewall enforcement surface: action-tree choice metadata and command summaries. Confirm claim-choice metadata may include the tile's own grade/label ONLY in the claimant's own seat view (never opponent/pedestal data), and the tile ID never appears in any public summary, preview, or diagnostic.

## Architecture Check

1. An actor-specific legal tree gives legality-by-construction (the responder simply has no claim actions) rather than validation patches over a shared action space — the pattern the spec credits to `secret_draft`.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free (`claim`/`grade`/`tile` nouns are game-local); no `game-stdlib` claim helper.

## Verification Layers

1. Claim legality (every held tile × grade `1..=5`) -> rule test (full suite in GAT11MASCLABLU-010).
2. Validation fail-closed -> rule test asserting rejects: stale freshness token, wrong/unseated actor, terminal phase, claim by non-claimant, unowned tile, out-of-range grade.
3. Diagnostics viewer-safe -> diagnostics test that messages name no hidden tile identity.
4. Claim-path no-leak -> no-leak visibility test that the public command summary and choice metadata carry only the declared grade, not the tile ID.

## What to Change

### 1. `src/actions.rs` (claim phase)

Claimant tree: `claim/<tile-id>/<grade>` per held tile and grade `1..=5`; choice metadata includes the tile's own grade/label only in the claimant's seat view plus a public score preview for the declared grade. Responder / non-claimant: flat empty tree with safe waiting metadata naming who acts next and why.

### 2. `src/rules.rs` (validation)

Reject: stale freshness token; actor not seated; terminal phase; claim outside `Phase::Claim` or by the non-claimant; claim of a tile not in the actor's hand; out-of-range grade; malformed/extra path segments. Diagnostics are viewer-safe ("it is not your turn to respond", "that mask is not in your hand") and never reference hidden tile identities.

## Files to Touch

- `games/masked_claims/src/actions.rs` (new)
- `games/masked_claims/src/rules.rs` (new)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Reaction-window action tree and window-open effects (GAT11MASCLABLU-006).
- Applying a claim / conditional resolution / scoring (GAT11MASCLABLU-006/007).
- Public/seat view projection (GAT11MASCLABLU-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` claim-legality and diagnostics tests pass.
2. Validation rejects every illegal claim shape with a viewer-safe message.
3. No public summary, preview, or diagnostic exposes a tile ID.

### Invariants

1. TypeScript decides no legality; the claim tree and previews originate in Rust (FOUNDATIONS §2).
2. The claim action path's tile ID never reaches a public surface (FOUNDATIONS §11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/rules.rs` `#[cfg(test)]` — claim legality + each rejection branch.
2. `games/masked_claims/src/actions.rs` `#[cfg(test)]` — claimant tree shape; responder empty waiting tree.

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. Unit-level rule tests are the correct boundary; end-to-end no-leak sweeps over all surfaces land in GAT11MASCLABLU-010.

## Outcome

Completed: 2026-06-11

What changed:

- Added `games/masked_claims/src/actions.rs` with claimant-only `claim/<tile-id>/<grade>` action trees, actor-seat mapping, claim parsing, fail-closed validation, viewer-safe diagnostics, and public command-summary redaction to `claim/grade-<g>`.
- Added `games/masked_claims/src/rules.rs` as a narrow placeholder seam for the validated claim output; application/resolution remains out of scope for later tickets.
- Updated `games/masked_claims/src/lib.rs` exports for actions and rules.

Deviations from original plan:

- None for behavior. The claim path keeps tile IDs only in the actor-submitted internal action path; public metadata and summaries are redacted to grade.

Verification:

- `cargo test -p masked_claims` passed.
- `cargo clippy -p masked_claims --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
- New tests cover claimant tree shape, responder/unseated empty trees, validation reject branches, viewer-safe diagnostics, and absence of tile IDs from public metadata/summary surfaces.
