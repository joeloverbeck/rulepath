// Catalog-vs-docs drift check.
//
// The web-shell catalog (the set of browser-exposed games) is registered in
// `crates/wasm-api/src/lib.rs` as `GAME_*` / `GAME_*_DISPLAY_NAME` const pairs.
// When a new game ships, its human-orientation doc surfaces must grow with it.
// History (the Gate 8 and Gate 9 aftermath passes) shows three surfaces drift:
//
//   1. the `apps/web/README.md` intro catalog list,
//   2. the root `README.md` "current official games are" list,
//   3. the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet.
//
// Each surface has a clean, authoritative source of truth, so each is checked
// mechanically here. The `apps/web/README.md` **Shell Surface renderer** bullet
// is intentionally NOT checked: it has no uniform source of truth (`race_to_n`
// is described separately as a public board, not a "board renderer"), so a
// mechanical check would false-positive. That surface is enforced by process —
// `docs/OFFICIAL-GAME-CONTRACT.md` §10 and the spec §Documentation-updates law.

import { readFile } from "node:fs/promises";

const WASM_API = "crates/wasm-api/src/lib.rs";
const WEB_README = "apps/web/README.md";
const ROOT_README = "README.md";
const WEB_PACKAGE = "apps/web/package.json";

// Smoke files in `smoke:e2e` that are not per-game (no catalog entry).
const NON_GAME_SMOKE = new Set(["shell", "a11y-noleak"]);

const failures = [];

// --- Catalog (authoritative game set) ---------------------------------------

async function readCatalog() {
  const src = await readFile(WASM_API, "utf8");
  const ids = new Map(); // stem -> id
  const names = new Map(); // stem -> display name

  const constRe = /const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g;
  for (const m of src.matchAll(constRe)) {
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
    failures.push(`${WASM_API}: parsed zero catalog games — has the registration format changed?`);
  }
  return catalog;
}

// --- Scoping helpers --------------------------------------------------------

// Intro = everything before the first `## ` section heading.
function introSection(readme) {
  const idx = readme.search(/^## /m);
  return idx === -1 ? readme : readme.slice(0, idx);
}

// The root README list paragraph anchored on "current official games are".
function officialGamesParagraph(readme) {
  const anchor = readme.indexOf("current official games are");
  if (anchor === -1) return null;
  const fence = readme.indexOf("```", anchor);
  return readme.slice(anchor, fence === -1 ? undefined : fence);
}

// The `smoke:e2e` bullet inside the "## Smoke Layers" section.
function smokeE2eBullet(readme) {
  const section = readme.indexOf("## Smoke Layers");
  if (section === -1) return null;
  const lines = readme.slice(section).split("\n");
  const start = lines.findIndex((l) => /^- `smoke:e2e`/.test(l));
  if (start === -1) return null;
  const collected = [lines[start]];
  for (let i = start + 1; i < lines.length; i++) {
    if (/^- `/.test(lines[i]) || /^## /.test(lines[i])) break;
    collected.push(lines[i]);
  }
  return collected.join("\n");
}

// --- Checks -----------------------------------------------------------------

function checkPresence(catalog, scope, surfaceLabel, present) {
  if (scope === null) {
    failures.push(`${surfaceLabel}: anchor not found — has the section moved or been renamed?`);
    return;
  }
  for (const game of catalog) {
    if (!present(game, scope)) {
      failures.push(`${surfaceLabel}: missing \`${game.id}\` / ${game.display}`);
    }
  }
}

async function checkSmokeLayer(catalog) {
  const pkg = JSON.parse(await readFile(WEB_PACKAGE, "utf8"));
  const script = pkg.scripts?.["smoke:e2e"] ?? "";
  const slugs = [...script.matchAll(/e2e\/([a-z0-9-]+)\.smoke\.mjs/g)].map((m) => m[1]);

  const readme = await readFile(WEB_README, "utf8");
  const bullet = smokeE2eBullet(readme);
  if (bullet === null) {
    failures.push(`${WEB_README} Smoke Layers: \`smoke:e2e\` bullet not found`);
    return;
  }

  for (const slug of slugs) {
    if (NON_GAME_SMOKE.has(slug)) continue;
    const id = slug.replace(/-/g, "_");
    const game = catalog.find((g) => g.id === id);
    if (!game) {
      failures.push(
        `${WEB_PACKAGE} smoke:e2e: \`${slug}.smoke.mjs\` maps to no catalog game (\`${id}\`) — stale smoke file or renamed game?`,
      );
      continue;
    }
    if (!bullet.includes(game.display)) {
      failures.push(`${WEB_README} Smoke Layers smoke:e2e bullet: missing ${game.display} (chained in ${WEB_PACKAGE})`);
    }
  }
}

// --- Main -------------------------------------------------------------------

const catalog = await readCatalog();

if (catalog.length > 0) {
  const webReadme = await readFile(WEB_README, "utf8");
  const rootReadme = await readFile(ROOT_README, "utf8");

  checkPresence(
    catalog,
    introSection(webReadme),
    `${WEB_README} intro catalog list`,
    (game, scope) => scope.includes(game.id) || scope.includes(game.display),
  );
  checkPresence(
    catalog,
    officialGamesParagraph(rootReadme),
    `${ROOT_README} "current official games are" list`,
    (game, scope) => scope.includes(game.id),
  );
  await checkSmokeLayer(catalog);
}

if (failures.length > 0) {
  console.error("catalog-docs check failed:");
  for (const f of failures) console.error(`  - ${f}`);
  console.error(
    "\nA registered game is missing from a human-orientation doc surface. Update the named surface(s); see docs/OFFICIAL-GAME-CONTRACT.md §10/§12.",
  );
  process.exit(1);
}

console.log(`catalog-docs check passed — ${catalog.length} games reflected in intro, root, and smoke surfaces`);
