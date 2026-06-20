import { createReadStream } from "node:fs";
import { access, readFile } from "node:fs/promises";
import http from "node:http";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { launchBrowser } from "./launch.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const distDir = join(__dirname, "..", "dist");
const mountPath = "/rulepath/";
const executablePath = process.env.PUPPETEER_EXECUTABLE_PATH || "/usr/bin/google-chrome";
const expectedGames = [
  "Race to 21",
  "Three Marks",
  "Column Four",
  "Directional Flip",
  "Draughts Lite",
  "High Card Duel",
  "Token Bazaar",
  "Veiled Draft",
  "Crest Ledger",
  "Plain Tricks",
  "Masked Claims",
  "Flood Watch",
  "Frontier Control",
  "Event Frontier",
  "River Ledger",
  "Briar Circuit",
];
const staticForbiddenTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
  "seed_evidence",
  "bot_candidate",
  "candidate_ranking",
  "commands",
];
const hiddenGameForbiddenTerms = {
  "Crest Ledger": [
    "low_dawn",
    "low_dusk",
    "middle_dawn",
    "middle_dusk",
    "high_dawn",
    "high_dusk",
    "Sprout Dawn",
    "Sprout Dusk",
    "Current Dawn",
    "Current Dusk",
    "Crown Dawn",
    "Crown Dusk",
    '"rank":"low"',
    '"rank":"middle"',
    '"rank":"high"',
  ],
  "Veiled Draft": ["ember_1", "commit/ember_1"],
  "High Card Duel": ["hcd:r"],
};

await access(executablePath);

