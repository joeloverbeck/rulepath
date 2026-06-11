# Primitive Pressure Ledger: masked claims hidden masks and reaction window

Candidate name: `masked-claims-hidden-masks-reaction-window`

Status: fourth-use hard-gate decision recorded; extraction deferred/rejected

Decision date: 2026-06-11

Last updated: 2026-06-11

Prepared by: `Codex`

## Hard Gate

This ledger records Gate 11's primitive-pressure decision for Masked Claims.
The deterministic shuffle / private hand / staged reveal decision reopens here
because `masked_claims` is the fourth official game to repeat the shape already
used by `high_card_duel`, `poker_lite`, and `plain_tricks`.

Decision: defer/reject extraction and keep the mechanics local.

No helper is added to `engine-core` or `game-stdlib`. No promotion debt is
created, so `docs/MECHANIC-ATLAS.md` §10A remains empty.

This decision is recorded before any `masked_claims` shuffle, deal, visibility,
replay, bot, or browser implementation code is written. The repository-level
record is [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md)
§10B. The prior hard-gate record is
[../../plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md](../../plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md).

This ledger also records the first official local use of the
reaction-window/pending-response shape. It is not a promotion decision and does
not authorize a generic reaction-window helper.

## Mechanic Shape

Repeated hidden-component shape:

```text
deterministic shuffle of small opaque component IDs;
per-seat private holdings;
viewer-filtered deal, placement, reveal, and export surfaces;
hidden residue that remains redacted by default;
public replay/export redaction.
```

First-use reaction-window shape:

```text
a game-local phase opened by a public claim;
one responder receives constrained legal choices;
the claimant receives an empty tree plus safe waiting metadata;
resolution is conditional on response choice and hidden information.
```

Both shapes are real. The hidden-component lifecycle still differs enough that
a runtime helper would either be trivial shuffle-only code or behavior-bearing
visibility/reveal/export policy. The reaction-window shape is a first official
local use and is explicitly not eligible for promotion.

