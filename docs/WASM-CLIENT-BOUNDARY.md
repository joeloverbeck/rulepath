# WASM Client Boundary

Status: Rust/WASM-to-browser client boundary for the public catalog.

Rulepath's browser shell is presentation-only. Rust/WASM owns setup, legal action
trees, validation, state transitions, semantic effects, bot decisions, replay
projection, diagnostics, and viewer-safe public/private views. TypeScript owns
loading the artifact, encoding/decoding JSON strings, local shell state, layout,
keyboard interaction, and browser smoke checks.

This document describes the boundary conceptually. It does not by itself change
the exported JSON schema or raw ABI. Incompatible browser-facing schema changes
require their own accepted ADR or implementation spec.

## Loading

The web app builds `crates/wasm-api` to `apps/web/public/wasm_api.wasm`, then Vite
copies it into `dist`. `apps/web/src/wasm/client.ts` resolves the artifact from
`import.meta.env.BASE_URL`, so the built shell can run from a root mount or a nested
static mount such as `/rulepath/`.

The raw ABI remains deliberately small. The client calls exported `rulepath_*`
functions, writes string arguments through `rulepath_alloc`, reads JSON from
`rulepath_last_output_ptr` / `rulepath_last_output_len`, and frees arguments with
`rulepath_dealloc`. This keeps the bridge small and explicit; `wasm-bindgen` remains
deferred until a concrete boundary problem justifies an ADR.

## Operation Groups

| Group | Rust exports | TypeScript methods | Authority |
|---|---|---|---|
| Version and capability | `rulepath_placeholder_version_*`, `rulepath_feature_report` | `version`, `featureReport` | Rust reports API version, operations, and feature names. |
| Catalog | `rulepath_list_games` | `listGames` | Rust provides the public game catalog and setup metadata. |
| Match lifecycle | `rulepath_new_match`, `rulepath_get_view` | `newMatch`, `getView` | Rust creates in-memory matches from `game_id`, seed, seats, options, and projects the requested viewer. |
| Legal actions | `rulepath_get_action_tree`, `rulepath_apply_action` | `getActionTree`, `applyAction` | Rust returns legal choices for the actor/viewer and validates submitted paths/freshness tokens. |
| Bots | `rulepath_run_bot_turn` | `runBotTurn` | Rust chooses random legal bot actions. TypeScript never chooses legal moves. |
| Effects | `rulepath_get_effects` | `getEffects` | Rust returns viewer-safe semantic effects for UI feedback and logs. |
| Replay | `rulepath_export_replay`, `rulepath_import_replay`, `rulepath_replay_step`, `rulepath_replay_reset` | `exportReplay`, `importReplay`, `replayStep`, `replayReset` | Rust exports/imports replay documents for the requested viewer scope and projects replay states. |

Conceptual multi-seat operation shapes:

```text
new_match(game_id, seed, seats, options)
get_view(match, viewer)
get_action_tree(match, actor_or_viewer)
submit_action(match, actor, action_path, freshness_token)
run_bot_turn(match, bot_seat, limits)
get_effects(match, since_cursor, viewer)
export_replay(match, viewer_scope)
```

Game views may expose `active_seat`, `active_seats`, pending responders, or
phase-owned waiting state depending on the game. Rust owns those facts.
TypeScript displays but never computes turn order, active seats, pending
responders, legal seat counts, legality, outcome allocation, or hidden-info
redaction. Multi-seat projection obligations are defined in
[MULTI-SEAT-AND-SURFACE-CONTRACT.md](MULTI-SEAT-AND-SURFACE-CONTRACT.md).

Player-facing `HOW-TO-PLAY.md` text is currently delivered as static web
presentation content, not through a WASM operation. Adding a future `get_rules`
operation would require updating this boundary document and proving the
operation is viewer-safe and behavior-free.

## Data Shapes

All bridge calls use JSON strings at the raw ABI. Successful calls return status
`0` and a typed JSON payload. Failed calls return nonzero status and a typed
diagnostic with `code` and `message`.

Viewer-safe browser payloads are:

- game catalog entries: game id, display name, rules version, schema version;
- public/private views for the requested viewer: game-specific visible state,
  active or pending seats, terminal outcome, and freshness token;
- action choices: segment, label, accessibility label;
- semantic effects: viewer-filtered event payloads only;
- diagnostics: public code and message;
- replay documents and replay projections produced by Rust.

For perfect-information games, viewer projections may be output-equivalent. For
hidden-information games, Rust/WASM must filter every view, action tree,
preview, effect, diagnostic, bot explanation, and replay export before any
browser payload exists. TypeScript must not receive hidden state and hide it in
CSS, DOM conditionals, local state, or dev toggles.

## Private Catalog Semantics

The public WASM catalog contains only public games. A sanctioned private lane
uses a private repository and private WASM/web build for private catalog
entries. Public `rulepath_list_games`, public catalog docs, public smoke output,
and public web bundles must not contain private game titles, ids, module names,
fixture names, e2e names, renderer keys, or source expressions.

Catalog seam plan:

1. keep the public registry private-free and deterministic;
2. if a private build needs additional entries, add them through a private
   overlay after the public registry has produced its public entries;
3. keep setup metadata, display metadata, and renderer keys viewer-safe and
   private-build-only for private games;
4. prove public catalog cleanliness with public checks and prove private overlay
   alignment only in private CI.

This section is doctrine only. It does not change the current exported ABI or
add an adapter in this readiness unit.

## Replay Safety

Replay import is local-only. The authoritative import-size guard is Rust/WASM's
`import_replay` path, backed by `MAX_REPLAY_IMPORT_BYTES` in `crates/wasm-api`;
the shell imposes no stricter UI cap and defers oversize rejection to Rust. The
bound is sized to admit the catalog's own full-length exports, including
Starbridge Crossing 6-seat 2000-ply documents, while still rejecting
pathological local input before Rust parsing. The UI does not mutate replay
contents to make them legal; it passes the document to `rulepath_import_replay`,
then displays Rust-projected reset/step output.

Replay export/import is viewer-scoped for hidden-information games according to
ADR 0004. Internal full traces remain native/dev evidence; browser exports are
public-observer or explicitly labelled seat-private observation timelines.
Future schema migration is not a TypeScript concern unless a later spec adds a
documented migration surface.

## Canonical Seat Grammar

Rust/WASM owns seat-id parsing, formatting, and viewer projection. TypeScript
passes through Rust-provided seat ids and requested viewers; it must not
normalize, infer, reorder, or validate seat ids on its own.

The going-forward canonical seat grammar for external browser/replay payloads is
`seat_<zero-based>`, such as `seat_0`. `SeatId` remains opaque in the Rust
contract today, so this document is a boundary policy and does not by itself
change the exported API schema.

During the migration window, replay/import-facing code may accept only these
bounded import-only aliases:

- `seat_<n>`: canonical underscore form;
- `seat-<n>`: legacy hyphen form;
- `seat-a`: legacy letter form, accepted only where an importer has an explicit
  seat-order mapping for the document being imported.

Unknown forms are rejected. Alias handling is not an open-ended normalizer, and
it must not allow TypeScript to decide which private timeline a viewer receives.
Strict canonical output and Rust parser migration are deferred to the Part C
implementation unit; until then, the WASM exported-API schema is unchanged.

## Developer Panel Safety

The developer panel is secondary to the play surface and shows only whitelisted,
viewer-safe data:

- API version, feature names, and operation count from `featureReport`;
- selected public game name;
- match id, seed, play mode, local viewer, active/pending actors, public
  freshness token;
- action choice count, effect cursor/count, pending operation;
- replay id/cursor;
- public diagnostics.

It does not show full Rust state, hidden state, private bot reasoning, candidate
rankings, or raw memory.

## Deferred Work

- `wasm-bindgen` or a generated binding layer;
- hosted deployment or backend authority;
- incompatible multi-seat exported-API schema changes without a spec/ADR;
- hidden-information renderer proof beyond the current no-leak pattern;
- search, MCTS/ISMCTS, ML, or RL bots.
