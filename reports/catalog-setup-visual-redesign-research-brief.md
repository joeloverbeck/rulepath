# Deep-research brief — Catalog & setup visual redesign (Rulepath P2)

> **You are ChatGPT-Pro running a locked deep-research session.** Produce the
> two deliverables described in §6 **directly**. Do **not** interview, ask
> clarifying questions, or re-decide what work comes next — every decision you
> need is already fixed in §3. Your job is to research deeply and externally,
> then synthesize, against the repository you have been given.

---

## 1. How to read the repository (fetch baseline + manifest)

You have access to the Rulepath repository. **Fetch every file from one exact
commit so your citations are stable:**

- **Fetch baseline commit:** `2b86670`
  (full: `2b8667087431e1ec6ea4396938f6f1d3284bf610`), branch `main`,
  tree clean at author time.
- **Uploaded manifest:** `reports/manifest_2026-06-13_2b86670.txt` — the exact
  `git ls-tree -r --name-only` of that commit (1336 paths). Treat it as the
  authoritative file list. If a path you expect is absent from the manifest, it
  does not exist at the baseline — do not invent it.

If your repo view shows a different HEAD, still read and cite the baseline
commit above; note any divergence you observe rather than silently following a
newer tree.

Rulepath is **Rust-first**: Rust owns *all* behavior; TypeScript/React is
**presentation only**. This brief concerns a pure-presentation redesign plus
one small typed-content addition — but the boundary law still binds it. Read
the law before proposing anything (§2).

---

## 2. Read-in-full set (authority-ordered, with load-bearing reasons)

Read these completely; they are the constitution and the contract this redesign
must satisfy. Authority order follows `docs/README.md`.

1. **`docs/FOUNDATIONS.md`** — the constitution. Load-bearing: **§1** priority
   order (polished public playable site is priority 1); **§2** TypeScript MUST
   own the *game picker and setup UI* and MUST NOT decide legality; **§5** typed
   inert content; **§7** the *cozy premium board-game table* aesthetic doctrine
   (warm, tactile, polished, original, readable, restrained, inviting; avoid
   casino vibes, SaaS-dashboard coldness, debug-console dominance, aggressive
   skeuomorphism, proprietary mimicry, trade-dress imitation); **§10** IP
   conservatism; **§11** universal invariants (static data is typed
   content/parameters/metadata only; no hidden-information leaks through
   payloads, DOM, test IDs, or exports); **§13** ADR triggers (so you can show
   this redesign needs **no** ADR).
2. **`docs/ENGINE-GAME-DATA-BOUNDARY.md`** — Load-bearing for the new variant
   description field: **§5** allowed static data (manifests, *display names and
   short descriptions*, icon IDs, theme tokens, typed variant selection,
   authored inert prose); **§6** forbidden static data (selectors, conditionals,
   triggers, behavior-by-naming; suspicious field names `when/if/then/selector/
   trigger/...`); **§11** UI-metadata boundary (UI metadata MAY include labels,
   icon IDs, short help, theme tokens, accessibility labels; MUST NOT include
   legality, hidden behavior, hidden identities, or any field whose meaning
   mutates state).
3. **`docs/UI-INTERACTION.md`** — the UI doctrine. Load-bearing: **§1** the game
   picker / match setup are core public UI, play-mode-first, debug secondary;
   **§2** the *Prefer/Avoid* visual-direction lists (warm surfaces, soft depth,
   premium abstract components, original SVG icons and components, *color plus
   shape — not color alone*, readable typography, respectful empty states;
   avoid proprietary mimicry, casino vibes, SaaS coldness, pasted
   screenshots/scans, trade-dress imitation); **§3** ownership split (app shell,
   picker, setup, panels are TS; renderer MUST NOT decide legality); **§10A**
   the governance statement that *repeated presentation shapes (per-game
   `ui.rs` display-metadata modules, presentation layouts) are governed by this
   document and are NOT mechanic-atlas promotion pressure* — this is why the
   redesign needs no atlas row; **§17** the accessibility baseline and the rule
   that the public UI MUST expose a shared How-to-Play / Rules surface for every
   catalog game *from the picker and setup*, rendering authored `HOW-TO-PLAY.md`
   prose only; **§19** the UI acceptance checks (display names never raw IDs;
   normal-mode surfaces carry no engine/debug vocabulary; visuals are original
   and avoid proprietary presentation).
