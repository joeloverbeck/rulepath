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
    const raw = output();
    const parsed = JSON.parse(raw);
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

const version = read(
  wasm.rulepath_placeholder_version_ptr(),
  wasm.rulepath_placeholder_version_len(),
);
assert(version === "rulepath-wasm-api/0.1.0", "wasm artifact loads");

const created = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 1n),
  ["race_to_n"],
);
assert(created.match_id, "start match returns a match id");

const initialView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(initialView.counter === 0, "initial public view is visible");
assert(initialView.active_seat === "seat_0", "human seat starts");

const tree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [created.match_id, "seat_0"],
);
assert(tree.choices.some((choice) => choice.segment === "add-1"), "Rust choices are displayed");

const afterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      0n,
    ),
  [created.match_id, "seat_0", "add-1"],
);
assert(afterHuman.view.counter > 0, "human action advances the counter");

const afterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [created.match_id, "seat_1"],
);
assert(afterBot.view.active_seat === "seat_0" || afterBot.view.winner, "bot turn resolves");

const effects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [created.match_id],
);
assert(effects.length > 0, "semantic effects are available");

const exportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(exportedReplay.commands.length > 0, "run exports a replay command stream");

const importedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(exportedReplay)],
);
assert(importedReplay.replay_id, "replay import returns a replay handle");
const replayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [importedReplay.replay_id],
);
assert(replayReset.cursor === 0, "replay reset returns cursor zero");
const replayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [importedReplay.replay_id],
);
assert(replayStep.cursor === 1 && replayStep.view.counter > 0, "replay step returns Rust-projected view");

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
        0n,
      ),
    [created.match_id, "seat_0", "add-1"],
  );
} catch (error) {
  staleDiagnostic = error.diagnostic;
}
assert(staleDiagnostic?.code === "stale_action", "stale submission returns Rust diagnostic");

const catalog = invoke(() => wasm.rulepath_list_games(), []);
assert(catalog.some((game) => game.game_id === "race_to_n"), "Rust catalog includes race_to_n");
assert(catalog.some((game) => game.game_id === "three_marks"), "Rust catalog includes three_marks");
assert(
  catalog.some(
    (game) => game.game_id === "token_bazaar" && game.variants.includes("token_bazaar_standard"),
  ),
  "Rust catalog includes token_bazaar standard variant",
);

const threeMarks = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 4n),
  ["three_marks"],
);
assert(threeMarks.match_id, "three_marks start match returns a match id");
assert(threeMarks.variant_id === "three_marks_standard", "three_marks standard variant starts");
const threeView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [threeMarks.match_id],
);
assert(threeView.game_id === "three_marks", "three_marks Rust view is returned");
assert(threeView.variant_id === "three_marks_standard", "three_marks Rust view reports selected variant");

const tokenBazaar = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 9n),
  ["token_bazaar"],
);
assert(tokenBazaar.match_id, "token_bazaar start match returns a match id");
assert(tokenBazaar.variant_id === "token_bazaar_standard", "token_bazaar standard variant starts");
const tokenView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [tokenBazaar.match_id],
);
assert(tokenView.game_id === "token_bazaar", "token_bazaar Rust view is returned");
assert(tokenView.market_slots.length === 3, "token_bazaar projects three market slots");
assert(tokenView.legal_actions.some((choice) => choice.action_segment === "collect/amber"), "token_bazaar view exposes legal actions");
const tokenTree = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [tokenBazaar.match_id, "seat_0"],
);
assert(tokenTree.choices.some((choice) => choice.segment === "collect/amber"), "token_bazaar action tree exposes collect/amber");
const tokenAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(tokenTree.freshness_token),
    ),
  [tokenBazaar.match_id, "seat_0", "collect/amber"],
);
assert(tokenAfterHuman.view.supply.amber < tokenView.supply.amber, "token_bazaar collect updates public accounting");
assert(
  tokenAfterHuman.effects.some((effect) => effect.payload.type === "resource_collected"),
  "token_bazaar emits resource accounting effect",
);

const hotseat = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 2n),
  ["race_to_n"],
);
const hotseatSeat0 = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [hotseat.match_id, "seat_0"],
);
assert(hotseatSeat0.choices.length > 0, "hotseat seat_0 gets Rust choices");
invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      0n,
    ),
  [hotseat.match_id, "seat_0", hotseatSeat0.choices[0].segment],
);
const hotseatView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [hotseat.match_id],
);
const hotseatSeat1 = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [hotseat.match_id, hotseatView.active_seat],
);
assert(hotseatView.active_seat === "seat_1", "hotseat alternates to seat_1");
assert(hotseatSeat1.choices.length > 0, "hotseat active seat gets Rust choices");

const botVsBot = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 3n),
  ["race_to_n"],
);
const botStep0 = invoke(
  (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 300n),
  [botVsBot.match_id, "seat_0"],
);
assert(botStep0.view.counter > 0, "bot-vs-bot first step advances through Rust bot");
if (!botStep0.view.winner) {
  const botStep1 = invoke(
    (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 301n),
    [botVsBot.match_id, botStep0.view.active_seat],
  );
  assert(botStep1.view.counter >= botStep0.view.counter, "bot-vs-bot second step advances or holds terminal");
}

console.log(
  JSON.stringify({
    version,
    match_id: created.match_id,
    counter: afterBot.view.counter,
    effects: effects.length,
    diagnostic: staleDiagnostic.code,
    modes: ["human_vs_bot", "hotseat", "bot_vs_bot"],
    replay_cursor: replayStep.cursor,
    token_bazaar_match_id: tokenBazaar.match_id,
  }),
);
