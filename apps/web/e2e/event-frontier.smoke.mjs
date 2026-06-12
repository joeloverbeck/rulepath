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
  "full_deck_order",
  "internal_trace_full_deck_hash",
  "private_state",
  "hidden_state",
  "bot_candidate",
  "candidate_ranking",
];
const authoredEventLabels = [
  "Border Survey",
  "Toll Roads",
  "River Mists",
  "Storehouse Fire",
  "Survey Ban",
  "High Meadow Fair",
  "First Reckoning",
  "Depot Grants",
  "Long Season",
  "Trail Washout",
  "Charter Audit",
  "Freeholder Moot",
  "Requisition",
  "Second Reckoning",
  "Old Mill Strike",
  "Crossing Market",
  "Granite Pass Snows",
  "Cache Boom",
  "Agents Recall",
  "Last Light",
  "Final Reckoning",
];
const authoredEventSummaries = [
  "Charter places an agent at Crossing.",
  "An edict makes movement along roads harder until the next Reckoning.",
  "Freeholders move a settler from Landing toward High Meadow.",
  "Charter loses one fund if any.",
  "An edict limits survey operations until the next Reckoning.",
  "Freeholders gain a provision and rally a settler at High Meadow.",
  "Resolve the first scoring Reckoning.",
  "Charter gains two funds.",
  "An edict extends operation capacity until the next Reckoning.",
  "A settler at Crossing washes back to Landing.",
  "Charter gains a fund and removes one cache at Landing.",
  "Freeholders gain two provisions.",
  "An edict changes requisition pressure until the next Reckoning.",
  "Resolve the second scoring Reckoning.",
  "Remove one Charter agent from Charterhouse if present.",
  "Both factions gain one resource.",
  "Charter loses one fund if any.",
  "Freeholders place a cache at High Meadow.",
  "Move one Charter agent from Crossing back to Charterhouse if possible.",
  "Resolve the final scoring Reckoning.",
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

  await startEventFrontier(page, baseUrl, "Hotseat", 2, "Event Frontier: Hard Winter");
  await assertRenderedTextView(
    page,
    (view) => view?.game_id === "event_frontier" && view.variant_id === "event_frontier_hard_winter",
  );
  await assertNoLeak(page, consoleMessages, "hard winter variant start");

  await startEventFrontier(page, baseUrl, "Hotseat", 3);
  await assertFactionFirstCopy(page);
  await assertEventFrontierBoardA11y(page);
  await assertEventFrontierDetailsAndStatusCopy(page);
  await assertRenderedTextView(page, (view) => view?.game_id === "event_frontier" && view.active_seat === "seat_0");
  await installAnimationProbe(page);
  await chooseAndConfirmAction(page, ["Event"]);
  await waitForText(page, "Edict activated");
  await assertAnimationTargets(page, ["event-frontier-deck"], "event action");
  await chooseAndConfirmAction(page, ["Pass"]);
  await waitForText(page, "Reckoning resolved");
  await waitForText(page, "reckoning 1");
  await waitForText(page, "no instant victory");
  await assertAnimationTargets(
    page,
    ["event-frontier-map", "event-frontier-resource-faction_charter", "event-frontier-score-faction_charter"],
    "reckoning",
  );
  await assertAnimationTargetPrefix(page, "event-frontier-site-", "reckoning site highlights");
  await page.waitForSelector('[data-testid="event-frontier-board"] .frontier-site');
  await assertEventFrontierTurnReport(page);
  await assertEventFrontierDiscardDisclosure(page);
  await clickButtonText(page, "Developer panel");
  await clickButtonText(page, "Submit Stale Action");
  await waitForText(page, "stale_action");
  await assertNoLeak(page, consoleMessages, "charter event edict reckoning");

  await startEventFrontier(page, baseUrl, "Hotseat", 1);
  await assertRenderedTextView(page, (view) => view?.game_id === "event_frontier" && view.active_seat === "seat_1");
  await exerciseActionBuilderBackCancel(page);
  await chooseFirstOperationPath(page);
  await waitForText(page, "Operation resolved");
  await assertRenderedTextView(page, (view) => view?.game_id === "event_frontier" && view.active_seat === "seat_0");
  await chooseAndConfirmAction(page, ["Pass"]);
  await waitForText(page, "Reckoning resolved");
  await assertNoLeak(page, consoleMessages, "freeholder multi-site operation and pass");

  await startEventFrontier(page, baseUrl, "Bot vs bot", 3);
  const charterInstant = await playBotVsBotToTerminal(page);
  assert(charterInstant.status.includes("charter_instant"), "seed 3 reaches Charter instant victory");
  assert(charterInstant.seenSeats.has("seat_0") && charterInstant.seenSeats.has("seat_1"), "bot-vs-bot stepped both Event Frontier factions");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertEventFrontierBotWhy(page);
  await assertNoLeak(page, consoleMessages, "charter instant terminal");

  await startEventFrontier(page, baseUrl, "Bot vs bot", 55);
  const freeholderInstant = await playBotVsBotToTerminal(page);
  assert(freeholderInstant.status.includes("freeholder_instant"), "seed 55 reaches Freeholder instant victory");
  await page.waitForSelector(".outcome-explanation-panel");
  await assertNoLeak(page, consoleMessages, "freeholder instant terminal");

  await startEventFrontier(page, baseUrl, "Bot vs bot", 1);
  const finalFallback = await playBotVsBotToTerminal(page);
  assert(finalFallback.status.includes("final_fallback"), "seed 1 reaches final fallback victory");
  await page.waitForSelector(".outcome-explanation-panel");
  await clickButtonText(page, "Export Current Run");
  const replayText = await replayTextareaValue(page);
  assert(replayText.includes('"game_id": "event_frontier"'), "export keeps event_frontier game id");
  assert(replayText.includes('"hidden_information": "undrawn_deck_order"'), "export names the hidden surface without raw order");
  assert(!replayText.includes('"commands"'), "event_frontier public replay export omits raw command stream");
  assertNoForbiddenTerms(replayText, "event_frontier public replay export", forbiddenLeakTerms);
  await clickButtonText(page, "Import Replay");
  await waitForText(page, "Replay viewer");
  await waitForText(page, "Event Frontier");
  await waitForText(page, "Cursor 0 /");
  await clickButtonText(page, "Step");
  await waitForText(page, "Cursor 0 /");
  await assertNoLeak(page, consoleMessages, "event frontier replay import and step");

  await page.select(".motion-field select", "reduce");
  await page.waitForSelector(".event-frontier-board.reduced");
  const animationName = await page.$eval(".event-frontier-board.reduced .frontier-site", (element) =>
    window.getComputedStyle(element).animationName,
  );
  assert(animationName === "none", "reduced motion suppresses Event Frontier site animation");

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".frontier-layout");
  const columns = await page.$eval(".frontier-layout", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive Event Frontier layout remains measurable");

  await assertStorageClean(page);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", forbiddenLeakTerms);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "event_frontier event op reckoning terminal replay noleak reduced" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startEventFrontier(page, baseUrl, modeLabel, seed, variantLabel = "Event Frontier") {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "Event Frontier");
  await assertPickerAndSetupHygiene(page);
  await clickButtonText(page, "Event Frontier");
  if (variantLabel !== "Event Frontier") {
    await page.select(".setup-region label.field select", variantLabelToId(variantLabel));
    await waitForText(page, variantLabel);
  }
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
  await assertEventFrontierSetupCopy(page, modeLabel);
  await clickButtonText(page, "Start Match");
  await page.waitForSelector('[data-testid="event-frontier-board"]');
}

