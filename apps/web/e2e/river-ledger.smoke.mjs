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
const ranks = ["two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen", "king", "ace"];
const suits = ["clubs", "diamonds", "hearts", "spades"];
const cardIds = ranks.flatMap((rank) => suits.map((suit) => `${rank}_${suit}`));
const internalTerms = [
  "deck_tail",
  "deck tail",
  "reserved_community",
  "reserved community",
  "burn",
  "seed_evidence",
  "hidden_state",
  "private_state",
  "internal_state",
  "debug_state",
  "candidate_ranking",
  "bot_candidate",
];
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
  await page.setViewport({ width: 1180, height: 920 });
  await page.emulateMediaFeatures([{ name: "prefers-reduced-motion", value: "reduce" }]);
  await assertRawSeatIdGuardTrips(page, baseUrl);

  const selectedSeatCount = 4;
  await startRiverLedger(page, baseUrl, "Human vs bot", selectedSeatCount);
  await page.waitForSelector('[data-testid="river-ledger-board"]');
  await waitForText(page, "Seat 1 to choose");
  await assertModeStatusUsesSeatLabel(page);
  await assertRiverLedgerA11y(page, true, selectedSeatCount);
  await assertHandRankingReferenceDuringPlay(page);
  await assertSeatAndStreetAffordancesDuringPlay(page);
  const seat0Cards = await ownPrivateCardLabels(page);
  assert(seat0Cards.length === 2, "seat 0 private view exposes two own cards");
  await assertPrivateCardComponent(page);
  await assertRiverLedgerCardContainment(page, "private card render");
  await assertNoLeak(page, consoleMessages, "seat 0 view", cardIds.filter((id) => !seat0Cards.map(labelToId).includes(id)));

  await clickSeatFrameButton(page, "Observer");
  await waitForText(page, "Observer view");
  await assertNoLeak(page, consoleMessages, "observer view", cardIds);
  await assertStorageClean(page);

  await clickSeatFrameButton(page, "Seat 2");
  await waitForText(page, "Seat 2 view");
  const seat1Cards = await ownPrivateCardLabels(page);
  assert(seat1Cards.length === 2, "seat 1 private view exposes two own cards");
  await assertNoLeak(page, consoleMessages, "wrong-seat view", seat0Cards.map(labelToId));

  await assertPreShowdownPairwiseNoLeak(page, consoleMessages, selectedSeatCount);

  await clickSeatFrameButton(page, "Seat 1");
  await waitForText(page, "Available choices");
  const choices = await page.$$eval('[data-testid^="choice-river-ledger-"]', (buttons) =>
    buttons.map((button) => ({
      label: button.textContent ?? "",
      disabled: button.disabled,
      aria: button.getAttribute("aria-label") ?? "",
    })),
  );
  assert(choices.length > 0, "river_ledger exposes Rust-provided legal action buttons");
  assert(choices.every((choice) => choice.aria.length > 0), "river_ledger choices have accessible names");
  assert(choices.every((choice) => !choice.disabled), "river_ledger renders legal enabled choices only for active human seat");
  await assertActionPanelCostCopy(page);

  await clickRiverAction(page, "Fold");
  await page.waitForFunction(() => document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]'), {
    timeout: 15000,
  });
  await waitForText(page, "Outcome");
  const outcomeText = await page.$eval(".outcome-explanation-panel", (element) => element.textContent ?? "");
  assert(outcomeText.includes("Seat"), "river_ledger terminal outcome names seats");
  await assertFoldedSeatNoStrength(page);
  await assertNoRawSeatIds(page, "terminal surface");
  assertNoForbiddenTerms(await fullBrowserSurface(page), "terminal surface", internalTerms);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "console logs", internalTerms);

  await page.setViewport({ width: 390, height: 820 });
  await page.waitForSelector(".river-ledger-layout");
  const columns = await page.$eval(".river-ledger-layout", (element) => window.getComputedStyle(element).gridTemplateColumns);
  assert(!columns.includes(" 0px "), "responsive river_ledger layout remains measurable");

  await page.setViewport({ width: 1180, height: 920 });
  await playWorkedExampleShowdown(page, baseUrl);
  await assertWorkedExampleShowdown(page);
  await assertNoRawSeatIds(page, "worked-example showdown surface");
  await assertShowdownCardComponents(page);
  await assertRiverLedgerCardContainment(page, "showdown card render");
  await assertHandRankingReferenceAfterShowdown(page);
  await assertTeachingAidAfterShowdown(page);
  await assertStreetStripAfterShowdown(page);
  assertNoForbiddenTerms(await fullBrowserSurface(page), "worked-example showdown surface", [...cardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "worked-example console logs", internalTerms);
  await assertStorageClean(page);

  await playSeed10018Showdown(page, baseUrl);
  await assertSeed10018ShowdownLabels(page);
  await assertNoRawSeatIds(page, "seed-10018 showdown surface");
  assertNoForbiddenTerms(await fullBrowserSurface(page), "seed-10018 showdown surface", internalTerms);

  await playShortStackAllInTerminal(page, baseUrl);
  await assertShortStackAllInTerminal(page);
  await assertNoRawSeatIds(page, "short-stack all-in terminal surface");
  assertNoForbiddenTerms(await fullBrowserSurface(page), "short-stack all-in terminal surface", internalTerms);

  await assertSixSeatFrameSelector(page, baseUrl);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "river_ledger noleak legal controls terminal responsive" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startRiverLedger(page, baseUrl, modeLabel, seatCount, seed = null, stackSetup = null) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "River Ledger");
  await assertFixedSeatSetup(page);
  await clickText(page, "button", "River Ledger");
  await assertVariableSeatSetup(page, seatCount);
  if (seed !== null) {
    await setSetupSeed(page, seed);
  }
  await clickLabel(page, modeLabel);
  if (stackSetup) {
    await configureRiverLedgerStacks(page, stackSetup);
  }
  await clickText(page, "button", "Start Match");
}

