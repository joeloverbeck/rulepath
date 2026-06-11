import { readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import * as esbuild from "esbuild";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const artifactPath = join(rootDir, "public", "wasm_api.wasm");
const feedbackBundlePath = join(tmpdir(), `rulepath-effect-feedback-${process.pid}.mjs`);

await esbuild.build({
  entryPoints: [join(rootDir, "src", "components", "effectFeedback.ts")],
  outfile: feedbackBundlePath,
  bundle: true,
  platform: "node",
  format: "esm",
  logLevel: "silent",
});

const { feedbackForEffect } = await import(pathToFileURL(feedbackBundlePath).href);

try {
  const bytes = await readFile(artifactPath);
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const wasm = instance.exports;
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();
  const entries = [];
  const observed = new Map();
  let cursor = 0;

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

  function newMatch(gameId, seed) {
    return invoke(
      (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, BigInt(seed)),
      [gameId],
    );
  }

  function getView(matchId) {
    return invoke((args) => wasm.rulepath_get_view(args[0].ptr, args[0].len), [matchId]);
  }

  function getActionTree(matchId, actor) {
    return invoke(
      (args) =>
        wasm.rulepath_get_action_tree_for_viewer(
          args[0].ptr,
          args[0].len,
          args[1].ptr,
          args[1].len,
          args[2].ptr,
          args[2].len,
        ),
      [matchId, actor, actor],
    );
  }

  function applyAction(matchId, actor, path, freshnessToken) {
    return invoke(
      (args) =>
        wasm.rulepath_apply_action(
          args[0].ptr,
          args[0].len,
          args[1].ptr,
          args[1].len,
          args[2].ptr,
          args[2].len,
          BigInt(freshnessToken),
        ),
      [matchId, actor, path],
    );
  }

  function firstLeafPath(choices, prefix = []) {
    for (const choice of choices) {
      const path = [...prefix, choice.segment];
      const child = choice.next?.choices ?? [];
      if (child.length === 0) {
        return path;
      }
      const nested = firstLeafPath(child, path);
      if (nested.length > 0) {
        return nested;
      }
    }
    return [];
  }

  function activeSeat(view, fallback = "seat_0") {
    return view.active_seat ?? view.active_actor ?? view.current_actor ?? fallback;
  }

  function recordEffects(gameId, effects) {
    for (const effect of effects) {
      const entry = { cursor: ++cursor, effect };
      entries.push({ gameId, entry });
      const type = effect.payload?.type ?? "<missing>";
      const games = observed.get(type) ?? new Set();
      games.add(gameId);
      observed.set(type, games);
    }
  }

  function playFirstLegal(matchId, gameId, view) {
    const actor = activeSeat(view);
    const tree = getActionTree(matchId, actor);
    const path = firstLeafPath(tree.choices);
    assert(path.length > 0, `${gameId} exposes at least one legal path for ${actor}`);
    const result = applyAction(matchId, actor, path.join(">"), tree.freshness_token);
    recordEffects(gameId, result.effects);
    return result.view;
  }

  function playGame(gameId, seed, turns) {
    const created = newMatch(gameId, seed);
    let view = getView(created.match_id);
    for (let turn = 0; turn < turns && activeSeat(view, null); turn += 1) {
      view = playFirstLegal(created.match_id, gameId, view);
    }
  }

  const catalog = invoke(() => wasm.rulepath_list_games(), []);
  const turnCounts = new Map([
    ["high_card_duel", 2],
    ["plain_tricks", 12],
    ["secret_draft", 2],
    ["masked_claims", 2],
  ]);

  for (const [index, game] of catalog.entries()) {
    playGame(game.game_id, 101 + index, turnCounts.get(game.game_id) ?? 1);
  }

  assert(entries.length > 0, "effect feedback smoke collected emitted effects");

  const sentinelPattern = /\b(?:undefined|null|NaN)\b|\[object Object\]/;
  for (const { gameId, entry } of entries) {
    const feedback = feedbackForEffect(entry);
    const rendered = `${feedback.title}\n${feedback.detail}`;
    assert(
      !sentinelPattern.test(rendered),
      `${gameId} ${entry.effect.payload.type} rendered unprojected data: ${JSON.stringify(feedback)}`,
    );
  }

  const requiredCoverage = [
    ["masked_claims", "claim_turn_advanced"],
    ["masked_claims", "claim_score_changed"],
    ["plain_tricks", "round_scored"],
    ["high_card_duel", "round_scored"],
  ];

  for (const [gameId, type] of requiredCoverage) {
    assert(
      observed.get(type)?.has(gameId),
      `effect feedback smoke did not observe ${gameId} ${type}`,
    );
  }

  console.log(
    JSON.stringify({
      smoke: "effect-feedback",
      effects_checked: entries.length,
      effect_types: [...observed.keys()].sort(),
      required: requiredCoverage.map(([gameId, type]) => `${gameId}:${type}`),
    }),
  );
} finally {
  await rm(feedbackBundlePath, { force: true });
}
