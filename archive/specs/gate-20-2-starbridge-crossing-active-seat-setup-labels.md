# Gate 20.2 — Starbridge Crossing discontinuous active-seat setup labels

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `gate-20-2-starbridge-crossing-active-seat-setup-labels` |
| Stage / unit | Public scaling phase — Gate 20 correctness follow-on (post-`Done`) |
| Gate | Gate 20.2 (presentation/contract fix on shipped Gate 20 `starbridge_crossing`) |
| Status | `Done` |
| Date | 2026-06-27 |
| Owner | TBD |
| Authority order | `docs/FOUNDATIONS.md` → `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `games/starbridge_crossing/docs/RULES.md` → this spec |

This follows the established post-gate correctness-spec pattern used for
Gate 20.1: a shipped official game has a defect that is not a bounded
ticket-only change because the boundary-correct fix requires a small Rust-owned
catalog contract addition consumed by the presentation shell.

## 2. Objective

Make the browser **match-setup screen** name the correct active seats for every
supported Starbridge Crossing seat count. Today the setup screen mislabels the
active seats for 2-, 3-, and 4-seat matches because the shared web shell derives
the active-seat set itself — by slicing the first *N* labels of the catalog's
six-point ring — instead of consuming a Rust-owned active-seat-by-count mapping.
Starbridge Crossing is the first game whose active-seat selection is
**discontinuous** (`SC-SETUP-003`), so the shell's "first *N*" heuristic is wrong
for it.

## 3. Reproduction (observed)

Built web shell, opened **Starbridge Crossing → Players & roles / Mode** on the
setup screen (no match started yet):

| Seat count | Setup screen shows (wrong) | Rust actually assigns (`active_points_for_seat_count`, `RULES.md` `SC-SETUP-003`) |
|---:|---|---|
| 2 | North, **North East** | North, **South** |
| 3 | North, **North East, South East** | North, **South East, South West** |
| 4 | North, North East, **South East, South** | North, North East, **South, South West** |
| 6 | North, North East, South East, South, South West, North West | identical (only correct case) |

The mislabel appears in `modeDetail` (e.g. _"North is you; North East is an
automated opponent"_) and in the `Players & roles` list (`setupSeatRoles`). Once
a match starts, the in-match seat legend is correct (e.g. _"Seat 1 north to
south"_) because it is rendered from the Rust public view — so the defect is
scoped to the **pre-match setup preview**, which has no view and falls back to a
TypeScript-derived guess.

Root cause: `apps/web/src/components/MatchSetup.tsx::setupLabelsForCount` returns
`labels.slice(0, selectedSeatCount)` over the flat six-seat catalog ring
(`crates/wasm-api/src/catalog.rs::catalog_starbridge_seat_labels_json`). No
catalog field communicates which ring positions are active for a given seat
count, so the shell invents the mapping — a presentation surface deciding a
Rust-owned setup fact, contrary to `SC-UI-001` and the
`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` "Rust owns setup" posture.

## 4. Scope

**In scope**

- Rust-owned catalog metadata: extend the Starbridge catalog entry so that, for
  each supported seat count, Rust declares the active **ring indices** (the
  clockwise positions into the existing six-point `seat_labels` ring) drawn from
  its own `active_points_for_seat_count` mapping via
  `StarPoint::clockwise_index()` (`games/starbridge_crossing/src/ids.rs:119`).
  The data is identity/metadata only (no formulas, selectors, or behavior
  language) and is sourced from the existing Rust setup authority, not
  re-authored. The concrete edit site is the early-return inline catalog JSON
  for `RegisteredGame::StarbridgeCrossing`
  (`crates/wasm-api/src/catalog.rs:302`–`311`) and its
  `catalog_starbridge_seat_labels_json` neighbor (`catalog.rs:353`–`355`) —
  **not** the shared `with_catalog_seat_metadata`/`catalog_seat_metadata_fields`
  path, which Starbridge bypasses via the early return at `catalog.rs:313`–`318`.
  Indices (not formatted label strings) are the canonical form because
  `StarPoint::label()` emits lowercase-underscore (`"north_east"`,
  `ids.rs:99`) while the catalog ring labels are title-case-with-space
  (`"North East"`); deriving by index keeps the Rust↔catalog bridge format-free.
  Where the metadata is surfaced as resolved labels, reuse the existing
  `active_seat_labels` / `SeatDisplayLabel[]` shape already projected by the
  in-match public view (e.g. `crates/wasm-api/src/games/river.rs:94`,
  `apps/web/src/wasm/client.ts:925`) rather than inventing a divergent shape.
- Web shell consumption: `setupLabelsForCount` (and any sibling setup-label
  resolution) must consume the Rust-provided active-seat mapping when present and
  use it for `modeDetail`, `setupSeatRoles`, and any other setup-preview surface
  that names seats. This requires declaring the new field on the `GameCatalogEntry`
  bridge type (`apps/web/src/wasm/client.ts:95`, alongside `seat_labels?` /
  `supported_seats?` at 103–104) so it is typed and available to the consumer.
  The "first *N*" slice remains **only** as a fallback for
  games that do not provide the mapping (all current games except Starbridge,
  whose active seats are contiguous so the fallback is already correct).
- Tests: a failing-first assertion that Starbridge 2/3/4-seat setup previews name
  the Rust-correct seats (North+South for 2, etc.), plus a regression that
  contiguous-seat games are unchanged. A wasm-api/catalog test that the new
  metadata's active indices equal the `clockwise_index()` of
  `active_points_for_seat_count` for `{2,3,4,6}` (compare on indices, not on
  formatted label strings).
- A web e2e/smoke assertion (extend `apps/web/e2e/starbridge-crossing.smoke.mjs`
  or a setup-focused smoke) that the Players & roles list for a 2-seat Starbridge
  setup contains `South` and not `North East`.

**Out of scope**

- Any movement, finish, terminal, visibility, bot, or in-match view change.
- New variants, seat counts, or piece counts.
- Re-labelling seats or changing the six-point ring names.
- Replay/hash/fixture/trace determinism artifacts — this is additive catalog
  metadata and presentation; it does not change any accepted command stream,
  state, effect, or hash. The catalog snapshot test
  `crates/wasm-api/tests/snapshots/api_surface.tsv` carries the full catalog JSON
  (in its `_global/list_games` row); the Starbridge entry's added field is the
  single expected additive diff.

**Not allowed**

- TypeScript computing the active-seat-to-home mapping for any game
  (`SC-UI-001`; `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`).
- Behavior language (formulas/selectors/conditions) in catalog static data
  (`docs/ENGINE-GAME-DATA-BOUNDARY.md`).
- Any `engine-core`/`game-stdlib` topology/seat-ring noun to express this; the
  mapping stays game-local in `games/starbridge_crossing` and is surfaced through
  the existing catalog plumbing.

## 5. Work breakdown

1. **GAT202STACROACT-001** — Rust catalog metadata: add the Rust-owned
   active-seat-by-count metadata (active ring indices) to the Starbridge catalog
   entry, sourced from `active_points_for_seat_count` via
   `StarPoint::clockwise_index()`; refresh the additive catalog snapshot and add
   the wasm-api metadata regression test.
2. **GAT202STACROACT-002** — Web consumption: declare the new field on
   `GameCatalogEntry`, make `setupLabelsForCount` consume the Rust mapping
   before falling back to contiguous first-N labels for games without it, and
   extend the Starbridge browser smoke to cover 2/3/4-seat setup previews.
3. **GAT202STACROACT-003** — Evidence and closeout: update
   `games/starbridge_crossing/docs/UI.md`,
   `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `specs/README.md`, and this
   spec's status after the code/web tickets pass.

