# Gate 19.1 Implementation Spec — Meldfall Ledger multi-round completion (ML-MATCH-006)

> **Spec path:** `specs/gate-19-1-meldfall-ledger-multi-round-completion.md`
> **Deliverable type:** roadmap-gate completion/remediation spec, not ticket decomposition.

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `GATE-19-1-MELDFALL-LEDGER-MULTI-ROUND-COMPLETION` |
| Stage | Public scaling phase, Gate 19 follow-on (19.1) |
| Gate | Gate 19 — Five Hundred Rummy / Rummy 500 family (completion of a shipped-deferred exit item) |
| Status | `Planned` |
| Date | 2026-06-26 |
| Owner | Rulepath maintainers |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs → `docs/ROADMAP.md` → `games/meldfall_ledger/docs/RULES.md` → this spec → future tickets. Accepted ADRs supersede only the sections they name. |
| Internal game id | `meldfall_ledger` |
| Variant id | `classic_500_single_deck_v1` (unchanged) |
| Rules version | `meldfall-ledger-rules-v1` (unchanged — no rule text changes) |
| Kernel stance | Consumes the foundation set. **No foundation amendment and no ADR expected**: this spec implements an already-accepted rule (`ML-MATCH-006`) that shipped deferred. If implementation surfaces an unavoidable contract change, stop and open an ADR per FOUNDATIONS §13 before proceeding. |
| Primitive stance | Multi-round cumulative scoring to a 500 target is already declared a Gate 19 first-use local-only primitive. This spec completes its wiring; it introduces no new shared helper and no `engine-core` noun. |

---

## 2. Objective

Gate 19 shipped Meldfall Ledger with a single-round engine. The match-level
multi-round transition required by the gate — `docs/ROADMAP.md` line 119
("multi-round target"), the archived Gate 19 spec §2.6, §6 "Round/match flow
covered" — was **intentionally deferred at ship time** and never completed. (The
archived gate's `multi-round-first-to-500.trace.json` deliverable shipped only as
a *static scoring-illustration fixture*, not an executable transition; see §7.3.)
The deferral is documented in code:

- `crates/wasm-api/src/games/meldfall.rs` → `round_score_index()` states: *"The
  multi-round transition (ML-MATCH-006) is intentionally deferred, so a match
  plays exactly one round, scored as round index 0."*
- `games/meldfall_ledger/src/bots.rs` returns an empty action tree for
  `TurnPhase::RoundSettled | TurnPhase::MatchComplete`, and no code path deals a
  subsequent round.

**Observable defect.** Because no single round produces a cumulative score ≥ 500,
every match dead-ends at `round_settled` with no legal action and no terminal:

- `cargo run -p simulate -- --game meldfall_ledger --games 3` reports
  `completion_rate_percent=0.00` with every game `bounded_nonterminal_at_cap`.
- In the browser, Bot-vs-bot autoplay stalls at "Round settled" with "No actions
  available"; a Human-vs-bot match can never be finished.

This contradicts the rule the project already accepts. `RULES.md` `ML-MATCH-006`
reads: *"A non-terminal settled round advances dealer clockwise, clears
round-only table state, deals a fresh round, and starts with the seat left of the
new dealer. Transition is deterministic and Rust-owned."*

The objective is to make Meldfall Ledger playable to a real terminal: implement
the deterministic, Rust-owned multi-round transition so cumulative play continues
across rounds until a unique seat reaches the 500 target (or ties continue per
`ML-MATCH-003`), with full determinism, no-leak, replay, and host parity.

This is a completion of existing rule authority, not a new game and not a rule
change. `RULES.md`, `MECHANICS.md`, and the variant pin are unchanged.

---

## 3. Scope

### 3.1 In scope

1. **Round-transition engine step (`ML-MATCH-006`).** A Rust-owned, deterministic
   `advance_to_next_round` operation that, when the just-settled round is
   non-terminal, performs in order:
   - increments a match-level round counter;
   - rotates the dealer clockwise: `dealer_index = next_clockwise_index(dealer_index, seat_count)` (reuse the existing helper);
   - clears all round-only state (see §3.4) while preserving `cumulative_scores`;
   - deterministically deals a fresh round via the existing `setup::deal_for_round`;
   - sets the new active seat to the seat left of the new dealer and phase to `Draw`.
