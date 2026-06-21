# Primitive Pressure Ledger: hidden trick hands and staged reveal

Candidate name: `plain-tricks-hidden-trick-hands`

Status: third-use hard-gate decision recorded; extraction deferred/rejected

Decision date: 2026-06-09

Last updated: 2026-06-21

Prepared by: `Codex`

## Hard Gate

This ledger records Gate 10.1's primitive-pressure decision for Plain Tricks.
The third-use hard gate fires here because `plain_tricks` repeats the
deterministic shuffle / private hand / staged reveal shape already used by
`high_card_duel` and `poker_lite`.

Decision: defer/reject extraction and keep the mechanics local.

No helper is added to `engine-core` or `game-stdlib`. No promotion debt is
created. GAT101PLATRI-003 is not required for the current decision unless a
maintainer later changes this ledger to a promote decision.

This decision is recorded before any `plain_tricks` shuffle, hand, deal,
visibility, replay, bot, or browser implementation code is written. The
repository-level record is [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md)
§10B. The second-use comparison to cross-reference, not duplicate, is
[../../poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md](../../poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md).

## Mechanic Shape

Mechanic shape:

```text
deterministic shuffle of a small opaque-id deck;
per-seat private holdings;
viewer-filtered deal or reveal effects;
hidden residue that is not public by default;
public replay/export redaction.
```

The repeated shape is real, but the behavior-bearing edges differ enough that a
shared helper would either be too small to matter or broad enough to encode game
policy.

