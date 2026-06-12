# CARACTPRES-002: Event Frontier Rust player-facing copy hygiene

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/event_frontier` (player-facing string resolution in rules/effects copy); no schema-shape change beyond regenerated trace text
**Deps**: CARACTPRES-001

## Problem

Rust-supplied player-facing text leaks raw internal identifiers into normal-mode UI: the live app renders "Freeholders is ineligible: event_choice." in the eligibility status and effect log. The raw reason token originates at `games/event_frontier/src/rules.rs:184,218` (`mark_ineligible(state, …, "event_choice", effects)`) and flows through effect text generation in `src/effects.rs` (`public_effect_text`). Spec WB2 (D5 copy hygiene, Rust side): player-facing strings resolve to authored labels; machine-readable codes stay internal.

## Assumption Reassessment (2026-06-12)

1. The `"event_choice"` literal exists at `games/event_frontier/src/rules.rs:184,218` and player-facing effect prose is produced by `public_effect_text()` in `games/event_frontier/src/effects.rs` (line 163 region) — verified by grep this session. The spec's WB2 row names only `effects.rs`; decomposition expands Files to Touch to `rules.rs` + `effects.rs` per the validated origin (mechanical propagation noted at Step 2 — no intent change).
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D5 and §9 exit criterion 2 require no raw snake_case identifiers in normal-mode player-facing copy; `docs/UI-INTERACTION.md` §2 forbids raw-JSON/debug-flavored public UX.
3. Cross-artifact boundary under audit: effect-envelope payload text consumed by `apps/web/src/components/effectFeedback.ts` and recorded in golden traces. Changing the human-facing string content is data-content change, not schema-shape change; the TS consumer keys on `effect.payload.type` (verified: `effectFeedback.ts` keyed by payload type), so no TS code change is required.
4. FOUNDATIONS §7/§11 motivate this ticket (public UI play-first, not debug-dominated; effect logs are player-facing surfaces). Restated: debug vocabulary in normal-mode surfaces is the defect class this spec exists to remove — the fix must not weaken the machine-readable reason codes used internally for state/tests.
5. Deterministic replay surface (§11): effect text lives in golden traces, so changed strings require trace regeneration through the ordinary migration path. No hash-semantics change, no reveal-timing change, no new information in any payload (the resolved label describes the same fact the code did).

## Architecture Check

1. Resolving codes to authored copy at the effect-text boundary (keeping `"event_choice"` as the internal reason code) is cleaner than renaming the code itself: state, tests, and any future diagnostics keep a stable machine token while every player-facing surface gets prose — the same code/label separation CARACTPRES-001 establishes for cards.
2. No backwards-compatibility aliasing/shims: no dual emission of old + new strings; traces regenerate once.
3. `engine-core` untouched; no `game-stdlib` change; all edits stay in `games/event_frontier` (§3/§4).

## Verification Layers

1. No raw snake_case tokens in player-facing effect/eligibility prose -> unit test scanning `public_effect_text` outputs for `[a-z]+_[a-z]+` tokens across all effect kinds.
2. Internal reason codes unchanged (no behavior drift) -> codebase grep-proof that `mark_ineligible` call sites still pass stable codes + existing rules tests stay green unmodified.
3. Deterministic replay after string changes -> regenerated golden traces + `replay-check --game event_frontier --all`.
4. Single-layer scope note: not applicable — three invariants above each map to a distinct surface.

## What to Change

### 1. Reason-code → label resolution

Add a small resolution function (in `src/effects.rs`, or `src/ui.rs` from CARACTPRES-001 if it already owns authored copy) mapping internal eligibility/reason codes to authored player prose (e.g. `event_choice` → "already chose an event this cycle" — exact prose authored at implementation, original and viewer-safe).

### 2. Apply at every player-facing emission site

Route `public_effect_text()` and any eligibility-status text through the resolver; sweep `src/rules.rs` / `src/effects.rs` for other raw-token emissions into player-facing strings (faction/site/card tokens already label-resolved stay as-is).

### 3. Regenerate traces

Regenerate golden traces whose recorded effect text changed; update any string-asserting tests to the new prose.

## Files to Touch

- `games/event_frontier/src/effects.rs` (modify)
- `games/event_frontier/src/rules.rs` (modify)
- `games/event_frontier/src/ui.rs` (modify — if the resolver lands beside the authored copy from 001)
- `games/event_frontier/tests/golden_traces/` (modify — regenerated trace JSON as surfaced; parent verified)

## Out of Scope

- TS-side headings/status copy ("Rust legal choices", "Rust projection") — CARACTPRES-005/007/009.
- Renaming internal reason codes, effect payload `type` values, or any machine-readable schema token.
- New effect kinds, eligibility rules, or behavior changes of any sort.
- The CI copy guard — CARACTPRES-009.

## Acceptance Criteria

### Tests That Must Pass

1. New unit test: no `snake_case` raw tokens in any player-facing effect/eligibility string across all effect kinds and reason codes.
2. Existing rules/effects tests green (unmodified except authored-prose assertions).
3. `cargo test -p event_frontier && cargo run -p replay-check -- --game event_frontier --all` green.

### Invariants

1. Machine-readable codes remain stable internal vocabulary; player-facing prose is authored, original, and viewer-safe (FOUNDATIONS §7/§10/§11).
2. Effect text changes carry no new information content — same facts, readable words (no-leak firewall unchanged).

## Test Plan

### New/Modified Tests

1. `games/event_frontier/src/effects.rs` (inline `#[cfg(test)]`) — resolver coverage per reason code + no-raw-token scan over emitted prose.
2. `games/event_frontier/tests/golden_traces/` — regenerated traces (text-only deltas).

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all && cargo run -p fixture-check -- --game event_frontier`
3. Narrow boundary rationale: only this crate's strings change; workspace-wide tests run at CARACTPRES-010 closeout evidence.
