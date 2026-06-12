import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import ts from "typescript";

const schedulerUrl = await transpileModuleUrl("apps/web/src/animation/scheduler.ts", new Map([["./bursts", await transpileModuleUrl("apps/web/src/animation/bursts.ts")]]));
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

const { createAnimationRegistry } = await import(registryUrl);
const { genericPresentationKind } = await import(presentersUrl);

const step = (type, extra = {}) => ({
  burst: { id: "burst", label: "Burst", markerKind: "initial", marker: null, entries: [], visibleEntries: [] },
  entry: {
    cursor: 1,
    effect: {
      payload: {
        type,
        ...extra,
      },
    },
  },
  durationMs: 180,
  reducedMotion: false,
});

assert.equal(genericPresentationKind(step("resource_collected")), "move", "movement tone maps to move presentation");
assert.equal(genericPresentationKind(step("turn_changed")), "turn-banner", "turn tone maps to turn banner presentation");
assert.equal(genericPresentationKind(step("terminal", { outcome: "won", summary: "done" })), "terminal-settle", "terminal tone maps to settle presentation");
assert.equal(genericPresentationKind(step("action_completed")), "highlight", "neutral tone maps to highlight presentation");
assert.equal(genericPresentationKind(step("commit_face_down")), "redacted", "face-down/redacted effect maps to generic redacted presentation");
assert.equal(genericPresentationKind(step("custom_hidden", { redacted: true })), "redacted", "redacted payload flag maps to generic redacted presentation");

const registry = createAnimationRegistry();
assert.equal(registry.has("event_frontier", "event_resolved"), false, "registry starts without game override");
registry.register("event_frontier", "event_resolved", () => ({ done: Promise.resolve() }));
assert.equal(registry.has("event_frontier", "event_resolved"), true, "registry records game override");
const resolved = await registry.resolve("event_frontier", step("event_resolved"));
assert(resolved.done instanceof Promise, "registry override resolves instead of generic presentation");

console.log(JSON.stringify({ presenters: "ok" }));

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
