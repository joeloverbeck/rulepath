# rules-display-shared-surface — Shared “How to Play / Rules” surface

- **Spec ID:** `rules-display-shared-surface`
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate.
- **Roadmap build gate:** None (non-gate UI-infrastructure spec; see §12.7 and §15).
- **Status:** Done
- **Date:** 2026-06-09
- **Owner:** joeloverbeck
- **Authority order:** `docs/FOUNDATIONS.md` → area docs (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/UI-INTERACTION.md`, `docs/WASM-CLIENT-BOUNDARY.md`, `docs/IP-POLICY.md`) → accepted ADRs → this spec.
- **Subordination:** This spec is subordinate to the foundation docs, area docs, accepted ADRs, and the official-game contract. It does not silently amend or weaken them.

> **Reader orientation — canonical section map.** This spec retains its original `§1`–`§18` numbering rather than the canonical Rulepath section set (see `specs/README.md` → "Spec format"). The canonical sections map onto this body as: **Objective** → §1; **Scope** (in / out / not allowed) → §2; **Deliverables** → §9.1 and §14; **Work breakdown** → §14 (RULSUR-001…006); **Exit criteria & Acceptance evidence** → §11; **FOUNDATIONS & boundary alignment** → §4; **Forbidden changes** → §14 ("Forbidden changes for every ticket"); **Documentation updates** → §12 and §13; **Sequencing / Roadmap placement** → §12.7 and §15; **Assumptions** → §A (end of spec).

> **Baseline note.** `docs/WASM-CLIENT-BOUNDARY.md` still carries historical Gate 3 / `race_to_n`-only framing, while `crates/wasm-api/src/lib.rs` exposes the full nine-game catalog. This spec treats that older gate wording as historical context, not current-catalog evidence; realigning that doc is documentation hygiene (see §12.6), not a blocker for this spec.

---

## 1. Objective

Build a single shared, cross-game **How to Play / Rules** surface in `apps/web` for every game in the Rulepath catalog. The surface renders authored, version-controlled player-facing rules prose. It uses no LLM, no runtime text generation, no behavior DSL, no legality decisions in TypeScript, and no game-state-derived hidden information.

The feature closes the current gap between:

- formal per-game `RULES.md` documents, which are implementation-facing rule contracts with stable rule IDs, validation notes, coverage tables, and Rust-owned behavior language; and
- the public UI, which needs a readable player aid that teaches goal, turn flow, available actions, visibility, scoring, and win/loss conditions without duplicating strategy guides.

The spec also establishes a repeatable per-game authoring contract so every catalog game ships with a player-safe, IP-safe `HOW-TO-PLAY.md` document.

---

## 2. Goal, scope, and non-goals

### 2.1 Goal

Provide players with a reliable, accessible way to answer “How do I play this game?” from three places:

1. **Game picker:** before a game is selected.
2. **Match setup:** after a game is selected but before the match starts.
3. **In play:** during a live match, without leaving or mutating the match.

The surface must work uniformly for all nine catalog games in the current Rulepath catalog:

| Game ID | Display name | Hidden information? | New required player doc |
|---|---:|---:|---|
| `race_to_n` | Race to 21 | No | `games/race_to_n/docs/HOW-TO-PLAY.md` |
| `three_marks` | Three Marks | No | `games/three_marks/docs/HOW-TO-PLAY.md` |
| `column_four` | Column Four | No | `games/column_four/docs/HOW-TO-PLAY.md` |
| `directional_flip` | Directional Flip | No | `games/directional_flip/docs/HOW-TO-PLAY.md` |
| `draughts_lite` | Draughts Lite | No | `games/draughts_lite/docs/HOW-TO-PLAY.md` |
| `high_card_duel` | High Card Duel | Yes | `games/high_card_duel/docs/HOW-TO-PLAY.md` |
| `token_bazaar` | Token Bazaar | No | `games/token_bazaar/docs/HOW-TO-PLAY.md` |
| `secret_draft` | Veiled Draft | Yes | `games/secret_draft/docs/HOW-TO-PLAY.md` |
| `poker_lite` | Crest Ledger | Yes | `games/poker_lite/docs/HOW-TO-PLAY.md` |

`poker_lite` / **Crest Ledger** is the fully drafted pilot in this spec.

### 2.2 Scope

In scope:

- A shared `apps/web` rules surface component and open/close state.
- Access points from `GamePicker`, `MatchSetup`, and in-play shell/action area.
- A new per-game `HOW-TO-PLAY.md` content contract under each game’s `docs/` directory.
- Static build-copy delivery of those docs into the web app.
- A CI coverage and staleness guard ensuring every catalog game has an up-to-date player rules doc.
- A UI smoke test and accessibility/no-leak checks for the rules surface.
- Foundation/area/template amendment list needed to make the obligation permanent.
- A new `templates/GAME-HOW-TO-PLAY.md` template body included in this spec.

### 2.3 Non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Strategy guide replacement | Not applicable | `COMPETENT-PLAYER.md` remains the strategy/competent-play document. `HOW-TO-PLAY.md` teaches rules and turn flow only. |
| Rendering `games/<id>/docs/RULES.md` directly | Explicitly rejected | `RULES.md` is formal, rule-ID-driven, implementation-facing, and contains validation/visibility implementation notes that are not suitable player copy. |
| Tutorial mode, guided replay, or playable walkthrough | Not in core scope | Those can be future UX improvements, but this spec is a static player-rules surface. |
| Runtime text generation or LLM explanations | Forbidden | The surface renders authored, version-controlled content only. |
| TypeScript legality, scoring, effects, bot, or validation logic | Forbidden | Rust/WASM remains the only behavior authority. |
| Changes to replay, RNG, serialization, trace shape, hashes, or engine determinism | Forbidden | The surface is inert presentation. |
| Adding game nouns to `engine-core` | Forbidden | This feature does not touch kernel mechanics. |
| YAML or behavior DSL | Forbidden | The player docs are Markdown prose plus inert visible metadata only; no selectors, triggers, branches, conditions, action schemas, or behavior-like data. |
| Hidden-information inspection | Forbidden | The rules surface never reads private match state and never interpolates viewer-specific secrets. |
| IP-derived rulebook copying or proprietary assets | Forbidden | All prose is original Rulepath prose using neutral names. |
| Hosted accounts, multiplayer infrastructure, or cloud docs service | Not applicable | Static app delivery remains local/browser-only. |

---

## 3. Committed decisions

### D1 — Build one shared rules surface in `apps/web`

The implementation adds one reusable web surface, not nine bespoke help widgets. Game-specific content varies only by catalog `game_id` and authored Markdown file.

### D2 — Add a new per-game player document named `HOW-TO-PLAY.md`

Chosen filename: **`HOW-TO-PLAY.md`**.

Each official catalog game must have:

```text
games/<game_id>/docs/HOW-TO-PLAY.md
```

The document is player-facing original prose. It is distinct from:

- `RULES.md`, which remains the formal rule contract and Rust validation authority; and
- `COMPETENT-PLAYER.md`, which remains strategy guidance and bot-quality evidence.

The web surface renders `HOW-TO-PLAY.md`, never formal `RULES.md` directly.

### D3 — Use static-bundled Markdown as the default delivery mechanism

Recommendation: copy the authored per-game Markdown docs into `apps/web/public/rules/<game_id>.md` during web build, then have TypeScript fetch and render the selected file as inert presentation content.

This is the best fit for the current boundary doctrine because the content is help text, not behavior. It avoids expanding `wasm-api` for static prose, preserves the Rust/WASM boundary for game operations, and can be guarded by deterministic CI checks.

### D4 — Use a dedicated rules drawer / sheet, not tooltips as the primary surface

The primary UI is a dedicated rules panel:

- desktop/tablet: a right-side drawer or split-panel style surface;
- small screens: an accessible modal sheet/dialog;
- all sizes: close button, heading, table of contents, section landmarks, and keyboard/focus handling.

Tooltips are allowed only as optional glossary affordances after the core surface exists. Tooltips must never be the only way to learn the rules.

### D5 — Access is required before and during play

Rules must be reachable from:

- every game card or row in `GamePicker`;
- selected-game details in `MatchSetup`; and
- an in-play “Rules” / “How to Play” button near the app shell or action area.

### D6 — The feature is catalog-complete

No catalog game may ship without a matching player rules doc and rendered web asset. A catalog/docs sync check must fail CI when a new game is added without `HOW-TO-PLAY.md`.

---

## 4. Foundation and boundary alignment

| Authority | Constraint engaged | Spec alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §2 | Rust owns behavior; TypeScript presents only; static files cannot define rule behavior. | The rules surface renders inert prose. It never decides legality, scoring, effects, visibility, bot behavior, or replay semantics. |
| `docs/FOUNDATIONS.md` §7 | Public UI is a central product, not a demo shell. | The surface makes rules discoverable from picker, setup, and in-play. |
| `docs/FOUNDATIONS.md` §11 | Universal invariants: deterministic, no hidden leaks, Rust authority, no copied IP. | No runtime generation; no match-state interpolation; original prose; no trace/RNG/hash interaction. |
| `docs/FOUNDATIONS.md` §12 | Stop conditions before behavior/visibility/IP divergence. | Any attempt to include behavior selectors, copied rulebook prose, hidden state, or TS legality is a stop condition. |
| `docs/FOUNDATIONS.md` §13 | ADR triggers for DSL/schema/platform divergence. | No new behavior DSL, YAML, kernel change, replay change, or wasm operation is introduced, so no ADR is required for the chosen static path. |
| `docs/ARCHITECTURE.md` §2 | `apps/web` reaches games through `wasm-api`; game behavior remains Rust-owned. | The app fetches static docs by catalog ID only. Runtime game operations still use `wasm-api`; no game behavior crosses through static docs. |
| `docs/ARCHITECTURE.md` §3 | Ownership split. | Games own authored docs; `apps/web` owns presentation shell, panel, focus, and layout; scripts own sync checks. |
| `docs/ARCHITECTURE.md` §10 | WASM API shape. | Chosen design does not add a wasm operation; rejected Rust/WASM-served option is documented below. |
| `docs/ARCHITECTURE.md` §11 | Per-game docs folder convention. | Adds `HOW-TO-PLAY.md` as a required player-facing doc beside existing formal docs. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5 | Static data may include help text/localization/UI metadata; cannot define rule behavior. | Player rules text is allowed static presentation content; CI forbids behavior-looking structured content. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` §11 | UI metadata boundary. | Text labels, summaries, and glossary terms are presentation. They must not encode legality, consequences, or action filters. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` §12 | Explanation-template boundary. | `HOW-TO-PLAY.md` is not an effect explanation template and is not keyed to runtime consequences. |
| `docs/OFFICIAL-GAME-CONTRACT.md` | Official docs, original prose, UI exposure. | This spec adds `HOW-TO-PLAY.md` as a new official-player-doc obligation while keeping `RULES.md` authoritative. |
| `docs/UI-INTERACTION.md` | Public UI, ownership split, browser payload safety, hidden-info safety, accessibility baseline. | Rules are accessible from public UI surfaces, rendered safely, and never include private state in DOM/storage/logs/test IDs. |
| `docs/WASM-CLIENT-BOUNDARY.md` §2 | Current operation groups. | No new operation is required for static-bundled text. A future `get_rules` would require a boundary-doc amendment. |
| `docs/IP-POLICY.md` | No copied public rulebook prose/assets; neutral names. | Every player doc must be original Rulepath wording and avoid proprietary assets/trade dress. |
| `docs/AGENT-DISCIPLINE.md` | Bounded task packet, forbidden changes, failing-test protocol. | Ticket candidates below include explicit forbidden changes and test requirements. |

---

## 5. Content model and new per-game player-rules-doc contract

### 5.1 Required file

Each official game must include:

```text
games/<game_id>/docs/HOW-TO-PLAY.md
```

The document must be written for a player who has opened the web app and wants to know what to do, what information is visible, what actions mean, and how the game ends.

### 5.2 Required sections

Every `HOW-TO-PLAY.md` must contain these sections in this order unless a section is explicitly marked `Not applicable` with one sentence explaining why:

1. `# <Display Name> — How to Play`
2. **At a glance** — 3–6 bullets explaining goal, turn shape, and win condition.
3. **What you can see** — public information, private information from the player’s own perspective, and information no one sees in the browser.
4. **Setup** — player-facing setup summary, not implementation fixtures.
5. **On your turn** — the turn or phase sequence in ordinary language.
6. **Actions** — player-facing meaning of each action label that can appear in the UI.
7. **Scoring and winning** — how points, pool, victory, draw, or split outcomes work.
8. **Hidden information and reveal timing** — required for hidden-info games; otherwise explicitly `Not applicable — this is a perfect-information game.`
9. **Common terms** — short glossary for labels that appear in the UI.
10. **What this page is not** — short statement that it is not strategy advice and not the formal rules contract.
11. **Source notes for maintainers** — visible or comment-style inert metadata identifying the formal `RULES.md` source and rules version checked.

Recommended inert metadata format:

```markdown
_Formal rules source: `games/<game_id>/docs/RULES.md`_  
_Formal rules version checked: `<rules-version-from-RULES.md>`_  
_Strategy guide: `games/<game_id>/docs/COMPETENT-PLAYER.md`_
```

The build/runtime must not interpret these lines as behavior. CI may read them only to check coverage and staleness.

### 5.3 Required prose qualities

The prose must be:

- **Original:** no copied public rulebook text, no proprietary examples, no copied diagrams, no trade dress.
- **Neutral:** use Rulepath’s game IDs, display names, and original component names.
- **Player-facing:** explain what the user sees and does, not how Rust validates it.
- **Short enough to scan:** use headings, bullets, and tables for recognition rather than asking players to remember everything from a long wall of text.[^nng-recognition]
- **Progressively disclosed:** put “At a glance” first and deeper edge cases lower down or in collapsible sections.[^nng-progressive]
- **Hidden-info-safe:** describe rules from the player’s own perspective and public perspective; never reveal opponent private state, deck tails, unrevealed commitments, seed-derived data, or match-specific hidden values.
- **Strategy-neutral:** do not advise optimal play, bluffing patterns, card counting, bot weaknesses, or competent-player heuristics. Strategy belongs in `COMPETENT-PLAYER.md`.

### 5.4 Forbidden content

A `HOW-TO-PLAY.md` must not contain:

- rule IDs as the main teaching structure;
- Rust-owned validation notes;
- action-tree schemas;
- UI selector logic;
- YAML front matter;
- condition/action/trigger tables that could be read as a behavior DSL;
- JSON/TOML snippets intended for runtime behavior;
- fixture data or seed-specific examples;
- copied third-party rulebook prose;
- proprietary assets, logos, fonts, board layouts, or trade dress;
- hidden match-state examples that reveal private order, deck tail, unrevealed opponent data, or seed-derived secrets;
- strategy advice.

### 5.5 Relationship to existing docs

| Document | Role after this spec | Rendered in web rules surface? |
|---|---|---:|
| `RULES.md` | Formal implementation contract: stable rule IDs, invariants, legality, visibility, validation, coverage. | No. It is source material only. |
| `HOW-TO-PLAY.md` | Player-facing original prose: how to play, what actions mean, how scoring/winning works, visibility safety. | Yes. This is the rendered document. |
| `COMPETENT-PLAYER.md` | Strategy guide and competent-player/bot evidence. | No. May be referenced only as “strategy guide,” not merged into player rules. |
| `UI.md` | Product-facing UI contract for the game. | No. The shared surface may be documented there, but player prose comes from `HOW-TO-PLAY.md`. |
| `SOURCES.md` | Source/IP notes. | No. Maintainers use it to verify original/neutral prose. |

---

## 6. Delivery mechanism

### 6.1 Recommendation: static-bundled Markdown

Use a build step to copy each source document:

```text
games/<game_id>/docs/HOW-TO-PLAY.md
```

to a generated web asset:

```text
apps/web/public/rules/<game_id>.md
```

The web client then fetches:

```text
<base-url>/rules/<game_id>.md
```

and renders it in the shared rules panel.

Vite serves files under the project `public` directory at the root during dev and copies them to the build output as-is during production build, which matches this static-asset use case.[^vite-public]

### 6.2 Why static-bundled is the right default

| Criterion | Static-bundled Markdown | Rust/WASM-served `get_rules` | Decision |
|---|---|---|---|
| Source of truth | `games/<id>/docs/HOW-TO-PLAY.md` remains the authored source. | Same source can be embedded with Rust `include_str!` or generated Rust constants. | Tie. |
| Boundary fit | Treats rules prose as inert UI/help text, explicitly allowed static content if it carries no behavior. | Makes Rust/WASM carry presentation prose even though no game operation is needed. | Static wins. |
| TypeScript authority risk | TS fetches and renders text only; CI forbids behavior-like content. | TS calls a viewer-safe operation and renders text. | Tie if guarded. |
| WASM payload | No increase. | Increases WASM payload by embedding nine docs and future localizations. | Static wins. |
| WASM API churn | No new operation group. | Adds `get_rules` or catalog metadata extension; requires boundary docs and tests. | Static wins now. |
| CI enforceability | Strong: parse catalog, check source docs, copy targets, hashes, version tags. | Strong: Rust compile can fail if docs missing, but still needs prose staleness checks. | Tie. |
| Browser deployment | Simple: Vite public asset copy. | Single wasm artifact but larger and less cache-friendly for text-only changes. | Static wins. |
| Future localization | Can extend static paths later: `/rules/<locale>/<id>.md`, still inert. | Could centralize locale lookup in Rust, but that becomes a broader product decision. | Static wins until localization becomes a product requirement. |

**Default:** static-bundled Markdown.

**Rejected for now:** Rust/WASM-served rules text. It is defensible if the project later wants all viewer-safe catalog payloads to originate from a Rust-generated manifest, or if localizations, offline packaging, and version pinning become hard to guard through static assets. At this commit, adding a new `wasm-api` operation for inert prose is unnecessary surface-area growth.

### 6.3 Static delivery design

Add a build/sync script:

```text
scripts/copy-player-rules.mjs
```

Responsibilities:

1. Parse catalog game IDs from `crates/wasm-api/src/lib.rs` using the same mechanical-source pattern as `scripts/check-catalog-docs.mjs`.
2. For each catalog `game_id`, read `games/<game_id>/docs/HOW-TO-PLAY.md`.
3. Fail if the source doc is missing, empty, lacks required headings, lacks `Formal rules version checked`, or contains forbidden raw HTML/script tags.
4. Copy the Markdown to `apps/web/public/rules/<game_id>.md`.
5. Write an optional generated manifest such as `apps/web/public/rules/manifest.json` containing only inert coverage metadata:
   - `game_id`
   - `display_name`
   - `rules_asset_path`
   - `source_rules_version_checked`
   - content hash
6. Never include action schemas, legal filters, rule conditions, selectors, triggers, seed data, or match state.

Add a check script:

```text
scripts/check-player-rules.mjs
```

Responsibilities:

1. Parse catalog game IDs and display names from `crates/wasm-api/src/lib.rs`.
2. Assert every catalog game has `games/<game_id>/docs/HOW-TO-PLAY.md`.
3. Assert every source doc declares the same game ID and display name as the catalog.
4. Extract the formal rules version from `games/<game_id>/docs/RULES.md`.
5. Assert `HOW-TO-PLAY.md` declares that same formal rules version as checked.
6. Assert required sections are present.
7. Assert hidden-info games have a `Hidden information and reveal timing` section that is not `Not applicable`.
8. Assert perfect-information games explicitly mark hidden info as not applicable.
9. Assert generated `apps/web/public/rules/<game_id>.md` is byte-identical to the source after running the copy step, or require the check to run the copy step in dry-run mode and fail on drift.
10. Assert no source or generated doc contains raw `<script>`, event-handler attributes, iframe/embed/object tags, or YAML front matter.

Add to `apps/web/package.json`:

```text
"build:rules": "node ../../scripts/copy-player-rules.mjs",
"check:rules": "node ../../scripts/check-player-rules.mjs"
```

and run `build:rules` before `vite build`.

### 6.4 Web runtime design

Add a presentation-only rules loader in `apps/web`.

Required behavior:

- It accepts only a `game_id` that already came from the loaded catalog.
- It rejects IDs that do not match the catalog or fail a conservative `^[a-z0-9_]+$` check.
- It fetches from the generated rules asset path, not from arbitrary user input.
- It stores only document loading state and rendered content state.
- It never calls `get_view`, `get_view_for_viewer`, `get_action_tree`, `get_effects`, replay APIs, bot APIs, or hidden-info views to populate the rules text.
- It never writes the rules content to replay export, effect log, match state, or local storage.
- It renders with raw HTML disabled or sanitized out.
- It uses stable semantic headings, not hidden match-data `data-testid` values.

Permitted shell state additions:

```text
rulesPanelOpen: boolean
rulesPanelGameId: string | null
rulesPanelStatus: idle | loading | loaded | error
rulesPanelMarkdown: string | null
```

These are presentation-only. They must not feed back into Rust actions or legality.

### 6.5 Rust/WASM-served alternative, documented but not chosen

If the team later chooses Rust/WASM delivery, the likely shape is:

```text
get_rules(game_id: string) -> { game_id, display_name, rules_version, markdown }
```

or a catalog extension:

```text
GameCatalogEntry.rules_asset = { kind: "wasm", operation: "get_rules" }
```

That path would require:

- `docs/WASM-CLIENT-BOUNDARY.md` §2 amendment adding a new viewer-safe operation group.
- `docs/ARCHITECTURE.md` §10 amendment listing the operation.
- wasm-api tests proving every catalog game has embedded player rules.
- payload-size review.
- hidden-info no-leak review proving `get_rules` never depends on viewer ID or match state.

This spec does not choose that path.

---

## 7. UI/UX design

### 7.1 Pattern recommendation

Use a dedicated rules drawer/sheet with progressive sections.

Why:

- Players should recognize the available help where they are making a choice, rather than recall that docs exist elsewhere.[^nng-recognition]
- A rules surface should expose the most common “what do I do?” answers first and defer edge cases into lower sections or disclosures.[^nng-progressive]
- The app already has distinct pre-play and in-play contexts; contextual help aligns with the general usability heuristic of making help available near the relevant task.[^nng-heuristics]
- Board/card-game learning commonly benefits from a short player aid separate from a complete rulebook; this maps cleanly onto `HOW-TO-PLAY.md` as player prose and `RULES.md` as the formal contract.[^tabletop-aid][^rulebook-accessibility]

### 7.2 Entry points

#### Game picker

In `GamePicker.tsx`, each game entry gets a secondary button:

```text
How to Play
```

Placement:

- near the existing display name / metadata area;
- visually secondary to game selection;
- keyboard focusable;
- with an accessible name such as `How to play Crest Ledger`.

Behavior:

- opens the rules panel for that card’s `game_id`;
- does not select the game unless the current component structure makes selection unavoidable, in which case selection is a harmless UI-only selection and must not start a match;
- does not create a match, seed, replay, or Rust state.

#### Match setup

In `MatchSetup.tsx`, the selected-game details area gets:

```text
How to Play / Rules
```

Placement:

- near selected game metadata and before the Start Match action;
- adjacent to short catalog descriptors if those are later added.

Behavior:

- opens the same rules panel;
- uses the selected catalog entry’s `game_id`;
- remains available while adjusting seed, play mode, and seats.

#### In play

In the in-play controls area — e.g. `ModeControls.tsx` / `ActionControls.tsx`, beside the existing match controls, reading the active `game_id` from shell state (`AppShell.tsx` is a minimal wrapper that does not itself receive `game_id`) — add a persistent button:

```text
Rules
```

Placement:

- near the match title / game heading / action controls;
- not inside a dev-only panel;
- present for all viewer modes.

Behavior:

- opens the same panel for the active match’s `game_id`;
- never changes the match;
- never switches viewer perspective;
- never reads hidden data.

### 7.3 Layout

| Viewport | Pattern | Requirements |
|---|---|---|
| Desktop / wide tablet | Right-side drawer or split panel | Board remains visible where possible; panel has its own scroll region; close button; focus enters panel on open and returns to trigger on close. |
| Narrow tablet / mobile | Modal sheet/dialog | Treat as modal; trap focus; close with explicit button and `Esc`; return focus to trigger; avoid offscreen focus. |
| Reduced-motion users | Same layout, no required animation | Any slide animation must respect reduced-motion settings. |
| Print / copy | Not core | Optional future player-aid output; not required here. |

### 7.4 Information architecture inside the panel

The rendered `HOW-TO-PLAY.md` should appear as a structured article:

1. Title and formal source/version note collapsed or de-emphasized.
2. “At a glance” always visible near the top.
3. Table of contents generated from headings.
4. Main sections in document order.
5. Optional accessible accordions for longer sections, using WAI-ARIA accordion/disclosure expectations if custom controls are implemented.[^wai-accordion]
6. Error state if the Markdown fails to load: “Rules are unavailable for this game,” with no stack trace or internal path leak.

Do not bury the whole rules surface behind hover-only tooltips. Hover-only help is poor for keyboard, touch, and screen-reader users, and it cannot carry the whole rule explanation. Tooltips may later supplement glossary terms, but the primary help is the rules article.

### 7.5 Accessibility requirements

The implementation must satisfy `docs/UI-INTERACTION.md` §16 and these concrete requirements:

- The entry controls are real `<button>` elements or equivalent accessible controls.
- Each trigger has a game-specific accessible name.
- The panel title is connected to the container via `aria-labelledby`.
- Modal mode follows WAI dialog focus containment: focus moves into the dialog, `Tab`/`Shift+Tab` remain inside while modal, `Esc` closes, and focus returns to the opener.[^wai-dialog]
- Drawer mode either behaves as a non-modal complementary region with sane focus order, or as a modal dialog; it must not create unreachable offscreen focus.
- Focus indicators are visible and meet the project accessibility baseline; WCAG 2.2 focus appearance guidance is the target for keyboard focus visibility.[^wcag-focus]
- The panel uses semantic headings and lists so screen-reader users can navigate the article.
- Text is not conveyed by color alone.
- Contrast, font size, and spacing follow the app baseline.
- Loading and error states are announced through an appropriate status region.
- The rules content is available by keyboard before match creation and during play.

### 7.6 Hidden-information safety in the UI

For hidden-information games, the panel must not be viewer-personalized with private match state. It may say “you can see your own private card/crest/commitment when the game view allows it,” but it must not name a player’s actual private card, secret draft choice, deck order, or hidden center card.

The rules panel must not add hidden information to:

- DOM text;
- `data-testid`;
- local/session storage;
- console logs;
- effect logs;
- replay export/import;
- dev inspector;
- screenshots generated by tests;
- analytics or telemetry, if those ever exist.

---

## 8. Per-game authoring plan

### 8.1 Authoring workflow for all nine games

For each game:

1. Read its formal `games/<id>/docs/RULES.md`.
2. Read `games/<id>/docs/UI.md` if available to match player-facing labels.
3. Read `games/<id>/docs/COMPETENT-PLAYER.md` only to avoid duplicating strategy.
4. Draft `games/<id>/docs/HOW-TO-PLAY.md` from scratch in original prose.
5. Verify all player-visible action labels are explained.
6. Verify all scoring/win/draw/terminal outcomes are explained.
7. For hidden-info games, write visibility from the player’s perspective and explicitly state what is never shown.
8. Add inert source/version note.
9. Run `node scripts/check-player-rules.mjs`.
10. Run the web rules smoke test.

### 8.2 Per-game authoring inventory

| Game | Required player-doc focus | Hidden-info authoring note |
|---|---|---|
| Race to 21 | Exact-count race: add 1–3, reach target exactly, invalid overshoot not player-selectable. | Not applicable — perfect information. |
| Three Marks | 3×3 placement, one mark per turn, line win, draw when full. | Not applicable — perfect information. |
| Column Four | Column choice, gravity/drop behavior, connect four horizontally/vertically/diagonally, full-column unavailable, draw. | Not applicable — perfect information. |
| Directional Flip | Place on legal empty cell, bracket opposing markers in directions, flip captured markers, forced pass, terminal on full board or consecutive passes. | Not applicable — perfect information. |
| Draughts Lite | Dark-cell movement, mandatory capture, multi-jump continuation, promotion, terminal on no pieces/no legal moves. | Not applicable — perfect information, but multi-step action tree must be explained without implementation internals. |
| High Card Duel | Private hand, simultaneous commit/reveal, higher rank scores, tie behavior, round count. | Never reveal opponent hand or deck order. Explain only player-owned private cards and public reveal timing. |
| Token Bazaar | Public resource collection, exchange, fulfill, market/contract/supply exhaustion, turn cap. | Not applicable — perfect information. |
| Veiled Draft | Visible item pool, secret choices, simultaneous reveal, contested pick fallback, scoring categories. | Never expose pending choices; explain that commitments stay hidden until reveal. |
| Crest Ledger | Two rounds of pledges, private crest, center reveal, yield, match/lift/hold/press, showdown comparison. | Pilot below. Never expose opponent crest, hidden center before reveal, or deck tail. |

### 8.3 Worked pilot: `games/poker_lite/docs/HOW-TO-PLAY.md`

The following is the required pilot text for `poker_lite` / **Crest Ledger**. It is intentionally player-facing and strategy-neutral.

```markdown
# Crest Ledger — How to Play

_Game ID: `poker_lite`_  
_Formal rules source: `games/poker_lite/docs/RULES.md`_  
_Formal rules version checked: `poker-lite-rules-v1`_  
_Strategy guide: `games/poker_lite/docs/COMPETENT-PLAYER.md`_

## At a glance

Crest Ledger is a two-player hidden-crest pledge game.

- You have one private crest. Your opponent cannot see it until a showdown happens.
- A center crest starts hidden. It is revealed only if the first pledge round closes without a yield.
- Players add pledge markers to a shared pool over up to two rounds.
- If a player yields while facing pressure, the other player wins the pool immediately and private crests stay hidden.
- If both rounds close without a yield, the private crests reveal together and the stronger crest result wins the pool.
- A private crest that matches the center crest’s rank makes a pair. A pair beats no pair.

## What you can see

You can always see public match information:

- whose turn it is;
- the current pledge round;
- the shared pool;
- each player’s public contribution;
- whether there is an outstanding pledge to answer;
- whether the round’s lift has already been used;
- the center crest after it has been revealed.

You can see your own private crest. You cannot see your opponent’s private crest unless the game reaches showdown.

No browser view shows the hidden deck tail. A yield result does not reveal either player’s private crest.

## Setup

Each match uses six crests: two low crests, two middle crests, and two high crests. The copies have different names, but copy names do not break ties.

At the start:

- each player receives one private crest;
- one center crest is set aside face down;
- each player contributes one marker to the pool;
- round 1 begins with player 1 active.

## On your turn

Your available actions depend on whether you are facing an outstanding pledge.

If there is no outstanding pledge, you may either keep the round calm or create pressure.

If you are facing an outstanding pledge, you must answer it by matching, lifting, or yielding.

The UI only offers legal actions for the current position.

## Actions

### Hold

Use **Hold** when there is no outstanding pledge. You add no markers.

If both players hold in the same round, that round closes.

### Press

Use **Press** when there is no outstanding pledge. You add the current round’s pledge unit to your contribution and ask your opponent to answer.

Round 1 uses one-marker pressure. Round 2 uses two-marker pressure.

### Match

Use **Match** when you are facing an outstanding pledge. You add enough markers to equal your opponent’s contribution, then the round closes.

### Lift

Use **Lift** when you are facing an outstanding pledge and the round’s lift has not been used. You match the outstanding pledge, add one more pledge unit, and send pressure back to your opponent.

Only one lift can be used in a round.

### Yield

Use **Yield** when you are facing an outstanding pledge and do not want to continue. The match ends immediately. Your opponent wins the pool, and private crests are not revealed.

## How rounds close

A round can close because both players held or because an outstanding pledge was matched.

If round 1 closes without a yield, the center crest is revealed and round 2 begins with player 2 active.

If round 2 closes without a yield, the game goes to showdown.

## Scoring and winning

There are two ways to win the pool:

1. **Yield win:** your opponent yields while facing your pressure. You win immediately.
2. **Showdown win:** both rounds close without a yield, then private crests reveal and the stronger result wins.

At showdown:

- first, check whether each private crest matches the center crest’s rank;
- a matching private crest makes a pair;
- any pair beats any non-pair;
- if both players have the same pair status, the higher private crest rank wins;
- if both players are still tied, the pool is split evenly.

Crest copy names do not break ties.

## Hidden information and reveal timing

Your private crest belongs to your view. Your opponent’s private crest stays hidden from you until showdown.

The center crest stays hidden during round 1. It is revealed only if round 1 closes without a yield.

Private crests reveal together only at showdown. If the match ends by yield, private crests stay hidden.

The hidden deck tail is never shown in the browser.

## Common terms

| Term | Meaning |
|---|---|
| Crest | The private or center item used to determine showdown strength. |
| Center crest | The shared crest that starts hidden and may later be revealed. |
| Pair | A private crest whose rank matches the center crest’s rank. |
| Pool | The shared markers awarded to the winner or split on a true tie. |
| Pledge round | One of the game’s two pressure rounds. |
| Outstanding pledge | A contribution lead that the other player must answer. |
| Lift | A one-per-round raise after matching pressure. |
| Yield | Ending the match immediately while giving the pool to the other player. |

## What this page is not

This page teaches the rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

For maintainers, the formal rule source is `games/poker_lite/docs/RULES.md`. Strategy and competent-play notes belong in `games/poker_lite/docs/COMPETENT-PLAYER.md`.
```

### 8.4 Pilot validation notes

The pilot text deliberately:

- does not copy any external poker or casino rules prose;
- uses Rulepath’s neutral/original `Crest Ledger` and crest vocabulary;
- explains `Hold`, `Press`, `Match`, `Lift`, and `Yield` in player terms;
- explains the two pledge rounds without exposing Rust implementation details;
- states all hidden-information reveal timing;
- avoids strategy advice such as when to press, lift, or yield;
- avoids deck-tail, seed, and opponent-private leaks;
- does not use formal rule IDs as the player-facing structure;
- deliberately omits the per-seat maximum contribution cap (a formal `RULES.md` rule): the UI only offers legal actions, so the cap never needs to be recalled from the player aid, and omitting it keeps "At a glance" scannable. Recorded here so a future editor does not treat the omission as a coverage gap.

---

## 9. Web implementation design

### 9.1 New or changed web modules

| Area | Change |
|---|---|
| `apps/web/src/components/RulesPanel.tsx` | New shared component for drawer/modal presentation, loading/error/content states, focus management hooks, and safe Markdown rendering. |
| `apps/web/src/components/GamePicker.tsx` | Add per-game “How to Play” secondary control. |
| `apps/web/src/components/MatchSetup.tsx` | Add selected-game “How to Play / Rules” control. |
| `apps/web/src/components/ModeControls.tsx` / `ActionControls.tsx` (in-play controls) | Add persistent in-play “Rules” control, reading the active `game_id` from shell state. `AppShell.tsx` is a minimal wrapper without `game_id`. |
| `apps/web/src/state/shellReducer.ts` | Add presentation-only rules panel state and actions. |
| `apps/web/src/wasm/client.ts` | No WASM operation change. Optionally add a helper type for static rules asset metadata; do not alter game operation semantics. |
| `apps/web/src/styles.css` | Add drawer/sheet, article typography, focus, status, and responsive rules styles. |
| `apps/web/package.json` | Add `build:rules`, `check:rules`, and e2e smoke script wiring. |

### 9.2 Markdown rendering constraints

The renderer must support only a safe presentation subset:

- headings;
- paragraphs;
- emphasis;
- lists;
- tables if already supported or implemented safely;
- inline code for game IDs/action labels;
- internal anchors/table of contents.

The renderer must reject, strip, or ignore:

- raw HTML;
- scripts;
- iframes, objects, embeds;
- event handler attributes;
- external media;
- style attributes;
- custom directives;
- any syntax interpreted as behavior.

If a dependency is added, it must be configured with raw HTML disabled. If a local renderer is written, it must remain a presentation parser and must not become a rules DSL.

**Default:** add a single vetted, permissively-licensed Markdown renderer configured with raw HTML disabled (raw HTML, `<script>`, iframe/embed/object, event-handler and `style` attributes stripped or ignored), in preference to a hand-rolled parser. The required subset includes tables and a heading-anchor table of contents, where a hand parser is error-prone and easy to drift into unsafe rendering. This is the only new runtime dependency the feature introduces; pin it, and keep the §10.1 forbidden-tag scan and §10.3 / §10.5 no-leak smoke as the enforcement net regardless of which renderer is chosen.

### 9.3 Error and loading states

Loading state:

```text
Loading rules…
```

Error state:

```text
Rules are unavailable for this game.
```

Do not display raw file-system paths, stack traces, fetch URLs with query strings, or internal exception dumps to players. Test logs may include enough detail for diagnosis, but must not include hidden match data.

### 9.4 State isolation

Rules UI state is separate from match state. Opening, closing, loading, failing to load, or scrolling the panel must not:

- apply an action;
- request a bot turn;
- alter active seat;
- alter viewer perspective;
- alter replay state;
- export/import a replay;
- add effect-log entries;
- alter RNG or seeds;
- change serialization order;
- change any Rust data structure.

---

## 10. Testing and CI

### 10.1 Catalog-to-player-rules coverage check

Add:

```text
scripts/check-player-rules.mjs
```

It should follow the mechanical style of `scripts/check-catalog-docs.mjs`: parse known catalog declarations from `crates/wasm-api/src/lib.rs`, then verify docs and generated assets.

Minimum assertions:

| Assertion | Failure example |
|---|---|
| Every catalog game has `games/<id>/docs/HOW-TO-PLAY.md`. | New game added to `list_games()` without player rules. |
| Every source doc has required sections. | Missing “Scoring and winning”. |
| Hidden-info games have real hidden-info/reveal text. | `secret_draft` says hidden info is not applicable. |
| Perfect-info games explicitly mark hidden info not applicable. | `column_four` silently omits visibility section. |
| Formal rules version checked matches `RULES.md`. | `RULES.md` updated but `HOW-TO-PLAY.md` still cites old version. |
| Generated web asset matches source. | `apps/web/public/rules/poker_lite.md` stale after editing source. |
| No raw HTML/script/iframe/embed/object/event handler attributes. | A doc contains `<script>` or `<iframe>`. |
| No YAML front matter. | A doc starts with `---`. |
| No forbidden behavior-keywords as structured headings. | A doc introduces “Triggers”, “Selectors”, or “Conditions” tables as runtime-like data. Human review still required; this is a guardrail, not semantic proof. |

The script should print a concise list of failing game IDs and missing/invalid sections.

### 10.2 Build-copy check

Add:

```text
scripts/copy-player-rules.mjs
```

CI should run:

```text
node scripts/copy-player-rules.mjs
node scripts/check-player-rules.mjs
```

The check must fail if generated `apps/web/public/rules/*.md` files are not current with source docs.

### 10.3 Web UI smoke test

Add:

```text
apps/web/e2e/rules-display.smoke.mjs
```

Minimum coverage:

1. Load the app.
2. Confirm every game card exposes a keyboard-focusable “How to Play” control.
3. Open rules from the game picker for at least one perfect-info game and for `poker_lite`.
4. Confirm the panel heading matches the game display name.
5. Confirm “At a glance”, “Actions”, and “Scoring and winning” are present.
6. Close the panel and confirm focus returns to the trigger.
7. Select `poker_lite`, open rules from `MatchSetup`, close successfully.
8. Start a `poker_lite` match, open rules in play, close successfully.
9. Confirm opening rules does not change active legal actions or match view JSON.
10. Confirm no console errors.

### 10.4 Accessibility smoke test

Add accessibility checks to the same smoke or a dedicated no-leak/a11y smoke:

- trigger buttons have accessible names;
- modal mode uses `role="dialog"` or equivalent semantics when modal;
- panel title is associated with the panel;
- `Esc` closes modal/sheet mode;
- keyboard focus does not escape modal mode;
- visible focus remains clear;
- status/error messages are accessible;
- no hover-only path to rules.

### 10.5 Hidden-information no-leak test

For hidden-info games, especially `poker_lite`, `secret_draft`, and `high_card_duel`:

1. Start a seeded match.
2. Capture public/observer view before opening rules.
3. Open rules.
4. Assert the rules panel contains static authored text only.
5. Assert it does not include runtime private IDs, opponent private card/crest labels from the current match, hidden deck order, unrevealed choices, or seed-derived hidden values.
6. Assert replay export before and after opening rules is identical.
7. Assert effect log before and after opening rules is identical.

### 10.6 Existing gates must not be weakened

Do not remove, skip, narrow, or weaken existing tests. If an existing test fails while implementing this feature, follow `docs/AGENT-DISCIPLINE.md` failing-test protocol:

1. determine whether the failing test is still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue without deleting coverage to get green.

### 10.7 Suggested verification commands

```text
node scripts/check-catalog-docs.mjs
node scripts/copy-player-rules.mjs
node scripts/check-player-rules.mjs
npm --prefix apps/web run build
npm --prefix apps/web run smoke:e2e
npm --prefix apps/web run smoke:ui
cargo test --workspace
```

`cargo test --workspace` is expected to remain green because this spec should not require Rust behavior changes. If the static path is implemented without Rust edits, the cargo run is regression evidence, not because the feature belongs in Rust.

---

## 11. Acceptance and exit criteria

| Criterion | Acceptance evidence |
|---|---|
| All nine catalog games have `HOW-TO-PLAY.md`. | `scripts/check-player-rules.mjs` passes and lists nine games. |
| `poker_lite` pilot text exists and matches the worked pilot’s intent. | `games/poker_lite/docs/HOW-TO-PLAY.md` includes the drafted Crest Ledger player prose or a faithful edited version preserving all rules/safety points. |
| Web renders the player docs, not formal `RULES.md`. | UI smoke confirms `At a glance` and player sections; no rule-ID validation tables appear. |
| Rules accessible from picker, setup, and in-play. | E2E smoke opens and closes from all three surfaces. |
| Static delivery is deterministic and current. | Build-copy and check scripts pass; generated assets match source docs. |
| No TypeScript legality or behavior drift. | Code review confirms no action legality/scoring/effects/replay logic added to rules UI; before/after match JSON unchanged when opening rules. |
| No hidden-information leak. | Hidden-info smoke passes for `poker_lite`, `secret_draft`, and `high_card_duel`. |
| Accessibility baseline met. | Keyboard/focus/dialog smoke passes; no hover-only access. |
| IP-safe player prose. | Docs review confirms original prose, neutral names, no copied external rulebook text/assets. |
| Existing gates remain intact. | Existing web smoke and cargo tests are not weakened or deleted. |
| Documentation contract is permanent. | `docs/**` and `templates/**` amendments land with this spec or in the associated docs ticket. |

Exit requires all rows above to be satisfied or explicitly marked blocked by an upstream contradiction requiring an ADR. No ADR is expected for the chosen static path.

---

## 12. `docs/**` amendment list

These amendments are part of the implementation plan. They are not separate specs.

### 12.1 `docs/OFFICIAL-GAME-CONTRACT.md`

Add a required official-player-doc subsection near the existing official game documentation requirements:

```markdown
### Player-facing rules document

Every official catalog game MUST include `games/<game_id>/docs/HOW-TO-PLAY.md`.

`HOW-TO-PLAY.md` is original Rulepath prose for players. It teaches the goal, setup summary, turn flow, action meanings, scoring/winning, and hidden-information/reveal timing. It is the only per-game rules prose intended for the shared web How to Play / Rules surface.

`RULES.md` remains the formal rule contract and Rust validation authority. The web app MUST NOT render `RULES.md` directly as player help.

`HOW-TO-PLAY.md` MUST NOT duplicate `COMPETENT-PLAYER.md` strategy guidance.

Hidden-information games MUST describe visibility from the player's own perspective and public perspective without exposing opponent secrets, deck tails, unrevealed commitments, or seed-derived hidden data.
```

Also add `HOW-TO-PLAY.md` to the official docs checklist and to the UI-exposure expectations.

### 12.2 `docs/UI-INTERACTION.md`

Add a public-UI rules affordance requirement:

```markdown
The public web UI MUST expose a shared How to Play / Rules surface for every catalog game. The surface must be reachable from the game picker, match setup, and in-play. It renders authored player-facing `HOW-TO-PLAY.md` content only.

The surface is presentation-only: it must not decide legality, inspect private state, write replay/effect data, or generate runtime rules text.

The surface must meet the accessibility baseline: keyboard access, visible focus, accessible names, focus management for modal/sheet mode, and screen-reader navigable headings.
```

Add the no-hover-only rule for primary rules access.

### 12.3 `docs/ENGINE-GAME-DATA-BOUNDARY.md`

Formalize authored player help as allowed static UI content:

```markdown
Authored player-facing rules/help text, such as `games/<game_id>/docs/HOW-TO-PLAY.md`, is allowed static presentation content when it is inert prose. It may include labels, glossary text, setup summaries, and player-facing explanations.

It MUST NOT include selectors, conditions, triggers, action schemas, validation rules, scoring logic, visibility filters, YAML front matter, or any DSL-like structure that could become behavior authority.

Runtime legality, effects, visibility, scoring, and replay semantics remain Rust-owned.
```

### 12.4 `docs/IP-POLICY.md`

Add `HOW-TO-PLAY.md` to public rules documentation requirements:

```markdown
Player-facing `HOW-TO-PLAY.md` files are public documentation. They MUST use original Rulepath prose, neutral names, and no copied rulebook text, examples, diagrams, art, logos, fonts, or trade dress. They may summarize the Rulepath implementation's rules but must not reproduce external protected expression.
```

### 12.5 `docs/ARCHITECTURE.md`

Add `HOW-TO-PLAY.md` to the game module docs-folder convention and ownership table:

```markdown
Games own authored player-facing rules prose in `games/<game_id>/docs/HOW-TO-PLAY.md`.

`apps/web` owns the shared rules panel/drawer, static Markdown loading, rendering, accessibility, and responsive layout.

For the static-bundled path, `wasm-api` has no new operation; game behavior and runtime views continue to cross the JS boundary only through existing Rust/WASM operations.
```

If the rejected Rust/WASM-served path is later adopted, amend §10 with `get_rules` or its chosen operation name.

### 12.6 `docs/WASM-CLIENT-BOUNDARY.md`

For this static-bundled implementation: **no operation amendment required.** Add a clarifying note only if maintainers want to make the absence explicit:

```markdown
Player-facing `HOW-TO-PLAY.md` text is currently delivered as static web presentation content, not through a WASM operation. Adding a future `get_rules` operation would require updating this boundary document and proving the operation is viewer-safe and behavior-free.
```

Also consider updating the historical Gate 3 / `race_to_n`-only framing separately if this doc is meant to describe the full nine-game catalog. That cleanup is documentation hygiene, not a blocker for this spec.

### 12.7 `docs/ROADMAP.md`

No required roadmap ladder change. Optionally add one non-gate maintenance note:

```markdown
Cross-game public UI infrastructure specs, such as the shared How to Play / Rules surface, may be tracked outside the mechanic gate ladder. They do not introduce a new mechanic gate unless explicitly accepted as a roadmap item.
```

### 12.8 `docs/AGENT-DISCIPLINE.md`

No amendment required. Ticket packets for this work must follow the existing bounded-task, forbidden-changes, failing-test, UI, IP, and hidden-info protocols.

### 12.9 `specs/README.md`

Add this spec to the spec index/progress tracker. The index table schema is `| Stage | Gate | Spec | Status |`, and the documented status set is `Not started → Planned → In progress → Done` (there is no `Proposed` status). Since this is non-gate UI infrastructure, record it with no stage/gate and `Status: Planned`:

```markdown
| — | Non-gate (UI infra) | [`rules-display-shared-surface.md`](rules-display-shared-surface.md) | Planned |
```

If maintainers prefer to keep the gate-keyed ladder table strictly for mechanic gates, list this instead under a short "Non-gate UI-infrastructure specs" note in `specs/README.md`, using the same `Planned` status value.

---

## 13. `templates/**` decision

### 13.1 Decision: add a new template, do not amend `GAME-RULES.md` for player prose

Create:

```text
templates/GAME-HOW-TO-PLAY.md
```

Rationale:

- `templates/GAME-RULES.md` is formal and rule-ID-driven. Amending it to include player prose would blur the line between formal Rust-owned rule contracts and public help.
- `COMPETENT-PLAYER.md` is explicitly strategy-oriented. Player rules must not become strategy guidance.
- A separate template makes the obligation easy for CI and agents to follow.
- A separate template lets the web app render exactly one public-help document without scraping formal docs.

Update `templates/README.md` to list `GAME-HOW-TO-PLAY.md` as required for every official catalog game.

### 13.2 Proposed `templates/GAME-HOW-TO-PLAY.md` body

```markdown
# <Game Display Name> — How to Play

_Game ID: `<game_id>`_  
_Formal rules source: `games/<game_id>/docs/RULES.md`_  
_Formal rules version checked: `<rules-version>`_  
_Strategy guide: `games/<game_id>/docs/COMPETENT-PLAYER.md`_

## At a glance

Write 3–6 short bullets for a first-time player:

- What is the goal?
- What does a normal turn or round look like?
- What are players trying to collect, place, reveal, score, or avoid?
- How does the game end?
- What is the most important visibility rule, if any?

Do not include strategy advice.

## What you can see

Describe public information first.

For hidden-information games, describe only:

- what the player can see from their own perspective;
- what all players can see publicly;
- what remains hidden until reveal;
- what is never exposed in the browser.

For perfect-information games, write:

`Not applicable — this is a perfect-information game. All game state needed for play is public in the normal game view.`

## Setup

Explain setup in player terms. Do not paste fixtures, seeds, Rust structs, JSON, or validation tables.

## On your turn

Explain the turn or phase flow in ordinary language.

If turns can branch, explain what the player sees and chooses. Do not encode selectors, conditions, or legality rules as a behavior table.

## Actions

List the action labels that can appear in the UI and explain what each means to a player.

Use one subsection per action:

### <Action label>

Plain-language explanation.

## Scoring and winning

Explain scoring, victory, loss, draw, split, terminal, or exhaustion outcomes.

## Hidden information and reveal timing

Required for hidden-information games.

For perfect-information games, write:

`Not applicable — this is a perfect-information game.`

For hidden-information games, explain reveal timing without exposing hidden values.

## Common terms

| Term | Meaning |
|---|---|
| <Term> | <Player-facing meaning> |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and implementation details belong in the other game docs.

## Source notes for maintainers

Confirm before merging:

- [ ] Prose is original Rulepath wording.
- [ ] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [ ] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [ ] No hidden match-state examples or seed-specific data.
- [ ] No YAML front matter.
- [ ] No selectors, conditions, triggers, or action schemas.
- [ ] Formal rules version checked matches `RULES.md`.
```

### 13.3 `templates/README.md` amendment

Add an entry:

```markdown
- `GAME-HOW-TO-PLAY.md` — player-facing rules prose rendered by the shared web How to Play / Rules surface. Required for every official catalog game. Distinct from formal `GAME-RULES.md` and strategy-oriented `COMPETENT-PLAYER.md`.
```

---

## 14. Work breakdown as ticket candidates

Each ticket must be converted into `templates/AGENT-TASK.md` format before execution and must include the forbidden-changes declaration below.

### RULSUR-001 — Document contract and template

**Goal:** Amend official docs and templates to establish `HOW-TO-PLAY.md` as a required per-game player rules doc.

**Deliverables:**

- `templates/GAME-HOW-TO-PLAY.md`
- `templates/README.md` update
- `docs/OFFICIAL-GAME-CONTRACT.md` update
- `docs/UI-INTERACTION.md` update
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` update
- `docs/IP-POLICY.md` update
- `docs/ARCHITECTURE.md` update
- `specs/README.md` index update

**Tests/evidence:** docs link check, template checklist review.

### RULSUR-002 — Author player rules for all catalog games

**Goal:** Create `HOW-TO-PLAY.md` for all nine games, using `poker_lite` as pilot.

**Deliverables:**

- nine `games/<id>/docs/HOW-TO-PLAY.md` files
- `poker_lite` pilot text matching this spec’s worked example
- original-prose/IP review notes in PR description

**Tests/evidence:** `scripts/check-player-rules.mjs` once available; manual hidden-info review for `high_card_duel`, `secret_draft`, and `poker_lite`.

### RULSUR-003 — Static copy and CI guard

**Goal:** Implement static-bundled delivery scripts and CI coverage/staleness checks.

**Deliverables:**

- `scripts/copy-player-rules.mjs`
- `scripts/check-player-rules.mjs`
- generated `apps/web/public/rules/*.md`
- package script wiring
- CI workflow wiring if required

**Tests/evidence:** scripts fail on missing/stale/invalid docs; scripts pass with all nine docs.

### RULSUR-004 — Shared rules panel UI

**Goal:** Add the shared rules panel and integrate access points.

**Deliverables:**

- `RulesPanel` component
- rules loader
- shell state/actions
- `GamePicker` access point
- `MatchSetup` access point
- in-play access point
- styles

**Tests/evidence:** component/unit checks if available; manual keyboard pass; build passes.

### RULSUR-005 — E2E, accessibility, and no-leak smoke

**Goal:** Prove the surface works and does not mutate or leak match state.

**Deliverables:**

- `apps/web/e2e/rules-display.smoke.mjs`
- hidden-info no-leak checks for `poker_lite`, `secret_draft`, `high_card_duel`
- focus/keyboard checks

**Tests/evidence:** web smoke suite passes.

### RULSUR-006 — Closeout and maintenance docs

**Goal:** Ensure final docs, spec index, and acceptance evidence are complete.

**Deliverables:**

- completed acceptance checklist
- command output summary
- any final docs cleanup

**Tests/evidence:** all commands in §10.7 pass or are explicitly justified.

### Forbidden changes for every ticket

- Do not add TypeScript legality, scoring, validation, visibility, effects, bot, RNG, replay, or serialization logic.
- Do not alter `engine-core` nouns or kernel behavior.
- Do not add YAML or a behavior DSL.
- Do not render formal `RULES.md` directly as player help.
- Do not copy external rulebook prose or proprietary assets.
- Do not leak hidden information into DOM, test IDs, logs, storage, rules text, effect logs, replay exports, or dev tools.
- Do not weaken existing tests or smoke gates.
- Do not add a `wasm-api` operation unless the delivery decision is explicitly reopened through docs/spec amendment and, if needed, ADR review.

---

## 15. Roadmap note

This is cross-game UI infrastructure. It sits alongside the mechanic-ladder gates as a public-product improvement and maintenance contract. It is **not** a numbered gate and should not be treated as a required mechanic workstream unless the roadmap explicitly promotes it.

---

## 16. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Player docs drift from formal rules. | `Formal rules version checked` staleness guard; review docs with every formal rule change. |
| Static prose accidentally becomes behavior metadata. | No YAML/front matter; no selectors/triggers/conditions/action schemas; CI text guard plus review. |
| Hidden-info docs overexplain secrets. | Hidden-info section required; no seed examples; manual no-leak review for hidden-info games; panel never reads private match state. |
| Players confuse rules page with strategy guide. | “What this page is not” section; no strategy advice; `COMPETENT-PLAYER.md` remains separate. |
| Rules panel hurts keyboard/mobile accessibility. | Use WAI dialog/accordion patterns; E2E focus tests; visible focus baseline. |
| Public assets become stale after editing source docs. | Copy script and byte-identical drift check. |
| Raw Markdown rendering creates XSS risk. | Disable/strip raw HTML; test forbidden tags; no external media embeds. |
| New games forget player docs. | Catalog parser check fails when `list_games()` changes without matching `HOW-TO-PLAY.md`. |

---

## 17. Claude’s extra suggestions beyond the core ask

These are adjacent improvements, not part of core acceptance unless separately accepted.

### 17.1 Catalog short descriptions

Add a one-sentence inert `short_description` for every catalog game. It can improve game-picker comprehension before opening full rules. Keep it static presentation metadata only, with no legality or behavior content.

### 17.2 Game-local glossary anchors

After `HOW-TO-PLAY.md` exists for all games, allow glossary terms to generate local anchor links inside the rules panel. Optional hover/focus glossary popovers are acceptable only if they duplicate content reachable in the article and remain keyboard/screen-reader accessible.

### 17.3 First-run onboarding

A future first-run overlay could point players to “How to Play,” legal action controls, replay import/export, and viewer mode. It must be UI-only and should not be merged into this core rules-display feature.

### 17.4 Localization readiness

The static path can later become:

```text
apps/web/public/rules/<locale>/<game_id>.md
```

Do not start localization until the English contract is stable. Locale routing must remain presentation-only and must not affect behavior, rules versions, or replay.

### 17.5 Print-friendly player aid

A future script could generate print-friendly player aids from `HOW-TO-PLAY.md`. That would be useful for playtesting, but it should remain downstream of the canonical Markdown and avoid creating a second source of truth.

### 17.6 Rules version badge

The panel could show a small “Rules checked against `<rules-version>`” badge. This helps maintainers and testers, but it should be visually secondary for players.

---

## A. Assumptions

These assumptions were validated against the codebase during spec reassessment on 2026-06-09 and are recorded so decomposition does not re-investigate them.

- **Catalog source of truth.** The nine catalog `game_id`s and display names are declared as `const GAME_*` / `GAME_*_DISPLAY_NAME` pairs in `crates/wasm-api/src/lib.rs` and surfaced by `list_games()`. The new `copy-player-rules.mjs` / `check-player-rules.mjs` scripts parse them with the same regex `scripts/check-catalog-docs.mjs` uses (`/const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g`). Display names verified exact, including `secret_draft` → "Veiled Draft" and `poker_lite` → "Crest Ledger".
- **Rules-version tokens are per-game literal strings, not derivable from `game_id`.** Each `RULES.md` declares its version on a `Rules version: \`<token>\`` line, but the tokens mix forms — underscore (`race_to_n-rules-v1`, `three_marks-rules-v1`, `column_four-rules-v1`, `directional_flip-rules-v1`, `draughts_lite-rules-v1`) and hyphen (`high-card-duel-rules-v1`, `token-bazaar-rules-v1`, `secret-draft-rules-v1`, `poker-lite-rules-v1`). The staleness check (§6.3 steps 4–5, §10.1) MUST compare the `RULES.md` token literally against the `HOW-TO-PLAY.md` "Formal rules version checked" value; it MUST NOT reconstruct the token from `game_id`.
- **E2E harness already exists.** `apps/web/package.json` already defines `smoke:e2e`, chaining nine `.mjs` smoke files (including `e2e/a11y-noleak.smoke.mjs`). The new `e2e/rules-display.smoke.mjs` (§10.3) chains into that existing script and runner convention; only `build:rules` / `check:rules` (§6.3) are new package scripts.
- **No Markdown renderer is currently a dependency.** `apps/web` ships only react / react-dom / vite / typescript; the §9.2 renderer decision adds the sole new runtime dependency for this feature.
- **Per-game prerequisite docs exist.** All nine games already have `docs/RULES.md`, `docs/UI.md`, and `docs/SOURCES.md`; seven have `docs/COMPETENT-PLAYER.md` (`race_to_n` and `three_marks` do not). No game has `HOW-TO-PLAY.md` yet. Authoring (§8.1) may read `UI.md` / `COMPETENT-PLAYER.md` only where present.

## Closeout

Completed: 2026-06-09

Implementation summary:

- Added the permanent `HOW-TO-PLAY.md` contract and template, plus the copy/check scripts that generate and verify static web rules assets for all nine catalog games.
- Authored all nine catalog player-facing rules docs and generated `apps/web/public/rules/*.md` plus `manifest.json`.
- Added the shared `RulesPanel`, presentation-only shell state, picker/setup/in-play access points, focus containment, and visible focus treatment for rules triggers.
- Added `apps/web/e2e/rules-display.smoke.mjs` and chained it into `smoke:e2e` to cover rules access, content, a11y, no hidden-info leak, and no match mutation.
- Closed the capstone by flipping `specs/README.md` to `Done` and allowing the cross-game rules smoke in `scripts/check-catalog-docs.mjs`.

Acceptance evidence:

- `node scripts/check-catalog-docs.mjs` passed.
- `node scripts/copy-player-rules.mjs` copied 9 catalog games.
- `node scripts/check-player-rules.mjs` passed.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `cargo test --workspace` passed.
- `node scripts/check-doc-links.mjs` passed after the `Done` index flip.

---

## 18. References

### External UX, accessibility, and delivery references

[^nng-recognition]: Nielsen Norman Group, “Memory Recognition and Recall in User Interfaces,” 2024. https://www.nngroup.com/articles/recognition-and-recall/

[^nng-progressive]: Nielsen Norman Group, “Progressive Disclosure,” 2006. https://www.nngroup.com/articles/progressive-disclosure/

[^nng-heuristics]: Nielsen Norman Group, “10 Usability Heuristics for User Interface Design.” https://www.nngroup.com/articles/ten-usability-heuristics/

[^wai-dialog]: W3C WAI-ARIA Authoring Practices Guide, “Dialog (Modal) Pattern.” https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/

[^wai-accordion]: W3C WAI-ARIA Authoring Practices Guide, “Accordion Pattern.” https://www.w3.org/WAI/ARIA/apg/patterns/accordion/

[^wcag-focus]: W3C, WCAG 2.2 Understanding Success Criterion 2.4.13: Focus Appearance. https://www.w3.org/WAI/WCAG22/Understanding/focus-appearance.html

[^vite-public]: Vite, “Static Asset Handling — The public Directory.” https://vite.dev/guide/assets#the-public-directory

[^tabletop-aid]: Board Game Academics, “Everyone at the Table: Accessibility and Universal Design in Board Games,” notes that reference sheets and simplified rule summaries can improve accessibility and engagement. https://boardgameacademics.com/everyone-at-the-table-accessibility-and-universal-design-in-board-games/

[^rulebook-accessibility]: Karim et al., “Exploring Rulebook Accessibility and Companionship in Board Games,” ACM, 2023; cited here only for the general accessibility risk that inaccessible rulebooks hinder rule learning, especially for blind/low-vision players. https://dl.acm.org/doi/fullHtml/10.1145/3563657.3595970
