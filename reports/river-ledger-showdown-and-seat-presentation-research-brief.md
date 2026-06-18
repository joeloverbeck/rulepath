# Research brief — River Ledger: showdown-winner contradiction + seat-presentation defects

> **You are ChatGPT-Pro, Session 2 of a two-stage routine.** Session 1 (Claude, with full
> repository access) already explored the repo, drove the live app, and interviewed the user to
> lock the intent below. **Your job is to produce the deliverable in §7 directly.** Do **not**
> interview, do **not** ask clarifying questions — the requirements here are final. Explore the
> repository and research online as deeply as needed, then deliver.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-18_c4910a2.txt`) is the path inventory of the
`joeloverbeck/rulepath` repo — a Rust-first, rule-enforcing, replayable, testable card/board-game
platform where **Rust owns all behavior and TypeScript/React present only**. The foundation docs
are an ordered, layered authority indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution)
→ `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier
documents govern later ones, and accepted ADRs supersede them only by explicitly naming the
affected sections.

**Fetch every file from commit `c4910a2` (`c4910a211eb172d6d0eed28997cd9871eb8dbfee`, branch
`main`) — the uploaded manifest reflects exactly that tree.** If any other report you read cites a
different "commit of record," that is that report's own baseline; use `c4910a2`.

`games/river_ledger` is the platform's most complex game — an implementation of Texas Hold 'Em
("River Ledger" is the neutral, IP-safe name; the chip pot is the "ledger"). Two prior UX/correctness
passes already shipped and are archived (see §3); **this task is a delta on that shipped baseline,
not a cold start.**

This brief was authored with direct repo access and a live drive of the running web shell
(`http://127.0.0.1:4173/`). The concrete observations in §3/§4 are ground truth captured from that
session; treat them as starting evidence to confirm and extend, not as the full diagnosis.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything:

```
docs/README.md — the authority order and the layering rule (earlier docs govern later ones).
docs/FOUNDATIONS.md — the constitution: priority order, §2 behavior authority (Rust owns view projection / terminal detection / serialization), §7 public-UI-is-central-product, §11 universal acceptance invariants (deterministic + leak-safe viewer projections), §12 stop conditions (TS legality, hidden-info leak, debug-first UI), §13 ADR triggers. Every change here must satisfy these.
docs/ARCHITECTURE.md — Rust/WASM/TS ownership split; dependency direction; who renders what.
docs/ENGINE-GAME-DATA-BOUNDARY.md — engine-core stays generic + noun-free; mechanic nouns (card, deck, pot, seat, betting) live in games/* — confirms the showdown + seat-label fixes belong in games/river_ledger (Rust), and that shared shell wiring is presentation-only.
docs/UI-INTERACTION.md — the public-UI law: Rust owns legal actions / views / bot explanations; TS owns layout, renderer, accessibility only. Governs the card rendering and the viewpoint picker.
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md — N-seat authority: Rust owns seat-range declaration (min/max/default), stable IP-safe seat labels, per-viewer visibility/redaction, pairwise no-leak. THE governing doc for the seat-count + viewpoint-selection family; reconcile the fix against its seat-label and per-viewer-projection rules.
docs/IP-POLICY.md — public/private content boundary; neutral naming; card/suit presentation must stay original and non-casino. Governs the card-typography fix and any suit-label wording change.
docs/WASM-CLIENT-BOUNDARY.md — the Rust/WASM→browser JSON seam; how view/label/showdown-presentation fields cross to client.ts deterministically. Relevant if the seat-label or showdown payload shape changes.
docs/AGENT-DISCIPLINE.md — bounded-task + failing-test protocol (§4: never delete/weaken tests to get green); the spec's tickets must obey it.
games/river_ledger/docs/RULES.md — the correctness oracle: RL-* rule assertions, showdown/evaluation contract, visibility (RL-VIS-*) and no-leak (RL-NO-LEAK-*) rules, and the UI law rows (RL-UI-SHOWDOWN-001: Rust owns hand names / winner / card-usage; RL-UI-ACTIONS-001).
games/river_ledger/docs/UI.md — the showdown-UI spec (post-RIVLEDSHOWUX target state): V2 ranked-standings payload, decisive-contrast, banner, card-usage marks, design tokens. The fix must keep this contract coherent.
games/river_ledger/docs/RULE-COVERAGE.md — the UI-row coverage matrix + golden-trace mapping; shows which fixtures/traces must be updated when showdown narration or seat labels change.
specs/README.md — the living spec index: confirms Gate 15 (River Ledger base) Done and the two prior non-gate UX specs Done; place the new fix-spec here as a sibling non-gate spec.
tickets/_TEMPLATE.md — the ticket shape your spec must be decomposable into (one reviewable diff per ticket).
archive/specs/river-ledger-showcase-ux.md — the SHIPPED RIVLEDSHOWUX spec (17 tickets, 2026-06-16): V2 showdown presentation, table recomposition, seat-label work, bot "Why?". The delta baseline — do NOT re-propose its shipped surfaces as missing.
archive/specs/river-ledger-showdown-legibility-and-table-presentation.md — the SHIPPED RIVLEDSHO spec (2026-06-15): showdown-explanation fields, neutral card component, hand-ranking reference. Establishes the card component you are repairing.
reports/river-ledger-showcase-ux-report.md — the audit that motivated RIVLEDSHOWUX; its backlog framing and the "leaked seat_N" history are directly relevant to the seat-label regression.
reports/river-ledger-correctness-and-presentation-report.md — the external correctness/presentation audit that motivated RIVLEDSHO; the correctness-oracle framing for showdown.
```