## Games Exerting Pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `high_card_duel` | Gate 8 | `games/high_card_duel/src/setup.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | first deterministic private-card/reveal proof | implemented | Multi-round private hands, face-down commitments, simultaneous reveal, owner/private/public projection, redacted export. |
| `poker_lite` | Gate 10 poker_lite half | `games/poker_lite/src/setup.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | second deterministic private-card/staged-reveal pressure | implemented | Owner-private crests, hidden center crest, staged center reveal, grouped showdown reveal, yield without private reveal, deck tail. |
| `plain_tricks` | Gate 10.1 | `games/plain_tricks/src/setup.rs`, `actions.rs`, `rules.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | third deterministic private-hand/staged-reveal hard gate | implemented | Six-card hands, must-follow-suit legality from private hand contents, play-by-play reveal, hidden tail never revealed, two fresh round deals. |
| `masked_claims` | Gate 11 | planned `games/masked_claims/src/setup.rs`, `actions.rs`, `rules.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | fourth deterministic hidden-component/staged-reveal reopen; first reaction-window use | admitted, not implemented | Five-mask hands, hidden reserve, claim pedestal, accept keeps identity hidden forever, challenge reveals exactly one mask, response window constrains legal actions. |

## Local Implementations Compared

| Aspect | `high_card_duel` | `poker_lite` | `plain_tricks` | planned `masked_claims` | Same shape? | Notes |
|---|---|---|---|---|---:|---|
| component construction | Local canonical card IDs. | Local canonical crest IDs. | Local canonical trick card IDs. | Local canonical mask IDs and grade labels. | partial | Stable opaque IDs repeat; component identity and labels stay game-local. |
| shuffle | Local Fisher-Yates over game card IDs using `SeededRng`. | Local Fisher-Yates over game crest IDs using `SeededRng`. | Local Fisher-Yates over trick card IDs, including a second round from the continuing RNG stream. | Planned deterministic shuffle over fifteen mask IDs. | yes | The small algorithm repeats, but extraction still provides low value and creates conformance/hash review. |
| deal shape | Private hands and hidden deck remainder. | Private crests, hidden center crest, hidden tail. | Six-card hands and hidden tail each round. | Five-mask hands and five-mask reserve. | partial | Counts, zones, and terminal hidden residue differ. |
| reveal model | Face-down commitments reveal together. | Center and showdown reveals are staged; yield suppresses private reveal. | Cards reveal one at a time when played; tail never reveals. | Accepted masks never reveal; challenged masks reveal one at a time. | no | Reveal timing and no-reveal lifetime are game policy. |
| legality coupling | Private card identity drives actor choices. | Hidden strength affects showdown and bot policy. | Follow-suit legality depends on private hand contents. | Claim leaves depend on own hand; response legality depends on the public pending window, not hidden identity. | partial | A helper must not expose private alternatives or decide action policy. |
| pending response | none | pledge/yield response flow local to betting half | none | accept/challenge reaction window after every claim | no | Reaction-window shape is first official use and stays local. |
| hidden residue | Deck remainder hidden from browser export. | Tail hidden; private crests may reveal at showdown unless yielded. | Tail and unplayed hand cards remain hidden forever. | Reserve, accepted masks, and unplayed hand masks remain hidden forever. | partial | Lifetime redaction obligations differ by game. |
| effects | Private deal/commit/reveal/outcome effects. | Private deal, staged center reveal, grouped showdown, allocation effects. | Private deal plus public card-play, trick, round, rotation, terminal effects. | Claim placed, reaction window opened, accepted, challenge declared, mask revealed, challenge resolved, terminal effects. | partial | Effect schemas are game-owned. |
| replay/export | Internal traces may contain hidden state; public export is redacted. | Same taxonomy with yield/showdown-specific redaction. | Public export preserves played cards while redacting hands and tail. | Public export redacts claim tile IDs to declared grades and never exposes accepted masks, hands, or reserve. | partial | Export policy repeats at a high level, not as a safe helper boundary. |
| bot use | Bot uses own legal action data only. | Authored bot uses own private rank plus public ledger facts. | Authored bot uses own hand, legal tree, and public trick history. | Level 1 bot claims from own hand and responds from own view plus public counts only. | partial | Bot inputs and explanations stay game-local. |

## Similarities

- All four games use Rust-owned deterministic setup and viewer-safe projection.
- All four need redacted public export/import behavior and no-leak browser
  surfaces.
- All four keep hidden setup facts out of TypeScript legality, DOM text,
  `data-testid`, local storage, dev panels, public replay exports, bot
  explanations, and candidate rankings.
- All four use game-local component IDs and game-local effect/view schemas.

## Differences

- Holding size and zone topology differ: commitments, crest slots, trick hands,
  mask hands, hidden center, tails, reserves, pedestal, veiled galleries, and
  exposed rows are not one shared shape.
- Reveal models differ materially: simultaneous reveal, staged center/showdown
  reveal, play-by-play reveal, and conditional accept/challenge reveal with
  never-revealed accepted masks.
- Masked Claims adds a response-window phase whose legal action shape is not
  part of the prior hidden-hand games.
- Masked Claims requires command-summary redaction of internal claim tile IDs
  while still preserving declared grades and public response reasons.
- A common helper broad enough to cover reveal timing, private observations,
  action metadata, export redaction, response windows, and diagnostics would
  become a behavior language with flags for game policy.

## Extraction Decision

Decision: defer/reject extraction and keep local.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for deterministic shuffle plus private holdings plus redacted projection/export |
| helper can stay narrow and typed? | not enough to justify promotion for this gate |
| helper belongs in `game-stdlib`? | no for Gate 11 |
| would contaminate `engine-core`? | yes if mask/card/deck/hand/claim/reaction/reveal nouns moved there; therefore forbidden |
| static-data behavior risk? | medium if deal counts, reveal policy, reaction legality, scoring, or bot thresholds become configurable behavior; current plan keeps them Rust-local |
| replay/hash impact acceptable? | no extraction impact is justified; existing game traces/hashes should remain untouched |
| visibility/no-leak impact acceptable? | yes for local implementation; extraction would need a new proof surface |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no; benchmark pressure has not proven a shared hot path worth the boundary cost |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary changes are made |

Rationale:

- The only obviously identical runtime piece remains a small deterministic
  shuffle over game-local opaque IDs. Extracting it now would force conformance
  and trace/hash review while leaving deal shape, reveal timing, action
  metadata, response windows, redaction, diagnostics, and bot inputs local.
- Masked Claims makes the hidden lifecycle less generic, not more generic:
  accepted masks never reveal, challenged masks reveal exactly once, claim
  paths need public redaction to declared grade, and responder legality is a
  game-local pending phase.
- Keeping the implementation local preserves the Rulepath boundary: typed game
  modules own mechanics; `game-stdlib` earns only narrow behavior-free helpers
  after the atlas proves a clean boundary.

## Reaction-Window Decision

Decision: first official local use; keep local and mark `ADR-required` if
generalized broadly.

| Compared shape | `masked_claims` stance |
|---|---|
| pending response | A claim opens one timeout-free accept/challenge window. |
| active legal tree | The responder receives exactly accept/challenge. |
| waiting seat | The claimant receives an empty gameplay tree and safe waiting metadata. |
| resolution | Response choice controls accept or challenge resolution; challenge may reveal hidden information. |
| helper pressure | first official use only; no promotion allowed. |

The next reaction-capable official game should compare its response-window
shape against this one. Broad priority systems, interrupt stacks, cancellation
chains, timeout policy, hosted networking, or cross-game reaction engines
require ADR review before any promotion.

## Simultaneous Commitment Review

Decision: Masked Claims is not a second official use of the simultaneous
commitment/reveal + visible draft-pool removal shape.

| Compared shape | `secret_draft` | `masked_claims` stance |
|---|---|---|
| commitment timing | Multiple seats commit hidden choices before synchronized reveal. | One claimant places one hidden mask sequentially. |
| pending facts | Public pending booleans for seats. | Public pending responder and declared grade for one claim. |
| reveal batch | Synchronized reveal and conflict fallback. | Only a challenge reveals, and only one mask. |
| visible pool removal | Draft pool changes visibly after reveal. | No draft pool exists. |
| helper pressure | first official use after Gate 9.1. | no second use; row remains local-only candidate. |

## Rejected Alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse an existing promoted primitive | No promoted hidden-component/reveal helper exists. | Decision option 1 does not apply. |
| Promote a behavior-free shuffle helper now | The repeated algorithm is small; extraction would create conformance and hash-review work without covering hidden lifecycle policy. | Reopen only if future games prove shuffle implementation divergence or benchmark pressure. |
| Promote a hidden-component lifecycle helper now | Reveal timing, accepted-never-revealed policy, command-summary redaction, diagnostics, and replay export policy differ too much. | A helper would likely need behavior flags. |
| Promote a reaction-window helper now | Masked Claims is the first official reaction-window use. | First use must stay local. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## API Sketch In Prose Only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/masked_claims` |
| determinism requirements | Masked Claims must use Rulepath deterministic RNG locally and prove replay stability |
| replay/hash requirements | no migration of `high_card_duel`, `poker_lite`, or `plain_tricks` |
| visibility requirements | hidden masks, accepted galleries, hands, and reserve projection remain game-local and Rust-owned |
| effect/log requirements | effect names and payloads remain game-local |
| bot-facing notes | bots consume game-local safe inputs only |
| non-goals | generic cards, masks, hands, claims, challenges, reaction windows, reveal framework, TypeScript legality |
| good-fit examples | none until a later ledger proves a narrow behavior-free shape with worthwhile conformance payoff |
| anti-examples | decide claim legality, choose responder, reveal accepted masks, expose opponent hand alternatives, redact replay exports via policy language, resolve challenges, score claims, encode deal/reveal/reaction policy in data |