## Games Exerting Pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `high_card_duel` | Gate 8 | `games/high_card_duel/src/setup.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | first deterministic private-card/reveal proof | implemented | Multi-round private hands, face-down commitments, simultaneous reveal, owner/private/public projection, redacted export. |
| `poker_lite` | Gate 10 poker_lite half | `games/poker_lite/src/setup.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | second deterministic private-card/staged-reveal pressure | implemented | Owner-private crests, hidden center, staged center reveal, grouped showdown reveal, yield terminal without private reveal, deck tail. |
| `plain_tricks` | Gate 10.1 | planned `games/plain_tricks/src/setup.rs`, `actions.rs`, `rules.rs`, `visibility.rs`, `effects.rs`, replay/export docs and tests | third deterministic private-hand/staged-reveal pressure | admitted, not implemented | Six-card hands, must-follow-suit legality from private hand contents, play-by-play reveal, hidden tail never revealed, two fresh round deals. |

## Local Implementations Compared

| Aspect | `high_card_duel` | `poker_lite` | planned `plain_tricks` | Same shape? | Notes |
|---|---|---|---|---:|---|
| deck construction | Local canonical deck of 24 cards. | Local canonical deck of six crests. | Local canonical deck of 18 cards. | partial | Stable opaque IDs repeat; deck identity and labels stay game-local. |
| shuffle | Local Fisher-Yates over game card IDs using `SeededRng` and unbiased bounded index. | Local Fisher-Yates over game crest IDs using `SeededRng` and unbiased bounded index. | Planned deterministic shuffle over trick card IDs; round 2 continues the same RNG stream. | yes | The small algorithm repeats, but extracting only this algorithm has low value and still creates back-port/hash risk. |
| deal shape | Alternating hands; hidden deck remainder. | Two private crests, one hidden center, hidden tail. | Six cards to each seat, six-card tail, repeated for two rounds. | partial | Counts, zones, round rotation, and exhaustion errors differ. |
| reveal model | Face-down commitments reveal together. | Center reveals after round 1; private crests reveal only at showdown; yield suppresses reveal. | Cards reveal one at a time when played; tail never reveals, including terminal. | no | Reveal timing is game policy. |
| legality coupling | Private card identity drives actor choices but not suit-following constraints. | Legal pledge choices are mostly public; hidden strength affects bot policy and showdown. | Follow-suit legality depends directly on the actor's private hand contents. | no | A helper must not know follow-suit or expose private alternatives. |
| hidden residue | Deck remainder hidden from browser export. | Deck tail hidden; center may become public; private crests may reveal at showdown. | Tail hidden forever; unplayed hand cards remain owner-only until played. | partial | Residue visibility and terminal treatment differ. |
| effects | Private deal/commit/reveal/outcome effects. | Private deal, staged center reveal, grouped showdown, allocation effects. | Private deal plus public card-play, trick, round, rotation, terminal effects. | partial | Effect schemas are game-owned. |
| replay/export | Internal traces may contain hidden state; public export is redacted. | Same taxonomy with yield/showdown-specific redaction. | Same taxonomy, but public export must not reconstruct hands or tail while preserving played-card history. | partial | Export policy repeats at a high level, not as a safe helper boundary. |
| bot use | Bot uses own legal action data only. | Level 2 uses own private rank and public center/ledger facts. | Level 2 may use own hand, legal tree, and public trick history only. | partial | Bot inputs and explanations stay game-local. |

## Similarities

- All three games use Rust-owned deterministic setup and viewer-safe projection.
- All three need redacted public export/import behavior and no-leak browser
  surfaces.
- All three keep hidden setup facts out of TypeScript legality, DOM text,
  `data-testid`, local storage, dev panels, public replay exports, bot
  explanations, and candidate rankings.
- `high_card_duel` and `poker_lite` already carry local evidence that small
  hidden-card games can remain game-local without kernel changes.

## Differences

- Holding size differs materially: `high_card_duel` commitments, one-card
  Crest Ledger private slots, and six-card Plain Tricks hands create different
  action tree and projection surfaces.
- Reveal models differ: simultaneous commitment reveal, staged center/showdown
  reveal, and play-by-play trick reveal with a never-revealed tail.
- Plain Tricks is the first of these games where legal action availability
  depends directly on suit distribution inside the actor's private hand.
- Plain Tricks has two round deals from one continuing RNG stream; the prior
  games perform one setup deal for the match.
- A common helper broad enough to cover reveal timing, private observations,
  action metadata, export redaction, and diagnostics would become a behavior
  language with flags for game policy.

## Extraction Decision

The recorded decision is defer/reject extraction and keep local.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for deterministic shuffle plus private holdings plus redacted projection/export |
| helper can stay narrow and typed? | not enough to justify promotion for this gate |
| helper belongs in `game-stdlib`? | no for Gate 10.1 |
| would contaminate `engine-core`? | yes if card/deck/hand/trick/reveal nouns moved there; therefore forbidden |
| static-data behavior risk? | medium if deal counts, reveal policy, or follow-suit behavior become configurable formulas; current plan keeps them Rust-local |
| replay/hash impact acceptable? | no extraction impact is justified; existing game traces/hashes should remain untouched |
| visibility/no-leak impact acceptable? | yes for local implementation; extraction would need a new proof surface |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no; benchmark pressure has not proven a shared hot path worth the boundary cost |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary changes are made |

Rationale:

- The only obviously identical piece is a small Fisher-Yates shuffle over
  game-local opaque IDs. Extracting it now would force prior-game conformance
  work and trace/hash review while leaving the risky parts, namely deal shape,
  reveal timing, action metadata, redaction, and diagnostics, local anyway.
- The actual third-use pressure is not "shuffle a vector"; it is hidden
  component lifecycle. That lifecycle differs enough that a helper would need
  policy hooks.
- Keeping the Plain Tricks implementation local preserves the current
  Rulepath boundary: typed game modules own mechanics; `game-stdlib` earns
  only narrow behavior-free helpers after the atlas proves a clean boundary.

## Accounting-Shape Decision

Decision: Plain Tricks trick-count scoring is outcome scoring, not a third use
of the public resource accounting / shared-ledger shape.

| Compared shape | `token_bazaar` | `poker_lite` | `plain_tricks` stance |
|---|---|---|---|
| public resources | token supplies, inventories, market costs, contract progress | per-seat contributions and shared pool | none; trick counts are points earned by resolved tricks |
| payment/allocation | exact payments, refills, contract scoring | exact pledge additions, yield award, showdown allocation, split | no payments, costs, pooled assets, or allocation |
| conservation/accounting invariant | public economy totals and market availability | contribution/pool totals and terminal distribution | total match points equal resolved tricks; this is scoring/outcome |
| helper pressure | repeated candidate after Gate 10 | repeated candidate after Gate 10 | no third economy/accounting use fires |

Plain Tricks still needs tests proving total points equal 12 and terminal
scoring is deterministic, but that is `scoring/outcome` pressure, not the
resource-accounting ledger pressure tracked for Token Bazaar and Crest Ledger.

## Rejected Alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse an existing promoted primitive | No promoted card/private-hand helper exists. | Decision option 1 does not apply. |
| Promote a behavior-free shuffle helper now | The repeated algorithm is small; extraction would create conformance and hash-review work without covering hidden lifecycle policy. | Reopen only if future games prove shuffle implementation divergence or benchmark pressure. |
| Promote a hidden-hand/reveal helper now | Reveal timing, legal-action coupling, diagnostics, and replay export policy differ too much. | A helper would likely need behavior flags. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |
| Treat trick points as public resource accounting | Trick points are outcome scoring, not spendable resources, market costs, pool allocation, or conservation ledger. | Resource-accounting third-use gate does not fire here. |

## API Sketch In Prose Only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/plain_tricks` |
| determinism requirements | Plain Tricks must use Rulepath deterministic RNG locally and prove replay stability |
| replay/hash requirements | no migration of `high_card_duel` or `poker_lite` |
| visibility requirements | hidden-hand and tail projection remain game-local and Rust-owned |
| effect/log requirements | effect names and payloads remain game-local |
| bot-facing notes | bots consume game-local safe inputs only |
| non-goals | generic cards, generic hands, generic trick-taking, generic reveal framework, TypeScript legality |
| good-fit examples | none until a later ledger proves a narrow behavior-free shape with worthwhile conformance payoff |
| anti-examples | decide follow-suit legality, reveal tail cards, expose opponent hand alternatives, resolve tricks, score rounds, encode deal policy in data |

