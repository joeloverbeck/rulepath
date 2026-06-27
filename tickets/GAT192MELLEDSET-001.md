# GAT192MELLEDSET-001: Rust `last_settlement` retention + visibility projection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/meldfall_ledger` (`src/state.rs`, `src/visibility.rs`, possibly `src/scoring.rs`); tests `tests/visibility.rs`, `tests/rules.rs`, `tests/serialization.rs`
**Deps**: None

## Problem

The web settlement panel can show only the net round delta, never the
tabled-positive-vs-in-hand-penalty composition or the round-end reason, because
neither is projected onto a persistent public surface. `ML-VIS-006` already
authorizes those facts as public, and `SeatSettlement`
(`games/meldfall_ledger/src/scoring.rs:36-46`) already computes every one of them
per seat — but the data is consumed transiently at settlement and dropped, and
the public `round_end` view field (`MeldfallView.round_end`, `visibility.rs:32`)
is cleared the instant the next round is dealt (`ML-MATCH-006`). This ticket adds
a Rust-owned, viewer-safe `last_settlement` projection that retains the most
recently settled round's public detail across the next round until the following
settlement replaces it (spec §3.1.1, §3.1.3).

## Assumption Reassessment (2026-06-27)

1. `SeatSettlement` (`games/meldfall_ledger/src/scoring.rs:36-46`) already carries
   `seat_index`, `tabled_positive`, `in_hand_penalty`, `remaining_hand_count`,
   `round_delta`, `cumulative_score`, `rank`, `winner`; `RoundSettlement` is the
   container `{ seats: Vec<SeatSettlement>, terminal: Option<MatchOutcome> }`
   (`scoring.rs:13-17`). `SeatSettlement::stable_public_string()` (`scoring.rs:49-61`)
   already emits the `ML-VIS-006` public encoding — reuse it as the canonical
   public encoding (spec §6.1) rather than inventing a parallel one. No scoring
   computation changes.
2. `MatchState` (`state.rs:28-36`) carries `variant, seats, base_seed,
   cumulative_scores, dealer_index, rounds_settled, round, terminal` — the
   settlement snapshot must be retained here (not on `RoundState`, which is
   reset/cleared at the `ML-MATCH-006` transition; `advance_to_next_round`,
   `rules.rs`). `round_end: Option<RoundEndSummary>` lives on `RoundState`
   (`state.rs:100`) and `RoundEndSummary::stable_string()` =
   `"{reason}:seat={seat_index}"` (`state.rs:214-215`) is the round-end-reason
   source to snapshot at settlement.
3. Cross-artifact boundary under audit: `project_view(state: &MatchState, viewer:
   &Viewer) -> MeldfallView` (`visibility.rs:188`) is the projection seam; the new
   `last_settlement` field is added to `MeldfallView` (`visibility.rs:19`) and
   filled here from the retained `MatchState` snapshot. The shared contract is the
   `ML-VIS-006` public allow-list — every projected field must be on it, and no
   field outside it (no exact unmelded card identities, no stock order) may appear.
4. FOUNDATIONS §11 no-leak firewall restated: `last_settlement` exposes only
   `ML-VIS-006`-authorized totals/counts. It must be identical in the public,
   observer, and every seat-private view, and must not include any seat's exact
   unmelded cards or stock order (`ML-VIS-003`, spec §3.3). The opponent-hand
   no-leak invariant (D4) is asserted here, not deferred.
5. §11 deterministic replay/hash surface named: the retained snapshot lands on
   `MatchState`, whose `stable_internal_summary()` (`state.rs:59`) is the input to
   the replay record's `state_hash` (`replay_support.rs:67`, fed at line 95 via
   `state_summary: state.stable_internal_summary()`). The retained `last_settlement`
   snapshot **MUST be excluded from `stable_internal_summary()`** — it is a pure
   derived function of already-hashed cumulative state, so excluding it keeps
   `state_hash` and replay parity byte-identical (spec §3.3 hard gate, §5.1). If the
   cleanest implementation instead requires folding it into the stable summary,
   widening the `round_score` effect payload, or widening the trace schema — stop
   and open an ADR (replay/hash semantics) + a `ML-REPLAY-003` migration note
   before proceeding; do not silently change serialization.
6. Schema extension: `MeldfallView` gains `last_settlement:
   Option<MeldfallSettlementView>` (additive, nullable, `None` before any round
   settles). Consumers of `MeldfallView`: the WASM bridge `meldfall_view_json`
   (`crates/wasm-api/src/games/meldfall.rs:364`) — updated in GAT192MELLEDSET-002,
   not here. The extension is additive-only.

## Architecture Check

1. Retaining the settlement snapshot on `MatchState` and projecting it through the
   existing `project_view` seam is cleaner than re-deriving it in the renderer
   (which would move settlement math into TypeScript, violating §2/`ML-UI-001`) or
   widening the effect/trace stream (which would trip the §13 replay/hash ADR
   trigger). The view is a recomputed viewer projection, so the addition changes no
   accepted command stream, effect stream, or replay hash.
2. No backwards-compatibility shim: the field is additive and nullable; existing
   round-0-before-settlement behavior is `None`, and existing traces/hashes stay
   byte-identical because the snapshot is excluded from `stable_internal_summary()`.
3. `engine-core` is untouched (no kernel noun); no `game-stdlib` promotion — the
   projection is a game-local type in `games/meldfall_ledger`. The settlement
   breakdown is the declared first-use local primitive, not a third use.

