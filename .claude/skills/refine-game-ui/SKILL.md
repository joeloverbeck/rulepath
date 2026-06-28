---
name: refine-game-ui
description: "Use to iteratively improve a web-exposed game's interface so the human player has all the viewer-safe information they need at every stage. Designed to be driven by /loop (e.g. `/loop 10m /refine-game-ui starbridge_crossing`): each loop fire is one refinement iteration that reads the game's player-facing docs to build a per-stage information model, plays the game in the web shell via Puppeteer, finds interface/information deficiencies against docs/UI-INTERACTION.md, fixes presentation-only gaps in TypeScript/React (committing each unit to main), and coordinates across fires through a /tmp journal. Stops the loop when a finding is spec-worthy (a behavioral bug, a need for new Rust view data, or a shared-surface UI policy). Ends the loop after two consecutive iterations with no meaningful interface improvement. Produces: presentation-only commits on main, an updated /tmp/<slug>-ui-vN.md journal, and (on a spec-worthy finding) a new specs/* file. Mutates: apps/web presentation + UI smoke tests, the journal, optionally specs/."
user-invocable: true
arguments:
  - name: game
    description: "The game to refine: a games/* folder name (e.g. starbridge_crossing) or its hyphen slug (e.g. starbridge-crossing)."
    required: true
---

# Refine Game UI

Drive one web-exposed game through the app and iteratively make its interface the
best, most useful surface for a human player — giving them every piece of
**viewer-safe** information they need or want at every stage of play. Built to run
under `/loop`: **each loop fire performs exactly one refinement iteration**, picks
up where the last fire left off via a `/tmp` journal, and either continues, stops
the loop (a spec is needed), or ends the loop (two iterations with no meaningful
improvement).

```
/loop 10m /refine-game-ui <game>
```

This is the sibling of `playtest-game`. That skill hunts behavioral **bugs** and
fixes them in Rust via TDD. This skill hunts **interface and information
deficiencies** and fixes them in the presentation layer. When this skill hits an
actual behavioral bug — or any deficiency that needs new Rust data or design — it
does **not** fix it here: it writes a spec and stops the loop.

<HARD-GATE>
These hold on every iteration and are never relaxed, including under auto/loop operation:

- **No hidden-information leaks — ever.** The goal is "all the information the
  player needs," but that means **only viewer-safe** information. You MUST NOT
  close an information gap by exposing hidden state through visible text, hidden
  DOM text, accessibility labels, `data-testid`s, CSS classes, storage, logs,
  effect logs, replay exports, dev panels, or bot explanations. For multi-seat /
  hidden-information games, respect pairwise seat-private redaction
  (`FOUNDATIONS §11`, `UI-INTERACTION §12`). If the info you want to show is
  hidden from the current viewer, showing it is forbidden — not a fix.
- **TypeScript presents only; Rust owns behavior and data.** Every improvement
  draws **only** from data Rust already projects: public/private views, legal
  action trees, Rust previews, viewer-filtered semantic effects, diagnostics,
  bot explanations, and typed UI metadata. TypeScript MUST NOT decide legality,
  compute a game fact (score, winner, hand strength, winning line, tiebreak), or
  synthesize display text from raw internal IDs. If the information the player
  needs is not in a Rust view/effect/metadata, that is **spec-worthy** (§6), not
  a presentation fix.
- **Align to the UI law.** Every change conforms to `docs/UI-INTERACTION.md` —
  especially §7 (legal-only controls), §12 (hidden-information safety), §16
  (outcome/victory surface), §17 (accessibility baseline), and the §19
  acceptance check — and to `FOUNDATIONS §7` (public UI is central product work)
  and `§11` (universal invariants). When in doubt, **stop and reassess rather
  than generalize** (`FOUNDATIONS §12`).
- **Never emit a false loop-completion signal** to escape the loop. Only declare
  a stop/end when its condition is genuinely and unequivocally true (see §6).
- **Deliver complete files or coherent complete sections, not diffs.**
</HARD-GATE>

## Before anything: read the law and build the information model

On the **first** iteration of a run (and after any context reset), read enough to
refine correctly:

