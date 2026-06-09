# Primitive Pressure Ledger: hidden cards, public accounting, and bounded pledge

Candidate name: `poker-lite-hidden-accounting-pledge`

Status: repeated-shape candidate for hidden cards and public accounting; local-only first use for bounded pledge/shared-pool allocation

Last updated: 2026-06-09

Prepared by: `Codex`

## Hard Gate

This ledger records Gate 10's primitive-pressure decision for Crest Ledger. No
third-use hard gate fires in this spec.

Decision: defer/reject extraction and keep the mechanics local.

`poker_lite` is the second official similar use of deterministic private-card
visibility after `high_card_duel`, and the second official similar use of
public accounting after `token_bazaar`. It is the first official use of bounded
pledge rounds and shared-pool terminal allocation. These shapes are recorded in
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) §10B. No
helper is added to `engine-core` or `game-stdlib`.

## Mechanic Shapes

Repeated shapes considered:

- deterministic setup shuffle with viewer-scoped private-card projection;
- hidden center/staged reveal and grouped showdown reveal;
- public resource/accounting ledger with exact terminal allocation.

First-use shape considered:

- bounded pledge rounds with fixed units, one-lift cap, response choices, yield
  terminal, and shared-pool allocation.

These are game mechanics, not kernel contracts. They must not add `card`,
`deck`, `hand`, `bet`, `pool`, `pledge`, `showdown`, or similar nouns to
`engine-core`.

## Games Exerting Pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `high_card_duel` | Gate 8 | `games/high_card_duel/src/*`, hidden-info docs/traces | first deterministic private-card/reveal proof | implemented | Private hands, hidden face-down commitment, simultaneous reveal, redacted export. |
| `token_bazaar` | Gate 9 | `games/token_bazaar/src/*`, economy docs/traces | first public accounting proof | implemented | Public supply, inventories, exact payments, visible costs, deterministic refill and scoring. |
| `poker_lite` | Gate 10 poker_lite half | `games/poker_lite/src/*`, docs/traces/browser smoke | second card/private and accounting pressure; first pledge/shared-pool pressure | implemented | Owner-private crest, hidden center, public contributions, bounded pledge, yield/showdown allocation. |

## Local Implementations Compared

| Aspect | `high_card_duel` | `token_bazaar` | `poker_lite` | Same shape? | Notes |
|---|---|---|---|---:|---|
| state shape | Two private cards plus reveal state. | Public resource supplies, inventories, market/contract state. | Two private crests, hidden center crest, public contributions, shared pool, pledge round state. | partial | `poker_lite` overlaps both prior pressures but combines them with pledge state. |
| action shape | Simple Rust-owned reveal/choice flow. | Rust-owned economy actions. | Rust-owned `hold`/`press`/`lift`/`match`/`yield` choices. | partial | Action legality remains too game-specific for a helper. |
| validation | Hidden-info and terminal validation. | Public payment/availability validation. | Active-seat, stale, lift-cap, pledge, yield, and terminal validation. | partial | Shared validation would invite rule-policy flags. |
| transitions | Deterministic reveal and outcome. | Deterministic exchanges/refills/scoring. | Round close, staged center reveal, yield terminal, showdown reveal, allocation. | partial | Transition policy differs materially. |
| semantic effects | Deal/reveal/outcome effects. | Accounting and market effects. | Private deal, pledge, center reveal, showdown reveal, ledger allocation, terminal effects. | partial | Effect names and payloads remain game-owned. |
| visibility | Owner/private/public card filtering and public export redaction. | Perfect information public economy. | Owner-private crest, hidden center, yield no-reveal, grouped showdown, public export redaction. | partial | Visibility overlap is real, but no common safe helper boundary is proven. |
| UI pattern | Hidden card proof UI. | Public accounting/economy UI. | Seat ledgers, center/private view, action buttons, grouped showdown, no-leak e2e. | partial | UI remains presentation-only. |
| bot use | Bot acts from allowed information. | Bot uses public economy facts. | Level 2 uses own private rank plus public center/ledger state only. | partial | Bot policies stay local. |
| replay/hash impact | Public export redaction and deterministic traces. | Stable accounting traces. | Golden traces, public export/import, WASM export, browser replay import/step. | yes | Extraction would risk trace/hash migration without clear payoff. |
| benchmark pressure | Small hidden-info proof. | Economy benchmark proof. | Setup/playout, action generation, apply, projection, export/import benches. | partial | Current benchmarks do not require shared helpers. |

## Similarities

- `high_card_duel` and `poker_lite` both prove deterministic setup, private
  owner knowledge, redacted public/opponent views, replay/export safety, and
  browser no-leak behavior.
- `token_bazaar` and `poker_lite` both prove public deterministic accounting
  with exact terminal effects.
- All three rely on Rust for legality, validation, effects, replay, views, and
  bot decisions.

## Differences

- `poker_lite` has an initially hidden center crest, staged center reveal, and
  yield terminal without private reveal; `high_card_duel` does not.
- `poker_lite` accounting is a shared contribution pool with pledge responses;
  `token_bazaar` accounting is an economy/market proof with visible costs and
  refill rules.
- `poker_lite` response windows and lift caps are first-use local pledge
  behavior, not a repeated generic reaction primitive.
- A helper broad enough to cover all overlap would need flags for visibility,
  reveal timing, accounting policy, terminal allocation, effects, and bot
  context, which is exactly the behavior language the atlas forbids.

