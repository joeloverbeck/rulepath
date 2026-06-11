# EFFMAP-001: Disambiguate masked_claims effect-log discriminators (fix `undefined is now active`)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (`masked_effect_json` browser projection only); `apps/web/src/components/effectFeedback.ts`. No `engine-core`/`game-stdlib`/`games/*` rules, traces, hashes, or schemas change.
**Deps**: None

## Problem

Playing `masked_claims`, the effect log renders `Turn advanced` / `undefined is now active.`. Root cause: the browser-projection effect `type` discriminator is a **flat global namespace** consumed by a single TypeScript `switch` in `apps/web/src/components/effectFeedback.ts`, but two games emit the **same discriminator with divergent payload fields**:

- `token_bazaar` → `{"type":"turn_advanced","previous_seat","active_seat","turns_taken"}` (`crates/wasm-api/src/lib.rs:6588`)
- `masked_claims` → `{"type":"turn_advanced","turn","claimant"}` (`crates/wasm-api/src/lib.rs:7091`)

The TS `case "turn_advanced"` (`effectFeedback.ts:427`) reads `payload.active_seat`, which does not exist on the masked_claims shape → `undefined`. A sibling collision exists for `score_changed`: `masked_claims` emits `{seat,delta,total,reason}` (`lib.rs:7085`) but the TS `case "score_changed"` (`effectFeedback.ts:223`) returns the hard-coded `secret_draft` text *"Secret Draft scores were updated by Rust."*, mislabeling a masked_claims event.

This ticket fixes both collisions at the source by giving masked_claims unique discriminators and dedicated render cases. The cross-game regression guard that prevents recurrence is EFFMAP-002.

## Assumption Reassessment (2026-06-11)

1. The collision is real and confined to the browser projection: `masked_effect_json` in `crates/wasm-api/src/lib.rs:7091` (TurnAdvanced → `turn_advanced`) and `:7085` (ScoreChanged → `score_changed`); the TS consumers are `effectFeedback.ts:427` (reads `payload.active_seat`) and `:223` (hard-coded secret_draft text). Verified by direct read.
2. The Rust effect enum `MaskedClaimsEffect::TurnAdvanced { turn, claimant, log }` and `::ScoreChanged { seat, delta, total, reason }` (`games/masked_claims/src/effects.rs:72,66`) are unchanged by this ticket — only their `wasm-api` JSON `type` string changes. No `docs/` contract names these discriminator strings (checked `docs/WASM-CLIENT-BOUNDARY.md`, `docs/UI-INTERACTION.md`, `docs/OFFICIAL-GAME-CONTRACT.md`).
3. Shared boundary under audit: the Rust↔browser effect-envelope JSON contract (`docs/WASM-CLIENT-BOUNDARY.md` §Effects, `docs/ENGINE-GAME-DATA-BOUNDARY.md`). The discriminator string is the browser-projection key; it is NOT the canonical effect representation used for hashing.
4. FOUNDATIONS principle restated: §11 "Semantic effects drive animation; renderer diffs are diagnostics only" and §11 "Public/private views are viewer-safe" / no hidden-information leak through effect logs. The fix keeps Rust authoritative over effect content and adds no legality logic to TS (§11 "TypeScript does not decide legality").
5. Deterministic replay/hash surface confirmed unaffected: golden traces store numeric `expected_effect_hashes` computed over the Rust effect (e.g. `games/masked_claims/tests/golden_traces/shortest-normal.trace.json` → `"expected_effect_hashes":{"final":10102}`); no JSON file in the repo contains the string `turn_advanced`, and no `crates/wasm-api` test pins it. Renaming the browser-projection discriminator is therefore NOT a hash/replay migration. The masked_claims `claimant`/`turn`/`seat`/`delta`/`total` fields are already public (present in the public view at `lib.rs:5833`), so no new field is exposed — no §11 no-leak concern.
6. Schema extension classification: this changes the `type` string value of two browser-projection effect envelopes. The only consumer of these two strings is `effectFeedback.ts` (grep-verified: the strings appear nowhere else in `apps/web/src` or `apps/web/scripts`, and `MaskedClaimsBoard.tsx` does not switch on `payload.type`). The change is therefore self-contained, not a breaking schema change for any other consumer.
7. Rename blast radius (repo-wide grep): `"turn_advanced"` and `"score_changed"` as TS cases occur only in `apps/web/src/components/effectFeedback.ts`. `token_bazaar` retains `turn_advanced` and `secret_draft` retains `score_changed` (their existing TS cases are unchanged). No `docs/`, `specs/`, `templates/`, or `.claude/skills/` reference these discriminator strings.
8. Adjacent contradiction handling: the `round_scored` field-presence branching (`effectFeedback.ts:168`) is fragile but currently SAFE (plain_tricks has `round_counts`, high_card_duel has `winner`); it is NOT fixed here — it is covered by the EFFMAP-002 regression guard, not a separate bug fix.

## Architecture Check

