# Rulepath Implementation Specs

This directory holds **implementation specs**: one bounded plan per roadmap gate.

Specs sit between law and execution:

- [`../docs/ROADMAP.md`](../docs/ROADMAP.md) is the prescriptive ladder (law).
- A **spec** turns one roadmap gate into a concrete, reviewable plan.
- A filled [`../templates/AGENT-TASK.md`](../templates/AGENT-TASK.md) is the
  bounded packet an agent or human actually executes.

Specs are subordinate to the foundation set in
[`../docs/README.md`](../docs/README.md). A spec MUST NOT redefine or override any
foundation contract. Where a spec and a foundation document disagree, the
foundation document wins.

## Epoch rollover note (2026-06-13)

This index was rolled over on 2026-06-13 to open the **public scaling phase** —
the next-phase ladder that proves 3+ official seats and substantially larger
surfaces through public, IP-safe games. The pre-rollover index (the fully
annotated record of Gates 0–14) is frozen at
[`../archive/specs/README-2026-06-13.md`](../archive/specs/README-2026-06-13.md).

The scaling phase is seeded from two advisory research reports:

- [`../archive/reports/foundation-doc-realignment.md`](../archive/reports/foundation-doc-realignment.md)
  — the doc/template realignment the phase needs before execution.
- [`../archive/reports/public-game-ladder-and-implementation-order.md`](../archive/reports/public-game-ladder-and-implementation-order.md)
  — the public game ladder (Gate 15+) and phased implementation order.

**Roadmap admission.** ADR 0007 is accepted, and `docs/ROADMAP.md` now records
the public scaling phase after Gate 14 with Gate P moved to the final tail. The
Gate 15+ rows below are admitted as roadmap law, but remain `Not started` until
their predecessor specs are authored and closed in order. The infrastructure
interlocks additionally depend on the Phase 0 doctrine: the multi-seat contract,
N-seat no-leak taxonomy, and template realignment.

## Completed — public mechanic ladder (Gates 0–14)

This ladder is `Done`. Full annotations and links live in the frozen snapshot
[`../archive/specs/README-2026-06-13.md`](../archive/specs/README-2026-06-13.md);
each archived spec carries its own Outcome section.

