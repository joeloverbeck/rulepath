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
  "hidden_state",
  "private_state",
  "internal_state",
  "candidate_ranking",
  "bot_candidate",
  "deck_tail",
  "opponent_private",
  "unrevealed_private",
  "secret_commitment",
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

  const catalog = await catalogNames(page);
  assert(catalog.length >= 9, `catalog has outcome coverage candidates: ${catalog.join(", ")}`);

  await playRaceToTerminal(page, baseUrl);
  await assertOutcomePanel(page, "Race to 21");
  await assertOutcomeStyled(page, "Race to 21");
  await assertHumanizedOutcomeCopy(page, "Race to 21");
  const racePanel = await panelText(page);
  await setReducedMotion(page);
  assert((await panelText(page)) === racePanel, "reduced motion preserves race outcome text");
  await assertNoLeak(page, consoleMessages, "race outcome");

  await playThreeMarksWin(page, baseUrl);
  await assertOutcomePanel(page, "Three Marks");
  await assertOutcomeStyled(page, "Three Marks");
  await assertHumanizedOutcomeCopy(page, "Three Marks");
  await assertWinnerFirstStanding(page, "Three Marks");
  await assertDisclosureKeyboardAndPointer(page);
  await assertNoLeak(page, consoleMessages, "three_marks outcome");

  await playThreeMarksDraw(page, baseUrl);
  await assertOutcomePanel(page, "Three Marks draw");
  await assertHumanizedOutcomeCopy(page, "Three Marks draw");
  await assertDrawStandingParity(page, "Three Marks draw");
  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  await playRiverLedgerFoldout(page, baseUrl);
  await assertOutcomePanel(page, "River Ledger");
  await assertOutcomeStyled(page, "River Ledger");
  await assertHumanizedOutcomeCopy(page, "River Ledger");
  await assertRiverLedgerOutcome(page);
  await assertNoLeak(page, consoleMessages, "river_ledger outcome");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "outcome explanation panel a11y noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function playRaceToTerminal(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Race to 21");
  await clickText(page, "button", "Race to 21");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (let index = 0; index < 6; index += 1) {
    await clickText(page, "button", "Add 3");
  }
  await markPreTerminalStatusRegion(page);
  await clickText(page, "button", "Add 3");
  await page.waitForSelector(".outcome-explanation-panel");
}

async function playThreeMarksWin(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Three Marks");
  await clickText(page, "button", "Three Marks");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (const cell of ["r1c1", "r2c1", "r1c2", "r2c2"]) {
    await clickCell(page, cell);
  }
  await markPreTerminalStatusRegion(page);
  await clickCell(page, "r1c3");
  await page.waitForSelector(".outcome-explanation-panel");
}

async function playThreeMarksDraw(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Three Marks");
  await clickText(page, "button", "Three Marks");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  for (const cell of ["r1c1", "r1c2", "r1c3", "r2c1", "r2c3", "r2c2", "r3c1", "r3c3"]) {
    await clickCell(page, cell);
  }
  await markPreTerminalStatusRegion(page);
  await clickCell(page, "r3c2");
  await page.waitForSelector(".outcome-explanation-panel");
}

async function playRiverLedgerFoldout(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "River Ledger");
  await clickText(page, "button", "River Ledger");
  await page.waitForSelector('select[aria-label="Supported seats from Rust catalog"]');
  await page.select('select[aria-label="Supported seats from Rust catalog"]', "3");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await waitForText(page, "Available choices");
  await markPreTerminalStatusRegion(page);
  await clickText(page, "button", "Fold");
  await waitForText(page, "Available choices");
  await clickText(page, "button", "Fold");
  await page.waitForSelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
}

