# GAT201STACROHOP-003: Governed determinism migration + evidence reconciliation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — WASM API action-tree snapshot, golden trace / fixture verification, benchmark baseline review, `games/starbridge_crossing/docs/GAME-EVIDENCE.md`
**Deps**: GAT201STACROHOP-001, GAT201STACROHOP-002

## Problem

The `legal_jump_landings` guard in `GAT201STACROHOP-001` narrows the enumerated
action tree (origin-return nodes are gone), which changes dependent action-tree
hashes, golden traces, and replay fixtures. The game's evidence artifacts go
**stale the moment the guard lands**: the golden-trace round-trip assertions in
`tests/serialization.rs` (`include_str!` at `:61-78`) and `tests/bots.rs`, plus
`replay-check --all` / `fixture-check`, fail until the affected artifacts are
regenerated. This ticket performs the governed, per-artifact ADR-0009 migration —
regenerating **only** the artifacts whose pre-fix state offered an origin-return
continuation, with reviewed hashes and per-artifact authority notes — and closes
the gate's exit criteria.

## Assumption Reassessment (2026-06-27)

1. Golden traces live in `games/starbridge_crossing/tests/golden_traces/` (22
   files) and are asserted via `include_str!` in `tests/serialization.rs`
   (`:61-78`) and `tests/bots.rs` (`bot-l0-action.trace.json`). The affected
   subset is the traces whose recorded action tree offered an origin-return
   continuation — candidate jump/multi-hop traces (`jump-chain`,
   `multi-hop-change-direction`, `jump-chain-stop-midway`, `bot-l0-action`) — with
   the **exact** set determined in-ticket by inspecting which recorded states had
   an origin-return node. Setup / terminal / visibility / blocked-pass traces that
   never offered one stay byte-identical (no blanket regen).
2. Spec `specs/gate-20-1-...-prohibition.md` §5 STACROSORIG-004, §6 exit criteria,
   and §8 require an explicit ADR-0009-governed migration. ADR 0009
   (`docs/adr/0009-replay-fixture-hash-taxonomy.md`) is the authority for
   per-artifact regeneration with authority notes — confirm in-ticket whether it
   requires a `rules_version`/`manifest.toml` bump for an action-tree-narrowing
   change, and follow it (the spec scopes this as a bounded per-artifact
   migration, not a blanket version cutover).
3. Cross-artifact boundary under audit: golden traces ↔ `tests/serialization.rs`
   round-trip ↔ `replay-check` / `fixture-check` tools ↔ `data/fixtures/*.fixture.json`
   ↔ `benches/thresholds.json`. The shared invariant is that every regenerated
   artifact reflects **only** the origin-return removal — each hash reviewed
   individually, never bulk-accepted.
4. FOUNDATIONS §11 (replay/hashes/serialization/traces remain deterministic **or
   are explicitly migrated**) and §13 (a replay/hash-semantics change is
   ADR-governed — here the already-accepted ADR 0009) motivate this ticket. A
   legality narrowing reveals no hidden information, so the public/private view
   projection and the no-leak firewall are unaffected.
5. Determinism/no-leak enforcement surfaces: `replay-check --all`, `fixture-check`,
   the `tests/serialization.rs` golden round-trip, and `benches/thresholds.json`.
   Confirm regeneration introduces no nondeterministic input into canonical forms
   (deterministic RNG only) and preserves viewer-safe redaction — the visibility
   traces (`public-observer-all-public`, `seat-viewer-parity`,
   `public-replay-round-trip`) must remain byte-identical, since legality
   narrowing changes no projection.

## Architecture Check

1. Per-artifact, authority-annotated regeneration limited to states that offered
   an origin-return node is cleaner and safer than a blanket golden regen: the
   diff stays auditable, and bulk regeneration would mask unrelated drift the
   reviewer should catch.