| Stage | Gate | Game / focus | Spec | Status |
|---:|---|---|---|---|
| 0 | Gate 0 | Repository skeleton | [`gate-0-repository-skeleton.md`](../archive/specs/gate-0-repository-skeleton.md) | Done |
| 1 | Gate 1 | `race_to_n` | [`gate-1-race-to-n.md`](../archive/specs/gate-1-race-to-n.md) | Done |
| 1 | Gate 2 | Trace/replay/benchmark hardening | [`gate-2-trace-replay-benchmark-hardening.md`](../archive/specs/gate-2-trace-replay-benchmark-hardening.md) | Done |
| 1 | Gate 3 | WASM static web shell | [`gate-3-wasm-static-web-shell.md`](../archive/specs/gate-3-wasm-static-web-shell.md) | Done |
| 2 | Gate 4 | `three_marks` | [`gate-4-three-marks-board-smoke.md`](../archive/specs/gate-4-three-marks-board-smoke.md) | Done |
| 3 | Gate 5 | `column_four` | [`gate-5-column-four-public-polish.md`](../archive/specs/gate-5-column-four-public-polish.md) | Done |
| 4 | Gate 6 | `directional_flip` | [`gate-6-directional-flip.md`](../archive/specs/gate-6-directional-flip.md) | Done |
| 5 | Gate 7 | `draughts_lite` (compound action tree) | [`gate-7-draughts-lite-compound-action-tree.md`](../archive/specs/gate-7-draughts-lite-compound-action-tree.md) | Done |
| 5M | Gate 7.1 | `board_space` primitive back-port | [`gate-7-1-board-space-primitive-back-port.md`](../archive/specs/gate-7-1-board-space-primitive-back-port.md) | Done |
| 5M | Gate 7.2 / 6 | `high_card_duel` (hidden-info / chance) | [`gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`](../archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md) | Done |
| 6C | Post-Gate-8 | Blackjack-lite placement audit | [`../docs/adr/0006-blackjack-lite-roadmap-placement.md`](../docs/adr/0006-blackjack-lite-roadmap-placement.md) | Done |
| 6M | Gate 8 aftermath | Roadmap realignment | [`gate-8-aftermath-roadmap-realignment.md`](../archive/specs/gate-8-aftermath-roadmap-realignment.md) | Done |
| 7 | Gate 9 | `token_bazaar` | [`gate-9-token-bazaar-browser-proof.md`](../archive/specs/gate-9-token-bazaar-browser-proof.md) | Done |
| 7M | Gate 9 aftermath | Web README realignment | [`gate-9-aftermath-roadmap-realignment.md`](../archive/specs/gate-9-aftermath-roadmap-realignment.md) | Done |
| 8 | Gate 9.1 | `secret_draft` (commitment/reveal) | [`gate-9-1-secret-draft-commitment-reveal.md`](../archive/specs/gate-9-1-secret-draft-commitment-reveal.md) | Done |
| 9 | Gate 10 | `poker_lite` (betting/showdown) | [`gate-10-poker-lite-betting-showdown.md`](../archive/specs/gate-10-poker-lite-betting-showdown.md) | Done |
| 10 | Gate 10.1 | `plain_tricks` (trick-taking) | [`gate-10-1-plain-tricks-trick-taking-proof.md`](../archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md) | Done |
| 11 | Gate 11 | `masked_claims` (bluff/reaction) | [`gate-11-masked-claims-bluff-reaction-proof.md`](../archive/specs/gate-11-masked-claims-bluff-reaction-proof.md) | Done |
| 12 | Gate 12 | `flood_watch` (cooperative event pressure) | [`gate-12-flood-watch-cooperative-event-pressure-proof.md`](../archive/specs/gate-12-flood-watch-cooperative-event-pressure-proof.md) | Done |
| 13 | Gate 13 | `frontier_control` (asymmetric area control) | [`gate-13-frontier-control-asymmetric-area-control-proof.md`](../archive/specs/gate-13-frontier-control-asymmetric-area-control-proof.md) | Done |
| 14 | Gate 14 | `event_frontier` (event-complexity capstone) | [`gate-14-event-frontier-event-complexity-capstone.md`](../archive/specs/gate-14-event-frontier-event-complexity-capstone.md) | Done |
| — | Non-gate (UI infra) | Rules-display shared surface | [`rules-display-shared-surface.md`](../archive/specs/rules-display-shared-surface.md) | Done |
| — | Non-gate (UI infra) | Victory-explanation shared surface | [`victory-explanation-shared-surface.md`](../archive/specs/victory-explanation-shared-surface.md) | Done |
| — | Non-gate (UI infra) | Card & action presentation shared surfaces | [`card-and-action-presentation-shared-surfaces.md`](../archive/specs/card-and-action-presentation-shared-surfaces.md) | Done |
| — | Non-gate (UI infra) | Action-consequence & match-context surfaces | [`action-consequence-and-match-context-shared-surfaces.md`](../archive/specs/action-consequence-and-match-context-shared-surfaces.md) | Done |
| — | Non-gate (UI infra) | Effect animation & turn orchestration | [`effect-animation-and-turn-orchestration.md`](../archive/specs/effect-animation-and-turn-orchestration.md) | Done |
| — | Non-gate (UI infra) | Catalog/setup visual redesign | [`catalog-setup-visual-redesign.md`](../archive/specs/catalog-setup-visual-redesign.md) | Done |

## Active epoch — public scaling phase (progress tracker)

This table is the **living progress record** for the public scaling phase. A new
brainstorm that wants to "produce the next spec to continue the roadmap" should
read this first and pick the lowest unit whose status is not `Done`, honoring the
interlocks below (open primitive-promotion debt in
[`../docs/MECHANIC-ATLAS.md`](../docs/MECHANIC-ATLAS.md) closes before the next
mechanic-ladder gate). Phase 0 is closed; every later unit is a forward seed
authored when it becomes the lowest non-`Done` row. `docs/ROADMAP.md` is not
edited to record progress; this table is.

