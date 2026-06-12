# Primitive Pressure Ledger: Event Frontier event complexity

Candidate name: `event-frontier-event-complexity`

Status: pre-implementation hard gates recorded; repeated shapes stay local; new shapes stay local-only

Decision date: 2026-06-12

Last updated: 2026-06-12

Prepared by: `Codex`

## Hard gate

This ledger records Gate 14's primitive-pressure decisions before any
`event_frontier` implementation code, typed card data, shuffle/setup code,
eligibility logic, operation logic, Reckoning logic, visibility code, replay
export, bot policy, WASM bridge, or browser UI is written.

Decision: keep Event Frontier mechanics local. No helper is added to
`engine-core` or `game-stdlib`. No promotion debt is created, so
`docs/MECHANIC-ATLAS.md` §10A remains `_None_`.

The repository-level record is
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) §10/§10B.

This ledger records two hard-gate reviews:

- public resource accounting / shared ledgers: third official public-accounting
  pressure is real, and extraction is explicitly deferred/rejected;
- multi-action turn budgets: Event Frontier is a non-use, because each faction
  makes one event-card choice and each operation is one compound command.

It records five second-use or repeated-shape comparisons:

- event-deck environment automation, compared with Flood Watch;
- role/faction modifiers, compared with Flood Watch and Frontier Control;
- graph-map topology / adjacency legality, compared with Frontier Control;
- site control, compared with Frontier Control;
- faction-asymmetric action sets and scoring, compared with Frontier Control.

It records four negative or no-op reviews:

- `game-stdlib::board_space` is not applicable;
- Event Frontier is not reaction-capable;
- Event Frontier is not a shared-outcome cooperative terminal use;
- Event Frontier does not repeat the full deterministic shuffle / private hand /
  staged reveal shape.

It records four first official uses:

- event-card initiative/eligibility sequencing;
- periodic scoring/reset pipeline;
- asymmetric instant victory conditions;
- timed rule-exception modifiers.

## Mechanic shape

Public resource-accounting hard gate:

```text
Event Frontier has public per-faction resources, operation payments, pass
income, Reckoning income, and public caps. This is a third public economy /
accounting pressure after Token Bazaar and Crest Ledger, but the accounting
shape is not the same helper shape: Token Bazaar is visible market purchase and
inventory conversion, Crest Ledger is shared-pool pledge/showdown allocation,
and Event Frontier is faction-owned operation funding plus income. A helper
would need policy branches for purchase markets, shared pots, faction resources,
income timing, terminal allocation, and operation affordability.
```

Multi-action-budget hard-gate candidate:

```text
Event Frontier is not a budgeted-turn use. A faction takes exactly one choice
per event-card opportunity: event, operation, limited operation, or pass. An
operation may be a progressive compound command, but it applies atomically as
one command and the legal tree does not regenerate through a remaining action
budget. If later code introduces regenerated budgeted commands, the hard gate
reopens before implementation continues.
```

`board_space` audit:

```text
Event Frontier uses named graph sites and trails. It has no rectangular
dimensions, row-major coordinate iteration, `rNcM` coordinate identity, or
generic board-space coordinate parsing. The promoted
`game-stdlib::board_space` primitive is therefore not applicable.
```

Negative and comparison reviews:

```text
Event Frontier has sequential eligibility, not a reaction window: no responder
tree, interrupt, cancellation, or pending response opens.

Event Frontier is competitive: one faction wins. It does not repeat Flood
Watch's all-seats-win or all-seats-lose shared terminal shape.

Event Frontier uses deterministic shuffle and public staged reveal, but it has
no per-seat private holdings. The undrawn deck order is hidden from every
viewer, not owner-private hand state.

Flood Watch's event automation is an environment batch after a turn-ending
command. Event Frontier's event deck is player-facing card flow plus automatic
Reckonings. Both stay game-local.

Flood Watch roles modify action magnitudes; Frontier Control factions have
disjoint action sets and asymmetric scoring; Event Frontier edicts are temporal
event-imposed modifiers and its factions have different operations and victory
conditions. These are related public modifier/asymmetry pressures, but not one
narrow helper.

Frontier Control has graph movement, connectivity scoring, and contest rules.
Event Frontier has a smaller graph used for adjacency legality and majority
presence, with no path-scored connectivity and no clash resolution.
```

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `three_marks`, `column_four`, `directional_flip`, `draughts_lite`; `frontier_control` audit | Gates 4-7, 13 | `game-stdlib::board_space` conformance and audit docs | promoted primitive audit | implemented | Event Frontier is another graph-site audit and is not applicable. |
| `token_bazaar` | Gate 9 | local resource market and scoring docs/tests | first public resource/accounting use | implemented | Public resources, costs, and conversions. |
| `poker_lite` | Gate 10 | pledge/shared-pool/showdown allocation docs/tests | second public resource/accounting use | implemented | Public shared-pool accounting and terminal allocation. |
| `event_frontier` | Gate 14 | planned `games/event_frontier/src/{actions,rules,effects,state}.rs` | third public resource/accounting pressure | admitted, not implemented | Defer/reject extraction; keep local. |
| `flood_watch` | Gate 12 | local budget, event automation, role modifier, visibility, bot modules | first budget/event/role use | implemented | Budgeted turn and event automation precedent. |
| `frontier_control` | Gate 13 | local graph, budget, faction/asymmetry, scoring modules | second budget; first graph/control/faction-asymmetry use | implemented | Graph/control/asymmetry comparison precedent. |
| `masked_claims` | Gate 11 | local reaction window modules | first reaction-window use | implemented | Event Frontier is not a second reaction use. |
| `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | Gates 8-11 | local shuffle/private holdings/reveal modules | deterministic private-hand/staged-reveal pressure | implemented | Event Frontier has hidden deck order but no private hands. |

## Local implementations compared

| Aspect | Token Bazaar | Crest Ledger | Event Frontier | Same shape? | Notes |
|---|---|---|---|---:|---|
| resource owner | per-seat/player public tokens | public pledges and shared pool | per-faction public funds/provisions | partial | All are public accounting; ownership and use differ. |
| payment shape | market purchases and conversions | bounded pledge additions and pool allocation | operation costs, pass income, Reckoning income | no | Different lifecycle and terminal semantics. |
| conservation/allocation | market/inventory accounting | shared-pool award/split handling | faction-owned pools capped at 9 | partial | Shared-pool allocation is not repeated. |
| action coupling | purchase legality | pledge/yield/showdown legality | operation affordability and edict costs | partial | Helper would need policy flags. |
| extraction risk | market-specific | pot/showdown-specific | event-operation-specific | high | Defer/reject keeps behavior readable locally. |

| Aspect | Flood Watch budgets | Frontier Control budgets | Event Frontier | Same shape? | Notes |
|---|---|---|---|---:|---|
| budget | active seat spends several actions | active faction spends two actions | no remaining action budget | no | Event Frontier has one choice per opportunity. |
| tree regeneration | after each budget action | after each budget action | progressive construction only before one command | no | Compound path construction is not a budgeted turn. |
| `end_turn` | legal budget action | legal budget action | no turn-budget end action | no | Passing is a card choice, not end-turn budget control. |
| waiting metadata | teammate waits | other faction waits | non-active faction waits during the other faction's choice | partial | Waiting metadata alone is not budget pressure. |

| Aspect | Flood Watch event deck | Event Frontier event deck | Same shape? | Notes |
|---|---|---:|---|
| automation trigger | environment batch after turn end/final budget | current card drives player choices; Reckonings auto-resolve | partial | Both use Rust-owned cards and effects. |
| hidden order | hidden from all until forecast/draw | hidden beyond current and next public cards | partial | Different reveal cadence, same no-leak discipline. |
| player choice | no event choice | event/op/pass eligibility menu | no | Event Frontier is initiative-driven. |
| helper pressure | first use | second comparison | no extraction | Keep local until a narrower behavior-free shape appears. |

| Aspect | Frontier Control graph/control/asymmetry | Event Frontier graph/control/asymmetry | Same shape? | Notes |
|---|---|---:|---|
| graph | site/edge graph with movement and connectivity scoring | site/trail graph with adjacency legality | partial | Event Frontier has no path-scored connectivity. |
| control | deterministic clashes, forts, stakes, supplied scoring | majority presence with no clashes | partial | Related public site control, distinct contest model. |
| factions | disjoint actions and scoring formulas | different operations and instant victory conditions | partial | Related but still game-specific policy. |
| bots | per-faction public policies | per-faction public policies | partial | Bot strategy remains local. |

## First-use local records

| Mechanic shape | Status | Current decision | Second-use revisit trigger |
|---|---|---|---|
| event-card initiative/eligibility sequencing | `local-only` first official use | Keep printed first-faction, eligibility markers, first/second menus, pass eligibility, and next-card consequences local to `games/event_frontier`. | Revisit when another official game repeats player-facing event-card eligibility sequencing. |
| periodic scoring/reset pipeline | `local-only` first official use | Keep Reckoning victory-check, site-scoring, income, edict-expiry, and eligibility-reset order local. | Revisit when another official game repeats scheduled scoring/reset cards or rounds with comparable replay pressure. |
| asymmetric instant victory conditions | `local-only` first official use | Keep different faction instant win checks, both-met rule, final fallback, and outcome rationale local. | Revisit when another official game repeats asymmetric instant victory conditions. |
| timed rule-exception modifiers | `local-only` first official use | Keep edict activation, stable modifier ordering, validation/application hooks, and expiry local. | Revisit when another official game repeats expiring rule exceptions. |

## Extraction decision

Decision: defer/reject resource-accounting extraction; record budget non-use;
keep comparison rows local; keep first-use shapes local-only.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for public resource accounting; no for budgeted turns; partial comparisons for graph/control/asymmetry/event decks/modifiers |
| helper can stay narrow and typed? | no for resource accounting at this gate |
| helper belongs in `game-stdlib`? | no |
| would contaminate `engine-core`? | yes if resource, card, deck, event, faction, site, eligibility, edict, scoring, or victory nouns moved there |
| static-data behavior risk? | high if a helper encouraged behavior-bearing card or scenario data |
| replay/hash impact acceptable? | yes for local implementation; no shared helper or migration is authorized |
| visibility/no-leak impact acceptable? | yes for local implementation with explicit hidden-order tests |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary change is made |

Rationale:

- Resource accounting is genuinely repeated, but the three uses differ in
  ownership, lifecycle, action coupling, allocation, and terminal semantics.
  A helper would either be too trivial to matter or broad enough to hide policy.
- Event Frontier is not a third budgeted-turn use. The hard-gate candidate is
  satisfied by this recorded non-use and reopens only if code drifts.
- Event decks, graph/site control, public modifiers, faction asymmetry, and
  hidden-order shuffle all have useful comparison value, but none justifies a
  helper in this gate.
- The new Event Frontier shapes are first official uses. Rulepath's first-use
  rule keeps them local.
- No existing trace/hash migration or prior-game conformance work is required.

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Promote public-resource accounting now | Token markets, pledge/shared-pool allocation, and faction operation funding are different enough that a helper would carry policy flags. | Reopen after another close economy/accounting use or repeated bugs prove a narrow contract. |
| Promote a budget helper | Event Frontier does not use budgeted turns. | If implementation drifts into regenerated budgeted commands, stop and reopen the hard gate. |
| Reuse `game-stdlib::board_space` | Event Frontier uses named graph sites, not rectangular coordinates. | Audit records not applicable. |
| Promote event-deck, edict, initiative, scoring, victory, graph, or faction helpers | These are first or second uses and too policy-bearing. | Keep game-local. |
| Move nouns into `engine-core` | Foundation law forbids mechanic nouns in the kernel. | `engine-core` remains unchanged. |
| Encode event or edict behavior in card data | Static data must remain typed content/parameters only. | Behavior stays in Rust. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## API sketch in prose only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/event_frontier` |
| determinism requirements | setup shuffle, card flow, operation order, edict order, Reckonings, effects, views, and replay remain deterministic locally |
| replay/hash requirements | no migration of existing games or trace hashes |
| visibility requirements | undrawn deck order projection and replay/export redaction remain game-local and Rust-owned |
| effect/log requirements | card reveal, choice, op, edict, Reckoning, resource, scoring, and terminal effects remain game-local |
| bot-facing notes | bots consume legal tree and public view only; no deck peeking or hidden-state sampling |
| non-goals | generic economy, budget, event deck, initiative, edict, graph, control, faction, scoring, victory, bot, or TypeScript legality helper |
| good-fit examples | none until a later ledger proves a repeated behavior-free shape with worthwhile conformance payoff |
| anti-examples | decide card effects from data, compute eligibility in TypeScript, expose deck order, define resource policies in a helper with flags |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | Preserve event/op/pass menu order, operation path order, edict ordering, and Reckoning order locally. | future action/rules tests and golden traces |
| diagnostics | Keep wrong-seat, stale, ineligible, menu, affordability, edict, operation, and terminal diagnostics viewer-safe. | future rules/visibility tests |
| semantic effects | Emit Rust effects for card reveal, choices, operations, edicts, resources, Reckonings, scoring, and terminal. | future effect/order traces and browser smoke |
| trace hashes | preserve existing games; create Event Frontier traces locally | future replay-check for `event_frontier` |
| serialization | stable local state/effect/view ordering | future serialization tests |
| seed/randomness | deterministic setup shuffle only; no random sampling after setup | future setup/replay tests |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | undrawn deck order must be absent beyond current/next public cards | future visibility/no-leak tests |
| action tree | legal choices must not identify deeper undrawn cards | future action-tree visibility tests |
| preview | previews must cite public card/site/resource/edict facts only | future preview/visibility tests |
| diagnostics | diagnostics must not reveal event order | future invalid-action tests |
| effect log | card identity appears only from current/next/reveal/discard timing | future golden traces/effect tests |
| DOM/test IDs/local storage/replay export | no deeper undrawn-card order/identity | future browser no-leak smoke and export tests |
| bot explanations/candidate rankings | no deck peeking or hidden sampling | future bot tests |
| dev inspector | viewer-safe public data only | future WASM/browser no-leak checks |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | game-local effect set for cards, choices, operations, edicts, Reckonings, and terminal | future `effects.rs`, WASM mirror, and `effectFeedback.ts` |
| animation mapping | event-card and Reckoning story animates from Rust effects | future `EventFrontierBoard.tsx` and E2E smoke |
| Rust-generated previews | eligibility, operation cost, edict annotations, and victory distances from Rust only | future `ui.rs`/WASM tests |
| UI controls/action tree mapping | constrained menus and progressive operation construction | future web smoke |
| reduced-motion behavior | preserve effect order with less motion | future E2E smoke |
| accessibility labels/summaries | public state only, no deck tail | future a11y/no-leak smoke |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | choose from Rust legal tree only | future bot legality tests |
| Level 1 baseline | separate public Charter and Freeholder policies | future strategy evidence and determinism tests |
| Level 2 authored policy | not claimed by Gate 14 | not applicable |
| Level 3 shallow deterministic search | not allowed/claimed | not applicable |

