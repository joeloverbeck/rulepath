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
  "full_deck_order",
  "event_deck",
  "deck_order",
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
  "seed_evidence",
  "candidate_ranking",
  "bot_candidate",
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

  await startFloodWatch(page, baseUrl, "Hotseat", 41);
  await assertFloodWatchA11y(page);
  await assertNoLeak(page, consoleMessages, "initial hotseat");
  await waitForText(page, "Pumpwright");
  await waitForText(page, "Levee Warden");
  await clickText(page, "button", "Forecast");
  await waitForText(page, "Forecast revealed");
  await assertFloodWatchA11y(page);
  await assertNoLeak(page, consoleMessages, "forecast reveal");
  await clickFirstFloodAction(page, "Reinforce");
  await waitForText(page, "Levee placed");
  await assertNoLeak(page, consoleMessages, "reinforce action");
  await clickText(page, "button", "End turn");
  await waitForText(page, "Storm card drawn");
  await assertFloodWatchTurnReport(page);
  await waitForText(page, "Seat 1");
  await assertNoLeak(page, consoleMessages, "environment phase");

  await clickText(page, "button", "Export Current Run");
  const replayText = await replayTextareaValue(page);
  assert(replayText.includes('"game_id": "flood_watch"'), "export keeps flood_watch game id");
  assert(replayText.includes('"viewer": "observer"'), "export is observer scoped");
  assert(replayText.includes('"redacted_command_summary"'), "export includes redacted command summaries");
  assert(!replayText.includes('"commands"'), "public export omits raw command stream");
  assertNoForbiddenTerms(replayText, "flood_watch public replay export", forbiddenTerms);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await assertReplayViewerNoLeak(page);

  await startFloodWatch(page, baseUrl, "Human vs bot", 41);
  await clickText(page, "button", "End turn");
  await waitForRenderedView(page, (view) => view?.game_id === "flood_watch" && view.freshness_token > 1);
  await page.waitForFunction(() =>
    ["Levee placed", "District bailed", "Forecast revealed", "Storm card drawn"].some((text) =>
      document.body.textContent?.includes(text),
    ),
  );
  await assertNoLeak(page, consoleMessages, "human_vs_bot teammate turn");

  await startFloodWatch(page, baseUrl, "Bot vs bot", 27);
  await clickText(page, "button", "Step Bot");
  await waitForRenderedView(page, (view) => view?.game_id === "flood_watch" && view.freshness_token > 0);
  await assertNoLeak(page, consoleMessages, "bot_vs_bot first step");
  await clickText(page, "button", "Export Current Run");
  const botReplayText = await replayTextareaValue(page);
  assert(botReplayText.includes('"game_id": "flood_watch"'), "bot-vs-bot export keeps flood_watch game id");
  assertNoForbiddenTerms(botReplayText, "bot-vs-bot public replay export", forbiddenTerms);

  await startFloodWatch(page, baseUrl, "Bot vs bot", 1);
  await stepBotUntilTerminal(page, (status) => status.includes("Shared win"));
  await waitForText(page, "Shared win");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertNoLeak(page, consoleMessages, "shared win terminal");

  await startFloodWatch(page, baseUrl, "Hotseat", 15);
  await clickText(page, "button", "End turn");
  await waitForText(page, "Shared loss");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertNoLeak(page, consoleMessages, "shared loss terminal");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".flood-watch-board.reduced");
  const animationName = await page.$eval(".flood-watch-board.reduced .plain-seat", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses flood_watch board animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".plain-tricks-table");
  const columns = await page.$eval(".plain-tricks-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive flood_watch table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenTerms);

  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state?.view?.game_id === "flood_watch", "render_game_to_text reports flood_watch view");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "flood_watch coop noleak replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startFloodWatch(page, baseUrl, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Flood Watch");
  await clickText(page, "button", "Flood Watch");
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
  await page.waitForSelector('[data-testid="flood-watch-board"]');
}

