# Visual Interaction Contract

Status: operational contract between Rust rules, WASM API, React shell, renderer, and public UX.

This document turns the UI doctrine into enforceable behavior. The UI may be beautiful, but it must be legally safe, visibility-safe, replay-safe, and effect-driven.

## 1. Inputs to the UI

The browser may receive only viewer-safe payloads:

| Payload | Produced by | Contains | Must not contain |
|---|---|---|---|
| `PublicView` | Rust game module | visible state for one viewer | hidden state for other seats |
| `ActionTree` | Rust game module | legal choices/action paths for actor/viewer | illegal branches in normal mode unless explicitly requested for debug and safe |
| `Preview` | Rust game module | viewer-safe cost/effect estimate for partial or full action | hidden identities or actual hidden state |
| `EffectLog` | Rust game module | viewer-filtered semantic effects | unauthorized hidden outcomes |
| `Diagnostics` | Rust engine/game | stale/invalid/unavailable reasons | hidden information disguised as reasons |
| `UiMetadata` | Rust game module/static data | labels, icons, layout hints, accessibility labels | legality, rule behavior, hidden state |

The UI MUST treat these as data from the authority. It must not supplement them with rule guesses.

## 2. Action lifecycle

```text
1. UI requests public view and action tree for viewer/actor.
2. Rust returns view, action tree, and view token.
3. UI maps legal choices to controls/hit targets.
4. Player selects choices.
5. For partial/compound choices, UI requests Rust preview.
6. UI displays safe preview and next legal choices.
7. Player confirms full action path.
8. UI submits action path plus view token.
9. Rust validates freshness and legality.
10. Rust applies command or returns diagnostic.
11. UI receives new public view and semantic effects.
12. Renderer animates effects.
13. Renderer settles to new public view.
```

## 3. Freshness and stale actions

Every action tree SHOULD be associated with a freshness marker such as a view token, state hash, action-tree hash, or cursor.

Rust MUST reject stale submissions gracefully.

A stale diagnostic SHOULD include:

- that the action is stale;
- whether the UI should refresh;
- no hidden reason that leaks state;
- no panic or generic failure unless there is a bug.

UI response:

- stop pending animation for the rejected command;
- refresh view/action tree;
- show a gentle message such as “The position changed. Choose again.”;
- preserve replay/debug context.

## 4. Legal controls

The renderer MUST create hit targets from legal `ActionChoice` data.

Normal mode:

- legal choices are enabled;
- illegal choices are absent or inert;
- unavailable choices are not shown unless needed for clarity;
- disabled reasons are not shown unless Rust supplies safe reasons.

Learning/debug mode:

- may show disabled illegal choices;
- must label the mode clearly;
- must use Rust diagnostics;
- must not leak hidden state.

## 5. Progressive construction

Compound action UI MUST not ask the player to build raw command objects.

Use staged choices:

```text
Action type
  -> source/origin
  -> target/destination
  -> additional selections
  -> payment/resources
  -> preview
  -> confirm
```

At each stage:

- Rust owns the set of legal next choices;
- UI owns presentation, grouping, and focus;
- preview is safe and Rust-generated;
- hidden information remains hidden.

## 6. Effect-to-animation contract

Effects are semantic facts. The renderer translates them into animations.

Example mapping:

| Effect | Possible animation | Required settle check |
|---|---|---|
| `ItemPlaced` | token appears/drops/fades in | item appears at final public-view location |
| `ItemMoved` | move along path | item ends at target public-view location |
| `ItemRevealed` | flip/reveal animation | public face matches viewer-safe view |
| `CounterChanged` | number rolls/highlights | displayed counter equals public view |
| `ChoiceRevealed` | commitments flip together | only now-visible choices are displayed |
| `GameEnded` | outcome banner | outcome matches public view |

Animation scheduler MUST be tolerant of grouped effects and simultaneous effects.

Reduced-motion mode SHOULD replace movement-heavy animations with simple fades/highlights.

## 7. Settle-to-view rule

After animations complete, the renderer MUST settle to the latest public view.

The UI SHOULD have a dev assertion mode that detects:

- effect animation ended in a visual state inconsistent with public view;
- missing effect coverage caused a visual jump;
- extra visual objects remain;
- hidden data appears in DOM or logs.

State diffs MAY help diagnose missing effects. They MUST NOT be the primary causal source.

## 8. Hidden information safety

The browser MUST NOT receive hidden state for unauthorized viewers.

Safety applies to:

- public views;
- action trees;
- previews;
- effect logs;
- diagnostics;
- UI metadata;
- DOM attributes;
- data-test IDs;
- local storage;
- replay exports;
- debug panels;
- bot explanation payloads.

Bad:

```text
<div data-card-id="secret_ace_of_spades" class="card back">
```

Good:

```text
<div data-visible-kind="face_down_card" class="card back">
```

## 9. Dev inspector boundary

Local developer builds MAY inspect internal state.

Public hosted builds MUST NOT ship internal state to the browser for unauthorized viewers.

If a dev inspector needs full state, it must be:

- local-only;
- clearly labeled;
- excluded from public build or guarded at the data-source level;
- unable to appear merely by toggling CSS or route flags in public static files.

## 10. Renderer adapter expectations

The renderer boundary SHOULD conceptually accept:

```text
render(public_view, ui_metadata, legal_affordances, animation_state)
schedule(effect_batch, public_view_before, public_view_after, reduced_motion)
hit_test(pointer_or_keyboard_event) -> action_choice_id?  // choice IDs came from Rust
debug_overlay(optional_inspector_data)
```

`hit_test` returns references to Rust-provided choices. It does not invent action legality.

## 11. Stage-specific examples

### `column_four`

- Rust action tree exposes legal columns only.
- UI highlights legal columns.
- Hover preview asks Rust or uses Rust-provided landing metadata.
- Apply emits placement, turn change, and possible win-line effects.
- Renderer animates token drop and win line.
- Settle view confirms token grid and outcome.

### `draughts_lite`

- Rust action tree exposes legal origins.
- After origin, Rust exposes legal destinations/captures.
- If continuation is forced, Rust exposes only continuation choices.
- UI shows path construction and confirmation.
- Apply emits movement/capture/promotion/turn effects.
- Renderer animates each segment from effect detail.

### Hidden commitment game

- Before reveal, public view shows committed/not-committed status only.
- Owner view may show own committed card/choice.
- Logs say “Seat B committed” without identity.
- Reveal phase emits viewer-safe revealed choices.
- Renderer flips all now-public choices together.

## 12. Interaction acceptance checklist

Before exposing a game publicly:

- every clickable gameplay target maps to a Rust legal choice;
- illegal choices are absent/inert in normal mode;
- stale action rejection is tested;
- previews come from Rust;
- effect batches animate and settle to view;
- reduced-motion path works;
- hidden info is absent from DOM and local storage;
- replay uses command/effect data, not guessed diffs;
- keyboard/focus path exists where practical;
- UI smoke tests cover at least one human action, one bot action, and one replay step.

## Source notes

See `SOURCES.md`, especially React, MDN SVG, MDN Canvas API, PixiJS, boardgame.io logs/time-travel precedent, Rust/WASM, command-log replay, and IP sources.
