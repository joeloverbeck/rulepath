import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const artifactPath = join(__dirname, "..", "public", "wasm_api.wasm");
const bytes = await readFile(artifactPath);
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

function invoke(call, values) {
  const args = values.map(write);
  try {
    const status = call(args);
    const raw = output();
    const parsed = JSON.parse(raw);
    if (status !== 0) {
      const error = new Error(parsed.message);
      error.diagnostic = parsed;
      throw error;
    }
    return parsed;
  } finally {
    for (const arg of args) {
      wasm.rulepath_dealloc(arg.ptr, arg.len);
    }
  }
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function hasVariant(game, id, label) {
  return game.variants?.some((variant) => variant.id === id && variant.label === label);
}

function variantById(game, id) {
  return game.variants?.find((variant) => variant.id === id);
}

function assertVariantDescription(game, id, expected) {
  const variant = variantById(game, id);
  assert(variant, `${game.game_id} includes variant ${id}`);
  const behaviorToken = /\b(if|when|then|selector|trigger|valid_if|legal|effect|action)\b/i;
  if (expected === undefined) {
    assert(!Object.prototype.hasOwnProperty.call(variant, "description"), `${id} omits description when absent`);
    return;
  }
  assert(typeof variant.description === "string", `${id} description is a string`);
  assert(variant.description.trim().length > 0, `${id} description is non-empty`);
  assert(variant.description.length <= 120, `${id} description is <=120 characters`);
  assert(!behaviorToken.test(variant.description), `${id} description contains no behavior-looking token`);
  assert(variant.description === expected, `${id} description matches Rust-authored catalog copy`);
}

const version = read(
  wasm.rulepath_placeholder_version_ptr(),
  wasm.rulepath_placeholder_version_len(),
);
assert(version === "rulepath-wasm-api/0.1.0", "wasm artifact loads");

const created = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 1n),
  ["race_to_n"],
);
assert(created.match_id, "start match returns a match id");

const initialView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(initialView.counter === 0, "initial public view is visible");
assert(initialView.active_seat === "seat_0", "human seat starts");

const tree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [created.match_id, "seat_0"],
);
assert(tree.choices.some((choice) => choice.segment === "add-1"), "Rust choices are displayed");

const afterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      0n,
    ),
  [created.match_id, "seat_0", "add-1"],
);
assert(afterHuman.view.counter > 0, "human action advances the counter");

const afterBot = invoke(
  (args) =>
    wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
  [created.match_id, "seat_1"],
);
assert(afterBot.view.active_seat === "seat_0" || afterBot.view.winner, "bot turn resolves");

const effects = invoke(
  (args) => wasm.rulepath_get_effects(args[0].ptr, args[0].len, 0n, 0, 0),
  [created.match_id],
);
assert(effects.length > 0, "semantic effects are available");

const exportedReplay = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [created.match_id],
);
assert(exportedReplay.commands.length > 0, "run exports a replay command stream");

const importedReplay = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(exportedReplay)],
);
assert(importedReplay.replay_id, "replay import returns a replay handle");
const replayReset = invoke(
  (args) => wasm.rulepath_replay_reset(args[0].ptr, args[0].len),
  [importedReplay.replay_id],
);
assert(replayReset.cursor === 0, "replay reset returns cursor zero");
const replayStep = invoke(
  (args) => wasm.rulepath_replay_step(args[0].ptr, args[0].len, 1),
  [importedReplay.replay_id],
);
assert(replayStep.cursor === 1 && replayStep.view.counter > 0, "replay step returns Rust-projected view");

let staleDiagnostic = null;
try {
  invoke(
    (args) =>
      wasm.rulepath_apply_action(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
        0n,
      ),
    [created.match_id, "seat_0", "add-1"],
  );
} catch (error) {
  staleDiagnostic = error.diagnostic;
}
assert(staleDiagnostic?.code === "stale_action", "stale submission returns Rust diagnostic");

