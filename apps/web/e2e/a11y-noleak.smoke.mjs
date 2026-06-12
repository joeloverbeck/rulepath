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
  "hole_card",
  "candidate_ranking",
  "bot_explanation",
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
  await page.setViewport({ width: 390, height: 820 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Race to 21");
  await assertNamedControls(page);
  await assertNoLeak(page, consoleMessages, "initial DOM");

  await focusByText(page, "Start Match");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Choose a Rust-supplied action");

  await focusByText(page, "Add 1");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await page.waitForFunction(() => {
    const counter = document.querySelector('[data-testid="counter"]')?.textContent ?? "";
    return !counter.startsWith("0 /");
  });
  await assertNonColorCues(page);

  await focusByText(page, "Developer panel");
  await assertFocusedVisible(page);
  await page.keyboard.press("Enter");
  await waitForText(page, "Operations");
  await focusByText(page, "Submit Stale Action");
  await page.keyboard.press("Enter");
  await waitForText(page, "stale_action");

  await focusByText(page, "Export Current Run");
  await page.keyboard.press("Enter");
  const replayText = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  const replayExport = await replayText.jsonValue();
  assertNoForbiddenTerms(replayExport, "replay export");
  assert(
    replayExport.includes('"expected_private_view_hashes"') && replayExport.includes('"not_applicable"'),
    "replay export marks private views explicitly not applicable for this perfect-information game",
  );
  await focusByText(page, "Import Replay");
  await page.keyboard.press("Enter");
  await waitForText(page, "Replay viewer");
  await focusByText(page, "Step");
  await page.keyboard.press("Enter");
  await waitForText(page, "Cursor 1 /");

  await page.select(".motion-field select", "reduce");
  await page.waitForFunction(() => document.querySelector(".effect-entry.reduced"));
  await assertReducedMotion(page);
  await assertStorageClean(page);
  await assertNoLeak(page, consoleMessages, "played DOM");
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startDirectionalFlip(page, "Hotseat");
  await assertDirectionalBoardA11y(page);
  await page.select(".motion-field select", "reduce");
  await keyboardPlaceDirectional(page);
  await page.waitForSelector(".directional-flip-board.reduced .directional-cell.flipped .directional-disc");
  const animationName = await page.$eval(".directional-flip-board.reduced .directional-cell.flipped .directional-disc", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "directional_flip reduced-motion suppresses flip animation");
  await assertNoLeak(page, consoleMessages, "directional_flip DOM");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startEventFrontier(page);
  await assertEventFrontierBoardA11y(page);
  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".event-frontier-board.reduced");
  const eventAnimationName = await page.$eval(".event-frontier-board.reduced .frontier-site", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(eventAnimationName === "none", "event_frontier reduced-motion suppresses site animation");
  await assertNoLeak(page, consoleMessages, "event_frontier DOM");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "a11y noleak keyboard reduced directional_flip event_frontier" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startDirectionalFlip(page, mode = "Human vs bot") {
  await clickText(page, "button", "Directional Flip");
  if (mode !== "Human vs bot") {
    await clickLabel(page, mode);
  }
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="directional-flip-board"]');
}

async function startEventFrontier(page) {
  await clickText(page, "button", "Event Frontier");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="event-frontier-board"]');
}

async function assertEventFrontierBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      sites: document.querySelectorAll(".event-frontier-board .frontier-site").length,
      trails: document.querySelectorAll(".event-frontier-board .frontier-trail").length,
      missingNames: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      redaction: document.body.textContent?.includes("Hidden order") ?? false,
      latest: document.querySelector(".event-frontier-board .plain-latest")?.textContent ?? "",
    };
  });
  assert(summary.sites === 6, `event_frontier renders six sites, got ${summary.sites}`);
  assert(summary.trails >= 6, `event_frontier renders public trail lines, got ${summary.trails}`);
  assert(summary.missingNames.length === 0, `event_frontier buttons have accessible names: ${summary.missingNames.join(", ")}`);
  assert(summary.redaction, "event_frontier renders explicit hidden-order redaction");
  assert(summary.latest.length > 0, "event_frontier latest-effect region has text");
}

async function assertDirectionalBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const cells = Array.from(document.querySelectorAll('[data-testid^="directional-cell-"]'));
    return {
      cells: cells.length,
      legal: cells.filter((cell) => cell.classList.contains("legal")).length,
      missingNames: cells
        .filter((cell) => !(cell.getAttribute("aria-label") ?? "").trim())
        .map((cell) => cell.getAttribute("data-testid")),
      seat0Marks: document.querySelectorAll(".directional-disc-seat-0 .directional-disc-mark").length,
      seat1Marks: document.querySelectorAll(".directional-disc-seat-1 .directional-disc-mark").length,
      status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
    };
  });
  assert(summary.cells === 64, `directional_flip renders sixty-four cells, got ${summary.cells}`);
  assert(summary.legal === 4, `directional_flip exposes four Rust legal targets, got ${summary.legal}`);
  assert(summary.missingNames.length === 0, `directional_flip cells have accessible names: ${summary.missingNames.join(", ")}`);
  assert(summary.seat0Marks > 0 && summary.seat1Marks > 0, "directional_flip seats use non-color SVG marks");
  assert(summary.status.length > 0, "directional_flip has text turn status");
}

async function keyboardPlaceDirectional(page) {
  await focusByTestId(page, "directional-cell-r1c1");
  await assertFocusedVisible(page);
  for (const key of ["ArrowDown", "ArrowDown", "ArrowRight", "ArrowRight", "ArrowRight", "Enter"]) {
    await page.keyboard.press(key);
  }
  await page.waitForFunction(() => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token >= 1;
  });
}

