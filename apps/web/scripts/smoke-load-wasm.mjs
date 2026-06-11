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
  "rulepath_get_view_for_viewer",
  "rulepath_get_action_tree",
  "rulepath_get_action_tree_for_viewer",
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
assert(catalog.some((game) => game.game_id === "directional_flip"), "list_games includes directional_flip");
assert(catalog.some((game) => game.game_id === "draughts_lite"), "list_games includes draughts_lite");
assert(
  catalog.some(
    (game) => game.game_id === "token_bazaar" && game.variants.includes("token_bazaar_standard"),
  ),
  "list_games includes token_bazaar standard variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "secret_draft" &&
      game.variants.includes("secret_draft_standard") &&
      game.hidden_information === true,
  ),
  "list_games includes secret_draft standard hidden-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "plain_tricks" &&
      game.variants.includes("plain_tricks_standard") &&
      game.hidden_information === true &&
      game.tags.includes("trick_taking"),
  ),
  "list_games includes plain_tricks standard hidden-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "masked_claims" &&
      game.variants.includes("masked_claims_standard") &&
      game.hidden_information === true &&
      game.tags.includes("reaction_window"),
  ),
  "list_games includes masked_claims standard hidden-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "flood_watch" &&
      game.variants.includes("flood_watch_standard") &&
      game.variants.includes("flood_watch_deluge") &&
      game.hidden_information === true &&
      game.cooperative === true,
  ),
  "list_games includes flood_watch cooperative hidden-information variants",
);

const maskedCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 11n),
  ["masked_claims"],
);
assert(maskedCreated.variant_id === "masked_claims_standard", "masked_claims starts standard variant");
const maskedObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [maskedCreated.match_id],
);
assert(maskedObserver.game_id === "masked_claims", "masked_claims observer view is game-specific");
assert(maskedObserver.private_view.status === "observer", "masked_claims observer view is public");
assert(JSON.stringify(maskedObserver).includes("mask_g") === false, "masked_claims observer hides hand and reserve ids");
const maskedSeat0 = invoke(
  (args) => wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [maskedCreated.match_id, "seat_0"],
);
assert(maskedSeat0.private_view.own_hand.length > 0, "masked_claims seat view includes own hand");
const maskedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [maskedCreated.match_id, "seat_0"],
);
const claimChoice = maskedTree.choices.find((choice) => choice.segment === "claim");
assert(claimChoice?.next?.choices?.length > 0, "masked_claims claim tree exposes own mask choices");
const maskChoice = claimChoice.next.choices[0];
const declaredChoice = maskChoice.next.choices.find((choice) => choice.segment === "5") ?? maskChoice.next.choices[0];
const maskedClaimPath = `claim>${maskChoice.segment}>${declaredChoice.segment}`;
const maskedAfterClaim = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(maskedTree.freshness_token),
    ),
  [maskedCreated.match_id, "seat_0", maskedClaimPath],
);
assert(maskedAfterClaim.view.phase.includes("reaction"), "masked_claims claim opens reaction window");
assert(JSON.stringify(maskedAfterClaim.view).includes(maskChoice.segment) === false, "masked_claims pending pedestal is redacted");
const maskedBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [maskedCreated.match_id, "seat_1"],
);
assert(maskedBot.policy_id === "masked-claims-level1-v1", "masked_claims bot reports policy id");
assert(JSON.stringify(maskedBot).includes("reserve") === false, "masked_claims bot payload omits reserve references");
const maskedExport = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [maskedCreated.match_id],
);
assert(maskedExport.game_id === "masked_claims", "masked_claims export is game-specific");
assert(JSON.stringify(maskedExport).includes("claim/mask_g") === false, "masked_claims export redacts claim tile path");

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

const directionalCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 17n),
  ["directional_flip"],
);
assert(directionalCreated.match_id, "directional_flip new_match returns a match id");
assert(
  directionalCreated.variant_id === "directional_flip_standard",
  "directional_flip starts standard variant",
);

