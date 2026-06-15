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
const committedItemId = "ember_1";
const forbiddenPreRevealTerms = [
  committedItemId,
  `commit/${committedItemId}`,
  "hidden_state",
  "private_state",
  "internal_state",
  "candidate_ranking",
  "bot_candidate",
  "debug_state",
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

  await waitForText(page, "Veiled Draft");
  await clickText(page, "button", "Veiled Draft");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="secret-draft-board"]');
  await assertSecretDraftA11y(page);

  await focusByTestId(page, "secret-draft-choice-1-0");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForPendingStatus(page, "seat_0", "Committed");
  await waitForPendingStatus(page, "seat_1", "Waiting");
  await assertNoPreRevealLeak(page, consoleMessages, "after first commitment");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Redacted for hidden-information viewer safety.");
  await assertNoPreRevealLeak(page, consoleMessages, "developer panel after first commitment");
  await clickText(page, "button", "Developer panel");

  await focusByTestId(page, "secret-draft-choice-1-0");
  await page.keyboard.press("Enter");
  await waitForText(page, "Choices revealed");
  await waitForText(page, "Seat 0 won the conflict");
  await waitForText(page, "Ember One / Ember Two");
  await assertRevealRendered(page);

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "viewer_scoped_observation_v1"'), "export is viewer-scoped public timeline");
  assert(replayText.includes('"game_id": "secret_draft"'), "export keeps secret_draft game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assert(!replayText.includes('"seed_evidence"'), "public export omits seed evidence");

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await clickText(page, ".replay-actions button", "Step");
  await waitForText(page, "Cursor 1 /");
  await assertReplayViewerNoForbidden(page);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".secret-draft-board.reduced");
  const animationName = await page.$eval(".secret-draft-board.reduced .secret-item", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses secret_draft item animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".secret-pool-grid");
  const columns = await page.$eval(".secret-pool-grid", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive secret_draft pool grid remains measurable");

  await assertHumanVsBotCommitPath(page, baseUrl, consoleMessages);

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenPreRevealTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "secret_draft noleak reveal replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function assertSecretDraftA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="secret-draft-board"]')),
      choices: document.querySelectorAll('[data-testid^="secret-draft-choice-"]').length,
      pending: document.querySelectorAll('[data-testid^="secret-draft-pending-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".secret-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "secret_draft board renders");
  assert(summary.choices > 0, "secret_draft exposes Rust-provided commit buttons");
  assert(summary.pending === 2, "secret_draft renders two pending seat statuses");
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "secret_draft latest-effect region has text");
}

async function assertRevealRendered(page) {
  const reveal = await page.evaluate(() => ({
    effectText: document.querySelector('[data-testid="effects"]')?.textContent ?? "",
    historyText: document.querySelector(".secret-reveals")?.textContent ?? "",
    pendingText: document.querySelector(".secret-pending")?.textContent ?? "",
  }));
  assert(reveal.effectText.includes("Reveal batch"), "effect log groups reveal batch start");
  assert(reveal.effectText.includes("Draft resolved"), "effect log reports draft resolution");
  assert(reveal.historyText.includes("Round 1"), "reveal history records round one");
  assert(reveal.pendingText.includes("Seat 0") && reveal.pendingText.includes("Seat 1"), "pending seats remain seat-anchored");
}

async function assertHumanVsBotCommitPath(page, baseUrl, consoleMessages) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Veiled Draft");
  await clickText(page, "button", "Veiled Draft");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="secret-draft-board"]');
  await focusByTestId(page, "secret-draft-choice-1-0");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Choices revealed");
  await waitForText(page, "Draft resolved");
  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state?.view?.freshness_token >= 2, "human_vs_bot flow includes automatic Rust bot commitment");
  await assertRevealRendered(page);
  await assertNoPostRevealHiddenSurface(page, consoleMessages, "human_vs_bot reveal");
}

async function assertNoPreRevealLeak(page, consoleMessages, label) {
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
  assertNoForbiddenTerms(surface, label, forbiddenPreRevealTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbiddenPreRevealTerms);
}

async function assertNoPostRevealHiddenSurface(page, consoleMessages, label) {
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
  const forbidden = forbiddenPreRevealTerms.filter((term) => term !== committedItemId && term !== `commit/${committedItemId}`);
  assertNoForbiddenTerms(surface, label, forbidden);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbidden);
}

async function assertReplayViewerNoForbidden(page) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assertNoForbiddenTerms(surface, "secret_draft public replay viewer", ["hidden_state", "private_state", "internal_state", "debug_state"]);
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

async function focusByTestId(page, testId) {
  await page.focus(`[data-testid="${testId}"]`);
}

async function assertFocusedVisible(page) {
  const visible = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    const style = window.getComputedStyle(element);
    return rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none";
  });
  assert(visible, "focused control is visible");
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

async function waitForPendingStatus(page, seat, status) {
  await page.waitForFunction(
    (seatId, expected) => {
      const element = document.querySelector(`[data-testid="secret-draft-pending-${seatId}-round-1"]`);
      return element?.textContent?.includes(expected);
    },
    {},
    seat,
    status,
  );
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
  for (const term of terms) {
    assert(!lower.includes(term.toLowerCase()), `${label} leaks forbidden term ${term}`);
  }
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
