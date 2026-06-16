# River Ledger correctness and presentation report

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/rulepath`.

## Exact-commit evidence ledger

```text
Requested repository: joeloverbeck/rulepath
Target commit: 351dc1ec47b976aecc376022b718d8f921ca4bcb
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Code search used: no
Clone used: no
URL fetch method: web.open / web.find on full exact-commit raw.githubusercontent.com URLs, with two exact-commit github.com/blob URL inspections for large bridge/type files
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/FOUNDATIONS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/ENGINE-GAME-DATA-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/UI-INTERACTION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/WASM-CLIENT-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/OFFICIAL-GAME-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/IP-POLICY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/TESTING-REPLAY-BENCHMARKING.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/docs/ROADMAP.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/RULES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/UI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/HOW-TO-PLAY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/RULE-COVERAGE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/COMPETENT-PLAYER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/evaluator.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/cards.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/setup.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/betting.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/pot.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/actions.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/rules.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/showdown.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/visibility.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/src/ui.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/rules.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/property.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/visibility.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/replay.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/bots.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/apps/web/src/components/RiverLedgerBoard.tsx
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/apps/web/src/components/OutcomeExplanationPanel.tsx
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/apps/web/src/components/outcomeExplanationTemplates.ts
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/apps/web/src/wasm/client.ts
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/crates/wasm-api/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/setup-3p.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/setup-4p.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/setup-5p.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/setup-6p.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/invalid-seat-count.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/preflop-blinds-call-check-advance.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/flop-small-bet-cap.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/turn-river-big-bet.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/raise-cap-diagnostic.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/wrong-seat-diagnostic.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/high-card-showdown.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/pair-beats-high-card.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/full-house-tiebreak.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/flush-kicker-order.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/straight-ace-low.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/split-pot-even.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/split-pot-remainder-button-order.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/foldout-last-live-hand.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/public-observer-no-leak.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/deal-private-no-leak.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/seat-private-view.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/public-replay-export-import.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/351dc1ec47b976aecc376022b718d8f921ca4bcb/games/river_ledger/tests/golden_traces/bot-vs-bot-full-game-6p.trace.json
- https://github.com/joeloverbeck/rulepath/blob/351dc1ec47b976aecc376022b718d8f921ca4bcb/apps/web/src/wasm/client.ts
- https://github.com/joeloverbeck/rulepath/blob/351dc1ec47b976aecc376022b718d8f921ca4bcb/crates/wasm-api/src/lib.rs
Contamination observed: no
Connector/tool namespace trusted as evidence: no
```

Operational note: every repository path above was first checked against the uploaded manifest. The manifest was used only as a path inventory. No repository metadata, default-branch lookup, branch fetch, code search, clone, prior chat context, or connector namespace label was used as evidence.

## Executive verdict

River Ledger is **substantially correct against its own Gate 15 `RL-*` rule contract** at commit `351dc1ec47b976aecc376022b718d8f921ca4bcb`. I found no gameplay-correctness defect in setup, blinds, fixed-limit betting, round closure, single-pot accounting, evaluator category ordering, ordered tie-break vectors, ace-low straight handling, best-five-of-seven selection, showdown winner/split resolution, or no-leak projection.

The meaningful defect is not in the evaluator. It is a **high-severity presentation/usability defect**: the current post-showdown surface exposes correct machine facts (`category`, `tie_break_vector`, and `best_five`) but makes the decisive comparison illegible to ordinary players. In the worked example, the engine correctly encodes **Pair of Queens beats Pair of Eights** as `[12,10,8,6]` versus `[8,12,10,6]`; the UI should say that plainly instead of showing `one_pair` and raw integers.

The intentionally absent casino features — no all-in, no side pots, no no-limit/pot-limit, no stacks, no real-money framing — are confirmed **not bugs**. They are explicit Gate 15 exclusions under `RL-POT-ALLIN-001`, `RL-OOS-ALLIN-001`, `RL-OOS-NOLIMIT-001`, `RL-BET-AMB-001`, and `RL-VAR-ALLIN-001`.

---

# Part A — Correctness audit

## A.1 Audit basis and doctrine

Authority flows from the foundation documents into the game documents. The controlling constraints are:

- Rust owns setup, legal actions, validation, street transitions, scoring, terminal detection, hidden-information projection, replay/hash behavior, and bot decisions.
- TypeScript presents viewer-safe Rust/WASM output only. It must not compute legality, active turn, hand strength, winners, split, or hidden-card redaction.
- `engine-core` stays noun-free; River Ledger nouns such as card, hand, pot, betting, showdown, evaluator, and ledger stay in `games/river_ledger`.
- Static data stays typed content/metadata; no YAML/DSL or behavior-in-data is introduced.
- Hidden information must not leak into public/seat payloads, DOM, accessibility labels, test IDs, logs, storage, snapshots, replays, effects, diagnostics, or bot explanations.
- Public presentation must avoid casino trade dress, real-money framing, copied table/card art, rake/payout language, tournament branding, and affiliation implication.

The correctness oracle is `games/river_ledger/docs/RULES.md`, not casino No-Limit Hold'em practice.

## A.2 Verdict table

| Area | Verdict | Severity | Evidence |
|---|---:|---:|---|
| Setup, deck, seat counts, button/blinds | correct | none | Rules: `RL-SETUP-SEATS-001`, `RL-SETUP-SEATS-002`, `RL-SETUP-VARIANT-001`, `RL-DEAL-DECK-001`, `RL-DEAL-DECK-002`, `RL-DEAL-SHUFFLE-001`, `RL-DEAL-HOLE-001`, `RL-DEAL-HOLE-002`, `RL-DEAL-BOARD-002`, `RL-BET-BUTTON-002`, `RL-BET-BLINDS-002`, `RL-STREET-PREFLOP-001`. Modules: `cards.rs`, `setup.rs`, `state.rs`. Tests/traces: `rules.rs`; `setup-3p`, `setup-4p`, `setup-5p`, `setup-6p`, `invalid-seat-count`. |
| Fixed-limit betting and raise cap | correct | none | Rules: `RL-BET-ACTION-001` through `RL-BET-ACTION-006`, `RL-BET-LIMIT-001`, `RL-BET-LIMIT-002`, `RL-BET-CAP-001`, `RL-BET-CALL-001`, `RL-BET-RAISE-001`, `RL-BET-CHECK-001`. Modules: `actions.rs`, `betting.rs`, `rules.rs`, `state.rs`. Tests/traces: `rules.rs`; `preflop-blinds-call-check-advance`, `flop-small-bet-cap`, `turn-river-big-bet`, `raise-cap-diagnostic`, `wrong-seat-diagnostic`. |
| Round closure and street transitions | correct | none | Rules: `RL-STREET-PREFLOP-002`, `RL-STREET-FLOP-001`, `RL-STREET-TURN-001`, `RL-STREET-RIVER-001`, `RL-STREET-SHOWDOWN-001`, `RL-STREET-FOLDOUT-001`, plus the betting-action rules above. Modules: `betting.rs`, `rules.rs`. Tests/traces: `preflop-blinds-call-check-advance`, `turn-river-big-bet`, `foldout-last-live-hand`, `rules.rs` river checkdown/terminal tests. |
| Pot allocation and split remainder | correct | none | Rules: `RL-POT-SINGLE-001`, `RL-POT-SINGLE-002`, `RL-SCORE-POT-AWARD`, `RL-SCORE-SPLIT`, `RL-POT-REMAINDER-001`, `RL-POT-AMB-001`. Module: `pot.rs`, called from `showdown.rs` and `rules.rs`. Tests/traces: `split-pot-even`, `split-pot-remainder-button-order`, `foldout-last-live-hand`, plus pot/accounting invariants in `rules.rs` and `property.rs`. |
| Hand evaluation: categories, kickers, ace-low straight, best five of seven | correct | none | Rules: `RL-EVAL-FIVE-001`, `RL-EVAL-ACELOW-001`, `RL-EVAL-SEVEN-001`, `RL-EVAL-TIEBREAK-001`, `RL-EVAL-USED-001`, `RL-EVAL-AMB-001`. Modules: `cards.rs`, `evaluator.rs`. Tests/traces: `high-card-showdown`, `pair-beats-high-card`, `full-house-tiebreak`, `flush-kicker-order`, `straight-ace-low`, evaluator unit tests, comparator properties in `property.rs`. |
| Tie-break ordering | correct | none | Rules: `RL-EVAL-TIEBREAK-001`, `RL-SCORE-SHOWDOWN`. Module: `evaluator.rs`, `showdown.rs`. Tests/traces: `flush-kicker-order`, `full-house-tiebreak`, `straight-ace-low`, `pair-beats-high-card`, evaluator comparator tests. External cross-check: standard poker ranks category first, then card ranks; suits do not break ties. [EXT-1], [EXT-2] |
| Showdown winner/split/foldout | correct | none | Rules: `RL-SHOW-ELIGIBLE-001`, `RL-SHOW-WINNER-001`, `RL-SHOW-SPLIT-001`, `RL-SHOW-FOLDOUT-001`, `RL-END-LAST-LIVE`, `RL-END-SHOWDOWN`. Modules: `showdown.rs`, `rules.rs`, `pot.rs`, `visibility.rs`. Tests/traces: `split-pot-even`, `split-pot-remainder-button-order`, `foldout-last-live-hand`, `public-replay-export-import`, rules showdown/foldout tests. |
| Visibility and no-leak at showdown | correct | none | Rules: `RL-VIS-PUBLIC-001`, `RL-VIS-PRIVATE-HOLE-001`, `RL-VIS-OPPONENT-HOLE-001`, `RL-VIS-DECKTAIL-001`, `RL-VIS-DIAGNOSTIC-001`, `RL-VIS-SHOWDOWN-001`, `RL-VIS-FOLDOUT-001`, `RL-VIS-VIEWHASH-001`, `RL-UI-NOLEAK-001`, replay redaction rules. Modules: `visibility.rs`, `state.rs`, `replay_support.rs`, `ui.rs`. Tests/traces: `visibility.rs`, `serialization.rs`, `replay.rs`, `public-observer-no-leak`, `deal-private-no-leak`, `seat-private-view`, `public-replay-export-import`, bot no-hidden-explanation tests. |
| Showdown explanation legibility | presentation bug, not evaluator bug | high UX severity | Rules: `RL-UI-SHOWDOWN-001`, `RL-UI-PRESENT-001`, `RL-UI-NOLEAK-001`; official-game outcome-rationale requirements. Modules: `showdown.rs` emits raw-ish summaries; `visibility.rs` projects `category`, `tie_break_vector`, `best_five`; `OutcomeExplanationPanel.tsx` and `outcomeExplanationTemplates.ts` render generic/jargon copy. Tests prove plumbing, not novice legibility. |
| UI coverage documentation state | ambiguous documentation drift | low | `RULE-COVERAGE.md` still has some intentionally-deferred UI rows, while `PUBLIC-RELEASE-CHECKLIST.md` and the current inspected code indicate River Ledger UI/outcome plumbing is present. This is not a gameplay defect, but the coverage map should be refreshed after the redesign. |

## A.3 Setup, deck, blinds, and preflop actor

`cards.rs` defines a game-local standard deck with four suits and ranks Two through Ace, with rank values `2..14`. `setup.rs` validates exactly 3, 4, 5, or 6 seats, constructs and shuffles the game-local 52-card deck with deterministic Rulepath RNG, deals two private hole cards per seat, reserves the five community cards, and keeps the remaining deck tail internal. `state.rs` applies small and big blind forced contributions before preflop action and chooses the first active seat after the big blind.

The golden setup traces confirm the public setup shape: 3p starts action at `seat_0` after button `seat_0`, small blind `seat_1`, big blind `seat_2`; 4p/5p/6p start at `seat_3`; future board cards and deck tail are only counted, not named. `invalid-seat-count` confirms a deterministic public diagnostic for unsupported counts.

Verdict: **correct**.

## A.4 Fixed-limit betting, raise cap, and validation

`Street::unit` assigns small units to preflop/flop and big units to turn/river. `actions.rs` derives the legal action tree from the Rust state: facing a call price yields `fold`/`call` and `raise` only when the cap allows it; no open price yields `check`/`bet`; terminal yields no normal gameplay actions. `rules.rs` validates every command before mutation, applies calls, bets, raises, folds, and stale/wrong-seat/cap diagnostics deterministically. `betting.rs` computes call price, response order, and round closure from live seats and current street contributions.

The traces exercise the risky points: `preflop-blinds-call-check-advance` proves blind response order and big-blind check advance; `flop-small-bet-cap` proves one opening bet plus three raises and suppresses a fourth raise; `turn-river-big-bet` proves big-unit behavior on later streets; `raise-cap-diagnostic` and `wrong-seat-diagnostic` prove public-safe invalid-action paths.

Verdict: **correct**.

## A.5 Round closure and street transitions

The implementation closes a street only when no actors remain pending and all live seats have matched the current street contribution or checked when no contribution is owed. On closure, `rules.rs` advances preflop to a three-card flop, flop to one turn card, turn to one river card, and river to showdown when at least two live seats remain. If a fold leaves exactly one live seat, `rules.rs` goes immediately to last-live-hand terminal and does not reveal folded hole cards.

This matches `RL-STREET-PREFLOP-002`, `RL-STREET-FLOP-001`, `RL-STREET-TURN-001`, `RL-STREET-RIVER-001`, `RL-STREET-SHOWDOWN-001`, and `RL-STREET-FOLDOUT-001`. The closure tests and traces cover both normal street advance and foldout.

Verdict: **correct**.

## A.6 Pot allocation and split remainder

`pot.rs` implements the Gate 15 single-pot model. Every contribution feeds one public `pot_total`; there is no side-pot graph and no all-in eligibility state. `allocate_single_pot` divides the pot equally among tied winners and assigns any integer remainder one unit at a time in stable button order among tied winners.

The even split trace allocates 12 as 6/6. The remainder trace allocates an 11-unit pot among winners `seat_2`, `seat_3`, and `seat_0` with button `seat_2` as 4/4/3, proving the documented button-order remainder rule. `foldout-last-live-hand` proves last-live award without showdown reveal.

Verdict: **correct**.

## A.7 Hand evaluator: category ordering, kickers, ace-low, and best-five-of-seven

`evaluator.rs` implements a conventional high-hand evaluator with these ordered categories:

1. high card
2. one pair
3. two pair
4. three of a kind
5. straight
6. flush
7. full house
8. four of a kind
9. straight flush

It evaluates five-card hands by detecting flush, detecting straights with Ace also available as low only for A-2-3-4-5, counting ranks, and constructing ordered tie-break vectors. The tie-break vector shape is correct for the documented rules: `[quad, kicker]` for four of a kind; `[trips, pair]` for full house; `[trip, kickers...]`; `[high pair, low pair, kicker]`; `[pair, kickers...]`; and descending ranks for flush/high card. `best_five_from_seven` enumerates the 21 five-card subsets of two hole cards plus five board cards.

External cross-check: standard poker compares five-card hands by category first, then ranked cards within category; the Ace can be low only in the five-high straight; suits do not break ties. [EXT-1], [EXT-2]

The traces prove the risk areas: `straight-ace-low` has tie break `[5]`; `full-house-tiebreak` uses trip rank before pair rank; `flush-kicker-order` compares the ordered ranks in a flush; `pair-beats-high-card` confirms category ordering; `high-card-showdown` confirms descending high-card vector.

Verdict: **correct**.

## A.8 Showdown resolution and split/foldout correctness

`showdown.rs` evaluates only showdown-eligible live seats, chooses the maximum evaluation by category and tie-break vector, selects all seats equal to that max as winners, and delegates allocation to `pot.rs`. Folded seats are excluded from public showdown evaluation. `visibility.rs` exposes showdown strength only for authorized revealed seats. Last-live foldout is a distinct terminal path and does not reveal folded hands.

This matches `RL-SHOW-ELIGIBLE-001`, `RL-SHOW-WINNER-001`, `RL-SHOW-SPLIT-001`, `RL-SHOW-FOLDOUT-001`, `RL-SCORE-SHOWDOWN`, `RL-SCORE-SPLIT`, and `RL-SCORE-POT-AWARD`.

Verdict: **correct**.

## A.9 Visibility and no-leak correctness

The projection code separates public view, seat-private view, terminal outcome, and terminal rationale. Before showdown, a seat viewer sees only that seat's hole cards; public observer and other seats receive counts/redactions. Deck tail, burn/future board identities, and unauthorized opponent hole cards do not enter browser-facing payloads. At authorized showdown, revealed live contenders carry `best_five`, `category`, and `tie_break_vector`; folded/non-revealed private cards remain redacted. The replay/export tests confirm viewer-scoped public exports omit hidden facts and seed evidence.

Verdict: **correct**.

## A.10 Out-of-scope abstractions confirmed not bugs

The absence of all-in calls, side pots, no-limit/pot-limit sizing, table stakes, stacks, rake, payouts, tournaments, and casino framing is deliberate. Relevant rules and deviations: `RL-POT-SINGLE-001`, `RL-POT-SINGLE-002`, `RL-POT-ALLIN-001`, `RL-OOS-ALLIN-001`, `RL-OOS-NOLIMIT-001`, `RL-BET-AMB-001`, `RL-VAR-LIMIT-001`, `RL-VAR-ALLIN-001`, and `RL-VAR-PRESENT-001`.

Do not add these to fix presentation. Gate 15.1 owns all-in/side-pot pressure if the project later chooses to model it.

## A.11 Worked showdown example: Pair of Queens beats Pair of Eights

Given board:

```text
4C 3D QH 6H 8H
```

Seat 4's best five:

```text
QC QH 10S 8H 6H
Category: one_pair
Tie-break vector: [12,10,8,6]
Human name: Pair of Queens, kickers Ten, Eight, Six
```

Seat 0's best five:

```text
QH 10C 8D 8H 6H
Category: one_pair
Tie-break vector: [8,12,10,6]
Human name: Pair of Eights, kickers Queen, Ten, Six
```

The engine's encoding is correct. `cards.rs` maps Queen to rank value 12 and Eight to 8. The evaluator compares category first; both hands are `one_pair`. It then compares the tie-break vector lexicographically. The first entry is the pair rank. Since `12 > 8`, **Seat 4 wins because a Pair of Queens beats a Pair of Eights**. The kickers never need to decide this comparison.

The current UI's problem is that it exposes the machine explanation instead of the player explanation. A novice should not have to know that the first integer is the pair rank or that 12 means Queen.

---

# Part B — Presentation and usability overhaul

## B.1 Design goal

The highest-priority redesign should make one sentence immediately obvious after showdown:

> Seat 4 wins because a Pair of Queens beats a Pair of Eights.

Everything else should support that sentence: visual cards, named hand categories, the exact five cards used, the decisive comparison, per-seat allocation, and an always-available hand-ranking reference. This follows the repo's own outcome-rationale doctrine and the general usability principle that interfaces should favor recognition over recall and provide help in context. [EXT-5]

## B.2 Current problem summary

Current surface from the live capture:

```text
OUTCOME — Seat 4 wins
"One showdown hand has the strongest Rust-evaluated five-card result."