const catalog = invoke(() => wasm.rulepath_list_games(), []);
assert(catalog.some((game) => game.game_id === "race_to_n"), "Rust catalog includes race_to_n");
assert(catalog.some((game) => game.game_id === "three_marks"), "Rust catalog includes three_marks");
const raceCatalog = catalog.find((game) => game.game_id === "race_to_n");
assert(raceCatalog, "Rust catalog includes race_to_n entry");
assert(
  JSON.stringify(raceCatalog.seat_labels) === JSON.stringify([
    { seat: "seat_0", label: "Player 1" },
    { seat: "seat_1", label: "Player 2" },
  ]),
  "Rust catalog exposes Race player labels",
);
const twoSeatGames = catalog.filter((game) => !["river_ledger", "briar_circuit", "vow_tide", "blackglass_pact"].includes(game.game_id));
assert(
  twoSeatGames.every(
    (game) =>
      game.min_seats === 2 &&
      game.max_seats === 2 &&
      game.default_seats === 2 &&
      Array.isArray(game.supported_seats) &&
      game.supported_seats.length === 1 &&
      game.supported_seats[0] === 2 &&
      Array.isArray(game.seat_labels) &&
      game.seat_labels.length === 2,
  ),
  "Rust catalog exposes two-seat setup metadata for current two-seat games",
);
const blackglassCatalog = catalog.find((game) => game.game_id === "blackglass_pact");
assert(blackglassCatalog, "Rust catalog includes blackglass_pact");
assert(
  blackglassCatalog.min_seats === 4 &&
    blackglassCatalog.max_seats === 4 &&
    blackglassCatalog.default_seats === 4 &&
    JSON.stringify(blackglassCatalog.supported_seats) === JSON.stringify([4]) &&
    Array.isArray(blackglassCatalog.seat_labels) &&
    blackglassCatalog.seat_labels.length === 4,
  "Rust catalog exposes Blackglass Pact fixed-four setup metadata",
);
const tokenBazaarCatalog = catalog.find((game) => game.game_id === "token_bazaar");
assert(tokenBazaarCatalog, "Rust catalog includes token_bazaar");
assertVariantDescription(tokenBazaarCatalog, "token_bazaar_standard", undefined);
const riverLedgerCatalog = catalog.find((game) => game.game_id === "river_ledger");
assert(riverLedgerCatalog, "Rust catalog includes river_ledger");
assertVariantDescription(riverLedgerCatalog, "river_ledger_standard", undefined);
assert(
  riverLedgerCatalog.hidden_information === true &&
    riverLedgerCatalog.min_seats === 3 &&
    riverLedgerCatalog.max_seats === 6 &&
    riverLedgerCatalog.default_seats === 6 &&
    JSON.stringify(riverLedgerCatalog.supported_seats) === JSON.stringify([3, 4, 5, 6]) &&
    riverLedgerCatalog.viewer_modes.includes("seat_5") &&
    riverLedgerCatalog.seat_labels.length === 6,
  "Rust catalog exposes river_ledger 3-6 seat metadata",
);
const briarCircuitCatalog = catalog.find((game) => game.game_id === "briar_circuit");
assert(briarCircuitCatalog, "Rust catalog includes briar_circuit");
assertVariantDescription(briarCircuitCatalog, "briar_circuit_standard", undefined);
assert(
  briarCircuitCatalog.hidden_information === true &&
    briarCircuitCatalog.min_seats === 4 &&
    briarCircuitCatalog.max_seats === 4 &&
    briarCircuitCatalog.default_seats === 4 &&
    JSON.stringify(briarCircuitCatalog.supported_seats) === JSON.stringify([4]) &&
    briarCircuitCatalog.viewer_modes.includes("seat_3") &&
    briarCircuitCatalog.seat_labels.length === 4,
  "Rust catalog exposes briar_circuit fixed-four metadata",
);
assert(
  JSON.stringify(briarCircuitCatalog.seat_labels) === JSON.stringify([
    { seat: "seat_0", label: "Seat 1" },
    { seat: "seat_1", label: "Seat 2" },
    { seat: "seat_2", label: "Seat 3" },
    { seat: "seat_3", label: "Seat 4" },
  ]),
  "Rust catalog exposes Briar Circuit one-based seat labels",
);
const vowTideCatalog = catalog.find((game) => game.game_id === "vow_tide");
assert(vowTideCatalog, "Rust catalog includes vow_tide");
assertVariantDescription(vowTideCatalog, "vow_tide_standard", undefined);
assert(
  vowTideCatalog.hidden_information === true &&
    vowTideCatalog.min_seats === 3 &&
    vowTideCatalog.max_seats === 7 &&
    vowTideCatalog.default_seats === 4 &&
    JSON.stringify(vowTideCatalog.supported_seats) === JSON.stringify([3, 4, 5, 6, 7]) &&
    vowTideCatalog.viewer_modes.includes("seat_6") &&
    vowTideCatalog.seat_labels.length === 7,
  "Rust catalog exposes vow_tide 3-7 seat metadata",
);
const floodWatchCatalog = catalog.find((game) => game.game_id === "flood_watch");
assert(floodWatchCatalog, "Rust catalog includes flood_watch");
assertVariantDescription(
  floodWatchCatalog,
  "flood_watch_standard",
  "Cooperative flood planning with steady pressure and full district visibility.",
);
assert(
  catalog.some(
    (game) => game.game_id === "token_bazaar" && hasVariant(game, "token_bazaar_standard", "Token Bazaar"),
  ),
  "Rust catalog includes token_bazaar standard variant",
);
assert(
  catalog.some(
    (game) => game.game_id === "plain_tricks" && hasVariant(game, "plain_tricks_standard", "Plain Tricks"),
  ),
  "Rust catalog includes plain_tricks standard variant",
);
assert(
  catalog.some(
    (game) => game.game_id === "masked_claims" && hasVariant(game, "masked_claims_standard", "Masked Claims"),
  ),
  "Rust catalog includes masked_claims standard variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "flood_watch" &&
      hasVariant(game, "flood_watch_standard", "Flood Watch") &&
      game.hidden_information === true &&
      game.cooperative === true,
  ),
  "Rust catalog includes cooperative hidden-info flood_watch standard variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "frontier_control" &&
      hasVariant(game, "frontier_control_standard", "Frontier Control") &&
      game.hidden_information === false,
  ),
  "Rust catalog includes frontier_control standard perfect-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "event_frontier" &&
      hasVariant(game, "event_frontier_standard", "Event Frontier") &&
      game.hidden_information === true &&
      game.tags.includes("event_deck"),
  ),
  "Rust catalog includes event_frontier standard hidden-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "river_ledger" &&
      hasVariant(game, "river_ledger_standard", "River Ledger") &&
      game.hidden_information === true &&
      game.tags.includes("public_accounting"),
  ),
  "Rust catalog includes river_ledger standard hidden-information variant",
);
assert(
  catalog.some(
    (game) =>
      game.game_id === "vow_tide" &&
      hasVariant(game, "vow_tide_standard", "Vow Tide") &&
      game.hidden_information === true &&
      game.tags.includes("trick_taking") &&
      game.tags.includes("bidding"),
  ),
  "Rust catalog includes vow_tide standard hidden-information variant",
);

