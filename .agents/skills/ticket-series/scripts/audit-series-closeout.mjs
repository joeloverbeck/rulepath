#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

const argv = process.argv.slice(2);
if (argv.includes("--help") || argv.includes("-h")) {
  printUsage();
  process.exit(0);
}

const args = parseArgs(argv);
const prefix = args["ticket-prefix"];
const activeReference = args["active-reference"];
const archivedReference = args["archived-reference"];
const expectedCount = args["expected-count"];
const expectedTicketRange = args["expected-ticket-range"];
const expectedTicketList = args["expected-ticket-list"];
const ledgerFormat = args["ledger-format"];
const referenceOnly = args["reference-only"] === true;
const summary = args.summary === true;
const forbiddenTokenFile = args["forbidden-token-file"];
const forbiddenScanRoots = args["forbidden-scan-roots"];
const contractPreview = args["contract-preview"] === true;

if (!prefix && !referenceOnly && !contractPreview) {
  printUsage(console.error);
  process.exit(2);
}
if (contractPreview && !prefix && !activeReference && !archivedReference) {
  printUsage(console.error);
  process.exit(2);
}
if (ledgerFormat !== undefined && ledgerFormat !== "compact") {
  console.error(`Invalid --ledger-format: ${ledgerFormat}`);
  console.error("Expected: compact");
  process.exit(2);
}
if (referenceOnly && !archivedReference) {
  console.error("--reference-only requires --archived-reference");
  process.exit(2);
}

let expectedTicketNames = null;
if (expectedTicketRange) {
  expectedTicketNames = expandTicketRange(expectedTicketRange);
}
if (expectedTicketList) {
  const listNames = readExpectedTicketList(expectedTicketList);
  expectedTicketNames = expectedTicketNames
    ? [...new Set([...expectedTicketNames, ...listNames])]
    : listNames;
}

let failures = 0;

function section(title) {
  console.log(`\n== ${title} ==`);
}

function fail(message) {
  failures += 1;
  console.log(`FAIL: ${message}`);
}

function ok(message) {
  console.log(`OK: ${message}`);
}

function printUsage(write = console.log) {
  write(
    "Usage: audit-series-closeout.mjs --ticket-prefix PREFIX " +
      "[--active-reference specs/name.md] " +
      "[--archived-reference archive/specs/name.md] " +
      "[--expected-count N] " +
      "[--expected-ticket-list FILE] " +
      "[--expected-ticket-range PREFIX-001..020] " +
      "[--forbidden-token-file FILE] " +
      "[--forbidden-scan-roots roots,comma,separated] " +
      "[--contract-preview] " +
      "[--summary] " +
      "[--ledger-format compact]\n" +
      "       audit-series-closeout.mjs --reference-only " +
      "[--active-reference specs/name.md] " +
      "--archived-reference archive/specs/name.md " +
      "[--forbidden-token-file FILE] " +
      "[--forbidden-scan-roots roots,comma,separated] " +
      "[--summary]",
  );
}

function run(cmd, cmdArgs, options = {}) {
  console.log(`$ ${[cmd, ...cmdArgs].join(" ")}`);
  const result = spawnSync(cmd, cmdArgs, {
    cwd: process.cwd(),
    encoding: "utf8",
    shell: false,
  });
  if (result.stdout && !options.quietStdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);
  if (!options.allowExitCodes?.includes(result.status ?? 0) && result.status !== 0) {
    fail(`${cmd} exited ${result.status}`);
  }
  return result;
}

function readLines(file) {
  try {
    return fs.readFileSync(file, "utf8").split(/\r?\n/);
  } catch {
    return null;
  }
}

function listArchivedTickets() {
  const dir = path.join(process.cwd(), "archive", "tickets");
  if (!fs.existsSync(dir)) return [];
  return fs
    .readdirSync(dir)
    .filter((name) => name.startsWith(prefix) && name.endsWith(".md"))
    .sort()
    .map((name) => path.join("archive", "tickets", name));
}

