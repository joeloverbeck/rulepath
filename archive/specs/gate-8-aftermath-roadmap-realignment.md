# Gate 8 Aftermath / Roadmap Realignment

| Field | Value |
|---|---|
| Spec ID | `gate-8-aftermath-roadmap-realignment` |
| Roadmap stage | 6M (maintenance interlock after Gate 8, before Gate 9) |
| Roadmap build gate | Post-Gate-8 maintenance pass — **non-feature** |
| Status | Planned |
| Date | 2026-06-08 |
| Owner | Rulepath maintainers |
| Authority order | [`docs/FOUNDATIONS.md`](../../docs/FOUNDATIONS.md) → [`docs/ROADMAP.md`](../../docs/ROADMAP.md) → this spec. Where this spec and a foundation document disagree, the foundation document wins. |

> Reader orientation: this spec carries the canonical Rulepath section set
> (Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance
> evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation
> updates required, Sequencing, Assumptions). The original narrative material —
> Decision, Product intent, Implementation guidance, Documentation update rules,
> Risks — is preserved below the canonical sections as detailed reference.

## Objective

Do a small Gate 8 aftermath pass before Gate 9 implementation. This is not a new
gameplay gate. It is a truthfulness and routing pass so the next coding session
does not inherit contradictory instructions.

The live documents disagree about Gate 8 status:

- root `README.md` and `progress.md` still describe Gate 8 as planned and include
  a Blackjack checkpoint before Gate 9;
- `specs/README.md`, `docs/ROADMAP.md`, and `docs/MECHANIC-ATLAS.md` already say
  High Card Duel is the accepted Gate 8 proof, Blackjack Lite is deferred, and
  Gate 9 should move toward `token_bazaar` / `secret_draft`;
- `apps/web/README.md` still names only the older browser games even though the
  WASM catalog and E2E suite include newer games.

This spec exists to remove that ambiguity. Make the repository tell the truth
after Gate 7.2 / Gate 8:

1. Gate 8 High Card Duel is complete in the current worktree.
2. Blackjack Lite is deferred by ADR 0006 and is not a blocker for Gate 9.
3. Gate 9 starts with public resource/accounting pressure through `token_bazaar`,
   while `secret_draft` remains a valuable later simultaneous/waiting/reveal proof.
4. The living-doc workflow remains intact: roadmap is mechanic-ladder law, specs
   index tracks progress, archived specs remain historical, root docs orient
   humans, and CI/docs scripts keep links and boundaries honest.

## Scope

### In scope

Update the smallest set of living documents needed to make status and routing
consistent, plus the CI smoke-hygiene fix proven necessary by validation.

Confirmed-stale files (work remains):

- `README.md` — Status section (Gate 8 "planned" + Blackjack-before-Gate-9
  framing) **and** the per-game command-example list, which currently ends at
  `draughts_lite` and omits `high_card_duel`.
- `progress.md` — the header (Gate 8 "planned" + mandatory `blackjack_lite`
  checkpoint) **and** the absence of any Gate 8 completion entry.
- `apps/web/README.md` — names only `race_to_n`, `three_marks`, and
  `column_four`; omits `directional_flip`, `draughts_lite`, and `high_card_duel`
  in the intro, Shell Surface, and Smoke Layers sections.
- `.github/workflows/gate-1-game-smoke.yml` — `high_card_duel` has **no** native
  smoke steps (simulate / replay-check / fixture-check / rule-coverage), and the
  Browser E2E step name omits `high_card_duel` though the step already runs
  `high-card-duel.smoke.mjs`. See Work breakdown WB5 / Implementation guidance §4.
- `docs/SOURCES.md` — only if the aftermath adds source notes for the Gate 9
  candidate-placement rationale (the OpenSpiel / BGG-mechanic / W3C references).
- `specs/README.md` — add a maintenance row tracking this aftermath pass (the
  index has no row for it). The Gate 8 / Blackjack / Gate 9 status content is
  already correct and MUST NOT be re-litigated.

### Out of scope (validated already-correct — do not touch)

Reassessment confirmed these are already reconciled. Editing them risks
re-introducing churn:

