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
  await waitForText(page, "Seat 0 to choose");
  await assertRiverLedgerA11y(page, true, selectedSeatCount);
  await assertHandRankingReferenceDuringPlay(page);
  await assertSeatAndStreetAffordancesDuringPlay(page);
  const seat0Cards = await ownPrivateCardLabels(page);
  assert(seat0Cards.length === 2, "seat 0 private view exposes two own cards");
  await assertPrivateCardComponent(page);
  await assertNoLeak(page, consoleMessages, "seat 0 view", cardIds.filter((id) => !seat0Cards.map(labelToId).includes(id)));

  await clickSeatFrameButton(page, "Observer");
  await waitForText(page, "Observer view");
  await assertNoLeak(page, consoleMessages, "observer view", cardIds);
  await assertStorageClean(page);

  await clickSeatFrameButton(page, "Seat 1");
  await waitForText(page, "Seat 1 view");
  const seat1Cards = await ownPrivateCardLabels(page);
  assert(seat1Cards.length === 2, "seat 1 private view exposes two own cards");
  await assertNoLeak(page, consoleMessages, "wrong-seat view", seat0Cards.map(labelToId));

  await clickSeatFrameButton(page, "Seat 0");
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
  await assertHandRankingReferenceAfterShowdown(page);
  await assertTeachingAidAfterShowdown(page);
  await assertStreetStripAfterShowdown(page);
  assertNoForbiddenTerms(await fullBrowserSurface(page), "worked-example showdown surface", [...cardIds, ...internalTerms]);
  assertNoForbiddenTerms(consoleMessages.join("\n"), "worked-example console logs", internalTerms);
  await assertStorageClean(page);

  console.log(JSON.stringify({ browser: "puppeteer", smoke: "river_ledger noleak legal controls terminal responsive" }));
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}

async function startRiverLedger(page, baseUrl, modeLabel, seatCount, seed = null) {
  await page.goto(baseUrl, { waitUntil: "networkidle0" });
  await waitForText(page, "River Ledger");
  await assertFixedSeatSetup(page);
  await clickText(page, "button", "River Ledger");
  await assertVariableSeatSetup(page, seatCount);
  if (seed !== null) {
    await setSetupSeed(page, seed);
  }
  await clickLabel(page, modeLabel);
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

async function assertWorkedExampleShowdown(page) {
  const summary = await page.evaluate(() => {
    const panel = document.querySelector('.outcome-explanation-panel[data-outcome-game="river_ledger"]');
    const showdown = panel?.querySelector(".river-ledger-showdown-panel");
    return {
      text: panel?.textContent ?? "",
      showdownText: showdown?.textContent ?? "",
      handCount: showdown?.querySelectorAll(".river-ledger-showdown-hand").length ?? 0,
      cardCount: showdown?.querySelectorAll(".river-ledger-showdown-card").length ?? 0,
      foldedStrengthRows: Array.from(panel?.querySelectorAll(".outcome-standing-row") ?? [])
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
  assert(summary.cardCount === 20, `worked example renders each best-five hand: ${summary.cardCount}`);
  assert(summary.foldedStrengthRows === 0, "worked example renders no folded-seat hand strength");
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
    const boardGroup = document.querySelector(".river-ledger-board-cards");
    const showdownGroups = Array.from(document.querySelectorAll(".river-ledger-showdown-cards"));
    return {
      boardCards: boardCards.length,
      boardGlyphs: boardCards.filter((card) => card.querySelector(".river-ledger-card-suit b")?.textContent?.trim()).length,
      showdownCards: showdownCards.length,
      showdownGlyphs: showdownCards.filter((card) => card.querySelector(".river-ledger-card-suit b")?.textContent?.trim()).length,
      showdownSuitWords: showdownCards.filter((card) => card.querySelector(".river-ledger-card-suit small")?.textContent?.trim()).length,
      boardGroupLabel: boardGroup?.getAttribute("aria-label") ?? "",
      showdownGroupLabels: showdownGroups.map((group) => group.getAttribute("aria-label") ?? ""),
    };
  });
  assert(summary.boardCards === 5, `terminal board uses card component: ${summary.boardCards}`);
  assert(summary.boardGlyphs === 5, `terminal board renders suit glyphs: ${summary.boardGlyphs}`);
  assert(summary.showdownCards === 20, `showdown best-five uses card component: ${summary.showdownCards}`);
  assert(summary.showdownGlyphs === 20, `showdown best-five renders suit glyphs: ${summary.showdownGlyphs}`);
  assert(summary.showdownSuitWords === 20, `showdown best-five renders suit words: ${summary.showdownSuitWords}`);
  assert(summary.boardGroupLabel.includes("Public board cards"), `board group has accessible label: ${summary.boardGroupLabel}`);
  assert(
    summary.showdownGroupLabels.length === 4 && summary.showdownGroupLabels.every((label) => label.includes("Best five")),
    `showdown groups have accessible labels: ${summary.showdownGroupLabels.join(" | ")}`,
  );
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
      markerIcons: Array.from(document.querySelectorAll(".river-ledger-markers b span")).map((icon) => icon.textContent ?? ""),
      currentStreetText: currentStreet?.textContent ?? "",
      currentStreetAria: currentStreet?.getAttribute("aria-current") ?? "",
      streetRows: document.querySelectorAll(".river-ledger-street-strip li").length,
    };
  });
  assert(summary.activeMarker, "seat affordances include active text");
  assert(summary.buttonMarker, "seat affordances include button text");
  assert(summary.smallBlindMarker, "seat affordances include small blind text");
  assert(summary.bigBlindMarker, "seat affordances include big blind text");
  assert(summary.markerIcons.length >= 4, `seat affordances include icons: ${summary.markerIcons.join(", ")}`);
  assert(summary.streetRows === 5, `street strip renders five steps: ${summary.streetRows}`);
  assert(summary.currentStreetText.includes("Preflop"), `street strip marks preflop from public state: ${summary.currentStreetText}`);
  assert(summary.currentStreetAria === "step", `current street exposes aria-current step: ${summary.currentStreetAria}`);
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
    const foldedRows = Array.from(panel?.querySelectorAll(".outcome-standing-row") ?? []).filter((row) =>
      /folded/i.test(row.textContent ?? ""),
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
  const handle = await waitForTextHandle(page, ".seat-frame-viewers button", text);
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
