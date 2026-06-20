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
const maskIds = [
  "mask_g1_a",
  "mask_g1_b",
  "mask_g1_c",
  "mask_g2_a",
  "mask_g2_b",
  "mask_g2_c",
  "mask_g3_a",
  "mask_g3_b",
  "mask_g3_c",
  "mask_g4_a",
  "mask_g4_b",
  "mask_g4_c",
  "mask_g5_a",
  "mask_g5_b",
  "mask_g5_c",
];
const internalTerms = [
  "reserve",
  "deck_order",
  "hidden_state",
  "private_state",
  "internal_state",
  "seed_evidence",
  "candidate_ranking",
  "bot_candidate",
  "debug_state",
];
const forbiddenTerms = [...maskIds, ...internalTerms];

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

  await startMaskedClaims(page, baseUrl, "Hotseat", 13);
  await assertMaskedClaimsA11y(page, true);
  await assertClaimRiskCues(page);
  await assertNoLeak(page, consoleMessages, "initial hotseat");
  await focusByTestId(page, "masked-claims-claim-turn-0-0-0");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Response");
  await waitForText(page, "may accept or challenge");
  await assertResponseControls(page, true);
  await assertNoLeak(page, consoleMessages, "pending accepted claim");

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Viewer filtered");
  await waitForText(page, "Redacted for hidden-information viewer safety.");
  await assertNoLeak(page, consoleMessages, "developer panel pending claim");
  await clickText(page, "button", "Developer panel");

  await clickText(page, "button", "Accept");
  await waitForText(page, "Claim accepted");
  await waitForText(page, "accepted");
  await assertNoLeak(page, consoleMessages, "accepted veiled claim");

  await clickText(page, "button", "Export Current Run");
  const replayTextHandle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayText = await replayTextHandle.jsonValue();
  assert(replayText.includes('"export_class": "viewer_scoped_observation"'), "export is viewer-scoped public timeline");
  assert(replayText.includes('"game_id": "masked_claims"'), "export keeps masked_claims game id");
  assert(!replayText.includes('"commands"'), "public export omits command stream");
  assertNoForbiddenTerms(replayText, "accepted public replay export", forbiddenTerms);

  await clickText(page, "button", "Import Replay");
  await waitForReplayCursor(page, "Cursor 0 /");
  await assertReplayViewerNoLeak(page);

  await startMaskedClaims(page, baseUrl, "Hotseat", 17);
  await clickTestId(page, "masked-claims-claim-turn-0-0-4");
  await waitForText(page, "Response");
  await clickText(page, "button", "Challenge");
  await waitForText(page, "Mask revealed");
  await waitForText(page, "Challenge resolved");
  await waitForText(page, "Exposed masks");
  await assertNoLeak(page, consoleMessages, "challenge resolution");

  await startMaskedClaims(page, baseUrl, "Bot vs bot", 19);
  await clickText(page, "button", "Step Bot");
  await waitForText(page, "is waiting for a response");
  await assertResponseControls(page, false);
  await assertNoLeak(page, consoleMessages, "bot claimant waiting");
  await clickText(page, "button", "Step Bot");
  await page.waitForFunction(() =>
    ["Claim accepted", "Challenge declared", "Mask revealed", "Challenge resolved"].some((text) =>
      document.body.textContent?.includes(text),
    ),
  );

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".masked-claims-board.reduced");
  const animationName = await page.$eval(".masked-claims-board.reduced .plain-facedown", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses masked_claims card animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".plain-tricks-table");
  const columns = await page.$eval(".plain-tricks-table", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive masked_claims table remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenTerms);

  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  assert(state?.view?.game_id === "masked_claims", "render_game_to_text reports masked_claims view");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "masked_claims reaction noleak replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startMaskedClaims(page, baseUrl, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Masked Claims");
  await clickText(page, "button", "Masked Claims");
  await page.$eval(
    ".field input[type='number']",
    (input, value) => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
      setter?.call(input, String(value));
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    seed,
  );
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="masked-claims-board"]');
}

async function assertMaskedClaimsA11y(page, expectChoices) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="masked-claims-board"]')),
      ownHand: Boolean(document.querySelector('[data-testid="masked-claims-own-hand"]')),
      choices: document.querySelectorAll('[data-testid^="masked-claims-claim-turn-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".plain-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "masked_claims board renders");
  assert(summary.ownHand, "masked_claims seat view renders own hand");
  if (expectChoices) {
    assert(summary.choices > 0, "masked_claims exposes Rust-provided claim buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "masked_claims latest-effect region has text");
}

async function assertClaimRiskCues(page) {
  const summary = await page.evaluate(() => {
    const cards = Array.from(document.querySelectorAll('[data-testid="masked-claims-own-hand"] .plain-card'));
    return {
      legend: document.querySelector('[data-testid="masked-claims-legend"]')?.textContent?.trim() ?? "",
      badges: cards.map((card) => card.querySelector(".masked-grade-badge")?.textContent?.trim() ?? ""),
      relations: cards.map((card) =>
        Array.from(card.querySelectorAll(".masked-declare .masked-claim-cue")).map((cue) => cue.textContent?.trim() ?? ""),
      ),
    };
  });
  assert(/true grade/i.test(summary.legend), `claim risk legend renders, got "${summary.legend}"`);
  assert(
    summary.badges.length > 0 && summary.badges.every((badge) => /true grade \d of \d/i.test(badge)),
    `every held mask shows its true grade rank, got ${JSON.stringify(summary.badges)}`,
  );
  const flat = summary.relations.flat();
  assert(flat.includes("true"), "at least one declaration is tagged as the true grade");
  assert(flat.includes("bluff"), "overclaim declarations are tagged as a bluff");
  assert(flat.includes("under"), "underclaim declarations are tagged as under");
  // Each held mask must expose exactly one truthful declaration.
  assert(
    summary.relations.every((card) => card.filter((relation) => relation === "true").length === 1),
    `each mask has exactly one true-grade declaration, got ${JSON.stringify(summary.relations)}`,
  );
}

async function assertResponseControls(page, expected) {
  const count = await page.$$eval('[data-testid^="masked-claims-response-turn-"]', (buttons) => buttons.length);
  assert(expected ? count === 2 : count === 0, `response controls expected=${expected} count=${count}`);
}

async function assertReplayViewerNoLeak(page) {
  const surface = await page.$eval(".replay-viewer", (element) => element.textContent ?? "");
  assertNoForbiddenTerms(surface, "masked_claims public replay viewer", forbiddenTerms);
}

async function assertNoLeak(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, forbiddenTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbiddenTerms);
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

async function clickTestId(page, testId) {
  await page.click(`[data-testid="${testId}"]`);
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

async function waitForReplayCursor(page, text) {
  try {
    await waitForText(page, text);
  } catch (error) {
    const body = await page.evaluate(() => document.body.textContent ?? "");
    throw new Error(`Replay import did not show ${text}. Body: ${body.slice(0, 1200)}`, { cause: error });
  }
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