async function assertPickerAndSetupHygiene(page) {
  const summary = await page.evaluate(() => ({
    pickerText: document.querySelector(".game-list")?.textContent ?? "",
    setupText: document.querySelector(".setup-region")?.textContent ?? "",
    eventCardSelectable: Boolean(document.querySelector('.game-card[aria-label="Select Event Frontier"]')),
  }));
  for (const text of [summary.pickerText, summary.setupText]) {
    assert(!text.includes("rules 1"), "normal setup surfaces omit rules version copy");
    assert(!text.includes("schema 1"), "normal setup surfaces omit schema version copy");
    assert(!text.includes("event_frontier_hard_winter"), "normal setup surfaces omit raw hard-winter variant id");
    assert(!text.includes("event_frontier_land_rush"), "normal setup surfaces omit raw land-rush variant id");
  }
  assert(summary.eventCardSelectable, "Event Frontier game-card wrapper is selectable");
}

function variantLabelToId(label) {
  switch (label) {
    case "Event Frontier: Hard Winter":
      return "event_frontier_hard_winter";
    case "Event Frontier: Land Rush":
      return "event_frontier_land_rush";
    default:
      return "event_frontier_standard";
  }
}

async function assertEventFrontierSetupCopy(page, modeLabel) {
  await page.waitForFunction(() => document.querySelector(".seat-roles")?.textContent?.includes("Charter"));
  const summary = await page.evaluate(() => ({
    roles: Array.from(document.querySelectorAll(".seat-roles > div")).map((element) => element.textContent ?? ""),
    setup: document.querySelector(".setup-region")?.textContent ?? "",
  }));
  const expectedFirstRole = modeLabel === "Bot vs bot" ? "bot" : "you (local)";
  const expectedSecondRole = modeLabel === "Hotseat" ? "local" : "bot";
  assert(
    summary.roles.some((text) => text.includes("Charter") && text.includes(expectedFirstRole)),
    `setup names Charter without raw seat copy: ${JSON.stringify(summary.roles)}`,
  );
  assert(
    summary.roles.some((text) => text.includes("Freeholders") && text.includes(expectedSecondRole)),
    "setup names Freeholders without raw seat copy",
  );
  assert(!summary.setup.includes("Seat 0"), "Event Frontier setup omits Seat 0 copy");
  assert(!summary.setup.includes("Seat 1"), "Event Frontier setup omits Seat 1 copy");
}

