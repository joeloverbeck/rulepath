# Data/Rust Boundary

Status: project law. This document is intentionally strict because the boundary is the most likely place for architecture rot.

Rulepath is game-agnostic at the engine contract level. Rulepath is not data-driven at the rule-behavior level.

Typed Rust owns behavior in v1.

## 1. One-sentence rule

Static files may describe content and parameters; they must not become an untyped programming language.

## 2. Ownership summary

| Layer | Allowed responsibility | Forbidden responsibility |
|---|---|---|
| `engine-core` | generic contracts and infrastructure | game nouns, mechanics, rule helpers, content language |
| `game-stdlib` | earned reusable typed mechanics | speculative universal behavior engine |
| `games/*` Rust | behavior, validation, transitions, visibility, bots | browser shell, generic kernel policy |
| `games/*/data` | typed content, labels, fixtures, tables, variants | selectors, loops, triggers, rule branches, exception logic |
| `apps/web` | rendering and presentation | legality, hidden-state authority, behavior |

## 3. Allowed static data

Static data MAY include:

- game manifests;
- display names;
- icon IDs;
- theme tokens;
- board labels;
- coordinate labels;
- original/public card IDs;
- non-proprietary display text for original or public games;
- deck composition;
- initial setup constants;
- scoring tables;
- typed variant selection among compiled Rust variants;
- localization strings;
- explanation templates;
- UI metadata;
- golden traces;
- benchmark fixtures.

Allowed data MUST deserialize into typed Rust structures or validated browser-facing schemas.

## 4. Forbidden static data

Static data MUST NOT include:

- selectors as strings;
- rule branches;
- loops;
- triggers;
- conditional card effects;
- tactical AI conditions;
- exception logic;
- mandatory action rules;
- hidden default behavior;
- arbitrary expression languages;
- nested untyped objects interpreted as behavior;
- behavior encoded by naming convention;
- YAML behavior.

## 5. Variant nuance

Variant flags in data are allowed only when they deserialize into typed Rust enums whose behavior is implemented and tested in Rust.

Good:

```text
variant = "misere"
```

where Rust has a documented and tested `Variant::Misere` implementation.

Bad:

```text
if_last_move_takes_final_token_then = "current_player_loses"
```

because the data is now defining rule behavior.

## 6. Card/effect identity nuance

Card or effect identity may be data-listed only when it maps to compiled Rust behavior.

Good:

```text
card_id = "draw_two"
effect = "DrawTwo"
```

where Rust has `EffectId::DrawTwo` or equivalent, documented and tested.

Bad:

```text
effect:
  when: "played"
  select: "current_player.deck.top(2)"
  then:
    - move: "$selected"
      to: "current_player.hand"
```

because the data defines selectors, behavior, and mutation.

## 7. Format decision table

| Format | Use for | Do not use for |
|---|---|---|
| TOML | manifests, simple options, metadata, narrow variants | complex nested rules, selectors, procedures |
| JSON | browser payload fixtures, golden traces, replay summaries, machine reports | hand-authored behavior, rule logic |
| RON | Rust-shaped fixtures, enum-heavy setup data, complex typed static content | untyped behavior or hidden DSLs |
| CSV | tabular card lists, scoring tables, coverage matrices, benchmark exports | procedural effects or nested state machines |
| Postcard or binary Serde | compact internal snapshots/caches, non-hand-authored replay artifacts | hand-authored rules, public docs |
| YAML | not default in v1 | any behavior; any use without ADR |

`serde_yaml` being unmaintained strengthens the default ban, but maintenance status is not the only reason. The stronger reason is that YAML previously becomes an accidental untyped programming language too easily.

## 8. Static data import pipeline

Every static file consumed by Rust SHOULD follow this path:

```text
file bytes
  -> parser for approved format
  -> typed schema deserialize
  -> semantic validation with diagnostics
  -> version/hash recording
  -> read-only game content structure
  -> Rust behavior consumes typed values
```

Validation SHOULD detect:

- missing IDs;
- duplicate IDs;
- references to unknown typed variants;
- invalid numeric ranges;
- invalid localization keys;
- invalid UI metadata references;
- deck/list totals that fail invariants;
- proprietary/private IDs in public builds;
- behavior-looking fields such as `when`, `if`, `then`, `selector`, `trigger`, `condition`, `loop`, `script`.

## 9. UI metadata boundary

UI metadata MAY include:

- labels;
- short help text;
- icon IDs;
- layout hints;
- coordinate labels;
- piece shapes;
- theme tokens;
- action grouping tags;
- accessibility labels;
- explanation-template IDs.

UI metadata MUST NOT include:

- rule legality;
- hidden behavior;
- hidden card/piece identities in public payloads;
- tactical AI conditions;
- action consequences not supplied by Rust effects/previews.

## 10. Explanation templates

Templates are presentation text, not source-of-truth rules.

Good:

```text
move_piece = "Move {piece} from {from} to {to}."
change_score = "{seat} gains {amount} point(s)."
```

Bad:

```text
when phase == "reaction" and actor.has("shield") then cancel_attack
```

Templates MUST be keyed to semantic actions/effects emitted by Rust.

## 11. Game-specific data is allowed

Data-driven content may be game-specific. That does not violate engine agnosticism.

Correct distinction:

- `engine-core` must be game-agnostic;
- `games/token_bazaar/data/cards.csv` may be specific to `token_bazaar`;
- Rust in `games/token_bazaar` interprets that typed content through game-specific behavior;
- no universal untyped content language is created.

## 12. Review checklist for new static data

A static-data addition is acceptable only when all answers are yes:

- Does it deserialize into typed Rust or a strict schema?
- Is every field content/parameter/metadata rather than behavior?
- Are behavior IDs backed by compiled Rust implementations?
- Are variants typed enums with tests?
- Are unknown fields rejected by default?
- Are diagnostics source-located enough for humans and agents?
- Are hashes/version fields updated for replay?
- Is public/private build separation preserved?
- Would a reviewer understand the behavior without reading the data as code?

## 13. ADR triggers

An ADR is required for:

- introducing YAML anywhere;
- introducing a new hand-authored data format;
- adding expression parsing;
- adding selectors;
- moving repeated mechanics into `game-stdlib` before two-game pressure;
- adding a DSL;
- adding binary formats to public replay interchange;
- changing replay hashes or data-version semantics.

## 14. Anti-patterns

MUST NOT:

- encode behavior in `id` naming conventions;
- add `script`, `selector`, `condition`, `trigger`, `when`, `if`, `then`, `foreach`, or equivalent fields;
- hide exception rules in card rows;
- implement tactical AI by data weights and conditions;
- bypass Rust validation for data-defined commands;
- add YAML because it is convenient for one file;
- call a format “not a DSL” when humans must learn its semantics to understand behavior;
- create a universal content language before the game ladder proves the shapes.

## Source notes

See `SOURCES.md`, especially serde_yaml, TOML, JSON, RON, CSV, Postcard, Ludii, Regular Boardgames, Regular Games, and Board Game Arena AI-development guidance.
