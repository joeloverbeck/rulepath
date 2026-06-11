# Masked Claims - How to Play

_Game ID: `masked_claims`_
_Formal rules source: `games/masked_claims/docs/RULES.md`_
_Formal rules version checked: `masked-claims-rules-v1`_
_Strategy guide: `games/masked_claims/docs/COMPETENT-PLAYER.md`_

## At a glance

- Score points by claiming one of your hidden masks has a grade from 1 to 5.
- Each claim gives the other player a response window: accept it or challenge it.
- Accepted masks stay hidden forever and score the declared grade.
- Challenged masks reveal; honest claims reward the claimant, exposed lies reward the challenger.
- The game ends after eight claims, then public scores and public tiebreak counters decide the result.
- Unplayed masks, accepted masks, and the reserve are never exposed in the browser.

## What you can see

You can see your own hand, the public score, public counters, the current turn,
the claimant, any pending declared grade, veiled gallery counts and declared
grades, and any masks that were revealed by a challenge.

Your opponent cannot see your unplayed hand. Observers cannot see either hand.
Nobody sees the reserve. A mask accepted into a veiled gallery stays hidden for
the rest of the match, including at the final result.

## Setup

Masked Claims has two seats. The game creates fifteen masks: three masks at each
grade from 1 through 5. The deterministic setup shuffle deals five masks to each
seat and leaves five masks in an internal reserve. Seat 0 makes the first claim.

## On your turn

The game alternates between claim phases and response windows.

When you are the claimant, choose one mask from your hand and choose the grade
you want to declare for it. The declared grade becomes public, but the mask
identity stays hidden.

When your opponent claims, you become the responder. You may accept the claim or
challenge it. The claimant waits during this response window and has no gameplay
action until your response resolves.

## Actions

### Claim

Choose one mask from your hand and declare it as grade 1, 2, 3, 4, or 5. The
public table shows only the declared grade, not the mask identity.

### Accept

Accept the pending claim. The claimant scores the declared grade. The mask moves
to that claimant's veiled gallery and is not revealed later.

### Challenge

Challenge the pending claim. The mask reveals. If its actual grade is at least
the declared grade, the claim was honest and the claimant scores the actual
grade plus a truth bonus. If the actual grade is lower than the declared grade,
the lie is exposed and the responder scores the difference between the declared
grade and the actual grade.

## Scoring and winning

Accepted claim: the claimant scores the declared grade.

Honest challenged claim: if the revealed mask's actual grade is greater than or
equal to the declared grade, the claimant scores the actual grade plus 2.

Exposed lie: if the revealed mask's actual grade is lower than the declared
grade, the claimant scores 0 for that claim and the responder scores declared
grade minus actual grade.

The game ends after eight claims resolve. The higher final score wins. If the
score is tied, Rust applies these public tiebreakers in order:

1. Fewer exposed lies.
2. More successful challenges.
3. Fewer challenges declared.
4. If all of those are tied, the game is a draw.

The final explanation cites only public scores and public counters. It does not
reveal accepted masks, unplayed hand masks, or the reserve.

## Hidden information and reveal timing

Your own unplayed hand is visible to you only. Opponent hand masks are hidden.
The reserve is hidden from everyone and never appears in browser-facing output.

When a claim is pending, the declared grade is public but the mask identity is
hidden. If the response is accept, that mask enters a veiled gallery and remains
hidden forever. If the response is challenge, that one mask reveals and remains
public in the exposed row.

Replay exports and public command summaries redact claim mask IDs to declared
grades. Bot explanations may cite public facts and the bot's own allowed view,
but they must not reveal hidden masks.

## Common terms

| Term | Meaning |
|---|---|
| Mask | A hidden tile with a grade from 1 to 5. |
| Grade | The value of a mask. Higher grades are worth more. |
| Claim | A statement that one of your masks has a declared grade. |
| Pedestal | The public pending-claim area. It shows the claimant and declared grade, not the mask. |
| Response window | The moment after a claim when the other player chooses accept or challenge. |
| Accept | Let the claim score as declared without revealing the mask. |
| Challenge | Reveal the pending mask and score based on whether the claim was honest or exposed. |
| Veiled gallery | Accepted masks. Their declared grades/counts are public, but identities stay hidden. |
| Exposed row | Challenged masks that have been revealed. |
| Reserve | Undealt masks that are internal only and never revealed. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not
the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and
implementation details belong in the other game docs.

## Source notes for maintainers

- [x] Prose is original Rulepath wording.
- [x] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [x] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [x] No hidden match-state examples or seed-specific data.
- [x] No YAML front matter.
- [x] No selectors, conditions, triggers, or action schemas.
- [x] Formal rules version checked matches `RULES.md`.
