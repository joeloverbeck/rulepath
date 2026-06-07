# GAT72GAT8HIG-011: No-leak test suite — visibility / property / serialization

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/tests/{visibility.rs,property.rs,serialization.rs}`
**Deps**: GAT72GAT8HIG-008, GAT72GAT8HIG-009

## Problem

The hidden-information acceptance heart of Gate 8 is a cross-cutting test suite
that proves, across random seeds and actions, that no projection / effect set /
serialized form / replay export ever exposes hidden state to an unauthorized
viewer, and that card conservation and public-projection stability hold.

## Assumption Reassessment (2026-06-07)

1. Verified the surfaces under test exist after prior tickets: views/effects
   (`visibility.rs`/`effects.rs` from 005/008), transitions (`rules.rs` from
   007), replay export (`replay_support.rs` from 009). The per-module tickets
   carry focused unit tests; this ticket consolidates the cross-cutting proofs.
2. Verified against the spec: §4.2.8 enumerates the visibility, property/
   invariant, and serialization test sets (observer/seat no-leak, effect-filter
   sets, conservation, monotonic rounds, public projection never grows hidden
   fields, public export never contains unrevealed identities, stable view
   schemas, strict unknown-field rejection where conventions require).
3. Cross-artifact boundary under audit: the public/private view + effect +
   replay-export schemas together — this suite is the integration firewall over
   all three, not a single-module test.
4. FOUNDATIONS principle under audit (§11 no-leak firewall + determinism):
   hidden information must not reach any unauthorized viewer surface, and public
   projection/serialization must be stable and deterministic.
5. Enforcement surface named: the §11 no-leak firewall and deterministic
   serialization. This is the gate's primary enforcement test; confirm it covers
   observer + `seat_0` + `seat_1` effect-filter sets, public-projection field
   stability across seeds, and public-export absence of `hcd:r..` identities.
   Note: `tests/serialization.rs` is a dedicated file — existing games fold
   serialization coverage into `visibility.rs`/`replay.rs`; this consolidates the
   §4.2.8 serialization set into one file (a deliberate, cleaner divergence).

## Architecture Check

1. A consolidated cross-cutting suite gives the no-leak firewall one
   authoritative proof surface, rather than scattering leak assertions where
   they can be missed; per-module unit tests remain for fast feedback.
2. No backwards-compatibility shims — new tests.
3. Tests only; no `engine-core`/`game-stdlib` change.

## Verification Layers

1. View no-leak across seeds -> no-leak visibility test: observer view never grows hidden fields over random seeds/actions; seat views stay scoped.
2. Effect-filter correctness -> no-leak visibility test: effect sets for observer/`seat_0`/`seat_1` are correct.
3. Card conservation -> property test: no duplicate/lost card; no card in two zones; committed cards reveal exactly once.
4. Serialization stability + unknown-field rejection -> schema/serialization validation: stable public/seat-private/internal schemas; unknown fields rejected where conventions require.

## What to Change

### 1. `tests/visibility.rs` (extend)

Observer/seat no-leak across seeds; disabled-reasons/previews/action metadata
carry no private ids; effect-filter sets per viewer.

### 2. `tests/property.rs` (extend)

Conservation, single-reveal, monotonic rounds, public-projection stability,
internal-trace reproduction, public-export no-leak.

### 3. `tests/serialization.rs` (new)

Strict unknown-field rejection where required; stable public/seat-private/
internal schemas; public export has no hidden fields; import semantics.

## Files to Touch

- `games/high_card_duel/tests/visibility.rs` (modify — extend)
- `games/high_card_duel/tests/property.rs` (modify — extend)
- `games/high_card_duel/tests/serialization.rs` (new)

## Out of Scope

- Golden traces / fixture (GAT72GAT8HIG-012) and e2e/browser no-leak (019).
- Production logic changes — this ticket only adds tests against shipped behavior.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel --test visibility`, `--test property`, `--test serialization` all pass.
2. `public_projection_never_grows_hidden_fields_across_seeds`, `public_replay_export_never_contains_unrevealed_internal_card_identities`.
3. `effect_filtering_returns_correct_sets_for_observer_seat0_seat1`.

### Invariants

1. No unauthorized viewer surface ever carries hidden state (§11 no-leak firewall).
2. Public/seat/internal serialized schemas are stable and deterministic.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs`, `tests/property.rs`, `tests/serialization.rs` — the cross-cutting no-leak/conservation/serialization proofs.

### Commands

1. `cargo test -p high_card_duel --test visibility --test property --test serialization`
2. `cargo test -p high_card_duel`
3. This suite is the correct native boundary; browser-surface no-leak is proven separately in 019.
