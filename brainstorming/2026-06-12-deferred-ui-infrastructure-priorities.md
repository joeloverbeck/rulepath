# Deferred / out-of-scope UI-infrastructure work — prioritized implementation brainstorm

- **Date:** 2026-06-12
- **Owner:** joeloverbeck
- **Sources analyzed:** `archive/specs/action-consequence-and-match-context-shared-surfaces.md`
  (§4.3 non-goals, §13 successors, §14 assumptions) and
  `archive/specs/card-and-action-presentation-shared-surfaces.md` (§3.3 non-goals,
  §13 successor, §15 deferred rows) — both Done, archived 2026-06-12.
- **Alignment authorities consulted:** `docs/FOUNDATIONS.md` (read in full),
  `docs/UI-INTERACTION.md` §10/§15/§19, `docs/MECHANIC-ATLAS.md` (§4 hard gate,
  §10A debt register — currently empty), `docs/ROADMAP.md` (Gate 14 Done; Gate P
  private/optional), `docs/AI-BOTS.md`, `docs/IP-POLICY.md`,
  `docs/TESTING-REPLAY-BENCHMARKING.md`, `specs/README.md` workflow rules,
  `docs/adr/` (0001–0006), `templates/**` (15 templates).
- **Status of this document:** brainstorm / prioritization input. It commits to
  nothing; each recommendation names its delivery vehicle (spec, ADR, atlas row,
  or rejection rationale).

---

## 1. Why this pool matters now

All four cross-game UI-infrastructure specs are Done (`rules-display`,
`victory-explanation`, `card-and-action-presentation`,
`action-consequence-and-match-context`). The mechanic atlas carries **zero open
promotion debt**, and no public mechanic-ladder gate remains — Gate P is
private, optional, and explicitly subordinate (FOUNDATIONS §1 priority 5 vs.
priority 1, "polished public playable site"). The deferred pool from the two
archived specs is therefore not a side queue: **it is the natural public
continuation of the roadmap.** The only question is order and admission.

## 2. Candidate inventory (provenance)

| # | Candidate | Where deferred/forbidden | Current legal status |
|---|---|---|---|
| C1 | Effect-driven board animation / animation scheduler | actconmat §4.3 "Deferred … future spec"; cardactpres §3.3 | Doctrine already mandated (FOUNDATIONS §7, §11; UI-INTERACTION §10) — unbuilt |
| C2 | Auto-running bot turns (remove manual trigger; pacing) | actconmat §4.3 "Deferred … candidate follow-up" | No doctrine; orchestration is TS presentation policy |
| C3 | Action-tree restructuring for staged multi-target encoding | actconmat §4.3 "Forbidden … requires ADR" | FOUNDATIONS §13 ADR trigger (replay/path-encoding semantics) |
| C4 | Visibility-contract moves (e.g. expose EF undrawn count) | cardactpres §3.3 "Forbidden here" | FOUNDATIONS §13 ADR trigger; EF-VIS-002 is a design stance |
| C5 | Game-picker visual redesign (catalog card art/layout) | actconmat §4.3 + cardactpres §13 "candidate follow-up spec" | Pure presentation; IP-POLICY gate on assets |
| C6 | Variant descriptions / richer setup copy | actconmat A9 ("revisit if richer setup copy is wanted") | Inert typed content — sanctioned by FOUNDATIONS §5 |
| C7 | `game-stdlib` UiMetadata / label-helper promotion | cardactpres §3.3 "routes through MECHANIC-ATLAS later" | Atlas governs behavior primitives; this is a presentation convention |
| C8 | Effect-log redesign | cardactpres §3.3 "copy hygiene only" | Partially absorbed by `TurnReportPanel`; residue is history UX |

Code-grounded state (verified 2026-06-12): the wasm bridge already projects
ordered, viewer-filtered semantic effects with kinds and targets
(`effectFeedback.ts` consumes them; `TurnReportPanel` groups them per burst);
in `human_vs_bot` one bot turn auto-runs synchronously in the same frame as the
human action (`apps/web/src/main.tsx:165-173`), the manual "Run Bot Turn"
button (`ModeControls.tsx:59`) covers bot-first starts and consecutive bot
turns, and `bot_vs_bot` autoplay already paces with `setTimeout` (520ms; 80ms
reduced-motion, `main.tsx:321-334`). A replay viewer with stepping exists
(`main.tsx:271-287`). `ActionPathBuilder` composes multi-target toggles from
the legal leaf set; submission is byte-identical. `game-stdlib` contains only
`board_space`. Eight of fourteen games carry `ui.rs`.

## 3. External grounding

Bounded practitioner pass (2026-06-12); see §7 note on research degradation.

