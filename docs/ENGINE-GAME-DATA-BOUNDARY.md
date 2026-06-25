# Rulepath Engine, Game, and Data Boundary

Status: boundary law for `engine-core`, `game-stdlib`, `games/*`, static data, and future language pressure. Supersede only by accepted ADR.

This document is the single source of truth for where behavior, vocabulary, data, and reusable mechanics belong.

## 1. Canonical boundary statement

Rulepath behavior is typed Rust. Static files provide typed content, parameters, presentation metadata, fixtures, traces, and variant selection. Static files do not define rule behavior in v1/v2.

Use these phrases:

- typed Rust game modules;
- typed static content;
- content-driven parameters;
- typed variant selection.

Do not use “data-driven rules” approvingly. In Rulepath, that phrase names the failure mode.

## 2. Layer responsibilities

| Layer | Allowed | Forbidden |
|---|---|---|
| `engine-core` | generic contracts: ids, versions, seeds, viewer/actor contracts, action trees, command envelopes, diagnostics, effect envelopes, replay/hash/checkpoints, visibility contracts, serialization boundaries | game nouns, mechanics, rule helpers, content schemas, renderer concerns, bot strategy, network services, persistence |
| `game-stdlib` | narrow typed helpers after primitive-pressure evidence | speculative universal mechanics, one-game abstractions, private-game pressure, hidden policy, data-language behavior |
| `games/*` Rust | state, setup, actions, validation, transitions, scoring, terminal detection, visibility, effects, variants, game-specific bots, UI metadata hooks | generic kernel policy, browser shell, accounts, hosted services |
| `games/*/data` | manifests, typed content, labels, typed variants, fixtures, traces, source notes, UI metadata, explanation templates | selectors, branches, triggers, loops, rule procedures, tactical AI conditions, hidden defaults, arbitrary expressions |
| `ai-core` | bot traits, random legal bot, deterministic tie-break helpers, policy-node utilities, instrumentation | game strategy, hidden-state peeking |
| `wasm-api` | thin batched bridge over Rust behavior | rules, hidden-state filtering by UI, renderer logic |
| `apps/web` | presentation, controls derived from Rust legal choices, replay UI, accessibility, responsive layout | legality, behavior, hidden-state authority, bot decisions |

## 2A. Reuse Lanes

Rulepath uses four explicit reuse lanes. The narrowest-layer-wins rule applies:
put the helper in the lowest layer that can own it without violating behavior
authority, vocabulary, visibility, determinism, or static-data boundaries.

| Lane | Home | Allowed | Forbidden |
|---|---|---|---|
| Kernel ergonomics | `engine-core` | Tiny helpers over allowed generic contract vocabulary such as ids, versions, action paths, command envelopes, visibility scopes, effect envelopes, replay/checkpoint/hash contracts, and serialization boundaries. | Mechanic nouns, rule helpers, game strategy, renderer policy, network policy, storage policy, or content schemas. |
| Mechanical scaffolding | `game-stdlib`, future dev-only `game-test-support`, `wasm-api`, or local code depending on scope | Behavior-free typed infrastructure governed by [ADR 0008](adr/0008-mechanical-scaffolding-governance.md) and [MECHANICAL-SCAFFOLDING-REGISTER.md](MECHANICAL-SCAFFOLDING-REGISTER.md): seat-ID parse/format, action-tree encoding, stable-byte helpers, effect-envelope construction, or evidence harnesses when they do not decide game behavior. | Deal/reveal/projection policy, betting, pot allocation, trick lifecycle, teams, graph semantics, accounting, reaction windows, scoring, terminal outcome, bot strategy, legality, or hidden-state policy. |
| Behavioral mechanics | `games/*` first, then `game-stdlib` only after mechanic-atlas approval | Typed game behavior and narrow promoted primitives after repeated official-game evidence, examples, anti-examples, tests, benchmarks, and required back-ports. | Speculative universal mechanics, unearned helpers, promotion debt without an accepted exception, or `engine-core` mechanic vocabulary. |
| Typed content | `games/*/data`, manifests, fixtures, traces, docs, templates | Inert typed content, parameters, labels, variants, fixtures, traces, evidence receipts, source notes, rules/help prose, and UI metadata. | Selectors, branches, triggers, loops, conditions, formulas, tactical AI logic, procedural mutation instructions, hidden defaults, or any rule behavior. |

