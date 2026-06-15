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
  await page.setViewport({ width: 1280, height: 900 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "rulepath-wasm-api/0.1.0");
  await assertPlayFirst(page);
  await waitForText(page, "Choose a game");
  await waitForText(page, "Race to 21");
  await clickText(page, "button", "River Ledger");
  await assertSelectedGameCardFlags(page, "river_ledger");
  await clickText(page, "button", "Race to 21");
  await waitForText(page, "Match setup");

  await keyboardStart(page);
  await waitForSelectorText(page, '[data-testid="counter"]', "0 / 21");
  await waitForText(page, "Choose a Rust-supplied action");
  await clickText(page, "button", "Add 1");
  await page.waitForFunction(() => {
    const counter = document.querySelector('[data-testid="counter"]')?.textContent ?? "";
    return !counter.startsWith("0 /");
  });
  await waitForEffectCount(page, 1);

  await clickText(page, "button", "Developer panel");
  await waitForText(page, "Operations");
  await clickTestId(page, "stale-action");
  await waitForSelectorText(page, '[data-testid="diagnostic"]', "stale_action");

  await clickText(page, "button", "Export Current Run");
  await page.waitForFunction(() => document.querySelector("textarea")?.value.includes('"commands"'));
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Replay viewer");
  await waitForText(page, "Cursor 0 /");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 1 /");

  await page.reload({ waitUntil: "networkidle0" });
  await waitForText(page, "Race to 21");
  await clickLabel(page, "Bot vs bot");
  await clickText(page, "button", "Start Match");
  await waitForText(page, "Bot vs bot");
  await clickText(page, "button", "Step Bot");
  await page.waitForFunction(() => {
    const counter = document.querySelector('[data-testid="counter"]')?.textContent ?? "";
    return !counter.startsWith("0 /");
  });
  await clickText(page, "button", "Start Autoplay");
  await waitForText(page, "Pause");
  await clickText(page, "button", "Pause");

  await page.select(".motion-field select", "reduce");
  await page.waitForFunction(() => document.querySelector(".effect-entry.reduced"));

  console.log(JSON.stringify({ base: mountPath, browser: "puppeteer", flow: "picker setup choice bot replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function keyboardStart(page) {
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  const focusedText = await page.evaluate(() => document.activeElement?.textContent ?? "");
  if (!focusedText.includes("Start Match")) {
    await clickText(page, "button", "Start Match");
    return;
  }
  await page.keyboard.press("Enter");
}

async function assertPlayFirst(page) {
  const pageShape = await page.evaluate(() => {
    const bodyText = document.body.textContent?.trim() ?? "";
    const preCount = document.querySelectorAll("pre").length;
    const textareaCount = document.querySelectorAll("textarea").length;
    return { bodyText, preCount, textareaCount };
  });
  assert(!pageShape.bodyText.startsWith("{"), "normal page is not raw JSON");
  assert(pageShape.preCount === 0, "normal page is not debug-pre dominated");
  assert(pageShape.textareaCount === 0, "initial page is play-first, before replay tools");
}

async function assertSelectedGameCardFlags(page, gameId) {
  const result = await page.evaluate((gameId) => {
    const card = document.querySelector(`.game-card[data-game-id="${gameId}"]`);
    if (!card) return { ok: false, reason: `missing card ${gameId}` };
    const option = card.querySelector(".game-option");
    const mark = card.querySelector(".game-selected-mark");
    const flags = Array.from(card.querySelectorAll(".game-flags > span"));
    if (!option || !mark) return { ok: false, reason: "missing selected card elements" };
    if (flags.length === 0) return { ok: false, reason: "selected card has no flag pills" };

    const optionRect = option.getBoundingClientRect();
    const markRect = mark.getBoundingClientRect();
    for (const [index, flag] of flags.entries()) {
      const flagRect = flag.getBoundingClientRect();
      const intersects =
        markRect.left < flagRect.right &&
        markRect.right > flagRect.left &&
        markRect.top < flagRect.bottom &&
        markRect.bottom > flagRect.top;
      if (intersects) {
        return { ok: false, reason: `selected badge overlaps flag ${index}` };
      }
      const contained =
        flagRect.left >= optionRect.left &&
        flagRect.right <= optionRect.right &&
        flagRect.top >= optionRect.top &&
        flagRect.bottom <= optionRect.bottom;
      if (!contained) {
        return { ok: false, reason: `flag ${index} is clipped outside card` };
      }
    }
    return { ok: true, reason: "" };
  }, gameId);
  assert(result.ok, result.reason);
}

async function clickTestId(page, testId) {
  await page.click(`[data-testid="${testId}"]`);
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

async function waitForSelectorText(page, selector, text) {
  await page.waitForFunction(
    ({ selector, text }) => document.querySelector(selector)?.textContent?.includes(text),
    {},
    { selector, text },
  );
}

async function waitForEffectCount(page, minimum) {
  await page.waitForFunction(
    (minimum) => document.querySelectorAll('[data-testid="effects"] li').length >= minimum,
    {},
    minimum,
  );
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