async function playWorkedExampleShowdown(page, baseUrl) {
  await startRiverLedger(page, baseUrl, "Hotseat", 4, 79);
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
    if (action !== "Check" || !(await hasRiverLedgerOutcome(page))) {
      await waitForText(page, "Available choices").catch(async () => {
        if (!(await hasRiverLedgerOutcome(page))) {
          throw new Error(`River Ledger did not advance after ${action}`);
        }
      });
    }
  }
  await page.waitForSelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
}

async function playSeed10018Showdown(page, baseUrl) {
  await startRiverLedger(page, baseUrl, "Hotseat", 4, 10018);
  await waitForText(page, "Available choices");
  for (const action of ["Call", "Call", "Call", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check", "Check"]) {
    await clickRiverAction(page, action);
    if (action !== "Check" || !(await hasRiverLedgerOutcome(page))) {
      await waitForText(page, "Available choices").catch(async () => {
        if (!(await hasRiverLedgerOutcome(page))) {
          throw new Error(`River Ledger seed 10018 did not advance after ${action}`);
        }
      });
    }
  }
  await page.waitForSelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
}

async function playShortStackAllInTerminal(page, baseUrl) {
  await startRiverLedger(page, baseUrl, "Hotseat", 3, 12, { mode: "custom", stacks: [2, 2, 2] });
  await waitForText(page, "Available choices");
  await assertShortStackSetupRendered(page);
  for (let index = 0; index < 6 && !(await hasRiverLedgerOutcome(page)); index += 1) {
    if (await hasEnabledRiverAction(page, "Call")) {
      await clickRiverAction(page, "Call");
    } else if (await hasEnabledRiverAction(page, "Check")) {
      await clickRiverAction(page, "Check");
    } else {
      break;
    }
    if (!(await hasRiverLedgerOutcome(page))) {
      await waitForText(page, "Available choices").catch(async () => {
        if (!(await hasRiverLedgerOutcome(page))) {
          throw new Error("Short-stack all-in path did not advance");
        }
      });
    }
  }
  await page.waitForSelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
}

async function assertWorkedExampleShowdown(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    const showdown = panel?.querySelector(".river-ledger-showdown-panel");
    return {
      text: panel?.textContent ?? "",
      showdownText: showdown?.textContent ?? "",
      handCount: showdown?.querySelectorAll(".river-ledger-showdown-hand").length ?? 0,
      cardCount: showdown?.querySelectorAll(".river-ledger-showdown-card").length ?? 0,
      boardUsageCards: showdown?.querySelectorAll(".river-ledger-showdown-board .river-ledger-showdown-usage-card").length ?? 0,
      foldedStrengthRows: Array.from(panel?.querySelectorAll(".river-ledger-showdown-folded p, .outcome-standing-row") ?? [])
        .filter((row) => row.textContent?.includes("Folded"))
        .filter((row) => /Pair|High Card|tie break/i.test(row.textContent ?? "")).length,
    };
  });
  assert(summary.text.includes("wins with Pair of Queens."), `worked example headline is rendered: ${summary.text}`);
  assert(
    summary.text.includes("Pair of Queens beats Pair of Eights."),
    `worked example decisive comparison is rendered: ${summary.text}`,
  );
  assert(
    summary.text.includes("Both hands are one pair, so the pair rank decides first: Queens > Eights."),
    `worked example comparison basis is rendered: ${summary.text}`,
  );
  assert(summary.handCount === 4, `worked example renders four revealed hands: ${summary.handCount}`);
  assert(summary.boardUsageCards === 5, `worked example renders board usage once: ${summary.boardUsageCards}`);
  assert(summary.cardCount >= 25, `worked example renders board usage plus each best-five hand: ${summary.cardCount}`);
  assert(summary.foldedStrengthRows === 0, "worked example renders no folded-seat hand strength");
}

async function assertSeed10018ShowdownLabels(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    const latest = document.querySelector(".river-ledger-latest");
    const heading = document.querySelector("#river-ledger-heading");
    const banner = panel?.querySelector('[data-animation-target="river-ledger-showdown-banner"]');
    const standings = Array.from(panel?.querySelectorAll(".outcome-standing-row, .river-ledger-showdown-standing") ?? []).map(
      (row) => row.textContent ?? "",
    );
    return {
      panelText: panel?.textContent ?? "",
      latestText: latest?.textContent ?? "",
      headingText: heading?.textContent ?? "",
      bannerText: banner?.textContent ?? "",
      standings,
    };
  });

  const combined = [summary.panelText, summary.latestText, summary.headingText, summary.bannerText, ...summary.standings].join("\n");
  assert(summary.headingText.includes("Seat 1 wins"), `generic outcome heading uses Rust label: ${summary.headingText}`);
  assert(summary.latestText.includes("Seat 1 wins"), `live outcome announcement uses Rust label: ${summary.latestText}`);
  assert(
    summary.bannerText.includes("Seat 1 wins with Two pair, Queens and Fives."),
    `showdown banner uses same winner label: ${summary.bannerText}`,
  );
  assert(
    summary.panelText.includes("Two pair outranks One pair."),
    `decisive reason is rendered for seed 10018: ${summary.panelText}`,
  );
  assert(summary.panelText.includes("Seat 3"), `closest challenger label is one-based: ${summary.panelText}`);
  assert(!combined.includes("Seat 0 wins"), `seed 10018 surface must not use zero-based winner label: ${combined}`);
}

