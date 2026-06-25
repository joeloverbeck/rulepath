# Rulepath Architecture

Status: architectural law. Supersede only by accepted ADR.

Rulepath uses a Rust-workspace-first architecture with a React/TypeScript public web app. Rust is the behavior authority. TypeScript is the presentation client.

## 1. Recommended repository shape

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
    future_game_crates/
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
```

The shape MAY evolve by ADR. The ownership rule MUST NOT change: Rust owns behavior; TypeScript owns presentation.

## 2. Dependency direction

```text
apps/web -> wasm-api package boundary
wasm-api -> games registry + engine-core contracts
games/* -> engine-core + ai-core traits + optional game-stdlib
ai-core -> engine-core contracts
game-stdlib -> engine-core contracts only when needed
engine-core -> no Rulepath crate with game mechanics
```

`engine-core` MUST NOT depend on `game-stdlib`, `ai-core`, `wasm-api`, `apps/web`, or `games/*`.

`apps/web` MUST NOT import game Rust internals except through the WASM/API package boundary.

## 3. Ownership table

| Layer | Owns | Must not own |
|---|---|---|
| `engine-core` | generic identities, versions, seeds, actor/viewer contracts, action tree/action path contracts, command envelopes, diagnostics, effect envelopes, replay/hash/checkpoint contracts, visibility contracts, serialization boundaries | game nouns, mechanics, rule helpers, bot strategy, renderer metadata, networking policy, accounts, persistence |
| `game-stdlib` | narrow typed helpers promoted after primitive-pressure evidence | speculative universal mechanics, game-specific exceptions, kernel law, data-language behavior |
| `ai-core` | bot traits, random legal bot, deterministic bot RNG helpers, candidate/ranking structures, policy-node utilities, instrumentation, decision limits, simulation hooks | game strategy, hidden-state shortcuts, UI code |
| `games/*` | game state, rules, typed actions, validation, transitions, effects, visibility projection, variants, game-specific bots, UI metadata, authored player-facing rules prose, docs, tests, traces, benchmarks | kernel contracts, browser shell, networking, accounts |
| `wasm-api` | thin batched browser-facing API over Rust behavior | rule logic, hidden-state leakage, renderer policy, chatty hot-loop crossings |
| `apps/web` | app shell, routing if used, picker, setup, layout, renderer integration, panels, static Markdown rules loading/rendering, settings, replay UI, accessibility, safe local import/export | legality, hidden-state authority, bot decisions, replay authority |
| `tools/*` | simulation, replay checking, trace inspection, rule coverage, benchmark reports, seed reduction, fixture validation | game behavior not present in games, public UI polish |


### 3A. Reuse Ownership Matrix

Accepted ADR 0008 separates behavior-free mechanical scaffolding from behavioral
mechanic promotion. The ownership matrix below decides where scaffolding-shaped
work may live. The narrowest lawful owner wins.

| Reuse lane | Owner | Allowed scope | Dependency rule |
|---|---|---|---|
| Kernel ergonomics | `engine-core` | Tiny helpers over generic contract vocabulary already allowed in the kernel: ids, versions, actor/viewer contracts, action paths, command envelopes, effect envelopes, visibility scopes, replay/hash/checkpoint contracts, and serialization boundaries. | Normal dependency of all Rust crates that need core contracts. |
| Shared behavior-free game-layer scaffolding | `game-stdlib` | Typed helpers over game-layer inputs that are behavior-free, deterministic, leak-safe, and registered in [MECHANICAL-SCAFFOLDING-REGISTER.md](MECHANICAL-SCAFFOLDING-REGISTER.md). | Optional production dependency from games that adopt the helper. |
| Dev-only evidence/test scaffolding | `game-test-support` crate | Pairwise no-leak harnesses, replay/evidence profile harnesses, fixture builders, and similar test-only infrastructure governed by the scaffolding register. | Dev-dependency only. Production crates and WASM/browser surfaces MUST NOT depend on it. |
| Browser bridge adapters | `wasm-api` | Thin adapters that serialize Rust-owned safe payloads or evidence/export profiles without deciding legality, visibility, or rules. | `wasm-api` may depend on Rust behavior crates; `apps/web` consumes only the exported API. |
| Local-only scaffolding | owning crate or game | Repetition that is not semantically identical, not proven behavior-free, or not worth extracting. | No shared dependency. Revisit at the next register trigger. |

If a helper needs game nouns or decides behavior, it is not kernel ergonomics. If
it decides legality, scoring, reveal, projection, effect meaning, bot policy, or
hidden-state semantics, it is not mechanical scaffolding and must remain
game-local, follow the mechanic atlas, or require a separate ADR.

### 3B. Forward mechanical-scaffolding conformance

Every new official game MUST perform a mechanical-scaffolding reuse-first audit
before serious implementation. The audit compares the game's planned
behavior-free infrastructure against the mechanical-scaffolding register and the
lawful shared homes in §3A.

The forward conformance sequence is:

1. reuse an existing registered/promoted helper when its accepted boundary fits;
2. record a register-backed exception before introducing a parallel local shape;
3. register every newly invented behavior-free scaffolding shape, including a
   first-use shape that remains local;
4. name earlier official games whose local code now matches the new shape; and
5. queue a bounded follow-on refactoring unit for those earlier sites, or record
   an accepted `local-only`, `deferred`, or `rejected` disposition with evidence
   and a next review trigger.

A queued unit is conformance work, not permission to broaden the helper. It MUST
preserve behavior by default and MUST follow ADR 0009 for any byte, hash,
fixture, RNG, export, or visibility migration.

This section does not govern behavioral mechanics. `ARCHITECTURE.md` §3.1 and
`MECHANIC-ATLAS.md` continue to govern promoted behavioral helpers, including
the unchanged third-use hard gate.

### 3.1 Promoted-helper conformance

When a helper is promoted to `game-stdlib`, dependency direction alone is not enough. Every official game whose local code matches the promoted primitive's scope MUST either depend on and use that helper, be audited not applicable, or carry an accepted atlas exception. The obligation is retroactive: older admitted games are still official games and must not silently fork a primitive once the primitive has been promoted.

Conformance work MUST preserve each game's public behavior by default: replay hashes, trace JSON, action order, diagnostics, semantic-effect order, view payloads, UI/WASM behavior, bot legality, benchmark identities, and data versions are stable unless an accepted spec explicitly authorizes a migration.

The architecture line remains narrow. A promoted helper may own behavior-free reusable vocabulary such as typed coordinates or deterministic iteration. It MUST NOT become a game-description language, an occupancy engine, a legality engine, or a place to hide game-specific rules. Open promotion debt blocks further mechanic-ladder advancement until closed, audited not applicable, or explicitly excepted in the atlas.

## 4. Runtime transition model

Every game follows this conceptual pipeline:

```text
setup(seed, seats, options)
  -> internal state
legal action tree(state, actor/viewer)
  -> viewer-safe legal choices + freshness token
preview(partial_or_full_action_path, actor/viewer, state, freshness token)
  -> viewer-safe preview or diagnostic
validate(action_path, actor, state, freshness token)
  -> command envelope or diagnostic
apply(command, state, deterministic_rng)
  -> new state + ordered semantic effects
project_view(state, viewer)
  -> viewer-safe public/private view
replay(seed, seats, options, command stream)
  -> same states, effects, views, action trees, and hashes
```

The UI may hold a local match handle through WASM, but Rust remains the only authority for legality, previews, validation, application, effects, bots, replay, and views.

### 4.1 Multi-seat match model

Game crates declare their supported seat counts, seat labels, and any
game-local role/team metadata. Setup validation for unsupported counts is a Rust
responsibility, not a browser rule.

Rust owns the active-seat model. A game may have one active actor, an active set,
pending responders, simultaneous commitments, all-active phases, or forced
wait/pass states, but those are game-local action-tree and phase facts. They are
not new `engine-core` concepts. TypeScript may display active and pending seats
from viewer-safe Rust/WASM payloads; it must not infer turn order from seat
index, local mode, or DOM state.

Every view projection accepts a viewer: public observer (`seat_id: None`) or an
authorized seat viewer. Perfect-information games may produce equivalent
projections for all viewers, but hidden-information games must filter each
projection in Rust before it reaches the browser. Multi-seat and hidden-info
projection obligations are detailed in
[MULTI-SEAT-AND-SURFACE-CONTRACT.md](MULTI-SEAT-AND-SURFACE-CONTRACT.md).

Replay records ordered seat assignments. That order is part of deterministic
setup, trace readability, and result summaries, but game-local rules still own
dealer rotation, lead selection, initiative, partnerships, teams, and response
priority.

## 5. Action tree, action path, and command contracts

| Concept | Meaning | Authority |
|---|---|---|
| Action tree | Legal choice structure at a decision point. May be flat or progressive. | Rust |
| Action node | A step where the actor chooses among legal options. | Rust |
| Action choice | A selectable legal option with stable ID/path segment, label/metadata, accessibility text, tags, and optional preview hook. | Rust supplies; UI presents |
| Action path | The selected route through the tree. | UI submits; Rust validates |
| Partial action | Accumulated choices before confirmation. | Rust supplies next legal choices/previews |
| Command envelope | Validated, replayable action ready to apply. | Rust |
| Diagnostic | Rust-supplied stale/invalid/unavailable/hidden reason, already viewer-safe. | Rust |
| Freshness token | Version marker used to reject stale UI submissions gracefully. | Rust |

Simple games MAY expose flat action trees. Compound games MUST expose progressive construction rather than raw command editing.

## 6. Public/private view model

Internal state and viewer-safe views MUST be different types or otherwise impossible to confuse.

The browser MUST receive only data safe for the current viewer. Do not ship hidden state and rely on UI hiding.

Visibility contracts MUST cover:

- hidden components and face-down identities;
- private hands/zones/commitments;
- unrevealed random order;
- secret roles or asymmetric setup;
- private logs and diagnostics;
- previews;
- effect filtering;
- bot inputs;
- candidate rankings;
- explanations;
- serialized public views;
- replay exports;
- dev inspectors and UI test fixtures.

A public payload should be auditable by serializing it and searching for known hidden IDs.

## 7. Semantic effect log model

Effects are semantic facts emitted by Rust. They are not animations.

Effects MUST be:

- deterministic;
- ordered;
- replayable;
- hashable;
- viewer-filtered;
- safe for the receiving viewer;
- sufficiently expressive for animation, replay UI, logs, bot explanations, and diagnostics.

The renderer schedules animations from effects and then settles to the latest public view. State diffs MAY detect missing effect coverage in dev mode; they MUST NOT become normal animation authority.

Effects should cover action start/completion, actor choice, visible placement/removal/movement, reveal/redaction, counter changes, ownership changes, phase/turn changes, visible random samples, commitments/reveals, pending responses, grouped batches, and game end. Game-specific semantic payloads belong in game modules behind generic effect envelopes.

## 8. Replay, checkpoint, and hash model

Replay is first-class from the first tiny game.

A replay SHOULD include:

- game id;
- rules version;
- engine version;
- manifest/data version;
- serialization/schema version;
- seed;
- seats and player mapping;
- options/variants;
- ordered command stream;
- optional checkpoints;
- state hashes;
- effect hashes;
- legal-action-tree hashes;
- public-view hashes for selected viewers;
- build/source metadata when available;
- migration notes if compatibility changed.

Identical game id, rules version, data version, seed, seats, options, and command stream MUST reproduce identical states, effects, action trees, public/private views, and hashes.

Breaking replay compatibility requires ADR or explicit migration notes and intentionally updated golden traces.

## 9. Determinism rules

All randomness MUST pass through the Rust deterministic RNG contract. Random samples that affect replay or are visible later MUST be represented in replayable state/effects according to the game contract.

Rule behavior MUST NOT depend on:

- wall-clock time;
- OS randomness;
- browser APIs;
- unordered iteration order;
- thread scheduling;
- network timing;
- object identity or pointer addresses;
- locale-sensitive sorting;
- floating point unless an ADR defines exact deterministic constraints.

Serialization used for hashes MUST be stable. Hashes must fail loudly on unplanned drift.

## 10. WASM API shape

`wasm-api` is a batched boundary. It should expose operations conceptually like:

```text
list_games
new_match(game_id, seed, seats, options)
load_match(snapshot_or_replay)
get_view(match, viewer)
get_action_tree(match, actor_or_viewer)
preview_action(match, actor, partial_or_full_action_path, freshness_token)
apply_action(match, actor, action_path, freshness_token)
run_bot_turn(match, bot_seat, limits)
get_effects(match, since_cursor, viewer)
get_replay(match, export_mode, viewer)
serialize_match(match, mode, viewer)
```

The API MUST avoid crossing the JS/WASM boundary inside rule hot loops. Prefer coarse calls that return complete viewer-safe payloads.

Any browser-facing payload MUST be safe for the viewer before it crosses the boundary.

## 11. Game module shape

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
    RULES.md
    HOW-TO-PLAY.md
    SOURCES.md
    RULE-COVERAGE.md
    MECHANICS.md
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

Concrete file names MAY vary. Required responsibilities do not: setup, legal actions/action tree, validation, transition application, terminal/outcome detection, semantic effects, visibility projection, serialization, replay, random legal bot, docs, tests, traces, benchmarks, and mechanic inventory.

Games own authored player-facing rules prose in
`games/<game_id>/docs/HOW-TO-PLAY.md`. `apps/web` owns the shared rules
panel/drawer, static Markdown loading, rendering, accessibility, and responsive
layout. For the static-bundled path, `wasm-api` has no new operation; game
behavior and runtime views continue to cross the JS boundary only through
existing Rust/WASM operations.

## 12. Static local-first deployment

Initial public deployment is static:

```text
index.html
assets/*.js
assets/*.css
assets/*.wasm
assets/game-data/*
```

V1/v2 includes human vs bot, local hotseat, bot vs bot replay, replay viewer, and local replay import/export.

V1/v2 excludes accounts, database, hosted multiplayer, matchmaking, chat, ranked play, server persistence, and public authoritative server deployment.

Local storage MAY store safe user preferences and safe local replay data. It MUST NOT store hidden information for unauthorized viewers.

## 13. Future hosted multiplayer path

Future hosted multiplayer MUST use an authoritative Rust server running the same rule code natively.

Future flow:

```text
client requests legal choices / previews locally or from server
client proposes action path
server validates against authoritative Rust state
server applies command through Rust
server appends command and effects
server sends each viewer filtered view/effects
client renders and reconciles
```

Browser clients MAY preview locally through WASM. They MUST NOT own authoritative state.

Deterministic command logs preserve multiplayer readiness. They do not commit Rulepath to peer-to-peer lockstep. Hidden-information games are safer with an authoritative server that sends only viewer-allowed data.

Persistence, accounts, abuse handling, matchmaking, chat, ranking, and networking protocol require later ADRs.

## 14. Architecture acceptance checks

Before accepting a major architectural change, verify:

- the change supports public playable Rulepath before research/private stress tests;
- Rust remains behavior authority;
- TypeScript remains presentation-only;
- `engine-core` remains generic and noun-free;
- game-specific mechanics live in `games/*` or earned `game-stdlib` helpers;
- static data remains typed content/parameters/metadata/fixtures/traces only;
- replay/hash determinism is preserved or explicitly migrated;
- hidden information remains viewer-safe across all payloads;
- WASM calls are batched enough for public play;
- every new official game has a closed mechanical-scaffolding audit receipt,
  current register disposition, and a named prior-game retrofit unit or accepted
  no-refactor disposition;
- v1/v2 remain local-first unless an accepted ADR says otherwise.
