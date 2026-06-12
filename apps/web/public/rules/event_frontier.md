# Event Frontier - How to Play

_Game ID: `event_frontier`_
_Formal rules source: `games/event_frontier/docs/RULES.md`_
_Formal rules version checked: `event-frontier-rules-v1`_
_Strategy guide: `games/event_frontier/docs/COMPETENT-PLAYER.md`_

## At a glance

- Event Frontier is a two-seat competitive game across six public sites.
- The Charter uses agents, depots, and funds.
- The Freeholders use settlers, caches, and provisions.
- Each public event card offers an event choice, operation choices, or a pass.
- Reckoning cards check instant victories, score site presence, pay income, and reset edicts.
- The current card and next public card are visible. Deeper undrawn deck order is hidden.

## What you can see

All seats and observers see the same public projection: seats, factions, sites,
trails, agents, settlers, depots, caches, public resources, public scores,
eligibility, active edicts, the current card, the next public card, discard
history, Reckoning count, victory-distance summaries, and terminal result.

The exact order of undrawn cards beyond the next public card is hidden from
every browser-facing viewer and bot. Replay export and terminal explanations do
not reveal that order.

## Setup

The standard game starts with six sites: Charterhouse, Landing, Crossing,
Granite Pass, High Meadow, and Old Mill. Trails connect those sites in a fixed
public graph.

Rust creates a seeded event deck in three epochs. Each epoch has one Reckoning
card and six event cards, and a Reckoning is never first in its epoch. Scenario
data supplies public starts, resource totals, thresholds, and labels; Rust owns
all legality, event behavior, edict behavior, scoring, and victory checks.

## On your turn

Only the faction named by the current card flow acts. A faction can be eligible
or ineligible for the current card. If the printed first faction is ineligible,
Rust offers the first choice to the other eligible faction. If no faction is
eligible, Rust discards the card unresolved and advances.

The first acting faction chooses the event, an operation, or pass. That first
choice constrains the second faction's menu. The browser presents only the Rust
legal action tree; it does not decide costs, eligibility, edict limits, or
victory.

## Actions

### Event

Resolve the current card's public typed event. Taking the event usually gives a
strong card effect now, but it affects future eligibility through the Rust card
flow.

### Charter operations

The Charter may survey, fortify, or writ when those actions are legal.

Survey places an agent at a legal adjacent site or at Charterhouse. Fortify
builds a depot where enough Charter agents are present and no depot already
exists. Writ removes one public cache from a site with Charter agent presence
and gains a fund.

### Freeholder operations

The Freeholders may trek, cache, or rally when those actions are legal.

Trek moves a settler along a public trail. Cache lays a cache at a legal
settler-occupied, depot-free site below the cache cap. Rally adds a settler at
Landing or at a public cache site, up to the settler cap.

### Pass

Pass gives the passing faction one resource, up to the cap, and preserves that
faction's eligibility. If both factions pass, Rust discards the current card.

## Costs and economy

Funds belong to the Charter. Provisions belong to the Freeholders. Both
resource pools are public, capped, and owned by Rust.

Passing gains one matching resource if the faction is below its cap. Operations
spend the printed resource cost before their effect resolves. Survey, fortify,
and writ spend Charter funds. Trek, cache, and rally spend Freeholder
provisions. Rust blocks unaffordable operations from the legal action tree.

Active edicts can change operation costs or block a class of operation until
the next Reckoning. Reckoning income then pays each faction from its public
presence and clears temporary edicts before the next card flow begins.

## Scoring and winning

At each Reckoning, Rust checks instant victories before site scoring. The
Charter wins instantly if it has majority presence at enough sites for the
variant threshold. The Freeholders win instantly if their public caches meet
the variant threshold. If both instant conditions are true at the same
Reckoning, the Freeholders win.

If no instant victory happens, each site awards one public score point to the
faction with strictly greater presence there. Charter presence is agents plus
depots. Freeholder presence is settlers. Caches help the Freeholder instant
condition, but caches do not count as site presence.

After the third Reckoning, if no instant victory has fired, the higher
cumulative score wins. Tied final fallback scores go to the Freeholders.

## Hidden information and reveal timing

The undrawn event deck order below the next public card is hidden after setup.
The current card and next public card are public. Discarded and resolved cards
remain public history.

No player view, action tree, effect payload, bot input, bot explanation, replay
export, browser storage, log, or terminal result may reveal the deeper undrawn
deck order. Public victory distances, remaining public components, and discard
history are legal public facts.

## Common terms

| Term | Meaning |
|---|---|
| Charter | The institutional faction that uses agents, depots, and funds. |
| Freeholders | The independent settler faction that uses settlers, caches, and provisions. |
| Site | A public map place connected by trails. |
| Agent | Charter presence at a site. |
| Depot | Charter structure that also counts as Charter presence. |
| Settler | Freeholder presence at a site. |
| Cache | Public Freeholder marker for the cache victory threshold. |
| Edict | A public event-imposed modifier active until the next Reckoning. |
| Reckoning | The card that checks instant victory, scores sites, pays income, expires edicts, and resets eligibility. |

## What this page is not

This page teaches player-facing flow. It is not the formal implementation
contract, not a strategy guide, and not a data schema.

Formal implementation details, coverage, bot evidence, source/IP notes, UI
presentation constraints, mechanic inventory, and implementation constraints
live in the other Event Frontier docs, including `MECHANICS.md`, `UI.md`, and
`AI.md`.
