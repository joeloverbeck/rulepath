# frontier_control Implementation Admission

Game ID: `frontier_control`

Public display name: `Frontier Control`

Implemented variant: `frontier_control_standard`, `frontier_control_highlands`

Roadmap stage/gate: Gate 13 — asymmetric area-control proof

Public role: original portfolio game

Prepared by: Codex

Date: 2026-06-11

## Purpose

This document records the admission and final implementation evidence for
Frontier Control. It does not waive the official-game contract; the gate closes
only because the docs, Rust rules, traces, tools, browser surface, and archive
evidence passed.

## Prerequisite Documents

| Prerequisite | Path | Complete? | Notes |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | Original Rulepath game; no copied rulebook prose, assets, or trade dress. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | Stable `FC-*` rule IDs cover setup, graph legality, control, scoring, terminal, UI, bots, and boundaries. |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | `cargo run -p rule-coverage -- --game frontier_control` passed. |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) | yes | Graph/control/asymmetry first-use rows stay game-local. |
| primitive-pressure ledger | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | yes | Board-space audit not applicable; budget and role/faction second-use comparisons recorded; no open promotion debt. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | Current Level 1 balance limitation is explicitly recorded. |
| ADR | ../../../docs/adr/ | not applicable | No foundation-changing decision was required. |

## Admission Decision

Decision: admitted with explicit constraints

Rationale:
- Frontier Control keeps graph, site, edge, faction, unit, stake, fort, clash,
  supply, control, and scoring nouns local to `games/frontier_control`.
- Rust owns adjacency legality, clash resolution, supply connectivity, scoring,
  terminal detection, public view projection, semantic effects, and bot choices.
- Static data is typed content only: map labels, edges, starts, values, caps,
  budgets, and round limits. Behavior-looking fields are rejected.
- The game is perfect-information and deterministic; hidden-information and
  game-rule randomness surfaces are explicitly not applicable.

Explicit constraints:
- Level 1 vs Level 1 simulation currently reports a 1000-0 Garrison result on
  the standard map. This is documented in [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)
  and [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) as balance
  retune debt before any stronger public balance claim.
- No generic graph/control/faction helper is admitted by this gate.

## Boundary Checks

| Check | Result | Evidence |
|---|---|---|
| `engine-core` remains noun-free | pass | `bash scripts/boundary-check.sh` passed. |
| no `game-stdlib` promotion | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) and `docs/MECHANIC-ATLAS.md` keep shapes local. |
| TypeScript presentation-only | pass | Frontier browser smoke and code review: UI renders Rust view/effects/action trees only. |
| no behavior-in-data | pass | `tests/serialization.rs`, `fixture-check`, and typed TOML/fixture validation. |
| no hidden-info leak | pass | Perfect-information visibility tests plus browser no-leak smoke and replay export checks. |

## Verification Evidence

Capstone verification on 2026-06-11:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo run -p simulate -- --game frontier_control --games 1000`
- `cargo run -p replay-check -- --game frontier_control --all`
- `cargo run -p fixture-check -- --game frontier_control`
- `cargo run -p rule-coverage -- --game frontier_control`
- `cargo bench -p frontier_control`
- `bash scripts/boundary-check.sh`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-doc-links.mjs`

## Sign-off

Prepared by: Codex

Reviewed by: Rulepath maintainers

Date: 2026-06-11