async function assertShortStackSetupRendered(page) {
  const summary = await page.evaluate(() => {
    const seats = Array.from(document.querySelectorAll(".river-ledger-seat")).map((seat) => seat.textContent ?? "");
    const pot = document.querySelector('[data-testid="river-ledger-pot-tiers"]')?.textContent ?? "";
    return {
      seats,
      pot,
      allInCount: seats.filter((seat) => seat.includes("All-in")).length,
      actions: Array.from(document.querySelectorAll('[data-testid^="choice-river-ledger-"]')).map((button) => button.textContent ?? ""),
    };
  });
  assert(summary.seats.length === 3, `short-stack setup renders three seats: ${summary.seats.join(" | ")}`);
  assert(
    summary.seats.every((seat) => seat.includes("Stack") && seat.includes("/ 2")),
    `short-stack seats render remaining/starting stacks: ${summary.seats.join(" | ")}`,
  );
  assert(summary.allInCount >= 1, `blind-posted short-stack setup marks an all-in seat: ${summary.seats.join(" | ")}`);
  assert(summary.pot.includes("Main pot") && summary.pot.includes("Eligible"), `short-stack setup renders public pot tier: ${summary.pot}`);
  assert(
    summary.actions.some((text) => text.includes("Call") && text.includes("Call price 2") && text.includes("Adds 2")),
    `short-stack action copy exposes Rust cost rows: ${summary.actions.join(" | ")}`,
  );
}

async function assertShortStackAllInTerminal(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    return {
      heading: document.querySelector("#river-ledger-heading")?.textContent ?? "",
      potText: document.querySelector('[data-testid="river-ledger-pot-tiers"]')?.textContent ?? "",
      seatText: Array.from(document.querySelectorAll(".river-ledger-seat")).map((seat) => seat.textContent ?? "").join("\n"),
      outcomeText: panel?.textContent ?? "",
      allocationRows: Array.from(panel?.querySelectorAll(".outcome-breakdown-section") ?? []).map((section) => section.textContent ?? ""),
      effectText: document.querySelector('[data-testid="effects"]')?.textContent ?? "",
    };
  });
  assert(summary.heading.includes("wins") || summary.heading.includes("split"), `short-stack hand reaches terminal: ${summary.heading}`);
  assert(summary.potText.includes("Main pot") && summary.potText.includes("Eligible"), `terminal keeps public pot tiers: ${summary.potText}`);
  assert(summary.seatText.includes("All-in"), `terminal keeps all-in seat indicators: ${summary.seatText}`);
  assert(summary.outcomeText.includes("Terminal allocations"), `terminal outcome renders allocation section: ${summary.outcomeText}`);
  assert(
    summary.allocationRows.some((row) => row.includes("Terminal allocations") && row.includes("Seat")),
    `terminal allocation rows use seat labels: ${summary.allocationRows.join(" | ")}`,
  );
  assert(
    summary.effectText.includes("All-in") || summary.effectText.includes("Stack updated") || summary.effectText.includes("Pot awarded"),
    `effect log names stack/pot all-in effects: ${summary.effectText}`,
  );
}

async function assertSixSeatFrameSelector(page, baseUrl) {
  await startRiverLedger(page, baseUrl, "Hotseat", 6, 7);
  await waitForText(page, "Available choices");
  const summary = await page.evaluate(() => ({
    labels: Array.from(document.querySelectorAll(".seat-frame-viewers label")).map((label) => label.textContent?.trim()),
    checked: document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.trim() ?? "",
    radioCount: document.querySelectorAll(".seat-frame-viewers input[type='radio']").length,
    rail: Array.from(document.querySelectorAll(".seat-frame-rail li")).map((item) => item.textContent?.trim() ?? ""),
  }));
  assert(
    JSON.stringify(summary.labels) === JSON.stringify(["Observer", "Seat 1", "Seat 2", "Seat 3", "Seat 4", "Seat 5", "Seat 6"]),
    `six-seat selector uses active labels only: ${JSON.stringify(summary.labels)}`,
  );
  assert(summary.radioCount === 7, `six-seat selector exposes one observer plus six seats: ${summary.radioCount}`);
  assert(summary.checked === "Seat 4", `hotseat starts on active six-seat viewer: ${summary.checked}`);
  assert(summary.rail.length === 6 && summary.rail.every((row, index) => row.includes(`Seat ${index + 1}`)), `six-seat rail is active-scoped: ${summary.rail.join(" | ")}`);

  for (const label of ["Observer", "Seat 1", "Seat 2", "Seat 3", "Seat 4", "Seat 5", "Seat 6"]) {
    await clickSeatFrameButton(page, label);
    await page.waitForFunction(
      (expected) => document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.includes(expected),
      {},
      label,
    );
    await waitForText(page, label === "Observer" ? "Observer view" : `${label} view`);
  }

  await page.focus('.seat-frame-viewers input[value="observer"]');
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Space");
  await page.waitForFunction(() => document.querySelector(".seat-frame-viewers input:checked")?.closest("label")?.textContent?.includes("Seat 1"));
  await waitForText(page, "Seat 1 view");
}

