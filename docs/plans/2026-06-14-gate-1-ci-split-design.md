# Gate 1 CI split — build-once + per-game matrix

## Brainstorm context

- **Original request:** "One workflow runs ~4.5 min, significantly longer than
  the rest, and these grow with each new game added. Analyze whether there is a
  way to split that workflow which would be more valuable than what we have now."
- **Reference material:** none — the request itself was the spec; analysis is
  grounded in the live CI run data for PR #38.
- **Classification:** dev-process/tooling (CI workflow structure; no product
  behavior changes; preserves gate doctrine).
- **Final confidence:** ~90% at design time; approach **C** and the free/public
  billing model were confirmed by the user.
- **Key decisions that shaped the design:** wall-clock (not CPU-minutes) is the
  sole objective because Actions minutes are free/public; the real lever is
  execution parallelism, not compile caching; growth is driven by a few heavy
  games, so per-game fan-out is the durable fix.

## Problem

`gate-1-game-smoke.yml` is a single job with ~75 serial steps. On PR #38 it took
**295s** (~5 min) — the other two gates run as separate parallel workflows at
~1 min each, so Gate 1 alone sets the PR's wall-clock. Cost concentration
(measured from the PR #38 job's step timings):

| Bucket | Time | Share | Notes |
|---|---|---|---|
| Browser E2E (1 step, all games serial) | 108s | 37% | `shell` + `a11y-noleak` + 15 per-game `.smoke.mjs` |
| Simulations (14 games serial) | ~143s | 48% | Skewed: `directional_flip` 42s + `event_frontier` 41s + `flood_watch` 19s + `draughts_lite` 14s + `race_to_n` 14s (incl. first compile); the other 9 games total ~5s |
| Web build | 23s | 8% | |
| Setup (checkout/node/rust target/npm ci) | ~15s | 5% | |
| Replay + fixture + coverage + docs + boundary (~56 steps) | ~6s | 2% | Essentially free, despite being most of the step *count* |

Three load-bearing facts:

1. ~93% of the cost is sims + E2E + web build. The 56 replay/fixture/coverage/
   docs steps that make the file *look* huge cost ~6s combined — splitting those
   out buys nothing.
2. **No cargo caching exists in any gate.** Every job recompiles the workspace
   from scratch. Today that is cheap (~14s, hidden inside the first sim) because
   there is one job; it becomes the dominant tax the moment work fans out into
   many jobs.
3. **Growth is skewed, not uniform.** A new game adds a sim + an e2e file. Most
   games are ~0–2s; the pain is a *few* heavy games that currently serialize
   behind each other (`directional_flip` 42s *then* `event_frontier` 41s = 83s
   back-to-back).

The lever is execution parallelism. Compile caching alone cannot touch the
42s/41s/108s *runtime* costs, and splitting out the already-free checks does
nothing.

## Approaches considered

- **A — Check-type split** (~4 parallel jobs: sims / replay+fixture+coverage /
  web+e2e / docs+boundary). Roughly halves wall-clock now with a small change,
  but the sims and e2e jobs still grow O(N) internally and each job recompiles —
  the problem returns as games are added.
- **B — Matrix over games (naive)**. Per-game lanes give flat wall-clock, but
  without shared artifacts every lane recompiles the workspace and rebuilds web,
  bloating each lane's critical path.
- **C — Build-once + game matrix (chosen)**. Compile/build once, fan out
  per-game lanes that reuse the artifacts, and run game-independent checks once.
  The only option that makes Gate 1's wall-clock stop growing with the game
  count — the actual problem named in the request.

## Design

### Job graph

```
build  ──► game (matrix: one lane per game)
   └─────► repo-checks (game-independent, runs once)
```

1. **`build`** — `Swatinem/rust-cache` + `cargo build` the four tool binaries
   (`simulate`, `replay-check`, `fixture-check`, `rule-coverage`) + `npm ci` +
   `npm run build` (web dist + wasm). Uploads two artifacts: the tool binaries
   and `apps/web/dist`. Because minutes are free/public, this job exists to keep
   ~15–20s of recompile off every lane's critical path, not to save billing.
2. **`game` (matrix)** — one lane per game. Downloads the artifacts, restores
   executable permissions, then runs that game's `simulate` / `replay-check` /
   `fixture-check` / `rule-coverage` and its per-game `*.smoke.mjs` e2e.
   Wall-clock = slowest single game (~42s `directional_flip`), not the sum.
3. **`repo-checks`** — the game-independent steps that do not grow per game:
   `scripts/boundary-check.sh`, `check-doc-links.mjs`, `check-catalog-docs.mjs`,
   `check-presentation-copy.mjs`, `copy-player-rules.mjs` + `check-player-rules.mjs`,
   `check-outcome-explanations.mjs`, plus the cross-cutting e2e
   (`shell.smoke.mjs`, `a11y-noleak.smoke.mjs`, `rules-display.smoke.mjs`,
   `outcome-explanation.smoke.mjs`). Runs once.

### Key decisions

- **Matrix source = committed `ci/games.json` manifest** (`{id, sim_flags,
  e2e_file?}` per game), not a bare directory listing — because of the per-game
  action-cap flags (`masked_claims --action-cap 24`, `poker_lite --action-cap
  16`, `plain_tricks --action-cap 32`), the hyphen/underscore e2e filename
  mismatch, and `race_to_n` having no per-game e2e. A drift check (manifest ids
  == `ls games/`) fails CI if a game is added without a manifest row. This
  replaces today's hand-maintained, 14×-repeated step lists with one source of
  truth.
- **Cross-cutting e2e stays out of the matrix.** `shell`, `a11y-noleak`,
  `rules-display`, `outcome-explanation` are not per-game; they run once in
  `repo-checks` (or a dedicated `base-e2e` lane). Only the per-game
  `*.smoke.mjs` files fan out.
- **Gate boundaries unchanged.** Same checks, same pass/fail semantics; Gate 0
  and Gate 2 untouched (Gate 2's lanes are ADR-0002-governed and out of scope).

### Edge cases

- Browser/Playwright deps must be installed in each `game` lane that runs e2e;
  cache the browser download to keep it off the critical path.
- A game with no per-game e2e omits that step via the manifest (`e2e_file`
  absent).
- Artifact permissions on the tool binaries must be restored (`chmod +x`) after
  download.
- `directional_flip`/`event_frontier` are the wall-clock floor; if either grows,
  it caps the gate — worth a manifest annotation so the slow lane is visible.

### Estimated wall-clock

`build` ~40s → slowest `game` lane ~ download 5s + `directional_flip` sim 42s +
e2e ~15s ≈ 60s → total ~100s, flat as games are added (down from 295s).

## Verification

Open a no-op PR and confirm:

1. **Coverage parity** — every check that ran before still runs (cross-check the
   step inventory against the current `gate-1-game-smoke.yml`).
2. **Green/red parity** — a deliberately broken game fails its lane; a broken
   repo-wide check fails `repo-checks`.
3. **Wall-clock** — Gate 1 completes in < ~120s.
4. **Scaling** — adding a 15th game (manifest row + lane) does not raise
   wall-clock and the drift check passes.

## FOUNDATIONS alignment

Dev-process/tooling. Preserves gate doctrine (no determinism, no-leak, or
`engine-core`-boundary surface touched). No ADR required — Gate 1 is not
ADR-governed (unlike Gate 2, whose benchmark lanes are fixed by
`docs/adr/0002-ci-benchmark-gating-lanes.md`).
