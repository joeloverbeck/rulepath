# ADR: Mechanical Scaffolding Governance

Status: Accepted

Status note: accepted on 2026-06-22 as part of the maintainer-requested
PREGAT18REUDOC pre-Gate-18 realignment. The downstream foundation and area-doc
updates named here are required before this lane is used for production code.

Date: 2026-06-22

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/AGENT-DISCIPLINE.md`
- `archive/specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md`

Required scaling / supersession fields:

- Affected foundation sections: `FOUNDATIONS.md` §4, §11, §12, §13;
  `ENGINE-GAME-DATA-BOUNDARY.md` §13; `MECHANIC-ATLAS.md` §§4-8;
  `ARCHITECTURE.md` ownership matrix; `UI-INTERACTION.md` §10A.
- Superseded decision, if any: none. This ADR adds a separate scaffolding lane;
  it does not supersede the behavioral mechanic third-use gate.
- Hidden-information no-leak compatibility: the lane may only cover helpers whose
  APIs are visibility-neutral or viewer-safe by construction; no helper may
  expose hidden facts, bypass game-owned projection, or create a new browser,
  trace, bot-explanation, candidate-ranking, DOM, storage, log, or replay-export
  leak path.
- No-DSL/no-YAML compatibility: mechanical scaffolding is typed Rust
  infrastructure only. It may not introduce selectors, rule branches,
  conditions, triggers, formulas, scripts, YAML, or a DSL.
- Evidence classification: doctrine / code-governance. This ADR authorizes a
  future code lane; it changes no code, trace, fixture, hash, RNG, benchmark, or
  serialization byte.
- Compatibility window: effective for new mechanical-scaffolding decisions only
  after the named foundation/area-doc updates and scaffolding register land in
  this pre-Gate-18 series.
- Accepted exceptions: none.
- Effective only after named foundation updates land: yes. The required updates
  are `FOUNDATIONS.md`, `ARCHITECTURE.md`,
  `ENGINE-GAME-DATA-BOUNDARY.md`, `MECHANIC-ATLAS.md`,
  `UI-INTERACTION.md`, `AGENT-DISCIPLINE.md`, and the new
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
- Proposed-ADR review trigger / expiry: not applicable after acceptance. If the
  named foundation updates do not land in this series, reopen before using the
  lane.
- Rollback / contamination risk: rollback by treating all repeated plumbing as
  local code again. Do not move mechanic nouns, game behavior, hidden-state
  policy, static-rule data, renderer policy, or private-game pressure into
  `engine-core` or `game-stdlib` under this ADR.

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| Foundation docs | Add the scaffolding lane and invariants named above | Pre-Gate-18 realignment | Before any code extraction uses the lane | Link check + boundary check |
| Scaffolding register | Create `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` with decision entries | Pre-Gate-18 realignment | Before any helper promotion | Register review + link check |
| Part C code extraction | Evaluate candidate helpers against this ADR and the register | Successor unit | After this spec is Done | Dedicated tickets, tests, no-leak/replay/hash proof |

## Context

Rulepath has shipped 17 games. The behavioral reuse doctrine has held up: typed
game behavior starts local, repeated behavioral mechanics are reviewed by the
mechanic atlas, and a third official use hard-gates further game admission until
the atlas records reuse, narrow promotion, deferral, rejection, or ADR
escalation.

The 17-game corpus also exposed repeated non-behavioral plumbing around generic
contracts: effect-envelope construction, seat-ID parse/format, seat-count and
ring arithmetic, action-tree encoding, stable-byte writing, and dev-only
evidence harnesses. Those examples are not themselves card, board, trick,
betting, scoring, reveal, reaction, or partnership behavior. Treating them as
behavioral mechanic pressure overloads the mechanic atlas; ignoring them leaves
duplicated contract plumbing in every new gate.

This ADR creates a separate, narrower lane for mechanical scaffolding:
behavior-free typed infrastructure that supports existing generic contracts
without deciding game rules. It is not an engine-generalization permission slip.
The lane is lawful only because it is accepted before the foundation text
changes that will reference it.

## Decision

Rulepath adopts a mechanical-scaffolding reuse lane for typed, behavior-free
infrastructure that repeatedly supports generic Rulepath contracts.

Mechanical scaffolding is code or test infrastructure that:

- works over already-allowed generic vocabulary such as effect envelopes, seat
  IDs, actor/viewer IDs, action trees, command envelopes, visibility scopes,
  replay/hash bytes, serialization boundaries, or benchmark/evidence records;
- does not know or encode mechanic nouns, game-local state, strategy,
  scoring, legality, phase policy, hidden-state semantics, renderer policy, or
  private licensed content;
- is deterministic, leak-safe, and schema/version explicit where relevant;
- is reusable because the duplicated shape is semantically identical, not merely
  textually similar;
- has a migration set that can be completed or explicitly rejected without
  leaving parallel incompatible copies.

Allowed homes are narrow:

- `engine-core` may receive contract ergonomics only when the API uses allowed
  kernel vocabulary and remains meaning-opaque with respect to game mechanics.
- `game-stdlib` may receive behavior-free scaffolding tied to game-layer typed
  inputs only when it does not compete with or bypass the behavioral mechanic
  atlas.
- a future dev-only `game-test-support` crate may receive test and evidence
  harnesses that production crates do not depend on.
- `wasm-api` may receive adapters that translate Rust-owned safe payloads for
  the browser without deciding legality, visibility, or rules.

The behavioral third-use gate remains word-for-word effective:

> third official use: hard gate. The game MUST NOT proceed until the primitive-pressure ledger decides reuse, narrow promotion, explicit deferral/rejection with rationale, or ADR escalation.

A scaffolding candidate is explicitly excluded from this lane if it controls,
configures, or interprets any of the following behavior: deal/reveal/projection
policy, betting or pot allocation, trick lifecycle, teams or partnerships,
graph/accounting semantics, reaction windows, scoring, terminal outcome, bot
strategy, rule legality, effect meaning, or UI presentation policy.

Decision rule:

1. At a second exact duplication of a behavior-free scaffolding shape, reviewers
   must either keep it local with rationale, record it in the scaffolding
   register, or propose a narrow extraction.
2. Before a third copy is introduced, the register must decide reuse, promotion,
   explicit deferral/rejection, or ADR escalation.
3. Second-use promotion is allowed only when semantic identity is proven, the API
   is noun-free or correctly game-layer typed, behavior-neutral, deterministic,
   leak-safe, and the migration set is complete.
4. A promoted scaffolding helper must include tests, examples or anti-examples
   where useful, and no-leak/replay/hash evidence when it touches those
   surfaces.
5. Any candidate that drifts into behavior returns to the mechanic-atlas process
   or a separate ADR.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Keep all repeated scaffolding local forever | Preserves the strictest possible local-first posture. | Rejected. It leaves exact duplicated contract plumbing in every gate and obscures real behavior-vs-plumbing review. |
| Treat scaffolding as normal mechanic-atlas pressure | Reuses an existing process. | Rejected. The atlas is for behavioral mechanics; using it for behavior-free envelope/seat/action-tree plumbing makes the third-use gate noisy. |
| Promote scaffolding freely into `engine-core` or `game-stdlib` | Would reduce duplication quickly. | Rejected. It risks kernel contamination and speculative helpers. This ADR requires evidence, exclusions, register entries, and narrow homes. |
| Adopt a separate mechanical-scaffolding lane | It distinguishes behavior from contract plumbing while preserving the behavioral hard gate. | Accepted. It solves the real duplication pressure without lowering Rulepath's mechanic-boundary law. |

## Consequences

Positive consequences:

- Repeated contract plumbing has a lawful review path.
- The mechanic atlas remains focused on behavioral mechanics.
- Future code extraction can be scoped by evidence, exclusions, and migration
  completeness instead of broad "generalize the engine" language.
- The register can track rejected helpers and non-promotion examples before they
  become architecture drift.

Negative or risky consequences:

- The category can be abused if reviewers label behavior as plumbing. Mitigation:
  the non-promotion list, no-leak/determinism requirements, and boundary checks
  are explicit.
- Some helpers may sit between game-local typed inputs and generic contracts.
  Mitigation: narrowest-layer-wins; if the API needs game nouns, it is not
  `engine-core` scaffolding.
- Register maintenance becomes required. Mitigation: downstream tickets add the
  register and closeout workflow.

Operational requirements:

- Add the scaffolding lane to the named foundation and area docs.
- Create `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
- Add an agent/task protocol for scaffold-refactor work.
- Do not implement production code extraction until the downstream docs land and
  the successor unit scopes each helper.