## Determinism, Visibility, UI, Bot, And Benchmark Impact

| Area | Impact | Required safeguard/test |
|---|---|---|
| trace hashes | none for existing games; no extraction or migration | existing game replay checks remain unchanged; Plain Tricks adds its own replay evidence later |
| serialization | none now; Plain Tricks stable serialization remains local | `cargo test -p plain_tricks --test serialization` in later tickets |
| public view/action tree/effects | no shared helper; local viewer filtering required | visibility tests, WASM tests, browser no-leak smoke |
| replay export/import | no shared helper; local redaction required | golden traces, public export/import tests, browser replay import/export smoke |
| UI controls/action mapping | TypeScript still maps Rust action choices only | `npm --prefix apps/web run smoke:ui`, `smoke:e2e` later |
| bot policy | no shared helper; Level 2 remains authored and local | bot tests and evidence pack |
| benchmarks | no shared helper; benchmark evidence remains local | `cargo bench -p plain_tricks` later |

## Tests Required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | later `RULE-COVERAGE.md` ticket |
| golden trace preservation/update notes | yes if promoted | not applicable now; no migration |
| property/invariant tests | yes | later Plain Tricks property tests |
| replay/hash tests | yes | later Plain Tricks replay tests |
| serialization tests | yes | later Plain Tricks serialization tests |
| visibility/no-leak tests | yes | later native, WASM, and browser evidence |
| bot tests | yes | later Plain Tricks bot tests |
| benchmark tests | yes | later Plain Tricks benchmarks |

## Traces Affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `high_card_duel` | preserve | No shared helper or migration. | prior hidden-card pressure |
| existing golden traces | `poker_lite` | preserve | No shared helper or migration. | second-use hidden-card pressure |
| future `games/plain_tricks/tests/golden_traces/*.trace.json` | `plain_tricks` | create locally | Local implementation must prove its own deterministic setup, visibility, and replay/export behavior. | `PT-*` rules |

## Back-Port And Conformance Plan

No back-port is required because no helper is promoted.

Affected prior games:

- `high_card_duel`: no code or trace change.
- `poker_lite`: no code or trace change.

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
  policy, if several games need the same string-search or export-audit shape.

## Anti-Examples

Not a fit:

- decide follow-suit legality;
- decide which cards are dealt to which zone;
- reveal or hide cards by phase;
- expose action-tree card metadata to a non-actor;
- redact replay exports based on a generic hidden-zone policy language;
- resolve tricks or score rounds;
- infer opponent voids;
- read static data as deal, reveal, legality, or bot formulas;
- add card, deck, hand, suit, rank, trick, lead, follow, void, or deal nouns to
  `engine-core`.

## Agent Misuse Risks

