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
const internalTerms = [
  "deck_tail",
  "deck tail",
  "reserved_community",
  "reserved community",
  "burn",
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

  const selectedSeatCount = 4;
  await startRiverLedger(page, baseUrl, "Human vs bot", selectedSeatCount);
  await page.waitForSelector('[data-testid="river-ledger-board"]');
  await waitForText(page, "Seat 0 to choose");
  await assertRiverLedgerA11y(page, true, selectedSeatCount);
  const seat0Cards = await ownPrivateCardLabels(page);
  assert(seat0Cards.length === 2, "seat 0 private view exposes two own cards");
  await assertNoLeak(page, consoleMessages, "seat 0 view", cardIds.filter((id) => !seat0Cards.map(labelToId).includes(id)));

  await clickSeatFrameButton(page, "Observer");
  await waitForText(page, "Observer view");
  await assertNoLeak(page, consoleMessages, "observer view", cardIds);
  await assertStorageClean(page);

  await clickSeatFrameButton(page, "Seat 1");
  await waitForText(page, "Seat 1 view");
  const seat1Cards = await ownPrivateCardLabels(page);
  assert(seat1Cards.length === 2, "seat 1 private view exposes two own cards");
  await assertNoLeak(page, consoleMessages, "wrong-seat view", seat0Cards.map(labelToId));

  await clickSeatFrameButton(page, "Seat 0");
  await waitForText(page, "Available choices");
  const choices = await page.$$eval('[data-testid^="choice-river-ledger-"]', (buttons) =>
    buttons.map((button) => ({
      label: button.textContent ?? "",
      disabled: button.disabled,
      aria: button.getAttribute("aria-label") ?? "",
    })),
  );
  assert(choices.length > 0, "river_ledger exposes Rust-provided legal action buttons");
  assert(choices.every((choice) => choice.aria.length > 0), "river_ledger choices have accessible names");
  assert(choices.every((choice) => !choice.disabled), "river_ledger renders legal enabled choices only for active human seat");

  await clickText(page, "button", "Fold");
  await page.waitForFunction(() => document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]'), {
    timeout: 15000,
  });
  await waitForText(page, "Outcome");
  const outcomeText = await page.$eval(".outcome-explanation-panel", (element) => element.textContent ?? "");
  assert(outcomeText.includes("Seat"), "river_ledger terminal outcome names seats");
  assertNoForbiddenTerms(await fullBrowserSurface(page), "terminal surface", internalTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".river-ledger-layout");
  const columns = await page.$eval(".river-ledger-layout", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive river_ledger layout remains measurable");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "river_ledger noleak legal controls terminal responsive" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startRiverLedger(page, baseUrl, modeLabel, seatCount) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "River Ledger");
  await assertFixedSeatSetup(page);
  await clickText(page, "button", "River Ledger");
  await assertVariableSeatSetup(page, seatCount);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
}

async function assertRiverLedgerA11y(page, expectChoices, expectedSeatCount) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="river-ledger-board"]')),
      seats: document.querySelectorAll(".river-ledger-seat").length,
      choices: document.querySelectorAll('[data-testid^="choice-river-ledger-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".river-ledger-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "river_ledger board renders");
  assert(
    summary.seats === expectedSeatCount,
    `river_ledger renders ${expectedSeatCount} selected seat rows, got ${summary.seats}`,
  );
  if (expectChoices) {
    assert(summary.choices > 0, "river_ledger exposes Rust-provided action buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "river_ledger latest-effect region has text");
}

async function assertFixedSeatSetup(page) {
  const summary = await page.evaluate(() => {
    const setup = document.querySelector(".setup-region");
    const seatSelect = setup?.querySelector('select[aria-label="Supported seats from Rust catalog"]');
    const seatOutput = setup?.querySelector('.seat-count-static[aria-label="Supported seats from Rust catalog"]');
    return {
      hasSeatSelect: Boolean(seatSelect),
      outputText: seatOutput?.textContent?.trim() ?? "",
      caption: seatOutput?.closest(".field")?.querySelector("small")?.textContent ?? "",
    };
  });
  assert(!summary.hasSeatSelect, "fixed-seat setup renders no seat select");
  assert(summary.outputText === "2 seats", `fixed-seat setup shows static two-seat value: ${summary.outputText}`);
  assert(summary.caption.includes("Fixed at 2 seats."), `fixed-seat caption is read-only: ${summary.caption}`);
}

async function assertVariableSeatSetup(page, seatCount) {
  await page.waitForSelector('select[aria-label="Supported seats from Rust catalog"]');
  const summary = await page.evaluate(() => {
    const select = document.querySelector('select[aria-label="Supported seats from Rust catalog"]');
    return {
      disabled: select?.disabled ?? true,
      options: Array.from(select?.querySelectorAll("option") ?? []).map((option) => option.value),
    };
  });
  assert(!summary.disabled, "variable-seat setup select is enabled");
  assert(
    ["3", "4", "5", "6"].every((value) => summary.options.includes(value)),
    `river_ledger setup exposes supported seat counts: ${summary.options.join(", ")}`,
  );
  await page.select('select[aria-label="Supported seats from Rust catalog"]', String(seatCount));
}

async function ownPrivateCardLabels(page) {
  return page.$$eval(".river-ledger-private .river-ledger-card.private strong", (cards) =>
    cards.map((card) => card.textContent?.trim() ?? "").filter(Boolean),
  );
}

async function assertNoLeak(page, consoleMessages, label, forbiddenCardIds) {
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, [...forbiddenCardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function fullBrowserSurface(page) {
  return page.evaluate(() =>
    [
      document.body.textContent ?? "",
      Array.from(document.querySelectorAll("*"))
        .flatMap((element) => Array.from(element.attributes).map((attribute) => `${attribute.name}=${attribute.value}`))
        .join("\n"),
      Array.from(document.querySelectorAll("[data-testid]"))
        .map((element) => element.getAttribute("data-testid"))
        .join("\n"),
      Object.keys(localStorage).join("\n"),
      Object.values(localStorage).join("\n"),
      Object.keys(sessionStorage).join("\n"),
      Object.values(sessionStorage).join("\n"),
    ].join("\n"),
  );
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

async function clickSeatFrameButton(page, text) {
  const handle = await waitForTextHandle(page, ".seat-frame-viewers button", text);
  await handle.click();
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
  const match = label.match(/^([2-9]|10|J|Q|K|A)([CDHS])$/);
  if (!match) return label.toLowerCase();
  const rank = {
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    J: "jack",
    Q: "queen",
    K: "king",
    A: "ace",
  }[match[1]];
  const suit = { C: "clubs", D: "diamonds", H: "hearts", S: "spades" }[match[2]];
  return `${rank}_${suit}`;
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
