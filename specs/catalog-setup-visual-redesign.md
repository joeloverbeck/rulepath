# catalog-setup-visual-redesign — Catalog & match-setup visual redesign with optional typed variant descriptions

- **Filename:** `specs/catalog-setup-visual-redesign.md`
- **Spec ID:** `catalog-setup-visual-redesign`
- **Target type:** New spec
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate
- **Roadmap build gate:** None. Non-gate sibling of `rules-display-shared-surface`,
  `victory-explanation-shared-surface`, `card-and-action-presentation-shared-surfaces`,
  `action-consequence-and-match-context-shared-surfaces`, and
  `effect-animation-and-turn-orchestration`. Sourced from the prioritization
  brainstorm `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md`
  (recommendation **P2** = candidates **C5** picker visual redesign + **C6**
  one-line variant descriptions).
- **Status:** Planned
- **Date:** 2026-06-13
- **Owner:** joeloverbeck
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` →
  `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs
  (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`,
  `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`,
  `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` →
  `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` →
  `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → this spec.
- **Subordination:** Subordinate to the foundation set and accepted ADRs. It
  drafts lift-ready amendments; it does not silently amend upstream law.
- **Research basis:** Two delivered research artifacts —
  `reports/catalog-setup-visual-redesign-research-report.md` (Deliverable A: deep
  research, exact-commit-grounded against `f8c94e6`) and
  `reports/catalog-setup-design-direction.md` (Deliverable B: concrete
  implementation direction). No fresh research run: this spec embeds no original
  game/rules design — it is pure presentation plus one inert typed content field
  — so the delivered reports are sufficient (§14 A8).

---

## 1. Objective

Make the public **catalog picker** and **match-setup** surfaces present all 14
official games as a coherent, polished, IP-safe board-game portfolio rather than
a text-first debug index. A visitor should be able to understand the collection,
pick a game, choose a variant, understand who plays what, and reach How-to-Play
**without ever seeing raw IDs or engine/debug vocabulary** (FOUNDATIONS §1, §7).

The redesign is **pure presentation except for one optional typed inert field**:
`GameVariantCatalogEntry.description?`, authored as `Option<String>` in Rust
variant data and projected through `crates/wasm-api` into TypeScript catalog
data. It adds **no** legality decision, hidden-information payload, behavior
selector, rule procedure, bot logic, renderer change, UI framework, AI
illustration, or trade-dress-adjacent asset.

Three fused workstreams:

1. A **CSS design-token layer** over the existing single vanilla stylesheet, with
   primitive and semantic tokens and per-game accent/shape theming.
2. An **original inline-SVG icon system** for all 14 games, plus inert per-game
   `catalog_theme` UI metadata in `games/<id>/src/ui.rs`, and a redesigned
   fixed-aspect catalog card and a selected-game match-setup hero with prominent
   seat/faction labels.
3. An **optional one-line variant `description?`** for the **multi-variant**
   games, threaded from their variant catalog structs (`variants.rs` / variant
   TOML) → `crates/wasm-api` → `GameVariantCatalogEntry.description?`, omitted when
   absent, guarded by parser behavior-key rejection and a `smoke-ui.mjs` shape
   assertion. Single-variant games keep their existing "Single setup" line (§4.3).

This is the largest remaining first-impression gap: the functional groundwork
(variant selector, faction-labeled seats, whole-card click, How-to-Play surface,
viewer-filtered effects, animation scheduler) is all shipped by the five Done
UI-infrastructure specs; only the catalog/setup *visual identity* and the
choice-support variant copy remain.

---

## 2. Current state (code-grounded, verified 2026-06-13 against current `main`, `f8c94e6` at authoring)

- **The catalog is text-first.** `apps/web/src/components/GamePicker.tsx` renders
  a `.game-list` grid of `.game-card` entries with a primary selection button,
  display-name heading, a `gameSummary()` line, hidden-info / viewer-mode badges,
  and a secondary "How to Play" button. Whole-card click already works. No
  per-game art, accent, or theme identity exists.
- **Setup is functional but flat.** `apps/web/src/components/MatchSetup.tsx`
  renders the selected game's display name, a variant `<select>` when multiple
  variants exist, mode radios, a How-to-Play button, and a `.seat-roles` grid
  derived from `ui.seat_labels` (`MatchSetup.tsx:169`). `ui.faction_labels`
  exists in TypeScript catalog metadata but is **not surfaced prominently** in
  setup (no `faction_labels` reference in `MatchSetup.tsx`).
- **Variant catalog type is `id + label` only.** `GameVariantCatalogEntry`
  is `{ id: string; label: string }` (`apps/web/src/wasm/client.ts:75-78`);
  `GameCatalogEntry` already carries `variants?`, viewer-mode flags,
  hidden/cooperative/tags, and `ui?` with `seat_labels?` and `faction_labels?`.
- **WASM projection emits `id + label` only.** `variant_json(id: &str, label:
  &str)` and `variants_json(variants: &[(&str, &str)])`
  (`crates/wasm-api/src/lib.rs:376,384`) emit `id` and `label` only. All 14
  games' catalogs route through `variants_json` in `list_games()`, but with a
  split data source: the 3 multi-variant games (`flood_watch`,
  `frontier_control`, `event_frontier`, `lib.rs:478-504`) build their tuples
  from the games' **variant catalog structs** (`flood_variants.deluge.display_name`,
  …), while the 11 single-variant games build theirs from **hardcoded
  display-name constants** (e.g. `(VARIANT_TOKEN_BAZAAR_STANDARD,
  GAME_TOKEN_BAZAAR_DISPLAY_NAME)`, `lib.rs:513`) and read no variant struct.
