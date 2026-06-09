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
const forbiddenLeakTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "candidate_ranking",
  "bot_candidate",
  "deck_tail",
  "opponent_private",
  "unrevealed_private",
  "secret_commitment",
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
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  const catalog = await catalogNames(page);
  assert(catalog.length >= 9, `catalog has outcome coverage candidates: ${catalog.join(", ")}`);

  await playRaceToTerminal(page, baseUrl);
  await assertOutcomePanel(page, "Race to 21");
  const racePanel = await panelText(page);
  await setReducedMotion(page);
  assert((await panelText(page)) === racePanel, "reduced motion preserves race outcome text");
  await assertNoLeak(page, consoleMessages, "race outcome");

  await playThreeMarksWin(page, baseUrl);
  await assertOutcomePanel(page, "Three Marks");
  await assertDisclosureKeyboardAndPointer(page);
  await assertNoLeak(page, consoleMessages, "three_marks outcome");

  await playThreeMarksDraw(page, baseUrl);
  await assertOutcomePanel(page, "Three Marks draw");
  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "outcome explanation panel a11y noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function playRaceToTerminal(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Race to 21");
  await clickText(page, "button", "Race to 21");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (let index = 0; index < 7; index += 1) {
    await clickText(page, "button", "Add 3");
  }
  await page.waitForSelector(".outcome-explanation-panel");
}

async function playThreeMarksWin(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Three Marks");
  await clickText(page, "button", "Three Marks");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (const cell of ["r1c1", "r2c1", "r1c2", "r2c2", "r1c3"]) {
    await clickCell(page, cell);
  }
  await page.waitForSelector(".outcome-explanation-panel");
}

async function playThreeMarksDraw(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Three Marks");
  await clickText(page, "button", "Three Marks");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (const cell of ["r1c1", "r1c2", "r1c3", "r2c1", "r2c3", "r2c2", "r3c1", "r3c3", "r3c2"]) {
    await clickCell(page, cell);
  }
  await page.waitForSelector(".outcome-explanation-panel");
}

async function assertOutcomePanel(page, label) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector(".outcome-explanation-panel");
    const status = panel?.querySelector('[role="status"]');
    const headingId = panel?.getAttribute("aria-labelledby") ?? "";
    const heading = headingId ? document.getElementById(headingId)?.textContent ?? "" : "";
    return {
      exists: Boolean(panel),
      heading,
      statusText: status?.textContent ?? "",
      standingRows: panel?.querySelectorAll(".outcome-standing-row").length ?? 0,
      disclosureButtons: panel?.querySelectorAll("button[aria-expanded][aria-controls]").length ?? 0,
      text: panel?.textContent ?? "",
    };
  });
  assert(summary.exists, `${label} renders shared outcome panel`);
  assert(summary.heading.length > 0, `${label} panel has associated heading`);
  assert(summary.statusText.length > 0, `${label} panel exposes role status text`);
  assert(summary.standingRows >= 1, `${label} panel renders final standing`);
  assert(summary.disclosureButtons >= 1, `${label} panel renders disclosure control`);
  assert(summary.text.includes("Outcome"), `${label} panel includes outcome label`);
}

async function assertDisclosureKeyboardAndPointer(page) {
  const button = await page.waitForSelector(".outcome-breakdown-section button");
  await button.focus();
  const focused = await page.evaluate(() => document.activeElement?.matches(".outcome-breakdown-section button") ?? false);
  assert(focused, "disclosure button receives focus");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector(".outcome-breakdown-section button")?.getAttribute("aria-expanded") === "true");
  await page.click(".outcome-breakdown-section button");
  await page.waitForFunction(() => document.querySelector(".outcome-breakdown-section button")?.getAttribute("aria-expanded") === "false");
}

async function panelText(page) {
  return page.$eval(".outcome-explanation-panel", (panel) => panel.textContent ?? "");
}

async function catalogNames(page) {
  await page.waitForSelector(".game-card");
  return page.$$eval(".game-card", (cards) =>
    cards.map((card) => card.querySelector("h2,h3,strong")?.textContent?.trim() ?? card.textContent?.trim() ?? ""),
  );
}

async function setReducedMotion(page) {
  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".reduced-motion .outcome-explanation-panel");
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
    local: Object.values(localStorage).join("\n"),
    session: Object.values(sessionStorage).join("\n"),
  }));
  assertNoForbiddenTerms(`${storage.local}\n${storage.session}`, "storage");
}

async function clickCell(page, cell) {
  await page.click(`[data-testid="three-cell-${cell}"]`);
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

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
}

async function assertFocusedVisible(page) {
  const focusStyle = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return null;
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

function assertNoForbiddenTerms(surface, label) {
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function contentTypeFor(filePath) {
  switch (extname(filePath)) {
    case ".js":
      return "text/javascript";
    case ".css":
      return "text/css";
    case ".wasm":
      return "application/wasm";
    case ".json":
      return "application/json";
    default:
      return "text/html";
  }
}
