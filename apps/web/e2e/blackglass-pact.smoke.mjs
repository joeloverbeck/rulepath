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
const ranks = ["two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen", "king", "ace"];
const suits = ["clubs", "diamonds", "hearts", "spades"];
const cardIds = ranks.flatMap((rank) => suits.map((suit) => `${rank}_${suit}`));
const cardLabels = ranks.flatMap((rank) => suits.map((suit) => `${rank} ${suit}`));
const internalTerms = [
  "deck_order",
  "seed_evidence",
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
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
  browser = await launchBrowser(executablePath);

  const page = await browser.newPage();
  const consoleMessages = [];
  page.on("console", (message) => consoleMessages.push(message.text()));
  page.on("pageerror", (error) => consoleMessages.push(error.message));
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);

  await startBlackglass(page, baseUrl, "Bot vs bot", 18);
  await page.waitForSelector('[data-testid="blackglass-pact-board"]');
  await assertBlackglassA11y(page);
  await assertSeatFrameFixedFour(page);
  await assertTeamGrouping(page);
  await assertBlindPhaseNoCards(page, consoleMessages, "bot observer blind phase");
  await assertNoGenericActionPanel(page);

  await startBlackglass(page, baseUrl, "Hotseat", 18);
  await page.waitForSelector('[data-testid="blackglass-pact-board"]');
  await assertBlackglassA11y(page);
  await assertBlindPhaseNoCards(page, consoleMessages, "hotseat blind phase");
  await focusFirstBlackglassAction(page);
  await assertFocusedVisible(page);
  await advanceUntilOwnHand(page);
  const ownLabels = await ownCardLabels(page);
  assert(ownLabels.length === 13, `hotseat private view exposes 13 own cards after deal, got ${ownLabels.length}`);
  await assertHandSorted(page, "hotseat dealt hand");
  await assertSeatNoLeak(page, consoleMessages, ownLabels, "hotseat dealt hand");
  await assertBidOrPlayControls(page);

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"game_id": "blackglass_pact"'), "export keeps blackglass_pact game id");
  assertNoForbiddenTerms(replayText, "public replay export", [...cardIds, ...internalTerms]);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await page.waitForSelector('[data-testid="blackglass-pact-board"]');
  await assertNoForbiddenTerms(await fullBrowserSurface(page), "public replay viewer", [...cardIds, ...internalTerms]);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".blackglass-board.reduced");
  const animationName = await page.$eval(".blackglass-board.reduced .blackglass-card", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses blackglass_pact card animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".blackglass-table");
  const columns = await page.$eval(".blackglass-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive blackglass_pact table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "blackglass_pact fixed-four board replay noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startBlackglass(page, baseUrl, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Blackglass Pact");
  await clickText(page, "button", "Blackglass Pact");
  await setSetupSeed(page, seed);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
}

async function assertBlackglassA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="blackglass-pact-board"]')),
      seats: document.querySelectorAll(".blackglass-seat").length,
      teams: document.querySelectorAll(".blackglass-team").length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
    };
  });
  assert(summary.board, "blackglass_pact board renders");
  assert(summary.seats === 4, `blackglass_pact renders four seats, got ${summary.seats}`);
  assert(summary.teams === 2, `blackglass_pact renders two team summaries, got ${summary.teams}`);
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
}

async function assertSeatFrameFixedFour(page) {
  const labels = await page.$$eval(".seat-frame-viewers label", (nodes) => nodes.map((node) => node.textContent?.trim() ?? ""));
  assert(labels.includes("Observer"), "viewer selector exposes Observer");
  for (const label of ["North", "East", "South", "West"]) {
    assert(labels.includes(label), `viewer selector exposes ${label}`);
  }
}

async function assertTeamGrouping(page) {
  const text = await fullBrowserSurface(page);
  assert(text.includes("north-south"), "board labels the North-South team");
  assert(text.includes("east-west"), "board labels the East-West team");
  assert(text.includes("contract"), "board renders Rust-projected contract fields");
  assert(text.includes("bags"), "board renders Rust-projected bag fields");
}

