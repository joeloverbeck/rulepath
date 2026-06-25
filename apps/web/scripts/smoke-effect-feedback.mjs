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
    if (gameId === "vow_tide") {
      return invoke(
        (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, BigInt(seed), 7),
        [gameId],
      );
    }
    if (gameId === "river_ledger") {
      return invoke(
        (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, BigInt(seed), 4),
        [gameId],
      );
    }
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

  function playRequiredSegment(matchId, gameId, view, segment) {
    const actor = activeSeat(view);
    const tree = getActionTree(matchId, actor);
    const choice = tree.choices.find((candidate) => candidate.segment === segment);
    assert(choice, `${gameId} exposes ${segment} for ${actor}`);
    const result = applyAction(matchId, actor, segment, tree.freshness_token);
    recordEffects(gameId, result.effects);
    return result.view;
  }

  function playPreferredSegment(matchId, gameId, view, preferredSegment) {
    const actor = activeSeat(view);
    const tree = getActionTree(matchId, actor);
    const choice = tree.choices.find((candidate) => candidate.segment === preferredSegment) ?? tree.choices[0];
    assert(choice, `${gameId} exposes a legal path for ${actor}`);
    const result = applyAction(matchId, actor, choice.segment, tree.freshness_token);
    recordEffects(gameId, result.effects);
    return result.view;
  }

  function playFrontierControl(matchId) {
    let view = getView(matchId);
    view = playRequiredSegment(matchId, "frontier_control", view, "march/site_base_camp/site_ford");
    view = playRequiredSegment(matchId, "frontier_control", view, "stake/site_ford");
    view = playRequiredSegment(matchId, "frontier_control", view, "patrol/site_gatehouse/site_ford");
    view = playRequiredSegment(matchId, "frontier_control", view, "dismantle/site_ford");
    view = playRequiredSegment(matchId, "frontier_control", view, "muster");
    view = playPreferredSegment(matchId, "frontier_control", view, "end_turn");
    view = playRequiredSegment(matchId, "frontier_control", view, "reinforce/site_gatehouse");
    for (let turn = 0; turn < 30 && view.terminal?.kind === "non_terminal"; turn += 1) {
      view = playPreferredSegment(matchId, "frontier_control", view, "end_turn");
    }
    assert(view.terminal?.kind !== "non_terminal", "frontier_control reaches terminal in effect feedback smoke");
  }

  function playEventFrontier(matchId) {
    let view = getView(matchId);
    for (let turn = 0; turn < 12 && view.active_seat; turn += 1) {
      const tree = getActionTree(matchId, view.active_seat);
      const preferred =
        tree.choices.find((choice) => choice.segment === "event") ??
        tree.choices.find((choice) => choice.segment === "pass") ??
        tree.choices[0];
      assert(preferred, "event_frontier exposes a legal event/pass/operation choice");
      const path = preferred.next?.choices?.length ? firstLeafPath([preferred]) : [preferred.segment];
      const result = applyAction(matchId, view.active_seat, path.join(">"), tree.freshness_token);
      recordEffects("event_frontier", result.effects);
      view = result.view;
    }
  }

  function playRiverLedgerShowdown(matchId) {
    let view = getView(matchId);
    for (const segment of [
      "call",
      "call",
      "call",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
      "check",
    ]) {
      if (!activeSeat(view, null) || view.terminal?.terminal) {
        break;
      }
      view = playRequiredSegment(matchId, "river_ledger", view, segment);
    }
    assert(view.terminal?.terminal === true, "river_ledger reaches terminal showdown in effect feedback smoke");
  }

  function playGame(gameId, seed, turns) {
    const created = newMatch(gameId, seed);
    let view = getView(created.match_id);
    if (gameId === "flood_watch") {
      const actor = activeSeat(view);
      const tree = getActionTree(created.match_id, actor);
      const endTurn = tree.choices.find((choice) => choice.segment === "end_turn");
      assert(endTurn, "flood_watch exposes end_turn for storm feedback smoke");
      const result = applyAction(created.match_id, actor, "end_turn", tree.freshness_token);
      recordEffects(gameId, result.effects);
      return;
    }
    if (gameId === "frontier_control") {
      playFrontierControl(created.match_id);
      return;
    }
    if (gameId === "event_frontier") {
      playEventFrontier(created.match_id);
      return;
    }
    if (gameId === "river_ledger") {
      playRiverLedgerShowdown(created.match_id);
      return;
    }
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
    ["flood_watch", 2],
    ["frontier_control", 2],
    ["vow_tide", 1],
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

  // Shared effect fallbacks must stay game-neutral: a fallback that hardcodes one
  // game's name leaks the wrong public copy into every other game that emits the
  // same effect type without a Rust-supplied summary (e.g. blackglass_pact's
  // match_completed previously rendered "Rust finalized Vow Tide standings.").
  const fallbackNeutralityCases = [
    { type: "match_completed", payload: { type: "match_completed" } },
  ];
  const gameNamePattern =
    /\b(?:vow tide|blackglass pact|race to|three marks|column four|directional flip|draughts lite|high card duel|masked claims|flood watch|frontier control|event frontier|token bazaar|secret draft|veiled draft|poker lite|crest ledger|plain tricks|river ledger|briar circuit)\b/i;
  for (const fallbackCase of fallbackNeutralityCases) {
    const feedback = feedbackForEffect({ cursor: 0, effect: { payload: fallbackCase.payload } });
    assert(
      !gameNamePattern.test(feedback.detail),
      `${fallbackCase.type} fallback must stay game-neutral, got: ${JSON.stringify(feedback)}`,
    );
  }

  const requiredCoverage = [
    ["masked_claims", "claim_turn_advanced"],
    ["masked_claims", "claim_score_changed"],
    ["plain_tricks", "round_scored"],
    ["high_card_duel", "round_scored"],
    ["flood_watch", "event_drawn"],
    ["frontier_control", "crew_marched"],
    ["frontier_control", "stake_placed"],
    ["frontier_control", "guard_patrolled"],
    ["frontier_control", "clash_resolved"],
    ["frontier_control", "stake_dismantled"],
    ["frontier_control", "crew_mustered"],
    ["frontier_control", "guard_reinforced"],
    ["frontier_control", "round_scored"],
    ["frontier_control", "terminal"],
    ["event_frontier", "choice_taken"],
    ["event_frontier", "event_resolved"],
    ["river_ledger", "river_ledger_contribution_changed"],
    ["river_ledger", "river_ledger_street_advanced"],
    ["river_ledger", "river_ledger_showdown_resolved"],
    ["vow_tide", "bid_accepted"],
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
