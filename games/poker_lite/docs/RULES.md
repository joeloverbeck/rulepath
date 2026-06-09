# Crest Ledger Rules

Game ID: `poker_lite`

Public display name: `Crest Ledger`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Prepared by: `Codex`

Created: 2026-06-09

Last updated: 2026-06-09

## Rule authority

This document is the original Rulepath rules summary for the implemented
variant. Sources belong in `SOURCES.md`; this document states the Rulepath
implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

## Metadata

| Field | Value |
|---|---|
| game id | `poker_lite` |
| public display name | `Crest Ledger` |
| variant | `poker_lite_standard` |
| rules version | `poker-lite-rules-v1` |
| source note | `games/poker_lite/docs/SOURCES.md` |
| coverage matrix | `games/poker_lite/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/poker_lite/docs/MECHANICS.md` |
| implementation admission | `games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Crest Ledger is a two-seat, original Rulepath hidden-information pledge game.
Each seat receives one private crest, one center crest starts hidden and later
becomes public, and seats decide whether to hold, press, lift, match, or yield
through two bounded pledge rounds. The game proves deterministic hidden-card
setup, public shared-pool accounting, viewer-safe reveal timing, deterministic
showdown, Rust-owned legality, and browser-safe no-leak presentation.

Rust owns setup, legal actions, validation, private card storage, center reveal
timing, pledge accounting, terminal allocation, showdown comparison, semantic
effects, replay behavior, visibility projection, and bot decisions. TypeScript
may present only Rust/WASM output.

The game does not implement a general card engine, casino poker, real-money
stakes, blinds, rake, stacks, side pools, tournaments, configurable variants,
more than two seats, or shared engine vocabulary for this game's local nouns.

## Implemented variant

The only shipped variant is `poker_lite_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| deck | six crests: three ranks, two copies per rank |
| private crests | one per seat |
| center crest | one, hidden until round 1 closes without yield |
| opening contribution | 1 marker from each seat |
| pledge rounds | two |
| round units | round 1 = 1 marker; round 2 = 2 markers |
| lift cap | at most one lift per round |
| maximum contribution | 7 markers per seat |
| terminal outcomes | yield win, showdown win, or split |
| randomness | deterministic seeded shuffle only |

## Components and game-local vocabulary

