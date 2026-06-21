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
const forbiddenLeakTerms = ["hidden_state", "private_state", "internal_state", "candidate_ranking"];
const drawColumns = [
  "c6",
  "c4",
  "c3",
  "c4",
  "c2",
  "c6",
  "c4",
  "c2",
  "c1",
  "c2",
  "c5",
  "c2",
  "c3",
  "c6",
  "c1",
  "c6",
  "c7",
  "c7",
  "c3",
  "c1",
  "c7",
  "c1",
  "c5",
  "c3",
  "c4",
  "c1",
  "c4",
  "c5",
  "c3",
  "c4",
  "c3",
  "c7",
  "c1",
  "c5",
  "c2",
  "c2",
  "c6",
  "c5",
  "c5",
  "c6",
  "c7",
  "c7",
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
  await page.setViewport({ width: 1280, height: 900 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Column Four");
  await assertPlayFirst(page);

  await startColumnFour(page);
  await assertBoard(page);
  await assertAccessibleColumnControls(page);
  await assertPreview(page, "c4");
  await clickColumn(page, "c4");
  await waitForCellTitle(page, "r1c4", "occupied by Seat 1");
  await assertBotRationale(page);
  await assertNoLeak(page, consoleMessages, "human-vs-bot column_four");
  await exportImportAndStepReplay(page);
  await assertNoLeak(page, consoleMessages, "column_four replay");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Hotseat");
  for (let index = 0; index < 6; index += 1) {
    await clickColumn(page, "c1");
    await waitForPly(page, index + 1);
  }
  const fullColumnDisabled = await page.$eval('[data-testid="column-four-control-c1"]', (element) => element.disabled);
  assert(fullColumnDisabled, "full column control is inert");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Hotseat");
  for (const column of ["c1", "c1", "c2", "c2", "c3", "c3", "c4"]) {
    await clickColumn(page, column);
  }
  await waitForText(page, "Seat 1 wins");
  const winningCells = await page.$$eval(".column-cell.winning", (cells) => cells.length);
  assert(winningCells === 4, `winning line highlights exactly four cells, got ${winningCells}`);
  await assertTerminalControlsInert(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Hotseat");
  for (const column of drawColumns) {
    await clickColumn(page, column);
  }
  await waitForText(page, "Draw");
  await assertTerminalControlsInert(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Hotseat");
  await focusControlByTestId(page, "column-four-control-c4");
  await assertFocusedVisible(page);
  await page.waitForSelector(".column-cell.preview");
  await page.keyboard.press("Enter");
  await waitForCellTitle(page, "r1c4", "occupied by Seat 1");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Bot vs bot");
  for (let index = 0; index < 50; index += 1) {
    if (await isTerminal(page)) {
      break;
    }
    await clickText(page, "button", "Step Bot");
    await page.waitForFunction((previous) => {
      const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
      return state?.view?.freshness_token !== previous;
    }, {}, index);
  }
  assert(await isTerminal(page), "bot-vs-bot reaches and stops on a terminal state");
  await assertTerminalControlsInert(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startColumnFour(page, "Hotseat");
  await page.select(".motion-field select", "reduce");
  await clickColumn(page, "c4");
  await page.waitForSelector(".column-four-board.reduced .column-cell.landed .column-piece");
  const animationName = await page.$eval(".column-four-board.reduced .column-cell.landed .column-piece", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "column_four landed piece disables animation under reduced motion");
  await assertNoLeak(page, consoleMessages, "column_four reduced-motion");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "column_four board a11y noleak replay terminal" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startColumnFour(page, mode = "Human vs bot") {
  await clickText(page, "button", "Column Four");
  if (mode !== "Human vs bot") {
    await clickLabel(page, mode);
  }
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="column-four-board"]');
}

async function assertBoard(page) {
  const board = await page.evaluate(() => ({
    cells: document.querySelectorAll(".column-cell").length,
    controls: document.querySelectorAll('[data-testid^="column-four-control-"]').length,
    legal: Array.from(document.querySelectorAll('[data-testid^="column-four-control-"]')).filter((control) => !control.disabled).length,
    status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
  }));
  assert(board.cells === 42, `board renders forty-two cells, got ${board.cells}`);
  assert(board.controls === 7, `board renders seven column controls, got ${board.controls}`);
  assert(board.legal === 7, `initial board exposes seven Rust legal targets, got ${board.legal}`);
  assert(board.status.length > 0, "board exposes text status");
}

async function assertAccessibleColumnControls(page) {
  const missing = await page.$$eval('[data-testid^="column-four-control-"]', (controls) =>
    controls.filter((control) => !(control.getAttribute("aria-label") ?? "").trim()).map((control) => control.getAttribute("data-testid")),
  );
  assert(missing.length === 0, `all column controls have accessible names: ${missing.join(", ")}`);
}

async function assertPreview(page, column) {
  await page.hover(`[data-testid="column-four-control-${column}"]`);
  await page.waitForSelector(".column-cell.preview");
  const previewCount = await page.$$eval(".column-cell.preview", (cells) => cells.length);
  assert(previewCount === 1, `hover exposes one Rust landing preview, got ${previewCount}`);
}

async function exportImportAndStepReplay(page) {
  await clickText(page, "button", "Export Current Run");
  const replayText = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayExport = await replayText.jsonValue();
  assert(replayExport.includes('"game_id": "column_four"'), "column_four replay export preserves game id");
  assertNoForbiddenTerms(replayExport, "column_four replay export");
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector(".replay-board [data-testid=column-four-board]");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 1 /");
  await waitForCellTitle(page, "r1c4", "occupied by Seat 1", ".replay-board");
  const sequence = await page.$$eval(".placement-sequence li", (items) => items.length);
  assert(sequence >= 1, "replay placement sequence renders");
}

async function assertBotRationale(page) {
  await page.waitForSelector('[data-testid="bot-explanation"]');
  const rationale = await page.$eval('[data-testid="bot-explanation"]', (element) => element.textContent ?? "");
  assert(rationale.trim().length > "Bot".length, "bot rationale renders public prose");
  assertNoForbiddenTerms(rationale, "column_four bot rationale");
}

async function assertTerminalControlsInert(page) {
  const enabled = await page.$$eval('[data-testid^="column-four-control-"]', (controls) =>
    controls.filter((control) => !control.disabled).map((control) => control.getAttribute("data-testid")),
  );
  assert(enabled.length === 0, `terminal board exposes no playable controls: ${enabled.join(", ")}`);
}

async function assertPlayFirst(page) {
  const pageShape = await page.evaluate(() => {
    const bodyText = document.body.textContent?.trim() ?? "";
    const preCount = document.querySelectorAll("pre").length;
    const textareaCount = document.querySelectorAll("textarea").length;
    return { bodyText, preCount, textareaCount };
  });
  assert(!pageShape.bodyText.startsWith("{"), "normal page is not raw JSON");
  assert(pageShape.preCount === 0, "normal page is not debug-pre dominated");
  assert(pageShape.textareaCount === 0, "initial page is play-first, before replay tools");
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

function assertNoForbiddenTerms(surface, label) {
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

async function clickColumn(page, column) {
  await page.click(`[data-testid="column-four-control-${column}"]`);
}

async function waitForCellTitle(page, cell, text, root = "body") {
  await page.waitForFunction(
    ({ root, cell, text }) => {
      const scope = document.querySelector(root);
      return Array.from(scope?.querySelectorAll(".column-cell title") ?? []).some((title) => {
        const value = title.textContent ?? "";
        return value.includes(cell) && value.includes(text);
      });
    },
    {},
    { root, cell, text },
  );
}

async function waitForPly(page, ply) {
  await page.waitForFunction((expected) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token >= expected;
  }, {}, ply);
}

async function isTerminal(page) {
  return page.evaluate(() => {
    const text = document.querySelector('[data-testid="turn"]')?.textContent ?? "";
    return text.includes("wins") || text.includes("Draw");
  });
}

async function clickLabel(page, text) {
  await page.evaluate((labelText) => {
    const label = Array.from(document.querySelectorAll("label")).find((candidate) =>
      candidate.textContent?.includes(labelText),
    );
    if (!label) {
      throw new Error(`Missing label: ${labelText}`);
    }
    label.click();
  }, text);
}

async function clickText(page, selector, text) {
  await page.evaluate(
    ({ selector, text }) => {
      const element = Array.from(document.querySelectorAll(selector)).find((candidate) =>
        candidate.textContent?.includes(text),
      );
      if (!element) {
        throw new Error(`Missing ${selector} with text: ${text}`);
      }
      element.click();
    },
    { selector, text },
  );
}

async function focusControlByTestId(page, testId) {
  for (let index = 0; index < 80; index += 1) {
    const focused = await page.evaluate(
      (expected) => document.activeElement?.getAttribute("data-testid") === expected,
      testId,
    );
    if (focused) {
      return;
    }
    await page.keyboard.press("Tab");
  }
  throw new Error(`Unable to keyboard-focus ${testId}`);
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
  assert(focusStyle.outlineStyle !== "none" || focusStyle.outlineWidth !== "0px", "focused control has visible focus");
}

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
}

async function waitForSelectorText(page, selector, text) {
  await page.waitForFunction(
    ({ selector, text }) => document.querySelector(selector)?.textContent?.includes(text),
    {},
    { selector, text },
  );
}

function contentTypeFor(path) {
  switch (extname(path)) {
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