- `docs/UI-INTERACTION.md` — the primary law for this skill (public UI target,
  ownership split, browser payload rules, legal-only controls, previews,
  effect-driven animation, outcome surface, accessibility, and the §19 UI
  acceptance check).
- `docs/FOUNDATIONS.md` — §2 (behavior authority), §7 (public UI), §11
  (universal invariants, including the no-leak list), §12 (stop conditions).
- `docs/AGENT-DISCIPLINE.md` — bounded tasks, forbidden changes.
- The game's **player-facing** docs under `games/<folder>/docs/*` — at minimum
  `HOW-TO-PLAY.md`, `UI.md`, `COMPETENT-PLAYER.md`, and `RULES.md`/`MECHANICS.md`.
  Read until you can answer, for each stage of play, **what a competent player
  needs to know and whether that information is viewer-safe**.

From these, build a **per-stage information model** — the spine of every
iteration. For each stage, list the viewer-safe facts a competent player needs:

- **Setup / picker:** what game, variant, seats, who plays whom, how to start,
  where the rules/How-to-Play surface is.
- **On-turn:** whose turn, what the legal moves are (obvious, legal-only), the
  current state the player must read to choose (score, resources, phase, board),
  any reserved cost/consequence metadata on choices.
- **Mid-compound-action:** the staged choices so far, the Rust preview of cost /
  effects / next legal choices, how to confirm or cancel.
- **After a bot / automated move:** what just happened, narrated near the board
  from viewer-filtered effects in authored vocabulary; a "why?" affordance for
  non-random bots.
- **Terminal:** the shared outcome surface — final result, decisive cause, every
  player's viewer-safe final standing, per-player breakdown, rule references.

Record in the journal that you've read the law + docs and capture the
information model so later iterations don't re-derive it. If a later fire finds
the journal already records `Law + docs read: yes` **and** the information model
is still in your context, trust it — re-read only when the journal is absent, or
when your context was actually reset/compacted. (A fresh `/loop` fire that
retained context is not a reset.)

## Process flow (one iteration = one loop fire)

```
Resolve game + slug
        |
        v
Locate / create the journal  (continue this run, or start a fresh version)
        |
        v
[first iteration only] Read law + player docs; build the per-stage information model
        |
        v
Serve the web app + drive it with Puppeteer through full play (incl. terminal)
        |
        v
At each stage: compare rendered interface vs. information model + §19 checklist
        |
        v
Finding? --spec-worthy?--yes--> write specs/* + STOP THE LOOP
        |          |
        |          no--> fix presentation-only in TS/React; verify; commit to main
        v
Update the journal (stages reviewed, findings, fixes, meaningful-improvement count)
        |
        v
Two iterations with no meaningful improvement? --yes--> END THE LOOP
        |
        no--> let the next loop fire run the next iteration
```

### 1. Resolve the game and slug

Accept either a `games/*` folder name (`starbridge_crossing`) or its hyphen slug
(`starbridge-crossing`). Resolve to the actual `games/<folder>/` directory; if it
doesn't exist, list `games/` and stop with a clear error. Derive the **slug** by
replacing `_` with `-` — the slug is used for the journal filename and matches the
docs/specs naming convention.

### 2. Locate or create the journal

The journal coordinates work across loop fires. It lives at `/tmp/<slug>-ui-vN.md`
— note the `-ui-` infix, which keeps it distinct from `playtest-game`'s
`/tmp/<slug>-vN.md` so the two loops never collide on the same game.

- List `/tmp/<slug>-ui-v*.md` and find the highest `N`.
- **None exists** → create `/tmp/<slug>-ui-v1.md` with `Status: ACTIVE`.
- **Highest is `Status: ACTIVE`** → this is the current run's journal; **append**
  this iteration to it.
- **Highest is `Status: ENDED` or `Status: STOPPED`** → the previous run
  finished; this is a fresh run, so **bump the version**: create
  `/tmp/<slug>-ui-v{N+1}.md` with `Status: ACTIVE`.

This makes the version bump happen once per fresh `/loop` invocation, while
iterations within one loop accumulate in a single document.

