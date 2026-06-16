# river-ledger-showcase-ux — Showcase-grade River Ledger presentation V2 (ranked-standings showdown, table recomposition, identity)

- **Filename:** `specs/river-ledger-showcase-ux.md`
- **Spec ID:** `river-ledger-showcase-ux`
- **Target type:** New spec
- **Roadmap stage:** Non-gate public-polish spec for the shipped Gate 15 game `river_ledger` — not a mechanic-ladder gate.
- **Roadmap build gate:** None. This is a non-gate post-Gate-15 presentation spec, the same admission class as the archived `river-ledger-showdown-legibility-and-table-presentation` / `victory-explanation-shared-surface` / `card-and-action-presentation-shared-surfaces` UI-infra specs. Motivated by `reports/river-ledger-showcase-ux-report.md` — a **delta** UX audit on the already-shipped `RIVLEDOUT-*` / `RIVLEDSHO-*` state.
- **Status:** Planned
- **Date:** 2026-06-16
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → this spec.
- **Subordination:** This spec is subordinate to the foundation docs, area docs, accepted ADRs, the official-game contract, and `games/river_ledger/docs/RULES.md` (the `RL-*` correctness oracle). It plans presentation work on a correct game; it does not redefine any rule, visibility contract, or foundation principle.

---

## 1. Objective

Make River Ledger feel like the **star showcase game** it is meant to be: after a showdown a player sees *who won, why this winner not the closest challenger, and which cards made the hand* at a single glance; the in-play table reads as a warm, centered tabletop surface rather than a diagnostic panel; and every player-facing string is born clean in Rust. All of this **without moving any evaluation, winner, split, hand-strength, legality, or bot-decision logic out of Rust**.

The motivating input is `reports/river-ledger-showcase-ux-report.md`, a source-level audit at commit `f1d6d39` (current `main` HEAD). Its **Part A** re-audit confirms — again — that River Ledger is **correct** against its `RL-*` contract: no defect in setup/blinds, fixed-limit betting, street closure, single-pot allocation, split remainders, evaluator category/tie-break/kicker ordering, ace-low straight, best-five-of-seven, showdown winner/split/foldout, or no-leak projection. **No correctness work is warranted** (§3.3). The report is explicitly a **delta** on the RIVLEDSHO pass: it does not re-recommend the already-shipped V1 showdown explanation layer, `RiverLedgerCard`, hand-ranking reference, teaching aid, action metadata, seat affordances, or copy audit. It improves their showcase quality and closes the polish/leak items V1 did not.

This spec packages the report's actionable backlog (Part D, the ranked 15-item list) into **one non-gate spec with five workstreams** mirroring the report's slices:

