#!/usr/bin/env node
// Validates ci/games.json against the games/ directory (the canonical game
// enumeration) and the per-game e2e files it names, so the Gate 1 matrix can
// never silently drift from the real game set. With --emit it prints the
// minified manifest as a single line for use as a GitHub Actions matrix
// (`matrix=<json>` when GITHUB_OUTPUT is set, otherwise bare JSON to stdout).
//
//   node scripts/check-ci-games.mjs           # validate only (CI drift gate)
//   node scripts/check-ci-games.mjs --emit     # validate, then emit the matrix

import { readdirSync, readFileSync, existsSync, appendFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = join(root, "ci", "games.json");
const gamesDir = join(root, "games");
const e2eDir = join(root, "apps", "web", "e2e");

const errors = [];

const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
if (!Array.isArray(manifest)) {
  console.error("ci/games.json must be a JSON array of game entries.");
  process.exit(1);
}

const gameDirs = readdirSync(gamesDir, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name)
  .sort();

const manifestIds = manifest.map((entry) => entry.id);

// Shape + e2e-file validation, one entry at a time.
const seen = new Set();
for (const entry of manifest) {
  if (typeof entry.id !== "string" || entry.id.length === 0) {
    errors.push(`entry ${JSON.stringify(entry)} is missing a string "id"`);
    continue;
  }
  if (seen.has(entry.id)) {
    errors.push(`duplicate manifest entry for "${entry.id}"`);
  }
  seen.add(entry.id);
  if (typeof entry.sim_flags !== "string") {
    errors.push(`"${entry.id}" must have a string "sim_flags" (use "" for none)`);
  }
  if (typeof entry.e2e !== "string") {
    errors.push(`"${entry.id}" must have a string "e2e" (use "" for none)`);
  } else if (entry.e2e.length > 0 && !existsSync(join(e2eDir, entry.e2e))) {
    errors.push(`"${entry.id}" names e2e file "${entry.e2e}" which does not exist in apps/web/e2e/`);
  }
}

// Set-equality drift check: every game dir has a manifest row and vice versa.
const idSet = new Set(manifestIds);
const dirSet = new Set(gameDirs);
for (const dir of gameDirs) {
  if (!idSet.has(dir)) {
    errors.push(`games/${dir} has no row in ci/games.json (add one when you add a game)`);
  }
}
for (const id of manifestIds) {
  if (!dirSet.has(id)) {
    errors.push(`ci/games.json names "${id}" but games/${id} does not exist`);
  }
}

if (errors.length > 0) {
  console.error("ci/games.json drift detected:");
  for (const message of errors) {
    console.error(`  - ${message}`);
  }
  process.exit(1);
}

if (process.argv.includes("--emit")) {
  const json = JSON.stringify(manifest);
  if (process.env.GITHUB_OUTPUT) {
    appendFileSync(process.env.GITHUB_OUTPUT, `matrix=${json}\n`);
  } else {
    process.stdout.write(`${json}\n`);
  }
} else {
  console.log(`ci/games.json OK — ${manifest.length} games in sync with games/.`);
}