Seat 4   WIN              Contribution 2  Allocation 12
         Category  one_pair
         Tie break 12,10,8,6
         Best five QC QH 10S 8H 6H
Seat 0   SHOWDOWN_LOSS    Contribution 2  Allocation 0
         Category  one_pair
         Tie break 8,12,10,6
         Best five QH 10C 8D 8H 6H
```

This is technically true but user-hostile. The decisive fact is absent. The UI forces the player to decode category keys, integer ranks, vector ordering, rank abbreviations, and comparison rules. It also uses engine-facing wording (`Rust-evaluated`) in a public player surface.

## B.3 Concrete annotated redesign of the showdown outcome panel

Default recommendation: a persistent post-showdown panel with the concise answer first, then an expandable detailed comparison. Do not hide the reason behind a hover-only tooltip; the owner has identified this as the main comprehension failure. Use progressive disclosure for advanced vector/debug facts, not for the human reason. [EXT-4]

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ OUTCOME — Seat 4 wins the ledger                                           │
│                                                                            │
│ Why Seat 4 won                                                             │
│ Pair of Queens beats Pair of Eights.                                       │
│ Both hands are one pair, so the pair rank decides first: Queens > Eights.  │
│                                                                            │
│ Board                                                                      │
│ [4♣ Clubs] [3♦ Diamonds] [Q♥ Hearts] [6♥ Hearts] [8♥ Hearts]              │
│                                                                            │
│ Winner                                                                     │
│ ★ Seat 4 — Pair of Queens                         Received 12 / Put in 2  │
│ Best five                                                                  │
│ [Q♣ Clubs] [Q♥ Hearts] [10♠ Spades] [8♥ Hearts] [6♥ Hearts]               │
│ Judged by: pair rank Queen; kickers Ten, Eight, Six.                       │
│ Screen-reader summary: Seat 4 wins with a pair of Queens; kickers Ten,     │
│ Eight, and Six.                                                            │
│                                                                            │
│ Closest challenger                                                         │
│ Seat 0 — Pair of Eights                           Received 0 / Put in 2   │
│ Best five                                                                  │
│ [8♦ Diamonds] [8♥ Hearts] [Q♥ Hearts] [10♣ Clubs] [6♥ Hearts]             │
│ Loses because: same category, but Queen pair outranks Eight pair.          │
│                                                                            │
│ Other revealed showdown hands                                              │
│ Seat 1 — Queen-high                                Received 0             │
│ Seat 2 — ...                                                               │
│                                                                            │
│ Hand ranking reference                                                     │
│ Straight flush > Four of a kind > Full house > Flush > Straight >          │
│ Three of a kind > Two pair > Pair > High card                              │
│ Current winning category: Pair                                             │
│                                                                            │
│ Details ▸ show tie-break vectors and rule IDs                              │
│   Seat 4 vector [12,10,8,6]; Seat 0 vector [8,12,10,6];                    │
│   rules RL-EVAL-TIEBREAK-001, RL-SCORE-SHOWDOWN.                           │
└────────────────────────────────────────────────────────────────────────────┘
```

