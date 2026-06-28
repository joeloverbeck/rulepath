# Evidence Fixture Contract

Status: governed by accepted
[ADR 0009](adr/0009-replay-fixture-hash-taxonomy.md).

This document defines Rulepath's evidence fixture profiles. It separates native
command/replay evidence, viewer-scoped browser exports, setup evidence, and
game-local domain evidence so authors and validators do not treat every
`*.trace.json`-adjacent artifact as the same schema.

This contract changes no existing fixture, trace, export, hash, RNG,
serialization, or benchmark byte. Profile migration happens only in named
successor tickets.

## Common Fields

Every evidence artifact should declare these fields once its profile is adopted.

| Field | Meaning |
|---|---|
| `profile_id` | One of the named profiles in this document. |
| `profile_version` | Version anchor for the profile contract, such as `v1`. |
| `visibility_class` | `public`, `viewer-scoped`, `seat-private`, `internal-dev`, or `private-source`. |
| `validator_owner` | Tool, crate, smoke, or game-local validator that owns validation. |
| `game_id` | Game id when the artifact is game-specific. |
| `rules_version` | Game rules version when behavior or replay evidence is involved. |
| `data_version` | Static-data/manifest version when setup or content evidence is involved. |
| `hash_surface_version` | Version anchor for canonical hash surfaces, or `not-applicable`. |
| `canonical_byte_authority` | Owner of byte-stable serialization, or `none`. |
| `migration_update_note` | Required when expected fields, hashes, visibility, or bytes intentionally change. |
| `not_applicable` | Explicit rationale for omitted replay, hash, visibility, or private-view evidence. |

The filename suffix is non-authoritative. A file's profile id, visibility class,
and validator owner classify the artifact; a `.trace.json`, `.fixture.json`,
`.export.json`, or other suffix is only a naming convention.

## Visibility Classes

| Class | Allowed contents | Public/browser policy |
|---|---|---|
| `public` | Facts safe for every viewer, public docs, logs, CI artifacts, and public replay exports. | May ship to browser and public CI. |
| `viewer-scoped` | Facts authorized for one named viewer at one timeline step or export scope. | May ship only to that viewer scope. |
| `seat-private` | Facts authorized for one named seat. | Must be explicitly labelled; pairwise no-leak proof is required for hidden-information games. |
| `internal-dev` | Omniscient or private test evidence needed by native replay, fixture, or no-leak proof. | Test-only; must not be the default browser export or public replay payload. |
| `private-source` | Non-public, licensed, or stress-test material. | Forbidden from public files, public CI, public docs, public traces, public bundles, and public WASM/JS. |

Allowed private data is test-only. If an artifact contains hidden hands, deck
order, hidden commitments, private choices, private bot candidates,
hidden-state-derived explanations, or private-source content, it must be
classified as `internal-dev` or `private-source` and kept out of browser/public
export paths unless Rust has made those facts public for that viewer.

Private-source evidence must live in the private repository or another
authorized private store. Public artifacts may cite only opaque receipt IDs,
profile IDs, and sanitized status; they must not contain private titles, card or
event names, rules prose, source excerpts, fixture names, e2e names, screenshots,
or catalog strings.

## Profiles

### `replay-command-v1`

Purpose: native command/replay evidence for behavior authority.

Validator ownership:

- `fixture-check` validates structure, duplicate IDs, unknown fields,
  behavior-looking keys, version anchors, and migration notes.
- `replay-check` replays commands and compares declared replay/hash surfaces.

Visibility classification:

- Default: `internal-dev`.
- May be `public` only for perfect-information command evidence that contains no
  private seed, hidden order, hidden action, private view, or private-source
  material.

Version anchors:

- `profile_id = replay-command-v1`
- `profile_version = v1`
- `rules_version`, `data_version`, and `hash_surface_version` required when
  replay or hash surfaces are asserted.

Canonical-byte authority:

- Native Rust replay and fixture validators own command/replay canonical bytes.
- Declared hash surfaces may include state, effect, action-tree, public-view,
  seat-private-view, and game-local domain hashes.

### `public-export-v1`

Purpose: browser-safe public observer replay/export observation timeline.

Validator ownership:

- Rust/WASM export code owns serialization.
- Import smoke/e2e checks own browser import behavior.
- No-leak smoke owns hidden-token absence in browser/public artifacts.

Visibility classification:

- Always `public`.
- Must not contain hidden deck order, unauthorized private hands, unrevealed
  commitments, private action paths, hidden-state-derived bot explanations,
  private candidates, or private-source content.

