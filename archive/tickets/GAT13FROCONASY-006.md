# GAT13FROCONASY-006: Round scoring, supply connectivity, and terminal

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/frontier_control/src/{rules,effects}.rs` (end-of-round scoring, Rust connectivity traversal, terminal detection + tiebreak, `RoundScored`/`Terminal` effects)
**Deps**: GAT13FROCONASY-005

## Problem

When the Garrison's turn ends, the round must score as a deterministic consequence of that command's application (the Gate 12 automation mechanism at small scale): the Garrison scores per held fort (≥1 guard, 0 crews); the Prospectors score each staked site's value if a guard-free edge path connects it to the base camp (a Rust graph traversal — connectivity is score-bearing behavior and must never be a TypeScript computation). After the configured final round, the match ends: higher total wins, with a deterministic Garrison tiebreak. `RoundScored` and `Terminal` carry the full per-faction breakdown for the effect log, score tracks, and outcome surface.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/src/rules.rs` end-of-phase automation (scoring inside the turn-ending command's application) is the mechanism exemplar; the state model + adjacency index (GAT13FROCONASY-004) and the application path (GAT13FROCONASY-005) are in place.
2. Spec §Round scoring and §Terminal define the formulas (Garrison fort points; Prospector supply-connected stake values), the supplied/cut semantics (cut stakes score zero but remain), the eight-round bound, and the Garrison tiebreak (stable rule ID `FC-TERM-GARRISON-TIEBREAK` from GAT13FROCONASY-001).
3. Cross-crate boundary under audit: the per-stake supplied/cut set computed here is projected into the public view (GAT13FROCONASY-007) so the UI never re-derives connectivity; the `RoundScored`/`Terminal` breakdown is consumed by effects, the score track, and the outcome-explanation surface.
4. FOUNDATIONS §2 behavior authority under audit: connectivity traversal, scoring, terminal detection, and tiebreak are Rust; TypeScript receives the computed supplied/cut flags and totals only.
5. §11 determinism enforcement surface: round scoring runs exactly once per round, the traversal is deterministic over the validated graph, and terminal occurs exactly at the configured round bound; these feed the replay/hash surface (GAT13FROCONASY-007) and must add no nondeterminism. No hidden information exists.
6. Schema extension: `RoundScored { round, garrison_points, prospector_points, fort_breakdown, stake_breakdown }` and `Terminal { winner, totals, tiebreak_applied, summary }` extend the effect envelope additively; consumers are visibility/replay and the web effect log / outcome templates.

## Architecture Check

1. Computing supply connectivity once at scoring time and projecting the per-stake result keeps the single source of truth in Rust and removes any incentive for a TypeScript graph traversal; recomputing in the view layer would be a §2/§12 violation.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; the connectivity/control logic stays in `games/frontier_control` (first official use, local per GAT13FROCONASY-002); no `ConnectivityScorer` helper is promoted.

## Verification Layers

1. Supply-connectivity scoring -> golden trace (`supply-cut-scores-zero.trace.json`: a stake disconnected by a guard scores zero, then reconnects and restores) + rule tests.
2. Per-round + terminal correctness -> golden traces (`round-scoring-breakdown`, `standard-garrison-win`, `standard-prospector-win`, `tie-garrison-tiebreak`) + property test (scoring runs once/round; terminal at the round bound; winner matches comparison + tiebreak).
3. Determinism (§11) -> deterministic replay-hash check (scoring breakdown + terminal reproduce under the same command stream — exercised in GAT13FROCONASY-007/009).

## What to Change

### 1. Connectivity + round scoring (`rules.rs`)

In the Garrison turn-ending command's application, compute Garrison fort points and the Prospector supplied/cut set via a guard-free path traversal to the base camp; total per-faction scores; record the breakdown.

### 2. Terminal detection + tiebreak (`rules.rs`)

After the final round scores, set the terminal outcome (higher score; Garrison tiebreak), carrying winner, both totals, the final breakdown, and the decisive cause.

### 3. Effects (`effects.rs`)

Emit `RoundScored` (with fort/stake breakdown and per-stake supplied/cut flags) and `Terminal`.

## Files to Touch

- `games/frontier_control/src/rules.rs` (modify)
- `games/frontier_control/src/effects.rs` (modify)

## Out of Scope

- Public projection of the supplied/cut set and replay hashing (GAT13FROCONASY-007).
- Bots, full trace set, benchmarks (later tickets).
- Outcome-explanation TypeScript templates (GAT13FROCONASY-015/016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` scoring tests: connected stakes score value; a stake cut on every path scores zero; reconnecting restores it; Garrison fort points correct; terminal + tiebreak correct.
2. Property test: round scoring runs exactly once per round; terminal occurs exactly at the round bound; winner matches score comparison + tiebreak.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. Supply connectivity and scoring are computed only in Rust; the view carries computed flags, never a traversal input that invites recomputation (§2).
2. Scoring and terminal are deterministic and bounded (§11).

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/rules.rs` — scoring, supply-cut, terminal, tiebreak cases.
2. `games/frontier_control/tests/property.rs` — once-per-round scoring + terminal-bound invariants (expanded in GAT13FROCONASY-009).

### Commands

1. `cargo test -p frontier_control scoring`
2. `cargo test -p frontier_control`
3. Crate-scoped tests are the correct boundary; cross-tool replay-hash of the scoring breakdown is exercised after the replay surface lands in GAT13FROCONASY-007.

## Outcome

Completed on 2026-06-11.

Changed `games/frontier_control/src/rules.rs`, `games/frontier_control/src/effects.rs`, and `games/frontier_control/src/lib.rs`.

Implemented Rust-owned round scoring inside the Garrison turn-ending command, including held-fort points, guard-free supply connectivity traversal from stakes to Base Camp, cumulative scores, final-round terminal detection, and the Garrison score-tie tiebreak. Added public `RoundScored` and `Terminal` effects with fort and stake breakdown payloads, including per-stake supplied/cut flags.

Deviation: standalone property tests and golden traces remain deferred to the later verification/replay tickets named by this ticket; crate-local scoring tests cover connected stakes, cut stakes scoring zero, reconnection restoring stake value, fort scoring, final-round terminal, and tiebreak behavior.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p frontier_control scoring` — passed, 3 tests.
3. `cargo test -p frontier_control` — passed, 21 tests plus doc tests.
4. `cargo clippy -p frontier_control --all-targets -- -D warnings` — passed.