const riverLedger = invoke(
  (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, 61n, 6),
  ["river_ledger"],
);
assert(riverLedger.match_id, "river_ledger 6-seat start match returns a match id");
assert(riverLedger.variant_id === "river_ledger_standard", "river_ledger standard variant starts");
const riverObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [riverLedger.match_id],
);
assert(riverObserver.game_id === "river_ledger", "river_ledger Rust view is returned");
assert(riverObserver.seats.length === 6, "river_ledger Rust view exposes six seat ledger rows");
assert(riverObserver.board.length === 0, "river_ledger setup exposes no public board cards");
assert(!("view_summary" in riverObserver), "river_ledger view uses structured browser payload");
assert(!("deck_tail_count" in riverObserver), "river_ledger view omits deck-tail internals");
for (const seatCount of [3, 4, 5, 6]) {
  const match = invoke(
    (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, BigInt(70 + seatCount), seatCount),
    ["river_ledger"],
  );
  const view = invoke((args) => wasm.rulepath_get_view(args[0].ptr, args[0].len), [match.match_id]);
  assert(view.seats.length === seatCount, `river_ledger ${seatCount}-seat view exposes compact rail seats`);
  assert(view.board_slots.length === 5, `river_ledger ${seatCount}-seat view exposes central board well slots`);
}
const riverSeat3 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [riverLedger.match_id, "seat_3"],
);
assert(riverSeat3.private_view.status === "seat", "river_ledger seat viewer receives own private view");
assert(riverSeat3.private_view.hole_cards.length === 2, "river_ledger seat viewer receives two own private cards");
const riverTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [riverLedger.match_id, riverObserver.active_seat, riverObserver.active_seat],
);
assert(riverTree.choices.length > 0, "river_ledger active seat exposes legal actions");
const riverBot = invoke(
  (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 88n),
  [riverLedger.match_id, riverObserver.active_seat],
);
assert(riverBot.bot_explanation, "river_ledger bot turn returns Rust-authored public explanation");
assert(riverBot.bot_explanation.seat_label.startsWith("Seat "), "river_ledger bot explanation uses public seat label");
assert(riverBot.bot_explanation.short_reason.endsWith("."), "river_ledger bot explanation carries one-sentence reason");
assert(
  riverBot.bot_explanation.public_facts.some((fact) => fact.label === "Call price"),
  "river_ledger bot explanation carries public fact rows",
);
assert(
  !JSON.stringify(riverBot.effects).includes("bot_explanation"),
  "river_ledger bot explanation is not routed through effect payloads",
);