### Code seams to inspect directly (inspect, not read-fully — not pasted here)

- `games/river_ledger/src/showdown.rs` — **the showdown defect lives here.** `resolve_showdown()`
  (`:38`), `winning_seats()` (`:102`, evaluation order), `decisive_comparison()`/`comparison_basis()`/
  `comparison_note()` (narration string builders), `showdown_presentation_v2()` (`~:200`, builds
  `result_banner` + `decisive_reason` + ranked `standings`), `primary_winner()` (`:371`),
  `closest_challenger()`. Note `:40` `winners = winning_seats(...)` vs `:61`
  `winners: allocation.winners` and `:205` `let winners = &allocation.winners;`.
- `games/river_ledger/src/pot.rs` — `allocate_single_pot()` / `winners_in_button_order()` (`~:44`):
  the button-order re-sort that diverges from evaluation order.
- `games/river_ledger/src/state.rs` — `TerminalOutcome::Showdown`, `RiverLedgerShowdownPresentationV2`,
  `ShowdownResultBanner`, `ShowdownDecisiveReason`, `ShowdownStandingPresentation` (the carriers).
- `games/river_ledger/src/ui.rs` — `ui_metadata()` (`:74`) emits `seat_labels(STANDARD_MAX_SEATS)`
  (`:83`); `seat_labels(count)` (`:252`). The seat-overcount source.
- `games/river_ledger/src/ids.rs` — `STANDARD_MIN_SEATS=3`, `STANDARD_MAX_SEATS=6`,
  `STANDARD_DEFAULT_SEATS`.
- `apps/web/src/main.tsx` — `changeSeatFrameViewerMode` (`:354`) with the dead-button guard
  `if (viewerMode.seat === "seat_0" || viewerMode.seat === "seat_1")` (`:360`); wired at `:544`.
  Also the play-mode/setup copy and `seatRoleLabels` (`~:834`).
- `apps/web/src/components/SeatFrame.tsx` — `catalogSeatLabels()` (`:74`) reads
  `game?.seat_labels ?? game?.ui?.seat_labels`; renders the viewpoint button row + seat rail.
- `apps/web/src/components/RiverLedgerCard.tsx` — renders `<strong>{card.label}</strong>` (rank) and
  `<span class="river-ledger-card-suit"><b>{glyph}</b><small>{card.suit}</small></span>` (full suit word).
- `apps/web/src/styles.css` — `.river-ledger-card` (`~:4133`, `text-align:center; padding:12px`),
  `.river-ledger-card-suit` (`~:4168`, `display:inline-flex; justify-content:center`),
  `.river-ledger-card small` (`~:4191`, no width/overflow). The overflow/centering source.
- `crates/wasm-api` — the JSON bridge if seat-label or showdown-presentation payload shape changes.

---

## 3. Settled intentions (locked — these pre-empt every clarifying question)

1. **One deliverable: a single implementation-ready fix-spec** covering all of the issues below,
   styled like the prior `river-ledger-*` specs and decomposable into tickets (`tickets/_TEMPLATE.md`,
   one reviewable diff per ticket). Filename in §7.

