# River Ledger Rules

Game ID: `river_ledger`

Public display name: `River Ledger`

Implemented variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v1`

Prepared by: `Codex`

Created: 2026-06-14

Last updated: 2026-06-18

## Rule authority

This document is the original Rulepath rules summary for the planned River
Ledger implementation. Sources belong in `SOURCES.md`; this document states the
Rulepath implementation contract that later code, traces, coverage, bots, and UI
must satisfy.

Stable `RL-*` rule IDs are requirements. They must remain stable after
implementation unless intentionally migrated with a migration note and matching
updates in `RULE-COVERAGE.md`, traces, tests, and player-facing docs.

## Metadata

| Field | Value |
|---|---|
| game id | `river_ledger` |
| public display name | `River Ledger` |
| variant | `river_ledger_standard` |
| rules version | `river-ledger-rules-v1` |
| source note | `games/river_ledger/docs/SOURCES.md` |
| coverage matrix | `games/river_ledger/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/river_ledger/docs/MECHANICS.md` |
| implementation admission | `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

River Ledger is a 3-6 seat, fixed-limit, hidden-information community-card game
in the Texas Hold'Em rules family. It proves that Rulepath can support an
N-seat private-hand betting game while keeping setup, legal actions,
contribution accounting, visibility, showdown evaluation, outcome explanation,
replay, and bots in Rust.

Public presentation uses the original name River Ledger and neutral board-game
language. The implementation uses abstract contribution units only. It does not
implement real-money features, tournament structure, rake, payouts, stacks,
side pots, all-in handling, no-limit play, pot-limit play, hosted multiplayer,
or casino presentation.

Game nouns in this document belong to `games/river_ledger` only. They do not
authorize `card`, `deck`, `hand`, `street`, `pot`, `blind`, `button`, `bet`,
`raise`, `showdown`, or evaluator vocabulary in `engine-core`.

## Implemented variant

The only planned Gate 15 variant is `river_ledger_standard`.

| Field | Value |
|---|---|
| seats | exactly 3, 4, 5, or 6 |
| deck | standard 52-card deck, game-local representation |
| private cards | two hole cards per seat |
| public cards | five community cards revealed as flop, turn, river |
| contribution model | fixed-limit abstract contribution units |
| forced opening contributions | small blind and big blind |
| street sequence | preflop, flop, turn, river, showdown |
| raise cap | one opening bet plus three raises per street |
| terminal outcomes | last-live-hand foldout, single showdown winner, split showdown |
| randomness | deterministic seeded shuffle only |
| excluded scope | all-in, side pots, no-limit, pot-limit, tournaments, real-money features |

## Components and game-local vocabulary

Static data may carry IDs, labels, variants, metadata, fixtures, traces, and
reports. Static data must not carry legality, betting, evaluator, visibility,
bot, replay, trigger, selector, condition, or formula behavior.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `RL-SETUP-SEATS-001` | seat | A participating player position in stable `SeatId` order. | public | Official counts are 3-6 only. |
| `RL-DEAL-DECK-001` | deck | The game-local 52-card ordered source for deterministic shuffle and deal. | internal except visible dealt cards | Deck tail and burn positions never reach browser-facing views. |
| `RL-DEAL-CARD-001` | card | A game-local rank/suit identity used by the evaluator and safe display metadata. | private, public, or internal by location | Card IDs are not engine-core vocabulary. |
| `RL-DEAL-HOLE-001` | hole cards | The two private cards dealt to one seat. | owner only until authorized showdown reveal | Opponent hole cards remain redacted. |
| `RL-DEAL-BOARD-001` | community cards | Public shared cards revealed in the flop/turn/river sequence. | public after reveal | Unrevealed future board cards stay internal. |
| `RL-BET-LEDGER-001` | contribution ledger | Public per-seat and total contribution counts for the current hand. | public counts only | Units are abstract; no real-world value or product value. |
| `RL-BET-BUTTON-001` | button | The deterministic seat-order marker used for blind assignment and remainder order. | public | Rotates only when a multi-hand fixture intentionally models rotation. |
| `RL-BET-BLIND-001` | small blind / big blind | Forced opening contribution roles assigned from button order. | public | Forced contributions happen before preflop action. |
| `RL-STREET-PHASE-001` | street | One of preflop, flop, turn, river, or showdown. | public | Street controls contribution unit and reveal timing. |
| `RL-POT-SINGLE-001` | single pot | The one public total allocated at terminal. | public | Side pots and all-in eligibility are absent in Gate 15. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `RL-SETUP-SEATS-002` | Accept exactly 3, 4, 5, or 6 seats and reject every other count with a deterministic diagnostic. | deterministic | public | Wrong-seat-count diagnostics expose no hidden setup facts. |
| `RL-SETUP-VARIANT-001` | Select `river_ledger_standard` unless a typed future variant is explicitly added. | deterministic | public | Variant data is metadata, not behavior. |
| `RL-DEAL-DECK-002` | Construct the game-local 52-card deck in stable order before shuffle. | deterministic | internal before projection | Stable order supports replay and trace checks. |
| `RL-DEAL-SHUFFLE-001` | Shuffle with Rulepath deterministic RNG discipline from the match seed. | seeded deterministic | internal except projected cards | Same seed, variant, and command stream reproduce the same internal state. |
| `RL-BET-BUTTON-002` | Assign button, small blind, and big blind from stable seat order. | deterministic | public | The base single-hand setup records the chosen button. |
| `RL-BET-BLINDS-002` | Apply small blind and big blind forced contributions before preflop action. | deterministic | public counts | Contributions are abstract units. |
| `RL-DEAL-HOLE-002` | Deal two private hole cards to every seat. | seeded deterministic | owner only | Observer and other seats receive redacted placeholders or counts. |
| `RL-DEAL-BOARD-002` | Reserve the community-card sequence and any burn advancement internally. | seeded deterministic | internal until reveal | Burn/deck-tail identities are never projected. |
| `RL-STREET-PREFLOP-001` | Start in preflop with the first legal actor after the big blind by seat order. | deterministic | public actor only | Action order wraps through live seats. |

