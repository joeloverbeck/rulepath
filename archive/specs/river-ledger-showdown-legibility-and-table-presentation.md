# river-ledger-showdown-legibility-and-table-presentation — Rust-authored showdown explanation + table-readability overhaul

- **Filename:** `specs/river-ledger-showdown-legibility-and-table-presentation.md`
- **Spec ID:** `river-ledger-showdown-legibility-and-table-presentation`
- **Target type:** New spec
- **Roadmap stage:** Non-gate public-polish spec for the shipped Gate 15 game `river_ledger` — not a mechanic-ladder gate.
- **Roadmap build gate:** None. This is a non-gate post-Gate-15 presentation spec, the same admission class as the archived `victory-explanation-shared-surface` / `card-and-action-presentation-shared-surfaces` UI-infra specs, motivated by the `reports/river-ledger-correctness-and-presentation-report.md` audit + UX overhaul.
- **Status:** Done
- **Date:** 2026-06-15
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → this spec.
- **Subordination:** This spec is subordinate to the foundation docs, area docs, accepted ADRs, the official-game contract, and `games/river_ledger/docs/RULES.md` (the `RL-*` correctness oracle). It plans presentation work on a correct game; it does not redefine any rule, visibility contract, or foundation principle.

---

## 1. Objective

Make River Ledger's post-showdown surface answer **who won and why** at a glance, and make the whole table readable, **without moving any evaluation, winner, split, or hand-strength logic out of Rust**.

The motivating finding is the external audit `reports/river-ledger-correctness-and-presentation-report.md` (commit `351dc1e`). Its verdict: the game is **substantially correct** against its own `RL-*` rule contract — no defect in setup, blinds, fixed-limit betting, round closure, single-pot accounting, evaluator category ordering, tie-break vectors, ace-low straight, best-five-of-seven, showdown winner/split, or no-leak projection. The only high-severity issue is a **presentation/usability defect**: the showdown panel exposes correct machine facts (`category`, `tie_break_vector`, `best_five`) but makes the decisive comparison illegible to ordinary players.

In the audit's worked example (board `4C 3D QH 6H 8H`), the engine correctly encodes **Pair of Queens beats Pair of Eights** as `[12,10,8,6]` vs `[8,12,10,6]`; the UI shows `one_pair` and raw integers and never states the decisive sentence. The owner specifically struggles to read Texas Hold 'Em hand strength, so "what beats what, and why" being intuitive is the heart of this work.

This spec packages the audit's actionable backlog (Part C, R1–R12, plus the documentation-drift item) into **one non-gate spec with two workstreams**:

- **Workstream A — Outcome comprehension (the named pain point).** Add Rust-authored showdown explanation fields, redesign the terminal showdown panel around the decisive sentence + best-five visual cards, move raw vectors/rule IDs to a details disclosure, and lock the fix with explanation-string and no-leak tests plus an e2e worked-example check.
- **Workstream B — Table readability.** A neutral River Ledger card component (rank + suit glyph + suit word + accessible label), an always-available hand-ranking reference, action-panel copy from Rust metadata, seat/turn-flow affordances, an optional leak-safe teaching-strength aid, and a public-copy casino-vocabulary audit.

Plus the **`RULE-COVERAGE.md` UI-row refresh** (B7) that the audit flagged as documentation drift.

The correctness half of the report (Part A) requires **no implementation**: there is no gameplay defect to fix, and the deliberately-absent casino features (all-in, side pots, no-limit/pot-limit) are confirmed intended Gate-15 exclusions, already tracked as the planned **Gate 15.1** unit — they are explicitly out of scope here (§3.3).

---

## 2. Current state and motivating evidence

Verified against the working tree at `351dc1e` (current `main` HEAD) and the audit's live capture:

