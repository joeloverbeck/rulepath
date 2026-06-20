# River Ledger Competent Player Analysis

Game ID: `river_ledger`

Implemented variant: `river_ledger_standard`

Rules version checked: `river-ledger-rules-v2`

Date: 2026-06-20

## Purpose and Authority

This is strategy analysis for the implemented River Ledger base variant. It
informs the Level 2 authored bot but does not define rules. If this document
conflicts with [RULES.md](RULES.md), the rules win.

## Sources and References

| Source/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|
| [RULES.md](RULES.md) | 2026-06-14 | rules authority | legal actions, betting rounds, showdown, allocation | none | Local Rulepath prose. |
| [RULE-COVERAGE.md](RULE-COVERAGE.md) | 2026-06-14 | coverage plan | rule families and proof surfaces | none | Planned and implemented proof map. |
| `src/visibility.rs` | 2026-06-14 | executable projection boundary | authorized bot-visible fields | none | Public view and seat-private view fields only. |

## Competent-Player Summary

Competent River Ledger play is fixed-limit contribution management under
N-seat hidden information:

- choose only Rust-generated legal actions for the active seat;
- protect terminal safety by folding only when the public call price is poor;
- check when no contribution is owed and no pressure reason exists;
- call affordable prices to preserve showdown access, including call all-in when the stack is already committed by price;
- use opening bets and raises with strong own-hole classes, coordinated public
  boards, favorable street/cap pressure, or low live-opponent count;
- recognize that short all-in raises can add pressure without always reopening action;
- account for side-pot eligibility: a strong hand may still lose or win only the pots it is eligible to contest;
- tighten as live-opponent count rises because more hands can overtake one
  pair or high-card holdings;
- respect the three-raise cap and avoid spending the last raise without a
  clear public or own-hand reason;
- treat every opponent hole card, future board card, and deck-tail card as
  unknown.

## Phases and Situations

