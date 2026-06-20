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
  await page.setViewport({ width: 1280, height: 900 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Directional Flip");
  await assertPlayFirst(page);
  await assertDevPanelSecondary(page);

  await startDirectionalFlip(page);
  await assertBoard(page);
  await assertSeatIdentity(page);
  await assertAccessibleCells(page);
  await assertPreview(page, "r3c4");
  await clickCell(page, "r3c4");
  await waitForPly(page, 2);
  await waitForText(page, "Bot chose action");
  await assertBotRationale(page);
  await assertNoLeak(page, consoleMessages, "human-vs-bot directional_flip");
  await exportImportAndStepReplay(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startDirectionalFlip(page, "Hotseat");
  await assertNoSeatIdentity(page);
  await keyboardPlaceAtR3c4(page);
  await waitForPly(page, 1);
  await page.keyboard.press("Escape");
  await assertNoLeak(page, consoleMessages, "directional_flip keyboard");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startDirectionalFlip(page, "Bot vs bot");
  await stepBotsUntilForcedPass(page);

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startDirectionalFlip(page, "Hotseat");
  await page.select(".motion-field select", "reduce");
  await clickCell(page, "r3c4");
  await page.waitForSelector(".directional-flip-board.reduced .directional-cell.flipped .directional-disc");
  const animationName = await page.$eval(".directional-flip-board.reduced .directional-cell.flipped .directional-disc", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "directional_flip disables flip animation under reduced motion");
  await assertStorageClean(page);
  await assertNoLeak(page, consoleMessages, "directional_flip reduced-motion");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "directional_flip board replay forced-pass reduced noleak" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startDirectionalFlip(page, mode = "Human vs bot") {
  await clickText(page, "button", "Directional Flip");
  if (mode !== "Human vs bot") {
    await clickLabel(page, mode);
  }
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="directional-flip-board"]');
}

async function assertBoard(page) {
  const board = await page.evaluate(() => {
    const cells = Array.from(document.querySelectorAll('[data-testid^="directional-cell-"]'));
    return {
      cells: cells.length,
      legal: cells.filter((cell) => cell.classList.contains("legal")).length,
      status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
      discs: document.querySelectorAll(".directional-disc").length,
    };
  });
  assert(board.cells === 64, `board renders sixty-four cells, got ${board.cells}`);
  assert(board.legal === 4, `initial board exposes four Rust legal targets, got ${board.legal}`);
  assert(board.discs === 4, `initial board renders four starting discs, got ${board.discs}`);
  assert(board.status.length > 0, "board exposes text status");
}

async function assertSeatIdentity(page) {
  const identity = await page.$eval('[data-testid="directional-identity"]', (element) => element.textContent ?? "");
  assert(
    /you play the ring disc/i.test(identity),
    `human-vs-bot exposes the local seat identity, got "${identity}"`,
  );
  const seat0Role = await page.$eval('[data-testid="directional-score-seat_0"]', (element) => element.textContent ?? "");
  const seat1Role = await page.$eval('[data-testid="directional-score-seat_1"]', (element) => element.textContent ?? "");
  assert(/\(you\)/i.test(seat0Role), `seat_0 score is tagged as the human, got "${seat0Role}"`);
  assert(/\(bot\)/i.test(seat1Role), `seat_1 score is tagged as the bot, got "${seat1Role}"`);
}

async function assertNoSeatIdentity(page) {
  const present = await page.$('[data-testid="directional-identity"]');
  assert(present === null, "hotseat does not assert a single-seat human identity");
}

async function assertAccessibleCells(page) {
  const missing = await page.$$eval('[data-testid^="directional-cell-"]', (cells) =>
    cells.filter((cell) => !(cell.getAttribute("aria-label") ?? "").trim()).map((cell) => cell.getAttribute("data-testid")),
  );
  assert(missing.length === 0, `all board cells have accessible names: ${missing.join(", ")}`);
}

async function assertPreview(page, cell) {
  await page.hover(`[data-testid="directional-cell-${cell}"]`);
  await page.waitForSelector(".directional-cell.preview-target");
  await page.waitForSelector(".directional-cell.preview-flip");
  const preview = await page.evaluate(() => ({
    targets: document.querySelectorAll(".directional-cell.preview-target").length,
    flips: document.querySelectorAll(".directional-cell.preview-flip").length,
    copy: document.querySelector(".directional-preview")?.textContent ?? "",
  }));
  assert(preview.targets === 1, `hover exposes one Rust target preview, got ${preview.targets}`);
  assert(preview.flips >= 1, `hover exposes Rust flip preview cells, got ${preview.flips}`);
  assert(preview.copy.trim().length > "Preview".length, "preview includes Rust explanation text");
}

async function keyboardPlaceAtR3c4(page) {
  await focusCellByKeyboard(page, "directional-cell-r1c1");
  await assertFocusedVisible(page);
  for (const key of ["ArrowDown", "ArrowDown", "ArrowRight", "ArrowRight", "ArrowRight"]) {
    await page.keyboard.press(key);
  }
  const focused = await page.evaluate(() => document.activeElement?.getAttribute("data-testid"));
  assert(focused === "directional-cell-r3c4", `arrow keys move focus to r3c4, got ${focused}`);
  await page.keyboard.press("Enter");
}

async function exportImportAndStepReplay(page) {
  await clickText(page, "button", "Export Current Run");
  const replayText = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayExport = await replayText.jsonValue();
  assert(replayExport.includes('"game_id": "directional_flip"'), "directional_flip replay export preserves game id");
  assertNoForbiddenTerms(replayExport, "directional_flip replay export");
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector(".replay-board [data-testid=directional-flip-board]");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 1 /");
  await page.waitForFunction(() =>
    document.querySelector(".replay-board [data-testid=directional-cell-r3c4]")?.className.includes("seat_0"),
  );
  const sequence = await page.$$eval(".placement-sequence li", (items) => items.length);
  assert(sequence >= 1, "replay placement sequence renders");
}

async function assertBotRationale(page) {
  await page.waitForSelector('[data-testid="bot-explanation"]');
  const rationale = await page.$eval('[data-testid="bot-explanation"]', (element) => element.textContent ?? "");
  assert(rationale.trim().length > "Bot".length, "bot rationale renders public prose");
  assertNoForbiddenTerms(rationale, "directional_flip bot rationale");
}

async function stepBotsUntilForcedPass(page) {
  for (let index = 0; index < 96; index += 1) {
    const before = await page.evaluate(() => {
      const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
      return {
        token: state?.view?.freshness_token ?? -1,
        body: document.body.textContent ?? "",
        turn: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
      };
    });
    if (before.body.includes("Pass taken")) {
      return;
    }
    assert(!before.turn.includes("wins") && !before.turn.includes("Draw"), "bot-vs-bot reached terminal before forced pass");
    await clickText(page, "button", "Step Bot");
    await page.waitForFunction((previous) => {
      const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
      return (state?.view?.freshness_token ?? -1) !== previous || document.body.textContent?.includes("Pass taken");
    }, {}, before.token);
    const passed = await page.evaluate(() => document.body.textContent?.includes("Pass taken") ?? false);
    if (passed) {
      await waitForText(page, "Pass taken");
      return;
    }
  }
  throw new Error("bot-vs-bot did not reach a forced pass within 96 Rust bot steps");
}

async function assertDevPanelSecondary(page) {
  const open = await page.evaluate(() => document.querySelector(".dev-panel-body") !== null);
  assert(!open, "developer panel starts secondary/collapsed");
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

async function clickCell(page, cell) {
  await page.click(`[data-testid="directional-cell-${cell}"]`);
}

async function waitForPly(page, ply) {
  await page.waitForFunction((expected) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token >= expected;
  }, {}, ply);
}

async function focusCellByKeyboard(page, testId) {
  for (let index = 0; index < 100; index += 1) {
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
