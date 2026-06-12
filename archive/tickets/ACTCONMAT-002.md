# ACTCONMAT-002: Reserved action-metadata key convention + explanation templates

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier` (explanation-template static data + `ui.rs` projection of tag → plain-language text) and `apps/web/src/wasm/client.ts` (presentation-only type additions). No `engine-core` change.
**Deps**: None

## Problem

Rust already attaches affordance metadata to operation leaves (`actions.rs:480-488`: `op`, `site_count`, `cost`, `ops_bound`, `eligibility_consequence`) and to the operation-kind node (`actions.rs:435`: `cost_rule = "base_one_resource_per_site"`), and the TS `ActionChoice` type already carries `metadata`/`tags`. But there is no documented convention for which keys are reserved presentation keys, and no plain-language source for the consequence/cost-rule tags — `eligibility_consequence = "acting_forfeits_next_card"` is an opaque token. The shared surfaces (ACTCONMAT-003) need a typed convention plus authored explanation templates keyed to those tags.

## Assumption Reassessment (2026-06-12)

1. Operation-leaf metadata keys are `op`, `site_count`, `cost`, `ops_bound`, `eligibility_consequence` (`games/event_frontier/src/actions.rs:480-488`); `cost_rule` is on the operation-kind node (`actions.rs:435`), NOT the leaf. The reserved-key projection must read each from where it is emitted.
2. Spec D1 / §4.2: reserved presentation keys are `cost` + `eligibility_consequence` (leaf) and `cost_rule` (op-kind node); `op`/`site_count`/`ops_bound` are existing internal keys the surfaces ignore. The UI-INTERACTION reserved-key table text is *drafted* here and lifted into the doc at ACTCONMAT-012.
3. Cross-artifact boundary: the `ActionChoice.metadata` opaque key/value contract (`engine-core` kernel) and the per-game explanation-template static data. `client.ts` `ActionChoice` (`apps/web/src/wasm/client.ts:1021-1028`) already accepts `metadata?: Array<{key,value}>` and `tags?: string[]`.
4. FOUNDATIONS §3 (engine-core is a contract kernel): the kernel must not know what `cost`/`cost_rule` mean. The key *convention* is documented in UI law and implemented per game; metadata stays opaque strings in the kernel. FOUNDATIONS §5: explanation templates keyed to Rust actions are the sanctioned static-content category (typed, unknown fields rejected, no selectors/branches).
5. Schema: `ActionChoice.metadata`/`tags` are existing fields (no change); the addition is the explanation-template static data + a typed projection of tag → plain-language string. Consumers: the shared action surfaces (ACTCONMAT-003). Additive — no existing consumer breaks.

## Architecture Check

1. Documenting reserved keys in UI law (a doc table) rather than typing them into `engine-core` keeps the kernel noun-free while giving the surfaces a stable contract — the minimal home at current pressure (two games emit candidate keys). Explanation templates as keyed static data reuse the §5 sanctioned category instead of computing strings in TS.
2. No shim: the convention is new; no alias to a prior key scheme.
3. `engine-core` untouched (metadata stays opaque). No `game-stdlib` promotion — explanation-template shape is first-use, kept local to `games/event_frontier`.

## Verification Layers

1. Reserved keys read from their actual emission site (leaf vs op-kind) -> codebase grep-proof + unit test on the projection.
2. Tag → plain-language resolution is static and typed -> schema/serialization validation (unknown fields rejected).
3. No hidden information in the templates -> no-leak visibility test (templates are public authored prose keyed to public tags).

## What to Change

### 1. Explanation-template static data

Add per-game explanation templates (typed, in `games/event_frontier/data/`) mapping consequence/cost-rule tags to plain-language strings — e.g. `acting_forfeits_next_card` → "Acting now forfeits your eligibility for the next card."; `base_one_resource_per_site` → its rule-reference phrasing. Reject unknown fields.

### 2. Tag projection

Project the resolved plain-language strings through the `ui.rs` metadata channel so the wasm view carries them (the surfaces never invent text).

### 3. client.ts types

Extend the presentation types so the shared surfaces can read reserved keys + resolved template text. No legality, no behavior.

### 4. Draft the UI-INTERACTION reserved-key table

Author the §5/§9 reserved-key table text inside this ticket's notes for lifting at ACTCONMAT-012 (not applied to the doc here, per spec §10 "applied at WB10, not before").

## Files to Touch

- `games/event_frontier/data/` (new explanation-template file)
- `games/event_frontier/src/ui.rs` (modify; tag → text projection)
- `apps/web/src/wasm/client.ts` (modify; reserved-key/template type additions)

## Out of Scope

- Rendering the keys/templates in the action surfaces (ACTCONMAT-003).
- Editing `docs/UI-INTERACTION.md` — the table is drafted here, lifted at ACTCONMAT-012.
- Any `engine-core` change — metadata stays opaque.

## Acceptance Criteria

### Tests That Must Pass

1. Unit test: the projection resolves `eligibility_consequence`/`cost_rule` tags to authored strings, reading `cost_rule` from the op-kind node and `eligibility_consequence` from the leaf.
2. Loader rejects an unknown field in the explanation-template file (fail-closed).
3. `cargo test -p event_frontier` green; `npm --prefix apps/web run build` typechecks the new `client.ts` types.

### Invariants

1. `engine-core` never learns the meaning of `cost`/`cost_rule` (§3); meaning lives in UI law + per-game data.
2. Explanation templates are inert typed content — no selectors, branches, or triggers (§5).

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/` (or `ui.rs` unit module) — tag → template resolution + unknown-field rejection.
2. `apps/web` typecheck — `client.ts` additions compile.

### Commands

1. `cargo test -p event_frontier`
2. `npm --prefix apps/web run build`
3. `bash scripts/boundary-check.sh` (engine-core stays noun-free)

## Outcome

Completed: 2026-06-12

Implemented Event Frontier action-affordance explanation templates as inert,
typed static presentation data in
`games/event_frontier/data/action_affordance_templates.toml`. The new
`ActionAffordanceTemplateCatalog` rejects unknown fields and duplicate/empty
entries, and `UiMetadata.action_affordance_templates` projects resolved public
text through Rust/WASM for later shared action surfaces. The existing
`ActionChoice.metadata` keys remain opaque to `engine-core`; no kernel changes
were made.

The projection covers the current reserved tags:

- `eligibility_consequence = acting_forfeits_next_card` on operation leaves.
- `cost_rule = base_one_resource_per_site` on operation-kind nodes.

Draft for ACTCONMAT-012 lift into `docs/UI-INTERACTION.md` §5/§9:

```text
Action-tree leaves MAY carry reserved presentation metadata keys with fixed
meaning: `cost` (viewer-visible cost in the acting seat's primary resource),
`cost_rule` (stable rule-reference tag), `eligibility_consequence` (stable
consequence tag resolved through authored explanation templates). When a game
emits reserved keys, shared action surfaces MUST render them at choice and
confirmation time. Reserved keys are documented here, not typed into
engine-core; the kernel treats metadata as opaque.
```

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p event_frontier` passed.
- `cargo test -p wasm-api` passed.
- `cargo run -p replay-check -- --game event_frontier --all` passed.
- `npm --prefix apps/web run build` passed.
- `bash scripts/boundary-check.sh` passed.