| Phase/situation | What competent players notice | Important rules | Notes |
|---|---|---|---|
| Preflop facing blinds | Own two hole cards, button/blind positions, public call price, live count. | blinds, call price, fixed small unit | Strong pairs/high broadway classes can apply pressure; weak disconnected lows prefer cheap calls or folds under pressure. |
| Flop | Three public cards, own-hole fit, call price, cap pressure. | small unit, board reveal | Made pair-or-better and strong draw texture support continuing; air facing raises should fold more often. |
| Turn | Four public cards and big unit. | big unit | The price doubles; continue with stronger made hands, high-quality draws, or cheap checks. |
| River | Full public board and final big-unit round. | river closure, showdown | No future improvement remains; decide from made hand strength, price, and opponent count. |
| Showdown/foldout | Terminal allocation and explanation are Rust-authored. | side pots, returns, split/remainder | Foldout reveals no folded private cards; showdown reveal and per-pot allocation are handled by Rust outcome records. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Bot feature candidate? |
|---|---|---|---:|
| Check free weak hands | No amount owed and weak own-hole/board fit | Preserves contribution units without surrendering equity. | yes |
| Call affordable public price | Facing a small price with reasonable own-hole/board fit | Keeps showdown access in a fixed-limit game. | yes |
| Fold poor price | Facing a big-unit or capped street price with weak fit | Avoids adding units with low expected strength. | yes |
| Bet strong own-hole class | No amount owed and strong pair/high-card class or made board fit | Extracts value and can narrow live opponents. | yes |
| Raise with cap pressure | Facing price and strong hand or short live-opponent count | Uses bounded pressure while the cap still permits it. | yes |
| Preserve eligibility awareness | Multi-way all-in or short-stack hand | A seat can win only pots where it is eligible, so later choices should consider whether added units contest new value. | yes |
| Avoid final raise with weak fit | Cap nearly spent and weak own information | Saves contribution units in high-uncertainty states. | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own two hole cards | yes | yes | yes | low | From `PrivateView::Seat.hole_cards`. |
| public board cards already revealed | yes | yes | yes | none | From `PublicView.board`. |
| own/other public contributions | yes | yes | yes | none | From `PublicView.seats` and `pot_total`. |
| starting/remaining stacks and all-in status | yes | yes | yes | none | Public accounting fields only. |
| pot tiers, eligibility, and returns | yes | yes | yes | none | Public allocation facts; no hidden hand strength. |
| legal action tree | yes | yes | yes | none | Candidate source only. |
| active/button/blind/seat status | yes | yes | yes | none | From `PublicView.active_seat`, role fields, and `SeatView.status`. |
| opponent hole cards | no | no | no | high | Never use, sample, summarize, or mention. |
| future community cards | no | no | no | high | `reserved_community_count` is a count only. |
| deck tail or burn/order facts | no | no | no | high | Only counts may appear in public projection. |
| hidden diagnostics or raw internal trace | no | no | no | high | Bot sees validated legal candidates, not private diagnostics. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation |
|---|---|---|---|---|
| `RL-S-EX-001` | Preflop, no pair, weak disconnected low cards, facing a raise | fold, call, raise | fold | Contribution price is poor and own-hole class is weak. |
| `RL-S-EX-002` | Flop, made top pair, affordable call | fold, call, raise | call | Preserve showdown access without spending cap pressure unnecessarily. |
| `RL-S-EX-003` | Turn, strong made hand, raise available, few live opponents | fold, call, raise | raise | Big-unit pressure is justified by own strength and low live count. |
| `RL-S-EX-004` | River, no amount owed, marginal hand, many live opponents | check, bet | check | No future improvement and many opponents make thin value pressure weaker. |
| `RL-S-EX-005` | Short stack faces more than it can cover | fold, call all-in | call all-in with playable own cards | The action preserves eligibility for pots the seat can contest without pretending it can match more than its stack. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Bot test implied? |
|---|---|---|---:|
| Raising whenever available | Fixed-limit caps are scarce and public. | Reserve raises for strength, price, and opponent-count reasons. | yes |
| Calling every big-unit price | Turn/river costs are larger. | Fold weak fit when price and live count are unfavorable. | yes |
| Ignoring side-pot eligibility | A seat cannot win pots beyond its committed cap. | Track which public pots the seat can contest and avoid reading aggregate allocation as one winner-take-all result. | yes |
| Acting as if future board cards are known | Leaks or assumes unavailable information. | Use only revealed board texture and counts. | yes |
| Treating all opponent counts equally | N-seat showdown risk changes with live count. | Tighten pressure/continuation as live count rises. | yes |

## Translation to Level 2 Bot Features

| Candidate feature | Visible to bot? | Authorized source | Used for | Hidden-info risk |
|---|---:|---|---|---|
| legal actions | yes | Rust action tree | candidate extraction | none |
| active seat, street, button/blinds | yes | `PublicView` role and phase fields | position and obligation context | none |
| own hole class | yes | `PrivateView::Seat.hole_cards` | preflop/fit pressure | low |
| revealed board texture | yes | `PublicView.board` | made hand/draw texture estimate | none |
| call price and contribution gap | yes | action metadata plus `SeatView` contributions | fold/call/raise posture | none |
| stack pressure and all-in metadata | yes | action metadata plus `SeatView` stack fields | distinguish ordinary actions from call all-in, short raise all-in, and no-action all-in states | none |
| pot eligibility and returns | yes | `PublicView` pot tiers and terminal allocation fields | explain side-pot pressure and terminal results | none |
| live-opponent count | yes | `SeatView.status` | tighten/loosen policy | none |
| cap pressure | yes | legal action availability and action metadata | avoid impossible or poor raises | none |

## Review Checklist

- Strategy prose is original.
- Rules authority is separate from strategy.
- Hidden-information boundaries are explicit.
- Every bot feature maps to an authorized view or legal-action field.
- No strategy claim requires MCTS, ISMCTS, Monte Carlo, ML, RL, solver output,
  hidden-state sampling, or TypeScript legality.