async function assertEventFrontierBoardA11y(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="event-frontier-board"]')),
      sites: document.querySelectorAll(".frontier-site").length,
      trails: document.querySelectorAll(".frontier-trail").length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      latest: document.querySelector(".plain-latest")?.textContent ?? "",
      deckText: document.querySelector('[data-testid="deck-flow-panel"]')?.textContent ?? "",
      currentText: document.querySelector('[data-testid="deck-current-card"]')?.textContent ?? "",
      nextText: document.querySelector('[data-testid="deck-next-card"]')?.textContent ?? "",
      faceDownText: document.querySelector('[data-testid="deck-face-down"]')?.textContent ?? "",
      faceDownCount: Boolean(document.querySelector('[data-testid="deck-face-down-count"]')),
      discard: Boolean(document.querySelector('[data-testid="deck-discard"]')),
      actionButtons: Array.from(document.querySelectorAll('[data-testid^="action-path-choice-"]')).map((button) =>
        button.textContent?.trim(),
      ),
    };
  });
  assert(summary.board, "event_frontier board renders");
  assert(summary.sites === 6, `event_frontier renders six sites, got ${summary.sites}`);
  assert(summary.trails >= 6, `event_frontier renders public trail lines, got ${summary.trails}`);
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.latest.length > 0, "event_frontier latest-effect region has text");
  assert(summary.deckText.includes("Event deck"), "event_frontier renders Rust deck label");
  assert(includesAny(summary.currentText, authoredEventLabels), "event_frontier renders authored current card label");
  assert(includesAny(summary.currentText, authoredEventSummaries), "event_frontier renders authored current card summary");
  assert(includesAny(summary.nextText, authoredEventLabels), "event_frontier renders authored next card label");
  assert(includesAny(summary.nextText, authoredEventSummaries), "event_frontier renders authored next card summary");
  assert(summary.faceDownText.includes("Face-down event deck"), "event_frontier renders Rust face-down label");
  assert(summary.faceDownText.includes("Order hidden until cards become public."), "event_frontier renders Rust face-down summary");
  assert(!summary.faceDownCount, "event_frontier face-down slot omits count when Rust provides none");
  assert(summary.discard, "event_frontier renders discard disclosure");
  assert(!summary.deckText.includes("ef_"), "event_frontier deck panel omits raw card ids");
  assert(summary.actionButtons.some((label) => label === "Event"), "event_frontier renders Rust event choice label");
  assert(summary.actionButtons.some((label) => label === "Operation"), "event_frontier renders Rust operation choice label");
}

async function assertEventFrontierDetailsAndStatusCopy(page) {
  const summary = await page.evaluate(() => {
    const cards = Array.from(document.querySelectorAll('[data-testid="deck-current-card"] .deck-flow-card, [data-testid="deck-next-card"] .deck-flow-card'));
    const details = cards.map((card) => {
      const disclosure = card.querySelector(".deck-flow-card-details");
      if (disclosure instanceof HTMLDetailsElement) {
        disclosure.open = true;
      }
      return {
        label: card.querySelector("strong")?.textContent ?? "",
        summary: card.querySelector("p")?.textContent ?? "",
        detail: disclosure?.querySelector("p")?.textContent ?? "",
      };
    });
    return {
      details,
      eligibility: document.querySelector('[aria-label="Eligibility and victory distance"]')?.textContent ?? "",
    };
  });
  assert(summary.details.length > 0, "event_frontier card slots render cards");
  assert(summary.details.every((entry) => entry.detail && entry.detail !== entry.summary), "event_frontier card details use authored deep prose");
  assert(
    summary.details.some((entry) => entry.detail.includes("until the next Reckoning") || entry.detail.includes("public scoring")),
    "event_frontier detail prose explains edict or Reckoning scope",
  );
  assert(summary.eligibility.includes("controlled sites for instant victory"), "Charter threshold copy is framed");
  assert(summary.eligibility.includes("caches for instant victory"), "Freeholders threshold copy is framed");
}

