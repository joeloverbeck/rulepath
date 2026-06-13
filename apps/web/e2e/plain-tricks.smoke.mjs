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
const cardLabels = [
  "Gale 1",
  "Gale 2",
  "Gale 3",
  "Gale 4",
  "Gale 5",
  "Gale 6",
  "River 1",
  "River 2",
  "River 3",
  "River 4",
  "River 5",
  "River 6",
  "Ember 1",
  "Ember 2",
  "Ember 3",
  "Ember 4",
  "Ember 5",
  "Ember 6",
];
const cardIds = cardLabels.map((label) => label.toLowerCase().replace(" ", "_"));
const internalTerms = [
  "deck_order",
  "hidden_state",
  "private_state",
  "internal_state",
  "seed_evidence",
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

  await startPlainTricks(page, baseUrl, "Bot vs bot");
  await page.waitForSelector('[data-testid="plain-tricks-board"]');
  await assertPlainTricksA11y(page, false);
  await assertObserverNoLeak(page, consoleMessages);

  await startPlainTricks(page, baseUrl, "Hotseat", 11);
  await page.waitForSelector('[data-testid="plain-tricks-own-hand"]');
  await assertPlainTricksA11y(page, true);
  const initialOwnLabels = await ownHandLabels(page);
  assert(initialOwnLabels.length === 6, "seat view shows six own cards");
  await assertSeatNoLeak(page, consoleMessages, initialOwnLabels, "initial seat view");

  await focusByTestId(page, "choice-plain-tricks-trick-0-0");
  await assertFocusedVisible(page);
  const playedLabel = await page.$eval('[data-testid="choice-plain-tricks-trick-0-0"]', (button) =>
    (button.getAttribute("aria-label") ?? button.textContent ?? "").replace(/^Play\s+/i, "").trim(),
  );
  await page.keyboard.press("Enter");
  await waitForText(page, "Card played");
  await waitForText(page, "Seat 1 to play");
  const followerOwnLabels = await ownHandLabels(page);
  await assertForcedFollowSurface(page);
  await assertSeatNoLeak(page, consoleMessages, [...followerOwnLabels, playedLabel], "after first public play");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Viewer filtered");
  await waitForText(page, "Redacted for hidden-information viewer safety.");
  await assertNoForbiddenTerms(await fullBrowserSurface(page), "developer panel", [...cardIds, ...internalTerms]);
  await clickText(page, "button", "Submit Stale Action");
  await waitForText(page, "stale_action");
  await assertNoForbiddenTerms(await fullBrowserSurface(page), "stale diagnostic", [...cardIds, ...internalTerms]);

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "viewer_scoped_observation_v1"'), "export is viewer-scoped public timeline");
  assert(replayText.includes('"game_id": "plain_tricks"'), "export keeps plain_tricks game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assert(!replayText.includes('"seed_evidence"'), "public export omits seed evidence");
  assertNoForbiddenTerms(replayText, "public replay export", [
    ...cardIds.filter((cardId) => cardId !== labelToId(playedLabel)),
    ...internalTerms,
  ]);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await clickText(page, ".replay-actions button", "Step");
  await waitForText(page, "Cursor 1 /");
  await assertReplayViewerNoForbidden(page, playedLabel);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".plain-tricks-board.reduced");
  const animationName = await page.$eval(".plain-tricks-board.reduced .plain-card", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses plain_tricks card animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".plain-tricks-table");
  const columns = await page.$eval(".plain-tricks-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive plain_tricks table remains measurable");

  await playHumanVsBotToTerminal(page, baseUrl);
  await waitForText(page, "Trick totals");
  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "plain_tricks noleak replay reduced terminal" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startPlainTricks(page, baseUrl, modeLabel, seed = null) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Plain Tricks");
  await clickText(page, "button", "Plain Tricks");
  if (seed !== null) {
    await page.$eval(
      ".field input[type='number']",
      (input, value) => {
        const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
        setter?.call(input, String(value));
        input.dispatchEvent(new Event("input", { bubbles: true }));
      },
      seed,
    );
  }
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="plain-tricks-board"]');
}

async function assertPlainTricksA11y(page, expectChoices) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="plain-tricks-board"]')),
      choices: document.querySelectorAll('[data-testid^="choice-plain-tricks-trick-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".plain-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "plain_tricks board renders");
  if (expectChoices) {
    assert(summary.choices > 0, "plain_tricks exposes Rust-provided action buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "plain_tricks latest-effect region has text");
}

async function assertObserverNoLeak(page, consoleMessages) {
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("Observer"), "observer mode is visible");
  assert(surface.includes("Cards hidden"), "observer hand surface is face-down");
  assertNoForbiddenTerms(surface, "plain_tricks observer", [...cardIds, ...cardLabels, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "plain_tricks observer console", internalTerms);
}

async function assertSeatNoLeak(page, consoleMessages, allowedLabels, label) {
  const forbiddenLabels = cardLabels.filter((card) => !allowedLabels.includes(card));
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("Cards hidden"), "opponent hand renders as hidden count");
  assertNoForbiddenTerms(surface, label, [...cardIds, ...forbiddenLabels, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertForcedFollowSurface(page) {
  const summary = await page.evaluate(() => ({
    enabled: Array.from(document.querySelectorAll('[data-testid^="choice-plain-tricks-trick-"]')).filter(
      (button) => !button.disabled,
    ).length,
    total: document.querySelectorAll('[data-testid^="choice-plain-tricks-trick-"]').length,
  }));
  assert(summary.total > 0, "follower has card buttons");
  assert(summary.enabled > 0, "follower has at least one Rust legal card");
  assert(summary.enabled < summary.total, "forced-follow state disables non-legal held cards");
}

async function playHumanVsBotToTerminal(page, baseUrl) {
  await startPlainTricks(page, baseUrl, "Human vs bot", 3);
  for (let index = 0; index < 30; index += 1) {
    const terminal = await page.evaluate(() => document.body.textContent?.includes("Trick totals") ?? false);
    if (terminal) {
      return;
    }
    const hasHumanChoice = await page.$('[data-testid^="choice-plain-tricks-trick-"]:not(:disabled)');
    if (hasHumanChoice) {
      await hasHumanChoice.click();
      await page.waitForFunction(() => document.body.textContent?.includes("Card played"));
    } else {
      await page.waitForFunction(
        () =>
          document.body.textContent?.includes("Bot chose action") &&
          (document.querySelector('[data-testid^="choice-plain-tricks-trick-"]:not(:disabled)') ||
            document.body.textContent?.includes("Trick totals")),
      );
    }
    await delay(40);
  }
  assert(await page.$(".outcome-explanation-panel"), "human-vs-bot reaches terminal outcome");
}

async function ownHandLabels(page) {
  return page.$$eval('[data-testid="plain-tricks-own-hand"] .plain-card', (buttons) =>
    buttons
      .map((button) => (button.getAttribute("aria-label") ?? button.textContent ?? "").replace(/^Play\s+/i, "").trim())
      .filter(Boolean),
  );
}

async function assertReplayViewerNoForbidden(page, playedLabel) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assertNoForbiddenTerms(surface, "plain_tricks public replay viewer", [
    ...cardIds.filter((cardId) => cardId !== labelToId(playedLabel)),
    ...internalTerms,
  ]);
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

function labelToId(label) {
  return label.toLowerCase().replace(/\s+/g, "_");
}

function delay(ms) {
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
