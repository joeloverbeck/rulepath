# UNI8CMECSCA-009: Pilot canonical seat IDs in Race to N and River Ledger

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/race_to_n/src/ids.rs`, `games/river_ledger/src/ids.rs`
**Deps**: UNI8CMECSCA-008

## Problem

Route Race to N and River Ledger seat formatting/parsing through the kernel canonical `SeatId` API (UNI8CMECSCA-007) instead of bespoke `format!("seat_{}", …)`/local `parse`. Legacy hyphen trace inputs must keep importing (via the UNI8CMECSCA-008 adapter); historical actor/viewer strings are **not** rewritten merely to adopt canonical output. Any hash-bearing string change is a separate, named ADR-0009 migration — not hidden in this pilot.

## Assumption Reassessment (2026-06-22)

1. `games/race_to_n/src/ids.rs` and `games/river_ledger/src/ids.rs` already format underscore canonical (`"seat_0"`, `format!("seat_{}", self.index)`) and parse underscore (confirmed by grep). So adopting the kernel `from_zero_based_index`/`parse_canonical`/`canonical_zero_based_index` is byte-neutral for these games' `ids.rs`.
2. Spec §5 8C-009 review boundary: existing legacy hyphen fixtures import; canonical output round-trips; River variable-seat IDs remain identical; any hash-bearing string change is a separate named migration. The UNI8CMECSCA-003 packet pins the current seat spellings to compare against.
3. Cross-artifact boundary under audit: each game's `ids.rs` seat type and the golden traces / fixtures that carry seat strings (`games/race_to_n/tests/golden_traces/shortest-normal.trace.json` is hyphen via the trace adapter; `games/river_ledger` traces/fixtures are underscore). The pilot changes the *source of* canonical formatting, not the committed strings.
4. FOUNDATIONS §2: canonical parse/format authority stays in Rust (kernel); no TypeScript change. §11 determinism: output must round-trip identically.
5. Deterministic replay/hash surface under audit (§11/EC-06): Race's committed hyphen golden remains readable and unchanged; this ticket does **not** flip `trace_race_seat` to underscore (that hash-bearing flip, if ever taken, is its own named ADR-0009 migration with before/after evidence and rollback). River variable-seat IDs stay byte-identical.

## Architecture Check

1. Centralizing each game's seat formatting on the kernel API removes per-game `format!`/`parse` drift while preserving committed bytes.
2. No backwards-compatibility shim — the games call the kernel API directly; legacy reading lives in the UNI8CMECSCA-008 adapter, not duplicated here.
3. `engine-core` untouched (API already landed); no game policy moves to shared code.

## Verification Layers

1. Race/River seat round-trip through the kernel API is byte-identical to baseline → `cargo run -p replay-check -- --game race_to_n --all`, `--game river_ledger --all`.
2. Legacy hyphen fixtures still import → `cargo test -p race_to_n` (legacy-fixture read test).
3. River variable-seat IDs (3–6) unchanged → `cargo test -p river_ledger`.
4. No hidden seat-string rewrite → grep-proof the committed goldens are untouched.

## What to Change

### 1. `games/race_to_n/src/ids.rs`

Replace local `format!`/`parse` with `SeatId::from_zero_based_index` / `parse_canonical` / `canonical_zero_based_index`, preserving the exact emitted strings.

### 2. `games/river_ledger/src/ids.rs`

Same adoption for the variable-seat IDs, preserving byte-identical output across counts 3–6.

## Files to Touch

- `games/race_to_n/src/ids.rs` (modify)
- `games/river_ledger/src/ids.rs` (modify)

## Out of Scope

- Flipping `trace_race_seat` (or any hyphen-emitting adapter) to underscore — a separate named ADR-0009 migration.
- Rewriting historical actor/viewer strings in committed traces/fixtures.
- Adopting canonical seats in any other game.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game race_to_n --all` and `--game river_ledger --all` pass with unchanged hashes.
2. `cargo test -p race_to_n -p river_ledger` passes, including legacy-fixture import and 3–6 seat coverage.
3. `cargo test --workspace` passes.

### Invariants

1. No committed golden trace or fixture seat string changes.
2. River variable-seat IDs are byte-identical across counts 3–6.

## Test Plan

### New/Modified Tests

1. `None — no new test; the existing Race/River replay suites, legacy-fixture import tests, and the UNI8CMECSCA-003 baseline are the regression guard.`

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all && cargo run -p replay-check -- --game river_ledger --all`
2. `cargo test -p race_to_n -p river_ledger`
3. `replay-check` plus the games' suites are the correct boundary because byte/hash identity proves the adoption is neutral.

## Outcome

Completed: 2026-06-22

What changed:
- Routed `RaceSeat::parse` through the kernel `SeatId::parse_canonical` / `canonical_zero_based_index` API while preserving the existing borrowed `as_str()` output.
- Routed River Ledger seat formatting, parsing, `SeatId` construction, and actor construction through the kernel canonical seat API while preserving byte-identical `seat_<n>` output.
- Flipped `MSC-8C-002` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from `candidate` to `accepted` with the 007-009 proof evidence.

Deviations:
- None. `trace_race_seat` and historical hyphen trace/fixture strings were not changed; no committed golden trace or fixture path changed.

Verification:
- `cargo fmt --all --check`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo test -p race_to_n -p river_ledger`
- `cargo test --workspace`
- `git diff --quiet -- games/race_to_n/tests/golden_traces games/river_ledger/tests/golden_traces games/river_ledger/data games/race_to_n/data`
- `node scripts/check-doc-links.mjs`