async function assertFactionFirstCopy(page) {
  const summary = await page.evaluate(() => {
    const body = document.body.textContent ?? "";
    const devPanel = document.querySelector(".dev-panel")?.textContent ?? "";
    return {
      body,
      devPanel,
      board: document.querySelector('[data-testid="event-frontier-board"]')?.textContent ?? "",
      mode: document.querySelector(".mode-controls")?.textContent ?? "",
    };
  });
  assert(summary.board.includes("You play the Charter"), "board states the local faction identity");
  assert(summary.board.includes("Funds - Charter (you)"), "fund resource names Charter owner");
  assert(summary.board.includes("Provisions - Freeholders (local)"), "provision resource names Freeholders owner");
  assert(summary.mode.includes("Charter (you) to act"), "mode line uses faction-first active actor copy");
  const normalSurface = summary.body.replace(summary.devPanel, "");
  assert(!normalSurface.includes("Seat 0"), "normal Event Frontier surface omits Seat 0 copy");
  assert(!normalSurface.includes("Seat 1"), "normal Event Frontier surface omits Seat 1 copy");
  assert(!normalSurface.includes("seat_0"), "normal Event Frontier surface omits raw seat_0 copy");
  assert(!normalSurface.includes("seat_1"), "normal Event Frontier surface omits raw seat_1 copy");
}

async function assertEventFrontierTurnReport(page) {
  const report = await page.$eval('[data-testid="turn-report-panel"]', (element) => element.textContent ?? "");
  assert(report.includes("Turn report"), "Event Frontier turn report renders near the board");
  assert(report.includes("Reckoning resolved"), "Event Frontier turn report includes the Reckoning burst");
  assert(report.includes("First Reckoning") || report.includes("Reckoning 1"), "Event Frontier turn report uses authored Reckoning vocabulary");
  assert(!report.includes("full_deck_order"), "Event Frontier turn report does not expose hidden deck order");
}

function includesAny(value, candidates) {
  return candidates.some((candidate) => value.includes(candidate));
}

async function assertEventFrontierDiscardDisclosure(page) {
  const summary = await page.evaluate(() => {
    const disclosure = document.querySelector('[data-testid="deck-discard"]');
    if (disclosure instanceof HTMLDetailsElement) {
      disclosure.open = true;
    }
    return {
      open: disclosure instanceof HTMLDetailsElement ? disclosure.open : false,
      labels: Array.from(document.querySelectorAll('[data-testid="deck-discard-card"]')).map((card) => card.textContent ?? ""),
      text: disclosure?.textContent ?? "",
    };
  });
  assert(summary.open, "event_frontier discard disclosure expands");
  assert(summary.labels.length > 0, "event_frontier discard disclosure lists resolved cards");
  assert(summary.labels.some((label) => includesAny(label, authoredEventLabels)), "event_frontier discard disclosure lists authored card labels");
  assert(!summary.text.includes("ef_"), "event_frontier discard disclosure omits raw card ids");
}

async function installAnimationProbe(page) {
  await page.evaluate(() => {
    if (window.__eventFrontierAnimationProbeInstalled) {
      window.__eventFrontierAnimationTargets = [];
      return;
    }
    window.__eventFrontierAnimationProbeInstalled = true;
    window.__eventFrontierAnimationTargets = [];
    const originalAnimate = Element.prototype.animate;
    Element.prototype.animate = function patchedAnimate(...args) {
      const target = this.closest("[data-animation-target]")?.getAttribute("data-animation-target");
      if (target) {
        window.__eventFrontierAnimationTargets.push(target);
      }
      return originalAnimate.apply(this, args);
    };
  });
}

