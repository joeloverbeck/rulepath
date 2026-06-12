# effect-animation-and-turn-orchestration — Effect-driven animation scheduler and turn orchestration/pacing

- **Filename:** `specs/effect-animation-and-turn-orchestration.md`
- **Spec ID:** `effect-animation-and-turn-orchestration`
- **Target type:** New spec
- **Roadmap stage:** Cross-game web UI infrastructure — not a mechanic-ladder gate
- **Roadmap build gate:** None. Non-gate sibling of `rules-display-shared-surface`,
  `victory-explanation-shared-surface`, `card-and-action-presentation-shared-surfaces`,
  and `action-consequence-and-match-context-shared-surfaces`. Sourced from the
  prioritization brainstorm `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md`
  (recommendation P1 = candidates C1 + C2, absorbing part of C8).
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

Close the largest remaining gap between repo law and repo reality: the
constitution already mandates effect-driven animation — FOUNDATIONS §7
("Animation MUST be driven by semantic effects emitted by Rust"), FOUNDATIONS
§11 ("Semantic effects drive animation; renderer diffs are diagnostics only"),
and `docs/UI-INTERACTION.md` §10 specs the scheduler's required behaviors —
and none of it exists. Every prerequisite was built by the four Done
UI-infrastructure specs: authored labels, the viewer-filtered semantic effect
stream, burst narration (`TurnReportPanel`), and copy hygiene. The data
already crosses the boundary and renders only as text.

Two fused workstreams:

1. A shared **effect-driven animation scheduler** in `apps/web` implementing
   the full UI-INTERACTION §10 requirement list: ordered effects, grouped
   effects, simultaneous/reveal batches, redacted effects, reduced-motion
   mode, interruption by replay stepping, settle-to-view reconciliation.
2. **Turn orchestration/pacing** built on it: bot turns and auto-resolved
   phases play out on the animation timeline instead of resolving in the same
   synchronous frame; the manual "Run Bot Turn" trigger disappears; skip and
   pause are always available; input never hard-blocks.

