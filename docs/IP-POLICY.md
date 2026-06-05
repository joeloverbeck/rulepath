# Rulepath IP Policy

Status: public repository and public website IP law.

This is operational engineering policy, not legal advice. It is deliberately conservative because Rulepath is a public portfolio, not a risky adaptation showcase.

## 1. Public allowed

Public Rulepath may include:

- public-domain/classic games;
- original games;
- permissioned games;
- neutral implementations of abstract mechanics;
- original rules summaries;
- original graphics/icons/assets;
- compatible open-licensed assets with license notes;
- AI-generated assets after review;
- citations/links to public rules sources;
- documented variants;
- generic engine contracts and earned primitives;
- public benchmark traces containing no proprietary data.

## 2. Public forbidden unless explicitly permissioned

Public Rulepath must not include:

- copied rulebook prose;
- proprietary card text;
- proprietary board art;
- proprietary icons;
- proprietary fonts or font files without verified redistribution rights;
- screenshots or scans of commercial components;
- trademark-forward presentation;
- trade-dress mimicry;
- private licensed modules;
- hidden licensed modules bundled in public JS/WASM;
- public test fixtures containing proprietary data;
- public docs that make Rulepath look secretly focused on private licensed games.

## 3. Game ideas versus expression

Game ideas, methods, procedures, systems, and mechanics are different from expressive text, art, logos, component design, and presentation.

Rulepath policy:

- mechanics may be implemented when lawful and safely presented;
- expression must be original, permissioned, or compatibly licensed;
- public naming and visuals must avoid affiliation confusion;
- when unsure, omit and request human/legal review.

## 4. Public rules documentation

Every public game must have:

- original-language rules summary;
- source notes with URLs and consulted dates;
- chosen variant statement;
- known ambiguities and resolutions;
- rule coverage matrix;
- statement that no rule prose/assets were copied unless explicitly reviewed and permissioned.

Default: do not quote. Summarize in original language.

## 5. Common names and neutral names

Common descriptive names may be used when safe. Neutral names should be used when a commercial title, trademark, trade dress, or product identity creates avoidable risk.

Examples:

| Risk profile | Safer Rulepath ID/name |
|---|---|
| generic Tic-Tac-Toe is usually safe, but neutral portfolio naming is cleaner | `three_marks` |
| commercial four-in-a-row brand risk | `column_four` |
| Reversi/Othello ambiguity | `directional_flip` |
| Checkers/Draughts variant ambiguity | `draughts_lite` |
| War-like card comparison | `high_card_duel` |
| Poker subset | `poker_lite` |
| proprietary hidden-role/bluffing games | original names only |

## 6. Original prose and assets

Public rules text must be written for Rulepath. Public assets must be original, project-owned, generated-reviewed, or compatibly licensed.

Do not imitate proprietary boards, cards, component shapes, icons, color schemes, marketing copy, screenshots, or table presentation.

## 7. AI-generated asset review

AI-generated assets require review for:

- recognizable proprietary similarity;
- logos or accidental text;
- trade-dress mimicry;
- unclear rights terms;
- license compatibility;
- inappropriate style imitation;
- source-identifying resemblance.

Generated assets should be editable and replaceable. Keep prompts and review notes when practical.

## 8. Font and file licensing warning

Never include font files unless redistribution rights are verified and license notes are preserved. Prefer system fonts or open-licensed fonts loaded and distributed according to their license.

Do not ship unknown files just because they appear in a design mockup, asset pack, copied website, or AI output.

## 9. Private licensed experiments

Private licensed experiments are late red-team tests only.

They must:

- live in private repositories, private submodules, or local-only folders;
- be excluded from public CI;
- be excluded from public builds;
- be excluded from public docs except generic process notes;
- not leak names, card text, assets, scenarios, screenshots, IDs, or presentation;
- load private data only from private/local sources;
- have build separation and `.gitignore` safeguards;
- undergo kernel-contamination review.

They must not be foundation cases, public tests, public assets, public WASM modules, or reasons to add game nouns to `engine-core`.

## 10. Public web build rule

If it ships to an unauthorized browser, it has shipped.

Do not bundle private licensed modules into public JS/WASM. Do not rely on credentials, hidden routes, feature flags, admin toggles, or CSS hiding to protect bundled private content. Build public and private artifacts separately. Inspect public bundles before release.

## 11. Release checklist

Before public release, verify:

- game names are neutral or permissioned;
- rules docs use original prose;
- source notes exist;
- chosen variants are documented;
- assets are original/compatible;
- no proprietary text appears in static data;
- no screenshots/scans of commercial components are used;
- public traces contain no licensed data;
- public JS/WASM contains no private module IDs;
- private folders are excluded from public CI/build;
- README and public copy do not imply affiliation;
- `engine-core` contains no private-game nouns;
- font licenses are verified;
- AI-generated assets have review notes.

## 12. Human/legal review triggers

Human/legal review is required when:

- a public implementation resembles a commercial presentation;
- a name has living trademark/product identity risk;
- an asset is generated or externally sourced;
- rules wording is close to source wording;
- a private experiment needs any public-facing mention;
- a dependency bundles fonts/assets;
- a trace or fixture may contain licensed content;
- trade dress, logo, packaging, component layout, or distinctive visual identity may be implicated.