async function assertAnimationTargets(page, targets, label) {
  try {
    await page.waitForFunction(
      (expectedTargets) => {
        const seen = window.__eventFrontierAnimationTargets ?? [];
        return expectedTargets.every((target) => seen.includes(target));
      },
      {},
      targets,
    );
  } catch (error) {
    const debug = await page.evaluate(() => ({
      seen: window.__eventFrontierAnimationTargets ?? [],
      targets: Array.from(document.querySelectorAll("[data-animation-target]")).map((element) =>
        element.getAttribute("data-animation-target"),
      ),
      text: window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null,
    }));
    throw new Error(`${label} animation targets missing ${targets.join(", ")}; debug=${JSON.stringify(debug)}`, { cause: error });
  }
  const seen = await page.evaluate(() => window.__eventFrontierAnimationTargets ?? []);
  for (const target of targets) {
    assert(seen.includes(target), `${label} animated ${target}`);
  }
}

async function assertAnimationTargetPrefix(page, prefix, label) {
  await page.waitForFunction(
    (expectedPrefix) => (window.__eventFrontierAnimationTargets ?? []).some((target) => target.startsWith(expectedPrefix)),
    {},
    prefix,
  );
  const seen = await page.evaluate(() => window.__eventFrontierAnimationTargets ?? []);
  assert(seen.some((target) => target.startsWith(prefix)), `${label} animated target with prefix ${prefix}`);
}

async function exerciseActionBuilderBackCancel(page) {
  await clickButtonText(page, "Operation");
  await waitForActionTrail(page, "Operation");
  await clickButtonText(page, "Back");
  await waitForText(page, "Event");
  await clickButtonText(page, "Operation");
  await waitForActionTrail(page, "Operation");
  await clickButtonText(page, "Cancel");
  await waitForText(page, "Event");
}

async function chooseFirstOperationPath(page) {
  await clickButtonText(page, "Operation");
  await clickFirstActionPathChoice(page);
  await assertActionCostDisplay(page);
  await clickPreferredLeafChoice(page);
  await waitForText(page, "Ready");
  await assertActionConfirmSummary(page);
  await clickButtonText(page, "Confirm");
}

async function chooseAndConfirmAction(page, labels) {
  for (const label of labels) {
    await clickButtonText(page, label);
  }
  await waitForText(page, "Ready");
  await clickButtonText(page, "Confirm");
}

async function waitForActionTrail(page, text) {
  await page.waitForFunction((expected) => document.querySelector('[data-testid="action-path-trail"]')?.textContent?.includes(expected), {}, text);
}

async function clickFirstActionPathChoice(page) {
  await page.waitForSelector('[data-testid^="action-path-choice-"]');
  const choices = await page.$$('[data-testid^="action-path-choice-"]');
  if (!choices.length) {
    throw new Error("No action path choices rendered");
  }
  await choices[0].click();
}

async function clickPreferredLeafChoice(page) {
  const composer = await page.$('[data-testid="action-target-composer"]');
  if (composer) {
    await assertTargetComposer(page);
    return;
  }
  await page.waitForSelector('[data-testid^="action-path-choice-"]');
  const choices = await page.$$('[data-testid^="action-path-choice-"]');
  for (const choice of choices) {
    const text = await choice.evaluate((element) => element.textContent ?? "");
    if (text.includes(",")) {
      await choice.click();
      return;
    }
  }
  if (!choices.length) {
    throw new Error("No leaf choices rendered");
  }
  await choices[0].click();
}

async function assertTargetComposer(page) {
  await page.waitForSelector('[data-testid^="action-target-toggle-"]');
  const before = await page.evaluate(() => ({
    targetCount: document.querySelectorAll('[data-testid^="action-target-toggle-"]').length,
    confirmCount: document.querySelectorAll('[data-testid="action-target-confirm"]').length,
    legacyLeafButtons: Array.from(document.querySelectorAll('[data-testid^="action-path-choice-"]')).filter((button) =>
      button.textContent?.includes(","),
    ).length,
  }));
  assert(before.targetCount > 1, "target composer renders per-target toggles");
  assert(before.targetCount + before.confirmCount <= before.targetCount + 1, "target composer renders no more controls than targets plus confirm");
  assert(before.legacyLeafButtons === 0, "target composer replaces pre-joined combination buttons");

  const toggles = await page.$$('[data-testid^="action-target-toggle-"]');
  await toggles[0].click();
  if (toggles.length > 1) {
    await toggles[1].click();
  }
  const after = await page.evaluate(() => ({
    disabledRemainder: Array.from(document.querySelectorAll('[data-testid^="action-target-toggle-"]'))
      .filter((button) => button.getAttribute("aria-pressed") !== "true")
      .every((button) => button.hasAttribute("disabled")),
    highlighted: document.querySelectorAll(".frontier-site.highlighted").length,
  }));
  assert(after.disabledRemainder, "composer disables target additions without a matching Rust leaf");
  assert(after.highlighted >= Math.min(2, toggles.length), "map sites highlight selected composer targets");
}

