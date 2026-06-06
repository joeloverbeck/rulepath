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
  }),
);
