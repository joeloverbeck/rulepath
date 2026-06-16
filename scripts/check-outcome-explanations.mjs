// Fail-closed catalog-to-outcome-explanation validation.
//
// The check spans inert contract surfaces only:
//   1. `crates/wasm-api/src/lib.rs` catalog consts,
//   2. `games/<game_id>/docs/UI.md` outcome explanation sections,
//   3. `games/<game_id>/docs/RULES.md` stable scoring/end rule IDs,
//   4. `apps/web/src/wasm/client.ts` viewer-safe rationale mirrors,
//   5. `apps/web/src/components/outcomeExplanationTemplates.ts` static copy keys.
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
const CLIENT_TS = process.env.RULEPATH_WEB_CLIENT_TS ?? path.join(ROOT, "apps/web/src/wasm/client.ts");
const TEMPLATES_TS =
  process.env.RULEPATH_OUTCOME_TEMPLATES_TS ?? path.join(ROOT, "apps/web/src/components/outcomeExplanationTemplates.ts");
const GAME_ID_FILTER = (process.env.RULEPATH_OUTCOME_GAME_IDS ?? "")
  .split(",")
  .map((id) => id.trim())
  .filter(Boolean);

const CATALOG_RE = /const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g;
const OUTCOME_SECTION = "Outcome / victory explanation";
const REQUIRED_UI_MARKERS = [
  { label: "terminal result variants", re: /terminal result variants?/i },
  { label: "decisive cause variants", re: /decisive cause|cause variant/i },
  { label: "per-player breakdown fields", re: /per-player|breakdown/i },
  { label: "hidden-info redaction rules", re: /hidden|no-leak|redaction|reveal/i },
  { label: "RULES.md rule IDs", re: /RULES\.md|rule ids?|[A-Z]+-(?:SCORE|END)-/i },
  { label: "web smoke coverage", re: /smoke/i },
];
const FORBIDDEN_TEMPLATE_PATTERNS = [
  { label: "YAML front matter", re: /^---\s*$/m },
  { label: "conditional/comparison logic", re: /\b(?:if|when|condition|valid_if)\b[^;\n]*(?:[<>]=?|===?|!==?|&&|\|\|)/i },
  { label: "selector/trigger/script-like field", re: /\b(?:selector|trigger|script|effect_script|foreach|loop|priority_expression|ai_condition)\b/i },
  { label: "tiebreak/order behavior", re: /\b(?:tiebreak_order|rank_order|compare(?:Score|Cards|Ranks|Strength)|resolveTiebreak)\b/i },
  { label: "TypeScript outcome decision helper", re: /\b(?:determineWinner|findWinningLine|scoreOutcome)\b/i },
  { label: "hidden-info leak marker", re: /\b(?:deck_tail|opponent_private|unrevealed_private|secret_commitment)\b/i },
  { label: "raw seat id in outcome copy", re: /\bseat_\d+\b/i },
];

const failures = [];

async function readOptional(filePath) {
  try {
    return await readFile(filePath, "utf8");
  } catch {
    return null;
  }
}

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
    catalog.push({ id, display, stem });
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

function pascalCase(gameId) {
  return gameId
    .split("_")
    .map((part) => part.slice(0, 1).toUpperCase() + part.slice(1))
    .join("");
}

function extractTemplateKeys(markdown, gameId) {
  const keys = new Set();
  const keyRe = new RegExp(`\\b${gameId}\\.[a-z0-9_.-]+\\b`, "g");
  for (const m of markdown.matchAll(keyRe)) {
    keys.add(m[0]);
  }
  return [...keys];
}

