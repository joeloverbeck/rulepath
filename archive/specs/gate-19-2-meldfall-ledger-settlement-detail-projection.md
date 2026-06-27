# Gate 19.2 Implementation Spec — Meldfall Ledger settlement-detail projection

> **Spec path:** `specs/gate-19-2-meldfall-ledger-settlement-detail-projection.md`
> **Deliverable type:** roadmap-gate follow-on / completion spec, not ticket decomposition.

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `GATE-19-2-MELDFALL-LEDGER-SETTLEMENT-DETAIL-PROJECTION` |
| Stage | Public scaling phase, Gate 19 follow-on (19.2) |
| Gate | Gate 19 — Five Hundred Rummy / Rummy 500 family (presentation-completeness follow-on) |
| Status | `Done` |
| Date | 2026-06-26 |
| Owner | Rulepath maintainers |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs → `docs/ROADMAP.md` → `games/meldfall_ledger/docs/RULES.md` → this spec → future tickets. Accepted ADRs supersede only the sections they name. |
| Internal game id | `meldfall_ledger` |
| Variant id | `classic_500_single_deck_v1` (unchanged) |
| Rules version | `meldfall-ledger-rules-v1` (unchanged — no rule text changes) |
| Kernel stance | Consumes the foundation set. **No `engine-core` change.** No new shared helper, no `engine-core` noun. |
| ADR stance | The breakdown values are already authorized as public by `ML-VIS-006`, so this is **not** a new visibility contract. However, the chosen carrier may touch replay/effect serialization, which is an ADR trigger (FOUNDATIONS §13 "changing replay/hash semantics") and the `ML-REPLAY-003` "no trace schema migration" note. §6 pins the carrier so the default path needs **no ADR**; if implementation cannot avoid changing effect/trace serialization or hashes, stop and open an ADR before proceeding. |

---

## 2. Objective

Gate 19 + 19.1 shipped Meldfall Ledger fully playable to a 500-target terminal,
and the web renderer presents the live board, a persistent "Last round settled"
panel (round number, per-seat round delta, cumulative score, leader), and the
terminal outcome panel.

Playtesting the shipped web build surfaced a **presentation-completeness gap** at
round settlement. Two pieces of information that the rules already declare public
at settlement are not available to the presentation layer in a form it can show
persistently:

1. **Per-seat settlement breakdown.** `RULES.md` `ML-VIS-006` authorizes public
   exposure of "tabled-card totals, in-hand penalty totals, remaining hand
   counts, round deltas, cumulative scores, ranks, and winner flags". Rust
   already computes all of these per seat in `games/meldfall_ledger/src/scoring.rs`
   (`RoundSettlement { seats: Vec<SeatSettlement>, terminal }`, where each
   `SeatSettlement` carries `tabled_positive`, `in_hand_penalty`,
   `remaining_hand_count`, `round_delta`, `cumulative_score`, `rank`, and
   `winner`). But the only public carrier of settlement data,
   the `round_score` effect (`games/meldfall_ledger/src/effects.rs`), exposes
   **only** `round_index`, `deltas`, and `cumulative_scores`. The
   tabled-positive / in-hand-penalty split, remaining counts, ranks, and winner
   flags are dropped on the floor for non-terminal rounds. A player therefore
   cannot see *why* a delta is what it is (how many points were tabled vs lost to
   cards held) until the match reaches terminal.

2. **Round-end reason.** The reason a round ended — a seat went out
   (`go_out_without_discard` / `go_out_by_final_discard:seat=N`) or the stock was
   exhausted (`stock_exhausted`, `ML-TURN-009`) — exists only on the transient
   `round_settled` view (`MeldfallLedgerPublicView.round_end`). It is cleared the
   instant the next round is dealt (`ML-MATCH-006`). Because bot orchestration
   auto-deals the next round, this value is `null` by the time presentation can
   capture it, and no effect payload carries it. The web settlement panel can
   show the deltas but never *how* the round ended.

**Observable symptom (current build).** With Bot-vs-bot autoplay, the persistent
settlement panel reads "Round N" with no round-end reason across every settled
round, and shows only the net delta — never the tabled-vs-penalty composition the
strategy guide (`COMPETENT-PLAYER.md`) treats as core feedback. The data exists
in Rust; it simply is not projected.

**Objective.** Project the already-public round-settlement detail in a form the
presentation layer can render persistently and deterministically, so the web
settlement panel (and any other viewer) can show, per seat, the tabled-positive
total, the in-hand-penalty total, the resulting delta, the cumulative score, the
rank, and the winner flag, plus the round-end reason — without weakening any
hidden-information boundary and without changing rule text.