**Journal format:**

```markdown
# Refine-UI journal — <slug> — v<N>
Status: ACTIVE            # ACTIVE | STOPPED | ENDED
Game folder: games/<folder>
Loop cron job: <id-or-none>   # fixed-interval /loop cron id, recorded on the first fire so any later fire can cancel it on stop/end
Iterations with no meaningful improvement: 0
Law + docs read: yes/no

## Information model (viewer-safe facts the player needs, by stage)
- Setup: <...>
- On-turn: <...>
- Mid-action: <...>
- After bot/automated move: <...>
- Terminal: <...>

## Iteration <k> — <what was reviewed>
- Stages reviewed: <e.g. "setup, on-turn, terminal — 1 full game seat A vs bot">
- Findings: <list with stage + deficiency, or "none">
- Presentation fixes: <one line per fix + commit short-sha + screenshot name/path — MCP `puppeteer_screenshot` returns the image inline, so reference it by the `name` you gave it, or save the data URI to the scratchpad when a real path is wanted>
- Spec-worthy: <none | spec path + what it covers>
- Meaningful improvement this iteration? YES | NO
- Notes for next iteration: <anything the next fire needs>
```

### 3. Serve and drive the web app

- Build the web shell (`npm --prefix apps/web run build`) and serve `apps/web/dist`
  on `127.0.0.1`. `npm --prefix apps/web run preview` works in principle, but in
  this repo's base-path config `vite preview` has been observed to return the SPA
  `index.html` fallback (content-type `text/html`) for hashed JS/CSS/`.wasm`
  assets — the module script then never executes and the React root stays empty
  (a blank page, no `window.render_game_to_text`). The reliable mechanism is the
  same static server the e2e smoke embeds: serve `dist` with per-extension MIME
  types (`.wasm` → `application/wasm`, `.js` → `text/javascript`, …) and an
  `index.html` fallback for unknown paths. After serving, **confirm the app
  actually mounted** before driving — e.g. `typeof window.render_game_to_text ===
  "function"` and the catalog renders — not just that the server returns 200.
- Reuse a running server across iterations if it's already up — but first confirm
  it is actually serving Rulepath (check the page title / catalog), not an
  unrelated dev server squatting on a shared port. Reuse confirms *identity*, not
  *currency*: if any source changed since the server started (a fix landed between
  fires, or you rebuilt `dist`), rebuild and restart before driving, because a
  stale server can re-confirm an already-fixed gap or fake a pass.
- **Lean on the game's e2e smoke.** If the game ships
  `apps/web/e2e/<slug>.smoke.mjs`, run it and read it to learn stable selectors
  (e.g. `data-*` ids, legal-target classes) and the start-match flow. It is the
  floor for interface coverage, not the ceiling.
- Drive it with the Puppeteer MCP tools (`puppeteer_navigate`, `_click`,
  `_select`, `_fill`, `_screenshot`, `_evaluate`). Navigate the catalog, select
  **this** game, and play through **every stage** in the information model to a
  terminal state — at least one full game, plus targeted setups (e.g. start
  mid-compound-action; reach a showdown/terminal) as needed to exercise each
  stage.
  - **Read after render, never in the same call as a click.** React re-renders
    asynchronously; a `puppeteer_evaluate` that clicks and then reads the DOM in
    the *same* call returns **stale** state. Click in one call; read in a
    separate call (or await a tick / `waitForFunction`).
  - **Reaching terminal / bot stages.** Use the shell's **Bot vs bot** mode +
    **Start Autoplay** to reach terminal and to review the after-bot-move and
    outcome stages. Do not use a foreground `sleep` to wait (harness-blocked) —
    poll inside `puppeteer_evaluate` with a `Promise`/`setTimeout` loop on the
    Rust view status, or run a background task.
