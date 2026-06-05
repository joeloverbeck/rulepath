# <game_id> Sources

Game ID: `<game_id>`

Public display name: `<display_name>`

Implemented variant: `<variant>`

Prepared by: `<name/agent>`

Created: YYYY-MM-DD

Last updated: YYYY-MM-DD

Rules version connected to this source note: `<rules_version>`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context, public-history context, and ambiguity resolution.

Sources do not authorize copied prose, card text, icons, board art, screenshots, scans, fonts, assets, or trade dress. Rulepath public rule prose, UI copy, visual presentation, assets, icons, and component text MUST be original, project-owned, public-domain where verified, or separately license-reviewed.

Do not paste proprietary text into this file. Summarize in original language.

## Consulted sources

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| `<source_name>` | `<url_or_reference>` | YYYY-MM-DD | official rules / publisher / rules authority / standards body / reputable secondary / community reference / unverified | rule verification / variant comparison / ambiguity / naming / historical context / strategy context / asset license | none / quoted short excerpt reviewed / license-reviewed asset / human review needed | `<notes>` |

Source quality guidance:

- Prefer official rules, standards bodies, public-domain authorities, and reputable references.
- Community summaries MAY help locate ambiguities but MUST NOT be sole authority for release-critical rules unless marked and reviewed.
- Strategy sources are evidence, not rule authority.
- If a source is uncertain, mark it uncertain. Do not invent support.

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `<variant>` | `<source/rationale>` | yes/no |
| player count | `<count>` | `<source/rationale>` | yes/no |
| optional rule included | `<rule>` / not applicable | `<source/rationale>` | yes/no |
| optional rule excluded | `<rule>` / not applicable | `<source/rationale>` | yes/no |
| Rulepath deviation from common variant | `<deviation>` / none | `<source/rationale>` | yes/no |
| out-of-scope variant | `<variant>` / none | `<source/rationale>` | yes/no |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `AMB-001` | `<ambiguity>` | `<sources>` | `<resolution>` | `<rule_ids>` | `<tests/traces>` | open / resolved / human review needed |

## Public naming rationale

Public ID: `<game_id>`

Display name: `<display_name>`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes/no/unclear | `<notes>` |
| neutral name chosen? | yes/no/not applicable | `<notes>` |
| trademark risk considered? | yes/no | `<notes>` |
| trade-dress risk considered? | yes/no | `<notes>` |
| affiliation implication avoided? | yes/no | `<notes>` |
| public help text needs disclaimer? | yes/no | `<notes>` |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---|---|---:|
| `<concern>` | none / low / medium / high / human review needed | `<mitigation>` | yes/no |

Avoid proprietary mimicry. Public games should look and read like Rulepath, not like a clone of a commercial product.

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| `<asset_group>` | `<files_or_ids>` | original / project-owned / public-domain verified / license-reviewed / generated-reviewed / placeholder / human review needed | `<details>` | `<notes>` | yes/no |

No copied card text, board art, scans, screenshots, icons, fonts, or trade dress may enter public files without explicit review.

## Generated asset review notes

| Generated asset | Tool/model if known | Prompt/source inputs safe? | Similarity/trade-dress risk | Human review result | Public-safe? |
|---|---|---:|---|---|---:|
| `<asset>` | `<tool>` | yes/no/unclear | none/low/medium/high | `<notes>` | yes/no |

Generated assets MUST be reviewed as assets. Generation does not automatically make them safe.

## Font status

| Font | Source/license | Bundled in public artifact? | Review status | Notes |
|---|---|---:|---|---|
| system font stack | not bundled | no | safe by default | `<notes>` |
| `<font_name>` | `<source/license>` | yes/no | reviewed / human review needed / blocked | `<notes>` |

Never ship font files unless their license and redistribution status are reviewed.

## Public/private content boundary

| Content | Public allowed? | Location | Notes |
|---|---:|---|---|
| Rulepath original rules summary | yes/no | `<path>` | `<notes>` |
| public-domain/classic rule facts | yes/no | `<path>` | `<notes>` |
| commercial/licensed rules text | no by default | `<path>` | `<notes>` |
| private licensed stress-test content | no public shipment | `<private_path_or_none>` | must not shape architecture |
| source screenshots/scans | no by default | `<path>` | `<notes>` |

Private licensed stress tests are late, isolated, optional, non-public, and non-architectural. They MUST NOT contaminate `engine-core`, public assets, public docs, or public web bundles.

## Human/legal review triggers

Mark any trigger that applies.

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | yes/no | `<notes>` |
| copied or closely paraphrased rules prose | yes/no | `<notes>` |
| card/component text from a protected source | yes/no | `<notes>` |
| scanned/copied art, icon, screenshot, board, card, or UI asset | yes/no | `<notes>` |
| bundled font file | yes/no | `<notes>` |
| generated art with possible trade-dress similarity | yes/no | `<notes>` |
| uncertainty about public-domain status | yes/no | `<notes>` |
| source forbids redistribution or reuse | yes/no | `<notes>` |
| private licensed content touched public path | yes/no | `<notes>` |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| `<concern>` | yes/no | `<resolution>` | `<owner>` |

## Rule-source-to-rule-ID cross-reference

Every release-relevant rule ID in `GAME-RULES.md` SHOULD have source or rationale support here. Original games may cite design rationale instead of external rule sources.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `R-SCOPE-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-COMP-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-SETUP-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-TURN-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-ACTION-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-RESTRICT-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-SCORE-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-END-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-VIS-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-VAR-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |
| `R-AMB-001` | `<summary>` | `<sources/rationale>` | yes/no | `<notes>` |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Assets are original, verified public-domain, license-reviewed, or generated-reviewed.
- Fonts are system-only or license-reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
