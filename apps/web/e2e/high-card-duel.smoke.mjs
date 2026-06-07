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
  "hcd:r",
  "deck_order",
  "deck order",
  "bot_candidate",
  "candidate_ranking",
  "hidden_state",
  "private_state",
  "internal_state",
  "secret",
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

  await waitForText(page, "High Card Duel");
  await clickText(page, "button", "High Card Duel");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="high-card-duel-board"]');
  await assertHighCardA11y(page);
  await assertNoLeak(page, consoleMessages, "initial seat_0 DOM");

  await clickText(page, "button", "Observer");
  await waitForText(page, "Observer only");
  await assertNoLeak(page, consoleMessages, "observer DOM");

  await clickText(page, "button", "Seat 0");
  await page.waitForSelector('[data-testid="high-card-commit-0"]');
  await focusByTestId(page, "high-card-commit-0");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Commitment placed");
  await assertNoLeak(page, consoleMessages, "seat_0 committed DOM");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Redacted for hidden-information viewer safety.");
  await assertNoLeak(page, consoleMessages, "dev panel open DOM");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "public_observer_projection_v1"'), "default export is public observer projection");
  assert(!replayText.includes('"seed"'), "default export omits hidden seed");
  assertNoForbiddenTerms(replayText, "pre-reveal replay export");

  await clickText(page, "button", "Seat 1");
  await page.waitForSelector('[data-testid="high-card-commit-0"]');
  await page.click('[data-testid="high-card-commit-0"]');
  await waitForText(page, "Cards revealed");
  await assertNoLeak(page, consoleMessages, "revealed DOM");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".high-card-duel-board.reduced");
  const animationName = await page.$eval(".high-card-duel-board.reduced .duel-card", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses high_card_duel card animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".duel-table");
  const columns = await page.$eval(".duel-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive duel table remains measurable");

  await assertStorageClean(page);
  await assertNoLeak(page, consoleMessages, "final DOM");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "high_card_duel noleak viewer a11y reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function assertHighCardA11y(page) {
  const summary = await page.evaluate(() => {
    const controls = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="high-card-duel-board"]')),
      viewerButtons: Array.from(document.querySelectorAll(".high-card-viewer-controls button")).map(
        (button) => button.textContent?.trim() ?? "",
      ),
      commitButtons: document.querySelectorAll('[data-testid^="high-card-commit-"]').length,
      unnamed: controls
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".board-status")?.textContent ?? "",
    };
  });
  assert(summary.board, "high_card_duel board renders");
  assert(summary.viewerButtons.includes("Seat 0"), "viewer selector exposes Seat 0");
  assert(summary.viewerButtons.includes("Seat 1"), "viewer selector exposes Seat 1");
  assert(summary.viewerButtons.includes("Observer"), "viewer selector exposes observer");
  assert(summary.commitButtons > 0, "active seat has Rust-provided private commit buttons");
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "board status announces current state");
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
  const storage = await page.evaluate(() =>
    [
      Object.keys(localStorage).join("\n"),
      Object.values(localStorage).join("\n"),
      Object.keys(sessionStorage).join("\n"),
      Object.values(sessionStorage).join("\n"),
    ].join("\n"),
  );
  assertNoForbiddenTerms(storage, "browser storage");
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