## Determinism impact

This ADR changes no RNG, iteration order, clocks, floating point, parallelism,
serialization order, replay, or hashes.

Any future scaffolding helper that writes stable bytes, orders action-tree
segments, parses IDs, constructs effects, or supports replay/evidence fixtures
must prove deterministic output and name any hash-surface impact before adoption.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, trace format, or migration rules change in this ADR.

Future helpers that touch replay/hash surfaces must name the affected hash
surface, preserve existing bytes by default, or follow ADR 0009 and an explicit
migration ticket.

## Visibility impact

No visibility contract changes in this ADR.

Future scaffolding must preserve the no-leak firewall: private facts must not
reach unauthorized payloads, DOM, local storage, logs, previews, diagnostics,
effect logs, bot explanations, candidate rankings, replay exports, traces, or
tests. Visibility helpers may only operate on already viewer-safe inputs or on
generic visibility scopes without inspecting game-private meaning.

## Data/Rust boundary impact

No static data field, hand-authored format, expression, selector, behavior ID,
variant, or schema is introduced. The lane is typed Rust infrastructure only.

Mechanical-scaffolding evidence records are documentation and test receipts, not
rule data. They must not become procedural configuration.

## `engine-core` contamination risk

This ADR does not add code to `engine-core`.

`engine-core` remains a generic contract kernel. It may own ergonomic helpers
for allowed contract vocabulary only when those helpers remain meaning-opaque.
It must not gain game nouns, mechanic nouns, strategy, renderer concerns,
networking, storage, private content, or game-specific rule policy.

