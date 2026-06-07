# GAT72GAT8HIG-006: Legal action tree + validation + diagnostics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/actions.rs` and the legality/validation/diagnostic surface of `src/rules.rs`
**Deps**: GAT72GAT8HIG-004

## Problem

Gate 8 must generate actor-scoped legal action trees (commit-one-card-from-own-
hand) and validate actions Rust-side, returning public-safe or seat-private
diagnostics for wrong-seat, wrong-phase, invalid-private-card, and stale actions
— without leaking hidden card identities through action labels/paths or
diagnostics.

## Assumption Reassessment (2026-06-07)

1. Verified the action surface: `crates/engine-core/src/action.rs` defines the
   generic action-tree / action-path vocabulary; `legal_action_tree` is
   actor-scoped (sibling `games/draughts_lite::legal_action_tree`). The diagnostic
   envelope is generic in `engine-core`.
2. Verified against the spec: §4.2.2 fixes `HCD-ACT-001..008` (only active
   lead/reply seat may commit; commit names one card in the actor's own hand; no
   double-commit; observer/no-seat has no commit actions; terminal empty) and
   `HCD-DIAG-001..006` (wrong-seat/phase = public-safe; invalid private card =
   redacted to unauthorized; stale = no hidden leak; redacted public tokens in
   browser-visible logs/test-IDs).
3. Cross-artifact boundary under audit: the action-tree + diagnostic envelopes
   (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`). Action-tree
   labels for the authorized actor MAY include that actor's own card identity
   (`HCD-ACT-008`); public/observer/opponent views must not.
4. FOUNDATIONS principle under audit (§2 behavior authority + §11): legality is
   computed in Rust only; TypeScript never decides it. The action tree is private
   data scoped to the acting seat.
5. Enforcement surface named: the §11 no-leak firewall on action labels/paths
   and diagnostics. Confirm invalid-private-card diagnostics redact the card id
   for unauthorized viewers and may include it only in the acting seat's private
   diagnostic (`HCD-DIAG-003`); stale-action diagnostics carry no current hidden
   state (`HCD-DIAG-004`). Proven by golden traces (012) + no-leak suite (011).

## Architecture Check

1. Computing legality + diagnostics in one rules surface keeps the legal/illegal
   boundary single-sourced in Rust; the UI consumes the tree, never recomputes it.
2. No backwards-compatibility shims — new game-local logic.
3. `engine-core` action/diagnostic vocabulary reused as generic contracts; no
   mechanic noun added to the kernel; no `game-stdlib` change.

## Verification Layers

1. Actor-scoped legality -> bot legality check + unit test: only the active lead/reply seat has commit actions; observer/no-seat has none (`HCD-ACT-001/002/005`).
2. Action-label privacy -> no-leak visibility test: authorized actor's tree may name its own card; opponent/observer trees do not.
3. Diagnostic redaction -> golden trace check (012) + unit test: wrong-seat/phase public-safe; invalid-private-card redacted for unauthorized; stale carries no hidden state.
4. Validation is fail-closed -> FOUNDATIONS alignment check: illegal commits are rejected in Rust (§2/§11), not surfaced as clickable.

## What to Change

### 1. `actions.rs`

Actor-scoped legal action tree: commit-card-from-own-hand for the active seat in
the current phase; empty for observer/no-seat and terminal (`HCD-ACT-001..007`);
authorized-actor labels may carry own card identity (`HCD-ACT-008`).

### 2. `rules.rs` validation + diagnostics

Validate commits Rust-side; emit `HCD-DIAG-001..006` diagnostics with redacted
public tokens for unauthorized viewers and (where safe) seat-private detail.

## Files to Touch

- `games/high_card_duel/src/actions.rs` (modify — fill stub)
- `games/high_card_duel/src/rules.rs` (modify — legality/validation/diagnostics portion)

## Out of Scope

- Applying a validated action / reveal / score / refill (GAT72GAT8HIG-007).
- View projection (008) and WASM action-tree authorization (016).

## Acceptance Criteria

### Tests That Must Pass

1. `observer_has_no_private_commit_actions` — no-seat viewer has an empty commit tree.
2. `wrong_seat_diagnostic_public_safe`, `wrong_phase_diagnostic_public_safe` — public-safe diagnostics.
3. `invalid_private_card_diagnostic_redacted_for_unauthorized`, `stale_action_diagnostic_no_hidden_leak` — redaction holds.

### Invariants

1. Legality is decided only in Rust; the action tree is actor-private (§2/§11).
2. No diagnostic leaks opponent card identity before reveal (`HCD-DIAG-005`).

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/rules.rs` — legality + diagnostic-redaction cases (extends the file from 004).

### Commands

1. `cargo test -p high_card_duel --test rules`
2. `cargo test -p high_card_duel`
3. Rule-level tests are the correct boundary; end-to-end no-leak/diagnostic redaction across viewers is proven in 011/012.