This is a completion of an already-accepted visibility contract (`ML-VIS-006`),
not a new game and not a rule change.

---

## 3. Scope

### 3.1 In scope

1. **A persistent `last_settlement` projection on the public view.** Add a
   Rust-owned, viewer-scoped, structured field to the Meldfall Ledger public view
   (e.g. `last_settlement: MeldfallLedgerSettlementView | null`) that holds the
   most recently settled round's public detail and **persists across the next
   round until the following settlement replaces it** (or `null` before any round
   has settled in the match). Shape, per the `ML-VIS-006` allow-list:
   - `round_index`
   - `round_end_reason` — a stable, viewer-safe enum/string mirroring the public
     `round_end` value (who went out, or stock exhausted)
   - per seat, in stable `seat_0..seat_5` order: `tabled_positive`,
     `in_hand_penalty`, `delta`, `cumulative_score`, `rank`, `winner`
2. **Determinism + replay parity.** The projected field is a pure function of
   already-recorded state; it must be identical under replay of the same command
   stream (`ML-REPLAY-001`) and must carry no wall-clock or RNG input.
3. **No-leak.** The projection exposes only `ML-VIS-006`-authorized public
   settlement facts. It must **not** include any seat's exact unmelded card
   identities, stock order, or private bot features, in any viewer (public,
   observer, or seat), per `ML-VIS-003` / `ML-VIS-006`. Seat-private views may
   continue to include the viewer's own remaining cards elsewhere, but the
   shared `last_settlement` carries totals/counts only.
4. **Web presentation.** Update `apps/web/src/components/MeldfallLedgerBoard.tsx`
   to read `view.last_settlement` directly (replacing the current effects-buffer
   capture heuristic) and render, in the existing "Last round settled" panel:
   the round-end reason, and a per-seat row showing `tabled_positive` and
   `in_hand_penalty` composing the `delta`, alongside the existing cumulative /
   leader presentation. TypeScript renders Rust-authored values only — no
   settlement math in TS (`ML-UI-001`).
5. **Verification.** Trace/fixture/coverage/no-leak updates per §7; web smoke
   (`smoke:ui`, `meldfall-ledger.smoke.mjs`, `a11y-noleak.smoke.mjs`) extended to
   assert the projected breakdown renders and that no forbidden term leaks.

### 3.2 Out of scope

- Any change to `RULES.md` rule text, the variant pin, deal counts, or scoring
  formulas. Card values and deltas are unchanged; this spec only *projects*
  already-computed, already-public results.
- Any new `engine-core` vocabulary or shared `game-stdlib` helper.
- Bot strategy changes; L1+ remains out of scope per `ML-BOT-002`.
- Exposing any seat's exact unmelded cards at settlement (explicitly forbidden by
  `ML-VIS-006`'s chosen resolution).

### 3.3 Carrier decision (avoids an ADR on the default path)

The breakdown is carried on the **view projection** (`last_settlement`), not by
widening the `round_score` effect payload or the Trace Schema v1 record. Rationale:

- Views are recomputed viewer projections; adding a derived field does not change
  the accepted command stream, the effect stream, or replay **hashes**, so it
  avoids the FOUNDATIONS §13 "changing replay/hash semantics" trigger and the
  `ML-REPLAY-003` "no trace schema migration is authorized" constraint.
- It is strictly additive and viewer-scoped, matching how other Rust-owned public
  facts already reach the renderer.

**Hard gate.** Before implementation, confirm that the view projection is **not**
an input to any persisted replay/outcome hash. The retained snapshot lands in
`MatchState` (it must survive the `ML-MATCH-006` round reset; see §4/§5), and
`MatchState::stable_internal_summary()` is the input to the replay record's
`state_hash` (`games/meldfall_ledger/src/replay_support.rs` lines 67/95).
Therefore the retained `last_settlement` snapshot **must be excluded from
`stable_internal_summary()`** — it is a pure derived function of already-hashed
cumulative state, so excluding it keeps `state_hash` and replay parity
byte-identical. If the cleanest implementation instead requires folding it into
the stable summary, widening the `round_score` effect payload, or widening the
trace schema — **stop and open an ADR** (replay/hash semantics) and a
`ML-REPLAY-003` migration note before proceeding. Do not silently change
serialization.

---

## 4. Affected surfaces (survey, verify before editing)

