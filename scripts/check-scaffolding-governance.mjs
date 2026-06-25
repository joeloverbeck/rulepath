#!/usr/bin/env node

import {
  existsSync,
  readFileSync,
  readdirSync,
} from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const scriptRoot = join(dirname(fileURLToPath(import.meta.url)), "..");

const HISTORICAL_LEGACY_8C_GAMES = [
  "race_to_n",
  "three_marks",
  "column_four",
  "directional_flip",
  "draughts_lite",
  "high_card_duel",
  "masked_claims",
  "flood_watch",
  "frontier_control",
  "event_frontier",
  "token_bazaar",
  "secret_draft",
  "poker_lite",
  "plain_tricks",
  "river_ledger",
  "briar_circuit",
  "vow_tide",
];

const KNOWN_SIGNALS = new Set([
  "MSC-8C-001.effect-envelope-literal",
  "MSC-8C-002.local-seat-grammar",
  "MSC-8C-004.local-action-tree-v1-framing",
  "MSC-8C-005.local-stable-byte-writer",
  "MSC-8C-006.production-support-edge",
]);

const TOP_LEVEL_FIELDS = new Set(["schema_version", "legacy_8c_games", "games"]);
const GAME_FIELDS = new Set([
  "id",
  "coverage",
  "evidence_paths",
  "audit",
  "evidence",
  "register_entries_reviewed",
  "register_decisions",
  "disposition",
  "prior_matching_games",
  "follow_on_unit",
  "no_follow_on_decision",
  "known_signal_dispositions",
  "compatibility",
]);
const SIGNAL_FIELDS = new Set(["signal", "decision", "evidence"]);
const COMPATIBILITY_FIELDS = new Set([
  "hash_migration",
  "visibility_migration",
  "determinism_migration",
  "hashes",
  "visibility",
  "determinism",
  "migration_authority",
]);

const COVERAGE_VALUES = new Set(["legacy-8c-covered", "forward-v1"]);
const DISPOSITION_VALUES = new Set([
  "legacy-8c-covered",
  "reuse-only",
  "no-new-scaffolding",
  "register-updated",
  "accepted-local-only",
  "accepted-deferred",
  "accepted-rejected",
]);
const SIGNAL_DECISIONS = new Set([
  "legacy-reviewed",
  "reused",
  "exception",
  "not-present",
]);

function readJson(path, errors) {
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch (error) {
    errors.push(`${relativePath(path)} is not valid JSON: ${error.message}`);
    return null;
  }
}

function sortedUnique(values) {
  return [...new Set(values)].sort();
}

function setDifference(left, right) {
  const rightSet = new Set(right);
  return left.filter((value) => !rightSet.has(value));
}

function sameSet(left, right) {
  return (
    left.length === right.length &&
    setDifference(left, right).length === 0 &&
    setDifference(right, left).length === 0
  );
}

function relativePath(path, root = scriptRoot) {
  return relative(root, path).replaceAll("\\", "/");
}