## Verification Layers

1. Determinism: identical `(seed, seat_count, variant, rules)` ⇒ identical
   `last_settlement` across two runs/replays -> unit test comparing the projected
   field across a replayed command stream (`tests/rules.rs` or `tests/visibility.rs`).
2. `state_hash` invariance: the retained snapshot does not enter
   `stable_internal_summary()` -> `tests/serialization.rs` assertion that
   `stable_internal_summary()` is byte-identical before/after a settlement is
   retained (and existing `replay-check` stays green).
3. No-leak: opponent-hand mutation leaves every public `last_settlement` field
   unchanged; the field is identical in public/observer/seat views -> no-leak
   visibility test in `tests/visibility.rs`.
4. Persistence across `ML-MATCH-006`: `last_settlement` survives the next deal and
   is replaced at the following settlement -> multi-round unit test in `tests/rules.rs`.
5. Round-end-reason fidelity: the projected reason mirrors the public `round_end`
   for go-out-by-final-discard, go-out-without-discard, and stock exhaustion ->
   per-scenario unit assertions.

## What to Change

### 1. Settlement snapshot retention (`state.rs`)

Add a retained field to `MatchState` holding the most recent settled round's
public detail (round index, round-end reason, per-seat `SeatSettlement` public
facts) — populated when a round settles, preserved across the
`advance_to_next_round` transition, and replaced at the next settlement. The field
MUST NOT be folded into `MatchState::stable_internal_summary()` (per Assumption 5).

### 2. `MeldfallSettlementView` + projection (`visibility.rs`)

Add a viewer-safe `MeldfallSettlementView` struct mirroring the `ML-VIS-006`
allow-list (`round_index`, `round_end_reason`, and per seat in stable
`seat_0..seat_5` order: `tabled_positive`, `in_hand_penalty`, `delta`,
`cumulative_score`, `rank`, `winner`). Add `last_settlement:
Option<MeldfallSettlementView>` to `MeldfallView` and fill it in `project_view`
from the retained `MatchState` snapshot. Reuse `SeatSettlement::stable_public_string()`
as the canonical public encoding where a stable string is needed.

### 3. Scenario + no-leak + persistence tests

Author the Verification-Layers matrix: go-out-by-final-discard,
go-out-without-discard, stock exhaustion, multi-round persistence across
`ML-MATCH-006`, tie continuation, determinism (replay), `state_hash` invariance,
and the opponent-hand-mutation no-leak invariant.

## Files to Touch

- `games/meldfall_ledger/src/state.rs` (modify) — `MatchState` retention field; keep out of `stable_internal_summary()`
- `games/meldfall_ledger/src/visibility.rs` (modify) — `MeldfallSettlementView`, `last_settlement` on `MeldfallView`, fill in `project_view`
- `games/meldfall_ledger/src/scoring.rs` (modify) — only if a small public-encoding accessor is needed; otherwise reuse `SeatSettlement::stable_public_string()` unchanged
- `games/meldfall_ledger/tests/visibility.rs` (modify) — no-leak invariance + view-equality tests
- `games/meldfall_ledger/tests/rules.rs` (modify) — settlement-scenario, persistence, determinism tests
- `games/meldfall_ledger/tests/serialization.rs` (modify) — `state_hash` / `stable_internal_summary` invariance assertion

## Out of Scope

- WASM bridge + `client.ts` type — GAT192MELLEDSET-002.
- Web rendering + retiring the effects-buffer capture + web smokes — GAT192MELLEDSET-003.
- Docs reconciliation + `specs/README.md` `Done`-flip — GAT192MELLEDSET-004.
- Any `RULES.md` rule-text, variant-pin, deal-count, or scoring-formula change (spec §3.2).
- Widening the `round_score` effect payload or the trace schema (spec §3.3 — would require an ADR).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger visibility` — no-leak + view-equality tests pass.
2. `cargo test -p meldfall_ledger` — full crate green (existing settlement/replay tests unaffected).
3. `cargo run -p replay-check -- --game meldfall_ledger --all` — replay hashes byte-identical (state_hash unchanged).
4. `cargo test --workspace` — no regression.

### Invariants

1. `last_settlement` is Rust-owned, deterministic, and a pure function of settled
   state; identical inputs+versions produce an identical projection.
2. The retained snapshot is excluded from `stable_internal_summary()`; `state_hash`
   and all golden-trace hashes are byte-identical to pre-change.
3. Every projected field is on the `ML-VIS-006` allow-list; the field is identical
   in public/observer/seat views and unaffected by opponent-hand mutation.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — settlement-scenario (go-out-by-final-discard,
   go-out-without-discard, stock exhaustion), multi-round persistence across
   `ML-MATCH-006`, tie continuation, and determinism-under-replay assertions.
2. `games/meldfall_ledger/tests/visibility.rs` — opponent-hand-mutation no-leak
   invariance and public/observer/seat view-equality for `last_settlement`.
3. `games/meldfall_ledger/tests/serialization.rs` — `stable_internal_summary()` /
   `state_hash` byte-invariance across a retained settlement.

### Commands

1. `cargo test -p meldfall_ledger visibility rules serialization`
2. `cargo run -p replay-check -- --game meldfall_ledger --all`
3. `cargo test --workspace`
4. The crate-scoped test filters are the correct boundary for the projection;
   `replay-check --all` is the determinism/hash guard and `--workspace` the
   regression guard.
