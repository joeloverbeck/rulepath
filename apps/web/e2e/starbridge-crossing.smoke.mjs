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
const forbiddenTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "candidate_ranking",
  "bot_candidate",
  "deck_order",
  "stock_order",
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

  await assertStarbridgeSetupPreviewSeatLabels(page, baseUrl);

  await startStarbridge(page, baseUrl, "Hotseat", 20, 6);
  await assertBoardSurface(page, 6);
  await assertRenderedTextView(page, (view) => view?.game_id === "starbridge_crossing" && view.status?.includes("ply 0"));
  await assertNoLeak(page, consoleMessages, "initial starbridge board");

  await selectPegWithTarget(page, "step");
  await waitForText(page, "Selected path");
  await assertLegalTargets(page);
  await clickFirstLegalSpace(page, { emptyOnly: true, stepOnly: true });
  await waitForText(page, "Step moved");
  await assertRenderedTextView(page, (view) => view?.game_id === "starbridge_crossing" && view.freshness_token >= 1);

  await exerciseKeyboardPath(page);
  await waitForText(page, "Step moved");

  await findAndPlayJump(page, 80);
  await waitForText(page, "Jump chain");
  await assertNoLeak(page, consoleMessages, "after starbridge jump chain");

  await startStarbridge(page, baseUrl, "Hotseat", 22, 2);
  await selectPegWithTarget(page, "step");
  await clickFirstLegalSpace(page, { emptyOnly: true, stepOnly: true });
  await waitForText(page, "Step moved");
  await clickText(page, "button", "Export Current Run");
  const replayText = await replayTextareaValue(page);
  assert(replayText.includes('"game_id":"starbridge_crossing"') || replayText.includes('"game_id": "starbridge_crossing"'), "export keeps starbridge_crossing game id");
  assertNoForbiddenTerms(replayText, "starbridge public replay export", forbiddenTerms);
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Replay viewer");
  await waitForText(page, "Cursor 0 /");
  await waitForText(page, "Starbridge Crossing");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".starbridge-board.reduced");
  const reducedDash = await page.$eval(".starbridge-board.reduced .starbridge-legal-ring", (element) =>
    window.getComputedStyle(element).strokeDasharray,
  );
  assert(reducedDash === "none", "reduced motion removes animated-style dashed legal rings");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector('[data-testid="starbridge-board"]');
  const stageMetrics = await page.$eval('[data-testid="starbridge-board"]', (element) => ({
    width: element.getBoundingClientRect().width,
    scrollWidth: element.scrollWidth,
  }));
  assert(stageMetrics.width > 0 && stageMetrics.scrollWidth >= stageMetrics.width, "responsive starbridge board remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "starbridge_crossing board jump replay noleak responsive" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startStarbridge(page, baseUrl, modeLabel, seed, seatCount) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Starbridge Crossing");
  await clickText(page, "button", "Starbridge Crossing");
  await page.select('select[aria-label="Supported seats from Rust catalog"]', String(seatCount));
  await setSetupSeed(page, seed);
  await clickLabel(page, modeLabel);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="starbridge-board"]');
}

async function assertStarbridgeSetupPreviewSeatLabels(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Starbridge Crossing");
  await clickText(page, "button", "Starbridge Crossing");

  const cases = [
    { seatCount: 2, expected: ["North", "South"], absent: ["North East"] },
    { seatCount: 3, expected: ["North", "South East", "South West"], absent: ["North East"] },
    { seatCount: 4, expected: ["North", "North East", "South", "South West"], absent: ["South East"] },
  ];

  for (const testCase of cases) {
    await page.select('select[aria-label="Supported seats from Rust catalog"]', String(testCase.seatCount));
    const labels = await page.$$eval(".players-roles .seat-roles div span", (nodes) =>
      nodes.map((node) => node.textContent?.trim() ?? ""),
    );
    assertDeepEqual(labels, testCase.expected, `${testCase.seatCount}-seat Starbridge setup preview uses Rust active seats`);
    for (const label of testCase.absent) {
      assert(!labels.includes(label), `${testCase.seatCount}-seat Starbridge setup preview does not include ${label}`);
    }
  }
}

