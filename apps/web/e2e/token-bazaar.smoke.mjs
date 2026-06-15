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
const forbiddenLeakTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
  "candidate_ranking",
  "bot_candidate",
  "bot_explanation",
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
  browser = await launchBrowser(executablePath);

  const page = await browser.newPage();
  const consoleMessages = [];
  page.on("console", (message) => consoleMessages.push(message.text()));
  page.on("pageerror", (error) => consoleMessages.push(error.message));
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Token Bazaar");
  await clickText(page, "button", "Token Bazaar");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="token-bazaar-board"]');
  await assertBoardA11y(page);
  await assertCollectAmberGain(page);
  await assertInventoryChipsCompact(page);
  await assertNoLeak(page, consoleMessages, "initial token_bazaar DOM");

  await focusByTestId(page, "token-action-collect-amber");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForFreshness(page, 2);
  await waitForText(page, "Resources collected");
  await assertBotResponse(page);
  await assertNoLeak(page, consoleMessages, "after human and bot actions");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Public accounting");
  await waitForText(page, "collect/amber");
  await assertNoLeak(page, consoleMessages, "dev panel open DOM");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"game_id": "token_bazaar"'), "replay export keeps token_bazaar game id");
  assert(replayText.includes('"collect/amber"'), "replay export includes human action path");
  assert(replayText.includes('"expected_public_export_hashes"'), "replay export includes public export hash");
  assertNoForbiddenTerms(replayText, "token_bazaar replay export");

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector(".replay-board .placement-sequence");
  await clickText(page, ".replay-actions button", "Step");
  await waitForText(page, "Cursor 1 /");
  await assertReplayViewerNoLeak(page);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".token-bazaar-board.reduced");
  await page.waitForSelector(".effect-entry.reduced");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".token-market-row");
  const columns = await page.$eval(".token-market-row", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive market row remains measurable");

  await assertStorageClean(page);
  await assertNoLeak(page, consoleMessages, "final token_bazaar DOM");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "token_bazaar action bot replay a11y noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function assertBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    const resourceChips = Array.from(document.querySelectorAll(".resource-chip")).map((chip) => ({
      code: chip.querySelector("b")?.textContent?.trim() ?? "",
      name: chip.querySelector("span")?.textContent?.trim() ?? "",
      count: chip.querySelector("strong")?.textContent?.trim() ?? "",
    }));
    return {
      board: Boolean(document.querySelector('[data-testid="token-bazaar-board"]')),
      slots: document.querySelectorAll(".token-contract").length,
      actionButtons: document.querySelectorAll('[data-testid^="token-action-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      resourceChips,
      status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
      live: document.querySelector(".token-recent")?.textContent ?? "",
    };
  });
  assert(summary.board, "token_bazaar board renders");
  assert(summary.slots === 3, `token_bazaar renders three market slots, got ${summary.slots}`);
  assert(summary.actionButtons > 0, "token_bazaar exposes Rust action buttons");
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.status.length > 0, "turn status has text cue");
  assert(summary.live.length > 0, "recent accounting region has text cue");
  assert(
    summary.resourceChips.every((chip) => chip.code.length > 0 && chip.name.length > 0 && chip.count.length > 0),
    "resource chips include code, name, and numeric count",
  );
}

async function assertCollectAmberGain(page) {
  const gain = await page.$eval('[data-testid="token-action-collect-amber"] small', (element) =>
    Array.from(element.querySelectorAll(".resource-chip")).map((chip) => ({
      code: chip.querySelector("b")?.textContent?.trim() ?? "",
      name: chip.querySelector("span")?.textContent?.trim() ?? "",
      count: chip.querySelector("strong")?.textContent?.trim() ?? "",
    })),
  );
  assert(gain.length === 3, `collect amber action renders three gain chips, got ${gain.length}`);
  assert(
    JSON.stringify(gain) ===
      JSON.stringify([
        { code: "AM", name: "amber", count: "2" },
        { code: "JA", name: "jade", count: "0" },
        { code: "IR", name: "iron", count: "0" },
      ]),
    `collect amber gain renders Rust metadata amber 2 / jade 0 / iron 0: ${JSON.stringify(gain)}`,
  );
}

async function assertInventoryChipsCompact(page) {
  const grid = await page.$eval(".token-seat .resource-chips", (element) => {
    const columns = window.getComputedStyle(element).gridTemplateColumns.trim();
    return {
      columns,
      tracks: columns.length > 0 ? columns.split(/\s+/).length : 0,
    };
  });
  assert(grid.tracks === 1, `seat inventory resource chips use one compact grid track: ${JSON.stringify(grid)}`);
}

async function assertBotResponse(page) {
  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state?.view?.freshness_token >= 2, "human_vs_bot flow includes automatic Rust bot response");
  const effects = await page.evaluate(() =>
    Array.from(document.querySelectorAll('[data-testid="effects"] li')).map((element) => element.textContent ?? ""),
  );
  assert(effects.length >= 2, "effect log contains human and bot accounting effects");
}

async function assertReplayViewerNoLeak(page) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assert(surface.includes("collect/amber"), "replay viewer shows action path");
  assertNoForbiddenTerms(surface, "replay viewer");
}

async function assertNoLeak(page, consoleMessages, label) {
  const surface = await page.evaluate(() =>
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
  assertNoForbiddenTerms(surface, label);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`);
}

async function assertStorageClean(page) {
  const storage = await page.evaluate(() => ({
    local: Object.fromEntries(Array.from({ length: localStorage.length }, (_, index) => {
      const key = localStorage.key(index);
      return [key, key ? localStorage.getItem(key) : null];
    })),
    session: Object.fromEntries(Array.from({ length: sessionStorage.length }, (_, index) => {
      const key = sessionStorage.key(index);
      return [key, key ? sessionStorage.getItem(key) : null];
    })),
  }));
  const localEntries = Object.entries(storage.local);
  assert(
    localEntries.every(([key, value]) => key === "rulepath.reducedMotion" && (value === "reduce" || value === "motion")),
    `localStorage contains only the reduced-motion UI preference: ${JSON.stringify(storage.local)}`,
  );
  assert(Object.keys(storage.session).length === 0, "sessionStorage remains empty");
}

function assertNoForbiddenTerms(surface, label) {
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

async function waitForFreshness(page, minimum) {
  await page.waitForFunction((expected) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token >= expected;
  }, {}, minimum);
}

async function focusByTestId(page, testId) {
  await page.focus(`[data-testid="${testId}"]`);
}

async function assertFocusedVisible(page) {
  const focusStyle = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) {
      return null;
    }
    const style = window.getComputedStyle(element);
    return {
      outlineWidth: style.outlineWidth,
      outlineStyle: style.outlineStyle,
    };
  });
  assert(Boolean(focusStyle), "focus target exists");
  assert(
    focusStyle.outlineStyle !== "none" || focusStyle.outlineWidth !== "0px",
    `focused control has visible focus: ${JSON.stringify(focusStyle)}`,
  );
}

async function clickText(page, selector, text) {
  const clicked = await page.evaluate(
    ({ selector, text }) => {
      const element = Array.from(document.querySelectorAll(selector)).find((candidate) =>
        candidate.textContent?.includes(text),
      );
      if (!element) {
        return false;
      }
      element.click();
      return true;
    },
    { selector, text },
  );
  assert(clicked, `clicked ${selector} containing ${text}`);
}

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
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