- `games/meldfall_ledger/src/scoring.rs` — source of the per-seat
  `SeatSettlement` values (inside `RoundSettlement`; already computed). Its
  `SeatSettlement::stable_public_string()` already emits the `ML-VIS-006`
  public encoding and is the canonical encoding to reuse.
- `games/meldfall_ledger/src/state.rs` — needs to retain the last settlement
  snapshot across the `ML-MATCH-006` transition (round-only state is cleared;
  the settlement snapshot must survive into the next round until replaced).
- `games/meldfall_ledger/src/visibility.rs` — public view projection; add the
  viewer-safe `last_settlement` field.
- `crates/wasm-api/src/games/meldfall.rs` — bridge the new field to JSON.
- `apps/web/src/wasm/client.ts` — add the `MeldfallLedgerSettlementView` type and
  the `last_settlement` field on `MeldfallLedgerPublicView`.
- `apps/web/src/components/MeldfallLedgerBoard.tsx` — render from
  `view.last_settlement`; retire the effects-buffer `RoundSettlement` capture.
- `games/meldfall_ledger/docs/RULE-COVERAGE.md`, `UI.md`, `GAME-EVIDENCE.md` —
  record the projection + evidence.
- Trace fixtures / `fixture-check` / `rule-coverage` / `replay-check` — confirm
  parity; update fixtures only if and as the projection requires (no schema
  migration without an ADR per §3.3).

---

## 5. Determinism, replay, and no-leak requirements

1. **Determinism.** `last_settlement` is a pure function of settled-round state;
   identical across runs and replays of the same seed/seat-count/variant/rules/
   data versions (`ML-REPLAY-001`, `ML-SETUP-006`). The retained snapshot is
   excluded from `MatchState::stable_internal_summary()`, so the replay
   `state_hash` (`replay_support.rs`) is byte-identical before and after this
   change (see the §3.3 hard gate).
2. **Replay parity.** Public and seat-private exports never elevate privilege on
   import (`ML-REPLAY-002`); the projected field is recomputed, never trusted from
   an imported blob.
3. **No-leak.** Browser no-leak proof (`ML-UI-003`): DOM text, a11y names,
   `data-testid`, storage, console logs, and effect text must not contain any
   unauthorized hidden card identity or stock order as a result of this field.
   The web `a11y-noleak` and `meldfall-ledger` smokes must continue to pass with
   the new panel content.

---

## 6. Deliverables

1. Rust `last_settlement` projection (state retention + visibility) with unit
   tests covering: go-out-by-final-discard, go-out-without-discard, stock
   exhaustion, multi-round persistence across `ML-MATCH-006`, and tie
   continuation. Reuse `SeatSettlement::stable_public_string()` (`scoring.rs`)
   as the canonical `ML-VIS-006` public encoding rather than introducing a
   parallel one.
2. WASM bridge + `client.ts` types.
3. Web settlement panel rendering the per-seat tabled/penalty breakdown and
   round-end reason; retire the effects-buffer capture heuristic.
4. No-leak invariance tests (opponent-hand mutation does not change any public
   `last_settlement` field).
5. Doc updates: `RULE-COVERAGE.md` (map to `ML-VIS-006`, `ML-SCORE-*`),
   `UI.md` (settlement surface), `GAME-EVIDENCE.md`.
6. `specs/README.md` row for Gate 19.2 **already exists** (`specs/README.md`
   line 110, Status `Planned`); at completion, flip its Status to `Done`
   rather than authoring a new row.

---

## 7. Verification (CI gates)