- `specs/README.md` Gate 8 / 6C / Gate 9 rows — already mark Gate 8 =
  `high_card_duel` Done and close the Blackjack checkpoint via ADR 0006.
- `docs/ROADMAP.md` §10 — already names `high_card_duel` as the Gate 8 chance /
  hidden-information proof; §11 already routes Gate 9 to
  `token_bazaar` / `resource_race` and `secret_draft`. Touch ROADMAP **only** if
  the `resource_race` alias clarification (WB4) genuinely needs placement law,
  not progress.
- `docs/MECHANIC-ATLAS.md` — already records Gate 8 card/deck pressure and the
  `blackjack_lite` deferral under ADR 0006 (atlas row for deterministic shuffle /
  private hand / hidden commitment).
- `docs/README.md` — an ordered doc-set index with no gate-status summary to
  update.
- `games/high_card_duel/data/manifest.toml` — `readiness = "planned-official-game"`
  is the **intended shared pre-release label**, not stale: `column_four`,
  `directional_flip`, `draughts_lite`, and `high_card_duel` all use it, while the
  two plumbing games (`race_to_n`, `three_marks`) use `foundation-smoke`. Leave
  the label alone.

### Not allowed

This aftermath pass MUST NOT:

- implement `token_bazaar`;
- implement or resurrect Blackjack Lite;
- create tickets, AGENT-TASK files, or decomposition plans;
- alter gameplay behavior for any existing game except a truly minimal hygiene fix
  proven necessary by validation;
- promote resource, card, deck, hand, market, contract, or bot-policy primitives
  into `engine-core` or `game-stdlib`;
- change trace schemas, replay hashes, or golden traces without an explicit
  migration note and a direct reason;
- rewrite the roadmap as a progress diary;
- rewrite archived gate specs to fit the new status narrative (broken-link fixes
  for `scripts/check-doc-links.mjs` only);
- use GitHub branch names, repository code search, or default-branch metadata as
  evidence.

## Deliverables

| # | Artifact | Change |
|---|---|---|
| D1 | `README.md` | Truthful Status section (Gate 8 done, no Blackjack block); per-game command list includes `high_card_duel`. |
| D2 | `progress.md` | Truthful header; a Gate 8 `high_card_duel` completion entry matching the existing per-gate entry style. |
| D3 | `apps/web/README.md` | Names all browser-exposed games and smoke layers, including `directional_flip`, `draughts_lite`, and `high_card_duel`. |
| D4 | Candidate-placement note | Concise placement table for `token_bazaar` / `resource_race` / `secret_draft` / `blackjack_lite` / `poker_lite` / `plain_tricks`, with `resource_race` marked alias/alternate. Lives in the progress/specs index area. |
| D5 | `.github/workflows/gate-1-game-smoke.yml` | `high_card_duel` native smoke wired in (or explicit defer documented); stale E2E step name corrected. |
| D6 | `specs/README.md` | Maintenance row tracking this aftermath pass. |
| D7 | `docs/SOURCES.md` | Gate 9 candidate-placement source notes, only if added. |

## Work breakdown

Each item is a candidate AGENT-TASK. Dependency order is mostly independent;
WB1–WB3 are the human-orientation core and should land first.

| WB | Task | Depends on | Notes |
|---|---|---|---|
| WB1 | Fix `README.md` Status section + per-game command list (D1) | — | A new contributor must not read it and believe Gate 8 is planned or that Blackjack blocks Gate 9. |
| WB2 | Fix `progress.md` header + add Gate 8 HCD completion entry (D2) | — | Mirror the existing dated entry style (scope, boundary notes); name Gate 9 as next target. |
| WB3 | Fix `apps/web/README.md` browser games + smoke layers (D3) | — | Name `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel` consistently across intro, Shell Surface, Smoke Layers. |
| WB4 | Candidate-placement / `resource_race`-alias note (D4) | — | Place in progress/specs index area; touch ROADMAP only if placement law itself needs it. |
| WB5 | CI native-smoke hygiene for `high_card_duel` (D5) | — | Either add simulate/replay-check/fixture-check/rule-coverage steps and fix the E2E step name, **or** document the explicit defer reason in the progress/index. No silent omission. |
| WB6 | Add this spec's maintenance row to `specs/README.md` (D6) | WB1–WB5 | Record as a 6M maintenance interlock; flip to `Done` when exit criteria pass. |
| WB7 | Add Gate 9 candidate source notes to `docs/SOURCES.md` (D7) | WB4 | Only if the candidate-placement note cites external research. |

