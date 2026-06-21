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
const cardIds = ["low_dawn", "low_dusk", "middle_dawn", "middle_dusk", "high_dawn", "high_dusk"];
const cardLabels = ["Sprout Dawn", "Sprout Dusk", "Current Dawn", "Current Dusk", "Crown Dawn", "Crown Dusk"];
const rankTerms = ['"rank":"low"', '"rank":"middle"', '"rank":"high"', "rank=low", "rank=middle", "rank=high"];
const internalTerms = [
  "deck_order",
  "deck order",
  "bot_candidate",
  "candidate_ranking",
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
  "seed_evidence",
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

  await startPokerLite(page, baseUrl, "Bot vs bot");
  await page.waitForSelector('[data-testid="poker-lite-board"]');
  await assertPokerLiteA11y(page, false);
  await assertObserverNoLeak(page, consoleMessages, "bot_vs_bot observer initial");

  await startPokerLite(page, baseUrl, "Hotseat");
  await page.waitForSelector(".poker-lite-private .poker-lite-card.private");
  await assertPokerLiteA11y(page, true);
  await assertSeatPrivateView(page);
  await assertStrengthCue(page, "hidden");
  await focusByTestId(page, "choice-poker-lite-round-0-1");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Pressed");
  await assertNoHiddenIds(page, consoleMessages, "after press before center reveal");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Viewer filtered");
  await assertNoHiddenIds(page, consoleMessages, "developer panel before reveal");
  await clickText(page, "button", "Submit Stale Action");
  await waitForText(page, "stale_action");
  await assertNoHiddenIds(page, consoleMessages, "stale diagnostic before reveal");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "viewer_scoped_observation_v1"'), "export is viewer-scoped public timeline");
  assert(replayText.includes('"game_id": "poker_lite"'), "export keeps poker_lite game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assert(!replayText.includes('"seed_evidence"'), "public export omits seed evidence");
  assertNoForbiddenTerms(replayText, "pre-reveal replay export", [...cardIds, ...cardLabels, ...rankTerms, ...internalTerms]);

  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Cursor 0 /");
  await clickText(page, ".replay-actions button", "Step");
  await waitForText(page, "Cursor 1 /");
  await assertReplayViewerNoForbidden(page);

  await clearReplayDocument(page);
  await clickText(page, "button", "Developer panel");
  await clickText(page, "button", "Match");
  await waitForText(page, "Center revealed");
  await page.waitForSelector(".poker-lite-center .poker-lite-card.center");
  await clickText(page, "button", "Hold");
  await waitForText(page, "Held");
  await clickText(page, "button", "Hold");
  await waitForText(page, "Showdown revealed");
  await waitForText(page, "Showdown");
  await assertShowdownRendered(page);

  await startPokerLite(page, baseUrl, "Hotseat", 2);
  await clickText(page, "button", "Hold");
  await waitForText(page, "Held");
  await clickText(page, "button", "Hold");
  await waitForText(page, "Center revealed");
  await assertStrengthCue(page, "revealed");
  await clickText(page, "button", "Hold");
  await waitForText(page, "Held");
  await clickText(page, "button", "Hold");
  await waitForText(page, "wins on the revealed showdown rank");
  await assertPrivateRankOutcomeRendered(page);

  await startPokerLite(page, baseUrl, "Hotseat");
  await clickText(page, "button", "Press");
  await waitForText(page, "Pressed");
  await clickText(page, "button", "Yield");
  await waitForText(page, "wins after");
  const hasShowdown = await page.$(".poker-lite-showdown");
  assert(!hasShowdown, "yield terminal renders without showdown reveal panel");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".poker-lite-board.reduced");
  const animationName = await page.$eval(".poker-lite-board.reduced .poker-lite-card", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses poker_lite crest animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".poker-lite-table");
  const columns = await page.$eval(".poker-lite-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive poker_lite table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "poker_lite noleak reveal replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startPokerLite(page, baseUrl, modeLabel, seed = null) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Crest Ledger");
  await clickText(page, "button", "Crest Ledger");
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
  await page.waitForSelector('[data-testid="poker-lite-board"]');
}

async function assertPokerLiteA11y(page, expectChoices) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="poker-lite-board"]')),
      choices: document.querySelectorAll('[data-testid^="choice-poker-lite-round-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".poker-lite-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "poker_lite board renders");
  if (expectChoices) {
    assert(summary.choices > 0, "poker_lite exposes Rust-provided action buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "poker_lite latest-effect region has text");
}

async function assertObserverNoLeak(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assert(surface.includes("Observer view"), "observer mode is visible");
  assert(surface.includes("Center crest hidden"), "center remains hidden to observer");
  assertNoForbiddenTerms(surface, label, [...cardIds, ...cardLabels, ...rankTerms, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertStrengthCue(page, phase) {
  const cue = await page.evaluate(() => {
    const element = document.querySelector('[data-testid="poker-lite-strength"]');
    return element ? { tone: element.getAttribute("data-tone"), text: element.textContent ?? "" } : null;
  });
  assert(cue !== null, "private crest shows a showdown-strength cue");
  if (phase === "hidden") {
    assert(cue.tone === "hidden", `before reveal the cue notes the center is hidden, got tone=${cue.tone}`);
    assert(/center crest is still hidden/i.test(cue.text), `hidden cue explains the center is unknown, got "${cue.text}"`);
  } else {
    assert(cue.tone === "pair" || cue.tone === "nopair", `after reveal the cue states pair status, got tone=${cue.tone}`);
    assert(/pair/i.test(cue.text), `revealed cue mentions pair status, got "${cue.text}"`);
  }
}

async function assertSeatPrivateView(page) {
  const summary = await page.evaluate(() => ({
    privateText: document.querySelector(".poker-lite-private")?.textContent ?? "",
    centerText: document.querySelector(".poker-lite-center")?.textContent ?? "",
    testIds: Array.from(document.querySelectorAll("[data-testid]")).map((element) => element.getAttribute("data-testid")),
  }));
  assert(summary.privateText.includes("Private view"), "private panel renders");
  assert(cardLabels.some((label) => summary.privateText.includes(label)), "seat view shows own private crest label");
  assert(summary.centerText.includes("Center crest hidden"), "center stays hidden before reveal");
  assertNoForbiddenTerms(summary.testIds.join("\n"), "poker_lite data-testid values", [...cardIds, ...cardLabels, ...rankTerms]);
}

async function assertNoHiddenIds(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, [...cardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertShowdownRendered(page) {
  const summary = await page.evaluate(() => ({
    showdownText: document.querySelector(".poker-lite-showdown")?.textContent ?? "",
    terminalText: document.querySelector(".outcome-explanation-panel")?.textContent ?? "",
  }));
  assert(summary.showdownText.includes("Seat 1"), "showdown includes seat 1 reveal");
  assert(summary.showdownText.includes("Center"), "showdown includes center reveal");
  assert(summary.showdownText.includes("Seat 2"), "showdown includes seat 2 reveal");
  assert(summary.terminalText.includes("Shared pool") || summary.terminalText.includes("Split ledger"), "outcome panel summarizes ledger");
}

async function assertPrivateRankOutcomeRendered(page) {
  const terminalText = await page.$eval(".outcome-explanation-panel", (panel) => panel.textContent ?? "");
  assert(terminalText.includes("wins on the revealed showdown rank"), "seed 2 showdown uses Rust private-rank rationale");
  assert(!terminalText.includes("wins with a pair"), "seed 2 showdown does not use pair fallback");
}

async function assertReplayViewerNoForbidden(page) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assertNoForbiddenTerms(surface, "poker_lite public replay viewer", [...cardIds, ...cardLabels, ...rankTerms, ...internalTerms]);
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

async function clearReplayDocument(page) {
  await page.$eval("textarea", (element) => {
    const setter = Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, "value")?.set;
    setter?.call(element, "");
    element.dispatchEvent(new Event("input", { bubbles: true }));
  });
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