Version anchors:

- `profile_id = public-export-v1`
- `profile_version = v1`
- Export schema/version and game rules/data versions required when export bytes
  are compared.

Canonical-byte authority:

- Rust/WASM export serialization for public observer scope.
- Import replays an observation timeline, not omniscient state.

### `seat-private-export-v1`

Purpose: browser-safe replay/export observation timeline for one authorized
seat.

Validator ownership:

- Rust/WASM export code owns serialization.
- Pairwise no-leak harness owns cross-seat private-token absence.
- Import smoke/e2e checks own browser import behavior for the labelled viewer.

Visibility classification:

- Always `seat-private`.
- Must be labelled with the viewer seat.
- May include only facts that the labelled seat was authorized to observe at
  each timeline step.

Version anchors:

- `profile_id = seat-private-export-v1`
- `profile_version = v1`
- Viewer-seat grammar/version required when the artifact serializes a viewer
  identity.

Canonical-byte authority:

- Rust/WASM export serialization for the requested seat scope.
- Import replays a seat-scoped observation timeline, not omniscient state.

### `setup-evidence-v1`

Purpose: deterministic setup, options, seat, variant, and manifest evidence
without command replay authority.

Validator ownership:

- Fixture/static-data validators own shape and version checks.
- Game-local setup validators own game-specific setup semantics.

Visibility classification:

- Default: `public`.
- May be `viewer-scoped`, `seat-private`, or `internal-dev` only when the setup
  evidence intentionally contains private or test-only setup facts and is kept
  out of browser/public exports.

Version anchors:

- `profile_id = setup-evidence-v1`
- `profile_version = v1`
- Game setup schema, rules version, data version, and seat grammar/version where
  relevant.

Canonical-byte authority:

- `none` unless a validator explicitly defines stable bytes.
- Setup evidence is structured validation input, not command replay authority.

### `domain-evidence-v1`

Purpose: game-local edge-case evidence such as evaluator, allocator, topology,
scoring, no-leak matrix, or other domain-specific proof inputs.

Validator ownership:

- The game-local validator or fixture-check extension named by the artifact.
- For no-leak matrices, the pairwise no-leak harness owns leak checks.

Visibility classification:

- Must declare `public`, `viewer-scoped`, `seat-private`, `internal-dev`, or
  `private-source` explicitly.
- Hidden-information domain evidence is test-only unless it is already
  viewer-safe for the declared viewer.

Version anchors:

- `profile_id = domain-evidence-v1`
- `profile_version = v1`
- Game rules/data/domain schema version required when semantic checks are
  asserted.

Canonical-byte authority:

- `none` unless the named domain validator defines stable bytes.
- Domain evidence must not become rule behavior, procedural configuration, or a
  DSL.

### Private-source evidence profiles

Private-source evidence reuses the profile system with
`visibility_class = private-source`. The profile id should still describe the
validator owner and semantic purpose, such as `setup-evidence-v1` for private
setup receipts or `domain-evidence-v1` for private event/effect coverage. A
private-source artifact may support public-safe doctrine only through an opaque
receipt and sanitized rationale.

For private large games, source/evidence receipts should identify:

- source receipt id and consult date;
- owning private repository or private evidence store;
- validator owner;
- whether the artifact is setup, domain, event/effect, no-leak, benchmark, or
  release evidence;
- public-safe status summary;
- explicit statement that no public artifact contains private source expression.

The presence of private-source evidence never authorizes public helper
promotion, public trace updates, or public schema migration by itself.

## Hash Surface Rules

Hash surfaces must name what they cover:

- `state`
- `effect`
- `action-tree`
- `public-view`
- `seat-private-view`
- `public-export`
- `seat-private-export`
- game-local domain surface

Changing hash meaning, field order, visibility set, redaction rule, or canonical
bytes requires a migration/update note. Foundation-level changes require an
accepted ADR.

## Authoring Rules

- Do not infer a profile from filename suffix.
- Do not put selectors, conditions, triggers, formulas, scripts, loops,
  behavior IDs, hidden defaults, or procedural mutation instructions in evidence
  fixtures.
- Do not ship `internal-dev` or `private-source` evidence to public browser
  exports.
- Do not use setup or domain evidence as rule behavior.
- Keep Rust as setup, legality, transition, projection, effect, replay, and
  export authority.
- Record not-applicable rationale instead of silently omitting hidden-info,
  replay, hash, or private-view evidence.
