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
  "stock_order",
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

  for (const seatCount of [2, 4, 6]) {
    await startMeldfall(page, baseUrl, "Bot vs bot", 190 + seatCount, seatCount);
    await page.waitForSelector('[data-testid="meldfall-ledger-board"]');
    await assertSetupSurface(page, seatCount);
    await assertObserverNoPrivateHand(page);
    if (seatCount === 2) {
      await assertSettlementBreakdown(page);
      assertNoForbiddenTerms(await fullBrowserSurface(page), "settlement DOM and attributes", cardIds);
    }
  }

  await startMeldfall(page, baseUrl, "Hotseat", 216, 4);
  await page.waitForSelector('[data-testid="meldfall-ledger-board"]');
  await waitForText(page, "Seat 2 acts");
  await assertMeldfallA11y(page);
  await assertNoGenericActionPanel(page);
  await assertKeyboardFocus(page, '[data-testid="meldfall-discard-card"]');
  const privateCardCount = await page.$$eval(".meldfall-private-card, .meldfall-card.private", (nodes) => nodes.length);
  assert(privateCardCount >= 7, `hotseat view renders a large private hand, got ${privateCardCount}`);

  await clickSeatFrameButton(page, "Observer");
  await waitForViewer(page, "observer");
  await assertObserverNoPrivateHand(page);
  assertNoForbiddenTerms(await fullBrowserSurface(page), "observer surface", internalTerms);
  await assertStorageClean(page);

  await clickSeatFrameButton(page, "Seat 2");
  await waitForViewer(page, "seat_1");
  await clickEnabled(page, '[data-testid="meldfall-discard-card"]');
  await waitForText(page, "Table phase");
  await waitForText(page, "Meld new");
  await clickActionByText(page, "Meld new");
  await waitForTableCards(page, 3);
  await waitForText(page, "Lay off");
  await clickActionByText(page, "Lay off");
  await waitForTableCards(page, 4);
  await clickActionByText(page, "Finish turn");
  await waitForText(page, "Discard phase");
  await clickActionByText(page, "Discard");
  await waitForText(page, "Seat 3 acts");
  await assertTableauRendered(page);

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"game_id":"meldfall_ledger"') || replayText.includes('"game_id": "meldfall_ledger"'), "export keeps meldfall_ledger game id");
  assert(replayText.includes("meldfall_ledger_viewer_scoped_observation_v1"), "export is viewer scoped");
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  await startMeldfall(page, baseUrl, "Hotseat", 1, 4);
  await page.waitForSelector('[data-testid="meldfall-ledger-board"]');
  await clickEnabled(page, '[data-testid="meldfall-stock"]');
  await waitForText(page, "Card drawn");
  await waitForText(page, "Meld new");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".meldfall-layout");
  const columns = await page.$eval(".meldfall-layout", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive meldfall_ledger layout remains measurable");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "meldfall_ledger setup actions replay noleak responsive" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startMeldfall(page, baseUrl, modeLabel, seed, seatCount) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Meldfall Ledger");
  await clickText(page, "button", "Meldfall Ledger");
  await page.select('select[aria-label="Supported seats from Rust catalog"]', String(seatCount));
  await setSetupSeed(page, seed);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  if (modeLabel === "Bot vs bot") {
    await clickText(page, "button", "Start Autoplay");
  }
}

async function assertSetupSurface(page, seatCount) {
  const summary = await page.evaluate(() => ({
    board: Boolean(document.querySelector('[data-testid="meldfall-ledger-board"]')),
    seats: document.querySelectorAll(".meldfall-seat").length,
    metrics: document.querySelectorAll(".meldfall-metric").length,
    privateCards: document.querySelectorAll(".meldfall-card.private").length,
    text: document.body.textContent ?? "",
  }));
  assert(summary.board, "meldfall_ledger board renders");
  assert(summary.seats === seatCount, `meldfall_ledger renders ${seatCount} seat ledgers, got ${summary.seats}`);
  assert(summary.metrics >= 4, "meldfall_ledger status metrics render");
  if (seatCount === 2) {
    assert(summary.text.includes("13"), "2-seat setup exposes 13-card hand counts");
  }
}

async function assertMeldfallA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="meldfall-ledger-board"]')),
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
    };
  });
  assert(summary.board, "meldfall_ledger board renders");
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
}

