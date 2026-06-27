# Starbridge Crossing Rules

Game ID: `starbridge_crossing`

Public display name: `Starbridge Crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Rules version: `starbridge-crossing-rules-v1`

Data version: `starbridge-crossing-data-v1`

Prepared by: `Codex`

Created: 2026-06-27

Last updated: 2026-06-27

## Rule Authority

This document is the original Rulepath rules summary for Starbridge Crossing,
a neutral Rulepath implementation in the Star Halma rules family. Sources and
variant comparisons belong in `SOURCES.md`; this document
states the implementation contract that Rust code, traces, rule coverage, bots,
replay, WASM, and UI must satisfy.

Stable `SC-*` rule IDs are requirements. They must remain stable after
implementation unless intentionally migrated with a migration note and matching
updates in `RULE-COVERAGE.md`, traces, tests, player-facing docs, and any
consumer that names the affected rule.

Rust owns setup, legal actions, validation, board occupancy, step and hop-chain
legality, finish ranks, deterministic turn-limit handling, semantic effects,
visibility projection, replay behavior, terminal detection, and bot decisions.
TypeScript may present only Rust/WASM output.

Game nouns in this document belong to `games/starbridge_crossing` only. They
do not authorize `board`, `space`, `peg`, `hole`, `coordinate`, `adjacency`,
`jump`, `path`, `home`, `target`, `graph`, or related vocabulary in
`engine-core`.

## Metadata

| Field | Value |
|---|---|
| game id | `starbridge_crossing` |
| public display name | `Starbridge Crossing` |
| variant | `starbridge_crossing_classic_star_v1` |
| rules version | `starbridge-crossing-rules-v1` |
| data version | `starbridge-crossing-data-v1` |
| source note | `games/starbridge_crossing/docs/SOURCES.md` |
| coverage matrix | `games/starbridge_crossing/docs/RULE-COVERAGE.md` |

## Purpose And Scope

Starbridge Crossing proves an official large-board, perfect-information public
race game: a 121-space six-pointed star, variable seats, public peg occupancy,
single steps, stop-anywhere hop chains, finish-order standings, deterministic
replay, and a browser renderer for a dense non-rectangular surface.

Public presentation uses the original name Starbridge Crossing. Family and
source-history labels belong in `SOURCES.md`, not in product identity. The game
uses original Rulepath prose and presentation, with no copied rulebook text,
diagrams, board art, peg art, app layout, or trade dress.

The only Gate 20 variant is `starbridge_crossing_classic_star_v1`.

## Identity, Seats, And Setup

| Rule ID | Rule | Visibility | Notes |
|---|---|---|---|
| `SC-ID-001` | The game id is `starbridge_crossing`; the variant is `starbridge_crossing_classic_star_v1`; the rules/data versions are `starbridge-crossing-rules-v1` and `starbridge-crossing-data-v1`. | public metadata | Manifest, catalog, WASM, tools, traces, and docs must agree. |
| `SC-ID-002` | Public copy uses **Starbridge Crossing** as the game name. Rules-family labels may appear in source notes and maintenance context only. | public copy | This avoids source confusion and copied trade dress. |
| `SC-SETUP-001` | Supported seat counts are exactly 2, 3, 4, and 6. Default setup uses 2 seats. Every other seat count, including 1 and 5, is rejected by Rust with a stable viewer-safe diagnostic. | public diagnostic | There are no teams, partnerships, roles, or elimination. |
| `SC-SETUP-002` | Stable seat labels are `north`, `north_east`, `south_east`, `south`, `south_west`, and `north_west` in clockwise ring order. | public | Active seats are deterministic subsets of this ring. |
| `SC-SETUP-003` | For 2 seats, active homes are `north` and `south`. For 3 seats, active homes are `north`, `south_east`, and `south_west`. For 4 seats, active homes are two opposite pairs with one opposite pair unused. For 6 seats, every point is active. | public | Each active home targets its opposite point. |
| `SC-SETUP-004` | The board has exactly 121 stable spaces in a six-pointed star topology. Each space has a stable game-local id, coordinate metadata, UI anchor metadata, zone metadata, and neighbor metadata. | public | Static content describes spaces; Rust owns legal behavior. |
| `SC-SETUP-005` | Each active seat starts with exactly 10 public pegs in that seat's home point. | public | The 15-piece two-player variant is out of scope. |
| `SC-SETUP-006` | Initial setup, active-seat order, peg ids, space ids, and any randomized fixture data are deterministic from match seed, seat count, variant, rules version, and data version. | replay authority | Wall-clock time and browser randomness are not inputs. |

## Visibility

| Rule ID | Rule | Visible to whom | Notes |
|---|---|---|---|
| `SC-VIS-001` | A public observer sees the full board topology, all peg occupancy, active seat, legal public action tree, effects, move history, finish ranks, terminal reason, and viewer-safe diagnostics. | public observer and all seat viewers | Starbridge Crossing has no hidden-information class. |
| `SC-VIS-002` | Seat viewers receive the same board facts as the public observer. Seat-local presentation may label the viewer's own pegs, but may not add private rules facts. | all seat viewers | Any "you" affordance is presentation of public ownership only. |
| `SC-VIS-003` | There are no private hands, hidden decks, concealed commitments, secret roles, hidden bot rankings, or hidden score facts. | not applicable | ADR 0004 hidden-info replay/export redaction is not needed for this game. |
| `SC-VIS-004` | Public replay exports may include seed, setup, command stream, public views, public effects, finish ranks, and terminal explanation because every game fact is public. | public export | Export/import still remains deterministic and versioned. |

## Turn Flow

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `SC-TURN-001` | One active seat acts at a time. Play advances clockwise through active seats, skipping seats that already have a finish rank. | active-seat turn | Wrong-seat, stale-token, unsupported-seat, and terminal commands are rejected. |
| `SC-TURN-002` | A turn is exactly one move by one owned peg, or a forced blocked pass when no move exists. | turn unit | A legal move is one step or one hop chain; a step and hop cannot be mixed. |
| `SC-TURN-003` | A finished seat keeps its pegs on the public board and is skipped for future active-seat selection. | finish-order turn flow | Finished pegs remain ordinary occupancy for later movement constraints. |

## Legal Moves

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `SC-MOVE-001` | A step moves one owned peg from its current space to an adjacent empty space. | `move/<peg>/step/<space>` | Adjacency is Rust-owned from game-local topology content. |
| `SC-MOVE-002` | A step is illegal if the destination is occupied, off-board, non-adjacent, or not named by the current Rust legal action tree. | diagnostic | Invalid steps do not mutate state. |
| `SC-MOVE-003` | A hop jumps one owned peg over exactly one adjacent occupied space into the empty space immediately beyond in the same direction. | hop action-tree node | The jumped peg is not captured or moved. |
| `SC-MOVE-004` | A hop is illegal if the midpoint is empty, the landing is missing, the landing is occupied, or the direction is not a legal six-direction line from the current space. | diagnostic | Invalid hops do not mutate state. |
| `SC-MOVE-005` | After a legal hop landing, the same peg may continue hopping from its new space. A hop chain may change direction after each landing. | compound action tree | Rust enumerates continuation nodes deterministically. |
| `SC-MOVE-006` | The player may stop a hop chain after any legal hop landing. | `stop` leaf | Stop-anywhere is part of the pinned variant. |
| `SC-MOVE-007` | A hop chain may not revisit a landing space already present in that same turn's hop path. | finite-chain guard | This is a Rulepath deterministic action-tree resolution. |
| `SC-MOVE-008` | A move may not combine a step with any hop. | diagnostic | The accepted command path is either step-only or hop-chain-only. |
| `SC-MOVE-009` | If the active seat has no legal step and no legal hop, Rust exposes exactly one forced `pass_blocked` action. | `pass_blocked` | The pass records the no-move condition and advances the turn. |

## Finish, Rankings, And Terminal Conditions

Stable terminal tokens for later coverage, outcome, and trace consumers:

- `finish-all-pegs-target-home`
- `finish-order-continues`
- `terminal-all-but-one-finished`
- `terminal-turn-limit`

| Rule ID | Rule | Notes |
|---|---|---|
| `SC-FINISH-001` | A seat receives the next finish rank when all 10 of that seat's pegs occupy that seat's target home at the end of an accepted move. | Finish is checked after state mutation for the accepted move. |
| `SC-FINISH-002` | Finish ranks are assigned in completion order starting at rank 1. | Lower rank is better. |
| `SC-FINISH-003` | The match normally ends when all but one active seat have finish ranks. The last unfinished seat receives the final rank. | This produces full standings for 3+ seats. |
| `SC-FINISH-004` | Terminal standings are seat-keyed, stable in seat-ring order, and include finish rank, finished flag, winner flag for rank 1, and any public progress facts needed for explanation. | TypeScript renders Rust-authored standings only. |
| `SC-FINISH-005` | Official variants include a deterministic `max_plies` option. Default public simulations and benchmarks use 2000 plies unless a fixture intentionally sets a lower value. | This is a replay/simulation safeguard. |
| `SC-FINISH-006` | On turn limit, Rust records a `turn_limit` terminal with completed finish ranks plus deterministic unfinished-seat standings by progress vector and clockwise seat order. | The progress vector is Rust-owned and public. |

## Replay, Bots, And UI

| Rule ID | Rule | Notes |
|---|---|---|
| `SC-REPLAY-001` | The same accepted command stream reproduces state, effects, views, and hashes under fixed seed, seat count, variant, rules version, and data version. | Replay is deterministic and Rust-owned. |
| `SC-REPLAY-002` | Trace Schema v1 records setup, step moves, hop-chain moves, blocked passes, finish assignment, turn-limit terminal state, public visibility notes, and migration notes as applicable. | No trace schema migration is authorized by this rules doc. |
| `SC-BOT-001` | L0 random-legal bots select deterministically from Rust legal action paths and submit through normal validation. | L0 never constructs legality itself. |
| `SC-BOT-002` | Any higher bot may use only public facts, Rust legal actions, deterministic authored preferences, and the evidence allowed by `AI.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`. | Hidden-world search is not relevant, and forbidden algorithms remain forbidden. |
| `SC-BOT-003` | MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, and runtime LLM move selection are forbidden for public v1/v2. | This applies even though the game is perfect-information. |
| `SC-UI-001` | Browser controls present Rust legal actions and Rust-safe previews; TypeScript must not compute adjacency, enumerate jumps, validate paths, assign finish ranks, choose bot moves, or decide terminal state. | Legal-only UI is required. |
| `SC-UI-002` | The public UI must support the 121-space board, peg selection, legal step targets, progressive hop-chain construction, stop leaves, replay/import/export controls, and a no-drag-required action path. | Keyboard and single-pointer alternatives are required. |
| `SC-UI-003` | DOM text, accessibility names, `data-testid` values, storage, console logs, and animation/effect surfaces may contain public board facts only and must not invent legality. | The all-public no-leak audit is explicit, not omitted. |

## Diagnostics

The implementation must expose stable, viewer-safe diagnostic codes for at
least these cases: wrong rules version, stale command, invalid seat count,
unknown seat, wrong active seat, terminal state, malformed action, unknown peg,
peg not owned by active seat, unknown space, occupied destination, non-adjacent
step, invalid hop midpoint, missing hop landing, occupied hop landing, repeated
hop landing, mixed step and hop, no legal move except `pass_blocked`, and
unsupported variant.

Diagnostics may name public spaces, public occupants, public seat labels, public
active-seat facts, and public finish ranks where relevant. They must not imply
that browser code owns legality or that any hidden game fact exists.

## Known Ambiguities And Chosen Resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `SC-SETUP-001` | Some table variants support different piece counts or seat arrangements. | Support exactly 2, 3, 4, and 6 seats, 10 pegs per active seat. | Gate 20 variant pin. | setup 2/3/4/6 traces and invalid 5-seat diagnostic | Discontinuous seat support is intentional. |
| `SC-MOVE-003` | Some variants describe long-distance hops across gaps. | A hop crosses exactly one adjacent occupied space into the empty space immediately beyond. | Gate 20 classic-star scope. | one-hop and invalid long-hop traces | Long-distance variants are excluded. |
| `SC-MOVE-006` | Whether a player must continue a jump chain when another hop is available. | A player may stop after any legal hop landing. | Gate 20 source comparison and variant pin. | jump-stop-midway trace | Stop-anywhere must appear in the action tree. |
| `SC-MOVE-007` | Cyclic hop paths can make naive action trees unbounded. | A single move cannot revisit a landing space already used in that hop chain. | Rulepath deterministic finite action-tree resolution. | repeat-landing rejected trace | This is documented as an implementation resolution. |
| `SC-MOVE-009` | Physical play can stall through blocked positions. | Rust exposes a forced `pass_blocked` when no step or hop exists. | Replay/simulation explicitness. | blocked-pass trace | This does not create a strategic pass option. |
| `SC-FINISH-003` | Many casual rules focus on first finisher only. | Continue until all but one active seat have ranks, then assign the last rank. | Rulepath multi-seat outcome contract. | finish-order and terminal standings traces | Needed for 3+ seat standings. |
| `SC-FINISH-006` | The rules family has no natural ply limit. | Official variants include a deterministic turn-limit terminal fallback. | Simulation and benchmark safeguard. | turn-limit trace | Normal play should end by finish ranks. |

## Explicit Out-Of-Scope Variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `SC-SETUP-001` | One-seat, five-seat, 15-piece two-player, two-set-per-player, and partnership variants. | Gate 20 proves the `{2,3,4,6}` individual-race variant. | Later accepted spec only. |
| `SC-MOVE-003` | Long-distance hop-across-empty variants, capture, swap rules, must-leave-home rules, and anti-permanent-block house rules. | Gate 20 pins a compact classic step/hop/no-capture variant. | Later accepted spec only. |
| `SC-SETUP-004` | Square Halma board or any rectangular-board variant. | Starbridge Crossing is the six-pointed-star gate. | Later accepted spec only. |
| `SC-BOT-002` | L2 authored policy or L3 bounded deterministic search as public-default behavior. | L0 is the Gate 20 floor; higher bots require evidence pack and latency proof. | Future strategy evidence pack or ADR as applicable. |
| `SC-UI-001` | Browser-side adjacency, jump, finish, terminal, or outcome math. | TypeScript is presentation-only. | Not admissible without foundation change. |

## Rule Coverage Link

The implementation and evidence mapping will live in `RULE-COVERAGE.md`.
Every `SC-*` rule in this document must appear there. Silent gaps are not
allowed.

## Rule-ID Migration Notes

None yet. Any migration must update this document, `RULE-COVERAGE.md`, traces,
tests, player-facing docs, and consumers that name the affected rule IDs.
