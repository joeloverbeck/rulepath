# UNI8CMECSCA-006: Pilot C-01 effect-envelope constructors in Race to N and River Ledger

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/race_to_n/src/effects.rs`, `games/river_ledger/src/effects.rs`
**Deps**: UNI8CMECSCA-005, UNI8CMECSCA-003

## Problem

Prove the C-01 constructors (UNI8CMECSCA-005) against real call sites by replacing matching `EffectEnvelope` struct literals in Race to N (public effects) and River Ledger (public + seat-private effects) with `EffectEnvelope::public` / `private_to`. The change must be byte/hash/visibility identical — it adopts a constructor, it does not change payload construction or ordering. This is the first real consumer of `MSC-8C-001` and lets that register entry flip to `accepted`.

## Assumption Reassessment (2026-06-22)

1. `games/race_to_n/src/effects.rs` and `games/river_ledger/src/effects.rs` exist and construct `EffectEnvelope<T>` by struct literal; River builds both public and seat-private envelopes (confirmed by grep). The constructors `EffectEnvelope::public`/`private_to` exist after UNI8CMECSCA-005.
2. The UNI8CMECSCA-003 characterization packet pins Race public effects and River public/private effect bytes/hashes/visibility; this pilot must reproduce them exactly. Spec §4.5 limits Race's adoption: "Does not authorize mass conversion of legacy hyphenated fixture bytes."
3. Cross-artifact boundary under audit: each game's effect-construction sites and their golden traces / visibility suites (`games/race_to_n/tests/replay_tests.rs`, `games/river_ledger/tests/visibility.rs`, `tests/replay.rs`). The extension is constructor-for-literal only.
4. FOUNDATIONS §11 behavior neutrality (EC-04): public/private filtering, payload, order, serialized bytes, hashes, and visibility are preserved; the constructor sets the same `VisibilityScope` the literal did.
5. No-leak surface under audit (§11): River seat-private envelopes must keep their exact `PrivateToSeat` scope; the pilot changes the construction syntax, not the redaction. Existing no-leak/visibility tests must stay green and are the proof.

## Architecture Check

1. Replacing literals at real sites is the minimum proof that the kernel constructor is adoptable without behavior change; it removes per-site visibility-scope repetition.
2. No backwards-compatibility shim — literals are replaced, not aliased.
3. `engine-core` untouched (helper already landed); no game policy moves into shared code.

## Verification Layers

1. Race public effects byte/hash/visibility identical to the UNI8CMECSCA-003 baseline → `cargo run -p replay-check -- --game race_to_n --all`.
2. River public/private effects byte/hash/visibility identical → `cargo run -p replay-check -- --game river_ledger --all`.
3. River seat-private redaction unchanged → `cargo test -p river_ledger` (visibility suite).
4. Constructor adoption compiles and the workspace stays green → `cargo test --workspace`.

## What to Change

### 1. `games/race_to_n/src/effects.rs`

Replace public `EffectEnvelope { visibility: Public, payload }` literals with `EffectEnvelope::public(payload)`. No payload change.

### 2. `games/river_ledger/src/effects.rs`

Replace public literals with `public(..)` and seat-private literals with `private_to(seat, ..)`, preserving the exact seat and payload.

## Files to Touch

- `games/race_to_n/src/effects.rs` (modify)
- `games/river_ledger/src/effects.rs` (modify)

## Out of Scope

- Migrating any other game's effects (deferred to the C-11 waves).
- Changing payloads, ordering, fixture bytes, or seat spellings.
- Adopting canonical seat IDs or counts (UNI8CMECSCA-009/011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game race_to_n --all` and `--game river_ledger --all` pass with unchanged hashes.
2. `cargo test -p race_to_n -p river_ledger` passes (visibility + replay suites).
3. `cargo test --workspace` passes.

### Invariants

1. Effect bytes, hashes, and `VisibilityScope` are identical to the UNI8CMECSCA-003 baseline.
2. No seat-private fact changes scope or leaks.

## Test Plan

### New/Modified Tests

1. `None — no new test; the existing Race/River replay + visibility suites and the UNI8CMECSCA-003 characterization assertions are the regression guard.`

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all && cargo run -p replay-check -- --game river_ledger --all`
2. `cargo test -p race_to_n -p river_ledger`
3. `replay-check` plus the visibility suites are the correct boundary because behavior-neutrality is proven by byte/hash/visibility identity, not by new assertions.

## Outcome

Completed: 2026-06-22

What changed:
- Replaced the Race to N public effect-envelope literal with `EffectEnvelope::public(payload)`.
- Replaced River Ledger public and seat-private effect-envelope literals with `EffectEnvelope::public(payload)` and `EffectEnvelope::private_to(owner_seat_id, payload)`.
- Flipped `MSC-8C-001` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from `candidate` to `accepted` with the constructor and pilot verification evidence.

Deviations:
- None. Payload construction, effect order, seat scopes, goldens, hashes, and visibility filtering were unchanged.

Verification:
- `cargo fmt --all --check`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo test -p race_to_n -p river_ledger`
- `cargo test --workspace`
- `node scripts/check-doc-links.mjs`