async function assertPreShowdownPairwiseNoLeak(page, consoleMessages, seatCount) {
  const seatLabels = Array.from({ length: seatCount }, (_, index) => `Seat ${index + 1}`);
  const cardsBySeat = new Map();

  for (const seatLabel of seatLabels) {
    await clickSeatFrameButton(page, seatLabel);
    await waitForText(page, `${seatLabel} view`);
    const ownCards = await ownPrivateCardLabels(page);
    assert(ownCards.length === 2, `${seatLabel} private view exposes two own cards`);
    cardsBySeat.set(seatLabel, ownCards.map(labelToId));
  }

  await clickSeatFrameButton(page, "Observer");
  await waitForText(page, "Observer view");
  await assertNoLeak(page, consoleMessages, "observer all-seat private-card surface", Array.from(cardsBySeat.values()).flat());

  for (const sourceSeat of seatLabels) {
    const sourceCards = cardsBySeat.get(sourceSeat) ?? [];
    for (const viewerSeat of seatLabels) {
      if (viewerSeat === sourceSeat) {
        continue;
      }
      await clickSeatFrameButton(page, viewerSeat);
      await waitForText(page, `${viewerSeat} view`);
      await assertNoLeak(page, consoleMessages, `${viewerSeat} view excludes ${sourceSeat} private cards`, sourceCards);
    }
  }
}

async function assertPrivateCardComponent(page) {
  const summary = await page.evaluate(() => {
    const privateCards = Array.from(document.querySelectorAll(".river-ledger-private .river-ledger-card.private"));
    const group = document.querySelector(".river-ledger-private-cards");
    return {
      cards: privateCards.length,
      glyphs: privateCards.filter((card) => card.querySelector(".river-ledger-card-suit b")?.textContent?.trim()).length,
      suitWords: privateCards.filter((card) => card.querySelector(".river-ledger-card-suit small")?.textContent?.trim()).length,
      ranks: privateCards.filter((card) => card.querySelector(".river-ledger-card-rank")?.textContent?.trim()).length,
      groupLabel: group?.getAttribute("aria-label") ?? "",
    };
  });
  assert(summary.cards === 2, `private card component renders two cards: ${summary.cards}`);
  assert(summary.glyphs === 2, `private cards render suit glyphs: ${summary.glyphs}`);
  assert(summary.suitWords === 2, `private cards render suit words: ${summary.suitWords}`);
  assert(summary.ranks === 2, `private cards render rank words: ${summary.ranks}`);
  assert(summary.groupLabel.includes("private cards"), `private card group has accessible label: ${summary.groupLabel}`);
}

async function assertShowdownCardComponents(page) {
  const summary = await page.evaluate(() => {
    const boardCards = Array.from(document.querySelectorAll(".river-ledger-board-cards .river-ledger-card.board"));
    const showdownCards = Array.from(document.querySelectorAll(".river-ledger-showdown-card.river-ledger-card.showdown"));
    const bestFiveCards = Array.from(document.querySelectorAll(".river-ledger-showdown-hand > .river-ledger-showdown-cards .river-ledger-showdown-card.river-ledger-card.showdown"));
    const usageCards = Array.from(document.querySelectorAll(".river-ledger-showdown-board .river-ledger-showdown-usage-card"));
    const boardGroup = document.querySelector(".river-ledger-board-cards");
    const showdownGroups = Array.from(document.querySelectorAll(".river-ledger-showdown-hand > .river-ledger-showdown-cards"));
    return {
      boardCards: boardCards.length,
      boardGlyphs: boardCards.filter((card) => card.querySelector(".river-ledger-card-suit b")?.textContent?.trim()).length,
      showdownCards: showdownCards.length,
      bestFiveCards: bestFiveCards.length,
      usageCards: usageCards.length,
      showdownGlyphs: showdownCards.filter((card) => card.querySelector(".river-ledger-card-suit b")?.textContent?.trim()).length,
      showdownSuitWords: showdownCards.filter((card) => card.querySelector(".river-ledger-card-suit small")?.textContent?.trim()).length,
      boardGroupLabel: boardGroup?.getAttribute("aria-label") ?? "",
      showdownGroupLabels: showdownGroups.map((group) => group.getAttribute("aria-label") ?? ""),
    };
  });
  assert(summary.boardCards === 5, `terminal board uses card component: ${summary.boardCards}`);
  assert(summary.boardGlyphs === 5, `terminal board renders suit glyphs: ${summary.boardGlyphs}`);
  assert(summary.bestFiveCards === 20, `showdown best-five uses card component: ${summary.bestFiveCards}`);
  assert(summary.usageCards === 5, `showdown board usage renders once: ${summary.usageCards}`);
  assert(summary.showdownCards >= 25, `showdown uses card component for best-five plus usage: ${summary.showdownCards}`);
  assert(summary.showdownGlyphs >= 25, `showdown renders suit glyphs: ${summary.showdownGlyphs}`);
  assert(summary.showdownSuitWords >= 25, `showdown renders suit words: ${summary.showdownSuitWords}`);
  assert(summary.boardGroupLabel.includes("Public board cards"), `board group has accessible label: ${summary.boardGroupLabel}`);
  assert(
    summary.showdownGroupLabels.length === 4 && summary.showdownGroupLabels.every((label) => label.includes("Best five")),
    `showdown groups have accessible labels: ${summary.showdownGroupLabels.join(" | ")}`,
  );
}

