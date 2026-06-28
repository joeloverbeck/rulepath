# Multi-Seat and Larger-Surface Contract

Status: area law for N-seat and larger-surface public gates. Subordinate to
`FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, accepted
visibility ADRs, and `AI-BOTS.md`.

This document codifies existing Rulepath principles for games with more than two
seats, team/role structure, larger boards, larger card zones, larger public
objects, or wider action fanout. It does not change trace-schema semantics,
WASM exported-API schemas, replay/hash contracts, bot law, kernel vocabulary, or
the `game-stdlib` promotion process.

## 1. Authority

The constitution still wins:

- Rust owns setup, legal actions, validation, effects, views, replay, hashes,
  deterministic randomness, and bot decisions.
- TypeScript displays Rust/WASM output only.
- `engine-core` remains a generic contract kernel.
- Static data is typed content, parameters, metadata, fixtures, traces, and
  presentation prose, not behavior.
- Hidden information must not leak through payloads, DOM, storage, logs,
  previews, effects, bot explanations, candidate rankings, replay exports,
  traces, tests, or diagnostics.

N-seat and larger-surface work is not a license to generalize the engine. Seat
models, teams, partnerships, tables, pots, card evaluators, graph topologies,
walls, decks, territories, routes, and large maps start in `games/*`. Reuse moves
to `game-stdlib` only through `MECHANIC-ATLAS.md`.

Large map is not a DSL license. Topology, deck lists, setup constants, and
surface metadata may be typed content; conditions, triggers, selectors, formulas,
procedures, legality, scoring, visibility, and bot strategy remain typed Rust.

## 2. Seat-Range Declaration

Every N-seat official-game spec must declare:

| Field | Requirement |
|---|---|
| Minimum seats | Lowest supported official seat count. |
| Maximum seats | Highest supported official seat count. |
| Default seats | Public setup default. |
| Supported sets | Exact supported counts when the range is discontinuous. |
| Seat labels | Stable, original, IP-safe labels for UI and traces. |
| Setup diagnostics | Rust-owned wrong-seat-count diagnostics. |

`Game::setup(seed, seats, setup)` already receives a seat slice. A game may
reject unsupported counts in Rust. The browser may present setup controls and
validation messages supplied by Rust/WASM, but it must not decide which counts
are legal.

Seat IDs must be stable for a replay. Seat array order is authoritative for
deterministic setup, turn order seeds, trace readability, and reproducible
summaries.

## 3. Roles, Teams, and Partnerships

Roles, teams, partnerships, factions, coalitions, dealer buttons, blinds, and
seat-relative obligations are game-local concepts unless later promoted through
the atlas process.

A game that uses roles or teams must state:

- whether roles are public, private, rotating, fixed, or derived during setup;
- whether teams are fixed, inferred, temporary, asymmetric, or absent;
- how role/team data appears in public view, seat-private view, traces, and
  replay exports;
- how outcomes aggregate per seat and, when applicable, per team.

No role/team fact may be present in a viewer payload unless that viewer is
authorized to know it.

## 4. Turn-Order Model

Rust owns turn order and response timing.

An N-seat game must expose viewer-safe state for:

- current active seat when exactly one actor may act;
- active seat set when multiple actors may act;
- pending responder set when reaction windows or simultaneous commitments exist;
- pass/wait/skip obligations when a seat has no legal action but remains part of
  the phase;
- round, dealer, lead, initiative, priority, or phase facts that are public or
  seat-authorized.

TypeScript may visualize the active/pending data it receives. It must not infer
who can act from seat index, DOM state, local setup mode, or rendered labels.

## 5. Viewer Matrix

Every game spec with 3+ seats or hidden information must include a viewer matrix.
At minimum:

| Viewer | Required projection |
|---|---|
| Public observer | Facts safe for everyone, including replay viewers. |
| Seat viewer | Public facts plus only that seat's authorized private facts. |
| Team viewer, if applicable | Public facts plus facts lawfully shared with that team. |
| Dev/internal tools | Explicitly separated from public browser export. |

The matrix must cover view payloads, action trees, previews, diagnostics,
semantic effects, bot explanations, outcome surfaces, replay exports, DOM/test
IDs, and local storage.

Perfect-information games may produce equivalent projections for every seat, but
that equivalence must be intentional and testable. Hidden-information games must
derive each projection in Rust before it reaches the browser.

## 6. Pairwise No-Leak Matrix

For hidden-information N-seat games, pairwise no-leak proof is mandatory.

For every source seat A and every distinct viewer seat B, B must not receive A's
private payload unless Rust has made that fact public or team-authorized. This
applies across:

- view payloads;
- legal action trees;
- previews and diagnostics;
- semantic effects and effect logs;
- bot explanations and candidate/ranking outputs;
- replay exports and imports;
- DOM, CSS class names, `data-testid` values, logs, screenshots, and storage;
- trace fixtures and tests that claim to be public or viewer-scoped.

ADR 0004 remains the replay/export authority. Internal full traces may carry
omniscient evidence for native replay checks; browser/default exports for
hidden-information games are viewer-scoped observation timelines.

### 6A. Asymmetric faction and 5-viewer no-leak floor

Large asymmetric faction games often have more viewer classes than "public" and
"owning seat." A private large-game spec must name every viewer class before
implementation starts. The default no-leak floor for a four-role/faction
private game is the 5-viewer matrix:

- public observer;
- viewer for role/faction/seat 1;
- viewer for role/faction/seat 2;
- viewer for role/faction/seat 3;
- viewer for role/faction/seat 4.

If a game has teams, shared intelligence, eliminated seats, administrators, or
scenario-specific viewers, the matrix grows; it does not replace the 5-viewer
floor. Every asymmetric faction or role must prove pairwise redaction across
view payloads, action trees, previews, diagnostics, effects, bot explanations,
candidate rankings, replay exports, DOM/test IDs, logs, storage, screenshots,
and any private fixture/export that claims to be viewer-scoped.

Public observer safety is mandatory even when the private build is not publicly
released, because private screenshots, replay exports, and playtest artifacts
can otherwise leak into public surfaces. A private title, id, source token,
card/event name, fixture name, or e2e name must never be the token used in a
public proof artifact.

## 7. Public-Observer Rules

A public observer is a viewer with no seat. Public observers must be safe for
spectating, local replay export, documentation screenshots, and public smoke
tests.

Public-observer payloads must omit:

- private hands, hidden commitments, secret roles, concealed melds, and hidden
  setup assignments;
- unrevealed deck/wall/order tails and future random outcomes;
- raw private action paths that encode hidden choices;
- bot private candidates, hidden-state-derived explanations, or belief state;
- terminal hidden information that the rules did not reveal.

If a game has no public-observer mode, its spec must say so and justify the
public product consequence. That exception does not authorize leaking hidden
state to seat viewers.

## 8. Larger-Surface Budgets

Large surfaces must be declared before implementation. A game spec must record:

- maximum official seat count;
- maximum public objects rendered at once;
- maximum private objects per seat;
- maximum board/topology sites or graph edges;
- maximum deck/wall/pile/list sizes;
- maximum visible counters, tracks, or public zones;
- maximum legal action choices at a decision point;
- maximum progressive action-tree depth;
- maximum semantic-effect batch size for normal and terminal actions;
- largest fixture used for native and browser performance proof.

Budgets are not behavior. They make tests, benchmarks, UI layout, and review
tractable. If real implementation exceeds a budget, update the spec/ticket and
re-run the relevant proof rather than hiding the growth.

## 9. Action-Fanout and Progressive Choice

N-seat games often produce wide choices. Rust still owns legality.

Specs must identify which choice surfaces are expected to be flat, grouped,
progressive, filtered by phase, or previewed. Large fanout should be handled by
Rust-owned action tree shape and viewer-safe metadata, not by TypeScript
inventing filters that change what is legal.

If a game needs search, generated candidate grouping, or expensive preview
calculation, the spec must define latency budgets and benchmark surfaces before
the implementation ticket starts.

## 10. Semantic-Effect Batching

Larger games may emit many visible consequences from one action. Effects remain
semantic facts, not animation instructions.

Specs must state:

- which effects are visible to the public observer;
- which effects are seat-private or team-private;
- whether large batches are grouped into Rust-owned summary effects;
- how the renderer settles to the latest viewer-safe view after animation;
- how no-leak tests inspect effect payloads and logs.

Batching must preserve deterministic order and replay/hash stability.

## 11. Outcomes and Final Breakdowns

Every official game must provide a final breakdown for every seat, and when
applicable every team. The outcome surface must be Rust-owned and viewer-safe.

At minimum, terminal views must be able to explain:

- winner set, loser set, draw/tie/split outcome, or ranked standings;
- per-seat score, rank, elimination state, or terminal reason;
- per-team score/rank when teams exist;
- decisive public facts that explain the result;
- redaction for folded, concealed, unrevealed, or private data.

If the game has showdown, evaluation, allocation, or comparison logic, Rust must
produce each contender's evaluated result, comparison vector or equivalent
ranking facts, tie/split reason, and the decisive comparison reason. TypeScript
may render this explanation; it must not calculate it.

## 12. Trace and View-Hash Expectations

No trace-schema migration is authorized by this document.

N-seat traces must use existing replay concepts consistently:

- `seats` array order is stable and authoritative;
- every actor seat is present in the replay seats array;
- public-view hashes should exist for public-observer projections when the game
  supports public observer replay;
- hidden-information games should include view-hash evidence for every
  authorized seat viewer unless the game spec documents a sampled matrix and
  why it is enough;
- terminal trace summaries should use per-seat or per-team standing arrays, not
  fixed `seat_0`/`seat_1` scalar assumptions.

Any change to trace fields, trace version, hash semantics, replay compatibility,
or migration policy requires its own accepted ADR.

## 13. Simulator Summaries

Simulator output for N-seat games must not assume exactly two players.

Simulator summaries represent:

- games completed and failed;
- ordered seat IDs;
- win counts keyed by seat ID;
- loss/draw/tie/split counts keyed by seat ID or winner set;
- team results keyed by stable team ID when teams exist;
- terminal reason counts;
- sample seeds and command evidence for failures;
- deterministic key ordering for machine-readable output.

Existing two-seat simulator runs use the same seat-keyed shape (`seat_order`,
`wins_by_seat`, and related `*_by_seat` maps) as future N-seat games. New
simulator summaries must extend that shape rather than reintroducing fixed
`seat_0`/`seat_1` scalar counters.

## 14. Spec and Ticket Minimums

Every Gate 15+ spec must fill in the relevant N-seat/surface fields before code
work begins:

- supported seat range and setup diagnostics;
- turn-order and pending-responder model;
- viewer matrix;
- pairwise no-leak matrix for hidden-information games;
- public-observer export stance;
- surface and action-fanout budgets;
- outcome and final-breakdown model;
- trace/view-hash evidence plan;
- simulator summary needs;
- atlas pressure from seat count, topology, zones, accounting, reactions, teams,
  or evaluator logic.

Tickets must then name the exact verification surface for each invariant. Broad
"make it multiplayer" work is not bounded enough for Rulepath.
