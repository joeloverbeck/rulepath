# Rulepath Data/Rust Boundary

Status: project law. Supersede only by accepted ADR.

## 1. Canonical boundary statement

Rulepath is data-driven for game content, parameters, presentation metadata, fixtures, traces, and typed variant selection. Rulepath is not data-driven for rule behavior in v1/v2. Game behavior is typed Rust in game modules or in promoted typed Rust helpers.

Use phrases such as typed content-driven game modules, typed static content, and data-driven content and parameters. Do not use “data-driven rules” except when explicitly warning against it.

Static files may describe content and parameters. Static files must not become an untyped programming language.

## 2. Ownership summary

| Layer | Allowed responsibility | Forbidden responsibility |
|---|---|---|
| `engine-core` | generic contracts and infrastructure | game nouns, mechanics, rule helpers, content language |
| `game-stdlib` | earned reusable typed mechanics after pressure | speculative universal behavior engine |
| `games/*` Rust | setup, validation, transitions, visibility, effects, bots, variant behavior | browser shell, generic kernel policy |
| `games/*/data` | typed content, labels, fixtures, tables, typed variants, metadata | selectors, loops, triggers, rule branches, exception logic |
| `apps/web` | rendering and presentation | legality, hidden-state authority, behavior |

## 3. Allowed data

Static data may include:

- manifests;
- display names and short descriptions;
- icon IDs and theme tokens;
- coordinate labels and help text;
- original/public component IDs;
- deck/list composition for original/public games;
- setup constants;
- scoring tables;
- typed variant selection among compiled Rust variants;
- localization strings;
- explanation templates keyed to Rust effects/actions;
- UI metadata;
- fixtures;
- golden traces;
- replay summaries;
- benchmark fixtures and reports;
- source notes and rule coverage tables.

Allowed data must deserialize into typed Rust structures or strict browser-facing schemas. Unknown fields must be rejected by default.

## 4. Forbidden data

Static data must not include:

- selectors as strings;
- rule branches;
- loops;
- triggers;
- conditional effects;
- mandatory action rules;
- exception logic;
- tactical AI conditions;
- procedural mutation instructions;
- arbitrary expressions;
- hidden default behavior;
- nested untyped objects interpreted as behavior;
- behavior encoded by naming convention;
- YAML behavior.

Behavior-looking fields are banned unless an ADR and typed lowering policy explicitly permits them. Suspicious names include `when`, `if`, `then`, `else`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, `ai_condition`, `effect_script`, `rule`, and `requires`.

## 5. Typed static content pipeline

Every static file consumed by Rust should follow this path:

```text
file bytes
  -> approved parser
  -> typed schema deserialization
  -> unknown-field rejection
  -> semantic validation with source-located diagnostics
  -> version/hash recording
  -> read-only typed content structure
  -> Rust behavior consumes typed values
```

Validation should detect duplicate IDs, missing IDs, unknown references, invalid ranges, invalid localization keys, invalid UI metadata references, composition totals that fail invariants, private/proprietary IDs in public builds, and behavior-looking field names.

## 6. Variant enum rules

Variant selection in data is allowed only when it maps to a documented Rust enum or equivalent typed value whose behavior is implemented and tested in Rust.

Good:

```text
variant = "misere"
```

This is acceptable only if Rust has a documented, tested `Misere` variant and all behavior lives in Rust.

Bad:

```text
if_final_token_taken = "current_player_loses"
```

That data defines behavior and is forbidden.

Variant combinations must be enumerated, validated, covered in rule docs, included in data versioning, and represented in traces where they affect replay.

## 7. Card/effect identity nuance

A static file may list an effect or component identity only when the identity maps to compiled Rust behavior.

Good:

```text
card_id = "draw_two"
effect_id = "DrawTwo"
```

This is acceptable only if Rust defines, documents, tests, and version-controls `DrawTwo` behavior.

Bad:

```text
effect:
  when: played
  select: current_player.deck.top(2)
  then: move selected to current_player.hand
```

That defines selectors, behavior, and mutation. It is forbidden.

Card text for public games must be original or permissioned. Static content IDs must not smuggle proprietary text into public builds.

## 8. UI metadata boundary

UI metadata may include labels, icon IDs, short help, layout hints, coordinate labels, visual tags, theme tokens, action grouping tags, accessibility labels, and explanation-template IDs.

UI metadata must not include legality, hidden behavior, tactical AI conditions, rule consequences not supplied by Rust previews/effects, hidden identities in public payloads, or any field whose meaning changes game state.

The UI may group, sort, and decorate Rust-provided legal choices. It must not invent legality.

## 9. Explanation templates boundary

Explanation templates are presentation text. They are not source-of-truth rules.

Good:

```text
move_visible_item = "{actor} moves {item} from {from} to {to}."
score_change = "{seat} gains {amount} point(s)."
```

Bad:

```text
when phase == reaction and actor.has(shield) then cancel_attack
```

Templates must be keyed to semantic actions/effects emitted by Rust. Templates must be viewer-safe and must not reveal hidden information through interpolation.

## 10. Data format table

| Format | Use for | Do not use for |
|---|---|---|
| TOML | manifests, simple options, metadata, narrow typed variants | complex nested rules, selectors, procedures |
| JSON | browser payload fixtures, golden traces, replay summaries, machine reports | hand-authored behavior, rule logic |
| RON | Rust-shaped fixtures, enum-heavy setup data, complex typed static content | untyped behavior or hidden DSLs |
| CSV | tabular card lists, scoring tables, coverage matrices, benchmark exports | procedural effects or nested state machines |
| Postcard or binary Serde | compact non-hand-authored internal snapshots/caches when approved | hand-authored rules or public text interchange by default |
| YAML | not default in v1/v2 | any behavior; any use without ADR |

The `serde_yaml` maintenance status strengthens the YAML ban. The deeper reason is that YAML too easily becomes an accidental untyped programming language.

## 11. Unknown-field rejection

All hand-authored data schemas must reject unknown fields unless an ADR grants a narrow migration exception.

Unknown fields are dangerous because agents and humans can believe they are changing behavior when Rust ignores them. Validation diagnostics should name the file, field, path, and nearest known alternatives.

## 12. Behavior-looking field detection

Static validators should scan for suspicious field names and values. A field is suspicious when a reviewer must read it as code to understand behavior.

A suspicious field must resolve to one of:

- renamed as presentation/content;
- moved into typed Rust behavior;
- converted into a typed enum value with Rust implementation;
- rejected;
- escalated to ADR.

Do not add comments saying “not a DSL” while leaving humans to learn procedural semantics.

## 13. ADR triggers

ADR is required for:

- introducing YAML anywhere;
- introducing a new hand-authored data format;
- adding selectors, expressions, or rule-like conditions;
- changing unknown-field policy;
- moving repeated mechanics into `game-stdlib` before normal pressure;
- introducing a DSL;
- adding binary formats to public replay interchange;
- changing replay hashes or data-version semantics;
- allowing static data to select among behavior more broadly than a typed documented enum.

## 14. Review checklist

A static-data addition is acceptable only when all answers are yes:

- Does it deserialize into typed Rust or strict viewer-safe schema?
- Are unknown fields rejected?
- Is every field content, parameter, fixture, trace, or metadata rather than behavior?
- Are behavior IDs backed by compiled Rust implementations?
- Are variants typed enums with tests and rule docs?
- Are diagnostics source-located enough for humans and agents?
- Are hashes/version fields updated for replay?
- Is public/private build separation preserved?
- Would a reviewer understand the behavior without reading the data as code?
- Does the addition avoid approving “data-driven rules” as a project direction?
