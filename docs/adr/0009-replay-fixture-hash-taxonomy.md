# ADR: Replay, Fixture, Export, And Hash Taxonomy v2

Status: Accepted

Status note: accepted on 2026-06-22 as part of the maintainer-requested
PREGAT18REUDOC pre-Gate-18 realignment. This ADR is decision-only; no trace,
fixture, export, hash, RNG, serialization, or benchmark byte changes in this
pass.

Date: 2026-06-22

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/TRACE-SCHEMA-v1.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/WASM-CLIENT-BOUNDARY.md`
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md`
- `archive/specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md`

Required scaling / supersession fields:

- Affected foundation sections: `FOUNDATIONS.md` §11 and §13;
  `TESTING-REPLAY-BENCHMARKING.md` trace/replay/hash sections;
  `TRACE-SCHEMA-v1.md`; `WASM-CLIENT-BOUNDARY.md` replay/export sections.
- Superseded decision, if any: none for ADR 0004. This ADR preserves and
  strengthens ADR 0004's hidden-information internal-trace vs viewer-scoped
  export taxonomy. It narrows the future role of Trace Schema v1 to legacy
  command/replay evidence rather than all evidence fixtures.
- Hidden-information no-leak compatibility: preserved and strengthened. Browser
  exports remain viewer-scoped; public exports must not contain seat-private
  facts; seat-private exports must contain only facts authorized for that seat at
  each timeline step.
- No-DSL/no-YAML compatibility: artifact classes are evidence contracts only.
  They must not carry selectors, conditions, triggers, formulas, procedural
  mutation instructions, or rule behavior.
- Evidence classification: doctrine / trace / fixture / export / hash.
- Compatibility window: existing artifacts remain valid under their current
  validators until a successor migration unit explicitly changes a class,
  version, hash surface, or canonical-byte authority.
- Accepted exceptions: none.
- Effective only after named foundation updates land: yes for downstream docs.
  The taxonomy is accepted now, but `TRACE-SCHEMA-v1.md`,
  `TESTING-REPLAY-BENCHMARKING.md`, `WASM-CLIENT-BOUNDARY.md`, and the new
  evidence-fixture contract must be updated before future tickets rely on the
  new profile names as documentation law.
- Proposed-ADR review trigger / expiry: not applicable after acceptance. Reopen
  if a future migration needs to change canonical bytes or weaken ADR 0004.
- Rollback / contamination risk: rollback by keeping Trace Schema v1 as the sole
  documented evidence shape and requiring per-game migration notes. Do not mix
  public exports with internal full traces or let setup/domain fixtures become
  rule data.

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| Trace Schema v1 docs | Narrow to legacy command/replay evidence or mark superseded by profile | Pre-Gate-18 realignment | No byte changes in this pass | Link check + no trace diff |
| Evidence fixture contract | Define profile fields, validators, visibility classes, and version anchors | Pre-Gate-18 realignment | Before Part C fixture migration | Link check + manual review |
| Part C fixture/hash work | Migrate only explicitly named artifacts under profile/version rules | Successor unit | After this spec is Done | Fixture/replay/no-leak/hash checks |

## Context

Trace Schema v1 began as Gate 2's canonical JSON evidence shape for golden
traces and replay fixtures. Later gates added hidden-information games,
viewer-scoped replay exports, setup fixtures, domain evidence fixtures,
N-seat no-leak matrices, benchmark reports, and public/private export proofs.
Those artifacts now have different visibility, validator, byte-stability, and
hash-authority needs, but the docs still tend to speak as though every
`*.trace.json`-adjacent artifact belongs to one schema.

ADR 0004 already created the essential hidden-information split:

- internal full traces are deterministic native/dev evidence and may contain
  seed, full command stream, private action choices, checkpoints, and hashes;
- viewer-scoped replay exports are browser-safe observation timelines and must
  not contain unauthorized hidden information.

ADR 0009 keeps that split and generalizes the evidence taxonomy so fixture and
hash migrations after pre-Gate-18 have named target profiles. It does not change
any existing artifact bytes.

FOUNDATIONS §13 requires an ADR for replay/hash semantics and public/private
visibility contract changes. FOUNDATIONS §11 requires replay, hashes, RNG,
serialization order, and traces to remain deterministic unless explicitly
migrated.

## Decision

Rulepath adopts Replay / Fixture / Export / Hash Taxonomy v2.

