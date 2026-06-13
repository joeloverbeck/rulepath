import assert from "node:assert/strict";
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
    const parsed = JSON.parse(output());
    if (status !== 0) {
      throw new Error(parsed.message);
    }
    return parsed;
  } finally {
    for (const arg of args) {
      wasm.rulepath_dealloc(arg.ptr, arg.len);
    }
  }
}

function newMatch(seed) {
  return invoke((args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, BigInt(seed)), ["race_to_n"]);
}

function view(matchId) {
  return invoke((args) => wasm.rulepath_get_view(args[0].ptr, args[0].len), [matchId]);
}

function effects(matchId, cursor) {
  return invoke((args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, BigInt(cursor), 0, 0), [matchId]);
}

function bot(matchId, actor, seed) {
  return invoke((args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, BigInt(seed)), [matchId, actor]);
}

function replay(matchId) {
  return invoke((args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len), [matchId]);
}

function botSeed(publicView) {
  return publicView.freshness_token + (publicView.active_seat === "seat_0" ? 101 : 211);
}

function runImmediate() {
  const created = newMatch(77);
  let current = view(created.match_id);
  for (let index = 0; index < 2 && !current.winner; index += 1) {
    current = bot(created.match_id, current.active_seat, botSeed(current)).view;
  }
  return replay(created.match_id);
}

function runSchedulerShape() {
  const created = newMatch(77);
  let current = view(created.match_id);
  let cursor = 0;
  for (let index = 0; index < 2 && !current.winner; index += 1) {
    effects(created.match_id, cursor);
    current = bot(created.match_id, current.active_seat, botSeed(current)).view;
    const nextEffects = effects(created.match_id, cursor);
    cursor = nextEffects.reduce((latest, entry) => Math.max(latest, entry.cursor), cursor);
  }
  return replay(created.match_id);
}

const immediateReplay = runImmediate();
const schedulerReplay = runSchedulerShape();
assert.deepEqual(schedulerReplay.commands, immediateReplay.commands, "bot_vs_bot command logs are byte-identical");
assert.equal(JSON.stringify(normalizedReplay(schedulerReplay)), JSON.stringify(normalizedReplay(immediateReplay)), "bot_vs_bot replay exports are byte-identical except export handle");

console.log(JSON.stringify({ commands: immediateReplay.commands.length, replay: "byte-identical" }));

function normalizedReplay(document) {
  return {
    ...document,
    trace_id: "normalized-export-id",
  };
}
