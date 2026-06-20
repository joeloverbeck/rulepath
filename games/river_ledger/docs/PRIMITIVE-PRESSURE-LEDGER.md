# River Ledger Primitive-Pressure Ledger

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Created: 2026-06-14

Last updated: 2026-06-20

## Decision summary

Mechanic shape: N-seat hidden private hands, deterministic shuffle/deal,
community-card reveal, fixed-limit contribution accounting, single-pot
allocation, seven-card showdown evaluation, viewer-safe outcome explanation.

Status: `rejected/deferred with rationale`.

Decision: keep River Ledger mechanics game-local for Gate 15. No `game-stdlib`
promotion and no `engine-core` vocabulary expansion are authorized.

Review owner/date: Codex, 2026-06-14.

## Games exerting pressure

| Shape | Prior / comparison games | River Ledger pressure |
|---|---|---|
| deterministic shuffle plus private holdings | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | 3-6 seats, two private hole cards, public board, no-leak matrix, replay export/import. |
| staged reveal and public/private projection | `high_card_duel`, `poker_lite`, `masked_claims` | Flop/turn/river plus authorized showdown reveal and foldout no-reveal terminal. |
| public accounting / allocation | `token_bazaar`, `poker_lite`, `event_frontier` | Blinds, fixed-limit street contributions, one pot, split/remainder order. |
| terminal comparison explanation | `high_card_duel`, `poker_lite` | Poker-category evaluator, rank-vector tie breaks, best-five used cards. |
| N-seat surface | Infra A-D shared seat/public infrastructure | Official 3, 4, 5, and 6 seat support with ordered pair no-leak proof. |

## Implementation evidence

| Pressured shape | Implemented files | Tests/traces |
|---|---|---|
| standard deck and deterministic shuffle | `games/river_ledger/src/cards.rs`, `games/river_ledger/src/setup.rs`, `games/river_ledger/src/replay_support.rs` | `games/river_ledger/tests/serialization.rs`, `games/river_ledger/tests/replay.rs`, `games/river_ledger/tests/golden_traces/setup-3p.trace.json`, `games/river_ledger/tests/golden_traces/setup-4p.trace.json`, `games/river_ledger/tests/golden_traces/setup-5p.trace.json`, `games/river_ledger/tests/golden_traces/setup-6p.trace.json` |
| hidden hole cards and owner-private views | `games/river_ledger/src/setup.rs`, `games/river_ledger/src/visibility.rs`, `games/river_ledger/src/effects.rs` | `games/river_ledger/tests/visibility.rs`, `games/river_ledger/tests/golden_traces/deal-private-no-leak.trace.json`, `games/river_ledger/tests/golden_traces/public-observer-no-leak.trace.json`, `games/river_ledger/tests/golden_traces/seat-private-view.trace.json`, `games/river_ledger/tests/golden_traces/wrong-seat-diagnostic.trace.json` |
| N-seat projections and pairwise no-leak | `games/river_ledger/src/ids.rs`, `games/river_ledger/src/visibility.rs`, `games/river_ledger/src/replay_support.rs` | `games/river_ledger/tests/visibility.rs`, `crates/wasm-api` no-leak dispatch, `apps/web/e2e/river-ledger.smoke.mjs` |
| fixed-limit contribution ledger and cap | `games/river_ledger/src/betting.rs`, `games/river_ledger/src/actions.rs`, `games/river_ledger/src/rules.rs`, `games/river_ledger/src/state.rs` | `games/river_ledger/tests/rules.rs`, `games/river_ledger/tests/property.rs`, `games/river_ledger/tests/golden_traces/preflop-blinds-call-check-advance.trace.json`, `games/river_ledger/tests/golden_traces/flop-small-bet-cap.trace.json`, `games/river_ledger/tests/golden_traces/turn-river-big-bet.trace.json`, `games/river_ledger/tests/golden_traces/raise-cap-diagnostic.trace.json` |
| seven-card evaluator and showdown rationale | `games/river_ledger/src/evaluator.rs`, `games/river_ledger/src/showdown.rs`, `games/river_ledger/src/rules.rs` | `games/river_ledger/tests/rules.rs`, `games/river_ledger/tests/golden_traces/high-card-showdown.trace.json`, `games/river_ledger/tests/golden_traces/pair-beats-high-card.trace.json`, `games/river_ledger/tests/golden_traces/straight-ace-low.trace.json`, `games/river_ledger/tests/golden_traces/flush-kicker-order.trace.json`, `games/river_ledger/tests/golden_traces/full-house-tiebreak.trace.json` |
| split allocation and deterministic remainder | `games/river_ledger/src/pot.rs`, `games/river_ledger/src/showdown.rs`, `games/river_ledger/src/state.rs` | `games/river_ledger/tests/rules.rs`, `games/river_ledger/tests/property.rs`, `games/river_ledger/tests/golden_traces/split-pot-even.trace.json`, `games/river_ledger/tests/golden_traces/split-pot-remainder-button-order.trace.json` |
| bots and viewer-safe explanations | `games/river_ledger/src/bots.rs`, `games/river_ledger/src/visibility.rs` | `games/river_ledger/tests/bots.rs`, `games/river_ledger/tests/golden_traces/bot-vs-bot-full-game-6p.trace.json` |

