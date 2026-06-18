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
const forbiddenLeakTerms = [
  "hidden_state",
  "private_state",
  "internal_state",
  "hole_card",
  "candidate_ranking",
  "bot_explanation",
  "hcd:r",
];
const rawIdentifierRe = /\b[a-z0-9]+_[a-z0-9_]+\b/;
const rawSeatIdRe = /\bseat_\d+\b/;

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
  await page.setViewport({ width: 390, height: 820 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await page.goto(baseUrl, { waitUntil: "networkidle0" });

  await waitForText(page, "Race to 21");
  await assertNamedControls(page);
  await assertRawIdentifierGuardTrips(page);
  await assertRawSeatIdGuardTrips(page);
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
  await startHighCardDuel(page);
  await assertSeatFrameViewerNoLeak(page, consoleMessages);

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
  await startFloodWatch(page);
  await assertFixedTwoSeatSeatFrameNoLeak(page, consoleMessages, "flood_watch");
  await assertFloodWatchDeckA11y(page);
  await assertNoLeak(page, consoleMessages, "flood_watch DOM");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startEventFrontier(page);
  await assertFixedTwoSeatSeatFrameNoLeak(page, consoleMessages, "event_frontier");
  await assertEventFrontierBoardA11y(page);
  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".event-frontier-board.reduced");
  const eventAnimationName = await page.$eval(".event-frontier-board.reduced .frontier-site", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(eventAnimationName === "none", "event_frontier reduced-motion suppresses site animation");
  await assertNoLeak(page, consoleMessages, "event_frontier DOM");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startRiverLedgerBotWhy(page);
  await assertRiverLedgerBotWhy(page);
  await assertNoLeak(page, consoleMessages, "river_ledger bot why DOM");

  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await startRiverLedgerShowdown(page);
  await assertRiverLedgerStatusAnnouncement(page);
  await assertNoLeak(page, consoleMessages, "river_ledger status announcement DOM");

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "a11y noleak keyboard reduced high_card_seat_frame directional_flip flood_watch event_frontier river_ledger" }));
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

async function startFloodWatch(page) {
  await clickText(page, "button", "Flood Watch");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="flood-watch-board"]');
}

async function startHighCardDuel(page) {
  await clickText(page, "button", "High Card Duel");
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="high-card-duel-board"]');
}

async function startRiverLedgerShowdown(page) {
  await clickText(page, "button", "River Ledger");
  await page.select('select[aria-label="Supported seats from Rust catalog"]', "4");
  await page.$eval('.setup-grid input[type="number"]', (input) => {
    input.value = "79";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
  });
  await clickLabel(page, "Hotseat");
  await clickText(page, "button", "Start Match");
  await waitForText(page, "Available choices");
  for (const action of [
    "Call",
    "Call",
    "Call",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
    "Check",
  ]) {
    await clickRiverAction(page, action);
  }
  await page.waitForSelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
}

async function startRiverLedgerBotWhy(page) {
  await clickText(page, "button", "River Ledger");
  await page.select('select[aria-label="Supported seats from Rust catalog"]', "4");
  await clickText(page, "button", "Start Match");
  await page.waitForSelector('[data-testid="river-ledger-board"]');
  await page.waitForSelector('[data-testid="river-ledger-bot-explanation"]');
}

async function assertRiverLedgerBotWhy(page) {
  const summary = await page.evaluate(() => {
    const why = document.querySelector('[data-testid="river-ledger-bot-explanation"]');
    const effectLog = document.querySelector('[data-testid="effects"]')?.textContent ?? "";
    return {
      exists: Boolean(why),
      label: why?.getAttribute("aria-label") ?? "",
      text: why?.textContent ?? "",
      effectLog,
    };
  });
  assert(summary.exists, "river_ledger bot why disclosure renders for a non-random bot");
  assert(summary.label.includes("Why Seat "), `river_ledger bot why aria label uses public seat label: ${summary.label}`);
  assert(summary.text.includes("Why?"), "river_ledger bot why exposes compact summary text");
  assert(summary.text.includes("Call price"), "river_ledger bot why renders Rust public facts");
  assert(summary.text.includes("This public explanation omits private hole cards"), "river_ledger bot why renders Rust no-hidden-info notice");
  assert(!summary.text.toLowerCase().includes("candidate"), "river_ledger bot why omits candidate data");
  assert(!rawSeatIdRe.test(summary.text), `river_ledger bot why avoids raw seat ids: ${summary.text}`);
  assert(!summary.effectLog.includes("This public explanation"), "river_ledger bot why is not dumped into effect log");
}