## Exit criteria

The aftermath work is done when all of the following are true:

1. `README.md` no longer says Gate 8 is merely planned or that a Blackjack
   checkpoint blocks Gate 9, and its per-game command list names `high_card_duel`.
2. `progress.md` states that Gate 8 High Card Duel is complete in the current
   worktree and names Gate 9 as the next implementation target, via a Gate 8
   completion entry.
3. `specs/README.md` remains the canonical progress/spec index, contains no
   ambiguous Gate 8 / Gate 9 / Blackjack contradiction, and carries a maintenance
   row for this pass.
4. `apps/web/README.md` accurately describes the Rust/WASM browser shell and the
   browser games/smoke layers currently supported.
5. The candidate-placement note exists and explicitly places `token_bazaar`,
   `resource_race`, `secret_draft`, `blackjack_lite`, `poker_lite`, and
   `plain_tricks`.
6. `high_card_duel` native smoke is either wired into `gate-1-game-smoke.yml`
   (simulate / replay-check / fixture-check / rule-coverage) with the E2E step
   name corrected, or the deliberate defer is documented; no silent omission
   remains.
7. No gameplay code changes are made unless validation proves a minimal hygiene
   fix is required; any such fix is named in the final implementation summary.
8. No archived gate spec is rewritten for narrative convenience.
9. No resource/card/market/contract helper is promoted in `engine-core` or
   `game-stdlib`.
10. Boundary checks and doc-link checks pass.
11. The implementation summary clearly states that the work analyzed the current
    worktree and did not independently prove anything about another branch unless
    the user separately supplied that proof.

## Acceptance evidence

Run from the repository root:

```bash
cargo fmt --all --check
cargo test --workspace
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
npm --prefix apps/web ci
npm --prefix apps/web run build
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
```

If WB5 wires `high_card_duel` into native smoke, also run (and add the matching
CI steps):

```bash
cargo run -p simulate      -- --game high_card_duel --games 1000
cargo run -p replay-check  -- --game high_card_duel --all
cargo run -p fixture-check -- --game high_card_duel
cargo run -p rule-coverage -- --game high_card_duel
cargo bench -p high_card_duel -- legal_actions
```

These commands are known to be supported: `high_card_duel` is registered in
`crates/wasm-api`, all five game tools, and the `smoke:e2e` browser suite
(`apps/web/e2e/high-card-duel.smoke.mjs`). The only gap is that
`gate-1-game-smoke.yml` does not yet run the native commands. If any command
cannot run, do not hide it — register it as a smoke-hygiene fix or document the
explicit defer reason and create a follow-up note in the progress/spec index.

Game-level acceptance evidence (rule coverage, golden traces, replay, visibility,
bot legality, benchmarks) is **not applicable** to this pass: `high_card_duel`
already carries it as an accepted Gate 8 game; this pass changes only docs and CI
inventories.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | Docs and CI only; no setup/legality/validation/effects/views/replay/bots move out of Rust, and TypeScript stays presentation-only. |
| §3 `engine-core` is a contract kernel | aligned | No mechanic noun enters the kernel; the Not-allowed list forbids resource/card/deck/hand/market/contract promotion. |
| §4 `game-stdlib` is earned | aligned | No helper promotion; `high_card_duel` remains the first card/deck use, insufficient for promotion (ADR 0006). |
| §5 Static data is not behavior | aligned | The `manifest.toml` readiness label is metadata, left untouched; no behavior-looking fields introduced. |
| §6 Official games are evidence-heavy | aligned | `high_card_duel` already satisfies the contract; WB5 only exposes its existing native smoke in CI. |
| §11 Universal acceptance invariants | aligned | Determinism preserved (Not-allowed forbids trace/hash/golden changes without a migration note); no hidden-info leak introduced by doc edits. |
| §12 Stop conditions | clear | None crossed — the game carries full coverage; the CI gap is hygiene, not a missing-coverage stop condition. |
| §13 ADR triggers | N/A | No architecture-changing decision; the Blackjack reclassification was already made by the accepted [ADR 0006](../../docs/adr/0006-blackjack-lite-roadmap-placement.md). |