## Extraction Decision

The recorded decision is defer/reject extraction and keep local.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for card/private and public accounting; no for bounded pledge yet |
| helper can stay narrow and typed? | no current helper boundary is narrow enough |
| helper belongs in `game-stdlib`? | no for Gate 10 |
| would contaminate `engine-core`? | yes if card/accounting/pledge nouns moved there; therefore forbidden |
| static-data behavior risk? | medium if formulas or reveal policy become configurable; current data remains non-behavioral |
| replay/hash impact acceptable? | no extraction impact is justified; local traces stay preserved |
| visibility/no-leak impact acceptable? | yes for local implementation; extraction would require new proof |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no; benchmarks prove local performance only |
| ADR required? | no, because no architecture or boundary change is made |

Rationale:

- First use remains local by atlas law.
- Second similar use normally remains local after honest comparison.
- Crest Ledger's card/private and accounting overlaps are real, but the
  differences are behavior-bearing.
- `plain_tricks` or another later official game may create the third-use review
  trigger. That future task must decide reuse, promotion, explicit deferral, or
  ADR before duplicating the same shape again.

## Rejected Alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Promote card/deck/private-hand helpers now | Only two similar official uses exist, and reveal timing differs. | Third card/private use must reopen. |
| Promote public accounting helpers now | Token Bazaar and Crest Ledger account different resources with different terminal policy. | Third economy/accounting use must reopen. |
| Promote pledge/shared-pool helpers now | Crest Ledger is the first official bounded pledge/shared-pool use. | First use stays local. |
| Move nouns into `engine-core` | Forbidden by FOUNDATIONS; these are game/mechanic nouns. | No kernel change. |
| Encode formulas or reveal policy in data | Would violate the static-data boundary. | Rust keeps behavior. |

## API Sketch In Prose Only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/poker_lite` |
| determinism requirements | existing game-local traces and hashes remain stable |
| replay/hash requirements | no migration |
| visibility requirements | hidden-info projection remains game-local and Rust-owned |
| effect/log requirements | effect names and payloads remain game-local |
| bot-facing notes | bots consume game-local safe inputs only |
| non-goals | generic cards, generic pool accounting, generic pledge/betting, generic showdown, TypeScript legality |
| good-fit examples | none until a later third-use ledger proves a narrow behavior-free shape |
| anti-examples | decide reveal timing, compare strengths, allocate a pool, generate pledge choices, expose hidden cards |

## Determinism, Visibility, UI, Bot, And Benchmark Impact

| Area | Impact | Required safeguard/test |
|---|---|---|
| trace hashes | none; no extraction or migration | `cargo run -p replay-check -- --game poker_lite` |
| serialization | none; game-local stable serialization remains | `cargo test -p poker_lite --test serialization` |
| public view/action tree/effects | none; game-local viewer filtering remains | visibility tests, WASM tests, browser no-leak smoke |
| replay export/import | none; redaction remains game-local | golden traces, WASM export fixture, browser replay import/step |
| UI controls/action mapping | none; TypeScript maps Rust action choices only | `npm --prefix apps/web run smoke:ui`, `smoke:e2e` |
| bot policy | none; Level 2 remains authored and local | bot tests and evidence pack |
| benchmarks | none; benchmark evidence remains local | `cargo bench -p poker_lite` |

## Tests Required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | covered by `RULE-COVERAGE.md` |
| golden trace preservation/update notes | yes | preserved; no migration |
| property/invariant tests | yes | covered locally |
| replay/hash tests | yes | covered locally |
| serialization tests | yes | covered locally |
| visibility/no-leak tests | yes | covered locally, WASM, and browser |
| bot tests | yes | covered locally |
| benchmark tests | yes | covered locally |

## Traces Affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `high_card_duel`, `token_bazaar` | preserve | No shared helper or migration. | prior card/accounting pressure |
| all `games/poker_lite/tests/golden_traces/*.json` | `poker_lite` | preserve | Gate 10 local implementation is the accepted evidence. | `CL-*` rules |

## Examples

Good fits for future review only:

- a behavior-free hidden-component projection helper proven by at least three
  official games with matching visibility semantics;
- a behavior-free accounting total helper that does not encode payment,
  availability, terminal allocation, or game vocabulary.

## Anti-Examples

Not a fit:

- generate legal pledge choices;
- decide center or showdown reveal timing;
- compare private and center crests;
- award a shared pool;
- leak owner-private observations into public bot rationale;
- read static data as a behavior formula;
- add card/accounting nouns to `engine-core`.

## ADR Need

ADR required? no

Reason:

- This ledger records no architecture, replay/hash, data-policy, kernel,
  browser-authority, bot-policy, or public/private-content change. It keeps the
  behavior local and records the next review triggers.

## Review Checklist

- No third-use hard gate fires for Crest Ledger.
- Repeated card/private and public-accounting shapes are compared honestly.
- Bounded pledge/shared-pool allocation is recorded as first use.
- `engine-core` remains noun-free.
- `game-stdlib` receives no new helper.
- Static data remains typed content, parameters, metadata, fixtures, traces, and
  reports only.
- Golden traces are preserved.
- Visibility/no-leak evidence is covered by native, WASM, and browser checks.
- UI/effect and bot impacts are covered by local docs and tests.
- Atlas §10A remains empty.