function checkUiDoc(game, uiDoc) {
  if (!hasSection(uiDoc, OUTCOME_SECTION)) {
    failures.push(`${game.id}: missing ${OUTCOME_SECTION} section in games/${game.id}/docs/UI.md`);
    return [];
  }

  const body = sectionBody(uiDoc, OUTCOME_SECTION);
  for (const marker of REQUIRED_UI_MARKERS) {
    if (!marker.re.test(body)) {
      failures.push(`${game.id}: ${OUTCOME_SECTION} section does not name ${marker.label}`);
    }
  }

  return extractTemplateKeys(body, game.id);
}

function checkRulesDoc(game, rulesDoc) {
  if (!/^## Scoring and accounting\s*$/m.test(rulesDoc)) {
    failures.push(`${game.id}: games/${game.id}/docs/RULES.md missing Scoring and accounting section`);
  }
  if (!/^## Terminal conditions\s*$/m.test(rulesDoc)) {
    failures.push(`${game.id}: games/${game.id}/docs/RULES.md missing Terminal conditions section`);
  }
  if (!/\b[A-Z]+-(?:SCORE|END|TERM)-[A-Z0-9]+(?:-[A-Z0-9]+)*\b/.test(rulesDoc)) {
    failures.push(`${game.id}: games/${game.id}/docs/RULES.md lacks stable scoring/end rule IDs`);
  }
}

function checkClientMirror(game, clientTs) {
  const pascal = pascalCase(game.id);
  const publicType = new RegExp(`export\\s+type\\s+${pascal}PublicView\\s*=\\s*\\{([\\s\\S]*?)\\n\\};`, "m").exec(clientTs)?.[1] ?? "";
  const gameOutcomeType = new RegExp(`\\b${pascal}\\w*(?:Outcome|Victory|Terminal)\\w*(?:Explanation|Rationale)\\b`);
  const outcomeField = /\b(?:outcome|victory|terminal)_(?:explanation|rationale)\b/i;
  if (!gameOutcomeType.test(clientTs) || !outcomeField.test(publicType)) {
    failures.push(`${game.id}: apps/web/src/wasm/client.ts lacks an outcome rationale type or field mirror`);
  }
}

function checkTemplatesFile(templatesTs, requiredTemplateKeys) {
  if (templatesTs === null) {
    failures.push(`${TEMPLATES_TS}: missing static outcome templates file`);
    return;
  }

  for (const pattern of FORBIDDEN_TEMPLATE_PATTERNS) {
    if (pattern.re.test(templatesTs)) {
      failures.push(`${TEMPLATES_TS}: forbidden ${pattern.label}`);
    }
  }

  for (const key of requiredTemplateKeys) {
    if (!templatesTs.includes(key)) {
      failures.push(`${TEMPLATES_TS}: missing template key ${key}`);
    }
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

const clientTs = (await readOptional(CLIENT_TS)) ?? "";
if (clientTs.length === 0) {
  failures.push(`${CLIENT_TS}: missing or empty TypeScript client mirror`);
}
const templatesTs = await readOptional(TEMPLATES_TS);
const requiredTemplateKeys = new Set();

for (const game of selectedCatalog) {
  const uiPath = path.join(GAMES_DIR, game.id, "docs/UI.md");
  const rulesPath = path.join(GAMES_DIR, game.id, "docs/RULES.md");
  const uiDoc = await readOptional(uiPath);
  const rulesDoc = await readOptional(rulesPath);

  if (uiDoc === null) {
    failures.push(`${game.id}: missing UI doc ${uiPath}`);
  } else {
    for (const key of checkUiDoc(game, uiDoc)) {
      requiredTemplateKeys.add(key);
    }
  }

  if (rulesDoc === null) {
    failures.push(`${game.id}: missing formal rules doc ${rulesPath}`);
  } else {
    checkRulesDoc(game, rulesDoc);
  }

  checkClientMirror(game, clientTs);
}

checkTemplatesFile(templatesTs, requiredTemplateKeys);

if (failures.length > 0) {
  console.error("outcome-explanations check failed:");
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}

console.log(`outcome-explanations check passed — ${selectedCatalog.length} catalog games validated`);
