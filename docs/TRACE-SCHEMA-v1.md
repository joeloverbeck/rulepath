# Rulepath Trace Schema v1

Status: canonical trace and replay fixture schema for Gate 2.

Trace Schema v1 defines the JSON shape for golden traces and replay fixtures. A trace is typed evidence for deterministic replay, hash drift detection, diagnostics, visibility surfaces, and benchmark or simulation reproduction. A trace is never rule behavior.

Rust game code remains the behavior authority. Static trace files may record setup data, command evidence, expected hashes, expected diagnostics, expected outcomes, migration notes, and not-applicable rationale. They MUST NOT contain selectors, rule branches, procedural instructions, or any other DSL-like behavior.

Legacy key-value `.trace` files may be imported only as temporary migration inputs. After Gate 2, CI golden traces use Trace Schema v1 JSON. Any retained legacy fixture must be explicitly marked as migration-only and checked by `fixture-check`.

## 1. File Contract

Trace Schema v1 files are JSON objects.

Required parser behavior:

- reject unknown root fields by default;
- reject unknown nested fields by default;
- reject duplicate trace IDs across a checked trace set;
- reject malformed JSON;
- reject unsupported `schema_version`;
- reject empty human notes;
- reject missing migration or update notes when expected hashes, schema fields, or trace format are updated;
- reject behavior-looking keys recursively.

`schema_version` is the Trace Schema version, not a game rules version. Gate 2 uses Trace Schema v1.

## 2. Root Fields

| Field | Required | Type | Meaning |
|---|---:|---|---|
| `schema_version` | yes | integer or exact schema string | Trace Schema v1 marker. |
| `trace_id` | yes | string | Stable unique ID within the checked trace set. |
| `fixture_kind` | yes | string enum | Classification only: `commands`, `terminal`, `bot`, `invalid`, `diagnostic`, or `not_applicable`. |
| `purpose` | yes | string | Short machine-readable reason for the trace. |
| `note` | yes | string | Human explanation for why the trace exists. Empty notes fail validation. |
| `migration_update_note` | yes | string | Human explanation for migration or intentional expected-surface updates. |
| `game_id` | yes | string | Gate 2 uses `race_to_n`. |
| `rules_version` | yes | string | Canonical Gate 2 form for Race to N is `race_to_n-rules-v1`. |
| `engine_version` | yes | string | Engine contract version used for replay and hashes. |
| `data_version` | yes | string | Manifest/data version used for setup and variants. |
| `seed` | yes | integer or string | Setup seed. Record it even when game rules do not consume randomness. |
| `variant` | yes | string | Selected variant ID, such as `race_to_21`. |
| `options` | yes | object | Structured setup/options object, even if empty or default. |
| `seats` | yes | array | Ordered seat/player mapping where relevant. |
| `commands` | yes, except `fixture_kind = not_applicable` | array | Explicit replay command records. |
| `checkpoints` | yes | array | Ordered replay checkpoints, including at least the final checkpoint for normal traces. |
| `expected_state_hashes` | yes | object | Expected state hash by checkpoint or final checkpoint. |
| `expected_effect_hashes` | yes | object | Expected effect surface hash by checkpoint or final checkpoint. |
| `expected_action_tree_hashes` | yes where an action tree exists | object | Expected action-tree hash by checkpoint. Terminal traces must record the terminal empty-tree hash. |
| `expected_public_view_hashes` | yes | object | Expected public-view hashes for selected viewers. Race to N requires the public/all-viewer surface. |
| `expected_private_view_hashes` | conditional | object or not-applicable entry | Required when private views exist; otherwise record an explicit not-applicable rationale. |
| `expected_diagnostics` | conditional | array | Required for invalid, stale, or diagnostic traces. |
| `expected_outcome` | yes | object | Terminal/winner status or explicit non-terminal expectation. |
| `expected_terminal_state` | yes | object | Boolean terminal status plus winner/rationale where terminal. |
| `not_applicable` | yes | object | Explicit rationale for omitted hidden-information and stochastic game-rule event surfaces. |

