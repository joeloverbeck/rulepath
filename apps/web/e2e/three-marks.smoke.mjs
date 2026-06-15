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
  await page.setViewport({ width: 1280, height: 900 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Race to 21");
  await waitForText(page, "Three Marks");
  await assertDevPanelSecondary(page);

  await startThreeMarks(page);
  await assertBoard(page);
  await assertAccessibleCells(page);
  await clickCell(page, "r1c1");
  await waitForOccupied(page, "r1c1");
  await waitForText(page, "Bot chose action");
  await assertOccupiedCellInert(page, "r1c1");
  await assertNoLeak(page, "human-vs-bot board");

  await exportImportAndStepReplay(page);
  await assertReducedMotion(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startThreeMarks(page, "Hotseat");
  for (const cell of ["r1c1", "r2c1", "r1c2", "r2c2", "r1c3"]) {
    await clickCell(page, cell);
    await waitForOccupied(page, cell);
  }
  await waitForText(page, "seat_0 wins");
  const winningCells = await page.$$eval(".three-cell.winning", (cells) => cells.length);
  assert(winningCells === 3, `winning line highlights exactly three cells, got ${winningCells}`);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startThreeMarks(page, "Hotseat");
  for (const cell of ["r1c1", "r1c2", "r1c3", "r2c1", "r2c3", "r2c2", "r3c1", "r3c3", "r3c2"]) {
    await clickCell(page, cell);
    await waitForOccupied(page, cell);
  }
  await waitForText(page, "Draw");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startThreeMarks(page, "Hotseat");
  await focusCellByKeyboard(page, "three-cell-r1c1");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForOccupied(page, "r1c1");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "three_marks board replay win draw keyboard" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startThreeMarks(page, mode = "Human vs bot") {
  await clickText(page, "button", "Three Marks");
  if (mode !== "Human vs bot") {
    await clickLabel(page, mode);
  }
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="three-marks-board"]');
}

async function assertBoard(page) {
  const board = await page.evaluate(() => ({
    cells: document.querySelectorAll('[data-testid^="three-cell-"]').length,
    legal: Array.from(document.querySelectorAll('[data-testid^="three-cell-"]')).filter((cell) => !cell.disabled).length,
    status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
  }));
  assert(board.cells === 9, `board renders nine cells, got ${board.cells}`);
  assert(board.legal === 9, `initial board exposes nine Rust legal targets, got ${board.legal}`);
  assert(board.status.length > 0, "board exposes text status");
}

async function assertAccessibleCells(page) {
  const missing = await page.$$eval('[data-testid^="three-cell-"]', (cells) =>
    cells.filter((cell) => !(cell.getAttribute("aria-label") ?? "").trim()).map((cell) => cell.getAttribute("data-testid")),
  );
  assert(missing.length === 0, `all board cells have accessible names: ${missing.join(", ")}`);
}

async function assertOccupiedCellInert(page, cell) {
  const disabled = await page.$eval(`[data-testid="three-cell-${cell}"]`, (element) => element.disabled);
  assert(disabled, `${cell} is inert after occupation`);
}

async function exportImportAndStepReplay(page) {
  await clickText(page, "button", "Export Current Run");
  await page.waitForFunction(() => document.querySelector("textarea")?.value.includes("three_marks"));
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector(".replay-board [data-testid=three-marks-board]");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 1 /");
  await page.waitForFunction(() =>
    document.querySelector(".replay-board [data-testid=three-cell-r1c1]")?.className.includes("occupied"),
  );
  const sequence = await page.$$eval(".placement-sequence li", (items) => items.length);
  assert(sequence >= 1, "replay placement sequence renders");
}

async function assertReducedMotion(page) {
  await page.select(".motion-field select", "reduce");
  await page.waitForFunction(() => document.querySelector(".reduced-motion .three-marks-board"));
  const transitions = await page.$$eval(".three-cell", (cells) =>
    cells.map((cell) => window.getComputedStyle(cell).transitionProperty),
  );
  assert(transitions.every((value) => value === "none"), "three marks cells disable transitions under reduced motion");
}

async function assertDevPanelSecondary(page) {
  const open = await page.evaluate(() => document.querySelector(".dev-panel-body") !== null);
  assert(!open, "developer panel starts secondary/collapsed");
}

async function assertNoLeak(page, label) {
  const surface = await page.evaluate(() =>
    [
      document.body.textContent ?? "",
      Array.from(document.querySelectorAll("*"))
        .flatMap((element) => Array.from(element.attributes).map((attribute) => `${attribute.name}=${attribute.value}`))
        .join("\n"),
      Object.keys(localStorage).join("\n"),
      Object.keys(sessionStorage).join("\n"),
    ].join("\n"),
  );
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

async function clickCell(page, cell) {
  await page.click(`[data-testid="three-cell-${cell}"]`);
}

async function waitForOccupied(page, cell) {
  await page.waitForFunction(
    (cellId) => document.querySelector(`[data-testid="three-cell-${cellId}"]`)?.className.includes("occupied"),
    {},
    cell,
  );
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

async function focusCellByKeyboard(page, testId) {
  for (let index = 0; index < 40; index += 1) {
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
  assert(focusStyle.outlineStyle !== "none" || focusStyle.outlineWidth !== "0px", "focused cell has visible focus");
}

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
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
