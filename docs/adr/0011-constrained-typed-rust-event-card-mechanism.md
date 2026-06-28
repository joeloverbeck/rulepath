# ADR: Constrained Typed Rust Event-Card Mechanism

Status: Accepted

Date: 2026-06-28

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/AGENT-DISCIPLINE.md`
- `archive/specs/private-lane-foundation-readiness.md`

Required scaling / supersession fields:

- Affected foundation sections: `FOUNDATIONS.md` static-data/no-DSL section; `ENGINE-GAME-DATA-BOUNDARY.md` typed-content/behavior line; `MECHANIC-ATLAS.md` private event-pressure notes.
- Superseded decision, if any: none; this ADR narrows a sanctioned private pattern without replacing the public no-DSL rule.
- Hidden-information no-leak compatibility: event-card behavior must be Rust-owned and viewer-filtered; private card identities, text, and state remain private-lane content and may not enter public artifacts.
- No-DSL/no-YAML compatibility: no YAML, script, untyped JSON/TOML/RON effect rows, selectors, formulas, or declarative behavior language is authorized.
- Evidence classification: doctrine/data-boundary/IP only.
- Compatibility window: permanent until superseded by a later accepted ADR.
- Accepted exceptions: a game-local private typed Rust event-card mechanism; no public helper promotion without public-safe evidence.
- Effective only after named foundation updates land: yes; downstream PLP1-RDY tickets must update `FOUNDATIONS.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, and `MECHANIC-ATLAS.md`.
- Proposed-ADR review trigger / expiry: any proposal to move event-card helpers into public `game-stdlib`, public static data, `engine-core`, YAML, or a DSL requires fresh review.
- Rollback / contamination risk: if event-card behavior appears in public static data, a declarative language, or `engine-core`, stop and move it back to typed Rust game-local code or require a new ADR.

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| `docs/FOUNDATIONS.md` | Record the accepted typed Rust event-card exception while preserving no-DSL/no-YAML law. | PLP1-RDY | Permanent | Foundation grep and doc-link check. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Add typed-registry guidance distinguishing inert event content from Rust behavior. | PLP1-RDY | Permanent | Boundary-doc grep and doc-link check. |
| `docs/MECHANIC-ATLAS.md` | Record private event pressure as game-local/private unless public-safe evidence earns helper review. | PLP1-RDY | Permanent | Atlas grep and doc-link check. |

## Context

Private Lane P1 needs event-card behavior at a larger scale than current public
games. Rulepath's foundation law already permits typed static content such as
component IDs, deck/list composition, inert display metadata, setup constants,
fixtures, traces, and source notes. The same law forbids static data from
defining rule behavior through selectors, conditions, triggers, procedural
mutation, formulas, hidden defaults, YAML, or a DSL.

The private lane needs a pattern that can represent event-card identity and
inert card metadata without opening a behavior-in-data path. Because the pattern
touches the rule-like-data boundary and could otherwise be mistaken for a
declarative card-effect language, the private-lane readiness spec requires an
accepted ADR before downstream boundary-doc and template updates land.

This ADR is doctrine only. It does not add a private game crate, event registry,
effect trait, source file, fixture, trace, hash migration, or public helper.

## Decision

Authorize a game-local typed Rust event-card mechanism. Card identity, deck
order, inert display metadata, and non-behavioral parameters may be typed
static content in the private crate. Every condition, selector, trigger, rule
override, target choice, legality check, state transition, visibility decision,
diagnostic, and semantic effect is implemented as Rust behavior through
explicit functions/match arms/traits. No YAML, script, untyped JSON/TOML effect
rows, or declarative behavior language is allowed.

*Amends:* `FOUNDATIONS.md` static-data/no-DSL section; `ENGINE-GAME-DATA-BOUNDARY.md`
typed-content/behavior line; `MECHANIC-ATLAS.md` private event-pressure notes.
Mechanism is game-local/private until public-safe evidence justifies any public
helper.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Encode event effects as YAML or untyped data rows | It can look concise for many cards. | Rejected. It creates behavior in static data, crosses the no-DSL/no-YAML boundary, and is hard for agents to keep deterministic, typed, and viewer-safe. |
| Put a generic event-card engine in `engine-core` | It would centralize event dispatch. | Rejected. `engine-core` must remain noun-free and cannot know cards, decks, selectors, triggers, target choices, or effect meanings. |
| Promote a public `game-stdlib` event helper now | It might eventually be reusable. | Rejected for now. Private pressure alone does not earn public helper promotion; public-safe repeated evidence and the mechanic atlas are required. |
| Use a game-local private typed Rust registry with explicit Rust behavior | It keeps inert content typed while making every rule decision compiled Rust. | Accepted. It preserves behavior authority, determinism, visibility review, and no-DSL law. |

## Consequences

Positive consequences:

- Private event-card implementation can scale without inventing a DSL.
- Card identity and inert metadata remain typed content, while behavior remains
  readable Rust.
- `engine-core` remains protected from card/event vocabulary.
- Public helper promotion remains evidence-gated rather than private-pressure
  driven.