If a proposed helper fits multiple lanes, choose the narrowest lawful home. If a
candidate needs game nouns or behavior policy, it is not kernel ergonomics. If it
decides rules, scoring, visibility, effects, or bot choices, it is not
mechanical scaffolding. If static data would control behavior, stop and require
an ADR before proceeding.

## 3. Generic contract vocabulary versus mechanic vocabulary

`engine-core` MAY use generic contract vocabulary:

| Allowed in `engine-core` | Reason |
|---|---|
| game id, match id, seat id, player id | identity/contract infrastructure |
| actor, viewer, visibility scope | public/private view contract |
| rules version, manifest/data version, schema version | replay and migration contract |
| seed, deterministic RNG contract | reproducible randomness |
| action tree, action path, command envelope | legal-choice and replay contract |
| diagnostic | viewer-safe error/explanation contract |
| effect envelope | semantic event transport without mechanic meaning |
| public/private view contract | visibility boundary |
| replay, checkpoint, hash | determinism and drift detection |
| serialization boundary | stable interchange and storage contract |

`engine-core` MUST NOT use mechanic/domain vocabulary:

| Forbidden in `engine-core` | Where it belongs first |
|---|---|
| board, grid, cell, coordinate, adjacency, line | `games/*`, then maybe `game-stdlib` after pressure |
| card, deck, pile, hand, suit, rank | `games/*`, then maybe `game-stdlib` after pressure |
| faction, scenario, role, territory, combat | `games/*` only until repeated pressure proves a narrow helper |
| trick, pot, auction, betting, drafting | `games/*`, maybe `game-stdlib` after repeated public games |
| resource, movement, capture, flip, promotion | `games/*`, maybe `game-stdlib` after ledger decision |

Ambiguous term decision rule:

> `engine-core` may know that an opaque game-defined payload exists and can be serialized, hashed, redacted, or routed. It may not inspect the payload to apply mechanic meaning.

Examples:

| Question | Decision |
|---|---|
| Can `engine-core` store an opaque game-defined effect payload? | Yes, if it treats it as data for transport/hash/redaction contracts. |
| Can `engine-core` define `CardMoved`? | No. “Card” and “move” are mechanic nouns. |
| Can `engine-core` define `VisibilityScope::PrivateToSeat`? | Yes. That is contract vocabulary. |
| Can `engine-core` define `BoardPosition` because many games have boards? | No. Use game-local types first; consider `game-stdlib` only after pressure. |
| Can `engine-core` know an action path has segments? | Yes. It must not know that a segment means “source square” or “card choice.” |

## 4. Typed Rust game module authoring

Every official game SHOULD define typed Rust models for:

- state;
- seats and player mapping;
- phases/turn model;
- typed action payloads or action-path decoding;
- validated commands;
- semantic effects;
- visibility projection;
- scoring and terminal outcome;
- typed variants;
- bot policy hooks;
- UI metadata hooks.

Verbose local game code is acceptable. A contaminated kernel is not.

Game rules should be readable as game rules. Do not hide core rule behavior behind generic factories unless the mechanic atlas has explicitly promoted a helper.

N-seat and larger-surface pressure starts game-local too. Seat-range validators,
graph topology, route networks, community-card evaluators, wall/deck shuffles,
partnerships, side-pot allocators, tile-meld validators, and similar surfaces
belong in `games/*` first. After repeated official use, `game-stdlib` may accept
narrow typed helpers only through the [MECHANIC-ATLAS.md](MECHANIC-ATLAS.md)
primitive-pressure process. None of these examples justify an `engine-core`
noun.

## 5. Allowed static data

Static data MAY include:

- manifests;
- display names and short descriptions;
- icon IDs and theme tokens;
- coordinate labels or help text for a specific game;
- original/public component IDs;
- deck/list composition for original/public games;
- setup constants;
- scoring tables;
- typed variant selection among compiled Rust variants;
- localization strings;
- explanation templates keyed to Rust effects/actions;
- authored player-facing rules/help text such as `games/<game_id>/docs/HOW-TO-PLAY.md`, when it is inert prose;
- UI metadata;
- fixtures;
- golden traces;
- replay summaries;
- benchmark fixtures and reports;
- source notes;
- rule coverage tables.

Static data MUST deserialize into typed Rust structures or strict viewer-safe browser schemas. Unknown fields MUST be rejected by default.

Authored player-facing rules/help text may include labels, glossary text, setup
summaries, and player-facing explanations. It MUST NOT include selectors,
conditions, triggers, action schemas, validation rules, scoring logic,
visibility filters, YAML front matter, or any DSL-like structure that could
become behavior authority. Runtime legality, effects, visibility, scoring, and
replay semantics remain Rust-owned.

