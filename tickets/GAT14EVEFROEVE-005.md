# GAT14EVEFROEVE-005: Eligibility/initiative card flow

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/src/{actions,rules,effects}.rs` (card reveal, first/second-eligible determination, constrained menus, eligibility transitions)
**Deps**: GAT14EVEFROEVE-004

## Problem

The gate's turn engine is a fully deterministic, sequential, timeout-free initiative system with no simultaneity and no reaction windows. Each card names a first-eligible faction; the first eligible's choice (event / operation / pass) constrains the second's menu; acting costs eligibility on the next card; passing preserves it and pays +1. This ticket implements card reveal (current public, next public), first/second-eligible determination from card data + eligibility state, the typed eligibility constraint table, pass income, eligibility transitions, and the no-eligible-faction discard — with the waiting faction's tree empty and safe so the flow can never stall.

## Assumption Reassessment (2026-06-12)

1. The state and card inventory this flow drives exist: verified ticket 004 authored deck state (`current`/`next_public`/`undrawn`/`discard`/`epoch`), `eligibility: [Eligibility; 2]`, and `card_phase`; ticket 003's `CardId`/`cards.rs` provide each card's first-eligible faction. The generic action-tree/command/diagnostic envelopes come from `engine-core` (as used by `games/frontier_control/src/actions.rs`).
2. The eligibility constraint table is specified: verified the spec's "Sequence per event card" — first Event → second {Operation, Pass}; first Operation → second {Event, Limited Operation, Pass}; first Pass → second full menu (double pass discards, both stay eligible); Reckonings always resolve. This ticket implements the menu-construction half; operation construction itself is ticket 006.
3. Cross-crate boundary under audit: the legal action tree the choosing faction receives, and the empty-but-safe tree the waiting faction receives, flow through the same generic action-tree contract every prior game used; the menu constraint is Rust-authored and must never be recomputable in TypeScript (§2). The waiting tree must carry safe waiting metadata and never expose undrawn deck order.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: legal action generation, validation, and eligibility computation are Rust-only. Restated before trusting the spec: TypeScript must not decide eligibility or menu constraints; the menu the player sees is exactly the Rust-supplied constrained tree.

## Architecture Check

1. Modeling eligibility as a small per-faction state machine driven by card data + choices is cleaner than ceremony/flags: it is deterministic, replayable, and keeps "who may act and what they may choose" a pure function of public state.
2. No backwards-compatibility aliasing/shims — fills the action/rules stubs.
3. `engine-core` stays noun-free (eligibility/initiative nouns are `games/event_frontier`-local); no `game-stdlib` promotion (the §4 ledger recorded these as `local-only` first uses).

## Verification Layers

1. Every eligibility-table cell (§2 behavior authority) -> rule tests for first event/op/pass → second's menu, double pass, and the no-eligible-faction discard.
2. Flow never stalls -> a property test that every non-terminal state offers a legal choice or auto-resolves (Reckoning/discard); the waiting tree is empty with safe metadata.
3. Pass income + eligibility transitions -> rule tests that passing pays +1 and preserves eligibility, acting forfeits next-card eligibility, and eligibility restores at Reckoning (Reckoning reset itself is ticket 008).
4. No-leak in the flow surface -> a visibility assertion that `CardRevealed`/menu payloads expose only the public current/next card, never undrawn order (full no-leak suite is ticket 009/011).

## What to Change

### 1. Card reveal and eligibility determination (`src/rules.rs`)

On advancing to a card: set `current`/`next_public`, compute the first-eligible faction (printed faction if eligible, else the other if eligible, else discard unresolved — except Reckonings always resolve), and set `card_phase` to `FirstChoice { faction }`.

### 2. Constrained menus and transitions (`src/actions.rs`, `src/rules.rs`)

Build the choosing faction's legal tree (`event`, `op/...` placeholder paths expanded in ticket 006, `pass`) per the eligibility constraint table; build the second faction's constrained menu from `first_took`; apply pass income (+1, cap-respecting), eligibility transitions (acting → ineligible next card; passing → stays eligible), and the double-pass / no-eligible-faction discard. The waiting faction's tree is empty with safe waiting metadata.

### 3. Flow effects (`src/effects.rs`)

Emit `CardRevealed { card, next_public }`, `ChoiceTaken { faction, choice }`, `CardDiscarded { card, reason }`, `EligibilityChanged { faction, eligible, reason }`, `ResourcesChanged` (pass income) — public, never exposing undrawn order.

## Files to Touch

- `games/event_frontier/src/actions.rs` (modify; created by 003)
- `games/event_frontier/src/rules.rs` (modify; created by 003)
- `games/event_frontier/src/effects.rs` (modify; created by 003)

## Out of Scope

- Operation construction, site selection, costs (ticket 006) — `op` menu entries are placeholders here.
- Event/edict effect bodies (ticket 007) and the Reckoning pipeline / eligibility restoration semantics (ticket 008).
- Reaction windows / pending responses — explicitly forbidden; the flow is strictly sequential (spec Out of scope).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes every eligibility-table cell test (first event/op/pass → second's menu; double pass; no-eligible discard).
2. The flow-never-stalls property test passes over many bot-seeded games.
3. Pass-income and eligibility-transition tests pass; the waiting faction's tree is empty with safe metadata.

### Invariants

1. Eligibility and menu constraints are computed in Rust only; the waiting faction's tree never contains a legal action.
2. No card-flow payload exposes undrawn deck order beyond the public next card.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/rules.rs` — eligibility-table cells, pass income, discard cases.
2. `games/event_frontier/tests/property.rs` — flow-never-stalls and eligibility-consistency invariants.

### Commands

1. `cargo test -p event_frontier --test rules`
2. `cargo test -p event_frontier`
3. The per-crate rule/property tests are the correct boundary — initiative correctness is provable without operations/effects, which land in later tickets.