The taxonomy defines these artifact classes:

| Class | Profile id | Purpose | Default visibility | Validator owner |
|---|---|---|---|---|
| Internal command/replay trace | `replay-command-v1` | Native replay/checkpoint/hash evidence for behavior authority | dev/test internal; may be omniscient when explicitly marked | `fixture-check` + `replay-check` |
| Public replay export | `public-export-v1` | Browser-safe public observer observation timeline | public observer only | WASM/export importer + no-leak smoke |
| Seat-private replay export | `seat-private-export-v1` | Browser-safe observation timeline for one authorized seat | exactly one labelled seat viewer | WASM/export importer + pairwise no-leak |
| Setup evidence fixture | `setup-evidence-v1` | Deterministic setup/options/seat/variant evidence without command replay authority | public unless marked test-only and viewer-scoped | fixture/static-data validators |
| Domain evidence fixture | `domain-evidence-v1` | Game-local edge-case evidence such as evaluator, allocator, scoring, topology, or no-leak matrix inputs | class-specific; must declare public, seat-private, or test-only internal | game-local validator or fixture-check extension |

The taxonomy defines these visibility classes:

- `public`: safe for public browser payloads, docs, logs, and replay exports.
- `viewer-scoped`: safe only for the named viewer at the named timeline step.
- `seat-private`: safe only for the named seat; pairwise no-leak proof required
  for hidden-information games.
- `internal-dev`: may contain omniscient or private evidence; must not be the
  default browser export or public replay payload.
- `private-source`: may contain non-public or licensed stress-test content; must
  not enter public files, public CI, public docs, public traces, public bundles,
  or public WASM/JS.

Each evidence artifact must identify:

- artifact class / profile id;
- visibility class;
- validator owner;
- schema/profile version;
- game id and rules/data/hash-surface versions where applicable;
- canonical-byte authority, or an explicit statement that no canonical byte
  comparison is defined;
- migration/update note when expected surfaces intentionally change;
- not-applicable rationale for omitted hidden-information, replay, or hash
  surfaces.

Canonical-byte authority:

- `replay-command-v1` canonical bytes are owned by the native Rust replay and
  fixture validators. They may include state/effect/action-tree/public-view and
  seat-private-view hashes as declared by the game/spec.
- `public-export-v1` and `seat-private-export-v1` canonical bytes are owned by
  Rust/WASM export serialization for the requested viewer scope. Import replays
  an observation timeline, not omniscient state.
- `setup-evidence-v1` and `domain-evidence-v1` define canonical bytes only when
  their validator says so. Otherwise they are structured evidence inputs whose
  semantic checks are owned by the named validator.

Hash-surface versioning:

- Hash surfaces must name the surface: `state`, `effect`, `action-tree`,
  `public-view`, `seat-private-view`, `public-export`, `seat-private-export`, or
  a game-local domain surface.
- Changing the meaning, field order, visibility set, redaction rule, or
  canonical bytes of a hash surface requires a migration note and, when
  foundation-level, an accepted ADR.
- File suffixes and filenames are non-authoritative. The profile id and
  validator contract decide the class.

Trace Schema v1 disposition:

- Trace Schema v1 remains valid for existing command/replay evidence until a
  named migration changes it.
- Downstream docs should narrow Trace Schema v1 away from "all replay fixtures"
  and toward `replay-command-v1` / legacy command-replay evidence.
- Setup and domain evidence fixtures should move under the new evidence-fixture
  contract rather than pretending to be Trace Schema v1 command traces.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Keep Trace Schema v1 as the single schema for all evidence | Avoids new taxonomy. | Rejected. It conflates command replay, browser exports, setup fixtures, and domain evidence with different visibility and validators. |
| Replace ADR 0004 with one larger replay/export ADR | Could centralize all replay law. | Rejected. ADR 0004 is correct and accepted; this ADR preserves and extends it by named relationship. |
| Migrate all trace/fixture bytes immediately | Would make docs and data line up at once. | Rejected. The spec is docs/doctrine only; byte/hash migration belongs in the successor unit. |
| Adopt taxonomy v2 without byte changes | Separates decision authority from migration work. | Accepted. It gives future tickets named profiles while preserving current evidence. |

## Consequences

Positive consequences:

- Future fixture/hash migration has explicit target profiles.
- Browser replay exports remain separate from internal replay authority.
- Setup and domain evidence can be validated without pretending to be command
  streams.