async function assertOutcomePanel(page, label) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector(".outcome-explanation-panel");
    const status = document.querySelector('[role="status"][data-outcome-preterminal="true"]');
    const headingId = panel?.getAttribute("aria-labelledby") ?? "";
    const heading = headingId ? document.getElementById(headingId)?.textContent ?? "" : "";
    const summaryText = panel?.querySelector(".outcome-summary p:last-child")?.textContent ?? "";
    return {
      exists: Boolean(panel),
      heading,
      panelStatusCount: panel?.querySelectorAll('[role="status"], [aria-live]').length ?? 0,
      statusText: status?.textContent ?? "",
      summaryText,
      standingRows: panel?.querySelectorAll(".outcome-standing-row").length ?? 0,
      disclosureButtons: panel?.querySelectorAll("button[aria-expanded][aria-controls]").length ?? 0,
      text: panel?.textContent ?? "",
    };
  });
  assert(summary.exists, `${label} renders shared outcome panel`);
  assert(summary.heading.length > 0, `${label} panel has associated heading`);
  assert(summary.statusText.includes(summary.heading), `${label} pre-existing status node announces heading`);
  assert(summary.statusText.includes(summary.summaryText), `${label} pre-existing status node announces decisive cause`);
  assert(summary.panelStatusCount === 0, `${label} panel does not mount its own live/status region`);
  assert(summary.standingRows >= 1, `${label} panel renders final standing`);
  assert(summary.disclosureButtons >= 1, `${label} panel renders disclosure control`);
  assert(summary.text.includes("Outcome"), `${label} panel includes outcome label`);
}

async function markPreTerminalStatusRegion(page) {
  await page.evaluate(() => {
    const status = document.querySelector('[role="status"]');
    if (!status) {
      throw new Error("Missing pre-terminal status region");
    }
    status.setAttribute("data-outcome-preterminal", "true");
  });
}

async function assertOutcomeStyled(page, label) {
  const styles = await page.evaluate(() => {
    const panel = document.querySelector(".outcome-explanation-panel");
    const heading = panel?.querySelector("h2");
    const standingRow = panel?.querySelector(".outcome-standing-row");
    const disclosureButton = panel?.querySelector(".outcome-breakdown-section button");
    if (!panel || !heading || !standingRow || !disclosureButton) {
      return null;
    }
    const panelStyle = window.getComputedStyle(panel);
    const headingStyle = window.getComputedStyle(heading);
    const standingStyle = window.getComputedStyle(standingRow);
    const buttonBefore = window.getComputedStyle(disclosureButton, "::before");
    return {
      borderTopWidth: panelStyle.borderTopWidth,
      backgroundColor: panelStyle.backgroundColor,
      headingFontSize: headingStyle.fontSize,
      standingFontSize: standingStyle.fontSize,
      disclosureBeforeContent: buttonBefore.content,
      disclosureWidth: window.getComputedStyle(disclosureButton).width,
    };
  });
  assert(Boolean(styles), `${label} has styled outcome elements`);
  assert(styles.borderTopWidth !== "0px", `${label} outcome panel has a card border`);
  assert(styles.backgroundColor !== "rgba(0, 0, 0, 0)", `${label} outcome panel has a non-transparent background`);
  assert(
    Number.parseFloat(styles.headingFontSize) > Number.parseFloat(styles.standingFontSize),
    `${label} outcome heading has a stronger type scale: ${JSON.stringify(styles)}`,
  );
  assert(
    styles.disclosureBeforeContent !== "none" || Number.parseFloat(styles.disclosureWidth) > 0,
    `${label} outcome disclosure has a visible affordance: ${JSON.stringify(styles)}`,
  );
}

async function assertDisclosureKeyboardAndPointer(page) {
  const button = await page.waitForSelector(".outcome-breakdown-section button");
  const initialExpanded = await page.$eval(".outcome-breakdown-section button", (element) =>
    element.getAttribute("aria-expanded"),
  );
  assert(initialExpanded === "true", "decisive outcome disclosure is expanded by default");
  await button.focus();
  const focused = await page.evaluate(() => document.activeElement?.matches(".outcome-breakdown-section button") ?? false);
  assert(focused, "disclosure button receives focus");
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => document.querySelector(".outcome-breakdown-section button")?.getAttribute("aria-expanded") === "false");
  await page.click(".outcome-breakdown-section button");
  await page.waitForFunction(() => document.querySelector(".outcome-breakdown-section button")?.getAttribute("aria-expanded") === "true");
}

async function panelText(page) {
  return page.$eval(".outcome-explanation-panel", (panel) => panel.textContent ?? "");
}

async function assertHumanizedOutcomeCopy(page, label) {
  const text = await panelText(page);
  const rawTokens = ["seat_0", "seat_1", "high_card", "r1c1", "r1c2", "r1c3", "win", "loss", "split"];
  const hits = rawTokens.filter((token) => new RegExp(`\\b${token}\\b`).test(text));
  assert(hits.length === 0, `${label} outcome panel exposes raw visible tokens: ${hits.join(", ")}`);
}