Dependency order: 001 → 002 → 003.

## 6. Exit criteria

- Starbridge setup previews name North+South (2), North+South East+South West (3),
  and North+North East+South+South West (4), matching
  `active_points_for_seat_count` and `SC-SETUP-003`.
- No TypeScript code derives the active-seat set for any game by position; the
  shell consumes Rust-provided data, with the slice retained only as a documented
  fallback for games that genuinely have contiguous active seats.
- Contiguous-seat games' setup previews are byte-for-byte unchanged.
- CI gate 0 (`fmt`, `clippy -D warnings`, `build`, `test`) and gate 1
  (`simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`,
  `boundary-check.sh`, `check-doc-links.mjs`, web smokes/build) pass with only the
  single additive catalog-snapshot diff.

## 7. Acceptance evidence

- Failing-first transcript was superseded by the live GAT202STACROACT split;
  regression coverage landed in the wasm-api catalog test and Starbridge browser
  smoke, then passed.
- `cargo test -p wasm-api` (catalog metadata matches `active_points_for_seat_count`).
- `cargo test --workspace`.
- `npm --prefix apps/web run build` and `node apps/web/e2e/starbridge-crossing.smoke.mjs`.
- Manual Puppeteer recheck: 2-seat Starbridge Players & roles shows `South`,
  not `North East`.

## 8. FOUNDATIONS & boundary alignment

- **Behavior authority / `SC-UI-001`** — which seats are active for a seat count
  is Rust setup behavior; the shell must present it, not recompute it.
- **§12 stop condition closed** — the shipped slice heuristic has the shell
  decide a Rust-owned setup fact, which is a live `docs/FOUNDATIONS.md §12`
  "TypeScript decides … behavior" crossing. This fix removes that crossing rather
  than introducing one; it is a contract correction, not a cosmetic relabel.