## Forbidden changes

See Scope → Not allowed. In particular: no feature work, no Blackjack work, no
ticket/AGENT-TASK creation, no primitive promotion, no trace/hash/golden
migration without an explicit note, no rewriting of archived specs, and no
branch-name / code-search / default-branch evidence.

## Documentation updates required

- `README.md`, `progress.md`, `apps/web/README.md` — per Deliverables D1–D3.
- Candidate-placement note (D4) in the progress/specs index area.
- `specs/README.md` — add a 6M maintenance row for this pass; flip it to `Done`
  when exit criteria pass.
- `docs/SOURCES.md` — Gate 9 candidate-placement source notes, only if added (D7).
- Do not edit `docs/ROADMAP.md` except for genuine placement-law clarification
  (WB4); progress belongs in `progress.md` and `specs/README.md`.

## Sequencing

- **Predecessor**: Gate 8 (`high_card_duel`) — `Done` in `specs/README.md`
  (rows 36–37). The 6C post-Gate-8 Blackjack-placement checkpoint is closed by
  ADR 0006 (`specs/README.md` row 38).
- **Successor**: Gate 9 (`token_bazaar` / `secret_draft`) — `Not started`. This
  maintenance pass is an interlock: it removes contradictory orientation before
  Gate 9 is specced, but it does not gate Gate 9 admission on any new game work.
- **Admission rule**: this is a non-feature maintenance pass; it may proceed
  immediately and does not wait on a new mechanic-ladder gate.

## Assumptions

| # | Assumption | Status |
|---|---|---|
| A-1 | Gate 8 `high_card_duel` is complete and wired (wasm-api, all five tools, `smoke:e2e`). | Confirmed at reassessment 2026-06-08. |
| A-2 | `specs/README.md`, `docs/ROADMAP.md`, and `docs/MECHANIC-ATLAS.md` already reflect Gate 8 = HCD and Blackjack deferred. | Confirmed; out of scope to re-edit. |
| A-3 | `readiness = "planned-official-game"` is the intended shared pre-release label, not a per-game stale status. | Confirmed (4 showcase games share it; plumbing games use `foundation-smoke`). Manifest left untouched. |
| A-4 | `gate-1-game-smoke.yml` lacks `high_card_duel` native smoke; the E2E step name is stale. | Confirmed; resolved by WB5. |
| A-5 | The implementation session is at or past commit `5a489b1`; it must validate the worktree, not a pinned commit. | Confirmed; Implementation guidance §1 governs. |

---

## Reference: original narrative

### Decision

Do a small Gate 8 aftermath pass before Gate 9 implementation. This is not a new
gameplay gate. It is a truthfulness and routing pass so the next coding session
does not inherit contradictory instructions. The detailed disagreement is
captured in the Objective above.

### Required status reconciliation

The implementation session must make these statements consistent across the live
docs:

- Gate 0 through Gate 8 are complete in the current worktree.
- Gate 8's accepted proof is `high_card_duel`, covering deterministic setup
  shuffle, private views, viewer-filtered effects/logs, public replay/export
  redaction, bot view discipline, browser no-leak smoke, and benchmark smoke
  floors.
- Blackjack Lite is deferred by ADR 0006 and is not a Gate 8.1 blocker.
- Gate 9 is the next mechanic-ladder implementation target, and the primary
  target is `token_bazaar` unless a later accepted spec explicitly changes it.
- `secret_draft` is still valuable, but it should not be bundled into the same
  implementation session as `token_bazaar`.
- `resource_race` is an alias/alternate name for the economy proof, not a second
  parallel build target.

### Candidate-placement note (D4 content)