Gate 0 (Rust hygiene), Gate 1 (per-game + web), Gate 2 (benchmarks unaffected):

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p simulate      -- --game meldfall_ledger --games 1000
cargo run -p replay-check  -- --game meldfall_ledger --all
cargo run -p fixture-check -- --game meldfall_ledger
cargo run -p rule-coverage -- --game meldfall_ledger
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
node scripts/check-catalog-docs.mjs
```

Plus targeted web e2e (`meldfall-ledger.smoke.mjs`, `a11y-noleak.smoke.mjs`)
asserting the breakdown renders and no forbidden term leaks.

## 8. Acceptance criteria

- The web "Last round settled" panel shows, persistently across the next round,
  the round-end reason and each seat's tabled-positive, in-hand-penalty, delta,
  cumulative score, and rank — all sourced from `view.last_settlement`, with no
  settlement math in TypeScript.
- All Gate 0/1 commands above pass; no replay/hash change (or, if unavoidable, an
  accepted ADR + migration note is linked here first).
- No hidden-information leak in any viewer; opponent-hand mutation leaves every
  public `last_settlement` field unchanged.
- `RULES.md` is unchanged; `RULE-COVERAGE.md` maps the projection to `ML-VIS-006`
  and `ML-SCORE-*`.

## 9. Provenance

Authored from web-app playtesting of the shipped Meldfall Ledger build
(2026-06-26), during a UI-deficiency review loop that landed seven presentation
improvements (commits `f48247e`..`c288452`: card values + in-hand penalty +
target, persistent settlement panel, discard-pickup commitment surfacing, hand
sort toggle, action-label glyphs, near-go-out flag, round-delta clarification).
The settlement-detail and round-end-reason gaps were the only deficiencies found
that cannot be closed in presentation alone, because the data is computed in Rust
but not projected — hence this spec.

## 10. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | Rust owns the `last_settlement` projection; §3.1.4 forbids settlement math in TypeScript (`ML-UI-001`). TypeScript renders Rust-authored totals only. |
| §3 `engine-core` is a contract kernel | aligned | No `engine-core` change; all new types are game-local in `games/meldfall_ledger` (§1 Kernel stance). No mechanic noun enters the kernel. |
| §4 `game-stdlib` is earned | aligned | No helper promotion; no new shared `game-stdlib` surface (§3.2). |
| §5 Static data is not behavior | aligned | No static-data change; the projection is computed Rust state, not authored data. |
| §11 Universal acceptance invariants | aligned | Viewer-safe totals/counts only (`ML-VIS-006`); no-leak proven by the `a11y-noleak` / `meldfall-ledger` smokes (§5, §7); determinism preserved by excluding the retained snapshot from `state_hash` (§3.3, §5). |
| §13 ADR triggers | aligned | Replay/hash semantics unchanged by the pinned view carrier (§3.3); a hard gate stops and opens an ADR if serialization must change. |
| §12 Stop conditions | clear | No stop condition crossed: no kernel noun, no TypeScript legality, no hidden-information leak, no replay/hash change on the default path. |

## Outcome

Completed: 2026-06-27

Gate 19.2 closed through tickets `GAT192MELLEDSET-001` through
`GAT192MELLEDSET-004`.

- Rust now retains and projects a nullable `last_settlement` snapshot containing
  round index, round-end reason, tabled-positive totals, in-hand penalties,
  held-card counts, round deltas, cumulative scores, ranks, and winner flags.
- The retained snapshot is excluded from `MatchState::stable_internal_summary()`,
  preserving replay/hash semantics and avoiding any trace schema migration or
  ADR.
- WASM and TypeScript expose the field as nullable public view JSON.
- The web board renders the persistent settlement breakdown directly from
  `view.last_settlement`; the former effects-buffer settlement capture was
  removed.
- `RULES.md` was intentionally unchanged. `RULE-COVERAGE.md`, `UI.md`,
  `GAME-EVIDENCE.md`, and `specs/README.md` record the closeout and evidence.

Verification:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo run -p simulate -- --game meldfall_ledger --games 1000 --action-cap 20000`
- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo run -p fixture-check -- --game meldfall_ledger`
- `cargo run -p rule-coverage -- --game meldfall_ledger`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `node scripts/check-catalog-docs.mjs`
- `node apps/web/e2e/meldfall-ledger.smoke.mjs`
- `node apps/web/e2e/a11y-noleak.smoke.mjs`

The simulator run used an explicit `--action-cap 20000` as a bounded
multi-round verifier guard. No hidden-information leak, TypeScript legality
move, rule-text change, or replay/hash change was required.

## 11. Assumptions

| # | Assumption | Correctable by |
|---|---|---|
| A-1 | The public view is a recomputed viewer projection and is **not** an input to the replay/outcome `state_hash`; only `MatchState::stable_internal_summary()` is (`replay_support.rs` lines 67/95). The retained `last_settlement` snapshot is therefore kept out of the stable summary. | Confirm at implementation per the §3.3 hard gate. |
| A-2 | `round_end` (`MeldfallView.round_end`, populated from `RoundEndSummary`) is the only round-end-reason source after bot auto-deal clears round state (`ML-MATCH-006`); the projection snapshots it at settlement. | Verify the snapshot is taken before the next deal. |
| A-3 | `SeatSettlement` (`scoring.rs`) already carries every `ML-VIS-006` field; no new scoring computation is required, only retention + projection. | Grep `scoring.rs` for the `SeatSettlement` struct fields. |
