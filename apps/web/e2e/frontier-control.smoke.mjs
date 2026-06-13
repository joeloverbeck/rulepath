import { createReadStream } from "node:fs";
import { access, readFile } from "node:fs/promises";
import http from "node:http";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import puppeteer from "puppeteer";

const __dirname = dirname(fileURLToPath(import.meta.url));
const distDir = join(__dirname, "..", "dist");
const mountPath = "/rulepath/";
const executablePath = process.env.PUPPETEER_EXECUTABLE_PATH || "/usr/bin/google-chrome";
const forbiddenTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "candidate_ranking",
  "bot_candidate",
  "deck_order",
  "event_deck",
];

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
  browser = await puppeteer.launch({
    executablePath,
    headless: "new",
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  const page = await browser.newPage();
  const consoleMessages = [];
  page.on("console", (message) => consoleMessages.push(message.text()));
  page.on("pageerror", (error) => consoleMessages.push(error.message));
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);

  await startFrontierControl(page, baseUrl, "Hotseat", 41);
  await assertFrontierBoardA11y(page);
  await assertNoLeak(page, consoleMessages, "initial frontier board");
  await assertRenderedTextView(page, (view) => view?.game_id === "frontier_control" && view.active_seat === "seat_1");
  await waitForText(page, "March Base Camp to Ford");
  await waitForText(page, "End turn");

  await clickText(page, "button", "March Base Camp to Ford");
  await waitForText(page, "Crew marched");
  await clickText(page, "button", "Stake Ford");
  await waitForText(page, "Stake placed");
  await waitForText(page, "supplied");
  await assertNoLeak(page, consoleMessages, "prospector march and stake");

  await clickText(page, "button", "Patrol Gatehouse to Ford");
  await waitForText(page, "Clash resolved");
  await clickText(page, "button", "Dismantle Ford");
  await waitForText(page, "Stake dismantled");
  await clickText(page, "button", "End turn");
  await waitForText(page, "Round scored");
  await assertNoLeak(page, consoleMessages, "garrison patrol clash scoring");

  await clickText(page, "button", "Export Current Run");
  const replayText = await replayTextareaValue(page);
  assert(replayText.includes('"game_id": "frontier_control"'), "export keeps frontier_control game id");
  assertNoForbiddenTerms(replayText, "frontier_control public replay export", forbiddenTerms);
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Replay viewer");
  await waitForText(page, "Cursor 0 /");
  await waitForText(page, "Frontier Control");
  await assertNoLeak(page, consoleMessages, "frontier replay viewer");

  await startFrontierControl(page, baseUrl, "Human vs bot", 41);
  await waitForRenderedView(page, (view) => view?.game_id === "frontier_control" && view.freshness_token > 0);
  await waitForText(page, "Turn ended");
  await assertNoLeak(page, consoleMessages, "human_vs_bot prospector bot turn");

  await startFrontierControl(page, baseUrl, "Bot vs bot", 7);
  await stepBotUntilSeat(page, "seat_0");
  const beforeGarrisonBot = await textView(page);
  await clickText(page, "button", "Step Bot");
  await waitForFreshnessGreaterThan(page, beforeGarrisonBot.freshness_token);
  await assertNoLeak(page, consoleMessages, "bot_vs_bot garrison bot step");
  await clickText(page, "button", "Export Current Run");
  const botReplayText = await replayTextareaValue(page);
  assert(botReplayText.includes('"game_id": "frontier_control"'), "bot-vs-bot export keeps frontier_control game id");
  assertNoForbiddenTerms(botReplayText, "bot-vs-bot frontier export", forbiddenTerms);

  await startFrontierControl(page, baseUrl, "Hotseat", 0);
  await playProspectorWin(page);
  await waitForText(page, "Prospectors wins the frontier");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertRenderedTextView(page, (view) => view?.status?.includes("Prospectors wins the frontier"));
  await assertNoLeak(page, consoleMessages, "prospector terminal outcome");

  await startFrontierControl(page, baseUrl, "Hotseat", 0);
  await playEndTurnsUntilTerminal(page);
  await waitForText(page, "Garrison wins the frontier");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertRenderedTextView(page, (view) => view?.status?.includes("Garrison wins the frontier"));
  await assertNoLeak(page, consoleMessages, "garrison terminal outcome");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".frontier-control-board.reduced");
  const animationName = await page.$eval(".frontier-control-board.reduced .frontier-site", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses frontier board animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".frontier-layout");
  const columns = await page.$eval(".frontier-layout", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive frontier layout remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenTerms);

  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state?.view?.game_id === "frontier_control", "render_game_to_text reports frontier_control view");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "frontier_control asymmetric graph noleak replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startFrontierControl(page, baseUrl, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Frontier Control");
  await clickText(page, "button", "Frontier Control");
  await page.$eval(
    ".field input[type='number']",
    (input, value) => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
      setter?.call(input, String(value));
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    seed,
  );
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="frontier-control-board"]');
}

