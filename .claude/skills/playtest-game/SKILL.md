---
name: playtest-game
description: "Use to shake out bugs in a freshly-implemented game by repeatedly playing it to completion in the web app via Puppeteer and fixing defects through TDD. Designed to be driven by /loop (e.g. `/loop 10m /playtest-game starbridge_crossing`): each loop fire is one playtest iteration that reads game docs, plays full game(s) in the web shell, fixes bugs in alignment with docs/**, commits each unit to main, and coordinates across fires through a /tmp journal. Stops the loop when a defect needs a spec; ends the loop after two consecutive clean iterations. Produces: bug-fix commits on main, an updated /tmp/<slug>-vN.md journal, and (on a spec-worthy finding) a new specs/* file. Mutates: source/tests under fix, the journal, optionally specs/."
user-invocable: true
arguments:
  - name: game
    description: "The game to play-test: a games/* folder name (e.g. starbridge_crossing) or its hyphen slug (e.g. starbridge-crossing)."
    required: true
---

# Playtest Game

Drive one game hard through the web app to surface and fix defects. Built to run under `/loop`: **each loop fire performs exactly one playtest iteration**, picks up where the last fire left off via a `/tmp` journal, and either continues, stops the loop (spec needed), or ends the loop (two clean iterations).

```
/loop 10m /playtest-game <game>
```

<HARD-GATE>
These hold on every iteration and are never relaxed, including under auto/loop operation:

- **TDD, never weaken tests.** Every bug fix is a failing test FIRST, then the code change that makes it pass. Never delete, skip, weaken, or adapt a test to get green — follow the failing-test protocol (`docs/AGENT-DISCIPLINE.md` §4). Never adapt tests to match a bug; fix the code.
- **Rust owns behavior.** Fixes to legality, validation, effects, scoring, terminal detection, RNG, serialization, views, or bot choices go in `engine-core`/`game-stdlib`/`games/*` — never TypeScript. TypeScript/React present only.
- **Stay inside the boundary.** `engine-core` stays generic and noun-free; typed nouns live in `games/*`. No YAML/DSL, no hidden-information leaks, keep replay/hashes/RNG/serialization deterministic. When in doubt, **stop and reassess rather than generalize** (FOUNDATIONS §12).
- **Never emit a false loop-completion signal** to escape the loop. Only declare a stop/end when its condition is genuinely and unequivocally true (see §6).
- **Deliver complete files or coherent complete sections, not diffs.**
</HARD-GATE>

## Before anything: read the law

On the **first** iteration of a run (and after any context reset), read enough of the foundations to fix correctly:

- `docs/FOUNDATIONS.md` — the constitution (priority order, §11 invariants, §12 stop conditions, §13 ADR triggers).
- `docs/AGENT-DISCIPLINE.md` — bounded tasks, forbidden changes, the failing-test protocol.
- The game's own docs under `games/<folder>/docs/*` — at minimum `RULES.md`, `MECHANICS.md`, `HOW-TO-PLAY.md`, and `UI.md`. Read until you are **100% certain** how the game is supposed to behave. If the docs leave a behavior genuinely undetermined, that is a spec-worthy gap (§6), not a guess.

Record in the journal that you've read these so later iterations don't re-derive from scratch. If a later fire finds the journal already records `Law + docs read: yes` **and** its captured rule summary is still in your context, trust it — re-read the foundations and game docs only when the journal is absent, or when your context was actually reset/compacted and that summary is no longer available. (A fresh `/loop` fire that retained context is not a reset.)

## Process flow (one iteration = one loop fire)

```
Resolve game + slug
        |
        v
Locate / create the journal  (decide: continue this run, or start a fresh version)
        |
        v
[first iteration only] Read law + game docs to 100% certainty
        |
        v
Serve the web app + drive it with Puppeteer; play one or more games to completion
        |
        v
Defect found? --yes--> spec-worthy? --yes--> write specs/* + STOP THE LOOP
        |                    |
        no                   no--> fix via TDD; commit each unit to main
        |
        v
Update the journal (games played, bugs, clean/not-clean, consecutive-clean count)
        |
        v
Two consecutive clean iterations? --yes--> END THE LOOP
        |
        no--> let the next loop fire run the next iteration
```

### 1. Resolve the game and slug

Accept either a `games/*` folder name (`starbridge_crossing`) or its hyphen slug (`starbridge-crossing`). Resolve to the actual `games/<folder>/` directory; if it doesn't exist, list `games/` and stop with a clear error. Derive the **slug** by replacing `_` with `-` (`starbridge_crossing` → `starbridge-crossing`) — the slug is used for the journal filename and matches the docs/specs naming convention.

### 2. Locate or create the journal

The journal coordinates work across loop fires. It lives at `/tmp/<slug>-vN.md`.

- List `/tmp/<slug>-v*.md` and find the highest `N`.
- **None exists** → create `/tmp/<slug>-v1.md` with `Status: ACTIVE` (see format below).
- **Highest is `Status: ACTIVE`** → this is the current run's journal; **append** this iteration to it.
- **Highest is `Status: ENDED` or `Status: STOPPED`** → the previous run finished; this is a fresh run, so **bump the version**: create `/tmp/<slug>-v{N+1}.md` with `Status: ACTIVE`.

This makes the version bump happen once per fresh `/loop` invocation, while iterations within one loop accumulate in a single document.

**Journal format:**