async function assertRiverLedgerStatusAnnouncement(page) {
  const summary = await page.evaluate(() => {
    const status = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"] .river-ledger-showdown-lead[role="status"]');
    return {
      exists: Boolean(status),
      atomic: status?.getAttribute("aria-atomic") ?? "",
      label: status?.getAttribute("aria-label") ?? "",
      text: status?.textContent ?? "",
      panelText: document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]')?.textContent ?? "",
    };
  });
  assert(summary.exists, "river_ledger terminal banner exposes a status region");
  assert(summary.atomic === "true", `river_ledger status region is atomic: ${summary.atomic}`);
  assert(summary.label.includes("wins with") || summary.label.includes("split the ledger"), `river_ledger status uses Rust-authored result label: ${summary.label}`);
  assert(!rawSeatIdRe.test(summary.label), `river_ledger status avoids raw seat ids: ${summary.label}`);
  assert(summary.panelText.includes("Board usage"), "river_ledger reduced-motion terminal keeps reveal facts visible");
}

async function assertSeatFrameViewerNoLeak(page, consoleMessages) {
  const summary = await page.evaluate(() => ({
    labels: Array.from(document.querySelectorAll(".seat-frame-viewers label")).map((label) => label.textContent?.trim()),
    selected: document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.trim() ?? "",
    rail: document.querySelector(".seat-frame-rail")?.textContent ?? "",
  }));
  assert(summary.labels.includes("Observer"), "seat frame exposes observer viewer");
  assert(summary.labels.includes("Seat 0"), "seat frame exposes Seat 0 viewer");
  assert(summary.labels.includes("Seat 1"), "seat frame exposes Seat 1 viewer");
  assert(summary.selected === "Seat 0", `seat frame starts on Seat 0, got ${summary.selected}`);
  assert(summary.rail.includes("Active"), "seat frame rail reflects Rust-projected active seat");
  await assertNoPrivateSeatFrameLeak(page, consoleMessages, "high_card_duel seat-frame seat_0 DOM");

  await clickSeatFrameButton(page, "Seat 1");
  await page.waitForFunction(() => document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.includes("Seat 1"));
  await waitForText(page, "Waiting for active seat");
  await assertNoPrivateSeatFrameLeak(page, consoleMessages, "high_card_duel seat-frame seat_1 DOM");

  await clickSeatFrameButton(page, "Observer");
  await page.waitForFunction(() => document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.includes("Observer"));
  await waitForText(page, "Observer only");
  await assertNoPrivateSeatFrameLeak(page, consoleMessages, "high_card_duel seat-frame observer DOM");
  await assertStorageClean(page);
}

async function assertFixedTwoSeatSeatFrameNoLeak(page, consoleMessages, gameId) {
  const summary = await page.evaluate(() => ({
    labels: Array.from(document.querySelectorAll(".seat-frame-viewers label")).map((label) => label.textContent?.trim()),
    values: Array.from(document.querySelectorAll(".seat-frame-viewers input")).map((input) => input.value),
    rail: Array.from(document.querySelectorAll(".seat-frame-rail li")).map((item) => item.textContent?.trim() ?? ""),
  }));
  assert(
    summary.labels.length === 3 && summary.labels[0] === "Observer",
    `${gameId} generic viewer selector exposes observer plus two active seats: ${JSON.stringify(summary.labels)}`,
  );
  assert(
    JSON.stringify(summary.values) === JSON.stringify(["observer", "seat_0", "seat_1"]),
    `${gameId} generic viewer selector uses active seat ids: ${JSON.stringify(summary.values)}`,
  );
  assert(summary.rail.length === 2, `${gameId} rail has exactly two active seats: ${summary.rail.join(" | ")}`);

  for (const label of ["Observer", summary.labels[2], summary.labels[1]]) {
    await clickSeatFrameButton(page, label);
    await page.waitForFunction(
      (expected) => document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.trim() === expected,
      {},
      label,
    );
    await assertNoPrivateSeatFrameLeak(page, consoleMessages, `${gameId} ${label} selector surface`);
  }
}