- **No `description` field exists anywhere.** No `games/*/src/variants.rs`
  defines a `description` field (all use `id` + `display_name` + parameters and
  reject unknown/behavior-looking keys). Multi-variant TOML catalogs
  (`flood_watch`, `frontier_control`, `event_frontier` `data/variants.toml`) list
  variant IDs and display names only.
- **Smoke shape test asserts `id + label`.** `apps/web/scripts/smoke-ui.mjs`
  `hasVariant(game, id, label)` (line 53) asserts id/label pairs plus game IDs,
  flags, hidden/cooperative/tags, and representative UI metadata; it does **not**
  assert an optional `description` field. `smoke-effect-feedback.mjs` carries no
  catalog-shape checks and should remain unaffected.
- **Styling is one vanilla stylesheet, no framework.** The app is React 19 + Vite
  + TypeScript; `apps/web/src/styles.css` is a single vanilla stylesheet (~3,607
  lines) with no UI or CSS framework dependency.
- **`ui.rs` is the proven local home for inert presentation metadata.** 13 of 14
  games have `games/<id>/src/ui.rs` carrying original presentation metadata
  (shapes, pattern labels, seat/faction labels, reduced-motion tokens).
  `race_to_n` is the **only** official game without `src/ui.rs` at this commit
  (verified: `games/race_to_n/src/` has no `ui.rs`).
- **`game-stdlib` is not the home.** `crates/game-stdlib` exports only the
  rule-agnostic `board_space` helper; it is not an appropriate home for catalog
  presentation helpers (and §6 D14 keeps theme metadata local).
- **No atlas pressure.** `docs/MECHANIC-ATLAS.md` §10A open promotion-debt
  register is empty; UI-INTERACTION §10A states repeated presentation shapes
  (per-game `ui.rs` display metadata) are governed by UI law and are **not**
  mechanic-atlas promotion pressure. This is presentation work, not promotion.

---

## 3. External grounding

Delivered research basis (see `reports/`); presentation guidance only, no
architecture decision rests on an external source.

- **First impressions are a product surface.** The aesthetic-usability effect and
  Lindgaard et al.'s ~50 ms first-impression finding mean the catalog's first
  screen needs immediate structure, warmth, and confidence, not a raw data list
  ([Aesthetic-Usability, NN/g]; [Lindgaard 2006]). The current text-only cards
  undercut the public-playable priority (FOUNDATIONS §1).