## Determinism, Visibility, UI, Bot, And Benchmark Impact

| Area | Impact | Required safeguard/test |
|---|---|---|
| trace hashes | none for existing games; no extraction or migration | existing game replay checks remain unchanged; Masked Claims adds its own replay evidence later |
| serialization | none now; Masked Claims stable serialization remains local | `cargo test -p masked_claims --test serialization` in later tickets |
| public view/action tree/effects | no shared helper; local viewer filtering required | visibility tests, WASM tests, browser no-leak smoke |
| replay export/import | no shared helper; local redaction required | golden traces, public export/import tests, browser replay import/export smoke |
| UI controls/action mapping | TypeScript still maps Rust action choices only | `npm --prefix apps/web run smoke:ui`, `smoke:e2e` later |
| bot policy | no shared helper; Level 1 remains rule-informed and local | bot tests and evidence pack |
| benchmarks | no shared helper; benchmark evidence remains local | `cargo bench -p masked_claims` later |

## Tests Required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | later `RULE-COVERAGE.md` ticket |
| golden trace preservation/update notes | yes if promoted | not applicable now; no migration |
| property/invariant tests | yes | later Masked Claims property tests |
| replay/hash tests | yes | later Masked Claims replay tests |
| serialization tests | yes | later Masked Claims serialization tests |
| visibility/no-leak tests | yes | later native, WASM, and browser evidence |
| bot tests | yes | later Masked Claims bot tests |
| benchmark tests | yes | later Masked Claims benchmarks |

