# GAT91SECDRACOM-003: secret_draft state model + typed IDs + deterministic setup + variants

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft` only: `src/state.rs`, `src/setup.rs`, `src/variants.rs`, `src/ids.rs`. No `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-002

## Problem

The game needs its typed state model, ID types, deterministic setup, and the standard variant before legal actions or effects can be written. The state must model a shared visible pool with removal, hidden per-seat commitment slots kept internal, public drafted collections, round/priority/scores/terminal, and a freshness token — the substrate the no-leak firewall (GAT91SECDRACOM-006) later projects from.

## Assumption Reassessment (2026-06-08)

1. `games/high_card_duel/src/state.rs` is the behavioral template (verified): it carries `commitments: [Option<CardId>; 2]` (internal), `revealed_history: Vec<RevealedRound>`, a phase enum (`LeadCommit`/`ReplyCommit`/`Revealed`), and accessor `commitment_for(seat)`. `secret_draft` mirrors this with draft-local types: `commitments: [Option<DraftItemId>; 2]`, `visible_pool: Vec<DraftItemId>`, `drafted: [Vec<DraftItemId>; 2]`, `revealed_history: Vec<RevealedRound>`, `phase: Phase { Commit, Terminal }`.
2. Spec §State sketch (`specs/gate-9-1-secret-draft-commitment-reveal.md`) enumerates the state fields: `variant`, `seats: [SeatId; 2]`, `round_number: u8`, `phase`, `visible_pool`, `drafted`, `commitments` (internal only), `fallback_awards: [u8; 2]`, `priority_conflict_wins: [u8; 2]`, `scores: [u32; 2]`, `revealed_history`, `terminal_outcome: Option<TerminalOutcome>`, `freshness_token`. The standard variant uses twelve tiles / six rounds (spec A3); setup may be fully fixed (spec A4).
3. Cross-artifact boundary under audit: the state struct is the contract consumed by actions (004), rules/effects (005), visibility (006), replay (007), and bots (008). Field visibility matters — `commitments` is internal and MUST NOT be exposed by any view; this ticket establishes that field as private-by-construction so the firewall in 006 has a clean internal/public split.
4. §11 determinism is the motivating invariant: restate before trusting spec — setup must be deterministic. The standard variant is fully fixed (no RNG); if any seeded ordering is later added it must use the engine's existing `SeededRng` contract, never wall-clock (spec A4). Stable serialization/iteration order: `visible_pool` is an ordered `Vec` in stable item order, not a hash set.
5. Replay/serialization substrate: the state shape feeds deterministic state hashing (GAT91SECDRACOM-007/010). This ticket introduces no nondeterministic field (no timestamps, no hash-map iteration in canonical form); the freshness token is a monotonic counter, not wall-clock.

## Architecture Check

1. Modeling `commitments` as an internal `[Option<DraftItemId>; 2]` separate from the public `drafted` collections is cleaner than a single tagged field: it makes the pre-reveal/post-reveal visibility split structural rather than a runtime filter, reducing leak surface.
2. No backwards-compatibility aliasing/shims — fills in skeleton stubs from GAT91SECDRACOM-002.
3. `engine-core` stays noun-free; all state nouns are game-local. No `game-stdlib` helper introduced; high_card_duel commitment machinery is adapted in-crate, not promoted.

## Verification Layers

1. Deterministic setup -> unit test: two setups of `secret_draft_standard` produce byte-identical state (stable pool order, empty commitments, round 1, priority seat_0).
2. State shape / serialization order -> serialization unit test (stable summary) + boundary against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
3. Internal-commitment invariant -> grep-proof that `commitments` is not in any public view type yet (full no-leak coverage in GAT91SECDRACOM-006/009).
4. Boundary noun-freedom -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. `src/ids.rs`

Finalize `DraftItemId` (stable original IDs `ember_1`…`grove_4`), seat indexing helpers, and any `FreshnessToken` type, mirroring `high_card_duel`/`token_bazaar` ID conventions.

### 2. `src/state.rs`

Define the full state struct per the spec sketch, with `commitments` private/internal and a `RevealedRound`/`TerminalOutcome` type. Provide accessors that never expose raw `commitments` pre-reveal. Include `phase: Phase { Commit, Terminal }` (optional transient `Resolving` only if a later test needs it).

### 3. `src/setup.rs`

Deterministic fixed setup for `secret_draft_standard`: load twelve tiles in stable order into `visible_pool`, round 1, priority `seat_0`, empty commitments/drafted, scores 0, terminal `None`, freshness 0. Validate exactly two seats.

### 4. `src/variants.rs`

Define `secret_draft_standard` variant (twelve tiles, six rounds) wired to the validated manifest/variants data from GAT91SECDRACOM-002.

## Files to Touch

- `games/secret_draft/src/ids.rs` (modify)
- `games/secret_draft/src/state.rs` (modify)
- `games/secret_draft/src/setup.rs` (modify)
- `games/secret_draft/src/variants.rs` (modify)

## Out of Scope

- Legal action generation and validation (GAT91SECDRACOM-004).
- Apply/resolve/reveal transitions, scoring, terminal detection (GAT91SECDRACOM-005).
- View projection and no-leak filtering (GAT91SECDRACOM-006) — this ticket only keeps `commitments` structurally internal.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` — deterministic-setup test (two setups identical) passes.
2. Serialization/summary unit test produces a stable summary for initial state.
3. Setup rejects a non-two-seat configuration.

### Invariants

1. Setup is deterministic and uses no wall-clock / nondeterministic input (§11/§2).
2. `commitments` is internal — absent from any public-facing type introduced so far (§11 no-leak substrate).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/setup.rs` (or `state.rs`) inline unit tests — deterministic setup + two-seat validation + stable initial summary.

### Commands

1. `cargo test -p secret_draft`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Simulation/replay CLI verification is deferred to GAT91SECDRACOM-010+ (no actions/effects exist yet); unit tests are the correct boundary for a state/setup ticket.