2. **Showdown winner contradiction is a Rust correctness defect with top priority.** The user
   observed, verbatim, a single post-showdown "Outcome" block that simultaneously says
   *"Seat 0 wins — The strongest revealed five-card hand receives the ledger."* (banner) and
   *"Seat 1 wins with Two pair, Queens and Fives. … Two pair outranks One pair. Closest challenger:
   Seat 3."* (decisive reason). **Leading root cause (confirmed in code, to be verified by you):**
   `resolve_showdown()` computes `winners = winning_seats(evaluations)` in **evaluation order** and
   builds the decisive-comparison narration from it, but sets `TerminalOutcome.winners =
   allocation.winners` in **button order** (`pot.rs`), and `showdown_presentation_v2()` builds the
   **banner headline** from `allocation.winners`. When button-order `winners[0]` ≠ evaluation-order
   `primary_winner`, the banner names one seat and the decisive line another. **Decision:** establish a
   single authoritative winner source so the banner, the decisive-reason text, the ranked standings,
   and the awarded ledger always name the same seat(s) — including split-pot/tie cases (where the
   narration's "equal category and tie-break ranks" path applies). Fix in Rust; TS only renders.

3. **The seat-count + viewpoint family is fixed platform-wide.** Three live-confirmed surfaces all
   render six seats when the match was configured with four:
   (a) the **viewpoint picker row** ("Observer, Seat 0 … Seat 5");
   (b) the **setup-mode copy** ("Hotseat: Seat 0, … Seat 5 are local"; "Bot vs bot: All 6 seats are
   automated");
   (c) the **right-hand seat rail** (six seat frames; seats 4–5 show "WAITING").
   Root: `ui.rs:83` emits `seat_labels(STANDARD_MAX_SEATS=6)` (catalog *capability* metadata) and the
   shell renders the full catalog label set rather than the **active match's** seat count.
   **Plus** the **dead viewpoint buttons**: `main.tsx:360` hardcodes `seat_0 || seat_1`, so clicking
   Seat 2–5 is silently ignored (live-confirmed: only Observer / Seat 0 / Seat 1 respond).
   **Decision:** (i) seat count for every match-scoped surface must derive from the **active match's**
   seat count, not `STANDARD_MAX_SEATS`; (ii) the dead-button guard must be replaced by a **general
   seat→viewpoint mapping** in the shared shell that works for any declared seat. Because the shell
   shell components are shared, **investigate and fix this across all multi-seat games**, with
   river_ledger as the worked case; the spec must say which other games are affected and require the
   fix to cover them (or explicitly justify any per-game exception). Honor the boundary: **Rust owns
   seat-range/labels/visibility; TypeScript must not hardcode seat identity or decide which seats
   exist.**

4. **Private-view card: contain + center, do not redesign.** Live-confirmed: in the PRIVATE VIEW the
   full suit word (e.g. `diamonds`) overflows the card's right edge (measured ~17px past the 78px-wide
   card), and rank+suit are not visually centered. **Decision:** a minimal, surgical fix —
   contain the suit label within the card bounds and center the rank+suit — consistent with the
   neutral card component shipped by RIVLEDSHO/RIVLEDSHOWUX. **No card redesign, no new visual
   language.** `assumption:` the exact suit presentation — keep the full word (contained + centered) vs.
   abbreviate/truncate to a short label or glyph-only — is **delegated to you**: recommend one with a
   one-line justification; **default = keep the full word, contained and centered**, and keep it
   IP-safe/neutral per `IP-POLICY.md`. The user can override later.

5. **This is a delta on shipped work — build on it, do not rebuild.** RIVLEDSHO (2026-06-15) and
   RIVLEDSHOWUX (17 tickets, 2026-06-16) are **Done and archived**. They built the V2 ranked-standings
   showdown presentation, the neutral card component, the table recomposition, the bot "Why?"
   disclosure, and a prior "seat-label fix." **Do NOT re-recommend any of those shipped surfaces as if
   missing.** Treat the four issues as regressions/gaps *within* that shipped baseline and repair them;
   note explicitly where a shipped fix (e.g. the prior seat-label work) failed to cover the
   capability-vs-active-seat-count distinction or the viewpoint-button wiring.

---

## 4. The task

Author one **implementation-ready fix-spec** (a thorny-fix + presentation-hardening spec) that
diagnoses and prescribes the fix for: (1) the showdown winner contradiction (Rust correctness);
(2) the platform-wide seat-count overcount across viewpoint row, setup copy, and seat rail;
(3) the dead Seat 2–5 viewpoint buttons in the shared shell; and (4) the private-view card suit
overflow + centering. For each, the spec must give the root-cause statement, the prescribed
fix (naming the Rust vs TypeScript ownership split correctly), the acceptance criteria, and the
test/golden-trace/no-leak proof obligations — decomposable into one-reviewable-diff tickets.
Reproduce the showdown contradiction by seed-hunting (a quick bot-vs-bot autoplay sample did **not**
reproduce it — the divergence is seed-dependent), and base the fix on the confirmed mechanism, not
on the single observed string.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as
deeply as needed** — similar open-source Texas Hold 'Em / poker engines and their showdown-winner
determination and human-readable narration; UX/usability literature and prior art on multi-seat
"viewpoint/spectator" selection, hand-result presentation, and playing-card typography on the web;
accessibility and web-design guidance for compact card components and overflow containment. Use it
wherever it sharpens the fix or the acceptance criteria. **Cite sources for any external claim that
shapes a decision.** The deep online research is explicitly your job, not Session 1's.

---

## 6. Doctrine & constraints (honor these — they bound every recommendation)

- `docs/FOUNDATIONS.md` is the constitution: every product-behavior decision must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence requires an accepted
  ADR superseding the affected principle first — never design against it silently.
- **Authority order:** foundation docs govern area docs govern specs govern tickets. If a proposed
  fix conflicts with `MULTI-SEAT-AND-SURFACE-CONTRACT.md` or `FOUNDATIONS.md`, the fix is wrong.
- **TypeScript never decides legality, seat existence, winner, or hand identity.** Legal actions,
  views, seat labels/ranges, visibility, the showdown winner, and hand names all come from
  Rust/WASM. The seat-viewpoint mapping and card layout are presentation-only; they must consume
  Rust-authored data, not hardcode it.
- `engine-core` stays generic and **noun-free** — `card`, `deck`, `pot`, `seat`, `betting` live in
  `games/river_ledger`; earned shared helpers go to `game-stdlib` only via the mechanic atlas.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot explanations,
  or replay exports. The seat-viewpoint fix must preserve per-viewer redaction and pairwise no-leak;
  the showdown reveal must reveal only what the showdown legitimately reveals.
- **Determinism:** replay, hashes, RNG, serialization order, and traces stay deterministic (or are
  explicitly migrated). A change to the winner source or showdown payload must keep golden traces
  deterministic and update them through the proper protocol.
- **No YAML / no DSL** without an accepted ADR; static data stays typed content/metadata only.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`AGENT-DISCIPLINE.md` §4). Add the showdown-contradiction regression as a test/golden trace.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **`specs/river-ledger-showdown-and-seat-presentation-fixes.md`** — **new** file (a non-gate
  Rulepath fix-spec; do not overwrite the archived `river-ledger-showcase-ux.md` or
  `river-ledger-showdown-legibility-and-table-presentation.md`). Match the structure and depth of the
  prior `river-ledger-*` specs so it is directly decomposable by the repo's spec→tickets workflow.

