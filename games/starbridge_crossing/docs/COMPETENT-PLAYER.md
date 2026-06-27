# Starbridge Crossing Competent Player Analysis

Game ID: `starbridge_crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Rules version checked: `starbridge-crossing-rules-v1`

Date: 2026-06-27

## Purpose And Authority

This document is strategy analysis for future bot work. It is not rule
authority. [RULES.md](RULES.md) wins over this document whenever they differ.
All prose is original Rulepath prose; source posture is recorded in
[SOURCES.md](SOURCES.md).

## Sources And Observations

| Source/reference | Date consulted | Used for | Copied prose status | Notes |
|---|---:|---|---|---|
| [RULES.md](RULES.md) | 2026-06-27 | implemented rule IDs and legal boundaries | none | Rule authority. |
| [SOURCES.md](SOURCES.md) | 2026-06-27 | family facts and source/IP posture | none | Consulted-not-copied source notes. |
| [AI.md](AI.md) | 2026-06-27 | shipped bot limit | none | L0 random legal only. |
| [BENCHMARKS.md](BENCHMARKS.md) | 2026-06-27 | large-board pressure | none | Native benchmark lanes and smoke floors. |
| self-play/code review | 2026-06-27 | strategy implications from Rust action trees, finish ranks, and traces | none | No external strategy prose copied. |

## Rules Cross-Check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| setup and target assignment | `SC-SETUP-001` through `SC-SETUP-004` | no | Seats have opposite home/target points. |
| step and jump legality | `SC-MOVE-001` through `SC-MOVE-009` | no | Legal paths are Rust-owned and occupancy-dependent. |
| blocked pass | `SC-MOVE-010` | no | Pass exists only when Rust proves no legal move. |
| finish ranks and terminal state | `SC-FINISH-001` through `SC-FINISH-006` | no | Full standings, not first-finisher-only. |
| visibility and bot inputs | `SC-VIS-001`, `SC-BOT-001`, `SC-BOT-002` | no | All game facts are public; forbidden algorithms remain forbidden. |

## Competent-Player Summary

A competent Starbridge Crossing player advances pegs out of home without
blocking their own lanes, uses short steps to set up longer jump chains, keeps
central lanes flexible, and avoids stranding a last peg far from the target.
They watch every public peg because opponent occupancy creates both obstacles
and jump opportunities. In multi-seat games they also balance finish-order
pressure: first rank matters, but the match continues until the full terminal
standing is determined.

The shipped Level 0 bot is not a competent-player proxy. It is a safe,
deterministic random-legal baseline over Rust action trees.

## Situation Model

| Situation | Competent principle | Rule IDs | Bot feature candidate? |
|---|---|---|---:|
| opening home exit | spread pegs to avoid self-blocking and expose future jumps | `SC-MOVE-*` | future |
| central crossing | prefer moves that create multiple legal next-turn paths | `SC-MOVE-001` through `SC-MOVE-009` | future |
| jump-chain choice | continue only when the landing improves target progress or future mobility | `SC-MOVE-006` through `SC-MOVE-009` | future |
| near target | prioritize filling target spaces without blocking remaining own pegs | `SC-FINISH-001`, `SC-FINISH-002` | future |
| opponent congestion | use public opponent pegs as jump anchors without assuming cooperation | `SC-VIS-001` | future |
| turn-limit pressure | improve deterministic progress vector and avoid regressions | `SC-FINISH-005`, `SC-FINISH-006` | future |

## Forbidden Shortcuts

No public Starbridge bot may use TypeScript-computed legality, omniscient dev
state, static-data tactical scripts, MCTS, ISMCTS, Monte Carlo search,
determinization, machine learning, reinforcement learning, runtime LLM policy,
or hidden candidate rankings. Hidden-world search is not applicable because the
game is fully public, but the algorithm ban still applies to public v1/v2.

## Current Bot Status

| Level | Status | Evidence | Notes |
|---|---|---|---|
| L0 | shipped | [AI.md](AI.md), `games/starbridge_crossing/tests/bots.rs`, `bot-l0-action.trace.json` | deterministic random legal path selection. |
| L1 | not started | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | no heuristic policy admitted. |
| L2 | not started | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | no competent policy admitted. |
| L3 | not applicable | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | runtime LLM play is forbidden. |