const threeMarks = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 4n),
  ["three_marks"],
);
assert(threeMarks.match_id, "three_marks start match returns a match id");
assert(threeMarks.variant_id === "three_marks_standard", "three_marks standard variant starts");
const threeView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [threeMarks.match_id],
);
assert(threeView.game_id === "three_marks", "three_marks Rust view is returned");
assert(threeView.variant_id === "three_marks_standard", "three_marks Rust view reports selected variant");

const tokenBazaar = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 9n),
  ["token_bazaar"],
);
assert(tokenBazaar.match_id, "token_bazaar start match returns a match id");
assert(tokenBazaar.variant_id === "token_bazaar_standard", "token_bazaar standard variant starts");
const tokenView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [tokenBazaar.match_id],
);
assert(tokenView.game_id === "token_bazaar", "token_bazaar Rust view is returned");
assert(tokenView.market_slots.length === 3, "token_bazaar projects three market slots");
assert(tokenView.legal_actions.some((choice) => choice.action_segment === "collect/amber"), "token_bazaar view exposes legal actions");
const tokenTree = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [tokenBazaar.match_id, "seat_0"],
);
assert(tokenTree.choices.some((choice) => choice.segment === "collect/amber"), "token_bazaar action tree exposes collect/amber");
const tokenAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(tokenTree.freshness_token),
    ),
  [tokenBazaar.match_id, "seat_0", "collect/amber"],
);
assert(tokenAfterHuman.view.supply.amber < tokenView.supply.amber, "token_bazaar collect updates public accounting");
assert(
  tokenAfterHuman.effects.some((effect) => effect.payload.type === "resource_collected"),
  "token_bazaar emits resource accounting effect",
);

const plainTricks = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 11n),
  ["plain_tricks"],
);
assert(plainTricks.match_id, "plain_tricks start match returns a match id");
assert(plainTricks.variant_id === "plain_tricks_standard", "plain_tricks standard variant starts");
const plainObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [plainTricks.match_id],
);
assert(plainObserver.game_id === "plain_tricks", "plain_tricks Rust view is returned");
assert(plainObserver.private_view.status === "observer", "plain_tricks observer view is redacted");
const plainSeat0 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [plainTricks.match_id, "seat_0"],
);
assert(plainSeat0.private_view.own_hand.length === 6, "plain_tricks seat view exposes own hand");
const plainBlockedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [plainTricks.match_id, "seat_0", "seat_1"],
);
assert(plainBlockedTree.choices.length === 0, "plain_tricks non-actor tree is empty");
const plainTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [plainTricks.match_id, "seat_0", "seat_0"],
);
const plainPlay = plainTree.choices.find((choice) => choice.segment === "play");
const plainCard = plainPlay?.next?.choices?.[0];
assert(plainCard, "plain_tricks action tree exposes Rust card choices");
const plainAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(plainTree.freshness_token),
    ),
  [plainTricks.match_id, "seat_0", `play>${plainCard.segment}`],
);
assert(plainAfterHuman.view.current_trick.plays.length === 1, "plain_tricks card play updates current trick");
assert(
  plainAfterHuman.effects.some((effect) => effect.payload.type === "card_played"),
  "plain_tricks emits card-play effect",
);
const plainExport = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [plainTricks.match_id],
);
assert(plainExport.game_id === "plain_tricks", "plain_tricks replay export preserves game id");
assert(plainExport.export_class === "viewer_scoped_observation_v1", "plain_tricks replay export is viewer scoped");
const plainImport = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(plainExport)],
);
assert(plainImport.public_export === true, "plain_tricks public replay imports");