async function assertFrontierBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="frontier-control-board"]')),
      sites: document.querySelectorAll(".frontier-site").length,
      trails: document.querySelectorAll(".frontier-trail").length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".plain-latest")?.textContent ?? "",
      actionFamilies: Array.from(document.querySelectorAll(".frontier-action-group h3")).map((element) => element.textContent ?? ""),
    };
  });
  assert(summary.board, "frontier_control board renders");
  assert(summary.sites === 7, `frontier_control renders seven sites, got ${summary.sites}`);
  assert(summary.trails >= 10, `frontier_control renders trail lines, got ${summary.trails}`);
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "frontier_control latest-effect region has text");
  assert(summary.actionFamilies.some((label) => label.includes("march")), "prospector action group renders from Rust tree");
}

async function playProspectorWin(page) {
  await clickText(page, "button", "March Base Camp to Timberline");
  await clickText(page, "button", "March Timberline to Goldfield");
  await clickText(page, "button", "End turn");
  await clickText(page, "button", "Stake Goldfield");
  await clickText(page, "button", "End turn");
  await playEndTurnsUntilTerminal(page);
}

async function playEndTurnsUntilTerminal(page, maxTurns = 24) {
  for (let turn = 0; turn < maxTurns; turn += 1) {
    const view = await textView(page);
    if (view?.status?.includes("wins the frontier")) {
      return;
    }
    await clickText(page, "button", "End turn");
    await sleep(60);
  }
  const finalState = await page.evaluate(() => (window.render_game_to_text ? window.render_game_to_text() : document.body.textContent));
  throw new Error(`Frontier Control did not reach terminal after end-turn script: ${finalState.slice(0, 1200)}`);
}

async function waitForRenderedView(page, predicate) {
  await page.waitForFunction(
    (predicateSource) => {
      if (!window.render_game_to_text) return false;
      const state = JSON.parse(window.render_game_to_text());
      return Function("view", `return (${predicateSource})(view);`)(state.view);
    },
    {},
    predicate.toString(),
  );
}

async function waitForFreshnessGreaterThan(page, freshnessToken) {
  await page.waitForFunction(
    (minimum) => {
      if (!window.render_game_to_text) return false;
      const state = JSON.parse(window.render_game_to_text());
      return state?.view?.game_id === "frontier_control" && state.view.freshness_token > minimum;
    },
    {},
    freshnessToken,
  );
}

async function stepBotUntilSeat(page, seat, maxSteps = 8) {
  for (let step = 0; step < maxSteps; step += 1) {
    const view = await textView(page);
    if (view?.active_seat === seat) {
      return;
    }
    await clickText(page, "button", "Step Bot");
    await sleep(80);
  }
  const finalView = await textView(page);
  throw new Error(`Frontier Control bot-vs-bot did not reach ${seat}: ${JSON.stringify(finalView)}`);
}

async function textView(page) {
  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  return state?.view ?? null;
}

async function assertRenderedTextView(page, predicate) {
  const ok = await page.evaluate((predicateSource) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return Function("view", `return (${predicateSource})(view);`)(state?.view);
  }, predicate.toString());
  assert(ok, "render_game_to_text view satisfied expected Frontier condition");
}

async function assertNoLeak(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, forbiddenTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbiddenTerms);
}

async function assertStorageClean(page) {
  const storage = await page.evaluate(() => ({
    local: Object.fromEntries(Object.entries(localStorage)),
    session: Object.fromEntries(Object.entries(sessionStorage)),
  }));
  const allowedLocal = Object.keys(storage.local).every((key) => key === "rulepath.reducedMotion");
  assert(allowedLocal, `localStorage only stores UI motion preference: ${JSON.stringify(storage.local)}`);
  assert(Object.keys(storage.session).length === 0, `sessionStorage remains empty: ${JSON.stringify(storage.session)}`);
}

async function fullBrowserSurface(page) {
  return page.evaluate(() =>
    [
      document.body.textContent ?? "",
      Array.from(document.querySelectorAll("*"))
        .flatMap((element) => Array.from(element.attributes).map((attribute) => `${attribute.name}=${attribute.value}`))
        .join("\n"),
      Object.keys(localStorage).join("\n"),
      Object.values(localStorage).join("\n"),
      Object.keys(sessionStorage).join("\n"),
      Object.values(sessionStorage).join("\n"),
    ].join("\n"),
  );
}

async function replayTextareaValue(page) {
  const handle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  return handle.jsonValue();
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

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function assertNoForbiddenTerms(value, label, terms) {
  const lower = value.toLowerCase();
  const hits = terms.filter((term) => lower.includes(term.toLowerCase()));
  assert(hits.length === 0, `${label} contains forbidden terms: ${hits.join(", ")}`);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
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