| Surface | Current state | Evidence |
|---|---|---|
| Showdown reveal data | `ShowdownReveal` carries `category` as a raw enum string (`"one_pair"`), `tie_break_vector: Vec<u8>`, `best_five`, `hole_cards` — correct machine facts, no human-readable layer | `games/river_ledger/src/showdown.rs:142-152`, `state.rs` `ShowdownReveal` |
| Per-seat showdown explanation | `ShowdownSeatExplanation.summary` is a machine string: `"{seat} reached showdown with {category}; tie_break={:?}; allocated=…; total_contribution=…"` — no hand name, no decisive comparison | `games/river_ledger/src/showdown.rs:113-139` |
| Decisive "why X beat Y" | **Does not exist** anywhere in Rust or TS; the winner-vs-challenger comparison is never authored | `showdown.rs` (no comparison string emitted) |
| UI outcome contract | `UI.md` already reserves a `comparison vector/reason` row marked "Rust-authored; TypeScript only renders it", and template keys `river_ledger.showdown_best_hand_win` / `_split_pot` / `last_live_fold_win` — the slot exists; only jargon flows through it | `games/river_ledger/docs/UI.md:152-165, 137-145` |
| Terminal copy | Public string `"One showdown hand has the strongest Rust-evaluated five-card result."` leaks engine jargon into a player surface | audit §B.2; `apps/web/src/components/outcomeExplanationTemplates.ts` |
| Card rendering | Board / hole / best-five render as plain text blocks with raw suit/rank text — no suit glyphs, no red/black, no card-like visuals, no group a11y label | audit §1, §B.6; `apps/web/src/components/RiverLedgerBoard.tsx` (card markup inline, no separate component) |
| Hand-ranking reference | None — the category ladder is nowhere shown | audit §B.3 |
| `RULE-COVERAGE.md` UI rows | `RL-UI-PRESENT/SEATS/ACTIONS/PREVIEW/LEDGER/SHOWDOWN/NOCASINO/NOLEAK-001` and `RL-OOS-BROWSER-001` are all still `intentionally-deferred` ("Browser presentation pending") even though RIVLEDOUT-001/002 shipped the outcome UI | `games/river_ledger/docs/RULE-COVERAGE.md:92-99, 114` |
| Correctness core | No defect found by the audit; `showdown.rs:142-151` tie-break encoding verified (`12>8` ⇒ Queens beat Eights) | audit Part A; `evaluator.rs`, golden traces |

The evaluator, pot, and visibility cores are **not** targets of this spec beyond additive explanation-field projection. This spec frames the showdown fix as a **delta on already-wired plumbing**: `category` / `tie_break_vector` / `best_five` already reach the client; the gap is the missing human-readable layer and the missing visual/educational presentation.

---

## 3. Goal, scope, and non-goals

### 3.1 Goal

A player who cannot fluently read Texas Hold 'Em hand strength can, after a showdown, immediately see **who won, with what hand, and why it beat the next-best hand** — with every word of that explanation authored in Rust and projected through `visibility.rs` → `crates/wasm-api` → the client, never synthesized in TypeScript by re-interpreting the tie-break vector. The rest of the table (cards, seats, actions, turn flow) reads as a cozy, original board-game surface, not a debugger or a casino.

### 3.2 In scope

- **WS-A** new Rust-authored showdown explanation fields (headline, decisive comparison, comparison basis, per-seat hand name, rank explanation, per-seat comparison note, result label, accessibility labels), projected viewer-safe through `visibility.rs` and the WASM bridge.
- **WS-A** terminal showdown panel redesign (decisive sentence first, best-five visual cards, winner + closest-challenger contrast, raw vectors/rule IDs behind a details disclosure).
- **WS-A** explanation-string unit tests, no-leak coverage for the new fields, and an e2e worked-example check ("Queens beat Eights").
- **WS-B** a neutral local River Ledger card component (rank + suit glyph + suit word, colorblind-safe, group accessibility labels) used by board, hole cards, and best-five.
- **WS-B** an always-available hand-ranking reference (Rust/static-supplied labels), default-visible after showdown, collapsible during play.
- **WS-B** action-panel copy from Rust legal-action metadata (call price, adds-to-ledger, cap remaining, viewer-safe unavailable reasons).
- **WS-B** seat / turn-flow affordances (button / blind / active state by text + icon, street strip, reduced-motion-aware board reveal).
- **WS-B** an optional, clearly-labeled, leak-safe terminal-only teaching-strength aid (category-ladder position form only).
- **WS-B** a public-copy casino-vocabulary audit (`RL-UI-NOCASINO-001`).
- **B7** `RULE-COVERAGE.md` UI-row refresh and any `UI.md` reconciliation the redesign requires.
- TS view-type updates in `apps/web/src/wasm/client.ts` for the new viewer-safe payload fields.
- Smoke, accessibility, no-leak, replay/fixture/serialization, and golden-trace test updates the view-schema additions require.

