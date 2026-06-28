# GAT203STACROTER-001: Rust-owned Starbridge terminal outcome rationale

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/starbridge_crossing` (`src/visibility.rs`: new game-local `StarbridgeOutcomeRationaleView` + per-seat standing type, projected on `StarbridgePublicView` as `terminal_rationale`; `tests/visibility.rs`)
**Deps**: None

## Problem

A finished Starbridge Crossing match cannot present the Rust-authored terminal outcome explanation that `games/starbridge_crossing/docs/UI.md` ("Outcome / victory explanation") and `RULES.md` `SC-FINISH-004` / `SC-FINISH-006` require. The public view (`games/starbridge_crossing/src/visibility.rs::StarbridgePublicView`) projects only a flat `terminal: Option<String>` (`"complete"` / `"turn_limit:2000"`) plus `finish_ranks` / per-seat `finish_rank`. It carries no structured rationale (decisive cause, decisive rule IDs, seat-ring-ordered standings with the rank-1 winner flag, turn-limit progress facts). This ticket adds that rationale, Rust-owned, derived only from existing public terminal state — the foundation the wasm-api serialization (002) and web rendering (003) consume.

## Assumption Reassessment (2026-06-28)

1. `StarbridgePublicView` (`games/starbridge_crossing/src/visibility.rs:14`) currently exposes `finish_ranks: Vec<FinishRank>` (`:22`), `terminal: Option<String>` (`:23`), per-seat `finish_rank: Option<u8>` (`:50`), and is built by `project_view` (`:60`); `stable_summary` (`:118`) / `stable_bytes` (`:179`) encode `terminal` as a flat string. No `terminal_rationale` exists. Confirmed.
2. The spec `specs/gate-20-3-starbridge-crossing-terminal-outcome-explanation.md` §4 and Assumption A1/A2 fix the two causes to `TerminalStatus::Complete` → `finish_order_complete` and `TerminalStatus::TurnLimit { max_plies }` → `turn_limit_progress_vector` (`src/state.rs:50`), with the turn-limit per-seat count being `rules.rs::progress_score` (`:452`, used by `assign_turn_limit_ranks` `:432`). Confirmed.
3. Cross-artifact boundary under audit: the per-game outcome-rationale pattern of `games/river_ledger/src/visibility.rs::OutcomeRationaleView` (`:193`, with `terminal_rationale: Option<OutcomeRationaleView>` at `:39`). The new type mirrors that shape but stays game-local in `games/starbridge_crossing`; no `engine-core` / `game-stdlib` noun is introduced.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: the decisive cause, rule IDs, winner, and terminal standings are Rust-owned public facts (`SC-FINISH-004` "TypeScript renders Rust-authored standings only"). The rationale is computed here so TS never derives it.
5. Deterministic replay/hash & serialization surface: `StarbridgePublicView::stable_summary` / `stable_bytes`. The rationale MUST NOT enter `stable_bytes` (it is a projection of already-public terminal state, not new state), so replay hashes, golden traces, setup fixtures, and benchmark thresholds are byte-unchanged. No-leak firewall (§11): Starbridge is fully public; the rationale adds only public facts already in `finish_ranks` / progress scoring — no hidden information.
6. Schema extension: `StarbridgePublicView` gains an additive `terminal_rationale: Option<StarbridgeOutcomeRationaleView>` (default `None` while live). Consumer is the wasm-api projection (002) and, transitively, `apps/web/src/wasm/client.ts::StarbridgeCrossingPublicView.terminal_rationale?` which already declares the optional slot (`client.ts:1617`). Additive-only.

## Architecture Check

1. Deriving the rationale inside `project_view` from existing public state (`terminal_status`, `finish_ranks`, the seat ring, `progress_score` accounting) keeps a single Rust source of truth and avoids a parallel TS computation that §2 forbids. Mirroring the river per-game shape (rather than inventing a new one) keeps the wasm/web adapters uniform.
2. No backwards-compatibility shim: `terminal_rationale` is a new optional field, not an alias over `terminal`; the flat `terminal` string is retained unchanged for existing consumers.
3. `engine-core` stays free of mechanic nouns — the rationale type lives in `games/starbridge_crossing`; no outcome/standings/topology noun enters the kernel or `game-stdlib` (§3/§4).

## Verification Layers

1. Rationale presence/shape per cause → unit test in `tests/visibility.rs`: `project_view` on a `Complete` terminal emits `decisive_cause = "finish_order_complete"`, correct `template_key`, seat-ring-ordered `final_standing` with rank-1 `winner = true`; on a `TurnLimit` terminal emits `turn_limit_progress_vector` with per-seat progress counts; `None` while live.
2. Determinism (rationale excluded from hash) → `stable_summary` / `stable_bytes` regression test asserting a live-vs-terminal pair's `stable_bytes` is unchanged by the new field (the byte string omits `terminal_rationale`).
3. No-leak (all-public) → FOUNDATIONS §11 alignment review: every rationale field is sourced from `finish_ranks` / `progress_score`, already public; no viewer-private input.

## What to Change

### 1. Add the game-local rationale types

In `games/starbridge_crossing/src/visibility.rs`, add `StarbridgeOutcomeRationaleView { result_kind, decisive_cause, template_key, decisive_rule_ids: Vec<String>, final_standing: Vec<StarbridgeOutcomeStandingView> }` and `StarbridgeOutcomeStandingView { seat: SeatId, seat_index, finish_rank: Option<u8>, winner: bool, finished: bool, progress: Option<u8> }` (progress populated only for the turn-limit cause, per `SC-FINISH-006`). Standings are stable in seat-ring order (`SC-FINISH-004`). No raw `seat_<n>` string is placed in any human-facing field.

### 2. Project it from terminal state only

Add `fn outcome_rationale(state: &StarbridgeState) -> Option<StarbridgeOutcomeRationaleView>` returning `Some` only when `state.terminal_status.is_some()`, branching on `Complete` vs `TurnLimit` for `result_kind` / `decisive_cause` / `template_key` / `decisive_rule_ids` (`SC-FINISH-001..004` for finish order; `SC-FINISH-005..006` for turn limit), and build `final_standing` from `finish_ranks` + the seat ring, reusing `rules`-level `progress_score` accounting for the turn-limit count. Set `terminal_rationale` in `project_view`; leave `stable_summary` / `stable_bytes` untouched.

## Files to Touch

- `games/starbridge_crossing/src/visibility.rs` (modify)
- `games/starbridge_crossing/tests/visibility.rs` (modify)

## Out of Scope

- wasm-api serialization of the field (GAT203STACROTER-002) and web rendering (GAT203STACROTER-003).
- Any movement / finish-rank / turn-limit *behavior* change — the rationale only projects existing terminal state.
- Any change to `terminal`, `stable_summary`, `stable_bytes`, golden traces, fixtures, or benchmark thresholds.

## Acceptance Criteria

### Tests That Must Pass

1. New `tests/visibility.rs` cases: `Complete` terminal → `finish_order_complete` rationale with seat-ring-ordered standings and rank-1 `winner`; `TurnLimit` terminal → `turn_limit_progress_vector` rationale with per-seat progress; live match → `terminal_rationale == None`.
2. `stable_bytes` determinism-guard test: the new field does not alter `StarbridgePublicView::stable_bytes`.
3. `cargo test -p starbridge_crossing`.

### Invariants

1. `terminal_rationale` is populated only at terminal and derived solely from already-public state (no hidden input).
2. `terminal_rationale` is absent from `stable_bytes` — replay/hash/trace artifacts are byte-identical.

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/visibility.rs` — rationale presence/shape per cause + live-`None`, and the `stable_bytes` determinism guard.

### Commands

1. `cargo test -p starbridge_crossing`
2. `cargo run -p replay-check -- --game starbridge_crossing --all` (confirms hashes/traces unchanged)
3. `cargo test -p starbridge_crossing` is the correct boundary — the field is game-local; cross-crate serialization is verified in GAT203STACROTER-002.
