# GAT17VOWTIDOHHEL-007: Bidding actions, exact dealer hook, legal tree, validation, first-use ledger

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/vow_tide/src/{actions,rules,effects}.rs` (bidding paths); modifies `state.rs`; new `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md`; modifies `docs/MECHANIC-ATLAS.md` (bidding first-use row)
**Deps**: 005, 006

## Problem

Implement sequential public bidding: each seat bids `0..=H` once, starting left of dealer, dealer last, with the exact `H-S` hook excluded only when in range. Bidding is numeric trick-contract — the first official use — so it stays typed game-local Rust and earns a new `local-only` primitive-pressure ledger entry, with no shared bidding framework and no behavior in data.

## Assumption Reassessment (2026-06-21)

1. `games/vow_tide/src/state.rs` (005/006) owns phase/active-seat/hand-size; this ticket adds the `bid/<u8>` action family, the seat-keyed `Option<u8>` public bids, and the hook logic. No prior official game implements a numeric trick contract (spec §2.4 confirms `secret_draft`/`masked_claims`/`high_card_duel` are commitment/claim, not numeric bid) — first use.
2. Spec §3.1 + Appendix B + `VT-BID-ORDER/RANGE/HOOK/PUBLIC-001` fix the legal-tree, the `H-S` exclusion (only when in `[0,H]`), immutability, and the `VT_BID_*` diagnostics. The legality order is §3.1's ordered checks.
3. Cross-artifact boundary: the bidding action tree + `VT_BID_*` diagnostics + bid effects are the contract consumed by views (010), bots (012), WASM ops (017), and UI (018); legal-tree↔validator equivalence is the invariant under audit.
4. FOUNDATIONS §5 (no behavior in data) and §2 (Rust owns legality) are under audit: the hook/range/order live in typed Rust; data carries identity/presentation only; TypeScript never sums bids or removes the hook value.
5. §11 fail-closed validation enforcement surface: the validator independently re-checks range/hook/seat/phase (never trusts leaf membership); every state retains ≥1 legal dealer bid (the `H-S` exclusion never empties the set since out-of-range `H-S` removes nothing). No hidden information enters bid metadata (cards/stock excluded).

## Architecture Check

1. Generating `0..=H` leaves with the single hook value omitted (rather than emitting it and rejecting on submit) keeps the normal UI legal-only and the validator authoritative — the §7/§11 contract.
2. No shims; new action/rules/effects modules.
3. `engine-core` untouched; bidding stays local (`game-stdlib` not earned — one use); the first-use ledger records why.

## Verification Layers

1. Bid order/range/immutability → `cargo test -p vow_tide --test rules` + bid-order golden traces.
2. Hook excludes exactly `H-S` when in range, never empties the legal set → exhaustive property over `H` and reachable prefix sums.
3. Legal-tree ↔ validator equivalence → property test (every leaf validates; every valid bid is a leaf; no stale/wrong-seat/hook-forbidden validates).
4. No behavior in data → `cargo run -p fixture-check -- --game vow_tide` (once registered, 015) + manual data review.
5. First-use ledger recorded → grep `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md` + `docs/MECHANIC-ATLAS.md` bidding row.

## What to Change

### 1. Bid action family + legal tree

`actions.rs`: `bid/<decimal_u8>` paths; Rust-generated ascending leaves `0..=H`, with the hook-forbidden value omitted for the dealer when `H-S ∈ [0,H]`. Public metadata (`hand_size`, `current_bid_total`, `is_dealer`, `hook_forbidden_bid`); never cards/stock.

### 2. Validation + diagnostics + effects

`rules.rs`: the §3.1 ordered checks and `VT_BID_OUT_OF_RANGE`/`VT_BID_HOOK_FORBIDDEN`/`VT_BID_ALREADY_SET`/`VT_WRONG_SEAT`/`VT_WRONG_PHASE`/`VT_STALE_COMMAND`. `effects.rs`: public bid-accepted + hook-constrained effects. Accepted bids immutable.

### 3. First-use ledger

Author `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md` bidding `local-only` entry (spec §8.3) and add the `numeric trick bid / contract-vs-result / last-bidder hook` first-use row to `docs/MECHANIC-ATLAS.md` §10.

## Files to Touch

- `games/vow_tide/src/actions.rs` (new)
- `games/vow_tide/src/rules.rs` (new)
- `games/vow_tide/src/effects.rs` (new)
- `games/vow_tide/src/state.rs` (modify)
- `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Trick play (008), scoring (009).
- Any shared bidding helper/framework or `engine-core` bid noun; any bid-change/secret-bid action.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test rules` — bid order/range/hook/immutability.
2. `cargo test -p vow_tide --test property` — exhaustive hook + legal-tree/validator equivalence.
3. `cargo clippy -p vow_tide --all-targets -- -D warnings`.

### Invariants

1. Total bids can never be forced equal to `H` via the dealer; every reachable state has ≥1 legal dealer bid.
2. No bid legality/hook/order/range appears in static data or TypeScript.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/rules.rs` — valid/invalid bids, wrong-seat/phase, immutability.
2. `games/vow_tide/tests/golden_traces/{bidding-left-of-dealer-through-dealer,dealer-hook-forbidden-total,dealer-hook-out-of-range-no-removal,bid-zero-accepted,bid-upper-bound-accepted}.trace.json`.

### Commands

1. `cargo test -p vow_tide --test rules --test property`
2. `cargo test -p vow_tide`
3. Narrower command rationale: rules+property suites are the legality boundary; data-behavior rejection is proven by fixture-check (015).
