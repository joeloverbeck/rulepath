# card-and-action-presentation-shared-surfaces — Component display metadata, deck presentation, and progressive action construction

- **Filename:** `specs/card-and-action-presentation-shared-surfaces.md`
- **Spec ID:** `card-and-action-presentation-shared-surfaces`
- **Target type:** New spec
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate
- **Roadmap build gate:** None. This is a non-gate sibling of `rules-display-shared-surface` and `victory-explanation-shared-surface`, motivated by Gate 14 (`event_frontier`) exposing presentation debt that also affects earlier games.
- **Status:** Planned
- **Date:** 2026-06-12
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → this spec.
- **Subordination:** This spec is subordinate to the foundation docs, area docs, accepted ADRs, and the official-game contract. It drafts lift-ready amendments; it does not silently amend or weaken upstream law.

---

## 1. Objective

Make the play surface communicate **what game components are and do**, and make **action selection feel like playing a game rather than reading a debugger**, across the whole catalog and for every future game.

Event Frontier (Gate 14) is the motivating failure, verified in the running app:

1. **Event cards are meaningless to the player.** The event deck panel shows `Current: high meadow fair / Next: reckoning one`, but nothing anywhere in the UI says what either card will do when it resolves. The names themselves are TypeScript-derived from raw card IDs (`cardLabel()` strips `ef_` and replaces underscores — `apps/web/src/components/EventFrontierBoard.tsx:303-305`) even though authored labels exist in `games/event_frontier/data/cards.toml` and are parsed into `VariantSetup.labels` (`games/event_frontier/src/variants.rs:496-527`) — they are never projected into the public view.
2. **Hidden information is presented as debug output.** The deck panel's third row reads `Hidden order / redacted / undrawn beyond next card` — hardcoded TypeScript placeholder prose (`EventFrontierBoard.tsx:134-137`), not a designed "face-down deck" presentation.
3. **The legal-action panel is a raw tree dump.** The panel titled "Rust legal choices" flattens the action tree to leaves and renders raw path segments: `operation / survey / operation / survey / site charterhouse,site crossing` (`EventFrontierBoard.tsx:226-246`, `collectLeaves` at `:279-285`, `actionLabel` at `:311-313`). It **discards** the human-readable `label` and structure Rust already supplies on every `ActionChoice` (`games/event_frontier/src/actions.rs:427-476`, `:772-778`). Compound actions are not built through staged choices, contrary to the progressive-construction doctrine in `docs/UI-INTERACTION.md` §8.
4. **Debug vocabulary leaks into player-facing copy.** Headings and status text such as "Rust legal choices", "Rust projection", "Rust/WASM supplies card flow, eligibility, operation paths, and public scoring.", and effect/eligibility copy containing raw internal identifiers ("Freeholders is ineligible: event_choice.") violate the play-first, non-debug-dominated doctrine (`docs/FOUNDATIONS.md` §7, §11).

These are not Event-Frontier-only defects. `flood_watch` has the same ID-only event-deck projection (`games/flood_watch/src/visibility.rs:28-31`) and a stub `ui.rs`; `frontier_control`'s public view carries no `ui` metadata field at all; and the established per-game `ui.rs` UiMetadata pattern (`token_bazaar`, `high_card_duel`, `masked_claims`, `plain_tricks`, `secret_draft`, `poker_lite`) was simply not applied to the three newest games. Eight games hand-roll action panels directly from `actionTree.choices` with no shared construction surface.

This spec creates three pieces of shared, future-binding infrastructure:

- **Workstream A — Component display metadata.** Every game whose public view projects component identifiers (cards, contracts, edicts, items) must project Rust-owned, viewer-safe display metadata for them: label, short effect/help text, family/type tag, accessibility label.
- **Workstream B — Shared deck/pile presentation surface.** One shared React surface presents ordered card flows (current / revealed-next / face-down remainder / discard) with designed hidden-information presentation instead of hardcoded "redacted" prose.
- **Workstream C — Shared progressive action construction surface.** One shared React surface walks the Rust action tree stage by stage (family → target(s) → confirm) using Rust-supplied labels, with grouping, back/cancel, and confirmation — replacing flat leaf dumps where boards have no better board-native affordance.

Plus a cross-cutting **player-facing copy hygiene** pass that removes debug vocabulary from normal-mode UI across the catalog.

---

## 2. Current state and motivating evidence

Verified against the working tree and the running app (2026-06-12):

