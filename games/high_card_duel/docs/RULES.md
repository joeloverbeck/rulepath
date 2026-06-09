# High Card Duel Rules

Game ID: `high_card_duel`

Public display name: `High Card Duel`

Implemented variant: `high_card_duel_standard`

Rules version: `high-card-duel-rules-v1`

Prepared by: `Codex`

Created: 2026-06-07

Last updated: 2026-06-07

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
| game id | `high_card_duel` |
| public display name | `High Card Duel` |
| variant | `high_card_duel_standard` |
| rules version | `high-card-duel-rules-v1` |
| source note | `games/high_card_duel/docs/SOURCES.md` |
| coverage matrix | `games/high_card_duel/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/high_card_duel/docs/MECHANICS.md` |
| implementation admission | `games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

High Card Duel is a two-seat, original Rulepath commitment game for proving
chance and hidden-information boundaries. Rust owns setup, deterministic
shuffle, private hands, hidden commitments, legal actions, reveal timing,
diagnostics, effects, replay behavior, and terminal scoring. TypeScript may
present only Rust/WASM output.

The game uses a neutral wayfarer duel-table theme. It does not use casino,
poker, blackjack, betting, chips, commercial card trade dress, or copied War
rulebook expression.

## Implemented variant

The only shipped variant is `high_card_duel_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| round limit | 6 |
| starting lead seat | `seat_0` |
| card count | 24 |
| ranks | numeric ranks `1` through `12` |
| sigils | two neutral identity sigils per rank |
| shuffle | deterministic Rust-owned shuffle from match seed |

## Components and game-local vocabulary

Game nouns in this section belong to `games/high_card_duel` only. They do not
authorize `card`, `deck`, `hand`, `rank`, `suit`, `pile`, or similar nouns in
`engine-core`.

The local duel deck contains two identities for each numeric rank:
`hcd:r01:a`, `hcd:r01:b`, `hcd:r02:a`, `hcd:r02:b`, through `hcd:r12:a` and
`hcd:r12:b`. Numeric rank controls comparison. Sigils distinguish otherwise
equal-rank identities and have no scoring or tie-break effect.

Public rank labels may be themed, but the numeric rank `1` through `12` must
remain visible in docs, tests, coverage, and accessible UI text.

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `HCD-SETUP-001` | Build the game-local 24-card deck in canonical sorted order. | deterministic | internal before projection | Canonical order is rank ascending, then sigil identity `a` before `b`. |
| `HCD-SETUP-002` | Shuffle the deck using deterministic Rust-owned randomness from the match seed. | random from seed | internal | The shuffle algorithm is Fisher-Yates version `hcd-shuffle-v1` using unbiased bounded indices; public projections must not reveal deck order. |
| `HCD-SETUP-003` | Deal three private cards to each seat, alternating `seat_0`, then `seat_1`, until both hands contain three cards. | deterministic from shuffled deck | private to each owner | The alternating deal order is part of replay and hash truth. |
| `HCD-SETUP-004` | Set round number to `1`, score to `0-0`, phase to `lead_commit`, and lead seat to `seat_0`. | deterministic | public | The reply seat is the other seat. |
| `HCD-SETUP-005` | Store remaining deck order internally only. Public and unauthorized projections may expose only deck count. | deterministic from shuffled deck | internal except count | Terminal public views still do not reveal unused deck order by default. |

## Round structure

