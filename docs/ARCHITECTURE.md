# Rulepath Architecture

Status: architectural law. Supersede only by accepted ADR.

Rulepath uses a Rust-workspace-first architecture with a React/TypeScript public web app. Rust is the source of truth for behavior, legality, visibility, replay, and bots. TypeScript is the presentation client.

## 1. Repository shape

Recommended initial shape:

```text
/
  Cargo.toml
  crates/
    engine-core/
    game-stdlib/
    ai-core/
    wasm-api/
  games/
    race_to_n/
    three_marks/
    column_four/
    directional_flip/
    draughts_lite/
    high_card_duel/
    token_bazaar/
    ...
  apps/
    web/
  tools/
    simulate/
    replay-check/
    trace-viewer/
    rule-coverage/
    bench-report/
    seed-reducer/
    fixture-check/
  benches/
  docs/
    adr/
  templates/
```

This shape may change by ADR. The ownership rule must not change: Rust owns behavior; TypeScript owns presentation.

## 2. Dependency direction

Recommended dependency direction:

```text
apps/web -> wasm-api package boundary
wasm-api -> games registry + engine-core contracts
ai-core -> engine-core contracts
games/* -> engine-core + ai-core traits + optional game-stdlib
game-stdlib -> engine-core contracts only when necessary
engine-core -> no Rulepath crate with game mechanics
```

`engine-core` must not depend on `game-stdlib`, `ai-core`, `wasm-api`, `apps/web`, or `games/*`.

## 3. Ownership table

| Area | Owns | Must not own |
|---|---|---|
| `engine-core` | identities, versions, seeds, actor/viewer contracts, action tree contracts, commands, diagnostics, effect contracts, replay/hash contracts, visibility contracts, serialization boundaries | game nouns, rules, mechanics, bot strategy, renderer metadata, networking, accounts, persistence |
| `game-stdlib` | earned reusable typed helpers after primitive-pressure evidence | speculative abstractions, kernel law, game-specific exceptions |
| `ai-core` | bot traits, random legal bot, deterministic bot RNG, policy helpers, candidate/ranking structures, instrumentation, decision limits | game strategy, hidden-state cheating, UI code |
| `games/*` | game nouns, rules, state, actions, visibility, effects, variants, game-specific bots, UI metadata, docs, tests, traces, benchmarks | kernel contracts, browser shell, networking |
| `wasm-api` | thin batched browser-facing API over Rust | rule logic, hidden-state leakage, chatty hot-loop crossings |
| `apps/web` | app shell, game picker, setup, layout, renderer integration, panels, settings, replay UI, accessibility | rule legality, bot decisions, hidden-state authority |
| `tools/*` | simulation, replay checking, trace inspection, coverage, benchmark reports, seed reduction, fixture validation | public UI polish, game behavior not present in games |

## 4. Engine-core allowed and forbidden responsibilities

`engine-core` may contain:

- `GameId`, `RulesVersion`, `ManifestVersion`, `MatchId`, `SeatId`, `PlayerId`, viewer identity;
- deterministic seed and RNG contracts;
- action tree, action path, command, freshness token, and diagnostic contracts;
- semantic effect log contracts;
- replay, checkpoint, version, hash, and migration contracts;
- visibility/public-view contracts;
- serialization hooks and generic errors.

`engine-core` must not contain board, grid, card, deck, pile, hand, suit, faction, scenario, trick, pot, resource, role, combat, territory, movement, adjacency, line, capture, flip, promotion, or similar game nouns.