## Street and action sequence

Rust must generate every legal action tree and validate every submitted command.
TypeScript must not decide legality, action availability, contribution amounts,
street closure, hand strength, winner, split, or hidden-card visibility.

| Rule ID | Street or situation | Required sequence | Advances when |
|---|---|---|---|
| `RL-STREET-PREFLOP-002` | Preflop | Forced blinds are posted; action begins after the big blind. | Live non-folded seats have matched the current street contribution or checked where allowed. |
| `RL-STREET-FLOP-001` | Flop | Reveal exactly three public community cards, then run a small-unit contribution round. | The round closes. |
| `RL-STREET-TURN-001` | Turn | Reveal exactly one additional public community card, then run a big-unit contribution round. | The round closes. |
| `RL-STREET-RIVER-001` | River | Reveal exactly one final public community card, then run a big-unit contribution round. | The round closes and at least two live seats remain. |
| `RL-STREET-SHOWDOWN-001` | Showdown | Evaluate all showdown-eligible live seats and allocate the single pot. | Terminal outcome is recorded. |
| `RL-STREET-FOLDOUT-001` | Foldout | If all but one live seat folds, end immediately with a last-live-hand outcome. | The fold action is applied. |

| Rule ID | Situation | Legal actions | Rust-owned validation notes |
|---|---|---|---|
| `RL-BET-ACTION-001` | Active live seat may leave the hand. | `fold` | Folding removes the seat from live action and may trigger foldout. |
| `RL-BET-ACTION-002` | Active live seat owes no additional contribution and no bet is open. | `check`, `bet` | `bet` opens the street contribution at the current street unit. |
| `RL-BET-ACTION-003` | Active live seat faces an open contribution. | `call`, `raise`, `fold` | `call` matches; `raise` matches plus one street unit if the cap allows it. |
| `RL-BET-ACTION-004` | Street raise cap has been reached. | `call`, `fold`; `check` only if no amount is owed | Diagnostics may name the public cap, never hidden card facts. |
| `RL-BET-ACTION-005` | Terminal state. | none | Terminal states expose no normal gameplay actions. |
| `RL-BET-ACTION-006` | Wrong actor, malformed path, unavailable action, or stale command. | reject without mutation | Diagnostics are viewer-safe and deterministic. |

## Fixed-limit contribution rules