function reportStatusOutcome(file) {
  const lines = readLines(file);
  if (!lines) {
    fail(`${file} is missing`);
    return false;
  }

  const matches = [];
  let hasValidStatus = false;
  let hasOutcome = false;
  let informal = false;
  lines.forEach((line, index) => {
    if (/^\*\*Status\*\*:|^## Outcome/.test(line)) {
      matches.push(`${file}:${index + 1}:${line}`);
    }
    if (/^\*\*Status\*\*: .*?(COMPLETED|REJECTED|DEFERRED|NOT IMPLEMENTED)$/.test(line)) {
      hasValidStatus = true;
    }
    if (/^## Outcome/.test(line)) hasOutcome = true;
    if (/^\*\*Status\*\*: (DONE|COMPLETE|ACCEPTED)$|^## Completion Notes/.test(line)) {
      informal = true;
      matches.push(`${file}:${index + 1}:INFORMAL_STATUS:${line}`);
    }
  });
  if (matches.length === 0) {
    fail(`${file} has no status or outcome lines`);
  } else if (!summary) {
    console.log(matches.join("\n"));
  }
  if (!hasValidStatus) fail(`${file} has no valid archival status line`);
  if (!hasOutcome) fail(`${file} has no ## Outcome`);
  if (informal) fail(`${file} contains informal status syntax`);
  const ok = matches.length > 0 && hasValidStatus && hasOutcome && !informal;
  if (!ok && summary && matches.length > 0) console.log(matches.join("\n"));
  return ok;
}

function reportArchivedReference(file) {
  const lines = readLines(file);
  if (!lines) {
    fail(`${file} is missing`);
    return false;
  }

  const validStatus = /^\*\*Status\*\*: .*?(COMPLETED|REJECTED|DEFERRED|NOT IMPLEMENTED)$|^\| Status \| `?Done`? \|/;
  const informalStatus = /^\s*-?\s*\*\*Status\*\*:\s*(Done|ACCEPTED)|^\s*- \*\*Status:\*\*/;
  const boldSpecStatus = /^\| \*\*Status\*\* \|.*\|/;
  const numberedOutcome = /^## .+Outcome\b/;
  const matches = [];
  let hasValidStatus = false;
  let hasOutcome = false;
  let informal = false;
  let nearMiss = false;
  let outcomeNearMiss = false;

  lines.forEach((line, index) => {
    if (validStatus.test(line) || /^## Outcome/.test(line)) {
      matches.push(`${file}:${index + 1}:${line}`);
    }
    if (validStatus.test(line)) hasValidStatus = true;
    if (/^## Outcome/.test(line)) hasOutcome = true;
    if (informalStatus.test(line)) {
      informal = true;
      console.log(`${file}:${index + 1}:INFORMAL_STATUS:${line}`);
    }
    if (boldSpecStatus.test(line)) {
      nearMiss = true;
      console.log(`${file}:${index + 1}:NEAR_MISS_STATUS:${line}`);
    }
    if (numberedOutcome.test(line) && !/^## Outcome/.test(line)) {
      outcomeNearMiss = true;
      console.log(`${file}:${index + 1}:NEAR_MISS_OUTCOME:${line}`);
    }
  });

  if (!summary && matches.length > 0) console.log(matches.join("\n"));
  if (!hasValidStatus) fail(`${file} has no valid archival status line`);
  if (!hasOutcome) fail(`${file} has no ## Outcome`);
  if (informal) fail(`${file} contains informal status syntax`);
  if (nearMiss) {
    fail(
      `${file} uses a bolded spec status label; use exact row: | Status | \`Done\` |`,
    );
  }
  if (outcomeNearMiss) {
    fail(`${file} uses a numbered Outcome heading; use exact heading: ## Outcome`);
  }
  const referenceOk =
    hasValidStatus && hasOutcome && !informal && !nearMiss && !outcomeNearMiss;
  if (summary) {
    if (referenceOk) {
      ok(`archived reference status/outcome present for ${file}`);
    } else if (matches.length > 0) {
      console.log(matches.join("\n"));
    }
    if (!referenceOk) {
      console.log(
        `repair_hint=${file}: archived specs require | Status | \`Done\` | and exact ## Outcome`,
      );
    }
  }
  return referenceOk;
}

function parseArgs(argv) {
  const parsed = {};
  const flagArgs = new Set(["contract-preview", "reference-only", "summary"]);
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (!arg.startsWith("--")) {
      console.error(`Unexpected argument: ${arg}`);
      process.exit(2);
    }
    const key = arg.slice(2);
    if (flagArgs.has(key)) {
      parsed[key] = true;
      continue;
    }
    const value = argv[i + 1];
    if (!value || value.startsWith("--")) {
      console.error(`Missing value for ${arg}`);
      process.exit(2);
    }
    parsed[key] = value;
    i += 1;
  }
  return parsed;
}

function inferArchivedReference(activeReferencePath) {
  if (!activeReferencePath) return null;
  if (activeReferencePath.startsWith("specs/")) {
    return path.join("archive", activeReferencePath);
  }
  if (activeReferencePath.startsWith("docs/triage/")) {
    return path.join("archive", "triage", path.basename(activeReferencePath));
  }
  if (activeReferencePath.startsWith("tasks/")) {
    return path.join("archive", "tasks", path.basename(activeReferencePath));
  }
  if (activeReferencePath.startsWith("plans/")) {
    return path.join("archive", "plans", path.basename(activeReferencePath));
  }
  return null;
}

function countActiveTicketFiles() {
  if (!prefix) return null;
  const dir = path.join(process.cwd(), "tickets");
  if (!fs.existsSync(dir)) return 0;
  return fs
    .readdirSync(dir)
    .filter((name) => name.startsWith(prefix) && name.endsWith(".md")).length;
}

function printContractPreview() {
  section("Closeout Contract Preview");
  const activeTicketCount = countActiveTicketFiles();
  if (activeTicketCount === null) {
    console.log("ticket_prefix=(none)");
    console.log("active_ticket_count=unknown");
  } else {
    console.log(`ticket_prefix=${prefix}`);
    console.log(`active_ticket_count=${activeTicketCount}`);
  }

  const expectedArchivedReference =
    archivedReference ?? inferArchivedReference(activeReference);
  const activeReferenceExists = activeReference
    ? fs.existsSync(activeReference)
    : null;
  const archivedReferenceExists = expectedArchivedReference
    ? fs.existsSync(expectedArchivedReference)
    : null;

  console.log(`active_reference=${activeReference ?? "(none)"}`);
  console.log(
    `active_reference_exists=${
      activeReferenceExists === null ? "unknown" : String(activeReferenceExists)
    }`,
  );
  console.log(`expected_archived_reference=${expectedArchivedReference ?? "(none)"}`);
  console.log(
    `archived_reference_exists=${
      archivedReferenceExists === null ? "unknown" : String(archivedReferenceExists)
    }`,
  );

  let contract;
  if (!activeReference && !expectedArchivedReference) {
    contract = "ticket-only";
  } else if (activeReferenceExists === false && archivedReferenceExists === true) {
    contract = "reference-archive closeout appears complete";
  } else if (activeReferenceExists === true) {
    contract = "reference-archive closeout required or explicit status-only instruction needed";
  } else if (activeReferenceExists === false && archivedReferenceExists === false) {
    contract = "reference path absent but archived reference missing";
  } else {
    contract = "undetermined";
  }
  console.log(`contract=${contract}`);
  console.log(
    `reference_archival_pending=${String(
      activeReferenceExists === true || (activeReferenceExists === false && archivedReferenceExists === false),
    )}`,
  );
  console.log("note=preview only; final closeout audit must still pass after final commit");
}

function expandTicketRange(range) {
  const match = /^(.*-)(\d+)\.\.(\d+)$/.exec(range);
  if (!match) {
    console.error(`Invalid --expected-ticket-range: ${range}`);
    console.error("Expected form: PREFIX-001..020");
    process.exit(2);
  }
  const [, stem, startText, endText] = match;
  const start = Number.parseInt(startText, 10);
  const end = Number.parseInt(endText, 10);
  if (!Number.isInteger(start) || !Number.isInteger(end) || end < start) {
    console.error(`Invalid --expected-ticket-range bounds: ${range}`);
    process.exit(2);
  }
  const width = startText.length;
  return Array.from({ length: end - start + 1 }, (_, offset) => {
    const number = String(start + offset).padStart(width, "0");
    return path.join("archive", "tickets", `${stem}${number}.md`);
  });
}

function readExpectedTicketList(file) {
  const lines = readLines(file);
  if (!lines) {
    console.error(`Missing --expected-ticket-list file: ${file}`);
    process.exit(2);
  }
  const names = lines
    .map((line) => line.trim())
    .filter((line) => line && !line.startsWith("#"))
    .map(normalizeExpectedTicketName);
  if (names.length === 0) {
    console.error(`Empty --expected-ticket-list file: ${file}`);
    process.exit(2);
  }
  return names;
}

function normalizeExpectedTicketName(value) {
  if (value.startsWith("archive/tickets/") && value.endsWith(".md")) {
    return value;
  }
  const basename = value.endsWith(".md") ? value : `${value}.md`;
  return path.join("archive", "tickets", basename);
}

function readForbiddenTokenFile(file) {
  const lines = readLines(file);
  if (!lines) {
    console.error(`Missing --forbidden-token-file: ${file}`);
    process.exit(2);
  }
  const tokens = [];
  lines.forEach((line, index) => {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) return;
    const separator = trimmed.includes("\t") ? "\t" : "=";
    const separatorIndex = trimmed.indexOf(separator);
    if (separatorIndex <= 0 || separatorIndex === trimmed.length - 1) {
      console.error(
        `Invalid --forbidden-token-file line ${index + 1}; expected label=literal or label<TAB>literal`,
      );
      process.exit(2);
    }
    const label = trimmed.slice(0, separatorIndex).trim();
    const value = trimmed.slice(separatorIndex + 1);
    if (!/^[A-Za-z0-9_.:-]+$/.test(label)) {
      console.error(
        `Invalid --forbidden-token-file label on line ${index + 1}; use letters, numbers, dot, underscore, colon, or dash`,
      );
      process.exit(2);
    }
    if (value.length === 0) {
      console.error(`Empty forbidden token value on line ${index + 1}`);
      process.exit(2);
    }
    tokens.push({ label, value });
  });
  if (tokens.length === 0) {
    console.error(`Empty --forbidden-token-file: ${file}`);
    process.exit(2);
  }
  return tokens;
}