async function assertBlindPhaseNoCards(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("hidden"), `${label} states private cards are hidden`);
  assertNoForbiddenTerms(surface, label, [...cardIds, ...cardLabels, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function advanceUntilOwnHand(page) {
  for (let i = 0; i < 12; i += 1) {
    const count = await page.$$eval(".blackglass-private .blackglass-card", (buttons) => buttons.length);
    if (count === 13) {
      return;
    }
    const clicked = await page.$$eval('[data-testid^="blackglass-action-"]', (buttons) => {
      const button = buttons.find((candidate) => !candidate.disabled);
      if (!button) return false;
      button.click();
      return true;
    });
    assert(clicked, "blackglass_pact has a Rust-provided blind or bid action to advance");
    await page.waitForFunction(
      (previous) => document.querySelectorAll(".blackglass-private .blackglass-card").length !== previous,
      { timeout: 3000 },
      count,
    ).catch(() => undefined);
  }
  const finalCount = await page.$$eval(".blackglass-private .blackglass-card", (buttons) => buttons.length);
  assert(finalCount === 13, `blackglass_pact reached dealt owner hand, got ${finalCount}`);
}

async function assertBidOrPlayControls(page) {
  const controls = await page.evaluate(() => ({
    bidButtons: document.querySelectorAll('[data-testid^="blackglass-action-bid"]').length,
    legalCards: document.querySelectorAll(".blackglass-card.legal").length,
  }));
  assert(
    controls.bidButtons > 0 || controls.legalCards > 0,
    `blackglass_pact exposes Rust-provided bid or play controls: ${JSON.stringify(controls)}`,
  );
}

async function assertSeatNoLeak(page, consoleMessages, allowedLabels, label) {
  const normalizedAllowed = new Set(allowedLabels.map((value) => value.toLowerCase()));
  const visibleCardLabels = await page.$$eval(".blackglass-card-face", (nodes) =>
    nodes.map((node) => (node.getAttribute("data-card-label") ?? "").trim().toLowerCase()).filter(Boolean),
  );
  const forbiddenVisible = visibleCardLabels.filter((value) => !normalizedAllowed.has(value));
  assert(forbiddenVisible.length === 0, `${label} rendered non-owner cards: ${forbiddenVisible.join(", ")}`);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertHandSorted(page, label) {
  const faces = await page.$$eval(".blackglass-private .blackglass-card .blackglass-card-face", (nodes) =>
    nodes.map((node) => (node.getAttribute("data-card-label") ?? "").trim()),
  );
  assert(faces.length > 0, `${label} renders owner card faces for sort check`);
  const suitOrder = { D: 0, C: 1, H: 2, S: 3 };
  const rankOrder = { 2: 0, 3: 1, 4: 2, 5: 3, 6: 4, 7: 5, 8: 6, 9: 7, 10: 8, J: 9, Q: 10, K: 11, A: 12 };
  const keys = faces.map((face) => [suitOrder[face.slice(-1)], rankOrder[face.slice(0, -1)]]);
  for (let index = 1; index < keys.length; index += 1) {
    const prev = keys[index - 1];
    const next = keys[index];
    const ordered = prev[0] < next[0] || (prev[0] === next[0] && prev[1] <= next[1]);
    assert(ordered, `${label} hand not sorted by suit then rank: ${faces[index - 1]} before ${faces[index]}`);
  }
}

async function ownCardLabels(page) {
  return page.$$eval(".blackglass-private .blackglass-card", (buttons) =>
    buttons.map((button) => (button.getAttribute("aria-label") ?? button.textContent ?? "").trim().toLowerCase()),
  );
}

async function focusFirstBlackglassAction(page) {
  await page.focus('[data-testid^="blackglass-action-"]');
}

async function assertNoGenericActionPanel(page) {
  const hasPanel = await page.evaluate(() => Boolean(document.querySelector(".action-panel")));
  assert(!hasPanel, "blackglass_pact uses its board-native controls without the generic action panel");
}

async function setSetupSeed(page, seed) {
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

async function clickLabel(page, label) {
  const clicked = await page.$$eval(
    "label",
    (labels, target) => {
      const labelElement = labels.find((candidate) => (candidate.textContent ?? "").includes(target));
      const control = labelElement?.control;
      if (!control) return false;
      control.click();
      return true;
    },
    label,
  );
  assert(clicked, `label exists: ${label}`);
}

async function clickText(page, selector, text) {
  const clicked = await page.$$eval(
    selector,
    (elements, target) => {
      const element = elements.find((candidate) => (candidate.textContent ?? "").includes(target));
      if (!element) return false;
      element.click();
      return true;
    },
    text,
  );
  assert(clicked, `clicked ${selector} containing ${text}`);
}

async function waitForText(page, text) {
  await page.waitForFunction((needle) => document.body.textContent?.includes(needle), {}, text);
}

async function assertFocusedVisible(page) {
  const focused = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0;
  });
  assert(focused, "keyboard focus is visible on a measurable element");
}

async function fullBrowserSurface(page) {
  return page.evaluate(() => {
    const attrs = Array.from(document.querySelectorAll("*")).flatMap((element) =>
      Array.from(element.attributes).map((attr) => `${attr.name}=${attr.value}`),
    );
    return [document.body.textContent ?? "", ...attrs, window.location.href].join("\n").toLowerCase();
  });
}

async function assertStorageClean(page) {
  const storage = await page.evaluate(async () => ({
    local: { ...localStorage },
    session: { ...sessionStorage },
    url: window.location.href,
  }));
  assertNoForbiddenTerms(JSON.stringify(storage), "browser storage", [...cardIds, ...internalTerms]);
}

function assertNoForbiddenTerms(surface, label, terms) {
  const normalized = surface.toLowerCase();
  const leaked = terms.filter(Boolean).filter((term) => normalized.includes(String(term).toLowerCase()));
  assert(leaked.length === 0, `${label} leaked forbidden terms: ${leaked.join(", ")}`);
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