### 3.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Any correctness fix to evaluator / pot / betting / showdown logic | None warranted | Part A found the game correct against `RULES.md`; this spec changes presentation, not behavior. |
| All-in handling, side pots, no-limit / pot-limit, stacks, rake, payouts, tournaments | Forbidden here | Intended Gate-15 exclusions (`RL-OOS-ALLIN-001`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, `RL-OOS-NOLIMIT-001`, `RL-BET-AMB-001`); tracked as planned **Gate 15.1**. Their absence is not a defect. |
| Pre-showdown odds / equity meter | Forbidden | Would invite hidden-information inference; the only strength aid here is terminal-only and derived solely from authorized revealed hands. |
| Any change to the visibility contract or reveal timing | Forbidden | Visibility-contract changes are a FOUNDATIONS §13 ADR trigger. New fields derive **only** from already-authorized showdown reveals; folded/unrevealed seats get no explanation. |
| TypeScript-derived hand names, category labels, winner/loser, or "why" narrative | Forbidden going forward | That is the defect's root risk. TS renders Rust-authored strings only (`RL-UI-SHOWDOWN-001`, `docs/UI-INTERACTION.md`). |
| Promoting the card component or explanation shape to `game-stdlib` / a shared web surface | Deferred | This is River-Ledger-local presentation. A shared neutral-card surface, if earned, routes through `docs/MECHANIC-ATLAS.md` / a later UI-infra spec — not this one. |
| New `engine-core` types for cards/hands/explanations | Forbidden | `engine-core` stays noun-free (§3); explanation/card types are `games/river_ledger`-local; shared TS components may know mechanic nouns. |
| A canonical numeric hand "score" presented as engine-computed | Forbidden | Hold 'Em has no numeric score; the teaching aid is a labeled category-ladder position, not a fake value. |
| YAML / DSL for any new copy or metadata | Forbidden | Typed Rust strings or typed static content keyed to Rust IDs only; unknown fields rejected. |

---

## 4. Foundation and boundary alignment

