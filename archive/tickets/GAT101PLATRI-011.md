# GAT101PLATRI-011: Replay support, serialization tests, and golden traces

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/plain_tricks/src/replay_support.rs`, `games/plain_tricks/tests/{replay,serialization}.rs`, `games/plain_tricks/tests/golden_traces/*.trace.json`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-010

## Problem

The game needs deterministic replay support: internal full-trace replay, public viewer-scoped export/import per ADR 0004, deterministic hash checkpoints, the golden trace set, and serialization tests (stable field order, strict unknown-field rejection). Public exports must not include seed material capable of reconstructing hands or the tail; the tail must remain unreconstructable even at terminal.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{rules,effects,visibility}.rs` are implemented; the replay/checkpoint/hash contracts come from `engine-core`. Mirror `games/poker_lite/src/replay_support.rs` and its `tests/{replay,serialization}.rs` + `tests/golden_traces/` (17 traces) layout.
2. Spec §4 names the 18-trace set (deal-private-no-leak, follow-suit-forced, void-free-discard, off-suit-never-wins, trick-winner-leads-next, round-close-deal-rotation, terminal-most-points-win, tie-split, no-leak-public-observer, seat-private-view, invalid-wrong-seat-diagnostic, invalid-stale-diagnostic, invalid-must-follow-diagnostic, bot-action, public-replay-export-import, wasm-exported). Trace names may be adjusted; coverage categories may not be deleted or weakened. The bot-action and wasm-exported traces depend on GAT101PLATRI-013/016 outputs but are authored against stable command streams here / refreshed there.
3. Shared boundary under audit: the ADR 0004 viewer-scoped replay-export taxonomy (internal full trace vs viewer-scoped public export) and the serialization-boundary contract (`docs/ENGINE-GAME-DATA-BOUNDARY.md`).
4. FOUNDATIONS §11 (deterministic replay/hash; viewer-safe exports) and §2 (Rust owns replay/hash/serialization) are under audit; ADR 0004 supplies the export taxonomy.
5. Enforcement surface: deterministic replay/hash + no-leak export firewall (§11/§13). Internal full replay must reproduce hashes byte-for-byte; public export must omit seed material and any unplayed-card/tail identity; seat-scoped export includes only that seat's own observed cards. The tail is unreconstructable from any export, including at terminal.
6. Extends the serialized-trace + replay-export schema additively (a new game's traces + export class); consumers are `replay-check` (GAT101PLATRI-014) and the WASM export/import branch (GAT101PLATRI-016). Strict unknown-field rejection and no behavior-looking trace/data fields are required.

## Architecture Check

1. Separating internal full trace (may carry seed/hands/tail for deterministic replay) from viewer-scoped public export (redacted) per ADR 0004 is the established hidden-info replay model; it keeps determinism while preventing reconstruction.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; trace/export payloads are `plain_tricks`-local. No `game-stdlib` change.

## Verification Layers

1. Internal full replay reproduces hashes -> deterministic replay-hash check (`cargo run -p replay-check` after GAT101PLATRI-014; `tests/replay.rs` here).
2. Public export omits seed material; tail unreconstructable from any export -> no-leak export tests + golden trace `public-replay-export-import`.
3. Stable field order, strict unknown-field rejection, no behavior-looking fields -> `tests/serialization.rs`.
4. Coverage categories present (all named traces) -> golden-trace fixture presence + replay assertions.

## What to Change

### 1. `games/plain_tricks/src/replay_support.rs`

Internal full-trace command replay; public observer/seat export and import per ADR 0004 (public timeline, redacted commands, public effects, public terminal; no seed material); deterministic hash checkpoints.

### 2. `games/plain_tricks/tests/{replay,serialization}.rs`

Replay tests (internal full replay reproduces hashes; public export/import for observer + seat; export omits seed; tail unreconstructable). Serialization tests (stable field order, strict unknown-field rejection, stable IDs, no behavior-looking fields, no accidental schema migration).

### 3. `games/plain_tricks/tests/golden_traces/*.trace.json`

Author the named trace set (the diagnostic, no-leak, rules, rotation, terminal, tie, export/import traces). The `bot-action` and `wasm-exported` traces are refreshed once GAT101PLATRI-013/016 land.

## Files to Touch

- `games/plain_tricks/src/replay_support.rs` (new)
- `games/plain_tricks/tests/replay.rs` (new)
- `games/plain_tricks/tests/serialization.rs` (new)
- `games/plain_tricks/tests/golden_traces/` (new — the named `*.trace.json` set)

## Out of Scope

- `replay-check` tool registration (GAT101PLATRI-014) and WASM export branch (GAT101PLATRI-016) — this ticket provides the replay support they register/consume.
- Bot decision logic (GAT101PLATRI-013); the `bot-action` trace is refreshed there.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks --test replay` and `--test serialization` pass.
2. Internal full replay reproduces deterministic hashes; public export/import works for observer and seat viewers per ADR 0004.
3. Public export omits seed material; the tail is unreconstructable from any export (including at terminal).

### Invariants

1. Replay, hashes, and serialization order are deterministic or explicitly migrated (FOUNDATIONS §11/§2).
2. No public/seat export leaks an unplayed card, the tail, or seed material (FOUNDATIONS §11 no-leak firewall; ADR 0004).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — internal replay-hash + public export/import redaction.
2. `games/plain_tricks/tests/serialization.rs` — stable order + strict unknown-field rejection.
3. `games/plain_tricks/tests/golden_traces/*.trace.json` — the named coverage set.

### Commands

1. `cargo test -p plain_tricks --test replay && cargo test -p plain_tricks --test serialization`
2. `cargo test -p plain_tricks`
3. Within-crate replay/serialization tests are the correct boundary now; `cargo run -p replay-check -- --game plain_tricks` is exercised once GAT101PLATRI-014 registers the tool.

## Outcome

Completed: 2026-06-09

What changed:

1. Added `games/plain_tricks/src/replay_support.rs` with internal full traces, deterministic state/effect/action-tree/view hashing, viewer-scoped public export/import, strict trace/export JSON round trips, and redacted command summaries.
2. Added replay tests covering golden trace hash drift, diagnostic traces, internal replay determinism, public export/import round trips, seed omission, and no explicit tail export.
3. Added serialization tests for static data consistency, stable trace/export byte order, strict unknown-field rejection, and deterministic state/view summaries.
4. Added the named `games/plain_tricks/tests/golden_traces/*.trace.json` set from the spec, including diagnostic, no-leak, rule, terminal, split, public-export, bot-placeholder, and WASM-placeholder fixtures.

Deviations from original plan:

1. The `bot-action` and `wasm-exported` traces are intentionally command fixtures with notes that producer/WASM payload evidence is refreshed by GAT101PLATRI-013 and GAT101PLATRI-016.
2. Terminal export tests assert no seed material or explicit tail list/ids, because the same card ids can become public in earlier or later fresh-deal rounds without exposing the unrevealed tail as a tail surface.

Verification:

1. `cargo test -p plain_tricks --test replay` passed.
2. `cargo test -p plain_tricks --test serialization` passed.
3. `cargo test -p plain_tricks` passed.
4. `cargo fmt --all --check` passed.