| Surface | Current state | Evidence |
|---|---|---|
| `event_frontier` public view | `current_card`, `next_public_card`, `discard` are raw ID strings; no `ui` field; no card metadata | `games/event_frontier/src/visibility.rs:15-39` |
| `event_frontier` card data | 21 cards; authored labels in TOML; `EF-COMP-012` keeps card data "identity and parameters only"; no description/effect-preview text anywhere | `games/event_frontier/data/cards.toml`, `games/event_frontier/src/cards.rs` |
| `event_frontier` action labels | Rust builds `ActionChoice::leaf(segment, label, accessibility_label)` with copy like "Apply {kind} to {payload}"; the web panel ignores `label` and renders path segments | `games/event_frontier/src/actions.rs:427-476`, `EventFrontierBoard.tsx:239` |
| `flood_watch` public view | `drawn_cards`, `forecast` are raw ID strings; `undrawn_count` public; `ui.rs` stub | `games/flood_watch/src/visibility.rs:28-31`, `games/flood_watch/src/ui.rs` |
| `frontier_control` public view | No `ui` metadata field; `ui.rs` placeholder | `games/frontier_control/src/visibility.rs`, `games/frontier_control/src/ui.rs` |
| Established UiMetadata pattern | `ui.rs` struct + `ui_metadata()` + `pub ui` on PublicView + authored accessibility/preview copy | `games/token_bazaar/src/ui.rs:1-97`, `games/token_bazaar/src/visibility.rs:34` |
| Shared web surfaces | Rules display and outcome explanation are shared and catalog-complete; action selection and deck presentation are per-board one-offs | `apps/web/src/components/RulesPanel.tsx`, `OutcomeExplanationPanel.tsx`; `main.tsx:388-545` |
| Generic fallback action panel | `ActionControls.tsx` renders `choice.label` buttons for boards without custom panels; custom boards bypass it | `apps/web/src/components/ActionControls.tsx:19-48` |
| Debug copy in play mode | "Rust legal choices", "Rust projection", "Rust modifier", waiting text naming Rust/WASM, raw `event_choice` in eligibility status | `EventFrontierBoard.tsx:181, 209, 221, 228`; live-app screenshots |

The effect log is in comparatively good shape (`effectFeedback.ts` produces copy such as "high meadow fair resolved: fair raised provisions and a settler.") and is **not** a target of this spec beyond copy hygiene for raw identifiers.

---

## 3. Goal, scope, and non-goals

### 3.1 Goal

A player looking at any catalog game's play surface can answer, without leaving normal mode: **what is this component, what will it do, what can I do right now, and what am I about to commit to?** — with all answers sourced from Rust-owned views/labels and inert authored presentation metadata, never TypeScript guesses.

### 3.2 In scope

- A typed, viewer-safe **component display metadata** contract projected through per-game public views (Workstream A), backfilled to `event_frontier`, `flood_watch`, and (audit-level) `frontier_control`.
- Authored presentation metadata files in `games/*/data` for event cards (label, short effect summary, family tag, accessibility text), kept separate from rule-bearing card identity/parameters so `EF-COMP-012` stands.
- A shared **deck/pile presentation** React surface adopted by `event_frontier` and `flood_watch` (Workstream B).
- A shared **progressive action construction** React surface adopted by `event_frontier`, with a recorded adoption audit for every other catalog game (Workstream C).
- A catalog-wide **player-facing copy hygiene** pass (normal mode only) plus a CI guard against debug vocabulary and missing component metadata.
- `apps/web/src/wasm/client.ts` type updates for the new viewer-safe payload fields.
- Smoke, accessibility, no-leak, replay/fixture, and serialization test updates the view changes require.
- Lift-ready amendments to `docs/UI-INTERACTION.md` and `docs/OFFICIAL-GAME-CONTRACT.md` (§17 below).