Annotations:

- The first two lines answer **who won** and **why**.
- The contrastive line names the best challenger, because people usually need to know “why this hand rather than that hand,” not merely why the winner is good. [EXT-7]
- The board is visually card-like but neutral; it uses suit glyph + suit word, not color alone.
- The winner is marked with text and icon (`Winner`, `★`), not color alone.
- The hand-ranking ladder is always visible or one click away and never requires the player to memorize the category order.
- Raw vectors move to a details disclosure for verification/debugging, not the main player explanation.

## B.4 Before/after copy table

All proposed after-copy should be **Rust-authored** and projected through `visibility.rs` / WASM. TypeScript may choose layout and typography, but it must not infer the category label, hand name, winner, loser, decisive comparison, or tie-break explanation.

| Current public copy/data | Problem | Proposed player-facing copy/data |
|---|---|---|
| `One showdown hand has the strongest Rust-evaluated five-card result.` | Engine jargon; no decisive reason. | `Seat 4 wins with Pair of Queens.` plus `Pair of Queens beats Pair of Eights.` |
| `Category one_pair` | Raw enum key; not a hand name. | `Hand: Pair of Queens` and secondary `Category: Pair`. |
| `Tie break 12,10,8,6` | Requires hidden knowledge of vector semantics and rank values. | `Judged by: pair rank Queen; kickers Ten, Eight, Six.` |
| `Best five QC QH 10S 8H 6H` | Compact poker notation only; poor for novices. | Visual cards with glyph + suit word: `[Q♣ Clubs] [Q♥ Hearts] [10♠ Spades] [8♥ Hearts] [6♥ Hearts]`; accessible label: `Queen of Clubs, Queen of Hearts, Ten of Spades, Eight of Hearts, Six of Hearts`. |
| `SHOWDOWN_LOSS` | Raw result key. | `Showdown loss` or `Lost at showdown`. |
| `Allocation 12` | Accounting noun is okay but not friendly. | `Received 12 from the ledger` or `Ledger received: 12`. Keep units abstract. |
| `Contribution 2` | Okay but could read financial. | `Put in: 2 abstract units`. Avoid money/chips framing. |
| `best_showdown_hand` | Internal decisive-cause key. | `Best revealed hand won.` |
| `equal_best_hand_split` | Internal decisive-cause key. | `Tied best hands split the ledger.` |
| `Remainder by button order` | Needs explaining when it occurs. | `One leftover unit went to Seat N first by button order.` |
| `last_live_after_folds` | Internal key. | `Seat N wins because every other live seat folded. Folded cards stay hidden.` |