| Candidate | Placement after aftermath |
|---|---|
| `token_bazaar` | Primary Gate 9 implementation target. Public resource economy proof. |
| `resource_race` | Alias or alternate design label only; do not implement separately unless a future spec replaces `token_bazaar`. |
| `secret_draft` | Later simultaneous commitment / waiting / reveal proof, preferably after Token Bazaar proves public resources and browser economy UI. |
| `blackjack_lite` | Deferred comparison case under ADR 0006. Not a Gate 8.1 interlock and not a Gate 9 prerequisite. |
| `poker_lite` / `plain_tricks` | Gate 10+ card depth after hidden info, resources/accounting, and action-tree discipline have landed. |
| private monster-game red-team | Not part of this pass. Leave late, optional, and isolated. |

### Implementation guidance

Be boring and precise.

1. Start by checking the current worktree state. The implementation session is at
   or past commit `5a489b1`; update only claims that are actually stale in the
   worktree, and validate against the worktree rather than any pinned commit.
2. Fix root orientation first: a new contributor should not read `README.md` or
   `progress.md` and believe Gate 8 is still planned or that Blackjack blocks
   Gate 9.
3. Fix browser orientation second: `apps/web/README.md` should name all currently
   supported browser games and smoke layers, including High Card Duel and the
   other already-exposed games the shell supports.
4. Fix validation coverage language third. Validation **confirms** the gap:
   `gate-1-game-smoke.yml` runs native simulate/replay-check/fixture-check/
   rule-coverage for `race_to_n`, `three_marks`, `column_four`, `directional_flip`,
   and `draughts_lite`, but **not** for `high_card_duel`; the Browser E2E step
   already runs `high-card-duel.smoke.mjs` but its step name omits the game.
   Either wire the missing native smoke and fix the step name, or document the
   deliberate defer reason. Do not leave silent omissions.
5. Preserve archived specs as historical records. Add links from living docs to
   archives when helpful, but do not edit accepted gate specs to fit the new
   status narrative.
6. Use the existing doc-link script and boundary script as the minimum validation
   floor.

### Documentation update rules

- Use short status prose, not broad restatements of every prior gate.
- Keep `docs/ROADMAP.md` as ladder law. It may clarify candidate placement, but
  progress belongs in `progress.md` and `specs/README.md`.
- Keep `docs/MECHANIC-ATLAS.md` focused on mechanic pressure and promotion
  decisions.
- Keep `apps/web/README.md` focused on how to build, serve, and smoke the shell.
- Do not create a new ADR for this aftermath unless implementation discovers a
  real architecture decision, not just stale prose.

### Risks and mitigations

| Risk | Mitigation |
|---|---|
| The pass grows into feature work. | Treat this as a maintenance spec. Anything beyond truthful docs and smoke-hygiene registration moves to Gate 9 or a later spec. |
| `docs/ROADMAP.md` becomes a progress log. | Update `progress.md` and `specs/README.md` for status; touch ROADMAP only for placement law. |
| CI/workflow changes accidentally mask High Card Duel gaps. | Add explicit validation commands and fail visibly when a tool does not support an accepted game. |
| Candidate names create parallel work. | State that `resource_race` is an alias/alternate, not a separate Gate 9 game unless a later accepted spec changes the target. |

> Resolved at reassessment: the earlier risk that `planned-official-game` might be
> assumed stale is closed — it is confirmed the intended shared pre-release label
> (see Assumption A-3). The manifest is left untouched.

### External references

Used as vocabulary/rationale for the Gate 9 candidate placement only; not an
implementation model and not copied rules. Route into `docs/SOURCES.md` if the
candidate-placement note cites them (WB7):

- OpenSpiel — vocabulary for sequential / simultaneous / stochastic /
  imperfect-information games.
  <https://openspiel.readthedocs.io/en/latest/intro.html>,
  <https://arxiv.org/abs/1908.09453>
- BoardGameGeek mechanic vocabulary — "Market" and "Contracts" as labels for
  buy/sell rows, prices/quantities, goal-fulfillment rewards. Vocabulary only.
  <https://boardgamegeek.com/boardgamemechanic/2900/market>,
  <https://boardgamegeek.com/boardgamemechanic/2912/contracts>
- W3C/WAI — resource state not encoded by color alone; dense interactive
  grids/rows remain keyboard navigable.
  <https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html>,
  <https://www.w3.org/WAI/ARIA/apg/patterns/grid/>