Large map is not a DSL license. Topology data, route lists, seat labels,
component lists, and surface-size budgets may be typed content when they remain
inert inputs to Rust behavior. Conditions, triggers, formulas, selectors,
procedural mutation, legality, scoring, visibility, bot tactics, and exception
logic remain Rust-owned.

## 6. Forbidden static data

Static data MUST NOT include:

- selectors as strings;
- rule branches;
- loops;
- triggers;
- conditional effects;
- mandatory-action rules;
- exception logic;
- tactical AI conditions;
- procedural mutation instructions;
- arbitrary expressions;
- hidden default behavior;
- nested untyped objects interpreted as behavior;
- behavior encoded by naming convention;
- YAML behavior.

Behavior-looking field names are banned unless an ADR and typed lowering policy explicitly permits them. Suspicious names include:

`when`, `if`, `then`, `else`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, `ai_condition`, `effect_script`, `rule`, `requires`, `valid_if`, `on_play`, `on_reveal`.

A suspicious field must be moved into typed Rust behavior, converted into a typed enum selecting compiled Rust behavior, renamed as presentation/content, rejected, or escalated to ADR.

## 7. Typed static content pipeline

Every hand-authored static file consumed by Rust SHOULD follow this pipeline:

```text
file bytes
  -> approved parser
  -> typed schema deserialization
  -> unknown-field rejection
  -> behavior-looking-field scan
  -> semantic validation with source-located diagnostics
  -> version/hash recording
  -> read-only typed content structure
  -> Rust behavior consumes typed values
```

Validation SHOULD detect duplicate IDs, missing IDs, unknown references, invalid ranges, invalid localization keys, invalid UI metadata references, composition totals that fail invariants, private/proprietary IDs in public builds, and behavior-looking field names.

Diagnostics SHOULD name file, field path, offending value, and nearest known alternatives when practical.

## 8. Approved v1/v2 data formats

| Format | Use for | Do not use for |
|---|---|---|
| TOML | manifests, simple options, metadata, narrow typed variants | complex nested rules, selectors, procedures |
| JSON | browser payload fixtures, golden traces, replay summaries, machine reports | hand-authored rule behavior |
| RON | Rust-shaped fixtures, enum-heavy setup data, complex typed static content | untyped behavior or hidden DSLs |
| CSV | tabular card lists, scoring tables, rule coverage matrices, balance tables, benchmark exports | procedural effects or nested state machines |
| Postcard or equivalent compact Serde format | approved non-hand-authored snapshots/caches/internal artifacts | hand-authored rules or default public replay interchange |
| YAML | not default in v1/v2 | any behavior; any use without ADR |

The maintenance status of common Rust YAML tooling reinforces the YAML ban. The deeper reason is architectural: YAML too easily becomes an accidental untyped programming language.

## 9. Variant enum discipline

Static data MAY select among compiled Rust variants only when the variant maps to a documented typed Rust enum or equivalent typed value.

Allowed shape:

```text
variant = "misere"
```

This is acceptable only when Rust defines, documents, tests, versions, and hashes the `misere` behavior.

Forbidden shape:

```text
if_final_token_taken = "current_player_loses"
```

That data defines behavior and is forbidden.

Variant combinations MUST be enumerated, validated, covered in rules docs, included in data versioning, and represented in traces when replay-affecting.

## 10. Component and effect identity nuance

A static file MAY list an effect/component identity only when the identity maps to compiled Rust behavior.

Allowed shape:

```text
card_id = "draw_two"
effect_id = "DrawTwo"
```

This is acceptable only if Rust defines and tests what `DrawTwo` means.

Forbidden shape:

```text
effect:
  when: played
  select: current_player.deck.top(2)
  then: move selected to current_player.hand
```

That defines selectors and mutation behavior. It is forbidden.

Public static content IDs MUST NOT smuggle proprietary text or private module references into public builds.

## 11. UI metadata boundary

UI metadata MAY include labels, icon IDs, short help, layout hints, coordinate labels, visual tags, theme tokens, action grouping tags, accessibility labels, and explanation-template IDs.

UI metadata MUST NOT include legality, hidden behavior, tactical AI conditions, rule consequences not supplied by Rust previews/effects, hidden identities in public payloads, or any field whose meaning mutates game state.

The UI may group, sort, decorate, and animate Rust-provided legal choices. It MUST NOT invent legality.