- Hash-surface changes become easier to identify and review.

Negative or risky consequences:

- Existing docs must be narrowed carefully so readers do not assume old suffixes
  imply the new class.
- Validators may need follow-up support to enforce profile ids. That is deferred
  to the successor unit.
- The taxonomy introduces more terms. Mitigation: each artifact declares its
  class, visibility, validator, and version.

Operational requirements:

- Author `docs/EVIDENCE-FIXTURE-CONTRACT.md` from this ADR.
- Narrow `docs/TRACE-SCHEMA-v1.md` and update testing law without changing
  existing artifact bytes.
- Update WASM/export docs to classify public and seat-private exports.
- Defer all trace, fixture, hash, RNG, serialization, and benchmark-byte
  migrations to successor tickets.

## Determinism impact

This ADR changes no RNG, iteration order, clocks, floating point, parallelism,
serialization order, replay, hash, trace, fixture, or export bytes.

The decision strengthens deterministic evidence by requiring every profile to
name canonical-byte authority or state that no canonical byte comparison is
defined.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, export hashes, trace format bytes, or migration
rules change in this pass.

Future changes to hash meaning, field order, visibility set, redaction rule, or
canonical bytes must name the affected surface version and carry migration
evidence.

## Visibility impact

The decision preserves and strengthens ADR 0004.

Public exports remain public-observer-safe by default. Seat-private exports must
be explicitly labelled by viewer seat and must contain only that seat's
authorized observations at each step. Internal-dev evidence may be omniscient
only when it is kept out of public/browser surfaces. Private-source evidence may
not enter public files or CI.

## Data/Rust boundary impact

No new rule data, DSL, YAML, selector, condition, trigger, formula, or behavior
field is introduced.

Evidence artifacts are typed evidence, not rule behavior. Rust remains behavior
authority for setup, legality, transitions, projection, effects, replay, and
export derivation.

## `engine-core` contamination risk

No code changes. No game noun or mechanic noun is added to `engine-core`.

The taxonomy uses generic contract vocabulary: artifact, profile, validator,
version, visibility, replay, export, checkpoint, and hash.

## `game-stdlib` / primitive-pressure impact

No `game-stdlib` helper is introduced or promoted.

This ADR does not alter behavioral mechanic pressure. Any future helper for
fixture validation or evidence construction must route through ADR 0008's
mechanical-scaffolding lane or the mechanic atlas, depending on whether it is
behavior-free.

## UI impact

No UI files or browser schemas change in this ADR.

The browser continues to receive only Rust/WASM-provided viewer-safe exports.
TypeScript remains presentation-only and must not reconstruct hidden state from
export/import artifacts.

## Bot impact

No bot policy or legal action API changes.

Public exports and evidence fixtures must not include private bot candidates,
hidden-state-derived explanations, or candidate rankings unless the artifact is
internal-dev and kept out of public/browser surfaces.

## IP impact

No public content changes.

The `private-source` visibility class makes explicit that private licensed or
stress-test evidence must not enter public files, public CI, public docs, public
traces, public bundles, or public WASM/JS.

## Benchmark impact

No benchmark thresholds, harnesses, reports, or CI lanes change.

Benchmark reports may later cite profile ids when they measure a named evidence
or export surface, but that is not implemented here.

## Migration notes

Existing docs to update:

- `docs/EVIDENCE-FIXTURE-CONTRACT.md`
- `docs/TRACE-SCHEMA-v1.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/WASM-CLIENT-BOUNDARY.md`

Existing games to back-port:

- None in this ADR.

Existing traces to preserve or update:

- All existing trace and fixture bytes are preserved in this pass.

Existing data/schema versions to bump:

- None.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before using this ADR for a migration or new evidence profile, verify:

- the artifact declares its class, visibility, validator, and version;
- public/browser exports are viewer-safe and no-leak by default;
- internal-dev evidence cannot be exported as a public browser payload;
- Trace Schema v1 command/replay evidence remains valid or has an explicit
  migration note;
- canonical-byte authority is named, or no canonical-byte comparison is defined;
- hash-surface version changes are explicit;
- Rust remains behavior authority;
- TypeScript does not decide legality or reconstruct hidden state;
- `engine-core` remains noun-free;
- static data remains evidence/content, not rule behavior;
- IP/public-private boundaries are preserved.
