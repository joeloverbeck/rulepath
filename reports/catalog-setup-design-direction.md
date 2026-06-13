# Catalog & setup visual redesign — design direction for Rulepath P2

**Target repository:** `joeloverbeck/rulepath`  
**Target commit:** `f8c94e6017084e212276395dff582c9e93242445`  
**Freshness claim:** user-supplied target commit only; this design direction does **not** independently verify latest `main`.  
**Deliverable B:** concrete implementation direction.

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/rulepath`.

This direction is constrained by the exact-commit repository evidence listed in `catalog-setup-visual-redesign-research-report.md`. The important baseline facts are: `apps/web` owns picker/setup presentation only; Rust owns behavior; `GamePicker.tsx` is currently a text-first card grid; `MatchSetup.tsx` already has variant select, seat labels, modes, and How-to-Play but does not prominently surface faction labels; `GameVariantCatalogEntry` is currently `id + label`; `crates/wasm-api` projects variants as `id + label`; the app uses React 19 + Vite + a single vanilla CSS file; 13 of 14 games have `ui.rs`; `race_to_n` does not; and no variant has `description` yet.

## 1. Product stance

P2 should make the catalog and setup feel like a polished public board-game table, not a debug index. The redesign stays pure presentation except for one optional typed inert field, `GameVariantCatalogEntry.description?`, projected from Rust variant data into TypeScript catalog data. It must not add legality decisions, hidden-information payloads, behavior selectors, rule procedure, bot logic, renderer changes, a UI framework, AI illustration, or trade-dress-adjacent assets.

The practical goal is: **a visitor should understand the collection, pick a game, choose a variant, understand who plays what, and reach How-to-Play without ever seeing raw IDs or engine/debug vocabulary.**

## 2. Styling architecture recommendation

### Required default: token layer over vanilla CSS

Use a CSS custom-property design-token layer inside `apps/web/src/styles.css` as the required default. Do not introduce Tailwind, Chakra, MUI, Radix-as-framework, Bootstrap, or any other full UI framework. Do not replace React + inline SVG.

Why this is the right default:

- The current app already uses one global vanilla stylesheet and no CSS framework.
- The redesign needs coherent theming more than component encapsulation.
- CSS custom properties give scoped per-game theming directly on `.game-card` and setup header elements.
- A token layer is a low-risk migration: it can be added above existing selectors and adopted incrementally.
- CSS Modules are useful only if catalog/setup split into many nested components and class collisions become real; they are not required for P2 and would add migration cost before the design vocabulary is stable.

### CSS Modules policy

CSS Modules may be introduced later for newly extracted component families, but only if the implementation spec records a local containment reason. A reasonable phase-2 threshold is: catalog/setup styles exceed a coherent section boundary, duplicated selectors start leaking across board surfaces, or card/setup components become reusable across multiple routes. Even then, tokens remain global CSS custom properties.

## 3. Design tokens

The token model has two layers:

1. **Primitive tokens**: raw color, spacing, radius, elevation, typography scale.
2. **Semantic tokens**: UI meaning, mapped to primitives and overridable per game.

The exact color values below are a starting palette, not a law. Before ship, every foreground/background and focus/non-text pair must be checked against WCAG text and non-text contrast requirements.

```css
:root {
  /* Primitive color: warm table */
  --rp-color-parchment-50: #fff8ea;
  --rp-color-parchment-100: #f7ead2;
  --rp-color-parchment-200: #ead6b6;
  --rp-color-walnut-900: #21160f;
  --rp-color-walnut-800: #352318;
  --rp-color-walnut-700: #4b3324;
  --rp-color-felt-800: #203d35;
  --rp-color-felt-700: #2c5648;
  --rp-color-brass-500: #b8873b;
  --rp-color-brass-300: #dfbd75;
  --rp-color-ink-950: #16110c;
  --rp-color-ink-800: #2f271f;
  --rp-color-ink-650: #5e5144;
  --rp-color-ink-500: #7c6f62;
  --rp-color-cream-0: #fffdf8;
  --rp-color-danger-600: #9a3f32;

  /* Primitive spacing */
  --rp-space-0: 0;
  --rp-space-1: 0.25rem;
  --rp-space-2: 0.5rem;
  --rp-space-3: 0.75rem;
  --rp-space-4: 1rem;
  --rp-space-5: 1.25rem;
  --rp-space-6: 1.5rem;
  --rp-space-8: 2rem;
  --rp-space-10: 2.5rem;
  --rp-space-12: 3rem;

  /* Primitive radius */
  --rp-radius-xs: 0.375rem;
  --rp-radius-sm: 0.625rem;
  --rp-radius-md: 0.875rem;
  --rp-radius-lg: 1.25rem;
  --rp-radius-xl: 1.75rem;
  --rp-radius-pill: 999px;

  /* Primitive depth */
  --rp-shadow-1: 0 1px 2px rgb(33 22 15 / 0.18), 0 6px 18px rgb(33 22 15 / 0.08);
  --rp-shadow-2: 0 3px 8px rgb(33 22 15 / 0.18), 0 18px 36px rgb(33 22 15 / 0.12);
  --rp-shadow-inset-soft: inset 0 1px 0 rgb(255 255 255 / 0.48);

  /* Primitive typography: system fonts only; no font asset risk */
  --rp-font-ui: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --rp-font-display: ui-serif, Georgia, Cambria, "Times New Roman", serif;
  --rp-text-xs: 0.75rem;
  --rp-text-sm: 0.875rem;
  --rp-text-md: 1rem;
  --rp-text-lg: 1.125rem;
  --rp-text-xl: 1.35rem;
  --rp-text-2xl: clamp(1.5rem, 2vw, 2.125rem);
  --rp-line-tight: 1.15;
  --rp-line-normal: 1.45;

  /* Semantic surfaces */
  --rp-page-bg: radial-gradient(circle at top left, #fff4dc 0, #f0dcc0 30rem, #d5b990 100%);
  --rp-surface-table: var(--rp-color-parchment-100);
  --rp-surface-card: color-mix(in oklab, var(--rp-color-cream-0) 88%, var(--rp-color-parchment-100));
  --rp-surface-card-hover: var(--rp-color-cream-0);
  --rp-surface-recessed: color-mix(in oklab, var(--rp-color-parchment-100) 82%, var(--rp-color-walnut-700));
  --rp-border-soft: rgb(75 51 36 / 0.18);
  --rp-border-strong: rgb(75 51 36 / 0.36);
  --rp-text-primary: var(--rp-color-ink-950);
  --rp-text-secondary: var(--rp-color-ink-650);
  --rp-text-muted: var(--rp-color-ink-500);
  --rp-focus-ring: #294d9b;
  --rp-danger: var(--rp-color-danger-600);

  /* Semantic game defaults; overridden per game */
  --game-accent: var(--rp-color-brass-500);
  --game-accent-2: var(--rp-color-felt-700);
  --game-accent-contrast: var(--rp-color-cream-0);
  --game-card-art-bg: color-mix(in oklab, var(--game-accent) 12%, var(--rp-color-cream-0));
  --game-card-art-line: var(--game-accent);
  --game-card-pattern-opacity: 0.16;

  /* Interaction */
  --rp-transition-fast: 140ms ease;
  --rp-transition-medium: 210ms ease;
}

@media (prefers-reduced-motion: reduce) {
  :root {
    --rp-transition-fast: 0ms;
    --rp-transition-medium: 0ms;
  }
}
```

### Per-game token examples

These can start in CSS keyed by `data-game-id`, then move to Rust-projected typed UI metadata when the implementation threads `catalog_theme` through `ui.rs` and the WASM catalog.

```css
.game-card[data-game-id="race_to_n"] {
  --game-accent: #a36a2a;
  --game-accent-2: #365f4b;
}
.game-card[data-game-id="column_four"] {
  --game-accent: #2f6f8f;
  --game-accent-2: #b8873b;
}
.game-card[data-game-id="poker_lite"] {
  --game-accent: #6e3f74;
  --game-accent-2: #8a6134;
}
.game-card[data-game-id="flood_watch"] {
  --game-accent: #2f6f8f;
  --game-accent-2: #4f7d62;
}
```

Do not rely on these colors alone. The icon silhouette and card accent shape must remain distinct in monochrome.

## 4. Catalog card anatomy

Use a single, repeated anatomy for all 14 games.

```text
article.game-card[data-game-id]
├─ div.game-card__accent-rail        decorative theme rail/pattern, not label
├─ button.game-card__primary          primary whole-card selection target
│  ├─ div.game-card__art              fixed art well
│  │  └─ GameCatalogIcon              original inline SVG, decorative if title visible
│  ├─ div.game-card__body
│  │  ├─ p.game-card__eyebrow         e.g. "Abstract duel" / "Cooperative" / "Hidden info"
│  │  ├─ h3.game-card__title          display_name only, never raw ID
│  │  ├─ p.game-card__summary         variant count or selected variant description
│  │  └─ ul.game-card__flags          Hidden info, N views, Coop, etc.
│  └─ span.game-card__selected-mark   only when selected; shape + text, not color alone
└─ button.game-card__rules            secondary How-to-Play affordance
```

### Interaction structure

Avoid nested buttons. The simplest implementation is to keep the current pattern: the outer `.game-card` handles whole-card click and ignores clicks originating inside `.game-card__rules` or other controls, while `.game-card__primary` remains an explicit button for keyboard users. If the implementation changes semantics, preserve two separate actions: select game and open How-to-Play.

### Hierarchy

- **Art well:** first visual anchor; 72–96px icon at desktop, 56–72px at narrow widths.
- **Eyebrow:** optional, derived from existing safe catalog metadata only: “Cooperative,” “Hidden info,” “Two seats,” “Multi-variant.” Avoid mechanics claims that are not already typed public metadata.
- **Title:** `display_name`; never raw ID.
- **Summary:** for single-variant games, the existing summary can remain “Single setup” or use the optional variant description once present. For multi-variant games, show “2 variants” / “3 variants” plus the selected/default variant label; the detailed description appears in setup.
- **Flags:** compact chips with icon shape + text. Do not encode hidden state beyond existing public catalog booleans.
- **Rules affordance:** stable secondary button labeled “How to Play”; reachable independently from card selection.

### Fixed aspect and responsive grid

Use a consistent card ratio and clamp copy to protect scanning.

```css
.game-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 17.5rem), 1fr));
  gap: var(--rp-space-5);
  align-items: stretch;
}

