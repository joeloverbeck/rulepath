# Starbridge Crossing Bot Strategy Evidence Pack

Game ID: `starbridge_crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Rules version: `starbridge-crossing-rules-v1`

Bot target: future Level 1 or Level 2 authored policy

Policy name/version: not admitted

Date: 2026-06-27

## Purpose And Gate

This is the formal status record for future Starbridge Crossing strategy bots.
Gate 20 admits only the Level 0 random legal bot. A Level 1 or Level 2 policy
must not be coded until this pack is completed, reviewed, and accepted in a
later bounded task.

Decision: `L1/L2 not started / intentionally deferred`

Level 0 status: `starbridge-crossing-l0-random-legal-v1` shipped.

Level 1 status: `not_started_pending_strategy_evidence`

Level 2 status: `not_started_pending_competent_policy`

Level 3 status: `not applicable / runtime LLM play forbidden`

## Explicit Public V1/V2 Exclusions

No public Starbridge bot may use TypeScript-computed legality, dev-only state,
unbounded weight soup, static-data tactical scripts, MCTS, ISMCTS, Monte Carlo
rollouts or search, determinization, machine learning, reinforcement learning,
runtime LLM policy, or hidden candidate rankings. Starbridge is perfect
information, but the forbidden algorithm list still applies.

## Source Documents Consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| rules | [RULES.md](RULES.md) | yes | read | Stable `SC-*` rule IDs. |
| sources | [SOURCES.md](SOURCES.md) | yes | read | Source/IP posture. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | informative only | Not an admitted policy. |
| bot registry | [AI.md](AI.md) | yes | read | L0 only. |
| implementation | `games/starbridge_crossing/src/bots.rs` | yes | read | Current bot uses Rust legal action tree. |
| tests | `games/starbridge_crossing/tests/bots.rs` | yes | read | L0 legality and explanation coverage. |

## Evidence Pack Status

Decision: `L1/L2 not started / intentionally deferred`

| Area | Status | Required future work |
|---|---|---|
| accepted competent-player taxonomy | partial | Review and accept [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) as design input. |
| exact policy ID/version | not assigned | Freeze a policy id and version before code. |
| authorized input schema | not implemented | Freeze public view/action/history fields. |
| deterministic priority vector | not implemented | Define lexicographic priorities for home exit, center mobility, jump continuation, and target fill. |
| fixed scenario corpus | not complete | Cover 2/3/4/6 seats, home exit, central jump, target fill, blocked pass, turn-limit pressure. |
| L0 evaluation | incomplete | Compare against L0 with fixed seeds and by-seat summaries. |
| no-leak proof | partial | Perfect information, but candidate rankings and dev state must stay out of public surfaces. |
| legality/replay/hash evidence | incomplete | Decisions must validate through Rust command path and replay deterministically. |
| benchmark evidence | not complete | Add strategy decision and full playout benchmark lanes. |
| implementation admission update | not complete | Update admission/evidence docs after all rows pass. |

## Authorized Future Input View

| Input item | Included? | Source | Visible to every viewer? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action paths | yes | Rust action tree | yes for active public actor context | Candidate base; no constructed illegal moves. |
| public view | yes | Rust projection | yes | Board occupancy, active seat, finish ranks, terminal reason. |
| public command/effect history | possible | viewer-filtered logs | yes | Must remain public and deterministic. |
| policy seed/tie-break state | yes | bot framework | not game info | Deterministic only. |
| TypeScript adjacency/jump math | no | forbidden | no | Rust owns legality. |
| dev diagnostics or private framework fields | no | forbidden | no | Not public bot input. |
| candidate rankings in public output | no | forbidden until admitted and redacted | no | Future strategy proof required. |

## Required Future Scenario Corpus

| Corpus area | Required cases | Rule IDs | Evidence status |
|---|---|---|---|
| home exit | multiple legal exits and self-blocking examples | `SC-MOVE-*` | future |
| jump-chain choice | stop now versus continue, direction changes, repeated landing rejection | `SC-MOVE-006` through `SC-MOVE-009` | future |
| target fill | near-finish target occupancy and final peg pathing | `SC-FINISH-001`, `SC-FINISH-002` | future |
| blocked pass | no legal move and pass-only tree | `SC-MOVE-010` | future |
| multiseat standings | 3, 4, and 6 seat finish pressure | `SC-FINISH-003`, `SC-FINISH-004` | future |
| turn limit | progress-vector ordering | `SC-FINISH-005`, `SC-FINISH-006` | future |

## Current Evidence

| Surface | Status | Evidence | Notes |
|---|---|---|---|
| L0 legality | pass | `games/starbridge_crossing/tests/bots.rs` | Bot selects one Rust legal action path. |
| L0 trace | pass | `bot-l0-action.trace.json` | Replay-visible public command/effect evidence. |
| L0 benchmark | pass | [BENCHMARKS.md](BENCHMARKS.md) `simulate_l0_6p_64_actions` | Throughput only, not skill. |
| L1/L2/L3 strategy | not started | this document | No higher policy admitted. |