Game nouns in this section belong to `games/poker_lite` only. They do not
authorize `card`, `deck`, `hand`, `bet`, `pot`, `raise`, `fold`, `showdown`,
`pool`, `pledge`, `marker`, or similar nouns in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `CL-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | No other player counts ship in this variant. |
| `CL-COMP-002` | crest | One game-local card-like component with a stable id, rank, copy, and neutral label. | private, hidden, or public depending on timing | The public UI uses crest language rather than casino language. |
| `CL-COMP-003` | rank | One of `low`, `middle`, or `high`, with values 1, 2, and 3. | visible only with a visible crest | Used by showdown comparison. |
| `CL-COMP-004` | copy | One of `dawn` or `dusk` for each rank. | visible only with a visible crest | Copy identity distinguishes the two crests of each rank. |
| `CL-COMP-005` | private crest | The one crest dealt to a seat during setup. | private to owner until showdown; never publicly revealed on yield | Owner seat view may show only its own private crest. |
| `CL-COMP-006` | center crest | The one crest dealt face down during setup. | hidden until round 1 closes without yield; public afterward | The deck tail is never inferred from it. |
| `CL-COMP-007` | deck tail | The three crests left after private and center deal. | internal only | It is not needed for legal decisions after setup. |
| `CL-COMP-008` | marker | The abstract accounting unit contributed to the shared pool. | public as counts only | No real-money or casino value is implied. |
| `CL-COMP-009` | shared pool | The public total of contributed markers waiting for terminal allocation. | public | Accounting is exact and Rust-owned. |
| `CL-COMP-010` | pledge round | One of two bounded action rounds with a fixed marker unit and one-lift cap. | public | Round 1 precedes center reveal; round 2 precedes showdown. |
| `CL-COMP-011` | outstanding pledge | A public marker amount one seat must match, lift, or yield against. | public | It is derived only from public contributions. |
| `CL-COMP-012` | lift cap | The per-round limit allowing at most one lift. | public | Resets between round 1 and round 2. |
| `CL-COMP-013` | terminal allocation | The final shared-pool award or split. | public | Yield terminal does not reveal hidden private crests. |

### Standard crest list

Static data may carry crest IDs, labels, ranks, copies, stable order, metadata,
fixtures, and version declarations. Static data must not carry legality,
accounting, showdown, bot, hidden-info routing, selector, trigger, or formula
behavior.

| Stable order | Crest ID | Rank | Copy | Label | Rank value |
|---:|---|---|---|---|---:|
| 1 | `low_dawn` | `low` | `dawn` | Sprout Dawn | 1 |
| 2 | `low_dusk` | `low` | `dusk` | Sprout Dusk | 1 |
| 3 | `middle_dawn` | `middle` | `dawn` | Current Dawn | 2 |
| 4 | `middle_dusk` | `middle` | `dusk` | Current Dusk | 2 |
| 5 | `high_dawn` | `high` | `dawn` | Crown Dawn | 3 |
| 6 | `high_dusk` | `high` | `dusk` | Crown Dusk | 3 |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `CL-SETUP-001` | Create exactly two seats, `seat_0` and `seat_1`. | deterministic | public | No other seat counts ship. |
| `CL-SETUP-002` | Construct the six-crest deck in stable order, then shuffle it with Rulepath's deterministic seeded RNG discipline. | seeded deterministic | internal until projection | Same seed and rules version must reproduce setup. |
| `CL-SETUP-003` | Deal the top shuffled crest to `seat_0`, the next to `seat_1`, the next to the center, and leave the remaining three as the internal deck tail. | seeded deterministic | mixed | Private crests, hidden center, and deck tail are not public at setup. |
| `CL-SETUP-004` | Set phase to pledge round 1, active seat to `seat_0`, center visible to false, and terminal outcome to none. | deterministic | public except hidden center identity | Round index is stored as zero-based internally if desired. |
| `CL-SETUP-005` | Set each seat's opening contribution to 1 marker and the shared pool to 2 markers. | deterministic | public | Opening contributions are automatic. |
| `CL-SETUP-006` | Initialize the round state with no outstanding pledge and an unused lift cap. | deterministic | public | The first actor may hold or press. |
| `CL-SETUP-007` | Emit private setup effects for each owner crest and public setup effects containing only counts and public marker totals. | deterministic | mixed | Public setup effects must not contain crest ids or ranks. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `CL-TURN-001` | Start of pledge round 1. | `seat_0` | The active seat may hold or press because no outstanding pledge exists. | A valid action is applied. |
| `CL-TURN-002` | No outstanding pledge after one seat has held. | the other seat | The other seat may hold or press. | A second hold closes the round, or a press creates an outstanding pledge. |
| `CL-TURN-003` | A seat faces an outstanding pledge. | facing seat | The facing seat may match, yield, or lift if the round lift cap is unused. | Match closes the round, yield ends the game, or lift passes an outstanding pledge back. |
| `CL-TURN-004` | Round 1 closes without yield. | none during resolution | Rust reveals the center crest in a public grouped effect, marks it visible, advances to pledge round 2, makes `seat_1` the round lead, and resets round pledge state. | Resolution completes. |
| `CL-TURN-005` | Start of pledge round 2. | `seat_1` | The second round repeats the pledge sequence using a 2-marker unit and a fresh one-lift cap. | A valid action is applied. |
| `CL-TURN-006` | Round 2 closes without yield. | none during resolution | Rust emits a grouped showdown reveal, compares strengths, allocates the shared pool, and enters terminal state. | Terminal outcome is recorded. |
| `CL-TURN-007` | Terminal state. | none | Expose outcome and no normal gameplay actions. | No further gameplay action advances the game. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality,
availability, accounting, reveal timing, showdown result, terminal outcome,
tie handling, hidden-info filtering, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `CL-ACT-001` | Active seat has no outstanding pledge. | `hold`, `press` | flat action tree leaf paths | `hold` adds no markers; `press` adds the current round unit and creates an outstanding pledge. |
| `CL-ACT-002` | Active seat faces an outstanding pledge. | `match`, `yield`; `lift` if the lift cap is unused | flat action tree leaf paths | `match` equalizes current-round contribution; `yield` ends immediately; `lift` matches plus one round unit and consumes the cap. |
| `CL-ACT-003` | A round already used its lift cap. | `match`, `yield` only when facing an outstanding pledge | flat action tree leaf paths | A second lift in the same round is unavailable and rejected if submitted. |
| `CL-ACT-004` | Terminal state. | none | empty gameplay tree | Terminal states expose no normal gameplay actions. |
| `CL-ACT-005` | Any generated legal action. | safe public metadata only | action metadata | Metadata may include action family, round index, round unit, actor, required-to-match, adds-to-pool, pool-after, lift-cap status, center-visible boolean, and accessibility copy. It must not include crest id, rank, deck tail, hidden center identity, opponent strength, inferred state, or bot ranking. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `CL-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only `seat_0` and `seat_1` can act. |
| `CL-RESTRICT-002` | The wrong seat submits while another seat is active. | Reject without mutation. | Viewer-safe wrong-seat diagnostic. | Diagnostic may name public active seat only. |
| `CL-RESTRICT-003` | A malformed or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action diagnostic. | Diagnostic must not include hidden crest facts. |
| `CL-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `CL-RESTRICT-005` | A second lift is submitted in the same pledge round. | Reject without mutation. | Viewer-safe lift-cap diagnostic. | Lift cap status is public. |
| `CL-RESTRICT-006` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Pledge accounting and reveal resolution

| Rule ID | Resolution rule | Effect | Notes |
|---|---|---|---|
| `CL-PLEDGE-001` | A hold with no outstanding pledge adds no markers. | Active seat changes to the other seat unless the second hold closes the round. | The shared pool is unchanged. |
| `CL-PLEDGE-002` | A press with no outstanding pledge adds the current round unit for the actor. | Shared pool and actor contribution increase; the other seat faces an outstanding pledge. | Round 1 unit is 1; round 2 unit is 2. |
| `CL-PLEDGE-003` | A lift while facing an outstanding pledge adds the amount needed to match plus one current round unit. | Shared pool and actor contribution increase; the other seat faces the new outstanding pledge; the round lift cap is consumed. | At most one lift per round. |
| `CL-PLEDGE-004` | A match while facing an outstanding pledge adds exactly the amount needed to equalize current-round contribution. | Shared pool and actor contribution increase; the current round closes. | Showdown contributions are equal by construction when round 2 closes this way. |
| `CL-PLEDGE-005` | A yield while facing an outstanding pledge ends the game immediately. | Non-yielding seat wins the current shared pool. | No private crest reveal occurs because of yield. |
| `CL-REVEAL-001` | Round 1 close without yield reveals the center crest to all viewers in one public grouped reveal. | Center visible becomes true and round 2 begins with `seat_1` active. | Private crests and deck tail remain hidden. |
| `CL-REVEAL-002` | Round 2 close without yield starts showdown with one grouped public reveal. | Both private crests and the already public center crest are available to resolve terminal allocation. | No partial private reveal ordering is exposed. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `CL-SCORE-001` | Opening contributions add 1 marker per seat, setting the shared pool to 2. | setup | none | Public accounting starts immediately. |
| `CL-SCORE-002` | Press, lift, and match add exact marker amounts to both the acting seat contribution and shared pool. | action application | Reject invalid actions without mutation. | Static data carries no formulas. |
| `CL-SCORE-003` | Maximum contribution is bounded at 7 markers per seat. | whole game | opening 1 + round 1 max 2 + round 2 max 4 | The action system must not allow unbounded pledge growth. |
| `CL-SCORE-004` | At showdown, each seat's strength is `(pair_flag, private_rank_value)`, where `pair_flag` is true only if that seat's private rank equals the center rank. | round 2 close | pair beats no pair; higher private rank breaks same pair flag | Copy identity does not break ties. |
| `CL-SCORE-005` | A true equal-strength showdown splits the shared pool exactly in half. | showdown terminal | No seat-priority allocation for split. | Equalized showdown contributions make the pool even. |
| `CL-SCORE-006` | A yield awards the current shared pool to the non-yielding seat. | yield terminal | Private crests remain unrevealed publicly. | The terminal effect may name yield actor and winner only. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `CL-END-001` | A seat yields while facing an outstanding pledge. | `YieldWin` for the non-yielding seat. | none | The yielded private crest is not publicly revealed. |
| `CL-END-002` | Pledge round 2 closes without yield and one seat has the higher showdown strength. | `ShowdownWin` for the stronger seat. | `CL-END-003` if strengths tie | Reveal contains both private crests and center crest. |
| `CL-END-003` | Pledge round 2 closes without yield and both seats have equal showdown strength. | `Split` with `each = shared_pool / 2`. | exact split | The terminal outcome is a split, not a priority win. |

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `CL-VIS-001` | Shared pool, per-seat contributions, active seat, round index, round unit, outstanding amount, lift-cap status, center visibility status, and terminal status. | observer and both seat viewers | after setup and after each state projection | public view, seat view, action tree, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `CL-VIS-002` | A seat's private crest. | owning seat only before showdown; all viewers at showdown; never public on yield terminal | setup for owner, showdown for all if reached | public view, opponent view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Opponent and observer payloads must not contain private crest id, rank, copy, or label before showdown. |
| `CL-VIS-003` | Hidden center crest. | no public or seat-facing viewer before center reveal | round 1 close without yield | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Before reveal, only hidden/count status is allowed. |
| `CL-VIS-004` | Deck tail. | no browser-facing viewer | never | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Native internal tests may inspect it under test authority. |
| `CL-VIS-005` | Legal pledge choices. | active actor through Rust-authorized action tree | nonterminal active turn | action tree and controls | Choices depend only on public pledge state, not hidden cards. |
| `CL-VIS-006` | Showdown reveal. | all viewers | after round 2 closes without yield | public view, effects, replay export, DOM | Reveal is grouped and includes both private crests together. |
| `CL-VIS-007` | Yield terminal. | all viewers | immediately after yield | public view, effects, replay export, DOM | Public payload may show winner, loser, pool, and already revealed center only. |
| `CL-VIS-008` | Bot rationale and candidate ranking. | public only if projected by Rust as viewer-safe text/data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Public rationale may cite legal action family and public pledge facts only. Private actor rationale may cite own strength bucket, never opponent hidden crest or deck tail. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `CL-RNG-001` | Crest Ledger uses a deterministic seeded shuffle for setup and no later random draw. | Same seed, rules version, variant, and command sequence must reproduce internal state and effects. | internal for hidden setup facts | No wall-clock or nondeterministic input is allowed. |
| `CL-RNG-002` | Public replay export is viewer-scoped and redacted. | Public exports before showdown or after yield must not include seed material that reconstructs private crests, hidden center, or deck tail. | public export is redacted | Seat-scoped export may include only that seat's own visible private observations. |
| `CL-RNG-003` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | mixed | Stable order covers crests, actions, effects, and view summaries. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. They guide `COMPETENT-PLAYER.md`,
`BOT-STRATEGY-EVIDENCE-PACK.md`, and `AI.md`. Strategy claims must be checked
against rule IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| Before center reveal | High private rank can justify controlled pressure; low private rank should often hold unless matching is cheap. | `CL-ACT-001`, `CL-ACT-002`, `CL-VIS-002`, `CL-VIS-003` | May use own private crest only; no hidden center or opponent crest. |
| After center reveal | A paired own crest is a strong visible-to-owner bucket; no-pair low is weak. | `CL-SCORE-004`, `CL-VIS-002`, `CL-VIS-003` | May use public center and own private crest only. |
| Facing outstanding pledge | Required-to-match and shared-pool size can guide match/yield/lift choices. | `CL-PLEDGE-003`, `CL-PLEDGE-004`, `CL-PLEDGE-005`, `CL-VIS-001` | May not infer or enumerate opponent private crest. |
| Multiple legal choices rank equally | Choose by stable documented ordering. | `CL-ACT-005`, `CL-RNG-003` | Tie-breaking must not sample hidden state. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `CL-AMB-001` | Whether the owning seat may see its private crest before showdown. | Yes, only that owning seat's private view may show its own private crest. | Gate 10 seat-view requirement. | visibility/no-leak tests, seat-private-view trace, browser smoke | Opponent and observer views stay redacted. |
| `CL-AMB-002` | Whether the center crest is revealed if round 1 ends by yield. | No. Yield terminal occurs immediately and reveals no new hidden crest. | Gate 10 yield-terminal requirement. | yield-terminal-no-showdown trace | Public view may show only already public facts. |
| `CL-AMB-003` | Whether yielded private crests become public after terminal. | No. Yield terminal never publicly reveals private crests. | Gate 10 no-leak requirement. | yield terminal no-leak tests and public export tests | Owner seat-local view may still show its own private crest. |
| `CL-AMB-004` | Whether a same-strength showdown uses seat priority. | No. Equal strength splits exactly. | Gate 10 split requirement. | tie-split trace | Copy identity does not break ties. |
| `CL-AMB-005` | Whether static data may encode pledge or showdown formulas. | No. Data carries typed content and metadata only; formulas live in Rust. | Rulepath static-data boundary. | variant unknown-field tests and boundary review | Behavior-looking keys are rejected. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `CL-VAR-001` | Commercial casino games often include money framing, stacks, blinds, side pools, many seats, and branded table presentation. | Crest Ledger uses abstract markers, a tiny fixed deck, two seats, no stacks beyond bounded contributions, no real-money framing, and neutral board-game presentation. | The gate proves hidden-info accounting and showdown without casino product scope. | yes |
| `CL-VAR-002` | Some benchmark games use source names and established terminology. | Public docs and UI use original Crest Ledger terms such as crest, marker, pledge, shared pool, hold, press, lift, match, and yield. | Avoid copied expression, public confusion, and casino trade dress. | yes |
| `CL-VAR-003` | Some games reveal all private cards when a player gives up. | Yield terminal reveals no additional private crests. | This is the safer no-leak proof and keeps the terminal meaningful. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `CL-OOS-001` | Texas Hold'em, Omaha, draw poker, tournaments, blinds, rake, cash balances, payouts, side pools, all-in logic, and multiplayer tables. | The gate is a small original two-seat proof, not a public poker engine. | Accepted spec or ADR only. |
| `CL-OOS-002` | Generic card/deck/hand, betting, pool, or showdown helpers. | Repeated mechanic pressure is not sufficient for extraction yet, and `engine-core` stays noun-free. | Mechanic atlas third-use review or accepted ADR. |
| `CL-OOS-003` | MCTS, ISMCTS, Monte Carlo equity simulation, ML, RL, opponent-card enumeration, or hidden-state sampling. | Public v1/v2 bots must not cheat or introduce prohibited policy forms. | None for this gate. |
| `CL-OOS-004` | Browser legality, browser showdown comparison, browser accounting, or browser hidden-card reconstruction. | Rust owns behavior and TypeScript presents Rust/WASM output only. | None. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| none | not applicable | Initial rule ID set. | not applicable | 2026-06-09 |