## What is repeated

- deterministic shuffled card-like components;
- owner-private hidden holdings;
- redacted observer/opponent projections;
- public contribution counters and terminal allocation;
- Rust-authored terminal rationale;
- legal-action-only bots with viewer-safe explanations.

## What differs

- River Ledger has variable official seat counts from 3 to 6.
- River Ledger uses two hole cards plus five public community cards, not a
  single private card, small custom deck, or trick hand.
- Legal actions are coupled to fixed-limit street contribution state and a
  street-specific raise cap.
- Showdown requires seven-card best-hand evaluation and split-remainder
  allocation.
- Foldout terminal must keep folded private cards redacted.
- Pairwise no-leak proof covers every ordered pair of distinct seats for four
  different seat counts.

## Why local duplication is acceptable now

The repeated shapes are real, but the shared subset is either too small
(deterministic shuffle) or too behavior-bearing (visibility, contribution
accounting, evaluator, terminal allocation). Promoting a helper now would invite
policy flags for reveal timing, betting eligibility, pot allocation, bot inputs,
and terminal explanation. Those policies belong in the game crate.

## Boundary impact

Why not `engine-core`: all named mechanics are game nouns. `engine-core` remains
generic and noun-free.

Why not `game-stdlib`: no narrow behavior-free helper has enough proven value
for Gate 15. Existing promoted `board_space` is not applicable. Public resource
accounting and deterministic private-hand shapes already have deferred/rejected
atlas decisions, and River Ledger does not change that decision at admission.

Data/Rust boundary impact: static data may describe variants, labels, fixtures,
and reports only. Betting, evaluation, visibility, and bot behavior remain
typed Rust.

Replay/hash impact: no shared helper changes existing replay or hash semantics.
River Ledger has game-local traces and hashes under
`games/river_ledger/tests/golden_traces/`.

Visibility impact: visibility is the main risk and stays local. Rust visibility
tests, WASM projection dispatch, public replay export/import traces, and browser
no-leak smoke prove the implemented game-specific boundary without promoting a
visibility helper.

Bot impact: bot policy remains game-local and authorized-view-only.

UI/effect impact: UI metadata and effects may share presentation shells, but
River Ledger behavior facts remain Rust-owned in the game crate.

## Final evidence reviewed

- `games/river_ledger/docs/MECHANICS.md` is complete and consistent with this
  ledger.
- Pairwise no-leak coverage exists for 3, 4, 5, and 6 seats through Rust
  visibility tests, WASM redaction, public replay export/import traces, and the
  River Ledger browser smoke.