Each game has exactly six rounds unless an accepted future variant changes the
round limit.

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `HCD-ROUND-001` | Start of each round. | Lead seat first. | Exactly one seat is lead and one seat is reply. | The lead commit phase begins. |
| `HCD-ROUND-002` | `lead_commit`. | Lead seat only. | The lead seat chooses one card from its own private hand and commits it face-down. | A valid lead commitment is stored. |
| `HCD-ROUND-003` | `reply_commit`. | Reply seat only. | The reply seat chooses one card from its own private hand without seeing the lead commitment identity. | A valid reply commitment is stored. |
| `HCD-ROUND-004` | Both commitments exist. | none; automatic resolution. | Both committed cards reveal simultaneously. | Reveal effects and public state are emitted together. |
| `HCD-ROUND-005` | Reveal comparison. | none; automatic resolution. | Higher rank wins the round and earns one point. | Score update is recorded. |
| `HCD-ROUND-006` | Reveal comparison with equal ranks. | none; automatic resolution. | If ranks tie, no point is awarded and the round is recorded as a tie. | Tie outcome is recorded. |
| `HCD-ROUND-007` | After reveal. | none; automatic resolution. | Revealed cards move to a revealed/discard history visible to all viewers. | The history keeps only cards that have become public. |
| `HCD-ROUND-008` | After scoring. | none; automatic resolution. | Refill hands up to three cards each from the internal deck while cards remain. | Private deal effects are emitted only to card owners. |
| `HCD-ROUND-009` | Refill step. | none; automatic resolution. | Refill order starts with the next round's lead seat, then alternates seats until both hands are full or the deck is empty. | Draw order is deterministic and internal card identities stay private. |
| `HCD-ROUND-010` | Lead assignment. | none; automatic resolution. | Lead seat alternates by round: odd rounds `seat_0`, even rounds `seat_1`. | The next lead/reply pair is public. |
| `HCD-ROUND-011` | Cleanup after nonterminal resolution. | none; automatic resolution. | Advance to the next round after refill and cleanup. | The next round enters `lead_commit`. |
| `HCD-ROUND-012` | Cleanup after round six. | none; automatic resolution. | After round six resolves, terminal state is reached. | No gameplay actions remain. |
| `HCD-ROUND-013` | Terminal scoring. | none. | Terminal winner is the higher score; equal score is a draw. | Unrevealed deck tail and private unplayed hands remain hidden in public exports. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `HCD-END-001` | Round six has resolved. | Terminal scoring. | The final score after six revealed rounds decides the outcome. | The browser must not compare scores as behavior authority. |
| `HCD-END-002` | One seat has a higher final score after the round limit. | Terminal scoring. | The higher-scoring seat wins. | Per-round explanation uses only revealed history. |
| `HCD-END-003` | Final scores are equal after the round limit. | Terminal scoring. | The match is a draw. | There is no tiebreaker. |

## Terminal conditions

Terminal public views also expose a Rust-owned outcome rationale. Wins use template key `high_card_duel.final_score_win`, decisive cause `final_score_after_round_limit`, final score, revealed per-round breakdowns, and rule IDs `HCD-ROUND-005`, `HCD-END-001`, and `HCD-END-002`. Draws use template key `high_card_duel.final_score_draw`, decisive cause `final_score_after_round_limit`, final score, revealed per-round breakdowns, and rule IDs `HCD-ROUND-006`, `HCD-END-001`, and `HCD-END-003`.

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `HCD-ACT-001` | Phase is `lead_commit`. | Only the active lead seat may commit. | private action tree for authorized actor | Observer and reply-seat views do not receive private lead choices. |
| `HCD-ACT-002` | Phase is `reply_commit`. | Only the active reply seat may commit. | private action tree for authorized actor | Reply choices are generated from the reply seat's own hand only. |
| `HCD-ACT-003` | Any commit action. | A commit action must identify one card currently in the actor's own private hand. | private action path containing actor-owned card identity | Opponent cards and deck identities are never valid action targets. |
| `HCD-ACT-004` | A seat already committed this round. | A seat may not commit twice in the same round. | no legal second commit | Duplicate submissions are rejected without mutation. |
| `HCD-ACT-005` | Viewer has no seat. | Observer/no-seat viewers have no legal private commit actions. | empty private tree | Public controls must not synthesize hidden choices. |
| `HCD-ACT-006` | Terminal state. | Terminal states have no legal gameplay actions. | empty gameplay tree | Terminal views preserve result only. |
| `HCD-ACT-007` | Terminal or not-applicable tool state. | Existing engine patterns for not-applicable/no-op actions may be used only if already required by cross-game tooling; otherwise terminal action tree is empty. | no-op only if tooling requires it | No new no-op convention is introduced for this game. |
| `HCD-ACT-008` | Action-tree projection. | Action-tree labels returned to an authorized actor may show that actor's own card identity. Public, observer, and opponent views must not receive those labels or paths. | viewer-scoped private labels | This includes UI, logs, replay exports, command summaries, and tests. |

## Invalid and stale actions

Diagnostics must be public-safe unless the viewer is explicitly authorized to
see the private fact. Invalid submissions must not mutate state.