async function assertRiverLedgerCardContainment(page, label) {
  const realFailures = await riverLedgerCardOverflowFailures(page, ".river-ledger-card:not(.hidden)");
  assert(realFailures.length === 0, `${label} real cards stay contained: ${JSON.stringify(realFailures)}`);

  await page.evaluate(() => {
    document.querySelector("[data-testid='river-ledger-card-containment-fixture']")?.remove();
    const fixture = document.createElement("section");
    fixture.dataset.testid = "river-ledger-card-containment-fixture";
    fixture.style.display = "grid";
    fixture.style.gridTemplateColumns = "repeat(3, minmax(58px, 1fr))";
    fixture.style.gap = "6px";
    fixture.style.width = "320px";
    fixture.style.maxWidth = "100%";
    fixture.style.padding = "4px";
    const suits = [
      ["clubs", "\u2663"],
      ["diamonds", "\u2666"],
      ["hearts", "\u2665"],
      ["spades", "\u2660"],
    ];
    for (const tone of ["board", "private", "showdown"]) {
      for (const [suit, glyph] of suits) {
        const card = document.createElement("div");
        card.className = `river-ledger-card ${tone} suit-${suit}`;
        card.setAttribute("aria-label", `Ace of ${suit}`);
        card.innerHTML = `<strong>AD</strong><span class="river-ledger-card-suit"><b aria-hidden="true">${glyph}</b><small>${suit}</small></span><span class="river-ledger-card-rank">ace</span>`;
        fixture.append(card);
      }
    }
    document.body.append(fixture);
  });

  const fixtureFailures = await riverLedgerCardOverflowFailures(
    page,
    "[data-testid='river-ledger-card-containment-fixture'] .river-ledger-card",
  );
  assert(fixtureFailures.length === 0, `${label} all-suit fixture stays contained: ${JSON.stringify(fixtureFailures)}`);

  const originalViewport = page.viewport();
  await page.setViewport({ width: 320, height: 820 });
  await page.evaluate(() => {
    document.documentElement.dataset.riverLedgerOriginalFontSize = document.documentElement.style.fontSize;
    document.documentElement.style.fontSize = "200%";
  });
  const zoomFailures = await riverLedgerCardOverflowFailures(
    page,
    "[data-testid='river-ledger-card-containment-fixture'] .river-ledger-card",
  );
  assert(
    zoomFailures.length === 0,
    `${label} all-suit fixture stays contained at 200% text / 320px: ${JSON.stringify(zoomFailures)}`,
  );
  await page.evaluate(() => {
    document.documentElement.style.fontSize = document.documentElement.dataset.riverLedgerOriginalFontSize ?? "";
    delete document.documentElement.dataset.riverLedgerOriginalFontSize;
    document.querySelector("[data-testid='river-ledger-card-containment-fixture']")?.remove();
  });
  if (originalViewport) {
    await page.setViewport(originalViewport);
  }
}

async function riverLedgerCardOverflowFailures(page, selector) {
  return page.$$eval(selector, (cards) => {
    const tolerance = 0.75;
    const failures = [];
    for (const card of cards) {
      const cardRect = card.getBoundingClientRect();
      if (cardRect.width <= 0 || cardRect.height <= 0) {
        continue;
      }
      const children = Array.from(
        card.querySelectorAll(
          "strong, .river-ledger-card-suit, .river-ledger-card-suit b, .river-ledger-card-suit small, .river-ledger-card-rank",
        ),
      );
      for (const child of children) {
        const rect = child.getBoundingClientRect();
        const text = child.textContent?.trim() ?? child.className;
        if (rect.width <= 0 || rect.height <= 0) {
          failures.push({ card: card.getAttribute("aria-label") ?? card.textContent, child: text, reason: "not visible" });
          continue;
        }
        if (rect.left < cardRect.left - tolerance || rect.right > cardRect.right + tolerance) {
          failures.push({
            card: card.getAttribute("aria-label") ?? card.textContent,
            child: text,
            reason: "inline overflow",
            cardLeft: cardRect.left,
            cardRight: cardRect.right,
            childLeft: rect.left,
            childRight: rect.right,
          });
        }
      }
    }
    return failures;
  });
}

async function assertHandRankingReferenceDuringPlay(page) {
  const summary = await page.evaluate(() => {
    const details = document.querySelector(".river-ledger-hand-rankings details");
    return {
      exists: Boolean(details),
      open: details?.hasAttribute("open") ?? false,
      rows: details?.querySelectorAll("li").length ?? 0,
      firstLabel: details?.querySelector("li strong")?.textContent ?? "",
      text: details?.textContent ?? "",
    };
  });
  assert(summary.exists, "hand ranking reference is reachable during play");
  assert(!summary.open, "hand ranking reference starts collapsed during play");
  assert(summary.rows === 9, `hand ranking reference has nine rows: ${summary.rows}`);
  assert(summary.firstLabel === "Straight flush", `hand ranking reference uses Rust order: ${summary.firstLabel}`);
  assert(summary.text.includes("High card"), `hand ranking reference includes low category: ${summary.text}`);
}