Any kernel change must answer the [kernel-change protocol](INVARIANTS.md#2-kernel-change-protocol). Default answer: do not change `engine-core`.

## 5. Game-stdlib role

`game-stdlib` contains typed helpers only after earned pressure. It may eventually hold narrow helpers for repeated mechanic shapes such as spatial topology, directional scanning, typed zones, deterministic shuffle helpers, counters, graph maps, simultaneous choice, drafting, auction/betting accounting, or reaction windows.

Promotion requires the mechanic atlas and primitive-pressure ledger. A helper extracted for one hypothetical future game is forbidden without ADR.

A third official game with the same mechanic shape may not proceed until the ledger records reuse, promotion, or explicit deferral with rationale.

## 6. Game module shape

Recommended game module shape:

```text
games/<game_id>/
  Cargo.toml
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
    rule_tests.rs
    visibility_tests.rs
    simulation_tests.rs
    serialization_tests.rs
    bot_tests.rs
```

Concrete file names may vary. Required responsibilities do not: setup, legal action tree or legal actions, action validation, transition application, terminal/outcome detection, semantic effect emission, public/private view projection, serialization, random legal bot, docs, tests, traces, benchmarks, and mechanic inventory.

## 7. WASM API shape

`wasm-api` is a batched boundary. It should expose operations conceptually like:

```text
list_games -> game catalog
new_match(game id, seed, seats, options) -> match handle
load_match(snapshot or replay) -> match handle
get_public_view(match, viewer) -> viewer-safe payload
get_action_tree(match, viewer) -> legal choices + freshness token
preview_action(match, viewer, partial/full action path, freshness token) -> safe preview or diagnostic
apply_action(match, viewer, action path, freshness token) -> apply result, view, effects, diagnostics
run_bot_turn(match, bot seat, limits) -> apply result
get_effects(match, since cursor, viewer) -> viewer-filtered effects
get_replay(match) -> replay payload
serialize_match(match, mode, viewer) -> snapshot/export payload
```

The API must avoid crossing the JS/WASM boundary inside rule hot loops. Payloads sent to the browser must be viewer-safe.

## 8. Public/private view model

Internal state and viewer-safe views are different types. A browser view must be safe to send to that viewer; do not ship hidden state and rely on UI hiding.

Visibility contracts must cover hidden components, face-down identities, secret commitments, unrevealed random outcomes, private logs, previews, diagnostics, bot information access, serialization/export, dev inspectors, and replay redaction.

## 9. Action tree, action path, and command model

Core transition separation:

```text
setup(seed, seats, options) -> internal state
legal action tree(state, actor/viewer) -> legal choice structure
preview(action path, actor/viewer, state) -> safe preview or diagnostic
validate(action path, actor, state, freshness token) -> command or diagnostic
apply(command, state, rng) -> new state + semantic effects
public view(state, viewer) -> viewer-safe projection
replay(seed, options, command stream) -> same states, effects, and hashes
```

Concepts:

| Concept | Meaning |
|---|---|
| Action tree | Legal choice structure at a decision point. |
| Action node | A point where the actor chooses among options. |
| Action choice | Selectable legal option with label, tags, accessibility text, and optional preview hook. |
| Action path | Selected path through the legal tree. |
| Partial action | Accumulated choices before confirmation. |
| Command | Validated action ready to apply. |
| Diagnostic | Rust-supplied reason a path is stale, invalid, disabled, hidden, or unavailable. |
| Freshness token | Version marker used to reject stale UI submissions gracefully. |

Simple games may expose flat actions. Compound games must expose action trees or progressive construction.

## 10. Semantic effect log model

Effects are semantic facts emitted by Rust. They are not animations. The renderer schedules animations from effects and then settles to the new public view.

Effects must be deterministic, ordered, hashable, replayable, and viewer-filtered. Effects must not leak hidden information.

Effect contracts should support action start/completion, actor choice, placement/removal/movement of visible items, reveal/redaction, counter or score changes, ownership changes, phase/turn changes, random samples when visible or replay-relevant, commitments/reveals, pending responses, and game end. Game modules may define game-specific semantic payloads through generic contracts.

## 11. Replay, version, and hash model

Replay is first-class. A replay should include:

- game id;
- rules version;
- engine version;
- manifest/data version;
- seed;
- seats and player mapping;
- options/variants;
- ordered command stream;
- optional checkpoints;
- state, effect, legal-action, and public-view hashes at checkpoints;
- serialization version;
- source-build metadata when available.

Breaking replay compatibility requires ADR or explicit migration notes. Golden traces must fail loudly on unplanned drift.

## 12. Determinism model

Identical game version, seed, seats, options, static data version, and command stream must produce identical states, effects, views, action trees, and hashes.

Rules:

- all randomness passes through the Rust RNG contract;
- random samples are logged when visible or replay-relevant;
- wall-clock time, OS randomness, browser APIs, unordered iteration, and thread scheduling must not affect rules;
- floating-point values must not decide rule outcomes unless an ADR defines exact constraints;
- serialization order used for hashes must be stable.

## 13. Local-first static deployment

Initial deployment is static:

```text
index.html
assets/*.js
assets/*.css
assets/*.wasm
assets/game-data/*
```

V1/v2 includes human vs bot, local hotseat, bot vs bot replay, replay viewer, and local replay import/export. It does not include accounts, database, hosted multiplayer, matchmaking, chat, ranked play, server persistence, or public server deployment.

## 14. Future authoritative server multiplayer

Future hosted multiplayer must use an authoritative Rust server running the same rule code natively.

Future flow:

```text
client proposes action path
server validates against authoritative Rust state
server applies command through Rust
server appends command and effects
server sends each client filtered view and effects
client renders and reconciles
```

Browser clients may preview locally through WASM. Server validation remains authoritative.

Do not confuse deterministic command logs with a commitment to peer-to-peer lockstep. Hidden-information games are safer with a server that sends only viewer-allowed data.

Persistence, accounts, abuse handling, matchmaking, and networking require later ADRs.

## 15. Architecture acceptance checklist

Before accepting a major change, verify the [universal acceptance invariants](INVARIANTS.md#3-universal-acceptance-invariants), plus these architecture-specific items:

- game modules own game-specific rules and nouns;
- action trees and diagnostics come from Rust;
- effects are semantic, deterministic, replayable, and filterable;
- v1/v2 remain local-first.