async function assertWinnerFirstStanding(page, label) {
  const summary = await page.evaluate(() => {
    const rows = Array.from(document.querySelectorAll(".outcome-standing-row"));
    return {
      rowCount: rows.length,
      firstEmphasized: rows[0]?.classList.contains("emphasized") ?? false,
    };
  });
  assert(summary.rowCount >= 2, `${label} has multiple standing rows`);
  assert(summary.firstEmphasized, `${label} winner row is first`);
}

async function assertDrawStandingParity(page, label) {
  const summary = await page.evaluate(() => {
    const rows = Array.from(document.querySelectorAll(".outcome-standing-row"));
    const emphasized = rows.filter((row) => row.classList.contains("emphasized")).length;
    const classes = rows.map((row) => row.className);
    return { rowCount: rows.length, emphasized, classes };
  });
  assert(summary.rowCount >= 2, `${label} has multiple standing rows`);
  assert(summary.emphasized === 0, `${label} draw has no emphasized standing row`);
  assert(new Set(summary.classes).size === 1, `${label} draw rows use matching treatment`);
}

async function assertRiverLedgerOutcome(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    return {
      text: panel?.textContent ?? "",
      standingRows: panel?.querySelectorAll(".outcome-standing-row").length ?? 0,
      rules: Array.from(panel?.querySelectorAll(".outcome-rule-refs code") ?? []).map((code) => code.textContent ?? ""),
    };
  });
  assert(
    summary.text.includes("last live seat receives the ledger"),
    `river_ledger uses foldout template: ${summary.text}`,
  );
  assert(summary.standingRows === 3, `river_ledger renders one standing row per seat: ${summary.standingRows}`);
  assert(summary.rules.includes("RL-END-LAST-LIVE"), `river_ledger exposes terminal rule id: ${summary.rules.join(", ")}`);
  assert(summary.rules.includes("RL-SCORE-POT-AWARD"), `river_ledger exposes scoring rule id: ${summary.rules.join(", ")}`);
}

async function catalogNames(page) {
  await page.waitForSelector(".game-card");
  return page.$$eval(".game-card", (cards) =>
    cards.map((card) => card.querySelector("h2,h3,strong")?.textContent?.trim() ?? card.textContent?.trim() ?? ""),
  );
}

async function setReducedMotion(page) {
  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".reduced-motion .outcome-explanation-panel");
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
  const storage = await page.evaluate(() => ({
    local: Object.values(localStorage).join("\n"),
    session: Object.values(sessionStorage).join("\n"),
  }));
  assertNoForbiddenTerms(`${storage.local}\n${storage.session}`, "storage");
}

async function clickCell(page, cell) {
  await page.click(`[data-testid="three-cell-${cell}"]`);
  await page.waitForFunction(
    (cellId) => document.querySelector(`[data-testid="three-cell-${cellId}"]`)?.className.includes("occupied"),
    {},
    cell,
  );
}

async function clickLabel(page, text) {
  await page.evaluate((labelText) => {
    const label = Array.from(document.querySelectorAll("label")).find((candidate) =>
      candidate.textContent?.includes(labelText),
    );
    if (!label) {
      throw new Error(`Missing label: ${labelText}`);
    }
    label.click();
  }, text);
}

async function clickText(page, selector, text) {
  await page.evaluate(
    ({ selector, text }) => {
      const element = Array.from(document.querySelectorAll(selector)).find((candidate) =>
        candidate.textContent?.includes(text),
      );
      if (!element) {
        throw new Error(`Missing ${selector} with text: ${text}`);
      }
      element.click();
    },
    { selector, text },
  );
}

async function waitForText(page, text) {
  await page.waitForFunction((expected) => document.body.textContent?.includes(expected), {}, text);
}

async function assertFocusedVisible(page) {
  const focusStyle = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) return null;
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

function assertNoForbiddenTerms(surface, label) {
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function contentTypeFor(filePath) {
  switch (extname(filePath)) {
    case ".js":
      return "text/javascript";
    case ".css":
      return "text/css";
    case ".wasm":
      return "application/wasm";
    case ".json":
      return "application/json";
    default:
      return "text/html";
  }
}
