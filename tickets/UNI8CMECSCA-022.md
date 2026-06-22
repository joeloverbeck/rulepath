# UNI8CMECSCA-022: Implement five profile drivers in `game-test-support::profiles`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/game-test-support/src/profiles.rs` (new), `crates/game-test-support/src/lib.rs`
**Deps**: UNI8CMECSCA-018, UNI8CMECSCA-003

## Problem

The five ADR-0009 fixture profiles need reusable test orchestration that validates inputs/expected evidence without relocating game behavior. This ticket implements five **distinct** typed profile-driver modules — `ReplayCommandV1Driver`, `PublicExportV1Driver`, `SeatPrivateExportV1Driver`, `SetupEvidenceV1Driver`, `DomainEvidenceV1Driver` — plus shared metadata-validation primitives. It must **not** be one permissive fixture union: each driver rejects wrong profile ID/version, missing visibility, mismatched validator owner, illegal canonical-byte claims, absent migration notes, and profile-specific field misuse (C-08).

## Assumption Reassessment (2026-06-22)

1. The five profile names are defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`: `replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, `domain-evidence-v1` (confirmed by grep at the reassessed commit). `crates/game-test-support` exists with a `profiles` module stub after UNI8CMECSCA-018.
2. Spec §5 "C-08 profile-driver contract" fixes each driver's shared-vs-delegated split: each may own profile/version validation, round-trip/loop sequencing, and metadata checks; each must delegate setup/legality/projection/redaction/authorization/reveal/evaluator/scoring/hashing to game-owned code. ADR 0009 (`docs/adr/0009-replay-fixture-hash-taxonomy.md`, `Accepted`) governs. Register entry `MSC-8C-008` homes this in `game-test-support` + thin validator adapters.
3. Cross-artifact boundary under audit: the ADR-0009 profile taxonomy (`docs/EVIDENCE-FIXTURE-CONTRACT.md`) and the driver modules. Five distinct typed paths, not a union; unknown fields reject per the owning validator's strictness rule.
4. FOUNDATIONS §5/§11: profile drivers validate inputs/expected evidence and reject behavior-looking keys (selectors/formulas/triggers/procedures); `canonical_byte_authority = none` means validate structure/semantics without inventing a stable-byte claim.
5. Determinism/no-leak substrate (§11/EC-19/EC-22): the drivers are the enforcement surface for "fixtures are input/expected evidence only"; they execute no behavior from fixture keys and must not let a seat-private export restore omniscient state (the seat-private driver delegates authorization/reveal to the game).

## Architecture Check

1. Five distinct typed drivers with a shared metadata core (vs. one permissive union) is the only shape that can reject cross-profile field misuse — the spec's explicit anti-union requirement.
2. No backwards-compatibility shim — new module; nothing aliased.
3. `engine-core`/`game-stdlib` untouched; the drivers delegate all behavior to game-owned functions and own only orchestration/metadata.

## Verification Layers

1. Each driver rejects wrong profile ID/version, missing visibility, mismatched validator owner, illegal canonical-byte claim, absent migration note, and a foreign-profile field → `game-test-support` unit tests per driver.
2. No permissive union: a field valid for profile A is rejected by profile B's driver → cross-profile negative tests.
3. Drivers execute no behavior from fixture data → grep-proof `profiles.rs` reads no selector/formula/trigger.
4. ADR-0009 alignment → FOUNDATIONS/ADR-0009 alignment check in review.

## What to Change

### 1. `crates/game-test-support/src/profiles.rs` (new)

Implement the five driver types and shared metadata-validation primitives per the C-08 contract; each driver delegates the listed semantic work to game-supplied closures/adapters. Reject unknown/foreign-profile fields and illegal canonical-byte claims.

### 2. `crates/game-test-support/src/lib.rs`

Wire `pub mod profiles;` (replace the stub).

### 3. Driver unit tests

Positive + negative conformance per driver (wrong ID/version, missing visibility, mismatched owner, illegal byte claim, absent migration note, profile-specific field misuse).

## Files to Touch

- `crates/game-test-support/src/profiles.rs` (new)
- `crates/game-test-support/src/lib.rs` (modify)

## Out of Scope

- Adopting any driver in a game (Race UNI8CMECSCA-023, River 024, Vow 025, Briar 026) or tool (027).
- A permissive fixture union or any behavior executed from fixture keys.
- Projection/redaction/authorization/scoring logic (delegated to games).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-test-support` passes, with positive + negative conformance for all five drivers.
2. A cross-profile test proves a field valid for one profile is rejected by another's driver.
3. `cargo build --workspace` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. Five distinct typed drivers exist; no permissive union accepts another profile's fields.
2. No driver reads selectors/formulas/triggers/procedures from fixture data; `canonical_byte_authority = none` invents no hash.

## Test Plan

### New/Modified Tests

1. `crates/game-test-support/src/profiles.rs` (inline `#[cfg(test)]`) — per-driver positive/negative + cross-profile rejection (EV-PROFILES).

### Commands

1. `cargo test -p game-test-support`
2. `bash scripts/boundary-check.sh`
3. The `game-test-support` unit suite is the correct boundary — game/tool adoption follows in UNI8CMECSCA-023…027.