## B.5 Proposed Rust-authored field shape

Add display-oriented, deterministic explanation fields to the Rust projection rather than synthesizing them in React. Prefer additive fields to preserve compatibility with existing trace/replay consumers.

Recommended shape:

```rust
// games/river_ledger/src/visibility.rs or state-adjacent view model
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiverLedgerShowdownExplanationView {
    pub headline: String,
    // Example: "Seat 4 wins with Pair of Queens."
    pub decisive_comparison: String,
    // Example: "Pair of Queens beats Pair of Eights."
    pub comparison_basis: String,
    // Example: "Both hands are one pair, so the pair rank decides first: Queens > Eights."
    pub board: Vec<CardView>,
    pub contenders: Vec<RiverLedgerShowdownSeatExplanationView>,
    pub hand_ranking_ladder: Vec<HandRankingLegendRowView>,
    pub debug_rule_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiverLedgerShowdownSeatExplanationView {
    pub seat: SeatId,
    pub result: String,
    // "win", "showdown_loss", "split_win", etc.; existing key preserved.
    pub result_label: String,
    // "Win", "Showdown loss", "Split win".
    pub allocation: u16,
    pub contribution: u16,
    pub category_key: String,
    // "one_pair".
    pub category_label: String,
    // "Pair".
    pub hand_name: String,
    // "Pair of Queens" / "Queen-high" / "Fives full of Kings".
    pub rank_explanation: String,
    // "pair rank Queen; kickers Ten, Eight, Six".
    pub best_five: Vec<CardView>,
    pub best_five_accessibility_label: String,
    pub comparison_note: Option<String>,
    // Winner: "Beats Seat 0's Pair of Eights."
    // Loser: "Loses to Seat 4: Queen pair outranks Eight pair."
    pub tie_break_vector: Vec<u8>,
    // Keep for debug/details; do not require users to decode it.
    pub teaching_strength: Option<ShowdownTeachingStrengthView>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandRankingLegendRowView {
    pub category_key: String,
    pub category_label: String,
    pub rank_order_high_to_low: u8,
    pub short_definition: String,
    pub example: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShowdownTeachingStrengthView {
    pub label: String,
    // Example: "Teaching aid: Pair is category 8 of 9 from strongest to weakest."
    pub percent: Option<u8>,
    // Optional only; derived from revealed final hand facts, not hidden equity.
    pub caveat: String,
    // "Learning aid, not a game score."
}
```