- **`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`** — "Rust owns setup"; the browser
  "may present setup controls" but must source assignments from Rust. Discontinuous
  supported sets are explicitly contemplated (Supported sets row).
- **engine-game-data boundary** — the mapping is typed identity/metadata sourced
  from Rust, never a static-data selector; stays in `games/starbridge_crossing`.
- **Determinism** — additive metadata only; no accepted-command, state, effect,
  view, or hash change. No ADR trigger (no kernel/DSL/YAML/trace-hash/visibility/
  architecture change); flag for reviewer if the catalog-schema addition is judged
  to need one.

## 9. Forbidden changes

- No new pass option, variant, seat count, or piece count.
- No TypeScript legality or TypeScript active-seat derivation.
- No ring-label renaming; no in-match view change.
- No `engine-core`/`game-stdlib` seat-ring/topology noun.

## 10. Documentation updates required

- `games/starbridge_crossing/docs/UI.md` — note the setup-preview seat source is
  the Rust active-seat-by-count catalog metadata.
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` — fix receipt.
- `specs/README.md` — the Gate 20.2 tracker row exists and is flipped to `Done`
  at closeout.
- Web-shell catalog docs: confirm no renderer-list/smoke-list membership change
  (game already listed); only setup-label sourcing changes.

## 11. Sequencing

- Predecessor: Gate 20 (`Done`), Gate 20.1 (`Done`).
- Successor: does not block Gate 21; an independent correctness/contract follow-on
  on the shipped Gate 20 game, executable any time before public release (Gate 20
  closeout already notes IP/public-release review pending).

## 12. Assumptions (one-line-correctable)

- A1: `active_points_for_seat_count` (`games/starbridge_crossing/src/ids.rs:151`)
  is the single ground-truth active-seat mapping; the catalog metadata (active
  ring indices) must be derived from it via `StarPoint::clockwise_index()`
  (`ids.rs:119`), not re-authored.
- A2: The defect is presentation-only (pre-match setup preview); no accepted
  command stream, state, effect, view, or hash changes, so no ADR-0009
  determinism migration is required beyond the additive catalog-snapshot diff.
- A3: All other current games have contiguous active seats, so the retained
  "first *N*" fallback keeps them correct and unchanged.

## Outcome

Completed: 2026-06-28

Completed tickets:

- `archive/tickets/GAT202STACROACT-001.md` — Rust catalog metadata
  (`f98a32e`).
- `archive/tickets/GAT202STACROACT-002.md` — web setup consumption and browser
  smoke (`59f6bdd`).
- `archive/tickets/GAT202STACROACT-003.md` — Starbridge docs/evidence and
  tracker closeout (`e38d868`).

Implementation summary:

- Added additive Rust/WASM catalog metadata `active_seats_by_count` for
  Starbridge Crossing, derived from `active_points_for_seat_count` via
  `StarPoint::clockwise_index()`.
- Updated the web setup preview to consume that Rust-provided mapping before
  falling back to contiguous first-N labels for games without the field.
- Extended the Starbridge browser smoke to assert the 2-, 3-, and 4-seat setup
  preview labels before match start.
- Reconciled `games/starbridge_crossing/docs/UI.md`,
  `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, and `specs/README.md`.

Deviations:

- The drafted four-item `GAT202STASEAT-*` task split was superseded by the live
  three-ticket `GAT202STACROACT-*` series. The spec was reconciled to the actual
  delivered split before archive.
- The failing-first transcript called for in the draft acceptance evidence was
  superseded by the live ticket split; regression proof is the wasm-api catalog
  test plus the Starbridge browser smoke setup-preview assertions.

Verification evidence:

- Rust/catalog: `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface`,
  `cargo fmt --all --check`, `cargo test -p wasm-api`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `bash scripts/boundary-check.sh`, and `cargo test --workspace` passed.
- Web: `npm --prefix apps/web run build`,
  `node apps/web/e2e/starbridge-crossing.smoke.mjs`,
  `npm --prefix apps/web run smoke:e2e`, and
  `npm --prefix apps/web run smoke:ui` passed. The final `smoke:e2e` rerun
  included a fresh build after the docs/spec closeout edits.
- Docs/archive truthing before spec archive: `node scripts/check-doc-links.mjs`,
  status/stale-stem greps, and `git diff --check` passed.

Manual/non-command evidence:

- The Starbridge Puppeteer smoke now performs the requested setup-screen check:
  2-seat Players & roles shows North+South and excludes North East; 3- and
  4-seat setup previews match `SC-SETUP-003`.

Archive truthing:

- Active ticket glob `tickets/GAT202STACROACT*` is empty.
- Archived tickets `GAT202STACROACT-001..003` have `**Status**: COMPLETED` and
  `## Outcome`.
- `specs/README.md` is retargeted to the archived spec path during final
  reference closeout.

Unrelated worktree changes:

- None observed during closeout.
</content>
