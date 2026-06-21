# Primitive Pressure Ledger: fixed-four trick-taking

Candidate name: `briar-circuit-fixed-four-trick-taking`

Status: second-use comparison recorded; extraction deferred/rejected

Decision date: 2026-06-21

Last updated: 2026-06-21

Prepared by: `Codex`

## Second-Use Review

This ledger records Gate 16's second-use primitive-pressure decision for Briar
Circuit. Plain Tricks is the first official close use of follow-suit legality,
led-suit trick resolution, trick-winner-led turn order, and deterministic
trick-round redeal. Briar Circuit repeats those shapes, but with fixed four
seats, a standard 52-card deck, simultaneous private passing, first-trick point
restrictions, hearts-broken lead restrictions, penalty scoring, shoot-the-moon
transformation, and cumulative threshold play.

Decision: defer/reject extraction and keep the mechanics local.

No helper is added to `engine-core` or `game-stdlib`. No promotion debt is
created. Gate 17 Oh Hell is the next close trick-taking use and must resolve the
third-use hard gate before coding.

## Mechanic Shapes

Repeated shapes under review:

```text
follow-suit legality from the actor's private hand;
led-suit highest-card trick comparison;
trick winner leads the next trick;
deterministic deal/redeal rotation across trick rounds or hands;
hidden-hand projection with played-card public reveal.
```

The repeated shapes are real. The extraction boundary is not clean enough for a
Gate 16 helper because the behavior-bearing exceptions are central to the game.

## Games Exerting Pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `plain_tricks` | Gate 10.1 | `games/plain_tricks/src/actions.rs`, `rules.rs`, `setup.rs`, `state.rs`, `visibility.rs` | first official follow-suit/trick proof | implemented | Two seats, 18-card original deck, six tricks per round, positive trick-count scoring, no pass, no point-card restrictions, no hearts-broken state. |
| `briar_circuit` | Gate 16 | planned `games/briar_circuit/src/actions.rs`, `rules.rs`, `setup.rs`, `state.rs`, `scoring.rs`, `visibility.rs` | second close trick-taking use | admitted, not implemented | Four seats, 52-card deck, private passing, 13 tricks per hand, point restrictions, penalty scoring, moon, multi-hand threshold. |

## Local Implementations Compared

| Aspect | `plain_tricks` | planned `briar_circuit` | Same shape? | Notes |
|---|---|---|---:|---|
| seat count | exactly two seats | exactly four seats | partial | Briar Circuit adds ordered pairwise no-leak across 12 seat pairs plus observer. |
| deck | 18 original cards in three neutral suits | standard 52-card game-local deck | partial | Card identity, rank ordering, and suit set differ. |
| deal shape | six cards per seat, six-card hidden tail, two rounds | 13 cards per seat, no remainder, dealer rotates every hand | partial | Both are deterministic hidden-hand deals, but exhaustion and rotation semantics differ. |
| pass/commitment | none | left/right/across/hold pass cycle with exactly three private selections per seat | no | Passing is a major Briar-only hidden commitment surface. |
| follow-suit legality | follower must play led suit when holding it | every non-leader must play led suit when holding it, after opening/first-trick restrictions | yes, with exceptions | Shared core exists; Briar Circuit has more preconditions and point-card exceptions. |
| first play | fixed first leader by round | holder of 2 clubs must lead 2 clubs | partial | Briar Circuit needs a card-identity opening constraint. |
| hearts broken | not applicable | heart-led restriction until a heart is played, with all-hearts exception | no | Behavior-bearing state, diagnostics, and effects are game-local. |
| trick comparator | highest rank in led suit wins; one follower | highest rank in led suit wins among four cards | yes, with cardinality difference | A helper would still need game-specific card/rank types and no-trump policy. |
| trick winner leads | winner leads next trick unless round closes | winner leads next trick unless hand closes | yes | Turn-order shape repeats. |
| scoring | one positive point per trick; higher total wins or split | penalty card totals, moon transform, cumulative low score wins | no | Scoring is materially different. |
| terminal | fixed two-round terminal, 6-6 split | threshold after complete hand, tied low continues | no | No shared terminal helper. |
| visibility | owner hand, public played cards, hidden tail | owner hand, pass selections/provenance, played cards, hidden deck material | partial | Pass provenance and four-seat matrix are new pressure. |
| bots | L0 plus authored trick policy | L0 plus bounded L1; L2 not admitted | partial | Bot input and explanation policies differ. |