const server = http.createServer(async (request, response) => {
  if (!request.url?.startsWith(mountPath)) {
    response.writeHead(404);
    response.end("not found");
    return;
  }

  const relativeUrl = request.url.slice(mountPath.length).split("?")[0] || "index.html";
  const safePath = normalize(relativeUrl).replace(/^(\.\.[/\\])+/, "");
  const filePath = join(distDir, safePath);

  try {
    await readFile(filePath);
    response.writeHead(200, { "Content-Type": contentTypeFor(filePath) });
    createReadStream(filePath).pipe(response);
  } catch {
    response.writeHead(200, { "Content-Type": "text/html" });
    createReadStream(join(distDir, "index.html")).pipe(response);
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));

let browser;
try {
  const { port } = server.address();
  const baseUrl = `http://127.0.0.1:${port}${mountPath}`;
  browser = await launchBrowser(executablePath);

  const page = await browser.newPage();
  const consoleMessages = [];
  page.on("console", (message) => consoleMessages.push(message.text()));
  page.on("pageerror", (error) => consoleMessages.push(error.message));
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Race to 21");
  await assertRulesTriggers(page);
  await assertPickerRulesFlow(page, "Race to 21");
  await assertPickerRulesFlow(page, "Crest Ledger");
  await assertPickerRulesFlow(page, "Plain Tricks");
  await assertPickerRulesFlow(page, "Event Frontier");
  await assertSetupRulesFlow(page, "Crest Ledger");
  await assertInPlayRulesFlow(page, "Crest Ledger", '[data-testid="poker-lite-board"]');
  await assertHiddenInfoRulesFlow(page, baseUrl, "Veiled Draft", "Hotseat", '[data-testid="secret-draft-board"]');
  await assertHiddenInfoRulesFlow(page, baseUrl, "High Card Duel", "Hotseat", '[data-testid="high-card-duel-board"]');

  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", staticForbiddenTerms);
  console.log(JSON.stringify({ browser: "puppeteer", smoke: "rules display a11y noleak" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function assertRulesTriggers(page) {
  const summary = await page.evaluate(() =>
    Array.from(document.querySelectorAll('.game-card button[aria-label^="How to play "]')).map((button) => ({
      label: button.getAttribute("aria-label") ?? "",
      disabled: button.hasAttribute("disabled"),
      tabIndex: button.tabIndex,
      text: button.textContent ?? "",
    })),
  );
  assert(summary.length === expectedGames.length, `all games expose How to Play controls: ${JSON.stringify(summary)}`);
  for (const gameName of expectedGames) {
    const trigger = summary.find((button) => button.label === `How to play ${gameName}`);
    assert(Boolean(trigger), `${gameName} has a named How to Play trigger`);
    assert(!trigger.disabled, `${gameName} How to Play trigger is enabled`);
    assert(trigger.tabIndex >= 0, `${gameName} How to Play trigger is keyboard focusable`);
  }
}

async function assertPickerRulesFlow(page, gameName) {
  const trigger = await focusRulesTrigger(page, gameName);
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await assertRulesDialog(page, gameName);
  await assertFocusContained(page);
  await page.keyboard.press("Escape");
  await waitForNoDialog(page);
  await assertFocusReturnedTo(page, trigger);
}

async function assertSetupRulesFlow(page, gameName) {
  await clickText(page, "button", gameName);
  await waitForText(page, "Start Match");
  const trigger = await page.waitForSelector(".setup-summary .rules-trigger");
  await trigger.focus();
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await assertRulesDialog(page, gameName);
  await clickText(page, ".rules-panel button", "Close");
  await waitForNoDialog(page);
  await assertFocusReturnedTo(page, trigger);
}

async function assertInPlayRulesFlow(page, gameName, boardSelector) {
  await clickText(page, "button", "Start Match");
  await page.waitForSelector(boardSelector);
  await assertRulesDoesNotMutateMatch(page, gameName);
}

async function assertHiddenInfoRulesFlow(page, baseUrl, gameName, modeLabel, boardSelector) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, gameName);
  await clickText(page, "button", gameName);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector(boardSelector);
  await assertRulesDoesNotMutateMatch(page, gameName);
}

async function assertRulesDoesNotMutateMatch(page, gameName) {
  const beforeState = await captureMatchState(page);
  const beforeExport = await exportReplay(page);
  const effectsBefore = await effectsText(page);

  const trigger = await waitForTextHandle(page, ".mode-controls button", "Rules");
  await trigger.focus();
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await assertRulesDialog(page, gameName);
  await assertPanelStaticNoLeak(page, gameName);
  await clickText(page, ".rules-panel button", "Close");
  await waitForNoDialog(page);
  await assertFocusReturnedTo(page, trigger);

  const afterState = await captureMatchState(page);
  const effectsAfter = await effectsText(page);
  const afterExport = await exportReplay(page);

  assert(beforeState === afterState, `${gameName} public view JSON is unchanged by opening rules`);
  assert(effectsBefore === effectsAfter, `${gameName} effect log is unchanged by opening rules`);
  assert(beforeExport === afterExport, `${gameName} replay export is unchanged by opening rules`);
}

async function assertRulesDialog(page, gameName) {
  await page.waitForSelector('[role="dialog"][aria-modal="true"]');
  await page.waitForFunction(
    (name) => {
      const dialog = document.querySelector('[role="dialog"]');
      const text = dialog?.textContent ?? "";
      return (
        text.includes(`${name} Rules`) &&
        text.includes("At a glance") &&
        text.includes("Actions") &&
        text.includes("Scoring and winning")
      );
    },
    {},
    gameName,
  );
  const summary = await page.evaluate((name) => {
    const dialog = document.querySelector('[role="dialog"]');
    const labelledBy = dialog?.getAttribute("aria-labelledby") ?? "";
    const title = labelledBy ? document.getElementById(labelledBy)?.textContent ?? "" : "";
    return {
      title,
      text: dialog?.textContent ?? "",
      closeFocused: document.activeElement?.textContent?.includes("Close") ?? false,
      unnamedControls: Array.from(dialog?.querySelectorAll("button, a") ?? [])
        .filter((element) => !((element.getAttribute("aria-label") || element.textContent || "").trim()))
        .map((element) => element.outerHTML),
      ruleIdMentions: (dialog?.textContent ?? "").match(/Rule ID|Validation|LEG-|rule id/gi) ?? [],
    };
  }, gameName);
  assert(summary.title.includes(`${gameName} Rules`), `${gameName} rules dialog has a matching heading`);
  for (const expected of ["At a glance", "Actions", "Scoring and winning"]) {
    assert(summary.text.includes(expected), `${gameName} rules include ${expected}`);
  }
  assert(summary.closeFocused, `${gameName} rules dialog initially focuses Close`);
  assert(summary.unnamedControls.length === 0, `${gameName} rules dialog controls are named`);
  assert(summary.ruleIdMentions.length === 0, `${gameName} rules dialog omits rule-ID validation tables`);
  if (gameName === "Event Frontier") {
    await assertEventFrontierRulesRendering(page);
  }
}

async function assertEventFrontierRulesRendering(page) {
  const summary = await page.evaluate(() => {
    const dialog = document.querySelector('[role="dialog"]');
    return {
      text: dialog?.textContent ?? "",
      codeTexts: Array.from(dialog?.querySelectorAll("code") ?? []).map((element) => element.textContent ?? ""),
      emphasisTexts: Array.from(dialog?.querySelectorAll("em") ?? []).map((element) => element.textContent ?? ""),
    };
  });
  assert(summary.text.includes("Game ID: event_frontier"), "Event Frontier rules preserve underscored game ID text");
  assert(summary.codeTexts.includes("event_frontier"), "Event Frontier rules render event_frontier as one code span");
  assert(!summary.text.includes("seat_0"), "Event Frontier rules omit seat_0 from player-facing copy");
  assert(!summary.text.includes("seat_1"), "Event Frontier rules omit seat_1 from player-facing copy");
  assert(summary.text.includes("Costs and economy"), "Event Frontier rules include costs and economy section");
  assert(!summary.text.includes("Source notes for maintainers"), "Event Frontier rules omit maintainer source notes");
  assert(
    summary.emphasisTexts.every((text) => !text.includes("`event") && !text.includes("frontier`")),
    "Event Frontier rules do not split underscored identifiers into emphasis",
  );
}

async function assertFocusContained(page) {
  await assertFocusedVisible(page);
  await page.keyboard.down("Shift");
  await page.keyboard.press("Tab");
  await page.keyboard.up("Shift");
  await assertFocusInsideDialog(page, "Shift+Tab keeps focus in dialog");
  for (let i = 0; i < 8; i += 1) {
    await page.keyboard.press("Tab");
    await assertFocusInsideDialog(page, `Tab ${i + 1} keeps focus in dialog`);
  }
}

async function assertPanelStaticNoLeak(page, gameName) {
  const panelText = await page.$eval(".rules-panel", (element) =>
    [
      element.textContent ?? "",
      Array.from(element.querySelectorAll("*"))
        .flatMap((candidate) => Array.from(candidate.attributes).map((attribute) => `${attribute.name}=${attribute.value}`))
        .join("\n"),
    ].join("\n"),
  );
  assertNoForbiddenTerms(panelText, `${gameName} rules panel static text`, [
    ...staticForbiddenTerms,
    ...(hiddenGameForbiddenTerms[gameName] ?? []),
  ]);
}

async function captureMatchState(page) {
  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state, "render_game_to_text exposes current match state");
  return JSON.stringify({
    view: state.view,
    choices: state.choices,
    effects: state.effects,
  });
}

async function exportReplay(page) {
  await ensureDeveloperPanelOpen(page);
  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  return replayTextHandle.jsonValue();
}

async function ensureDeveloperPanelOpen(page) {
  const exportVisible = await page.evaluate(() =>
    Array.from(document.querySelectorAll("button")).some((button) => button.textContent?.includes("Export Current Run")),
  );
  if (!exportVisible) {
    await clickText(page, "button", "Developer panel");
    await waitForText(page, "Export Current Run");
  }
}

async function effectsText(page) {
  return page.evaluate(() => document.querySelector('[data-testid="effects"]')?.textContent ?? "");
}

async function focusRulesTrigger(page, gameName) {
  const handle = await page.waitForSelector(`button[aria-label="How to play ${gameName}"]`);
  await handle.focus();
  return handle;
}

async function assertFocusInsideDialog(page, label) {
  const inside = await page.evaluate(() => {
    const dialog = document.querySelector('[role="dialog"]');
    return Boolean(dialog && document.activeElement && dialog.contains(document.activeElement));
  });
  assert(inside, label);
}

async function assertFocusReturnedTo(page, expectedHandle) {
  await page.waitForFunction((element) => document.activeElement === element, {}, expectedHandle);
}

async function waitForNoDialog(page) {
  await page.waitForFunction(() => !document.querySelector('[role="dialog"]'));
}

async function assertFocusedVisible(page) {
  const focusStyle = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return null;
    const rect = element.getBoundingClientRect();
    const style = window.getComputedStyle(element);
    return {
      tag: element.tagName,
      text: element.textContent,
      ariaLabel: element.getAttribute("aria-label"),
      width: rect.width,
      height: rect.height,
      visibility: style.visibility,
      display: style.display,
      outlineWidth: style.outlineWidth,
      outlineStyle: style.outlineStyle,
    };
  });
  assert(Boolean(focusStyle), "focus target exists");
  assert(focusStyle.width > 0 && focusStyle.height > 0, `focused control is measurable: ${JSON.stringify(focusStyle)}`);
  assert(
    focusStyle.visibility !== "hidden" && focusStyle.display !== "none",
    `focused control is visible: ${JSON.stringify(focusStyle)}`,
  );
  assert(
    focusStyle.outlineStyle !== "none" || focusStyle.outlineWidth !== "0px",
    `focused control has visible focus: ${JSON.stringify(focusStyle)}`,
  );
}

