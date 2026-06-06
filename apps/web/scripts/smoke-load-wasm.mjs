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
assert(catalog.some((game) => game.game_id === "three_marks"), "list_games includes three_marks");
assert(catalog.some((game) => game.game_id === "column_four"), "list_games includes column_four");

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

const threeCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 7n),
  ["three_marks"],
);
assert(threeCreated.match_id, "three_marks new_match returns a match id");
assert(threeCreated.variant_id === "three_marks_standard", "three_marks starts standard variant");

const threeView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [threeCreated.match_id],
);
assert(threeView.game_id === "three_marks", "three_marks view is game-specific");
assert(threeView.variant_id === "three_marks_standard", "three_marks view reports standard variant");
assert(threeView.board_rows === 3 && threeView.board_columns === 3, "three_marks projects a 3x3 board");

const threeTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [threeCreated.match_id, "seat_0"],
);
const firstPlacement = threeTree.choices.find((choice) => choice.segment === "place/r1c1");
assert(firstPlacement, "three_marks action tree exposes placement actions");

const threeAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(threeTree.freshness_token),
    ),
  [threeCreated.match_id, "seat_0", firstPlacement.segment],
);
assert(threeAfterHuman.view.ply_count === 1, "three_marks human placement advances ply");
assert(threeAfterHuman.effects.some((effect) => effect.payload.type === "mark_placed"), "three_marks emits semantic placement effects");

const threeAfterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [threeCreated.match_id, threeAfterHuman.view.active_seat],
);
assert(threeAfterBot.view.ply_count === 2, "three_marks bot turn applies a Rust-selected placement");
assert(threeAfterBot.effects.some((effect) => effect.payload.type === "bot_chose_action"), "three_marks bot emits a semantic bot-choice effect");

const threeExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [threeCreated.match_id],
);
assert(threeExportedReplay.game_id === "three_marks", "three_marks export_replay preserves game id");
assert(threeExportedReplay.expected_replay_hashes.final, "three_marks export includes replay hash");

const threeImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(threeExportedReplay)],
);
assert(threeImportedReplay.game_id === "three_marks", "three_marks import_replay preserves game id");
assert(threeImportedReplay.command_count === threeExportedReplay.commands.length, "three_marks import preserves command count");

const columnCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 13n),
  ["column_four"],
);
assert(columnCreated.match_id, "column_four new_match returns a match id");
assert(columnCreated.variant_id === "column_four_standard", "column_four starts standard variant");

const columnView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [columnCreated.match_id],
);
assert(columnView.game_id === "column_four", "column_four view is game-specific");
assert(columnView.variant_id === "column_four_standard", "column_four view reports standard variant");
assert(columnView.board_rows === 6 && columnView.board_columns === 7, "column_four projects a 7x6 board");
assert(columnView.cells.length === 42, "column_four projects 42 cells");
assert(columnView.legal_targets.length === 7, "column_four starts with seven legal columns");
assert(columnView.private_view_status === "not_applicable_perfect_information", "column_four private view is explicitly not applicable");
assert(columnView.hidden_fields.length === 0, "column_four exposes no hidden fields");

const columnTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [columnCreated.match_id, "seat_0"],
);
assert(columnTree.choices.length === 7, "column_four action tree exposes seven legal columns");
assert(columnTree.choices.some((choice) => choice.segment === "drop/c4"), "column_four action tree exposes center drop");

const columnAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(columnTree.freshness_token),
    ),
  [columnCreated.match_id, "seat_0", "drop/c4"],
);
assert(columnAfterHuman.view.ply_count === 1, "column_four human drop advances ply");
assert(columnAfterHuman.view.cells.some((cell) => cell.cell === "r1c4" && cell.owner === "seat_0"), "column_four drop lands in Rust-projected cell");
assert(columnAfterHuman.effects.some((effect) => effect.payload.type === "piece_landed"), "column_four emits semantic landing effect");

let columnStaleDiagnostic = null;
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
        BigInt(columnTree.freshness_token),
      ),
    [columnCreated.match_id, "seat_1", "drop/c3"],
  );
} catch (error) {
  columnStaleDiagnostic = error.diagnostic;
}
assert(columnStaleDiagnostic?.code === "stale_action", "column_four stale submission returns typed diagnostic");

const columnFullCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 14n),
  ["column_four"],
);
for (let index = 0; index < 6; index += 1) {
  invoke(
    (args) =>
      wasm.rulepath_apply_action(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
        BigInt(index),
      ),
    [columnFullCreated.match_id, index % 2 === 0 ? "seat_0" : "seat_1", "drop/c1"],
  );
}
let fullColumnDiagnostic = null;
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
        6n,
      ),
    [columnFullCreated.match_id, "seat_0", "drop/c1"],
  );
} catch (error) {
  fullColumnDiagnostic = error.diagnostic;
}
assert(fullColumnDiagnostic?.code === "full_column", "column_four full column returns typed diagnostic");

const columnBotCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 15n),
  ["column_four"],
);
const columnAfterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [columnBotCreated.match_id, "seat_0"],
);
assert(columnAfterBot.view.ply_count === 1, "column_four bot turn applies a Rust-selected drop");
assert(columnAfterBot.effects.some((effect) => effect.payload.type === "bot_chose_action"), "column_four bot emits semantic bot-choice effect");
assert(columnAfterBot.effects.some((effect) => typeof effect.payload.rationale === "string"), "column_four bot rationale is public prose");

const columnEffects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [columnBotCreated.match_id],
);
assert(columnEffects.some((entry) => entry.effect.payload.type === "bot_chose_action"), "column_four effect log returns bot effect");

const columnExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [columnBotCreated.match_id],
);
assert(columnExportedReplay.game_id === "column_four", "column_four export_replay preserves game id");
assert(columnExportedReplay.rules_version === "column_four-rules-v1", "column_four export_replay preserves rules version");
assert(columnExportedReplay.expected_replay_hashes.final, "column_four export includes replay hash");

const columnImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(columnExportedReplay)],
);
assert(columnImportedReplay.game_id === "column_four", "column_four import_replay preserves game id");
assert(columnImportedReplay.command_count === columnExportedReplay.commands.length, "column_four import preserves command count");

const columnReplayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [columnImportedReplay.replay_id],
);
assert(columnReplayReset.cursor === 0, "column_four replay reset returns cursor zero");
assert(columnReplayReset.view.ply_count === 0, "column_four replay reset projects initial state");

const columnReplayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [columnImportedReplay.replay_id],
);
assert(columnReplayStep.cursor === 1, "column_four replay step advances to requested cursor");
assert(columnReplayStep.view.ply_count === 1, "column_four replay step projects applied drop");

console.log(
  JSON.stringify({
    version,
    operations: featureReport.operations.length,
    games: catalog.length,
    match_id: created.match_id,
    three_marks_match_id: threeCreated.match_id,
    column_four_match_id: columnCreated.match_id,
    effects: effects.length,
    diagnostic: staleDiagnostic.code,
    column_diagnostic: columnStaleDiagnostic.code,
    replay_cursor: replayStep.cursor,
    column_replay_cursor: columnReplayStep.cursor,
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
