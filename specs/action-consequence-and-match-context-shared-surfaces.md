# action-consequence-and-match-context-shared-surfaces — Action costs/consequences, match identity, and automation narration

- **Filename:** `specs/action-consequence-and-match-context-shared-surfaces.md`
- **Spec ID:** `action-consequence-and-match-context-shared-surfaces`
- **Target type:** New spec
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate
- **Roadmap build gate:** None. Non-gate sibling of `rules-display-shared-surface`,
  `victory-explanation-shared-surface`, and
  `card-and-action-presentation-shared-surfaces`, motivated by a live-app
  usability audit of `event_frontier` (Gate 14) on 2026-06-12.
- **Status:** Done
- **Date:** 2026-06-12
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

---

## 1. Objective

Make every catalog game answer, in normal mode and before commitment, the four
questions a player actually asks while playing:

1. **What will this action cost me, and what do I get?** (action consequence)
2. **Who am I, whose turn is it, and why?** (match identity and turn context)
3. **What just happened while I wasn't choosing?** (automation narration)
4. **What does this rule object actually do?** (deep detail on demand)

Event Frontier (Gate 14), the catalog's most complex game, is the motivating
failure. A full human-vs-bot session played in the running app (2026-06-12,
seed 1, standard variant) produced the finding set in §2. The headline defect
is the one that prompted this spec: **the Charter player can see funds
accumulate but the UI never says what funds are for** — even though Rust
already attaches a live, edict-adjusted `cost` metadata entry to every
operation leaf (`games/event_frontier/src/actions.rs:483-485`) and the
TypeScript `ActionChoice` type already carries `metadata`/`tags` fields
(`apps/web/src/wasm/client.ts:1021-1028`). The data crosses the boundary today
and dies unrendered in the shared action surface.

This is the successor to `card-and-action-presentation-shared-surfaces`
(archived, Done), which built the shared surfaces this spec extends:
`ActionPathBuilder`, `DeckFlowPanel`, the per-game `ui.rs` metadata channel,
and the presentation-copy CI guard. That spec made components legible; this
one makes **decisions** legible.

---

## 2. Findings from the live-app audit (2026-06-12)

Each finding cites what was observed and where the defect lives. "Ripple"
records whether the defect is shared-shell (affects every game), a per-game
defect with a shared pattern, or game-specific copy.

| # | Finding (observed in the running app) | Defect site | Ripple |
|---|---|---|---|
| O1 | **Operation costs are invisible end-to-end.** Operation/target buttons show no cost; the confirm stage shows no cost or resource balance; the funds-spend happens silently. Rust already emits `cost` leaf metadata, live-adjusted for edicts. The How-to-Play page also never explains the cost economy (its only "cost" mention is a disclaimer line, `apps/web/public/rules/event_frontier.md:48`). | `ActionPathBuilder.tsx` (metadata unrendered); `event_frontier.md` (content gap) | Shared surface + per-game content. Any game emitting cost-like metadata (bids, bets, budgets) inherits the fix. |
| O2 | **Raw internal IDs in player-facing action labels.** Target buttons read "Survey site_charterhouse,site_crossing"; aria-labels read "Apply Survey to site_charterhouse,site_crossing". Labels are built Rust-side from raw IDs (`actions.rs:475`, `encode_selections` at `:747-756`). The presentation-copy guard scans TS component *source* and cannot see Rust-supplied runtime strings, so this passed CI despite violating `docs/UI-INTERACTION.md` §19 ("no raw internal identifiers"). | `games/event_frontier/src/actions.rs`; guard scope (`scripts/check-presentation-copy.mjs`) | Per-game Rust defect + a shared guard hole every future game falls into. |
| O3 | **Multi-target selection is a combinatorial leaf dump.** Choosing Survey with ops 2 over 3 legal sites renders 6 pre-joined combination buttons instead of per-site staged picking; the confirm summary is the raw path string "Operation / Survey / Survey site_charterhouse,site_crossing"; targets are not selectable by clicking the map sites they name. | `ActionPathBuilder.tsx` rendering of multi-target leaves; `EventFrontierBoard.tsx` (no map linkage) | Shared surface. Worst case here is C(6,1)+C(6,2)+C(6,3)=41 buttons; future games scale worse. |
| O4 | **No faction identity or ownership framing.** Nothing in the match surface says the local player is the Charter. Setup says "SEAT 0 / Local"; the mode line says "Seat 0 is active". Resource tiles ("Funds 2", "Provisions 4") never name their owning faction. The operator's impression that "Charter is locked to the human" is this gap: Charter is pinned to seat_0 in Rust and Human-vs-bot pins seat 0 local, but the UI never states either fact. | Shared shell (`main.tsx` setup/mode line); board resource strip | Shared shell, all games. |
| O5 | **Auto-resolved phases happen invisibly.** Confirming one Survey triggered: funds 2→0 spend, Reckoning auto-resolution (instant-victory check, site scoring 0→2 both, +2 income both, edict expiry, eligibility reset) and two card transitions — in one click with no board narration. The only record is the effect log at the very bottom of the page. | Shared shell (no near-board narration surface); effect log placement | Shared shell. `flood_watch` environment automation and every bot turn have the same problem. |
| O6 | **Effect-log vocabulary drifts from the authored card vocabulary.** The log says "reckoning one is current" where the deck panel says "First Reckoning"; site/card names appear as lowercased internal-ish strings. Two surfaces narrate the same object with different names. | Rust effect copy (`visibility.rs` effect formatting) | Per-game copy; pattern repeats wherever effect strings bypass authored labels. |
| O7 | **The detail tier is a no-op.** Card "Details" disclosures repeat the one-line summary verbatim. Edicts never state their precise scope — "Survey Ban" says "limits survey operations" but not which operations at which sites, so the actual rule is unlearnable in-app. | `cards_presentation.toml` (no deep-detail field); `CardFaceView` schema | Shared metadata schema; `flood_watch` inherits. |
| O8 | **Status copy assumes rulebook knowledge.** "3 sites needed" / "4 caches needed" (victory thresholds with no framing), "Reckoning 1 resolved: none." (none of *what*?). | Per-game copy (`visibility.rs`, board panels) | Game-specific copy; the *pattern* (threshold framing) is shared. |
| O9 | **Constrained menus are unexplained.** As second chooser the Charter menu offered only Operation/Pass — no "Event" option, with no indication the menu was constrained or why. Rust already emits `eligibility_consequence` metadata ("acting_forfeits_next_card") that is never rendered. | Shared surface (metadata unrendered) + per-game copy | Shared: same fix path as O1. |
| O10 | **Rules surface defects.** The How-to-Play renderer mangles snake_case (underscores parsed as emphasis: "eventfrontier_ _Formal rules source…"); `seat_0`/`seat_1` vocabulary appears in player-facing prose; a "Source notes for maintainers" section ships to players (`event_frontier.md:132`); the economy (costs, caps, income) is undocumented. | `RulesPanel.tsx` markdown rendering; `event_frontier.md` authoring | Shared renderer bug + per-game content + authoring-contract gap. |
| O11 | **Waiting states read as broken states.** While the bot holds priority the action panel says "No legal actions available."; a placeholder panel says "Card flow, eligibility, operation paths, and public scoring will update here." Bot turns require a manual "Run Bot Turn" click with no narration of what the bot then did beyond the bottom-of-page log. | Shared shell | Shared shell, all games. |
| O12 | **Match setup gaps.** No variant selector exists, so `event_frontier_hard_winter` and `event_frontier_land_rush` (and `frontier_control_highlands`, `flood_watch_deluge`) are unreachable in the UI; picker cards and the setup panel show raw engine strings ("rules 1 / schema 1", raw variant IDs); the picker card is clickable only on its inner button, not the card surface. | Shared shell (picker/setup) | Shared shell, all games. |
| O13 | **No bot "why?" affordance.** `docs/UI-INTERACTION.md` §15 says public mode SHOULD offer one for non-random bots; Event Frontier ships scripted policy bots (Gate 14) but the session surfaced no explanation affordance. (Suspected gap — needs an audit of what the wasm bridge exposes; see assumption A6.) | Shared shell + per-game bot explanation plumbing | Shared, all games with Level 1+ bots. |