| Rule ID | Invalid/stale situation | Diagnostic expectation | Visibility notes |
|---|---|---|---|
| `HCD-DIAG-001` | Wrong-seat action. | Return a public-safe wrong-seat diagnostic. | It may identify expected actor by seat ID because active seat is public. |
| `HCD-DIAG-002` | Wrong-phase action. | Return a public-safe phase diagnostic. | It may identify the public phase, not hidden card facts. |
| `HCD-DIAG-003` | Invalid private card identity. | Return a redacted diagnostic to unauthorized viewers and include the card identity only in the acting seat's private diagnostic if safe. | Opponent and observer diagnostics must not echo hidden IDs. |
| `HCD-DIAG-004` | Stale action. | Use existing stale-action conventions and do not leak current hidden state. | Stale command summaries must remain redacted. |
| `HCD-DIAG-005` | Occupied or missing commitment conflict. | Report the reason class without revealing opponent card identity before reveal. | A face-down commitment is public; its card identity is not. |
| `HCD-DIAG-006` | Browser-visible diagnostics. | Logs, dev panels, test IDs, DOM attributes, and replay command summaries must use redacted public tokens unless viewer authorization is explicit. | No private card ID may appear on unauthorized surfaces. |

## Viewer projections

High Card Duel defines three viewer modes:

| Viewer mode | Engine viewer |
|---|---|
| observer | `Viewer { seat_id: None }` |
| seat 0 | `Viewer { seat_id: Some(seat_0) }` |
| seat 1 | `Viewer { seat_id: Some(seat_1) }` |

Observer/public projection includes only game ID, variant ID, round number,
round limit, phase, active/lead/reply seat IDs, scores, hand counts, deck count
if included, face-down commitment occupancy before reveal, revealed cards after
reveal, public effects, public-safe status, and terminal result.

Seat-private projection includes observer fields plus that seat's private hand
card identities, that seat's own committed card identity after commit, legal
private action affordances while acting, and private effects addressed to that
seat.

Seat-private projection must not include opponent private hand identities,
opponent face-down committed card identity before reveal, unrevealed deck order,
future draw identities, opponent bot candidates or explanations, or internal
state that reconstructs hidden cards.

## Effects

Use existing `EffectLog` visibility semantics.

| Effect | Visibility | Purpose |
|---|---|---|
| `hcd_deal_private_card` | `PrivateToSeat(owner)` | Owner learns card identity. |
| `hcd_hand_count_changed` | `Public` | Everyone sees hand count changes. |
| `hcd_commit_face_down` | `Public` | Everyone sees that a seat committed. |
| `hcd_own_commit_confirmed` | `PrivateToSeat(owner)` | Owner may see own committed card. |
| `hcd_cards_revealed` | `Public` | Both committed cards become public simultaneously. |
| `hcd_round_scored` | `Public` | Score or tie update. |
| `hcd_refill_started` | `Public` | Optional refill marker with no hidden identities. |
| `hcd_terminal` | `Public` | Final result. |
| `hcd_private_diagnostic` | `PrivateToSeat(owner)` | Optional private diagnostic that still must not expose opponent or deck facts. |
| `hcd_public_diagnostic` | `Public` | Redacted public-safe diagnostic. |

Do not emit private card identities in public effect payloads, text, keys, CSS
classes, DOM attributes, test IDs, console logs, storage, replay exports, or
command summaries.

## Replay and randomness notes

Setup uses deterministic Rust-owned randomness from the match seed. The shuffle
algorithm is Fisher-Yates version `hcd-shuffle-v1`, using an unbiased bounded
index for each shrinking interval. Same seed plus same variant must reproduce
the same internal deck order, deals, commands, effects, hashes, and internal
test traces. Different seeds may produce different deals.

Internal full traces may contain seed and private action choices for native
tests, fixtures, and golden replay checks. Public browser replay exports for
observer mode must contain only public projections, public effects, and redacted
command summaries. They must not contain unrevealed deck order, private hands,
private card IDs before reveal, hidden commitments before reveal, full seed
material that reconstructs hidden cards, bot private candidates, or raw action
paths containing private card IDs.

Terminal public exports do not reveal all hidden information by default. Unused
deck tail and unplayed private hands remain hidden unless a later accepted
postgame reveal policy defines an authorized viewer and passes no-leak tests.

## Bot-relevant non-authoritative strategy notes

A Level 0 bot may choose randomly among the normal Rust-generated legal action
tree for its authorized seat and submit through the normal command path. Bot
logic must use only the acting seat's allowed private view plus public state. No
bot may inspect opponent private cards, unrevealed deck order, future draws, or
internal state not available to that viewer.

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every stable rule ID in this document must appear in `RULE-COVERAGE.md`. Silent
gaps are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-07 |