Recommended flow:

1. `evaluator.rs` stays the source of category/tie-break/best-five facts.
2. `showdown.rs` builds deterministic human-readable hand names, rank explanations, and decisive comparison strings from already-authorized showdown results.
3. `state.rs` or `visibility.rs` adds an optional display object to `ShowdownReveal` / `ShowdownStrengthView` or the terminal rationale.
4. `visibility.rs` applies the existing authorization rule: only showdown-eligible revealed seats get hand explanations; folded/non-revealed seats do not.
5. `crates/wasm-api/src/lib.rs` exposes the augmented JSON as part of the normal view payload.
6. `apps/web/src/wasm/client.ts` adds the TypeScript types matching the Rust JSON.
7. `OutcomeExplanationPanel.tsx` and `RiverLedgerBoard.tsx` render these fields. They may choose layout, collapse/expand details, and card styling; they must not compute the category label, winner, hand name, comparison reason, or strength value.

Where to put the label logic: keep it in `games/river_ledger`, not `engine-core`, and not TypeScript. A helper such as `display.rs` or an internal section in `showdown.rs` is appropriate. It should be deterministic and covered by tests.

## B.6 Card and suit rendering recommendations

Current `RiverCard` renders a plain text block with raw suit/rank information. Replace it with a neutral, original card component inside `RiverLedgerBoard.tsx` or a small local component extracted from it.

