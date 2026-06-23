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
const ledgerFormat = args["ledger-format"];
const referenceOnly = args["reference-only"] === true;

if (!prefix && !referenceOnly) {
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
      "[--expected-ticket-range PREFIX-001..020] " +
      "[--ledger-format compact]\n" +
      "       audit-series-closeout.mjs --reference-only " +
      "[--active-reference specs/name.md] " +
      "--archived-reference archive/specs/name.md",
  );
}

function run(cmd, cmdArgs, options = {}) {
  console.log(`$ ${[cmd, ...cmdArgs].join(" ")}`);
  const result = spawnSync(cmd, cmdArgs, {
    cwd: process.cwd(),
    encoding: "utf8",
    shell: false,
  });
  if (result.stdout) process.stdout.write(result.stdout);
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
    return;
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
  } else {
    console.log(matches.join("\n"));
  }
  if (!hasValidStatus) fail(`${file} has no valid archival status line`);
  if (!hasOutcome) fail(`${file} has no ## Outcome`);
  if (informal) fail(`${file} contains informal status syntax`);
}

function reportArchivedReference(file) {
  const lines = readLines(file);
  if (!lines) {
    fail(`${file} is missing`);
    return;
  }

  const validStatus = /^\*\*Status\*\*: .*?(COMPLETED|REJECTED|DEFERRED|NOT IMPLEMENTED)$|^\| Status \| `?Done`? \|/;
  const informalStatus = /^\s*-?\s*\*\*Status\*\*:\s*(Done|ACCEPTED)|^\s*- \*\*Status:\*\*/;
  const boldSpecStatus = /^\| \*\*Status\*\* \| `?Done`? \|/;
  let hasValidStatus = false;
  let hasOutcome = false;
  let informal = false;
  let nearMiss = false;

  lines.forEach((line, index) => {
    if (validStatus.test(line) || /^## Outcome/.test(line)) {
      console.log(`${file}:${index + 1}:${line}`);
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
  });

  if (!hasValidStatus) fail(`${file} has no valid archival status line`);
  if (!hasOutcome) fail(`${file} has no ## Outcome`);
  if (informal) fail(`${file} contains informal status syntax`);
  if (nearMiss) {
    fail(
      `${file} uses a bolded spec status label; use exact row: | Status | \`Done\` |`,
    );
  }
}

function parseArgs(argv) {
  const parsed = {};
  const flagArgs = new Set(["reference-only"]);
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
  archivedTickets.forEach((file) => console.log(file));
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
    unexpected.forEach((file) => fail(`unexpected archived ticket for range: ${file}`));
    if (missing.length === 0 && unexpected.length === 0) {
      ok(`archived ticket names match expected range ${expectedTicketRange}`);
    }
  }

  section("Archived Ticket Status And Outcome");
  archivedTickets.forEach(reportStatusOutcome);
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
  const staleSweep = run(
    "rg",
    ["-n", "-P", livePatterns.join("|"), "specs", "tickets", "docs", "apps", "games", "scripts"],
    { allowExitCodes: [0, 1, 2] },
  );
  if (staleSweep.status === 0) fail("stale live-path references were found");
} else {
  console.log("SKIP: no --active-reference or --ticket-prefix provided");
}

if (!referenceOnly) {
  section("Archive Reference Sweep");
  const archivePatterns = [`archive/tickets/${escapeRegex(prefix)}`];
  if (archivedReference) archivePatterns.push(escapeRegex(archivedReference));
  run(
    "rg",
    ["-n", "-P", archivePatterns.join("|"), "specs", "docs", "apps", "games", "scripts"],
    { allowExitCodes: [0, 1, 2] },
  );

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