- **WS1 — Presentation correctness & copy guardrails** (report ranks #1–#5): Rust authoring-layer seat-label fix for the raw `seat_N` leak + strengthened runtime/audit guards; Rust-authored per-action display rows (hide irrelevant `Call price`/`Cap left` on Fold/Check); Rust-authored board-slot placeholder labels; player-facing seat-ledger labels with the redundant contribution bar removed; and `Player N → Seat N` status normalization by routing the status line through the **existing** `seat_labels` projection (extending the `event_frontier`-gated path to River Ledger).
- **WS2 — Showdown centerpiece V2** (#6–#9): a `RiverLedgerShowdownPresentationV2` Rust payload (result banner, decisive contrast-seat, ranked standings, folded rows, hole/board card-usage marks, accessibility labels); a River-Ledger V2 renderer branch in the shared outcome panel; card-usage visualization (text/shape, not colour alone); and a terminal result live-region.
- **WS3 — Table composition & motion** (#10–#12): recompose the table into a central board well + compact multi-seat rails + action/status band; add River-scoped design tokens/classes; route board-reveal and showdown pacing through the shared scheduler with a reduced-motion path.
- **WS4 — Bot "why?" & teaching** (#13–#14): a Rust-authored `BotDecisionPublicExplanation` (viewer-safe public facts only) and a compact, non-debug "Why?" disclosure.
- **WS5 — Catalog identity** (#15): an original `river_ledger` SVG catalog icon and theme entry.

The deliberately-absent casino features (all-in, side pots, no-limit/pot-limit, stacks, rake, payouts, tournaments) are **intended Gate-15 exclusions**, tracked as the planned **Gate 15.1** unit, and are out of scope here (§3.3).

---

## 2. Current state and motivating evidence

Verified against the working tree at `f1d6d39` (current `main` HEAD; same commit the report audited):

| # | Surface | Current state | Evidence |
|---|---|---|---|
| #1 | Seat-id leak at authoring layer | Showdown headline/summary/list strings are built from `RiverLedgerSeat::as_str()`, which returns `"seat_{index}"` — so the Rust-authored copy literally reads `"seat_5 wins with Pair of Jacks."`. Masked at runtime only by a TS fallback regex; the static-copy audit catches literals but not runtime Rust strings. | `games/river_ledger/src/showdown.rs:206,215,225,518-524`; `games/river_ledger/src/ids.rs:43`; `scripts/check-presentation-copy.mjs:24` (`seat_[0-9]+`); `apps/web/src/components/outcomeExplanationTemplates.ts` (fallback normalization) |
| #2 | Action panel metadata | Every choice renders `Call price`, `Adds`, `Cap left` rows unconditionally — so Fold and Check show decision-irrelevant `Call price`/`Cap left`. | `apps/web/src/components/RiverLedgerBoard.tsx:314-323` |
| #3 | Board placeholder | Unrevealed slots render a single `Pending` (`<strong>`); the raw `HIDDEN`+clipped `Pendin…` overflow is already mitigated, but there is no Rust-authored street-specific or accessibility slot label. | `apps/web/src/components/RiverLedgerBoard.tsx:113-115` |
| #4 | Seat ledger | Seat metrics read raw `Street` / `Total` / `Private` counters, plus a duplicate unlabeled contribution track-bar. | `apps/web/src/components/RiverLedgerBoard.tsx:217-219,255-264` |
| #5 | Status vocabulary | Active-actor fallback emits `Player {n+1}` while seat panels say `Seat N` — mixed vocabulary. The `seat_labels` projection + `SeatDisplayLabel` TS type **already exist**; `activeActorLabel` consumes them only for `event_frontier`, so River Ledger falls through to the `Player N` fallback. | `ModeControls.tsx:181-189` (event_frontier-gated consume + `Player N` fallback); `SeatFrame.tsx:75` (already consumes `game.seat_labels`); `client.ts:105` (`SeatDisplayLabel {seat,label}`); `wasm-api/src/lib.rs:449-457,618` (`catalog_seat_labels_json` → "Seat N") |
| #6-#9 | Showdown panel (V1) | RIVLEDSHO shipped V1 explanation fields (headline, decisive comparison, per-seat hand name, best-five cards, hand-ranking reference, a11y labels). It is **not** ranked-standings-shaped, does not name the closest challenger contrastively, shows the board once-per-seat rather than once-with-usage-marks, has no hole/board card-usage marks, and uses a11y labels but no `role=status` live region. | RIVLEDSHO Outcome; `games/river_ledger/src/showdown.rs`; `apps/web/src/components/OutcomeExplanationPanel.tsx` |
| #10-#12 | Table layout / motion | Vertical seat rail + blank upper-left region underuse table width; board reveal is reduced-motion-aware (V1) but showdown is not staged through one scheduler burst; no River-scoped token namespace. | `apps/web/src/components/RiverLedgerBoard.tsx`; `apps/web/src/animation/*`; `apps/web/src/styles.css` |
| #13-#14 | Bot explanation | `bots.rs` chooses through the legal action API; there is no viewer-safe in-play "why did the bot do that?" public explanation surface. | `games/river_ledger/src/bots.rs` |
| #15 | Catalog icon | `river_ledger` is absent from the `GameCatalogIcon.tsx` icon map; the showcase game falls back to the generic icon. | `apps/web/src/components/GameCatalogIcon.tsx` (no `river_ledger` key) |
| — | Correctness core | No defect found by the re-audit (Part A); evaluator/pot/visibility unchanged. | report Part A; `evaluator.rs`, golden traces |

This spec is a **delta on already-wired plumbing**. The V1 explanation fields, the neutral card component, and the WASM bridge already exist; WS2 extends them into a V2 presentation object, and WS1/WS3/WS4/WS5 close the table-wide polish, the authoring-layer leak, the bot affordance, and the catalog identity that V1 did not.

---

## 3. Goal, scope, and non-goals

### 3.1 Goal

A novice who cannot fluently read Texas Hold 'Em can, after a showdown, immediately read **who won — with what hand — why over the closest challenger — and which cards made it**, from a compact ranked surface whose every word is Rust-authored. During play the table reads as a centered, cozy, original board-game surface with clean seat-ledger and action copy, a non-debug bot "why?" affordance, and a recognizable River Ledger identity. No TypeScript computes legality, evaluation, winner, hand name, card usage, ordering, or any "why" narrative.

### 3.2 In scope

- **WS1** Routing `showdown.rs` / `visibility.rs` terminal/showdown/rationale strings through a "Seat N" label so they are born clean (reusing the existing `seat_labels` "Seat N" form — see D1, no new label type); extending the existing `event_frontier`-gated `seat_labels` consumption in `activeActorLabel` to River Ledger so the status line reads `Seat N` instead of the `Player N` fallback; strengthened copy/e2e guards that fail `\bseat_\d+\b` in **visible text and accessibility labels** at runtime.
- **WS1** Rust-authored **per-action presentation rows** (which display rows are relevant per `fold`/`check`/`call`/`bet`/`raise`, plus helper text); TS renders only the supplied rows — irrelevant `Call price`/`Cap left` no longer appear on Fold/Check.
- **WS1** Rust-authored **board-slot view** (`reveal_state`, `street_label`, `visual_placeholder_label`, `accessibility_label`, nullable `card`); CSS-safe placeholder layout. No future-card identity emitted.
- **WS1** Rust-authored **seat-ledger display fields** (`This round`, `Hand total`, `Hole cards: N hidden/revealed`, role/status badges); the redundant unlabeled contribution bar removed (or, if retained, labeled with the same Rust-authored values and no chip/money framing).
- **WS2** `RiverLedgerShowdownPresentationV2` Rust payload: `result_banner`, `decisive_reason` (with `contrast_seat`/`contrast_seat_label`/`rule_refs`), `board_cards` (with `used_by_selected`), ranked `standings` (result/allocation labels, hand name, short comparison note, rank-ladder label, hole/board card-usage marks, best-five + a11y label, detail rows, default-expanded flag), and `folded_rows` — additive, reveal-scoped, deterministic.
- **WS2** River-Ledger **V2 renderer branch** in the shared `OutcomeExplanationPanel` keyed to the V2 payload's presence: banner + decisive contrast, board-once layout, compact ranked standings (winner + closest challenger visible, rest collapsed), detail expanders.
- **WS2** Card-usage visualization (used board/hole cards marked by text/shape/ring, not colour alone) and a terminal result **live-region** (`role=status` / atomic or the repo equivalent).
- **WS3** Table recomposition into central board well + compact multi-seat rails (responsive for 3–6 seats) + action/status band; River-scoped design tokens/classes (`--rl-*`) in `styles.css`; board-reveal and staged showdown pacing routed through the shared scheduler with a reduced-motion path and settle assertions.
- **WS4** Rust-authored `BotDecisionPublicExplanation` (seat label, action label, one-sentence public reason, public facts list, hidden-info notice) for non-random bots, viewer-safe; a compact non-debug "Why?" disclosure near the latest bot action / active status, optionally surfaced by shared `ModeControls` when the payload exists.
- **WS5** An original `RiverLedgerIcon` SVG + catalog/theme entry in `GameCatalogIcon.tsx`.
- TS view-type updates in `apps/web/src/wasm/client.ts` for every new viewer-safe payload field; bridge updates in `crates/wasm-api/src/lib.rs`.
- Smoke, accessibility, no-leak, replay/fixture/serialization, and golden-trace test updates the view-schema additions require.

### 3.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Any correctness fix to evaluator / pot / betting / showdown logic | None warranted | Part A re-audit found the game correct against `RULES.md`; this spec changes presentation, not behavior. |
| All-in handling, side pots, no-limit / pot-limit, stacks, rake, payouts, tournaments, casino presentation | Forbidden here | Intended Gate-15 exclusions (`RL-OOS-ALLIN-001`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, `RL-OOS-NOLIMIT-001`, `RL-BET-AMB-001`, `RL-OOS-TOURNAMENT-001`, `RL-UI-NOCASINO-001`); tracked as planned **Gate 15.1**. Their absence is not a defect. |
| Re-doing RIVLEDSHO V1 work (V1 explanation fields, neutral card component, hand-ranking reference, teaching aid, base seat affordances, no-casino audit) | Already shipped | This spec is a delta; it extends and recomposes those surfaces, it does not re-author them. |
| Any change to the visibility contract or reveal timing | Forbidden | Visibility-contract change is a FOUNDATIONS §13 ADR trigger. All new fields derive only from already-authorized reveals; folded/unrevealed seats get no hand explanation or card-usage marks. |
| TypeScript-derived hand names, category labels, winner/loser, card usage, standings order, or "why" narrative | Forbidden | The defect's root risk. TS renders Rust-authored strings and sorts only by Rust-provided order fields (`RL-UI-SHOWDOWN-001`, `docs/UI-INTERACTION.md`). |
| Bot explanation exposing candidate rankings, hidden hand strength, opponent private cards, future board, deck tail, or solver claims | Forbidden | Public bot law / `RL-BOT-EXPLAIN-001`; viewer-safe public facts only. |
| Pre-showdown odds / equity meter; a fake canonical numeric hand "score" | Forbidden | Would invite hidden-information inference; Hold 'Em has no numeric score. |
| Promoting the V2 payload, card component, design tokens, or bot-explanation shape to `game-stdlib` / a shared web surface | Deferred | River-Ledger-local presentation. A shared neutral-card / outcome-renderer surface, if earned across games, routes through `docs/MECHANIC-ATLAS.md` and a separate UI-infra spec. |
| New `engine-core` types for cards / hands / explanations / bot reasons | Forbidden | `engine-core` stays noun-free (§3); these types are `games/river_ledger`-local; shared TS components may know mechanic nouns. |
| Global repaint of the app palette; redesign of `GamePicker` / `MatchSetup` / `AppShell` / `RulesPanel` | Forbidden | River-scoped tokens/classes only; the table reclaims internal width without changing shell chrome. Highest visual-regression risk lives in `styles.css`. |
| YAML / DSL for any new copy or metadata | Forbidden | Typed Rust strings or typed static content keyed to Rust IDs only; unknown fields rejected. |

---

## 4. Foundation and boundary alignment

| Authority | Constraint engaged | Spec alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §2 | Rust owns view projection, scoring, terminal detection, bot decisions; TS presents only | Every new banner, decisive reason, hand name, standings order, card-usage mark, action display row, seat-ledger label, board-slot label, and bot reason is a Rust-authored view field built in `showdown.rs` / `visibility.rs` / `ui.rs` / `bots.rs` from already-authorized state; TS chooses layout/typography/collapse and sorts only by Rust order fields. |
| `docs/FOUNDATIONS.md` §3 | `engine-core` noun-free | No `engine-core` change. V2 presentation / card-usage / bot-explanation / board-slot types stay in `games/river_ledger`; "card"/"seat"-aware React components stay in `apps/web`. |
| `docs/FOUNDATIONS.md` §5 + `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Static data is typed content/metadata keyed to Rust IDs; no behavior fields | Board-slot labels, seat-ledger labels, action helper copy, and the catalog icon's accessible title are inert content keyed to Rust IDs; no selectors/branches; unknown fields rejected. |
| `docs/FOUNDATIONS.md` §7 / §11 | Public UI play-first, cozy, original, not casino, not debug-dominated; effect-driven animation | Central board well + tabletop tokens; bot "why?" is a one-sentence public affordance, not a debug dump; board reveal and staged showdown ride existing semantic effects through the shared scheduler; reduced-motion preserves all facts; no future-card preload. |
| `docs/FOUNDATIONS.md` §8 | Public bots competent, explainable, fair, deterministic; no hidden-state use | `BotDecisionPublicExplanation` is a concise viewer-safe explanation of a deterministic policy using only the bot's authorized view; no candidate rankings, no hidden state, no solver claims. |
| `docs/FOUNDATIONS.md` §10 / `docs/IP-POLICY.md` | IP conservatism; no casino trade dress | Original `RiverLedgerIcon` (abstract cards / ledger line, no chips / felt / casino oval / branded backs); River tokens tuned away from casino green; abstract units stay abstract. |
| `docs/FOUNDATIONS.md` §11 | No hidden-info leaks; deterministic serialization; multi-seat pairwise redaction | V2 fields, card-usage marks, and bot reasons exist only for authorized reveals / public facts; folded/unrevealed seats carry none; new serialized fields are deterministic and covered by fixture/golden-trace updates; a11y labels / test IDs / effect logs carry no unrevealed facts; pairwise no-leak preserved across 3–6 seats. |
| `docs/FOUNDATIONS.md` §12 | Stop conditions: TS legality/evaluation, hidden-info leak, debug-first UI, bot hidden-state | Each is an explicit forbidden change (§11) for every ticket decomposed from this spec. |
| `docs/FOUNDATIONS.md` §13 | ADR triggers: visibility-contract change, replay/hash semantics, public bot AI class | **Not tripped.** No reveal line moves; bots keep the existing deterministic policy class (no MCTS/ML/RL); view-schema additions ride the ordinary fixture/trace migration path. |
| `docs/UI-INTERACTION.md` / `docs/WASM-CLIENT-BOUNDARY.md` | TS renders Rust/WASM output; bridge contract for new fields | New V2 / action-row / board-slot / seat-ledger / bot-explanation fields pass through the documented `crates/wasm-api` JSON seam; `client.ts` mirrors them; no TS evaluation. |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | Per-viewer projection, authorized reveal, N-seat shared labels | Ranked standings, card-usage, bot reasons, and seat labels obey per-viewer authorization; the shared seat-label path must not assume two seats; pairwise no-leak preserved 3–6 seats. |

§12 stop conditions remain clear: no `engine-core` nouns added, no static-data behavior, no TS legality/evaluation/ordering, no hidden-info leak, no debug-first UI, no bot hidden-state.

---

## 5. External UX grounding (presentation details only)

External research shapes presentation, not architecture; sources are cited in the report's **Sources** section (accessed 2026-06-16):

- **Decisive answer first, contrastive, progressively disclosed.** Lead with who-won + why-over-the-closest-challenger; collapse the rest of the field and reserve raw vectors/rule IDs for a details tier. (NN/g Recognition & Recall [EXT-03]; NN/g Progressive Disclosure [EXT-04]; Miller, *Explanation in AI* [EXT-05].)
- **Spatial table orientation.** Players around the edge, community/ledger info centered, action near the active player — without casino trade dress. (PokerTH layout [EXT-10]; Board Game Arena framing [EXT-11].)
- **Colour is never the sole channel; contrast + text alternatives.** Suit via glyph + word; used cards marked by shape/ring/text; WCAG AA contrast; group accessible labels. (WCAG 2.2 SC 1.4.1 / 1.4.3 [EXT-06][EXT-07].)
- **Reduced-motion safety; status announcements.** A way to prevent non-essential animation; `role=status` for the terminal result region. (WCAG 2.1 SC 2.3.3 [EXT-08]; ARIA22 [EXT-09].)

These reinforce decisions D1–D9; they do not reopen any boundary.

---

## 6. Committed design decisions

### D1 — Seat labels reuse the existing `seat_labels` projection; player-facing strings are born clean
River Ledger's catalog already projects `seat_labels` as `{seat:"seat_N", label:"Seat N"}` (`wasm-api/src/lib.rs:618` via `catalog_seat_labels_json`), and `SeatDisplayLabel` already exists in TS as `{seat, label}` (`client.ts:105`). **No new seat-label type or `public_label`/`short_label` field is invented** — that would conflict with the existing `{seat, label}` shape. The genuinely-missing Rust work is a small `ids.rs`/`ui.rs` helper returning the same "Seat N" form, used to route every terminal/showdown/rationale/seat-list string in `showdown.rs` and `visibility.rs`, never `RiverLedgerSeat::as_str()`. Raw `seat_N` remains an internal ID and stable serialization/hash key, but no visible text or accessibility label consumes it directly. The TS fallback normalization in `outcomeExplanationTemplates.ts` may stay as legacy safety for other games but is not the River Ledger fix.

### D2 — Per-action presentation rows are Rust-authored; TS renders only supplied rows
`betting.rs`/`actions.rs`/`ui.rs` supply a per-action presentation object (segment, label, helper text, `display_rows[]` with `{label,value,tone}`, accessibility label). Which rows are relevant is decided in Rust (Fold/Check carry no `Call price`/`Cap left`). TS renders exactly the supplied rows — it no longer reads raw metadata counters directly. This stays compatible with `RL-UI-ACTIONS-001`: Rust still owns legal actions and metadata; presentation becomes semantic instead of dumping every public counter.

### D3 — Board slots are Rust-authored placeholders; no future-card identity
A Rust `RiverLedgerBoardSlotView` carries `slot`, `reveal_state`, `street_label`, `visual_placeholder_label` ("Pending"), `accessibility_label` ("Unrevealed turn card. It is not available yet."), and a nullable `card`. The card identity stays `null` until Rust reveals it. TS renders a quiet placeholder with a CSS-safe single label.

### D4 — Seat-ledger fields are player-facing and labeled; the redundant bar is removed
A Rust `RiverLedgerSeatLedgerDisplay` carries `seat_label`, `role_badges`, `status_label`, `round_contribution {label:"This round",value}`, `hand_contribution {label:"Hand total",value}`, and `hole_card_summary {label:"Hole cards",value:"2 hidden"/"2 revealed",accessibility_label}`. The separate unlabeled contribution bar is removed; if any visual ledger remains it is a labeled compact strip whose accessible name carries the same Rust value and implies no chips/cash.

### D5 — `Seat N` in the status line by extending the existing `event_frontier` label path
`activeActorLabel` (`ModeControls.tsx:181-186`) already consumes `view.ui.seat_labels` — but only for `event_frontier`; River Ledger falls through to the `Player N` fallback (`:189`). Two steps, both following the `event_frontier` precedent (Q1=(a)): (1) mirror `event_frontier`'s per-match `ui.seat_labels` view projection for River Ledger in `wasm-api` (`event_frontier_seat_display_label_json`, `wasm-api/src/lib.rs:8157-8169` — River Ledger projects `seat_labels` only on the catalog today, not its per-match view); (2) extend the `activeActorLabel` branch (or generalize its gate to "any view carrying `ui.seat_labels`") so River Ledger status copy reads `Seat N to act`. No new label type and no wider `activeActorLabel` signature change. `event_frontier` and games without `ui.seat_labels` are unaffected (exit criterion #5b).

### D6 — `RiverLedgerShowdownPresentationV2` is Rust-authored, additive, and reveal-scoped
A new additive Rust payload (built in `showdown.rs`/`visibility.rs` alongside the V1 fields, not replacing them) carries: `result_banner {headline, subheadline, accessibility_label}`; `decisive_reason {short_text, contrast_seat, contrast_seat_label, rule_refs}`; `board_cards[] {slot, card, public_label, used_by_selected}`; ranked `standings[]` (`seat`, `seat_label`, `result_label`, `allocation_label`, `hand_name`, `comparison_note_short`, `rank_ladder_label`, `hole_card_usage[]`, `board_card_usage[]`, `best_five`, `best_five_accessibility_label`, `detail_rows[]`, `default_expanded`); and `folded_rows[]` (`seat`, `seat_label`, `result_label`, `redaction_label`). `standings` arrives **already ranked**; TS renders that order and never derives it. `best_five`/usage marks come from `evaluator.rs` output. **Leak stance:** standings/card-usage exist only for showdown-eligible revealed seats; folded seats appear only as `folded_rows` with a redaction label and no hand strength. Raw `category_key`/`tie_break_vector`/rule IDs are retained for the details tier (additive, never removed — preserves trace/replay consumers).

### D7 — The V2 renderer is a River-Ledger branch in the shared outcome panel
`OutcomeExplanationPanel` stays shared. Add a `RiverLedgerShowdownV2` subrenderer behind a discriminated optional payload (like the existing River Ledger branch). It renders: banner → decisive reason + closest challenger → board once with usage marks → winner's five → compact ranked standings (winner + challenger visible, remaining seats collapsed by default) → detail expanders. The generic outcome surface for all other 14 games is unchanged; a cross-game non-regression check (§9) guards it. `RiverLedgerCard` stays the local card renderer; its import into the shared panel remains a recorded River-specific exception (long-term: invert via injected game-specific card renderer — deferred, §3.3).

### D8 — Table recomposition, River-scoped tokens, scheduler-routed motion
Recompose `RiverLedgerBoard` into a central board well + compact multi-seat rails (responsive two-rail/shallow-horseshoe for 3–6 seats) + an action/status band; seat order, active seat, roles, contributions, live/folded status, and labels all come from Rust/static metadata (`RL-UI-SEATS-001`). Add River-scoped `--rl-*` tokens/classes in `styles.css` (table surface, ledger marks, card state, showdown hierarchy) without overwriting global tokens. Route board reveal and a staged showdown burst (banner → board usage highlights → standings settle) through the shared scheduler with a reduced-motion path and settle assertions; no ad-hoc timers, no hidden future DOM.

### D9 — Bot "why?" is a Rust-authored, viewer-safe public affordance
A Rust `BotDecisionPublicExplanation` (`seat`, `seat_label`, `action_label`, `short_reason`, `public_facts[] {label,value}`, `hidden_information_notice`) for non-random bots, built in `bots.rs`/`visibility.rs` from the bot's authorized view and public state only. A compact non-debug "Why?" disclosure renders it near the latest bot action / active status, optionally surfaced by shared `ModeControls` when the payload exists. No candidate rankings, hidden hand strength, opponent private cards, future board, deck tail, or solver claims; not dumped into the effect log. Random/no-explanation bots show no affordance.

### D10 — Original catalog icon and River Ledger identity
Add an original `RiverLedgerIcon` SVG to `GameCatalogIcon.tsx`: abstract cards forming a river bend / ledger fan, a thin brass ledger line with tally marks, monochrome-friendly shape differences, accessible title from the catalog entry. No chips, cash, green felt, casino oval, branded backs, or poker-room imagery. Touches the shared catalog only through the icon map (River-local entry); no picker/setup redesign.

---

## 7. Deliverables

```text
games/river_ledger/src/ids.rs             ("Seat N" label helper for showdown/visibility strings; reuses existing seat_labels form, no new type)
games/river_ledger/src/showdown.rs        (clean seat strings via display labels; additive V2 presentation builder)
games/river_ledger/src/visibility.rs      (reveal-scoped projection of V2 + card-usage + bot-explanation fields)
games/river_ledger/src/state.rs           (additive V2 / seat-ledger / board-slot view fields on projections)
games/river_ledger/src/betting.rs         (per-action presentation rows; cap-remaining already projected)
games/river_ledger/src/actions.rs         (action presentation row authorship)
games/river_ledger/src/bots.rs            (BotDecisionPublicExplanation for non-random bots)
games/river_ledger/src/ui.rs              (board-slot labels; seat-ledger labels; action helper copy; bot-why copy)
crates/wasm-api/src/lib.rs                (augmented terminal + in-play JSON for all new viewer-safe fields; River Ledger per-match ui.seat_labels view projection, mirroring event_frontier)
apps/web/src/wasm/client.ts               (TS types mirroring the new Rust shapes)
apps/web/src/components/OutcomeExplanationPanel.tsx   (River-Ledger V2 showdown renderer branch; no other-game regression)
apps/web/src/components/outcomeExplanationTemplates.ts (River terminal strings consume clean labels; fallback kept for legacy)
apps/web/src/components/RiverLedgerBoard.tsx          (table recomposition; board slots; seat ledger; action rows; bot why; card-usage marks)
apps/web/src/components/ModeControls.tsx              (extend the event_frontier-gated activeActorLabel seat_labels branch to river_ledger; optional last_bot_explanation mount)
apps/web/src/components/SeatFrame.tsx                 (already consumes game.seat_labels:75 — verify multi-seat coverage; likely no change)
apps/web/src/components/GameCatalogIcon.tsx           (original river_ledger SVG + theme entry)
apps/web/src/styles.css                              (River-scoped --rl-* tokens/classes; no global token overwrite)
apps/web/src/animation/{scheduler,registry,bursts,presenters,settleAssertion}.ts (River reveal/showdown pacing; reduced-motion path)
scripts/check-presentation-copy.mjs       (extend seat_N guard coverage as needed)
scripts/check-outcome-explanations.mjs    (fail \bseat_\d+\b in terminal outcome / a11y strings)
games/river_ledger/tests/{rules,property,visibility,serialization,replay,bots}.rs (V2 + card-usage + bot-explanation + no-leak coverage)
games/river_ledger/tests/golden_traces/*.trace.json   (view-JSON updates where new fields appear)
apps/web/e2e/river-ledger.smoke.mjs       (ranked-standings / board-usage / action-row / no raw seat_N runtime sweep)
apps/web/e2e/a11y-noleak.smoke.mjs        (live-region + card-usage a11y; no-leak DOM/storage/log)
apps/web/e2e/outcome-explanation.smoke.mjs (V2 branch present; generic branch unchanged)
games/river_ledger/docs/UI.md             (reconcile outcome/contract/seat/action/bot rows with V2)
games/river_ledger/docs/RULE-COVERAGE.md  (UI-row updates; any new RL-UI-* sub-rule coverage)
apps/web/README.md                        (catalog/Shell Surface reconciliation if a surface name changes)
specs/README.md                           (index row maintained: Planned → Done at closeout)
```

Per-game Rust changes ride the ordinary verification set: unit/rule tests, `fixture-check`, `replay-check`, `rule-coverage`, serialization tests, golden-trace updates where view JSON changes, `simulate` smoke, and `boundary-check.sh`.

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

Dependency order; each item is one reviewable diff. Decomposed into `tickets/` via `/reassess-spec` then `/spec-to-tickets` after acceptance.

| # | Item | Workstream | Depends on |
|---|---|---|---|
| WB1 | "Seat N" label helper in `ids.rs`/`ui.rs` (reusing the existing `seat_labels` form, **not** a new type); route `showdown.rs`/`visibility.rs` strings through it; unit tests prove no `seat_N` in any authored visible/a11y string; serialization/golden-trace updates | WS1 (#1) | — |
| WB2 | Strengthen guards: extend `check-outcome-explanations.mjs` (+ `check-presentation-copy.mjs` as needed) and the e2e DOM/a11y sweep to fail `\bseat_\d+\b` in **runtime** visible text and accessibility labels for River Ledger public surfaces | WS1 (#1) | WB1 |
| WB3 | Rust per-action presentation rows (D2): helper text + relevant `display_rows` per segment; bridge + `client.ts` types; `RiverLedgerBoard` renders only supplied rows (Fold/Check lose `Call price`/`Cap left`); ui smoke | WS1 (#2) | — |
| WB4 | Rust board-slot view (D3): street-specific placeholder + accessibility labels; CSS-safe placeholder layout; no future-card identity; ui smoke | WS1 (#3) | — |
| WB5 | Rust seat-ledger display fields (D4): `This round`/`Hand total`/`Hole cards`; remove redundant bar (or relabel compact, value-equal, no chip framing); ui smoke | WS1 (#4) | — |
| WB6 | `Seat N` status (D5): mirror `event_frontier`'s per-match `ui.seat_labels` view projection for river_ledger in `wasm-api`, then extend the `activeActorLabel` event_frontier branch (`ModeControls.tsx:181-189`) to river_ledger; **cross-game status non-regression** (other 13 games' status copy unchanged) + multi-seat label tests; status-copy smoke | WS1 (#5, M1) | — |
| WB7 | `RiverLedgerShowdownPresentationV2` Rust payload (D6): banner, decisive contrast, ranked standings, folded rows, hole/board card-usage marks, a11y labels; additive serialization + golden-trace updates; unit tests on the worked example, split, foldout | WS2 (#6) | WB1 |
| WB8 | Reveal-scoped projection of V2 + card-usage through `visibility.rs`; no-leak tests proving folded/unrevealed seats carry no standings/usage; viewer-hash + replay-export coverage; `wasm-api` JSON + `client.ts` types; bridge no-leak test | WS2 (#6) | WB7 |
| WB9 | River-Ledger V2 showdown renderer branch in `OutcomeExplanationPanel` (D7): banner, board-once + usage marks, winner/closest-challenger, compact collapsible standings, detail expanders; card-usage shape/text marks (#8); cross-game outcome-panel non-regression smoke | WS2 (#7,#8) | WB8 |
| WB10 | Terminal result live-region (#9): `role=status`/atomic announcement for the showdown banner + expanded details; reduced-motion preserves all facts; a11y smoke | WS2 (#9) | WB9 |
| WB11 | Table recomposition (D8, #10): central board well + compact multi-seat rails + action/status band; Rust/static view data only; ui smoke across 3–6 seats | WS3 (#10) | WB3,WB4,WB5,WB9 |
| WB12 | River-scoped `--rl-*` tokens/classes (#11): tabletop surface, ledger marks, card state, showdown hierarchy; contrast checks; no global token regression; snapshot | WS3 (#11) | WB11 |
| WB13 | Scheduler-routed reveal/showdown pacing (#12): staged showdown burst + board reveal through shared scheduler; reduced-motion path; settle assertions; no ad-hoc timers/hidden future DOM | WS3 (#12) | WB11,WB12 |
| WB14 | Rust `BotDecisionPublicExplanation` (D9, #13): viewer-safe public facts for non-random bots; projection + bridge + `client.ts` types; no-leak tests (no candidate rankings/hidden state/opponent cards) | WS4 (#13) | WB8 |
| WB15 | Compact "Why?" disclosure (#14): non-debug surface near bot action / active status, optional shared `ModeControls` mount when payload exists; no effect-log dump; a11y labels clean; ui smoke | WS4 (#14) | WB14 |
| WB16 | Original `RiverLedgerIcon` SVG + catalog/theme entry (D10, #15): no casino iconography; accessible label from catalog; rendered-at-display-size legibility check; `check-catalog-docs.mjs` green | WS5 (#15) | — |
| WB17 | Closeout: reconcile `UI.md` + `RULE-COVERAGE.md` (any new `RL-UI-*` sub-rule coverage); `apps/web/README.md` catalog/Shell reconciliation if surface names change; doc-link/catalog checks; flip this spec's index row to `Done` with evidence | all | all |

---

## 9. Exit criteria

1. No River Ledger public surface (visible text or accessibility label) contains a raw `seat_N` token at runtime; the seat-id strengthened guard (copy audit + e2e DOM/a11y sweep) fails on a deliberately re-introduced `seat_N` and is green on the clean tree. (#1)
2. Fold and Check action controls show no `Call price` / `Cap left` rows; every rendered action row comes from a Rust-supplied `display_rows` set; TS computes no metadata relevance. (#2)
3. Unrevealed board slots render a single CSS-safe placeholder with a Rust-authored street-specific label and accessibility text; no future-card identity reaches the DOM / a11y / test IDs. (#3)
4. Seat panels read player-facing `This round` / `Hand total` / `Hole cards: N hidden/revealed` with role/status badges; the redundant unlabeled contribution bar is gone (or relabeled, value-equal, no chip/money framing). (#4)
5. All River Ledger status copy reads `Seat N` (e.g. `Seat 1 to act`) by extending the existing `event_frontier`-gated `seat_labels` consumption in `activeActorLabel`; River Ledger no longer falls through to the `Player N` fallback. (#5)
5b. The shared `activeActorLabel` change causes **no status-copy regression** for the other 13 catalog games: `event_frontier` keeps its labels, and games whose view carries no `ui.seat_labels` keep their current status label; a `smoke:ui` status-line assertion guards this. (#5, M1)
6. After a showdown, the V2 panel answers, in order, who won (banner) / why over the **named closest challenger** / which board+hole cards were used (marked by text/shape, not colour) / a compact ranked field (winner + challenger visible, rest collapsed) — all from Rust-authored fields; TS sorts only by Rust order fields and computes no hand name, usage, order, or "why". (#6,#7,#8)
7. The terminal result banner is announced through a `role=status`/atomic (or repo-equivalent) live region; expanders are keyboard-accessible; reduced motion preserves every reveal fact. (#9)
8. The table renders as a central board well + compact multi-seat rails + action/status band, responsive across 3–6 seats, using only Rust/static view data; River-scoped `--rl-*` tokens add the tabletop identity with no global token regression and WCAG-AA contrast. (#10,#11)
9. Board reveal and the staged showdown pacing run through the shared scheduler with a reduced-motion path and passing settle assertions; no ad-hoc timers; no hidden future board in DOM / a11y / test IDs / animation ghosts. (#12)
10. Non-random bots expose a one-sentence viewer-safe `BotDecisionPublicExplanation` reachable via a compact non-debug "Why?" disclosure; it shows only public facts (no candidate rankings, hidden strength, opponent cards, future board, deck tail, solver claims) and is not dumped into the effect log; random bots show no affordance. (#13,#14)
11. The catalog shows an original `river_ledger` SVG icon (no casino iconography, accessible label), legible at display size; `check-catalog-docs.mjs` stays green. (#15)
12. All view-schema additions pass `cargo test --workspace`, `fixture-check`, `replay-check`, `rule-coverage`, serialization tests, and updated golden traces; `simulate` smoke behavior unchanged; `boundary-check.sh` clean.
13. No-leak evidence: observer / wrong-seat DOM, `data-testid`, storage, console, effect log, and replay export contain no unrevealed hole cards, future board, deck tail, folded-seat hand strength, bot-private reasons, or raw `seat_N`; folded seats carry only `folded_rows` redaction; card-usage marks exist only for authorized reveals.
14. No regression to the other 14 catalog games' outcome rendering: `smoke:ui` / `outcome-explanation.smoke` stay green, and the V2 River-Ledger layout activates only when the V2 payload is present.

## 10. Acceptance evidence

- Rust: unit tests for seat display labels (no `seat_N` in authored strings), per-action presentation rows, board-slot labels, seat-ledger fields, the V2 payload (worked example, split, foldout, high-card, every category), card-usage marks, and `BotDecisionPublicExplanation`; additive serialization/golden-trace fixtures; reveal-scoped + bot-explanation no-leak tests; `cargo fmt/clippy/build/test` gate 0 clean.
- Tools: `fixture-check`, `replay-check`, `rule-coverage` for `river_ledger`; `boundary-check.sh` clean.
- Web: `smoke:wasm`, `smoke:ui`, `smoke:effects`, `smoke:e2e` green; new cases for the V2 ranked-standings panel, board-usage marks, live region, action display rows, seat-ledger labels, `Seat N` status, table recomposition, bot "why?" disclosure, and the catalog icon; cross-game status-line non-regression (other 13 games' actor labels unchanged); runtime no-leak + no-`seat_N` DOM/storage/log sweep.
- Screenshots: before/after of the showdown V2 panel and the recomposed table; the catalog icon rendered at display size with its accessible label; the bot "why?" disclosure.

## 11. Forbidden changes

- No change to evaluator, pot, betting, showdown, visibility, or bot-decision *behavior* — presentation and additive view fields only.
- No visibility-contract change, no new reveal, no reveal-timing change (FOUNDATIONS §13 ADR trigger); folded/unrevealed seats gain no explanation, standings entry, or card-usage marks.
- No public bot AI-class change (no MCTS / ISMCTS / Monte Carlo / ML / RL); bot explanations expose no candidate rankings or hidden state.
- No `engine-core` edits; no `game-stdlib` additions or helper promotion outside the atlas process.
- No TypeScript-computed legality, evaluation, winner, split, hand name, category label, card usage, standings order, action-row relevance, or "why" narrative; no rank/suit/seat text derived from raw IDs.
- No all-in / side-pot / no-limit / pot-limit / stacks / rake / payout / tournament features; their absence is not a defect.
- No casino trade dress, real-money framing, copied card/table art, or global palette repaint; abstract units stay abstract; River tokens stay River-scoped.
- No pre-showdown / hidden-derived strength meter; no fake canonical numeric hand score.
- No YAML, no DSL, no behavior-looking fields in any new static content; unknown fields rejected.
- No replay/hash semantic change; trace/fixture updates only through the ordinary migration path.
- No weakening or deletion of existing tests (AGENT-DISCIPLINE §4); fixes ship with strengthened coverage.

## 12. Documentation updates required

- `games/river_ledger/docs/UI.md`: outcome/contract/seat/action/board/bot rows reconciled with the V2 panel, the action presentation rows, the seat-ledger labels, the board-slot view, and the bot-explanation surface.
- `games/river_ledger/docs/RULE-COVERAGE.md`: UI rows updated for the new surfaces; whether the new fields warrant `RL-UI-*` sub-rule IDs in `games/river_ledger/docs/RULES.md` is decided at decomposition (assumption A4); if so, the IDs and coverage rows land with WB7/WB17.
- `apps/web/README.md`: catalog/Shell Surface reconciliation only if a surface name changes (`check-catalog-docs.mjs` must stay green).
- `specs/README.md`: index row for this spec (added at authoring as `Planned`; flipped to `Done` at WB17 with evidence).

## 13. Sequencing

- **Predecessor:** the non-gate `river-ledger-showdown-legibility-and-table-presentation` spec (RIVLEDSHO) — `Done` (2026-06-15); its shipped V1 showdown UI, neutral card component, and table affordances are the surfaces this spec extends. The motivating input is `reports/river-ledger-showcase-ux-report.md`, a delta audit on that state.
- **Position:** Non-gate public-polish spec, same admission class as the archived River Ledger / UI-infra polish specs: complete it as polish on the shipped game; it does **not** block and is not blocked by the next mechanic-ladder unit. No `game-stdlib` change, so it introduces no promotion debt and is not gated by the open-debt interlock.
- **Relation to Gate 15.1:** independent. Gate 15.1 (all-in / side pots) remains the next *gate* unit; this presentation work neither depends on nor blocks it. If Gate 15.1 later changes the terminal outcome shape, the additive V2 fields extend rather than conflict.
- **Successor:** none required; a future shared neutral-card / V2-outcome-renderer / bot-explanation surface, if earned across games, routes through `docs/MECHANIC-ATLAS.md` and a separate UI-infra spec.

## 14. Assumptions (one-line-correctable)

1. **(A1) Spec packaging** — assuming one combined non-gate spec with five workstreams (matching your singular "create a spec" directive and the RIVLEDSHO precedent), not split per slice; flip to per-slice specs if you prefer smaller units.
2. **(A2) Index placement** — assuming a new non-gate row in the active-epoch tracker adjacent to the RIVLEDSHO row (admission class "complete as polish; non-blocking"), not a new gate.
3. **(A3) Field carriers** — assuming additive V2 / seat-ledger / board-slot / action-row / bot-explanation view fields on the existing projections (D2/D3/D4/D6/D9) rather than reshaping V1 structures; flip to a reshape only if additive proves awkward at decomposition.
4. **(A4) New rule IDs** — assuming the new fields satisfy existing `RL-UI-SHOWDOWN-001`/`RL-UI-ACTIONS-001`/`RL-UI-SEATS-001`/`RL-BOT-EXPLAIN-001` and may add `RL-UI-*` sub-rules at decomposition; any RULES.md ID additions are a WB7/WB17 doc deliverable, not a behavior change.
5. **(A5) Bot-why scope** — assuming `BotDecisionPublicExplanation` covers non-random (Level 1+) bots only, public facts only; drop WB14/WB15 entirely if unwanted — only WS4 depends on them.
6. **(A6) Contribution bar** — assuming the redundant seat contribution bar is removed (D4); keep it as a labeled, value-equal compact strip instead if you prefer a visual ledger.
7. **(A7) Local scope** — assuming River-Ledger-local presentation throughout (V2 payload, card-usage, tokens, bot explanation, card renderer); promotion to shared surfaces / `game-stdlib` is explicitly deferred (§3.3).
8. **(A8) Research basis** — external UX grounding reuses the report's cited sources (NN/g, WCAG 2.2, WAI, Miller; PokerTH/BGA layout references); no fresh research pass was run. Commission `research-brief`/`deep-research` if deeper grounding is wanted.
9. **(A9) Seat-label infra reuse (verified)** — the `seat_labels` projection, the `SeatDisplayLabel {seat,label}` TS type, river_ledger's catalog "Seat N" labels, and the `event_frontier`-gated `activeActorLabel` consume-pattern already exist (`client.ts:105`, `wasm-api/src/lib.rs:449-457,618,8157-8169`, `ModeControls.tsx:181-189`, `SeatFrame.tsx:75`); WB1/WB6 extend them rather than inventing a new label type. If decomposition finds river_ledger's per-match view cannot cheaply carry `ui.seat_labels`, fall back to passing the catalog `game.seat_labels` into `activeActorLabel` (the wider-signature option weighed and rejected at reassessment).

---

## 15. Triage of report backlog → workstreams

Source: `reports/river-ledger-showcase-ux-report.md` Part D (ranked backlog #1–#15) + Part A. Verified against the working tree at `f1d6d39`.

| Report item | Verdict | Disposition |
|---|---|---|
| Part A — correctness re-audit (no defect) | informational | No implementation; recorded as §3.3 non-goal. |
| Absent all-in / side pots / no-limit | reject-as-bug | Intended Gate-15 exclusion; tracked as planned Gate 15.1. |
| #1 — fix raw `seat_N` leak at Rust authoring layer + strengthen audits | accept-with-modification | WS1 — WB1/WB2 (D1). Static-copy guard already at `check-presentation-copy.mjs:24`; add Rust fix + runtime/a11y guard. |
| #2 — Rust-authored action presentation rows; hide call/cap on Fold/Check | accept | WS1 — WB3 (D2). Confirmed `RiverLedgerBoard.tsx:321-323`. |
| #3 — Rust-authored board-slot placeholder labels; CSS-safe layout | accept-with-modification | WS1 — WB4 (D3). Raw overflow already mitigated to "Pending"; add Rust street/a11y labels. |
| #4 — player-facing seat-ledger labels; remove redundant bar | accept | WS1 — WB5 (D4). Confirmed `RiverLedgerBoard.tsx:217-219,255-264`. |
| #5 — normalize `Player N` → `Seat N` via catalog/Rust labels | accept-with-modification | WS1 — WB6 (D5). _Mod_: `seat_labels`/`SeatDisplayLabel` infra already exists (`client.ts:105`, `wasm-api:618`); extend the `event_frontier`-gated `activeActorLabel` branch rather than build a new label path. Confirmed `ModeControls.tsx:181-189`. |
| #6 — `RiverLedgerShowdownPresentationV2` payload | accept | WS2 — WB7/WB8 (D6) |
| #7 — V2 showdown renderer branch | accept | WS2 — WB9 (D7) |
| #8 — card-usage visualization | accept | WS2 — WB9 (D6/D7) |
| #9 — terminal result live-region / a11y | accept | WS2 — WB10 (D7) |
| #10 — table recomposition (central board well + rails) | accept | WS3 — WB11 (D8) |
| #11 — River-scoped design tokens | accept | WS3 — WB12 (D8) |
| #12 — scheduler-routed reveal/showdown pacing | accept | WS3 — WB13 (D8) |
| #13 — `BotDecisionPublicExplanation` | accept | WS4 — WB14 (D9) |
| #14 — compact "Why?" disclosure | accept | WS4 — WB15 (D9) |
| #15 — original `river_ledger` catalog icon | accept | WS5 — WB16 (D10). Confirmed missing from `GameCatalogIcon.tsx`. |
