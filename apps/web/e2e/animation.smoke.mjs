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
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "no-preference" }]);

  await startGame(page, baseUrl, "Event Frontier", "Hotseat", 3);
  await installAnimationProbe(page, "__animationSmokeTargets");
  await clickText(page, "button", "Event");
  await waitForText(page, "Ready");
  await clickText(page, "button", "Confirm");
  await waitForText(page, "Edict activated");
  await assertAnimationTargets(page, "__animationSmokeTargets", ["event-frontier-deck"], "event frontier action");
  await assertNoRunningAnimations(page);

  await clickText(page, "button", "Pass");
  await waitForText(page, "Ready");
  await clickText(page, "button", "Confirm");
  await waitForText(page, "Reckoning resolved");
  await assertAnimationTargetPrefix(page, "__animationSmokeTargets", "event-frontier-site-", "event frontier reckoning");
  await assertNoRunningAnimations(page);

  await clickText(page, "button", "Export Current Run");
  await page.waitForFunction(() => document.querySelector("textarea")?.value.includes('"game_id": "event_frontier"'));
  await clickText(page, "button", "Import Replay");
  await waitForText(page, "Replay viewer");
  await clickText(page, "button", "Step");
  await waitForText(page, "Cursor 0 /");

  await startGame(page, baseUrl, "Flood Watch", "Human vs bot", 41);
  const before = await currentFreshness(page);
  await clickText(page, "button", "End turn");
  await clickText(page, "button", "Skip");
  await waitForFreshnessGreaterThan(page, before);
  await page.waitForFunction(() =>
    ["Levee placed", "District bailed", "Forecast revealed", "Storm card drawn"].some((text) =>
      document.body.textContent?.includes(text),
    ),
  );

  await startGame(page, baseUrl, "Flood Watch", "Hotseat", 41);
  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".flood-watch-board.reduced");
  await installAnimationProbe(page, "__animationSmokeReducedTargets");
  await clickText(page, "button", "Forecast");
  await waitForText(page, "Forecast revealed");
  await waitForText(page, "The next public storm card");
  await assertAnimationTargets(page, "__animationSmokeReducedTargets", ["flood-watch-deck"], "reduced motion forecast");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "animation settle skip replay reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startGame(page, baseUrl, gameLabel, modeLabel, seed) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, gameLabel);
  await clickText(page, "button", gameLabel);
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
}

async function installAnimationProbe(page, key) {
  await page.evaluate((targetKey) => {
    window[targetKey] = [];
    if (window.__animationSmokeProbeInstalled) {
      return;
    }
    window.__animationSmokeProbeInstalled = true;
    const originalAnimate = Element.prototype.animate;
    Element.prototype.animate = function patchedAnimate(...args) {
      for (const probeKey of ["__animationSmokeTargets", "__animationSmokeReducedTargets"]) {
        if (Array.isArray(window[probeKey])) {
          const target = this.closest("[data-animation-target]")?.getAttribute("data-animation-target");
          if (target) {
            window[probeKey].push(target);
          }
        }
      }
      return originalAnimate.apply(this, args);
    };
  }, key);
}

async function assertAnimationTargets(page, key, targets, label) {
  await page.waitForFunction(
    (probeKey, expectedTargets) => {
      const seen = window[probeKey] ?? [];
      return expectedTargets.every((target) => seen.includes(target));
    },
    {},
    key,
    targets,
  );
  const seen = await page.evaluate((probeKey) => window[probeKey] ?? [], key);
  for (const target of targets) {
    assert(seen.includes(target), `${label} animated ${target}`);
  }
}

async function assertAnimationTargetPrefix(page, key, prefix, label) {
  await page.waitForFunction(
    (probeKey, expectedPrefix) => (window[probeKey] ?? []).some((target) => target.startsWith(expectedPrefix)),
    {},
    key,
    prefix,
  );
  const seen = await page.evaluate((probeKey) => window[probeKey] ?? [], key);
  assert(seen.some((target) => target.startsWith(prefix)), `${label} animated target with prefix ${prefix}`);
}

async function assertNoRunningAnimations(page) {
  await page.waitForFunction(() => document.getAnimations().every((animation) => animation.playState !== "running"));
  const ghosts = await page.$$eval(".animation-ghost", (elements) => elements.length);
  assert(ghosts === 0, "animation smoke leaves no ghost overlays after settle");
}

async function currentFreshness(page) {
  return page.evaluate(() => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return state?.view?.freshness_token ?? 0;
  });
}

async function waitForFreshnessGreaterThan(page, freshnessToken) {
  await page.waitForFunction(
    (minimum) => {
      if (!window.render_game_to_text) return false;
      const state = JSON.parse(window.render_game_to_text());
      return (state?.view?.freshness_token ?? 0) > minimum;
    },
    {},
    freshnessToken,
  );
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

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
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