async function assertBoardSurface(page, seatCount) {
  const summary = await page.evaluate(() => {
    const spaces = Array.from(document.querySelectorAll("[data-starbridge-space]"));
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="starbridge-board"]')),
      spaces: spaces.length,
      legal: document.querySelectorAll(".starbridge-space.legal").length,
      pegs: document.querySelectorAll(".starbridge-peg").length,
      legend: document.querySelectorAll(".starbridge-legend > div").length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
    };
  });
  assert(summary.board, "starbridge board renders");
  assert(summary.spaces === 121, `starbridge renders 121 spaces, got ${summary.spaces}`);
  assert(summary.legal > 0, "starbridge exposes Rust-supplied legal board affordances");
  assert(summary.pegs === seatCount * 10, `starbridge renders ${seatCount * 10} public pegs, got ${summary.pegs}`);
  assert(summary.legend === seatCount, `starbridge renders ${seatCount} seat legend rows, got ${summary.legend}`);
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
}

async function assertLegalTargets(page) {
  const summary = await page.evaluate(() => ({
    legal: document.querySelectorAll(".starbridge-space.legal").length,
    selectedPath: document.querySelector(".starbridge-path")?.textContent ?? "",
  }));
  assert(summary.legal > 0, "selected starbridge peg exposes Rust legal targets");
  assert(/move >/.test(summary.selectedPath), `selected path updates from Rust action tree, got ${summary.selectedPath}`);
}

async function exerciseKeyboardPath(page) {
  await clearPath(page);
  await selectPegWithTarget(page, "step", { keyboard: true });
  await page.waitForFunction(() => (document.querySelector(".starbridge-path")?.textContent ?? "").includes("move >"));
  const target = await page.$(".starbridge-space.legal .starbridge-legal-ring.step");
  assert(target, "keyboard smoke finds a legal step target");
  await target.evaluate((ring) => ring.closest(".starbridge-space")?.focus());
  await page.keyboard.press("Enter");
}

async function selectPegWithTarget(page, targetKind, options = {}) {
  const legalSpaces = await page.$$eval(".starbridge-space.legal", (nodes) =>
    nodes.map((node) => node.getAttribute("data-starbridge-space")).filter(Boolean),
  );
  for (const space of legalSpaces) {
    if (options.keyboard) {
      await page.$eval(`[data-starbridge-space="${space}"]`, (element) => element.focus());
      await page.keyboard.press("Enter");
    } else {
      await clickSpace(page, space);
    }
    const hasTarget = await page.$(`.starbridge-space.legal .starbridge-legal-ring.${targetKind}`);
    if (hasTarget) {
      return true;
    }
    await clearPath(page);
  }
  throw new Error(`starbridge smoke did not find a peg with ${targetKind} target`);
}

async function findAndPlayJump(page, maxTurns) {
  for (let turn = 0; turn < maxTurns; turn += 1) {
    await clearPath(page);
    const found = await selectPegWithJump(page);
    if (found) {
      await clickFirstLegalSpace(page, { jumpOnly: true });
      await page.waitForFunction(() => Array.from(document.querySelectorAll("button")).some((button) => /Stop/.test(button.textContent ?? "")));
      const continueAvailable = await page.evaluate(() =>
        Array.from(document.querySelectorAll("button")).some((button) => /Continue/.test(button.textContent ?? "") && !button.disabled),
      );
      if (continueAvailable) {
        await clickText(page, "button", "Continue");
        await clickFirstLegalSpace(page, { jumpOnly: true });
        await page.waitForFunction(() => Array.from(document.querySelectorAll("button")).some((button) => /Stop/.test(button.textContent ?? "")));
      }
      await clickText(page, "button", "Stop");
      return;
    }
    await selectPegWithTarget(page, "step");
    await clickFirstLegalSpace(page, { emptyOnly: true, stepOnly: true });
    await page.waitForFunction(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()).view.freshness_token : 0) > 0);
  }
  throw new Error(`starbridge smoke did not find a Rust jump path within ${maxTurns} turns`);
}

async function selectPegWithJump(page) {
  const legalSpaces = await page.$$eval(".starbridge-space.legal", (nodes) =>
    nodes.map((node) => node.getAttribute("data-starbridge-space")).filter(Boolean),
  );
  for (const space of legalSpaces) {
    await clickSpace(page, space);
    const hasJump = await page.$(".starbridge-space.legal .starbridge-legal-ring.jump");
    if (hasJump) {
      return true;
    }
    await clearPath(page);
  }
  return false;
}

