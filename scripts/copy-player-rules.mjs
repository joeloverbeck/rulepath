// Copy player-facing rules docs into the web public asset tree.
//
// Source of truth:
//   games/<game_id>/docs/HOW-TO-PLAY.md
// Generated asset:
//   apps/web/public/rules/<game_id>.md
//
// The copied Markdown is inert presentation content. This script validates the
// same safety shape as the check script before writing assets.

import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import path from "node:path";
import { fileURLToPath } from "node:url";

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const DEFAULT_ROOT = path.resolve(SCRIPT_DIR, "..");
const ROOT = process.env.RULEPATH_ROOT ?? DEFAULT_ROOT;
const WASM_API = process.env.RULEPATH_WASM_API ?? path.join(ROOT, "crates/wasm-api/src/constants.rs");
const GAMES_DIR = process.env.RULEPATH_GAMES_DIR ?? path.join(ROOT, "games");
const RULES_DIR = process.env.RULEPATH_WEB_RULES_DIR ?? path.join(ROOT, "apps/web/public/rules");
const GAME_ID_FILTER = (process.env.RULEPATH_PLAYER_RULES_GAME_IDS ?? "")
  .split(",")
  .map((id) => id.trim())
  .filter(Boolean);

const failures = [];

const REQUIRED_SECTIONS = [
  "At a glance",
  "What you can see",
  "Setup",
  "On your turn",
  "Actions",
  "Scoring and winning",
  "Hidden information and reveal timing",
  "Common terms",
  "What this page is not",
];

const FORBIDDEN_TAG_RE = /<\s*(script|iframe|embed|object)\b/i;
const FORBIDDEN_ATTR_RE = /\s(?:on[a-z]+|style)\s*=/i;
const CATALOG_RE = /const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g;

async function readCatalog() {
  const src = await readFile(WASM_API, "utf8");
  const ids = new Map();
  const names = new Map();

  for (const m of src.matchAll(CATALOG_RE)) {
    const symbol = m[1];
    const value = m[2];
    if (symbol.endsWith("_DISPLAY_NAME")) {
      names.set(symbol.slice(0, -"_DISPLAY_NAME".length), value);
    } else {
      ids.set(symbol, value);
    }
  }

  const catalog = [];
  for (const [stem, id] of ids) {
    const display = names.get(stem);
    if (!display) {
      failures.push(`${WASM_API}: GAME_${stem} has no GAME_${stem}_DISPLAY_NAME`);
      continue;
    }
    catalog.push({ id, display });
  }

  if (catalog.length === 0) {
    failures.push(`${WASM_API}: parsed zero catalog games`);
  }

  return catalog;
}

function hasSection(markdown, section) {
  const escaped = section.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return new RegExp(`^##\\s+${escaped}\\s*$`, "m").test(markdown);
}

function extractFormalVersion(markdown) {
  return markdown.match(/Formal rules version checked:\s*`([^`]+)`/)?.[1] ?? null;
}

function validateMarkdown(game, markdown, sourcePath) {
  if (markdown.trim().length === 0) {
    failures.push(`${game.id}: ${sourcePath} is empty`);
  }

  if (!markdown.includes(`_Game ID: \`${game.id}\``)) {
    failures.push(`${game.id}: missing matching Game ID metadata`);
  }

  if (!markdown.startsWith(`# ${game.display} - How to Play`) && !markdown.startsWith(`# ${game.display} — How to Play`)) {
    failures.push(`${game.id}: title must start with the catalog display name`);
  }

  for (const section of REQUIRED_SECTIONS) {
    if (!hasSection(markdown, section)) {
      failures.push(`${game.id}: missing section "${section}"`);
    }
  }

  if (!extractFormalVersion(markdown)) {
    failures.push(`${game.id}: missing Formal rules version checked metadata`);
  }

  if (markdown.startsWith("---\n")) {
    failures.push(`${game.id}: YAML front matter is forbidden`);
  }

  if (FORBIDDEN_TAG_RE.test(markdown)) {
    failures.push(`${game.id}: raw script/iframe/embed/object tags are forbidden`);
  }

  if (FORBIDDEN_ATTR_RE.test(markdown)) {
    failures.push(`${game.id}: raw event-handler/style attributes are forbidden`);
  }
}

function sha256(text) {
  return createHash("sha256").update(text).digest("hex");
}

const catalog = await readCatalog();
const selectedCatalog =
  GAME_ID_FILTER.length === 0 ? catalog : catalog.filter((game) => GAME_ID_FILTER.includes(game.id));
for (const id of GAME_ID_FILTER) {
  if (!catalog.some((game) => game.id === id)) {
    failures.push(`unknown game id filter: ${id}`);
  }
}
const manifest = [];

for (const game of selectedCatalog) {
  const sourcePath = path.join(GAMES_DIR, game.id, "docs/HOW-TO-PLAY.md");
  let markdown;
  try {
    markdown = await readFile(sourcePath, "utf8");
  } catch {
    failures.push(`${game.id}: missing ${sourcePath}`);
    continue;
  }

  validateMarkdown(game, markdown, sourcePath);
  manifest.push({
    game_id: game.id,
    display_name: game.display,
    rules_asset_path: `/rules/${game.id}.md`,
    source_rules_version_checked: extractFormalVersion(markdown),
    sha256: sha256(markdown),
  });
}

if (failures.length > 0) {
  console.error("copy-player-rules failed:");
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}

await mkdir(RULES_DIR, { recursive: true });

for (const game of selectedCatalog) {
  const sourcePath = path.join(GAMES_DIR, game.id, "docs/HOW-TO-PLAY.md");
  const targetPath = path.join(RULES_DIR, `${game.id}.md`);
  const markdown = await readFile(sourcePath, "utf8");
  await writeFile(targetPath, markdown, "utf8");
}

if (GAME_ID_FILTER.length === 0) {
  await writeFile(path.join(RULES_DIR, "manifest.json"), `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
}

console.log(`copied player rules for ${selectedCatalog.length} catalog games`);
