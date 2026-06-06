# GAT3WASMSTAWEB-003: Replay export/import/step WASM ops on the Gate 2 trace schema

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api` gains replay export/import/step ops and command-stream capture in the match store; reuses `engine-core` `CommandEnvelope`/`EffectLog` and `games/race_to_n` `replay_support`. No `engine-core`/`game-stdlib` change; no mechanic noun enters the kernel.
**Deps**: 001, 002

## Problem

Gate 3's required replay viewer and safe local import/export modes (spec §8.4,
§8.5, §15) need Rust-authoritative replay operations: export a replay document,
import/validate one, and step/reset a replay cursor — all projected through Rust,
never re-simulated in TypeScript (§15.1, FOUNDATIONS §2). The current
`crates/wasm-api` match store records `state` + `effects` but **not** the command
stream or the seed, so a reconstructable replay cannot be exported today. Spec §15
and the canonical Work breakdown (WB3) require anchoring the replay format on the
existing Gate 2 trace/replay infrastructure rather than inventing a parallel format.

## Assumption Reassessment (2026-06-06)

1. `crates/wasm-api/src/lib.rs` `MatchRecord` (lines 28–33) stores `game_id`,
   `state`, `effects: EffectLog<RaceEffect>` — **no command log, no seed**.
   `new_match` (line 44) takes `seed` but does not persist it; `apply_action`
   (line 85) and `run_bot_turn` (line 115) build a `CommandEnvelope` and discard
   it after `validate_command` + `apply_action`. Replay export therefore requires
   adding seed + an applied-command log to `MatchRecord`.
2. Spec §15.2 requires export to carry game id + version + seed/setup + command/
   effect data and exclude unauthorized data; §15.3 requires import to validate
   through Rust, reject wrong-game/unsupported-version/malformed/oversized inputs,
   and never auto-store as authoritative; §15.4 requires step/reset of a cursor.
   `docs/TRACE-SCHEMA-v1.md` is the canonical replay/trace schema this format
   anchors on (root fields incl. `schema_version`, `game_id`, `rules_version`,
   `seed`, `seats`, `commands`, hash fields).
3. Cross-artifact boundary under audit: the replay-document format ↔ the Gate 2
   trace schema (`docs/TRACE-SCHEMA-v1.md`) and the existing validators
   (`tools/replay-check/src/main.rs`). `games/race_to_n/src/replay_support.rs`
   exposes replay reconstruction/hash helpers — **confirm its exact public
   signatures at implementation start** and reuse them rather than re-deriving
   reconstruction in `wasm-api`. The replay doc MUST stay consistent with what
   `replay-check` already validates so a single replay authority holds.
4. FOUNDATIONS §11 / §2 (deterministic replay; behavior authority) motivate the
   design: identical inputs+versions reproduce identical state/effects, and Rust —
   not TS — replays. Restated before trusting the spec: reconstruction MUST be the
   recorded command stream re-applied through `validate_command`, so replay is
   deterministic without re-running bot RNG.
5. §11 enforcement substrate (deterministic replay/hash + no-leak export firewall):
   the export's `commands` are resolved action paths already applied (public facts
   for `race_to_n`); for future hidden-info games the format records command
   stream + seed, not internal private state, so no leakage path is introduced
   (§15.2). Determinism is preserved by recording resolved action paths (not bot
   seeds to re-roll). Hash fields follow `replay_support`'s existing hashing so the
   exported doc validates under the same Rust path Gate 2 hardened. The
   fail-closed validator surface is the import op itself (this ticket) plus
   `replay-check` (existing); no leakage/nondeterminism is deferred unresolved.
6. Schema/contract extension: adds (a) seed + applied-command log to `MatchRecord`
   (internal), and (b) a replay-document JSON shape on the wasm-api surface,
   anchored on `TRACE-SCHEMA-v1`. Consumers: the TS client + replay viewer UI
   (GAT3WASMSTAWEB-009), the WASM/API smoke (-012), and `replay-check`
   (compatibility). Additive-only to the wasm-api op set; the command-log field on
   `MatchRecord` is internal and breaks no external consumer.

## Architecture Check

1. Recording the applied command stream (resolved action paths) + seed and
   reconstructing through `setup_match` → `validate_command` → `apply_action`
   reuses the exact production rule path, so replay cannot diverge from live play —
   strictly better than a snapshot dump or a TS-side re-simulation. Anchoring on
   `TRACE-SCHEMA-v1` avoids a second replay authority.
2. No backwards-compatibility shims: there is no prior replay op to alias; the new
   doc format is the only replay surface introduced.
3. `engine-core` stays free of mechanic nouns — replay ops live in `wasm-api` and
   call `games/race_to_n`; the kernel's generic `CommandEnvelope`/`EffectLog`/replay
   contracts are reused, not extended with game vocabulary. `game-stdlib` untouched.

## Verification Layers

1. Export/import round-trip reproduces state → golden trace / deterministic
   replay-hash check: a Rust test plays a Race-to-N match, exports, imports, and
   asserts the reconstructed state/effect hashes equal the originals (per
   `docs/TESTING-REPLAY-BENCHMARKING.md`).
2. Import is fail-closed → schema/serialization validation: Rust tests assert
   wrong-game, unsupported-version, malformed, and oversized inputs return typed
   diagnostics (status 1) and never mutate/auto-store state (spec §15.3).
3. Replay is Rust-authoritative & deterministic → deterministic replay check:
   step/reset to cursor N yields the same projected view/effects as live play to
   turn N (FOUNDATIONS §2/§11).
4. Format anchored on Gate 2 schema → codebase grep-proof + compatibility: the
   exported doc's field names match `docs/TRACE-SCHEMA-v1.md`; `replay-check`
   accepts an exported doc (or the divergence is documented).
5. No-leak export → no-leak visibility test: exported doc contains command stream +
   seed + version metadata only, no internal/private state fields (spec §15.2).

## What to Change

### 1. `crates/wasm-api/src/lib.rs` — persist seed + command log

Extend `MatchRecord` with `seed: u64` and an applied-command log (e.g.
`Vec<AppliedCommand>` capturing actor seat + resolved `action_path` +
`rules_version`). Populate `seed` in `new_match`; push the resolved command in both
`apply_action` and `run_bot_turn` after successful validation/apply.

### 2. `crates/wasm-api/src/lib.rs` — `export_replay`

Add `pub fn export_replay(match_id: &str) -> Result<String, String>` emitting a
replay document anchored on `TRACE-SCHEMA-v1` (`schema_version`, `game_id`,
`rules_version`, `seed`, `seats`, `commands`, and the hash fields from
`replay_support`). Add `#[no_mangle] pub extern "C" fn rulepath_export_replay(...)`.

