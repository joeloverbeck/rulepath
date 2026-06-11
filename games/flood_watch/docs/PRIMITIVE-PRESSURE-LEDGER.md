# Primitive Pressure Ledger: Flood Watch cooperative event pressure

Candidate name: `flood-watch-cooperative-event-pressure`

Status: pre-implementation reviews recorded; first-use shapes stay local-only

Decision date: 2026-06-11

Last updated: 2026-06-11

Prepared by: `Codex`

## Hard gate

This ledger records Gate 12's primitive-pressure decisions before any
`flood_watch` implementation code, static scenario files, shuffle code,
visibility code, replay/export code, bot policy, WASM bridge, or browser UI is
written.

Decision: keep Flood Watch mechanics local. No helper is added to
`engine-core` or `game-stdlib`. No promotion debt is created, so
`docs/MECHANIC-ATLAS.md` §10A remains `_None_`.

The repository-level record is
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) §10B.

This ledger records two negative reviews:

- Flood Watch is not a second reaction-window/pending-response use.
- Flood Watch is not a fifth use of the deterministic shuffle / private hand /
  staged reveal shape.

It also records four first official uses:

- shared-outcome cooperative terminal;
- event-deck environment automation;
- role-modified action effects;
- multi-action turn budgets.

## Mechanic shape

Negative reaction-window review:

```text
Flood Watch has an environment automation phase, not a pending response. No
seat responds to another seat's pending action, no interrupt/cancel window
opens, and the teammate receives only safe waiting metadata while the active
seat spends budget.
```

Negative deterministic-shuffle/private-holdings review:

```text
Flood Watch has deterministic shuffle and staged public reveal through forecast
and event draw effects, but it has no per-seat private holdings. The event deck
order is hidden from everyone until reveal/draw and is not a hand, private
inventory, or owner-view surface.
```

First-use shapes:

```text
shared terminal outcome where all seats win or lose together;
deterministic environment automation resolved as a Rust consequence of a seat
command;
public role modifiers that change local action magnitude;
multi-action turn budget with legal-tree regeneration after each action.
```

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `masked_claims` | Gate 11 | `games/masked_claims/src/actions.rs`, `rules.rs`, `visibility.rs`; `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` | first reaction-window use | implemented | Claim opens one responder-only accept/challenge window; claimant waits. |
| `flood_watch` | Gate 12 | planned `games/flood_watch/src/actions.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `bots.rs` | negative reaction-window review; first cooperative/event/budget/role uses | admitted, not implemented | Environment automation is not a seat response. |
| `high_card_duel` | Gate 8 | setup, visibility, effects, replay/export docs and tests | deterministic private-card/staged-reveal pressure | implemented | Private hands plus staged reveal. |
| `poker_lite` | Gate 10 | setup, visibility, effects, replay/export docs and tests | deterministic private-card/staged-reveal pressure | implemented | Private crests plus center/showdown reveal. |
| `plain_tricks` | Gate 10.1 | setup, actions, visibility, effects, replay/export docs and tests | deterministic private-hand/staged-reveal hard gate | implemented | Private hands plus public plays and hidden tail. |
| `masked_claims` | Gate 11 | setup, actions, visibility, effects, replay/export docs and tests | fourth deterministic hidden-component/staged-reveal reopen | implemented | Private mask hands, hidden reserve, accepted masks never reveal. |
| `flood_watch` | Gate 12 | planned setup, visibility, effects, replay/export docs and tests | negative fifth-use review | admitted, not implemented | Hidden deck order has no per-seat private holdings. |

## Local implementations compared

| Aspect | `masked_claims` reaction window | planned `flood_watch` | Same shape? | Notes |
|---|---|---|---:|---|
| trigger | A claimant's public claim opens a responder window. | The active seat spends budget or ends turn, then Rust resolves environment automation. | no | Automation is not another seat's response. |
| active legal tree | Responder receives accept/challenge. | Active seat receives budgeted bail/reinforce/forecast/end-turn leaves; teammate waits. | no | Flood Watch has no responder-only choice. |
| waiting seat | Claimant waits while responder acts. | Teammate waits while active seat continues budgeted actions. | partial | Waiting metadata repeats, but no pending response repeats. |
| resolution | Response choice controls accept/challenge resolution. | Rust draws and resolves event cards deterministically. | no | No seat chooses the environment outcome. |
| helper pressure | First use; stays local. | Not a second use. | no | The atlas trigger remains armed. |

| Aspect | prior hidden-hand games | planned `flood_watch` | Same shape? | Notes |
|---|---|---|---:|---|
| shuffle | Deterministic shuffle of game-owned opaque components. | Deterministic shuffle of event cards. | partial | Shuffle repeats at a low level. |
| private holdings | Hands/crests/masks are private to seats. | No per-seat private holdings. | no | Deck order is hidden from all seats. |
| reveal timing | Reveals depend on commitments, showdown, plays, or challenge. | Forecast and event draw reveal public event cards. | partial | Staged reveal exists but not private-hand reveal/export. |
| redacted export | Public export redacts private holdings and hidden residue. | Public export redacts undrawn event order. | partial | Similar no-leak discipline, different lifecycle. |
| bot input | Bots may use own private holdings where allowed. | Bots may use only public state, forecast, composition counts, and legal tree. | no | No hidden-state sampling or deck peeking. |

## First-use local records

| Mechanic shape | Status | Current decision | Second-use revisit trigger |
|---|---|---|---|
| shared-outcome cooperative terminal | `local-only` first official use | Keep terminal outcome local to `games/flood_watch`; no shared-outcome helper. | Revisit when `frontier_control` or another official game adds asymmetric/team/shared victory comparison pressure. |
| event-deck environment automation | `local-only` first official use | Keep event resolution, event effects, and automation sequencing local to `games/flood_watch`. | Revisit when `event_frontier` or another official game repeats event decks or periodic automation. |
| role-modified action effects | `local-only` first official use | Keep role IDs, action magnitudes, validation, previews, effects, and bot priorities local to `games/flood_watch`. | Revisit when `frontier_control` faction powers or another official game repeats public role/faction modifiers. |
| multi-action turn budgets | `local-only` first official use | Keep budget accounting and legal-tree metadata local to `games/flood_watch`. | Revisit when `event_frontier`, `frontier_control`, or another official game repeats budgeted turns. |

## Extraction decision

Decision: defer/reject extraction for the reviewed repeated rows; keep first-use
shapes local-only.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | no for reaction-window; no for deterministic shuffle plus private holdings plus redacted reveal/export |
| helper can stay narrow and typed? | not applicable for first uses; not justified for the negative reviews |
| helper belongs in `game-stdlib`? | no |
| would contaminate `engine-core`? | yes if event, deck, card, role, scenario, district, flood, levee, budget, environment, cooperative, or shared-outcome nouns moved there |
| static-data behavior risk? | medium if scenario data encoded event effects, role powers, legality, automation scripts, or terminal logic; current plan forbids that |
| replay/hash impact acceptable? | yes for local implementation; no shared helper or migration is authorized |
| visibility/no-leak impact acceptable? | yes for local redaction; no runtime visibility helper is authorized |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no; no helper extraction is proposed |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary change is made |

Rationale:

- Flood Watch's teammate waiting state is not enough to count as a reaction
  window. No responder receives accept/challenge/interrupt choices and no seat
  answers another seat's pending action.
- Flood Watch repeats deterministic shuffle and staged reveal in a narrow
  sense, but it does not repeat per-seat private holdings. The row's fifth-use
  trigger stays armed for a future game that repeats the full triple.
- The four new shapes are first official uses. Rulepath's first-use rule keeps
  them local until a later official game creates real comparison pressure.
- A helper broad enough to cover event decks, automation, role powers, budgets,
  shared terminal outcomes, visibility, bot policy, and UI effects would become
  a behavior language with game-policy flags.

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Promote a reaction-window helper | Flood Watch is not reaction-capable, and `masked_claims` remains the first official use. | Atlas reopen trigger is preserved. |
| Promote a deterministic event-deck helper | The existing shuffle row concerns deterministic shuffle plus private holdings plus redacted reveal/export; Flood Watch lacks private holdings. | Reopen only if helper pressure appears without behavior policy. |
| Promote shared outcome, automation, role, or budget helpers | Each is a first official use. | First use must stay local. |
| Move nouns into `engine-core` | Foundation law forbids mechanic nouns in the kernel. | `engine-core` remains unchanged. |
| Encode event behavior or role powers in scenario data | Static data must remain typed content/parameters only. | Behavior stays in Rust. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## API sketch in prose only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/flood_watch` |
| determinism requirements | Flood Watch must use Rulepath deterministic RNG locally and prove replay stability |
| replay/hash requirements | no migration of existing games or trace hashes |
| visibility requirements | undrawn event order projection and replay/export redaction remain game-local and Rust-owned |
| effect/log requirements | event, levee, flood, forecast, terminal, and budget effects remain game-local |
| bot-facing notes | bots consume legal tree and public view only; no deck peeking or hidden-state sampling |
| non-goals | generic event deck, automation phase, role power, action budget, shared outcome, cooperative bot, TypeScript legality |
| good-fit examples | none until a later ledger proves a repeated behavior-free shape with worthwhile conformance payoff |
| anti-examples | decide event effects from data, choose an environment actor, infer legality in TypeScript, expose deck order, sample hidden deck order for bots, generalize shared outcome into `engine-core` |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | Preserve local action path order; regenerate budgeted legal tree after each action. | future action/rules tests and golden traces |
| diagnostics | Keep wrong-seat, stale, invalid-target, invalid-forecast, and terminal diagnostics viewer-safe. | future rules/visibility tests |
| semantic effects | Emit Rust effects for forecast, turn end, environment begin, event draw, levee absorption, flood rise, inundation, deck exhaustion, and terminal. | future effect/order traces and browser smoke |
| trace hashes | preserve existing games; create Flood Watch traces locally | future replay-check for `flood_watch` |
| serialization | stable local state/effect/view ordering | future serialization tests |
| seed/randomness | deterministic setup shuffle only; event draws are replayed from state | future setup/replay tests |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | undrawn deck order must be absent | future visibility/no-leak tests |
| action tree | legal choices must not identify undrawn cards | future action-tree visibility tests |
| preview | previews must cite public district/role facts only | future preview/visibility tests |
| diagnostics | diagnostics must not reveal event order | future invalid-action tests |
| effect log | card identity appears only from forecast or draw | future golden traces/effect tests |
| DOM/test IDs/local storage/replay export | no undrawn-card order/identity | future browser no-leak smoke and export tests |
| bot explanations/candidate rankings | no deck peeking or hidden sampling | future bot tests |
| dev inspector | viewer-safe public data only | future WASM/browser no-leak checks |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | game-local effect set for automation and terminal | future `effects.rs`, WASM mirror, and `effectFeedback.ts` |
| animation mapping | environment batch animates from Rust effects | future `FloodWatchBoard.tsx` and E2E smoke |
| Rust-generated previews | role magnitudes and public district facts only | future `ui.rs`/WASM tests |
| UI controls/action tree mapping | budgeted leaves and teammate waiting state | future web smoke |
| reduced-motion behavior | preserve effect order with less motion | future E2E smoke |
| accessibility labels/summaries | public state only, no deck tail | future a11y/no-leak smoke |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | choose from Rust legal tree only | future bot legality tests |
| Level 1 baseline | public priority policy only | future strategy evidence and determinism tests |
| Level 2 authored policy | not claimed by Gate 12 | not applicable |
| Level 3 shallow deterministic search | not allowed/claimed | not applicable |