The spec must contain, at minimum:
- a short context/delta framing that names the shipped RIVLEDSHO + RIVLEDSHOWUX baseline and states
  these are regressions/gaps within it (not new features);
- one work-section per issue (showdown contradiction; seat-count family; dead viewpoint buttons;
  private-view card), each with: **root cause** (file/symbol-level, citing the confirmed mechanism),
  **prescribed fix** with the correct Rust-vs-TS ownership split, **acceptance criteria**, and
  **test / golden-trace / no-leak proof obligations**;
- for the seat fix, an explicit **platform-wide impact list** (which other multi-seat games are
  affected) and the requirement to cover them;
- for the card fix, the delegated suit-presentation recommendation with its one-line justification
  and the stated default;
- a decomposition into ticket-sized units (one reviewable diff each), referencing
  `tickets/_TEMPLATE.md`'s shape, and the CI gates each must pass (per `CLAUDE.md` / `specs/README.md`).

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The deliverable set is exactly the one spec named in §7 — new file, correct slug, no overwrite
      of an archived spec.
- [ ] All four issues are covered, each with root cause + fix + acceptance + test/no-leak obligations;
      the showdown fix establishes a **single** winner source so banner, decisive reason, standings,
      and awarded ledger always agree (incl. split/tie).
- [ ] The seat fix derives seat count from the **active match** (not `STANDARD_MAX_SEATS`), replaces
      the `seat_0||seat_1` guard with a general mapping, and is scoped **platform-wide** with the
      affected-games list.
- [ ] Every fix respects the Rust-owns-behavior / TS-presents-only boundary and the
      `MULTI-SEAT-AND-SURFACE-CONTRACT` seat-label + per-viewer-redaction rules; no hidden-info leak
      is introduced.
- [ ] No new doctrine weakens an upstream foundation doc or silently amends an accepted ADR; any true
      divergence is flagged as needing an ADR.
- [ ] No shipped RIVLEDSHO/RIVLEDSHOWUX surface is re-proposed as missing.
- [ ] Every external claim that shaped a decision is cited.
- [ ] Commit `c4910a2` contains every file named in the §2 read-in-full list (it does).