### 3. `crates/wasm-api/src/lib.rs` — `import_replay`

Add `pub fn import_replay(doc: &str) -> Result<String, String>`: parse with
unknown-field rejection, validate `game_id`/`rules_version`/`schema_version`,
enforce a size bound, reconstruct via `setup_match` + replay of `commands` through
`validate_command`, store as a new match, and return a viewer-safe handle/summary.
Reject invalid inputs with typed diagnostics. Add the `#[no_mangle]` wrapper.

### 4. `crates/wasm-api/src/lib.rs` — `replay_step` / `replay_reset`

Add ops returning the Rust-projected public view + effects for a replay cursor
(reconstruct-to-cursor-N for `race_to_n`; reset = cursor 0). Add `#[no_mangle]`
wrappers. Add the matching typed methods to `apps/web/src/wasm/client.ts`.

### 5. Rust tests

Round-trip, fail-closed import, deterministic step/reset, and no-leak export tests.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify) — seed+command capture, replay export/import/step/reset ops, externs, tests
- `apps/web/src/wasm/client.ts` (modify) — typed client wrappers for the replay ops (created by GAT3WASMSTAWEB-001; see Deps)

## Out of Scope

- Replay viewer / import-export React UI — GAT3WASMSTAWEB-009.
- Backward stepping via per-step snapshots (reset+replay-through-Rust is sufficient for Race-to-N per §15.4).
- Browser/E2E smoke of the replay flow — GAT3WASMSTAWEB-013; node WASM/API smoke — GAT3WASMSTAWEB-012.
- Documenting the replay format/version/limits in repo docs — GAT3WASMSTAWEB-015.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — replay round-trip, fail-closed import, step/reset determinism, and no-leak export tests pass.
2. `cargo build -p wasm-api --target wasm32-unknown-unknown --release` — replay externs compile to WASM.
3. `cargo test -p race_to_n` and `cargo run -p replay-check -- <exported-fixture>` (or the existing replay-check invocation) — the exported document remains valid under the Gate 2 replay authority.

### Invariants

1. Replay is reconstructed by Rust from the recorded command stream + seed; TypeScript never replays by mutating state.
2. Import is fail-closed (rejects wrong-game/version/malformed/oversized) and never auto-stores imported data as authoritative; the export leaks no internal/private state.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`#[cfg(test)]`) — `replay_round_trip_reproduces_hashes`, `import_rejects_wrong_game_and_version`, `replay_step_matches_live_play` rationale: pin determinism, fail-closed import, and Rust-authoritative stepping.
2. A reusable exported-replay fixture for `replay-check` compatibility — rationale: prove single replay authority with the Gate 2 tool.

### Commands

1. `cargo test -p wasm-api`
2. `cargo build -p wasm-api --target wasm32-unknown-unknown --release`
3. `cargo run -p replay-check -- <path-to-exported-replay>` — confirms Gate 2 / Gate 3 replay-format agreement (narrower than full pipeline because cross-tool agreement is the specific risk here).