async function assertActionCostDisplay(page) {
  const summary = await page.evaluate(() => ({
    chipText: Array.from(document.querySelectorAll('[data-testid="action-cost-chip"]')).map((chip) => chip.textContent ?? ""),
    consequence: document.querySelector(".action-path-consequence")?.textContent ?? "",
  }));
  assert(summary.chipText.some((text) => /\b[1-9][0-9]* (funds?|provisions?)\b/.test(text)), "operation leaves render Rust-supplied cost chips");
  assert(
    summary.consequence.includes("forfeits your eligibility for the next card"),
    "operation stage renders Rust-authored eligibility consequence",
  );
}

async function assertActionConfirmSummary(page) {
  const summary = await page.$eval('[data-testid="action-path-confirm-summary"]', (element) => element.textContent ?? "");
  assert(summary.includes("Spends "), "confirm summary states operation cost");
  assert(summary.includes(" of your "), "confirm summary compares cost against visible balance");
  assert(summary.includes("forfeits your eligibility for the next card"), "confirm summary states eligibility consequence");
}

async function assertEventFrontierBotWhy(page) {
  await page.waitForSelector('[data-testid="bot-explanation"]');
  const open = await page.$eval('[data-testid="bot-explanation"]', (element) => element.hasAttribute("open"));
  if (!open) {
    await page.click('[data-testid="bot-explanation"] summary');
  }
  const text = await page.$eval('[data-testid="bot-explanation"]', (element) => element.textContent ?? "");
  assert(text.includes("Bot why"), "Event Frontier exposes the bot why affordance");
  assert(/Charter|Freeholders/.test(text), "Event Frontier bot why shows Rust rationale in player vocabulary");
  assert(text.includes("Level 1 bot policy"), "Event Frontier bot why identifies the human-readable Rust policy tier");
}

async function playBotVsBotToTerminal(page, maxSteps = 32) {
  const seenSeats = new Set();
  for (let step = 0; step < maxSteps; step += 1) {
    const view = await textView(page);
    if (view?.status?.includes("won by")) {
      return { status: view.status, seenSeats };
    }
    if (view?.active_seat) {
      seenSeats.add(view.active_seat);
    }
    const before = view?.freshness_token ?? 0;
    await clickButtonText(page, "Step Bot");
    await waitForFreshnessGreaterThan(page, before);
  }
  const finalView = await textView(page);
  throw new Error(`Event Frontier bot-vs-bot did not reach terminal: ${JSON.stringify(finalView)}`);
}

async function waitForFreshnessGreaterThan(page, freshnessToken) {
  await page.waitForFunction(
    (minimum) => {
      if (!window.render_game_to_text) return false;
      const state = JSON.parse(window.render_game_to_text());
      return state?.view?.game_id === "event_frontier" && state.view.freshness_token > minimum;
    },
    {},
    freshnessToken,
  );
}

async function waitForRenderedView(page, predicate) {
  await page.waitForFunction(
    (predicateSource) => {
      if (!window.render_game_to_text) return false;
      const state = JSON.parse(window.render_game_to_text());
      return Function("view", `return (${predicateSource})(view);`)(state.view);
    },
    {},
    predicate.toString(),
  );
}

async function assertRenderedTextView(page, predicate) {
  const ok = await page.evaluate((predicateSource) => {
    const state = window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null;
    return Function("view", `return (${predicateSource})(view);`)(state?.view);
  }, predicate.toString());
  assert(ok, "render_game_to_text view satisfied expected Event Frontier condition");
}

async function textView(page) {
  const state = await page.evaluate(() => (window.render_game_to_text ? JSON.parse(window.render_game_to_text()) : null));
  return state?.view ?? null;
}

async function assertNoLeak(page, consoleMessages, label) {
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, forbiddenLeakTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, forbiddenLeakTerms);
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

async function replayTextareaValue(page) {
  const handle = await page.waitForFunction(() => document.querySelector("textarea")?.value || "");
  return handle.jsonValue();
}

async function clickButtonText(page, text) {
  const handle = await waitForTextHandle(page, "button", text);
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
