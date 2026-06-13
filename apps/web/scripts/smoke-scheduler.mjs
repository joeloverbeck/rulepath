import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { mock } from "node:test";
import ts from "typescript";

const burstsUrl = await transpileModuleUrl("apps/web/src/animation/bursts.ts");
const schedulerUrl = await transpileModuleUrl("apps/web/src/animation/scheduler.ts", new Map([["./bursts", burstsUrl]]));
const { EffectAnimationScheduler } = await import(schedulerUrl);

const entry = (cursor, type, extra = {}) => ({
  cursor,
  effect: {
    payload: {
      type,
      ...extra,
    },
  },
});

mock.timers.enable({ apis: ["setTimeout"] });

try {
  await assertOrderedDrain();
  await assertFlushFinishesAndSettles();
  await assertRateScale();
  await assertReducedMotion();
  await assertActivityObservable();
} finally {
  mock.timers.reset();
}

console.log(JSON.stringify({ scheduler: "ok" }));

async function assertOrderedDrain() {
  const seen = [];
  let settled = 0;
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 40,
    presenter: (step) => {
      seen.push(step.entry.cursor);
    },
    settle: () => {
      settled += 1;
    },
  });
  const drained = scheduler.enqueueEffects([
    entry(1, "action_started", { actor: "seat_0" }),
    entry(2, "counter_advanced"),
    entry(3, "turn_changed"),
  ]);
  await flushMicrotasks();
  assert.deepEqual(seen, [2], "first step starts immediately");
  mock.timers.tick(40);
  await flushMicrotasks();
  assert.deepEqual(seen, [2, 3], "second step follows first dwell");
  mock.timers.tick(40);
  await drained;
  assert.deepEqual(seen, [2, 3], "ordered visible entries drain");
  assert.equal(settled, 1, "settle hook runs after drain");
}

async function assertFlushFinishesAndSettles() {
  let finished = 0;
  let settled = 0;
  const animation = {
    playbackRate: 1,
    finish: () => {
      finished += 1;
    },
  };
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 1000,
    animationSource: {
      getAnimations: () => [animation],
    },
    presenter: () => ({ animations: [animation] }),
    settle: () => {
      settled += 1;
    },
  });
  const drained = scheduler.enqueueEffects([entry(1, "action_started"), entry(2, "counter_advanced"), entry(3, "turn_changed")]);
  await flushMicrotasks();
  await scheduler.flush();
  assert.equal(finished, 2, "flush finishes source and in-flight animations");
  assert.equal(scheduler.pendingSteps, 0, "flush drains queued steps");
  assert.equal(settled, 1, "flush settles immediately");
  mock.timers.tick(1000);
  await drained;
}

async function assertRateScale() {
  const seen = [];
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 100,
    rate: 2,
    presenter: (step) => {
      seen.push(step.entry.cursor);
    },
  });
  const drained = scheduler.enqueueEffects([entry(1, "action_started"), entry(2, "counter_advanced"), entry(3, "turn_changed")]);
  await flushMicrotasks();
  assert.deepEqual(seen, [2], "rate test starts first step");
  mock.timers.tick(49);
  await flushMicrotasks();
  assert.deepEqual(seen, [2], "rate-scaled dwell has not elapsed early");
  mock.timers.tick(1);
  await flushMicrotasks();
  assert.deepEqual(seen, [2, 3], "rate halves dwell duration");
  mock.timers.tick(50);
  await drained;
}

async function assertReducedMotion() {
  const seen = [];
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 100,
    reducedMotionDurationMs: 20,
    reducedMotion: true,
    presenter: (step) => {
      seen.push({ cursor: step.entry.cursor, reducedMotion: step.reducedMotion, durationMs: step.durationMs });
    },
  });
  const drained = scheduler.enqueueEffects([entry(1, "action_started"), entry(2, "counter_advanced"), entry(3, "turn_changed")]);
  await flushMicrotasks();
  assert.deepEqual(seen, [{ cursor: 2, reducedMotion: true, durationMs: 20 }], "reduced-motion step is flagged and collapsed");
  mock.timers.tick(20);
  await flushMicrotasks();
  assert.equal(seen.length, 2, "reduced-motion dwell uses fast duration");
  mock.timers.tick(20);
  await drained;
}

async function assertActivityObservable() {
  const activity = [];
  const scheduler = new EffectAnimationScheduler({
    defaultDurationMs: 30,
    presenter: () => undefined,
  });
  const unsubscribe = scheduler.subscribeActivity((active) => {
    activity.push(active);
  });
  assert.deepEqual(activity, [false], "activity subscription reports initial idle state");

  const drained = scheduler.enqueueEffects([entry(1, "action_started"), entry(2, "counter_advanced"), entry(3, "turn_changed")]);
  await flushMicrotasks();
  assert.equal(scheduler.active, true, "scheduler reports active during drain");
  assert(activity.includes(true), "activity subscription reports active drain");

  mock.timers.tick(30);
  await flushMicrotasks();
  mock.timers.tick(30);
  await drained;
  assert.equal(scheduler.active, false, "scheduler reports idle after drain");
  assert.equal(activity.at(-1), false, "activity subscription reports idle after settle");

  unsubscribe();
  const countAfterUnsubscribe = activity.length;
  const secondDrain = scheduler.enqueueEffects([entry(4, "counter_advanced")]);
  await flushMicrotasks();
  mock.timers.tick(30);
  await secondDrain;
  assert.equal(activity.length, countAfterUnsubscribe, "unsubscribe stops activity updates");
}

async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

async function transpileModuleUrl(path, replacements = new Map()) {
  const source = await readFile(path, "utf8");
  let { outputText } = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ES2022,
      target: ts.ScriptTarget.ES2022,
      verbatimModuleSyntax: true,
    },
  });
  for (const [specifier, url] of replacements) {
    outputText = outputText.replaceAll(`from "${specifier}"`, `from "${url}"`);
  }
  return `data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`;
}
