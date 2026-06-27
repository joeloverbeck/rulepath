# Primitive Pressure Ledger: Starbridge Crossing topology and path-jump legality

Candidate name: `starbridge-crossing-topology-path-jump`

Status: pre-implementation hard gate recorded; defer/reject promotion

Decision date: 2026-06-27

Last updated: 2026-06-27

Prepared by: `Codex`

## Hard Gate

This ledger records Gate 20's primitive-pressure decision before any
`starbridge_crossing` crate, topology content, setup validation, movement
legality, jump-chain enumeration, replay fixture, WASM bridge, bot policy, or
browser renderer is written.

Decision: keep Starbridge Crossing topology, path, jump-chain legality,
finish/progress policy, diagnostics, effects, bot policy, and UI projection
local to `games/starbridge_crossing`. No helper is added to `engine-core` or
`game-stdlib`. No prior-game conformance work is required. No promotion debt is
created, so `docs/MECHANIC-ATLAS.md` §10A remains `_None_`.

The repository-level record is
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) §10B.

This ledger records three reviews:

- `game-stdlib::board_space` is not applicable.
- Frontier Control and Event Frontier are related graph/topology comparison
  points, but they do not prove a narrow shared helper with Starbridge Crossing.
- Gate 21 Pachisi-family race must reopen topology comparison before any track,
  graph, path, capture, or safety helper proposal.

## Mechanic Shape

`board_space` audit:

```text
The promoted game-stdlib::board_space helper covers rectangular dimensions,
row/column coordinates, bounds checks, deterministic row-major iteration,
signed offsets, stable rNcM parse/format, and generic parity. Starbridge
Crossing uses a masked six-pointed star with 121 stable spaces, six-direction
hex-like coordinate metadata, point-specific home/target zones, and hop-vector
legality over occupancy. The promoted rectangular helper is therefore not
applicable.
```

Prior graph/topology comparison:

```text
Frontier Control uses named sites and edges. Adjacency constrains movement,
and connectivity determines supplied-stake scoring.

Event Frontier uses named sites and trails. Adjacency constrains operations and
public presence majority, but it has no path-scored connectivity.

Starbridge Crossing uses stable spaces in a regular six-pointed star, opposite
home triangles, occupied midpoint checks, empty landing checks, progressive
hop-chain action-tree construction, stop-anywhere leaves, a repeated-landing
cycle guard, and finish-order rankings.

These are related topology pressures, but the shared surface is too small and
too behavior-bearing for a helper. A shared abstraction would either be an
anemic coordinate/adjacency carrier that every game still wraps with policy, or
it would encode movement, path, capture, scoring, finish, or UI rules that must
remain game-local Rust.
```

Hard-gate decision:

```text
Decision: defer/reject promotion. Do not extract graph, topology, adjacency,
path, jump-chain, reachability, capture, safety, home, target, finish, progress,
or movement legality helpers in Gate 20.
```

## Games Exerting Pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `three_marks`, `column_four`, `directional_flip`, `draughts_lite` | Gates 4-7 | `game-stdlib::board_space` conformance | promoted rectangular board-space identity | implemented | Rectangular coordinate identity is promoted and closed. Starbridge is not rectangular. |
| `frontier_control` | Gate 13 | `games/frontier_control/src/{actions,rules,effects,visibility,bots}.rs` plus ledger | first site/edge graph pressure | implemented | Named site graph, movement adjacency, connectivity scoring, site control, faction asymmetry. |
| `event_frontier` | Gate 14 | `games/event_frontier/src/{actions,rules,effects,state}.rs` plus ledger | second graph/site comparison | implemented | Named site/trail graph, adjacency legality, public presence majority, no connectivity scoring. |
| `starbridge_crossing` | Gate 20 | planned `games/starbridge_crossing/src/{topology,actions,rules,effects,visibility,bots}.rs` | third related topology/path pressure; first star hop-chain pressure | admitted by spec, not implemented | Masked six-point star, home/target triangles, occupied-midpoint hop legality, stop-anywhere chains, finish rankings. |
| Gate 21 Pachisi-family race | Gate 21 | future track-race spec | next topology comparison candidate | not started | Must compare track topology, chance, capture, and safety against the three games above. |

## Local Implementations Compared

| Aspect | Frontier Control | Event Frontier | Starbridge Crossing | Same helper shape? | Notes |
|---|---|---|---|---:|---|
| topology identity | named sites and edges | named sites and trails | 121 stable spaces with hex-like coordinate metadata | partial | All are non-rectangular, but identity schemes differ. |
| movement/path use | adjacency movement and connectivity scoring | adjacency operation eligibility and majority presence | step adjacency, hop midpoint/landing, chain path construction | no | Starbridge's path legality is occupancy-dependent and compound. |
| scoring/outcome coupling | supplied-stake connectivity and faction scoring | site scoring and asymmetric instant victory | finish ranks and progress-vector turn-limit standings | no | Outcome policy differs completely. |
| action tree | game-local actions over site ids | event/op/pass menus with compound operation commands | progressive peg/step/hop/stop action tree | no | A helper would need action-shape policy. |
| static content role | site/edge content | site/trail/event content | space/coord/zone/neighbor content | partial | Content describes topology only; behavior stays Rust. |
| visibility | public perfect-information | public graph plus hidden deck order | all-public perfect-information | partial | Visibility contracts differ around hidden order. |
| bot policy | faction-specific public policy | faction/event public policy | public race/jump policy | no | Bot heuristics stay local. |

## Extraction Decision