- Contribution-ledger behavior is covered by rule/property tests, street/cap
  golden traces, simulator evidence, and benchmark notes.
- Evaluator, showdown, split, and remainder behavior are covered by evaluator
  tests and named golden traces.
- Bot legality and bot-explanation no-leak boundaries are covered by
  `games/river_ledger/tests/bots.rs` and the Level 2 evidence pack.
- `docs/MECHANIC-ATLAS.md` records the Gate 15 decision as
  `game-local / no promotion` while §10A remains `Current debt: _None_.`

## Back-port or conformance plan

No helper is promoted, so no prior game requires back-porting. No promotion debt
is opened.

Affected prior games: not applicable.

Exceptions: not applicable.

Closure gate if debt is deferred: not applicable because there is no promotion
debt. Reopen this ledger for Gate 15.1 side-pot/all-in work.

## Agent misuse risks

- Extracting a generic card/deck/hand helper before visibility and replay
  evidence exists.
- Moving contribution/pot vocabulary into `engine-core`.
- Encoding betting formulas, evaluator decisions, or visibility selectors in
  static data.
- Letting browser code infer legal actions, hand strength, winners, splits, or
  hidden facts.
- Treating "Texas Hold'Em family" as permission to copy prose, tables, or
  presentation.

## Gate 15.1 side-pot / all-in pressure decision

Mechanic shape: deterministic layered contribution-cap construction,
folded-money inclusion, per-pot eligibility, uncalled-return extraction,
independent split/remainder allocation, and viewer-safe allocation explanation.

Status: `rejected/deferred with rationale`.

Decision: reuse River Ledger's local `winners_in_button_order` helper, implement
the contribution-layer constructor and per-pot allocator locally in
`games/river_ledger`, defer/reject `game-stdlib` promotion, authorize no
`engine-core` vocabulary expansion, and require no ADR for the planned
game-local delta.

Review owner/date: Rulepath maintainers / Gate 15.1 acceptance and closeout.

### Games exerting pressure

| Shape | Prior / comparison games | Gate 15.1 pressure |
|---|---|---|
| single-pot integer split and remainder | `poker_lite`, River Ledger Gate 15 | Gate 15.1 adds nested eligibility layers, returned top excess, independent per-pot remainders, and final-stack aggregation. |
| public accounting / allocation | `token_bazaar`, `poker_lite`, `event_frontier`, River Ledger Gate 15 | Gate 15.1 introduces public finite stacks and contribution caps, but allocation remains tied to River Ledger's betting history and showdown model. |
| hidden-information terminal explanation | `high_card_duel`, `poker_lite`, River Ledger Gate 15 | Gate 15.1 explains every pot and return while preserving folded/no-showdown card redaction. |

### Relevant files and docs

- `games/river_ledger/src/state.rs`
- `games/river_ledger/src/actions.rs`
- `games/river_ledger/src/betting.rs`
- `games/river_ledger/src/rules.rs`
- `games/river_ledger/src/pot.rs`
- `games/river_ledger/src/showdown.rs`
- `games/river_ledger/src/effects.rs`
- `games/river_ledger/src/visibility.rs`
- `games/river_ledger/src/ui.rs`
- `games/river_ledger/src/bots.rs`
- `archive/specs/gate-15-1-river-ledger-all-in-side-pots.md`
- `docs/ROADMAP.md` Gate 15.1

### What is repeated

- Integer contribution accounting and split/remainder allocation already exist
  locally in River Ledger and in earlier simpler games.
- Public contribution totals and terminal awards are already viewer-safe public
  facts in the shipped base game.
- The button-ordered remainder concept already exists inside River Ledger.

### What differs

- Side-pot construction depends on River Ledger contribution caps, folded/all-in
  statuses, fixed-limit action history, full-unit reopen state, button order,
  seven-card showdown evaluation, and public explanation model.
- Poker Lite and the Gate 15 base have single-pot split behavior, but not
  ordered nested eligibility layers or returned top layers.
