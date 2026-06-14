# River Ledger Game Implementation Admission

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Created: 2026-06-14

Last updated: 2026-06-14

## Admission status

Status: pre-coding admission spine in progress.

Implementation may begin only after the pre-coding docs are present, reviewed,
and archived through the first two Gate 15 tickets. This document records the
current admission receipt and blockers before any Rust crate scaffold.

## Pre-coding prerequisites

| Prerequisite | Evidence | Status |
|---|---|---|
| Gate 15 spec exists and is the lowest active unit. | `specs/gate-15-river-ledger-texas-holdem-base.md`; `specs/README.md` lists Gate 15 as the lowest non-`Done` public-scaling unit. | satisfied |
| Source notes and original rules summary exist. | `games/river_ledger/docs/RULES.md`; `games/river_ledger/docs/SOURCES.md`; global `docs/SOURCES.md` River Ledger note. | satisfied by GAT15RIVLEDTEX-001 |
| Stable `RL-*` rule-ID families are defined. | `RULES.md` defines `RL-SETUP`, `RL-DEAL`, `RL-BET`, `RL-STREET`, `RL-EVAL`, `RL-SHOW`, `RL-POT`, `RL-VIS`, `RL-BOT`, `RL-UI`, and `RL-REPLAY` families. | satisfied |
| Mechanic inventory exists. | `games/river_ledger/docs/MECHANICS.md`. | satisfied by this ticket |
| Planned coverage matrix exists. | `games/river_ledger/docs/RULE-COVERAGE.md`. | satisfied by this ticket, intentionally open until code lands |
| Primitive-pressure decision exists. | `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`. | satisfied by this ticket |
| Open promotion debt checked. | `docs/MECHANIC-ATLAS.md` open debt register is `_None_`; this ticket opens no debt. | satisfied |
| IP posture reviewed. | `SOURCES.md` documents original prose, source-use limits, asset/font status, and human/legal triggers. | satisfied for docs-only admission |

## Explicit blockers before public admission

These are not blockers to starting the crate scaffold after ticket 002; they are
blockers to official/public completion.

| Blocker | Owning later ticket surface |
|---|---|
| Rust crate scaffold, typed static data, and strict behavior-key rejection are not implemented. | crate scaffold/static-data ticket |
| Setup, shuffle/deal, blinds, action order, and initial visibility are not implemented. | setup ticket |
| Fixed-limit betting, contribution ledger, street advancement, and cap diagnostics are not implemented. | betting ticket |
| Evaluator, showdown, split allocation, and outcome rationale are not implemented. | evaluator/showdown tickets |
| Pairwise 3-6 seat no-leak proof does not exist yet. | visibility/no-leak tickets |
| Replay export/import, hashes, fixtures, and golden traces are not implemented. | replay/test/tool tickets |
| Bots and Level 2 evidence pack are not implemented. | bot docs and bot implementation tickets |
| Native simulation, benchmarks, and tool registration are not wired. | simulation/benchmark/tool tickets |
| WASM, web renderer, public rules surface, e2e smoke, and catalog docs are not wired. | WASM/web tickets |
| Public-release checklist and final atlas review are not complete. | trailing docs, atlas, capstone tickets |

## Foundation alignment

| Foundation rule | Admission stance |
|---|---|
| Rust owns behavior. | All setup, legality, validation, effects, views, replay, evaluator, outcome, and bots are planned for Rust. |
| TypeScript presents only. | Web tickets may render Rust/WASM payloads but must not decide legality, hand strength, winners, splits, or hidden facts. |
| `engine-core` stays noun-free. | River Ledger game nouns remain in `games/river_ledger`. |
| `game-stdlib` is earned. | No helper promotion is authorized; atlas debt remains `_None_`. |
| Static data is not behavior. | Variant/static data may carry metadata and labels only; strict tests will reject behavior-looking keys. |
| Hidden information does not leak. | Pairwise N-seat no-leak proof is a first-class gate requirement. |
| Public bots are not research AI. | L0/L1/L2 only; no MCTS, ISMCTS, Monte Carlo, ML, RL, solvers, or hidden-state sampling. |
| IP is conservative. | River Ledger uses original prose/name/presentation; sources verify facts only. |

## Admission decision

Pre-coding docs now establish a reviewable contract for the crate scaffold. The
crate may be started after this ticket is archived and committed, provided later
tickets continue to treat the open coverage rows and blockers as required work
rather than as completed evidence.