| Order | Unit | Spec | Status | Interlock |
|---:|---|---|---|---|
| 0 | Phase 0 — Foundation realignment & next-phase admission | [`phase-0-next-phase-foundation-realignment.md`](../archive/specs/phase-0-next-phase-foundation-realignment.md) | Done | ADR 0007 accepted; multi-seat contract, foundation docs, templates, ROADMAP, and this index reconciled. Evidence below. |
| 1 (15A) | Infra A–D — N-seat setup/catalog, simulator summaries, multi-seat shell, N-player no-leak harness | [`infra-a-d-n-seat-public-infrastructure.md`](../archive/specs/infra-a-d-n-seat-public-infrastructure.md) | Done | Completed 2026-06-14. Seat metadata/setup bridge, seat-keyed simulator summaries, shared seat frame, and pairwise no-leak harness landed; exit checks recorded in the spec Outcome. |
| 5 | Gate 15 — River Ledger / Texas Hold'Em base | [`gate-15-river-ledger-texas-holdem-base.md`](../archive/specs/gate-15-river-ledger-texas-holdem-base.md) | Done | Completed 2026-06-14. First official 3-6-seat hidden-information betting game; fixed-limit capped-raise; split pots; Rust-authored showdown; N-player no-leak. |
| — | Non-gate (River Ledger UX) — showdown legibility & table presentation | [`river-ledger-showdown-legibility-and-table-presentation.md`](../archive/specs/river-ledger-showdown-legibility-and-table-presentation.md) | Done | Completed 2026-06-15. Rust-authored showdown explanation fields + panel redesign + worked-example e2e; neutral cards, hand-ranking reference, action/seat/turn copy, leak-safe teaching aid, no-casino audit; `RULE-COVERAGE.md` UI rows and `UI.md` reconciled. Correctness audit (Part A) found no defect. Independent of Gate 15.1. |
| — | Non-gate (River Ledger UX) — showcase presentation V2 | [`river-ledger-showcase-ux.md`](../archive/specs/river-ledger-showcase-ux.md) | Done | Completed 2026-06-16. RIVLEDSHOWUX-001..017 shipped Rust-authored seat/action/board/ledger copy, V2 showdown ranked standings and card-usage marks, live-region result banner, central-board table recomposition, River-scoped tokens, scheduler-routed River effects, viewer-safe bot "Why?", original catalog icon, docs reconciliation, and no-leak/browser proof. Evidence: archived tickets `archive/tickets/RIVLEDSHOWUX-001.md` through `archive/tickets/RIVLEDSHOWUX-017.md`; final docs/status closeout recorded in RIVLEDSHOWUX-017. |
| 6 | Gate 15.1 — River Ledger all-in / side pots | _(seed; unwritten)_ | Not started | Pending Gate 15. Public-resource/allocation accounting; kept separate from base Hold'Em. |
| 7 | Gate 16 — Hearts | _(seed; unwritten)_ | Not started | Pending Gate 15. Fixed 4-seat trick-taking; trick-taking promotion evaluation. |
| 8 | Gate 17 — Oh Hell | _(seed; unwritten)_ | Not started | Pending Gate 16. Variable-N (3–7) bidding/trick-taking; trick-taking helper promotion likely. |
| 9 | Gate 18 — Spades (partnerships) | _(seed; unwritten)_ | Not started | Pending Gate 17. Teams/partnership scoring + UI grouping. |
| 10 | Gate 19 — Five Hundred Rummy | _(seed; unwritten)_ | Not started | Pending Gate 18. Public meld tableau + private hands; meld/tableau primitive pressure. |
| 11 | Gate 20 — Star Halma / Chinese Checkers | _(seed; unwritten)_ | Not started | Pending Gate 19. 121-space board topology; topology helper hard gate likely. |
| 12 | Gate 21 — Pachisi-family race | _(seed; unwritten)_ | Not started | Pending Gate 20. Track topology + deterministic chance; capture/safety semantics. |
| 13 | Gate 22 — Four Winds Melds (scoped Mahjong-family) | _(seed; unwritten)_ | Not started | Pending Gate 21. Reaction-window hard gate; wall/concealed-set no-leak. |
| 14 | Gate 23 — Commonwealth Frontier capstone | _(seed; unwritten)_ | Not started | Pending Gate 22 + all armed atlas promotions resolved. Medium-heavy original asymmetric map. |
| 15 | Gate P — private monster-game red-team | _(private; non-public)_ | Not started | Last. Isolated, optional; must not drive public architecture. |

Status values: `Not started` → `Planned` (spec written) → `In progress`
(AGENT-TASKs executing) → `Done` (gate exit criteria pass). Flip a spec to
`Done` only after its exit-criteria section is satisfied with evidence.

### Phase 0 closeout evidence

Completed: 2026-06-13

Evidence:

- ADR 0007 is `Status: Accepted`.
- `docs/ROADMAP.md` records the public scaling phase after Gate 14 and Gate P as
  the tail.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `node scripts/check-catalog-docs.mjs` passed (`catalog-docs check passed — 14 games reflected in intro, root, and smoke surfaces`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).

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

1. Pick the lowest non-`Done` unit from the active-epoch tracker. The public
   scaling phase is admitted by accepted ADR 0007 and recorded in ROADMAP; the
   next unit after Phase 0 is the combined Infra A–D unit (`15A`), now `Planned`.
2. Before drafting a new mechanic-ladder spec, check `docs/MECHANIC-ATLAS.md` for
   open promotion debt and close it first unless an accepted exception or ADR says
   otherwise.
3. Write its spec from the format above, grounded in ROADMAP + the foundation set.
4. Decompose the work breakdown into `tickets/` AGENT-TASK packets via
   `/reassess-spec` then `/spec-to-tickets` after the spec is accepted.
5. Execute, gathering the acceptance evidence.
6. When exit criteria pass, flip the index status to `Done` and admit the next unit.
