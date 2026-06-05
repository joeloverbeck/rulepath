# IP POLICY

Status: public repository and public website IP law.

This is an operational engineering policy, not legal advice. The project should be stricter than the bare minimum because the goal is a clean public portfolio, not a risky commercial adaptation.

## 1. Public allowed

The public repository and public website MAY include:

- public-domain/classic games;
- original games;
- permissioned games;
- neutral implementations of abstract mechanics;
- original rules summaries written in the project's own words;
- original graphics/icons/assets;
- citations/links to public rules sources;
- deliberate documented variants;
- generic engine primitives;
- public benchmark traces that contain no proprietary data.

## 2. Public forbidden unless permissioned

The public repository and public website MUST NOT include:

- copyrighted rulebook prose copied from published games;
- proprietary card text;
- proprietary board art;
- proprietary icons, fonts, screenshots, or assets;
- trademark-forward presentation;
- licensed game modules;
- private licensed game data;
- hidden licensed modules bundled in public builds;
- public test fixtures containing proprietary data;
- private licensed WASM modules;
- scans/photos of proprietary components used as assets.

## 3. Rules documentation for public games

Every public game MUST have structured rules documentation in the repo.

Required:

- original-language rules summary;
- rules-source notes with citations/links where appropriate;
- variant statement;
- rule coverage matrix mapping rules to tests and implementation modules;
- known ambiguities and chosen resolutions;
- no copied rulebook prose except tiny fair-use-like references if approved by human review.

Default behavior: do not quote. Summarize in original language.

## 4. Classic and public-domain games

Classic mechanics are not automatically risk-free in every presentation.

For classic games:

- verify rules from reputable public sources;
- document the chosen variant;
- use original wording;
- use original assets;
- avoid trademark-forward names when a neutral name is safer;
- avoid mimicking a proprietary product's layout, iconography, colors, or component style.

Examples of safer naming:

- use `four_in_a_row` rather than a trademark-forward title;
- use `directional_flip` or `reversi_style` where appropriate;
- use `draughts_lite` or `checkers_variant` with variant notes;
- use original microgame names for bluffing, drafting, auction, and cooperative stages.

## 5. Trademark-risk presentation

Names, logos, trade dress, and source-identifying presentation create avoidable risk.

The project SHOULD:

- use neutral names for abstract mechanics;
- avoid logos and commercial product styling;
- avoid marketing copy that suggests affiliation;
- avoid screenshots or component mimicry;
- clarify when a game is an original variant.

## 6. Private licensed experiments

Private licensed experiments MAY exist only as late-stage red-team tests.

They MUST:

- live in private repositories, private submodules, or local-only folders;
- be excluded from public CI;
- be excluded from public builds;
- be excluded from public docs except generic architecture notes;
- never leak proprietary names, card text, assets, scenarios, or presentation;
- load private data only from local/private sources;
- have a clear `.gitignore`/build separation plan.

They MUST NOT:

- be foundation cases;
- be required for public tests;
- be bundled into public WASM;
- be hidden in a public hosted app by credentials;
- motivate kernel nouns;
- dictate public ladder priorities.

## 7. Public web build rule

If data or code ships to an unauthorized browser, it has shipped.

Therefore:

- do not bundle private licensed modules into public JS/WASM;
- do not include private assets in public static files;
- do not rely on UI hiding, admin credentials, feature flags, or route guards to protect bundled licensed content;
- build public and private artifacts separately.

## 8. Source notes in game modules

Each public game SHOULD include a `docs/SOURCES.md` or source section with:

- rules sources consulted;
- variant chosen;
- date consulted;
- deviations from common variants;
- proof that rule prose was rewritten;
- asset authorship/licensing notes.

## 9. Asset policy

Public assets MUST be:

- original;
- project-owned;
- explicitly licensed for use;
- generated under clear rights terms and reviewed;
- or from a source whose license is compatible with the repository.

Do not include font files in the repo unless licensing and redistribution rights are verified. Prefer system fonts or web-safe open-licensed fonts loaded according to their license.

## 10. Legal background

U.S. copyright sources distinguish game ideas/methods from expressive rule text and art. That distinction supports implementing neutral mechanics, but it does not make copying prose, assets, card text, or trade dress acceptable for this project.

The project policy is deliberately conservative: even when something might be legally arguable, avoid public IP risk unless permission is clear.

## Source notes

See `SOURCES.md`, especially U.S. Copyright Office Games, USPTO Copyright Basics, Baker v. Selden, public rules sources, and UI/original-asset guidance.