Player-facing rules prose is allowed presentation content, not UI metadata that
can drive controls. The web app may render `HOW-TO-PLAY.md` text, but it MUST NOT
interpret it as action availability, scoring, visibility, effects, or bot logic.

## 12. Explanation template boundary

Explanation templates are presentation text. They are not source-of-truth rules.

Allowed shape:

```text
score_change = "{seat} gains {amount} point(s)."
```

Forbidden shape:

```text
when phase == reaction and actor.has(shield) then cancel_attack
```

Templates MUST be keyed to semantic actions/effects emitted by Rust. Interpolation MUST be viewer-safe and MUST NOT reveal hidden information.

## 13. `game-stdlib` promotion boundary

A helper belongs in `game-stdlib` only when:

- at least two implemented official games have been compared;
- a third official use is blocked or imminent;
- the repeated shape is narrow and typed;
- examples and anti-examples define the boundary;
- back-porting preserves traces or intentionally migrates them;
- tests and benchmarks prove the helper;
- no game noun enters `engine-core`;
- no static data language appears.

Promotion creates conformance work as well as shared code. When a helper enters `game-stdlib`, the atlas MUST name every official game already using the promoted mechanic shape and classify it as one of:

- migrated to the helper;
- to be migrated by a named closure gate;
- not applicable after audit;
- explicitly excepted with evidence.

An exception MUST name the game, the primitive, the reason for non-migration, the evidence that no generic primitive is being duplicated or forked, and the next review trigger. A same-gate deferral that leaves matching games local is promotion debt, not completion. Open promotion debt blocks further mechanic-ladder advancement unless an accepted ADR or exception says otherwise.

A helper with many game-specific flags is not ready. A helper invented for a future private monster game is forbidden.

## 13A. Forward mechanical-scaffolding conformance boundary

The behavioral `game-stdlib` promotion process in §13 remains unchanged.
Mechanical scaffolding follows the separate ADR 0008 lane and the mechanical-
scaffolding register.

Before serious implementation, every new official game MUST audit its planned
behavior-free infrastructure against existing `engine-core` contract ergonomics,
registered `game-stdlib` scaffolding, dev-only `game-test-support` harnesses, and
thin `wasm-api` adapters. A matching lawful helper is reused unless the register
records an accepted exception.

A new behavior-free shape MUST be registered even on first use. Its entry names
the narrowest lawful home, behavior exclusions, exact current site, affected
hash/visibility/determinism surfaces, acceptance evidence, and next review
trigger. First-use registration is inventory and boundary control; it is not
promotion authority.

When the new game creates or exposes matching scaffolding in earlier official
games, its closeout MUST either queue a named bounded refactoring unit or record
an accepted `local-only`, `deferred`, or `rejected` disposition with rationale
and next review trigger. A third copy remains blocked by ADR 0008's hard
decision rule.

Any candidate that owns deal/reveal/projection policy, betting or pot semantics,
trick lifecycle, teams, graph semantics, accounting, reaction windows, scoring,
terminal outcome, legality, strategy, or hidden-state policy is not mechanical
scaffolding. Reject it from this lane and route it to the game-local behavioral
implementation, mechanic atlas, or an ADR.

No conformance action may silently change replay bytes, hashes, fixture/export
authority, RNG output, serialization order, or viewer authorization. Such a
change requires the applicable ADR 0009 migration authority and explicit
compatibility evidence.

## 14. Future DSL policy

No DSL at project start.

A future DSL MAY be proposed only after multiple public Rust game modules show repeated, painful, stable behavior shapes that typed Rust plus narrow `game-stdlib` helpers cannot maintain cleanly.

A DSL ADR MUST include:

- problem cases from implemented games;
- rejected Rust/helper alternatives;
- grammar or typed schema;
- static typing model;
- deterministic lowering/compilation;
- source spans and diagnostics;
- formatter and linter plan;
- versioning and migration policy;
- tests and benchmarks;
- examples and anti-examples;
- replay/hash implications;
- visibility and hidden-default rules;
- agent-safety plan;
- public/private data policy.

A DSL MUST NOT be introduced to rescue one complex game or private experiment.

## 15. ADR triggers

ADR is required for:

- introducing YAML anywhere;
- introducing a new hand-authored data format;
- adding selectors, expressions, triggers, conditions, or rule-like data;
- changing unknown-field policy;
- broadening static variant behavior beyond typed documented enums;
- introducing a DSL;
- adding binary formats to public replay interchange;
- changing replay hash or data-version semantics;
- moving repeated mechanics into `game-stdlib` before the normal hard gate;
- changing `engine-core` vocabulary.
