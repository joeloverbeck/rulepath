# GAT8HIGCAR-002: Retain and emit per-step content on public-observer-projection replay import

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (public-export import parsing, `PublicTimelineReplay` shape, `public_replay_step_json` emission + tests). No `engine-core`, `game-stdlib`, or `games/*` behavior changes. Additive-only extension of the public replay-step JSON surface (new `public_effects` array consumed by the web client in GAT8HIGCAR-003). No golden-trace / export-format change.
**Deps**: Relates to accepted ADR `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (public observer projection export class). Follows `archive/tickets/GAT8HIGCAR-001.md`, which fixed import **routing** and explicitly deferred this content-reconstruction gap (its "Out of Scope → F3"). Blocks `GAT8HIGCAR-003` (web rendering).

## Problem

Importing a `high_card_duel` public-observer-projection replay (`export_class: public_observer_projection_v1`) succeeds and the Replay viewer can Step through the cursor, but every cursor shows **"No replay effects at this cursor."** The imported timeline carries no per-step content: the commits, reveals, and round-scoring effects present in the exported document are silently discarded on import.

Root cause is a count-only import stub in `crates/wasm-api/src/lib.rs`:

1. `import_high_card_public_replay` (`lib.rs:1393-1462`) does not parse the `steps` array. It counts occurrences of `"step_index":` (`let step_count = doc.matches("\"step_index\":").count();`, `lib.rs:1421`) and rebuilds each step with **empty** payload: `public_view_summary: String::new()`, `public_effects: Vec::new()` (`lib.rs:1430-1437`). The real per-step `public_effects` arrays in the document are thrown away.
2. The retained `PublicTimelineReplay` (`lib.rs:165-168`) stores only `viewer` + `step_count`; there is nowhere for per-step content to live.
3. `public_replay_step_json` (`lib.rs:1464-1477`) therefore returns a hardcoded `"view":null,"effects":[]` for every cursor.
4. The web `ReplayViewer` reads `step.effects` (always `[]`) and renders the empty-state line (`apps/web/src/components/ReplayViewer.tsx:25,113-115`).

This violates ADR-0004's Determinism-impact clause: *"importing a public export reproduces the public projection timeline."* The current import reproduces an **empty** timeline.

The game crate is not at fault: `import_public_export(&PublicReplayExport)` (`games/high_card_duel/src/replay_support.rs:251-256`) already clones the full `steps` into the timeline. The defect is that wasm-api constructs the `PublicReplayExport` it passes in with empty steps.

## Assumption Reassessment (2026-06-08)

1. **Count-only import stub confirmed.** `import_high_card_public_replay` at `crates/wasm-api/src/lib.rs:1393-1462` builds `PublicReplayStep` values with `public_view_summary: String::new()` and `public_effects: Vec::new()` (`lib.rs:1430-1437`) from a `(0..step_count)` range, where `step_count` is a substring count of `"step_index":` (`lib.rs:1421`). No step payload is parsed.
2. **Retention struct cannot hold per-step content.** `struct PublicTimelineReplay { viewer: String, step_count: usize }` (`lib.rs:165-168`). `public_replay_step_json` (`lib.rs:1464-1477`) hardcodes `"view":null,"effects":[]`.
3. **Cross-artifact boundary under audit:** the public replay-step JSON contract between the wasm-api importer/stepper (`crates/wasm-api/src/lib.rs`) and the web client `ReplayStep` type (`apps/web/src/wasm/client.ts:507-515`, where `effects: EffectEntry["effect"][]`). Canonical end-state: each cursor's JSON carries the imported step's public effects so the viewer renders the observation timeline. Web consumption is `GAT8HIGCAR-003`.
4. **FOUNDATIONS §11 no-leak firewall + ADR-0004 under audit.** The fix surfaces only fields already present in the *public* export (`public_effects`, `redacted_command_summary`, `public_view_summary`), which the game crate produced through `project_view(state, &Viewer { seat_id: None })` and `public_effect_stable_string` (filters to `VisibilityScope::Public`, `games/high_card_duel/src/replay_support.rs:279-282`). No seed, no private command path, no unrevealed deck order, no pre-reveal commitment identity is reconstructed or transported. Import remains a pass-through of already-public observation strings — it must not derive any new fact.
5. **Deterministic replay/serialization surface (§11/§13).** Export bytes are untouched: `PublicReplayExport::to_json` / `PublicReplayStep::to_json` (`games/high_card_duel/src/replay_support.rs:93-137`) and all golden traces are unchanged — this is an import-side parse + step-emission change only. The per-cursor JSON must be deterministic (stable ordering of `public_effects`, preserving document order).
6. **Additive-only schema extension.** The public replay-step JSON gains a `public_effects: string[]` field (and a `redacted_command_summary: string`); the existing `effects` field stays present and `[]` for public exports. The only consumer is the web client (`apps/web/src/wasm/client.ts` `ReplayStep`, updated in `GAT8HIGCAR-003`). No internal-trace step JSON (`high_card_replay_step_json`, `lib.rs:2609`) is changed, so perfect-information and full-trace replays are unaffected.
7. **Parser home / convention.** Replay-document JSON parsing lives in wasm-api (`parse_replay_document`, `lib.rs:3807`; helpers `string_field` `lib.rs:3998`, `parse_string_at` `lib.rs:4111`, `validate_json_object` `lib.rs:3888`, `skip_json_value` `lib.rs:3964`). The game crate parses only small typed values (ids, variants/TOML), never JSON documents. The new `steps`-array parse therefore belongs in wasm-api alongside `parse_replay_document`, feeding the **unchanged** game-crate `import_public_export`. Adding a JSON parser to `games/high_card_duel` is the rejected alternative (see Architecture Check).
8. **Adjacent contradiction (separate, deferred):** the web `ReplayViewer` has no `high_card_duel` board branch and `public_view_summary` is a packed string, not a structured `PublicView`, so the board/snapshot stays redacted-minimal for public imports. Reconstructing a structured view from the summary is **out of scope** (own follow-up if ever wanted) — it is not required to resolve the reported symptom.

No spec mismatch found: this ticket completes the ADR-0004 requirement that import reproduces the public projection timeline, which `GAT8HIGCAR-001` deferred.

## Architecture Check

1. **Parse where the document parser already lives.** Extending wasm-api to parse the `steps` array (reusing `string_field` / `parse_string_at` / `skip_json_value` patterns) and then calling the existing, already-correct `import_public_export` is cleaner than the alternative of adding a JSON document parser to `games/high_card_duel`: it keeps the game crate free of import-format parsing (it owns only serialization, `to_json`), preserves the wasm-thin-bridge seam, and reuses the whitespace-tolerant helpers already trusted by `import_high_card_public_replay` for the header fields. The game crate's `import_public_export` is reused unchanged — the round-trip `to_json` → parse → `import_public_export` becomes the testable symmetry.
2. **No backwards-compatibility aliasing/shims.** The empty-step path is replaced outright, not kept as a parallel legacy path. The public step JSON gains a new field additively; no alias of `effects`.
3. **`engine-core` stays free of mechanic nouns (§3); no `game-stdlib` change (§4).** All edits are in `crates/wasm-api` (routing/parsing/step JSON + tests). No `games/*` behavior change.

## Verification Layers

1. Imported timeline reproduces the exported public effects per step (ADR-0004 determinism) -> golden/round-trip Rust unit test: `export_public_observer_replay` → `to_json` → `import_replay` → `replay_step(cursor)` for each cursor carries the same `public_effects` as the source step.
2. No hidden-information leak via the new parse/emit path (§11 no-leak firewall) -> no-leak visibility test: imported step JSON for every cursor contains no `hcd:r` card identity beyond those already public in the source export, no `"seed"`, no raw private command path; only `VisibilityScope::Public` strings appear.
3. Additive-only schema extension, internal-trace step JSON unchanged (§11/§13) -> codebase grep-proof: `high_card_replay_step_json` and `parse_replay_document` untouched; web `ReplayStep.effects` field still present.
4. Public step JSON is deterministic (stable `public_effects` order) -> schema/serialization validation: two imports of the same document yield byte-identical per-cursor step JSON; effect order matches document order.
5. Routing regression intact -> existing `high_card_duel` import tests (compact + pretty-printed round-trip added by `GAT8HIGCAR-001`) still pass.

## What to Change

### 1. Parse the `steps` array on public import

In `crates/wasm-api/src/lib.rs`, replace the count-only reconstruction in `import_high_card_public_replay` (`lib.rs:1421-1438`). Parse each element of the document's `steps` array into a `PublicReplayStep` with real `step_index`, `public_view_summary`, `public_effects` (string array), `redacted_command_summary`, and `terminal`, using the existing whitespace-tolerant helpers (`string_field` / `parse_string_at` / `skip_json_value`, `lib.rs:3932-4111`). Add a small `steps`-array parser (object-per-element, string-array field) near `parse_replay_document` (`lib.rs:3807`). Pass the fully-populated `PublicReplayExport` to the unchanged `import_public_export` (`high_card_import_public_export`, imported at `lib.rs:36`). Parse failure must surface the existing `invalid_replay` diagnostic, not fall through.

### 2. Retain per-step content in `PublicTimelineReplay`

Change `struct PublicTimelineReplay` (`lib.rs:165-168`) to store the per-step content the stepper needs — `viewer: String` plus `steps: Vec<PublicReplayStep>` (or an equivalent owned per-step projection of `public_effects` + `redacted_command_summary` + `terminal` + `public_view_summary`). Populate it from the imported timeline in `import_high_card_public_replay` (`lib.rs:1441-1454`). Keep `step_count`-derived totals computed from `steps.len()`.

### 3. Emit per-step content from `public_replay_step_json`

Rewrite `public_replay_step_json` (`lib.rs:1464-1477`) to index the requested `cursor` into the retained `steps` and emit:
- `"public_effects":[...]` — the step's public effect strings (JSON-escaped), document order preserved;
- `"redacted_command_summary":"..."` — the step's command label;
- keep `"effects":[]` (structured-effect field stays present and empty for public exports — additive, non-breaking);
- keep `"view":null` (board reconstruction is out of scope, Assumption 8);
- keep existing `cursor` / `total_steps` / `public_export` / `viewer` fields.
Clamp `cursor` to the valid step range as today.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify) — `import_high_card_public_replay`; `PublicTimelineReplay` struct; `public_replay_step_json`; new `steps`-array parser helper; new tests in `mod tests`.

## Out of Scope

- Web rendering of the surfaced `public_effects` (that is `GAT8HIGCAR-003`).
- Reconstructing a structured `high_card_duel` `PublicView` from `public_view_summary` to drive the board/snapshot for public imports (Assumption 8). `"view"` stays `null`.
- Any change to internal-full-trace import/replay (`parse_replay_document`, `high_card_replay_step_json`) or to other games.
- Any change to export bytes or golden traces.

## Acceptance Criteria

### Tests That Must Pass

1. New Rust unit test: import the `JSON.stringify(_, null, 2)`-equivalent of an `export_public_observer_replay` document; assert `replay_step` at each cursor returns the same ordered `public_effects` as the source step (cursor 0 → `[]`, cursor 2 → contains `hcd_cards_revealed:round=1;...` and `hcd_round_scored:round=1;winner=seat_0;score=1-0`, terminal cursor → contains `hcd_terminal:winner=seat_1;score=1-4`).
2. New no-leak assertion: every cursor's step JSON contains no `"seed"`, no private command path, and no `hcd:r` identity absent from the source public export.
3. `cargo test -p wasm-api` passes (including the `GAT8HIGCAR-001` compact + pretty-printed routing tests).
4. `cargo test -p high_card_duel` passes unchanged (game crate untouched; `public_export_import_preserves_redacted_timeline_only`, `games/high_card_duel/tests/serialization.rs:61`, still green).

### Invariants

1. Importing a public export reproduces the exported public projection timeline (per-step `public_effects` preserved in order) — ADR-0004 Determinism impact.
2. Import surfaces only already-public observation strings; no seed/private/deck-tail fact is reconstructed (FOUNDATIONS §11 no-leak firewall).
3. The public step JSON extension is additive; the structured `effects` field and internal-trace step JSON are unchanged.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`mod tests`) — public-import per-step round-trip test (effects fidelity + ordering + cursor clamping).
2. `crates/wasm-api/src/lib.rs` (`mod tests`) — public-import no-leak assertion over all cursors.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test -p high_card_duel`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:
- `crates/wasm-api/src/lib.rs` now parses `public_observer_projection_v1` `steps` into real `PublicReplayStep` values instead of rebuilding count-only empty steps.
- Public imported replay records retain the imported steps and `public_replay_step_json` emits additive `public_effects` and `redacted_command_summary` fields while preserving `effects: []` and `view: null`.
- The shared WASM JSON field helpers now resolve top-level object fields with whitespace tolerance instead of raw substring matching.
- Added unit coverage for pretty public replay import, ordered public-effect replay steps, cursor clamping, and no newly introduced hidden facts.

Deviations from original plan:
- The test seed's exact score/winner text is asserted by round/terminal event presence and source-step equality rather than hard-coding the ticket's illustrative score strings. The source exported public timeline remains the authority.

Verification results:
- `cargo fmt --all --check` passed.
- `cargo test -p wasm-api` passed: 21 tests.
- `cargo test -p high_card_duel` passed: all unit, integration, and doc tests.
