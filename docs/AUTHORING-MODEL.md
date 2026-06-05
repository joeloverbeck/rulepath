# Rulepath Authoring Model

Status: authoring law for official game modules, docs, static content, and agent work.

Rulepath game authoring is typed, explicit, testable, and hostile to accidental mini-languages. Humans and agents write game behavior in Rust; static files supply typed content and parameters.

## 1. V1/V2 behavior model

Game behavior must be written in typed Rust game modules or in narrow typed Rust helpers promoted through the mechanic atlas.

Allowed behavior locations:

- `games/<game_id>/src/setup.rs` or equivalent;
- `games/<game_id>/src/actions.rs` or equivalent;
- `games/<game_id>/src/rules.rs` or equivalent;
- `games/<game_id>/src/visibility.rs` or equivalent;
- `games/<game_id>/src/effects.rs` or equivalent;
- `games/<game_id>/src/bots.rs` or equivalent;
- `game-stdlib` helpers after pressure and review.

Forbidden behavior locations:

- TOML, JSON, RON, CSV, YAML;
- UI metadata;
- localization strings;
- explanation templates;
- TypeScript renderer code;
- `engine-core`.

## 2. Typed Rust game module authoring

Every official game module should define typed models for:

- game state;
- seat/player mapping;
- phases or turn model;
- action payloads or action-path decoding;
- validated commands;
- semantic effects;
- visibility projection;
- scoring and terminal outcome;
- typed variants;
- bot policy hooks;
- UI metadata hooks.

Rules should read like rules. Verbose local game code is acceptable. A contaminated kernel is not.

## 3. Required per-game docs

Every official game must maintain:

```text
games/<game_id>/docs/
  MECHANICS.md
  RULES.md
  SOURCES.md
  RULE-COVERAGE.md
  AI.md
  UI.md
  BENCHMARKS.md
```

Use the templates in `/templates` when creating the game.

## 4. Recommended game shape

```text
games/<game_id>/
  src/
    lib.rs
    ids.rs
    state.rs
    setup.rs
    actions.rs
    rules.rs
    visibility.rs
    effects.rs
    variants.rs
    bots.rs
    ui.rs
  data/
    manifest.toml
    variants.toml
    fixtures/
  docs/
    MECHANICS.md
    RULES.md
    SOURCES.md
    RULE-COVERAGE.md
    AI.md
    UI.md
    BENCHMARKS.md
  tests/
    golden_traces/
```

Concrete file names may vary, but responsibilities must stay explicit. Do not hide core rule behavior behind generic factories unless the mechanic atlas justifies the helper.

## 5. Game content and data authoring

Static content is allowed for manifests, metadata, labels, typed variants, original component IDs, deck/list composition, scoring tables, fixtures, traces, and UI/explanation templates.

Static content must:

- use approved formats from `DATA-RUST-BOUNDARY.md`;
- reject unknown fields;
- deserialize into typed structures;
- receive semantic validation;
- participate in data versioning and replay hashes when behavior-affecting parameters change;
- avoid behavior-looking fields.

Static content must not encode rule branches, procedural effects, selectors, triggers, loops, tactical AI conditions, or hidden defaults.

## 6. Source notes

Every public game needs source notes before public exposure.

Each source note should record:

```text
Source: name + URL
Consulted: YYYY-MM-DD
Used for: rule verification / variant comparison / historical note
Copied prose/assets: none
Variant choice: chosen variant
Rulepath deviations: any deliberate changes
Public name rationale: common name safe or neutral name chosen
Asset status: original / project-owned / licensed / generated-reviewed
```

Source notes are not permission to copy prose, art, card text, screenshots, iconography, or trade dress.

## 7. Rule coverage

Rule coverage is part of authoring, not late QA polish.

A new rule should be added in this order:

1. Update `RULES.md` with original-language rule text.
2. Update `SOURCES.md` and variant notes.
3. Update `RULE-COVERAGE.md`.
4. Add unit/rule tests.
5. Add or update golden traces.
6. Add invariant/simulation coverage.
7. Implement typed Rust behavior.
8. Update effects, previews, UI metadata, and bot policy as needed.
9. Benchmark affected hot paths.

Every omitted rule must be marked not applicable, intentionally deferred, unsupported, or open.

## 8. Mechanic inventory requirement

Every official game must include `MECHANICS.md` using `templates/GAME-MECHANICS.md`.

The inventory must classify mechanics across topology/spatial model, components/zones, action shape, turn/phase model, randomness/chance, visibility, resources, movement/capture/placement, pattern/directional scanning, commitment/reveal, reaction/pending response, scoring/outcome, effect shape, UI pattern, bot pattern, and benchmark pressure.

After updating a game inventory, update the repo-level mechanic atlas or primitive-pressure ledger when repeated shape appears. A third official game may not reimplement an already repeated mechanic shape without ledger decision.

## 9. Future DSL policy

No DSL at project start.

A future DSL may be proposed only after multiple Rust modules show repeated, painful, stable behavior shapes that typed Rust plus `game-stdlib` cannot maintain cleanly.

A DSL proposal must include problem cases, rejected Rust/helper alternatives, grammar or typed schema, static typing model, deterministic lowering, source spans, formatter, linter, versioning and migration, tests, benchmarks, replay/hash implications, examples, anti-examples, hidden-default prevention, agent-safety plan, and public/private data policy.

A DSL must not be introduced to make one monster game possible.

## 10. Public naming policy

Public game IDs and names should be neutral when commercial trademark or trade-dress risk exists. Common descriptive names may be used when safe.

Recommended IDs:

| Mechanic family | Safer Rulepath ID |
|---|---|
| take-away counter game | `race_to_n` or `nim_lite` |
| Tic-Tac-Toe-like placement | `three_marks` |
| gravity four-in-a-row | `column_four` |
| directional flipping | `directional_flip` |
| checkers/draughts-like movement | `draughts_lite` |
| War-like comparison | `high_card_duel` |
| simple draw/stand scoring | `blackjack_lite` |
| resource economy microgame | `token_bazaar` or `resource_race` |
| simultaneous commitment | `secret_draft` or original name |
| poker subset | `poker_lite` |
| trick-taking | `plain_tricks` |
| bluffing/claims | original name only |

## 11. Private licensed modules policy

Private licensed experiments are late, isolated, optional, and never architecture-driving.

They must live outside public repository artifacts and public builds. Public CI must not require them. Public docs must not leak proprietary names, scenarios, text, assets, or IDs. Public WASM/JS must not bundle them. Local/private builds must load private data only from private sources.

Do not hide private licensed content in a public static build behind credentials or feature flags. If it ships to an unauthorized browser, it has shipped.

## 12. Agent authoring task template

Use `templates/AGENT-TASK.md` for agent work. Every authoring task should state context, target game/module, ladder stage, mechanics, goal, non-goals, forbidden changes, sources/docs, tests, benchmarks, docs, output format, and review checklist.

Agents must output complete files or coherent complete sections, not diffs. They must follow the failing-test protocol and must not invent architecture.

## 13. Authoring acceptance checklist

Before calling a game official, verify:

- typed Rust owns behavior;
- static data is content/parameters only;
- per-game docs are complete;
- source notes and public naming are safe;
- rule coverage has no silent gaps;
- mechanic inventory is complete;
- atlas/ledger pressure is updated;
- random legal bot exists;
- non-random bots have docs, tests, explanations, and benchmarks;
- traces, replay, visibility, serialization, simulations, benchmarks, and UI smoke tests exist;
- `engine-core` remains noun-free;
- no private licensed content ships publicly.
