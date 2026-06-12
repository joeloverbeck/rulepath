# GAT14EVEFROEVE-006: Operations — compound action trees, validation, application

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier/src/{actions,rules,effects}.rs` (progressive compound op trees, per-op validation, application)
**Deps**: GAT14EVEFROEVE-005

## Problem

Operations are the gate's large-action-tree proof: a compound multi-site action built through the existing progressive-construction contracts at the largest branching factor of any official game. Each operation is **one compound command** — op type → site set bounded by the card's ops value → per-site sub-choice — resolved through the action tree, **not** a budgeted sequence of separately validated commands. This ticket implements both factions' op types, progressive site selection bounded by ops value and affordability, per-site sub-choices, resource-cost validation against pools, and application, keeping tree generation within latency budgets.

## Assumption Reassessment (2026-06-12)

1. The card flow and `op` menu placeholders this expands exist: verified ticket 005 built the constrained menu with `op/...` placeholder paths and the eligibility/limited-op distinction (limited op = 1 site). State (resources, per-site components, adjacency) comes from ticket 004's `state.rs`.
2. The op catalog is specified: verified the spec's "Operations" — Charter `survey`/`fortify`/`writ`, Freeholder `trek`/`cache`/`rally`, cost 1 resource per selected site, N = ops value (limited op N = 1), with per-op legality (survey adjacency, fortify ≥2 agents + no depot, writ requires a cache, trek requires a settler + trail, cache at settler-occupied depot-free site under cap, rally at valid site).
3. Cross-crate boundary under audit: progressive op construction reuses the generic action-tree/action-path contract (`engine-core`) that served symmetric games — op type → sites → sub-choice as expanding tree levels with Rust-supplied per-step metadata (costs, ops-value bound, eligibility consequence). No new tree infrastructure (spec Assumption A7); the largest branching factor must still generate within the `<100ms` latency budget (`docs/TESTING-REPLAY-BENCHMARKING.md`).
4. FOUNDATIONS §2 (behavior authority) and §7 (legal-only UI) motivate this ticket: op legality, site bounds, and costs are Rust-only; the browser presents the Rust-supplied progressive tree. Restated before trusting the spec: TypeScript computes no op cost or site-bound; illegal site selections are absent from the tree, not validated after the fact.
5. Edict-modified costs/legality are consulted but defined later: this ticket validates against base costs/legality and pools; the active-edict modifier list (ticket 007) is consulted at the op validation/application points it modifies. Confirm the validation structure leaves a clean insertion point for the edict modifier list without reverse-patching base rules, and that one compound command (not budgeted turns) is enforced so the multi-action-budget non-use (ledger ticket 002) holds.

## Architecture Check

1. One compound command per operation (built progressively, validated as a whole) is cleaner than budgeted sub-commands: it proves the large-action-tree shape without re-arming the multi-action-budget hard gate, and keeps replay a single command per op.
2. No backwards-compatibility aliasing/shims — extends the action/rules stubs; no new tree contract.
3. `engine-core` stays noun-free (op nouns are local); no `game-stdlib` promotion.

## Verification Layers

1. Op legality and bounds (§2) -> rule tests for each op type's preconditions, site-count bounds (full vs limited), per-site sub-choices, and resource-cost/cap enforcement.
2. Compound-command shape (ledger non-use) -> a test asserting an operation is a single applied command over 1..N sites, not a sequence of separately validated commands.
3. Latency -> the peak-op-branching tree generates within budget (benchmark identity `legal_tree_peak_op_branching`, ticket 012); here, a property test that the tree is bounded and every path is legal.
4. Diagnostics viewer-safe -> diagnostic tests for unaffordable ops, over-budget site selection, and precondition violations name the typed reason without leaking hidden state.

## What to Change

### 1. Progressive op trees (`src/actions.rs`)

Expand the `op` menu placeholders into compound paths: op type → site set (1..N where N = ops value; limited op N = 1; bounded by affordability) → per-site sub-choice where the type requires it. Attach Rust-supplied per-step metadata (cost, remaining ops value, eligibility consequence).

### 2. Validation (`src/rules.rs`)

Validate: op type legal for the faction; site count within bound; affordability (1 resource/site vs pool); per-op preconditions (survey adjacency, fortify ≥2 agents + no depot, writ on a cache, trek settler+trail, cache depot-free under cap, rally valid site); reject stale freshness token, wrong-seat, ineligible/out-of-phase faction, menu-constraint violations. Leave a documented consultation point for the active-edict modifier list (ticket 007).

### 3. Application and effects (`src/effects.rs`)

Apply the compound op as one command; emit `OpResolved { faction, op, sites }` plus per-site effects (`AgentPlaced`, `DepotBuilt`, `CacheRemoved`, `SettlerMoved`, `CacheLaid`, `SettlerRallied`, …) in stable order, and `ResourcesChanged` for the cost.

## Files to Touch

- `games/event_frontier/src/actions.rs` (modify; created by 003)
- `games/event_frontier/src/rules.rs` (modify; created by 003)
- `games/event_frontier/src/effects.rs` (modify; created by 003)

## Out of Scope

- Event/edict effect bodies and the edict modifier system (ticket 007) — only the consultation point is left here.
- Reckoning, scoring, victory (ticket 008).
- Budgeted multi-command turns — explicitly forbidden (spec Out of scope; ledger non-use).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes per-op-type legality, site-bound (full vs limited), resource-cost, and cap tests.
2. The single-compound-command test passes (one op = one applied command over 1..N sites).
3. Diagnostic tests for unaffordable / over-budget / precondition-violating ops pass and are viewer-safe.

### Invariants

1. An operation is exactly one compound command; no budgeted command sequence exists.
2. Op legality, site bounds, and costs are Rust-computed; the action tree contains only legal paths.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/rules.rs` — per-op legality, bounds, costs, caps, diagnostics.
2. `games/event_frontier/tests/property.rs` — bounded legal op tree; compound-command shape.

### Commands

1. `cargo test -p event_frontier --test rules`
2. `cargo test -p event_frontier`
3. The per-crate tests are the correct boundary; peak-branching latency is asserted at the benchmark layer (ticket 012), not here.

## Outcome

Implemented base Event Frontier operations as single compound commands:

- Expanded operation action trees into typed operation roots and legal target leaves for Charter `survey`/`fortify`/`writ` and Freeholder `trek`/`cache`/`rally`.
- Added compound operation parsing/validation for `operation/<kind>/<payload>` and `limited_operation/<kind>/<payload>`, including faction ownership, full vs limited site bounds, affordability, duplicate-site rejection, and per-op preconditions.
- Added the documented ticket-007 edict consultation metadata point without implementing edict modifiers early.
- Applied operations as one command with stable selected-site order, resource spending, public `OpResolved`, and per-site effects for agent placement, depot build, cache removal, settler movement, cache laying, and rally.
- Added rule/property coverage for every op family, resource/cap/bound/precondition diagnostics, limited operations, single-command shape, and bounded legal operation leaves.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p event_frontier --test rules` — passed, 13 tests.
3. `cargo test -p event_frontier --test property` — passed, 2 tests.
4. `cargo test -p event_frontier` — passed, 16 unit tests, 18 integration tests, 0 doctests.