Requirements:

- Show rank large: `Q`, `10`, `A`.
- Show suit glyph and suit word: `♥ Hearts`, `♦ Diamonds`, `♣ Clubs`, `♠ Spades`.
- Use red/black or warm/cool color as a secondary aid only. WCAG says color must not be the only way to convey information. [EXT-3]
- Meet WCAG AA text contrast: normal text at least 4.5:1; UI component boundaries/icons at least 3:1 where applicable. [EXT-8]
- Provide accessible labels for every card and every best-five group. WAI guidance requires text alternatives that convey the same information as visual images/icons. [EXT-6]
- Use shape and text double-coding: hearts/diamonds/clubs/spades glyphs plus words; do not invent a private suit icon language that users must learn.
- Avoid casino trade dress: no green felt table, chip stacks, currency symbols, branded card backs, or copied playing-card art. Use Rulepath's neutral “ledger/tabletop” identity and abstract units.

Recommended visual style:

```text
┌──────┐
│ Q  ♥ │   top-left rank/suit
│      │   neutral light surface, high-contrast border
│Heart │   suit word or short suit label for colorblind/novice support
└──────┘
```

For compact table cards, use:

```text
[Q♥ Hearts]  [10♠ Spades]
```

For screen readers, do not rely on each glyph being pronounced correctly. Provide a group-level label such as:

