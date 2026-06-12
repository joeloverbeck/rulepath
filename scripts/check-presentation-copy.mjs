import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { readdir, readFile } from "node:fs/promises";

// Fail-closed normal-play copy guard.
//
// Scope: board components and shared play-surface components under
// `apps/web/src/components`. DevPanel, ReplayViewer, tests, generated public
// rules, and Rust-origin strings are intentionally out of scope. This guard
// catches source literals that make normal play surfaces debug-first: engine
// vocabulary, transport vocabulary, and raw internal ids. Machine-readable
// props such as test ids, template keys, CSS class names, and rule ids are
// exempt by line-level context.

const repoRoot = process.cwd();
const componentRoot = join(repoRoot, "apps/web/src/components");
const scannedNames = /(?:Board|DeckFlowPanel|ActionPathBuilder|OutcomeExplanationPanel)\.tsx$/;
const debugTerms = [/\bRust\b/, /\bWASM\b/, /\bprojection\b/i, /\bredacted\b/i, /\bpayload\b/i];
const rawIdentifier = /\b(?:ef_[a-z0-9_]+|site_[a-z0-9_]+|faction_[a-z0-9_]+|seat_[0-9]+|[a-z]+_[a-z0-9_]+\/[a-z0-9_]+)\b/;
const exemptLine =
  /^\s*\/\/|data-testid|className|gameId|templateKey|ruleIds|decisiveCause|resultKind|type ===|payload\.|effect\.payload|metadata|segment|variant_id|rules_version|terminal_kind|private_view|active_seat|terminal_rationale|current_card|next_public_card|display_name|item_id|tile_id|slot_id|cell_id|district_/;

if (process.argv.includes("--self-test")) {
  await runSelfTest();
} else {
  const files = await collectFiles(componentRoot);
  const failures = [];
  for (const file of files) {
    const source = await readFile(file, "utf8");
    failures.push(...scanSource(relative(repoRoot, file), source));
  }
  finish(failures, `presentation-copy check passed - ${files.length} play-surface files scanned`);
}

async function collectFiles(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await collectFiles(path)));
    } else if (entry.isFile() && scannedNames.test(entry.name)) {
      files.push(path);
    }
  }
  return files;
}

function scanSource(file, source) {
  const failures = [];
  const lines = source.split(/\r?\n/);
  lines.forEach((line, index) => {
    if (exemptLine.test(line)) {
      return;
    }
    const lineNumber = index + 1;
    for (const term of debugTerms) {
      if (term.test(line)) {
        failures.push(`${file}:${lineNumber}: debug vocabulary in play-surface copy: ${line.trim()}`);
        break;
      }
    }
    if (looksLikeJsxText(line) && rawIdentifier.test(line)) {
      failures.push(`${file}:${lineNumber}: raw internal identifier in play-surface copy: ${line.trim()}`);
    }
  });
  return failures;
}

function looksLikeJsxText(line) {
  return />[^<{]*</.test(line);
}

async function runSelfTest() {
  const root = await mkdtemp(join(tmpdir(), "rulepath-copy-"));
  try {
    const violation = join(root, "SampleBoard.tsx");
    await writeFile(
      violation,
      `export function SampleBoard() {
  return <section><h2>Rust legal choices</h2><p>site_gatehouse is raw.</p></section>;
}
`,
    );
    const devPanel = join(root, "DevPanel.tsx");
    await writeFile(devPanel, `export function DevPanel() { return <p>Rust diagnostics payload</p>; }\n`);
    const source = await readFile(violation, "utf8");
    const failures = scanSource("SampleBoard.tsx", source);
    if (failures.length < 2 || !failures.some((item) => item.includes("SampleBoard.tsx:2"))) {
      console.error("presentation-copy self-test failed: seeded violation was not diagnosed");
      process.exit(1);
    }
    const devFailures = scannedNames.test("DevPanel.tsx")
      ? scanSource("DevPanel.tsx", await readFile(devPanel, "utf8"))
      : [];
    if (devFailures.length > 0) {
      console.error("presentation-copy self-test failed: dev panel should be outside scanned naming scope");
      console.error(devFailures.join("\n"));
      process.exit(1);
    }
    console.log("presentation-copy self-test passed");
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

function finish(failures, successMessage) {
  if (failures.length > 0) {
    console.error("presentation-copy check failed:");
    console.error(failures.join("\n"));
    process.exit(1);
  }
  console.log(successMessage);
}