**Verification note.** O1/O2/O9 were verified down to the generating code:
cost and consequence metadata exist on the operation leaves (`actions.rs:480-488`),
whose metadata vector carries the keys `op`, `site_count`, `cost`, `ops_bound`,
and `eligibility_consequence`. The `cost_rule` tag is emitted **not** on the
leaf but on the operation-*kind* metadata node (`actions.rs:435`, value
`base_one_resource_per_site`); the reserved-key projection in WB2 must read it
from there. Labels are formatted from raw IDs (`actions.rs:475`, with
`encode_selections`/`selection_label` at `:747-760`), and the TS types already
accept the metadata (`client.ts:1021-1028`). `SiteView` projects raw `SiteId`
with no display label (`games/event_frontier/src/visibility.rs:45-52`), so site
display names currently live only in per-board TS — the label resolution in WB1
must add a Rust-side site-label channel, not reuse a TS one.

---

## 3. External grounding

Practitioner and research sources gathered 2026-06-12 (presentation guidance
only; no architecture decisions rest on them):

- Digital adaptations of asymmetric games solve the "which faction am I and
  what can it do" problem with explicit faction framing and onboarding —
  [Root (Dire Wolf)](https://www.direwolfdigital.com/root/) highlights
  decision areas, auto-computes scores, and fronts faction identity because
  asymmetry prevents learning by watching the opponent
  ([review](https://amostagreeablepastime.com/2021/11/26/root-review-the-board-game-retains-its-brilliance-in-the-switch-to-digital/),
  [analysis](https://cjleo.com/blog/what-root-and-other-board-games-gain-and-lose-in-digital-format/)).
- Compute-before-commit: digital Suburbia "does the math for you before you
  commit to placing a tile — that way, you know what you're getting"
  ([Board game UX, UX Collective](https://uxdesign.cc/board-game-ux-including-technology-7c33f1c44968)).
  This is exactly the §9 Rust-preview doctrine; the gap is rendering it.
- Progressive disclosure: surface the decision-relevant facts (cost, key
  consequence) at the choice point; push full rule text to an on-demand tier
  ([UXPin](https://www.uxpin.com/studio/blog/what-is-progressive-disclosure/),
  [LogRocket](https://blog.logrocket.com/ux-design/progressive-disclosure-ux-types-use-cases/),
  [strategy-game tooltip practice](https://www.patternsgameprog.com/strategy-game-20-tooltips),
  [Game Developer UI dos/don'ts](https://www.gamedeveloper.com/design/ui-strategy-game-design-dos-and-don-ts)).
- The predecessor spec's grounding (BGA status-prompt pattern, two-tier card
  rendering, public-info-never-hover-gated, Game Accessibility Guidelines)
  carries forward unchanged; this spec adds no hover-only information.

---

## 4. Goal, scope, and non-goals

### 4.1 Goal

Before confirming any action, a player can see what it costs, what it
consumes, and its non-obvious consequences; at any moment they can see which
faction/role they are, who is acting and why; whenever the game advances
without their input (bot turns, auto-resolved phases) the board narrates what
happened in authored vocabulary; and every rule object on the table can
explain itself fully on demand. All facts come from Rust views, Rust action
metadata, or inert authored presentation data — never TypeScript guesses.

### 4.2 In scope

- A typed **action-affordance metadata convention** (reserved presentation
  keys: `cost` and `eligibility_consequence` on the leaf, `cost_rule` on the
  operation-kind node, plus a documented registry; the existing internal leaf
  keys `op`, `site_count`, and `ops_bound` are catalogued as non-presentation
  keys the surfaces ignore) and its rendering in the shared action surfaces
  (`ActionPathBuilder`, `ActionControls`): cost chips on choices, a
  consequence line, and a confirm-stage summary interpolating cost against
  the viewer's visible resource balance.
- **Rust label resolution** for `event_frontier` action labels and
  accessibility labels (display names, never raw IDs), including a Rust-side
  site display-label channel, with the multi-target stage rendered as a
  grouped target list rather than a pre-joined ID string.
- A **presentation-only multi-target composer** in the shared action surface:
  per-target toggles synthesized from the legal leaf set Rust supplied (TS
  only groups/filters legal leaves; it never invents a combination Rust did
  not emit), with optional board-target highlighting hooks boards can adopt.
- A **match-context surface**: "You play the Charter — Freeholders (bot) to
  act"-style framing driven by per-game seat/faction display labels projected
  through the existing `ui.rs` channel; seat vocabulary (`Seat 0`, `seat_0`)
  removed from normal-mode shell copy (setup seats, mode line, status line);
  resource tiles gain owner attribution ("Funds — Charter (you)").
- A shared **turn report surface** near the board: the viewer-filtered
  semantic effects since the player's last decision point, grouped per
  resolution burst (bot turn, auto-resolved phase), in authored-label
  vocabulary; the existing bottom-of-page effect log remains as history.
- **Rust effect-copy alignment**: effect strings resolve authored labels and
  consistent ordinal vocabulary (e.g. the effect log's `reckoning {n} resolved`
  numeral vs. the deck panel's `First Reckoning` ordinal) instead of drifting
  from the authored card vocabulary, for `event_frontier` first and as a
  recorded audit row for the other narrating games. (Site names already resolve
  via `site.label()` in `visibility.rs`; the gap is card/ordinal vocabulary.)
- A **deep-detail tier**: optional `details` prose field (a new parallel array)
  in the presentation metadata schema (`cards_presentation.toml`, `CardFaceView`),
  rendered by the existing Details disclosure; authored edict/card detail text for
  `event_frontier` (including precise edict scope) and `flood_watch` parity.
- **Status-copy clarity pass** for `event_frontier`: victory-threshold
  framing, reckoning-resolution copy, constrained-menu explanation (rendering
  the `eligibility_consequence` metadata in plain language).
- **Rules-surface fixes**: RulesPanel markdown rendering of snake_case;
  authoring contract for the canonical player rules file (no maintainer
  sections in the player file, no seat IDs in player prose, a required
  "Costs and economy" section for resource games); content updates authored
  in the canonical source `games/event_frontier/docs/HOW-TO-PLAY.md` and
  regenerated into `apps/web/public/rules/event_frontier.md` via
  `scripts/copy-player-rules.mjs` (kept green by `scripts/check-player-rules.mjs`).
- **Match setup**: a typed variant selector (display label, sourced from the
  per-variant `*_display_name` already in `variants.toml` and projected through
  the catalog) for games with multiple variants; picker/setup copy hygiene
  (engine strings out of normal mode); whole-card click affordance on the
  `.game-card` wrapper while keeping the separate "How to Play" affordance.
- **Runtime presentation guard**: extend the smoke-level DOM sweep to fail on
  raw snake_case identifiers in normal-mode visible text and accessibility
  labels (closing the Rust-supplied-string hole the source-scan guard cannot
  see), with negative-test proof that the guard fails on induced drift.
- **Bot "why?" audit** (O13): determine what the bridge exposes for the
  scripted policy bots; either render the §15 affordance or record the
  blocking gap as a named follow-up.
- Lift-ready amendments to `docs/UI-INTERACTION.md` and
  `docs/OFFICIAL-GAME-CONTRACT.md` (§10 below).

### 4.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Changing legality, action-tree **structure**, or path encoding | Forbidden | Submitted paths must stay byte-identical; replay/serialization untouched. The multi-target composer is presentation grouping over the existing legal leaf set. Restructuring the tree into per-site stages would change command encoding — that is a replay-semantics change requiring ADR per FOUNDATIONS §13. |
| Moving any visibility line (undrawn counts, hidden order, redacted facts) | Forbidden | FOUNDATIONS §13 ADR trigger. Cost/consequence facts rendered here are already viewer-safe leaf metadata or public view fields. |
| Animation scheduler / effect-driven board animation work | Deferred | The turn report is a narration surface, not animation. Semantic-effect animation remains the §10 doctrine and a future spec; nothing here blocks it. |
| Auto-running bot turns (removing the manual trigger) | Deferred | Orchestration-policy change with pacing/accessibility implications; the spec fixes the *copy and narration* around the existing trigger. Candidate follow-up. |
| FOUNDATIONS amendment / new ADR | Not needed | Verified against `docs/FOUNDATIONS.md`: every fix lands inside existing law (§2 ownership, §5 static-data limits, §7 UI doctrine). UI-INTERACTION §9 already sanctions previews showing "visible cost"; §19 already bans raw identifiers. This spec *enforces* existing law and extends two area docs via the precedented lift-ready-amendment path (as `card-and-action-presentation-shared-surfaces` did). No constitutional principle changes meaning. |
| `engine-core` changes | Forbidden | `ActionChoice` metadata is already an opaque key/value contract in the kernel. The key *convention* is documented in `docs/UI-INTERACTION.md` (presentation law), not typed into `engine-core` — the kernel must not know what `cost` means (FOUNDATIONS §3). |
| `game-stdlib` helpers | Deferred | Any repeated label-resolution/metadata shape routes through `docs/MECHANIC-ATLAS.md` pressure later. |
| Picker visual redesign (catalog card art/layout) | Deferred | Carried from predecessor §13. This spec takes only the *functional* setup gaps (variant selector, identity labels, copy hygiene, click target). |
| Tooltip-only access to public information | Forbidden | Carried from predecessor: glanceable first, detail tier second. |
| YAML / DSL / behavior-bearing data fields | Forbidden | FOUNDATIONS §5. `details` prose and label rows are inert typed content. |

---

## 5. Foundation and boundary alignment

| Authority | Constraint engaged | Alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §2 (behavior authority) | Rust owns legality, views, effects; TS presents | All new facts are Rust-emitted (leaf metadata, view fields, effects) or inert authored metadata keyed to Rust IDs. TS interpolates safe template parameters (cost vs. balance) exactly as the outcome surface already does; it computes no legality, no costs, no eligibility. The multi-target composer only groups/filters leaves Rust emitted — selecting targets without a matching legal leaf is impossible. |
| `docs/FOUNDATIONS.md` §3 (`engine-core` noun-free) | Metadata key convention | No kernel change. Keys are strings in the existing opaque metadata contract; their meaning is documented in UI law and implemented per game. |
| `docs/FOUNDATIONS.md` §5 (static data is content) | `details` field, label rows, explanation templates | Inert prose keyed to Rust IDs/tags; typed loaders; unknown fields rejected — the sanctioned "explanation templates keyed to Rust effects/actions" category. |
| `docs/FOUNDATIONS.md` §7 / §11 (play-first UI, no leaks) | Whole spec | This spec exists to enforce §7. Every rendered fact is already viewer-safe; no new payload category is introduced. The turn report renders the same viewer-filtered effect log the page already receives. |
| `docs/FOUNDATIONS.md` §12 (stop conditions) | TS legality; debug-first UI; hidden-info leak | Explicit stop conditions for every decomposed ticket. The runtime guard *reduces* §12 exposure by catching raw-identifier regressions. |
| `docs/UI-INTERACTION.md` §5/§8/§9 (payloads, staging, previews) | Cost/consequence display | §9 previews "MAY include visible cost, expected visible effects" — this spec makes that a MUST when the game emits the reserved keys, via §19 amendment. Staging (§8) gains the preview step it currently skips. |
| `docs/UI-INTERACTION.md` §15 (bot why) | O13 audit | Implements or formally defers the existing SHOULD. |
| `docs/OFFICIAL-GAME-CONTRACT.md` §10/§12 | Per-game UI metadata, catalog docs | Seat/faction labels and detail prose ride the established `ui.rs`/presentation-TOML channels; catalog README updates ride the existing closeout checklist. |
| `docs/IP-POLICY.md` | Authored prose | All new labels/details/explanations are original Rulepath prose. |
| `docs/TESTING-REPLAY-BENCHMARKING.md` | Fixture/trace updates | View/metadata additions ride the ordinary fixture/golden-trace update path; no hash semantics change. |

**Verdict on foundations amendments (the operator asked):** none required.
The constitution already mandates everything this spec enforces; the gaps are
implementation debt plus two area-doc strengthenings (§10) on the precedented
amendment path. An ADR would be required only for the explicitly-deferred
items (tree restructuring, visibility moves, auto-bot orchestration).

---

## 6. Committed design decisions

### D1 — Reserved action-metadata keys, documented in UI law

`docs/UI-INTERACTION.md` gains a reserved-key table for `ActionChoice.metadata`:
`cost` (viewer-visible numeric cost in the acting faction's primary resource),
`cost_rule` (stable rule-reference tag), `eligibility_consequence` (stable
consequence tag). In `event_frontier` today `cost` and `eligibility_consequence`
ride the operation leaf while `cost_rule` rides the operation-kind node
(`actions.rs:435`); the reserved-key projection reads each from where the game
emits it. Games MAY emit them; when emitted, the shared surfaces MUST
render them. Plain-language strings for tags come from authored explanation
templates keyed to the tag (static, typed, per game) — e.g.
`acting_forfeits_next_card` → "Acting now forfeits your eligibility for the
next card." The key registry lives in UI law, not `engine-core`.

### D2 — Cost and consequence render at choice and confirm time

`ActionPathBuilder`/`ActionControls`: choices carrying `cost` render a cost
chip ("2 funds"); stages carrying `eligibility_consequence` render the
resolved template line once per stage; the confirm stage renders a summary
("Survey — Charterhouse and Crossing · Spends 2 of your 2 funds · Acting now
forfeits your next-card eligibility") built from Rust labels, leaf metadata,
and the public view's resource balance. No TS arithmetic beyond display
interpolation of Rust-supplied numbers.

### D3 — Labels resolve to display names everywhere, enforced at runtime

`event_frontier` gains a Rust-side site display-label table (presentation
TOML, same pattern as cards) projected through `SiteView`; action label and
accessibility-label generation (`actions.rs:475`, `:747-760`) switches to
resolved labels with human list joining ("Charterhouse and Crossing").
Effect copy aligns to authored labels (O6). The smoke-level DOM sweep gains a
normal-mode raw-identifier assertion (visible text + aria attributes must not
match `[a-z0-9]+_[a-z0-9_]+` token patterns, allowlisting dev panels), with a
negative test proving the guard trips on induced drift.

### D4 — Multi-target selection is a composer, not a combination dump

When a stage's choices are multi-target leaves sharing one operation, the
shared surface renders per-target toggles + a live summary instead of one
button per combination. The toggle set and its enabled/disabled states are
derived purely from the legal leaf set (a combination is selectable iff its
leaf exists). Confirm submits the exact matching leaf path — byte-identical
encoding. Boards MAY register target-highlight hooks (map sites light up as
toggles); `EventFrontierBoard` adopts them.

### D5 — Match context is explicit and faction-first

Per-game `ui.rs` metadata gains seat display labels ("Charter", "Freeholders";
games without factions use existing player naming). The shared shell renders:
setup seats as "Charter — you (local)" / "Freeholders — bot"; an in-match
identity line ("You play the Charter"); active-turn status in faction terms
("Freeholders (bot) to act"); resource tiles with owner attribution. The
strings `Seat 0`/`seat_0` disappear from normal-mode surfaces (the dev panel
keeps seat vocabulary).

### D6 — A turn report narrates every non-interactive advance

A shared `TurnReportPanel` (name indicative) sits adjacent to the action
surface and shows the viewer-filtered effects since the player's last
decision, grouped by resolution burst with authored-label copy ("First
Reckoning resolved — no instant victory. Sites scored: Charter +2,
Freeholders +2. Income: +2 funds, +2 provisions. Edicts expired."). It is a
re-presentation of the existing effect-log payload — no new data crosses the
boundary. The bottom effect log remains as full history. Adopted by
`event_frontier` and `flood_watch`; other games get an audit row (most have
no automation bursts and may record "not applicable").

### D7 — The detail tier carries real content

`CardFaceView`/presentation TOML gain an optional `details` field (full
effect/scope prose). The Details disclosure renders it when present and is
omitted when absent (no more no-op disclosure). `event_frontier` authors
details for all 21 cards — edicts state their precise scope ("Blocks Survey
and Rally at contested sites — sites holding both agents and settlers — until
the next Reckoning."). `flood_watch` reaches parity.

### D8 — Rules surface is correct, complete, and player-only

`RulesPanel` markdown rendering stops treating intra-word underscores as
emphasis (code-span or escaped rendering for identifiers). The player-rules
authoring contract (in `docs/OFFICIAL-GAME-CONTRACT.md` §5's "Player-facing
rules document" subsection) adds: no maintainer/source-note sections in the
player file (they move to the game's `SOURCES.md`), no seat IDs in player
prose, and a required "Costs and economy" section for games with spendable
resources. The canonical source `games/event_frontier/docs/HOW-TO-PLAY.md` is
edited (funds/provisions economy, operation costs, edict cost modifiers,
income, caps; `Source notes for maintainers` block relocated to
`games/event_frontier/docs/SOURCES.md`; `seat_0`/`seat_1` prose replaced with
faction names) and `apps/web/public/rules/event_frontier.md` is regenerated
from it via `scripts/copy-player-rules.mjs`, leaving `scripts/check-player-rules.mjs`
green.

### D9 — Setup offers variants and drops engine strings

Match setup gains a variant selector populated from the catalog's typed
variant list. Variant **display labels** are sourced Rust-side: the
per-variant `*_display_name` already in `variants.toml` is projected through
the catalog. This requires extending the catalog projection in
`crates/wasm-api/src/lib.rs` (currently each game's `variants` is a flat array
of bare ID strings) to carry per-variant `{id, label}` objects, and the
matching `GameCatalogEntry.variants` type change in `client.ts`. Variant
*descriptions* are explicitly out of scope (no description text exists in the
manifest today; labels-only avoids authoring net-new presentation content —
see A9). Default remains the standard variant. Picker cards and the setup
panel drop "rules N / schema N" and raw variant IDs from normal mode (kept in
the dev panel); the whole `.game-card` wrapper becomes the click target while
the separate "How to Play" button is preserved.

### D10 — Future-binding

A new official web-exposed game is not UI-complete unless: actions emitting
reserved metadata render cost/consequence (D1/D2); labels pass the runtime
identifier guard (D3); multi-target stages use the composer or record a
board-native exception (D4); match context renders faction-first (D5);
automation bursts render a turn report or record `not applicable` (D6);
how-to-play satisfies the extended authoring contract (D8). Lands as §19
additions and contract amendments (§10).

---

## 7. Deliverables

```text
games/event_frontier/data/sites_presentation.toml        (site display labels)
games/event_frontier/data/cards_presentation.toml        (details parallel array added)
games/event_frontier/data/*                              (explanation templates for consequence/cost-rule tags)
games/event_frontier/docs/HOW-TO-PLAY.md                 (canonical player rules: economy section; maintainer block removed; seat prose → faction names)
games/event_frontier/docs/SOURCES.md                     (relocated maintainer/source-note content)
games/event_frontier/src/ui.rs                           (seat/faction labels; template projection)
games/event_frontier/src/visibility.rs                   (SiteView label; effect-copy label/ordinal resolution)
games/event_frontier/src/actions.rs                      (resolved action/accessibility labels)
games/event_frontier/data/variants.toml                  (per-variant display label already present; projected by wasm-api)
games/flood_watch/{data,src}                             (details parity; seat labels; turn-report adoption)
games/*/src/ui.rs                                        (seat-label backfill, one audit row per game)
crates/wasm-api/src/lib.rs                               (catalog projection: per-variant {id, label} objects; seat/faction labels)
apps/web/src/wasm/client.ts                              (type additions: GameCatalogEntry.variants objects, labels, details, templates)
apps/web/src/components/ActionPathBuilder.tsx            (cost/consequence rendering; multi-target composer; confirm summary)
apps/web/src/components/ActionControls.tsx               (cost chip parity)
apps/web/src/components/TurnReportPanel.tsx              (new shared surface)
apps/web/src/components/RulesPanel.tsx                   (markdown identifier rendering fix)
apps/web/src/components/EventFrontierBoard.tsx           (identity line; resource attribution; map-target hooks; copy)
apps/web/src/main.tsx                                    (setup variant selector; seat→faction labels; waiting copy)
apps/web/public/rules/event_frontier.md                  (regenerated from HOW-TO-PLAY.md via scripts/copy-player-rules.mjs — not hand-edited)
apps/web/e2e/                                            (smoke: cost display, composer, turn report, runtime identifier guard + negative test, variant selector)
scripts/check-presentation-copy.mjs                      (guard scope note; runtime guard lives in smoke)
scripts/copy-player-rules.mjs                            (re-run to regenerate the rendered rules asset)
scripts/check-player-rules.mjs                           (source↔generated parity guard kept green)
docs/UI-INTERACTION.md, docs/OFFICIAL-GAME-CONTRACT.md   (lift-ready amendments applied at closeout)
apps/web/README.md                                       (Shell Surface: TurnReportPanel, composer, variant selector)
specs/README.md                                          (index row maintained)
```

Rust changes ride the ordinary verification set (unit/rule tests,
fixture-check, replay-check, serialization, golden traces, simulate smoke).

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

| # | Item | Depends on |
|---|---|---|
| WB1 | `event_frontier` Rust label resolution: site presentation TOML + loader/validation, `SiteView` label projection, resolved action/accessibility labels, effect-copy alignment (card/ordinal vocabulary — sites already use `site.label()`); fixture/trace/serialization updates; unit coverage proving no `site_`/`ef_` tokens in emitted labels | — |
| WB2 | Reserved-key convention + explanation templates: tag-template data + projection for `event_frontier` (`eligibility_consequence`, `cost_rule`); `client.ts` types; UI-INTERACTION reserved-key table drafted (lifted at WB10) | — |
| WB3 | `ActionPathBuilder`/`ActionControls`: cost chips, consequence lines, confirm summary with resource-balance interpolation | WB1, WB2 |
| WB4 | Multi-target composer in `ActionPathBuilder` (leaf-set-derived toggles, byte-identical submission) + `EventFrontierBoard` map-highlight hooks; smoke proving submitted paths unchanged vs. leaf-button flow | WB3 |
| WB5 | Match context: seat/faction display labels in `ui.rs` channel (EF + catalog backfill audit), setup/mode/status faction-first copy, resource-tile attribution, waiting-state copy fix (O11) | — |
| WB6 | `TurnReportPanel` + adoption in `event_frontier` and `flood_watch`; catalog audit rows; smoke for reckoning-burst narration | WB1 |
| WB7 | Detail tier: `details` schema field + EF 21-card authoring (incl. edict scopes) + `flood_watch` parity; status-copy clarity pass (O8: thresholds, reckoning copy) | WB1 |
| WB8 | Rules surface: RulesPanel identifier-rendering fix; player-rules contract additions (`OFFICIAL-GAME-CONTRACT.md` §5 subsection); edit canonical `games/event_frontier/docs/HOW-TO-PLAY.md` (economy section + maintainer block relocated to `SOURCES.md` + seat→faction prose), regenerate `apps/web/public/rules/event_frontier.md` via `scripts/copy-player-rules.mjs` and keep `scripts/check-player-rules.mjs` green; catalog rules-file audit against the new contract | — |
| WB9 | Setup: variant selector (display labels projected from `variants.toml` `*_display_name` via `crates/wasm-api/src/lib.rs` catalog projection + `GameCatalogEntry.variants` object type in `client.ts`; default standard; descriptions out of scope per A9), picker/setup engine-string removal, whole-`.game-card` click target; smoke incl. starting a non-standard variant | — |
| WB10 | Runtime identifier guard in the smoke DOM sweep + negative test; bot-why audit (O13) → implement minimal §15 affordance or record named follow-up; closeout: lift amendments, `apps/web/README.md`, index flip to Done with evidence | WB1–WB9 |

---

## 9. Exit criteria

1. Playing `event_frontier` normal mode: every operation choice shows its
   live cost before selection; the confirm stage states cost against the
   current balance and any eligibility consequence; after confirming, the
   player's funds change is narrated in the turn report. The original
   complaint — "funds exist but nothing says what uses them" — is closed at
   choice time, confirm time, and in the rules page.
2. No normal-mode surface (visible text or accessibility attributes) renders
   raw snake_case identifiers; the runtime DOM guard passes catalog-wide and
   demonstrably fails on induced drift (negative test recorded).
3. Multi-target selection never renders more controls than targets + confirm;
   submitted action paths are byte-identical to the pre-composer encoding
   (replay/serialization proof attached).
4. At any point in any catalog game, the UI states which faction/role the
   local viewer plays and who is acting, in faction terms; `Seat N` appears
   only in the dev panel. Setup seats are faction-labeled; multi-variant
   games are startable in every variant from the UI.
5. Auto-resolved phases and bot turns produce a readable turn report adjacent
   to the board in authored vocabulary; `flood_watch` parity; audit rows for
   the rest of the catalog.
6. Card/edict Details disclosures show authored deep detail or are absent;
   every `event_frontier` edict states its precise scope in-app.
7. The How-to-Play surface renders identifiers correctly, contains no
   maintainer sections or seat IDs, and documents the economy for every
   resource-spending game per the extended contract. Player-rules edits are
   authored in the canonical `games/<game_id>/docs/HOW-TO-PLAY.md`; the
   rendered `apps/web/public/rules/<game_id>.md` asset is regenerated, and
   `node scripts/check-player-rules.mjs` passes (source↔generated parity).
8. All Rust changes pass gate 0 (`fmt`, `clippy`, `build`, `test`) and the
   per-game gate (`simulate`, `replay-check`, `fixture-check`,
   `rule-coverage`) for touched games; web `smoke:wasm`, `smoke:ui`,
   `smoke:effects`, `smoke:e2e` green; no-leak sweeps unchanged-or-stronger.
9. Amendments applied to `docs/UI-INTERACTION.md` / `docs/OFFICIAL-GAME-CONTRACT.md`;
   `apps/web/README.md` updated; `node scripts/check-doc-links.mjs` and
   `node scripts/check-catalog-docs.mjs` pass; index row flipped with evidence.

### Acceptance evidence

The re-runnable confirmation set (folded into the exit criteria above rather
than a gate's row-for-row exit list, since this is a non-gate UI-infra spec):

- **Rust**: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo test --workspace`; per touched game `cargo run -p
  simulate/replay-check/fixture-check/rule-coverage`; updated golden traces and
  serialization fixtures for the `SiteView`/effect-copy/`ui.rs` additions.
- **Web**: `npm --prefix apps/web run smoke:wasm | smoke:ui | smoke:effects |
  smoke:e2e`, including the new cost-display, composer byte-identity,
  turn-report, runtime-identifier-guard (+ negative test), and variant-selector
  smokes.
- **Docs/boundary**: `node scripts/check-doc-links.mjs`,
  `node scripts/check-catalog-docs.mjs`, `node scripts/check-player-rules.mjs`,
  `node scripts/check-presentation-copy.mjs`, `bash scripts/boundary-check.sh`.

Capstone evidence recorded 2026-06-12:

- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace`, and `cargo test --workspace` passed.
- `event_frontier`: `simulate --games 1000` passed with
  `simulation_pass_rate_percent=100.00`; `replay-check --all`,
  `fixture-check`, and `rule-coverage` passed.
- `flood_watch`: `simulate --games 1000` passed; `replay-check --all`,
  `fixture-check`, and `rule-coverage` passed.
- Web: `smoke:wasm`, `smoke:ui`, `smoke:effects`, and `smoke:e2e` passed.
- Docs/boundary: `check-doc-links`, `check-catalog-docs`,
  `check-player-rules`, `check-presentation-copy`, and `boundary-check`
  passed.
- Event Frontier detail enumeration: 21 card ids, 21 labels, and 21 detail
  entries in the static presentation rows.

---

## 10. Lift-ready amendment text (applied at WB10, not before)

**`docs/UI-INTERACTION.md` §5/§9 addition (reserved metadata keys):**

```text
Action-tree leaves MAY carry reserved presentation metadata keys with fixed
meaning: `cost` (viewer-visible cost in the acting seat's primary resource),
`cost_rule` (stable rule-reference tag), `eligibility_consequence` (stable
consequence tag resolved through authored explanation templates). When a game
emits reserved keys, shared action surfaces MUST render them at choice and
confirmation time. Reserved keys are documented here, not typed into
engine-core; the kernel treats metadata as opaque.
```

**`docs/UI-INTERACTION.md` §19 additions:**

```text
- choices carrying reserved cost/consequence metadata display them before
  selection and in the confirmation summary, interpolated only with
  Rust-supplied numbers and viewer-safe balances;
- action labels and accessibility labels contain display names, never raw
  internal identifiers; a runtime DOM sweep enforces this in normal mode;
- multi-target stages render as composed target selection or a recorded
  board-native mapping, never one control per combination;
- normal-mode surfaces name the local viewer's faction/role and the acting
  faction in display terms; seat indices are dev-panel vocabulary;
- non-interactive advances (bot turns, automated phases) are narrated near
  the board from viewer-filtered semantic effects in authored vocabulary.
```

**`docs/OFFICIAL-GAME-CONTRACT.md` §5 "Player-facing rules document" additions:**

```text
The public How-to-Play file MUST NOT contain maintainer-facing sections or
internal seat identifiers, and MUST document the game's resource economy
(sources, sinks, caps, costs) when the game has spendable resources. Games
with multiple public variants MUST expose every variant through match setup
with a typed display label.
```

---

## 11. Forbidden changes

- No legality, action-tree-structure, path-encoding, or validation changes;
  submitted command bytes stay identical.
- No visibility-contract change; no new exposure of hidden facts
  (`EF-VIS-002` stance unchanged).
- No `engine-core` edits; no `game-stdlib` additions outside the atlas path.
- No TypeScript-computed legality, costs, ordering, or label derivation from
  IDs.
- No behavior-looking fields in presentation data; no YAML; no DSL.
- No replay/hash semantic changes; trace updates only through the documented
  migration path.
- No weakening or deletion of existing tests (AGENT-DISCIPLINE §4); the
  predecessor's guards stay intact and are only strengthened.
- No bot policy changes; the bot-why work renders existing explanations or
  records the gap — it does not add new bot reasoning.

---

## 12. Documentation updates required

- `docs/UI-INTERACTION.md` — §5/§9 reserved-key table, §19 additions (§10).
- `docs/OFFICIAL-GAME-CONTRACT.md` — §5 "Player-facing rules document"
  authoring-contract additions (§10).
- `apps/web/README.md` — Shell Surface (TurnReportPanel, composer, variant
  selector); Smoke Layers (runtime identifier guard).
- `games/event_frontier/docs/HOW-TO-PLAY.md` (canonical player rules edited)
  and `games/event_frontier/docs/SOURCES.md` (relocated maintainer/source-note
  content); `games/event_frontier/docs/` and `games/flood_watch/docs/` UI
  metadata notes. The rendered `apps/web/public/rules/event_frontier.md` is a
  generated asset (regenerated via `scripts/copy-player-rules.mjs`), not a
  hand-edited doc.
- `specs/README.md` — index row added now (`Planned`); flipped to `Done` at
  WB10 with evidence.

---

## 13. Sequencing

- **Predecessor:** `card-and-action-presentation-shared-surfaces` (Done,
  archived). It built the surfaces this spec extends and explicitly deferred
  the picker/setup gaps (its §13 successor note and assumption A2) that
  WB9 now takes up in functional form.
- **Admission:** Non-gate UI-infrastructure spec; no mechanic-ladder gate is
  blocked. `docs/MECHANIC-ATLAS.md` carries no open promotion debt naming
  this work. Gate P (private) is unaffected.
- **Successors (named, deferred):** picker visual redesign (card art/layout);
  effect-driven board animation (UI-INTERACTION §10 scheduler); auto-run bot
  turn orchestration; any action-tree restructuring for staged multi-target
  encoding (ADR-gated).

---

## 14. Assumptions (one-line-correctable)

1. **(A1) One combined spec** — assuming a single spec covering the five
   surfaces (consequence, identity, narration, detail, setup) since they
   share one audit and one law-amendment batch; split into siblings if
   preferred.
2. **(A2) Variant selector is functional scope, not visual polish** —
   assuming the predecessor's deferred "picker/setup polish" splits, with
   functional gaps (unreachable variants, seat labels, engine strings) in
   scope here and visual redesign still deferred.
3. **(A3) Composer over tree restructuring** — assuming multi-target UX is
   fixed presentation-side over the existing leaf encoding (replay-safe);
   restructuring the Rust tree is explicitly deferred behind an ADR.
4. **(A4) Reserved-key registry lives in UI-INTERACTION** — assuming a doc
   table (not `engine-core` types, not a `game-stdlib` helper) is the right
   home at current pressure (two games emit candidate keys today).
5. **(A5) Turn report is re-presentation only** — assuming the existing
   viewer-filtered effect payload suffices (verified for the Reckoning burst
   in the live session); if a game's effects prove too sparse to narrate, that
   game's effect coverage is the gap and is fixed Rust-side, not TS-side.
6. **(A6) Bot-why exposure unverified** — assuming the wasm bridge's bot
   explanation surface needs the WB10 audit before committing to rendering;
   if nothing viewer-safe is exposed, WB10 records a named follow-up instead.
7. **(A7) Research depth** — practitioner sources + one prior DiGRA anchor
   reused from the predecessor; no new academic pass was run. Commission
   `research-brief` if deeper grounding is wanted.
8. **(A8) `flood_watch` parity scope** — assuming details + turn report +
   seat labels; its rules file and status copy get the audit pass, not a
   rewrite.
9. **(A9) Variant selector is labels-only** — resolved during reassessment:
   variant *display labels* are projected Rust-side from the `*_display_name`
   already in `variants.toml` (via a `crates/wasm-api/src/lib.rs` catalog
   projection extending each game's `variants` from bare ID strings to
   `{id, label}` objects). One-line variant *descriptions* are out of scope —
   none exist in the manifest today, and authoring net-new presentation prose
   is deferred; revisit if richer setup copy is wanted.
