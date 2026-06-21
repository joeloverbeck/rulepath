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
  const consoleMessages = [];
  page.on("console", (message) => consoleMessages.push(message.text()));
  page.on("pageerror", (error) => consoleMessages.push(error.message));
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Draughts Lite");
  await startDraughts(page);
  await assertBoardA11y(page);
  await assertKeyboardQuietMove(page);
  await waitForPly(page, 1);

  await playPointerPath(page, ["r6c3", "r5c2"]);
  await waitForPly(page, 2);
  await assertMandatoryCapture(page);
  await playPointerPath(page, ["r4c1", "r6c3"]);
  await waitForPly(page, 3);
  await waitForText(page, "Capture step");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"game_id": "draughts_lite"'), "replay export keeps draughts_lite game id");
  assert(replayText.includes('"from/r4c1"') && replayText.includes('"jump/r6c3"'), "replay export keeps capture path segments");
  await waitForText(page, "from/r4c1 > jump/r6c3");
  assertNoForbiddenTerms(replayText, "draughts_lite replay export");

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector(".replay-board [data-testid=draughts-lite-board]");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 1 /");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".draughts-lite-board.reduced");
  const transition = await page.$eval(".draughts-lite-board.reduced .draughts-cell", (element) =>
    window.getComputedStyle(element).transitionProperty,
  );
  assert(transition === "none" || transition === "all", "reduced motion removes draughts cell transitions");
  await assertNoLeak(page, consoleMessages, "draughts_lite DOM");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "draughts_lite keyboard capture replay reduced noleak" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startDraughts(page) {
  await clickText(page, "button", "Draughts Lite");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="draughts-lite-board"]');
}

async function assertBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const cells = Array.from(document.querySelectorAll('[data-testid^="draughts-cell-"]'));
    return {
      cells: cells.length,
      legal: cells.filter((cell) => cell.classList.contains("legal")).length,
      missingNames: cells
        .filter((cell) => !(cell.getAttribute("aria-label") ?? "").trim())
        .map((cell) => cell.getAttribute("data-testid")),
      pieces: document.querySelectorAll(".draughts-piece").length,
      crowns: document.querySelectorAll(".draughts-piece.crown").length,
      active: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
      activeDescendant: document.querySelector('[data-testid="draughts-lite-board"]')?.getAttribute("aria-activedescendant") ?? "",
      live: document.querySelector('[data-testid="draughts-live-status"]')?.textContent ?? "",
    };
  });
  assert(summary.cells === 64, `draughts_lite renders sixty-four cells, got ${summary.cells}`);
  assert(summary.legal === 4, `draughts_lite exposes four Rust legal origins, got ${summary.legal}`);
  assert(summary.missingNames.length === 0, `draughts_lite cells have accessible names: ${summary.missingNames.join(", ")}`);
  assert(summary.pieces === 24, `draughts_lite renders twenty-four pieces, got ${summary.pieces}`);
  assert(summary.active.includes("Seat 1"), "draughts_lite exposes humanized text turn status");
  assert(summary.activeDescendant.startsWith("draughts-cell-"), "draughts_lite grid exposes active descendant");
  assert(summary.live.includes("Rust-provided"), "draughts_lite live region announces Rust-provided choices");
  await assertFocusedVisibleAfterFocus(page, "draughts-cell-r1c1");
}

async function assertKeyboardQuietMove(page) {
  await focusByTestId(page, "draughts-cell-r1c1");
  await assertFocusedVisible(page);
  for (const key of ["ArrowDown", "ArrowDown", "ArrowRight"]) {
    await page.keyboard.press(key);
  }
  await assertFocusedTestId(page, "draughts-cell-r3c2");
  await page.keyboard.press("Enter");
  await waitForText(page, "from/r3c2");
  for (const key of ["ArrowDown", "ArrowLeft"]) {
    await page.keyboard.press(key);
  }
  await assertFocusedTestId(page, "draughts-cell-r4c1");
  await page.keyboard.press(" ");
}

async function assertMandatoryCapture(page) {
  await waitForText(page, "Capture is mandatory");
  const summary = await page.evaluate(() => ({
    live: document.querySelector('[data-testid="draughts-live-status"]')?.textContent ?? "",
    cues: document.querySelector(".draughts-lite-cues")?.textContent ?? "",
    legalOrigins: Array.from(document.querySelectorAll(".draughts-cell.legal")).map((cell) =>
      cell.getAttribute("data-testid"),
    ),
  }));
  assert(summary.live.includes("Capture is mandatory"), "live region announces mandatory capture");
  assert(summary.cues.includes("Captures") && summary.cues.includes("1"), "capture cue reports capture origins");
  assert(summary.cues.includes("mandatory"), "capture cue flags that capturing is mandatory");
  assert(summary.legalOrigins.includes("draughts-cell-r4c1"), "mandatory capture exposes the capturing origin");
}

async function playPointerPath(page, cells) {
  for (const cell of cells) {
    await page.click(`[data-testid="draughts-cell-${cell}"]`);
  }
}

async function waitForPly(page, ply) {
  await page.waitForFunction((expected) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token >= expected;
  }, {}, ply);
}

async function assertFocusedVisibleAfterFocus(page, testId) {
  await focusByTestId(page, testId);
  await assertFocusedVisible(page);
}

async function assertFocusedTestId(page, expected) {
  const focused = await page.evaluate(() => document.activeElement?.getAttribute("data-testid"));
  assert(focused === expected, `focused ${expected}, got ${focused}`);
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

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
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

async function clickLabel(page, text) {
  const clicked = await page.evaluate((labelText) => {
    const label = Array.from(document.querySelectorAll("label")).find((candidate) =>
      candidate.textContent?.includes(labelText),
    );
    if (!label) {
      return false;
    }
    label.click();
    return true;
  }, text);
  assert(clicked, `clicked label containing ${text}`);
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
