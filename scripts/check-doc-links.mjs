import { readFile, readdir } from "node:fs/promises";
import { existsSync } from "node:fs";
import { dirname, extname, join, normalize, relative } from "node:path";

const roots = ["docs", "specs"];
const repoRoot = process.cwd();
const failures = [];

async function collectMarkdownFiles(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await collectMarkdownFiles(path)));
    } else if (entry.isFile() && extname(entry.name) === ".md") {
      files.push(path);
    }
  }

  return files;
}

function slugify(heading) {
  return heading
    .trim()
    .toLowerCase()
    .replace(/[`*_]/g, "")
    .replace(/[^\p{Letter}\p{Number}\s-]/gu, "")
    .replace(/\s+/g, "-");
}

function anchorSet(markdown) {
  const anchors = new Set();
  for (const line of markdown.split(/\r?\n/)) {
    const match = /^(#{1,6})\s+(.+?)\s*$/.exec(line);
    if (match) {
      anchors.add(slugify(match[2]));
    }
  }
  return anchors;
}

function isExternal(target) {
  return /^[a-z][a-z0-9+.-]*:/i.test(target) || target.startsWith("mailto:");
}

function splitTarget(target) {
  const hashIndex = target.indexOf("#");
  if (hashIndex === -1) {
    return { pathPart: target, anchor: "" };
  }

  return {
    pathPart: target.slice(0, hashIndex),
    anchor: decodeURIComponent(target.slice(hashIndex + 1)),
  };
}

const markdownFiles = (await Promise.all(roots.map(collectMarkdownFiles))).flat();
const markdownByPath = new Map();
const anchorsByPath = new Map();

for (const file of markdownFiles) {
  const markdown = await readFile(file, "utf8");
  markdownByPath.set(file, markdown);
  anchorsByPath.set(file, anchorSet(markdown));
}

for (const [file, markdown] of markdownByPath) {
  const linkPattern = /(?<!!)\[[^\]]+\]\(([^)\s]+)(?:\s+"[^"]*")?\)/g;

  for (const match of markdown.matchAll(linkPattern)) {
    const rawTarget = match[1];
    if (isExternal(rawTarget) || rawTarget.startsWith("#")) {
      continue;
    }

    const { pathPart, anchor } = splitTarget(rawTarget);
    if (!pathPart) {
      continue;
    }

    const resolved = normalize(join(dirname(file), pathPart));
    const relativePath = relative(repoRoot, resolved);

    if (relativePath.startsWith("..")) {
      failures.push(`${file}: link escapes repository: ${rawTarget}`);
      continue;
    }

    if (!existsSync(resolved)) {
      failures.push(`${file}: missing link target: ${rawTarget}`);
      continue;
    }

    if (anchor && extname(resolved) === ".md") {
      const targetMarkdown = await readFile(resolved, "utf8");
      const anchors = anchorsByPath.get(resolved) ?? anchorSet(targetMarkdown);
      if (!anchors.has(anchor)) {
        failures.push(`${file}: missing anchor ${rawTarget}`);
      }
    }
  }
}

if (failures.length > 0) {
  console.error(failures.join("\n"));
  process.exit(1);
}

console.log(`Checked ${markdownFiles.length} markdown files`);
