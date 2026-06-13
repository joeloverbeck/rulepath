import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import ts from "typescript";

const source = await readFile("apps/web/src/animation/bursts.ts", "utf8");
const { outputText } = ts.transpileModule(source, {
  compilerOptions: {
    module: ts.ModuleKind.ES2022,
    target: ts.ScriptTarget.ES2022,
    verbatimModuleSyntax: true,
  },
});
const moduleUrl = `data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`;
const { isDecisionMarker, latestResolutionBurst, segmentResolutionBursts } = await import(moduleUrl);

const entry = (cursor, type, extra = {}) => ({
  cursor,
  effect: {
    payload: {
      type,
      ...extra,
    },
  },
});

const effects = [
  entry(1, "action_started", { actor: "seat_0" }),
  entry(2, "resource_collected", { seat: "seat_0", amount: 1 }),
  entry(3, "turn_changed", { next_actor: "seat_1" }),
  entry(4, "bot_chose_action_public", { policy_id: "level1" }),
  entry(5, "event_drawn", { event: "storm_surge" }),
  entry(6, "environment_phase_began", { turn: 2 }),
  entry(7, "flood_level_rose", { district: "market" }),
];

const bursts = segmentResolutionBursts(effects);
assert.equal(bursts.length, 3, "human, bot, and automated markers create three bursts");
assert.deepEqual(
  bursts.map((burst) => burst.markerKind),
  ["human_action", "bot_action", "automated_phase"],
  "bursts preserve marker kinds",
);
assert.deepEqual(
  bursts.map((burst) => burst.visibleEntries.map((visible) => visible.cursor)),
  [[2, 3], [5], [7]],
  "visible entries exclude decision markers and stay in marker-delimited bursts",
);
assert.equal(latestResolutionBurst(effects)?.visibleEntries[0]?.cursor, 7, "latest burst selects the automated phase");
assert.equal(isDecisionMarker(entry(10, "choice_taken")), true, "choice_taken is a decision marker");
assert.equal(isDecisionMarker(entry(11, "turn_changed")), false, "turn_changed is visible burst content");

const leading = segmentResolutionBursts([entry(20, "counter_advanced"), entry(21, "turn_changed")]);
assert.equal(leading.length, 1, "leading effects without a marker form an initial burst");
assert.equal(leading[0].markerKind, "initial", "leading burst is explicitly initial");
assert.deepEqual(
  leading[0].visibleEntries.map((visible) => visible.cursor),
  [20, 21],
  "leading burst keeps visible entries",
);

console.log(JSON.stringify({ bursts: bursts.length, latest: latestResolutionBurst(effects)?.label }));
