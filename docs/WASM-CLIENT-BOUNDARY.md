# WASM Client Boundary

Status: Gate 3 client contract for `race_to_n`.

Rulepath's browser shell is presentation-only. Rust/WASM owns setup, legal action
trees, validation, state transitions, semantic effects, bot decisions, replay
projection, diagnostics, and viewer-safe public views. TypeScript owns loading the
artifact, encoding/decoding JSON strings, local shell state, layout, keyboard
interaction, and browser smoke checks.

## Loading

The web app builds `crates/wasm-api` to `apps/web/public/wasm_api.wasm`, then Vite
copies it into `dist`. `apps/web/src/wasm/client.ts` resolves the artifact from
`import.meta.env.BASE_URL`, so the built shell can run from a root mount or a nested
static mount such as `/rulepath/`.

The raw ABI is retained for Gate 3. The client calls exported `rulepath_*`
functions, writes string arguments through `rulepath_alloc`, reads JSON from
`rulepath_last_output_ptr` / `rulepath_last_output_len`, and frees arguments with
`rulepath_dealloc`. This keeps the bridge small and explicit; `wasm-bindgen` remains
deferred until a concrete boundary problem justifies an ADR.

## Operation Groups

| Group | Rust exports | TypeScript methods | Authority |
|---|---|---|---|
| Version and capability | `rulepath_placeholder_version_*`, `rulepath_feature_report` | `version`, `featureReport` | Rust reports API version, operations, and feature names. |
| Catalog | `rulepath_list_games` | `listGames` | Rust provides the Gate 3 game catalog. Gate 3 supports `race_to_n` only. |
| Match lifecycle | `rulepath_new_match`, `rulepath_get_view` | `newMatch`, `getView` | Rust creates in-memory matches and projects public views. |
| Legal actions | `rulepath_get_action_tree`, `rulepath_apply_action` | `getActionTree`, `applyAction` | Rust returns legal choices and validates submitted paths/freshness tokens. |
| Bots | `rulepath_run_bot_turn` | `runBotTurn` | Rust chooses random legal bot actions. TypeScript never chooses legal moves. |
| Effects | `rulepath_get_effects` | `getEffects` | Rust returns viewer-safe semantic effects for UI feedback and logs. |
| Replay | `rulepath_export_replay`, `rulepath_import_replay`, `rulepath_replay_step`, `rulepath_replay_reset` | `exportReplay`, `importReplay`, `replayStep`, `replayReset` | Rust exports/imports replay documents and projects replay states. |

## Data Shapes

All bridge calls use JSON strings at the raw ABI. Successful calls return status
`0` and a typed JSON payload. Failed calls return nonzero status and a typed
diagnostic with `code` and `message`.

Viewer-safe browser payloads are:

- game catalog entries: game id, display name, rules version, schema version;
- public views: counter, target, max add, active seat, winner, freshness token;
- action choices: segment, label, accessibility label;
- semantic effects: public event payloads only;
- diagnostics: public code and message;
- replay documents and replay projections produced by Rust.

For `race_to_n`, hidden information is not applicable. The replay schema may include
`expected_private_view_hashes.not_applicable` as an explicit perfect-information
marker; it must not include private view payloads.

## Replay Safety

Replay import is local-only and capped in the UI at 128 KiB before calling Rust.
Rust remains the parser and projector. The UI does not mutate replay contents to make
them legal; it passes the document to `rulepath_import_replay`, then displays
Rust-projected reset/step output.

Gate 3 supports the replay schema version exported by the current `wasm-api` for
`race_to_n`. Future schema migration is not a TypeScript concern unless a later spec
adds a documented migration surface.

## Developer Panel Safety

The developer panel is secondary to the play surface and shows only whitelisted,
viewer-safe data:

- API version, feature names, and operation count from `featureReport`;
- selected public game name;
- match id, seed, play mode, active actor, public freshness token;
- action choice count, effect cursor/count, pending operation;
- replay id/cursor;
- public diagnostics.

It does not show full Rust state, hidden state, private bot reasoning, candidate
rankings, or raw memory.

## Deferred Work

- `wasm-bindgen` or a generated binding layer;
- hosted deployment or backend authority;
- multiple-game catalog guarantees beyond `race_to_n`;
- hidden-information renderer proof beyond the Gate 3 no-leak pattern;
- search, MCTS/ISMCTS, ML, or RL bots.