async function assertFloodWatchA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    const countText = document.querySelector('[data-testid="deck-face-down-count"]')?.textContent ?? "";
    const undrawnMetric =
      Array.from(document.querySelectorAll(".plain-tricks-metrics div")).find((item) => item.querySelector("span")?.textContent === "Undrawn")?.querySelector("strong")?.textContent ?? "";
    const disclosure = document.querySelector('[data-testid="deck-discard"]');
    if (disclosure instanceof HTMLDetailsElement) {
      disclosure.open = true;
    }
    return {
      board: Boolean(document.querySelector('[data-testid="flood-watch-board"]')),
      districts: document.querySelectorAll('[data-testid^="flood-watch-district-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      deckText: document.querySelector('[data-testid="deck-flow-panel"]')?.textContent ?? "",
      nextText: document.querySelector('[data-testid="deck-next-card"]')?.textContent ?? "",
      faceDownText: document.querySelector('[data-testid="deck-face-down"]')?.textContent ?? "",
      faceDownCount: countText,
      viewUndrawnCount: undrawnMetric,
      discardOpen: disclosure instanceof HTMLDetailsElement ? disclosure.open : false,
      discardText: disclosure?.textContent ?? "",
      liveText: document.querySelector(".plain-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "flood_watch board renders");
  assert(summary.districts === 5, "flood_watch renders five district gauges");
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.deckText.includes("Storm deck"), "flood_watch renders Rust deck label");
  assert(summary.nextText.includes("Forecast"), "flood_watch renders Rust forecast slot label");
  if (!summary.nextText.includes("None")) {
    assert(summary.nextText.includes("Downpour") || summary.nextText.includes("Storm Surge") || summary.nextText.includes("Reprieve"), "flood_watch renders authored forecast card label");
    assert(summary.nextText.includes("rises") || summary.nextText.includes("faces") || summary.nextText.includes("No district rises"), "flood_watch renders authored forecast summary");
  }
  assert(summary.faceDownText.includes("Remaining storm cards stay face down. The count is public."), "flood_watch renders Rust face-down summary");
  assert(summary.faceDownCount === summary.viewUndrawnCount, "flood_watch face-down count matches the public view");
  assert(summary.discardOpen, "flood_watch drawn-card disclosure expands");
  assert(summary.discardText.includes("Resolved storm cards"), "flood_watch drawn-card disclosure uses Rust label");
  assert(!summary.deckText.includes("downpour/") && !summary.deckText.includes("storm_surge/"), "flood_watch deck panel omits raw card ids");
  assert(summary.liveText.length > 0, "flood_watch latest-effect region has text");
}

async function assertFloodWatchTurnReport(page) {
  const report = await page.$eval('[data-testid="turn-report-panel"]', (element) => element.textContent ?? "");
  assert(report.includes("Turn report"), "flood_watch turn report renders near the board");
  assert(report.includes("Storm phase"), "flood_watch turn report includes the environment automation burst");
  assert(report.includes("Storm card drawn"), "flood_watch turn report narrates drawn storm cards");
  assert(!report.includes("event_deck") && !report.includes("deck_order"), "flood_watch turn report omits hidden deck terms");
}

async function clickFirstFloodAction(page, text) {
  const handle = await waitForTextHandle(page, ".flood-watch-board button", text);
  await handle.click();
}

async function assertReplayViewerNoLeak(page) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assertNoForbiddenTerms(surface, "flood_watch public replay viewer", forbiddenTerms);
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

async function stepBotUntilTerminal(page, terminalPredicate, maxSteps = 80) {
  for (let step = 0; step < maxSteps; step += 1) {
    const terminal = await page.evaluate((predicateSource) => {
      const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
      const terminalText = state?.view?.terminal?.outcome ?? state?.view?.status ?? "";
      return Function("terminalText", `return (${predicateSource})(terminalText);`)(terminalText);
    }, terminalPredicate.toString());
    if (terminal) {
      return;
    }
    await clickText(page, "button", "Step Bot");
    await sleep(100);
  }
  const finalState = await page.evaluate(() => (window.render_game_to_text ? window.render_game_to_text() : document.body.textContent));
  throw new Error(`Flood Watch bot-vs-bot did not reach the expected terminal state: ${finalState.slice(0, 1200)}`);
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
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