function isPlainObject(value) {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function unknownFields(object, allowed) {
  return Object.keys(object).filter((key) => !allowed.has(key));
}

function requireString(value, field, errors, context) {
  if (typeof value !== "string" || value.length === 0) {
    errors.push(`${context}.${field} must be a non-empty string`);
    return false;
  }
  return true;
}

function requireArray(value, field, errors, context) {
  if (!Array.isArray(value)) {
    errors.push(`${context}.${field} must be an array`);
    return false;
  }
  return true;
}

function assertRepoPath(root, rawPath, errors, context) {
  if (typeof rawPath !== "string" || rawPath.length === 0) {
    errors.push(`${context} has a blank path`);
    return false;
  }
  const resolved = resolve(root, rawPath);
  const rel = relative(root, resolved);
  if (rel.startsWith("..") || rel === "") {
    errors.push(`${context} escapes the repository: ${rawPath}`);
    return false;
  }
  if (!existsSync(resolved)) {
    errors.push(`${context} does not exist: ${rawPath}`);
    return false;
  }
  return true;
}

function collectGameDirs(root) {
  const gamesDir = join(root, "games");
  return readdirSync(gamesDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();
}

function readRegisterIds(root) {
  const registerPath = join(root, "docs", "MECHANICAL-SCAFFOLDING-REGISTER.md");
  const register = readFileSync(registerPath, "utf8");
  const ids = new Set(register.match(/\bMSC-[A-Z0-9-]+\b/g) ?? []);
  const promotionDebtOpenIds = new Set();
  for (const match of register.matchAll(/^###\s+(MSC-[A-Z0-9-]+)[^\n]*\n(?:[\s\S]*?^- Entry id:[^\n]*status `promotion-debt-open`[^\n]*$)/gm)) {
    promotionDebtOpenIds.add(match[1]);
  }
  return { ids, register, promotionDebtOpenIds };
}

function readSpecIds(root) {
  const specsReadme = join(root, "specs", "README.md");
  if (!existsSync(specsReadme)) {
    return "";
  }
  return readFileSync(specsReadme, "utf8");
}

function validateManifest(root, errors) {
  const manifestPath = join(root, "ci", "scaffolding-audits.json");
  const gamesManifestPath = join(root, "ci", "games.json");
  const receipt = readJson(manifestPath, errors);
  const gameManifest = readJson(gamesManifestPath, errors);
  if (!receipt || !gameManifest) {
    return null;
  }

  if (!isPlainObject(receipt)) {
    errors.push("ci/scaffolding-audits.json must be a JSON object");
    return null;
  }
  for (const field of unknownFields(receipt, TOP_LEVEL_FIELDS)) {
    errors.push(`ci/scaffolding-audits.json has unknown top-level field "${field}"`);
  }
  if (receipt.schema_version !== 1) {
    errors.push("ci/scaffolding-audits.json schema_version must be 1");
  }
  if (!requireArray(receipt.legacy_8c_games, "legacy_8c_games", errors, "receipt")) {
    receipt.legacy_8c_games = [];
  }
  if (!requireArray(receipt.games, "games", errors, "receipt")) {
    receipt.games = [];
  }
  if (!Array.isArray(gameManifest)) {
    errors.push("ci/games.json must be an array");
    return null;
  }

  const legacy = sortedUnique(receipt.legacy_8c_games);
  const historical = [...HISTORICAL_LEGACY_8C_GAMES].sort();
  if (!sameSet(legacy, historical)) {
    errors.push(
      `legacy_8c_games must equal the frozen Unit 8C set; missing ${JSON.stringify(
        setDifference(historical, legacy),
      )}, extra ${JSON.stringify(setDifference(legacy, historical))}`,
    );
  }

  const ciIds = sortedUnique(gameManifest.map((entry) => entry.id));
  const dirIds = collectGameDirs(root);
  const receiptIds = receipt.games.map((entry) => entry.id);
  const receiptUniqueIds = sortedUnique(receiptIds);
  if (receiptIds.length !== receiptUniqueIds.length) {
    errors.push("ci/scaffolding-audits.json contains duplicate game ids");
  }
  if (!sameSet(ciIds, dirIds)) {
    errors.push(
      `ci/games.json and games/ differ; missing dirs ${JSON.stringify(
        setDifference(ciIds, dirIds),
      )}, missing manifest rows ${JSON.stringify(setDifference(dirIds, ciIds))}`,
    );
  }
  if (!sameSet(receiptUniqueIds, ciIds) || !sameSet(receiptUniqueIds, dirIds)) {
    errors.push(
      `scaffolding audit game ids must equal ci/games.json and games/; missing ${JSON.stringify(
        setDifference(ciIds, receiptUniqueIds),
      )}, extra ${JSON.stringify(setDifference(receiptUniqueIds, ciIds))}`,
    );
  }

  return receipt;
}

function validateGameEntry(root, game, index, receipt, support, errors) {
  const context = `games[${index}]${game?.id ? ` (${game.id})` : ""}`;
  if (!isPlainObject(game)) {
    errors.push(`${context} must be an object`);
    return;
  }
  for (const field of unknownFields(game, GAME_FIELDS)) {
    errors.push(`${context} has unknown field "${field}"`);
  }
  if (!requireString(game.id, "id", errors, context)) {
    return;
  }
  if (!requireString(game.coverage, "coverage", errors, context)) {
    return;
  }
  if (!COVERAGE_VALUES.has(game.coverage)) {
    errors.push(`${context}.coverage has unknown value "${game.coverage}"`);
  }
  if (game.coverage === "legacy-8c-covered" && !receipt.legacy_8c_games.includes(game.id)) {
    errors.push(`${context} uses legacy-8c-covered outside the frozen Unit 8C set`);
  }
  if (game.coverage === "forward-v1" && receipt.legacy_8c_games.includes(game.id)) {
    errors.push(`${context} is in the frozen Unit 8C set but claims forward-v1`);
  }

  validatePathArray(root, game.evidence_paths, "evidence_paths", errors, context);
  if (game.audit !== undefined) {
    assertRepoPath(root, game.audit.split("#")[0], errors, `${context}.audit`);
  }
  if (game.evidence !== undefined) {
    assertRepoPath(root, game.evidence.split("#")[0], errors, `${context}.evidence`);
  }

  validateRegisterIds(game.register_entries_reviewed, "register_entries_reviewed", support.registerIds, errors, context);
  validateRegisterIds(game.register_decisions, "register_decisions", support.registerIds, errors, context);

  if (!requireString(game.disposition, "disposition", errors, context)) {
    return;
  }
  if (!DISPOSITION_VALUES.has(game.disposition)) {
    errors.push(`${context}.disposition has unknown value "${game.disposition}"`);
  }
  if (
    ["register-updated", "accepted-local-only", "accepted-deferred", "accepted-rejected"].includes(
      game.disposition,
    ) &&
    (!Array.isArray(game.register_decisions) || game.register_decisions.length === 0)
  ) {
    errors.push(`${context}.disposition requires at least one register_decisions entry`);
  }

  if (!requireArray(game.prior_matching_games, "prior_matching_games", errors, context)) {
    game.prior_matching_games = [];
  }
  for (const prior of game.prior_matching_games) {
    if (typeof prior !== "string" || prior.length === 0) {
      errors.push(`${context}.prior_matching_games contains a blank or non-string value`);
    } else if (!support.gameIds.has(prior)) {
      errors.push(`${context}.prior_matching_games references unknown game "${prior}"`);
    }
  }
  if (
    game.prior_matching_games.length > 0 &&
    !game.follow_on_unit &&
    !game.no_follow_on_decision
  ) {
    errors.push(`${context} names prior matching games but has no follow_on_unit or no_follow_on_decision`);
  }
  if (game.follow_on_unit !== null && game.follow_on_unit !== undefined) {
    if (!requireString(game.follow_on_unit, "follow_on_unit", errors, context)) {
      return;
    }
    if (!support.specsReadme.includes(game.follow_on_unit)) {
      errors.push(`${context}.follow_on_unit "${game.follow_on_unit}" is absent from specs/README.md`);
    }
  }
  if (game.no_follow_on_decision !== null && game.no_follow_on_decision !== undefined) {
    if (!requireString(game.no_follow_on_decision, "no_follow_on_decision", errors, context)) {
      return;
    }
    if (
      game.no_follow_on_decision !== "legacy-8c-covered" &&
      !support.registerIds.has(game.no_follow_on_decision)
    ) {
      errors.push(`${context}.no_follow_on_decision must be legacy-8c-covered or an MSC id`);
    }
  }

  validateSignals(root, game.known_signal_dispositions, errors, context, support.registerIds);
  validateCompatibility(game.compatibility, errors, context);
}

function validatePathArray(root, value, field, errors, context) {
  if (!requireArray(value, field, errors, context)) {
    return;
  }
  if (value.length === 0) {
    errors.push(`${context}.${field} must not be empty`);
  }
  for (const rawPath of value) {
    assertRepoPath(root, rawPath.split("#")[0], errors, `${context}.${field}`);
  }
}

function validateRegisterIds(value, field, registerIds, errors, context) {
  if (!requireArray(value, field, errors, context)) {
    return;
  }
  for (const id of value) {
    if (typeof id !== "string" || id.length === 0) {
      errors.push(`${context}.${field} contains a blank or non-string value`);
    } else if (!registerIds.has(id)) {
      errors.push(`${context}.${field} references unknown register id "${id}"`);
    }
  }
}

function validateSignals(root, signals, errors, context, registerIds) {
  if (!requireArray(signals, "known_signal_dispositions", errors, context)) {
    return;
  }
  for (const [index, signal] of signals.entries()) {
    const signalContext = `${context}.known_signal_dispositions[${index}]`;
    if (!isPlainObject(signal)) {
      errors.push(`${signalContext} must be an object`);
      continue;
    }
    for (const field of unknownFields(signal, SIGNAL_FIELDS)) {
      errors.push(`${signalContext} has unknown field "${field}"`);
    }
    if (!requireString(signal.signal, "signal", errors, signalContext)) {
      continue;
    }
    if (!KNOWN_SIGNALS.has(signal.signal)) {
      errors.push(`${signalContext}.signal has unknown value "${signal.signal}"`);
    }
    if (!requireString(signal.decision, "decision", errors, signalContext)) {
      continue;
    }
    if (!SIGNAL_DECISIONS.has(signal.decision)) {
      errors.push(`${signalContext}.decision has unknown value "${signal.decision}"`);
    }
    if (!requireString(signal.evidence, "evidence", errors, signalContext)) {
      continue;
    }
    const evidencePath = signal.evidence.split("#")[0];
    if (!registerIds.has(signal.evidence) && !assertRepoPath(root, evidencePath, errors, `${signalContext}.evidence`)) {
      continue;
    }
  }
}

function validateCompatibility(compatibility, errors, context) {
  if (!isPlainObject(compatibility)) {
    errors.push(`${context}.compatibility must be an object`);
    return;
  }
  for (const field of unknownFields(compatibility, COMPATIBILITY_FIELDS)) {
    errors.push(`${context}.compatibility has unknown field "${field}"`);
  }

  const hashValue = compatibility.hash_migration ?? compatibility.hashes;
  const visibilityValue = compatibility.visibility_migration ?? compatibility.visibility;
  const determinismValue = compatibility.determinism_migration ?? compatibility.determinism;
  const authority = compatibility.migration_authority;
  for (const [field, value] of [
    ["hash", hashValue],
    ["visibility", visibilityValue],
    ["determinism", determinismValue],
  ]) {
    if (!["none", "unchanged", "migration-authorized"].includes(value)) {
      errors.push(`${context}.compatibility ${field} value must be none, unchanged, or migration-authorized`);
    }
  }
  if (typeof authority !== "string" || authority.length === 0) {
    errors.push(`${context}.compatibility.migration_authority must be a non-empty string`);
    return;
  }
  const needsAuthority = [hashValue, visibilityValue, determinismValue].includes("migration-authorized");
  if (needsAuthority && !/ADR[- ]?0009/i.test(authority)) {
    errors.push(`${context}.compatibility migration authority must cite ADR 0009`);
  }
  if (!needsAuthority && authority !== "none") {
    errors.push(`${context}.compatibility.migration_authority must be none unless migration is authorized`);
  }
}

function validateBehaviorFreeReceiptText(receipt, errors) {
  const text = JSON.stringify(receipt);
  for (const word of ["selector", "condition", "trigger", "formula"]) {
    if (text.toLowerCase().includes(word)) {
      errors.push(`ci/scaffolding-audits.json contains behavior-looking term "${word}"`);
    }
  }
}

function collectFiles(dir, predicate) {
  const files = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectFiles(path, predicate));
    } else if (entry.isFile() && predicate(path)) {
      files.push(path);
    }
  }
  return files;
}

