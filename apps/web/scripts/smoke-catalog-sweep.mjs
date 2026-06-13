import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import ts from "typescript";

const GENERIC_ONLY_GAMES = [
  "race_to_n",
  "three_marks",
  "column_four",
  "directional_flip",
  "draughts_lite",
  "frontier_control",
  "high_card_duel",
  "masked_claims",
  "plain_tricks",
  "poker_lite",
  "secret_draft",
  "token_bazaar",
];
const ADOPTER_GAMES = ["event_frontier", "flood_watch"];

const burstsUrl = await transpileModuleUrl("apps/web/src/animation/bursts.ts");
const schedulerUrl = await transpileModuleUrl("apps/web/src/animation/scheduler.ts", new Map([["./bursts", burstsUrl]]));
const effectFeedbackUrl = await transpileModuleUrl("apps/web/src/components/effectFeedback.ts");
const presentersUrl = await transpileModuleUrl(
  "apps/web/src/animation/presenters.ts",
  new Map([
    ["../components/effectFeedback", effectFeedbackUrl],
    ["./scheduler", schedulerUrl],
  ]),
);
const registryUrl = await transpileModuleUrl(
  "apps/web/src/animation/registry.ts",
  new Map([
    ["./scheduler", schedulerUrl],
    ["./presenters", presentersUrl],
  ]),
);
const settleAssertionUrl = await transpileModuleUrl("apps/web/src/animation/settleAssertion.ts");

const { EffectAnimationScheduler } = await import(schedulerUrl);
const { createAnimationRegistry } = await import(registryUrl);
const { assertSettledView } = await import(settleAssertionUrl);

const registry = createAnimationRegistry();
for (const gameId of GENERIC_ONLY_GAMES) {
  assert.equal(registry.has(gameId, "action_completed"), false, `${gameId} has no authored action_completed override`);
  const calls = [];
  const root = animationRoot(calls);
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 0,
    reducedMotionDurationMs: 0,
    presenter: (step) => registry.resolve(gameId, step, { root, reducedMotion: false }),
  });

  await scheduler.enqueueEffects([
    {
      cursor: 1,
      visibility: { scope: "public" },
      effect: {
        payload: { type: "action_completed" },
      },
    },
  ]);
  assert(calls.length > 0, `${gameId} generic presentation animated a baseline target`);
}

const seededGap = assertSettledView(
  {
    querySelector(selector) {
      if (selector === ".animation-ghost") {
        return {};
      }
      if (selector === '[data-testid="race-to-n-board"]') {
        return {};
      }
      return null;
    },
    getAnimations() {
      return [];
    },
  },
  { game_id: "race_to_n" },
);
assert.equal(seededGap.ok, false, "settle assertion reports a seeded coverage gap");
assert(seededGap.issues.some((issue) => issue.code === "lingering_ghost"), "seeded gap reports lingering ghost coverage");

const readme = await readFile("apps/web/README.md", "utf8");
const matrix = readme.slice(
  readme.indexOf("### Effect Animation Adoption Audit"),
  readme.indexOf("## Smoke Layers"),
);
for (const gameId of [...GENERIC_ONLY_GAMES, ...ADOPTER_GAMES]) {
  assert(matrix.includes(`| \`${gameId}\` |`), `adoption matrix includes ${gameId}`);
}
assert.equal((matrix.match(/\| `[^`]+` \| (adopt|generic-only|board-native mapping|not applicable) \|/g) ?? []).length, 14, "adoption matrix has 14 classified rows");

console.log(JSON.stringify({ catalogSweep: "ok", genericOnly: GENERIC_ONLY_GAMES.length, adopters: ADOPTER_GAMES.length }));

function animationRoot(calls) {
  const target = {
    animate(keyframes, timing) {
      calls.push({ keyframes, timing });
      return {
        playState: "finished",
        playbackRate: 1,
        finish() {
          this.playState = "finished";
        },
      };
    },
  };
  return {
    querySelector() {
      return target;
    },
    getAnimations() {
      return [];
    },
  };
}

async function transpileModuleUrl(path, replacements = new Map()) {
  const source = await readFile(path, "utf8");
  let { outputText } = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ES2022,
      target: ts.ScriptTarget.ES2022,
      jsx: ts.JsxEmit.ReactJSX,
      verbatimModuleSyntax: true,
    },
  });
  if (path.endsWith("effectFeedback.ts")) {
    outputText = outputText.replace('import { useEffect, useState } from "react";', "const useEffect = () => undefined; const useState = () => [undefined, () => undefined];");
  }
  for (const [specifier, url] of replacements) {
    outputText = outputText.replaceAll(`from "${specifier}"`, `from "${url}"`);
  }
  return `data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`;
}
