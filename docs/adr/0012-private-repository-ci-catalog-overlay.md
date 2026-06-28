# ADR: Private Repository, CI Federation, and Catalog Overlay

Status: Accepted

Date: 2026-06-28

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/IP-POLICY.md`
- `docs/WASM-CLIENT-BOUNDARY.md`
- `docs/ARCHITECTURE.md`
- `docs/AGENT-DISCIPLINE.md`
- `apps/web/README.md`
- `specs/private-lane-foundation-readiness.md`

Required scaling / supersession fields:

- Affected foundation sections: `FOUNDATIONS.md` private-architecture trigger; `IP-POLICY.md` private build/repo rules; `WASM-CLIENT-BOUNDARY.md` catalog boundary; `ARCHITECTURE.md` overlay shape.
- Superseded decision, if any: none; this ADR defines the default sanctioned private-lane repository/build shape.
- Hidden-information no-leak compatibility: private games, fixtures, e2e, renderer overlays, private catalog entries, private WASM/web builds, and private CI artifacts remain private.
- No-DSL/no-YAML compatibility: no new data language, DSL, YAML, or rule-like static data is authorized.
- Evidence classification: doctrine/IP/CI/catalog seam plan only.
- Compatibility window: permanent until superseded by a later accepted ADR.
- Accepted exceptions: private games may live in a separate private repository that pins a public Rulepath commit and owns private build artifacts.
- Effective only after named foundation updates land: yes; downstream PLP1-RDY tickets must update `FOUNDATIONS.md`, `IP-POLICY.md`, `WASM-CLIENT-BOUNDARY.md`, `ARCHITECTURE.md`, `apps/web/README.md`, and the private release template.
- Proposed-ADR review trigger / expiry: any proposal to put private games in public workspace members, public catalog constants, public CI manifests, public submodules, public optional dependencies, or public bundles requires fresh ADR review.
- Rollback / contamination risk: if a public file, CI artifact, catalog surface, bundle, or WASM/JS artifact names or carries private content, stop and remove the contaminated public surface before continuing.

Migration matrix:

| Adopter / surface | Required change | Owner | Compatibility window | Verification |
|---|---|---|---|---|
| `docs/IP-POLICY.md` | Record the default separate-private-repo decision and public-tree exclusion rules. | PLP1-RDY | Permanent | IP-policy grep plus leak scan. |
| `docs/ARCHITECTURE.md` | Document the private overlay lane and public seam-plan constraints. | PLP1-RDY | Permanent | Architecture grep plus doc-link check. |
| `docs/WASM-CLIENT-BOUNDARY.md` | Document public catalog plus private overlay semantics without changing ABI. | PLP1-RDY | Permanent | Boundary grep plus doc-link check. |
| `apps/web/README.md` | Record private renderer overlay as doctrine, not a public catalog row. | PLP1-RDY | Permanent | Catalog-docs check. |
| CI/catalog seam implementations | Seed forward only; no implementation in PLP1-RDY. | Later unit | None in this readiness unit | No `.rs`/`.mjs`/`.yml`/`.toml` diff for this ticket. |

## Context

Private games must not contaminate public architecture, public source, public
catalogs, public CI, public docs, public traces, public bundles, or
`engine-core`. The private-lane readiness spec therefore needs an accepted ADR
that chooses a default repository and build architecture before Part C doctrine
and seam plans are written.

The safest default is a separate private repository that pins a public Rulepath
commit. That repository can own private game crates, docs, fixtures, renderer
overlays, e2e tests, private CI manifests, and private WASM/web builds. The
public repository may later expose generic, private-free extension seams, but it
must not name or bundle private licensed content.

This ADR is doctrine only. It does not implement catalog adapters, renderer
registries, reusable workflow inputs, drift checks, private repository
creation, or any public code/CI refactor.

## Decision

Default private games to a separate private repository that pins the public
Rulepath commit and owns private game crates, docs, fixtures, renderer overlay,
e2e, private CI manifests, and private WASM/web build. Public repo changes may
add only generic extension seams and reusable-workflow inputs. Public catalog
contains only public games. Private catalog entries appear only in private
build artifacts. A public submodule/feature/optional dependency that names
private games is rejected as the default.

*Amends:* `FOUNDATIONS.md` private-architecture trigger; `IP-POLICY.md` private
build/repo rules; `WASM-CLIENT-BOUNDARY.md` catalog boundary; `ARCHITECTURE.md`
overlay shape. **Records doctrine + documented seam plans only — no public
code/CI change in the readiness unit** (the catalog/renderer/CI/drift seam
implementations are seeded forward).

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Put private games in the public workspace behind feature flags | It would be convenient for builds and reuse. | Rejected. Public workspace members, feature names, manifests, lockfiles, and bundles could leak private identifiers or licensed content. |
| Add private games as public submodules or default optional dependencies | It preserves one public repository shape while separating source trees. | Rejected as the default. Public repository metadata would still name private games and pull private pressure into public workflows. |
| Keep private games only in local untracked folders | It avoids public leakage. | Rejected as a default because it weakens reproducibility, CI, review, and pinned-public-commit discipline. |
| Use a separate private repository pinned to a public Rulepath commit | It maximizes public/private separation while allowing private CI and build artifacts to compose with public Rulepath code. | Accepted. Public changes remain generic, private-free seams only. |

## Consequences

Positive consequences:

- Private licensed content stays out of public source, public docs, public CI,
  public catalog constants, public traces, public bundles, and public WASM/JS.
- Private builds can still pin and test against a known public Rulepath commit.
- Public seam work remains reviewable as generic architecture rather than
  private-game-specific code.
- Public catalog checks can continue treating the public catalog as public-only.

Negative or risky consequences:

- Private development requires repository synchronization and drift checks.
- Reusable workflow and catalog/renderer seam implementations are deferred,
  creating later integration work.
- Reviewers must police accidental private names in public seam plans and
  documentation.

Operational requirements:

- Public repository changes for PLP1-RDY record doctrine and seam plans only.
- Later seam implementations must use generic names and must be verifiable
  without private licensed content.
- Private CI and private web builds must be produced from private manifests and
  artifacts, not public catalog or public bundle paths.
- Public release checks must inspect public bundles for private identifiers.

## Determinism impact

No change. This ADR affects repository/build doctrine only. It does not affect
RNG, iteration order, clocks, floating point, parallelism, serialization order,
replay, or hashes.

Later private CI must prove determinism for the private game through its own
private tests, replay checks, fixtures, traces, and benchmarks.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, trace format, migration rule, or golden trace
changes. No public replay or fixture migration is authorized.

Private traces and fixtures belong in private artifacts and must not enter
public CI or public source unless rewritten as private-free public evidence by a
later accepted process.

## Visibility impact

The decision strengthens visibility and IP separation by requiring private
catalog entries, renderer overlays, e2e tests, fixtures, docs, private WASM/web
builds, and private CI artifacts to remain private.

Public browser payloads and public catalog surfaces continue to contain only
public games. Any generic public seam must be safe to inspect without private
licensed names, IDs, rules text, card text, assets, or fixtures.

## Data/Rust boundary impact

No new static data field, hand-authored format, expression, selector, behavior
ID, variant, or schema is introduced. Behavior remains in typed Rust.

This ADR does not authorize YAML, DSLs, public private-game manifests, public
private catalog rows, or rule-like static data. Private repository data remains
subject to the same typed Rust behavior authority and no-DSL/no-YAML rules.

## `engine-core` contamination risk

This ADR adds no game noun, mechanic noun, private licensed content, strategy,
renderer concern, networking concern, storage concern, catalog concern, or CI
concern to `engine-core`.

Private repository pressure must not shape `engine-core`. Public seams must be
generic and private-free, and any kernel change still requires the normal
foundation/ADR review.

## `game-stdlib` / primitive-pressure impact

No helper is introduced or promoted by this ADR.

Private implementation evidence does not silently promote public primitives.
Any later reusable helper must go through public-safe mechanic-atlas or
mechanical-scaffolding governance.

## UI impact

No UI files, renderer registrations, public catalog rows, action trees,
previews, effect-log animation, accessibility behavior, reduced-motion
handling, or dev inspectors change.

Later private renderer overlays live in private builds. Public `apps/web` may
gain only generic, private-free extension seams in a later implementation unit.
TypeScript remains presentation-only.

## Bot impact

No bot code, bot policy class, candidate ranking, explanation output,
hidden-information handling, public bot level, or benchmark changes.

Private bot work belongs in the private repository and must use the same legal
action and viewer-safety rules.

## IP impact

This ADR is an IP-separation decision. Private licensed content belongs in the
private repository and private build artifacts only.

Public files, public docs, public CI manifests/artifacts, public submodules,
public optional dependencies, public catalog constants, public traces, public
fixtures, public e2e names, public app bundles, and public WASM/JS must not name
or carry private licensed material.

## Benchmark impact

No benchmarks, thresholds, harnesses, CI lanes, or benchmark policies change.

Later private CI may run private benchmarks in private artifacts. Public
benchmark lanes remain private-free unless a later accepted public-safe seam
requires a generic change.

## Migration notes

Existing docs to update:

- `docs/FOUNDATIONS.md`
- `docs/IP-POLICY.md`
- `docs/WASM-CLIENT-BOUNDARY.md`
- `docs/ARCHITECTURE.md`
- `apps/web/README.md`
- `templates/PRIVATE-RELEASE-CHECKLIST.md`

Existing games to back-port:

- None.

Existing traces to preserve or update:

- Preserve all existing public traces; no public trace update is authorized.

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