2. **Match-level round identity and seed retention.** Add to `MatchState` the base
   match seed and a `round_index`/`rounds_settled` counter (both currently absent;
   `MatchState` holds only `variant, seats, cumulative_scores, dealer_index, round,
   terminal`). These feed (a) deterministic per-round deal-seed derivation and (b)
   the `round_score` effect's `round_index`.
3. **Deterministic per-round deal seed.** A documented, stable, game-crate
   derivation of the round deal seed from the base match seed and round index,
   built on the existing `engine-core` seed primitives (`Seed(u64)` /
   `SeededRng::from_seed`) — no seed-derivation helper exists in `engine-core`,
   and none is added. Round 0 continues to use the existing
   `setup_match`/`deal_for_round(seed)` path unchanged, so the first deal stays
   byte-identical to the shipped single-round setup; the derivation governs
   rounds 1+ only. Seat count, variant, rules version, and data version are fixed
   variant pins (the determinism guarantee of `ML-SETUP-006`), not new RNG inputs
   to fold in. The function must be pure and replay-stable; once evidence exists
   it is effectively frozen.
4. **`round_score_index` correction.** Replace the hardcoded `0` with the settled
   round count so the `RoundScore` effect reports the true round index.
5. **Apply-path wiring (both hosts).** The round transition is the game-crate
   `advance_to_next_round` operation (§3.1.1), invoked by **both** the WASM bridge
   (`crates/wasm-api/src/games/meldfall.rs`) and the `tools/simulate` loop — the
   two have independent apply paths (simulate does not route through the WASM
   bridge; it currently returns at `RoundSettled`), so both must call the shared
   operation for host parity. In the WASM bridge, after `settle_round` when
   `settlement.terminal` is `None`, perform the transition in the same apply
   transaction and emit a public round-transition effect. The effect mirrors the
   **field shape** used by other games' `refill_started` (`next_round_number`,
   `next_lead_seat`, plus the new dealer seat) but MUST use a distinct,
   meldfall-owned effect `kind` — `next_round_dealt` — **not** `refill_started`:
   the web `describeEffect` switch keys on `payload.type ?? payload.kind`, and
   `high_card_duel` already owns `refill_started` there, so reusing that string
   would collide with high_card_duel's case rather than add a new one. The new
   round's action tree must be produced for the new active seat so play continues
   without any player "advance" action.
6. **Simulate host parity.** `tools/simulate` must drive matches to terminal:
   `completion_rate_percent` becomes > 0 for ordinary seeds, and
   `wins_by_seat` records real winners. The simulate loop and the WASM apply path
   must produce identical state/score/hash sequences for the same seed.
7. **Effect feedback (web).** `apps/web/src/components/effectFeedback.ts` gains a
   new `next_round_dealt` case (distinct from the existing `refill_started` case,
   which belongs to `high_card_duel`), using friendly seat labels (consistent
   with the existing `meldfallSeatLabel` helper), e.g. "Round 2 dealt — Seat 3
   deals; Seat 4 leads off." The existing enriched `round_score` settlement
   summary remains.
8. **Web continuation.** Confirm the board advances out of `round_settled` into the
   next round automatically and that autoplay now reaches the terminal outcome
   panel. The `round_settled` heading/turn-pill copy added in the loop work
   remains valid as a transient state.
9. **Determinism, no-leak, replay across rounds.** Stock order, opponent hands,
   and private draws stay hidden across the re-deal; viewer-scoped exports and
   replay remain deterministic and hash-stable across multiple rounds.

### 3.2 Out of scope

- Any change to legal-move, meld, lay-off, pickup-commitment, or single-round
  scoring semantics (those shipped and pass their gates).
- Any change to `RULES.md` rule **text** (`ML-MATCH-006` already exists). Only
  cross-reference/version-checked notes in non-authoritative docs may be updated.