async function assertNamedControls(page) {
  const unnamedControls = await page.evaluate(() =>
    Array.from(document.querySelectorAll("button, input, select, textarea"))
      .filter((element) => {
        const label = element.labels?.[0]?.textContent ?? "";
        const text = element.textContent ?? "";
        const aria = element.getAttribute("aria-label") ?? "";
        const title = element.getAttribute("title") ?? "";
        return `${label}${text}${aria}${title}`.trim().length === 0;
      })
      .map((element) => element.outerHTML),
  );
  assert(unnamedControls.length === 0, `all controls have accessible names: ${unnamedControls.join("\n")}`);
}

async function assertFocusedVisible(page) {
  const focusStyle = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element) {
      return null;
    }
    const style = window.getComputedStyle(element);
    const wrapperStyle = element.closest("label") ? window.getComputedStyle(element.closest("label")) : null;
    return {
      tag: element.tagName,
      text: element.textContent,
      outlineWidth: style.outlineWidth,
      outlineStyle: style.outlineStyle,
      wrapperOutlineWidth: wrapperStyle?.outlineWidth ?? "0px",
      wrapperOutlineStyle: wrapperStyle?.outlineStyle ?? "none",
    };
  });
  assert(Boolean(focusStyle), "focus target exists");
  assert(
    focusStyle.outlineStyle !== "none" ||
      focusStyle.outlineWidth !== "0px" ||
      focusStyle.wrapperOutlineStyle !== "none" ||
      focusStyle.wrapperOutlineWidth !== "0px",
    `focused control has visible focus: ${JSON.stringify(focusStyle)}`,
  );
}

async function assertNonColorCues(page) {
  const cues = await page.evaluate(() => ({
    status: document.querySelector('[data-testid="turn"]')?.textContent ?? "",
    effectTitles: Array.from(document.querySelectorAll('[data-testid="effects"] strong')).map((element) =>
      element.textContent?.trim(),
    ),
    diagnostic: document.querySelector('[data-testid="diagnostic"]')?.textContent ?? "",
  }));
  assert(cues.status.length > 0, "turn status has text cue");
  assert(cues.effectTitles.some(Boolean), "effects include text labels, not only color");
}

async function assertReducedMotion(page) {
  const reduced = await page.evaluate(() => {
    const shellReduced = document.querySelector(".reduced-motion") !== null;
    const animatedEntries = Array.from(document.querySelectorAll(".effect-entry")).map((element) => {
      const style = window.getComputedStyle(element);
      return { animation: style.animationName, transition: style.transitionProperty };
    });
    return { shellReduced, animatedEntries };
  });
  assert(reduced.shellReduced, "reduced-motion class is present");
  assert(
    reduced.animatedEntries.every((entry) => entry.animation === "none"),
    "effect entries disable animation under reduced motion",
  );
}

async function assertStorageClean(page) {
  const storage = await page.evaluate(() => ({
    local: Object.fromEntries(Array.from({ length: localStorage.length }, (_, index) => {
      const key = localStorage.key(index);
      return [key, key ? localStorage.getItem(key) : null];
    })),
    session: Object.fromEntries(Array.from({ length: sessionStorage.length }, (_, index) => {
      const key = sessionStorage.key(index);
      return [key, key ? sessionStorage.getItem(key) : null];
    })),
  }));
  const localEntries = Object.entries(storage.local);
  assert(
    localEntries.every(([key, value]) => key === "rulepath.reducedMotion" && (value === "reduce" || value === "motion")),
    `localStorage contains only the reduced-motion UI preference: ${JSON.stringify(storage.local)}`,
  );
  assert(Object.keys(storage.session).length === 0, "sessionStorage remains empty");
}

async function assertNoLeak(page, consoleMessages, label) {
  const surface = await page.evaluate(() => {
    const attributes = Array.from(document.querySelectorAll("*")).flatMap((element) =>
      Array.from(element.attributes).map((attribute) => `${attribute.name}=${attribute.value}`),
    );
    const testIds = Array.from(document.querySelectorAll("[data-testid]")).map((element) =>
      element.getAttribute("data-testid"),
    );
    const devPanel = document.querySelector(".dev-panel")?.textContent ?? "";
    return [document.body.textContent ?? "", attributes.join("\n"), testIds.join("\n"), devPanel].join("\n");
  });
  assertNoForbiddenTerms(surface, label);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`);
}

function assertNoForbiddenTerms(surface, label) {
  const lower = surface.toLowerCase();
  const hits = forbiddenLeakTerms.filter((term) => lower.includes(term));
  assert(hits.length === 0, `${label} contains forbidden leak terms: ${hits.join(", ")}`);
}

async function focusByText(page, text) {
  for (let index = 0; index < 40; index += 1) {
    const focused = await page.evaluate((expected) => {
      const element = document.activeElement;
      const focusable = element?.matches("button, input, select, textarea, a[href]") ?? false;
      return {
        text: element?.textContent ?? "",
        label: element?.labels?.[0]?.textContent ?? "",
        disabled: element?.disabled ?? false,
        matches: focusable && `${element?.textContent ?? ""} ${element?.labels?.[0]?.textContent ?? ""}`.includes(expected),
      };
    }, text);
    if (focused.matches && !focused.disabled) {
      return;
    }
    await page.keyboard.press("Tab");
  }
  throw new Error(`Unable to focus control by text: ${text}`);
}

async function focusByTestId(page, testId) {
  for (let index = 0; index < 100; index += 1) {
    const focused = await page.evaluate(
      (expected) => document.activeElement?.getAttribute("data-testid") === expected,
      testId,
    );
    if (focused) {
      return;
    }
    await page.keyboard.press("Tab");
  }
  throw new Error(`Unable to focus control by test id: ${testId}`);
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

function contentTypeFor(path) {
  switch (extname(path)) {
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

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
