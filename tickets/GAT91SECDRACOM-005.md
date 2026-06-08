# GAT91SECDRACOM-005: secret_draft apply/resolve/reveal effects + conflict fallback + scoring + terminal/tie-breaks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/secret_draft/src/effects.rs` and `src/rules.rs`; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-004

## Problem

This is the architectural center of the gate: applying a validated commitment, emitting pending-only effects for the first commit, then resolving a single synchronized reveal batch after the second commit — including deterministic conflict fallback, drafting-with-removal, public scoring, round advance, terminal detection, and the tie-break ladder. No pre-reveal item ID may appear in any public/browser-bound effect payload.

## Assumption Reassessment (2026-06-08)

1. State + validated action from GAT91SECDRACOM-003/004 are inputs. `games/high_card_duel/src/effects.rs` (commit → reveal → resolved-round effect sequence) is the behavioral template; `games/token_bazaar/src/{effects.rs,rules.rs}` is the structural template for terminal/tie-break emission.
2. Spec §"Semantic effect model" defines the effect set and visibility: `CommitmentPlaced { seat, round }` (public, no item ID), optional `OwnCommitAccepted` (no item ID pre-reveal), `PendingSeatsChanged { round, seat_0_committed, seat_1_committed }` (booleans), `RevealBatchStarted { round, group_id }`, `ChoicesRevealed { round, seat_0_item, seat_1_item }` (first public appearance of item IDs), `DraftResolved { round, awards, removed_items, conflict }`, `PoolChanged`, `ScoreChanged { scores, tie_break_summary }`, `RoundAdvanced { next_round, priority_seat }`, `Terminal { outcome, final_scores, tie_break_summary }`, plus `PrivateDiagnostic`/`PublicDiagnostic`. Spec §"Conflict resolution", §Scoring, §"Terminal and tie-breaks" define the deterministic rules.
3. Cross-artifact boundary under audit: the effect-envelope contract (`docs/ARCHITECTURE.md` §effects — confirmed it lists "reveal/redaction, commitments/reveals, pending responses, grouped batches") and the pre-reveal payload rule. The effect payloads here are the exact surface GAT91SECDRACOM-006 filters and GAT91SECDRACOM-007 exports; payload field choices are load-bearing for no-leak.
4. §11 no-leak firewall + §2 behavior authority are the motivating principles: restate before trusting spec — committed item IDs live only in internal state until `ChoicesRevealed`; first-commit and pending effects carry seat/round/booleans only. Conflict fallback, scoring, terminal detection, and tie-breaks are typed Rust, not data formulas (§5).
5. §11 determinism: reveal resolution, conflict fallback (priority seat takes contested item; other seat takes lowest stable-order remaining item after removal), scoring, and the tie-break ladder must be deterministic and stable-ordered. Effect ordering within the reveal batch is fixed (`RevealBatchStarted` → `ChoicesRevealed` → `DraftResolved` → `ScoreChanged` → `RoundAdvanced`/`Terminal`); this ordering feeds replay-hash (GAT91SECDRACOM-010) and must not change accidentally.

## Architecture Check

1. Resolving the reveal batch synchronously on the second commit (rather than a separate reveal action) is cleaner and avoids a post-reveal reaction window the gate explicitly excludes, while still proving simultaneous choice + removal.
2. No backwards-compatibility aliasing/shims — fills GAT91SECDRACOM-002 stubs.
3. `engine-core` stays noun-free; all reveal/conflict/scoring nouns are game-local. No `game-stdlib` helper (first official use of simultaneous commitment/reveal; atlas §10A `_None_`).

## Verification Layers

1. Pre-reveal no-leak -> unit test: after first commit, emitted effects contain `PendingSeatsChanged`/`CommitmentPlaced` with no item ID; the committed ID exists only in internal state.
2. Reveal batch ordering -> unit test asserting the fixed effect sequence and both item IDs appearing together in `ChoicesRevealed`.
3. Conflict fallback determinism -> unit test: both seats pick same item → priority seat gets it, other gets lowest stable-order remaining, exactly two items removed.
4. Scoring + tie-break ladder -> unit tests covering base/set/high-thread/conflict-discipline bonuses and each tie-break rung down to `Draw`.
5. Terminal -> unit test: game ends after round 6 resolves.

## What to Change

### 1. `src/effects.rs`

Define the game-local semantic effects per the spec table, each behind the generic effect envelope, with payload rules enforcing no pre-reveal item ID. Implement first-commit (pending-only) emission and the grouped reveal batch emission.

### 2. `src/rules.rs`

Implement: apply a `ValidatedAction` (store commitment internally, increment freshness); on second commit, resolve — conflict fallback + drafting-with-removal, scoring (base + set + high-thread + conflict-discipline bonuses), score/round/terminal updates, and the public tie-break ladder producing winner/draw + tie-break summary. Clear commitments and remove awarded items on cleanup.

## Files to Touch

- `games/secret_draft/src/effects.rs` (modify)
- `games/secret_draft/src/rules.rs` (modify)

## Out of Scope

- View projection / effect filtering for viewers (GAT91SECDRACOM-006) — this ticket only guarantees payloads carry no pre-reveal ID at emission.
- Replay export/import (GAT91SECDRACOM-007).
- Golden traces (GAT91SECDRACOM-010) — unit tests here, named traces there.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` — first-commit-no-leak, reveal-batch-ordering, conflict-fallback, scoring, tie-break, and terminal tests pass.
2. A property-style test (or seeded sequences) confirms exactly two items leave the pool per resolved round and terminal occurs within six rounds (full property suite in GAT91SECDRACOM-009).

### Invariants

1. No committed item ID appears in any effect payload before `ChoicesRevealed` (§11 no-leak).
2. Conflict/scoring/terminal/tie-breaks are typed Rust, deterministic, stable-ordered; effect order within the reveal batch is fixed (§5/§11).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/{effects.rs,rules.rs}` inline unit tests — pre-reveal payload safety, reveal-batch order, conflict fallback, scoring components, tie-break ladder, terminal cap.

### Commands

1. `cargo test -p secret_draft rules` and `cargo test -p secret_draft effects`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Targeted `rules`/`effects` filters are the correct boundary; full deterministic replay-hash of the effect ordering is proven in GAT91SECDRACOM-010 golden traces.