- **14 games is a grid, not a storefront.** Choice-overload and Hick's-law
  evidence support a light fixed-ratio grid with clear hierarchy over a filter
  sidebar / onboarding quiz / launcher chrome ([Iyengar & Lepper 2000];
  [Hick's Law]). Storefront catalogs (BGA, Tabletopia, Steam, itch) add filters
  only at large scale; their transferable lesson is fixed safe areas and
  small-size legibility, **not** licensed box art or commerce cues.
- **Fixed slots beat masonry.** Scanning-pattern and grid/list research argue for
  equal-height fixed-aspect cards with a consistent icon well, title, summary,
  flags, and How-to-Play placement, plus text clamping
  ([F-pattern, NN/g]; [Grid vs. List, NN/g]; [Baymard product lists]).
- **Icons are identity marks, not labels.** Icon-usability and WCAG 2.2 evidence
  require visible text labels, shape-plus-color distinction (not color alone),
  non-text contrast, and small-size comprehension testing; decorative SVGs hide
  from assistive tech when adjacent text names the game, standalone SVGs need a
  real accessible name ([Icon Usability, NN/g]; [WCAG 2.2]; [WAI decorative
  images]; [Scott O'Hara SVG a11y]).
- **Tokens over vanilla CSS, no framework.** The Design Tokens Community Group
  format and CSS custom properties support primitive→semantic tokens and scoped
  per-game theming without a build pipeline or UI framework; `prefers-reduced-
  motion` minimizes non-essential motion ([DTCG 2025]; [MDN CSS custom
  properties]; [MDN prefers-reduced-motion]).
- **Descriptions are choice-support microcopy, not rules.** Progressive
  disclosure and UX-writing guidance favor a concise one-line descriptor that
  helps a user choose a variant (tone, pressure, duration, public setup feel) and
  keeps procedure in the authored How-to-Play surface ([Progressive Disclosure,
  NN/g]; [3 C's of UX Writing, NN/g]).
- **IP-safe abstraction by game family.** Represent the abstract pressure, not the
  commercial look: no red/yellow plastic 4-in-a-row grid, no checkerboard/crown,
  no suits/chips/green felt/casino vocabulary, no bridge/whist branding, no mask
  clichés — original geometric SVG is both safer and a better match for the
  "warm, tactile, polished, original" doctrine ([IP-POLICY]; FOUNDATIONS §7/§10).

---

## 4. Goal, scope, and non-goals

### 4.1 Goal

Every official game presents a distinct, original, color-plus-shape catalog
identity on a consistent fixed-aspect card; the selected game carries its
identity into a polished setup hero with variant description and prominent
seat/faction labels; an optional one-line variant `description?` flows from Rust
typed inert content to TypeScript and renders only when present; and How-to-Play
is reachable from every card and the setup hero. Rust behavior authority and
TypeScript presentation-only boundaries are preserved throughout.

### 4.2 In scope

| In scope |
|---|
| CSS custom-property design-token layer (primitive + semantic tokens) inside `apps/web/src/styles.css`. |
| Fixed-aspect responsive catalog-card redesign in `GamePicker.tsx` (art well, eyebrow, title, summary, flags, How-to-Play), preserving display names, whole-card selection, and a separate How-to-Play action. |
| Original inline-SVG icon set for all 14 official games in a new `GameCatalogIcon.tsx` registry; 24×24 base grid, `currentColor`/CSS variables, monochrome-legible at small sizes. |
| Per-game inert `catalog_theme` UI metadata in `games/<id>/src/ui.rs` (icon id, theme key, accent/shape token names, a11y label); add a minimal `games/race_to_n/src/ui.rs`. |
| Match-setup polish: selected-game hero, variant-description display, and a "Players & roles" section surfacing `ui.seat_labels` and `ui.faction_labels` prominently. |
| Optional typed inert `description?` for the **multi-variant** games: `Option<String>` in `variants.rs` / variant TOML, fed from the variant catalog structs through `variant_json`/`variants_json` into `GameVariantCatalogEntry.description?`; omitted when absent. |
| Parser hardening: accept only the new description key; keep behavior-key rejection (`when`/`if`/`then`/`selector`/`trigger`/`effect`/`action`/`legal`, …); length and behavior-like-prose validation. |
| `smoke-ui.mjs` shape assertion for `description?` (presence/absence, length ≤120, no behavior-like prose). |
| Hover / focus-visible / active / selected / loading / empty / failure states; keyboard traversal; reduced-motion compliance. |
| Per-asset IP closeout: one originality + smallest-size-legibility row per SVG/motif/theme. |

### 4.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Any Rust behavior change — legality, validation, state transitions, scoring, terminal detection, RNG, visibility/view projection, bots, replay/hash, serialization | Forbidden | FOUNDATIONS §2/§11/§12. The only Rust change is an inert `Option<String>` content field; nothing behavioral. |
| New legality, action availability, or rule procedure in TypeScript | Forbidden | FOUNDATIONS §2/§7. TS renders only; descriptions are never parsed for behavior, conditions, tags, or layout. |
| Renderer replacement (Canvas/PixiJS) or route-architecture rewrite | Forbidden | React + SVG is the v1 default (FOUNDATIONS §7; UI-INTERACTION §4); replacement needs profiling/ADR. |
| Tailwind / Chakra / MUI / Radix-as-framework / Bootstrap / any full UI framework | Forbidden | §6 D2; the redesign is achievable with tokens over vanilla CSS at low migration risk. |
| AI-generated, figurative, or proprietary illustration; copied icons/prose/screenshots/fonts/trade dress | Forbidden | FOUNDATIONS §10; IP-POLICY. Every SVG is project-authored abstract geometry. |
| `game-stdlib` UI helper or mechanic-atlas promotion row | Forbidden here | Presentation shapes are not atlas pressure (UI-INTERACTION §10A; MECHANIC-ATLAS §10A register empty); theme/icon metadata stays local in `ui.rs` (§6 D5/D14). |
| Effect-log / turn-orchestration / animation-scheduler changes | Out of scope | Delivered by `effect-animation-and-turn-orchestration` (P1, Done). P2 keeps `smoke-effect-feedback.mjs` green and untouched except incidental build compatibility. |
| Variant *behavior*, new variant selection logic, or re-spec of whole-card click | Out of scope | The functional base shipped by `action-consequence-and-match-context-shared-surfaces`; P2 reuses it (§14 A4). |
| Projected `description?` for **single-variant** games | Out of scope | Their `list_games()` catalog tuples are constant-fed, not struct-fed (`lib.rs:513`); descriptions are choice-support for picking *between* variants, so single-variant games keep the existing "Single setup" line. Revisit only if a single-variant description is later wanted (§14 A10). |
| Amending `templates/PUBLIC-RELEASE-CHECKLIST.md` | Out of scope | Per the brainstorm note, P2 carries per-asset IP-check rows in its **own** closeout, not a template change (§12; §6 D13). |
| FOUNDATIONS amendment / new ADR | Not needed | §5 verdict (verified against `docs/FOUNDATIONS.md`, read in full): the work lands inside §5/§7/§10 law and trips no §13 ADR trigger. |
| YAML / DSL / behavior-bearing data | Forbidden | FOUNDATIONS §5. Descriptions and theme tokens are typed inert content, not selectors. |

---

## 5. Foundation and boundary alignment

| Authority | Constraint engaged | Stance | Alignment |
|---|---|---|---|
| `docs/FOUNDATIONS.md` §1 (priority order) | Polished public playable site first | aligns | The catalog/setup is the front door of the portfolio site; this is the highest-leverage first-impression investment left. |
| `docs/FOUNDATIONS.md` §2 (behavior authority) | Rust owns behavior; TS presents | aligns | Rust supplies typed inert `description`/`catalog_theme`; TS renders only. No legality, outcome, or effect is computed in TS; descriptions are never parsed for behavior. |
| `docs/FOUNDATIONS.md` §3 (`engine-core` noun-free) | No kernel change | aligns / N/A | No `engine-core` edit; catalog/variant vocabulary lives in `games/*` and `apps/web`. |
| `docs/FOUNDATIONS.md` §4 / §11 (`game-stdlib` earned) | No speculative promotion | aligns | Theme/icon metadata stays local in `ui.rs`; no `game-stdlib` helper, no atlas row (§6 D5/D14; UI-INTERACTION §10A). |
| `docs/FOUNDATIONS.md` §5 (static data is typed content) | Allowed: display metadata, typed variants, UI metadata, icon IDs; forbidden: selectors/triggers/behavior | aligns | `description?` and `catalog_theme` are typed inert content; the parser rejects behavior-like keys/prose; unknown-key rejection keeps holding. No YAML/DSL. |
| `docs/FOUNDATIONS.md` §7 (public UI central; cozy-premium; React+SVG) | Visual direction; renderer default | aligns | Token system + fixed card anatomy + original SVG deliver warm/tactile/restrained; React+SVG unchanged. Avoids casino/SaaS/debug/trade-dress per the §7 prohibition list. |
| `docs/FOUNDATIONS.md` §10 / `docs/IP-POLICY.md` | IP conservatism; original assets | aligns | All SVG/motif/theme is project-authored abstract geometry; no copied art/prose/fonts/screenshots/trade dress; per-asset originality + smallest-size legibility recorded in closeout. |
| `docs/FOUNDATIONS.md` §11 (no hidden-info leak) | Payload/DOM/test-ID/log/export safety | aligns | Descriptions and theme metadata forbid hidden identities/state; `description` is omitted (never `null`) when absent; no new payload category; no leak surface added. |
| `docs/FOUNDATIONS.md` §12 (stop conditions) | "static files act procedural"; "TS decides legality"; "hidden info reaches payloads" | aligns | Every decomposed ticket carries these: descriptions are inert prose with behavior-key rejection; TS never derives legality; no hidden state enters the catalog payload. |
| `docs/FOUNDATIONS.md` §13 (ADR triggers) | Renderer/DSL/visibility/replay/bot authority | N/A | None triggered: no renderer replacement, no data format, no visibility-contract move, no replay/hash change, no bot-authority change. |
| `docs/UI-INTERACTION.md` §2/§7/§10A/§16/§17 | Restrained motion; color-plus-shape; presentation-shape governance; a11y facts as text | aligns | Motion is brief/tone-keyed; every identity/state has shape+text, not color alone; theme metadata is governed by §10A as non-atlas presentation; all facts remain text-available. |
| `docs/OFFICIAL-GAME-CONTRACT.md` §10/§12 | Per-game UI metadata; catalog docs | aligns | `catalog_theme` rides the established `ui.rs` convention; `check-catalog-docs.mjs` / `apps/web/README.md` catalog lists stay reconciled (no new web-exposed game is added). |

**Foundations-amendment verdict (the operator's explicit ask): none required.**
Verified against `docs/FOUNDATIONS.md` (read in full this session). The redesign
is already covered by existing UI law (§7), the static-data boundary (§5, which
explicitly allows "display metadata, typed variants, … original component IDs,
… UI metadata"), and IP law (§10). It introduces no new architecture-changing
decision in the FOUNDATIONS §13 list: React + SVG stays the default renderer, no
YAML/DSL/data format is added, no public/private visibility contract moves, no
replay/hash semantics change, and no bot authority changes. Therefore **no
FOUNDATIONS amendment and no ADR**. The only doctrine touched is the precedented,
non-FOUNDATIONS area-doc/template lift in §10 documenting the *new* inert
`description?` / `catalog_theme` contract — no existing principle changes meaning,
so per the editorial gate this is meaning-preserving strengthening, not a
supersession (§14 A7 lets it be dropped to spec-local documentation if preferred).

---

## 6. Committed design decisions

### D1 — CSS custom-property token layer over vanilla CSS (required default)

A two-layer token model in `apps/web/src/styles.css`: **primitive tokens** (raw
color/spacing/radius/elevation/typography) and **semantic tokens** (UI meaning,
mapped to primitives and overridable per game). Added above existing selectors
and adopted incrementally; existing selectors migrate to tokens. The starter
palette in the design direction is a starting point, not law — every
foreground/background and focus/non-text pair is WCAG-checked before ship.

### D2 — No UI framework; React + inline SVG + vanilla CSS only

No Tailwind/Chakra/MUI/Radix-as-framework/Bootstrap. CSS Modules are a *future*
containment option for newly extracted component families, recorded with a local
containment reason; not the default for P2. No new runtime dependency.

### D3 — Fixed-aspect responsive catalog card with stable slots

One repeated card anatomy for all 14 games: decorative accent rail, primary
whole-card selection button (art well → eyebrow → title → summary → flags),
selected mark (shape + text, not color alone), and a separate How-to-Play button.
`grid-template-columns: repeat(auto-fit, minmax(...))`, a fixed aspect ratio, and
`-webkit-line-clamp` text clamping; settles to 3–4 desktop columns, 1 narrow
column preserving slot order. Titles are `display_name`, never raw IDs.

### D4 — Original inline SVG only; color-plus-shape identity

Project-authored abstract geometry on a 24×24 base grid, rendered at 72/96px in
card art via vector scaling; 1.75–2px stroke; `currentColor` + CSS variables
(`--game-card-art-line`, `--game-accent-2`); recognizable at 24/48/72px, in
monochrome, high contrast, and 200% zoom. Decorative + `aria-hidden` next to a
visible title; `role="img"` + `<title>` from typed `a11y_label` when standalone.
**No AI generation and no figurative/proprietary illustration.** Each game's
identity differs by **shape family** as well as accent, per the §3.5 motif
direction (e.g. `race_to_n` ascending step path; `column_four` four-token column;
`poker_lite` center-pool slab — abstract pressure, never commercial look).

### D5 — Per-game presentation metadata stays local in `ui.rs`

Inert per-game `catalog_theme` metadata (icon id, theme key, accent/secondary-
accent token names, shape token, `a11y_label`) lives in `games/<id>/src/ui.rs`,
consistent with the 13 existing `ui.rs` modules and UI-INTERACTION §10A. It
selects presentation tokens and icon IDs only — never legality, selectors, hidden
identities, action availability, rule branches, or behavior-by-naming. A minimal
`games/race_to_n/src/ui.rs` is added (the only official game lacking one). Theming
MAY bootstrap as CSS keyed on `data-game-id` and move to Rust-projected typed
metadata in the same spec or a recorded decomposition decision (§14 A3).

### D6 — Setup hero with variant description and prominent seat/faction labels

Redesign `MatchSetup.tsx` around a selected-game hero (icon, display name, public
summary, flags, How-to-Play), the existing functional variant selector with the
selected variant's optional description beneath it, and a "Players & roles"
section surfacing `ui.seat_labels` (seat + actor) and `ui.faction_labels`
(faction chips with shape token when present), falling back to "Player 1/2" only
when Rust UI metadata is absent. For asymmetric games faction labels appear near
the hero, not below mode controls. Normal-mode copy is user-facing ("Solo against
an automated opponent"), never "Rust legal bot" or engine/debug phrasing.

### D7 — Preserve two separate actions: select game and open How-to-Play

The outer card handles whole-card selection and ignores clicks originating inside
the How-to-Play control; the primary selection button stays an explicit button
for keyboard users; nested buttons are avoided. How-to-Play remains reachable from
every catalog card, the setup hero, and in-play controls, rendering authored
HOW-TO-PLAY Markdown only (never interpreted as legality/scoring/visibility).

### D8 — Optional typed inert variant `description?`

Add `description: Option<String>` (or equivalent) near each variant's display
name in `variants.rs`; for TOML-backed multi-variant games (`flood_watch`,
`frontier_control`, `event_frontier`) accept a `description` key inside each
existing variant table using the least-disruptive existing parser pattern. Only
the multi-variant games' descriptions are projected to the catalog (their
`list_games()` tuples are struct-fed); single-variant games are out of scope
(§4.3) and keep "Single setup".
Authoring contract: optional; one line; recommended 55–95 chars; **hard maximum
120 chars** after trimming; neutral original choice-support prose; **must not**
contain hidden information, rule procedure, conditionals, selectors, triggers,
legality, scoring, strategy advice, trademarks, copied prose, casino terms, or
raw IDs.

### D9 — WASM projects `description` only when present; never `null`, never TS-synthesized

Extend `variant_json`/`variants_json` so `description` is emitted only when
present and omitted entirely when absent (never `null`). The TS type becomes
`{ id; label; description? }`; rendering trims defensively and renders nothing
when absent/blank. TypeScript never synthesizes copy from a raw ID and never
parses the description for conditions, rules, tags, or layout.

### D10 — Parser hardening preserves behavior-key rejection

Variant parsers reject unknown keys by default; the change allows **only** the new
`description` key and adds length + behavior-like-prose validation, while keeping
rejection of `when`/`if`/`then`/`selector`/`trigger`/`effect`/`action`/`legal`
and similar forbidden semantics. The exact error type matches each game's existing
variant-parser style.

### D11 — Smoke shape assertion for `description?`

Extend `apps/web/scripts/smoke-ui.mjs` so `hasVariant(...)` (or a sibling) asserts:
when `description` is absent the property is absent; when present it is a non-empty
string ≤120 chars containing no behavior-like token
(`/\b(if|when|then|selector|trigger|valid_if|legal|effect|action)\b/i`). At least
one positive assertion on an authored described variant (a multi-variant game such
as `flood_watch`, `frontier_control`, or `event_frontier`) and one negative
assertion on a game intentionally left without descriptions.

### D12 — `smoke-effect-feedback.mjs` stays green and untouched

Except for incidental build compatibility; P2 is orthogonal to the effect-feedback
and animation surfaces shipped by P1.

### D13 — Per-asset IP closeout rows, not a template amendment

P2 records every SVG/motif/theme as project-authored, neutral, non-proprietary,
no trade-dress proximity, and smallest-size legible in its **own** acceptance
evidence — it does **not** amend `templates/PUBLIC-RELEASE-CHECKLIST.md`.

### D14 — No ADR, no FOUNDATIONS change, no mechanic-atlas row, no `game-stdlib` helper

Per §5. The only doc lift is the precedented area-doc/template documentation of
the new inert metadata contract (§10).

---

## 7. Deliverables

```text
apps/web/src/components/GamePicker.tsx        redesigned card anatomy, original icon slot, separate How-to-Play action
apps/web/src/components/MatchSetup.tsx        setup hero, variant-description display, prominent seat/faction labels
apps/web/src/components/GameCatalogIcon.tsx   new inline-SVG registry (14 original icons), presentation-only
apps/web/src/wasm/client.ts                   GameVariantCatalogEntry.description?
apps/web/src/styles.css                       primitive/semantic token layer + catalog/setup sections + states
apps/web/scripts/smoke-ui.mjs                 variant-description shape assertions (present/absent, length, no behavior prose)
crates/wasm-api/src/lib.rs                    variant_json/variants_json projection of optional description
games/<id>/src/variants.rs                    optional inert description field where variants are declared
games/{flood_watch,frontier_control,event_frontier}/data/variants.toml  descriptions for TOML-backed variants
games/<id>/src/ui.rs                          inert catalog_theme/icon metadata; NEW games/race_to_n/src/ui.rs
docs/UI-INTERACTION.md                        lift-ready amendment applied at closeout (§10 — catalog-theme/description contract)
templates/GAME-UI.md                          catalog identity + variant-description authoring row (§10)
specs/README.md                               index row maintained: Planned → In progress → Done with evidence
brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md  P2 marked Done at closeout, pointing to this spec
```

No behavioral Rust change is expected: the only Rust edits are the inert
`Option<String>` description field, its parser hardening, the WASM projection,
and the inert `catalog_theme` metadata.

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

| # | Item | Depends on |
|---|---|---|
| WB1 | **Token foundation:** add primitive + semantic CSS tokens to `styles.css`; migrate catalog/setup selectors; WCAG contrast pass on the palette; reduced-motion token block. | — |
| WB2 | **SVG icon registry:** create `GameCatalogIcon.tsx`; author 14 original abstract SVGs per the §3.5 motif direction; smallest-size + monochrome legibility checks. | WB1 |
| WB3 | **Per-game theme metadata:** add inert `catalog_theme` to each `games/<id>/src/ui.rs`; add minimal `games/race_to_n/src/ui.rs`; CSS `data-game-id` bootstrap and/or Rust projection per D5. | WB2 |
| WB4 | **Catalog card redesign:** rebuild `GamePicker.tsx` card anatomy (fixed aspect, art well, eyebrow/title/summary/flags, selected mark, separate How-to-Play); hover/focus/active/selected/loading/empty/failure states; keyboard traversal. | WB1, WB2, WB3 |
| WB5 | **Variant-description Rust structs + parsers:** add optional `description` to variant structs and TOML parsers; behavior-key rejection + length + behavior-prose validation; update unknown-key/parser tests. | — |
| WB6 | **WASM/TS projection (multi-variant):** extend `variant_json`/`variants_json` to carry an optional description; feed it from the 3 multi-variant games' variant catalog structs in `list_games()`; add `GameVariantCatalogEntry.description?`; defensive trim-and-render in UI (omit when absent). Single-variant constant-fed tuples are untouched (§4.3). | WB5 |
| WB7 | **Setup polish:** setup hero, selected-variant description display, prominent "Players & roles" seat/faction section; user-facing mode copy. | WB1, WB3, WB6 |
| WB8 | **Smoke + a11y checks:** extend `smoke-ui.mjs` description assertions (positive + negative); run `smoke:wasm`/`smoke:ui`/`smoke:effects`/`smoke:e2e`; verify no raw IDs and no engine/debug normal-mode copy; focus-visible + reduced-motion checks. | WB4, WB6, WB7 |
| WB9 | **Closeout:** lift §10 amendments into `docs/UI-INTERACTION.md` + `templates/GAME-UI.md`; record per-asset IP rows; flip `specs/README.md` to `Done` with evidence; mark brainstorm P2 Done pointing here. | WB1–WB8 |

Estimated size: 8–12 tickets (comparable to the predecessor UI-infra specs;
re-grounded by `/reassess-spec` before decomposition). WB1/WB5 are independent and
may start in parallel.

---

## 9. Exit criteria

1. The catalog presents all 14 official games with display names and **no raw
   IDs**; each game has a distinct original SVG identity distinguishable by shape
   **and** color (monochrome-legible at the smallest card size).
2. The card grid is responsive, fixed-aspect, fixed-slot, keyboard-reachable, and
   visually consistent; hover/focus-visible/active/selected/loading/empty/failure
   states are present and not color-only; focus is obvious and offset.
3. How-to-Play is reachable from every catalog card and from the setup surface,
   as two distinct actions from card selection.
4. Setup surfaces the selected game's identity (hero), the selected variant's
   description when present, seat labels, and faction labels when present;
   normal-mode copy has no engine/debug vocabulary.
5. `GameVariantCatalogEntry.description?` is optional at every layer for the
   multi-variant games (`variants.rs`/TOML struct → `variant_json`/`variants_json`
   → TS type → render) and is
   **omitted when absent** (never emitted as `null`, never synthesized in TS).
6. No description contains hidden state, selectors, triggers, conditionals, raw
   IDs, copied/proprietary prose, casino terms, or strategy advice; parser
   behavior-key rejection and the `smoke-ui.mjs` assertion enforce this.
7. `smoke-ui.mjs` carries a positive described-variant assertion and a negative
   omission assertion; `smoke:wasm`, `smoke:ui`, `smoke:effects`, and relevant
   `smoke:e2e` remain green; `smoke-effect-feedback.mjs` is unchanged-or-green.
8. Rust regression clean: `cargo fmt --all --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`; per touched game
   `simulate`/`replay-check`/`fixture-check`/`rule-coverage` pass (the only Rust
   change is the inert description field + theme metadata — run as regression
   proof; replays/hashes/traces unchanged).
9. No ADR, FOUNDATIONS update, mechanic-atlas change, `game-stdlib` helper,
   renderer change, or UI framework is introduced.
10. Amendments applied (`docs/UI-INTERACTION.md`, `templates/GAME-UI.md`);
    `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`,
    `node scripts/check-presentation-copy.mjs`, and `bash scripts/boundary-check.sh`
    pass; `specs/README.md` row flipped to `Done` with evidence; brainstorm P2
    marked Done.

### Acceptance evidence

Re-runnable confirmation set (non-gate spec; folded into the criteria above):

- **Web:** `npm --prefix apps/web run smoke:wasm | smoke:ui | smoke:effects |
  smoke:e2e`.
- **Screenshots:** desktop catalog, mobile catalog, selected setup, multi-variant
  setup, focus-visible card, focus-visible setup control, and a high-contrast /
  contrast-check note.
- **Variant-description JSON sample** proving present/absent optional behavior at
  the catalog boundary.
- **IP asset table:** one row per SVG motif — origin "project-authored", no copied
  references, no AI generation, no proprietary/trade-dress proximity, smallest-size
  legibility checked.
- **Boundary audit row** proving TypeScript does not parse descriptions for
  behavior.
- **Rust (regression only):** `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`; per touched game the
  simulate/replay/fixture/rule-coverage set.
- **Docs/boundary:** `node scripts/check-doc-links.mjs`,
  `node scripts/check-catalog-docs.mjs`, `node scripts/check-presentation-copy.mjs`,
  `bash scripts/boundary-check.sh`.

---

## 10. Lift-ready amendment text (applied at WB9, not before)

These are precedented area-doc/template strengthenings that **document a new inert
metadata contract**. They change no FOUNDATIONS principle's meaning and trigger no
ADR (§5 verdict; §14 A7 permits demoting them to spec-local documentation).

**`docs/UI-INTERACTION.md` addition (catalog identity + variant-description contract):**

```text
Each official game carries an inert per-game catalog identity in
games/<id>/src/ui.rs (icon id, theme key, accent/shape token names, and an
accessibility label). This metadata selects presentation tokens and an original
SVG icon only; it MUST NOT encode legality, selectors, hidden identities, action
availability, rule branches, or behavior-by-naming. Catalog identity is rendered
with color PLUS shape (and visible text), never color alone.

A variant MAY carry an optional one-line `description` (Option<String> in Rust,
projected as GameVariantCatalogEntry.description? in TypeScript). It is inert
choice-support prose — one line, <=120 characters, neutral and original. It MUST
NOT contain hidden information, rule procedure, conditionals, selectors,
triggers, legality, scoring, strategy advice, trademarks, copied prose, casino
terms, or raw IDs. It is omitted entirely when absent (never null), never
synthesized in TypeScript, and never parsed for behavior. Repeated per-game
catalog-theme/description shapes are governed here and are not mechanic-atlas
promotion pressure (see §10A).
```

**`templates/GAME-UI.md` — catalog identity + variant-description authoring row:**

```text
Catalog identity: original SVG icon id + theme key + accent/shape token names +
a11y label, recorded in games/<id>/src/ui.rs (color-plus-shape, smallest-size
legible, project-authored — no AI/figurative/proprietary art).
Variant descriptions: optional one-line inert description? per variant
(<=120 chars, neutral, no behavior/hidden-info/trademark/raw-ID), omitted when
absent.
```

---

## 11. Forbidden changes

- No legality, action availability, validation, state mutation, scoring, terminal
  detection, RNG, visibility/view-projection, bot, replay, hash, serialization, or
  any other engine behavior change; submitted command bytes, seeds, traces,
  replays, and hashes stay identical. The only Rust change is the inert
  `Option<String>` description field, its parser, the WASM projection, and inert
  `catalog_theme` metadata.
- No hidden information in catalog payloads, DOM, test IDs, rules surface, storage,
  logs, or exports; `description` carries no hidden state and is omitted (never
  `null`) when absent.
- No TypeScript legality, no parsing of descriptions for behavior/conditions/tags/
  layout, no synthesizing copy from raw IDs (FOUNDATIONS §2/§12).
- No renderer replacement (Canvas/PixiJS); no Tailwind or any full UI framework;
  no new runtime dependency.
- No AI-generated or figurative illustration; no copied icons, prose, screenshots,
  fonts, scans, trademark-forward terms, or trade-dress mimicry (FOUNDATIONS §10).
- No `game-stdlib` UI helper; no mechanic-atlas promotion row; no `engine-core`
  edit.
- No behavior-looking fields in static data; no YAML; no DSL. Descriptions/theme
  tokens are typed inert content (FOUNDATIONS §5).
- No ADR or FOUNDATIONS amendment.
- No weakening or deletion of existing tests/guards (AGENT-DISCIPLINE §4);
  `smoke-effect-feedback.mjs` and predecessor guards stay intact; unknown-key /
  behavior-key rejection stays intact and is extended, never relaxed.

---

## 12. Documentation updates required

- `docs/UI-INTERACTION.md` — catalog identity + variant-description contract
  addition + §10A non-atlas governance reference (§10 above; lifted at WB9).
- `templates/GAME-UI.md` — catalog identity + variant-description authoring row
  (§10 above).
- `specs/README.md` — index row added now (`Planned`, non-gate UI-infra);
  `Planned` → `In progress` (AGENT-TASKs executing) → `Done` (exit criteria pass
  with evidence) across lifecycle. `apps/web/README.md` needs **no** catalog-list
  change (no new web-exposed game is added; `check-catalog-docs.mjs` stays green).
- `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` — at WB9
  closeout, mark **P2** (§4) **Status: DONE** naming this spec, flip §5 sequence
  row 2 and §8 next-step 2 to Done with the same pointer. (Authoring this spec
  flips only the `specs/README.md` row to `Planned`; the brainstorm P2 row is
  marked Done at exit, not now.)
- **Do NOT** amend `templates/PUBLIC-RELEASE-CHECKLIST.md`; per-asset IP-check
  rows live in this spec's acceptance evidence (§6 D13).

---

## 13. Sequencing

- **Predecessor:** `effect-animation-and-turn-orchestration` (P1, Done, archived
  2026-06-12). It explicitly names "Catalog & setup visual redesign (P2): card
  art, picker layout, variant descriptions" as its **named successor** (its §4.3
  "Deferred" row and §13 Successor), with **no dependency** on it — P1 upgrades
  every minute of play; P2 upgrades the first thirty seconds. The earlier
  `action-consequence-and-match-context-shared-surfaces` and
  `card-and-action-presentation-shared-surfaces` shipped the functional base
  (variant selector, faction-labeled seats, whole-card click, copy hygiene) and
  twice deferred this exact presentation work.
- **Admission:** Non-gate UI-infrastructure spec; no mechanic-ladder gate is
  blocked; `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register is empty.
  Gate P (private, optional) remains subordinate to public polish (FOUNDATIONS
  §1). UI-INTERACTION §10A keeps repeated presentation metadata local, so no
  atlas work precedes this.
- **Successor (named):** none new. The remaining deferred-pool items are dormant,
  ADR-gated triggers already recorded in P1's sequencing — they are **not**
  re-owned here: **C3** staged multi-target action encoding (write the FOUNDATIONS
  §13 ADR only when leaf enumeration becomes a measured payload/bench problem) and
  **C4** EF undrawn-count visibility (a deliberate EF-VIS-002 stance; revisit only
  on playtest evidence). P2 introduces no new ADR trigger.

---

## 14. Assumptions (one-line-correctable)

1. **(A1) 14-game roster** — assuming the official catalog stays the
   target-commit 14-game roster (`race_to_n`, `three_marks`, `column_four`,
   `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`,
   `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims`, `flood_watch`,
   `frontier_control`, `event_frontier`); re-scope WB2/WB3 if the roster changes.
2. **(A2) `description?` stays optional at every layer** — assuming no game is
   forced to author one; omission is a first-class state, not a `null`.
3. **(A3) Theme-metadata home is `ui.rs`, with CSS-bootstrap latitude** — assuming
   per-game `catalog_theme` lands in `games/<id>/src/ui.rs`; theming MAY bootstrap
   as CSS `data-game-id` rules and move to Rust projection within the spec or a
   recorded decomposition decision. Pin to CSS-only if Rust projection is deemed
   unnecessary for P2.
4. **(A4) Setup reuses existing function** — assuming the redesign reuses the
   existing setup/variant-selection/whole-card-click logic and does not re-spec
   behavior; only presentation changes.
5. **(A5) All SVGs are project-authored manually** — assuming no AI generation and
   no figurative/proprietary art; every icon is original abstract geometry with a
   recorded IP row.
6. **(A6) Stack unchanged** — assuming the app stays React 19 + inline SVG +
   vanilla-CSS tokens for P2; no framework, no renderer change.
7. **(A7) §10 lift is area-doc/template strengthening, droppable to spec-local** —
   assuming the UI-INTERACTION + GAME-UI documentation of the new inert contract is
   worth landing (prevents future re-litigation, mirrors the predecessor's
   precedented lift); demote to spec-local-only documentation if the operator
   prefers no doc edits, with no effect on the build scope.
8. **(A8) No fresh research** — assuming the two delivered reports
   (`reports/catalog-setup-*.md`, exact-commit-grounded against `f8c94e6`, which
   was `main` at authoring) suffice; the spec embeds no original game/rules design, so no
   `research-brief`/`deep-research` pass was run. Commission one only if deeper
   grounding is wanted before decomposition.
9. **(A9) Effort is spec-sized analogy** — 8–12 tickets, sized against the
   predecessor UI-infra specs, not measured; `/reassess-spec` re-grounds it before
   decomposition.
10. **(A10) `description?` projection is multi-variant-only** — assuming only the
   3 multi-variant games (`flood_watch`, `frontier_control`, `event_frontier`)
   carry projected descriptions, fed from their variant catalog structs through
   `variants_json`; the 11 single-variant games' constant-fed `list_games()`
   tuples (`lib.rs:513`) are out of scope and keep "Single setup". Bring a
   single-variant game in scope only by threading a description source into its
   `list_games()` tuple.
11. **(A11) `description` is display-only** — assuming `description`, like the
   existing `display_name`, is catalog/setup presentation metadata only and never
   enters any canonical serialization, hash, or trace form (exit criterion 8
   proves this as a regression check).