| Rule ID | Contribution rule | Timing | Notes |
|---|---|---|---|
| `RL-BET-LIMIT-001` | Preflop and flop use the small contribution unit. | preflop/flop | Unit labels are abstract. |
| `RL-BET-LIMIT-002` | Turn and river use the big contribution unit. | turn/river | Big unit amount is fixed by Rust rules. |
| `RL-BET-CAP-001` | Each street allows one opening bet plus at most three raises. | per street | The cap resets on each street. |
| `RL-BET-CALL-001` | Calling adds exactly the amount needed to match the current live street contribution. | action application | Invalid calls mutate nothing. |
| `RL-BET-RAISE-001` | Raising adds call amount plus exactly one current street unit. | action application | A fourth raise on a street is unavailable and rejected if submitted. |
| `RL-BET-CHECK-001` | Checking is legal only when the active seat owes no additional contribution. | action generation/validation | Browser controls must come from Rust legal actions. |
| `RL-POT-SINGLE-002` | All contributions feed one terminal pot in Gate 15. | whole hand | No side-pot model is created. |
| `RL-POT-ALLIN-001` | All-in handling is not available and states requiring it must not be generated. | whole hand | Contribution capacity is high enough for legal play. |

## Scoring and accounting

The terminal rationale uses these stable scoring IDs when explaining how the
single public ledger is awarded. Rust computes the result and TypeScript only
renders the supplied fields.

| Rule ID | Scoring/accounting rule | Applies to | Notes |
|---|---|---|---|
| `RL-SCORE-POT-AWARD` | Award the full single pot to the last live seat when a foldout ends the hand. | foldout terminal | Does not reveal folded seats' hole cards. |
| `RL-SCORE-SHOWDOWN` | Compare showdown-eligible seats by evaluated five-card category and ordered tie-break vector. | showdown terminal | Uses `RL-EVAL-*` evaluator rules. |
| `RL-SCORE-SPLIT` | Divide the single pot among seats tied for the best showdown hand. | split showdown terminal | Canonical tied winners remain in stable seat/evaluation order. Remainder assignment follows `RL-POT-REMAINDER-001`. |

## Hand evaluation and showdown

| Rule ID | Evaluation/showdown rule | Timing | Notes |
|---|---|---|---|
| `RL-EVAL-FIVE-001` | A five-card hand receives a category and ordered tie-break vector. | showdown/evaluator tests | Categories are high card, one pair, two pair, three of a kind, straight, flush, full house, four of a kind, and straight flush. |
| `RL-EVAL-ACELOW-001` | Ace-low straight is recognized as the lowest straight. | evaluator | Ace-high remains the highest straight. |
| `RL-EVAL-SEVEN-001` | A seat's best hand is found by enumerating the 21 five-card subsets of two hole cards plus five community cards. | showdown | No lookup-table evaluator in Gate 15. |
| `RL-EVAL-TIEBREAK-001` | Comparison uses category first, then ordered rank vector. | showdown | Suits never break ties. |
| `RL-EVAL-USED-001` | The exact five used cards are recorded for explanation where viewer-authorized. | showdown | Redaction rules still apply. |
| `RL-SHOW-ELIGIBLE-001` | Only live seats that have not folded before showdown are evaluated. | showdown | Folded seats are not evaluated for public result. |
| `RL-SHOW-WINNER-001` | The best evaluated hand wins the single pot when exactly one seat is strongest. | terminal | Rust computes and explains the decisive comparison. |
| `RL-SHOW-SPLIT-001` | Seats tied for best hand split the pot. | terminal | Equal shares are assigned first; tied winners are not re-ranked by payout order. |
| `RL-POT-REMAINDER-001` | Integer remainders from a split are assigned one unit at a time by stable button-order among tied winners. | terminal | Remainder allocation is public and explained. Button order selects odd-unit recipients only; it does not redefine the semantic winner order. |
| `RL-SHOW-FOLDOUT-001` | A last-live-hand terminal awards the pot without revealing folded seats' private hole cards. | terminal | Foldout explanation is distinct from showdown. |

## Terminal conditions

River Ledger has exactly three terminal result families in the base variant.
Each terminal result may emit a Rust-owned `terminal_rationale` with one of the
template keys documented in `UI.md`.

