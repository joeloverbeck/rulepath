# GAT101PLATRI-003: CONDITIONAL — seeded-shuffle helper extraction and high_card_duel/poker_lite back-port

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `crates/game-stdlib` (new behavior-free seeded shuffle/deal helper) and back-port of `games/high_card_duel/src/setup.rs` + `games/poker_lite/src/setup.rs`, with trace/hash-preservation evidence. **Only worked if GAT101PLATRI-002 decides *promote*.**
**Deps**: GAT101PLATRI-002

## Problem

If — and only if — the third-use primitive-pressure ledger (GAT101PLATRI-002) decides **promote**, FOUNDATIONS §4/§11 require that the promoted `game-stdlib` primitive be adopted by every matching prior official game in the same gate, or that named promotion debt be recorded with a closure gate. This ticket extracts the narrow, behavior-free seeded shuffle/deal-of-opaque-ids helper and back-ports `high_card_duel` and `poker_lite` to it with preserved traces/hashes. It is **not applicable** if GAT101PLATRI-002 decides reuse / defer-reject / ADR.

## Assumption Reassessment (2026-06-09)

1. Existing local shuffle/deal lives in `games/high_card_duel/src/setup.rs` and `games/poker_lite/src/setup.rs`; `crates/game-stdlib/src/lib.rs` is the promotion target. Whether these are migrated depends entirely on GAT101PLATRI-002's recorded decision.
2. Spec §5 item 2 binds this: "If the decision is **promote**, the same gate must either back-port `high_card_duel` and `poker_lite` to the helper with trace/hash preservation evidence, or record named promotion debt with a closure gate in atlas §10A — and the back-port/debt work becomes additional bounded AGENT-TASKs inside this gate." Spec appendix A6 scopes the only plausibly-promotable piece as a behavior-free seeded shuffle/deal of opaque ids; reveal timing, legality, and zone semantics stay local.
3. Shared boundary under audit: the `game-stdlib` public surface and the deterministic-RNG/shuffle contract shared by `high_card_duel`, `poker_lite`, and `plain_tricks`. The back-port must not alter deal order or RNG consumption for the existing games.
4. FOUNDATIONS §4 (promotion adopted by all matching games or explicit exception) and §11 (promoted primitives adopted, or non-adoption has an accepted exception) are the principles under audit.
5. Enforcement surface: deterministic replay/hash (§11/§13). The back-port MUST preserve every existing golden-trace hash for `high_card_duel` and `poker_lite`; any hash change is an accidental trace migration forbidden by spec §3 unless explicitly designed, documented, and tested. No hidden-information leak is introduced — the helper handles opaque ids only, never reveal policy.
6. Mismatch + correction: this ticket exists solely to keep promote-path coverage from being silently skipped. If GAT101PLATRI-002 decides otherwise, this ticket closes not-applicable and the shuffle stays local in GAT101PLATRI-005 (`games/plain_tricks/src/setup.rs`).

## Architecture Check

1. Extracting only the behavior-free seeded shuffle/deal-of-opaque-ids (not reveal timing or legality) keeps the promotion narrow and earned; in-gate back-port (vs. deferred debt) avoids leaving matching games un-migrated, which is a §12 stop condition.
2. No backwards-compatibility aliasing/shims — the back-ported games call the helper directly; the old local shuffle is removed, not aliased.
3. `engine-core` is untouched (the helper lands in `game-stdlib`, not the kernel); the promotion is earned via the GAT101PLATRI-002 atlas decision (FOUNDATIONS §4).

## Verification Layers

1. Trace/hash preservation for `high_card_duel` + `poker_lite` -> deterministic replay-hash check (`cargo run -p replay-check --game high_card_duel`; `--game poker_lite`).
2. Helper is behavior-free (opaque ids only; no reveal/legality) -> codebase grep-proof + manual review of `crates/game-stdlib` surface.
3. `engine-core` stays noun-free -> `bash scripts/boundary-check.sh`.
4. Promotion adopted by all matching games (no un-migrated sibling) -> FOUNDATIONS §4/§11 alignment check against the atlas.

## What to Change

### 1. `crates/game-stdlib` — narrow seeded shuffle/deal helper

Add a behavior-free helper that shuffles/deals opaque ids under the existing deterministic-RNG contract, with tests, docs, examples, and anti-examples (per spec §6 promote-branch exit criteria). No reveal, legality, or zone semantics.

### 2. Back-port `high_card_duel` and `poker_lite`

Replace the local shuffle/deal in `games/high_card_duel/src/setup.rs` and `games/poker_lite/src/setup.rs` with the helper, preserving deal order and RNG consumption so all existing golden-trace hashes are unchanged.

### 3. Atlas debt closure (if any)

If GAT101PLATRI-002 recorded §10A promotion debt instead of immediate back-port, this ticket closes it; otherwise confirm no §10A row remains open.

## Files to Touch

- `crates/game-stdlib/src/lib.rs` (modify)
- `games/high_card_duel/src/setup.rs` (modify)
- `games/poker_lite/src/setup.rs` (modify)
- `docs/MECHANIC-ATLAS.md` (modify — close §10A debt if opened by GAT101PLATRI-002)

## Out of Scope

- Proceeding at all if GAT101PLATRI-002 decided reuse / defer-reject / ADR — this ticket is then not-applicable and `games/plain_tricks/src/setup.rs` keeps the shuffle local (GAT101PLATRI-005).
- The `plain_tricks` setup itself (GAT101PLATRI-005); it consumes the helper only if promoted.
- Any change to reveal timing, legality coupling, or zone semantics of the back-ported games.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game high_card_duel` and `cargo run -p replay-check -- --game poker_lite` pass with unchanged hashes (no accidental trace migration).
2. `cargo test --workspace` passes.
3. `bash scripts/boundary-check.sh` confirms `engine-core` stays noun-free.

### Invariants

1. Every prior official game matching the promoted shape adopts the helper, or carries an explicit accepted exception (FOUNDATIONS §4/§11).
2. No existing golden-trace hash changes (deterministic replay preserved; §11/§13).

## Test Plan

### New/Modified Tests

1. `crates/game-stdlib` unit tests for the seeded shuffle/deal helper (determinism, opaque-id handling) — proves the helper is behavior-free and deterministic.
2. Existing `high_card_duel` / `poker_lite` golden traces re-run unchanged — proves back-port preserves hashes.

### Commands

1. `cargo run -p replay-check -- --game high_card_duel && cargo run -p replay-check -- --game poker_lite`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Full-workspace scope is correct here because the back-port touches two shipped games and a shared crate; narrower per-crate runs would miss cross-game hash regressions.