4. **`docs/IP-POLICY.md`** — the asset gate. Load-bearing: **§1** allowed
   (original / permissioned / public-domain / generated-after-review); **§2**
   forbidden (copied prose, proprietary icons/art/fonts, trademark-forward
   presentation, trade-dress mimicry); **§4** every public game's rules-doc
   obligations and the rule that `HOW-TO-PLAY.md` is original neutral prose;
   **§5** common vs. neutral names (use the neutral catalog IDs such as
   `race_to_n`, `column_four`, `three_marks`, `draughts_lite`); **§6** original
   prose and assets ("Original does not mean lavish. Simple original SVG
   components are better than risky imitation."); **§7** AI-generated-asset
   review checklist; **§10** "If it ships to an unauthorized browser, it has
   shipped."; **§12** the release checklist rows.
5. **`docs/ARCHITECTURE.md`** (§3 ownership table; §11 game-module shape —
   per-game `ui.rs` metadata, `games/<id>/docs/HOW-TO-PLAY.md` prose owned by
   the game, `apps/web` renders it) and **`docs/ROADMAP.md`** (§1 — the catalog
   must present **all 14 official games**; confirm the roster from the gate
   crosswalk). Load-bearing: tells you where catalog metadata lives vs. where
   the picker shell lives, and the full set of games a redesign must cover.
6. **`docs/AGENT-DISCIPLINE.md`** — **§2** output format (complete files or
   coherent complete sections, not diffs — applies to the eventual
   implementation tickets your design-direction doc anticipates); **§10** UI
   protocol; **§11** IP protocol.
7. **`brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md`** —
   the prioritization that *defines P2*. Read **§2 (candidates C5, C6), §4 P2,
   §5 (sequence row 2), §6 (amendment summary), §7 (assumptions)**. This is the
   source of truth for what P2 *is* and what it is *not*.
8. **`archive/specs/effect-animation-and-turn-orchestration.md`** — the
   just-shipped predecessor (P1, Done 2026-06-12). Read in full: it is the
   **canonical spec anatomy your design-direction doc and any spec skeleton must
   mirror** (§1 objective, §2 current state, §3 external grounding, §4
   scope/non-goals table, §5 foundation & boundary alignment audit, §6 committed
   design decisions D1–D10, §7 deliverables tree, §8 work breakdown with
   dependencies, §9 exit criteria, §10 acceptance evidence, §11 forbidden
   changes, §12 documentation updates, §13 sequencing, §14 assumptions). Its
   **§13** records P2 as the named successor.
9. **`archive/specs/action-consequence-and-match-context-shared-surfaces.md`**
   and **`archive/specs/card-and-action-presentation-shared-surfaces.md`** — the
   two specs that *deferred* this work to P2. Read the deferral language:
   action-consequence **§4.3** ("Picker visual redesign (catalog card
   art/layout) — Deferred … takes only the *functional* setup gaps") and
   **A9** (variant *display labels* shipped Rust-side; one-line variant
   *descriptions* deferred — "revisit if richer setup copy is wanted");
   card-and-action **§13** (successor: game-picker/match-setup presentation
   polish) and **§15** (O7 raw variant IDs deferred). These establish what
   functional groundwork already shipped (so you do **not** re-spec it).
10. **`specs/README.md`** — the living progress index and spec-authoring
    conventions (header format, authority-order header, audit-row adoption
    matrix, lift-ready amendments, index row `Planned → In progress → Done`).
    Tells you where the P2 non-gate UI-infra row lands and the exact 12-section
    spec template the eventual spec follows.

---

## 3. Settled intentions — DECISIONS ALREADY MADE (do not re-open)

These were locked in the authoring interview. Treat each as fixed. Where you
have design latitude it is called out explicitly; everything else is decided.

1. **The determination is fixed.** P2 (Catalog & setup visual redesign) is the
   correct next public work. In your deliverable, **confirm-and-document** this
   by citing the evidence — do not re-evaluate it open-endedly:
   - P1 (`effect-animation-and-turn-orchestration`) is **Done** (archived
     2026-06-12);
   - the mechanic-atlas promotion-debt register is **empty** (MECHANIC-ATLAS
     §10A);
   - no public mechanic-ladder gate remains (Gates 1–14 Done; Gate P is
     private/optional, FOUNDATIONS §1 priority 5 < priority 1);
   - P2 is the **twice-named successor** (card-and-action §13/§15;
     action-consequence §4.3/A9; effect-animation §13).
2. **Asset strategy = original SVG + per-game theme-token system.** Center the
   research and the design direction on a *project-authored geometric/abstract
   SVG iconography system* plus a *per-game color + shape theme-token* layer.
   **No figurative or AI-generated illustration.** No trade-dress proximity to
   any commercial title. This follows FOUNDATIONS §7, UI-INTERACTION §2, and
   IP-POLICY §6 ("simple original SVG components are better than risky
   imitation"). *Latitude:* the specific motifs, grid, palette, and token
   schema are yours to propose.
3. **Variant descriptions (C6) are IN scope.** Design a **new OPTIONAL typed
   field** — a one-line authored description per variant — that crosses the
   Rust→TS boundary: declared near each game's variant data
   (`games/*/src/variants.rs`), projected by the wasm bridge
   (`crates/wasm-api/src/lib.rs`, mirroring how variant `*_display_name` labels
   are already projected), and consumed in the TS catalog type
   (`apps/web/src/wasm/client.ts`, `GameVariantCatalogEntry`). It is **inert
   prose** governed by ENGINE-GAME-DATA-BOUNDARY §5/§6/§11 and IP-POLICY §4 — it
   MUST NOT contain selectors, conditionals, or anything behavior-like, and must
   be original/neutral. Specify the authoring contract (length, tone, what it
   may/may not say), the boundary projection shape, and how existing
   shape-asserting tests (`apps/web/scripts/smoke-ui.mjs`) must be extended.
4. **Styling architecture may upgrade to a design-token layer and/or CSS
   Modules — but NO full UI framework.** `apps/web` is React 19 + Vite with a
   single ~3600-line vanilla `styles.css` and no CSS framework. You MAY propose
   introducing design tokens (CSS custom properties) and/or CSS Modules if you
   justify it against maintainability and the redesign's needs. You MAY NOT
   propose Tailwind or any other UI framework, and MAY NOT propose a renderer
   change (React + inline SVG stays). Deliver a **recommendation with a required
   default** between "token layer over vanilla CSS" and "CSS Modules + tokens",
   with migration cost and risk.
5. **Scope = catalog/picker AND match-setup visual polish**, built on the
   functional groundwork already shipped by the predecessors: variant selector,
   whole-`.game-card` click target, display-name projection (raw-ID removal),
   and `ui.rs` seat/faction labels. Do **not** re-spec that functional work;
   redesign its *presentation* and fill the visual gaps (e.g. seat/faction
   identity is projected but not yet surfaced prominently in the setup header).
6. **Locked / no questions.** Produce the deliverables directly. Do not ask the
   user anything.

**Out of scope (do not pull in):** animation/turn-orchestration (P1, done); the
effect-log history redesign (folded into P1); mechanic-atlas changes; any
renderer/engine/Rust-behavior change beyond the optional typed description
field; any ADR (this redesign triggers none — show why under FOUNDATIONS §13).

---

## 4. Current code reality (verified at baseline — inspect, don't trust blindly)

Confirm each of these against the baseline before relying on it; they are the
seams the redesign touches. **Inspect these files** (you need not read them in
full unless cited in §2):

- **`apps/web/src/components/GamePicker.tsx`** — current catalog UI. Renders a
  `.game-list` grid of `.game-card`s; each card is plain text: a `.game-option`
  button (heading = `display_name`, a `gameSummary()` line = variant count /
  single-variant label, `.game-flags` badges = "Hidden info" / "{N} views") and
  a secondary `.rules-trigger` "How to Play" button. Whole-card click already
  works. **No per-game art exists.**
- **`apps/web/src/components/MatchSetup.tsx`** — current setup UI. Variant
  `<select>` (shown when >1 variant), `.seat-roles` grid mapping
  `ui.seat_labels` → label/actor pairs (falls back to "Player 1/2"), mode radio
  buttons. Faction labels exist in the data (`ui.faction_labels`) but are not
  surfaced here.
- **`apps/web/src/wasm/client.ts`** — TS catalog types: `GameCatalogEntry`
  (`game_id`, `display_name`, `rules_version`, `schema_version`, `variants?`,
  `viewer_modes?`, `hidden_information?`, `cooperative?`, `tags?`, `ui?`),
  `GameVariantCatalogEntry` (`id`, `label` — **add `description?`**),
  `GameCatalogUiMetadata` (`seat_labels?`, `faction_labels?`). `listGames()`
  calls the wasm export `rulepath_list_games()`.
- **`crates/wasm-api/src/lib.rs`** — the catalog projection. `list_games()`
  (~line 395) builds JSON per game via `variant_json(id, label)` (~line 376) and
  `variants_json(...)` (~line 384), with hardcoded per-game `format!` blocks
  (~lines 417–543). The new description threads through these helpers (extend to
  `variant_json(id, label, description: Option<&str>)`). A catalog-shape test
  lives around line 10588 (`new_ops_use_status_output_convention`).
- **`games/*/src/variants.rs`** — per-game `Variant` / `ScenarioVariant` /
  `VariantCatalog` structs with `id` + `display_name` (+ game params), parsed
  from TOML with unknown-key rejection. **No `description` field exists yet.**
- **`games/*/src/ui.rs`** — 13 of 14 games carry a `UiMetadata` struct
  (labels, theme/reduced-motion tokens; richer games add `seat_labels`,
  `faction_labels`, affordance templates). `race_to_n` has none. This is the
  established per-game presentation-metadata home (UI-INTERACTION §10A).
- **`crates/game-stdlib`** — exports only `board_space` (no UI helpers; do not
  propose promoting one — that disposition is closed, brainstorm P4).
- **Styling:** single `apps/web/src/styles.css` (~3600 lines), vanilla CSS,
  no framework. SVG is used only inline (e.g. the race_to_n counter) and a
  `favicon.svg`; **no per-game catalog art assets exist.**
- **Tests that constrain the redesign:** `apps/web/scripts/smoke-ui.mjs`
  (asserts catalog entries, variant `id`+`label` pairs, game flags) and
  `apps/web/scripts/smoke-effect-feedback.mjs` (effect feedback, sentinel-free).
  The redesign and the new field must keep both green and the new field needs a
  shape assertion.
- **Roster:** the catalog must present **all 14 official games** (confirm names
  and count from `docs/ROADMAP.md` §1; neutral IDs per IP-POLICY §5).

The roadmap-relevant counts above (13/14 `ui.rs`, 14 games, ~3600-line CSS) are
author-time observations — **re-verify against the baseline manifest and files**
and correct any drift in your report.

---

## 5. What to research (external, deep — this is the part to externalize)

Go broad and cite sources. Organize findings so each maps to a design decision
in deliverable B. Cover at least:

1. **Similar implementations — game catalogs / launchers / portals.** How do
   mature products present a browseable grid of many games and lead a visitor
   from "list of titles" into "configured match"? Study, compare, and extract
   transferable patterns (card anatomy, hierarchy, hover/focus, theming,
   setup-on-select): Board Game Arena, Yucata, Tabletopia, BoardGameCore/other
   abstract-game sites, Steam / GOG Galaxy / Epic launchers, itch.io, and the
   **Game UI Database** (gameuidatabase.com) as a pattern reference. For each,
   note what *transfers* to a restrained, original, no-trade-dress portfolio
   site and what does **not** (casino/SaaS/proprietary patterns to avoid per
   FOUNDATIONS §7).
2. **Research papers / HCI evidence.** Ground the design in literature:
   the **aesthetic-usability effect** (first-impression and perceived quality);
   **choice / information overload** and how to keep a ~14-item grid scannable
   (Hick's law, categorization, default/recommended framing); **visual
   hierarchy and scannability** (F/Z patterns, fixed aspect ratios, layout
   consistency — irregular cards cause semirandom scanning); **first-impression
   / 50ms judgment** studies (Lindgaard et al. and successors); **icon
   comprehension and abstract iconography** legibility. Cite primary sources
   where possible.
3. **Web / visual design systems.** Design-token architecture (semantic vs.
   primitive tokens, theming via CSS custom properties), per-game theming
   strategies that stay coherent across 14 identities, card-grid responsive
   patterns, hover/focus/active/empty/loading states, motion restraint, and
   typography for a "cozy premium table" feel. Address the
   token-layer-over-vanilla-CSS vs. CSS-Modules decision (decision 4) with
   concrete trade-offs for a React 19 + Vite app with no current framework.
4. **Original SVG iconography systems + accessibility.** How to build a
   coherent, original, abstract icon set for ~14 distinct games on a consistent
   grid (24px industry standard), distinguishable by **shape, not color alone**
   (FOUNDATIONS §7 / UI-INTERACTION §2 / WCAG 1.4.1, 1.4.11); accessible-SVG
   technique (`role`, `<title>`, `aria-hidden` for decorative, focus-visible,
   high-zoom legibility, smallest-display-size legibility per IP-POLICY §6);
   theming SVG via `currentColor`/CSS variables for light/dark.
5. **Variant-description authoring patterns.** Short evidence on writing
   one-line variant/mode descriptors that aid choice without overload, and the
   constraint that they stay inert, original, neutral prose (no rule procedure,
   no trademark). Recommend a length/tone contract.
6. **IP-safe abstraction.** Concrete guidance on representing well-known game
   *families* (a 4-in-a-row, a checkers-like, a trick-taker, a poker-like) with
   original abstract iconography that avoids trade-dress and trademark proximity
   (IP-POLICY §2/§5/§6). Flag motifs to avoid.

Throughout, keep recommendations compatible with §3's locked decisions. Where
the evidence would push past a locked decision (e.g. toward illustration or a
framework), note it as a flagged future option — do not adopt it.

---

## 6. Deliverables — produce BOTH as downloadable markdown

### A. `catalog-setup-visual-redesign-research-report.md`
A cited deep-research report covering §5 (1–6). Requirements:
- Every non-obvious claim carries an inline source (URL or bibliographic ref);
  end with a reference list.
- A comparison table of the studied catalogs/launchers with the
  transferable-vs-avoid column.
- An explicit "what the evidence says for Rulepath" synthesis per subsection,
  written against the locked decisions in §3.
- A short section confirming-and-documenting the P2 determination (§3.1) with
  the cited evidence — framed as confirmation, not re-decision.

### B. `catalog-setup-design-direction.md`
A concrete, implementable design-direction proposal. Requirements:
- **Design tokens:** a proposed primitive + semantic token set (color, surface,
  depth, radius, spacing, typography) realizing the "cozy premium table" feel,
  expressed as CSS custom properties; plus the **styling-architecture
  recommendation** (decision 4) with a required default and migration/risk.
- **Card anatomy:** annotated structure for the redesigned catalog card (slots,
  hierarchy, the SVG icon, theme accent, flags, How-to-Play affordance,
  whole-card click), with a fixed aspect/consistency rule and responsive grid
  breakpoints.
- **Per-game theming approach:** how 14 games get distinct-yet-coherent
  identities via tokens + original abstract SVG, distinguishable by shape and
  color; an icon-system spec (grid, stroke, motif rules, IP-safe abstractions
  per game family) and an annotated reference gallery / mood description
  (described originals — **not** copied or linked proprietary art).
- **State design:** hover / focus-visible / active / selected / empty / loading,
  meeting WCAG and the §17 accessibility baseline; keyboard traversal of the
  grid.
- **Match-setup polish:** redesigned setup using the existing variant selector,
  seat/faction labels surfaced prominently, and the new variant descriptions;
  the How-to-Play surface reachable from picker and setup (UI-INTERACTION §17).
- **Variant-description field design (decision 3):** the authoring contract
  (length/tone/what-it-may-say), the boundary projection shape
  (`variants.rs` → `wasm-api` `variant_json`/`variants_json` →
  `GameVariantCatalogEntry.description?`), inertness/IP constraints, and the
  `smoke-ui.mjs` assertion to add.
- **Mapping to the eventual spec:** a section structured to drop into the
  Rulepath spec anatomy (mirroring the P1 spec's §1–§14 — objective, scope &
  non-goals table, foundation & boundary alignment audit, committed design
  decisions D1..n, deliverables tree, work breakdown, exit criteria, forbidden
  changes, documentation-updates, sequencing, assumptions), including the
  brainstorm §6 amendment note that P2 adds **per-asset IP-check rows to its own
  closeout** rather than amending `templates/PUBLIC-RELEASE-CHECKLIST.md`, and a
  statement that P2 needs **no ADR and no FOUNDATIONS change**.

Keep both deliverables consistent with the law in §2 and the decisions in §3.

---

## 7. Acceptance checks for your own output (self-verify before finishing)

- [ ] Every §2 doc was read and is reflected; quotes/section refs are accurate
      to the baseline commit.
- [ ] The P2 determination is **confirmed-and-documented** with cited evidence,
      not re-decided.
- [ ] All four locked decisions in §3 are honored: original SVG + theme tokens
      (no AI/illustration), variant descriptions in scope as an optional typed
      inert field, token/CSS-Modules upgrade only (no UI framework, no renderer
      change), catalog + setup scope.
- [ ] No proposal violates the boundary: TS decides no legality; the new field
      is inert prose with no forbidden field names; no hidden-information leak
      into payloads/DOM/test-IDs; static data stays typed content only.
- [ ] No trade-dress / proprietary-mimicry; all proposed assets are original and
      neutral-named; smallest-display-size legibility addressed.
- [ ] The redesign covers all 14 official games and keeps `smoke-ui.mjs` /
      `smoke-effect-feedback.mjs` green (and adds a shape assertion for the new
      field).
- [ ] You show the redesign needs **no ADR** (FOUNDATIONS §13) and **no
      FOUNDATIONS amendment**.
- [ ] Both deliverables are produced as downloadable markdown with sources.

---

## 8. Assumptions carried into this brief (correctable, but treat as decided)

- **(A1)** The catalog must present all 14 official games at baseline; confirm
  the exact roster from `docs/ROADMAP.md` §1 and correct if the count drifted.
- **(A2)** The variant-description field is *optional* at every layer (Rust
  struct, projection, TS type) so games without one render unchanged.
- **(A3)** Per-game `ui.rs` (UI-INTERACTION §10A) is the right home for any new
  per-game *presentation* metadata the icon/theme system needs; do not route it
  through `game-stdlib` or the mechanic atlas.
- **(A4)** "Setup visual redesign" reuses the already-shipped functional setup
  (variant selector, whole-card click, label projection) and redesigns its
  presentation only — no functional setup rework.