2. No backwards-compatibility shims or alias paths; this is artifact
   regeneration plus an evidence-doc note, not new production logic.
3. `engine-core` is untouched — deterministic evidence + game-local docs only; no
   mechanic noun enters the kernel and no `game-stdlib` change is made (§3/§4).

## Verification Layers

1. Only affected artifacts changed -> `git diff --stat` scoped to the named
   subset; unaffected traces/fixtures byte-identical (grep/diff proof).
2. Replay round-trips and deterministic hashes hold -> `replay-check --game
   starbridge_crossing --all`, `fixture-check --game starbridge_crossing`, and
   `cargo test -p starbridge_crossing` (`serialization.rs` golden round-trip) green.
3. `SC-MOVE-010` coverage holds post-migration -> `rule-coverage --game
   starbridge_crossing`.
4. No hidden-information leak introduced -> the visibility / viewer-parity traces
   are unchanged and `tests/visibility.rs` passes.
5. Web shell no longer offers the no-op -> `node
   apps/web/e2e/starbridge-crossing.smoke.mjs` green + the manual Puppeteer
   runbook (origin → A → origin no longer offered).
6. Performance within thresholds -> `cargo bench -p starbridge_crossing` (or its
   smoke filter) against `benches/thresholds.json`.

## What to Change

### 1. Determine the affected artifact set

Enumerate which golden traces and replay fixtures recorded an action tree (or
replayed through a state) that offered an origin-return continuation. Record the
resolved touch / don't-touch set so the no-blanket-regen scope is auditable.

### 2. Regenerate the affected artifacts (reviewed hashes)

Regenerate only the affected golden traces under `tests/golden_traces/` and the
affected `data/fixtures/*.fixture.json`, reviewing each changed hash to confirm
it reflects only the origin-return removal. Update `benches/thresholds.json` only
if jump-enumeration counts shifted.

### 3. Refresh `GAME-EVIDENCE.md`

Record the fix receipt and a per-artifact ADR-0009 authority note for each
regenerated artifact (what changed and why it was in scope).

### 4. Manual runbook — web no-op reproduction

Implementer checklist (not CI-runnable; the WASM shell has no browser-automation
harness beyond the smoke):
1. Build the web shell against the post-`-001` WASM.
2. Load Starbridge Crossing; select a peg and start a hop that returns toward its
   origin space.
3. Confirm the action tree no longer offers landing on the origin space (the
   `origin → A → origin` no-op is absent), and the shell settles to the latest
   viewer-safe public view.

## Files to Touch

- `crates/wasm-api/tests/snapshots/api_surface.tsv` (modify — affected
  Starbridge public action-tree snapshot discovered by workspace test)
- `games/starbridge_crossing/tests/golden_traces/` (modify only if affected
  traces surface; current verification may prove unchanged)
- `games/starbridge_crossing/data/fixtures/*.fixture.json` (modify only if
  affected fixtures surface; current verification may prove unchanged)
- `games/starbridge_crossing/benches/thresholds.json` (modify — only if
  enumeration counts shift)
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` (modify)

## Out of Scope

- The `legal_jump_landings` guard and its tests (`GAT201STACROHOP-001`); the
  `SC-MOVE-010` rule + coverage docs (`GAT201STACROHOP-002`).
- Regenerating any artifact whose pre-fix state never offered an origin-return
  node (no blanket golden/hash regeneration — `SC` Forbidden changes).
- Any new behaviour, variant, or seat/piece count; a `rules_version` cutover
  unless ADR 0009 is confirmed in-ticket to require one.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing` (full — `serialization.rs` / `bots.rs`
   golden round-trips green) and workspace `cargo test`.
2. `cargo run -p replay-check -- --game starbridge_crossing --all` and
   `cargo run -p fixture-check -- --game starbridge_crossing` pass with only the
   annotated affected-artifact diffs.
3. `cargo run -p rule-coverage -- --game starbridge_crossing` and
   `cargo run -p simulate -- --game starbridge_crossing --games 1000` pass.