Negative or risky consequences:

- Private game code may be more verbose than a declarative event table.
- Reviewers must keep checking that parameters remain non-behavioral and that
  behavior is not smuggled through IDs, names, or table structure.
- A future public helper proposal will need fresh public-safe evidence rather
  than relying on private implementation pressure.

Operational requirements:

- Downstream PLP1-RDY docs and templates must distinguish inert card content
  from Rust-owned effect behavior.
- Private event implementations must use typed Rust functions, match arms,
  traits, or equivalent compiled Rust constructs for every condition, selector,
  trigger, target choice, legality check, transition, visibility decision,
  diagnostic, and semantic effect.
- Unknown fields and behavior-looking static fields must remain rejected by
  default.

## Determinism impact

No direct change. This ADR adds no code, RNG, iteration order, clocks, floating
point, parallelism, serialization order, replay, or hash behavior.

The authorized private pattern is deterministic by design only when later
private implementation keeps deck order, random sampling, event resolution,
visibility, effects, and serialization in Rust-owned deterministic code with
explicit tests.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, trace format, migration rule, or golden trace
changes. No replay or fixture migration is authorized.

Later private event-card implementation must define its replay/hash evidence in
the private spec and must not change public trace schemas or hashes through this
ADR.

## Visibility impact

This ADR does not expose, filter, store, log, serialize, preview, animate,
explain, or rank private information.

Later private event-card behavior must make every reveal, redaction, diagnostic,
semantic effect, bot explanation, and replay/export projection Rust-owned and
viewer-safe. Private card identities, text, and hidden state must not leak into
public artifacts.

## Data/Rust boundary impact

This ADR permits only typed inert static content for card identity, deck order,
display metadata, and non-behavioral parameters inside the private crate. It
does not permit static data to decide behavior.

Every condition, selector, trigger, rule override, target choice, legality
check, state transition, visibility decision, diagnostic, and semantic effect
must be implemented as Rust behavior. YAML, scripts, untyped JSON/TOML/RON
effect rows, declarative behavior languages, formulas, procedural mutation
instructions, hidden defaults, and naming-convention behavior remain forbidden.

Unknown fields remain rejected by default. Suspicious behavior-looking fields
must be moved into typed Rust behavior, renamed as inert content, rejected, or
escalated through a later ADR.

## `engine-core` contamination risk

This ADR adds no card, deck, event, trigger, operation, selector, target,
faction, or private licensed vocabulary to `engine-core`.

The mechanism belongs first in the private game crate because it is game-local
behavior. It cannot live in `engine-core`, and it cannot enter public
`game-stdlib` until public-safe repeated evidence and the mechanic-atlas process
justify a narrow helper.

## `game-stdlib` / primitive-pressure impact

No helper is introduced or promoted by this ADR.

Private event-card pressure is evidence for private implementation, not a public
primitive-pressure shortcut. A future `game-stdlib` helper would require
public-safe examples, anti-examples, tests, benchmarks, back-port/conformance
analysis, and a mechanic-atlas decision.

## UI impact

No UI files, action trees, previews, effect-log animation, renderer boundaries,
accessibility behavior, reduced-motion handling, or dev inspectors change.

Later UI may render Rust/WASM-provided event metadata and semantic effects, but
TypeScript must not compute event legality, target validity, reveal policy,
state transitions, diagnostics, outcomes, or bot reasoning.

## Bot impact

No bot code, bot policy class, candidate ranking, explanation output, hidden
information handling, public bot level, or benchmark changes.

Later private bots must use the normal legal action API, allowed views only, and
Rust-owned event diagnostics/explanations. Publisher flowcharts or priority
charts must not be copied into bot policy unless private-lane IP review permits
them inside private artifacts.

## IP impact

No public IP surface changes. This ADR does not authorize any private licensed
title, event name, card text, rules prose, artwork, fixture, e2e name, catalog
string, or rendered asset in the public repository.

Private event-card content and source notes belong in the private repository and
private build artifacts only.

## Benchmark impact

No benchmarks, thresholds, harnesses, CI lanes, or benchmark policies change.

Later private event-card implementation must benchmark event fanout, target
choice, visibility projection, replay/export, and browser payload size where
the private spec requires it.

## Migration notes

Existing docs to update:

- `docs/FOUNDATIONS.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/MECHANIC-ATLAS.md`
- relevant PLP1-RDY templates

Existing games to back-port:

- None.

Existing traces to preserve or update:

- Preserve all existing traces; no trace update is authorized.

Existing data/schema versions to bump:

- None.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research;
- Rust remains behavior authority;
- TypeScript does not decide legality;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters, not behavior;
- replay determinism is preserved or migration is explicit;
- visibility boundaries remain safe;
- bots remain fair and explainable;
- benchmarks exist for hot paths or are not applicable to this doctrine-only change;
- IP/public-private boundaries are preserved;
- affected foundation docs and per-game docs are updated by downstream PLP1-RDY tickets.