- **Effects→animation queues are the platform-scale norm.** Board Game Arena's
  modern framework drives all animation from the server's transactional
  notification queue via promise-based handlers (`setupPromiseNotifications`,
  `BgaAnimations` — Element.animate-based, async/awaitable), i.e. exactly
  Rulepath's "semantic effects are authoritative cause; timelines are
  presentation" doctrine, deployed across hundreds of titles
  ([BGA Studio docs](https://en.boardgamearena.com/doc/BgaAnimations),
  [notifications model](https://en.boardgamearena.com/doc/Main_game_logic:_yourgamename.game.php)).
- **Decoupling event emission from presentation consumption** is the canonical
  Event Queue pattern; replay engines re-run recorded commands through the same
  simulation — Rulepath already does both, so the scheduler is the only
  missing piece ([Nystrom, *Game Programming Patterns*, Event Queue](https://gameprogrammingpatterns.com/event-queue.html)).
- **Animation must not block input.** Hearthstone queues actions during attack
  animations rather than locking the UI — a requirement to carry into the
  scheduler design (skip/fast-forward; input stays live)
  ([analysis](https://anykeytostart.wordpress.com/2015/03/19/hearthstone/)).
- The predecessor specs' grounding carries forward: Suburbia's
  compute-before-commit, Root's faction framing, DiGRA 2015 "Digitising
  Boardgames" (reveals as designed rituals — the direct argument for animated,
  paced automation instead of instant state swaps).

---

## 4. Prioritized recommendations

### P1 — Animation scheduler + turn orchestration (C1 + C2, absorbing part of C8) — **do first**

**Status: DONE** — delivered by
`archive/specs/effect-animation-and-turn-orchestration.md` and archived
`EFFANITUR-001` through `EFFANITUR-010` on 2026-06-12.

**What:** One spec, two fused workstreams:

1. A shared **effect-driven animation scheduler** in `apps/web` implementing
   the full UI-INTERACTION §10 requirement list: ordered effects, grouped
   effects, simultaneous/reveal batches, redacted effects, reduced-motion mode,
   interruption by replay stepping, settle-to-view reconciliation. Per-game
   adoption rides the proven audit pattern (adopt / board-native mapping /
   not-applicable row per game), with `event_frontier` and `flood_watch` as
   the motivating adopters (Reckoning bursts, flood phases).
2. **Turn orchestration/pacing** built on it: bot turns and auto-resolved
   phases play out on the animation timeline instead of resolving in the same
   synchronous frame; the residual manual "Run Bot Turn" trigger disappears
   (bot-first starts and consecutive bot turns auto-advance); always-available
   skip/fast-forward; reduced-motion collapses to the existing fast path;
   input never hard-blocks (Hearthstone rule). `bot_vs_bot` autoplay swaps its
   fixed `setTimeout` for the same scheduler.

**Why first:** This is the largest remaining gap between repo law and repo
reality. FOUNDATIONS §7 *already mandates* "Animation MUST be driven by
semantic effects"; §11 lists it as a universal invariant; UI-INTERACTION §10
specs the scheduler — and none of it exists. Every prerequisite was built by
the two archived specs (authored labels, viewer-filtered effect stream, turn
report, copy hygiene), so the data already crosses the boundary; only the
presentation tier is missing. It directly attacks the two audit findings that
survived both specs (O5 invisible automation, O11 manual bot trigger), and it
is the single highest-leverage "polished premium table" investment
(FOUNDATIONS §1 priority 1, §7 aesthetic doctrine). Platform precedent (BGA)
proves the architecture at scale.

**Benefit/effort:** High benefit / medium-high effort (~comparable to either
archived spec; est. 8–12 tickets). No rearchitecting: presentation-only,
replay/hash untouched, no ADR.

**Law alignment:** FOUNDATIONS §2 (Rust still owns effects; TS times their
presentation), §7/§11 (this *implements* the invariant), §12 (animation from
effects, never guessed diffs — the scheduler must consume only the effect
stream). AI-BOTS unchanged (orchestration is when the existing
`run_bot_turn` is *called and rendered*, not how bots decide). Determinism:
wall-clock pacing stays out of Rust; replays/traces byte-identical.

**Amendments warranted:**
- `docs/UI-INTERACTION.md`: §10 gains scheduler acceptance criteria and §19
  rows (precedented lift-ready-amendment path); a new short **orchestration/
  pacing subsection** (auto-advance, skip, never-block-input, reduced-motion
  equivalence) — today no doctrine governs turn pacing at all.
- `templates/GAME-UI.md`: one row for animation/orchestration adoption status
  (adopt / board-native / n-a), mirroring the existing audit-row convention.
- No FOUNDATIONS change; no ADR.

### P2 — Catalog & setup visual redesign (C5 + C6) — **DONE**

**Status: DONE** — delivered by
`archive/specs/catalog-setup-visual-redesign.md` and its CATSETVIS ticket series on
2026-06-13.

**What:** The twice-named successor spec: original visual identity for the
game picker (per-game card art/iconography, layout, hover/focus states),
variant **descriptions** (one-line authored prose per variant — the A9
residue; manifest gains an optional typed field projected like
`*_display_name`), and richer setup framing on the functional base
(variant selector, faction-labeled seats, whole-card click) that
actconmat already shipped.

**Why second:** The picker is the front door of a portfolio site whose stated
win condition is "a visitor thinks: polished playable site, serious
architecture" — and it is currently plain text cards. Functional groundwork is
done, so this is now a bounded, pure-presentation spec. It ranks below P1 only
because P1 upgrades *every minute of play* while this upgrades the first
thirty seconds.

**Benefit/effort:** High first-impression benefit / medium effort. Authoring
load: 14 card treatments + ~6 variant descriptions (inert prose, §5-sanctioned).

**Law alignment:** FOUNDATIONS §5 (typed inert content), §7 (cozy premium
table; no casino/SaaS vibes), §10 + `docs/IP-POLICY.md` (original art only, no
trade-dress proximity — each asset needs the originality check recorded, and
smallest-display-size legibility verification per asset). React+SVG default
holds; no renderer change, so no ADR.

**Amendments warranted:** none to law. `templates/PUBLIC-RELEASE-CHECKLIST.md`
already covers asset/IP verification; the spec should add per-asset IP check
rows to its own closeout rather than amend the template.

### P3 — Effect-log history redesign (C8 residue) — **fold into P1 or defer**

**What's left after P1:** the bottom-of-page log as *browsable history* —
grouping by turn/burst, collapse, filter, possibly click-to-inspect past
states via the existing replay machinery.

**Recommendation:** Do **not** write a standalone spec. The scheduler work
(P1) already restructures effect consumption; give P1 a small workstream that
re-bases `EffectLog` on the same burst-grouping the `TurnReportPanel` uses,
and stop there. Full history-browsing/time-travel UX is real scope with
modest payoff — defer until the replay viewer gets product attention.

### P4 — `game-stdlib` UiMetadata/label-helper promotion (C7) — **defer; record the pressure properly**

**Status: DONE** — disposition recorded by
`archive/specs/effect-animation-and-turn-orchestration.md` and lifted into
`docs/UI-INTERACTION.md` §10A on 2026-06-12.

**Verdict:** Correctly deferred, and should stay deferred. The repeated
`ui.rs` shape is a presentation convention, not a behavior primitive; the
atlas hard gate (MECHANIC-ATLAS §4) governs *mechanic* shapes. Promotion would
buy boilerplate reduction only, while FOUNDATIONS §4 promotion law would force
migration of all eight adopting games or recorded exceptions — churn with no
behavior payoff and nonzero regression surface.

**Amendment warranted (the real gap):** this deferral currently lives only in
an archived spec's out-of-scope table. Either `docs/MECHANIC-ATLAS.md` gains a
small **presentation-shape register** (shape, adopting games, deferral
rationale, revisit trigger: e.g. "a third structural divergence between
`ui.rs` implementations" or "official game count > 20"), or
`docs/UI-INTERACTION.md` states explicitly that repeated presentation shapes
are governed by UI law and are *not* atlas pressure. One paragraph either way;
pick one home so the next audit doesn't re-litigate it.

### P5 — Staged multi-target action encoding (C3) — **defer behind a named ADR trigger**

**Verdict:** Do not do this now, despite the standing tolerance for
significant rearchitecture. The presentation composer already delivers the
staged UX with byte-identical encoding; the only thing restructuring buys
today is shrinking `event_frontier`'s worst-case 41-leaf enumeration — not a
measured problem (bench lanes are green; ADR 0003/0005 thresholds hold).
Speculative restructuring contradicts FOUNDATIONS §12 ("stop and reassess
rather than generalize") and forces a command-encoding + trace migration
across replay law for zero current consumers.

**Named trigger (record it):** when a future game's multi-target legal-leaf
enumeration becomes a *measured* payload/bench/bot-enumeration problem
(indicatively: leaf counts in the hundreds per stage, or a bench-lane
regression attributable to leaf explosion), write the ADR per FOUNDATIONS §13
(per-stage command encoding, replay/trace migration plan, fixture rebuild).
Until then the composer is the sanctioned answer. Suggested home for the
trigger: a successor note in `specs/README.md` or the P1/P2 spec's sequencing
section.

### P6 — Visibility-contract move: expose EF undrawn count (C4) — **reject as debt; revisit only on evidence**

**Verdict:** This is not deferred work; it is a deliberate design stance
(EF-VIS-002; ADR 0004's export taxonomy is built around the same line).
`DeckFlowPanel` already renders public counts for games whose contracts allow
them, so no infrastructure is missing. Moving the line is a per-game *game
design* decision that should be driven by playtest evidence of confusion, not
by architecture grooming. If evidence arrives: one small ADR (FOUNDATIONS §13)
plus fixture/no-leak sweep updates. Nothing to schedule today.

---

## 5. Recommended sequence and vehicles

| Order | Work | Vehicle | ADR? |
|---|---|---|---|
| 1 | Animation scheduler + turn orchestration (C1+C2+part C8) | Done via `archive/specs/effect-animation-and-turn-orchestration.md` | No |
| 2 | Catalog & setup visual redesign (C5+C6) | Done via `archive/specs/catalog-setup-visual-redesign.md`; per-asset IP checks recorded in closeout | No |
| 3 | Effect-log history residue (C8) | Done as burst-grouped `EffectLog`/`TurnReportPanel` work inside spec 1 | No |
| 4 | Presentation-shape register (C7 disposition) | Done as a UI-INTERACTION §10A governance paragraph in spec 1 closeout | No |
| 5 | Staged multi-target encoding (C3) | Dormant; named trigger recorded in spec 1's sequencing/successor section | Yes, when triggered |
| 6 | EF undrawn-count visibility (C4) | Rejected as debt; playtest-evidence-gated | Yes, if ever |

Both specs follow the established pattern: authority order header, foundation
alignment table, audit-row adoption matrix, lift-ready amendments applied at
closeout, `specs/README.md` index row added as `Planned` and flipped to `Done`
with evidence.

## 6. Doc/template amendment summary (the "point it out" ask)

| Doc | Amendment | Driver | Class |
|---|---|---|---|
| `docs/UI-INTERACTION.md` §10/§19 | Scheduler acceptance criteria + §19 rows | P1 | Precedented area-doc lift — no ADR |
| `docs/UI-INTERACTION.md` (new subsection) | Turn orchestration/pacing doctrine (auto-advance, skip, never-block-input, reduced-motion equivalence) | P1 | New area-doc doctrine — no ADR (no FOUNDATIONS principle changes meaning) |
| `templates/GAME-UI.md` | Animation/orchestration adoption-status row | P1 | Template addition |
| `docs/MECHANIC-ATLAS.md` *or* `docs/UI-INTERACTION.md` | Presentation-shape register / explicit non-atlas statement for repeated UI conventions | P4 | One paragraph; pick one home |
| `specs/README.md` | Successor/trigger note for C3 (staged encoding ADR trigger) and C4 (evidence-gated visibility ADR) | P5/P6 | Index hygiene |
| `docs/FOUNDATIONS.md` | **No amendment needed.** Every recommendation lands inside existing law; C3/C4 are already correctly ADR-gated by §13. | — | — |

## 7. Assumptions (one-line-correctable)

1. **(A1) Public polish outranks Gate P** — assuming FOUNDATIONS §1 priority 1
   keeps UI-infra work ahead of the optional private red-team; reorder if Gate
   P is wanted next.
2. **(A2) One fused P1 spec** — assuming animation scheduler and turn
   orchestration ship as one spec because pacing is implemented *by* the
   timeline; split into siblings if a smaller first diff is preferred.
3. **(A3) Research degraded** — mgrep web quota was exhausted (429); research
   fell back to built-in web search with a bounded practitioner pass (BGA,
   Nystrom, Hearthstone) plus the archived specs' carried grounding (Root,
   Suburbia, DiGRA 2015); no fresh academic pass was run. Commission
   `research-brief` before authoring P1 if deeper grounding is wanted.
4. **(A4) Effect-stream sufficiency** — assuming per-game effect coverage is
   rich enough to animate (verified for EF's Reckoning burst by the turn
   report); where sparse, the fix is Rust-side effect coverage, not TS
   inference (carries actconmat A5 forward).
5. **(A5) `brainstorming/` is a sanctioned destination** — assuming this new
   top-level directory (user-named) sits outside the specs/tickets workflow as
   a pre-spec ideation layer; no index obligations created.
6. **(A6) Effort estimates are spec-sized analogies** — P1/P2 sized against
   the two archived specs (8–12 tickets each), not measured; `/reassess-spec`
   will re-ground them.

## 8. Next steps

1. Done: P1 shipped as `archive/specs/effect-animation-and-turn-orchestration.md`.
2. Done: P2 shipped as `archive/specs/catalog-setup-visual-redesign.md`.
3. Land the P4 one-paragraph register with P1's closeout amendments.
4. Record the C3/C5 triggers in P1's sequencing section.