## Tests required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | future `RULE-COVERAGE.md` ticket |
| golden trace preservation/update notes | yes if behavior migrates | not applicable now |
| property/invariant tests | yes for local game implementation | future implementation tickets |
| replay/hash tests | yes for local game implementation | future implementation tickets |
| serialization tests | yes for local game implementation | future implementation tickets |
| visibility/no-leak tests | yes because undrawn deck order is hidden | future implementation tickets |
| bot tests | yes because bots are required | future implementation tickets |
| benchmark tests | yes for Gate 14 action-tree and playout budgets | future benchmark ticket |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing traces | existing official games | preserve | No helper, migration, or behavior change is authorized. | all prior mechanics |
| new Event Frontier traces | `event_frontier` | create locally later | Gate 14 requires new golden traces after implementation. | event flow, edicts, Reckonings, hidden order |

## Benchmarks affected

| Benchmark | Game(s) | Expected impact | Required threshold | Status |
|---|---|---|---:|---|
| existing game benchmarks | existing official games | none | unchanged | no migration |
| Event Frontier action tree / playout benchmarks | `event_frontier` | new local benchmark evidence | see Gate 14 benchmark ticket | future ticket |

## Examples

Good fits:

- Keep Charter funds, Freeholder provisions, edicts, eligibility, and Reckoning
  pipeline in `games/event_frontier`.
- Compare resource-accounting and budget pressure in docs before writing code.

## Anti-examples

Not a fit:

- A generic `ResourceLedger` that knows market purchases, shared-pool payouts,
  faction operation costs, income timing, and terminal allocation.
- A generic `EventDeck` that interprets card effects or edict conditions from
  data.
- TypeScript computing eligibility, affordability, edict effects, scoring, or
  victory.

## ADR need

ADR required? no

Reason:

- No architecture, replay/hash semantics, data policy, kernel boundary, browser
  authority, bot policy class, or public/private content policy changes.

ADR becomes required if a future proposal promotes helper APIs that change those
boundaries, moves mechanic nouns toward `engine-core`, changes visibility/export
contracts, or makes static card/scenario data behavior-bearing.
