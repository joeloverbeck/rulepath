# ADR: Sanctioned Parallel Private-Game Lane

Status: Accepted

Date: 2026-06-28

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `specs/private-lane-foundation-readiness.md`
- `specs/README.md`
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`

Required scaling / supersession fields:

- Affected foundation sections: `FOUNDATIONS.md` priority order, private-IP invariants/stop conditions, and ADR triggers; `ROADMAP.md` Gate P ordering; `IP-POLICY.md` private experiment timing.
- Superseded decision, if any: limits accepted ADR 0007's Gate-P tail for timing only; does not supersede ADR 0007's isolation, non-public, and non-architectural requirements.
- Hidden-information no-leak compatibility: no private content or hidden information may enter public source, public docs, public CI artifacts, public traces, public app bundles, public WASM/JS, browser payloads, DOM, logs, bot explanations, or replay exports.
- No-DSL/no-YAML compatibility: no new data language, DSL, YAML, selector table, or procedural static data is authorized.
- Evidence classification: doctrine/IP/roadmap only.
- Compatibility window: permanent until superseded by a later accepted ADR.
- Accepted exceptions: early private-lane timing only, after ADR-authorized readiness is complete.
- Effective only after named foundation updates land: yes; downstream PLP1-RDY tickets must update `FOUNDATIONS.md`, `ROADMAP.md`, `IP-POLICY.md`, `AGENT-DISCIPLINE.md`, and `specs/README.md`.
- Proposed-ADR review trigger / expiry: none.
- Rollback / contamination risk: if private content reaches public files, public bundles, public docs, public CI artifacts, public traces, public WASM/JS, or `engine-core`, stop and revert or isolate the contaminated surface before continuing.

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| `docs/FOUNDATIONS.md` | Add timing-only sanctioned private-lane carve-out and matching IP/invariant/stop/ADR-trigger text. | PLP1-RDY | Permanent | Doc review plus boundary/no-leak closeout gates. |
| `docs/ROADMAP.md` | Record Private Lane P1 beside the public ladder and note ADR 0007 is limited only for timing. | PLP1-RDY | Permanent | Roadmap/spec-index grep and doc-link check. |
| `docs/IP-POLICY.md` | Record sanctioned private-lane timing and public-tree no-leak obligations. | PLP1-RDY | Permanent | Manual public-tree leak scan and doc-link check. |
| `specs/README.md` | Track PLP1-RDY and later opaque private-lane rows without naming licensed content. | PLP1-RDY | Permanent | Spec-index grep and catalog-docs check. |

## Context

Rulepath's current public roadmap keeps Gate P as a private, optional, isolated,
non-architectural, and non-public tail item after the public scaling phase. That
tail placement is governed by accepted ADR 0007 and by the foundation priority
order, which puts later private stress tests behind polished public product,
correct deterministic rules, clean boundaries, and future multiplayer readiness.

The private-lane readiness spec authorizes a different timing shape: the first
sanctioned private licensed game may begin in parallel with the unfinished public
ladder after explicit doctrine, repository, CI, catalog, and no-contamination
rules are in place. That is a priority-order timing change, so `FOUNDATIONS.md`
requires an accepted ADR before the constitution, roadmap, or IP-policy edits
land.

This ADR is doctrine only. It does not create a private repository, implement a
game, add a public seam, change CI, add a catalog entry, or modify Rust/WASM/web
source.

## Decision

Create a sanctioned private-game lane that may run in parallel with the public
roadmap after explicit ADR approval. The lane permits private licensed
implementation work now, but only in private repositories/build artifacts. It
does not authorize private content in public source, public docs, public CI
artifacts, public traces, public app bundles, or `engine-core`. Public
architecture may gain only generic, private-free extension seams.

*Amends:* `FOUNDATIONS.md` priority order, private-IP invariants/stop
conditions, ADR triggers; `ROADMAP.md` Gate P ordering; `IP-POLICY.md` private
experiment timing. *Limits* (does not supersede) accepted ADR 0007's Gate-P
tail for timing.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Keep all private licensed work at the Gate P tail | Matches ADR 0007 exactly and minimizes doctrine changes. | Rejected for this sanctioned lane. Maintainers want a private lane to begin earlier, but only with explicit ADR-gated isolation and no public contamination. |
| Let private work happen in the public repository behind feature flags or optional dependencies | It would simplify local development and reuse public CI directly. | Rejected. Public files, workspace members, catalog constants, CI manifests, and bundles must not name or carry private licensed content. |
| Create a parallel private lane authorized by accepted ADR | It permits early timing while keeping private content in private repositories and allowing only generic, private-free public seams. | Accepted. It makes the timing exception explicit and auditable without weakening public-first product, boundary, determinism, no-leak, or IP rules. |

## Consequences

Positive consequences:

- Private Lane P1 can be planned and implemented earlier without pretending the
  public Gate P tail has disappeared.
- ADR 0007 remains accepted for its public-first and isolation intent.
- Public architecture remains protected: any public changes must be generic,
  private-free seams rather than private-game pressure.
- Public no-leak and IP review have a concrete stop condition for private-lane
  contamination.

Negative or risky consequences:

- The roadmap now has a timing carve-out that must be kept visibly subordinate
  to public product quality and clean boundaries.
- Agents may overread "parallel" as permission to put private content in public
  docs or CI; downstream foundation and IP-policy edits must make that a hard
  stop.
- Later public seam work needs careful review to prove it is private-free and
  generic.

Operational requirements:

- Update `FOUNDATIONS.md`, `ROADMAP.md`, `IP-POLICY.md`, `AGENT-DISCIPLINE.md`,
  and `specs/README.md` through the PLP1-RDY ticket series.
- Keep all private implementation, licensed rules content, fixtures, e2e names,
  catalog strings, and rendered assets in private repositories/build artifacts.
- Run the PLP1-RDY closeout doc-link, catalog-docs, boundary, and public-tree
  leak checks before marking the readiness spec complete.

## Determinism impact

No change. This ADR affects timing and doctrine only. It does not affect RNG,
iteration order, clocks, floating point, parallelism, serialization order,
replay, or hashes.

Later private work must prove deterministic setup, legal action generation,
state transitions, view projection, replay, hashes, fixtures, and benchmarks in
its own private implementation plan.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, trace format, migration rule, or golden trace
changes. No replay or fixture migration is authorized.

## Visibility impact

This ADR strengthens the visibility boundary for private work. Private licensed
content and private game identifiers must not reach public source, public docs,
public CI artifacts, public traces, public app bundles, public WASM/JS, browser
payloads, DOM, storage, logs, bot explanations, candidate rankings, or replay
exports.

The sanctioned lane may use private repositories/build artifacts for private
views and private tests. Any public seam must be private-free and viewer-safe.

## Data/Rust boundary impact

No new static data field, hand-authored format, expression, selector, behavior
ID, variant, or schema is introduced. Behavior remains in typed Rust under the
existing foundation rules.

This ADR does not authorize YAML, a DSL, rule-like static data, selectors,
conditions, triggers, formulas, procedural static content, or untyped private
behavior data in the public repository.

## `engine-core` contamination risk

This ADR adds no game noun, mechanic noun, strategy, renderer concern,
networking concern, storage concern, or private licensed content to
`engine-core`.

Private lane pressure must not shape `engine-core`. Any future public helper or
extension seam must remain generic, private-free, and justified through the
normal foundation, mechanic-atlas, mechanical-scaffolding, and ADR processes.

## `game-stdlib` / primitive-pressure impact

No helper is introduced or promoted by this ADR.

Private stress evidence does not silently force public `game-stdlib` promotion.
Any later reusable helper must be justified by public-safe evidence and the
mechanic-atlas or mechanical-scaffolding process, with private content removed
from the public proof surface.

## UI impact

No UI files, app contracts, renderer registrations, action trees, previews,
effect-log animation, accessibility behavior, reduced-motion handling, or dev
inspectors change.

Later private UI work must live in private build artifacts. TypeScript remains
presentation-only and must not decide legality, turn order, hidden state,
outcomes, or bot reasoning.

## Bot impact

No bot code, bot policy class, public bot level, candidate ranking, explanation
output, or benchmark changes.

Public v1/v2 still exclude MCTS, ISMCTS, Monte Carlo rollout/search bots, ML,
RL, and runtime LLM move selection. Any private bot deferral or private bot
policy must be recorded by later private-lane doctrine and implementation
artifacts.

## IP impact

This ADR is an IP-control decision. It permits earlier private timing only when
private licensed content remains in private repositories/build artifacts.

Public files must not contain the private licensed title, proprietary rules
prose, card text, private IDs, fixture or e2e names, screenshots, scans, art,
icons, fonts without redistribution rights, trade dress, catalog strings, or
private WASM/JS bundles. Human/legal review remains required before any public
release that could expose private material.

## Benchmark impact

No benchmarks, thresholds, harnesses, CI lanes, or benchmark policies change.
Later private work must define native and browser performance evidence in the
private implementation plan.

## Migration notes

Existing docs to update:

- `docs/FOUNDATIONS.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `docs/README.md`
- `specs/README.md`

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
