import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { checkScaffoldingGovernance } from "./check-scaffolding-governance.mjs";

const testRoot = dirname(fileURLToPath(import.meta.url));
const fixtureRoot = join(testRoot, "testdata", "scaffolding-governance");

const legacyGames = [
  "race_to_n",
  "three_marks",
  "column_four",
  "directional_flip",
  "draughts_lite",
  "high_card_duel",
  "masked_claims",
  "flood_watch",
  "frontier_control",
  "event_frontier",
  "token_bazaar",
  "secret_draft",
  "poker_lite",
  "plain_tricks",
  "river_ledger",
  "briar_circuit",
  "vow_tide",
];

const registerIds = [
  "MSC-8C-001",
  "MSC-8C-002",
  "MSC-8C-003",
  "MSC-8C-004",
  "MSC-8C-005",
  "MSC-8C-006",
  "MSC-8C-007",
  "MSC-8C-008",
  "MSC-8C-009",
  "MSC-8C-010",
];

function baseReceipt() {
  return {
    schema_version: 1,
    legacy_8c_games: [...legacyGames],
    games: legacyGames.map((id) => ({
      id,
      coverage: "legacy-8c-covered",
      evidence_paths: [
        "archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md",
        "docs/MECHANICAL-SCAFFOLDING-REGISTER.md",
      ],
      register_entries_reviewed: [...registerIds],
      register_decisions: [],
      disposition: "legacy-8c-covered",
      prior_matching_games: [],
      follow_on_unit: null,
      no_follow_on_decision: "legacy-8c-covered",
      known_signal_dispositions: [
        {
          signal: "MSC-8C-001.effect-envelope-literal",
          decision: "legacy-reviewed",
          evidence: "docs/MECHANICAL-SCAFFOLDING-REGISTER.md",
        },
      ],
      compatibility: {
        hash_migration: "none",
        visibility_migration: "none",
        determinism_migration: "none",
        migration_authority: "none",
      },
    })),
  };
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function writeSyntheticRepo(root, fixture) {
  mkdirSync(join(root, "ci"), { recursive: true });
  mkdirSync(join(root, "docs"), { recursive: true });
  mkdirSync(join(root, "specs"), { recursive: true });
  mkdirSync(join(root, "archive", "specs"), { recursive: true });
  mkdirSync(join(root, "games"), { recursive: true });

  const ciGames = legacyGames.map((id) => ({ id, sim_flags: "", e2e: "" }));
  const receipt = baseReceipt();

  for (const id of legacyGames) {
    mkdirSync(join(root, "games", id, "src"), { recursive: true });
    writeFileSync(
      join(root, "games", id, "Cargo.toml"),
      `[package]\nname = "${id}"\n\n[dependencies]\nengine-core = { path = "../../crates/engine-core" }\n\n[dev-dependencies]\ngame-test-support = { path = "../../crates/game-test-support" }\n`,
    );
    writeFileSync(join(root, "games", id, "src", "lib.rs"), "\n");
  }

  writeFileSync(
    join(root, "archive", "specs", "unit-8c-mechanical-scaffolding-code-extraction.md"),
    "# Unit 8C\n",
  );
  writeFileSync(
    join(root, "docs", "MECHANICAL-SCAFFOLDING-REGISTER.md"),
    `${registerIds.map((id) => `### ${id} - synthetic entry\n- Entry id: synthetic, status \`promoted\`, owner test.\n`).join("\n")}\n`,
  );
  writeFileSync(join(root, "specs", "README.md"), "| Unit | Status |\n|---|---|\n| FSGOV-FOLLOW-1 | Planned |\n");

  applyMutation(root, ciGames, receipt, fixture.mutation);

  writeJson(join(root, "ci", "games.json"), ciGames);
  writeJson(join(root, "ci", "scaffolding-audits.json"), receipt);
}

function applyMutation(root, ciGames, receipt, mutation) {
  switch (mutation) {
    case undefined:
      return;
    case "missing-game":
      receipt.games.pop();
      return;
    case "missing-path":
      receipt.games[0].evidence_paths = ["archive/specs/missing-evidence.md"];
      return;
    case "unknown-id":
      receipt.games[0].register_entries_reviewed = ["MSC-BOGUS-999"];
      return;
    case "unqueued-prior-site":
      receipt.games[0].prior_matching_games = ["three_marks"];
      receipt.games[0].no_follow_on_decision = null;
      return;
    case "invalid-exception":
      receipt.games[0].disposition = "accepted-local-only";
      receipt.games[0].no_follow_on_decision = "MSC-8C-001";
      return;
    case "forbidden-legacy-claim":
      ciGames.push({ id: "new_game", sim_flags: "", e2e: "" });
      mkdirSync(join(root, "games", "new_game", "src"), { recursive: true });
      writeFileSync(join(root, "games", "new_game", "Cargo.toml"), "[package]\nname = \"new_game\"\n");
      writeFileSync(join(root, "games", "new_game", "src", "lib.rs"), "\n");
      receipt.games.push({
        ...baseReceipt().games[0],
        id: "new_game",
        coverage: "legacy-8c-covered",
      });
      return;
    case "unknown-field":
      receipt.games[0].unreviewed_escape_hatch = true;
      return;
    case "false-positive-local-behavior":
      writeFileSync(
        join(root, "games", "race_to_n", "src", "local_policy.rs"),
        "fn local_deal_reveal_projection_scoring_policy() -> bool { true }\n",
      );
      return;
    default:
      throw new Error(`unknown fixture mutation ${mutation}`);
  }
}

function runFixture(name) {
  const fixturePath = join(fixtureRoot, name, "fixture.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
  const root = mkdtempSync(join(tmpdir(), `scaffolding-governance-${name}-`));
  try {
    writeSyntheticRepo(root, fixture);
    return { fixture, result: checkScaffoldingGovernance({ root }) };
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

for (const name of readdirSync(fixtureRoot).sort()) {
  test(`scaffolding governance fixture: ${name}`, () => {
    const { fixture, result } = runFixture(name);
    if (fixture.expect === "pass") {
      assert.equal(result.ok, true, result.errors.join("\n"));
    } else {
      assert.equal(result.ok, false, "fixture should fail closed");
      assert.notEqual(result.errors.length, 0);
    }
  });
}