Decision: defer/reject promotion for topology/path-jump legality.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | related topology pressure is real; close helper shape is not proven |
| helper can stay narrow and typed? | no for path/jump legality; only trivial adjacency carriers are plausible |
| helper belongs in `game-stdlib`? | no |
| would contaminate `engine-core`? | yes if board, space, peg, graph, path, jump, home, target, track, node, edge, capture, safety, or movement nouns moved there |
| static-data behavior risk? | medium if topology content started carrying movement, finish, capture, safety, path, or bot policy; current plan forbids that |
| replay/hash impact acceptable? | yes for local implementation; no shared helper or migration is authorized |
| visibility/no-leak impact acceptable? | yes; Starbridge is perfect-information and still Rust-owned |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary change is made |

Rationale:

- `game-stdlib::board_space` is rectangular and behavior-free; it is a bad fit
  for a masked star board with home/target triangles and hop-vector legality.
- Frontier Control and Event Frontier are useful comparison points, but their
  named graph/site policy is materially different from Starbridge's geometric
  occupancy-dependent hop chains.
- A promoted helper would either carry too little value or begin deciding
  movement/path behavior. The latter is forbidden by the mechanic atlas and the
  `engine-core` boundary.
- Keeping the implementation local preserves trace clarity, diagnostics, bot
  evidence, UI previews, benchmarks, and future comparison value for Gate 21.
- No existing game needs migration, no helper exists to reuse, and no
  promotion-debt closure gate is needed.

## Rejected Alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse `game-stdlib::board_space` | It models rectangular row/column spaces, not a masked six-point star. | Audit records not applicable, not an exception. |
| Promote a generic graph/topology helper | The repeated surface is not one narrow behavior-free helper shape. | Reopen at Gate 21 with track topology evidence. |
| Promote jump-chain/path legality | It would encode movement policy, occupancy policy, action-tree policy, and diagnostics. | Must stay game-local Rust. |
| Promote home/target/progress helpers | Finish and progress vector are outcome policy for this game. | Gate 21 capture/safety may differ sharply. |
| Move nouns into `engine-core` | Foundation law forbids mechanic nouns in the kernel. | `engine-core` remains unchanged. |
| Encode jump or finish behavior in static topology data | Static data may describe spaces and neighbors only. | Behavior stays in Rust. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## API Sketch In Prose Only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/starbridge_crossing` |
| determinism requirements | topology ordering, action generation, effects, finish ranks, views, and replay remain deterministic locally |
| replay/hash requirements | no migration of existing games or trace hashes |
| visibility requirements | all projected game facts are public; bot/dev output remains Rust-owned and viewer-safe |
| effect/log requirements | setup, step, hop-chain, blocked pass, finish, terminal, and turn-limit effects remain game-local |
| bot-facing notes | bots consume legal tree and public view only; race/jump heuristics remain game-local |
| non-goals | generic graph, track, pathfinding, jump, home/target, capture/safety, progress, scoring, bot, or TypeScript legality helper |
| good-fit examples | none until a later ledger proves a repeated behavior-free shape with worthwhile conformance payoff |
| anti-examples | compute adjacency in TypeScript, put topology nouns in `engine-core`, define jump rules in data, add helper flags for capture/safety/finish policy |

## Determinism And Replay Impact

| Impact | Required action | Tests/traces |
|---|---|---|
| topology ordering | Preserve stable `StarSpaceId` ordering and deterministic neighbor direction order. | future topology tests and golden traces |
| action ordering | Preserve deterministic peg order, step/hop ordering, direction ordering, depth-first continuation ordering, and stop leaves. | future action/rules tests and golden traces |
| diagnostics | Keep wrong-seat, stale, invalid-step, invalid-hop, repeated-landing, blocked-pass, and terminal diagnostics viewer-safe. | future rules/visibility tests |
| semantic effects | Emit Rust effects for setup, step, hop-chain substeps, blocked pass, finish, and terminal. | future effect/order traces and browser smoke |
| trace hashes | preserve existing games; create Starbridge traces locally | future replay-check for `starbridge_crossing` |
| serialization | stable local state/effect/view ordering | future serialization tests |
| seed/randomness | no game-rule randomness; bot RNG remains bot infrastructure | future setup/replay tests |

## Data And Rust Boundary

Topology content may define stable space ids, coordinates, zone labels, UI
anchors, and neighbor ids. It must not define rule branches, conditional
movement, jump scripts, path formulas, finish policy, blocked-pass policy,
progress scoring, bot heuristics, or renderer authority.

Rust validates every accepted step, hop, hop continuation, stop, blocked pass,
finish rank, terminal state, and diagnostic against the current state.

## Back-Port And Conformance Plan

No helper is promoted, so no prior-game back-port or conformance work is
required.

| Prior game | Conformance result |
|---|---|
| `frontier_control` | no change; named graph/site policy remains local |
| `event_frontier` | no change; named site/trail/event policy remains local |
| rectangular `board_space` adopters | no change; Starbridge is audited not applicable |

## Gate 21 Reopen Trigger

Before Gate 21 Pachisi-family race starts serious implementation, its spec or
first topology ticket must compare track topology, deterministic chance,
capture, safety spaces, multi-pawn movement, and route identity against:

- `game-stdlib::board_space`;
- Frontier Control's site/edge graph;
- Event Frontier's site/trail graph;
- Starbridge Crossing's star topology and path-jump legality.

Gate 21 must then decide reuse, promotion, defer/reject, or ADR before any
track/topology helper is introduced.

## Agent Misuse Risks

- Treating `topology.rs` as a reusable graph library.
- Moving space, peg, graph, path, or jump nouns into `engine-core`.
- Expanding `game-stdlib::board_space` into a generic graph helper.
- Putting jump legality or finish policy into static data.
- Letting TypeScript compute adjacency, legal landings, finish ranks, or
  terminal progress.
- Claiming perfect-information status means no visibility tests are needed.

## Review Owner And Date

Rulepath maintainers; recorded by Codex on 2026-06-27.
