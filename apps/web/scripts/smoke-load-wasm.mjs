import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const artifactPath = join(__dirname, "..", "public", "wasm_api.wasm");
const bytes = await readFile(artifactPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const wasm = instance.exports;
const encoder = new TextEncoder();
const decoder = new TextDecoder();

const requiredExports = [
  "memory",
  "rulepath_placeholder_version_ptr",
  "rulepath_placeholder_version_len",
  "rulepath_alloc",
  "rulepath_dealloc",
  "rulepath_last_output_ptr",
  "rulepath_last_output_len",
  "rulepath_feature_report",
  "rulepath_list_games",
  "rulepath_new_match",
  "rulepath_get_view",
  "rulepath_get_action_tree",
  "rulepath_apply_action",
  "rulepath_run_bot_turn",
  "rulepath_get_effects",
  "rulepath_export_replay",
  "rulepath_import_replay",
  "rulepath_replay_step",
  "rulepath_replay_reset",
];

for (const exportName of requiredExports) {
  assert(wasm[exportName], `required export is present: ${exportName}`);
}

const version = read(
  wasm.rulepath_placeholder_version_ptr(),
  wasm.rulepath_placeholder_version_len(),
);
assert(version === "rulepath-wasm-api/0.1.0", "wasm artifact loads");

const featureReport = invoke(() => wasm.rulepath_feature_report(), []);
assert(featureReport.api_version === version, "feature_report returns the API version");
for (const op of ["new_match", "get_view", "apply_action", "export_replay", "import_replay"]) {
  assert(featureReport.operations.includes(op), `feature_report includes ${op}`);
}

const catalog = invoke(() => wasm.rulepath_list_games(), []);
assert(catalog.some((game) => game.game_id === "race_to_n"), "list_games includes race_to_n");

const created = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 1n),
  ["race_to_n"],
);
assert(created.match_id, "new_match returns a match id");

const initialView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(initialView.counter === 0, "public view starts at counter zero");
assert(initialView.active_seat === "seat_0", "seat_0 starts active");

const tree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [created.match_id, "seat_0"],
);
assert(tree.choices.some((choice) => choice.segment === "add-1"), "action tree exposes legal add-1");

const afterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(tree.freshness_token),
    ),
  [created.match_id, "seat_0", "add-1"],
);
assert(afterHuman.view.counter > 0, "human legal action advances the counter");

const afterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [created.match_id, afterHuman.view.active_seat],
);
assert(afterBot.view.active_seat === "seat_0" || afterBot.view.winner, "bot turn resolves");

const effects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [created.match_id],
);
assert(effects.length > 0, "effect fetching returns semantic effects");

const exportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(exportedReplay.commands.length > 0, "export_replay returns command stream");

const importedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(exportedReplay)],
);
assert(importedReplay.replay_id, "import_replay returns a replay handle");
assert(importedReplay.command_count === exportedReplay.commands.length, "import_replay preserves command count");

const replayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [importedReplay.replay_id],
);
assert(replayReset.cursor === 0, "replay_reset returns cursor zero");
assert(replayReset.view.counter === 0, "replay_reset projects the initial state");

const replayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [importedReplay.replay_id],
);
assert(replayStep.cursor === 1, "replay_step advances to requested cursor");
assert(replayStep.view.counter === afterHuman.view.counter, "replay_step projects the applied action");

let staleDiagnostic = null;
try {
  invoke(
    (args) =>
      wasm.rulepath_apply_action(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
        BigInt(tree.freshness_token),
      ),
    [created.match_id, "seat_0", "add-1"],
  );
} catch (error) {
  staleDiagnostic = error.diagnostic;
}
assert(staleDiagnostic?.code === "stale_action", "stale submission returns typed diagnostic");
assert(typeof staleDiagnostic.message === "string", "stale diagnostic is message-only public output");

console.log(
  JSON.stringify({
    version,
    operations: featureReport.operations.length,
    games: catalog.length,
    match_id: created.match_id,
    effects: effects.length,
    diagnostic: staleDiagnostic.code,
    replay_cursor: replayStep.cursor,
  }),
);

function read(ptr, len) {
  return decoder.decode(new Uint8Array(wasm.memory.buffer, ptr, len));
}

function write(value) {
  const bytes = encoder.encode(value);
  const ptr = wasm.rulepath_alloc(bytes.length);
  new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
  return { ptr, len: bytes.length };
}

function output() {
  return read(wasm.rulepath_last_output_ptr(), wasm.rulepath_last_output_len());
}

function invoke(call, values) {
  const args = values.map(write);
  try {
    const status = call(args);
    const parsed = JSON.parse(output());
    if (status !== 0) {
      const error = new Error(parsed.message);
      error.diagnostic = parsed;
      throw error;
    }
    return parsed;
  } finally {
    for (const arg of args) {
      wasm.rulepath_dealloc(arg.ptr, arg.len);
    }
  }
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
