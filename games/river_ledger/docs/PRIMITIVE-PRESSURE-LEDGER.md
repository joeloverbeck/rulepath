# River Ledger Primitive-Pressure Ledger

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Created: 2026-06-14

Last updated: 2026-06-14

## Decision summary

Mechanic shape: N-seat hidden private hands, deterministic shuffle/deal,
community-card reveal, fixed-limit contribution accounting, single-pot
allocation, seven-card showdown evaluation, viewer-safe outcome explanation.

Status: `rejected/deferred with rationale`.

Decision: keep River Ledger mechanics game-local for Gate 15. No `game-stdlib`
promotion and no `engine-core` vocabulary expansion are authorized.

Review owner/date: Codex, 2026-06-14.

## Games exerting pressure

| Shape | Prior / comparison games | River Ledger pressure |
|---|---|---|
| deterministic shuffle plus private holdings | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | 3-6 seats, two private hole cards, public board, no-leak matrix, replay export/import. |
| staged reveal and public/private projection | `high_card_duel`, `poker_lite`, `masked_claims` | Flop/turn/river plus authorized showdown reveal and foldout no-reveal terminal. |
| public accounting / allocation | `token_bazaar`, `poker_lite`, `event_frontier` | Blinds, fixed-limit street contributions, one pot, split/remainder order. |
| terminal comparison explanation | `high_card_duel`, `poker_lite` | Poker-category evaluator, rank-vector tie breaks, best-five used cards. |
| N-seat surface | Infra A-D shared seat/public infrastructure | Official 3, 4, 5, and 6 seat support with ordered pair no-leak proof. |

## What is repeated

- deterministic shuffled card-like components;
- owner-private hidden holdings;
- redacted observer/opponent projections;
- public contribution counters and terminal allocation;
- Rust-authored terminal rationale;
- legal-action-only bots with viewer-safe explanations.

## What differs

- River Ledger has variable official seat counts from 3 to 6.
- River Ledger uses two hole cards plus five public community cards, not a
  single private card, small custom deck, or trick hand.
- Legal actions are coupled to fixed-limit street contribution state and a
  street-specific raise cap.
- Showdown requires seven-card best-hand evaluation and split-remainder
  allocation.
- Foldout terminal must keep folded private cards redacted.
- Pairwise no-leak proof covers every ordered pair of distinct seats for four
  different seat counts.

## Why local duplication is acceptable now

The repeated shapes are real, but the shared subset is either too small
(deterministic shuffle) or too behavior-bearing (visibility, contribution
accounting, evaluator, terminal allocation). Promoting a helper now would invite
policy flags for reveal timing, betting eligibility, pot allocation, bot inputs,
and terminal explanation. Those policies belong in the game crate.

## Boundary impact

Why not `engine-core`: all named mechanics are game nouns. `engine-core` remains
generic and noun-free.

Why not `game-stdlib`: no narrow behavior-free helper has enough proven value
for Gate 15. Existing promoted `board_space` is not applicable. Public resource
accounting and deterministic private-hand shapes already have deferred/rejected
atlas decisions, and River Ledger does not change that decision at admission.

Data/Rust boundary impact: static data may describe variants, labels, fixtures,
and reports only. Betting, evaluation, visibility, and bot behavior remain
typed Rust.

Replay/hash impact: no shared helper changes existing replay or hash semantics.
River Ledger will add its own traces and hashes.

Visibility impact: visibility is the main risk and stays local until tests prove
whether any helper is safe.

Bot impact: bot policy remains game-local and authorized-view-only.

UI/effect impact: UI metadata and effects may share presentation shells, but
River Ledger behavior facts remain Rust-owned in the game crate.

## Required evidence before final gate close

- `games/river_ledger/docs/MECHANICS.md` complete and consistent with this
  ledger.
- Pairwise no-leak matrix for 3, 4, 5, and 6 seats.
- Contribution-ledger property tests and simulator evidence.
- Evaluator and split/remainder traces.
- Bot legality and bot-explanation no-leak tests.
- Final atlas review in the later `GAT15RIVLEDTEX-020` ticket.

## Back-port or conformance plan

No helper is promoted, so no prior game requires back-porting. No promotion debt
is opened.

Affected prior games: not applicable.

Exceptions: not applicable.

Closure gate if debt is deferred: not applicable because there is no promotion
debt. Reopen this ledger for Gate 15.1 side-pot/all-in work.

## Agent misuse risks

- Extracting a generic card/deck/hand helper before visibility and replay
  evidence exists.
- Moving contribution/pot vocabulary into `engine-core`.
- Encoding betting formulas, evaluator decisions, or visibility selectors in
  static data.
- Letting browser code infer legal actions, hand strength, winners, splits, or
  hidden facts.
- Treating "Texas Hold'Em family" as permission to copy prose, tables, or
  presentation.