| Rule ID | Terminal result | Rationale template key | Decisive cause |
|---|---|---|---|
| `RL-END-LAST-LIVE` | One seat remains live after all other live seats fold. | `river_ledger.last_live_fold_win` | `last_live_after_folds` |
| `RL-END-SHOWDOWN` | The river betting round closes with two or more showdown-eligible seats, then Rust resolves a single winner or split. | `river_ledger.showdown_best_hand_win` / `river_ledger.showdown_split_pot` | `best_showdown_hand` / `equal_best_hand_split` |

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection |
|---|---|---|---|---|
| `RL-VIS-PUBLIC-001` | Seat order, button/blind roles, active seat, street, public board, public contributions, live/folded status, cap status, and terminal status. | observer and all seat viewers | when public | public view, seat view, action tree, diagnostics, effects, DOM, replay export |
| `RL-VIS-PRIVATE-HOLE-001` | A seat's own hole cards. | owning seat only before authorized showdown reveal | after setup | seat-private view and owner-authorized payloads only |
| `RL-VIS-OPPONENT-HOLE-001` | Other seats' unrevealed hole cards. | no unauthorized viewer | never before authorized showdown reveal | all browser-facing and replay surfaces |
| `RL-VIS-DECKTAIL-001` | Deck tail, future board identities, and burn advancement. | no browser-facing viewer | never | public/seat views, diagnostics, logs, traces exported to public viewers |
| `RL-VIS-DIAGNOSTIC-001` | Wrong-seat, stale, unavailable-action, and cap diagnostics. | requesting viewer if safe | after invalid command | diagnostics must cite public facts only |
| `RL-VIS-SHOWDOWN-001` | Showdown reveal and evaluated best hands. | all viewers for showdown-eligible revealed seats | showdown terminal | folded/non-revealed private data stays redacted |
| `RL-VIS-FOLDOUT-001` | Foldout terminal explanation. | all viewers | foldout terminal | folded seats' private cards remain redacted unless a future rule authorizes reveal |
| `RL-VIS-VIEWHASH-001` | Per-viewer hashes. | tooling/tests and safe payloads | after projection | Each viewer hash covers only that viewer's authorized projection. |

## Replay and serialization

| Rule ID | Replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `RL-REPLAY-RNG-001` | The seed, variant, seat order, setup options, and command stream reproduce internal state. | deterministic | internal for hidden facts | No wall-clock or browser randomness. |
| `RL-REPLAY-HASH-001` | State, effect, action-tree, and view hashes remain stable unless intentionally migrated. | deterministic | per authorized viewer | Hashes must not require leaking hidden cards to public viewers. |
| `RL-REPLAY-EXPORT-001` | Public replay export is redacted and cannot reconstruct unauthorized hole cards, deck tail, burn positions, or future board cards. | viewer-scoped | public export | Seat exports may include only that seat's authorized observations. |
| `RL-REPLAY-IMPORT-001` | Replay import validates command streams through normal Rust rules. | deterministic | viewer-scoped | TypeScript cannot repair or reinterpret commands. |
| `RL-REPLAY-SERIAL-001` | Serialization order for seats, cards, actions, effects, outcomes, and explanations is deterministic. | deterministic | mixed | Golden traces must catch accidental ordering drift. |

## Bots and strategy boundaries

Bot notes are not rule authority; they constrain later bot documents and code.

| Rule ID | Bot rule | Allowed inputs | Forbidden inputs |
|---|---|---|---|
| `RL-BOT-LEGAL-001` | Bots must choose from Rust-generated legal actions and submit normal action paths. | own authorized view and legal action tree | direct state mutation or bypassed validation |
| `RL-BOT-L0-001` | Level 0 chooses randomly among legal actions with deterministic seeded tie handling. | legal action tree | hidden opponent cards, deck tail, rollout sampling |
| `RL-BOT-L1-001` | Level 1 uses conservative public/own-hole heuristics. | own hole-card class, public board texture, call price, live-opponent count, street, cap pressure | opponent hole cards, future board cards, hidden deck identities |
| `RL-BOT-L2-001` | Level 2 uses the authored evidence-pack priority vector and deterministic tie-breaks. | fields named in the evidence pack | MCTS, ISMCTS, Monte Carlo, ML, RL, solvers, hidden-state sampling |
| `RL-BOT-ALLIN-001` | Bots distinguish stack-capped legal actions and return no action when all-in or terminal. | authorized seat view, public stack/call/eligibility facts, legal action metadata | opponent secrets, deck tail, bot-only legality, direct state mutation |
| `RL-BOT-EXPLAIN-001` | Bot explanations are viewer-safe. | public facts and, for owner-private rationale, own authorized hole-card buckets | opponent secrets, deck tail, unauthorized evaluated hands |

## UI and player-facing presentation

