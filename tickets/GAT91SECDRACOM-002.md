# GAT91SECDRACOM-002: secret_draft crate skeleton + workspace wiring + data manifests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new game crate `games/secret_draft` (new `games/*` module); root `Cargo.toml` workspace member addition; new typed static-data files under `games/secret_draft/data/`. No `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-001

## Problem

`secret_draft` needs a compiling crate skeleton and workspace registration before any rules logic lands, plus its typed static-data manifest/variants/fixture shell. This is the structural foundation every later ticket builds on; without it nothing else compiles or registers.

## Assumption Reassessment (2026-06-08)

1. The crate shape is mirrored from `games/token_bazaar` (confirmed: `games/token_bazaar/src/` has exactly `actions.rs, bots.rs, effects.rs, ids.rs, lib.rs, replay_support.rs, rules.rs, setup.rs, state.rs, ui.rs, variants.rs, visibility.rs`; `games/token_bazaar/data/` has `manifest.toml, variants.toml, fixtures/`). `token_bazaar` is the structural template; per the spec (§Deliverables, Workspace-and-crate row, as updated by reassessment) `games/high_card_duel` is the closer behavioral template for the hidden-info modules in later tickets.
2. Root `Cargo.toml` lists workspace members and currently includes `"games/high_card_duel"` and `"games/token_bazaar"` (verified at lines 13–14). `games/secret_draft` must be added as a member.
3. Cross-artifact boundary under audit: the workspace member list and the static-data manifest schema. The manifest/variants/fixture parsers must reject unknown and behavior-looking fields (the schema is the contract GAT91SECDRACOM-003/009 validate).
4. §5 static-data-is-not-behavior is the motivating principle: restate before trusting the spec — `data/manifest.toml`, `data/variants.toml`, and `data/fixtures/secret_draft_standard.fixture.json` may carry only typed metadata, constants, labels, variant IDs, item IDs/threads/values, and fixtures; never selectors, conditions, triggers, or formulas. Unknown fields are rejected by default (§11).
5. Substrate-only note: this ticket builds the static-data inputs that GAT91SECDRACOM-009's fail-closed serialization/unknown-field tests and the deterministic replay surface will later enforce. The skeleton introduces no leakage or nondeterminism path: the fixture is initial-state-only (no commitments), and the manifest carries constants, not behavior. Enforcement lands in GAT91SECDRACOM-009 (serialization/unknown-field rejection) and GAT91SECDRACOM-007/010 (deterministic replay/hash).
6. Schema extension check: the static-data manifest entry is a new instance of the existing per-game manifest pattern, not an extension of a shared schema; it is additive (a new game's own files). Parsers are game-local in `games/secret_draft` and consume only this game's files.

## Architecture Check

1. A thin compiling skeleton first (types stubbed, parsers + reject-unknown wired) is cleaner than a monolithic first commit: it lets every later ticket be a focused reviewable diff against a known module layout.
2. No backwards-compatibility aliasing/shims — all new files; the workspace member is a clean addition.
3. `engine-core` stays free of mechanic nouns — all draft/pool/tile/commitment nouns are game-local to `games/secret_draft`. No `game-stdlib` helper is added (atlas §10A open-debt register is `_None_`; `secret_draft` is first official local use of simultaneous commitment/reveal, not a promotion trigger).

## Verification Layers

1. Workspace membership -> `cargo build -p secret_draft` succeeds (grep-proof: `secret_draft` in root `Cargo.toml` members).
2. Static-data schema rejects unknown/behavior-looking fields -> serialization/parse unit test in this crate (fail-closed parse) + later full coverage in GAT91SECDRACOM-009.
3. Boundary: `engine-core` noun-free -> `bash scripts/boundary-check.sh`.
4. Fixture is initial-state-only (no commitments) -> manual review + parse test asserting empty commitment slots.

## What to Change

### 1. Workspace registration

Add `"games/secret_draft"` to the `members` list in root `Cargo.toml` (alongside `games/high_card_duel`, `games/token_bazaar`).

### 2. Crate manifest + module skeleton

Create `games/secret_draft/Cargo.toml` (mirror `games/token_bazaar/Cargo.toml` deps/edition) and the twelve source modules: `src/lib.rs` (module wiring + public re-exports), `src/ids.rs`, `src/state.rs`, `src/setup.rs`, `src/variants.rs`, `src/actions.rs`, `src/rules.rs`, `src/effects.rs`, `src/visibility.rs`, `src/replay_support.rs`, `src/bots.rs`, `src/ui.rs`. Stub each with the minimal types/signatures so the crate compiles; substantive logic lands in later tickets. Define typed ID newtypes (`DraftItemId`, variant ID) and the manifest/variants/fixture parser entry points with unknown-field rejection here.

### 3. Static data

Create `games/secret_draft/data/manifest.toml` (game id, version, display name `Veiled Draft`, twelve item entries with `item_id`/`thread`/`value`/`label`, setup constants: rounds=6, seats=2), `games/secret_draft/data/variants.toml` (`secret_draft_standard`), and `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json` (initial state: full visible pool in stable order, empty commitments, empty drafted collections, scores 0, round 1, priority seat_0). Typed metadata only — no behavior-looking fields.

## Files to Touch

- `Cargo.toml` (modify)
- `games/secret_draft/Cargo.toml` (new)
- `games/secret_draft/src/lib.rs` (new)
- `games/secret_draft/src/ids.rs` (new)
- `games/secret_draft/src/state.rs` (new)
- `games/secret_draft/src/setup.rs` (new)
- `games/secret_draft/src/variants.rs` (new)
- `games/secret_draft/src/actions.rs` (new)
- `games/secret_draft/src/rules.rs` (new)
- `games/secret_draft/src/effects.rs` (new)
- `games/secret_draft/src/visibility.rs` (new)
- `games/secret_draft/src/replay_support.rs` (new)
- `games/secret_draft/src/bots.rs` (new)
- `games/secret_draft/src/ui.rs` (new)
- `games/secret_draft/data/manifest.toml` (new)
- `games/secret_draft/data/variants.toml` (new)
- `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json` (new)

## Out of Scope

- Substantive state/setup/action/effect/visibility/bot logic (GAT91SECDRACOM-003 onward) — stubs only here.
- Tool/WASM/CI/web registration (GAT91SECDRACOM-012/013/016).
- Tests beyond a minimal parse/reject-unknown unit test (full suite in GAT91SECDRACOM-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p secret_draft` succeeds.
2. `cargo build --workspace` succeeds (member wiring correct).
3. A manifest/variants/fixture parse unit test passes and an unknown-field input is rejected (fail-closed).

### Invariants

1. `engine-core` gains no mechanic noun (`bash scripts/boundary-check.sh` passes).
2. Static data files contain only typed metadata/constants/labels/IDs/fixtures — no selectors/conditions/triggers/formulas.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/<parser module>` inline unit test — manifest/variants/fixture parse + unknown-field rejection rationale.

### Commands

1. `cargo build -p secret_draft`
2. `cargo build --workspace && cargo test -p secret_draft && bash scripts/boundary-check.sh`
3. Full simulation/replay verification is not applicable yet (no rules logic until GAT91SECDRACOM-003+); the build + parse test is the correct boundary for a skeleton ticket.