1. Unique per-game discriminators are cleaner than TS-side field-presence branching (`active_seat ?? claimant`): a single global `switch` keyed on a non-unique discriminator is inherently ambiguous, and presence-branching is exactly the fragile pattern already weakening `round_scored`. Each game owning a distinct discriminator keeps the render mapping unambiguous and additive.
2. No backwards-compatibility alias/shim: the old masked_claims `turn_advanced`/`score_changed` strings are removed from the masked projection, not dual-emitted. No consumer depends on them (item 7).
3. `engine-core` stays noun-free (untouched); `game-stdlib` untouched; no mechanic-atlas pressure (this is a presentation-projection naming fix, not a mechanic).

## Verification Layers

1. Discriminator uniqueness (no masked_claims effect shares a `type` string with a divergent shape) -> codebase grep-proof over `crates/wasm-api/src/lib.rs` + `effectFeedback.ts`.
2. Effect-envelope JSON conformance (masked_claims emits the new strings with the expected fields) -> `crates/wasm-api` serialization assertion / `apps/web/scripts/smoke-ui.mjs` masked_claims effect check.
3. Rendered feedback carries no `undefined` for masked_claims turn-advance and score-change -> `effectFeedback` render check (delivered by EFFMAP-002; named here as the consuming proof).
4. Deterministic replay/hash unaffected -> `cargo run -p replay-check -- --game masked_claims --all` (effect hashes unchanged because the Rust effect is unchanged).

## What to Change

### 1. Rename the two colliding masked_claims discriminators in `crates/wasm-api/src/lib.rs` (`masked_effect_json`)

- `MaskedClaimsEffect::TurnAdvanced` (`:7091`): emit `"type":"claim_turn_advanced"` (keep fields `turn`, `claimant`).
- `MaskedClaimsEffect::ScoreChanged` (`:7085`): emit `"type":"claim_score_changed"` (keep fields `seat`, `delta`, `total`, `reason`).
- Leave `token_bazaar` `turn_advanced` (`:6588`) and `secret_draft` `score_changed` (`:7183`) untouched.

### 2. Add dedicated render cases in `apps/web/src/components/effectFeedback.ts`

- `case "claim_turn_advanced"`: `{ title: "Turn advanced", detail: \`${payload.claimant} is now active.\`, tone: "turn" }`.
- `case "claim_score_changed"`: `{ title: "Score changed", detail: \`${payload.seat} now holds ${payload.total} (+${payload.delta}).\`, tone: "turn" }` (use the masked_claims `seat`/`delta`/`total` fields; do not reuse the secret_draft text).
- Leave the existing `case "turn_advanced"` and `case "score_changed"` for token_bazaar / secret_draft unchanged.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — `masked_effect_json` only)
- `apps/web/src/components/effectFeedback.ts` (modify)

## Out of Scope

- The cross-game no-`undefined` regression harness (EFFMAP-002).
- `round_scored` fragility hardening (covered by EFFMAP-002's guard; no behavior change here).
- Any change to the Rust effect enum, public/terminal views, traces, or hashes.
- Typed discriminated-union refactor of the TS `payload` bag (deferred; not needed for this fix).

## Acceptance Criteria

### Tests That Must Pass

1. Playing/driving `masked_claims` through to a second turn renders `Turn advanced` / `<seat> is now active.` (never `undefined`) and a score change renders the masked_claims seat/total, not the secret_draft text — proven by the EFFMAP-002 guard or `npm --prefix apps/web run smoke:ui`.
2. `cargo run -p replay-check -- --game masked_claims --all` passes (effect hashes unchanged).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace && cargo test --workspace` and `npm --prefix apps/web run build`.

### Invariants

1. No masked_claims browser-projection effect shares a `type` discriminator with another game's divergent payload shape.
2. The masked_claims effect content (Rust enum, fields, visibility) and all replay/effect/state hashes are unchanged; only the browser-projection `type` strings and their TS render cases change.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — extend the existing masked_claims section to assert the emitted effect carries `type: "claim_turn_advanced"` after a turn advance (additive assertion; full render coverage lands in EFFMAP-002).
2. `crates/wasm-api/src/lib.rs` (or its test module) — if a serialization unit assertion exists for masked effects, pin the two new discriminator strings; otherwise rely on the smoke assertion above.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `cargo run -p replay-check -- --game masked_claims --all && cargo test --workspace`
3. `cargo run -p fixture-check -- --game masked_claims` — confirms effect fixtures remain valid (narrow per-game boundary; only masked_claims projection changed).

## Outcome

Completed: 2026-06-11

What changed:

- Renamed masked_claims browser-projection effect discriminators to `claim_score_changed` and `claim_turn_advanced` in `crates/wasm-api/src/lib.rs`.
- Added dedicated `feedbackForEffect` render cases for the new masked_claims effect types in `apps/web/src/components/effectFeedback.ts`.
- Extended `apps/web/scripts/smoke-ui.mjs` to assert masked_claims response effects use the claim-specific discriminators.

Deviations from original plan:

- No Rust unit serialization assertion was added because the existing web smoke path directly exercises the WASM-emitted browser projection and now pins both renamed discriminators.

Verification:

- `npm --prefix apps/web run smoke:ui` — passed.
- `cargo run -p replay-check -- --game masked_claims --all` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace` — passed.
- `cargo test --workspace` — passed.
- `npm --prefix apps/web run build` — passed.
- `cargo run -p fixture-check -- --game masked_claims` — passed.
