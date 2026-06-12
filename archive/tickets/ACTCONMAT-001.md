# ACTCONMAT-001: Event Frontier Rust label resolution

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier` (`visibility.rs` SiteView label projection + effect-copy/ordinal resolution, `actions.rs` resolved action/accessibility labels, `ui.rs`/`cards.rs` site-label loader, new `data/sites_presentation.toml` static-data manifest); fixtures/golden traces/serialization tests
**Deps**: None

## Problem

Event Frontier emits player-facing action labels, accessibility labels, and effect strings built from raw internal IDs: target buttons read "Survey site_charterhouse,site_crossing" and aria-labels read "Apply Survey to site_charterhouse,site_crossing" (`actions.rs` `selection_label`/`encode_selections`). `SiteView` projects a raw `SiteId` with no display label, so site display names live only in per-board TypeScript — there is no Rust-side site-label channel for the shared surfaces to consume. Effect copy also drifts from authored card vocabulary (ordinal "First Reckoning" vs. numeral "reckoning 1 resolved"). This is the foundation every downstream presentation ticket builds on (`docs/UI-INTERACTION.md` §19 bans raw internal identifiers in player-facing surfaces).

## Assumption Reassessment (2026-06-12)

1. `SiteView` (`games/event_frontier/src/visibility.rs:45-52`) currently holds `site: SiteId, agents, settlers, depot, cache_count` — no display-label field. Site display names are applied in effect text via `SiteId::label()` (confirmed: `public_effect_text` already uses `site.label()`), but the *view* carries no label for the TS board/action surfaces. Action labels are built by `selection_label`/`encode_selections` (`games/event_frontier/src/actions.rs:475`, `:747-760`) from `selection.site.as_str()` — raw IDs.
2. Spec `specs/action-consequence-and-match-context-shared-surfaces.md` §2 (verification note) and D3 require a Rust-side site display-label table (presentation TOML, same pattern as cards) projected through `SiteView`; the effect-copy alignment is card/ordinal vocabulary (sites already resolve via `site.label()`).
3. Cross-artifact boundary under audit: the public/private view projection (`SiteView`) and the static-data manifest loader. `cards_presentation.toml` is loaded in `games/event_frontier/src/ui.rs`/`cards.rs`; the new `sites_presentation.toml` rides the same typed-loader pattern (unknown fields rejected by default).
4. FOUNDATIONS §2 (behavior authority): labels are Rust output, never TS-derived from IDs. This ticket keeps label generation in Rust; TS only renders the resolved strings. No legality, no TS string-from-ID derivation.
5. Deterministic replay/hash & serialization surface: adding a display label to `SiteView` changes the view's serialized shape. The label is a pure function of the static `sites_presentation.toml` (no RNG, no wall-clock), so replay/hash stay deterministic; golden traces and serialization fixtures are regenerated as an explicit migration, not a semantics change. No hidden information enters the view (site labels are public).
6. Schema extension: `SiteView` gains a `label` field (additive) and `sites_presentation.toml` is a new static-data manifest entry. Consumers of `SiteView`: `games/event_frontier/src/visibility.rs` producers, the wasm projection, and the TS `EventFrontierBoard`/action surfaces (downstream tickets). Extension is additive (new field); no consumer breaks on read.

## Architecture Check

1. Reusing the existing `cards_presentation.toml` typed-loader pattern for `sites_presentation.toml` keeps site labels in the same sanctioned static-content channel (FOUNDATIONS §5) rather than inventing a new mechanism. Projecting the label through `SiteView` gives the shared surfaces one Rust-authored source of truth instead of per-board TS label maps.
2. No backwards-compatibility shim: the raw-ID label path is replaced, not aliased. Old golden traces are regenerated, not dual-supported.
3. `engine-core` untouched — `SiteView` and the label table are game-local (`games/event_frontier`); no mechanic noun enters the kernel. No `game-stdlib` promotion (first use of a site-label table).

## Verification Layers

1. No raw `site_`/`ef_` tokens in emitted action/accessibility labels -> codebase grep-proof + a unit test over generated labels.
2. `SiteView` label is deterministic and viewer-safe -> schema/serialization validation + no-leak visibility test (labels are public static data).
3. Replay/hash unchanged-or-migrated -> golden trace / deterministic replay-hash check (`replay-check`, `fixture-check`).
4. Effect-copy ordinal/card vocabulary aligned -> unit test on `public_effect_text` output.

## What to Change

### 1. Site display-label table + loader

Add `games/event_frontier/data/sites_presentation.toml` (typed parallel arrays: `site_ids`, `labels`, `accessibility_labels`), loaded via the same typed-loader pattern as `cards_presentation.toml` (`ui.rs`/`cards.rs`), rejecting unknown fields.

### 2. SiteView label projection

Add a `label` field to `SiteView` (`visibility.rs`), populated from the loaded site-label table. Update the view producers.

### 3. Resolved action/accessibility labels

Switch `selection_label`/`encode_selections` (`actions.rs`) to resolve display names with human list joining ("Charterhouse and Crossing"); aria-labels use resolved names.

### 4. Effect-copy ordinal/card alignment

Align `public_effect_text` reckoning/ordinal vocabulary to authored copy (e.g. ordinal "First Reckoning") so the effect log matches the deck panel.

### 5. Fixture / trace / serialization updates

Regenerate golden traces and serialization fixtures for the new view shape; add unit coverage asserting no `site_`/`ef_` token appears in emitted labels.

## Files to Touch

- `games/event_frontier/data/sites_presentation.toml` (new)
- `games/event_frontier/src/visibility.rs` (modify)
- `games/event_frontier/src/actions.rs` (modify)
- `games/event_frontier/src/ui.rs` (modify)
- `games/event_frontier/src/cards.rs` (modify; if the site loader co-locates with the card loader)
- `games/event_frontier/tests/golden_traces.rs` (modify)
- `games/event_frontier/tests/golden_traces/` (modify; regenerated fixtures)
- `games/event_frontier/tests/serialization.rs` (modify)
- `games/event_frontier/tests/visibility.rs` (modify)

## Out of Scope

- Any TypeScript rendering of the resolved labels (ACTCONMAT-003/004/005).
- Action-tree structure / path-encoding changes — submitted command bytes stay identical (forbidden by spec §11).
- Visibility-contract changes / new hidden-fact exposure (`EF-VIS-002` stance unchanged).

## Acceptance Criteria

### Tests That Must Pass

1. New unit test: every emitted action label and accessibility label for Survey/operation leaves contains no `[a-z]+_[a-z_]+` raw-ID token (`site_`, `ef_`).
2. `cargo run -p fixture-check -- --game event_frontier` and `cargo run -p replay-check -- --game event_frontier --all` pass against regenerated traces.
3. `cargo test -p event_frontier` green (visibility, serialization, golden_traces, property).

### Invariants

1. Site labels originate from `sites_presentation.toml` (Rust), never from TypeScript ID parsing (§2).
2. The `SiteView` label addition is additive and deterministic; replay/hash differ only by the documented fixture migration (§11).

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/visibility.rs` — assert `SiteView.label` resolves and no raw-ID tokens leak into labels.
2. `games/event_frontier/tests/golden_traces/` + `golden_traces.rs` — regenerated for the new view shape.
3. `games/event_frontier/tests/serialization.rs` — round-trip the extended `SiteView`.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p fixture-check -- --game event_frontier && cargo run -p replay-check -- --game event_frontier --all && cargo run -p rule-coverage -- --game event_frontier`
3. `cargo run -p simulate -- --game event_frontier --games 1000` (smoke; label resolution exercised under play)

## Outcome

Completed: 2026-06-12

Implemented Event Frontier Rust-side site presentation metadata through
`games/event_frontier/data/sites_presentation.toml`, a strict
`SitePresentationCatalog`, and `SiteView.label` projection. Operation labels and
accessibility labels now use authored site names while retaining byte-identical
raw action path segments for replay and validation. Public effect copy now uses
authored Reckoning ordinal vocabulary. The WASM Event Frontier view serializer
now emits the Rust-projected `SiteView.label` field.

Golden trace public-view and affected action-tree hashes were regenerated for
the additive deterministic view/label surface; rule state and effect behavior
were not changed intentionally.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p event_frontier` passed.
- `cargo test -p wasm-api` passed.
- `cargo run -p fixture-check -- --game event_frontier` passed.
- `cargo run -p replay-check -- --game event_frontier --all` passed.
- `cargo run -p rule-coverage -- --game event_frontier` passed.
- `cargo run -p simulate -- --game event_frontier --games 1000` passed
  (`games_run=1000`, `simulation_pass_rate_percent=100.00`).