| Rule ID | UI rule | Requirement |
|---|---|---|
| `RL-UI-PRESENT-001` | The web UI presents Rust/WASM output only. |
| `RL-UI-SEATS-001` | Seat order, active/pending status, button, small blind, and big blind are rendered from Rust metadata/view fields. |
| `RL-UI-ACTIONS-001` | Legal action controls are built from Rust legal action trees. |
| `RL-UI-PREVIEW-001` | Previews, if shown, come from Rust and must be viewer-safe. |
| `RL-UI-LEDGER-001` | The contribution ledger displays abstract public counts only. |
| `RL-UI-SHOWDOWN-001` | Outcome and showdown explanation render Rust-authored fields without TypeScript winner or evaluator logic. |
| `RL-UI-NOCASINO-001` | Public presentation avoids casino trade dress, real-money framing, copied table/card art, tournament branding, rake/payout language, and affiliation implication. |
| `RL-UI-NOLEAK-001` | DOM, accessibility labels, test IDs, logs, storage, snapshots, and e2e traces must not contain unauthorized hidden card or deck facts. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Tests/traces required |
|---|---|---|---|
| `RL-SETUP-AMB-001` | Whether heads-up is an official mode. | No; official Gate 15 seat counts are 3-6. | seat-count validation tests |
| `RL-DEAL-AMB-001` | Whether burn cards are visible. | No; burn advancement is internal if modeled. | no-leak visibility tests |
| `RL-BET-AMB-001` | Whether the base model can enter all-in states. | No; contribution capacity avoids those states and all-in is Gate 15.1 scope. | rule/property tests |
| `RL-EVAL-AMB-001` | Whether royal flush is a separate category. | No; it is the highest straight flush unless docs intentionally add an alias later. | evaluator category tests |
| `RL-POT-AMB-001` | Whether split remainder uses suit, seat ID priority, or winner ranking. | No; stable button-order among tied winners assigns remainders only, while canonical tied winners remain in stable seat/evaluation order. | split remainder trace |
| `RL-VIS-AMB-001` | Whether folded hands reveal at foldout terminal. | No; last-live-hand terminal reveals no unauthorized folded hole cards. | foldout no-leak tests |

## Rulepath deviations from broad Hold'Em family variants

| Rule ID | Common variant behavior | River Ledger behavior | Why |
|---|---|---|---|
| `RL-VAR-SEATS-001` | Some tables allow two or more seats, often with heads-up-specific blind rules. | Gate 15 supports exactly 3-6 seats. | Proves N-seat scaling without heads-up special cases. |
| `RL-VAR-LIMIT-001` | No-limit and pot-limit variants exist. | Fixed-limit only. | Keeps action trees bounded and testable. |
| `RL-VAR-CAP-001` | Raise caps vary by venue/variant. | One opening bet plus three raises per street. | Fixed cap is explicit for replay, diagnostics, bots, and benchmarks. |
| `RL-VAR-ALLIN-001` | Many variants allow all-in and side pots. | All-in and side pots are absent. | Gate 15.1 owns that separate accounting pressure. |
| `RL-VAR-PRESENT-001` | Commercial presentation may use casino/product framing. | Rulepath uses original River Ledger identity and neutral abstract units. | IP and product posture. |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope rule or feature | Reason out of scope | Future review trigger |
|---|---|---|---|
| `RL-OOS-ALLIN-001` | All-in handling and side pots. | Deferred to Gate 15.1. | Accepted Gate 15.1 spec/tickets. |
| `RL-OOS-NOLIMIT-001` | No-limit or pot-limit contribution rules. | Gate 15 is fixed-limit and capped. | Future spec only. |
| `RL-OOS-TOURNAMENT-001` | Tournament/lobby/account/ranking structures. | Rulepath v1/v2 are static/local-first. | ADR for hosted multiplayer/product scope. |
| `RL-OOS-ENGINE-001` | Generic card/deck/betting/pot/evaluator engine helpers. | Mechanic-atlas review keeps River Ledger local; no promotion is authorized. | Mechanic-atlas hard-gate evidence. |
| `RL-OOS-BOT-001` | MCTS, ISMCTS, Monte Carlo, ML, RL, external solvers, or hidden-state sampling. | Public bot law forbids these. | ADR only. |
| `RL-OOS-BROWSER-001` | Browser legality, browser evaluator, browser winner/split computation, or browser hidden-card reconstruction. | Rust owns behavior. | None. |

## Rule coverage link

`games/river_ledger/docs/RULE-COVERAGE.md` maps these rule IDs to modules,
tests, traces, replay checks, UI proof, bots, and benchmarks. It is authored in
the admission-spine ticket and reconciled as implementation lands.