```text
Best five for Seat 4: Queen of Clubs, Queen of Hearts, Ten of Spades, Eight of Hearts, Six of Hearts. Hand: Pair of Queens. Wins because Queen pair outranks Eight pair.
```

## B.7 Optional strength indicator as a teaching aid

Recommendation: include a **terminal-only teaching aid**, not an in-play equity meter.

Safe default:

```text
Teaching aid — not a game score
Category tier: Pair (8 of 9 from strongest to weakest)
Within Pair: Queens outrank Eights; kickers would decide only if pair ranks tied.
```

Avoid a fake canonical numeric score. Texas Hold'em uses category plus ordered kickers, not an engine-computed point score. A 0-100 “strength” number can be misleading unless it is clearly defined. If a meter is desired, make it one of these:

- **Category ladder position:** deterministic, easy to explain, no hidden information.
- **Revealed-showdown relative order:** winner at top, others sorted below; label as “order among revealed hands,” not absolute power.
- **Optional normalized teaching percentage:** only if Rust computes it from the final revealed best hand and labels it “learning aid, not game value.” Do not use hidden deck tails, folded unrevealed hands, future board cards, or opponent private cards before authorized reveal.

Do not implement a pre-showdown odds/equity meter in this redesign. That would invite hidden-information and probabilistic-inference ambiguity, plus it would distract from the owner’s main pain point.

## B.8 Secondary table UI improvements

These are lower priority than showdown but should be batched after the outcome panel because they reuse the same card component and copy discipline.

1. **Seat panels:** show Button / Small blind / Big blind badges from Rust view fields; show active seat with text, icon, and focus state, not color alone.
2. **Contribution language:** use “Put in” and “Ledger total” rather than casino chip/pot visual language. “Pot” may remain in rule/debug detail, but public copy should lean on River Ledger’s abstract identity.
3. **Action panel:** group legal actions by Rust-provided action family and show call price/adds-to-ledger from Rust metadata. Disable unavailable actions only if the unavailable reason is Rust-authored and viewer-safe.
4. **Turn flow strip:** show `Preflop → Flop → Turn → River → Showdown` with the current street highlighted by text and icon. Explain the next reveal: “Next: reveal one river card after this betting round closes.” This must come from public street state, not inferred from DOM ordering.
5. **Board reveal animation:** use reduced-motion-aware reveal. The card identities must already be in the authorized public view; animation must not preload hidden future board into DOM/a11y labels/test IDs.
6. **Hand-ranking reference:** provide a persistent small ladder near the board or a collapsible panel pinned in the outcome panel. Default to visible after showdown; collapsible during play.
7. **Debug details:** keep rule IDs, raw vectors, and trace-ish strings in a `Details` disclosure for verification. They are useful for developers, but they should not be the player-facing explanation.

---

# Part C — Prioritized recommendation backlog

| Rank | Recommendation | Why it matters | Boundary constraints | Effort / impact |
|---:|---|---|---|---|
| 1 | Add Rust-authored showdown explanation display fields: headline, decisive comparison, hand names, rank explanations, per-seat comparison notes, and accessibility labels. | Fixes the core “what scored and why” failure without moving evaluator logic to TypeScript. | Rust-authored; viewer-safe; deterministic serialization; no engine-core nouns. | Medium effort / very high impact. |
| 2 | Redesign the terminal showdown panel around the decisive sentence and best-five visual cards. | Makes winning logic legible at a glance. | TS render-only; no hidden leaks; no casino trade dress. | Medium effort / very high impact. |
| 3 | Replace raw category/vector main display with named hand copy and move vectors/rule IDs into details. | Preserves auditability while removing novice-hostile jargon. | Rust-authored copy; deterministic; TS layout only. | Low-medium effort / high impact. |
| 4 | Build a local neutral River Ledger card component with glyph + suit word + accessible label. | Improves board, hole cards, and best-five readability across the whole table. | TS presentation only; uses Rust `CardView`; no copied assets; color not sole cue. | Medium effort / high impact. |
| 5 | Add always-available hand-ranking reference. | Reduces memorization load and helps players learn category order. | Rust-authored labels/definitions preferred; TS can render static layout; no evaluator logic. | Low-medium effort / high impact. |
| 6 | Add terminal-only teaching strength aid. | Helps novices understand “how strong” a category is without inventing a score. | Rust-computed from revealed final hand only; labeled non-canonical; no hidden equity. | Medium effort / medium-high impact. |
| 7 | Strengthen tests for explanation strings and no-leak paths. | Ensures new labels do not regress determinism or leak folded/unrevealed cards through DOM/a11y/test IDs. | Replay/hash discipline; viewer-specific projections; no weakening tests. | Medium effort / high risk reduction. |
| 8 | Refresh `RULE-COVERAGE.md` UI rows after implementation. | Resolves documentation drift between coverage map and release checklist/current plumbing. | Docs only; authority order preserved. | Low effort / medium impact. |
| 9 | Improve action panel copy using Rust metadata: call price, adds-to-ledger, cap remaining, unavailable reasons. | Reduces betting confusion without changing rules. | Rust legal action tree and metadata; TS render-only. | Medium effort / medium impact. |
| 10 | Improve seat/turn-flow affordances with text/icon states and reduced-motion reveals. | Makes table state easier to scan. | Public Rust state only; no hidden future card preload; WCAG color/contrast. | Medium effort / medium impact. |
| 11 | Audit public copy for casino vocabulary and replace with River Ledger neutral terms where possible. | Keeps IP/product posture clean. | `RL-UI-NOCASINO-001`; abstract units only. | Low effort / medium impact. |
| 12 | Add e2e checks for the worked showdown scenario. | Locks the main user-facing fix: “Pair of Queens beats Pair of Eights.” | Can assert rendered Rust-authored strings; must not expose hidden facts. | Medium effort / high regression value. |