.game-card {
  position: relative;
  min-height: 18rem;
  aspect-ratio: 4 / 3;
}

.game-card__summary {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@media (max-width: 42rem) {
  .game-card {
    aspect-ratio: auto;
    min-height: 15.5rem;
  }
}
```

The desktop grid should generally settle into 3–4 columns. With 14 games, that yields a readable portfolio wall rather than a dense launcher. At small widths, use one column and preserve the same slot order.

## 5. Original SVG icon system

### System spec

- **Source format:** inline React SVG components, no external image files for P2.
- **Base grid:** `viewBox="0 0 24 24"` for the canonical icon; render at 72/96px in card art using vector scaling.
- **Stroke:** 1.75–2px, rounded joins/caps only if used consistently.
- **Fill:** limited to 1–2 filled primitive shapes; prefer stroke plus one accent fill.
- **Theme:** use `currentColor` and CSS variables such as `--game-card-art-line` and `--game-accent-2`.
- **Accessibility:** if the icon sits next to the visible title, set `aria-hidden="true"` and do not add redundant titles. If used standalone in a compact picker or selected badge, use `role="img"` plus a short `<title>` from typed `a11y_label`.
- **Legibility:** must be recognizable at 24px, 48px, and 72px, in monochrome, high contrast, and 200% zoom.
- **No AI and no figurative illustration:** every SVG is project-authored geometry.

### Inert metadata home

Add or extend local game UI metadata, not `game-stdlib` and not mechanic atlas:

```rust
pub struct CatalogThemeMetadata {
    pub icon_id: &'static str,
    pub theme_key: &'static str,
    pub accent_token: &'static str,
    pub secondary_accent_token: &'static str,
    pub shape_token: &'static str,
    pub a11y_label: &'static str,
}
```

This metadata is inert. It selects presentation tokens and icon IDs only. It must not contain legality, selectors, hidden identities, action availability, rule branches, or behavior-by-naming. `race_to_n` may get a minimal `src/ui.rs` because it is the only official game without one at the target commit; that is a local presentation-metadata addition, not a shared primitive promotion.

### Annotated reference gallery / original motif direction

These are descriptions of original motifs to author, not references to copy.

| Game | Original abstract motif | Shape distinction | IP/trade-dress cautions |
|---|---|---|---|
| `race_to_n` | Ascending step path with three pips and a terminal marker. | Stair-step silhouette. | Do not resemble a racing board, track logo, or branded counter. |
| `three_marks` | Three equal marks on a triangular micro-grid with a quiet crossing line. | Triad/delta silhouette. | Avoid exact tic-tac-toe board emphasis; title already differentiates. |
| `column_four` | Four neutral tokens aligned in a vertical channel with one offset guide line. | Four-token column. | Avoid red/yellow discs and plastic Connect-Four-style grid. |
| `directional_flip` | Two opposing crescent arcs around a center dot with small direction ticks. | Rotational flip silhouette. | Avoid black/white Othello/Reversi board trade dress. |
| `draughts_lite` | Diagonal chevrons with a stacked ring and small promotion notch. | Diagonal + stack. | Avoid full checkerboard, copied crown, or national draughts iconography. |
| `high_card_duel` | Two facing card slabs with rank bars, no suits. | Duel slabs. | Avoid proprietary card backs, suits as primary identity, or casino styling. |
| `token_bazaar` | Three different token shapes in a tray with a small exchange bend. | Mixed token silhouettes. | Avoid money/coin/market-trademark imagery. |
| `secret_draft` | Two veiled tiles sliding toward a shared reveal seam. | Veil + split seam. | Do not expose hidden choices; avoid envelope/trademark spy motifs. |
| `poker_lite` | Shared center pool as abstract rounded slabs and a ledger tick. | Center-pool slab. | Avoid chips, green felt, suits, casino vocabulary, and poker-room cues. |
| `plain_tricks` | Three plain cards in sequence with one neutral lead marker. | Ordered card fan. | Avoid bridge/whist/suit branding or ornate playing-card art. |
| `masked_claims` | Offset tile under a semi-opaque veil with a claim mark. | Veil/offset tile. | Avoid theatrical mask clichés and hidden-identity leaks. |
| `flood_watch` | Waterline crossing levee blocks with two rain ticks. | Horizontal waterline. | Avoid disaster-stock illustration; keep abstract/cooperative. |
| `frontier_control` | Node network with two asymmetric banners and a route line. | Network + banners. | Avoid colonial/real-world flags, maps, or faction iconography. |
| `event_frontier` | Horizon path with an event burst above a neutral card slab. | Horizon + burst. | Avoid copied card art, fantasy-map tropes, or event-deck mimicry. |

## 6. State design

### Hover

Hover should suggest tactile lift, not flashy animation.

```css
.game-card {
  background: var(--rp-surface-card);
  border: 1px solid var(--rp-border-soft);
  border-radius: var(--rp-radius-xl);
  box-shadow: var(--rp-shadow-1), var(--rp-shadow-inset-soft);
  transition: transform var(--rp-transition-fast), box-shadow var(--rp-transition-fast), border-color var(--rp-transition-fast);
}

@media (hover: hover) {
  .game-card:hover {
    transform: translateY(-2px);
    border-color: var(--rp-border-strong);
    box-shadow: var(--rp-shadow-2), var(--rp-shadow-inset-soft);
  }
}

@media (prefers-reduced-motion: reduce) {
  .game-card:hover {
    transform: none;
  }
}
```

### Focus-visible

Focus must be obvious and not color-only. Use an outline, offset, and a persistent internal focus mark.

```css
.game-card__primary:focus-visible,
.game-card__rules:focus-visible,
.match-setup select:focus-visible,
.match-setup input[type="radio"]:focus-visible {
  outline: 3px solid var(--rp-focus-ring);
  outline-offset: 4px;
  box-shadow: 0 0 0 6px color-mix(in oklab, var(--rp-focus-ring) 18%, transparent);
}
```

### Active and selected

- Active press: reduce lift, deepen inset, keep focus visible.
- Selected: use a left/top accent rail plus a visible “Selected” chip or check shape. Do not use accent color alone.
- Disabled: avoid if possible in catalog; games should be playable. If a future state is unavailable, explain with public copy only.

### Empty, loading, and failure states

- Loading: warm skeleton cards with stable dimensions; no engine terms.
- Empty: “No catalog games are available in this build.” plus a retry/reload affordance if relevant. No stack traces.
- Failure: “The game catalog could not be loaded.” Provide a public support/retry message; keep debug details out of normal UI.

### Keyboard traversal

Use native document order first: each card’s primary selection button and How-to-Play button are tab stops. A custom `role="grid"` is not recommended unless all grid keyboard semantics are implemented. Optional enhancement: left/right/up/down arrow keys move focus among `.game-card__primary` buttons only, preserving normal Tab traversal to each card’s How-to-Play button. The implementation must not hide the secondary rules button from keyboard users.

## 7. Match-setup polish

### Setup page anatomy

```text
section.match-setup
├─ header.setup-hero[data-game-id]
│  ├─ GameCatalogIcon
│  ├─ h2 selectedGame.display_name
│  ├─ p.setup-hero__summary      public summary, no raw IDs
│  ├─ ul.setup-hero__flags       hidden info/co-op/views, if present
│  └─ button How to Play
├─ section.setup-variant
│  ├─ label/select or segmented cards for selectedGame.variants
│  └─ p.variant-description      selected variant description, if present
├─ section.setup-players
│  ├─ seat/faction chips
│  └─ actor assignment summary
├─ section.setup-mode
│  └─ existing human/bot mode radios, with public copy
└─ actions Start Match / Back to catalog
```

### Variant selector

Keep the existing functional selector. Presentation can improve as follows:

- For one variant: render a quiet non-interactive “Setup” strip with the label and optional description.
- For multiple variants: keep the native `<select>` for robustness, or add an accessible segmented/card list only if it stays in sync with the existing `variantId` state. Do not remove the native path without equal keyboard and screen-reader behavior.
- Display the selected variant’s optional description under the control. If absent, omit the paragraph entirely; do not generate fallback rules text.

### Seat and faction labels

Current setup already maps `ui.seat_labels` to label/actor pairs. P2 should add a “Players & roles” section that surfaces:

- `ui.seat_labels`: visible seat labels and actor assignment.
- `ui.faction_labels`: visible faction chips when present, with icon/shape token if available.
- Fallbacks: “Player 1” / “Player 2” only when Rust UI metadata is absent.

For asymmetric games, faction labels should appear near the setup header, not buried below mode controls. For cooperative games, role labels should be explicit but should not reveal hidden/private state. For normal public mode, avoid “Rust legal bot” or engine/debug phrasing; use user-facing copy such as “Solo against an automated opponent” or “Watch automated seats play.”

### How-to-Play surface

The same How-to-Play/Rules surface must be reachable from:

- each catalog card,
- the selected setup hero,
- in-play controls.

The surface renders authored HOW-TO-PLAY Markdown only. It must not interpret rules prose as legality, scoring, availability, visibility, or bot logic.

## 8. Variant-description field design

### Type and authoring contract

Add an optional description per variant. It is typed inert content, not behavior.

**Field:** `description?: string` in TypeScript; `Option<String>` or equivalent optional Rust field in variant structs.  
**Recommended length:** 55–95 characters.  
**Hard maximum:** 120 characters after trimming.  
**Format:** one line, no Markdown, no raw IDs, no trailing period requirement.  
**Tone:** neutral, original, human-facing, choice-supportive.  
**May say:** public tone, pressure, duration, broad scenario feel, public setup emphasis.  
**Must not say:** hidden information, exact rule procedure, conditionals, selectors, triggers, legality, action availability, scoring formulas, strategy advice, trademarks, copied rulebook prose, casino terms, or raw IDs.

Bad examples:

- “If the flood deck triggers Surge, then districts rise twice.”
- “selector: legal_moves where role=warden.”
- “Play poker with chips and casino-style betting.”
- “Use the Connect Four classic layout.”

Good examples:

- “A steady cooperative flood puzzle with clear role pressure.”
- “A tighter storm setup with fewer quiet turns.”
- “A denser frontier map that sharpens route control.”
- “A faster event mix with early public pressure.”

### Rust variant layer

Add `description` as optional inert content near the existing display name. Do not use behavior-like field names.

For TOML-backed multi-variant games, accept `<variant_prefix>_description` or a consistent `description` key inside each existing variant table, depending on the current parser shape. The implementation should prefer the least disruptive existing pattern. Update unknown-key rejection so only the new description key is allowed. Keep behavior-key rejection for names such as `when`, `if`, `then`, `selector`, `trigger`, `effect`, `action`, `legal`, and similar forbidden semantics.

Suggested helper contract:

```rust
fn parse_variant_description(raw: Option<&str>) -> Result<Option<String>, VariantError> {
    let Some(raw) = raw else { return Ok(None); };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if trimmed.chars().count() > 120 {
        return Err(VariantError::InvalidDescriptionLength);
    }
    if looks_behavior_like(trimmed) {
        return Err(VariantError::BehaviorLikeDescription);
    }
    Ok(Some(trimmed.to_owned()))
}
```

The exact error type should match each game’s existing variant parser style. This is a coherent-section sketch, not a required helper name.

### WASM projection

Extend the catalog helpers so the field is emitted only when present:

```rust
fn variant_json(id: &str, label: &str, description: Option<&str>) -> String {
    match description {
        Some(description) => format!(
            r#"{{"id":"{}","label":"{}","description":"{}"}}"#,
            escape_json(id),
            escape_json(label),
            escape_json(description),
        ),
        None => format!(
            r#"{{"id":"{}","label":"{}"}}"#,
            escape_json(id),
            escape_json(label),
        ),
    }
}
```

If the implementation uses `serde_json` values instead of string `format!`, even better. The required shape is the same: `description` is omitted when absent, never emitted as `null`, never generated in TypeScript, and never used as behavior.

### TypeScript catalog type

```ts
export type GameVariantCatalogEntry = {
  id: string;
  label: string;
  description?: string;
};
```

Rendering rule:

```ts
const selectedVariant = selectedGame.variants?.find((variant) => variant.id === variantId);
const description = selectedVariant?.description?.trim();
```

If `description` is absent or blank after defensive trimming, render nothing. Do not synthesize copy from raw ID. Do not parse the description for conditions, rules, tags, or layout decisions.

### Smoke assertion to add

Extend `apps/web/scripts/smoke-ui.mjs` with a shape assertion. The exact code should fit the current script style, but the assertion should cover these facts:

```js
function hasVariant(game, id, label, description) {
  const variant = game.variants?.find((entry) => entry.id === id);
  assert(variant, `${game.game_id} variant ${id} missing`);
  assert.equal(variant.label, label);
  assert.equal(typeof variant.id, "string");
  assert.equal(typeof variant.label, "string");
  if (description === undefined) {
    assert(!Object.prototype.hasOwnProperty.call(variant, "description"));
  } else {
    assert.equal(variant.description, description);
    assert.equal(typeof variant.description, "string");
    assert(variant.description.length > 0);
    assert(variant.description.length <= 120);
    assert(!/\b(if|when|then|selector|trigger|valid_if|legal|effect|action)\b/i.test(variant.description));
  }
}
```

Add at least one positive described-variant assertion once descriptions are authored, preferably in a multi-variant game such as `flood_watch_deluge`, `frontier_control_highlands`, or `event_frontier_hard_winter`. Add at least one negative assertion for a game intentionally left without descriptions to confirm optional omission.

## 9. Implementation scope and non-goals

| In scope | Out of scope |
|---|---|
| Catalog card visual redesign. | Animation/turn orchestration; P1 is done. |
| Match-setup visual polish. | Effect-log history redesign. |
| Original inline SVG icon set for 14 official games. | AI-generated or figurative illustration. |
| Per-game theme tokens and inert `ui.rs` metadata. | Shared `game-stdlib` UI helper or mechanic-atlas promotion. |
| Optional one-line variant descriptions from Rust to TS. | Any Rust behavior, legality, visibility, action, bot, replay, or scoring change. |
| CSS custom-property token layer over vanilla CSS. | Tailwind or any full UI framework. |
| Smoke shape assertion for `description?`. | Renderer replacement or route architecture rewrite. |
| How-to-Play reachable from picker and setup. | Copying proprietary rules, art, icons, fonts, screenshots, or trade dress. |

## 10. Mapping to the eventual Rulepath spec anatomy

This section is written to drop into the future P2 spec using the P1 anatomy.

### §1 Objective

Redesign the public catalog picker and match-setup surfaces so all 14 official games present as a coherent, polished, IP-safe board-game portfolio. Add optional typed one-line variant descriptions as inert Rust-authored content projected to TypeScript. Preserve Rust behavior authority and TypeScript presentation-only boundaries.

### §2 Current state

- `GamePicker.tsx` renders text-first cards: display name, variant summary, flags, and How-to-Play.
- Whole-card click already works.
- `MatchSetup.tsx` already supports variant selection, modes, seat labels, and How-to-Play.
- Faction labels exist in catalog UI metadata but are not surfaced prominently in setup.
- `GameVariantCatalogEntry` has `id` and `label` only.
- `crates/wasm-api` projects variant JSON as `id + label` only.
- Variants have `id` and `display_name`, no `description`.
- 13 of 14 games have `ui.rs`; `race_to_n` lacks one.
- `apps/web/src/styles.css` is a single vanilla stylesheet; no UI framework exists.

### §3 External grounding

- Aesthetic-usability and 50ms first-impression research justify treating the catalog as a product-quality first screen, not a utility list.
- Choice-overload and Hick’s-law evidence support a light 14-item grid with clear grouping, not a heavyweight filter/sidebar system.
- Scanning/grid research supports fixed card anatomy, fixed icon well, and stable metadata slots.
- Icon-comprehension and WCAG evidence require visible text labels, shape-plus-color distinction, and non-color-only states.
- Design-token practice and CSS custom properties support scoped per-game theming without framework adoption.

### §4 Scope and non-goals

Use the scope table in §9 above.

### §5 Foundation & boundary alignment audit

| Rulepath law | P2 alignment |
|---|---|
| Public playable polished site is priority 1. | Redesign improves the first public catalog/setup impression. |
| Rust owns behavior; TS owns picker/setup UI. | Rust supplies typed inert metadata; TS renders only. |
| Static data may include labels, descriptions, icon IDs, theme tokens. | Variant descriptions and theme metadata are typed inert content. |
| Static data must not contain selectors/triggers/behavior. | Description contract and parser/smoke checks reject behavior-like content. |
| UI must be cozy premium, original, readable, restrained. | Token system, fixed card anatomy, and original SVG deliver this. |
| Color plus shape, not color alone. | Every icon and state has shape/outline/text as well as accent color. |
| How-to-Play surface from picker/setup. | Card and setup hero both expose How-to-Play. |
| IP conservatism. | No copied art/prose/fonts/screenshots; per-asset closeout rows. |
| No hidden-information leaks. | Descriptions and theme metadata forbid hidden identities and behavior. |
| ADR triggers. | None triggered: no renderer/DSL/legality/visibility/replay/bot authority change. |

### §6 Committed design decisions

D1. Use a CSS custom-property token layer over vanilla CSS as the required default.  
D2. Do not introduce any full UI framework.  
D3. Use original project-authored inline SVG only; no AI-generated illustration and no figurative/proprietary art.  
D4. Give every official game a color-plus-shape catalog identity through tokens and original SVG.  
D5. Keep per-game presentation metadata local in `games/<id>/src/ui.rs`; add minimal `race_to_n/src/ui.rs` if needed.  
D6. Use a fixed-aspect responsive catalog card with stable slots and text clamping.  
D7. Preserve separate actions for selecting a game and opening How-to-Play.  
D8. Redesign setup around a selected-game hero, variant description, and prominent seat/faction labels.  
D9. Add optional typed variant descriptions through `variants.rs` → `crates/wasm-api` → `GameVariantCatalogEntry.description?`.  
D10. Omit `description` when absent; do not emit `null` and do not synthesize from raw IDs.  
D11. Extend `smoke-ui.mjs` to assert variant-description shape, length, optional omission, and no behavior-like prose.  
D12. Keep `smoke-effect-feedback.mjs` green and untouched except for incidental build compatibility.  
D13. Add per-asset IP-check rows to P2 closeout instead of amending `templates/PUBLIC-RELEASE-CHECKLIST.md`.  
D14. No ADR, no FOUNDATIONS change, no mechanic-atlas row, no `game-stdlib` UI helper.

### §7 Deliverables tree

```text
apps/web/src/components/
  GamePicker.tsx              # redesigned card anatomy, original icon slot, rules affordance preserved
  MatchSetup.tsx              # setup hero, variant description display, seat/faction polish
  GameCatalogIcon.tsx         # new inline SVG registry, presentation-only
apps/web/src/wasm/
  client.ts                   # GameVariantCatalogEntry.description?
apps/web/src/styles.css       # token layer + catalog/setup sections
apps/web/scripts/
  smoke-ui.mjs                # variant description shape assertions
crates/wasm-api/src/lib.rs    # variant_json/variants_json projection of optional description
games/<id>/src/variants.rs    # optional description field where variants are declared
games/<id>/data/variants.toml # descriptions for TOML-backed multi-variant games
games/<id>/src/ui.rs          # inert catalog theme/icon metadata; add race_to_n ui.rs if chosen
specs/...
  catalog-setup-visual-redesign.md
```

### §8 Work breakdown with dependencies

1. **Token foundation:** add primitive/semantic tokens and migrate catalog/setup selectors to them. No behavior dependency.
2. **SVG icon registry:** create `GameCatalogIcon.tsx`, author 14 original SVGs, add smallest-size visual checks.
3. **Per-game theme metadata:** add inert `catalog_theme` UI metadata, starting with CSS-only data attributes if needed, then Rust projection if implementation chooses to expose typed theme metadata in catalog JSON.
4. **Catalog card redesign:** update `GamePicker.tsx` anatomy while preserving display names, whole-card selection, and How-to-Play action.
5. **Variant-description Rust structs:** add optional description to variant structs/parsers; update unknown-key tests.
6. **WASM/TS projection:** extend `variant_json`/`variants_json`, `GameVariantCatalogEntry`, and defensive UI rendering.
7. **Setup polish:** add setup hero, selected variant description, and prominent seat/faction label section.
8. **Smoke and a11y checks:** extend `smoke-ui.mjs`; run smoke UI/effects/e2e; verify no raw IDs and no engine/debug normal-mode copy.
9. **IP closeout:** record every SVG/motif/theme row as original, neutral, non-proprietary, and smallest-size legible.

### §9 Exit criteria

- Catalog presents all 14 official games with display names and no raw IDs.
- Each game has a distinct original SVG identity distinguishable by shape and color.
- The card grid is responsive, fixed-slot, keyboard reachable, and visually consistent.
- How-to-Play is reachable from every catalog card and the setup surface.
- Setup surfaces selected game identity, variant descriptions when present, seat labels, and faction labels when present.
- `GameVariantCatalogEntry.description?` is optional at every layer and omitted when absent.
- No description contains hidden state, selectors, triggers, conditionals, raw IDs, copied prose, or proprietary terms.
- `smoke-ui.mjs`, `smoke-effect-feedback.mjs`, and relevant e2e smoke scripts remain green.
- Normal-mode UI has no engine/debug vocabulary.
- No ADR, FOUNDATIONS update, mechanic-atlas change, renderer change, or UI framework is introduced.

### §10 Acceptance evidence

The implementation closeout should include:

- Screenshot set: desktop catalog, mobile catalog, selected setup, multi-variant setup, focus-visible card, focus-visible setup control, high-contrast or contrast-check notes.
- Smoke output: `npm run smoke:ui`, `npm run smoke:effects`, and relevant e2e smoke commands.
- Variant-description catalog JSON sample proving present/absent optional behavior.
- IP asset table: one row per SVG motif with origin “project-authored,” no copied references, no AI generation, no proprietary/trade-dress proximity, smallest-size legibility checked.
- Boundary audit row proving TypeScript does not parse descriptions for behavior.

### §11 Forbidden changes

- No legality, action availability, state mutation, scoring, visibility, bot, replay, or engine behavior changes.
- No hidden information in catalog payloads, DOM, test IDs, rules surface, or logs.
- No renderer replacement.
- No Tailwind/full UI framework.
- No proprietary art, copied icons, copied prose, screenshots, scans, trademark-forward terms, or trade-dress mimicry.
- No AI-generated or figurative illustration.
- No shared `game-stdlib` UI helper.
- No mechanic-atlas promotion row.
- No ADR or FOUNDATIONS amendment.

### §12 Documentation updates

- Add the P2 spec row to `specs/README.md` as a non-gate UI-infrastructure spec, moving Planned → In progress → Done during lifecycle.
- Add/update game UI docs only if they document the inert catalog icon/theme metadata contract.
- Add P2 closeout asset-IP rows in the spec acceptance evidence.
- Do **not** amend `templates/PUBLIC-RELEASE-CHECKLIST.md`; the brainstorm note says P2 carries per-asset IP-check rows in its own closeout.

### §13 Sequencing

P2 follows P1. It does not block on Gate P because Gate P is private/optional and lower priority than polished public play. It also does not wait for mechanic-atlas work because the atlas promotion-debt register is empty and UI-INTERACTION §10A keeps repeated presentation metadata local unless a future threshold is met.

### §14 Assumptions

A1. The official catalog remains the 14-game roster from the target-commit roadmap.  
A2. `description` remains optional at every layer.  
A3. `ui.rs` remains the correct home for local per-game presentation metadata.  
A4. The setup redesign reuses existing setup function and does not re-spec variant selection or whole-card click.  
A5. All SVG icons are project-authored manually.  
A6. The app remains React + inline SVG + vanilla CSS tokens for P2.  
A7. Rules prose remains authored HOW-TO-PLAY content only and is not interpreted by UI.

## 11. Final recommendation

Ship P2 as a restrained, tokenized, original-SVG redesign with the optional variant-description field. The strongest implementation path is to first add the CSS token layer, then redesign card/setup anatomy, then thread `description?` through Rust/WASM/TS with smoke assertions, and finally close with IP/a11y evidence. Do not overbuild filters, do not import a framework, and do not make the SVGs look like board-game box art. The catalog should feel like a carefully arranged shelf of playable abstract games, not a storefront, not a casino, and not a debug console.
