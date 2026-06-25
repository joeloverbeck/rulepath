# GAT18BLAPACSPA-004: blind-nil commitment and deterministic deal

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/blackglass_pact` (setup/bidding/state/effects, RNG, owner-private hands) + golden traces
**Deps**: GAT18BLAPACSPA-003

## Problem

Implement the pre-deal `BlindNilCommitment` phase and the deterministic full deal. Eligible seats (team trailing ≥100) decide `blind_nil/declare` or `blind_nil/decline` left of dealer **before any card identity exists**; the shuffle/deal is then derived from match seed + hand index + versions only (blind choices cannot perturb RNG), dealing 13 owner-private cards to each of four seats with no tail (spec §3.1 blind rows, §3.2, §8.7, `BP-BLIND-*`/`BP-DEAL-*`, candidate task `GAT18-BLAPAC-004`).

## Assumption Reassessment (2026-06-25)

1. The `Phase::BlindNilCommitment { pending, next_index }` stub from GAT18BLAPACSPA-003 (`state.rs`, Appendix B.2) is modified here; `setup.rs` gains eligibility + deal. The deck-construction model from `cards.rs` is reused unchanged.
2. Spec §3.1 pins eligibility (≥100 deficit), order (left of dealer clockwise), immutability, and RNG independence; spec §8.7 pins seed inputs.
3. Cross-artifact boundary under audit: the pre-deal action tree / view / bot input / effect surfaces must carry zero card identity — the same no-leak firewall the visibility ticket (GAT18BLAPACSPA-008) later proves pairwise.
4. FOUNDATIONS §11 (no-leak firewall) and §2 (deterministic RNG) motivate this ticket: the blind surface is the gate's headline hidden-information proof; declare/decline must not observe or steer future cards.
5. Enforcement surfaces named: pre-deal action tree / preview / bot input / effect / export must expose no card, suit, rank, deck index, or RNG sample (§11); the deal bytes must be identical under declare vs decline for a fixed seed (§2/§11 determinism). Both are proven by golden traces + paired-seed property tests here and re-proven cross-viewer in 008.

## Architecture Check

1. Committing blind nil **before** the shuffle (rather than dealing then hiding cards in the UI) makes authorization and replay evidence unambiguous and removes any "the actor secretly saw cards" path.
2. No shims; isolates bot/decision RNG from game-deal RNG via the engine's deterministic contract.
3. `engine-core` untouched; blind/deal policy is game-local; no `game-stdlib` change.

## Verification Layers

1. No card/future-deck identity on any pre-deal surface -> no-leak golden trace `blind-nil-declare-before-deal-no-card-surface` + visibility unit test.
2. Declare vs decline produce byte-identical deal for a fixed seed -> paired-seed property test + `deal-identical-after-blind-declare-vs-decline` trace.
3. Full 52-card deal, 13 per seat, no tail; eligibility boundary at 99/100 -> property test + boundary unit tests + deterministic-deal trace.

## What to Change

### 1. Blind-nil eligibility and action tree

`setup.rs`/`bidding.rs`: per-seat eligibility (team deficit ≥100 at hand start), clockwise decision order, deterministic skip of ineligible seats, exactly `blind_nil/declare`|`blind_nil/decline` leaves for the active eligible seat, public immutable acceptance.

### 2. Deterministic deal + private hands

`setup.rs`/`state.rs`: derive shuffle from seed+hand_index+versions (independent of blind choices), deal singly clockwise from left of dealer, 13 owner-private cards per seat, populate `hands` only after the final blind decision.

### 3. Effects + traces

`effects.rs`: `BlindNilWindowOpened`/`BlindNilDeclared`/`BlindNilDeclined`/`DealCompleted` (public, no card identities); `PrivateHandReceived` (seat-private). Add the blind/deal golden traces (spec §7.6 #5–#13).

## Files to Touch

- `games/blackglass_pact/src/{setup,bidding,state,effects}.rs` (modify)
- `games/blackglass_pact/tests/{rules,property,visibility}.rs` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — blind/deal scenarios)
- `games/blackglass_pact/data/fixtures/blackglass_pact_blind_nil.fixture.json` (new)

## Out of Scope

- Ordinary bidding (GAT18BLAPACSPA-005) and trick play (GAT18BLAPACSPA-006).
- Cross-viewer pairwise no-leak harness + exports (GAT18BLAPACSPA-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test visibility` (no pre-deal card identity on any surface).
2. `cargo test -p blackglass_pact --test property` (paired-seed deal invariance; 13×4 conservation).
3. `cargo test -p blackglass_pact --test rules` (eligibility at 99 vs 100; ineligible-seat skip).

### Invariants

1. No card, deck order, or RNG sample is observable before the blind decision completes.
2. Blind declare/decline never changes shuffle seed, draw count, or deal order.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/visibility.rs` — pre-deal no-card-surface assertions.
2. `games/blackglass_pact/tests/property.rs` — declare/decline deal-byte invariance under fixed seed.
3. `games/blackglass_pact/tests/golden_traces/blind-nil-declare-before-deal-no-card-surface.trace.json` — replayable no-leak evidence.

### Commands

1. `cargo test -p blackglass_pact --test visibility --test property --test rules`
2. `cargo test -p blackglass_pact`
3. Crate-scoped tests are the boundary; `replay-check` registration (GAT18BLAPACSPA-011) validates the traces cross-cuttingly.