## Implementation order I recommend

Do this in two slices:

**Slice 1 — Outcome comprehension:** add Rust-authored explanation fields, render the new showdown panel, add tests for the worked example, and move raw vectors to details. This directly solves the owner’s pain point.

**Slice 2 — Table readability:** introduce neutral card visuals, hand-ranking reference, action panel copy, seat/turn-flow polish, and no-casino copy audit. This improves the whole game without risking the correctness core.

---

# Sources

## Repository sources

All repository evidence came from the exact URLs in the ledger at commit `351dc1ec47b976aecc376022b718d8f921ca4bcb`. Key internal files referenced in the findings include:

- `games/river_ledger/docs/RULES.md`
- `games/river_ledger/docs/RULE-COVERAGE.md`
- `games/river_ledger/docs/UI.md`
- `games/river_ledger/docs/HOW-TO-PLAY.md`
- `games/river_ledger/src/evaluator.rs`
- `games/river_ledger/src/showdown.rs`
- `games/river_ledger/src/pot.rs`
- `games/river_ledger/src/betting.rs`
- `games/river_ledger/src/rules.rs`
- `games/river_ledger/src/actions.rs`
- `games/river_ledger/src/state.rs`
- `games/river_ledger/src/visibility.rs`
- `apps/web/src/components/RiverLedgerBoard.tsx`
- `apps/web/src/components/OutcomeExplanationPanel.tsx`
- `apps/web/src/components/outcomeExplanationTemplates.ts`
- `apps/web/src/wasm/client.ts`
- `crates/wasm-api/src/lib.rs`

## External sources

[EXT-1] Pagat, “Rules of Poker,” especially hand ranking, showdown, equal suits, ace-low straight, five-card hand comparison, and betting-round closure. https://www.pagat.com/poker/rules/ Accessed 2026-06-15.

[EXT-2] Wikipedia, “List of poker hands,” for cross-checking standard high-hand category ordering, rank comparison, ace-low straight exception, and suits not ranking hands. https://en.wikipedia.org/wiki/List_of_poker_hands Accessed 2026-06-15.

[EXT-3] W3C WAI, “Understanding Success Criterion 1.4.1: Use of Color,” for the requirement that color is not the only visual means of conveying information. https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html Accessed 2026-06-15.

[EXT-4] Nielsen Norman Group, “Progressive Disclosure,” for using a small initial display and disclosing advanced details only when needed. https://www.nngroup.com/articles/progressive-disclosure/ Accessed 2026-06-15.

[EXT-5] Nielsen Norman Group, “10 Usability Heuristics for User Interface Design,” especially recognition rather than recall and contextual help. https://www.nngroup.com/articles/ten-usability-heuristics/ Accessed 2026-06-15.

[EXT-6] W3C WAI, “Images Tutorial,” including text alternatives for informative images and groups of images. https://www.w3.org/WAI/tutorials/images/ Accessed 2026-06-15.

[EXT-7] Buçinca et al., “Contrastive Explanations That Anticipate Human Misconceptions Can Improve Human Decision-Making Skills,” CHI 2025 / ACM entry and arXiv HTML, used for the design choice to explain “why P rather than Q.” https://dl.acm.org/doi/abs/10.1145/3706598.3713229 and https://arxiv.org/html/2410.04253v1 Accessed 2026-06-15.

[EXT-8] W3C, “Web Content Accessibility Guidelines (WCAG) 2.2,” and W3C WAI “Understanding Success Criterion 1.4.3: Contrast (Minimum),” for contrast targets for text and UI elements. https://www.w3.org/TR/WCAG22/ and https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html Accessed 2026-06-15.