function existingRoots(roots) {
  return roots.filter((root) => fs.existsSync(root));
}

function publicScanRoots() {
  const defaultRoots = [
    ".github",
    "apps",
    "archive",
    "ci",
    "crates",
    "docs",
    "games",
    "reports",
    "scripts",
    "specs",
    "templates",
    "tickets",
    "tools",
  ];
  const roots = forbiddenScanRoots
    ? forbiddenScanRoots
        .split(",")
        .map((root) => root.trim())
        .filter(Boolean)
    : defaultRoots;
  return existingRoots(roots);
}

function collectTrackedFiles(roots) {
  if (roots.length === 0) return [];
  const result = spawnSync("git", ["ls-files", "--", ...roots], {
    cwd: process.cwd(),
    encoding: "utf8",
    shell: false,
  });
  if (result.stderr) process.stderr.write(result.stderr);
  if (result.status !== 0) {
    fail(`git ls-files exited ${result.status} while preparing redacted forbidden-token scan`);
    return [];
  }
  return result.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

function runForbiddenTokenScan(file) {
  section("Redacted Forbidden Token Scan");
  const tokens = readForbiddenTokenFile(file);
  const roots = publicScanRoots();
  console.log(`tokens=${tokens.length}`);
  console.log(`roots=${roots.length}`);
  const trackedFiles = collectTrackedFiles(roots);
  console.log(`tracked_files=${trackedFiles.length}`);
  if (trackedFiles.length === 0) {
    fail("redacted forbidden-token scan found no tracked files to inspect");
    return;
  }

  const matches = new Map(tokens.map(({ label }) => [label, { path: 0, content: 0 }]));
  trackedFiles.forEach((trackedFile) => {
    tokens.forEach(({ label, value }) => {
      if (trackedFile.includes(value)) {
        matches.get(label).path += 1;
      }
    });
    let content;
    try {
      content = fs.readFileSync(trackedFile, "utf8");
    } catch {
      return;
    }
    tokens.forEach(({ label, value }) => {
      if (content.includes(value)) {
        matches.get(label).content += 1;
      }
    });
  });

  let matched = false;
  for (const [label, counts] of matches) {
    if (counts.path > 0 || counts.content > 0) {
      matched = true;
      fail(
        `forbidden token label ${label} matched path_count=${counts.path} content_file_count=${counts.content}`,
      );
    }
  }
  if (!matched) {
    ok("redacted forbidden-token scan found zero path or content matches");
  }
}

function nonEmptyLines(output) {
  return (output ?? "").trim().split(/\r?\n/).filter(Boolean);
}

function uniqueRgPaths(output) {
  return [...new Set(nonEmptyLines(output).map((line) => line.split(":")[0]))];
}

if (contractPreview) {
  printContractPreview();
  process.exit(0);
}

if (!referenceOnly) {
  section("Active Ticket References");
  const activeTicketRefs = run("rg", ["-n", prefix, "tickets"], { allowExitCodes: [0, 1] });
  if (activeTicketRefs.status === 0) {
    fail(`active tickets still reference ${prefix}`);
  } else {
    ok(`no active ticket references for ${prefix}`);
  }

  section("Archived Tickets");
  const archivedTickets = listArchivedTickets();
  if (summary && archivedTickets.length > 0) {
    console.log(`first=${archivedTickets[0]}`);
    console.log(`last=${archivedTickets[archivedTickets.length - 1]}`);
  } else {
    archivedTickets.forEach((file) => console.log(file));
  }
  console.log(`count=${archivedTickets.length}`);
  if (archivedTickets.length === 0) fail(`no archived tickets found for ${prefix}`);
  if (expectedCount !== undefined) {
    const count = Number.parseInt(expectedCount, 10);
    if (!Number.isInteger(count) || String(count) !== expectedCount) {
      fail(`invalid --expected-count value: ${expectedCount}`);
    } else if (archivedTickets.length !== count) {
      fail(`expected ${count} archived tickets, found ${archivedTickets.length}`);
    } else {
      ok(`archived ticket count matches expected ${count}`);
    }
  }
  if (expectedTicketNames) {
    const actual = new Set(archivedTickets);
    const expected = new Set(expectedTicketNames);
    const missing = expectedTicketNames.filter((file) => !actual.has(file));
    const unexpected = archivedTickets.filter((file) => !expected.has(file));
    missing.forEach((file) => fail(`expected archived ticket missing: ${file}`));
    unexpected.forEach((file) => fail(`unexpected archived ticket for expected set: ${file}`));
    if (missing.length === 0 && unexpected.length === 0) {
      ok("archived ticket names match expected set");
    }
  }

  section("Archived Ticket Status And Outcome");
  const validArchivedTicketCount = archivedTickets.filter(reportStatusOutcome).length;
  if (summary && archivedTickets.length > 0 && validArchivedTicketCount === archivedTickets.length) {
    ok(`${validArchivedTicketCount}/${archivedTickets.length} archived tickets have valid status/outcome`);
  }
}

section("Reference Paths");
if (activeReference) {
  if (fs.existsSync(activeReference)) {
    fail(`${activeReference} still exists`);
  } else {
    ok(`${activeReference} is absent`);
  }
} else {
  console.log("SKIP: no --active-reference provided");
}
if (archivedReference) {
  reportArchivedReference(archivedReference);
} else {
  console.log("SKIP: no --archived-reference provided");
}

section("Stale Live Path Sweep");
const livePatterns = [];
if (activeReference) livePatterns.push(`(?<!archive/)${escapeRegex(activeReference)}`);
if (prefix) livePatterns.push(`(?<!archive/)tickets/${escapeRegex(prefix)}`);
if (livePatterns.length > 0) {
  const sweepRoots = existingRoots(["specs", "tickets", "docs", "apps", "games", "scripts"]);
  if (sweepRoots.length === 0) {
    fail("stale live-path sweep found no existing roots to inspect");
  } else {
    const staleSweep = run(
      "rg",
      ["-n", "-P", livePatterns.join("|"), ...sweepRoots],
      { allowExitCodes: [0, 1, 2], quietStdout: summary },
    );
    if (staleSweep.status === 0) fail("stale live-path references were found");
    if (summary && staleSweep.stdout) process.stdout.write(staleSweep.stdout);
  }
} else {
  console.log("SKIP: no --active-reference or --ticket-prefix provided");
}

if (forbiddenTokenFile) {
  runForbiddenTokenScan(forbiddenTokenFile);
}

if (!referenceOnly) {
  section("Archive Reference Sweep");
  const archivePatterns = [`archive/tickets/${escapeRegex(prefix)}`];
  if (archivedReference) archivePatterns.push(escapeRegex(archivedReference));
  const sweepRoots = existingRoots(["specs", "docs", "apps", "games", "scripts"]);
  if (sweepRoots.length === 0) {
    fail("archive reference sweep found no existing roots to inspect");
  } else {
    const archiveSweep = run(
      "rg",
      ["-n", "-P", archivePatterns.join("|"), ...sweepRoots],
      { allowExitCodes: [0, 1, 2], quietStdout: summary },
    );
    if (summary) {
      const count = nonEmptyLines(archiveSweep.stdout).length;
      console.log(`matches=${count}`);
      if (count > 0 && count <= 10) {
        uniqueRgPaths(archiveSweep.stdout).forEach((file) => {
          console.log(`match_path=${file}`);
        });
      }
    }
  }

  section("Commit Ledger");
  const commitLedger = run("git", ["log", "--oneline", `--grep=${prefix}`, "--all"], {
    allowExitCodes: [0],
  });
  if (ledgerFormat === "compact") {
    section("Compact Commit Ledger");
    printCompactLedger(commitLedger.stdout ?? "");
  }
}

section("Git Status");
run("git", ["status", "--short"], { allowExitCodes: [0] });
run("git", ["diff", "--cached", "--name-status"], { allowExitCodes: [0] });

process.exitCode = failures > 0 ? 1 : 0;

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function printCompactLedger(logOutput) {
  const ticketPattern = new RegExp(`\\b${escapeRegex(prefix)}-(\\d+)\\b`);
  const rows = [];
  logOutput
    .trim()
    .split(/\r?\n/)
    .filter(Boolean)
    .forEach((line) => {
      const match = /^(?<sha>[0-9a-f]+)\s+(?<subject>.+)$/.exec(line);
      if (!match?.groups) return;
      const ticketMatch = ticketPattern.exec(match.groups.subject);
      if (!ticketMatch) return;
      rows.push({ number: ticketMatch[1], sha: match.groups.sha });
    });

  if (rows.length === 0) {
    fail(`no commit ledger rows matched ${prefix}-NNN`);
    return;
  }

  rows
    .sort((left, right) => Number(left.number) - Number(right.number))
    .reduce((lines, row, index) => {
      const item = `${row.number} ${row.sha}`;
      const lineIndex = Math.floor(index / 5);
      lines[lineIndex] = lines[lineIndex] ? `${lines[lineIndex]}, ${item}` : item;
      return lines;
    }, [])
    .forEach((line) => console.log(line));
}
