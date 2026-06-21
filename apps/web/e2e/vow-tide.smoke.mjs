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
const cardIds = [
  "two_clubs",
  "three_clubs",
  "four_clubs",
  "five_clubs",
  "six_clubs",
  "seven_clubs",
  "eight_clubs",
  "nine_clubs",
  "ten_clubs",
  "jack_clubs",
  "queen_clubs",
  "king_clubs",
  "ace_clubs",
  "two_diamonds",
  "three_diamonds",
  "four_diamonds",
  "five_diamonds",
  "six_diamonds",
  "seven_diamonds",
  "eight_diamonds",
  "nine_diamonds",
  "ten_diamonds",
  "jack_diamonds",
  "queen_diamonds",
  "king_diamonds",
  "ace_diamonds",
  "two_hearts",
  "three_hearts",
  "four_hearts",
  "five_hearts",
  "six_hearts",
  "seven_hearts",
  "eight_hearts",
  "nine_hearts",
  "ten_hearts",
  "jack_hearts",
  "queen_hearts",
  "king_hearts",
  "ace_hearts",
  "two_spades",
  "three_spades",
  "four_spades",
  "five_spades",
  "six_spades",
  "seven_spades",
  "eight_spades",
  "nine_spades",
  "ten_spades",
  "jack_spades",
  "queen_spades",
  "king_spades",
  "ace_spades",
];
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

  await startVow(page, baseUrl, "Bot vs bot", 3, 17);
  await page.waitForSelector('[data-testid="vow-tide-board"]');
  await assertSeatCount(page, 3);
  await assertObserverNoLeak(page, consoleMessages, "3-seat observer");

  await startVow(page, baseUrl, "Bot vs bot", 7, 18);
  await page.waitForSelector('[data-testid="vow-tide-board"]');
  await assertSeatCount(page, 7);
  await assertViewerSelector(page, 7);
  await assertObserverNoLeak(page, consoleMessages, "7-seat observer");

  await startVow(page, baseUrl, "Hotseat", 7, 19);
  await page.waitForSelector('[data-testid="vow-tide-board"]');
  await assertVowA11y(page, true);
  const firstSeatLabels = await ownCardLabels(page);
  assert(firstSeatLabels.length === 7, `initial hotseat hand exposes 7 own cards, got ${firstSeatLabels.length}`);
  await focusFirstBid(page);
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Bid accepted");
  await waitForText(page, "Tide 3 to bid");
  await assertHotseatHandoff(page, firstSeatLabels);

  for (let seat = 3; seat <= 7; seat += 1) {
    await waitForText(page, `Tide ${seat} to bid`);
    await clickFirstBid(page);
  }
  await waitForText(page, "Tide 1 to bid");
  await assertNoBid(page, 7);
  await clickFirstBid(page);
  await waitForText(page, "to play");
  await page.waitForSelector(".vow-tide-card.legal");
  await page.focus(".vow-tide-card.legal");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Card played");
  await assertVowA11y(page, false);
  await assertNoGenericActionPanel(page);

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"game_id": "vow_tide"'), "export keeps vow_tide game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assertNoForbiddenTerms(replayText, "public replay export", internalTerms);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await assertNoForbiddenTerms(await fullBrowserSurface(page), "public replay viewer", internalTerms);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".vow-tide-board.reduced");
  const animationName = await page.$eval(".vow-tide-board.reduced .vow-tide-card-face", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses vow_tide card-face animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".vow-tide-table");
  const columns = await page.$eval(".vow-tide-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive vow_tide table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "vow_tide seats bidding dealer-hook replay noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startVow(page, baseUrl, modeLabel, seatCount, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Vow Tide");
  await clickText(page, "button", "Vow Tide");
  await setSetupSeed(page, seed);
  await page.select('.field select[aria-label="Supported seats from Rust catalog"]', String(seatCount));
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
}

async function assertVowA11y(page, expectBids) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="vow-tide-board"]')),
      bidButtons: document.querySelectorAll('[data-testid^="vow-tide-action-bid-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".vow-tide-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "vow_tide board renders");
  if (expectBids) {
    assert(summary.bidButtons > 0, "vow_tide exposes Rust-provided bid controls");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "vow_tide live status has text");
}

async function assertSeatCount(page, expected) {
  const seatCount = await page.$$eval(".vow-tide-seat", (seats) => seats.length);
  assert(seatCount === expected, `vow_tide renders ${expected} seats, got ${seatCount}`);
}

async function assertViewerSelector(page, seatCount) {
  const labels = await page.$$eval(".seat-frame-viewers label", (nodes) => nodes.map((node) => node.textContent?.trim() ?? ""));
  assert(labels.includes("Observer"), "viewer selector exposes Observer");
  for (let index = 1; index <= seatCount; index += 1) {
    assert(labels.includes(`Tide ${index}`), `viewer selector exposes Tide ${index}`);
  }
  for (let index = 1; index <= seatCount; index += 1) {
    await clickText(page, ".seat-frame-viewers label", `Tide ${index}`);
    await page.waitForFunction(
      (value) => document.querySelector(`.seat-frame-viewers input[value="${value}"]`)?.checked === true,
      {},
      `seat_${index - 1}`,
    );
  }
  await clickText(page, ".seat-frame-viewers label", "Observer");
  await page.waitForFunction(() => document.querySelector('.seat-frame-viewers input[value="observer"]')?.checked === true);
  await page.waitForSelector('[data-testid="vow-tide-private-hidden"]');
}

async function assertObserverNoLeak(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("hidden for observer"), `${label} states private hand is hidden`);
  assertNoForbiddenTerms(surface, label, [...cardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertHotseatHandoff(page, previousLabels) {
  const labels = await ownCardLabels(page);
  assert(labels.length === 7, `next hotseat hand exposes 7 own cards, got ${labels.length}`);
  const prior = new Set(previousLabels);
  const overlap = labels.filter((label) => prior.has(label));
  assert(overlap.length < previousLabels.length, "hotseat handoff refreshes the private hand for the active seat");
}

async function assertNoGenericActionPanel(page) {
  const hasPanel = await page.evaluate(() => Boolean(document.querySelector(".action-panel")));
  assert(!hasPanel, "vow_tide uses its board-native controls without the generic action panel");
}

async function assertNoBid(page, bid) {
  const hasBid = await page.evaluate(
    (value) =>
      Array.from(document.querySelectorAll('[data-testid^="vow-tide-action-bid-"]')).some(
        (button) => (button.textContent ?? "").trim().startsWith(`Bid ${value}`),
      ),
    bid,
  );
  assert(!hasBid, `dealer hook omits forbidden bid ${bid}`);
}

async function ownCardLabels(page) {
  return page.$$eval(".vow-tide-private .vow-tide-card", (buttons) =>
    buttons.map((button) => (button.getAttribute("aria-label") ?? button.textContent ?? "").trim().toLowerCase()),
  );
}

async function focusFirstBid(page) {
  await page.focus('[data-testid^="vow-tide-action-bid-"]');
}

async function clickFirstBid(page) {
  await page.click('[data-testid^="vow-tide-action-bid-"]');
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