const directionalView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [directionalCreated.match_id],
);
assert(directionalView.game_id === "directional_flip", "directional_flip view is game-specific");
assert(
  directionalView.variant_id === "directional_flip_standard",
  "directional_flip view reports standard variant",
);
assert(
  directionalView.board_rows === 8 && directionalView.board_columns === 8,
  "directional_flip projects an 8x8 board",
);
assert(directionalView.cells.length === 64, "directional_flip projects 64 cells");
assert(directionalView.legal_targets.length > 0, "directional_flip exposes legal targets");
assert(
  directionalView.legal_targets.some((target) => target.preview?.ordered_flip_cells.length > 0),
  "directional_flip legal targets include Rust preview flips",
);
assert(
  directionalView.private_view_status === "not_applicable_perfect_information",
  "directional_flip private view is explicitly not applicable",
);
assert(directionalView.hidden_fields.length === 0, "directional_flip exposes no hidden fields");

const directionalTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [directionalCreated.match_id, "seat_0"],
);
const directionalPlacement = directionalTree.choices.find((choice) =>
  choice.segment.startsWith("place/"),
);
assert(directionalPlacement, "directional_flip action tree exposes placement actions");

const directionalAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(directionalTree.freshness_token),
    ),
  [directionalCreated.match_id, "seat_0", directionalPlacement.segment],
);
assert(directionalAfterHuman.view.ply_count === 1, "directional_flip human placement advances ply");
assert(
  directionalAfterHuman.effects.some((effect) => effect.payload.type === "disc_placed"),
  "directional_flip emits semantic disc placement effect",
);
assert(
  directionalAfterHuman.effects.some((effect) => effect.payload.type === "discs_flipped"),
  "directional_flip emits semantic flip effect",
);

let directionalStaleDiagnostic = null;
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
        BigInt(directionalTree.freshness_token),
      ),
    [directionalCreated.match_id, "seat_1", directionalPlacement.segment],
  );
} catch (error) {
  directionalStaleDiagnostic = error.diagnostic;
}
assert(
  directionalStaleDiagnostic?.code === "stale_action",
  "directional_flip stale submission returns typed diagnostic",
);

const directionalAfterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [directionalCreated.match_id, directionalAfterHuman.view.active_seat],
);
assert(directionalAfterBot.view.ply_count === 2, "directional_flip bot turn applies a Rust-selected placement");
assert(
  directionalAfterBot.effects.some((effect) => effect.payload.type === "bot_chose_action"),
  "directional_flip bot emits a semantic bot-choice effect",
);
assert(
  directionalAfterBot.effects.some((effect) => typeof effect.payload.rationale === "string"),
  "directional_flip bot rationale is public prose",
);

const directionalEffects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [directionalCreated.match_id],
);
assert(
  directionalEffects.some((entry) => entry.effect.payload.type === "bot_chose_action"),
  "directional_flip effect log returns bot effect",
);

const directionalExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [directionalCreated.match_id],
);
assert(
  directionalExportedReplay.game_id === "directional_flip",
  "directional_flip export_replay preserves game id",
);
assert(
  directionalExportedReplay.rules_version === "directional_flip-rules-v1",
  "directional_flip export_replay preserves rules version",
);
assert(directionalExportedReplay.expected_replay_hashes.final, "directional_flip export includes replay hash");
assert(
  !JSON.stringify(directionalExportedReplay).includes("initial_snapshot"),
  "directional_flip export omits internal replay snapshots",
);

const directionalImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(directionalExportedReplay)],
);
assert(
  directionalImportedReplay.game_id === "directional_flip",
  "directional_flip import_replay preserves game id",
);
assert(
  directionalImportedReplay.command_count === directionalExportedReplay.commands.length,
  "directional_flip import preserves command count",
);

const directionalReplayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [directionalImportedReplay.replay_id],
);
assert(directionalReplayReset.cursor === 0, "directional_flip replay reset returns cursor zero");
assert(directionalReplayReset.view.ply_count === 0, "directional_flip replay reset projects initial state");

const directionalReplayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [directionalImportedReplay.replay_id],
);
assert(directionalReplayStep.cursor === 1, "directional_flip replay step advances to requested cursor");
assert(directionalReplayStep.view.ply_count === 1, "directional_flip replay step projects applied placement");

const draughtsCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 23n),
  ["draughts_lite"],
);
assert(draughtsCreated.match_id, "draughts_lite new_match returns a match id");
assert(draughtsCreated.variant_id === "draughts_lite_standard", "draughts_lite starts standard variant");

const draughtsView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [draughtsCreated.match_id],
);
assert(draughtsView.game_id === "draughts_lite", "draughts_lite view is game-specific");
assert(draughtsView.variant_id === "draughts_lite_standard", "draughts_lite view reports standard variant");
assert(draughtsView.board_rows === 8 && draughtsView.board_columns === 8, "draughts_lite projects an 8x8 board");
assert(draughtsView.cells.length === 64, "draughts_lite projects 64 cells");
assert(draughtsView.cells.filter((cell) => cell.occupancy === "occupied").length === 24, "draughts_lite standard setup has 24 pieces");
assert(draughtsView.private_view_status === "not_applicable_perfect_information", "draughts_lite private view is explicitly not applicable");
assert(draughtsView.hidden_fields.length === 0, "draughts_lite exposes no hidden fields");

const draughtsTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [draughtsCreated.match_id, "seat_0"],
);
const draughtsPath = firstCompletePath(draughtsTree.choices);
assert(draughtsPath.length >= 2, "draughts_lite action tree exposes multi-segment paths");
assert(draughtsPath[0].startsWith("from/"), "draughts_lite first segment selects an origin");
assert(draughtsPath[1].startsWith("to/") || draughtsPath[1].startsWith("jump/"), "draughts_lite second segment selects a landing");

const draughtsAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(draughtsTree.freshness_token),
    ),
  [draughtsCreated.match_id, "seat_0", draughtsPath.join(">")],
);
assert(draughtsAfterHuman.view.ply_count === 1, "draughts_lite human move advances ply");
assert(
  draughtsAfterHuman.effects.some((effect) => effect.payload.type === "move_committed"),
  "draughts_lite emits semantic move effect",
);

const draughtsAfterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [draughtsCreated.match_id, draughtsAfterHuman.view.active_seat],
);
assert(draughtsAfterBot.view.ply_count === 2, "draughts_lite bot turn applies a Rust-selected move");
assert(
  draughtsAfterBot.effects.some((effect) => effect.payload.type === "bot_chose_action"),
  "draughts_lite bot emits a semantic bot-choice effect",
);

const draughtsEffects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [draughtsCreated.match_id],
);
assert(
  draughtsEffects.some((entry) => entry.effect.payload.type === "bot_chose_action"),
  "draughts_lite effect log returns bot effect",
);

const draughtsExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [draughtsCreated.match_id],
);
assert(draughtsExportedReplay.game_id === "draughts_lite", "draughts_lite export_replay preserves game id");
assert(
  draughtsExportedReplay.rules_version === "draughts_lite-rules-v1",
  "draughts_lite export_replay preserves rules version",
);
assert(
  Array.isArray(draughtsExportedReplay.commands[0].action_path) &&
    draughtsExportedReplay.commands[0].action_path.length >= 2,
  "draughts_lite export preserves ordered multi-segment action paths",
);
assert(draughtsExportedReplay.expected_replay_hashes.final, "draughts_lite export includes replay hash");
assert(
  !JSON.stringify(draughtsExportedReplay).includes("initial_snapshot"),
  "draughts_lite export omits internal replay snapshots",
);

const draughtsImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(draughtsExportedReplay)],
);
assert(draughtsImportedReplay.game_id === "draughts_lite", "draughts_lite import_replay preserves game id");
assert(
  draughtsImportedReplay.command_count === draughtsExportedReplay.commands.length,
  "draughts_lite import preserves command count",
);

const draughtsReplayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [draughtsImportedReplay.replay_id],
);
assert(draughtsReplayReset.cursor === 0, "draughts_lite replay reset returns cursor zero");
assert(draughtsReplayReset.view.ply_count === 0, "draughts_lite replay reset projects initial state");

const draughtsReplayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [draughtsImportedReplay.replay_id],
);
assert(draughtsReplayStep.cursor === 1, "draughts_lite replay step advances to requested cursor");
assert(draughtsReplayStep.view.ply_count === 1, "draughts_lite replay step projects applied move");

const secretCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 31n),
  ["secret_draft"],
);
assert(secretCreated.match_id, "secret_draft new_match returns a match id");
assert(secretCreated.variant_id === "secret_draft_standard", "secret_draft starts standard variant");

const secretObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [secretCreated.match_id],
);
assert(secretObserver.game_id === "secret_draft", "secret_draft view is game-specific");
assert(secretObserver.private_view.status === "observer", "secret_draft observer view is redacted");
assert(secretObserver.commitments.seat_0.committed === false, "secret_draft starts without commitments");

const secretTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [secretCreated.match_id, "seat_0"],
);
const secretCommit = secretTree.choices.find((choice) => choice.segment.startsWith("commit/"));
assert(secretCommit, "secret_draft action tree exposes commit actions to actor");

const secretAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(secretTree.freshness_token),
    ),
  [secretCreated.match_id, "seat_0", secretCommit.segment],
);
assert(secretAfterHuman.view.commitments.seat_0.committed === true, "secret_draft commit updates pending state");
assert(
  secretAfterHuman.effects.some((effect) => effect.payload.type === "own_commit_accepted"),
  "secret_draft actor receives private commit confirmation without item id",
);
assert(
  !JSON.stringify(secretAfterHuman.effects).includes(`${secretCommit.segment}"`),
  "secret_draft first commit effect does not echo committed item path",
);

const secretObserverEffects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [secretCreated.match_id],
);
assert(
  secretObserverEffects.some((entry) => entry.effect.payload.type === "commitment_placed"),
  "secret_draft observer receives face-down commitment effect",
);
assert(
  secretObserverEffects.some((entry) => entry.effect.payload.type === "own_commit_accepted"),
  "secret_draft observer receives item-free commit acknowledgement",
);
assert(
  !JSON.stringify(secretObserverEffects).includes(`${secretCommit.segment}"`),
  "secret_draft observer effects do not echo committed item path",
);

const secretAfterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [secretCreated.match_id, "seat_1"],
);
assert(secretAfterBot.effects.some((effect) => effect.payload.type === "choices_revealed"), "secret_draft second commit reveals batch");
assert(!JSON.stringify(secretAfterBot).includes("candidate"), "secret_draft bot output avoids candidate internals");
assert(!JSON.stringify(secretAfterBot).includes("debug"), "secret_draft bot output avoids debug internals");

const secretExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [secretCreated.match_id],
);
assert(secretExportedReplay.game_id === "secret_draft", "secret_draft export_replay preserves game id");
assert(secretExportedReplay.export_class === "viewer_scoped_observation_v1", "secret_draft export is viewer-scoped");
assert(secretExportedReplay.viewer === "observer", "secret_draft export defaults to observer");
assert(!JSON.stringify(secretExportedReplay).includes('"commands"'), "secret_draft public export omits command stream");
assert(!JSON.stringify(secretExportedReplay).includes('"seed_evidence"'), "secret_draft public export omits seed evidence");

const secretImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(secretExportedReplay)],
);
assert(secretImportedReplay.game_id === "secret_draft", "secret_draft import_replay preserves game id");
assert(secretImportedReplay.public_export === true, "secret_draft import marks public export");

const secretReplayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [secretImportedReplay.replay_id],
);
assert(secretReplayReset.public_export === true, "secret_draft replay reset uses public timeline");

const plainCreated = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 37n),
  ["plain_tricks"],
);
assert(plainCreated.match_id, "plain_tricks new_match returns a match id");
assert(plainCreated.variant_id === "plain_tricks_standard", "plain_tricks starts standard variant");

const plainObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [plainCreated.match_id],
);
assert(plainObserver.game_id === "plain_tricks", "plain_tricks view is game-specific");
assert(plainObserver.private_view.status === "observer", "plain_tricks observer view is redacted");
assert(plainObserver.hand_counts.seat_0 === 6, "plain_tricks exposes hand counts to observer");

const plainSeat0 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [plainCreated.match_id, "seat_0"],
);
assert(plainSeat0.private_view.status === "seat", "plain_tricks seat viewer receives private view");
assert(plainSeat0.private_view.own_hand.length === 6, "plain_tricks seat view includes own hand");

const plainUnauthorizedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [plainCreated.match_id, "seat_0", "seat_1"],
);
assert(plainUnauthorizedTree.choices.length === 0, "plain_tricks non-actor tree is redacted");

const plainTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [plainCreated.match_id, "seat_0", "seat_0"],
);
const plainPath = firstCompletePath(plainTree.choices);
assert(plainPath[0] === "play" && plainPath[1], "plain_tricks action tree exposes card play paths");

const plainAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(plainTree.freshness_token),
    ),
  [plainCreated.match_id, "seat_0", plainPath.join(">")],
);
assert(plainAfterHuman.view.current_trick.plays.length === 1, "plain_tricks card play reaches Rust view");
assert(
  plainAfterHuman.effects.some((effect) => effect.payload.type === "card_played"),
  "plain_tricks emits semantic card-play effect",
);
for (const card of plainSeat0.private_view.own_hand.filter((card) => card.card_id !== plainPath[1])) {
  assert(
    !JSON.stringify(plainObserver).includes(card.card_id),
    `plain_tricks observer omits unplayed private card ${card.card_id}`,
  );
}

const plainExportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [plainCreated.match_id],
);
assert(plainExportedReplay.game_id === "plain_tricks", "plain_tricks export_replay preserves game id");
assert(
  plainExportedReplay.export_class === "viewer_scoped_observation_v1",
  "plain_tricks export is viewer-scoped",
);
assert(plainExportedReplay.viewer === "observer", "plain_tricks export defaults to observer");
assert(!JSON.stringify(plainExportedReplay).includes('"commands"'), "plain_tricks public export omits command stream");
assert(!JSON.stringify(plainExportedReplay).includes('"seed_evidence"'), "plain_tricks public export omits seed evidence");
for (const card of plainSeat0.private_view.own_hand.filter((card) => card.card_id !== plainPath[1])) {
  assert(
    !JSON.stringify(plainExportedReplay).includes(card.card_id),
    `plain_tricks export omits unplayed private card ${card.card_id}`,
  );
}

const plainImportedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(plainExportedReplay)],
);
assert(plainImportedReplay.game_id === "plain_tricks", "plain_tricks import_replay preserves game id");
assert(plainImportedReplay.public_export === true, "plain_tricks import marks public export");

const plainReplayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [plainImportedReplay.replay_id],
);
assert(plainReplayReset.public_export === true, "plain_tricks replay reset uses public timeline");

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
    directional_diagnostic: directionalStaleDiagnostic.code,
    replay_cursor: replayStep.cursor,
    column_replay_cursor: columnReplayStep.cursor,
    directional_replay_cursor: directionalReplayStep.cursor,
    draughts_lite_match_id: draughtsCreated.match_id,
    draughts_lite_replay_cursor: draughtsReplayStep.cursor,
    secret_draft_match_id: secretCreated.match_id,
    secret_draft_public_export: secretImportedReplay.public_export,
    plain_tricks_match_id: plainCreated.match_id,
    plain_tricks_public_export: plainImportedReplay.public_export,
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

function firstCompletePath(choices, prefix = []) {
  for (const choice of choices) {
    const path = [...prefix, choice.segment];
    if (choice.next?.choices?.length) {
      const nested = firstCompletePath(choice.next.choices, path);
      if (nested.length > 0) {
        return nested;
      }
    } else {
      return path;
    }
  }
  return [];
}