## Tests required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | future `RULE-COVERAGE.md` ticket |
| golden trace preservation/update notes | yes if promoted | not applicable now; no migration |
| property/invariant tests | yes | future Flood Watch property tests |
| replay/hash tests | yes | future Flood Watch replay tests |
| serialization tests | yes | future Flood Watch serialization tests |
| visibility/no-leak tests | yes | future native, WASM, and browser evidence |
| bot tests | yes | future Flood Watch bot tests |
| benchmark tests | yes | future Flood Watch benchmarks |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | preserve | No shared helper or migration. | prior hidden-hand/reveal pressure |
| future `games/flood_watch/tests/golden_traces/*.trace.json` | `flood_watch` | create locally | Local implementation must prove deterministic setup, automation, shared terminal outcome, visibility, and replay/export behavior. | `FW-*` rules |

## Back-port and conformance plan

No back-port is required because no helper is promoted.

Affected prior games:

- `masked_claims`: no code or trace change; reaction-window row remains first-use.
- `high_card_duel`: no code or trace change.
- `poker_lite`: no code or trace change.
- `plain_tricks`: no code or trace change.

Exceptions:

- None. This is a no-promotion decision, not a promoted primitive with
  exceptions.

Closure gate if debt is deferred:

- Not applicable. No promotion debt is created, so `docs/MECHANIC-ATLAS.md`
  §10A remains `_None_`.

## Examples

Good fits for future review only:

- Another official cooperative game whose terminal surface is shared or partly
  shared and can be compared against Flood Watch.
- Another official game whose event automation resolves from a command stream
  without a seat actor.
- Another official game whose public role/faction powers modify action
  magnitudes or effect payloads.
- Another official game whose turn grants a fixed budget of multiple validated
  actions.

## Anti-examples

Not a fit:

- A seat response window, interrupt, cancel, or pending challenge.
- A private hand or per-seat hidden holding.
- A graph-movement or adjacency helper.
- A data-driven event script or formula interpreter.
- A browser timer that drives environment resolution.
- A generic cooperative engine or role-power framework.

## ADR need

ADR required? no

Reason:

- No architecture, replay/hash semantics, data policy, kernel boundary, browser
  authority, bot policy class, or public/private content policy changes are
  proposed.

ADR becomes required if a future proposal adds a generic event deck, automation
phase, role-power, shared-outcome, action-budget, visibility/export helper,
data behavior language, or new kernel vocabulary.

## Review checklist

- Third-game hard gate is not triggered by Flood Watch.
- Fifth hidden-hand/reveal reopen is not triggered because there are no
  per-seat private holdings.
- Reaction-window second-use reopen is not triggered because the environment is
  automation, not a pending response.
- No game noun enters `engine-core`.
- No helper is added to `game-stdlib`.
- No untyped behavior language is created.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports
  only.
- Golden traces are preserved because no prior game migration occurs.
- Replay/hash and serialization impacts are recorded.
- Visibility/no-leak impacts are covered for future implementation tickets.
- UI/effect and bot impacts are covered for future implementation tickets.
- ADR need is explicit.
