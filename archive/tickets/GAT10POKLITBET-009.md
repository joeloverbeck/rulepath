# GAT10POKLITBET-009: Replay support, serialization tests, and golden traces

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/poker_lite/src/replay_support.rs`, `games/poker_lite/tests/replay.rs`, `games/poker_lite/tests/serialization.rs`, and the `games/poker_lite/tests/golden_traces/*.json` set. Consumes `engine-core` replay/checkpoint/hash + ADR 0004 export taxonomy. No kernel change.
**Deps**: GAT10POKLITBET-008

## Problem

Deterministic replay and viewer-safe public export are core official-game evidence. `poker_lite` needs internal full-trace replay (with hash checkpoints), public observer/seat export+import per the ADR 0004 viewer-scoped taxonomy, strict serialization, and the named golden-trace set proving deal/reveal/showdown/yield/tie/diagnostic/export coverage — with public exports that cannot reconstruct hidden private cards.

## Assumption Reassessment (2026-06-08)

1. The replay-support pattern matches `games/secret_draft/src/replay_support.rs` + `tests/replay.rs` + `tests/serialization.rs` + `tests/golden_traces/`. Verified `engine-core` exposes `ReplayRecord`, `Checkpoint`, `HashValue`, `HashSurface`, `ViewHash`, `EffectLog` (`crates/engine-core/src/replay.rs`). `secret_draft`'s golden-trace set (14 traces incl. `public-replay-export-import.trace.json`, `public-observer-no-leak.trace.json`, `wasm-exported.trace.json`) is the layout precedent.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 golden-trace list, §D replay/export model, §7 Replay + Serialization tests) names the trace set and the export rules: internal full trace may include seed/private deal/hidden center/deck tail (tests only); public export follows ADR 0004 (public timeline, redacted commands, public effects/terminal, no seed material that reconstructs private cards); seat-scoped export may include that seat's own observations only.
3. Cross-artifact boundary under audit: `engine-core`'s replay/hash/serialization contract (`docs/TESTING-REPLAY-BENCHMARKING.md`) and **ADR 0004** (`docs/adr/0004-hidden-info-replay-export-taxonomy.md`, Status: Accepted — verified this session). This ticket owns all golden traces **except** `bot-action.trace.json` (GAT10POKLITBET-010) and `wasm-exported.trace.json` (GAT10POKLITBET-014).
4. FOUNDATIONS §11 (replay/hash/serialization deterministic) and §13 (replay/hash semantics gated by ADR — satisfied by the accepted ADR 0004) motivate this ticket. Restated before trusting the spec narrative.
5. Replay-determinism + no-leak-export surface under audit (§11/§13): internal replay must reproduce hashes byte-identically for `(seed, version)`; serialization must be stable-ordered with strict unknown-field rejection and no behavior-looking trace fields; public export for pre-showdown/yield terminal MUST NOT include seed material capable of reconstructing private cards, and folded private cards must remain unreconstructable from a public export. This is the export no-leak surface (ADR 0004 taxonomy).

## Architecture Check

1. Separating an internal full trace (seed + hidden state, for deterministic tests) from a viewer-scoped public export (no reconstructable secrets) is exactly the ADR 0004 model — it preserves replay determinism without a leak path. Matches `high_card_duel`/`secret_draft`.
2. No backwards-compatibility aliasing/shims — new module + new traces.
3. `engine-core` stays noun-free (replay support is crate-local over the generic replay contract, §3); no `game-stdlib` promotion (§4); no replay/hash *semantic* change (ADR 0004 already governs the taxonomy, so no new ADR trigger, §13).

## Verification Layers

1. Internal replay determinism (full trace reproduces hashes) -> `cargo test -p poker_lite --test replay` hash-checkpoint assertions + `cargo run -p replay-check -- --game poker_lite` (after GAT10POKLITBET-012 registers it).
2. Serialization stability (stable field order; strict unknown-field rejection; no behavior-looking fields) -> `cargo test -p poker_lite --test serialization`.
3. Public-export no-leak (export omits seed material reconstructing privates; folded cards unreconstructable) -> export/import redaction tests + `public-replay-export-import` / `public-observer-no-leak` golden traces.
4. Golden-trace coverage (deal/reveal/showdown/yield/tie/diagnostic categories present) -> `tests/golden_traces/*.json` deterministic-replay assertions.

## What to Change

### 1. `games/poker_lite/src/replay_support.rs`

Internal full-trace command replay; deterministic hash checkpoints; public observer/seat export + import per ADR 0004 (redacted command summaries, public timeline). Public export omits any seed material that would reconstruct private cards; seat export includes only that seat's own observations.

### 2. `games/poker_lite/tests/replay.rs` + `tests/serialization.rs`

Internal replay reproduces hashes; public export/import for observer and seat viewers; stable serialization order, strict unknown-field rejection, no behavior-looking fields, no accidental schema migration.

### 3. `games/poker_lite/tests/golden_traces/*.json`

Author the trace set (all from spec §4 **except** `bot-action` and `wasm-exported`): `deal-private-no-leak`, `hold-hold-center-reveal`, `press-match-showdown-reveal`, `lift-match-showdown`, `yield-terminal-no-showdown`, `pair-beats-high-card`, `high-card-showdown`, `tie-split`, `no-leak-public-observer`, `seat-private-view`, `invalid-wrong-seat-diagnostic`, `invalid-stale-diagnostic`, `invalid-lift-cap-diagnostic`, `invalid-private-card-redacted`, `public-replay-export-import`.

## Files to Touch

- `games/poker_lite/src/replay_support.rs` (new)
- `games/poker_lite/tests/replay.rs` (new)
- `games/poker_lite/tests/serialization.rs` (new)
- `games/poker_lite/tests/golden_traces/deal-private-no-leak.trace.json` (new)
- `games/poker_lite/tests/golden_traces/hold-hold-center-reveal.trace.json` (new)
- `games/poker_lite/tests/golden_traces/press-match-showdown-reveal.trace.json` (new)
- `games/poker_lite/tests/golden_traces/lift-match-showdown.trace.json` (new)
- `games/poker_lite/tests/golden_traces/yield-terminal-no-showdown.trace.json` (new)
- `games/poker_lite/tests/golden_traces/pair-beats-high-card.trace.json` (new)
- `games/poker_lite/tests/golden_traces/high-card-showdown.trace.json` (new)
- `games/poker_lite/tests/golden_traces/tie-split.trace.json` (new)
- `games/poker_lite/tests/golden_traces/no-leak-public-observer.trace.json` (new)
- `games/poker_lite/tests/golden_traces/seat-private-view.trace.json` (new)
- `games/poker_lite/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json` (new)
- `games/poker_lite/tests/golden_traces/invalid-stale-diagnostic.trace.json` (new)
- `games/poker_lite/tests/golden_traces/invalid-lift-cap-diagnostic.trace.json` (new)
- `games/poker_lite/tests/golden_traces/invalid-private-card-redacted.trace.json` (new)
- `games/poker_lite/tests/golden_traces/public-replay-export-import.trace.json` (new)
- `games/poker_lite/src/lib.rs` (modify — add `mod replay_support;` + re-exports)

## Out of Scope

- `bot-action.trace.json` (GAT10POKLITBET-010) and `wasm-exported.trace.json` (GAT10POKLITBET-014).
- `replay-check` tool registration (GAT10POKLITBET-012) — this ticket authors traces; the tool arm that scans them lands there.
- Benchmarks (GAT10POKLITBET-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test replay` — internal replay reproduces hashes; public export/import works for observer and seat.
2. `cargo test -p poker_lite --test serialization` — stable order, strict unknown-field rejection, no behavior-looking fields.
3. Public-export no-leak test: a pre-showdown / yield-terminal public export cannot reconstruct either hidden private card.

### Invariants

1. Internal replay is byte-deterministic for `(seed, rules version)`; serialization order is stable (§11).
2. No public export contains seed material or fields that reconstruct a hidden/folded private card (§11 no-leak; ADR 0004).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/replay.rs` — internal replay-hash + public export/import.
2. `games/poker_lite/tests/serialization.rs` — stable order + strict rejection.
3. `games/poker_lite/tests/golden_traces/*.json` — the 15 traces above, each a deterministic-replay fixture.

### Commands

1. `cargo test -p poker_lite --test replay`
2. `cargo test -p poker_lite --test serialization`
3. `cargo run -p replay-check -- --game poker_lite` — full golden-trace scan (passes once GAT10POKLITBET-012 registers the game in `replay-check`; `--all` is the default mode).

## Outcome

Completed on 2026-06-09.

Changed:

- Added crate-local replay support for internal traces, deterministic hash surfaces, viewer-scoped public export/import, and strict stable JSON round trips.
- Added replay and serialization integration tests.
- Added the 15 GAT10POKLITBET-009 golden traces, excluding `bot-action` and `wasm-exported` as reserved for later tickets.

Deviations:

- `cargo run -p replay-check -- --game poker_lite` remains deferred to GAT10POKLITBET-012, which owns registering `poker_lite` in the tool.

Verification:

- `cargo fmt --all --check`
- `cargo test -p poker_lite --test replay`
- `cargo test -p poker_lite --test serialization`
- `cargo test -p poker_lite`