const briarCircuit = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 17n),
  ["briar_circuit"],
);
assert(briarCircuit.match_id, "briar_circuit start match returns a match id");
assert(briarCircuit.variant_id === "briar_circuit_standard", "briar_circuit standard variant starts");
const briarObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [briarCircuit.match_id],
);
assert(briarObserver.game_id === "briar_circuit", "briar_circuit Rust view is returned");
assert(briarObserver.private_view_status === "observer", "briar_circuit observer view is redacted");
assert(briarObserver.own_hand.length === 0, "briar_circuit observer omits private hand");
const briarSeat0 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [briarCircuit.match_id, "seat_0"],
);
assert(briarSeat0.own_hand.length === 13, "briar_circuit seat view exposes own hand");
const briarBlockedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [briarCircuit.match_id, "seat_0", "seat_1"],
);
assert(briarBlockedTree.choices.length === 0, "briar_circuit non-actor tree is empty");
const briarTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [briarCircuit.match_id, "seat_0", "seat_0"],
);
const briarPass = briarTree.choices.find((choice) => choice.segment === "pass");
const briarSelect = briarPass?.next?.choices?.find((choice) => choice.segment === "select");
const briarCard = briarSelect?.next?.choices?.[0];
assert(briarCard, "briar_circuit action tree exposes Rust pass card choices");
const briarAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(briarTree.freshness_token),
    ),
  [briarCircuit.match_id, "seat_0", `pass>select>${briarCard.segment}`],
);
assert(briarAfterHuman.view.pass.own_selection.length === 1, "briar_circuit pass selection updates private view");
assert(
  briarAfterHuman.effects.some((effect) => effect.payload.type === "pass_selection_updated"),
  "briar_circuit emits private pass-selection effect",
);
const briarExport = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [briarCircuit.match_id],
);
assert(briarExport.export_class === "viewer_scoped_observation_v1", "briar_circuit replay export is viewer scoped");
assert(!JSON.stringify(briarExport).includes(briarCard.segment), "briar_circuit public replay omits private pass card");

for (const seatCount of [3, 4, 5, 6, 7]) {
  const match = invoke(
    (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, BigInt(120 + seatCount), seatCount),
    ["vow_tide"],
  );
  const view = invoke((args) => wasm.rulepath_get_view(args[0].ptr, args[0].len), [match.match_id]);
  assert(view.game_id === "vow_tide", `vow_tide ${seatCount}-seat Rust view is returned`);
  assert(Object.keys(view.hand_counts).length === seatCount, `vow_tide ${seatCount}-seat view exposes hand-count rail`);
}
const vowTide = invoke(
  (args) => wasm.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, 127n, 7),
  ["vow_tide"],
);
assert(vowTide.match_id, "vow_tide start match returns a match id");
assert(vowTide.variant_id === "vow_tide_standard", "vow_tide standard variant starts");
const vowObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [vowTide.match_id],
);
assert(vowObserver.private_view_status === "observer", "vow_tide observer view is redacted");
assert(vowObserver.own_hand.length === 0, "vow_tide observer omits private hand");
assert(vowObserver.cumulative_scores.seat_0 === 0, "vow_tide view exposes public cumulative scores");
const vowSeat1 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [vowTide.match_id, "seat_1"],
);
assert(vowSeat1.own_hand.length === 7, "vow_tide seat view exposes own hand at seven seats");
const vowBlockedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [vowTide.match_id, "seat_1", "seat_0"],
);
assert(vowBlockedTree.choices.length === 0, "vow_tide non-actor tree is empty");
const vowTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [vowTide.match_id, "seat_1", "seat_1"],
);
const vowBid = vowTree.choices.find((choice) => choice.segment === "bid")?.next?.choices?.[0];
assert(vowBid, "vow_tide action tree exposes Rust bid choices");
const vowAfterBid = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(vowTree.freshness_token),
    ),
  [vowTide.match_id, "seat_1", `bid>${vowBid.segment}`],
);
assert(vowAfterBid.view.public_bids.seat_1 !== null, "vow_tide bid updates public bids");
assert(
  vowAfterBid.effects.some((effect) => effect.payload.type === "bid_accepted"),
  "vow_tide emits bid effect",
);