It also absorbs the effect-log residue (brainstorm candidate C8): `EffectLog`
re-bases on the same burst grouping the `TurnReportPanel` uses. It directly
closes the two audit findings that survived both predecessor specs (actconmat
O5 "auto-resolved phases happen invisibly" and O11 "bot turns require a manual
trigger with no narration of pacing").

---

## 2. Current state (code-grounded, verified 2026-06-12)

- **Effects already cross the boundary, ordered and viewer-filtered.**
  `EffectEntry { cursor, effect: { payload: { type, ... } } }`
  (`apps/web/src/wasm/client.ts:1060-1073`), fetched incrementally via
  `getEffects(matchId, sinceCursor, viewerMode)` (`client.ts:1306`).
- **No animation exists.** Effects render as text only: `EffectLog.tsx`
  (full history + reduced-motion selector), `TurnReportPanel.tsx` (adopted by
  `event_frontier`/`flood_watch`, naive "last 6 non-marker effects" burst
  heuristic at `TurnReportPanel.tsx:47`), and the `feedbackForEffect` copy/tone
  taxonomy (`effectFeedback.ts` — tones `neutral`/`movement`/`turn`/`terminal`).
- **Bot turns resolve invisibly in the same frame.** In `human_vs_bot`,
  `api.runBotTurn(...)` is called synchronously inside the same `playChoice`/
  `playPath` callback as the human's `applyAction`, with one `refresh` at the
  end (`apps/web/src/main.tsx`, `playChoice`) — the human action and the bot's
  whole reply land as one instant state swap.
- **A manual trigger papers over orchestration.** "Run Bot Turn"
  (`apps/web/src/components/ModeControls.tsx:59`) covers bot-first starts and
  consecutive bot turns.
- **`bot_vs_bot` autoplay paces with a raw `setTimeout`** — 520 ms, 80 ms
  under reduced motion (`main.tsx` autoplay `useEffect`) — exactly the
  timing-outside-the-manager shape that production frameworks document as the
  source of broken fast/replay modes (§3).
- **Replay stepping exists** (`stepReplay`/`resetReplay` in `main.tsx`), and a
  persisted reduced-motion override (`system`/`reduce`/`motion`,
  `rulepath.reducedMotion`) flows through `state/shellReducer.ts`.
- **The per-game template already demands what this spec builds.**
  `templates/GAME-UI.md` carries "Semantic effect-to-animation mapping",
  "Settle-to-view checks", and "Reduced-motion behavior" sections that no
  shared machinery currently backs.
- **No atlas pressure.** `docs/MECHANIC-ATLAS.md` names animation only as an
  inventory question ("What effects must exist for logs, animation, replay,
  bots, and explanations?"); the §10A debt register is empty. This is
  presentation work, not mechanic promotion.

---

## 3. External grounding

Fresh research pass, 2026-06-12 (practitioner + standards; URLs cited in the
brainstorm and the research transcript). Presentation guidance only; no
architecture decision rests on an external source.

- **The platform-scale norm is a promise-based queue over a server-ordered
  event stream.** Board Game Arena's modern framework drives all animation
  from the transactional notification queue: handlers are async functions and
  the queue advances when each handler's promise (plus a declared minimum
  duration) resolves (`setupPromiseNotifications`); the official
  `BgaAnimations` library is `Element.animate`-based and awaitable
  ([BGA notifications](https://en.doc.boardgamearena.com/Game_interface_logic:_yourgamename.js),
  [BgaAnimations](https://en.doc.boardgamearena.com/BgaAnimations)). This is
  exactly Rulepath's doctrine deployed across hundreds of titles: effects are
  authoritative cause; timelines are presentation.
- **Skip/fast modes break wherever timing escapes the manager.** BGA's
  fast-replay (`instantaneousMode`) accelerates only framework-routed
  durations; custom `setTimeout`s and ad-hoc waits are not accelerated and are
  the documented bug source
  ([BGA devs blog](https://bga-devs.github.io/blog/posts/a-real-fast-replay-mode/)).
  Design consequence: all effect timing flows through one manager, and skip is
  a single flush-and-settle code path, not "1 ms durations everywhere".
- **The Web Animations API supplies the primitives.** `Animation.finished` is
  an awaitable promise; `finish()` jumps to the end state while `cancel()`
  discards effects (so skip uses finish/commit, never bare cancel);
  `playbackRate` gives in-flight fast-forward; `document.getAnimations()`
  enumerates for bulk flush
  ([MDN](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)).
  SVG caveat: WAAPI animates CSS properties only — geometry attributes are not
  animatable — but `transform` on SVG elements is a presentation attribute
  since SVG 2, so move/scale animations on `<g>` wrappers with
  `transform-box: fill-box` are the safe path
  ([MDN transform](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/transform)).
- **FLIP + ghost overlays reconcile imperative timelines with declarative
  React.** Render the settled state, measure first/last rects, animate the
  inverted transform away
  ([Lewis/CSS-Tricks](https://css-tricks.com/animating-layouts-with-the-flip-technique/),
  [Comeau](https://www.joshwcomeau.com/react/animating-the-unanimatable/));
  cross-container moves use a cloned ghost element in an overlay/portal layer —
  BGA's production library clones and reparents the animated element for the
  same reason ([BgaAnimations](https://en.doc.boardgamearena.com/BgaAnimations)).
- **The closest published analog to "engine emits typed effects, client
  animates them" is boardgame.io's bgio-effects plugin**: typed effect queue
  with explicit timeline placement for overlapping/simultaneous effects, and
  two interruption primitives — `clear()` (drop pending) and `flush()` (apply
  everything instantly = settle)
  ([bgio-effects](https://delucis.github.io/bgio-effects/)).
- **Input must never hard-block on animation.** Hearthstone queues further
  actions while attack animations resolve — the player can act faster than the
  visuals
  ([analysis](https://anykeytostart.wordpress.com/2015/03/19/hearthstone/)).
- **Reduced motion replaces movement, never feedback.** Honor
  `prefers-reduced-motion`; swap movement/zoom for instant transitions plus
  brief fades/highlights; every state change stays perceivable
  ([web.dev](https://web.dev/articles/prefers-reduced-motion),
  [Smashing](https://www.smashingmagazine.com/2020/09/design-reduced-motion-sensitivities/)).
  WCAG 2.2 SC 2.3.3 wants interaction-triggered motion disable-able; SC 2.2.2
  requires pause/stop for auto-playing content longer than 5 s — which an
  auto-advancing bot game is
  ([WCAG 2.3.3](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html),
  [WCAG 2.2.2](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html)).
- **Automation pacing is a comprehension problem, not polish.** DiGRA 2015
  ("Digitising Boardgames") documents that automating away "articulation work"
  changes what players perceive; BGA's own bug tracker carries "auto replay
  speed is too fast to follow" — pacing must be a per-event-type concern, not
  one global speed
  ([DiGRA](https://dl.digra.org/index.php/dl/article/view/725),
  [BGA bug](https://en.boardgamearena.com/bug?id=103237)).
- Carried grounding from the predecessor specs (Root's faction framing,
  Suburbia compute-before-commit, BGA status prompts, Nystrom's Event Queue
  pattern) remains in force.

---

## 4. Goal, scope, and non-goals

### 4.1 Goal

Every visible advance of a match — a human action's consequences, a bot's
turn, an auto-resolved phase, a replay step — plays out as restrained,
legible, effect-driven motion that a player can follow, skip, pause, or
disable, after which the renderer settles to the latest viewer-safe public
view. The scheduler consumes only the Rust-emitted, viewer-filtered semantic
effect stream; TypeScript times and draws, it never infers. Bot turns advance
on the timeline without a manual trigger, and waiting never looks broken.

### 4.2 In scope

- A shared **animation scheduler** module in `apps/web` (indicative home:
  `apps/web/src/animation/`): ordered queue per resolution burst, async
  promise-based steps, explicit grouping (sequential vs. simultaneous/reveal
  batches), redacted-effect presentation, manager-owned timing (no stray
  `setTimeout` anywhere in the play path), skip/flush-to-settle, global rate
  control, reduced-motion collapse, replay-step interruption, and
  settle-to-view reconciliation hooks.
- A shared **burst-segmentation module**: one definition of "resolution burst"
  (the effects between two player decision points) consumed by the scheduler,
  `TurnReportPanel` (replacing the last-6 heuristic), and `EffectLog`
  (grouped, browsable bursts — the C8 residue this spec absorbs).
- A **presentation layer** for animation realization: WAAPI helpers on
  transform/opacity of SVG `<g>` wrappers (`transform-box: fill-box`), FLIP
  measurement for moves, a ghost/overlay layer for cross-container
  transitions, and **generic effect presentations** (highlight, fade, count-up)
  keyed to the existing `feedbackForEffect` tone taxonomy so every catalog
  game gets baseline motion without per-game work.
- **Per-game adoption** on the proven audit pattern: a recorded row per
  catalog game — `adopt` (game-specific effect→animation registrations) /
  `board-native mapping` / `generic-only` / `not applicable` — with
  `event_frontier` (Reckoning bursts, card transitions, funds/score changes)
  and `flood_watch` (flood phases, environment automation) as the motivating
  adopters.
- **Turn orchestration/pacing** on the scheduler: in `human_vs_bot` the bot
  turn is decoupled from the human action's synchronous frame and plays after
  the human's effects settle, with authored dwell; consecutive bot turns and
  bot-first starts auto-advance; the manual "Run Bot Turn" control is removed;
  `bot_vs_bot` autoplay swaps its fixed `setTimeout` for scheduler pacing;
  skip/fast-forward is always available; pause/stop satisfies WCAG 2.2.2;
  input never hard-blocks (acting mid-animation flushes to settle, then
  submits).
- **Replay integration**: stepping interrupts via the scheduler's single flush
  path; replay playback uses the same pacing machinery (UI-INTERACTION §14
  pause/speed/reduced-motion).
- **Dev-mode coverage diagnostics**: a dev-only settle assertion comparing the
  post-animation DOM against the authoritative view, flagging missing effect
  coverage — state diffs as diagnostics only, per §10/§11.
- **Lift-ready amendments** (§10 below): UI-INTERACTION §10 scheduler
  acceptance criteria, a new §10A turn-orchestration/pacing doctrine, §19
  acceptance rows, a presentation-shape governance paragraph (brainstorm P4),
  and a `templates/GAME-UI.md` adoption-status row.
- **Closeout bookkeeping**: flip this spec's `specs/README.md` row to `Done`
  with evidence, and edit
  `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` to mark
  P1 (C1 + C2 + the absorbed C8 share) and the P4 register **Done**, each with
  a pointer to this spec (§12).

### 4.3 Out of scope / non-goals

| Non-goal | Status | Reason |
|---|---:|---|
| Action-tree restructuring / path-encoding changes (staged multi-target encoding, brainstorm C3) | Forbidden here; dormant ADR trigger recorded in §13 | Replay-semantics change requiring ADR per FOUNDATIONS §13. The composer remains the sanctioned answer. |
| Visibility-contract moves (e.g. EF undrawn count, brainstorm C4) | Forbidden here; evidence-gated ADR trigger recorded in §13 | FOUNDATIONS §13 ADR trigger; EF-VIS-002 is a deliberate stance. Redacted effects animate generically — they never reveal. |
| Canvas / PixiJS / any renderer replacement | Forbidden | React + SVG is the v1 default; replacement needs profiling evidence or ADR (FOUNDATIONS §7, UI-INTERACTION §4). This spec's motion stays within SVG/CSS/WAAPI capability. |
| New animation library dependency (GSAP, Motion, react-spring) | Not by default | Raw WAAPI + FLIP covers the restrained motion doctrine without inverting control toward render-state; adopting a library is a recorded decomposition decision if implementation shows real pressure (A2). |
| New Rust effect *semantics* or wholesale effect redesign | Out of scope | Animation rides the existing effect stream. Where a motivating adopter lacks an effect for a visible transition, adding that effect is ordinary per-game work via the documented fixture/trace migration path (carries actconmat A5 forward); anything broader stops and reassesses. |
| Bot decision changes; bot pacing in Rust | Forbidden | Orchestration changes *when `run_bot_turn` is called and how its effects render*, never how bots decide (AI-BOTS unchanged). Wall-clock pacing stays out of Rust; seed derivation and command order are unchanged. |
| `engine-core` changes | Forbidden | The effect envelope is already an opaque kernel contract; animation vocabulary belongs in UI law and `apps/web` (FOUNDATIONS §3). |
| `game-stdlib` helpers | Deferred | Presentation shapes are not atlas pressure; the P4 governance paragraph (§10) records this so audits stop re-litigating it. |
| Sound / haptics | Deferred | Separate sensory channel; nothing here blocks it. |
| Catalog & setup visual redesign (brainstorm P2: card art, picker layout, variant descriptions) | Deferred | Named successor spec (§13). |
| Full history-browsing / time-travel UX for the effect log | Deferred | This spec re-bases `EffectLog` on burst grouping and stops there; click-to-inspect past states waits for replay-viewer product attention (brainstorm P3 verdict). |
| FOUNDATIONS amendment / new ADR | Not needed | Verified against `docs/FOUNDATIONS.md` (read in full): this spec *implements* §7/§11 doctrine inside existing law. The two ADR-shaped items are explicitly deferred with named triggers (§13). |
| YAML / DSL / behavior-bearing data | Forbidden | FOUNDATIONS §5. Pacing/dwell constants are TS presentation policy, not hand-authored behavior data (A8). |

---

## 5. Foundation and boundary alignment

| Authority | Constraint engaged | Alignment |
|---|---|---|
| `docs/FOUNDATIONS.md` §1 (priority order) | Polished public playable site first | This is the single highest-leverage "premium table" investment left: it upgrades every minute of play in all 14 games. |
| `docs/FOUNDATIONS.md` §2 (behavior authority) | Rust owns effects, bots, replay; TS presents | The scheduler consumes the existing viewer-filtered effect stream and the existing `runBotTurn` API. TS decides *when to call and how to draw* — presentation policy — never what happens. No legality, no outcome, no effect content is computed in TS. |
| `docs/FOUNDATIONS.md` §3 (`engine-core` noun-free) | No kernel change | The effect envelope stays opaque to the kernel; animation vocabulary lives in `apps/web` and UI law. |
| `docs/FOUNDATIONS.md` §7 / §11 (effect-driven animation, settle-to-view) | Whole spec | This spec exists to implement these invariants. Renderer state diffs appear only in the dev-mode coverage diagnostic, exactly as §7 sanctions. |
| `docs/FOUNDATIONS.md` §12 (stop conditions) | "animation depends on guessed state diffs instead of Rust effects" | The scheduler's input type is the effect stream alone; the dev diff-assertion is diagnostics-only and excluded from normal animation authority. Every decomposed ticket carries this stop condition. |
| Determinism (`docs/TESTING-REPLAY-BENCHMARKING.md` §4/§5) | Replay/hash/trace stability | Wall-clock never enters Rust. Command submission order, bot seed derivation, and serialization are byte-identical; traces and replays are unchanged. Animation is droppable presentation over an unchanged command log. |
| `docs/UI-INTERACTION.md` §10/§11/§14 | Scheduler requirement list, settle rule, replay UI | Implemented in full; §10 gains acceptance criteria and §10A gains the orchestration doctrine via the precedented lift path. |
| `docs/UI-INTERACTION.md` §2 (restrained satisfying motion) | Visual direction | Motion is brief, purposeful, and tone-keyed; no casino flourish, no parallax, no attention-grabbing loops. |
| `docs/AI-BOTS.md` | Bot law | Untouched. Bots still decide through the same API with the same seeds; orchestration is presentation timing around the existing call. |
| `docs/UI-INTERACTION.md` §12 / FOUNDATIONS §11 (no leaks) | Redacted effects | Redacted/hidden effects animate with generic viewer-safe presentation (a face-down slide, an unspecified stir) derived only from what the filtered stream already says; animation introduces no new payload category and no DOM leakage. |
| `docs/IP-POLICY.md` | Original motion design | All motion/easing/iconography is original; no trade-dress mimicry (no proprietary game's signature animations). |
| `docs/OFFICIAL-GAME-CONTRACT.md` §10/§12 | Per-game UI metadata, catalog docs | Adoption rows ride the established audit-row convention; `apps/web/README.md` updates ride the existing closeout checklist. |

**Foundations-amendment verdict (the operator asked):** none required. The
constitution already mandates this work; all amendments are area-doc and
template strengthenings on the precedented lift path. The two ADR-shaped
deferrals (C3, C4) are already correctly gated by FOUNDATIONS §13 and are
recorded as dormant triggers in §13 below.

---

## 6. Committed design decisions

### D1 — The scheduler owns advancement and consumes only the effect stream

A single ordered queue per resolution burst (BGA `notifqueue` model). Input is
the existing `EffectEntry[]` from `getEffects` — nothing else; a combination
the effect stream did not state cannot be animated. Each step is an async
handler returning a promise; the queue advances on resolution plus a declared
minimum dwell (`setupPromiseNotifications` pattern). **All** play-path timing
flows through this one manager — the `bot_vs_bot` `setTimeout` and any future
ad-hoc waits are migrated into it, because timing that escapes the manager is
the documented cause of broken skip/fast modes.

### D2 — React owns settled state; animation is a transient presentation layer

The authoritative public view applies to React state on arrival (state is
always the settled truth, never held hostage by motion). The scheduler drives
presentation on top: WAAPI (`Element.animate`) on transform/opacity of SVG
`<g>` wrappers with `transform-box: fill-box`, FLIP measurement for moves, and
a ghost/overlay layer for cross-container transitions (the BGA clone pattern).
No new animation dependency by default (A2). Reverse-direction staging (board
shows pre-effect state until its step plays) is permitted *within* a burst via
the overlay layer only — the underlying DOM always reflects the authoritative
view, so an interrupt/flush at any instant is correct by construction.

### D3 — Grouping is explicit, never inferred

Sequential vs. simultaneous presentation derives from effect kinds, burst
boundaries, and per-game registrations — declared structure over the stream
(bgio-effects timeline model), never renderer inference from state diffs.
Reveal batches (simultaneous commitments, multi-seat scoring) play as one
grouped step. Redacted effects get the generic redacted presentation (D8).

### D4 — Skip, fast-forward, and interruption are one flush path

Skip = flush: `getAnimations().forEach(a => a.finish())`, drain the queue's
remaining steps instantly, settle to the latest view — a single code path used
by the skip control, by acting mid-animation (input never hard-blocks: the
flush runs, then the action submits — the Hearthstone rule), by replay
stepping, and by teardown. Fast-forward = a global rate scale applied through
the manager. `cancel()` is never used to skip (it discards end states).

### D5 — Reduced motion is equivalence, not absence

Under reduced motion (system query or the existing persisted override) the
scheduler collapses every step to instant transition plus brief non-motion
feedback (highlight/fade), preserving the dwell pacing of orchestration at the
existing fast values. Equivalence rule: every fact motion conveys must exist
as text (turn report, effect log, status line) — already true today and kept
true. This satisfies WCAG 2.3.3 and keeps the GAME-UI reduced-motion rows
honest.

### D6 — Turn orchestration: auto-advance with authored dwell

A shell-level orchestration state machine, driven by the scheduler: after a
human action's burst settles, if the active seat is a bot, its turn runs after
an authored dwell and its burst animates; consecutive bot turns chain;
bot-first starts auto-advance without a click. The manual "Run Bot Turn"
control is removed. `bot_vs_bot` autoplay becomes the same machinery with a
running/paused flag. Pacing is per-effect-type dwell (a reveal lingers, a
counter tick is quick), not one global speed — the comprehension lesson from
§3. A pause/stop control is always reachable (WCAG 2.2.2), and skip remains
live throughout. Determinism: `runBotTurn` is invoked with the same seed
derivation against the same state as today — only *when it is called within
the frame timeline* changes, so command logs, traces, and replays are
byte-identical.

### D7 — Settle-to-view with a dev assertion

After every drain, flush, or interruption the renderer settles to the latest
viewer-safe public view (UI-INTERACTION §11). A dev-only assertion compares
post-settle rendered state against the authoritative view and reports missing
effect coverage (lingering ghosts, unanimated transitions) — diagnostics only,
excluded from public builds per the §13 inspector boundary.

### D8 — Every game gets baseline motion; adopters get authored motion

Generic presentations keyed to the existing `feedbackForEffect` tone taxonomy
(`movement` → FLIP/slide where targets are registered, else highlight;
`turn` → actor-banner transition; `terminal` → outcome settle; `neutral` →
brief highlight; redacted → generic face-down/stir treatment) give all 14
catalog games baseline motion through the shared surfaces with zero per-game
code. Games then record an adoption row: `adopt` (registered game-specific
effect→animation mappings — `event_frontier`, `flood_watch` here),
`board-native mapping`, `generic-only`, or `not applicable` with rationale —
the same audit-row convention the predecessor specs proved.

### D9 — One burst definition, three consumers

A shared burst-segmentation module defines "resolution burst" once (effects
between player decision points, split per bot turn / automated phase).
`TurnReportPanel` drops its last-6 heuristic for real burst grouping;
`EffectLog` renders grouped, labeled bursts as browsable history (the absorbed
C8 residue); the scheduler plays bursts. No new data crosses the boundary.

### D10 — Future-binding

A new official web-exposed game is not UI-complete unless its semantic effects
drive animation through the shared scheduler (or a recorded board-native/
not-applicable row), reduced-motion equivalence holds, automated advances play
on the orchestration timeline with skip/pause, and the renderer settles to the
public view. Lands as UI-INTERACTION §10/§10A/§19 and `templates/GAME-UI.md`
amendments (§10).

---

## 7. Deliverables

```text
apps/web/src/animation/scheduler.ts            (queue, manager-owned timing, flush/rate, reduced-motion collapse)
apps/web/src/animation/bursts.ts               (shared burst segmentation — scheduler, TurnReportPanel, EffectLog)
apps/web/src/animation/presenters.ts           (WAAPI/FLIP helpers, ghost overlay, generic tone-keyed presentations)
apps/web/src/animation/registry.ts             (per-game effect→animation registration surface)
apps/web/src/main.tsx                          (orchestration state machine; bot decoupling; autoplay/replay on scheduler)
apps/web/src/state/shellReducer.ts             (orchestration/pacing state; pause/skip)
apps/web/src/components/ModeControls.tsx       (manual bot trigger removed; pause/skip/speed controls)
apps/web/src/components/TurnReportPanel.tsx    (re-based on shared bursts)
apps/web/src/components/EffectLog.tsx          (burst-grouped browsable history)
apps/web/src/components/EventFrontierBoard.tsx (adopter registrations: reckoning bursts, card transitions, score/funds)
apps/web/src/components/FloodWatchBoard.tsx    (adopter registrations: flood phases, environment automation)
apps/web/src/components/ReplayViewer.tsx       (stepping interrupts via flush; pacing controls)
apps/web/src/styles.css                        (overlay layer, highlight/fade treatments, reduced-motion styles)
apps/web/e2e/animation.smoke.mjs               (new smoke: animate-and-settle, skip, input-not-blocked, replay interrupt, reduced-motion equivalence)
apps/web/e2e/*.smoke.mjs                       (existing game smokes updated for auto-advancing bot turns)
apps/web/package.json                          (wire animation.smoke.mjs into the hand-maintained `smoke:e2e` command chain)
games/*/docs/* or spec adoption matrix         (one adoption row per catalog game)
docs/UI-INTERACTION.md                         (lift-ready amendments applied at closeout — §10 below)
templates/GAME-UI.md                           (adoption-status row)
apps/web/README.md                             (Shell Surface: scheduler/orchestration; Smoke Layers: animation smoke)
specs/README.md                                (index row maintained: Planned → Done with evidence)
brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md  (P1/P4 marked Done at closeout)
```

No Rust changes are expected. If an adopter needs a missing effect for a
visible transition, that game's effect addition rides the ordinary
unit/fixture/golden-trace/serialization update path (§4.3).

---

## 8. Work breakdown (candidate AGENT-TASK decomposition)

| # | Item | Depends on |
|---|---|---|
| WB1 | Burst-segmentation module + `TurnReportPanel`/`EffectLog` re-base (D9); unit tests for burst boundaries across human/bot/automated advances | — |
| WB2 | Scheduler core (D1, D4, D5): queue, promise steps, manager-owned timing, flush/skip, rate scale, reduced-motion collapse, settle hooks; deterministic unit tests with fake timers | WB1 |
| WB3 | Presentation layer (D2, D8): WAAPI/FLIP helpers, ghost overlay, SVG transform discipline, generic tone-keyed presentations; registry surface for per-game mappings | WB2 |
| WB4 | Orchestration in `human_vs_bot` (D6): decouple bot turn from the human frame, auto-advance with dwell, remove "Run Bot Turn", waiting/acting status copy; skip + pause controls; byte-identity proof (command log/trace comparison before/after) | WB2 |
| WB5 | `bot_vs_bot` autoplay + replay stepping on the scheduler (D4, D6): `setTimeout` removed, pause/speed, replay-step flush; UI-INTERACTION §14 conformance | WB2, WB4 |
| WB6 | `event_frontier` adoption: registered animations for reckoning bursts, card transitions, funds/score changes, map highlights; adoption row | WB3 |
| WB7 | `flood_watch` adoption: flood phases, environment-automation bursts; adoption row | WB3 |
| WB8 | Catalog sweep: generic-presentation verification + adoption row (`generic-only` / `board-native` / `not applicable`) for the remaining 12 games; dev settle-assertion (D7) | WB3 |
| WB9 | Animation smoke suite: animate-and-settle, skip mid-burst, act mid-animation (input not blocked + flush correctness), replay-step interrupt, reduced-motion equivalence (all facts present as text, play not blocked); wire `e2e/animation.smoke.mjs` into the `smoke:e2e` chain in `apps/web/package.json`; update existing game smokes for auto-advancing bots | WB4–WB8 |
| WB10 | Closeout: lift §10 amendments into `docs/UI-INTERACTION.md` + `templates/GAME-UI.md`; `apps/web/README.md`; flip `specs/README.md` to Done with evidence; mark brainstorm P1/P4 Done | WB1–WB9 |

Estimated size: 8–12 tickets (comparable to either predecessor spec;
re-grounded by `/reassess-spec` before decomposition).

---

## 9. Exit criteria

1. In every catalog game, a human action's consequences animate from semantic
   effects and the renderer settles to the latest viewer-safe public view;
   no animation is driven by state diffs (the dev assertion exists and is
   diagnostics-only).
2. In `human_vs_bot`, bot turns auto-advance on the animation timeline with
   visible pacing — no manual "Run Bot Turn" control exists; bot-first starts
   and consecutive bot turns advance without clicks; the actconmat O5/O11
   findings are closed (automated phases narrate *and* animate; waiting states
   read as turns in progress, not breakage).
3. Skip is always available and instantaneous; acting mid-animation never
   blocks input — the in-flight burst flushes to the settled view and the
   action submits; replay stepping interrupts cleanly through the same path.
4. `bot_vs_bot` autoplay and the replay viewer pace through the scheduler
   (no raw `setTimeout` in the play path), with pause/stop and speed control
   (WCAG 2.2.2 satisfied).
5. Reduced motion (system or override) preserves every fact and the full
   ability to play: instant transitions + non-motion feedback; the smoke
   proves equivalence and that play is not blocked.
6. Redacted/hidden effects animate generically; the no-leak sweeps
   (DOM/a11y/test-ID/storage/log) pass unchanged-or-stronger — animation adds
   no leak surface.
7. Command logs, traces, replays, and hashes are byte-identical to
   pre-orchestration behavior (recorded comparison evidence); `cargo test
   --workspace` and per-touched-game `simulate`/`replay-check`/
   `fixture-check`/`rule-coverage` pass (expected: no Rust changes — run as
   regression proof).
8. Per-game adoption matrix complete: 14 rows, each `adopt` /
   `board-native mapping` / `generic-only` / `not applicable` with rationale;
   `event_frontier` and `flood_watch` carry authored adoptions.
9. Web `smoke:wasm`, `smoke:ui`, `smoke:effects`, `smoke:e2e` (including the
   new animation smoke and updated game smokes) green.
10. Amendments applied (`docs/UI-INTERACTION.md`, `templates/GAME-UI.md`,
    `apps/web/README.md`); `node scripts/check-doc-links.mjs`,
    `node scripts/check-catalog-docs.mjs`,
    `node scripts/check-presentation-copy.mjs`, and
    `bash scripts/boundary-check.sh` pass; `specs/README.md` row flipped to
    Done with evidence; brainstorm P1/P4 marked Done.

### Acceptance evidence

Re-runnable confirmation set (non-gate spec; folded into the criteria above):

- **Web**: `npm --prefix apps/web run smoke:wasm | smoke:ui | smoke:effects |
  smoke:e2e` including `e2e/animation.smoke.mjs`; scheduler/burst unit tests.
- **Determinism**: recorded byte-identity comparison of command logs/replay
  exports for a scripted `human_vs_bot` and `bot_vs_bot` session before/after
  orchestration.
- **Rust (regression only)**: `cargo fmt --all --check`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo test --workspace`; per
  touched game (expected none) the simulate/replay/fixture/rule-coverage set.
- **Docs/boundary**: `node scripts/check-doc-links.mjs`,
  `node scripts/check-catalog-docs.mjs`,
  `node scripts/check-presentation-copy.mjs`, `bash scripts/boundary-check.sh`.

---

## 10. Lift-ready amendment text (applied at WB10, not before)

**`docs/UI-INTERACTION.md` §10 addition (scheduler acceptance criteria):**

```text
The shared scheduler is the single owner of effect-presentation timing: all
play-path animation and pacing flows through it, and ad-hoc timers outside it
are defects. Skip, acting during animation, and replay stepping share one
flush-and-settle path that finishes (never discards) in-flight animation and
renders the latest viewer-safe public view. Reduced-motion mode replaces
motion with instant transitions plus non-motion feedback while preserving
every conveyed fact as text. Redacted effects receive generic viewer-safe
presentation; animation introduces no new payload or leak surface.
```

**`docs/UI-INTERACTION.md` new §10A (turn orchestration and pacing):**

```text
## 10A. Turn orchestration and pacing

Non-interactive advances (bot turns, automated phases, autoplay, replay
playback) play out on the shared animation timeline with authored per-effect
dwell, not as instant state swaps and not behind manual advance triggers.

- Bot turns auto-advance: after the human's effects settle, the bot's turn
  runs and animates; bot-first starts and consecutive bot turns need no click.
- Skip/fast-forward is always available and instantaneous.
- Input never hard-blocks on animation: acting mid-animation flushes the
  timeline to the settled view, then submits.
- Auto-playing sequences expose pause/stop and speed control.
- Reduced-motion mode preserves pacing comprehension through the fast path
  and text narration; it never removes feedback or blocks play.
- Orchestration is presentation policy in TypeScript. It changes when bot and
  automation APIs are called and how results render — never what they decide.
  Wall-clock time stays out of Rust; command logs, traces, replays, and
  hashes are unaffected by pacing.

Repeated presentation shapes across games (per-game `ui.rs` display-metadata
modules, board adapters, effect→animation registrations, presentation-TOML
layouts) are governed by this document and the official-game contract; they
are not mechanic-atlas promotion pressure. Promotion of presentation helpers
into `game-stdlib` is deferred until a third structural divergence between
implementations of the same presentation shape, or an official-game count
above 20, and routes through the atlas ledger at that time.
```

**`docs/UI-INTERACTION.md` §19 additions:**

```text
- semantic effects animate through the shared scheduler (or a recorded
  board-native/not-applicable adoption row), and the renderer settles to the
  latest viewer-safe public view after every burst, skip, or interruption;
- bot turns and automated phases auto-advance on the animation timeline with
  always-available skip and pause; no manual advance trigger exists in normal
  mode;
- acting during animation is never blocked; it flushes to the settled view
  and submits;
- reduced-motion mode conveys every animated fact through non-motion
  presentation and never blocks play.
```

**`templates/GAME-UI.md` — "Semantic effect-to-animation mapping" section
addition (adoption-status row, mirroring the audit-row convention):**

```text
Scheduler adoption status: `adopt` (game-registered effect→animation
mappings) / `board-native mapping` (recorded alternative) / `generic-only`
(shared tone-keyed presentations suffice) / `not applicable` (rationale).
Orchestration adoption: auto-advance and skip verified for this game's bot
and automated phases.
```

---

## 11. Forbidden changes

- No legality, action-tree, path-encoding, validation, scoring, or visibility
  changes; submitted command bytes, seeds, traces, replays, and hashes stay
  identical.
- No animation driven by state diffs, renderer inference, or TS-guessed
  causality (FOUNDATIONS §12 stop condition); the dev diff-assertion stays
  diagnostics-only and out of public builds.
- No `engine-core` edits; no `game-stdlib` additions; no new wasm-api payload
  categories (the effect stream as-is is the input).
- No bot policy, seed-derivation, or decision changes; orchestration touches
  call timing and rendering only.
- No Canvas/PixiJS or renderer replacement; no new animation library without
  a recorded decomposition decision.
- No behavior-looking fields in static data; no YAML; no DSL. Pacing
  constants are TS presentation code.
- No hidden-information exposure through animation: redacted effects animate
  generically; no leak surface added to DOM, test IDs, storage, or logs.
- No weakening or deletion of existing tests/guards (AGENT-DISCIPLINE §4);
  the predecessor specs' guards stay intact; existing game smokes are updated
  for auto-advance, never removed.
- No removal of the reduced-motion override or regression of accessibility
  baselines (UI-INTERACTION §16/§17 facts stay text-available).

---

## 12. Documentation updates required

- `docs/UI-INTERACTION.md` — §10 acceptance criteria, new §10A orchestration/
  pacing doctrine + presentation-shape governance paragraph, §19 rows (§10
  above; lifted at WB10).
- `templates/GAME-UI.md` — adoption-status row (§10 above).
- `apps/web/README.md` — Shell Surface (scheduler, orchestration, burst
  grouping); Smoke Layers (animation smoke).
- `specs/README.md` — index row added now (`Planned`); flipped to `Done` at
  WB10 with evidence.
- `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` — at
  WB10 closeout, mark the feature clearly done: P1 (§4) gets a
  **Status: DONE** line naming this spec; the §5 sequence table rows 1, 3,
  and 4 (P1 spec, C8 residue, P4 register) are marked Done with the same
  pointer; §8 next-step 1 is marked complete. P2's row is untouched (it
  remains the next candidate spec).

---

## 13. Sequencing

- **Predecessor:** `action-consequence-and-match-context-shared-surfaces`
  (Done, archived 2026-06-12). It built the viewer-filtered narration
  surfaces, authored labels, and turn report this spec animates, and
  explicitly deferred both of this spec's workstreams (its §4.3 "Deferred"
  rows for animation and auto-run bot turns).
- **Admission:** Non-gate UI-infrastructure spec; no mechanic-ladder gate is
  blocked; `docs/MECHANIC-ATLAS.md` carries no open promotion debt. Gate P
  (private, optional) remains subordinate to public polish (FOUNDATIONS §1).
- **Successor (named):** catalog & setup visual redesign (brainstorm P2 —
  per-game card art/iconography, picker layout, variant descriptions, richer
  setup framing; IP-POLICY originality checks per asset). No dependency on
  this spec; may be authored in parallel after this spec is accepted.
- **Dormant ADR triggers (recorded here so future audits don't re-litigate):**
  - **C3 — staged multi-target action encoding.** Write the FOUNDATIONS §13
    ADR (per-stage command encoding, replay/trace migration, fixture rebuild)
    only when a game's multi-target legal-leaf enumeration becomes a
    *measured* problem: leaf counts in the hundreds per stage, or a
    bench-lane regression attributable to leaf explosion. Until then the
    presentation composer is the sanctioned answer.
  - **C4 — visibility-contract moves (e.g. EF undrawn count).** A deliberate
    design stance (EF-VIS-002), not debt. Revisit only on playtest evidence
    of player confusion; the change is then a small ADR plus fixture/no-leak
    sweep updates.

---

## 14. Assumptions (one-line-correctable)

1. **(A1) One fused spec** — assuming animation scheduler + turn
   orchestration ship together because pacing is implemented *by* the
   timeline; split into siblings if a smaller first diff is preferred.
2. **(A2) Raw WAAPI, no animation library** — assuming `Element.animate` +
   FLIP + a ghost overlay covers the restrained motion doctrine; if
   implementation shows real pressure (SVG attribute tweens, complex
   choreography), adopting a library is a recorded decomposition decision,
   not silent drift.
3. **(A3) Presentation-shape register home is UI-INTERACTION** — assuming the
   governance paragraph lands in the new §10A (UI law governs presentation
   conventions; the atlas stays behavior-only); move it to a
   `docs/MECHANIC-ATLAS.md` register instead if a tabular register is
   preferred — note that `docs/MECHANIC-ATLAS.md` already uses `§10A` for its
   open-promotion-debt register, so the atlas fallback would need its own
   (non-§10A) heading, whereas `docs/UI-INTERACTION.md` has a clean §10→§11 gap
   for the new §10A.
4. **(A4) Effect-stream sufficiency** — assuming per-game effect coverage is
   rich enough to animate (verified for EF's Reckoning burst via the turn
   report); where sparse, the fix is a Rust-side effect addition through the
   ordinary migration path, not TS inference (carries actconmat A5 forward).
5. **(A5) Manual bot trigger is removed outright** — assuming auto-advance +
   always-available skip/pause makes the "Run Bot Turn" control redundant
   with no replacement setting; reinstate as an opt-in "manual pacing" toggle
   if user testing wants it.
6. **(A6) Research basis** — fresh practitioner/standards pass run 2026-06-12
   (BGA framework, WAAPI/MDN, FLIP, bgio-effects, Hearthstone, WCAG 2.2,
   DiGRA 2015) via built-in web search after the session's mandated mgrep web
   tool had exhausted its quota earlier the same day; no dedicated academic
   sweep beyond the DiGRA anchor was run — commission `research-brief` if
   deeper academic grounding is wanted before decomposition.
7. **(A7) Scheduler lives in `apps/web/src/animation/`** — assuming a shared
   shell module (names indicative throughout §7), not per-board code and not
   a separate package; relocate during decomposition if the shell layout
   prefers otherwise.
8. **(A8) Pacing constants are TS presentation policy** — assuming dwell
   times/easings live in TypeScript (they are presentation, FOUNDATIONS §2),
   not in static data files and not in Rust; no per-game authored pacing data
   unless an adopter proves the need, and then as inert typed content.
9. **(A9) Effort is spec-sized analogy** — 8–12 tickets, sized against the
   two predecessor specs, not measured; `/reassess-spec` re-grounds it before
   decomposition.

---

## Outcome

Completed on 2026-06-12 through archived tickets `EFFANITUR-001` through
`EFFANITUR-010`.

Implemented the shared burst segmentation, scheduler core, presentation
registry, human-vs-bot orchestration, bot-vs-bot scheduler pacing, Event
Frontier and Flood Watch authored animation adoption, catalog sweep, dev
settle assertion, animation smoke wiring, and closeout documentation lift.

Verification evidence included:

- `npm --prefix apps/web run smoke:e2e` including `e2e/animation.smoke.mjs`;
- `npm --prefix apps/web run smoke:animation`;
- `npm --prefix apps/web run smoke:wasm`;
- `npm --prefix apps/web run smoke:ui`;
- `npm --prefix apps/web run smoke:effects`;
- `node scripts/check-doc-links.mjs`;
- `node scripts/check-catalog-docs.mjs`;
- `node scripts/check-presentation-copy.mjs`;
- `bash scripts/boundary-check.sh`.
