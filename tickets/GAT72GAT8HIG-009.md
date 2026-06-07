# GAT72GAT8HIG-009: Replay support — internal full trace + public/viewer-scoped export/import split

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/high_card_duel/src/replay_support.rs` (+ a hidden-info-safe trace classification if the existing trace tooling cannot represent both modes)
**Deps**: GAT72GAT8HIG-007, GAT72GAT8HIG-008, GAT72GAT8HIG-022

## Problem

Gate 8 must separate internal replay truth (seed + full command stream with
private choices, for native tests/golden traces) from public/viewer-scoped
browser exports that are no-leak by default. A public observer export must
replay a public projection timeline and must not let an unauthorized viewer
reconstruct unrevealed deck order, private hands, or hidden commitments.

## Assumption Reassessment (2026-06-07)

1. Verified the replay primitives: `crates/engine-core/src/replay.rs` plus
   sibling `games/draughts_lite::replay_support::replay_commands` (re-exported in
   `crates/wasm-api/src/lib.rs:25`); `tools/replay-check` validates golden traces
   against a per-game `replay_support`.
2. Verified against the spec: §4.2.4 fixes the four modes (internal full,
   public observer export, optional seat-private export, terminal behavior) and
   §8.5 requires splitting the "public export = seed + command stream"
   assumption for hidden-info games.
3. Cross-artifact boundary under audit: the replay/trace schema
   (`docs/TRACE-SCHEMA-v1.md`) and the replay-export taxonomy. The internal full
   trace is the canonical deterministic-replay/hash artifact; the public export
   is a new, derived, redacted form.
4. FOUNDATIONS principle under audit (§11 determinism + no-leak): internal replay
   stays byte-identical (`docs/TESTING-REPLAY-BENCHMARKING.md`); public export
   must carry no material that reconstructs hidden cards.
5. Enforcement surface named: deterministic replay/hash (§11) AND the no-leak
   firewall on replay exports (§11/§12). Confirm the public export omits seed
   material that would reconstruct hidden cards, raw action paths containing
   private card ids, and the unrevealed deck tail (including at terminal — no
   auto-reveal). The internal full trace is fenced to test/dev surfaces only.
6. Schema extension classification: this adds a hidden-info-safe trace
   classification distinct from the internal full trace. Its consumers are
   `tools/replay-check` (013), the WASM export/import surface (016), the no-leak
   suite (011), and golden traces (012). Whether this is an additive trace-class
   or a taxonomy change is the §13 question resolved by **GAT72GAT8HIG-022**
   (ADR) — this ticket implements the decision recorded there.

## Architecture Check

1. Modeling two explicit replay modes (internal full vs public projection) is
   cleaner and safer than reusing one command-stream export and trusting the
   client to redact — the firewall stays in Rust.
2. No backwards-compatibility shims — existing public-perfect-information replay
   for prior games is left intact; hidden-info games add a parallel mode rather
   than mutating the shared one.
3. `engine-core` replay contract reused; no mechanic noun in the kernel; no
   `game-stdlib` change.

## Verification Layers

1. Internal replay determinism -> deterministic replay-hash check: replaying the internal full trace reproduces the same revealed sequence/hash.
2. Public export no-leak -> no-leak visibility test: the public observer export contains no unrevealed deck order / private hands / pre-reveal commitments / reconstructing seed material.
3. Import semantics -> golden trace check (012): importing a public export replays a public projection timeline, not omniscient hidden state.
4. Terminal hides tail -> no-leak test: terminal public export does not auto-reveal the unused deck tail.

## What to Change

### 1. `replay_support.rs`

- Internal full trace: seed + full command stream (private choices) for native
  tests / golden traces / `replay-check`.
- Public observer export: public projections/effects + redacted command
  summaries; omit deck order, private hands, pre-reveal commitments, hidden-card-
  reconstructing seed material, bot private candidates, raw private action paths.
- Optional seat-private export (only if safe): that seat's authorized
  observations, labelled seat-scoped.
- Terminal: no auto-reveal of the unused tail.
- If trace tooling cannot represent both modes, add a minimal hidden-info-safe
  trace classification (per spec §4.2.9) rather than forcing the public export to
  use the full internal command stream.

## Files to Touch

- `games/high_card_duel/src/replay_support.rs` (modify — fill stub)

## Out of Scope

- WASM `export_replay`/`import_replay` wiring (GAT72GAT8HIG-016).
- Authoring the ADR (GAT72GAT8HIG-022) — this ticket implements its decision.
- `tools/replay-check` registration (013) and golden traces (012).

## Acceptance Criteria

### Tests That Must Pass

1. `replaying_internal_full_trace_reproduces_revealed_sequence`.
2. `public_replay_export_has_no_unrevealed_internal_card_identities`.
3. `import_public_export_produces_public_timeline_without_hidden_reconstruction`.

### Invariants

1. Internal replay is byte-identical/deterministic (§11); public export is no-leak by default (§11/§12).
2. Terminal public export does not auto-reveal hidden state.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/replay.rs` — internal determinism + public-export no-leak + import-semantics cases.

### Commands

1. `cargo test -p high_card_duel --test replay`
2. `cargo test -p high_card_duel`
3. The replay test is the correct boundary; `tools/replay-check` over golden traces (012/013) is the full-pipeline confirmation.