- L1/L2 bot strategy work (the L0 random-legal bot is the simulation baseline).
- Variant changes (still `classic_500_single_deck_v1`, single 52-card deck).
- Any new `engine-core` noun, shared `game-stdlib` helper, or helper promotion.
- A redesigned terminal outcome panel (the existing one is reused as-is).

### 3.3 Not allowed

- TypeScript deciding round-transition legality, dealer rotation, the re-deal,
  terminal evaluation, or scoring. The transition is Rust-owned; TS presents only.
- Non-deterministic re-deals (wall-clock, browser RNG, unseeded shuffles).
- Reshuffling the discard pile into a new stock mid-round (this variant does not;
  `ML-TURN-009` stands). The re-deal is a fresh full-deck deal for the new round.
- Leaking stock order / opponent hands through the transition effect, DOM,
  storage, logs, or replay export.
- Weakening or deleting existing tests/golden traces to absorb the new flow;
  follow the AGENT-DISCIPLINE §4 failing-test protocol.
- Regenerating unrelated golden/fixture/hash artifacts; only Meldfall Ledger
  artifacts that legitimately change (new multi-round traces, completion profile)
  may move, governed by ADR 0009.

### 3.4 Round-only state that the transition MUST reset vs. preserve

| State | On round transition | Notes |
|---|---|---|
| `cumulative_scores` | **preserve** (carry forward) | Match-level ledger. |
| `dealer_index` | **rotate** clockwise | `ML-MATCH-006`, `ML-SETUP-005`. |
| round counter / base seed | **advance / retain** | New match-level fields. |
| `round.stock` | **re-deal** | Fresh hidden stock from the new deterministic deal. |
| `round.discard` | **reset** to the new single initial discard | `ML-SETUP-004`. |
| `round.tableau` (meld groups) | **clear** | Public melds are round-only. |
| each seat `hand` | **re-deal** | New private hands (13 for 2 seats, else 7). |
| `round_played_scores` | **reset to 0** | Per-round tabled credit. |
| `pending_pickup` | **clear** | No commitment carries across rounds. |
| `round_end` summary | **clear** | Belongs to the prior round. |
| `phase` / `active_seat_index` | **set** to `Draw` / seat left of new dealer | `ML-SETUP-005`. |
| meld id counter | **define explicitly** | Decide and document whether meld ids reset per round or remain monotonic per match; pick the option that keeps trace hashes and the tableau view unambiguous, and assert it in a test. |

---

## 4. Deliverables