- **At each stage, compare the rendered interface against the information model
  and the `UI-INTERACTION.md §19` acceptance check.** Screenshot each stage into
  the journal's evidence trail (MCP `puppeteer_screenshot` returns the image
  inline — give each a stable `name` and reference that name in the journal, or
  save the data URI to the scratchpad when you want a resolvable path). Look for
  deficiencies such as:
  - viewer-safe information the player needs that is **missing, buried, or
    ambiguous** (score, phase, whose turn, resources, legal-move clarity,
    reserved cost/consequence metadata, what a bot just did, the outcome
    breakdown);
  - illegal moves clickable in normal mode, or legal moves not obvious;
  - compound actions not using progressive construction / previews;
  - a missing or thin outcome surface (§16), or missing How-to-Play/rules surface
    reachable from picker, setup, and in-play (§17);
  - accessibility gaps (§17): color-only encoding, missing accessible names,
    no reduced-motion fallback, missing screen-reader state/legal-action
    summaries, unreachable rules help;
  - raw internal IDs / engine-debug vocabulary in normal-mode surfaces;
  - any hidden-information leak (§12) — this is a **defect to fix toward safety**,
    never something to "complete."

### 3A. Research the best improvement when it is non-obvious (optional, bounded)

Most findings have an obvious fix from the UI law and the information model
(a missing label, a buried score, a raw ID, an unreachable rules surface) — fix
those directly. But when the **best** presentation of a viewer-safe fact is
genuinely unclear (how to organize a dense showdown breakdown, lay out a
multi-seat rail, narrate non-interactive advances under reduced motion, structure
a complex outcome explanation), research what good interfaces do before choosing.

- **How.** Use the mandated web search (`mgrep --web "..."`) for a bounded
  lookup; reserve the `deep-research` skill for a genuinely hard, multi-source
  design question. Draw on web-usability and information-design principles,
  accessibility patterns (WCAG / ARIA), generic board-game UX conventions, and
  relevant research papers.
- **IP boundary — load-bearing.** Extract **general principles and patterns
  only**. You MUST NOT copy a specific commercial product's layout, icons,
  copy, or trade dress (`FOUNDATIONS §10`, `UI-INTERACTION §2` — avoid
  proprietary mimicry and trade-dress imitation). Research tells you *how to
  organize viewer-safe information well in the abstract*, never *which game's
  interface to imitate*. If a pattern is identifiably one commercial game's trade
  dress, it is out of bounds.
- **Viewer-safety still binds.** Research may only change how **already
  viewer-safe** information is presented. No external pattern ever justifies
  surfacing hidden state; a pattern that assumes showing more than Rust
  authorizes for the current viewer is inapplicable here.
- **Bounded, and "too deep" is a signal.** Keep research scoped to the open
  question; do not let it balloon the iteration. A design question that needs
  sustained multi-source synthesis is itself a hint the change is a shared-surface
  UI policy with several viable designs — i.e. **spec-worthy (§6)**, not an inline
  fix.
- **Auditable.** Record the research basis and the principle you applied (with
  source) in the journal's fix rationale, so the design decision is reviewable.

### 4. Disposition of each finding

For each finding, decide **presentation-only** vs **spec-worthy** (§6), then:

**Presentation-only** — the information already exists in a Rust view / effect /
metadata and the gap is purely how it is shown (missing label, poor hierarchy,
buried fact, accessibility wrapper, narration of an existing effect):

1. Confirm the fact is genuinely available from an existing Rust payload — read
   the WASM view / effect / metadata it would come from. If it is **not** there,
   the finding is spec-worthy; do not synthesize it in TypeScript.
2. If the **best** presentation is non-obvious, research it first (§3A). Make
   the change in `apps/web` (TypeScript/React presentation only).
3. **Verify before committing:**
   - add or extend a UI-smoke assertion in `apps/web/e2e/<slug>.smoke.mjs` where
     the improvement is testable (e.g. the fact now appears, the control is
     reachable, the label is a display name not a raw ID);
   - capture **before/after screenshots** into the journal evidence trail and
     confirm the improvement visually, including a reduced-motion / small-screen
     check when relevant;
   - run the gates: `npm --prefix apps/web run smoke:ui` (and `smoke:effects`
     when effects/animation changed) and the web build, plus a DOM no-leak /
     no-raw-ID sweep where the change touched payload-facing surfaces.
