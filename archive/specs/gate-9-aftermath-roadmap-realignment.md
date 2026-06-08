# Gate 9 Aftermath / Roadmap Realignment

| Field | Value |
|---|---|
| Spec ID | `gate-9-aftermath-roadmap-realignment` |
| Roadmap stage | 7M (maintenance interlock after Gate 9, before Gate 9.1) |
| Roadmap build gate | Post-Gate-9 maintenance pass — **non-feature** |
| Status | Planned |
| Date | 2026-06-08 |
| Owner | Rulepath maintainers |
| Primary target | `apps/web/README.md` |
| Browser implementation | Not applicable; documentation truthfulness pass only |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ROADMAP.md` → `docs/AGENT-DISCIPLINE.md` → `archive/specs/gate-8-aftermath-roadmap-realignment.md` precedent → this spec |

Where this spec and a foundation document disagree, the foundation document wins. This spec does not verify that commit `65ec79d403e8481b439b1908332c263c73e1d002` is the current `main`; it is authored against that user-supplied target commit as the file baseline.

> Reader orientation: this spec carries the canonical Rulepath section set in a deliberately small maintenance form. It is a truthfulness cleanup spec, not a gameplay gate, implementation ticket, or rewrite pass.

## Objective

Make the post-Gate-9 web-shell README tell the truth about `token_bazaar` / Token Bazaar.

At the target commit, the repository has already completed Gate 9 and registered Token Bazaar across native tooling, WASM, browser E2E, benchmark workflows, root status, progress, mechanic atlas, and spec index. The stale surface is `apps/web/README.md`, which still describes the browser shell as if it ends at `high_card_duel` in three places:

1. the intro browser-games list;
2. the **Shell Surface** board-renderer list;
3. the **Smoke Layers** `smoke:e2e` description.

This pass updates that documentation only, preserving the Gate 8 aftermath precedent: validate what is already correct, touch the smallest stale living surface, and do not convert a status cleanup into feature work.

## Scope

### In scope

- Update `apps/web/README.md` intro list to include `token_bazaar` / Token Bazaar as a local browser game.
- Update `apps/web/README.md` **Shell Surface** list to include the Token Bazaar board renderer.
- Update `apps/web/README.md` **Smoke Layers** `smoke:e2e` description to include Token Bazaar / `token-bazaar.smoke.mjs`.
- Keep the current `directional_flip` note unless the implementation also changes `apps/web/package.json`. At the target commit, `apps/web/package.json` chains `token-bazaar.smoke.mjs` in `smoke:e2e` but does **not** chain `directional-flip.smoke.mjs`; the README note is therefore truthful for the package script, even though CI also runs Directional Flip E2E manually.
- Add a maintenance row to `specs/README.md` for this aftermath pass if the maintainers want the spec index to track the cleanup before it is archived.
- Run doc-link validation after the README edit.

### Out of scope (validated already-correct — do not touch)

These surfaces were checked against the exact target commit and should not be edited by this pass:

| Surface | Validation result | Stance |
|---|---|---|
| Root `README.md` | Already says Gates 0–9 are complete and names Token Bazaar as an official game. | Do not touch. |
| `progress.md` | Already records Gate 9 Token Bazaar completion. | Do not touch. |
| `docs/ROADMAP.md` | ROADMAP remains ladder law; it already lists Gate 9 candidates `token_bazaar` / `resource_race` and `secret_draft`. It is not a progress diary. | Do not touch. |
| `docs/MECHANIC-ATLAS.md` | Already records `token_bazaar` public resource/accounting as local-only first-use pressure and no open promotion debt. | Do not touch. |
| `specs/README.md` Gate 9 row | Already marks Gate 9 Done and notes `secret_draft` deferred to Gate 9.1. Only add a new maintenance row if tracking this cleanup; do not rewrite existing rows. |
| `.github/workflows/gate-1-game-smoke.yml` | Already runs Token Bazaar simulation, replay-check, fixture-check, rule-coverage, and browser E2E. | Do not touch. |
| `.github/workflows/gate-2-benchmarks.yml` | Already includes Token Bazaar benchmark smoke and threshold gate. | Do not touch. |
| `crates/wasm-api/src/lib.rs` | Already registers Token Bazaar in browser-facing surfaces. | Do not touch. |
| `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` | Already have post-Gate-9 game registration evidence through CI and fetched dispatch surfaces. | Do not touch. |
| `apps/web/package.json` | `smoke:e2e` already chains `token-bazaar.smoke.mjs`. It does not chain `directional-flip.smoke.mjs`; this makes the existing README Directional Flip caveat truthful for the package script. | Do not touch. |
| `apps/web/e2e/token-bazaar.smoke.mjs` | Exists. | Do not touch. |
| Archived specs | Historical records; do not rewrite to match current status. | Do not touch. |

### Exact-commit divergence note

The research brief says `docs/SOURCES.md` already names `token_bazaar`. In the exact fetched target commit, the repo-level `docs/SOURCES.md` source-note table does not contain `token_bazaar`; it also is not an exhaustive list of all official games. This pass does **not** classify that as a stale web-shell truthfulness issue, because the required stale surface is `apps/web/README.md` and Token Bazaar has its own per-game source note under `games/token_bazaar/docs/SOURCES.md`. If maintainers want the repo-level sources table to enumerate every official game, that should be a separate source-index cleanup, not this tiny web README pass.

### Not allowed

This aftermath pass MUST NOT:

- implement `secret_draft`, modify `token_bazaar`, or change any gameplay code;
- edit Rust crates, WASM logic, tools, CI, traces, fixtures, benchmarks, or golden traces;
- create tickets, AGENT-TASK files, or decomposition packets;
- promote any primitive into `engine-core` or `game-stdlib`;
- add behavior-in-data, YAML, DSLs, selectors, formulas, or static rule logic;
- rewrite `docs/ROADMAP.md` as a progress diary;
- rewrite archived specs;
- weaken, delete, or bypass tests;
- invent CI changes for Directional Flip or Token Bazaar;
- touch any “validated already-correct” file except the optional `specs/README.md` maintenance row.

## Deliverables

| # | Artifact | Required change |
|---:|---|---|
| D1 | `apps/web/README.md` | Add Token Bazaar to the intro local-games list. |
| D2 | `apps/web/README.md` | Add Token Bazaar to the **Shell Surface** renderer list. |
| D3 | `apps/web/README.md` | Add Token Bazaar / `token-bazaar.smoke.mjs` to the **Smoke Layers** `smoke:e2e` description. |
| D4 | `specs/README.md` | Optional maintenance row: Stage `7M`, Gate `Gate 9 aftermath / web README realignment`, Spec `gate-9-aftermath-roadmap-realignment.md`, Status `Planned` then `Done` after the README/doc-link evidence passes. Do not rewrite the existing Gate 9 row. |
| D5 | Handoff note | Record exact files changed, validation command(s), and the fact that no gameplay/CI/tooling files were touched. |

No other files are deliverables for this spec.

## Work breakdown

| # | Candidate task | Depends on | Notes |
|---:|---|---|---|
| 1 | Re-read `apps/web/README.md` and `apps/web/package.json` | — | Confirm the three Token Bazaar omissions and the Directional Flip script caveat before editing. |
| 2 | Patch `apps/web/README.md` intro and Shell Surface copy | 1 | Add Token Bazaar without reorganizing the README or changing command docs. |
| 3 | Patch `apps/web/README.md` Smoke Layers copy | 1 | State that `smoke:e2e` includes Token Bazaar. Preserve the Directional Flip caveat unless package script changes. |
| 4 | Add optional `specs/README.md` maintenance row | 2,3 | Only if maintainers want to track this cleanup while active. Do not touch ROADMAP. |
| 5 | Validate and hand off | 2,3,4 | Run `node scripts/check-doc-links.mjs`; optionally grep for `Token Bazaar` / `token_bazaar` in `apps/web/README.md`. Report no gameplay changes. |

## Exit criteria

| Criterion | Evidence required |
|---|---|
| Intro browser-games list is truthful | `apps/web/README.md` names `token_bazaar` / Token Bazaar alongside existing local browser games. |
| Shell Surface renderer list is truthful | `apps/web/README.md` names the Token Bazaar board renderer / Token Bazaar board surface. |
| Smoke Layers description is truthful | `apps/web/README.md` says `smoke:e2e` includes Token Bazaar / `token-bazaar.smoke.mjs`, matching `apps/web/package.json`. |
| Directional Flip caveat is not accidentally falsified | If package script remains unchanged, the README may continue to say the standalone Directional Flip E2E file is not chained by `smoke:e2e`; CI’s manual E2E step is not a reason to edit package-script docs. |
| No scope creep | No Rust, WASM, tool, CI, game, trace, fixture, benchmark, or archived-spec files changed. |
| Spec index remains truthful | Optional maintenance row added without rewriting Gate 9 status. |
| Links pass | `node scripts/check-doc-links.mjs` passes. |

## Acceptance evidence

Minimum acceptance transcript:

```bash
node scripts/check-doc-links.mjs
```

Recommended review checks:

```bash
grep -n "Token Bazaar\|token_bazaar\|token-bazaar" apps/web/README.md
git diff -- apps/web/README.md specs/README.md
```

The diff must be documentation-only. If any test fails, follow the failing-test protocol: first determine whether the failing check is valid, then whether the issue is in the edited docs or the check, then fix without weakening the check.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Not engaged beyond preservation | No behavior changes; Rust remains authority. |
| §3 `engine-core` kernel | Preserved | No engine files touched; no mechanic nouns added. |
| §4 `game-stdlib` earned | Preserved | No helpers or primitive promotions. |
| §5 Static data | Not engaged | No data files touched. |
| §7 Public UI | Aligned | User-facing web documentation should describe the shipped browser surface truthfully. |
| §11 Universal invariants | Preserved | No replay/hash/visibility/bot/benchmark contracts change. |
| §12 Stop conditions | Clear | Stop if implementation touches code, CI, tools, ROADMAP progress, archived specs, or primitive boundaries. |
| §13 ADR triggers | None | Documentation truthfulness edit only; no architecture, replay/hash, data-format, visibility, or bot-class change. |

## Forbidden changes

Do not:

- change gameplay code, Rust crates, WASM exports, tools, CI, traces, fixtures, benchmarks, golden traces, or browser components;
- rewrite `docs/ROADMAP.md` as progress tracking;
- edit archived specs;
- create or modify ticket files or AGENT-TASK packets;
- promote or introduce primitives;
- change `apps/web/package.json` to address Directional Flip unless a separate accepted spec authorizes that work;
- delete or weaken tests/checks to make doc validation pass;
- expand this pass into Gate 9.1 implementation.

## Documentation updates required

- `apps/web/README.md`: required three Token Bazaar truthfulness edits.
- `specs/README.md`: optional maintenance row while this spec is active; flip to `Done` only after exit criteria pass.
- No `docs/ROADMAP.md` progress edit.
- No root `README.md`, `progress.md`, `docs/MECHANIC-ATLAS.md`, CI, tools, or WASM edits.

## Sequencing

- **Predecessor:** Gate 9 Token Bazaar is Done.
- **This pass:** Small post-Gate-9 web README truthfulness cleanup.
- **Successor:** Gate 9.1 `secret_draft` spec/implementation work may proceed after this cleanup or in parallel if maintainers accept the stale README as non-blocking. It must still check the mechanic atlas for open promotion debt before coding.
- **Admission rule:** No open promotion debt is created or skipped; no primitive promotion is part of this pass.

## Assumptions

- A1: `apps/web/README.md` is intended to describe the package script `smoke:e2e`, not every manual CI E2E invocation.
- A2: Token Bazaar is browser-exposed and has a board renderer at the target commit; the README omissions are stale copy, not missing implementation.
- A3: The optional `specs/README.md` row is acceptable even though this is a maintenance pass rather than a feature gate.
- A4: `docs/SOURCES.md` is not treated as a complete game-status index for this pass; any repo-level source-index expansion belongs in a separate cleanup.
- A5: If a maintainer wants broader post-Gate-9 documentation reconciliation, write a new bounded cleanup spec rather than expanding this one.

---

# Implementation reference

## Proposed `apps/web/README.md` edits

### Intro list

Replace the intro sentence’s game list so it includes Token Bazaar. The intro
is a **two-sentence paragraph** — extend the games list only and preserve the
trailing `Rust/WASM owns game behavior; …` sentence verbatim (do not drop it
when replacing the paragraph):

```markdown
`apps/web` is the static React shell for Rulepath's local browser games: `race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column Four, `directional_flip` / Directional Flip, `draughts_lite` / Draughts Lite, `high_card_duel` / High Card Duel, and `token_bazaar` / Token Bazaar. Rust/WASM owns game behavior; TypeScript presents Rust-provided catalog entries, views, action trees, effects, diagnostics, bot turns, and replay projections.
```

### Shell Surface list

Update the board-renderer bullet to include Token Bazaar:

```markdown
- first-class board renderers for Three Marks, Column Four, Directional Flip, Draughts Lite, High Card Duel, and Token Bazaar;
```

### Smoke Layers list

Update the `smoke:e2e` bullet to include Token Bazaar:

```markdown
- `smoke:e2e`: Puppeteer rendered-browser smoke plus accessibility/no-leak smoke for the shell, Three Marks, Column Four, Draughts Lite, High Card Duel, and Token Bazaar. A standalone Directional Flip E2E smoke file also exists under `e2e/`, but is not chained by `smoke:e2e`.
```

If `apps/web/package.json` is later changed to include `directional-flip.smoke.mjs` in `smoke:e2e`, update the Directional Flip caveat in the same later task. Do not make that CI/package-script change here.

## Final handoff checklist

- `apps/web/README.md` has exactly the Token Bazaar truthfulness edits.
- Optional `specs/README.md` maintenance row added or intentionally skipped in handoff.
- `node scripts/check-doc-links.mjs` result recorded.
- No gameplay code or CI/tooling touched.
- No ROADMAP progress edit.
