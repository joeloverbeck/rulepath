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
const cardLabels = ranks.flatMap((rank) => suits.map((suit) => `${rank} ${suit}`));
const cardIds = ranks.flatMap((rank) => suits.map((suit) => `${rank}_${suit}`));
const internalTerms = [
  "deck_order",
  "pass_provenance",
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

  await startBriar(page, baseUrl, "Bot vs bot", 17);
  await page.waitForSelector('[data-testid="briar-circuit-board"]');
  await assertBriarA11y(page, false);
  await assertObserverNoLeak(page, consoleMessages);

  await startBriar(page, baseUrl, "Human vs bot", 17);
  await page.waitForSelector('[data-testid="briar-circuit-own-hand"]');
  await assertBriarA11y(page, true);
  await assertBriarControlSurface(page);
  const seat0Labels = await ownCardLabels(page);
  assert(seat0Labels.length === 13, "seat 0 private view exposes 13 own cards");
  await assertSeatNoLeak(page, consoleMessages, seat0Labels, "seat 0 view");

  await page.focus(".briar-card.legal");
  await assertFocusedVisible(page);
  const selectedLabel = await page.$eval(".briar-card.legal", (button) => (button.getAttribute("aria-label") ?? "").toLowerCase());
  await page.keyboard.press("Enter");
  await waitForText(page, "Selected");
  await waitForText(page, "1 of 3 selected");
  await assertSeatNoLeak(page, consoleMessages, [...seat0Labels, selectedLabel], "seat 0 after pass selection");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "viewer_scoped_observation_v1"'), "export is viewer-scoped public timeline");
  assert(replayText.includes('"game_id": "briar_circuit"'), "export keeps briar_circuit game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assertNoForbiddenTerms(replayText, "public replay export", [...cardIds, selectedLabelToForbidden(selectedLabel), ...internalTerms]);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await assertNoForbiddenTerms(await fullBrowserSurface(page), "public replay viewer", [...cardIds, ...internalTerms]);

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".briar-board.reduced");
  const animationName = await page.$eval(".briar-board.reduced .briar-card", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses briar_circuit card animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".briar-table");
  const columns = await page.$eval(".briar-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive briar_circuit table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "briar_circuit noleak keyboard replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startBriar(page, baseUrl, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Briar Circuit");
  await clickText(page, "button", "Briar Circuit");
  await setSetupSeed(page, seed);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
}

async function assertBriarA11y(page, expectChoices) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="briar-circuit-board"]')),
      choices: document.querySelectorAll(".briar-card.legal").length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".briar-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "briar_circuit board renders");
  if (expectChoices) {
    assert(summary.choices > 0, "briar_circuit exposes Rust-provided action buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "briar_circuit latest-effect region has text");
}

async function assertBriarControlSurface(page) {
  const surface = await page.evaluate(() => {
    const panel = document.querySelector(".action-panel");
    return {
      genericPanel: Boolean(panel),
      restart: Boolean(document.querySelector('[data-testid="briar-circuit-restart"]')),
    };
  });
  // Briar Circuit interaction is fully owned by its board (cards + confirm + restart).
  // The generic action panel must not coexist with it: its buttons cannot submit a
  // nested play/pass leaf and were rendered permanently disabled, which is confusing.
  assert(!surface.genericPanel, "briar_circuit does not render the redundant generic action panel");
  assert(surface.restart, "briar_circuit board exposes its own restart control");
}

async function assertObserverNoLeak(page, consoleMessages) {
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("hidden"), "observer hand surface is hidden");
  assertNoForbiddenTerms(surface, "briar_circuit observer", [...cardIds, ...cardLabels, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "briar_circuit observer console", internalTerms);
}

async function assertSeatNoLeak(page, consoleMessages, allowedLabels, label) {
  const normalizedAllowed = new Set(allowedLabels.map(labelToCardLabel));
  const forbiddenLabels = cardLabels.filter((card) => !normalizedAllowed.has(card));
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, [...cardIds, ...forbiddenLabels, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function ownCardLabels(page) {
  return page.$$eval(".briar-card", (buttons) => buttons.map((button) => (button.getAttribute("aria-label") ?? "").toLowerCase()));
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

function selectedLabelToForbidden(label) {
  return labelToCardLabel(label);
}

function labelToCardLabel(label) {
  const normalized = label.toLowerCase().replace(/^select\s+/, "").replace(/^unselect\s+/, "").replace(/^play\s+/, "").trim();
  return normalized;
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