### 3.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Changing any visibility contract (e.g. exposing `event_frontier`'s undrawn count, currently redacted by `EF-VIS-002`) | Forbidden here | Visibility-contract changes are a FOUNDATIONS §13 ADR trigger. This spec improves the *presentation* of redaction; it does not move the redaction line. |
| Game picker / match setup polish (raw variant IDs visible on cards) | Deferred | Real but separable; candidate follow-up spec. Keeping it out bounds this spec to the play surface the user named. |
| Force-migrating every game's action panel to the shared construction surface | Rejected | Board-native affordances (clicking a card in hand, a column, a site) are often *better* UX than a panel. Workstream C defines adoption criteria and records a per-game audit instead. |
| Generic `engine-core` card/deck/metadata types | Forbidden | `engine-core` stays noun-free (FOUNDATIONS §3). Metadata types are per-game Rust; shared TypeScript components live in `apps/web`, which may know mechanic nouns. |
| `game-stdlib` UiMetadata helper promotion | Deferred | The repeated `ui.rs` shape is presentation convention, not a behavior primitive; promotion pressure, if any, routes through `docs/MECHANIC-ATLAS.md` later. This spec standardizes the *contract shape*, not a shared crate helper. |
| Rules-text generation from Rust rule code | Forbidden | Effect summaries are authored inert prose keyed to Rust IDs (`docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§12), not generated or interpreted. |
| YAML or DSL for metadata | Forbidden | TOML/RON typed content only; unknown fields rejected. |
| TypeScript-derived labels from IDs (`replaceAll("_", " ")`) as a sanctioned fallback | Forbidden going forward | That is the defect. TS may render only Rust/static-supplied display text. |
| Tooltip-only access to public-by-rule information | Forbidden | Research-grounded (§5): public info must be at-a-glance; hover/tap detail is for *full* effect text on top of a glanceable summary. |
| Effect-log redesign, animation scheduler work | Not in scope | Out of the named defect set; copy hygiene only. |

---

## 4. Foundation and boundary alignment

| Authority | Constraint engaged | Spec alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §2 | Rust owns views, effects, legality; TS presents only | All new display data is Rust-projected view fields or inert static metadata keyed to Rust IDs. The action surface renders Rust's `ActionTree` stages and labels; it invents no legality and no labels. |
| `docs/FOUNDATIONS.md` §3 | `engine-core` noun-free | No `engine-core` change. `ActionChoice` label/accessibility fields already exist in the generic contract; metadata types stay in `games/*`; "card"-aware components stay in `apps/web`. |
| `docs/FOUNDATIONS.md` §5 + `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12 | Static data is typed content; UI metadata may include labels, short help, grouping tags, accessibility labels; explanation templates keyed to Rust IDs; no behavior fields | Card presentation metadata carries exactly that and nothing else; deserialized into typed Rust structs; unknown fields rejected; validation per §7 of that doc. |
| `docs/FOUNDATIONS.md` §7 / §11 | Public UI play-first, polished, not debug-dominated; legal-only controls unchanged | This spec exists to enforce it. Copy hygiene removes debug vocabulary from normal mode; legal-only behavior is untouched (same trees, same submission paths). |
| `docs/FOUNDATIONS.md` §11 | No hidden-information leaks; deterministic serialization | Metadata is keyed by ID and rendered only for IDs Rust projected to this viewer. Deck composition + discard are already public by rule, so the static card encyclopedia leaks nothing; undrawn *order* and redacted counts stay redacted. New view fields are deterministic and covered by fixture/golden-trace updates. |
| `docs/FOUNDATIONS.md` §12 | Stop conditions: TS legality, static-data behavior, debug-first UI, hidden-info leak | Each is an explicit stop condition for every ticket decomposed from this spec. |
| `docs/FOUNDATIONS.md` §13 | ADR triggers: visibility contracts, replay/hash semantics | Not tripped: no visibility line moves, no hash/replay semantic change — view-schema additions ride the ordinary fixture/trace update path (`docs/TESTING-REPLAY-BENCHMARKING.md`). |
| `docs/UI-INTERACTION.md` §5/§7/§8 | Payload table; legal-only; progressive construction | Workstream C implements §8 for compound actions; payload additions stay within the §5 table (view + UI metadata rows). |
| `docs/IP-POLICY.md` | Original prose/assets | All card labels/summaries are original Rulepath prose; original SVG card-back/face treatments only. |

---

## 5. External UX grounding (presentation details only)

External research does not reopen architecture decisions; it shapes presentation. Sources gathered 2026-06-12:

- **Three-slot deck anatomy.** Face-down pile rendered as a card back (count badge where the count is public by rule), a dedicated face-up "next" slot, and an enlarged current/active slot; reveals are flip transitions, not text swaps. (BGA Studio Cookbook — boardgamearena.com; Wingspan digital reviews, Meeple Mountain.)
- **Two-tier card rendering.** Glanceable mini-card (name + family icon + key stat) with full effect text in a hover-zoom / tap-inspect detail view; key stats anchored at fixed positions; full rules text reserved for the detail tier. (Hearthstone/Slay the Spire/Balatro pattern analyses; BGA `addTooltipHtml` convention.)
- **Public info is at-a-glance, never hover-gated.** Wingspan digital is criticized precisely for hiding public opponent state behind hover. Current/next card identity and short summary must be visible without interaction; only the *full* text may live in the detail tier. (Meeple Mountain.)
- **Status prompt + staged construction.** BGA's house pattern: persistent "who acts / what to do" title line; contextual action buttons per state; client-side states walk choose-action → choose-target → confirm with cheap cancel back to the last committed server state — mapping directly onto Rust-owned trees with TS-side staging. (BGA Studio docs.)
- **Highlight legal targets only after selecting the action/piece**; explicit confirm for commitment. (Workinman "Turning Board Games into Video Games"; Wingspan reviews.)
- **Accessibility anchors.** No essential information by color alone; readable/scalable text; large spaced touch targets; player-paced reveals; reinforce text with visuals. (Game Accessibility Guidelines, gameaccessibilityguidelines.com; WCAG 1.4.1/4.1.3 as already adopted by `victory-explanation-shared-surface`.)
- **Academic anchor.** Rogerson, Gibbs & Smith, "Digitising Boardgames: Issues and Tensions" (DiGRA 2015): digital adaptations hide "articulation work" (shuffles, reveals) and surface new information; both are deliberate design decisions. This spec makes Rulepath's stance explicit: reveals are presented as designed rituals (flip + effect summary), and redaction is presented as a designed face-down pile, not an apology string.

---

## 6. Committed design decisions

### D1 — Component display metadata is Rust-projected, per-game, and ID-keyed

Each game owning ordered card/component flows defines, in its own crate:

- a typed presentation-metadata table loaded from `games/<game_id>/data/` (TOML), keyed by the component ID enum, carrying per component: `label`, `summary` (short effect/help prose), `family` (existing UI family tags, e.g. ordinary/edict/reckoning), `accessibility_label`;
- validation at load: every component ID has exactly one row; unknown IDs/fields rejected (`ENGINE-GAME-DATA-BOUNDARY` §7);
- projection: the public view's existing component references gain resolved display data, either inline (`current_card: Option<CardFaceView>`) or via a `ui` metadata block listing faces for every ID the view projects. Inline resolved views are preferred (one lookup, no TS join).

Rule-bearing card data stays identity + parameters (`EF-COMP-012` unchanged); presentation rows live in a sibling file (e.g. `cards_presentation.toml`).

**Leak stance:** deck composition and discard contents are public by rule in both event-deck games, so component metadata is rulebook-equivalent content. Metadata may only be *rendered* for IDs the viewer-safe view actually projects; the undrawn order and any redacted counts remain redacted. No metadata row may encode position, order, or remaining-count facts.

### D2 — Games missing the UiMetadata channel adopt the established pattern

`event_frontier`, `flood_watch` gain `ui.rs` + `pub ui` view fields per the `token_bazaar` precedent (labels for panels, headings, deck slots, hidden-pile copy, accessibility strings). `frontier_control` gets an audit ticket: adopt the field where its board currently hardcodes copy, or record explicitly why its existing label projection suffices.

The **hidden-pile presentation copy** ("Order unknown until drawn", face-down slot labels) is Rust/static-supplied via this channel — replacing the hardcoded "Hidden order / redacted" strings. TS never authors gameplay-meaningful copy.

### D3 — One shared deck surface component

`apps/web/src/components/DeckFlowPanel.tsx` (name indicative): slots for current (enlarged), next (face-up), face-down remainder (card back; count badge only when the game's view supplies a public count; otherwise a designed "order hidden" treatment), and a disclosure-expandable discard list. Mini-card faces show label + family band (color + icon, never color alone) + one-line summary; full summary text in an accessible tooltip/disclosure detail tier. Adopted by `event_frontier` and `flood_watch`; future deck games must use it or record a board-native exception.

### D4 — One shared progressive action construction surface

`apps/web/src/components/ActionPathBuilder.tsx` (name indicative): walks `ActionTree` stage by stage — render the current stage's `choices` as grouped controls using `choice.label` / `accessibility_label`, maintain the selected path, offer Back (pop one stage) and Cancel (reset to root; the last committed Rust state is untouched, so cancel is free), and a Confirm control on leaves summarizing the full path in Rust-label terms. Multi-target stages (the comma-joined `site charterhouse,site crossing` leaves) render as the staged selection Rust's tree already encodes, not as pre-joined leaf strings.

Adoption: mandatory for `event_frontier` (replacing the flat leaf dump). Every other catalog game gets one audit row: *adopt* (panel-driven compound games), *board-native* (clicks on cards/columns/sites already map 1:1 to Rust choices — keep, but route any residual flat leaf lists through the shared surface), or *fallback* (`ActionControls` already adequate for single-stage games). The audit is a deliverable, not optional.

### D5 — Player-facing copy hygiene is catalog-wide and CI-guarded

Normal-mode play surfaces must not contain: the words "Rust", "WASM", "projection", "redacted", "payload", raw snake_case identifiers, or internal enum strings. Dev panel and dev-mode surfaces are exempt. A CI guard (`scripts/check-presentation-copy.mjs`, sibling of `check-catalog-docs.mjs`) scans `apps/web/src/components` player-surface components for the banned vocabulary and verifies that games projecting component IDs ship presentation metadata rows for all of them. Rust-side player-facing strings (eligibility/effect copy such as "ineligible: event_choice") switch to label-resolved copy with unit coverage.

### D6 — Future-binding contract

A new official web-exposed game is not UI-complete unless: component flows project display metadata (D1), deck flows use the shared surface or record an exception (D3), action selection passes the D4 audit, and the copy guard passes. This lands as lift-ready amendments to `docs/UI-INTERACTION.md` §19 and `docs/OFFICIAL-GAME-CONTRACT.md` (§17 below).

---

## 7. Deliverables

```text
games/event_frontier/data/cards_presentation.toml      (21 authored rows)
games/event_frontier/src/ui.rs                          (UiMetadata + card-face resolution)
games/event_frontier/src/visibility.rs                  (ui field; resolved card faces; copy fixes)
games/event_frontier/src/effects.rs                     (label-resolved eligibility/effect copy)
games/flood_watch/data/cards_presentation.toml          (authored rows for its event cards)
games/flood_watch/src/ui.rs + visibility.rs             (same enrichment)
games/frontier_control/                                  (audit outcome: ui field or recorded exception)
apps/web/src/wasm/client.ts                             (CardFaceView/UiMetadata type additions)
apps/web/src/components/DeckFlowPanel.tsx               (shared deck surface)
apps/web/src/components/ActionPathBuilder.tsx           (shared progressive construction surface)
apps/web/src/components/EventFrontierBoard.tsx          (adopt both surfaces; copy hygiene)
apps/web/src/components/FloodWatchBoard.tsx             (adopt deck surface; copy hygiene)
apps/web/src/components/*Board.tsx                      (copy-hygiene pass; audit-driven changes only)
scripts/check-presentation-copy.mjs                     (CI guard)
apps/web/e2e/                                            (smoke updates: deck surface, action builder, no-leak)
docs/UI-INTERACTION.md, docs/OFFICIAL-GAME-CONTRACT.md  (lift-ready amendments applied on acceptance)
apps/web/README.md                                       (Shell Surface section: new shared surfaces)
specs/README.md                                          (index row maintained)
```

Per-game Rust changes ride the ordinary verification set: unit/rule tests, fixture-check, replay-check, serialization tests, golden-trace updates where view JSON changes, simulate smoke.

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

Dependency order; each item is one reviewable diff.

| # | Item | Depends on |
|---|---|---|
| WB1 | `event_frontier` presentation metadata: `cards_presentation.toml` (21 authored summaries), typed loader + validation, `ui.rs` UiMetadata, view projection of resolved card faces + `ui` field; fixture/trace/serialization updates | — |
| WB2 | `event_frontier` Rust copy hygiene: label-resolved eligibility/effect strings (no raw `event_choice`-style identifiers in player-facing text); unit coverage | WB1 |
| WB3 | `flood_watch` parity: presentation metadata + `ui.rs` + view projection; fixture/trace updates | — (pattern from WB1) |
| WB4 | `frontier_control` UiMetadata audit: adopt or record exception; small backfill if adopted | — |
| WB5 | `client.ts` types + shared `DeckFlowPanel`; adopt in `EventFrontierBoard` and `FloodWatchBoard`; remove hardcoded hidden-order prose; smoke coverage | WB1, WB3 |
| WB6 | Shared `ActionPathBuilder`; adopt in `EventFrontierBoard` (replace leaf dump); smoke coverage incl. back/cancel/confirm and stale-tree rejection path | WB1 |
| WB7 | Catalog action-presentation audit: one recorded row per game (adopt / board-native / fallback) + any residual flat-leaf-list migrations the audit mandates | WB6 |
| WB8 | Catalog copy-hygiene pass (normal-mode headings/status text across all boards) + `scripts/check-presentation-copy.mjs` + CI wiring | WB5, WB6 |
| WB9 | Closeout: lift amendments into `docs/UI-INTERACTION.md` / `docs/OFFICIAL-GAME-CONTRACT.md`, update `apps/web/README.md` Shell Surface, flip this spec's index row to `Done` with evidence | all |

---

## 9. Exit criteria

1. Playing `event_frontier` in normal mode, the current and next event cards show an authored label, family band, and one-line effect summary at a glance, with full summary in an accessible detail tier; the face-down remainder is a designed card-back treatment whose copy comes from the Rust/static channel; the discard is browsable.
2. No normal-mode play surface in the catalog renders the banned debug vocabulary or raw snake_case identifiers; `scripts/check-presentation-copy.mjs` passes in CI and fails on reintroduction.
3. `event_frontier` action selection is staged (family → targets → confirm) with Back/Cancel, rendered from Rust labels; no flat leaf-path dump remains; every submitted path is byte-identical to what the old panel would have submitted (replay/serialization unchanged).
4. The catalog action audit exists with one explicit row per game; every "adopt" row is implemented; no game silently keeps a flat leaf list.
5. `flood_watch` reaches presentation parity (metadata + deck surface); `frontier_control` audit row recorded.
6. All view-schema additions pass `cargo test --workspace`, `fixture-check`, `replay-check`, and updated golden traces for the touched games; `simulate` smoke unchanged in behavior.
7. No-leak evidence: DOM/test-ID/log sweeps show no hidden order, no redacted counts, and metadata rendered only for view-projected IDs (per `docs/UI-INTERACTION.md` §12).
8. Accessibility: deck slots and card faces have accessible names; detail tiers use disclosure semantics; family conveyed by color + icon/text; reduced motion preserves all information; existing smoke baseline extended.
9. `docs/UI-INTERACTION.md` and `docs/OFFICIAL-GAME-CONTRACT.md` amendments applied; `apps/web/README.md` updated; `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` pass.

---

## 10. Acceptance evidence

- Rust: unit tests for metadata load/validation (duplicate/missing/unknown IDs), label-resolved copy, view projection; updated serialization/golden-trace fixtures; `cargo fmt/clippy/build/test` gate 0 clean.
- Tools: `fixture-check`, `replay-check`, `rule-coverage` for `event_frontier` and `flood_watch`; `boundary-check.sh` clean (no kernel contamination).
- Web: `smoke:wasm`, `smoke:ui`, `smoke:effects` green; new smoke cases for deck surface, action builder staging, and copy guard; no-leak DOM sweep evidence attached to the closeout ticket.
- Screenshots: before/after of the event deck panel and action panel attached to WB5/WB6 outcomes.

---

## 11. Forbidden changes

- No visibility-contract change (no new counts, orders, identities, or reveal timing); `EF-VIS-002` redaction stance unchanged.
- No `engine-core` edits.
- No `game-stdlib` additions; no helper promotion outside the atlas process.
- No TypeScript-computed legality, ordering, or label derivation from IDs.
- No behavior-looking fields in metadata files; no YAML; no DSL.
- No replay/hash semantic changes; trace updates only through the ordinary migration path.
- No weakening or deletion of existing tests (AGENT-DISCIPLINE §4).
- No changes to bot policy, scoring, legality, or effects beyond copy-string resolution.

---

## 12. Documentation updates required

- `docs/UI-INTERACTION.md`: §19 acceptance additions (component metadata, deck surface, staged construction, copy guard) — lift-ready text in §17.
- `docs/OFFICIAL-GAME-CONTRACT.md`: UI-metadata requirement extended to component display metadata for component-flow games.
- `apps/web/README.md`: Shell Surface section gains the two shared components.
- `specs/README.md`: index row for this spec (added at spec authoring; flipped to `Done` at WB9 with evidence).
- Game docs: `games/event_frontier/docs/` and `games/flood_watch/docs/` UI metadata notes updated.

---

## 13. Sequencing

- **Predecessor:** Gate 14 (`event_frontier`) — Done; its closeout exposed this debt. No open promotion debt (atlas §10A empty); no mechanic-ladder gate is blocked by or blocks this work.
- **Position:** Non-gate UI-infrastructure spec, same admission class as `rules-display-shared-surface` and `victory-explanation-shared-surface`: complete before the next product gate (Gate P is private and unaffected; any future public gate inherits D6's future-binding contract).
- **Successor:** Candidate follow-up spec for game-picker/match-setup presentation polish (raw variant IDs, catalog card design) — deliberately out of scope here.

---

## 14. Assumptions (one-line-correctable)

1. **(A1) Spec count** — assuming one combined spec for all three workstreams (the user asked for "a spec"); split into per-surface siblings if preferred.
2. **(A2) Picker/setup scope** — assuming game-picker and match-setup polish stays out (play surface only), recorded as a follow-up candidate in §13.
3. **(A3) Action-panel migration breadth** — assuming audit-with-criteria (D4) rather than force-migrating all 8 custom panels; board-native affordances stay where they are better UX.
4. **(A4) Visibility stance** — assuming `event_frontier`'s undrawn count stays redacted (presentation improves, contract unmoved); exposing it would need an ADR per FOUNDATIONS §13.
5. **(A5) Metadata carrier** — assuming authored TOML presentation files in `games/*/data` resolved through Rust views (D1), not TS-side static tables; flip to Rust-source-only constants if data files feel heavy for 21 rows.
6. **(A6) Naming** — component names `DeckFlowPanel` / `ActionPathBuilder` are indicative, not binding.
7. **(A7) Research basis** — external research used practitioner + accessibility sources and one DiGRA paper; no deeper academic pass was run (one ACM DOI unverified behind a 403). Sufficient for presentation grounding; commission `research-brief` if deeper grounding is wanted.

---

## 15. Triage of observed defects → workstreams

| # | Observed defect (live app, 2026-06-12) | Disposition |
|---|---|---|
| O1 | Event cards unintelligible (IDs, no meaning) | Fix — WB1/WB3/WB5 (Workstreams A+B) |
| O2 | "Hidden order / redacted / undrawn beyond next card" debug prose | Fix — WB1/WB5 (D2/D3) |
| O3 | Legal-choices panel: flat leaf dump, duplicated segments, ignored Rust labels | Fix — WB6 (Workstream C) |
| O4 | Debug vocabulary in normal-mode headings/status ("Rust legal choices", "Rust projection", "event_choice") | Fix — WB2/WB8 (D5) |
| O5 | Discard pile count-only, not browsable | Fix — WB5 (D3) |
| O6 | Same gaps latent in `flood_watch`, `frontier_control` | Fix/audit — WB3/WB4 |
| O7 | Picker/setup shows raw variant IDs | Deferred — §13 successor candidate (A2) |

---

## 16. Risks

- **Golden-trace churn:** adding view fields touches fixtures/traces for two games; mitigated by riding the documented trace-update path, one game per ticket.
- **Copy-guard false positives:** dev-panel components share files with play components in places; the guard must scope to normal-mode surfaces — guard design is part of WB8, with an allowlist file rejected in favor of component-scoped scanning if feasible.
- **ActionPathBuilder regression risk:** submitted paths must stay byte-identical (exit criterion 3); smoke + replay determinism tests cover it.
- **Authoring load:** ~30 short summaries of already-implemented effects; original prose, IP-clean by construction.

---

## 17. Lift-ready amendment text (applied at WB9, not before)

**`docs/UI-INTERACTION.md` §19 additions:**

```text
- components with ordered card/component flows display Rust/static-supplied
  labels and short effect summaries at a glance; full text sits in an
  accessible detail tier; TypeScript derives no display text from IDs;
- face-down/redacted piles use the shared deck presentation with
  Rust/static-supplied copy; no hardcoded redaction prose;
- compound actions route through the shared progressive construction
  surface or a recorded board-native mapping; no flat leaf-path dumps;
- normal-mode surfaces contain no engine/debug vocabulary or raw internal
  identifiers; the presentation-copy CI guard passes.
```

**`docs/OFFICIAL-GAME-CONTRACT.md` (UI metadata clause) addition:**

```text
Games whose public views project component identifiers MUST project
viewer-safe component display metadata (label, short summary, family tag,
accessibility label) for every projected identifier, loaded from typed
static presentation data or authored Rust copy, validated at load.
```