- Token Bazaar and Event Frontier exert public-accounting pressure, but they do
  not have hidden hands, all-in contribution caps, per-pot eligibility, or
  showdown allocation.

### Why local duplication is acceptable now

The full layered-eligibility/allocation shape has one implementation pressure
point: River Ledger Gate 15.1. Premature extraction would either encode poker
nouns and policies into a shared helper or expose a generic API whose real
boundary is still unproven. A local pure constructor and local allocator are
acceptable because the repo has no third close official use and no narrow
behavior-free cross-game contract.

### Boundary impact

Why not `engine-core`: the shape contains game nouns and policy: seat,
contribution, fold, all-in, pot, eligibility, button, and showdown. Those are
forbidden in the noun-free kernel.

Why not `game-stdlib`: no promoted primitive is appropriate yet. The narrowest
useful helper would still need eligibility, returned excess, per-pot split,
remainder, visibility, and explanation policy. Those policies are not proven
across official games.

Data/Rust boundary impact: stack defaults and neutral presets may be typed
setup data. Validation, cap handling, layer construction, eligibility, returns,
allocation, and explanations remain typed Rust behavior.

Replay/hash impact: Gate 15.1 intentionally creates River Ledger v2 state,
hash, serialization, and trace drift. Ordered pots, returns, shares, awards,
and final stacks must be stable and hashed game-locally. No global Trace Schema
or hash algorithm change is authorized.

Visibility impact: public stacks, contributions, pot amounts, and eligibility
are public facts; private cards, deck order, folded hand contents, evaluator
internals, bot rankings, and unauthorized replay data remain redacted by
viewer. Pairwise N-seat no-leak proof is required.

Bot impact: bots may read legal actions plus authorized public stack, pot,
call-price, and eligibility facts. They may not inspect opponent hands, deck
order, omniscient rankings, rollout data, or any forbidden search/learning
method.

UI/effect impact: Rust authors all all-in, stack, return, pot-tier, award,
remainder, and explanation effects/projections. TypeScript renders those facts
and computes no legality, eligibility, winner, or allocation.

### Tests and benchmarks required

Required tests: pure constructor unit/property tests, rule/state tests,
replay/serialization checks, 3-6-seat pairwise no-leak, bot legality and
determinism, WASM API snapshots, browser no-leak/e2e smoke, and the Gate 15.1
golden trace set.

Required benchmarks: six-seat maximum-layer construction, multi-pot split
allocation, all-in showdown, projection, serialization/replay, bot policy, and
full-game all-in-pressure paths.

### Back-port or conformance plan

No helper is promoted, so no prior official game requires back-porting.
Preserve Poker Lite and River Ledger Gate 15 behavior except for the explicit
River Ledger v2 migration.

Affected prior games: none behaviorally. `poker_lite` is comparison evidence
only.

Exceptions: River Ledger's local `winners_in_button_order` remains local and may
be reused by the Gate 15.1 per-pot allocator.

Closure gate if debt is deferred: not applicable. This decision closes with no
promotion debt and keeps the atlas §10A debt register empty.

### Examples and anti-examples

Examples:

- A pure `games/river_ledger/src/pot.rs` function from ordered
  contributions/statuses to ordered pots and returns.
- A game-local per-pot allocator that uses River Ledger's existing button order.

Anti-examples:

- `engine_core::SidePot`.
- A universal `game-stdlib` poker-pot helper.
- TOML or JSON formulas for eligibility or allocation.
- TypeScript constructing pots, winners, shares, or final stacks.

### Agent misuse risks for Gate 15.1

- Treating public eligibility as authority to reveal cards.
- Using hash-map iteration for pot, contributor, eligible, share, or effect
  ordering.
- Creating a separate pot for every folded contribution boundary without
  coalescing identical eligibility.
- Using saturation to hide accounting overflow.
- Reconstructing stack, pot, winner, or final-stack arithmetic in WASM or
  TypeScript.
