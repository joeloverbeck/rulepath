# River Ledger Mechanics

Game ID: `river_ledger`

Public display name: `River Ledger`

Variant: `river_ledger_standard`

Created: 2026-06-14

Last updated: 2026-06-14

## Mechanic inventory

This inventory records Gate 15 River Ledger pressure for the repo-level mechanic
atlas. All mechanics are game-local unless the atlas later authorizes a narrow
`game-stdlib` helper. No promotion is authorized by this document.

| Atlas category | River Ledger shape | Status |
|---|---|---|
| topology/spatial model | No board topology. Seat order is a stable ring for action order, button/blind assignment, and split-remainder order. | game-local |
| component/zone model | Game-local 52-card deck; private per-seat hole-card zones; public community board; internal deck tail and optional burn advancement. | repeated-shape pressure; local |
| action shape | Flat Rust legal actions: `fold`, `check`, `call`, `bet`, `raise`, plus Rust-owned forced street advancement when rounds close. | game-local |
| turn/phase model | Preflop, flop, turn, river, showdown; action rotates through live seats; foldout can end early. | game-local |
| randomness/chance | Deterministic seeded shuffle at setup only; no browser randomness and no later random draw outside reserved deck order. | repeated-shape pressure; local |
| visibility/hidden information | Public observer plus each seat viewer; owner-only hole cards; redacted opponents, deck tail, burn, and future board; pairwise no-leak proof required for 3-6 seats. | repeated-shape pressure; local |
| resource/accounting | Abstract public contribution ledger, blinds, street contributions, one single pot, split allocation, deterministic remainder order. | repeated-shape pressure; local |
| movement/capture/placement | Not applicable. | not applicable |
| pattern/line/directional scanning | Not applicable. | not applicable |
| commitment/reveal | No simultaneous commitments. Staged reveal of public community cards and authorized showdown cards. | related hidden-info pressure; local |
| reaction/window/pending response | No interrupt or reaction window. Active actor advances by betting-state rules only. | not applicable |
| scoring/outcome | Last-live-hand foldout, single best-hand showdown winner, or tied showdown split with remainder explanation. | game-local |
| semantic effect shape | Public contribution/street/board/showdown/foldout effects plus viewer-filtered private setup/reveal effects. | game-local |
| UI interaction pattern | N-seat seat frame, public board, contribution ledger, legal-only controls, safe previews, Rust-authored outcome explanation. | game-local presentation |
| bot policy pattern | L0 legal-random; L1 conservative authorized-view heuristic; L2 authored opponent-count-aware heuristic; no search/RL/sampling. | game-local |
| benchmark/performance pressure | Setup, legal actions, apply, per-view projection, no-leak matrix, replay export/import, evaluator showdown batch, full 3-6 seat playouts. | game-local |

## Repeated-shape comparison

River Ledger is a new pressure point for several shapes already seen in public
games, but the planned implementation remains local.

| Shape | Comparison games | Similarity | Difference that keeps it local |
|---|---|---|---|
| deterministic shuffle with hidden holdings | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | Seeded card order and viewer-redacted private components. | River Ledger has 3-6 seats, two hole cards, five public board cards, street reveals, and showdown-specific authorization. A helper would either be trivial shuffle code or behavior-bearing visibility policy. |
| private-hand/public-reveal no-leak | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | Owner-private facts and redacted observer/opponent views. | River Ledger must prove every ordered seat pair in 3, 4, 5, and 6 seat matches plus public observer, replay, bot explanation, and browser surfaces. |
| public contribution accounting | `token_bazaar`, `poker_lite`, `event_frontier` | Public counters, payments/contributions, terminal allocation. | River Ledger has street-sized fixed-limit contributions, blinds, raise caps, live/folded eligibility, and split-remainder order. Gate 15.1 side pots are explicitly separate. |
| showdown/ranking explanation | `poker_lite`, `high_card_duel` | Rust computes terminal comparison and explains decisive facts. | River Ledger evaluates best five of seven cards with category/rank-vector comparison and split allocation. |
| N-seat public surface | Infra A-D shared surfaces | Uses supported seat counts and shared seat-frame expectations. | Game-specific action order, private-card visibility, and outcome rationale remain River Ledger behavior. |

## Atlas decision

Decision: `game-local / no promotion`.

Why not `engine-core`: card/deck/hand/street/pot/blind/button/evaluator and
contribution nouns are game mechanics. `engine-core` remains a generic contract
kernel.

Why not `game-stdlib`: the repeated shapes differ in reveal timing, seat count,
legal-action coupling, diagnostics, replay export, bot inputs, and terminal
explanations. A useful helper would risk encoding game policy; a behavior-free
helper is not yet worth promotion for Gate 15.

Debt status: no `game-stdlib` primitive is promoted, so there is no promotion
debt and the repo-level open debt register remains `_None_`.

Review trigger: reopen before Gate 15.1 side-pot/all-in work and again before
later trick-taking/private-hand/card games if repeated defects or a third close
shape proves a narrow behavior-free helper.
