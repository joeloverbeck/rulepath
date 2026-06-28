# Rulepath IP Policy

Status: public repository and public website IP law.

This is operational engineering policy, not legal advice. It is deliberately conservative because Rulepath is a public portfolio, not a risky adaptation showcase.

## 1. Public allowed

Public Rulepath MAY include:

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

Public Rulepath MUST NOT include:

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

## 3. Ideas, systems, and expression

Rulepath policy distinguishes game mechanics from expression:

- mechanics and abstract procedures may be implemented when lawful and safely presented;
- expressive text, art, card wording, graphic layout, logos, distinctive component design, and presentation must be original, permissioned, public-domain, or compatibly licensed;
- public naming and visuals must avoid source confusion;
- when unsure, omit and request human/legal review.

This policy remains conservative even where the law may allow more.

## 4. Public rules documentation

Every public game MUST have:

- original-language Rulepath rules summary;
- source notes with URLs/bibliographic identifiers and consulted dates;
- chosen variant statement;
- known ambiguities and resolutions;
- rule coverage matrix;
- statement that no rule prose/assets were copied unless explicitly reviewed and permissioned.
- player-facing `HOW-TO-PLAY.md` prose when the game is in the public catalog.

Default: do not quote. Summarize in original language.

Player-facing `HOW-TO-PLAY.md` files are public documentation. They MUST use
original Rulepath prose, neutral names, and no copied rulebook text, examples,
diagrams, art, logos, fonts, or trade dress. They may summarize the Rulepath
implementation's rules but must not reproduce external protected expression.

## 4A. IP evidence receipt and source IDs

Each official game records its centralized IP receipt in
[`GAME-EVIDENCE.md`](../templates/GAME-EVIDENCE.md). That receipt links the
source/IP fields that prove the public artifact is original, public-domain,
permissioned, or otherwise compatible with this policy. It is a status and
artifact-link receipt only; detailed consulted-source notes, ambiguity
resolution, naming rationale, asset posture, font posture, copied-content
status, and human/legal review questions remain in the game's source notes.

Per-game source notes SHOULD assign stable source IDs for every consulted source
that affects rule facts, variant selection, naming, assets, fonts, or release
blockers. Other documents should cite those source IDs rather than copying
source prose or repeating IP narratives. `GAME-EVIDENCE.md` then links the
source-note artifact and summarizes the current source/IP receipt status.

Repository-level comparable systems and external prior art live in
[SOURCES.md](SOURCES.md). Those sources may inform doctrine, but each entry must
record the Rulepath-specific lesson and any explicit non-adoption so external
framework assumptions do not override this policy.

## 5. Common names and neutral names

Common descriptive names MAY be used when safe. Neutral names SHOULD be used when a commercial title, trademark, trade dress, or product identity creates avoidable risk.

Examples:

| Risk profile | Safer Rulepath ID/name |
|---|---|
| take-away counter game | `race_to_n` or `nim_lite` |
| Tic-Tac-Toe-like placement | `three_marks` |
| commercial four-in-a-row brand risk | `column_four` |
| Reversi/Othello ambiguity | `directional_flip` |
| Checkers/Draughts variant ambiguity | `draughts_lite` |
| War-like card comparison | `high_card_duel` |
| simple draw/stand threshold scoring | original non-casino name required; `blackjack_lite` is a deferred comparison label only under [ADR 0006](adr/0006-blackjack-lite-roadmap-placement.md) |
| resource economy microgame | `token_bazaar` or `resource_race` |
| simultaneous commitment | `secret_draft` or original name |
| poker subset | `poker_lite` |
| trick-taking | `plain_tricks` |
| bluffing/claims | original name only |

Blackjack, poker, pontoon, casino, betting, chip, table, payout, insurance, and similar public-facing terms are source-research descriptors, not default Rulepath product presentation. A draw/stand threshold game SHOULD use an original non-casino name, original prose, and neutral visual language unless an accepted ADR and human/legal review justify a narrower exception.

Public-domain or common card systems, including Texas Hold'Em rules facts, may
be researched and implemented only with original Rulepath prose, neutral display
names where useful, original card art/icons, and presentation that avoids casino
product framing. `poker_lite` / Crest Ledger is an existing scoped Rulepath
microgame and is not the same public product commitment as a proper Hold'Em
family implementation. A future Hold'Em-family gate must document its naming,
variant, source notes, asset posture, and trade-dress review before public
exposure.

## 6. Original prose and assets

