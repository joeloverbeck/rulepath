#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

const args = parseArgs(process.argv.slice(2));
const prefix = args["ticket-prefix"];
const activeReference = args["active-reference"];
const archivedReference = args["archived-reference"];

if (!prefix) {
  console.error(
    "Usage: audit-series-closeout.mjs --ticket-prefix PREFIX " +
      "[--active-reference specs/name.md] " +
      "[--archived-reference archive/specs/name.md]",
  );
  process.exit(2);
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
  let hasValidStatus = false;
  let hasOutcome = false;
  let informal = false;

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
  });

  if (!hasValidStatus) fail(`${file} has no valid archival status line`);
  if (!hasOutcome) fail(`${file} has no ## Outcome`);
  if (informal) fail(`${file} contains informal status syntax`);
}

function parseArgs(argv) {
  const parsed = {};
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (!arg.startsWith("--")) {
      console.error(`Unexpected argument: ${arg}`);
      process.exit(2);
    }
    const key = arg.slice(2);
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

section("Archived Ticket Status And Outcome");
archivedTickets.forEach(reportStatusOutcome);

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
livePatterns.push(`(?<!archive/)tickets/${escapeRegex(prefix)}`);
const staleSweep = run(
  "rg",
  ["-n", "-P", livePatterns.join("|"), "specs", "tickets", "docs", "apps", "scripts"],
  { allowExitCodes: [0, 1, 2] },
);
if (staleSweep.status === 0) fail("stale live-path references were found");

section("Archive Reference Sweep");
const archivePatterns = [`archive/tickets/${escapeRegex(prefix)}`];
if (archivedReference) archivePatterns.push(escapeRegex(archivedReference));
run(
  "rg",
  ["-n", "-P", archivePatterns.join("|"), "specs", "docs", "apps", "scripts"],
  { allowExitCodes: [0, 1, 2] },
);

section("Commit Ledger");
run("git", ["log", "--oneline", `--grep=${prefix}`, "--all"], { allowExitCodes: [0] });

section("Git Status");
run("git", ["status", "--short"], { allowExitCodes: [0] });
run("git", ["diff", "--cached", "--name-status"], { allowExitCodes: [0] });

process.exitCode = failures > 0 ? 1 : 0;

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