## `game-stdlib` / primitive-pressure impact

This ADR does not promote any helper to `game-stdlib`.

Behavioral mechanics remain governed by the mechanic atlas. Mechanical
scaffolding may enter `game-stdlib` only when it is behavior-free and does not
preempt the atlas. If a helper decides legality, scoring, reveal, trick,
betting, team, graph, action policy, or other behavior, it is not scaffolding.

## UI impact

No UI files change.

This ADR supersedes the UI helper count trigger only after downstream docs land:
presentation-helper promotion must be reviewed by semantic scaffolding pressure,
not by a raw official-game count. TypeScript remains presentation-only and may
not infer legality, hidden state, turn order, or outcomes.

## Bot impact

No bot code or policy changes.

Scaffolding may not give bots hidden information, bypass the legal action API,
alter candidate ranking policy, or introduce MCTS, ISMCTS, Monte Carlo bots, ML,
RL, or runtime LLM move selection.

## IP impact

No public naming, rules prose, assets, fonts, fixtures, traces, private content,
or browser-shipped bundles change.

Private licensed experiments may not use this lane to shape public architecture
or introduce public files.

## Benchmark impact

No benchmark threshold, harness, CI lane, or benchmark policy changes.

Future scaffolding helpers on hot paths must carry benchmark evidence when the
surface is performance-relevant.

## Migration notes

Existing docs to update:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/UI-INTERACTION.md`
- `docs/AGENT-DISCIPLINE.md`
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

Existing games to back-port:

- None in this ADR. Successor tickets may name per-game migration sets.

Existing traces to preserve or update:

- None.

Existing data/schema versions to bump:

- None.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before using this ADR for a scaffolding decision, verify:

- the helper is behavior-free and uses allowed generic vocabulary or correctly
  game-layer typed inputs;
- the behavioral third-use gate remains effective;
- Rust remains behavior authority;
- TypeScript does not decide legality;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters, not behavior;
- replay determinism and hash surfaces are preserved or explicitly migrated;
- visibility boundaries remain safe;
- bots remain fair, legal-API-bound, and non-omniscient;
- benchmarks exist for hot paths;
- IP/public-private boundaries are preserved;
- the scaffolding register has the candidate, duplicate sites, exclusions,
  migration set, and verification evidence.