function findNormalGameTestSupportEdges(root) {
  const findings = [];
  const cargoFiles = collectFiles(join(root, "games"), (path) => path.endsWith("Cargo.toml"));
  for (const cargoFile of cargoFiles) {
    const lines = readFileSync(cargoFile, "utf8").split(/\r?\n/);
    let section = "";
    for (const [index, line] of lines.entries()) {
      const heading = /^\s*\[([^\]]+)\]\s*$/.exec(line);
      if (heading) {
        section = heading[1];
        continue;
      }
      if (section === "dependencies" && /^\s*game-test-support\s*=/.test(line)) {
        findings.push(`${relativePath(cargoFile, root)}:${index + 1} normal dependency on game-test-support`);
      }
    }
  }
  return findings;
}

function validateKnownSignals(root, receipt, errors) {
  const productionSupportFindings = findNormalGameTestSupportEdges(root);
  for (const finding of productionSupportFindings) {
    errors.push(`${finding} violates MSC-8C-006.production-support-edge`);
  }

  const receiptSignalCount = receipt.games.reduce(
    (count, game) => count + (Array.isArray(game.known_signal_dispositions) ? game.known_signal_dispositions.length : 0),
    0,
  );

  return {
    knownSignals: KNOWN_SIGNALS.size,
    sourceScans: 1,
    receiptSignalCount,
  };
}