async function assertNoPrivateSeatFrameLeak(page, consoleMessages, label) {
  const surface = await page.evaluate(() =>
    [
      document.body.textContent ?? "",
      Array.from(document.querySelectorAll("*"))
        .flatMap((element) => Array.from(element.attributes).map((attribute) => `${attribute.name}=${attribute.value}`))
        .join("\n"),
      Array.from(document.querySelectorAll("[data-testid]"))
        .map((element) => element.getAttribute("data-testid"))
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

async function assertFloodWatchDeckA11y(page) {
  const summary = await page.evaluate(() => {
    const countText = document.querySelector('[data-testid="deck-face-down-count"]')?.textContent ?? "";
    const undrawnMetric =
      Array.from(document.querySelectorAll(".plain-tricks-metrics div")).find((item) => item.querySelector("span")?.textContent === "Undrawn")?.querySelector("strong")?.textContent ?? "";
    const faceDown = document.querySelector('[data-testid="deck-face-down"]');
    return {
      deckText: document.querySelector('[data-testid="deck-flow-panel"]')?.textContent ?? "",
      faceDownText: faceDown?.textContent ?? "",
      faceDownAttributes: faceDown ? Array.from(faceDown.attributes).map((attribute) => `${attribute.name}=${attribute.value}`) : [],
      faceDownCount: countText,
      viewUndrawnCount: undrawnMetric,
    };
  });
  assert(summary.deckText.includes("Storm deck"), "flood_watch renders Rust deck label");
  assert(summary.faceDownCount === summary.viewUndrawnCount, "flood_watch face-down count matches the public view");
  assert(summary.faceDownText.includes("Remaining storm cards stay face down. The count is public."), "flood_watch renders face-down no-leak copy");
  assert(!summary.faceDownText.includes("downpour/") && !summary.faceDownText.includes("storm_surge/"), "flood_watch face-down text omits raw card ids");
  assert(!summary.faceDownAttributes.join("\n").includes("downpour/"), "flood_watch face-down attributes omit raw card ids");
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
      deckText: document.querySelector('[data-testid="deck-flow-panel"]')?.textContent ?? "",
      currentText: document.querySelector('[data-testid="deck-current-card"]')?.textContent ?? "",
      nextText: document.querySelector('[data-testid="deck-next-card"]')?.textContent ?? "",
      faceDownText: document.querySelector('[data-testid="deck-face-down"]')?.textContent ?? "",
      faceDownAttributes: document.querySelector('[data-testid="deck-face-down"]')
        ? Array.from(document.querySelector('[data-testid="deck-face-down"]').attributes).map((attribute) => `${attribute.name}=${attribute.value}`)
        : [],
      faceDownCount: Boolean(document.querySelector('[data-testid="deck-face-down-count"]')),
      discard: Boolean(document.querySelector('[data-testid="deck-discard"]')),
      latest: document.querySelector(".event-frontier-board .plain-latest")?.textContent ?? "",
    };
  });
  assert(summary.sites === 6, `event_frontier renders six sites, got ${summary.sites}`);
  assert(summary.trails >= 6, `event_frontier renders public trail lines, got ${summary.trails}`);
  assert(summary.missingNames.length === 0, `event_frontier buttons have accessible names: ${summary.missingNames.join(", ")}`);
  assert(summary.deckText.includes("Event deck"), "event_frontier renders Rust deck label");
  assert(summary.currentText.includes("High Meadow Fair"), "event_frontier renders authored current card label");
  assert(summary.currentText.includes("Freeholders gain a provision and rally a settler at High Meadow."), "event_frontier renders authored current card summary");
  assert(summary.nextText.includes("First Reckoning"), "event_frontier renders authored next card label");
  assert(summary.nextText.includes("Resolve the first scoring Reckoning."), "event_frontier renders authored next card summary");
  assert(summary.faceDownText.includes("Face-down event deck"), "event_frontier renders Rust face-down label");
  assert(summary.faceDownText.includes("Order hidden until cards become public."), "event_frontier renders Rust face-down summary");
  assert(!summary.faceDownCount, "event_frontier face-down slot omits count when Rust provides none");
  assert(summary.discard, "event_frontier renders discard disclosure");
  assert(!summary.faceDownText.includes("ef_"), "event_frontier face-down text omits raw card ids");
  assert(!summary.faceDownAttributes.join("\n").includes("ef_"), "event_frontier face-down attributes omit raw card ids");
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
  await assertNoRawIdentifiers(page, label);
  await assertNoRawSeatIds(page, label);
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

async function assertRawIdentifierGuardTrips(page) {
  await page.evaluate(() => {
    const probe = document.createElement("p");
    probe.dataset.rawIdentifierProbe = "true";
    probe.textContent = "site_charterhouse should fail the runtime identifier guard";
    document.querySelector("main")?.append(probe);
  });
  const hits = await collectRawIdentifierHits(page);
  await page.evaluate(() => document.querySelector("[data-raw-identifier-probe]")?.remove());
  assert(
    hits.some((hit) => hit.token === "site_charterhouse"),
    `runtime raw-identifier guard did not catch induced drift: ${JSON.stringify(hits)}`,
  );
}

async function assertNoRawIdentifiers(page, label) {
  const hits = await collectRawIdentifierHits(page);
  assert(hits.length === 0, `${label} contains raw internal identifiers: ${JSON.stringify(hits)}`);
}

async function assertRawSeatIdGuardTrips(page) {
  await page.evaluate(() => {
    const probe = document.createElement("button");
    probe.type = "button";
    probe.dataset.rawSeatProbe = "true";
    probe.setAttribute("aria-label", "seat_5 should fail the raw seat guard");
    probe.textContent = "seat_5 should fail the raw seat guard";
    document.querySelector("main")?.append(probe);
  });
  const hits = await collectRawSeatIdHits(page);
  await page.evaluate(() => document.querySelector("[data-raw-seat-probe]")?.remove());
  assert(
    hits.some((hit) => hit.token === "seat_5"),
    `runtime raw-seat guard did not catch induced drift: ${JSON.stringify(hits)}`,
  );
}

async function assertNoRawSeatIds(page, label) {
  const hits = await collectRawSeatIdHits(page);
  assert(hits.length === 0, `${label} contains raw seat ids: ${JSON.stringify(hits)}`);
}

async function collectRawIdentifierHits(page) {
  const samples = await page.evaluate(() => {
    const excludedSelector = '.dev-panel, .replay-io, .replay-viewer, .diagnostic, [data-testid="effects"], script, style';
    const textSamples = [];
    const attributeSamples = [];

    const isExcluded = (element) => Boolean(element.closest(excludedSelector));
    const isVisible = (element) => {
      if (isExcluded(element) || element.getAttribute("aria-hidden") === "true") return false;
      const style = window.getComputedStyle(element);
      return style.display !== "none" && style.visibility !== "hidden";
    };

    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    while (walker.nextNode()) {
      const node = walker.currentNode;
      const parent = node.parentElement;
      const text = node.textContent?.trim() ?? "";
      if (parent && text && isVisible(parent)) {
        textSamples.push({ source: "text", value: text });
      }
    }

    for (const element of Array.from(document.querySelectorAll("*"))) {
      if (isExcluded(element) || !isVisible(element)) continue;
      for (const name of ["aria-label", "title", "alt", "aria-valuetext"]) {
        const value = element.getAttribute(name);
        if (value?.trim()) {
          attributeSamples.push({ source: name, value });
        }
      }
    }

    return [...textSamples, ...attributeSamples];
  });
  return samples.flatMap((sample) => {
    const tokens = sample.value.match(new RegExp(rawIdentifierRe.source, "g")) ?? [];
    return tokens.map((token) => ({ ...sample, token }));
  });
}

async function collectRawSeatIdHits(page) {
  const samples = await page.evaluate(() => {
    const excludedSelector = '.dev-panel, .replay-io, .replay-viewer, .diagnostic, [data-testid="effects"], script, style';
    const textSamples = [];
    const attributeSamples = [];

    const isExcluded = (element) => Boolean(element.closest(excludedSelector));
    const isVisible = (element) => {
      if (isExcluded(element) || element.getAttribute("aria-hidden") === "true") return false;
      const style = window.getComputedStyle(element);
      return style.display !== "none" && style.visibility !== "hidden";
    };

    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    while (walker.nextNode()) {
      const node = walker.currentNode;
      const parent = node.parentElement;
      const text = node.textContent?.trim() ?? "";
      if (parent && text && isVisible(parent)) {
        textSamples.push({ source: "text", value: text });
      }
    }

    for (const element of Array.from(document.querySelectorAll("*"))) {
      if (isExcluded(element) || !isVisible(element)) continue;
      for (const name of ["aria-label", "title", "alt", "aria-valuetext"]) {
        const value = element.getAttribute(name);
        if (value?.trim()) {
          attributeSamples.push({ source: name, value });
        }
      }
    }

    return [...textSamples, ...attributeSamples];
  });
  return samples.flatMap((sample) => {
    const tokens = sample.value.match(new RegExp(rawSeatIdRe.source, "gi")) ?? [];
    return tokens.map((token) => ({ ...sample, token }));
  });
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

async function clickSeatFrameButton(page, text) {
  const clicked = await page.evaluate((expected) => {
    const label = Array.from(document.querySelectorAll(".seat-frame-viewers label")).find(
      (candidate) => candidate.textContent?.trim() === expected,
    );
    if (!label) {
      return false;
    }
    label.click();
    return true;
  }, text);
  assert(clicked, `seat frame viewer option exists: ${text}`);
}

async function clickRiverAction(page, label) {
  const clicked = await page.evaluate((expected) => {
    const button = Array.from(document.querySelectorAll('[data-testid^="choice-river-ledger-"]')).find((candidate) =>
      candidate.textContent?.includes(expected),
    );
    if (!button) {
      return false;
    }
    button.click();
    return true;
  }, label);
  assert(clicked, `river_ledger action exists: ${label}`);
  await page.waitForFunction(
    () =>
      document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]') ||
      document.body.textContent?.includes("Available choices"),
  );
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
