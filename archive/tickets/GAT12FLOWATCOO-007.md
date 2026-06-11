# GAT12FLOWATCOO-007: Shared outcome and terminal detection

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/src/rules.rs` (terminal detection), `src/effects.rs` (`Terminal` effect with shared outcome + public summary)
**Deps**: GAT12FLOWATCOO-006

## Problem

`flood_watch` produces a **team** result, not a per-seat winner — a new shape for terminal state and the outcome-explanation surface. The match is `Lost` the moment any district reaches the inundation level (level 3), including mid-environment-phase, and `Won` when the final deck card resolves without a loss. The `Terminal` effect carries the shared outcome, surviving flood levels, and a public summary; there are no per-seat scores and no tie-breaks. After terminal, no action is legal.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-006 emits `DistrictInundated` (on level 3) and `DeckExhausted` (deck empty, no loss) but does not finalize the outcome; GAT12FLOWATCOO-004 defines `SharedOutcome` (`Won` / `Lost { district }`) and `Phase::Terminal`. The terminal-effect-carries-summary shape mirrors `games/masked_claims/src/effects.rs` `Terminal` but with a shared (non-per-seat) outcome.
2. The spec (§Implementation reference "Terminal", Work-breakdown item 6, Exit-criteria "shared outcome is tested") fixes: `Lost` immediately on any district at level 3 (mid-phase included); `Won` when the final deck card resolves without loss; `Terminal { outcome, summary }` carries surviving levels and drawn-card count; no per-seat winner. The `RULES.md` terminal IDs (`FW-END-*`, authored in GAT12FLOWATCOO-001) are the stable contract this binds to.
3. Cross-artifact boundary under audit: the `Terminal` effect's `summary` is consumed by the outcome-explanation surface (`outcomeExplanationTemplates.ts` + `check-outcome-explanations.mjs`, GAT12FLOWATCOO-017) and the WASM view (GAT12FLOWATCOO-014). The summary must be viewer-safe public data (surviving levels, drawn count) and carry no undrawn-deck data beyond the cards already drawn.
4. FOUNDATIONS §2 (Rust owns terminal detection and scoring) motivates this ticket: terminal is detected in Rust and exposed as a shared result; TypeScript renders it but does not compute it. The §11 invariant that views are viewer-safe applies to the terminal summary.
5. Enforcement surface: post-terminal immutability is a §11 acceptance invariant — no action is legal in `Phase::Terminal` (the tree is empty, validation rejects submissions), and the terminal state and its hash are stable for replay. The shared-outcome summary must not leak any undrawn-deck data, including post-terminal.

## Architecture Check

1. A single `SharedOutcome` on the state (set once, immutable) plus a `Terminal` effect carrying a public summary is cleaner than per-seat score fields the game does not have: it makes "no per-seat winner" structural and gives the outcome surface one team result to render.
2. No backwards-compatibility aliasing/shims; built on GAT12FLOWATCOO-006's `DistrictInundated`/`DeckExhausted` emissions.
3. `engine-core` stays noun-free — `SharedOutcome` and the terminal summary are game-local; only the generic effect-envelope contract is used.

## Verification Layers

1. Loss/win triggers -> rule tests: loss on any district reaching level 3 (including mid-phase early stop), win on the final card resolving without loss.
2. Always shared, bounded, immutable -> property tests: the outcome is always shared (never per-seat), terminal is reached within the deck bound, and no post-terminal action is legal.
3. Stable terminal state -> deterministic replay-hash check: terminal state and outcome reproduce across runs.
4. Outcome-explanation contract -> grep-proof the `Terminal` summary carries the public fields (`FW-END-*` IDs) the outcome surface consumes; rendering verified in GAT12FLOWATCOO-017.

## What to Change

### 1. `games/flood_watch/src/rules.rs`

Detect terminal during environment resolution: set `terminal_outcome = Lost { district }` and `Phase::Terminal` the moment a district reaches level 3 (consume the early-stop signal from GAT12FLOWATCOO-006); set `Won` when the final deck card resolves without a loss. Ensure validation rejects all submissions in `Phase::Terminal` and the legal tree is empty.

### 2. `games/flood_watch/src/effects.rs`

Define `Terminal { outcome, summary }` carrying the shared outcome, surviving flood levels, and drawn-card count as viewer-safe public data. No per-seat winner field; no undrawn-deck data.

## Files to Touch

- `games/flood_watch/src/rules.rs` (modify — terminal detection + post-terminal rejection)
- `games/flood_watch/src/effects.rs` (modify — `Terminal` effect)

## Out of Scope

- Public/private projection of the terminal view and effect filtering (GAT12FLOWATCOO-008).
- Outcome-explanation templates and their rule-ID mirrors on the web surface (GAT12FLOWATCOO-017).
- Bot behavior at/after terminal (GAT12FLOWATCOO-010).

## Acceptance Criteria

### Tests That Must Pass

1. Rule tests cover the loss trigger (any district at level 3, including mid-environment-phase early stop) and the win trigger (final card resolved without loss).
2. Property tests assert the outcome is always shared (never per-seat), terminal occurs within the deck bound, and no post-terminal action is legal.
3. A replay test asserts the terminal state and outcome reproduce identically under the same inputs.

### Invariants

1. The outcome is a team result with no per-seat winner and no tie-break.
2. Terminal is immutable: no action is legal afterward, and the terminal state/hash is replay-stable; the summary leaks no undrawn-deck data.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/rules.rs` — loss/win trigger cases including mid-phase early stop (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/property.rs` — shared-outcome / deck-bound / post-terminal-illegality invariants (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test rules`
2. `cargo test -p flood_watch`
3. The outcome-explanation render is verified at GAT12FLOWATCOO-017 via `check-outcome-explanations.mjs`; the rule + property tests are the correct boundary for the Rust terminal logic.

## Outcome

Accepted on 2026-06-11. Implemented shared terminal detection for Flood
Watch: inundation immediately sets `SharedOutcome::Lost`, deck exhaustion
without loss sets `SharedOutcome::Won`, terminal states move to
`Phase::Terminal`, and post-terminal action trees/validation are closed.
`Terminal` effects now carry a shared outcome plus a public summary containing
the stable `FW-END-*` rule ID, drawn-card count, and surviving flood levels,
with no per-seat winner and no undrawn-deck data.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p flood_watch --test rules`
3. `cargo test -p flood_watch --test property`
4. `cargo test -p flood_watch --test replay`
5. `cargo clippy -p flood_watch --all-targets -- -D warnings`
6. `cargo test -p flood_watch`