```markdown
# Playtest journal — <slug> — v<N>
Status: ACTIVE            # ACTIVE | STOPPED | ENDED
Game folder: games/<folder>
Loop cron job: <id-or-none>   # fixed-interval /loop cron id, recorded on the first fire so any later fire can cancel it on stop/end
Consecutive clean iterations: 0
Law + docs read: yes/no

## Iteration <k> — <what was done>
- Games played: <count + brief outcome, e.g. "2 games to terminal, seat A won both">
- Bugs found: <list, or "none">
- Fixes (TDD): <one line per fix + commit short-sha>
- Verdict: CLEAN | FIXED <n> | BLOCKED
- Notes for next iteration: <anything the next fire needs>
```

### 3. Serve and drive the web app

- **Cheap sweep first.** A quick `simulate`/`replay-check`/`fixture-check`/`rule-coverage` run for the game (the §4 gate commands) is a low-cost way to surface or rule out defects before the slower UI play.
- Build and serve the web shell locally (e.g. `npm --prefix apps/web run build` then `npm --prefix apps/web run preview`, which serves on `127.0.0.1`). Reuse the running server across iterations if it's already up — but first confirm it is actually serving Rulepath (check the page title / catalog), not an unrelated dev server squatting on a shared port.
- **Lean on the game's e2e smoke.** If the game ships `apps/web/e2e/<slug>.smoke.mjs`, run it for fast baseline coverage and read it to learn stable selectors (e.g. `data-*` space ids, legal-target classes) and the start-match flow before manual exploration. The smoke is the floor, not the ceiling — then go beyond it.
- Drive it with the Puppeteer MCP tools (`puppeteer_navigate`, `_click`, `_select`, `_fill`, `_screenshot`, `_evaluate`). Navigate to the catalog, select **this** game, and play **one or more complete games to a terminal state** — as many as you judge useful to exercise the rules.
  - **Read after render, never in the same call as a click.** React re-renders asynchronously, so a `puppeteer_evaluate` that dispatches a click and then reads the board or `render_game_to_text()` in the *same* call returns **stale** state and can fake a defect. Click in one call; read in a separate call (or await a tick / `waitForFunction`).
  - **Reaching terminal on long games.** When a game is too long to hand-play to a terminal (e.g. a multi-thousand-ply race), use the shell's **Bot vs bot** mode + **Start Autoplay**, then wait for terminal status. Do not use a foreground `sleep` to wait (harness-blocked) — poll inside `puppeteer_evaluate` with a `Promise`/`setTimeout` loop on the Rust view status, or run a background task.
- Watch for defects of every kind: wrong legal actions, illegal moves accepted, mis-scored or mis-detected terminals, desynced/incorrect views, broken or misleading UI, non-deterministic or leaking behavior, crashes, console errors. Screenshot anything suspicious into the journal's evidence trail.

### 4. Fix defects via TDD, commit each unit

For each defect:

1. Confirm it against the game docs — is the *implementation* wrong, or your understanding? Re-read the relevant `docs/*` rather than assume.
2. Write a **failing test first** at the right layer (`docs/TESTING-REPLAY-BENCHMARKING.md` — unit, rule, golden trace, property, simulation, replay, serialization, visibility/no-leak, bot-legality, or UI smoke).
3. Fix the **Rust** behavior (or the presentation layer only if the defect is genuinely presentational) so the test passes, staying inside the kernel/boundary law.
4. Run the relevant gates before committing (per `CLAUDE.md` §Commands: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and the per-game `simulate`/`replay-check`/`fixture-check`/`rule-coverage` for this game).
5. **Commit each distinct unit of work to `main`** with a focused message. One logical fix = one commit. Stage only the files for this unit; do not sweep up unrelated working-tree changes.

### 5. Update the journal

Append the iteration entry. Set `Verdict: CLEAN` only if you played to completion and found **no** bug to fix this iteration. Update `Consecutive clean iterations`: increment on a CLEAN verdict, reset to 0 on any FIXED/BLOCKED.

### 6. Stop conditions

Two distinct terminations — apply whichever fires first:

- **Spec needed → STOP THE LOOP.** If a defect is not a quick fix but a deficiency requiring design (a missing mechanic, an ambiguous/undefined rule, a contract or boundary question, anything tripping a FOUNDATIONS §12 stop condition or §13 ADR trigger), do **not** hack it: write a spec under `specs/*` aligned with `docs/**` and the `specs/README.md` conventions, commit it (with any `specs/README.md` tracker row) as its own unit per step 5 above — staging only those spec files, set the journal `Status: STOPPED`, record the spec path, and **stop the loop**. A shared-surface UI policy with several viable designs is a typical spec-worthy case; committing to one bound by editing a magic constant would be the hack this bullet forbids.
- **Two clean iterations → END THE LOOP.** When `Consecutive clean iterations` reaches **2**, the game is shaken out for now: set the journal `Status: ENDED` and **end the loop**.

**How to signal the loop to stop/end:** finish the iteration with an explicit, unambiguous terminal line as the very last thing you output, e.g. `PLAYTEST LOOP ENDED: <slug> — two consecutive clean iterations` or `PLAYTEST LOOP STOPPED: <slug> — spec written to specs/<file>`. Do not schedule another iteration (in self-paced `/loop`, omit the next wakeup). If the loop is a fixed-interval `/loop` that keeps re-firing, the terminal line plus `Status: ENDED/STOPPED` in the journal tells the next fire to no-op. If you know the loop's cron job id (recorded as `Loop cron job` in the journal header, or still in your context from creating it), proactively `CronDelete` it so it stops firing no-ops; otherwise surface that the user can cancel the interval. Never emit a terminal line whose condition isn't genuinely met (HARD-GATE).

## What this skill does NOT do

- It does not implement a game from scratch or add new mechanics — spec-worthy work routes to `specs/*` and stops the loop.
- It does not decide legality, validation, or any behavior in TypeScript.
- It does not weaken, skip, or delete tests to get green.
- It does not commit a spec-and-stop and a fix in the same breath without recording both in the journal.