| Authority | Constraint engaged | Spec alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §2 | Rust owns view projection, scoring, terminal detection; TS presents only | Every new hand name, category label, decisive comparison, and rank explanation is a Rust-authored view field built in `showdown.rs`/`visibility.rs` from already-authorized showdown results; TS chooses layout/typography only. |
| `docs/FOUNDATIONS.md` §3 | `engine-core` noun-free | No `engine-core` change. Explanation/card view types stay in `games/river_ledger`; "card"-aware React components stay in `apps/web`. |
| `docs/FOUNDATIONS.md` §5 + `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Static data is typed content/metadata; explanation templates keyed to Rust effects/actions; no behavior fields | Hand-ranking ladder labels/definitions and any copy templates are inert content keyed to Rust IDs; no selectors/branches; unknown fields rejected. |
| `docs/FOUNDATIONS.md` §7 / §11 | Public UI play-first, cozy, not casino, not debug-dominated; effect-driven animation | Redesign removes engine jargon (`Rust-evaluated`), adds card-like visuals; board reveal is reduced-motion-aware and rides existing semantic effects; no future-card preload. |
| `docs/FOUNDATIONS.md` §10 / `docs/IP-POLICY.md` | IP conservatism; no casino trade dress | Original neutral card/ledger styling; suit glyph + word, not copied card art; abstract units; `RL-UI-NOCASINO-001` audit (B6). |
| `docs/FOUNDATIONS.md` §11 | No hidden-info leaks; deterministic serialization | New fields rendered only for showdown-eligible revealed seats; folded/unrevealed seats get none; new serialized fields are deterministic and covered by fixture/golden-trace updates; a11y labels/test IDs carry no unrevealed facts. |
| `docs/FOUNDATIONS.md` §12 | Stop conditions: TS legality/evaluation, hidden-info leak, debug-first UI | Each is an explicit forbidden change (§11) for every ticket decomposed from this spec. |
| `docs/FOUNDATIONS.md` §13 | ADR triggers: visibility-contract change, replay/hash semantics | **Not tripped.** No reveal line moves; view-schema additions ride the ordinary fixture/trace migration path (`docs/TESTING-REPLAY-BENCHMARKING.md`). |
| `docs/UI-INTERACTION.md` / `docs/WASM-CLIENT-BOUNDARY.md` | TS renders Rust/WASM output; bridge contract for new fields | New explanation fields pass through the documented `crates/wasm-api` JSON seam; TS types in `client.ts` mirror them; no TS evaluation. |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | Per-viewer projection, authorized reveal | Explanation fields obey the existing per-viewer authorization: public observer and each seat see only authorized reveals; pairwise no-leak preserved across 3–6 seats. |

§12 stop conditions remain clear: no `engine-core` nouns added, no static-data behavior, no TS legality/evaluation, no hidden-info leak, no debug-first UI.

---

## 5. External UX grounding (presentation details only)

External research shapes presentation, not architecture; sources collected by the audit (2026-06-15):

- **Decisive answer first, advanced detail disclosed.** Lead with who-won + why; reserve raw tie-break vectors and rule IDs for a progressive-disclosure "Details" tier — do not hide the human reason behind a hover-only tooltip. (NN/g Progressive Disclosure; NN/g 10 Usability Heuristics.)
- **Contrastive explanation.** Name the closest challenger and why the winner beat *that* hand, not merely why the winner is good — people need "why P rather than Q." (Buçinca et al., CHI 2025.)
- **Color is never the sole channel.** Suit conveyed by glyph + suit word + (secondary) red/black; winner marked by text + icon, not color alone. (WCAG 2.2 SC 1.4.1; W3C WAI Images Tutorial.)
- **Contrast + text alternatives.** Card text ≥ 4.5:1, UI boundaries/icons ≥ 3:1; group-level accessible labels for every card and best-five group. (WCAG 2.2 SC 1.4.3; WAI.)
- **Recognition over recall.** An always-available hand-ranking ladder removes the memorization load for the category order. (NN/g heuristics.)

These reinforce decisions D1–D6; they do not reopen any boundary.

---

## 6. Committed design decisions

### D1 — Showdown explanations are Rust-authored, additive, and reveal-scoped

A new Rust-authored view object carries the human-readable layer, built in `games/river_ledger` (a `display.rs` helper or an internal section of `showdown.rs`), deterministic and test-covered:

- match-level: `headline` ("Seat 4 wins with Pair of Queens."), `decisive_comparison` ("Pair of Queens beats Pair of Eights."), `comparison_basis` ("Both hands are one pair, so the pair rank decides first: Queens > Eights.");
- per-revealed-seat: `result_label` ("Win" / "Showdown loss" / "Split win"), `hand_name` ("Pair of Queens" / "Queen-high"), `rank_explanation` ("pair rank Queen; kickers Ten, Eight, Six"), `comparison_note` (winner: beats challenger; loser: loses-to-winner), `best_five` + `best_five_accessibility_label`;
- raw `category_key` / `tie_break_vector` / rule IDs are **retained** for the details/debug tier — additive, never removed (preserves trace/replay consumers).

The fields are **additive** to the existing terminal projection (preferred over reshaping `ShowdownReveal`/`ShowdownStrengthView`) so existing serialization and replay consumers stay compatible; the view-schema addition rides the ordinary fixture/golden-trace update path. `evaluator.rs` stays the sole source of category/tie-break/best-five facts; the explanation builder consumes its output. **Leak stance:** explanation fields exist only for showdown-eligible revealed seats; folded/non-revealed seats carry no hand explanation, matching `RL-VIS-SHOWDOWN-001` / `RL-VIS-FOLDOUT-001`.

### D2 — The bridge and TS types mirror the Rust shape; TS adds no logic

`crates/wasm-api/src/lib.rs` exposes the augmented terminal JSON; `apps/web/src/wasm/client.ts` adds matching TS types. `OutcomeExplanationPanel.tsx` / `RiverLedgerBoard.tsx` render the fields and choose layout, collapse/expand, and card styling — they must not compute category labels, hand names, winner, comparison reason, or strength.

### D3 — Terminal showdown panel leads with the decisive sentence

The panel renders, in order: headline → decisive comparison + basis → board (visual cards) → winner (best-five cards, rank explanation, allocation/contribution, screen-reader summary) → closest challenger (best-five + loses-because note) → other revealed hands → hand-ranking reference (current winning category highlighted) → a "Details" disclosure carrying raw vectors and rule IDs. Engine jargon (`Rust-evaluated`) is removed. `OutcomeExplanationPanel` is a **shared catalog surface** (mounted by 15 boards via `outcomeSurfaceData()`, governed by the archived `victory-explanation-shared-surface` spec): this rich showdown layout is a **River-Ledger-specific rendering path keyed to the new explanation fields' presence**, so the other games' outcome rendering is unchanged and a cross-game non-regression check (§9) guards it.

### D4 — One neutral local River Ledger card component

A local component (extracted within/near `RiverLedgerBoard.tsx`) renders rank large, suit glyph + suit word, red/black as a *secondary* aid only, high-contrast neutral surface, and a group-level accessible label per best-five / board group. No copied card art, no green felt, no chips/currency, no branded backs. Adopted by board, hole cards, and best-five. It consumes the Rust `CardView`; it derives no rank/suit text from raw IDs.

### D5 — Always-available hand-ranking reference

A persistent (collapsible during play, default-visible after showdown) ladder: Straight flush > … > High card, with the current winning category marked. Labels/definitions are Rust/static-supplied; TS lays them out and adds no evaluation.

### D6 — Teaching-strength aid is terminal-only, ladder-form, and leak-safe

If included, the aid is a **category-ladder position** ("Pair is category 8 of 9 from strongest to weakest"), derived only from the revealed final hand, terminal-only, and visibly labeled "teaching aid, not a game value." No 0–100 equity meter, no pre-showdown meter, no derivation from deck tail / folded hands / future board / opponent private cards. This is the lowest-priority WS-B item and may be dropped without affecting any other item.

### D7 — Action-panel, seat, turn-flow, and copy hygiene

Action controls show call price / adds-to-ledger / cap-remaining from Rust legal-action metadata (`required_to_call` and `adds_to_pot` already exist on the action choice — render-only; cap-remaining is not yet a projected field and needs an **additive Rust projection**, never a TS computation); unavailable reasons appear only if Rust-authored and viewer-safe. Seat panels show button/blind/active state by text + icon (not color alone). A street strip shows `Preflop → Flop → Turn → River → Showdown` from public street state. Board reveal is reduced-motion-aware and rides existing semantic effects (no future-card preload into DOM/a11y/test IDs). A normal-mode public-copy audit removes casino vocabulary per `RL-UI-NOCASINO-001`; "pot" may remain in debug/rule detail only.

---

## 7. Deliverables

```text
games/river_ledger/src/showdown.rs        (or new display.rs: deterministic explanation builder)
games/river_ledger/src/state.rs           (additive explanation view fields on terminal projection)
games/river_ledger/src/visibility.rs      (reveal-scoped projection of explanation fields)
games/river_ledger/src/ui.rs              (hand-ranking ladder labels/definitions; copy metadata)
crates/wasm-api/src/lib.rs                (augmented terminal JSON)
apps/web/src/wasm/client.ts               (TS types for the new viewer-safe fields)
apps/web/src/components/OutcomeExplanationPanel.tsx   (shared surface — river-ledger-specific showdown rendering path; no regression to other games)
apps/web/src/components/outcomeExplanationTemplates.ts (jargon removed; player-facing copy)
apps/web/src/components/RiverLedgerBoard.tsx          (neutral card component; seat/turn-flow; action panel copy)
games/river_ledger/tests/{rules,property,visibility,serialization,replay}.rs (explanation + no-leak coverage)
games/river_ledger/tests/golden_traces/*.trace.json   (view-JSON updates where explanation fields appear)
apps/web/e2e/river-ledger.smoke.mjs       (worked-example showdown assertion; no-leak DOM/storage/log sweep)
games/river_ledger/docs/RULE-COVERAGE.md  (UI-row refresh: deferred → covered/covered-by-trace)
games/river_ledger/docs/UI.md             (reconcile outcome/contract rows with the redesign)
apps/web/README.md                        (catalog/Shell Surface reconciliation if surface names change)
specs/README.md                           (index row maintained: Planned → Done at closeout)
```

Per-game Rust changes ride the ordinary verification set: unit/rule tests, `fixture-check`, `replay-check`, `rule-coverage`, serialization tests, golden-trace updates where view JSON changes, `simulate` smoke, and `boundary-check.sh`.

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

Dependency order; each item is one reviewable diff. Decomposed into `tickets/` via `/reassess-spec` then `/spec-to-tickets` after acceptance.

| # | Item | Workstream | Depends on |
|---|---|---|---|
| WB1 | Rust explanation builder + additive terminal view fields (D1); unit tests for headline/decisive/hand-name/rank-explanation on the worked example and split/foldout paths; serialization + golden-trace updates | A (R1) | — |
| WB2 | Reveal-scoped projection through `visibility.rs`; no-leak tests proving folded/unrevealed seats carry no explanation; viewer-hash + replay-export coverage | A (R1, R7) | WB1 |
| WB3 | `wasm-api` JSON + `client.ts` types (D2); bridge no-leak test | A (R1) | WB2 |
| WB4 | River-Ledger-specific showdown rendering path in the shared `OutcomeExplanationPanel` (D3), keyed to the new explanation fields: decisive sentence, winner/challenger contrast, details disclosure; jargon removed from `outcomeExplanationTemplates.ts`; ui smoke incl. cross-game outcome-panel non-regression | A (R2, R3) | WB3 |
| WB5 | e2e worked-example assertion ("Queens beat Eights" rendered) + DOM/storage/log no-leak sweep in `river-ledger.smoke.mjs` | A (R12, R7) | WB4 |
| WB6 | Neutral River Ledger card component (D4): glyph + suit word + group a11y label; adopt for board, hole cards, best-five; ui smoke | B (R4) | WB4 |
| WB7 | Hand-ranking reference (D5): Rust/static labels via `ui.rs`; persistent/collapsible; default-visible after showdown | B (R5) | WB3, WB6 |
| WB8 | Action-panel copy from Rust metadata (D7): call price / adds-to-ledger (render `required_to_call`/`adds_to_pot`, already projected) / cap-remaining (needs an additive Rust projection field) / viewer-safe unavailable reasons | B (R9) | WB6 |
| WB9 | Seat + turn-flow affordances (D7): button/blind/active by text+icon, street strip, reduced-motion reveal | B (R10) | WB6 |
| WB10 | Optional teaching-strength aid (D6): terminal-only category-ladder position, labeled non-canonical, leak-safe | B (R6) | WB7 |
| WB11 | Public-copy casino-vocabulary audit (D7): normal-mode copy hygiene, `RL-UI-NOCASINO-001` | B (R11) | WB4, WB8, WB9 |
| WB12 | Closeout: refresh `RULE-COVERAGE.md` UI rows (deferred → covered) + reconcile `UI.md`; doc-link/catalog checks; flip this spec's index row to `Done` with evidence | B7 (R8) | all |

---

## 9. Exit criteria

1. After a showdown, the panel states the decisive sentence (e.g. "Pair of Queens beats Pair of Eights.") and a basis line, with named hands ("Pair of Queens", not `one_pair`) — all from Rust-authored fields; no TS computes any of it.
2. The winner's and closest challenger's best-five render as neutral visual cards (glyph + suit word, colorblind-safe), each with a group accessibility label; raw tie-break vectors and rule IDs appear only behind the "Details" disclosure.
3. The worked example (board `4C 3D QH 6H 8H`, Seat 4 vs Seat 0) renders the legible explanation and is asserted by an e2e test; the engine's `[12,10,8,6]` vs `[8,12,10,6]` encoding is confirmed by the explanation it produces.
4. A hand-ranking reference is reachable at all times and default-visible after showdown, with the winning category marked.
5. No normal-mode River Ledger surface contains engine jargon (`Rust-evaluated`, etc.) or raw enum/vector text outside the details tier; the casino-vocabulary audit row is recorded.
6. All view-schema additions pass `cargo test --workspace`, `fixture-check`, `replay-check`, `rule-coverage`, serialization tests, and updated golden traces; `simulate` smoke behavior unchanged; `boundary-check.sh` clean.
7. No-leak evidence: observer/wrong-seat DOM, `data-testid`, storage, and console contain no unrevealed hole cards, future board, hand-strength of folded seats, or private rationale; folded seats carry no explanation field; the teaching aid (if shipped) derives only from authorized revealed hands.
8. Accessibility: cards and best-five groups have accessible names; the details tier uses disclosure semantics; suit/winner conveyed by text + icon (not color alone); reduced motion preserves all facts; contrast meets WCAG AA.
9. `RULE-COVERAGE.md` UI rows reflect the shipped surface (no longer `intentionally-deferred` where the redesign now proves them); `UI.md` reconciled; `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` pass.
10. No regression to the other 14 catalog games' outcome rendering: `smoke:ui` across the catalog stays green, and the River-Ledger showdown layout activates only when the new explanation fields are present on the projected rationale.

## 10. Acceptance evidence

- Rust: unit tests for the explanation builder (worked example, split, foldout, high-card, every category's hand-name/rank-explanation), additive serialization/golden-trace fixtures, reveal-scoped no-leak tests; `cargo fmt/clippy/build/test` gate 0 clean.
- Tools: `fixture-check`, `replay-check`, `rule-coverage` for `river_ledger`; `boundary-check.sh` clean.
- Web: `smoke:wasm`, `smoke:ui`, `smoke:effects` green; new smoke/e2e cases for the showdown panel, neutral card component, hand-ranking reference, and the worked example; no-leak DOM/storage/log sweep attached to WB5/WB2 outcomes.
- Screenshots: before/after of the showdown panel and the board cards attached to WB4/WB6 outcomes; teaching aid (if shipped) rendered with its non-canonical label.

## 11. Forbidden changes

- No change to evaluator, pot, betting, showdown, or visibility *behavior* — presentation and additive view fields only.
- No visibility-contract change, no new reveal, no reveal-timing change (FOUNDATIONS §13 ADR trigger); folded/unrevealed seats gain no explanation.
- No `engine-core` edits; no `game-stdlib` additions or helper promotion outside the atlas process.
- No TypeScript-computed legality, evaluation, winner, split, hand name, category label, or "why" narrative; no rank/suit text derived from raw IDs.
- No all-in / side-pot / no-limit / pot-limit / stacks / rake / payout / tournament features; their absence is not a defect.
- No casino trade dress, real-money framing, or copied card/table art; abstract units stay abstract.
- No fake canonical numeric hand score; no pre-showdown or hidden-derived strength meter.
- No YAML, no DSL, no behavior-looking fields in any new static content; unknown fields rejected.
- No replay/hash semantic change; trace/fixture updates only through the ordinary migration path.
- No weakening or deletion of existing tests (AGENT-DISCIPLINE §4); fixes ship with strengthened coverage.

## 12. Documentation updates required

- `games/river_ledger/docs/RULE-COVERAGE.md`: UI rows (`RL-UI-PRESENT/SEATS/ACTIONS/PREVIEW/LEDGER/SHOWDOWN/NOCASINO/NOLEAK-001`, `RL-OOS-BROWSER-001`) refreshed from `intentionally-deferred` to the proven status with the new evidence (WB12).
- `games/river_ledger/docs/UI.md`: outcome/contract rows reconciled with the redesigned panel and the new Rust-authored explanation fields.
- `apps/web/README.md`: catalog/Shell Surface reconciliation only if a surface name changes (`check-catalog-docs.mjs` must stay green).
- `specs/README.md`: index row for this spec (added at authoring as `Planned`; flipped to `Done` at WB12 with evidence).
- Whether the new Rust-authored explanation fields warrant new `RL-UI-*` sub-rule IDs in `games/river_ledger/docs/RULES.md` is decided at decomposition (assumption A4); if so, those IDs and their coverage rows land with WB1/WB12.

## 13. Sequencing

- **Predecessor:** Gate 15 (`river_ledger`) — `Done` (2026-06-14); its shipped showdown UI (RIVLEDOUT-001/002) is the surface this spec improves. The audit `reports/river-ledger-correctness-and-presentation-report.md` is the motivating input.
- **Position:** Non-gate public-polish spec, same admission class as the archived `victory-explanation-shared-surface` / `card-and-action-presentation-shared-surfaces` UI-infra specs: complete it as polish on the shipped game; it does **not** block and is not blocked by the next mechanic-ladder unit. No open promotion debt is introduced (no `game-stdlib` change).
- **Relation to Gate 15.1:** independent. Gate 15.1 (all-in / side pots) remains the next *gate* unit; this presentation work neither depends on nor blocks it. If Gate 15.1 later changes the terminal outcome shape, the additive explanation fields extend rather than conflict.
- **Successor:** none required; a future shared neutral-card / explanation surface, if earned across games, routes through `docs/MECHANIC-ATLAS.md` and a separate UI-infra spec.

## 14. Assumptions (one-line-correctable)

1. **(A1) Spec packaging** — assuming one combined spec with two workstreams (user-selected), not split per slice; Slice 2 stays in-spec rather than a deferred seed row.
2. **(A2) Index placement** — assuming a non-gate row in the active-epoch tracker (admission class "complete as polish; non-blocking"), not a new gate; move it to the completed-ladder non-gate section if preferred.
3. **(A3) Field carrier** — assuming additive view fields on the existing terminal projection (D1) rather than a reshaped `ShowdownReveal`; flip to a reshape only if the additive path proves awkward during decomposition.
4. **(A4) New rule IDs** — assuming the explanation fields satisfy existing `RL-UI-SHOWDOWN-001` and may add `RL-UI-*` sub-rules at decomposition; if RULES.md must gain IDs, that is a WB1/WB12 doc deliverable, not a behavior change.
5. **(A5) Teaching aid** — assuming the strength aid (D6/WB10) is included in the ladder-only, leak-safe form; drop WB10 entirely if unwanted — no other item depends on it.
6. **(A6) Card component scope** — assuming a River-Ledger-local card component, not a shared web surface; promotion is explicitly deferred (§3.3).
7. **(A7) Research basis** — external UX grounding reuses the audit report's cited sources (NN/g, WCAG 2.2, WAI, Buçinca et al.); no fresh research pass was run. Sufficient for presentation grounding; commission `research-brief`/`deep-research` if deeper grounding is wanted.

## 15. Triage of audit backlog → workstreams

Source: `reports/river-ledger-correctness-and-presentation-report.md` Part C (R1–R12) + Part A.

| Audit item | Verdict | Disposition |
|---|---|---|
| Part A — correctness audit (no defect) | informational | No implementation; recorded as §3.3 non-goal. |
| Absent all-in / side pots / no-limit | reject-as-bug | Intended Gate-15 exclusion; tracked as planned Gate 15.1. |
| R1 — Rust-authored showdown explanation fields | accept | WS-A — WB1/WB2/WB3 (D1/D2) |
| R2 — redesign terminal showdown panel | accept | WS-A — WB4 (D3) |
| R3 — named copy main display; vectors to details | accept | WS-A — WB4 (D3) |
| R4 — neutral card component | accept | WS-B — WB6 (D4) |
| R5 — always-available hand-ranking reference | accept | WS-B — WB7 (D5) |
| R6 — terminal teaching-strength aid | accept-with-modification | WS-B — WB10, ladder-only/leak-safe (D6) |
| R7 — strengthen explanation/no-leak tests | accept | WS-A — WB2/WB5 |
| R8 — refresh `RULE-COVERAGE.md` UI rows | accept | B7 — WB12 (verified drift) |
| R9 — action-panel copy from Rust metadata | accept | WS-B — WB8 (D7) |
| R10 — seat/turn-flow affordances | accept | WS-B — WB9 (D7) |
| R11 — public copy casino-vocabulary audit | accept | WS-B — WB11 (D7) |
| R12 — e2e worked-showdown scenario | accept | WS-A — WB5 |

---

## Closeout

Completed: 2026-06-15

Implementation summary:

- Added Rust-authored, reveal-scoped River Ledger showdown explanation fields
  for headline, decisive comparison, comparison basis, per-seat result label,
  hand name, rank explanation, comparison note, best-five cards, best-five
  accessibility label, and terminal-only category-ladder position.
- Projected the explanation fields through visibility and the WASM bridge,
  preserving folded-seat redaction and keeping TypeScript as presentation only.
- Reworked the River Ledger showdown panel around the decisive player-facing
  sentence, best-five card groups, hand-ranking reference, and details-only raw
  category/vector/rule data.
- Added the local neutral River Ledger card component, hand-ranking reference,
  action metadata copy, seat/turn affordances, street strip, no-casino public
  copy audit, and e2e worked-example assertion for Pair of Queens beating Pair
  of Eights.
- Reconciled `games/river_ledger/docs/RULE-COVERAGE.md`,
  `games/river_ledger/docs/UI.md`, and the active `specs/README.md` row. The
  `RL-UI-PREVIEW-001` row remains `intentionally-deferred` because this series
  did not ship a separate River Ledger preview surface.

Acceptance evidence:

- `cargo test -p river_ledger`
- `cargo test -p wasm-api`
- `cargo run -p fixture-check -- --game river_ledger`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p rule-coverage -- --game river_ledger`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `node apps/web/e2e/outcome-explanation.smoke.mjs`

Screenshot note:

- No standalone before/after screenshot artifacts were added during this ticket
  series. The shipped browser behavior was verified through the dedicated
  River Ledger and full-catalog e2e smoke lanes instead.