async function clickText(page, selector, text) {
  const handle = await waitForTextHandle(page, selector, text);
  await handle.click();
}

async function clickLabel(page, text) {
  const handle = await waitForTextHandle(page, "label", text);
  await handle.click();
}

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
}

async function waitForTextHandle(page, selector, text) {
  await page.waitForFunction(
    (query, expected) =>
      Array.from(document.querySelectorAll(query)).some((element) => element.textContent?.includes(expected)),
    {},
    selector,
    text,
  );
  const handles = await page.$$(selector);
  for (const handle of handles) {
    const value = await handle.evaluate((element) => element.textContent ?? "");
    if (value.includes(text)) {
      return handle;
    }
  }
  throw new Error(`No ${selector} containing ${text}`);
}

function assertNoForbiddenTerms(value, label, terms) {
  const lower = value.toLowerCase();
  const hits = terms.filter((term) => lower.includes(term.toLowerCase()));
  assert(hits.length === 0, `${label} contains forbidden terms: ${hits.join(", ")}`);
}

function contentTypeFor(filePath) {
  switch (extname(filePath)) {
    case ".html":
      return "text/html";
    case ".js":
      return "text/javascript";
    case ".css":
      return "text/css";
    case ".wasm":
      return "application/wasm";
    default:
      return "application/octet-stream";
  }
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
