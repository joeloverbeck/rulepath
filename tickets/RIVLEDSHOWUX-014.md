# RIVLEDSHOWUX-014: `BotDecisionPublicExplanation` Rust payload

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/bots.rs`, `games/river_ledger/src/visibility.rs`, `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`, `games/river_ledger/tests/bots.rs`
**Deps**: RIVLEDSHOWUX-008

## Problem

There is no viewer-safe in-play "why did the bot do that?" explanation. Add a Rust-authored `BotDecisionPublicExplanation` (seat, seat label, action label, one-sentence public reason, public facts, hidden-information notice) for non-random bots, built from the bot's authorized view and public state only. **Optional per spec Assumption A5** — droppable with no dependents (only RIVLEDSHOWUX-015 consumes it).

## Assumption Reassessment (2026-06-16)

1. Verified: `games/river_ledger/src/bots.rs` chooses through the legal action API (no direct state mutation, no hidden state); there is no bot-explanation surface today.
2. Verified against spec §6 D9 + §8 WB14 (#13) + §14 A5 (optional); `RULES.md` `RL-BOT-EXPLAIN-001`; FOUNDATIONS §8 (competent, explainable, fair, no hidden state).
3. Shared boundary under audit: the bot-decision projection through `visibility.rs` → `crates/wasm-api` — the explanation is additive and viewer-safe.
4. FOUNDATIONS §8 (public bots produce concise viewer-safe explanations; no MCTS/ML/RL — this ticket adds no bot-AI class, only an explanation of the existing deterministic policy) motivates this ticket.
5. No-leak firewall: the explanation uses only the bot's authorized view + public facts; it exposes no candidate rankings, hidden hand strength, opponent private cards, future board, deck tail, or solver claims, and is not written to the effect log (§11 no-leak; §12).
6. Schema extension: the bot/view projection gains an additive `BotDecisionPublicExplanation` (`seat`, `seat_label`, `action_label`, `short_reason`, `public_facts[]{label,value}`, `hidden_information_notice`); consumer is RIVLEDSHOWUX-015; additive-only; random bots emit none.

## Architecture Check

1. A Rust-authored explanation of the existing deterministic policy (vs a TS-synthesized rationale) keeps bot reasoning Rust-owned and viewer-safe; public-facts-only scoping makes the no-leak contract structural.
2. No shims; the explanation is new additive output, not an alias.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); no new bot-search class (§8/§13).

## Verification Layers

1. Non-random bots emit a one-sentence public reason + public facts; random bots emit none -> `games/river_ledger/tests/bots.rs`.
2. No candidate rankings / hidden strength / opponent cards / future board / deck tail in the explanation, and none in the effect log -> `games/river_ledger/tests/{bots,visibility}.rs` no-leak assertions.
3. Bot still chooses via the legal action API only -> `games/river_ledger/tests/bots.rs` (bot legality).

## What to Change

### 1. `games/river_ledger/src/bots.rs`

For non-random bots, author `BotDecisionPublicExplanation` from the authorized view + public state: action label, one-sentence reason, public facts (street, call price, raises left, live seats), hidden-information notice.

### 2. `games/river_ledger/src/visibility.rs` + `crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`

Project the explanation viewer-safe; expose it through the bridge; add the TS type.

## Files to Touch

- `games/river_ledger/src/bots.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `games/river_ledger/tests/bots.rs` (modify)

## Out of Scope

- The "Why?" disclosure UI (RIVLEDSHOWUX-015).
- Proceeding at all if the bot "Why?" affordance is dropped per spec A5 (this ticket and RIVLEDSHOWUX-015 are then not-applicable; no other ticket depends on them).
- Any owner-private (hole-card-bucket) explanation — public facts only this pass.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — non-random bots emit a viewer-safe explanation; random bots emit none.
2. `cargo run -p replay-check -- --game river_ledger --all` — explanation leaks no private/hidden state into payloads or replay exports.
3. `npm --prefix apps/web run build` + `npm --prefix apps/web run smoke:wasm` — bridge JSON ↔ TS type parity.

### Invariants

1. The explanation uses only the bot's authorized view + public facts; no candidate rankings / hidden state / opponent cards (§8, §11, `RL-BOT-EXPLAIN-001`).
2. No new bot-AI class; the explanation describes the existing deterministic policy (§8, §13).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/bots.rs` — explanation present for non-random bots, absent for random; no-leak content assertion.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. `npm --prefix apps/web run smoke:wasm`
