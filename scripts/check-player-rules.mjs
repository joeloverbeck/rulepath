// Fail-closed catalog-to-player-rules validation.
//
// The check spans three inert surfaces:
//   1. `crates/wasm-api/src/lib.rs` catalog consts,
//   2. `games/<game_id>/docs/HOW-TO-PLAY.md` source docs,
//   3. `apps/web/public/rules/<game_id>.md` generated assets.
//
// It does not read match state, viewer IDs, private views, actions, effects, or
// replay data.

import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const DEFAULT_ROOT = path.resolve(SCRIPT_DIR, "..");
const ROOT = process.env.RULEPATH_ROOT ?? DEFAULT_ROOT;
const WASM_API = process.env.RULEPATH_WASM_API ?? path.join(ROOT, "crates/wasm-api/src/lib.rs");
const GAMES_DIR = process.env.RULEPATH_GAMES_DIR ?? path.join(ROOT, "games");
const RULES_DIR = process.env.RULEPATH_WEB_RULES_DIR ?? path.join(ROOT, "apps/web/public/rules");
const GAME_ID_FILTER = (process.env.RULEPATH_PLAYER_RULES_GAME_IDS ?? "")
  .split(",")
  .map((id) => id.trim())
  .filter(Boolean);

const CATALOG_RE = /const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g;
const HIDDEN_INFO_GAMES = new Set([
  "high_card_duel",
  "secret_draft",
  "poker_lite",
  "plain_tricks",
  "masked_claims",
  "flood_watch",
  "event_frontier",
]);
const FORBIDDEN_TAG_RE = /<\s*(script|iframe|embed|object)\b/i;
const FORBIDDEN_ATTR_RE = /\s(?:on[a-z]+|style)\s*=/i;
const BEHAVIOR_HEADING_RE = /^#{2,4}\s+(Triggers?|Selectors?|Conditions?|Action schemas?|Validation rules?|Scoring logic|Visibility filters?)\s*$/im;

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
  "Source notes for maintainers",
];

const failures = [];

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

function sectionBody(markdown, section) {
  const escaped = section.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = new RegExp(`^##\\s+${escaped}\\s*$`, "m").exec(markdown);
  if (!match) return "";
  const rest = markdown.slice(match.index + match[0].length);
  const next = rest.search(/^##\s+/m);
  return next === -1 ? rest : rest.slice(0, next);
}

function extractFormalVersion(markdown) {
  return markdown.match(/Formal rules version checked:\s*`([^`]+)`/)?.[1] ?? null;
}

function extractRulesVersion(markdown) {
  return markdown.match(/^Rules version:\s*`([^`]+)`\s*$/m)?.[1] ?? null;
}

function checkMarkdownShape(game, markdown, sourcePath) {
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

  if (markdown.startsWith("---\n")) {
    failures.push(`${game.id}: YAML front matter is forbidden`);
  }

  if (FORBIDDEN_TAG_RE.test(markdown)) {
    failures.push(`${game.id}: raw script/iframe/embed/object tags are forbidden`);
  }

  if (FORBIDDEN_ATTR_RE.test(markdown)) {
    failures.push(`${game.id}: raw event-handler/style attributes are forbidden`);
  }

  if (BEHAVIOR_HEADING_RE.test(markdown)) {
    failures.push(`${game.id}: behavior-looking structured heading is forbidden`);
  }

  const hiddenInfo = sectionBody(markdown, "Hidden information and reveal timing");
  const saysNotApplicable = /not applicable/i.test(hiddenInfo);
  if (HIDDEN_INFO_GAMES.has(game.id)) {
    if (saysNotApplicable || hiddenInfo.trim().length < 80) {
      failures.push(`${game.id}: hidden-info game needs real reveal-timing prose`);
    }
  } else if (!saysNotApplicable) {
    failures.push(`${game.id}: perfect-information game must mark hidden information not applicable`);
  }
}

async function readRequired(filePath, gameId, label) {
  try {
    return await readFile(filePath, "utf8");
  } catch {
    failures.push(`${gameId}: missing ${label} ${filePath}`);
    return null;
  }
}

const catalog = await readCatalog();
const selectedCatalog =
  GAME_ID_FILTER.length === 0 ? catalog : catalog.filter((game) => GAME_ID_FILTER.includes(game.id));
for (const id of GAME_ID_FILTER) {
  if (!catalog.some((game) => game.id === id)) {
    failures.push(`unknown game id filter: ${id}`);
  }
}

for (const game of selectedCatalog) {
  const howToPath = path.join(GAMES_DIR, game.id, "docs/HOW-TO-PLAY.md");
  const formalRulesPath = path.join(GAMES_DIR, game.id, "docs/RULES.md");
  const generatedPath = path.join(RULES_DIR, `${game.id}.md`);

  const howTo = await readRequired(howToPath, game.id, "player rules doc");
  const formalRules = await readRequired(formalRulesPath, game.id, "formal rules doc");
  const generated = await readRequired(generatedPath, game.id, "generated rules asset");

  if (!howTo) continue;

  checkMarkdownShape(game, howTo, howToPath);

  const citedVersion = extractFormalVersion(howTo);
  const formalVersion = formalRules ? extractRulesVersion(formalRules) : null;

  if (!citedVersion) {
    failures.push(`${game.id}: missing Formal rules version checked metadata`);
  } else if (!formalVersion) {
    failures.push(`${game.id}: missing literal Rules version in ${formalRulesPath}`);
  } else if (citedVersion !== formalVersion) {
    failures.push(`${game.id}: Formal rules version checked \`${citedVersion}\` does not match RULES.md \`${formalVersion}\``);
  }

  if (generated !== null && generated !== howTo) {
    failures.push(`${game.id}: generated asset ${generatedPath} is stale`);
  }
}

if (failures.length > 0) {
  console.error("player-rules check failed:");
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}

console.log(`player-rules check passed — ${selectedCatalog.length} catalog games validated`);