| Risk | Guardrail |
|---|---|
| Copying shuffle code is mistaken for permission to generalize card mechanics. | This ledger approves no helper; card/trick behavior stays game-local. |
| A future agent encodes deal counts or follow-suit logic in data. | `RULES.md` and this ledger require Rust-owned behavior and strict data parsing. |
| Browser code filters legal card choices by suit. | TypeScript legality remains forbidden; non-actor action trees are empty. |
| Tail or opponent-hand identifiers leak through tests, DOM, exports, or bot rationale. | Later no-leak tests must sweep every named surface. |
| Conditional ticket GAT101PLATRI-003 is run despite a defer/reject decision. | It is not required unless the ledger decision changes to promote. |

## ADR Need

ADR required? no

Reason:

- This ledger records no architecture, replay/hash, data-policy, kernel,
  browser-authority, bot-policy, or public/private-content change. It keeps the
  behavior local and records the next review trigger.

## Next Review Trigger

Reopen this decision before a fourth official game repeats deterministic
shuffle plus private holdings plus redacted reveal/export, or sooner if
implementation uncovers one of these facts:

- local shuffle/deal code diverges in a way that causes replay/hash bugs;
- multiple games need the same behavior-free no-leak test helper;
- benchmark evidence shows repeated setup/projection cost that a narrow helper
  can improve without policy hooks;
- a maintainer proposes any runtime helper that would affect visibility,
  reveal timing, action metadata, replay export, or bot input.

## Gate 16 Trick-Taking Second-Use Addendum

Date: 2026-06-21

Briar Circuit records the second close use of Plain Tricks' trick-taking shapes
in [../../briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md](../../briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md).

The repeated shapes are follow-suit legality, led-suit trick comparison,
trick-winner-led turn order, deterministic trick-hand deal rotation, and
hidden-hand projection with played-card public reveal.

Decision: keep local / defer extraction. No Plain Tricks behavior, traces,
hashes, action paths, diagnostics, renderer, bot policy, or docs-generated
player rules change. No `engine-core` noun, no `game-stdlib` helper, and no
promotion debt are created. Gate 17 Oh Hell remains the third-use hard-gate
trigger for close trick-taking behavior.

Any reopen must again choose exactly one of reuse, promote, defer/reject, or
ADR before a new repeated implementation proceeds.

## Gate 17 Helper-Conformance Addendum

Date: 2026-06-21

Gate 17 Vow Tide created the third close use of follow-suit selection and
led-suit/trump trick comparison. The repository-level decision in
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) promotes the
narrow pure `game-stdlib::trick_taking` helper:

- `follow_suit_indices` selects stable held-card indices matching the led suit,
  or every held-card index when the hand is void in that suit.
- `winning_play_index` selects the stable winning play index from
  caller-projected suit/rank values. Plain Tricks uses it with `trump = None`.

Plain Tricks now adopts both helper functions. The conformance is intentionally
behavior-preserving:

- local `TrickCardId`, `TrickSuit`, `TrickRank`, seats, phases, diagnostics,
  action paths, effect order, scoring, visibility projection, bot policy, replay
  support, and UI surfaces remain Plain Tricks owned;
- the helper returns only indices, which Plain Tricks maps back to its existing
  cards and seats without reordering leaves or effects;
- trick-winner-led turn order and deal/redeal policy remain explicit
  anti-examples, not promoted helper behavior.

Verification receipt for this addendum is recorded in
`archive/tickets/GAT17VOWTIDOHHEL-003.md`. The intended proof is unchanged
Plain Tricks tests, replay-check hashes, and native benchmark execution after
the helper swap. No §10A promotion debt is opened because Gate 17 performs the
matching prior-game conformance in-gate.

## Review Checklist

- The third-use hard gate for deterministic shuffle / private hand / staged
  reveal is resolved before Plain Tricks rules implementation.
- The Gate 17 follow-suit/comparator hard gate is resolved by the promoted
  helper, and Plain Tricks adopts it without broadening helper scope.
- Public resource accounting is explicitly not triggered by trick-count
  scoring.
- `engine-core` remains noun-free.
- `game-stdlib` receives only the narrow pure trick-taking helper; no card,
  hand, trick phase, winner-leads, deal, score, visibility, or bot helper is
  admitted.
- Static data remains typed content, parameters, metadata, fixtures, traces, and
  reports only.
- Existing `high_card_duel` and `poker_lite` traces are preserved.
- Atlas §10A remains empty.