| Area | Deliverable |
|---|---|
| Engine state | `MatchState` gains base-seed + round-index fields; constructors/threading updated; `stable_internal_summary` includes the round index — this shifts the canonical summary string for all states incl. round 0, so the `starts_with` assertion in `games/meldfall_ledger/tests/serialization.rs` is updated to match (an ADR-0009-governed Meldfall artifact movement, not test-weakening). |
| Engine logic | `advance_to_next_round` (deterministic, Rust-owned) + per-round seed derivation; reuse `deal_for_round`, `next_clockwise_index`. |
| Effects | New public `MeldfallEffect::NextRoundDealt` (`kind` `next_round_dealt`, distinct from `high_card_duel`'s `refill_started`) with stable string + JSON encoding and viewer projection; `round_score_index` corrected. |
| WASM bridge | `crates/wasm-api/src/games/meldfall.rs` apply path drives settle → (terminal? finish : `advance_to_next_round`) and emits the new effect; new round's action tree served. |
| Simulate | `tools/simulate` (independent apply loop, currently returns at `RoundSettled`) calls the same `advance_to_next_round` and drives to terminal; summary shows real completion and winners. |
| Web | `effectFeedback.ts` round-transition case; verify autoplay reaches the terminal panel; no `round_settled` dead-end. |
| Tests | New/updated Rust tests (§7.2), transition-evidence artifact(s) (§7.3), no-leak matrix across rounds, host-parity test. |
| Docs | Update non-authoritative docs (§10) to record completion; remove/supersede the "intentionally deferred" code comment; `GAME-EVIDENCE.md` receipt; `specs/README.md` row. |

---

## 5. Work breakdown (suggested ticket seams)

1. **State + seed plumbing.** Add base-seed/round-index to `MatchState`; thread
   from `setup_match`/`InitialSetup`; update internal summaries and serialization.
2. **Transition logic + deterministic re-deal seed.** Implement
   `advance_to_next_round` and the seed derivation; unit-test reset/preserve,
   dealer rotation, active-seat, determinism, and meld-id policy.
3. **Round-transition effect.** Add the `MeldfallEffect::NextRoundDealt` variant
   (`kind` `next_round_dealt` — not `refill_started`), stable/JSON encodings,
   viewer projection, and `round_score_index` fix; effect smoke.
4. **WASM apply wiring + simulate parity.** Drive settle→transition/terminal in
   both hosts; add the host-parity determinism test; confirm simulate completion.
5. **Web feedback + browser verification.** `effectFeedback.ts` case; Puppeteer
   verification that a full match reaches the terminal outcome panel.
6. **Evidence + docs + closeout.** Author the new transition-evidence artifact(s)
   (§7.3) — leaving the existing scoring-illustration fixtures intact; update
   docs/evidence; flip this spec and the `specs/README.md` note.

Each seam is one reviewable diff with its own tests; decompose via
`templates/AGENT-TASK.md` using the `spec-to-tickets` skill.

---

## 6. Exit criteria

| # | Criterion | Evidence |
|---|---|---|
| 1 | A non-terminal settled round deterministically deals the next round (dealer rotated, round-only state cleared, active seat left of new dealer). | Round-transition unit tests; golden trace. |
| 2 | Cumulative play reaches a real terminal: a unique seat ≥ 500 wins; ties at/above 500 continue (`ML-MATCH-002/003/004`). | Host-parity full-match test reaching a unique-500 terminal and a tie-continuation; `round-transition-resets-table-state` trace; terminal standings test. |
| 3 | `simulate` completes ordinary matches. | `cargo run -p simulate -- --game meldfall_ledger --games 1000` shows `completion_rate_percent` > 0 and populated `wins_by_seat`. |
| 4 | WASM apply path and simulate produce identical deterministic state/score/hash sequences for a seed across multiple rounds. | Host-parity determinism test (the authoritative parity check — `replay-check` validates meldfall traces declaratively and does not compare cross-host hash sequences); `replay-check --all` green incl. any new trace. |
| 5 | No hidden-info leak across the transition (stock order, opponent hands, private draws) in state, effects, DOM, storage, logs, replay export. | Pairwise no-leak matrix extended to a multi-round trace; web a11y/no-leak smoke. |
| 6 | Browser: a full Bot-vs-bot match autoplays to the terminal outcome panel; no `round_settled` dead-end. | Puppeteer/e2e evidence + screenshot. |
| 7 | No rule-text change; `rule-coverage` and `fixture-check` green; only legitimate Meldfall artifacts changed (ADR 0009). | CI gate 0/1 logs. |

---

## 7. Acceptance evidence

### 7.1 Command suite (must pass)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
cargo run -p simulate      -- --game meldfall_ledger --games 1000   # completion > 0
cargo run -p replay-check  -- --game meldfall_ledger --all
cargo run -p fixture-check -- --game meldfall_ledger
cargo run -p rule-coverage -- --game meldfall_ledger
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
npm --prefix apps/web run build
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:ui
```

### 7.2 Test taxonomy (minimum new/extended)

- round-transition reset/preserve matrix (every row of §3.4);
- dealer rotation across N seats (2..6) and active-seat-left-of-dealer;
- deterministic re-deal: same seed ⇒ identical multi-round sequence;
- full match to unique 500 winner; tie-at/above-500 continuation (`ML-MATCH-003`);
- card conservation across rounds (52 cards accounted every round);
- WASM-vs-simulate host parity for a multi-round seed;
- no-leak across the transition (stock order / opponent hands / private draws);
- meld-id policy assertion (per §3.4).

### 7.3 Evidence artifacts (minimum new set)

The transition's *executable* proof is the host-parity full-match test (§7.2),
**not** a declarative trace — `replay-check` validates meldfall traces by
parsing / card-conservation / setup-projection, never by replaying commands. The
one new declarative artifact records the reset/preserve contract:

| Artifact | Asserts |
|---|---|
| `round-transition-resets-table-state.trace.json` (new) | Dealer rotates, tableau/discard/hands/pending/round_end reset, scores carry forward. |

The existing scoring-illustration fixtures
`games/meldfall_ledger/tests/golden_traces/multi-round-first-to-500.trace.json`
and `target-tie-continues.trace.json` (static `before`/`round_deltas`/`after`
snapshots keyed to `ML-MATCH-001/002/003/004/005`) **predate this gate and are
left intact** — they are not rewritten or clobbered. The unique-500-winner and
tie-continuation *behaviors* are proved by the host-parity full-match test and
`simulate` completion (exit criteria 2–3), not by those static fixtures.

Existing single-round fixtures remain **semantically** valid; the only assertion
that legitimately moves is the `stable_internal_summary` `starts_with` check in
`games/meldfall_ledger/tests/serialization.rs`, updated (not weakened) to the
round-index-extended format under ADR 0009. "Still valid" therefore means valid
after that one governed update, not byte-identical.

---

## 8. FOUNDATIONS & boundary alignment

- **Behavior authority / boundary.** All transition logic, rotation, re-deal,
  terminal evaluation, and scoring stay in Rust (`games/meldfall_ledger` +
  `crates/wasm-api` bridge). TypeScript presents Rust-authored effects/views only.
- **Noun-free kernel.** No `engine-core` change; no card/meld/deck noun crosses
  the boundary. Seed/RNG use the existing `engine-core` primitives.
- **§11 invariants.** Determinism, hashes, RNG, serialization order, and traces
  stay deterministic or are explicitly migrated under ADR 0009.
- **§12 stop conditions.** If completing `ML-MATCH-006` appears to require a new
  shared helper, an `engine-core` noun, or a contract change, **stop and reassess**
  rather than generalize; open an ADR (§13) before continuing.
- **No-leak.** The re-deal must not expose stock order or opponent hands through
  any surface; the new effect is public-counts/seat-labels only.

---

## 9. Forbidden changes

- No rule-text edits to `RULES.md` (`ML-MATCH-006` already governs this).
- No discard-pile reshuffle into stock; no rearranging tabled melds.
- No TypeScript legality, rotation, or terminal decisions.
- No blanket golden/fixture/hash regeneration; no test weakening to go green.
- No new shared helper, `engine-core` noun, or variant/deck-count change.

---

## 10. Documentation updates required

| Doc | Update |
|---|---|
| `crates/wasm-api/src/games/meldfall.rs` | Remove/supersede the "intentionally deferred" `round_score_index` comment once the transition lands. |
| `games/meldfall_ledger/docs/RULE-COVERAGE.md` | Mark `ML-MATCH-006` (and the `ML-MATCH-003` tie-continuation) as covered with the new traces/tests. |
| `games/meldfall_ledger/docs/GAME-EVIDENCE.md` | Add the multi-round completion receipt (commands, traces, host-parity). |
| `games/meldfall_ledger/docs/MECHANICS.md` / `HOW-TO-PLAY.md` | Confirm round/match-flow prose matches the now-implemented transition (version-checked note; no behavior claims beyond `RULES.md`). |
| `specs/README.md` | Add the Gate 19.1 row; flip to `Done` on closeout with evidence. |
| `docs/adr/` | None expected. Only if §8 §12 forces a contract change. |

---

## 11. Assumptions

1. `ML-MATCH-006` text is authoritative and complete; this spec implements it
   verbatim and adds no new rule.
2. The single-round engine (meld/lay-off/pickup/scoring/settlement) is correct and
   unchanged; only the match-level loop is missing.
3. `deal_for_round` and `next_clockwise_index` are reusable as-is for the re-deal
   and rotation; only seed derivation and state-reset wiring are new.
4. ADR 0009 governs any legitimate Meldfall trace/fixture/hash movement; no
   unrelated artifact regeneration is implied.
5. If any assumption is false against the codebase at execution time, correct this
   spec first (per the Ticket Execution Contract) before implementing.