const maskedClaims = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 13n),
  ["masked_claims"],
);
assert(maskedClaims.match_id, "masked_claims start match returns a match id");
assert(maskedClaims.variant_id === "masked_claims_standard", "masked_claims standard variant starts");
const maskedObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [maskedClaims.match_id],
);
assert(maskedObserver.game_id === "masked_claims", "masked_claims Rust view is returned");
assert(maskedObserver.private_view.status === "observer", "masked_claims observer view is redacted");
assert(!JSON.stringify(maskedObserver).includes("mask_g"), "masked_claims observer view hides unrevealed mask ids");
const maskedSeat0 = invoke(
  (args) =>
    wasm.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [maskedClaims.match_id, "seat_0"],
);
assert(maskedSeat0.private_view.own_hand.length > 0, "masked_claims seat view exposes own hand");
const maskedTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [maskedClaims.match_id, "seat_0", "seat_0"],
);
const maskedClaim = maskedTree.choices.find((choice) => choice.segment === "claim");
const maskedTile = maskedClaim?.next?.choices?.[0];
const maskedDeclared = maskedTile?.next?.choices?.[0];
assert(maskedClaim && maskedTile && maskedDeclared, "masked_claims action tree exposes Rust claim path");
const maskedAfterClaim = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(maskedTree.freshness_token),
    ),
  [maskedClaims.match_id, "seat_0", `claim>${maskedTile.segment}>${maskedDeclared.segment}`],
);
assert(maskedAfterClaim.view.phase.includes("reaction"), "masked_claims claim opens reaction window");
assert(!JSON.stringify(maskedAfterClaim.view).includes(maskedTile.segment), "masked_claims pending claim hides unrevealed tile id");
const maskedResponseTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [maskedClaims.match_id, "seat_1", "seat_1"],
);
assert(
  maskedResponseTree.choices.some((choice) => choice.tags.includes("respond")),
  "masked_claims response tree exposes Rust response controls",
);
const maskedResponse = maskedResponseTree.choices.find((choice) => choice.tags.includes("respond"));
const maskedAfterResponse = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(maskedResponseTree.freshness_token),
    ),
  [maskedClaims.match_id, "seat_1", maskedResponse.segment],
);
assert(maskedAfterResponse.effects.length > 0, "masked_claims response emits effects");
assert(
  maskedAfterResponse.effects.some((effect) => effect.payload.type === "claim_score_changed"),
  "masked_claims score changes use a claim-specific effect discriminator",
);
assert(
  maskedAfterResponse.effects.some((effect) => effect.payload.type === "claim_turn_advanced"),
  "masked_claims turn advances use a claim-specific effect discriminator",
);
const maskedExport = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [maskedClaims.match_id],
);
assert(maskedExport.game_id === "masked_claims", "masked_claims replay export preserves game id");
assert(maskedExport.export_class === "viewer_scoped_observation", "masked_claims replay export is viewer scoped");
assert(!JSON.stringify(maskedExport).includes(maskedTile.segment), "masked_claims replay export hides unrevealed mask ids");
const maskedImport = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(maskedExport)],
);
assert(maskedImport.public_export === true, "masked_claims public replay imports");