## Traces Affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `high_card_duel` | preserve | No shared helper or migration. | prior hidden-card pressure |
| existing golden traces | `poker_lite` | preserve | No shared helper or migration. | second-use hidden-card pressure |
| existing golden traces | `plain_tricks` | preserve | No shared helper or migration. | third-use hidden-hand pressure |
| future `games/masked_claims/tests/golden_traces/*.trace.json` | `masked_claims` | create locally | Local implementation must prove deterministic setup, reaction windows, visibility, and replay/export behavior. | `MC-*` rules |

## Back-Port And Conformance Plan

No back-port is required because no helper is promoted.

Affected prior games:

- `high_card_duel`: no code or trace change.
- `poker_lite`: no code or trace change.
- `plain_tricks`: no code or trace change.

Exceptions:

- None. This is a defer/reject decision, not a promoted primitive with
  exceptions.

Closure gate if debt is deferred:

- Not applicable. No promotion debt is created, so `docs/MECHANIC-ATLAS.md`
  §10A remains empty.

## Examples

Good fits for future review only:

- a behavior-free shuffle utility if several games demonstrate divergent local
  shuffle bugs or benchmark pressure while preserving identical replay/hash
  semantics;
- a behavior-free redaction assertion helper used only in tests, not runtime
  policy, if several games need the same string-search or export-audit shape;
- a narrow response-window test helper if later games repeat empty-tree waiting
  proof without sharing runtime policy.

## Anti-Examples

Not a fit:

- decide which masks are dealt to which zone;
- choose who may respond;
- decide accept/challenge legality;
- reveal or hide masks by phase;
- expose action-tree mask metadata to a non-actor;
- redact replay exports based on a generic hidden-zone policy language;
- resolve challenges or score claims;
- infer opponent holdings;
- read static data as deal, reveal, legality, response, or bot formulas;
- add card, deck, hand, mask, grade, claim, challenge, reaction, response
  window, pedestal, gallery, reveal, or deal nouns to `engine-core`.

## Agent Misuse Risks

| Risk | Guardrail |
|---|---|
| Copying shuffle code is mistaken for permission to generalize hidden components. | This ledger approves no helper; hidden lifecycle behavior stays game-local. |
| A future agent encodes deal counts, reveal policy, or response legality in data. | `RULES.md` and this ledger require Rust-owned behavior and strict data parsing. |
| Browser code filters accept/challenge availability or computes scoring. | TypeScript legality and scoring remain forbidden; action trees and terminal rationale come from Rust. |
| Accepted mask, hand, or reserve identifiers leak through tests, DOM, exports, or bot rationale. | Later no-leak tests must sweep every named surface. |
| Reaction-window first use is mistaken for permission to add a generic pending-response engine. | This ledger records first local use only and preserves ADR-required posture for broad generalization. |

## ADR Need

ADR required? no

Reason:

- This ledger records no architecture, replay/hash, data-policy, kernel,
  browser-authority, bot-policy, or public/private-content change. It keeps the
  behavior local and records the next review triggers.

## Next Review Trigger

Reopen the deterministic shuffle / private hand / staged reveal decision before
a fifth official game repeats deterministic shuffle plus private holdings plus
redacted reveal/export, or sooner if implementation uncovers one of these facts:

- local shuffle/deal code diverges in a way that causes replay/hash bugs;
- multiple games need the same behavior-free no-leak test helper;
- benchmark evidence shows repeated setup/projection cost that a narrow helper
  can improve without policy hooks;
- a maintainer proposes any runtime helper that would affect visibility,
  reveal timing, action metadata, replay export, or bot input.

Reopen the reaction-window row when a second reaction-capable official game
appears. A broad reaction-window abstraction, priority system, interrupt stack,
networked timeout policy, or cancellation/replacement chain requires ADR before
promotion.

Any reopen must again choose exactly one of reuse, promote, defer/reject, or
ADR before a new repeated implementation proceeds.

## Review Checklist

- The fourth-use hard gate for deterministic shuffle / private hand / staged
  reveal is resolved before Masked Claims implementation.
- The recorded hidden-component decision is exactly one option: defer/reject
  extraction and keep local.
- Reaction window/pending response is recorded as first official local use and
  not promoted.
- The Stage 11 simultaneous-commitment review is recorded and does not count
  Masked Claims as a second use.
- `engine-core` remains noun-free.
- `game-stdlib` receives no new helper.
- Static data remains typed content, parameters, metadata, fixtures, traces, and
  reports only.
- Existing `high_card_duel`, `poker_lite`, and `plain_tricks` traces are
  preserved.
- Atlas §10A remains empty.