async function assertHandRankingReferenceAfterShowdown(page) {
  const summary = await page.evaluate(() => {
    const details = document.querySelector(".river-ledger-hand-rankings details");
    const current = details?.querySelector("li.current");
    return {
      exists: Boolean(details),
      open: details?.hasAttribute("open") ?? false,
      rows: details?.querySelectorAll("li").length ?? 0,
      currentText: current?.textContent ?? "",
      currentAria: current?.getAttribute("aria-current") ?? "",
    };
  });
  assert(summary.exists, "hand ranking reference remains reachable after showdown");
  assert(summary.open, "hand ranking reference is default-visible after showdown");
  assert(summary.rows === 9, `post-showdown hand ranking reference has nine rows: ${summary.rows}`);
  assert(summary.currentText.includes("One pair"), `winning category is marked from showdown category: ${summary.currentText}`);
  assert(summary.currentAria === "true", `winning category exposes aria-current: ${summary.currentAria}`);
}

async function assertTeachingAidAfterShowdown(page) {
  const summary = await page.evaluate(() => {
    const aid = document.querySelector(".river-ledger-teaching-aid");
    return {
      exists: Boolean(aid),
      label: aid?.querySelector("span")?.textContent ?? "",
      text: aid?.textContent ?? "",
    };
  });
  assert(summary.exists, "terminal teaching aid renders after showdown");
  assert(summary.label === "Teaching aid, not a game value", `teaching aid has non-canonical label: ${summary.label}`);
  assert(
    summary.text.includes("One pair is category 8 of 9 from strongest to weakest."),
    `teaching aid renders Rust ladder position: ${summary.text}`,
  );
}

async function assertActionPanelCostCopy(page) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll('[data-testid^="choice-river-ledger-"]'));
    return buttons.map((button) => button.textContent ?? "");
  });
  assert(
    summary.some((text) => text.includes("Call") && text.includes("Call price 2") && text.includes("Adds 2")),
    `action panel renders call price and adds-to-ledger: ${summary.join(" | ")}`,
  );
  assert(
    summary.some((text) => text.includes("Fold") && !text.includes("Call price") && !text.includes("Raises left")),
    `fold action omits irrelevant call/cap rows: ${summary.join(" | ")}`,
  );
  assert(
    summary.some((text) => text.includes("Raise") && text.includes("Raises left 3")),
    `action panel renders Rust raises remaining for raise choices: ${summary.join(" | ")}`,
  );
}

async function assertSeatAndStreetAffordancesDuringPlay(page) {
  const summary = await page.evaluate(() => {
    const seats = Array.from(document.querySelectorAll(".river-ledger-seat"));
    const currentStreet = document.querySelector(".river-ledger-street-strip li.current");
    return {
      activeMarker: seats.some((seat) => seat.textContent?.includes("Active")),
      buttonMarker: seats.some((seat) => seat.textContent?.includes("Button")),
      smallBlindMarker: seats.some((seat) => seat.textContent?.includes("Small blind")),
      bigBlindMarker: seats.some((seat) => seat.textContent?.includes("Big blind")),
      ledgerLabels: seats.map((seat) => seat.textContent ?? ""),
      trackBars: document.querySelectorAll(".river-ledger-track-bar").length,
      currentStreetText: currentStreet?.textContent ?? "",
      currentStreetAria: currentStreet?.getAttribute("aria-current") ?? "",
      streetRows: document.querySelectorAll(".river-ledger-street-strip li").length,
    };
  });
  assert(summary.activeMarker, "seat affordances include active text");
  assert(summary.buttonMarker, "seat affordances include button text");
  assert(summary.smallBlindMarker, "seat affordances include small blind text");
  assert(summary.bigBlindMarker, "seat affordances include big blind text");
  assert(
    summary.ledgerLabels.every(
      (text) => text.includes("This round") && text.includes("Hand total") && text.includes("Hole cards") && text.includes("2 hidden"),
    ),
    `seat ledgers render Rust-authored labels: ${summary.ledgerLabels.join(" | ")}`,
  );
  assert(summary.trackBars === 0, "seat ledger removes duplicate contribution track bar");
  assert(summary.streetRows === 5, `street strip renders five steps: ${summary.streetRows}`);
  assert(summary.currentStreetText.includes("Preflop"), `street strip marks preflop from public state: ${summary.currentStreetText}`);
  assert(summary.currentStreetAria === "step", `current street exposes aria-current step: ${summary.currentStreetAria}`);
}

async function assertModeStatusUsesSeatLabel(page) {
  const modeText = await page.$eval(".mode-controls", (element) => element.textContent ?? "");
  assert(modeText.includes("Seat 1 (you) to act"), `mode status uses Rust seat labels: ${modeText}`);
  assert(!modeText.includes("Player 1 to act"), `mode status avoids Player fallback: ${modeText}`);
}

async function assertStreetStripAfterShowdown(page) {
  const summary = await page.evaluate(() => {
    const currentStreet = document.querySelector(".river-ledger-street-strip li.current");
    return {
      currentStreetText: currentStreet?.textContent ?? "",
      currentStreetAria: currentStreet?.getAttribute("aria-current") ?? "",
      completeRows: document.querySelectorAll(".river-ledger-street-strip li.complete").length,
    };
  });
  assert(summary.currentStreetText.includes("Showdown"), `street strip marks showdown after terminal: ${summary.currentStreetText}`);
  assert(summary.currentStreetAria === "step", `terminal street exposes aria-current step: ${summary.currentStreetAria}`);
  assert(summary.completeRows === 4, `street strip marks prior streets complete: ${summary.completeRows}`);
}

