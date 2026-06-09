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
the lowest-numbered gate or maintenance interlock whose status is not `Done`.
Open primitive-promotion debt is treated as an interlock before the next new
mechanic-ladder gate. ROADMAP.md is not edited to record progress; this table is.

| Stage | Gate | Spec | Status |
|---:|---|---|---|
| 0 | Gate 0 | [`gate-0-repository-skeleton.md`](../archive/specs/gate-0-repository-skeleton.md) | Done |
| 1 | Gate 1 | [`gate-1-race-to-n.md`](../archive/specs/gate-1-race-to-n.md) | Done |
| 1 | Gate 2 | [`gate-2-trace-replay-benchmark-hardening.md`](../archive/specs/gate-2-trace-replay-benchmark-hardening.md) | Done |
| 1 | Gate 3 | [`gate-3-wasm-static-web-shell.md`](../archive/specs/gate-3-wasm-static-web-shell.md) | Done |
| 2 | Gate 4 | [`gate-4-three-marks-board-smoke.md`](../archive/specs/gate-4-three-marks-board-smoke.md) | Done |
| 3 | Gate 5 | [`gate-5-column-four-public-polish.md`](../archive/specs/gate-5-column-four-public-polish.md) | Done |
| 4 | Gate 6 | [`gate-6-directional-flip.md`](../archive/specs/gate-6-directional-flip.md) | Done |
| 5 | Gate 7 | [`gate-7-draughts-lite-compound-action-tree.md`](../archive/specs/gate-7-draughts-lite-compound-action-tree.md) | Done |
| 5M | Gate 7.1 | [`gate-7-1-board-space-primitive-back-port.md`](../archive/specs/gate-7-1-board-space-primitive-back-port.md) | Done |
| 5M | Gate 7.2 | [`gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`](../archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md) | Done |
| 6 | Gate 8 | [`gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`](../archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md) | Done |
| 6C | Post-Gate-8 Blackjack placement audit | [`../docs/adr/0006-blackjack-lite-roadmap-placement.md`](../docs/adr/0006-blackjack-lite-roadmap-placement.md) closes the checkpoint: `blackjack_lite` is not a Gate 8.1 implementation target and is deferred as a Gate 10-or-later comparison case. Gate 8's `high_card_duel` evidence satisfies deterministic shuffle, private-view, effect-filter, public replay/export, bot-view, browser no-leak, and benchmark proof. No Blackjack interlock blocks Gate 9. | Done |
| 6M | Gate 8 aftermath / roadmap realignment | [`gate-8-aftermath-roadmap-realignment.md`](../archive/specs/gate-8-aftermath-roadmap-realignment.md) reconciles root, progress, web, source-note, and CI smoke routing after Gate 8 so Gate 9 starts from truthful docs. | Done |
| 7 | Gate 9 | [`gate-9-token-bazaar-browser-proof.md`](../archive/specs/gate-9-token-bazaar-browser-proof.md) (`token_bazaar`; `secret_draft` deferred to a Gate 9.1 commitment/reveal gate) | Done |
| 7M | Gate 9 aftermath / web README realignment | [`gate-9-aftermath-roadmap-realignment.md`](../archive/specs/gate-9-aftermath-roadmap-realignment.md) realigns the web-shell README (intro, Shell Surface, Smoke Layers) to register Token Bazaar after Gate 9. | Done |
| - | Non-gate (UI infra) | [`rules-display-shared-surface.md`](rules-display-shared-surface.md) | Done |
| 8 | Gate 9.1 | [`gate-9-1-secret-draft-commitment-reveal.md`](../archive/specs/gate-9-1-secret-draft-commitment-reveal.md) (`secret_draft` / Veiled Draft; simultaneous commitment/reveal and pending-seat no-leak proof) | Done |
| 9 | Gate 10 | [`poker_lite` / Crest Ledger betting-showdown proof](../archive/specs/gate-10-poker-lite-betting-showdown.md) complete; `plain_tricks` trick/follow-suit proof not yet specced | In progress |
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
    For a web-exposed game gate, this MUST name the web-shell catalog README
    ([`../apps/web/README.md`](../apps/web/README.md)) — its intro catalog list,
    Shell Surface renderer list, and Smoke Layers `smoke:e2e` list — as a closeout
    surface so the gate (via its capstone ticket) reconciles it rather than a later
    aftermath pass. `scripts/check-catalog-docs.mjs` enforces the intro and smoke
    lists in CI; see [`../docs/OFFICIAL-GAME-CONTRACT.md`](../docs/OFFICIAL-GAME-CONTRACT.md) §10/§12.
11. **Sequencing** — predecessor/successor gate; admission rule.
12. **Assumptions** — one-line-correctable.

## Workflow

1. Pick the lowest non-`Done` gate or maintenance interlock from the index.
2. Before drafting a new mechanic-ladder spec, check `docs/MECHANIC-ATLAS.md` for open promotion debt and close it first unless an accepted exception or ADR says otherwise.
3. Write its spec from the format above, grounded in ROADMAP + the foundation set.
4. Decompose the work breakdown into `tasks/` AGENT-TASK packets after the spec is accepted.
5. Execute, gathering the acceptance evidence.
6. When exit criteria pass, flip the index status to `Done` and admit the next gate.
