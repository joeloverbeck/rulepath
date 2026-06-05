# <game_id> UI

Game ID: `<game_id>`

Rules version: <version>

Last updated: YYYY-MM-DD

## Renderer assumptions

- Default renderer: React + SVG unless profiled/ADR otherwise.
- Expected object count:
- Animation pressure:
- Screen sizes supported:
- Reduced-motion behavior:

## Legal action mapping

| Rust action choice / tree node | UI control | Normal mode behavior | Debug/learning mode behavior |
|---|---|---|---|
| <choice> | <cell/button/card/etc.> | enabled only if legal | safe diagnostics if shown |

TypeScript must not decide legality.

## Progressive construction

Use this section for compound actions.

1. <stage: action type/source/target/etc.>
2. <stage>
3. preview from Rust
4. confirm action path

At every stage, the next choices come from Rust.

## Rust-generated previews

| Preview | Source | Viewer-safe contents | Must not contain |
|---|---|---|---|
| <preview> | Rust | <contents> | hidden state / guessed TS consequences |

## Effect-to-animation mapping

| Semantic effect | Animation | Reduced motion | Settle-to-view check |
|---|---|---|---|
| <effect> | <animation> | <fallback> | <check> |

## Accessibility labels

| Element | Label / description | Keyboard path | Notes |
|---|---|---|---|
| <element> | <label> | <keys/focus> | <notes> |

## Reduced motion

- Default animation:
- Reduced-motion replacement:
- Tests/smoke coverage:

## Debug payloads

Debug mode may show:

- seed, rules version, data version;
- public view inspector;
- action tree inspector;
- selected action path;
- effect log;
- command log;
- bot timing and safe candidate ranking.

Debug mode must not receive unauthorized hidden state in public builds.

## Hidden-information safeguards

| Surface | Safeguard | Test |
|---|---|---|
| public view | <safeguard> | <test> |
| action tree | <safeguard> | <test> |
| preview | <safeguard> | <test> |
| effect log | <safeguard> | <test> |
| DOM attributes/test IDs | <safeguard> | <test> |
| local storage/replay export | <safeguard> | <test> |
| dev inspector | <safeguard> | <test> |

## UI smoke tests

- load game picker;
- start match;
- show public view;
- show legal actions;
- apply one human action;
- run one bot turn;
- show effect log;
- replay at least one step;
- reduced-motion smoke;
- responsiveness smoke.