async function clearPath(page) {
  const clear = await page.evaluate(() => {
    const button = Array.from(document.querySelectorAll("button")).find((candidate) => candidate.textContent?.trim() === "Clear");
    if (!button || button.disabled) {
      return false;
    }
    button.click();
    return true;
  });
  if (clear) {
    await page.waitForFunction(() => !(document.querySelector(".starbridge-path")?.textContent ?? "").includes(" > "));
  }
}

async function clickFirstLegalSpace(page, options = {}) {
  const space = await page.evaluate((opts) => {
    const nodes = Array.from(document.querySelectorAll(".starbridge-space.legal"));
    const match = nodes.find((node) => {
      if (opts.emptyOnly && node.querySelector(".starbridge-peg")) {
        return false;
      }
      if (opts.stepOnly && !node.querySelector(".starbridge-legal-ring.step")) {
        return false;
      }
      if (opts.jumpOnly && !node.querySelector(".starbridge-legal-ring.jump")) {
        return false;
      }
      return true;
    });
    return match?.getAttribute("data-starbridge-space") ?? null;
  }, options);
  assert(space, `found legal starbridge space for ${JSON.stringify(options)}`);
  await clickSpace(page, space);
}

async function clickSpace(page, space) {
  await page.evaluate((spaceId) => {
    const element = document.querySelector(`[data-starbridge-space="${spaceId}"]`);
    element?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
  }, space);
}

async function assertNoLeak(page, consoleMessages, label) {
  assertNoForbiddenTerms(await fullBrowserSurface(page), label, forbiddenTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbiddenTerms);
}

async function fullBrowserSurface(page) {
  return page.evaluate(() => {
    const attrs = Array.from(document.querySelectorAll("*")).flatMap((node) =>
      Array.from(node.attributes ?? []).map((attr) => `${attr.name}=${attr.value}`),
    );
    return `${document.body.textContent ?? ""}\n${attrs.join("\n")}`;
  });
}

function assertNoForbiddenTerms(text, label, terms) {
  const normalized = text.toLowerCase();
  const found = terms.filter((term) => normalized.includes(term.toLowerCase()));
  assert(found.length === 0, `${label} leaked forbidden terms: ${found.join(", ")}`);
}

async function replayTextareaValue(page) {
  const handle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  return handle.jsonValue();
}

async function assertRenderedTextView(page, predicate) {
  await page.waitForFunction(
    (predicateSource) => {
      const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
      return state?.view && Function("view", `return (${predicateSource})(view);`)(state.view);
    },
    {},
    predicate.toString(),
  );
}

async function assertStorageClean(page) {
  const keys = await page.evaluate(() => Object.keys(window.localStorage).filter((key) => /hidden|private|deck|stock|candidate/i.test(key)));
  assert(keys.length === 0, `localStorage contains forbidden keys: ${keys.join(", ")}`);
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

async function waitForText(page, text) {
  await page.waitForFunction((needle) => document.body.textContent?.includes(needle), {}, text);
}

async function clickLabel(page, text) {
  const clicked = await page.evaluate((labelText) => {
    const label = Array.from(document.querySelectorAll("label")).find((candidate) => candidate.textContent?.includes(labelText));
    if (!label) {
      return false;
    }
    label.click();
    return true;
  }, text);
  assert(clicked, `clicked label ${text}`);
}

async function clickText(page, selector, text) {
  const clicked = await page.evaluate(
    ({ selector: targetSelector, text: targetText }) => {
      const element = Array.from(document.querySelectorAll(targetSelector)).find((candidate) =>
        candidate.textContent?.includes(targetText),
      );
      if (!element || element.disabled) {
        return false;
      }
      element.click();
      return true;
    },
    { selector, text },
  );
  assert(clicked, `clicked ${selector} containing ${text}`);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertDeepEqual(actual, expected, message) {
  const actualJson = JSON.stringify(actual);
  const expectedJson = JSON.stringify(expected);
  assert(actualJson === expectedJson, `${message}: expected ${expectedJson}, got ${actualJson}`);
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
    case ".md":
      return "text/markdown; charset=utf-8";
    default:
      return "text/html";
  }
}