Public rules text MUST be written for Rulepath.

Public assets MUST be original, project-owned, generated-reviewed, public-domain, or compatibly licensed.

Do not imitate proprietary boards, cards, component shapes, iconography, color schemes, marketing copy, screenshots, or table presentation.

Original does not mean lavish. Simple original SVG components are better than risky imitation.

## 7. AI-generated asset review

AI-generated assets require review for:

- recognizable proprietary similarity;
- logos or accidental text;
- trade-dress mimicry;
- unclear rights terms;
- license compatibility;
- inappropriate style imitation;
- source-identifying resemblance;
- inability to edit/replace later.

Generated assets SHOULD be editable and replaceable. Keep prompts and review notes when practical.

Do not assume AI output is safe because it is “new.”

## 8. Fonts

Never include font files unless redistribution rights are verified and license notes are preserved.

Prefer system fonts or open-licensed fonts loaded and distributed according to their license.

Do not ship unknown font files from design mockups, copied websites, asset packs, or AI output.

Font licenses can differ for desktop, web, app embedding, redistribution, and modification. Verify the actual use.

## 9. Private licensed experiments

Private licensed experiments are isolated red-team tests only.

They MUST:

- live in private repositories, private submodules, or local-only folders;
- be excluded from public CI;
- be excluded from public builds;
- be excluded from public docs except generic process notes;
- avoid leaking names, card text, assets, scenarios, screenshots, IDs, or presentation;
- load private data only from private/local sources;
- have build separation and ignore safeguards;
- undergo kernel-contamination review.

They MUST NOT be foundation cases, public tests, public assets, public WASM modules, or reasons to add game nouns to `engine-core`.

### 9A. Sanctioned private lane

A sanctioned private-game lane MAY begin before the public ladder completes only
after accepted ADR approval and the readiness interlocks named by that ADR. The
early timing exception does not weaken isolation: private work remains
non-public, private-build-only, and unable to shape `engine-core` or public
architecture except through generic, private-free seams.

The private repository owns licensed rules references, private source notes,
private docs, private e2e names, private trace and fixture artifacts, private
card or event metadata, private renderers, private catalog entries, and private
WASM/web builds. Public docs may contain only generic doctrine and opaque
private-lane placeholders.

No-name/no-ID public no-leak checklist:

- no private game title, game id, module id, or catalog string in public files;
- no private card, event, scenario, faction, setup, fixture, or e2e filename in
  public files;
- no copied private rules-page text, examples, screenshots, scans, art, icons,
  trade dress, or flowchart text in public files;
- no private renderer, private WASM/JS bundle, private CI artifact, private
  trace, or private fixture in public builds;
- no exception unless separately license-reviewed, explicitly approved, and
  recorded in the relevant private release evidence.

## 10. Public web build rule

If it ships to an unauthorized browser, it has shipped.

Do not bundle private licensed modules into public JS/WASM. Do not rely on credentials, hidden routes, feature flags, admin toggles, or CSS hiding to protect bundled private content.

Public and private artifacts MUST be built separately. Inspect public bundles before release.

## 11. Source notes checklist

Before public exposure, each game source note SHOULD answer:

```text
What sources were consulted?
When were they consulted?
What variant was selected?
What deviations did Rulepath make?
What names/IDs are public-facing?
Are rules prose and assets original or permissioned?
Are generated/external assets reviewed?
Are font licenses verified?
Is any content private or proprietary?
What human/legal review questions remain?
```

## 12. Release checklist

Before public release, verify:

- game names are neutral or permissioned;
- rules docs use original prose;
- source notes exist;
- chosen variants are documented;
- assets are original/compatible/generated-reviewed;
- no proprietary text appears in static data;
- no screenshots/scans of commercial components are used;
- public traces contain no licensed data;
- public JS/WASM contains no private module IDs;
- private folders are excluded from public CI/build;
- README and public copy do not imply affiliation;
- `engine-core` contains no private-game nouns;
- font licenses are verified;
- AI-generated assets have review notes.

## 13. Human/legal review triggers

Human/legal review is required when:

- a public implementation resembles a commercial presentation;
- a name has living trademark/product identity risk;
- an asset is generated or externally sourced;
- rules wording is close to source wording;
- a private experiment needs any public-facing mention;
- a dependency bundles fonts/assets;
- a trace or fixture may contain licensed content;
- trade dress, logo, packaging, component layout, or distinctive visual identity may be implicated.
