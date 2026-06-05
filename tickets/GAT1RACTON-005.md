# GAT1RACTON-005: race_to_n rules — legal actions, validation, transitions, terminal, effects

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/race_to_n` gains flat legal action generation, validation (viewer-safe diagnostics + freshness-token rejection), state transitions, terminal/outcome detection, and semantic effect emission.
**Deps**: GAT1RACTON-004

## Problem

The behavior core of `race_to_n`: from a state, produce the flat legal **action
tree**; **validate** a submitted action path (emitting viewer-safe `Diagnostic`s
and rejecting stale submissions via the freshness token); **apply** the command
to produce a new state plus ordered **semantic effects**; and detect
**terminal/outcome**. This is where Rust holds behavior authority (FOUNDATIONS
§2) and where fail-closed validation (FOUNDATIONS §11) is enforced.

## Assumption Reassessment (2026-06-05)

1. After GAT1RACTON-004, `games/race_to_n` has typed state/ids/setup/variant and
   a partial `engine_core::Game` impl. The engine contracts this ticket uses —
   `ActionTree`/`ActionPath`/`FreshnessToken` (GAT1RACTON-002), `EffectEnvelope`/
   `Diagnostic` (`engine-core` existing) — are in place. `Diagnostic { code,
   message }` already exists (`crates/engine-core/src/lib.rs:54`).
2. The legal actions, validation rules, transitions, and win condition MUST match
   the pinned rule IDs in `games/race_to_n/docs/RULES.md` (GAT1RACTON-001); each
   rule test references its stable ID (OFFICIAL-GAME-CONTRACT §5; TESTING §2).
3. Cross-crate boundary under audit: this ticket consumes the action-tree /
   freshness-token contracts and the effect-log/effect-envelope contract; it must
   route all legality through the action-tree API (no TypeScript legality ever —
   FOUNDATIONS §2/§7). The freshness token is validated here against the version
   the action tree was produced under.
4. FOUNDATIONS §2 (behavior authority is Rust) and §11 (validation is
   deterministic, fail-closed, blocking; warnings distinct from blockers)
   motivate this ticket. The action generator must never produce an action that
   leads to an invalid state (TESTING §6).
5. Fail-closed enforcement surface: validation here is the §11 enforcement
   surface for invalid/stale action paths. Confirm: invalid paths return a
   viewer-safe `Diagnostic` (not a panic, not silent acceptance); a stale
   freshness token is rejected with a diagnostic; validation is blocking
   (rejected commands do not mutate state). No hidden information leaks through
   diagnostics — `race_to_n` is perfect-information, so all diagnostics are
   public-safe by construction (recorded as the no-leak rationale).
6. Schema/contract: effects emitted here are game-specific payloads behind the
   generic `EffectEnvelope<T>` (ARCHITECTURE §7) — additive use of the existing
   contract, no kernel change. Effects must be deterministic + ordered (TESTING
   §6; ARCHITECTURE §7) so GAT1RACTON-008's effect hashes are stable.

## Architecture Check

1. Flat legal-action generation is correct for `race_to_n` (simple games MAY
   expose flat action trees, ARCHITECTURE §5); progressive construction would be
   over-engineering. Emitting effects from `apply` (not from the renderer) keeps
   animation downstream of semantic facts (FOUNDATIONS §7; ARCHITECTURE §7).
2. No backwards-compatibility shims.
3. `engine-core` untouched — all mechanic logic lives in `games/race_to_n`
   (FOUNDATIONS §3). `game-stdlib` untouched (first use local-only, §4).

## Verification Layers

1. Behavior authority in Rust -> codebase grep-proof (legality lives in
   `games/race_to_n/src/{actions,rules}.rs`; no legality in any TS — confirmed by
   GAT1RACTON-012 too).
2. Fail-closed validation -> named rule tests (invalid path → diagnostic, not
   panic; stale token → rejection; blocked command does not mutate state).
3. Action-generation safety -> property/invariant test (legal action gen never
   panics; applying any legal action never yields an invalid state — TESTING §6).
4. Effect determinism -> deterministic replay-hash check (same command sequence →
   identical ordered effects; surfaced fully in GAT1RACTON-008).

## What to Change

### 1. Legal action generation (`src/actions.rs`)

Produce the flat `ActionTree` of legal moves for the current actor from state,
with stable path segments and labels/accessibility text.

### 2. Validation (`src/rules.rs` or `src/actions.rs`)

Validate a submitted `ActionPath` + freshness token: reject illegal paths with a
viewer-safe `Diagnostic`; reject stale submissions (token mismatch) with a
diagnostic; produce a `CommandEnvelope` on success. Deterministic, blocking,
warnings-vs-blockers distinguished.

### 3. Transitions + terminal/outcome (`src/rules.rs`)

Apply a validated command → new state + ordered semantic effects. Detect terminal
state and outcome (win/loss per the pinned variant).

### 4. Semantic effects (`src/effects.rs`)

Define game-specific effect payloads behind `EffectEnvelope`, covering
action start/completion, counter change, turn change, and game end (ARCHITECTURE
§7). Deterministic + ordered.

## Files to Touch

- `games/race_to_n/src/actions.rs` (new)
- `games/race_to_n/src/rules.rs` (new)
- `games/race_to_n/src/effects.rs` (new)
- `games/race_to_n/src/lib.rs` (modify) — complete the apply/validate/legal-actions parts of the `Game` impl
- `games/race_to_n/tests/rule_tests.rs` (new)
- `games/race_to_n/tests/property_tests.rs` (new)

## Out of Scope

- Public-view projection + serialization round-trips (GAT1RACTON-006).
- Replay/hash/golden traces (GAT1RACTON-008).
- Bot (GAT1RACTON-007), wasm/UI (011/012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n` — named rule tests (referencing `RULES.md` IDs), validation tests (invalid + stale paths → diagnostics), and property tests pass.
2. Property test: legal action generation never panics and never yields an invalid state across many states.
3. `cargo clippy -p race_to_n --all-targets -- -D warnings` — clean.

### Invariants

1. All legality/validation/transition logic is in Rust; nothing decides legality outside Rust (FOUNDATIONS §2).
2. Validation is fail-closed and blocking; a rejected command never mutates state (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/rule_tests.rs` — one named test per `RULES.md` rule ID (legal moves, win condition, stale/invalid rejection).
2. `games/race_to_n/tests/property_tests.rs` — invariants: no-panic legal-action gen, no invalid state from legal apply, conservation/turn-order.

### Commands

1. `cargo test -p race_to_n`
2. `cargo test --workspace`
3. `grep -rniE 'legal|valid' apps/web/src` — expect no legality logic in TS (boundary spot-check; full UI proof in GAT1RACTON-012).