const floodWatch = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 41n),
  ["flood_watch"],
);
assert(floodWatch.match_id, "flood_watch start match returns a match id");
assert(floodWatch.variant_id === "flood_watch_standard", "flood_watch standard variant starts");
const floodObserver = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [floodWatch.match_id],
);
assert(floodObserver.game_id === "flood_watch", "flood_watch Rust view is returned");
assert(floodObserver.variant_id === "flood_watch_standard", "flood_watch Rust view reports selected variant");
assert(floodObserver.remaining_composition.reprieves >= 0, "flood_watch projects public composition counts");
assert(floodObserver.undrawn_count > 0, "flood_watch projects only undrawn count");
assert(!JSON.stringify(floodObserver).includes("full_deck_order"), "flood_watch observer view hides full deck order");
assert(!JSON.stringify(floodObserver).includes("event_deck_internal"), "flood_watch observer view hides internal event deck");
const floodTree = invoke(
  (args) =>
    wasm.rulepath_get_action_tree_for_viewer(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
    ),
  [floodWatch.match_id, "seat_0", "seat_0"],
);
assert(floodTree.choices.some((choice) => choice.segment.startsWith("reinforce/")), "flood_watch exposes reinforce actions");
assert(floodTree.choices.some((choice) => choice.segment === "forecast"), "flood_watch exposes forecast action");
const floodAfterHuman = invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      BigInt(floodTree.freshness_token),
    ),
  [floodWatch.match_id, "seat_0", "end_turn"],
);
assert(floodAfterHuman.view.active_seat === "seat_1", "flood_watch alternates to teammate after storm resolution");
assert(
  floodAfterHuman.effects.some((effect) => effect.payload.type === "event_drawn"),
  "flood_watch emits public storm draw effect",
);
assert(!JSON.stringify(floodAfterHuman.view).includes("full_deck_order"), "flood_watch updated view hides full deck order");
const floodBot = invoke(
  (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 410n),
  [floodWatch.match_id, "seat_1"],
);
assert(floodBot.view.game_id === "flood_watch", "flood_watch bot turn returns public view");
assert(JSON.stringify(floodBot).includes("flood_watch_level1_public_priority_v1"), "flood_watch bot emits public policy id");
const floodExport = invoke(
  (args) => wasm.rulepath_export_replay(args[0].ptr, args[0].len),
  [floodWatch.match_id],
);
assert(floodExport.game_id === "flood_watch", "flood_watch replay export preserves game id");
assert(floodExport.viewer === "observer", "flood_watch replay export is observer scoped");
assert(floodExport.redacted_command_summary || floodExport.steps?.[0]?.redacted_command_summary, "flood_watch replay export redacts commands");
assert(!("commands" in floodExport), "flood_watch replay export omits raw command stream");
assert(!JSON.stringify(floodExport).includes("full_deck_order"), "flood_watch replay export hides full deck order");
assert(!JSON.stringify(floodExport).includes("event_deck_internal"), "flood_watch replay export hides internal event deck");
const floodImport = invoke(
  (args) => wasm.rulepath_import_replay(args[0].ptr, args[0].len),
  [JSON.stringify(floodExport)],
);
assert(floodImport.public_export === true, "flood_watch public replay imports");

const hotseat = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 2n),
  ["race_to_n"],
);
const hotseatSeat0 = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [hotseat.match_id, "seat_0"],
);
assert(hotseatSeat0.choices.length > 0, "hotseat seat_0 gets Rust choices");
invoke(
  (args) =>
    wasm.rulepath_apply_action(
      args[0].ptr,
      args[0].len,
      args[1].ptr,
      args[1].len,
      args[2].ptr,
      args[2].len,
      0n,
    ),
  [hotseat.match_id, "seat_0", hotseatSeat0.choices[0].segment],
);
const hotseatView = invoke(
  (args) => wasm.rulepath_get_view(args[0].ptr, args[0].len),
  [hotseat.match_id],
);
const hotseatSeat1 = invoke(
  (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
  [hotseat.match_id, hotseatView.active_seat],
);
assert(hotseatView.active_seat === "seat_1", "hotseat alternates to seat_1");
assert(hotseatSeat1.choices.length > 0, "hotseat active seat gets Rust choices");

const botVsBot = invoke(
  (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 3n),
  ["race_to_n"],
);
const botStep0 = invoke(
  (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 300n),
  [botVsBot.match_id, "seat_0"],
);
assert(botStep0.view.counter > 0, "bot-vs-bot first step advances through Rust bot");
if (!botStep0.view.winner) {
  const botStep1 = invoke(
    (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 301n),
    [botVsBot.match_id, botStep0.view.active_seat],
  );
  assert(botStep1.view.counter >= botStep0.view.counter, "bot-vs-bot second step advances or holds terminal");
}

console.log(
  JSON.stringify({
    version,
    match_id: created.match_id,
    counter: afterBot.view.counter,
    effects: effects.length,
    diagnostic: staleDiagnostic.code,
    modes: ["human_vs_bot", "hotseat", "bot_vs_bot"],
    replay_cursor: replayStep.cursor,
    token_bazaar_match_id: tokenBazaar.match_id,
    plain_tricks_match_id: plainTricks.match_id,
    masked_claims_match_id: maskedClaims.match_id,
    flood_watch_match_id: floodWatch.match_id,
  }),
);
