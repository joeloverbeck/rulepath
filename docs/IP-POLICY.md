# Rulepath IP Policy

Status: public repository and public website IP law.

This is an operational engineering policy, not legal advice. It is deliberately stricter than the bare minimum because Rulepath's goal is a clean public portfolio, not a risky public adaptation.

## 1. Public allowed

The public repository and public website MAY include:

- public-domain/classic games;
- original games;
- permissioned games;
- neutral implementations of abstract mechanics;
- original rules summaries written in Rulepath's own words;
- original graphics/icons/assets;
- compatible open-licensed assets with preserved license notes;
- citations/links to public rules sources;
- deliberate documented variants;
- generic engine primitives;
- public benchmark traces containing no proprietary data.

## 2. Public forbidden unless explicitly permissioned

The public repository and public website MUST NOT include:

- copyrighted rulebook prose copied from published games;
- proprietary card text;
- proprietary board art;
- proprietary icons;
- proprietary fonts or font files without verified redistribution rights;
- screenshots or scans of proprietary components as assets;
- trademark-forward presentation;
- trade-dress mimicry;
- licensed game modules;
- private licensed game data;
- hidden licensed modules bundled in public builds;
- public test fixtures containing proprietary data;
- private licensed WASM modules;
- public docs that make the project look secretly focused on a private licensed game.

## 3. Game ideas vs expression

U.S. copyright sources distinguish game ideas, methods, procedures, and systems from expressive text and art. That supports implementing neutral mechanics, but it does not make copying prose, assets, card text, names-as-branding, or trade dress acceptable.

Rulepath policy is simple:

- mechanics may be implemented when lawful and safely presented;
- expression must be original, permissioned, or compatibly licensed;
- public naming and visuals must avoid affiliation confusion;
- when unsure, omit and ask for human/legal review.

## 4. Public rules documentation

Every public game MUST have structured rules documentation in the repo.

Required:

- original-language rules summary;
- rules-source notes with citations/links where appropriate;
- variant statement;
- rule coverage matrix mapping rules to tests and modules;
- known ambiguities and chosen resolutions;
- no copied rulebook prose except tiny reviewed references if explicitly approved by a human.

Default behavior: do not quote. Summarize in original language.

## 5. Classic and public-domain games

Classic mechanics are not automatically risk-free in every presentation.

For classic games:

- verify rules from reputable public sources;
- document the chosen variant;
- write original prose;
- use original assets;
- avoid trademark-forward names when neutral names are safer;
- avoid mimicking a proprietary product's layout, iconography, colors, component style, or marketing copy.

Safer naming examples:

| Riskier/commercial-forward | Safer Rulepath public ID |
|---|---|
| commercial Four-in-a-Row name | `column_four` |
| Tic-Tac-Toe as generic title is usually safe, but still plain | `three_marks` |
| Reversi/Othello ambiguity | `directional_flip` |
| Checkers/Draughts variant ambiguity | `draughts_lite` |
| War | `high_card_duel` |
| Poker subset | `poker_lite` |
| proprietary hidden-role/bluffing games | original names only |

## 6. Trademark and trade-dress presentation

Names, logos, trade dress, and source-identifying presentation create avoidable risk.

Rulepath SHOULD:

- use neutral names for abstract mechanics;
- avoid logos and commercial product styling;
- avoid marketing copy that suggests affiliation;
- avoid screenshots or component mimicry;
- clarify when a game is an original variant;
- use clean abstract premium visuals rather than imitation.

## 7. Assets

Public assets MUST be:

- original;
- project-owned;
- explicitly licensed for compatible use;
- generated under clear rights terms and reviewed;
- or from sources whose licenses are compatible with repository distribution.

Do not include font files unless licensing and redistribution rights are verified. Prefer system fonts or web-safe open-licensed fonts loaded according to their license.

AI-generated assets MUST be reviewed for:

- recognizable proprietary similarity;
- trade-dress mimicry;
- unclear rights terms;
- accidental logos/text;
- license compatibility.

## 8. Public source notes in game modules

Each public game SHOULD include `games/<game>/docs/SOURCES.md` with:

- rules sources consulted;
- date consulted;
- variant chosen;
- deviations from common variants;
- statement that rule prose was rewritten;
- asset authorship/licensing notes;
- trademark/presentation risk notes if relevant.

Template:

```text
Source: <name + URL>
Consulted: YYYY-MM-DD
Used for: rule verification / variant comparison / historical note
Copied prose/assets: none
Variant chosen: <description>
Rulepath deviations: <description>
Public name rationale: <description>
Asset status: original / licensed / generated-reviewed
```

## 9. Private licensed experiments

Private licensed experiments MAY exist only as late-stage red-team tests.

They MUST:

- live in private repositories, private submodules, or local-only folders;
- be excluded from public CI;
- be excluded from public builds;
- be excluded from public docs except generic architecture notes;
- never leak proprietary names, card text, assets, scenarios, screenshots, or presentation;
- load private data only from local/private sources;
- have a clear `.gitignore` and build separation plan;
- undergo kernel-contamination review.

They MUST NOT:

- be foundation cases;
- be required for public tests;
- be bundled into public WASM;
- be hidden in a public hosted app by credentials;
- motivate kernel nouns;
- dictate public ladder priorities;
- appear in public screenshots or marketing.

## 10. Public web build rule

If data or code ships to an unauthorized browser, it has shipped.

Therefore:

- do not bundle private licensed modules into public JS/WASM;
- do not include private assets in public static files;
- do not rely on UI hiding, admin credentials, feature flags, or route guards to protect bundled licensed content;
- build public and private artifacts separately;
- inspect public bundles for private IDs before release.

## 11. Release checklist

Before a public release, verify:

- game names are neutral or permissioned;
- rules docs use original prose;
- source notes exist;
- assets are original/compatible;
- no proprietary text appears in static data;
- no screenshots/scans of commercial components are used;
- public traces contain no licensed data;
- public WASM/JS bundles contain no private module IDs;
- private folders are excluded from public CI/build;
- README/marketing copy does not imply affiliation;
- `engine-core` contains no private-game nouns.

## 12. Human review triggers

Human review is required when:

- a public implementation resembles a commercial product presentation;
- a game has living trademark/product identity risk;
- an asset is generated or sourced externally;
- a rule summary is close to a source's wording;
- a private experiment needs any public-facing mention;
- a dependency bundles fonts/assets;
- a trace or fixture may contain licensed content.

## Source notes

See `SOURCES.md`, especially U.S. Copyright Office Games, USPTO Copyright Basics, Baker v. Selden, public rules sources, and UI/original-asset guidance.
