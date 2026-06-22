# UNI8CMECSCA-027: Thin profile dispatch in `fixture-check` and `replay-check`

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`
**Deps**: UNI8CMECSCA-023, UNI8CMECSCA-024, UNI8CMECSCA-025, UNI8CMECSCA-026

## Problem

The pilots (UNI8CMECSCA-023…026) need the canonical validators to recognize the ADR-0009 profiles they adopt. This ticket adds thin profile registration/dispatch to `fixture-check` and `replay-check` only where the pilots require it. The tools keep canonical validator ownership and invoke game-owned validators; they do not depend on `game-test-support` by default and acquire no game behavior. Unknown profile/fields reject; no behavior-looking fixture key becomes executable; no production dependency edge appears.

## Outcome

Completed: 2026-06-22

Implemented local profile registration/dispatch in `replay-check` for
`replay-command-v1`, `public-export-v1`, and `seat-private-export-v1`, and in
`fixture-check` for `setup-evidence-v1` and `domain-evidence-v1`. The tools
validate optional embedded profile metadata when present, reject unknown
profiles and cross-profile fields, and keep current legacy pilot artifacts
valid without changing their bytes. `fixture-check --game river_ledger` now
also includes the River 3-seat setup fixture, and `fixture-check --game
briar_circuit` includes the Briar moon and first-trick domain fixtures. No
`game-test-support` dependency was added to either tool.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p replay-check -p fixture-check`
3. `cargo run -p replay-check -- --game race_to_n --all`
4. `cargo run -p replay-check -- --game vow_tide --all`
5. `cargo run -p fixture-check -- --game river_ledger`
6. `cargo run -p fixture-check -- --game briar_circuit`
7. `cargo tree --workspace -e normal --invert game-test-support`
8. `cargo tree --workspace -e normal,build --invert game-test-support`
9. `bash scripts/boundary-check.sh`
10. `cargo test --workspace`

The literal `selector|formula|trigger|procedure` grep over the tool files only
finds the existing forbidden-key list/tests and unrelated Token Bazaar terminal
state field names; the new profile dispatch does not execute behavior-looking
fixture keys.

## Assumption Reassessment (2026-06-22)

1. `tools/replay-check/src/main.rs` and `tools/fixture-check/src/main.rs` exist and are the canonical validators (they own validation today). The five profiles are defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`; the pilots adopting them land in UNI8CMECSCA-023 (replay-command, Race), 024 (setup-evidence, River), 025 (public/seat-private export, Vow), 026 (domain-evidence, Briar).
2. Spec §4.1 + §5 8C-027 + A-07 fix the boundary: thin registration/profile-dispatch only where pilots require; tools remain validator owners and must not acquire game behavior; keep tools independent of `game-test-support` by default; a tool-only normal edge may be proposed only if the boundary law explicitly permits and guards it.
3. Cross-artifact boundary under audit: the validator CLIs (`tools/*`) and the profile taxonomy. The tools dispatch by profile and invoke game-owned validators; they do not relocate game logic.
4. FOUNDATIONS §2/§5/§11: validation stays fail-closed (unknown profile/fields reject); no behavior-looking fixture key becomes executable; the tools decide no game legality.
5. Determinism/no-leak substrate (§11/EC-23): the thin adapters validate the pilot profiles and retain canonical ownership; no production dependency edge to `game-test-support` appears (guarded by UNI8CMECSCA-018's boundary check).

## Architecture Check

1. Thin profile-dispatch in the existing validators (vs. moving validation into `game-test-support`) keeps canonical ownership in the tools and avoids contaminating their dependency graph.
2. No backwards-compatibility shim — additive profile arms; unknown profiles fail closed.
3. `engine-core`/`game-stdlib` untouched; tools stay independent of `game-test-support` by default (A-07).

## Verification Layers

1. CLI validates each pilot profile artifact → `cargo run -p replay-check -- --game race_to_n --all`, `--game vow_tide --all`; `cargo run -p fixture-check -- --game river_ledger`, `--game briar_circuit`.
2. Unknown profile/fields reject (fail-closed) → strict-rejection unit/integration tests in the tools.
3. No behavior-looking fixture key becomes executable → grep-proof the dispatch reads no selector/formula/trigger.
4. No production dependency edge to `game-test-support` → `cargo tree --workspace -e normal --invert game-test-support` (no tool edge) + `bash scripts/boundary-check.sh`.

## What to Change

### 1. `tools/replay-check/src/main.rs`

Add thin dispatch for the `replay-command-v1` (and export profiles where replay-check validates them) so the Race/Vow pilots' artifacts validate; invoke game-owned validators; reject unknown profile/fields.

### 2. `tools/fixture-check/src/main.rs`

Add thin dispatch for `setup-evidence-v1` and `domain-evidence-v1` so the River/Briar pilots' fixtures validate; reject unknown profile/fields.

## Files to Touch

- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)

## Out of Scope

- Moving any game validator logic into the tools or into `game-test-support`.
- A normal `game-test-support` edge from the tools (A-07; only if boundary law permits + guards).
- Executing behavior from fixture keys.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game race_to_n --all` and `--game vow_tide --all` pass; `cargo run -p fixture-check -- --game river_ledger` and `--game briar_circuit` pass.
2. A strict-rejection test proves unknown profile/fields reject.
3. `bash scripts/boundary-check.sh` and `cargo tree --workspace -e normal --invert game-test-support` show no tool→`game-test-support` normal edge.

### Invariants

1. The tools retain canonical validator ownership; no game logic relocates into them.
2. Unknown profile/fields reject; no behavior-looking key becomes executable.

## Test Plan

### New/Modified Tests

1. `tools/replay-check/src/main.rs` / `tools/fixture-check/src/main.rs` — profile-dispatch + strict-rejection tests (EV-TOOLS).

### Commands

1. `cargo run -p fixture-check -- --game river_ledger && cargo run -p fixture-check -- --game briar_circuit`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The tool CLIs plus the boundary check are the correct boundary — profile dispatch must validate the pilots without acquiring a production edge.
