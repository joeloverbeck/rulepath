import { createReadStream } from "node:fs";
import { access, readFile } from "node:fs/promises";
import http from "node:http";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { launchBrowser } from "./launch.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const distDir = join(__dirname, "..", "dist");
const wasmPath = join(__dirname, "..", "public", "wasm_api.wasm");
const mountPath = "/rulepath/";
const executablePath = process.env.PUPPETEER_EXECUTABLE_PATH || "/usr/bin/google-chrome";

await access(executablePath);

const catalog = await loadCatalog();
const cases = [
  { gameId: "race_to_n", playMode: "Hotseat", kind: "race" },
  { gameId: "briar_circuit", playMode: "Bot vs bot", kind: "briar" },
  { gameId: "river_ledger", playMode: "Bot vs bot", kind: "river" },
];

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
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);

  for (const testCase of cases) {
    const game = catalogGame(testCase.gameId);
    const catalogLabels = seatLabelsFor(game);
    await startGame(page, baseUrl, game.display_name, testCase.playMode);
    await assertSeatFrameMatchesCatalog(page, testCase.gameId, catalogLabels);
    if (testCase.kind === "race") {
      await assertRacePlayAreaMatchesCatalog(page, catalogLabels);
    } else if (testCase.kind === "briar") {
      await assertBoardLabels(page, ".briar-seat > span", testCase.gameId, catalogLabels);
    } else {
      await assertBoardLabels(page, ".river-ledger-seat .river-ledger-section-heading span", testCase.gameId, catalogLabels);
    }
  }

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "seat label consistency catalog viewer play-area" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startGame(page, baseUrl, gameLabel, playMode) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, gameLabel);
  await clickText(page, "button", gameLabel);
  await clickLabel(page, playMode);
  await clickText(page, "button", "Start Match");
  await page.waitForSelector(".seat-frame-viewers");
}

async function assertSeatFrameMatchesCatalog(page, gameId, catalogLabels) {
  const actual = await page.$$eval(".seat-frame-viewers label", (labels) =>
    labels
      .map((label) => label.textContent?.trim() ?? "")
      .filter((label) => label !== "Observer"),
  );
  assertDeepEqual(actual, catalogLabels.map((entry) => entry.label), `${gameId} viewer labels equal Rust catalog labels`);
  const rail = await page.$$eval(".seat-frame-rail li", (items) =>
    items.map((item) => ({
      seat: item.getAttribute("data-seat") ?? "",
      label: item.querySelector("span")?.textContent?.trim() ?? "",
    })),
  );
  assertDeepEqual(rail, catalogLabels, `${gameId} seat rail labels equal Rust catalog labels`);
}

async function assertRacePlayAreaMatchesCatalog(page, catalogLabels) {
  await waitForText(page, `${catalogLabels[0].label} to move`);
  await clickText(page, "button", "Add 1");
  await waitForText(page, `${catalogLabels[1].label} to move`);
}

async function assertBoardLabels(page, selector, gameId, catalogLabels) {
  const labels = await page.$$eval(selector, (items) => items.map((item) => item.textContent?.trim() ?? ""));
  assertDeepEqual(labels, catalogLabels.map((entry) => entry.label), `${gameId} play-area labels equal Rust catalog labels`);
}

function catalogGame(gameId) {
  const game = catalog.find((entry) => entry.game_id === gameId);
  assert(game, `catalog includes ${gameId}`);
  return game;
}

function seatLabelsFor(game) {
  assert(Array.isArray(game.seat_labels) && game.seat_labels.length > 0, `${game.game_id} exposes catalog seat labels`);
  return game.seat_labels.map((entry) => ({ seat: entry.seat, label: entry.label }));
}

async function loadCatalog() {
  const bytes = await readFile(wasmPath);
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const wasm = instance.exports;
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  function read(ptr, len) {
    return decoder.decode(new Uint8Array(wasm.memory.buffer, ptr, len));
  }

  function write(value) {
    const bytes = encoder.encode(value);
    const ptr = wasm.rulepath_alloc(bytes.length);
    new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
    return { ptr, len: bytes.length };
  }

  function output() {
    return read(wasm.rulepath_last_output_ptr(), wasm.rulepath_last_output_len());
  }

  function invoke(call, values = []) {
    const args = values.map(write);
    try {
      const status = call(args);
      const parsed = JSON.parse(output());
      if (status !== 0) {
        throw new Error(parsed.message);
      }
      return parsed;
    } finally {
      for (const arg of args) {
        wasm.rulepath_dealloc(arg.ptr, arg.len);
      }
    }
  }

  return invoke(() => wasm.rulepath_list_games());
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
  assert(clicked, `${selector} contains ${text}`);
}

async function waitForText(page, text) {
  await page.waitForFunction(
    (expected) => document.body?.textContent?.includes(expected),
    {},
    text,
  );
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertDeepEqual(actual, expected, message) {
  const actualJson = JSON.stringify(actual);
  const expectedJson = JSON.stringify(expected);
  if (actualJson !== expectedJson) {
    throw new Error(`${message}: expected ${expectedJson}, got ${actualJson}`);
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
    case ".json":
      return "application/json";
    default:
      return "application/octet-stream";
  }
}