## Similarities

- Both games use Rust-owned legal action trees and validation for follow-suit.
- Both games reveal card identities publicly only when cards are legally played.
- Both games resolve tricks by led suit and rank, with off-suit cards unable to
  win.
- Both games make the trick winner the next leader.
- Both games need deterministic replay/hash evidence and no-leak browser proof.

## Differences

- Briar Circuit adds simultaneous private passing and pass provenance privacy.
- Briar Circuit's legal set combines opening lead, follow-suit, first-trick
  point restriction, hearts-broken lead restriction, and all-hearts exception.
- Briar Circuit is a four-seat game with ordered pairwise no-leak obligations.
- Briar Circuit scores captured point cards, transforms moon hands, accumulates
  penalties across hands, and may continue after threshold ties.
- A shared helper broad enough to cover the differences would need policy hooks
  for pass timing, point-card exceptions, heart-breaking state, scoring, and
  outcome semantics.

## Extraction Decision

The recorded decision is defer/reject extraction and keep local.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for follow-suit, led-suit comparator, winner-leads, and deterministic trick-hand deal rotation |
| helper can stay narrow and typed? | not enough to justify promotion for Gate 16 |
| helper belongs in `game-stdlib`? | no for Gate 16 |
| would contaminate `engine-core`? | yes if card, suit, rank, hand, trick, pass, or heart nouns moved there; therefore forbidden |
| static-data behavior risk? | high if lead restrictions, point-card rules, pass routing, or scoring become configurable tables |
| replay/hash impact acceptable? | no extraction impact is justified; Plain Tricks traces/hashes should remain untouched |
| visibility/no-leak impact acceptable? | yes for local implementation; extraction would need new proof surfaces |
| benchmarks support extraction? | no; no benchmark evidence proves shared hot-path value |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary change is made |

Rationale:

- The common comparator and follow-suit core are small compared with the
  behavior-bearing exceptions.
- A helper would either be a trivial comparison utility with little payoff or a
  policy-bearing mini-engine for trick games.
- Keeping Briar Circuit local preserves the foundation boundary: typed game
  modules own mechanics, and Gate 17 will decide the third-use hard gate with
  two real implementations available for comparison.

## Rejected Alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse an existing promoted primitive | No promoted trick-taking helper exists. | Decision option 1 does not apply. |
| Promote follow-suit legality now | Briar Circuit's opening/first-trick/hearts-broken exceptions are central behavior, not metadata. | Reopen at Gate 17. |
| Promote led-suit comparator now | Comparator alone is too small to justify conformance work and still needs game-local card/rank types. | A future behavior-free helper may be reconsidered with benchmark/bug evidence. |
| Promote a trick engine | Would encode pass, lead restriction, scoring, terminal, visibility, and bot policy hooks. | This would violate the no-DSL/no-behavior-data boundary. |
| Escalate to ADR | No architecture or kernel boundary change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## Back-Port And Conformance Plan

No back-port is required because no helper is promoted.

Affected prior games:

- `plain_tricks`: no behavior, trace, hash, action path, or renderer change.
  Its docs gain this second-use comparison reference only.

Exceptions:

- None. This is a defer/reject decision, not a promoted primitive with
  exceptions.

Closure gate if debt is deferred:

- Not applicable. No promotion debt is created, so `docs/MECHANIC-ATLAS.md`
  §10A remains `_None_`.

## Tests Required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | later Briar Circuit implementation tickets |
| golden trace preservation/update notes | yes if promoted | not applicable now; no migration |
| property/invariant tests | yes | later Briar Circuit property tests |
| replay/hash tests | yes | later Briar Circuit replay tests |
| serialization tests | yes | later Briar Circuit serialization tests |
| visibility/no-leak tests | yes | later native, WASM, and browser evidence |
| bot tests | yes | later Briar Circuit bot tests |
| benchmark tests | yes | later Briar Circuit benchmarks |

## Traces Affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `plain_tricks` | preserve | No shared helper or migration. | Plain Tricks trick-taking pressure |
| future `games/briar_circuit/tests/golden_traces/*.trace.json` | `briar_circuit` | create locally | Local implementation must prove deterministic setup, pass, trick, score, visibility, and replay/export behavior. | `BC-*` rules |

## Agent Misuse Risks

| Risk | Guardrail |
|---|---|
| Second-use comparison is mistaken for permission to promote a helper. | This ledger approves no helper and creates no promotion debt. |
| A future agent encodes trick legality in data. | `RULES.md` and this ledger require Rust-owned behavior and strict data parsing. |
| Browser code filters legal card choices by suit or point status. | TypeScript legality remains forbidden; controls present Rust leaves only. |
| Pass provenance leaks through views, exports, DOM, or bot explanations. | Later no-leak tests must sweep every named surface. |
| Gate 17 starts without resolving the third-use hard gate. | This ledger names Gate 17 as the next hard-gate trigger. |

## ADR Need

ADR required? no

Reason:

- This ledger records no architecture, replay/hash, data-policy, kernel,
  browser-authority, bot-policy, or public/private-content change. It keeps the
  behavior local and records the next review trigger.

## Next Review Trigger

Reopen this decision before Gate 17 Oh Hell begins trick-taking behavior
implementation, or earlier if implementation uncovers one of these facts:

- Plain Tricks and Briar Circuit duplicate the same bug-prone follow-suit or
  comparator code in a way that a behavior-free helper could fix;
- benchmark evidence shows repeated hot-path cost that a narrow helper can
  improve without policy hooks;
- a maintainer proposes any runtime helper that would affect pass timing, lead
  restrictions, point-card rules, visibility, replay export, bot input, or
  scoring.

## Gate 17 Helper-Conformance Addendum

Date: 2026-06-21

Gate 17 Vow Tide created the third close use of follow-suit selection and
trick-winner comparison. The repository-level decision in
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) promotes the
narrow pure `game-stdlib::trick_taking` helper:

- `follow_suit_indices` selects stable held-card indices matching the led suit,
  or every held-card index when the hand is void.
- `winning_play_index` selects the stable winning play index from
  caller-projected suit/rank values. Briar Circuit uses it with `trump = None`.

Briar Circuit now adopts both helper functions for the pure repeated core while
retaining every Hearts-family policy locally:

- the 2 clubs opening requirement remains in `rules.rs` before helper selection;
- first-trick point-card and hearts-broken lead restrictions still filter the
  caller-owned local legal set after the base follow-suit decision;
- local `CardId`, `Suit`, `Rank`, seats, phases, diagnostics, effects,
  visibility projection, pass commitment/exchange, scoring, terminal, replay,
  bot policy, and UI surfaces remain Briar Circuit owned;
- trick-winner-led turn order and deal/redeal/dealer rotation remain explicit
  anti-examples, not promoted helper behavior.

Verification receipt for this addendum is recorded in
`archive/tickets/GAT17VOWTIDOHHEL-004.md`. The intended proof is unchanged Briar
Circuit tests, visibility tests, replay-check hashes, and native benchmark
execution after the helper swap. No §10A promotion debt is opened because Gate
17 performs matching prior-game conformance in-gate.
