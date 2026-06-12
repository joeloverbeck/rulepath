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
| - | Non-gate (UI infra) | [`rules-display-shared-surface.md`](../archive/specs/rules-display-shared-surface.md) | Done |
| - | Non-gate (UI infra) | [`victory-explanation-shared-surface.md`](../archive/specs/victory-explanation-shared-surface.md) | Done |
| 8 | Gate 9.1 | [`gate-9-1-secret-draft-commitment-reveal.md`](../archive/specs/gate-9-1-secret-draft-commitment-reveal.md) (`secret_draft` / Veiled Draft; simultaneous commitment/reveal and pending-seat no-leak proof) | Done |
| 9 | Gate 10 | [`poker_lite` / Crest Ledger betting-showdown proof](../archive/specs/gate-10-poker-lite-betting-showdown.md) plus [`plain_tricks` / Plain Tricks trick-taking proof](../archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md) complete the betting/showdown and trick/follow-suit halves | Done |
| 10 | Gate 10.1 | [`gate-10-1-plain-tricks-trick-taking-proof.md`](../archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md) (`plain_tricks` / Plain Tricks; lead/follow legality, trick resolution, round scoring, deal rotation; carries the third-use card/private-hand ledger hard gate; closes the remaining Gate 10 trick rows) | Done |
| 11 | Gate 11 | [`gate-11-masked-claims-bluff-reaction-proof.md`](../archive/specs/gate-11-masked-claims-bluff-reaction-proof.md) (`masked_claims` / Masked Claims; claim/challenge reaction-window, pending-response, and conditional-resolution proof; carries the fourth-use shuffle/private-hand ledger reopen as pre-implementation work) | Done |
| 12 | Gate 12 | [`gate-12-flood-watch-cooperative-event-pressure-proof.md`](../archive/specs/gate-12-flood-watch-cooperative-event-pressure-proof.md) (`flood_watch` / Flood Watch; shared-outcome cooperative event-pressure proof: deterministic effect-log-driven environment automation, role powers, multi-action budgets, scenario setup; carries pre-implementation atlas reviews for the reaction-window and deterministic-shuffle rows) | Done |
| 13 | Gate 13 | [`gate-13-frontier-control-asymmetric-area-control-proof.md`](../archive/specs/gate-13-frontier-control-asymmetric-area-control-proof.md) (`frontier_control` / Frontier Control; asymmetric graph-map area-control proof: graph topology, site control, faction-asymmetric actions and scoring, per-faction UI and bots; carries pre-implementation atlas reviews for the board_space audit, role-modifier and multi-action-budget second uses, and the shared-outcome and reaction-window comparison rows) | Done |
| 14 | Gate 14 | [`gate-14-event-frontier-event-complexity-capstone.md`](../archive/specs/gate-14-event-frontier-event-complexity-capstone.md) (`event_frontier` / Event Frontier; event-deck/eligibility-initiative/periodic-scoring/asymmetric-victory capstone proof with scripted policy bots, scenarios, and long-game replay; carries two pre-implementation atlas hard gates — the public-resource-accounting third use and the multi-action-budget third-use candidate — plus the second-use comparisons Gate 13 armed) | Done |
| - | Non-gate (UI infra) | [`card-and-action-presentation-shared-surfaces.md`](../archive/specs/card-and-action-presentation-shared-surfaces.md) (component display metadata + shared deck presentation + shared progressive action construction + catalog copy hygiene; motivated by Gate 14 presentation debt; backfills `event_frontier`/`flood_watch`, audits `frontier_control` and all action panels; future-binding via UI-INTERACTION/OFFICIAL-GAME-CONTRACT amendments) | Done |
| - | Non-gate (UI infra) | [`action-consequence-and-match-context-shared-surfaces.md`](action-consequence-and-match-context-shared-surfaces.md) (action cost/consequence display + faction-first match identity + turn-report narration of bot/automated advances + deep-detail tier + rules/setup fixes incl. variant selector; motivated by the 2026-06-12 `event_frontier` live-app usability audit; carries a runtime raw-identifier DOM guard and the bot-why §15 audit; future-binding via UI-INTERACTION/OFFICIAL-GAME-CONTRACT amendments; capstone evidence recorded in the spec's acceptance evidence) | Done |
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