4. `node apps/web/e2e/starbridge-crossing.smoke.mjs` is green; the manual
   Puppeteer reproduction confirms the `origin → A → origin` no-op is no longer
   offered.

### Invariants

1. Only artifacts whose pre-fix state offered an origin-return continuation are
   changed; every other trace/fixture is byte-identical.
2. Each regenerated artifact carries a per-artifact ADR-0009 authority note; no
   blanket regeneration.
3. Visibility / viewer-parity traces are unchanged — the legality narrowing
   leaks nothing.

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/golden_traces/*.trace.json` — regenerated
   affected subset, each hash individually reviewed (no bulk-accept).
2. `games/starbridge_crossing/data/fixtures/*.fixture.json` — regenerated
   affected subset (only replays traversing an origin-return-offering state).

### Commands

1. `cargo test -p starbridge_crossing && cargo test --workspace`
2. `cargo run -p replay-check -- --game starbridge_crossing --all && cargo run -p fixture-check -- --game starbridge_crossing && cargo run -p rule-coverage -- --game starbridge_crossing`
3. `node apps/web/e2e/starbridge-crossing.smoke.mjs` (CI-runnable) plus the
   §What-to-Change manual Puppeteer runbook (no browser-automation harness exists
   for the interactive reproduction, so it is an implementer checklist).

## Outcome

Completed: 2026-06-27

Resolved the governed evidence migration for the `SC-MOVE-010` legality
narrowing. The full Starbridge crate tests, replay checker, and fixture checker
initially passed without golden trace or setup-fixture diffs, proving the
committed trace/fixture corpus did not contain an origin-return continuation
that required Trace Schema v1 regeneration. The broad workspace test exposed the
actual stale deterministic artifact: the WASM API public action-tree snapshot
for `starbridge_crossing/action_tree/seat_0`, which still recorded the removed
origin-return continuations. Regenerated that single snapshot with
`UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface`, then verified
it with the focused test and the full workspace suite.

Updated `games/starbridge_crossing/docs/GAME-EVIDENCE.md` with a Gate 20.1
Origin-Return Migration Receipt naming the migrated WASM snapshot and the
unchanged golden trace, setup fixture, and benchmark-threshold surfaces. The
receipt records the ADR 0009 authority note: this is a public action-tree
contract migration only; no hidden-information, view-redaction, rule-version,
data-version, golden-trace, fixture, or benchmark-threshold migration was
needed.

Deviations: the ticket expected golden traces or replay fixtures to be the
changed artifacts. Current verification showed those surfaces were unchanged;
the required migration was instead the WASM API action-tree snapshot discovered
by `cargo test --workspace`. The browser smoke exercises Starbridge jump
integration through Puppeteer, but the exact synthetic origin-return no-op is
proved by `tests/rules.rs::hop_chain_cannot_return_to_origin_space` and the
Rust-owned action-tree/validator checks from `GAT201STACROHOP-001`, not by a
separate interactive browser fixture.

Verification:

- `cargo test -p starbridge_crossing` passed.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed; all
  traces accepted unchanged.
- `cargo run -p fixture-check -- --game starbridge_crossing` passed; all setup
  fixtures accepted unchanged.
- `cargo run -p rule-coverage -- --game starbridge_crossing` passed.
- `cargo run -p simulate -- --game starbridge_crossing --games 1000` passed
  (1000 games, 2,000,000 total actions, zero capped matches).
- `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface` passed and
  regenerated the single Starbridge API snapshot.
- `cargo test -p wasm-api --test api_surface` passed after regeneration.
- `cargo test --workspace` passed after regeneration.
- `cargo bench -p starbridge_crossing` passed all 14 benchmark operations.
- `npm --prefix apps/web run smoke:wasm` passed.
- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/starbridge-crossing.smoke.mjs` passed.
