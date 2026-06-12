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

function newMatch(gameId, seed) {
  return invoke((args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, BigInt(seed)), [gameId]);
}

function actionTree(matchId, actor) {
  return invoke((args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len), [matchId, actor]);
}

function apply(matchId, actor, path, token) {
  return invoke(
    (args) => wasm.rulepath_apply_action(args[0].ptr, args[0].len, args[1].ptr, args[1].len, args[2].ptr, args[2].len, BigInt(token)),
    [matchId, actor, path],
  );
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

function runInline() {
  const created = newMatch("race_to_n", 91);
  const initial = view(created.match_id);
  const tree = actionTree(created.match_id, "seat_0");
  const choice = tree.choices.find((candidate) => candidate.segment === "add-1") ?? tree.choices[0];
  const afterHuman = apply(created.match_id, "seat_0", choice.segment, initial.freshness_token);
  bot(created.match_id, afterHuman.view.active_seat, botSeed(afterHuman.view));
  return replay(created.match_id);
}

function runOrchestratedShape() {
  const created = newMatch("race_to_n", 91);
  const initial = view(created.match_id);
  const tree = actionTree(created.match_id, "seat_0");
  const choice = tree.choices.find((candidate) => candidate.segment === "add-1") ?? tree.choices[0];
  apply(created.match_id, "seat_0", choice.segment, initial.freshness_token);
  const afterRefresh = view(created.match_id);
  effects(created.match_id, 0);
  bot(created.match_id, afterRefresh.active_seat, botSeed(afterRefresh));
  return replay(created.match_id);
}

const inlineReplay = runInline();
const orchestratedReplay = runOrchestratedShape();
assert.deepEqual(orchestratedReplay.commands, inlineReplay.commands, "command logs are byte-identical");
assert.equal(JSON.stringify(normalizedReplay(orchestratedReplay)), JSON.stringify(normalizedReplay(inlineReplay)), "replay exports are byte-identical except export handle");

console.log(JSON.stringify({ commands: inlineReplay.commands.length, replay: "byte-identical" }));

function normalizedReplay(document) {
  return {
    ...document,
    trace_id: "normalized-export-id",
  };
}