async function assertFoldedSeatNoStrength(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    const foldedRows = Array.from(panel?.querySelectorAll(".river-ledger-showdown-folded p, .outcome-standing-row") ?? []).filter(
      (row) => /folded/i.test(row.textContent ?? ""),
    );
    return {
      text: panel?.textContent ?? "",
      foldedRows: foldedRows.length,
      foldedStrengthRows: foldedRows.filter((row) => /Pair|High Card|tie break/i.test(row.textContent ?? "")).length,
    };
  });
  assert(summary.foldedRows >= 1, `terminal path includes a folded seat row: ${summary.text}`);
  assert(summary.foldedStrengthRows === 0, `folded terminal rows contain no hand strength: ${summary.text}`);
}

async function setSetupSeed(page, seed) {
  const selector = '.setup-grid input[type="number"]';
  await page.$eval(
    selector,
    (input, value) => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
      setter?.call(input, String(value));
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    seed,
  );
  await page.waitForFunction(
    (query, expected) => document.querySelector(query)?.value === String(expected),
    {},
    selector,
    seed,
  );
}

async function configureRiverLedgerStacks(page, setup) {
  await page.waitForSelector(".river-stack-setup");
  if (setup.mode === "short_stack") {
    await clickText(page, ".river-stack-mode label", "Short-stack");
    return;
  }
  if (setup.mode === "custom") {
    await clickText(page, ".river-stack-mode label", "Custom");
    await page.waitForFunction(() =>
      Array.from(document.querySelectorAll(".river-stack-field input")).every((input) => !input.disabled),
    );
    for (let index = 0; index < setup.stacks.length; index += 1) {
      await setRiverLedgerStackInput(page, index, setup.stacks[index]);
    }
  }
}

async function setRiverLedgerStackInput(page, index, value) {
  await page.$eval(
    `.river-stack-field:nth-of-type(${index + 1}) input`,
    (input, nextValue) => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
      setter?.call(input, String(nextValue));
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    value,
  );
  await page.waitForFunction(
    (selector, expected) => document.querySelector(selector)?.value === String(expected),
    {},
    `.river-stack-field:nth-of-type(${index + 1}) input`,
    value,
  );
}

async function hasRiverLedgerOutcome(page) {
  return page.evaluate(() => Boolean(document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]')));
}

async function assertRiverLedgerA11y(page, expectChoices, expectedSeatCount) {
  const summary = await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll("button"));
    return {
      board: Boolean(document.querySelector('[data-testid="river-ledger-board"]')),
      seats: document.querySelectorAll(".river-ledger-seat").length,
      pendingSlots: Array.from(document.querySelectorAll(".river-ledger-board-cards .river-ledger-card.hidden")).map((slot) => ({
        text: slot.textContent ?? "",
        aria: slot.getAttribute("aria-label") ?? "",
      })),
      choices: document.querySelectorAll('[data-testid^="choice-river-ledger-"]').length,
      unnamed: buttons
        .filter((button) => !((button.getAttribute("aria-label") || button.textContent || "").trim()))
        .map((button) => button.getAttribute("data-testid") ?? button.className),
      liveText: document.querySelector(".river-ledger-latest")?.textContent ?? "",
    };
  });
  assert(summary.board, "river_ledger board renders");
  assert(
    summary.seats === expectedSeatCount,
    `river_ledger renders ${expectedSeatCount} selected seat rows, got ${summary.seats}`,
  );
  assert(summary.pendingSlots.length === 5, `river_ledger renders five pending board slots: ${summary.pendingSlots.length}`);
  assert(
    summary.pendingSlots.some((slot) => slot.text.includes("Flop 1 pending") && slot.aria.includes("Unrevealed Flop 1 card")),
    `river_ledger pending slots use Rust-authored street labels: ${JSON.stringify(summary.pendingSlots)}`,
  );
  assert(
    summary.pendingSlots.every((slot) => !cardIds.some((cardId) => slot.text.includes(cardId) || slot.aria.includes(cardId))),
    "river_ledger pending slots omit future card ids",
  );
  if (expectChoices) {
    assert(summary.choices > 0, "river_ledger exposes Rust-provided action buttons");
  }
  assert(summary.unnamed.length === 0, `buttons have accessible names: ${summary.unnamed.join(", ")}`);
  assert(summary.liveText.length > 0, "river_ledger latest-effect region has text");
}

async function assertFixedSeatSetup(page) {
  const summary = await page.evaluate(() => {
    const setup = document.querySelector(".setup-region");
    const seatSelect = setup?.querySelector('select[aria-label="Supported seats from Rust catalog"]');
    const seatOutput = setup?.querySelector('.seat-count-static[aria-label="Supported seats from Rust catalog"]');
    return {
      hasSeatSelect: Boolean(seatSelect),
      outputText: seatOutput?.textContent?.trim() ?? "",
      caption: seatOutput?.closest(".field")?.querySelector("small")?.textContent ?? "",
    };
  });
  assert(!summary.hasSeatSelect, "fixed-seat setup renders no seat select");
  assert(summary.outputText === "2 seats", `fixed-seat setup shows static two-seat value: ${summary.outputText}`);
  assert(summary.caption.includes("Fixed at 2 seats."), `fixed-seat caption is read-only: ${summary.caption}`);
}