export function checkScaffoldingGovernance(options = {}) {
  const root = options.root ? resolve(options.root) : scriptRoot;
  const errors = [];
  const receipt = validateManifest(root, errors);
  if (!receipt) {
    return { ok: false, errors, summary: null };
  }

  validateBehaviorFreeReceiptText(receipt, errors);

  const { ids: registerIds, promotionDebtOpenIds } = readRegisterIds(root);
  const support = {
    gameIds: new Set(collectGameDirs(root)),
    registerIds,
    specsReadme: readSpecIds(root),
  };

  for (const [index, game] of receipt.games.entries()) {
    validateGameEntry(root, game, index, receipt, support, errors);
  }

  for (const id of promotionDebtOpenIds) {
    const hasClosure = receipt.games.some(
      (game) =>
        Array.isArray(game.register_decisions) &&
        game.register_decisions.includes(id) &&
        (game.follow_on_unit || game.no_follow_on_decision),
    );
    if (!hasClosure) {
      errors.push(`register entry ${id} is promotion-debt-open without closure evidence in ci/scaffolding-audits.json`);
    }
  }

  const signalSummary = validateKnownSignals(root, receipt, errors);
  const legacyReceipts = receipt.games.filter((game) => game.coverage === "legacy-8c-covered").length;
  const forwardReceipts = receipt.games.filter((game) => game.coverage === "forward-v1").length;
  const registerDecisionCount = receipt.games.reduce(
    (count, game) => count + (Array.isArray(game.register_decisions) ? game.register_decisions.length : 0),
    0,
  );
  const followOnCount = receipt.games.filter((game) => Boolean(game.follow_on_unit)).length;

  return {
    ok: errors.length === 0,
    errors,
    summary: {
      games: receipt.games.length,
      forwardReceipts,
      legacyReceipts,
      knownSignals: signalSummary.knownSignals,
      sourceScans: signalSummary.sourceScans,
      receiptSignalCount: signalSummary.receiptSignalCount,
      registerDecisionCount,
      followOnCount,
    },
  };
}

function printResult(result) {
  if (!result.ok) {
    console.error("scaffolding-governance check failed:");
    for (const error of result.errors) {
      console.error(` - ${error}`);
    }
    return 1;
  }

  const summary = result.summary;
  console.log(
    `scaffolding-governance OK — games=${summary.games}; forward-v1=${summary.forwardReceipts}; legacy=${summary.legacyReceipts}; known_signals=${summary.knownSignals}; source_scans=${summary.sourceScans}; receipt_signals=${summary.receiptSignalCount}; register_decisions=${summary.registerDecisionCount}; follow_on_units=${summary.followOnCount}`,
  );
  return 0;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const result = checkScaffoldingGovernance();
  process.exitCode = printResult(result);
}