Field names in this table are authoritative. Older names such as `action_stream` and `expected_legal_action_hashes` are legacy wording and MUST NOT appear in Trace Schema v1 JSON.

## 3. Command Records

Each command in `commands` is a JSON object.

| Field | Required | Type | Meaning |
|---|---:|---|---|
| `index` | yes | integer | Zero-based command index. |
| `actor_seat` | yes | string | Explicit acting seat. Fixture data must not rely on implicit active-seat behavior. |
| `action_path` | yes | array of strings | Ordered action path segments as submitted evidence. |
| `freshness_token` | yes | string | Token submitted by the command or expected by the diagnostic path. |
| `expect` | yes | string enum | `applied` or `diagnostic`. |
| `expected_diagnostic_code` | conditional | string | Required when `expect` is `diagnostic`. |
| `producer` | conditional | object | For bot traces, record bot policy, policy version, seed, and the explicit command produced. |

Replay tooling may verify that a bot still chooses the recorded command, but the trace still records the command stream. The command record is evidence of a replayable action path, not an executable script.

## 4. Checkpoints And Hashes

Checkpoints identify the replay points where expected surfaces are compared. The exact checkpoint IDs are game-local evidence, but they must be stable within the trace and usable by `replay-check`.

Trace Schema v1 requires deterministic expected surfaces:

- state hashes;
- effect hashes;
- action-tree hashes where an action tree exists;
- public-view hashes for selected viewers;
- private-view hashes where private views exist;
- diagnostic codes for invalid, stale, and diagnostic traces;
- terminal and outcome expectations.

Intentional changes to any expected surface require a non-empty `migration_update_note`. Unexplained drift is a failure.

## 5. Not Applicable Surfaces

Race to N is a perfect-information game with no stochastic game-rule events. Trace Schema v1 still requires explicit not-applicable rationale so later games cannot silently omit visibility or randomness evidence.

For Race to N, `not_applicable` MUST record at least:

- hidden-information redaction is not applicable because all game state is public;
- stochastic game-rule events are not applicable because the game rules use no randomness;
- private-view hashes are not applicable unless a private-view API is added;
- preview hashes are not applicable unless a Rust preview surface is added.

Bot RNG is separate from stochastic game-rule events. Bot traces record producer metadata and explicit commands.

## 6. Forbidden Behavior Keys

Trace files and replay fixtures MUST NOT contain behavior-looking keys at any depth unless a future accepted ADR explicitly permits typed lowering.

Forbidden keys include:

```text
when
if
then
else
selector
condition
trigger
script
loop
foreach
priority_expression
ai_condition
effect_script
rule
requires
valid_if
on_play
on_reveal
```

This list is intentionally aligned with [FOUNDATIONS.md](FOUNDATIONS.md) and [ENGINE-GAME-DATA-BOUNDARY.md](ENGINE-GAME-DATA-BOUNDARY.md). Fixture data is typed content and evidence, not a rule language.

## 7. Version Anchors

For Gate 2 Race to N traces:

- `game_id` is `race_to_n`;
- `rules_version` is the canonical string `race_to_n-rules-v1`;
- `data_version` must match `games/race_to_n/data/manifest.toml`;
- `engine_version` must identify the replay/hash contract used by the current engine.

If another artifact still records Race to N rules version as an integer, tooling must reduce it to `race_to_n-rules-v1` or fail with a consistency error. New Trace Schema v1 files record the canonical string form directly.

## 8. Tool Responsibilities

`fixture-check` owns strict schema validation, duplicate ID detection, unknown-field rejection, behavior-key rejection, migration status checks, and version consistency.

`replay-check` owns executing setup/options/commands through Rust behavior and comparing actual replay surfaces with expected trace surfaces.

`trace-viewer` owns human-readable inspection of Trace Schema v1 evidence. It is not the Gate 3 browser replay viewer.