async function assertVariableSeatSetup(page, seatCount) {
  await page.waitForSelector('select[aria-label="Supported seats from Rust catalog"]');
  const summary = await page.evaluate(() => {
    const select = document.querySelector('select[aria-label="Supported seats from Rust catalog"]');
    return {
      disabled: select?.disabled ?? true,
      options: Array.from(select?.querySelectorAll("option") ?? []).map((option) => option.value),
    };
  });
  assert(!summary.disabled, "variable-seat setup select is enabled");
  assert(
    ["3", "4", "5", "6"].every((value) => summary.options.includes(value)),
    `river_ledger setup exposes supported seat counts: ${summary.options.join(", ")}`,
  );
  await page.select('select[aria-label="Supported seats from Rust catalog"]', String(seatCount));
  await page.waitForFunction(
    (expected) => document.querySelector('select[aria-label="Supported seats from Rust catalog"]')?.value === String(expected),
    {},
    seatCount,
  );
  const rows = await page.$$eval(".seat-roles > div", (items) => items.map((row) => row.textContent ?? ""));
  assert(rows.length === seatCount, `setup role rows match selected seat count ${seatCount}: ${rows.join(" | ")}`);
  assert(
    rows.every((row, index) => row.includes(`Seat ${index + 1}`)),
    `setup role rows use selected active labels: ${rows.join(" | ")}`,
  );
  const setupCopy = await page.$eval(".setup-region", (region) => region.textContent ?? "");
  assert(setupCopy.includes(`Seat ${seatCount}`), `setup copy names selected final seat: ${setupCopy}`);
  assert(!setupCopy.includes(`Seat ${seatCount + 1}`), `setup copy omits phantom next seat: ${setupCopy}`);
}

async function ownPrivateCardLabels(page) {
  return page.$$eval(".river-ledger-private .river-ledger-card.private strong", (cards) =>
    cards.map((card) => card.textContent?.trim() ?? "").filter(Boolean),
  );
}

async function assertNoLeak(page, consoleMessages, label, forbiddenCardIds) {
  await assertNoRawSeatIds(page, label);
  const surface = await fullBrowserSurface(page);
  assertNoForbiddenTerms(surface, label, [...forbiddenCardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), `${label} console`, internalTerms);
}

async function assertRawSeatIdGuardTrips(page, baseUrl) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await page.evaluate(() => {
    const probe = document.createElement("button");
    probe.type = "button";
    probe.dataset.testid = "river-ledger-raw-seat-probe";
    probe.setAttribute("aria-label", "seat_5 should fail the River Ledger copy guard");
    probe.textContent = "seat_5 should fail the River Ledger copy guard";
    document.querySelector("main")?.append(probe);
  });
  const hits = await collectRawSeatIdHits(page);
  await page.evaluate(() => document.querySelector("[data-testid='river-ledger-raw-seat-probe']")?.remove());
  assert(
    hits.some((hit) => hit.token === "seat_5"),
    `River Ledger raw-seat guard did not catch induced drift: ${JSON.stringify(hits)}`,
  );
}

async function assertNoRawSeatIds(page, label) {
  const hits = await collectRawSeatIdHits(page);
  assert(hits.length === 0, `${label} contains raw seat ids in visible/a11y/test-id surface: ${JSON.stringify(hits)}`);
}

async function collectRawSeatIdHits(page) {
  const samples = await page.evaluate(() => {
    const excludedSelector = ".dev-panel, .replay-io, .replay-viewer, script, style";
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
      for (const name of ["aria-label", "title", "alt", "aria-valuetext", "data-testid"]) {
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

async function fullBrowserSurface(page) {
  return page.evaluate(() =>
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

async function clickSeatFrameButton(page, text) {
  const handle = await waitForTextHandle(page, ".seat-frame-viewers label", text);
  await handle.click();
}

async function clickRiverAction(page, label) {
  await page.waitForFunction(
    (expected) =>
      Array.from(document.querySelectorAll('[data-testid^="choice-river-ledger-"]')).some(
        (button) => !button.disabled && button.querySelector("strong")?.textContent?.trim() === expected,
      ),
    {},
    label,
  );
  const handles = await page.$$('[data-testid^="choice-river-ledger-"]');
  for (const handle of handles) {
    const match = await handle.evaluate(
      (button, expected) => !button.disabled && button.querySelector("strong")?.textContent?.trim() === expected,
      label,
    );
    if (match) {
      await handle.click();
      return;
    }
  }
  throw new Error(`No River Ledger action labeled ${label}`);
}

async function hasEnabledRiverAction(page, label) {
  return page.evaluate((expected) =>
    Array.from(document.querySelectorAll('[data-testid^="choice-river-ledger-"]')).some(
      (button) => !button.disabled && button.querySelector("strong")?.textContent?.trim() === expected,
    ),
  label);
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

function labelToId(label) {
  const match = label.match(/^([2-9]|10|J|Q|K|A)([CDHS])$/);
  if (!match) return label.toLowerCase();
  const rank = {
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    J: "jack",
    Q: "queen",
    K: "king",
    A: "ace",
  }[match[1]];
  const suit = { C: "clubs", D: "diamonds", H: "hearts", S: "spades" }[match[2]];
  return `${rank}_${suit}`;
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