async function assertObserverNoPrivateHand(page) {
  const summary = await page.evaluate(() => ({
    privateCards: document.querySelectorAll(".meldfall-card.private").length,
    hidden: Boolean(document.querySelector('[data-testid="meldfall-private-hidden"]')),
    surface: [
      document.body.textContent ?? "",
      ...Array.from(document.querySelectorAll("*")).flatMap((element) =>
        Array.from(element.attributes).map((attr) => `${attr.name}=${attr.value}`),
      ),
    ].join("\n").toLowerCase(),
  }));
  assert(summary.privateCards === 0, `observer/public mode must not mount private cards, got ${summary.privateCards}`);
  assert(summary.hidden, "observer/public mode renders hidden private-hand notice");
  assertNoForbiddenTerms(summary.surface, "observer DOM and attributes", cardIds);
}

async function assertTableauRendered(page) {
  const summary = await page.evaluate(() => ({
    groups: document.querySelectorAll(".meldfall-group").length,
    cards: document.querySelectorAll('[data-testid="meldfall-table-card"]').length,
    actions: document.querySelectorAll('[data-testid^="meldfall-action-"]').length,
  }));
  assert(summary.groups >= 1, "meldfall_ledger renders a public meld group");
  assert(summary.cards >= 4, `meldfall_ledger renders meld plus lay-off cards, got ${summary.cards}`);
  assert(summary.actions > 0, "meldfall_ledger uses board-native Rust action buttons");
}

async function assertSettlementBreakdown(page) {
  await page.waitForFunction(
    () => {
      const panel = document.querySelector(".meldfall-settlement");
      const text = panel?.textContent ?? "";
      return (
        text.includes("Last round settled") &&
        (/Stock exhausted/.test(text) || /Seat \d+ went out/.test(text)) &&
        text.includes("tabled") &&
        text.includes("held penalty") &&
        text.includes("cards held at settlement") &&
        /[+-]\d+/.test(text)
      );
    },
    { timeout: 20000 },
  );

  const summary = await page.evaluate(() => {
    const panel = document.querySelector(".meldfall-settlement");
    return {
      text: panel?.textContent ?? "",
      seats: panel?.querySelectorAll(".meldfall-settlement-seat").length ?? 0,
      breakdowns: panel?.querySelectorAll(".meldfall-settlement-breakdown").length ?? 0,
    };
  });
  assert(summary.seats >= 2, `settlement renders per-seat rows, got ${summary.seats}: ${summary.text}`);
  assert(summary.breakdowns >= 2, `settlement renders per-seat breakdowns, got ${summary.breakdowns}: ${summary.text}`);
}

async function waitForTableCards(page, count) {
  await page.waitForFunction(
    (expected) => document.querySelectorAll('[data-testid="meldfall-table-card"]').length >= expected,
    {},
    count,
  );
}

async function clickActionByText(page, text) {
  const clicked = await page.$$eval(
    '[data-testid^="meldfall-action-"]',
    (buttons, target) => {
      const button = buttons.find((candidate) => !candidate.disabled && (candidate.textContent ?? "").includes(target));
      if (!button) return false;
      button.click();
      return true;
    },
    text,
  );
  assert(clicked, `clicked Meldfall action containing ${text}`);
}

async function clickEnabled(page, selector) {
  const clicked = await page.$$eval(selector, (nodes) => {
    const button = nodes.find((candidate) => candidate instanceof HTMLButtonElement && !candidate.disabled);
    if (!button) return false;
    button.click();
    return true;
  });
  assert(clicked, `clicked enabled ${selector}`);
}

async function assertKeyboardFocus(page, selector) {
  await page.focus(selector);
  const focused = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0;
  });
  assert(focused, `${selector} receives measurable keyboard focus`);
}

async function clickSeatFrameButton(page, text) {
  await clickLabel(page, text);
}

async function waitForViewer(page, value) {
  await page.waitForFunction(
    (viewer) => {
      const input = document.querySelector(`input[name="seat-frame-viewer"][value="${viewer}"]`);
      return input instanceof HTMLInputElement && input.checked;
    },
    {},
    value,
  );
}

async function assertNoGenericActionPanel(page) {
  const hasPanel = await page.evaluate(() => Boolean(document.querySelector(".action-panel")));
  assert(!hasPanel, "meldfall_ledger uses board-native controls without the generic action panel");
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