4. **Commit each distinct unit of work to `main`** with a focused message. One
   logical improvement = one commit. Stage only the files for this unit; do not
   sweep up unrelated working-tree changes.

**One defect, two surfaces.** A single root-cause deficiency can be
presentation-fixable on one surface yet spec-worthy on another — e.g. a board
panel you can fix from existing view data, while the *same* defect on a shared
surface (seat rail, turn bar, outcome panel) needs new Rust view data. Before
shipping the presentation-only half, check whether it leaves the sibling shared
surface **visibly inconsistent** until the spec lands (e.g. the board reads
"North to move" directly above a shared bar still reading "Seat 1 to act"). If
so, prefer to either (a) ship the standalone half **only** when it is a real
improvement on its own and the spec explicitly records the interim state and the
follow-up migration (the half-fix may become redundant once the Rust change
lands), or (b) fold both halves into the spec when the partial fix would read as
a regression. Record the choice and its rationale in the journal.

**Spec-worthy** (§6) — route to a spec and stop the loop; do **not** hack it.

### 5. Update the journal

Append the iteration entry. Set **`Meaningful improvement this iteration? YES`**
only if you committed at least one change that adds or clarifies player-needed
viewer-safe information, or fixes a `§19` acceptance item — **not** cosmetic
whitespace, churn, or a no-op. Update `Iterations with no meaningful improvement`:
reset to 0 when this iteration was meaningful; increment when it was not.

### 6. Stop conditions

Two distinct terminations — apply whichever fires first:

- **Spec needed → STOP THE LOOP.** Write a spec and stop when a finding is not a
  presentation-only fix. This covers:
  - a behavioral **bug** (wrong legal actions, illegal move accepted, mis-scored
    or mis-detected terminal, desynced/incorrect view, crash, console error,
    non-deterministic or leaking behavior);
  - an **information need that requires new Rust data** — a view field, semantic
    effect, preview datum, or UI metadata the player needs but Rust does not yet
    project (this is a product-behavior change to view projection, not a TS fix);
  - a **shared-surface UI policy** with several viable designs (e.g. a
    cross-game outcome-surface or seat-rail change), where committing to one bound
    by editing a magic constant would be a hack.

  Write the spec under `specs/*` aligned with `docs/**` and the `specs/README.md`
  conventions — grounding any proposed design in `§3A` research where the best
  approach is non-obvious — commit it (with any `specs/README.md` tracker row) as its own unit
  per step 4 above — staging only those spec files — set the journal
  `Status: STOPPED`, record the spec path, and **stop the loop**.

- **Two unproductive iterations → END THE LOOP.** When
  `Iterations with no meaningful improvement` reaches **2**, the interface is
  refined for now: set the journal `Status: ENDED` and **end the loop**.

**How to signal the loop to stop/end:** finish the iteration with an explicit,
unambiguous terminal line as the very last thing you output, e.g.
`REFINE-UI LOOP ENDED: <slug> — two consecutive iterations with no meaningful improvement`
or `REFINE-UI LOOP STOPPED: <slug> — spec written to specs/<file>`. Do not
schedule another iteration (in self-paced `/loop`, omit the next wakeup). If the
loop is a fixed-interval `/loop` that keeps re-firing, the terminal line plus
`Status: ENDED/STOPPED` in the journal tells the next fire to no-op. If you know
the loop's cron job id (recorded as `Loop cron job` in the journal header, or
still in your context from creating it), proactively `CronDelete` it so it stops
firing no-ops; otherwise surface that the user can cancel the interval. Never emit
a terminal line whose condition isn't genuinely met (HARD-GATE).

## What this skill does NOT do

- It does not fix behavioral bugs or add Rust behavior/views — those route to
  `specs/*` and stop the loop.
- It does not decide legality, validation, or any game fact in TypeScript.
- It does not "complete" the player's information by exposing hidden state; the
  information goal is bounded by viewer safety.
- It does not weaken, skip, or delete tests to get green.
- It does not commit a spec-and-stop and a presentation fix in the same breath
  without recording both in the journal.
