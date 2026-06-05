# Rulepath Implementation Specs

This directory holds **implementation specs**: one bounded plan per roadmap gate.

Specs sit between law and execution:

- [`../docs/ROADMAP.md`](../docs/ROADMAP.md) is the prescriptive ladder (law).
- A **spec** turns one roadmap gate into a concrete, reviewable plan.
- A filled [`../templates/AGENT-TASK.md`](../templates/AGENT-TASK.md) (in `tasks/`)
  is the bounded packet an agent or human actually executes.

Specs are subordinate to the foundation set in
[`../docs/README.md`](../docs/README.md). A spec MUST NOT redefine or override any
foundation contract. Where a spec and a foundation document disagree, the
foundation document wins.

## Spec index (progress tracker)

This table is the **living progress record**. A new brainstorm that wants to
"produce the next spec to continue the roadmap" should read this first and pick
the lowest-numbered gate whose status is not `Done`. ROADMAP.md is not edited to
record progress; this table is.

| Stage | Gate | Spec | Status |
|---:|---|---|---|
| 0 | Gate 0 | [`gate-0-repository-skeleton.md`](../archive/specs/gate-0-repository-skeleton.md) | Done |
| 1 | Gate 1 | [`gate-1-race-to-n.md`](../archive/specs/gate-1-race-to-n.md) | Done |
| 1 | Gate 2 | trace/replay/benchmark hardening — not yet specced | Not started |
| 1 | Gate 3 | WASM/static web shell — not yet specced | Not started |
| 2 | Gate 4 | `three_marks` — not yet specced | Not started |
| 3 | Gate 5 | `column_four` — not yet specced | Not started |
| 4 | Gate 6 | `directional_flip` — not yet specced | Not started |
| 5 | Gate 7 | `draughts_lite` — not yet specced | Not started |
| 6 | Gate 8 | `high_card_duel` / `blackjack_lite` — not yet specced | Not started |
| 7 | Gate 9 | `token_bazaar` / `secret_draft` — not yet specced | Not started |
| 9 | Gate 10 | `poker_lite` / `plain_tricks` — not yet specced | Not started |
| 11 | Gate 11 | `masked_claims` — not yet specced | Not started |
| 12 | Gate 12 | `flood_watch` — not yet specced | Not started |
| 13 | Gate 13 | `frontier_control` — not yet specced | Not started |
| 14 | Gate 14 | `event_frontier` — not yet specced | Not started |
| — | Gate P | private monster-game red-team (late, isolated, non-public) | Not started |

Status values: `Not started` → `Planned` (spec written) → `In progress`
(AGENT-TASKs executing) → `Done` (gate exit criteria pass). Flip a spec to
`Done` only after its exit-criteria section is satisfied with evidence.

## Spec format

Each spec follows this structure (see
[`gate-0-repository-skeleton.md`](../archive/specs/gate-0-repository-skeleton.md)
as the canonical example). Use explicit `not applicable` rows over silent
omissions.

1. **Header** — Spec ID, stage, gate, status, date, owner, authority order.
2. **Objective** — what the gate achieves, sourced from ROADMAP.
3. **Scope** — in scope / out of scope / not allowed (carry the gate's ROADMAP
   "Not allowed" list).
4. **Deliverables** — concrete artifacts/tree, grounded in ARCHITECTURE.md.
5. **Work breakdown** — bounded items, each a candidate AGENT-TASK, with
   dependency order.
6. **Exit criteria** — mapped row-for-row to the gate's ROADMAP exit list.
7. **Acceptance evidence** — tests/traces/benchmarks/reviews; mark game-level
   evidence `not applicable` when the gate has no game.
8. **FOUNDATIONS & boundary alignment** — principles engaged, with stance and
   rationale; keep §12 stop conditions clear.
9. **Forbidden changes** — gate-specific prohibitions.
10. **Documentation updates required** — including this index's status flip.
11. **Sequencing** — predecessor/successor gate; admission rule.
12. **Assumptions** — one-line-correctable.

## Workflow

1. Pick the lowest non-`Done` gate from the index.
2. Write its spec from the format above, grounded in ROADMAP + the foundation set.
3. Decompose the work breakdown into `tasks/` AGENT-TASK packets.
4. Execute, gathering the acceptance evidence.
5. When exit criteria pass, flip the index status to `Done` and admit the next gate.
